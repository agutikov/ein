# Q-M1f.58 — A fact reads in English in the trace, from its first recorded justification; a `:why` reads the form it is written on

**A `:why` reads the form it is written on.** A declaration is the fact it states: `(relation owns Person Pet :why
"a {?3} {?2} a {?4}")` is the fact `(relation owns Person Pet)`, and `{?n}` is that fact's n-th arrow, `{?1}` the
atom `relation`, `{?2}` the relation's name, `{?k+2}` its k-th label, so it reads *a Person owns a Pet*: the
declaration's **statement**. That fact reads by the text however a run reaches it; the companion `(relation owns)`
the loader stores beside a typed declaration is no form the program wrote, and an untyped declaration, `(relation z
:why "…")`, is its own form. A rule's and a macro's `:why` are filled by their variables.

**A fact of a relation takes no text from the declaration, and carries none of its own.** `(owns Ann Dog)` reads by
its **reason**, read off its first recorded justification: a given's `:source`, or the deriving rule's `:why`
filled with that firing's bindings; and where it has none, as written, never as invented prose.

`--trace` writes a firing's reason in its step's quote, a given premise with its `:source`, a declaration a rule
reads with its statement after it on the premises line, and each fact the step derives as written, followed by a
statement only where it is a declaration; `ein render kb` labels a fact's arrow with its source or its rule. The
read-out's `(model …)` blocks, `ein solve`'s stdout and `answer.txt` stay s-expressions, since a claim is pasted from
them, and a saved state holds no fact of the state to render
([Q-M1f.392](../answer/Q-M1f.392-a-saved-state-is-saved-and-loaded-as-itself.md)). A stated fact takes `:source`
alone, and it is optional.

A `:why` is required on a relation, as on a rule and a macro, and `--no-require-why` loads a program
without ([Q-M1f.62](../cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)). The relations the project
ships keep words written to read one fact of them, so many read stiffly as their declaration: `(relation valid
Number :why "{?3} is a decimal number")` reads *Number is a decimal number*.

**Why.** A text is about the form that carries it, so one rule reads every `:why`: the arrows of a declaration, the
variables of a rule. A declaration is one fact, and a template for every fact of its relation would be a second
meaning of the same text. A line of the trace is where a reader wants the program's own words, and it stops no
claim form being Ein. A fact's justifications arrive in the agenda's order, so its reason is an explanation, as a core is
([Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md)); within a run it is deterministic.

**Held by.** `ein_render::sentence`'s `statement` and `rule_sentence`; `ein-cli/tests/natural_language_text.rs`:
`a_declaration_reads_its_own_arrows_and_a_fact_none`, `the_trace_prints_the_rules_why_and_the_premises_source`;
`ein-ir/tests/template_wellformedness.rs`, which holds every declaration's references to its arrows, `{?2}` to
`{?n+2}`; `ein-cli/tests/natural_language_page.rs`: `the_pages_trace_lines_are_a_runs`. The page:
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md#3-a-fact-in-english-the-fallback-ladder).
