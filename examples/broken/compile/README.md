# Compile-negative fixtures

The place for files that **parse** and **load** and then fail to **compile**: the shapes
[`ein-infer/src/compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) raises `CompileError` for. Each
`<name>.ein` sits beside a `<name>.expected` holding the exact message.

**No program reaches a compile refusal.** The grammar states every primitive's shape (`eq` takes two terms,
`absent` one premise, `neq` two), and the loader refuses what the grammar cannot see: a premise that is not a
form, a relation variable no parameter binds, an `(or …)` written as a premise of an `and`, and a malformed
primitive a macro's expansion writes. Each of those refusals is on every command, the views and `kb save` among
them, and its fixture is a sibling: `../*.ein` fail at the parse, `../load/*.ein` at load. The compiler keeps its
refusals for a direct caller of `compile_rule`, and the corpus's `compile-negative` group has no entry.

## Why the compiler refuses at all

A dropped premise is not a smaller query: it is a **different** one, and wrong in both directions.

- Drop a premise and a plan can end up with **no** steps, which the matcher reads as one *vacuous* match, so the
  rule fires unconditionally.
- Drop a premise inside an `(absent …)` and the sub-plan is empty, which matches vacuously too, so the guard
  fails against every possible knowledge base.

`../load/nested_or.ein` shows both polarities in a single shape: `(and (a ?x) (or (p ?x) (q ?x)))` would fire
with neither `p` nor `q` present, and `(absent (or (p ?x) (q ?x)))` would never fire with neither present. A
shape that cannot be compiled faithfully is refused, never read as something weaker.

## The one file here

`activator_arity.ein` is a `positive` corpus entry whose run **succeeds**, and that is the point of it. Two guards
hold an activator to its rule's parameters: the `CompileError` in its `.expected`, and the arity filter of
`activators_for`, which drops the activator before the compiler ever sees it. The filter is what the engine relies
on, so the error is unreachable through the engine; reaching it needs the unfiltered plan,
`ein_infer::plan_shape_with(…, false)`. The fixture therefore pins two things at once: the message a direct
caller of `compile_rule` gets, and, as an ordinary corpus entry, that `(pairwise r)` derives nothing.

## Format

```
<name>.ein          the fixture — a file that loads and does not compile
<name>.expected     the exact `CompileError` message, one line
```

`{FILE}` stands for the fixture's own absolute path, and it appears where the refused form has a position: the
line starts `{FILE}:<line>:<col>: rule 'r': …`, as a parse error places itself. A premise has a position, a
top-level form none, so `activator_arity`'s line, about an activator, names the rule and no place.

Consumers: [`ein-infer/tests/compile_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/compile_semantics.rs),
and [`ein-cli/tests/primitive_arity.rs`](../../../ein.rs/crates/ein-cli/tests/primitive_arity.rs), which holds every
kernel primitive at every arity: a wrong arity is a positioned parse error.
