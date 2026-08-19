# 04 — IR frontend: lexer, parser, AST, dumper, macros, imports

**Settles:** how ein.rs reads `.ein` source and writes it back.
**Phase:** [P1a.1](../p1a.1_ir_frontend/README.md).
**Replaces:** `ein/ir/{parser,ast,types,dump,strings,macros}.py`,
`ein/kb/imports.py`.
**Does not replace:** `ein/ir/grammar.lark` — that file stays the spec of
record (M2's GBNF lift reads it) and becomes a *test input*, not a build
input.

---

## 1. Decision: hand-written, not generated

ein.py parses with **Lark's Earley** parser and a dynamic lexer. ein.rs
uses a hand-written maximal-munch lexer plus recursive descent.

Reasons, in order of weight:

1. **Error-message parity is the whole job.** ein.py's message is
   `"{file}:{line}:{col}: unexpected input\n{context}"` where `context`
   is Lark's `e.get_context(text)` — the offending source line plus a
   caret. No generator reproduces another generator's diagnostics for
   free; a hand-written parser is the only way to control the byte
   output. Verified shapes today:

   ```
   examples/broken/keyword_as_value.ein:3:14: unexpected input
   (query :goal :solve)
                ^
   examples/broken/unclosed_paren.ein:-1:-1: unexpected input
     :assert b
              ^
   ```
   Note the `-1:-1` for EOF and the caret column being *one past* the
   token in that case — quirks that must be reproduced (§4).

2. **The grammar is tiny and shape-pinned.** ~25 productions, all
   `"(" LITERAL … ")"`. Recursive descent is ~500 lines.

3. **Speed.** Parsing is 200–410 ms of a solve today (Earley is
   general-purpose and pays for it). A hand-written lexer/parser on a
   6 KB file should be well under 1 ms — the largest single-line win in
   the port and one the user sees on every invocation.

4. **No build-time codegen.** Keeps [12](12_toolchain_and_layout.md)'s
   dependency budget at zero for the frontend.

**Risk and mitigation.** Earley explores tokenizations; a greedy lexer
does not. The grammar was already tuned for maximal munch (the
`WILDCARD: /_(?![A-Za-z0-9_])/` lookahead exists precisely so `__closed__`
does not lex as `_`,`_`,`closed__`, and `SYMBOL`'s reserved-word
lookahead is start-anchored so `std.rule` lexes as one atom while
`rule.x` is rejected). The mitigation is a **grammar conformance
fixture**: for every corpus file and every mutation the fuzzer produces,
`lark.parse` and `ein.rs`'s parser must agree on accept/reject and, when
accepting, on the dumped AST. That is a stage in
[P1a.1](../p1a.1_ir_frontend/README.md), not an afterthought.

---

## 2. Lexer

One token type per grammar terminal, longest-match, with the reserved-word
lookahead implemented as a post-match check rather than a regex:

| token | rule | notes |
|---|---|---|
| `LParen` / `RParen` | `(` `)` | |
| `Symbol` | `(?:__)?[A-Za-z][A-Za-z0-9_*.-]*` | **rejected** if it equals or starts with `<word>-` for `word ∈ {not, and, or, neq, rule, hrule, query, config, trace, macro, import}`; the check is start-anchored, so `std.rule` is fine and `rule-x` is not |
| `Var` | `\?[A-Za-z][A-Za-z0-9_*-]*` | no `.` — deliberate asymmetry with `Symbol` |
| `Keyword` | `:[a-z][A-Za-z0-9_-]*` | lower-case first char |
| `Wildcard` | `_` not followed by `[A-Za-z0-9_]` | |
| `Eq` | `=` | a named terminal, reaches the AST as `Atom("=")` |
| `Range` | `[0-9]+\.\.([0-9]+\|\*)` | tried **before** `Int` (digit-anchored) |
| `Int` | `-?[0-9]+` | value is arbitrary precision — see [03](03_data_model.md) §3 |
| `String` | `"([^"\\]\|\\.)*"` | |
| reserved words | the 11 literals above, when they appear as a bare token | emitted as distinct kinds so the parser can dispatch |
| comments | `;…\n`, `#\|…\|#` | skipped |
| whitespace | `\s+` | skipped |

### String unescaping — exactly ein.py's minimal set

`ast.py::STRING` unescapes `\n`→LF, `\t`→TAB, `\r`→CR, and **any other
`\X` → `X`**. There is no `\xNN`, no `\uNNNN`, no octal. So `"\d"` is
`d` and `"\\"` is `\`. The dumper's `escape_string_literal` emits
`\\`, `\"`, `\n`, `\t`, `\r` in that replacement order. The pair
round-trips; ein.rs implements both verbatim and pins them with a
property test over arbitrary strings.

### Positions

`Loc { file, line, col }` with **1-based line and column**, taken from
the token start — that is what Lark's `propagate_positions` gives and
what `_loc(tok, filename)` records.

---

## 3. Parser and AST

Recursive descent over the token stream, with the top-level form
classified by head exactly as the grammar's `?form` alternation orders
it: `relation` → `rule` → `hrule` → `query` → `config` → `trace` →
`macro` → `import` → *anything else is a fact*.

Two ambiguities the Earley grammar resolves implicitly and the port must
resolve explicitly:

- **`relation` is a valid `SYMBOL`** (the one declarator that is not
  excluded, so rules can match `(relation ?R ?A ?B)` patterns). So
  `(relation R A B)` matches both `relation_decl` and `generic_fact`.
  Rule: **try `relation_decl`'s shape first; on shape failure, fall back
  to `generic_fact`.** That reproduces the documented behaviour that
  `(relation R (T1 T2))` parses as a *fact* and is then rejected by the
  loader.
- **`value*` then `kw_pair*`** in `generic_fact` / `relation_decl`: read
  values until a `Keyword` appears, then read `:kw value` pairs to the
  close paren. A value *after* a kw-pair is a parse error — pin it.

### AST representation

ein.py's nodes are frozen dataclasses with `loc` excluded from
`__eq__`/`__hash__`, which is what makes
`parse(dump(parse(x))) == parse(x)` hold. ein.rs mirrors that:

```rust
pub struct NodeId(u32);

pub enum Node {
    Atom(Symbol),
    Var(Symbol),
    Keyword(Symbol),
    Wildcard,
    Str(Symbol),                 // the *unescaped* value, interned
    Int(IntId),
    Range { low: i64, high: Option<i64> },
    KwPair { key: Symbol, value: NodeId },
    SForm { head: NodeId, args: Range<u32> },   // args in a flat arena
}

pub struct Ast { nodes: Vec<Node>, args: Vec<NodeId>, locs: Vec<Option<Loc>> }
```

- `locs` is a **side table**, so structural equality is `Node` equality
  and the round-trip property holds by construction — the same reason
  ein.py uses `field(compare=False)`.
- Arena + `u32` ids: no `Rc`, no recursion on drop, cheap subtree copies
  for macro expansion (§5).

### Synthetic heads and the `loc` quirk

Two AST details that leak into observable output and must be copied:

- Headless parens carry synthetic heads: `()` → `Atom("@empty")`,
  `(?p1 ?p2)` after `rule`/`macro` → `Atom("@params")`. The dumper
  detects `@`-prefixed head names and emits without a head.
- **Top-level forms have `loc = None`.** `_topform`, `relation_decl`,
  `generic_fact`, `eq_fact`, `not_form`, `rule_decl`, … all construct
  `SForm(...)` without passing `loc`; only `generic_list` sets
  `loc=head.loc`. So a loader error that interpolates `at {form.loc}`
  literally prints `at None` for a top-level form. ein.rs must render
  `None` there too. (Improving this is a *post-parity* change to both
  implementations, tracked as Q-M1a.6 — it is a genuine usability bug,
  but fixing it during the port would break T3 and hide regressions.)

---

## 4. Parse errors

The one place where "hand-written for message control" has to be cashed
in. ein.py's `parse` catches Lark's `UnexpectedInput` and formats:

```
{filename or '<string>'}:{e.line}:{e.column}: unexpected input
{e.get_context(text).rstrip("\n")}
```

`get_context` renders the source line containing the error plus a caret
line. Observed behaviours to replicate, each pinned by a fixture from
`examples/broken/`:

| case | ein.py output | port note |
|---|---|---|
| stray top-level atom | `file:4:1`, caret under the atom | first token that cannot start a form |
| keyword in value position | `file:3:14`, caret under the `:` | the `?value` alternation excludes `KEYWORD` |
| `rule` missing its params list | `file:4:3`, caret under `:match` | shape failure reported at the token that broke it |
| unclosed paren (EOF) | `file:-1:-1`, caret one past the last token | Lark reports `-1` for an EOF error and `get_context` still renders the last line |

The `-1:-1` case is the awkward one: the port has to *choose* to report
`-1` on EOF. That is not a bug being copied for its own sake — the
harness diffs stderr, and a "better" message is a T3 failure. Recorded
in [01](01_parity_contract.md)'s ledger if it ever proves impractical;
recommendation is to implement it exactly and revisit both
implementations together after parity.

`IRParseError` maps to a Rust error type whose `Display` is that string;
the CLI prints it to stderr and returns exit code 1, as ein.py does.

---

## 5. Macro expansion

`(macro NAME (?p…) BODY)` is a load-time AST rewrite
(`ir/macros.py`): the expander substitutes a rule clause's `(NAME a…)`
invocation with `BODY` under the parameter binding, and this is how
`forall` / `open` exist as *ein source* (`stdlib/macro.ein`) rather than
compiler arms.

Port notes:

- Substitution is structural, over the arena — copy the body's subtree
  with `Var` nodes replaced by the argument node ids. No `HashMap<String,
  Node>` clone per invocation.
- **Expansion is iterated to a fixpoint with a depth cap**; mirror
  ein.py's `MacroError` conditions (arity mismatch, unknown macro,
  recursion) message-for-message.
