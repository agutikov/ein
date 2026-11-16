# S1f.10.16 — What the walk left in the generator

**Phase:** [P1f.10](README.md)
**Status:** planned 2026-11-15, at the re-plan. **Ran to its end, 2026-11-16** ([§ Record](#record-2026-11-16)):
`generate_one_branch` and `one_branch` with their selector `oblgen::Choice`, `score_hypothesis` with
`most-constrained`, and the lookahead's unread kill list are deleted; the lookahead, its kill cache, the walk's
`generate` and `emit_closed` are kept, read by the shape golden's `hyp` and `hyp+closed` ops alone, and whether
those ops stay is put to the user as Q-M1f.312, not acted on; the retired config names stay refused at load and
in the `.einb` layout; the stale headers and pages are restated. No golden moved. **As ruled at the close,
2026-11-16** ([§ Record](#record-2026-11-16-as-ruled-at-the-close)): Q-M1f.312 (e), both ops removed and what only
they read deleted — the walk's `generate`, `complete`, the lookahead and its kill cache, `emit_closed` — with K-29's
no-good store and K-30's `is_solution_node`; the golden lost its 1 322 `hyp` and `hyp+closed` lines and nothing else.
**Depends on:** nothing. It runs with whichever batch of code commits it fits.
**Takes over:** the old [S1f.10.5](s1f.10.5_ordering.md)'s *`most-constrained`, implemented or deleted*, and
the old [S1f.10.7](s1f.10.7_tree_calibration_and_flag.md)'s call on `generate_one_branch`.

## Context

The walk was deleted on 2026-11-03 and the generator stayed, because H is built by its ladder. Some of what
stayed has no reader on any command's path. This phase is about to write code beside it and pages about it, so
it settles each piece first: deleted, or kept with the reader that needs it named at the item. An argument for
leaving something alone holds only while its premise is enforced by something that fails
([`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)).

## The items, as found on 2026-11-15 at `dbedc535`

| | item | what was found |
|---|---|---|
| 1 | `hypgen::generate_one_branch`, and the `one_branch` argument it threads through `generate_rungs`, `generate_with` and `oblgen::generate` | `pub`, and nothing calls it. It was the walk's tree's generator. The obligation tree reads `oblgen::alternatives` instead |
| 2 | `hypgen::score_hypothesis`, re-exported from `lib.rs`; the mode `most-constrained`, which returns `0.0`; `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight` | called by its own tests. The fields are in `config.rs`'s list of retired names with the reason *an order lever, and every route's order is part of its definition*, and still in its `FIELDS` |
| 3 | `Lookahead`, the one-step simulator, and `enable-pre-branch-lookahead` | reached through `hypgen::generate`, which no command runs: the universe is taken by `generate_config_free`. Its readers are the shape digest's `hyp` op, the `hypgen_calls` example and tests |
| 4 | `lattice-order`, `lattice-order-seed`, `candidate-order-seed` and the other names retired at the switch that `FIELDS` still holds | what a program that sets one gets today is for the stage to read off `config_reference.rs` and state |
| 5 | module headers that describe the walk as the engine's search: `oblgen.rs`, § What it proposes (*a breadth-first lattice over root's `alive` set*); `hypgen.rs`, § What must not change (*it becomes `layer_1`'s singleton order*); `oblgen.rs`'s note on `apriori::order_candidates` | stale statements of what the engine does |

## Tasks

1. **T1f.10.16.1 — the readers.** For each item, every reader in the workspace named: a command's path, a
   test, a golden, an example, a page. For 3, in particular: which goldens hold the `hyp` shape
   (`corpus_shapes.md5`), and whether that op digests anything a route uses.
2. **T1f.10.16.2 — the dispositions**, each by one rule: **a piece no command's path, no golden and no page a
   user reads depends on is deleted with its tests; a piece one of them depends on is kept, and the dependence
   is written at the item.** A golden that would move is named here before it moves. If the `hyp` op digests
   only a generator no command runs, that is a finding about the digest, shown, and the op is not removed here
   without the user's word: it is part of the per-commit shape check.
3. **T1f.10.16.3 — the headers.** Item 5's three, and any other T1 finds, restated to say what the engine
   does, with no stage id and no date.
4. **T1f.10.16.4 — the pages.** [`configuration.md`](../../../docs/kernel/configuration.md)'s rows for every
   name touched, pinned by `config_reference.rs`; `docs/api/rust.md` if a `pub` item it shows goes.

## Acceptance

- Each item has a disposition recorded here, with its readers.
- `generate_one_branch` and `one_branch` are gone, or the reader that keeps them is named.
- **`most-constrained` is in one of two states**: it computes something a reader uses, or it does not exist. A
  documented mode that returns a constant does not survive the stage.
- No answer and no read-out changes; a golden that moved is on T2's list.
- `./run_tests.sh` green on the batch's last commit.

## What it must not do

- **Not delete the generator's ladder or `generate_config_free`**: H is built by them.
- **Not remove an op from the shape digest** on its own judgment.
- **Not bring a retired lever back** to give a piece a reader.

## Record (2026-11-16)

Run on `060576ec` (`master`'s head, the branch fast-forwarded to it), in a worktree. The items were found at
`dbedc535`; each was re-found against `060576ec` first, and the readers below are that tree's (`git grep` over
the workspace, the docs and the plans). The numbers marked *probe* are from a scratch example, kept as
`build/s1f.10.16/probe.rs` with its output (`probe.txt`, `probe2.txt`) and deleted from the tree: for every corpus
program that loads it builds the two ops' texts as the golden does, H's pass at root as `region::prepare_with`
runs it, and `generate`'s list with its lookahead kills on another fork of the same root.

### T1: the readers

A **deciding** reader is one the plan's rule names: a command's path, a golden, a page a user reads. Tests,
examples and records (`docs/history/`, `plans/`) are named and decide nothing.

**Item 1: `hypgen::generate_one_branch` and `one_branch`.** `generate_one_branch` is `pub` in a `pub` module and
not re-exported; **nothing calls it** (`git grep -n generate_one_branch`: its definition, and two M1e records,
`docs/history/m1e_review_processing/open_questions.md` and
`search_soundness_probes/d2_q6_which_decline_to_construct.md`, which describe the walk's tree that called it).
`one_branch` is threaded by `generate_rungs` and `generate_with` into `oblgen::generate`, and every other caller
passes `false`: `generate`, `generate_blind`, `generate_config_free`, and the two inner tests of `oblgen.rs`. The
obligation tree reads each owed instance's alternatives from L5's table (`oblgen::alternatives`, through
`explore.rs`'s `Space::table`), never from `oblgen::generate`.

T1 found one piece more, **`oblgen::Choice`**, the order `oblgen::generate` walks the owed instances in. Every
call passes `Choice::RuleOrder` (`hypgen::GenOptions::CONFIG_FREE` and `generate_with`'s config default);
`FailFirst` and `Off` are reached by no call. Its doc says it was kept as *the interface a depth-first traversal
would need on day one*, and the comment at `one_branch`'s selection says the heuristic was *measured inert under
a lattice and kept for this*: it is `one_branch`'s selector. Its one observable is the `rung` event's `reason`,
`rule-order` on every call: in `--events` streams, in the `hyp` and `hyp+closed` digests, and asserted by
`oblgen.rs`'s test `an_obligation_that_scans_a_relation_a_commitment_derives_declines`. Readers that decide
nothing: `obligation_rung.rs`'s `the_lever_and_the_modes_keep_their_names` (the three `as_str`), and three pages,
`implementation.md` (the `oblgen.rs` row), `events.md` (the `rung` line's `reason`) and `configuration.md` (§ the
`EIN_*` names, the history of `Choice::from_env`).

**Item 2: `score_hypothesis`, `most-constrained`, the three scoring names.** `hypgen::score_hypothesis` and
`ScoreError` are `pub` and re-exported from `lib.rs`; **their only callers are their own three unit tests**
(`no_config_block_scores_neutral_not_popular`, `popularity_weighs_the_relation_and_each_named_argument`,
`the_unwired_modes_report_their_fixed_text`). No command, no golden. Four pages name it, none depends on it:
`implementation.md`'s `hypgen.rs` row, `reserved_engine_strings.md` § Protocol enums (a `hypgen scoring` row,
`most-constrained · popularity`), `inference/README.md` § Determinism (a reconcile note on the removed tree
solver's order, whose score key it was) and `architecture_and_algorithms.md` O8's *Ein today*, the record by its
banner. `most-constrained` is a match arm of `score_hypothesis` returning `0.0`, and one test; elsewhere the word
is a refused value in `ein-ir/tests/golden/config_coercion.txt`, a cell of `utils/feature_matrix.py` (the walk's
instrument, run against a binary at or before `7ec8a480`), rows of `features.md` (the walk's record) and English
in a comment of `examples/branching/09_hrule.ein`.

`hypgen-scoring`, `hypgen-rel-weight` and `hypgen-obj-weight` are in `ein_core::config::RETIRED`, which the
grammar reads (`ein-ir/src/grammar.rs`): a program that sets one is refused at load, on every command (item 4).
Their fields are on `SolverConfig` and in `FIELDS`, read by `score_hypothesis` alone and by the `.einb` container
(`ein-einb/src/meta.rs`), which writes and reads every entry of `FIELDS` into a file's `META` section, in order.

**Item 3: `Lookahead` and `enable-pre-branch-lookahead`.** `Lookahead::new` is called by `hypgen::generate_with`
where `GenOptions::lookahead` is on, which is `generate`'s path alone: it reads
`SolverConfig::enable_pre_branch_lookahead` and `enable_lookahead_kill_cache`, retired, so `true` on every
program that loads (`examples/hypgen_cost.rs` sets the first on a loaded KB directly). `generate` and the two
predicates over it, `complete` (`complete_counted`) and `open_hypotheses`, are **run by no command**: H is
`generate_config_free`'s list (`region::prepare_with`), and `EIN_LEFTOVER`'s probe is `generate_blind`. Their
callers: `shape::hyp_shape_with`, for the shape golden's `hyp` and `hyp+closed` ops; the tests
`search_semantics.rs` (the lookahead's kill conditions, `Lookahead::new` directly; `generate` for the query
lists; the solution node), `compile_semantics.rs`, `explain_semantics.rs` and `hypgen_coverage.rs` (the eight
counters of the `hyp` text, `filtered.lookahead_killed` among them); and the examples `hypgen_calls.rs` and
`hypgen_cost.rs`.

**The goldens that hold it: `ein-render/tests/golden/corpus_shapes.md5`'s `hyp` and `hyp+closed`**, 659 lines
each, every corpus program that loads (391 outside `tests/`, which the per-commit sweep reads; `EIN_CORPUS_SLOW=1`
reads all). What the lookahead puts in them (*probe*): its `lookahead_killed` verdict in the `hyp` text of 59
programs; `Lookahead::new`'s `compile` events in 568; and its kill cache's `(not h)`, which turns a later repeat
of a killed candidate into `negated_fact` where H's pass says `seen_in_call`, in 29. So deleting the lookahead,
or its kill cache, moves at least the 568 `hyp` lines with a `compile` event and their `hyp+closed` twins.

T1 found two pieces more. **`HypGenStats::lookahead_killed`**, the list of the candidates the lookahead dropped,
is pushed on every kill and **read by nothing**: its doc and `open_hypotheses_counted`'s say a solve reads the
config-free universe from it, which no route does, H being `generate_config_free`'s list.
**`open_hypotheses_counted`** is called by `open_hypotheses` alone. Neither is in a golden: `report_lines`, which
the ops digest, reads the counter `filtered[LookaheadKilled]`, not the list.

**`closed::emit_closed`** ([`findings.md`](findings.md)'s H9-5): called outside its own test by
`shape::hyp_shape_with(…, true)` alone, the `hyp+closed` op, and by two tests (`cli_semantics.rs`,
`explain_semantics.rs`). Its pass closes a relation on 612 of the 659 programs (*probe*); on the other 47 the op's
text is `hyp`'s with a `CLOSED []` line.

**What the two ops digest, against what a route uses.** `generate` and H's pass are one body, `generate_with`,
and share everything but `GenOptions`: the switch (`rung`), the hrule and obligations rungs, `candidate_objects`,
the relation plan and the filters `negated_fact`, `fact_already_exists` and `seen_in_call`. H's pass runs neither
the lookahead nor its kill cache, and sorts and dedups its list by raw text (`region::order_key`, the
generator's order the tie-break) before any route reads it. The
*probe*, on all 659 programs: **H is exactly `generate`'s emitted candidates with its lookahead-killed ones**; and
`hyp`'s event stream is the narration of H's pass line for line once the `compile` events, the ordinals, the
lookahead's verdict and the kill cache's repeat verdict are set aside (identical outright on 91). So **the `hyp`
op digests the candidate stream of H's pass under two filters no route runs**, with a trailer, `COMPLETE` and
`OPEN`, read after the kill cache wrote; it is the golden's only digest of the generator's narration and
counters. **`hyp+closed` digests the same after a pass no command calls.** Neither digests anything a route reads
beyond what H's own pass computes; the routes read H, which the read-out's `universe` rows, `ein show hyps` and
every state hold.

**Item 4: the retired names `FIELDS` still holds.** What a program that sets one gets, read off
`config_reference.rs`: `the_retired_flags_are_refused_with_why` writes `(config :NAME true)` for each of
`RETIRED`'s fifteen and holds `ein solve` to exit 1 with `config flag :NAME was retired at M1e2 STAGE: WHY`. So
**every one of them, `lattice-order`, `lattice-order-seed`, `candidate-order-seed` and `print-alive` with the
rest, is refused at load**, as a parse error with the reason and a repair (`config_coercion.txt`,
`grammar_table.txt`, `load_semantics.rs`), on every command, and a `.einb` whose stored program sets one is
refused on open, since the container re-loads its program (`configuration.md` § 2.4). The fields: `FIELDS` holds
all seventeen and `SolverConfig` a field each; `rendered_fields`, which `--dump-config` and the KB shape print,
filters to `FLAGS`, so no retired field is printed anywhere; the container round-trips every entry of `FIELDS`;
`config.rs`'s `every_field_has_a_flag_and_no_flag_is_orphaned` pins 17, 2 and 15. Read in code: the lookahead's
two by `generate` (item 3), the scoring three by `score_hypothesis` (item 2), `lattice_order_seed`'s default by
one assertion of `compile_semantics.rs`; the other nine by nothing.

T1 found stale statements here: `SolverConfig`'s docs of `print_alive` and `candidate_order_seed` cite a test,
`config_reference.rs::the_two_inert_flags_are_still_inert`, that does not exist (it went when the two were
retired), and so does `config_reference.rs`'s own header, its claim 5; the docs of `lattice_order` and
`lattice_order_seed` describe the walk's orders as live (*"lex" or "score-sum"*, *a seed makes traversal order a
deterministic permutation*), and `hypgen_scoring`'s its modes.

**Item 5: the headers.** All three re-found: `oblgen.rs` § What it proposes, and why it is the union (*a
breadth-first lattice over root's `alive` set — layer `k` enters the `k`-subsets of one fixed set*); `hypgen.rs`
§ What must not change (*the candidate order … becomes `layer_1`'s singleton order and therefore the whole
traversal*); `oblgen.rs`'s note on `apriori::order_candidates`, which is `Choice`'s doc (item 1). T1 found these
others stating the walk as what the engine does: `shape::hyp_shape`'s doc (the same `layer_1` sentence);
`region.rs` § The universe (the lookahead *a config switch*, and *today's `alive₀` is taken after the
forced-positive cascade and through the lookahead*); `lookahead.rs` § Its sign (the lookahead a lever whose
default *does not move without* a measurement); `lib.rs`'s header (*two summary blocks still call it*);
`hypgen::eligible_relations`' note and `closed.rs`'s test doc (`emit_closed` *runs on a fork for `--hyp-stats`
and the summary*); `oblgen.rs`'s `Branch` doc (*what `Choice` orders and what a depth-first traversal would
branch on*); the headers of the examples `hypgen_calls.rs` (*called once per alive entering … once per layer*)
and `hypgen_cost.rs` (*the search loop that calls it ~100 times*). In the pages: `implementation.md`'s `hypgen.rs`
row (*a search's own root call records what the lookahead dropped … so a solve reads the same list*), and
`reserved_engine_strings.md` and `06_reserved_names.md`, which say `emit_closed` runs for the `hyp` op, where it
is `hyp+closed`.

### T2: the dispositions

By the plan's rule: **a piece no command's path, no golden and no page a user reads depends on is deleted with
its tests; a piece one of them depends on is kept, and the dependence is written at the item.**

| item | piece | what depends on it | disposition |
|---|---|---|---|
| 1 | `generate_one_branch`, the `one_branch` argument | nothing | **deleted**. No test calls it; the two inner tests that pass `false` lose the argument |
| 1 | `oblgen::Choice`, its `FailFirst` and `Off`, the `choice` argument | the `rung` line's `reason`, `rule-order` on every call | **deleted**; the reason is narrated as the constant it always was, so the protocol and every golden are as they were. `obligation_rung.rs` loses the three `Choice` assertions |
| 2 | `score_hypothesis`, `ScoreError`, the mode `most-constrained` | nothing: no command, no golden; four pages describe it, none depends on it | **deleted with its three tests**; `implementation.md` and `reserved_engine_strings.md` restated, `inference/README.md`'s note told it went. `most-constrained` then exists as no mode |
| 2, 4 | the fifteen retired names, the three scoring ones among them, in `RETIRED`, `FIELDS` and `SolverConfig` | the loader's refusal on every command (`RETIRED`), and the `.einb` layout (`FIELDS`: `ein kb save` writes it, every command that opens a `.einb` reads it) | **kept**. A program that sets one is refused at load with the reason, as it is today. The fields stay at their defaults because the container writes `FIELDS` whole: removing one is a layout change, a `FORMAT_MAJOR` bump that refuses every `.einb` saved before it. Their docs are restated to what reads each |
| 3 | `Lookahead`, the kill cache, `enable-pre-branch-lookahead` and `enable-lookahead-kill-cache` as `generate` reads them, `generate`, `complete`, `open_hypotheses` | **the shape golden's `hyp` and `hyp+closed` ops**: without the lookahead at least 568 of their lines move each | **kept**, the dependence written in `lookahead.rs`, `hypgen.rs` and `shape.rs`. The ops are not removed here: whether they stay, and on what, is [Q-M1f.312](../open_questions.md#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest), put |
| 3 | `HypGenStats::lookahead_killed`, `open_hypotheses_counted` | nothing (`report_lines` reads the counter, not the list) | **deleted** |
| 3 | `closed::emit_closed` (H9-5) | the `hyp+closed` op | **kept**, the dependence written in `closed.rs`; Q-M1f.312 |
| 5 | the headers of item 5 and T1's | — | restated to what the engine does (T3), with no stage id, plan path, finding id or date |

**Not acted on, found by T1 outside the five items**, each a finding: `hypgen::is_solution_node` and
`consistent`, the walk's definition of an answer, read by one test alone (K-30); `ein_core::Kb`'s learned no-good
store, whose doc describes the walk's fan-out as the shipping engine (K-29).

### What will move, named before the code changes

**No golden moves.** Every deletion above is of code no output reaches, and the `rung` line's `reason` stays
`rule-order`. Held unmoved:

- `ein-render/tests/golden/corpus_shapes.md5`: none of its lines, the 659 `hyp` and 659 `hyp+closed` among them,
  per commit and with `EIN_CORPUS_SLOW=1`;
- `ein-ir/tests/golden/config_coercion.txt` and `grammar_table.txt`: the retired names' refusals;
- `ein-cli/tests/golden/help_shape.txt` and `corpus_exits.txt`; `configuration.md`'s generated defaults block
  (`FLAGS` is untouched); `read_out_examples.md`; `docs/api/rust.md`'s block, which shows none of the pieces.

The pages edited, in prose: `docs/kernel/inference/implementation.md`, `events.md`, `reserved_engine_strings.md`,
`inference/README.md`'s reconcile note, `docs/kernel/configuration.md` § 2.4 and § the `EIN_*` names, and
`ir/03-ein-lang/06_reserved_names.md`. A golden that moves and is not on this list is a stop.

### What was deleted

Over `ein.rs/`, 426 lines out and 188 in, doc comments included, 390 of the 426 in `ein-infer`
(`git diff -U0 8feb21ad -- ein.rs`, counted on the code commit):

- `hypgen::generate_one_branch` (`pub`), the private `generate_rungs`, and the `one_branch` argument of
  `generate_with` and `oblgen::generate`; with it `oblgen::generate`'s selection of one instance and its guard
  on the later debts' candidates. `generate` calls `generate_with` directly.
- `oblgen::Choice` (`pub`: `RuleOrder`, `FailFirst`, `Off`, `as_str`), the `choice` argument of
  `oblgen::generate` and `narrate`, `GenOptions::choice`, the `FailFirst` sort and the `Off` decline. The
  `rung` line's `reason` is `oblgen::ORDER`, `pub const ORDER: &str = "rule-order"`, the one value it ever
  carried. `obligation_rung.rs`'s test is `the_order_and_the_modes_keep_their_names`, holding `ORDER` where it
  held the three `as_str`; `oblgen.rs`'s own test still asserts the narrated `"reason": "rule-order"`.
- `hypgen::score_hypothesis` and `hypgen::ScoreError` with its `Display` (both `pub`, both re-exported from
  `lib.rs`, the re-exports gone), and their three unit tests with the `scored` helper. `most-constrained` is no
  mode of anything in the engine.
- `HypGenStats::lookahead_killed` (`pub` field) and its push in `apply_filters`; `hypgen::open_hypotheses_counted`
  (`pub`), folded into `open_hypotheses`, its one caller.

Kept, each with its reader written where it lives: `Lookahead` and its kill cache (`lookahead.rs` § Who runs
it), `hypgen::generate`, `complete`, `open_hypotheses` (`generate`'s doc), `closed::emit_closed` (`closed.rs`'s
header), all for the shape golden's two ops (`shape::hyp_shape`'s doc); the retired fields of `SolverConfig` and
`FIELDS` for the loader's refusal and the `.einb` layout (`config.rs`' `RETIRED` and the fields' docs).

### T3: the headers

Restated to what the engine does, with no stage id, plan path, finding id or date added in what was written
(a sentence kept beside a restated one keeps its own, `S1.9.E16` and the like; the restated sentences that
dropped one dropped it):

- **`oblgen.rs` § What it proposes, and why it is the union**: the rung builds H, one set built at root, so it
  proposes the union in report order; the branch on one instance is the obligation tree's, which reads L5's
  table, `alternatives`, scanned by the same projection.
- **`hypgen.rs` § What must not change, the candidate order**: a route reads it only as the tie-break of H's sort
  by raw text (`region::order_key`); it is observable as the order of the `hyp` events and which repeat
  `seen_in_call` counts.
- **`oblgen.rs`'s note on `apriori::order_candidates`** went with `Choice`; `ORDER`'s doc says what the order is
  and where it reaches. `Branch`'s doc no longer names `Choice` or a depth-first traversal.
- T1's others: `shape::hyp_shape`'s doc (what it runs is the walk's generator, how its stream stands to H's
  pass, and that it is the reason the lookahead stays); `region.rs` § The universe (config-free, sorted by raw
  text, a candidate the lookahead would drop a node like any other); `lookahead.rs` § Its sign, now § Who runs
  it; `lib.rs`'s header; `hypgen::generate`'s, `generate_config_free`'s and `generate_with`'s docs;
  `eligible_relations`' note and `closed.rs`'s header and test doc on `emit_closed`; `ein-core/src/config.rs`'
  `RETIRED` doc and the retired fields' docs (each *retired*, what reads it, kept for the `.einb` layout; the
  citation of the test that does not exist gone); `config_reference.rs`' header, whose claim 5 names
  `the_retired_flags_are_refused_with_why`; the headers of the examples `hypgen_calls.rs` and `hypgen_cost.rs`
  (the calls they time are the walk's, run today by the `hyp` ops alone).

### T4: the pages

- **`configuration.md`**: § 2.4's paragraph says what reads each retired field (the lookahead's two by
  `hypgen::generate`, which the `hyp` ops run; the other thirteen by nothing) and that removing a field is a
  `.einb` layout change; § the `EIN_*` names says the rung has one order, named `rule-order`. Its tables, its
  generated block and its counts are as they were, and `config_reference.rs` holds them: the names touched are
  the fifteen retired ones, whose behaviour, a load refusal with the reason, is what
  `the_retired_flags_are_refused_with_why` holds and § 2.4 states.
- **`implementation.md`**: the `hypgen.rs` row (no `score_hypothesis`; `generate`'s readers; the
  `lookahead_killed` sentence gone), the `oblgen.rs` row (no `Choice`), the `lookahead.rs` and `closed.rs` rows
  (who runs each).
- **`events.md`**: the `rung` line's `reason` is `rule-order`, the one order, described as what it is.
- **`reserved_engine_strings.md`**: the `hypgen scoring` row of § Protocol enums is gone, with a sentence
  saying where the modes went and that the flag is refused; `emit_closed`'s caller is the `hyp+closed` op.
- **`inference/README.md`**: the reconcile note says `score_hypothesis` is deleted, and where H's content sort is.
- **`06_reserved_names.md`**: `emit_closed` runs for the `hyp+closed` op.
- **`docs/api/rust.md`**: unchanged. No `pub` item it shows went (`embedding.rs`' region shows none of them).

### Defaults taken

Each the user's to revise.

1. **`oblgen::Choice` was taken as item 1's**, though the plan does not name it: its doc and the comment at the
   selection say it was kept for the depth-first move `one_branch` made, and no call passes `FailFirst` or
   `Off`. Deleted whole rather than left with one variant; the narrated `rule-order` is kept as a constant, so the
   events protocol does not move.
2. **`HypGenStats::lookahead_killed` and `open_hypotheses_counted` were taken as item 3's**: the lookahead's
   output and its one stated reader, both unread.
3. **The retired fields stay on `SolverConfig` and in `FIELDS`**, the fewest surfaces: removing them is a
   `.einb` `FORMAT_MAJOR` bump that refuses every saved cache. What a program that sets one gets is unchanged.
4. **`is_solution_node` and `consistent` were not acted on**, though reached through `generate` as item 3 is:
   not one of the five items, filed as K-30. Nor `Kb`'s no-good store, K-29.
5. **Q-M1f.312's default is (b)**, re-point `hyp` at H's pass and remove `hyp+closed`, not (a): the rule the
   stage settled every other piece by. Not acted on.
6. **The records under `docs/history/` and the walk's record pages** (`features.md`, the O8 paragraph of
   `architecture_and_algorithms.md`, the body of `inference/README.md`'s § Determinism) keep their mentions of
   the deleted names: they describe what was, by their banners.
7. **The probe was not banked** as a test: it measured the ops for the question and is kept under `build/`; an
   invariance between `hyp` and H's pass would be a new per-commit check, which is the question's to decide.

### Verification

In the worktree, on the gate profile, on `55dec562`, the code commit; nothing under `ein.rs/` or `docs/` changed
after it, and this record is the only later edit:

- `cargo test --profile gate -p ein-infer -p ein-cli -p ein-render -p ein-corpus`, whole crates: 1 072 tests,
  green. A first run on the tree before the last comment edit, with `-p ein-core` added, was green too (1 165).
- `EIN_CORPUS_SLOW=1`, once each: `-p ein-render --test id_order_invariance` and `-p ein-infer --test explore`,
  green.
- **The shape golden did not move**: `git diff 060576ec -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5
  | grep -c '^[-+][0-9a-f]'` is 0, per commit and after the slow run, and no other golden is in
  `git diff --name-only 060576ec`. So `lever_invariance`, `explore`, `claim_routes`, `read_out_reference` and
  `config_reference` held every answer and read-out where they were.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean. `python3 utils/check_hashmap_iteration.py`:
  ok. `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md
  examples/README.md stdlib/README.md utils/README.md c/README.md`: 0 findings, after this record too. The tests'
  scratch directory was left empty.
- Not run, as the stage was told: `./run_tests.sh`, the nightly, `utils/ci_container.sh`. No wall here is a
  measurement.

### What is not done

- **The two ops**: not removed or re-pointed; Q-M1f.312 is put, not ruled. Until it is, the lookahead, its kill
  cache, the walk's `generate` with `complete` and `open_hypotheses`, and `emit_closed` stay.
- **K-29 and K-30** are filed, not acted on: `Kb`'s no-good store, and `is_solution_node` with `consistent`.
- **The retired fields** stay in `FIELDS` and on `SolverConfig`; their removal is a `.einb` format change no
  stage has been asked for.
- `./run_tests.sh`, the nightly and the runner's container: the main session's, on the batch.

**Findings outside the stage's scope**: [K-29 and K-30](findings.md#from-the-stages-of-tracks-s-and-r); K-30 is
now the last.

## Record (2026-11-16): as ruled at the close

[Q-M1f.312](../open_questions.md#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest) was ruled **(e)**
at the phase's close (*"(e) Remove both"*), and K-29 and K-30 were to be fixed now, as proposed
([`close.md`](close.md) § The user's answers). So both ops leave the shape golden, and what only they read is
deleted by this stage's rule: **a piece no command's path, no golden and no page a user reads depends on is
deleted with its tests**; a piece one of them still depends on stays, with that reader named.

Run on `cb0fbde8`, in a worktree. Every reader below was re-found on that tree before anything was deleted
(`grep -rn` over `ein.rs/crates`, `docs/`, `utils/` and the top-level pages), and this inventory is committed
before the code.

### The inventory: what goes, with its readers

A **deciding** reader is a command's path, a golden, or a page a user reads; tests, examples and records decide
nothing. Once the two ops are gone, each piece below has none.

| # | piece | readers on `cb0fbde8` | disposition |
|---|---|---|---|
| 1 | `corpus_ops::Op::Hyp`, its two entries in `ops()` (`hyp`, `hyp+closed`), its `Display` and `run_loaded` arms | `id_order_invariance` (the golden and the permuted runs); `jobs_invariance` reads `ops()` but runs only `takes_jobs`, which `Hyp` is not | **deleted**, as ruled |
| 2 | `shape::hyp_shape`, `shape::hyp_shape_with` (`pub`, re-exported) | `corpus_ops` (item 1); `hypgen_coverage.rs` | **deleted** |
| 3 | `ein-infer/tests/hypgen_coverage.rs` | reads `hyp_shape_with`'s text for eight counters; its stated reason is that the `hyp` digests are evidence about the filters | **deleted**: the digests it vouched for are gone, and the filters H's pass runs are held by what H decides (the read-out's `universe` rows, every route's answer) |
| 4 | `hypgen::generate` (`pub`, re-exported) and its reading of `enable_pre_branch_lookahead` and `enable_lookahead_kill_cache` (`GenOptions`, the `opts` argument of `generate_with`) | `hyp_shape_with`; `complete_counted`, `open_hypotheses` (item 5); the tests `compile_semantics.rs` (`guessed_relations`), `explain_semantics.rs` (`hypotheses`) and `search_semantics.rs` (`candidates`); the examples `hypgen_calls.rs` and `hypgen_cost.rs` | **deleted**. The three test helpers ask what the generator proposes, scoped by the program's lists, closures and signatures, which H's pass decides: they are re-pointed at `hypgen::generate_config_free`, the pass every command runs. `generate_with` takes no options: it is the config-free pass |
| 5 | `hypgen::complete`, `complete_counted`, `open_hypotheses` (`pub`; the first and last re-exported) | `hyp_shape_with` (its `COMPLETE` and `OPEN` lines); `hypgen_calls.rs`; `search_semantics.rs`' `solution_node_needs_both_halves`; a doc row of `kb.rs` | **deleted** |
| 6 | `hypgen::consistent`, `hypgen::is_solution_node` (`pub`, re-exported): **K-30** | `search_semantics.rs`' `solution_node_needs_both_halves` alone; named by `architecture.md`'s *complete model* row, `architecture_and_algorithms.md` § 2 and `implementation.md`'s `hypgen.rs` row | **deleted with that test**. The answer is R; `contradiction::has_contradiction`, which `consistent` wrapped, stays |
| 7 | the lookahead: `lookahead.rs` whole (`Lookahead`, re-exported, `dies_immediately`, its three unit tests); in `hypgen`, `Ctx::lookahead`, `Ctx::kill_cache`, the filter's arm in `apply_filters`, `write_negated` (the kill cache) and `Drop::LookaheadKilled`; `Matcher::run_seeded_at` and `Matcher::holds_seeded` (`pub`) | `generate_with` where `generate`'s config switched it on (item 4); `hypgen_calls.rs` (`Lookahead::new`); `search_semantics.rs`' `dies` and its three kill-condition tests; `run_seeded_at` and `holds_seeded` are called by `dies_immediately` alone; `Drop::LookaheadKilled` is named by `ein-cli/src/summary.rs`' sparse `filtered` map, which H's pass never bumps (`answer_dump.md`: *it never appears*) | **deleted** with those tests. The summary's map lists the three filters H's pass runs; its output does not move, since a zero count prints no key |
| 8 | `HypGenStats::report_lines`, `HypGenStats::balances`, `DROPS`, `SKIPS`, and the unit test `the_stats_report_omits_the_keys_that_never_fired` | `hyp_shape_with` (its `STATS` and `BALANCE` lines) alone: `--hyp-stats` prints H by relation, not these lines | **deleted** |
| 9 | `closed.rs` whole: `emit_closed`, `producible_relations` (`pub`, re-exported), `closed::CLOSED` (a copy of `hypgen::CLOSED`), its unit test | `hyp_shape_with(…, true)`, the `hyp+closed` op; `cli_semantics.rs`' `root_saturated`, which runs it before root's saturation; `explain_semantics.rs`' `emit_closed_on`, run on both arms of `fork_and_direct_saturation_agree`; `producible_relations` by `emit_closed` alone; `closed::CLOSED` by nothing | **deleted**. `root_saturated` then saturates root as `ein solve` does, which its doc says it does; `fork_and_direct_saturation_agree` compares the two arms without the pass, which neither needs |
| 10 | the examples `hypgen_calls.rs` and `hypgen_cost.rs` | time `generate`, `complete`, `open_hypotheses` and `Lookahead::new`; no `utils/README.md` row, no script runs them | **deleted** |
| 11 | counters `hypgen_complete` and `lookahead_probe` | bumped by `complete_counted` and `dies_immediately` alone; `rows()` lists them; no script or page reads either by name outside `docs/history/` | **deleted**; `rows()` holds 54 |
| 12 | **K-29**: `ein_core::Nogoods`, `NogoodFreeze` (`pub`, re-exported), `Kb::nogoods`, `Kb::freeze_nogoods`, the `nogoods` field and its handling in `Kb`'s `Clone`, `new`, `with_program`, `branch`, `snapshot` and `diff`; `kb.rs`' three tests of the store; the `.einb` `NOGOODS` section (`sections::write_nogoods`, `read_nogoods`, `Kind::Nogoods`) | the container: `ein kb save` writes the section **only when the store is non-empty**, and opening a `.einb` inserts a file's clauses. No route writes a clause (`grep -rn 'nogoods()' ein.rs/crates`: the container and `kb.rs`' tests), so the store is empty on every KB a command builds | **deleted, with no layout change.** No 2.x writer can have written the section: it is written only from a non-empty store, the store is filled only by a file's own section, and this reader refuses 1.x files, the last a writer with clauses could have made (the walk's learned clauses went before format 2.0). So a 2.0 file reads the same before and after, and kind 9 is not reused: a section of it is an unknown one and skipped, as kind 3 is. `Kb::snapshot` stays, read by the `kb-since` view and `kb_semantics.rs`: it is a fork |

### What stays, with its reader

| piece | the reader that keeps it |
|---|---|
| `hypgen::generate_config_free`, `generate_blind`, the ladder (`generate_with`, `rung`, the three rungs, the filters `negated_fact`, `fact_already_exists`, `seen_in_call`), `HypGenStats` and its counters | H at root (`region::prepare_with`), every command; `EIN_LEFTOVER`'s probe; the `--json-summary` root block; `ein show hyps`; the `hyp` and `hypskip` events |
| `Drop` (three variants) and `Skip` | the `hyp` event's `verdict`, the summary's sparse `filtered` and `pre_candidate` maps |
| `<lookahead-dies-immediately>`: `terms::LOOKAHEAD_DIES`, its `ENGINE` entry and `Kernel::lookahead_dies` | **the grammar**: the generated lexer's `WRITEBACK` token refuses the name wherever a program writes it (`ein.l`, `text::is_symbol`, `engine_names.rs`, `00_ebnf.md` § 1), a language contract no ruling asked to change; and the kernel's id layout, where a kernel name's id is its position in `Terms::new`'s list and dropping one moves every later id. No pass writes it any more |
| the fifteen retired config fields on `SolverConfig` and in `FIELDS`, the lookahead's two among them | `RETIRED`'s refusal at load, on every command, and the `.einb` `META` layout, which writes `FIELDS` whole (the user accepted that default). After this no code reads either of the lookahead's two |
| `-L/--no-lookahead` and `-K/--no-kill-cache` | the CLI's retired flags, refused with the reason (`solve.rs`, `region_modes.rs`) |
| `Matcher::holds`, `Engine::with_memo`, `Engine::plan_arc`, `firing::build_conclusion`, `compile::asserted_relation` | the saturator, the mechanisms check, `naf_deps`, `oblgen`, `obligations`, `verdict`, `detector`, `hrule`, `scope`, `forbid`, the plan shape |
| the `examples/branching/*lookahead*` and `*kill_cache*` fixtures | corpus programs, each with its answer held; their headers describe the walk's lookahead, which is a finding (K-71), not this stage's edit: editing a corpus file's text moves its parse digests |
| the `SOLUTIONS` section's `nogoods_emitted` and `nogoods_subsumed` | the `.einb` layout of a saved solution set; outside this stage |

### The goldens that will move

- **`ein-render/tests/golden/corpus_shapes.md5`: every `::hyp` and every `::hyp+closed` line, 661 of each**
  (`grep -c '::hyp$'`, `grep -c '::hyp+closed$'`), 393 of each outside `tests/` (the per-commit sweep's) and 268
  of each under it (`EIN_CORPUS_SLOW=1`'s). They are removed by a bless with `EIN_CORPUS_SLOW=1`, and no other
  line of the file may move.
- **No other golden.** Held unmoved: every other line of `corpus_shapes.md5`; `ein-cli/tests/golden/help_shape.txt`
  and `corpus_exits.txt`; `ein-ir/tests/golden/*` (`config_coercion.txt` and `grammar_table.txt` hold the retired
  names' refusals and the writeback name's); `ein-core/tests/golden/*`; `ein-render/tests/golden/` beyond the
  manifest; `configuration.md`'s generated block (`FLAGS` is untouched); `read_out_examples.md`;
  `docs/api/rust.md`'s block, which shows none of the pieces. **A golden that moves and is not on this list is
  a stop.**

### The pages to restate

The current pages under `docs/kernel/` that state a piece above as present, by
`grep -rn 'hyp+closed\|hyp_shape\|emit_closed\|lookahead\|kill cache\|is_solution_node\|nogood' docs/kernel`, read
mention by mention: `architecture.md`, `configuration.md` § 2.4, `glossary.md`, `ir/01-ein-graph/03_ein_model.md`,
`ir/03-ein-lang/01_grammar.md` and `06_reserved_names.md`, and under `inference/`: `README.md`,
`absent_semantics.md`, `answer_dump.md`, `architecture_and_algorithms.md`, `events.md`, `implementation.md`,
`reserved_engine_strings.md`, `solution_semantics.md`. The three superseded pages (`algorithm_layer_n.md`,
`lattice_diagrams.md`, `parity_baselines.md`) and their diagrams are not rewritten. A mention that is the
language's (the writeback name), a retired name's, a fixture's file name, `N`'s *minimal nogoods*, or a sentence
stating what the walk did, in the past, stays.

### What was deleted

Three commits on the branch: the inventory above (`c7c1e4fd`), the code with the golden (`26d0bc56`), the pages
(`ff63da53`); this record is the fourth. Over `ein.rs/` less the golden, 1 898 lines out and 150 in
(`git diff --shortstat c7c1e4fd 26d0bc56 -- ein.rs ':!ein.rs/crates/ein-render/tests/golden'`), 1 445 of them out of
`ein-infer`. Every item of the inventory went, as listed, with these particulars:

- **`ein-infer`**: `lookahead.rs`, `closed.rs`, `tests/hypgen_coverage.rs`, `examples/hypgen_calls.rs` and
  `examples/hypgen_cost.rs`, whole. In `hypgen.rs`: `generate`, `GenOptions` and `generate_with`'s `opts` argument
  (it is the config-free pass, and `generate_config_free` and `generate_blind` call it with the rungs alone),
  `Ctx::lookahead`, `Ctx::kill_cache`, `Ctx::matcher`, the lookahead's arm of `apply_filters`, `write_negated`,
  `Drop::LookaheadKilled` (so `HypGenStats::filtered` holds three), `report_lines`, `balances`, `DROPS`, `SKIPS`, the
  section *Solution predicates* (`open_hypotheses`, `complete`, `complete_counted`, `consistent`,
  `is_solution_node`) and the unit test of the report. `match_.rs`: `run_seeded_at`, `holds_seeded`. `shape.rs`:
  `hyp_shape`, `hyp_shape_with`. `lib.rs`: the modules `closed` and `lookahead` and every re-export of the above.
  Tests: `search_semantics.rs` lost `dies` with its three kill-condition tests and `solution_node_needs_both_halves`;
  its `candidates`, `compile_semantics.rs`' `guessed_relations` and `explain_semantics.rs`' `hypotheses` ask
  `hypgen::generate_config_free`; `explain_semantics.rs`' `emit_closed_on` and `cli_semantics.rs`' call of
  `emit_closed` went, and an assertion message that printed `report_lines` prints the skip counters. **That
  message was a reader the inventory missed**: found by the build, a test's failure text, deciding nothing.
- **`ein-core`**: `Nogoods`, `NogoodFreeze` and their re-exports; `Kb`'s field, `nogoods`, `freeze_nogoods`, and
  the store's handling in `Clone`, `new`, `with_program`, `branch`, `snapshot` (now `self.fork()`) and `diff`; the
  tests `a_fork_shares_no_goods_and_a_snapshot_copies_them`, `a_frozen_store_refuses_a_write`,
  `the_freeze_lifts_when_the_guard_drops`. The counters `hypgen_complete` and `lookahead_probe`.
- **`ein-einb`**: `sections::write_nogoods`, `read_nogoods`, `Kind::Nogoods`; `header.rs` says what kind 9 was.
- **`ein-cli`**: `summary.rs`' `filtered` map names the three filters, and `sparse` takes a slice.
- **`ein-render`**: `corpus_ops::Op::Hyp`, its two entries in `ops()`, its arms.

### The goldens that moved

`corpus_shapes.md5`, blessed once with `EIN_BLESS=1 EIN_CORPUS_SLOW=1`, which reads every corpus program and, under
the slow flag, permutes the id space of every one: 1 322 lines out, none in. The filter over the diff:

```sh
git diff -U0 cb0fbde8 26d0bc56 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 \
  | grep '^[-+][0-9a-f]' | grep -vcE '^-[0-9a-f]{16} +[0-9]+L  .*::hyp(\+closed)?$'   # 0: no other line moved
git diff -U0 cb0fbde8 26d0bc56 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 | grep -c '^+[0-9a-f]'   # 0
git diff -U0 cb0fbde8 26d0bc56 -- …corpus_shapes.md5 | grep -cE '::hyp$'          # 661
git diff -U0 cb0fbde8 26d0bc56 -- …corpus_shapes.md5 | grep -cE '::hyp\+closed$'  # 661
grep -cE '::hyp(\+closed)?$' ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5        # 0 left
```

No other golden is in `git diff --name-only cb0fbde8 26d0bc56`. So the inventory's list was the list.

### The pages

Restated to what the engine does, with no plan path, stage id, finding id or date added: `architecture.md` (the
worlds and complete-model rows; L3 has no writer left, L4 is gone with the code it named, and the leak list's
counts), `configuration.md` § 2.4 (no engine code reads a retired field; its tables and generated block as they
were, so `config_reference.rs` holds them unchanged), `defined_behaviour.md` (no pipeline writes between two
candidates), `implementation.md` (the `hypgen.rs` row; `lookahead.rs` and `closed.rs` marked deleted, unlinked;
`SolverConfig`'s row), `reserved_engine_strings.md` (the writeback name has no writer; `__closed__` has one engine
side, `hypgen.rs`'s), `inference/README.md` (the banner, the guard evaluation the lookahead was, the generator's
link), `absent_semantics.md` (C7, P6, D3), `answer_dump.md` (two rows' prose; the row names the test reads
unchanged), `architecture_and_algorithms.md` (the banner, a § 3 row, O2's gap, O8's *Ein today*),
`solution_semantics.md` (§ 1.4's `alive₀`, § 7's note), `03_ein_model.md`, `01_grammar.md`'s signature row and
`06_reserved_names.md`'s `__closed__` row. Kept as they are, each a record or not a statement about the deleted
code: § 6 of `solution_semantics.md`, the banner'd sections of `inference/README.md`, `features.md`,
`lattice_dump.md`, `region_dump.md`'s past tense, `events.md`'s walk kinds, `standard_of_proof.md`'s worked
example, `glossary.md`'s *Hypothesis set*, and `domain_elim_vs_hypothesis.md`, which is K-12's and P1f.20's.
`docs/api/rust.md` is unchanged: no `pub` item it shows went. `AGENTS.md` is unchanged: it names neither op, and
no rule or command it states moved. `utils/README.md` has no row of either example.

**Left for the merge**: `hypgen::candidate_objects`' doc still says, in a parenthesis, that a `(not h)` is
*written mid-call by the kill cache*. The paragraph is the one H9-1's branch edits, so it is not touched here to
keep the two branches from conflicting; the parenthesis goes with that edit or at the merge.

### Defaults taken

Each the user's to revise.

1. **`hypgen_coverage.rs` deleted, not re-pointed at H's pass.** Its stated reason was the `hyp` digests; with
   them gone the counters it floored are in no golden, which is (e)'s stated consequence, and re-pointing it would
   be a new check of H's narration that the ruling did not ask for.
2. **`Drop::LookaheadKilled` deleted with the filter**, though the summary's map named it: H's pass never bumped
   it, so the summary prints the same.
3. **`Matcher::run_seeded_at` and `holds_seeded` taken as the lookahead's**: `pub`, and called by
   `dies_immediately` alone.
4. **`closed.rs` deleted whole**: once `emit_closed` and `producible_relations` went, its `CLOSED` was a copy of
   `hypgen::CLOSED` that nothing read. The pages point at `hypgen.rs`.
5. **K-29 deleted without a format bump**, against the finding's first reading: no 2.x file can hold the section,
   so 2.0's layout is unchanged, and kind 9 is skipped as an unknown kind, as kind 3 is.
6. **The writeback name and its kernel symbol kept**: removing either is a grammar change or an id-layout change,
   and no ruling asked for one.
7. **`Hrules::candidates`' ahead branch kept**, its doc restated: it is on H's path, not *read only by the ops*;
   K-73.
8. **The fixtures named for the lookahead not edited**: a corpus file's text is digested; K-71.

### Verification

On the gate profile, on `ff63da53` (the code as of `26d0bc56`, the pages after it; nothing under `ein.rs/` changed
after the code commit), with `TMPDIR` under `build/`:

- `cargo test --profile gate --no-fail-fast -p ein-infer -p ein-render -p ein-corpus -p ein-core -p ein-ir
  -p ein-einb -p ein-narration`, whole crates: 954 tests in 98 binaries, green; `explore`, `claim_routes`,
  `stdlib_coverage` and the per-commit `id_order_invariance` among them. A first run without `--no-fail-fast`
  stopped at `ein-einb`'s `a_saturated_zebra2_is_small_and_opens_cold_in_under_a_millisecond`, a wall clock, at
  29.7 ms against its 20 ms budget under a load average of 16 to 31 from the other worktrees' runs; alone it took
  10.7 ms, and the open path lost only the read of a section no 2.x file holds. Not a measurement.
- `cargo test --profile gate -p ein-cli -- --skip no_config_flag_moves_a_line_of_any_mode`: 407 tests, green;
  `lever_invariance`'s other five by name in it (`every_declared_lever_cell_moves_no_line_of_any_mode`, the
  describe route, the core exemption, W36, `--no-search`), `read_out_reference` and `config_reference` among them.
  The skipped one is the cell another worktree is fixing.
- `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test id_order_invariance`, once, on the
  code commit's tree: green, every corpus program permuted, and it wrote the golden above.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean. `python3 utils/check_hashmap_iteration.py`:
  ok. `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md
  examples/README.md stdlib/README.md utils/README.md c/README.md`: 0 findings, after this record too.
  `utils/check_test_scratch.py` over the tests' scratch directory: empty.
- Not run, as the stage was told: `./run_tests.sh`, the nightly, `utils/ci_container.sh`.

### Findings outside the stage's scope

Filed in [`findings.md`](findings.md#from-the-stages-of-tracks-s-and-r), open: **K-71**, eight `branching/` fixtures
named for the lookahead or the kill cache, whose headers describe them; **K-72**, the `.einb` `SOLUTIONS` section,
which no command writes and which carries the walk's no-good counters; **K-73**, `Hrules::candidates`' order
guarding against a write no pass makes. K-29 and K-30 are closed.
