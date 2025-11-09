// SPDX-License-Identifier: Apache-2.0 or MIT

//! Definitions for civilizations in AoEIV.

use serde::{Deserialize, Serialize};

/// A civilization in AoEIV.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::Display,
    strum::VariantArray,
    strum::EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Civilization {
    English,
    French,
    HolyRomanEmpire,
    Rus,
    Mongols,
    Chinese,
    AbbasidDynasty,
    DelhiSultanate,
    Ottomans,
    Malians,
    Byzantines,
    Japanese,
    JeanneDarc,
    Ayyubids,
    ZhuXisLegacy,
    OrderOfTheDragon,
    KnightsTemplar,
    HouseOfLancaster,
    GoldenHorde,
    MacedonianDynasty,
    SengokuDaimyo,
    TughlaqDynasty,
}

impl PartialOrd for Civilization {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_string().cmp(&other.to_string()))
    }
}

impl Ord for Civilization {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(Civilization);

    test_enum_to_string!(Civilization);
}
