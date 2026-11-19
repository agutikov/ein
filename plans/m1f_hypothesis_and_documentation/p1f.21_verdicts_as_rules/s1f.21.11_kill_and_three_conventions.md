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
**Waits on:** no decision. Its two questions are **ruled by the user, 2026-11-19**
([the rulings](README.md#the-rulings-of-the-answers-file)): [Q-M1f.557](../open_questions.md#q-m1f557--how-the-contradictions-rules-reach-every-relation-and-count) **(d)**,
the contradiction is a theory a program imports, and
[Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) **(c)**, the
argument-count check is dropped. In order it runs after [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md),
[S1f.21.16](s1f.21.16_hrules_lists_activators.md) and
[S1f.21.17](s1f.21.17_the_loader_warns_and_std_core_by_import.md), and **starts on the user's word**.
**Built on:** [S1f.21.10](s1f.21.10_the_kernels_prefix.md), the key and the module every program gets, done: the
kernel reads no `(false)` to judge a state, a rule of `std.core` is read through in an explanation (`Rule::implicit`),
and `monotone::leads_to_an_end` is what the agenda and the detector read;
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md), the contradiction's rules.
**Writes:** `ein-infer/src/` (`contradiction.rs`, `saturator.rs`, `monotone.rs`, `naf_deps.rs`,
`structure.rs`, `explain.rs`, `hypgen.rs`, `known_dead.rs`), the loader, `stdlib/`, the read-out, the events, the
goldens. After it the kernel reads no name of a theory to judge a state.

## Task 1 — the examples (stops for the user)

On real corpus programs with their answers computed, each put as a question. They are what is left of questions 1
and 6 of [`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers), which were S1f.21.4's.

1. **The contradiction is a theory a program imports** (ruled, Q-M1f.557 (d)). The user: *"(d) if any program writes a negation and wants KB with P and not P dead - then it should also import std.contradiction. Otherwise in this program's theory there is no contradictions. Or it/they are defined different. For example: Kleene K3, Priest's LP, Belnap–Dunn FDE, Relevant logic, Paraconsistent logic."*
   A scratch build with the kernel's pair check off, over the corpus:
   - the programs that write or derive a stored negation and whose answer moves with the check off, listed (the
     survey counted 65 files that write a negation and import nothing of the library);
   - each of them with `(import std.contradiction)` added, and its answer then, which is to be today's;
   - those where it is not: a pair over a relation the program does not declare, or of more than six arguments,
     each with what would give it its death, a declaration or an activator written by hand.

   **Three readings taken of the ruling, shown here and built only after the user has seen them:**
   - **the import is the switch**, with no fact beside it, so `(clash-of-declared)` goes from the module. Item 35's
     last line asked it, and the answer says *"should also import std.contradiction"*; item 40's *"as built"*
     reads the other way on this one fact;
   - **a module of the standard library that concludes a negation and means a death by it imports the module**, so
     a program that imports one of those has the contradiction; which modules, listed;
   - **a corpus program of the first list gains the import**, so that no corpus answer moves; a program that is
     better read as a theory with no contradiction is shown and left to the user.

   And shown with them: what the candidate root denies, and licence L8's denial, read where the module is not
   imported, since a state that holds a fact beside its negation is then alive.
2. **V-1 is answered**: nothing is refused. The user ruled that a non-monotone program loads under a warning
   ([Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes)), and
   [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md), which runs before this stage, takes *forbid*'s refusal out. So
   the seven programs load with the contradiction a rule, as flagged programs, and this stage re-aims no *forbid*.
3. **What moves at the same $R$** when a pair's death is a rule's: the firing that concludes `(false)`, which a
   state dead by a pair does not hold today; `refuted by` and the core, which name both sides' rules today.
   Defaults: as S1f.21.10's readings 4 and 5, the module's rules left out of an explanation. Today's engine shows
   the question already, with the rules imported
   ([V-15](findings.md#v-15--refuted-by-names-one-of-two-contradictions-by-the-relations-name)).
4. **The argument-count check goes** (ruled, Q-M1f.447 (c)). The user: *"(c) and I accept program may be not well-formed intensionally or occasionally and be silent while not binding relations with different args count. More than that - let's consider this as relations overloading but in only meaning possible in ein - in structural meaning, that means different number of args, because there is no types in ein and will not ever be."*
   Shown: each corpus program the check refuses today, loading, with its answer; a program that writes one relation
   at two counts, each query answering its count. **A reading taken, shown and built only after it is seen**: a
   relation may be **declared** once a count, where a second declaration of a name is a duplicate today, and a
   second declaration of one count stays one; and what reads a declaration reads each: the blind rung's atoms, a
   `:why`'s `{?n}`, a goal's and a claim's check, and the contradiction's `clash-of-N`, one activator a count. With
   [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
5. **The cost**: the bench set by `utils/bench_env.sh`, the pair check against the rules. A measurement, shown; no
   estimate is written.
6. **The elimination stays a module a program imports** (review item 38's default, which no answer changed, and
   what Q-M1f.557 rules of the contradiction): `std.core` gets neither. The refusal that stood beside it
   ([V-12](findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands)) goes before this stage,
   by S1f.21.16 and S1f.21.17. Nothing is shown here.
7. **What the module every program gets brings with a declaration**
   ([V-14](findings.md#v-14--a-declaration-an-import-brings-is-not-inert)). `std.core` may declare, as any module
   (the user: *"there is no requirement "std.core must declare nothing" - it may declare"*). A declaration there is
   then every program's: its labels are names a free `neq` binds over. So what the stage adds to the module is run
   over the corpus first, and a claim that moves by a declaration is shown, with the program, and not re-banked.

## Task 2 — the build, on the rulings, in this order

Each a commit that leaves the gate green, so the stage can stop between any two.

1. **The contradiction as the module's rules** (item 26), by the ruling: the kernel's pair check goes, and
   `std.contradiction` is the contradiction, on where a program imports it, for the relations the program declares;
   `not` a relation of one argument whose meaning is that rule set and, where imported, the elimination. The
   programs and the library's modules of task 1.1 gain the import in this commit. The five readers of `not`
   re-aimed: the death, the agenda's first half, the static check, the candidate root denies, and licence L8's
   denial.
2. **The surfaces' words** (item 27): the read-out, `:expect (false)`, the events' `kind: dead`, the agenda's
   first half and the `refuted by` row.
3. **The reserved words** (item 29): what a declarator may not bind is `eq`, `neq` and the connectives; `false` and
   `not` are the standard library's names, and `std.core` may then declare them, which it could not while they
   were reserved (S1f.21.10's record).
4. **`relation`** (item 28): the count check is dropped (Q-M1f.447 (c)), and a relation written at several counts
   is overloaded by its count; the standing rule in `AGENTS.md` § The language and the diagnostics' section are
   rewritten in the same commit; the blind rung's reading of the declarations stays until
   [S1f.21.12](s1f.21.12_hypotheses_as_facts.md).

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
