# Reserved names — the ein surface language

The **authoritative** list of names an ein puzzle author may *write* but
not *redefine*: the kernel gives them fixed meaning. This is the
surface-language view (what you type in a `.ein` file). For the
engine-internal vocabulary (carrier heads, protocol enums) see
[`../../inference/reserved_engine_strings.md`](../../inference/reserved_engine_strings.md).

After the S1.7.23/.24 kernel-purity pass, the reserved set is small: the
kernel imposes **no type system** (`is-a` / `T` are ordinary
relation/atom — [S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md))
and **no symmetric semantics** (`symmetric` is a plain user property tag —
[S1.7.24](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md)).
A name is reserved **iff** it appears in this table or the engine-strings
doc — nothing else is special.

## Declarators

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `relation` | `(relation R A B)` | declare a relation-type node + its arg-type signature | `kb.from_ir`; `entities.KERNEL_META_RELATIONS` |
| `rule` | `(rule N (?p…) :match … :assert …)` | declare a saturation rewrite rule | `kb.from_ir` |
| `hrule` | `(hrule N (?p…) :match … :assert …)` | declare a hypothesis-generation rule (drives `hypgen`, never fired by the saturator) | `kb.from_ir`; `hypgen` |

## Rule-body / ⊥ primitives (kept M1 kernel vocabulary)

Declared once in [`inference/primitives.py`](../../../../ein.py/src/ein_bot/inference/primitives.py)
(`primitives.STRUCTURAL`); the deep behaviour lives at the *engine site*.

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `not` | 1 | propositional negation; `(not X)` is a stored octagon fact whose arg is the negated proposition | matcher (`match.py`) + contradiction detector (`contradiction.py`) |
| `false` | 0+ | direct ⊥ — `(false)` asserts the firing rule reached a contradiction (args empty by convention) | contradiction detector |
| `and` | 2+ | conjunction; flattened into sibling premises of one plan | compiler (`compile.py`) |
| `or` | 2+ | disjunction; a **top-level** `(or …)` in a `:match` is lowered to one rule per disjunct at load time | loader (`kb.from_ir._match_disjuncts`) |
| `absent` | 1 | negation-as-failure on a sub-pattern (`AbsentGuard`) | compiler + matcher |

## Computed predicates

Declared in [`inference/predicates.py`](../../../../ein.py/src/ein_bot/inference/predicates.py)
(`predicates.names()`). A predicate's truth is *computed* from the current
bindings, not looked up in the KB.

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `eq` | 2 | `(eq ?a ?b)` true iff the slots resolve equal | matcher `Guard` opcode |
| `neq` | 2 | `(neq ?a ?b)` true iff the slots resolve unequal | matcher `Guard` opcode |

## Desugaring sugar (→ P1.8 macros)

These desugar at **compile time** into the primitives above; they carry no
standalone kernel commitment and are slated to become importable P1.8
macros ([S1.5.9](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)).
Listed in `primitives.SUGAR`.

| name | form | desugars to | engine site |
|------|------|-------------|-------------|
| `open` | `(open P)` | `(and (absent P) (absent (not P)))` — P is neither asserted nor negated | `compile.py:_desugar_open` |
| `forall` | `(forall ?b G B)` | `(absent (and G (absent B)))` — guarded universal ∀b. G→B | `compile.py:_desugar_forall` |

## Hypothesis / query control

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `closed` | `(closed R)` | suppress hypothesis generation for R (its extension is fixed). Author-writable, but usually **auto-inferred** by `emit_closed` for any relation no rule produces. Kept kernel mechanism for M1 ([S1.7.10](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.10_closed.md)). | `inference/closed.py` (`CLOSED`); `hypgen._is_closed` |
| `hypothesis-relations` | `(query … :hypothesis-relations (R₁ R₂ …))` | restrict the blind enumerator to the listed relations | `hypgen` (`HYPOTHESIS_RELATIONS`) |

## Not reserved (removed)

- **`is-a` / `T`** — ordinary relation / atom since
  [S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md);
  a puzzle's inheritance rules ARE its type system, in user space.
- **`symmetric`** (and `transitive` / `functional` / …) — plain user
  *property tags*, no kernel search-special-casing since
  [S1.7.24](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md);
  symmetry is entirely the user's `(rule symmetric)`.

See also the graph-node subset in
[`../01-ein-graph/03_ein_model.md` §6](../01-ein-graph/03_ein_model.md).
