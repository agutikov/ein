# Compile-negative fixtures

Files that **parse** and **load** and then fail to **compile**: the shapes
[`ein-infer/src/compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs)
raises `CompileError` for. Each `<name>.ein` sits beside a `<name>.expected`
holding the exact message.

**One since M1f S1f.5.24 (2026-11-10)**: `activator_arity`, a `positive` whose
error only a direct caller of `compile_rule` reaches. `eq_in_a_macro_body`
moved to `../load/` (W66): the loader holds a macro's expansion to the
grammar's shapes since, so it refuses `(eq ?x)` written in a macro's body where
the macro is called, on every command, the views and `kb save` among them,
and no program reaches a compile refusal; the `compile-negative` group has no
corpus entry.

**Two from M1f S1f.5.21 (2026-11-10) until then**: `activator_arity` and
`eq_in_a_macro_body`. `nested_or` moved to `../load/` (W62): the loader refuses
an `(or …)` written as a premise since, on every command, where the compiler
refused it only where it compiled the rule, in the Python IR's spelling, and
four commands that never compiled it answered. `eq_in_a_macro_body` is the one
compile refusal a program still reaches, `(eq ?x)` written in a macro's body,
which the grammar never reads expanded (S1f.5.24's, on the user's ruling on
Q-M1f.67); its line is in Ein's words on every command that compiles, the
place in the macro's body first, and the views and `kb save` accept the
program, which none of them executes.

**Two from M1f S1f.5.5's swap (T1f.5.5.4, 2026-11-09) until S1f.5.21**:
`nested_or` and `activator_arity`. The grammar states `eq`'s and `absent`'s shapes, `eq`
taking two terms and `absent` one premise, so the second batch below is
refused at the parse, and its four fixtures moved to `../` beside the
parse-negatives (Q-M1f.2 (a)), each `.expected` the parse report; what
follows is their record.

**Six, in two batches, until then.** S1.22.0's four are below, two of them
here since M1e2 S1e2.2.24; M1e
[S1e.2.1](../../../docs/history/m1e_review_processing/README.md#s1e21--correctness-the-panic-the-guard-the-traversal)
added four more — `eq` and `absent`, each below and above its arity — which is
[§ The second batch](#the-second-batch-m1e-arity).

**Two moved to `../load/` on 2026-11-05** (M1e2 S1e2.2.24, W24, the user's
ruling on [Q-M1e2.18](../../../plans/m1e2_correctness/open_questions.md#q-m1e218--what-else-in-a-rule-reads-nothing-in-silence)
(v)): `empty_absent.ein`, `(absent ?x)`, and `unbound_head.ein`, `(?R ?x)`
with `?R` no parameter. The loader refuses a premise that is not a form and a
relation variable no parameter binds since that stage, so neither reaches the
compiler, and each `.expected` is its load line now. The compiler keeps both
refusals for a direct caller of `compile_rule`, which this directory no longer
pins.

Sibling `../*.ein` fail at parse and `../load/*.ein` at load. The split is by
**where** a file stops being usable, and it is what let M1a gate on one
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

`nested_or.ein` (under `../load/` since M1f S1f.5.21) is the one that showed
both polarities in a single shape: `(and (a ?x) (or (p ?x) (q ?x)))` fired
with neither `p` nor `q` in the KB (unsound), and `(absent (or (p ?x) (q
?x)))` never fired with neither present (incomplete).

## The odd one out

`activator_arity.ein` is the only fixture here whose CLI run **succeeds**, and
that is the point of it. S1.22.0 added *two* guards for the mismatch: the
`CompileError` below, and `activators_for`'s arity filter, which drops the
activator before the compiler ever sees it. The filter is what the engine
relies on, so the error is unreachable through the engine — reaching it needs
the unfiltered walk, `ein_infer::plan_shape_with(…, false)`. The fixture
therefore pins two things at once: the message a direct caller of
`compile_rule` gets, and — as an ordinary `positive` corpus entry — that
`(pairwise r)` derives nothing.

## The second batch (M1e: arity)

`CO-H1` was **one** shape — `(eq ?x)`, which panicked the process at exit 101
with a runtime `assert!`'s internal message for a diagnostic. S1e.1.6's sweep over
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
[Q-M1e.18](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments),
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

`{FILE}` is the fixture's own absolute path, and it appears where the refused
form has a position: since M1f S1f.5.21 the line starts with it,
`{FILE}:<line>:<col>: rule 'r': …`, as a parse error places itself, where it
ended `at Loc(file=…, line=…, col=…)` until then. A premise has a position, a
top-level form none, so `activator_arity`'s line, about an activator, names
the rule and no place.

Consumers:
[`ein.rs/crates/ein-infer/tests/compile_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/compile_semantics.rs)
and the corpus sweep, whose `compile-negative` group is this directory less
`activator_arity.ein`; and `ein-cli`'s
`cli_semantics::a_compile_refusal_reads_the_same_on_every_command`, which
holds `eq_in_a_macro_body`'s line on every command that compiles.
