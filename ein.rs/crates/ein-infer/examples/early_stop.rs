//! What a dead node stopped at its first contradiction would save, under
//! FIFO and under a fail-first agenda — M1f S1f.10.19's T2, beside
//! [`price_list`](price_list.rs). It measures; no route reads any of it,
//! and every route runs as it does.
//!
//! ```sh
//! cargo run --release -p ein-infer --example early_stop -- \
//!     [--out DIR] [--jobs J] [--alive-orders] [--cores] [FILE…]
//! ```
//!
//! Every corpus file and register witness that loads and **searches** (the
//! determinism sweeps' one definition), or the `FILE`s given. For each, each
//! of the six questions `ein solve` asks, describe, `--count`, `--single`,
//! `-n 1`, `--minimal` and `--maximal`, is asked once at the default budget
//! with every node handed to a reader (`ein_infer::events::Watch`, what
//! `--dump-lattice` reads). Every distinct node visited is then saturated
//! again, in process, through the node function's own fork
//! (`ein_infer::region::Nodes::with_fixpoint_traced`):
//!
//! - **a dead node** to quiescence and stopped at its first contradiction
//!   (`ein_infer::saturator::stopping_at_first_contradiction`), under each
//!   order: FIFO, and the two fail-first agendas of
//!   `ein_infer::saturator::fail_first`, `pairs` and `negations`. Each run
//!   gives its firings (every application the saturator returns, a redundant
//!   one and the mirror's included) and its facts beyond root. The stop is
//!   also read off the quiescent run's own trace, the first point where the
//!   facts derived so far hold a contradiction, and the two readings must
//!   agree. At the stop, the node's closure, what it denies, what it owes
//!   and, with `--cores`, its core are read against the quiescent node's,
//!   and a fail-first stop's against FIFO's; each fail-first fixpoint is read
//!   against FIFO's the same way, and must hold the same facts;
//! - **an alive node** to quiescence under FIFO, for its firings (its facts
//!   are the watch's); with `--alive-orders` under each fail-first agenda
//!   too, with whether its fixpoint is FIFO's.
//!
//! The whole node the tree's order reads (`explore::Space::charge_whole_node`,
//! a dead node whose commitment is every candidate and whose closure the
//! watch is handed empty) is saturated to quiescence under each order and
//! counted, and not stopped: its reader needs every application.
//!
//! It writes, under `--out` (default `build/early_stop` under the repo
//! root): `early_stop.jsonl`, a line a program (the distinct nodes, the
//! checks, what a stopped node reports, and per question its cell), and
//! `verdicts.json` and `early_stop.md`, the corpus's tables and the verdicts
//! by S1f.10.19's rule for *go*. Progress is a line a program on stderr.
//! Every count is the same on any machine and at any `--jobs`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use ein_core::{FactId, Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, Visited, Watch};
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::firing::Firing;
use ein_infer::region::{Cores, Nodes};
use ein_infer::saturator::{FailFirst, fail_first, stopping_at_first_contradiction};
use ein_ir::{Ast, load, parse};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use serde_json::{Value, json};

// What *a program searches* means, by the determinism sweeps' one
// definition.
#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

/// The orders: FIFO, as ruled, and the two fail-first agendas.
const ORDERS: [(&str, Option<FailFirst>); 3] = [
    ("fifo", None),
    ("pairs", Some(FailFirst::Pairs)),
    ("negations", Some(FailFirst::Negations)),
];

/// The six questions `ein solve` asks: a key, the flags, the mode.
fn questions() -> [(&'static str, &'static str, Mode); 6] {
    let m = |which, ask| Mode { which, ask };
    [
        ("describe", "describe", m(Which::All, Ask::Describe)),
        ("count", "--count", m(Which::All, Ask::Count)),
        ("single", "--single", m(Which::All, Ask::Single)),
        ("n1", "-n 1", m(Which::All, Ask::Find(1))),
        ("minimal", "--minimal", m(Which::Minimal, Ask::Describe)),
        ("maximal", "--maximal", m(Which::Maximal, Ask::Describe)),
    ]
}

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

// ── A question, asked and watched ───────────────────────────────────────

/// One node a route visited.
#[derive(Clone, Debug)]
struct Seen {
    set: Box<[u32]>,
    dead: bool,
    /// The whole node the tree's order reads.
    whole: bool,
    facts: u64,
}

struct Collect {
    index: Arc<FxHashMap<String, u32>>,
    n: usize,
    kept: Arc<Mutex<(Vec<Seen>, u64)>>,
}

