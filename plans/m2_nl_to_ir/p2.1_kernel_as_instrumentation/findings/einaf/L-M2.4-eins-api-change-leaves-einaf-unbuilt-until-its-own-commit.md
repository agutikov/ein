# L-M2.4 — Ein's API change leaves EinAf unbuilt until its own commit

**Status:** open
**Found:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md#einafs-build), in both halves, 2026-11-25; filed from the
records by the restructuring of M2's ledgers
**Owner:** none

## What

A commit of Ein's that changes a public type of the library breaks EinAf's build, and EinAf's own commit, which
follows, mends it: between the two, master holds an Ein whose EinAf step does not compile. It happened twice in the
phase:

- **`83b82833`** added `explore::Stopped::Steps`, and EinAf's `match &a.stopped` in `einaf/src/kernel/outcome.rs` is
  exhaustive. S2.1.2's record, verbatim: *"A downstream exhaustive `match` on `Stopped` needs one arm, and EinAf's
  `einaf/src/kernel/outcome.rs` has one (`match &a.stopped`), so the merge needs, beside its `Deadline` arm: …"*. EinAf's
  arm is `d61176a3`, two commits later (`fd2fcd48` between).
- **`62a8ee45`** made `TextError::Parse` carry a `ParseError`. S2.1.2's record, verbatim: *"`cargo build
  --manifest-path einaf/Cargo.toml --profile gate --all-targets` breaks at one place, `einaf/src/kernel/mod.rs:402`,
  `refusal_of_load`'s parse arm (*expected `String`, found `ParseError`*)."* EinAf's fix is `d13dc4d9`, four commits
  later (`95f5dc49`, `e63a21b0`, `e74139cf` between).

## Evidence

`git log --oneline 83b82833^..d61176a3` and `git log --oneline 62a8ee45^..d13dc4d9`; `git show
83b82833:einaf/src/kernel/outcome.rs` has no wildcard arm in its `match &a.stopped`. The gate ran once on the batch's
last commit ([S2.1.2](../../s2.1.2_diagnostics_surface.md)'s *Tests run*: *"No `./run_tests.sh` … the coordinator runs
the phase's gate"*), so no gate run read the commits between.

## What it costs

A commit of master between the two fails the gate's EinAf step, so a bisection over master, or a gate run on such a
commit, reads a failure that is not the commit's subject. [S2.1.4](../../s2.1.4_bugs_the_phase_found.md) plans the same
order again (*"T2.1.4.6 — EinAf … a commit of its own, after Ein's"*); it ran so, and its Ein commit, `df75643c`,
changed no API EinAf names (`grep -rn 'repr_value\|repr_fact\|bindings_repr\|RefArg' einaf/` finds none), so EinAf
built on it and its comparison, which stripped the repository's root from both sides, still agreed. The order follows from the projects being two;
whether one commit may carry Ein's change and EinAf's mend together is the process's to say.
