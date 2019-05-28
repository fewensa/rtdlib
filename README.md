rtdlib
===

[![Build Status](https://api.travis-ci.org/fewensa/rtdlib.svg)](https://travis-ci.org/fewensa/rtdlib/)


`rtdlib` is [TDLib](https://github.com/tdlib/td) for rust.

`rtdlib` crate have [TDLib](https://github.com/tdlib/td) type (classes). and `tdjson` [binding](./src/tdjson.rs). not have client api. so if use `rtdlib` you need include `libtdjson.so` to you build path.



# Build

`rtdlib` include a prebuilt td types, you can check [types.rs](./src/types.rs), so `rtdlib` does not actively rebuild, if want. you can set an environment `BUILD_TDAPI=true`, when `rtdlib` check `BUILD_TDAPI` is true, will rebuild td type, and generate a new [types.rs](./src/types.rs).

**BUILD TYPE.RS IS VERY SLOW.**

**Notice**

`rtdlib` build td type, use telegram official document, https://core.telegram.org/tdlib/docs/td__api_8h.html .

So there is a possibility of compilation failure.

It depends on whether the document has changed.


# Usage

```toml
[dependencies]
rtdlib = "0.1"
```


## type

```rust
let json =  r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#;
let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
assert_eq!("updateAuthorizationState", state.td_name());
```

## tdjson

```rust
use rtdlib::tdjson::Tdlib;
let tdlib = Tdlib::new();
let request = r#"{"@type": "getMe"}"#;
tdlib.send(request);
```

and more document you need check [telegram api](https://core.telegram.org/api)

**How include libtdjson.so**

The first you need read [td](https://github.com/tdlib/td#building) know how to build td.

And then, when you have `libtdjson.so` copy this file to `/usr/lib`.

Or set an environment

```bash
export RUSTFLAGS="-C link-args=-Wl,-rpath,/path/to/libtdjson.so"
cargo run
```

In the development phase, you can set `RUSTFLAGS` environment to you IDE.


