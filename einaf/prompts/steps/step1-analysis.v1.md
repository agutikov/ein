%% Step 1 given an analysis of its text: steps/step1.v1.md word for word, and the analysis after the text, in
%% the slot analysis, one form of it under analysis/ rendered (docs/einaf/09-analysis.md § 4).
%% Implements docs/einaf/04-contract.md § 3.1 (step 1), § 1.2 (the instance, the query, the ontology),
%% § 1.3 (the order), § 1.5 (the answer) and § 5 (what the model receives: the excerpt, the examples,
%% the text with its answer line; no catalogue).
# Step 1 of 3: the facts and a partial ontology

A formalization is made in three steps, and this is the first. Step 2 will complete the ontology and select the theory from the standard library; step 3 will write the rules the library does not have and the hypothesis mechanism. Write step 1 only.

Step 1 writes, in this order:

1. **The statements**: each sentence of the text that states something, as a fact or as several, in the text's own words. The relation is the sentence's verb in its own voice (lives-in, is-sold-at, has), its arguments the things the sentence names, each a capitalised name (Ann, Green, Old_Gold), and the sentence itself, word for word, is the fact's :source. A sentence that says something does not hold is a stored negation, (not (R a b) :source "..."). A number a rule will compute with is a numeral: 5 is (n z 5), 12 is (n (n z 1) 2).
2. **The answer query**: one query whose :goal asks what the text's question asks, over the statements' own relations, as literally as a statement, with a variable where each answer stands, and whose :goal-text is the sentence the answer line asks for, with {?x} where each value of the answer stands.
3. **The ontology, partial**: the declaration of every relation the statements and the query use, at the count they use it, with a label for each place and a :why that reads the declaration ({?3} its first label, {?4} its second); then the memberships the statements need as far as they are seen: what kind of thing each name is, (instance Green Color), quoting the first sentence that names the thing.

Step 1 decides which words name the same thing (he and Bob; the green stall and the stall where apples are sold), what is a thing and what is a relation, one fact a sentence or several, and whether the things are of one kind or of several. It writes no import, no rule and no solve. The ontology it begins may be completed in step 2; the statements and the answer query are never rewritten.

Here are examples: each is a text, and the step 1 a formalizer wrote for it. Their texts are not yours.

{{examples}}
# Your task

Text:
{{text}}

{{analysis}}

The answer line: {{answer_line}}

Write step 1 of the formalization of this text. Reply with step 1 alone, in one fenced block.
