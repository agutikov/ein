# `ein.ir` — parse, AST, dump

<!-- api-history-banner -->
> ### 🏛 History — the embedding contract of the engine that was
>
> **This page describes a Python package that no longer exists**, and it is
> filed as a record rather than as a promise. `ein.py/` was deleted at M1a
> [S1a.10.5](../history/m1a_rust/README.md#s1a105--the-removal)
> (2026-08-21); the PyO3 module that was to succeed it was **deferred the same
> day** for want of a consumer, with three trip-wires recorded in
> [Q-M1a.23](../history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding).
>
> It is kept **whole and unedited** for one reason: a deferral is cheap to
> reverse only while the specification survives it. On the day a trip-wire
> fires, this is a contract to implement instead of a blank page. So read
> every code block as a record — and **do not "fix" one to match `ein.rs`'s
> internals.** A page rewritten to describe the current engine would be
> neither history nor a specification.
>
> **The embedding surface that exists is Rust**, and it is
> [`rust.md`](rust.md) — the crates, whose worked example is a test the gate
> runs. The other surface that runs is the CLI: `ein solve <file>` ·
> `ein test` · `ein saturate` · `ein render` · `ein kb` (`ein --help`,
> [`docs/install.md`](../install.md)).
<!-- /api-history-banner -->

The S-expression front end: text → typed AST forms, and back. The engine
behind it is [`ein-ir`](../../ein.rs/crates/ein-ir/src/); the grammar is
[the EBNF](../kernel/ir/03-ein-lang/00_ebnf.md).

> **Audience: embedders.** Most embedders treat the AST as opaque
> `SForm`s passed straight to [`ein.kb`](kb.md); you only need `parse`
> + `IRParseError`. The node types matter if you build IR
> programmatically or post-process it.

*Verified against commit `60c192b` (2026-06-16) — **against the Python engine, which no longer exists**. These signatures are a record of what that engine offered, not a description of anything in the tree and no longer a contract anything is scheduled to implement ([Q-M1a.23](../history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).*

## Parsing

### `parse(text, *, filename=None) -> tuple[SForm, ...]`

Parse S-expression source into a tuple of top-level forms. `filename` is
used only for error locations. Does **not** read files or resolve
`(import …)` — that is the loader's job (see [`kb.md`](kb.md)). Raises
`IRParseError` on malformed input.

```python
from ein.ir import parse
forms = parse("(relation likes Person Thing)\n(likes Alice Tea)")
```

### `parse_tree(text) -> lark.Tree`

Escape hatch returning the raw Lark parse tree before AST lowering.
Internal/diagnostic — embedders want `parse`.

> **This symbol does not survive**, and the working tree carried a
> `TODO: What lark in Rust?` here until S1a.9.4 answered it: there is no
> answer, because there is no parser generator.
> [design/04 §1](../history/m1a_rust/design/04_ir_frontend.md) rules one out
> — `ein-ir` is a hand-written lexer and recursive-descent parser, and its
> only exposed tree is the AST `parse` returns. A PyO3 successor would not
> reintroduce a Lark tree to have something to put here; it would drop the
> function. Recorded rather than deleted, because what a restored contract
> must *not* promise is as much a part of it as what it must.

### `IRParseError`

Exception raised by `parse` / `parse_tree` on a syntax error.

## AST nodes

All frozen dataclasses from `ein.ir.types`; round-trip through `dump`
modulo `Loc`. You rarely construct these by hand — `parse` produces them
and [`KnowledgeBase`](kb.md) consumes them.

| node | what it is |
|------|------------|
| `SForm` | an S-expression: `head` (an `Atom`) + `args` tuple. The unit a KB form is built from. |
| `Atom` | a bare name / symbol (`relation`, `likes`, `Alice`, `true`). |
| `Var` | a `?name` pattern variable. |
| `Keyword` | a `:keyword`. |
| `KwPair` | a `:key value` pair (`.key`, `.value`). |
| `Wildcard` | the `_` placeholder. |
| `String` | a `"quoted"` literal (`.value`). |
| `Int` | an integer literal (`.value`). |
| `Range` | an `N..M` numeric range. |
| `Loc` | source location metadata (excluded from equality). |
| `IRNode` | the base type all of the above share. |

## Dumping (round-trip)

`dump`, `dump_canonical`, `dump_compact` — render forms back to text.
`parse(dump_canonical(forms))` round-trips modulo `Loc`. Useful for
normalising or re-emitting IR; not needed for the solve flow.

```python
from ein.ir import parse, dump_canonical
print(dump_canonical(parse("(likes Alice Tea)")))
```

## Not the contract

`ein.ir.to_dot` (+ the `render_*` helpers) render IR to Graphviz DOT — a
visualisation utility, documented with the rest of rendering, not the
embedding flow.

## See also

- [`kb.md`](kb.md) — what consumes these forms.
- [`docs/kernel/ir/03-ein-lang/`](../kernel/ir/03-ein-lang/) — the
  *language* these nodes encode (grammar, patterns, reserved names).
