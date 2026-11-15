//! The price list's reading, held on a few programs: what
//! `examples/shared/prices.rs` reads of a route is the route's own work,
//! and what it says of the structure is what the structure says.
//!
//! - **A watched run is the plain run.** Each question is asked twice, once
//!   with every node handed to a reader and a core taken at every death and
//!   once with neither: one answer, one count of saturations, one count of
//!   facts beyond root, and the nodes the reader was handed are that count
//!   and those facts. So reading a route's nodes changes no route.
//! - **The split is a partition of the work**: its parts sum to the cell's
//!   saturations and to its facts.
//! - **A way is applied on a monotone program alone**, removes no more than
//!   the cell holds, and asking the pair layer node by node never asks for
//!   more than the layer has.
//! - **A stored negation is upward**: on a monotone program no alive node
//!   holds a candidate that an alive node visited before it and inside it
//!   denies, which is what the `denials` way rests on.
//! - **The controls.** A conflict of three is no pair's: on
//!   `lattice/02_genuine_3set_death.ein` no way removes a node under any
//!   question, and its one death beyond root lies inside one edge of the
//!   coupling. And a dead node that holds two members of one group is one
//!   the theory's pairs remove.

use std::path::PathBuf;

use ein_corpus::repo_root;

#[allow(dead_code)]
#[path = "../examples/shared/prices.rs"]
mod prices;

use prices::{Cell, PARTS, Read};

/// The programs read, each with the questions asked of it: every question
/// of the small ones, and of the two larger the questions a tree decides.
const PROGRAMS: [(&str, &[&str]); 7] = [
    ("examples/lattice/02_genuine_3set_death.ein", &[]),
    ("examples/branching/06_lookahead_on.ein", &[]),
    ("examples/lattice/05_owed_seats_bounded.ein", &[]),
    ("examples/lattice/synth/one_model_per_component.ein", &[]),
    // Flagged by the static check.
    ("examples/lattice/04_obligation_trigger_hole.ein", &[]),
    ("examples/zebra2.ein", &["single", "n1", "maximal"]),
    (
        "examples/beyond_arith/send_more_money.ein",
        &["single", "n1"],
    ),
];

struct Program {
    rel: &'static str,
    read: Read,
    cells: Vec<Cell>,
}

fn read(unit: &(&'static str, &[&str])) -> Program {
    let (rel, only) = *unit;
    let path: PathBuf = repo_root().join(rel);
    let read = Read::of(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut denials = prices::Denials::default();
    let cells = prices::questions()
        .into_iter()
        .filter(|(key, _, _)| only.is_empty() || only.contains(key))
        .map(|(key, flags, mode)| prices::cell(&path, &read, key, flags, mode, &mut denials))
        .collect();
    Program { rel, read, cells }
}

fn programs() -> Vec<Program> {
    ein_corpus::pool::map(
        "price_list",
        &PROGRAMS,
        |u| (u.0.to_string(), String::new()),
        read,
    )
}

#[test]
fn a_routes_nodes_are_read_and_no_route_is_changed() {
    let mut failures: Vec<String> = Vec::new();
    let (mut cells, mut flagged) = (0, 0);
    for p in programs() {
        if !p.read.layer_same {
            failures.push(format!(
                "{}: the layer read node by node is not the owner's",
                p.rel
            ));
        }
        flagged += usize::from(!p.read.monotone());
        for c in &p.cells {
            let at = format!("{} {}", p.rel, c.flags);
            let run = match &c.run {
                Ok(run) => run,
                Err(e) => {
                    failures.push(format!("{at}: {e}"));
                    continue;
                }
            };
            cells += 1;
            // The watched run is the plain one.
            if !c.same_work {
                failures.push(format!(
                    "{at}: the watched run is not the plain run's answer and work"
                ));
            }
            if c.unmapped != 0 {
                failures.push(format!("{at}: {} texts no candidate has", c.unmapped));
            }
            // The split is a partition of the work.
            let split = c.priced.split;
            let sum = |u: usize| split.iter().map(|x| x[u]).sum::<u64>();
            if sum(0) != run.saturations || sum(1) != run.facts {
                failures.push(format!(
                    "{at}: the split sums to {} saturations and {} facts, the cell is {} and {}",
                    sum(0),
                    sum(1),
                    run.saturations,
                    run.facts
                ));
            }
            // What a node under an alive node denies is not above it.
            if c.priced.detail.contains_key("alive_and_denied_under") {
                failures.push(format!(
                    "{at}: an alive node holds a candidate a node under it denies"
                ));
            }
            if c.priced.parts.len() as u64 != run.saturations {
                failures.push(format!("{at}: a node has no part"));
            }
            let layer = p.read.layer.len() as u64;
            for (name, d) in &c.priced.ways {
                if d.applies && !p.read.monotone() {
                    failures.push(format!("{at}: `{name}` is applied on a flagged program"));
                }
                if !d.applies && (d.added != [0, 0] || d.removed != [0, 0]) {
                    failures.push(format!("{at}: `{name}` is not applied and has a price"));
                }
                if d.removed[0] > run.saturations || d.removed[1] > run.facts {
                    failures.push(format!("{at}: `{name}` removes more than the cell holds"));
                }
                if matches!(*name, "layer-by-need" | "death-pairs") && d.added[0] > layer {
                    failures.push(format!(
                        "{at}: `{name}` asks for {} of a layer of {layer} nodes",
                        d.added[0]
                    ));
                }
            }
            // Two members of one group under a dead node: the theory's
            // pairs remove it.
            let theory = c.priced.ways.iter().find(|(w, _)| *w == "theory-shared");
            if let Some((_, d)) = theory
                && d.applies
                && d.removed[0] < c.priced.part("dead_theory")[0]
            {
                failures.push(format!(
                    "{at}: {} dead nodes hold a pair of the theory and its pairs remove {}",
                    c.priced.part("dead_theory")[0],
                    d.removed[0]
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(cells >= 30, "{cells} cells read");
    assert_eq!(flagged, 1, "one flagged program is read");
}

#[test]
fn a_conflict_of_three_is_no_pairs_to_remove() {
    let unit = PROGRAMS[0];
    let p = read(&unit);
    assert!(p.read.monotone());
    for c in &p.cells {
        let run = c.run.as_ref().expect("a run");
        let at = format!("{} {}", p.rel, c.flags);
        for part in ["dead_theory", "dead_puzzle", "dead_single"] {
            assert_eq!(c.priced.part(part), [0, 0], "{at}: {part}");
        }
        for (name, d) in &c.priced.ways {
            assert!(d.applies, "{at}: `{name}` is applied");
            assert_eq!(d.removed, [0, 0], "{at}: `{name}` removes nothing");
        }
        // Every node is in a part the table names.
        let named: u64 = PARTS.iter().map(|part| c.priced.part(part)[0]).sum();
        assert_eq!(named, run.saturations, "{at}");
    }
    // Its describe meets the triple, dead, inside the coupling's one edge.
    let describe = &p.cells[0];
    assert_eq!(describe.key, "describe");
    assert_eq!(describe.priced.part("dead_edge")[0], 1);
    assert_eq!(describe.priced.part("dead_other")[0], 0);
}
