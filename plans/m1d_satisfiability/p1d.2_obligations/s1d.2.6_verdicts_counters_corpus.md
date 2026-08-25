# S1d.2.6 — What it changes: verdicts, counters, corpus

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 2 days
**Depends on:** [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md); reads
[S1d.2.5](s1d.2.5_hypotheses_from_obligations.md)'s re-baseline if it has
landed, and does not wait for it.

## Context

The phase's ledger stage: the openness census, the verdict word, and
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
moved with fixtures rather than prose. Until this stage, every verdict word
is unchanged (S1d.2.4 shipped a report line); this is where the phase's
"where the two definitions disagree, that entry is a test" bullet is paid.

**The word is decided: `Open`** — the user's triple (`open` / `false` /
`satisfy`) carried into the rendering, with the count attached:
*"Open — owes n (…)"*. `Incomplete` was `ideas.md`'s word for the same
outcome; the proposal's supersedes it, and the decoupling note in
[`obligation_forms.md` § The naming menu](obligation_forms.md) already said
the printed aggregate is free to carry the count. What this stage decides is
not the word but **which states get it**, measured first.

**The scope rule, decided 2026-08-25 — a program that states no obligation
keeps today's verdict.** Without it this stage is not two days and not ten
entries. The read-out "owes = 0 and consistent ⇒ *satisfy*" is vacuously true
wherever nothing is owed, and after S1d.2.4's duals ship the obligations
reach only the programs that declare a property carrying a lower bound:
**23 of the 173 `.ein` files** under `examples/` and `tests/` declare
`bijective`, `total` or `surjective` (16 / 17 / 11, measured 2026-08-25).
The other 150 would all flip to *satisfy* by discharge — the engine calling
a stuck state a model because nobody told it what the state owed.

So the three-state read-out applies **only where the program states at least
one obligation**; everywhere else `complete(kb)` is untouched and the verdict
words are exactly today's. The rule is not a hedge but the honest reading of
what G decides: a state is judged by discharge when it has been told what it
owes, and by exhaustion when it has not. It is also what keeps the phase
additive — the property G was chosen for — and it is **re-openable by
measurement**: an entry that states obligations *and* disagrees with
generator-exhaustion is a test this stage writes, and if the census finds the
unobligated majority disagreeing in some way that matters, that finding
re-opens the rule with a number attached rather than by argument.

## Tasks

### Task T1d.2.6.1 — the openness census

Every corpus entry, root quiescence: owes how much, discharged how far, and
under the S1d.2.5 rung (where landed) per explored node. Banked as
`openness_census.md` beside this file — the layer census's sibling, taken
with the engine's own tally (T1d.2.4.5 proved it against the hand count).

### Task T1d.2.6.2 — the ten entries, partitioned

Q-M1d.6's list — **and it is the ten and not the corpus because of the
scope rule above**; an entry among them that states no obligation is out
of scope by that rule and keeps its word. Each classified by the census: **owes something reachable**
(⇒ `Open — owes n`), **owes nothing** (⇒ the vacuous edge: a consistent
quiescent state with no obligations is *satisfy* under closed-world
completion — `ideas.md` § "Когда fixed point является решением" — where
today's `complete(kb)` says `Contradiction k=0`), or **genuinely dead** (a
scan fires; today's word was right). The classification is the census's,
not this file's guess.

### Task T1d.2.6.3 — the verdict change, as fixture moves

For each entry whose word moves: the golden, the exit expectation, the
[`features.md`](../../../docs/kernel/inference/features.md) lever-table rows
written in the old word, and the corpus exit table. Exit codes: `Open`
exits like today's `Contradiction`-at-cap did (0) — the *claim* channel is
`:expect`, unchanged; a program may now state `:expect` against an open
verdict only if S1c.1.2's grammar grows a word for it, which is **not** this
stage (noted for [P1d.4](../p1d.4_model_set_closure/README.md)).
`exhausted` keeps its meaning throughout — Q-M1d.1's constraint.

### Task T1d.2.6.4 — Q-M1d.6 closed

The written answer: `Contradiction` is never again said of a state that owes
something it can still pay — candidate (c), with the user's word. The ten
entries' new lines quoted in the question's closing entry, and the
`docs/api/inference.md` parenthetical (the one page that tied the word to
exhaustion) cited as the reading that won.

### Task T1d.2.6.5 — the phase ledger

The closing record in the phase README: decisions taken and where, stages
landed with commits, the two censuses, what was deferred (E, A, numeric
bounds) and what evidence un-defers each. The rule from the milestone: a
deferral is cheap to reverse only while the specification survives it.

## Acceptance

- `openness_census.md` banked, engine-tallied, all entries — including the
  ones the scope rule excludes from the read-out, because the census is what
  makes the exclusion a measurement rather than an assumption.
- **The scope rule holds on the corpus**: every entry stating no obligation
  reports exactly the verdict word it reported before this phase, and the
  count of such entries is in the census.
- The ten entries each carry their measured classification and their new
  verdict line; no entry's word moved without its census row.
- `complete` means **discharged** in the verdict read-out (`false` outranks;
  owes = 0 and consistent ⇒ satisfy), and the entries where that disagrees
  with generator-exhaustion are exactly the tests this stage added.
- Q-M1d.6 status: closed, with the fixture list. Q-M1d.1 untouched —
  `exhausted` still means the lattice.
- The whole gate green after the one re-bless this stage owns.
