## Architecture

This document covers major aspects of the prelate-rs architecture, including an explanation of:
- The structure of the project
- (More Coming Soon)

## Structure of the Project

- [`testdata`]: JSON files used in schema smoke tests
- [`src/lib.rs`]: Main library entry point, contains definitions for async functions used to fetch data from the API
- [`src/pagination.rs`]: Wrapper types to support transparent pagination, not exposed to the user
- [`src/testutils.rs`]: Helper functions for writing unit tests
- [`src/types`]: Various types used in the API schema

[`testdata`]: https://github.com/willfindlay/prelate-rs/tree/main/testdata
[`src/lib.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/lib.rs
[`src/pagination.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/pagination.rs
[`src/testutils.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/testutils.rs
[`src/types`]: https://github.com/willfindlay/prelate-rs/tree/main/src/types
