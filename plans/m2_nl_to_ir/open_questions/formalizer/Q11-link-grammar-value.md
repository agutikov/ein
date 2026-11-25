# Q11 — Link-grammar value

**Status:** open

The user's open question: *"does feeding link-grammar output to
the LLM enrich the input usefully?"*

**Working answer**: unknown. The experiment runs with a small
benchmark, with metric = correctness of generated IR on a gold set.
Default deployment is no-link-grammar unless it ships a measurable
improvement.

**Where it runs now:** [S2.6.4](../../p2.6_ablations/s2.6.4_representation_ablations.md),
as one arm of the representation ablation (the plan's G8 asks the same
shape of question about the *output* side — direct Ein vs a decomposed
intermediate — and this is the *input* side), with the pre-registered
decision rule the old S2.5.2 wrote. The metric is no longer "IR-F1 on five
puzzles": it is constraint precision / recall on the benchmark's synthetic
families, where the gold theory is exact. The submodule note does not stand: no
submodule is registered and none is needed, since [`nl2smt/`](../../../../nl2smt/README.md) drives the installed
`link-parser` by its command line, and the stage does the same.
