# P1a.5 — Presentation and CLI

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3 weeks (14 days of stages)
**Depends on:** [P1a.4](../p1a.4_search_layer/README.md)
**Blocks:** [P1a.6](../p1a.6_performance/README.md) — nothing is
optimised until the byte gate is closed.

## Goal

**The gate: T3, corpus-wide.** Every byte ein.py writes, ein.rs writes:
the solution table, `--stats`, `--timing`'s layout, `--print-final-*`,
the markdown trace with its inline DOT, all four `render` subcommands,
`saturate`'s output and `--dump`, the `--dump-states` tree, and every
`--help` and error message.

After this phase ein.rs *is* a drop-in replacement, and the milestone's
invariant I1 is discharged.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.5.1](s1a.5.1_dot_renderers.md) | DOT renderers | 4 d |
| [S1a.5.2](s1a.5.2_trace_and_answer.md) | Trace and answer rendering | 4 d |
| [S1a.5.3](s1a.5.3_state_dumps.md) | State dumps | 2 d |
| [S1a.5.4](s1a.5.4_cli.md) | The CLI | 4 d |

## Acceptance for the phase

- **T3 on the whole corpus × run matrix**, with only the closed
  normalisation list from [design/01](../design/01_parity_contract.md) §5
  applied.
- `plans/m1a_rust/divergences.md` is empty, or every entry has a written
  justification and a "what would make this unacceptable".
- All 19 checked-in goldens reproduce byte-for-byte.
- `ein.rs --help` and every subcommand `--help` match (or Q-M1a.13 is
  resolved the other way, in the ledger, deliberately).
- A user can `alias ein=ein.rs/target/release/ein` and every script in
  `utils/` keeps working — including `feature_matrix.py`,
  `profile_solve.py --no-profile`, `zebra2_trace.sh`,
  `render_examples.sh`.

## Notes

- This is the phase where `pyrepr` / `pyfmt`
  ([S1a.1.2](../p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md)) earn their
  keep: `explain`'s tie-breaks, `--dump-states`' node ordering, and the
  percentage columns all go through them.
- `state_hash.txt` and any `state_digest` in a message are on the
  normalisation list — ein.py itself is not stable there
  ([design/02](../design/02_determinism_and_order.md) §8).
- Rendering lives in `ein-render` alone, so the T3 surface is one crate.

## Cross-links

- [design/01 — Parity contract](../design/01_parity_contract.md) §2 T3
- [design/02 §7–§8 — `pyrepr`, hashes](../design/02_determinism_and_order.md)
- [`docs/kernel/ir/03-ein-lang/04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
