# Ein's documentation

Ein is a reasoner for logic puzzles and the problems that look like them: a program states relations, facts and
rules in an S-expression language, the engine saturates the rules to a fixpoint, and the answer is the region R of
every set of hypothesised facts whose state is consistent and owes nothing. These pages describe Ein as it ships, in
three perspectives, each asked of every paragraph:

| perspective | what it holds | the question it answers |
|---|---|---|
| [**user/**](user/README.md) | the language, how to run `ein`, what the answer means and every row it prints, a guide that solves the Zebra puzzle | *what do I write, and what does it tell me?* |
| [**reasoning/**](reasoning/README.md) | what a loaded program means, how its knowledge base grows, what the answer is and why each way of finding it gives the same one | *what is computed, and why is it right?* — true of any implementation |
| [**ein.rs/**](ein.rs/README.md) | the Rust workspace: its crates and modules, the design contracts the code cites, the engine's own read-outs, the library API | *how does this implementation compute it?* |

A paragraph that would still be true of a second implementation is the reasoning's; one that names a crate, a module
or a type is `ein.rs`'s; one about what a program says or what `ein` prints, with no word on how, is the user's.

## Beside them

- [**Decisions**](../decisions/README.md): every question about Ein that has an answer, as that answer, each in a
  file under its id. A page cites a decision by its id rather than restating why.
- [**The standard of proof**](standard-of-proof.md): what it takes to call a claim on these pages settled.
- [**Overview**](overview/README.md): how Ein relates to other reasoning systems, and the lineage of its language.
- [**Demo**](../demo/README.md): the article and the talk.

## Where to start

- **To write and solve a puzzle**: [the guide](user/02-guide/README.md), then [install](user/01-install.md) and
  [the command line](user/04-reference/05-cli.md).
- **To read an answer**: [the read-out](user/04-reference/08-read-out.md), every row `ein solve` prints and what it counts.
- **To know what the answer is**: [the solution semantics](reasoning/02-kb-evolution/06-solution-semantics.md).
- **To embed the engine**: [the library API](ein.rs/04-api.md).
