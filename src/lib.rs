// SPDX-License-Identifier: Apache-2.0

//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

pub mod types;

mod pagination;

use anyhow::Result;
use futures::Stream;

use pagination::{PaginatedRequest, PaginationClient};
use types::{
    games::{self, Game, GamesPlayed},
    profile::Profile,
};

/// Get profile stats for a player.
pub async fn profile(profile_id: u64) -> Result<Profile> {
    reqwest::get(format!(
        "https://aoe4world.com/api/v0/players/{}",
        profile_id
    ))
    .await?
    .json()
    .await
    .map_err(anyhow::Error::from)
}

/// Get games for a player.
pub async fn games(
    profile_id: u64,
    filter: Option<games::Filters>,
) -> Result<impl Stream<Item = Result<Game>>> {
    let client = PaginationClient::<GamesPlayed, Game>::default();
    let url = format!("https://aoe4world.com/api/v0/players/{}/games", profile_id).parse()?;
    let url = if let Some(filter) = filter {
        filter.query_params(url)
    } else {
        url
    };
    let pages = client
        .into_pages_concurrent(PaginatedRequest::new(url))
        .await?;
    Ok(pages.items())
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::StreamExt;

    const ONLY_CAMS_ID: u64 = 10433860;

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test]
    async fn profile_api_smoke() {
        profile(ONLY_CAMS_ID.into())
            .await
            .expect("API call should succeed");
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test(flavor = "multi_thread")]
    async fn games_api_smoke() {
        let games: Vec<_> = games(ONLY_CAMS_ID.into(), None)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        for (i, game) in games.iter().enumerate() {
            assert!(game.is_ok(), "game {} not ok: {:?}", i, game)
        }
        println!("{:?}", games);
    }
}
