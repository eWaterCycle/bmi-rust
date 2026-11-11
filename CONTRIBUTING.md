# Contributing

## AI usage declaration

This project records AI assistance in [aidecl.yaml](./aidecl.yaml). 
Contributors are responsible for reviewing and verifying AI-assisted contributions.

## Development

Run these checks from the crate directory:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps
```

## Publishing releases

This repo has been configured for trusted publishing.

To release a version:

1. Update `version` in `Cargo.toml`.
2. Update the dependency example in `README.md` when appropriate.
3. Commit and push the changes.
4. Merge the pull request into `main`.
5. [Publish a GitHub release](https://github.com/eWaterCycle/bmi-rust/releases/new)
   with a matching tag, such as `v0.1.1` for version `0.1.1`.
   Start the release description with
   `Rust trait for the CSDMS Basic Model Interface` so the Zenodo entry is
   self-documenting.
