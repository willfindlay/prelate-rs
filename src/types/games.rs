// SPDX-License-Identifier: Apache-2.0 or MIT

//! Games played.

use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    pagination::{Paginated, Pagination},
    query::ProfileQuery,
    types::{civilization::Civilization, profile::ProfileId},
};

use super::{leaderboards::Leaderboard, maps::Map};

/// Filters for games returned by the API.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    strum::VariantArray,
    strum::Display,
    strum::EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum GamesOrder {
    StartedAt,
    UpdatedAt,
}

/// Global games.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct GlobalGames {
    #[serde(flatten)]
    pagination: Pagination,
    #[serde(default)]
    games: Vec<Game>,
    #[serde(default)]
    #[cfg_attr(test, arbitrary(value = HashMap::default()))]
    filters: HashMap<String, Value>,
}

impl Paginated<Game> for GlobalGames {
    fn pagination(&self) -> &Pagination {
        &self.pagination
    }

    fn data(self) -> Vec<Game> {
        self.games
    }
}

/// Per-profile games.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct ProfileGames {
    #[serde(flatten)]
    pagination: Pagination,
    #[serde(default)]
    games: Vec<Game>,
    #[serde(default)]
    #[cfg_attr(test, arbitrary(value = HashMap::default()))]
    filters: HashMap<String, Value>,
}

impl Paginated<Game> for ProfileGames {
    fn pagination(&self) -> &Pagination {
        &self.pagination
    }

    fn data(self) -> Vec<Game> {
        self.games
    }
}

/// Information on a specific game.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Game {
    /// The ID of the game on aoe4world.
    pub game_id: u32,
    /// When the game was started.
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When the state of the game was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// How long the game lasted in seconds.
    pub duration: Option<u32>,
    /// Map on which the game was played.
    pub map: Option<Map>,
    /// The kind of game.
    pub kind: Option<GameKind>,
    /// Leaderboard of the game.
    pub leaderboard: Option<Leaderboard>,
    /// Leaderboard used to determine MMR for this game.
    pub mmr_leaderboard: Option<Leaderboard>,
    /// Season in which the game was played.
    pub season: Option<u32>,
    /// Server on which the game was played.
    pub server: Option<String>,
    /// Patch on which the game was played.
    pub patch: Option<u32>,
    /// Average rating of the game.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average_rating: Option<f64>,
    /// Rating deviation of the game.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average_rating_deviation: Option<f64>,
    /// Average ELO of the game.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average_mmr: Option<f64>,
    /// ELO deviation of the game.
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average_mmr_deviation: Option<f64>,
    /// Whether the match is still ongoing.
    /// True if and only if the match is still being played.
    pub ongoing: Option<bool>,
    /// Whether the match was just finished.
    /// True if and only if the match has finished but results have not yet been decided.
    pub just_finished: Option<bool>,
    /// The teams in the game.
    #[serde(default)]
    pub teams: Vec<Vec<PlayerWrapper>>,
}

/// Type of game being played. Equivalent to [`Leaderboard`] but without `RmSolo` and
/// `RmTeam`.
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
    PartialOrd,
    Ord,
)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum GameKind {
    /// 1v1 ranked.
    #[serde(rename = "rm_1v1")]
    #[strum(serialize = "rm_1v1")]
    Rm1v1,
    /// 2v2 ranked.
    #[serde(rename = "rm_2v2")]
    #[strum(serialize = "rm_2v2")]
    Rm2v2,
    /// 3v3 ranked.
    #[serde(rename = "rm_3v3")]
    #[strum(serialize = "rm_3v3")]
    Rm3v3,
    /// 4v4 ranked.
    #[serde(rename = "rm_4v4")]
    #[strum(serialize = "rm_4v4")]
    Rm4v4,

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

    /// 1v1 nomad quick match.
    #[serde(rename = "qm_1v1_nomad")]
    #[strum(serialize = "qm_1v1_nomad")]
    Qm1v1Nomad,
    /// 2v2 nomad quick match.
    #[serde(rename = "qm_2v2_nomad")]
    #[strum(serialize = "qm_2v2_nomad")]
    Qm2v2Nomad,
    /// 3v3 nomad quick match.
    #[serde(rename = "qm_3v3_nomad")]
    #[strum(serialize = "qm_3v3_nomad")]
    Qm3v3Nomad,
    /// 4v4 nomad quick match.
    #[serde(rename = "qm_4v4_nomad")]
    #[strum(serialize = "qm_4v4_nomad")]
    Qm4v4Nomad,

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

    /// Console 1v1 ranked.
    #[serde(rename = "rm_1v1_console")]
    #[strum(serialize = "rm_1v1_console")]
    Rm1v1Console,
    /// Console 2v2 ranked.
    #[serde(rename = "rm_2v2_console")]
    #[strum(serialize = "rm_2v2_console")]
    Rm2v2Console,
    /// Console 3v3 ranked.
    #[serde(rename = "rm_3v3_console")]
    #[strum(serialize = "rm_3v3_console")]
    Rm3v3Console,
    /// Console 4v4 ranked.
    #[serde(rename = "rm_4v4_console")]
    #[strum(serialize = "rm_4v4_console")]
    Rm4v4Console,

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

    /// Console 1v1 nomad quick match.
    #[serde(rename = "qm_1v1_nomad_console")]
    #[strum(serialize = "qm_1v1_nomad_console")]
    Qm1v1NomadConsole,
    /// Console 2v2 nomad quick match.
    #[serde(rename = "qm_2v2_nomad_console")]
    #[strum(serialize = "qm_2v2_nomad_console")]
    Qm2v2NomadConsole,
    /// Console 3v3 nomad quick match.
    #[serde(rename = "qm_3v3_nomad_console")]
    #[strum(serialize = "qm_3v3_nomad_console")]
    Qm3v3NomadConsole,
    /// Console 4v4 nomad quick match.
    #[serde(rename = "qm_4v4_nomad_console")]
    #[strum(serialize = "qm_4v4_nomad_console")]
    Qm4v4NomadConsole,

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

    /// FFA quick match.
    #[serde(rename = "qm_ffa")]
    #[strum(serialize = "qm_ffa")]
    QmFfa,
    /// Empires Wars FFA quick match.
    #[serde(rename = "qm_ffa_ew")]
    #[strum(serialize = "qm_ffa_ew")]
    QmFfaEw,
    /// Nomad FFA quick match.
    #[serde(rename = "qm_ffa_nomad")]
    #[strum(serialize = "qm_ffa_nomad")]
    QmFfaNomad,

    /// Console FFA quick match.
    #[serde(rename = "qm_ffa_console")]
    #[strum(serialize = "qm_ffa_console")]
    QmFfaConsole,
    /// Console Empires Wars FFA quick match.
    #[serde(rename = "qm_ffa_ew_console")]
    #[strum(serialize = "qm_ffa_ew_console")]
    QmFfaEwConsole,
    /// Console Nomad FFA quick match.
    #[serde(rename = "qm_ffa_nomad_console")]
    #[strum(serialize = "qm_ffa_nomad_console")]
    QmFfaNomadConsole,

    /// Console A custom game.
    #[serde(rename = "custom")]
    #[strum(serialize = "custom")]
    Custom,
}

