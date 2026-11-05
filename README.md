# rtdlib

[![CI](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml/badge.svg)](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml)

`rtdlib` is Rust types for [TDLib](https://github.com/tdlib/td).

This release is **1.8.69**, generated from TDLib 1.8.69 commit [`4c6377aa`](https://github.com/tdlib/td/commit/4c6377aa7386c39217878a9a56d581922ad37ca4). It is the highest version of this crate. The older `0.8.x` snapshot line and `1.8.0` / `1.8.1` (TDLib v1.8.0) are not current. Pin the version. See [CHANGELOG](CHANGELOG.md).

Minimum supported Rust version: **1.89**.

## Usage

```toml
[dependencies]
rtdlib = "=1.8.69"
```

The default crate contains TDLib types only. To call `tdjson`, enable `sys`. That re-exports `rtdlib_sys::Tdlib` and needs `libtdjson` at link time. `rtdlib-sys` is still 0.1.0.

```toml
[dependencies]
rtdlib = { version = "=1.8.69", features = ["sys"] }
```

## Example

### types

```rust
let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
let state: UpdateAuthorizationState = serde_json::from_str(json).expect("Json fail");
assert_eq!("updateAuthorizationState", state.td_name());
let rjson = state.to_json();
assert!(rjson.is_ok());
```

### tdjson

With the `sys` feature:

```rust
use rtdlib::Tdlib;
let tdlib = Tdlib::new();
let request = r#"{"@type": "getMe"}"#;
tdlib.send(request);
```

Initialization parameters are fields of `SetTdlibParameters`. There is no `TdlibParameters` type.

## td

Further protocol documentation is on the [Telegram API](https://core.telegram.org/api) site.

## Security

Report vulnerabilities privately. See [SECURITY.md](SECURITY.md).

## Regenerating types

Types are generated in [fantasy](https://github.com/fewensa/fantasy), not in this repository. From a fantasy checkout:

```bash
RTD_OUT=target/rtd-out cargo run --release -p fantasy
```

Copy `target/rtd-out/src/types` over `src/types` here.
