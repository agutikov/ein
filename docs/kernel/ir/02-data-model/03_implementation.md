# Data model — implementation map

The **module-by-module** developer reference for the KB: the layout, the
identity mechanics, and the concrete collection shapes + complexity. The
*idiomatic* level — what each entity carries and how the store behaves
abstractly — is [`01_entities.md`](01_entities.md) (entities) and
[`02_store.md`](02_store.md) (store); this page is the code-level companion
(the data-model analog of
[`../../inference/implementation.md`](../../inference/implementation.md)).

> **Audience: engine contributors.** Puzzle authors want
> [`../03-ein-lang/`](../03-ein-lang/); a reimplementer wants the abstract
> shapes in `01_entities` / `02_store` and can skip this page.
>
> **This page is a map, not a specification.** It was a map of
> `ein.py/src/ein/kb/` until M1a
> [S1a.10.6](../../../history/m1a_rust/README.md#s1a106--the-docs-after-the-oracle),
> and unlike the engine's map it is not a rename: the *abstract* store is the
> same store, but almost none of §2 and §3 survived the port, because they
> described CPython mechanics (frozen dataclasses, `object.__setattr__`, dict
> shapes) rather than the model. What replaced each is stated below.

## 1. Module map

Source root: [`ein.rs/crates/ein-core/src/`](../../../../ein.rs/crates/ein-core/src/),
with the loader in `ein-ir` and the DOT view in `ein-render`.

| module | role |
|--------|------|
| [`intern.rs`](../../../../ein.rs/crates/ein-core/src/intern.rs) | the symbol table — every name in a program as a `u32` `Symbol`. Ids are assignment-ordered and `Symbol` deliberately has no `Ord`, so id order cannot reach an output by accident ([design/02](../../../history/m1a_rust/design/02_determinism_and_order.md)) |
| [`value.rs`](../../../../ein.rs/crates/ein-core/src/value.rs) | `Value` — a fact argument in 4 bytes: `[tag:2][payload:30]`, `Sym` / `Int` / `Fact`. The int pool behind `Int` lives here too |
| [`facts.rs`](../../../../ein.rs/crates/ein-core/src/facts.rs) | the fact store — interned rows, and the `FactId` every proposition gets. Identity is the id: `probe` is O(1) where a tuple compare was O(arity) recursing into string equality |
| [`terms.rs`](../../../../ein.rs/crates/ein-core/src/terms.rs) | `Terms` — the three intern tables (symbols, integers, facts) held together, because a `Value`'s tag says which one to read. `cmp_semantic` is the total order over arguments; `STRUCTURAL` / `RESERVED` / `PREDICATES` are here because the lexer needs them as well as the engine |
| [`entities.rs`](../../../../ein.rs/crates/ein-core/src/entities.rs) | the typed views over the graph — `Relation`, `Rule`, `Macro`, `Query`, `Pattern` — and the insertion-ordered `Registry` they live in |
| [`program.rs`](../../../../ein.rs/crates/ein-core/src/program.rs) | the registries a load produces that facts do not: relations, rules, hrules, macros, the query, the config. An `Arc<Program>` every `Kb` holds, so a fork cannot write one and sharing costs a refcount. The `add_*` rules are load-time *semantics* — first-declaration-wins for rules, declared-wins-over-open-world for relations |
| [`kb.rs`](../../../../ein.rs/crates/ein-core/src/kb.rs) | `Kb` — a stack of immutable `Arc<Layer>`s plus one writable top; the indexes, `fork`, the mutation API, the justification tables, the no-good store, `EqClasses`, `FactView` |
| [`prov.rs`](../../../../ein.rs/crates/ein-core/src/prov.rs) | `Prov` (4 kinds) + its constructors, and the globally interned provenance arena — plus the **fork region** of it that is not global: the search opens one per entering and discards it when the fork dies, and `Kb::promote_provenance` copies out what a solution keeps. *Which* records a KB has is a per-KB table in `kb.rs`; the policy that decides what may be recorded is `Kb::record_justification` |
| [`ein-ir/src/from_ir.rs`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs) | the flat-form loader (route by head, per-fact provenance, open-world auto-vivify, cycle check) |
| [`ein-ir/src/imports.rs`](../../../../ein.rs/crates/ein-ir/src/imports.rs) | the module-import resolver (`std.<path>` → `stdlib/<path>.ein`; `:as` / `:symbols` + auto-closure, P1.8) |
| [`ein-render/src/kb_dot.rs`](../../../../ein.rs/crates/ein-render/src/kb_dot.rs) | the schema/fact DOT renderer (the schema nodes read the puzzle's `is-a` facts directly) |

## 2. Identity, and the back-pointer that is gone

Entity identity is unchanged from [`01_entities.md` §5](01_entities.md):
`Relation` = `(name, signature)`, `Rule` = `(name,)`, a fact = `(relation,
args)` recursively, provenance = all fields except `loc`. Metadata never
affects equality.

What changed is the **mechanism**, and it deleted a documented caveat rather
than porting it. Every entity used to carry a `_kb` back-pointer, wired after
construction through the frozen dataclass and excluded from equality — which
made `Relation.facts` answerable with no argument, and made it answer *for the
wrong KB* on a fork, since the shared entity's back-pointer still saw the
root. Here an accessor takes `&Kb` (and `&Terms`) explicitly. There is no
back-pointer to exclude from equality, no attach/detach step, and the caveat
has nothing to reproduce.

Identity itself is now an integer. A proposition is a `FactId`, so `==` is an
integer compare and the id *is* the hash; a nested fact argument is a `FactId`
like any other rather than an unregistered object.

## 3. Collections & complexity

A `Kb` is `Vec<Arc<Layer>>` plus a writable top layer. **Read** = walk the
layers oldest-first, which for an ordered list is concatenated iteration —
exactly the order "copy the list, then append" produced. **Write** = append to
the top layer only; a sealed layer is never mutated, so it is shared by `Arc`
with no lock. **`fork()`** = seal the top, clone the `Vec` of `Arc`s, start a
fresh top — allocation count independent of `|facts|`, where the shallow copy
it replaced was O(|facts|) plus six index dicts.

That is trivially correct because the KB is **append-only within a run** — the
property the fail-fast fork (S1.9.E23) and the monotone-growth argument
(S1.21.8) already lean on. A layer that only adds, over layers that never
change, cannot disagree with a copy mutated in place; `Kb::materialise` is
that copy, and the tests assert the two agree.

Per layer:

| member | shape | lookup | note |
|--------|-------|--------|------|
| `facts` | `Vec<FactId>` | concatenated iteration | append-only; dedupe by id, first-seen provenance wins |
| `present` | bitset over `FactId` | O(1) membership | |
| `by_rel` | `Symbol → Vec<FactId>` | O(1) by relation | the extent |
| `by_rel_slot_val` | `SlotKey → Vec<FactId>` | O(1) by (relation, slot, value) | the participation index (S1.8.B-idx), keyed **one level inside** a nested-fact argument since T1a.6.3.0 — a narrowing, not a filter: the matcher re-checks every slot regardless |
| `slot_bloom` | Bloom filter over those keys | O(1) skip | derived, rebuilt with the map, read only for skipping |
| `negated` | bitset of the inner id of every `(not X)` | O(1) | where the reference kept a set of `(relation, args)` tuples |
| `rule_apps_by_rule` / `rule_apps_on_rel` | `Symbol → Vec<FactId>` | O(1) | plus a per-layer `rule_apps` count, so "did any rule gain an activator?" is O(layers) rather than O(rules × activators) |
| `names` | insertion-ordered registry | O(1) by name | |
| `primary` | `FactId → ProvId` | O(1) | the first-recorded justification |
| `alts` | `FactId → Box<[ProvId]>` | topmost layer with the key | whole-list copy-on-write, because the list is *not* append-only: an arrival can land in the middle and `MAX_ALT_JUSTIFICATIONS` (32) evicts from the end, keeping the shortest |
| `EqClasses` | union-find | ~O(α) | M1 placeholder; not wired to firings |

**Reverse-index removal (S1.7.23):** `_types_by_parent` / `_instances_by_type`
/ `_facts_by_instance` / `_rules_by_type` are gone with the `Type` / `Instance`
entity-view; named-type projection is a user-space rule.

## See also

- [`01_entities.md`](01_entities.md) / [`02_store.md`](02_store.md) — the
  idiomatic entity + store reference this maps to code.
- [`../../inference/implementation.md`](../../inference/implementation.md) — the
  engine's module map (same treatment for the inference layer).
- [`../../architecture.md`](../../architecture.md) — where the data model sits
  in the crate dependency map.
- [design/03](../../../history/m1a_rust/design/03_data_model.md) — why each
  representation is the one it is, with the measurements that chose it.
