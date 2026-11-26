# Q-M2.42 — Are E-104 and E-105 fixed before the benchmark is built?

**Status:** open

**Raised 2026-11-26 by [the analysis of P2.2](../../p2.2_formalizer/report/ANALYSIS.md#94-what-the-library-and-the-kernel-owe)**
(its proposal 14), and by the routing of the phase's results at its close, 2026-11-27.

**The question.** P2.2 filed two bugs of Ein with no owner.
[E-104](../../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md): a relation
declared a second time is refused where the program imports a module, against Q-M1f.447; it cost the model's
`unused` in both conditions. [E-105](../../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md):
`std.slots` lets two values stated next to each other share a house, so an under-determined puzzle over it answers
55 states where 32 is right. A fix of either leaves Ein's syntax and semantics as they are
([Q-M2.16](Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)), and by [the frame's default 11](../../README.md#the-defaults-the-re-plan-took)
each such change is a task of the stage that needs it. Which stage, and when?

**The default taken at the close**: a short phase of their own,
[P2.2e](../../p2.2e_kernel_fixes/README.md), before P2.3, planned and not started.

**Why before the benchmark** (the coordinator's inference, not run): the benchmark's attribute programs follow
the formalizer's target shape ([S2.3.1](../../p2.3_benchmark/s2.3.1_families_and_seed_set.md)), which selects
`std.slots`. The claims of an under-determined instance would then be written against 55 states, its generator's
own count would say 32, and a fix after the splits are frozen changes content the digests are taken over.

**The alternatives.**

1. **A phase before P2.3** (the default).
2. **Tasks of the stages that need them**: E-105 the first task of S2.3.1, E-104 of the stage that first runs the
   model on the benchmark, [E-126](../../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md)'s
   clock of [S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md).
3. **The iteration of Ein**: the benchmark's attribute programs written over `std.bijection`, whose count is
   right, and the model's programs over `std.slots` judged against it as they are.

**What bears on it.** [Q-M2.34](Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md):
with the lever set by EinAf's call, E-126's cost to the application goes without a change of the kernel;
[Q-M2.24](Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md), a test and a decision the same
phase can write.
