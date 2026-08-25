# S1d.2.3 — The obligation: form, surface, and where it lives

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days — unchanged, and the two halves moved in opposite
directions: the assert-side binder carve-out the triple needed is gone, and
the projection's well-formedness check (below) is new.
**Depends on:** [S1d.2.2](s1d.2.2_domains.md) for the domain contract; the
*decisions* below are already taken and do not wait on it.

## The decisions, as taken 2026-08-24 and revised 2026-08-25

This stage was planned as the phase's decision point. The decisions were made
in-session instead, on [`obligation_forms.md`](obligation_forms.md), and this
file records them; what is left for the stage is the implementation.
**Revised 2026-08-25 in two places**: item 3's argument shape, and item 1,
which gained *when* the obligation rules run. Items 2 and 5 stand as taken.

1. **The form is G** — a reserved verdict atom in `:assert`, derived only by
   rules, tallied per quiescent KB, never stored (§ G's extension-stability
   argument: contradiction survives extension, openness exists to be
   destroyed by it). Q-M1d.2 closes at **(c) a rule shape**.

   **Where it lives instead** (the user's note, 2026-08-25): not in the KB —
   truth maintenance is the reason — but *in the node*. The tally is state of
   the search-lattice node, and the slot already exists:
   [`CommitmentSetResult`](../../../ein.rs/crates/ein-infer/src/commitment.rs)
   is `{ commitment, kb, firings, kind, unsat_core, hypothesis_facts }`, where
   `kind` is a per-node verdict that is not a fact. The tally belongs beside
   it. "Not stored" names where it isn't; "a field of the lattice node" names
   where it is, and [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) is
   written against the second.

   **When they run** (the user, 2026-08-25): **after saturation completes,
   not mixed into it.** Obligation rules are not in the saturation agenda —
   they are one pass over the quiescent KB, run once the fixpoint is reached.
   § G had reached for a priority band (500) to keep an obligation from
   reporting a debt that negative-completion (240) and elimination (400) were
   about to pay; a band is the wrong instrument, because it orders *selection
   inside* the loop and what openness needs is to be read *after* it. An
   `open` conclusion is never admitted, so an obligation rule derives
   nothing, enables nothing, and has no business in a queue that exists to
   order derivation — and read at the fixpoint the tally is a function of the
   final KB rather than of a moment during the walk toward it.

   Two consequences this stage implements. `:priority` on an obligation rule
   no longer selects against saturation rules; its residual meaning is the
   **report order among obligation rules**, which is what keeps the
   outstanding-obligations list deterministic. And **a rule whose `:assert`
   contains `open` may assert nothing else** — a mixed rule would belong to
   both passes, and refusing it at load is cheaper than deciding which pass
   owns it.
2. **The atom is `open`** — naming pair P3, and the probe rename that frees
   the word is **already executed** (`7e1192c`): `std.macro`'s third-state
   probe is `(unknown P)`. The reservation of `open` happens here, closing
   the window the naming decision's ordering constraint left.
3. **The argument is the relation** — `(open ?R)`, decided 2026-08-25,
   superseding the triple `(open ?b G B)` this file recorded on 2026-08-24
   and the positional `(open ?R 0 ?a)` the numeral-free revision had already
   killed:

   ```lisp
   (rule total-owed (?R ?isa)
     :match  (and (relation ?R ?A ?B) (?isa ?a ?A)
                  (absent (and (?isa ?b ?B) (?R ?a ?b))))   ; ∄ typed witness
     :assert (open ?R)
     :why    "{?R} owes {?a} a {?B}")
   ```

   Read it as **the set of `?R` facts in this KB is incomplete**. The
   obligation is stated *once*, in the `absent`; the atom names which
   relation's extent the absence is about, and the engine projects the rest
   out of the compiled guard. All other conjuncts are the domain or the
   image.

   **Why the triple went.** It restated the guard in the head, and the guard
   already said everything it restated:

   | what a consumer wants | the guard alone | the triple added |
   |---|---|---|
   | detect openness | the rule fires ⟺ ∄ witness — the guard **is** the detection | the same predicate, restated |
   | discharge | the guard stops matching, via `absents_still_pass` | a second `∃b: G ∧ B` query per instance |
   | the report | `:why` renders from the match bindings | nothing |
   | instance identity | `(rule, bindings)` | nothing |
   | **the branch slot** | buried in the sub-plan | **marked** |

   Only the last line was content, and a head that restates a guard can
   disagree with it. `(open ?R)` points *into* the guard instead — the same
   relationship `(false)` has to its firing chain, which is the precedent
   that a verdict atom loses nothing by carrying no operands: the unsat core
   comes from the derivation DAG, not from arguments
   ([`contradiction.rs`](../../../ein.rs/crates/ein-infer/src/contradiction.rs) —
   for a direct ⊥ "the `(false …)` fact, whose DAG *is* the firing chain of
   the rule that emitted it").

   **The projection, and it is static.** Rules compile per `(rule,
   activator)` and the compiler substitutes the parameters, baking concrete
   relation names into the plan
   ([`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) head
   note), so `?R` is a `Symbol` *before matching begins* and the witness step
   is found by symbol equality inside `Step::Absent { sub }` over `RelStep {
   rel, slots, shared, … }`
   ([`plan.rs`](../../../ein.rs/crates/ein-infer/src/plan.rs)). Resolved once
   per activator and cached in the plan memo: **zero per-firing cost**, and a
   malformed obligation is caught when the rule is activated rather than when
   the search reaches it.

   Three resolution rules, which this stage states and checks:

   | | the rule | refused when |
   |---|---|---|
   | which `absent` | the one holding a step whose `rel` is `?R`'s binding | none holds one, or two do |
   | which step | the positive, free-variable-bearing one — `(not (?R …))` is never a commit target | two candidates remain |
   | which slots | those unbound at the absent's entry: `?a` arrives bound from `(?isa ?a ?A)`, `?b` does not, so `?b` is the scan | none is free — a ground body is a plain `absent` check, not an obligation |

   The third needs care rather than assumption: `RelStep.shared` records what
   an earlier premise already mentioned, but `plan.rs`'s own note is explicit
   that register-boundness at a step depends on the entry point, since
   `run_seeded` binds a step first. Static for the rule's normal run; this
   stage writes the statement down, and T1d.2.3.2 is where it is checked
   rather than believed.

   **Both duals use this one atom**, and the *direction* falls out of which
   slot is free — `total-owed` scans `?b`, `surjective-owed` scans `?a`. No
   position index, which is what the numeral-free revision demanded and what
   `(open ?R 0 ?a)` violated.

   **What it cannot say**: a compound witness — `(absent (and (?R ?a ?x) (?R
   ?x ?b)))`, "there must exist a 2-chain" — where two positive `?R` steps
   bear free variables. Rule 2 refuses it rather than guessing. That leaves
   every stdlib property (`total`, `surjective`, the slots duals) and every
   pairing bound in the numeral-free decomposition expressible, so no corpus
   entry needs it; by
   [P1c.1](../../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance)'s
   keyword rule the richer form arrives when one cannot be stated without it.
4. **Bare `(open)` stays legal**, and the two forms now *nest* rather than
   sitting apart:

   | | counts | reports (`:why`) | attributes | branches |
   |---|---|---|---|---|
   | `(open)` | yes | yes | no | no |
   | `(open ?R)` | yes | yes | **per relation** | **yes** |

   Attribution is a gain the triple did not have either: a per-relation tally
   makes `--json-summary` say `owes: {pet-loc: 9, nation-loc: 8, …}`, so
   [§5](obligation_forms.md#5-what-this-looks-like-on-zebra2-minus-15)'s
   conservation audit checks per relation and not only in total; and
   `:no-hypothesis` becomes a membership test on the atom's own argument,
   which is what makes [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md)'s
   "owed but not branchable ⇒ stuck, reported" a question one can pose.

   **Spelling is parenthesized**, like `(false)`: verified 2026-08-25 that a
   bare symbol in assert position is `TypeError: expected NestedPattern at
   :assert top-level, got Atom`, so the proposal's `:assert open` is
   `:assert (open)`.
5. **Not taken:** D's numeric sugar (`at-least 1 …` — a numeral); **A**
   (`:cardinality` — deferred until a corpus entry asks, P1c.1's keyword
   rule); **E** (materialised clauses — deferred to
   [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md)'s evidence); **B**'s
   stored carrier fact (nothing left for it to carry); **F** (subsumed by
   the supersession ladder); and, since 2026-08-25, **the triple** `(open ?b
   G B)` — superseded by item 3, not deferred. Numeric bounds beyond `0/1`
   are pairings to reference extents when they are ever needed —
   [`obligation_forms.md` § Cardinality without numerals](obligation_forms.md).
6. **What the revision dissolved.** Two findings of the 2026-08-25 review of
   this phase, both of them costs of the triple and neither of them a cost
   any more:
   - **the binder carve-out.** `(open ?b G B)`'s `?b` was bound only inside
     the match's `absent`, and an assert-side variable in that state is
     `KeyError: "unbound var ?b in :assert — bindings: {…}"`, exit 1 — one of
     the thirteen in
     [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md), so
     the form needed an exception to a *normative* diagnostic, and one whose
     failure mode was silent misbinding rather than an error. `(open ?R)`'s
     `?R` is bound by an ordinary premise, `(relation ?R ?A ?B)`. Nothing
     bends.
   - **the double-stated discharge.** The triple let the head's `G ∧ B`
     disagree with the guard's, with no rule saying which was authoritative —
     and S1d.2.4's `owes = 46` acceptance depends on the answer. There is now
     one statement, and it is the guard.

## What the stage implements

- **Reserve `open`.** A row in
  [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  beside `false`; the reserved-name guard refuses `(relation open …)`,
  `(macro open …)` and a declared name `open`, with the defined-diagnostic
  treatment ([`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
  gains the string). The corpus is already clean — the rename saw to it, and
  a re-check on 2026-08-25 found no bare `open` head in any `.ein`: every
  occurrence is `open-slot` / `state-open` / `maybe-open` / `open-candidate`
  or sits inside a `:why` string.
- **Load the two forms.** `(open)` and `(open ?R)` legal in `:assert` only;
  `(open …)` in `:match` is a load error with a named diagnostic (the probe
  is `(unknown …)` — say so in the message). Arity 2 or more: load error.
- **Classify obligation rules out of the saturation agenda.** A rule
  asserting `open` is loaded into the obligation pass, not the saturator's
  rule set, and a rule whose `:assert` mixes `open` with anything else is a
  load error (item 1, *When they run*). The pass itself is
  [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md)'s; what this stage owes
  is the classification and the refusal, so that no obligation rule ever
  reaches the agenda even while the pass is a no-op.
- **Resolve the projection, and refuse what will not resolve.** The three
  rules of item 3, checked per `(rule, activator)` when the plan is compiled.
  **The stage's one open decision**: a malformed obligation *refuses* or is
  *skipped*. The precedent cuts the other way — S1.22.0 skips activators
  whose arity does not match the parameter list — but silence is the thing
  this phase exists to remove, so the recommendation is refuse, with the
  diagnostic named in `defined_behaviour.md` beside the others this stage
  adds.
- **Round-trip.** The dumper emits what the parser read;
  `ir_semantics.rs`'s round-trip suite gains both forms. The TextMate
  grammar in `utils/vscode-ein/` learns the head.
- **No behaviour yet.** This stage loads, resolves and round-trips the atom;
  the saturator ignores it until
  [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md). A program using it is
  inert but legal — which keeps this stage's diff small and its risk zero.

## Tasks

### Task T1d.2.3.1 — reserve, load, round-trip

`open` reserved; both forms accepted in `:assert` and refused in `:match` and
at arity ≥ 2; dumper and round-trip suite; the grammar pages.

### Task T1d.2.3.2 — the projection, and its three refusals

The witness step located by symbol equality inside the absent sub-plan, per
`(rule, activator)`, cached with the plan. A fixture per refusal (no `?R`
step; two `absent`s holding one; two positive free-bearing `?R` steps — the
compound witness; a ground body), and the boundness claim of rule 3 checked
against a seeded run rather than assumed.

### Task T1d.2.3.3 — the two stdlib duals, written but inert

`total-owed` / `surjective-owed` in the file S1d.2.1's audit says owns
totality, asserting `(open ?R)` and nothing else, fanned out by
`bijective-setup`. They load, they activate, they resolve — and the saturator
does nothing with them until S1d.2.4. Writing them here is what proves the
projection resolves on the shapes the phase actually ships.

## Acceptance

- `open` reserved, with the diagnostic strings in `defined_behaviour.md`;
  the P3 ordering window is closed.
- Both forms load, dump, and round-trip; `:match` placement refused by name;
  arity ≥ 2 refused.
- The projection resolves on `total-owed` and `surjective-owed`, statically,
  per activator — and each of the four malformed shapes is refused with its
  own diagnostic.
- No assert-side variable is introduced anywhere: `defined_behaviour.md`'s
  unbound-`:assert` row is untouched, which is the check that item 6's first
  bullet stayed true.
- **No obligation rule is in the saturation agenda**, provable rather than
  asserted: with the duals of T1d.2.3.3 loaded and activated, the firing
  counts and the rule-selection order of every corpus entry are bit-identical
  to the pre-stage run. A mixed `:assert` is refused, with its fixture.
- Grammar pages updated (`01_grammar.md` § patterns table gains the assert
  row; `06_reserved_names.md`); **M2's GBNF lift is untouched** — no new
  top-level head, no parser change, which is the cost C would have incurred
  and G does not.
- `cargo test --workspace` green with the atom present-but-inert; every
  existing verdict and counter unchanged.
