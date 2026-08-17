# F10 — M1 refactor-debt tail (ex-P1.7c Track B)

The decompositions / unifications P1.7b identified but deferred, one stage
each. **Relocated** from `p1.7c_block_head_removal/` Track B when the M1
plan folder was deleted (P1.22 S1.22.99); Track A (S1.7c.1–.5, .8 — the
block-head removal) shipped 2026-06-02 and died with the folder.

**17 of the original 23 remain open.** The six that were assessed, done,
re-scoped or dropped during Track B were deleted rather than carried as
tombstones — see [§Closed](#closed--no-stage-file) for the one-line record
of each.

[`findings.md`](findings.md) — P1.7b's 40-finding, code-cited review — moved
with them: it is the source register every stage below cites.

## Trigger

**Before the [M1a Rust port](../../m1a_rust/README.md).** That is P1.7b's own
recommendation and the strongest reason to drain this: `ein.rs` should
transcribe the clean reference implementation, not the remaining scar
tissue. Otherwise these are opportunistic — each is behaviour-preserving,
independently landable, and gated by the same invariant (`run_tests.sh` +
`bench_solve_monotonic_pypy.sh`).

## The tail

| ID | title | finding | leverage / risk |
|---|---|---|---|
| **S1.7c.10** | `FactId` neutral home | F-KER-6 | trivial |
| **S1.7c.11** | Unify the two swapped-arg `_resolve` | F-KER-7 | low |
| **S1.7c.12** | Unify the provenance-chain DFS | F-KER-10 | low–med |
| **S1.7c.13** | `_lattice_public` post-amble | F-ENG-5 (+14) | low |
| **S1.7c.14** | Collapse unsat-core synthesis | F-ENG-7 | med |
| **S1.7c.16** | Factor `_BaseStats` | F-ENG-9 | low |
| **S1.7c.17** | `_TimelineMixin` for dumpers | F-ENG-11 | low |
| **S1.7c.18** | Drop redundant `consistent()` (perf) | F-ENG-12 | perf |
| **S1.7c.19** | Remove the two `type: ignore` | F-ENG-13 | trivial |
| **S1.7c.20** | Decompose `rebuild_indexes` | F-KB-2 | med |
| **S1.7c.21** | `snapshot` shallow-copy | F-KB-6 | med |
| **S1.7c.24** | Restore `Query` annotations | F-KB-13 | low |
| **S1.7c.25** | Shared DOT emitter API | F-RTC-1 (+F-KB-8) | high (headline) |
| **S1.7c.26** | Decompose `to_dot` | F-KB-10 ≡ F-RTC-6 | med |
| **S1.7c.27** | Split `_build_parser` | F-RTC-2 | low–med |
| **S1.7c.29** | Flatten `parse_trace_steps` (depth 9) | F-RTC-4 | med |
| **S1.7c.32** | Share the S-expr escaper (fixes a bug) | F-RTC-10 | low (+regression) |

**Suggested ordering.** `.10`/`.11`/`.16`/`.17`/`.19`/`.24`/`.32` first
(trivial / low risk); then `.20`/`.21` (the typed-wrapper prerequisite was
re-scoped and closed, so these two stand alone); the RTC DOT pair
`.25 → .26`; then `.29`. Reuse the P1.7b acceptance gate
(`run_tests.sh` + `bench_solve_monotonic_pypy.sh`) as the invariant for
every stage.

## Closed — no stage file

Two groups, both verified against the code and **not** carried here.
Recorded so a reader doesn't rediscover them as gaps.

**Assessed and closed during Track B (2026-06-03):**

- **S1.7c.15** (split `_LatticeLoopState`) — split *assessed and rejected*;
  one documented class kept instead.
- **S1.7c.22** (typed index wrappers) — re-scoped to a copy-helper and done.
- **S1.7c.23** (flatten `from_ir.load`) — superseded by Track A's flat-form
  loader rewrite.
- **S1.7c.28** (unify the two trace pipelines) — re-scoped and done.
- **S1.7c.30** (`linearize` dispatch table) — **won't-do**, user decision.
- **S1.7c.31** (public KB/verdict accessors) — done, both halves.

**Never opened, verified during the 2026-06-02 breakdown** — already closed
or deliberately dropped:

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

- [M1a Rust port](../../m1a_rust/README.md) — the trigger above.
- [F9 — hypothesis-loop E-catalog](../f9_e_catalog/README.md) — the other
  body relocated out of the M1 plans; that one is *feature* backlog, this
  one is *structural* debt.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat parser dispatches on (Track A's subject).
