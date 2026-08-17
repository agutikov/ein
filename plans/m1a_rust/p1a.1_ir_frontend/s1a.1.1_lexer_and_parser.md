# S1a.1.1 — Lexer, parser, error messages

**Phase:** P1a.1 (IR frontend)
**Estimate:** 4 days
**Depends on:** [P1a.0](../p1a.0_conformance_harness/README.md)
**Implements design:** [design/04](../design/04_ir_frontend.md) §§1–4

## Context

Replace Lark/Earley with a hand-written maximal-munch lexer and a
recursive-descent parser, matching `grammar.lark` exactly — including the
two ambiguities Earley resolves implicitly (`relation` being a legal
`SYMBOL`; `value* kw_pair*` ordering) and including parse-error text.

## Acceptance

- Accept/reject agreement with `lark.parse` on the corpus and on ≥ 10⁶
  fuzzer mutations (byte flips, token drops, paren imbalance,
  reserved-word splices).
- The four `examples/broken/*.ein` messages byte-identical, EOF quirk
  included.
- Lexing + parsing `zebra2.ein` (6 KB, 94 forms) under 1 ms.
- No allocation per token (tokens are `(kind, span)`).

## Tasks

### Task T1a.1.1.1 — Tokeniser

All eleven terminals plus the reserved-word set, comments (`;…`,
`#|…|#`), whitespace. `Range` is tried before `Int` (digit-anchored);
`Wildcard` uses the negative lookahead so `__closed__` lexes as one
`Symbol`; the `Symbol` reserved-word rejection is start-anchored so
`std.rule` passes and `rule-x` fails. String bodies are unescaped with
ein.py's **minimal** set: `\n`/`\t`/`\r` mapped, every other `\X` → `X`.

### Task T1a.1.1.2 — Parser

`start: form*`, with the head-classified `?form` alternation in grammar
order. Implement `relation_decl` as try-then-fall-back to `generic_fact`.
Implement `value* kw_pair*` as "values until a `Keyword`, then pairs" and
reject a value after a pair. Cover every production including the trace
events (`step`, `branch-open`, `branch-close`, `branch-ref`,
`contradiction`, `symmetry-class`) — the loader ignores `(trace …)` but
the parser must accept it.

### Task T1a.1.1.3 — Error rendering

`{file}:{line}:{col}: unexpected input\n{source line}\n{caret}`. Match
Lark's `get_context` layout, the `-1:-1` EOF case, and the caret column.
Drive from the `examples/broken/` fixtures plus a generated set of
malformed inputs; every mismatch is either fixed or logged against
Q-M1a.3.

### Task T1a.1.1.4 — Grammar conformance fixture

A test that, for every corpus file and every fuzzer mutation, asserts
both parsers agree on accept/reject. Wire it into the nightly tier with
a persistent corpus of minimised failures.

## Notes

- Keep the lexer and parser in `ein-ir` with no dependency on
  `ein-core`'s interner *for tokenising* — interning happens when
  building the AST, so the lexer stays a pure function over `&str` and is
  trivially fuzzable.
- The `EQ` terminal is named (not anonymous) in the Lark grammar
  specifically so it survives token filtering and reaches the AST as
  `Atom("=")` in two positions. Reproduce that: `=` is an `Atom` node,
  not a punctuation token.
