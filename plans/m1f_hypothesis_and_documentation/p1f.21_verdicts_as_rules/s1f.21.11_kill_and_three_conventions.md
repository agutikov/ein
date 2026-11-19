# S1f.21.11 — `(ein.kill)`, and `false`, `not` and `relation` as conventions

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**. Its direction is **ruled** by [the fourth instruction](README.md#the-fourth-instruction): `not`,
`relation` and `false` are the standard library's conventions, `eq` and `neq` stay the kernel's.
**Group:** [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), items
24 to 29.
**Waits on:** [S1f.21.10](s1f.21.10_the_kernels_prefix.md), the prefix;
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the rules it moves;
[Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), the verb's name;
[Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention), for
task 2.5 only; [V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none), for task 2.2.
**Writes:** `ein-infer/src/` (`contradiction.rs`, `saturator.rs`, `forbid.rs`, `monotone.rs`, `naf_deps.rs`,
`structure.rs`, `explain.rs`, `hypgen.rs`, `known_dead.rs`), the loader, `stdlib/`, the read-out, the events, the
goldens. It is the phase's largest stage, and the one after which the kernel reads no name of a theory to judge a
state.

## Task 1 — the examples (stops for the user)

On real corpus programs with their answers computed, each put as a question with a default. They are questions 1
and 6 of [`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers), which were S1f.21.4's.

1. **The module every program gets**: its name (the review's mock says `ein.core`), that `ein saturate --dump`
   shows its rules as any rule, and whether a program may leave it out or replace it. Default: named under the
   prefix, always loaded, not replaceable in this phase.
2. **What moves at the same $R$** when the kill is a rule, from
   [item 33's emulation](review/33-Q-M1f.399/README.md#3-the-options): the refuting rule's place in the agenda,
   since it no longer concludes the fact the agenda reads by name; `refuted by`, which names the module's rule; two
   more facts in a dead node. Defaults: the agenda's first half is *what leads to `(ein.kill)`*, read through the
   module's rules, so the order is today's; `refuted by` names the program's rule, the module's left out of an
   explanation.
3. **The contradiction without the kernel**: a scratch build with the pair check off and the rule set on, over the
   corpus, and the programs whose answer moves, counted. This is where
   [`modifications.md`'s third point](modifications.md#what-the-listing-shows) shows: the rule set takes its
   activators from declarations and is written for 0 to 6 arguments, and a relation is any head
   (Q-M1f.522). The ways out, each shown: an activator for every head a program writes at every count, made at
   load; a premise that is a variable for a whole fact, which
   [Q-M1f.393](../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md) refuses; the
   kernel keeping the pair check as its way of running the module's rule. **No default**: the third undoes part of
   the ruling, and the choice is the user's.
4. **V-1**: the seven corpus programs *forbid* refuses once the contradiction is a rule it reads. Default: *forbid*
   reads the module's contradiction as it reads the kernel's today, that is, not as a refutation resting on an
   absence; what *forbid* should read is M1n's, and stays there.
5. **The argument-count check**, Q-M1f.447's three options, each on the corpus programs the check refuses or that
   write a relation at two counts, with [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
   Shown and not chosen.
6. **The cost**: the bench set by `utils/bench_env.sh`, the pair check against the rules. A measurement, shown; no
   estimate is written.

## Task 2 — the build, on the rulings, in this order

Each a commit that leaves the gate green, so the stage can stop between any two.

1. **`(ein.kill)`** (items 24, 25): the kernel reads it held, and nothing else, to end a saturation and call the
   state dead; `(false)` is an ordinary proposition, and the module's rule concludes the kill from it. Refused
   under an arrow.
2. **The contradiction as the module's rules** (item 26), by the ruling of task 1.3 and 1.4; `not` a relation of
   one argument whose meaning is that rule set and the elimination. The six readers of `not` re-aimed: the death,
   the agenda's first half, *forbid*, the static check, the candidate root denies, and licence L8's denial.
3. **The surfaces' words** (item 27): the read-out, `:expect (false)`, the events' `kind: dead`, the agenda's
   first half and the `refuted by` row.
4. **The reserved words** (item 29): what a declarator may not bind is the prefix, `eq`, `neq` and the
   connectives.
5. **`relation`** (item 28): the count check by Q-M1f.447's ruling; the blind rung's reading of the declarations
   stays until [S1f.21.12](s1f.21.12_hypotheses_as_facts.md).

## What must hold

- **Every answer**: $R$ of every corpus program by the corpus sweep, each mode against the oracle
  (`ein-infer/tests/explore.rs`), `ein-cli/tests/lever_invariance.rs`, `import_invariance.rs`,
  `ein-render/tests/renaming_invariance.rs`, `ein-infer/tests/agenda.rs` and `first_contradiction.rs`.
- **The register**: every open row answers as it does (`python3 utils/wrong_answers.py`), and every row of
  `corpus/regressions.toml` stays right.
- **A core is an explanation** and may name other rules; a core's facts are the program's, not the module's.
- The goldens move only where a dead node's facts or words do, and the diff is read.
- The standing rules it rewrites, in place and in the same commits: `AGENTS.md` § The language (`false`, `not`,
  *the agenda fires first what leads to `(false)`*), Q-M1f.313's and Q-M1f.436's decisions.
- A dead node's drawing is a DOT writer's: `utils/ci_container.sh`; and the nightly, a large change to the engine.

## Acceptance

`grep` finds no read of `false` or `not` by name in a decision of `ein-infer/src/`, by item 33's census re-taken;
the gate, the nightly and the container gate on the stage's last commit.

## Record

Nothing yet.
