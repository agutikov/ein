//! The solution region, by brute force — M1e2
//! [S1e2.1a.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//!
//! The answer to a program is `R = { C : C alive and discharged }`, a region
//! of its hypothesis lattice
//! ([Q-M1e2.7](../../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).
//! The search is what is being checked, so this computes `R` **without it**:
//! every subset of the root candidate list, each one a fork of root with the
//! subset written in and saturated to quiescence, classified by what the
//! program's rules say and by nothing the search writes.
//!
//! ### The cube
//!
//! Without merging, the lattice over `n` candidates is the `n`-cube: a node is
//! an `n`-bit mask, bit `i` the `i`-th candidate in **rendered-text order**
//! (so the cube does not depend on interner ids), and an edge adds or removes
//! one fact. Each node carries two independent bits, [`DEAD`] and [`OWING`],
//! and the owing bit is read on dead nodes too — the minimal discharging sets
//! `E` cannot be computed without it, and `owes.root` reports 0 on a dead root
//! whatever it owes. Everything in [`Topology`] is computed over that status
//! array, never by re-saturating, and never by assuming `R` has no holes.
//!
//! ### What a node is
//!
//! **The fork route**: root's fixpoint, the commitment's facts written with
//! hypothesis provenance, the saturation resumed and run to quiescence —
//! never stopped at the first contradiction, so a dead node's debts are its
//! fixpoint's. That is the engine's own notion of a commitment, minus every
//! search-time write: no lookahead, no forced positive, no kill cache, no
//! writeback, no learned clause. Where adding a fact can take a conclusion
//! away the from-scratch route can disagree with it (W8), and which one `R` is
//! over is
//! [P1e2.1b](../../../../plans/m1e2_correctness/p1e2.1b_the_non_monotone_lattice/README.md)'s
//! question. [`crate::monotone`] says whether a program can.
//!
//! ### The universe
//!
//! The generator's list at root's fixpoint, **config-free**
//! ([`crate::hypgen::generate_config_free`]): the lookahead is a config switch,
//! so the candidates it would have dropped are nodes of the lattice. Today's
//! `alive₀` is taken after the forced-positive cascade and through the
//! lookahead, so the two can differ, and the caller reports where they do.
//!
//! ### What it costs
//!
//! `2ⁿ` saturations, which is why [`DEFAULT_LIMIT`] exists and why the caller
//! is told the projected cost before a large run ([`Projection`]). The
//! topology is `O(n · 2ⁿ)` bit work, except the maximal sub-cubes of a region
//! **with** holes, which have no shortcut and are skipped past a work bound
//! the record says it hit.

use std::time::{Duration, Instant};

use ein_core::prov::NafArg;
use ein_core::{FactId, Kb, Prov, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::SharedMemo;
use crate::events::{Events, sexpr};
use crate::hypgen::HypGenStats;
use crate::monotone::Monotonicity;
use crate::saturator::{Saturator, Session, Snapshot};
use crate::solve::SolveError;

/// The status bit of a node that derives `(false)`.
pub const DEAD: u8 = 1;
/// The status bit of a node with an undischarged obligation.
pub const OWING: u8 = 2;

/// `n` past which the oracle refuses unless asked — the user's ruling of
/// 2026-10-15: *"we can safely limit n with 16 by default"*.
pub const DEFAULT_LIMIT: u32 = 16;
/// From here a raised limit warns on one line.
pub const WARN_FROM: usize = 17;
/// From here a raised limit warns prominently, with the projected cost.
pub const LOUD_FROM: usize = 20;
/// How many nodes, spread over the cube, are timed before a large run is
/// projected.
pub const PROBE_NODES: usize = 64;
/// The interval search on a region with holes stops past this many subset
/// tests, and the topology records that it did.
const SUBCUBE_WORK: u64 = 50_000_000;

/// One coordinate of the cube.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub fact: FactId,
    pub text: String,
}

/// What one node is, beyond its two bits.
#[derive(Clone, Default, Debug)]
pub struct Node {
    /// How many obligation instances its fixpoint leaves undischarged.
    pub owes: u32,
    /// Those instances, rendered through their `:why`.
    pub owed: Vec<String>,
    /// The unsat core of a dead node — the search's own reading,
    /// [`crate::explain::smallest_contradiction_frontier`].
    pub core: Vec<String>,
    /// The rules whose firing produced the contradiction the core explains.
    pub refuted_by: Vec<String>,
    /// The `absent` premises its derivation of the contradiction rests on —
    /// where a hole's failure is repairable.
    pub absent_premises: Vec<String>,
    /// The smallest mask whose fixpoint is this node's fact set.
    pub class: u64,
    /// The candidates its fixpoint holds — the node saturation *moves it to*
    /// inside the cube. Always a superset of the node's own mask; equal to it
    /// when saturation derives no candidate, which is the cube with no rule
    /// that writes a hypothesised fact.
    pub closure: u64,
    /// The candidates whose `(not h)` its fixpoint holds: adding one of them
    /// is a pair contradiction one edge up.
    pub excluded: u64,
    /// Facts its fixpoint holds beyond root's that are **not** candidates —
    /// what the node is that its coordinates do not say.
    pub beyond: u32,
    /// Of [`Self::beyond`], the positive facts over a relation the candidate
    /// list proposes: facts a generator could have proposed and root's did
    /// not.
    pub beyond_hypothesised: u32,
}

