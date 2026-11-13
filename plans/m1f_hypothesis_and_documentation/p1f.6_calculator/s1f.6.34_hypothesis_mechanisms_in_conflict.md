# S1f.6.34 — A conflict between the hypothesis mechanisms is reported

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-13, on the user's ruling of that day on E-47. Not started. **An engine stage.**
**Depends on:** nothing in the phase. Independent of the stdlib layout. It owns
[E-47](findings.md) of the phase's findings.

## The ruling

[S1f.6.28](s1f.6.28_word_problems_second_pass.md)'s T4 filed E-47: *forbid*'s refusal names
`(solve :no-hypothesis (R))` as its first remedy, and on a program where an hrule proposes facts of R the remedy
does nothing, so the refusal stands word for word on the program that wrote it. The finding asked whether the
message should stop naming `:no-hypothesis` there, or `:no-hypothesis` should also filter an hrule's candidates.
The user, 2026-11-13, on that question:

> E-47: report conflict if any hypothesis mechanism constradicts itself

So neither of the two: **a program whose hypothesis declarations contradict one another is told so**, and the
contradiction E-47 met, `:no-hypothesis (R)` beside an hrule that proposes R, is one case of it, not the whole.

## What is known

The ladder, as `AGENTS.md` and [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
state it: the puzzle's `(hrule …)` if it declares one, else the facts that would discharge what the state owes
(`oblgen.rs`), else the blind enumerator (`hypgen.rs`). What scopes each, read off the code on 2026-11-13:

| declaration | read by | not read by |
|---|---|---|
| `(solve :hrules …)` | the hrule rung: which hrules, with which activators | the obligation rung, the blind enumerator |
| `(solve :hypothesis-relations …)` | the blind enumerator (`hypgen::relation_plan`) and the obligation rung (`oblgen::scoping`), *speculate only these* | the hrule rung |
| `(solve :no-hypothesis …)` | the same two, *never these* | the hrule rung |
| `(__closed__ R)`, a stated or derived closure | the same two, before the lists | the hrule rung |

So **every scoping declaration is silent toward the hrule rung**, and a program can state that R is never
guessed, or that only S is, while an hrule it activates guesses R. E-47's witness is
`build/s1f.6.28/p2/kinds_listed_has_hyp_stated.ein` (gitignored; the stage re-creates it): an hrule supposing
`(has ?p apples (n z 1) ?t)`, `(solve :hrules (maybe-apples) :no-hypothesis (has))`, refused by *forbid* with
the remedy it already wrote. Without `listed_is_all` its twin loads, and the read-out's `universe` row reads
*1 candidate* of `has` with `:no-hypothesis (has)` written.

## Tasks

1. **T1f.6.34.1 — the census of contradictions, on real programs, shown before the loader moves.** Every pair
   of hypothesis declarations that can say opposite things of one relation, at least: an active hrule whose
   `:assert` heads a relation that `:no-hypothesis` lists, that `:hypothesis-relations` omits where it is given,
   or that a stated `(__closed__ R)` closes; a relation both lists name. For each: a witness program, what each
   command answers today, and how many corpus programs (`examples/`, `tests/`, `stdlib/`) carry it, by a
   script, never by reading the directories. A relation an hrule proposes only through a parameter (`?R`) is
   the activator's, and the census says how the check reads it. Recorded as the record's first section.
2. **T1f.6.34.2 — the report.** **Default, the user's to revise:** a load refusal, one line per conflict,
   placed at the declaration, on every command, exit 1 (`ein test`: 2), naming both sides, as
   [`defined_behaviour.md` § 4.20](../../../docs/kernel/defined_behaviour.md#420-a-solve--list-names-what-the-program-has--m1f-s1f78)
   refuses a list that names nothing. It is the form the language's other contradictions take, and it keeps
   *no config option changes R* true, since nothing is filtered differently. A corpus program the census finds
   in conflict is shown with its answer before it is refused, and is either corrected, its conflict being an
   accident, or kept as a fixture of the refusal. Every new refusal a section of § 4 and a row of the grammar
   page's table where it is a shape; Q-M1f rows for the defaults.
3. **T1f.6.34.3 — *forbid*'s remedies.** A remedy that the loader would now refuse as a conflict is not offered:
   where R's candidates include an hrule's, the message names the hrule (deactivate it, or let it assert the
   relation the claim does not read) and not `:no-hypothesis`. Held by a test on E-47's witness.
4. **T1f.6.34.4 — the pages and the records:** `defined_behaviour.md`, the grammar's § Solve,
   `06_reserved_names.md` § Hypothesis / query control (what each declaration scopes, the table above made
   normative), the read-out reference if a printed line moves; E-47 closed in [`findings.md`](findings.md); the
   phase README's row; the stage's record.

## Acceptance

- Every pair of hypothesis declarations that can contradict one another over a relation is listed in the record,
  with its witness, and each is reported by the loader or named there as not a contradiction, with the reason.
- E-47's witness is refused for the conflict, naming the hrule and the list, and no longer for *forbid* with a
  remedy it already wrote; its twin without `listed_is_all` is refused for the same conflict.
- No program that loads and has no conflict changes its answer or its output: the sweep over every tracked
  program, before and after.
- *Forbid* offers no remedy the loader refuses.

## What it must not do

- **Not make `:no-hypothesis` or `:hypothesis-relations` filter the hrule rung**, nor the hrule rung read them:
  the ruling is to report the contradiction, not to resolve it one way.
- **Not move R** on any program that loads with no conflict; no lever, option or flag decides a conflict.
- Not touch F25's R-2 and R-3 (a bare name in `:hrules` that selects nothing) unless the census finds them to
  be a contradiction between two mechanisms, in which case they are shown and the stage says which.

## The standing rules

A program the stage newly refuses is shown with its answer first. A wrong answer of the engine met on the way is
a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Findings go to [`findings.md`](findings.md)
under the next free id; questions to [`open_questions.md`](../open_questions.md) under the next free id. Every
answer exits 0. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` on the stage's last commit;
`utils/ci_container.sh` is not needed unless a DOT writer, the review bundle or what CI installs changes.
