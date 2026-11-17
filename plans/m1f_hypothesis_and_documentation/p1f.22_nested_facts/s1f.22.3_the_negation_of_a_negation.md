# S1f.22.3 — the negation of a negation is a fact

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **started 2026-11-17**, [S1f.22.2](s1f.22.2_the_explanation_written.md) being done.
**Ruling:** [Q-M1f.391](../open_questions.md#q-m1f391--the-negation-of-a-negation-that-a-rule-builds), (c):
`(not (not g))` is a fact a file may write and a rule may match; the kernel is not changed; the rule that
concludes `g` from it is a theory's, and where that rule lives is P1f.21's.

## What it builds

1. **The grammar** (`ein-ir/src/ein.y`): a negation under a negation, at any depth, as a fact, as a fact under an
   arrow, in a pattern and in a conclusion. The grammar page's two generated regions and its diagrams follow
   (`EIN_BLESS=1`), and the row of its table that refuses the shape goes.
2. **The loader and the value rule**: an object is an atom, a fact, or the negation of a fact, a negation among
   them. What stays refused: `not` of an atom, and a bare variable under a nested `not`
   ([`contexts.md` § 6](contexts.md#6-the-negation-of-a-negation-and-its-elimination-as-a-rule)).
3. **Nothing in the kernel.** A fact held with its negation is dead, for every fact; so `(not g)` beside
   `(not (not g))` is dead and `g` beside `(not (not g))` is alive, as the engine does today. Every reader of a
   rule's negative literal (the strata, the monotone check, *forbid*, the mechanisms' check, the invariant) is run
   on a premise and a conclusion that nest a negation, and what each reads is stated in the stage's record.
4. **The witnesses, as corpus programs** under `examples/beliefs/`, each with its `:expect`: the liar's rule over
   a negative statement (`e11`, `f1`, `f2` of the analysis), and one that writes the elimination itself, one rule
   for a count of arguments with the relation a parameter, to show that a theory can.
5. **The pages, in the same commits**: the knowledge-base page's shape of $\mathsf{not}$ and *"$\lnot\lnot g$ is
   no fact"*; the rewriting-rules page's $\lnot t$; the user's objects page, the syntax, the diagnostics; the
   decision Q-M1f.391, and Q-M1f.113's sentence re-worded where it says otherwise.
6. **The explanation's witnesses, as corpus programs** under `examples/beliefs/`, each with its `:expect`, for the
   claims S1f.22.2 wrote with no test of their own ([its record](s1f.22.2_the_explanation_written.md#record)): the
   three cases with their two questions; a global rule beside its wrapped twin; the context as a parameter, with
   `(false)` in a context and *everybody* by `forall`; a declaration, an activator and `(false)` under an arrow
   doing nothing. `08-reasoning-in-code/01-static-ir.md` names each beside the claim it holds. The comments of the
   folder's programs and their corpus notes take the ruled words (N-17).

## Acceptance

`./run_tests.sh` exit 0 on the stage's last commit; the parser's fuzz step of the nightly, both seeds, since the
grammar changed. No R of a corpus program changes: `ein-cli/tests/regressions.rs` and the corpus sweep hold it.
What it finds outside its scope is in [`findings.md`](findings.md).

## Record

Not started.
