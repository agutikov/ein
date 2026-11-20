# S1f.21.11 — `not` and `relation` as conventions: the contradiction as rules

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
re-planned twice the same day, on the answer to Q-M1f.399 and then when the user unblocked
[S1f.21.10](s1f.21.10_the_kernels_prefix.md), which now builds the engine's key and takes `false` out of the
kernel's reading (items 24 and 25). **Done 2026-11-20** ([the record](#record)); started the same day on the user's word (*"Start stages, if anything can be
done in parallel - do it."*), beside S1f.21.17. Its direction is **ruled**:
[the fourth instruction](README.md#the-fourth-instruction), `not`, `relation` and `false` are the standard
library's conventions, `eq` and `neq` stay the kernel's.
**Group:** [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), items
26 to 29.
**Waits on:** no decision. Its two questions are **ruled by the user, 2026-11-19**
([the rulings](README.md#the-rulings-of-the-answers-file)): [Q-M1f.557](../open_questions.md#q-m1f557--how-the-contradictions-rules-reach-every-relation-and-count) **(d)**,
the contradiction is a theory a program imports, and
[Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) **(c)**, the
argument-count check is dropped. In order it runs after [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md) and
[S1f.21.17](s1f.21.17_the_loader_warns_and_std_core_by_import.md), and **starts on the user's word**
(S1f.21.16, which stood between them, is dropped).
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

   **Three readings taken of the ruling. The user confirmed them, 2026-11-20** (*"b - yes"*), so they are built, and
   the lists below are the stage's record and no stop:
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
   at two counts, each query answering its count. **The reading, confirmed and widened by the user, 2026-11-20**: *"c - yes, it may because relation declaration is
   ordinary fact, and it may appear multiple times, rules may define what it means and how those multiple declarations
   used."* So a relation may be **declared** more than once, at one count or at several, and no declaration is refused
   as a duplicate, where a second declaration of a name is refused today; and what reads a declaration reads each: the blind rung's atoms, a
   `:why`'s `{?n}`, a goal's and a claim's check, and the contradiction's `clash-of-N`, one activator a count. With
   [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count).
5. **The cost**: the bench set by `utils/bench_env.sh`, the pair check against the rules. A measurement, shown; no
   estimate is written.
6. **The elimination stays a module a program imports** (review item 38's default, which no answer changed, and
   what Q-M1f.557 rules of the contradiction): `std.core` gets neither. The refusal that stood beside it
   ([V-12](findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands)) goes before this stage,
   by S1f.21.17; what S1f.21.16 would have taken of it goes with the `hrule` form, in S1f.21.18. Nothing is shown here.
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

Run 2026-11-20 in a worktree branched from `master` at `669d7d43`, as the brief asked; the worktree's branch stood at
`e4a5f8fb` and was fast-forwarded to `669d7d43` first. Task 1 was no stop (its readings confirmed by the user,
2026-11-20); task 1.5, the cost, was not taken (below).

### Task 1 — the surveys

**How they were taken.** Every corpus entry, 931 files, `ein solve FILE --json-summary`, six at a time, by a script
in the worktree's scratch (not in the tree), on three binaries of the gate profile:

- **base**, the tree at `669d7d43`;
- **check off**, a scratch build of the same tree with the kernel's pair check off (`contradiction::detect`,
  `has_contradiction` and the incremental `contradicts` reading no pair) and every reading of a denial off
  (`Kb::is_negated` answering no, which turns off the candidate root denies, L8, and rung 2's and the structure's
  readings of a denial);
- **rules**, the same scratch build with the denials on, over a copy of the corpus where each program of the first
  list below gained `(import std.contradiction)`, and a copy of the library where `std.contradiction`'s
  `clash-of-N` read no `(clash-of-declared)` and `std.algebra`, `std.bijection` and `std.slots` import the module.

The answer compared: the exit code; the summary's `region` but `stopped` and `complete_up_to` (its counts, states,
lines, components, sub-cubes, `empty`, `holes`, `universe`, `licences`, `nogoods`, cores and `refuted_by`); every
query's classes; the claim's outcome; the static verdict; root's `dead`.

**1.1 (a) What moves with the check off: 47 of 931 programs.** 36 move in their answer (R, a query's classes, a
claim, an exit), 11 more in H, the licences or the nogoods only (the economies a denial buys) or in an explanation
alone. `A` an answer, `E` the economies or an explanation:

| | program | what moved |
|---|---|---|
| A | `examples/beliefs/04_liar_denial_contradicted.ein` | root alive, R {∅}; claim and exit |
| A | `examples/branching/01_saturate_only.ein` to `11_kill_cache_off.ein` (11 programs) | R, the queries |
| E | `examples/branching/12_typed_blind_solve.ein` | H, licences, nogoods |
| A | `examples/domain_elim/ab.ein`, `b_only.ein` | R's states, the maximal line |
| E | `examples/ein-bugs/zebra2-bad.ein` | core, `refuted by` |
| E | `examples/ein-bugs/zebra2-hints.ein` | core, `refuted by`, H |
| A | `examples/features/01_not_and_absent.ein`, `04_open.ein`, `05_stdlib_domain_elim.ein`, `09_adjacent_via_same_house.ein` | R, the queries (09: root alive) |
| A | `examples/saturation/hypothesis-contradiction/coloc-disproved.ein`, `next-to-disproved.ein`, `right-of-disproved.ein` | R, the queries |
| A | `examples/saturation/type-exclusivity/colors.ein`, `nationalities.ein`, `pets.ein` | R, the queries |
| E | `examples/zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein` | H, licences |
| A | `examples/zebra2-hints.ein`, `zebra2-minus-15.ein`, `zebra2-minus-15-obligations.ein` | R's states (the last two stop at the budget) |
| A | `examples/ein-bugs/d3-unsat-core.ein` | R |
| E | `examples/ein-bugs/nested-core.ein`, `wrong-answers/negative-only.ein`, `negative-only-declared.ein` | H, nogoods |
| A | `examples/ein-bugs/wrong-answers/pair-core-root-given.ein`, `-derived.ein`, `-negative.ein`, `forall-core-elim.ein`, `goal-nested-var.ein` | R |
| A | `examples/ein-bugs/wrong-answers/hrule-reads-absent-not.ein`, `hrule-reads-unknown.ein` | R, the queries, exit |
| E | `tests/stdlib/bijection/06_blind_enumeration.ein` | H, nogoods |
| E | `tests/stdlib/contradiction/04_clash_3.ein` | `refuted by` |
| A | `tests/region/10_denied_extensions.ein` | R, exit |

The survey's earlier count, *65 files that write a negation and import nothing of the library*, is 81 on this tree
(81 write `(not …)` outside a comment and take the contradiction from no module of the library as the base had it);
of those, 45 do not move with the check off: they write or derive a negation and no state of theirs rests a death
on it (`examples/beliefs/01`, `03`, `05`, `06`, `09`, `10`, `11`, `features/12_expect_false.ein`, the
`syntax/` programs, most of the `wrong-answers/` refusal witnesses, `tests/stdlib/elim/01`, `02`, `macro/02`,
`negation/01`, and others; the full list is the scratch file `negation-unmoved.txt`, taken with the record's
script).

**1.1 (b) Each with the import.** The 35 of the 47 that import no module that carries the contradiction gained
`(import std.contradiction)` (the default, *it gains the import, so no answer moves*); the other 12 import
`std.algebra`, `std.bijection` or `std.slots`, which import it now. On the **rules** build every one of the 47
answered as on base in every field compared, H, licences and nogoods among them, but one, below. On the stage's
binary (task 2.1) the same holds of all 931: no answer moved but those below, and no core and no `refuted by`
moved but `tests/stdlib/contradiction/`'s (§ 1.3).

**1.1 (c) Not restored by the import.**

- `examples/beliefs/04_liar_denial_contradicted.ein`: its pair is `(not (= 5 (+ 2 2)))` beside its own negation,
  a pair over the relation `not`, which no declaration names and which, a reserved word, no declaration can name
  nor an activator write: `(clash-1 not)` does not parse (`unexpected not`). **What gives it its death**: the
  program states its own rule for the one shape it writes, `denial-denied`, `(and (not (= ?a ?b)) (not (not (= ?a
  ?b))))` ⟹ `(false)`. Its answer, core and `refuted by (given), distrust` are then the base's. With task 2.3's
  `not` a name, `(clash-1 not)` written by hand would do it.
- `tests/stdlib/contradiction/10_clash_of_not_asked.ein`: it held that the import alone activates nothing, which the
  ruling reverses. Rewritten as `10_undeclared_not_reached.ein` (below).
- No corpus pair is over a relation of more than six arguments (S1f.21.8's census: no negation of a fact of seven or
  more arguments at root), and none over an undeclared relation but the liar's.

The two inline programs of `ein-cli/tests/negation_of_a_negation.rs` that rest a death on a negation of a negation
(`(not g)` beside `(not (not g))`, and `(ok b)` beside `(not (ok b))` under a search) state their own rules for their
shapes, as the liar program does; the one of `ein-cli/tests/hrule_reads_not.rs` gains the import.

**1.1 (d) The modules.** `std.algebra` (`symmetric-negative`, `compose-negative-s`, `-r`), `std.bijection`
(`functional-negative`, `injective-negative`) and `std.slots` (eleven exclusion rules) conclude a stored negation
and mean a death by it: each imports `std.contradiction`. `std.debts` imports `std.algebra` and `std.slots`, so it
carries it too. `std.elim` reads negations and concludes `(false)` itself; `std.negation` concludes positives; no
other module writes `(not …)`.

**1.1 (e) What the candidate root denies and L8, where the module is not imported.** Read off the rules now
(`contradiction::Pairs`): a candidate root holds `(not c)` of is out of H only where a compiled rule refutes `c`
beside `(not c)` alone; with no such rule it is in H, and committing it gives an alive state holding both. Where the
module is not imported, a state denies nothing, and L8 grants nothing. Over the corpus with the imports above, H,
the licences and the nogoods of every program are the base's. One inline program of a test showed the other side
(`hrule_reads_not.rs`' `two.ein`, before its import: `(paid Bob)` joined H, eight states where there were four).

**1.2.** Nothing is refused: S1f.21.15 took *forbid* out before this stage, so the seven programs it refused load
with the contradiction a rule, flagged as they were.

**1.3 What moves at the same R.** The default taken, as S1f.21.10's readings 4 and 5: an explanation reads through
the module's rules. Built as a reading of the provenance with no name (`contradiction::detect`): a firing of the
rule every program gets is read through to what it matched, the `(false)`, and through every recorded derivation of
that `(false)` whose premises are a fact and one fact holding it as its one argument, to that pair, explained
together as the kernel's pair was. So over the corpus no core and no `refuted by` moved, but
`tests/stdlib/contradiction/01` to `07` but `04`, which read `refuted by clash-N` and now `(given)`, as `04` did:
**V-15 is gone**, the state holding one contradiction where it held two. What moved: a dead-by-a-pair state holds the
`(false)` the rule concluded, and the firings of `clash-N` and of `std.core`'s rule; so the read-out's `work` row
(facts beyond root) of 23 programs, the budget numbers of the two `zebra2-minus-15` programs, the trace's steps.

**1.4 The count check.** It refuses seven corpus programs, each a witness of `corpus/regressions.toml`, and the
duplicate refusal one (task 2.4 records what each answers once it loads).

**1.5 The cost: not taken**, on the brief's word (a shared machine). It is owed to the coordinator: the bench set by
`utils/bench_env.sh`, the pair check against the rules.

**1.6** Nothing: the elimination stays a module a program imports.

**1.7** `std.core` gains no declaration in this stage (task 2.3 below).

### Task 2.1 — the contradiction as the module's rules

**The kernel's check is gone.** `ein_infer::contradiction` reads no `not`: a state is dead where a rule that
states `:ein inconsistent` fired, and in no other way (`has_contradiction` is `Kb::is_inconsistent`; the
incremental `contradicts` and the saturator's stop on it are gone). `std.contradiction`'s `clash-of-0` to
`clash-of-6` read no `(clash-of-declared)`: the import is the switch, for every relation the program declares.
`std.algebra`, `std.bijection` and `std.slots` import the module. The five readers, re-aimed with no name:

1. **the death**: `clash-N` concludes `(false)`, `std.core`'s rule ends the state. An explanation reads a firing of
   `std.core`'s rule through to the `(false)` it matched, and through each recorded derivation of that `(false)`
   whose premises are a fact and one fact holding it as its one argument to the pair (`contradiction::detect`,
   `Contradiction::Pair` now *a pair a rule refuted*), so cores and `refuted by` read as before;
2. **the agenda**: what refutes a pair is read off the compiled plans (`contradiction::Pairs`: a plan whose
   disjunct is `(R ?a …)` of distinct variables and `(N (R ?a …))`, alone, and whose firing ends a state). Its
   candidate fires before every other, as a rule that states the key does (`Saturator::at_once`), so a state ends
   at the firing that completes the pair, whatever was queued; the first half holds a conclusion that is either
   side of such a pair (`FailFirst::Pairs`, `Negations` the holding side alone);
3. **the static check** (`monotone`): `not` is a relation as any other, read at relation grain (`Rel` lost its
   `negated` field and the wildcard); `Refutes::Not` is `Refutes::Holding { wrapper }`, a conclusion that is the
   holding side of a pair the rules refute; `compile::asserted_relation` and `naf_relation_refs` read `not` as any
   relation and `negated_relation` is gone; `extendable`'s heads, `scope`, `unheld` likewise. The corpus's verdicts
   do not move (the scratch survey, every program): two flagged read-outs' labels do, `(not r1)` → `not`;
4. **the candidate root denies** (`hypgen`, and rung 2's seed in `oblgen`): `Pairs::denies`, root holding `(N c)`
   for a rule that refutes `c` beside it;
5. **L8's denial** (`region`'s node, through `Axes`, and the structure's `WitnessIs::Denied` and L5's count in
   `oblgen`): the same.

`hrule.rs`' look-ahead over a premise that reads `not` is gone: no caller writes between two candidates, so each
hrule is matched as it is reached, which answers alike. `structure::subject` reads `(N X)` as `X`'s relation where
a rule refutes the pair, else as `N`'s.

**What changed in the corpus.**

- 34 programs gained `(import std.contradiction)` with one comment line (§ 1.1 (b)'s list less the liar program
  and the contradiction test); `examples/beliefs/04_liar_denial_contradicted.ein` states `denial-denied`.
- `tests/stdlib/contradiction/`: the comments say the rule is the verdict; `09` loses `(clash-of-declared)`;
  `10_clash_of_not_asked.ein` is renamed `10_undeclared_not_reached.ein` and holds a relation no declaration names,
  its fact beside its negation alive.
- `std.algebra`, `std.bijection`, `std.slots` import `std.contradiction`; the manifest is written again.
- **The register**: `W36/alt-flag-refuted-by` is fixed (it reads `refuted by no-room` under the flag, the right
  answer: one contradiction where there were two) and `W36/alt-flag-two-derivations` no longer shows the gap (its
  pair is no contradiction, and under the flag the core moves, the exemption, with `refuted by k1` both ways). Both
  rows moved to `corpus/regressions.toml` (`alt-flag-refuted-by`, `alt-flag-two-derivations`), their witnesses into
  `examples/ein-bugs/wrong-answers/` with corpus entries and comments that name no row; `lever_invariance`'s
  `REFUTED_BY_MOVES` and its test went with them. `W10/second-preparation`'s recorded nogood core moved, to
  another explanation (21 facts where it recorded 18; the right one is 2, F22's): its `[case.now]` is today's.
  The register holds 9 open rows (11 before).

### Task 2.2 — the surfaces' words

Every dead state is now one a rule that states `:ein inconsistent` ended, so the words stay true and stay: the
read-out's *holds a contradiction*, `:expect (false)` (a claim's word for an empty R), the events' `kind: dead`, and
`refuted by` (the rule that concluded the `(false)`, read through as § 1.3 has it). What moved: `ein saturate --dump`
counts the `(false)` a dead state holds as one fact, and `ein render h-space` says first on stderr that a flagged
program's answer is not claimed, as every other command that answers about a commitment does. Pages: `01-objects.md`,
`03-facts.md`, `21-core.md`, `05-cli.md`.

### Task 2.3 — the reserved words

**`false` and `not` are the standard library's names.** `ein_core::LIBRARY_NAMES` holds the two, and a declarator
binds a name where `ein_core::binds_no_reserved` says so: none of `RESERVED` but those two, as a relation, a rule,
an hrule or an import's `:as`. **A macro may not** (default below): a macro named `false` or `not` would make every
`(false)` and `(not X)` its call, `std.core`'s own rule among them, which then fails to load in a way that names
the module. What a declarator binds none of: `eq`, `neq`, `and`, `or`, `absent`, `open`, `relation`, `__closed__`.

**`not` is a position word**, as `false` already was: a token of its own and a name wherever a name goes
(`ein.y`'s `Name`), but where a form's head is read (`HeadName`: a template's head, an obligation's `(open …)`),
since there `(not X)` is the stored negation and nothing else. So `(clash-1 not)` and `(relation not Claim)`
parse, and the printer prints `not` bare as an argument (`ein_core::text::LEXER_KEYWORDS` is twelve words, as
`grammar::RESERVED_WORDS` is; `grammar::POSITION_WORDS` fourteen). `DeclName` takes `false` and `not`. No LR
conflict. A first build that took `not` in `DeclName` alone printed the name quoted as an argument, so that a
declaration's fact `(relation "not" Claim)` drew `not` as two nodes of the KB view; that build was not committed.

**The liar program** (`examples/beliefs/04_liar_denial_contradicted.ein`) imports `std.contradiction` and declares
`(relation not Claim)`, in place of the rule of its own it stated in task 2.1: `clash-1` reads its pair, and its
core and `refuted by (given), distrust` are as they were.

**What moved**: one repair line of `grammar_decisions.txt` (`(x (()))` lists *Insert a name, …* once, `not` being
a name there now); the grammar page's generated regions and three EBNF diagrams (`Name`, `HeadName`, `DeclName`,
the lexer's two word classes), drawn again; the generator `grammar_pages.rs` reads the words `Name` takes through
`HeadName`. Pages: `03-reserved-names.md` (the guard, the two sets, `not`'s row: a contradiction where a rule
refutes the pair), `01-grammar.md` § 1, `03-design/04-ir-frontend.md` § 2; `AGENTS.md` § The language.

### Task 2.4 — `relation`: overloading by the count

**The count check is gone, and so is the duplicate refusal** (Q-M1f.447). The loader's `declared_counts` and
`wrong_counts` passes are deleted, with the line they printed; `ingest_relation` refuses no second declaration: the
registry (`Program::relations`) keeps the first, and `Program::declarations` (a new `ein_core::Declaration`, name,
signature, `:why`) keeps every one in the order written, read through `Program::signatures(name)`. What reads a
declaration reads each:

- **the blind rung** (`hypgen`, `invariant`'s labels): a relation is guessed at each count it is declared with, of
  one or two, and no label of any declaration is a guessable atom; the scope note (`scope::outside`) names a
  relation once, at the count first declared, and not where the rung proposes it at any count;
- **the trace** (`ein-render`'s `sentence`): a declaration's fact reads by that declaration's `:why`;
- **`saturate --dump`** and `oblgen`: each declaration;
- **the loader's goal and claim checks** and the `eq` naming: each signature;
- **`std.contradiction`'s `clash-of-N`**: one activator per declared count, by the rule as it stands.

**The corpus.** `examples/broken/load/relation_duplicate.{ein,expected}` and its entry are gone (no refusal);
`examples/features/22_declared_twice.ein` is new: one relation declared twice at two arguments and once at one,
four claims computed by hand. The seven `arity-*` witnesses load: their comments say the answer by hand is the
loaded one, their corpus notes say so, and their `corpus/regressions.toml` rows hold `answered = true` with the
region each answers (one state, root, in R), under `decisions = ["Q-M1f.447"]`.

**The pages.** Diagnostics § 2.26 is gone and § 2.27–2.31 are § 2.26–2.30, with every reference; the *duplicate
relation* sentence is gone. `02-syntax.md`, `02-relations.md`, `01-knowledge-base.md`, `02-rewriting-rules.md`,
`07-calculator.md`, `16-nine-digit-numbers.md`, `stdlib/README.md`, the three calculator modules' headers,
`08-reasoning-in-code/01-static-ir.md`, `01-grammar.md` § 3, `03-design/04-ir-frontend.md` § 9 and `AGENTS.md`
§ The language say the current state. Q-M1f.315, .201, .202,
.203 and .204 are deleted (superseded) and every citation is Q-M1f.447's, written as a decision file; Q-M1f.372 is
rewritten in place. The fuzz suite's property 8 (no accepted program holds a form at another count) is gone with
the check it held.

**The tests.** `ein-cli/tests/declared_counts.rs` is rewritten: every shape the check refused loads on the commands
that state no claim (`ein test` exits 2 on a file with no `:expect`, and `ein oracle` refuses a cube past its limit,
which is no load), the seven witnesses load, a relation declared twice and at two counts answers each count's facts;
its obligation case writes a well-formed obligation now that the count no longer refused it first. The count case of
`negation_of_a_negation.rs`' refusals goes. `dot_wellformed.rs` names `examples/ein-bugs/wrong-answers` among the
directories with a view past its bound: the calculator's count witnesses load and hold the tables. The loader's unit
test of accumulated errors and `load_semantics.rs`' pass order take a relation named after the dunder for pass 1's
error.

**The calculator.** A wrong-count numeral or request, `(n 1)`, `(add (n z 5))`, a `num9` of eight digits, is another
fact no rule of the calculator reads: the request is answered by silence, as the ruling has it, and the modules'
headers, `stdlib/README.md` and the two reference pages say so; `arith_theory.rs`' `the_tagged_calls_keep_their_shapes`
is now what holds a caller's count against the interface.

**The order of the commits.** Task 2.4 was committed before task 2.3: a first build of 2.3 that took `not` in
`DeclName` alone changed four parse fixtures' repairs and printed the name quoted as an argument, and was set aside;
2.4 was built and committed on 2.2, and 2.3 rebuilt on it, with `not` a position word. Each commit leaves the tests it
touches green; the goldens are banked in a commit of their own, after the rebase onto `master`.

**The rebase onto `master`** (S1f.21.17, `24491e80`; the ledger's re-aimed links, `f9a818e6`). Three conflicts in
2.1 (`ein-infer`'s export list, `stdlib/README.md`'s `std.core` paragraph, the manifest) and five in 2.4 (the
corpus manifest's features and the moved `derived_activator` fixtures, the features row of `examples/README.md`, the
features count of `test_cli.rs`, the diagnostics' § 2.7 paragraph, which reads § 2.29 for the saved file, and the
manifest), each resolved by keeping both sides' content. The new example is `22_declared_twice.ein`, since `master`
took `19` to `21`. The goldens were banked on the rebased tip.

### The census of the kernel's names, re-taken

`grep -rnE 'kernel(\(\))?\.NAME\b' ein.rs/crates/*/src`, and the same for the local spelling `k.NAME`, which the
census's command does not see, on the stage's last commit. **No read of `not` or `false` by name is left in a
decision of `ein-infer/src/`**, none of the death, the agenda, the static check, the candidates root denies, L8, the
explanation or the structure's denial. What is left, each writing or shaping and judging no state:

| where | reads | what for |
|---|---|---|
| `ein-infer/src/firing.rs` `build_conclusion` | `kernel.not` | a conclusion `(not ?x)` with `?x` bound to a name builds no fact (the language's *`not` of an atom is the negation of no fact*) |
| `ein-infer/src/firing.rs` `computed` | `k.not` | `(not (eq a b))` is the computed `(neq a b)`: `eq` and `neq` stay the kernel's |
| `ein-infer/src/firing.rs` `build_conclusion` | `kernel.r#false` | a false `eq` or `neq` conclusion writes `(false)`, the convention's fact |
| `ein-infer/src/monotone.rs` `refutes`, `writes_of` | `kernel.r#false` | the same, as the static check reads a plan's conclusions |
| `ein-infer/src/hypgen.rs` `candidate_objects` | `k.not`, `k.r#false` | the grammar's words the blind rung never guesses as objects |
| `ein-infer/src/match_.rs` | `kernel.not` | a unit test's fixture |
| `ein-core/src/kb.rs` | `kernel.not` | the KB's index of the facts a `(not …)` holds (`Kb::is_negated`, `Kb::negated`), which no reader in `ein-infer` reads now; the KB's shape dump prints it |

Outside `ein-infer`: the grammar and the loader's shapes of `not` and `false`, a claim's `(false)`, the printer.

### The defaults taken

Each where the plan says *shown and left to the user*, or where the build met a choice the plan does not make; none
moves a corpus answer.

| id | default |
|---|---|
| Q-M1f.585 | Every corpus program whose answer rests on a fact beside its stored negation **gains `(import std.contradiction)`**, one comment line with it, so that no answer moves (§ 1.1 (b)); and the library's modules that conclude a stored negation and mean a death by it, `std.algebra`, `std.bijection`, `std.slots`, import it. |
| Q-M1f.586 | **An explanation reads through the rules**: a firing of `std.core`'s rule is read through to the `(false)` it matched, and through each recorded derivation of that `(false)` whose premises are a fact and one fact holding it as its one argument, to that pair. Cores and `refuted by` read as before (V-15 is gone with it). |
| Q-M1f.587 | **A rule that refutes a pair fires at once**, in the band of a rule that states the key, so a state ends at the firing that completes the pair whatever was queued; the agenda's first half holds either side of a pair a rule refutes. Read off the compiled plans, `contradiction::Pairs`, with no name. |
| Q-M1f.588 | **What read a denial reads `Pairs`**: a candidate root holds `(N c)` of is out of H only where a compiled rule refutes `c` beside `(N c)` alone; L8's denial, rung 2's seed, L5's count and the structure's `Denied` alike. Where no rule refutes the pair, root denies nothing. |
| Q-M1f.589 | **The static check reads `not` as any relation**, at relation grain (`Rel` has no `negated` field), and `Refutes::Holding { wrapper }` is a conclusion that is the holding side of a pair the rules refute. No verdict of the corpus moved; two flagged read-outs' labels did, `(not r1)` → `not`. |
| Q-M1f.590 | **An hrule is matched as it is reached**: the look-ahead over a premise that reads `not` goes, no caller writing between two candidates. |
| Q-M1f.591 | **The surfaces' words stay** (task 2.2): *holds a contradiction*, `:expect (false)`, `kind: dead`, `refuted by`. |
| Q-M1f.592 | **The register**: `W36`'s two rows answer right and move to `corpus/regressions.toml` with their witnesses (`alt-flag-refuted-by`, `alt-flag-two-derivations`); `W10`'s recorded nogood core is today's, another explanation of the same answer. |
| Q-M1f.593 | **`not` is a position word**: a name wherever a name goes but where a form's head is read (`HeadName`), so `(clash-1 not)` and `(relation not Claim)` are written and print bare. A relation, a rule, an hrule and an alias may bind `false` and `not`; **a macro may not**, its calls being every `(false)` and `(not X)`, `std.core`'s among them. `relation` stays one a declarator does not bind. The liar program declares `not` in place of a rule of its own. |
| Q-M1f.594 | **The count check's witnesses stay**: the seven `arity-*` programs are regressions whose right answer is the loaded one (`answered = true`, their region); the duplicate-declaration fixture goes. A relation declared at several counts is read at each by every reader of a declaration; the scope note names it once, at the count first declared, and the blind rung proposes it where any count is one or two. |

### Findings outside the stage

- **V-35 — `renaming_invariance.rs` does not rename `false` and `not`.** The plan's *What must hold* has it rename
  them with every other name. They are tokens of their own the renamer does not touch, and the kernel still reads
  both where `eq` and `neq` are its own: `(not (eq a b))` is the computed `neq`, a false `eq` or `neq` conclusion
  writes `(false)`, `(not ?x)` of a name builds no fact, and the blind rung's objects leave both out (the census
  below). Renaming them takes those four readings off the kernel first.
- **V-36 — `ein-core`'s index of negations has no reader that decides.** `Kb::is_negated` and `Kb::negated` are read
  by no decision of `ein-infer` now; the KB's shape dump prints the index, a unit test of the loader reads it, and
  two measurement examples under `ein-infer/examples/` (`early_stop.rs`, `shared/prices.rs`) still ask it whether a
  candidate is denied, which is now `Pairs::denies`. A candidate for removal, with the dump's rows.
- **V-37 — every declared relation activates `clash-0` as well as the rule of its count.** `clash-of-0` matches
  `(relation ?R)`, which is also the membership fact every declaration stores, so a program that imports
  `std.contradiction` compiles a pair rule of no argument for each declared relation; the traces narrate one
  `clash-of-0` step a relation at root. No answer depends on it; it costs a plan a relation and the trace's lines.
- **V-38 — a dead state's explanation names the pairs concluded before it ended.** The state ends at the firing of
  `std.core`'s rule, which a pair-refuting rule reaches at once, so where the kernel's check listed every pair a
  state held, `detect` lists the pairs whose `(false)` was derived by then, and the recorded derivations are capped
  (`MAX_ALT_JUSTIFICATIONS`). No core and no `refuted by` of the corpus moved; a reader of every contradiction a
  state holds (the structure) reads `detect_all`.
- **V-39 — the cost is not measured** (task 1.5, on the brief's word): the bench set by `utils/bench_env.sh`, the
  pair check against the rules, is owed. The import adds the module's fourteen rules to every program that takes
  it, and two pair plans a declared relation, its count's and V-37's.
- **[V-30](findings.md#v-30--a-duplicated-declaration-loads-once-an-import-is-appended)** is answered by
  Q-M1f.447: a declaration twice loads, wherever it is written.

### The commits

- `0345e799` The contradiction is a theory a program imports: the kernel's pair check goes, std.contradiction's import is the switch, and what read a denial reads the rules that refute a pair (Q-M1f.557; S1f.21.11, task 2.1).
- `d6773d76` The surfaces after the contradiction's move: the saturated dump counts one fact as one fact, and ein render h-space says first on stderr that a flagged program's answer is not claimed (S1f.21.11, task 2.2).
- `663ca7d1` A relation is overloaded by its count of arguments: the loader holds no form to its declaration's count and refuses no second declaration, and what reads a declaration reads each (Q-M1f.447; S1f.21.11, task 2.4).
- `3617f009` false and not are the standard library's names: not is a position word, a name wherever a name goes but a head, and a relation, a rule, an hrule and an alias may bind either; a macro may not (Q-M1f.557; S1f.21.11, task 2.3).
- `7f69b9ac` The goldens banked again on the contradiction as rules, the count of arguments overloaded and not a name (S1f.21.11).
- `04665c38` contradiction::Pairs::wrappers says why its order is not observed: its one caller collects into a set (the determinism lint).
- this record, plans only.

### The gate

`./run_tests.sh` once, on the rebased tip (`04665c38`, on `master` at `f9a818e6`), with the default job counts:
green, `exit 0`. The ten static checks clean (`doc_audit`: 2409 files, 0 findings; the release voice; the
determinism lint, after `04665c38` annotated the one iteration it found on the first run, which stopped there);
`cargo test --profile gate --workspace`, 167 test binaries, 1373 tests passed, none failed; then, as the gate prints
them:

<!-- output -->
```text
── utils/check_generated.py ──────────────────────────────────────
check_generated: 9 generators, every file they write as written
── utils/check_region_routes.py ──────────────────────────────────
check_region_routes: the oracle agrees with the per-subset route on 8 programs, and holds 300 random regions written as programs to their verdict, the monotone ones node for node
── utils/wrong_answers.py ────────────────────────────────────────
wrong_answers: 9 open rows, 0 defects in the register's keys
── utils/review_bundle.py --smoke ────────────────────────────────
smoke: the register pass read 9 rows, 0 failed
smoke: 24 SVGs, link check exit 0
── cargo bench --bench engine -- --test ──────────────────────────
Testing region_single/zebra2
Success
```

Not run, on the brief's word: the nightly, the dev host's tier and `utils/ci_container.sh`, which the plan's
acceptance names and the stage's DOT changes (a dead node's drawing) make part of done: **owed**, with the cost
(V-39). The census of the kernel's names was re-taken on the tip and is the table above.

### The coordinator's part: the runs on the merged part 1

Taken by the coordinator after both builds of part 1 were merged (S1f.21.17 as `24491e80`, S1f.21.11 up to
`04665c38`, master at `ddf09492`), in the runner's image:

- **`utils/ci_container.sh`, the gate: exit 0** on `ddf09492` (`build/ci-logs/gate-ddf09492.log`, on this machine).
- **`utils/ci_container.sh --nightly`: exit 101**, at its first test step, *corpus_cli, the slow entries*
  (`build/ci-logs/nightly-ddf09492.log`). One test fails, `the_slow_flag_still_describes_the_sweep`, on two entries'
  cost: `examples/beyond_arith/equation_two_unknowns.ein`, 6034 ms of cells with no `slow` flag, and
  `examples/beyond_arith/send_more_money_written.ein`, recorded at 1430 ms and swept in 5739 ms. The steps after it
  did not run.
- **The cause is the cost of the contradiction as rules, not the machine's load alone.** Each program run by turns
  on the build before S1f.21.11 (`build/review-bin/ein-head-edbe6c3b`, with that tree's library and program) and
  on the merged build, three times each, under one load, the same work in both:

  | program | run | saturations | before | after |
  |---|---|---:|---:|---:|
  | `equation_two_unknowns.ein` | `ein solve` | 1 191 | 0.52 s | 1.37 s |
  | `send_more_money_written.ein` | `ein solve --single` | 18 032 | 1.00 s | 1.39 s |
  | `zebra2.ein` | `ein solve` | 120 | 0.12 s | 0.15 s |

  They are no measurement: two other stages were building at the time. The measurement by `utils/bench_env.sh`
  is still owed (V-39), on a quiet machine; and the corpus's costs and `slow` flags are taken again then, once
  the phase's builds are all in, since each of them moves the costs.
- **The nightly's other steps, run in the main tree with that one test skipped**, to find what it hid
  (`./run_tests.sh --nightly` from a scratch copy that skips `the_slow_flag_still_describes_the_sweep` alone):
  - the first run stopped at *dot_wellformed, every program*: `tests/stdlib/bool/inputs/01_in_drives.ein` and
    `02_required.ein` have a `kb-since` view of 1 085 and 1 097 edges with the contradiction's rules in their
    library, past the sweep's bound of 1 000, in a directory its list did not name. Fixed by the coordinator,
    `a010b369`: the directory is named;
  - **the second run, on `a010b369`, exit 0**: all 76 steps, the deep corpus, the four shards, the twelve shards of
    the mutation census, the fuzzer's two seeds and the bench smoke (`build/ci-logs/nightly-rest-local-a010b369.log`).
  So the merged part 1 passes the nightly but for the cost check. That run was in the main tree, with this
  machine's Graphviz; the nightly in the runner's image is run whole once the costs are taken again.
