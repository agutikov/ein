# Q-M2.31 — Does a step's grammar bound the comment lines the model writes?

**Status:** open

**Raised 2026-11-26 by [S2.2.4](../../s2.2.4_passes.md)**, from its runs of the steps with the model
([L-M2.16](../../findings/model/L-M2.16-with-thinking-off-the-model-thinks-in-comment-lines.md)).

**The question.** Every step's grammar lets the model write a comment line between two forms, as many as it likes
(S2.2.3's default 15). With thinking off the model thinks there, and where greedy decoding loops it loops there, to
the budget: in S2.2.4's runs, `youngest`'s step 2 wrote 1 236 comment lines and `unused`'s steps 2 and 3 656 and 793.
A grammar could bound them: so many comment lines in a row, so many in all, or a comment of so many characters.

**The default taken: no bound.** The steps' grammars are S2.2.3's as amended; a loop costs a call its budget and is
recorded.

**The alternatives.**

1. **The default**: comments as the model writes them; the record keeps the reasoning; P2.2b gives the thinking a
   place of its own and measures what it does.
2. **A bound on comment lines in a row** (three, say): a loop in comments is cut at once, and the model is held to
   writing forms. The model's section headers and short notes stay; its prose reasoning does not.
3. **No comment at all but the marker**: the smallest grammar; the model's headers, which its examples write, are
   refused, which moved it off its own program before ([L-M2.14](../../findings/model/L-M2.14-a-grammar-that-leaves-out-the-fence-a-prompt-asks-for-derails-the-model.md)).

**What bears on it.** How often a step loops in comments (S2.2.4's tables); whether a bound turns the loop into a
loop of forms; and P2.2b, which asks for the reasoning in the thinking and can measure whether the comments then stop.
