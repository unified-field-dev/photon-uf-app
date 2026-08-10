# Contributing to Photon UF App

Thank you for improving this project.

## Development setup

1. Clone [deathbreakfast/photon-uf-app](https://github.com/deathbreakfast/photon-uf-app)
2. Install Rust stable
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
cargo check -p photon-backend
cargo check -p photon-app --features ssr
```

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when user-facing flows or host mounting steps change.
