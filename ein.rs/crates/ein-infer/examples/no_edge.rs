//! **A state that closes no coupling cannot die?** M1f S1f.10.18's T2: the
//! claim measured on the nodes the routes visit, with no route changed.
//!
//! ```sh
//! cargo run --release -p ein-infer --example no_edge -- [--out DIR] [--jobs J] [FILE…]
//! cargo run --release -p ein-infer --example no_edge -- --report [--out DIR]
//! ```
//!
//! **The claim**, as the stage states it: on a monotone program, let `S` be
//! an alive state and `b` a candidate. If no edge of the coupling read off
//! the whole node ([`ein_infer::structure::NodeCoupling`]) that holds a
//! question of `b` has each of its questions met by `S ∪ {b}`, then
//! `S ∪ {b}` is alive. A question is met by a set that holds one of its
//! members.
//!
//! For every corpus file and register witness that loads and **searches**
//! (or the `FILE`s given), this reads:
//!
//! - **the whole node**, once, by `structure::node_coupling` on a load of its
//!   own: its questions, edges, pairs, what it cost (one saturation, its
//!   facts beyond root) and what its bounds cut, with how many of its facts'
//!   labels are a union of two classes or more (`NodeWork::merged`);
//! - **the singles**: each candidate's own node, by the node function
//!   ([`ein_infer::region::Nodes`]), dead or alive. The reader's, no cell's;
//! - **each of the six questions** `ein solve` asks, describe, `--count`,
//!   `--single`, `-n 1`, `--minimal` and `--maximal`, at the default budget,
//!   its nodes read in process through [`ein_infer::events::Watch`] in visit
//!   order. A node whose commitment is `S ∪ {b}`, `S` the closure of an alive
//!   node visited before it and `b ∉ S`, is an **extension**; for each such
//!   way of reading it, four tests:
//!   - `E`, **the claim's**: some edge through a question of `b` has every
//!     question met by `S ∪ {b}`;
//!   - `P`: `b` and a member of `S` are one of the node's pairs, a conflict of
//!     exactly two candidates, which the reading makes no edge of;
//!   - `Q`: `S` holds a member of a question of `b`, where a contradiction
//!     inside one question is no edge either;
//!   - `A`: `b` is dead alone.
//!
//!   An extension **closes no edge** as stated where some way of reading it
//!   fails `E`, and **closes nothing** under the narrowed test where some way
//!   fails all four. One that closes none and was alive is a saturation the
//!   licence would save (an upper bound: the licence decides the node's
//!   `dead` bit, not its closure or its debts); **one that closes none and
//!   was dead is a counterexample**, listed with its set and its `b`;
//! - **the cube**, on every monotone program within `ein oracle`'s limit
//!   ([`ein_infer::region::compute`]): every alive state `S` and every
//!   `b ∉ S`, the same tests against the node of `S ∪ {b}`.
//!
//! Under `--out` (default `build/no_edge` under the repo root) it writes
//! `no_edge.jsonl`, a line a program, and `counterexamples.jsonl`, a line a
//! dead node that closes no edge. `--report` reads them back and prints the
//! tables and **the rule for *go***, S1f.10.13's as its defaults read it:
//! per question, over the programs that search, the work with the licence
//! (each alive node closing none saved, its saturation and its facts, and
//! the whole node's cost added where the run did not read that node itself)
//! against the work without; *go* where the total is lower and no program's
//! cell is higher by more than one tenth, in saturations and in facts alike,
//! a stopped cell counted with what it spent and a flagged program in every
//! total with nothing applied to it. Four ways are priced, each on the same
//! node: `stated` (the test `E`), `narrowed` (all four tests), `unmerged`
//! (the narrowed test, saving only on a program whose whole node gives no
//! fact a label of two classes, the node paid on every monotone program),
//! and `late` (`unmerged`, the node read only where the tree's order would
//! read it, after twenty saturations and within a twentieth of the facts,
//! and only the nodes visited after that moment saved).
//!
//! It measures; no route reads any of it.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::events::{Events, Visited, Watch};
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::region::{self, Cores, Nodes};
use ein_infer::structure::{self, NodeCoupling};
use ein_ir::{Ast, load, parse};
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::{Value, json};

