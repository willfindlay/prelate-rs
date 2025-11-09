# Architecture

This document covers major aspects of the prelate-rs architecture, including:
- The structure of the project
- Core architectural patterns
- Data flow and pagination system
- Type system design
- Testing strategy

## Project Structure

```
prelate-rs/
├── src/
│   ├── lib.rs              # Main library entry point with public API
│   ├── pagination.rs       # Internal pagination machinery
│   ├── testutils.rs        # Test utilities and macros
│   └── types/              # Type definitions for API schemas
│       ├── civilization.rs # Civilization enums (22 civilizations)
│       ├── games.rs        # Game data structures
│       ├── leaderboards.rs # Leaderboard types
│       ├── maps.rs         # Map definitions (~50 maps)
│       ├── profile.rs      # Player profiles and stats
│       ├── rank.rs         # League/rank system
│       └── search.rs       # Search results
├── testdata/               # JSON files for schema validation tests
└── .github/workflows/      # CI/CD workflows
```

### Module Descriptions

- **[`src/lib.rs`]**: Main library entry point exposing public API through convenience functions (`profile()`, `search()`, `global_games()`, etc.) and the `query` module containing query builders
- **[`src/pagination.rs`]**: Internal pagination system using the `page-turner` crate to provide transparent concurrent pagination to users
- **[`src/testutils.rs`]**: Test macros and utilities for property-based testing and JSON schema validation
- **[`src/types/`]**: Type definitions matching the aoe4world API schema, organized by domain
- **[`testdata/`]**: Real API response JSON files used for schema validation tests

## Core Architectural Patterns

### 1. Query Builder Pattern

All API endpoints use the builder pattern powered by `derive-setters`:

```rust
#[derive(Setters, Default)]
#[setters(prefix = "with_")]
#[setters(into)]
pub struct ProfileGamesQuery {
    profile_id: Option<ProfileId>,
    game_kind: Option<Vec<GameKind>>,
    leaderboard: Option<Vec<Leaderboard>>,
    opponent_profile_id: Option<ProfileId>,
    since: Option<chrono::DateTime<chrono::Utc>>,
}
```

**Benefits:**
- Fluent, chainable API
- Optional parameters with sensible defaults
- Type-safe parameter validation at compile time
- Easy to extend with new parameters

**Implementation Pattern:**
1. Query builder struct with `Option<T>` fields
2. `.get(limit)` method that validates and executes query
3. Private `query_params()` method to build URL query string
4. Top-level convenience function wrapping the builder

### 2. Transparent Pagination System

The pagination system is the most sophisticated part of the architecture, making paginated API endpoints feel like simple streams.

**Key Components:**

- **`Pagination` struct** (`src/pagination.rs:28-34`): Contains API pagination metadata
  ```rust
  pub(crate) struct Pagination {
      pub page: u32,
      pub per_page: u32,
      pub count: u32,
      pub total_count: Option<u32>,
      pub offset: u32,
  }
  ```

- **`Paginated<T>` trait** (`src/pagination.rs:36-42`): Implemented by paginated response types
  ```rust
  pub(crate) trait Paginated<T> {
      fn pagination(&self) -> &Pagination;
      fn data(self) -> Vec<T>;
  }
  ```

- **`PaginationClient<T, U>`** (`src/pagination.rs:62-129`): Generic pagination handler implementing `PageTurner` trait

**Pagination Flow:**

1. User calls `.get(limit)` on query builder
2. Creates `PaginationClient<ResponseType, ItemType>` with item limit
3. Client implements `PageTurner::turn_page()`:
   - Adds `limit` and `page` query params to URL
   - Makes HTTP request via `reqwest`
   - Deserializes JSON response
   - Checks `total_count` vs `count + offset` to determine if more pages exist
   - Returns `TurnedPage::next(data, request)` or `TurnedPage::last(data)`
4. `into_pages_concurrent()` calculates total pages: `(limit + per_page - 1) / per_page`
5. Fetches up to 8 pages concurrently (configurable via `DEFAULT_PAGES_CONCURRENCY`)
6. Returns `Stream<Item = Result<T>>` that user can collect/iterate

**Key Optimizations:**
- Pre-calculates total pages needed for exact concurrency
- Default 50 items per page minimizes API calls
- Concurrent fetching reduces latency
- Stream-based interface allows lazy consumption

### 3. Type-Safe API Modeling

All API types follow consistent patterns:

**Derive Macros:**
```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Profile {
    // fields...
}
```

**Enum Patterns:**
```rust
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq,
    strum::Display, strum::VariantArray, strum::EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Civilization {
    English,
    French,
    // ... 20 more variants
}
```

**Benefits:**
- `#[serde(rename_all = "snake_case")]`: Automatic field name conversion
- `#[cfg_attr(test, derive(arbitrary::Arbitrary))]`: Property-based testing
- `#[cfg_attr(test, serde(deny_unknown_fields))]`: Strict schema validation in tests
- `strum` derives: String conversion, iteration, variant counting

