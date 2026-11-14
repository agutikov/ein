# S1g.1.3 — The conventions: a formula's shape, the connectives' names, the truth values, the protocol, and where things ship

**Phase:** [P1g.1](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.1.1](s1g.1.1_what_ein_represents_today.md); and
[S1f.6.17](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md)'s ruling on the
gates' names, which this stage follows.
**For the user:** three questions, [Q-M1g.1](../open_questions.md#q-m1g1--the-spelling-of-the-connectives)
(already defaulted), [Q-M1g.2](../open_questions.md#q-m1g2--where-the-logic-ships) and
[Q-M1g.3](../open_questions.md#q-m1g3--the-two-truth-values). They are put in `design.md` with the phase's
other questions, at S1g.1.4's stop.

## What it asks

*"how preposition logic formulas can be represented?"* (the user, 2026-11-11). The question has a short
answer, a nested fact, and several choices under it that every later theory inherits. The stage shows each
choice on real programs, computed, before anything is named in a shipped file (the user's rule for new
syntax: real examples first).

## The choices

1. **The shape of a formula.**
   - **(a) A tree:** `(Imp (And p q) p)`, a nested fact. A formula is a value: it can be an argument, a
     request, a line of a proof. Shared subformulas are shared by interning.
   - **(b) A circuit:** S1f.6.17's shape, a gate instance a fact and a wire a name:
     `(gate g1 And p q) (gate g2 Imp g1 p)`. Flat, joins on names (so the fact index serves it, where it does
     not serve (a): E-3 of the calculator's
     [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)), but a formula is
     then a set of facts and cannot be passed as one value.
   - **(c) Both, with a conversion:** a request that names a tree's subformulas, which is what probe 1's
     `eval` requests already do.
   - The default is (a) for every theory about formulas, since proofs and substitution need a formula as a
     value; (b) stays S1f.6.17's; the page measures (a) against (b) on evaluation
     ([S1g.2.1](../p1g.2_propositional_logic/s1g.2.1_formulas_and_evaluation.md)).
2. **The connectives' names.** The user ruled that any spelling other than the engine's words will do. The
   default is capitalised: `And`, `Or`, `Not`, `Imp`, `Iff`, and the constants `Top` and `Bot`. To be checked
   on the binary: that `Not` and `not` are two names; that nothing reserves a capitalised word; what a
   formula looks like in a read-out, a trace and a `:why` under each candidate spelling.
3. **Which connectives are primitive.** A full set (`And`, `Or`, `Not`, `Imp`, `Iff`, `Top`, `Bot`) with a
   table and a rule each, or a small basis (`Imp` and `Bot`; or `Not` and `Imp`, Łukasiewicz's) with the rest
   defined by rewriting. Proof systems are stated over a basis, so P1g.3 needs the rewriting either way. The
   default: the full set at the semantic level, and a `define` rewriting into each proof system's basis.
4. **Propositional letters.** Bare names, declared by a fact, `(atom p)`, so that a rule can tell a letter
   from a compound without a negative test (the theory writes no `absent`). A letter is then a name of the
   program, and the blind enumerator may propose facts about it: each module closes its relations, as
   `std.math.decimal-arith` does.
5. **The truth values.** `0` and `1`, shared with the gates, or names of their own (Q-M1g.3).
6. **The protocol.** The calculator's: a request fact answered by a result fact that repeats the request,
   `(eval F)` by `(value-of F V)`, so nothing is computed that was not asked for. And whether the theory
   follows the calculator's *rule of the theory*, no rule joining two facts on a nested value, with a
   continuation on every call. Probe 1's `value-binary` breaks it, on purpose, as the natural way to write
   the rule; S1g.2.1 measures what that costs.
7. **Where things ship** (Q-M1g.2): `examples/logic/` first, the stdlib for what a second program imports.
8. **Reading a formula.** A `:why` renders `(Imp (And p q) p)` as written. Infix, `(p ∧ q) → p`, would need
   non-ASCII text, which a string may hold
   ([`defined_behaviour.md` § 1.6](../../../docs/kernel/defined_behaviour.md)) and a template cannot build
   from a nested value today. To be shown as it is; a readable rendering is
   [S1g.6.3](../p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md)'s.

## Tasks

1. **T1g.1.3.1 — each choice on two programs,** Peirce's law and one of S1f.6.17's circuits, computed, side
   by side: the program text, the read-out, a trace excerpt.
2. **T1g.1.3.2 — the name checks** of choice 2, each a one-line program and its result.
3. **T1g.1.3.3 — the recommendation,** one line a choice, and the three questions written for the user.

## Acceptance

- The section of `design.md` with every choice shown on computed programs and a recommendation for each.
- No file outside `plans/` and `build/`.

## What it may find

- A clash: a capitalised name some surface treats specially, or a spelling that reads badly in the
  natural-language text S1f.5.14 renders. Then the spelling changes here, once, before anything ships.
