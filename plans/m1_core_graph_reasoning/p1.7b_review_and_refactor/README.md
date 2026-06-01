# P1.7b — Review & refactor (post-M1 debt paydown)

**Estimate:** 1–2.5 weeks (review + structural cleanup; **non-gating**).
**Depends on:** [P1.7a](../p1.7a_solution_search_refactor/README.md) (the sound
`solve()` — the stable behavioural baseline we refactor *under*),
[P1.5b](../p1.5b_lattice_search/README.md) (the unified engine being cleaned).
**Blocks:** nothing for M1 (M1 shipped via P1.7a). **Recommended before**
[M1a — Rust port](../../m1a_rust/README.md) (don't transcribe a 621-line fused
function or dead code into `ein.rs`) and
[P1.10 — kernel docs](../p1.10_kernel_docs/README.md) (cleaner code → simpler
docs).
**Created:** 2026-06-01, from a review of the implementation as it stands after
the M1 gate.

## Why this phase exists

M1 is *done* — but it got there through P1.5a → P1.5b → P1.7 → P1.7a, a
sequence that **removed two whole search engines** (`tree/`, then
`monotonic_solve`) and **bolted soundness onto a loop built for goal-match
termination**. The engine works and is gated, but it carries the scar tissue:
a 621-line function that fuses three search modes, seven dead functions, nine
references to a removed function, docstrings describing machinery that no
longer exists, a per-fact O(\|names\|) index rebuild on the hot path, and the
same DOT-string helpers re-rolled in six files.

ruff is **green** — none of this is visible to the linter. It is structural
debt: the cost of *reading*, *extending*, and *porting* the code. This phase
is the deliberate paydown, grounded in a code-cited review
([`findings.md`](findings.md)), done **without changing one bit of engine
behaviour**.

## The one hard constraint

> **No behaviour change.** Every stage keeps the full pytest suite (604
> tests) and the PyPy 3-variant acceptance green, and every bench
> (`bench_solve_monotonic_pypy.sh`, `bench_lattice_pypy.sh`,
> `demo/bench_*.py`) produces an **identical verdict + bindings** to its
> pre-stage baseline.

This mirrors the discipline of the
[S1.5b.1 file-split](../p1.5b_lattice_search/s1.5b.1_file_split_refactor.md)
("no semantic change is permitted in this stage"). Refactor stages that touch
the hot path (S1.7b.4) additionally must not regress wall-clock — measured,
not assumed. The two latent **bugs** the review surfaced (F-KER-4 config
crash, F-KB-3 double-index) are the *only* sanctioned behaviour changes, each
gated by a new regression test that fails before and passes after.

## Scope — the review ([`findings.md`](findings.md))

The review covered the whole `ein_bot` package (13.3k LOC, 51 files) via an
AST span/branch/nesting scan + four parallel subsystem audits, with every
correctness- or deletion-bound claim re-verified against the code path. It
catalogues **40 findings** across four areas (ENG / KER / KB / RTC), each with
a stable id, severity, category, and `file:line`. The user-requested axes map
onto it as:

- **architecture** → the fused loop (F-ENG-1), the add/index contract
  (F-KB-3), the missing shared DOT emitter (F-RTC-1).
- **too-long functions** → 9 functions > 100 lines, 10 nested ≥ 5 deep.
- **dead code** → 7 dead functions + 1 unused hook + 9 stale refs (all
  grep-verified).
- **duplications** → DOT helpers ×5, symmetric-lookup ×4, provenance-walker
  ×3, unsat-core ×4, the entry post-amble, the stats/dumper pairs.
- **overcomplicated** → the `mode` neutraliser, the `phase_2_done` goto, the
  `_coerce` if-ladder.
- **best practices / readability / maintainability / extensibility** → the
  decompositions + the typed index wrapper + injected-predicate termination.
- **performance** → the `fork`/`add_fact`/`_index_fact`/`snapshot` hot-path
  cluster (F-KB-1..6).

## Stages

Ordered low-risk → high-risk so confidence compounds. **S1.7b.1 goes first**
(pure subtraction shrinks the surface everything else touches); **2–5 are
independent** and may run in any order or in parallel; **S1.7b.6 is the gate**.

