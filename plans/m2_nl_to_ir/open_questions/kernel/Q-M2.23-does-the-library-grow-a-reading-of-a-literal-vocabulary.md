# Q-M2.23 — Does the library grow a reading of a literal vocabulary?

**Status:** open

**Raised 2026-11-26 by [S2.2.0a](../../p2.2_formalizer/s2.2.0a_zebra_formulations.md)**, whose four literal encodings of the Zebra
puzzle ([`zebra_formulations/`](../../p2.2_formalizer/zebra_formulations/README.md)) found what the library does not have
([E-110](../../evaluation/stdlib/E-110-the-library-has-no-reading-of-a-literal-vocabulary.md)).

**The question.** A formalization whose facts are the text's own words needs its theory to read those words: a verb
as a part of the theory's link (`(includes drinks same-house)`), an active and its passive as converses, a passive
of place through the man who lives there, a description `(the Red House)` as the name it is built on, an adjective
`(red House)` as a value. The library has the first three as relation algebra (`includes`, `converse`, `compose`,
`top`, `meet`, which `zebra3.1.ein` selects with 32 facts and no rule) and none of the last two (`zebra3.2.ein` and
`zebra3.3.ein` write six and five rules of their own). Should `stdlib/` grow a module for them, and which?

**The default taken**: no module now. [S2.2.4](../../p2.2_formalizer/s2.2.4_passes.md)'s catalogue writes the readings the library
has as recipes, each a program the kernel runs (a verb into the link; a pair of voices; a passive of place), and a
reading the library lacks stays a program's own rule, which the formalizer synthesises and the record counts.
`zebra3.1.ein` is the target shape ([the phase's default 1](../../p2.2_formalizer/README.md#defaults-taken-at-the-re-plan)), so the
first need is the recipes; whether a description or an adjective reading earns a module is weighed with what the
probe and the steps by hand find, at the iteration of Ein.

**What the default costs.** A literal vocabulary's theory is many small selections, each a spelling that can fail
in silence ([E-107](../../evaluation/diagnostics/E-107-a-theory-s-missing-activating-fact-is-reported-as-the.md));
a module could select the whole reading with one fact. A description or an adjective is synthesis, which F12 prices
high.

**The alternatives.**

1. **No module** (the default).
2. **A module `std.words`**: one fact `(verb-of-house drinks Nationality Drink)` that brings the verb into the link,
   its passive, its passive of place and its read-back; one per description shape.
3. **A module for descriptions alone**, `the` read as its first argument in any relation it stands in, and the
   verbs left to the relation algebra.

**What bears on it.** Q-M2.16 (what of the kernel may change inside M2: a library module leaves the syntax and the
semantics as they are); [E-105](../../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md),
a rule of `std.slots` a literal encoding relies on. **The user, 2026-11-27, on a name made of a phrase's words**
(*"from fact (is drunk by) make is-drunk-by atom"*): not possible in Ein, possible outside it, *"if Ein rules will
produce some special forms like e.g. (defrel (is drunk by)) and EinAF post process the output"*; written down in
[S2.2f.3](../../p2.2f_text_analysis/s2.2f.3_step1_with_the_analysis.md)'s arm 5, where an analysis' phrase is made a
relation, and a way a literal vocabulary's verb could become the relation the theory reads.