impl Watch for Collect {
    fn node(&mut self, v: &Visited<'_>) {
        let mut miss = 0;
        let mut set: Vec<u32> = v
            .commitment
            .iter()
            .filter_map(|t| {
                let b = self.index.get(t).copied();
                miss += u64::from(b.is_none());
                b
            })
            .collect();
        set.sort_unstable();
        set.dedup();
        let dead = v.kind == "dead";
        let whole = dead && self.n > 0 && set.len() == self.n && v.closure.is_empty();
        let mut kept = self.kept.lock().expect("one thread watches");
        kept.1 += miss;
        kept.0.push(Seen {
            set: set.into_boxed_slice(),
            dead,
            whole,
            facts: v.facts,
        });
    }
}

/// What one run answered and what it cost, and the nodes it visited.
struct Asked {
    saturations: u64,
    facts: u64,
    stopped: Option<Stopped>,
    exhausted: bool,
    route: String,
    nodes: Vec<Seen>,
    unmapped: u64,
    universe_same: bool,
}

fn ask(path: &Path, key: &str, mode: Mode, universe: &Universe) -> Result<Asked, String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    // As `ein solve` asks it: no core of a visited node, `N` not taken under
    // `--count`.
    let opts = explore::Options {
        cores: Cores::READ_OUT,
        nogoods: key != "count",
        ..explore::Options::new(mode)
    };
    let kept = Arc::new(Mutex::new((Vec::new(), 0)));
    let mut events = Events::off().watched(Box::new(Collect {
        index: universe.index.clone(),
        n: universe.texts.len(),
        kept: kept.clone(),
    }));
    let a = explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
        .map_err(|e| e.to_string())?;
    drop(events);
    let universe_same = a.candidates.len() == universe.texts.len()
        && a.candidates
            .iter()
            .zip(&universe.texts)
            .all(|(c, t)| c.text == *t);
    let (nodes, unmapped) = std::mem::take(&mut *kept.lock().expect("the run is over"));
    Ok(Asked {
        saturations: a.saturations,
        facts: a.facts,
        stopped: a.stopped.clone(),
        exhausted: a.exhausted,
        route: format!("{:?}", a.route),
        nodes,
        unmapped,
        universe_same,
    })
}

fn stopped(s: &Option<Stopped>) -> Value {
    match s {
        None => Value::Null,
        Some(Stopped::Budget { .. }) => json!("budget"),
        Some(Stopped::Limit { .. }) => json!("limit"),
        Some(Stopped::Deadline) => json!("deadline"),
        Some(Stopped::NoSearch) => json!("no-search"),
    }
}

// ── A node, saturated again ─────────────────────────────────────────────

/// The program's universe, as every route builds it.
struct Universe {
    texts: Vec<String>,
    index: Arc<FxHashMap<String, u32>>,
}

/// What a node reports, read off a fixpoint or a stopped one.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
struct Report {
    closure: Vec<u32>,
    denied: Vec<u32>,
    owes: u32,
    owed: Vec<String>,
    core: Option<Vec<String>>,
}

/// One saturation of a node under one order, to quiescence or stopped.
#[derive(Clone, Default, Debug)]
struct Sat {
    firings: u64,
    facts: u64,
    /// The fixpoint's facts beyond root, hashed: two equal hashes are one
    /// state.
    state: u64,
    dead: bool,
}

/// A dead node under one order.
#[derive(Clone, Default, Debug)]
struct DeadRun {
    quiet: Sat,
    stop: Sat,
    /// The first contradiction: its shape, and what completes it, the rule
    /// of the firing or `(commitment)`.
    kind: String,
    by: String,
    report_quiet: Report,
    report_stop: Report,
}

fn hash_of(mut v: Vec<FactId>) -> u64 {
    v.sort_unstable();
    let mut h = FxHasher::default();
    v.hash(&mut h);
    h.finish()
}

/// What a node's reading needs beside the nodes themselves.
struct Plain<'a> {
    ast: &'a Ast,
    memo: SharedMemo,
    root: FxHashSet<FactId>,
    facts: Vec<FactId>,
    cores: bool,
}

struct Reader<'a> {
    nodes: Nodes<'a>,
    plain: Plain<'a>,
}

