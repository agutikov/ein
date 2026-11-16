# Q31 — A `:why` template refers to a variable as `{?var}`

A reference in a `:why` or a `:goal-text` is `{?` then a variable's name as the program writes it after its `?`, up
to the first `}`: in `"{?rel} is transitive: {?a} →{?rel}→ {?b}"`, `{?rel}` prints the name `?rel` is bound to. The
`?` is part of the reference, so a reference reads as the variable it names in `:match` and `:assert`. Any `{x}`
without the `?` is literal text, and a reference to no variable bound stays as written. A relation's `:why` names
its arguments by position, `{?1}`, `{?2}`.

**Why.** One spelling for a variable, in a pattern and in its sentence, leaves nothing to translate; braces with no
`?` stay free for the author's text.

**Held by.** `ein_core::why::render_why` and its unit tests in `ein-core/src/why.rs`:
`a_bound_reference_is_substituted_and_an_unbound_one_is_kept`,
`a_positional_slot_is_a_reference_and_a_bare_brace_is_not`. A reference that could name no variable is refused at
load: [Q-M1f.11](Q-M1f.11-a-template-may-not-refer-to-a-variable-whose.md).
