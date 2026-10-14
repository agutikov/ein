# 04 — IR frontend: lexer, parser, AST, dumper, macros, imports

**Settles:** how ein.rs reads `.ein` source and writes it back.
**Phase:** [P1a.1](../README.md#p1a1--ir-frontend).

---

## 1. Decision: hand-written, not generated

ein.rs uses a hand-written maximal-munch lexer plus recursive descent,
not a generated parser.

Reasons, in order of weight:

1. **Error-message control is the whole job.** The message is
   `"{file}:{line}:{col}: unexpected input\n{context}"` where `context`
   is the offending source line plus a caret
   ([defined_behaviour.md §1](../../../kernel/defined_behaviour.md#1-parse-and-load-diagnostics)). A generated parser's
   diagnostics are the generator's; a hand-written parser is the only way
   to control the byte output. The shapes:

   ```
   examples/broken/keyword_as_value.ein:3:14: unexpected input
   (query :goal :solve)
                ^
   examples/broken/unclosed_paren.ein:-1:-1: unexpected input
     :assert b
              ^
   ```
   Note the `-1:-1` for EOF and the caret column being *one past* the
   token in that case (§4).

2. **The grammar is tiny and shape-pinned.** ~25 productions, all
   `"(" LITERAL … ")"`. Recursive descent is ~500 lines.

3. **Speed.** A hand-written lexer/parser on a 6 KB file should be well
   under 1 ms — a cost the user sees on every invocation.

4. **No build-time codegen.** Keeps [12](12_toolchain_and_layout.md)'s
   dependency budget at zero for the frontend.

**Risk and mitigation.** A general parser explores tokenizations; a
greedy lexer does not. The grammar
([`00_ebnf.md`](../../../kernel/ir/03-ein-lang/00_ebnf.md)) is tuned for
maximal munch (the
`WILDCARD: /_(?![A-Za-z0-9_])/` lookahead exists precisely so `__closed__`
does not lex as `_`,`_`,`closed__`, and `SYMBOL`'s reserved-word
lookahead is start-anchored so `std.rule` lexes as one atom while
`rule.x` is rejected), so a greedy lexer reads it as written; the corpus
and the fuzzer's mutations pin accept/reject and, when accepting, the
dumped AST.

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
| whitespace | Unicode `White_Space` plus U+001C–U+001F | skipped |

### String unescaping — a minimal set

A string literal unescapes `\n`→LF, `\t`→TAB, `\r`→CR, and **any other
`\X` → `X`**. There is no `\xNN`, no `\uNNNN`, no octal. So `"\d"` is
`d` and `"\\"` is `\`. The dumper's `escape_string_literal` emits
`\\`, `\"`, `\n`, `\t`, `\r` in that replacement order. The pair
round-trips, pinned by a property test over arbitrary strings.

### Positions

`Loc { file, line, col }` with **1-based line and column**, taken from
the token start.

---

## 3. Parser and AST

Recursive descent over the token stream, with the top-level form
classified by head exactly as the grammar's `form` alternation orders
it: `relation` → `rule` → `hrule` → `query` → `config` → `trace` →
`macro` → `import` → *anything else is a fact*.

Two ambiguities the grammar leaves to its alternation order, resolved
explicitly:

- **`relation` is a valid `SYMBOL`** (the one declarator that is not
  excluded, so rules can match `(relation ?R ?A ?B)` patterns). So
  `(relation R A B)` matches both `relation_decl` and `generic_fact`.
  Rule: **try `relation_decl`'s shape first; on shape failure, fall back
  to `generic_fact`.** So `(relation R (T1 T2))` parses as a *fact* and
  is then rejected by the loader
  ([defined_behaviour.md §1](../../../kernel/defined_behaviour.md#1-parse-and-load-diagnostics)).
- **`value*` then `kw_pair*`** in `generic_fact` / `relation_decl`: read
  values until a `Keyword` appears, then read `:kw value` pairs to the
  close paren. A value *after* a kw-pair is a parse error — pin it.

### AST representation

A node's `loc` takes no part in its equality or its hash, which is what
makes `parse(dump(parse(x))) == parse(x)` hold:

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
  and the round-trip property holds by construction.
- Arena + `u32` ids: no `Rc`, no recursion on drop, cheap subtree copies
  for macro expansion (§5).

### Synthetic heads and the `loc` quirk

Two AST details that leak into observable output:

- Headless parens carry synthetic heads: `()` → `Atom("@empty")`,
  `(?p1 ?p2)` after `rule`/`macro` → `Atom("@params")`. The dumper
  detects `@`-prefixed head names and emits without a head.
- **Top-level forms have no `loc`.** Every top-level form — relation,
  fact, `=`-fact, `not`, rule, … — is built without one; only a nested
  list takes its head's. So a loader error that interpolates the form's
  location prints `at None` for a top-level form
  ([defined_behaviour.md §1](../../../kernel/defined_behaviour.md#1-parse-and-load-diagnostics)). It is a genuine usability
  bug, tracked as Q-M1a.6, and kept so that fixing it is one visible
  re-baseline.

---

## 4. Parse errors

The one place where "hand-written for message control" has to be cashed
in. A parse error formats as:

```
{filename or '<string>'}:{line}:{column}: unexpected input
{context}
```

The context is the source line containing the error plus a caret line.
Each behaviour is pinned by a fixture from `examples/broken/`:

| case | output | note |
|---|---|---|
| stray top-level atom | `file:4:1`, caret under the atom | first token that cannot start a form |
| keyword in value position | `file:3:14`, caret under the `:` | the `?value` alternation excludes `KEYWORD` |
| `rule` missing its params list | `file:4:3`, caret under `:match` | shape failure reported at the token that broke it |
| unclosed paren (EOF) | `file:-1:-1`, caret one past the last token | an EOF error has no position, and the context still renders the last line |

The `-1:-1` case is the awkward one: the parser has to *choose* to report
`-1` on EOF, and it does
([defined_behaviour.md §1](../../../kernel/defined_behaviour.md#1-parse-and-load-diagnostics)); improving it is Q-M1a.3's
option (c).

A parse error is a typed error whose `Display` is that string; the CLI
prints it to stderr and returns exit code 1.

---

## 5. Macro expansion

`(macro NAME (?p…) BODY)` is a load-time AST rewrite: the expander substitutes a rule clause's `(NAME a…)`
invocation with `BODY` under the parameter binding, and this is how
`forall` / `open` exist as *ein source* (`stdlib/macro.ein`) rather than
compiler arms.

Notes:

- Substitution is structural, over the arena — copy the body's subtree
  with `Var` nodes replaced by the argument node ids. No `HashMap<String,
  Node>` clone per invocation.
- **Expansion is iterated to a fixpoint with a depth cap**; the
  `MacroError` conditions (arity mismatch, unknown macro, recursion) have
  fixed messages.
- The unimported-macro guard (S1.8a.f20) — a rule whose match head names
  a `std.macro` macro that was never imported is a load error, because
  unexpanded it would silently never fire — reads the macro names *from*
  `stdlib/macro.ein` rather than hardcoding them.

---

## 6. Import resolution

Import resolution is flatten-then-load (A1 D8): every `(import …)` is
replaced *at the form level* by the module's fully-resolved form list,
qualified per tier, before `load()` runs.

| tier | form | effect |
|---|---|---|
| whole-module | `(import std.macro)` | every defined name prefixed `std.macro.` |
| aliased | `(import std.macro :as m)` | prefixed `m.` |
| selective | `(import std.macro :symbols (forall))` | listed declarations, flat and unrenamed |

Obligations:

- **Resolution order**: `std.*` → the stdlib root; anything else →
  file-relative to the importing file's directory. The stdlib root itself
  is [11](11_shared_assets.md)'s subject.
- **Recursive, bottom-up, re-qualified under the outer namespace** (D6),
  with cycle detection. A qualified diamond never collides
  (`B.D.x` ≠ `C.D.x`); a flat one collides into a duplicate-name error,
  and that strictness is intended (D3).
- **`resolve_and_minimize`** — the tree-shaking pass that drops imported
  declarations nothing references, including its activation-closure walk
  over match heads — fixes the surviving set, and the surviving rule set
  is observable, through the plan count and through the firing order.
- Errors (`module not found`, `:as` and `:symbols` together, empty
  `:symbols`, bare `std`) have fixed messages.

Because a resolved program is a pure function of `(source text, stdlib
content, base_dir)`, it is content-addressable — which is what
[`.einb`](10_binary_format.md)'s `PROGRAM` section stores and what its
`META` digests invalidate. P1a.1 ships the frontend stateless; caching *across
runs* is [P1a.8](../README.md#p1a8--binary-kb-container)'s concern.

**Amended at [S1a.6.5](../README.md#s1a65--frontend-and-load-path) (T1a.6.5.3):
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
`--dump-states`, and by the trace, so their bytes are pinned.

- compact — single line; `(head arg…)`, `()` / `(args…)` for
  `@`-headed forms, `:key value` for kw-pairs, `?x` for vars, `_` for
  wildcard, `low..high` / `low..*` for ranges, `escape_string_literal`
  for strings, decimal for ints.
- pretty `(node, indent, width)` — try compact; if
  `indent*len(INDENT) + len(compact) > width` and the node is an `SForm`
  with args, emit the head on its own line and each arg indented by
  `indent+1`, closing with `)` appended to the last arg's line.
  `DEFAULT_WIDTH` is 80 and `INDENT` two spaces.
- `dump_canonical` over an iterable joins with a blank line and appends
  a trailing newline when non-empty.

The width-driven line breaking is where an off-by-one hides; the golden
corpus (`tests/golden/zebra2.golden` is 293 lines of exactly this
output) is the fixture.

---

## 8. Acceptance for this design

- `parse(dump(parse(x))) == parse(x)` as a property test, and
  `dump(parse(x))` pinned by the goldens.
- Every `examples/broken/*.ein` error message matches its `.expected`.
- Every load error in `examples/broken/load/` matches its `.expected`
  (this is P1a.2's gate but the import/macro half lands here).
- Parse + resolve of `zebra2.ein` under 2 ms.

## Cross-links

- [03 — Data model](03_data_model.md) — where parsed atoms go.
- [11 — Shared assets](11_shared_assets.md) — stdlib resolution.
- [`docs/kernel/ir/03-ein-lang/`](../../../kernel/ir/03-ein-lang/README.md)
  — the language reference the grammar implements.
