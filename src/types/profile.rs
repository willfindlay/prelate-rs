// SPDX-License-Identifier: Apache-2.0

//! API response types for player and profile stats.

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use anyhow::Result;
use futures::Stream;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{games, profile, types::rank::RankLeague, Game, Leaderboard};

/// Player profile ID on aoe4world.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub struct ProfileId(u64);

impl Deref for ProfileId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProfileId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        profile(**self).await
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
        opponent_id: Option<u64>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<impl Stream<Item = Result<Game>>> {
        games(**self, leaderboard, opponent_id, since).await
    }
}

/// Player profile and statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    #[cfg_attr(test, arbitrary(with = crate::testutils::arbitrary_with::clamped_option_f64(0.0, 100.0)))]
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
    use std::str::FromStr;

    use super::*;

    use crate::{testutils::assert_serde_roundtrip, types::rank::RankDivision};

    use arbitrary::Arbitrary;
    use pretty_assertions::assert_eq;
    use serde_json::from_str;

    const NEPTUNE_JSON: &str = include_str!("../../testdata/neptune.json");
    const HOUSEDHORSE_JSON: &str = include_str!("../../testdata/housedhorse.json");

    #[test]
    fn profile_examples_deserialize_smoke() {
        let _: Profile = from_str(NEPTUNE_JSON).expect("neptune should deserialize");

        let profile: Profile = from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
        let profile_expected = Profile {
            name: "[DEBILS] HousedHorse".into(),
            profile_id: 3176.into(),
            steam_id: Some("76561198094298920".into()),
            site_url: Some("http://aoe4world.com/players/3176".try_into().unwrap()),
            avatars: Some(Avatars {
                small: Some( "https://avatars.akamai.steamstatic.com/220b1f84a5204c3e79087cfa0a197045ec0fb0f2.jpg".try_into().unwrap()),
                medium: Some( "https://avatars.akamai.steamstatic.com/220b1f84a5204c3e79087cfa0a197045ec0fb0f2_medium.jpg".try_into().unwrap()),
                full: Some( "https://avatars.akamai.steamstatic.com/220b1f84a5204c3e79087cfa0a197045ec0fb0f2_full.jpg".try_into().unwrap())
            }),
            social: Some(Social {
                twitch: Some("https://www.twitch.tv/housedhorse".try_into().unwrap()),
                youtube: None,
                liquipedia: None,
                twitter: None,
                reddit: None,
                instagram: None
            }),
            modes: Some(GameModes {
                rm_solo: None,
                rm_team: Some(GameModeStats {
                    rating: Some(1742),
                    max_rating: Some(1742),
                    max_rating_7d: Some(1742),
                    max_rating_1m: Some(1742),
                    rank: Some(27),
                    streak: Some(5),
                    games_count: Some(123),
                    wins_count: Some(109),
                    losses_count: Some(14),
                    drops_count: Some(0),
                    last_game_at: Some(chrono::DateTime::from_str("2022-12-14T03:10:20.000Z").unwrap()),
                    win_rate: Some(88.6),
                    rank_level: Some(RankLeague::Conqueror(RankDivision::Three)),
                    rating_history: HashMap::from([
                        ("1669316350".to_owned(), RatingHistoryEntry {
                            rating: Some(1678),
                            streak: Some(7),
                            games_count: Some(113),
                            wins_count: Some(100),
                            drops_count: Some(0),
                        }),
                        ("1669317710".to_owned(), RatingHistoryEntry {
                            rating: Some(1687),
                            streak: Some(8),
                            games_count: Some(114),
                            wins_count: Some(101),
                            drops_count: Some(0),
                        }),
                        ("1669325297".to_owned(), RatingHistoryEntry {
                            rating: Some(1697),
                            streak: Some(9),
                            games_count: Some(115),
                            wins_count: Some(102),
                            drops_count: Some(0),
                        }),
                        ("1669419363".to_owned(), RatingHistoryEntry {
                            rating: Some(1715),
                            streak: Some(10),
                            games_count: Some(116),
                            wins_count: Some(103),
                            drops_count: Some(0),
                        }),
                        ("1669420489".to_owned(), RatingHistoryEntry {
                            rating: Some(1731),
                            streak: Some(11),
                            games_count: Some(117),
                            wins_count: Some(104),
                            drops_count: Some(0),
                        }),
                        ("1669471603".to_owned(), RatingHistoryEntry {
                            rating: Some(1708),
                            streak: Some(-1),
                            games_count: Some(118),
                            wins_count: Some(104),
                            drops_count: Some(0),
                        }),
                        ("1670108125".to_owned(), RatingHistoryEntry {
                            rating: Some(1709),
                            streak: Some(1),
                            games_count: Some(119),
                            wins_count: Some(105),
                            drops_count: Some(0),
                        }),
                        ("1670110151".to_owned(), RatingHistoryEntry {
                            rating: Some(1718),
                            streak: Some(2),
                            games_count: Some(120),
                            wins_count: Some(106),
                            drops_count: Some(0),
                        }),
                        ("1670112063".to_owned(), RatingHistoryEntry {
                            rating: Some(1719),
                            streak: Some(3),
                            games_count: Some(121),
                            wins_count: Some(107),
                            drops_count: Some(0),
                        }),
                        ("1670290359".to_owned(), RatingHistoryEntry {
                            rating: Some(1729),
                            streak: Some(4),
                            games_count: Some(122),
                            wins_count: Some(108),
                            drops_count: Some(0),
                        }),
                        ("1670987420".to_owned(), RatingHistoryEntry {
                            rating: Some(1742),
                            streak: Some(5),
                            games_count: Some(123),
                            wins_count: Some(109),
                            drops_count: Some(0),
                        }),
                ]) }),
                rm_1v1: None,
                rm_2v2: None,
                rm_3v3: None,
                rm_4v4: None,
                qm_1v1: None,
                qm_2v2: None,
                qm_3v3: None,
                qm_4v4: None,
                custom: None,
            }),
        };

        assert_eq!(profile, profile_expected);
        assert_serde_roundtrip(profile_expected);
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
