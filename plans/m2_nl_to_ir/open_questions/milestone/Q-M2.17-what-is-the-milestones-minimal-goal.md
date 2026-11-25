# Q-M2.17 — What is the milestone's minimal goal?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25**, to the re-plan's note that a 4B model may not formalize the Zebra puzzle in one
shot: *"I just checked in ollama Qwen 3.5 started "Step-by-Step Deduction" in default thinking mode, but didn't
manage to find the solution and stopped (probably on some limit, I'm not aware about). So if this model with help
of Ein or through Ein will became able to solve the project's signature problem - It would be already success. So
this is a minimal goal."*

**Read as:** the milestone's minimal goal is that **the chosen model, which does not solve the Zebra puzzle by
reasoning in text, solves it with Ein**: *with the help of Ein*, the tool, or *through Ein*, the filter
([Q-M2.13](../einaf/Q-M2.13-who-calls-the-kernel.md)). Reaching it is already a success, whatever the tables then say. It is
not one shot: any arm that uses Ein counts, the loop included.

**What it changes in the plan:**

- **It is the first line of the milestone's acceptance**, before the levels, and the Level B criterion that asked
  for the Zebra puzzle in one shot is that goal's first attempt, not a bar of its own
  ([README](../../README.md#acceptance)).
- **The other side is taken as a record, not as an observation.** The user's run was one, through ollama, cut by
  a limit nobody set on purpose; in the host check ollama had loaded the model with a context of 4 096 tokens
  ([`analysis/host.md`](../../analysis/host.md)), which a step-by-step deduction of this puzzle may well exceed. So the
  model alone on the Zebra text is run by EinAf, thinking on, with the context and the budget stated in the
  record, first in [S2.2.2](../../p2.2_formalizer/s2.2.2_model_runtime.md)'s probe and again as arm B1 on the puzzle
  ([S2.5.2](../../p2.5_harness/s2.5.2_baselines.md)): *the model alone does not solve it* is then a claim with its
  evidence, and the goal a difference between two records.
- **Each stage that runs the model on the Zebra text says where the goal stands**: one shot
  ([S2.2.5](../../p2.2_formalizer/s2.2.5_from_text.md)), the filter's loop ([S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md)),
  the tool ([S2.4.6](../../p2.4_loop/s2.4.6_ein_as_a_tool.md)).
