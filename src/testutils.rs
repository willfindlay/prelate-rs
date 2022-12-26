// SPDX-License-Identifier: Apache-2.0

//! Utility functions for testing.

#![cfg(test)]

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

pub fn assert_serde_roundtrip<T>(obj: T)
where
    T: Serialize + DeserializeOwned + Debug + PartialEq,
{
    let obj_str = serde_json::to_string(&obj).expect("obj should serialize");
    println!("json: {}", obj_str);
    let obj_de: T = serde_json::from_str(&obj_str).expect("obj should deserialize");
    println!("struct: {:?}", obj_de);
    assert_eq!(obj, obj_de, "serialization should be idempotent");
}

pub fn some_clamped_arbitrary_f64(
    min: f64,
    max: f64,
) -> impl Fn(&mut arbitrary::Unstructured) -> arbitrary::Result<Option<f64>> {
    move |u: &mut arbitrary::Unstructured| -> arbitrary::Result<Option<f64>> {
        let steps = u32::MAX;
        let factor = (max - min) as f64 / (steps as f64);
        let random_int: u32 = u.int_in_range(0..=steps)?;
        let random = min as f64 + factor * (random_int as f64);
        Ok(Some(random))
    }
}
