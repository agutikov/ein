# Compile-negative fixtures

Files that **parse** and **load** and then fail to **compile**: the four
shapes [`ein/inference/compile.py`](../../../ein.py/src/ein/inference/compile.py)
raises `CompileError` for. Each `<name>.ein` sits beside a `<name>.expected`
holding the exact message.

Sibling `../*.ein` fail at parse and `../load/*.ein` at load. The split is by
**where** a file stops being usable, and it is what lets a port gate on one
layer at a time — [P1a.1](../../../plans/m1a_rust/p1a.1_ir_frontend/README.md)
on parse, [P1a.2](../../../plans/m1a_rust/p1a.2_kb_core/README.md) on load,
[S1a.3.1](../../../plans/m1a_rust/p1a.3_deductive_core/s1a.3.1_compiler.md) on
these.

## Why the four exist at all

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
the unfiltered walk (`utils/ir_oracle.py`'s `plan-shape` with
`"filter": false`, and `ein_infer::plan_shape_with(…, false)`). The fixture
therefore pins two things at once: the message a direct caller of
`compile_rule` gets, and — as an ordinary `positive` corpus entry — that
`(pairwise r)` derives nothing.

## Format

```
<name>.ein          the fixture — a file that loads and does not compile
<name>.expected     the exact `CompileError` message, one line
```

No placeholders: unlike a `KBLoadError`, none of these messages names a path.

Consumers:
[`ein.py/tests/inference/test_compile_negative.py`](../../../ein.py/tests/inference/test_compile_negative.py)
and
[`ein.rs/crates/ein-infer/tests/compile_parity.rs`](../../../ein.rs/crates/ein-infer/tests/compile_parity.rs)
— both suites, because either one alone can be the suite nobody ran.
