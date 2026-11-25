//! The orders of the obligation tree, measured: `price_list --orders`.
//!
//! Each order of [`orders`] is a value of [`ein_infer::order::Order`], which
//! the tree takes for the calling thread while a question is asked
//! ([`ein_infer::order::with`]), but `routes`, which is the order the
//! routes take, `Order::of` the question, with no `order::with`. For every
//! corpus program that searches and each question the tree answers
//! ([`QUESTIONS`]: `--single`, `-n 1`, each line whole, and each line under
//! `-n 1` and `--single`), the run's saturations and facts beyond root, with
//! **the cost of the structure the order reads in them**: the whole node is
//! charged to the run by the tree itself, and the full structure, read
//! before the run, is added here ([`Cell::added`]).
//!
//! `--orders` writes one file an order, `orders/<name>.jsonl`, a line a
//! program, so that several processes can share the orders between them;
//! `--orders-report` reads every file of that directory and prints the
//! tables, each order against `today` by the rule the price list states:
//! *go* where the corpus total is lower and no program's cell is higher by
//! more than one tenth, in saturations and in facts. It reads the rule two
//! ways. The first table of a question takes the total over the cells not
//! stopped both ways, a cell a budget stops under both orders having spent
//! that budget either way, and a cell an order stops where `today` finishes
//! sinks it. The second, *by the price list's defaults*, is the price
//! list's own reading: every cell in the total and in the tenth, a stopped
//! cell with what it had spent.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ein_corpus::repo_root;
use ein_infer::explore::{Mode, Stopped};
use ein_infer::order::{self, Blame, Choice, Given, Order, Queue, Structure};
use serde_json::{Value, json};

use super::prices::{self, Read};

/// The questions the tree answers: of all of `R`, `--single` and `-n 1`;
/// each line whole; and each line under `-n 1` and `--single`.
const QUESTIONS: [&str; 8] = [
    "single",
    "n1",
    "minimal",
    "maximal",
    "minimal-n1",
    "minimal-single",
    "maximal-n1",
    "maximal-single",
];

/// [`QUESTIONS`] as modes.
fn questions() -> Vec<(&'static str, Mode)> {
    use ein_infer::explore::{Ask, Which};
    let m = |which, ask| Mode { which, ask };
    vec![
        ("single", m(Which::All, Ask::Single)),
        ("n1", m(Which::All, Ask::Find(1))),
        ("minimal", m(Which::Minimal, Ask::Describe)),
        ("maximal", m(Which::Maximal, Ask::Describe)),
        ("minimal-n1", m(Which::Minimal, Ask::Find(1))),
        ("minimal-single", m(Which::Minimal, Ask::Single)),
        ("maximal-n1", m(Which::Maximal, Ask::Find(1))),
        ("maximal-single", m(Which::Maximal, Ask::Single)),
    ]
}

/// The instances, by tag.
pub const INSTANCES: [(&str, &str); 7] = [
    ("I-Z1", "examples/zebra.ein"),
    ("I-Z2", "examples/zebra2.ein"),
    ("I-Z2M", "examples/zebra2-minus-15.ein"),
    ("I-B06", "examples/branching/06_lookahead_on.ein"),
    ("I-L02", "examples/lattice/02_genuine_3set_death.ein"),
    ("I-SEND", "examples/beyond_arith/send_more_money.ein"),
    (
        "I-SENDW",
        "examples/beyond_arith/send_more_money_written.ein",
    ),
];

/// What an order reads beyond the state and the memory of what is dead.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reads {
    Nothing,
    /// The whole node, by the tree, counted in the run.
    Node,
    /// The full structure, read before the run and added to it.
    Full,
}

struct Named {
    name: &'static str,
    what: &'static str,
    reads: Reads,
    /// `None`: the order the routes take ([`Order::of`] the question), with
    /// no [`order::with`].
    order: Option<Order>,
}

fn order(choice: Choice, queue: Queue, structure: Structure, restarts: bool) -> Order {
    Order {
        choice,
        queue,
        structure,
        restarts,
    }
}

