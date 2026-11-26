# Q-M2.33 — Is one class with several rows an answer of several values, or the text that the statements do not determine the answer?

**Status:** open

**Raised 2026-11-26 by [S2.2.5](../../s2.2.5_from_text.md)**, whose answer rule ([`einaf::answer`](../../../../../einaf/src/answer.rs),
[from a text to a short answer § 2](../../../../../docs/einaf/08-from-text.md#2-the-answer)) reads the answer query's
classes, as S2.2.4 left it to (E-124).

**The question.** Where R has a state and the answer query one class over a complete set, the program's sentence is
printed, a line a row. A class can hold several rows: a question of several values (*who is older than Bob?*), or a
question of one value that the program answers with a row a pass of a rule that reads what it concludes
([E-124](../../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md): the model's
`bob1` prints *Bob has 1 apples. Bob has 3 apples. Bob has 5 apples.*). The kernel cannot tell the two apart: a query
lists every binding a state holds. Is such a class printed as its lines, an answer of several values the reader then
scores, or as *The statements do not determine the answer.*?

**The default taken: its lines.** The program's own sentence, each row, as the kernel filled it and formatted; the
stand-in reader scores it, and a line stating a number the gold does not makes it wrong. The fixed text is kept for
what the classes say (two classes, or one with no row), not for what a reader of the question knows of its values.

**The alternatives.** The fixed text where a class has two rows or more and the question asks for one value, which
needs the question's kind (*how many*, *who*) as a field of the instance, the benchmark's
([P2.3](../../../p2.3_benchmark/README.md)); or a line of EinAf's beneath the rows, *several values for one*, scored as
the rows are.

**What bears on it.** S2.5.3's answer rule, which this stage's stands in for; the loop's repair of E-124, the facts'
moments, which no diagnostic points at either way.
