# M1a — divergence ledger

Differences between `ein` (Python, the oracle) and `ein.rs` that are
**accepted** rather than fixed. Empty is the goal at the
[P1a.5](p1a.5_presentation/README.md) byte gate; a non-empty ledger is
allowed only with a written reason per entry.

The precedent for this shape is
[`docs/kernel/inference/parity_baselines.md`](../../docs/kernel/inference/parity_baselines.md),
which recorded the tree-vs-monotonic divergences explicitly rather than
treating them as failures or hiding them behind `xfail`. The difference
is the standard: that comparison was between two *different engines*;
this one is between an engine and its port, so the bar for an entry is
much higher.

## Rules

1. An entry needs **"what would make this unacceptable"** — a stated
   condition under which it becomes a bug. An entry without one is not a
   decision, it is a shrug.
2. An entry needs a **fixture** in the corpus that demonstrates it, so it
   cannot silently widen.
3. Anything on the [normalisation list](design/01_parity_contract.md) §5
   is *not* a divergence — that list is closed and lives in the design
   doc. Adding to it requires an [open question](open_questions.md).
4. When an entry is fixed, keep it with `**Status:** fixed in <stage>`.
   The trail is the memory.

## Template

```markdown
### D<n> — <one-line title>

**Found:** <date>, <phase/stage>
**Tier:** T<k>
**Status:** accepted | fixed in S1a.<p>.<s>
**Fixture:** <corpus entry>

**What.** <the observable difference, both sides quoted>

**Why it is acceptable.** <argument>

**What would make it unacceptable.** <the condition>
```

## Entries

### D1 — a rule may not bind more than 256 variables

**Found:** 2026-08-18, [S1a.3.1](p1a.3_deductive_core/s1a.3.1_compiler.md)
**Tier:** T2 (it would surface as a compile refusal, hence a firing difference)
**Status:** accepted
**Fixture:** `ein.rs/crates/ein-infer/tests/compile_limits.rs` — built in the
test, **not** checked into `examples/`: a corpus file ein.py compiles and
ein.rs refuses would fail the corpus parity test, which is the alarm this
ledger wants left armed for divergences nobody chose.

**What.** A `:match` binding more than `MAX_REGS` = 256 distinct variables
compiles in ein.py and is a `CompileError` in ein.rs:

```
more than 256 distinct variables in one `:match` — ein.rs numbers a rule's
variables into a fixed register file (256 slots) so the matcher's inner loop
allocates nothing. Split the rule.
```

**Why it is acceptable.** ein.py's bindings are a `dict`, so it has no bound to
port; ein.rs resolves every variable to a register in a fixed-size file,
because that is what makes the inner loop allocation-free
([design/05](design/05_matcher.md) §3 — the change the whole matcher rewrite is
for). A ceiling therefore exists by construction, and the only question is
where. 256 is **42×** the widest rule anything in the corpus compiles
(`domain-elimination`, 6 registers), and the overflow is a typed error with a
remedy rather than a panic or a silent truncation.

**What would make this unacceptable.** A rule anyone actually writes coming
within 8× of the ceiling. `compile_limits.rs::the_corpus_is_nowhere_near_the_ceiling`
measures that distance on every corpus file and fails when it closes, so the
condition is checked rather than remembered.

### D2 — `sorted(alive)` raises in ein.py where ein.rs answers

