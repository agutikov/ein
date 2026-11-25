# Q-M2.16 — What of the kernel may change inside M2?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"we can change kernel without changing ein language syntax and semantics, any
additional kernel features (logs, traces, measurement, timeouts, limits, etc) can be added; bug fixes allowed"*.

The question was raised by the re-plan as a narrower one, *is a saturation bounded inside the kernel?*: nothing in
a solve stops a saturation that has no fixpoint ([`grow.ein`](../../analysis/probes/p2.7/grow.ein), E-45 of
[the evaluation](../../evaluation/README.md)), and the re-plan's default was to change nothing in the kernel and have EinAf
stop it from outside. `git show cce0f0ff:plans/m2_nl_to_ir/open_questions.md` has the two options.

**Read as:** inside M2 the kernel may be changed wherever **Ein's syntax and semantics stay as they are**.

- **Allowed, with no further ruling:** a feature that makes the kernel say, measure or bound more (a log, a trace,
  a measurement, a timeout, a limit, a key of a machine surface, a value of the library), and **a bug fix**. Each
  is made under Ein's rules, as [S2.1.2](../../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md) is: its
  pages and tests in the same commit, the gate, a decision file where it decides something, and no word of EinAf
  in Ein.
- **Not allowed by this:** a change to the language (a form, a keyword, a name the kernel reads) or to what a
  program means and what its answer is. That waits for the iteration of Ein that follows
  ([Q-M2.15](../milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)), or for a ruling of its own.
- **It supersedes** the re-plan's default that the kernel changes inside M2 *only where a ruling says so*, and the
  sentences of the plan that called S2.1.2 the milestone's one kernel change.

**What it changes in the plan:**

- **A saturation is bounded by the kernel.** The clock (`-T`) and the budget the kernel already has reach inside a
  saturation, and a run they stop says what a stopped run says today: no new word and no new outcome. It is a task
  of [S2.1.2](../../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md). The word of its own for *did not
  stop*, a bound that is the saturation's own, and a static check are
  [M1g](../../../m1g_logic/README.md)'s, in its stage on a saturation that does not stop, with the user's question
  there.
- **EinAf calls the kernel in its own process**, as ruled
  ([Q-M2.7](../einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)). A worker process is built only if a probe shows a
  case the kernel's own bounds do not stop ([S2.1.3](../../p2.1_kernel_as_instrumentation/s2.1.3_feedback_object.md)).
- **An entry of the evaluation that is a gap or a bug may be closed inside M2**, by the stage that needs it, and
  the entry then says so. An entry nothing in M2 needs stays for the iteration of Ein.
