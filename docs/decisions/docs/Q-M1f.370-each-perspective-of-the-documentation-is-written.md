# Q-M1f.370 — Each perspective of the documentation is written in its own languages

The three perspectives of `docs/ein/` differ in what they are written in, as they differ in the question they
answer.

| perspective | its main language | beside it | what it does not hold |
|---|---|---|---|
| `reasoning/` | **mathematics**: definitions, statements and arguments as formulas | graphs, of facts, of rules, of the lattice of commitments; and words, which say what a formula, an arrow or a node means | Ein source, a command line, Rust, a file's path, a test's name, a table of tests, an algorithm |
| `user/` | **Ein**: programs a reader can run, and what `ein` prints for them | graphs, which visualise what the Ein says; mathematics, where it explains; and words, which say what the Ein says | a crate, a module, an algorithm |
| `ein.rs/` | whatever the software needs: Rust, pseudocode, shell, JSON | diagrams, of an algorithm, a state machine, the structure of the crates, a process | nothing: it is where crates, tests and algorithms are named |

A page of `reasoning/` states what is true of any implementation, so what one implementation does to compute a
thing, and the tests that hold it there, are `ein.rs/`'s to state. A page of `reasoning/` links pages of
`docs/ein/` and decisions, and nothing else. An example program appears in `reasoning/` as its graph and its
formulas; where a reader needs its Ein spelling, the page links the page of `user/` that has it.

How a diagram and a formula are written is the same in all three
([Q-M1f.368](Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md),
[Q-M1f.369](Q-M1f.369-a-formula-is-written-in-tex-displayed-or-inline.md)).

**Why.** Each perspective has a reader. The one who writes programs reads Ein, and a formula helps only where it
explains what the Ein says. The one who asks what is computed and why it is right asks a mathematical question,
and an answer that cites a file or a test of `ein.rs` is true of that implementation alone, which is not what was
asked. The one who works on the engine reads its code, and needs the algorithms and the tests named.

**Held by.** [`docs/ein/README.md`](../../ein/README.md), which states the three perspectives and the question
asked of every paragraph, and the three trees under it, each written in its languages.
