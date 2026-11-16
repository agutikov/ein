//! **The coupling read from the whole node alone**
//! (`ein_infer::structure::NodeCoupling`), which is what the obligation tree
//! reads to choose the instance a state branches on: held to its controls,
//! to the full coupling, and to the order a program states its facts in.
//!
//! - **The controls.** SEND + MORE = MONEY's edges are its four columns,
//!   each with the letters its carry comes from, and `{D, E}`; the triple of
//!   `lattice/02_genuine_3set_death.ein` is one edge of its three
//!   candidates. Each for one saturation.
//! - **The written order.** `send_more_money.ein` and
//!   `send_more_money_written.ein` differ in the order of eight facts, and
//!   the two readings are one value, field for field.
//! - **No config flag.** A route orders its branches by this reading, and no
//!   `(config …)` flag may move a route's work: the reading of a program
//!   with each flag set either way is the reading of the program.
//! - **Against the full coupling** (`ein_infer::structure::coupling`, both
//!   pair layers and the same node), on every corpus program that searches
//!   and that both read whole: the candidates of every edge read here lie
//!   inside the candidates of **one component** of the full coupling, and
//!   inside one of its edges wherever the full reading has an edge that
//!   large. What is finer and what is coarser is counted and printed.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test node_coupling -- --nocapture
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::structure::{self, CouplingOptions, NodeCoupling, Options, Question};
use ein_ir::{Ast, load, parse};

#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn rel_of(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

/// The reading of the program at `rel`, and its candidates' texts.
fn read(rel: &str) -> (NodeCoupling, Vec<String>) {
    let path = repo_root().join(rel);
    let (mut kb, mut terms, ast) = loaded(&path).unwrap_or_else(|| panic!("{rel} loads"));
    let c = structure::node_coupling(&mut kb, &mut terms, &ast, &Options::default())
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    let texts = c.candidates.iter().map(|c| c.text.clone()).collect();
    (c, texts)
}

/// Everything a reading says, its candidates as text.
fn said(c: &NodeCoupling) -> String {
    let texts: Vec<&str> = c.candidates.iter().map(|c| c.text.as_str()).collect();
    format!(
        "{texts:?} {:?} {} {:?} {:?} {:?} {:?} {:?} {} {:?} {:?} {:?}",
        c.questions,
        c.owed,
        c.question_of,
        c.edges,
        c.one_step,
        c.pairs,
        c.components,
        c.dead,
        c.node,
        c.node_stopped,
        c.cut
    )
}

/// An edge as the letters its questions ask the digit of: `(val D 0)` …
/// `(val D 9)` is `D`.
fn letters(c: &NodeCoupling, texts: &[String], edge: &[u32]) -> String {
    let mut out: Vec<String> = edge
        .iter()
        .map(|&q| {
            let names: BTreeSet<&str> = c.questions[q as usize]
                .iter()
                .map(|&b| texts[b as usize].split(' ').nth(1).expect("a letter"))
                .collect();
            assert_eq!(names.len(), 1, "a question of one letter");
            names.into_iter().next().expect("one").to_string()
        })
        .collect();
    out.sort();
    out.join("")
}

const SEND: &str = "examples/beyond_arith/send_more_money.ein";
const SEND_WRITTEN: &str = "examples/beyond_arith/send_more_money_written.ein";

/// **SEND's columns, from one node**: the eight letters are the questions
/// root owes, and the edges are `{D, E}`, the units column `{D, E, Y}`, and
/// each column above with the letters its carry comes from. No digit's
/// question, and no pair of letters for sharing a digit.
#[test]
fn sends_edges_are_its_columns() {
    let (c, texts) = read(SEND);
    assert!(c.complete(), "{:?} {:?}", c.node_stopped, c.cut);
    assert_eq!(c.n(), 80);
    assert_eq!(c.owed, 8, "the eight letters owe a digit");
    assert_eq!(
        c.questions.len(),
        8,
        "every candidate is some letter's digit"
    );
    let node = c.node.expect("read");
    assert_eq!(node.saturations, 1);
    assert!(c.dead, "the whole node holds every letter's every digit");
    let edges: Vec<String> = c.edges.iter().map(|e| letters(&c, &texts, e)).collect();
    let mut want = vec!["DE", "DEY", "DENR", "DENOR", "DEMNORS"];
    want.sort();
    let mut got = edges.clone();
    got.sort();
    assert_eq!(got, want, "the edges");
    let one_step: BTreeSet<String> = c.one_step.iter().map(|e| letters(&c, &texts, e)).collect();
    assert_eq!(
        one_step,
        BTreeSet::from(["DE".to_string(), "DEY".to_string()]),
        "one application back"
    );
    // Different letters, different digits: a pair, and no edge.
    assert!(!c.pairs.is_empty());
    assert_eq!(c.components.len(), 1);
    eprintln!(
        "{SEND}: {} questions, edges {edges:?}, {} pairs, the node {} facts and {} applications",
        c.questions.len(),
        c.pairs.len(),
        node.facts,
        node.applications
    );
}

