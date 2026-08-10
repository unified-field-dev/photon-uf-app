# photon-uf-app verification

Re-run after code or doc changes. This workspace is the Photon operations app
(`photon-app` Leptos UI + `photon-backend` pure server contracts). Layer 1 unit +
integration tests cover topic/subscription/event/dashboard helpers backing the
`#[server]` surface, plus sibling-source UI surface contracts for `photon-app`.
No Leptos UI e2e, `*-e2e` crate, or AWS campaign is required for this workspace.
Photon core IsolatedLab / storage contracts own transport persistence; this repo
verifies the UF app mapping layer over Photon (`admin_snapshot`,
`list_recent_events`, `list_events_by_topic`, `get_event`).

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
```

## Teaching host (Pass 3 gate)

Axum oneshot under [`examples/protected-photon-host`](../examples/protected-photon-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-photon-host
cargo run -p protected-photon-host
```

Success line: `protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to a composite product host).

## Layer 1 — Unit + integration (CI)

GitHub Actions (`.github/workflows/ci.yml`) covers this Layer 1 subset plus the
teaching host and photon-backend rustdoc gate below. It does not build
`photon-app` (Leptos UI / SSR).

Sibling-source UI contracts (no Orbital / `photon-app` compile):

```bash
cargo test -p photon-backend --test workspace_members --test product_surface
```

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt -p photon-backend -p photon-app -p protected-photon-host -- --check
cargo clippy -p photon-backend --all-targets -- -D warnings
cargo clippy -p protected-photon-host --all-targets -- -D warnings
cargo test -p photon-backend
```

`cargo fmt --all` can fail in this monorepo checkout when a path-patched
`neutrino/uf-host` sits outside that workspace; package-scoped fmt is the honest
local gate.

Full workspace (includes `photon-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Photon backend contract gap.
Surface needles for routes, nav testids, `RequireAuthenticated`, and
`PhotonAdmin` live in `product_surface`.

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p photon-app --features ssr
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

## Layer 2 — E2E

**Waived.** Topic/subscription list+detail, topic-scoped subscription filter,
dashboard 24h counting, and event preview/transport-expired shapes are exercised
by Layer 1 integration tests named below. A Leptos/UI browser suite or IsolatedLab
`*-e2e` crate is out of scope for this backend-first remediation; live Photon
transport/persistence IsolatedLab belongs in photon core.

Covering integ tests for the e2e waiver:

- `get_topics_list_sorted_and_named_happy_path` / `get_topic_detail_matches_list_entry_happy_path` / `get_topic_unknown_name_is_none_sad`
- `get_subscription_detail_matches_list_entry_happy_path` / `get_subscription_unknown_id_is_none_sad`
- `topic_detail_filters_subscriptions_for_topic_happy_path` / `topic_detail_filters_subscriptions_unknown_topic_empty_sad`
- `validate_topic_name_accepts_table_happy_path` / `validate_topic_name_rejects_blank_sad`
- `validate_subscription_id_accepts_id_happy_path` / `validate_subscription_id_rejects_blank_sad`
- `dashboard_stats_aggregates_counts_happy_path` / `count_since_24h_window_happy_path` / `count_since_all_older_is_zero_sad`
- `event_summary_list_row_preview_happy_path` / `event_detail_transport_expired_shape_happy_path`
- `validate_event_id_accepts_id_happy_path` / `validate_event_id_rejects_blank_sad`
- `photon_product_workspace_members_happy_path`
- `photon_routes_mount_happy_path` / `layout_auth_gate_and_nav_happy_path` / `ops_reads_require_photon_admin_happy_path`
- `protected_photon_host_matches_uf_app_happy_path`

## Layer 3 — AWS campaigns + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Photon UF app DTO/mapping contracts only.

## Rustdoc policy

Preferred deny gate (no UI graph):

```bash
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p photon-backend --no-deps
```

Workspace `rustdoc::broken_intra_doc_links` is `allow` in `Cargo.toml` because
sibling/cfg-gated links often fail under `--no-deps`. Prefer the
`RUSTDOCFLAGS` deny form above for the backend contract crate. `photon-app`
rustdoc with deny flags is pin-dependent on Orbital / host graphs.
`photon-app` still uses `#![allow(missing_docs)]` on macro-heavy UI surfaces.

## Notes

- Prefer `cargo test -p photon-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Photon contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` so audits detect them.
- `PhotonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers covered by `topic_subscription_contract` and
  `event_dashboard_contract`.
