# F10 — M1 refactor-debt tail (ex-P1.7c Track B)

The 23 decompositions / unifications P1.7b identified but deferred, one
stage each. **Relocated verbatim** from `p1.7c_block_head_removal/` Track B
when `plans/m1_core_graph_reasoning/` was deleted (P1.22 S1.22.99); Track A
(S1.7c.1–.5, .8 — the block-head removal) shipped 2026-06-02 and died with
the folder.

[`findings.md`](findings.md) — P1.7b's 40-finding, code-cited review — moved
with them: it is the source register every stage below cites.

## Trigger

**Before the [M1a Rust port](../m1a_rust/README.md).** That is P1.7b's own
recommendation and the strongest reason to drain this: `ein.rs` should
transcribe the clean reference implementation, not the remaining scar
tissue. Otherwise these are opportunistic — each is behaviour-preserving,
independently landable, and gated by the same invariant (`run_tests.sh` +
`bench_solve_monotonic_pypy.sh`).

## The tail

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
| **S1.7c.23** | [Flatten `from_ir.load`](s1.7c.23_flatten_from_ir_load.md) | F-KB-7 | med |
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
typed wrappers make the decomposition + shallow-copy structurally safe);
the RTC DOT pair `.25 → .26`; the trace chain `.29 → .28`; `.15 → .19`.
`.23` was to be coordinated with Track A's `load` rewrite — Track A has
since shipped, so it is now unblocked and standalone.

## Not carried over

Verified against the code during the 2026-06-02 breakdown — already closed
or deliberately dropped, so no stage exists:

- **F-RTC-7** (`to_dot` unreachable return) — *done*: `_atom_arg_attrs` is a
  clean `Var`/`Wildcard`/default ladder; the trailing return is reachable.
- **F-RTC-8** (`cli.py` `and` short-circuit) — *done*.
- **F-KER-15** (`_instance_like_objects` hoist) — *moot*: the function no
  longer exists (subsumed by the name-free `_candidate_objects`, S1.7.23).
- **F-KB-11** (`type_names` vestige) — *parked, keep*: a deliberate S1.7.6
  seam, not a refactor deferral; folded into `.20`'s early-skip note.
- **EntryPolicy** (F-ENG-1 ideal) — *won't-do*: assessed a poor fit for
  three fixed entries (a re-dispatch sentinel = more indirection than the
  localized ladders).

## Connections

- [M1a Rust port](../m1a_rust/README.md) — the trigger above.
- [F9 — hypothesis-loop E-catalog](../f9_e_catalog/README.md) — the other
  body relocated out of the M1 plans; that one is *feature* backlog, this
  one is *structural* debt.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat parser dispatches on (Track A's subject).
