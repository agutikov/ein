# S1g.3.2 — Finding a proof: the flood measured, goals as requests, guesses as hypotheses, and the deduction theorem

**Phase:** [P1g.3](README.md)
**Status:** planned 2026-11-11, not started. A report first; what ships follows what it finds.
**Depends on:** [S1g.3.1](s1g.3.1_hilbert_systems.md).

## What it asks

Checking a proof is saturation. Finding one is where the instruction's *"what are hypothesis in this case?"*
has its least obvious answer. The stage tries the three ways Ein could find a Hilbert proof, measures each,
and says which, if any, is worth shipping. Everything here is **to be tried**; the coordinator's
expectations are marked as such.

## The three ways

**1. Forward: derive every theorem.** Replace the written lines by rules that *produce* theorems: every
instance of an axiom is a theorem, and modus ponens closes the set.

- It cannot be written as it stands. `(rule k () :match () :assert (thm (Imp ?a (Imp ?b ?a))))` is refused at
  load: a conclusion's variable nothing binds
  ([`defined_behaviour.md` § 4.13](../../../docs/kernel/defined_behaviour.md)); the binary's line is *rule
  'k': its :assert writes ?a, which no premise or parameter binds* (one run, release binary at `8860d616`,
  2026-11-11). And rightly: the instances
  are infinitely many.
- So the instances range over a **universe of formulas** given as facts, `(formula ?a)`: every formula up to
  a size over the goal's letters, or the subformulas of the goal. Then saturation terminates.
- Two things to measure. **The flood:** how many theorems and firings the closure has as the universe
  grows; expected to be most of the universe's tautologies of the implicational shape, very fast. **The
  reach:** a Hilbert proof is not analytic, so its lines are not subformulas of its goal. The proof of
  `(Imp p p)` needs `(Imp (Imp p p) p)` inside its instances. So the subformula universe is expected to be
  incomplete, and the smallest universe that proves each of S1g.3.1's theorems is the number to report.
- It derives theorems, not proofs. The proof is in the engine's record: each `thm` fact's justification is a
  derivation, and `--trace` prints it. Whether that record can be turned back into S1g.3.1's proof format
  and re-checked is the test that the two notions of proof, the engine's and the theory's, agree.

**2. Backward: a goal as a request.** The calculator's protocol: `(prove F)` answered by `(proved F PROOF)`.

- For an axiom instance the answer is immediate.
- For modus ponens the request *prove B* must ask for *prove (Imp A B)* and *prove A* for some A, and
  nothing says which A: the cut formula. A request cannot leave a variable open.
- So backward search needs the same universe to draw A from, and asks for every A in it: demand-driven in
  name, a flood in fact. Expected: no better than forward for a Hilbert system. It is tried because the same
  protocol is expected to work well for natural deduction's introduction rules, where the goal's shape
  decides the step ([S1g.3.3](s1g.3.3_natural_deduction.md)), and the contrast is the finding.

**3. Search: a guess as a hypothesis.** An hrule proposes the cut formula, or the next line of a proof of
bounded length, and the checker of S1g.3.1 runs under each commitment.

- R is then the set of *proofs* (or of sets of lines that contain a proof) within the bound.
- The lattice has no notion of a goal to be reached except through an obligation: *the goal is owed a
  checked line*. Then a discharged state is one in which the goal is proved, the minimal line is the
  smallest sets of guessed lines that prove it, and `--minimal` asks for a shortest proof. **To be tried;**
  it is the plan's one idea for making Ein's own search look for proofs, and it may be hopeless past a few
  lines, since the candidates are every (line, formula, reason) triple.
- The dead states are few (a wrong line is merely unchecked, not contradictory), so nothing prunes. That is
  the expected finding: Ein's search learns from contradictions, and a failed proof attempt is not one.

## The deduction theorem

*If B is derivable from A then `(Imp A B)` is a theorem*, by a transformation of the derivation, line by
line, using K and S. It matters three ways and the stage does each:

- **As a rewriting theory in Ein:** a request `(discharge A PROOF)` answered by a proof of the implication, in
  S1g.3.1's proof-term form, each case a rule. Search-free. The output is checked by the checker. It is the
  first program in the milestone that computes *on proofs*.
- **As the bridge to natural deduction:** it is what makes an assumption's discharge admissible in a system
  that has no assumptions, and so it is how the Rust harness supplies Hilbert proofs for S1g.3.4.
- **As a measure:** the blow-up in proof length, counted.

## Tasks

1. **T1g.3.2.1 — forward,** over universes of growing size: theorems, firings, time, and the smallest
   universe that reaches each of S1g.3.1's theorems. Each run under a timeout that fails loudly.
2. **T1g.3.2.2 — the engine's record against the theory's proof:** one theorem's justification read from
   `--trace` or the dump, written as an S1g.3.1 proof, and checked.
3. **T1g.3.2.3 — backward,** on the same theorems; the requests counted against forward's firings.
4. **T1g.3.2.4 — search,** the obligation form, on `(Imp p p)` with the bound at five lines and a small
   formula universe; what R, the minimal line and N are; where it stops being runnable.
5. **T1g.3.2.5 — the deduction theorem** as a theory, its outputs checked, its blow-up counted.
6. **T1g.3.2.6 — the report,** `finding_a_proof.md` in this directory, and a recommendation: what ships (the
   deduction-theorem theory is the likely one) and what stays a probe under `examples/`.

## Acceptance

- Each way has a program, its run and its numbers, each number with its command; or a stated reason it could
  not be run.
- The deduction-theorem theory is search-free and its every output is accepted by S1g.3.1's checker, held by
  a test, on every banked proof with a premise.
- The report says which way finds which of S1g.3.1's theorems, and names the smallest one none finds.

## What it may find

- **A saturation that does not stop,** if a universe is defined by a rule that grows formulas. Every
  universe here is written as facts or bounded by a counter the rules walk down; the engine's own word for
  the other case is [S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md)'s.
- **That the engine's justification of a `thm` fact is not the shortest proof,** nor canonical: a core is an
  explanation (the user's ruling of 2026-11-08), and so is a derivation. The page says which proof the
  engine recorded and does not call it *the* proof.
- **That proof search wants a strategy Ein has no place for** (an ordering on goals, iterative deepening on
  the bound). Then it is a row of P1g.6 only if a later phase needs it; otherwise the report says so and
  stops.
