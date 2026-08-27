# Solution semantics — hypothesis, commitment, solution, model

> **Normative, and new (2026-08-28).** This page defines the vocabulary the
> search is stated in and the two objects it produces. It supersedes no page;
> before it, `hypothesis` and `commitment` had no definition anywhere in this
> tree (the [glossary](../glossary.md) defined *Layer* in terms of
> "commitment sets" without defining a commitment, and had no entry at all for
> `alive`, `solution` or `obligation`).
>
> **§6 is the honest half.** The definitions here are the *intent*; the engine
> implements an approximation of one of them, and §6 says which, in which
> direction it errs, and how to reproduce it. A reader who needs only what the
> binary does today should read §6 first.

---

## 1. The vocabulary, in dependency order

Each term is defined using only the ones above it.

### Hypothesis

A **fact the program has not decided** and the generator is willing to guess:
a fact of a hypothesis-eligible relation that the state neither asserts nor
refutes.

Three things make a relation hypothesis-eligible or not — the query's
`:hypothesis-relations` allow-list, its `:no-hypothesis` deny-list, and
`(__closed__ R)`. Scoping is part of *what the program asked to be solved
for*, so a fact of an excluded relation is not a hypothesis at all; it is not
a hypothesis the engine declines to try.

Which hypotheses exist is the **generation ladder**'s answer
([`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)), and the ladder
has three rungs: the program's own `(hrule …)` if it declares one, else the
facts that would discharge what the state owes
([`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)), else the blind
combinatorial enumerator.

### The hypothesis set — L1

