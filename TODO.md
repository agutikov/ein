







Update S1.2.1 data model
Data model is not SForm and Graph+Node+Edge
Data model is: Rule, Relation, Type, instance and cross-references between and interaction
for example: how to get all relations for the rule, all rules for relation, all rules for type, etc...






---

cardinality

ordinality



---


Variable should have types, if not deducable from context

Type system - relation type, rule type, ...


---

type-exclusivity is a property of `instance` or of `co-located`?



---

ideas:

- induction: facts -> rules on relations



- how LLM can produce facts, relations, types and rules?
    - we provide set of questions to answer about every word in every sentence
    - questions are specific for different word types (noun, verb, ...) and role (subj, predicate, ...)

