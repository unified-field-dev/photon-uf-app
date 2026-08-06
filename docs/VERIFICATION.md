# photon-uf-app verification

Re-run after code or doc changes. This workspace is the Photon operations app
(`photon-app` Leptos UI + `photon-backend` pure server contracts). Layer 1 unit +
integration tests cover topic/subscription/event/dashboard helpers backing the
`#[server]` surface. No Leptos UI e2e, `*-e2e` crate, or AWS campaign is required
for this workspace. Photon core IsolatedLab / storage contracts own transport
persistence; this repo verifies the UF app mapping layer over Photon
(`admin_snapshot`, `list_recent_events`, `list_events_by_topic`, `get_event`).

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-uf-app
```

## Layer 1 — Unit + integration (CI)

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt --all --check
cargo clippy -p photon-backend --all-targets -- -D warnings
cargo test -p photon-backend
```

Full workspace (includes `photon-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Photon backend contract gap:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p photon-app --features ssr
```

### TEST_MAP

| Behavior | Level | Happy | Sad | Notes |
|----------|-------|-------|-----|-------|
| `validate_topic_name` | unit+integ | non-empty / trimmed name | blank / whitespace → `"required"` | gate for topic detail |
| `validate_subscription_id` | unit+integ | non-empty id | blank → `"required"` | gate for subscription detail |
| `validate_event_id` | unit+integ | non-empty id | blank → `"required"` | gate for event detail |
| `find_topic_by_name` (`get_topic`) | unit+integ | exact name → summary | unknown → `None` | list/detail contract |
| `find_subscription_by_id` (`get_subscription`) | unit+integ | exact id → summary | unknown → `None` | list/detail contract |
| `filter_subscriptions_by_topic` | unit+integ | topic-scoped subset | unknown topic → `[]` | topic detail page |
| `sort_topics_by_name` (`get_topics`) | unit+integ | lexicographic order | — | stable list |
| `dashboard_stats` / `count_since` | unit+integ | KPI shape / 24h window | all older → `0` | dashboard |
| `event_summary_from_meta` / preview | unit+integ | `[status]` preview | — | recent/events list |
| `event_summary_from_transport` / `event_detail_from_transport` | unit | `[stored]` preview / live detail | — | Photon list/get path |
| `subscription_summary_from_handler` / `find_checkpoint_seq` | unit | registry_key id + checkpoint match | missing sub → `None` | admin_snapshot mapping |
| `event_detail_transport_expired` | unit+integ | null payload + flag | — | transport gone |
| `stub_checkpoint_lag` | unit | always `0` | — | lag UI stub (known gap) |
| `clamp_event_list_limit` | unit+integ | caps at `MAX_EVENT_LIST_LIMIT` | oversized → 100 | PH-03 scope |
| Higgs `#[server]` fns + PhotonAdmin session | — | — | — | deferred — needs host SSR (PH-01..04) |
| Leptos UI / Playwright / `cargo leptos` e2e | e2e | — | — | **waived** — covering integ named below |
| IsolatedLab topic/subscription e2e | e2e | — | — | **waived** — covered by photon core + Layer 1 integ |
| AWS / soak | AWS | — | — | **waived** — no cloud resources |
| Micro-benchmarks | bench | — | — | **waived** — no hot-path campaign |

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

## Layer 3 — AWS campaigns + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Photon UF app DTO/mapping contracts only.

## Notes

- Prefer `cargo test -p photon-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Photon contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` so audits detect them.
- `PhotonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers listed in the TEST_MAP.
