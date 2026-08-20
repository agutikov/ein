# The oracle ledger — what only ein.py proves, and who owns it now

**Stage:** [S1a.10.1](s1a.10.1_bank_the_oracle.md) · **Phase:**
[P1a.10](README.md) · **Written:** 2026-08-20
**Gate:** nothing in this phase is deleted until every row here has an owner.

---

## How to read a row

Each row is one **behaviour the differential setup asserts** — not one test and
not one file, because the same behaviour is often asserted twice (once by the
harness over the CLI, once by an ein.rs test over the library) and the question
is whether *anything* still asserts it afterwards.

| disposition | meaning |
|---|---|
| **covered** | an ein.rs test already asserts it, without the oracle. The test is named. |
| **banked** | a new ein.rs test asserts it, and that test landed in S1a.10.1. |
| **retired** | it was a claim *about ein.py* — its exception classes, its `argparse` layout, its `sorted()` raising — and it dies with its subject. |
| **accepted loss** | nothing will assert it again. Every one of these carries a sentence in [§6](#6-accepted-loss) saying what could now pass unnoticed. |

**The measurements this ledger is written against**, all 2026-08-20 on
`master` @ `d4de23f`:

| | |
|---|---|
| corpus | 111 entries, 623 declared cells; a default run is **505 cells over 94 entries** (`slow` excluded) |
| last full T3 run (PyPy 3.11 vs `ein.rs/target/release/ein`) | **503 same, 2 DIFF**, 0 skip, 738.2 s of engine time. Both DIFFs are [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers) |
| `cargo test --workspace` before this stage | **312 tests**, 9 m 13 s, green |
| — of which **integration** tests | 91, in 28 files |
| — of which **reach the oracle** | **42** — see [§2](#2-the-finding-46--of-einrss-own-integration-tests-are-differential) |
| `cargo test --workspace` after it | **319 tests** — 3 integration, 4 unit — and 18 s of the added time |
| pytest | **1 517** unit + **21** acceptance |

---

## 1. The four tiers

`ein-conformance` runs one corpus cell as one process pair and compares it at a
tier ([design/01 §2](../design/01_parity_contract.md#2-the-four-tiers)). What
each tier reads is mechanical, so the rows below are per *observable*, not per
tier.

| # | what the harness asserts | tier | disposition | owner after |
|---|---|---|---|---|
| 1.1 | the verdict type, `k`, `exhausted`, the model as a fact set, `goal_bindings`, the unsat core | T0 | **covered** + **banked** | `ein-infer/tests/acceptance.rs` asserts the *answer* on the three zebra fixtures; `ein-cli/tests/summary_properties.rs` asserts the verdict block is internally coherent on 365 cells; `ein-render/tests/golden/corpus_shapes.md5` pins the bytes |
| 1.2 | the process exit code | T0 | **covered** | `ein-cli/tests/help_parity.rs` for usage errors; the negative fixtures' `.expected` files for load/parse errors |
| 1.3 | every search counter — `enterings_*`, `layers_explored`, `saturate_count`, `nogoods_*`, `facts_merged`, `forced_positives`, `solution_nodes` | T1 | **banked** | `summary_properties.rs` — thirteen identities, measured over the corpus before being written down |
| 1.4 | `Saturator.{naf_rounds, naf_admitted, naf_retired, naf_dropped}` | T1 | **banked** | `summary_properties.rs`; `naf_dropped == 0` is asserted **with its reason** (S1.21.8's boundary), not as a number |
| 1.5 | `HypGenStats.{raw, emitted, filtered.*, pre_candidate.*}` | T1 | **covered** + **banked** | `hypgen_parity.rs::every_filter_and_skip_fires_somewhere_in_the_corpus` (no oracle); `summary_properties.rs` adds the conservation law `raw == emitted + Σ filtered` |
| 1.6 | fact counts per relation after root saturation | T1 | **banked** | `summary_properties.rs`: `root.facts == Σ root.facts_by_relation`, plus the digest of `plan` / `hyp` / `solve` shapes |
| 1.7 | the resolved `config` block, key for key | T1 | **banked** | `summary_properties.rs::CONFIG_KEYS` — the 17 keys, exactly; a `SolverConfig` field added without being reported fails |
| 1.8 | the **ordered event log** — every firing, park, admission, retirement, entering, no-good, hypgen decision | T2 | **covered** + **banked** | `ein-infer/tests/golden_events.rs` (3 whole streams, and a test that they *contain* every elided class); `corpus_shapes.md5` digests `solve[*]`, `commit[*]`, `lattice`, `naf` over 111 files |
| 1.9 | `solve` / `saturate` stdout, `--stats`, the solution table | T3 | **banked** | `corpus_shapes.md5` — `trace[answer]`'s `--- table` block is the same text |
| 1.10 | `ein render {rules,rule,constraints,lattice}` DOT | T3 | **covered** + **banked** | `golden_dot.rs` (16 checked-in artefacts); `corpus_shapes.md5` digests **17** views × 111 files, which is the superset |
| 1.11 | `--trace` markdown, DOT blocks included | T3 | **covered** + **banked** | `golden_trace.rs` (5 goldens), `idea08_acceptance.rs` (the walkthrough rules); `corpus_shapes.md5` digests `trace[trace|answer|no-proof]` |
| 1.12 | `--dump-states` — `summary.json`, `00_timeline.jsonl`, per-layer snapshots | T3 | **covered** + **banked** | `golden_dump.rs` (2 goldens); `corpus_shapes.md5` digests all 5 dump modes |
| 1.13 | every parse error, `file:line:col` and message | T3 | **covered** + **banked** | `ein-ir/tests/parse_parity.rs` is oracle-gated, **but** `examples/broken/**`'s checked-in `.expected` files are not: `load_parity.rs::the_load_negative_fixtures_are_byte_identical` and `imports_parity.rs::the_import_and_macro_failures_are_byte_identical` hold ein.rs to the same bytes ein.py was held to |
| 1.14 | every `KBLoadError`, in pass order | T3 | **covered** | as 1.13 — 29 `load-negative` fixtures, each with its `.expected` |
| 1.15 | `--help` for every subcommand | T3 | **retired** as bytes, **covered** as structure | `argparse` and `clap` never agreed on layout ([Q-M1a.13](../open_questions.md#q-m1a13--argparse-surface-parity)); the *content* comparison is `help_parity.rs::the_extractor_finds_the_whole_surface` + `a_renamed_short_key_fails`, neither of which needs the oracle. What dies is `the_surface_matches_argparse` |
| 1.16 | `crash-parity` — exit code + exception class on the 8 `ein-bugs` fixtures | T3 | **retired** | The claim is "ein.py raises `TypeError` here". ein.rs *answers* on both of the two cells that differ ([D2](../divergences.md)); the other 11 are agreement on an error that is ein.py's, not the language's. See [§6](#6-accepted-loss) L4 |

### The tier that has no successor, stated plainly

T2's *ordering* claim is the weakest-covered of the four. A digest pins the
whole stream, so a reordering fails — but it fails as "this rendering moved",
where the harness failed as "event 4 122 is `fire(R2)` on one side and
`fire(R7)` on the other". The difference is diagnostic, not detective: both
notice.

---

## 2. The finding — 46 % of ein.rs's own integration tests are differential

This is not in the phase's plan and it is the largest single row in this
ledger. **`cargo test --workspace` is the phase's stated gate, and 42 of its
91 integration tests shell out to `ein.py`.**

Measured by making `python3` unrunnable and running the suite:

```text
$ PATH=<a python3 that exits 127> \
    cargo test --workspace --no-fail-fast -- --nocapture --test-threads=1
318 passed; 1 failed
```

**41 of those 318 passes asserted nothing** — one `SKIP` line each, on a
stderr `cargo test` captures — and the 1 failure is the forty-second, which
panics instead. `--test-threads=1` is not decoration: without it the skip lines
interleave and cannot be attributed to a test.

Three separate problems, and only the first is obvious:

1. **The tests skip rather than fail.** `ein_oracle::skip` prints to stderr —
   which `cargo test` **captures for passing tests**, so a skipped parity test
   is indistinguishable from a passing one unless someone runs with
   `--nocapture` *and* `--test-threads=1` and reads the noise. The helper's own
   doc comment says stderr is used *because* stdout is swallowed; both are. So
   does the CI file, which installs `ein.py` into the Rust job and explains
   why: "Without this they skip — **loudly**, but they skip, and a silent gate
   is not a gate" (`.github/workflows/per-commit.yml`). The reasoning is right
   and the adverb is wrong, which is the only reason this went unnoticed.
2. **One test panics instead** — `help_parity.rs::the_surface_matches_argparse`
   `expect`s the oracle. It is the only honest one in the set, by accident.
3. **Five crates carry `ein-oracle` as a dev-dependency**, so the delete is a
   compile error in five `Cargo.toml`s, not a test failure.

| what the 42 assert | disposition | owner after |
|---|---|---|
| the parser accepts/rejects what `lark` does, with `lark`'s message (`parse_parity`, `fuzz_parity`) | **covered** for the corpus + the 4 `broken/` fixtures (their `.expected` files); **accepted loss** for the *fuzzed* input space — [§6](#6-accepted-loss) L1 | |
| the dumper is byte-identical to `ir/dump.py`, and `parse ∘ dump` is the identity (`dump_parity`) | **covered** + **banked** | `the_goldens_are_reproduced` + `dump_then_parse_is_a_fixed_point`, both oracle-free; `zebra.golden` / `zebra2.golden` are ein.py's own bytes — see [§4](#4-what-the-removal-must-relocate); `corpus_shapes.md5` adds `ir[parse]` and `ir[dump-compact]` over every file |
| imports, macro expansion, module-path mangling (`imports_parity`) | **covered** for the failures (`.expected` fixtures); **banked** for the successes | `corpus_shapes.md5` digests `ir[resolve]`, `ir[minimize]` and `ir[expand]` — the same three ops `imports_parity` sweeps, rendered through the same `dump_canonical` |
| the KB after load, plan for plan, match for match (`load_parity`, `compile_parity`, `match_parity`, `saturate_parity`) | **banked** | `corpus_shapes.md5` — `plan`, `match`, `hyp`, `lattice`, `naf` digests over 111 files |
| the whole solve, in five regimes (`hypgen_parity`, 11 sweeps) | **banked** | `corpus_shapes.md5` — `solve[default|exhaustive|shuffled]`, `commit[±fail-fast]`, `explain[±alts]` |
| CPython's `repr`, `float` formatting, `sorted` over mixed types, the int pool's canonicalisation (`cpython_parity`, `values_parity`) | **accepted loss** — [§6](#6-accepted-loss) L2 | the *reached* shapes are pinned by every digest that renders a value; the *unreached* ones lose their sweep |
| every DOT view, the trace, the dump tree (`dot_parity`, `trace_parity`, `dump_parity`) | **banked** | `corpus_shapes.md5`, 4 228 renderings |
| the `--help` surface (`help_parity`) | **retired** — row 1.15 | |

**None of this changes before S1a.10.2**, which is where the 42 lose their
oracle arm. What S1a.10.1 owed was the answer to "and then what asserts it",
and the answer is the manifest — which is why the manifest had to be blessed
*here*, in a tree where the differential half was still running and green.

---

## 3. The instruments that are not tiers

| # | instrument | what it proves | disposition | owner after |
|---|---|---|---|---|
| 3.1 | **the determinism sweep** — `--env-a PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42 --strict`, which found hazards H1 and H4 | no output depends on a hash-order accident | **banked** | `ein-render/tests/id_order_invariance.rs`. One engine, the id space permuted instead of the hash seed — and it is *stronger*, because ein.rs has no salted hash to perturb: see [§5](#5-what-the-successor-found) |
| 3.2 | **`utils/check_hashmap_iteration.py`** — the grep for an iteration whose order could reach an output | the static half of the same question | **covered**, and repaired | it reads Rust source and has no ein.py dependency; it is listed here because 3.1 is often mistaken for it. The grep finds what *could* leak, 3.1 finds what *does*. It was **red on `master`** when this row was written — six unannotated aggregate iterations, every one of them a `.sum()` or a histogram, arriving one at a time from S1a.5.4 (`index_sizes`), T1a.6.2.2 (`layout_shape`) and S1a.6.1 (`footprint`) — so the lint has been red since 2026-08-18, one day after it landed. All six now carry their `determinism-ok:` reason. That the CI check *for hash-order leaks* had itself been failing for two phases is a datum about the cost of a gate nobody watches, and the reason [§3](#3-the-instruments-that-are-not-tiers).1 is a `cargo test` rather than a script |
| 3.3 | **the differential fuzzer** (`utils/fuzz_ein.py`) — four parity bugs in its first twenty minutes | the input space no fixture covers | **accepted loss** for the differential arm — [§6](#6-accepted-loss) L1; the generator, the minimiser and the self-checkable properties survive to S1a.10.4 | |
| 3.4 | **the parser fuzzer** (`ein-ir/tests/fuzz_parity.rs`, 2.2 M mutations) | ditto, for the frontend | **accepted loss**, same row; its *seed replay* survives — the checked-in `fuzz_findings/` still have to parse |
| 3.5 | **the liveness check** — "did either implementation ever exit 0?" | a harness that cannot fail | **retired** | it exists because two dead engines agree. One engine that never runs is a test that never passes |
| 3.6 | **the corpus completeness check** — every `.ein` under `examples/` and `stdlib/` has an entry | the corpus cannot silently miss a file | **covered** (5 claims) + **banked** (4) | `ein-conformance/src/corpus.rs`'s unit tests had 5 of the 9 `ein.py/tests/test_corpus_manifest.py` makes; the missing four — unique paths, negatives grouped by where they fail, every compile-negative has its `.expected`, the load-negative group matches its directory — landed here |
| 3.7 | **`ein-parity`'s relaxation is load-bearing** — 8 + 10 unit tests, and `utils/mutant_ein.py` deleting one event from a shipping binary's log | the D3 cut still catches a dropped productive firing | **covered** | the unit tests need no oracle; `mutant_ein.py` runs one binary twice and is already single-engine |
| 3.8 | **the acceptance gate** — 21 tests, the three zebra2 task classes | the *answer*, not the agreement | **covered**, partly; the rest is [S1a.10.2](s1a.10.2_port_the_suite.md)'s | `ein-infer/tests/acceptance.rs` (3) + `ein-render/tests/idea08_acceptance.rs` (5) |

---

## 4. What the removal must relocate

Not a disposition — a **defect list for [S1a.10.5](s1a.10.5_removal.md)**.
These ein.rs tests are oracle-*free* and would have survived the experiment in
§2 unscathed, because they do not run Python: they read files that live under
`ein.py/`. A `git rm` breaks all five.

| test | reads |
|---|---|
| `ein-ir/tests/dump_parity.rs::the_goldens_are_reproduced` | `ein.py/tests/golden/{zebra,zebra2}.golden` |
| `ein-render/tests/golden_trace.rs` | `ein.py/tests/golden/trace_3step.md` |
| `ein-render/tests/derivation_dot.rs` | `ein.py/tests/golden/dot/kb_provenance_dag.dot` |
| `ein-render/tests/golden_dot.rs` (7 tests) | `ein.py/tests/golden/dot/**` (15 files) + `kb_zebra_unified.dot` |
| `ein-conformance/src/corpus.rs::tracked` | scans `ein.py/src/ein/stdlib` as a fallback stdlib location |

**Move the files, do not regenerate them.** All 19 are ~96 KB and every one of
them is *ein.py's own output*, checked in. Carried across by `git mv` they keep
saying "ein.rs reproduces what the other implementation produced"; blessed
afresh from ein.rs they would say "ein.rs reproduces itself", and the
distinction is the only independent provenance the repo has left. Recommended
destination: `ein.rs/crates/<crate>/tests/golden/`, beside the twelve
S1a.6.11 goldens, with a header note naming their origin.

---

## 5. What the successor found

`id_order_invariance` is [§3](#3-the-instruments-that-are-not-tiers).1's
replacement and it is the one row here that produced a result rather than a
disposition.

Each corpus file is run twice: once ordinarily, and once from a `Terms` where
every name it will intern has already been interned **in a shuffled order**,
every integer literal likewise, and every fact re-interned in an order shuffled
within its nesting depth. Same file, same code, different integers.

| | 1 seed | 8 seeds |
|---|---:|---:|
| `(file, op)` pairs permuted | 2 544 | 2 544 |
| pairs with no ids to permute — `dot`'s parse views and every `ir[*]` op, which answer off the AST and never build a KB | 1 684 | 1 684 |
| permutations run | 2 544 | 20 352 |
| **renderings that moved** | **66** | **495** |
| — only where a dying fork stopped (`firings`, `n_firings`, the event ordinal, a `dead-post` core) | 44 | 310 |
| — only in the *body* of a rendered derivation | 22 | 185 |
| **answers that moved** | **0** | **0** |
| wall clock | 10.5 s | 48.0 s |

**The answer does not depend on which integer a name got. The proof does.** And
what moves is *exactly* the three observables
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
already calls narration — a dying fork's stopping point, a firing count, and
which of a fact's equally valid justifications was recorded first — now reached
from **inside one engine** instead of between two. `EIN_PARITY_STRICT=1` turns
the cut off and prints the 66 by op:

| where a dying fork stopped | | in the body of a derivation | |
|---|---:|---|---:|
| `solve[exhaustive]` | 8 | `trace[trace]` | 7 |
| `solve[shuffled]` | 8 | `dot[slice]` | 5 |
| `solve[default]` | 7 | `trace[answer]` | 5 |
| `dump[lattice]` | 7 | `trace[no-proof]` | 4 |
| `dump[monotonic]` | 7 | `dump[snapshot]` | 1 |
| `dump[progress]` | 7 | | |
| **44** | | **22** | |

Every name in the right-hand column is already on
`ein_parity::is_narration`'s closed list. **Nothing had to be added to it.**

**This is P1a.8's problem, not only P1a.10's.**
[`intern`](../../../ein.rs/crates/ein-core/src/intern.rs)'s module note says
`.einb` "is the only thing that crosses interner boundaries, and it remaps on
open" — that remap is a permutation, and
[P1a.8](../p1a.8_binary_container/README.md)'s gate is `ein solve x.einb`
**byte-identical** to `ein solve x.ein`. On the numbers above that gate is
reachable for the answer and not reachable for the trace, unless the remap is
order-preserving. Recorded in [open_questions](../open_questions.md) rather
than solved here.

**The mutation floor.** Reversing `apriori::layer_1`'s comparator — the
traversal order, and D2's own site — moves **138 of 4 228** renderings in
`corpus_shapes`, and doubling `enterings_dead_post` breaks **137 of 365** cells
in `summary_properties` on two identities. Both instruments were checked
against a deliberate break before being trusted, because a golden nobody has
ever seen fail is a file, not a test.

---

## 6. Accepted loss

Four rows. A short list is a result; an empty one would be a claim to be
suspicious of.

**L1 — the differential fuzzers.** `utils/fuzz_ein.py` found four real parity
bugs in twenty minutes on a surface five phases had signed off
([S1a.6.6](../p1a.6_performance/s1a.6.6_differential_fuzzer.md)), and
`ein-ir`'s parser fuzzer ran 2.2 M mutations. Both compared **two engines on
generated input**, and neither has a second operand afterwards.

What survives, decided here and implemented by
[S1a.10.4](s1a.10.4_utils.md) T1a.10.4.2 — the generator and the minimiser are
untouched, and every property that one engine can check on its own is kept:

1. **no panic, and no non-zero exit that is not a diagnosed error** — the
   generated program either loads or is refused with a message;
2. **`dump → parse → dump` is a fixed point** on every generated program;
3. **id-order determinism** — the same program under a permuted interner
   answers the same way ([§5](#5-what-the-successor-found)'s instrument,
   applied to generated input rather than to the corpus). This is what
   replaces "the same program under two `PYTHONHASHSEED`s";
4. **`--jobs` invariance**, when [P1a.7](../p1a.7_parallelism/README.md)
   resumes and there is a second value of `--jobs` to be invariant under.

None of the four bugs the fuzzer found would have been caught by any of
those — they were all *wrong answers*, not crashes — and the script's header
must stop advertising "four parity bugs in twenty minutes" as something the
surviving arm can do.
*What could now pass unnoticed:* a wrong answer on a program shape nobody wrote
a fixture for. This is the single largest loss in the phase and it has no
mitigation other than [P1a.11](../p1a.11_stdlib_conformance/README.md)'s
stated expectations.

**L2 — CPython's value semantics on unreached shapes.** `cpython_parity` and
`values_parity` sweep *every reachable value shape*, every code point where the
`repr` escape table turns over, and CPython's float formatting — far beyond
what any `.ein` file contains. Afterwards, only the shapes the corpus actually
renders are pinned.
*What could now pass unnoticed:* a `repr` or float rendering that is wrong for
a value no corpus file holds — a large negative integer, a code point in a
plane the corpus does not use — surfacing the first time a real puzzle uses one.

**L3 — the second reading of the specification.** ein.py is an *implementation
of the same document*, written by a different route, and every place the two
agreed was a place two independent readings of `docs/kernel/` coincided. That
is gone in a way no test replaces: a self-golden says "still what it was", never
"and what it was is right".
*What could now pass unnoticed:* a misreading of the kernel semantics that
ein.rs has held consistently since P1a.3. Nothing in this ledger can find one.
[P1a.11](../p1a.11_stdlib_conformance/README.md) exists partly for this and is
the only row of it that is being built.

**L4 — `crash-parity`.** Eleven cells asserted that both engines die the same
way on the same input. Afterwards ein.rs *answers* on the two D2 fixtures and
the group's remaining claim is "ein.rs does not crash", which the corpus sweep
already makes.
*What could now pass unnoticed:* nothing about the language. The row is here
because retiring a group of 13 cells deserves to be written down rather than
noticed later as a count that changed.

---

## 7. Two counters the harness compared 505 times and learned nothing from

Not a loss — a **finding**, and the reason `summary_properties.rs` asserts
zeroes with reasons rather than trusting a golden.

- **`stats.enterings_dead_pre` is 0 on all 176 solve cells, and structurally
  so.** A fork is `dead-pre` when `contradiction::detect` fires on the
  hypothesis facts alone, which needs a commitment holding some `X` **and**
  `(not X)`. `hypgen::generate` only ever proposes positives and drops any
  whose negation is already believed (`negated_fact`), and
  `apriori::filter_candidate` re-drops any candidate whose element left
  `alive` — which is exactly what the singleton writeback does. So no
  commitment can carry a pair. Every death in this engine is `dead-post`.
- **`stats.nogoods_subsumed` is 0 on all 176.** This one is a claim about the
  *corpus*, not the engine: no entry reaches a death whose learned clause is
  already implied. A fixture that does is a corpus growth item, not a bug.

`naf_dropped` is the third and was already known ([design/01
§2](../design/01_parity_contract.md#2-the-four-tiers): "structurally 0 since
S1.21.8"). All three were compared on every T1 cell of every run since P1a.4,
and two zeroes agree for the wrong reason.

---

## 8. The divergence ledger, re-read

| entry | what it records | after the oracle |
|---|---|---|
| [D1](../divergences.md#d1--a-rule-may-not-bind-more-than-256-variables) — 256 registers | a limit ein.py does not have | **ein.rs's own defined behaviour.** Already fixtured without an oracle: `compile_limits.rs`, 3 tests, one of which measures the corpus's distance from the ceiling. Nothing to do |
| [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers) — `sorted(alive)` raises in ein.py | ein.py crashes where ein.rs answers, in two shapes | **historical.** The two fixtures stay in `examples/ein-bugs/` and stay in the corpus — they are now ordinary entries whose answer is pinned by `corpus_shapes.md5`. What is lost is the *assertion that ein.py still raises*, which is asserted in six places today (`DIVERGENT` lists in four sweeps) and in none afterwards. That is not a loss worth a row in §6: it is the point of the phase |
| [D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it) — a fork resumes root's saturation | the two engines narrate different amounts of one derivation | **promoted, not retired.** §5 shows the same three observables move *within* ein.rs under a permuted id space, so `ein-parity`'s rule stops being a statement about two engines and becomes a statement about what a derivation *is*. `ein-parity` therefore survives the harness that motivated it — which [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) T1a.10.3.3 should read before deciding the crate's fate |

---

## 9. Cross-links

- [S1a.10.1](s1a.10.1_bank_the_oracle.md) — the stage
- [design/01 §5](../design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list)
  — the normalisation list, whose closed set §5 above re-uses unchanged
- [divergences.md](../divergences.md) — D1–D3, re-read in
  [§8](#8-the-divergence-ledger-re-read)
- [`conformance/README.md`](../../../conformance/README.md) — the harness,
  retired by [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md)
