# S1f.21.24 — a hypothesis rule reads its `absent` premises

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **not started**.
**Writes:** `ein-infer/src/hypothesis_rules.rs`, `open.rs` and what reads a rule's `(open F)` conclusions, their
tests; `plans/wrong_answers/register.toml` and its witness, then `corpus/regressions.toml` and the corpus; the
oracle's listing of a node; two help texts; the pages and goldens that move.

## What it is

[V-89](findings.md#v-89--a-rule-that-concludes-a-hypothesis-reads-none-of-its-absent-premises), a wrong answer the renders stage found and the coordinator checked: a rule that
concludes `(open F)` is matched without its `absent` premises. Review item 62's probe,
[`probe-hypothesis-rule-absent.ein`](review/part-2/62-what-a-fork-drops/programs/probe-hypothesis-rule-absent.ein):
one match, `(and (p ?x) (absent (q ?x)))`, under a rule that concludes a fact and under one that concludes a
hypothesis; with `(q a)` written the first concludes `(s b)` alone and the second writes `(open (r a))` and
`(open (r b))`. The program is monotone by the static check, so the engine claims $H$ of two and $R$ of four
states where the match gives one and two, and `ein oracle` agrees, building $H$ the same way. The engine before the
phase did the same.

**The reading taken, a default ([Q-M1f.685](../open_questions.md#q-m1f685--a-rule-that-concludes-a-hypothesis-reads-its-absent-premises))**:
the `absent` is read, as every rule's is. The syntax reference says a hypothesis rule's `:match` reads the state it
is asked about, and the user ruled a hypothesis rule a rule that concludes `(open F)` and nothing apart
(Q-M1f.567).

## Tasks

1. **The wrong answer filed**, by [the register's rules](../../wrong_answers/README.md): a row of
   `plans/wrong_answers/register.toml` with its witness under `plans/wrong_answers/witnesses/`, owned by this stage,
   in a commit of its own, `python3 utils/wrong_answers.py` green on it.
2. **The fix.** Where a rule's `(open F)` conclusions are read, at root for $H$ and at each state the pass writes,
   the guards the plan lifts out of the match are judged as the saturator judges them for a rule that concludes a
   fact: against the state's fixpoint. A rule that concludes a fact and a hypothesis in one `:assert` gives both
   of the same matches.
   - **The static check.** A hypothesis rule whose `absent` reads a relation a commitment can change proposes at
     root what a state above it would not: say what the check's sites make of it today, and whether a site is
     owed. A change to what is flagged is shown, program by program.
3. **The corpus.** Every program whose $H$ or $R$ moves is shown with both answers. Each is a program whose
   hypothesis rule has an `absent` premise the filter of $H$ did not already cover. Its new answer is its match's;
   an `:expect` that stated the old one flips in the fixing commit, as the register's rule has it. **Stop and
   report**, with the program, where what a program meant is not plain from its text.
4. **The row moved**, in the fixing commit: into `corpus/regressions.toml`, its witness into
   `examples/ein-bugs/wrong-answers/` with its corpus entry and its line in the catalogue.
5. **[V-91](findings.md#v-91--no-command-prints-the-open-facts-of-a-state-above-root-outside-r)**: the oracle's listing of a node lists the node's `open` facts, as a listing of
   a state does (Q-M1f.572); `-P` leaves them out (Q-M1f.577).
6. **[V-93](findings.md#v-93--pages-and-help-texts-that-say-what-the-binary-does-not)'s help texts and [V-94](findings.md#v-94--a-test-of-the-obligations-reports-checks-proposals-by-short-names-only)**: `ein show --help` and
   `ein show hyps --help` in the words the engine uses; `08-read-out.md`'s sentence on the licence; the test of
   the obligations' reports compares full names.

## What must hold

- Every mode agrees with `ein oracle` on every corpus program, and **the oracle is right on the probe**: a test
  holds $H$ to the match by a second route that does not read the function under repair, the rule that concludes a
  fact of the same match.
- A program whose hypothesis rules have no `absent` premise does not move: its `ein solve -p` and the digest of
  its events are byte for byte what they were.
- No config option, lever or flag changes $R$. A flagged program is claimed nothing of.
- The gate on the last commit, and the two slow render sweeps.

## Record

Nothing yet.
