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
    games::{self, Game, GamesPlayed, Leaderboard},
    profile::Profile,
};

// Rexports
pub use chrono;
pub use futures;

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
///
/// # Params
/// - `profile_id` is aoe4world the ID of the player whose games should be searched.
/// - `leaderboard` is an optional leaderboard to be searched against (e.g.
/// [`Leaderboard::RmTeam`]).
/// - `opponent_ids` is an optional list of opponent profile IDs to search against.
pub async fn games(
    profile_id: u64,
    leaderboard: Option<Leaderboard>,
    opponent_ids: Option<&[u64]>,
    since: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<impl Stream<Item = Result<Game>>> {
    let client = PaginationClient::<GamesPlayed, Game>::default();
    let url = format!("https://aoe4world.com/api/v0/players/{}/games", profile_id).parse()?;
    let filter = games::Filters {
        leaderboard,
        opponent_profile_ids: opponent_ids.map(|ids| ids.to_vec()).unwrap_or_default(),
        since,
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
}
