// SPDX-License-Identifier: Apache-2.0

//! Games played.

use std::fmt::Display;

use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    pagination::{Paginated, Pagination},
    profile,
    types::civilization::Civilization,
};

use super::profile::Profile;

/// Filters for games returned by the API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Filter {
    /// Filter by leaderboard category.
    pub leaderboard: Option<Leaderboard>,
    /// Filter over an opponent's profile ID.
    #[serde(default)]
    pub opponent_profile_id: Option<u64>,
    /// Filter by time played since a specific date.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub since: Option<chrono::DateTime<chrono::Utc>>,
}

impl Filter {
    pub(crate) fn query_params(&self, mut url: Url) -> Url {
        if let Some(ref leaderboard) = self.leaderboard {
            url.query_pairs_mut()
                .extend_pairs(&[("leaderboard", leaderboard.to_string())]);
        }
        if let Some(id) = self.opponent_profile_id {
            url.query_pairs_mut()
                .extend_pairs(&[("opponent_profile_id", id.to_string())]);
        }
        if let Some(since) = self.since {
            url.query_pairs_mut()
                .extend_pairs(&[("since", since.to_string())]);
        }
        url
    }
}

/// Games played and related statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub(crate) struct GamesPlayed {
    #[serde(flatten)]
    pagination: Pagination,
    games: Vec<Game>,
}

impl Paginated<Game> for GamesPlayed {
    fn pagination(&self) -> &Pagination {
        &self.pagination
    }

    fn data(self) -> Vec<Game> {
        self.games
    }
}

/// Information on a specific game.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Game {
    /// The ID of the game on aoe4world.
    pub game_id: Option<u32>,
    /// When the game was started.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When the state of the game was last updated.
    #[cfg_attr(test, arbitrary(value = Some(chrono::Utc::now())))]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// How long the game lasted in seconds.
    pub duration: Option<u32>,
    /// Map on which the game was played.
    pub map: Option<String>,
    /// The kind of game.
    pub kind: Option<GameKind>,
    /// Leaderboard the game counts towards.
    pub leaderboard: Option<Leaderboard>,
    /// Season in which the game was played.
    pub season: Option<u32>,
    /// Server on which the game was played.
    pub server: Option<String>,
    /// Patch on which the game was played.
    pub patch: Option<u32>,
    /// Average ELO rating of the game.
    pub average_rating: Option<f64>,
    /// Whether the match is still ongoing.
    /// True if and only if the match is still being played.
    pub ongoing: Option<bool>,
    /// Whether the match was just finished.
    /// True if and only if the match has finished but results have not yet been decided.
    pub just_finished: Option<bool>,
    /// The teams in the game.
    #[serde(default)]
    pub teams: Vec<Vec<TeamMember>>,
}

/// Type of game being played. Equivalent to [`Leaderboard`] but without `RmSolo` and
/// `RmTeam`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum GameKind {
    /// 1v1 ranked.
    #[serde(rename = "rm_1v1")]
    Rm1v1,
    /// 2v2 ranked.
    #[serde(rename = "rm_2v2")]
    Rm2v2,
    /// 3v3 ranked.
    #[serde(rename = "rm_3v3")]
    Rm3v3,
    /// 4v4 ranked.
    #[serde(rename = "rm_4v4")]
    Rm4v4,
    /// 1v1 quick match.
    #[serde(rename = "qm_1v1")]
    Qm1v1,
    /// 2v2 quick match.
    #[serde(rename = "qm_2v2")]
    Qm2v2,
    /// 3v3 quick match.
    #[serde(rename = "qm_3v3")]
    Qm3v3,
    /// 4v4 quick match.
    #[serde(rename = "qm_4v4")]
    Qm4v4,
    /// A custom game.
    #[serde(rename = "custom")]
    Custom,
}

impl Display for GameKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

/// Which leaderboard a game was played on. Equivalent to [`GameKind`] but with the
/// addition of `RmSolo` and `RmTeam`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum Leaderboard {
    /// Solo ranked.
    #[serde(rename = "rm_solo")]
    RmSolo,
    /// Team ranked.
    #[serde(rename = "rm_team")]
    RmTeam,
    /// 1v1 ranked.
    #[serde(rename = "rm_1v1")]
    Rm1v1,
    /// 2v2 ranked.
    #[serde(rename = "rm_2v2")]
    Rm2v2,
    /// 3v3 ranked.
    #[serde(rename = "rm_3v3")]
    Rm3v3,
    /// 4v4 ranked.
    #[serde(rename = "rm_4v4")]
    Rm4v4,
    /// 1v1 quick match.
    #[serde(rename = "qm_1v1")]
    Qm1v1,
    /// 2v2 quick match.
    #[serde(rename = "qm_2v2")]
    Qm2v2,
    /// 3v3 quick match.
    #[serde(rename = "qm_3v3")]
    Qm3v3,
    /// 4v4 quick match.
    #[serde(rename = "qm_4v4")]
    Qm4v4,
    /// A custom game.
    #[serde(rename = "custom")]
    Custom,
}

impl Display for Leaderboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

/// The result of a match. Either a win or a loss.
///
/// No-Result outcomes are not currently supported by the aoe4world API, but this may
/// change in the future.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum GameResult {
    Win,
    Loss,
    Unknown,
}

/// Wrapper around a Player who is a member of a team.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct TeamMember {
    pub player: Option<Player>,
}

/// A player in the game.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Player {
    /// Name of the player.
    pub name: Option<String>,
    /// Profile ID of the player on aoe4world.
    pub profile_id: Option<u64>,
    /// Result of the game.
    pub result: Option<GameResult>,
    /// Civilization played in the game.
    pub civilization: Option<Civilization>,
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// Rating points or ELO gained or lost.
    pub rating_diff: Option<i32>,
}

impl Player {
    /// Fetch the profile information for this player, if it exists.
    pub async fn profile(&self) -> Result<Option<Profile>> {
        match self.profile_id {
            Some(id) => Ok(Some(profile(id).await?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testutils::assert_serde_roundtrip;

    use arbitrary::Arbitrary;
    use serde_json::from_str;

    const NEPTUNE_GAMES_JSON: &str = include_str!("../../testdata/neptune-games.json");

    #[test]
    fn game_examples_deserialize_smoke() {
        let _: GamesPlayed =
            from_str(NEPTUNE_GAMES_JSON).expect("neptune games should deserialize");
    }

    #[test]
    fn game_serde_rountrip() {
        fn prop(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<()> {
            let obj = GamesPlayed::arbitrary(u)?;
            assert_serde_roundtrip(obj);
            Ok(())
        }
        arbtest::builder().run(prop);
    }
}
