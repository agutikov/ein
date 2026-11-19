# S1f.21.7 — the debt's term

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [C](modifications.md#c-q-m1f390-a-and-b-the-verdict-stays-a-verdict), items 14 to 16.
**Waits on:** [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact).
Under (a) there is nothing to build and the stage is not run. It is run under (b), the default, and under either
held form of [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules),
where the term is the J of `(open F J)`.
**Writes:** `ein-infer/src/obligations.rs` and what prints a debt; `docs/ein/ein.rs/05-events.md` where the `owe`
event gains a key; the tests of `ein-infer/tests/obligation_reports.rs`.

## What it does

A debt is an instance of an obligation rule: the rule, its activator's arguments, its trigger's bindings
(`obligations::Owed`). The kernel computes it already and prints its `:why`. The stage gives it one written form.

1. **One function writes the instance as a term**, a fact of the rule's name:
   `(total-owed seats is-a Guest Chair Ann)` ([`design.md` § 4.1](design.md#41-what-the-justification-is), J3, the
   one candidate under which a tuple is a debt). The term is made by the kernel and held by no knowledge base.
2. **Where it is printed.** **Reading 1**, default: on the `owe` event as `term`, and in `--dump-lattice`'s
   `nodes.json` beside `owed`; the read-out's `owed` rows keep the rule's `:why`, which is the sentence a reader
   wants. The other readings: nowhere yet (the function and its test only), or on the read-out too.
3. **The test**: a node's terms are as many as its `owes`, and two instances an obligation owes over one relation
   are two terms, which is the first instruction's property (1) answered.

**Not built**: item 16, the rule writing the term as a second argument of its verdict. The four obligations of
the standard library and every program's own stay as written.

## What must hold

- **No answer moves**, and no count: `owes` at every node as before (`obligation_reports.rs`, the corpus sweep).
- A key added to an event is the protocol's: [`05-events.md`](../../../docs/ein/ein.rs/05-events.md) in the same
  commit, and `ein-cli/tests/events_reference.rs`.
- No golden moves but those that hold an `owe` line or a node's record.

## Acceptance

`ein solve examples/lattice/05_owed_seats_bounded.ein --events` prints four `owe` lines at root with four distinct
terms; the test of task 3 over every `tests/stdlib/` program with an obligation; one gate.

## Record

Nothing yet.
