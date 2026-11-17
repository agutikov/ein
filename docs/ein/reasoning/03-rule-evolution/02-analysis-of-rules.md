# Analysis of rules — the rule graph and its strata

A rule set has a structure before any fact is derived from it: which relations each rule reads, which it writes,
and which of its reads stand under an `absent`. Ein computes one such structure for every program, its rules'
**dependency graph**, and from the graph the **strata** and the **negative cycles**. What the graph decides is
whether a stratification exists. It is a diagnostic: it orders nothing and refuses nothing.

## 1. The dependency graph

**Nodes** are relations. A stored negation's relation is a node of its own, `(not R)` beside `R`, and `false` is
the node a rule that concludes `(false)` writes.

**Edges** are read off the rules as root instantiates them. A rule with parameters names no relation until a fact
activates it: `(rule symmetric (?rel) …)` reads and writes `?rel`, and `(symmetric next-to)` makes it a rule over
`next-to`. So the graph is over each rule once per activator root holds, and a rule nothing activates adds no edge.
For each such rule:

- an edge from each relation its pattern reads to each relation its conclusion writes, **negative** where the read
  stands under an odd number of `absent`s and positive otherwise, so a read under `forall`'s two `absent`s is
  positive again;
- a read of a stored negation whose relation is a variable, `(not ?)`, reads every stored negation the graph names;
- a positive edge from the rule's activator relation, the relation named as the rule, whose facts switch it on,
  to each relation it writes.

A guard, `eq` or `neq`, is computed and never stored, and adds no edge.

```lisp
(relation p T :why "…") (relation q T :why "…") (relation r T :why "…") (relation s T :why "…")
(p a)
(rule one () :match (p ?x) :assert (s ?x) :why "…")
(rule g ()   :match (and (s ?x) (absent (q ?x))) :assert (r ?x) :why "…")
```

Its graph has three edges: `p → s` from `one`, and `s → r` and `q →¬ r` from `g`, the last negative.

## 2. Strata

A **relation's stratum** is the greatest number of negative edges on a path into it. It is counted over the
graph's strongly connected components: the relations of one component share a stratum, and only the edges between
components are counted. A **rule's stratum** is the highest stratum of the relations it writes, and the program has
**one more stratum than the highest**.

In the program above `p`, `s` and `q` are in stratum 0 and `r` in stratum 1, so `one` is in stratum 0, `g` in
stratum 1, and the program has two strata. A stratum says that everything a rule reads under an `absent` can be
complete before the rule's conclusions are needed: `q` is complete before `r` is derived from its absence.

## 3. Negative cycles, and their parity

A strongly connected component with a negative edge inside it is a **negative cycle**. **A program has a
stratification exactly when its graph has no negative cycle.** Across a cycle a stratum is still assigned, from the
edges into the component, and the cycle is reported beside it.

A cycle's **parity** is *odd* when some cycle inside the component passes an odd number of negative edges, which is
when the component's relations cannot be split in two so that every negative edge crosses the split and every
positive edge stays on one side; otherwise it is *even*. The parity says what the graph allows of root's fixpoint:

- **even**: several fixpoints may exist, and which one root reaches may depend on the order in which the boundary
  admits what it parked. `examples/features/07_unstratifiable.ein`, `p ← absent q` and `q ← absent p`, has the
  cycle `p, q — derive-p: q →¬ p · derive-q: p →¬ q`: either of `p` and `q` is a fixpoint without the other.
- **odd**: a fixpoint supported at the boundary need not exist, and the one root reaches may depend on the
  boundary's order. `q ← p ∧ absent q` is the shortest: a rule that reads under an `absent` the relation it writes,
  `g: q →¬ q`.

What Ein computes on such a program, a fixpoint supported at the boundary, and what the boundary is, are
[`absent`: negation as failure](../02-kb-evolution/03-absent-and-naf.md)'s.

## 4. What the graph decides, and what it does not

**It is a diagnostic, not a refusal.** A program with a negative cycle loads, is solved and exits as any other.

**It says what the graph allows, never what a run did.** A cycle whose rules never fire changes nothing:
`examples/syntax/rule-guards.ein` writes no fact, so its rule `g`, which reads `s` under an `absent` and writes
`s`, never fires and its one fixpoint is root's empty one, yet its graph has the odd cycle `g: s →¬ s`. The words
the diagnostic uses are *may* and *need not*.

