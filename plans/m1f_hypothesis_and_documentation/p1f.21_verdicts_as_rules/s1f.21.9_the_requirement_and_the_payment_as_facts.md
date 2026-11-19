# S1f.21.9 — the requirement and the payment as the kernel's facts

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started, and run only on a ruling for (c)** of
[Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact).
**Group:** [E](modifications.md#e-q-m1f390-c-the-requirement-and-the-payment-as-the-kernels-facts), item 20.
**Waits on:** that ruling. Under (a), (b) or the user's form it is not run.
**Writes:** the compiler (`ein-infer/src/compile.rs`), `ein-core/src/terms.rs` for two reserved relations, the
read-out, and every golden of a program with an obligation.

## Why it is planned and not recommended

(c) is the one option that makes *owed* and *discharged* ordinary rewriting, and the survey calls it the strongest
argument against its own default. Two things planned beside it give most of what it buys without it:

- [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md)'s third module states the requirement, the payment and
  `discharged` as rules a program imports, so a rule can read what is required and what is paid where a program
  asks for it;
- under [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules)
  (b), the candidates are held with the debt each would pay, so *why is this owed* has an object in the graph.

What (c) alone adds is that the kernel makes those facts for every program, at the cost of 78 of the 567 answered
programs' goldens, 12 to 24 facts at the seats program's root and 390 to 478 at `zebra2-obligations`', and a
requirement stated twice in the knowledge base, which
[Q-M1d.2](../../../docs/decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md) chose not
to have.

## What it would do

1. **Examples** (stops for the user): [`design.md` § 2.4](design.md#24-the-verdicts-together-which-may-be-a-fact)'s
   emulation over every corpus program with an obligation, where the survey ran two: every field of `region` the
   same; whether the two counts agree on every dead node a route stops early; the time, by `utils/bench_env.sh`.
2. **The build**: each obligation lowered to two more plans, its trigger and its trigger with its witness; the two
   relation names reserved; *owed* their difference; `(discharged)` a fact a claim may state; the read-out's
   listings and its `outside` row.

## What must hold

$R$ of every corpus program, by the corpus sweep and the oracle (`ein-infer/tests/explore.rs`); the static check's
verdict on every program as before; `ein-cli/tests/lever_invariance.rs`.

## Record

Nothing yet.
