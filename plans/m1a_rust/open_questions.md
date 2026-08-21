# Open Questions — M1a (Rust port)

Milestone-scoped questions. Ids are **sticky** — `Q-M1a.<n>`, following
the `Q-S1.5a.6.B` style used inside M1 stages rather than the global
`Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md), so
the two namespaces cannot collide. A closed id is never reused.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1a.1](#q-m1a1--port-boundary-a-full-vs-b-hot-loop) | Port boundary — A (full) vs B (hot loop behind PyO3) | **resolved 2026-08-17 — A** |
| [Q-M1a.2](#q-m1a2--does-einpy-have-a-sunset) | Does ein.py have a sunset? | **decided 2026-08-20 — yes**, reversing the recommendation; [P1a.10](p1a.10_single_implementation/README.md) **shipped 2026-08-21** |
| [Q-M1a.3](#q-m1a3--parse-error-message-parity) | Parse-error message parity, including `-1:-1` at EOF | **resolved 2026-08-18 — (a)** |
| [Q-M1a.4](#q-m1a4--sorted-over-mixed-type-fact-args) | `sorted()` over mixed-type fact args raises in ein.py | **resolved 2026-08-18 — (a), [D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)** |
| [Q-M1a.5](#q-m1a5--reproducing-cpythons-shuffle) | Reproducing CPython's `random.shuffle` for `--shuffle` | **resolved 2026-08-18 — (a), ported** |
| [Q-M1a.6](#q-m1a6--at-none-in-loader-messages) | `at None` in loader messages (top-level forms carry no `loc`) | open — post-parity fix; reproduced at P1a.1 |
| [Q-M1a.7](#q-m1a7--may---jobs--1-move-counters) | May `--jobs > 1` move counters? | open — recommendation stands; **measured 2026-08-20** at S1a.7.0: 0.1 % corpus-wide, 36–50 % where it matters |
| [Q-M1a.8](#q-m1a8--_binding_key-drops-non-string-activator-args) | `_binding_key` drops non-string activator args | open — port as-is, flag upstream |
| [Q-M1a.9](#q-m1a9--where-do-goldens-live) | Where do goldens live? | **answered 2026-08-21 — `ein.rs/crates/<crate>/tests/golden/`** |
| [Q-M1a.10](#q-m1a10--does-f11-d1-beta-memories-land-inside-m1a) | Does F11 D1 (beta-memories) land inside M1a? | **answered no** 2026-08-19 — an index key was the lever, not a memory |
| [Q-M1a.11](#q-m1a11--server-wire-protocol) | Server wire protocol — JSON-RPC vs gRPC vs bespoke | **closed moot 2026-08-18 — no server** |
| [Q-M1a.12](#q-m1a12--remote-access-and-auth) | Remote access and auth for `ein serve` | **closed moot 2026-08-18 — no server** |
| [Q-M1a.13](#q-m1a13--argparse-surface-parity) | Reproducing `argparse` `--help` and error text | **resolved 2026-08-18 — (b): behaviour exact, presentation normalised** |
| [Q-M1a.14](#q-m1a14--crash-parity) | Crash parity — inputs where ein.py raises an unhandled exception | **mostly resolved 2026-08-18 — ein.rs names the class** |
| [Q-M1a.15](#q-m1a15--float-formatting-parity) | Float formatting parity in reported numbers | **resolved 2026-08-18 — `pyfmt` landed** |
| [Q-M1a.16](#q-m1a16--how-does-the-harness-drive-the-lever-matrix) | How does the harness drive the `SolverConfig` lever matrix? | open — found at S1a.0.1 |
| [Q-M1a.17](#q-m1a17--win-bs-80--assumed-monotone-guards-dominate) | Win B's ≥ 80 % assumed monotone guards dominate — they are 7–16 % | **closed 2026-08-20: the mechanism is declined at a measured 1.4–2.2 % ceiling**, in [S1a.6.12](p1a.6_performance/s1a.6.12_boundary_and_snapshot.md), which took 38 % off `zebra -e` without it |
| [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) | May a fork stop re-narrating the root's fixpoint? | **resolved 2026-08-19: yes, in ein.rs only** — D3; mechanism shipped in S1a.6.10 / S1a.6.11 |
| [Q-M1a.19](#q-m1a19--how-does-a-program-state-what-it-expects) | How does a program state what it expects? | **moved 2026-08-21 with P1a.11 → [Q-M1c.1](../m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)** |
| [Q-M1a.20](#q-m1a20--what-may-an-expectation-say) | What may a `(test …)` expectation say? | **moved 2026-08-21 with P1a.11 → [Q-M1c.2](../m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)** |
| [Q-M1a.21](#q-m1a21--may-the-search-stop-before-the-lattice-is-exhausted) | May the search stop before the lattice is exhausted? | **moved 2026-08-21 with P1a.12 → [Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)** |
| [Q-M1a.22](#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate) | Is `.einb`'s id remap order-preserving enough for its own gate? | open — **measured 2026-08-20** at [S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md): a permuted id space moves 0 answers and 66 renderings |

---

## Q-M1a.1 — Port boundary: A (full) vs B (hot loop)

**Resolved 2026-08-17: A.** The placeholder deferred this; the milestone
brief settles it — ein.rs re-implements the whole stack with a 1:1
surface, and PyO3 becomes an *output* ([P1a.9](p1a.9_bindings_release/README.md))
rather than the boundary. Boundary B's advantage was preserving M1's
tooling without re-implementation; the parity harness
([design/01](design/01_parity_contract.md)) buys that back more cheaply
than an FFI seam through the hottest loop in the engine would have.

## Q-M1a.2 — Does ein.py have a sunset?

Once ein.rs is the shipping engine, ein.py is (a) the parity oracle,
(b) the reference implementation for M2 experiments, and (c) the
"Python users get a working solver" fallback. Keeping it green costs CI
time and every semantic change has to land twice.

**Recommendation: no sunset.** The oracle is what makes the port
falsifiable, and a second implementation of a research kernel is a
feature, not debt. Revisit only if double-landing becomes the dominant
cost of a semantic change — and note that M1 is *shipped*, so semantic
changes should be rare.

**Decided 2026-08-20 — yes, there is a sunset**, at the user's direction, and
the recommendation above is overruled rather than satisfied: double-landing
never became the dominant cost. [P1a.10](p1a.10_single_implementation/README.md)
is the phase, and it **shipped 2026-08-21** — six stages, `ein.py/` deleted,
`cargo test --workspace` (542 tests) the whole gate.

What the recommendation got right is the price, and it is worth restating as
the thing to watch rather than as an objection that was answered. **The oracle
is what makes the port falsifiable**, and after P1a.10 the corpus's expected
outputs are *self*-goldens: they say "ein.rs still does what ein.rs did", not
"ein.rs does what the semantics say". The evidence that this is a real cost and
not a theoretical one is
[S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md) — four genuine
parity bugs in the fuzzer's first twenty minutes, on a surface five phases of
byte parity had signed off. Nothing that replaces the oracle would have found
those.

Three things follow, and they are the phase's shape:

1. **Bank before deleting.**
   [S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md) is a
   gate, with an explicit *accepted-loss* list — the regressions that can now
   pass unnoticed. A short list is a result; an empty one is a claim to be
   suspicious of.
2. **`docs/kernel/` becomes the only statement of intent that is not also the
   implementation**, so the quirks that were defined as "whatever ein.py did"
   — [Q-M1a.3](#q-m1a3--parse-error-message-parity)'s parse positions,
   [D2](divergences.md)'s `sorted()` — have to be *stated*
   ([S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md) T1a.10.6.3).
   Undefined behaviour in a specification repo is worse than a quirk.
   **Done 2026-08-21:**
   [`docs/kernel/defined_behaviour.md`](../../docs/kernel/defined_behaviour.md)
   is thirteen of them, and enumerating them found two that are *bugs* rather
   than quirks — the binding key that drops non-string activator args
   ([Q-M1a.8](#q-m1a8--binding_key-drops-non-string-activator-args)) and the
   Python exception classes the CLI prints
   ([Q-M1a.14](#q-m1a14--crash-parity)), which are now a name with no
   referent. Neither is changed; both are checked-in expected output, and
   changing either is a decision this phase had no standing to make.
3. **[P1c.1](../m1c_external_validation/p1c.1_stdlib_conformance/README.md)
   is the partial answer** to what replaces it — it was P1a.11 until
   2026-08-21 — and the argument is why it left: an expectation written next
   to a rule is an *external* check, the only kind that gets stronger when the
   oracle goes, and that makes it
   [M1c](../m1c_external_validation/README.md)'s subject rather than the
   port's.

## Q-M1a.3 — Parse-error message parity

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §1.1–1.4](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

ein.py wraps Lark's `UnexpectedInput` as
`{file}:{line}:{col}: unexpected input\n{context}` where `context` is
`e.get_context(text)`. Observed quirks: EOF errors report `-1:-1`, and
the caret lands one past the last token
([design/04](design/04_ir_frontend.md) §4).

Options: (a) reproduce exactly, quirks included; (b) reproduce for the
non-EOF cases and accept a ledger entry for EOF; (c) improve both
implementations together, re-baselining the four `examples/broken/`
fixtures.

**Recommendation: (a) for the port, then (c) as a separate, deliberate
change once T3 is green** — improving diagnostics while the harness is
still finding bugs would hide regressions in noise.

**Resolved 2026-08-18 at [S1a.1.1](p1a.1_ir_frontend/s1a.1.1_lexer_and_parser.md):
(a), and it needed more than the EOF case.** Four behaviours had to be
reproduced, and only the first was known when this question was written:

1. **`-1:-1` at EOF**, with `get_context` rendering the last line and a
   caret one past its end — Lark's `UnexpectedEOF` sets
   `pos_in_stream = -1` and Python's negative slicing does the rest.
2. **A ±40-character context window**, applied *before* the line is
   trimmed, so an error past column 40 renders a **truncated** source
   line.
3. **The `%ignore` delayed-match quirk**: `xearley.py` writes a
   `delayed_matches[m.end()]` key at every position where whitespace or a
   comment matches — including inside a string literal, and including
   when `to_scan` is empty, which still creates the key in a
   `defaultdict`. A dict holding one empty list is truthy, so the error
   is held back until the scanner walks past it. `(y";"{` reports the
   `{`; `(y";"{?` reports the `?`. Found by the differential fuzzer, not
   by reading, and simulated in `parse::death_position`.
4. **Ambiguity resolution prefers the earlier alternative**, which is
   what makes `(rulex …)` a rule named `x` rather than a fact named
   `rulex`.

All four are pinned by `parse_parity.rs` and by 2.2 M fuzzer mutations.
The (c) half — improving the diagnostics in both implementations
together — stays open and belongs after the P1a.5 byte gate.

## Q-M1a.4 — `sorted()` over mixed-type fact args

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §2.1](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

`apriori.layer_1` does `sorted(alive)` over `(relation, args)` tuples;
if two facts of the same relation have `str` in a slot for one and `int`
for the other, CPython raises `TypeError`. `canon.state_key` deliberately
avoids this with `key=repr`; `apriori` does not.
([design/02](design/02_determinism_and_order.md) §5 H2.)

ein.rs's `Value` is totally ordered and cannot raise. So on such an
input the two implementations *must* differ: one crashes, one answers.

Options: (a) accept the divergence with a fixture pinning both
behaviours; (b) fix ein.py to sort by `repr` here, re-baselining every
affected candidate order; (c) reject such inputs at load time in both.

**Recommendation: (a)**, unless a real puzzle needs mixed slot types —
then (b), because a crash is not a semantics anyone wants to preserve.

**S1a.0.1 — reproduced, and the scope is narrower than it looked.** Blind
hypothesis generation *cannot* reach it: `hypgen._raw_candidates` builds
candidates out of `kb.names`, and `store.rebuild_indexes` only enters an
arg into that index `if isinstance(a, str)` — so every blind candidate is
all-strings. Only an `hrule` can carry a non-string through, because its
`:assert` args come from bindings. The reproducer is therefore one hrule,
one variable, and two facts binding it to `1` and to `left`:
[`examples/ein-bugs/mixed-type-hypothesis.ein`](../../examples/ein-bugs/mixed-type-hypothesis.ein),
pinned by `ein.py/tests/inference/test_mixed_type_hypothesis.py` (which
also pins the scope claim, so a future change that lets blind hypgen emit
a non-string arg re-opens this question by failing).

That strengthens (a): no puzzle without an hrule can hit this, and (b)
would re-baseline every candidate order in the corpus to fix an input
nobody has written.

**The comparator (a) needs landed at
[S1a.2.1](p1a.2_kb_core/s1a.2.1_interner_and_values.md)**:
`Terms::cmp_semantic` orders `Int < Sym < Fact` across tags, as H2
recommends, and within a tag by the interner's rank table or by numeric
value at any width. `Value` deliberately has no `Ord`, so the identity
order cannot reach a sort site by accident.

**Resolved 2026-08-18 at [S1a.4.3](p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md)
— (a), and the behaviour is now reachable rather than argued.** The
`lattice-shape` diff runs `layer_1` over every corpus file's alive set;
exactly one file diverges, exactly the predicted one, and the port
answers `[{(seat Ann 1)}, {(seat Ann left)}]` where ein.py raises. The
ledger entry is [D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
and the parity sweep **asserts** the divergence rather than tolerating
it, so a file that stopped diverging fails as loudly as one that
started.

**Re-opened in scope — not in decision — 2026-08-20 by
[S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s fuzzer.** "Two
facts of the same relation have `str` in a slot for one and `int` for the
other" is not the only way in. Two **`Fact`** arguments raise at the same
sort (`'<' not supported between instances of 'Fact' and 'Fact'`, which
design/02 § H2 named and nothing covered), and reaching that needs **no mixed
types at all** — one `(hrule … :assert (not (R ?x c)))` produces candidates
whose argument is a nested fact. No corpus puzzle had a negative hypothesis
head, which is why five phases of parity never saw it;
[`examples/ein-bugs/nested-fact-hypothesis.ein`](../../examples/ein-bugs/nested-fact-hypothesis.ein)
is the fixture now. The **decision stands at (a)** — the fix still re-baselines
every candidate order in the corpus — but the argument for it is one reason
lighter, and D2's "what would make this unacceptable" is restated
accordingly.

## Q-M1a.5 — Reproducing CPython's `shuffle`

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §3.1](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

`--shuffle` seeds `random.Random(seed)` and shuffles each layer's
candidates, carrying RNG state across layers.

Options: (a) port MT19937 seeding + `random.shuffle` +
`_randbelow_with_getrandbits` (~60 lines, table-tested against
CPython output) and keep T3 everywhere; (b) declare shuffled runs
T0-only, on the grounds that shuffle-invariance is the point.

**Recommendation: (a).** It is cheap, it is testable, and `--shuffle`
runs are exactly the ones where a silent ordering difference would be
easiest to dismiss.

**Resolved 2026-08-18 at [S1a.4.5](p1a.4_search_layer/s1a.4.5_solve_loop.md)
— (a), and it took about the size the recommendation guessed.**
`ein-infer/src/mt19937.rs` is CPython's `_randommodule.c`: the twister,
`init_by_array` seeding (absolute value, split into 32-bit words —
`Random(-7)` and `Random(7)` are the same generator), `getrandbits`
including its multi-word path, `_randbelow`'s rejection loop, and
`shuffle`'s downward Fisher–Yates.

It is checked twice. A **table** against CPython 3.14 — the first three
words for four seeds, one of them wider than a word and one negative,
plus a two-shuffle sequence that pins the state carrying across calls.
And on **real data**: `solve-shape`'s third regime runs every corpus
entry with `lattice_order_seed = 7` and compares the whole `enter`
sequence — **65 files, 5 207 enterings, 0 differences** — where the
traversal differs from the unshuffled one on 9 of the 14
`examples/branching` files, so the generator is doing something rather
than agreeing by inertia.

## Q-M1a.6 — `at None` in loader messages

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §1.5](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

Top-level `SForm`s are constructed without a `loc`
([design/04](design/04_ir_frontend.md) §3), so loader errors that
interpolate `at {form.loc}` print `at None`. ein.rs has the position and
would naturally print it.

**Recommendation: print `at None` during the port (T3), then fix both
implementations together** in a post-parity stage. Tracked here so the
fix is not forgotten; it is a genuine usability bug.

**Reproduced at [S1a.1.3](p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md)**:
`ast::loc_repr` renders `None` for a top-level form and Python's
dataclass `repr` (`Loc(file='…', line=6, col=20)`) otherwise, which is
what makes the eleven `examples/broken/load/import_*.expected` messages
byte-identical — every one of them ends `at None`. The fix, when it
comes, re-baselines all of them in both implementations at once.

**Confirmed across the whole loader at
[P1a.2](p1a.2_kb_core/README.md)**: all eighteen remaining
`examples/broken/load/` messages end `at None` too, and the one that does
*not* — `macro_arity_mismatch`, whose error is raised inside macro
expansion on a nested node — carries a real `Loc`. So the re-baseline is
exactly "every loader message except one".

## Q-M1a.7 — May `--jobs > 1` move counters?

[design/08](design/08_parallelism.md) commits to deterministic parallel
execution (same counters, same output) via speculate-and-validate, with
`--unordered` as an opt-in that relaxes to T0.

The open part is whether the validation cost is acceptable in the regimes
that matter (a large no-good store with frequent singleton writebacks).
Measure the re-validation rate in [P1a.7](p1a.7_parallelism/README.md); if
it is high, the fallback is to make `--unordered` the documented
recommendation for large searches rather than to weaken the default.

**Measured 2026-08-20 at
[S1a.7.0](p1a.7_parallelism/s1a.7.0_speculation_audit.md), before any of the
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
   [S1a.7.2](p1a.7_parallelism/s1a.7.2_parallel_enterings.md) has to settle**,
   and it is what decides whether `--jobs N` can keep the T3 promise or has to
   take a `--jobs`-scoped divergence the way
   [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) took
   one for the fork.

The **recommendation is unchanged**: no counter movement, plus `--unordered`
as the opt-in escape. What changed is that its price is now a number rather
than a hope — see
[scaling.md §3](p1a.7_parallelism/scaling.md#3-the-audit).

## Q-M1a.8 — `_binding_key` drops non-string activator args

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §3.2](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

`Saturator._binding_key` uses `plan.activator_args`, which
`compile_rule` builds as `tuple(a for a in activator.args if
isinstance(a, str))` — while the *plan cache* key stringifies **all**
args. Two activators differing only in an `int` arg therefore share a
binding key and can suppress each other's firings.

Almost certainly unintended. **Port as-is** (it is current behaviour and
T2 would flag any change), and open an ein.py issue with a fixture that
demonstrates it. Fix both together, after parity.

## Q-M1a.9 — Where do goldens live?

`ein.py/tests/golden/**` holds cross-implementation artefacts inside a
Python-specific tree ([design/11](design/11_shared_assets.md) §5). Read
in place, or promote to repo-root `testdata/golden/`?

**Recommendation: read in place until the [P1a.5](p1a.5_presentation/README.md)
gate; promote when ein.rs starts producing goldens too.**

**Answered 2026-08-21: `ein.rs/crates/<crate>/tests/golden/`** — neither in
place nor repo-root. The recommendation's trigger fired at
[S1a.6.11](p1a.6_performance/s1a.6.11_fixture_goldens.md), when ein.rs started
producing goldens of its own, and they landed beside the crate whose output
they pin rather than in a shared tree: a golden is read by exactly one test,
and `ein_corpus::golden_path(krate, name)` is the whole of the convention.
[S1a.10.2](p1a.10_single_implementation/s1a.10.2_port_the_suite.md) then
`git mv`-ed the nineteen cross-implementation artefacts into
`tests/golden/from_ein_py/` — a directory of their own, read-only, because
they are the last independent provenance in the repo and re-blessing one from
ein.rs would turn "ein.rs reproduces what the other implementation produced"
into "ein.rs agrees with itself".

[S1a.10.3](p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
is what closes it, because the corpus's `golden` group was empty *pending this
question* and an empty group is a question with a home. The group is gone.

## Q-M1a.10 — Does F11 D1 (beta-memories) land inside M1a?

[F11](../followups/f11_deductive_layer_perf.md) parks RETE beta-memories
on a fork-state design problem that [design/03](design/03_data_model.md)
§5 dissolves. [design/05](design/05_matcher.md) §7 sketches the answer,
and [P1a.6](p1a.6_performance/README.md) schedules it.

Open: whether it is still the largest lever *after* the register matcher
and the semi-naive boundary land. It may not be — those two remove the
costs that made partial-join recomputation expensive. **Decide by
profile, not by plan**; if it is a wash, revert it and leave F11 open,
exactly as P1.8a's D3 was handled.

**Answered *no*, 2026-08-19, by the profile
([S1a.6.3](p1a.6_performance/s1a.6.3_beta_memories.md)).** It is not the
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
decided at P1a.8 kickoff "informed by what M1b picks for its stack" —
and that is exactly what dissolved it. M1b picked Tauri
([M1b § Stack](../m1b_gui/README.md#stack)), whose backend is a Rust
process linking `ein-core`/`ein-ir`/`ein-infer` directly; a wire protocol
between the GUI and the engine would have been a serialisation boundary
inside one process. With M2 crossing into CPython through PyO3
([P1a.9](p1a.9_bindings_release/README.md)) and the CLI running in-process,
no consumer was left. The JSON-RPC recommendation and the rest of
`design/09` are in git history if a hosted use case ever revives them.

## Q-M1a.12 — Remote access and auth

**Closed moot 2026-08-18 with [Q-M1a.11](#q-m1a11--server-wire-protocol).**
There is nothing to expose: the engine is a library and a CLI. If hosted
use is ever wanted, the posture recorded here still holds as a starting
point — a reverse proxy plus a token in front of a purpose-built service,
never an auth system inside the engine.

## Q-M1a.13 — `argparse` surface parity

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §5](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

**Resolved 2026-08-18: (b), with (c)'s content half made binding.** ein.rs
uses `clap`; `--help` layout *and* usage-error text go on the
[normalisation list](design/01_parity_contract.md) §5. Everything a script
or a habit can depend on stays exact — the difference is presentation, and
only presentation.

T3 includes `--help` output and CLI error messages. `argparse` has a very
specific layout (usage line wrapping, `options:` heading, metavar
rendering, two-space indent) and its own error text
(`argument -n/--solutions: invalid int value: 'x'`). `clap` does not
match it and cannot be configured to.

The options were: (a) hand-roll the argument parser and the help renderer
to match `argparse` byte-for-byte; (b) use `clap` and put
`--help`/CLI-error text on the normalisation list; (c) match the
*semantics* (flags, defaults, mutual exclusion, exit codes) exactly and
accept different help text.

### What stays exact

- The three subcommands, the four `render` sub-subcommands, and the
  delegated dispatch — `ein saturate --help` prints `saturate`'s own help
  under `prog="ein saturate"`, and `saturate` still appears in
  `ein --help` though the top parser never parses it.
- Every option at every level: long name, short key, metavar, arity,
  default, `choices`, mutually-exclusive group — and its help *string*,
  which is content, not layout.
- The accept/reject verdict on every invocation, and the exit code.
- Which stream each byte goes to.

Free: wrapping, indentation, headings, ordering within a section, and the
wording of a diagnosis.

### Why not (a)

The two halves are not separable. `argparse` welds its wrapped `usage:`
block onto *every* error, so exempting the layout exempts the message —
measured 2026-08-18:

    $ ein solve examples/zebra.ein -n x
    usage: ein solve [-h] [-n N | -e] [-m MAX_SET_SIZE] [-T MAX_TIME]
                     [-E MAX_ENTERINGS] [-L] [-K] [-o {lex,score-sum}] [-y] [-z]
                     [-d SEED] [-v] [-g PROGRESS_EVERY] [-D DIR] [-c] [-H] [-t]
                     [-s] [-p] [-P] [-f] [--events FILE.jsonl]
                     [--events-level {normal,verbose}] [--json-summary FILE.json]
                     [-r FILE.md] [-G] [-F] [-R] [-l]
                     file
    ein solve: error: argument -n/--solutions: invalid int value: 'x'
    → exit 2

A byte-exact error therefore needs argparse's usage formatter, which is
most of what (a) was priced at. The middle option — reproduce the
`ein solve: error: …` diagnosis line and drop the usage block — was
offered and declined: half a formatter for a line nothing reads
mechanically.

### What replaces the byte diff

The byte comparison of `--help` was the only thing checking that ein.rs
had not silently *lost* an option, so it is replaced rather than dropped.
Both engines' help is parsed into a structure —
`{subcommand → {option → short, metavar, arity, default, choices, group,
help}}` — and the structures are diffed. On the property that matters
this is *stronger* than the byte diff: a renamed short key or a changed
default fails on its own line, instead of somewhere inside an 89-line
text blob. Same instrument shape as
[S1a.5.3](p1a.5_presentation/s1a.5.3_state_dumps.md)'s `dump-shape` —
when there is no line protocol to diff over, render one.

### What would make this unacceptable

A consumer that reads `ein --help` or matches on ein's stderr text. There
is none as of 2026-08-18: no script under `utils/` parses either, and
`feature_matrix.py` only *echoes* a failing child's stderr into a report
field. The day one is written, this is the entry to revisit.

## Q-M1a.14 — Crash parity

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §4](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

Some inputs make ein.py raise an unhandled exception (Q-M1a.4's
`TypeError`; a `KeyError` from an unbound `:assert` var is *caught*
nowhere and surfaces as a traceback). ein.rs will not have Python
tracebacks.

Proposal: the harness compares **exit code + the first line of stderr**
for crash cases and records them as a distinct corpus group
(`crash-parity`), with the traceback body normalised away. Any input in
that group is also a candidate ein.py bug report.

**S1a.0.1 — the first-stderr-line half is wrong; implemented as exit code
+ exception class.** The first `crash-parity` fixture (Q-M1a.4's
`mixed-type-hypothesis.ein`) raises `TypeError: '<' not supported between
instances of 'int' and 'str'` — and *which operand is named first*
depends on the `frozenset` iteration order inside `sorted`, so ein.py
alternates between two messages across `PYTHONHASHSEED` values. A rule
that compares that line makes the determinism sweep fail on a difference
that is not one. `tier::compare_crash` therefore takes the exception
class off the last traceback line and drops the message body.

**A second fixture, from the CLI surface itself (found 2026-08-18, while
resolving Q-M1a.13).** A missing input file is not an argument error:
`cli/_common._parse_or_exit` and `cli/solve._timed_load` both call
`Path.read_text` unguarded, so `ein solve /nope.ein` is a
`FileNotFoundError` traceback and exit 1 — not the clean message
[S1a.5.4](p1a.5_presentation/s1a.5.4_cli.md) originally listed among its
argument errors. It belongs to this group instead, and it sharpens the
open half below: the first fixture needs a mixed-type puzzle, this one
needs a typo.

**S1a.5.4 — the open half, answered for every path the CLI reaches: name the
class.** ein.rs now prints CPython's own last line, so the comparison passes
on the whole line rather than only on the class it extracts:

- a missing input file → `FileNotFoundError: [Errno 2] No such file or
  directory: '<path>'`, exit 1;
- a `CompileError` out of `solve` or `saturate` →
  `ein.inference.compile.CompileError: <message>`, exit 1 — the *message*
  was already at parity from P1a.3, so naming the class was the whole gap.

That was 6 of the 7 `crash-parity` cells; the seventh was D2. **2026-08-20
added three more, all from [S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s
fuzzer**: `nested-fact-hypothesis.ein` (D2's second shape — it diverges),
`unbound-relation-head.ein` (`(?R ?x)` with `?R` unbound, which **passes** —
identical message, identical exit code, and only ein.py's traceback wrapper
around it) and `unbound-assert-var.ein`, which is **this question's own first
example**: "a `KeyError` from an unbound `:assert` var is *caught* nowhere".
Nothing in the corpus had ever reached it, and reaching it found the one gap
left in the answer above — ein.rs printed the message alone where CPython
prints `KeyError: "…"`, because `KeyError`'s `str` is the *repr* of its key.
Fixed at the printer; the two other firing errors (`TypeError` for a
non-fact `:assert` head, `SaturatorStepLimitError` for the step budget) were
given their class names in the same edit, before an input reaches them. Naming a Python class from Rust is not a category error
here: the class is the *oracle's* observable, and reproducing it is what I1
asks for. What stays open is the
narrower question the relaxation would answer — whether a future ein.rs-only
error, with no Python counterpart to name, joins this group or a new one.
Nothing in the corpus reaches one.

## Q-M1a.15 — Float formatting parity

> **Now stated as ein's own defined behaviour** — [`docs/kernel/defined_behaviour.md` §2.3](../../docs/kernel/defined_behaviour.md) (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)). This entry is the *decision*; that page is what the engine now promises.

Several reported numbers are formatted floats — `--hyp-stats`'s
`{100.0 * n / total:>5.1f}` percentages, `--timing`'s `{ms:9.2f}` (whose
*values* are normalised away, but whose *widths* are not), and
`--stats`' `{elapsed_ms:.1f}`. Rust's `{:.1}` and Python's `%.1f` agree
on round-half-to-even for `f64`, but the two differ on `-0.0`, on `inf`
/ `nan` spellings, and on very large magnitudes.

Proposal: a `pyfmt` helper beside `pyrepr`
([design/02](design/02_determinism_and_order.md) §7) covering `f`-format
with width/precision, differentially tested over a wide float corpus.
Small, and it removes a whole class of one-character T3 diffs.

**Resolved 2026-08-18 at [S1a.1.2](p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md):
`ein-core::pyfmt`**, covering `[[fill]align][sign][0][width][.precision]f`
and rejecting anything outside that subset rather than guessing at it.
230 values × 19 specs against CPython, 0 differences. Three findings
beyond the proposal: Rust spells NaN `NaN`; a NaN never carries a sign in
CPython while an infinity does; and an **empty** spec is `str(x)`, not
`.6f` — so the `f` is required, not assumed. The digits themselves come
from Rust's `{:.*}`, which agrees with Python on round-half-even over the
exact binary value.

`pyrepr` landed with it, and needed a **generated Unicode table**:
`repr()` escapes by general category and `rustc` exposes only
`is_control()`, so `printable.rs` is generated from CPython's own tables
by `utils/gen_unicode_printable.py` (737 ranges, Unicode 16.0.0). A
CPython upgrade that moves a category surfaces as a named code point in
the differential test rather than as a mystery diff at P1a.5.

## Q-M1a.16 — How does the harness drive the lever matrix?

[design/01](design/01_parity_contract.md) §4 puts "each `SolverConfig` lever
flipped off (the same matrix [`utils/feature_matrix.py`](../../utils/feature_matrix.py)
already drives)" in the corpus run matrix. Building the manifest at
[S1a.0.1](p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md)
found that only **four of the ten** are reachable from the CLI — `-L`
(`enable_pre_branch_lookahead`), `-K` (`enable_lookahead_kill_cache`), `-y`
(`lattice_sanity_check`) and `-o score-sum` (`lattice_order`). The other six —
`enable_path_nogoods`, `enable_symmetric_mirror`, `enable_singleton_writeback`,
`enable_forced_positive`, `enable_fail_fast_fork`, `hypgen_scoring` — exist only
as a Python kwarg or a puzzle's own `(config …)` block. `feature_matrix.py`
reaches them because it imports the engine; the harness shells out, so it
cannot.

That matters more than it looks: those six gate exactly the optimisations
[P1a.6](p1a.6_performance/README.md) will re-implement, and "lever off" is the
cheapest way to isolate a parity failure to one of them.

**Restated 2026-08-21 (S1a.10.3).** The harness is gone and the `levers` column
outlived it: the sweep runs the four CLI-reachable levers as extra `solve`
cells exactly as before. But the reason for the question moved. There is no
parity failure to isolate any more, and the six unreachable levers are now
reachable in the *only* way that matters — `ein-infer`'s tests construct a
`SolverConfig` directly, which is how `search_semantics.rs` and
`lattice_semantics.rs` already exercise them. What the CLI still cannot do is
put a lever into a **corpus** cell, so what the question is asking has narrowed
to "should the corpus sweep cover the other six?" — a coverage question about
one test, not a gap in the gate.

Options:

- **(a) add `--config KEY=VALUE` to both CLIs** (repeatable, kebab-cased,
  parsed by the same coercer `(config …)` uses). Additive, ~20 lines, makes
  `levers = "all"` real. Costs: one more flag on the T3 surface both
  implementations must match, and a way to set a lever that a puzzle file
  cannot audit.
- **(b) generate per-lever puzzle variants** — copy the fixture with a
  `(config …)` block appended into a temp dir. No CLI change, but the corpus
  entry is then not the file in `examples/`, which weakens the "both
  implementations read the same bytes" guarantee for those runs.
- **(c) leave the six unexercised** and note the gap. The corpus keeps four
  levers; the other six are covered only by ein.py's own test suite.

**Recommendation: (a)**, decided before [P1a.6](p1a.6_performance/README.md)
rather than at it — the flag has to exist in *both* implementations, so it is
cheapest to add while the Rust CLI is still a stub. Until then the manifest's
`levers` lists the four, and `corpus/README.md` says why.

## Q-M1a.17 — Win B's ≥ 80 % assumed monotone guards dominate

**Found 2026-08-18 at [S1a.3.4](p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md),
by measurement rather than by argument.**

[design/06](design/06_saturation.md) § Win B projects that guard sub-plan
**evaluations** drop by ≥ 80 %, and names the mechanism: a *monotone*
guard's query is purely positive, so if it found nothing at round *r* it
can only start finding something through a fact added since — which is
`run_seeded` on the guard's sub-plan, restricted to Δ ∩ watched.

The port instruments that split (`Saturator::guard_evals` /
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
The two refinements that do not depend on monotonicity **landed** and are
T2-green: the per-round `(guard, projected env) → verdict` memo, and an
allocation-free watch stamp on an ordered parked set instead of a
pop-and-re-push heap. Together they moved the boundary by ~2 % at root
scale, which is the honest number.

~~**Open:** does the *exhaustive* mix differ?~~ **Answered 2026-08-19, by
running it** — the recommendation below said the question would be settled that
way and it was, at the head of
[S1a.6.12](p1a.6_performance/s1a.6.12_boundary_and_snapshot.md). `guard_eval`
and `guard_eval_monotone` are the `Saturator`'s pair summed over every fork of a
solve, which is the aggregation no per-saturation field could do
([baseline.md §17](p1a.6_performance/baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it)):

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
boundary stays available at [T1a.6.12.4](p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6124--the-semi-naive-guard-re-evaluation-at-its-measured-reach)
if the days are there; it is last in the stage and expected not to run.

**Closed 2026-08-20. It did not run, and the ceiling is now measured on both
sides of its product** rather than on one
([baseline.md §18](p1a.6_performance/baseline.md#18-s1a612--the-boundary-and-the-premise-that-had-nothing-left-to-bind)):

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
[T1a.6.12.1](p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6121--visit-what-changed-not-everything).

**The recommendation that got here, kept for the record:** carry the semi-naive
guard re-evaluation (T1a.3.4.5) into [P1a.6](p1a.6_performance/README.md) as a
*measured* optimisation with this as its trigger condition, rather than landing
a mechanism here whose measured reach is a tenth of its stated one. If the
exhaustive mix is monotone-dominated, it lands there with a number; if it is
not, the boundary needs a different idea and this question is where that gets
decided.

---

## Q-M1a.18 — May a fork stop re-narrating the root's fixpoint?

**Resolved 2026-08-19: yes, in ein.rs only.** `Saturator::resume` is ein.rs's
shipping path; ein.py keeps the fresh saturator and the narration divergence is
permanent, recorded as
[D3](divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it).

The principle that moved with it, and it is bigger than this question: **the
parity contract's hard requirement is that the two engines produce identical
final solutions**, not identical bytes. P1a.1–P1a.5 built ein.rs equal to
ein.py byte for byte, which is what made the port falsifiable; from P1a.6 on,
byte-identical *narration* is a means that has served its purpose, and ein.rs's
own regression coverage moves to checked-in fixtures
([S1a.6.11](p1a.6_performance/s1a.6.11_fixture_goldens.md)) rather than to the
oracle. T0 and T1 stay exact, and are now compared *more* carefully than
before. [S1a.6.10](p1a.6_performance/s1a.6.10_parity_contract.md) is the
mechanism.

Two things decided it, and neither was the speed. **The answers do not move** —
3.2 M enterings compared fact by fact, every verdict, model, unsat core and all
85 `summary.json` counters identical. And the trace gets *better*: it now opens
with root's own derivations, then `Assuming …`, then what the hypothesis adds,
which is how [`zebra_walkthrough.md`](../../docs/kernel/inference/zebra_walkthrough.md)
tells it and what the accidental re-derivation was standing in for.

The cost accepted, stated plainly: **267 529 facts record a different — equally
valid — one of their derivations as the primary**, because a resumed fork
inherits root's parked candidates with root's tiebreakers and the NAF boundary
admits one per round. The engine never promised *which* derivation of a
multiply-derivable fact it records first; it does now promise less than it
happened to deliver.

**Both halves landed the same day.** S1a.6.10 wrote the rule once — a fork's
derivation, and anything keyed on a dying fork's stopping point, is narration —
into [design/01 §5](design/01_parity_contract.md#the-fork-row-stated-once) and
into one crate, `ein.rs/crates/ein-parity`, replacing the six ad-hoc cuts
S1a.6.9 had left behind; the harness went back to **T3 472/473 and T2 239/240**
with [D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
the only differing cell in either, and `--strict` keeps the byte-identical
contract one flag away for the determinism sweep. S1a.6.11 replaced the elided
bytes with twelve ein.rs goldens and ported idea-08's walkthrough-rule
assertion to the engine that ships. **This question is closed.**

---

*What follows is the question as it stood, with the evidence T1a.6.9.2/3
produced. Kept because the reasoning is the record.*

**Found 2026-08-18 at [S1a.6.9](p1a.6_performance/s1a.6.9_fork_entry_delta.md),
by measurement.** The numbers are in
[baseline.md §9](p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation).

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
[baseline.md §11](p1a.6_performance/baseline.md#11-the-resumed-fork-saturator-measured);
the rendered before/after this question was to be decided against is
[fork_delta_trace.md](p1a.6_performance/fork_delta_trace.md).

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

- **T2** loses 62.5 % / 75.9 % of its lines at `verbose` — **and 58.8 % /
  74.2 % at `normal`**, which the question did not anticipate: a redundant
  firing is not emitted at `normal`, but the ~1 790 `enqueue` lines per
  entering that produce it are;
- **T3** moves `n_firings` in `--trace`, the `("firings", len)` counts in
  `--dump-states`, the *first five firings* `render/shape.rs` prints — and,
  for the 90 002 facts above, **which rule the proof names**;
- **T0/T1** do not move: `BaseStats` never counts a firing, and every counter
  it does keep is unchanged;
- **`ein.py/tests/trace/test_idea08_acceptance.py::test_zebra2_fires_walkthrough_rules`
  fails**: the solution's trace covers 12 rules instead of 24 and `symmetric`
  is not among them, because `symmetric` closes `next-to` at *root*. `--trace`
  gets root's whole closure for free today, by accident, because every fork
  re-derives it. Adopting this means the trace has to render root saturation
  as its own section — which is what a human walkthrough does anyway, and is a
  change to the renderer rather than to the engine.

**The options.**

- **(a) No.** I1 is the milestone's spine and the trace is an observable.
  The win is then taken only where it is invisible:
  [S1a.6.8](p1a.6_performance/s1a.6.8_compile_cache_and_extents.md) for the
  compile share and [S1a.6.3](p1a.6_performance/s1a.6.3_beta_memories.md)'s
  *root* beta-memories for the match share — same firings, same order,
  discovered by lookup instead of by rescanning.
- **(b) Yes, in both engines.** ein.py changes first, ein.rs follows, and the
  change is recorded in [divergences.md](divergences.md) as a *joint* change
  rather than a divergence. There are no goldens to regenerate — the harness
  diffs two live engines — so "both or neither" is literal. This is a change
  to the M1 engine, so it is a followup and a new stage, never a retrofit into
  a shipped phase. **It now also buys a renderer change** (root saturation as
  its own trace section) and a re-baselined `test_idea08_acceptance`, and it
  accepts that a fact's recorded proof may name a different one of its valid
  derivations.
- **(c) Yes, behind a flag** that is off in the parity build. Keeps I1 and
  gets the speed for the M1b/M2 consumers — at the cost of a second code
  path through the saturator's most delicate ordering, which is exactly
  what P1a.6 Rule 3 (a wash is a revert) exists to discourage. The flag is
  **already built** (`fork-delta` + `EIN_FORK_DELTA`), so this option is the
  cheapest to take and the only one that is currently *true* of the tree.

**Recommendation: still (b), and the argument is still not primarily speed —
but it is now a bigger change than it looked.** A fork's firing list is what
[`08-human-style-deductive-trace`](../ideas/08-human-style-deductive-trace.md)
renders, and 961 re-derivations of what was already true before the
hypothesis is noise in it — the human walkthrough in
[`zebra_walkthrough.md`](../../docs/kernel/inference/zebra_walkthrough.md)
narrates what a hypothesis *adds*, and the rendered after-trace does exactly
that from its second step. The speed is a consequence, and it happens to be
the consequence that closes the milestone's last open target.

What the evidence adds to the recommendation is the **cost line**: the M1
engine would stop promising *which* derivation of a multiply-derivable fact
it records first. That promise is not written down anywhere as a guarantee —
`record_justification`'s contract is "first derivation wins", and which
derivation is first was already a function of priority, FIFO order and the
boundary's one-admission-per-round rule — but 90 002 facts is not a rounding
error, and a proof that names `injective-negative` where it used to name
`functional-negative` is a *different explanation of the same fact*, not a
shorter one.

**So the question is now sharper than "may the trace get shorter":** may the
M1 engine record a different one of a fact's valid derivations as its primary,
in exchange for a trace that starts at the hypothesis and a solver that meets
its last target? **(a)** says no and takes the invisible half through
[S1a.6.3](p1a.6_performance/s1a.6.3_beta_memories.md). **(c)** says not on the
parity build.

**Decided: a fourth option, (d) — yes in ein.rs, and the contract relaxes with
it.** Not (b): ein.py does not follow, so the two engines narrate different
amounts of the same derivation, permanently. Not (c): the flag is not the
shipping configuration, it is [D3](divergences.md)'s fixture. The evidence is
[baseline.md §11](p1a.6_performance/baseline.md#11-the-resumed-fork-saturator-measured)
and [fork_delta_trace.md](p1a.6_performance/fork_delta_trace.md); what it cost
the harness — 7 T3 cells, 97 T2 — is the specification of
[S1a.6.10](p1a.6_performance/s1a.6.10_parity_contract.md).

## Q-M1a.19 — How does a program state what it expects?

**Moved 2026-08-21 to [Q-M1c.1](../m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)**,
with the phase that raised it: P1a.11 is now
[P1c.1](../m1c_external_validation/p1c.1_stdlib_conformance/README.md) in
[M1c](../m1c_external_validation/README.md). The text went unchanged apart
from ids and paths. **The id stays reserved and is never reused.**

## Q-M1a.20 — What may an expectation say?

**Moved 2026-08-21 to [Q-M1c.2](../m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)**,
with Q-M1a.19 and for the same reason — the two are one decision and
[S1c.1.2](../m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md)
still settles both.

## Q-M1a.21 — May the search stop before the lattice is exhausted?

**Moved 2026-08-21 to [Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)**,
with P1a.12, which is now
[P1d.1](../m1d_satisfiability/p1d.1_exhaustive_search/README.md) in
[M1d](../m1d_satisfiability/README.md) — the milestone that exists to answer
it. What the move adds is a fourth candidate this framing did not have: a
state that can report what it still *owes*
([P1d.2](../m1d_satisfiability/p1d.2_obligations/README.md)).

## Q-M1a.22 — Is `.einb`'s id remap order-preserving enough for its own gate?

[P1a.8](p1a.8_binary_container/README.md)'s gate is `ein solve x.einb`
**byte-identical** to `ein solve x.ein`, and
[design/10 §3](design/10_binary_format.md#3-ids-across-the-boundary) answers
the id problem with a translation table plus a fast path: "when the live
interner is empty … both tables are the identity and the pass is skipped
entirely. That is the mmap-and-go case."

[S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md)'s
determinism successor measured what happens when they are *not* the identity.
Over 2 544 `(file, op)` pairs with the id space permuted:

- **0 answers move** — verdict, model, counters, every non-derivation
  rendering, at 1 and at 8 seeds;
- **66 renderings move**, and all of them are what
  [D3](divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
  already calls narration: 44 where `enable_fail_fast_fork` stopped a dying
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
[S1a.8.1](p1a.8_binary_container/README.md).
