# The second-order boundary — a rule is a sentence about the world it fires in

**Stage:** [S1d.4.2](README.md#s1d42--the-second-order-boundary) · **Phase:** [P1d.4](README.md)
**Written:** 2026-08-26. **Closes**
[Q-M1d.7](open_questions.md#q-m1d7--may-a-program-require-its-own-model-count)
— **no, a program may not require its own model count**, and the reason
generalises to every second-order claim anyone proposes next.
**Method:** the rule-shape test [Q-M1d.2](open_questions.md#q-m1d2--where-does-a-requirement-live)
used one level down, applied honestly rather than assumed; plus a survey of the
four neighbouring systems, cited from [`docs/lib/`](../../../docs/lib/README.md).
**Nothing was implemented.** This stage's whole output is prose, one catalogue
entry, and a question's closing.

---

## 1. The question, and the asymmetry that makes it one

The **same s-expression** means two different things in two keywords:

| where | `(or A B)` means | quantifies over |
|---|---|---|
| `:match` | this world satisfies A or B | facts in **one** KB |
| `:expect` | the model set is exactly {A, B} | the set of **KBs** |

A test may say the second. May a **puzzle**? That is not a preference — a rule
fires *in a world*, which is what [`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs)
compiles, what `match_.rs` walks and what a firing's provenance records — so
*"and there are no others"* has nothing in the rule language to attach to. The
question is whether that is **a defect to fix or a boundary to state**.

It is a boundary. Three readings were tested and all three fail, for three
*different* reasons, and the third is the one that matters most.

---

## 2. The rule-shape test, applied

[Q-M1d.2](open_questions.md#q-m1d2--where-does-a-requirement-live) asked
where a *requirement* lives and answered **(c) a rule shape asserting a
reserved verdict atom** — form G,
[`(open ?R)`](README.md#s1d23--the-form). That worked because a requirement *is* a
sentence about one world: `(open ?R)` says **this KB's `R`-extent is
incomplete**. The precedent is a warning as much as a template — it looked
impossible right up until the atom made it local — so the analogous question
here gets the same test rather than the same answer.

> **Is there a rule shape whose firing means "the model set is closed"?**

### (a) A guard that is an `absent` over models — *refuted on compilation*

The shape would be `:match (absent «another model exists»)`. `absent` is the
NAF boundary and it compiles to **a sub-plan over the fact store**: its
question is *"does this KB contain a row matching this pattern?"*, answered by
scanning this KB.

There is no KB in which *"another model exists"* is a fact, and there cannot
be, because **another model is a different KB**. That is the whole refutation,
and it is not a limitation of `absent`: any guard, present or future, is
evaluated against the rows of the store the rule is firing over.

The contrast with `(open ?R)` is exact and is what makes this a test rather
than an assertion. Incompleteness of `R` *in this KB* is decidable by one pass
over this KB — the obligation rule's own `?isa`-parameterised scan, standing
beside the witness step inside its `absent`
([`domain_contract.md`](domain_contract.md) C1–C2). Closure
of the **model set** is not a property of any KB at all. It is a property of
the lattice of KBs, and no single node of that lattice carries it.

### (b) A rule reading the search's own state — *refuted on grounds, not on feasibility*

This is the one that has to be argued, because the engine **could** do it. `k`,
`exhausted`, `layers_explored` and `alive` all exist as counters; exposing one
of them as a fact is a hundred lines. The refusal is that a rule reading it
would make **derivation depend on the traversal**, and the traversal is
precisely the thing this engine promises is not observable.

Three test families enforce that promise today, and they are not decoration:

| | what it holds fixed | scale |
|---|---|---|
| `--shuffle` | the within-layer commitment order | the verdict is shuffle-invariant; a fresh seed every run unless `--seed` pins it |
| `id_order_invariance` | the interning order (`EIN_ID_SEEDS`) | the answer is the same under a permuted id space |
| `jobs_invariance` | the number of workers (`EIN_JOBS_SWEEP`) | **20 712** (file, op, jobs) cells, closed 2026-08-23 |

and behind them, M1a
[S1a.7.0](../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s
purity control: **1 078 154 speculations against a stale root with 0
differences** — the corpus-scale form of the property parallel entering rests
on.

**The concrete cost of allowing (b) is one sentence and it is decisive.** `-m`
is a **budget**. A rule that read `exhausted` would fire on
`examples/zebra2-minus-15-obligations.ein` at `-m 38` and not at `-m 5` — same
program, same facts, different derived set — so a *budget* would change what is
provable. The same goes for `-T`, `-E` and `--jobs`. Ein already has a place
where the search's state is read and reported, and it is the **verdict**, which
is downstream of every derivation and is allowed to depend on how much of the
lattice was walked. That separation is the design; (b) erases it.

[design/08 §7](../../../docs/history/m1a_rust/design/08_parallelism.md) rejected
parallel depth-first on exactly this ground, and
[Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
is the shape of the decision it takes to move any of it deliberately.

### (c) A verdict atom — *refuted on evaluability, which is the new one*

The reading the stage plan did not list, and the one form G invites: if
`(open ?R)` is a rule-asserted atom the engine tallies at quiescence, why not
`(closed)`?

Because **a verdict atom is only worth having if the engine can evaluate the
claim it makes**, and the two differ exactly there:

| atom | what the engine must do to honour it | cost |
|---|---|---|
| `(open ?R)` | one pass over the quiescent KB, per obligation instance | **46 instances** on `zebra2-minus-15`, read after the fixpoint |
| `(closed)` | exhaust the lattice | **17 204 592 enterings, 24 min 56 s at `-j16`** on the same file ([the milestone's opening measurement](README.md#acceptance-for-the-milestone)) |

So even in the shape that worked one level down, the atom would be an assertion
the engine cannot afford to evaluate — and one whose evaluation is the very
thing the phase exists because nobody can afford. **The affordability problem
is not downstream of the vocabulary; it is the reason the vocabulary cannot
exist.**

---

## 3. The neighbours — four systems, and the nearest miss

Q-M1d.7's prior is that no language of this family lets a program constrain its
own model count. Checked rather than repeated, because *"nobody does this"* is a
claim about a literature and the repo has most of that literature catalogued.

| system | the mechanism that looks like it | what it actually quantifies over |
|---|---|---|
| **ASP** / clingo | `#count`, `#sum` aggregates; `#minimize` | aggregates count atoms **within** an answer set; optimisation *ranks* answer sets from outside the program's logic. `1 { p(X) : q(X) } 1` — the note's own `L ≤ # ≤ U` — is a cardinality bound on atoms in **one** answer set ([`docs/lib/02` § ASP, § Clingo](../../../docs/lib/02-solvers-csp-sat-smt.md)) |
| **Alloy** | `run p for 5` / `check a for 5 but 3 Node` | a **scope** — a bound on how large a universe the analyser searches — and a *command*, not a sentence in the model. `#r` counts tuples **inside** an instance ([`docs/lib/03` § Alloy](../../../docs/lib/03-theorem-proving-formal-methods.md), added by this stage) |
| **SMT** / Z3 | the blocking-clause enumeration loop | a **procedure outside the formula**; the formula never mentions how many models it has. [M10](README.md) owns the encoding that would run it |
| **#SAT** / projected model counting | the count itself | an operation **on** a program, which is the meta level the question is asking about |

**Alloy is the nearest counterexample and it is not one.** It is the only one of
the four whose *bound* appears in the file the modeller writes, which is why it
looked like a case of a program constraining its own model space. It is not: a
scope is a search budget the analyser is given, so **the ein analogue of `for 5`
is `--max-set-size 5`, not `:expect`.** Ein already has Alloy's mechanism, in
Alloy's position, spelled as a flag.

The pattern across all four is the finding: **every one of them puts the count
at the meta level**, and the two that come closest to putting it in the program
— Alloy's scope, clingo's `--models`/`--enum-mode` — put it on the **command
line**, which is exactly where ein's is. Nothing contradicts the prior.

**One catalogue gap, closed.** Alloy was not in
[`docs/lib/`](../../../docs/lib/README.md) at all, which is a gap found by
*using* the catalogue and therefore the catalogue's own maintenance rule; it is
now in `03` § 5. The knowledge graph is deliberately **not** touched: it is a
curated subset rather than an index — `Curry–Howard`, `Natural deduction`,
`Monte Carlo Tree Search`, `Constraint Programming` and a dozen others are
catalogued without a node — and
[AGENTS.md](../../../AGENTS.md)'s re-render rule fires when
`knowledge-graph.dot` changes, not when an entry is added.

---

## 4. The boundary, stated once

> **A rule is a sentence about the world it fires in.** A claim about the *set*
> of worlds is a sentence about the search, and the search is not a thing rules
> may read — because a rule that read it would make derivation depend on the
> traversal, and the traversal is a budget. So closure claims live at the
> **meta** level; `:expect` is already that level; and the question is not
> *where to put the claim* but *what the meta level can afford to check*.

Three corollaries, and they are why this is written once rather than per
keyword:

1. **It settles every second-order claim.** *"exactly one model"*, *"an even
   number of models"*, *"the same models as that other file"*, *"at most k
   models"* — none needs re-litigating. Each is a sentence about the lattice of
   KBs, none is a property of any node in it, and (b)'s refutation applies to
   all of them identically.
2. **It does not forbid a runner knob.** *How deep to search* is not a
   second-order claim at all — it is Alloy's scope, and it belongs where
   `--max-set-size` already is. If a program ever needs to say *"check me at
   depth 38"*, that is a budget declaration and this boundary has no opinion on
   it.
3. **It does not weaken `:expect`.** The meta level is allowed to state what it
   cannot verify, provided it says so:
   [`Outcome::NotChecked`](../../../ein.rs/crates/ein-infer/src/expect.rs) is
   that provision, it takes a failing exit code, and
   [S1d.4.1](closure_census.md) measured that **no corpus claim is in that
   state** while **ten entries would be if one were written**.

---

## 5. What it hands forward

[S1d.4.3](README.md#s1d43--the-vocabulary) inherits a question that is now smaller
and better posed. It is not *"where does the closure claim live"* — it lives at
the meta level and `:expect` is the meta level. It is:

- what the meta level can **afford** to check ([the closure census](closure_census.md)'s
  four tables are the input),
- what it **says** when it cannot ( `NOT CHECKED` today, and whether the exit
  code and the stream that carries it are right),
- and whether M1c's pipeline sentence, which promises `ein test` re-checks
  `zebra2-minus-15`'s 32 models, is honoured or rewritten.

**None of those is a grammar question**, which is the practical value of
closing Q-M1d.7 before them.
