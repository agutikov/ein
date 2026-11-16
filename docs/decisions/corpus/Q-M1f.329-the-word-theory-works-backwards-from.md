# Q-M1f.329 — The word theory works backwards from the question

The word theory of `examples/word_problems/` plans backwards from the question and computes forwards from the stated
numbers. `(asked Q)` makes Q `wanted`; a wanted quantity makes the quantities it is computed from wanted, each step a
`(needs Q Q')`; and where the quantities a step needs have a value, the rule for the word asks the calculator and
reads its answer back as `(value Q N)`. Nothing is computed that no question needs: a sentence no question reaches
is silent, and a contradiction among the sentences is found only where a question reaches it.

**Why.** The plan is what makes a two-step problem readable, the order of its operations the derivation's own, and
it keeps an unused sentence from costing anything. Which parts of a problem an answer used is then visible in what
was derived.

**Held by.** `examples/word_problems/unused.ein` and `bystander.ein`, whose claims state the answer and whose unused
sentences derive nothing wanted; `contra.ein`, whose contradiction the question reaches; `words.ein`'s rules.