/// **The reading does not read the order a program states its facts in**:
/// the two SENDs are one value.
#[test]
fn the_reading_is_the_same_whatever_order_the_letters_are_declared_in() {
    let (a, ta) = read(SEND);
    let (b, tb) = read(SEND_WRITTEN);
    assert_eq!(ta, tb, "one universe");
    assert_eq!(said(&a), said(&b));
}

/// **No `(config …)` flag moves the reading.** The obligation tree orders
/// its branches by it, and a flag that moved it would move a run's work and
/// the read-out's `route` row with it. Each flag a program can set, either
/// way, in a block appended to the program as written (the loader keeps the
/// last block whole): the reading is the program's own, field for field.
#[test]
fn no_config_flag_moves_the_reading() {
    for rel in [
        SEND,
        "examples/zebra2.ein",
        "examples/zebra.ein",
        "examples/beyond_arith/cryptarithm.ein",
        "examples/lattice/02_genuine_3set_death.ein",
        "tests/region/10_denied_extensions.ein",
    ] {
        let path = repo_root().join(rel);
        let text = std::fs::read_to_string(&path).expect("reads");
        let of = |text: &str| -> String {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let forms = parse(&mut ast, text, path.to_str()).expect("parses");
            let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
            let c = structure::node_coupling(&mut kb, &mut terms, &ast, &Options::default())
                .unwrap_or_else(|e| panic!("{rel}: {e}"));
            said(&c)
        };
        let as_written = of(&text);
        for (flag, _) in ein_core::config::FLAGS {
            for value in ["true", "false"] {
                let flagged = format!("{text}\n(config :{flag} {value})\n");
                assert_eq!(of(&flagged), as_written, "{rel} under :{flag} {value}");
            }
        }
    }
}

/// **The triple**: three candidates no two of which conflict, one edge of
/// the three, with no obligation and so no question but the candidates.
#[test]
fn a_conflict_of_three_is_one_edge() {
    let (c, _) = read("examples/lattice/02_genuine_3set_death.ein");
    assert!(c.complete());
    assert_eq!((c.n(), c.owed), (3, 0));
    assert_eq!(c.edges, vec![vec![0, 1, 2]]);
    assert!(c.pairs.is_empty());
    assert_eq!(c.components, vec![vec![0, 1, 2]]);
    assert_eq!(c.node.expect("read").saturations, 1);
}

/// Every corpus file and register witness, once, but the `slow` entries
/// that are not named here.
fn files() -> Vec<PathBuf> {
    let keep = [
        "examples/zebra.ein",
        "examples/zebra2.ein",
        SEND,
        SEND_WRITTEN,
    ];
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !keep.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| !slow.contains(f));
    files
}

#[derive(Default)]
struct Tally {
    programs: usize,
    edges: usize,
    /// Edges whose candidates lie inside one edge of the full reading.
    inside_an_edge: usize,
    /// Edges that lie inside one component of the full reading and inside
    /// no edge of it.
    inside_a_component: Vec<String>,
    /// Edges inside no component of the full reading.
    outside: Vec<String>,
    /// The full reading's edges of a conflict or a derivation of three
    /// questions or more, and how many of them hold the candidates of no
    /// edge read here.
    full_of_three: usize,
    full_unseen: Vec<String>,
    saturations: u64,
}

