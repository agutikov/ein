# Learn Ein by solving the Zebra puzzle

A hands-on tutorial: you'll learn Ein by building up the classic
**Zebra (Einstein) puzzle** from scratch — objects and facts first, then
rules, then the whole puzzle solved end to end.

> **Audience: newcomers.** This is a *tutorial*, learned by example. It is
> **not** a reference and it does **not** explain how the engine works
> inside — it shows Ein from a puzzle author's seat and links out to the
> reference docs when you want depth:
> - the language reference — [`docs/kernel/ir/03-ein-lang/`](../kernel/ir/03-ein-lang/)
> - what Ein reasons over (the graph) — [`docs/kernel/ir/01-ein-graph/`](../kernel/ir/01-ein-graph/)
> - driving Ein from Python — [`docs/api/`](../api/)
> - how the engine searches — [`docs/kernel/inference/`](../kernel/inference/)

## Chapters

1. **[Objects & relations](01_objects_and_relations.md)** — the three
   things every Ein model is made of (objects, relations, facts), each
   shown three ways: plain English, ein-lang, and as a graph.
2. **[First rules](02_first_rules.md)** — how a *rule* derives new facts:
   `symmetric`, `transitive`, `co-located`, with the graph before and after
   each one fires.
3. **[The rule families](03_rule_families.md)** — the machinery that
   actually cracks the Zebra puzzle: domain-elimination, disjunctive-prune,
   spatial adjacency, negative-completion — and which rules you *import*
   vs. which you *write*.
4. **[Solving the whole puzzle](04_solving_the_whole_puzzle.md)** — put it
   together, run `ein solve`, read the answer, and hand off to the full
   deductive trace.

## The two views that go with this guide

- The complete, step-by-step **human solution** —
  [`docs/kernel/inference/zebra_walkthrough.md`](../kernel/inference/zebra_walkthrough.md).
  This guide *teaches the pieces*; the walkthrough *shows them all firing*
  on the real puzzle. Read it after Chapter 4.
- The puzzle files themselves — [`examples/`](../../examples/)
  ([catalog](../../examples/README.md)); `zebra2.ein` is the one this guide
  builds toward.

## Running the examples

Install (`./venv_install.sh`), then every snippet is runnable:

```sh
ein solve <file>          # solve a puzzle and print the answer
ein render rule --name <R> <file>   # draw a rule as a graph (DOT)
```

Each chapter ends with the exact command to reproduce what it shows.
