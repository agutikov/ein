# S1f.21.25 — an activator that names no relation activates nothing, in silence

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-24 on the user's ruling of review item 82
([the README](README.md#the-answers-to-part-1s-items-66-to-82)); **not started**, on the user's word.
**Writes:** `ein-ir/src/from_ir.rs`, `ein-infer/src/compile.rs` and its three callers, `ein-cli/src/common.rs`, their
tests; the decisions and pages that say a line is printed; the headers of the programs that show one.

## What it is

An activator that binds a rule's relation parameter to a fact, `(symmetric (H1 H2))` for a rule
`symmetric (?rel)` whose `?rel` heads a pattern, activates nothing: the engine compiles no plan for it. Today three
`warning:` lines say so:

| line | printed where | decided by |
|---|---|---|
| the load's, for one a rule *could* derive | read off the program's text, on every command | Q-M1f.1, with Q-M1f.41 and Q-M1f.42 |
| the written one's, for one the program writes | at load | Q-M1e2.18, its second half |
| the run's, for one a state holds | when a run ends, placed at the rule | Q-M1f.1 |

**The user ruled all three gone** (*"I think this warning is false by desing. We do use relations as arguments.
And there could be any number of not activated rules if program imports something but not use."*, and, asked back
of the other two, *"All three go (Recommended)"*). The first fires on valid programs: review item 82's
`zebra2-about-1.ein`, where it names `std.algebra.imply1`, which nothing activates; `imply1`'s own use toward a
rule, `(imply1 equivalence symmetric)`, which works; and item 81's programs, eighteen lines of rules with no plan.
The item's page and programs are in `git log`, last at `0f7190b9`.

So such an activator says nothing, as an activator of another count says nothing
([Q-M1f.115](../../../docs/decisions/language/Q-M1f.115-an-activator-is-a-fact-of-its-rules-name-and-one.md)).
**No answer and no exit code moves**: Q-M1f.1 already says none moves for a line.

## Tasks

1. **The lines go, the reading stays.**
   - `ein-ir/src/from_ir.rs`: `warn_derived_relation_activators` writes no warning. The positions it reads stay,
     for their two other readers: `refuse_eq_at_another_arity`
     ([Q-M1f.46](../../../docs/decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)),
     and `RelationPlaces`, which `relations_written` takes into the relations of the program
     ([Q-M1f.522](../../../docs/decisions/language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)).
     The function is renamed for what it does now. `validate_relation_parameters` writes no warning for a written
     activator; what else it does, if anything, stays.
   - `ein-infer/src/compile.rs`: `note_inert_activators` and `take_inert_activators` go, with their calls in
     `engine.rs`, `obligations.rs` and `hypothesis_rules.rs` and the printing in `ein-cli/src/common.rs`.
     `activators_for`, which hands no plan such an activator, stays.
   - The comments that cite a line: `ein-core`'s `entities.rs` and `program.rs`, `ein-infer`'s `plan.rs`.
2. **The tests hold the silence.** `ein-cli/tests/cli_semantics.rs`'
   `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load` and its siblings, the unit
   test of that name in `from_ir.rs`, and `ein-infer/tests/compile_semantics.rs`'
   `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`: each holds that no line is printed, on
   every command, and keeps what it holds of the engine (no plan, the answer). `solve_lists.rs` and
   `stdlib_semantics.rs` where they cite the lines.
3. **The decisions.** Q-M1f.1 rewritten under its id, as S1f.21.17 rewrote it: such an activator activates nothing
   and nothing says so. Q-M1e2.18's second half the same; its first, the keyword pair refused at the parse, stays.
   Q-M1f.41 and Q-M1f.42 reworded for what reads the positions now. Q-M1f.580: when lines print stands for every
   line; the words it set for these go. Where Q-M1f.115, Q-M1f.567, Q-M1f.555, Q-M1f.559, Q-M1e2.26 and Q-M1f.46
   cite the lines; the language decisions' README.
4. **The pages.** The diagnostics page's § 2.9, *a rule shape that reads nothing*, and its rows; `std.negation`'s
   *A program the import warns of*, which goes; and every page `git grep -e 'activates nothing' -e 'Q-M1f\.1\b'
   -e 'Q-M1e2\.18' -- docs` finds, read for a claim that a line is printed: `08-read-out.md`, `02-syntax.md`,
   `05-rules.md`, the reasoning pages `02-rewriting-rules.md` and `02-saturation.md`, and `08-reasoning-in-code/`'s
   `01-static-ir.md` and `02-saturation-and-negation.md`.
5. **The programs' words.** The headers of `examples/features/19_derived_activator.ein` and
   `20_derived_activator_never.ein`, which stay as fixtures of an activator that activates nothing, and of
   `examples/ein-bugs/wrong-answers/derived-activator.ein`, `derived-activator-never.ein`,
   `nested-fact-activator.ein` and `kwpair-in-guard.ein`; their lines in `examples/README.md`,
   `corpus/corpus.toml` and `corpus/regressions.toml`; the comments of `stdlib/negation.ein`,
   `stdlib/math/decimal-arith.ein` and `stdlib/math/positional.ein`, and `stdlib/README.md` (a stdlib file that
   changes takes `utils/stdlib_manifest.py --write`); `examples/broken/compile/activator_arity.expected` and
   `examples/broken/load/README.md`, read for which shape they are about.
6. **The check over the corpus**: no corpus program's stderr, under `ein solve`, `ein saturate` and `ein show hyps`,
   holds a line of the three.

## What must hold

- **No answer moves**: $H$, $R$ and every exit code on every corpus program, by the corpus sweeps as they are.
- **Fixtures 19 and 20 stay flagged** where they are: the static monotone check is `ein-infer`'s and reads
  activators itself, not the loader's positions.
- **Q-M1f.46's refusal and the relations of the program do not move**: they read the positions, which stay.
- The gate on the batch's last commit.

## Record

*Not started.*
