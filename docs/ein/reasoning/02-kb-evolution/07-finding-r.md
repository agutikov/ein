# Finding R: the oracle and the routes

The answer to a program is the region R of its commitments ([Solution semantics § 2](06-solution-semantics.md)). Two
computations give it:

- **the oracle** evaluates the definition: it computes every node of the cube. It is `ein oracle`;
- **a route** computes the same R, or the part of it a question asks about, and on a monotone program skips every
  node that a licence decides without computing it. Every question `ein solve` answers is a route.

The oracle is the reference: every answer a route gives is the oracle's
([Q-M1e2.32](../../../decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)), and each question
does the least work it needs to give it
([Q-M1e2.31](../../../decisions/answer/Q-M1e2.31-each-question-about-r-does-the-least-work-it.md)). This page writes
both down as formulas, states each licence and the argument for it, and answers three questions: what happens to a
candidate that is refuted, how a run proves that R is one state without visiting the lattice, and why nothing a node
derives is written back into root. What a run prints about its route and its work is the read-out's
([`08-read-out.md`](../../user/04-reference/08-read-out.md)).

## 1. A node

**Root** is the program's own knowledge base saturated once: `root = sat(KB₀)`. It is sealed. Every node is a
copy-on-write fork of it, read and thrown away, so nothing a node derives ever reaches root or another node.

**`sat` fires in one order, the agenda**: first what can make a contradiction, a rule concluding `(false)`, a stored
negation, or a fact of a relation whose negation some rule concludes or the knowledge base holds; then the rest; FIFO
in the written and arrival order within each half
([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)). Root, every route's
node, the oracle's and every reader saturate so. On a monotone program the order decides no fixpoint and no answer;
it decides when a fact is derived, so where a dead node stops, its narration, and which derivation a fact's record
keeps first, which a core is explained from. Of a flagged program no answer is claimed.