/// What the oracle measured about itself.
#[derive(Clone, Copy, Default, Debug)]
pub struct Cost {
    pub root: Duration,
    pub nodes: Duration,
    pub topology: Duration,
}

/// The caller's warning hook for a raised limit: what `2ⁿ` will cost.
#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub n: usize,
    pub nodes: u64,
    pub per_node: Duration,
    pub projected: Duration,
}

/// How the oracle is asked.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// The largest `n` it runs.
    pub limit: u32,
    /// Keep every node's state (its facts beyond root's) as text.
    pub keep_states: bool,
    /// Threads. `1` is the sequential oracle; `N` gives the same record byte
    /// for byte (`region::jobs_change_nothing`), because a worker that would
    /// have to number a new fact hands its node back, and every new fact is
    /// therefore numbered by the calling thread in the order `1` numbers it.
    pub jobs: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            limit: DEFAULT_LIMIT,
            keep_states: false,
            jobs: 1,
        }
    }
}

/// Nodes per fan-out.
const BATCH: usize = 1024;

/// The whole cube, classified.
#[derive(Clone, Debug)]
pub struct Region {
    pub candidates: Vec<Candidate>,
    /// [`DEAD`] / [`OWING`] per mask.
    pub status: Vec<u8>,
    pub nodes: Vec<Node>,
    /// Root's own fixpoint derives `(false)`.
    pub root_dead: bool,
    pub monotonicity: Monotonicity,
    pub topology: Topology,
    pub cost: Cost,
    /// Each node's state beyond root's, sorted by text — only when
    /// [`Options::keep_states`].
    pub states: Option<Vec<Vec<String>>>,
}

impl Region {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// `{a, b}` or `∅`, in candidate order.
    pub fn name(&self, mask: u64) -> String {
        if mask == 0 {
            return "∅".to_string();
        }
        let parts: Vec<&str> = (0..self.n())
            .filter(|i| mask >> i & 1 == 1)
            .map(|i| self.candidates[i].text.as_str())
            .collect();
        format!("{{{}}}", parts.join(", "))
    }

    /// Alive and discharged.
    pub fn in_r(&self, mask: u64) -> bool {
        self.status[mask as usize] == 0
    }
}

/// What the oracle did.
#[derive(Clone, Debug)]
pub enum Outcome {
    Region(Box<Region>),
    /// `n` is over the limit: nothing was enumerated, and the static check
    /// still ran.
    Refused {
        n: usize,
        limit: u32,
        candidates: Vec<Candidate>,
        monotonicity: Monotonicity,
    },
}

