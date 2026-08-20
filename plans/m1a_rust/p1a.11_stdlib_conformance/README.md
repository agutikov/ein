# P1a.11 — stdlib conformance

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 2.5 weeks (13 days of stages)
**Depends on:** [P1a.10](../p1a.10_single_implementation/README.md) — a new
surface form is cheap to add to one implementation and expensive to add to two
in step. Landing it before the oracle leaves means writing it twice.

## Goal

**The stdlib's rules get expectations of their own, checked by the engine
rather than by a diff.** A corpus of small `.ein` programs, one per rule or
rule family, each stating what it should derive — and an `ein test` that runs
them and reports pass/fail without anybody reading output.

## Why this is not more of the same

Every check the repo has today is *relative*: T0–T3 compare two engines, and
the goldens compare ein.rs to its own past. Both answer "did this change?" and
neither answers "is this right?". The stdlib is where that gap is widest:
`std.algebra`, `std.bijection`, `std.elim`, `std.closure`, `std.slots`,
`std.typing` and `std.macro` are the rules every puzzle imports, they are
exercised **only** as a side effect of whatever the zebra corpus happens to
need, and a rule that is never activated by any corpus entry is not tested at
all — it is merely not contradicted.

The session that produced this phase is the argument. `disjunctive-prune`'s
`(neq ?h_other ?h1)` guard was wrong for a year, in a rule five phases of byte
parity signed off, and it took an *independent* enumeration of a puzzle's
models to find it — because both engines agreed, and agreement was all
anything checked. That rule was puzzle-level. The stdlib has no such
independent check at all.

An expectation written next to a rule is that check, and it is the one kind
that gets **stronger** when the oracle leaves.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.11.1](s1a.11.1_what_the_stdlib_promises.md) | What the stdlib promises, and what is exercised | 3 d |
| [S1a.11.2](s1a.11.2_test_form.md) | The `(test …)` form | 3 d |
| [S1a.11.3](s1a.11.3_test_subcommand.md) | `ein test` | 2 d |
| [S1a.11.4](s1a.11.4_stdlib_corpus.md) | The stdlib corpus | 4 d |
| [S1a.11.5](s1a.11.5_gate.md) | In the gate | 1 d |

## Acceptance for the phase

- **Every stdlib rule has at least one program that activates it and states
  what it should derive**, and the coverage claim is *measured* — a firing
  count per rule over the corpus, not a reading of the source.
- `ein test <file>` exits 0 / non-zero and prints what failed. No stdout
  diffing, no golden file, no second engine.
- A **negative** case per rule wherever one is meaningful: not only "this fires
  and derives X" but "this does *not* fire", which is where a guard bug lives.
  `disjunctive-prune`'s was exactly that shape.
- The form is in `grammar.lark` — the spec of record — and
  [M2](../../m2_nl_to_ir/README.md)'s GBNF lift reads it, so the grammar change
  is a deliberate cross-milestone edit and not a local convenience.
- Adding a rule to the stdlib without a test fails the gate, the same way a
  file without a corpus entry does today.

## Risks

- **A test form is language surface.** M1a's non-goals say "no new syntax, no
  new keywords" — that was written to keep the *port* honest against an oracle.
  With one implementation the constraint changes meaning, but the cost does
  not vanish: `(test …)` has to be parsed, dumped, macro-expanded, rendered
  and round-tripped like any other form, and every tool that walks an AST
  grows a case. **The alternative is a sidecar** (expectations in
  `corpus.toml`, or a `.expect` file beside the `.ein`) which costs no grammar
  at all. [S1a.11.2](s1a.11.2_test_form.md) has to choose, and the user's
  stated preference is the in-file form.
- **Expressive creep.** The moment expectations can say "this fact holds", the
  next request is "this fact holds *because*", then a quantifier, then a
  little language. The stdlib's rules are the reason to stop early: what they
  need is *derives / does not derive / verdict is*, and each addition past
  that should have a rule that demanded it.
- **A test that only restates the rule.** `functional-negative` asserts
  `(not (R a b'))`; a test that says "and then `(not (R a b'))` holds" has
  checked that the engine can read. The expectations that matter state the
  *consequence at a distance* — what the rule makes possible two firings
  later — and the stage says so explicitly because the cheap kind is easy to
  write by the dozen.
- **`test` as a head is not a reserved word today.** A puzzle may already use
  it as a relation name. `grammar.lark`'s SYMBOL exclusions are the mechanism
  and the change is not free.

## Cross-links

- [`stdlib/`](../../../stdlib/) — the seven modules under test
- [`docs/kernel/ir/03-ein-lang/`](../../../docs/kernel/ir/03-ein-lang/) — the
  surface language the new form joins
- [P1a.10](../p1a.10_single_implementation/README.md) —
  [S1a.10.1](../p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md)'s
  accepted-loss list is this phase's first input
