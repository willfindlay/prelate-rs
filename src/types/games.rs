// SPDX-License-Identifier: Apache-2.0

//! Games played.

use std::{fmt::Display, ops::Deref};

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    pagination::{Paginated, Pagination},
    types::{
        civilization::Civilization,
        profile::{Profile, ProfileId},
    },
};

/// Filters for games returned by the API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Filter {
    /// Filter by leaderboard category.
    pub leaderboard: Option<Leaderboard>,
    /// Filter over an opponent's profile ID.
    #[serde(default)]
    pub opponent_profile_id: Option<ProfileId>,
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
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
    pub average_rating: Option<f64>,
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

/// Wrapper around a Player. This is unfortunately needed due to the schema of the
/// aoe4world API.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
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

/// A player in the game.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct Player {
    /// Profile information for the player.
    #[serde(flatten)]
    pub profile: Profile,
    /// Game information for the player.
    #[serde(flatten)]
    pub game_info: PlayerGameInfo,
}

/// A player in the game.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct PlayerGameInfo {
    /// Result of the game.
    pub result: Option<GameResult>,
    /// Civilization played in the game.
    pub civilization: Option<Civilization>,
    /// Rating points or ELO.
    pub rating: Option<u32>,
    /// Rating points or ELO gained or lost.
    pub rating_diff: Option<i32>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    use crate::testutils::assert_serde_roundtrip;

    use arbitrary::Arbitrary;
    use pretty_assertions::assert_eq;
    use serde_json::from_str;

    const NEPTUNE_GAMES_JSON: &str = include_str!("../../testdata/neptune-games.json");

    #[test]
    fn game_examples_deserialize_smoke() {
        let games: GamesPlayed =
            from_str(NEPTUNE_GAMES_JSON).expect("neptune games should deserialize");
        assert_eq!(games.games.len(), 50);

        let game = games.games.get(0).expect("first game should be present");
        let game_expected = Game {
            game_id: Some(56783543),
            started_at: Some(chrono::DateTime::from_str("2022-12-20T14:10:13.000Z").unwrap()),
            updated_at: Some(chrono::DateTime::from_str("2022-12-20T14:45:55.713Z").unwrap()),
            duration: Some(1450),
            map: Some("Forest Ponds".into()),
            kind: Some(GameKind::Rm4v4),
            leaderboard: Some(Leaderboard::RmTeam),
            season: Some(3),
            server: Some("Korea".into()),
            patch: Some(148),
            average_rating: Some(1632f64),
            ongoing: Some(false),
            just_finished: Some(false),
            teams: vec![
                vec![
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "Kyo".into(),
                                profile_id: 106457.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Win),
                                civilization: Some(Civilization::Chinese),
                                rating: Some(1875),
                                rating_diff: Some(21),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "(✧ᴗ✧) CDSG.MeomaikA".into(),
                                profile_id: 6961598.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Win),
                                civilization: Some(Civilization::Mongols),
                                rating: Some(1613),
                                rating_diff: Some(20),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "[TLCT] Nhà Cái Từ Châu Âu".into(),
                                profile_id: 10438052.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Win),
                                civilization: Some(Civilization::French),
                                rating: Some(1588),
                                rating_diff: Some(22),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "Nyako~".into(),
                                profile_id: 11395443.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Win),
                                civilization: Some(Civilization::AbbasidDynasty),
                                rating: Some(1060),
                                rating_diff: Some(27),
                            },
                        },
                    },
                ],
                vec![
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "布偶".into(),
                                profile_id: 11658402.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Loss),
                                civilization: Some(Civilization::AbbasidDynasty),
                                rating: Some(1545),
                                rating_diff: Some(-35),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "A catty cat".into(),
                                profile_id: 10019352.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Loss),
                                civilization: Some(Civilization::Mongols),
                                rating: Some(1805),
                                rating_diff: Some(-36),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "neptune".into(),
                                profile_id: 4635035.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Loss),
                                civilization: Some(Civilization::Malians),
                                rating: Some(1785),
                                rating_diff: Some(-48),
                            },
                        },
                    },
                    PlayerWrapper {
                        player: Player {
                            profile: Profile {
                                name: "T r ico".into(),
                                profile_id: 7304568.into(),
                                steam_id: None,
                                site_url: None,
                                avatars: None,
                                social: None,
                                modes: None,
                            },
                            game_info: PlayerGameInfo {
                                result: Some(GameResult::Loss),
                                civilization: Some(Civilization::English),
                                rating: Some(1783),
                                rating_diff: Some(-33),
                            },
                        },
                    },
                ],
            ],
        };

        assert_eq!(game, &game_expected);
        assert_serde_roundtrip(game_expected);
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
