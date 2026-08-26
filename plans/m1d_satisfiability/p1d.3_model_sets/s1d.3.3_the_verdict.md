# S1d.3.3 — What the verdict says, and whether this ships

**Phase:** [P1d.3](README.md) (Model sets without enumeration)
**Estimate:** 2 days
**Depends on:** [S1d.3.2](s1d.3.2_representations.md) — the priced candidates;
and [S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md), whose
verdict vocabulary anything here has to fit inside rather than beside.

**Status: done 2026-08-26.** Q-M1d.5 closed, the rendering rule shipped with
three fixtures, `--models key` shipped, the semantics stated and unchanged —
banked in [`the_verdict.md`](the_verdict.md). See § What it found.

## What it found

**The user's decision was *both*, and the half nobody had asked for was the
one that was broken.** The stage expected to weigh (b) against *enumerate*;
what writing down the guarantee rule found is that the enumeration was
**stating a model count it had no right to state**.

| task | asked for | done |
|---|---|---|
| T1d.3.3.1 — Q-M1d.5 | one of (a)–(e), or *enumerate* | **both** — the count, qualified; and **(b)** behind `--models key`, S1d.3.2's recommendation taken |
| T1d.3.3.2 — the guarantee | one sentence, **two fixtures** | **three fixtures on three entries**, and a live defect: `type-exclusivity/colors.ein -e` printed `k = 5` where the file has **9** |
| T1d.3.3.3 — the semantics | which reading, and change nothing | **stated**: a reported model is a *state*, `k` counts states, closure is **per relation and opt-in** — and the language already has it twice (`:expect`, the domain contract). Neither of the two places changed |
| T1d.3.3.4 — additional, never a replacement | `verdict.solutions` byte-identical, the golden diff cell by cell | **five of 8 171 renderings moved**, all `trace[answer]`, all on `Ambiguity` entries, **every line count unchanged** — and the summaries compare equal with the flag on and off |
| T1d.3.3.5 — the phase ledger | the closing record | [P1d.3 § The ledger](README.md) |

**Four things the stage did not ask for and got.**

- **The defect was on five corpus entries, and the asymmetry was backwards.**
  `Solution` has hedged its `k = 1` since ein.py — a guess about *uniqueness*.
  `Ambiguity` hedged nothing, where the claim is a **number** and five of the
  ten entries that make it are short. `exhausted` was printed by `--stats` and
  by nothing else.
- **(b) fails in its margins where (a) fails in its cells.** A key row's
  values are read off a model that exists, so no row can be falsified — a 33rd
  model **adds** a row or **shares** one, which withdraws the table's
  completeness and the key's sufficiency and nothing else. (a)'s core is an
  intersection, and intersecting a subset gives a **superset** of the truth, so
  a 33rd model can contradict any of its 312 printed facts. That is the phase
  acceptance's third bullet with a winner attached, and it is an argument
  S1d.3.2 could not make because it was pricing forms rather than stating
  guarantees.
- **The shipped table is the census's table, on every model set the corpus
  has.** Two independent implementations of the decision-variable rules and the
  minimum-hitting-set search: all **32 rows** of `zebra2-minus-15` identical
  with the same 22 minimum keys and the same two forced columns, and **11 of
  11** other entries agreeing on `k` and on the key size at the census's own
  caps. The closest thing the repo still has to an oracle diff.
- **The fallback had to be built, and the corpus supplied its case.**
  `branching/06_lookahead_on.ein` needs an 8-variable key over 42 slots —
  `C(42, 8) = 118 030 185` — so `--models key` declines and prints the models,
  which is (e), which was a legitimate winner all along. Finding that out costs
  **42 ms**, against **12.4 s** for the same search in the census's Python —
  the shipped one precomputes the branch table, because a budget counted in
  nodes only means something in seconds if a node is cheap.

