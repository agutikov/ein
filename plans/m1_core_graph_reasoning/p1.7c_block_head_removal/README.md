# P1.7c — Post-M1 cleanup: block-head removal + P1.7b debt tail

**Estimate:** TBD (grammar + loader; small but touches every `.ein`).
**Status:** **Track A (S1.7c.1–.5) DONE 2026-06-02** — block heads removed;
the surface is a flat sequence of forms classified by head, layer attributed
per fact ([S1.7c.1](s1.7c.1_layer_attribution_decision.md)). Full pytest
green + M1 acceptance gate green with verdicts intact. **Track B
(S1.7c.10–.32) still pending.**

> **Two tracks.** This phase carries two independent post-M1 cleanups:
> - **Track A (S1.7c.1–.5)** — the block-head removal described below
>   (surface-syntax purity).
> - **Track B (S1.7c.10–.32)** — the [P1.7b](../p1.7b_review_and_refactor/README.md)
>   review's **deferred refactor tail**: one stage per deferral, see
>   [Track B](#track-b--p17b-refactor-debt-tail-s17c10) at the bottom.
>
> The two share nothing but the directory; the `.10` start leaves room in
> `.1–.9` for Track A. Both inherit P1.7b's **one hard constraint — no
> behaviour change** (full pytest + PyPy 3-variant acceptance green,
> verdicts byte-identical) — except the two stages that close a latent bug
> ([S1.7c.32](s1.7c.32_share_sexpr_escaper.md), each gated by a regression
> test).
**Origin (user, 2026-06-02):**

> remove ein heads: `rules`, `ontology`, `facts` — make plain code. We
> already have special names for: `rule`, `hrule`, `relation`, `query`,
> `config`, and will have one more — `macro`; everything else is a fact.
> So the parser can easily detect facts by *not* being in
> `{rule, hrule, relation, query, config, macro}`.

**Depends on:** the P1.7 kernel-purity arc (S1.7.23/.24/.25) — this is the
*surface-syntax* continuation of the same "fewer special forms" thrust.
The reserved-name set it keys on is exactly
[`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
(declarators) — once `macro` lands in [P1.8 S1.5.9](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md).
**Blocks:** nothing — M1 is done; this is a post-M1 ergonomics/purity
cleanup.

## The idea

Today a `.ein` file groups forms under wrapper block heads — `(ontology
…)`, `(facts …)`, `(reasoning …)`, `(rules …)`. The proposal: drop the
wrappers and write a **flat list of forms**; the parser classifies each
top-level form by its head:

- head ∈ `{rule, hrule}` → a rule / hypothesis-rule declaration;
- head = `relation` → a relation signature declaration;
- head = `query` → the query block;
- head = `config` → solver config;
- head = `macro` → a macro definition (once P1.8 S1.5.9 lands);
- **anything else → a fact.**

This removes three reserved *block* heads (`rules` / `ontology` / `facts`,
+ `reasoning`) and makes the grammar uniform: a program is just forms.

## Open design questions (the reason it's not trivial)

1. **Layer attribution.** The block heads currently carry the
   :class:`Layer` of their children — `(ontology …)` → ONTOLOGY,
   `(facts …)` → FACT, `(reasoning …)` → REASONING. Flat facts need
   another layer signal. Candidates: derive it (schema-shaped facts —
   `relation`/`type`/`instance`/property tags → ONTOLOGY; a `:source
   "(N)"` annotation → FACT; everything user-asserted → FACT); or a
   per-form `:layer` keyword; or drop the distinction for authored input
   (REASONING is engine-only anyway, so authored input is just
   ONTOLOGY-vs-FACT). **This is the crux** — the layer split is
   load-bearing for the contradiction detector's cross-layer rule and
   for the renderer's styling.
2. **Migration.** Every `examples/*.ein` + the test inline fixtures use
   the block heads; a flat-form rewrite is a wide (mechanical) churn, or
   the loader stays **back-compatible** (accept both the wrapped and flat
   forms) for a deprecation window.
3. **Reserved-head collision.** A fact whose head happens to be a future
   declarator name would be misclassified — pins the declarator set as
   *closed* (the [reserved-names](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
   doc becomes the parser's source of truth).
4. **Grammar shape.** Whether the top level is a bare sequence of forms
   or still a single `(program …)`-style root.

## Stages (Track A)

Sequential — S1.7c.1 is the gating design call; .2/.3 are the grammar+loader
core; .4 is the migration; .5 is doc-sync. The hard constraint is
**KB-preserving**: a flat rewrite of any `.ein` must load to a byte-identical
KB (same facts, per-fact `layer`, provenance) — syntax changes, semantics
don't.

| ID | title | resolves | gist |
|---|---|---|---|
| **S1.7c.1** | [Layer-attribution decision](s1.7c.1_layer_attribution_decision.md) | Q1 (the crux) | flat facts lose the block's `Layer`; recover it. Only `REASONING` is inference-load-bearing; ONTOLOGY-vs-FACT is render/provenance only. **Decision: derive from provenance (`:rule`/`:using`→REASONING, `:source`→FACT, else ONTOLOGY), with an explicit `:layer` override** for the 23 corpus facts where pure derivation disagrees (sourced ONTOLOGY, unsourced FACT, authored REASONING). Design-only. |
| **S1.7c.2** | [Flat-form grammar](s1.7c.2_flat_form_grammar.md) | Q3, Q4 | `start: form*` over a declarator/fact alternation; classify by head against the closed reserved set; **else → fact**. `trace` stays an engine-emitted sibling. |
| **S1.7c.3** | [Loader flat routing + shim](s1.7c.3_loader_flat_routing.md) | Q2 | `from_ir.load` routes per-head with `_layer_of`; back-compat shim accepts the wrapped form (deprecation warning) so the 4 examples + ~40 fixtures keep loading. Pairs with [S1.7c.23](s1.7c.23_flatten_from_ir_load.md). |
| **S1.7c.4** | [Migrate + drop shim](s1.7c.4_migrate_and_drop_shim.md) | Q2 | scripted KB-preserving unwrap of `examples/` + fixtures; delete the shim; flat becomes the only surface. |
| **S1.7c.5** | [Docs: grammar + reserved-names](s1.7c.5_docs_grammar_reserved_names.md) | doc-sync | rewrite `01_grammar.md`, the `grammar.lark` header, `06_reserved_names.md` (now the classifier's source of truth) + re-snippet examples. |
| **S1.7c.8** | [VSCode ein syntax highlighting](s1.7c.8_vscode_syntax_highlighting.md) | ergonomics | TextMate `.tmLanguage.json` for the flat surface — relocated from [P1.6](../p1.6_rendering_and_trace/README.md) (it highlights the surface syntax P1.7c just changed). **Planned** (stage doc only). |

## Track B — P1.7b refactor-debt tail (S1.7c.10+)

P1.7b shipped its high-leverage core (the flagship `_explore_layers`
620→103, the `Mode` retirement, the two latent-bug fixes, the hot-path perf)
but **deferred a long tail** of decompositions / unifications — see the
[P1.7b ledger](../p1.7b_review_and_refactor/README.md#what-shipped-2026-06-01--and-whats-deferred).
Each surviving deferral becomes **one stage** here, grouped by subsystem and
ordered low-risk → high-risk so confidence compounds. Every stage cites its
finding id in [`findings.md`](../p1.7b_review_and_refactor/findings.md).

| ID | title | finding | leverage / risk |
|---|---|---|---|
| **S1.7c.10** | [`FactId` neutral home](s1.7c.10_factid_neutral_home.md) | F-KER-6 | trivial |
| **S1.7c.11** | [Unify the two swapped-arg `_resolve`](s1.7c.11_unify_resolve_leaf.md) | F-KER-7 | low |
| **S1.7c.12** | [Unify the provenance-chain DFS](s1.7c.12_unify_provenance_dfs.md) | F-KER-10 | low–med |
| **S1.7c.13** | [`_lattice_public` post-amble](s1.7c.13_lattice_public_postamble.md) | F-ENG-5 (+14) | low |
| **S1.7c.14** | [Collapse unsat-core synthesis](s1.7c.14_unify_unsat_core.md) | F-ENG-7 | med |
| **S1.7c.15** | [Split `_LatticeLoopState`](s1.7c.15_split_lattice_loop_state.md) | F-ENG-8 | med–high |
| **S1.7c.16** | [Factor `_BaseStats`](s1.7c.16_factor_base_stats.md) | F-ENG-9 | low |
| **S1.7c.17** | [`_TimelineMixin` for dumpers](s1.7c.17_timeline_mixin.md) | F-ENG-11 | low |
| **S1.7c.18** | [Drop redundant `consistent()` (perf)](s1.7c.18_drop_redundant_consistent.md) | F-ENG-12 | perf |
| **S1.7c.19** | [Remove the two `type: ignore`](s1.7c.19_drop_type_ignore.md) | F-ENG-13 | trivial |
| **S1.7c.20** | [Decompose `rebuild_indexes`](s1.7c.20_decompose_rebuild_indexes.md) | F-KB-2 | med |
| **S1.7c.21** | [`snapshot` shallow-copy](s1.7c.21_snapshot_shallow_copy.md) | F-KB-6 | med |
| **S1.7c.22** | [Typed index wrappers](s1.7c.22_typed_index_wrappers.md) | F-KB-9 | high |
| **S1.7c.23** | [Flatten `from_ir.load`](s1.7c.23_flatten_from_ir_load.md) | F-KB-7 | med (coord. Track A) |
| **S1.7c.24** | [Restore `Query` annotations](s1.7c.24_restore_query_annotations.md) | F-KB-13 | low |
| **S1.7c.25** | [Shared DOT emitter API](s1.7c.25_shared_dot_emitter.md) | F-RTC-1 (+F-KB-8) | high (headline) |
| **S1.7c.26** | [Decompose `to_dot`](s1.7c.26_decompose_to_dot.md) | F-KB-10 ≡ F-RTC-6 | med |
| **S1.7c.27** | [Split `_build_parser`](s1.7c.27_split_build_parser.md) | F-RTC-2 | low–med |
| **S1.7c.28** | [Unify the two trace pipelines](s1.7c.28_unify_trace_pipelines.md) | F-RTC-3 | med |
| **S1.7c.29** | [Flatten `parse_trace_steps` (depth 9)](s1.7c.29_flatten_parse_trace_steps.md) | F-RTC-4 | med |
| **S1.7c.30** | [`linearize` dispatch table](s1.7c.30_linearize_dispatch.md) | F-RTC-5 | low–med |
| **S1.7c.31** | [Public KB/verdict accessors](s1.7c.31_public_kb_accessors.md) | F-RTC-9 | low |
| **S1.7c.32** | [Share the S-expr escaper (fixes a bug)](s1.7c.32_share_sexpr_escaper.md) | F-RTC-10 | low (+regression) |

**Suggested ordering.** `.10`/`.11`/`.16`/`.17`/`.19`/`.24`/`.31`/`.32`
first (trivial / low risk); the KB cluster `.22 → .20/.21` together (the
typed wrappers make the decomp + shallow-copy structurally safe); the RTC
DOT pair `.25 → .26`; the trace chain `.29 → .28`; `.15 → .19`; `.23`
coordinated with Track A's `load` rewrite. Reuse the P1.7b acceptance gate
(`run_tests.sh` + `bench_solve_monotonic_pypy.sh`) as the invariant for every
stage.

### Not carried over (verified during this breakdown, 2026-06-02)

Re-checked against the code — already closed or deliberately dropped, so **no
stage**:

- **F-RTC-7** (`to_dot` unreachable return) — *done*: `_atom_arg_attrs`
  (`ir/to_dot.py:122`) is now a clean `Var`/`Wildcard`/default ladder; the
  trailing return is reachable.
- **F-RTC-8** (`cli.py:105` `and` short-circuit) — *done*: now
  `",".join(sorted(_LAYER_BY_NAME))`.
- **F-KER-15** (`_instance_like_objects` hoist) — *moot*: the function no
  longer exists (subsumed by the name-free `_candidate_objects` from S1.7.23).
- **F-KB-11** (`type_names` vestige) — *parked, keep* (a deliberate S1.7.6
  seam, not a refactor deferral); folded into the `.20` early-skip note.
- **EntryPolicy** (F-ENG-1 ideal) — *won't-do*: P1.7b assessed it a poor fit
  for three fixed entries (a re-dispatch sentinel = more indirection than the
  localized ladders). Not reopened.

## Connections

- [P1.7b — review & refactor](../p1.7b_review_and_refactor/README.md) —
  Track B is its deferred tail; [`findings.md`](../p1.7b_review_and_refactor/findings.md)
  is the source register every Track-B stage cites.
- [M1a Rust port](../../m1a_rust/README.md) — the strongest reason to drain
  Track B before porting: `ein.rs` should transcribe the clean reference, not
  the remaining scar tissue (P1.7b's own "recommended before M1a" note).
- [P1.7 kernel-purity arc](../p1.7_bootstrapping_zebra/) — same "fewer
  special forms" thrust, one layer up (surface syntax vs engine).
- [P1.8 S1.5.9 macros](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)
  — introduces the `macro` declarator this keys on.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat-parser dispatches on.
