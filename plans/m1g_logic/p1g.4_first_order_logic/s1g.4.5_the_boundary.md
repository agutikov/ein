# S1g.4.5 — The boundary: what Ein decides, what it only semi-decides, and what it cannot state; the glossary of *consistent* and *complete*

**Phase:** [P1g.4](README.md)
**Status:** planned 2026-11-11, not started. A page, with a program per row.
**Depends on:** [S1g.4.1](s1g.4.1_the_fragment.md) to [S1g.4.4](s1g.4.4_quantifier_rules.md).
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.4;
[F26](../../followups/f26_consistency_and_completeness.md)'s promotions 1 (the glossary), 2 (the
decidability table) and 4 (the chase reading), and its sketch of a map, row by row;
[F8](../../followups/f8_FCA_RCA_odis_tptp/ideas.md) for TPTP;
[F21](../../followups/f21_non_monotone_programs_decided.md) for programs with no stable reading.

## What it asks

PFL.4: *"identify the first-order fragment ein cannot decide; document the boundary"*, and F1b's note that
since M3 was dropped *"the boundary is no longer where a solver takes over but simply where Ein stops, which
makes documenting it more load-bearing, not less."* The instruction's *"what is required"* is the same
question from the other side. The stage draws the line, with a program on each side of it.

## The decidability table

By the shape of a program, what `ein solve` can promise. The rows are the coordinator's sketch from F26; each
is **to be established**, with a witness program, and corrected where the witness disagrees:

| a program whose rules… | the facts it can derive | consistency of a commitment | R | witness |
|---|---|---|---|---|
| conclude only facts over the program's names (no conclusion nests deeper than its premises) | finitely many | decided: saturation stops | decided by a finite search over H, up to the region limit and budget | any zebra program |
| build terms, on demand only (every constructor application answers a request that shrinks) | finitely many for each request | decided, if the demand is well-founded: the theory's own argument, not the engine's | decided | the calculator; P1g.2's evaluation |
| build terms without bound | unboundedly many | semi-decided at best: a contradiction is found if there is one and the agenda is fair; otherwise the run does not end | not computed | probe 3; a Skolem father who is a person |
| read `absent` over a relation a commitment can change | finitely many, but the state depends on the order | defined only once F21's decisions are taken; refused where a refutation rests on it | flagged, described by the cube | F21's witnesses |

Three questions the table raises, each answered on the page:

- **Is the second row checkable?** A static test that every rule's conclusion is no deeper than its
  premises accepts row one and rejects the calculator. A test for *well-founded demand* is a termination
  analysis. What is cheap to check, and what [S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md)
  should therefore build (a bound and a word, rather than a proof of termination), is this stage's input to
  that one.
- **Is the agenda fair?** Semi-decidability needs every enabled firing to happen eventually. The agenda is
  FIFO since S1f.5.15, which suggests it is. To be confirmed on a program with one diverging rule and one
  contradiction that a starved agenda would never reach.
- **What is undecidable here is consistency itself** (F26): Horn clauses with function symbols compute
  anything computable, so *does root derive `(false)`* is the halting problem. The page shows it the direct
  way, with a program: a small Turing machine or a counter machine as Ein rules over nested facts, halting
  wired to `(false)`. One machine that halts, with the trace, and one that does not.

## What Ein cannot state

Separately from what it cannot decide. Each with the nearest thing a program can write:

- a **disjunctive fact**, and an **existential fact** with no witness (S1g.4.1): stated only as a lattice;
- a **universal fact**: *all numbers…* is a rule, never a member of a state, so it cannot be a premise,
  queried, or refuted by a counterexample the engine looks for ([S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md));
- a **nested implication** or a rule as a premise of a rule (hereditary Harrop formulas; F5);
- **equality as a theory:** two names for one thing (S1g.4.4);
- a statement about a **rule**, or about **provability** ([S1g.6.5](../p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md));
- a statement about **another state** of the lattice (S1g.3.3's discharge).

## The glossary, F26's promotion 1

One page: each of Ein's words, **alive, dead, owes, discharged, exhausted, closed, outside, monotone**,
beside the logical notion it is and the ones it is not, with a program for each. It is F26's first table
made into a kernel page, with F26's *sketch of the map* either confirmed row by row or corrected:

| F26's row | to establish |
|---|---|
| a theory is inconsistent ↔ root is dead | and what a dead state *contains*: nothing, per the read-out; the facts before the contradiction, per `--trace`. Stated, with the division-by-zero program as the example |
| an existential lacks a witness ↔ a state owes | **the chase reading (promotion 4):** is the obligations rung a chase step restricted to the existing names (so: finite model reasoning, always terminating, incomplete for the unrestricted chase)? Checked against the definitions of the restricted and the oblivious chase and the standard termination conditions (weak acyclicity), which S1g.4.4's two readings of a witness, Skolem term and obligation, correspond to on the coordinator's reading. **Not established;** if it stays so, the page says so |
| a theory is incomplete ↔ R has more than one state | on S1g.2.2's contingent formula and a zebra program with a clue removed: each differing fact is an independent sentence |
| categoricity ↔ k_s = 1 | and what plays isomorphism: two states that differ by renaming are two states. One program with a symmetric pair shows it |
| the model is not the world ↔ `outside`, H | the read-out's `outside` and `types` rows, as they are |

And the **three senses of complete** F26 separates (a state, a search, a rule set) beside logic's (a proof
system, a theory, a closed world), with [S1g.3.4](../p1g.3_proof_systems/s1g.3.4_soundness_and_completeness.md)
as the worked case of the rule-set sense.

## TPTP

Not run here ([M10](../../m10_external_benchmarks/README.md) runs external corpora). The stage reads TPTP's
problem classes (CNF and FOF; Horn and non-Horn; with and without equality; unit equality; effectively
propositional) against the fragment and says, for each class, *statable directly*, *statable by a detour
this milestone built*, or *not statable*, with one problem a class translated by hand and run where it is
small enough. The effectively propositional class (no function symbols: finite Herbrand universe) is
expected to be Ein's home ground, and the page says whether a zebra puzzle is in it.

## Tasks

1. **T1g.4.5.1 — the decidability table,** a witness per row; the fairness program; the machine.
2. **T1g.4.5.2 — the cannot-state list,** each with its nearest program.
3. **T1g.4.5.3 — the glossary page,** a current kernel page
   ([Q-M1g.12](../open_questions.md#q-m1g12--where-the-milestones-kernel-pages-go)); F26's file gets a line
   per promotion saying what was done.
4. **T1g.4.5.4 — the chase reading,** established or marked not established with what was read.
5. **T1g.4.5.5 — the TPTP classes,** a table and a handful of translated problems, handed to M10.
6. **T1g.4.5.6 — F1b closed:** its banner updated to *promoted and answered*, each PFL with its page.

## Acceptance

- Every row of the decidability table has a program that shows it, and every program that may not stop is
  run under a bound that fails loudly (or under S1g.6.1's word, once built).
- The glossary gives each of Ein's words exactly one logical reading or says it has none, each with a
  program; it passes the kernel pages' checks (`utils/doc_audit.py`, and every command it shows is run).
- The page names what is *not established* as such. No row of F26's sketch is copied as fact.
- F1b's four PFLs each point at the page that answers them.

## What it may find

- **That the agenda is not fair** in some corner (a rule whose firings always precede another's). That is a
  finding about the engine: shown, and a row of P1g.6 if a theory of this milestone can meet it.
- **That the obligations rung is not a chase** in any standard sense, because it proposes and the search
  decides. Then the page says what it is instead, in Ein's terms, and F26's row is corrected.