/// Every order measured. `today` is the tree as it stood before any order
/// was derived, and what each other is held against.
fn orders() -> Vec<Named> {
    let node = |after, one_step| Structure::Node {
        after,
        one_step,
        share: 0,
    };
    let within = |after, share| Structure::Node {
        after,
        one_step: false,
        share,
    };
    let written = Choice::Written { denials: true };
    let named = |name, what, reads, order| Named {
        name,
        what,
        reads,
        order: Some(order),
    };
    use Queue::{BySize, Deepest};
    use Reads::{Full, Node, Nothing};
    vec![
        Named {
            name: "routes",
            what: "the order the routes take, `Order::of` the question, with no `order::with`: \
                   coupled/20, and under -n N written+coupled/20",
            reads: Node,
            order: None,
        },
        named(
            "today",
            "fewest alternatives not known dead, the first reported among equals; by size",
            Nothing,
            order(written, BySize, Structure::None, false),
        ),
        named(
            "deaths-count",
            "today's, the count not reading what a state denies",
            Nothing,
            order(
                Choice::Written { denials: false },
                BySize,
                Structure::None,
                false,
            ),
        ),
        named(
            "reversed",
            "today's, the last reported among equals",
            Nothing,
            order(Choice::Reversed, BySize, Structure::None, false),
        ),
        named(
            "text",
            "fewest not known dead, then rendered text",
            Nothing,
            order(Choice::Text, BySize, Structure::None, false),
        ),
        named(
            "node@0",
            "fewest not known dead, then closest to closing an edge of the whole node, then text; \
             the node read at the first tied choice",
            Node,
            order(Choice::OpenCoupled, BySize, node(0, false), false),
        ),
        named(
            "node@10",
            "the same, the node read at the first tied choice once the run has saturated 10 \
             nodes, the tree then started again",
            Node,
            order(Choice::OpenCoupled, BySize, node(10, false), false),
        ),
        named(
            "node@32",
            "the same, once it has saturated 32",
            Node,
            order(Choice::OpenCoupled, BySize, node(32, false), false),
        ),
        named(
            "node@100",
            "the same, once it has saturated 100",
            Node,
            order(Choice::OpenCoupled, BySize, node(100, false), false),
        ),
        named(
            "node-first@0",
            "closest to closing an edge first, then fewest not known dead, then text",
            Node,
            order(Choice::CoupledOpen, BySize, node(0, false), false),
        ),
        named(
            "node-first@10",
            "the same, the node read once the run has saturated 10 nodes",
            Node,
            order(Choice::CoupledOpen, BySize, node(10, false), false),
        ),
        named(
            "node-first@32",
            "the same, the node read once the run has saturated 32 nodes",
            Node,
            order(Choice::CoupledOpen, BySize, node(32, false), false),
        ),
        named(
            "open-first/20",
            "node@0's keys, the node read at the first tied choice once the run has saturated \
             20 nodes and within a twentieth of the facts it has derived, tried again at four \
             times the work where it needs more; the tree then started again, every choice \
             made afresh",
            Node,
            order(Choice::OpenCoupled, BySize, within(20, 20), false),
        ),
        named(
            "coupled/20",
            "fewest not known dead, then text; the node read as open-first/20 reads it; from \
             then on closest to closing an edge, then fewest not known dead, then text, a \
             state that had chosen choosing again by the counts it had, and the tree started \
             again where one chooses otherwise",
            Node,
            order(
                Choice::Coupled { written: false },
                BySize,
                within(20, 20),
                false,
            ),
        ),
        named(
            "coupled/10",
            "the same within a tenth, once it has saturated 10",
            Node,
            order(
                Choice::Coupled { written: false },
                BySize,
                within(10, 10),
                false,
            ),
        ),
        named(
            "coupled/40",
            "the same within a fortieth, once it has saturated 40",
            Node,
            order(
                Choice::Coupled { written: false },
                BySize,
                within(40, 40),
                false,
            ),
        ),
        named(
            "written+coupled/20",
            "coupled/20 with the first reported among equals where it has text",
            Node,
            order(
                Choice::Coupled { written: true },
                BySize,
                within(20, 20),
                false,
            ),
        ),
        named(
            "written+coupled/10",
            "the same within a tenth, once it has saturated 10",
            Node,
            order(
                Choice::Coupled { written: true },
                BySize,
                within(10, 10),
                false,
            ),
        ),
        named(
            "written+coupled/40",
            "the same within a fortieth, once it has saturated 40",
            Node,
            order(
                Choice::Coupled { written: true },
                BySize,
                within(40, 40),
                false,
            ),
        ),
        named(
            "coupled/20/deepest",
            "coupled/20's choice; the largest state first",
            Node,
            order(
                Choice::Coupled { written: false },
                Deepest,
                within(20, 20),
                false,
            ),
        ),
        named(
            "written+coupled/20/deepest",
            "written+coupled/20's choice; the largest state first",
            Node,
            order(
                Choice::Coupled { written: true },
                Deepest,
                within(20, 20),
                false,
            ),
        ),
        named(
            "one-step@32",
            "node@32 with the edges one application back",
            Node,
            order(Choice::OpenCoupled, BySize, node(32, true), false),
        ),
        named(
            "full",
            "fewest not known dead, then closest to closing an edge of the full coupling, then \
             text; both pair layers and the node paid before the run",
            Full,
            order(Choice::OpenCoupled, BySize, Structure::None, false),
        ),
        named(
            "learned-branch",
            "fewest not known dead over the deaths of the instance's own alternatives, then text",
            Nothing,
            order(
                Choice::Learned(Blame::Branch),
                BySize,
                Structure::None,
                false,
            ),
        ),
        named(
            "learned-path",
            "the same, a death counted against every instance the dying state had paid",
            Nothing,
            order(Choice::Learned(Blame::Path), BySize, Structure::None, false),
        ),
        named(
            "learned-branch/restarts",
            "learned-branch, the tree started again each time its saturations double from 32",
            Nothing,
            order(
                Choice::Learned(Blame::Branch),
                BySize,
                Structure::None,
                true,
            ),
        ),
        named(
            "today/deepest",
            "today's choice; the largest state first",
            Nothing,
            order(written, Deepest, Structure::None, false),
        ),
        named(
            "text/deepest",
            "text's choice; the largest state first",
            Nothing,
            order(Choice::Text, Deepest, Structure::None, false),
        ),
        named(
            "node@0/deepest",
            "node@0's choice; the largest state first",
            Node,
            order(Choice::OpenCoupled, Deepest, node(0, false), false),
        ),
        named(
            "node@32/deepest",
            "node@32's choice; the largest state first",
            Node,
            order(Choice::OpenCoupled, Deepest, node(32, false), false),
        ),
        named(
            "learned-branch/deepest",
            "learned-branch's choice; the largest state first",
            Nothing,
            order(
                Choice::Learned(Blame::Branch),
                Deepest,
                Structure::None,
                false,
            ),
        ),
        named(
            "learned-path/deepest",
            "learned-path's choice; the largest state first",
            Nothing,
            order(
                Choice::Learned(Blame::Path),
                Deepest,
                Structure::None,
                false,
            ),
        ),
        named(
            "learned-branch/deepest/restarts",
            "the same as learned-branch/deepest, started again as its saturations double from 32",
            Nothing,
            order(
                Choice::Learned(Blame::Branch),
                Deepest,
                Structure::None,
                true,
            ),
        ),
        named(
            "learned-path/deepest/restarts",
            "the same as learned-path/deepest, started again as its saturations double from 32",
            Nothing,
            order(Choice::Learned(Blame::Path), Deepest, Structure::None, true),
        ),
    ]
}

