//! prelate-rs is an async-ready library wrapper around the [aoe4world] API.
//!
//! Use it to retrieve game statistics, player information, and general awesomeness from
//! aoe4world in your Rust applications.
//!
//! [aoe4world]: https://aoe4world.com/api

pub mod types;

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::types::profile::ProfileStatsResponse;

    const NEPTUNE_JSON: &str = include_str!("../testdata/neptune.json");
    const HOUSEDHORSE_JSON: &str = include_str!("../testdata/housedhorse.json");

    #[test]
    fn profile_deserialize_smoke() {
        let _: ProfileStatsResponse = from_str(NEPTUNE_JSON).expect("neptune should deserialize");
        let _: ProfileStatsResponse =
            from_str(HOUSEDHORSE_JSON).expect("housedhorse should deserialize");
    }

    #[cfg_attr(not(feature = "api"), ignore)]
    fn profile_deserialize_api() {
        todo!("make some actual API calls here")
    }
}
