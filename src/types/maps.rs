// SPDX-License-Identifier: Apache-2.0 or MIT

//! Contains type definitions related to aoe4 maps.

use serde::{Deserialize, Serialize};
use strum::VariantArray;

/// A map in AoE4.
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString,
)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Map {
    #[serde(rename = "Crafted Map")]
    #[strum(serialize = "Crafted Map")]
    CraftedMap,
    #[serde(rename = "Altai")]
    #[strum(serialize = "Altai")]
    Altai,
    #[serde(rename = "Ancient Spires")]
    #[strum(serialize = "Ancient Spires")]
    AncientSpires,
    #[serde(rename = "Archipelago")]
    #[strum(serialize = "Archipelago")]
    Archipelago,
    #[serde(rename = "Black Forest")]
    #[strum(serialize = "Black Forest")]
    BlackForest,
    #[serde(rename = "Boulder Bay")]
    #[strum(serialize = "Boulder Bay")]
    BoulderBay,
    #[serde(rename = "Confluence")]
    #[strum(serialize = "Confluence")]
    Confluence,
    #[serde(rename = "Danube River")]
    #[strum(serialize = "Danube River")]
    DanubeRiver,
    #[serde(rename = "Dry Arabia")]
    #[strum(serialize = "Dry Arabia")]
    DryArabia,
    #[serde(rename = "French Pass")]
    #[strum(serialize = "French Pass")]
    FrenchPass,
    #[serde(rename = "High View")]
    #[strum(serialize = "High View")]
    HighView,
    #[serde(rename = "Hill and Dale")]
    #[strum(serialize = "Hill and Dale")]
    HillAndDale,
    #[serde(rename = "King of the Hill")]
    #[strum(serialize = "King of the Hill")]
    KingOfTheHill,
    #[serde(rename = "Lipany")]
    #[strum(serialize = "Lipany")]
    Lipany,
    #[serde(rename = "Mongolian Heights")]
    #[strum(serialize = "Mongolian Heights")]
    MongolianHeights,
    #[serde(rename = "Mountain Pass")]
    #[strum(serialize = "Mountain Pass")]
    MountainPass,
    #[serde(rename = "Nagari")]
    #[strum(serialize = "Nagari")]
    Nagari,
    #[serde(rename = "Warring Islands")]
    #[strum(serialize = "Warring Islands")]
    WarringIslands,
    #[serde(rename = "MegaRandom")]
    #[strum(serialize = "MegaRandom")]
    MegaRandom,
    #[serde(rename = "The Pit")]
    #[strum(serialize = "The Pit")]
    ThePit,
    #[serde(rename = "Oasis")]
    #[strum(serialize = "Oasis")]
    Oasis,
    #[serde(alias = "Mediterranean")]
    #[serde(rename = "Baltic")]
    #[strum(serialize = "Baltic")]
    Baltic,
    #[serde(rename = "Forest Ponds")]
    #[strum(serialize = "Forest Ponds")]
    ForestPonds,
    #[serde(rename = "Wetlands")]
    #[strum(serialize = "Wetlands")]
    Wetlands,
    #[serde(rename = "Prairie")]
    #[strum(serialize = "Prairie")]
    Prairie,
    #[serde(rename = "Watering Holes")]
    #[strum(serialize = "Watering Holes")]
    WateringHoles,
    #[serde(rename = "Hideout")]
    #[strum(serialize = "Hideout")]
    Hideout,
    #[serde(rename = "Mountain Clearing")]
    #[strum(serialize = "Mountain Clearing")]
    MountainClearing,
    #[serde(rename = "Continental")]
    #[strum(serialize = "Continental")]
    Continental,
    #[serde(rename = "Marshland")]
    #[strum(serialize = "Marshland")]
    Marshland,
    #[serde(rename = "Four Lakes")]
    #[strum(serialize = "Four Lakes")]
    FourLakes,
    #[serde(rename = "Migration")]
    #[strum(serialize = "Migration")]
    Migration,
    #[serde(rename = "Volcanic Island")]
    #[strum(serialize = "Volcanic Island")]
    VolcanicIsland,
    #[serde(rename = "Golden Heights")]
    #[strum(serialize = "Golden Heights")]
    GoldenHeights,
    #[serde(rename = "African Waters")]
    #[strum(serialize = "African Waters")]
    AfricanWaters,
    #[serde(rename = "Thickets")]
    #[strum(serialize = "Thickets")]
    Thickets,
    #[serde(rename = "Golden Pit")]
    #[strum(serialize = "Golden Pit")]
    GoldenPit,
    #[serde(rename = "Cliffside")]
    #[strum(serialize = "Cliffside")]
    Cliffside,
    #[serde(rename = "Gorge")]
    #[strum(serialize = "Gorge")]
    Gorge,
    #[serde(rename = "Canal")]
    #[strum(serialize = "Canal")]
    Canal,
    #[serde(rename = "Glade")]
    #[strum(serialize = "Glade")]
    Glade,
    #[serde(rename = "Haywire")]
    #[strum(serialize = "Haywire")]
    Haywire,
    #[serde(rename = "Turtle Ridge")]
    #[strum(serialize = "Turtle Ridge")]
    TurtleRidge,
    #[serde(rename = "Rocky River")]
    #[strum(serialize = "Rocky River")]
    RockyRiver,
    #[serde(rename = "Himeyama")]
    #[strum(serialize = "Himeyama")]
    Himeyama,
    #[serde(rename = "Forts")]
    #[strum(serialize = "Forts")]
    Forts,
    #[serde(rename = "Hidden Valley")]
    #[strum(serialize = "Hidden Valley")]
    HiddenValley,
    #[serde(untagged)]
    #[strum(default)]
    #[cfg(not(test))]
    Unknown(String),
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_string().cmp(&other.to_string()))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl VariantArray for Map {
    const VARIANTS: &'static [Self] = &[
        Self::CraftedMap,
        Self::Altai,
        Self::AncientSpires,
        Self::Archipelago,
        Self::BlackForest,
        Self::BoulderBay,
        Self::Confluence,
        Self::DanubeRiver,
        Self::DryArabia,
        Self::FrenchPass,
        Self::HighView,
        Self::HillAndDale,
        Self::KingOfTheHill,
        Self::Lipany,
        Self::MongolianHeights,
        Self::MountainPass,
        Self::Nagari,
        Self::WarringIslands,
        Self::MegaRandom,
        Self::ThePit,
        Self::Oasis,
        Self::Baltic,
        Self::ForestPonds,
        Self::Wetlands,
        Self::Prairie,
        Self::WateringHoles,
        Self::Hideout,
        Self::MountainClearing,
        Self::Continental,
        Self::Marshland,
        Self::FourLakes,
        Self::Migration,
        Self::VolcanicIsland,
        Self::GoldenHeights,
        Self::AfricanWaters,
        Self::Thickets,
        Self::GoldenPit,
        Self::Cliffside,
        Self::Gorge,
        Self::Canal,
        Self::Glade,
        Self::Haywire,
        Self::TurtleRidge,
        Self::RockyRiver,
        Self::Himeyama,
        Self::Forts,
        Self::HiddenValley,
    ];
}

