# Open Questions — M1g

Questions **this milestone raises**, with sticky `Q-M1g.<n>` ids. Do not reuse a closed id.

M1g was planned on 2026-11-11. These are the questions the plan could not decide. None is ruled. Each names
the stage that puts it to the user with real programs, and the plan's default, which is what a stage does
until the user says otherwise (a default is not a ruling, and any may be reopened).

| id | question | needed by | state |
|---|---|---|---|
| [Q-M1g.1](#q-m1g1--the-spelling-of-the-connectives) | the spelling of the connectives | S1g.1.3 | **defaulted 2026-11-11**, on the user's *"whatever"*: capitalised; revisable |
| [Q-M1g.2](#q-m1g2--where-the-logic-ships) | where the logic ships: the stdlib, or `examples/` | S1g.1.3 | open |
| [Q-M1g.3](#q-m1g3--the-two-truth-values) | the two truth values: the bits `0` and `1`, or names of their own | S1g.1.3 | open; waits for S1f.6.17 |
| [Q-M1g.4](#q-m1g4--how-a-program-claims-a-tautology) | how a program claims a tautology | S1g.2.2 | open |
| [Q-M1g.5](#q-m1g5--is-a-refuted-assumption-a-fact) | is a refuted assumption a fact? | S1g.3.3, S1g.6.5 | open; **the user's** |
| [Q-M1g.6](#q-m1g6--the-word-for-a-saturation-that-does-not-stop) | the word for a saturation that does not stop | S1g.6.1 | open; **the user's** |
| [Q-M1g.7](#q-m1g7--a-hypothesis-about-a-nested-fact) | a hypothesis about a nested fact, named in `(solve :hrules …)` | S1g.6.4 | open; a grammar question, **the user's** |
| [Q-M1g.8](#q-m1g8--fresh-names) | fresh names: Skolem terms only, or names the engine makes | S1g.4.4, S1g.6.4 | open; **the user's** |
| [Q-M1g.9](#q-m1g9--the-bound-of-an-exhaustive-check-and-its-tier) | the bound of an exhaustive check, and its tier | S1g.2.1, S1g.3.4 | open |
| [Q-M1g.10](#q-m1g10--the-optional-proof-systems) | the optional proof systems: sequents, resolution, tableaux | S1g.3.5 | open |
| [Q-M1g.11](#q-m1g11--how-far-induction-goes) | how far induction goes: a checked proof, or a bounded check | S1g.5.3 | open; **the user's** |
| [Q-M1g.12](#q-m1g12--where-the-milestones-kernel-pages-go) | where the milestone's kernel pages go | S1g.4.1, S1g.4.5 | open; waits for P1f.20 |
| [Q-M1g.13](#q-m1g13--may-a-finished-search-call-an-unpayable-debt-dead) | may a finished search call an unpayable debt dead? | P1g.7 (S1g.7.3) | open; **the user's**, deferred on 2026-11-12 |
| [Q-M1g.14](#q-m1g14--the-types-theory-its-shape-and-its-name) | the types theory: its shape and its name | P1g.8 (S1g.8.1) | open |
| [Q-M1g.15](#q-m1g15--where-m1h-sits-against-this-milestone) | where M1h sits against this milestone | the coordinator, before either starts | open; **the user's** |

---

## Q-M1g.1 — The spelling of the connectives

`and`, `or`, `eq`, `neq`, `absent` and `false` head no value
([`defined_behaviour.md` § 4.22](../../docs/kernel/defined_behaviour.md)), and `(not F)` is the engine's
stored negation of a fact, so a formula's negation spelt `not` would make `(holds (not p))` mean something to
the engine that the theory did not say. The user, 2026-11-11: *"for "and" element, function can use _and,
_and_, __and, AND, And, l_and, And2 - whatever"*.

- **(a), the default:** capitalised, `And`, `Or`, `Not`, `Imp`, `Iff`, `Top`, `Bot`; quantifiers `All` and
  `Ex` in P1g.4.
  **All sixteen binary connectives, on the user's instruction of 2026-11-11** (*"in preposition logic -
  conjunction, disjunction, implication, pierse arrow - also should be 16, if not - explain why"*): there are
  sixteen, each with a name, and the table with the proposed spellings (`Bot And Nimp Fst Nrimp Snd Xor Or Nor
  Iff Nsnd Rimp Nfst Imp Nand Top`) is in
  [S1f.6.17 § All sixteen functions of two inputs](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md#all-sixteen-functions-of-two-inputs-the-user-2026-11-11).
  S1g.1.3 and S1g.2.1 take all sixteen as connectives of the formula language, with `Not` the one unary;
  S1g.2.3's normal forms say which they rewrite away, and P1g.3's proof systems which they take as primitive.
- **(b)** a prefix, `l-and`, `l-or`: reads as a family, and sorts together.
- **(c)** symbols the name rule allows, if it allows any that read as connectives; to be checked against
  `00_ebnf.md`'s allow-list.

S1f.6.17 chooses the gates' spelling first. If it differs, M1g follows it, so that a gate and a connective
that are one function have one name or two on purpose.

## Q-M1g.2 — Where the logic ships

- **(a)** `stdlib/`, as `std.logic` and beside it: importable, and held by the stdlib's gates (a program per
  rule under `tests/stdlib/`, the mutation sweep).
- **(b)** `examples/logic/`, imported by path: a study to read, with corpus entries and `:expect`s, and no
  promise that another program depends on it.
- **The default: (b) first, (a) for what a second program imports.** The truth tables and the evaluation
  rules are the likely stdlib part, since P1g.3's soundness checks and S1f.6.17's circuits both read them.

## Q-M1g.3 — The two truth values

S1f.6.16 has the digits `0` and `1`, and S1f.6.17's gates are tables over them. A formula's value could be the
same two names, so a formula *is* a circuit and one set of tables serves both, or names of its own (`T`, `F`),
so a truth value is never mistaken for a digit of a number. Probe 1 used `0` and `1`. The default follows
S1f.6.17's ruling.

## Q-M1g.4 — How a program claims a tautology

*True under every valuation* is, in probe 1, *the query has one projection class and its value is 1*. A
query's `:expect (model (value-of F 1))` over all of R says exactly that today, since a claim lists the
classes. The question is whether that reading is plain enough to a reader of the program, or whether the
pages should recommend a second, explicit form: a rule that makes a state in which the formula is 0 dead, with
`(solve :expect (region …))` listing the 2ⁿ valuations. No new syntax is proposed either way. S1g.2.2 shows
both on the same formulas.

## Q-M1g.5 — Is a refuted assumption a fact?

Probe 4: the assumption `(Not p)` is dead alone and N names it, with its core. Root does not hold
`(Not (Not p))`, and no rule can read N. In natural deduction that step, *not-introduction*, is the point of
assuming.

- **(a)** It stays a report. Reductio is read off the read-out, and a theory that needs the conclusion carries
  its contexts as data, `(proves CTX A)`.
- **(b)** A program may ask for it: a declared way for a dead singleton to become a stored negation at root.
  The walk the engine had until M1e2 wrote such facts back and was deleted; what was wrong with that is part
  of the evidence S1g.3.3 collects.
- **(c)** A rule may read the lattice (a premise over *the state in which A is committed*). The largest
  change, and F5's neighbour.

No default: (a) is what stages do until the user rules.

## Q-M1g.6 — The word for a saturation that does not stop

Probe 3 ran until a timeout. [F26](../followups/f26_consistency_and_completeness.md)'s promotion 3 names two
shapes: a bound and an outcome, or a static check that rules cannot grow a term. For a bound: what is bounded
(facts, firings, nesting depth), what the read-out says, and what the exit code is, given the user's ruling
that every answer exits 0 and only an error does not. *Did not stop within the bound* is neither an answer
about R nor an error in the program.

## Q-M1g.7 — A hypothesis about a nested fact

Probe 4b: `(solve :hrules (suppose ((Not p))))` is a parse error. An activator's arguments in `:hrules` are
names (`ein.y`'s `HruleItem`), though an activator *fact* binds a parameter to a nested fact since S1f.5.18.
An hrule that matches a marker fact, `(assumable (Not p))`, does the same work today. Whether the list should
take a value is a grammar change and waits for the user; the default is the marker fact.

## Q-M1g.8 — Fresh names

Ein never makes a name at run time. First-order proof needs something fresh in two places: a witness for an
existential, and the arbitrary individual of a universal's introduction. A **Skolem term**, a nested fact
over the names in scope such as `(father-of Ann)`, is a value today (probe 5). The question is whether that is
enough for everything M1g wants, and S1g.4.4 is where it is found out; names made by the engine would be a
change to what a program's names *are* ([`solution_semantics.md` § 1.2](../../docs/kernel/inference/solution_semantics.md)),
and the M1 alive-set invariant says no rule asserts a name the program lacks.

## Q-M1g.9 — The bound of an exhaustive check, and its tier

*Every formula up to a bound* has to name the bound: a count of connectives, over how many letters. The
number of formulas grows very fast, so the bound decides the test's cost. Which bound each check takes, and
whether the larger sweeps run per commit, nightly or on the dev host (S1f.5.27's three tiers), is each
stage's to propose with the measured cost.

## Q-M1g.10 — The optional proof systems

Sequent calculus, resolution and analytic tableaux each map to Ein differently (a sequent is a fact; a
resolvent is a forward rule over clauses; a tableau's branch is a commitment). S1g.3.5 is marked optional.
The default: run it only for the systems the user picks after reading S1g.3.3's page.

## Q-M1g.11 — How far induction goes

- **(a)** Bounded checking only: `x + y = y + x` for every pair up to a bound, as S1f.6.10's item 4 has it.
- **(b)** A written induction proof checked by Ein: induction as a rule of the proof checker over formulas
  with a variable. It needs S1g.4.3's substitution and S1g.4.4's quantifier rules.
- **(c)** Ein finds the induction.

The plan aims at (b) for one theorem and names (c) as out of reach unless S1g.5.3 shows otherwise.

## Q-M1g.12 — Where the milestone's kernel pages go

The fragment table and the glossary of *consistent* and *complete* are kernel documentation, a current page
each under `docs/kernel/`. [P1f.20](../m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md)
rebuilds that tree as `docs/ein/` before M1g starts. Where the two pages live is decided against the tree as
P1f.20 leaves it.

## Q-M1g.13 — May a finished search call an unpayable debt dead?

Added 2026-11-12, from question 24 of the calculator's
[`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q24) and the user's reply
(*"this question open vs. false is deep. […] It looks like an entire separate phase."*). Where every alive
node owes and no extension pays, R is empty and `--single` says `Open`, which reads as *not yet* where the
run has shown *never, among the candidates*.

- **(a)** Keep the words; print the explanation (N, or the open state's core).
- **(b)** A debt that provably cannot be paid within the candidates is a refutation, where the search has
  finished: the state is called dead.
- **(c)** Leave everything and say on the page how to read it.

R is the same set under each. No default is taken: [P1g.7](p1g.7_unsatisfiable_against_owing/README.md)'s
third stage lists what moves under (b) and the user rules with that list.

## Q-M1g.14 — The types theory: its shape and its name

Added 2026-11-12, from the user's reply to question 12 (*"types theory deserves it's own separate theory
file in stdlib"*). Which relations it has (`has-type`, `subtype`, a coercion), whether a type is read off a
value's shape on request or declared, and what the module is called and where it sits in the stdlib's
layout. Default: [P1g.8](p1g.8_a_theory_of_types/README.md)'s first stage proposes on computed programs and
stops for the user; nothing is named before then.

## Q-M1g.15 — Where M1h sits against this milestone

[M1h](../m1h_beyond_word_problems/README.md) was created on 2026-11-12. This milestone *"runs after M1f is
complete"*; nothing says whether M1h runs before it, after it or beside it. M1h's first phases (temporal
and causal theories, slicing) need nothing of this milestone; its gaps-and-assumptions phase reads P1g.7's
ruling if that exists. No default is taken; it is the same question as M1h's own, asked once.
