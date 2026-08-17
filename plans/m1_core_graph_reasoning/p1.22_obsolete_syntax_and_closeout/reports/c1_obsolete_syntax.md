# C1 — Obsolete-syntax census

**Stage:** [S1.22.1](../s1.22.1_obsolete_syntax.md), task T1.22.1.1
(read-only). **Date:** 2026-08-17. **Tree:** post-S1.22.0 (`c19add1`).

## Method and the classification rule

Per the user's 2026-08-17 correction 1, sites are classified by **whether the
kernel special-cases the head**, not by the head's spelling. `type` and
`instance` are not forbidden atoms — `(type Subtype Type)` is well-formed the
moment a puzzle writes `(relation type T T)`. What is obsolete is:

1. a `(type …)` / `(instance …)` form the **kernel** reads as a type
   declaration or as membership, and the registries / printers that served it;
2. the same forms appearing **undeclared** in a fixture — an error under the
   ordinary rule that an undeclared head is an undeclared relation, and only
   tolerated because `kb.from_ir` auto-vivifies undeclared fact heads as
   open-world relations;
3. the removed block heads `(rules …)` / `(ontology …)` / `(facts …)`
   (P1.7c Track A).

**Every one of the 7 `.ein` files below is case 2**, not case 1: none declares
`(relation instance T T)`. Verified per file:

```
examples/saturation/square-unique/terminus.ein        declares_instance=0 declares_type=0
examples/saturation/square-unique/corner-house.ein    declares_instance=0 declares_type=0
examples/saturation/square-unique/cul-de-sac.ein      declares_instance=0 declares_type=0
examples/saturation/type-exclusivity/nationalities.ein declares_instance=0 declares_type=0
examples/saturation/type-exclusivity/pets.ein         declares_instance=0 declares_type=0
examples/saturation/type-exclusivity/colors.ein       declares_instance=0 declares_type=0
examples/zebra.ein                                    declares_instance=1 declares_type=1
```

`examples/zebra.ein` is the exception, and it is **out of this stage's
scope** by the same ruling: it *declares* both heads, so its `(type …)` /
`(instance …)` are ordinary relations, not kernel-special-cased syntax. Its
ontology is [S1.22.1a](../s1.22.1a_zebra_ein_modernisation.md)'s. This stage
touches it only for prose that calls it a dead dialect (§d).

## (a) Real obsolete syntax in fixtures — 6 files

All six are `examples/saturation/` demos, consumed by
`tests/inference/test_demos.py`, which globs `examples/saturation/**/*.ein`
and asserts each produces ≥ 1 firing whose rule matches its directory name.
That is the invariant the rewrite must preserve.

| file | obsolete forms | rewrite |
|---|---|---|
| `type-exclusivity/colors.ein` | `(type Color)`, `(instance Red\|Blue Color)`, rule LHS `(instance ?a ?T)` ×2, 2 comment lines | ↓ |
| `type-exclusivity/nationalities.ein` | `(type Nationality)`, `(instance …)` ×2, rule LHS ×2, 2 comment lines | ↓ |
| `type-exclusivity/pets.ein` | `(type Pet)`, `(instance …)` ×3, rule LHS ×2, 1 comment line | ↓ |
| `square-unique/terminus.ein` | `(type Station\|Service)`, `(instance …)` ×6, rule LHS ×1, 1 comment | ↓ |
| `square-unique/corner-house.ein` | `(type House\|Nationality\|Color)`, `(instance …)` ×4, rule LHS ×1, 1 comment | ↓ |
| `square-unique/cul-de-sac.ein` | same shape, 9 hits | ↓ |

**Uniform rewrite**, semantics-preserving:

- `(type X)` and `(type X Parent)` → **deleted**. A type needs no
  declaration; `T` is an ordinary top atom
  ([`06_reserved_names.md:147`](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)).
  Where the file used `(type X Parent)` to state a hierarchy, that becomes
  `(is-a X Parent)`.
- `(instance A T)` → `(is-a A T)`, with `(relation is-a T T)` declared —
  the `zebra2.ein:352` form.
- rule LHS `(instance ?a ?T)` → `(is-a ?a ?T)`.
- the `;;;` header comments that narrate "Red is an instance of Color" are
  reworded to the `is-a` reading.

Same fact count, same arity, same firing count — `type-exclusivity` still
fires N(N−1) times over N same-type members, so `test_demos.py` is unaffected.

## (b) Code that emits or special-cases the obsolete forms — 5 sites

The load-bearing half of the purge. Once (a) lands, every branch here is dead
code that would silently keep working if a fixture regressed.

