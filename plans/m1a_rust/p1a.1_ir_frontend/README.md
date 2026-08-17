# P1a.1 — IR frontend

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 2 weeks (10 days of stages)
**Depends on:** [P1a.0](../p1a.0_conformance_harness/README.md)
**Blocks:** [P1a.2](../p1a.2_kb_core/README.md)

## Goal

Read `.ein` and write it back, byte-identically to ein.py — lexer,
parser, AST, dumper, macro expander, import resolver. This is the first
phase that produces engine code, and it is deliberately the one with the
smallest semantic surface and the strictest output gate: every artefact
it produces is text a golden file already pins.

Design: [design/04](../design/04_ir_frontend.md).

## Why this order

The frontend is a pure function (source → forms) with no engine state,
so it can be brought to T3 parity in isolation. It also front-loads the
two compatibility modules the rest of the port depends on
(`python_repr`, `pyfmt`) and settles Q-M1a.3 (parse-error text), which is
the one open question that could force a contract change.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.1.1](s1a.1.1_lexer_and_parser.md) | Lexer, recursive-descent parser, error messages | 4 d |
| [S1a.1.2](s1a.1.2_ast_and_dumper.md) | AST arena, `python_repr`/`pyfmt`, dumper | 3 d |
| [S1a.1.3](s1a.1.3_macros_and_imports.md) | Macro expansion, import resolution, minimisation | 3 d |

## Acceptance for the phase

- Accept/reject agreement with `lark.parse` on the whole corpus and on
  ≥ 10⁶ fuzzer mutations.
- `dump(parse(x))` byte-identical to ein.py's for every corpus file;
  `parse(dump(parse(x))) == parse(x)` as a property test.
- Every `examples/broken/*.ein` message byte-identical.
- Import resolution produces the same resolved form list (compared via
  `dump_canonical`) for every corpus file, including the tree-shaken
  `resolve_and_minimize` output.
- `zebra2.ein` parse + resolve under 2 ms (baseline: 200 ms CPython /
  410 ms PyPy).

## Cross-links

- [design/04 — IR frontend](../design/04_ir_frontend.md)
- [design/02 §7 — `python_repr`](../design/02_determinism_and_order.md)
- [`ein.py/src/ein/ir/grammar.lark`](../../../ein.py/src/ein/ir/grammar.lark)
  — the spec of record
- [`docs/kernel/ir/03-ein-lang/`](../../../docs/kernel/ir/03-ein-lang/README.md)