/// Compute the region of `root`, a **freshly loaded, unsaturated** program
/// that nothing else will use — the oracle's own load, so that its interning
/// cannot renumber anyone else's facts.
///
/// `on_projection` is called once, before the run, when `n ≥ WARN_FROM`: it is
/// how the caller prints the raised limit's warning with a measured cost
/// rather than a guess — [`PROBE_NODES`] nodes spread over every layer, each
/// saturated and discarded, projected at one job.
pub fn compute(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    on_projection: &mut dyn FnMut(&Projection),
) -> Result<Outcome, SolveError> {
    let t_root = Instant::now();
    let Prepared {
        snapshot,
        root_dead,
        candidates,
        hypothesised,
        monotonicity,
        memo,
    } = prepare(root, terms, ast)?;
    let n = candidates.len();

    if n > opts.limit as usize || n >= 64 {
        return Ok(Outcome::Refused {
            n,
            limit: opts.limit,
            candidates,
            monotonicity,
        });
    }
    let cost_root = t_root.elapsed();

    let axes = Axes::new(root, &candidates, hypothesised, opts.keep_states);
    let root = root.sealed();
    let size = 1usize << n;
    let mut status = vec![0u8; size];
    let mut nodes = vec![Node::default(); size];
    let mut states = opts.keep_states.then(|| vec![Vec::new(); size]);
    let mut classes: FxHashMap<Box<[FactId]>, u64> = FxHashMap::default();

    // Layer order, as the stage asks: the probe's first nodes are then the
    // small commitments, and every node is independent of the order anyway.
    let mut order: Vec<u64> = (0..size as u64).collect();
    order.sort_by_key(|m| (m.count_ones(), *m));

    let facts_of = |mask: u64| -> Vec<FactId> {
        (0..n)
            .filter(|b| mask >> b & 1 == 1)
            .map(|b| candidates[b].fact)
            .collect()
    };
    let pool = pool_of(opts.jobs);

    // The projection, before the run: [`PROBE_NODES`] nodes spread evenly over
    // the masks, so over every layer, each saturated and discarded. It was the
    // first 64 nodes in layer order until S1e2.1a.8 — the cheapest commitments
    // of the cube — and `branching/10` (n = 18) projected 22.9 s for a run
    // still going after 11 minutes on 8 jobs.
    if n >= WARN_FROM {
        let t_probe = Instant::now();
        for i in 0..PROBE_NODES as u64 {
            let mask =
                ((size as u64 - 1) as u128 * i as u128 / (PROBE_NODES as u64 - 1) as u128) as u64;
            node(root, &snapshot, terms, ast, &memo, &facts_of(mask), &axes)?;
        }
        let per_node = t_probe.elapsed() / PROBE_NODES as u32;
        on_projection(&Projection {
            n,
            nodes: size as u64,
            per_node,
            // At one job: a worker hands back every node that would number a
            // new fact, so `--jobs` divides it by less than the jobs —
            // `square-bwd/houses` (n = 20) is 35 s here and 11 s at 16.
            projected: per_node * size as u32,
        });
    }

    let t_nodes = Instant::now();
    let mut start = 0usize;
    while start < size {
        let batch = &order[start..(start + BATCH).min(size)];
        start += batch.len();
        let sets: Vec<Vec<FactId>> = batch.iter().map(|&m| facts_of(m)).collect();
        let outs = fan_out(
            pool.as_ref(),
            root,
            &snapshot,
            terms,
            ast,
            &memo,
            &axes,
            &sets,
        )?;
        // The commit, in layer order.
        for (out, &mask) in outs.into_iter().zip(batch) {
            let class = *classes.entry(out.state).or_insert(mask);
            let m = mask as usize;
            status[m] = (if out.dead { DEAD } else { 0 }) | (if out.owes > 0 { OWING } else { 0 });
            nodes[m] = Node { class, ..out.node };
            if let Some(st) = states.as_mut() {
                st[m] = out.delta;
            }
        }
    }
    let cost_nodes = t_nodes.elapsed();

    let t_topo = Instant::now();
    let topology = Topology::of(n, &status);
    let cost_topology = t_topo.elapsed();

    Ok(Outcome::Region(Box::new(Region {
        candidates,
        status,
        nodes,
        root_dead,
        monotonicity,
        topology,
        cost: Cost {
            root: cost_root,
            nodes: cost_nodes,
            topology: cost_topology,
        },
        states,
    })))
}

/// What both routes start from: root's fixpoint, the universe, and the
/// static check. [`compute`] (the oracle, route 2) and
/// [`crate::downset::compute`] (route 1) read every node against it.
pub(crate) struct Prepared {
    pub snapshot: Snapshot,
    /// Root's own fixpoint derives `(false)`.
    pub root_dead: bool,
    /// The universe, config-free, in rendered-text order.
    pub candidates: Vec<Candidate>,
    /// The candidates' relations — `H`.
    pub hypothesised: FxHashSet<ein_core::Symbol>,
    pub monotonicity: Monotonicity,
    pub memo: SharedMemo,
}

/// Saturate root, take the universe at its fixpoint, and classify the plans.
/// Root is written by its own saturation and by nothing else.
pub(crate) fn prepare(root: &mut Kb, terms: &mut Terms, ast: &Ast) -> Result<Prepared, SolveError> {
    let memo = SharedMemo::default();
    let mut off = Events::off();

    // Root's fixpoint, and the snapshot every node resumes from.
    let snapshot = {
        let mut s = Session {
            kb: root,
            terms,
            ast,
            events: &mut off,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s)?;
        sat.saturate(&mut s, None, &mut |_| {})?;
        sat.snapshot(s.kb)
    };
    let root_dead = crate::contradiction::has_contradiction(root, terms);

    // The universe, config-free, in rendered-text order.
    let mut proposed: Vec<FactId> = Vec::new();
    {
        let mut s = Session {
            kb: root,
            terms,
            ast,
            events: &mut off,
            memo: memo.clone(),
        };
        let mut stats = HypGenStats::new();
        crate::hypgen::generate_config_free(&mut s, &mut stats, &mut |f| {
            proposed.push(f);
            std::ops::ControlFlow::Continue(())
        })?;
    }
    let mut candidates: Vec<Candidate> = proposed
        .into_iter()
        .map(|fact| Candidate {
            fact,
            text: sexpr(terms, fact),
        })
        .collect();
    candidates.sort_by(|a, b| a.text.cmp(&b.text));
    candidates.dedup_by(|a, b| a.fact == b.fact);

    // The static check, over root's plans. Root's tally is what compiles the
    // obligation plans; a node can only compile one root lacks through an
    // activator a commitment derives, and that is a site of its own.
    crate::obligations::tally(root, terms, ast, &memo, &mut off)?;
    let hypothesised: FxHashSet<_> = candidates
        .iter()
        .map(|c| terms.facts.get(c.fact).0)
        .collect();
    let monotonicity = crate::monotone::classify(
        root,
        terms,
        &crate::monotone::plans_of(&memo),
        &hypothesised,
    );
    Ok(Prepared {
        snapshot,
        root_dead,
        candidates,
        hypothesised,
        monotonicity,
        memo,
    })
}

