# S1f.5.9 — An empty `:match` fires

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.9**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1 day.
- A quarter of a day to re-take the rows [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed, list every move and show the programs.
- Half a day for rules and hrules.
- A quarter of a day for the refusal's line and the pages.

**Status:** specified 2026-11-07 on the user's rulings of that day on
[F12](README.md#f12-an-empty-match-never-fires).
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
**Depends on:** nothing. [S1f.5.5](s1f.5.5_the_grammar.md)'s grammar carries the `(and)` refusal once it
lands, and [S1f.5.8](s1f.5.8_no_contradiction_no_core.md)'s test meets the first empty core this stage
makes.

## The ruling

The agent put three questions on F12. The user, 2026-11-07: *"1 fire; 2 same; 3 reject (and) while it is the
same as () and `and` should have arguments"*.

1. **An empty `:match` fires.** An empty conjunction is true, so a rule with no premise fires once, or once
   per activator fact when it has parameters, and concludes what it asserts.
2. **An hrule's empty `:match` fires the same way**, and proposes its conclusion.
3. **`(and)` stays refused.** It means what `()` means, and `and` takes at least one conjunct, so the empty
   conjunction is written `()` alone.

A fourth question, a conclusion's variable that nothing binds, was filed as F13 and ruled the same day: refused
at load, by [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md), which lands first.

## The case

Taken on 2026-11-07 at `916d97ac`, with `ein saturate FILE --dump` and `ein solve FILE`, over
`(relation q Node) (relation p Node) (q a)` or `(p a)`. No corpus program writes an empty `:match` (`grep`
over `examples/`, `tests/`, `stdlib/` and `corpus/`).

| the rule | today | right |
|---|---|---|
| `(rule r () :match () :assert (false))` | loads, never fires: ∅ in R | root is dead, so R is empty. Its core is `[]`, the empty set ([Q-M1e2.27](../../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)), and `refuted by r` |
| `(rule r () :match () :assert (q b))` | loads, never fires: no `(q b)` | root holds `(q b :rule r)` |
| `(rule r (?x) :match () :assert (hit ?x))` with `(r a)` | loads, never fires: no `(hit a)` | root holds `(hit a :rule r)`, one firing per activator |
| `(hrule h () :match () :assert (q b))` with `(solve :hrules (h))` | proposes nothing: *no candidate* | proposes `(q b)`, as `:match (p ?x)` does: one candidate |
| `(rule r () :match (and) :assert (false))` | refused at parse: *unexpected input* | refused, with a line that says `and` takes a conjunct and the empty conjunction is `()` |

The first four are wrong answers, a shape that reads nothing in silence, as W24's were. [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed them
on 2026-11-08 as W31, `open` and owned by this stage, each witness under `examples/ein-bugs/wrong-answers/`
and a corpus entry, and step 2 fixes them.

## What it changes

- **Rules.** A rule whose `:match` is `()` compiles to a plan with no premise. It fires once at root, or once
  per activator fact, with only its parameters bound.
- **Hrules.** The same, so the hrule rung proposes the conclusion once, or once per activator.
- **`(and)`.** Refused at load on today's parser, in Ein's spelling, naming the rule and the clause. On
  S1f.5.5's grammar, `and` takes one conjunct or more, and the parse error says so.
- **What it reaches.** A contradiction derived this way has an empty core, the first `[]` core in Ein. A
  derivation with no premise is the shape of W25, which F22 owns: the store does not record one as an
  alternative justification. This stage makes it reachable on a rule written for it, and does not change it.
- **What does not move.** No corpus program writes it, so no corpus answer moves.

## The method

### Task T1f.5.9.1 — the rows, every move listed

It starts from W31's four rows, which S1f.5.0 filed `open` on 2026-11-08, each `now` taken at `ec004a48`
(the engine `916d97ac`'s), release and debug alike, every one the case's answer above:

| row | witness | `now`, today |
|---|---|---|
| `W31/empty-match-false` | `empty-match-false.ein` | `solve --minimal`: root's state, ∅ in R |
| `W31/empty-match-fact` | `empty-match-fact.ein` | `solve --minimal`: `(model (q a))`, no `(q b)` |
| `W31/empty-match-activator` | `empty-match-activator.ein` | `solve --minimal`: no `(hit a)` |
| `W31/empty-match-hrule` | `empty-match-hrule.ein` | `solve`: no candidate, R {∅} |

Each re-taken, with the `(and)` refusal's probe. `ein-infer/tests/alive_invariant.rs`' `SEEDED` names `b` on
`empty-match-fact` and `empty-match-hrule`, a name only the conclusion writes; step 2 says whether the fix
moves either line. The corpus swept, and
`cargo test --workspace` run on a prototype. Every move must be one of the case's rows. **Stop and show the
user the programs**, answers computed.

### Task T1f.5.9.2 — the empty conjunction fires

The compiler and the hrule rung, and the case's rows fixed. `ein-infer/tests/explore.rs` and the oracle
agree on each witness. `ein-infer/tests/corpus_cores.rs` holds the dead witness's core, `[]`, and its rule.

### Task T1f.5.9.3 — the refusal and the pages

The `(and)` line. `01_grammar.md`'s rule section: an empty `:match` is the empty conjunction, true, written
`()`, and `and` takes a conjunct. `02_patterns.md` likewise. The inventory's F12 says what fixed it.

## Acceptance

- A rule or hrule whose `:match` is `()` fires once, or once per activator fact, on every command.
- `(rule r () :match () :assert (false))` makes root dead, with the core `[]` and `refuted by r`.
- `(and)` is refused at load with a line naming the rule, and `()` is the one spelling of the empty
  conjunction.
- The case's rows are `fixed`, and no corpus answer moves.
- `./run_tests.sh` is green.
