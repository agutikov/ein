# Compile-negative fixtures

Files that **parse** and **load** and then fail to **compile**: the shapes
[`ein-infer/src/compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs)
raises `CompileError` for. Each `<name>.ein` sits beside a `<name>.expected`
holding the exact message.

**Eight, in two batches.** S1.22.0's four are below; M1e
[S1e.2.1](../../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md)
added four more — `eq` and `absent`, each below and above its arity — which is
[§ The second batch](#the-second-batch-m1e-arity).

Sibling `../*.ein` fail at parse and `../load/*.ein` at load. The split is by
**where** a file stops being usable, and it is what lets a port gate on one
layer at a time — [P1a.1](../../../docs/history/m1a_rust/README.md#p1a1--ir-frontend)
on parse, [P1a.2](../../../docs/history/m1a_rust/README.md#p1a2--kb-core) on load,
[S1a.3.1](../../../docs/history/m1a_rust/README.md#s1a31--the-pattern-compiler) on
these.

## Why the first four exist at all

All four used to be a silent `return []`. S1.22.0 made them errors because a
dropped premise is not a smaller query — it is a **different** one, and wrong in
both directions:

- drop a premise and a plan can end up with **no** steps, which the matcher
  reads as one *vacuous* match, so the rule fires unconditionally;
- drop a premise inside an `(absent …)` and the sub-plan is empty, which
  matches vacuously too — so the guard fails against every possible KB, and
  because such a guard is `monotone`, its candidates are **retired
  permanently**.

`nested_or.ein` is the one that showed both polarities in a single shape:
`(and (a ?x) (or (p ?x) (q ?x)))` fired with neither `p` nor `q` in the KB
(unsound), and `(absent (or (p ?x) (q ?x)))` never fired with neither present
(incomplete).

## The odd one out

`activator_arity.ein` is the only fixture here whose CLI run **succeeds**, and
that is the point of it. S1.22.0 added *two* guards for the mismatch: the
`CompileError` below, and `Engine._activators_for`'s arity filter, which drops
the activator before the compiler ever sees it. The filter is what the engine
relies on, so the error is unreachable through the engine — reaching it needs
the unfiltered walk — `ein_infer::plan_shape_with(…, false)`, which is what
`utils/ir_oracle.py`'s `plan-shape` had a `"filter": false` for until
S1a.10.4 removed it. The fixture
therefore pins two things at once: the message a direct caller of
`compile_rule` gets, and — as an ordinary `positive` corpus entry — that
`(pairwise r)` derives nothing.

## The second batch (M1e: arity)

`CO-H1` was **one** shape — `(eq ?x)`, which panicked the process at exit 101
with a runtime `assert!`'s parity note for a message. S1e.1.6's sweep over
every kernel meta-primitive at every arity turned it into a rule:

> [`00_ebnf.md` §2](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) has a block
> headed **Kernel meta-primitives (shape-pinned)** with four productions —
> `NotForm`, `NeqForm`, `AndForm`, `OrForm`. The engine has **seven** such
> primitives. Every cell that panicked or silently misbehaved was one of the
> three the block does not name.

So `neq` at arity 0, 1, 3 and 4 is a positioned *parse* error and always was,
and `eq` — the same registry, the unpinned half — was not checked at all. The
four fixtures are that rule's whole surface:

| fixture | before S1e.2.1 |
|---|---|
| `eq_arity_low.ein` | **panic**, exit 101 |
| `eq_arity_high.ein` | **fires**, exit 0 — the tail past `args[1]` dropped |
| `absent_arity_zero.ein` | **silent** — the rule retired for the run, nothing said |
| `absent_arity_high.ein` | **fires**, exit 0 — everything past `args[0]` dropped |

The two that fired are the worse pair, and the half the review could not have
guessed from its one instance: a panic is loud and stops the run, and a guard
that quietly evaluates a weaker condition than the one written is a wrong
answer with a success exit code. All 21 cells of the sweep, defects included,
are
[`ein-cli/tests/primitive_arity.rs`](../../../ein.rs/crates/ein-cli/tests/primitive_arity.rs);
the three-candidate menu the fix was chosen from is
[Q-M1e.18](../../../plans/m1e_review_processing/open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments),
and S1e.2.1 took its (2) — check the arity where the form is read, leaving the
grammar and the lexer's `SYMBOL` set alone.

`false` is the third unpinned primitive and is **not** here: it is silent in a
`:match` at every arity, which is what a reader expects of it, so the sweep
found no cell to fix.

## Format

```
<name>.ein          the fixture — a file that loads and does not compile
<name>.expected     the exact `CompileError` message, one line
```

`{FILE}` is the fixture's own absolute path, and it appears in the four M1e
messages only. A premise is a `generic_list`, the one production the parser
hands a `Loc`, so a refusal of one can say **where** — `at Loc(file=…,
line=…, col=…)`, the loader's own form. The S1.22.0 four predate anyone asking
and end in nothing rather than in `at None`.

Consumers:
`ein.py/tests/inference/test_compile_negative.py` (gone since S1a.10.5)
and
[`ein.rs/crates/ein-infer/tests/compile_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/compile_semantics.rs)
— both suites, because either one alone can be the suite nobody ran.
