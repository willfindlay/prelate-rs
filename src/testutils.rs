// SPDX-License-Identifier: Apache-2.0 or MIT

//! Utility functions for testing.

#![cfg(test)]

use std::fmt::Debug;

use pretty_assertions::assert_eq;
use serde::{de::DeserializeOwned, Serialize};

pub fn assert_serde_roundtrip<T>(obj: T)
where
    T: Serialize + DeserializeOwned + Debug + PartialEq,
{
    let obj_str = serde_json::to_string(&obj).expect("obj should serialize");
    let obj_de: T = serde_json::from_str(&obj_str).expect("obj should deserialize");
    assert_eq!(obj, obj_de, "serialization should be idempotent");
}

pub mod arbitrary_with {
    use arbitrary::Arbitrary;

    pub fn url(u: &mut arbitrary::Unstructured) -> arbitrary::Result<url::Url> {
        let s = String::arbitrary(u)?;
        let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        url::Url::parse(&format!("https://www.example.com/{s}"))
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }

    pub fn option_url(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Option<url::Url>> {
        if u.int_in_range(0..=1)? == 1 {
            return Ok(None);
        }
        Ok(Some(url(u)?))
    }

    pub fn clamped_option_f64(
        min: f64,
        max: f64,
    ) -> impl Fn(&mut arbitrary::Unstructured) -> arbitrary::Result<Option<f64>> {
        move |u: &mut arbitrary::Unstructured| -> arbitrary::Result<Option<f64>> {
            let steps = u32::MAX;
            let factor = (max - min) / (steps as f64);
            let random_int: u32 = u.int_in_range(0..=steps)?;
            let random = min + factor * (random_int as f64);
            Ok(Some(random))
        }
    }
}