// What *a program searches* means, by the determinism sweeps' one
// definition.
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

fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return false;
    };
    searching::searches(&ast, &mut terms, &mut kb)
}

/// The six questions `ein solve` asks: a key and the mode.
fn questions() -> [(&'static str, Mode); 6] {
    let m = |which, ask| Mode { which, ask };
    [
        ("describe", m(Which::All, Ask::Describe)),
        ("count", m(Which::All, Ask::Count)),
        ("single", m(Which::All, Ask::Single)),
        ("n1", m(Which::All, Ask::Find(1))),
        ("minimal", m(Which::Minimal, Ask::Describe)),
        ("maximal", m(Which::Maximal, Ask::Describe)),
    ]
}

// ── The reading ─────────────────────────────────────────────────────────

/// What the tests read of one program: the whole node's coupling, by
/// candidate index, and the singles.
struct Reading {
    n: usize,
    texts: Vec<String>,
    index: Arc<FxHashMap<String, u32>>,
    questions: Vec<Vec<u32>>,
    question_of: Vec<Vec<u32>>,
    edges: Vec<Vec<u32>>,
    /// For each candidate, the edges that hold one of its questions.
    edges_of: Vec<Vec<u32>>,
    /// For each candidate, the candidates it is a pair with.
    pair_of: Vec<Vec<u32>>,
    dead_alone: Vec<bool>,
}

/// The four tests of one way of reading a node: `S ∪ {b}`, `x` its members
/// as a mask over the candidates.
#[derive(Clone, Copy, Debug, Default)]
struct Tests {
    edge: bool,
    pair: bool,
    question: bool,
    alone: bool,
}

impl Tests {
    fn narrowed(self) -> bool {
        self.edge || self.pair || self.question || self.alone
    }
    /// The first test after `E` that the way meets, which is what explains
    /// a death the claim as stated misses.
    fn why(self) -> &'static str {
        if self.alone {
            "dead-alone"
        } else if self.pair {
            "pair"
        } else if self.question {
            "one-question"
        } else {
            "none"
        }
    }
}

impl Reading {
    fn met(&self, q: u32, x: &[bool]) -> bool {
        self.questions[q as usize].iter().any(|&c| x[c as usize])
    }

    /// The tests of `S ∪ {b}`, `x` its members and `b` in it.
    fn tests(&self, x: &[bool], b: u32) -> Tests {
        let edge = self.edges_of[b as usize]
            .iter()
            .any(|&e| self.edges[e as usize].iter().all(|&q| self.met(q, x)));
        let pair = self.pair_of[b as usize].iter().any(|&c| x[c as usize]);
        let question = self.question_of[b as usize].iter().any(|&q| {
            self.questions[q as usize]
                .iter()
                .any(|&c| c != b && x[c as usize])
        });
        Tests {
            edge,
            pair,
            question,
            alone: self.dead_alone[b as usize],
        }
    }
}

