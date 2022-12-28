## prelate-rs

`prelate-rs` is an idiomatic, asynchronous Rust wrapper around the [aoe4world API][api]. Very much a WIP at this stage.

[![Dual Apache-2.0 and MIT licensed][mit-apache-badge]][copyright-url]
[![Checks and lints][check-and-lint-badge]][check-and-lint-url]
[![Unit tests][unit-tests-badge]][unit-tests-url]

[crates-badge]: https://img.shields.io/crates/v/prelate-rs.svg
[crates-url]: https://crates.io/crates/prelate-rs
[mit-apache-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue
[copyright-url]: https://github.com/willfindlay/prelate-rs/blob/main/COPYRIGHT.md
[check-and-lint-badge]: https://github.com/willfindlay/prelate-rs/actions/workflows/check-and-lint.yaml/badge.svg
[check-and-lint-url]: https://github.com/willfindlay/prelate-rs/actions/workflows/check-and-lint.yaml
[unit-tests-badge]: https://github.com/willfindlay/prelate-rs/actions/workflows/unit-tests.yaml/badge.svg
[unit-tests-url]: https://github.com/willfindlay/prelate-rs/actions/workflows/unit-tests.yaml

### Project Status

We currently support the [following endpoints][api] (those that are ticked):

- [x] `GET /api/v0/players/:profile_id`
- [x] `GET /api/v0/players/:profile_id/games`
- [ ] `GET /api/v0/players/:profile_id/games/last`
- [x] `GET /api/v0/players/search`
- [ ] `GET /api/v0/players/autocomplete`
- [ ] `GET https://aoe4world.com/api/v0/leaderboards/:leaderboard`
- [ ] `GET https://aoe4world.com/api/v0/games`
- [ ] `GET https://aoe4world.com/api/v0/stats/qm_1v1/civilizations`
- [ ] `GET https://aoe4world.com/api/v0/stats/qm_2v2/civilizations`
- [ ] `GET https://aoe4world.com/api/v0/stats/qm_3v3/civilizations`
- [ ] `GET https://aoe4world.com/api/v0/stats/qm_4v4/civilizations`

[api]: https://aoe4world.com/api
