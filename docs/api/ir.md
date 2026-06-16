# `ein.ir` — parse, AST, dump

The S-expression front end: text → typed AST forms, and back. Source:
[`ein.py/src/ein/ir/`](../../ein.py/src/ein/ir/) (grammar in
[`grammar.lark`](../../ein.py/src/ein/ir/grammar.lark)).

> **Audience: embedders.** Most embedders treat the AST as opaque
> `SForm`s passed straight to [`ein.kb`](kb.md); you only need `parse`
> + `IRParseError`. The node types matter if you build IR
> programmatically or post-process it.

*Verified against commit `60c192b` (2026-06-16).*

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