impl Map {
    /// Alias for [`MapType::Baltic`].
    #[allow(non_upper_case_globals)]
    pub const Mediterranean: Self = Self::Baltic;

    pub fn map_type(&self) -> MapType {
        match self {
            Map::CraftedMap => MapType::Unknown,
            Map::Altai => MapType::Land,
            Map::AncientSpires => MapType::Hybrid,
            Map::Archipelago => MapType::Water,
            Map::BlackForest => MapType::Hybrid,
            Map::BoulderBay => MapType::Hybrid,
            Map::Confluence => MapType::Hybrid,
            Map::DanubeRiver => MapType::Hybrid,
            Map::DryArabia => MapType::Land,
            Map::FrenchPass => MapType::Land,
            Map::HighView => MapType::Land,
            Map::HillAndDale => MapType::Land,
            Map::KingOfTheHill => MapType::Land,
            Map::Lipany => MapType::Land,
            Map::MongolianHeights => MapType::Hybrid,
            Map::MountainPass => MapType::Land,
            Map::Nagari => MapType::Hybrid,
            Map::WarringIslands => MapType::Water,
            Map::MegaRandom => MapType::Hybrid,
            Map::ThePit => MapType::Land,
            Map::Oasis => MapType::Hybrid,
            Map::Baltic => MapType::Hybrid,
            Map::ForestPonds => MapType::Hybrid,
            Map::Wetlands => MapType::Hybrid,
            Map::Prairie => MapType::Land,
            Map::WateringHoles => MapType::Hybrid,
            Map::Hideout => MapType::Land,
            Map::MountainClearing => MapType::Land,
            Map::Continental => MapType::Hybrid,
            Map::Marshland => MapType::Land,
            Map::FourLakes => MapType::Hybrid,
            Map::Migration => MapType::Water,
            Map::VolcanicIsland => MapType::Hybrid,
            Map::GoldenHeights => MapType::Hybrid,
            Map::AfricanWaters => MapType::Hybrid,
            Map::Thickets => MapType::Hybrid,
            Map::GoldenPit => MapType::Land,
            Map::Cliffside => MapType::Land,
            Map::Gorge => MapType::Land,
            Map::Canal => MapType::Hybrid,
            Map::Glade => MapType::Land,
            Map::Haywire => MapType::Land,
            Map::TurtleRidge => MapType::Land,
            Map::RockyRiver => MapType::Hybrid,
            Map::Himeyama => MapType::Land,
            Map::Forts => MapType::Hybrid,
            Map::HiddenValley => MapType::Land,
            #[cfg(not(test))]
            Map::Unknown(_) => MapType::Unknown,
        }
    }
}

/// A type of map in AoE4.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum::Display,
    strum::EnumString,
    strum::VariantArray,
    PartialOrd,
    Ord,
)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MapType {
    /// Custom or unknown maps don't have a canonical map type.
    Unknown,
    /// A land map.
    Land,
    /// A hybrid map.
    Hybrid,
    /// A water map.
    Water,
}

#[cfg(test)]
mod test_super {
    #![allow(unused_imports)]

    use crate::testutils::{test_enum_to_string, test_serde_roundtrip_prop};

    use super::*;

    test_serde_roundtrip_prop!(Map);
    test_serde_roundtrip_prop!(MapType);

    test_enum_to_string!(Map);
    test_enum_to_string!(MapType);
}
