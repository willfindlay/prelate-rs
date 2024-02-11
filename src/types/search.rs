// SPDX-License-Identifier: Apache-2.0 or MIT

//! Search for players.

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    pagination::{Paginated, Pagination},
    types::profile::Profile,
};

/// Filters for players returned by the API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Filter {
    /// Search query.
    pub query: Option<String>,
    /// Should the results exactly match the query.
    pub exact: Option<bool>,
}

impl Filter {
    pub(crate) fn query_params(&self, mut url: Url) -> Url {
        if let Some(query) = &self.query {
            url.query_pairs_mut()
                .extend_pairs(&[("query", query.to_string())]);
        }
        if let Some(exact) = self.exact {
            url.query_pairs_mut()
                .extend_pairs(&[("exact", exact.to_string())]);
        }
        url
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct SearchResults {
    #[serde(flatten)]
    pagination: Pagination,
    players: Vec<Profile>,
    filters: Option<Filter>,
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
