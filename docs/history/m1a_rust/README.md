# M1a — the Rust port (ein.rs)

**Ran 2026-08-17 → 2026-08-23. Shipped.** Eleven phases, 53 stage documents,
and an engine that replaced the one it was measured against.

> **This is history, not a plan.** M1a's plan tree — `plans/m1a_rust/`, its
> milestone README, eleven phase READMEs and 53 stage files, 65 files and
> 13 950 lines — was deleted on 2026-08-23,
> once the milestone had shipped and the only thing a stage file could still do
> was describe work that was already done. What is kept is what is still
> *read*: this record, the design contracts, the measurements, the divergence
> ledger, the questions and the oracle ledger. The stage files are in git
> history — `git log --diff-filter=D -- plans/m1a_rust` names the commit, and
> `git show <commit>^:plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md`
> reads one.
>
> **The instruments these numbers were taken with are mostly gone.** `ein.py`,
> `ein-conformance`, `ein-oracle`, the T0–T3 tiers and eleven `utils/` scripts
> left the tree at S1a.10.3–S1a.10.5, so every figure here is a **record**
> rather than something that can be re-run. Which question each retired script
> answers now is the census in
> [`utils/README.md`](../../../utils/README.md#the-census); the CPython and
> PyPy columns in [`measurements/`](measurements/) are **frozen constants**,
> because the instruments that produced them left with the engine they
> measured.

## What is in this directory

| file | what it is |
|---|---|
| [`design/`](design/README.md) | the eleven design contracts — the *how* of the port, and the documents `ein.rs` source comments cite as their specification. `design/README.md` § Measured is the per-phase measured record |
| [`measurements/`](measurements/) | [`baseline.md`](measurements/baseline.md) (P1a.6, 2 898 lines — the optimisation programme's every profile), [`scaling.md`](measurements/scaling.md) and [`shared_state.md`](measurements/shared_state.md) (P1a.7), [`corpus_cost.md`](measurements/corpus_cost.md) and [`feature_cost.md`](measurements/feature_cost.md) (P1a.9), [`fork_delta_trace.md`](measurements/fork_delta_trace.md) (S1a.6.9) |
| [`divergences.md`](divergences.md) | D1–D3 — where the two engines differed, and why each was accepted rather than fixed |
| [`open_questions.md`](open_questions.md) | Q-M1a.1–23: twenty-one settled, **two still open on purpose** |
| [`oracle_ledger.md`](oracle_ledger.md) | what only a second implementation proved, and what owns each claim now |
| [`suite_dispositions.md`](suite_dispositions.md) | the Python suite, file by file: what was ported, what was already covered, and the 96 subjects that died with the thing they tested |

---

## What shipped

`ein.rs` is the engine — **the only one**. Both of the milestone's invariants
held the whole way: **I1**, a 1:1 observable surface, gated against `ein.py`
until [P1a.10](#p1a10--one-implementation) banked what only an oracle could
prove and retired it; **I2**, a free hand inside, cashed out as integers
instead of objects, a register matcher, a layered copy-on-write KB and a
fanned-out lattice layer.

| | measured | where |
|---|---|---|
| `solve zebra2.ein -e` end-to-end | PyPy **4.53 s → 29.0 ms (157×)**, peak RSS 223 → 17 MB | [baseline.md](measurements/baseline.md) |
| `solve zebra.ein -e` end-to-end | 8.33 s → **47.5 ms (175×)**; **585.8 → 47.5 ms** across P1a.6 alone | [baseline.md](measurements/baseline.md) |
| parse + load `zebra2.ein` | 0.43 s → **0.67 ms (642×)** | [baseline.md](measurements/baseline.md) |
| the three acceptance fixtures | 36.0 s under PyPy → **0.127 s** | [baseline.md](measurements/baseline.md) |
| `--jobs 8` on 8 P-cores | **3.17–4.40×** on the measurement set (`branching/06 -e` 194.2 → 44.1 ms) against a **≥ 6×** target — not met, and named | [scaling.md](measurements/scaling.md) |
| `--jobs N` is the same computation | 20 712 (file, op, jobs) cells, **0 moved**, 30 s; the verbose event stream byte-identical at both job counts; 10 000 paired fuzz runs, zero findings | [scaling.md](measurements/scaling.md) |
| memory under parallelism | per-worker provenance: `features/01 -e` **684–708 → 85–91 MB**; peak RSS 79.8 / 90.3 MB at `--jobs 1 / 16` | [shared_state.md](measurements/shared_state.md) |
| `.einb`, the binary KB container | a saturated `zebra2` is **57 688 bytes**, opens cold in **0.614 ms**; `solve x.einb` byte-identical to `solve x.ein` | [P1a.8](#p1a8--binary-kb-container) |
| the gate | 312 tests in 9 m 13 s with a Python process in 42 of them → **619 tests, 0 failures, 1 m 51 s**, no Python in any of them (619 re-counted 2026-08-23 after the tree came out) | [oracle_ledger.md](oracle_ledger.md) |
| what replaced the two-engine oracle | 4 228 renderings banked as digests, 13 counter identities over every solve cell, an id-space permutation sweep: **0 answers moved**, 66 renderings (all narration); four accepted losses, named | [oracle_ledger.md](oracle_ledger.md) |

**The estimate was ~7 months and it took seven days.** 156 days of stage
estimates, run 2026-08-17 → 2026-08-23. That is worth recording
without a lesson attached to it: the plan's *shape* — parity first, speed
second, scale third — is what the work followed, and its duration is not.

### The two invariants, and how each ended

> **I1 — Outside, nothing changes.** ein.rs is a drop-in replacement for
> `ein`: same surface language, same CLI, same stdout bytes, same exit codes,
> same DOT, same markdown trace, same verdicts, same counters, same error
> messages. Any observable difference is a bug in ein.rs, not a design liberty.
>
> **I2 — Inside, everything is on the table.** Atoms and facts become integers,
> tuples become flat interned rows, the fork becomes a zero-copy layer, the
> matcher becomes a register machine, the search layer runs on many cores, and
> a loaded KB can be stored and mapped back from a binary file. None of that is
> allowed to leak through I1.

**I1 was discharged at [P1a.5](#p1a5--presentation-and-cli)** — T3 over the
whole corpus × run matrix, 472 of 473 cells byte-identical, the one exception
[D2](divergences.md) — and then **amended twice, both times in writing**. At
[S1a.6.9](#s1a69--the-fork-entry-delta-the-resumed-saturator) the hard
requirement became *the same answer* rather than *the same bytes*, because a
fork that resumes root's saturation narrates a quarter as much and that is
worth the last unmet performance target ([D3](divergences.md),
[Q-M1a.18](open_questions.md)). At [P1a.10](#p1a10--one-implementation) the
clause "`ein.py/` stays in the repo permanently as the oracle" was reversed:
the case for the oracle was never that a second implementation is valuable in
itself, but that **a rewrite with a byte-exact oracle is a measurable
rewrite** — and that argument expires when the byte gate closes.

I1 is what made I2 safe. It is also why [P1a.0](#p1a0--conformance-harness-and-shared-assets),
the harness, came before a single line of engine code.

---

## What was declined, and on what number

The record of what was *not* built is the milestone's most reusable output.
Every row was decided by a measurement, and every one of them is written down
where a reader who proposes it again will find it.

| declined | the number that declined it |
|---|---|
| **Server mode** (8 stages: daemon, sessions, JSON-RPC, streaming, a solution cache) | no consumer — the GUI links the crates, the CLI is the only other client. Dropped 2026-08-18; `design/09` and the seven stages are in git history |
| **The PyO3 binding** | every candidate consumer wants something else: the GUI links crates, the benchmark runner *must* shell out to be fair, M2's llama.cpp is an HTTP server. Deferred 2026-08-21 with three trip-wires ([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)); `docs/api/`'s five Python pages are kept whole as the specification a trip-wire would restore |
| **Beta-memories** (F11 D1) | the intermediate they materialise is **2.2 tuples per step entered** (47.4 before the phase, 0.45 after), and a per-fork copy of that shape measured **+7.6 %**. An index key was the lever, not a memory ([Q-M1a.10](open_questions.md)) |
| **WCOJ** (F11 D2) | the cyclic body exists in `std.slots`; the cost trigger is further away at the end of the phase than at the start |
| **Win B's semi-naive guard re-evaluation** | a measured **1.4–2.2 %** ceiling ([Q-M1a.17](open_questions.md)) — and the ≥ 80 % target that named it was itself wrong: monotone guards are 11 % of guard evaluations on `zebra2` and 30 % on `zebra`, because a failing monotone guard is retired on the spot |
| **design/08's speculate-and-validate** | 248 of 248 writebacks are in layer 1, so a fanned-out layer has no `W` to repair |
| **A concurrent interner, and a lock on the fact store** | four distinct names arrive between root saturation and the end of a solve; a search assigns 41–417 fact ids against 5.8–26 M reads, and four of six workloads append **zero** per entering |
| **`loom`, and the multi-threaded stress** | every structure design/08 §6 named ends `&`-shared or per-worker, so there is no protocol to model |
| **Level 3, the parallel boundary round** | 0.0 % of three of the four measurement-set workloads, which never park a candidate; a median of one on the fourth, and a round is 0.18 µs against a ~10 µs barrier |
| **Level 2, the parallel enqueue pass** | right about the share (10.6–31.2 %), wrong about the width: **1.4–3.1 tasks per pass** against a fan-out of 8 |
| **`--unordered`** | **0 %** — `fan_out` is a barrier, so when the ordered commit runs every worker has already finished |
| **An `events` compile-time feature** | it never existed (the emitter is unconditional), and the strongest version of it — `Events::on()` folded to `false` — is **+3.9 % / +1.8 % slower** |
| **`zstd` and `boxcar`** | priced and not taken up; recorded beside the two above rather than left as ideas |

**Levels 2 and 3 share one sentence with `--unordered`:** they fan out units
that arrive one to three at a time and cost a fraction of a microsecond, inside
loops that run hundreds of thousands of times. And the reason is worth more
than the decision — **incrementality and parallelism compete for the same
work, and P1a.6 got there first**: S1a.6.12's epoch invalidation and S1a.3.4's
delta seeding had already removed the bulk those levels were specified to
spread over cores.

---

## The eleven phases

Each phase's stages are listed under it. A stage's entry says what it landed
and what it cost; the tables and profiles behind the numbers are in
[`measurements/`](measurements/), and the contracts they were built against are
in [`design/`](design/README.md).

### P1a.0 — Conformance harness and shared assets

**4 stages, shipped 2026-08-17.** Build the machinery that makes "100 % surface
match" a measurable claim *before* there is anything to measure: at the end of
the phase the repo could prove **ein.py ≡ ein.py** across the whole corpus at
every tier — 556 cells over 95 entries, 0 differences — which sounds circular
and is exactly the point. A harness that cannot detect a difference between an
implementation and itself cannot detect one between two implementations either.

**The premise was "building the harness first finds ein.py bugs", and it found
five**, three of them predicted: the `__symmetric__` mirror seed iterating a
`frozenset` (so with ≥ 2 markers the firing order depended on
`PYTHONHASHSEED`); mixed `str`/`int` hypothesis args crashing `apriori.layer_1`
(narrowed to hrule-generated candidates); `--shuffle` confirmed benign; and
one nobody predicted — `unsat_core` iterated raw at two display sites, so **the
same puzzle produced two different `--trace` files across runs**. Plus one bug
in the harness itself, found by its own first whole-corpus run: it polled a
child whose stdout was a pipe, so `render lattice` blocked on `write` while the
harness waited for it to exit. Two 0.3 s cells sat for two minutes. It is the
failure mode a harness is least able to report on itself — it does not crash
and it does not diff, it simply never finishes.

The phase also re-measured the acceptance gate the milestone was targeting:
**43.7 s under PyPy 3.11, not the ~91 s** the plan carried from M1.

#### S1a.0.1 — Parity contract, corpus manifest, divergence ledger
The four tiers, the corpus manifest and the empty ledger
([design/01](design/01_parity_contract.md), [design/02](design/02_determinism_and_order.md)).
The determinism audit is this stage's, and so are H1–H4.

#### S1a.0.2 — The oracle event protocol
`--events FILE` and `--events-level {normal,verbose}` on `solve` and
`saturate`, 17 event kinds, versioned `ein-events/1` — the schema that outlived
the harness and is now
[`docs/kernel/inference/events.md`](../../kernel/inference/events.md). The
protocol gained two things over design/01 §3: `hypskip` as its own kind, and a
`hyp` verdict that is a *filter name* rather than a boolean, so a counter
difference locates itself.

#### S1a.0.3 — Shared stdlib and examples
The stdlib moved to repo-root [`stdlib/`](../../../stdlib/README.md) — seven
modules, a README and `MANIFEST.sha256` — with one resolution order in both
implementations (`$EIN_STDLIB` → the checkout → the embedded copy) and a drift
check that was verified by corrupting a module and watching the test name the
file ([design/11](design/11_shared_assets.md)).

#### S1a.0.4 — Workspace skeleton and CI
The `ein.rs/` workspace, the CI tiers, the benches — and the **determinism
lint** (`utils/check_hashmap_iteration.py`, with `// determinism-ok: <reason>`
as its escape hatch), whose rule is stronger than "don't use `RandomState`":
`FxHashMap` is deterministic run to run, but its order is still an artefact of
hash values and insertion history, where ein.py's observables come from
insertion-ordered `dict`s and explicit `sorted()`.

### P1a.1 — IR frontend

**3 stages, shipped 2026-08-18.** Read `.ein` and write it back byte-identically
— lexer, parser, AST, dumper, macro expander, import resolver
([design/04](design/04_ir_frontend.md)). **95 files and 2.2 M fuzzer mutations,
0 differences.** Parse is **758 µs against 760.6 ms CPython and 230.9 ms PyPy**
(1 003× / 305×); `zebra2` parse + resolve + expand is **824 µs**.

**Two Lark artefacts reach the output and are reproduced rather than
corrected**, which is what I1 means in practice: the dynamic lexer is not
maximal munch, so `(rulex …)` parses as a rule named `x` (eighteen literals
behave this way, and the parser therefore lexes on demand and backtracks at
form heads); and `%ignore` holds the error position back, so `(y";"{?` reports
the `?` rather than the `{`.

**And the phase's closing check found the failure mode a parity harness must
not have.** The per-commit tier reported 438 cells, 0 differences, in 16.1 s —
because both sides had exited 1 with the same `ModuleNotFoundError` and the
harness called it perfect agreement. It gained a **liveness check**: a side
that never exits 0 across the positive group did not run, and the run exits 2
saying so.

#### S1a.1.1 — Lexer, parser, error messages
`lex.rs`, `parse.rs`, `ast.rs`; no allocation per token; all four
`examples/broken/*.ein` messages byte-identical, EOF `-1:-1` quirk included.
The AST arena moved here from S1a.1.2 — the parser has to build something.

#### S1a.1.2 — AST arena, compatibility renderers, dumper
`pyrepr`, `pyfmt` and the dumper: 40 value shapes and a 1 700-code-point sweep
for `repr`, 230 values × 19 specs for float formatting, 0 differences. `repr`
needs *CPython's* Unicode printability table, not a Rust equivalent. The dumper
was byte-identical on the first run.

#### S1a.1.3 — Macro expansion and import resolution
`imports.rs` and `macros.rs`: 91 files identical under `resolve`, `minimize`
and `expand`, the eleven import-error fixtures compared against the files
themselves. The loader-side macro checks stayed with the loader.

### P1a.2 — KB core

**4 stages, shipped 2026-08-18.** The data model, and the phase where I2 is
cashed in: interning, `Value`/`FactId` as integers, the fact row store, the
seven indexes, the layered copy-on-write KB, provenance and the loader
([design/03](design/03_data_model.md)). **95 corpus files at KB-shape parity, 0
differences**, every load error byte-identical, and `fork()` **O(1)** — the
same `(allocations, bytes)` at 10 facts and at 10 000, asserted with a counting
allocator.

| bench | ein.py | ein.rs | |
|---|---:|---:|---|
| `load` zebra2 | 625.6 ms | **1.03 ms** | 607× |
| `fork` + first delta write | 17.3 µs | **248 ns** | 70× |
| peak RSS, `load(zebra2)` | 46.6 MB | **3.1 MB** | 15× |

The stages landed **2.1 → 2.2 → 2.4 → 2.3**, because the loader builds a
`Provenance` for every fact and calls `detect_provenance_cycles` itself:
provenance is the loader's dependency, not its sequel.

Two scope decisions outlived the phase. The **`Prov` arena is global**, beside
the fact store, where design/03 §5 sketches it inside `KbCore` — the same trade
interning makes — at the cost that a dead fork's records are not reclaimed
until the run ends, which is exactly what
[T1a.7.1.7](#s1a71--making-the-shared-state-sync) came back for with a number.
And a KB has no CLI surface, so the phase built one: `ein_core::shape` renders
registries, indexes and participation counts as one deterministic text, which
is what the parity diff could compare.

#### S1a.2.1 — Interner, `Value`, `FactId`, the fact store
Every atom, literal and proposition becomes a `u32` (design/03 §§2–4).

#### S1a.2.2 — The KB, its seven indexes, and the layered fork
The layered COW KB, `Kb::check_layering` asserting that a flattened fork equals
a materialised copy (design/03 §§5–6, 8).

#### S1a.2.3 — The loader (`from_ir`)
Validation, registries and load errors: all 18 loader fixtures byte-identical,
plus five accumulation programs pinning the cross-pass order — macro →
relation → rule → fact → config.

#### S1a.2.4 — Provenance and derivation walks
Provenance, alternatives and the derivation walks; `ein-render` opened early
with the derivation DAG, because a provenance walk whose output nobody can read
is a walk nobody can check.

### P1a.3 — Deductive core

**4 stages, shipped 2026-08-18.** The engine proper: the pattern compiler, the
register matcher, the two-phase closure/boundary saturator, the `World` NAF
boundary and contradiction detection
([design/05](design/05_matcher.md), [design/06](design/06_saturation.md)).
**T2 event-trace parity: 64 files, 23 848 events, 0 differences** — every
firing, in order, with its provenance; 231 plans at plan-shape parity; 1 879
matches; and **0 allocations** in the matcher's inner loop under a counting
allocator.

| bench | ein.py | ein.rs | |
|---|---:|---:|---|
| `saturate_root` zebra2 | 90 ms | **2.89 ms** | 31× |
| `match_hot` (every plan over the saturated root) | 2 110 µs | **38.6 µs** | 55× |

**Two design claims did not survive contact, and both are recorded rather than
worked around.** The `Probe` cannot be fully static (design/05 §2 says it can):
two of `_candidates`' three conditions are dynamic, so the compile-time win is
*narrowing the scan*, not removing it. And **Win B's ≥ 80 % target assumed
monotone guards dominate** — they are 11 % of guard evaluations on `zebra2` and
30 % on `zebra`, for a structural reason: a candidate that is still parked has
a guard that *failed*, and a failing monotone guard is retired on the spot, so
every re-judged candidate's failing guard is non-monotone. That became
[Q-M1a.17](open_questions.md), and P1a.6 closed it by declining the mechanism
at a measured ceiling.

The phase's own risk register read *"order drift is invisible until it is
expensive"*, and the T2 diff earned its place on the first working saturation:
it reported exactly one difference (`n_guards` counting disjuncts rather than
guarded premises) against 23 000 events that already matched.

#### S1a.3.1 — The pattern compiler
Plan bytecode, and **Win A**: compile once per distinct `(rule, activator)`
pair, process-wide. `zebra2` compiles 19 plans in 21.8 µs.

#### S1a.3.2 — The register matcher
Registers, the backtrack trail, candidate probes and entry points. The register
file is one space per plan rather than per disjunct, which is what lets
`:assert` templates compile once. [D1](divergences.md#d1--a-rule-may-not-bind-more-than-256-variables)
— a rule may not bind more than 256 variables — is this stage's ceiling.

#### S1a.3.3 — The saturator
The closure loop, the semi-naive delta and the queues. **The boundary landed
here, not in S1a.3.4**: `Saturator::step` does not terminate without an
`admit_from_boundary`.

#### S1a.3.4 — The NAF boundary
Negative provenance, clash detection, and the delta seeding that
[P1a.7](#p1a7--parallelism) later found had already removed the work level 2
was specified to spread over cores — "91 % of matcher output was re-discovery a
full re-match would recompute".

### P1a.4 — Search layer

**6 stages, shipped 2026-08-18.** Everything above the fixpoint: hypothesis
generation, one-step lookahead, the Apriori commitment lattice, no-good
learning, the commitment primitive, and the three-phase `solve` loop with its
verdict synthesis ([design/07](design/07_search_layer.md)). **T1 corpus-wide in
three regimes** — `fast` (5 174 enterings), `exhaustive` (1 618), `shuffled`
(5 207) — 65 files, 0 differences in each; T2 on the branching, lattice and
domain-elimination fixtures; and the three acceptance fixtures in **0.87 s**
against minutes under PyPy, so they run with the ordinary suite instead of as a
separate gate.

`solve zebra2 -e` is **194 ms against 5.00 s** (26×) for the search, and ~195 ms
end-to-end against PyPy's 4.07 s — **21×, the milestone's ≥ 20× target, met at
parity time and before any optimisation phase.**

**They did not ship in plan order** (1 → 2 → 3 → 6 → 4 → 5): S1a.4.4's
`try_commitment_set` needs `smallest_contradiction_frontier`, which is
S1a.4.6's, whose own dependency was declared on stages that depend on it. The
cycle was only in the *acceptances*.

Two findings are worth more than the parity result. The re-measured lever
matrix showed **ein.rs answering a cell ein.py could only time out on** —
`enable_singleton_writeback` off finishes at 3 831 enterings in 11.3 s where the
Python matrix aborted at "3 336+, still climbing" — and `enable_fail_fast_fork`
got *more* valuable as the engine got faster (1.9× → 3.0×), because what it
avoids is saturation and saturation is now 31× cheaper relative to everything
around it. And **mutation testing found four paths the corpus could not
reach**: two got fixtures (`branching/13_lookahead_naf_world.ein` and
`14_lookahead_unjudgeable.ein`, both of which flip *Solution → Contradiction*
when their guard is removed), one got an instrument line, one is recorded.

#### S1a.4.1 — Hypothesis generation
The candidate enumerator and the filter pipeline, with the name of the filter
that dropped each candidate carried into the event stream.

#### S1a.4.2 — Lookahead, closure marking, NAF dependency map
The one-step lookahead — which accounts for 547 of the corpus's 4 479 raw
candidates — and the static stratification proxy.

#### S1a.4.3 — Apriori candidate generation and the no-good store
The prefix join and the no-good store. The join's `break` is a cost win and
*only* a cost win: replacing it with a `continue` is byte-identical on all 65
files.

#### S1a.4.4 — The commitment primitive
`try_commitment_set` — fork, write the commitment's hypothesis facts, saturate,
judge — the primitive [S1a.6.9](#s1a69--the-fork-entry-delta-the-resumed-saturator)
later found was re-deriving root's fixpoint once per entering.

#### S1a.4.5 — The solve loop and verdict synthesis
The three-phase loop and the verdict, plus `--shuffle` reproducing CPython's
`random.shuffle` exactly ([Q-M1a.5](open_questions.md#q-m1a5--reproducing-cpythons-shuffle)
— MT19937 ported, checked by table and on every corpus entry).

#### S1a.4.6 — Explanation and unsat cores
The three searches over the AND/OR graph. The environment representation
*changed* — a sorted rank vector rather than a `frozenset` — and the risk was
managed the way it asked: shape-for-shape loop structure, and the diff run on
every corpus entry rather than the zebra ones.

### P1a.5 — Presentation and CLI

**4 stages, shipped 2026-08-18 — and this is where the byte gate closed.**
Every byte ein.py wrote, ein.rs wrote: the solution table, `--stats`,
`--timing`, `--print-final-*`, the markdown trace with its inline DOT, all four
`render` subcommands, `saturate`'s output and `--dump`, and the `--dump-states`
tree. **T3 over the whole corpus × run matrix: 472 of 473 cells identical**,
the one exception [D2](divergences.md), accepted with a fixture since P1a.4.
**Milestone invariant I1 is discharged here.**

**Two exceptions were decided rather than met**
([Q-M1a.13](open_questions.md#q-m1a13--argparse-surface-parity)): the `--help`
layout and the usage-error *text* are normalised, because ein.rs uses `clap`
and `clap` cannot be configured into `argparse`'s formatter. Their **content**
is compared by a structural diff that is stricter than the byte one on the
property the byte one was guarding — no subcommand, option, short key,
metavar, arity, default, `choices` value or exclusive group may differ, at all
8 parsers, and the extractor is shown to find them before the diff is believed.

#### S1a.5.1 — DOT renderers
All four `render` subcommands, plus `render_why` brought forward from S1a.5.2
because `render/slice` labels every rule node with a rendered `:why`.

#### S1a.5.2 — Trace and answer rendering
The markdown trace with its inline DOT, and the answer table.

#### S1a.5.3 — State dumps
`--dump-states`, with one ein.py bug fixed first (`LatticeDumper` had no
`root_saturating`) and one acceptance item that turned out to describe code
that does not exist.

#### S1a.5.4 — The CLI
`clap` against `argparse`, content-parity. Three counts in the stage document
were wrong and the structural check that replaced the byte diff is what found
them.

### P1a.6 — Performance

**12 stages, shipped 2026-08-20. All four targets met with room** — the
tightest by 8.4×, and with **88 % of headroom** after the last stage where it
had 0.7 % after the first.

| workload | PyPy | target | at the close |
|---|---:|---:|---:|
| `solve zebra2.ein -e` end-to-end | 4.53 s | ≤ 0.20 s | **28.9 ms — 157×** |
| `solve zebra.ein -e` end-to-end | 8.33 s | ≤ 0.40 s | **47.5 ms — 175×** |
| parse + load `zebra2.ein` | 0.43 s | ≤ 0.015 s | **0.67 ms — 642×** |
| the acceptance gate (3 fixtures) | 36.0 s | ≤ 5 s | **0.127 s** |

**The method was fixed and non-negotiable: profile, change one thing, re-diff,
re-measure, record. A change that cannot be attributed is reverted, and a wash
is a revert.** Everything after the first stage was *chosen* by the profile
rather than by the plan: two stages were added, one shortened, one un-gated,
and the run order became the profile's. The phase re-measured before choosing
each next stage, on the finding that the profile did not look like the one the
phase had been planned against — and had no reason to hold still after two
stages either.

**Rule 1 was amended mid-phase**, and it is the milestone's largest single
decision: byte-identical *narration* stopped being the gate at
[S1a.6.9](#s1a69--the-fork-entry-delta-the-resumed-saturator). What stayed hard
is that the **answer** — verdict, `k`, models, query bindings, unsat core and
every counter — is identical.

#### S1a.6.1 — Fresh profile and bench baseline
**Shipped 2026-08-18.** The stage that re-planned the phase, and the reflex the
milestone then used twice more ([S1a.7.0](#s1a70--the-speculation-audit),
[S1a.9.0](#s1a90--the-slow-corpus-re-priced)): measure the premise before
spending four days on it. It also re-measured the PyPy column the targets are
ratios against — two of the four numbers moved.
[baseline.md](measurements/baseline.md) §1–§9 is its output.

#### S1a.6.8 — The compile cache and the extent counts
**Shipped 2026-08-18** — −30.5 % / −7.8 %. design/06's Win A finally built as a
per-*run* memo: `plan_compile` 17 430 → **305**, `ein_infer::compile` 21.1 % →
2.4 % cumulative, and half the run's allocations went with it.

#### S1a.6.9 — The fork-entry delta (the resumed saturator)
**Shipped 2026-08-19 — the last unmet target, and the first place where
matching ein.py byte for byte and building the better engine pulled apart.**
95.0 % of `zebra -e` was inside `try_commitment_set`, and **94.6 % of what a
fork did there was re-deriving root's fixpoint**, once per entering. Resuming
it instead cut fork firings by 74–77 % and fork compiles to zero:
`solve zebra -e` 539.9 → **397.2 ms** against a ≤ 400 ms target. Verified over
3 228 853 enterings compared fact by fact: verdict, `k`, models, printed unsat
core, entering counts and **all 85 `summary.json` fields** identical. What is
not identical is the narration, and **the primary justification of 267 529
facts**, which pick a different equally valid derivation because a resumed fork
inherits root's parked candidates with root's tiebreakers.
[Q-M1a.18](open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
decided it, [D3](divergences.md) records it, and
[fork_delta_trace.md](measurements/fork_delta_trace.md) is the before/after
trace the decision was taken against. `--trace` gained a *Before any
assumption* section, without which the solution's proof silently lost every
rule that fires only at root.

#### S1a.6.10 — The parity contract relaxes: answers, not narration
**Shipped 2026-08-19.** One rule in `ein-parity` replaced six ad-hoc cuts, with
`--strict` putting them all back; the cut itself was chosen by running six
candidates over the captured logs. **T3 472/473 and T2 239/240, D2 the only
cell in either** — T2 had been 142/240 the day before.

#### S1a.6.11 — ein.rs's own fixtures for what parity stopped comparing
**Shipped 2026-08-19.** Twelve goldens over real solves — the trace, a `slice`
cone, a fork's own dump, the snapshot, the event stream — because a narration
the harness no longer compares needs a checked-in owner. Idea-08's walkthrough
assertion was ported to ein.rs and un-gated.

#### S1a.6.2 — Memory layout
**Shipped 2026-08-19** — −23.5 % / −12.1 %, on **two** of eight tasks: the
`snmalloc` global allocator (chosen by measuring three, at +1.2 MB peak RSS
where `mimalloc` cost +7.2 MB) and a *bigger* 20-byte row with two arguments
inline. **Five tasks were closed by measurement rather than by code**, and one
was built and reverted at +7.6 % — a flat extent index is 8 % faster on
`match_hot` and 7.6 % slower on the search, because a fork shares its parent's
index behind an `Arc`.

#### S1a.6.3 — Beta-memories (F11 D1)
**Shipped 2026-08-19 without the memory** — and it is the phase's largest
single win. The alpha index keyed only non-nested arguments, so a
`(not (R ?b ?i))` premise walked a 368-fact extent; the key now reaches one
level *inside* a nested argument. **Candidates 25 160 149 → 1 171 385**,
`zebra -e` **349 → 78 ms**, with every counter that measures a *decision*
identical to the digit. Then a per-layer Bloom filter, −7.3 %. The gate the
stage carried said no to the memory itself, and
[F11](../../../plans/followups/f11_deductive_layer_perf.md) D1 was re-priced.

#### S1a.6.4 — Hypgen and lattice hot paths
**Shipped 2026-08-19, aimed elsewhere by its own measurement.** A hypgen call
offers **125** raw candidates, not design/07's 18 k — that figure was ein.py's
*blind* arm, and the milestone's workloads are all hrule-driven — while
`complete()` spent **71 %** of itself on setup, 219 compile-cache keys per
call. The deeper finding is that **no milestone target runs the blind
enumerator at all**, and the corpus's slowest solves are the ones that do
(`features/05_stdlib_domain_elim -e` is 46× `solve zebra -e`); two new tasks
took 15 % off those. Three planned tasks closed against numbers instead of
code.

#### S1a.6.5 — Frontend and load path
**Shipped 2026-08-19** — a one-day confirmation of a path already 8× inside its
acceptance, which found a load parsing **3.30× the bytes on disk**: import
resolution parsed a module once per *edge* of a diamond. One cache per
resolution, a byte-wise `skip_trivia`, and `FxHashMap` for a 157-entry interner
index: `load/zebra2` **−25.5 %**, and 0.23 ms off *every* invocation. Two of
six tasks proposed pre-sizing and both lost.

#### S1a.6.12 — The NAF boundary and the per-entering snapshot
**Shipped 2026-08-20** — the stage the profile had named since S1a.6.3, and
**76.7 → 47.5 ms** on `zebra -e`. Two tasks went where the plan predicted (one
epoch per guard set: 494 566 → 12 864 extent probes; the fork no longer
deep-copying the candidate arena). Two did not: the *index* the stage was named
for was **built twice and reverted twice**, because a round stops at its first
admission and the cost was copying the parked set rather than walking it — and
the instrument meant only to check whether guards reach S1a.6.3's index found
**71.8 % of guard premises have every slot bound**, which makes them a hash
lookup rather than a ten-fact scan: candidates **1 172 870 → 238 567**.
[Q-M1a.17](open_questions.md) closed here.

#### S1a.6.6 — The differential fuzzer
**Shipped 2026-08-20** — ~700 cases/min, 86 % of them loading, and **four
genuine parity bugs in the first twenty minutes**, on a surface five phases of
byte parity had signed off: an integer goal binding written as a string, a
nested-fact one that made ein.py's `json.dumps` raise and write no summary, a
`KeyError` printed without its class, and a query goal ein.rs would not reject.
Plus D2's second shape and two D3 reaches, recorded rather than fixed. Its own
three controls each failed once before they held.

#### S1a.6.7 — Re-measure the lever matrix
**Shipped 2026-08-20** — both engines, same day, same harness, and a **`control`
row** that prices each column: 1.2× under PyPy against 1.0× under ein.rs, which
**retired four of the old table's ten conclusions**. It also found that
`enable_pre_branch_lookahead` is **not a prune** — `complete()` asks the
hypothesis generator and the generator's candidates are lookahead-filtered, so
turning it off turns `Ambiguity` into `Contradiction` in *both* engines, which
is [F4 Q40](../../../plans/followups/f4_cross_cutting.md).

### P1a.7 — Parallelism

**6 stages — four shipped, two declined by measurement. Closed 2026-08-23 at
3.17–4.40× on 8 P-cores against a ≥ 6× target.** `--jobs N` is a `rayon` pool
built once per solve and only when `jobs > 1` (a default run creates no thread
at all), a bounded batch, and an **ordered commit**: a worker builds its
narration into its own buffer with a hole where the event ordinal goes, and the
replay fills it in at the commit. That is what makes the verbose event stream
byte-identical at both job counts, `branching/06 -e`'s 2 200 561 lines included.

**The shortfall is measured rather than papered over.** The serial terms are
8–17 %, so Amdahl would allow 7.5×; what does not deliver it is the fan-out's
own ~5× on 8 cores, which the profile puts on **memory rather than contention**
— no lock in it, 11 % allocator. That makes the remaining 1.5× a question about
what a fork allocates, which is P1a.6-shaped work and not more threads.

**The phase's pattern is that a parallel run is an instrument that finds
sequential waste**, because it is the one place a serial millisecond cannot
hide: the first fan-out was 2.19–2.89×, and four things the *measurement* found
closed the rest — 192 of 269 ms of a commit loop that was freeing memory
another thread allocated, a downward-closure filter that had not been fanned
out, a candidate list cloned instead of ordered in place, and `record_node`
promoting a fork's provenance *before* asking whether the node was a duplicate
(1 221 calls to keep 22 nodes). Three of the four make `--jobs 1` faster too.

The phase **paused for two days** while P1a.8–P1a.10 ran, and P1a.10 retired
the harness four of its five remaining stages had written their acceptance in.
The restatement came first on resumption, and in one place it promises *more*:
"T3-identical" became "identical except where a permuted id space already
moves, and no wider" — a criterion a reviewer can check rather than a diff a
reviewer can only read — at 2 000× less cost.

#### S1a.7.0 — The speculation audit
**Shipped 2026-08-20**, a stage the plan did not have, because the phase's
central risk was measurable before any of it was built. **1 078 704 enterings
speculated against layer-start root.** The control held (0 differences on
1 078 154 case-1 speculations, which is the corpus-scale form of the purity
property level 1 rests on); **design/08 §2's claim that "case 1 is the whole of
layer 1" is inverted** — layer 1 is the only layer that can write a fact to
root mid-layer, so every layer ≥ 2 needs no validator at all; the re-validation
rate is 0.1 % corpus-wide but **36–50 % on the zebra family**, so the "≤ a few
percent" acceptance would have passed on the average while being wrong on every
recognisable workload; and on **35** enterings the speculation is *wrong*
rather than stale, which is the evidence that no read-set filter would have
been sound.

#### S1a.7.1 — Making the shared state `Sync`
**Closed 2026-08-22, and it lost three of its eight tasks to its own
measurement** ([shared_state.md](measurements/shared_state.md)): design/08 §6
specified three *write* strategies and nobody had taken the write rate. A
search assigns **41 to 417 fact ids** against 5.8–26 M reads, and per
*entering* — which is what a worker runs — four of six workloads have **zero**.
So no lock was built, `intern` stays on the committing thread, and the type
system is the enforcement. What the measurement found instead is the structure
design/08 §6 has **no row for**: the **provenance arena**, written by 100 % of
enterings, 2 135 093 records and 205 MB on `features/01 -e` — and *nothing
references one of them when the solve ends*. It became per-worker with
promotion only on the solution path, which took that file from **684–708 MB to
85–91 MB at `--jobs 1`** and is where the phase's "memory scales with jobs"
risk actually lived.

#### S1a.7.2 — Level 1: parallel enterings
**Closed 2026-08-23.** Its layer-1 question was decided on paper and **the
decision deleted more than it built**: one more pass over the event stream
found **248 of 248 writebacks are in layer 1**, across 8 158 205 enterings, and
layer 1 is 0.016 % of them. The rule is therefore *a layer is fanned out iff it
cannot write a fact to root*, which deletes design/08 §2's validator entire,
along with the fail-fast interaction, the case-3 fixture and the read-set
refinement. Its first task — flattening root's layer stack at the barrier — is
worth **3.17× at `--jobs 1`** on `branching/07 -e`, and took that entry off the
corpus's `slow` list. Then the threads: **3.16–4.30× on 8 P-cores**, the same
computation on all 47 corpus entries, and an early stop whose batch now ramps
with the commits rather than sitting flat (**1.69 → 3.13×** on the CLI's
*default* `-n 1` run).

#### S1a.7.3 — Level 3: the parallel boundary round
**Declined 2026-08-23**, premise measured before the build: three of the four
measurement-set workloads never park a candidate, the fourth judges a median of
one per round, and a round is 0.18 µs against a ~10 µs barrier
([scaling.md §9](measurements/scaling.md#9-levels-2-and-3-measured-before-they-are-built)).
Nothing was built, gated off or left half-present.

#### S1a.7.4 — Level 2: the parallel enqueue pass
**Declined 2026-08-23** — right about the share (10.6–31.2 %, more than level
3's on every cell) and wrong about the width: **1.4–3.1 tasks per pass** against
a fan-out of 8, and a pass is 0.26 µs. What would re-open either is a workload,
not an argument, and re-taking the tables is a morning's work.

#### S1a.7.5 — The `--jobs` contract
**Closed 2026-08-23.** `jobs_invariance` — 20 712 (file, op, jobs) cells over
128 files × 45 ops at `--jobs {2,4,8,16}`, **0 moved, in 30 s** against the
retired harness's 738 — plus `--jobs auto`, the ruling that a job count stays
**out of `SolverConfig`** (a puzzle file must not set a thread count),
`Terms::lend` so a worker panic cannot leave the tables lent, and the failure-
mode rulings. `--unordered` was declined here at 0 %.

### P1a.8 — Binary KB container

**1 stage, shipped 2026-08-21.** `.einb` — a loaded, optionally saturated KB as
a file the engine can `mmap` back with no parse and no load
([design/10](design/10_binary_format.md)). A saturated `zebra2` is **57 688
bytes** and opens cold in **0.614 ms**; `ein solve x.einb` is byte-identical to
`ein solve x.ein` across four puzzles and five diagnostic flags, with two lines
normalised — the path `solve` echoes, and `--stats`'s wall clock.

`ein-einb` is the eighth workspace member and **the only crate that is not
`#![forbid(unsafe_code)]`**: design/12 §2 permits `unsafe` in exactly one
audited module, `forbid` cannot be lifted per module, and the crate boundary is
what makes "exactly one" a fact rather than a promise.

**Two open design questions were answered by measurement**: `PROGRAM` is
canonical *text* rather than the AST arenas, because a resolved `zebra2`'s
arenas are past 60 KB against a 64 KB budget while `dump_canonical` of the same
forms is 11 KB; and there is **no `INDEXES` section**, because
`rebuild_indexes` *is* the projection that defines them. That second decision
found the stage's one real bug — `rules_by_relation` is **not** a projection,
so rebuilding it from a saturated fact set produces a larger map than the KB
ever had. The bit-flip sweep found the other: 20 000 fuzzed inputs and 3 348
single-bit flips rejected by the digest, and the flips showed the header is not
under the digest, so its reserved words are now required to be zero.

`.einb` is a **private, versioned cache format, never an interchange one** —
and a `SOLUTIONS` section ships with a library API and *no* CLI producer, which
is [F9](../../../plans/followups/f9_e_catalog.md)'s measurement hazard handled
structurally: a stored answer memoises the puzzle rather than improving the
reasoner, so there is no way for a run to open one.

#### S1a.8.1 — The `.einb` container
The header, sections, tables, the id remap and `ein kb save`.
[Q-M1a.22](open_questions.md#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate)
— whether the remap is order-preserving enough for its own gate — is answered
(a): byte-identity on the fast path, answer-identity always.

### P1a.9 — Release

**3 stages, shipped 2026-08-23 — and all three found something the plan had
assumed.** S1a.9.0 found twelve of seventeen `slow` corpus entries were never
slow; S1a.9.3 found two of three feature flags gating nothing and one that does
not exist; S1a.9.4 found two published measurements that were wrong rather than
stale. That is not a run of luck — it is what happens when a release phase is
made to **re-measure the claims it is about to ship** instead of collecting
them.

**The phase was re-topiced on 2026-08-21**, from *Bindings and release*: the
two PyO3 stages (S1a.9.1, S1a.9.2) were cut and their numbers left as a
deliberate gap, so a link that meant "the PyO3 stage" cannot silently come to
mean something else. Both files are in git history. What replaced them is
S1a.9.0, which was never in the plan and is the milestone's own reflex.

#### S1a.9.0 — The slow corpus, re-priced
**Shipped 2026-08-22.** The corpus's seventeen `slow = true` entries were
priced against CPython — the exclusion notes said so in words — and re-measuring
against the engine being released found **twelve were never slow by any
threshold worth stating**, the flagship `zebra2.ein` among them at 16 ms. What
the release gate sweeps is **641 cells in 19.4 s where it was 660 in 242.6 s**.
The measurement is [corpus_cost.md](measurements/corpus_cost.md), `slow = true`
is now a claim with a `cost_ms` behind it, and the stage found something it did
not fix: **ten entries where `solve` costs exactly what `solve -e` costs**,
because all ten return `Contradiction` / `exhausted = false` at
`layers == max_set_size` — the search is running out of commitment depth, not
proving unsat. That went to
[M1d Q-M1d.6](../../../docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false).

#### S1a.9.3 — Packaging and release
**Shipped 2026-08-23.** `ein --version` — engine semver, event protocol,
container format, the features compiled in, and SHA-256 of the stdlib manifest
**as resolved**, naming which of the three resolution steps answered;
`.gitattributes`, without which the first Windows corpus sweep would have been
a wall of red about CRLF rather than about Windows;
[`docs/install.md`](../../install.md); and a six-job release workflow whose
`publish` `needs:` the gate, the `--jobs` cross-diff and the dependency-light
build. Its `--no-default-features` task was written as a compile check and run
as a **measurement**, which found that two of the three features that build was
documented to drop **were not being dropped**
([feature_cost.md](measurements/feature_cost.md)). **What no machine here has
run is the build matrix**: this repository has never been built for aarch64,
macOS, Windows or musl, and the first `v*` tag runs all four for the first
time. The stage says so rather than letting a badge say otherwise.

#### S1a.9.4 — Documentation
**Shipped 2026-08-23, and closed the milestone.** `docs/api/` got a subject by
changing what it is about: [`rust.md`](../../api/rust.md) is the crates — the
surface the GUI binds against and nothing documented — and its worked example
is **the marked region of `ein-cli/tests/embedding.rs`**, compared text-to-text
by a test in that file. That mechanism is the stage's whole argument: the five
Python pages were a *good* contract, were verified against a named commit, and
still went stale, because verification with a date on it is a claim about the
past. They are kept whole with a history banner; nothing in them is edited to
match ein.rs, because a page rewritten that way is neither history nor a
specification. The stage's re-take of `features.md`'s ein.rs half then found
**two of twelve `zebra2` cells did not reproduce** — the table was *wrong*
rather than stale, and a recorded conclusion resting on one of them is amended.

### P1a.10 — One implementation

**6 stages, shipped 2026-08-21.** `ein.py/` is gone — 183 files, tagged
`two-implementations` at the parent commit — along with the differential
harness, the PyPy and venv tooling, and two submodules that made every
recursive clone fetch large upstream repositories for work that has not
started. `cargo test --workspace` is the whole gate, and **coverage did not
drop**.

**This phase reversed two standing decisions on purpose**, and the argument is
the one worth keeping: the case for the oracle was never that a second
implementation is valuable in itself — it was that a rewrite with a byte-exact
oracle is a *measurable* rewrite, and that argument expires at the end of
P1a.5. So the question is not "was the oracle worth it" (it was) but **"what
does it still prove that nothing else does, and can that be banked?"** —
which is why S1a.10.1 is a gate rather than an inventory: nothing was deleted
until every claim the harness carried had a checked-in owner. The cost is
falsifiability, permanently, and the phase's own risk register says so.

#### S1a.10.1 — Bank what only the oracle proves
**Shipped 2026-08-20.** The stage was written as an inventory of the harness
and found that **the largest differential surface in the repo was
`cargo test --workspace` itself**: 42 of its 91 integration tests started a
Python process, and skipped invisibly when one would not start — the phase's
own acceptance criterion quietly not being one. The
[ledger](oracle_ledger.md) banks the rest in three instruments: 4 228
renderings as digests, 13 counter identities, and a determinism sweep that
permutes the **id space** instead of a hash seed and prices the whole
difference at **0 answers and 66 renderings**.

#### S1a.10.2 — Port the Python test suite
**Shipped 2026-08-20.** The Python suite's 1 538 tests reduce to **275
behaviours** in fifteen new Rust files, with the 96 dying subjects named in
[suite_dispositions.md](suite_dispositions.md); all 42 differential tests
became un-differential. `cargo test --workspace` became **566 tests in 1 m
07 s** where it was 312 in 9 m 13 s, and the experiment that had found 41
silent skips — `PATH` pointing at a `python3` that exits 127 — now leaves all
566 passing.

#### S1a.10.3 — The corpus without a second engine
**Shipped 2026-08-21.** The harness itself retired — `ein-conformance`,
`ein-oracle`, T0–T3, 2 164 lines — and the corpus it read became
[`corpus/`](../../../corpus/README.md) with a runner that is a **sweep** rather
than a diff: 542 cells as processes in 2.5 s, a banked exit table, and a
per-cell timeout so a non-terminating program fails the gate instead of hanging
it. The exit table is banked rather than ruled, because the corpus does not
obey a group rule — `render rules` never loads the KB, so 10 of 30
load-negatives exit 0.

#### S1a.10.4 — `utils/`, re-aimed at one engine
**Shipped 2026-08-21.** `utils/` went from 28 scripts to 17, all driving
ein.rs; the eleven that compared two engines or measured the Python one are
gone, each either banked by a checked-in test or superseded by a named
instrument, and the census of what answers their question now is in
[`utils/README.md`](../../../utils/README.md#the-census). Every CPython and
PyPy number in the measurement documents is labelled a **frozen constant**,
because its instrument is gone. The fuzzer kept its generator and lost its
differ — and **found three things in twenty minutes**: a `debug_assert!` an
ordinary `(hrule …)` reading `not` trips, an unsat core whose contents depend
on interning order, and the same for the goal-binding row the solve table
prints.

#### S1a.10.5 — The removal
**Shipped 2026-08-21.** 183 files. The Lark grammar became EBNF *first* — the
user's own precondition, promoted to the stage's first task and now
[§3 of `01_grammar.md`](../../kernel/ir/03-ein-lang/01_grammar.md) — and one
acceptance line was **amended on evidence**: "`nlp/` and `smt/` gone" is true
of the active tree and the wrong test, because every file in them has a named
dependent in a scheduled milestone. The two *submodules* are what actually cost
something, and they are what went.

#### S1a.10.6 — The docs after the oracle
**Shipped 2026-08-21, closing the phase.** The removal left 224 dangling links,
and the ratio is the finding: ~150 module pointers with a 1:1 counterpart and
~60 `.py`-named link texts were mechanical, 46 were **symbols ein.rs does not
have**, and ~20 were **claims that only made sense with two engines**. That
last 9 % is what the stage was for, and it would have been invisible without
walking all 224. Its substantive output is
[`docs/kernel/defined_behaviour.md`](../../kernel/defined_behaviour.md) —
**thirteen behaviours whose only statement was a Python source file**, now
normative — and enumerating them found two that are **bugs rather than
quirks**: the binding key that drops non-string activator args
([Q-M1a.8](open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)),
and the six Python exception classes the CLI still prints, which are now a name
without a referent.

---

## What outlived the milestone

- **Two questions that were open on purpose** — one of them still is.
  [Q-M1a.6](open_questions.md#q-m1a6--at-none-in-loader-messages) (a loader
  message that says `at None`) and
  [Q-M1a.8](open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)
  (**a bug** — as this milestone stated it, *a puzzle with integer rule
  parameters can lose a firing, silently*). Both are things I1 forbade fixing:
  they are what ein.py did, and while an oracle existed "improve it" and
  "diverge from it" were the same act. With one implementation that constraint
  is gone and only the cost of re-blessing the corpus goldens remains.

  **Q-M1a.8 closed 2026-08-29**, and not the way it expected: the trigger it
  names is not a trigger at all — an `int` activator argument binds its
  parameter and so reaches the key — while an `int` beside a nested `Fact` in
  one position does lose a derivation. The probe is M1e
  [S1e.1.4](../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)'s
  and the live half is
  [Q-M1e.16](../../../plans/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
  Nothing above is rewritten: it is what the milestone believed, and the
  ledger entry carries the correction.
- **A deferral with three trip-wires.**
  [Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  holds the conditions that would revive the PyO3 binding, and `docs/api/`'s
  five Python pages are kept whole so that reversing the deferral is cheap.
- **Three questions that left with their phases** on 2026-08-21, keeping their
  text: Q-M1a.19 and Q-M1a.20 became
  [Q-M1c.1](../m1c_external_validation/open_questions.md) and
  Q-M1c.2 with P1a.11; Q-M1a.21 became
  [Q-M1d.1](../../../docs/history/m1d_satisfiability/open_questions.md) with P1a.12.
  The M1a ids stay reserved and redirect.
- **Three decisions P1a.6 left rather than took**: `lattice_order =
  "score-sum"` (0.6× on `zebra`, 1.2× on `zebra2`, in both engines to the
  digit); whether a performance lever may decide what a complete model is
  ([F4 Q40](../../../plans/followups/f4_cross_cutting.md)); and whether the
  solve table should sort the binding row it prints — an **under-determined
  rendering**, which design/02 already forbids leaving open.
- **Five recorded fuzz findings**, each a semantics decision rather than a
  defect to patch, in
  [`corpus/fuzz_findings/`](../../../corpus/fuzz_findings/README.md) — where
  *empty* is the steady state, so a find that stays is one somebody has to
  decide.
- **A build matrix no machine has run.** The release workflow is written and
  reviewed; the first `v*` tag builds aarch64, macOS, Windows and musl for the
  first time.
- **1.5× of parallel speedup**, named: not a serial fraction but the fan-out's
  own ~5× on 8 cores, and on memory rather than contention.
- **The engine's only statement of intent is now `docs/kernel/`.** With one
  implementation, "what the engine does" is whatever ein.rs does — which makes
  that tree more load-bearing, not less, and is why
  [M1c](../m1c_external_validation/README.md) and
  [M10](../../../plans/m10_external_benchmarks/README.md) exist.

## Where the rest is

`plans/m1a_rust/` held eleven phase READMEs and 53 stage files. They were
deleted on 2026-08-23, after this record was written from them.

```sh
git log --diff-filter=D -- plans/m1a_rust        # the commit that removed them
git show <commit>^ --stat -- plans/m1a_rust      # what was in the tree
git show <commit>^:plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md
git log --oneline two-implementations            # the last commit with two engines
```

Also in git history and not in this record: `design/09` (server mode, deleted
2026-08-18 with the server), the seven server stages, and S1a.9.1 / S1a.9.2
(the PyO3 surface and its API-parity suite, cut 2026-08-21).

## Cross-links

- [`docs/kernel/`](../../kernel/README.md) — the specification ein.rs
  implements, and since S1a.10.6 the only statement of intent that is not also
  the implementation; [`defined_behaviour.md`](../../kernel/defined_behaviour.md)
  is what "whatever ein.py did" used to define
- [`docs/api/rust.md`](../../api/rust.md) — the embedding surface S1a.9.4
  wrote, whose worked example is a test
- [`corpus/`](../../../corpus/README.md) — what the conformance corpus became
- [`utils/README.md`](../../../utils/README.md#the-census) — the census: which
  instrument answers a retired script's question
- [F11 — deductive-layer perf](../../../plans/followups/f11_deductive_layer_perf.md)
  — D1 and D2, both re-priced and re-parked by P1a.6
- [F9 — the E-catalog](../../../plans/followups/f9_e_catalog.md) — the rejected
  search-layer optimisations; read before proposing one
- [M20 — GUI](../../../plans/m20_gui/README.md) ·
  [M2 — NL → IR](../../../plans/m2_nl_to_ir/README.md) — the consumers this
  engine ships for
- [M1c](../m1c_external_validation/README.md) ·
  [M1d](../../../docs/history/m1d_satisfiability/README.md) — created 2026-08-21 out
  of this milestone's last two phases
