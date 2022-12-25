// SPDX-License-Identifier: Apache-2.0

//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

pub mod types;

mod pagination;

#[cfg(test)]
mod testutils;

use anyhow::Result;
use futures::Stream;

use pagination::{PaginatedRequest, PaginationClient};
use types::{
    games::{self, Game, GamesPlayed, Leaderboard},
    profile::Profile,
    search::{self, SearchResults},
};

// Rexports
pub use chrono;
pub use futures;

/// Get profile stats for a player.
///
/// # Params
/// - `profile_id` is aoe4world the ID of the player whose games should be searched.
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

/// Get games for a player. Games as returned as an async stream.
///
/// # Params
/// - `profile_id` is aoe4world the ID of the player whose games should be searched.
/// - `leaderboard` is an optional leaderboard to be searched against (e.g.
/// [`Leaderboard::RmTeam`]).
/// - `opponent_ids` is an optional opponent profile ID to search against.
pub async fn games(
    profile_id: u64,
    leaderboard: Option<Leaderboard>,
    opponent_id: Option<u64>,
    since: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<impl Stream<Item = Result<Game>>> {
    let client = PaginationClient::<GamesPlayed, Game>::default();
    let url = format!("https://aoe4world.com/api/v0/players/{}/games", profile_id).parse()?;
    let filter = games::Filter {
        leaderboard,
        opponent_profile_id: opponent_id,
        since,
    };
    let url = filter.query_params(url);
    let pages = client
        .into_pages_concurrent(PaginatedRequest::new(url))
        .await?;
    Ok(pages.items())
}

/// Search for a player. Results returned as an async stream.
///
/// # Params
/// - `query` the player name to search for.
/// - `exact` determines whether the search should exactly match the player name.
pub async fn search(query: &str, exact: bool) -> Result<impl Stream<Item = Result<Profile>>> {
    let client = PaginationClient::<SearchResults, Profile>::default();
    let url = "https://aoe4world.com/api/v0/players/search".parse()?;
    let filter = search::Filter {
        query: Some(query.to_owned()),
        exact: Some(exact),
    };
    let url = filter.query_params(url);
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
    const ONLY_CAMS_NAME: &str = "🐪🐪🐪OnlyCams🐪🐪🐪";
    const DEBILS_NAME: &str = "DEBILS";

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
        let games: Vec<_> = games(ONLY_CAMS_ID.into(), None, None, None)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        for (i, game) in games.iter().enumerate() {
            assert!(game.is_ok(), "game {} not ok: {:?}", i, game)
        }
        println!("{:?}", games);
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test(flavor = "multi_thread")]
    async fn search_api_smoke() {
        let profiles: Vec<_> = search(ONLY_CAMS_NAME.into(), true)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        for (i, profile) in profiles.iter().enumerate() {
            assert!(profile.is_ok(), "profile {} not ok: {:?}", i, profile)
        }
        println!("{:?}", profiles);

        let profiles: Vec<_> = search(DEBILS_NAME.into(), false)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        for (i, profile) in profiles.iter().enumerate() {
            assert!(profile.is_ok(), "profile {} not ok: {:?}", i, profile)
        }
        println!("{:?}", profiles);
    }
}