impl Plain<'_> {
    fn report(&self, kb: &Kb, terms: &mut Terms) -> Report {
        let mut closure = Vec::new();
        let mut denied = Vec::new();
        for (b, &c) in self.facts.iter().enumerate() {
            if kb.contains(c) {
                closure.push(b as u32);
            }
            if kb.is_negated(c) {
                denied.push(b as u32);
            }
        }
        let owes =
            ein_infer::obligations::tally(kb, terms, self.ast, &self.memo, &mut Events::off())
                .expect("the plans compiled at root compile");
        let mut owed: Vec<String> = owes
            .instances()
            .iter()
            .map(|o| {
                let act: Vec<String> = o.activator.iter().map(|&v| terms.display(v)).collect();
                let bind: Vec<String> = o
                    .bindings
                    .iter()
                    .map(|&(k, v)| format!("{}={}", terms.sym(k), terms.display(v)))
                    .collect();
                format!(
                    "{} [{}] {{{}}}",
                    terms.sym(o.rule),
                    act.join(" "),
                    bind.join(" ")
                )
            })
            .collect();
        owed.sort();
        let core = self.cores.then(|| {
            let cs = ein_infer::contradiction::detect(kb, terms);
            if cs.is_empty() {
                return Vec::new();
            }
            let e = ein_infer::explain::contradiction_frontier(
                kb,
                terms,
                &cs,
                &ein_infer::explain::ExplanationBudget::default(),
            );
            let mut v: Vec<String> = e
                .frontier
                .iter()
                .map(|&f| ein_infer::events::sexpr(terms, f))
                .collect();
            v.sort();
            v
        });
        Report {
            closure,
            denied,
            owes: owes.total() as u32,
            owed,
            core,
        }
    }

    /// The quiescent run's trace, read: firings, facts, state, and the first
    /// contradiction, the point where the facts written and derived so far
    /// first hold one.
    fn read_trace(
        &self,
        kb: &Kb,
        terms: &mut Terms,
        written: &[FactId],
        firings: &[Firing],
        read_report: bool,
    ) -> Quiet {
        let root = &self.root;
        // When each fact beyond root came: 0 for the commitment's, the
        // firing's place (from 1) for a derived one.
        let mut time: FxHashMap<FactId, u64> = FxHashMap::default();
        for &h in written {
            if !root.contains(&h) {
                time.entry(h).or_insert(0);
            }
        }
        for (i, f) in firings.iter().enumerate() {
            if f.redundant {
                continue;
            }
            for &d in f.derived.iter() {
                if !root.contains(&d) {
                    time.entry(d).or_insert(i as u64 + 1);
                }
            }
        }
        let beyond: Vec<FactId> = kb.facts().filter(|f| !root.contains(f)).collect();
        let untimed = beyond.iter().filter(|f| !time.contains_key(f)).count() as u64;
        let cs = ein_infer::contradiction::detect(kb, terms);
        let mut first: Option<(u64, usize)> = None;
        for (k, c) in cs.iter().enumerate() {
            let t = c
                .sides()
                .iter()
                .map(|s| {
                    if root.contains(s) {
                        0
                    } else {
                        time.get(s).copied().unwrap_or(u64::MAX)
                    }
                })
                .max()
                .unwrap_or(0);
            if first.is_none_or(|(ft, _)| t < ft) {
                first = Some((t, k));
            }
        }
        let (offline, kind, by) = match first {
            None => ((0, 0), String::new(), String::new()),
            Some((t, k)) => {
                // determinism-ok: a count, the same in every order.
                let at = time.values().filter(|&&x| x <= t).count() as u64;
                let by = if t == 0 {
                    "(commitment)".to_string()
                } else if t == u64::MAX {
                    "(untimed)".to_string()
                } else {
                    terms.sym(firings[t as usize - 1].rule).to_string()
                };
                ((t, at), cs[k].kind.as_str().to_string(), by)
            }
        };
        let report = if read_report {
            self.report(kb, terms)
        } else {
            Report::default()
        };
        Quiet {
            sat: Sat {
                firings: firings.len() as u64,
                facts: beyond.len() as u64,
                state: hash_of(beyond),
                dead: !cs.is_empty(),
            },
            offline,
            kind,
            by,
            untimed,
            report,
        }
    }
}

/// A node to quiescence, read.
struct Quiet {
    sat: Sat,
    offline: (u64, u64),
    kind: String,
    by: String,
    untimed: u64,
    report: Report,
}

