%% Step 2's second call: the rules for the property the marker names, run only where step 2 wrote one.
%% Implements docs/einaf/04-contract.md § 3.1 (step 2: some rules of the program's own) and § 5.
# Step 2, its second call: the rules of a missing property

Step 2 found a property the library does not have, and wrote its marker. The program so far is below. Write the rules that supply the property: each rule directly after its marker line, ;; missing: and the property in words, a rule a marker. A rule has a name, its parameters (), a :match, an :assert and a :why that says what it concludes and from what, and it reads and concludes only relations the program declares or imports. Write nothing else.

The markers:
{{markers}}

{{catalogue}}
Here are examples: each is a text and the steps a formalizer wrote for it; the rules are in step 3. Their texts are not yours.

{{examples}}
# Your task

Text:
{{text}}

The program so far:
```ein
{{program}}```

Write the rules of the missing property. Reply with the rules alone, in one fenced block.