**`alive₀`**, the hypotheses at root's fixpoint: root saturated, the
forced-positive cascade run, no contradiction. It is computed once
(`Run::phase1`, [`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs))
and it is **the space the search quantifies over** — every later definition on
this page reads *"hypothesis"* as *"member of `alive₀`"*.

Called **L1** because the search's first layer enters its singletons.

### Commitment — L{n}

A **set of hypotheses assumed together**: `{h₁ … h_n} ⊆ alive₀`. The lattice's
node, `CanonicalSetId = Vec<FactId>` in the code, and the object a *layer* is
a layer of — layer `n` enters the `n`-subsets of `alive₀`.

An L{n} commitment **is** n L1 hypotheses. The two phrasings name one thing,
and this page uses whichever reads better locally.

### Entering

**What the search does with a commitment**: fork root, add the commitment's
facts, saturate, look for a contradiction. Three outcomes
([`commitment.rs`](../../../ein.rs/crates/ein-infer/src/commitment.rs)):

| outcome | meaning |
|---|---|
| `alive` | the fork reached a fixpoint with no contradiction |
| `dead-pre` | contradictory as soon as the hypotheses were written — no saturation ran |
| `dead-post` | contradictory at or before the fixpoint |

**Dead** is the union of the last two. *Entering* is a verb about the search;
`enterings_total` counts them and is the engine's unit of work.

### Integrated

The hypotheses that are **true in the resulting state** — the committed ones
*plus any that saturation derived*.

The distinction is not pedantic and it is where a naive reading goes wrong.
Commit three hypotheses; saturation derives ten further facts; if five of
those ten are themselves members of `alive₀`, the state has **eight**
hypotheses integrated, not three. Every quantifier below reads over
`alive₀ \ integrated(S)`, never over `alive₀ \ committed(S)`.

---

## 2. Solution

> A **solution** is a saturated, consistent state in which **every hypothesis
> it has not integrated is inconsistent with it**.

Formally, for a state `S` reached by entering commitment `C`:

```
solution(S)  ≡  S is saturated
              ∧ S is consistent
              ∧ ( S owes nothing                                    [obligations arm]
                ∨ ∀ h ∈ alive₀ \ integrated(S):  S ∪ {h} is dead )  [maximality arm]
```

The two arms are the ladder's two regimes. A program that **states** an
obligation is judged by **discharge**: it has said what it owes, and owing
nothing is the answer. A program that states none is judged by
**maximality**: nothing can be added.

### The operational form

The maximality arm has a shorter reading, and it is the useful one:

> **A solution is a maximal alive commitment — one with no live child.**

`C` is a solution exactly when no L{n+1} extension of it is consistent, which
is to say: *for this commitment, the search below it ends.* That makes the
definition a **stopping rule** as much as a predicate, and it makes it
something the lattice can answer from its own next layer rather than by asking
the generator at the node — see §6.

### Worked: `examples/lattice/02_genuine_3set_death.ein`

Three hypotheses `h₁ = (a-prop X)`, `h₂ = (b-prop X)`, `h₃ = (c-prop X)`; one
rule asserting `(false)` when all three hold.

| commitment | alive? | live child? | solution? |
|---|---|---|---|
| `{}` | yes | yes — all three singletons | no |
| `{h₁}` | yes | yes — `{h₁,h₂}`, `{h₁,h₃}` | no |
| `{h₁,h₂}` | yes | **no** — `{h₁,h₂,h₃}` dies | **yes** |
| `{h₁,h₃}` | yes | no | **yes** |
| `{h₂,h₃}` | yes | no | **yes** |
| `{h₁,h₂,h₃}` | no | — | no |

**Three solutions.** No pair excludes — the conflict is genuinely three-way,
which is what the fixture is named for — so nothing shorter than an entering
establishes any of the three "no live child" cells.

---

## 3. Model

> A **model** is the **positive part of a solution's KB, minus the positive
> part of the initial KB**.

The **initial KB** is the program as loaded, *before the first saturation*:
problem statements, `(relation …)` signatures, `(is-a …)` declarations,
rules — no derived facts.

So a model carries no `(not …)`, nothing the file itself stated, and nothing
the loader wrote down. It is **what the puzzle did not say and the solve
established**.

The distinction from the solution's KB is load-bearing, because two runs can
agree on every model and disagree on every solution KB. `ein solve -e
examples/lattice/02_genuine_3set_death.ein` records `(not (c-prop X))` in the
first state — the lookahead kill cache is what writes it, with provenance
`<lookahead-dies-immediately>` — and `-K` records the same solution without
it. Same models; different KBs.

---

## 4. The obligations arm — owes, discharged, open

An **obligation** is a rule that asserts the verdict atom `(open ?R)` while a
witness is missing. What a state **owes** is its undischarged obligation
instances; a state that owes nothing is **discharged**.

The arm is **scoped**: only a program that *states* an obligation can be
judged by discharge. A program that states none is judged by maximality, which
is why 92 of the 121 corpus entries that reach a fixpoint report exactly the
words they reported before obligations existed.

Normative detail lives in
[`domain_contract.md`](../../history/m1d_satisfiability/domain_contract.md)
(C1–C4) and
[`completeness.md`](../../history/m1d_satisfiability/completeness.md).

---

## 5. What the verdict says

| word | means | `k` |
|---|---|---|
| `Solution` | one model | 1 |
| `Ambiguity` | several distinct models — the puzzle is under-determined | > 1 |
| `Open` | consistent and quiescent, with an obligation the program stated still unwitnessed | 0 models |
| `Contradiction` | no model | 0 |

`k` counts **models**. `stats.solution_nodes` counts what the *search*
recorded, and the two are not the same number in the mixed regime.

**`exhausted`** reports that the search did not stop early — no depth cap hit
with a live frontier, no `--solutions` cut, not the tree traversal. Read §6
before reading it as a certification of the model set.

---

## 6. What the engine implements, and where it differs

Everything above is the definition. The engine tests it with a **different
predicate**, and the difference is observable today.

### `complete()` is a sound, incomplete approximation of maximality

[`hypgen::complete`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) asks the
generator *at the node, now*: a state is complete iff the generator proposes
nothing. A candidate is not proposed when it is already asserted, already
negated, already seen in this call, or **provably dies in one rule firing**
against this state (the pre-branch lookahead).

| | holds? | consequence |
|---|---|---|
| `complete(S) ⇒ solution(S)` | **yes** | every filter that can make `complete` true is a genuine refutation, so **the engine never records a false model** |
| `solution(S) ⇒ complete(S)` | **no** | a remaining hypothesis that needs *two* firings to die is still proposed, so a real solution goes unrecorded |

The engine therefore **under-reports**, always. With
`enable-pre-branch-lookahead` off it under-reports far more: `complete` is
then true only when every remaining hypothesis is already asserted or already
negated in the KB.

**The lookahead at root loses nothing.** If `root ∪ {h}` is refuted then so is
`S ∪ {h}` at every descendant, so dropping `h` from `alive₀` removes no
solution. The gap is at the deeper nodes, and it is the *approximation*, not
the filtering.

### `exhausted` certifies the lattice, not the model set

```
$ ein solve -e --no-lookahead examples/lattice/02_genuine_3set_death.ein
  solutions (k)   0            exhausted = true      7 enterings, 3 layers
  verdict         No solution — the constraints are contradictory
```

That program has the three solutions of §2. The search **found all three
states** — it entered each pair, each survived, then it proved every triple
dead — and reported that there are none. Nothing was truncated: the lattice
really was walked to the end.

*The lattice was walked to the end* and *every solution in it was recognised*
are two claims, and only the first is what `exhausted` tracks. A verdict
saying *the constraints are contradictory* asserts the second.

### The premise both of them rest on

The maximality arm reads *"no live child"*, and reading it from the next
layer's results — rather than by re-forking every remaining hypothesis —
requires **`dead` to be upward-closed**: `X ⊆ Y ∧ dead(X) ⇒ dead(Y)`.

That premise is not new here. It is what
[`apriori.rs`](../../../ein.rs/crates/ein-infer/src/apriori.rs)'s
downward-closure filter and the no-good store already assume, and
[design/08 § The objects](../../history/m1a_rust/design/08_parallelism.md)
states it as a definition. [C3](absent_semantics.md) states what looks like
its contrapositive as a live caveat — *removing a fact can flip an absent and
fabricate a contradiction the full KB never had* — so the two pages are worth
reading together before anything further leans on it.

---

## 7. Where each term lives in the code

| term | site |
|---|---|
| hypothesis, the ladder | [`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs), [`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs), [`hrule.rs`](../../../ein.rs/crates/ein-infer/src/hrule.rs) |
| `alive₀` | `Run::phase1` → `compute_alive` → `hypgen::open_hypotheses` |
| commitment | `apriori::CanonicalSetId`; the layer walk is `Run::phase2` |
| entering, alive/dead | [`commitment.rs`](../../../ein.rs/crates/ein-infer/src/commitment.rs) `try_commitment_set`, `Kind` |
| complete | `hypgen::complete` |
| owes, discharged | [`obligations.rs`](../../../ein.rs/crates/ein-infer/src/obligations.rs) `tally` |
| solution record, model set | `Run::record_node`, `Run::finalise` |
| verdict words, `k` | [`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs) |
| `exhausted` | `Run::finalise`, from `lstate.truncated` |

## Cross-references

- [`absent_semantics.md`](absent_semantics.md) — worlds, the NAF boundary,
  and the corollaries the upward-closure premise has to be read beside.
- [`completeness.md`](../../history/m1d_satisfiability/completeness.md) — why
  an obligation's alternatives are jointly exhaustive, which is the discharge
  arm's warrant.
- [`domain_contract.md`](../../history/m1d_satisfiability/domain_contract.md)
  — C1–C4, what an obligation quantifies over.
- [`features.md`](features.md) — the measured feature × config matrix,
  including what the lookahead is worth.
- [`glossary.md`](../glossary.md) — one-line entries for every term on this
  page.
