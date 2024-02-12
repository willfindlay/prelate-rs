// SPDX-License-Identifier: Apache-2.0 or MIT

//! Definitions for civilizations in AoEIV.

use serde::{Deserialize, Serialize};

/// A civilization in AoEIV.
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::VariantArray,
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
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use strum::VariantArray;

    use super::*;

    test_serde_roundtrip_prop!(Civilization);

    test_enum_to_string!(Civilization);
}
