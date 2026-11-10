# S1f.7.8 — A `(solve …)` list that names nothing the program declares

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 3), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
**Done 2026-11-11**: the records are below ([T1](#t1-record-2026-11-11), [T2](#t2-record-2026-11-11),
[T3](#t3-record-2026-11-11), [T4](#t4-record-2026-11-11)). W76, five rows, filed and fixed; Q-M1f.114 to .116
defaulted; three rows of F25.
**Depends on:** nothing.

## The finding

Recorded on the release binary at `f62e34e1`:

> **A `(solve …)` list naming nothing the program declares loads in silence.** `(solve :hrules (zzz))` beside a
> declared hrule `h` answers as the program without the form (two states), `(solve :hypothesis-relations (zzz))`
> leaves H empty and `:no-hypothesis (zzz)` restricts nothing, each with no word that the name matches no hrule or
> relation; a query's `:over zzz` and a claim's undeclared relation are refused at load.

So a misspelt name changes the answer in silence (H empty, or no restriction), while the same mistake in a query
is refused.

## The direction the rulings point to

A query's `:over` and a claim's relation are refused at load when they name nothing. The direction that agrees is
the same refusal for every name a `(solve …)` list holds: one line in S1f.5.21's spelling naming the form, the list
and the name. That is the default, since it moves no program that names what it declares.

## Tasks

- **T1f.7.8.1, confirmation and the census.** Re-take the finding. List every clause of `(solve …)`, `(query …)` and
  `(config …)` that names a relation, an hrule, a rule or a type, and what each does with a name the program does
  not declare, on every command.
- **T1f.7.8.2, the rows.** One witness per list whose answer moves. Each is filed `open` and held to today's engine.
- **T1f.7.8.3, the refusal.** At load, with the rows flipped and a test on every command.
- **T1f.7.8.4, the pages.** `01_grammar.md` § Solve, `defined_behaviour.md`, and the stage's records.

## Acceptance

- Every name a `(solve …)` list holds names something the program declares, or the program is refused on every
  command with one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.8 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `63f1e569`. On a program with one hrule, `guess (?R ?T1 ?T2)`, a declared
relation `at`, `(functional at)` and one colour and three houses:

| the `(solve …)` writes | H | R |
|---|---|---|
| `:hrules (guess (at Color House))` | 3 candidates | 4 states, `functional at` killing every pair |
| `:hrules (gues (at Color House))` | none | {∅} |
| `:hrules (guess (att Color House))` | 3 candidates of `att` | 8 states |
| `:hrules (guess (at Color))` | none | {∅} |
| `:hrules (guess (at Colour House))` | none | {∅} |
| `:hrules (guess zzz (at Color House))` | 3 candidates | 4 states: `zzz` is read as nothing |

and on a program with a declared `likes`, a derived `glad`, one person and one colour: `:hypothesis-relations
(like)` leaves H empty where `(likes)` gives two candidates, and `:no-hypothesis (like is-a)` leaves the two
candidates `(likes is-a)` excludes. Each loads with no word, on every command (exit 0 on sixteen runs of
`solve-hrules-no-such-relation.ein`: `saturate`, `saturate --dump`, six `solve` questions, `oracle`, five `render`
views and `kb save`), and a container saved from one answers as the text does.

**The census: every clause that names something of the program.**

| clause | names | a name the program does not have, before the stage |
|---|---|---|
| `(solve :hrules (h …))`, the first name and a bare name after it | an hrule | loads; a parameterised hrule gets no activator, so it proposes nothing; a bare name after the first is read by nothing |
| `(solve :hrules (h (a b)))`, an activator | its hrule's arguments | loads; another arity than the hrule's parameters is skipped; a relation the program does not have is proposed |
| `(solve :hypothesis-relations (r))` | relations | loads; a whitelist that holds no relation of the program, H empty |
| `(solve :no-hypothesis (r))` | relations | loads; excludes nothing |
| `(solve :expect (region (model (r k))))` | a claim's relations | refused at load, *names r, which no declaration or fact makes a relation* |
| `(query :over w)` | `all`, `minimal` or `maximal` | refused at load; it names no relation, so it is not the analogue |
| `(query :expect (model (r k)))` | a claim's relations | refused at load, as the solve form's |
| `(query :goal (r ?x))` | relations | loads, no row: [S1f.7.11](s1f.7.11_a_goal_over_a_relation_nothing_makes.md)'s |
| `(config :flag v)` | one of two flags | a parse error; it names nothing of the program |

So the stage's class is the first four rows. Three things the stage file did not say:

- **`:hrules` selects no hrule.** `Hrules::new` compiles every hrule without parameters whether the list names it
  or not, and reads the list only for activators. So a bare name does nothing, right or wrong
  ([F25](../../followups/f25_roughness_found_outside_zebra.md)'s R-3).
- **An activator makes its own relation.** `guess` concludes `(?R ?a ?b)`, so `(att Color House)` is a program
  that proposes facts of `att`, and *nothing makes it* does not describe the mistake. What does is that the program
  writes `att` nowhere else ([Q-M1f.114](../open_questions.md#q-m1f114--what-a-relation-in-a-solve--list-must-be)).
- **The claim check is not the model.** It asks for a declaration or a fact, and refuses a relation only a rule
  derives. The lists accept one.

## T2 record, 2026-11-11

Five witnesses under `examples/ein-bugs/wrong-answers/`, one per way the engine answered, each a corpus entry
(`solve`, `saturate`) and a register row, W76, class `refusal`:

| row | witness | answered at `63f1e569` |
|---|---|---|
| `W76/hrules-name` | `solve-hrules-no-such-hrule.ein` | H empty, R {∅} |
| `W76/hrules-activator-relation` | `solve-hrules-no-such-relation.ein` | H three candidates of `att`, R 8 states |
| `W76/hrules-activator-arity` | `solve-hrules-activator-arity.ein` | H empty, R {∅} |
| `W76/hypothesis-relations` | `solve-hypothesis-relations-no-such-relation.ein` | H empty, R {∅} |
| `W76/no-hypothesis` | `solve-no-hypothesis-no-such-relation.ein` | H two candidates of `likes`, R 4 states |

Each was filed `open`, its `now` the run `saturate --dump` answering (S1f.5.23's form for a row whose right answer
is a refusal), and held to the unchanged engine: `cargo test -p ein-corpus` and `cargo test -p ein-cli --test
wrong_answers` green with the loader's sources as at `63f1e569` and the five rows open (2026-11-11). The register's
schema is unmoved, `/21`.

**The goldens the fix moves**, named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt` (the five
witnesses' two runs each, exit 1, and `examples/syntax/query-full.ein`'s `render rules`, which exited 1 with *no
rule forms* and draws its two hrules since), and `ein-render/tests/golden/corpus_shapes.md5` (the five witnesses'
parse digests and `query-full`'s). The bless moved those two and no other, `corpus_shapes.md5` on those six files alone. One list in a test moved with the fixture: `ein-infer/tests/alive_invariant.rs`'s `SEEDED` gains
`query-full`'s `House`, `Water` and `Tea`, constants of the two hrules it now declares.

## T3 record, 2026-11-11

**The refusal** is the loader's, `ein_ir::from_ir`'s `refuse_solve_names`, after `validate_solve`, so every command
that reads a program makes it, a container's stored program included. One line per name, placed at the name, in
S1f.5.21's shape, `<file>:<line>:<col>: solve: its <list> …`:

```text
kb load error: t.ein:37:17: solve: its :hrules names gues, and the program declares no hrule gues — it declares guess
kb load error: t.ein:36:24: solve: its :hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3, (?R ?T1 ?T2) — an activator binds each parameter once
kb load error: t.ein:38:24: solve: its :hrules gives hrule 'guess' (att Color House), whose att is its parameter ?R, which names a relation, and the program names no relation att — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal
kb load error: t.ein:31:31: solve: its :hypothesis-relations names like, and the program names no relation like — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal
kb load error: t.ein:31:24: solve: its :no-hypothesis names like, and the program names no relation like — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal
```

**What a relation of the program is** (Q-M1f.114 (b)): a name the program writes as a relation outside the three
lists. The check collects the declared and fact-made relations, `is-a`, `eq` and `neq`, the relation of every form
at any depth in a rule's, an hrule's or an obligation's `:match` and `:assert` as expanded, in a query's `:goal`
and in a fact, and every name a fact or a conclusion writes at a position that holds relation names. That last set
is S1f.5.18's: `refuse_derived_relation_activators` now returns its positions and the names written at them
(`RelationPlaces`), and the same positions say which of an hrule's parameters names a relation. An imported
rule's conclusion counts under the name the import gives it.

**What it does not do.** A program the loader already refused gets no line from it, since a refused hrule is
missing from the registry and the line would say *declares no hrule*; eight register witnesses of earlier stages
showed that on the first sweep, and their recorded lines are unmoved. `()` names nothing and is accepted. An
activator's argument at a parameter that names no relation is not checked
([Q-M1f.116](../open_questions.md#q-m1f116--an-activators-argument-that-is-a-value), F25's R-2).

**The sweep.** Every tracked `.ein` file, 612, under `ein saturate` on the binary at `63f1e569` and on the stage's
(2026-11-11): the exit and stderr differ on one file, `examples/syntax/query-full.ein`, which named `h1` and `h2`
and declared neither. It declares both now, two hrules without parameters, and keeps its purpose, a `(solve
:hrules …)` beside a query carrying `:goal` and `:goal-text`; its answer is R {∅} before and after, there being no
house. No zebra program's load, exit or stderr moved.

One page moved with it: the excerpt of `zebra2.ein` in `docs/guide/04_solving_the_whole_puzzle.md`, which
`guide_programs.rs` loads with its elided forms left out, named `drink-loc`, `smoke-loc` and `pet-loc` in its
`:hrules` and declared them only in a comment. The block writes the three declarations out, as `zebra2.ein` has
them.

**The rows** are `fixed` by `c78e6430`, each `now` the refusal's line: 254 fixed, 13 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11).

**The pin**, `ein-cli/tests/solve_lists.rs`: nine shapes of a list naming nothing and the five witnesses, each on
twenty commands, with one first line, the same on each, and no container written; and fourteen programs whose
lists name what the program has, each loading on five commands: a declared relation, one only passed to a rule,
one only a rule's or an hrule's `:assert` derives, one only a goal reads, an imported rule's, `is-a`, `eq`, the
empty lists, the hrule's name written again, and a value that is no relation. With `refuse_solve_names`'s call
taken out the first two tests fail.

**Found outside the stage**, each a row of [F25](../../followups/f25_roughness_found_outside_zebra.md): R-2, an
`:hrules` entry that proposes nothing at root says nothing; R-3, a bare name in `:hrules` selects nothing; R-4,
`ein render rules` exits 1 on a program whose every rule is imported. `hypgen`'s comment that an empty
`:hypothesis-relations` list is unrestricted was wrong, `()` whitelisting nothing, and is corrected in place.

## T4 record, 2026-11-11

- [`defined_behaviour.md` § 4.20](../../../docs/kernel/defined_behaviour.md#420-a-solve--list-names-what-the-program-has--m1f-s1f78),
  new: the four lines, what a relation of the program is, what is not refused, the pin.
- [`01_grammar.md` § Solve](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#solve): what each list's names must
  be.
- `06_reserved_names.md`'s `hypothesis-relations` row.
- `examples/README.md`'s count of witnesses, `query-full.ein`'s header, and the guide's chapter 4 block (T3).
- `00_ebnf.md` is unmoved: the grammar did not change, and the refusal is the loader's.

## Acceptance re-taken, 2026-11-11

- *Every name a `(solve …)` list holds names something the program declares, or the program is refused on every
  command with one line*: `solve_lists.rs`, with what *declares* means for a relation in Q-M1f.114 and the one
  kind of name left unchecked, an activator's value, in Q-M1f.116.
- *Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves*: the
  T2 and T3 records; one fixture rewritten, named above.
- *`./run_tests.sh` green*: exit 0 on the stage's tree (`build/gate-s1f.7.8.log`, 2026-11-11).
