//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

pub mod types;

use anyhow::Result;

use types::profile::ProfileStatsResponse;

/// Get profile stats for a player.
pub async fn profile_stats(profile_id: u64) -> Result<ProfileStatsResponse> {
    reqwest::get(format!(
        "https://aoe4world.com/api/v0/players/{}",
        profile_id
    ))
    .await?
    .json()
    .await
    .map_err(anyhow::Error::from)
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::{profile_stats, types::profile::ProfileStatsResponse};

    const ONLY_CAMS_ID: u64 = 10433860;

    const NEPTUNE_JSON: &str = include_str!("../testdata/neptune.json");
    const HOUSEDHORSE_JSON: &str = include_str!("../testdata/housedhorse.json");

    #[test]
    fn profile_deserialize_smoke() {
        let _: ProfileStatsResponse = from_str(NEPTUNE_JSON).expect("neptune should deserialize");
        let _: ProfileStatsResponse =
            from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
    }

    #[cfg_attr(not(feature = "test-api"), ignore)]
    #[tokio::test]
    async fn profile_deserialize_api() {
        profile_stats(ONLY_CAMS_ID.into())
            .await
            .expect("API call should succeed");
    }
}
