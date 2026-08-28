# D2 — Q6: which decline condition should the probe construct?

**Blocks:** [T1e.1.1.4](README.md#task-t1e114--q6-try-to-build-the-inner-node-rung-flip).
**Decides:** what [S1e.2.1](../../p1e.2_high/s1e.2.1_correctness.md) T3 does
about [CO-H3](../../README.md#the-findings)(c) — a `debug_assert`, a re-probe
per node, or a hard decline.

## What is already settled

The tree probes the generation rung **once at root** and keeps the answer
([`solve.rs:894-914`](../../../../ein.rs/crates/ein-infer/src/solve.rs)), on
the stated premise that *"the mode is a property of the program rather than
of the node, so asking once is asking enough."*

**That premise is already refuted by this repo's own doc comment.**
`activators_for`
([`compile.rs:54-69`](../../../../ein.rs/crates/ein-infer/src/compile.rs)):

> A parameterised one consults the **fork's** `rule_apps_by_rule`, not the
> load-time KB's, because a fork derives activators of its own during
> saturation (the stats-determinism violation S1.5a.2a tracked was a direct
> consequence of reading the wrong one).

and `oblgen::generate` calls `plans_for(s.kb, …)` per node. So the mode **is**
a function of the node's facts. Nothing left to argue there.

## What is still open

Whether a *flip* is constructible. Rung 2's gate is
`!program().obligations.is_empty()` — static — so the flip has to come from
`generate` returning `Declined`, and there are exactly three fact-dependent
ways
([`oblgen.rs:232-262`](../../../../ein.rs/crates/ein-infer/src/oblgen.rs)):

| # | condition | what the probe needs |
|---|---|---|
| **1** | a bare `(open)` plan — no relation to branch on | an obligation rule asserting `(open)` whose activator is derived only in a fork |
| **2** | a projection that will not resolve for that activator | the same, with a head shape `project()` returns `None` for |
| **3** | **C4** — an accepted obligation *scans* a relation the rung itself proposes | a derived activator adding an obligation on `S`, where an existing obligation's guard reads `S` |

Sketch for **3**, the likeliest — root has one obligation on `r`; committing
a hypothesis derives `(marker …)`, which activates a second obligation whose
relation is scanned by the first:

```lisp
;;; SKETCH — not run. The shape, not the syntax.
(rule owe-r (?isa ?D)
  :match  (and (?isa ?a ?D) (absent (and (?isa ?b ?D) (r ?a ?b) (s ?b))))
  :assert (open r))            ; guard scans `s` …

(rule owe-s (?isa ?D)          ; … and this rung proposes `s`
  :match  (and (marker ?D) (?isa ?b ?D) (absent (s ?b)))
  :assert (open s))

(rule derive-marker ()         ; the activator, derived only under a hypothesis
  :match  (r ?a ?b)
  :assert (marker T))
```

At root there is no `(r …)`, so no `(marker T)`, so `owe-s` has no activator
and the rung reports `Obligations` — the tree accepts. One commitment later
the marker exists, `owe-s` gets a plan, C4 sees `owe-r` scanning `s`, and the
call **declines** — falling through to the blind enumerator with no event
`tree_node` reads and no assert.

## The loss mechanism — and why the assertion is not a count

The finding says the tree "would treat a non-exhaustive branch set as
exhaustive and miss models". Reading `tree_node`, that is not the first thing
that happens: the tree enters **every** candidate the blind rung returns and
recurses, because `one_branch` is a parameter the blind rung ignores. What
you get first is the `d!`-per-path walk P1.5b deleted.

The **missed model** comes from `complete` changing meaning at that node. A
node whose obligations are all discharged is a solution under rung 2 and is
*not* complete under rung 3 while the blind enumerator still proposes
anything — and `branching/06` is the standing proof that it proposes junk long
after any real debt is settled ([D8](d8_branching06_untyped_models.md)).

So the test asserts two things, not one:

1. `tree_models ⊇ lattice_models`, compared **fact for fact** (the
   [`tree_traversal.rs`](../../../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)
   idiom — *"never of `k`"*), and
2. **no node emitted a `rung` event with a mode other than `obligations`**.

(2) is the one that survives even if (1) happens to pass by luck.

## Options

| | probe | consequence |
|---|---|---|
| **A** | condition 3 (C4) only | one fixture, ~half a day. Likeliest to fire and closest to a shape a real program could reach |
| **B** | all three conditions, one minimal fixture each | ~1 day. Settles *constructible at all* rather than *constructible one way*, which is exactly what the `debug_assert`-vs-re-probe choice turns on |
| **C** | condition 1 (bare `(open)`) only | simplest to write, proves constructibility fastest, weakest as a shape anyone would ship |
| **D** | skip construction; take the fix and a gate test | cheapest and arguably sufficient — the re-probe is a few lines and the tree already pays a full generation call per node, so the marginal cost is near zero. But it closes a **risk** finding by argument, which [Q-M1e.1](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted) forbids |

**Recommended: A, then apply D's fix regardless of whether A fires.** The
fixture is what makes the guard removable by a later milestone; the fix is
what makes the guard exist.

## What each outcome means for `CO-H3`(c)

| outcome | S1e.2.1 T3 |
|---|---|
| constructible, and the tree misses models | re-probe at every node, hard-decline on a flip; the probe is the regression test |
| constructible, and the tree still agrees | find out why, then the assert or a written argument |
| not constructible from any `.ein` program | `debug_assert` at `solve.rs:894` plus the reason, stated there |