/// What a node's fixpoint is read against: root's facts and the cube's axes.
pub(crate) struct Axes {
    root_facts: FxHashSet<FactId>,
    /// The candidates in bit order.
    candidates: Vec<FactId>,
    is_candidate: FxHashSet<FactId>,
    /// The candidates' relations.
    hypothesised: FxHashSet<ein_core::Symbol>,
    /// Render each node's facts beyond root's — [`Options::keep_states`].
    keep_states: bool,
}

impl Axes {
    pub(crate) fn new(
        root: &Kb,
        candidates: &[Candidate],
        hypothesised: FxHashSet<ein_core::Symbol>,
        keep_states: bool,
    ) -> Axes {
        Axes {
            root_facts: root.facts().collect(),
            candidates: candidates.iter().map(|c| c.fact).collect(),
            is_candidate: candidates.iter().map(|c| c.fact).collect(),
            hypothesised,
            keep_states,
        }
    }
}

/// One node's fixpoint, read.
pub(crate) struct NodeOut {
    pub dead: bool,
    pub owes: u32,
    /// Its two masks are set only while `n < 64`; [`Self::closure`] is the
    /// first of them at any `n`.
    pub node: Node,
    /// Its fixpoint's facts **beyond root's**, sorted by id — the state's key.
    /// Every fixpoint holds root's, since a fork only adds, so two keys are
    /// equal exactly when the two states are.
    pub state: Box<[FactId]>,
    /// The candidates its fixpoint holds, as bit indices.
    pub closure: Vec<u32>,
    /// [`Self::state`] rendered, only under [`Axes::keep_states`].
    pub delta: Vec<String>,
}

#[cfg(feature = "parallel")]
pub(crate) type Pool = rayon::ThreadPool;
/// No pool without the `parallel` feature: every node runs on the caller.
#[cfg(not(feature = "parallel"))]
pub(crate) type Pool = ();

pub(crate) fn pool_of(jobs: usize) -> Option<Pool> {
    #[cfg(feature = "parallel")]
    {
        (jobs > 1)
            .then(|| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(jobs)
                    .thread_name(|i| format!("ein-region-{i}"))
                    .build()
                    .ok()
            })
            .flatten()
    }
    #[cfg(not(feature = "parallel"))]
    {
        let _ = jobs;
        None
    }
}

/// Every node of `sets`, in `sets`' order — fanned out over `pool` where a
/// worker can finish one without numbering a new fact, and the rest computed
/// here, in order, which is where their new facts get their numbers. The
/// result is therefore the sequential one byte for byte at any `jobs`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn fan_out(
    pool: Option<&Pool>,
    root: &Kb,
    snapshot: &Snapshot,
    terms: &mut Terms,
    ast: &Ast,
    memo: &SharedMemo,
    axes: &Axes,
    sets: &[Vec<FactId>],
) -> Result<Vec<NodeOut>, SolveError> {
    // `None` is a node handed back.
    #[allow(unused_mut)]
    let mut speculated: Vec<Option<Result<NodeOut, SolveError>>> =
        sets.iter().map(|_| None).collect();
    #[cfg(feature = "parallel")]
    if let Some(pool) = pool {
        use rayon::prelude::*;
        let lent = terms.lend();
        let shared = lent.get();
        pool.install(|| {
            sets.par_iter()
                .map(|facts| {
                    let mut view = shared.worker();
                    view.provs.open_fork();
                    let out = node_in_region(root, snapshot, &mut view, ast, memo, facts, axes);
                    let refused = view.refused();
                    let _ = view.provs.take_fork();
                    (!refused).then_some(out)
                })
                .collect_into_vec(&mut speculated)
        });
    }
    #[cfg(not(feature = "parallel"))]
    let _ = pool;
    let mut outs = Vec::with_capacity(sets.len());
    for (k, facts) in sets.iter().enumerate() {
        outs.push(match speculated[k].take() {
            Some(out) => out?,
            None => node(root, snapshot, terms, ast, memo, facts, axes)?,
        });
    }
    Ok(outs)
}

