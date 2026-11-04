# Type generator

`rtdlib` types are generated from TDLib's `td_api.tl` by the `fantasy` generator (the same tool as [fewensa/fantasy](https://github.com/fewensa/fantasy)), adapted in this directory.

The schema checked in at `schema/td_api.tl` is TDLib **1.8.69** at commit `4c6377aa7386c39217878a9a56d581922ad37ca4`.

```bash
cd generate
cargo run --release
# writes ../target/rtd-out by default
# copy target/rtd-out/src/types over src/types, then cargo fmt
```

`RTD_OUT` overrides the output directory. The generator replaces that directory's `src`. It does not edit this crate's `Cargo.toml`, `src/lib.rs`, or `src/errors.rs`.

Adaptations on top of fantasy:

- scalar `int64` fields deserialize from a JSON string or a number (`serde-aux`)
- a TL field named `extra` is Rust `extra_` and still serializes as `extra`, so it does not clash with `@extra`
- direct cycles (`RichText`, `PageBlock`, `InternalLinkType`/`TargetChat`) are broken with `Box`

This package is a developer tool. It is not part of the published `rtdlib` crate.
