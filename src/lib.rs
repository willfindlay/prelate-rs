//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

use std::ops::{Deref, DerefMut};

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
            .map_err(|e| de::Error::custom(format!("unable to parse URL: {e}")))?;
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
    // TODO: What goes in here?`
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllGameModeStats {
    pub rm_solo: Option<GameModeStats>,
    pub rm_team: Option<GameModeStats>,
    pub rm_1v1: Option<GameModeStats>,
    pub rm_2v2: Option<GameModeStats>,
    pub rm_3v3: Option<GameModeStats>,
    pub rm_4v4: Option<GameModeStats>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GameModeStats {
    pub rating: u32,
    pub max_rating: u32,
    pub max_rating_7d: u32,
    pub max_rating_1m: u32,
    pub rank: u32,
    pub streak: u32,
    pub games_count: u32,
    pub wins_count: u32,
    pub losses_count: u32,
    pub drops_count: u32,
    pub last_game_at: chrono::DateTime<chrono::Utc>,
    pub win_rate: u32,
    // TODO: rank level enum?
    //pub rank_level: ,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProfileStats {
    pub name: String,
    pub profile_id: u32,
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

    const PROFILE_JSON: &str = include_str!("../testdata/profile.json");

    #[test]
    fn profile_deserialize_smoke() {
        let _: ProfileStats = from_str(PROFILE_JSON).expect("should deserialize");
    }
}