**Found:** 2026-08-18, [S1a.4.3](p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md)
— predicted at S1a.0.1 as
[design/02](design/02_determinism_and_order.md) § H2 and
[Q-M1a.4](open_questions.md#q-m1a4--sorted-over-mixed-type-fact-args), and
reached by the first op that runs the layer arithmetic.
**Tier:** T1 (a search-layer counter; T0 as a crash)
**Status:** accepted
**Fixtures:** [`examples/ein-bugs/mixed-type-hypothesis.ein`](../../examples/ein-bugs/mixed-type-hypothesis.ein)
and, since 2026-08-20,
[`examples/ein-bugs/nested-fact-hypothesis.ein`](../../examples/ein-bugs/nested-fact-hypothesis.ein)
— both in the `crash-parity` group, pinned on the Python side by
`ein.py/tests/inference/test_mixed_type_hypothesis.py` and on the Rust side by
`hypgen_parity.rs`'s `divergent` list and the two `DIVERGENT` consts in
`dump_parity.rs` / `trace_parity.rs` — which **assert** the divergence, so a
file that stopped diverging would fail as loudly as one that started.

**What.** `apriori.layer_1` opens the search with `sorted(alive)` over
`(relation_name, args)` tuples. Two candidates of one relation whose slot *i*
holds a `str` in one and an `int` in the other are incomparable:

```
TypeError: '<' not supported between instances of 'str' and 'int'
```

ein.rs orders `Int < Sym < Fact` by tag and answers:

```
LAYER1 [{(seat Ann 1)}, {(seat Ann left)}]
```

**A second shape, found 2026-08-20 by
[S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s fuzzer.** The
same `sorted(alive)`, over two **`Fact`** arguments rather than a `str` and an
`int`:

```
TypeError: '<' not supported between instances of 'Fact' and 'Fact'
```

design/02 § H2 named it in one clause — "and a `Fact` has no `__lt__` at all" —
but no fixture covered it, because it takes an hrule whose `:assert` head is a
`(not …)`, and **no corpus puzzle has one**. That is the part worth writing
down: the shape needs *no mixed slot types*. One hrule, one negative
conclusion, two candidates, and the search-layer sort raises.

**Why it is acceptable.** Three reasons, in order of weight.

1. **Nothing can reach it without an `hrule`.** Blind hypgen builds candidates
   out of `kb.names`, and `rebuild_indexes` only enters an argument there
   `if isinstance(a, str)`, so every blind candidate is all-strings. Only an
   hrule carries a non-string through, because its `:assert` args come from
   bindings. That scope claim is itself a test
   (`test_blind_hypgen_cannot_produce_a_non_string_arg`), so a change that
   widened it would re-open this rather than quietly extend it.
2. **A crash is not semantics anyone wants preserved.** Reproducing it would
   mean making the port fail on an input it can answer, and answering is the
   behaviour a user would ask for if asked.
3. **The alternative costs the whole corpus.** Fixing ein.py to sort by `repr`
   here — as `canon.state_key` already does — changes the candidate order of
   every puzzle and re-baselines every T2 golden, to buy an input nobody has
   written.

The order ein.rs picks is not arbitrary: `Terms::cmp_semantic` agrees with
Python's `sorted` on every pair Python *can* compare, and the cross-tag order
is only consulted where Python raises.

**What would make this unacceptable.** A real puzzle wanting mixed-type slots —
at which point option (b) of Q-M1a.4 becomes right and ein.py is fixed first,
both ports moving together. ~~The trigger is visible: this is the only entry in
`crash-parity` that is a *search-layer* crash, and a second one would mean the
scope claim above is wrong.~~

**The trigger half-fired, 2026-08-20.** There is now a second search-layer
`crash-parity` entry, and it says something narrower than "the scope claim is
wrong": claim 1 (**an `hrule` is necessary**) survives untouched — the new
shape uses one — but the *reading* of claim 3 does not. "An input nobody has
written" was true of mixed slot types and is not true of
`:assert (not (R ?x c))`, which is ordinary ein. What has not changed is the
price of fixing it: sorting `alive` by a total key re-baselines the candidate
order of every puzzle in the corpus. So this stays **accepted**, with the
acceptability now resting on claims 1 and 2 rather than on all three, and with
the real trigger restated: **the first puzzle that wants a negative hypothesis
head and is not willing to crash.**


### D3 — a fork resumes root's saturation; ein.py re-derives it

**Found:** 2026-08-19, [S1a.6.9](p1a.6_performance/s1a.6.9_fork_entry_delta.md)
**Tier:** T2 and T3 — and as of
[S1a.6.10](p1a.6_performance/s1a.6.10_parity_contract.md) **neither reports
it**: the normalisation is on [design/01 §5](design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list)'s
list, T3 is 472/473 and T2 239/240 with [D2](#d2--sortedalive-raises-in-einpy-where-einrs-answers)
the only differing cell in both. Before that stage it cost 97 cells of 240 at
T2 and 7 of 473 at T3.
**T0 and T1 do not move at all**; see below, that is the whole argument.
**Status:** accepted
**Fixture:** `utils/fork_delta_verify.py` against a
`cargo build --features fork-delta --target-dir target-fd` build, which
compiles in the way back to the old path (`EIN_FORK_DELTA=0`) so both arms
come out of one binary. Not a corpus entry: the divergence is not a property
of any *input*, it is a property of every solve with more than one entering,
so the fixture that keeps it from widening is the differ, not a file. Since
[S1a.6.11](p1a.6_performance/s1a.6.11_fixture_goldens.md) there are twelve
more, of the other kind: ein.rs goldens over the trace, the `slice` cone, a
fork's own `enterings/` dump, the snapshot projection and the event stream —
everything this entry took out of the cross-engine diff, compared against
checked-in bytes instead. `utils/mutant_ein.py` is the negative control: it
deletes one productive firing from the shipping binary's event log, which the
relaxed gate must still catch.

**What.** `commitment::try_commitment_set` forks the saturated root. ein.py
builds a fresh `Saturator` there, whose first enqueue pass is a FULL pass, so
the fork re-derives root's entire deductive closure as `redundant` firings
before doing any work of its own — 94.6 % of a fork's firings on `zebra -e`
([baseline.md §9](p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation)).
ein.rs resumes root's saturation instead: the plan list in its order, `fired`,
`seen`, the candidate arena and the parked set with its watch stamps are
inherited, and the delta is what the fork has that root's snapshot did not.

Three observables follow.

1. **Fewer firings.** `zebra2 -e` 38 136 → 9 834, `zebra -e` 113 746 → 26 656.
   The T2 stream loses 62.5 % / 75.9 % of its lines at `verbose` and 58.8 % /
   74.2 % at `normal` — the second because the ~1 790 `enqueue` lines an
   entering emits go with the firings that produced them.
2. **A different one of a fact's valid derivations is recorded first.**
   267 529 facts across the corpus (17 on `zebra2 -e`, 198 on `zebra -e`;
   nine tenths of the total is six transitive-symmetric closure fixtures
   where most facts have many equally valid derivations). A fresh fork renumbers root's parked
   candidates in plan order; a resumed one inherits root's tiebreakers, so
   they sort first — and the NAF boundary admits at most one candidate per
   round, so a fact derivable two ways (`functional-negative` /
   `injective-negative`, `domain-elimination` / `range-elimination`,
   `total` / `surjective`) gets a different **first** derivation, and first
   derivation wins.
3. **A different partial state for a fork that dies.** `enable_fail_fast_fork`
   stops a dying fork at the firing that kills it, so a different firing order
   leaves a different prefix: 2 067 dead forks' `state_key`s, and 110 unsat
   cores that are each still a correct minimal frontier of the same conflict. With
   fail-fast **off** — every fork run to its fixpoint — all three of those go
   to **zero**.

**Why it is acceptable.** Because the answer does not move, and that was
made a measurement rather than an argument. `utils/fork_delta_verify.py` runs
one binary twice over every `solve`-family run of every `positive` and
`stdlib` corpus entry, comparing per entering: the fork's fact set fact by
fact, every recorded justification of every fact, the `kind` and the unsat
core; and per run: stdout, `summary.json`, and the `--dump-states` tree.

- **the verdict, `k`, the models, the query bindings and the printed unsat
  core** — identical on every entry, including the twelve whose verdict is
  *no solution* and whose answer therefore **is** the core;
- **`summary.json`** — all 85 fields, i.e. **T0 and T1 in full**: the same
  enterings, the same layers, the same saturations, the same merges, the same
  learned clauses. The two engines run the same search;
- **every alive fork's fixpoint**, fact for fact, over **3 228 853**
  enterings — and with fail-fast off, every dead fork's too, across another
  3 170 461.

What changes is how much of that derivation each engine *narrates*, and this
milestone's byte-level narration parity was a means to the answer, not the
end. [Q-M1a.18](open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
records the decision and the alternatives.

The trace does **not** lose the derivations, which was the near-miss: rendering
only the solution node's firings used to pick up root's whole closure by
accident, so removing the re-derivation dropped `symmetric` — a rule that fires
only at root — out of the proof entirely, and
`ein.py/tests/trace/test_idea08_acceptance.py::test_zebra2_fires_walkthrough_rules`
is the test that catches exactly that. ein.rs's `--trace` therefore gained a
**"Before any assumption"** section: 321 unconditional steps, then
`Assuming …`, then the 240 the hypothesis adds. Same 24 rules as before,
arranged the way `zebra_walkthrough.md` tells it.

**Where it shows, and what was done about it.** S1a.6.9 shipped with six
separate cuts, each made as the next test went red:

| gate | cells | the cut S1a.6.9 made |
|---|---|---|
| `ein-conformance --tier T3` | 7 of 473 | none |
| `ein-conformance --tier T2` | 97 of 240 | none |
| `ein-render` `dot_parity` | the `slice` view, 16 entries | `NARRATION`: run on both sides, both must answer, not byte-compared |
| `ein-infer` `hypgen_parity` | the three `solve-shape` sweeps | `Compare::IgnoringForkNarration`: blanks `firings=` / `"n_firings"` / the event ordinal `"n"`, and a **`dead-post`** entering's core |
| `ein-render` `dump_parity` | 79 of 325 dumps | four, in `dump_shape` and its `ir_oracle.py` twin: the timeline's `"firings"`, the whole `enterings/` subtree (file set only), the snapshot's `deads` (count only) and its two lattice DOTs (presence only) |
| `ein-render` `trace_parity` | 86 of 195 | `NARRATION_BLOCKS`: the `--- markdown`, `--- ir` and `--- ir-reparsed` bodies, and the whole `no-proof` mode, which is one rendered trace and nothing else. `--- answer`, `--- table` and `--- round-trip` stay exact |

Read downwards the chain is one sentence — **a fork's derivation, and anything
keyed on a dying fork's stopping point, is narration** — and
[S1a.6.10](p1a.6_performance/s1a.6.10_parity_contract.md) is where it became
one rule instead of six tolerances: written in
[design/01 §5](design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list),
implemented once in `ein.rs/crates/ein-parity`, and applied by the harness and
by all four tests. **This entry is therefore no longer a set of failing
cells** — T3 is 472/473 and T2 239/240, with D2 the only difference in either
— and what each cut cost before that is the table above.

`--strict` (`EIN_PARITY_STRICT=1`) puts every one of them back, which is how
the determinism sweep still runs on the unrelaxed contract.

Note where the chain *stops*: `--- answer`, `--- table`, `summary.json`,
stdout, every state dump outside `enterings/`, and the round-trip property are
all still byte-compared, and none of them moved. S1a.6.10 also **narrowed** two
of the six on the way past: the T2 stream is compared for the multiset of
facts each `enter`-delimited segment derived and the set of rules that derived
them rather than not at all, and the snapshot's dead state keys and lattice
DOTs are now rendered in full by both sides and elided at comparison time, so
`--strict` sees them.

`commit-shape` is untouched and still compares its `firings=` exactly: it
calls `try_commitment_set` without a snapshot, so it does not take the resumed
path. That is not an oversight — it is the control.

**One more observable moved than S1a.6.9 recorded, and S1a.6.10 found it by
measuring rather than by reading.** `compile` events: a plan-memo *miss* is
emitted once per enqueue pass that needs the rule, so a fresh fork that
re-derives root's closure misses where a resumed one does not — 244 against
128 on `examples/branching/02_one_dead_one_alive.ein`'s plain `solve`. The
*distinct* compiles are identical on both sides, rule for rule and activator
for activator; only how many times each was reached moves. It is on the elided
list for the same reason as the rest.

**The unsat core moves too, and this one is on the list above.** Found
2026-08-20 by [S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s
fuzzer, minimised to 11 forms
(`conformance/fuzz_findings/`, and reproduced here):

```lisp
(relation r0 T T) (relation r1 T T) (relation is-a T T)
(is-a o2 T) (is-a o3 T) (r0 o2 o3)
(rule fire-0 (?P) :match  (and (r1 ?v0 ?v1) (r0 ?v1 ?v0))
                  :assert (not (r1 ?v0 ?v0)))
(fire-0 T)
(rule fire-1 () :match  (and (is-a ?v0 T) (r0 ?v0 ?v1) (r1 ?v2 ?v3)
                             (absent (not (r1 o3 ?v1))))
                :assert (r1 ?v2 ?v2))
```

`solve --max-set-size 2`: both engines answer `Contradiction`, `k = 0`, with
**every counter identical** — 15 enterings, 13 alive, 2 dead, 2 no-goods, 2
layers — and different **unsat cores**:

| | core |
|---|---|
| ein.py | `(is-a o2 T) (is-a o3 T) (r0 o2 o3) (r0 o3 o2) (r1 o2 o3) (r1 o3 o2)` — 6 |
| ein.rs | `(is-a o3 T) (r0 o3 o2) (r1 o2 o3) (r1 o3 o2)` — **4, a strict subset** |

**It is this divergence and not another**, measured rather than argued:
`EIN_FORK_DELTA=0` on the `fork-delta` build reproduces ein.py's six facts
exactly. Observation 3 below predicted the mechanism — the explanation search
reads the primary justification, and a resumed fork records a different one —
and the unsat core is named in clause 1 as part of the line. Two things keep
it a divergence rather than a failure, and both are worth stating plainly:

- **No corpus entry reaches it.** T3 is green corpus-wide apart from D2, and
  the printed core is identical on every entry that prints one. This is an
  off-corpus input a fuzzer wrote.
- **The difference has a direction.** ein.rs's core is a *subset*: the
  explanation search finds a **shorter** frontier from the resumed fork's
  justifications. A smaller core is a better core, so what the resume costs
  here is agreement, not quality.

That is still the closest the milestone has come to clause 1, and it is the
first item that would change the D3 decision if a corpus puzzle reached it.

**A reach nobody had looked for, found 2026-08-20 by
[S1a.6.6](p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s fuzzer.** If a
model satisfies the `(query :goal …)` pattern **more than once**, the solve
table prints `rows[0]` of an *unsorted* match — and the row a matcher yields
first depends on the order the facts went into the KB, which is precisely what
resuming root's saturation changes. Seven forms are enough:

```lisp
(relation ok T) (relation blessed T) (relation cand T)
(rule promote () :match (and (ok ?x) (blessed ?x)) :assert (cand ?x))
(ok B) (blessed A)
(query :goal (cand ?x))
```

ein.py prints `?x = B`, ein.rs prints `?x = A`. **Everything else is
identical**: the verdict, `k = 1`, `exhausted`, the model as a *set*, all
twelve counters and every byte of `summary.json` — whose `goal_bindings`
carries **both** rows and sorts them, which is why T0 and T1 cannot see this
and the answer table can. No corpus entry reaches it: 59 scanned, 8 with a
multi-row goal, all stdout-identical.

The fix is available and is a *decision*, not a repair: show the lex-smallest
row, as `summary.json` already sorts, which would make the table
implementation-independent and shuffle-invariant at the cost of one visible
change to a checked-in fixture (`examples/branching/12_typed_blind_solve.ein`
would print `?c = Blue` where it prints `?c = Red`). Recorded here rather than
applied; the reproducer lives in `conformance/fuzz_findings/`.

**What would make this unacceptable.** Any of:

1. **a moved answer** — a verdict, a `k`, a model, a query binding, a printed
   unsat core or any `summary.json` field differing between the two engines on
   any corpus entry. That is what the fixture asserts, and it is the line the
   milestone does not cross. **Read "a query binding" as the binding *set*,
   which is what `summary.json` carries and what is asserted** — the
   observation above is the reason that distinction now has to be written
   down;
2. **a moved fixpoint** — an alive fork whose fact set differs, or, with
   fail-fast off, any fork's. The re-derivation would then be load-bearing
   rather than redundant and the whole change is wrong;
3. **a consumer of the primary justification that needs the *particular*
   derivation** — the explanation search and the trace read whichever
   derivation is recorded, and observation 2 above says which one that is may
   differ. A feature that promises "the shortest proof" or "the proof through
   rule R" makes this a bug rather than a divergence.