/// Fork root, write `facts`, saturate to quiescence, read both bits.
///
/// The provenance the fork pushes lives in a region of the arena that is
/// reclaimed before this returns, so `2ⁿ` nodes do not grow it; everything
/// the record keeps is rendered first.
fn node(
    root: &Kb,
    snapshot: &Snapshot,
    terms: &mut Terms,
    ast: &Ast,
    memo: &SharedMemo,
    facts: &[FactId],
    axes: &Axes,
) -> Result<NodeOut, SolveError> {
    terms.provs.open_fork();
    let out = node_in_region(root, snapshot, terms, ast, memo, facts, axes);
    terms.provs.discard_fork();
    out
}

fn node_in_region(
    root: &Kb,
    snapshot: &Snapshot,
    terms: &mut Terms,
    ast: &Ast,
    memo: &SharedMemo,
    facts: &[FactId],
    axes: &Axes,
) -> Result<NodeOut, SolveError> {
    let mut off = Events::off();
    let mut fork = root.branch();
    for &h in facts {
        let (rel, args) = terms.facts.get(h);
        let args = args.to_vec();
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        fork.add_and_index_fact(terms, rel, &args, Some(prov))
            .expect("room for a hypothesis");
    }
    {
        let mut s = Session {
            kb: &mut fork,
            terms,
            ast,
            events: &mut off,
            memo: memo.clone(),
        };
        let delta = snapshot.new_facts_of(s.kb);
        let mut sat = Saturator::resume(&mut s, snapshot, delta)?;
        sat.saturate(&mut s, None, &mut |_| {})?;
    }

    let contradictions = crate::contradiction::detect(&fork, terms);
    let mut node = Node::default();
    if let Some(first) = contradictions.first() {
        let witnesses: Vec<FactId> = contradictions.iter().map(|c| c.witness()).collect();
        let core = crate::explain::smallest_contradiction_frontier(&fork, terms, Some(&witnesses));
        node.core = sorted_text(terms, &core);
        let mut rules: Vec<String> = [first.positive, Some(first.negative)]
            .into_iter()
            .flatten()
            .filter_map(|f| fork.primary(f))
            .map(|p| match terms.provs.get(p).rule {
                Some(r) => terms.sym(r).to_string(),
                None => "(given)".to_string(),
            })
            .collect();
        rules.sort();
        rules.dedup();
        node.refuted_by = rules;
        node.absent_premises = absent_premises(&fork, terms, first.witness());
    }
    let owes = crate::obligations::tally(&fork, terms, ast, memo, &mut off)?;
    node.owes = owes.total() as u32;
    node.owed = owes
        .instances()
        .iter()
        .map(|o| {
            if o.why.is_empty() {
                terms.sym(o.rule).to_string()
            } else {
                o.why.clone()
            }
        })
        .collect();
    let masks = axes.candidates.len() < 64;
    let mut closure: Vec<u32> = Vec::new();
    for (b, &c) in axes.candidates.iter().enumerate() {
        if fork.contains(c) {
            closure.push(b as u32);
            if masks {
                node.closure |= 1 << b;
            }
        }
        if masks && fork.is_negated(c) {
            node.excluded |= 1 << b;
        }
    }
    let state = crate::canon::state_key(&fork);
    let delta: Vec<FactId> = state
        .iter()
        .copied()
        .filter(|f| !axes.root_facts.contains(f))
        .collect();
    for &f in &delta {
        if axes.is_candidate.contains(&f) {
            continue;
        }
        node.beyond += 1;
        if axes.hypothesised.contains(&terms.facts.rel(f)) {
            node.beyond_hypothesised += 1;
        }
    }
    Ok(NodeOut {
        dead: !contradictions.is_empty(),
        owes: node.owes,
        node,
        delta: if axes.keep_states {
            sorted_text(terms, &delta)
        } else {
            Vec::new()
        },
        state: delta.into_boxed_slice(),
        closure,
    })
}

fn sorted_text(terms: &Terms, facts: &[FactId]) -> Vec<String> {
    let mut v: Vec<String> = facts.iter().map(|&f| sexpr(terms, f)).collect();
    v.sort();
    v
}

/// The `absent` premises anywhere in `witness`'s primary derivation, rendered
/// and sorted — the guard a repairable failure rests on.
fn absent_premises(kb: &Kb, terms: &Terms, witness: FactId) -> Vec<String> {
    let mut seen: FxHashSet<FactId> = FxHashSet::default();
    let mut stack = vec![witness];
    let mut out: Vec<String> = Vec::new();
    while let Some(f) = stack.pop() {
        if !seen.insert(f) {
            continue;
        }
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        for r in prov.absent.iter() {
            out.push(format!("(absent {})", naf_text(terms, r.rel, &r.args)));
        }
        stack.extend(prov.premises.iter().copied());
    }
    out.sort();
    out.dedup();
    out
}

