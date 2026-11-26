# L-M2.26 — A program the kernel refuses carries no categories

**Status:** open
**Found:** by [the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#2-the-acceptance-item-by-item),
2026-11-26, reading the one shot's sidecars; filed at the phase's close, 2026-11-27
**Owner:** [S2.2d.6](../../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) (the categories
from the parse); the sidecar's place in a record is [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md)'s

## What

The phase's acceptance asks that *every form of every recorded program is in exactly one row of the contract's
table of categories*. `einaf::contract` computes the categories after a load, so **a program the kernel refuses at
the load has an empty table**: of the 29 one-shot programs, 22 sidecars hold the categories and 7 hold none
(`library/more`, `library/unused`; `module-dir/boxes12`, `gap_fact`, `kinds`, `more`, `unused`). A refused program
parses, and the categories need the forms, the input text and the catalogue, not a loaded program.

## Evidence

`plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/*/*/program.sidecar.json`: the key `categories` is empty in the
seven; counted again at the close.

## What it costs

The acceptance item is held by test for the reference programs and is short for the model's. A refused program is
the one whose forms a repair most needs told apart (a source fact from a generated one), and the faithfulness judge
([S2.4.4](../../p2.4_loop/s2.4.4_faithfulness.md)) reads the categories.
