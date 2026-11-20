# Q-M1f.271 — A hypothesis is one fact

`(open F)` takes one fact: a hypothesis of a conjunction, `(open (and (p a) (q a)))`, is refused on every command, at
the parse where it is written (*unexpected `and`*) and at load where a macro's expansion makes it, one line placed at
the conclusion (*rule 'h': its :assert writes (open (and (p a) (q a))), and (and (p a) (q a)) is a connective where a
value goes*). A rule supposes several facts as several hypotheses, a conjunction of `(open F)`, `(and (open (p a))
(open (q a)))`, each a candidate of its own
([Q-M1f.575](Q-M1f.575-open-r-is-a-rules-whole-assert-and-open-f-stands.md)).

**Why.** A hypothesis is a candidate and a commitment of its own. A conjunction under `open` could be read as *each of
these is a guess* or as *these together*, and no commitment states the second; several `(open F)` say the first with
no reading to choose.

**Held by.** `ein-ir/src/ein.y`, where `open`'s argument is a fact and `and` heads none; `ein_ir::from_ir`'s
`refuse_malformed_primitives`, a macro's expansion; `ein-cli/tests/conclusion_shapes.rs`: `a_hypothesis_is_one_fact`,
the conjunction written and through a macro's call refused on every command, and the conjunctions of hypotheses
loading; `examples/ein-bugs/wrong-answers/hrule-and.ein` and `hrule-and-in-and.ein`.
