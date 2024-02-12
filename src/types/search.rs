// SPDX-License-Identifier: Apache-2.0 or MIT

//! Search for players.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    pagination::{Paginated, Pagination},
    types::profile::Profile,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct SearchResults {
    #[serde(flatten)]
    pagination: Pagination,
    #[serde(default)]
    players: Vec<Profile>,
    #[serde(default)]
    #[cfg_attr(test, arbitrary(value = HashMap::default()))]
    filters: HashMap<String, Value>,
}

impl Paginated<Profile> for SearchResults {
    fn pagination(&self) -> &Pagination {
        &self.pagination
    }

    fn data(self) -> Vec<Profile> {
        self.players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testutils::{test_json, test_serde_roundtrip_prop};

    test_serde_roundtrip_prop!(SearchResults);

    test_json!(
        SearchResults,
        "../../testdata/search/barbecue.json",
        barbecue_search
    );

    test_json!(
        SearchResults,
        "../../testdata/search/onlycams.json",
        onlycams_search
    );

    test_json!(
        SearchResults,
        "../../testdata/search/jigly.json",
        jigly_search
    );
}
