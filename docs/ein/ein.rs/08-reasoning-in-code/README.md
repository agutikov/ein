# Reasoning in code

The pages of [`reasoning/`](../../reasoning/README.md) state Ein's theory with no word on an implementation: their
languages are mathematics, graphs and words
([Q-M1f.370](../../../decisions/docs/Q-M1f.370-each-perspective-of-the-documentation-is-written.md)). These pages are
the other half. For each page of the reasoning they say **where `ein.rs` implements it**, **what holds `ein.rs` to
it**, and **which commands show it**: the module, type or function a definition lives in, the test that fails
when a proposition stops being true of the engine, and the `ein` command lines that print or draw the thing.

| page | the reasoning it follows |
|---|---|
| [`01-static-ir.md`](01-static-ir.md) | [`reasoning/01-static-ir/`](../../reasoning/01-static-ir/): names and atoms, relations and facts, values and their order, rules as statements |
| [`02-saturation-and-negation.md`](02-saturation-and-negation.md) | [`reasoning/02-kb-evolution/`](../../reasoning/02-kb-evolution/), the knowledge base as it saturates: firing, the fixpoint and the agenda, negation as failure |
| [`03-hypotheses-and-the-answer.md`](03-hypotheses-and-the-answer.md) | [`reasoning/02-kb-evolution/`](../../reasoning/02-kb-evolution/), the hypothesis set and the region: generation, the hypothesis space, the region R and the routes over it, monotonicity, cores, satisfiability |
| [`04-rule-evolution.md`](04-rule-evolution.md) | [`reasoning/03-rule-evolution/`](../../reasoning/03-rule-evolution/): rules as data, analysis of rules, transformations of a rule set |

## How a page is laid out

One section for each page of the reasoning, under that page's title, and in each section:

- **Where it lives**: the notion beside the code that implements it, a module by its path under
  [`ein.rs/crates/`](../../../../ein.rs/crates/) and an item by its Rust path. The modules themselves are mapped in
  [`01-architecture.md`](../01-architecture.md), and the contracts they keep are the
  [design pages](../03-design/README.md).
- **What holds it**: each claim the reasoning makes of *any* implementation, beside the test of this one that
  fails when the claim stops being true. A test is named as `file`: `function`; a unit test is named with the
  source file it stands in. `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p <crate> --test <file>`
  runs one integration test file, and `cargo test … -p <crate> --lib` a crate's unit tests.
- **Commands that show it**: what to run to see the thing, and where a drawing on a page of the reasoning comes
  from. Every drawing there that the engine can draw is the DOT one of these commands writes, trimmed.

A claim with no test is not listed as held: the reasoning states it as a definition or an argument
([Q-M1e.1](../../../decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md),
[Q-M1e.2](../../../decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md)).
