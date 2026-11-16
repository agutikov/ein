# Transformations of a rule set

**The question this page is about:** *which transformations of a program's rule set leave its answer, the region R,
unchanged, and which of them does Ein perform?* A rule set is an object that can be rewritten: a rule split in two,
two rules fused, a macro written out, an import made whole, the rules written in another order. Each rewrite either
keeps R or changes it, and an engine that rewrites a rule set for speed may use only the first kind.

What follows is what holds today. The rules of a program are fixed when it is loaded
([Rules as data](rules-as-data.md#4-where-it-stops-a-rule-is-not-data)): every transformation below happens at load,
none during saturation or search, and Ein performs no other.

## What Ein does to a rule set at load

- **An import is resolved.** Every import brings its module whole, with the module's own imports, one copy per
  program, each declared name under its full name; the forms of an import differ only in which names the file may
  write short. The answer does not depend on the form
  ([Q-M1f.249](../../../decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).
- **A macro's call is expanded**, in a pattern, a conclusion and a goal alike, and its body is checked where it is
  called ([Q-M1f.130](../../../decisions/language/Q-M1f.130-a-macros-call-in-a-conclusion-is-expanded.md),
  [Q-M1f.67](../../../decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)). `forall` and
  `unknown` are such macros, so a program that writes them is rewritten into `absent`s before any rule runs.
- **A conclusion's `(and …)` is read flat**, at any depth: a rule concludes a conjunction of literals, and nesting
  adds nothing.
- **A rule with parameters is instantiated once per activator**, the relation names an activating fact supplies
  written into the rule before it matches anything; an activator derived later, at root or in a node, gets its
  instance when it arrives ([Q29](../../../decisions/engine/Q29-a-rule-compiles-once-per-activator.md)).

## What leaves R unchanged

| transformation | R |
|---|---|
| an import written selective, whole, or under an alias | unchanged: every state, both counts, root's verdict and debts, every query's classes and every claim's outcome, up to the names' prefixes |
| the order a monotone program's facts and rules are written in | unchanged: a monotone program's fixpoint does not depend on the order its rules fire in, and the two orders of `send_more_money.ein`'s eight letter facts are held to one answer and one amount of work |
| the agenda's order against the written order alone, on a monotone program | unchanged, for every question and every claim; only a dead node's core, an explanation, may differ ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)) |

Of a program the monotonicity check flags, Ein claims no answer under any transformation it does not hold by a test
([Monotone and non-monotone programs](../kb-evolution/monotonicity.md)).

Every other rewrite of a rule set, splitting, fusing, specialising or generalising rules, is not one Ein performs,
and what it does to R is a question of the rule set it is applied to.

## What holds it

| claim | test |
|---|---|
| every corpus program answers alike under every import form | `ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form` |
| `send_more_money_written.ein` is `send_more_money.ein` with one line of eight facts reordered, and `--single` and the maximal line do the same work and list the same states on both | `ein-cli/tests/cli_semantics.rs`: `the_written_send_is_the_by_column_send_with_one_line_reordered`; `ein-infer/tests/order.rs` |
| the agenda and the written order alone answer every question and every claim alike on every monotone program | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a rule with parameters gets an instance for an activator derived at run time, on a fork | `ein-infer/tests/explain_semantics.rs`: `a_runtime_derived_activator_gets_a_plan_on_a_fork` |
| a macro's body may call another macro, and a parameter in head position is substituted | `ein-ir/tests/ir_semantics.rs`: `a_macro_body_may_invoke_another_macro`, `a_macro_parameter_in_head_position_is_substituted` |
