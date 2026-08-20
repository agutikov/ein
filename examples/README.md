# Ein examples

Encoded puzzles and focused fixtures, in [ein-lang](../docs/kernel/ir/03-ein-lang/).
Run any with `ein solve <file>` (or `ein saturate <file>` for the
saturation demos); see [`docs/api/`](../docs/api/) to drive them from Python.

> The step-by-step **human Zebra walkthrough** (the M1 target trace) used to
> live here; it moved to
> [`docs/kernel/inference/zebra_walkthrough.md`](../docs/kernel/inference/zebra_walkthrough.md).

## Zebra puzzle — two ontologies, one puzzle

`zebra.ein` and `zebra2.ein` are not "classic vs modern". They encode the
*same* Zebra puzzle over deliberately different vocabularies, both solve to the
same model, and the pair is kept because the comparison is the only way to see
which of the engine's reasoning power is general and which is an artefact of one
encoding. `acceptance/test_zebra_two_ontologies.py` pins that they agree cell by
cell; the design comparison and its measurements are in
C2.

| | [`zebra.ein`](zebra.ein) | [`zebra2.ein`](zebra2.ein) |
|---|---|---|
| attribute link | **one generic** `co-located` equivalence over all 30 values, whose classes *are* the houses | **five typed** projections (`color-loc : Color → House`, …) |
| membership | split: `(instance V T)` + `(type Sub Super)` | unified `is-a` |
| a cross-attribute clue | an ordinary fact: `(co-located Englishman Red)` | a 4-ary activator: `(co-located nation-loc Englishman color-loc Red)` |
| a spatial clue | an ordinary fact: `(right-of Green Ivory)` | a 5-ary activator: `(adjacent-via right-of color-loc Ivory color-loc Green)` |
| the property that drives it | type-scoped: `(slot-partition co-located instance type Attribute House)` + one `(slot-spatial …)` per spatial relation — `std.slots` | per-relation: `(bijective color-loc)` ×5 — `std.bijection` |
| rules defined in the file | 0 (all imported) | 12 |
| `solve --exhaustive` | Solution, k=1, exhausted — ~21 s (PyPy) | Solution, k=1, exhausted — ~9 s (PyPy) |

`zebra2.ein` remains the **primary M1 acceptance target** (it also carries the
Ambiguity and Contradiction task-class variants below);
`zebra.ein` is the independent second reading.

| file | description |
|------|-------------|
| [`zebra2-hints.ein`](zebra2-hints.ein) | `zebra2` with solution hints injected (S1.5a.11 diagnostic) |
| [`zebra2-minus-15.ein`](zebra2-minus-15.ein) | `zebra2` with condition (15) removed — a reduced, under-determined variant |
| [`gen_zebra2_variants.py`](gen_zebra2_variants.py) | generator for `zebra2` clue-dropped variants |

## Feature fixtures (per engine capability)

| dir | what it exercises |
|-----|-------------------|
| [`features/`](features/) | language features: `not`/`absent`, `*` in identifiers, `forall`, `open`, stdlib domain-elimination, the `__symmetric__` kernel mirror, the unstratifiable `p ← absent q; q ← absent p` (which pins that the NAF boundary admits **one** candidate per round — a batch would derive both), and two `(or …)` disjuncts with different guards and one binding key (the only fixture where a parked candidate is already fired when the boundary reaches it) |
| [`branching/`](branching/) | the hypothesis loop: saturate-only, dead/alive branches, multi-level, lookahead on/off, kill-cache on/off, `hrule`, hypothesis-relation whitelist, typed blind solve, the lookahead's NAF world and its unjudgeable guard (P1.21 R4 / S1.21.8 D3) |
| [`saturation/`](saturation/) | per-rule saturation demos by family — symmetric, transitive, `implies`, square fwd/bwd/unique, type-exclusivity, hypothesis-contradiction (see [`saturation/README.md`](saturation/README.md)) |
| [`lattice/`](lattice/) | commitment-lattice search: subset-pruned, genuine 3-set death, state-hash collision |
| [`domain_elim/`](domain_elim/) | domain-elimination vs hypothesis measurement fixtures (see [`domain_elim/README.md`](domain_elim/README.md)) |

## Diagnostics & negative fixtures

| dir | what it holds |
|-----|---------------|
| [`ein-bugs/`](ein-bugs/) | contradiction / bug-repro puzzles (`zebra2-bad.ein` — injected-fact contradiction; `mixed-type-hypothesis.ein` and `nested-fact-hypothesis.ein` — the two shapes of the `sorted(alive)` crash in `apriori.layer_1`, recorded rather than repaired: M1a Q-M1a.4 / D2, the second needing no mixed types at all; `unbound-relation-head.ein` — `(?R ?x)` with `?R` unbound, a crash-parity cell that passes; `int-goal-binding.ein` and `fact-goal-binding.ein` — the two goal-binding shapes the M1a S1a.6.6 fuzzer found and both engines were **fixed** for, an integer binding that must stay a JSON number and a nested-fact binding that used to crash `json.dumps`) |
| [`broken/`](broken/) | curated **parse-failure** fixtures; each expects a `file:line:col` error (bare top-level atom, keyword-as-value, rule missing params, unclosed paren) |
| [`broken/load/`](broken/load/) | curated **load-failure** fixtures — files that parse and then fail `KnowledgeBase.from_ir`; each carries the exact `KBLoadError` message in a `.expected` beside it (see [`broken/load/README.md`](broken/load/README.md)) |
| [`broken/compile/`](broken/compile/) | curated **compile-failure** fixtures — files that parse and load and then hit one of the four `CompileError`s S1.22.0 turned from a silent `return []` into an error; each carries the exact message in a `.expected` beside it (see [`broken/compile/README.md`](broken/compile/README.md)). `activator_arity.ein` is the odd one: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
