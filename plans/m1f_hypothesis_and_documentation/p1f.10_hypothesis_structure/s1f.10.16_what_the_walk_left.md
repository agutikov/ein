# S1f.10.16 — What the walk left in the generator

**Phase:** [P1f.10](README.md)
**Status:** planned 2026-11-15, at the re-plan. Not started.
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
the lookahead nor its kill cache, and sorts and dedups its list by rendered text before any route reads it. The
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
