# Q9 — Ontology provenance

**Status:** open

Per [idea 04 §Open questions point 2](../../../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Options:**

- **A**: Per-puzzle declared ontology — user supplies/curates a
  small `ontology.ein` alongside the text.
- **B**: Inferred from text — LLM extracts type declarations.
- **C**: Library of standard ontologies (zebra, sudoku, einstein)
  + override.

**Working answer**: C for the three demo puzzles in M2's
acceptance; B for everything else, with the LLM's inferred
ontology subject to user review before solving. A always available
as escape hatch.

**Reframed by the reshape.** The question was asked about *ontologies* —
types and instances — and the answer turned out to be about *theories*. What
a Zebra text leaves implicit is not that there are houses and colours (the
LLM reads that off the text) but that the houses are linearly ordered and
every attribute is a bijection onto them; that is a theory, the stdlib
already has it (`std.bijection`, `std.slots`, `std.algebra`), and the
formalizer's job is to **select** it by asserting `(bijective pet-loc)`
rather than to write it ([F12](../../../followups/f12_rules_and_relations/ideas.md),
[F16](../../../followups/f16_autoformalization/ideas.md)). Option C is therefore
the stdlib catalogue in the prompt (the plan's B3), option B is the ontology
pass, and option A is `--ontology PATH` on `einaf from-text`, kept as the
escape hatch. "User review before solving" does not survive: the loop's
reviewer is the kernel, and a human in the loop is the one thing Stage D's
matched-budget comparison cannot price. [S2.2.4](../../p2.2_formalizer/s2.2.4_passes.md).

**Since (the re-plan of P2.2 to the user's steps, 2026-11-26).** The passes are three steps, the facts first
([the user's notes on P2.2](../../p2.2_formalizer/README.md#formalization-in-steps-the-users-notes-on-the-whole-phase)):
option B is the vocabulary step 1 begins from the text and step 2 completes, option C the catalogue step 2 selects
from, and option A is `--module-dir DIR` on `einaf from-text`, a module the program imports, whose declarations
step 1 is shown. [S2.2.4](../../p2.2_formalizer/s2.2.4_passes.md) decides.

**S2.2.4's default (2026-11-26), for the user: B with C in the main arms, A the escape hatch.** Status stays open.
The vocabulary is the text's own, begun by step 1 and completed by step 2 (**B**); the theory is selected from the
catalogue of the library, nineteen entries each a program the kernel runs, which steps 2 and 3 are shown and their
grammars held to (**C**); a module given with the problem, `--module-dir DIR` on `einaf from-text`, which the program
imports (**A**), is no part of the main arms and is the widest condition of the library ablation
([S2.6.4](../../p2.6_ablations/s2.6.4_representation_ablations.md)). The runs it rests on
([S2.2.4's report § 10](../../p2.2_formalizer/steps/README.md#10-q9-the-ontologys-provenance)):

- **B gets the vocabulary of a word problem from its text**: under the steps' grammars every one of the thirteen word
  texts' step 1 names its things and verbs from the text (`has`, `gave`, `holds`, `costs`), quotes every sentence, and
  declares what it uses; what it misses is no word of the text, the moment a change implies, which the hand's step 1
  makes up (`Start`, `After`) and the model leaves out, and the answer then has a row a pass of a rule that reads what
  it concludes (E-124).
- **C is taken up where it is the library's**: every word text's step 2 imports the calculator in its three lines,
  spelled as the entries spell them; the Zebra puzzle's selects `std.slots` and `std.algebra`, spelled right; no import
  and no selecting fact outside the catalogue in any run of the steps. Where the selection fails it is a name of the
  calculator's the import does not list (`sum-of` beside `:symbols (add product-of)`, four texts), which the check finds.
- **A did not do better where it was tried**: S2.2.2's probe gave thirteen word texts the corpus's word theory, the
  family's own module (*a theory given*, thinking off): 13 parsed, 11 loaded, 10 were *one*, and none printed the
  answer the corpus's README states ([the probe's record](../../p2.2_formalizer/model_runtime/README.md)).

- **A, given beside the library, was not taken up** (S2.2.5, 2026-11-26, `einaf from-text --module-dir
  examples/word_problems`, [its report](../../p2.2_formalizer/from_text/README.md)): steps 2 and 3 were shown the four
  modules of the word theory by their forms, beside the catalogue, and the steps' grammars offered their imports; no
  program of the thirteen imported one, every one importing the calculator as under B with C. One text printed the
  right answer under it (`bystander`, *Carol has 9 apples.*) and none under B with C alone; what made the difference
  was the longer prompt, under which step 2 ran to its budget and was read as the forms written before the cut, and
  step 3 wrote another theory over them, not an import of a module given.

**What would change it**: a family whose canonical vocabulary is a theory's own where B with C cannot reach the
answer and A can, on the benchmark (S2.6.4's library ablation); and the user's word on whether the word problems'
moments are the text's to state or the theory's to supply ([Q-M2.23](../kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md)).
