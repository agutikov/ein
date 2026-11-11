# S1g.6.4 — Witnesses and fresh names: Skolem terms, hypotheses about nested facts, and what H holds of them

**Phase:** [P1g.6](README.md) — candidates C-8, C-9
**Status:** planned 2026-11-11, not started. Provisional until P1g.1's stop.
**Depends on:** [S1g.4.4](../p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md)'s three witnesses and
[S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s assumptions, which are its evidence.
**For the user:** [Q-M1g.7](../open_questions.md#q-m1g7--a-hypothesis-about-a-nested-fact) and
[Q-M1g.8](../open_questions.md#q-m1g8--fresh-names).

## Why the logic needs it

Two places where a value that is not a name has to go where the engine expects a name.

**C-8: something new.** First-order reasoning introduces individuals: the witness of an existential, the
arbitrary x of a universal's proof. Ein's standing design is that **no rule asserts a name the program
lacks** (the M1 alive-set invariant, `ein-infer/src/invariant.rs`), and the universe H is built from the
program's names ([`solution_semantics.md` § 1.2 and § 1.4](../../../docs/kernel/inference/solution_semantics.md)).
A Skolem term, a nested fact over names in scope, is a new *value* and no new *name*, so it is legal today
(M1g's probe 5: `(parent (father-of Ann) Ann)`).

**C-9: a hypothesis about a nested fact.** An assumption `(Not p)`, a tableau's branch on a subformula, a
guessed line of a proof: each is a candidate whose content is a nested fact. As a candidate *fact*,
`(holds (Not p))` is proposed and committed today (probe 4, through a marker fact and an hrule with no
parameter). What is refused is naming it in the program's own list: `(solve :hrules (suppose ((Not p))))` is
a parse error at the inner parenthesis (probe 4b), because `ein.y`'s `HruleItem` is a name or a list of
names, while an activator *fact* has bound a parameter to a nested fact since S1f.5.18.

## What has to be established first

Before either is called a requirement, five things about today's engine, each a small program and its run
(none has been tried; the coordinator does not know the answers):

1. **What H holds when nested facts are around.** Does the blind enumerator propose candidates *about* a
   nested value (a fact whose argument is `(father-of Ann)`), or only about names? Does a name that occurs
   only inside a nested fact count as an object (S1e2.2.14 says a constant a fact names at any depth is a
   name of the program)? The calculator closes all its relations, so its corpus never asked.
2. **What the obligations rung proposes** when the witness domain of an `(open ?R)` is a relation whose
   members are nested facts: are they candidates?
3. **`outside` and `types`:** do the read-out's two rows about the universe say the right thing of a
   relation whose arguments are terms?
4. **A Skolem term in a core:** does a refutation that passed through `(father-of Ann)` name it readably?
5. **The alive-set invariant's test** on a program that derives nested values: what it checks and whether
   it still means what its name says.

Each answer is *right*, *wrong* (a register row), or *undefined* (a paragraph for the kernel page and, if a
choice is involved, a question).

## C-8: is a Skolem term enough?

The stage's claim to test, from S1g.4.4's evidence: **for everything M1g builds, a nested fact suffices and
no name need be made.** The cases:

- a rule's existential conclusion: a Skolem term over the rule's variables. Enough, with C-1's bound for the
  programs where terms then grow.
- a proof checker's eigenvariable: the proof supplies it. Nothing is made.
- substitution's renaming to avoid capture: a built variable `(prime y)`, or de Bruijn indices
  ([S1g.4.3](../p1g.4_first_order_logic/s1g.4.3_formulas_with_quantifiers.md)).
- Tseitin's new letter for a subformula: the subformula itself as the letter.
- a formula-to-circuit conversion's wire names: the subformula itself as the wire.
- a resolvent's name: the clause as a value, which wants a canonical form and so an order on names
  ([S1g.3.5](../p1g.3_proof_systems/s1g.3.5_other_calculi.md)).

The last is the likeliest not to fit, and what it wants is not a fresh name but an **order on names**, a
computed predicate beside `eq` and `neq`. That would be a new engine predicate: a question for the user if
and only if a stage of the milestone cannot do without it, with the detour (the order written as facts by
the program) measured first.

If the claim holds, C-8 closes as *nothing to build*, and the kernel page says, in one paragraph, that Ein's
fresh individuals are terms. If it fails, the failing program goes to the user with Q-M1g.8, and names made
by the engine are proposed only then: they would change what a program's names are, H, and the invariant.

## C-9: a value in the `:hrules` list

The detour works: a marker fact and an hrule with no parameter. What the list form would add is the
*explicit* lattice, each assumption named in `(solve …)` where a reader looks for the lattice, as the zebra
programs name `(guess (color-loc Color House) …)`. To put to the user, on S1g.3.3's programs written both
ways:

- **(a)** leave it: the marker fact is the idiom, documented;
- **(b)** let an activator's argument in the list be a value, as an activator fact's is. A grammar change
  (`HruleItem`), with § 4.20's check extended (an argument at a value parameter is not checked today: F25's
  R-2), its row in the grammar's table, and the sweep.

No default beyond (a).

## Tasks

1. **T1g.6.4.1 — the five establishing programs,** run; each answer filed as right, wrong or undefined.
2. **T1g.6.4.2 — the Skolem claim,** case by case, from the theory stages' programs; the order-on-names
   detour measured.
3. **T1g.6.4.3 — the two questions for the user,** each with its programs written both ways.
4. **T1g.6.4.4 — what is ruled, built:** nothing; or the grammar change of C-9 with its table row, its
   refusals and its sweep; or what the user rules for C-8.
5. **T1g.6.4.5 — the kernel paragraphs:** what H holds of nested values; that Ein's new individuals are
   terms; `solution_semantics.md` § 1.2 and § 1.4 and `defined_behaviour.md` amended where they are silent.

## Acceptance

- The five establishing questions each have an answer with a program, and every wrong answer among them is a
  register row.
- The Skolem claim is confirmed case by case or the failing case is named.
- Q-M1g.7 and Q-M1g.8 are ruled or explicitly left at their detours by the user.
- Whatever is built moves no existing answer; the grammar pages, if touched, are re-blessed from the grammar
  and never edited by hand.

## What it may find

- **That H is wrong, or surprising, around nested values** on programs that do not close their relations.
  The calculator's E-1 and E-7 are neighbours (a dunder and a relation name proposed as objects). This is
  the likeliest place in the milestone for a new register row.
- **That an order on names is wanted by three stages at once** (canonical normal forms, clauses, contexts
  as sets). Then it is the real requirement behind C-8 and goes to the user as such.
