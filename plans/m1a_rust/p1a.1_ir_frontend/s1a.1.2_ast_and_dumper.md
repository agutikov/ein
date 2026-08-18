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

---

## Outcome — 2026-08-18

The arena landed with [S1a.1.1](s1a.1.1_lexer_and_parser.md) (the parser has
to build something). This stage is the three renderers and the gate that
proves them: `ein-core::pyrepr`, `ein-core::pyfmt`, `ein-ir::dump`.

| acceptance item | result |
|---|---|
| `dump_compact` / `dump_canonical` byte-identical for every corpus file | 91 files (the four parse-negative fixtures excluded), 0 differences, both renderings |
| … and for `tests/golden/{zebra,zebra2}.golden` | reproduced exactly, compared against the **files** and not only through the oracle |
| round-trip property green on both sides | `parse(dump(parse(x))) == parse(x)` structurally in ein.rs, and `dump ∘ parse` is a textual fixed point in *both* implementations over the same inputs |
| `pyrepr` matches `repr()` over a generated corpus | 40 value shapes + a 1 700-code-point sweep, 0 differences |
| `pyfmt` matches `f"{x:9.2f}"` &c. over a wide `f64` corpus | 230 values × 19 specs = 4 370 formattings, 0 differences |

The dumper was byte-identical on the first run — unusual, and worth saying
plainly: the transcription had no bugs because `dump.py` is 124 lines of
straight-line rendering with one rule in it, and that rule was copied rather
than re-derived.

### `repr` needs a Unicode table, and it must be *CPython's*

`repr(str)` escapes every non-printable character, and CPython's
`Py_UNICODE_ISPRINTABLE` means "general category not in
`Cc`/`Cf`/`Cs`/`Co`/`Cn`/`Zl`/`Zp`/`Zs`", with ASCII space excepted. Rust's
standard library exposes only `is_control()` (that is `Cc` alone), and its
own Unicode tables are `rustc`'s, not the interpreter's — so the classification
comes from a table generated *from the oracle*:
`utils/gen_unicode_printable.py` → `ein-core/src/printable.rs`, 737 ranges,
Unicode 16.0.0. The differential test sweeps every code point where the
classification changes, so a CPython upgrade that moves a category shows up as
a named code point rather than as a mystery diff at P1a.5.

That is a deliberate 6 KB and a deliberate regeneration obligation. The
alternative — "non-ASCII is printable unless it is a control character" — is
right for every string a Zebra puzzle contains and wrong for NBSP, for the
zero-width space, for private-use and for unassigned code points, which is
exactly the class of thing that surfaces once, late, in something else's
golden.

### Two things Python does that Rust does not

- **`format!("{:.1}", f64::NAN)` is `NaN`.** Python's is `nan`, and a NaN never
  carries a sign there — `format(-float('nan'), '.1f')` is `'nan'` — while an
  infinity does. `pyfmt` handles sign, fill, alignment and zero-padding itself
  and uses Rust only for the digits, where the two agree (both round-half-even
  on the exact binary value; 4 370 differential formattings confirm it).
- **An empty format spec is not `.6f`.** `format(1.5, '')` is `'1.5'` — Python
  falls back to `str(x)`, a different algorithm — where `format(1.5, 'f')` is
  `'1.500000'`. `Spec::parse` therefore *requires* the `f`, and returns `None`
  for anything outside the supported subset rather than guessing: a site that
  meets a spec this module does not cover should say so.

### `canonical_int` moved to `ein-core`

It was in `ein-ir::ast` because the lexer needed it. It is the same question
`PyValue::Int` answers — what Python would have printed — and P1a.2's int pool
stores exactly that form ([design/03](../design/03_data_model.md) §3), so it
now lives in `ein-core::pyrepr` and `ein-ir` re-exports it. The move was
prompted by the differential test failing on `PyValue::Int("-0")`: a
non-canonical value cannot be constructed by accident if there is one
canonicaliser and it is in the crate that defines what the value means.

### The oracle became a crate

`ein-oracle` (dev-only, `publish = false`): `ein.py` and CPython kept warm
behind the JSON-Lines protocol, shared by `ein-ir` and `ein-core` and by every
phase after this one. `utils/py_oracle.py` joins `utils/ir_oracle.py` —
deliberately two scripts, because one is *ein.py's frontend* and the other is
*CPython itself*, and only the first goes stale when the engine changes.
`design/12` §1 records both, and why the fuzzer stayed next to the parser
instead of moving into `ein-conformance`.

### The fuzzer now compares trees, not verdicts

`rust_answer` returns `dump_canonical` of the parse, so an agreement that "this
is ein-lang" which disagreed on *what it means* would now fail. 1 000 000
mutations at that strength, plus the 1 200 000 accept/reject mutations from
S1a.1.1: 0 differences.