/// The whole node and the singles of the program at `path`, on a load of
/// their own.
fn read(path: &Path) -> Result<(Reading, NodeCoupling, u64), String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    let c = structure::node_coupling(&mut kb, &mut terms, &ast, &structure::Options::default())
        .map_err(|e| e.to_string())?;
    let n = c.n();
    let texts: Vec<String> = c.candidates.iter().map(|x| x.text.clone()).collect();
    let index: Arc<FxHashMap<String, u32>> = Arc::new(
        texts
            .iter()
            .enumerate()
            .map(|(i, t)| (t.clone(), i as u32))
            .collect(),
    );
    let mut edges_of: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (e, edge) in c.edges.iter().enumerate() {
        let mut members: FxHashSet<u32> = FxHashSet::default();
        for &q in edge {
            members.extend(c.questions[q as usize].iter().copied());
        }
        for b in members {
            edges_of[b as usize].push(e as u32);
        }
    }
    let mut pair_of: Vec<Vec<u32>> = vec![Vec::new(); n];
    for &(a, b) in &c.pairs {
        pair_of[a as usize].push(b);
        pair_of[b as usize].push(a);
    }
    // The singles, each the node function's own node of one candidate.
    let facts: Vec<_> = c.candidates.iter().map(|x| x.fact).collect();
    let mut dead_alone = vec![false; n];
    let mut saturations = 0u64;
    if n > 0
        && !c
            .node_stopped
            .is_some_and(|s| s == structure::NodeStop::RootDead)
    {
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &[]).map_err(|e| e.to_string())?;
        for (b, &f) in facts.iter().enumerate() {
            dead_alone[b] = nodes
                .with_fixpoints(&[vec![f]], |kbs, terms| {
                    ein_infer::has_contradiction(&kbs[0], terms)
                })
                .map_err(|e| e.to_string())?;
            saturations += 1;
        }
    } else {
        dead_alone = vec![true; n];
    }
    Ok((
        Reading {
            n,
            texts,
            index,
            questions: c.questions.clone(),
            question_of: c.question_of.clone(),
            edges: c.edges.clone(),
            edges_of,
            pair_of,
            dead_alone,
        },
        c,
        saturations,
    ))
}

// ── A question, watched ─────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Dead,
    Owing,
    InR,
}

struct Node {
    set: Box<[u32]>,
    kind: Kind,
    closure: Box<[u32]>,
    facts: u64,
}

#[derive(Default)]
struct Kept {
    nodes: Vec<Node>,
    unmapped: u64,
}

struct Collect {
    index: Arc<FxHashMap<String, u32>>,
    kept: Arc<Mutex<Kept>>,
}

impl Collect {
    fn set(&self, texts: &[String], miss: &mut u64) -> Box<[u32]> {
        let mut out: Vec<u32> = texts
            .iter()
            .filter_map(|t| {
                let b = self.index.get(t).copied();
                *miss += u64::from(b.is_none());
                b
            })
            .collect();
        out.sort_unstable();
        out.dedup();
        out.into_boxed_slice()
    }
}

impl Watch for Collect {
    fn node(&mut self, v: &Visited<'_>) {
        let kind = match v.kind {
            "dead" => Kind::Dead,
            "owing" => Kind::Owing,
            _ => Kind::InR,
        };
        let mut miss = 0;
        let set = self.set(v.commitment, &mut miss);
        let closure = if kind == Kind::Dead {
            Box::default()
        } else {
            self.set(v.closure, &mut miss)
        };
        let mut kept = self.kept.lock().expect("one thread watches");
        kept.unmapped += miss;
        kept.nodes.push(Node {
            set,
            kind,
            closure,
            facts: v.facts,
        });
    }
}

fn stopped(s: &Option<Stopped>) -> Value {
    match s {
        None => Value::Null,
        Some(Stopped::Budget { spent, projected }) => {
            json!({"why": "budget", "spent": spent, "projected": projected})
        }
        Some(Stopped::Limit { n, limit }) => json!({"why": "limit", "n": n, "limit": limit}),
        Some(Stopped::Deadline) => json!({"why": "deadline"}),
        Some(Stopped::NoSearch) => json!({"why": "no-search"}),
    }
}

/// What the tests counted over a set of extensions.
#[derive(Default)]
struct Tally {
    extensions: u64,
    alive: u64,
    dead: u64,
    /// As stated: closing no edge, alive (`[nodes, facts]`) and dead.
    stated_alive: [u64; 2],
    stated_dead: u64,
    /// Of the dead ones closing no edge, by the first other test met.
    stated_dead_by: BTreeMap<&'static str, u64>,
    /// Narrowed: failing all four, alive and dead.
    narrowed_alive: [u64; 2],
    narrowed_dead: u64,
}

