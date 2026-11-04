# rtdlib

[![CI](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml/badge.svg)](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml)

`rtdlib` is Rust types for [TDLib](https://github.com/tdlib/td).

This repository's `master` branch is the **0.8.x** line. Those types were generated from TDLib commit [`789b9c0`](https://github.com/tdlib/td/commit/789b9c0a554d779945db027fd2612909c676345f) (January 2022). They have not been regenerated for newer TDLib. The crates.io **1.8.x** versions track TDLib 1.8.0 and are published from the `1d8x` branch; `0.8.2` does not replace `1.8.1` as the highest version number. See [version mapping](https://github.com/fewensa/telegram-client/blob/master/version.md) and [CHANGELOG](CHANGELOG.md).

Minimum supported Rust version: **1.89**.

## Usage

Pin the version. TDLib types and `telegram-client` move together.

```toml
[dependencies]
rtdlib = "=0.8.2"
```

The default crate contains TDLib types only. To call `tdjson`, enable `sys`. That re-exports `rtdlib_sys::Tdlib` and needs `libtdjson` at link time. `rtdlib-sys` is still 0.1.0.

```toml
[dependencies]
rtdlib = { version = "=0.8.2", features = ["sys"] }
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

## td

Further protocol documentation is on the [Telegram API](https://core.telegram.org/api) site.

## Security

Report vulnerabilities privately. See [SECURITY.md](SECURITY.md).
