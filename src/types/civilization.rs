// SPDX-License-Identifier: Apache-2.0 or MIT

//! Definitions for civilizations in AoEIV.

use serde::{Deserialize, Serialize};

/// A civilization in AoEIV.
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString,
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
    #[serde(untagged)]
    #[strum(default)]
    #[cfg(not(test))]
    Unknown(String),
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

impl strum::VariantArray for Civilization {
    const VARIANTS: &'static [Self] = &[
        Self::English,
        Self::French,
        Self::HolyRomanEmpire,
        Self::Rus,
        Self::Mongols,
        Self::Chinese,
        Self::AbbasidDynasty,
        Self::DelhiSultanate,
        Self::Ottomans,
        Self::Malians,
        Self::Byzantines,
        Self::Japanese,
        Self::JeanneDarc,
        Self::Ayyubids,
        Self::ZhuXisLegacy,
        Self::OrderOfTheDragon,
        Self::KnightsTemplar,
        Self::HouseOfLancaster,
        Self::GoldenHorde,
        Self::MacedonianDynasty,
        Self::SengokuDaimyo,
        Self::TughlaqDynasty,
        // Note: Unknown variant intentionally excluded
    ];
}

#[cfg(test)]
mod test_super {
    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(Civilization);

    test_enum_to_string!(Civilization);

    #[test]
    fn test_known_civilization_deserialization() {
        // Verify known civilizations deserialize correctly
        let json = r#""english""#;
        let civ: Civilization = serde_json::from_str(json).unwrap();
        assert_eq!(civ, Civilization::English);

        let json = r#""golden_horde""#;
        let civ: Civilization = serde_json::from_str(json).unwrap();
        assert_eq!(civ, Civilization::GoldenHorde);
    }
}
