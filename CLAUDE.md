# Prelate-rs Project Context

Prelate-rs is an idiomatic, asynchronous Rust wrapper around the [aoe4world API](https://aoe4world.com/api). It provides type-safe access to Age of Empires IV player statistics, game data, leaderboards, and search functionality.

**Version:** 0.4.2
**License:** MIT OR Apache-2.0
**Repository:** https://github.com/willfindlay/prelate-rs

## Project Structure

```
prelate-rs/
├── src/
│   ├── lib.rs              # Main library entry point with public API
│   ├── pagination.rs       # Internal pagination machinery (transparent to users)
│   ├── testutils.rs        # Test utilities and macros
│   └── types/              # Type definitions for API schemas
│       ├── civilization.rs # Civilization enums (22 total)
│       ├── games.rs        # Game data structures
│       ├── leaderboards.rs # Leaderboard types
│       ├── maps.rs         # Map definitions (~50 maps)
│       ├── profile.rs      # Player profiles and stats
│       ├── rank.rs         # League/rank system
│       └── search.rs       # Search results
├── testdata/               # JSON files for schema validation tests
├── .github/workflows/      # CI/CD workflows
│   ├── check-and-lint.yaml # Formatting and linting checks
│   └── unit-tests.yaml     # Test suite with coverage
├── Cargo.toml             # Dependencies and metadata
├── rustfmt.toml           # Formatter configuration
└── ARCHITECTURE.md        # Architecture documentation
```

## Technologies

- **Language:** Rust 2021 Edition
- **Async Runtime:** Uses `tokio` (dev dependency), library is runtime-agnostic
- **HTTP Client:** `reqwest` with JSON support
- **Serialization:** `serde` and `serde_json` with custom derives
- **Pagination:** `page-turner` crate for concurrent page fetching
- **Testing:** `arbitrary` for property-based tests, `tokio-test` for async tests

## Development Workflow

### Build and Check

```bash
# Check compilation
cargo check --all --all-targets --all-features

# Build the library
cargo build

# Build release version
cargo build --release
```

### Testing

```bash
# Run all unit tests (excluding API integration tests)
cargo test --lib

# Run tests with API integration tests (requires internet)
cargo test --all-features

# Run specific test module
cargo test --lib types::civilization

# Run property-based tests
cargo test serde_roundtrip_prop
```

**Test Organization:**
- Property-based tests using `arbitrary` crate verify serde roundtrips
- Schema tests in `testdata/` validate deserialization of real API responses
- API smoke tests gated behind `test-api` feature flag (hit real API)

### Formatting

**IMPORTANT:** This project uses **nightly rustfmt** with custom configuration.

```bash
# Format code (requires nightly rustfmt)
cargo fmt

# Check formatting
cargo fmt --all -- --check
```

**Configuration** (`rustfmt.toml`):
- `unstable_features = true`
- `reorder_imports = true`
- `imports_granularity = "Crate"`
- `format_code_in_doc_comments = true`

### Linting

```bash
# Run clippy with warnings as errors
cargo clippy --all-features -- -D warnings

# Run clippy on all targets
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation

```bash
# Build and view documentation
cargo doc --open --all-features
```

## Architecture Patterns

### 1. Query Builder Pattern

All API endpoints use builder pattern with `derive-setters`:

```rust
use prelate_rs::{profile_games, types::games::GameKind};

let games = profile_games(PLAYER_ID)
    .with_leaderboard(Some(vec![GameKind::Rm1v1]))
    .with_since(Some(date))
    .get(100)  // Returns Stream<Item = Result<Game>>
    .await?;
```

### 2. Transparent Pagination

The library handles pagination automatically using the `PageTurner` trait:
- Calculates total pages needed upfront
- Fetches 8 pages concurrently by default
- Returns unified `Stream` of items
- User never sees `Pagination` struct

**Implementation location:** `src/pagination.rs:78-128`

### 3. Type-Safe API Responses

All API types use:
- `#[serde(rename_all = "snake_case")]` for API field names
- `#[cfg_attr(test, derive(arbitrary::Arbitrary))]` for property testing
- `#[cfg_attr(test, serde(deny_unknown_fields))]` for strict validation
- `strum` derives for `Display`, `EnumString`, `VariantArray`

### 4. Feature Flags

- `test-api`: Enables integration tests that hit real API (default: off)

## Common Tasks

### Adding New API Types

When the aoe4world API adds new data:

1. **Check for new enum variants:**
   ```bash
   # Fetch recent games and extract unique values
   curl -s "https://aoe4world.com/api/v0/games?limit=100" | \
     jq '.games[].teams[][].player.civilization' | sort -u
   ```

2. **Update enums in `src/types/`:**
   - Civilizations: `src/types/civilization.rs`
   - Maps: `src/types/maps.rs`
   - Game kinds: `src/types/games.rs`

3. **Add enum variants using PascalCase:**
   ```rust
   pub enum Civilization {
       // ... existing variants
       GoldenHorde,        // Maps to "golden_horde"
       MacedonianDynasty,  // Maps to "macedonian_dynasty"

       // IMPORTANT: Always include Unknown variant for forward compatibility
       #[serde(untagged)]
       #[strum(default)]
       #[cfg(not(test))]
       Unknown(String),
   }
   ```

4. **For enums with Unknown variant, manually implement VariantArray:**
   ```rust
   impl strum::VariantArray for Civilization {
       const VARIANTS: &'static [Self] = &[
           Self::English,
           Self::French,
           // ... all known variants
           // Note: Unknown variant intentionally excluded
       ];
   }
   ```

5. **Important notes about Unknown variants:**
   - Enums with `Unknown(String)` cannot derive `Copy` (String is not Copy)
   - Use `.clone()` when moving enum values
   - Unknown variant only exists in non-test builds (`#[cfg(not(test))]`)
   - Tests validate against strict known variants
   - Production builds gracefully handle new API values

6. **Run tests to verify:**
   ```bash
   cargo test --lib
   ```

### Adding New Endpoints

1. Create query builder struct in `src/lib.rs` (in `query` module)
2. Implement `.get()` method that returns `impl Stream<Item = Result<T>>`
3. Add convenience function at top level of `src/lib.rs`
4. Add integration test with `#[cfg_attr(not(feature = "test-api"), ignore)]`

Example pattern from `ProfileQuery`:
```rust
pub fn profile(profile_id: impl Into<ProfileId>) -> ProfileQuery {
    ProfileQuery::default().with_profile_id(Some(profile_id.into()))
}
```

### Updating Test Data

When API schema changes:

1. Fetch fresh API response and save to `testdata/`
2. Run tests to identify deserialization issues
3. Update type definitions in `src/types/`
4. Verify with `cargo test --lib`

## CI/CD Workflows

### Check and Lint (`check-and-lint.yaml`)

Runs on PRs and main branch pushes:
- `cargo check --all --all-targets --all-features`
- `cargo fmt --all -- --check` (with nightly)
- `cargo clippy --all-features -- -D warnings`

### Unit Tests (`unit-tests.yaml`)

Runs on PRs and main branch pushes:
- `cargo test --all-features --no-fail-fast`
- Runs with coverage instrumentation (nightly)
- Caches Cargo dependencies

## Key Dependencies

**Production:**
- `reqwest` (0.11) - HTTP client
- `serde`/`serde_json` (1.0) - Serialization
- `page-turner` (0.8) - Pagination abstraction
- `futures` (0.3) - Async stream handling
- `chrono` (0.4) - DateTime handling
- `isocountry` (0.3) - Country code validation
- `strum` (0.26) - Enum utilities

**Development:**
- `tokio` (1.23) - Async runtime for tests
- `arbitrary` (1.2) - Property-based testing
- `pretty_assertions` (1.3) - Better test output

## Code Style Conventions

1. **Imports:** Use `imports_granularity = "Crate"` (groups by crate)
2. **Error Handling:** Use `anyhow::Result` for flexibility
3. **Validation:** Validate required fields in `.get()` methods
4. **Async:** Library is runtime-agnostic (no `tokio` in main deps)
5. **Testing:** Three-pronged approach (property, schema, API smoke tests)

## API Coverage Status

Currently supported endpoints:
- ✅ `GET /api/v0/players/:profile_id`
- ✅ `GET /api/v0/players/:profile_id/games`
- ✅ `GET /api/v0/players/search`
- ✅ `GET /api/v0/leaderboards/:leaderboard`
- ✅ `GET /api/v0/games`

Not yet implemented:
- ❌ `GET /api/v0/players/:profile_id/games/last`
- ❌ `GET /api/v0/players/autocomplete`
- ❌ `GET /api/v0/stats/qm_{1v1,2v2,3v3,4v4}/civilizations`

## Common Patterns

### Query Parameter Building

All query builders have private `query_params()` method:

```rust
fn query_params(&self, mut url: Url) -> Url {
    if let Some(ref leaderboard) = self.leaderboard {
        url.query_pairs_mut()
            .append_pair("leaderboard", join(leaderboard, ",").as_str());
    }
    // ... more parameters
    url
}
```

### Newtype Wrappers with Conversions

```rust
#[derive(Serialize, Deserialize)]
pub struct ProfileId(u64);

impl From<u64> for ProfileId { /* ... */ }
impl From<ProfileId> for u64 { /* ... */ }
impl Deref for ProfileId {
    type Target = u64;
    // ...
}
```

### Paginated Response Trait

```rust
pub(crate) trait Paginated<T> {
    fn pagination(&self) -> &Pagination;
    fn data(self) -> Vec<T>;
}
```

Implemented by: `GlobalGames`, `ProfileGames`, `LeaderboardPages`, `SearchResults`

## Troubleshooting

**Test failures after API changes:**
- Update `testdata/` JSON files with fresh API responses
- Check for new enum variants in API responses
- Update type definitions to match schema

**Formatting issues:**
- Ensure using nightly rustfmt: `rustup component add rustfmt --toolchain nightly`
- Run: `cargo +nightly fmt`

**Clippy warnings about pagination:**
- Known issues exist in `src/pagination.rs` and `src/types/` (non-canonical `partial_cmp`, etc.)
- These are tracked but don't affect functionality

## References

- **API Documentation:** https://aoe4world.com/api
- **Architecture Guide:** See `ARCHITECTURE.md`
- **Contributing:** Follow standard Rust practices + project formatting rules
- **Crate Page:** https://crates.io/crates/prelate-rs
