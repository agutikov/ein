%% Step 2: the complete ontology and the theory selected.
%% Implements docs/einaf/04-contract.md § 3.1 (step 2), § 1.2 (the ontology complete, the theory begun), § 1.3
%% and § 5 (the excerpt, the catalogue, the examples, the text, step 1's program).
# Step 2 of 3: the complete ontology and the theory selected

A formalization is made in three steps, and this is the second. Step 1 wrote the statements, the answer query and a partial ontology, below. Step 3 will write the rules the library does not have and the hypothesis mechanism. Write step 2 only.

Step 2 writes, in this order:

1. **The ontology, whole**: every declaration of step 1, kept or completed, and every relation the theory will read that step 1 did not declare: the relation the theory reads the verbs into, a verb's passive, the pairs a read-back needs, the kinds. Then every membership: step 1's, the kinds and their place, (type Color Attribute), and the facts the text speaks of and never states. It replaces step 1's ontology in the program, so leave none of step 1's declarations or memberships out. Do not write the statements or the answer query again.
2. **The imports**, from the catalogue below, each listing after :symbols every short name of the module the program will write, the ones step 3 will write among them.
3. **The selection**: the facts that select what was imported, exactly as the catalogue's entries write them, over the program's own relations and names.

If the text needs a property no entry of the catalogue gives, write after the selection the marker line ;; missing: and the property in words; its rules are written in another call. Write no rule.

{{catalogue}}
Here are examples: each is a text, the step 1 a formalizer wrote for it, and its step 2. Their texts are not yours.

{{examples}}
# Your task

Text:
{{text}}

Step 1:
```ein
{{program}}```

Write step 2 of the formalization of this text. Reply with step 2 alone, in one fenced block.
