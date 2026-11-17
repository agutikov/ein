# Overview

This folder will hold the comparison of Ein with other reasoning systems, written with Ein's presentation: z3,
CVC4, Lean, K, OpenCog, Prolog and Certora, one page a system, each read along the same axes. Until then it holds
the lineage of Ein's language.

| page | what it holds |
|---|---|
| [`01-inspirations.md`](01-inspirations.md) | what Ein's language takes from SMT-LIB, miniKanren, AtomSpace, Datalog and double-pushout graph rewriting, and where it differs from each |

The axes each comparison will be read along:

- **what a problem is written as**: the surface language, and whether rules, facts and queries share one syntax;
- **what the answer is**: one model, every model, a proof, a refutation, or a region of the hypothesis space;
- **how the answer is explained**: the provenance a user can read back, and whether a refutation names its cause;
- **what decides it**: the procedure, and what of it is complete or bounded;
- **what is built in**: arithmetic, equality, types, theories, against what a library states as rules.

The systems themselves are described, outside this tree, in the catalogue under [`docs/lib/`](../../lib/README.md).
