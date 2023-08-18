# mangadex-api

## 3.0.0 developpement

A lot of changes will occur in 3.0.0. Please refer to [#27](https://github.com/tonymushah/mangadex-api/issues/27)

## Important

This git repo is just a fork from [gondolyr/mangadex-api](https://gitlab.com/gondolyr/mangadex-api) but the project and crate has been yanked so I will now maintain this crate for [special-eureka](https://github.com/tonymushah/special-eureka) and [eureka-manager](https://github.com/tonymushah/eureka-mmanager)

The `mangadex-api` crate provides a convenient, high-level wrapper
[client][library-client] for the [MangaDex API][mangadex-api-url],
written in [Rust][rust-homepage].

It covers all public endpoints covered by [their documentation][mangadex-api-docs-url].

[Documentation (docs.rs)](https://docs.rs/mangadex_api)

[Documentation (Project `main` branch)](https://gondolyr.gitlab.io/mangadex-api/mangadex_api)

Please note that as MangaDex is still in beta, this SDK will be subject to sudden breaking changes.

## Disclaimer

`mangadex-api` is not affiliated with [MangaDex][mangadex-homepage].

# Table of Contents

- [mangadex-api](#mangadex-api)
  - [Important](#important)
  - [Disclaimer](#disclaimer)
- [Table of Contents](#table-of-contents)
- [Requirements](#requirements)
- [How to install](#how-to-install)
- [Dependency Justification](#dependency-justification)
- [Features](#features)
- [HTTP Client](#http-client)
- [Response Structs](#response-structs)
- [Getting Started](#getting-started)
- [Using a custom reqwest Client](#using-a-custom-reqwest-client)
- [Searching manga by title](#searching-manga-by-title)
- [Searching manga by title with reference expansion](#searching-manga-by-title-with-reference-expansion)
- [Downloading chapter pages](#downloading-chapter-pages)
  - [Using the old way](#using-the-old-way)
  - [Using the `utils` feature](#using-the-utils-feature)
    - [Via `(filename, bytes)` vector based :](#via-filename-bytes-vector-based-)
    - [Via `tokio-stream` :](#via-tokio-stream-)
      - [Without checker](#without-checker)
      - [with checker](#with-checker)
- [Downloading a manga's main cover image](#downloading-a-mangas-main-cover-image)
  - [Use the legacy way](#use-the-legacy-way)
  - [Using the `utils` feature](#using-the-utils-feature-1)
    - [via a cover id](#via-a-cover-id)
    - [via a manga id](#via-a-manga-id)
- [Changelog](#changelog)
- [License](#license)
  - [Contribution](#contribution)
- [Contributing](#contributing)

# Requirements

[Back to top][readme-section-toc]

- [Rust 1.56+][rust-homepage]

# How to install

[Back to top][readme-section-toc]

Add `mangadex-api` to your dependencies:

```toml
[dependencies]
# ...
# Types and schemas are always required
mangadex-api-types-rust = "0.3.3"
mangadex-api-schema-rust = "0.3.2"
mangadex-api = "2.2.0"
```

If you are using [`cargo-edit`](https://github.com/killercup/cargo-edit), run

```bash
cargo add mangadex-api
```

# Dependency Justification

| Dependency                                         | Used for                                                                                                                                 | Included   |
|:---------------------------------------------------|:-----------------------------------------------------------------------------------------------------------------------------------------|:-----------|
| [`anyhow`][dependency-anyhow-docs]                 | Capturing unexpected errors.                                                                                                             | always     |
| [`mangadex-api-types-rust`][dependency-mangadex-api-types]                 | Enums and static data for Mangadex API                                                                                                              | always     |
| [`mangadex-api-schema-rust`][dependency-mangadex-api-schema]                 | Types used for Mangadex API                                                                                                              | always     |
| [`clap`][dependency-clap-docs]                     | Examples demonstrating the library's capabilities                                                                                        | dev builds |
| [`derive_builder`][dependency-derive_builder-docs] | Conveniently generating setters for the API endpoint builders.                                                                           | always     |
| [`fake`][dependency-fake-docs]                     | Generating random data for unit tests.                                                                                                   | dev builds |
| [`futures`][dependency-futures-docs]               | Async request processing.                                                                                                                | always     |
| [`reqwest`][dependency-reqwest-docs]               | Making HTTP requests to the [MangaDex API][mangadex-api-url].                                                                            | always     |
| [`serde`][dependency-serde-docs]                   | Se/dese/rializing HTTP response bodies into structs.                                                                                     | always     |
| [`serde_json`][dependency-serde_json-docs]         | Creating JSON objects for unit tests.                                                                                                    | dev builds |
| [`serde_qs`][dependency-serde_qs-docs]             | Query string serialization for HTTP requests.                                                                                            | always     |
| [`thiserror`][dependency-thiserror-docs]           | Customized error handling.                                                                                                               | always     |
| [`time`][dependency-time-docs]                     | Convenience types for handing time fields.                                                                                               | always     |
| [`tokio`][dependency-tokio-docs]                   | Async runtime to handle futures in __(only)__ examples and `utils` feature in chapter reporting                                                                      | dev builds + `utils` features |
| [`url`][dependency-url-docs]                       | Convenient `Url` type for validating and containing URLs.                                                                                | always     |
| [`uuid`][dependency-uuid-docs]                     | Convenient `Uuid` type for validating and containing UUIDs for requests and responses. Also used to randomly generate UUIDs for testing. | always     |
| [`wiremock`][dependency-wiremock-docs]             | HTTP mocking to test the [MangaDex API][mangadex-api-url].                                                                               | dev builds |

# Features

[Back to top][readme-section-toc]

All features are not included by default. To enable them, add any of the following to your project's `Cargo.toml` file.

- `multi-thread`

  Enable the `MangaDexClient` to be thread-safe, at the cost of operations being slightly more expensive.

- `legacy-auth`

  Enable the usage of the `< 5.9.0` login system in the SDK. Please visit the [Mangadex Discord](https://discord.com/invite/mangadex)  for more details

- `legacy-account`

  Enable the usage of the `< 5.9.0` account management system in the SDK. Please visit the [Mangadex Discord](https://discord.com/invite/mangadex)  for more details

- `utils`

  Enable the usage of the `MangaDexClient::download()`. Allows you to download chapters or covers image without tears and long code.

For example, to enable the `multi-thread` feature, add the following to your `Cargo.toml` file:

```toml
mangadex-api = { version = "2.1.0", features = ["multi-thread"] }
```

# HTTP Client

[Back to top][readme-section-toc]

The [`mangadex_api::MangaDexClient`][library-client] is asynchronous, using
[`reqwest`][reqwest] as the HTTP client.

# Response Structs

[Back to top][readme-section-toc]

The response structs can be found in the [`schemas` module][library-schema-module] and contain the fields in a JSON response.

# Getting Started

[Back to top][readme-section-toc]

This example demonstrates how to fetch a random manga.

```rust
use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let random_manga = client
        .manga()
        .random()
        .build()?
        .send()
        .await?;

    println!("{:?}", random_manga);

    Ok(())
}
```

# Using a custom reqwest Client

[Back to top][readme-section-toc]

By default, [`mangadex_api::MangaDexClient`][library-client] will use the default
[`reqwest::Client`][reqwest-client] settings.

You may provide your own [`reqwest::Client`][reqwest-client] to customize options such as the
request timeout.

```rust
use reqwest::Client;

use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let reqwest_client = Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()?;

let client = MangaDexClient::new(reqwest_client);
# Ok(())
# }
```

# Searching manga by title

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>

```rust
use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_results = client
    .manga()
    .search()
    .title("full metal")
    .build()?
    .send()
    .await?;

println!("manga results = {:?}", manga_results);
# Ok(())
# }
```

# Searching manga by title with reference expansion

[Back to top][readme-section-toc]

Every fetch will include all relationships but with minimal information such as the relationship type and ID. Reference expansion will include the full JSON object in the results for the types that are added to the request.

In the example below, any associated authors in the list of relationships will provide detailed information such as the author's name, biography, and website in the results.

References:

- <https://api.mangadex.org/docs/reference-expansion/>
- Endpoint: <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
- Author object: <https://api.mangadex.org/swagger.html#/Author/get-author-id>

```rust
use mangadex_api::types::{ReferenceExpansionResource, RelationshipType};
use mangadex_api::v5::schema::RelatedAttributes;
use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_results = client
    .manga()
    .search()
    .title("full metal")
    .include(&ReferenceExpansionResource::Author)
    .build()?
    .send()
    .await?;

println!("manga results = {:?}", manga_results);

let authors = manga_results.data.iter().filter_map(|manga| {
    for rel in &manga.relationships {
        if rel.type_ == RelationshipType::Author {
            return Some(rel);
        }
    }

    None
});

for author in authors {
    if let Some(RelatedAttributes::Author(author_attributes)) = &author.attributes {
        println!("{} - {}", author.id, author_attributes.name);
    }
}
# Ok(())
# }
```

# Downloading chapter pages

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/docs/reading-chapter/>

## Using the old way

```rust
// Imports used for downloading the pages to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let chapter_id = Uuid::new_v4();

let at_home = client
    .at_home()
    .server()
    .chapter_id(&chapter_id)
    .build()?
    .send()
    .await?;

let http_client = reqwest::Client::new();

// Original quality. Use `.data.attributes.data_saver` for smaller, compressed images.
let page_filenames = at_home.chapter.data;
for filename in page_filenames {
    // If using the data-saver option, use "/data-saver/" instead of "/data/" in the URL.
    let page_url = at_home
        .base_url
        .join(&format!(
            "/{quality_mode}/{chapter_hash}/{page_filename}",
            quality_mode = "data",
            chapter_hash = at_home.chapter.hash,
            page_filename = filename
        ))
        .unwrap();

    let res = http_client.get(page_url).send().await?;
    // The data should be streamed rather than downloading the data all at once.
    let bytes = res.bytes().await?;

    // This is where you would download the file but for this example,
    // we're just printing the raw data.
    // let mut file = File::create(&filename)?;
    // let _ = file.write_all(&bytes);
    println!("Chunk: {:?}", bytes);
}

# Ok(())
# }
```

## Using the `utils` feature

### Via `(filename, bytes)` vector based :

Not recommended if you want to handle each response error

```rust
use crate::{utils::download::chapter::DownloadMode, MangaDexClient};
use anyhow::{Ok, Result};
/// used for file exporting
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

/// It's from this manga called [`The Grim Reaper Falls In Love With A Human`](https://mangadex.org/title/be2efc56-1669-4e42-9f27-3bd232bca8ea/the-grim-reaper-falls-in-love-with-a-human)
///
/// [Chapter 1 English](https://mangadex.org/chapter/2b4e39a5-fba0-4055-a176-8b7e19faacdb) by [`Kredim`](https://mangadex.org/group/0b870e54-c75f-4d2e-8068-c40f939135fd/kredim)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "your-output-dir";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("32b229f6-e9bf-41a0-9694-63c11191704c")?;
    let chapter_files = client
        /// We use the download builder
        .download()
        /// Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        /// You also use `DownloadMode::Normal` if you want some the original quality
        /// 
        /// Default : Normal
        .mode(DownloadMode::DataSaver)
        /// Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint) if true 
        /// 
        /// Default : false
        .report(true)
        /// Something that i don`t really know about 
        /// 
        /// More details at : https://api.mangadex.org/docs/retrieving-chapter/#basics
        .force_port_443(false)
        .build()?
        .download_element_vec()
        .await?;
    create_dir_all(format!("{}{}", output_dir, chapter_id))?;
    for (filename, bytes) in chapter_files {
        if let Some(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?
        };
    }
    Ok(())
}
```

### Via `tokio-stream` :

With [`tokio-stream`](https://docs.rs/tokio-stream/), you can handle each response result

#### Without checker 

```rust
use crate::{utils::download::chapter::DownloadMode, MangaDexClient};
use anyhow::{Ok, Result};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tokio::pin;
use tokio_stream::StreamExt;

/// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
///
/// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "./test-outputs/";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
    create_dir_all(format!("{}{}", output_dir, chapter_id))?;
    let download = client
        /// We use the download builder
        .download()
        /// Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        /// You also use `DownloadMode::Normal` if you want some the original quality
        /// 
        /// Default : Normal
        .mode(DownloadMode::DataSaver)
        /// Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint) if true 
        /// 
        /// Default : false
        .report(true)
        /// Something that i don`t really know about 
        /// 
        /// More details at : https://api.mangadex.org/docs/retrieving-chapter/#basics
        .force_port_443(false)
        .build()?;
    let chapter_files = download.download_stream().await?;
    /// `pin!` Required for iteration
    pin!(chapter_files); 
    while let Some((data, _, _)) = chapter_files.next().await {
        let (filename, bytes_) = data?;
        if let Some(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?
        };
    }
    Ok(())
}

```

#### with checker

The checker is a function called after the response fetching but before retreiving the byte content.
Example :

```rust
    /// Some code here
    let download = client
        .download()
        .chapter(chapter_id)
        .mode(DownloadMode::DataSaver)
        .report(true)
        .build()?;
    let chapter_files = download
        .download_stream_with_checker(move |filename, response| {
            /// if this function return `true`, the current response will be skipped
            true
        })
        .await?;
    /// Some code here too
```

Real example :

The checker will check return `true` if a file with the response content length has been created

```rust
use crate::{utils::download::chapter::DownloadMode, MangaDexClient};
use anyhow::{Ok, Result};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tokio::pin;
use tokio_stream::StreamExt;

/// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
///
/// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "./test-outputs/";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
    create_dir_all(format!("{}{}", output_dir, chapter_id))?;
    let download = client
        .download()
        .chapter(chapter_id)
        .mode(DownloadMode::DataSaver)
        .report(true)
        .build()?;
    let chapter_files = download
        .download_stream_with_checker(move |filename, response| {
            let is_skip: bool = {
                /// Get the response content length
                let content_length = match response.content_length() {
                    None => return false,
                    Some(d) => d,
                };
                /// open the chapter image file
                if let core::result::Result::Ok(pre_file) = File::open(format!(
                    "{}{}/{}",
                    output_dir,
                    chapter_id,
                    filename.filename.clone()
                )) {
                    if let core::result::Result::Ok(metadata) = pre_file.metadata() {
                        /// compare the content length and the file length
                        metadata.len() == content_length
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            is_skip
        })
        .await?;
    pin!(chapter_files);
    while let Some((data, index, len)) = chapter_files.next().await {
        print!("{index} - {len} : ");
        if let core::result::Result::Ok(resp) = data {
            let (filename, bytes_) = resp ;
            // save the bytes if the `Option` hase Some value
            if let Some(bytes) = bytes_ {
                let mut file: File =
                    File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
                file.write_all(&bytes)?;
                println!("Downloaded {filename}");
            }else{
                println!("Skipped {filename}");
            }
        } else if let core::result::Result::Err(resp) = data {
            println!("{:#?}", resp);
        }
    }
    Ok(())
}
```

# Downloading a manga's main cover image

[Back to top][readme-section-toc]

## Use the legacy way

While this example could directly get the cover information by passing in the cover ID,
it is not often that one would have the ID off-hand, so the most common method would be from a
manga result.

If you want to get all of a manga's cover images, you will need to use the [cover list endpoint](https://api.mangadex.org/swagger.html#/Cover/get-cover)
and use the `manga[]` query parameter.

```rust
// Imports used for downloading the cover to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::types::RelationshipType;
use mangadex_api::v5::MangaDexClient;
use mangadex_api::CDN_URL;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_id = Uuid::new_v4();
let manga = client
    .manga()
    .get()
    .manga_id(&manga_id)
    .build()?
    .send()
    .await?;

let cover_id = manga
    .data
    .relationships
    .iter()
    .find(|related| related.type_ == RelationshipType::CoverArt)
    .expect("no cover art found for manga")
    .id;
let cover = client
    .cover()
    .get()
    .cover_id(&cover_id)
    .build()?
    .send()
    .await?;

// This uses the best quality image.
// To use smaller, thumbnail-sized images, append any of the following:
//
// - .512.jpg
// - .256.jpg
//
// For example, "https://uploads.mangadex.org/covers/8f3e1818-a015-491d-bd81-3addc4d7d56a/4113e972-d228-4172-a885-cb30baffff97.jpg.512.jpg"
let cover_url = Url::parse(&format!(
        "{}/covers/{}/{}",
        CDN_URL, manga_id, cover.data.attributes.file_name
    ))
    .unwrap();

let http_client = reqwest::Client::new();

let res = http_client.get(cover_url).send().await?;
// The data should be streamed rather than downloading the data all at once.
let bytes = res.bytes().await?;

// This is where you would download the file but for this example, we're just printing the raw data.
// let mut file = File::create(&filename)?;
// let _ = file.write_all(&bytes);
println!("Chunk: {:?}", bytes);
# Ok(())
# }
```

## Using the `utils` feature

### via a cover id

```rust
    use anyhow::Result;
    use uuid::Uuid;
    use crate::MangaDexClient;
    use std::{io::Write, fs::File};

    /// Download the volume 2 cover of [Lycoris Recoil](https://mangadex.org/title/9c21fbcd-e22e-4e6d-8258-7d580df9fc45/lycoris-recoil)
    #[tokio::main]
    async fn main() -> Result<()>{
        let cover_id : Uuid = Uuid::parse_str("0bc12ff4-3cec-4244-8582-965b8be496ea")?;
        let client : MangaDexClient = MangaDexClient::default();
        let (filename, bytes) = client.download().cover().build()?.via_cover_id(cover_id).await?;
        let mut file = File::create(format!("{}/{}", "your-output-dir", filename))?;
        file.write_all(&bytes)?;
        Ok(())
    }
```

### via a manga id

```rust
    use anyhow::Result;
    use uuid::Uuid;
    use crate::MangaDexClient;
    use std::{io::Write, fs::File};

    /// Download the [Kimi tte Watashi no Koto Suki Nandesho?](https://mangadex.org/title/f75c2845-0241-4e69-87c7-b93575b532dd/kimi-tte-watashi-no-koto-suki-nandesho) cover
    /// 
    /// For test... of course :3
    #[tokio::main]
    async fn main() -> Result<()>{
        let manga_id : Uuid = Uuid::parse_str("f75c2845-0241-4e69-87c7-b93575b532dd")?;
        let client : MangaDexClient = MangaDexClient::default();
        let (filename, bytes) = client
            .download()
            .cover()
            /// you can use
            /// 
            /// ```rust
            /// .quality(CoverQuality::Size512)
            /// ``` for 512
            /// or
            /// ```rust
            /// .quality(CoverQuality::Size256)
            /// ``` for 256
            .build()?.via_manga_id(manga_id).await?;
        let mut file = File::create(format!("{}/{}", "test-outputs/covers", filename))?;
        file.write_all(&bytes)?;
        Ok(())
    }
```


# Changelog

[Back to top][readme-section-toc]

The changelog can be found [here][changelog].

Changes are added manually to keep the changelog human-readable with summaries of the changes from each version.

# License

[Back to top][readme-section-toc]

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE][license-apache] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT][license-mit] or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

[Back to top][readme-section-toc]

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Contributing

[Back to top][readme-section-toc]

We welcome contributions from everyone. There are many ways to contribute and the
[CONTRIBUTING.md][contributing] document explains how you can contribute and get started.

[dependency-anyhow-docs]: https://docs.rs/anyhow
[dependency-mangadex-api-types]: https://github.com/tonymushah/mangadex-api/tree/main/mangadex-api-types
[dependency-mangadex-api-schema]: https://github.com/tonymushah/mangadex-api/tree/main/mangadex-api-schema
[dependency-clap-docs]: https://docs.rs/clap
[dependency-fake-docs]: https://docs.rs/fake
[dependency-derive_builder-docs]: https://docs.rs/derive_builder
[dependency-futures-docs]: https://docs.rs/futures
[dependency-reqwest-docs]: https://docs.rs/reqwest
[dependency-serde-docs]: https://docs.rs/serde
[dependency-serde_json-docs]: https://docs.rs/serde_json
[dependency-serde_qs-docs]: https://docs.rs/serde_qs
[dependency-thiserror-docs]: https://docs.rs/thiserror
[dependency-time-docs]: https://docs.rs/time
[dependency-tokio-docs]: https://docs.rs/tokio
[dependency-url-docs]: https://docs.rs/url
[dependency-uuid-docs]: https://docs.rs/uuid
[dependency-wiremock-docs]: https://docs.rs/wiremock

[mangadex-api-url]: https://api.mangadex.org
[mangadex-api-docs-url]: https://api.mangadex.org/swagger.html
[mangadex-homepage]: https://mangadex.org
[reqwest]: https://docs.rs/reqwest
[reqwest-client]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
[rust-homepage]: https://rust-lang.org

[changelog]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/CHANGELOG.md
[contributing]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/CONTRIBUTING.md
[library-client]: ./v5/struct.MangaDexClient.html
[library-schema-module]: ./v5/schema/index.html
[license-apache]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-APACHE
[license-mit]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-MIT

[readme-section-toc]: #table-of-contents
