# Q-M2.24 — An answer does not depend on the order a program's forms are written in

Two programs that differ only in the order their forms are written in, in the main file or in a module either
imports, are one program and have one answer: R, its states and its two counts, root's standing and what root owes,
every question's outcome (the description, `--single`, `-n N`, each line, `--count` and `--no-search`), every
query's classes and rows over all of R, every claim's verdict, and H. A run that answers the two differently is a
wrong answer, a bug of the engine, never a meaning of the program.

What may move under another order is how a run reaches the answer and the order it says it in
([`01-determinism-and-order.md` § 1.3](../../ein/ein.rs/03-design/01-determinism-and-order.md#13-what-varies-and-why-it-may)):
the trace and the event stream; the order rules fire in and which derivation of a fact is recorded first, and so a
dead node's core ([Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md)); a listing kept in the written order,
root's owed instances among them; where a question asks for some members of R and not all (`-n N`, a line's first
state), which it meets first, since the obligation tree branches on the owed instance written first among equals,
each member it lists being a member of R in either order; the `:source` a fact stated twice keeps, the first
([Q-M1f.139](../imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)); and, where a program writes
several `(config …)` forms, the one a run takes, the last, which changes no answer
([Q-M1e2.30](../answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)).

**Where a run falls short of it today.** A saturation's boundary admits the rules waiting on an `absent` first in,
first out in their written order within each half of the agenda
([Q-M1f.313](../engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)), so where a rule's `absent` reads a
relation that another rule waiting on an `absent` concludes, the one written first is admitted first, and that can
decide the fixpoint, root's as a node's
([`03-absent-and-naf.md` § What is not provided](../../ein/reasoning/02-kb-evolution/03-absent-and-naf.md#what-is-not-provided),
[`08-monotonicity.md` § 8.3](../../ein/reasoning/02-kb-evolution/08-monotonicity.md#83-the-fifo-gap)). Of a program
the static check passes, such a run is a wrong answer by this decision, known and not fixed. Of a program it flags,
nothing is claimed of the answer, and so not this either
([Q-M1f.564](../answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).

**Why.** A form is its own object, and a name is one object wherever it is written
([Q-M1f.380](Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)): a fact is its tuple wherever it stands, a
declaration speaks of its relation wherever it stands, and no form written later redefines a rule or a macro written
earlier, two of one name with two bodies being refused. And the load, which connects the names and builds the
program's runtime representation, its compiled rules, its indexes and its registries, reads the whole program,
every import resolved and brought whole
([Q-M1f.247](../imports/Q-M1f.247-every-import-brings-its-module-whole.md)), before anything is asked of it; no
module is loaded at run time. The written order outlives the load only as an order, the registries' and the lists',
which decides what fires first and what is listed first, and no answer. So a program written in the order it is
made, its statements before the relations they use and its rules last, is the program written in any other order.

**Held by.** No test: the guarantee is stated, and a run that answers two orders of one program differently is
filed as a wrong answer. Its argument, for a program the static check passes in which no rule's `absent` reads what
another rule waiting on an `absent` concludes, is Proposition 2 of
[`03-transformations.md` § 3](../../ein/reasoning/03-rule-evolution/03-transformations.md#3-what-leaves-r-unchanged).
The page: [`01-determinism-and-order.md` § 1.2](../../ein/ein.rs/03-design/01-determinism-and-order.md#12-what-it-may-not-depend-on).
