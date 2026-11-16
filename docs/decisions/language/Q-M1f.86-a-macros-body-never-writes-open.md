# Q-M1f.86 — A macro's body never writes `open`

`open` is no name, so no template writes it: a macro's body that writes `(open)` or `(open ?R)` is a parse error at
the `open`, whether a rule calls the macro or not, as `_` and a string are in a body. A rule that states the
verdict writes it as its own `:assert` ([Q-M1f.66](Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)).

**Why.** The verdict is written only as a rule's whole conclusion, and a macro's body is none. A word the grammar
refuses wherever it stands is refused where it is written, which a call does not change; what a call does decide, a
primitive's shape, is checked at the call ([Q-M1f.67](Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)).

**Held by.** `ein-ir/src/ein.y`'s `Template`, which takes no `open`; `ein-cli/tests/open_reserved.rs`:
`open_is_refused_wherever_it_is_not_the_verdict`, whose places include a macro's body called in an `:assert`,
called in a `:match`, and called by no rule.
