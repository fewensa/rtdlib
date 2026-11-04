# Changelog

## 1.8.69 - 2026-11-04

Breaking release. Types are regenerated from TDLib **1.8.69** (`td_api.tl` at commit `4c6377aa7386c39217878a9a56d581922ad37ca4`). This replaces both the crates.io `1.8.1` line (TDLib v1.8.0) and the unreleased `0.8.2` snapshot. Pin `rtdlib = "=1.8.69"`. A requirement such as `^1.8.1` will select this crate and will not compile against 1.8.1 code.

Edition 2021. Minimum supported Rust is 1.89. `uuid` 1 and `serde-aux` 4. `cargo audit` is clean. CI, `SECURITY.md`, and the tag publish workflow are unchanged from the maintenance pass except for this version.

### Breaking

- The TDLib schema moved from the January 2022 snapshot to 1.8.69. 15 type modules from 1.8.1 are gone and 505 modules are new. Removed modules: `chat_filter`, `chat_filter_info`, `chat_nearby`, `chat_report_reason`, `chat_statistics_message_interaction_info`, `chats_nearby`, `file_part`, `message_forward_origin`, `payment_form_theme`, `payments_provider_stripe`, `proxies`, `recommended_chat_filter`, `recommended_chat_filters`, `tdlib_parameters`, `web_page`.
- Renames that show up in client code: `ChatFilter` is `ChatFolder`, `WebPage` is `LinkPreview`, `MessageForwardOrigin` is `MessageOrigin`.
- `TdlibParameters` is gone. `setTdlibParameters` carries the fields directly. `database_encryption_key` is new. `enable_storage_optimizer` and `ignore_file_names` are gone.
- Scalar `int64` values deserialize from a JSON string or a number. TDLib's JSON client sends those values as strings.
- A TL field named `extra` (on `TonNft`) is the Rust field `extra_`. `@extra` is unchanged.
- Cycles in `RichText`, `PageBlock`, and `InternalLinkType` / `TargetChat` use `Box` on the field that closes the cycle.

### Not in this release

`rtdlib-sys` is still 0.1.0. The `sys` feature still re-exports `rtdlib_sys::Tdlib` and still links `libtdjson`.

## 1.8.1 - 2022-01-29

Types for TDLib v1.8.0.

## 0.8.1 - 2022-01-29

Types for TDLib commit `789b9c0a554d779945db027fd2612909c676345f`. Not the crates.io latest line.
