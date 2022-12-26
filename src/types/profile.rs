// SPDX-License-Identifier: Apache-2.0

//! API response types for player and profile stats.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{rank::RankLeague, Url};

/// Player profile and statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Profile {
    /// Name of the player.
    pub name: Option<String>,
    /// Profile ID of the player on aoe4world.
    pub profile_id: Option<u64>,
    /// Steam ID of the player.
    pub steam_id: Option<String>,
    /// URL of the profile on aoe4world.
    pub site_url: Option<Url>,
    /// Links to avatars used by the player.
    pub avatars: Option<Avatars>,
    /// Social information.
    pub social: Option<Social>,
    /// When the last game was played.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Statistics per game mode.
    #[serde(alias = "leaderboards")]
    pub modes: Option<GameModes>,
}

/// Links to avatars used by the player.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Avatars {
    /// Small size.
    pub small: Option<Url>,
    /// Medium size.
    pub medium: Option<Url>,
    /// Full size.
    pub full: Option<Url>,
}

/// Social information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Social {
    /// URL to the player's Twitch.
    pub twitch: Option<Url>,
    /// URL to the player's YouTube.
    pub youtube: Option<Url>,
    /// URL to the player's Liquipedia page.
    pub liquipedia: Option<Url>,
    /// URL to the player's Twitter.
    pub twitter: Option<Url>,
    /// URL to the player's Reddit.
    pub reddit: Option<Url>,
    /// URL to the player's Instagram.
    pub instagram: Option<Url>,
}

/// Statistics per game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct GameModes {
    /// Solo ranked stats. Rating is ranked points.
    pub rm_solo: Option<GameModeStats>,
    /// Team ranked stats. Rating is ranked points.
    pub rm_team: Option<GameModeStats>,
    /// 1v1 ranked stats. Rating is ELO.
    pub rm_1v1: Option<GameModeStats>,
    /// 2v2 ranked stats. Rating is ELO.
    pub rm_2v2: Option<GameModeStats>,
    /// 3v3 ranked stats. Rating is ELO.
    pub rm_3v3: Option<GameModeStats>,
    /// 4v4 ranked stats. Rating is ELO.
    pub rm_4v4: Option<GameModeStats>,
    /// 1v1 quick match stats. Rating is ELO.
    pub qm_1v1: Option<GameModeStats>,
    /// 2v2 quick match stats. Rating is ELO.
    pub qm_2v2: Option<GameModeStats>,
    /// 3v3 quick match stats. Rating is ELO.
    pub qm_3v3: Option<GameModeStats>,
    /// 4v4 quick match stats. Rating is ELO.
    pub qm_4v4: Option<GameModeStats>,
    /// Custom games. Rating is ELO.
    pub custom: Option<GameModeStats>,
}

/// Statistics for a game mode.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    /// How many games have been dropped.
    pub drops_count: Option<u32>,
    /// When the last game was played.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub last_game_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Win rate as a percentage out of 100.
    #[cfg_attr(test, arbitrary(with = crate::testutils::some_clamped_arbitrary_f64(0.0, 100.0)))]
    pub win_rate: Option<f64>,
    /// The player's league and division.
    pub rank_level: Option<RankLeague>,
    /// The player's rating history. Maps Game ID to RatingHistoryEntry.
    #[serde(default)]
    pub rating_history: HashMap<String, RatingHistoryEntry>,
    // TODO: add civilization entries, see neptune.json for an example.
}

/// An entry in the player's rating history.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    fn profile_examples_deserialize_smoke() {
        let _: Profile = from_str(NEPTUNE_JSON).expect("neptune should deserialize");
        let _: Profile = from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
    }

    #[test]
    fn profile_serde_rountrip() {
        fn prop(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<()> {
            let obj = Profile::arbitrary(u)?;
            assert_serde_roundtrip(obj);
            Ok(())
        }
        arbtest::builder().run(prop);
    }
}
