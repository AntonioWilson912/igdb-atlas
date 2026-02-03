# igdb-atlas

An asynchronous Rust wrapper for the [IGDB v4](https://api-docs.igdb.com) video game database API.

[![crates.io](https://img.shields.io/crates/v/igdb-atlas.svg)](https://crates.io/crates/igdb-atlas)
[![docs.rs](https://img.shields.io/docsrs/igdb-atlas.svg)](https://docs.rs/igdb-atlas)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/AntonioWilson912/igdb-atlas/actions/workflows/ci.yml/badge.svg)](https://github.com/AntonioWilson912/igdb-atlas/actions/workflows/ci.yml)

---

## Overview

`igdb-atlas` is a strongly-typed, async-first Rust library for interacting with the IGDB v4 API. It handles Twitch OAuth 2.0 authentication, proactive rate limiting, and provides an ergonomic query builder for constructing Apicalypse queries. The crate is designed with extensibility in mind — adding support for new IGDB endpoints requires minimal boilerplate.

---

## Features

- **Async by default** — Built on `tokio` and `reqwest` for non-blocking I/O
- **Automatic authentication** — Twitch OAuth 2.0 client credentials flow with transparent token caching and refresh
- **Proactive rate limiting** — Token bucket algorithm to stay within IGDB's 4 requests/second limit
- **Exponential backoff with jitter** — Automatic retry logic when 429 errors are encountered
- **Ergonomic query builder** — Fluent builder pattern supporting the full Apicalypse query syntax
- **Strongly-typed models** — Domain-organized model modules with proper handling of nullable fields
- **Extensible architecture** — Generic traits and modular design make adding new endpoints straightforward
- **Comprehensive documentation** — Full rustdoc coverage with inline examples

---

## Prerequisites

- Rust toolchain (stable, 1.85+)
- A Twitch Developer account with an application registered at [https://dev.twitch.tv/console](https://dev.twitch.tv/console)
- Your Twitch **Client ID** and **Client Secret**

> **Note:** IGDB requires valid Twitch credentials for all API requests. See [Authentication Setup](#authentication-setup) for details.

---

## Installation

Add `igdb-atlas` to your `Cargo.toml`:

```toml
[dependencies]
igdb-atlas = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## Authentication Setup

IGDB authentication is handled via Twitch OAuth 2.0. You will need a **Client ID** and **Client Secret** from your Twitch Developer application.

Credentials are passed directly when constructing the client via `ClientConfig`:

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    Ok(())
}
```

The token manager handles fetching, caching, and refreshing the OAuth token transparently. You do not need to manage token lifecycle manually.

---

## Quick Start

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};
use igdb_atlas::endpoints::traits::{Endpoint, Searchable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    let games = client
        .games()
        .search("The Witcher 3")
        .select(&["id", "name", "rating"])
        .limit(5)
        .execute()
        .await?;

    for game in &games {
        println!("{}: {:.1}★", game.display_name(), game.rating.unwrap_or(0.0));
    }

    Ok(())
}
```

---

## Query Builder

The query builder supports the full [Apicalypse query syntax](https://api-docs.igdb.com/#apicalypse-1) used by IGDB. Queries are constructed using a fluent builder pattern:

| Method                      | Apicalypse Equivalent  | Description                                  |
| --------------------------- | ---------------------- | -------------------------------------------- |
| `.select(&[...])`           | `fields name, rating;` | Select which fields to return                |
| `.where_clause("...")`      | `where rating > 80;`   | Filter results                               |
| `.sort("field", Sort::Asc)` | `sort rating asc;`     | Sort results                                 |
| `.limit(n)`                 | `limit 10;`            | Limit number of results                      |
| `.offset(n)`                | `offset 20;`           | Offset for pagination                        |
| `.search("...")`            | `search "zelda";`      | Full-text search (Searchable endpoints only) |

All methods are optional and can be chained in any order. Note that `.search()` is only available on endpoints that implement the `Searchable` trait — see the [Supported Endpoints](#supported-endpoints) table for details.

---

## Examples

### Fetching a Game by ID

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    let game = client
        .games()
        .select(&["name", "storyline", "rating", "first_release_date"])
        .where_clause("id = 1942")
        .execute_one()
        .await?;

    println!("Title: {}", game.name);
    println!("Rating: {:.1}", game.rating.unwrap_or(0.0));

    Ok(())
}
```

### Filtering and Sorting

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    // Top-rated RPGs released after 2020
    let games = client
        .games()
        .select(&["name", "rating", "genres.*"])
        .where_clause("genres.name = \"Role-playing Games\" & first_release_date.y > 2020")
        .sort("rating", igdb_atlas::query::Sort::Desc)
        .limit(10)
        .execute()
        .await?;

    for game in &games {
        println!("{} — Rating: {:.1}", game.name, game.rating.unwrap_or(0.0));
    }

    Ok(())
}
```

### Nested Field Expansion

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    let games = client
        .games()
        .select(&["name", "cover.*", "platforms.name", "involved_companies.company.name"])
        .where_clause("id = 1942")
        .execute()
        .await?;

    for game in &games {
        println!("Title: {}", game.name);

        if let Some(cover) = &game.cover {
            println!("Cover URL: https:{}", cover.url);
        }

        if let Some(platforms) = &game.platforms {
            let names: Vec<&str> = platforms.iter().map(|p| p.name.as_str()).collect();
            println!("Platforms: {}", names.join(", "));
        }
    }

    Ok(())
}
```

### Pagination

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("your_client_id", "your_client_secret");
    let client = IGDBClient::new(config).await?;

    let page_size = 20;

    for page in 0..5 {
        let games = client
            .games()
            .select(&["name", "rating"])
            .sort("rating", igdb_atlas::query::Sort::Desc)
            .limit(page_size)
            .offset(page * page_size)
            .execute()
            .await?;

        println!("--- Page {} ---", page + 1);
        for game in &games {
            println!("  {} — Rating: {:.1}", game.name, game.rating.unwrap_or(0.0));
        }
    }

    Ok(())
}
```

### Handling Errors

```rust,no_run
use igdb_atlas::{IGDBClient, ClientConfig, IGDBError};

#[tokio::main]
async fn main() {
    let config = ClientConfig::new("your_client_id", "your_client_secret");

    let client = match IGDBClient::new(config).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to initialize client: {}", e);
            std::process::exit(1);
        }
    };

    match client
        .games()
        .select(&["name", "rating"])
        .where_clause("id = 99999999")
        .execute_one()
        .await
    {
        Ok(game) => println!("Found: {}", game.name),
        Err(IGDBError::NotFound) => println!("No game found with that ID."),
        Err(IGDBError::RateLimit) => println!("Rate limited — the client will retry automatically."),
        Err(IGDBError::Auth(msg)) => eprintln!("Authentication error: {}", msg),
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
}
```

---

## Rate Limiting

IGDB enforces a limit of **4 requests per second**. `igdb-atlas` handles this in two ways:

1. **Proactive limiting** — A token bucket algorithm ensures requests are spaced appropriately and never exceed the 4/second threshold under normal conditions.
2. **Reactive backoff** — If a `429 Too Many Requests` response is received (e.g., due to clock skew or bursts), the client will automatically retry using exponential backoff with jitter. No action is required on your part.

---

## Testing

All tests live in the `tests/` directory. Mock response fixtures are provided under `tests/fixtures/` to allow tests to run without hitting the live API.

To run the full test suite:

```bash
cargo test
```

---

## License

This project is dual-licensed under your choice of:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

---

## Disclaimer

`igdb-atlas` is an unofficial, community-maintained wrapper. It is not affiliated with or endorsed by IGDB or Twitch. Usage of this library is subject to the [IGDB Terms of Service](https://www.igdb.com/content-policy) and the [Twitch Developer Agreement](https://legal.twitch.com/legal/developer-agreement/). Please ensure your use of the IGDB API complies with their policies.