/// The result of a match. Either a win or a loss.
///
/// No-Result outcomes are not currently supported by the aoe4world API, but this may
/// change in the future.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::Display,
    strum::EnumString,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum GameResult {
    Unknown,
    #[serde(rename = "noresult")]
    #[strum(serialize = "noresult")]
    NoResult,
    Loss,
    Win,
}

/// Wrapper around a Player. This is unfortunately needed due to the schema of the
/// aoe4world API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PlayerWrapper {
    pub player: Player,
}

impl Deref for PlayerWrapper {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}

impl From<PlayerWrapper> for Player {
    fn from(value: PlayerWrapper) -> Self {
        value.player
    }
}

/// Input type for a player.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    strum::VariantArray,
    strum::Display,
    strum::EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum InputType {
    Keyboard,
    Controller,
}

/// A player in the game.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Player {
    /// Name of the player.
    pub name: String,
    /// Profile ID of the player on aoe4world.
    pub profile_id: ProfileId,
    /// Result of the game.
    pub result: Option<GameResult>,
    /// Civilization played in the game.
    pub civilization: Option<Civilization>,
    /// Did the player select "random civ".
    pub civilization_randomized: Option<bool>,
    /// Rating points.
    pub rating: Option<u32>,
    /// Rating points gained or lost.
    pub rating_diff: Option<i64>,
    /// ELO.
    pub mmr: Option<i64>,
    /// ELO gained or lost.
    pub mmr_diff: Option<i64>,
    /// Input type (keyboard or controller).
    pub input_type: Option<InputType>,
}

impl Player {
    /// Returns a [`ProfileQuery`]. Used to get profile for this [`Player`].
    pub fn profile(&self) -> ProfileQuery {
        self.profile_id.profile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testutils::{test_enum_to_string, test_json, test_serde_roundtrip_prop};

    test_serde_roundtrip_prop!(GamesOrder);
    test_serde_roundtrip_prop!(GlobalGames);
    test_serde_roundtrip_prop!(ProfileGames);
    test_serde_roundtrip_prop!(Game);
    test_serde_roundtrip_prop!(GameKind);
    test_serde_roundtrip_prop!(GameResult);
    test_serde_roundtrip_prop!(PlayerWrapper);
    test_serde_roundtrip_prop!(Player);

    test_json!(
        ProfileGames,
        "../../testdata/games/neptune.json",
        neptune_games
    );

    test_json!(ProfileGames, "../../testdata/games/jigly.json", jigly_games);

    test_json!(
        GlobalGames,
        "../../testdata/games/global.json",
        global_games
    );

    test_json!(
        GlobalGames,
        "../../testdata/games/global_2024_03_18.json",
        global_games_2024_03_18
    );

    test_json!(
        GlobalGames,
        "../../testdata/games/games_negative_mmr.json",
        negative_mmr
    );

    test_enum_to_string!(GameKind);
    test_enum_to_string!(Leaderboard);
    test_enum_to_string!(GamesOrder);
    test_enum_to_string!(GameResult);

    #[test]
    fn test_foo() {}
}
