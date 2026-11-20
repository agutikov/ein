# Q-M1f.11 — A template may not refer to a variable whose name holds `}`

A template reference, `{?name}`, reads a variable's name up to the first `}`, and `}` is a name character, so a
variable such as `?a}b` is one no reference can name: `{?a}b}` would read as the reference `{?a}` and the text
`b}`. The loader refuses a template that writes such a reference to a variable of its own form: a rule's or a
macro's `:why`, and a query's `:goal-text`, one line a reference, *… its :why writes {?a}b}, a
reference to ?a}b, which no reference can name — a reference reads a variable's name up to its first }*. A
reference to no variable of the form stays text.

**Why.** Left alone, the sentence would print with text its author did not write, in silence; a refusal at load
says so where the template is written.

**Held by.** `ein-ir/tests/load_semantics.rs`: `a_template_naming_a_variable_no_reference_reaches_is_refused`; the
reference's reader, `ein_core::why::render_why`.
