# Design contracts

The contracts `ein.rs` is built to: what each part of the engine guarantees, how it is laid out, and why. The crates
cite these pages as their specification; each states the engine as it is and names the tests that hold it.

## Reading order

Three groups, each constraining the next:

1. **The contract**: what may not change.
   [Determinism and order](01-determinism-and-order.md) → [Shared assets](02-shared-assets.md)
2. **The machine**: what changes underneath it.
   [Data model](03-data-model.md) → [IR frontend](04-ir-frontend.md) → [Matcher](05-matcher.md) →
   [Saturation](06-saturation.md) → [Search](07-search.md)
3. **The scale-out and the build.**
   [Parallelism](08-parallelism.md) → [Binary format](09-binary-format.md) →
   [Toolchain and layout](10-toolchain-and-layout.md)

The first group is not a preamble: every choice in the second is legal only because the first can prove it changed
nothing, so a claim such as *this preserves the match order* in [`05-matcher.md`](05-matcher.md) is enforced by
[`01-determinism-and-order.md`](01-determinism-and-order.md).

## What each page settles

| page | settles |
|---|---|
| [`01-determinism-and-order.md`](01-determinism-and-order.md) | what a run's output may depend on, the ordered containers, the order-sensitive sites and the structure that fixes each, the value renderer, the tests |
| [`02-shared-assets.md`](02-shared-assets.md) | one stdlib and one corpus: the stdlib's resolution, drift between its copies, the corpus manifest |
| [`03-data-model.md`](03-data-model.md) | symbols, `Value`, `FactId`, the fact store, the layered copy-on-write KB, the indexes, provenance, the record shapes |
| [`04-ir-frontend.md`](04-ir-frontend.md) | the generated parser, the AST arena, parse errors, macros, imports by full name, the loader, the dumper |
| [`05-matcher.md`](05-matcher.md) | a rule compiled per activator to a plan, the register machine, candidate selection, which optimisations are legal |
| [`06-saturation.md`](06-saturation.md) | the two-phase loop, the agenda, the delta and the boundary, a node resumed from root, firing and alternatives |
| [`07-search.md`](07-search.md) | the hypothesis set, the node function, the licences, each question's route, a state's identity, cores, the static check |
| [`08-parallelism.md`](08-parallelism.md) | the `--jobs` contract, what runs in parallel, what must be `Sync` and how |
| [`09-binary-format.md`](09-binary-format.md) | the `.einb` container: layout, ids across the boundary, versioning, the zero-copy reads |
| [`10-toolchain-and-layout.md`](10-toolchain-and-layout.md) | the workspace, the dependency policy and `unsafe`, profiles, features, the tests, the vendored parser runtime, `ein --version` |

## Conventions

- **A snippet of Rust is the code's**: a type, a field or a function named here exists under that name; a page that
  drifts from the code is wrong, and the code wins.
- **A size is the code's** and is named with the type or constant that fixes it, never a measurement of a run.
- **A decision is cited by its file** under [`docs/decisions/`](../../../decisions/README.md); a page states what
  holds and the decision says why it was chosen.
