# S1d.2.3 — The obligation: form, surface, and where it lives

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1d.2.2](s1d.2.2_domains.md) for the domain contract; the
*decisions* below are already taken and do not wait on it.

## The decisions, as taken 2026-08-24

This stage was planned as the phase's decision point. The decisions were made
in-session instead, on [`obligation_forms.md`](obligation_forms.md), and this
file records them; what is left for the stage is the implementation.

1. **The form is G** — a reserved verdict atom in `:assert`, derived only by
   rules, tallied per quiescent KB, never stored (§ G's extension-stability
   argument: contradiction survives extension, openness exists to be
   destroyed by it). Q-M1d.2 closes at **(c) a rule shape**.
2. **The atom is `open`** — naming pair P3, and the probe rename that frees
   the word is **already executed** (`7e1192c`): `std.macro`'s third-state
   probe is `(unknown P)`. The reservation of `open` happens here, closing
   the window the naming decision's ordering constraint left.
3. **The argument shape is `forall`'s dual** — settled by the user's
   numeral-free revision, which killed the positional spelling
   (`(open ?R 0 ?a)` carries a position index, the currency the language
   refuses):

   ```lisp
   (rule total-owed (?R ?isa)
     :match  (and (relation ?R ?A ?B) (?isa ?a ?A)
                  (absent (and (?isa ?b ?B) (?R ?a ?b))))   ; ∄ typed witness
     :assert (open ?b (?isa ?b ?B) (?R ?a ?b))
     :priority 500
     :why    "{?R} owes {?a} a {?B}")
   ```

   `(open ?b G B)` — bound variable, guard, body — is exactly `forall`'s
   operand triple in the dual position. The variable is **form-bound**, the
   precedent `forall` set in `:match`, so no load rule about unbound
   assert-side variables bends. The guard is the witness domain,
   `?isa`-parameterised, is-a-free. Discharge is "∃b: G ∧ B present",
   computed by the engine at the boundary. Candidates, when a later stage
   wants them, are `{b : G(b), B neither present nor forbidden}`.
4. **Bare `(open)` stays legal** — the proposal's original degenerate: the
   whole condition in the rule's `:match`, the atom anonymous. Countable,
   `:why` is its report; no candidates derivable, no discharge beyond the
   rule ceasing to fire.
5. **Not taken:** D's numeric sugar (`at-least 1 …` — a numeral); **A**
   (`:cardinality` — deferred until a corpus entry asks, P1c.1's keyword
   rule); **E** (materialised clauses — deferred to
   [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md)'s evidence); **B**'s
   stored carrier fact (nothing left for it to carry); **F** (subsumed by
   the supersession ladder). Numeric bounds beyond `0/1` are pairings to
   reference extents when they are ever needed —
   [`obligation_forms.md` § Cardinality without numerals](obligation_forms.md).

## What the stage implements

- **Reserve `open`.** A row in
  [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  beside `false`; the reserved-name guard refuses `(relation open …)`,
  `(macro open …)` and a declared name `open`, with the defined-diagnostic
  treatment ([`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
  gains the string). The corpus is already clean — the rename saw to it.
- **Load the two forms.** `(open ?b G B)` and `(open)` legal in `:assert`
  only; `(open …)` in `:match` is a load error with a named diagnostic
  (the probe is `(unknown …)` — say so in the message). Arity anything else:
  load error.
- **Round-trip.** The dumper emits what the parser read;
  `ir_semantics.rs`'s round-trip suite gains both forms. The TextMate
  grammar in `utils/vscode-ein/` learns the head.
- **No behaviour yet.** This stage loads and round-trips the atom; the
  saturator ignores it until [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md).
  A program using it is inert but legal — which keeps this stage's diff
  small and its risk zero.

## Acceptance

- `open` reserved, with the diagnostic strings in `defined_behaviour.md`;
  the P3 ordering window is closed.
- Both forms load, dump, and round-trip; `:match` placement refused by name;
  wrong arity refused.
- Grammar pages updated (`01_grammar.md` § patterns table gains the assert
  row; `06_reserved_names.md`); **M2's GBNF lift is untouched** — no new
  top-level head, no parser change, which is the cost C would have incurred
  and G does not.
- `cargo test --workspace` green with the atom present-but-inert; every
  existing verdict and counter unchanged.
