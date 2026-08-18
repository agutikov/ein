# P1a.1 — IR frontend

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **shipped** 2026-08-18 — all three stages, acceptance below.
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

All met, 2026-08-18. Each was run, not read — 87 tests in `cargo test
--workspace`, of which 20 are differential against `ein.py`:

| item | result |
|---|---|
| accept/reject agreement with `lark.parse` on the whole corpus and on ≥ 10⁶ fuzzer mutations | 95 files and **2.2 M** mutations across three seeds — 1.2 M on accept/reject, 1 M on the **dumped AST** — 0 differences after the two finds below |
| `dump(parse(x))` byte-identical for every corpus file; `parse(dump(parse(x))) == parse(x)` | 91 resolvable files, both `dump_compact` and `dump_canonical`; both goldens reproduced; the round trip is a fixed point in *both* implementations |
| every `examples/broken/*.ein` message byte-identical | all four, EOF `-1:-1` quirk included |
| import resolution produces the same resolved form list, `resolve_and_minimize` included | 91 files under `resolve`, `minimize` and `expand`; the eleven `broken/load/import_*.expected` messages compared against the files |
| `zebra2.ein` parse + resolve under 2 ms | **824 µs** parse + resolve + expand (618.9 ms CPython, 193.7 ms PyPy — 751× / 235×). Parse alone over the whole bench set: 758 µs vs 760.6 ms / 230.9 ms |

### The two things the fuzzer found

Both are Lark artefacts that reach the output, and both are **reproduced**
rather than corrected (Q-M1a.3 recommendation (a) — a better message is a T3
failure while the harness is still finding bugs):

1. **The dynamic lexer is not maximal munch.** `(rulex (?a) :match X :assert Y)`
   parses as a *rule named `x`*, because the anonymous literal `rule` matches
   at the same position where `SYMBOL` matches `rulex`. Eighteen literals
   behave this way. The parser therefore lexes on demand and backtracks at
   form heads instead of consuming a token stream
   ([S1a.1.1](s1a.1.1_lexer_and_parser.md)).
2. **`%ignore` holds the error position back.** `xearley.py` writes a
   `delayed_matches` key for every position where whitespace or a comment
   matches — including inside a string literal, and including when nothing is
   listening — and a `defaultdict` entry holding an empty list is still
   truthy. So `(y";"{?` reports the `?`, not the `{`.

### Where the phase's scope moved

- The **AST arena** (T1a.1.2.1) landed with S1a.1.1: the parser has to build
  something.
- The **loader-side** macro checks (T1a.1.3.5's guard wiring, duplicate and
  reserved macro names) stay with the loader in
  [P1a.2](../p1a.2_kb_core/README.md); what they need from here — the macro
  names read from `stdlib/macro.ein`, the guard's message and its narrowness,
  the `MacroError` text — is in place and pinned.
- **`ein-oracle`** is a new dev-only crate: `ein.py` and CPython kept warm
  behind a JSON-Lines protocol. The conformance harness compares two CLIs, and
  the frontend has no CLI surface — re-adding one to serve a test would widen
  the T3 surface both implementations must match.

## Cross-links

- [design/04 — IR frontend](../design/04_ir_frontend.md)
- [design/02 §7 — `python_repr`](../design/02_determinism_and_order.md)
- [`ein.py/src/ein/ir/grammar.lark`](../../../ein.py/src/ein/ir/grammar.lark)
  — the spec of record
- [`docs/kernel/ir/03-ein-lang/`](../../../docs/kernel/ir/03-ein-lang/README.md)
