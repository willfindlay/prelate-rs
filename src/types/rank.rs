// SPDX-License-Identifier: Apache-2.0

//! Types related to a player's rank league.

use serde::{de, Deserialize, Serialize};

/// A player's rank league and division (e.g. Conq III).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum RankLeague {
    /// No rank.
    Unranked,
    /// Bronze X.
    Bronze(RankDivision),
    /// Silver X.
    Silver(RankDivision),
    /// Gold X.
    Gold(RankDivision),
    /// Plat X.
    Platinum(RankDivision),
    /// Diamond X.
    Diamond(RankDivision),
    /// Conq X.
    Conqueror(RankDivision),
}

impl<'de> Deserialize<'de> for RankLeague {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || s == "unranked" {
            return Ok(Self::Unranked);
        }

        // Split league and division strings at the _ character
        let (league, division) = s
            .split_once('_')
            .ok_or_else(|| de::Error::custom(format!("invalid rank string: {}", s)))?;

        // Parse division as an integer and map it onto a ranked division
        let division = division
            .parse::<u32>()
            .map_err(|e| de::Error::custom(format!("unable to parse division: {}", e)))?;
        let division = RankDivision::try_from(division).map_err(de::Error::custom)?;

        // Parse league
        let rank = match league {
            "bronze" => Self::Bronze(division),
            "silver" => Self::Silver(division),
            "gold" => Self::Gold(division),
            "platinum" => Self::Platinum(division),
            "diamond" => Self::Diamond(division),
            "conqueror" => Self::Conqueror(division),
            _ => {
                return Err(de::Error::custom(format!(
                    "invalid league string: {}",
                    league
                )))
            }
        };

        Ok(rank)
    }
}

impl Serialize for RankLeague {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (league, div) = match self {
            RankLeague::Unranked => return serializer.serialize_str("unranked"),
            RankLeague::Bronze(div) => ("bronze", div),
            RankLeague::Silver(div) => ("silver", div),
            RankLeague::Gold(div) => ("gold", div),
            RankLeague::Platinum(div) => ("platinum", div),
            RankLeague::Diamond(div) => ("diamond", div),
            RankLeague::Conqueror(div) => ("conqueror", div),
        };
        serializer.serialize_str(&format!(
            "{}_{}",
            league,
            serde_json::to_string(div)
                .map_err(|_| serde::ser::Error::custom(format!("invalid division")))?
        ))
    }
}

/// A player's division within their rank league.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum RankDivision {
    /// The lowest division within a league (e.g. Conqueror I).
    One,
    /// The middle division within a league (e.g. Conqueror II).
    Two,
    /// The highest division within a league (e.g. Conqueror III).
    Three,
}

impl<'de> Deserialize<'de> for RankDivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u32::deserialize(deserializer)?;
        RankDivision::try_from(n).map_err(de::Error::custom)
    }
}

impl Serialize for RankDivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let n: u32 = u32::from(*self);
        serializer.serialize_u32(n)
    }
}

impl TryFrom<u32> for RankDivision {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let division = match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => anyhow::bail!(
                "invalid rank division number: got {}, wanted 1 to 3 inclusive",
                value
            ),
        };
        Ok(division)
    }
}

impl From<RankDivision> for u32 {
    fn from(div: RankDivision) -> Self {
        match div {
            RankDivision::One => 1,
            RankDivision::Two => 2,
            RankDivision::Three => 3,
        }
    }
}
