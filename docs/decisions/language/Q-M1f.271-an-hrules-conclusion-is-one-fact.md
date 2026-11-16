# Q-M1f.271 — An hrule's conclusion is one fact

An hrule whose `:assert`, read flat (`ein_ir::Ast::conjuncts`) after its macros expand, is more than one conjunct
is refused at load, on every command, one line placed at the conclusion: *hrule 'h': its :assert writes (and (p a)
(q a)), 2 conclusions — an hrule proposes one fact per match, each a commitment of its own: write one hrule per
conclusion*. An `(and …)` of one conjunct is one conclusion and loads.

**Why.** An hrule proposes facts, each a candidate and a commitment of its own. A conjunction could be read as
*each of these is a guess* or as *these together*, and no commitment states the second; two hrules say the first
with no reading to choose.

**Held by.** `ein-cli/tests/conclusion_shapes.rs`: `an_hrule_concludes_one_fact`, the conjunction written and
through a macro's call.
