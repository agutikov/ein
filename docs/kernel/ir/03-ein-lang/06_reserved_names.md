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

## Top-level declarators — the closed classifier set (P1.7c)

A program is a **flat sequence of forms** (P1.7c — the `(ontology …)` /
`(facts …)` / `(reasoning …)` / `(rules …)` block wrappers were removed in
[S1.7c.4](../../../../plans/m1_core_graph_reasoning/p1.7c_block_head_removal/s1.7c.4_migrate_and_drop_shim.md)).
Each top-level form is classified by its **head**: a head in the table
below is a declarator (`trace` is the engine-emitted sibling); **any other
head is a fact** — "detect facts by *not* being reserved" (the author's
design note). This set is **closed**: the parser keys on it (`rule` / `hrule` / `query` /
`config` / `trace` / `macro` / `import` are SYMBOL-excluded, so a malformed declarator — e.g.
`(query)` with no kw-pairs — is a *parse* error; `relation` is the one
exception, kept a plain SYMBOL so rules can pattern-match
`(relation ?R ?A ?B)`, so its malformed form is rejected at *load* time),
and the loader
([`kb.from_ir`](../../../../ein.py/src/ein_bot/kb/from_ir.py)) routes by the
same set.

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `relation` | `(relation R A B …)` | declare a relation-type node + its arg-type signature | `kb.from_ir`; `entities.KERNEL_META_RELATIONS` |
| `rule` | `(rule N (?p…) :match … :assert …)` | declare a saturation rewrite rule | `kb.from_ir` |
| `hrule` | `(hrule N (?p…) :match … :assert …)` | declare a hypothesis-generation rule (drives `hypgen`, never fired by the saturator) | `kb.from_ir`; `hypgen` |
| `query` | `(query :mode … :goal … …)` | what to ask the engine | `kb.from_ir` (`store.Query`) |
| `config` | `(config [:flag v]*)` | solver-level knobs | `kb.from_ir`; `inference.config.SolverConfig` |
| `macro` | `(macro N (?p…) BODY)` | declare a load-time AST-rewrite alias; a rule clause's `(N a…)` invocation expands to BODY before compilation ([P1.8 S1.5.9](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)) | `kb.from_ir` (`_ingest_macros`); `ir.macros.expand_macros` |
| `import` | `(import M [:as A \| :symbols (S…)])` | pull in a library module `M` (a dotted logical name, e.g. `std.macro`); qualified-by-default, or aliased/flat-selective ([P1.8 S1.8.A1–A2](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.8.a1_module_system_design.md)) | `kb.from_ir` (grammar A2; resolve A3) |
| `trace` | `(trace <event>*)` | **engine-emitted** derivation log — parsed by [`trace/ast.py`](../../../../ein.py/src/ein_bot/trace/ast.py), ignored by `kb.from_ir`; a *sibling*, not part of the declarator-vs-fact dichotomy | `trace/` |

**Else → fact.** A top-level form whose head is none of the above is a
fact: `=`, `not`, or a generic `(NAME args*)`. Its knowledge **layer** is
per-fact (no longer positional): an explicit `:layer ontology|fact|reasoning`
wins, else it is derived — `:rule`/`:using` → REASONING, `:source` → FACT,
neither → ONTOLOGY ([S1.7c.1](../../../../plans/m1_core_graph_reasoning/p1.7c_block_head_removal/s1.7c.1_layer_attribution_decision.md)).
A former-wrapper head like `(facts …)` therefore now parses as a plain fact.

**Declared names are user-space**, with one guard (`_reserved_names`,
P1.8 S1.8.A1 D3): a `(rule …)` / `(hrule …)` / `(relation …)` / `(macro …)`
may not *bind* a name that shadows reserved kernel vocabulary — the
structural primitives (`absent` / `false`), the computed predicates
(`eq` / `neq`), or `relation`. The SYMBOL-excluded keywords
(`not` / `and` / `or` / `neq` / the declarators) can't be written as a declared
name at all (parse error). The guard is about *binding* a name; a **fact** may
still carry a reserved head (a stored `(not X)` octagon). `open` / `forall`
are deliberately *not* reserved — they migrated into the `std.macro` module
([`ein.py/src/ein_bot/stdlib/macro.ein`](../../../../ein.py/src/ein_bot/stdlib/macro.ein)).

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

## Pattern-macro sugar (`forall` / `open`) — NOT reserved

`forall` and `open` were compile-time desugars baked into `compile.py`.
Since [S1.5.9](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)
they are ordinary ein-lang `(macro …)` declarations (the `std.macro` module,
[`ein.py/src/ein_bot/stdlib/macro.ein`](../../../../ein.py/src/ein_bot/stdlib/macro.ein))
expanded at **load** time (`kb.from_ir` → `ir.macros.expand_macros`) — they
are **no longer kernel vocabulary**, no longer in `primitives.py`, and a
puzzle may even redefine them. A puzzle that wants them imports them
([S1.8.A1–A5](../../../../plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md)):
`(import std.macro :symbols (forall open))` (flat surface), or
`(import std.macro)` / `:as m` for qualified access.

| macro | form | expands to |
|-------|------|------------|
| `open` | `(open P)` | `(and (absent P) (absent (not P)))` — P is neither asserted nor negated |
| `forall` | `(forall ?b G B)` | `(absent (and G (absent B)))` — guarded universal ∀b. G→B |

## Hypothesis / query control

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `__closed__` | `(__closed__ R)` | suppress hypothesis generation for R (its extension is fixed). A **dunder** kernel-trigger name (the bare `closed` is now a free userspace name); author-writable, but usually **auto-inferred** by `emit_closed` for any relation no rule produces, or derived by `std.closure`. Kept kernel mechanism for M1 ([S1.7.10](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.10_closed.md)). | `inference/closed.py` (`CLOSED = "__closed__"`); `hypgen._is_closed` |
| `hypothesis-relations` | `(query … :hypothesis-relations (R₁ R₂ …))` | restrict the blind enumerator to the listed relations | `hypgen` (`HYPOTHESIS_RELATIONS`) |
| `no-hypothesis` | `(query … :no-hypothesis (R₁ R₂ …))` | the exclusion dual of `:hypothesis-relations` — never guess on the listed relations (saturation rules on them still fire) | `hypgen` (`NO_HYPOTHESIS`) |

## Not reserved (removed)

- **`closed`** (bare) — no longer a kernel trigger since the 2026-06-15 dunder
  split; the kernel keys on `__closed__` (above) and the bare `closed` is free
  for the stdlib/user to define.
- **`is-a` / `T`** — ordinary relation / atom since
  [S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md);
  a puzzle's inheritance rules ARE its type system, in user space.
- **`symmetric`** (and `transitive` / `functional` / …) — plain user
  *property tags*, no kernel search-special-casing since
  [S1.7.24](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md);
  symmetry is entirely the user's `(rule symmetric)`.

See also the graph-node subset in
[`../01-ein-graph/03_ein_model.md` §6](../01-ein-graph/03_ein_model.md).
