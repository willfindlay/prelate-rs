//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{de, Deserialize};

pub struct Url {
    url: reqwest::Url,
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let url = reqwest::Url::parse(&s)
            .map_err(|e| de::Error::custom(format!("unable to parse URL: {}", e)))?;
        Ok(Self { url })
    }
}

impl Deref for Url {
    type Target = reqwest::Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Avatars {
    pub small: Url,
    pub medium: Url,
    pub full: Url,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllGameModeStats {
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
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GameModeStats {
    /// Rating points or ELO.
    pub rating: u32,
    /// Max rating of all time.
    pub max_rating: u32,
    /// Max rating within the last 7 days.
    pub max_rating_7d: u32,
    /// Max rating within the last month.
    pub max_rating_1m: u32,
    /// Position on the leaderboard.
    pub rank: u32,
    /// How many games have been won or lost in a row.
    pub streak: i32,
    /// How many games have been played.
    pub games_count: u32,
    /// How many games have been won.
    pub wins_count: u32,
    /// How many games have been lost.
    pub losses_count: u32,
    /// How many games have been dropped.
    pub drops_count: u32,
    /// When the last game was played.
    pub last_game_at: chrono::DateTime<chrono::Utc>,
    /// Win rate as a percentage out of 100.
    pub win_rate: f64,
    /// The player's league and division.
    pub rank_level: Option<RankLeague>,
    /// The player's rating history. Maps Game ID to RatingHistoryEntry.
    pub rating_history: HashMap<String, RatingHistoryEntry>,
    // TODO: add civilization entries, see neptune.json for an example.
}

pub enum RankLeague {
    /// No rank.
    Unranked,
    /// Bronze X.
    Bronze(RankDivision),
    /// Silver X.
    Silver(RankDivision),
    /// Gold X.
    Gold(RankDivision),
    /// Plat X.
    Platinum(RankDivision),
    /// Diamond X.
    Diamond(RankDivision),
    /// Conq X.
    Conqueror(RankDivision),
}

impl<'de> Deserialize<'de> for RankLeague {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || s == "unranked" {
            return Ok(Self::Unranked);
        }

        // Split league and division strings at the _ character
        let (league, division) = s
            .split_once('_')
            .ok_or_else(|| de::Error::custom(format!("invalid rank string: {}", s)))?;

        // Parse division as an integer and map it onto a ranked division
        let division = division
            .parse::<u32>()
            .map_err(|e| de::Error::custom(format!("unable to parse division: {}", e)))?;
        let division = RankDivision::try_from(division).map_err(de::Error::custom)?;

        // Parse league
        let rank = match league {
            "bronze" => Self::Bronze(division),
            "silver" => Self::Silver(division),
            "gold" => Self::Gold(division),
            "platinum" => Self::Platinum(division),
            "diamond" => Self::Diamond(division),
            "conqueror" => Self::Conqueror(division),
            _ => {
                return Err(de::Error::custom(format!(
                    "invalid league string: {}",
                    league
                )))
            }
        };

        Ok(rank)
    }
}

pub enum RankDivision {
    /// The lowest division within a league (e.g. Conqueror I).
    One,
    /// The middle division within a league (e.g. Conqueror II).
    Two,
    /// The highest division within a league (e.g. Conqueror III).
    Three,
}

impl<'de> Deserialize<'de> for RankDivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u32::deserialize(deserializer)?;
        RankDivision::try_from(n).map_err(de::Error::custom)
    }
}

impl TryFrom<u32> for RankDivision {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let division = match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => anyhow::bail!(
                "invalid rank division number: got {}, wanted 1 to 3 inclusive",
                value
            ),
        };
        Ok(division)
    }
}

impl From<RankDivision> for u32 {
    fn from(div: RankDivision) -> Self {
        match div {
            RankDivision::One => 1,
            RankDivision::Two => 2,
            RankDivision::Three => 3,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RatingHistoryEntry {
    /// Rating points or ELO.
    pub rating: u32,
    /// How many games have been won or lost in a row.
    pub streak: i32,
    /// How many games have been played.
    pub games_count: u32,
    /// How many games have been won.
    pub wins_count: u32,
    /// How many games have been dropped.
    pub drops_count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Player {
    pub name: String,
    pub profile_id: u64,
    pub steam_id: String,
    pub site_url: Url,
    pub avatars: Avatars,
    pub social: Social,
    pub modes: AllGameModeStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    const NEPTUNE_JSON: &str = include_str!("../testdata/neptune.json");
    const HOUSEDHORSE_JSON: &str = include_str!("../testdata/housedhorse.json");

    #[test]
    fn profile_deserialize_smoke() {
        let _: Player = from_str(NEPTUNE_JSON).expect("neptune should deserialize");
        let _: Player = from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
    }

    #[cfg_attr(not(feature = "api"), ignore)]
    fn profile_deserialize_api() {
        todo!("make some actual API calls here")
    }
}