impl Reader<'_> {
    /// The node of `set` under `order`, to quiescence.
    fn quiet(
        &mut self,
        set: &[u32],
        order: Option<FailFirst>,
        read_report: bool,
    ) -> Result<Quiet, String> {
        let Reader { nodes, plain } = self;
        let written: Vec<FactId> = set.iter().map(|&b| plain.facts[b as usize]).collect();
        let mut run = || {
            nodes.with_fixpoint_traced(&written, |kb, terms, firings: &[Firing]| {
                plain.read_trace(kb, terms, &written, firings, read_report)
            })
        };
        let out = match order {
            None => run(),
            Some(o) => fail_first(o, run),
        };
        out.map_err(|e| e.to_string())
    }

    /// The node of `set` under `order`, stopped at its first contradiction.
    fn stopped(&mut self, set: &[u32], order: Option<FailFirst>) -> Result<(Sat, Report), String> {
        let Reader { nodes, plain } = self;
        let written: Vec<FactId> = set.iter().map(|&b| plain.facts[b as usize]).collect();
        let mut run = || {
            nodes.with_fixpoint_traced(&written, |kb, terms, firings: &[Firing]| {
                let beyond: Vec<FactId> = kb.facts().filter(|f| !plain.root.contains(f)).collect();
                let report = plain.report(kb, terms);
                let dead = ein_infer::contradiction::has_contradiction(kb, terms);
                (
                    Sat {
                        firings: firings.len() as u64,
                        facts: beyond.len() as u64,
                        state: hash_of(beyond),
                        dead,
                    },
                    report,
                )
            })
        };
        let out = stopping_at_first_contradiction(|| match order {
            None => run(),
            Some(o) => fail_first(o, run),
        });
        out.map_err(|e| e.to_string())
    }
}

// ── A program ───────────────────────────────────────────────────────────

#[derive(Default, Clone)]
struct Cell {
    key: &'static str,
    saturations: u64,
    facts: u64,
    stopped: Value,
    exhausted: bool,
    route: String,
    visited: u64,
    dead: u64,
    whole: u64,
    alive: u64,
    facts_alive: u64,
    facts_dead: u64,
    facts_whole: u64,
    /// Per order: firings of the alive nodes, of the whole node, and of the
    /// dead nodes to quiescence and stopped; facts of the dead nodes stopped.
    firings_alive: [u64; 3],
    firings_whole: [u64; 3],
    dead_quiet: [(u64, u64); 3],
    dead_stop: [(u64, u64); 3],
    /// Dead nodes whose first contradiction is the commitment's own.
    zero_firing: u64,
    /// Dead nodes whose watched facts are not their FIFO fixpoint's.
    facts_mismatch: u64,
}

impl Cell {
    fn json(&self) -> Value {
        let per = |a: &[u64; 3]| json!({ORDERS[0].0: a[0], ORDERS[1].0: a[1], ORDERS[2].0: a[2]});
        let pair = |a: &[(u64, u64); 3]| json!({ORDERS[0].0: [a[0].0, a[0].1], ORDERS[1].0: [a[1].0, a[1].1], ORDERS[2].0: [a[2].0, a[2].1]});
        json!({
            "saturations": self.saturations, "facts": self.facts, "stopped": self.stopped,
            "exhausted": self.exhausted, "route": self.route,
            "visited": self.visited, "dead": self.dead, "whole": self.whole, "alive": self.alive,
            "facts_alive": self.facts_alive, "facts_dead": self.facts_dead,
            "facts_whole": self.facts_whole,
            "charged": self.facts.saturating_sub(self.facts_alive + self.facts_dead + self.facts_whole),
            "firings_alive": per(&self.firings_alive), "firings_whole": per(&self.firings_whole),
            "dead_quiet": pair(&self.dead_quiet), "dead_stop": pair(&self.dead_stop),
            "zero_firing": self.zero_firing, "facts_mismatch": self.facts_mismatch,
        })
    }

    /// Today: facts and firings, every node to quiescence under FIFO.
    fn today(&self) -> (u64, u64) {
        (
            self.facts,
            self.firings_alive[0] + self.firings_whole[0] + self.dead_quiet[0].0,
        )
    }

    /// With the stop under order `o`, where it is applied.
    fn with(&self, o: usize, applied: bool) -> (u64, u64) {
        if !applied {
            return self.today();
        }
        let saved = self.facts_dead.saturating_sub(self.dead_stop[o].1);
        (
            self.facts - saved,
            self.firings_alive[o] + self.firings_whole[o] + self.dead_stop[o].0,
        )
    }
}

struct Program {
    rel: String,
    monotone: bool,
    cells: Vec<Cell>,
    line: Value,
}