impl Tally {
    fn json(&self) -> Value {
        json!({
            "extensions": self.extensions,
            "alive": self.alive,
            "dead": self.dead,
            "stated": {
                "alive_closing_none": self.stated_alive[0],
                "alive_closing_none_facts": self.stated_alive[1],
                "dead_closing_none": self.stated_dead,
                "dead_closing_none_by": self.stated_dead_by,
            },
            "narrowed": {
                "alive_closing_none": self.narrowed_alive[0],
                "alive_closing_none_facts": self.narrowed_alive[1],
                "dead_closing_none": self.narrowed_dead,
            },
        })
    }
}

/// One counterexample: a dead node, a way of reading it that closes no edge.
fn counterexample(r: &Reading, path: &str, question: &str, set: &[u32], b: u32, t: Tests) -> Value {
    let s: Vec<&str> = set
        .iter()
        .filter(|&&c| c != b)
        .map(|&c| r.texts[c as usize].as_str())
        .collect();
    json!({
        "program": path,
        "question": question,
        "s": s,
        "b": r.texts[b as usize],
        "why": t.why(),
        "narrowed": !t.narrowed(),
    })
}

/// Read one extension `set`, its ways `bs`: what it adds to `tally`, and the
/// counterexamples it is.
fn judge(
    r: &Reading,
    set: &[u32],
    bs: &[u32],
    dead: bool,
    facts: u64,
    tally: &mut Tally,
    out: &mut Vec<(u32, Tests)>,
) {
    let mut x = vec![false; r.n];
    for &c in set {
        x[c as usize] = true;
    }
    tally.extensions += 1;
    if dead {
        tally.dead += 1;
    } else {
        tally.alive += 1;
    }
    let ways: Vec<(u32, Tests)> = bs.iter().map(|&b| (b, r.tests(&x, b))).collect();
    let stated = ways.iter().find(|(_, t)| !t.edge);
    let narrowed = ways.iter().find(|(_, t)| !t.narrowed());
    if let Some(&(b, t)) = stated {
        if dead {
            tally.stated_dead += 1;
            *tally.stated_dead_by.entry(t.why()).or_default() += 1;
            out.push((b, t));
        } else {
            tally.stated_alive[0] += 1;
            tally.stated_alive[1] += facts;
        }
    }
    if narrowed.is_some() {
        if dead {
            tally.narrowed_dead += 1;
        } else {
            tally.narrowed_alive[0] += 1;
            tally.narrowed_alive[1] += facts;
        }
    }
}

/// The moment the order reads the whole node (`order::READ_AFTER`,
/// `READ_SHARE`): once the run has saturated this many nodes …
const READ_AFTER: u64 = 20;
/// … and the node costs at most this share of the facts it has derived.
const READ_SHARE: u64 = 20;