/// The full coupling's edges over candidates, and what reading it cost:
/// root's layer, the small KB's and the node.
fn full(r: &Read) -> Option<(Given, [u64; 2])> {
    let (cover, c) = (r.cover.as_ref()?, r.coupling.as_ref()?);
    let questions: Vec<Vec<u32>> = c
        .questions
        .iter()
        .map(|q| match q {
            ein_infer::structure::Question::Group(g) => cover.groups[*g as usize].clone(),
            ein_infer::structure::Question::Alone(b) => vec![*b],
        })
        .collect();
    let edges: Vec<Vec<u32>> = c
        .edges
        .iter()
        .filter(|e| e.is_tie())
        .map(|e| e.questions.clone())
        .collect();
    let (layer, small, node) = (r.layer_work(), r.small_work(), r.node_work());
    let cost = [
        layer[0][0] + layer[1][0] + small[0] + node[0],
        layer[0][1] + layer[1][1] + small[1] + node[1],
    ];
    Some((Given { questions, edges }, cost))
}

fn stopped(s: &Option<Stopped>) -> Value {
    match s {
        None => Value::Null,
        Some(Stopped::Budget { .. }) => json!("budget"),
        Some(Stopped::Limit { .. }) => json!("limit"),
        Some(Stopped::Deadline) => json!("deadline"),
        Some(Stopped::NoSearch) => json!("no-search"),
        Some(Stopped::Steps { .. }) => json!("steps"),
    }
}

