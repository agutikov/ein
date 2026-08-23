# Load-negative fixtures

Files that **parse** and then fail to **load**. Each `<name>.ein` sits beside a
`<name>.expected` holding the exact `KBLoadError` message it produces.

Sibling `../*.ein` are the *parse*-negative fixtures — they never reach the
loader. Everything here does.

Extracted from inline test strings in [S1a.0.1](../../../docs/history/m1a_rust/README.md#s1a01--parity-contract-corpus-manifest-divergence-ledger)
T1a.0.1.2 so that both implementations can be held to the same text: the Rust
port's [P1a.2](../../../docs/history/m1a_rust/README.md#p1a2--kb-core) gate is
byte-identical load errors, and a `pytest.raises(match=…)` fragment is not
something a second implementation can be held to.

## The `:expect` refusals — M1c S1c.1.2

Four of the fixtures here are newer than the rest and share a reason. `:expect`
lets a query state its own answer, so a `(query …)` can now carry a *test* —
and a test that silently checks nothing is worse than no test at all. Each of
these four is a way to write one:

| fixture | what it would have checked |
|---|---|
| `expect_unknown_keyword.ein` | nothing — `:expct` parsed, loaded and was ignored |
| `expect_unknown_relation.ein` | the extent of a relation that does not exist, which is empty for ever |
| `expect_omits_the_goal.ein` | something other than what the query asked |
| `expect_is_a_pattern.ein` | whatever the engine derived — a `?var` matches it |

They are also the first loader messages in the repo with no Python counterpart,
which is what [`defined_behaviour.md`
§4.1](../../../docs/kernel/defined_behaviour.md) is about.

## Format

```
<name>.ein          the fixture — a parseable file the loader rejects
<name>.expected     the exact message, one line, with placeholders
```

Placeholders — the only machine-specific text any message contains:

| placeholder | expands to |
|---|---|
| `{FILE}` | the fixture's own path, as handed to the loader |
| `{DIR}` | its directory — the base for file-relative imports |
| `{STDLIB}` | the resolved stdlib root |

Consumers: `ein.py/tests/kb/test_load_negative.py` (gone since S1a.10.5)
(one case per fixture; refresh with `UPDATE_GOLDEN=1`),
[`ein-ir/tests/load_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/load_semantics.rs)
(the same bytes, in Rust), and the corpus sweep
([`ein-cli/tests/corpus_cli.rs`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)),
which checks that each fixture is refused through the CLI as well as through
the loader.

`import_cycle.ein` and `import_cycle_b.ein` import each other. Both are
fixtures: the reported chain is the resolution stack, so entering the cycle
from either side is a different message.

## What is *not* here

Three loader messages have no fixture, all for structural reasons rather than
oversight. Recorded so the gap is a decision and not a hole:

| message | why no fixture |
|---|---|
| `(macro) needs name + params + body` · `malformed (macro …)` · `(rule) needs name + params` · `malformed (rule …)` · `malformed (import …) — missing module name` · `fact with non-atom head` | **grammar-unreachable.** `(macro m)`, `(rule r)`, `(import)`, `(?R a b)` are all `IRParseError` — the grammar guarantees the shape these checks re-verify. They are defence in depth for a caller that hand-builds an AST (`KnowledgeBase.from_ir` is public API), not a reachable surface for a `.ein` file. |
| `non-rule form where a rule was expected` · `unexpected top-level form` · `unresolved (import …) — internal error` | **unreachable by construction.** The top-level router only sends `rule`/`hrule`-headed forms to the rule ingester, `parse` only yields `SForm`s, and `resolve_imports` consumes every import before the router runs. Each is an internal-invariant assertion. |
| `(import M) — file-relative import needs a base directory` | **not expressible as a file.** It fires only when `base_dir is None`, i.e. `KnowledgeBase.from_ir(parse(src))` with no base — and loading a *file* always supplies one. It was an inline unit test in `ein.py/tests/kb/test_imports.py`, and is one in [`ein-ir/tests/load_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/load_semantics.rs). |

Almost every message ends `at None`. The loader interpolates `form.loc`, and
the AST lowerer synthesises the heads of `relation` / `rule` / `macro` /
`import` forms without one, so top-level forms carry no position. The single
exception is `macro_arity_mismatch`, raised during expansion on a nested
(located) node. That is **Q-M1a.6** as data — it is a real usability bug, and
the port prints `at None` until both implementations are fixed together.

Reserved-name shadowing has one fixture per declarator namespace
(`relation` / `rule` / `hrule` / `macro`) rather than one per reserved word:
the message differs by namespace, so the fixtures cover the message set, and
the sweep over `{absent, false, eq, relation}` stays a parametrized unit test
where it belongs.

## Growth rule

Any load error found outside this set becomes a fixture in the same commit that
fixes it ([design/01](../../../docs/history/m1a_rust/design/01_parity_contract.md) §4).
The set only grows.
