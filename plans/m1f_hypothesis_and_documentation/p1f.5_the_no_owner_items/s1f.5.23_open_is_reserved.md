# S1f.5.23 — `open` is reserved: a program writes it only as a rule's verdict

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 3
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.23.1, the explanation the
ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and its one open
part, `open` as a value, was put to the user as
[Q-M1f.66](../open_questions.md#q-m1f66--may-open-be-written-as-a-value).
**Depends on:** Q-M1f.66's ruling, for T1f.5.23.3's scope. It may edit `ein.y`, so it does not run beside
[S1f.5.22](s1f.5.22_unicode_only_in_a_string.md).

## The ruling

The close review's finding 3, as written on 2026-11-10:

> **`(p (open x))` loads and answers**: R has 2 states. Yet no rule may match `(open …)`, the verdict atom. It is
> open whether such a fact should be refused at load.

The user, 2026-11-10: *"new stage, Explain in more details, with program example, `open` is reserved word, here
it appers as what? as relation name? - then it is an error, if not - then explain"*.

## T1 record, 2026-11-10

Taken by the coordinator on the release binary at `21c24c07`.

**What `open` is.** A rule's `:assert` may be `(open)` or `(open ?R)`. That is the verdict atom: the rule
states an obligation, which is not stored, never fires a rule, and is read by one pass over the quiescent KB. The
state it is owed in reports *Open — owes n*. Four stdlib rules are written this way (`total-owed`,
`surjective-owed`, `slot-owed-room`, `slot-owed-fill`). The kernel interns `open` as its symbol 1
(`ein-core/src/entities.rs`).

**Where the grammar lets a program write it.** `open` is one of the grammar's *position words*: a token of its
own that is a keyword in one position and a name everywhere else. That is the design S1f.5.5's paper grammar took
for `model`, `region` and the rest (Q-M1e2.17). The productions that admit it are `Name`, `FactHead` and `ArgHead`
in [`00_ebnf.md` § 2](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md#2-phrase-grammar). The loader then refuses one
case: an `(open …)` form under a rule's `:match` (`from_ir.rs`'s `validate_open`).

| program | `open` stands as | today | under the ruling |
|---|---|---|---|
| `(open x)`, a top-level fact | **a relation name**: the head of a written fact | loads, an auto-vivified relation `open` holding `(open x)`, R 1 state | refused |
| `(p (open x))`, finding 3's | **a relation name**: the head of a nested fact, the value `p` holds | loads, R 2 states: ∅ and `{(p x)}`. The second is from `x`, a name of the program, so H is `{(p x)}`, and has nothing to do with `open` | refused |
| `(not (open a))` | **a relation name**: the head of a stored negation | loads, R 1 state | refused |
| `(rule copy (?R) :match (?R ?x) …)` with `(copy open)` and `(open a)` | **a relation name** passed through a relation parameter | loads; the rule matches `(open a)` and derives `(q a)`, as it does with any other name in place of `open` | refused (the written `(open a)`; and an activator binding a relation parameter to `open`, read as S1f.5.18 and S1f.5.9 read relation names) |
| `:match (p (open ?x))` | **a relation name**: the head of a pattern inside a premise | refused at load: *`(open …)` is a verdict about the KB and is legal only in :assert … at None* | refused, in Ein's words (no `at None`, F23's D-2) |
| `(relation open T)` | a declared relation's name | refused at the parse, *unexpected `open`* (`DeclName` excludes it) | unchanged |
| `(p open)` | **a value**: a name `p` holds, not a relation | loads | Q-M1f.66 |

**The census.** No program under `examples/`, `tests/`, `stdlib/` or `corpus/` writes `open` anywhere but in a rule's
`:assert` and in the refusal fixtures (`examples/broken/load/reserved_open_*`, `examples/broken/open_*`). So the
refusals move no corpus answer. They move only the witnesses this stage writes.

**Why `open` as a value is a separate question.** A name is a value, and a value can reach a relation parameter.
With `(copy open)`, a rule's `(?R ?x)` reads relation `open`. If every written `(open …)` fact is refused, that
relation holds nothing, so the parameter reads an empty relation and nothing is wrong. Refusing `open` as a value
as well would make `open` a reserved word in the plain sense: written only as the verdict. That is what *`open` is
reserved word* says read strictly. Keeping it as a value would make `open` a position word like `model`.

## What it changes

- **Refused at load, every command, one line per place** (`<file>:<line>:<col>: \`open\` is the verdict a rule's
  :assert states, and is not a relation: …`): `open` as the head of a fact, a nested fact, a stored negation, a
  pattern anywhere in a rule, and as an activator's argument bound to a relation parameter. Where the grammar can
  refuse it, the grammar does (`FactHead`, the `NestedFact` head, `ArgHead` lose `open`), with a row per place in
  `00_ebnf.md`'s table. The loader takes what only the program can tell (a relation parameter).
- **By Q-M1f.66**: `open` as a value is refused too (the `Name` production loses it outside `:assert`), or stays.
- **Register rows** for each witness that loads today and is refused after, filed before the fix
  ([Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row)).
- **Not here**: the other position words a fact head admits (`eq`, `absent`, `false`, `true`, `model`, `step`).
  `eq` is S1f.5.7's checked conclusion, and the rest are names by Q-M1e2.17. T1f.5.23.2 lists how each behaves as
  a fact head, in its record. If one turns out to be the same case, it is noted for the phase's close and not
  fixed here.

## Tasks

### Task T1f.5.23.1 — the explanation (done, [above](#t1-record-2026-11-10))

### Task T1f.5.23.2 — every place, banked

One witness per row of the table and per place Q-M1f.66 adds, under `examples/ein-bugs/wrong-answers/`, each run on
every command that reads a program. The register rows, filed `open`. Every golden and fixture the refusal moves,
named.

### Task T1f.5.23.3 — the refusals

The grammar's productions and the loader's check; the rows flipped; the `at None` line rewritten in Ein's words.

### Task T1f.5.23.4 — the pages

[`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) (where `open` may be written),
[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes), `00_ebnf.md` re-blessed, the stage's records.

## Acceptance

- `open` is accepted only as a rule's `(open)` or `(open ?R)` conclusion, and also as a value if Q-M1f.66 says so.
  Every other place is refused by every command with one line, held by the grammar's table and a test.
- Every witness that loaded before is a register row, `fixed`; no corpus answer moved.
- `./run_tests.sh` green.