| site | what it does | disposition |
|---|---|---|
| `cli/saturate.py:380-395` | dumps `;; Type facts (n)` / `;; Instance facts (n)` sections, reprinting `(type …)` / `(instance …)` from `_facts_by_relation` | **delete both blocks** — the generic per-relation counts below them already cover any `type`/`instance` relation a puzzle legitimately declares |
| `kb/render.py:92-135` `_schema_nodes` | scans facts for `rn == "type"` and `rn == "instance"` to build type boxes / instance ovals | **drop the `type` and `instance` branches; keep `is-a`** |
| `kb/render.py:275-300` `_emit_schema_nodes` | its caller; docstring names the three conventions | reword to `is-a` only |
| `ir/to_dot.py:235-247` | `render_ontology`'s `head == "type"` branch draws `(type Name [Parent])` boxes | **delete the branch.** `render_ontology` itself stays — the flat path (`to_dot.py:480-482`) synthesises an `(ontology …)` `SForm` as an internal grouping container, so the function is live even though the block head is not |
| `kb/pattern.py:9-10,41-46,102` | `Pattern.type_names` — "vestigial … always empty", kept "for the `Rule.types` / `_rules_by_type` / `Type.rules` API surface" | **delete the field.** That API surface no longer exists: `Type` was deleted at `entities.py:84`, and grep finds no `Rule.types` / `_rules_by_type` anywhere in `src/` (the only `types` hits are the `ein.ir.types` **module**). Dead field kept for a dead API |

Note `kb/render.py` argues it may know the convention because "presentation
may know the inheritance convention; it is not kernel reasoning". That is
true of `is-a` and stays. It is not a reason to keep two spellings of a
convention no fixture uses.

## (c) Comments / docstrings referencing removed forms

**c1 — `(type …)` / `(instance …)` history notes (7 sites).** Each says some
variant of "S1.7.23 — these are ordinary facts now, no registries". With the
forms gone from fixtures the sentence explains an absence nobody will wonder
about.

`cli/saturate.py:72-73`, `kb/store.py:259-260`, `kb/store.py:486-488`,
`kb/entities.py:86-92`, `kb/from_ir.py:29-30`, `inference/compile.py:321`,
`inference/compile.py:439`, `kb/views.py:12`, `inference/hypgen.py:16,526`.

Two need more than deletion:

