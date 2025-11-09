// SPDX-License-Identifier: Apache-2.0 or MIT

//! Types related to a player's rank league.

use serde::{Deserialize, Serialize};

/// A player's rank league and division (e.g. Conq III).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    strum::Display,
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

    /// Unknown league.
    #[serde(untagged)]
    #[strum(default)]
    #[cfg(not(test))]
    Unknown(String),
}

impl strum::VariantArray for League {
    const VARIANTS: &'static [Self] = &[
        Self::Unranked,
        Self::Bronze1,
        Self::Bronze2,
        Self::Bronze3,
        Self::Silver1,
        Self::Silver2,
        Self::Silver3,
        Self::Gold1,
        Self::Gold2,
        Self::Gold3,
        Self::Platinum1,
        Self::Platinum2,
        Self::Platinum3,
        Self::Diamond1,
        Self::Diamond2,
        Self::Diamond3,
        Self::Conqueror1,
        Self::Conqueror2,
        Self::Conqueror3,
        Self::Conqueror4,
        // Note: Unknown variant intentionally excluded
    ];
}

impl League {
    /// Is the league unranked?
    pub fn is_unranked(&self) -> bool {
        matches!(self, League::Unranked)
    }

    /// Is the league bronze?
    pub fn is_bronze(&self) -> bool {
        matches!(self, League::Bronze1 | League::Bronze2 | League::Bronze3)
    }

    /// Is the league silver?
    pub fn is_silver(&self) -> bool {
        matches!(self, League::Silver1 | League::Silver2 | League::Silver3)
    }

    /// Is the league gold?
    pub fn is_gold(&self) -> bool {
        matches!(self, League::Gold1 | League::Gold2 | League::Gold3)
    }

    /// Is the league platinum?
    pub fn is_platinum(&self) -> bool {
        matches!(
            self,
            League::Platinum1 | League::Platinum2 | League::Platinum3
        )
    }

    /// Is the league diamond?
    pub fn is_diamond(&self) -> bool {
        matches!(self, League::Diamond1 | League::Diamond2 | League::Diamond3)
    }

    /// Is the league conqueror?
    pub fn is_conqueror(&self) -> bool {
        matches!(
            self,
            League::Conqueror1 | League::Conqueror2 | League::Conqueror3 | League::Conqueror4
        )
    }

    /// Is this a metal league?
    pub fn is_metal(&self) -> bool {
        matches!(
            self,
            League::Bronze1
                | League::Bronze2
                | League::Bronze3
                | League::Silver1
                | League::Silver2
                | League::Silver3
                | League::Gold1
                | League::Gold2
                | League::Gold3
                | League::Platinum1
                | League::Platinum2
                | League::Platinum3
        )
    }
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(League);

    test_enum_to_string!(League);
}
