# S1f.10.9 — What the blind enumerator takes for an object

**Phase:** [P1f.10](README.md) (The structure of the hypothesis set)
**Status:** planned 2026-11-12. Not started. **It stops for the user** after its census, before anything
changes.
**Depends on:** nothing in the phase. It may run before [S1f.10.1](s1f.10.1_exclusion_census.md), whose
census reads the same candidate sets.
**Source:** row 56 of [section J](../p1f.6_calculator/questions.md#j-findings-that-need-an-owner) of the
calculator's questions, E-1 and E-7 of its [`findings.md`](../p1f.6_calculator/findings.md). The row's
suggestion was *(b) P1f.10, which plans how the candidates are built*; the user left the line empty on
2026-11-12, and an empty line takes the recommendation
([round 2, § Left empty](../p1f.6_calculator/questions_round2.md#empty)). So this stage stands on an empty
line and is dropped if the user names the row.

## The two findings

- **E-1. `__closed__` is proposed as an object.** Where the blind enumerator builds H for a declared
  relation and the program writes a `(__closed__ …)` fact of another, the dunder is among the candidate
  objects, so `(total __closed__)` is a candidate beside `(total 0)` to `(total 9)`. The witness: a program
  that imports the calculator's prototype, which closes its own relations, and declares `(relation total
  Number)` without closing it: 11 candidates, k_s 2 048 (2026-11-11).
- **E-7. The blind enumerator proposes as objects the relation names a rule asserts as values.** The
  witnesses are the import probes' `liquid` and `thing`; E-1's neighbour.

Both load, and both are *open, shown, no owner yet* in the findings. The user ruled the case behind them (a
program's own open relations beside the theory) *left as it is*; these rows are the roughness inside it.

## Why it is not simply a fix

**Removing a candidate changes H, and H is part of the answer.** k_s 2 048 on E-1's witness counts states
that hold `(total __closed__)`; without the candidate the count is another number. This phase's rule is
that it *may not change an answer. Not one model, not one fact, not one verdict word*. So the stage is the
one place in the phase where an answer would move, and it moves only if the user rules that today's is
wrong:

- The candidates are the program's names by ruling: *a constant a fact names at any depth, or a rule's
  `:match` or `:assert`, is a candidate* (Q-M1e2.9 (a)), and a rule's constant is a name of the program
  (Q-M1e.21 (b)). A relation name a rule asserts as a value is, read literally, such a constant (E-7).
- `__closed__` is a kernel name a program may not bind (Q-M1f.7 (b), *"Never bound"*) and *uses … as a
  relation or a value*. A written `(__closed__ R)` fact names `R`, not the dunder, as a constant; whether
  the dunder itself is a *name of the program* in the sense of Q-M1e2.9 is what E-1 asks.
- Where the blind enumerator binds a **type** as an object, every read-out already says so (`types as
  objects`, Q-M1e.12 (a): *the fixture, with the warning*). That is a precedent for the other reading:
  keep the candidate, and say it.

## Tasks

1. **T1f.10.9.1 — the two witnesses rebuilt and banked** under `build/` first, on the release binary and the shipped stdlib as they
   stand (both were found on the calculator's prototype): H listed, the candidates that
   are a dunder or a relation name marked, R with and without them computed by `ein oracle` on a copy of
   the program that closes the relation.
2. **T1f.10.9.2 — the census.** Over every corpus program: how many candidates of H are `__closed__`,
   `__symmetric__`, or a name that the program uses *only* as a relation or only as a relation name in
   value position; and how many programs' R would differ without them. Programs that state their lattice
   or close their relations have no such candidate; the census says how many are left.
3. **T1f.10.9.3 — the question, and the stop.** Under the next free id in
   [`open_questions.md`](../open_questions.md), with the census: (a) leave it, and the read-out names such
   candidates as it names types bound as objects; (b) a kernel dunder is never an object, which is a
   wrong answer today and a register row before its fix; (c) (b), and a relation name asserted as a value
   is not an object either, which narrows Q-M1e2.9 and needs the user to say so. **Default: (a) for E-7,
   since it is the ruling as written; (b) for E-1, since no program means a dunder as an object.** The
   default is not acted on before the user replies.
4. **T1f.10.9.4 — as ruled.** Under (b) or (c): the register rows filed `open` first, with the witnesses
   under `examples/ein-bugs/wrong-answers/`; the fix in `invariant::program_names` or the enumerator; the
   sweep, old binary against new over the corpus, with every moved R listed; the rows flipped in the fixing
   commit; `solution_semantics.md` § 1.2 and `defined_behaviour.md` restated. Under (a): the read-out's
   `universe` row and `root.hypgen` gain the count, with [`read_out.md`](../../../docs/kernel/inference/read_out.md)
   and its reference test in the same commit.

## Acceptance

- The two witnesses and the census are on the stage's record, each with its command.
- The user has ruled; E-1 and E-7 are closed in the calculator's `findings.md` with what was done.
- If an answer moved, it moved by a register row, and the phase's README names this stage as the one
  exception to *no answer changes*.

## What it must not do

- **Not remove a candidate before the ruling.**
- **Not reopen the ruling on a program's own open relations beside a theory:** left as it is.
- **Not touch the obligations rung or an hrule's candidates:** the findings are the blind enumerator's.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Config never changes an
answer. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` after a code commit;
`utils/ci_container.sh` is not needed.