fn naf_text(terms: &Terms, rel: ein_core::Symbol, args: &[NafArg]) -> String {
    let mut s = format!("({}", terms.sym(rel));
    for a in args {
        s.push(' ');
        match a {
            NafArg::Free => s.push('_'),
            NafArg::Value(v) => s.push_str(&crate::events::sexpr_value(terms, *v)),
            NafArg::Nested { rel, args } => s.push_str(&naf_text(terms, *rel, args)),
        }
    }
    s.push(')');
    s
}

// ── The topology ───────────────────────────────────────────────────

/// A status, as an edge class sees it: `(false)` first, then the tally.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Class {
    R,
    Owing,
    Dead,
}

impl Class {
    pub fn of(status: u8) -> Class {
        if status & DEAD != 0 {
            Class::Dead
        } else if status & OWING != 0 {
            Class::Owing
        } else {
            Class::R
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Class::R => "R",
            Class::Owing => "owing",
            Class::Dead => "dead",
        }
    }
}

/// One connected piece of `R` under one-fact edges.
#[derive(Clone, Debug)]
pub struct Component {
    pub size: u64,
    /// The global lines' members inside it — its own lines when `R` has no
    /// holes, since a convex region connects every node to the line members
    /// above and below it.
    pub minimal: Vec<u64>,
    pub maximal: Vec<u64>,
}

/// The duality cross-check: without holes, the lines are determined by `N`
/// and `E`.
#[derive(Clone, Debug)]
pub struct Duality {
    /// The alive members of `E`.
    pub minimal_from_e: Vec<u64>,
    /// The maximal sets containing no member of `N`, kept where discharged —
    /// the complements of the minimal hitting sets of `N`.
    pub maximal_from_n: Vec<u64>,
    pub minimal_holds: bool,
    pub maximal_holds: bool,
}

/// A violation of an upward closure the theorem needs, and the smallest pair
/// that shows it.
#[derive(Clone, Copy, Default, Debug)]
pub struct Violation {
    pub count: u64,
    pub witness: Option<(u64, u64)>,
}

/// Everything above the status array.
#[derive(Clone, Debug)]
pub struct Topology {
    pub size_r: u64,
    pub minimal: Vec<u64>,
    pub maximal: Vec<u64>,
    pub holes: Vec<u64>,
    pub minimal_holes: Vec<u64>,
    /// `(hole, bit, node of R)`: adding candidate `bit` revives the hole.
    pub repairs: Vec<(u64, usize, u64)>,
    pub components: Vec<Component>,
    /// Per mask, the index of its component, or `u32::MAX` outside `R`.
    pub component_of: Vec<u32>,
    /// `[a … b]`, maximal inside `R`. `None` when `R` has holes and the
    /// interval search passed its work bound.
    pub subcubes: Option<Vec<(u64, u64)>>,
    /// `N`, the minimal dead sets.
    pub nogoods: Vec<u64>,
    /// `E`, the minimal discharging sets, dead or not.
    pub discharging: Vec<u64>,
    pub duality: Duality,
    /// Nodes of `R` whose upward neighbours all leave `R` while the whole cube
    /// says they are not maximal, or the reverse.
    pub local_max: Vec<u64>,
    pub local_min: Vec<u64>,
    /// A dead node with an alive superset.
    pub dead_not_upward_closed: Violation,
    /// A discharged node with an owing superset.
    pub discharged_not_upward_closed: Violation,
    /// Upward edges, by the classes at their two ends.
    pub edges: Vec<((Class, Class), u64)>,
}