fn measure(path: &Path, rel: &str, alive_orders: bool, cores: bool) -> Result<Program, String> {
    // The universe and the reader, on one load.
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    let root_opts = explore::Options {
        no_search: true,
        cores: Cores::READ_OUT,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let a = explore::run(&mut kb, &mut terms, &ast, &root_opts).map_err(|e| e.to_string())?;
    let monotone = a.monotonicity.is_monotone();
    let texts: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let universe = Universe {
        index: Arc::new(
            texts
                .iter()
                .enumerate()
                .map(|(i, t)| (t.clone(), i as u32))
                .collect(),
        ),
        texts,
    };
    let facts: Vec<FactId> = a.candidates.iter().map(|c| c.fact).collect();
    let n = facts.len();

    // Each question, watched.
    let mut asked: Vec<(&'static str, Asked)> = Vec::new();
    for (key, _, mode) in questions() {
        asked.push((key, ask(path, key, mode, &universe)?));
    }

    let candidates = a.candidates.clone();
    let mut nodes =
        Nodes::new(&mut kb, &mut terms, &ast, &candidates).map_err(|e| e.to_string())?;
    let root: FxHashSet<FactId> = nodes
        .with_fixpoint(&[], |kb, _| kb.facts().collect())
        .map_err(|e| e.to_string())?;
    let mut reader = Reader {
        nodes,
        plain: Plain {
            ast: &ast,
            memo: SharedMemo::default(),
            root,
            facts,
            cores,
        },
    };

    // The distinct nodes.
    let mut dead_sets: Vec<Box<[u32]>> = Vec::new();
    let mut whole_sets: Vec<Box<[u32]>> = Vec::new();
    let mut alive_sets: Vec<Box<[u32]>> = Vec::new();
    let mut met: FxHashSet<(Box<[u32]>, u8)> = FxHashSet::default();
    for (_, q) in &asked {
        for s in &q.nodes {
            let kind = if s.whole { 2 } else { u8::from(s.dead) };
            if met.insert((s.set.clone(), kind)) {
                match kind {
                    2 => whole_sets.push(s.set.clone()),
                    1 => dead_sets.push(s.set.clone()),
                    _ => alive_sets.push(s.set.clone()),
                }
            }
        }
    }

    // Dead nodes: every order, to quiescence and stopped.
    let mut dead: FxHashMap<Box<[u32]>, [DeadRun; 3]> = FxHashMap::default();
    let mut checks = BTreeMap::<String, u64>::new();
    let mut by: BTreeMap<String, u64> = BTreeMap::new();
    let mut kinds: BTreeMap<String, u64> = BTreeMap::new();
    let mut reports: BTreeMap<String, u64> = BTreeMap::new();
    for set in &dead_sets {
        let mut runs: [DeadRun; 3] = Default::default();
        for (o, (name, order)) in ORDERS.iter().enumerate() {
            let Quiet {
                sat: quiet,
                offline,
                kind,
                by: by_rule,
                untimed,
                report: report_quiet,
            } = reader.quiet(set, *order, true)?;
            let (stop, report_stop) = reader.stopped(set, *order)?;
            if !quiet.dead {
                *checks
                    .entry(format!("{name}: dead node alive"))
                    .or_default() += 1;
            }
            if !stop.dead {
                *checks
                    .entry(format!("{name}: stopped node alive"))
                    .or_default() += 1;
            }
            if offline != (stop.firings, stop.facts) {
                *checks
                    .entry(format!("{name}: stop by the hook ≠ stop off the trace"))
                    .or_default() += 1;
            }
            if untimed > 0 {
                *checks.entry(format!("{name}: facts untimed")).or_default() += untimed;
            }
            runs[o] = DeadRun {
                quiet,
                stop,
                kind,
                by: by_rule,
                report_quiet,
                report_stop,
            };
        }
        let fifo = runs[0].clone();
        *by.entry(fifo.by.clone()).or_default() += 1;
        *kinds.entry(fifo.kind.clone()).or_default() += 1;
        for (o, (name, _)) in ORDERS.iter().enumerate() {
            let r = &runs[o];
            if o > 0 {
                if r.quiet.state != fifo.quiet.state {
                    *checks.entry(format!("{name}: fixpoint moved")).or_default() += 1;
                }
                if r.quiet.firings != fifo.quiet.firings {
                    *checks
                        .entry(format!("{name}: firings to quiescence moved"))
                        .or_default() += 1;
                }
            }
            let q = &fifo.report_quiet;
            let s = &r.report_stop;
            let mut differ = |what: &str, moved: bool| {
                if moved {
                    *reports.entry(format!("{name}: {what}")).or_default() += 1;
                }
            };
            differ("facts", r.stop.facts != fifo.quiet.facts);
            differ("closure", s.closure != q.closure);
            differ("denied", s.denied != q.denied);
            differ("owes", s.owes != q.owes);
            differ("owed", s.owed != q.owed);
            if cores {
                differ("core", s.core != q.core);
            }
            // A fail-first stop against FIFO's: what the order moves of what
            // a stopped node reports; and the same node to quiescence under
            // the order against FIFO's fixpoint, which no stop is in.
            if o > 0 {
                let rq = &r.report_quiet;
                differ("quiescent: closure against FIFO's", rq.closure != q.closure);
                differ("quiescent: owed against FIFO's", rq.owed != q.owed);
                if cores {
                    differ("quiescent: core against FIFO's", rq.core != q.core);
                }
                let f = &fifo.report_stop;
                differ("facts against FIFO's stop", r.stop.facts != fifo.stop.facts);
                differ("closure against FIFO's stop", s.closure != f.closure);
                differ("denied against FIFO's stop", s.denied != f.denied);
                differ("owed against FIFO's stop", s.owed != f.owed);
                if cores {
                    differ("core against FIFO's stop", s.core != f.core);
                }
            }
        }
        dead.insert(set.clone(), runs);
    }

    // The whole node: every order, to quiescence.
    let mut whole: FxHashMap<Box<[u32]>, [u64; 3]> = FxHashMap::default();
    for set in &whole_sets {
        let mut f = [0u64; 3];
        for (o, (_, order)) in ORDERS.iter().enumerate() {
            f[o] = reader.quiet(set, *order, false)?.sat.firings;
        }
        whole.insert(set.clone(), f);
    }

    // Alive nodes: FIFO, and each order where asked.
    let mut alive: FxHashMap<Box<[u32]>, [u64; 3]> = FxHashMap::default();
    for set in &alive_sets {
        let fifo = reader.quiet(set, None, false)?.sat;
        let mut f = [fifo.firings; 3];
        if alive_orders {
            for (o, (name, order)) in ORDERS.iter().enumerate().skip(1) {
                let r = reader.quiet(set, *order, false)?.sat;
                f[o] = r.firings;
                if r.state != fifo.state {
                    *checks
                        .entry(format!("{name}: alive fixpoint moved"))
                        .or_default() += 1;
                }
                if r.dead {
                    *checks
                        .entry(format!("{name}: alive node dead"))
                        .or_default() += 1;
                }
            }
        }
        if fifo.dead {
            *checks
                .entry("fifo: alive node dead".to_string())
                .or_default() += 1;
        }
        alive.insert(set.clone(), f);
    }

    // The cells.
    let mut cells = Vec::new();
    let mut cell_json = serde_json::Map::new();
    for (key, q) in &asked {
        if !q.universe_same {
            *checks.entry("another universe".to_string()).or_default() += 1;
        }
        if q.unmapped > 0 {
            *checks
                .entry("texts no candidate has".to_string())
                .or_default() += q.unmapped;
        }
        let mut c = Cell {
            key,
            saturations: q.saturations,
            facts: q.facts,
            stopped: stopped(&q.stopped),
            exhausted: q.exhausted,
            route: q.route.clone(),
            ..Cell::default()
        };
        for s in &q.nodes {
            c.visited += 1;
            if s.whole {
                c.whole += 1;
                c.facts_whole += s.facts;
                for (sum, f) in c.firings_whole.iter_mut().zip(whole[&s.set]) {
                    *sum += f;
                }
            } else if s.dead {
                c.dead += 1;
                c.facts_dead += s.facts;
                let runs = &dead[&s.set];
                if runs[0].quiet.facts != s.facts {
                    c.facts_mismatch += 1;
                }
                if runs[0].stop.firings == 0 {
                    c.zero_firing += 1;
                }
                for (o, run) in runs.iter().enumerate() {
                    c.dead_quiet[o].0 += run.quiet.firings;
                    c.dead_quiet[o].1 += run.quiet.facts;
                    c.dead_stop[o].0 += run.stop.firings;
                    c.dead_stop[o].1 += run.stop.facts;
                }
            } else {
                c.alive += 1;
                c.facts_alive += s.facts;
                for (sum, f) in c.firings_alive.iter_mut().zip(alive[&s.set]) {
                    *sum += f;
                }
            }
        }
        if c.facts_mismatch > 0 {
            *checks
                .entry("dead node's watched facts ≠ its FIFO fixpoint's".to_string())
                .or_default() += c.facts_mismatch;
        }
        if c.visited != c.saturations {
            *checks
                .entry("nodes watched ≠ saturations".to_string())
                .or_default() += 1;
        }
        cell_json.insert(key.to_string(), c.json());
        cells.push(c);
    }
    let line = json!({
        "program": rel,
        "n": n,
        "monotone": monotone,
        "nodes": {"dead": dead_sets.len(), "whole": whole_sets.len(), "alive": alive_sets.len()},
        "checks": checks,
        "first_contradiction": {"kind": kinds, "by": by},
        "reports_moved": reports,
        "cells": cell_json,
    });
    Ok(Program {
        rel: rel.to_string(),
        monotone,
        cells,
        line,
    })
}

// ── The rule for *go* ───────────────────────────────────────────────────

/// The five comparisons: an order with the stop against today, and a
/// fail-first order with the stop against FIFO with it.
const COMPARISONS: [(&str, usize, Option<usize>); 5] = [
    ("fifo", 0, None),
    ("pairs", 1, None),
    ("negations", 2, None),
    ("pairs-vs-fifo", 1, Some(0)),
    ("negations-vs-fifo", 2, Some(0)),
];

struct Verdict {
    /// Per unit, facts then firings: the totals, base and with.
    totals: [(u64, u64); 2],
    lower: [u64; 2],
    losers: [Vec<(String, u64, u64)>; 2],
    go: [bool; 2],
}

fn verdict(
    programs: &[Program],
    q: usize,
    cmp: (usize, Option<usize>),
    unstopped: bool,
) -> Verdict {
    let (with, base) = cmp;
    let mut v = Verdict {
        totals: [(0, 0); 2],
        lower: [0; 2],
        losers: [Vec::new(), Vec::new()],
        go: [false; 2],
    };
    for p in programs {
        let c = &p.cells[q];
        if unstopped && !c.stopped.is_null() {
            continue;
        }
        let b = match base {
            None => c.today(),
            Some(o) => c.with(o, p.monotone),
        };
        let w = c.with(with, p.monotone);
        for (u, (bu, wu)) in [(b.0, w.0), (b.1, w.1)].into_iter().enumerate() {
            v.totals[u].0 += bu;
            v.totals[u].1 += wu;
            if wu < bu {
                v.lower[u] += 1;
            }
            let worse = if bu == 0 { wu > 0 } else { wu * 10 > bu * 11 };
            if worse {
                v.losers[u].push((p.rel.clone(), bu, wu));
            }
        }
    }
    for u in 0..2 {
        v.go[u] = v.totals[u].1 < v.totals[u].0 && v.losers[u].is_empty();
    }
    v
}

fn pct(part: u64, whole: u64) -> String {
    if whole == 0 {
        "–".to_string()
    } else {
        format!("{:.1} %", 100.0 * part as f64 / whole as f64)
    }
}

fn num(x: u64) -> String {
    let s = x.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push('\u{202f}');
        }
        out.push(ch);
    }
    out
}

