# S1f.5.18 — An activator's nested fact: a value parameter binds it, and a relation parameter is refused at load

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Estimate:** 1.5 days.
- Half a day for W33: the parameter binds, with the corpus swept for every move.
- Three quarters of a day for the load-time check that G-14's ruling adds, prototyped and swept first.
- A quarter of a day for the pages and the record.

**Status:** specified 2026-11-08 on the user's two rulings of that day:
[Q-M1f.3](../open_questions.md#q-m1f3--three-wrong-answers-the-census-found-with-no-row) (file W33 and give it this
stage) and [Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b), whose
load-time check is the half of G-14 no grammar can see. Not started.
**Depends on:** [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md), which files W33 as an `open` row first.
It is independent of the grammar stages. It touches `ein-infer/src/compile.rs`, as
[S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) and [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md)
do, so it runs after them and before [S1f.5.9](s1f.5.9_an_empty_match_fires.md).

## The case

### W33: a value parameter bound to a nested fact stays unbound

Found 2026-11-08 by [S1f.5.5's census](s1f.5.5_census.md#findings-outside-the-grammar), on `ec004a48`'s
release binary:

```lisp
(relation p T) (relation hit T)
(p a) (p b) (p (f a))
(rule act (?y) :match (p ?y) :assert (hit ?y))
(act (f a))
```

It derives `(hit a)`, `(hit b)` and `(hit (f a))`. It should derive `(hit (f a))` only. When a plan is
compiled, `ein_infer::compile` seeds the rule's parameters from the activator's arguments, and skips an argument
that is a nested fact (`compile.rs`: *"A nested Fact does not bind"*). The parameter is then a free variable of
the plan, and `(p ?y)` matches every `p`. W24 (M1e2 S1e2.2.24) made that skip a refusal where the parameter
names a relation, through `fact_bound`. Where the parameter is a value, the skip is a wrong answer in silence.

A nested fact is a value: a fact can hold one as an argument, a pattern can match one, and the printer
prints one. So a parameter binds to it as it binds to a name.

### G-14's derived activator: a relation parameter a rule can bind to a fact

[S1f.5.5's paper grammar, Q1](s1f.5.5_paper_grammar.md#q1--g-14-with-assert-checked-by-the-grammar-is-a-bad-conclusion-that-never-fires-refused)
showed the one shape of G-14 a grammar cannot see. An activator that a rule **derives** binds a relation
parameter to a fact:

| program | today, both parsers |
|---|---|
| [`g14/derived-activator.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator.ein): `(rule r (?R) :match (box (?R b)) …)`, `(rule mk () :match (item ?f) :assert (r ?f))`, `(item (a b))` | the run stops when `(r (a b))`'s plan is compiled: *the relation head ?R of (?R b) … is bound to the fact (a b)*, exit 1 |
| [`g14/derived-activator-never.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator-never.ein): the same without `(item (a b))` | answers, 64 states, since no node derives the activator |

**The user ruled (b), 2026-11-08** (*"Q1 - b"*). Both programs are refused at load, by a static check that
some rule could derive an activator binding a relation parameter to a fact. The second program is refused
too, although no node derives the activator. The user chose (b) over (a) knowing it.

## What it changes

- **W33: the parameter binds.** A value parameter whose activator argument is a nested fact is seeded with
  that value, as a name is, and the plan matches only facts holding it. The compile key is already the
  activator's values (W16, S1e2.2.16), so two activators that differ only in a nested fact compile two
  plans. Every analysis that reads activators gets the bound value: `forbid`, `monotone`, the static check
  and `invariant::program_names`. The stage lists any that changes its answer before it lands.
- **The relation half stays refused.** A relation parameter bound to a nested fact is refused where the plan
  is compiled, as W24 made it. Since that refusal can now come only from a derived activator, the run-time
  line remains as the backstop behind the load-time check below.
- **The load-time check (Q-M1f.1 (b)).** At load, the program is refused when some rule's `:assert` writes an
  activator of a rule, in a relation parameter's position, with a term that could be a fact. Such a term is
  either a nested fact written there, or a variable whose premise binds it at a position that can hold a
  fact. One line per site, in Ein's spelling, naming the deriving rule, the activated rule and the parameter.
  Every command that loads exits 1, and `ein test` exits 2.

  **Defining *can hold a fact* is the stage's design work, and two constraints bound it.** A precise flow
  analysis would accept `derived-activator-never.ein`, since no fact or candidate puts a fact in `item`'s
  argument, but the ruling refuses it. A blanket reading, where any premise variable counts, would refuse
  zebra2's fan-out rules, which pass relation names on through `(co-located ?R1 ?V1 ?R2 ?V2)`. The check
  sits between the two: a position whose values are relation names is one that the program uses as a
  relation (a relation parameter, a `(?R …)` head, or a relation signature's name), and everything else
  can hold a fact. Step 1 prototypes it and sweeps the corpus. **If it refuses a corpus program that
  answers today, or no reading both refuses the two `g14` programs and keeps the corpus, the stage stops and
  shows the user** the programs and the readings, before any edit.

## The method

### Task T1f.5.18.1 — every move listed

W33's row re-taken on a fresh build, starting from S1f.5.0's row. Both changes prototyped. The corpus and
`corpus/fuzz_findings/` swept with every command that loads, comparing each run against today's. Every move
is listed: W33's witness turning right, and any program the check refuses, with its deriving rule.

### Task T1f.5.18.2 — W33 fixed

The seed, W33's row `fixed` in the same commit, and a test in `ein-infer` beside W24's that holds a value
parameter bound to a nested fact, in a single activator and in a derived one. `./run_tests.sh` green.

### Task T1f.5.18.3 — the load-time check

The check, its line, and the two `g14` programs banked as refusal fixtures under `examples/broken/load/` with
their `.expected`. A test holds each shape: a written nested fact, a variable from a value position, and the
zebra2 shape, which must load. `./run_tests.sh` green.

### Task T1f.5.18.4 — the pages

`defined_behaviour.md` § 4.9 (W24's rule shapes) gains the load-time line, `01_grammar.md`'s activator section
says that a parameter binds any value, nested facts included, and the record: this file's Status, the phase
README's row, Q-M1f.1 and Q-M1f.3 marked built, and a sentence in `AGENTS.md`.

## Acceptance

- `activator_nested_value`'s witness derives `(hit (f a))` only, and W33's row is `fixed`.
- `g14/derived-activator.ein` and `g14/derived-activator-never.ein` are refused at load, one line each.
- No corpus program that answers today is refused, unless the user has seen it and agreed.
- `./run_tests.sh` is green.
