rtdlib
===

[![Build Status](https://api.travis-ci.org/fewensa/rtdlib.svg)](https://travis-ci.org/fewensa/rtdlib/)


`rtdlib` is [td](https://github.com/tdlib/td) for rust.

`rtdlib` crate have [td](https://github.com/tdlib/td) type (classes). and `tdjson` [binding](../fantasy/template/rtdlib/src/tdjson.rs). if use `rtdlib` you need include `tdjson` dylib file(libtdjson.so/tdjson.lib/tdjson.dll) to you build path.


# Usage

```toml
[dependencies]
rtdlib = "1.6.*"
```

## version

Since the rtdlib version follows [td](https://github.com/tdlib/td), a version number less than 100 is reserved for td release.

Version mapping

| rtdlib    | td      |
|-----------|---------|
| 0.*       | master  |
| 1.3.*     | 1.3.*   |
| 1.4.*     | 1.4.*   |
| 1.5.*     | 1.5.*   |
| 1.6.*     | 1.6.*   |


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

## How include tdjson

The first you need read [td](https://github.com/tdlib/td#building) know how to build td.

And then, when you have `tdjson` dylib file (.so/.lib/.dll) copy the file to the path of the system `$PATH` variable.

Or set an environment `RUSTFLAGS`, in the development phase, you can set `RUSTFLAGS` environment to you IDE.

### Linux

```bash
export RUSTFLAGS="-C link-args=-Wl,-rpath,/path/to/lib_dir"
cargo run
```

### Windows

**MSVC**

```bash
RUSTFLAGS=-C link-args=-LIBPATH:D:/path/to/lib_dir
```



