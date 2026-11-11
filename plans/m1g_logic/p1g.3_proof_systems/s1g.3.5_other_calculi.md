# S1g.3.5 — Optional: sequent calculus, resolution, analytic tableaux

**Phase:** [P1g.3](README.md)
**Status:** planned 2026-11-11, not started. **Optional:** it runs only for the systems the user picks after
reading [S1g.3.3](s1g.3.3_natural_deduction.md)'s page
([Q-M1g.10](../open_questions.md#q-m1g10--the-optional-proof-systems)). Each of its three parts stands alone.
**Depends on:** S1g.3.3; for resolution, [S1g.2.3](../p1g.2_propositional_logic/s1g.2.3_normal_forms.md)'s
clause-set form.

## Why it is here

The instruction names Hilbert systems and natural deduction with an *e.g.*. Three other calculi are worth a
page each because each maps onto a **different part of Ein**, and seeing all five side by side is what
answers *"rules of inference? what are hypothesis in this case?"* in general, not for two examples:

| calculus | its object | maps to | a hypothesis is |
|---|---|---|---|
| Hilbert | a formula | a fact; a schema is a pattern | nothing |
| natural deduction | a formula under assumptions | a fact in a state, or a judgment with its context | an assumption |
| **sequent calculus** | a sequent, Γ ⊢ Δ | one fact, both sides data | nothing: every rule is local to a sequent |
| **resolution** | a clause | a fact; the resolvent is a forward rule's conclusion | nothing; the negated goal is a premise |
| **tableaux** | a branch | a state of the lattice | a branch's choice at a disjunction |

All three parts are **to be tried**; none has a probe.

## The three parts

**1. Sequent calculus.** A sequent is a fact, `(seq LEFT RIGHT)`, both sides nested lists. Gentzen's rules
read backward are goal-directed and, for propositional logic, **invertible**: each replaces a sequent by
simpler ones and no choice loses provability. So backward proof search needs no guess and terminates:
`(want (seq L R))` asks for the premises of the rule for the first compound formula, down to axioms. The
expectation is the milestone's best case for *finding* a proof by saturation alone: a decision procedure for
propositional validity, search-free, written as requests. It needs the list walks of S1g.3.3's contexts
(find a compound formula; find a formula on both sides). To check: its verdict against the truth table on
every formula up to the bound; its cost against [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)'s
2ⁿ states. And the cut rule, which is not invertible and is S1g.3.2's cut formula again.

**2. Resolution.** Clauses as facts, `(clause C)` with `(lit C L)`; or a clause as one value. The rule: two
clauses with a complementary pair give their resolvent. Forward saturation is then exactly resolution to a
fixpoint, and deriving the empty clause is `(false)`: refutation by Ein's own contradiction, with the engine's
core as the set of input clauses used. Two things to work out. A resolvent is a *new* clause: it needs a name
or must be a value, and as a value it must be canonical (a set of literals, sorted, no repeats) or the same
clause is derived many times; a clause as a nested fact has an order, and Ein has no order on names (the wall
S1g.2.3 names). And the closure is exponential in the worst case; the pigeonhole family of
[S1g.2.4](../p1g.2_propositional_logic/s1g.2.4_applications_and_scale.md) is the measure. Unit resolution
alone, the propagation of a SAT solver, is the cheap fragment and is the same shape as Ein's elimination
rules (`std.elim`); with the letters' values as hypotheses on top, the pair is DPLL, and the stage says how
close Ein's search with minimal dead sets is to it, by a count of states visited on the same formulas.

**3. Analytic tableaux.** A tableau refutes a formula by breaking it down along branches; a disjunction
splits a branch, and a branch with F and its negation closes. The mapping onto Ein is direct and is the
reason the part is here: signed formulas as facts, `(holds F)` and `(fails F)`; the non-branching rules as
forward rules; **each branching rule's alternatives as candidates, with an obligation that one is taken**; a
closed branch is a dead state; the formula is valid when R is empty, and an alive, discharged state is an
open branch, a countermodel read off its literals. It is reading 1 of S1g.3.3 without the discharge problem,
because a tableau never discharges: it only refutes. To be tried: writing an obligation whose alternatives
are two *nested-fact* candidates derived from a formula (the generation ladder's middle rung, and
[S1g.6.4](../p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md)'s witness); and whether R's
states, which are sets of commitments and not paths of a tree, count each open branch once.

## Tasks

For each part the user picks:

1. **The theory,** on request or forward as above, with a program per rule.
2. **The check,** a row of the Rust harness: the calculus's verdict against the truth table on every formula
   up to the bound.
3. **The comparison:** on the shared theorems of S1g.3.1, the work each calculus does (firings, states), in
   one table with Hilbert's and natural deduction's.
4. **A section of the phase's page,** and the row of the table above filled from what ran.

## Acceptance

- For each part run: verdicts agree with the truth table up to the bound, held by the Rust check; the
  search-free test holds where the part is claimed search-free; every number has its command.
- For each part not run: the table's row stays, marked *not tried*.

## What it may find

- **That the sequent calculus is the right default** for propositional proof in Ein, in which case P1g.4's
  quantifier rules are better stated over sequents, and S1g.4.4 says so.
- **That tableaux are the cleanest statement of what Ein's search already is.** That would be a sentence for
  [M5](../../m5_presentation/README.md)'s paper and for
  [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)'s readers, and it goes to
  the user as a finding.
