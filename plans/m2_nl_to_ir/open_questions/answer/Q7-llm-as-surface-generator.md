# Q7 — LLM as surface generator?

**Status:** open

Per [idea 08 §Open questions point 4](../../../ideas/08-human-style-deductive-trace.md#open-questions).

**Options:**

- **A**: Pure templates. Deterministic. Verifiable but stilted.
- **B**: LLM polish on top of templates. Drift risk.
- **C**: LLM end-to-end from `TraceStep` to prose. Most natural;
  worst verifiability.

**Working answer**: A for the *reasoning* layer (no LLM mid-proof);
B for the surface narration (only at the end, with a "render-only"
prompt that cannot change the rule firings). C never.

**Status after the reshape (2026-08-23):** the question is real and is not
this milestone's. The research plan's loop ends at *verified answer*
([`EinAf.md` § Stage Q](../../EinAf.md#stage-q--public-research-artifact)) and the
trace → NL direction appears nowhere in Stages A–Q; the old plan carried it as
a "stretch goal". It is the natural last slide of
[M5](../../../m5_presentation/README.md)'s demo and a thing
[M20](../../../m20_gui/README.md) would display, and it is parked here with its
working answer until one of them asks for it. The engine's `:why` templates
already render every fact and the headline (rendering A, shipped); what is
open is only B.

**Since (the user's answers, 2026-11-25).** The user's statement of the core idea
([README](../../README.md#the-two-main-ideas-the-user-2026-11-25)) asks that *"the final test problem answer must be short and
must be produced by ein program NL output"*. That is rendering A, the engine's own templates, and it puts the
answer's words inside the research question; what stays parked here is B, an LLM's polish. Ein to natural language
is [M1p](../../../m1p_ein_to_natural_language/README.md)'s, which does not run now: the answer is what Ein prints today
with EinAf's formatters over its values, the user's ruling on [Q-M2.12](Q-M2.12-what-prints-the-answer.md).
