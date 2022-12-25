// SPDX-License-Identifier: Apache-2.0

//! Definitions for civilizations in AoEIV.

use serde::{Deserialize, Serialize};

/// A civilization in AoEIV.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
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
}