**It orders nothing.** No stratum, and no class of rule derived from a rule's shape, orders the agenda or the
boundary: their order is [Saturation](../02-kb-evolution/02-saturation.md)'s, what leads to `(false)` first and then the
written and arrival order ([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).
On a monotone program the order decides no answer, and a dead node's core, which it may decide, is an explanation
([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).

**It is read at root, on every program.** The monotonicity check asks another question
([Monotone and non-monotone programs](../02-kb-evolution/08-monotonicity.md)): whether a commitment can change what an
`absent` reads, over the relations a commitment can change. Its cycles are cycles of this graph restricted to those
relations. A negative cycle among relations no commitment changes leaves a program monotone by that check, since
every node reads it alike, and still makes root's own fixpoint possibly one of several; this diagnostic is the one
that sees it.

## 5. Where it is shown

- **`ein solve`** prints a `strata` row under `lattice` only on a program with no stratification: how many cycles
  pass through an `absent`, and that root's fixpoint may depend on the boundary's admission order and need not be
  unique. A `negative cycle` row under it names each cycle: its relations, the rules on its negative edges, and
  what its parity allows. (`cycle`, without *negative*, is the monotonicity check's word for its own cycles.) The
  rows are [`08-read-out.md`](../../user/04-reference/08-read-out.md)'s.
- **`ein saturate --dump`** prints each rule's stratum beside it, and one `;; Strata` line: *stratified, k strata*,
  or *not stratifiable* with the same cycles.
- A stratified program prints no `strata` row.

## 6. The other readings of a rule set

The graph is one of several things Ein reads off a program's rules without deriving a fact. Each is stated on the
page of what it serves:

- **the monotonicity check**, whether a commitment can change what an `absent` reads, and **forbid**, the refusal
  of a refutation that could rest on such a read: [Monotone and non-monotone programs](../02-kb-evolution/08-monotonicity.md)
  and [`absent`: negation as failure](../02-kb-evolution/03-absent-and-naf.md);
- **the alive-set invariant**, that no rule asserts a name the program lacks:
  [Saturation](../02-kb-evolution/02-saturation.md);
- **what a rule can bring a commitment to owe**, read off the obligation rules, the second rung of the
  hypothesis ladder: [Hypothesis generation](../02-kb-evolution/04-hypothesis-generation.md);
- **the hypothesis mechanisms judged against one another**, an hrule, an obligation, the `(solve …)` lists and
  `(__closed__ R)`
  ([Q-M1f.245](../../../decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md));
- **the rule shapes refused at load**, a conclusion's variable nothing binds, a rule that reads nothing, a rule
  that could derive an activator binding a relation parameter to a fact:
  [`07-diagnostics.md` § 2](../../user/04-reference/07-diagnostics.md#2-errors-and-exit-codes).

What Ein does *to* a rule set, rather than reads from it, is
[Transformations of a rule set](03-transformations.md)'s.

## What holds it

| claim | test |
|---|---|
| a read under an `absent` puts what the rule writes one stratum higher; a rule that reads under an `absent` the relation it writes is an odd negative cycle; each rule's stratum is the highest of what it writes | `ein-infer/tests/strata.rs`: `an_absent_read_is_a_stratum_and_a_self_read_is_a_negative_cycle` |
| `features/07_unstratifiable.ein` has a negative cycle, and `features/01_not_and_absent.ein`, an `absent` over a relation no rule derives, has none | `ein-infer/tests/strata.rs`: `the_unstratifiable_fixture_has_a_negative_cycle_and_its_sibling_none` |
| `ein solve` names each negative cycle and its parity as a possibility, still answers and exits 0; a cycle that never fires is reported in the same words; a stratified program prints no `strata` row | `ein-cli/tests/strata_diagnostic.rs`: `solve_names_each_negative_cycle_and_still_answers` |
| `ein saturate --dump` says whether a stratification exists | `ein-cli/tests/strata_diagnostic.rs`: `saturate_dump_says_whether_a_stratification_exists` |
| the engine answers a program with an odd cycle, with a fixpoint supported at the boundary | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| the agenda's order and the written order alone give every question and every claim the same answer on every monotone program | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