/// Ask `mode` of the program at `path`, loaded again, watched. `node_facts`
/// is the whole node's facts beyond root, for the moment a run could read
/// it as the tree's order does: the *late* saving counts only the nodes
/// visited after it.
fn ask(
    path: &Path,
    rel: &str,
    (key, mode): (&str, Mode),
    r: &Reading,
    node_facts: u64,
    counter: &mut Vec<Value>,
) -> Result<Value, String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    let opts = explore::Options {
        cores: Cores::READ_OUT,
        nogoods: key != "count",
        ..explore::Options::new(mode)
    };
    let kept = Arc::new(Mutex::new(Kept::default()));
    let mut events = Events::off().watched(Box::new(Collect {
        index: r.index.clone(),
        kept: kept.clone(),
    }));
    let a = explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
        .map_err(|e| e.to_string())?;
    drop(events);
    let kept = std::mem::take(&mut *kept.lock().expect("the run is over"));
    let monotone = a.monotonicity.is_monotone();
    let mut alive: FxHashSet<Box<[u32]>> = FxHashSet::default();
    let mut tally = Tally::default();
    let mut whole = false;
    let mut out: Vec<(u32, Tests)> = Vec::new();
    let mut facts_so_far = 0u64;
    let mut late_from: Option<usize> = None;
    let mut late = [0u64; 2];
    for (i, x) in kept.nodes.iter().enumerate() {
        if x.set.len() == r.n && r.n > 0 {
            whole = true;
        }
        facts_so_far += x.facts;
        let before = tally.narrowed_alive;
        let bs: Vec<u32> = x
            .set
            .iter()
            .copied()
            .filter(|&b| {
                let s: Box<[u32]> = x.set.iter().copied().filter(|&c| c != b).collect();
                alive.contains(&s)
            })
            .collect();
        if monotone && !bs.is_empty() {
            out.clear();
            judge(
                r,
                &x.set,
                &bs,
                x.kind == Kind::Dead,
                x.facts,
                &mut tally,
                &mut out,
            );
            for &(b, t) in &out {
                counter.push(counterexample(r, rel, key, &x.set, b, t));
            }
        }
        if late_from.is_some() {
            late[0] += tally.narrowed_alive[0] - before[0];
            late[1] += tally.narrowed_alive[1] - before[1];
        } else if i as u64 + 1 >= READ_AFTER && facts_so_far >= READ_SHARE * node_facts {
            late_from = Some(i + 1);
        }
        if x.kind != Kind::Dead {
            alive.insert(x.closure.clone());
        }
    }
    Ok(json!({
        "saturations": a.saturations,
        "facts": a.facts,
        "route": format!("{:?}", a.route),
        "exhausted": a.exhausted,
        "stopped": stopped(&a.stopped),
        "visited": kept.nodes.len(),
        "unmapped": kept.unmapped,
        "whole_node_visited": whole,
        "monotone": monotone,
        "tally": tally.json(),
        "late": {"from": late_from, "alive_closing_none": late[0], "alive_closing_none_facts": late[1]},
    }))
}

/// The cube: every alive state and every candidate outside it.
fn cube(path: &Path, rel: &str, r: &Reading, counter: &mut Vec<Value>) -> Result<Value, String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    let opts = region::Options {
        cores: Cores::READ_OUT,
        ..region::Options::default()
    };
    let out = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {})
        .map_err(|e| e.to_string())?;
    let region::Outcome::Region(g) = out else {
        return Ok(json!({"refused": true}));
    };
    if !g.monotonicity.is_monotone() {
        return Ok(json!({"flagged": true}));
    }
    // The cube's coordinates, as the reading's candidates.
    let map: Vec<u32> = g
        .candidates
        .iter()
        .map(|c| r.index.get(&c.text).copied().ok_or(c.text.clone()))
        .collect::<Result<_, _>>()
        .map_err(|t| format!("the cube's candidate {t} is not the reading's"))?;
    let n = g.n();
    let mut tally = Tally::default();
    let mut states = 0u64;
    let mut ways: Vec<(u32, Tests)> = Vec::new();
    for s in 0..(1u64 << n) {
        if g.status[s as usize] & region::DEAD != 0 || g.nodes[s as usize].closure != s {
            continue;
        }
        states += 1;
        for i in 0..n {
            if s >> i & 1 == 1 {
                continue;
            }
            let x = s | 1 << i;
            let mut set: Vec<u32> = (0..n).filter(|j| x >> j & 1 == 1).map(|j| map[j]).collect();
            set.sort_unstable();
            let dead = g.status[x as usize] & region::DEAD != 0;
            ways.clear();
            judge(r, &set, &[map[i]], dead, 0, &mut tally, &mut ways);
            for &(b, t) in &ways {
                counter.push(counterexample(r, rel, "cube", &set, b, t));
            }
        }
    }
    Ok(json!({"n": n, "states": states, "tally": tally.json()}))
}

