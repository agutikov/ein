# Q-M1f.553 — A dead state holds a contradiction, in the words of every surface

Every surface says of a dead state that it **holds a contradiction**: the read-out's `∅` and `empty` rows (*root's
own fixpoint holds a contradiction*), the answer dump's and the region drawing's legend, the h-space's sentence and
the pages that quote them. The other words of a dead state are the same whatever ended it, `std.core`'s rule on a
held `(false)`, `std.contradiction`'s on a fact beside its negation, or a program's own rule that states
`:ein inconsistent`: `:expect (false)` is a claim's word for an empty R, an `enter` event's `kind` is `dead`, and the
`refuted by` row names the rule that concluded the `(false)`, or the rule that stated the word where no `(false)` is
between ([Q-M1f.551](../stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)).

**Why.** A dead state is one in which a rule that states the engine's word fired
([Q-M1f.550](../language/Q-M1f.550-the-engines-word-is-inconsistent.md)), and the sentence *holds a contradiction*
is true of each way that happens: of a held `(false)` and of a pair a rule refutes. A wording that named the rule or
said *derives (false)* would be false of one of them.

**Held by.** `ein-cli/tests/read_out_reference.rs`, which holds the read-out's rows to
[the read-out reference](../../ein/user/04-reference/08-read-out.md) and its examples page;
`ein-cli/tests/no_search.rs`' `solve_reports_root_and_says_r_was_not_searched`; `ein-render/src/region_dot.rs`'
legend; the shape golden, `ein-render/tests/golden/corpus_shapes.md5`.