| ID | title | leverage | findings | status |
|---|---|---|---|---|
| **S1.7b.1** | [Dead-code & stale-doc sweep](s1.7b.1_dead_code_sweep.md) | high / ~½ day, near-zero risk | F-KER-9, F-ENG-2/3/10, F-KB-11, F-RTC-7 | ✅ **shipped** (`0e124ea`) |
| **S1.7b.2** | [`_explore_layers` decomposition + `Mode` retirement](s1.7b.2_explore_layers_decomposition.md) | **flagship** | F-ENG-1/4/5/6/7/8/13/14, F-KER-1 | ⏸ **deferred** (see ledger) |
| **S1.7b.3** | [Inference-kernel function refactors](s1.7b.3_inference_kernel_functions.md) | high | F-KER-2/3/4/5/6/7/8/10/15 | ◐ **mostly shipped** (`0e124ea`,`030365c`) — helper unifications deferred |
| **S1.7b.4** | [KB hot-path & index refactor](s1.7b.4_kb_hotpath_and_indexes.md) | high (perf + a bug) | F-KB-1/2/3/4/5/6/9/13 | ◐ **bug+perf shipped** (`0e124ea`,`b658968`) — decomp/wrappers deferred |
| **S1.7b.5** | [Shared DOT emitter + render/trace/cli decomposition](s1.7b.5_dot_emitter_and_render_trace_cli.md) | high | F-RTC-1..10, F-KB-8/10 | ◐ **escape dedup shipped** (`ba01162`) — emitter API/cli/trace deferred |
| **S1.7b.6** | [Acceptance — green suite, benches unchanged, metrics re-measured](s1.7b.6_acceptance.md) | gate | — | ✅ **done** (this ledger) |

Low-priority items (F-ENG-9 `_BaseStats`, F-ENG-11 `_TimelineMixin`,
F-KB-13/F-RTC-9/10) are folded as *optional* tasks inside their nearest
stage; skip without guilt if the budget runs short — they don't move the
metrics much.

## Acceptance (this phase)

1. **Behaviour unchanged.** Full pytest suite + PyPy 3-variant acceptance
   green; `bench_solve_monotonic_pypy.sh zebra2` and `bench_lattice_pypy.sh`
   give byte-identical verdicts/bindings to the 2026-06-01 baseline. `ruff
   check .` green.
2. **The two bugs are closed**, each with a regression test: a
   `(config :lattice-order-seed 7)` file loads (F-KER-4); a fact re-derived
   by two rules appears **once** in `_facts_by_relation` (F-KB-3).
3. **Metrics moved** (re-run the S1.7b.6 scan): no function > ~120 lines
   except where justified in writing; zero nesting depth ≥ 8; zero
   verified-dead functions; zero `monotonic_solve`/`tree`-engine references in
   `src/`.