/// Everything about one program: a line of `no_edge.jsonl`, and its
/// counterexamples.
fn program(path: &Path) -> (Value, Vec<Value>) {
    let rel = path
        .strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let mut counter: Vec<Value> = Vec::new();
    let (r, c, single_saturations) = match read(path) {
        Ok(x) => x,
        Err(e) => return (json!({"path": rel, "error": e}), counter),
    };
    let node = c.node.map(|w| {
        json!({
            "saturations": w.saturations,
            "facts": w.facts,
            "applications": w.applications,
            "rounds": w.rounds,
            "merged": w.merged,
        })
    });
    let mut by_arity: BTreeMap<usize, u64> = BTreeMap::new();
    for e in &c.edges {
        *by_arity.entry(e.len()).or_default() += 1;
    }
    let coupling = json!({
        "complete": c.complete(),
        "stopped": c.node_stopped.map(|s| s.as_str()),
        "cut": {"ways": c.cut.ways, "descriptions": c.cut.descriptions, "unsettled": c.cut.unsettled},
        "dead": c.dead,
        "questions": c.questions.len(),
        "owed": c.owed,
        "edges": c.edges.len(),
        "edges_by_arity": by_arity,
        "pairs": c.pairs.len(),
        "components": c.components.len(),
        "node": node,
    });
    let mut cells = serde_json::Map::new();
    for (key, mode) in questions() {
        let node_facts = c.node.map_or(0, |w| w.facts);
        let cell = ask(path, &rel, (key, mode), &r, node_facts, &mut counter)
            .unwrap_or_else(|e| json!({"error": e}));
        cells.insert(key.to_string(), cell);
    }
    let cube = if r.n as u32 <= region::DEFAULT_LIMIT {
        cube(path, &rel, &r, &mut counter).unwrap_or_else(|e| json!({"error": e}))
    } else {
        Value::Null
    };
    // On a universe within the cube's limit, the reading itself, as text.
    let shown = (r.n as u32 <= region::DEFAULT_LIMIT).then(|| {
        let q = |q: u32| -> Vec<&str> {
            c.questions[q as usize]
                .iter()
                .map(|&b| r.texts[b as usize].as_str())
                .collect()
        };
        json!({
            "questions": (0..c.questions.len() as u32).map(q).collect::<Vec<_>>(),
            "edges": c.edges.iter().map(|e| e.iter().map(|&x| q(x)).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "pairs": c.pairs.iter().map(|&(a, b)| [r.texts[a as usize].as_str(), r.texts[b as usize].as_str()]).collect::<Vec<_>>(),
        })
    });
    let line = json!({
        "path": rel,
        "n": r.n,
        "reading": shown,
        "dead_alone": r.dead_alone.iter().filter(|&&d| d).count(),
        "singles_saturations": single_saturations,
        "coupling": coupling,
        "cells": cells,
        "cube": cube,
    });
    (line, counter)
}

fn run(args: Vec<String>) {
    let mut out = repo_root().join("build/no_edge");
    let mut jobs = 4usize;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = args.into_iter();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            "--jobs" => jobs = args.next().and_then(|j| j.parse().ok()).expect("--jobs J"),
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
        for row in ein_corpus::register().case {
            for (_, a) in row.answers() {
                files.push(repo_root().join(row.witness_of(a)));
            }
        }
        files.sort();
        files.dedup();
    }
    let files: Vec<PathBuf> = files
        .into_iter()
        .map(|p| {
            if p.is_absolute() {
                p
            } else {
                std::env::current_dir().expect("a directory").join(p)
            }
        })
        .collect();
    std::fs::create_dir_all(&out).expect("the output directory");
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<(usize, Value, Vec<Value>)>> = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    let Some(path) = files.get(k) else {
                        break;
                    };
                    if !searches(path) {
                        continue;
                    }
                    let (line, counter) = program(path);
                    eprintln!(
                        "{}: {} counterexamples",
                        line["path"].as_str().unwrap_or("?"),
                        counter.len()
                    );
                    done.lock().expect("a result").push((k, line, counter));
                }
            });
        }
    });
    let mut done = done.into_inner().expect("every worker is done");
    done.sort_by_key(|(k, _, _)| *k);
    let mut lines = String::new();
    let mut counter = String::new();
    for (_, line, cs) in &done {
        let _ = writeln!(lines, "{line}");
        for c in cs {
            let _ = writeln!(counter, "{c}");
        }
    }
    std::fs::write(out.join("no_edge.jsonl"), lines).expect("no_edge.jsonl");
    std::fs::write(out.join("counterexamples.jsonl"), counter).expect("counterexamples.jsonl");
    report(&out);
}

