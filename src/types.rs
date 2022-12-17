//! Contains type definitions needed to interact with the AoE4 world API.

use std::ops::{Deref, DerefMut};

use serde::{de, Deserialize};

pub mod profile;
pub mod rank;

/// Wraps a [`reqwest::Url`] and implements [`serde::Deserialize`].
///
/// Derefs into a bare `Url` struct for convenience.
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
