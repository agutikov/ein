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

## The user's simplification, 2026-11-13

> actually we can make it easier: the program ussually has either positive hypothesis generation - by
> obligations or hrules or hypothesis-relations, or negative filtering of all possible hypothesis by
> no-hypothesis and __closed__; are those all 5 control mechanisms? let's make all pairs and analyze potential
> conflicts; I guess it's easy to detect most of them during load, e.g. no-hypothesis make no sense if there
> are hrules - if the conflict then it's conflict, if not - then it filters nothing; filtered obligations are
> nonsense by definition; obligated relation can't be __closed__ also, naming same relations in
> hypothesis-relations and no-hypothesis - also nonsense; combining positives e.g. hrules with obligations and
> hypothesis-relations - maybe possible but not very practical, but they don't conflict at least

## The mechanisms, read off the code (2026-11-13)

`hypgen.rs`'s ladder: **hrules** if the program declares any `(hrule …)`, and then the other two rungs never
run; otherwise **obligations** while the state owes something it may branch on (`oblgen::generate`, *declined*
falls through); otherwise the **blind enumerator**. So the five the user names, and a sixth that no declaration
writes:

| | mechanism | what it does | read by |
|---|---|---|---|
| H | `(hrule …)`, activated by `(solve :hrules …)` | generates, rung 1, alone where declared | itself |
| O | an obligation rule, `:assert (open ?R)`, activated by a fact such as `(total R isa)` | **two roles**: a state that owes is not in R, whatever generated it; and, with no hrule, rung 2 proposes facts of the owed relation | the discharge test; rung 2 |
| B | the blind enumerator | generates, rung 3, the default: no hrule, nothing owed | — |
| HR | `(solve :hypothesis-relations …)` | a whitelist, *speculate only these* | rungs 2 and 3 (`oblgen::scoping`, `hypgen::relation_plan`) |
| NH | `(solve :no-hypothesis …)` | a blacklist, *never these* | rungs 2 and 3 |
| C | `(__closed__ R)`, stated or derived | excludes R from rungs 2 and 3, **and** says R's extent is complete for `absent` | rungs 2 and 3; the matcher |

HR is a filter that the user counts among the positives: it generates nothing of itself. **H and O together are
the Zebra puzzle's own form,** not an impractical one: `examples/zebra2.ein` declares `(hrule guess …)` and
imports `std.bijection`, whose `total-owed` and `surjective-owed` judge every state the hrule reaches.

## The pairs

| pair | verdict |
|---|---|
| H + O | **no conflict**: H generates, O judges (zebra2) |
| H + HR | HR is **dead**, rungs 2 and 3 never running; a **conflict** where an active hrule proposes a relation HR leaves out |
| H + NH | NH is **dead**; a **conflict** where an active hrule proposes a relation NH lists (E-47) |
| H + C | a **conflict** where an active hrule proposes a closed relation; closing another relation is C's second role and stands |
| O + HR | with no hrule, a **conflict** where an owed relation is outside HR: the owing state cannot branch, *stuck* |
| O + NH | with no hrule, a **conflict** where an owed relation is in NH: *"filtered obligations are nonsense by definition"* |
| O + C | a **conflict** where an owed relation is closed: *"obligated relation can't be __closed__ also"* |
| HR + NH | a **conflict** where one relation is in both; NH naming a relation HR already leaves out is **redundant** |
| HR + C | a **conflict** where a listed relation is closed |
| NH + C | an overlap is **redundant**, both saying the same; never a conflict |

**When each is known.** The lists, an hrule's `:assert` heads (a parameterised hrule's through the activators
`:hrules` gives it), a written `__closed__` fact and a written obligation activator are known at load. A
*derived* closure (the stdlib's closing rules, `(__closed__ fa)` and the like) and a derived activator
(`bijective-setup`'s `(total color-loc is-a)`) are known at root's fixpoint, before any search. So the check runs
once, on root, on every command that reads the program.

E-47's witness is `build/s1f.6.28/p2/kinds_listed_has_hyp_stated.ein` (gitignored; the stage re-creates it): an
hrule supposing `(has ?p apples (n z 1) ?t)`, `(solve :hrules (maybe-apples) :no-hypothesis (has))`, refused by
*forbid* with the remedy it already wrote; H + NH.

## Tasks

1. **T1f.6.34.1 — the pairs on real programs, shown before the loader moves.** For each row of § The pairs: a
   witness program, what each command answers today, and the corpus programs (`examples/`, `tests/`, `stdlib/`)
   that carry it, counted by a script that reads root's fixpoint, never by reading the directories. A row the
   corpus carries as a conflict is shown with its answer; a row the code reads otherwise than the table says is
   a finding, and the table is corrected. Recorded as the record's first section.
2. **T1f.6.34.2 — the report.** A **conflict** is refused, one line per conflict, naming both declarations and
   the relation, with the load-error exit (1; `ein test`: 2), on every command, as
   [`defined_behaviour.md` § 4.20](../../../docs/kernel/defined_behaviour.md#420-a-solve--list-names-what-the-program-has--m1f-s1f78)
   refuses a list that names nothing. A **dead** or **redundant** declaration prints one `note:` line on stderr
   and moves no exit code (the user: *"if not - then it filters nothing"*; Q-M1f row, defaulted). No lever,
   option or flag decides either. A corpus program in conflict is corrected where its conflict is an accident,
   or kept as a fixture of the refusal. Every new refusal is a section of § 4; Q-M1f rows for the defaults.
3. **T1f.6.34.3 — *forbid*'s remedies.** A remedy that the check would refuse is not offered: where R's
   candidates are an hrule's, the message names the hrule, not `:no-hypothesis`. Held by a test on E-47's
   witness.
4. **T1f.6.34.4 — the pages and the records:** `defined_behaviour.md`, the grammar's § Solve,
   `06_reserved_names.md` § Hypothesis / query control (the mechanisms table above made normative), the
   read-out reference if a printed line moves; E-47 closed in [`findings.md`](findings.md); the phase README's
   row; the stage's record.

## Acceptance

- Every row of § The pairs has a witness in the record and behaves as its verdict says: a conflict refused, a
  dead or redundant declaration noted, H + O silent.
- E-47's witness is refused for the conflict, naming the hrule and the list, and no longer for *forbid* with a
  remedy it already wrote; its twin without `listed_is_all` is refused for the same conflict.
- No program that loads and has no conflict changes its answer, its stdout or its exit code: the sweep over
  every tracked program, before and after; a dead or redundant declaration's `note:` is on stderr, and the
  record counts the programs that print one.
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