// ── The report ──────────────────────────────────────────────────────────

/// A cell's work: `[saturations, facts]`.
fn work(cell: &Value) -> [u64; 2] {
    [
        cell["saturations"].as_u64().unwrap_or(0),
        cell["facts"].as_u64().unwrap_or(0),
    ]
}

/// The cell's work with the licence of `variant` taken: each alive node
/// closing none saved, and the whole node's cost added where the run did
/// not read that node itself. A flagged program, a cell in error, and a
/// program whose whole node was not read whole are as they were but for the
/// node's cost, which the licence pays to learn that.
fn with(line: &Value, cell: &Value, variant: &str) -> [u64; 2] {
    let base = work(cell);
    if !cell["monotone"].as_bool().unwrap_or(false) || cell.get("error").is_some() {
        return base;
    }
    let c = &line["coupling"];
    // `late` reads the node only where the order would: never, on a cell
    // that ends before the moment comes.
    let read = variant != "late" || !cell["late"]["from"].is_null();
    let cost = if !read || cell["whole_node_visited"].as_bool().unwrap_or(false) {
        [0, 0]
    } else {
        [
            c["node"]["saturations"].as_u64().unwrap_or(0),
            c["node"]["facts"].as_u64().unwrap_or(0),
        ]
    };
    let applies = read
        && c["complete"].as_bool().unwrap_or(false)
        && (variant == "stated"
            || variant == "narrowed"
            || c["node"]["merged"].as_u64() == Some(0));
    let t = match variant {
        "stated" => &cell["tally"]["stated"],
        "late" => &cell["late"],
        _ => &cell["tally"]["narrowed"],
    };
    let saved = if applies {
        [
            t["alive_closing_none"].as_u64().unwrap_or(0),
            t["alive_closing_none_facts"].as_u64().unwrap_or(0),
        ]
    } else {
        [0, 0]
    };
    [base[0] - saved[0] + cost[0], base[1] - saved[1] + cost[1]]
}

