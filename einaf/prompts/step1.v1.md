# Step 1: the facts and a partial ontology

A formalization is made in three steps, and this is the first: **the facts and a partial ontology**. Step 2 will
complete the ontology and select the theory; step 3 will write the rules still missing and the hypothesis
mechanism. Write step 1 only.

**What step 1 writes:**

- **each statement of the text as a fact, or as several, in the text's own words**: a relation named by the
  sentence's verb in its voice, its arguments the words the sentence names, and the sentence itself, word for word,
  as the fact's `:source`; a sentence that says something does not hold is a stored negation, `(not …)`;
- **the question as the answer query**: one `query` whose `:goal` asks what the question asks, over the facts' own
  relations, as literally as a statement, and whose `:goal-text` is a sentence that says the answer;
- **the declarations** of the relations the facts and the query use, each with a word for each place and a `:why`;
- **the memberships the facts need, as far as they are seen**: what kind of thing each name is, a fact such as
  `(person Ann)` or `(instance Red Color)`, quoting the sentence that names the thing.

**What step 1 decides:** which words name the same object (*he* and *Bob*, *the green house* and *the house where
coffee is drunk*); what is an object and what is a relation; one fact a sentence or several; whether the objects
are of one kind or of several.

**What step 1 does not write:** no `import`, no `rule`, no `solve`. The ontology it begins is partial on purpose:
its labels and memberships may be completed or renamed in step 2. The statements and the answer query are never
rewritten by a later step.

Step 1's program is loaded and judged at root: it must load, and its root must be alive.
