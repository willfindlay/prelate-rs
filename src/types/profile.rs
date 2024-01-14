// SPDX-License-Identifier: Apache-2.0 or MIT

//! API response types for player and profile stats.

use std::{collections::BTreeMap, fmt::Display, ops::Deref};

use anyhow::Result;
use futures::Stream;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{games, profile, types::rank::RankLeague, Game, Leaderboard};

use super::civilization::Civilization;

/// Player profile ID on aoe4world.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct ProfileId(u64);

impl Display for ProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<u64> for ProfileId {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl From<u64> for ProfileId {
    fn from(value: u64) -> Self {
        ProfileId(value)
    }
}

impl From<ProfileId> for u64 {
    fn from(value: ProfileId) -> Self {
        value.0
    }
}

impl ProfileId {
    /// Get the profile for this ProfileId.
    pub async fn profile(&self) -> Result<Profile> {
        profile(self.0).await
    }

    /// Get games for this ProfileId. Games are returned as an async stream.
    ///
    /// # Params
    /// - `leaderboard` is an optional leaderboard to be searched against (e.g.
    /// [`Leaderboard::RmTeam`]).
    /// - `opponent_ids` is an optional opponent profile ID to search against.
    /// - `since` is an optional datetime to search after.
    pub async fn games(
        &self,
        leaderboard: Option<Leaderboard>,
        opponent_id: Option<ProfileId>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<impl Stream<Item = Result<Game>>> {
        games(Some(self).cloned(), leaderboard, opponent_id, since).await
    }
}

/// Player profile and statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Profile {
    /// Name of the player.
    pub name: String,
    /// Profile ID of the player on aoe4world.
    pub profile_id: ProfileId,
    /// Steam ID of the player.
    pub steam_id: Option<String>,
    /// URL of the profile on aoe4world.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub site_url: Option<Url>,
    /// Links to avatars used by the player.
    pub avatars: Option<Avatars>,
    /// Social information.
    pub social: Option<Social>,
    /// Statistics per game mode.
    #[serde(alias = "leaderboards")]
    pub modes: Option<GameModes>,
}

impl Deref for Profile {
    type Target = ProfileId;

    fn deref(&self) -> &Self::Target {
        &self.profile_id
    }
}

/// Links to avatars used by the player.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Avatars {
    /// Small size.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub small: Option<Url>,
    /// Medium size.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub medium: Option<Url>,
    /// Full size.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub full: Option<Url>,
}

/// Social information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Social {
    /// URL to the player's Twitch.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub twitch: Option<Url>,
    /// URL to the player's YouTube.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub youtube: Option<Url>,
    /// URL to the player's Liquipedia page.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub liquipedia: Option<Url>,
    /// URL to the player's Twitter.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub twitter: Option<Url>,
    /// URL to the player's Reddit.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub reddit: Option<Url>,
    /// URL to the player's Instagram.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_url))]
    pub instagram: Option<Url>,
}

/// Statistics per game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct GameModes {
    /// Solo ranked stats. Rating is ranked points.
    pub rm_solo: Option<GameModeStats>,
    /// Team ranked stats. Rating is ranked points.
    pub rm_team: Option<GameModeStats>,
    /// 1v1 quick match stats. Rating is ELO.
    pub qm_1v1: Option<GameModeStats>,
    /// 2v2 quick match stats. Rating is ELO.
    pub qm_2v2: Option<GameModeStats>,
    /// 3v3 quick match stats. Rating is ELO.
    pub qm_3v3: Option<GameModeStats>,
    /// 4v4 quick match stats. Rating is ELO.
    pub qm_4v4: Option<GameModeStats>,
}

/// Statistics for a game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct GameModeStats {
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// Max rating of all time.
    pub max_rating: Option<u32>,
    /// Max rating within the last 7 days.
    pub max_rating_7d: Option<u32>,
    /// Max rating within the last month.
    pub max_rating_1m: Option<u32>,
    /// Position on the leaderboard.
    pub rank: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i32>,
    /// How many games have been played.
    pub games_count: Option<u32>,
    /// How many games have been won.
    pub wins_count: Option<u32>,
    /// How many games have been lost.
    pub losses_count: Option<u32>,
    /// How many games have been disputed.
    pub disputes_count: Option<u32>,
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// When the last game was played.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// The player's league and division.
    pub rank_level: Option<RankLeague>,
    /// The player's rating history. Maps Game ID to RatingHistoryEntry.
    #[serde(default)]
    pub rating_history: BTreeMap<String, RatingHistoryEntry>,
    /// Stats per-civ.
    #[serde(default)]
    pub civilizations: Vec<CivStats>,
    /// Which season the stats are from.
    pub season: Option<u32>,
    /// Previous season stats, if any. Note that this only exists in the context
    /// of rm_solo and rm_team for the current season.
    #[serde(default)]
    pub previous_seasons: Vec<PreviousSeasonStats>,
}

/// Statistics for previous season.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct PreviousSeasonStats {
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// Position on the leaderboard.
    pub rank: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i32>,
    /// How many games have been played.
    pub games_count: Option<u32>,
    /// How many games have been won.
    pub wins_count: Option<u32>,
    /// How many games have been lost.
    pub losses_count: Option<u32>,
    /// How many games have been disputed.
    pub disputes_count: Option<u32>,
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// When the last game was played.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// The player's league and division.
    pub rank_level: Option<RankLeague>,
    /// Which season the stats are from.
    pub season: Option<u32>,
}

/// An entry in the player's rating history.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct RatingHistoryEntry {
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i32>,
    /// How many games have been played.
    pub games_count: Option<u32>,
    /// How many games have been won.
    pub wins_count: Option<u32>,
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// How many games have been disputed.
    pub disputes_count: Option<u32>,
}

/// Per-Civilization stats.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct CivStats {
    /// The civilization.
    pub civilization: Option<Civilization>,
    /// Percentage of games won.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// Percentage of games where this civ was picked.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub pick_rate: Option<f64>,
    /// Number of games played with this civ.
    pub games_count: Option<u32>,
    /// Game length stats.
    pub games_length: Option<CivGameLengthStats>,
    // FIXME: add support for breakdown buckets
}

/// Per-Civilization game length stats.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct CivGameLengthStats {
    /// Average duration in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average: Option<f64>,
    /// Median duration in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub median: Option<f64>,
    /// Average duration for wins in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub wins_average: Option<f64>,
    /// Median duration for wins in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub wins_median: Option<f64>,
    /// Average duration for losses in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub losses_average: Option<f64>,
    /// Median duration for losses in seconds.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub losses_median: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testutils::assert_serde_roundtrip;

    use arbitrary::Arbitrary;
    use serde_json::from_str;

    const NEPTUNE_JSON: &str = include_str!("../../testdata/neptune.json");
    const HOUSEDHORSE_JSON: &str = include_str!("../../testdata/housedhorse.json");

    #[test]
    fn profile_serde_roundtrip() {
        fn prop(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<()> {
            let obj = Profile::arbitrary(u)?;
            assert_serde_roundtrip(obj);
            Ok(())
        }
        arbtest::builder().run(prop);
    }

    #[test]
    fn profile_examples_deserialize() {
        let neptune_profile: Profile = from_str(NEPTUNE_JSON).expect("neptune should deserialize");
        assert_serde_roundtrip(neptune_profile);

        let housedhorse_profile: Profile =
            from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
        assert_serde_roundtrip(housedhorse_profile);
    }
}
