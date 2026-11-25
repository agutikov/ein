# Q-M2.13 — Who calls the kernel?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25**, to the coordinator's reading that both are arms of one experiment, compared
with a chain of thought: *"yes, it is correct, "tool for the LLM" - means LLM can decide to use it on it's own, it
is recommened but optional, also qwen model (or other suitable models) most probably not aware about this
language, was not trained for synthesis of ein programs, newertheless ein semantic is very simalar to Datalog,
syntax is Lisp/SMT style, so writing ein program may be not impossible without GBNF; "filter of thoughts" - means
it is forced to write ein code and can not avoid it"*.

The question was raised by the user's statement of the core idea
([README](../../README.md#the-two-main-ideas-the-user-2026-11-25)): *"On one hand ein works like a tool for LLM, on
other hand it works like filter of thoughts"*.

**Read as:** two arrangements of one model and one kernel, both arms of the experiment, each compared with the
model reasoning in text ([Stage D](../../EinAf.md#stage-d--establish-baselines)'s B1).

| | **the tool** | **the filter** |
|---|---|---|
| who decides that Ein runs | the model, when it likes; the prompt recommends it, and the model may answer without it | the harness, on every generation |
| what the model writes | text, and an Ein program where it chooses to call | an Ein program and nothing else: it cannot avoid it |
| what holds the syntax | nothing: the model writes Ein as it can, and a parse error is what the tool returns | the harness accepts a program only; a grammar over the output makes that hold token by token ([Q-M2.6](../formalizer/Q-M2.6-one-gbnf-or-many.md)) |
| the answer | the model's last message; whether it is what its last run of Ein printed is recorded | what the Ein program prints ([Q-M2.12](../answer/Q-M2.12-what-prints-the-answer.md)); with no program the kernel passes there is no answer |
| in the plan | new: [S2.4.6](../../p2.4_loop/s2.4.6_ein_as_a_tool.md), and B6 of [S2.5.2](../../p2.5_harness/s2.5.2_baselines.md) | the plan's loop: [S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md), B2 to B5 |

**The user's remark on the model is a hypothesis the plan measures, not a premise it takes**: a model not trained
on Ein may still write it, the semantics being near Datalog's and the syntax Lisp's and SMT-LIB's. So *the model
writes Ein with no grammar* is measured where it is cheapest, as the first thing the formalizer's phase does
([S2.2.2](../../p2.2_formalizer/s2.2.2_model_runtime.md)), and the filter with its grammar off is an ablation
([P2.6](../../p2.6_ablations/README.md)).

**What the ruling leaves to the plan, each a default:** what the tool replies is a level of the feedback ladder,
the same object the filter renders ([S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md)); a tool episode is priced in
generated tokens with its calls counted, so that it compares with the others under
[Stage D](../../EinAf.md#stage-d--establish-baselines)'s *same model, similar budget*; the tool is a function the
harness declares to the model in its own process, and nothing of [M15](../../../m15_mcp_server/README.md), an MCP
server, starts by it.
