# S1a.1.2 — AST arena, compatibility renderers, dumper

**Phase:** P1a.1 (IR frontend)
**Estimate:** 3 days
**Depends on:** [S1a.1.1](s1a.1.1_lexer_and_parser.md)
**Implements design:** [design/04](../design/04_ir_frontend.md) §§3, 7 ·
[design/02](../design/02_determinism_and_order.md) §7

## Context

The AST is an arena of `u32`-indexed nodes with `loc` in a side table, so
structural equality ignores positions exactly as ein.py's
`field(compare=False)` does — that is what makes
`parse(dump(parse(x))) == parse(x)` hold.

This stage also lands the two small compatibility modules the whole port
leans on: `pyrepr` (Python `repr` for `str`/`int`/`tuple`/`Fact`, used at
sort and display sites) and `pyfmt` (Python float formatting, Q-M1a.15).
They are trivial to write and expensive to discover missing at
[P1a.5](../p1a.5_presentation/README.md).

## Acceptance

- `dump_compact` and `dump_canonical` byte-identical to ein.py for every
  corpus file and for `tests/golden/{zebra,zebra2}.golden`.
- Round-trip property green on both sides (shared generator).
- `pyrepr` matches `repr()` for every value shape reachable from a fact
  arg, over a generated corpus including quotes, backslashes, control
  characters, non-ASCII, 1-tuples, empty tuples, nested `Fact`s.
- `pyfmt` matches Python's `f"{x:9.2f}"` / `{x:.1f}` / `{x:>5.1f}` over
  a wide `f64` corpus including `-0.0`, subnormals, `inf`, `nan`.

## Tasks

### Task T1a.1.2.1 — Arena AST

`Node` enum + `args` arena + `locs` side table, with builders that mirror
each grammar production's node shape — including the synthetic heads
(`@empty` for `()`, `@params` for rule/macro parameter lists) and the
**`loc = None` on every top-level form** (only `generic_list` carries
`head.loc`).

### Task T1a.1.2.2 — `pyrepr`

`str` (single-quote preferred, double when the body contains `'` and no
`"`, with `\\`/`\n`/`\r`/`\t`/`\xNN`/`\uNNNN` escapes), `int`, `tuple`
(including the `(a,)` one-tuple form and `()`), and the `Fact` dataclass
repr `Fact(relation_name='…', args=(…))`. Differential test against
CPython over generated values.

### Task T1a.1.2.3 — `pyfmt`

`f`-presentation with width, precision, alignment and sign, matching
CPython. Differential test.

### Task T1a.1.2.4 — Dumper

`_compact` and `_pretty` with the same `_DEFAULT_WIDTH` / `_INDENT`
constants and the same break rule (`indent*len(INDENT) + len(compact) >
width` ∧ `SForm` ∧ has args → head on its own line, args indented, `)`
appended to the last arg). `dump_canonical` over an iterable joins with a
blank line and appends a trailing newline when non-empty.
`escape_string_literal` in ein.py's replacement order.

## Notes

- The dumper is where an off-by-one in the width rule hides. Diff against
  `zebra2.golden` (293 lines) early and often; it exercises deep nesting
  and long `:why` strings.
- `pyrepr`/`pyfmt` live in `ein-core` because both `ein-infer`
  (explanation tie-breaks) and `ein-render` (output) need them.
