// SPDX-License-Identifier: Apache-2.0 or MIT

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

use query::{GlobalGamesQuery, ProfileGamesQuery, ProfileQuery, SearchQuery};
use types::profile::ProfileId;

// Rexports
pub use chrono;
pub use futures;

/// Returns a [`ProfileQuery`]. Used to get profile for a player.
///
/// # Params
/// - `profile_id` is aoe4world the ID of the player.
pub fn profile(profile_id: impl Into<ProfileId>) -> ProfileQuery {
    ProfileQuery::default().with_profile_id(Some(profile_id.into()))
}

/// Returns a [`ProfileGamesQuery`]. Used to query the `/profile/{profile_id}/games` endpoint.
///
/// # Params
/// - `profile_id` is aoe4world the ID of the player whose games should be searched.
pub fn profile_games(profile_id: impl Into<ProfileId>) -> ProfileGamesQuery {
    ProfileGamesQuery::default().with_profile_id(Some(profile_id.into()))
}

/// Returns a [`GlobalGamesQuery`]. Used to query the `/games` endpoint.
///
/// # Examples
///
/// ## List Ranked 1v1 Games
///
/// In the following example, we collect the 100 most recent ranked 1v1 games into a [`Vec`]:
/// ```rust
/// # #[cfg(feature = "test-api")]
/// # tokio_test::block_on(async {
/// use prelate_rs::{futures::StreamExt, games, types::games::GameKind};
///
/// let stream = games()
///     .with_leaderboard(Some(vec![GameKind::Rm1v1]))
///     .get(100)
///     .await
///     .expect("query should succeed");
/// let games = stream.collect::<Vec<_>>().await;
///
/// for game in games {
///     // Do something with each game.
/// # game.expect("game should be valid");
/// }
/// # })
/// ```
pub fn games() -> GlobalGamesQuery {
    GlobalGamesQuery::default()
}

/// Returns a [`SearchQuery`]. Used to query the `/players/search` endpoint.
///
/// Note: the query must contain at least 3 characters.
///
/// # Params
/// - `query` is a search query (e.g. a player's username or part of a username).
///
/// # Examples
///
/// ## Fuzzy Search
///
/// In the following example, we collect the first 10 players who match the
/// search query `"jiglypuf"` into a [`Vec`]:
/// ```rust
/// # #[cfg(feature = "test-api")]
/// # tokio_test::block_on(async {
/// use prelate_rs::{futures::StreamExt, search};
///
/// let stream = search("jiglypuf")
///     .get(10)
///     .await
///     .expect("query should succeed");
/// let profiles = stream.collect::<Vec<_>>().await;
///
/// for profile in profiles {
///     // Do something with each profile.
/// # profile.expect("profile should be valid");
/// }
/// # })
/// ```
///
/// ## Exact Search
///
/// In the following example, we search for the player who matches exactly the
/// search query `"[DEBILS] HousedHorse"`:
/// ```rust
/// # #[cfg(feature = "test-api")]
/// # tokio_test::block_on(async {
/// use prelate_rs::{futures::StreamExt, search};
///
/// let mut stream = search("[DEBILS] HousedHorse")
///     .with_exact(Some(true))
///     .get(1)
///     .await
///     .expect("query should succeed");
/// let profile = stream
///     .next()
///     .await
///     .expect("there should be at least 1 matching profile");
///
/// // Do something with the profile.
/// # profile.expect("profile should be valid");
/// # })
/// ```
pub fn search(query: impl AsRef<str>) -> SearchQuery {
    SearchQuery::default().with_query(Some(query.as_ref().to_string()))
}

pub mod query {
    //! Contains query builders to interact with the aoe4world API.
    //!
    //! Using these directly is possible, but it may be more ergonomic to use
    //! the provided functions at the top-level of the library.

    // Clippy complains about needless update in derived setters.
    #![allow(clippy::needless_update)]

    use anyhow::{bail, Result};
    use derive_setters::Setters;
    use futures::Stream;
    use itertools::join;
    use url::Url;

    use crate::{
        pagination::{PaginatedRequest, PaginationClient},
        types::{
            games::{Game, GameKind, GamesOrder, GlobalGames, ProfileGames},
            profile::{Profile, ProfileId},
            search::SearchResults,
        },
    };

