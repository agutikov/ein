# F22 — The exact unsat core: re-derive what a full alternatives list refused

**Filed 2026-11-04**, from M1e2's
[S1e2.2.11](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md)
(W10), on the user's ruling of that day on
[Q-M1e2.11](../m1e2_correctness/open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it)
(i): *"(b) Every; defer rest"*. The promise stays *the smallest frontier
over every derivation the program records*; S1e2.2.11 built Q-M1e.15's (3),
which keeps each fact's smallest-closure arrival beside its capped list and
fixes `W10/alt-cap-core`; and what (3) cannot reach is this followup's.

**Widened 2026-11-05**, on the user's ruling of that day on
[Q-M1e2.15](../m1e2_correctness/open_questions.md#q-m1e215--what-does-a-derivation-through-a-forall-rest-on),
(b): a derivation through a `forall` rests on each fact its universal read,
and [S1e2.2.21](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md)
folds each at its own smallest frontier rather than searching the
combinations, which costs ×2 500 a member of N as the search stands
(`zebra2-hints`, 3 ms → 7.5 s, cut by its budget). So a core can be larger
than the smallest over every derivation the program records a second way:
on `examples/zebra2-hints.ein`, 13 of N's 73 cores are larger under the
fold than under the search (S1e2.2.21 T1, 2026-11-05, `build/s1e2.2.21/`).
That residual is this followup's too, held by no register row: the search
it needs is the one the cap's needs, a target's derivations combined where
a per-fact choice cannot see the saving.

**What it owns.** One register row, `W10/alt-cap-shared`
([`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml)), `open`,
reviewed `deferred`, with F22 as its `owner`. The milestone's acceptance,
amended by the user on 2026-11-03 for exactly this kind of row, names it at
M1e2's close beside `W8/features-07` (F21's).

## The case

[`examples/ein-bugs/wrong-answers/alt-cap-shared.ein`](../../examples/ein-bugs/wrong-answers/alt-cap-shared.ein):
`(false)` needs `(x K)` and `(z K)`; `(x K)` has 33 one-given derivations and
a 34th over `(a K)` `(b K)`; `(z K)` has only that pair. Over every
derivation the core is `{(a K), (b K)}`, 2. The 34th is `(x K)`'s per-fact
*worst*, so `Kb::record_justification`'s full list refuses it, and the
arrival (3) keeps beside the list is an `x-chain`: the core is `{(a K), (b K),
(c C01)}`, 3 (`ein solve FILE`, 2026-11-04; an uncapped build reports 2).

**No bounded retention is exact.** A list of fixed length must drop one of
any larger set of pairwise-incomparable derivations, and a program can be
written whose target needs the dropped one; a saving that lies *between* two
facts is invisible to every per-fact ranking (premise count, Q-M1e.15's (1),
(3)). So the fix is not a better ranking.

## The proposal

**Re-derive what was refused, when the search asks.** When a full list
refuses an arrival it does not keep beside it, mark the fact *overflowed* (a
bit in the layer, copied on write as `alts` is, and kept by the `.einb`).
When `explain.rs`'s `build_graph` reaches an overflowed fact, enumerate its
derivations again over the KB the search runs on: every rule that can
conclude the fact, its `:match` run with the conclusion bound to the fact,
and the symmetric mirror's arg-swap; each match an AND-node. Memory stays
the cap's; the cost falls on the search, and only where a list overflowed
and the search's closure reaches it.

What to settle before building it:

- **The same derivations.** A re-derived AND-node must be one the saturator
  would have recorded: its `absent` premises as the boundary records them,
  the binding-key identity (S1e2.2.7), the writeback rule (a premise-less
  record grounds out). The test is the uncapped build: every core equal to
  `MAX_ALT_JUSTIFICATIONS = 1 000 000`'s on the corpus, which S1e2.2.11
  measured equal to (3)'s on all of it but this witness.
- **The search's own budget.** `ExplanationBudget` cuts a label past 64
  environments and reports it (`Explanation::exhausted`); a re-derived
  fact can bring more. The cut stays reported.
- **Where it runs.** Of the corpus's 310 files before the witness was
  banked, ten reached the cap, always on `(false)` in a dead node (the
  `zebra2` family, `lattice/06_friends_tables`,
  `branching/12_typed_blind_solve`, `zebra2-bad` and the `alt-cap-core`
  pair; S1e2.2.11 T1, `ein solve FILE --events`, 2026-11-04); the witness
  reaches it on `(x K)`. An uncapped store costs `zebra2`'s
  describe ×2.15 (528 → 1 133 ms); re-deriving only on demand should cost
  less, and that is the number to take.
- **`explain.rs` reads the KB, not the rules.** Re-deriving needs the
  compiled plans, which live in the saturator; where the search gets them is
  the design question.

The alternative the user did not take, *no cap*, is exact by construction
and the reference this would be held to; it is here as the measurement, not
as the fix.

## Trigger

Any one:

- M1e2's close: the row is named there, and the milestone after it decides
  whether to take this;
- a program added to the corpus whose core moves under an uncapped build,
  which would be a second witness;
- an explanation consumer that needs the minimal core and not a minimal-ish
  one: idea 08's human-style trace, or M2's NL explanation of a refutation.

## Why it is parked

- The user's ruling, above: the rest is deferred.
- No corpus program but the witness written to show it has a core that moves
  between (3) and an uncapped store (S1e2.2.11 T1: 6 510 sweep runs).
- It needs the matcher at explanation time, which no surface uses today.

## Read before deciding

- [Q-M1e.15](../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported):
  the three fixes, priced 2026-08-29.
- [Q-M1e2.11](../m1e2_correctness/open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it):
  why (3) is not exact, and the ruling.
- [S1e2.2.11](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md):
  the measurements, the cap's reach under R's routes, and what (3) costs.
- [`explain.rs`](../../ein.rs/crates/ein-infer/src/explain.rs) and
  `Kb::record_justification` ([`kb.rs`](../../ein.rs/crates/ein-core/src/kb.rs)):
  what the search is offered.
- [`glossary.md` § Smallest contradiction frontier](../../docs/kernel/glossary.md):
  the promise this would make true everywhere.
