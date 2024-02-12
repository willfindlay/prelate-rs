// SPDX-License-Identifier: Apache-2.0 or MIT

//! API response types for player and profile stats.

pub use isocountry::CountryCode;
use serde_json::Value;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    ops::Deref,
};

use serde::{Deserialize, Serialize};

use crate::{
    profile, profile_games,
    query::{ProfileGamesQuery, ProfileQuery},
    types::rank::League,
};

use super::civilization::Civilization;

/// Player profile ID on aoe4world.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
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
    /// Returns a [`ProfileQuery`]. Used to get profile for a player.
    pub fn profile(&self) -> ProfileQuery {
        profile(self.0)
    }

    /// Constructs a query for the `/players/{profile_id}/games` endpoint for this [`ProfileId`].
    pub fn games(&self) -> ProfileGamesQuery {
        profile_games(self.0)
    }
}

/// Player profile and statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Profile {
    /// Name of the player.
    pub name: String,
    /// Profile ID of the player on aoe4world.
    pub profile_id: ProfileId,
    /// Steam ID of the player.
    pub steam_id: Option<String>,
    /// URL of the profile on aoe4world.
    pub site_url: Option<String>,
    /// Links to avatars used by the player.
    pub avatars: Option<Avatars>,
    /// Social information.
    pub social: Option<Social>,
    /// Country Code
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_country))]
    pub country: Option<CountryCode>,
    /// Statistics per game mode.
    #[serde(alias = "leaderboards")]
    pub modes: Option<GameModes>,
    /// [`chrono::DateTime`] when last game was played.
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
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
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Avatars {
    /// Small size.
    pub small: Option<String>,
    /// Medium size.
    pub medium: Option<String>,
    /// Full size.
    pub full: Option<String>,
}

/// Social information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Social {
    /// URL to the player's Twitch.
    pub twitch: Option<String>,
    /// URL to the player's YouTube.
    pub youtube: Option<String>,
    /// URL to the player's Liquipedia page.
    pub liquipedia: Option<String>,
    /// URL to the player's Twitter.
    pub twitter: Option<String>,
    /// URL to the player's Reddit.
    pub reddit: Option<String>,
    /// URL to the player's Instagram.
    pub instagram: Option<String>,
}

/// Statistics per game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GameModes {
    /// Solo ranked stats. Rating is ranked points.
    pub rm_solo: Option<GameModeStats>,
    /// Team ranked stats. Rating is ranked points.
    pub rm_team: Option<GameModeStats>,
    /// Deprecated.
    #[deprecated = "Use rm_solo instead."]
    pub rm_1v1: Option<GameModeStats>,
    /// 1v1 ranked stats. Rating is ELO.
    pub rm_1v1_elo: Option<GameModeStats>,
    /// 2v2 ranked stats. Rating is ELO.
    #[serde(alias = "rm_2v2")]
    pub rm_2v2_elo: Option<GameModeStats>,
    /// 3v3 ranked stats. Rating is ELO.
    #[serde(alias = "rm_3v3")]
    pub rm_3v3_elo: Option<GameModeStats>,
    /// 4v4 ranked stats. Rating is ELO.
    #[serde(alias = "rm_4v4")]
    pub rm_4v4_elo: Option<GameModeStats>,
    /// 1v1 quick match stats. Rating is ELO.
    pub qm_1v1: Option<GameModeStats>,
    /// 2v2 quick match stats. Rating is ELO.
    pub qm_2v2: Option<GameModeStats>,
    /// 3v3 quick match stats. Rating is ELO.
    pub qm_3v3: Option<GameModeStats>,
    /// 4v4 quick match stats. Rating is ELO.
    pub qm_4v4: Option<GameModeStats>,
    /// 1v1 Empire Wars quick match stats. Rating is ELO.
    pub qm_1v1_ew: Option<GameModeStats>,
    /// 2v2 Empire Wars quick match stats. Rating is ELO.
    pub qm_2v2_ew: Option<GameModeStats>,
    /// 3v3 Empire Wars quick match stats. Rating is ELO.
    pub qm_3v3_ew: Option<GameModeStats>,
    /// 4v4 Empire Wars quick match stats. Rating is ELO.
    pub qm_4v4_ew: Option<GameModeStats>,
    /// Custom stats.
    pub custom: Option<GameModeStats>,
}

/// Statistics for a game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GameModeStats {
    // Deprecation notice served by the API trips up our deny_unknown_fields attr during tests.
    #[cfg(test)]
    _notice_: Option<String>,
    /// Rating points or ELO.
    pub rating: Option<i64>,
    /// Max rating of all time.
    pub max_rating: Option<i64>,
    /// Max rating within the last 7 days.
    pub max_rating_7d: Option<i64>,
    /// Max rating within the last month.
    pub max_rating_1m: Option<i64>,
    /// Position on the leaderboard.
    pub rank: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i64>,
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
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// The player's league and division.
    pub rank_level: Option<League>,
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
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PreviousSeasonStats {
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// Position on the leaderboard.
    pub rank: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i64>,
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
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// The player's league and division.
    pub rank_level: Option<League>,
    /// Which season the stats are from.
    pub season: Option<u32>,
}

/// An entry in the player's rating history.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct RatingHistoryEntry {
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i64>,
    /// How many games have been played.
    pub games_count: Option<u32>,
    /// How many games have been won.
    pub wins_count: Option<u32>,
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// How many games have been disputed.
    pub disputes_count: Option<u32>,
    /// This field is populated the player has decayed between this match and the previous one. It contains the original rating after the decay but before the match was played.
    pub orig_rating: Option<u32>,
}

/// Per-Civilization stats.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
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
    pub game_length: Option<CivGameLengthStats>,
}

/// Per-Civilization game length stats.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
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
    // TODO: support this field properly
    #[cfg_attr(test, arbitrary(value = Vec::default()))]
    breakdown: Vec<HashMap<String, Value>>,
}

#[cfg(test)]
mod tests {
    use crate::testutils::{test_json, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(ProfileId);
    test_serde_roundtrip_prop!(Profile);
    test_serde_roundtrip_prop!(Avatars);
    test_serde_roundtrip_prop!(Social);
    test_serde_roundtrip_prop!(GameModes);
    test_serde_roundtrip_prop!(GameModeStats);
    test_serde_roundtrip_prop!(PreviousSeasonStats);
    test_serde_roundtrip_prop!(RatingHistoryEntry);
    test_serde_roundtrip_prop!(CivStats);
    test_serde_roundtrip_prop!(CivGameLengthStats);

    test_json!(
        Profile,
        "../../testdata/profile/neptune.json",
        neptune_profile
    );

    test_json!(
        Profile,
        "../../testdata/profile/housedhorse.json",
        housedhorse_profile
    );

    test_json!(Profile, "../../testdata/profile/jigly.json", jigly_profile);
}
