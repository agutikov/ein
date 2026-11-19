# S1f.21.11 — the engine's key `:ein`, and `false`, `not` and `relation` as conventions

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*), and
**re-planned the same day on the user's answer to Q-M1f.399**: what ends a state is no fact `(ein.kill)` but a
rule's key, `:ein`, whose argument is a word of the engine's. **Not started.** Its direction is **ruled**:
[the fourth instruction](README.md#the-fourth-instruction) (`not`, `relation` and `false` are the standard
library's conventions, `eq` and `neq` stay the kernel's) and
[Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open) (no prefix,
rules that infer `(false)`, the key).
**Group:** [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), items
24 to 29, with what [S1f.21.10](s1f.21.10_the_kernels_prefix.md) left.
**Waits on:** [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the rules it gives every program;
[Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein),
the key's word, for task 2.1;
[Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention), for
task 2.5 only; [V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none), for task 2.2.
**Writes:** the grammar (`ein-ir/src/ein.y`) and the loader; `ein-infer/src/` (`contradiction.rs`, `saturator.rs`,
`forbid.rs`, `monotone.rs`, `naf_deps.rs`, `structure.rs`, `explain.rs`, `hypgen.rs`, `known_dead.rs`); `stdlib/`;
the read-out, the events, the goldens. It is the phase's largest stage, and the one after which the kernel reads no
name of a theory to judge a state.

## What the key is

A rule states what it concludes under `:assert`, facts, or under `:ein`, a word addressed to the engine. The word
is no fact: no knowledge base holds it, no rule matches it, and it stands under no arrow. Written with the word
[Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein)
proposes:

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

Where such a rule fires, the saturation ends and the state is dead. `(false)` is then an ordinary proposition
that rules infer, held or under an arrow, and the one rule above, in a module every program gets, is what makes a
state that holds it dead.

## Task 1 — the examples (stops for the user)

On real corpus programs with their answers computed, each put as a question with a default. They are questions 1
and 6 of [`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers), which were S1f.21.4's, in the key's form. The
key does not parse today, so the runs emulate it with a stand-in fact, as
[item 33's](review/33-Q-M1f.399/README.md#3-the-options) did.

1. **The key's form**: a rule states `:assert` or `:ein`. Default: one of the two, never both, since the answer says
   *"another key, not :assert"*; a word after `:ein` that is no word of the engine's is refused at load, so a
   misspelt one does not load in silence; a program's own rule may state the key, from whatever it matches.
2. **The module every program gets**: its name, that `ein saturate --dump` shows its rules as any rule, and whether
   a program may leave it out or replace it. Default: a module of the standard library, `std.core`, always loaded
   with no import line, not replaceable in this phase.
3. **What moves at the same $R$** when the death is a rule's: the refuting rule's place in the agenda, since it no
   longer concludes what the agenda reads by name; `refuted by`, which would name the module's rule; what a dead
   node holds. Defaults: the agenda's first half is *what leads to a rule with the key*, read through the module's
   rules, so the order is today's; `refuted by` names the program's rule, the module's left out of an explanation;
   a core is taken from the premises of the firing that ended the state.
4. **The contradiction without the kernel**: a scratch build with the pair check off and the rule set on, over the
   corpus, and the programs whose answer moves, counted. This is where
   [`modifications.md`'s third point](modifications.md#what-the-listing-shows) shows: the rule set takes its
   activators from declarations and is written for 0 to 6 arguments, and a relation is any head
   (Q-M1f.522). The ways out, each shown: an activator for every head a program writes at every count, made at
   load; a premise that is a variable for a whole fact, which
   [Q-M1f.393](../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md) refuses; the
   kernel keeping the pair check as its way of running the module's rule. **No default**: the third undoes part of
   the ruling, and the choice is the user's.
5. **V-1**: the seven corpus programs *forbid* refuses once the contradiction is a rule it reads. Default: *forbid*
   reads the module's contradiction as it reads the kernel's today, that is, not as a refutation resting on an
   absence; what *forbid* should read is M1n's, and stays there.
6. **The argument-count check**, Q-M1f.447's three options, each on the corpus programs the check refuses or that
   write a relation at two counts, with [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
   Shown and not chosen.
7. **The cost**: the bench set by `utils/bench_env.sh`, the pair check against the rules. A measurement, shown; no
   estimate is written.

## Task 2 — the build, on the rulings, in this order

Each a commit that leaves the gate green, so the stage can stop between any two.

1. **The key** (items 24 and 25, in the ruled form): the grammar's rule takes `:ein` *word*; the loader holds the
   word to the engine's; a firing of such a rule ends the saturation and makes the state dead, and writes no fact;
   the module's rule goes from `(false)` to it; the kernel reads `(false)` by its name nowhere.
2. **The contradiction as the module's rules** (item 26), by the ruling of tasks 1.4 and 1.5; `not` a relation of
   one argument whose meaning is that rule set and the elimination. The six readers of `not` re-aimed: the death,
   the agenda's first half, *forbid*, the static check, the candidate root denies, and licence L8's denial.
3. **The surfaces' words** (item 27): the read-out, `:expect (false)`, the events' `kind: dead`, the agenda's
   first half and the `refuted by` row.
4. **The reserved words** (item 29): what a declarator may not bind is `eq`, `neq` and the connectives; `:ein` is a
   key of a rule, and its words are no names a program can bind or write as a fact's head by mistake for them.
5. **`relation`** (item 28): the count check by Q-M1f.447's ruling; the blind rung's reading of the declarations
   stays until [S1f.21.12](s1f.21.12_hypotheses_as_facts.md).

## What must hold

- **Every answer**: $R$ of every corpus program by the corpus sweep, each mode against the oracle
  (`ein-infer/tests/explore.rs`), `ein-cli/tests/lever_invariance.rs`, `import_invariance.rs`,
  `ein-render/tests/renaming_invariance.rs`, `ein-infer/tests/agenda.rs` and `first_contradiction.rs`.
- **The register**: every open row answers as it does (`python3 utils/wrong_answers.py`), and every row of
  `corpus/regressions.toml` stays right.
- **A core is an explanation** and may name other rules; a core's facts are the program's, not the module's.
- A shape the grammar newly takes or refuses has its row in the grammar page's table
  (`ein-ir/tests/grammar_pages.rs`), and a refusal its section of
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2.
- The goldens move only where a dead node's facts or words do, and the diff is read.
- The standing rules it rewrites, in place and in the same commits: `AGENTS.md` § The language (`false`, `not`,
  *the agenda fires first what leads to `(false)`*, *a rule concludes a conjunction of literals*), Q-M1f.313's and
  Q-M1f.436's decisions.
- A dead node's drawing is a DOT writer's: `utils/ci_container.sh`; and the nightly, a large change to the engine.

## Acceptance

`grep` finds no read of `false` or `not` by name in a decision of `ein-infer/src/`, by item 33's census re-taken;
the gate, the nightly and the container gate on the stage's last commit.

## Record

Nothing yet.
