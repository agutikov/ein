# E-142 — An AMR graph reads as Ein facts under a spelling of five rules, and loads with no refusal

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27, on `dd3910aa` with the stage's
`einaf analysis`
**Owner:** —

## What

**An AMR graph reads as Ein facts under a spelling of five rules, and loads with no refusal.** A node is a fact
with its sentence and its concept, a role between two nodes a fact, a constant a fact, a name and a negation a fact
each, in eight relations EinAf declares; a fact that is an argument is not needed, since a node is named. Ein's names
take what AMR writes as it stands: a sense (`drink-01`, `have-rel-role-91`), a number (`5`, a name as `1` is), the
polarity `-`, the special concepts (`amr-unknown`). What they do not take is five things, each written one way: a
role's colon (`:ARG0` is `ARG0`, a colon beginning a keyword); a string, which is no object (`"Old" "Gold"`, a name's
words, is `Old_Gold`); a character outside the names a step's grammar lets the model write (`_`); a word of Ein's own
([E-143](E-143-amrs-and-and-or-are-words-ein-never-takes-as-a-name.md)); and `_` alone. A role read the other way is
the reader's, not the spelling's. The six texts' analyses, three parsers each, 32 to 296 facts a program, all load
under `--no-search` with no diagnostic, root alive, and draw.

## Evidence

`einaf/tests/analysis.rs`' `the_facts_form_loads_with_no_refusal_and_is_judged_at_root_alone` and
`ein_render_kb_draws_the_facts`; the spelling is `einaf::analysis::name`, its table
[`docs/einaf/09-analysis.md` § 3](../../../../../docs/einaf/09-analysis.md#3-the-forms).

## What it costs the application

Nothing: the parser's graph reaches the kernel as facts the kernel loads, and a rule can read it by AMR's own closed
vocabulary (its roles and special concepts as names), which the phase's reading arm needs
([S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md)). [M1k's P1k.20](../../../../m1k_notations_as_facts/p1k.20_s_expressions_as_facts/README.md),
S-expressions as facts, meets the same five.
