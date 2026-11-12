# F24 — Written provenance: what a fact's `:rule` and `:using` may claim

> **Pulled into [M12](../m12_rules_as_data/README.md) (2026-11-12)**, *rules as data*, on the user's instruction of that day; this file stays where it is. F24 keeps its question and its rows.

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

**Rows.** S1f.7.9 filed the witnesses above in `corpus/wrong_answers.toml` on 2026-11-11 (T1f.7.9.2), case W87,
each `open`, owned by this followup and reviewed `deferred`, each a program under `examples/ein-bugs/wrong-answers/`
with a corpus entry, held to today's engine by `ein-cli/tests/wrong_answers.rs`:

| row | witness | today (`[case.now]`) | `right`, and under which reading |
|---|---|---|---|
| `W87/written-rule-no-such-rule` | `written-rule-no-such-rule.ein`: `(paid Ann :rule zzz)`, one declared rule `seat` | `ein saturate --dump` answers, and prints `(paid Ann :rule zzz)` | refused, under (b); alternatives: (a) refused, (c) loads and is marked |
| `W87/written-rule-leaves-core` | `written-rule-leaves-core.ein`: the same fact, `seat` (paid → seated) and `clash` (seated ∧ paid → `(false)`); its twin writes `(paid Ann)` | R empty, root dead, core `[]` | core `(paid Ann)`, the twin's, under (c); alternatives: (a) refused, (b) refused for `zzz` and core `(paid Ann)` for a declared rule |
| `W87/written-using-never-held` | `written-using-never-held.ein`: `(paid Ann :rule seat :using ((invited Zoe)))`, `seat` and `clash` | R empty, root dead, core `[]` | core `(paid Ann)`, under (c); alternatives: (a) and (b) refused |

**No `right` is ruled.** The question above is the user's. Each row states the answer under one named reading and
the others as alternatives, as W8's rows record Q-M1e2.2 for F21
([Q-M1f.134](../m1f_hypothesis_and_documentation/open_questions.md#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled),
defaulted). The ruling restates each row's `right` before its fix flips it.

**What S1f.7.9's T1 added** (release, at `91b12be8`, 2026-11-11). The surfaces that name a written `:rule`: `ein
saturate --dump` (`(paid Ann :rule zzz)` among the derived facts), `ein render kb`, and the `--dump-answer` state
file, which writes the fact as the program wrote it and, in its fact list, `(paid Ann) ; rule zzz — Ann has paid`.
`:rule seat`, a declared rule, gives the empty core as `zzz` does. A `:using` with no `:rule`, `(paid Ann :using
((invited Zoe)))`, is stored as the given `(paid Ann)`, its `:using` dropped without a word, and the core is `(paid
Ann)`. Who writes `:rule` on a fact in the corpus: `examples/syntax/unary-and-nary.ein`,
`examples/broken/load/derivation_cycle.ein` and `examples/ein-bugs/wrong-answers/open-fact-clause.ein`, by hand; no
tool reloads a `saturate --dump`, and a state file's facts carry their provenance as a comment.

**A state file does carry the form.** It writes the program as loaded before the state's facts, so a fact the
program wrote with a `:rule` is written with it, and reloading the file stores that fact with a provenance, where
every fact of a state file is a given: `ein-cli/tests/answer_round_trip.rs` reported *1 facts reload with a
provenance, not as given* on `written-rule-no-such-rule.ein`, the first corpus program `ein solve` answers that
writes one (2026-11-11). The state's facts and its R are the same. The test names the file in `WRITTEN_PROVENANCE`
and holds it both ways, so the fix here takes it off that list. The analysis's sentence that no dump depends on the
form (S1f.7.9 § What the analysis of 2026-11-11 found) was about the fact lines, whose provenance is a comment.
