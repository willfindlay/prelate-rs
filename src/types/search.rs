// SPDX-License-Identifier: Apache-2.0

//! Search for players.

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    pagination::{Paginated, Pagination},
    types::profile::Profile,
};

/// Filters for players returned by the API.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SearchResults {
    #[serde(flatten)]
    pagination: Pagination,
    players: Vec<Profile>,
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

    use serde_json::from_str;

    const SEARCH_RESULTS_JSON: &str = include_str!("../../testdata/search-results.json");
    const ONLYCAMS_SEARCH_RESULTS_JSON: &str =
        include_str!("../../testdata/onlycams-search-results.json");

    #[test]
    fn search_examples_deserialize_smoke() {
        let _: SearchResults =
            from_str(SEARCH_RESULTS_JSON).expect("search results should deserialize");
        let _: SearchResults = from_str(ONLYCAMS_SEARCH_RESULTS_JSON)
            .expect("OnlyCams search results should deserialize");
    }
}
