rtdlib
===

[![Build Status](https://api.travis-ci.org/fewensa/rtdlib.svg)](https://travis-ci.org/fewensa/rtdlib/)


`rtdlib` is [TDLib](https://github.com/tdlib/td) for rust.

`rtdlib` crate have [TDLib](https://github.com/tdlib/td) type (classes). and `tdjson` [binding](./src/tdjson.rs). not have client api. if use `rtdlib` you need include `libtdjson.so` to you build path.



# Build

`rtdlib` include a prebuilt td types, you can check [types](./src/types). The version of `>0.3.0` not provide build code before run this crate, last support version is [0.2.1](https://github.com/fewensa/rtdlib/releases/tag/0.2.1), if want generate types from [telegram official document](https://core.telegram.org/tdlib/docs/td__api_8h.html) you can check [generate](./generate) project.

When build from official document, rtdlib will generate a td [schema](./schema/schema.toml) file, but this file not include in cargo package, if you need use this file, you need check [schema.toml](./schema/schema.toml).

**BUILD TYPE.RS IS SLOWLY.**

**Notice**

`rtdlib` build td type, use telegram official document, https://core.telegram.org/tdlib/docs/td__api_8h.html .

So there is a possibility of compilation failure.

It depends on whether the document has changed.

When rtdlib is stable, the use of code generation is not recommended.

# Usage

```toml
[dependencies]
rtdlib = "0.3"
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


