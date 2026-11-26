%% Step 3: the complete theory and the hypothesis mechanism.
%% Implements docs/einaf/04-contract.md § 3.1 (step 3), § 1.2 (the theory complete, the hypothesis mechanism),
%% § 1.3 and § 5 (the excerpt, the catalogue, the examples, the text, the program of steps 1 and 2).
# Step 3 of 3: the complete theory and the hypothesis mechanism

A formalization is made in three steps, and this is the last. Steps 1 and 2 wrote the program below: the statements, the answer query, the ontology, the imports and the selection. Write step 3 only.

Step 3 writes, in this order:

1. **The rest of the selection**: the facts that select what the imported modules can still give, over the program's own relations and names, such as the read-back that makes the answer query answerable.
2. **The rules the library does not have**, each directly after its marker line: ;; missing: and the property it supplies, in words. A rule has a name, its parameters (), a :match, an :assert and a :why that says what it concludes and from what, and it reads and concludes only relations the program declares or imports.
3. **The hypothesis mechanism**: nothing, where a theory step 2 selected proposes the hypotheses (the catalogue says which do); (solve :no-search) where the text states every fact there is and the answer follows from the rules; a rule that concludes (open F), or a written (open F), otherwise.

Do not write again what the program already holds, and write no import and no declaration: the program is complete after this step, and the kernel runs it.

{{catalogue}}
Here are examples: each is a text and the three steps a formalizer wrote for it. Their texts are not yours.

{{examples}}
# Your task

Text:
{{text}}

The program so far:
```ein
{{program}}```

Write step 3 of the formalization of this text. Reply with step 3 alone, in one fenced block.
