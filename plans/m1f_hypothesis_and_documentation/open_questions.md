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
program until F22 fixes it. S1f.5.15 files it before the removal.
