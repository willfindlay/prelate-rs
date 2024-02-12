// SPDX-License-Identifier: Apache-2.0 or MIT

//! Contains type definitions needed to interact with the AoE4 world API.

use std::{collections::HashMap, ops::Deref};

use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pagination::{Paginated, Pagination};

use super::{
    games::Leaderboard,
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