fn report(programs: &[Program]) -> (String, Value) {
    let qs = questions();
    let mut md = String::new();
    let mut verdicts = serde_json::Map::new();
    let _ = writeln!(md, "# Early stop: the corpus's tables\n");
    let _ = writeln!(
        md,
        "{} programs that search, {} monotone. Units: saturations, facts beyond root, firings.\n",
        programs.len(),
        programs.iter().filter(|p| p.monotone).count()
    );
    let _ = writeln!(md, "## Per question, every program\n");
    let _ = writeln!(
        md,
        "| question | stopped | saturations | dead (share) | facts | of them dead nodes' | firings | of them dead nodes' |"
    );
    let _ = writeln!(md, "|---|---:|---:|---:|---:|---:|---:|---:|");
    for (qi, (key, flags, _)) in qs.iter().enumerate() {
        let (mut st, mut sat, mut dead, mut facts, mut fd, mut fir, mut fird) =
            (0, 0, 0, 0, 0, 0, 0);
        for p in programs {
            let c = &p.cells[qi];
            st += u64::from(!c.stopped.is_null());
            sat += c.saturations;
            dead += c.dead + c.whole;
            facts += c.facts;
            fd += c.facts_dead + c.facts_whole;
            let t = c.today();
            fir += t.1;
            fird += c.dead_quiet[0].0 + c.firings_whole[0];
        }
        let _ = writeln!(
            md,
            "| {flags} | {st} | {} | {} ({}) | {} | {} ({}) | {} | {} ({}) |",
            num(sat),
            num(dead),
            pct(dead, sat),
            num(facts),
            num(fd),
            pct(fd, facts),
            num(fir),
            num(fird),
            pct(fird, fir)
        );
        let _ = key;
    }
    for (scope, unstopped) in [("every cell", false), ("the cells no budget stopped", true)] {
        let _ = writeln!(md, "\n## The verdicts, over {scope}\n");
        let _ = writeln!(
            md,
            "A cell: the verdict; facts, total → with; firings, total → with; programs lower / higher by more than a tenth, facts then firings.\n"
        );
        let _ = writeln!(md, "| comparison | {} |", qs.map(|q| q.1).join(" | "));
        let _ = writeln!(md, "|---|{}", "---|".repeat(qs.len()));
        for (name, with, base) in COMPARISONS {
            let mut row = format!("| {name} |");
            for (qi, (key, _, _)) in qs.iter().enumerate() {
                let v = verdict(programs, qi, (with, base), unstopped);
                let go = v.go[0] && v.go[1];
                let _ = write!(
                    row,
                    " {} · {} → {} · {} → {} · {}/{} , {}/{} |",
                    if go { "**go**" } else { "no" },
                    num(v.totals[0].0),
                    num(v.totals[0].1),
                    num(v.totals[1].0),
                    num(v.totals[1].1),
                    v.lower[0],
                    v.losers[0].len(),
                    v.lower[1],
                    v.losers[1].len()
                );
                let losers = |u: usize| {
                    v.losers[u]
                        .iter()
                        .map(|(p, b, w)| json!({"program": p, "base": b, "with": w}))
                        .collect::<Vec<_>>()
                };
                verdicts.insert(
                    format!("{scope}/{name}/{key}"),
                    json!({
                        "go": go,
                        "facts": {"total": v.totals[0].0, "with": v.totals[0].1, "lower_on": v.lower[0], "losers": losers(0), "go": v.go[0]},
                        "firings": {"total": v.totals[1].0, "with": v.totals[1].1, "lower_on": v.lower[1], "losers": losers(1), "go": v.go[1]},
                    }),
                );
            }
            let _ = writeln!(md, "{row}");
        }
    }
    let _ = writeln!(md, "\n## The losers, named (every cell)\n");
    for (name, with, base) in COMPARISONS {
        for (qi, (_, flags, _)) in qs.iter().enumerate() {
            let v = verdict(programs, qi, (with, base), false);
            for (u, unit) in ["facts", "firings"].iter().enumerate() {
                for (p, b, w) in &v.losers[u] {
                    let _ = writeln!(md, "- {name}, {flags}, {unit}: `{p}` {b} → {w}");
                }
            }
        }
    }
    (md, Value::Object(verdicts))
}