**H**, the universe of candidates, is built once at root by the generation ladder
([Solution semantics § 1.4](06-solution-semantics.md#14-candidate-and-universe);
[Hypothesis generation](04-hypothesis-generation.md)). It is listed by `ein show hyps FILE`, and its structure is
[The hypothesis space](05-hypothesis-space.md).

For a **commitment** `C ⊆ H`:

```text
st(C)     = sat(fork(root) + C)                 the node's state, saturated to quiescence
dead(C)   ⇔ st(C) holds a contradiction
owes(C)   = the obligation instances st(C) holds with no witness
cl(C)     = st(C) ∩ H                            its closure, C ⊆ cl(C)

R         = { C ⊆ H : ¬dead(C) ∧ owes(C) = ∅ }   alive and discharged
```

**Every route and the oracle read a node by this one definition**, so two of them cannot read one node differently:
they can only visit different nodes. A node's **core**, the facts its contradiction rests on ([Cores](09-cores.md)), is
taken only where a surface prints one.

**What a route computes of a dead node.** `st(C)` above is the definition, and the oracle computes it whole. On a
monotone program a route's node, root's excepted, is saturated **to quiescence, or to its first contradiction where
it is dead**:

```text
st_r(C)   = st(C)                                if st(C) holds no contradiction
          = the fork as it stands after the firing that makes its first contradiction,
            every conclusion of that firing written, or with C written where that
            already makes one                                               otherwise
dead_r(C) ⇔ st_r(C) holds a contradiction  ⇔  dead(C)          the knowledge base only grows
```

So an alive node is computed whole, and a dead one costs only the derivation of its contradiction. No route reads
more of a dead node than `dead(C)`: what it remembers is the set (§ 5), and a dead node's closure, denials and debts
decide nothing. R, N and every count are therefore the oracle's. What a dead node **reports** is `st_r(C)`: its facts,
and its debts and closure where a surface lists visited nodes. **Its core**, where a surface prints one, is taken over
`st(C)`, as the oracle's is: the saturation goes on from the stop, firing what it would have fired, so no core depends
on the stop. A computation that reads more of a dead node than its status computes the node whole: root, the oracle,
a flagged program's nodes, whose detector reads each node's fixpoint, and every reader of a node's derivations.

## 2. The oracle: the cube

```text
for every C ⊆ H:                  2ⁿ nodes, n = |H|
    compute st(C)
    read dead(C), owes(C), cl(C), and its core if dead
R = { C : ¬dead(C) ∧ owes(C) = ∅ }
```

It skips nothing (`L7 the cube`), so on a monotone program it is R by definition. On a program the static check flags
it is the region of the one construction of a node, and no more (§ 3). It also reads off what a description of R
needs:

- N, the minimal dead sets;
- the two lines, minimal and maximal;
- the shape, the holes, and the counts `k_s` (states) and `k_c` (commitments).

Its cost is `2ⁿ` saturations, so it refuses a universe past its limit, `--region-limit`, and says how many
saturations the region would have taken.

**What the oracle does not check.** It computes each node with the engine's own saturation and takes H from the
engine's own generator, so it is the reference every route is held to, not a check of those two: a derivation the
saturation misses, or a candidate the generator never proposes, is missing from the oracle's R as from every
route's. A relation the generator leaves out is named on every surface
([solution semantics § 1.4](06-solution-semantics.md#14-candidate-and-universe)).

## 3. A program the static check flags

A program is **flagged** when the static check cannot prove it monotone ([Monotone and non-monotone
programs](08-monotonicity.md)). On such a program a dead set can have an alive superset, and an alive superset of a
member of R can owe. No skip is sound, and every question takes the cube, as the oracle does (route 2). It takes the
cube in layer order, so a question that can be decided before the cube ends stops there.

The one licence a flagged program keeps is **L0**: a fork only adds, so when root is dead every node is. And one
question needs no licence: **where ∅ ∈ R, the minimal line is {∅}**, since ∅ is below every commitment, and whether
∅ is in R is root's fixpoint alone. So every question about the minimal line is decided by root, one saturation, on
a flagged program as on any other ([Q-M1f.23](../../../decisions/read-out/Q-M1f.23-where-root-decides-a-flagged-programs-minimal.md)).

| question | what it does | what it certifies |
|---|---|---|
| `-n N` | visits commitments in layer order and lists each node of R it meets | each state listed is a member of R, as the oracle's node is. *There are only k < N* needs the cube |
| `--single` | the same, and stops at the second state | `Ambiguity` is certified at once; `Solution` and `Contradiction` need the cube |
| `--minimal`, `--maximal`, describe | the cube | everything, within the oracle's limit. Past it, a part: a member met with no member of R met below it is minimal, every subset having been met first, so the minimal line certified is those members ({∅} where ∅ ∈ R); the maximal line needs the whole cube, so none is certified. The other members met are members of R, counted |

Past the limit a line question certifies what describe certifies, no more
([Q-M1e2.19](../../../decisions/answer/Q-M1e2.19-past-the-limit-a-line-mode-certifies-what.md)).

**On a flagged program nothing is claimed of the answer**: not soundness, not correctness, not completeness.
Saturation there can depend on the path a node is built by,

```text
sat(root + C₁ + C₂)  ≠  sat(sat(root + C₁) + C₂)        possible on a non-monotone program
```

and on the agenda's order, so *the set of nodes* is not one object. Every node is built one way, a fork of root's
fixpoint with all of `C` written at once (`st(C)` above), and the oracle and every question compute the region of
that construction ([Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)). They
agree with each other; that the region is the program's meaning is not claimed.

## 4. A monotone program: three properties and the licences

When no read of a compiled plan can take a conclusion away or add a debt as a commitment grows, three properties
hold:

```text
dead is upward-closed           dead(D) ∧ D ⊆ C            ⇒ dead(C)
discharged is upward-closed     owes(D) = ∅ ∧ D ⊆ C        ⇒ owes(C) = ∅
cl is a closure operator        st(C) = st(cl(C))
```

A **licence** is a theorem that lets a route skip work without changing its answer. Each holds of a class of
program, and the static check decides the class. A route skips a node, or concludes a line, only through a licence;
the read-out's `route` row names the licences a run used.

| licence | statement | what it saves |
|---|---|---|
| **L0** a fork only adds | root dead ⇒ every node dead | the whole search (every program) |
| **L1** dead is upward-closed | `dead(D) ∧ D ⊆ C ⇒ dead(C)` | every superset of a set known dead. A candidate dead alone leaves the universe |
| **L2** discharged is upward-closed | `C ∈ R ∧ C ⊆ C′ ∧ ¬dead(C′) ⇒ C′ ∈ R` | the owing check above a member of R. When ∅ ∈ R, R is the whole alive set |
| **L3** a commitment's state is its closure's | `st(C) = st(cl(C))` | a route visits closed **states**, not commitments, and counts `k_c` without visiting it |
| **L4** the lines are local | `s ∈ R ∧ ∀b ∉ s: dead(s ∪ {b}) ⇒ no member of R ⊋ s`; and a member of R every one-down subset of which owes is minimal | the climb, and the last step of `--single` (§ 6). A line member is certified with at most n saturations |
| **L5** an owed instance's alternatives are exhaustive | at an owing state `C` owing a licensed instance `I` with alternatives `A(I)`: `R ∩ ↑C = ⋃_{a ∈ A(I)} R ∩ ↑cl(C ∪ {a})` | branching on one instance's alternatives instead of on every extension. An alive node owing an instance whose every alternative is dead has no member of R above it |
| **L7** the cube | the definition | nothing: `2ⁿ` saturations decide everything. The oracle, and route 2 |
| **L8** what a state denies is dead above it | `(not b) ∈ st(S) ⇒ dead(S ∪ {b})` | a saturation for each candidate a state's own rules have already denied |
| **L9** a fact nothing proposes and nothing writes is in no state | a witness `w` of `I` with `w ∉ H`, `w ∉ st(∅)`, of a relation no plan writes and the kernel does not mirror, is in no `st(C)`; so where every other witness of `I` is a candidate or denied at root, L5's equation holds with `A(I) = H ∩ witnesses(I)` | branching on every extension at a state owing an instance L5 alone does not license |

On a program with restrictions only, root alive gives ∅ ∈ R, so the minimal line is {∅} in one saturation; with
obligations only, the maximal line is {⊤} on a monotone program: these are the **trivial lines**
([Solution semantics § 2.4](06-solution-semantics.md#24-shape)), read off the program's shape and never enumerated.

**L5's argument.** At a state C owing instance I, let D ⊇ C be a member of R. On a monotone program I's trigger reads
only relations no commitment changes, so I is an instance at D too, and D is discharged, so a guard of I holds there.
The premise: every read of that guard beside its witness step reads such a relation, so the guard holds at D exactly
when a witness fact w is in D's fixpoint, and w is one of the facts the guard admits at root; and each of those is a
candidate, or has its negation stored at root, which no alive fixpoint can hold. So w is a candidate in D's closure
and not in C's, D's state is above `C ∪ {w}`, and if that node is dead, so is D (L1, L3). The premise is needed: a
guard that reads a relation a commitment can grow could be paid by a fact already in C. Why an obligation's
alternatives are jointly exhaustive in general is [Satisfiability](10-satisfiability.md).

**L8's argument.** `st(S ∪ {b}) ⊇ st(S)` holds `(not b)`, and its commitment holds `b`: it is dead.

**L9's argument.** A fact enters a state only by being committed, by being in root, or by a rule or the kernel
writing it. A witness that is no candidate, is not in root, and is of a relation no plan writes and the kernel does
not mirror, enters none, so the alternatives of the instance are its witnesses in H.

**Each licence's premise is checked, not assumed.**

- L5's and L9's premises are read at load, instance by instance: an instance root owes is licensed only where its
  premise holds, and the reason is named where it does not. Each is checked again on every node a route computes,
  and a node that breaks it refuses the run.
- A route that meets a state contradicting what it knows (an alive set it holds dead, a member of R with an owing
  extension, a node owing an instance root did not) refuses the run. A refusal is an engine defect, never an answer.

## 5. What a run knows dead, and what happens to a refuted candidate

Every route on a monotone program asks one memory before it saturates anything:

```text
known dead:   D            for every set D a saturation found dead
              S ∪ {b}      for every state S met whose own fixpoint stores (not b)     (L8)

before computing C:   if some known-dead set D ⊆ C   ⇒ C is dead, not saturated     (L1)
```

**A refuted candidate.** Suppose `{h1}` is computed and found dead.

- **Nothing is written into root.** The fork that found the contradiction is thrown away, and root holds no
  `(not h1)`.
- **`{h1}` goes into the run's memory.** Every commitment that holds `h1` contains `{h1}`, so it is dead by L1 and
  never saturated, and no state is ever extended by `h1`. For the rest of the run the search is over `H ∖ {h1}`.
- **The read-out names it** among the candidates dead alone.
- **In general, a dead set D removes all of `↑D`**, the sets above it, from the search.

**Why a refutation is a constraint on commitments and not a fact in root:**

1. **It would change the answer.** R is defined over `st(C) = sat(root + C)`. A `(not h1)` in root would let every
   rule that reads the negation fire in every other node, so states, and with them R, could differ from the
   definition.
2. **It is only sound on a monotone program.** On a flagged program `{h1}` dead says nothing about the sets above it.
3. **Every route and the oracle must read the same node.** They can only if every node is a fork of one root that
   nothing changes.

**What the memory keeps of a death** is the set that was computed and found dead, not the smaller set its
contradiction rests on. A denial is the other source: a state's own rules store `(not b)` (domain elimination, for
instance), and L8 reads it with no saturation. It is a refutation of `b` *relative to that state*.

## 6. The routes, question by question

Each question does the least work it needs.

**Describe** (a bare `ein solve FILE`), and `--count` by the same route, has **two routes** on a monotone program;
`--describe-route` chooses between them, and no part of R depends on which
([Q-M1f.311](../../../decisions/answer/Q-M1f.311-describe-takes-the-tree-and-the-classes.md)).

**The state route**, every alive state once:

```text
visit the alive states by size, from cl(∅):
    for each state S, for each b ∈ H ∖ S:
        if S ∪ {b} is known dead          skip                                 (L1, L8)
        elif S ∪ {b} is a known state's   read it, no saturation               (L3)
        else                              compute st(S ∪ {b})
k_c(T) = 2^|T| − Σ_{S ⊊ T, S an alive state} k_c(S)                            (L3)
N      = the minimal transversals of { H ∖ M : M a maximal alive state }         (L1)
```

N, the merged states, the two lines and an empty R's owing frontier, its maximal alive states, are read off the
states it met.

**The tree and the classes**, the default where root owes an instance a licence takes:

```text
R's states  = the leaves of tree(cl(∅)) exhausted, and every alive state above them   (L5, L9, L2, L3)
dead alone  = { b : cl(∅) ∪ {b} is dead }
for each state s of R, its class  class(s) = { C ⊆ s : cl(C) = s }:
    M(s) = the maximal closed proper subsets of s        dualise and advance inside 2^s, cl the oracle
    generators(s) = the minimal transversals of { s ∖ m : m ∈ M(s) }
    |class(s)| = the subsets of s that meet every s ∖ m, m ∈ M(s)
k_c = Σ_s |class(s)|
the minimal line = the minimal sets among every state's generators     (∅ where root is in R)
the maximal line = the states of R every extension of which is dead     (L4)
```

Dualise and advance inside `s` asks each minimal transversal `T` of the complements known: where `cl(T) = s` it is a
generator, and otherwise `cl(T)` is climbed inside `s` to a new maximal closed proper subset and the transversals are
taken again; when every transversal generates `s`, `M(s)` is every maximal closed proper subset (a missing one would
hold a transversal that does not generate). Its seed is the closed subsets of `s` the run met, and where every one of
them was extended by every candidate it is complete and nothing is asked. **It does not compute N**, the border of
*alive*, nor an empty R's owing frontier, the maximal alive sets: where root owes, that border can be far larger than
the alive states the state route reads it off. The read-out says so, with the members met: the candidates dead alone,
and the owing states met with every extension dead.

Where root is in R, owes no instance a licence takes, or there is no candidate, the tree has nothing to branch on:
it would visit what the state route visits, and describe takes the state route.

**`--single`, `-n N`, `--minimal`, `--maximal`: the obligation tree.**

```text
tree(C):   if C is known dead or dead(C)              → nothing below C
           elif owes(C) = ∅                            → a leaf: C is a state of R
           elif C owes a licensed instance I           → children cl(C ∪ {a}), a ∈ A(I)   (L5, L9)
           else                                        → children cl(C ∪ {b}), b ∉ C
```

By L5, every member of R lies above some leaf of the tree once the tree is exhausted.

- **Which instance a state branches on** decides the work and never the answer. A route may order the owed instances
  by the coupling of [The hypothesis space](05-hypothesis-space.md), and the `route` row says in words which order a run
  took.
- **`-n N`** lists the tree's leaves, then the states of R above them (L2).
- **`--minimal`** shrinks each leaf to a minimal commitment that stays discharged, and gets the rest of the line by
  duality: the minimal commitments of R are the alive minimal transversals of the complements of the maximal owing
  states.
- **`--maximal`** climbs a leaf to a maximal state (L4), then finds the others by *dualise and advance*.

**Why each step is sound**, one line each:

- **The climb** adds a candidate whose extension is alive until none is, and ends at a node of R whose every one-up
  extension is dead, which L4 makes maximal.
- **The obligation tree**: every member of R above an owing state holds one of the branched instance's alternatives
  (L5), so the branch loses none, and a state whose every alternative is dead has none above it. Every minimal state
  of R is a leaf: the chain of closed sets below it owes at every step.
- **Dualise and advance**: a maximal state above a seed σ that is not found yet holds σ and a minimal transversal of
  the found maxima's complements. So when every such transversal is dead, the maxima above σ are all found, and
  every maximal state of R lies above a minimal one.
- **The shrink**: a subset of a commitment of R is alive (L1). If dropping x fails once, it fails on every smaller set
  (L2), so one pass ends at a minimal commitment.
- **The minimal line by duality**: an alive set is outside R exactly when an owing state holds it (L2), so the
  minimal commitments of R are the alive minimal transversals of the maximal owing states' complements.
- **A budget** is checked before each step. A run it stops has its certified part and its bounds, and nothing
  half-done: it reports a part of R, never a different R
  ([Q-M1d.1](../../../decisions/answer/Q-M1d.1-a-search-stops-early-only-on-a-decided-question.md)).

## 7. Proving that R is one state without the lattice

This is `--single`'s question, *is R one state?*

```text
if ∅ ∈ R:   s = cl(∅)                                       root's own state
else:       run the tree; stop at the second distinct leaf  → not one state
            if no leaf                                      → R is empty
            if exactly one leaf state s                     → R ⊆ ↑s          (L5)

then:       for each b ∈ H ∖ s:
                if s ∪ {b} is alive                         → it is in R (L2), another state:
                                                              not one state
            all dead                                        → every proper superset of s is
                                                              dead (L1), so R = { C : cl(C) = s }:
                                                              one state, proved                (L4)
```

**What the proof costs** is the tree's nodes and `s`'s one-up extensions, less every set the memory of § 5 decides
without a saturation: the size of the search tree, not of the cube. On a flagged program none of this holds. A dead
set can have an alive superset, so the only proof is the cube.

## 8. Why two routes cannot disagree

By construction:

1. **There is one node function.** Every route evaluates a commitment as the oracle does, `sat(root + C)`, from the
   one sealed root, saturation alone and no write. A route and the oracle therefore cannot disagree about a node;
   they can only visit different nodes.
2. **A route skips a node, or concludes a line, only through a licence** that the program's static class proves. A
   flagged program gets no licence beyond L0 and the cube, so its routes are certified within the oracle's limit and
   report a part past it.
3. **No option, lever or flag changes R**
   ([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)): an option
   chooses how much work a question does and which route does it, never which nodes are in R.

So every state a run lists is a member of the oracle's R, and a line, a count or a verdict it reports as certified
is the oracle's. What makes that a fact rather than an argument is the tests below.

## What holds it

| claim | test |
|---|---|
| every question gives the oracle's R on every monotone and flagged cube within the limit, describe and `--count` by both routes | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| describe's two routes give one R, every field both compute, on every corpus program; each is the oracle's; the default takes the tree where it branches | `ein-infer/tests/tree_classes.rs`; `ein-cli/tests/lever_invariance.rs` |
| the state route is the oracle on every monotone program it reaches | `ein-infer/tests/region.rs`: `the_state_route_is_the_oracle_on_every_monotone_program_it_reaches` |
| L5's premise is read as stated, and a licensed instance is owed exactly when none of its alternatives is in the state, on every monotone cube | `ein-infer/tests/region.rs`: `l5_is_the_oracle_on_every_monotone_program_it_reaches`, `l5_licenses_what_its_premise_holds_of_on_the_zebra_family` |
| L9 licenses where nothing writes the witness relation, gives every question the answer it has without it, and a node that breaks its premise refuses the run | `ein-infer/tests/unheld.rs` |
| a denial kills exactly where a computed node would, on every monotone cube; the run with and without the memory's denials gives one answer | `ein-infer/tests/known_dead.rs` |
| a route's dead node stopped at its first contradiction gives every question the answer the node run to quiescence gives | `ein-infer/tests/first_contradiction.rs` |
| the agenda gives every question and every claim the answer the written order alone gives, on every monotone program | `ein-infer/tests/agenda.rs` |
| every order the tree can take gives the oracle's answer | `ein-infer/tests/explore.rs`: `every_order_of_the_tree_is_the_oracle`; `ein-infer/tests/order.rs` |
| each claim a program states is decided alike by the question `ein test` asks, by describe and by the oracle | `ein-infer/tests/claim_routes.rs` |
| no lever, option or flag changes R | `ein-cli/tests/lever_invariance.rs` |
| where ∅ ∈ R every minimal question stops at root, on a flagged program too | `ein-cli/tests/region_modes.rs`: `where_empty_is_in_r_every_minimal_question_stops_at_root` |
| past the limit a line question certifies what describe certifies | `ein-render/tests/line_modes_past_the_limit.rs` |
| the oracle refuses past its limit | `ein-cli/tests/region_modes.rs`: `the_oracle_refuses_past_its_limit` |
