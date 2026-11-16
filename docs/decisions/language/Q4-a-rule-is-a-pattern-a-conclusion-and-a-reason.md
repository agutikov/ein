# Q4 — A rule is a pattern, a conclusion and a reason, written in Ein

A rule is a form of the language, `(rule NAME (PARAMS) :match PATTERN :assert CONCLUSION :why "TEMPLATE")`: a
pattern over the knowledge base, a conclusion to state where it matches, and the reason a firing gives, in English.
An hrule has the same form and proposes its conclusion as a hypothesis. Every rule the engine fires is written so,
by a program or by a stdlib module; a firing's premises are the facts its pattern matched, recorded with what it
derives.

**Why.** A rule written as a pattern and a conclusion fires with a structured list of premises, which a trace, a
core and a rendered explanation read; a rule written as host code would fire with none. Patterns are expressive
enough for the puzzles Ein reasons about: spatial structure among them
([Q17](Q17-spatial-structure-is-facts-and-rules-with-no.md)).

**Held by.** The grammar's `Rule` production, `ein-ir/src/ein.y`, the one way a rule is written; the stdlib, whose
every rule is Ein text under `stdlib/`, each activated by a program under `tests/stdlib/`
(`ein-infer/tests/stdlib_coverage.rs`: `every_stdlib_rule_is_activated_by_a_program`).