- The `_validate_unexpanded_macros` guard (S1.8a.f20) — a rule whose
  match head names a `std.macro` macro that was never imported is a load
  error, because unexpanded it would silently never fire — ports as-is,
  including reading the macro names *from* `stdlib/macro.ein` rather than
  hardcoding them.

---

## 6. Import resolution

`kb/imports.py` is flatten-then-load (A1 D8): every `(import …)` is
replaced *at the form level* by the module's fully-resolved form list,
qualified per tier, before `load()` runs.

| tier | form | effect |
|---|---|---|
| whole-module | `(import std.macro)` | every defined name prefixed `std.macro.` |
| aliased | `(import std.macro :as m)` | prefixed `m.` |
| selective | `(import std.macro :symbols (forall))` | listed declarations, flat and unrenamed |

Port obligations:

- **Same resolution order**: `std.*` → the stdlib root; anything else →
  file-relative to the importing file's directory. The stdlib root itself
  is [11](11_shared_assets.md)'s subject.
- **Recursive, bottom-up, re-qualified under the outer namespace** (D6),
  with cycle detection. A qualified diamond never collides
  (`B.D.x` ≠ `C.D.x`); a flat one collides into a duplicate-name error,
  and that strictness is intended (D3).
- **`resolve_and_minimize`** — the tree-shaking pass that drops imported
  declarations nothing references, including its activation-closure walk
  over match heads — must produce the *same surviving set*, because the
  surviving rule set is observable through `len(engine.cache)` and
  through T2 firing order.
