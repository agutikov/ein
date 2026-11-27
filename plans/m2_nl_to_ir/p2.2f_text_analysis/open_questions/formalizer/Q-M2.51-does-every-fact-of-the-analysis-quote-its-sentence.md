# Q-M2.51 — Does every fact of the analysis quote its sentence, or a sentence's facts once?

**Status:** ruled by the user, 2026-11-27

**Ruled by the user, 2026-11-27:** *"Q-M2.51 - yes"*: every fact of the analysis quotes its sentence, as the plan
asks and the facts form writes it (the default, the first alternative). Nothing changes.

**Raised 2026-11-27 by [S2.2f.2](../../s2.2f.2_the_analysis_written.md)**, writing the facts form.

**The question.** The plan asks for the analysis as Ein facts *each with the sentence it came from as its
`:source`*, and the facts form does that: `(amr-role s7-z3 ARG0 s7-z2 :source "The Old Gold smoker owns snails.")`.
On the Zebra puzzle the quotes are 2 354 of the form's 5 689 tokens ([L-M2.45](../../findings/text-analysis/L-M2.45-the-facts-form-is-four-times-the-text-forms-in-the-models-tokens.md)),
and the facts form is four times either text form. A node's sentence is also a fact of its own, `(amr-node S7 s7-z3
smoke-02)` naming the sentence `S7`, and a node's name carries its sentence's number. Should every fact quote its
sentence, as the plan says, or only the facts that name a sentence (`amr-top`, `amr-node`, `named-entity`), the
rest reached through their nodes?

**The default that stands**: every fact quotes its sentence, as the plan asks: it is what the kernel shows of a fact
(`:source` is what a fact reads by), what a rule's conclusion can carry on, and what a model copying a fact would
copy with it.

**The alternatives.**

1. **The default.**
2. **The facts that name a sentence quote it**; a role, a constant, a name, a negation and a word do not, and are
   reached through their nodes: 109 of the Zebra puzzle's 194 facts quote, 4 632 tokens.
3. **No fact quotes**: the sentences in comments, `;; 7. The Old Gold smoker …`, as the form writes them already; the
   kernel then shows no sentence of any fact: 3 335 tokens.

Both measured by hand on the stage's branch, `einaf analysis show einaf/tests/fixtures/analysis/zebra.json --form
facts` with the quotes taken out by `sed`, counted by `llama-tokenize` with the model's vocabulary.

**What bears on it.** [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md) measures the facts arm as the plan wrote it;
an alternative would be a second facts form, a version of its template and a flag, and an arm of its own.
