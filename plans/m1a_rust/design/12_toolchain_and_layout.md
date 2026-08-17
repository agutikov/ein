# 12 — Toolchain, workspace layout, and CI

**Settles:** where the Rust code lives, how it splits into crates, what
it is allowed to depend on, and what the gates are.
**Phase:** [P1a.0](../p1a.0_conformance_harness/README.md), stage
[S1a.0.4](../p1a.0_conformance_harness/s1a.0.4_workspace_skeleton_and_ci.md).

---

## 1. Repo layout

`ein.rs/` sits beside `ein.py/`, mirroring the naming the repo already
uses:

```
ein/
├── stdlib/                     ← shared, single source of truth (11)
├── examples/                   ← shared corpus
├── conformance/                ← corpus manifest + expected messages
├── ein.py/                     ← the oracle (unchanged, stays green)
├── ein.rs/
│   ├── Cargo.toml              (workspace)
│   ├── rust-toolchain.toml     (pinned stable + components)
│   ├── deny.toml               (cargo-deny: licences, advisories)
│   └── crates/
│       ├── ein-core/           values, interner, KB, indexes, provenance, pyrepr
│       ├── ein-ir/             lexer, parser, AST, dumper, macros, imports, stdlib embed
│       ├── ein-infer/          compile, match, saturate, world, search, verdict, explain
│       ├── ein-render/         DOT renderers, markdown trace, solution table
│       ├── ein-cli/            the `ein` binary
│       ├── ein-server/         the daemon (feature-gated)          [P1a.8]
│       ├── ein-py/             PyO3 bindings (maturin)             [P1a.9]
│       └── ein-conformance/    corpus runner, event differ, fuzzer
└── utils/                      (existing scripts; gains a couple of runners)
```

Why this split:

- **`ein-core` has no I/O and no engine.** It is the data model
  ([03](03_data_model.md)) plus the `python_repr` compatibility renderer
  ([02](02_determinism_and_order.md) §7). Everything depends on it;
  it depends on nothing.
- **`ein-ir` owns the only filesystem access in the engine** (import
  resolution + the embedded stdlib), which is what makes
  `--sandbox` in [09](09_server_mode.md) a single seam.
- **`ein-infer` never formats anything.** All rendering lives in
  `ein-render`, so the T3 surface is one crate and can be diffed as a
  unit.
- **`ein-conformance` is a normal crate, not a test harness bolted on.**
  It has a binary (`ein-conformance run|diff|fuzz`) so it is usable by
  hand, which is how it will actually get used during the port.

`ein.rs/` compiles to a binary named `ein`. It does **not** get installed
onto `$PATH` during the port — the harness invokes both engines by
explicit path, so there is never ambiguity about which `ein` ran.

---

## 2. Dependency policy

The engine's runtime dependency budget is deliberately tiny; every crate
listed has to earn its place, and the *engine* crates (`core`, `ir`,
`infer`) must stay usable in a `no_std`-adjacent, dependency-light build
if that ever matters.

| crate | where | why |
|---|---|---|
| `rustc-hash` (`FxHashMap`) | core, infer | deterministic, fast, non-DoS-resistant hashing — correct choice here, and it removes the `RandomState` iteration-order hazard ([02](02_determinism_and_order.md) §9) |
| `smallvec` | core, infer | premises, bindings, commitments — all small and hot |
| `bitvec` *or* a hand-rolled `BitSet` | core | presence / negated / alive sets. Hand-rolled is ~80 lines; prefer it |
| `md-5` | render | `hashed_id` parity ([02](02_determinism_and_order.md) §8) |
| `blake3` | server, einb | content addressing |
| `include_dir` | ir | embedded stdlib |
| `memchr` | ir | lexer scanning |
| `rayon` | infer | [P1a.7](../p1a.7_parallelism/README.md) only, behind a `parallel` feature |
| `serde` + `serde_json` | conformance, server, cli(`--events`) | the event protocol and JSON-RPC |
| `zstd` | einb | optional section compression, behind a feature |
| `pyo3` / `maturin` | ein-py | [P1a.9](../p1a.9_bindings_release/README.md) only |
| `criterion`, `proptest`, `arbitrary` | dev-dependencies | benches and property tests |

