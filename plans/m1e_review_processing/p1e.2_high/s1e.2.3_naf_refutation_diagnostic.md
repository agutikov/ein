# S1e.2.3 — Q-M1e.9's containment: a diagnostic for a refutation resting on an `absent`

**Phase:** [P1e.2](README.md) (High)
**Estimate:** 1 day
**Depends on:** nothing. The compiler already knows both sets this needs — a
rule's guards' watched relations, and the query's hypothesis-eligible ones.
**Blocks:** nothing. It is the *containment*;
[S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
later rules whether the diagnostic becomes a **refusal** and whether the shape
stays legal at all.
**Answers:** [Q-M1e.9](../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)'s
option **B**, ruled 2026-08-28
([D4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)).

> **Not one of the 63.** This is a hazard the milestone *found*, not a finding
> it was given, and it is scheduled here rather than deferred because it is the
> only item in M1e that leaves a **wrong answer** shipping. The same honesty
> marker P1f.10 and P1e.5 carry applies: M1e's acceptance does not depend on
> this stage, and cutting it costs a diagnostic, not a disposition.

## What it contains

`dead` is **not** upward-closed under `absent`. Probed 2026-08-28: a twenty-line
program whose rule refutes `(p A)` only while `(q A)` is missing has `{(p A)}`
dead and `{(p A), (q A)}` alive, and **five of six shipped configurations
answer it wrongly** — the writeback stores `(not (p A))` and layer 2 never sees
the pair. Three shipped mechanisms read the premise: the lookahead kill cache,
the singleton writeback, and the no-good store's width-1 clause.

D4's option **B**, in its own words: *narrow the claim, keep the machinery* —
state that the search is sound only for programs whose `(false)` derivations do
not pass an `absent` guard over a hypothesis-eligible relation, and enforce it
with a **load-time check**. The point is to turn a silent wrong answer into a
diagnostic, which is the repo's usual move.

**What it is not** is option C — making the three consumers world-aware. That is
the real fix, it wants its own stage, and it is filed as
[F18](../../followups/f18_world_aware_negatives.md) with `Prov::absent` as its
starting point.

## The check

For every rule the program **activates**: does its `:match` carry an
`(absent P)` whose relation is *hypothesis-eligible for this program*, and does
its `:assert` conclude `(false)` or a `(not …)`?

Both halves are already computed. The compiler knows each guard's watched
relations (it is what `derived_naf_warnings` reads), and the query's
`:hypothesis-relations` / `:no-hypothesis` clauses plus `(__closed__ …)` give
the eligible set. **Widen the existing warning rather than add a second
check** — `warn-derived-naf` today watches an `absent` over a *rule-derived*
relation, and this is the sibling case it does not cover, which is why the one
existing hazard signal is silent on the probe.

**Scope matters more than the check does.** A syntactic census (2026-08-28)
finds **60 rules** in `stdlib/`, `examples/` and `tests/` with an `absent`
guard and a refuting or negative conclusion. Almost all are safe, because their
`absent` reads a **given** structure — `std.slots`' prune/endpoint family reads
the position adjacency `?S`, which no generator proposes. A check that fires on
those is a check that gets turned off. The one stdlib rule with the exposed
shape is **`std.algebra`'s `connex`**, whose `absent` reads the subject relation
`?R` itself.

## The default, decided by measurement rather than in advance

A warning nobody sees is not containment, and a warning that fires on
`zebra2` is one that gets disabled. So the default follows the census, in this
order:

1. implement the check and run it over the whole corpus;
2. **empty** → the warning ships **on by default**: nothing prints today, and it
   fires only on a program outside the corpus, which is exactly the population
   at risk;
3. **non-empty** → the entries it names are the finding. The warning stays
   behind `(config :warn-derived-naf true)` until
   [S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
   disposes of them, and this stage's product is the list.

**A refusal is not available today**, and the reason is `connex`: refusing at
load would refuse a stdlib rule before anyone has decided whether it should be
rewritten in `total`'s stored-negative style. That decision is S1f.10.8 T2–T4's,
and promoting warn → refuse afterwards is a one-line change to code this stage
ships.

## Acceptance

- ✅ **The check exists and is one code path**, widened from
  `derived_naf_warnings` rather than added beside it
  ([AR-M1](../README.md#the-findings)) — and literally so: the old function
  *is* the new one over an empty eligible set.
- ✅ **The corpus is measured**, and the exposed set is named per entry — the
  number that decides the default, and the input
  [S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
  T1 starts from rather than re-derives.
- ✅ **The message names the replacement**, not just the hazard: `total`'s
  stored-negative form for a refutation, `(open ?R)` for a requirement. A
  diagnostic that says *don't* and not *instead* is a diagnostic people work
  around. Both strings are asserted by
  `the_probe_warns_and_the_message_names_both_replacements`.
- ✅ **A fixture** — D4's probe, banked as
  [`examples/ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein),
  with the warning in its recorded solve shape.
- ✅ **Not one answer moves.** Six `::naf` shape rows (the census, recorded in
  a checked-in file), 45 new rows for the new fixture, and **0** `solve` rows
  and 0 exit codes changed. `./run_tests.sh` green.
- ✅ **The narrowed claim is written where the premise is** — beside
  [design/08](../../../docs/history/m1a_rust/design/08_parallelism.md) § The
  objects' *monotone* definition and in
  [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md),
  which already states C3 and now has the probe that cashes it.

## Tasks

### Task T1e.2.3.1 — Widen the watch, and measure ✅

Half a day. Extend the guard scan to hypothesis-eligible relations, run it over
every corpus entry under its declared runs, and produce the exposed set.

### Task T1e.2.3.2 — The default, the message, the fixture ✅

Half a day. Set the default per the census; write the message with its two
replacements; bank D4's probe with the warning in its expected output; write
the narrowed claim beside design/08's premise and in `absent_semantics.md`.

## Notes

**Why this is a stage and not a task of
[S1e.2.1](s1e.2.1_correctness.md).** That stage is three named findings with
three dispositions; this is a hazard with a measurement and a default that
depends on it. Folding it in would bury the one item in M1e that is about a
wrong *answer* rather than a wrong *surface*.

**And why B rather than waiting for S1f.10.8.** S1f.10.8 runs in P1f.10, which
is late and cuttable. B is a day, it stops nothing else, and its product — the
exposed set — is what S1f.10.8's first task would otherwise have to compute
before it could rule.

---

# Record — done 2026-08-30

## T1e.2.3.1 — the widened watch

**One walk, two questions.** `compute_naf_map` already scanned every guard's
watched relations to ask whether they are *rule-derived*; it now asks a second
question of the same walk, and the two are genuinely complementary rather than
one refining the other:

| | the guard watches | the hazard | since |
|---|---|---|---|
| `DerivedNafWarning` | a **rule-derived** relation | the rule set may not be stratified, so the engine reports one model where several exist. **Sound** — the guard is judged at a fixpoint | ein.py, re-grounded S1.21.8 |
| `RefutationUnderAbsentWarning` | a relation the **generator can propose**, and the rule concludes `(false)` or a `(not …)` | a *commitment* would discharge the guard in a world the search never enters — `dead` is not upward-closed | M1e S1e.2.3 |

[`NafDep::refutation`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) is
`Some` **only** when both halves hold, so the field's presence *is* the
finding and no caller re-derives the conjunction. And there is no second
switch: `derived_naf_warnings` is `naf_warnings` over an **empty** eligible
set, because a refutation line needs a relation the generator can propose and
an empty set has none. That is what makes `AR-M1`'s "one code path" literal
rather than a claim — there is nothing to keep in sync.

**One decision the task did not anticipate.** `refuting_conclusion` walks
**every** `:assert`, where `asserted_relation` / `negated_relation` read
`assert_template()` alone (they reproduce ein.py's S1.7.4 map and have parity
to keep; this is a new question and has none). It is not tidiness:
`examples/syntax/rule-forall-and-not.ein`'s `:assert (and (r ?b ?a) (not (r ?a
?a)))` is exposed through its **second** arm, and reading the first would have
missed the census's only `(not …)` row.

### Eligibility is a property of the program, and that is the whole point

[`hypgen::eligible_relations`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)
reads the ladder the program declares: **hrules** → what they conclude (no
scoping, because `apply_filters` applies none to them); **obligations** → the
`(open ?R)` arguments, scoped; else **blind** → `relation_plan`'s filter, off
the same helpers so the two cannot drift. The closed set is the one *in the
KB*, not `emit_closed`'s, which runs on a fork for `--hyp-stats` and the
summary and so is invisible to a solve.

Taking the **union** with blind for obligation programs was tried and
rejected, and the number is why: on `examples/zebra2-obligations.ein` it makes
every declared relation eligible and the warning fires **40 times** on a puzzle
that solves correctly, naming `total`, `surjective`, `typecheck-arg-*`,
`disjunctive-prune-*` and `adjacent-via-*-negative`. That is the stage's own
warning about a warning nobody leaves on, met on the first measurement. The
residual — the obligations rung can *decline* and fall through to blind — is
written at the site rather than papered over.

## T1e.2.3.2 — the census, the default, the fixture, the claim

### The census corrects D4's prediction

**Nine rules over seven entries**, of the 60 that match syntactically —
[`refutation_under_absent.rs`](../../../ein.rs/crates/ein-infer/tests/refutation_under_absent.rs),
which states the set **exactly** because both directions are findings.

D4 predicted the `std.slots` family safe and `std.algebra`'s `connex` as the
one exposed stdlib rule. **The first half holds and the second does not:**

- Not one of the `slots` prune / endpoint / adjacency rules is exposed. Their
  `absent` reads the given position structure, exactly as D4 said.
- **`connex` is activated twice in the corpus — `tests/stdlib/algebra/08` and
  `12` — and is exposed neither time**, because both fixtures write
  `:no-hypothesis (instance lt)` and so exclude the subject relation from
  generation. The discipline that saves it is a *query keyword*, applied by
  the fixture author, not anything about the rule.
- What **is** exposed is `std.elim`'s `typecheck-arg-0` / `typecheck-arg-1` /
  `no-room-left` / `no-room`, whose guards read a **membership** relation —
  `is-a` on `features/05`, `instance` on `features/12` — that nothing in
  either file closes or excludes, so the blind enumerator can propose one.

So the census's real result is that **exposure is a property of the program,
not of the rule**, and a syntactic count of 60 resolves to nine only once the
generator is consulted. It is also why a fix scoped to "the rules with the
shape" would be scoped to the wrong thing, which is now written into
[F18](../../followups/f18_world_aware_negatives.md).

The other four rows are three probes (`branching/13`, `branching/14`, and D4's
own, banked below) plus `ein-bugs/alive-empty-interlayer.ein`, whose `totality`
rule has the probe's exact shape over a relation its own `(hrule guess …)`
proposes — a recorded bug fixture that was already sitting on this.

### The default: **non-empty, so it is gated** — the stage's own rule 3

Four of the nine rows are stdlib scans on files that answer correctly, so
shipping it on would print a warning on `ein solve
examples/features/05_stdlib_domain_elim.ein`. It rides
`(config :warn-derived-naf true)`.

**One flag for two questions, and not an eighteenth field**, for a reason that
is not taste: a `SolverConfig` field is rendered into the KB-shape digest, so a
new one re-blesses **every** shape golden in the corpus — the same argument
that kept `EIN_OBLIGATION_CHOICE` out of the config. Documented on the flag's
row in [`configuration.md`](../../../docs/kernel/configuration.md), in
`config.rs`'s doc comment, and in
[`inference/README.md`](../../../docs/kernel/inference/README.md).

### The fixture

[`examples/ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein)
— D4's probe, banked **where the gate sweeps**, because the `plans/` copy goes
when M1e's tree does, the way M1a's, M1c's and M1d's went. It carries
`(config :warn-derived-naf true)`, so the warning is in its recorded solve
shape (`solve_shape` filters the log to `enter` / `nogood` / `writeback` /
`warn`, and `solve` falls back to the KB's config), and an `:expect` that
states **today's wrong answer** — `(q A)` plus the cached `(not (p A))` — and
is meant to break when a world-aware fix reaches the maximal state. That is the
tripwire its three `alive-empty-*` / `complete-records-stale` siblings already
carry. Its goal is on `is-a` because the model contains no `p` fact and an
expectation cannot state an answer with an empty goal extent — Q-M1e.13, the
same reason `alive-empty-phase1.ein` moved its goal.

### The narrowed claim, written where the premise is

[design/08 § The objects](../../../docs/history/m1a_rust/design/08_parallelism.md)'s
`dead` row now says it is false under `absent`, why (*append-only* makes `sat`
inflationary, not monotone in its input), and that **claim (1) fails on its own
terms** — with the scope that keeps the rest of the section true: every claim
there holds on a program without the shape, which is every puzzle the port was
measured on. [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
gains it as a fifth § Explicitly-not-provided item, beside the C3 that had been
saying the same thing from the other direction all along.

## Outcome

| | |
|---|---|
| answered | [Q-M1e.9](../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)'s option **B**, shipped |
| the check | one walk, two questions, `eligible` as the switch — no second scan and no second flag |
| the census | **9 rules over 7 entries** of 60 syntactic matches, exact and pinned. D4's `connex` prediction **corrected**: activated twice, exposed neither time |
| the default | **gated** behind `warn-derived-naf`, per the stage's rule 3, because the corpus is not silent |
| new fixture | `examples/ein-bugs/naf-upward-closure.ein` + its corpus row, `:expect` stating today's answer |
| new test | `ein-infer/tests/refutation_under_absent.rs` — four claims: the exact set, the probe's message with both replacements, the warning in the fixture's own shape, and that the *old* warning does not cover this |
| answers moved | **none.** 6 `::naf` shape rows (the census, recorded), 45 new rows for the new fixture, **0** `solve` rows and 0 exit codes changed. The eligible set is computed inside the flag's gate, so a default run pays what it paid before |
| filed forward | [F18](../../followups/f18_world_aware_negatives.md) re-measured and given a third trigger — the test fails when the set moves; [S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md) starts from the banked set rather than re-deriving it |
| gate | `./run_tests.sh` green |

### One deviation from the stage text: it is not a **load-time** check

§ The check calls for a *load-time* check, and D4 recommends "a load-time
refusal". It is emitted **after root's saturation** instead, for the reason
`naf_deps`'s own header already gives about the other question: **the map is
only complete on a saturated cache.** Most NAF-bearing rules — the elimination,
totality and spatial families — are activated by facts a *rule* derives, so
their plan does not exist at load, and a check taken there would silently omit
exactly the rules the analysis is about. Six of the nine exposed rows are
activated rules (`typecheck-arg-*`, `no-room-left`, `no-room`), so this is not
hypothetical: at load the census would have been three.

It costs nothing that matters today — a warning is a warning wherever it is
emitted. It **does** constrain S1f.10.8: promoting warn → refuse is still a
one-line change, but the refusal lands at root saturation rather than at load,
so it is an `Answer`-shaped refusal and not a `kb load error:`. Worth knowing
before that stage writes its exit-code table.

### Two things the tasks did not predict

**1. The union was not a conservative choice, it was a useless one.** The task
treats eligibility as a lookup the compiler already has. It is — but *which*
lookup depends on the rung, and the honest-looking answer (take every rung's
set, since the obligations rung can decline) makes the warning fire 40 times on
`zebra2-obligations.ein`. The conservative direction for a hazard warning and
the usable direction are opposites here, and the stage's own sentence — *a
check that fires on those is a check that gets turned off* — is what decided
it. The under-warning is written down at the site instead.

**2. The stage predicted its own census wrong, and the fixtures are why.**
D4's `connex` reasoning is sound about the *rule* and silent about the
*program*, and two fixtures had already applied the discipline it says `connex`
lacks — at the query, with `:no-hypothesis`. Meanwhile `std.elim`'s type-check
scans, which D4 never considers, read `is-a` on a file that closes nothing.
Neither fact is visible without running the check, which is the argument for
the stage's ordering: *implement, then measure, then set the default*.
