# M1a — the Rust engine (ein.rs)

**Ran 2026-08-17 → 2026-08-23. Shipped.** Eleven phases, 53 stage documents,
and the engine every `ein` command runs.

> **This is history, not a plan.** M1a's plan tree — `plans/m1a_rust/`, its
> milestone README, eleven phase READMEs and 53 stage files, 65 files and
> 13 950 lines — was deleted on 2026-08-23,
> once the milestone had shipped and the only thing a stage file could still do
> was describe work that was already done. What is kept is what is still
> *read*: this record, the design contracts, the measurements and the
> questions. The stage files are in git
> history — `git log --diff-filter=D -- plans/m1a_rust` names the commit, and
> `git show <commit>^:plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md`
> reads one.

## What is in this directory

| file | what it is |
|---|---|
| [`design/`](design/README.md) | the ten design contracts — the *how* of the engine, and the documents `ein.rs` source comments cite as their specification. `design/README.md` § Measured is the per-phase measured record |
| [`measurements/`](measurements/) | [`baseline.md`](measurements/baseline.md) (P1a.6, 2 898 lines — the optimisation programme's every profile), [`scaling.md`](measurements/scaling.md) and [`shared_state.md`](measurements/shared_state.md) (P1a.7), [`corpus_cost.md`](measurements/corpus_cost.md) and [`feature_cost.md`](measurements/feature_cost.md) (P1a.9), [`fork_delta_trace.md`](measurements/fork_delta_trace.md) (S1a.6.9) |
| [`open_questions.md`](open_questions.md) | seventeen questions: fifteen settled, one deferred with three trip-wires, **one still open on purpose** |

---

## What shipped

`ein.rs` is the engine. Both of the milestone's invariants held the whole way:
**I1**, a fixed observable surface, amended once and in writing where
narration was the price of speed; **I2**, a free hand inside, cashed out as
interned integers, a register matcher, a layered copy-on-write KB and a
fanned-out lattice layer.

| | measured | where |
|---|---|---|
| `solve zebra2.ein -e` end-to-end | **29.0 ms**, peak RSS **17 MB** | [baseline.md](measurements/baseline.md) |
| `solve zebra.ein -e` end-to-end | **47.5 ms**; **585.8 → 47.5 ms** across P1a.6 | [baseline.md](measurements/baseline.md) |
| parse + load `zebra2.ein` | **0.67 ms** | [baseline.md](measurements/baseline.md) |
| the three acceptance fixtures | **0.127 s** | [baseline.md](measurements/baseline.md) |
| `--jobs 8` on 8 P-cores | **3.17–4.40×** on the measurement set (`branching/06 -e` 194.2 → 44.1 ms) against a **≥ 6×** target — not met, and named | [scaling.md](measurements/scaling.md) |
| `--jobs N` is the same computation | 20 712 (file, op, jobs) cells, **0 moved**, 30 s; the verbose event stream byte-identical at both job counts; 10 000 paired fuzz runs, zero findings | [scaling.md](measurements/scaling.md) |
| memory under parallelism | per-worker provenance: `features/01 -e` **684–708 → 85–91 MB**; peak RSS 79.8 / 90.3 MB at `--jobs 1 / 16` | [shared_state.md](measurements/shared_state.md) |
| `.einb`, the binary KB container | a saturated `zebra2` is **57 688 bytes**, opens cold in **0.614 ms**; `solve x.einb` byte-identical to `solve x.ein` | [P1a.8](#p1a8--binary-kb-container) |
| the gate | **619 tests, 0 failures, 1 m 51 s** (re-counted 2026-08-23) | [P1a.10](#p1a10--gate-corpus-and-specification) |
| what the gate banks | 4 228 renderings as digests, 13 counter identities over every solve cell, an id-space permutation sweep: **0 answers moved**, 66 renderings (all narration) | [S1a.10.1](#s1a101--digests-counter-identities-and-the-id-space-sweep) |

**The estimate was ~7 months and it took seven days.** 156 days of stage
estimates, run 2026-08-17 → 2026-08-23. That is worth recording
without a lesson attached to it: the plan's *shape* — surface first, speed
second, scale third — is what the work followed, and its duration is not.

### The two invariants, and how each ended

> **I1 — Outside, nothing moves.** The surface language, the CLI, stdout
> bytes, exit codes, DOT, the markdown trace, verdicts, counters and error
> messages are M1's. Any observable difference is a bug in ein.rs, not a
> design liberty.
>
> **I2 — Inside, everything is on the table.** Atoms and facts become integers,
> tuples become flat interned rows, the fork becomes a zero-copy layer, the
> matcher becomes a register machine, the search layer runs on many cores, and
> a loaded KB can be stored and mapped back from a binary file. None of that is
> allowed to leak through I1.

**I1 held through [P1a.5](#p1a5--presentation-and-cli)**, where the last of
the surface landed, and was then **amended once, in writing**. At
[S1a.6.9](#s1a69--the-fork-entry-delta-the-resumed-saturator) the hard
requirement became *the same answer* rather than *the same bytes*, because a
fork that resumes root's saturation narrates a quarter as much and that is
worth the last unmet performance target
([Q-M1a.18](open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)).

I1 is what made I2 safe. It is also why [P1a.0](#p1a0--corpus-and-shared-assets)
— the corpus, the event protocol and the determinism lint — came before a
single line of engine code.

---

## What was declined, and on what number

The record of what was *not* built is the milestone's most reusable output.
Every row was decided by a measurement, and every one of them is written down
where a reader who proposes it again will find it.

| declined | the number that declined it |
|---|---|
| **Server mode** (8 stages: daemon, sessions, JSON-RPC, streaming, a solution cache) | no consumer — the GUI links the crates, the CLI is the only other client. Dropped 2026-08-18; `design/09` and the seven stages are in git history |
| **The PyO3 binding** | every candidate consumer wants something else: the GUI links crates, the benchmark runner *must* shell out to be fair, M2's llama.cpp is an HTTP server. Deferred 2026-08-21 with three trip-wires ([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)) |
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

### P1a.0 — Corpus and shared assets

**4 stages, shipped 2026-08-17.** What the engine would be checked against,
built before there was an engine: the corpus manifest and the determinism
audit, the event protocol, the stdlib at the repo root, and the workspace with
its CI tiers and its determinism lint.

#### S1a.0.1 — Corpus manifest and determinism audit
The corpus manifest, and the determinism audit
([design/02](design/02_determinism_and_order.md)).

#### S1a.0.2 — The event protocol
`--events FILE` and `--events-level {normal,verbose}` on `solve` and
`saturate`, 17 event kinds, versioned `ein-events/1` — the schema that is now
[`docs/kernel/inference/events.md`](../../kernel/inference/events.md). It has
`hypskip` as its own kind, and a `hyp` verdict that is a *filter name* rather
than a boolean, so a counter difference locates itself.

#### S1a.0.3 — Shared stdlib and examples
The stdlib at repo-root [`stdlib/`](../../../stdlib/README.md) — seven
modules, a README and `MANIFEST.sha256` — with one resolution order
(`$EIN_STDLIB` → the checkout → the embedded copy) and a drift
check that was verified by corrupting a module and watching the test name the
file ([design/11](design/11_shared_assets.md)).

#### S1a.0.4 — Workspace skeleton and CI
The `ein.rs/` workspace, the CI tiers, the benches — and the **determinism
lint** (`utils/check_hashmap_iteration.py`, with `// determinism-ok: <reason>`
as its escape hatch), whose rule is stronger than "don't use `RandomState`":
`FxHashMap` is deterministic run to run, but its order is still an artefact of
hash values and insertion history, where an observable's order has to come
from the program — insertion order or an explicit sort.

### P1a.1 — IR frontend

**3 stages, shipped 2026-08-18.** Read `.ein` and write it back
— lexer, parser, AST, dumper, macro expander, import resolver
([design/04](design/04_ir_frontend.md)). Parse is **758 µs**; `zebra2` parse +
resolve + expand is **824 µs**.

**Two parser behaviours reach the output and are kept rather than
corrected**, which is what I1 means in practice: the lexer is not maximal
munch, so `(rulex …)` parses as a rule named `x` (eighteen literals behave
this way, and the parser therefore lexes on demand and backtracks at form
heads); and a pending whitespace-or-comment match holds the error position
back, so `(y";"{?` reports the `?` rather than the `{`.

#### S1a.1.1 — Lexer, parser, error messages
`lex.rs`, `parse.rs`, `ast.rs`; no allocation per token; all four
`examples/broken/*.ein` messages, the EOF `-1:-1` position included.
The AST arena moved here from S1a.1.2 — the parser has to build something.

#### S1a.1.2 — AST arena, value renderers, dumper
`repr`, `numfmt` and the dumper: 40 value shapes and a 1 700-code-point sweep
for `repr`, 230 values × 19 specs for float formatting. `repr` escapes by
Unicode general category, so it needs a generated printability table rather
than Rust's `is_control()`.

#### S1a.1.3 — Macro expansion and import resolution
`imports.rs` and `macros.rs`: `resolve`, `minimize` and `expand` over 91
files, and the eleven import-error fixtures checked against their expected
messages. The loader-side macro checks stayed with the loader.

### P1a.2 — KB core

**4 stages, shipped 2026-08-18.** The data model, and the phase where I2 is
cashed in: interning, `Value`/`FactId` as integers, the fact row store, the
seven indexes, the layered copy-on-write KB, provenance and the loader
([design/03](design/03_data_model.md)). `fork()` is **O(1)** — the same
`(allocations, bytes)` at 10 facts and at 10 000, asserted with a counting
allocator.

| bench | ein.rs |
|---|---:|
| `load` zebra2 | **1.03 ms** |
| `fork` + first delta write | **248 ns** |
| peak RSS, `load(zebra2)` | **3.1 MB** |

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
is what a digest can pin.

#### S1a.2.1 — Interner, `Value`, `FactId`, the fact store
Every atom, literal and proposition becomes a `u32` (design/03 §§2–4).

#### S1a.2.2 — The KB, its seven indexes, and the layered fork
The layered COW KB, `Kb::check_layering` asserting that a flattened fork equals
a materialised copy (design/03 §§5–6, 8).

#### S1a.2.3 — The loader (`from_ir`)
Validation, registries and load errors: all 18 loader fixtures,
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
([design/05](design/05_matcher.md), [design/06](design/06_saturation.md)),
with **0 allocations** in the matcher's inner loop under a counting
allocator.

| bench | ein.rs |
|---|---:|
| `saturate_root` zebra2 | **2.89 ms** |
| `match_hot` (every plan over the saturated root) | **38.6 µs** |

**Two design claims did not survive contact, and both are recorded rather than
worked around.** The `Probe` cannot be fully static (design/05 §2 says it can):
two of the candidate probe's three conditions are dynamic, so the compile-time
win is *narrowing the scan*, not removing it. And **Win B's ≥ 80 % target
assumed monotone guards dominate** — they are 11 % of guard evaluations on
`zebra2` and 30 % on `zebra`, for a structural reason: a candidate that is
still parked has a guard that *failed*, and a failing monotone guard is retired
on the spot, so every re-judged candidate's failing guard is non-monotone. That
became [Q-M1a.17](open_questions.md), and P1a.6 closed it by declining the
mechanism at a measured ceiling.

#### S1a.3.1 — The pattern compiler
Plan bytecode, and **Win A**: compile once per distinct `(rule, activator)`
pair, process-wide. `zebra2` compiles 19 plans in 21.8 µs.

#### S1a.3.2 — The register matcher
Registers, the backtrack trail, candidate probes and entry points. The register
file is one space per plan rather than per disjunct, which is what lets
`:assert` templates compile once. Its ceiling is this stage's: a rule may not
bind more than 256 variables.

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
verdict synthesis ([design/07](design/07_search_layer.md)). The three
acceptance fixtures solve in **0.87 s**, so they run with the ordinary suite
instead of as a separate gate.

`solve zebra2 -e` is **~195 ms** end-to-end — **inside the milestone's
≤ 0.20 s target before any optimisation phase.**

**They did not ship in plan order** (1 → 2 → 3 → 6 → 4 → 5): S1a.4.4's
`try_commitment_set` needs `smallest_contradiction_frontier`, which is
S1a.4.6's, whose own dependency was declared on stages that depend on it. The
cycle was only in the *acceptances*.

Two findings outlived the phase. The re-measured lever matrix has
`enable_singleton_writeback` off finishing at 3 831 enterings in 11.3 s, and
`enable_fail_fast_fork` worth **3.0×**. And **mutation testing found four
paths the corpus could not reach**: two got fixtures
(`branching/13_lookahead_naf_world.ein` and `14_lookahead_unjudgeable.ein`,
both of which flip *Solution → Contradiction* when their guard is removed),
one got an instrument line, one is recorded.

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
The three-phase loop and the verdict, plus `--shuffle`'s generator — MT19937
with `init_by_array` seeding and a downward Fisher–Yates
([`defined_behaviour.md` §3.1](../../kernel/defined_behaviour.md)), checked by
table.

#### S1a.4.6 — Explanation and unsat cores
The three searches over the AND/OR graph, with the environment as a sorted
rank vector.

### P1a.5 — Presentation and CLI

**4 stages, shipped 2026-08-18 — and this is where the surface closed.**
The solution table, `--stats`, `--timing`, `--print-final-*`, the markdown
trace with its inline DOT, all four `render` subcommands, `saturate`'s output
and `--dump`, and the `--dump-states` tree. **Milestone invariant I1 is
discharged here.**

**Two exceptions were decided rather than met**
([Q-M1a.13](open_questions.md#q-m1a13--help-and-usage-error-text)): the `--help`
layout and the usage-error *text* are `clap`'s. Their **content** is pinned by
a structural check — no subcommand, option, short key, metavar, arity, default,
`choices` value or exclusive group may change, at all 8 parsers, and the
extractor is shown to find them before the check is believed.

#### S1a.5.1 — DOT renderers
All four `render` subcommands, plus `render_why` brought forward from S1a.5.2
because `render/slice` labels every rule node with a rendered `:why`.

#### S1a.5.2 — Trace and answer rendering
The markdown trace with its inline DOT, and the answer table.

#### S1a.5.3 — State dumps
`--dump-states`, and one acceptance item that turned out to describe code
that does not exist.

#### S1a.5.4 — The CLI
The CLI on `clap`, its content pinned structurally. Three counts in the stage
document were wrong and the structural check is what found them.

### P1a.6 — Performance

**12 stages, shipped 2026-08-20. All four targets met with room** — the
tightest by 8.4×, and with **88 % of headroom** after the last stage where it
had 0.7 % after the first.

| workload | target | at the close |
|---|---:|---:|
| `solve zebra2.ein -e` end-to-end | ≤ 0.20 s | **28.9 ms** |
| `solve zebra.ein -e` end-to-end | ≤ 0.40 s | **47.5 ms** |
| parse + load `zebra2.ein` | ≤ 0.015 s | **0.67 ms** |
| the acceptance gate (3 fixtures) | ≤ 5 s | **0.127 s** |

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
spending four days on it. [baseline.md](measurements/baseline.md) §1–§9 is its
output.

#### S1a.6.8 — The compile cache and the extent counts
**Shipped 2026-08-18** — −30.5 % / −7.8 %. design/06's Win A finally built as a
per-*run* memo: `plan_compile` 17 430 → **305**, `ein_infer::compile` 21.1 % →
2.4 % cumulative, and half the run's allocations went with it.

#### S1a.6.9 — The fork-entry delta (the resumed saturator)
**Shipped 2026-08-19 — the last unmet target, and the first place where
byte-identical output and building the better engine pulled apart.**
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
decided it, and
[fork_delta_trace.md](measurements/fork_delta_trace.md) is the before/after
trace the decision was taken against. `--trace` gained a *Before any
assumption* section, without which the solution's proof silently lost every
rule that fires only at root.

#### S1a.6.10 — Answers, not narration
**Shipped 2026-08-19.** One rule in `ein-narration` — a fork's derivation, and
anything keyed on a dying fork's stopping point, is narration, not content —
replaced six ad-hoc cuts, with `EIN_NARRATION_STRICT=1` putting them all back;
the cut itself was chosen by running six candidates over the captured logs.

#### S1a.6.11 — Fixtures for the narration
**Shipped 2026-08-19.** Twelve goldens over real solves — the trace, a `slice`
cone, a fork's own dump, the snapshot, the event stream — because narration
that S1a.6.10 no longer holds to the same bytes needs a checked-in owner.
Idea-08's walkthrough assertion became an ein.rs test and was un-gated.

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
offers **125** raw candidates, not design/07's 18 k — that figure was the
*blind* arm's, and the milestone's workloads are all hrule-driven — while
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

#### S1a.6.6 — The fuzzer
**Shipped 2026-08-20** — ~700 cases/min, 86 % of them loading, and **three
genuine bugs in the first twenty minutes**, on a surface five phases had
signed off: an integer goal binding written as a string, a `KeyError` printed
without its class, and a query goal ein.rs would not reject. Its own three
controls each failed once before they held.

#### S1a.6.7 — Re-measure the lever matrix
**Shipped 2026-08-20** — with a **`control` row** that prices each column at
1.0×, which **retired four of the old table's ten conclusions**. It also found
that `enable_pre_branch_lookahead` is **not a prune** — `complete()` asks the
hypothesis generator and the generator's candidates are lookahead-filtered, so
turning it off turns `Ambiguity` into `Contradiction`, which is
[F4 Q40](../../../plans/followups/f4_cross_cutting.md).

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

The phase **paused for two days** while P1a.8–P1a.10 ran, and on resumption
its acceptance was restated first. In one place it promises *more*: a
`--jobs N` run is identical "except where a permuted id space already moves,
and no wider" — a criterion a reviewer can check rather than a diff a reviewer
can only read.

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
128 files × 45 ops at `--jobs {2,4,8,16}`, **0 moved, in 30 s** — plus
`--jobs auto`, the ruling that a job count stays
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
**Shipped 2026-08-22.** The corpus's seventeen `slow = true` entries had never
been priced against the engine being released, and measuring found **twelve
were never slow by any threshold worth stating**, the flagship `zebra2.ein`
among them at 16 ms. What
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
**Shipped 2026-08-23, and closed the milestone.** `docs/api/` got its subject:
[`rust.md`](../../api/rust.md) is the crates — the surface the GUI binds
against and nothing documented — and its worked example is **the marked region
of `ein-cli/tests/embedding.rs`**, compared text-to-text by a test in that
file. That mechanism is the stage's whole argument: verification with a date
on it is a claim about the past. The stage's re-take of `features.md` then
found **two of twelve `zebra2` cells did not reproduce** — the table was
*wrong* rather than stale, and a recorded conclusion resting on one of them is
amended.

### P1a.10 — Gate, corpus and specification

**6 stages, shipped 2026-08-21.** `cargo test --workspace` became the whole
gate, the corpus a sweep, and the behaviours the engine's output carried
without a written statement became
[`defined_behaviour.md`](../../kernel/defined_behaviour.md). Two submodules
that made every recursive clone fetch large upstream repositories for work
that has not started went with them.

#### S1a.10.1 — Digests, counter identities and the id-space sweep
**Shipped 2026-08-20.** Three instruments, each a checked-in test: 4 228
renderings banked as digests, 13 counter identities over every solve cell, and
a determinism sweep that permutes the **id space** and prices the whole
difference at **0 answers and 66 renderings**.

#### S1a.10.2 — The behaviour suite
**Shipped 2026-08-20.** **275 behaviours** in fifteen new Rust test files.
`cargo test --workspace` became **566 tests in 1 m 07 s**.

#### S1a.10.3 — The corpus as a sweep
**Shipped 2026-08-21.** The corpus became
[`corpus/`](../../../corpus/README.md) with a runner that is a **sweep**: 542
cells as processes in 2.5 s, a banked exit table, and a
per-cell timeout so a non-terminating program fails the gate instead of hanging
it. The exit table is banked rather than ruled, because the corpus does not
obey a group rule — `render rules` never loads the KB, so 10 of 30
load-negatives exit 0.

#### S1a.10.4 — `utils/` and the fuzzer
**Shipped 2026-08-21.** `utils/` settled at 17 scripts, each driving ein.rs
and naming its binary — `$EIN_BIN` or `--bin`. The fuzzer runs the binary
directly, and **found three things in twenty minutes**: a `debug_assert!` an
ordinary `(hrule …)` reading `not` trips, an unsat core whose contents depend
on interning order, and the same for the goal-binding row the solve table
prints.

#### S1a.10.5 — The grammar as EBNF
**Shipped 2026-08-21.** The grammar was written down as EBNF — the user's own
requirement, promoted to the stage's first task and now
[`00_ebnf.md`](../../kernel/ir/03-ein-lang/00_ebnf.md), which
[`01_grammar.md`](../../kernel/ir/03-ein-lang/01_grammar.md) names in its
first line as the source of truth for syntax — and one
acceptance line was **amended on evidence**: "`nlp/` and `smt/` gone" is true
of the active tree and the wrong test, because every file in them has a named
dependent in a scheduled milestone. The two *submodules* are what actually cost
something, and they are what went.

#### S1a.10.6 — Defined behaviour
**Shipped 2026-08-21, closing the phase.** Its substantive output is
[`docs/kernel/defined_behaviour.md`](../../kernel/defined_behaviour.md) —
**thirteen behaviours whose only statement was the code**, now
normative — and enumerating them found two that are **bugs rather than
quirks**: the binding key that drops non-string activator args
([Q-M1a.8](open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)),
and the six exception class names the CLI's crash diagnostics print, which
name no type the engine has.

---

## What outlived the milestone

- **Two questions that were open on purpose** — one of them still is.
  [Q-M1a.6](open_questions.md#q-m1a6--at-none-in-loader-messages) (a loader
  message that says `at None`) and
  [Q-M1a.8](open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)
  (**a bug** — as this milestone stated it, *a puzzle with integer rule
  parameters can lose a firing, silently*). Both are things I1 forbade fixing,
  and what is left of that constraint is the cost of re-blessing the corpus
  goldens.

  **Q-M1a.8 closed 2026-08-29**, and not the way it expected: the trigger it
  names is not a trigger at all — an `int` activator argument binds its
  parameter and so reaches the key — while an `int` beside a nested `Fact` in
  one position does lose a derivation. The probe is M1e
  [S1e.1.4](../m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3)'s
  and the live half is
  [Q-M1e.16](../m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
  Nothing above is rewritten: it is what the milestone believed, and the
  ledger entry carries the correction.
- **A deferral with three trip-wires.**
  [Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  holds the conditions that would revive the PyO3 binding.
- **Three questions that left with their phases** on 2026-08-21, keeping their
  text: Q-M1a.19 and Q-M1a.20 became
  [Q-M1c.1](../m1c_external_validation/open_questions.md) and
  Q-M1c.2 with P1a.11; Q-M1a.21 became
  [Q-M1d.1](../../../docs/history/m1d_satisfiability/open_questions.md) with P1a.12.
  The M1a ids stay reserved and redirect.
- **Three decisions P1a.6 left rather than took**: `lattice_order =
  "score-sum"` (0.6× on `zebra`, 1.2× on `zebra2`); whether a performance lever
  may decide what a complete model is
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
- **The engine's only statement of intent that is not also the implementation
  is `docs/kernel/`** — which makes that tree load-bearing, and is why
  [M1c](../m1c_external_validation/README.md) and
  [M10](../../../plans/m10_external_benchmarks/README.md) exist.

## Where the rest is

`plans/m1a_rust/` held eleven phase READMEs and 53 stage files. They were
deleted on 2026-08-23, after this record was written from them.

```sh
git log --diff-filter=D -- plans/m1a_rust        # the commit that removed them
git show <commit>^ --stat -- plans/m1a_rust      # what was in the tree
git show <commit>^:plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md
```

Also in git history and not in this record: `design/09` (server mode, deleted
2026-08-18 with the server), the seven server stages, and S1a.9.1 / S1a.9.2
(the PyO3 surface and its test suite, cut 2026-08-21).

## Cross-links

- [`docs/kernel/`](../../kernel/README.md) — the specification ein.rs
  implements, and since S1a.10.6 the only statement of intent that is not also
  the implementation; [`defined_behaviour.md`](../../kernel/defined_behaviour.md)
  states the behaviours whose only statement was the code
- [`docs/api/rust.md`](../../api/rust.md) — the embedding surface S1a.9.4
  wrote, whose worked example is a test
- [`corpus/`](../../../corpus/README.md) — the corpus, a sweep since S1a.10.3
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