fn file_of(name: &str) -> String {
    name.replace('/', "_").replace('@', "-at-") + ".jsonl"
}

/// `--orders`: ask the four questions of every program under each order
/// named by `--only` (all of them without it).
pub fn run(args: Vec<String>) {
    let mut out = repo_root().join("build/structure");
    let mut only: Option<Vec<String>> = None;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = args.into_iter();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            "--only" => {
                only = Some(
                    args.next()
                        .expect("--only NAME,…")
                        .split(',')
                        .map(str::to_string)
                        .collect(),
                )
            }
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
        for row in ein_corpus::regressions().case {
            files.push(repo_root().join(&row.witness));
        }
        files.sort();
        files.dedup();
    }
    let files: Vec<(PathBuf, String)> = files
        .into_iter()
        .map(|path| {
            let path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()
                    .expect("a working directory")
                    .join(path)
            };
            let rel = path
                .strip_prefix(repo_root())
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            (path, rel)
        })
        .filter(|(path, _)| super::searches(path))
        .collect();
    let at = out.join("orders");
    std::fs::create_dir_all(&at).expect("the orders directory");
    let qs = questions();
    for named in orders() {
        if only
            .as_ref()
            .is_some_and(|o| !o.iter().any(|n| n == named.name))
        {
            continue;
        }
        let mut lines = String::new();
        for (path, rel) in &files {
            let mut order = named.order.clone();
            let mut added = [0u64; 2];
            if let (Reads::Full, Some(order)) = (named.reads, order.as_mut()) {
                // The full structure is read of a monotone program alone, as
                // every way of spending it is.
                match Read::of(path) {
                    Ok(r) if r.monotone() => {
                        if let Some((given, cost)) = full(&r) {
                            order.structure = Structure::Given(Arc::new(given));
                            added = cost;
                        }
                    }
                    Ok(_) => {}
                    Err(e) => eprintln!("{rel}: {e}"),
                }
            }
            let mut cells: BTreeMap<&str, Value> = BTreeMap::new();
            for (key, mode) in &qs {
                let asked = match &order {
                    Some(order) => {
                        order::with(order.clone(), || prices::ask(path, key, *mode, None))
                    }
                    None => prices::ask(path, key, *mode, None),
                };
                let cell = match asked {
                    Ok((run, _, _)) => json!({
                        "saturations": run.saturations,
                        "facts": run.facts,
                        "added": added,
                        "route": format!("{:?}", run.route),
                        "exhausted": run.exhausted,
                        "verdict": run.verdict,
                        "stopped": stopped(&run.stopped),
                        "states": run.states,
                    }),
                    Err(e) => json!({"error": e}),
                };
                cells.insert(key, cell);
            }
            let _ = writeln!(
                lines,
                "{}",
                json!({"path": rel, "order": named.name, "cells": cells})
            );
            eprintln!("{}: {rel}", named.name);
        }
        std::fs::write(at.join(file_of(named.name)), lines).expect("the order's file");
    }
}

// ── The report ──────────────────────────────────────────────────────────

/// One cell as the report reads it: the work with what the order's
/// structure added.
#[derive(Clone)]
struct Cell {
    work: [u64; 2],
    added: [u64; 2],
    exhausted: bool,
    verdict: Option<String>,
    stopped: bool,
    tree: bool,
    states: Value,
}

