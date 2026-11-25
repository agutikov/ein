# Q-M2.15 — Where does M2 stop for the next iteration of Ein?

**Status:** open

**Raised 2026-11-25 by the user's second main idea**
([README](../../README.md#the-two-main-ideas-the-user-2026-11-25)): *"evaluate ein.rs in this auto-frmalization
application, find/highlight it's weaknesses, strength, gaps, bugs and limits, and then go to next iteration of
ein/ein.rs development with this feedback"*. **The user's, when there is something to decide on.**

The plan reaches Level D in ten phases. The idea says the milestone is also a measurement of Ein, after which Ein
is developed again; it does not say after which phase. What the plan does by default:

- [the evaluation of Ein](../../evaluation/README.md) is kept from the first stage on, every stage adding what it found,
  and is shown at each phase's close;
- **each level's gate is a stop**: at Level B ([P2.3](../../p2.3_benchmark/README.md)), Level C
  ([P2.7](../../p2.7_failure_scaling_generalization/README.md)) and Level D
  ([P2.10](../../p2.10_result_artifact_demo/README.md)) the evaluation is written up, and the user says whether EinAf
  goes on or an iteration of Ein comes first;
- between gates the kernel gains what the application needs of it and a bug is fixed, where Ein's syntax and
  semantics stay as they are ([Q-M2.16](../kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)); what would change the
  language waits for the iteration of Ein. A bug that is a wrong answer is filed in the wrong-answer register as
  it is found, whoever fixes it.

Nothing waits for this question: the first gate is the first time it has an answer to give.
