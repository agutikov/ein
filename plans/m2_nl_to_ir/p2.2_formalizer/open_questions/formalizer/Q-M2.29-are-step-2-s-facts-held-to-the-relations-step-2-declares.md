# Q-M2.29 — Are step 2's facts held to the relations step 2 declares?

**Status:** open

**Raised 2026-11-26 by [S2.2.3](../../s2.2.3_gbnf.md)**, writing a grammar a step.

**The question.** A grammar closes a program's vocabulary only over what is known when it is rendered. Step 1 writes
its facts before their declarations, so no grammar of step 1 holds a fact to a declaration, as the stage foresaw.
**Step 2 has the same limit within one call**: it writes the complete ontology, declarations the program did not have
among it (`same-house`, `type`, the kinds), and in the same output facts of them (`(type Color Attribute)`) and
activating facts over them (`(includes drinks same-house)`). A context-free grammar cannot hold a fact to a
declaration written earlier in the same text, so step 2's grammar leaves its ontology's facts open (any relation, any
count) and holds its imports and the heads and counts of its activating facts to the catalogue, their arguments open.
Steps 2b and 3 are rendered on the program of steps 1 and 2 and close everything they write. Does the formalizer hold
step 2's facts to step 2's declarations, and how?

**The default taken**: no grammar holds them; the contract's check does
([S2.2.4](../../s2.2.4_passes.md), T2.2.4.6: a fact of a relation the program does not declare, a form at another
count), on step 2's program as on every step's.

**The alternatives.**

1. **The default**: step 2's ontology open in its grammar, the check after it.
2. **Step 2 in two calls**: the ontology first, then the selection under a grammar rendered on it, as step 2's second
   call is rendered for the rules: a call more, a closed selection.
3. **A grammar swapped inside the call**: EinAf owns the loop over tokens, and could render the selection's grammar
   at the first `(import` from the declarations written so far; a runtime feature the binding allows and nothing
   builds yet.

**What bears on it.** How often a model's step 2 writes a fact its own declarations do not match, which S2.2.4's runs
of step 2 measure; the cost of a call, which the speeds put at the prompt's evaluation and the generation alone.
