
        # photon-uf-app extraction

        Upstream playbook for [deathbreakfast/photon-uf-app](https://github.com/deathbreakfast/photon-uf-app).

        ## Workspace crates

        | Phase | Work | Status |
        |-------|------|--------|
        | 0 | skeleton | shipped — workspace + stub crates |
| 1 | core import | NOT_STARTED |
| 2 | git deps + verify | NOT_STARTED |


        ## Dependencies (git)

        Platform libraries are pinned in the root `Cargo.toml` `[workspace.dependencies]`.
        Use release tags when available; `branch = "main"` during initial skeleton phase.

        ## Gating (before tag)

        - No zone vocabulary in public docs
        - Hero README with quick-start git dependency
        - `cargo check --workspace` green on skeleton