    /// Constructs a query for the `/players/{profile_id}/games` endpoint.
    #[derive(Setters, Default)]
    #[setters(prefix = "with_")]
    #[setters(into)]
    pub struct ProfileGamesQuery {
        /// [`ProfileId`] to query.
        profile_id: Option<ProfileId>,
        /// Filter by game kind category.
        ///
        /// NOTE: this is named `leaderboard` but uses the [`GameKind`] enum.
        leaderboard: Option<Vec<GameKind>>,
        /// Filter over an opponent's profile ID.
        opponent_profile_id: Option<ProfileId>,
        /// Filter over a list of opponent profile IDs.
        opponent_profile_ids: Option<Vec<ProfileId>>,
        /// Filter by time played since a specific date.
        since: Option<chrono::DateTime<chrono::Utc>>,
    }

    impl ProfileGamesQuery {
        /// Get the games for this profile.
        pub async fn get(self, limit: usize) -> Result<impl Stream<Item = Result<Game>>> {
            if self.profile_id.is_none() {
                bail!("missing profile_id")
            }

            let client = PaginationClient::<ProfileGames, Game>::default();
            let url = format!(
                "https://aoe4world.com/api/v0/players/{}/games",
                self.profile_id.unwrap()
            )
            .parse()?;
            let url = self.query_params(url);

            let pages = client
                .into_pages_concurrent(PaginatedRequest::new(url), limit)
                .await?;
            Ok(pages.items())
        }

        fn query_params(&self, mut url: Url) -> Url {
            if let Some(ref leaderboard) = self.leaderboard {
                url.query_pairs_mut()
                    .append_pair("leaderboard", join(leaderboard, ",").as_str());
            }
            if let Some(ref id) = self.opponent_profile_id {
                url.query_pairs_mut()
                    .append_pair("opponent_profile_id", id.to_string().as_str());
            }
            if let Some(ref ids) = self.opponent_profile_ids {
                url.query_pairs_mut()
                    .append_pair("opponent_profile_ids", join(ids, ",").as_str());
            }
            if let Some(ref since) = self.since {
                url.query_pairs_mut()
                    .append_pair("since", since.to_rfc3339().as_str());
            }
            url
        }
    }

    /// Constructs a query for the `/games` endpoint.
    #[derive(Setters, Default)]
    #[setters(prefix = "with_")]
    #[setters(into)]
    pub struct GlobalGamesQuery {
        /// Filter by game kind category.
        ///
        /// NOTE: this is named `leaderboard` but uses the [`GameKind`] enum.
        pub leaderboard: Option<Vec<GameKind>>,
        /// Filter over an opponent's profile ID.
        pub opponent_profile_id: Option<ProfileId>,
        /// Filter over a list of profile IDs.
        pub profile_ids: Option<Vec<ProfileId>>,
        /// Filter by time played since a specific date.
        pub since: Option<chrono::DateTime<chrono::Utc>>,
        /// Filter by time played since a specific date.
        pub order: Option<GamesOrder>,
    }

    impl GlobalGamesQuery {
        /// Get the games.
        pub async fn get(self, limit: usize) -> Result<impl Stream<Item = Result<Game>>> {
            let client = PaginationClient::<GlobalGames, Game>::default();

            let url = "https://aoe4world.com/api/v0/games".parse()?;
            let url = self.query_params(url);

            let pages = client
                .into_pages_concurrent(PaginatedRequest::new(url), limit)
                .await?;
            Ok(pages.items())
        }

        fn query_params(&self, mut url: Url) -> Url {
            if let Some(ref leaderboard) = self.leaderboard {
                url.query_pairs_mut()
                    .append_pair("leaderboard", join(leaderboard, ",").as_str());
            }
            if let Some(id) = self.opponent_profile_id {
                url.query_pairs_mut()
                    .append_pair("opponent_profile_id", id.to_string().as_str());
            }
            if let Some(ref ids) = self.profile_ids {
                url.query_pairs_mut()
                    .append_pair("profile_ids", join(ids, ",").as_str());
            }
            if let Some(ref since) = self.since {
                url.query_pairs_mut()
                    .append_pair("since", since.to_rfc3339().as_str());
            }
            if let Some(ref order) = self.order {
                url.query_pairs_mut()
                    .append_pair("order", serde_json::to_string(order).unwrap().as_str());
            }
            url
        }
    }

