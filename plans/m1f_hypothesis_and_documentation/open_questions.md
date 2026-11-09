# Open Questions — M1f

Questions **this milestone raises**, with sticky `Q-M1f.<n>` ids. Do not reuse
a closed id.

M1f was created on 2026-08-29 by moving two phases out of
[M1e](../../docs/history/m1e_review_processing/README.md). The first questions it raised came from
[S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s paper grammar on 2026-11-08.

| id | question | state |
|---|---|---|
| [Q-M1f.1](#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) | G-14: is a bad conclusion that never fires refused? | **ruled 2026-11-08, (b)**; S1f.5.5 and [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md) |
| [Q-M1f.2](#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) | the 55 files the grammar refuses that today's parser accepts | **ruled 2026-11-08, (a)**; S1f.5.5 |
| [Q-M1f.3](#q-m1f3--three-wrong-answers-the-census-found-with-no-row) | three wrong answers the census found with no row | **ruled 2026-11-08, as recommended**; [S1f.5.0](p1f.5_the_no_owner_items/s1f.5.0_the_known_wrong_answers_filed.md) files W33–W35 |
| [Q-M1f.4](#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs) | removing `:priority` moves the facts of cores on seven corpus programs | **ruled 2026-11-08, (b)**: derive the order from the rules first; [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) |
| [Q-M1f.5](#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core) | the alternatives flag moves `refuted by` beside an unmoved core | **ruled 2026-11-08, (a)**: W36, open, owned by F22; S1f.5.15 files it |
| [Q-M1f.6](#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not) | the derived agenda holds the cores; the FIFO boundary does not | **ruled 2026-11-08, (a)** (re-ruled the same day from (b)): plain FIFO, a core is an explanation, F22 makes it canonical; [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) |
| [Q-M1f.7](#q-m1f7--where-may-a-program-write-the-kernels-dunders) | where may a program write the kernel's dunders? | **ruled 2026-11-08, (b)**: never bound; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T11 |
| [Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it) | the parser's remaining cost: within grmtools, or past it? | **defaulted 2026-11-09 (the user's instruction): within grmtools; alternatives parked for the close**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T12 |
| [Q-M1f.9](#q-m1f9--what-a-deleted-loader-line-leaves-behind) | what a deleted loader line leaves behind | **defaulted 2026-11-09 (the user's instruction): an `expect`, `unreachable!` or `assert!` naming the grammar's production; a hand-built `Ast` the grammar would refuse panics**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T5 |
| [Q-M1f.10](#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone) | the `.einb` `INTS` section, once the int kind is gone | **defaulted 2026-11-09 (the user's instruction): the section stays, written empty, and a file that holds an int is refused; no format bump**; [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.3 |
| [Q-M1f.11](#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name) | a template reference to a variable no reference can name | **defaulted 2026-11-09 (the user's instruction): the loader refuses it, one line a reference**; [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.5 |
| [Q-M1f.12](#q-m1f12--which-surfaces-carry-natural-language-text) | which surfaces carry natural-language text | **defaulted 2026-11-09 (the user's instruction): those that carried it before the swap, each held byte for byte; none added**; [S1f.5.2](p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)'s T1f.5.2.2 |

**The questions those two phases already own are M1e's and keep their ids.**
They are not copied here and they are not renumbered — the stage files cite
them across the milestone boundary, which is what a sticky id is for:

| id | what it is | who owns it here |
|---|---|---|
| [Q-M1e.5](../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | is *experimental* a licence to ship a surface whose read-out is false? | [S1f.10.7](p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md)'s `--traversal` flag waits on it |
| [Q-M1e.6](../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model) | what is a **solution**, and what is a **model**? | decided 2026-08-28; [P1f.10](p1f.10_hypothesis_structure/README.md) is its first consumer |
| [Q-M1e.9](../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent) | is `dead` really upward-closed under `absent`? | the **language** half is [S1f.10.8](p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)'s |
| [Q-M1e.10](../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert) | two `(config …)` flags are **inert** | raised by the shipped `S1e.5.1`; owner unassigned |
| [Q-M1e.11](../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) | what happens to an obligation **derived under a hypothesis**? | **owned** by [S1f.10.6](p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) |

A question raised by an M1f stage from now on gets a `Q-M1f.<n>` here.

---

### Q-M1f.1 — G-14: is a bad conclusion that never fires refused?

**Raised** 2026-11-08 by S1f.5.5's step 2
([paper grammar, Q1](p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md#q1--g-14-with-assert-checked-by-the-grammar-is-a-bad-conclusion-that-never-fires-refused)).
With `:assert` checked by the grammar, a conclusion no fact can hold (`(q ?x :k v)`, `(q _)`, `?x`) is a
parse error whether or not its rule fires. One shape is left that no grammar sees: an activator a rule
*derives* binding a relation parameter to a fact (`g14/derived-activator.ein`), which today stops the run
where its plan is compiled, and answers (64 states) where no node derives it
(`g14/derived-activator-never.ein`). The options were (a) refused at the parse, the derived shape a refusal
where compiled; (b) as (a), and the derived shape refused at load by a static check, which also refuses
`derived-activator-never.ein`; (c) checked only when a rule fires. The agent recommended (a).

**Ruled 2026-11-08: (b).** The user: *"Q1 - b"*. The grammar's half is S1f.5.5's; the load-time check is
[S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s, which stops and shows the
user any corpus program it would refuse.

### Q-M1f.2 — The 55 files the grammar refuses that today's parser accepts

**Raised** 2026-11-08 by S1f.5.5's step 2
([paper grammar, Q2](p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)),
re-taking the stage's 34. None is a puzzle, a stdlib module or a `tests/` program. The agent proposed (a):
the 26 refusal fixtures whose refusal the grammar takes over stay refusal fixtures and move to
`examples/broken/`, except `reserved_open_direct`, which three fixtures import by path;
`derivation_cycle` and five `examples/syntax/` fixtures are rewritten; 17 register witnesses and 6
regression fixtures are the owning stages' (S1f.5.1, S1f.5.2, S1f.5.4, S1f.5.11). The alternatives were (b)
the 26 left in place with only their `.expected` moved, and (c) the 26 rewritten.

**Ruled 2026-11-08: (a).** The user: *"Q2 - a"*. Since the user's re-plan the same day, the 23 the owning stages
had land in S1f.5.5's swap, each credited to its stage's ruling, and of the rest what moves no answer lands just
before it ([S1f.5.5 § The order](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-order-re-planned-2026-11-08)).

### Q-M1f.3 — Three wrong answers the census found with no row

**Raised** 2026-11-08 by S1f.5.5's step 1
([census § Findings outside the grammar](p1f.5_the_no_owner_items/s1f.5.5_census.md#findings-outside-the-grammar)):
an activator binding a value parameter to a nested fact leaves it unbound; `:assert ()` stores a fact
named `@empty`, which joins H; and `(absent ())` holds. The agent recommended filing all three now as
`open` rows, W33–W35, as a fourth task of S1f.5.0: W33 owned by a new stage, S1f.5.18, where the parameter
binds as any value does; W34 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md), whose
case table it corrects; W35 by S1f.5.5, whose grammar refuses `(absent ())`.

**Ruled 2026-11-08, as recommended.** The user: *"Q3 - as recommended"*.

### Q-M1f.4 — Removing `:priority` moves the facts of cores on seven corpus programs

**Raised** 2026-11-08 by [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s T1
([§ T1 record](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t1-record-2026-11-08)), which stopped on it
before any removal. Taking every `:priority` out of the corpus and the stdlib (`without_priority`, 475 clauses in
136 files) moves **no** R, N, H, claim, exit code or stored fact on any of the 511 programs under the five roots,
nor on any of the corpus's 1 378 declared runs. It moves **the facts of cores**: on `zebra.ein` 292 of N's 458
(13 193 → 14 697 facts over N), on the zebra2 family 24 to 215 a program (+0.4 % to +3.7 % of N's facts), and on
W25's control, `guard-only-first.ein`, whose core `(p a)`, the row's right answer, becomes the witness's wrong
`(b a) (p a) (seed a)`. Each move goes through an order-dependency the store already has, each owned: the
alternatives cap (the zebra2 family; at 10⁶ the two arms agree; Q-M1e.15, F22), `:record-alternative-
justifications false`, which `zebra.ein` sets (the config principle's one exemption, M1e2 S1e2.2.11), and W25
(F22). The typed number decides the order those three read; without it the written order does. The same
stripped corpus also takes away the `alt-cap-core` pair's difference (they are one `:priority` apart) and shows
[Q-M1f.5](#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core)'s breach on a corpus program.

- **(a) Remove it now, and accept the cores' moves.** The agenda is FIFO within one priority, the written order.
  W25's control is restated as a program whose `w` derivation arrives first by its rules, not by a number;
  the `alt-cap-core` pair's tests and the flag's exemption take a witness that differs by its written order;
  and every moved core is named before the commit that moves it, as F22's to make order-free.
- **(b) Derive the order first, then remove.** T3 builds the rule graph and a band per rule from its shape; the
  closure's agenda is ordered by that band where the number orders it today, and the boundary stays FIFO, since
  admitting by stratum is F21's D2. The sweep is re-taken with it before any removal: if the cores hold, the
  removal moves none, and what still moves comes back to the user as in (a). It takes the user's note of
  2026-08-28 literally, *"remove `:priority` completely / analyze rules structure instead"*.
- **(c) Keep `:priority` until F22 makes a core a function of the program**, the written refusal the stage's
  acceptance allows: the production comes back to the generated grammar before S1f.5.5's T9, S1f.5.4's
  `:priority` guard with it, and the graph, the strata and the diagnostic (T3, T4) ship either way.

**The agent recommends (b)**, and (a) for whatever (b) leaves. No order is semantic, but the bands the authors
typed buy smaller cores on the puzzles, and a band derived from the rules is the replacement this stage was
written to find. (b) moves no answer of its own, since no order moved R, N, H or a claim on the corpus, and its
measurement says whether (a)'s cost remains.

**Ruled 2026-11-08, (b).** The user chose *"Derive order first"*: T3's band, derived from each rule's shape,
orders the agenda where the number does today, the boundary FIFO; the sweep is re-taken before any removal, and
what still moves comes back to the user.

### Q-M1f.5 — The alternatives flag moves `refuted by` beside an unmoved core

**Raised** 2026-11-08 by the same T1, whose snapshot of the corpus without `:priority` failed `lever_invariance`.
`examples/features/12_expect_false.ein` written without its `:priority` prints `refuted by no-room` and, with
`(config :record-alternative-justifications false)`, `refuted by forced, occupied`, beside the same five-fact
core (`ein solve`, today's binary, 2026-11-08). The flag's exemption is the core (the user on Q-M1e2.11 (ii),
*"(b) Exempt the core"*, which `configuration.md` § 2.5 words *"and nothing else"*), and `refuted by` may follow
it only where the core moved (Q-M1e2.21 (a), argued from *"where the core is the same the primaries are a
derivation from it"*). Here one
core derives the contradiction through two rules, and the flag decides which one the store keeps. **It is a
wrong answer today**, on a program anyone can write; the typed order is what keeps the corpus from showing it,
and if `:priority` goes (Q-M1f.4 (a) or (b)) `features/12` shows it and the gate fails.

- **(a) File it now as an `open` register row, W36**, its witness `features/12` without `:priority` under
  `examples/ein-bugs/wrong-answers/`, owned by F22, whose search re-derives what the store does not keep, so
  that `refuted by` no longer depends on it. `lever_invariance` exempts the row's program until then.
- **(b) Widen the exemption to the core and the rules `refuted by` names**, beside or without a moved core:
  `refuted by` is the core's explanation, and the flag decides what the store keeps for explanations.
  `configuration.md` § 2.5 and `lever_invariance` say so; no row.

**The agent recommends (a)**: the user exempted the core, not its explanation, and a wrong answer is a row
before it is fixed.

**Ruled 2026-11-08, (a).** The user chose *"Register row W36"*: `open`, owned by F22, its witness
`features/12` without `:priority` under `examples/ein-bugs/wrong-answers/`, and `lever_invariance` exempting that
program until F22 fixes it. S1f.5.15 files it before the removal: filed the same day by T1f.5.15.7 as
`W36/alt-flag-refuted-by`, the witness `examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein` with the flag set
in the file, under the register's schema `/14`.

### Q-M1f.6 — The derived agenda holds the cores; the FIFO boundary does not

**Raised** 2026-11-08 by [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s T3
([§ T3 record](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t3-record-2026-11-08)), on Q-M1f.4 (b)'s
instruction that whatever still moves comes back to the user. T3 derives a band per rule from its shape and, in a
`derived-order` build, orders the closure's agenda by it with the boundary FIFO, as ruled. T1's sweep re-taken
with it (that build on the corpus without `:priority`, against today's binary on the corpus as written) moves no
R, N, H, claim, exit code or stored fact on the 512 programs or the 1 380 declared runs, and moves **the facts of
cores** on the same programs T1 found: `zebra.ein` 292 of N's 458 (13 193 → 14 699 facts), the zebra2 family 24 to
212 a program, and W25's control; plus `W29/core-two-ways`' witness, 4 facts → 2, its twin's, and three records;
and `refuted by` beside an unmoved core on the zebra family and `zebra2-bad`.

Measured one part at a time on builds of T3's commit: **the derived agenda with the declared boundary moves no core
of `zebra.ein`, the zebra2 family or `zebra2-bad`**; the declared agenda with a FIFO boundary moves exactly what the
derived arm moves. So the typed numbers' work that the shape does not reproduce is at the **boundary**, where the
candidates of `absent`-reading rules (`domain-elimination`, the `slot-*` exclusions and prunes) are admitted one a
round. On the zebra2 family that order chooses which 32 derivations the alternatives cap keeps (uncapped, no core
moves, Q-M1e.15 and F22's); on `zebra.ein` it is the first derivation, `:record-alternative-justifications false`
(the flag's exemption). The agenda moves the rest, W25's control (`early` fires before `w`, all three rules
`propagate`) and W29's witness (`s` before `m1`), whichever the boundary.

- **(a) Remove it as ruled, the boundary FIFO, and accept the moves**: every move above is named in the removal
  commit as F22's to make order-free; W25's control and W29's `now` are restated; the `alt-cap-core` tests and the
  flag's exemption take a witness that differs by its written order; `features/12` joins W36's exemption.
- **(b) Order the boundary by the derived band as well, then remove.** The band is the agenda's key, not a stratum,
  and the boundary is ordered by a key today (`:priority`). Measured over the whole corpus: the zebra2 family and
  `zebra2-bad` hold every core and `refuted by`; 35 of `zebra.ein`'s 458 cores move (through the flag's first
  derivation), every one smaller or the same size (13 193 → 13 136 facts over N); W36's witness prints its right
  answer, `refuted by no-room`; and the agenda's moves of (a) remain. What is left is restated as in (a). On a program with a negative cycle the band picks the
  admission order where `:priority` does today, which is F21's D2's ground.
- **(c) Keep `:priority` until F22 makes a core a function of the program**, the written refusal the stage allows:
  the production comes back to the generated grammar before S1f.5.5's T9, S1f.5.4's guard with it, and the graph,
  the strata, the band read-out and T4's diagnostic ship.

**The agent recommends (b)**, then (a) for the 35 and the agenda's moves. It keeps the discipline the typed numbers
gave the boundary with a key derived from the rules, holds every core the cap decides, and leaves only the moves the
order-dependencies the store already has decide (`zebra.ein`'s flag, W25, W29), each owned. It does take an order at
the boundary, which the ruling of Q-M1f.4 left FIFO on purpose: if that is D2's to decide, (a) is the ruling's own
consequence and (c) the refusal.

**Ruled 2026-11-08, (b).** The user chose *"Order boundary by band"*: the boundary admits its candidates in
derived-band order, as the agenda does, and `:priority` is removed; what still moves (`zebra.ein`'s cores through
the flag, W25's control, W29's witness, the records) is named in the removal commit as F22's, each witness restated.

**Re-ruled 2026-11-08, (a).** Asked what `:priority` is for, the user reasoned that it orders saturation and is in
effect an optimisation, so that a removal needs only the saturation results to agree, not the numbers reproduced
(*"it is not about particular values of priorities, it is about order of rules during saturation. What if
completely remove priority and don't make ordering of rules at all, what changes?"*). Told that T1's sweep is that
experiment (no stored fact, R, N, H, claim or exit code moved; cores, `refuted by`, narration and cost did), that
no order and no band makes a core a function of the program, and that the choice is whether a core is part of the
answer, the user ruled: *"plain FIFO, core is an explanation, F22 makes it canonical"*. So the agenda and the
boundary are FIFO, the written order, and no band orders either; Q-M1f.4 (b)'s derived agenda is withdrawn with
this (b). Every core and `refuted by` the removal moves is named in its commit as F22's; each stays a valid core
(`ein-infer/tests/corpus_cores.rs`). T3's rule graph and strata stay for T4's diagnostic; nothing orders by them.

### Q-M1f.7 — Where may a program write the kernel's dunders?

**Raised** 2026-11-08 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T11
([§ T11 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t11-record-2026-11-08)), before any edit. The user
ruled at the T9 review stop that the engine's five names *"are also reserved names"*, and the coordinator read it,
for the user to correct: the three writeback names `<forced-positive>`, `<monotonic-unconditional>` and
`<lookahead-dies-immediately>` are refused wherever a program writes them, and the dunders `__closed__` and
`__symmetric__` keep the positions the corpus writes them in, the relation of a fact, of a pattern and of an
`:assert`, and are refused as a declared relation, a rule's name, or **an argument value**. The writeback half
meets nothing. The dunder half meets two things the reading did not see:

- **The stdlib documents a dunder as an argument value, and a test holds it.** `stdlib/algebra.ein`'s `imply1`,
  *"Doubles as property→property: (imply1 functional __closed__) turns a functional relation closed"*;
  [`07_stdlib_api.md`](../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md)'s `imply1` row, *e.g. `(imply1
  functional __closed__)`*; and `ein-infer/tests/algebra_semantics.rs`'
  `imply1_lifts_a_property_marker_onto_a_kernel_trigger`, which holds that the program derives `(__closed__
  foo)`, *"userspace reaching a kernel switch"*. Today's binary and T9's generated build both derive `(__closed__
  foo :rule imply1)` from it (`ein saturate --dump`, `build/s1f.5.5/t11/imply1.ein`, 2026-11-08). No corpus
  program writes it outside that comment (`grep` over `examples/`, `tests/`, `stdlib/`, `corpus/`, 2026-11-08), so
  no corpus answer moves either way.
- **The stage file's test cannot hold under the reading.** T11 asks for T9's probe refused at each of its five
  names; the probe writes its dunders as `(__symmetric__ s)` and `(__closed__ hit)`, the relation of a fact, which
  the reading keeps legal. Under the reading the probe is refused at its three writeback names only.

Where each position stands (`grep` of every `.ein` under the five roots, the Rust suites' inline programs, and
two probes on today's binary and T9's generated build, 2026-11-08):

| position | written by | today, both builds | the reading | (b) |
|---|---|---|---|---|
| the relation of a fact, `(__closed__ is-a)` | `branching/12`, `features/06` (three) | loads | legal | legal |
| the relation of a premise, `:match (__closed__ ?R)` | `tests/stdlib/closure/02`, `/03` | loads | legal | legal |
| the relation of a conclusion, `:assert (__closed__ ?R)` | `stdlib/closure.ein` | loads | legal | legal |
| an argument value, `(imply1 functional __closed__)` | `stdlib/algebra.ein`'s comment, `07_stdlib_api.md`, `algebra_semantics`; `(a __closed__)` in `ir_semantics`' and `grammar_decisions`' parse tables | loads, derives `(__closed__ foo)` | refused | legal |
| a declared relation, `(relation __closed__ T)` | none | loads, and declares it | refused | refused |
| a rule's or hrule's name, `(rule __symmetric__ () …)` | none | loads, and its firings print `:rule __symmetric__`, the native mirror's provenance | refused | refused |
| a macro's name, an import's `:as` | none | loads | not stated | refused |
| a query's goal, a claim's fact, a nested fact's relation | none | loads | not stated (a pattern's or a fact's relation?) | legal |
| a `:hypothesis-relations` or `:no-hypothesis` list; a fact's `:rule __symmetric__` | none | loads | not stated (an argument?) | legal |

- **(a) The reading as written.** A dunder is legal only as a relation: of a fact, a premise, a conclusion, and,
  read the same way, of a goal, a claim's fact and a nested fact; refused everywhere else, an argument value and a
  name list included. `imply1`'s documented use is refused at load: the comment and the API row go, and
  `algebra_semantics`' test is restated to the refusal (a program that wants its functional relations closed
  writes the rule, `:match (functional ?R) :assert (__closed__ ?R)`). The refusal of an argument is a loader line
  (the grammar's `Name` is every name), which both parsers share.
- **(b) Never bound.** A dunder is refused where a program **binds** a name, the positions where the loader already
  refuses the kernel's `RESERVED` names (*shadows a reserved kernel name*): a declared relation, a rule's, an
  hrule's or a macro's name, and an import's `:as`. It is legal wherever a name is **used**, as a relation or as a
  value, which keeps `(imply1 functional __closed__)`. One loader check, `is_reserved`'s, shared by both parsers.
  T11's test is T9's probe refused at its three writeback names, and a probe binding each dunder in each of those
  positions refused at each.
- **(c) Never bound, and a value only where it names a relation**: (b), and a dunder as an argument refused unless
  the argument binds a rule's relation parameter, as `imply1`'s `?R2` does. A check on activators, not on
  positions, for a case no program writes.

**The agent recommends (b).** *Reserved* is written *"only with its fixed meaning, never redefined"* (§ The review
stop ruled), and only a binding position can give a dunder another meaning; a value names the kernel's trigger,
which is how `imply1` hands it to a rule that asserts it, a stdlib idiom that works on both builds today. Under
every option the three writeback names are refused wherever a program writes them: in the generated grammar as a
token of their own that no production takes, with its own text, and today at the parse already, `<` being no name
character in the hand-written lexer.

**Ruled 2026-11-08: (b).** The user chose *"Never bound (b)"*. A dunder is refused where a program binds a
name: a declared relation, a rule's, an hrule's or a macro's name, and an import's `:as`. It is legal wherever a
name is used, as a relation or as a value, so `(imply1 functional __closed__)`, its comment, its API row and
`algebra_semantics`' test stay as they are. The three writeback names are refused wherever a program writes
them. T11 builds it.

### Q-M1f.8 — The parser's remaining cost: within grmtools, or past it?

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T12
([§ T12 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t12-record-2026-11-09)), which recovered what it
could of the swap's cost inside grmtools: one rule for whitespace and comments and a start condition in `ein.l`,
and a lexer per thread in `grammar.rs`, which halve the lexing and move no token. What remains of the cost is the
shape of lrlex itself: it runs the regex of every rule its start condition allows at every position (55 rules
where a token starts), and it compiles every rule's regex once a process. Recovering more needs one of two things
the user's rulings rule out today, so the brief would have stopped here:

- **(a) Within grmtools**, as T12 leaves it: the lexer generated from `ein.l` by lrlex as published, `lrpar`
  patched only for F11.
- **(b) A second vendored patch, to `lrlex`**: its loop tries only the rules whose regex can start with the byte
  at the cursor, each rule's first bytes read off its DFA when the lexer is built. Measured by a probe of that loop
  over T4's rules, the 414 corpus files lex in 11.1 ms a pass, against lrlex's 108.3 ms on T4's `ein.l` and
  57.1 ms on T12's, the tokens the same on every file (`lexbench dispatch`, pinned to one P-core, 2026-11-09); it
  recovers lexing, not the 55 regexes a process compiles.
- **(c) A hand-written lexer feeding `lrpar`**, which the user's *"if speed will ever become an issue - will add
  super-optimized hand-written parser"* (2026-11-07) anticipated for the whole parser. The old hand-written parser
  is the measure of what it buys: `saturate` of `01_copiers.ein` 1.86 ms against T12's 4.77, of `zebra.ein` 4.67
  against 9.02 (pinned to one P-core, 50 runs each, 2026-11-09).

**Defaulted 2026-11-09 (the user's instruction): within grmtools; alternatives parked for the close.** The user,
2026-11-09: *"just continue with default decisions till the end of the phase"*. So (a) stands, and (b) and (c)
are parked for the phase's close with the numbers above ([§ T12 record, Parked for the phase
close](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#parked-for-the-phase-close)).

### Q-M1f.9 — What a deleted loader line leaves behind

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T5
([§ T5 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t5-record-2026-11-09)), which deleted the 55 refusal
lines the grammar covers since the swap, each shown unreachable from any program the parser accepts. Where a line
was an arm of a match on the tree's shape, something has to stand in its place, because `ein_ir::load` is public
API (`docs/api/rust.md`) and an `Ast` can be built by hand, past the grammar:

- **(a) An `expect`, `unreachable!` or `assert!` naming the production of `ein.y` that states the shape**: a
  hand-built `Ast` the grammar would refuse panics with the grammar's name for what was wanted, and the readers that
  can no longer fail say so in their types (`expect::parse`, `expect::fact`, `config_from_kw_pairs`).
- **(b) A single internal load error** in place of each, *a shape the grammar refuses*, keeping every reader's
  `Result`.
- **(c) Keep the lines**, as `examples/broken/load/README.md` argued for six of them (*defence in depth for a
  caller that hand-builds an AST*), which the stage's instruction rules out: *each loader line the grammar now covers
  goes*.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default
decisions till the end of the phase"*. (a) is the agent's recommendation: no program reaches any of them (940 tests
and 240 000 mutants with a panic at each, the record), a panic is louder than a load error for the one caller that
can, and (b) would keep 55 error arms for a shape nothing builds. Re-opening it means (b), at any later stage.

### Q-M1f.10 — The `.einb` `INTS` section, once the int kind is gone

**Raised** 2026-11-09 by [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.3
([§ T1f.5.4.3 record](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#t1f543-record-2026-11-09)), which retired
`Value`'s int tag, the int pool and its id space. A `.einb` 1.2 file carries an `INTS` section (kind 3, a string
table, required by every 1.x reader) and stores a value as its 32 bits, the int's tag `0b01`. The stage file planned
the section retired *with its format version bumped*, and the brief asked that the format's rule be followed if the
layout changes and that an old file holding an int be refused cleanly, never misread. design/10 §4: the major bumps
on any layout change, and a minor bump only adds a section.

- **(a) Keep the section in the layout, written empty, and refuse a file that holds an int**: an `INTS` entry or a
  value tagged `0b01`, with its own sentence, `EinbError::Int`. Nothing about the layout changes, so no version
  moves: a 1.x reader opens what this writer writes (the empty table is the bytes the empty pool wrote), this reader
  opens what a 1.x writer wrote for a program with no int, and a file from a program with one is refused.
- **(b) Drop the section and bump the major to 2.0**: every 1.x file is refused with the version line, a file with
  no int among them; `ein --version`'s `container einb/1.2`, `docs/install.md`'s transcript and the reader's two
  `head.minor` checks move with it.
- **(c) Drop the section under a minor bump, 1.3**: a 1.2 reader refuses a 1.3 file for its missing `INTS`, which
  breaks the rule that a minor bump is additive.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default
decisions till the end of the phase"*. (a) is the agent's recommendation: the plan's bump assumed the section had to
go, and (a) keeps its bytes so that nothing needs one; it moves no golden and no output (`ein-einb/tests/invalidation.rs`'
`a_file_that_holds_an_integer_is_refused` builds both shapes of an old file and holds the refusal), and the next
major bump, whenever one is made, can drop the section. Re-opening it means (b).

### Q-M1f.11 — A template reference to a variable no reference can name

**Raised** 2026-11-08 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T9 review (§ 11) and carried
by [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.5, the fix of W37
([§ T1f.5.4.5 record](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#t1f545-record-2026-11-09)). A template
reference reads a variable's name up to the first `}`, and `}` is one of the name rule's characters (the user,
2026-11-07, *for now*), so a variable holding one, `?a}b`, cannot be named: `{?a}b}` is the reference `{?a}` and the
text `b}`. No corpus program writes such a variable.

- **(a) The loader refuses a template that writes a reference to a variable in its scope holding `}`**: a rule's,
  an hrule's or a macro's `:why` and a query's `:goal-text`, one line a reference naming the form and the clause.
- **(b) Leave it**: the reference reads `{?a}`, unbound or bound to another variable, and the sentence prints with
  `b}` after it, which its author did not write.
- **(c) Take `}` out of the name rule**, which re-opens the user's ruling on what a name is.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is what S1f.5.5's T9 review recommended and S1f.5.4's task wrote down: a refusal at
load says so where (b) prints a wrong sentence in silence, and (c) is the user's to rule. It moves no corpus answer.
Re-opening it means (b), or (c) if the name rule changes.

### Q-M1f.12 — Which surfaces carry natural-language text

**Raised** 2026-11-09 by [S1f.5.2](p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)'s T1f.5.2.2,
from S1f.5.5's T9 review (its § *Unicode on every surface that carries it*), which said *which surfaces should carry
the text, and the test that holds each byte for byte, is S1f.5.2's T1f.5.2.2*. A string is natural-language text,
written only after `:why`, `:source` and `:goal-text` (Q-M1e2.23). Before and since the swap, on the T9 probe:
the read-out and `answer.txt` print a query's `:goal-text` filled; `--trace` a rule's `:why` filled at each firing
and a premise's `:source`; `ein saturate --dump` each `:source` and each `:goal-text`; a `--dump-answer` state file
all three, the program re-written; `ein render program` a `:goal-text`; and a `.einb` all three. `ein render rules`,
`kb`, `constraints` and `lattice`, `--events`, `summary.json` and `answer.json` carry none.

- **(a) The surfaces that carry it today, each held byte for byte, and none added.**
- **(b) Add the text where a reader might want it**: a relation's `:why` in the read-out, a rule's `:why` in `ein
  render rules`, a `:goal-text` sentence in `summary.json`, the event stream. Each is a new surface and a schema or
  golden move.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) moves nothing a user sees, which the stage's acceptance asks of it, and the ruling
asks that a program *can* emit Unicode text, which (a) holds on every surface that emits any;
`ein-cli/tests/natural_language_text.rs` holds it. Re-opening it means (b), surface by surface; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md),
which makes `:why` required, is where a reader of the text is most likely to ask for one.