fn main() {
    let mut out = repo_root().join("build/early_stop");
    let mut files: Vec<PathBuf> = Vec::new();
    let mut alive_orders = false;
    let mut cores = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            "--alive-orders" => alive_orders = true,
            "--cores" => cores = true,
            "--jobs" => {
                // The pool reads `EIN_TEST_JOBS`.
                let j = args.next().expect("--jobs J");
                // SAFETY: set before any thread is started.
                unsafe { std::env::set_var("EIN_TEST_JOBS", j) };
            }
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
                std::env::current_dir()
                    .expect("a working directory")
                    .join(p)
            }
        })
        .collect();
    let rel = |p: &Path| {
        p.strip_prefix(repo_root())
            .unwrap_or(p)
            .to_string_lossy()
            .to_string()
    };
    let results = ein_corpus::pool::map(
        "early_stop",
        &files,
        |p| (rel(p), "measure".to_string()),
        |p| {
            let r = rel(p);
            let (mut kb, mut terms, ast) = loaded(p)?;
            if !searching::searches(&ast, &mut terms, &mut kb) {
                return None;
            }
            let m = measure(p, &r, alive_orders, cores);
            match &m {
                Ok(m) => eprintln!(
                    "{r}: {}",
                    m.cells
                        .iter()
                        .map(|c| format!("{} {}/{}", c.key, c.dead + c.whole, c.saturations))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Err(e) => eprintln!("{r}: error: {e}"),
            }
            Some((r, m))
        },
    );
    std::fs::create_dir_all(&out).expect("the output directory");
    let mut programs = Vec::new();
    let mut lines = String::new();
    for (r, m) in results.into_iter().flatten() {
        match m {
            Ok(p) => {
                lines.push_str(&p.line.to_string());
                lines.push('\n');
                programs.push(p);
            }
            Err(e) => {
                lines.push_str(&json!({"program": r, "error": e}).to_string());
                lines.push('\n');
            }
        }
    }
    std::fs::write(out.join("early_stop.jsonl"), lines).expect("early_stop.jsonl");
    let (md, verdicts) = report(&programs);
    std::fs::write(
        out.join("verdicts.json"),
        serde_json::to_string_pretty(&verdicts).expect("json"),
    )
    .expect("verdicts.json");
    std::fs::write(out.join("early_stop.md"), &md).expect("early_stop.md");
    print!("{md}");
}
