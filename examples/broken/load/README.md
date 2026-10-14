# Load-negative fixtures

Files that **parse** and then fail to **load**. Each `<name>.ein` sits beside a
`<name>.expected` holding the exact `KbLoadError` message it produces.

Sibling `../*.ein` are the *parse*-negative fixtures — they never reach the
loader. Everything here does.

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

They are also the first loader messages in the repo that name no exception
class, which is what [`defined_behaviour.md`
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

Consumers:
[`ein-ir/tests/load_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/load_semantics.rs)
(every fixture, byte for byte), and the corpus sweep
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
| `(macro) needs name + params + body` · `malformed (macro …)` · `(rule) needs name + params` · `malformed (rule …)` · `malformed (import …) — missing module name` · `fact with non-atom head` | **grammar-unreachable.** `(macro m)`, `(rule r)`, `(import)`, `(?R a b)` are all `ParseError` — the grammar guarantees the shape these checks re-verify. They are defence in depth for a caller that hand-builds an AST (`ein_ir::load` is public API), not a reachable surface for a `.ein` file. |
| `non-rule form where a rule was expected` · `unexpected top-level form` · `unresolved (import …) — internal error` | **unreachable by construction.** The top-level router only sends `rule`/`hrule`-headed forms to the rule ingester, `parse` only yields `SForm`s, and `resolve_imports` consumes every import before the router runs. Each is an internal-invariant assertion. |
| `(import M) — file-relative import needs a base directory` | **not expressible as a file.** It fires only when `base_dir` is `None`, i.e. `ein_ir::load(…, None)` over parsed source — and loading a *file* always supplies one. It is an inline unit test in [`ein-ir/tests/kb_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/kb_semantics.rs). |

Almost every message ends `at None`. The loader interpolates the form's
`loc`, and the AST lowerer synthesises the heads of `relation` / `rule` /
`macro` / `import` forms without one, so top-level forms carry no position.
The single exception is `macro_arity_mismatch`, raised during expansion on a
nested (located) node. It is a real usability bug, and
[`defined_behaviour.md` §1.5](../../../docs/kernel/defined_behaviour.md)
states `at None` as the behaviour until it is fixed.

Reserved-name shadowing has one fixture per declarator namespace
(`relation` / `rule` / `hrule` / `macro`) rather than one per reserved word:
the message differs by namespace, so the fixtures cover the message set, and
the sweep over `{absent, false, eq, relation}` stays a parametrized unit test
where it belongs.

## Growth rule

Any load error found outside this set becomes a fixture in the same commit that
fixes it. The set only grows.