    /// Constructs a query for the `/players/{profile_id}` endpoint.
    #[derive(Setters, Default)]
    #[setters(prefix = "with_")]
    #[setters(into)]
    pub struct ProfileQuery {
        /// [`ProfileId`] to query.
        profile_id: Option<ProfileId>,
    }

    impl ProfileQuery {
        /// Get the profile.
        pub async fn get(self) -> Result<Profile> {
            if self.profile_id.is_none() {
                bail!("missing profile_id")
            }

            reqwest::get(format!(
                "https://aoe4world.com/api/v0/players/{}",
                self.profile_id.unwrap()
            ))
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
        }
    }

    /// Constructs a query for the `/players/search` endpoint.
    #[derive(Setters, Default)]
    #[setters(prefix = "with_")]
    #[setters(into)]
    pub struct SearchQuery {
        /// Search query.
        pub query: Option<String>,
        /// Should the results exactly match the query.
        pub exact: Option<bool>,
    }

    impl SearchQuery {
        /// Get the search results.
        pub async fn get(self, limit: usize) -> Result<impl Stream<Item = Result<Profile>>> {
            if self.query.is_none() {
                bail!("missing search query");
            }
            if self.query.as_ref().unwrap().len() < 3 {
                bail!(
                    "search query must contain at least 3 characters, got {}",
                    self.query.as_ref().unwrap().len()
                );
            }

            let client = PaginationClient::<SearchResults, Profile>::default();

            let url = "https://aoe4world.com/api/v0/players/search".parse()?;
            let url = self.query_params(url);

            let pages = client
                .into_pages_concurrent(PaginatedRequest::new(url), limit)
                .await?;
            Ok(pages.items())
        }

        fn query_params(&self, mut url: Url) -> Url {
            if let Some(query) = &self.query {
                url.query_pairs_mut()
                    .append_pair("query", query.to_string().as_str());
            }
            if let Some(exact) = self.exact {
                url.query_pairs_mut()
                    .append_pair("exact", exact.to_string().as_str());
            }
            url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::StreamExt;

    const HOUSEDHORSE_ID: u64 = 3176;
    const ONLY_CAMS_ID: u64 = 10433860;
    const ONLY_CAMS_NAME: &str = "🐪🐪🐪OnlyCams🐪🐪🐪";
    const DEBILS_NAME: &str = "DEBILS";

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test]
    async fn profile_api_smoke() {
        profile(ONLY_CAMS_ID)
            .get()
            .await
            .expect("API call should succeed");

        profile(HOUSEDHORSE_ID)
            .get()
            .await
            .expect("API call should succeed");
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test(flavor = "multi_thread")]
    async fn player_games_api_smoke() {
        let g: Vec<_> = profile_games(ONLY_CAMS_ID)
            .get(100)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        assert_eq!(100, g.len());
        for (i, game) in g.iter().enumerate() {
            assert!(game.is_ok(), "game {i} not ok: {game:?}")
        }

        let g: Vec<_> = profile_games(HOUSEDHORSE_ID)
            .get(100)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        assert_eq!(100, g.len());
        for (i, game) in g.iter().enumerate() {
            assert!(game.is_ok(), "game {i} not ok: {game:?}")
        }
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test(flavor = "multi_thread")]
    async fn global_games_api_smoke() {
        let g: Vec<_> = games()
            .get(100)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        assert_eq!(100, g.len());
        for (i, game) in g.iter().enumerate() {
            assert!(game.is_ok(), "game {i} not ok: {game:?}")
        }
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test(flavor = "multi_thread")]
    async fn search_api_smoke() {
        let profiles: Vec<_> = search(ONLY_CAMS_NAME)
            .with_exact(Some(true))
            .get(100)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        assert!(profiles.len() <= 100);
        for (i, profile) in profiles.iter().enumerate() {
            assert!(profile.is_ok(), "profile {i} not ok: {profile:?}")
        }

        let profiles: Vec<_> = search(DEBILS_NAME)
            .with_exact(Some(false))
            .get(100)
            .await
            .expect("API call should succeed")
            .collect()
            .await;
        assert!(profiles.len() <= 100);
        for (i, profile) in profiles.iter().enumerate() {
            assert!(profile.is_ok(), "profile {i} not ok: {profile:?}")
        }
    }
}