**Not used:** any parser generator (see [04](04_ir_frontend.md) §1), any
async runtime in the engine (the server may use one; the engine stays
synchronous and `Send`), `lazy_static`/`once_cell` in favour of
`std::sync::OnceLock`, and `unsafe` outside a single audited module for
the `.einb` zero-copy casts (`#![forbid(unsafe_code)]` on every other
crate).

**MSRV**: current stable minus two releases, pinned in
`rust-toolchain.toml` and asserted in CI.

---

## 3. Feature flags

| feature | default | effect |
|---|---|---|
| `parallel` | off during P1a.0–6, on from P1a.7 | pulls `rayon`, enables `--jobs > 1` |
| `server` | off | builds `ein-server` and the `serve` subcommand |
| `einb` | on | `.einb` read/write |
| `events` | on | `--events FILE` emission (compiled out entirely when off, for a zero-overhead measurement build) |
| `python` | off | PyO3 bindings |

The `events` flag matters: benchmarks build with `--no-default-features
--features einb` so no branch on the hot path exists at all, and the
conformance runs build with it on. Both are checked in CI.

---

## 4. CI gates

Three tiers.

**Per-commit (~5 min)**

- `cargo fmt --check`, `cargo clippy -- -D warnings`
- `cargo test --workspace` (unit + property tests)
- `cargo deny check`
- **conformance-fast**: the pinned corpus subset at the current phase's
  tier ([01](01_parity_contract.md) § Tier → phase map)
- `stdlib-check` ([11](11_shared_assets.md) §3)
- corpus-completeness check
- the existing `./run_tests.sh --fast` for ein.py (the oracle must stay
  green — a parity failure is meaningless if the oracle moved)

**Nightly (~1 h)**

- **conformance-full**: the whole corpus × the whole run matrix
- `PYTHONHASHSEED` sweep on ein.py ([02](02_determinism_and_order.md) §9)
- ein.rs self-parity: every corpus entry run twice, byte-diffed
- `./run_tests.sh` full, including the acceptance gate
- benches (`criterion`) with regression thresholds
- the differential fuzzer for a fixed budget, corpus-minimising any find

**Release**

- Everything nightly, plus `--jobs {1,2,4,8}` cross-diff, thread
  sanitizer, a `--no-default-features` build, and the packaging matrix
  (Linux/macOS/Windows binaries, `maturin` wheels once P1a.9 lands).

### Benchmarks

`criterion` benches, all against the shared corpus so the numbers are
comparable with the Python baseline:

| bench | measures |
|---|---|
| `parse` | `zebra2.ein`, `zebra.ein`, the stdlib modules |
| `load` | parse + imports + macro expansion + index build |
| `saturate_root` | root saturation only |
| `match_hot` | `match::run` over the saturated root, per plan |
| `boundary` | a full `_admit_from_boundary` round |
| `solve_fast` / `solve_exhaustive` | end-to-end, both puzzles |
| `fork` | fork + first delta write |

The Python side gets a matching runner (`utils/bench_baseline.py`) so
`design/README.md § Measured` can be refreshed with one command per
implementation.

---

## 5. Repo hygiene

- `.gitignore`: `ein.rs/target/`, `ein.py/src/ein/stdlib/` (now
  build-generated — [11](11_shared_assets.md) §3), `conformance/out/`.
- `AGENTS.md` (= `CLAUDE.md`) gains an `ein.rs/` entry under *Where
  things live* and a note that `stdlib/` is shared. Do this in P1a.0, not
  at the end — the file is how future sessions orient.
- Commits go to `master` (repo convention), one stage per commit where
  practical, with the stage id in the subject.

## Cross-links

- [01 — Parity contract](01_parity_contract.md) — what CI is gating.
- [11 — Shared assets](11_shared_assets.md) — `stdlib-check`, the corpus
  manifest.
- [`run_tests.sh`](../../../run_tests.sh) — the existing two-phase Python
  runner the Rust gates sit beside.