**And one number the stage hands on rather than keeps.** Three entries answer
`Contradiction` with `exhausted = false`, and on
`saturation/type-exclusivity/pets.ein` the word is not merely unproven but
**wrong**: `k = 0` at `-m 5` through `-m 8`, and **35 models** at `-m 10`. The
ten entries Q-M1d.6 was opened for had no models at any depth. That one is
[Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
and [P1d.10](../p1d.10_exhaustive_search/README.md)'s.

## Context

The phase's decision stage:
[Q-M1d.5](../open_questions.md#q-m1d5--print-or-describe) answered, and if
something ships it ships here. Three things constrain the answer, and two of
them arrived after the phase README was written.

**The verdict vocabulary is now four words and one of them is about
incompleteness.** `Open — owes n` says *a state is consistent, quiescent and
not finished*. An `Ambiguity` reporting *"these 312 facts hold in every model
and these 23 slots are undecided"* is the same sentence about a **set** of
states, and it should not invent a second vocabulary for it. Whatever this
stage ships is phrased in S1d.2.6's terms or it explains why it cannot be.

**The completeness guarantee is the phase acceptance's third bullet and it
bites hardest here.**

> Whatever is reported carries **the same guarantee vocabulary** the rest of
> the milestone settles: a compact description of a model set claims
> completeness only when the search proved it.

And on the phase's own case the search has *not* proved it: `solve -e
zebra2-minus-15` reports `Ambiguity k=32, exhausted=false` — the depth-5
frontier is non-empty, so the cap stopped the search and not the lattice
([layer census §4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)).
So the honest compact form is a description of **a lower bound on the model
set**, and a "certain core" computed from 32 models that might not be all of
them is *not* certain: a 33rd model could contradict any of the 312. **A
compact form that quietly upgrades `exhausted = false` into a claim about all
models is the one outcome this stage must not ship**, and it is the easy
mistake, because the core is computed by intersection and intersection of a
subset is a superset of the truth.

**And the corpus offers one case.** Nine entries are multi-model; seven are
two- or three-model toys. A phase deciding presentation on n = 1 has a strong
prior toward "enumerate, and say so", and the burden is on shipping.

## The question P1d.2 handed forward

[`ideas.md`](../ideas.md) § *Когда fixed point является решением*, outcome 3,
asks the question this phase inherited and P1d.2 could not answer:

> Но здесь возникает важный вопрос: обязательно ли назначать значение каждому
> возможному факту?
>
> Если действует closed-world completion, все оставшиеся `open` считаются
> отсутствующими. Тогда получаем одну полную модель. Если open-world semantics,
> насыщенный граф может представлять сразу семейство моделей.

Two places in the shipped engine turn on the answer:

- **The closed-and-owing corner.**
  [`domain_contract.md` §3](../p1d.2_obligations/domain_contract.md) and
  [the openness census §6](../p1d.2_obligations/openness_census.md) both defer
  promotion of *owes-and-cannot-pay* to `(false)` to "closed-world completion,
  which is P1d.3's". The user's decision of 2026-08-25 was `Open` and **not**
  `(false)`, on the ground that `(false)` is a *derived* refutation; the
  deferral is of the inference that would license the promotion, not of the
  word.
- **The leftover-open count**, [T1d.3.1.4](s1d.3.1_what_the_models_differ_in.md).
  A model with n leftover open facts is one model closed-world and 2ⁿ
  open-world, and no surface says which Ein means.

**This stage does not have to adopt closed-world completion**, and probably
should not: it is a semantics change with reach far past model sets — it would
make every `(unknown …)` probe and every stored-negative discipline in the
stdlib mean something different. What it has to do is **say which semantics the
model set is reported under**, because the compact form is a claim about a
family of graphs and the family's size depends on the answer. A written
statement plus the fixture that would break if it changed is the deliverable;
adopting it is a milestone-scale decision and belongs to whoever wants it.

## Tasks

### Task T1d.3.3.1 — Q-M1d.5 answered

The written answer, with [S1d.3.1](s1d.3.1_what_the_models_differ_in.md)'s
factorisation behind it rather than an intuition — which is the phase
acceptance's first bullet, and the reason the measurement came first. The
question's own constraint is the test of the answer:

> What is not legitimate is a compact form that only the engine can read.

The answer names one of (a)–(e) from
[S1d.3.2](s1d.3.2_representations.md), or names none and says *enumerate*. A
recommendation that ships nothing is a valid close and the milestone has
precedent for it — [the choice heuristic measured
inert](../p1d.2_obligations/hypotheses_from_obligations.md) was kept with its
number; a representation measured unreadable is dropped with its number.

### Task T1d.3.3.2 — the guarantee, as a rendering rule

Whatever is reported, the rule that keeps it honest, in one sentence and one
fixture. The shape it has to have:

| the search | what a compact description may claim |
|---|---|
| `exhausted = true` | *these are the models* — the core is certain, the frontier is complete |
| `exhausted = false` | *these are models found* — the core is certain **of what was found**, and a further model may contradict any of it |

The second row is `zebra2-minus-15`, which is to say the only interesting case,
so the rendering that gets exercised is the qualified one. **A fixture for each
row**, and the second one is what fails if someone later drops the qualifier.

### Task T1d.3.3.3 — the semantics, stated

One section: under which semantics is a reported model set a set of models —
open-world, closed-world completion, or *"the engine does not say and here is
what that costs"*. It names the two places the answer would change something
(above), and it does **not** change either. The deliverable is a statement
precise enough that
[P1d.4](../p1d.4_model_set_closure/README.md) can build a claim on it, since a
claim about a model set is meaningless without it.

### Task T1d.3.3.4 — if it ships: additional output, never a replacement

The phase acceptance's second bullet, as an implementation constraint:

> If something ships, it is **additional output, not a replacement**: the
> models remain enumerable, because every consumer — the trace, the GUI,
> `:expect`, the benchmark adapters — reads models.

So a flag or a summary block, `verdict.solutions` untouched, and the corpus
sweep proving the untouched half by having nothing move outside the new cells —
the discipline
[S1d.2.5](../p1d.2_obligations/hypotheses_from_obligations.md) and
[S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md) both used,
where the golden diff's *shape* is the acceptance evidence and not just its
size.

`:expect` does not grow a word here. It did not grow one for `Open`, and for
the same reason: its three forms are assertions about facts. A claim about a
model **set** is [P1d.4](../p1d.4_model_set_closure/README.md)'s subject and
this stage must not pre-empt it.

### Task T1d.3.3.5 — the phase ledger

The closing record in [the phase README](README.md), in the form
[P1d.2's](../p1d.2_obligations/README.md) took: the decisions and where, the
stages with their numbers, the census, and what was deferred with the
specification that survives it and the trip-wire that un-defers it. The
deferrals this phase will have at least three of — symbolic saturation (d),
decision diagrams (c) if unread, and closed-world completion — and each needs
its trip-wire stated as a property of a corpus entry rather than of a wish.

## Acceptance

- **[Q-M1d.5](../open_questions.md#q-m1d5--print-or-describe) closed**, with
  the factorisation behind the answer and the answer's form named — including
  if the answer is *enumerate*.
- The guarantee rule exists as **two fixtures**, one per row of the table
  above, and the `exhausted = false` one is on a real multi-model entry.
- The semantics section states which reading a reported model set is under,
  names the two places that would change if it changed, and changes neither.
- If anything ships: `verdict.solutions` is byte-identical on every corpus
  entry, and the golden diff is accounted for cell by cell rather than
  re-blessed wholesale.
- The phase ledger is written, with every deferral carrying a trip-wire that is
  a property of a corpus entry.
- The gate green after the one re-bless this stage owns, if it owns one.
