# S1f.21.11 — `not` and `relation` as conventions: the contradiction as rules

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
re-planned twice the same day, on the answer to Q-M1f.399 and then when the user unblocked
[S1f.21.10](s1f.21.10_the_kernels_prefix.md), which now builds the engine's key and takes `false` out of the
kernel's reading (items 24 and 25). **Not started.** Its direction is **ruled**:
[the fourth instruction](README.md#the-fourth-instruction), `not`, `relation` and `false` are the standard
library's conventions, `eq` and `neq` stay the kernel's.
**Group:** [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), items
26 to 29.
**Waits on:** [S1f.21.10](s1f.21.10_the_kernels_prefix.md), the key and the module every program gets;
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the contradiction's rules;
[V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none) and the choice of task 1.1,
**which has no default**, for task 2.1;
[Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention), for
task 2.4 only.
**Writes:** `ein-infer/src/` (`contradiction.rs`, `saturator.rs`, `forbid.rs`, `monotone.rs`, `naf_deps.rs`,
`structure.rs`, `explain.rs`, `hypgen.rs`, `known_dead.rs`), the loader, `stdlib/`, the read-out, the events, the
goldens. After it the kernel reads no name of a theory to judge a state.

## Task 1 — the examples (stops for the user)

On real corpus programs with their answers computed, each put as a question. They are what is left of questions 1
and 6 of [`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers), which were S1f.21.4's.

1. **The contradiction without the kernel**: a scratch build with the pair check off and the rule set on, over the
   corpus, and the programs whose answer moves, counted. This is where
   [`modifications.md`'s third point](modifications.md#what-the-listing-shows) shows: the rule set takes its
   activators from declarations and is written for 0 to 6 arguments, and a relation is any head
   (Q-M1f.522). The ways out, each shown: an activator for every head a program writes at every count, made at
   load; a premise that is a variable for a whole fact, which
   [Q-M1f.393](../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md) refuses; the
   kernel keeping the pair check as its way of running the module's rule. **No default**: the third undoes part of
   the ruling, and the choice is the user's.
2. **V-1**: the seven corpus programs *forbid* refuses once the contradiction is a rule it reads. Default: *forbid*
   reads the module's contradiction as it reads the kernel's today, that is, not as a refutation resting on an
   absence; what *forbid* should read is M1n's, and stays there.
3. **What moves at the same $R$** when a pair's death is a rule's: the firing that concludes `(false)`, which a
   state dead by a pair does not hold today; `refuted by` and the core, which name both sides' rules today.
   Defaults: as S1f.21.10's readings 4 and 5, the module's rules left out of an explanation. Today's engine shows
   the question already, with the rules imported
   ([V-15](findings.md#v-15--refuted-by-names-one-of-two-contradictions-by-the-relations-name)).
4. **The argument-count check**, Q-M1f.447's three options, each on the corpus programs the check refuses or that
   write a relation at two counts, with [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
   Shown and not chosen.
5. **The cost**: the bench set by `utils/bench_env.sh`, the pair check against the rules. A measurement, shown; no
   estimate is written.
6. **The elimination, in a module every program gets**
   ([V-12](findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands)): `eliminate-N` is
   refused beside a program that names a rule of N parameters as an argument, four corpus programs today. The ways,
   each shown: the elimination stays a module a program imports, and `not`'s meaning in the module every program
   gets is the contradiction alone; the check reads the rule as it is; the rule is written another way. Default:
   the first, which changes no program.
7. **What the module every program gets may hold**
   ([V-14](findings.md#v-14--a-declaration-an-import-brings-is-not-inert)): no declaration and no name a program's
   rule can reach, as S1f.21.10 holds `std.core` to; the contradiction's rules are checked against it before they
   are added.

## Task 2 — the build, on the rulings, in this order

Each a commit that leaves the gate green, so the stage can stop between any two.

1. **The contradiction as the module's rules** (item 26), by the rulings of tasks 1.1 and 1.2; `not` a relation of
   one argument whose meaning is that rule set and the elimination. The six readers of `not` re-aimed: the death,
   the agenda's first half, *forbid*, the static check, the candidate root denies, and licence L8's denial.
2. **The surfaces' words** (item 27): the read-out, `:expect (false)`, the events' `kind: dead`, the agenda's
   first half and the `refuted by` row.
3. **The reserved words** (item 29): what a declarator may not bind is `eq`, `neq` and the connectives; `false` and
   `not` are the standard library's names.
4. **`relation`** (item 28): the count check by Q-M1f.447's ruling; the blind rung's reading of the declarations
   stays until [S1f.21.12](s1f.21.12_hypotheses_as_facts.md).

## What must hold

- **Every answer**: $R$ of every corpus program by the corpus sweep, each mode against the oracle
  (`ein-infer/tests/explore.rs`), `ein-cli/tests/lever_invariance.rs`, `import_invariance.rs`,
  `ein-render/tests/renaming_invariance.rs`, which then renames `false` and `not` with every other name,
  `ein-infer/tests/agenda.rs` and `first_contradiction.rs`.
- **The register**: every open row answers as it does (`python3 utils/wrong_answers.py`), and every row of
  `corpus/regressions.toml` stays right.
- **A core is an explanation** and may name other rules; a core's facts are the program's, not the module's.
- The goldens move only where a dead node's facts or words do, and the diff is read.
- The standing rules it rewrites, in place and in the same commits: `AGENTS.md` § The language (`not`, *a fact that
  is an argument*, the reserved words), Q-M1f.436's decision.
- A dead node's drawing is a DOT writer's: `utils/ci_container.sh`; and the nightly, a large change to the engine.

## Acceptance

`grep` finds no read of `false` or `not` by name in a decision of `ein-infer/src/`, by the census of the kernel's
names re-taken ([`open_facts.md` § 8](open_facts.md#8-the-reserved-names-that-are-left-and-what-the-kernel-reads)
has its commands); the gate, the nightly and the container gate on the stage's last commit.

## Record

Nothing yet.
