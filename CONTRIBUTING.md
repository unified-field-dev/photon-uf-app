# Contributing to Photon UF App

Thank you for improving this project.

## Development setup

1. Clone [unified-field-dev/photon-uf-app](https://github.com/unified-field-dev/photon-uf-app)
2. Install Rust **nightly** (see [`rust-toolchain.toml`](rust-toolchain.toml))
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
export PHOTON_ALLOW_DEV_TRANSPORT_KEY=1
export PHOTON_TRANSPORT_KEY='AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='
cargo check -p photon-backend
cargo check -p photon-app --features ssr
```

Full Layer 1 + Layer 2 gates: [`docs/VERIFICATION.md`](docs/VERIFICATION.md).

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when user-facing flows or host mounting steps change.
- CI runs fmt, clippy (backend + teaching host + `photon-app` SSR), contract tests, boundary integ, Playwright e2e, and backend rustdoc — see Verify in the README.
