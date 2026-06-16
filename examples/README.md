# Ein examples

Encoded puzzles and focused fixtures, in [ein-lang](../docs/kernel/ir/03-ein-lang/).
Run any with `ein solve <file>` (or `ein saturate <file>` for the
saturation demos); see [`docs/api/`](../docs/api/) to drive them from Python.

> The step-by-step **human Zebra walkthrough** (the M1 target trace) used to
> live here; it moved to
> [`docs/kernel/inference/zebra_walkthrough.md`](../docs/kernel/inference/zebra_walkthrough.md).

## Zebra puzzle

| file | description |
|------|-------------|
| [`zebra.ein`](zebra.ein) | classic Zebra/Einstein puzzle (the M1 S1.1.1 parse smoke test) |
| [`zebra2.ein`](zebra2.ein) | Zebra, "B1" encoding (unified `is-a` / `*-loc`) — the **active M1 acceptance target** |
| [`zebra2-hints.ein`](zebra2-hints.ein) | `zebra2` with solution hints injected (S1.5a.11 diagnostic) |
| [`zebra2-minus-15.ein`](zebra2-minus-15.ein) | `zebra2` with condition (15) removed — a reduced, under-determined variant |
| [`gen_zebra2_variants.py`](gen_zebra2_variants.py) | generator for `zebra2` clue-dropped variants |

## Feature fixtures (per engine capability)

| dir | what it exercises |
|-----|-------------------|
| [`features/`](features/) | language features: `not`/`absent`, `*` in identifiers, `forall`, `open`, stdlib domain-elimination |
| [`branching/`](branching/) | the hypothesis loop: saturate-only, dead/alive branches, multi-level, lookahead on/off, kill-cache on/off, `hrule`, hypothesis-relation whitelist, typed blind solve |
| [`saturation/`](saturation/) | per-rule saturation demos by family — symmetric, transitive, `implies`, square fwd/bwd/unique, type-exclusivity, hypothesis-contradiction (see [`saturation/README.md`](saturation/README.md)) |
| [`lattice/`](lattice/) | commitment-lattice search: subset-pruned, genuine 3-set death, state-hash collision |
| [`domain_elim/`](domain_elim/) | domain-elimination vs hypothesis measurement fixtures (see [`domain_elim/README.md`](domain_elim/README.md)) |

## Diagnostics & negative fixtures

| dir | what it holds |
|-----|---------------|
| [`ein-bugs/`](ein-bugs/) | contradiction / bug-repro puzzles (`zebra2-bad.ein` — injected-fact contradiction) |
| [`broken/`](broken/) | curated **parse-failure** fixtures; each expects a `file:line:col` error (bare top-level atom, keyword-as-value, rule missing params, unclosed paren) |
