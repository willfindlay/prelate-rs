// SPDX-License-Identifier: Apache-2.0

//! API response types for player and profile stats.

use std::collections::HashMap;

use serde::Deserialize;

use crate::types::{rank::RankLeague, Url};

/// Player profile and statistics.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProfileStatsResponse {
    /// Name of the player.
    pub name: String,
    /// Profile ID of the player on aeo4world.
    pub profile_id: u64,
    /// Steam ID of the player.
    pub steam_id: String,
    /// URL of the profile on aoe4world.
    pub site_url: Url,
    /// Links to avatars used by the player.
    pub avatars: Avatars,
    /// Social information.
    pub social: Social,
    /// Statistics per game mode.
    pub modes: GameModes,
}

/// Links to avatars used by the player.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Avatars {
    /// Small size.
    pub small: Url,
    /// Medium size.
    pub medium: Url,
    /// Full size.
    pub full: Url,
}

/// Social information.
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

/// Statistics per game mode.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
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
}

/// Statistics for a game mode.
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

/// An entry in the player's rating history.
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
