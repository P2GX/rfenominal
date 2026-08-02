# Developer Guide

## Feature flags

`fenominal` has an optional Python extension built on `pyo3`. The features are wired like this:

```toml
[dependencies]
pyo3 = { version = "0.21", features = ["extension-module", "abi3-py37"], optional = true }

[features]
default = ["serde"]
serde = ["dep:serde", "dep:serde_json"]
python = ["dep:pyo3"]
extension-module = ["pyo3?/extension-module"]   # note the `?` — see "Gotchas" below
```

- **`python`** — pulls in `pyo3` as a dependency (needed to build the Python bindings at all).
- **`extension-module`** — tells pyo3 *not* to link against `libpython`, because at runtime the
  already-running Python interpreter supplies those symbols. This is required for building an
  actual Python extension module (`.so`/`.dylib` loaded by `import fenominal`), and is fatal for
  anything that produces a standalone binary (plain `cargo build`/`cargo test`).
- **`abi3-py37`** — enables pyo3's stable ABI targeting CPython ≥ 3.7. Lets maturin build a
  single wheel that works across Python versions instead of needing one wheel per minor version.

## Release (Python wheel via maturin)

```bash
maturin develop --features extension-module
```

To release on PyPI: commit a version bump to `main`, then tag it (use the version number as the tag):

```bash
git tag v0.2.15
git push origin v0.2.15
```

## Running Rust tests (cargo test / rstest)

**Do not use `--all-features` or otherwise enable `extension-module` when running `cargo test`.**
`extension-module` makes pyo3 skip linking `libpython`. That's correct for a wheel (Python
supplies the symbols at import time) but `cargo test` builds a standalone binary with no Python
process around, so the link step fails with a wall of `symbol(s) not found for architecture ...`
errors (`_PyErr_Fetch`, `_PyBaseObject_Type`, etc.).

Run tests with an explicit, safe feature set instead:

```bash
cargo test --features serde
```

or just:

```bash
cargo test
```

since `serde` is already a default feature and `python`/`extension-module` are not.

### VS Code / rstest runner

Check `.vscode/settings.json` for `rust-analyzer.cargo.allFeatures: true` — this is what silently
enables `extension-module` and breaks the in-editor "Run Test" links. Replace it with an explicit
feature list:

```json
{
  "rust-analyzer.cargo.allFeatures": false,
  "rust-analyzer.cargo.features": ["serde"]
}
```

Also check `.vscode/tasks.json` / `launch.json` for any `cargo test ... --all-features` entries
and change them to `--features serde` (or drop the flag entirely).

## Gotchas

- **`pyo3/extension-module` vs `pyo3?/extension-module`**: writing the dependency-feature link as
  `pyo3/extension-module` in `[features]` implicitly activates the optional `pyo3` dependency
  itself whenever `extension-module` is enabled — even without the `python` feature. Use the `?`
  form (`pyo3?/extension-module`) so `extension-module` only forwards to pyo3 *if* pyo3 is already
  enabled via `python`, instead of pulling it in on its own. This is what makes `--all-features`
  safe(r) — though the safest approach is still to just not use `--all-features` for this crate.
- If you ever add tests that actually call into the Python interpreter (`Python::with_gil`, etc.),
  you'll additionally need pyo3's `auto-initialize` feature so `libpython` is initialized at
  runtime. That's a separate concern from the linking issue above.
