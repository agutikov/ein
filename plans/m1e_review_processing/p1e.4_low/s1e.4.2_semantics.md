# S1e.4.2 — Semantics (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 1 day
**Depends on:** [AR-M1](../p1e.3_medium/s1e.3.4_architecture.md) — both
findings are instances of its pattern, and `SE-L1`'s fix is one of its four
pairs.
**Findings:** [`SE-L1`](../review/semantics/low.md),
[`SE-L2`](../review/semantics/low.md).

## Context

Two findings, both *the same name or the same event meaning two things*.

**`SE-L1` — two timeline emitters, two key orders.** `MonotonicDumper` emits
`layer / outcome / commitment / kind / firings / facts_merged /
unsat_core_size / nogood_*`
([`dump/state.rs:129-143`](../../../ein.rs/crates/ein-render/src/dump/state.rs));
`LatticeDumper` emits `layer / outcome / commitment / facts_merged / nogood_* /
kind / firings / unsat_core_size`
([`dump/lattice.rs:140-152`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs)).
The JSON writer preserves insertion order **as a document property — its whole
reason to exist** — so `00_timeline.jsonl` records for the same conceptual
event differ in shape between the two dumpers, and nothing says why at either
site.

**`SE-L2` — two different sets both named `RESERVED`.**
[`lex.rs:128`](../../../ein.rs/crates/ein-ir/src/lex.rs) holds 11
SYMBOL-excluded lexer words;
[`terms.rs:191`](../../../ein.rs/crates/ein-core/src/terms.rs) holds 9
shadow-check names including `open`. Both docs are individually accurate —
[`00_ebnf.md:51-62`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) says
*eleven RESERVED words*,
[`defined_behaviour.md:326-332`](../../../docs/kernel/defined_behaviour.md)
says *`open` joined RESERVED* — and a reader of both concludes the lexer set
grew to twelve. It did not, and it must not: `(open ?R)` has to lex as a
SYMBOL.

Note the relationship between the two findings. `SE-L2` is the case where two
copies of a name are **legitimately** different sets, and `CO-H2` was the case
where they were illegitimately different copies of one set. The same surface
symptom, opposite fixes — which is why
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s written rule has to cover
both, and why this stage is where the rule gets its counterexample.

## Acceptance

- One timeline key order, or two with the parity reason stated at both sites.
- No two constants in the tree named `RESERVED` for different sets; the docs'
  arithmetic reconstructs.
- If a golden pins the current key divergence, the unification is named as a
  deliberate re-bless before it happens.

## Tasks

### Task T1e.4.2.1 — `SE-L1`: one key order

First establish whether either order is pinned. `00_timeline.jsonl` is a
dumped artefact, and the JSON writer's insertion-order preservation is
deliberate — so a golden may encode today's divergence. If one does, the fix
is a named re-bless; if not, it is free.

Then extract a shared emitter, which is the fix
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md) wants — one function taking
the fields, used by both dumpers — rather than editing one to match the other,
which leaves two copies that merely agree today.

If a parity constraint genuinely requires the difference (the review does not
think one does, but the dumpers have different provenance), say so **at both
sites**, not at one.

### Task T1e.4.2.2 — `SE-L2`: rename one of the two

The review's suggestion is the right shape: `LEXER_KEYWORDS` versus
`SHADOW_GUARDED`, or equivalent, in **code and docs together**. The two sets
answer different questions — *what may not lex as a SYMBOL* and *what a
declarator may not bind* — and the names should say which.

Then fix the arithmetic in the two pages so a reader can reconstruct each set:
`00_ebnf.md`'s eleven are the lexer's, `defined_behaviour.md`'s nine are the
shadow guard's, and the sentence about `open` joining belongs only to the
second. Add the one line that prevents the confusion recurring: **`open` must
lex as a SYMBOL**, with the reason — `(open ?R)` is an ordinary fact head.

After [CO-H2](../p1e.2_high/s1e.2.1_correctness.md) there should be exactly
two such constants, not three. Confirm that as part of this task; it is the
cheapest possible check that the High fix landed completely.

## Notes

Both fixes are small and both are worth doing even if the phase is otherwise
dropped, because they are the two instances that make
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s rule complete: one pair
that must be unified, one pair that must be renamed. A rule with only the
first kind of example teaches the wrong lesson.
