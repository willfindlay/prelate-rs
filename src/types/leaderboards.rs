// SPDX-License-Identifier: Apache-2.0 or MIT

//! Contains type definitions needed to interact with the AoE4 world API.

use std::{collections::HashMap, ops::Deref};

use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pagination::{Paginated, Pagination};

use super::{
    profile::{Avatars, ProfileId, Social},
    rank::League,
};

/// Global games.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct LeaderboardPages {
    #[serde(flatten)]
    pagination: Pagination,
    #[serde(flatten)]
    info: LeaderboardInfo,
    #[serde(default)]
    players: Vec<LeaderboardEntry>,
    #[serde(default)]
    #[cfg_attr(test, arbitrary(value = HashMap::default()))]
    filters: HashMap<String, Value>,
}

impl Paginated<LeaderboardEntry> for LeaderboardPages {
    fn pagination(&self) -> &Pagination {
        &self.pagination
    }

    fn data(self) -> Vec<LeaderboardEntry> {
        self.players
    }
}

/// Which leaderboard a game was played on. Similar to [`crate::games::GameKind`] but with the
/// addition of `RmSolo` and `RmTeam`.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::Display,
    strum::VariantArray,
    strum::EnumString,
)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Leaderboard {
    /// Solo ranked.
    #[serde(rename = "rm_solo")]
    #[serde(alias = "rm_1v1")]
    #[strum(serialize = "rm_solo")]
    RmSolo,
    /// Team ranked.
    #[serde(rename = "rm_team")]
    #[strum(serialize = "rm_team")]
    RmTeam,
    /// 1v1 quick match.
    #[serde(rename = "qm_1v1")]
    #[strum(serialize = "qm_1v1")]
    Qm1v1,
    /// 2v2 quick match.
    #[serde(rename = "qm_2v2")]
    #[strum(serialize = "qm_2v2")]
    Qm2v2,
    /// 3v3 quick match.
    #[serde(rename = "qm_3v3")]
    #[strum(serialize = "qm_3v3")]
    Qm3v3,
    /// 4v4 quick match.
    #[serde(rename = "qm_4v4")]
    #[strum(serialize = "qm_4v4")]
    Qm4v4,
    /// 1v1 empire wars quick match.
    #[serde(rename = "qm_1v1_ew")]
    #[strum(serialize = "qm_1v1_ew")]
    Qm1v1Ew,
    /// 2v2 empire wars quick match.
    #[serde(rename = "qm_2v2_ew")]
    #[strum(serialize = "qm_2v2_ew")]
    Qm2v2Ew,
    /// 3v3 empire wars quick match.
    #[serde(rename = "qm_3v3_ew")]
    #[strum(serialize = "qm_3v3_ew")]
    Qm3v3Ew,
    /// 4v4 empire wars quick match.
    #[serde(rename = "qm_4v4_ew")]
    #[strum(serialize = "qm_4v4_ew")]
    Qm4v4Ew,
    /// Console solo ranked.
    #[serde(rename = "rm_solo_console")]
    #[serde(alias = "rm_1v1_console")]
    #[strum(serialize = "rm_solo_console")]
    RmSoloConsole,
    // /// Console team ranked.
    // #[serde(rename = "rm_team_console")]
    // RmTeamConsole,
    /// Console 1v1 quick match.
    #[serde(rename = "qm_1v1_console")]
    #[strum(serialize = "qm_1v1_console")]
    Qm1v1Console,
    /// Console 2v2 quick match.
    #[serde(rename = "qm_2v2_console")]
    #[strum(serialize = "qm_2v2_console")]
    Qm2v2Console,
    /// Console 3v3 quick match.
    #[serde(rename = "qm_3v3_console")]
    #[strum(serialize = "qm_3v3_console")]
    Qm3v3Console,
    /// Console 4v4 quick match.
    #[serde(rename = "qm_4v4_console")]
    #[strum(serialize = "qm_4v4_console")]
    Qm4v4Console,
    /// Console 1v1 empire wars quick match.
    #[serde(rename = "qm_1v1_ew_console")]
    #[strum(serialize = "qm_1v1_ew_console")]
    Qm1v1EwConsole,
    /// Console 2v2 empire wars quick match.
    #[serde(rename = "qm_2v2_ew_console")]
    #[strum(serialize = "qm_2v2_ew_console")]
    Qm2v2EwConsole,
    /// Console 3v3 empire wars quick match.
    #[serde(rename = "qm_3v3_ew_console")]
    #[strum(serialize = "qm_3v3_ew_console")]
    Qm3v3EwConsole,
    /// Console 4v4 empire wars quick match.
    #[serde(rename = "qm_4v4_ew_console")]
    #[strum(serialize = "qm_4v4_ew_console")]
    Qm4v4EwConsole,
}

impl Leaderboard {
    /// Alias for [`Leaderboard::RmSolo`].
    #[allow(non_upper_case_globals)]
    pub const Rm1v1: Leaderboard = Leaderboard::RmSolo;
}

/// A ranked leaderboard.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct LeaderboardInfo {
    /// [`Leaderboard`] type.
    pub key: Option<Leaderboard>,
    /// Query used when fetching the leaderboard.
    pub query: Option<String>,
    /// Name of the leaderboard.
    pub name: Option<String>,
    /// Short name of the leaderboard.
    pub short_name: Option<String>,
    /// URL of the leaderboard on aoe4world.
    pub site_url: Option<String>,
}

/// An entry in a leaderboard. Includes a subset of
/// [`crate::types::profile::Profile`] and ranking information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LeaderboardEntry {
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
    /// Country Code
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::option_country))]
    pub country: Option<CountryCode>,
    /// Social information.
    pub social: Option<Social>,
    /// URL of the player's Twitch stream.
    pub twitch_url: Option<String>,
    /// Is the player's Twitch live?
    pub twitch_is_live: Option<bool>,
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
    /// The player's league and division.
    pub rank_level: Option<League>,
    /// How many games have been won or lost in a row.
    pub streak: Option<i64>,
    /// How many games have been played.
    pub games_count: Option<u32>,
    /// How many games have been won.
    pub wins_count: Option<u32>,
    /// How many games have been lost.
    pub losses_count: Option<u32>,
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// [`chrono::DateTime`] when last game was played.
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// Last change in rating.
    pub last_rating_change: Option<i64>,
}

impl Deref for LeaderboardEntry {
    type Target = ProfileId;

    fn deref(&self) -> &Self::Target {
        &self.profile_id
    }
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_json, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(Leaderboard);
    test_serde_roundtrip_prop!(LeaderboardInfo);
    test_serde_roundtrip_prop!(LeaderboardEntry);
    test_serde_roundtrip_prop!(LeaderboardPages);

    test_json!(
        LeaderboardPages,
        "../../testdata/leaderboards/rm_solo.json",
        rm_solo
    );

    test_json!(
        LeaderboardPages,
        "../../testdata/leaderboards/rm_team.json",
        rm_team
    );
}
