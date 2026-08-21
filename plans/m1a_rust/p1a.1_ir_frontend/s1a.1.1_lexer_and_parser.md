# S1a.1.1 — Lexer, parser, error messages

**Phase:** P1a.1 (IR frontend)
**Estimate:** 4 days
**Depends on:** [P1a.0](../p1a.0_conformance_harness/README.md)
**Implements design:** [design/04](../design/04_ir_frontend.md) §§1–4

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `ir_oracle.py` and `ein-conformance`. They are gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

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

---

## Outcome — 2026-08-18

Landed as `ein-ir`'s `lex.rs` (327 lines), `parse.rs` (615), `ast.rs` (330),
plus `utils/ir_oracle.py` and two integration tests. Green:

| acceptance item | result |
|---|---|
| accept/reject agreement with `lark.parse` on the corpus | 95 files, 0 differences (`the_whole_corpus_parses_identically`) |
| … and on ≥ 10⁶ fuzzer mutations | 1 200 000 across two seeds, 0 differences after the two finds below |
| the four `examples/broken/*.ein` messages byte-identical | all four, EOF `-1:-1` quirk included |
| lexing + parsing `zebra2.ein` under 1 ms | **195 µs** (criterion, release). The whole `parse` bench set — zebra2 + zebra + the seven stdlib modules — is **758 µs against CPython's 760.6 ms and PyPy's 230.9 ms**: 1 003× and 305× |
| no allocation per token | tokens are `(Term, span, Cursor)` by value; the only allocations in a parse are the three arenas and the interner |

### The AST arena moved here, from S1a.1.2

T1a.1.2.1 built the arena; the parser has to build *something*, so `ast.rs`
lands with this stage instead. S1a.1.2 keeps the rest of its scope (`pyrepr`,
`pyfmt`, the dumper) and gains the round-trip property, which needs a dumper
to state.

### Earley's dynamic lexer is not maximal munch, and it shows

Design §1 flagged the risk abstractly. It is concrete and it is *load-bearing*:

```text
(rulex (?a) :match X :assert Y)   →   (rule x (?a) :match X :assert Y)
```

Lark's dynamic lexer offers **every** terminal that matches at a position, so
the anonymous literal `rule` competes with `SYMBOL`'s `rulex`, and the split
reading wins wherever it is the one that parses. Eighteen literals behave this
way — the eleven `SYMBOL`-excluded reserved words, `relation`, and the six
trace-event heads — and the effect reaches the AST: `(importx :as m)` is an
`import` of `x`, `(trace (stepx))` is a `step` named `x`.

So the parser does not lex to a stream at all. `lex.rs` exposes *positional
matchers*; `parse.rs` tries each production from a saved cursor and takes the
first that consumes the whole `( … )`. That is sound because every alternative
ends at the same closing paren, so a choice never changes what the rest of the
file sees — and it reproduces Lark's ambiguity resolution, which prefers the
earlier alternative (verified case by case in
`the_documented_ambiguities_resolve_the_way_lark_resolves_them`).

### The two divergences the fuzzer found

**1. `(y";"{?` — reported at the `?`, not at the `{`.** The `;` *inside the
string literal* is what does it. `xearley.py`'s scanner advances one character
at a time and raises `UnexpectedCharacters(i)` only when the Earley set, the
scan buffer **and** the `delayed_matches` dict are all empty — and the
`%ignore` pass writes `delayed_matches[m.end()].extend(to_scan)` at *every*
position where whitespace or a comment matches, including positions no live
item is looking at and including an empty `to_scan`, which still creates the
key in a `defaultdict`. A dict holding one empty list is truthy, so those
phantom keys hold the error back until the scanner walks past them:
`;"{?` matched `;[^\n]*` at the position inside the string, ending one
character further along than the real failure.

Reproduced rather than corrected (Q-M1a.3 recommendation (a)): the harness
diffs stderr, so a better message is a T3 failure. `parse::death_position`
simulates it in ten lines — walk the text, keep the furthest pending trivia
end, and die at the first position at or after the real failure where nothing
is pending.

**2. The `40`-character context window.** `get_context` slices `pos ± 40`
*before* trimming to the line, so an error past column 40 renders a
**truncated** source line. Ported verbatim.

Both are pinned: the first as a fuzz seed, both by the corpus test.

### The fuzzer

`tests/fuzz_parity.rs`, budget `EIN_FUZZ_ITERS` (2 000 per-commit, 10⁶
nightly across two seeds), stream `EIN_FUZZ_SEED`. Nine mutations — character
drop/replace/insert/swap, word splice, paren injection, truncation, line
drop/duplicate — chained one to three deep, over seeds that include every
`examples/broken/` fixture and every previous find. `SPLICES` carries the
reserved words deliberately, so the `rulex` class is hit on purpose rather
than by luck. A divergence is delta-debugged to a minimal input and written to
`corpus/fuzz_findings/`, which the next run replays as a seed *before*
generating anything — the growth rule in `corpus/README.md`, applied to
the fuzzer.

### Why the oracle is a script and not the harness

`ein-conformance` compares two `ein` CLIs, and the frontend has no CLI surface
of its own — the `ir` inspector subcommands were removed in P1.11 and
re-adding one would widen the T3 surface both implementations must match, to
serve a test. `utils/ir_oracle.py` is a batch JSON-Lines protocol over
stdin/stdout instead (`accept` / `parse` / `compact` / `resolve` / `minimize` /
`macro-names`), one warm process for the whole run, because building the Lark
grammar costs ~0.5 s and the fuzzer sends 10⁶ inputs. `ein.py` therefore
becomes a **test** dependency of `ein.rs` from this stage on, and the
per-commit CI job installs it; without it the parity tests skip, loudly.