**Newtype Pattern:**
```rust
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct ProfileId(u64);

impl From<u64> for ProfileId { /* ... */ }
impl From<ProfileId> for u64 { /* ... */ }
impl Deref for ProfileId {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

This provides type safety while maintaining ergonomic conversion.

### 4. Async-First Design

The library is runtime-agnostic:
- No direct `tokio` dependency in production code
- Uses `async-trait` for trait-based async methods
- Returns `impl Stream` and `impl Future` for flexibility
- Tests use `tokio-test` but library works with any runtime

## Data Flow

### Example: Fetching Games for a Player

```
User Code:
  profile_games(PLAYER_ID)
    .with_leaderboard(Some(vec![GameKind::Rm1v1]))
    .get(100)
    ↓
ProfileGamesQuery::get(limit: 100)
  → Validates profile_id is set
  → Creates PaginationClient<ProfileGames, Game>
  → Builds URL with query params
    ↓
PaginationClient::into_pages_concurrent()
  → Calculates pages needed: ceil(100 / 50) = 2
  → Creates concurrent page requests (up to 8 at once)
    ↓
PageTurner::turn_page() for each page
  → GET https://aoe4world.com/api/v0/players/{id}/games?limit=50&page=1
  → Deserializes to ProfileGames
  → Extracts Vec<Game> via Paginated trait
  → Checks if more pages needed
    ↓
PagesStream<Game, Error>
  → Flattens all pages into single stream
  → User collects or iterates items
```

## Type System Design

### Domain Organization

Types are organized by API domain:

- **`civilization.rs`**: Game civilizations (22 variants)
- **`games.rs`**: Game matches, players, results, game kinds (40+ variants)
- **`leaderboards.rs`**: Leaderboard entries, league rankings
- **`maps.rs`**: Map definitions (~50 maps) with type classification (Land/Water/Hybrid)
- **`profile.rs`**: Player profiles, stats per game mode, rating history
- **`rank.rs`**: League system (Bronze/Silver/Gold/Platinum/Diamond/Conqueror)
- **`search.rs`**: Player search results

### Shared Patterns

**Optional Fields:**
Most fields are `Option<T>` to handle API inconsistencies and evolving schema.

**DateTime Handling:**
Uses `chrono::DateTime<chrono::Utc>` with automatic serde support.

**Country Codes:**
Uses `isocountry::CountryCode` for type-safe country representation.

**Collections:**
- `Vec<T>` for lists
- `BTreeMap<String, T>` for ordered key-value data (e.g., rating history)
- `HashMap<String, Value>` for unknown/dynamic data

## Testing Strategy

### Three-Pronged Approach

1. **Property-Based Tests**
   - Uses `arbitrary` crate to generate random instances
   - Tests serde roundtrip: `serialize(deserialize(serialize(x))) == serialize(x)`
   - Ensures all types can be serialized and deserialized correctly
   - Example: `test_serde_roundtrip_prop!(Civilization)`

2. **Schema Validation Tests**
   - Real API responses stored in `testdata/`
   - Tests deserialize actual JSON into Rust types
   - Catches schema mismatches and breaking API changes
   - Example: `test_json!(Profile, "../../testdata/profile/neptune.json", neptune_profile)`

3. **API Integration Tests**
   - Gated behind `test-api` feature flag
   - Hit real aoe4world API endpoints
   - Marked as `#[ignore]` by default (run with `cargo test --all-features`)
   - Verify end-to-end functionality with live data

### Test Macros

Located in `src/testutils.rs`:

```rust
test_serde_roundtrip_prop!(Type);  // Property-based serde test
test_json!(Type, "path.json", test_name);  // Schema validation test
test_enum_to_string!(Enum);  // Enum Display impl test
```

## Error Handling

- Uses `anyhow::Result` for flexible error propagation
- Validates required parameters in query builder `.get()` methods
- HTTP errors propagated from `reqwest`
- Deserialization errors from `serde_json`
- No custom error types (favors simplicity)

## Future Extensibility

### Adding New Endpoints

1. Create query builder struct in `src/lib.rs` query module
2. Implement `.get()` method returning `impl Stream<Item = Result<T>>`
3. Add paginated response type if needed (implement `Paginated<T>`)
4. Add convenience function at top level
5. Add tests in `src/lib.rs` tests module

### Adding New Types

1. Add enum variants or struct fields to `src/types/`
2. Ensure derives include serde, test attributes, strum (for enums)
3. Add property test: `test_serde_roundtrip_prop!(NewType)`
4. Fetch real API response and add to `testdata/`
5. Add schema test: `test_json!(NewType, "path.json", test_name)`

## Performance Considerations

- **Concurrent Pagination**: 8 concurrent requests significantly reduce latency
- **Stream-based API**: Allows lazy consumption, no need to buffer all results
- **Zero-copy where possible**: Uses references in iterator methods
- **Efficient JSON parsing**: `serde_json` with compile-time codegen
- **Connection pooling**: `reqwest` handles connection reuse automatically

## References

- **Main Documentation**: See `CLAUDE.md` for development workflow
- **API Specification**: https://aoe4world.com/api
- **page-turner crate**: https://docs.rs/page-turner

[`src/lib.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/lib.rs
[`src/pagination.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/pagination.rs
[`src/testutils.rs`]: https://github.com/willfindlay/prelate-rs/tree/main/src/testutils.rs
[`src/types/`]: https://github.com/willfindlay/prelate-rs/tree/main/src/types
[`testdata/`]: https://github.com/willfindlay/prelate-rs/tree/main/testdata
