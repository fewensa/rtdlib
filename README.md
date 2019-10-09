rtdlib
===

[![Build Status](https://api.travis-ci.org/fewensa/rtdlib.svg)](https://travis-ci.org/fewensa/rtdlib/)


`rtdlib` is [td](https://github.com/tdlib/td) for rust.

`rtdlib` crate have [td](https://github.com/tdlib/td) type (classes). and `tdjson` [binding](../fantasy/template/rtdlib/src/tdjson.rs). if use `rtdlib` you need include `libtdjson.so` to you build path.


# Note

This crate code is generate by [fantasy](https://github.com/fewensa/fantasy).


# Usage

## 1.3.*

```toml
[dependencies]
rtdlib = "1.3"
```

## 1.4.*

```toml
[dependencies]
rtdlib = "1.4"
```

## version

Since the rtdlib version follows [td](https://github.com/tdlib/td), a version number less than 100 is reserved for td release.

Version mapping

| rtdlib    | td      |
|-----------|---------|
| 1.3.*     | 1.3.*   |
| 1.4.*     | 1.4.*   |


# Example

## types

```rust
let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
assert_eq!("updateAuthorizationState", state.td_name());
let rjson = state.to_json();
assert!(rjson.is_ok(), true);
assert_eq!(json, rjson.unwrap());
```

## tdjson

```rust
use rtdlib::tdjson::Tdlib;
let tdlib = Tdlib::new();
let request = r#"{"@type": "getMe"}"#;
tdlib.send(request);
```


# td

More document you need check [telegram api](https://core.telegram.org/api)

## How include libtdjson.so

The first you need read [td](https://github.com/tdlib/td#building) know how to build td.

And then, when you have `libtdjson.so` copy this file to `/usr/lib`.

Or set an environment

```bash
export RUSTFLAGS="-C link-args=-Wl,-rpath,/path/to/lib_dir"
cargo run
```

In the development phase, you can set `RUSTFLAGS` environment to you IDE.


