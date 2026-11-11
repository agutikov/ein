# S1f.7.12 — A macro call written as a fact

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). **Done 2026-11-11**: W86, eight rows filed and fixed; a
fact that is or holds a macro's call is refused at load, and a conclusion's call stays expanded
([the records](#t1-record-2026-11-11)).
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), which lists every place a macro call is read and
not expanded.

## The finding

Taken on the release binary at `4bf6da73`, 2026-11-11: with `(macro pp (?a) (p ?a))`, a top-level `(pp j)` loads,
exit 0. It is stored as a fact of a relation `pp`, and `(p j)` is not stored: it appears only as a candidate. The
author who wrote the call has neither the fact the macro spells nor an error.

## The direction

A macro's body is a pattern, read in a rule's `:match`. A fact is not a pattern.

- **(a) Refused at load** (the default): a fact, at any depth, or a conclusion whose head is a macro's name, the
  line naming the macro and saying a macro is called in a `:match` or a `:goal`.
- **(b) Expanded**: the call replaced by its body where the body is one ground form. This gives a macro a second
  meaning, and a body with a connective has none as a fact.

## Tasks

- **T1f.7.12.1, confirmation and the census.** Re-take it. Every site a macro's name can head a form that is not
  expanded: a fact, a nested fact, a rule's and an hrule's `:assert`, a claim's model, a `(solve …)` list, an
  import's `:symbols` name used as a relation. The stdlib's macros (`unknown`, `forall`) written as a fact.
- **T1f.7.12.2, the rows**, `open`, held to today's engine.
- **T1f.7.12.3, the fix**, the rows flipped, a test on every command.
- **T1f.7.12.4, the pages**: `01_grammar.md`'s text on macros, `defined_behaviour.md`, the records.

## Acceptance

- No command stores a fact whose relation is a macro's name.
- Every witness is a register row. No corpus answer moves but the witnesses'.
- `./run_tests.sh` green.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `91b12be8`. With `(macro pp (?a) (p ?a))`, `ein saturate --dump` on each
site a macro's name can head a form:

| the program writes | at `91b12be8` |
|---|---|
| `(pp j)`, a fact | stored, `(pp j)`, a fact of an auto-vivified relation `pp`; `(p j)` is not stored |
| `(pp j k)`, another arity | stored |
| `(q (pp j))`, `(not (pp j))` | stored as written |
| `(q j :rule r :using ((pp j)))` | loads |
| `(unknown (p j))`, `std.macro`'s `unknown` imported | stored, a fact of `unknown` |
| `(std.macro.unknown (p j))` after `(import std.macro)` | stored |
| `(unknown (p j))`, no import | stored, where a rule's `:match` writing it is refused (S1.8a.f20) |
| `(p (forall (p j) (p k)))`, `forall` imported | stored |
| `(relation pp T)` beside the macro, and `(pp j)` | both load; the fact is of the declared relation |
| `(rule pp (?z) …)` beside the macro, and `(pp j)` | loads, the fact the rule's activator |
| `:assert (pp ?x)` in a rule | **expanded**: `(p k :rule r)` is stored |
| `:assert (s (pp ?x))` | **expanded**: `(s (p k) :rule r)` |
| `:assert (pp ?x)` in an hrule | expanded, the candidate is `(p k)` |
| `:match (pp ?x)`, `:match (q (pp ?x))`, `:goal (pp ?x)` | expanded (the goal since S1f.7.5) |
| `:expect (model (pp j))`, a query's | refused, *:expect names pp, and the program names no relation pp* (Q-M1f.118) |
| `:expect (model (q (pp j)))` beside `(q (p j))` | loads; the claim is FAILED, exit 1, *the claim lists (model (q (pp j))), which is no class* |
| `(solve :expect (region (model (pp j))))` | refused, *names pp, and the program names no relation pp* |
| `(solve :expect (region (model (q (pp j)))))` | loads; the claim is FAILED, exit 1 |
| `(solve :no-hypothesis (pp))`, `:hypothesis-relations (pp)` | refused since S1f.7.8, *the program names no relation pp* |
| `(q pp)`, the name as a value | stored: a name |

**A conclusion's call is expanded today.** The stage's plan listed a rule's and an hrule's `:assert` among the
sites that are not expanded. They are: `from_ir`'s `expand_pair` runs the expander over both clauses. So the class
is a fact, at any depth, and a fact a claim lists; what a conclusion's call does is
[Q-M1f.130](../open_questions.md#q-m1f130--a-macros-call-as-a-conclusion-refused-or-expanded-as-it-is), defaulted
to *as today*.

**The census.** No tracked `.ein` file writes a fact headed by a macro's name, a relation declared under one, or a
claim or `(solve …)` list holding one (a scan of the 651 files' top-level forms against each file's `(macro …)`
names and `std.macro`'s two, 2026-11-11; the engine's sweep in the T3 record agrees). Ten witnesses call a macro in
an `:assert`: eight `macro-*.ein`, `not-name-macro.ein` and `open-macro-verdict.ein` under
`examples/ein-bugs/wrong-answers/`. No program has a relation and a macro of one name.

## T2 record, 2026-11-11

Eight witnesses under `examples/ein-bugs/wrong-answers/`, each with a corpus entry (`solve`, `saturate`), case
**W86**:

| row | the witness writes | at `91b12be8` |
|---|---|---|
| `W86/macro-call-fact` | `(settled Ann)` beside `(macro settled (?g) (paid ?g))` and a rule over `(settled ?g)` | `(settled Ann)` stored, `(paid Ann)` not; the rule derives nothing |
| `W86/macro-call-nested-fact` | `(says (settled Ann))` and a rule over `(says (settled ?g))` | stored as written; the rule's expanded pattern does not match it |
| `W86/macro-call-negated-fact` | `(not (settled Bob))` beside `(paid Bob)` | root alive, where `(not (paid Bob))` kills it |
| `W86/macro-call-stdlib-fact` | `(unknown (paid Bob))`, `unknown` imported | stored |
| `W86/macro-call-stdlib-unimported` | the same, no import | stored |
| `W86/macro-call-in-claim` | `:expect (model (says (settled Ann)))` | loads; `ein solve` exits 1, the claim FAILED |
| `W86/macro-call-in-region-claim` | `(solve … :expect (region (model (says (settled Ann)))))` | loads; `ein solve` exits 1, the claim FAILED |
| `W86/macro-and-relation-one-name` | `(relation settled Guest)` beside the macro, and `(settled Ann)` | loads; the fact is stored and no premise reads it |

Each was filed `open`, its `now` `ein saturate --dump`, a command's run, which answers, and its `right` a refusal.
With the three S1f.7.9 rows beside them the register read 293 fixed, 24 open and 1 not wrong, and `ein-corpus`'s
tests, `ein-cli`'s `wrong_answers.rs` and `ein-infer`'s `register_region.rs` were green on the engine sources at
`91b12be8` (2026-11-11).

**The goldens the fix moves**, named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt` (the eight
witnesses' two runs each, exit 1) and `ein-render/tests/golden/corpus_shapes.md5` (the eight witnesses' lines). No
other golden, and no line of an existing entry.

## T3 record, 2026-11-11

**The refusal** is the loader's, `ein_ir::from_ir`'s `refuse_macro_facts`, after the facts are ingested, so every
command that loads a program refuses it with one line and the load error's exit:

```
kb load error: examples/ein-bugs/wrong-answers/macro-call-fact.ein:29:2: the fact (settled Ann) is a call of the macro settled, (macro settled (?g) (paid ?g)) — a macro is called in a :match, an :assert or a :goal, where its call is replaced by its body; a fact is not expanded, and no fact is of a macro: write the fact itself
kb load error: examples/ein-bugs/wrong-answers/macro-call-nested-fact.ein:29:8: the fact (says (settled Ann)) holds (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — …
kb load error: examples/ein-bugs/wrong-answers/macro-call-stdlib-unimported.ein:25:2: the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the program does not import — …
kb load error: examples/ein-bugs/wrong-answers/macro-call-in-claim.ein:29:46: query: its :expect lists (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — a claim lists facts, and a macro is called in …
kb load error: examples/ein-bugs/wrong-answers/macro-and-relation-one-name.ein:26:11: relation 'settled': settled is a macro too, (macro settled (?g) (paid ?g)) — a premise (settled …) is the macro's call, replaced by its body, and a fact (settled …) is refused, so nothing could be of the relation; give one of the two another name
```

It reads every written fact, at any depth and through a `:using`, for a form whose head is a macro's name: one the
program declares or imports, flat or qualified, or one of `std.macro`'s that it does not import
([Q-M1f.132](../open_questions.md#q-m1f132--a-stdmacro-macro-the-program-does-not-import-written-as-a-fact)). A
`(relation …)` declared under a declared macro's name is refused with its own line
([Q-M1f.131](../open_questions.md#q-m1f131--a-relation-declared-under-a-macros-name)). `refuse_macro_claims` reads
the facts a query's and a `(solve …)`'s `:expect` list, and adds its line only to a program nothing else refused, so
a claim headed by the call keeps Q-M1f.118's line
([Q-M1f.133](../open_questions.md#q-m1f133--a-macros-call-inside-a-fact-a-claim-lists-and-in-a-facts-using)). The
position is the call's name: the parser keeps none for a form.

**Not expanded** (the stage's default (a)). **A conclusion's call stays expanded** (Q-M1f.130).

**The sweep.** Every tracked `.ein` file, 651, under `ein saturate --dump` and `ein solve` (capped at 60 s and
30 s), on the release binary at `91b12be8` and on the stage's: no file's output differs, every zebra program among
them (`diff -rq`, 2026-11-11, before the witnesses were written).

**The rows** are `fixed`, each `now` the refusal's line: 301 fixed, 16 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11), the three more open rows S1f.7.9's.

**The pin**, `ein-cli/tests/macro_facts.rs`: fifteen refused shapes and the eight witnesses, each on twenty
commands, one first line of stderr, the same on each, and no container written; and seven programs that keep
loading, a call in a `:match`, inside a pattern, in a rule's `:assert`, inside a conclusion, in an hrule's
`:assert` and a goal, the name as a value, and a macro nothing calls, each storing what the call's body spells and
no fact of the macro's name. Every refusal fails against the binary at `91b12be8`. `ein-ir`'s unit test
`a_macros_call_written_as_a_fact_is_refused_at_load` holds the lines at the loader.

**Found outside the stage**: a rule or an hrule named as a macro, and a relation `unknown` declared without the
import, each loading without a word:
[F25](../../followups/f25_roughness_found_outside_zebra.md)'s R-8. Nothing moves a zebra program.

## T4 record, 2026-11-11

[`defined_behaviour.md` § 4.24](../../../docs/kernel/defined_behaviour.md#424-a-macros-call-is-no-fact--m1f-s1f712)
is new: where a macro is called, the six lines, what is not expanded and what stays.
[`01_grammar.md` § Facts](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) says a fact is not a macro's call,
`06_reserved_names.md`'s `macro` row the same, and `examples/README.md` names the witnesses. `ein_ir::macros`'
comments on what is not expanded say the loader refuses the fact. `00_ebnf.md`'s table is unmoved: the grammar
takes the fact, and the loader refuses it.

## Acceptance re-taken, 2026-11-11

- *No command stores a fact whose relation is a macro's name*: `macro_facts.rs`, twenty commands on each refused
  shape, and no `(settled …)` line in any accepted program's `saturate --dump`.
- *Every witness is a register row; no corpus answer moves but the witnesses'*: W86's eight; the sweep; the two
  goldens gained the witnesses' lines alone.
- *`./run_tests.sh` green*: on the stage's commit.