- Errors (`module not found`, `:as` and `:symbols` together, empty
  `:symbols`, bare `std`) reproduce byte-for-byte.

Because a resolved program is a pure function of `(source text, stdlib
content, base_dir)`, it is content-addressable — which is what
[`.einb`](10_binary_format.md)'s `PROGRAM` section stores and what its
`META` digests invalidate. P1a.1 ships the frontend stateless; caching *across
runs* is [P1a.8](../p1a.8_binary_container/README.md)'s concern.

**Amended at [S1a.6.5](../p1a.6_performance/s1a.6.5_frontend.md) (T1a.6.5.3):
one resolution now parses each module once.** "Stateless" had a cost the
design did not price — resolution is a *tree* and the corpus's trees are
diamonds, so a `zebra2` load parsed **3.30× the bytes on disk**, `std.macro`
four times over. The cache is a map from resolved path to the parsed form list,
threaded through the recursion rather than held on the `Resolver`, so it holds
`NodeId`s into the arena that resolution is building and cannot outlive it. No
content digest is needed at that scope — a file cannot change during its own
load — and nothing downstream is disturbed, because qualification *builds*
(`rename_atoms`) rather than rewrites. `load/zebra2` −19.8 %.

---

## 7. Dumper

`dump_compact` / `dump_canonical` are used by the golden tests, by
`--dump-states` (`_kb_to_ein_text`), and by the trace. They must be
byte-identical.

- `_compact` — single line; `(head arg…)`, `()` / `(args…)` for
  `@`-headed forms, `:key value` for kw-pairs, `?x` for vars, `_` for
  wildcard, `low..high` / `low..*` for ranges, `escape_string_literal`
  for strings, `str(value)` for ints.
- `_pretty(node, indent, width)` — try compact; if
  `indent*len(INDENT) + len(compact) > width` and the node is an `SForm`
  with args, emit the head on its own line and each arg indented by
  `indent+1`, closing with `)` appended to the last arg's line.
  `_DEFAULT_WIDTH` and `_INDENT` must be read off ein.py and copied.
- `dump_canonical` over an iterable joins with a blank line and appends
  a trailing newline when non-empty.

The width-driven line breaking is where an off-by-one hides; the golden
corpus (`tests/golden/zebra2.golden` is 293 lines of exactly this
output) is the fixture.

---

## 8. Acceptance for this design

- Accept/reject agreement with `lark.parse` on the whole corpus **and**
  on ≥ 10⁶ fuzzer-generated mutations.
- `dump(parse(x))` byte-identical to ein.py's for every corpus file;
  `parse(dump(parse(x))) == parse(x)` as a property test on both sides.
- Every `examples/broken/*.ein` error message byte-identical.
- Every `KBLoadError` in the extracted load-negative corpus
  byte-identical (this is P1a.2's gate but the import/macro half lands
  here).
- Parse + resolve of `zebra2.ein` under 2 ms (vs 200 ms / 410 ms today).

## Cross-links

- [01 — Parity contract](01_parity_contract.md) §2 T3 — the byte gate.
- [03 — Data model](03_data_model.md) — where parsed atoms go.
- [11 — Shared assets](11_shared_assets.md) — stdlib resolution.
- [`docs/kernel/ir/03-ein-lang/`](../../../docs/kernel/ir/03-ein-lang/README.md)
  — the language reference the grammar implements.
