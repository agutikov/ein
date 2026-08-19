# S1a.6.5 — Frontend and load path

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days, **shortened to 1** by
[baseline.md §8](baseline.md#8-what-this-chooses-for-the-rest-of-the-phase) —
the acceptance was already met by 8×, so the stage is a confirmation plus the
allocation report it asks for
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to [design/04](../design/04_ir_frontend.md)

**Status: shipped 2026-08-19.** The confirmation found something: a load
parsed **3.30× the bytes on disk**, because import resolution parses a module
once per *edge* of the tree and the corpus's trees are diamonds. `load/zebra2`
**891.4 → 664.1 µs (−25.5 %)**, `parse/zebra2_resolve` **745.0 → 509.5 µs
(−31.6 %)**, `parse/zebra2` −21.7 %, `parse/corpus` −15.0 %. On processes, that
is −0.23 ms on *every* invocation: `saturate zebra2` 3.9 → 3.6 ms and `render
rules zebra2` 1.6 → 1.5 ms, against −1.4 % on `solve zebra2 -e` and nothing
measurable on `solve zebra -e`, where a load is 0.9 % of the run.

Every acceptance clause is met with room, T3 472/473 and T2 239/240 with
[D2](../divergences.md) the only cell, 306 ein.rs tests green.

| task | outcome |
|---|---|
| [T1a.6.5.1](#task-t1a651--lexer) Lexer | **shipped in part, and two of its three questions closed at "no".** Zero allocation per token: confirmed, by counter. The eleven-word reserved walk: 1 250 runs a load, left alone. The per-character cursor: **−21.7 %** of `parse/zebra2` across three changes, one of which was **built vectorised and reverted at +14 %** |
| [T1a.6.5.2](#task-t1a652--arena-pre-sizing) Arena pre-sizing | **the task's own subject built and reverted at +1.2…+3.0 %** — the arenas are 1 111 nodes and 157 symbols, too small for a doubling to be worth pre-empting. What shipped instead is the interner's *hash*: SipHash was 7 % of a parse profile, **−10.5 %** |
| [T1a.6.5.3](#task-t1a653--import-resolution) Import resolution | **shipped, and it is the stage** — one parse per module per resolution instead of one per edge: 8 `parse()` calls → 5, 85 412 bytes → 59 757, `load/zebra2` **−19.8 %** |
| [T1a.6.5.4](#task-t1a654--macro-expansion) Macro expansion | **not built, measured: 3.8 % / 1.9 % / 2.2 % of a load.** The task said "probably already free — confirm and move on", and it is |
| [T1a.6.5.5](#task-t1a655--index-building) Index building | **not built.** The loader calls `rebuild_indexes` **once** (the note's first question), it is 4.3 % of a load, and pre-sizing its two maps is **−0.7 %** — inside the drift between two runs on this machine, and the second time this stage measured pre-sizing losing |
| [T1a.6.5.6](#task-t1a656--startup) Startup | **measured, nothing to fix.** `ein --help` is **1.02 ms** against ein.py's 97.6 ms, and 0.23 ms of it is what `/bin/true` costs. The one number worth recording: **snmalloc is 0.59 ms of every process start**, which doubles a trivial invocation and is repaid by the first 5 ms of engine work |

The numbers are
[baseline.md §16](baseline.md#16-s1a65--the-load-path-and-the-modules-it-parsed-twice).

## Context

Parse + load is 0.63 s of a 5.7 s CPython zebra2 run and 0.78 s under
PyPy — a fixed cost paid on *every* invocation, and the one a user feels
most directly because it happens before anything is printed. The
hand-written frontend should already have collapsed it by two orders of
magnitude; this stage confirms that and removes what is left.

It also matters disproportionately for the workloads around the engine:
the conformance runner makes thousands of invocations over the same
files, and `utils/feature_matrix.py` makes ~20 fresh-process runs per
matrix.

**What the confirmation actually found (2026-08-19).** The collapse had
happened — a load was 0.89 ms against a 5 ms acceptance — and the shape of
what was left was not in any of the six tasks. `examples/frontend_cost.rs`
split a load into its phases and **59 % of it was import resolution**, which
is a *parse* under another name: `zebra2` imports `std.algebra` and
`std.bijection`, `std.bijection` imports `std.algebra`, and all three import
`std.macro`, so eight `parse()` calls ran over 85 412 bytes for a 25 919-byte
puzzle. The task list had one line about that (T1a.6.5.3, written for the
*conformance runner's* repeated loads) and it turned out to be the stage.

The second finding is about method rather than the frontend, and it cost two
reverts: **on data this small, pre-sizing loses.** An AST arena of 1 111
nodes and an index map of 250 keys reach their size in eleven doublings that
cost less than the one oversized allocation and the rehash that replace them.
Both experiments were built, measured, and reverted — T1a.6.5.2 at +1.2…+3.0 %
and T1a.6.5.5 at −0.7 %.

## Acceptance

- ✅ Parse + resolve of `zebra2.ein` under 2 ms (P1a.1's gate) and under
  1 ms after this stage — **509.5 µs**, `parse/zebra2_resolve`.
- ✅ Full `load` (parse + imports + macro expansion + relation/rule/fact
  ingest + `rebuild_indexes`) under 5 ms — **664.1 µs**, 7.5× under.
- ✅ Allocation count for a load bounded and reported — **5 451**
  allocations and 474 KB of churn for `zebra2`, 4 570 for `zebra`, 1 264 for
  `features/05`; per phase, in
  [baseline.md §16](baseline.md#16-s1a65--the-load-path-and-the-modules-it-parsed-twice).
- ✅ T3 green — 472/473 with [D2](../divergences.md) the only cell, which is
  the phase's standing state since [S1a.6.10](s1a.6.10_parity_contract.md).

## Tasks

### Task T1a.6.5.1 — Lexer

Confirm zero allocation per token (tokens are `(kind, span)`), use
`memchr` for comment and string scanning, and check that the reserved-word
rejection is a perfect-hash / length-bucketed compare rather than a
linear walk of eleven strings per identifier.

**Outcome: two of the three closed at "no", the third shipped at −21.7 %.**

*Zero allocation per token* — confirmed, and now by a counter rather than by
reading the code. A whole `zebra2` load interns **1 900** times for **288**
misses, and a miss is the only interning that allocates; the matchers
themselves return `Lexeme { start, end, at, next }` and touch no heap.

*The eleven-word reserved walk* — **left alone**, because `lex_symbol` says it
runs **1 250** times in a load. Eleven `strip_prefix` calls × 1 250, against a
660 µs load: a perfect hash would replace an instruction that is not being
executed. (This is the same shape as
[T1a.6.4.4](s1a.6.4_hypgen_and_lattice.md)'s no-good bitmask, and it is why
this phase counts before it optimises.)

*The per-character cursor* — the real cost, and not the one the task names.
`skip_trivia` was **26.3 %** of a `parse/zebra2` profile and `match_term`
13.5 %, because a `Cursor` counts characters and lines and both walked the
source one `src[pos..].chars().next()` at a time. Three changes:

1. **`advance_to` walks bytes**, decoding only above 0x7f. The first version
   was the vectorised one — `is_ascii()`, then `rposition` for the last
   newline, then a `filter().count()` for the line delta — and it was **+10 to
   +14 %**: the spans are one space and a two-character indent, where three
   passes lose to one loop. The single byte loop is −3.5 %.
2. **A line comment is scanned with `str::find('\n')`** — which is `memchr`,
   so the task's ask arrives without a dependency for it — and then advanced by
   arithmetic, since it is the one long ASCII run in the file and cannot
   contain a newline: **−5.8 %**.
3. **`skip_trivia` splits** into an inlined one-byte test and an out-of-line
   remainder. Every alternative the parser tries asks for a terminal at the
   *same* position, so most calls have nothing to skip: **−3.4 %**.

**65 % of `zebra2.ein`'s bytes are comment and blank line, and parsing it
stripped of all of them is only 12 % faster** (25 919 → 9 055 bytes, 196.1 →
175.9 µs, measured before the three changes above) — so the cost was never the
comment bytes. It was the per-character walk over everything and the call
frequency of a backtracking parser, which is why the profile said 26 % and the
strip experiment said 12 % and both were needed to read either.

What is left above 10 % is the backtracking itself — `skip_trivia` 13.1 %,
`match_term` 10.9 %, `advance_to` 8.5 % of a parse+resolve. Removing it means
skipping trivia once per *position* rather than once per attempt, which is a
parser whose error positions are a byte-parity surface (`death_position`,
Q-M1a.3) redesigned for 0.2 ms on an acceptance already passed by 7×. Not
this stage's trade.

### Task T1a.6.5.2 — Arena pre-sizing

Size the AST node/args arenas from the source length (a good linear
estimate for s-expressions) so a parse does no reallocation. Same for the
interner's text arena on a cold start.

**Outcome: built and reverted at +1.2…+3.0 %; the interner's *hash* shipped
at −10.5 %.**

The estimate the task assumes is sound — `Ast::arena_sizes()` (added for this)
says `zebra2.ein` is **23.3 source bytes per node**, `zebra.ein` 40.0 and
`features/05` 26.9, so `len/24` is a safe linear bound. The premise under it is
not: the arenas it sizes are **1 111 nodes, 619 args and 157 symbols**. Eleven
doublings of a 13 KB vector cost less than one oversized allocation, and
`HashMap::reserve` on a table that never fills costs a rehash outright.
`parse/corpus` was the worst cell at **+3.0 %**, because it parses nine files
into one arena and each reserve re-grows what the last one left.

What the profile named instead was the interner's *index*: a
`std::collections::HashMap`, so every atom, variable, keyword, integer and
string in every module was SipHashed — **7 %** of a `parse/zebra2` profile for
a table of 157 entries. The keys are strings a source file chose and the map is
never iterated (its own doc says so, because
[design/02 §9](../design/02_determinism_and_order.md) requires it), so neither
of `RandomState`'s two properties buys anything here and its per-process seed
is the hazard the rest of the workspace already refuses. `FxHashMap`:
**`parse/zebra2` −10.5 %, `load/zebra2` −5.0 %**.

### Task T1a.6.5.3 — Import resolution

Resolving `zebra2.ein` pulls three stdlib modules, each parsed fresh.
Cache parsed module ASTs by resolved path + content hash **within a
process** — the same content-addressed key
[design/10](../design/10_binary_format.md) §4 uses for `.einb`
invalidation, just scoped to one run, and it is what makes the
conformance runner's repeated loads cheap.

Keep it off by default in the CLI if it perturbs anything observable; it
should not, since resolution is a pure function of
`(source, stdlib content, base_dir)`.

**Outcome: shipped, on by default, and worth more than the task expected —
because the repeats are not across loads, they are inside one.**

The task was written for the conformance runner making thousands of
invocations over the same files. Each of those is a fresh *process*, so a
within-process cache does nothing for them. What it does do is collapse the
diamond inside a single resolution:

| | before | after |
|---|---:|---:|
| `parse()` calls per `zebra2` load | 8 | **5** |
| bytes parsed | 85 412 (**3.30×** the file) | 59 757 (2.31×) |
| `load/zebra2` | 892.6 µs | **715.9 µs (−19.8 %)** |
| `parse/zebra2_resolve` | 744.8 µs | **565.3 µs (−24.1 %)** |

`std.macro` was parsed four times and `std.algebra` twice. Two repeats are
left, and neither is worth a second cache: the S1.8a.f20 macro guard builds its
own `Ast` (9.8 µs, and widening the cache's lifetime to reach it is exactly the
scope question below), and `locate` still *reads* a module whose parse is
cached, because the key the cache looks up is the resolved path `locate`
produces on its way to the text — six reads and six `canonicalize` calls per
`zebra2` load, ~7 µs of 804.

The cache holds `NodeId`s, which index the `Ast` the resolution is building, so
it is threaded through the recursion rather than kept on the `Resolver`: it
cannot outlive that arena, and the scope question the task raises ("within a
process") answers itself as "within a resolution". No content hash is needed
at that scope — a file cannot change during its own load — and `.einb`'s key
([design/10](../design/10_binary_format.md) §4) is still the right one for the
cross-process version, which is
[P1a.8](../p1a.8_binary_container/README.md)'s.

**Nothing is observable**, and the reason is structural rather than empirical:
nothing downstream mutates a node. `qualify` rewrites by *building*
(`rename_atoms`), `select` filters, `dedup_declarations` compares with
`eq_nodes` — and a diamond's second copy, now literally the same node, takes
its `a == b` fast path to the same answer it used to reach structurally.
`one_module_under_two_qualifications_does_not_leak_either_way` is the test for
the one way that could be false, and it asserts against `ein.py`, which has no
cache at all.

### Task T1a.6.5.4 — Macro expansion

Substitution copies subtrees. Measure whether copy-on-substitute or a
lazy view pays; the stdlib's macros are tiny (`macro.ein` is 21 lines),
so this is probably already free — confirm and move on rather than
optimising it speculatively.

**Outcome: confirmed free, and moved on.** `collect_macros` plus
`expand_rule_clauses` is **30.8 µs of an 804 µs load** on `zebra2` (3.8 %),
14.0 µs on `zebra` (1.9 %) and 2.8 µs on `features/05` (2.2 %). It is 790
allocations, which is 14 % of a load's — the copy the task asks about — and a
lazy view would be trading a measurable amount of aliasing risk for 30 µs.

### Task T1a.6.5.5 — Index building

`rebuild_indexes` is one pass over facts feeding six groupings. Size the
maps from the fact count up front, and check whether the incremental
`index_fact` path (used during saturation) and the batch path can share a
single implementation without either paying for the other's shape.

**Outcome: not built, and the note's first question answers most of it.**

*Is the loader calling it more than once?* No — `from_ir::load` calls it once,
and it is the only non-test caller in the workspace. It costs **34.5 µs, 4.3 %
of a load** (3.7 % on `zebra`, 7.8 % on `features/05`, where there is less
else to do).

*Pre-sizing the maps* was built: `by_rel.reserve(n)` and
`by_rel_slot_val.reserve(n * 3)` from the fact count. **−0.7 %** on
`load/zebra2`, which is inside the drift between two runs on this machine and
the second time this stage watched pre-sizing fail to pay. Reverted. The 586
allocations that pass makes are mostly the per-key `Vec`, which a reserve on
the map does not touch.

*Sharing one implementation* — the two paths already agree by construction and
`Kb::check_layering` is the executable proof, so what a merge would buy is
maintenance rather than speed. What it would cost is the shape of the hot path:
`index_fact` runs per fact insert during saturation (90 k+ on `zebra -e`) and
maintains `n_by_rel` incrementally, where the batch path derives it in one pass
at the end. Merging means one of them pays for the other's shape, against a
34.5 µs prize. No.

### Task T1a.6.5.6 — Startup

Measure process start to first output for `ein --help` and a trivial
solve. Binary size, dynamic-linking and the embedded stdlib all show up
here; a 5 ms engine behind a 40 ms startup is not a fast tool.

**Outcome: measured; the 40 ms start-up is ein.py's, not ein.rs's.**
`utils/e2e_baseline.py --startup` is the row set, added here:

| cell | ein.rs | ein.py CPython | ein.py PyPy |
|---|---:|---:|---:|
| `--help` | **1.02 ms** | 97.6 ms | 442.1 ms |
| `solve friends` (651 B, 1 rule, 1 fact) | **1.15 ms** | 132.3 ms | 542.4 ms |
| `saturate friends` | **1.20 ms** | 118.6 ms | 522.1 ms |

`/bin/true` on the same machine and the same timing loop is **0.23 ms**, so
0.8 ms is ein's own start. The binary is **3 581 440 B**, dynamically linked
against four libraries (`libstdc++`, `libgcc_s`, `libm`, `libc` — the first two
are snmalloc's), and the embedded stdlib is 67 369 B of it, **1.9 %**.

**The one number worth having: snmalloc costs 0.59 ms of every process
start.** The system-allocator build (`--no-default-features`) does `--help` in
0.43 ms and `solve friends` in 0.60 ms, against 1.02 and 1.15. That is the same
effect [§13](baseline.md#13-s1a62--the-layout-stage-and-the-profile-it-starts-from)
recorded as "0.5 ms" off `render rules zebra2`, measured now on a workload that
does no engine work at all, so it is the arena set-up and nothing else. It does
not change the decision — snmalloc is worth 8–16 % of a `solve`, so anything
past ~5 ms of engine work repays it — but it does mean **a corpus cell that
does nothing pays 0.59 ms to be able to allocate fast**, and the harness runs
thousands of those.

## Notes

- An embedder that loads once and asks many questions amortises this
  path away entirely, and `.einb`
  ([P1a.8](../p1a.8_binary_container/README.md)) skips it — so do not
  over-invest here at the expense of
  [S1a.6.3](s1a.6.3_beta_memories.md). The user-visible win is
  bounded by "already imperceptible".
- If `rebuild_indexes` shows up, check first whether the loader is
  calling it more than once. **It does not.**
- **The note above was the right instinct and it was still worth the day.**
  A load is 0.9 % of `solve zebra -e` and the acceptance was met before the
  stage started — but it is 18 % of a `saturate zebra2` process, and it is the
  *whole* of the 99 `parse-negative` and `load-negative` cells the harness runs
  per tier. And the thing that was actually wrong — a module parsed four times
  — would have gone on being wrong inside `.einb`'s producer too.
