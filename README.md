rtdlib
===

[![Build Status](https://api.travis-ci.org/fewensa/rtdlib.svg)](https://travis-ci.org/fewensa/rtdlib/)


`rtdlib` is [td](https://github.com/tdlib/td) for rust.

`rtdlib` crate have [td](https://github.com/tdlib/td) type (classes).


# Usage

```toml
[dependencies]
rtdlib = "1.8.*"
```

The default, `rtdlib` only have `td` types, not have call tdjson dylib, if you want , you need add `features` to your dependency.

```toml
[dependencies]
rtdlib = { version = "1.8.*", features = "sys" }
```

## version

Please read: [version](https://github.com/fewensa/telegram-client/blob/master/version.md)

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

If you enable `sys` features, you can use `Tdlib` to call tdjson dylib.

```rust
use rtdlib::Tdlib;
let tdlib = Tdlib::new();
let request = r#"{"@type": "getMe"}"#;
tdlib.send(request);
```


# td

More document you need check [telegram api](https://core.telegram.org/api)


