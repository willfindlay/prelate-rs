// SPDX-License-Identifier: Apache-2.0

//! Definitions for civilizations in AoEIV.

use serde::Deserialize;

/// A civilization in AoEIV.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
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
