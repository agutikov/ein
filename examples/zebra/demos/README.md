# Per-rule demos — S1.3.2 T1.3.2.10

A demo is the smallest IR file that exercises **one** of the seven
M1-core rules. Each demo's directory name is the rule name; each
file (`a.ein`, `b.ein`, `c.ein`) is a different scenario (same
rule, different ontology / facts).

Running the engine on any demo produces ≥ 1 firing whose `:rule`
field matches the demo's directory name. The test
[`tests/inference/test_demos.py`](../../../tests/inference/test_demos.py)
enforces this for every demo, parametrised over the 21 files.

## Index

| Rule                       | Type        | Band       | Priority | Demos                                                                                                |
|----------------------------|-------------|------------|---------:|------------------------------------------------------------------------------------------------------|
| `symmetric`                | T2          | propagate  | 100      | [a](symmetric/a.ein) · [b](symmetric/b.ein) · [c](symmetric/c.ein)                                   |
| `transitive`               | T2          | derive     | 200      | [a](transitive/a.ein) · [b](transitive/b.ein) · [c](transitive/c.ein)                                |
| `implies`                  | T2          | propagate  | 100      | [a](implies/a.ein) · [b](implies/b.ein) · [c](implies/c.ein)                                         |
| `square-fwd`               | T2          | derive     | 200      | [a](square-fwd/a.ein) · [b](square-fwd/b.ein) · [c](square-fwd/c.ein)                                |
| `square-bwd`               | T2          | derive     | 200      | [a](square-bwd/a.ein) · [b](square-bwd/b.ein) · [c](square-bwd/c.ein)                                |
| `type-exclusivity`         | non-generic | eliminate  | 300      | [a](type-exclusivity/a.ein) · [b](type-exclusivity/b.ein) · [c](type-exclusivity/c.ein)              |
| `hypothesis-contradiction` | non-generic | hypothesis | 900      | [a](hypothesis-contradiction/a.ein) · [b](hypothesis-contradiction/b.ein) · [c](hypothesis-contradiction/c.ein) |

## How to read a demo

Each demo is self-contained:

- `(rules …)` — the **one** rule the demo exercises (inline per Q30
  / P1.8 deferral).
- `(ontology …)` — minimum schema + activator facts.
- `(facts …)` or `(reasoning …)` — the premises that trigger the
  rule.

The demo body comments name the **given** facts and the **derived**
fact. The engine's actual firing count is recorded in the test
output; for most demos it's exactly 1, but `symmetric` demos (which
also re-match on their own conclusion) and `type-exclusivity` demos
(which fire over ordered pairs of distinct same-type instances)
legitimately produce more than one firing.

## Demos that exercise Q40 (nested-fact patterns)

The three `hypothesis-contradiction/*.ein` demos carry **synthetic
`(hypothesis …)` + `(contradiction-under …)` facts whose first
argument is itself a fact** — a relational node, per the kernel ein
model's "named vs relational" duality
([`docs/kernel/ir/01-ein-graph/03_ein_model.md` §3](../../../docs/kernel/ir/01-ein-graph/03_ein_model.md)).

R9's `Fact.args` widening (committed `0a783bc`) makes this load;
S1.3.1's matcher (committed `d9778b0`) makes the rule unify against
the nested fact at runtime.

In production, P1.5's fork / contradict machinery emits these
synthetic facts when a branch hits a contradiction. The demos load
them directly to exercise the rule independently.

## Running a single demo

```sh
python -c "
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase
from ein_bot.inference.engine import Engine

kb = KnowledgeBase.from_ir(parse(open('examples/zebra/demos/symmetric/a.ein').read()))
eng = Engine(kb); eng.compile_all()
for f in eng.saturate():
    print(f.rule, '→', f.derived.relation_name, f.derived.args)
"
```

## Rendering to DOT/SVG (deferred)

`utils/render_examples.sh` currently processes only top-level
`examples/*.ein`. Extending it to recurse into `examples/zebra/demos/`
is a small follow-up — when it lands, this README's table will gain
SVG links alongside the `.ein` links.

## See also

- [S1.3.2 plan](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/s1.3.2_ten_core_rules.md) — the rule catalogue.
- [zebra.ein](../zebra.ein) — wait, it doesn't actually live here.
  The shipping puzzle is at [`examples/zebra.ein`](../../zebra.ein)
  (kept at top level to minimise reference churn; cf. the spec's
  envisioned `examples/zebra/zebra.ein` location).
- [`tests/inference/test_demos.py`](../../../tests/inference/test_demos.py)
  — the test that exercises every demo here.