/// One program: the reading from the node alone against the full one.
fn against_the_full_reading(path: &Path) -> Tally {
    let mut t = Tally::default();
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return t;
    };
    if !searching::searches(&ast, &mut terms, &mut kb) {
        return t;
    }
    let rel = rel_of(path);
    let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
    let Ok(node) = structure::node_coupling(&mut kb, &mut terms, &ast, &Options::default()) else {
        return t;
    };
    let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
    let Ok((cover, full)) =
        structure::coupling(&mut kb, &mut terms, &ast, &CouplingOptions::default())
    else {
        return t;
    };
    if !full.monotone || !full.complete() || !node.complete() {
        return t;
    }
    t.programs = 1;
    t.saturations = node.node.map_or(0, |w| w.saturations);
    let members = |q: u32| -> Vec<u32> {
        match full.questions[q as usize] {
            Question::Group(g) => cover.groups[g as usize].clone(),
            Question::Alone(b) => vec![b],
        }
    };
    let full_edges: Vec<BTreeSet<u32>> = full
        .edges
        .iter()
        .filter(|e| e.is_tie())
        .map(|e| e.questions.iter().flat_map(|&q| members(q)).collect())
        .collect();
    // The full reading is of the candidates alive alone; one dead alone is
    // in no question of it that an edge holds.
    let alive: BTreeSet<u32> = full.components.iter().flatten().copied().collect();
    let node_edges: Vec<BTreeSet<u32>> = node
        .edges
        .iter()
        .map(|e| {
            node.members(e)
                .into_iter()
                .filter(|b| alive.contains(b))
                .collect()
        })
        .collect();
    for (k, e) in node_edges.iter().enumerate() {
        t.edges += 1;
        if full_edges.iter().any(|f| e.is_subset(f)) {
            t.inside_an_edge += 1;
        } else if full
            .components
            .iter()
            .any(|c| e.iter().all(|b| c.binary_search(b).is_ok()))
        {
            t.inside_a_component.push(format!("{rel} edge {k}"));
        } else {
            t.outside.push(format!("{rel} edge {k}"));
        }
    }
    for (k, e) in full.edges.iter().enumerate() {
        if !e.is_tie() || e.arity() < 3 || e.node.all() == 0 {
            continue;
        }
        t.full_of_three += 1;
        // Seen: some edge read here meets every question of it.
        let seen = node_edges.iter().any(|n| {
            e.questions
                .iter()
                .all(|&q| members(q).iter().any(|b| n.contains(b)))
        });
        if !seen {
            t.full_unseen.push(format!("{rel} edge {k}"));
        }
    }
    t
}

/// **The node alone against the full coupling**, on every corpus program
/// that searches and both read whole: an edge read from the node alone lies
/// inside one component of the full coupling, so it never ties what the
/// full reading holds apart.
#[test]
fn an_edge_of_the_node_alone_lies_inside_the_full_coupling() {
    let files = files();
    let results = ein_corpus::pool::map(
        "node_coupling::against_the_full_reading",
        &files,
        |p| (rel_of(p), String::new()),
        |p| against_the_full_reading(p),
    );
    let mut all = Tally::default();
    for t in results {
        all.programs += t.programs;
        all.edges += t.edges;
        all.inside_an_edge += t.inside_an_edge;
        all.inside_a_component.extend(t.inside_a_component);
        all.outside.extend(t.outside);
        all.full_of_three += t.full_of_three;
        all.full_unseen.extend(t.full_unseen);
        all.saturations += t.saturations;
    }
    eprintln!(
        "{} programs both read whole, {} saturations for the nodes alone: {} edges from the node \
         alone, {} inside one edge of the full reading, {} inside one component and no edge \
         ({:?}), {} inside no component ({:?}); of the full reading's {} edges of three questions \
         or more that the node gave, {} meet no edge read here in every question ({:?})",
        all.programs,
        all.saturations,
        all.edges,
        all.inside_an_edge,
        all.inside_a_component.len(),
        all.inside_a_component,
        all.outside.len(),
        all.outside,
        all.full_of_three,
        all.full_unseen.len(),
        all.full_unseen,
    );
    assert!(all.programs >= 100, "{} programs", all.programs);
    assert!(
        all.outside.is_empty(),
        "an edge of the node alone ties what the full coupling holds apart: {:?}",
        all.outside
    );
}