fn report(out: &Path) {
    let text = std::fs::read_to_string(out.join("no_edge.jsonl")).expect("no_edge.jsonl");
    let lines: Vec<Value> = text
        .lines()
        .map(|l| serde_json::from_str(l).expect("a line"))
        .collect();
    let read: Vec<&Value> = lines.iter().filter(|l| l.get("error").is_none()).collect();
    let mut s = String::new();
    let _ = writeln!(
        s,
        "{} programs that search, {} read ({} monotone in describe's cell), {} with the whole node read whole, \
         {} of those with no label a union",
        lines.len(),
        read.len(),
        read.iter()
            .filter(|l| l["cells"]["describe"]["monotone"].as_bool() == Some(true))
            .count(),
        read.iter()
            .filter(|l| l["coupling"]["complete"].as_bool() == Some(true))
            .count(),
        read.iter()
            .filter(|l| l["coupling"]["complete"].as_bool() == Some(true)
                && l["coupling"]["node"]["merged"].as_u64() == Some(0))
            .count(),
    );
    let _ = writeln!(
        s,
        "\n| question | extensions | alive / dead | stated: alive closing none (nodes · facts) | stated: dead closing none | by dead-alone / pair / one-question / none | narrowed: alive closing none | narrowed: dead closing none |"
    );
    let _ = writeln!(s, "|---|---:|---|---|---:|---|---|---:|");
    let keys = [
        "describe", "count", "single", "n1", "minimal", "maximal", "cube",
    ];
    for key in keys {
        let mut sum: BTreeMap<&str, u64> = BTreeMap::new();
        let mut by: BTreeMap<String, u64> = BTreeMap::new();
        for l in &read {
            let t = if key == "cube" {
                &l["cube"]["tally"]
            } else {
                &l["cells"][key]["tally"]
            };
            if t.is_null() {
                continue;
            }
            let mut add = |k: &'static str, v: &Value| {
                *sum.entry(k).or_default() += v.as_u64().unwrap_or(0);
            };
            add("extensions", &t["extensions"]);
            add("alive", &t["alive"]);
            add("dead", &t["dead"]);
            add("sa", &t["stated"]["alive_closing_none"]);
            add("saf", &t["stated"]["alive_closing_none_facts"]);
            add("sd", &t["stated"]["dead_closing_none"]);
            add("na", &t["narrowed"]["alive_closing_none"]);
            add("naf", &t["narrowed"]["alive_closing_none_facts"]);
            add("nd", &t["narrowed"]["dead_closing_none"]);
            if let Some(m) = t["stated"]["dead_closing_none_by"].as_object() {
                for (k, v) in m {
                    *by.entry(k.clone()).or_default() += v.as_u64().unwrap_or(0);
                }
            }
        }
        let g = |k: &str| sum.get(k).copied().unwrap_or(0);
        let b = |k: &str| by.get(k).copied().unwrap_or(0);
        let _ = writeln!(
            s,
            "| {key} | {} | {} / {} | {} · {} | {} | {} / {} / {} / {} | {} · {} | {} |",
            g("extensions"),
            g("alive"),
            g("dead"),
            g("sa"),
            g("saf"),
            g("sd"),
            b("dead-alone"),
            b("pair"),
            b("one-question"),
            b("none"),
            g("na"),
            g("naf"),
            g("nd"),
        );
    }
    // The rule for go, per question and variant.
    let _ = writeln!(
        s,
        "\n| question | variant | total, without | total, with | higher by more than a tenth | go? |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|");
    for key in ["describe", "count", "single", "n1", "minimal", "maximal"] {
        for variant in ["stated", "narrowed", "unmerged", "late"] {
            let mut base = [0u64; 2];
            let mut withs = [0u64; 2];
            let mut losers: Vec<String> = Vec::new();
            for l in &read {
                let cell = &l["cells"][key];
                if cell.is_null() {
                    continue;
                }
                let b = work(cell);
                let w = with(l, cell, variant);
                for u in 0..2 {
                    base[u] += b[u];
                    withs[u] += w[u];
                }
                let higher = (0..2).any(|u| {
                    if b[u] == 0 {
                        w[u] > 0
                    } else {
                        w[u] * 10 > b[u] * 11
                    }
                });
                if higher {
                    losers.push(format!(
                        "{} {}·{} → {}·{}",
                        l["path"].as_str().unwrap_or("?"),
                        b[0],
                        b[1],
                        w[0],
                        w[1]
                    ));
                }
            }
            let go = withs[0] < base[0] && withs[1] < base[1] && losers.is_empty();
            let _ = writeln!(
                s,
                "| {key} | {variant} | {} · {} | {} · {} | {} | {} |",
                base[0],
                base[1],
                withs[0],
                withs[1],
                losers.len(),
                if go { "go" } else { "no" }
            );
            if !losers.is_empty() {
                let mut shown = losers.clone();
                shown.truncate(8);
                let _ = writeln!(
                    s,
                    "|  |  |  |  | {}{} |  |",
                    shown.join("; "),
                    if losers.len() > 8 { "; …" } else { "" }
                );
            }
        }
    }
    std::fs::write(out.join("no_edge.md"), &s).expect("no_edge.md");
    print!("{s}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--report") {
        let mut out = repo_root().join("build/no_edge");
        let mut it = args.iter();
        while let Some(a) = it.next() {
            if a == "--out" {
                out = PathBuf::from(it.next().expect("--out DIR"));
            }
        }
        report(&out);
        return;
    }
    run(args);
}
