This is a dummy crate to publish to crates.io. It primarily exists to ensure
that folks trying to install clippy from crates.io get redirected to the
`crablangup` technique.

Before publishing, be sure to rename `clippy_dummy` to `clippy` in `Cargo.toml`,
it has a different name to avoid workspace issues.
