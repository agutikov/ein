# S1f.7.9 — A fact's `:rule` that names no rule

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 4), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
Not started. **Reduced to its rows on 2026-11-11** (the user: *"yes S1f.7.9 moves to a new F24"*): the fix and the
question under it are [F24](../../followups/f24_written_provenance.md)'s, and this stage does T1 and T2 below and
files its rows `open`, owned by F24. The sections after § The finding are the plan as it stood, kept as record.
**Done 2026-11-11, as reduced**: confirmed, and W87's three rows filed `open`, owned by F24
([the records](#t1-record-2026-11-11)). T3 and T4 are F24's.
**Depends on:** nothing.

## What the analysis of 2026-11-11 found

A written `:rule` fact is a derivation with no premises, whether or not its rule is declared, so the fact leaves
every core: `(p k :rule zzz)` with `p → q` and `q ∧ p → (false)` prints `unsat core ∅`, where the program without
`:rule` gives `(p k)`. `:rule r`, a declared rule, and `:using ((nope x))`, a fact never held, do the same. R is not
moved. A `--dump-answer` state file writes no `:rule` (its provenance is a comment), so no dump depends on the form.
Refusing an unknown name would leave the empty core, and would move `examples/syntax/unary-and-nary.ein`,
`examples/broken/load/derivation_cycle.ein` and four test files that write provenance by hand. The witnesses this
stage files are the three rows of F24's table.

## The finding

Recorded on the release binary at `f62e34e1`:

> **A fact's `:rule zzz` names no rule, in silence**: `(p k :rule zzz)` loads and `p k` is stored as derived by a
> rule the program does not declare (`ein saturate --dump`: `(p k :rule zzz)`).

So a written fact can claim a provenance the program does not have. Every surface that names a fact's rule (the
trace, the cores' `refuted by`, the natural-language rendering) then names a rule that does not exist.

## What T1 has to settle first

**Who writes `:rule` on a fact.** `ein saturate --dump` prints derived facts with `:rule`. If such a dump is meant
to load on its own, without the rules that derived it, then a `:rule` that names no rule of the reloaded program is
the dump's normal case and not a mistake. T1 lists every writer and every reader of a fact's `:rule`, and whether
anything reloads a dump.

**Then the direction.** If nothing reloads a dump without its rules, the direction that agrees with S1f.7.8 is to
refuse at load a `:rule` that names no rule. If something does, the choice is between that refusal, a provenance
the engine marks as not checked, and keeping it as it is but saying so on every surface. That choice is a question
for the user.

## Tasks

- **T1f.7.9.1, confirmation and the census.** Re-take the finding. List the writers and readers of a fact's
  `:rule`, and which surfaces print it.
- **T1f.7.9.2, the rows.** One witness per surface that names a rule the program lacks. Each is filed `open` and
  held to today's engine.
- **T1f.7.9.3, the fix.** The chosen direction, with the rows flipped and a test on every command.
- **T1f.7.9.4, the pages.** `01_grammar.md` (a fact's clauses), `defined_behaviour.md`, the dump's page, and the
  stage's records.

## Acceptance

- No surface names a rule the program does not declare, unless it says the provenance was not checked.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `91b12be8`, all three rows of
[F24](../../followups/f24_written_provenance.md)'s table. One guest, `seat` (paid → seated) and `clash` (seated ∧
paid → `(false)`):

| the program writes | `ein saturate --dump` | `ein solve` |
|---|---|---|
| `(paid Ann :rule zzz)`, `seat` alone | `(paid Ann :rule zzz)` among the derived facts | R {∅} |
| `(paid Ann :rule zzz)`, `seat`, `clash` | the same, and `(false :rule clash)` | R empty, root dead, `unsat core ∅, the empty set: the refutation rests on no fact`, `refuted by clash` |
| `(paid Ann)`, `seat`, `clash` | `(paid Ann)`, a given | R empty, `unsat core (paid Ann)` |
| `(paid Ann :rule seat)`, a declared rule | `(paid Ann :rule seat)` | the empty core, as `zzz` |
| `(paid Ann :rule seat :using ((invited Zoe)))` | `(paid Ann :rule seat)`, the `:using` not printed | the empty core |
| `(paid Ann :using ((invited Zoe)))`, no `:rule` | `(paid Ann)`, a given; the `:using` is dropped | `unsat core (paid Ann)` |

The summary writes `"core": []` and `nogoods.json` the same for ∅'s member of N. R is moved by none.

**Who writes a fact's `:rule`, and who reads it.** Writers: `ein saturate --dump`, for every derived fact, and the
`--dump-answer` state file, which writes the program as loaded, so a written `:rule` again, and each fact of the
state with its provenance as a comment, `(paid Ann) ; rule zzz — Ann has paid`. By hand: three corpus files,
`examples/syntax/unary-and-nary.ein`, `examples/broken/load/derivation_cycle.ein` and
`examples/ein-bugs/wrong-answers/open-fact-clause.ein` (`git ls-files '*.ein' | xargs grep -n ':rule \|:using'`,
2026-11-11), and the four test files F24 names. Readers: the loader, which stores the name as the fact's provenance
and the `:using` facts as its premises and checks neither; `ein render kb`, which labels the edge `paid by zzz`; the
core search, which follows a derivation with no premises to nothing. Nothing reloads a `saturate --dump`, and a
state file reloads by its facts, whose provenance is a comment (`answer_round_trip.rs`).

The direction is not taken here: it is F24's question, and the user's.

## T2 record, 2026-11-11

Three witnesses and one twin under `examples/ein-bugs/wrong-answers/`, each with a corpus entry (`solve`,
`saturate`), case **W87**, each row `open`, its `owner` F24, its review `deferred`:

| row | witness | `now`, held by `wrong_answers.rs` | `right` |
|---|---|---|---|
| `W87/written-rule-no-such-rule` | `written-rule-no-such-rule.ein` | `ein saturate --dump` answers | refused, under F24's (b); (a) refused and (c) loads, marked, as alternatives |
| `W87/written-rule-leaves-core` | `written-rule-leaves-core.ein`, twin `-twin.ein` | `ein solve`: R empty, root dead, core `[]` | core `(paid Ann)`, under (c); (a) and (b) refused as alternatives |
| `W87/written-using-never-held` | `written-using-never-held.ein` | `ein solve`: R empty, root dead, core `[]` | core `(paid Ann)`, under (c); (a) and (b) refused as alternatives |

**`right` is not ruled.** The register's schema requires one, and an existing deferred row whose reading is the
user's records it as the answer under one named reading with each other reading an alternative: `W8/features-07`,
for Q-M1e2.2 and F21. W87's rows follow it
([Q-M1f.134](../open_questions.md#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled),
defaulted). The class of each row is *contested*, which `ein-corpus`'s `an_open_row_is_open_by_the_method` reads.
Which reading a row names rules nothing.

The rows are held to today's engine: `ein-corpus`'s tests, `ein-cli`'s `wrong_answers.rs` and `ein-infer`'s
`register_region.rs` green, and `python3 utils/review_bundle.py --smoke`, in the gate, reads them. The register
reads 301 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11):
four of them F21's, eight F22's, one M1f S1f.10.6's and these three F24's (each open row's `owner`).
**A fourth surface, found by the gate.** `written-rule-no-such-rule.ein` is the first corpus program `ein solve`
answers that writes a fact with a `:rule`, and `ein-cli/tests/answer_round_trip.rs` failed on its state file:
*models/01.ein: 1 facts reload with a provenance, not as given*. A state file writes the program as loaded, so the
written `:rule` with it, and the reloaded state holds `(paid Ann)` as a fact derived by `zzz`, where every fact of
a state file is a given. It is the row's own wrong answer, so the test names the file, `WRITTEN_PROVENANCE`, as
`DO_NOT_LOAD` named W30's and W34's, and holds it both ways: the commit that fixes W87 takes it off the list.
F24's page had said no dump depends on the form; it is restated.

The goldens gained the four files' lines: `corpus_exits.txt` (two runs each, exit 0) and `corpus_shapes.md5`. No
engine source changed for this stage.

F24's § Rows names the three and carries what this T1 added. Filed in `43587986`, gate green (`build/gate-s1f.7.12.log`).
