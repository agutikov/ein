//! The `layer` event's arithmetic — M1d
//! [S1d.10.1](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md)
//! T1d.10.1.1.
//!
//! [`ein_infer::LayerCensus`] is an instrument, and an instrument that can
//! disagree with the engine it measures is worse than no instrument: a census
//! taken over the whole corpus is read as evidence, and nothing else would
//! notice a column that had drifted. So the two claims the census rests on are
//! tests.
//!
//! **The row adds up.** `joined = dropped_dead + dropped_nogood + candidates`
//! and `entered = alive + dead_pre + dead_post`, and the per-layer columns sum
//! to the whole-run [`MonotonicStats`] they were differenced out of. That last
//! one is what catches a counter bumped outside a layer, which is a real
//! possibility: the forced-positive cascade runs at the barrier, on the far
//! side of the row's close.
//!
//! **One of the generator's two filters is inert**, and it is inert
//! *structurally*, not on this corpus by luck — see
//! [`the_alive_arm_of_the_filter_never_fires`]. That is a finding rather than
//! a detail: it means the only thing that can shrink a layer is the clause
//! store, which is the whole of what the barren regime lacks.

use std::collections::BTreeMap;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::solve::{MonotonicStats, NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};

/// `(corpus entry, depth cap)` — one per regime the census names, and all
/// three cheap enough for the default gate.
///
/// | entry | regime | why this one |
/// |---|---|---|
/// | `zebra2.ein` | pruning | layer 1 kills 32 of 56, which is the regime every prune in the engine was measured on |
/// | `branching/07_lookahead_off.ein` | pruning, deep | the only cheap fixture that reaches layer 5 **and** whose clause store filters — 10 342 of 21 843 joined candidates dropped, so `dropped_nogood` is exercised rather than merely present |
/// | `branching/05_mini_zebra.ein` | barren | layer 1 enters three candidates and kills none, which is `zebra2-minus-15`'s shape at 1/1600th of its cost |
const CASES: [(&str, u32); 3] = [
    ("examples/zebra2.ein", 5),
    ("examples/branching/07_lookahead_off.ein", 5),
    ("examples/branching/05_mini_zebra.ein", 5),
];

/// One layer's row, as the event stream carries it.
type Row = BTreeMap<String, i64>;

fn census(rel: &str, max_set_size: u32) -> (Vec<Row>, MonotonicStats) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the fixture loads");
    let buf = Buffer::new();
    let mut events = Events::to(Box::new(buf.clone()), Level::Normal);
    let opts = SolveOptions {
        stop_after: None,
        max_set_size,
        on_budget: OnBudget::Verdict,
        ..SolveOptions::default()
    };
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .expect("the fixture solves");
    let rows = buf
        .to_string_lossy()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<serde_json::Value>(l).expect("one JSON object per line"))
        .filter(|v| v["e"] == "layer")
        .map(|v| {
            v.as_object()
                .expect("an object")
                .iter()
                .filter_map(|(k, v)| v.as_i64().map(|n| (k.clone(), n)))
                .collect()
        })
        .collect();
    (rows, solved.stats)
}

fn get(row: &Row, key: &str) -> i64 {
    *row.get(key)
        .unwrap_or_else(|| panic!("the `layer` event has no `{key}`: {row:?}"))
}

