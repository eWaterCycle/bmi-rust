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

CI runs these checks on every push, pull request, and published GitHub release.

## Publishing releases

Configure a [crates.io trusted publisher](https://crates.io/docs/trusted-publishing)
for `bmi-rust` with these GitHub Actions settings:

- Repository owner: `eWaterCycle`
- Repository name: `bmi-rust`
- Workflow filename: `publish.yml`
- Environment: `release`

Create the repository's GitHub Actions environment `release` to match this
configuration. The publish job uses OIDC to obtain a temporary crates.io token;
no `CARGO_REGISTRY_TOKEN` repository secret is needed.

To release a version, update `version` in `Cargo.toml` and the dependency example
in `README.md` when appropriate, and commit the changes. Publish a GitHub release
with a matching tag, such as `v0.1.0` for version `0.1.0`.

The publish workflow checks that the tag matches the package version, runs
formatting, tests, Clippy, and documentation checks, then publishes to crates.io.
Publishing a GitHub prerelease also triggers the workflow; use a matching Cargo
prerelease version and tag, such as `0.2.0-alpha.1` and `v0.2.0-alpha.1`.
