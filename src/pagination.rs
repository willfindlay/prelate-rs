// SPDX-License-Identifier: Apache-2.0 or MIT

//! Abstractions over pagination.

use std::{collections::HashMap, marker::PhantomData};

use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use page_turner::prelude::*;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

/// Default concurrency to use when making paginated requests.
const DEFAULT_PAGES_CONCURRENCY: usize = 8;

/// Default count per page to use as the limit query parameter for paginated data.
const DEFAULT_COUNT_PER_PAGE: usize = 50;

/// Pagination info for paginated data.
///
/// This is used as part of the transparent pagination streaming logic.
/// Should be embedded into paginated data using `#[serde(flatten)]`.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub count: u32,
    pub total_count: Option<u32>,
    pub offset: u32,
}

/// Implement this trait for paginated data so that we can transparently stream it.
pub(crate) trait Paginated<T> {
    /// Returns a reference to pagination info.
    fn pagination(&self) -> &Pagination;
    /// Consumes self and returns a Vec containing all the paginated data.
    fn data(self) -> Vec<T>;
}

/// A request for paginated data.
#[derive(new)]
pub(crate) struct PaginatedRequest {
    url: Url,
    #[new(value = "1")]
    page: u32,
}

impl RequestAhead for PaginatedRequest {
    fn next_request(&self) -> Self {
        Self {
            url: self.url.clone(),
            page: self.page + 1,
        }
    }
}

/// A dummy client for paginated data.
pub(crate) struct PaginationClient<T, U> {
    _dummy1: PhantomData<T>,
    _dummy2: PhantomData<U>,
}

impl<T, U> Default for PaginationClient<T, U> {
    fn default() -> Self {
        Self {
            _dummy1: Default::default(),
            _dummy2: Default::default(),
        }
    }
}

#[async_trait]
impl<T: Send + Sync + DeserializeOwned + Paginated<U>, U: Send + Sync> PageTurner<PaginatedRequest>
    for PaginationClient<T, U>
{
    type PageItem = U;
    type PageError = anyhow::Error;

    async fn turn_page(
        &self,
        mut request: PaginatedRequest,
    ) -> PageTurnerOutput<Self, PaginatedRequest> {
        request.url.query_pairs_mut().extend_pairs(&[
            ("limit", DEFAULT_COUNT_PER_PAGE.to_string()),
            ("page", request.page.to_string()),
        ]);

        let res: T = reqwest::get(request.url.clone())
            .await?
            .error_for_status()?
            .json()
            .await?;
        let pagination = res.pagination();

        if pagination.count + pagination.offset < pagination.total_count.unwrap_or(u32::MAX) {
            request.page += 1;
            Ok(TurnedPage::next(res.data(), request))
        } else {
            Ok(TurnedPage::last(res.data()))
        }
    }
}

impl<T: Send + Sync + DeserializeOwned + Paginated<U> + 'static, U: Send + Sync + 'static>
    PaginationClient<T, U>
{
    /// Returns a concurrent stream of pages.
    ///
    /// Number of pages is optimized by issuing a dummy query at the beginning to find out
    /// how much data we actually have.
    pub(crate) async fn into_pages_concurrent(
        self,
        mut request: PaginatedRequest,
    ) -> Result<PagesStream<'static, U, anyhow::Error>> {
        request
            .url
            .query_pairs_mut()
            .extend_pairs(&[("limit", "1"), ("page", "1")]);

        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
        #[serde(rename_all = "snake_case")]
        struct PaginationDummy {
            #[serde(flatten)]
            pagination: Pagination,
            #[serde(flatten)]
            rest: HashMap<String, Value>,
        }

        // Grab initial information about pagination
        let res: PaginationDummy = reqwest::get(request.url.clone())
            .await?
            .error_for_status()?
            .json()
            .await?;
        let res = res.pagination;

        // Ceiling division to get total number of pages
        let limit = res
            .total_count
            .map(|total_count| {
                Limit::Pages(((total_count + res.per_page - 1) / res.per_page) as usize)
            })
            .unwrap_or(Limit::None);

        Ok(self.into_pages_ahead(DEFAULT_PAGES_CONCURRENCY, limit, request))
    }
}
