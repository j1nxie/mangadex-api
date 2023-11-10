use std::env::{var, VarError};

use clap::Parser;

use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{
    Language, MangaFeedSortOrder, Password, ReferenceExpansionResource, Username,
};

pub type VarResult<T, E = std::io::Error> = Result<T, E>;

const CLIENT_ID: &str = "CLIENT_ID";

const CLIENT_SECRET: &str = "CLIENT_SECRET";

const USERNAME: &str = "USERNAME_";

const PASSWORD: &str = "PASSWORD_";

#[derive(Debug)]
struct PreUserInfos {
    username: String,
    password: String,
}

impl PreUserInfos {
    fn new() -> VarResult<Self> {
        Ok(Self {
            username: var(USERNAME).map_err(|e| match e {
                VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, USERNAME),
                VarError::NotUnicode(e) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_str().unwrap_or_default().to_string(),
                ),
            })?,
            password: var(PASSWORD).map_err(|e| match e {
                VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, PASSWORD),
                VarError::NotUnicode(e) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_str().unwrap_or_default().to_string(),
                ),
            })?,
        })
    }
}

#[derive(Debug)]
struct UserInfos {
    username: Username,
    password: Password,
}

impl TryFrom<PreUserInfos> for UserInfos {
    type Error = mangadex_api_types::error::Error;
    fn try_from(value: PreUserInfos) -> Result<Self, Self::Error> {
        Ok(Self {
            username: Username::parse(value.username)?,
            password: Password::parse(value.password)?,
        })
    }
}

fn get_client_info_from_var() -> VarResult<ClientInfo> {
    Ok(ClientInfo {
        client_id: var(CLIENT_ID).map_err(|e| match e {
            VarError::NotPresent => std::io::Error::new(std::io::ErrorKind::NotFound, CLIENT_ID),
            VarError::NotUnicode(e) => std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_str().unwrap_or_default().to_string(),
            ),
        })?,
        client_secret: var(CLIENT_SECRET).map_err(|e| match e {
            VarError::NotPresent => {
                std::io::Error::new(std::io::ErrorKind::NotFound, CLIENT_SECRET)
            }
            VarError::NotUnicode(e) => std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_str().unwrap_or_default().to_string(),
            ),
        })?,
    })
}

#[derive(Parser)]
#[clap(name = "MangaDex User Feed", about = "Fetch the user chapter feed")]
struct Args {
    /// Space-separated ISO 639-1 2-letter language code representation.
    #[clap(default_value = "en")]
    languages: Vec<Language>,
    /// Start the results from the specified page number, starting from 1.
    #[clap(short, long, default_value = "1")]
    page: u32,
    /// Set the maximum number of results to return.
    #[clap(short, long, default_value = "10")]
    limit: u32,
}

impl Args {
    pub fn offset(&self) -> u32 {
        get_page_offset(self.page, self.limit)
    }
}

async fn run(arg: Args) -> anyhow::Result<()> {
    let client_info = get_client_info_from_var()?;
    //println!("client_info: {:?}", client_info);
    let user_info: UserInfos = TryFrom::try_from(PreUserInfos::new()?)?;
    //println!("user_info : {:?}", user_info);

    let mut client = MangaDexClient::default();
    client.set_client_info(&client_info).await?;

    println!("Fetching your access token");
    let oauth_res = client
        .oauth()
        .login()
        .username(user_info.username)
        .password(user_info.password)
        .send()
        .await?;
    println!(
        "Your token will expire in {} minutes",
        (oauth_res.expires_in / 60)
    );

    println!("Fetching your manga feed");

    let feed = client
        .user()
        .follows()
        .manga()
        .feed()
        .get()
        .limit(arg.limit)
        .offset(arg.offset())
        .translated_language(arg.languages)
        .order(MangaFeedSortOrder::ReadableAt(
            mangadex_api_types::OrderDirection::Descending,
        ))
        .include(ReferenceExpansionResource::Manga)
        .send()
        .await?;
    println!("Fetched");
    println!("{}", serde_json::to_string_pretty(&feed)?);
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        use std::process;
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/// Calculate the offset needed to view the page results.
fn get_page_offset(page: u32, limit: u32) -> u32 {
    if page == 0 || limit == 0 {
        return 0;
    }

    limit * (page - 1)
}
