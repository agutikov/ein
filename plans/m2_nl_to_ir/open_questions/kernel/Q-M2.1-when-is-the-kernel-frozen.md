# Q-M2.1 — When is the kernel frozen?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"use current state - after M1f closed"*.

**Read as:** the instrument is the kernel as it stands now, after M1f's close (`458bf90b`). M2 is planned and run
against it and does not wait for a kernel milestone that is queued, unless the user says so
([Q-M2.12](../answer/Q-M2.12-what-prints-the-answer.md) asks about one). The dependencies this entry named are spent: M1c and
M1d are closed, and the engine says *not decided* in its own words, so the loop derives no `unknown`.

**What stays of the entry**, for [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md)'s re-plan: a record carries the
commit of Ein it was taken on (from git: `ein --version` prints none), the stdlib manifest's SHA and the resolved
configuration, so that a number says which kernel it came from. **What changes the kernel inside M2** is what a
ruling here asks for and nothing else: today that is the machine surface for diagnostics
([Q-M2.9](Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md)), which changes no answer.