impl Topology {
    /// Everything, from the status array alone.
    pub fn of(n: usize, status: &[u8]) -> Topology {
        let size = status.len();
        debug_assert_eq!(size, 1 << n);
        let r: Vec<bool> = status.iter().map(|&s| s == 0).collect();
        let dead: Vec<bool> = status.iter().map(|&s| s & DEAD != 0).collect();
        let owing: Vec<bool> = status.iter().map(|&s| s & OWING != 0).collect();
        let alive: Vec<bool> = dead.iter().map(|d| !d).collect();
        let discharged: Vec<bool> = owing.iter().map(|o| !o).collect();

        let r_below = subset_or(n, &r);
        let r_above = superset_or(n, &r);
        let minimal = minimal_of(n, &r, &r_below);
        let maximal = maximal_of(n, &r, &r_above);

        let hole: Vec<bool> = (0..size)
            .map(|m| !r[m] && r_below[m] && r_above[m])
            .collect();
        let holes: Vec<u64> = (0..size).filter(|&m| hole[m]).map(|m| m as u64).collect();
        let hole_below = subset_or(n, &hole);
        let minimal_holes = minimal_of(n, &hole, &hole_below);
        let mut repairs = Vec::new();
        for &h in &holes {
            for b in 0..n {
                let y = h | 1 << b;
                if y != h && r[y as usize] {
                    repairs.push((h, b, y));
                }
            }
        }

        // Components, by a walk over one-fact edges inside R.
        let mut component_of = vec![u32::MAX; size];
        let mut components: Vec<Component> = Vec::new();
        for start in 0..size {
            if !r[start] || component_of[start] != u32::MAX {
                continue;
            }
            let id = components.len() as u32;
            let mut stack = vec![start];
            component_of[start] = id;
            let mut count = 0u64;
            while let Some(x) = stack.pop() {
                count += 1;
                for b in 0..n {
                    let y = x ^ (1 << b);
                    if r[y] && component_of[y] == u32::MAX {
                        component_of[y] = id;
                        stack.push(y);
                    }
                }
            }
            components.push(Component {
                size: count,
                minimal: Vec::new(),
                maximal: Vec::new(),
            });
        }
        for &m in &minimal {
            components[component_of[m as usize] as usize]
                .minimal
                .push(m);
        }
        for &m in &maximal {
            components[component_of[m as usize] as usize]
                .maximal
                .push(m);
        }

        let subcubes = if holes.is_empty() {
            // Convex: [a … b] ⊆ R for every a ≤ b on the two lines, and each
            // such pair is maximal because both lines are antichains.
            let mut v = Vec::new();
            for &a in &minimal {
                for &b in &maximal {
                    if a & b == a {
                        v.push((a, b));
                    }
                }
            }
            Some(v)
        } else {
            subcubes_general(n, &r)
        };

        let dead_below = subset_or(n, &dead);
        let nogoods = minimal_of(n, &dead, &dead_below);
        let dis_below = subset_or(n, &discharged);
        let discharging = minimal_of(n, &discharged, &dis_below);

        // Duality: the N side and the E side.
        let minimal_from_e: Vec<u64> = discharging
            .iter()
            .copied()
            .filter(|&m| alive[m as usize])
            .collect();
        let mut has_nogood = vec![false; size];
        for &g in &nogoods {
            has_nogood[g as usize] = true;
        }
        let contains_n = superset_closure_of_points(n, &has_nogood);
        let free: Vec<bool> = contains_n.iter().map(|c| !c).collect();
        let free_above = superset_or(n, &free);
        let maximal_from_n: Vec<u64> = maximal_of(n, &free, &free_above)
            .into_iter()
            .filter(|&m| discharged[m as usize])
            .collect();
        let duality = Duality {
            minimal_holds: minimal_from_e == minimal,
            maximal_holds: maximal_from_n == maximal,
            minimal_from_e,
            maximal_from_n,
        };

        // Local against global.
        let mut local_max = Vec::new();
        let mut local_min = Vec::new();
        let is_max: FxHashSet<u64> = maximal.iter().copied().collect();
        let is_min: FxHashSet<u64> = minimal.iter().copied().collect();
        for m in 0..size {
            if !r[m] {
                continue;
            }
            let ups = (0..n).filter(|b| m >> b & 1 == 0).all(|b| !r[m | 1 << b]);
            let downs = (0..n)
                .filter(|b| m >> b & 1 == 1)
                .all(|b| !r[m & !(1 << b)]);
            if ups != is_max.contains(&(m as u64)) {
                local_max.push(m as u64);
            }
            if downs != is_min.contains(&(m as u64)) {
                local_min.push(m as u64);
            }
        }

        let alive_above = superset_or(n, &alive);
        let owing_above = superset_or(n, &owing);
        let dead_not_upward_closed = violation(n, &dead, &alive_above, &alive);
        let discharged_not_upward_closed = violation(n, &discharged, &owing_above, &owing);

        let mut edges: FxHashMap<(Class, Class), u64> = FxHashMap::default();
        for x in 0..size {
            for b in 0..n {
                if x >> b & 1 == 0 {
                    let y = x | 1 << b;
                    *edges
                        .entry((Class::of(status[x]), Class::of(status[y])))
                        .or_default() += 1;
                }
            }
        }
        // determinism-ok: sorted on the next line, by the classes' order.
        let mut edges: Vec<((Class, Class), u64)> = edges.into_iter().collect();
        edges.sort();

        Topology {
            size_r: r.iter().filter(|&&x| x).count() as u64,
            minimal,
            maximal,
            holes,
            minimal_holes,
            repairs,
            components,
            component_of,
            subcubes,
            nogoods,
            discharging,
            duality,
            local_max,
            local_min,
            dead_not_upward_closed,
            discharged_not_upward_closed,
            edges,
        }
    }
}

