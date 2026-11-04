# Changelog

## 0.8.2 - 2026-11-04

Maintenance release of the 0.8.x line (TDLib snapshot `789b9c0a554d779945db027fd2612909c676345f`). The Rust API is compatible with 0.8.1. Types were not regenerated.

### Changed

- Edition is 2021. The minimum supported Rust version is 1.89.
- `uuid` 0.8 is now `uuid` 1, used only to fill `@extra` on builders. `Uuid` is not part of the public API.
- `serde-aux` 0.6 is now `serde-aux` 4. Numeric fields still deserialize with `serde_aux::field_attributes::deserialize_number_from_string`.
- The unused direct `libc` dependency is gone. The `sys` feature still depends on `rtdlib-sys` 0.1, which links `libtdjson`.

### Fixed

- `RTDError` implements `Error::source`. `Error::cause` still reports that same underlying error through the trait default.
- Clippy warnings in hand-written code and in the generated builders and `is_*` helpers.
- JSON helpers no longer discard parse errors with `unwrap` after an `is_err` check.

### Not in this release

- Regeneration against current TDLib. The types still match the January 2022 snapshot above.
- The crates.io `1.8.x` line, which tracks TDLib 1.8.0 and lives on the `1d8x` branch. This release does not replace `1.8.1` as the highest version number.
- Changes to `rtdlib-sys`.

## 0.8.1 - 2022-01-29

Types for TDLib commit `789b9c0a554d779945db027fd2612909c676345f`.
