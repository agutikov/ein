# P1g.20 — Theorems searched, proved, saved and reused

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-14, on the user's word of that day (*"materialize both phases, let theorem-reuse be
p1g.20 and this this self-theory p1g.30"*). Not started. **The first phase of the milestone's third group**, the
final formalizations ([M1g § Three groups of phases](../README.md#three-groups-of-phases-the-user-2026-11-14)).
No stage file yet: the phase is this README, its stages are sections below, and they are written out as files
when the second group has answered [Q-M1g.16](../open_questions.md#q-m1g16--proof-system-theory-and-what-a-theorem-is)
and built what the answer needs. Until then every stage below is provisional and changes with that answer.
**Depends on:** the second group's answer and implementation; of the first group,
[P1g.3](../p1g.3_proof_systems/README.md) (a proof checked by saturation, proof search, natural deduction),
[P1g.5](../p1g.5_peano_and_induction/README.md) (Peano, induction), [P1g.10](../p1g.10_school_algebra/README.md)
(identities by normal forms) and [P1g.11](../p1g.11_school_geometry/README.md) (theorems as rules); of M1f,
[S1f.6.35](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.35_imports_by_full_name.md), an import
one copy of a module by full name, which is how a module of theorems is reused.
**Needed by:** [P1g.30](../p1g.30_ein_in_ein/README.md), whose encoded proof system is this phase's.

## What it is

The main goal's fourth and fifth things, made to work end to end on the features the second group decided and
built: **a theorem searched for, proved, checked, saved, and reused to prove another**, on the theories the first
group attempted. The first group shows each step separately and names what stops it; the second decides where a
proof system and a theory live in Ein and what a theorem is, and builds what that needs; this phase runs the whole
chain on real theories and measures what reuse buys.

## The stages, provisional

1. **S1g.20.1 — A theory as modules.** Axioms, rules of inference and theorems laid out as the decision says:
   which module holds which, what each imports, what a theorem's module exports and under which names. The
   propositional proof system of P1g.3 rewritten on that layout first, since every later stage checks against it.
2. **S1g.20.2 — A theorem searched for and proved.** Proof search on the decided features
   ([S1g.3.2](../p1g.3_proof_systems/s1g.3.2_finding_a_proof.md)), the proof checked by saturation
   ([S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)), and the result **saved**: how a proved theorem
   becomes loadable text in a module, by what writes it, with its proof kept beside it or checkable again.
3. **S1g.20.3 — Reused.** The saved theorem imported and used to prove another, at the level the decision chose:
   as an axiom of the next proof (object level), as an Ein rule applied forward (meta level), or both through
   the bridge from a proved schema to a rule. Measured: the second proof with and without the lemma, its length
   and the saturation's cost.
4. **S1g.20.4 — Across the theories.** The same chain on Peano (a lemma proved by induction, `0 + n = n`, reused
   for commutativity of addition, [S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md)), on school
   algebra (an identity reused in a derivation) and on school geometry (a proved theorem made a rule of
   [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md)'s base).
5. **S1g.20.5 — A theorem used as a rule changes nothing it should not.** A derived rule is admissible: adding it
   derives nothing the base system does not, checked up to a bound against the base, and, where the decision
   gives one, proved. The stdlib's own unproved theorems written as rules, `std.algebra`'s
   `converse-pair-symmetric` first (M1f's [E-53](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md),
   W93), each proved or marked as an assumption on its module's page.

## Acceptance

- **The chain runs on programs:** one theorem searched for, proved and checked, saved as a module, imported, and
  reused to prove another, on propositional logic and on Peano at least; or the page names exactly which step
  stops and why (the milestone's acceptance line on the main goal).
- **Reuse is measured,** the second proof with and without the saved theorem.
- **Every theorem-as-rule the stdlib ships is proved or marked**, and a theorem used as a rule is held to the
  base system up to a bound.

## What it must not do

- **Not decide Q-M1g.16:** the second group does, and the user rules; this phase builds on the answer.
- **Not build a feature the second group did not decide.** A step that needs one stops and is shown.