/// `f[m] = ∃ x ⊆ m: ind[x]`.
fn subset_or(n: usize, ind: &[bool]) -> Vec<bool> {
    let mut f = ind.to_vec();
    for b in 0..n {
        let bit = 1usize << b;
        for m in 0..f.len() {
            if m & bit != 0 && f[m ^ bit] {
                f[m] = true;
            }
        }
    }
    f
}

/// `f[m] = ∃ x ⊇ m: ind[x]`.
fn superset_or(n: usize, ind: &[bool]) -> Vec<bool> {
    let mut f = ind.to_vec();
    for b in 0..n {
        let bit = 1usize << b;
        for m in 0..f.len() {
            if m & bit == 0 && f[m | bit] {
                f[m] = true;
            }
        }
    }
    f
}

/// `f[m] = ∃ g ⊆ m: points[g]` — every superset of a marked point.
fn superset_closure_of_points(n: usize, points: &[bool]) -> Vec<bool> {
    subset_or(n, points)
}

/// The members of `set` with no proper subset in it, given `below = subset_or(set)`.
fn minimal_of(n: usize, set: &[bool], below: &[bool]) -> Vec<u64> {
    (0..set.len())
        .filter(|&m| set[m] && (0..n).all(|b| m >> b & 1 == 0 || !below[m ^ (1 << b)]))
        .map(|m| m as u64)
        .collect()
}

/// The members of `set` with no proper superset in it, given `above = superset_or(set)`.
fn maximal_of(n: usize, set: &[bool], above: &[bool]) -> Vec<u64> {
    (0..set.len())
        .filter(|&m| set[m] && (0..n).all(|b| m >> b & 1 == 1 || !above[m | (1 << b)]))
        .map(|m| m as u64)
        .collect()
}

/// Members of `from` with a proper superset in `to`, given
/// `to_above = superset_or(to)`, and the smallest such pair.
fn violation(n: usize, from: &[bool], to_above: &[bool], to: &[bool]) -> Violation {
    let mut v = Violation::default();
    let mut best: Option<u64> = None;
    for m in 0..from.len() {
        if from[m] && !to[m] && to_above[m] {
            v.count += 1;
            let better = best.is_none_or(|b| (m.count_ones(), m as u64) < (b.count_ones(), b));
            if better {
                best = Some(m as u64);
            }
        }
    }
    if let Some(m) = best {
        let m = m as usize;
        let up = (0..to.len())
            .filter(|&x| x & m == m && x != m && to[x])
            .min_by_key(|&x| (x.count_ones(), x));
        v.witness = up.map(|x| (m as u64, x as u64));
    }
    let _ = n;
    v
}

/// The maximal intervals inside a region **with** holes, where the two lines
/// say nothing. For each `a` in `R`, the free sets `F` with `[a … a ∪ F] ⊆ R`
/// form a down-set, computed by one pass over the subsets of `a`'s complement;
/// its maximal members give the candidate intervals, and the ones no other
/// candidate contains are kept. `None` past [`SUBCUBE_WORK`].
fn subcubes_general(n: usize, r: &[bool]) -> Option<Vec<(u64, u64)>> {
    let full = (1usize << n) - 1;
    let work: u64 = (0..r.len())
        .filter(|&a| r[a])
        .map(|a| 1u64 << (n - (a as u64).count_ones() as usize))
        .sum();
    if work > SUBCUBE_WORK {
        return None;
    }
    let mut cands: Vec<(u64, u64)> = Vec::new();
    for a in 0..r.len() {
        if !r[a] {
            continue;
        }
        let comp = full & !a;
        // Enumerate the subsets of `comp` in increasing order, so every
        // proper subset of F is decided before F.
        let bits: Vec<usize> = (0..n).filter(|b| comp >> b & 1 == 1).collect();
        let k = bits.len();
        let mut ok = vec![false; 1 << k];
        for local in 0..(1usize << k) {
            let f = expand(local, &bits);
            let mut good = r[a | f];
            if good {
                for j in 0..k {
                    if local >> j & 1 == 1 && !ok[local ^ (1 << j)] {
                        good = false;
                        break;
                    }
                }
            }
            ok[local] = good;
        }
        for local in 0..(1usize << k) {
            if ok[local] && (0..k).all(|j| local >> j & 1 == 1 || !ok[local | 1 << j]) {
                cands.push((a as u64, (a | expand(local, &bits)) as u64));
            }
        }
    }
    let keep: Vec<(u64, u64)> = cands
        .iter()
        .copied()
        .filter(|&(a, b)| {
            !cands
                .iter()
                .any(|&(c, d)| (c, d) != (a, b) && c & a == c && b & d == b)
        })
        .collect();
    Some(keep)
}

fn expand(local: usize, bits: &[usize]) -> usize {
    let mut out = 0;
    for (j, &b) in bits.iter().enumerate() {
        if local >> j & 1 == 1 {
            out |= 1 << b;
        }
    }
    out
}
