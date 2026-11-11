# S1g.2.1 — Formulas and evaluation: the connectives' tables, the value of a formula on request, and the bridge to circuits

**Phase:** [P1g.2](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s rulings on the conventions
([S1g.1.3](../p1g.1_survey_and_design/s1g.1.3_conventions.md)); S1f.6.17's gate tables.

## What it asks

How a propositional formula is represented and how its truth value is computed, as a theory a program
imports. The answer the plan expects: a nested fact, and a request answered by saturation over truth tables,
*"like it is done in electronics"* (the user's words for the calculator, 2026-11-08), with no search.

## The theory

The sketch is [S1g.1.1's](../p1g.1_survey_and_design/s1g.1.1_what_ein_represents_today.md#the-sketch-for-rows-2-to-4),
without its hypotheses: a valuation is written as facts, and the program asks for a value.

```lisp
;; sketch only: the theory's rules are S1g.1.1's sketch; this is the program that uses them
(atom p) (atom q)
(val p 1) (val q 0)
(eval (Imp (And p q) p))
(solve :hypothesis-relations ()             ; the module will close its relations; a sketch says it here
       :expect (region (model)))            ; nothing is guessed: R is {∅}
(query :goal (value-of (Imp (And p q) p) ?v)
       :expect (model (value-of (Imp (And p q) p) 1)))
```

What the stage settles about it:

- **The tables.** One per connective, generated with a `--check` if S1f.6.17's are, or simply imported from
  S1f.6.17's module if the truth values are the bits
  ([Q-M1g.3](../open_questions.md#q-m1g3--the-two-truth-values)). `Top` and `Bot` have a value and no table.
- **One rule for all binary connectives,** its table a parameter, `(value-binary And t-and)`: probe 1 wrote
  it so, and a relation parameter heading a nested pattern, `(eval (?C ?a ?b))`, loaded and matched. To be
  confirmed against [`defined_behaviour.md` § 4.9](../../../docs/kernel/defined_behaviour.md) for every
  activator the theory writes.
- **Demand.** A request for a compound asks for its parts; a value is derived only for what was asked. A
  formula never asked for has no value, and the page says so.
- **Closure.** The module closes its relations, so a program that imports it has no candidate from them. It
  is not optional: the sketch above with a bare `(solve :expect (region (model)))` gave the blind enumerator
  62 candidates and a lower bound of 39 161 states, and with `:hypothesis-relations ()` no candidate and both
  claims held (`ein solve` and `ein test` on the sketch joined to S1g.1.1's rules, release binary at
  `8860d616`, 2026-11-11, one run each).
- **Two ways to write the evaluation rule,** both built and measured:
  - **(a) the join,** as in the sketch: one premise is the request, two look up the parts' values. A part is
    a nested fact, and its value is the free slot, so each lookup is the scan E-3 describes
    ([`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)).
  - **(b) the continuation,** the calculator's design: evaluation walks the formula carrying a tag that says
    what to do with the value, so no rule joins two facts on a formula. More rules, and a trace that is a
    sequence.
  - The measure is the matcher's candidates per request on a batch (`--features counters`, as
    `ein-infer/tests/arith_batch.rs` takes it), at batches of several sizes, so that *linear* or *quadratic*
    is a count and not a clock.

## The bridge to circuits

A formula is a circuit with one output and no shared wire names. Two requests, each a chain of rules:

- **formula to circuit:** name each subformula's wire (the subformula itself can be the wire's name, a nested
  fact, if S1f.6.17's wires may be values; otherwise this direction needs names nothing can make, and the
  stage says so, [Q-M1g.8](../open_questions.md#q-m1g8--fresh-names));
- **circuit to formula:** unfold each wire to the formula of its gate, for a circuit with no feedback.

The test is S1f.6.17's gate-built full adder: its two outputs as formulas over three letters, each with the
same eight-row table as the gates.

## Tasks

1. **T1g.2.1.1 — the theory and its tables,** as ruled at P1g.1's stop, where it was ruled to live.
2. **T1g.2.1.2 — the independent check.** A Rust test, `logic_theory.rs` beside the calculator's
   `arith_theory.rs`: it enumerates every formula up to a bound (a count of connectives over a set of
   letters, [Q-M1g.9](../open_questions.md#q-m1g9--the-bound-of-an-exhaustive-check-and-its-tier)),
   evaluates each under every valuation with Rust's booleans, writes the Ein program, and compares every
   `value-of`. It is a table of checks, so that normal forms and proof systems add rows.
3. **T1g.2.1.3 — the search-free test,** the calculator's `arith_search_free.rs` extended or copied: no
   candidate, one saturation, R {∅}, the same facts under `ein solve`, `ein saturate` and `ein test`, and no
   `absent` or `not` premise in the module's text.
4. **T1g.2.1.4 — the two evaluation rules measured,** and one chosen for what ships; the other kept as an
   example with its numbers.
5. **T1g.2.1.5 — the bridge,** and the adder held equal.
6. **T1g.2.1.6 — examples and pages:** a program per connective, one for demand (a formula not asked for),
   and the module's entry wherever the stdlib's API page is by then.

## Acceptance

- A program states a formula and a valuation and reads the value; search-free, held by a test.
- Every value agrees with the Rust check on every formula up to the stated bound (the bound, the count of
  formulas and the command are on the page).
- The cost of a batch is stated as candidates per request for both rule shapes.
- The adder's formulas and its gates have one table, held by a test.

## What it may find

- That a relation parameter heading a nested pattern has a corner § 4.9 or § 4.14 refuses (a parameter that
  could be bound to `eq`). Then the theory writes a rule per connective.
- That (a) is quadratic enough to matter at the sizes P1g.3 needs. Then (b) ships and
  [S1g.6.2](../p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md) gets its witness.