#[test]
fn every_row_adds_up() {
    for (rel, cap) in CASES {
        let (rows, _) = census(rel, cap);
        assert!(!rows.is_empty(), "{rel} entered no layer");
        for row in &rows {
            let l = get(row, "layer");
            assert_eq!(
                get(row, "joined"),
                get(row, "dropped_dead") + get(row, "dropped_nogood") + get(row, "candidates"),
                "{rel} layer {l}: the filter must partition the join — {row:?}"
            );
            assert_eq!(
                get(row, "entered"),
                get(row, "alive_enterings") + get(row, "dead_pre") + get(row, "dead_post"),
                "{rel} layer {l}: every entering is alive, dead-pre or dead-post — {row:?}"
            );
            assert!(
                get(row, "entered") <= get(row, "candidates"),
                "{rel} layer {l}: entered more than it was handed — {row:?}"
            );
            // A solved commitment is recorded and **not** expanded, so the
            // frontier is what is left of the alive enterings. Models are
            // deduped by `state_key` and so are *not* the difference.
            assert!(
                get(row, "next") <= get(row, "alive_enterings"),
                "{rel} layer {l}: the frontier outgrew the alive enterings — {row:?}"
            );
            assert!(
                get(row, "models") <= get(row, "alive_enterings") - get(row, "next"),
                "{rel} layer {l}: more distinct models than enterings that reached one — {row:?}"
            );
        }
        // The retain between two layers only ever removes.
        for pair in rows.windows(2) {
            assert!(
                get(&pair[1], "frontier") <= get(&pair[0], "next"),
                "{rel}: layer {} joined over more sets than layer {} handed on",
                get(&pair[1], "layer"),
                get(&pair[0], "layer")
            );
        }
    }
}

/// Each per-layer column is a difference of two whole-run counters, so the
/// columns have to sum back to them. What this catches is a counter bumped
/// where no layer is open — `forced_positives` genuinely is one, which is why
/// it is not a census column at all.
#[test]
fn the_columns_sum_to_the_run() {
    for (rel, cap) in CASES {
        let (rows, stats) = census(rel, cap);
        let sum = |key: &str| rows.iter().map(|r| get(r, key)).sum::<i64>();
        let b = &stats.base;
        assert_eq!(sum("entered"), b.enterings_total as i64, "{rel}: enterings");
        assert_eq!(
            sum("alive_enterings"),
            b.enterings_alive as i64,
            "{rel}: alive enterings"
        );
        assert_eq!(
            sum("dead_pre"),
            b.enterings_dead_pre as i64,
            "{rel}: dead-pre"
        );
        assert_eq!(
            sum("dead_post"),
            b.enterings_dead_post as i64,
            "{rel}: dead-post"
        );
        assert_eq!(
            sum("nogoods_emitted"),
            b.nogoods_emitted as i64,
            "{rel}: clauses emitted"
        );
        assert_eq!(
            sum("nogoods_subsumed"),
            b.nogoods_subsumed as i64,
            "{rel}: clauses subsumed"
        );
        assert_eq!(rows.len() as u64, b.layers_explored, "{rel}: layers");
        // `models` is a *layer* total and `solution_nodes` a run total, and
        // they differ by the node the loop records when `alive` empties —
        // which happens at the barrier, after the last row has closed.
        assert!(
            sum("models") <= stats.solution_nodes as i64,
            "{rel}: more models in the layers than in the run"
        );
    }
}

/// **The `alive` arm of [`ein_infer::filter_reason`] cannot fire from the
/// solve loop, and that is structural.**
///
/// `phase2` ends a layer by recomputing `alive`, promoting any forced
/// positives, and then retaining only the commitments still entirely within
/// it; `a_prev` is what survives that. The next layer's join emits
/// `prefix + (s_last, t_last)` out of two surviving sets, so every element of
/// every candidate is in `alive` — and nothing touches `alive` between the
/// retain and the join. So `dropped_dead` is 0 at every layer above the first,
/// and layer 1 is not filtered at all.
///
/// The consequence is the reason this is a test rather than a remark: **the
/// only thing that can shrink a layer is the clause store.** The predicate's
/// own doc says the `alive` check covers "the single-element negatives the
/// singleton-death writeback wrote since `a_prev` was computed" — it does, but
/// the retain got there first, and a search whose layers go barren has neither.
///
/// What would break it is a change that lets `alive` shrink *during* a layer,
/// or a generator run against an `alive` newer than its frontier. Both are
/// legitimate designs; neither is this one, and this is where they announce
/// themselves.
#[test]
fn the_alive_arm_of_the_filter_never_fires() {
    for (rel, cap) in CASES {
        let (rows, _) = census(rel, cap);
        for row in &rows {
            assert_eq!(
                get(row, "dropped_dead"),
                0,
                "{rel} layer {}: the retain at the previous barrier should have \
                 removed every candidate with a dead element — {row:?}",
                get(row, "layer")
            );
        }
    }
}
