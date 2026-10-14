# Open Questions — M1a (the Rust engine)

Milestone-scoped questions. Ids are **sticky** — `Q-M1a.<n>`, following
the `Q-S1.5a.6.B` style used inside M1 stages rather than the global
`Q<n>` sequence in [`plans/open_questions.md`](../../../plans/open_questions.md), so
the two namespaces cannot collide. A closed id is never reused.

> ### Closed with the milestone — 2026-08-23 (S1a.9.4 T1a.9.4.5)
>
> **Seventeen questions, fifteen settled.** Twelve were resolved,
> decided, answered or closed moot inside M1a; three
> ([Q-M1a.19](#q-m1a19--how-does-a-program-state-what-it-expects),
> [.20](#q-m1a20--what-may-an-expectation-say),
> [.21](#q-m1a21--may-the-search-stop-before-the-lattice-is-exhausted)) moved
> with the phases that owned them into M1c and M1d, keeping their text; and
> [Q-M1a.23](#q-m1a23--when-does-the-engine-need-a-python-binding) is a
> **deferral with three trip-wires**, which is a decision about when to
> decide.
>
> **One is still open and is meant to be**, so the milestone does not close
> over it silently: [Q-M1a.6](#q-m1a6--at-none-in-loader-messages), a loader
> message that says `at None`. It was two until 2026-08-29, when
> [Q-M1a.8](#q-m1a8--_binding_key-drops-non-string-activator-args) was
> **closed as stated and refiled as its real self**: the probe M1e S1e.1.4 ran
> found that the collision it named does not exist and a neighbouring one
> does. Both were things the milestone's surface invariant **I1** forbade
> fixing, and what is left of that constraint is the cost of re-blessing the
> corpus goldens, which is a decision for whoever next has a reason — and for
> Q-M1a.8 there is now
> [Q-M1e.16](../m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one)
> to make it.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1a.3](#q-m1a3--parse-error-messages) | Parse-error messages, including `-1:-1` at EOF | **resolved 2026-08-18 — (a)** |
| [Q-M1a.6](#q-m1a6--at-none-in-loader-messages) | `at None` in loader messages (top-level forms carry no `loc`) | **open, and outlives the milestone** — a diagnostic-quality fix that I1 forbade. Owner: whoever needs it; [M2 Q25](../../../plans/m2_nl_to_ir/open_questions.md#q25--what-language-is-the-frontend-written-in)'s validator is the likeliest |
| [Q-M1a.7](#q-m1a7--may---jobs--1-move-counters) | May `--jobs > 1` move counters? | **decided 2026-08-22 — no, and validation is not what buys it.** Held by `jobs_invariance` at 20 712 cells, byte equality, 0 moved |
| [Q-M1a.8](#q-m1a8--_binding_key-drops-non-string-activator-args) | `_binding_key` drops non-string activator args | **closed 2026-08-29 as stated — the collision it named does not exist.** Two probes (M1e [S1e.1.4](../m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3), `rule_semantics::activators_differing_only_by_{an_int_argument_both_fire, a_nested_fact_argument_share_one_binding_key}`): an `int` argument seeds a register and reaches the key, a nested `Fact` does not — and the pair that *does* lose a firing is one of each, refiled as [Q-M1e.16](../m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) |
| [Q-M1a.9](#q-m1a9--where-do-goldens-live) | Where do goldens live? | **answered 2026-08-21 — `ein.rs/crates/<crate>/tests/golden/`** |
| [Q-M1a.10](#q-m1a10--does-f11-d1-beta-memories-land-inside-m1a) | Does F11 D1 (beta-memories) land inside M1a? | **answered no** 2026-08-19 — an index key was the lever, not a memory |
| [Q-M1a.11](#q-m1a11--server-wire-protocol) | Server wire protocol — JSON-RPC vs gRPC vs bespoke | **closed moot 2026-08-18 — no server** |
| [Q-M1a.12](#q-m1a12--remote-access-and-auth) | Remote access and auth for `ein serve` | **closed moot 2026-08-18 — no server** |
| [Q-M1a.13](#q-m1a13--help-and-usage-error-text) | `--help` layout and usage-error text | **resolved 2026-08-18 — (b): behaviour exact, presentation `clap`'s** |
| [Q-M1a.16](#q-m1a16--how-does-the-corpus-drive-the-lever-matrix) | How does the corpus drive the `SolverConfig` lever matrix? | **closed 2026-08-23.** `utils/feature_matrix.py` writes the knob into a copy of the puzzle's `(config …)` block and drives the binary, so all ten levers are reachable |
| [Q-M1a.17](#q-m1a17--win-bs--80--assumed-monotone-guards-dominate) | Win B's ≥ 80 % assumed monotone guards dominate — they are 7–16 % | **closed 2026-08-20: the mechanism is declined at a measured 1.4–2.2 % ceiling**, in [S1a.6.12](README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot), which took 38 % off `zebra -e` without it |
| [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) | May a fork stop re-narrating the root's fixpoint? | **resolved 2026-08-19: yes** — mechanism shipped in S1a.6.10 / S1a.6.11 |
| [Q-M1a.19](#q-m1a19--how-does-a-program-state-what-it-expects) | How does a program state what it expects? | **moved 2026-08-21 with P1a.11 → [Q-M1c.1](../m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)** |
| [Q-M1a.20](#q-m1a20--what-may-an-expectation-say) | What may a `(test …)` expectation say? | **moved 2026-08-21 with P1a.11 → [Q-M1c.2](../m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)** |
| [Q-M1a.21](#q-m1a21--may-the-search-stop-before-the-lattice-is-exhausted) | May the search stop before the lattice is exhausted? | **moved 2026-08-21 with P1a.12 → [Q-M1d.1](../../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)** |
| [Q-M1a.22](#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate) | Is `.einb`'s id remap order-preserving enough for its own gate? | **resolved 2026-08-23 — (a)**, the recommendation, as [P1a.8](README.md#p1a8--binary-kb-container) shipped it: byte-identity is promised on the fast path and answer-identity always. `ein solve x.einb` is byte-identical to `ein solve x.ein`, and `id_order_invariance` is the measurement it rests on |
| [Q-M1a.23](#q-m1a23--when-does-the-engine-need-a-python-binding) | When does the engine need a Python binding? | **deferred 2026-08-21** — no consumer today; three named trip-wires |

---

## Q-M1a.3 — Parse-error messages

> **Stated as ein's defined behaviour** — [`docs/kernel/defined_behaviour.md` §1.1–1.4](../../kernel/defined_behaviour.md) (M1a [S1a.10.6](README.md#s1a106--defined-behaviour)). This entry is the *decision*; that page is what the engine promises.

A parse error is `{file}:{line}:{col}: unexpected input\n{context}`.
Observed quirks: EOF errors report `-1:-1`, and the caret lands one past
the last token ([design/04](design/04_ir_frontend.md) §4).

Options: (a) keep the messages exactly, quirks included; (b) keep the
non-EOF cases and fix EOF; (c) improve the diagnostics, re-baselining the
four `examples/broken/` fixtures.

**Recommendation: (a) through M1a, then (c) as a separate, deliberate
change** — improving diagnostics while the engine is still being built
would hide regressions in noise.

**Resolved 2026-08-18 at [S1a.1.1](README.md#s1a11--lexer-parser-error-messages):
(a), and it needed more than the EOF case.** Four behaviours are kept, and
only the first was known when this question was written:

1. **`-1:-1` at EOF**, with the context rendering the last line and a caret
   one past its end.
2. **A ±40-character context window**, applied *before* the line is
   trimmed, so an error past column 40 renders a **truncated** source
   line.
3. **The delayed error position**: whitespace and comments are matched at
   every position the scanner passes — including inside a string literal,
   and including positions no grammar rule is looking at — and a pending
   match holds the error back until the scanner walks past it. `(y";"{`
   reports the `{`; `(y";"{?` reports the `?`. Found by the fuzzer, not
   by reading, and computed in `parse::death_position`.
4. **Ambiguity resolution prefers the earlier alternative**, which is
   what makes `(rulex …)` a rule named `x` rather than a fact named
   `rulex`.

`ein-ir/tests/grammar_decisions.rs` checks the four `examples/broken/`
messages and the ambiguity decisions. The (c) half — improving the
diagnostics — stays open.

## Q-M1a.6 — `at None` in loader messages

> **Stated as ein's defined behaviour** — [`docs/kernel/defined_behaviour.md` §1.5](../../kernel/defined_behaviour.md) (M1a [S1a.10.6](README.md#s1a106--defined-behaviour)). This entry is the *decision*; that page is what the engine promises.

Top-level `SForm`s are constructed without a `loc`
([design/04](design/04_ir_frontend.md) §3), so loader errors that
interpolate `at {form.loc}` print `at None`. ein.rs has the position and
would naturally print it.

**Recommendation: print `at None` through M1a, then fix it** in a later
stage. Tracked here so the fix is not forgotten; it is a genuine usability
bug.

**Landed at [S1a.1.3](README.md#s1a13--macro-expansion-and-import-resolution)**:
`ast::loc_repr` renders `None` for a top-level form and
`Loc(file='…', line=6, col=20)` otherwise, and every one of the eleven
`examples/broken/load/import_*.expected` messages ends `at None`. The fix,
when it comes, re-baselines all of them at once.

**Confirmed across the whole loader at
[P1a.2](README.md#p1a2--kb-core)**: all eighteen remaining
`examples/broken/load/` messages end `at None` too, and the one that does
*not* — `macro_arity_mismatch`, whose error is raised inside macro
expansion on a nested node — carries a real `Loc`. So the re-baseline is
exactly "every loader message except one".

## Q-M1a.7 — May `--jobs > 1` move counters?

[design/08](design/08_parallelism.md) commits to deterministic parallel
execution (same counters, same output) via speculate-and-validate, with
`--unordered` as an opt-in that relaxes the promise to the same answer.

The open part is whether the validation cost is acceptable in the regimes
that matter (a large no-good store with frequent singleton writebacks).
Measure the re-validation rate in [P1a.7](README.md#p1a7--parallelism); if
it is high, the fallback is to make `--unordered` the documented
recommendation for large searches rather than to weaken the default.

**Measured 2026-08-20 at
[S1a.7.0](README.md#s1a70--the-speculation-audit), before any of the
mechanism was built** — the sequential engine, and beside every entering the
same entering re-run against layer-start root. 1 078 704 enterings over 69
corpus entries. The decision is not made here; four things it needs are:

1. **The rate is bimodal, and an average hides it.** 0.1 % corpus-wide;
   **36–50 %** on the zebra family and **97.2 %** on `zebra2-hints -e`; 1.8 %
   on `branching/07 -e`; **0 %** on the other 65 runs. The phase's "≤ a few
   percent" criterion passes on the average while failing every workload a
   reader would recognise, and is restated per workload.
2. **Case 3 lives only in layer 1.** Layer *L* enters commitment sets of size
   *L*, a death licenses a clause of width *L*, and only a width-1 clause is a
   *fact* root can hold — so only layer 1 adds a fact to root mid-layer. Every
   layer above the first is case 1 by construction — and 98.2–99.9 % of the enterings of a workload big enough to
   want cores are there. So the question is not "is validation affordable"
   but "**what happens to layer 1**".
3. **The speculation is wrong, not stale.** 35 enterings come back `alive`
   where the sequential engine says `dead-post` — the mid-layer `(not h)` is a
   premise of the domain-elimination rules. No read-set filter that waved
   these through would be sound.
4. **Fail-fast is entangled with it, and design/08 never said so.** With
   `enable_fail_fast_fork` off the speculation's `core` errors collapse
   exactly onto its `kind` errors (35 = 35); with it on, 40 more cores differ
   because the two forks stopped at different firings of the same death. A
   continuation recovers `kind`; it recovers `core` only where the fork ran to
   quiescence. **That interaction is what
   [S1a.7.2](README.md#s1a72--level-1-parallel-enterings) has to settle**,
   and it is what decides whether `--jobs N` can keep byte-identical output or
   has to take a `--jobs`-scoped relaxation the way
   [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) took
   one for the fork.

The **recommendation is unchanged**: no counter movement, plus `--unordered`
as the opt-in escape. What changed is that its price is now a number rather
than a hope — see
[scaling.md §3](measurements/scaling.md#3-the-audit).

### Decided 2026-08-22 — no, and validation is not what buys that

[S1a.7.2](README.md#s1a72--level-1-parallel-enterings) § The decision,
taken before any of the mechanism was built, on one more measurement over the
same event stream:

> A layer is fanned out **iff it cannot write a fact to root** — every layer
> ≥ 2, and layer 1 when `enable_singleton_writeback` is off. Layer 1 with the
> writeback on runs sequentially.

So `--jobs > 1` moves **no** counter, and it does not need a validator to
manage that: a fanned-out layer computes against exactly the root the
sequential engine would give it. The four points above resolve as follows.

1. **The bimodal rate stops mattering.** It was the cost of *repairing*
   speculations, and nothing is repaired. The rate on a fanned-out layer is 0
   by construction; the workloads with a high one are the ones whose layer 1
   is now serial.
2. **Confirmed and widened.** 248 of 248 writebacks corpus-wide are in layer 1,
   over 8 158 205 enterings spanning five layers — and layer 1 is **0.016 %**
   of them. Serialising it costs 0.24 % of `branching/07 -e`'s Phase-2 firings
   and *nothing* on the other three workloads of the measurement set.
3. **Still true, and now moot.** The 35 wrong speculations are why no read-set
   filter would have been sound; they are the evidence for the decision rather
   than a problem it has to solve.
4. **Answered by removal.** The interaction was between `enable_fail_fast_fork`
   and a *continued* fork, and there are no continuations. Fail-fast keeps its
   1.9–2.4× whole, no fork's `core` is read off a different firing than the
   sequential engine's, and no `--jobs`-scoped relaxation is taken — so this
   question does **not** join
   [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint).

The one route that would have moved a counter is **batch-synchronous
integration**, and it is rejected for that: 101 → 521 enterings on `zebra2 -e`
under a whole-layer barrier. Its 2.8× on `branching/07 -e` is the layer-stack
depth rather than the deferral — identical entering count — and is taken
separately by flattening root at the barrier.

**What would re-open this**: a mechanism that writes a fact to root mid-layer
at any depth. The predicate is about *writing*, not about layer 1, and
S1a.7.2's debug assertion is what would notice.

> **And one thing the predicate never covered — closed 2026-08-29.** It is
> about a **fact**, and the learned no-good clause is not one: every dead
> entering of every fanned-out layer emits one into a store `Kb::branch` shares
> with every worker by `Arc`. That the sharing is harmless was true, and was
> written nowhere — which is what M1e's review asked as
> [Q1](../m1e_review_processing/review/open-questions.md). The
> argument is now [02](design/02_determinism_and_order.md) §6a and the premise
> it rests on is enforced by a freeze the fan-out takes; the answer to this
> question does not change, and the reason it is right is one layer deeper than
> "no counter moved".

## Q-M1a.8 — `_binding_key` drops non-string activator args

> **Stated as ein's defined behaviour** — [`docs/kernel/defined_behaviour.md` §3.2](../../kernel/defined_behaviour.md) (M1a [S1a.10.6](README.md#s1a106--defined-behaviour)). This entry is the *decision*; that page is what the engine promises.

A rule application's binding key takes `plan.activator_args`, which the
compiler builds from the activator's **symbol** arguments only — while the
*plan cache* key renders **all** args. Two activators differing only in an
`int` arg therefore share a binding key and can suppress each other's
firings.

Almost certainly unintended. **Keep it as-is through M1a** (it is current
behaviour, and changing it moves the surface), with a fixture that
demonstrates it, and fix it after.

> **Closed 2026-08-29 — the inference above is wrong.** M1e
> [S1e.1.4](../m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3)
> (the review's `Q3`) ran the probe this entry asked for and it came back the
> other way.
>
> The binding key has **three** components, not two. `plan.activator_args` is
> the second; the third is the register file, and an `int` activator argument
> **binds its parameter** — the compiler's bind loop binds a symbol or an
> `int` — so it is in the third. The entry read the second alone and
> concluded from it. Two activators differing only in an `int` argument have
> different keys and **both fire**
> (`ein-infer/tests/rule_semantics.rs::activators_differing_only_by_an_int_argument_both_fire`).
>
> What genuinely reaches neither component is a **nested `Fact`**, which binds
> nothing. Two activators differing only there do share a key — and lose
> nothing, because a `Fact` argument reaches the compiler nowhere but the bind
> loop that skips it, so the two plans are equal in every field
> (`…::activators_differing_only_by_a_nested_fact_argument_share_one_binding_key`).
>
> The shape that costs a derivation is an `int` in the position another
> activator gives a nested `Fact`: same key space, **different register
> layouts**, and the identity then compares different variables by position.
> That is a live bug, it is stated in
> [`defined_behaviour.md` §3.2](../../kernel/defined_behaviour.md#32-a-rule-applications-identity-ignores-nested-fact-activator-arguments),
> and it is owned by
> [Q-M1e.16](../m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
> No corpus program can reach any of the three shapes: every plan compiled by
> `ein solve -m 2` over all 204 corpus `.ein` files binds against 153 143
> activator arguments, and every one is a symbol.

## Q-M1a.9 — Where do goldens live?

A golden is a checked-in expected output. Does it live in one repo-root
`testdata/golden/`, or beside the code whose output it pins?

**Answered 2026-08-21: `ein.rs/crates/<crate>/tests/golden/`** — not
repo-root. The goldens
[S1a.6.11](README.md#s1a611--fixtures-for-the-narration) added landed beside
the crate whose output they pin rather than in a shared tree: a golden is read
by exactly one test, and `ein_corpus::golden_path(krate, name)` is the whole of
the convention.

[S1a.10.3](README.md#s1a103--the-corpus-as-a-sweep)
is what closes it, because the corpus's `golden` group was empty *pending this
question* and an empty group is a question with a home. The group is gone.

## Q-M1a.10 — Does F11 D1 (beta-memories) land inside M1a?

[F11](../../../plans/followups/f11_deductive_layer_perf.md) parks RETE beta-memories
on a fork-state design problem that [design/03](design/03_data_model.md)
§5 dissolves. [design/05](design/05_matcher.md) §7 sketches the answer,
and [P1a.6](README.md#p1a6--performance) schedules it.

Open: whether it is still the largest lever *after* the register matcher
and the semi-naive boundary land. It may not be — those two remove the
costs that made partial-join recomputation expensive. **Decide by
profile, not by plan**; if it is a wash, revert it and leave F11 open,
exactly as P1.8a's D3 was handled.

**Answered *no*, 2026-08-19, by the profile
([S1a.6.3](README.md#s1a63--beta-memories-f11-d1)).** It is not the
largest lever and it is not built. The gated stage ran and found that two
cheaper changes had removed what the memory was for:

- **the intermediate it would materialise is 2.2 tuples wide.** T1a.6.3.0
  keyed the participation index one level inside a nested argument, and an
  exhaustive `zebra`'s candidates went 25 160 149 → **1 171 385** — from 47.4
  per step entered to **2.21** — while the run went 349 → **78 ms**;
- **the per-fork copy is not free after all.** T1a.6.2.5 built the flat
  per-relation table design/05 §7 calls the root memory and **cloning it per
  fork cost 7.6 %**, while the fork-free bench got 8 % faster. A fork shares
  the layered index by `Arc`; a materialised memory gives every live fork its
  own copy of the thing the matcher scans;
- **the re-derivation the root memories would replay is gone** — S1a.6.9's
  resumed fork saturator removed it.

F11 D1 stays open and re-priced rather than closed: promotion now needs a
workload whose per-step candidate count is large again. The stage's other
answer, D2's trigger, is recorded there too — the *cyclic body* half is met by
`stdlib/slots.ein` and the *cost* half is not.

## Q-M1a.11 — Server wire protocol

**Closed moot 2026-08-18: there is no server.** The question was to be
decided at P1a.8 kickoff "informed by what M20 picks for its stack" —
and that is exactly what dissolved it. M20 picked Tauri
([M20 § Stack](../../../plans/m20_gui/README.md#stack)), whose backend is a Rust
process linking `ein-core`/`ein-ir`/`ein-infer` directly; a wire protocol
between the GUI and the engine would have been a serialisation boundary
inside one process. With M2 reaching the engine through PyO3
([P1a.9](README.md#p1a9--release)) and the CLI running in-process,
no consumer was left. The JSON-RPC recommendation and the rest of
`design/09` are in git history if a hosted use case ever revives them.

## Q-M1a.12 — Remote access and auth

**Closed moot 2026-08-18 with [Q-M1a.11](#q-m1a11--server-wire-protocol).**
There is nothing to expose: the engine is a library and a CLI. If hosted
use is ever wanted, the posture recorded here still holds as a starting
point — a reverse proxy plus a token in front of a purpose-built service,
never an auth system inside the engine.

## Q-M1a.13 — Help and usage-error text

> **Stated as ein's defined behaviour** — [`docs/kernel/defined_behaviour.md` §5](../../kernel/defined_behaviour.md) (M1a [S1a.10.6](README.md#s1a106--defined-behaviour)). This entry is the *decision*; that page is what the engine promises.

**Resolved 2026-08-18: (b), with (c)'s content half made binding.** ein.rs
uses `clap`; `--help` layout *and* usage-error text are `clap`'s. Everything
a script or a habit can depend on stays exact — the difference is
presentation, and only presentation.

The surface I1 fixes includes `--help` output and CLI error messages, and
M1's has its own layout and its own error text. `clap` does not match it and
cannot be configured to.

The options were: (a) hand-roll the argument parser and the help renderer
to match it byte-for-byte; (b) use `clap` and let `--help`/CLI-error text be
`clap`'s; (c) match the *semantics* (flags, defaults, mutual exclusion, exit
codes) exactly and accept different help text.

### What stays exact

- The three subcommands, the four `render` sub-subcommands, and the
  delegated dispatch — `ein saturate --help` prints `saturate`'s own help
  as `ein saturate`, and `saturate` still appears in `ein --help` though
  the top parser never parses it.
- Every option at every level: long name, short key, metavar, arity,
  default, `choices`, mutually-exclusive group — and its help *string*,
  which is content, not layout.
- The accept/reject verdict on every invocation, and the exit code.
- Which stream each byte goes to.

Free: wrapping, indentation, headings, ordering within a section, and the
wording of a diagnosis.

### Why not (a)

The two halves are not separable. M1's CLI welds its wrapped `usage:` block
onto *every* error, so exempting the layout exempts the message, and a
byte-exact error needs the usage formatter, which is most of what (a) was
priced at. The middle option — reproduce the `ein solve: error: …` diagnosis
line and drop the usage block — was offered and declined: half a formatter
for a line nothing reads mechanically.

### What pins the content

Without a byte comparison of `--help`, something else has to notice a lost
option. The help is parsed into a structure —
`{subcommand → {option → short, metavar, arity, default, choices, group,
help}}` — and the structure is checked in as a golden. On the property that
matters this is *stronger* than a byte diff: a renamed short key or a changed
default fails on its own line, instead of somewhere inside an 89-line
text blob. Same instrument shape as
[S1a.5.3](README.md#s1a53--state-dumps)'s `dump-shape` —
when there is no line protocol to diff over, render one.

### What would make this unacceptable

A consumer that reads `ein --help` or matches on ein's stderr text. There
is none as of 2026-08-18: no script under `utils/` parses either, and
`feature_matrix.py` only *echoes* a failing child's stderr into a report
field. The day one is written, this is the entry to revisit.

## Q-M1a.16 — How does the corpus drive the lever matrix?

The corpus run matrix is meant to include "each `SolverConfig` lever flipped
off (the same matrix [`utils/feature_matrix.py`](../../../utils/feature_matrix.py)
already drives)". Building the manifest at
[S1a.0.1](README.md#s1a01--corpus-manifest-and-determinism-audit)
found that only **four of the ten** are reachable from the CLI — `-L`
(`enable_pre_branch_lookahead`), `-K` (`enable_lookahead_kill_cache`), `-y`
(`lattice_sanity_check`) and `-o score-sum` (`lattice_order`). The other six —
`enable_path_nogoods`, `enable_symmetric_mirror`, `enable_singleton_writeback`,
`enable_forced_positive`, `enable_fail_fast_fork`, `hypgen_scoring` — exist only
in a `SolverConfig` built in code or a puzzle's own `(config …)` block, and the
corpus sweep runs a process, so it cannot reach them.

That matters more than it looks: those six gate exactly the optimisations
[P1a.6](README.md#p1a6--performance) will build, and "lever off" is the
cheapest way to isolate a failure to one of them.

**Restated 2026-08-21 (S1a.10.3).** The `levers` column of the sweep runs the
four CLI-reachable levers as extra `solve` cells, and the six unreachable
levers are reachable in the *only* way that matters — `ein-infer`'s tests
construct a `SolverConfig` directly, which is how `search_semantics.rs` and
`lattice_semantics.rs` already exercise them. What the CLI still cannot do is
put a lever into a **corpus** cell, so what the question is asking has narrowed
to "should the corpus sweep cover the other six?" — a coverage question about
one test, not a gap in the gate.

Options:

- **(a) add `--config KEY=VALUE` to the CLI** (repeatable, kebab-cased,
  parsed by the same coercer `(config …)` uses). Additive, ~20 lines, makes
  `levers = "all"` real. Costs: one more flag on the surface, and a way to set
  a lever that a puzzle file cannot audit.
- **(b) generate per-lever puzzle variants** — copy the fixture with a
  `(config …)` block appended into a temp dir. No CLI change, but the corpus
  entry is then not the file in `examples/`.
- **(c) leave the six unexercised** by the corpus and note the gap. The corpus
  keeps four levers; the other six are covered only by `ein-infer`'s tests.

**Recommendation: (a)**, decided before [P1a.6](README.md#p1a6--performance)
rather than at it, because the flag is cheapest to add while the CLI is still
a stub. Until then the manifest's `levers` lists the four, and
`corpus/README.md` says why.

## Q-M1a.17 — Win B's ≥ 80 % assumed monotone guards dominate

**Found 2026-08-18 at [S1a.3.4](README.md#s1a34--the-naf-boundary),
by measurement rather than by argument.**

[design/06](design/06_saturation.md) § Win B projects that guard sub-plan
**evaluations** drop by ≥ 80 %, and names the mechanism: a *monotone*
guard's query is purely positive, so if it found nothing at round *r* it
can only start finding something through a fact added since — which is
`run_seeded` on the guard's sub-plan, restricted to Δ ∩ watched.

ein.rs instruments that split (`Saturator::guard_evals` /
`guard_evals_monotone`), and at **root scale the mix is the other way
round**:

| root | rounds | guard evaluations | of which monotone |
|---|---:|---:|---:|
| `zebra2` | 40 | 958 | **109 (11 %)** |
| `zebra` | 119 | 945 | **280 (30 %)** |

The reason is structural rather than incidental. A candidate that is
*still parked* has a guard that **failed**, and a failing **monotone**
guard retires its candidate on the spot — so every re-judged candidate is
one whose failing guard is *non-monotone*, i.e. a `forall`'s
`(absent (and G (absent B)))`, which design/06 excludes from the
mechanism by name. What is left for the semi-naive path is the monotone
guards that *passed* earlier in the same `first_failing` scan.

The boundary is still where the time is — 80 % of a `zebra` root
saturation and 34 % of a `zebra2` one, and essentially all of it inside
the queries themselves (945 evaluations × 6.2 µs ≈ the whole 5.8 ms).
The two refinements that do not depend on monotonicity **landed**: the
per-round `(guard, projected env) → verdict` memo, and an
allocation-free watch stamp on an ordered parked set instead of a
pop-and-re-push heap. Together they moved the boundary by ~2 % at root
scale, which is the honest number.

~~**Open:** does the *exhaustive* mix differ?~~ **Answered 2026-08-19, by
running it** — the recommendation below said the question would be settled that
way and it was, at the head of
[S1a.6.12](README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot). `guard_eval`
and `guard_eval_monotone` are the `Saturator`'s pair summed over every fork of a
solve, which is the aggregation no per-saturation field could do
([baseline.md §17](measurements/baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it)):

| solve | guard evaluations | of which monotone |
|---|---:|---:|
| `zebra2` | 9 978 | 499 (5.0 %) |
| `zebra2 -e` | 30 691 | **2 250 (7.3 %)** |
| `zebra` | 8 645 | 906 (10.5 %) |
| `zebra -e` | 29 505 | **4 505 (15.3 %)** |
| `features/05 -e` (blind, 384 167 forks) | 4 719 834 | 493 985 (10.5 %) |

**The exhaustive mix is worse than the root-scale one**, and the reason is the
structural one above rather than an accident of these puzzles: a failing
monotone guard retires its candidate on the spot, so a *longer* run retires
more of them and what stays parked is more purely non-monotone. Scale moves
this the wrong way, permanently.

**Resolved: the mechanism is declined, and the number is its epitaph.** Win B's
headline reaches 15.3 % of `zebra -e`'s guard evaluations, which are 22.2 % of
the run — a **3.4 %** ceiling for the design's flagship saturation win.
`Matcher::holds_seeded` exists and the lookahead uses it, so wiring it into the
boundary stays available at [T1a.6.12.4](README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)
if the days are there; it is last in the stage and expected not to run.

**Closed 2026-08-20. It did not run, and the ceiling is now measured on both
sides of its product** rather than on one
([baseline.md §18](measurements/baseline.md#18-s1a612--the-boundary-and-the-premise-that-had-nothing-left-to-bind)):

| cell | monotone share of guard evaluations | `Matcher::holds` share of the run | ceiling |
|---|---:|---:|---:|
| `solve zebra -e` | 16.3 % | 13.7 % | **2.2 %** |
| `features/05 -e` | 11.1 % | 19.2 % | **2.1 %** |
| `features/01 -e` | **100 %** | **1.4 %** | **1.4 %** |

`features/01 -e` is the finding that settles it. Every one of its 599 375 guard
evaluations *is* monotone — design/06's ≥ 80 % is not wrong about programs in
general, it is wrong about which programs have a boundary worth optimising, and
that cell's boundary is 2.9 % of its run. No cell in the corpus has both a
monotone-dominated mix and a boundary that costs anything, and the structural
argument above says why that is not a coincidence.

The rest of S1a.6.12 took `zebra -e` from 76.7 to **47.5 ms** without the
mechanism at all — and the ceiling shrank as it did, because the denominator
did: `Matcher::holds` was 22.2 % of the run when this question was opened and
is 13.7 % now.

**What the boundary needs instead** — the second half of "if it is not, the
boundary needs a different idea, and this question is where that gets decided"
— is the refinement that never landed: **a third of the boundary is not the
queries at all.** It is visiting 248 043 parked candidates per solve to ask
29 865 questions, and ≥ 88 % of those visits are provably skippable. That is
design/06 § Win B refinement 3, and it is
[T1a.6.12.1](README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot).

**The recommendation that got here, kept for the record:** carry the semi-naive
guard re-evaluation (T1a.3.4.5) into [P1a.6](README.md#p1a6--performance) as a
*measured* optimisation with this as its trigger condition, rather than landing
a mechanism here whose measured reach is a tenth of its stated one. If the
exhaustive mix is monotone-dominated, it lands there with a number; if it is
not, the boundary needs a different idea and this question is where that gets
decided.

---

## Q-M1a.18 — May a fork stop re-narrating the root's fixpoint?

**Resolved 2026-08-19: yes.** `Saturator::resume` is the shipping path.

The principle that moved with it, and it is bigger than this question: **the
hard requirement is identical final solutions**, not identical bytes.
P1a.1–P1a.5 held the engine's output to the byte; from P1a.6 on,
byte-identical *narration* is a means that has served its purpose, and its
regression coverage moves to checked-in fixtures
([S1a.6.11](README.md#s1a611--fixtures-for-the-narration)). The answer and
the counters stay exact.
[S1a.6.10](README.md#s1a610--answers-not-narration) is the mechanism.

Two things decided it, and neither was the speed. **The answers do not move** —
3.2 M enterings compared fact by fact, every verdict, model, unsat core and all
85 `summary.json` counters identical. And the trace gets *better*: it now opens
with root's own derivations, then `Assuming …`, then what the hypothesis adds,
which is how [`zebra_walkthrough.md`](../../kernel/inference/zebra_walkthrough.md)
tells it and what the accidental re-derivation was standing in for.

The cost accepted, stated plainly: **267 529 facts record a different — equally
valid — one of their derivations as the primary**, because a resumed fork
inherits root's parked candidates with root's tiebreakers and the NAF boundary
admits one per round. The engine never promised *which* derivation of a
multiply-derivable fact it records first; it does now promise less than it
happened to deliver.

**Both halves landed the same day.** S1a.6.10 wrote the rule once — a fork's
derivation, and anything keyed on a dying fork's stopping point, is narration —
into one crate, `ein.rs/crates/ein-narration`, replacing the six ad-hoc cuts
S1a.6.9 had left behind, and `EIN_NARRATION_STRICT=1` keeps byte-identical
narration one variable away for the determinism sweep. S1a.6.11 replaced the
elided bytes with twelve ein.rs goldens and made idea-08's walkthrough-rule
assertion a test of the engine that ships. **This question is closed.**

---

*What follows is the question as it stood, with the evidence T1a.6.9.2/3
produced. Kept because the reasoning is the record.*

**Found 2026-08-18 at [S1a.6.9](README.md#s1a69--the-fork-entry-delta-the-resumed-saturator),
by measurement.** The numbers are in
[baseline.md §9](measurements/baseline.md#9-the-fork-entry-re-derivation).

Every entering builds a fresh `Saturator` over the forked root, so its
first enqueue pass is a FULL pass and the root's whole deductive closure
is re-derived inside the fork. Measured on `-e` runs: **95.6 %**
(`zebra2`, 36 442 / 38 136) and **94.6 %** (`zebra`, 107 610 / 113 746) of
a fork's firings are redundant re-derivations, and `try_commitment_set` is
**95.0 %** of `zebra -e` cumulatively — the one workload that misses its
milestone target.

Resuming the saturator from the root's state (`engine`, `seen`, `fired`,
`parked`, tiebreaker) with `delta = the commitment facts` removes them.

**Built and measured 2026-08-18** (T1a.6.9.2/3): `Saturator::resume`, behind
`--features fork-delta` and dormant without `EIN_FORK_DELTA=1`, so one binary
produces both arms and the shipping path never takes it. Full numbers in
[baseline.md §11](measurements/baseline.md#11-the-resumed-fork-saturator-measured);
the rendered before/after this question was to be decided against is
[fork_delta_trace.md](measurements/fork_delta_trace.md).

**It works, and it is worth what §9 said.** Fork firings 38 136 → 9 834 on
`zebra2 -e` and 113 746 → 26 656 on `zebra -e`; fork compiles → 0; identical
productive firings. `solve zebra.ein -e` 525.6 → **392.6 ms — the phase's one
unmet target, crossed**. The trace's solution node goes 561 → 240 steps and
opens on the hypothesis's own first consequence instead of on eight
`symmetric` closures of `next-to`.

**Three claims held and one did not.** Over the whole corpus, 1.08 M
enterings compared fact by fact and justification by justification
(`utils/fork_delta_verify.py`):

- ✅ **the fixpoint** — every alive fork's fact set is identical, and with
  `enable_fail_fast_fork` off so is every *dead* fork's;
- ✅ **the verdict, `k`, the models, the entering count, each entering's
  `kind`** — stdout is byte-identical on every corpus entry;
- ✅ **the unsat core** — identical with fail-fast off; with fail-fast on, 39
  dead-post cores differ, which is the fail-fast *prefix* (a different clash
  reached first ⇒ a different, equally minimal frontier), not a different
  conflict;
- ❌ **the provenance graph.** **90 002 facts get a different *primary*
  justification.** A fresh fork renumbers root's parked candidates in plan
  order; a resumed one inherits root's tiebreakers, so they sort first — and
  the boundary admits at most one candidate per round, so a fact derivable
  two ways (`functional-negative`/`injective-negative`,
  `domain-elimination`/`range-elimination`, `total`/`surjective`) gets a
  different **first** derivation, and first derivation wins. Matching a fresh
  pass's numbering requires running a fresh pass, so this **cannot be
  designed away**; it is the price, not a bug in the prototype.

So the narration moves further than this question assumed:

- **the event stream** loses 62.5 % / 75.9 % of its lines at `verbose` —
  **and 58.8 % / 74.2 % at `normal`**, which the question did not anticipate:
  a redundant firing is not emitted at `normal`, but the ~1 790 `enqueue`
  lines per entering that produce it are;
- **the rendered output** moves `n_firings` in `--trace`, the
  `("firings", len)` counts in `--dump-states`, the *first five firings*
  `render/shape.rs` prints — and, for the 90 002 facts above, **which rule the
  proof names**;
- **the answer and the counters** do not move: `BaseStats` never counts a
  firing, and every counter it does keep is unchanged;
- **idea-08's walkthrough-rule acceptance fails**: the solution's trace covers
  12 rules instead of 24 and `symmetric` is not among them, because
  `symmetric` closes `next-to` at *root*. `--trace` gets root's whole closure
  for free today, by accident, because every fork re-derives it. Adopting this
  means the trace has to render root saturation as its own section — which is
  what a human walkthrough does anyway, and is a change to the renderer rather
  than to the engine.

**The options.**

- **(a) No.** I1 is the milestone's spine and the trace is an observable.
  The win is then taken only where it is invisible:
  [S1a.6.8](README.md#s1a68--the-compile-cache-and-the-extent-counts) for the
  compile share and [S1a.6.3](README.md#s1a63--beta-memories-f11-d1)'s
  *root* beta-memories for the match share — same firings, same order,
  discovered by lookup instead of by rescanning.
- **(b) Yes, behind a flag** that is off by default. Keeps I1 and gets the
  speed for the M20/M2 consumers — at the cost of a second code path through
  the saturator's most delicate ordering, which is exactly what P1a.6 Rule 3
  (a wash is a revert) exists to discourage. The flag is **already built**
  (`fork-delta` + `EIN_FORK_DELTA`), so this option is the cheapest to take
  and the only one that is currently *true* of the tree.
- **(c) Yes, as the shipping path.** This is a change to the engine's
  narration, so it is a relaxation of I1 made in writing, never a silent
  retrofit. It buys a renderer change (root saturation as its own trace
  section) and a re-baselined walkthrough acceptance, and it accepts that a
  fact's recorded proof may name a different one of its valid derivations.

**Recommendation: (c), and the argument is not primarily speed — but it is a
bigger change than it looked.** A fork's firing list is what
[`08-human-style-deductive-trace`](../../../plans/ideas/08-human-style-deductive-trace.md)
renders, and 961 re-derivations of what was already true before the
hypothesis is noise in it — the human walkthrough in
[`zebra_walkthrough.md`](../../kernel/inference/zebra_walkthrough.md)
narrates what a hypothesis *adds*, and the rendered after-trace does exactly
that from its second step. The speed is a consequence, and it happens to be
the consequence that closes the milestone's last open target.

What the evidence adds to the recommendation is the **cost line**: the
engine would stop promising *which* derivation of a multiply-derivable fact
it records first. That promise is not written down anywhere as a guarantee —
`record_justification`'s contract is "first derivation wins", and which
derivation is first was already a function of priority, FIFO order and the
boundary's one-admission-per-round rule — but 90 002 facts is not a rounding
error, and a proof that names `injective-negative` where it used to name
`functional-negative` is a *different explanation of the same fact*, not a
shorter one.

**So the question is now sharper than "may the trace get shorter":** may the
engine record a different one of a fact's valid derivations as its primary,
in exchange for a trace that starts at the hypothesis and a solver that meets
its last target? **(a)** says no and takes the invisible half through
[S1a.6.3](README.md#s1a63--beta-memories-f11-d1). **(b)** says not by default.

**Decided: (c), and the contract relaxes with it.** Not (b): the flag is not
the shipping configuration, it is the fresh-saturator arm
`utils/fork_delta_verify.py` compares against. The evidence is
[baseline.md §11](measurements/baseline.md#11-the-resumed-fork-saturator-measured)
and [fork_delta_trace.md](measurements/fork_delta_trace.md);
[S1a.6.10](README.md#s1a610--answers-not-narration) is the mechanism it
specified.

## Q-M1a.19 — How does a program state what it expects?

**Moved 2026-08-21 to [Q-M1c.1](../m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)**,
with the phase that raised it: P1a.11 is now
[P1c.1](../m1c_external_validation/README.md#p1c1--stdlib-conformance) in
[M1c](../m1c_external_validation/README.md). The text went unchanged apart
from ids and paths. **The id stays reserved and is never reused.**

## Q-M1a.20 — What may an expectation say?

**Moved 2026-08-21 to [Q-M1c.2](../m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)**,
with Q-M1a.19 and for the same reason — the two are one decision and
[S1c.1.2](../m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
still settles both.

## Q-M1a.21 — May the search stop before the lattice is exhausted?

**Moved 2026-08-21 to [Q-M1d.1](../../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)**,
with P1a.12, which is now
[P1d.10](../../../docs/history/m1d_satisfiability/README.md#p1d10--exhaustive-search) in
[M1d](../../../docs/history/m1d_satisfiability/README.md) — the milestone that exists to answer
it. What the move adds is a fourth candidate this framing did not have: a
state that can report what it still *owes*
([P1d.2](../../../docs/history/m1d_satisfiability/README.md#p1d2--obligations)).

## Q-M1a.22 — Is `.einb`'s id remap order-preserving enough for its own gate?

[P1a.8](README.md#p1a8--binary-kb-container)'s gate is `ein solve x.einb`
**byte-identical** to `ein solve x.ein`, and
[design/10 §3](design/10_binary_format.md#3-ids-across-the-boundary) answers
the id problem with a translation table plus a fast path: "when the live
interner is empty … both tables are the identity and the pass is skipped
entirely. That is the mmap-and-go case."

[S1a.10.1](README.md#s1a101--digests-counter-identities-and-the-id-space-sweep)'s
id-space sweep measured what happens when they are *not* the identity.
Over 2 544 `(file, op)` pairs with the id space permuted:

- **0 answers move** — verdict, model, counters, every non-derivation
  rendering, at 1 and at 8 seeds;
- **66 renderings move**, and all of them are what
  [S1a.6.10](README.md#s1a610--answers-not-narration)'s rule already calls
  narration: 44 where `enable_fail_fast_fork` stopped a dying
  fork at a different firing, 22 in the body of a rendered derivation, where a
  fact with two equally valid justifications recorded the other one first.

So the gate as written is reachable on the fast path and **not** reachable off
it: a `.einb` opened into a non-empty interner produces the same answer and may
produce a different `--trace`.

Two things follow, and neither is decided here.

1. **The fast path's condition is stated wrongly.** `Terms::new` interns the
   eighteen kernel names before any caller can reach the table, so the live
   interner is never empty. The condition that actually makes both tables the
   identity is that the live interner is a **prefix** of the file's — which it
   is, for a fresh process opening a file written by the same build. Saying
   "empty" and meaning "a prefix" is the kind of thing that holds until
   somebody opens two files.
2. **The gate needs a scope.** Either (a) `.einb` promises byte-identity only
   on the fast path, and says so — cheap, honest, and enough for the CLI, which
   is the case the gate was written for; or (b) the remap is required to be
   order-preserving on the sections that reach a derivation, which is a
   stronger promise than the engine makes to *itself* (§ above: id order
   already decides which justification is recorded first), so it would have to
   be bought somewhere other than the container.

**Recommendation: (a)**, with `ein-render/tests/id_order_invariance.rs` as the
measurement it rests on, and the answer-level invariance — which is what a user
of a container cares about — asserted for `.einb` directly at
[S1a.8.1](README.md#p1a8--binary-kb-container).

## Q-M1a.23 — When does the engine need a Python binding?

**Deferred 2026-08-21**, with the two binding stages cut from
[P1a.9](README.md#p1a9--release). Not cancelled: this entry is the trip-wire,
so that "later" is a condition rather than a shrug.

### Why there is nothing to bind for

The phase justified PyO3 with one consumer and the census does not support it.
[M20](../../../plans/m20_gui/README.md) links the crates into a Tauri backend.
[M1c](../m1c_external_validation/README.md)'s runner is `ein-bench`, Rust, and
[S10.2](../../../plans/m10_external_benchmarks/s10.2_systems_and_install.md)
*requires* it to shell out — "`cargo build` never needs Z3" — because a linked
rival and a subprocess rival are not comparable measurements. `utils/`'s
seventeen scripts name their binary on purpose. That leaves
[M2](../../../plans/m2_nl_to_ir/README.md), whose two stated reasons both fail:

- **llama.cpp does not pull in Python.**
  [P2.2](../../../plans/m2_nl_to_ir/p2.2_formalizer/README.md) is a `llama-server` container
  plus a thin HTTP client, mirroring the out-of-tree `acva` — whose client
  is **C++**.
- **The one thing that wanted the binding argues against it.** M2's
  validator/repair loop needs *why* a load failed, as data. The CLI cannot
  supply that — [`defined_behaviour.md` §1/§4](../../kernel/defined_behaviour.md)
  pins those diagnostics as strings — so a binding would have had to grow a
  structured-diagnostics surface the engine does not have. A Rust frontend
  links `ein-ir` and has it with no boundary at all.

What remains genuinely Python in M2 is
[P2.5](../../../plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)'s link-grammar
binding — an experiment whose default deployment is *off* unless it measures a
win ([Q11](../../../plans/m2_nl_to_ir/open_questions.md)), and whose subject has a C API if
it ever graduates. An experiment harness that writes `.ein` files is not a
reason to bind an engine.

### The measurement the deferral rests on

Cold process, this machine, 2026-08-21:

| | |
|---|---:|
| `ein --help` — process startup floor | **1.68 ms** |
| `ein solve examples/zebra2.ein` | 11 ms |

At a 1.7 ms floor, the TPTP library's ~25 k problems cost **~40 s of process
overhead in total** — against per-problem solve times that will be seconds or
timeouts. No planned workload is startup-bound.

### The trip-wires

Bring it back when **any** of these is true. Each is checkable, and none is
true today.

1. **A per-item loop where startup dominates** — N in the millions, or a
   per-item cost near 1.7 ms. Every planned batch (the benchmark corpus, TPTP,
   a BBH sweep) is process-shaped and orders of magnitude above it.
2. **State that must persist across items** — an incremental KB answering many
   small queries, where re-loading rather than re-starting is the cost. Note
   `.einb` ([P1a.8](README.md#p1a8--binary-kb-container)) already answers a large
   part of this from outside the process: a saturated `zebra2` opens cold in
   0.614 ms.
3. **A Python consumer the project does not control** — a notebook, a
   downstream library, or a paper's artifact package.
   **This is the one most likely to arrive**, with
   [M5](../../../plans/m5_presentation/README.md), whose reviewers will expect
   `pip install`. It is a milestone away, not a phase.

### What is held in reserve

[design/12](design/12_toolchain_and_layout.md) §1's `ein-python` crate, §2's
`pyo3`/`maturin` row and §3's `python` feature stay reserved, which keeps this
deferral cheap to reverse.

Two things should be decided with a module in hand rather than in advance: the
**published name** and the **exception hierarchy**.
