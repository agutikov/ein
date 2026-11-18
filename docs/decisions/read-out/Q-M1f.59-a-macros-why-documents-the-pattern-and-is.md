# Q-M1f.59 — A macro's `:why` documents the pattern and is rendered nowhere

A macro's `:why` is required, as every relation's, rule's and hrule's is, and no surface renders it into a fact's
text. A macro is expanded at load, so a fact's justification names the rule whose `:match` used it, and that rule's
`:why` says what the pattern meant there. A state file carries the macro as the program writes it, its `:why` with
it.

**Why.** The rule that used the pattern already gives a fact its reason; the macro's text filled in beside it would
be a second reason for one fact.

**Held by.** `ein_render::sentence`, whose reason reads a rule's or hrule's `:why` and no macro's;
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md#2-every-declaration-carries-a-why) § 2 and § 5.
