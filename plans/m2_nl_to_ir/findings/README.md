# M2 — Findings

**What a finding of M2 is.** A thing the milestone finds that is neither a finding of Ein, which is
[the evaluation](../evaluation/README.md)'s, nor a question for the user, which is
[the open questions](../open_questions/README.md)': a thing of EinAf, of the plan, of the process, the gate or the
host, which the work has to know and nothing else holds.

**How it is kept.**

- **An id `L-M2.<n>`**, kept and never reused (M1f's findings were `L-n`, and are the followups' now). **One sequence
  for the milestone and its phases**: a phase takes the next free id, whichever folder it files in. **The next free id
  is L-M2.7.**
- **One file a finding**, `<topic>/L-M2.<n>-<slug>.md`, under a topic folder: its id and title, its status (open, or
  closed and by what), where it was found and its owner, then what was found (the record's words where a record
  states it), the evidence, and what it costs.
- **A phase keeps its findings in its own `findings/`** (every phase's folder has one). At the phase's close what is
  open moves here, under the same topic, and what is closed goes with the phase's folder, where `git log` keeps it;
  at the milestone's close what is open goes to [the followups](../../followups/README.md).

## Index

None found at the milestone's level: M2's README and [its analysis](../analysis/README.md) state no finding of this
kind that something else does not hold (the host's, for one, is [`analysis/host.md`](../analysis/host.md)'s). Those
below are P2.1's, moved here open at its close, 2026-11-26. L-M2.5 and L-M2.6 are P2.2's, filed by S2.2.0a
and S2.2.1 in [its findings](../p2.2_formalizer/findings/README.md).

| id | finding | topic | status |
|---|---|---|---|
| [L-M2.1](toolchain/L-M2.1-the-host-builds-with-rustc-1-100-not-the-pinned-1-97-1.md) | The host builds with rustc 1.100, not the pinned 1.97.1 | `toolchain` | open; the three lints it raised in the engine's code are closed by `acd04b13` |
| [L-M2.2](toolchain/L-M2.2-einaf-has-no-toolchain-pin-ci-builds-it-with-the-runners.md) | EinAf has no toolchain pin: CI builds it with the runner's stable | `toolchain` | open |
| [L-M2.4](einaf/L-M2.4-eins-api-change-leaves-einaf-unbuilt-until-its-own-commit.md) | Ein's API change leaves EinAf unbuilt until its own commit | `einaf` | open |

## In a phase's folder

One line a phase whose `findings/` holds entries, so that every id is found from here.

- **L-M2.1 to L-M2.4 were P2.1's**: the host's toolchain against the pin, EinAf's missing pin, EinAf's load judged
  with no bound, and Ein's API change leaving EinAf unbuilt until its own commit. At the phase's close, 2026-11-26,
  the three open moved [here](#index), and L-M2.3, closed in the phase by S2.1.3 (`0ab1aef0`), went with its folder
  (`git log --diff-filter=D -- plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation`).
- P2.2 to P2.10: none yet.
