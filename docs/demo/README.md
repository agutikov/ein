# Demo

This folder will hold the article and the talk *From Zebra puzzle to Autoformalization*, written with Ein's
presentation. Nothing of it is written yet.

The article's structure, five beats, each a step in how much of the reasoning a system is told rather than given:

1. **Enumeration with hard-coded predicates.** The Zebra puzzle solved by plain C programs that differ in one
   thing, how much the search is told about the constraints. They exist, outside the project, as the three
   baselines of [`c/`](../../c/README.md).
2. **Link grammar and SMT.** The puzzle's English sentences parsed by a link grammar and translated to an SMT
   problem a solver decides. A proof of concept exists, outside the project, in [`nl2smt/`](../../nl2smt/README.md).
3. **An intermediate representation as a reasoning substrate.** The puzzle as facts and rules that a reasoner
   derives from, and that explain each conclusion: Ein's language ([`docs/ein/`](../ein/README.md)).
4. **Ein.** The engine, its answer as a region of the hypothesis space, and the trace that reads like the human
   walkthrough.
5. **Theory selection, construction and neural-guided synthesis.** A language model that chooses or writes the
   theory a problem needs, with the reasoner checking what it writes.
