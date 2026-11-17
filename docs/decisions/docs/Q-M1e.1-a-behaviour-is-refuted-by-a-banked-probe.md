# Q-M1e.1 — A behaviour is refuted by a banked probe, an absence by naming its check, a risk by nothing

What it takes to refute a claim about Ein depends on what the claim is about:

| the claim is about… | it is refuted by… |
|---|---|
| a **behaviour**: *the engine does X* | an **executed probe, banked as a test** that fails if the behaviour appears; not a reading of the source, not an argument from the design |
| an **absence**: *nothing checks Y* | **naming the test or the page** that checks it |
| a **risk**: *X could happen* | **nothing**. A risk has three outcomes: fixed; accepted, with the argument written at the site ([Q-M1e.2](Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md)); or postponed, with what it waits for |

*"I could not build it"* is not a refutation. It is a probe that did not reproduce, and it is banked as one.

**Why.** A reading of the code that disagrees with a claim is the same kind of evidence as the reading that made the
claim. A probe banked as a test settles a behaviour for as long as the test runs; an absence is settled by the one
thing that would contradict it; and a risk is a statement about what could happen, which no account of what does
happen closes.

**Held by.** `docs/ein/standard-of-proof.md`, Rule 1, which states it and calibrates it; the pages that apply it,
[`monotonicity.md`](../../ein/reasoning/kb-evolution/monotonicity.md) (each claim beside the test that holds it) and
[`diagnostics.md`](../../ein/user/reference/diagnostics.md), what Ein refuses and prints when it does.
