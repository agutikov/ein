# M1a — Rust port (ein.rs)

**Estimate:** TBD.
**Status:** **placeholder** — slotted between M1 and M1b. Reserved
for the directional decision "the engine that ships from M2 onward
is Rust, not Python." The Python implementation
(`ein.py/`) stays as the reference / oracle through M2's NL → IR
work, but compute-heavy paths (saturator hot loop, lattice
backbone, hash/index work) move to a native-speed implementation.
**Depends on:** [M1](../m1_core_graph_reasoning/README.md) — needs
the engine semantics frozen by M1 (kernel rules, NAF, branching,
back-prop, set-indexed engines from P1.5b) before the port can
target a stable surface.
**Blocks:** [M1b](../m1b_gui/README.md) — GUI is a productivity
multiplier on top of *the engine that ships*; landing the Rust
port first means M1b binds to ein.rs and doesn't need a second
re-target when ein.rs lands later.
[M2](../m2_nl_to_ir/README.md)'s NL pipeline is unaffected (NL
frontend is CPython for llama.cpp / Python bindings — see
[S1.5a.6 Q-S1.5a.6.B](../m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.6_pypy_compat_perf.md#open-questions)
— but talks to ein.rs over a binding boundary).

## Why a port (not just PyPy)

The PyPy measurement landed in
[S1.5a.13.1](../m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.13_acceptance_zebra2_solves.md#task-t15a131--measurement-closed-2026-05-26)
2026-05-26: **6.0× on zebra2 d=1** over CPython, **6.7× on
saturate-time alone**. That clears the ≥5× threshold for "PyPy is
a viable primary perf path" — but PyPy is the wrong target for
the next phase for three reasons:

1. **Distribution / deployment.** M1b GUI + M2 NL frontend ship to
   end-users; PyPy adds a second interpreter for the user to
   install, ein.rs ships as a single binary.
2. **Ergonomics for the hot loop.** PyPy's JIT helps but Python
   data-model overhead persists (every Fact is a heap object,
   every set membership is a hash table lookup). Rust's ownership
   + struct layout removes both costs structurally.
3. **Concurrency / parallelism.** P1.8 Theme B's COW + P1.5b's
   per-set engine + future distributed search all want
   data-parallel primitives that are awkward in Python (GIL) and
   natural in Rust. Already a sketch in
   [S1.5a.20 § distributed](../m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md)
   (dropped, but the contract is what P1.5b's set-batch primitive
   delivers).

## Scope (to refine on promotion)

The promotion-time decision is **port everything** vs **port the
hot loop and keep a Python harness**. Two candidate boundaries:

### Boundary A — full port

Native Rust crate `ein` re-implementing:

- IR parser (lalrpop or pest)
- KB store + entity model
- Saturator + matcher + back-prop
- Solver (`_consume` + the P1.5b monotonic / lattice engines)
- Dumper + DOT renderer hooks
- CLI (`Ein`-equivalent)

Python becomes the reference oracle for differential testing
(`tests/golden/zebra2.golden` etc. cross-checked against
ein.rs); M2 NL frontend talks to ein.rs over PyO3 or a stdin/JSON
protocol.

### Boundary B — hot-loop port

Keep CPython for IR parser + CLI + glue; native Rust only for:

- KB store (`Fact` / `Provenance` / `_facts_by_relation`)
- Saturator's `_apply` + matcher's `_run_steps`
- `back_propagate` + `_negated_facts`

Surface via PyO3. Python `Saturator(kb)` becomes a thin wrapper
over the Rust core. Smaller surface; preserves M1's tooling
(state_dump, bench_solve) without re-implementation.

Decision deferred until measurement says where the bottleneck
*is* — likely Boundary B for the first cut, Boundary A on a
second wave if the parser / solver gain enough native-side
allocations to make crossing the FFI boundary expensive.

## Out of scope (deferred)

- **PyPy as a permanent constraint.** The PyPy path was the M1
  measurement vehicle; once ein.rs lands, PyPy stays only as a
  "Python users get a working solver" fallback. Not maintained as
  a deployment target.
- **Re-deriving M1's semantics.** Every M1 stage that established
  invariants (S1.5a.1 NAF re-eval, S1.5a.1a determinism,
  S1.5a.19 d=0 negative-completion, P1.5b set-indexed engines)
  is a port target, not a redesign target. ein.rs implements
  *the engine M1 delivers*, not a "Rust-y" reinterpretation.

## Open questions

- **Boundary A vs B.** See § Scope. Decide on promotion.
- **Build system.** `cargo` standalone, or `cargo` + `maturin`
  for the PyO3 surface, or a hybrid where Ein the CLI stays
  Python and only the engine crate is Rust.
- **Memory model.** Append-only KB makes COW trivially correct
  (P1.8 Theme B2). Rust's borrow checker turns this into a
  structural property — a `KnowledgeBase` is an `Arc<KbCore>`
  with per-fork lock-free additions.
- **State sync with M1b GUI.** If M1b is Electron / browser, the
  Rust engine talks over IPC; if it's Tauri, same crate; if it's
  pure-Rust desktop (egui / iced), the engine is a library
  dependency.
- **M2 NL frontend boundary.** PyO3 or stdin/JSON? PyO3 is
  faster but ties M2's release cadence to ein.rs ABI; JSON is
  cleaner but adds serialisation overhead per round-trip.

## Cross-links

- [M1 — core graph reasoning](../m1_core_graph_reasoning/README.md)
  — the engine semantics this port is faithful to.
- [P1.5a S1.5a.6 PyPy measurement](../m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.6_pypy_compat_perf.md)
  — the 6× headline that motivated picking the native-port path
  over PyPy-as-primary.
- [P1.8 Theme B PERFORMACE](../m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md#theme-b---performace)
  — the Python-side perf work; some of this folds into the port,
  some stays as Python-specific (COW, indexes, version-based KB).
- [M1b GUI](../m1b_gui/README.md) — composes; the GUI's stack
  choice ties to the engine's distribution shape.
- [M2 — NL → IR](../m2_nl_to_ir/README.md) — the NL frontend
  talks to ein.rs over a binding boundary; LLM infra
  (llama.cpp / Python bindings) stays CPython.
