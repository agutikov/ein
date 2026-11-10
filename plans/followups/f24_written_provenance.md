# F24 — Written provenance: what a fact's `:rule` and `:using` may claim

**Filed 2026-11-11**, on the user's ruling of that day (*"yes S1f.7.9 moves to a new F24"*), from M1f
[S1f.7.9](../m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.9_a_facts_rule_that_names_no_rule.md)
and the [analysis of 2026-11-11](../m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-analysis-of-2026-11-11).

**What it owns.** A program may write a fact with a provenance, `(p k :rule R :using (…))`. The loader stores what
is written and checks none of it. Three things follow, taken on the release binary at `4bf6da73` (2026-11-11):

| what | witness | today |
|---|---|---|
| `:rule` names a rule the program does not declare | `(p k :rule zzz)` | stored as derived by `zzz`; `ein saturate --dump` and the `--dump-answer` state file's comment name it |
| a written `:rule` fact has no premises, so it leaves every core | `(p k :rule zzz)`, `p → q`, `q ∧ p → (false)` | `unsat core ∅`, `"core": []` in `nogoods.json`, where the same program without `:rule` gives `(p k)`; the same with `:rule r`, a declared rule |
| `:using` names a fact the state never holds | `(p k :rule r :using ((nope x)))` | loads, and the core is ∅ |

R is not moved by any of them. The explanation is: a core that says the refutation rests on no fact, on a program
whose refutation rests on a written one.

**Why S1f.7.9's default was not taken.** Refusing a `:rule` that names no rule leaves the second and third rows as
they are, moves two corpus fixtures (`examples/syntax/unary-and-nary.ein`, `examples/broken/load/derivation_cycle.ein`)
and four test files that write provenance by hand (`ein-ir/tests/kb_semantics.rs`, `ir_semantics.rs`,
`load_semantics.rs`, `ein-render/tests/golden_dot.rs`), and
[`01_grammar.md` § Derived facts](../../docs/kernel/ir/03-ein-lang/01_grammar.md) documents the form as the
engine's working memory. What a written provenance may claim is the question, and it is the user's.

**The question.**

- **(a) Refused**: a program may not write `:rule` or `:using`; provenance is what the engine records.
- **(b) Checked**: `:rule` must name a declared rule and `:using` facts the program holds, and a written fact with
  a provenance is a given in every core.
- **(c) As given, and marked**: loaded as written and printed as *not checked* on every surface that names it, and
  a given in every core.

**It is not F22's.** F22 is the smallest core over the derivations the engine recorded. Here the loader makes up
the derivation. A fix here may still use F22's machinery.

**Trigger.** Any of: a consumer that reads cores or `refuted by` from programs it did not write; a dump meant to
load on its own; a stage that touches a fact's clauses in `ein.y`; F22's start.

**Rows.** S1f.7.9 files the witnesses above in `corpus/wrong_answers.toml`, `open` and owned by this followup.
