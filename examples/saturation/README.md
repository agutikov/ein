# Per-rule saturation demos

A demo is the smallest program that exercises **one** saturation rule. Each demo's directory name is the rule
name; each file is a named scenario (the same rule over a different ontology and different facts).

Every demo file has the same shape:

1. **A header comment**: natural-language prose for the ontology (background assumptions), the facts (what the
   puzzle states explicitly), the question, and the expected one-step derivation.
2. **The rule**, written in the file: a problem's own rules are in its own file.
3. **The relations and the activator facts** the rule needs.
4. **The facts** that trigger the rule.
5. **`(query :goal …)`**: the derived fact the engine should produce when it fires the named rule.

Running the engine on any demo produces at least one firing whose `:rule` matches the demo's directory name.
What enforces this for every demo is
[`ein-infer/tests/search_semantics.rs`](../../ein.rs/crates/ein-infer/tests/search_semantics.rs), with the
`saturation` group of the [corpus sweep](../../ein.rs/crates/ein-cli/tests/corpus_cli.rs).

## Index

| Rule | Parameters | Demos |
|---|---|---|
| `symmetric` | `?rel` | [couple](symmetric/couple.ein) · [neighbours](symmetric/neighbours.ein) · [friends](symmetric/friends.ein) |
| `transitive` | `?rel` | [colocation-chain](transitive/colocation-chain.ein) · [taxonomy](transitive/taxonomy.ein) · [mealtimes](transitive/mealtimes.ein) |
| `implies` | `?p ?q` | [right-then-next](implies/right-then-next.ein) · [parent-to-ancestor](implies/parent-to-ancestor.ein) · [org-chart](implies/org-chart.ein) |
| `square-fwd` | `?R` | [houses](square-fwd/houses.ein) · [meetings](square-fwd/meetings.ein) · [floors](square-fwd/floors.ein) |
| `square-bwd` | `?R` | [houses](square-bwd/houses.ein) · [meetings](square-bwd/meetings.ein) · [floors](square-bwd/floors.ein) |
| `square-unique` | `?R ?T` | [corner-house](square-unique/corner-house.ein) · [cul-de-sac](square-unique/cul-de-sac.ein) · [terminus](square-unique/terminus.ein) |
| `type-exclusivity` | `?R` | [colors](type-exclusivity/colors.ein) · [nationalities](type-exclusivity/nationalities.ein) · [pets](type-exclusivity/pets.ein) |
| `hypothesis-contradiction` | none | [coloc-disproved](hypothesis-contradiction/coloc-disproved.ein) · [right-of-disproved](hypothesis-contradiction/right-of-disproved.ein) · [next-to-disproved](hypothesis-contradiction/next-to-disproved.ein) |

A rule with parameters is [relation-polymorphic](../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md#relation-polymorphic):
it is applied once for each activator, a fact such as `(symmetric co-located)` that binds them. A rule with none is
[first-order](../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md#first-order) and names its relations.

## How to read a demo

Each demo is self-contained: the header explains what to read in the program. For most demos the rule fires
exactly once, but the `symmetric` demos (which also re-match on their own conclusion) and the
`type-exclusivity` demos (which fire over ordered pairs of distinct instances of one type) fire more than once.

## Nested-fact patterns

The three `hypothesis-contradiction/*.ein` demos carry **`(hypothesis …)` and `(contradiction-under …)` facts
whose argument is itself a fact**, a relational node
([`04-ontology.md` § Five words](../../docs/ein/user/03-syntax-and-semantics/04-ontology.md#five-words)), and the rule
matches the nested fact. The engine writes no such facts of its own; the demos state them to exercise the rule.

## Running a single demo

```sh
ein saturate examples/saturation/symmetric/couple.ein --dump
```

## Drawing a demo

```sh
ein render rules examples/saturation/symmetric/couple.ein > couple.rules.dot
ein render kb    examples/saturation/symmetric/couple.ein > couple.kb.dot
```

## See also

- The shipping puzzle is [`examples/zebra.ein`](../zebra.ein), at the top level of `examples/`.
- [`ein-infer/tests/search_semantics.rs`](../../ein.rs/crates/ein-infer/tests/search_semantics.rs): what
  exercises every demo here.
