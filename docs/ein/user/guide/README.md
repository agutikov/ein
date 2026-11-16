# Learn Ein by solving the Zebra puzzle

A hands-on tutorial: you learn Ein by building up the classic **Zebra (Einstein) puzzle** from scratch, objects and
facts first, then rules, then the whole puzzle solved end to end.

> **Audience: newcomers.** This is a *tutorial*, learned by example. It is **not** a reference, and it does **not**
> explain how the engine works inside: it shows Ein from a puzzle author's seat and links out when you want depth:
> - the language, form by form: [the syntax reference](../reference/syntax.md);
> - what each concept means, in Ein, as a graph, in words and in mathematics:
>   [syntax and semantics](../syntax-and-semantics/README.md);
> - what the answer is and every row `ein solve` prints: [solutions and models](../syntax-and-semantics/solutions-and-models.md)
>   and [the read-out](../reference/read-out.md);
> - driving Ein from your own program: [the Rust API](../../ein.rs/api.md).

## Chapters

1. **[Objects & relations](01_objects_and_relations.md)**: the three things every Ein model is made of (objects,
   relations, facts), each shown three ways: plain English, ein-lang, and as a graph.
2. **[First rules](02_first_rules.md)**: how a *rule* derives new facts: `symmetric`, `transitive`, `co-located`,
   with the graph before and after each one fires.
3. **[The rule families](03_rule_families.md)**: the machinery that cracks the Zebra puzzle: domain- and
   range-elimination, negative-completion, spatial adjacency, disjunctive-prune, and which rules you *import* and
   which you *write*.
4. **[Solving the whole puzzle](04_solving_the_whole_puzzle.md)**: put it together, run `ein solve`, read the
   answer.
5. **[Jack drinks coffee](05_jack_drinks_coffee.md)**: one small inference stated four ways, in words, in Ein, as a
   graph and in mathematics: an instance inherits the relations of its type.
6. **[The Zebra puzzle, step by step](06_zebra_walkthrough.md)**: the human solution, line by line, against what Ein
   derives on the same puzzle. Read it after chapter 4.

Beside the chapters:

- **[Examples](examples.md)**: worked fragments of ein-lang, and the two ways the Zebra puzzle is encoded.
- **[Word problems](word-problems.md)**: school arithmetic and the problems beyond it, solved with the standard
  library's calculator.
- The puzzle files themselves are under [`examples/`](../../../../examples/) (its
  [catalogue](../../../../examples/README.md)); `zebra2.ein` is the one this guide builds toward.

## Running the examples

[Install `ein`](../install.md) or build it from a checkout, then every command on these pages runs from the
repository's root, and every program block that writes no `…` runs as written:

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml -p ein-cli
export PATH="$PWD/ein.rs/target/release:$PATH"

ein solve <file>                    # solve a puzzle and print the answer
ein render rule --name <R> <file>   # draw a rule as a graph (DOT)
```

The build needs `cmake` and a C++ compiler for the bundled allocator; add `--no-default-features` to build against
the system one instead.

## Which output blocks are real runs

**Chapters 2 and 4 show what `ein` prints; chapters 1 and 3 run no command.** Of the blocks that show output, the
ones a page marks with an HTML comment that names the command are **generated**: the whole of what the binary
prints for that command, which the test suite runs and compares byte for byte, so the block cannot differ from a run. The rest
are **excerpts**: the two or three lines a section is about, with the header and the other rows elided as `…`. An
excerpt's lines are the binary's, but nothing compares them, because an exact comparison cannot express an elision.

| where | kind |
|---|---|
| [ch. 4](04_solving_the_whole_puzzle.md) § Solve it | **generated**: the whole run of `ein solve examples/zebra2.ein --single` |
| [ch. 2](02_first_rules.md), three blocks | **excerpts** |
| [ch. 6](06_zebra_walkthrough.md), the read-out lines it quotes | **excerpts** |

A generated block is never edited by hand: change the command in its marker, and the block is written again from a
run.

## Which program blocks are read

**Every `lisp` block on these pages loads on today's engine**, and the test suite holds it to that: a block that
writes a form `ein` refuses, a keyword its form does not take or a missing `:why`, fails the tests
([Q-M1f.69](../../../decisions/docs/Q-M1f.69-a-guide-block-that-writes-an-ellipsis-is.md)). A block that writes no
`…` is a **whole program**: it is loaded and solved, and `ein solve` must answer it. A block that writes `…` is an
**excerpt**: every top-level form it writes whole is loaded, together, where it stands, and a form with a `…` in it
is skipped as elided.

What is not read: the prose around a block, a shell command outside a marked transcript, and whether an excerpt
quotes the file it names. Chapter 3's and chapter 4's blocks quote `examples/zebra2.ein`, chapter 2's quote three
files under `examples/`, and each names its file so you can compare.
