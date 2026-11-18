# Ein's documentation

Ein is a reasoner for logic puzzles and the problems that look like them: a program states relations, facts and
rules in an S-expression language, the engine saturates the rules to a fixpoint, and the answer is the region R of
every set of hypothesised facts whose state is consistent and owes nothing. These pages describe Ein as it ships, in
three perspectives, each asked of every paragraph:

| perspective | what it holds | the question it answers |
|---|---|---|
| [**user/**](user/README.md) | the language, how to run `ein`, what the answer means and every row it prints, a guide that solves the Zebra puzzle | *what do I write, and what does it tell me?* |
| [**reasoning/**](reasoning/README.md) | what a loaded program means, how its knowledge base grows, what the answer is and why each way of finding it gives the same one | *what is computed, and why is it right?* — true of any implementation |
| [**ein.rs/**](ein.rs/README.md) | the Rust workspace: its crates and modules, its algorithms, the design contracts the code cites, the engine's own read-outs, the library API, and for each page of the reasoning where the code implements it and which test holds it ([reasoning in code](ein.rs/08-reasoning-in-code/README.md)) | *how does this implementation compute it?* |

A paragraph that would still be true of a second implementation is the reasoning's; one that names a crate, a module
or a type is `ein.rs`'s; one about what a program says or what `ein` prints, with no word on how, is the user's.

## The words every page uses

A knowledge base is a graph, and an **object** is a node of it: an **atom**, a labelled node with no outgoing arrow
whose label is its **name**, or a **fact**, a node with ordered arrows, the first at its **relation**, an atom, and
the others at its arguments. An object is identified by its name or by its tuple. A relation's **extent** is every
fact of it, held or not, and what a knowledge base holds of it is the **held extent**. The definitions, drawn and
stated, are [the knowledge base](reasoning/01-static-ir/01-knowledge-base.md), and they are decisions
[Q-M1f.379](../decisions/language/Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md) to
[Q-M1f.382](../decisions/language/Q-M1f.382-a-labelled-arrow-is-a-fact-of-three-arrows.md).

## Beside them

- [**Decisions**](../decisions/README.md): every question about Ein that has an answer, as that answer, each in a
  file under its id. A page cites a decision by its id rather than restating why. Three of them are the standard a
  claim on these pages is held to: [what refutes one](../decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md),
  [when an argument is enough](../decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md), and
  [what an exact number carries](../decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md).
- [**Overview**](overview/README.md): the lineage of Ein's language, what it takes from each tradition and where it
  differs.
- [**Demo**](../demo/README.md): the article and the talk.

## Where to start

- **To write and solve a puzzle**: [the guide](user/02-guide/README.md), then [install](user/01-install.md) and
  [the command line](user/04-reference/05-cli.md).
- **To read an answer**: [the read-out](user/04-reference/08-read-out.md), every row `ein solve` prints and what it counts.
- **To know what the answer is**: [the solution semantics](reasoning/02-kb-evolution/06-solution-semantics.md).
- **To embed the engine**: [the library API](ein.rs/04-api.md).
