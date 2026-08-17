# Load-negative fixtures

Files that **parse** and then fail to **load**. Each `<name>.ein` sits beside a
`<name>.expected` holding the exact `KBLoadError` message it produces.

Sibling `../*.ein` are the *parse*-negative fixtures — they never reach the
loader. Everything here does.

Extracted from inline test strings in [S1a.0.1](../../../plans/m1a_rust/p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md)
T1a.0.1.2 so that both implementations can be held to the same text: the Rust
port's [P1a.2](../../../plans/m1a_rust/p1a.2_kb_core/README.md) gate is
byte-identical load errors, and a `pytest.raises(match=…)` fragment is not
something a second implementation can be held to.

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

Consumers: [`ein.py/tests/kb/test_load_negative.py`](../../../ein.py/tests/kb/test_load_negative.py)
(one case per fixture; refresh with `UPDATE_GOLDEN=1`) and the conformance
runner, which compares the two implementations' CLI output on each entry.

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
| `(import M) — file-relative import needs a base directory` | **not expressible as a file.** It fires only when `base_dir is None`, i.e. `KnowledgeBase.from_ir(parse(src))` with no base — and loading a *file* always supplies one. Stays an inline unit test in `ein.py/tests/kb/test_imports.py`. |

Reserved-name shadowing has one fixture per declarator namespace
(`relation` / `rule` / `hrule` / `macro`) rather than one per reserved word:
the message differs by namespace, so the fixtures cover the message set, and
the sweep over `{absent, false, eq, relation}` stays a parametrized unit test
where it belongs.

## Growth rule

Any load error found outside this set becomes a fixture in the same commit that
fixes it ([design/01](../../../plans/m1a_rust/design/01_parity_contract.md) §4).
The set only grows.
