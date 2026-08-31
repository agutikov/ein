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
> - driving Ein from your own program — [`docs/api/rust.md`](../api/rust.md)
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

Build the engine once, then every snippet is runnable:

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml -p ein-cli
export PATH="$PWD/ein.rs/target/release:$PATH"

ein solve <file>                    # solve a puzzle and print the answer
ein render rule --name <R> <file>   # draw a rule as a graph (DOT)
```

The build needs `cmake` and a C++ compiler for the bundled allocator; add
`--no-default-features` to build against the system one instead.

## Which blocks are generated, and which are not

**Chapters 2 and 4 show real output; nothing else on these pages is a
transcript.** Chapters 1 and 3 carry no command at all, so the sentence that
used to stand here — *"each chapter ends with the exact command to reproduce
what it shows"* — was true of half the guide (M1e `CD-L2`).

Of the four blocks that do show output, one is **generated** and three are
hand-maintained **excerpts**, and the difference is worth knowing before you
edit either kind:

| where | kind | what keeps it true |
|---|---|---|
| [ch. 4](04_solving_the_whole_puzzle.md) § Solve it | **generated** — the whole of what the binary prints | `ein-cli/tests/guide_transcripts.rs`, which runs the command in the marker and diffs. *Edit the test, run it, paste; never edit the block by hand* — or `EIN_BLESS=1` writes it |
| [ch. 2](02_first_rules.md) ×3 | **excerpt** — the two or three lines the section is about, with the header, the rule and the empty `query bindings` elided | nothing, deliberately: an exact diff cannot express an elision, and pinning them would push seven lines of `(query has no :goal-text template)` into a tutorial to satisfy a test. Their lines are byte-correct as excerpts |

Chapter 4's block was **fabricated rather than drifted** — it packed four
sorted bindings two-per-line and dropped the rule under the title, and no
engine in this repo's history has printed that. A pin catches drift; only
taking the bytes from a run catches fabrication, which is why the block is
generated now and why this table says which kind each one is.
