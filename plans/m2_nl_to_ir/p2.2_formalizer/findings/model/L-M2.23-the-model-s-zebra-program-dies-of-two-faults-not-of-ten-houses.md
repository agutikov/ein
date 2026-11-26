# L-M2.23 — The model's Zebra program dies of two faults, not of *ten houses for five places*

**Status:** closed: the records that said *ten houses for five places* are corrected in the commit that files this
**Found:** by [the phase report](../../report/README.md)'s re-runs, 2026-11-26, on `fd10c690`
**Owner:** — (what the faults say of the model is [L-M2.18](L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)'s and
[L-M2.21](L-M2.21-the-model-s-zebra-answer-query-asks-one-person-both-questions.md)'s, a repair's
[P2.4](../../../p2.4_loop/README.md)'s)

## What

S2.2.4's and S2.2.5's records, the phase README's stage row, E-125's and E-126's Zebra lines and EinAf's
[`03-feedback.md`](../../../../../docs/einaf/03-feedback.md) § 4 read the death of the model's Zebra program at step 2 as
*ten houses for five places*. **The program names eleven houses**: `House1` to `House5`, five coloured houses
(`RedHouse`, `GreenHouse`, `IvoryHouse`, `YellowHouse`, `BlueHouse`) and `MiddleHouse`. **Root dies of either of two
faults alone**:

- step 2's four kinds with no member (`Smoker`, `Drinker`, `Owner`, `Color`), over which it selected `std.slots`'
  partition, so `std.slots.slot-no-fill` refutes every place;
- step 1's coloured houses, with *The Norwegian lives next to the blue house.* read as `(lives-in Norwegian BlueHouse)`
  beside `(lives-in Norwegian House1)`: the Norwegian in two houses, `std.slots.slot-no-room`.

Less both, root is alive, owes 28 instances and has no state, its `(solve :no-search)` taking the obligations'
candidates away. The core's *Coffee is drunk in the green house.* is one of the eleven houses' sentences, the one the
core's search took; with `(config :record-alternative-justifications false)` (Q-M2.34) the core names *The Norwegian
lives next to the blue house.* instead. So E-125's strength stands as *a core names a misread sentence*, one of the
misreadings that kill root, not the only one.

## Evidence

[The report § 4.1](../../report/README.md#41-the-zebra-puzzle-from-its-text-to-the-kernels-answer): the model's
program (`from_text/runs/library/zebra/program.ein`) and three variants of it under `ein solve`, less the coloured
houses, less the empty kinds, less both (`report/out/ein/model-zebra-less-*.{ein,out}`, written by
`report/make_report.sh`).