4. **No new public API churn beyond the intended ones.** The three engine
   entries keep their names + return types (internals change, signatures
   don't); CLI surface unchanged.
5. **Hot-path no-regression.** S1.7b.4's `fork`/`add_fact`/`_index_fact`
   changes show wall-clock ≤ baseline on `bench_solve_monotonic_pypy.sh`
   (ideally below — the O(n)→O(deg) and whole-dict→in-place changes should
   help).

## Out of scope (deferred)

- **New features / new heuristics.** The E1-E23 catalog is
  [P1.9](../p1.9_hypothesis_loop_followups/README.md); this phase only
  *cleans* what exists.
- **The package/CLI rename** (`ein-bot` → `ein`, demo merge, cli folder
  split) is [P1.11](../p1.11_package_restructure/README.md) — S1.7b.2's
  `cli._build_parser` split is internal-only and does **not** pre-empt P1.11's
  folder move.
- **Kernel-purity changes** (`symmetric`/`is-a`/`closed`) stay in
  [P1.7 S1.7.7](../p1.7_bootstrapping_zebra/s1.7.7_kernel_purity_analysis.md);
  P1.7b does not touch rule semantics.
- **MUS minimisation / unsat-core shrinking** — a P1.7a follow-up, not a
  refactor.

## Connections

- [P1.7a](../p1.7a_solution_search_refactor/README.md) named the debt it left
  behind ("pure per-branch, not the surgical-reuse plan"; "perf is the
  soundness tax"). P1.7b is the cleanup that P1.7a's ship-notes implied.
- [M1a Rust port](../../m1a_rust/README.md): the strongest argument for doing
  this *now* — `ein.rs` should transcribe a clean reference, not the scar
  tissue.
- The discipline is [S1.5b.1](../p1.5b_lattice_search/s1.5b.1_file_split_refactor.md)'s:
  a structural move with the test suite as the invariant.

## What shipped (2026-06-01) — and what's deferred

Executed in five commits on `master` (`7a9fe7a` docs → `0e124ea` → `030365c`
→ `b658968` → `ba01162`). The **hard constraint held**: every commit kept the
full suite green and the verdicts byte-identical — the PyPy 3-variant
acceptance gate re-ran at **8 passed / 4:43** with the *same* enterings counts
(zebra2 → Solution k=1 @ 101 enterings exhausted; minus-15 → Ambiguity k=2;
bad → Contradiction k=0) as the pre-refactor 4:39 baseline. `ruff check .`
green throughout.

**Shipped:**

- **Dead-code & stale-doc sweep (S1.7b.1) — complete.** 7 dead functions +
  the never-fired `early_terminate` hook + `verdict_entry` + `Saturator.solved`
  deleted; every stale `monotonic_solve` / `tree`-engine / false
  `NotImplementedError` doc reference rewritten. Grep-verified: **0** of each
  remaining in `src/`.
- **Two latent bugs fixed, each with regression tests.**
  - **F-KER-4** — `_coerce` rewritten as a dispatch table. The review found the
    `int | None` crash; execution found it was *broader* — **every numeric
    `(config …)` flag was unsettable via IR** (`Int` nodes carry `.value`, not
    `.name`). +11 tests (`tests/inference/test_config.py`).
  - **F-KB-3/4** — `add_and_index_fact` ends the `add_fact` + unconditional
    `_index_fact` double-index across 8 hot-path callers; dedup is now O(deg).
    +3 tests (`tests/kb/test_store_indexing.py`).
- **Decompositions (S1.7b.3).** `_compile_premise` (139→slim head-dispatch +
  `_desugar_open`/`_desugar_forall`/`_compile_relation`); `back_propagate`'s
  duplicated ancestor-bubble loop → one helper; symmetric-relation lookup
  unified into `kb.is_symmetric` / `kb.symmetric_relations()` (4 copies → 1).
- **KB hot-path perf (S1.7b.4).** `_index_fact` writes in place instead of
  rebuilding whole dicts per fact (O(|names|)→O(k)); `fork()` shares the four
  post-load-immutable type/rule indexes by reference. Both proven leak-free by
  an assignment-site audit + the fork-parity/shuffle-invariance tests.
- **DOT escape dedup (S1.7b.5).** `esc()`/`multiline()` promoted into
  `render/dot_util.py`; 5 byte-identical copies removed. Output byte-identical.

Net: `src/` shrank 13337 → 13269 LOC (despite added code) with **0** dead
functions; +14 regression tests (617 `test_` defs).

**Deferred — and why (honest):**

- **The flagship `_explore_layers` decomposition (S1.7b.2) — NOT done.** After
  reading all 620 lines: the per-candidate merge block mutates the
  loop-carried `alive`/`a_layer` through **7 entry-discriminated
  `phase_2_done` break-sites**, and a byte-exact EntryPolicy extraction is the
  plan's own "3–5 day" effort where a single mis-mapped break reintroduces a
  **P1.7a-class soundness bug** (the engine called a non-model a model / a SAT
  puzzle ⊥). Rushing it under one session's budget was judged the wrong risk
  against the "no behaviour change" constraint. It remains the highest-value
  follow-up; do it incrementally with the acceptance gate after each step.
- **Bounded items left on the table** (lower-value or coupled to the flagship):
  `rebuild_indexes` decomposition + snapshot shallow-copy + typed index
  wrappers (F-KB-2/6/9); `from_ir.load` / `parse_trace_steps` nesting
  flattening (F-KB-7 / F-RTC-4, still depth 8/9); the `node()/edge()/cluster()`
  emitter API + `cli._build_parser` split + trace-pipeline unification +
  `linearize` dispatch (rest of S1.7b.5); the F-KER-6/7/10/15 helper
  unifications; F-KB-13 type annotations.

**Acceptance scorecard (S1.7b.6):** #1 behaviour-unchanged ✅ · #2 both bugs
closed with tests ✅ · #3 metrics *partially* moved (0 dead funcs ✅, but
functions >120 lines and depth-≥8 nesting remain in the deferred items —
**not** met) · #4 no public-API churn ✅ · #5 hot-path no-regression ✅
(4:43 ≈ 4:39, same enterings).
