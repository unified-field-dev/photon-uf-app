# photon-uf-app verification

Re-run after code or doc changes. This workspace is the Photon operations app
(`photon-app` Leptos UI + `photon-backend` pure server contracts +
`photon-uf-app-e2e` lab host). Layer 1 covers DTO helpers, Photon IO ops helpers
(`ops` feature), and sibling-source UI needles. Layer 2 is Playwright against a
dedicated lab Leptos host that mounts eager `PhotonRoutes` with mem Valence and
in-process mem Photon. Photon core IsolatedLab / AWS still own transport and
broker persistence.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
export PHOTON_ALLOW_DEV_TRANSPORT_KEY=1
export PHOTON_TRANSPORT_KEY='AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='
```

Package name note: this workspace crate is `photon-backend@0.1.0`. Enabling
`ops` also pulls L0 `photon-backend@0.1.4` into the graph — prefer
`-p photon-backend@0.1.0` in commands when both are present.

## Teaching host

Axum oneshot under [`examples/protected-photon-host`](../examples/protected-photon-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-photon-host
cargo run -p protected-photon-host
```

Success line: `protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to a composite product host or `photon-uf-app-e2e`).

## Layer 1 — Unit + integration (CI)

GitHub Actions (`.github/workflows/ci.yml`) covers this Layer 1 subset plus the
teaching host, photon-backend rustdoc gate, ops Photon IO integ, and
`photon-uf-app-e2e` boundary contracts below.

Sibling-source UI contracts (structural smokes — not primary coverage):

```bash
cargo test -p photon-backend@0.1.0 --test workspace_members --test product_surface
```

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt -p photon-backend -p photon-app -p protected-photon-host -p photon-uf-app-e2e -- --check
cargo clippy -p photon-backend@0.1.0 --all-targets -- -D warnings
cargo clippy -p protected-photon-host --all-targets -- -D warnings
cargo test -p photon-backend@0.1.0
```

Photon IO ops helpers (mem Photon + validating happy/sad):

```bash
cargo test -p photon-backend@0.1.0 --features ops --test ops_photon_contract
```

Lab host boundary (mem Photon after `init_e2e_valence`, no browser):

```bash
cargo test -p photon-uf-app-e2e --features ssr --test boundary_contract
```

`cargo fmt --all` can fail when a sibling `neutrino/uf-host` checkout sits
outside this workspace; package-scoped fmt is the honest local gate.

Full workspace (includes `photon-app` UI). May fail when the sibling
`uf-product` / `uf-integrations` UI graph does not compile — that is a
host-product UI issue, not a Photon backend contract gap.

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p photon-app --features ssr
cargo check -p photon-uf-app-e2e --features ssr
```

### leptos-lints (local; hydrate UI)

Needs `cargo-dylint` / `dylint-link` 6.0.1 and toolchain `nightly-2025-05-14`
(see `leptos-lints@v0.1.2`). Workspace `[workspace.metadata.dylint]` pins the
library; rustc deny names are declared under `[workspace.lints.rust]`.

```bash
# cargo install cargo-dylint --locked --version 6.0.1
# cargo install dylint-link --locked --version 6.0.1
# rustup toolchain install nightly-2025-05-14 --component rustc-dev,llvm-tools-preview

export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
export CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback
export RUSTFLAGS="-D warnings -Zcrate-attr=feature(stdarch_x86_avx512)"

cargo dylint --all -p photon-app --no-deps -- --features hydrate
```

Hard CI job deferred: `photon-app` hydrate still depends on the Orbital / host
graph (same pin risk as UI compile in Layer 1). Run locally when that graph is
green.

## Layer 2 — E2E (lab host + Playwright)

Primary operator-UI gate. Dedicated lab host mounts eager `PhotonRoutes` pages
(same components as production Lazy routes), mem Valence, Higgs session injection,
and in-process mem Photon. Port `127.0.0.1:3190`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
export PHOTON_ALLOW_DEV_TRANSPORT_KEY=1
export PHOTON_TRANSPORT_KEY='AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='
# From the photon-uf-app workspace root. Builds SSR + hydrate, then Playwright.
cargo leptos end-to-end --project photon-uf-app-e2e
```

Do not interrupt the end-to-end run. It stops when Playwright finishes.

Scenario IDs (validating happy + sad):

- `e2e.auth.anonymous_gate` / `e2e.auth.no_admin` / `e2e.auth.admin_dashboard`
- `e2e.dashboard.load_happy`
- `e2e.topics.index_happy` / `e2e.topics.detail_happy` / `e2e.topics.not_found_sad`
- `e2e.subs.index_happy` / `e2e.subs.detail_happy` / `e2e.subs.not_found_sad`
- `e2e.events.index_happy` / `e2e.events.detail_happy` / `e2e.events.not_found_sad`

Catalog: [`photon-uf-app-e2e/README.md`](../photon-uf-app-e2e/README.md).

`product_surface` source needles remain Layer 1 structural smokes. They do not
replace Layer 2.

L5 host Playwright composition smoke for `/photon` lives on
`unified-field-embedded` (`e2e.l5.photon_shell_smoke`) and is not a substitute
for this lab catalog.

## Layer 3 — Cloud + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Photon UF app DTO/mapping/ops contracts and
the lab e2e host. Live broker fleets belong in photon core / `uf-live-cloud-lab`.

## Rustdoc policy

Preferred deny gate (no UI graph):

```bash
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p photon-backend@0.1.0 --no-deps
```

Workspace `rustdoc::broken_intra_doc_links` is `allow` in `Cargo.toml` because
sibling/cfg-gated links often fail under `--no-deps`. Prefer the
`RUSTDOCFLAGS` deny form above for the backend contract crate. `photon-app`
rustdoc with deny flags is pin-dependent on Orbital / host graphs.
`photon-app` still uses `#![allow(missing_docs)]` on macro-heavy UI surfaces.

## Notes

- Prefer `cargo test -p photon-backend@0.1.0` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Photon contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` / `integ_*` / `e2e.*` so audits detect them.
- `PhotonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over [`photon_backend::ops`] (feature `ops`) and the pure helpers in
  `topic_subscription_contract` / `event_dashboard_contract`.
