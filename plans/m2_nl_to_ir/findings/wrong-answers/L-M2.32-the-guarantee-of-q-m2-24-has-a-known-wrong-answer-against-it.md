# L-M2.32 — The guarantee of Q-M2.24 has a known wrong answer against it, the register's `W8/fifo-gap-root`

**Status:** open, as a known wrong answer; **the user ruled the choice 2026-11-27**: *"L-M2.32: keep, as
recommended"*: the guarantee stays as Q-M2.24 ruled it, for every program, and is not scoped to programs order-free
at the boundary; the root race stays a wrong answer by it, `W8/fifo-gap-root`'s, until M1n's stage fixes the
boundary's order
**Found:** by [S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md), 2026-11-27, writing Q-M2.24's decision, on
`e2b7c0c9`'s gate binary
**Owner:** the register's row's, S1n.2.1 ([M1n](../../../m1n_non_monotone_programs/README.md), P1n.2), which the
decision now bears on; nothing in this phase

## What

**The user ruled that an answer does not depend on the order a program's forms are written in, on the premise that
nothing does** ([Q-M2.24](../../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md):
*"nothing depends on lines order; … so if will ever see different behaviour it would be considered a bug"*). The
premise holds of the load, which reads the whole program before anything is asked of it. **It does not hold of a
saturation's boundary**, which admits the rules waiting on an `absent` first in, first out in their written order
within each half of the agenda (Q-M1f.313): where a rule's `absent` reads a relation another such rule concludes, the
rule written first is admitted first and can decide the fixpoint, root's as a node's. Ein's own pages say so
([`03-absent-and-naf.md` § What is not provided](../../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md#what-is-not-provided),
[`08-monotonicity.md` § 8.3](../../../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#83-the-fifo-gap),
Proposition 2 of [`03-transformations.md` § 3](../../../../docs/ein/reasoning/03-rule-evolution/03-transformations.md#3-what-leaves-r-unchanged),
which proves the guarantee only for a program order-free at the boundary), and the register has the case at root
since M1f's close: [`W8/fifo-gap-root`](../../../wrong_answers/register.toml), owner S1n.2.1, a program the static
check calls monotone whose query answers `?x = k` with the reader written first and has no row with the producer
first.

**What the stage did with it**, as its default (the ruling read as written: *different behaviour … a bug*): the
decision states the guarantee for every program, and says where a run falls short of it today, the boundary's order,
*of a program the static check passes … a wrong answer by this decision, known and not fixed; of a program it flags,
nothing is claimed* (Q-M1f.564). It does not scope the guarantee to programs order-free at the boundary, which is
the other way to make the decision true of today's binary, and the user's to choose instead. **The user kept the
default**, 2026-11-27: *"keep, as recommended"*.

**What the decision now bears on**, each outside this phase:

- **`W8/fifo-gap-root`'s right answer.** Of the row's readings, today's ((A) fork and (B) scratch, admitting in FIFO
  order) and (C) *guards are premises* each answer by the written order (the row says so of each), so under Q-M2.24
  none of them is a right answer; stratum-order admission, the row's primary `right`, is the one reading it lists
  under which both orders answer alike. The row's `question` list does not name Q-M2.24. S1n.2.1's.
- **The project's statements that the order decides nothing on a monotone program**: Q-M1f.313's *Why* (*"On a
  monotone program the fixpoint does not depend on the order, so the order decides no answer"*), which the root
  race contradicts, is among *the three statements that say more than holds*, the FIFO gap's call 3, P1n.2's. The
  reasoning pages state the order dependence as what is computed and do not say it is a wrong answer; whether they
  cite the decision there is the same call.
- **A flagged program with two supported fixpoints** (`08-monotonicity.md`'s c8, an even cycle through an `absent`):
  the written order picks one, and no order-free answer exists short of another semantics (stable models, a refusal),
  which is M1n's subject. The decision claims nothing of it, by Q-M1f.564.

## Evidence

[`../../probes/order-of-forms/`](../../p2.2e_kernel_fixes/probes/order-of-forms/), `run.sh`: `gap-reader-first.ein` and
`gap-producer-first.ein` (the register's witness and twin, renamed: two rules, `(a k) (b k)` given, `(solve
:no-search)`) differ under every question but `--count`, the query's class `?x = k` against *the goal has no row*,
both read-outs saying `lattice monotone (the static check)`. The other pair, `one-food-each-ann-first.ein` and
`-carl-first.ein`, is what the decision says may move and does: root's owed instances listed in the written order,
and the member `-n 1` meets first; every other question alike.

## What it costs

To the application, little today: the six Zebra encodings answered alike in both orders (E-112), and a program whose
rules race at the boundary is one the steps can write. Such a run, if a stage meets it, is filed as a wrong answer
and not answered by reordering what the steps write. To Ein, a guarantee stated with one known wrong answer against
it, until M1n's stage decides the boundary's order or the user scopes the guarantee.
