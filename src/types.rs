// SPDX-License-Identifier: Apache-2.0

//! Contains type definitions needed to interact with the AoE4 world API.

use std::ops::{Deref, DerefMut};

use serde::{de, Deserialize, Serialize};

pub mod civilization;
pub mod games;
pub mod profile;
pub mod rank;
pub mod search;

/// Wraps a [`reqwest::Url`] and implements [`serde::Deserialize`].
///
/// Derefs into a bare `Url` struct for convenience.
#[derive(Debug, PartialEq, Eq)]
pub struct Url {
    /// The inner URL.
    url: reqwest::Url,
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let url = reqwest::Url::parse(&s)
            .map_err(|e| de::Error::custom(format!("unable to parse URL: {}", e)))?;
        Ok(Self { url })
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.url.as_str();
        serializer.serialize_str(s)
    }
}

impl Deref for Url {
    type Target = reqwest::Url;

    fn deref(&self) -> &Self::Target {
        &self.url
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.url
    }
}

#[cfg(test)]
impl<'a> arbitrary::Arbitrary<'a> for Url {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Url {
            url: reqwest::Url::parse("https://www.example.com")
                .map_err(|_| arbitrary::Error::IncorrectFormat)?,
        })
    }
}
