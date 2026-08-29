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

- **The check exists and is one code path**, widened from
  `derived_naf_warnings` rather than added beside it
  ([AR-M1](../README.md#the-findings)).
- **The corpus is measured**, and the exposed set is named per entry — the
  number that decides the default, and the input
  [S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
  T1 starts from rather than re-derives.
- **The message names the replacement**, not just the hazard: `total`'s
  stored-negative form for a refutation, `(open ?R)` for a requirement. A
  diagnostic that says *don't* and not *instead* is a diagnostic people work
  around.
- **A fixture** — D4's probe, banked, with the warning in its expected output.
- **Not one answer moves.** A warning changes no verdict, no `k`, no model set
  and no exit code; `./run_tests.sh` is green with it on.
- **The narrowed claim is written where the premise is** — beside
  [design/08](../../../docs/history/m1a_rust/design/08_parallelism.md) § The
  objects' *monotone* definition and in
  [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md),
  which already states C3 and now has the probe that cashes it.

## Tasks

### Task T1e.2.3.1 — Widen the watch, and measure

Half a day. Extend the guard scan to hypothesis-eligible relations, run it over
every corpus entry under its declared runs, and produce the exposed set.

### Task T1e.2.3.2 — The default, the message, the fixture

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
