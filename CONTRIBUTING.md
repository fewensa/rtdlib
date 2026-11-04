# Contributing

Thanks for helping with `rtdlib`.

## Development

The minimum supported Rust version is 1.89. CI uses current stable as well.

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets --features sys -- -D warnings
cargo audit
```

`rustfmt.toml` uses two-space indentation. Run `cargo fmt` before pushing. `src/types` is generated TDLib bindings; formatting-only edits there are fine, but behavior changes need a compatibility note.

The public Rust API should stay compatible with the 0.8.x line when the change allows it. `telegram-client` depends on these types and on the `sys` feature, which re-exports `rtdlib_sys::Tdlib`.

## What this crate is

`master` is the 0.8.x snapshot line (TDLib commit `789b9c0`). It is not a regeneration of current TDLib, and it is not the crates.io `1.8.x` line (branch `1d8x`, TDLib 1.8.0). Regenerating types for a newer TDLib is a separate change and needs a version-scheme decision before it lands.

## Security

Report vulnerabilities privately. See [SECURITY.md](SECURITY.md). Do not file a public issue for a security report.
