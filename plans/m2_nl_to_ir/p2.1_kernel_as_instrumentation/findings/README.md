# P2.1 — findings: the phase's

What the phase finds that is neither a finding of Ein ([its evaluation](../evaluation/README.md)) nor a question for
the user ([its open questions](../open_questions/README.md)): of EinAf, the plan, the process, the gate, the host. One
file a finding, `<topic>/L-M2.<n>-<slug>.md`, its id the next free of the milestone's one sequence, which
[the milestone's findings](../../findings/README.md) names.

**At the phase's close** what is open moves to [the milestone's `findings/`](../../findings/README.md), under the same
topic; what is closed goes with the phase's folder.

## Index

Read off the four stages' records and [the phase report](../report/README.md): what they state of the host, the toolchains, EinAf's build and the gate that is neither an entry of the evaluation nor a question.

| id | finding | topic | status |
|---|---|---|---|
| [L-M2.1](toolchain/L-M2.1-the-host-builds-with-rustc-1-100-not-the-pinned-1-97-1.md) | The host builds with rustc 1.100, not the pinned 1.97.1 | `toolchain` | open; the three lints it raised in the engine's code are closed by `acd04b13` |
| [L-M2.2](toolchain/L-M2.2-einaf-has-no-toolchain-pin-ci-builds-it-with-the-runners.md) | EinAf has no toolchain pin: CI builds it with the runner's stable | `toolchain` | open |
| [L-M2.3](einaf/L-M2.3-einafs-load-judged-the-hypothesis-mechanisms-with-no-bound.md) | EinAf's load judged the hypothesis mechanisms with no bound | `einaf` | closed by [S2.1.3](../s2.1.3_feedback_object.md#what-was-built) (`0ab1aef0`) |
| [L-M2.4](einaf/L-M2.4-eins-api-change-leaves-einaf-unbuilt-until-its-own-commit.md) | Ein's API change leaves EinAf unbuilt until its own commit | `einaf` | open |
