# Per-rule demos — S1.3.2 T1.3.2.10

A demo is the smallest IR file that exercises **one** of the seven
M1-core rules. Each demo's directory name is the rule name; each
file is a named scenario (same rule, different ontology / facts).

Every demo file has the same shape:

1. **Header comment** — NL prose for the ontology (background
   assumptions), the facts (what the puzzle states explicitly), the
   question, and the expected one-step derivation.
2. **`(rules …)`** — the single rule the demo exercises (inline per
   Q30 / P1.8 deferral).
3. **`(ontology …)`** — minimum schema + activator facts.
4. **`(facts …)`** or **`(reasoning …)`** — the premises that
   trigger the rule.
5. **`(query :mode solve :goal …)`** — the derived fact the engine
   should produce when it fires the named rule.

Running the engine on any demo produces ≥ 1 firing whose `:rule`
field matches the demo's directory name. The test
[`tests/inference/test_demos.py`](../../../tests/inference/test_demos.py)
enforces this for every demo, parametrised over the 21 files.

## Index

Each demo carries its own NL header (ontology / facts / question /
expected derivation) and a `(query …)` block whose `:goal` is the
fact the named rule produces.

| Rule                       | Type        | Band       | Priority | Demos                                                                                                                                                          |
|----------------------------|-------------|------------|---------:|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `symmetric`                | T2          | propagate  | 100      | [couple](symmetric/couple.ein) · [neighbours](symmetric/neighbours.ein) · [friends](symmetric/friends.ein)                                                     |
| `transitive`               | T2          | derive     | 200      | [colocation-chain](transitive/colocation-chain.ein) · [taxonomy](transitive/taxonomy.ein) · [mealtimes](transitive/mealtimes.ein)                              |
| `implies`                  | T2          | propagate  | 100      | [right-then-next](implies/right-then-next.ein) · [parent-to-ancestor](implies/parent-to-ancestor.ein) · [org-chart](implies/org-chart.ein)                     |
| `square-fwd`               | T2          | derive     | 200      | [houses](square-fwd/houses.ein) · [meetings](square-fwd/meetings.ein) · [floors](square-fwd/floors.ein)                                                        |
| `square-bwd`               | T2          | derive     | 200      | [houses](square-bwd/houses.ein) · [meetings](square-bwd/meetings.ein) · [floors](square-bwd/floors.ein)                                                        |
| `square-unique`            | T2 (2-param)| derive     | 200      | [corner-house](square-unique/corner-house.ein) · [cul-de-sac](square-unique/cul-de-sac.ein) · [terminus](square-unique/terminus.ein)                            |
| `type-exclusivity`         | non-generic | eliminate  | 300      | [colors](type-exclusivity/colors.ein) · [nationalities](type-exclusivity/nationalities.ein) · [pets](type-exclusivity/pets.ein)                                |
| `hypothesis-contradiction` | non-generic | hypothesis | 900      | [coloc-disproved](hypothesis-contradiction/coloc-disproved.ein) · [right-of-disproved](hypothesis-contradiction/right-of-disproved.ein) · [next-to-disproved](hypothesis-contradiction/next-to-disproved.ein) |

## How to read a demo

Each demo is self-contained — the NL header explains what to read
in the IR body. The engine's actual firing count is recorded in the
test output; for most demos it's exactly 1, but `symmetric` demos
(which also re-match on their own conclusion) and `type-exclusivity`
demos (which fire over ordered pairs of distinct same-type
instances) legitimately produce more than one firing.

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
from ein.ir import parse
from ein.kb.store import KnowledgeBase
from ein.inference.engine import Engine

kb = KnowledgeBase.from_ir(parse(
    open('examples/zebra/demos/symmetric/couple.ein').read()))
eng = Engine(kb); eng.compile_all()
for f in eng.saturate():
    print(f.rule, '→', f.derived.relation_name, f.derived.args)
"
```

## Rendering to DOT/SVG

```sh
utils/render_examples.sh                  # → build/dot/
utils/render_examples.sh /tmp/out         # → /tmp/out/
```

The script recursively discovers every `.ein` under `examples/`
(top-level files + nested demos, minus `examples/broken/`) and
produces six per-form variants (`rule-{a,c}_trace-{a,b,c}`) plus
one unified KB view per demo. Outputs land at
`<out>/zebra/demos/<rule>/<scenario>/`.

## See also

- [S1.3.2 plan](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/s1.3.2_ten_core_rules.md) — the rule catalogue.
- [zebra.ein](../zebra.ein) — wait, it doesn't actually live here.
  The shipping puzzle is at [`examples/zebra.ein`](../../zebra.ein)
  (kept at top level to minimise reference churn; cf. the spec's
  envisioned `examples/zebra/zebra.ein` location).
- [`tests/inference/test_demos.py`](../../../tests/inference/test_demos.py)
  — the test that exercises every demo here.