impl Cell {
    fn of(v: &Value) -> Option<Cell> {
        let u = |k: &str| v[k].as_u64();
        Some(Cell {
            work: [u("saturations")?, u("facts")?],
            added: [v["added"][0].as_u64()?, v["added"][1].as_u64()?],
            exhausted: v["exhausted"].as_bool()?,
            verdict: v["verdict"].as_str().map(str::to_string),
            stopped: !v["stopped"].is_null(),
            tree: v["route"].as_str() == Some("Tree"),
            states: v["states"].clone(),
        })
    }

    fn with(&self, u: usize) -> u64 {
        self.work[u] + self.added[u]
    }
}

type Table = BTreeMap<String, BTreeMap<String, Cell>>;

fn read(at: &Path) -> BTreeMap<String, Table> {
    let mut out: BTreeMap<String, Table> = BTreeMap::new();
    let mut names: Vec<PathBuf> = std::fs::read_dir(at)
        .expect("the orders directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .collect();
    names.sort();
    for file in names {
        for line in std::fs::read_to_string(&file).expect("a file").lines() {
            let v: Value = serde_json::from_str(line).expect("a JSON line");
            let (path, order) = (
                v["path"].as_str().expect("a path").to_string(),
                v["order"].as_str().expect("an order").to_string(),
            );
            let cells = v["cells"].as_object().expect("cells");
            for (q, c) in cells {
                if let Some(cell) = Cell::of(c) {
                    out.entry(order.clone())
                        .or_default()
                        .entry(q.clone())
                        .or_default()
                        .insert(path.clone(), cell);
                } else {
                    eprintln!("{order} {q} {path}: {c}");
                }
            }
        }
    }
    out
}

fn n(x: u64) -> String {
    let s = x.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

/// `--orders-report`: the tables of every order under `DIR/orders`.
pub fn report(args: Vec<String>) {
    let mut out = repo_root().join("build/structure");
    let mut args = args.into_iter();
    while let Some(a) = args.next() {
        if a == "--out" {
            out = PathBuf::from(args.next().expect("--out DIR"));
        }
    }
    let at = out.join("orders");
    let all = read(&at);
    let Some(today) = all.get("today") else {
        eprintln!("no `today` under {}", at.display());
        return;
    };
    let names: Vec<&str> = orders()
        .iter()
        .map(|o| o.name)
        .filter(|n| all.contains_key(*n))
        .collect();
    let mut md = String::new();
    let _ = writeln!(md, "# The orders of the obligation tree, measured\n");
    for o in orders().iter().filter(|o| all.contains_key(o.name)) {
        let _ = writeln!(md, "- `{}`: {}", o.name, o.what);
    }

    // What may not move: a verdict, and a line listed whole.
    let _ = writeln!(md, "\n## Answers\n");
    let mut differing = 0u64;
    for name in &names {
        for q in QUESTIONS {
            let (Some(a), Some(b)) = (today.get(q), all[*name].get(q)) else {
                continue;
            };
            for (path, x) in a {
                let Some(y) = b.get(path) else {
                    let _ = writeln!(md, "- **missing** `{name}` {q} `{path}`");
                    differing += 1;
                    continue;
                };
                if x.stopped || y.stopped {
                    continue;
                }
                let whole = q == "minimal" || q == "maximal";
                let same = x.verdict == y.verdict
                    && x.exhausted == y.exhausted
                    && (!whole || {
                        let set = |c: &Cell| {
                            let mut v: Vec<String> = c
                                .states
                                .as_array()
                                .map(|a| a.iter().map(Value::to_string).collect())
                                .unwrap_or_default();
                            v.sort();
                            v
                        };
                        // The maximal line's states are their own
                        // commitments; a minimal state's commitment is one
                        // of several, so only their count is held here.
                        if q == "maximal" {
                            set(x) == set(y)
                        } else {
                            set(x).len() == set(y).len()
                        }
                    });
                if !same {
                    differing += 1;
                    let _ = writeln!(
                        md,
                        "- **ANOTHER ANSWER** `{name}` {q} `{path}`: {:?} {} against {:?} {}",
                        y.verdict, y.exhausted, x.verdict, x.exhausted
                    );
                }
            }
        }
    }
    let _ = writeln!(
        md,
        "\n{differing} cells answer otherwise than `today` among those no budget stopped either \
         way (a verdict, `exhausted`, the maximal line's states, the minimal line's count)."
    );

    // Per question: each order against today.
    for q in QUESTIONS.iter() {
        let _ = writeln!(md, "\n## `{q}`\n");
        let Some(base) = today.get(*q) else { continue };
        let tree_cells = base.values().filter(|c| c.tree).count();
        let _ = writeln!(
            md,
            "{} programs, {tree_cells} of them answered by a tree today.\n",
            base.len()
        );
        let _ = writeln!(
            md,
            "| order | saturations | facts | not stopped both ways: saturations | facts | \
             `today` on those cells: saturations | facts | finished both ways: saturations | \
             facts | lower | higher | higher by more than a tenth | stopped where today \
             finishes | finishes where today stops | verdict |"
        );
        let _ = writeln!(
            md,
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|"
        );
        let mut losers: Vec<String> = Vec::new();
        // The rule as the price list's defaults read it: a stopped cell
        // counts with what it had spent, in the total and in the tenth.
        let (mut every_rows, mut every_losers): (Vec<String>, Vec<String>) =
            (Vec::new(), Vec::new());
        for name in &names {
            let Some(cells) = all[*name].get(*q) else {
                continue;
            };
            let (mut every, mut every_base) = ([0u64; 2], [0u64; 2]);
            let mut over: Vec<String> = Vec::new();
            for (path, x) in base {
                let Some(y) = cells.get(path) else { continue };
                for u in 0..2 {
                    every[u] += y.with(u);
                    every_base[u] += x.with(u);
                }
                if (0..2).any(|u| y.with(u) * 10 > x.with(u) * 11) {
                    let mark = |c: &Cell| if c.stopped { ", stopped" } else { "" };
                    over.push(format!(
                        "`{path}` {} · {}{} → {} · {}{}",
                        n(x.with(0)),
                        n(x.with(1)),
                        mark(x),
                        n(y.with(0)),
                        n(y.with(1)),
                        mark(y)
                    ));
                }
            }
            let go = over.is_empty() && (0..2).all(|u| every[u] < every_base[u]);
            let word = if *name == "today" {
                "—"
            } else if go {
                "**go**"
            } else {
                "no"
            };
            every_rows.push(format!(
                "| `{name}` | {} · {} | {} · {} | {} | {word} |",
                n(every[0]),
                n(every[1]),
                n(every_base[0]),
                n(every_base[1]),
                over.len()
            ));
            if !over.is_empty() {
                every_losers.push(format!("- `{name}`: {}", over.join("; ")));
            }
        }
        for name in &names {
            let Some(cells) = all[*name].get(*q) else {
                continue;
            };
            let (mut tot, mut fin) = ([0u64; 2], [0u64; 2]);
            // Over the cells a budget does not stop both ways: a cell
            // stopped both ways spent its budget either way.
            let (mut decided, mut decided_base) = ([0u64; 2], [0u64; 2]);
            let (mut lower, mut higher, mut tenth, mut newly_stopped, mut newly_done) =
                (0u64, 0u64, 0u64, 0u64, 0u64);
            let mut lost: Vec<String> = Vec::new();
            for (path, x) in base {
                let Some(y) = cells.get(path) else { continue };
                for (u, t) in tot.iter_mut().enumerate() {
                    *t += y.with(u);
                }
                if !(x.stopped && y.stopped) {
                    for u in 0..2 {
                        decided[u] += y.with(u);
                        decided_base[u] += x.with(u);
                    }
                }
                match (x.stopped, y.stopped) {
                    (false, false) => {
                        for (u, f) in fin.iter_mut().enumerate() {
                            *f += y.with(u);
                        }
                    }
                    (false, true) => {
                        newly_stopped += 1;
                        lost.push(format!("`{path}` **stopped**"));
                        continue;
                    }
                    (true, false) => {
                        newly_done += 1;
                        continue;
                    }
                    (true, true) => continue,
                }
                let lo = (0..2).any(|u| y.with(u) < x.with(u));
                let hi = (0..2).any(|u| y.with(u) > x.with(u));
                let over = (0..2).any(|u| y.with(u) * 10 > x.with(u) * 11);
                lower += u64::from(lo && !hi);
                higher += u64::from(hi);
                if over {
                    tenth += 1;
                    lost.push(format!(
                        "`{path}` {} · {} → {} · {}",
                        n(x.with(0)),
                        n(x.with(1)),
                        n(y.with(0)),
                        n(y.with(1))
                    ));
                }
            }
            // The rule: the total lower with the structure's cost in it, in
            // saturations and in facts, and no program higher by more than
            // a tenth in either. The total is taken over the cells a budget
            // does not stop both ways.
            let go =
                tenth == 0 && newly_stopped == 0 && (0..2).all(|u| decided[u] < decided_base[u]);
            let word = if *name == "today" {
                "—"
            } else if go {
                "**go**"
            } else {
                "no"
            };
            let _ = writeln!(
                md,
                "| `{name}` | {} | {} | {} | {} | {} | {} | {} | {} | {lower} | {higher} | \
                 {tenth} | {newly_stopped} | {newly_done} | {word} |",
                n(tot[0]),
                n(tot[1]),
                n(decided[0]),
                n(decided[1]),
                n(decided_base[0]),
                n(decided_base[1]),
                n(fin[0]),
                n(fin[1]),
            );
            if !lost.is_empty() {
                losers.push(format!("- `{name}`: {}", lost.join("; ")));
            }
        }
        let _ = writeln!(md, "\n### The losers under `{q}`, named\n");
        for l in losers {
            let _ = writeln!(md, "{l}");
        }
        let _ = writeln!(
            md,
            "\n### `{q}` by the price list's defaults\n\nEvery cell in the total and in the \
             tenth, a cell a budget stopped with what it had spent; a *go* needs both units.\n\n\
             | order | every cell | `today`'s | cells higher by more than a tenth | verdict |\n\
             |---|---:|---:|---:|---|"
        );
        for r in every_rows {
            let _ = writeln!(md, "{r}");
        }
        let _ = writeln!(md);
        for l in every_losers {
            let _ = writeln!(md, "{l}");
        }
        // The instances.
        let _ = writeln!(md, "\n### The instances under `{q}`\n");
        let _ = write!(md, "| order |");
        for (tag, _) in INSTANCES {
            let _ = write!(md, " {tag} |");
        }
        let _ = writeln!(md, "\n|---|{}", "---:|".repeat(INSTANCES.len()));
        for name in &names {
            let Some(cells) = all[*name].get(*q) else {
                continue;
            };
            let _ = write!(md, "| `{name}` |");
            for (_, path) in INSTANCES {
                match cells.get(path) {
                    Some(c) => {
                        let _ = write!(
                            md,
                            " {} · {}{} |",
                            n(c.with(0)),
                            n(c.with(1)),
                            if c.stopped { ", stopped" } else { "" }
                        );
                    }
                    None => {
                        let _ = write!(md, " |");
                    }
                }
            }
            let _ = writeln!(md);
        }
        // The states listed.
        if !matches!(*q, "minimal" | "maximal") {
            let _ = writeln!(
                md,
                "\n### Programs whose listed states are others under `{q}`\n"
            );
            for name in &names {
                let Some(cells) = all[*name].get(*q) else {
                    continue;
                };
                let moved: Vec<&str> = base
                    .iter()
                    .filter(|(path, x)| {
                        cells
                            .get(*path)
                            .is_some_and(|y| !x.stopped && !y.stopped && x.states != y.states)
                    })
                    .map(|(path, _)| path.as_str())
                    .collect();
                if !moved.is_empty() {
                    let _ = writeln!(md, "- `{name}`, {}: {}", moved.len(), moved.join(", "));
                }
            }
        }
    }
    std::fs::write(at.join("orders.md"), &md).expect("orders.md");
    print!("{md}");
}