- **`kb/entities.py:84-93`** is the deletion note for the `Type` / `Instance`
  entity classes, and it **links to
  `plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_…md`**,
  which S1.22.2 deletes. It has to change regardless of this stage; trim to
  the one fact that still matters (the kernel imposes no type system; a
  puzzle's `is-a` rules are its type system) with no form names and no plan
  link.
- **`inference/compile.py:321,439`** teach `(instance Ent Type)` as the
  example of "an ordinary binary relation". Correct but obsolete-flavoured —
  reword to a neutral example.

**c2 — block-head references (10 sites).** `(rules …)` / `(ontology …)` /
`(facts …)` named as live structure rather than removed history:

| site | text |
|---|---|
| `cli/_common.py:49` | "(P1.7c — the `(rules …)` block wrapper is gone)" |
| `inference/hrule.py:4` | "in the ``(rules …)`` block" |
| `inference/hrule.py:14` | "(*not* in ``(ontology …)``…)" |
| `render/rules.py:32` | "delegates `(rule …)` / `(rules …)` rendering here" |
| `render/rules.py:377` | "Render a `(rules …)` library" |
| `render/constraints.py:6` | "the explicit puzzle conditions in `(facts …)`" |
| `render/constraints.py:102` | "rule-application facts in the `(ontology …)`" |
| `kb/from_ir.py:7-8` | "The four deprecated block wrappers … **are still accepted behind a back-compat shim until S1.7c.4**" — **stale**: S1.7c.4 landed, and `test_ir_ast.py:192-199` pins that `(facts (foo a))` now parses as a fact with relation `facts` |
| `kb/from_ir.py:33` | "Duplicate top-level block of the same kind (two `(ontology …)`) is fine; they merge" — describes a code path that no longer exists |
| `kb/from_ir.py:176`, `:276`, `:295` | "the wrapped `(ontology …)` pass", "Fed either a deprecated `(rules …)` block's ``form.args`` or the flat…", error string `f"non-rule form in (rules …): {child}"` |

`from_ir.py` is the worst of these: its module docstring actively misinforms
about current behaviour. `from_ir.py:295`'s error **string** is user-facing —
reword to name the flat top level.

**c3 — `grammar.lark:41,69-72,98-99`.** These are exactly the sanctioned
history note the phase README allows ("the only sanctioned mention is a single
line in the grammar doc's history note, if the census finds one is already
there"). **Keep**, but they cite `plans/m1_core_graph_reasoning/…` at line 3,
which S1.22.2 must rewire.

## (d) Docs prose — 14 files

| file | hits | disposition |
|---|---|---|
| `docs/kernel/ir/03-ein-lang/03_examples.md` | 14 | **heaviest.** Lines 32-40 and 66-74 *teach* the classic encoding ("Uses the kernel `(type …)` and `(instance …)` declarations"); 129-130 and 159-160 use it in a rule and a `:using` chain. Rewrite the whole example to `is-a` |
| `docs/kernel/ir/03-ein-lang/01_grammar.md` | 5 | lines 124-128 teach `(type …)`/`(instance …)`; 171 derives a category "from the `(instance X T)` facts" |
| `docs/kernel/ir/02-data-model/01_entities.md` | 4 | line 204 `has_instance_pattern: bool, # any (instance ?_ T) premise` — documents a field that does not exist; 226 uses `type-exclusivity`'s old LHS |
| `docs/kernel/ir/02-data-model/02_store.md` | 5 | |
| `docs/kernel/ir/01-ein-graph/04_jack_drinks_coffee.md` | 5 | |
| `docs/kernel/ir/03-ein-lang/04_dot_rendering.md` | 4 | pairs with the `kb/render.py` change in (b) |
| `docs/kernel/inference/README.md` | 2 | 118-119 "the engine treats `(type …)` / `(instance …)` / `is-a` uniformly" — reword to `is-a` |
| `docs/kernel/README.md` | 1 | line 58 frames the split as "`(type …)`/`(instance …)` vs unified `is-a`" — **§d narrative site**, see below |
| `docs/kernel/glossary.md` | 1 | line 346 calls `zebra.ein` "`(type …)` / `(instance …)` declarations" vs `zebra2.ein` "unified" — **§d narrative site** |
| `docs/kernel/ir/02-data-model/03_python_impl.md` | 1 | |
| `docs/kernel/ir/02-data-model/README.md` | 1 | |
| `docs/kernel/ir/03-ein-lang/02_patterns.md` | 1 | likely the `type_names` field from (b) |
| `docs/kernel/ir/03-ein-lang/05_inspirations.md` | 1 | the AtomSpace comparison row — **keep**: it describes *AtomSpace's* typing, not Ein's |
| `docs/kernel/inference/architecture_and_algorithms.md` | 1 | "`signature` (type atoms)" — **false positive**, see (e) |

**The narrative sites** (`docs/kernel/README.md:58`, `glossary.md:346`,
`examples/README.md`): these call `zebra.ein` "the classic encoding" as
though it were a dead dialect. Per the 2026-08-17 ruling that framing is
withdrawn — `zebra.ein` is valid Ein whose *ontology* is insufficient.
Reword to the two-ontologies-for-one-puzzle framing S1.22.1a
§Acceptance-3 requires. This stage does the *wording*; S1.22.1a does the file.

## (e) False positives — keep

- **`Layer.ONTOLOGY` / `Layer.FACT`** and every `layer` mention — the data
  model, not syntax (phase README §Acceptance-1 says so explicitly).
- **Python `isinstance(...)`** — 100+ hits, excluded from every grep here.
- **`kb.rules` / `kb.facts` / `kb.relations` registries**, `Rule.applications`,
  `render/__init__.py:10`'s `rules`/`constraints` module names,
  `dot_util.py:80` — Python identifiers that happen to spell a former block
  head.
- **`architecture_and_algorithms.md:168`** — "`signature` (type atoms)" is
  the word "type" in its ordinary sense.
- **`05_inspirations.md:15`** — describes AtomSpace, a comparison target.
- **`grammar.lark`'s history note** — sanctioned (c3).
- **`test_vscode_grammar.py:16`** and `utils/vscode-ein/ein.tmLanguage.json`
  — the test asserts the former block heads must **NOT** be highlighted as
  keywords. That is a *negative* pin of the removal; keeping it is the point.
- **All of `plans/m1_*`** — dies in S1.22.2; skipped per the stage brief.
- **`REVIEW_M1-01.md`** quotes.

## Kwargs ruling (stage item 3)

**`:layer` — parser-live, load-bearing, KEEP.** Not classic-only. Used in
`examples/features/01_not_and_absent.ein:51,54`,
`features/03_forall.ein:40`, `features/04_open.ein:53,57`,
`features/05_stdlib_domain_elim.ein:58,61,62`, and all three
`saturation/hypothesis-contradiction/*.ein`; specified in
`docs/kernel/ir/02-data-model/02_store.md:75` and `glossary.md:74` as the
per-fact layer override that *replaced* the block wrappers (P1.7c). It is the
modern mechanism, not a leftover of the old one. `zebra.ein:267-270`'s use of
it is ordinary.

**`:source` — KEEP**, emphatically: S1.22.1a §Scope-2 requires `:source` on
every given clue so the contradiction frontier and trace can name the clue a
conclusion rests on.

## Rewrite plan per test file — 20 files

Split by whether the obsolete syntax is *incidental* (a fixture that could be
written any way) or *the subject* (a test asserting the form parses).

**Incidental — mechanical `(instance a T)` → `(is-a a T)`, `(type T)` deleted
(13 files, 55 hits).** These use `(type T)` / `(instance a T)` purely as
throwaway fact shapes:

`inference/lattice/test_contradictions_backbone.py` (2),
`inference/lattice/test_gaps_backbone.py` (2),
`inference/lattice/test_p16_contract.py` (2),
`inference/monotonic/test_monotonic_dumper.py` (4),
`inference/test_commitment.py` (10), `inference/test_compile.py` (2),
`inference/test_engine.py` (5), `inference/test_rules.py` (17),
`inference/test_saturator.py` (2), `kb/test_entities.py` (3),
`kb/test_layers.py` (2), `kb/test_provenance.py` (2), `kb/test_render.py` (4).

Each needs `(relation is-a T T)` added to its inline source where the fixture
declares relations at all; where it relies on auto-vivification, `is-a`
auto-vivifies identically, so no declaration is needed and the change is a
pure string substitution.

`kb/test_render.py` additionally asserts on the type-box / instance-oval DOT
output that (b) removes — **its assertions change**, see the golden list.

**Subject-of-the-test — convert or delete (4 files).**

- `test_ir_ast.py` (6) — includes parametrised cases `"(type Person)"` and
  `"(instance Norwegian Nationality)"` asserting they parse, plus a rule
  matching `(instance ?a ?T)`. They parse *because they are generic facts*,
  which is still true and still worth pinning — **convert**: keep the
  parametrised cases but re-point them at a neutral generic head, and add one
  case pinning that an undeclared `(type …)` is an ordinary undeclared
  relation (correction 1's rule), which is the property actually worth a test.
- `test_ir_parser.py` (17) — same shape; the `(ontology :foo bar)` case at
  457-462 is a **negative** pin of the block-head removal — **keep**.
- `test_ir_to_dot.py` (6) — exercises `render_ontology`'s `type` branch that
  (b) deletes; **rewrite** against the `relation` + fact path.
- `kb/test_store.py` (22) — the largest; mostly incidental fixtures, but a
  few assert the *absence* of `kb.types` / `kb.instances` (S1.7.23 pins).
  Those are negative pins — **keep the assertion, drop the `(type …)` /
  `(instance …)` fixture text** that motivates it, or re-point at a declared
  `type` relation to pin correction 1.

**Goldens that shift.**

- `tests/golden/zebra.golden` (39 hits) — the golden for `examples/zebra.ein`.
  **Not this stage's**: S1.22.1a rewrites that file and regenerates this
  golden. Flagged so the purge does not touch it and the two stages do not
  collide.
- `tests/render/test_golden_dot.py` (4) and `tests/render/test_slice_dot.py`
  (2) — DOT goldens containing type boxes / instance ovals emitted by the
  `kb/render.py` branches (b) removes. **Regenerate.**
- `tests/kb/test_render.py` (4) — see above.

## Purge inventory for T1.22.1.2

Ordered so the suite stays green at each step.

1. **Fixtures** — the 6 `examples/saturation/` files (a). Run
   `test_demos.py` + `test_examples_load.py`.
2. **Tests** — the 13 incidental files, then the 4 subject files. Suite green.
3. **Code** — the 5 sites in (b), in this order: `kb/pattern.py`
   (`type_names`, unused), `cli/saturate.py` (printers), `ir/to_dot.py`
   (`type` branch), `kb/render.py` (`_schema_nodes` branches).
4. **Goldens** — regenerate `test_golden_dot.py` / `test_slice_dot.py` /
   `kb/test_render.py` after step 3, and diff them by eye: the only expected
   delta is the disappearance of type boxes and instance ovals that no
   fixture produces any more.
5. **Comments** — c1 (9 sites) and c2 (10 sites), incl. the stale
   `from_ir.py` module docstring and its user-facing error string.
6. **Docs** — the 12 non-false-positive files in (d), heaviest first
   (`03_examples.md`, `01_grammar.md`).
7. **Narrative** — the three "classic encoding" sites, reworded to the
   two-ontologies framing (S1.22.1a §Acceptance-3).
8. **Verify** — phase-README §Acceptance-1 grep, `./run_tests.sh`,
   `ruff check ein.py/`.

## Open items handed to other stages

- `examples/zebra.ein` + `tests/golden/zebra.golden` → **S1.22.1a**. Its
  `(type …)` / `(instance …)` are *declared* relations, so nothing here
  condemns them; the file's problem is ontological.
- Every `plans/m1_core_graph_reasoning/…` link in touched files
  (`grammar.lark:3`, `kb/entities.py:93`, and whatever the purge leaves) →
  **S1.22.2**'s inbound-link inventory, which runs on the post-purge tree.
