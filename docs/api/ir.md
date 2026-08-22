# `ein.ir` — parse, AST, dump

> ### ⚠ This contract has no implementation, and none is scheduled
>
> **`import ein` does not work in this repo.** The Python package these pages
> describe was deleted at M1a
> [S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
> (2026-08-21), when `ein.rs` became the only engine.
>
> A PyO3 module was to succeed it in
> [P1a.9](../../plans/m1a_rust/p1a.9_release/README.md). **That is deferred as
> of 2026-08-21** — the census found no consumer that needs it, and
> [Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
> records the three conditions that would bring it back.
>
> So these pages are **history, held in reserve**: the embedding contract of
> the engine that was, kept whole rather than deleted, because on the day a
> trip-wire fires this is a specification instead of a blank page. Read every
> code block as a record, not as a runnable snippet — and do not "fix" one to
> match ein.rs's internals; they describe something that no longer exists.
>
> **The surfaces that do run** are the CLI — `ein solve <file>` ·
> `ein saturate` · `ein render` — and the crates, whose embedding page
> [S1a.9.4](../../plans/m1a_rust/p1a.9_release/s1a.9.4_documentation.md)
> writes.

The S-expression front end: text → typed AST forms, and back. The engine
behind it is [`ein-ir`](../../ein.rs/crates/ein-ir/src/); the grammar is
[the EBNF](../kernel/ir/03-ein-lang/00_ebnf.md).

> **Audience: embedders.** Most embedders treat the AST as opaque
> `SForm`s passed straight to [`ein.kb`](kb.md); you only need `parse`
> + `IRParseError`. The node types matter if you build IR
> programmatically or post-process it.

*Verified against commit `60c192b` (2026-06-16) — **against the Python engine, which no longer exists**. These signatures are a record of what that engine offered, not a description of anything in the tree and no longer a contract anything is scheduled to implement ([Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).*

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
# TODO: What lark in Rust?

Escape hatch returning the raw Lark parse tree before AST lowering.
Internal/diagnostic — embedders want `parse`.

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
