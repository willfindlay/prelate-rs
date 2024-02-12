// SPDX-License-Identifier: Apache-2.0 or MIT

//! Types related to a player's rank league.

use serde::{Deserialize, Serialize};

/// A player's rank league and division (e.g. Conq III).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    strum::Display,
    strum::VariantArray,
    strum::EnumString,
)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum League {
    /// No rank.
    #[serde(rename = "unranked")]
    #[strum(serialize = "unranked")]
    Unranked,
    #[serde(rename = "bronze_1")]
    #[strum(serialize = "bronze_1")]
    Bronze1,
    #[serde(rename = "bronze_2")]
    #[strum(serialize = "bronze_2")]
    Bronze2,
    #[serde(rename = "bronze_3")]
    #[strum(serialize = "bronze_3")]
    Bronze3,
    #[serde(rename = "silver_1")]
    #[strum(serialize = "silver_1")]
    Silver1,
    #[serde(rename = "silver_2")]
    #[strum(serialize = "silver_2")]
    Silver2,
    #[serde(rename = "silver_3")]
    #[strum(serialize = "silver_3")]
    Silver3,
    #[serde(rename = "gold_1")]
    #[strum(serialize = "gold_1")]
    Gold1,
    #[serde(rename = "gold_2")]
    #[strum(serialize = "gold_2")]
    Gold2,
    #[serde(rename = "gold_3")]
    #[strum(serialize = "gold_3")]
    Gold3,
    #[serde(rename = "platinum_1")]
    #[strum(serialize = "platinum_1")]
    Platinum1,
    #[serde(rename = "platinum_2")]
    #[strum(serialize = "platinum_2")]
    Platinum2,
    #[serde(rename = "platinum_3")]
    #[strum(serialize = "platinum_3")]
    Platinum3,
    #[serde(rename = "diamond_1")]
    #[strum(serialize = "diamond_1")]
    Diamond1,
    #[serde(rename = "diamond_2")]
    #[strum(serialize = "diamond_2")]
    Diamond2,
    #[serde(rename = "diamond_3")]
    #[strum(serialize = "diamond_3")]
    Diamond3,
    #[serde(rename = "conqueror_1")]
    #[strum(serialize = "conqueror_1")]
    Conqueror1,
    #[serde(rename = "conqueror_2")]
    #[strum(serialize = "conqueror_2")]
    Conqueror2,
    #[serde(rename = "conqueror_3")]
    #[strum(serialize = "conqueror_3")]
    Conqueror3,
    /// Reserved for professional players.
    #[serde(rename = "conqueror_4")]
    #[strum(serialize = "conqueror_4")]
    Conqueror4,
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(League);

    test_enum_to_string!(League);
}
