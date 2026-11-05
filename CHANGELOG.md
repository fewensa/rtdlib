# Changelog

## 1.8.70 - 2026-11-05

Packaging, dependency, and documentation release. Generated types are unchanged. The TDLib schema is still **1.8.69** (`td_api.tl` at commit `4c6377aa7386c39217878a9a56d581922ad37ca4`). crates.io already published `1.8.69` from tag `v1.8.69`, so that version cannot be published again. Pin `rtdlib = "=1.8.70"`.

### Dependencies

Caret floors are the newest crates.io releases inside the existing major ranges. They still build on Rust 1.89. `Cargo.lock` already resolved these versions before the floors moved.

- `serde` `1.0.229`, `serde_derive` `1.0.229`, `serde_json` `1.0.154`, `serde-aux` `4.7.0`, `uuid` `1.29.0`.
- `rtdlib-sys` stays `0.1.0`, the only published release. The requirement is now written as `0.1.0`.

### CI

- `actions/checkout` is v7 in `.github/workflows/ci.yml` and `.github/workflows/publish.yml`. Tag `v1.8.69` used v4.

### Docs and build

These landed on master after tag `v1.8.69` and are not in the crates.io `1.8.69` package. The Build notes under the 1.8.69 heading were added in that same documentation change; they describe this tree.

- README Building section covers the RAM cost of compiling the generated types (about 6 GB for one `rustc` on a dev build; a 1 GB limit kills `rustc` with signal 9) and how to install `libtdjson` for the `sys` feature, including TDLib's `SplitSource.php` low-memory C++ build.
- `build.rs` warns when the visible memory limit is under 6 GiB. `build.rs` is listed in the package `include` so the warning ships.
- Crate-level docs and CONTRIBUTING state the RAM requirement and that `sys` links `libtdjson`.
- README, CONTRIBUTING, and SECURITY name crate `1.8.70`. `libtdjson` should still be TDLib 1.8.69.

### Unchanged

The public types match `1.8.69`. `rtdlib-sys` is still 0.1.0. The `sys` feature still re-exports `rtdlib_sys::Tdlib` and still links `libtdjson`.

## 1.8.69 - 2026-11-04

Breaking release. Types are regenerated from TDLib **1.8.69** (`td_api.tl` at commit `4c6377aa7386c39217878a9a56d581922ad37ca4`) with [fantasy](https://github.com/fewensa/fantasy) (`RTD_OUT=target/rtd-out cargo run --release -p fantasy`). This replaces both the crates.io `1.8.1` line (TDLib v1.8.0) and the unreleased `0.8.2` snapshot. Pin `rtdlib = "=1.8.69"`. A requirement such as `^1.8.1` will select this crate and will not compile against 1.8.1 code.

Edition 2021. Minimum supported Rust is 1.89. `uuid` 1 and `serde-aux` 4. `cargo audit` is clean. CI, `SECURITY.md`, and the tag publish workflow are unchanged from the maintenance pass except for this version.

### Breaking

- The TDLib schema moved from the January 2022 snapshot to 1.8.69. 15 type modules from 1.8.1 are gone and 505 modules are new. Removed modules: `chat_filter`, `chat_filter_info`, `chat_nearby`, `chat_report_reason`, `chat_statistics_message_interaction_info`, `chats_nearby`, `file_part`, `message_forward_origin`, `payment_form_theme`, `payments_provider_stripe`, `proxies`, `recommended_chat_filter`, `recommended_chat_filters`, `tdlib_parameters`, `web_page`.
- Renames that show up in client code: `ChatFilter` is `ChatFolder`, `WebPage` is `LinkPreview`, `MessageForwardOrigin` is `MessageOrigin`.
- `TdlibParameters` is gone. `setTdlibParameters` carries the fields directly. `database_encryption_key` is new. `enable_storage_optimizer` and `ignore_file_names` are gone.
- Scalar `int64` values deserialize from a JSON string or a number. TDLib's JSON client sends those values as strings.
- A TL field named `extra` (on `TonNft`) is the Rust field `extra_`. `@extra` is unchanged.
- Cycles in `RichText`, `PageBlock`, and `InternalLinkType` / `TargetChat` use `Box` on the field that closes the cycle.

### Build

- Documented RAM for compiling the generated types (about 6 GB for one `rustc` on the 1.8.69 dev build; a 1 GB limit kills `rustc` with signal 9) and how to install `libtdjson` for the `sys` feature, including TDLib's `SplitSource.php` low-memory C++ build.
- `build.rs` warns when the visible memory limit is under 6 GiB.

### Not in this release

`rtdlib-sys` is still 0.1.0. The `sys` feature still re-exports `rtdlib_sys::Tdlib` and still links `libtdjson`.

## 1.8.1 - 2022-01-29

Types for TDLib v1.8.0.

## 0.8.1 - 2022-01-29

Types for TDLib commit `789b9c0a554d779945db027fd2612909c676345f`. Not the crates.io latest line.
