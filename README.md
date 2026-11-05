# rtdlib

[![CI](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml/badge.svg)](https://github.com/fewensa/rtdlib/actions/workflows/ci.yml)

`rtdlib` is Rust types for [TDLib](https://github.com/tdlib/td).

This release is **1.8.70**. The generated types are still TDLib 1.8.69 commit [`4c6377aa`](https://github.com/tdlib/td/commit/4c6377aa7386c39217878a9a56d581922ad37ca4). It is the highest version of this crate. The older `0.8.x` snapshot line, `1.8.0` / `1.8.1` (TDLib v1.8.0), and crates.io `1.8.69` are not current. Pin the version. See [CHANGELOG](CHANGELOG.md).

Minimum supported Rust version: **1.89**.

## Usage

```toml
[dependencies]
rtdlib = "=1.8.70"
```

The default crate contains TDLib types only. To call `tdjson`, enable `sys`. That re-exports `rtdlib_sys::Tdlib` and needs `libtdjson` at link time. `rtdlib-sys` is still 0.1.0.

```toml
[dependencies]
rtdlib = { version = "=1.8.70", features = ["sys"] }
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

## Building

### Memory

`cargo build` compiles every generated TDLib type in one crate. On rustc 1.89, a dev build of 1.8.69 peaks at about **6 GB RSS** in that single `rustc` process (`cargo build -j 1`). The older 0.8.1 sources peak at about 2.8 GB. A machine or container limited to 1 GB kills `rustc` with signal 9 (`SIGKILL`). The log looks like `could not compile rtdlib` / `signal: 9`. `build.rs` prints a cargo warning when the visible RAM limit is under 6 GiB.

`CARGO_BUILD_JOBS=1` keeps dependency builds from stacking on top of that process. An 8 GB host is a practical size when the rest of the system is quiet. The RAM goes to type-checking the generated types.

### `libtdjson` (`sys` only)

The default library build compiles the Rust types only. This crate does not compile TDLib. `rtdlib-sys` records `#[link(name = "tdjson")]`, and the linker searches for that library when it links a binary or a test that enables `sys`. A missing library fails there:

```text
ld: cannot find -ltdjson: No such file or directory
```

Copy `libtdjson` onto a library path the linker searches (`/usr/lib`, `/usr/local/lib` after `ldconfig`, or `LIBRARY_PATH`). `$PATH` is for executables. At run time the dynamic loader also needs the shared library (`LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH` on macOS, the DLL directory on Windows).

Match `libtdjson` to the schema in this crate (TDLib 1.8.69). Build instructions for each OS are at <https://tdlib.github.io/td/build.html>. From a TDLib checkout:

```bash
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --target tdjson
```

Install `libtdjson.so` (and the static archive, if you want it) from `build`. On Windows the import library is `tdjson.lib`; pass its directory with `RUSTFLAGS=-C link-arg=/LIBPATH:D:/path/to/lib`.

TDLib's own C++ sources can be built on a small machine by splitting them first. That script lowers the C++ compiler's RAM use. The Rust compile above is unchanged.

```bash
php SplitSource.php
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --target tdjson
cmake --build . --target tdjson_static
cd ..
php SplitSource.php --undo
```

TDLib's README reports that, with this split, Clang 6 used under 500 MB per file and GCC 4.9 / 6.3 used under 1 GB per file.

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
