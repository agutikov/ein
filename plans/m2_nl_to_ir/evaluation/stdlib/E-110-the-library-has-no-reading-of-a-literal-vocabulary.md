# E-110 — The library has no reading of a literal vocabulary

**Kind:** gap
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** the user, [Q-M2.23](../../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md);
[S2.2.4](../../p2.2_formalizer/s2.2.4_passes.md)'s catalogue meanwhile
**Since P2.2's close (2026-11-27):** the user, [Q-M2.23](../../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md)

## What

**The library has no reading of a literal vocabulary.** A theory for the Zebra puzzle whose facts are the text's
verbs, `(drinks Ukrainian Tea)`, `(is-drunk-in Coffee Green)`, is the library's (`std.slots` and `std.algebra`) only
through many selecting facts, each a separate spelling: `zebra3.1.ein` writes no rule and 32 selecting facts (nine
`includes`, two `symmetric`, one `slot-partition`, two `slot-spatial`, four `converse`, six `compose`, four `top`,
four `meet`) where `zebra.ein`, whose facts are the theory's own `co-located`, writes eleven, five of them its
hypothesis rule's activators. A description written
as a fact, `(the Red House)`, or a house's adjective as a relation, `(red House)`, has no reading in the library at
all: `zebra3.2.ein` writes six rules of its own, `zebra3.3.ein` five, and `zebra3.4.ein`, the verb as an argument,
fourteen, `zebra2.ein`'s rules over the verbs.

## Evidence

`grep -c '^(rule ' examples/zebra3.*.ein`: 0, 6, 5, 14; the forms by kind, `python3 forms.py FILE` in
[`../zebra_formulations/probes/`](../../p2.2_formalizer/zebra_formulations/probes/) (`results/text.json`'s `kinds`).

## What it costs the application

Selection is spelling, and every selecting fact is one a model can misspell in silence (E-11, E-12, E-107): the
literal vocabulary with no rule of its own has three times the selecting facts. The descriptions and the
adjectives move the work from selection to synthesis, the step [F12](../../../followups/f12_rules_and_relations/ideas.md)
prices highest.
