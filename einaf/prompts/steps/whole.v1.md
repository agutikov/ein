%% The whole program in one call: the probe's prompt kept as a mode of the formalizer.
%% Implements docs/einaf/04-contract.md § 1 (the task: five parts, in their order) and § 5 (the excerpt, the
%% catalogue, the examples, the text with its answer line).
# The whole program in one call

Write the whole Ein program for the text, in the order a formalization makes it:

1. **The statements**: each sentence that states something as a fact or several, in the text's own words, the sentence word for word as its :source; a numeral where a rule computes with a number.
2. **The answer query**: one query over the statements' own relations, with the :goal-text the answer line asks for.
3. **The ontology**: the declaration of every relation the program uses, each with a :why, and the memberships, the kinds and the facts the text speaks of and never states.
4. **The theory**: the imports, from the catalogue below, each listing the short names the program writes, and the facts that select what was imported; then the rules the library does not have, each directly after its marker line, ;; missing: and the property in words.
5. **The hypothesis mechanism**: nothing where a selected theory proposes the hypotheses; (solve :no-search) where the text states every fact there is; a rule that concludes (open F), or a written (open F), otherwise.

{{catalogue}}
Here are examples: each is a text and the program a formalizer wrote for it. Their texts are not yours.

{{examples}}
# Your task

Text:
{{text}}

The answer line: {{answer_line}}

Write the whole program for this text. Reply with the program alone, in one fenced block.
