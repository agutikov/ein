# Q-M1f.48 — A text surface writes the empty core `∅`

Where a dead node's core was taken and holds no fact, as the dead root of `(rule r () :match () :assert (false))`'s
does, every text surface writes it `∅`: the read-out's `unsat core  ∅, the empty set: the refutation rests on no
fact`, `--trace`'s `Core:` with the same words, `core ∅` in `nodes.md`, `core ∅ by r` in `region.md` and in
`ein render region`'s table, `core: ∅` in its tooltips, and a minimal hole's `dead: ∅`. Every JSON surface writes the
same core `[]`. A core not taken prints as it otherwise does.

**Why.** `∅` is already the read-out's spelling of the empty set (*∅, root itself*), so the text and the JSON's `[]`
say one thing. A core printed fact by fact would print nothing, which reads as *no core*, the very confusion `[]` and
`null` are kept apart to avoid ([Q-M1e2.27](Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)).

**Held by.** `ein_infer::explain::EMPTY_CORE` and `core_text`, which every text surface writes a core through;
`ein-infer/tests/corpus_cores.rs`: `a_rule_with_no_premise_refutes_root_with_the_empty_core`. The row:
[08-read-out.md](../../ein/user/04-reference/08-read-out.md#6-the-listing) § 6.
