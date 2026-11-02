//! Every question `ein solve` asks about `R`, each answered with the least
//! work it needs — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4, the design the user approved on 2026-10-16
//! ([`solve_interface.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)).
//!
//! A question is a [`Mode`]: **which** part of `R` (all of it, the minimal
//! line, the maximal line) and **what is asked** (describe it, find `N` of
//! its states, or expect exactly one). [`run`] answers it.
//!
//! ### Why no mode can disagree with the oracle
//!
//! - **One node function.** Every commitment any mode looks at is
//!   [`crate::region`]'s own node: root's snapshot, the commitment written in,
//!   saturation alone, nothing written into root or the node by the search.
//!   A mode and the oracle can visit different nodes; they cannot read one
//!   differently.
//! - **A skip needs a licence** ([`Licence`]), a theorem the static check
//!   proves for the program's class. A program the check flags gets none but
//!   the cube, so its modes are exact up to `--region-limit` and partial past
//!   it.
//! - **A budget stops work and never changes an answer.** A stopped run says
//!   what it established — members found, a line's members found, a verdict
//!   reached — with [`Answer::exhausted`] false, and no more.
//!
//! `ein-infer/tests/region.rs`'s `every_mode_is_the_oracle` holds every mode
//! to the oracle on every program whose cube the oracle enumerates.
//!
//! ### The routes
//!
//! A monotone program takes **route 1**, the alive down-set
//! ([`crate::downset`]), with a cut per mode on what it expands and when it
//! stops, plus the **climb**: from a node of `R`, add the lowest-index
//! candidate whose extension is alive, until none is. A flagged program takes
//! **route 2**, the cube: in layer order where the question can be decided
//! early (`-n`, `--single`, over all of `R` or the minimal line), and whole
//! ([`crate::region`]) where it cannot.

use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::SharedMemo;
use crate::described::{self, Described};
use crate::detector::{self, StaleCheck, StaleNode, Status, Warning};
use crate::downset::{self, Declined, is_subset, join};
use crate::events::Events;
use crate::monotone::Monotonicity;
use crate::region::{self, Axes, Candidate, Outcome, Pool, Prepared, fan_out, pool_of};
use crate::saturator::Snapshot;
use crate::solve::SolveError;

/// The largest step an exploration takes. Steps start at one node and double,
/// so a question decided by its first node costs one saturation, and a long
/// run fans out as route 1 does.
const BATCH: usize = 1024;

/// Which part of `R` the question is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Which {
    All,
    Minimal,
    Maximal,
}

/// What is asked about it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ask {
    /// All of it.
    Describe,
    /// `N` of its states.
    Find(u64),
    /// Is it exactly one state?
    Single,
}

/// One question: `--minimal` / `--maximal` × describe / `-n N` / `--single`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mode {
    pub which: Which,
    pub ask: Ask,
}

/// How [`run`] is asked.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub mode: Mode,
    /// Facts beyond root, summed over every fixpoint the run computes — the
    /// ruled unit (2026-10-16). Route 1 checks it before each layer, every
    /// other step before itself; the whole cube is bounded by
    /// [`Self::limit`] instead.
    pub budget: u64,
    /// The largest `n` whose whole cube a flagged program gets.
    pub limit: u32,
    pub jobs: usize,
    /// `-T`: checked before each step, as the budget is.
    pub deadline: Option<Instant>,
}

impl Options {
    pub fn new(mode: Mode) -> Self {
        Options {
            mode,
            budget: downset::DEFAULT_BUDGET,
            limit: region::DEFAULT_LIMIT,
            jobs: 1,
            deadline: None,
        }
    }
}

/// Which route answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// Route 1: a monotone program's alive down-set, with the mode's cut.
    DownSet,
    /// Route 2: the cube, in layer order or whole.
    Cube,
}

/// A theorem a mode used to skip work —
/// [`solve_interface.md` § 3.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#31-the-licences).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Licence {
    /// L0: a fork only adds — every node is root's fixpoint and more, so when
    /// root is dead every node is, on any program (M1e2 T1e2.1a.5.3). It is
    /// the node function's own construction, not a theorem about the program,
    /// and it is what decides a flagged program whose root is dead.
    ForkOnlyAdds,
    /// L1: dead is upward-closed — no superset of a dead node is visited.
    DeadUpward,
    /// L2: discharged is upward-closed — an alive superset of a node of `R`
    /// is in `R`.
    DischargedUpward,
    /// L4: a node of `R` is maximal when every one-up extension is dead.
    LocalLines,
    /// L7: every node of the cube, nothing skipped.
    TheCube,
}

impl Licence {
    pub fn label(self) -> &'static str {
        match self {
            Licence::ForkOnlyAdds => "L0 a fork only adds: root is dead, so every node is",
            Licence::DeadUpward => "L1 dead is upward-closed",
            Licence::DischargedUpward => "L2 discharged is upward-closed",
            Licence::LocalLines => "L4 the lines are local",
            Licence::TheCube => "L7 the cube",
        }
    }
}

/// One state the run prints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// The first commitment the route met that reaches it. On the maximal
    /// line of a monotone program, the maximal commitment the climb reached.
    pub set: Box<[u32]>,
    /// The candidates its fixpoint holds. On the maximal line of a monotone
    /// program this is the maximal commitment itself.
    pub closure: Box<[u32]>,
}

/// `--single`'s answer, in today's three words and `Open`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Exactly one state.
    Solution,
    /// Two or more; [`Answer::states`] holds two of them.
    Ambiguity,
    /// None, and no node is alive.
    Contradiction,
    /// None, and every alive node owes.
    Open,
}

impl Verdict {
    pub fn word(self) -> &'static str {
        match self {
            Verdict::Solution => "Solution",
            Verdict::Ambiguity => "Ambiguity",
            Verdict::Contradiction => "Contradiction",
            Verdict::Open => "Open",
        }
    }
}

/// `R`'s shape — the describing question's answer over all of `R`, exact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    pub empty_in_r: bool,
    /// The minimal line, as commitments.
    pub minimal: Vec<Box<[u32]>>,
    /// The maximal line, as commitments.
    pub maximal: Vec<Box<[u32]>>,
    pub components: usize,
    /// `None` where the cube did not compute them.
    pub subcubes: Option<usize>,
    /// `N`, the minimal dead sets.
    pub nogoods: usize,
    /// The cube's holes; `None` on route 1, where a monotone program has
    /// none by the theorem.
    pub holes: Option<usize>,
    /// States reached by more than one commitment of `R`.
    pub merged: usize,
}

/// `|R|`: exact when [`Answer::exhausted`], lower bounds otherwise.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub k_c: u64,
    pub k_s: u64,
}

/// Why a run stopped before deciding its question.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stopped {
    /// The next step's projection would have passed the budget.
    Budget { spent: u64, projected: u64 },
    /// A flagged program whose question needs the whole cube, past the limit.
    Limit { n: usize, limit: u32 },
    /// `-T`.
    Deadline,
}

/// What [`run`] found.
#[derive(Clone, Debug)]
pub struct Answer {
    pub mode: Mode,
    pub candidates: Vec<Candidate>,
    pub monotonicity: Monotonicity,
    pub root_dead: bool,
    pub route: Route,
    pub licences: Vec<Licence>,
    /// The states the run prints, in the route's order: members of the
    /// chosen part of `R`. Under [`Ask::Single`] with
    /// [`Verdict::Ambiguity`], the two that decided it.
    pub states: Vec<State>,
    /// Under [`Which::Minimal`]: the minimal commitments found, each with the
    /// index of its state in [`Self::states`].
    pub minimal: Vec<(Box<[u32]>, usize)>,
    /// Describe, over all of `R`, exhausted.
    pub shape: Option<Shape>,
    /// Describe, over all of `R`.
    pub counts: Option<Counts>,
    /// Describe, over all of `R`: what the run established about it, exhausted
    /// or partial — what the read-out prints.
    pub described: Option<Box<Described>>,
    /// The chosen part of `R` was exhausted: [`Self::states`] is all of it.
    pub exhausted: bool,
    /// [`Ask::Single`]'s verdict, when the run reached one.
    pub verdict: Option<Verdict>,
    pub stopped: Option<Stopped>,
    /// Fixpoints computed.
    pub saturations: u64,
    /// Facts beyond root, summed over them — what the budget counts.
    pub facts: u64,
    pub wall: Duration,
    /// On a program the static check flags, what the detector found: the
    /// status, the stale facts of every node the run computed and, once the
    /// read-out has run, the parent-path spot check ([`crate::detector`]).
    /// `None` on a monotone program.
    pub warning: Option<Box<Warning>>,
}

impl Answer {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// `{a, b}` or `∅`, in candidate order.
    pub fn name(&self, set: &[u32]) -> String {
        if set.is_empty() {
            return "∅".to_string();
        }
        let parts: Vec<&str> = set
            .iter()
            .map(|&b| self.candidates[b as usize].text.as_str())
            .collect();
        format!("{{{}}}", parts.join(", "))
    }
}

/// Answer `opts.mode` about `root`, a **freshly loaded, unsaturated** program
/// nothing else will use, as [`crate::region::compute`] asks. Root is
/// saturated once and never written after.
pub fn run(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<Answer, SolveError> {
    run_with(root, terms, ast, opts, &mut Events::off())
}

/// [`run`], narrating into `events`: root as the walk narrates it, then every
/// node visited — its saturation's events and an `enter` line, in visit order
/// whatever `-j` is — and route 1's `layer` lines (M1e2 T1e2.1a.5.2).
pub fn run_with(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    events: &mut Events,
) -> Result<Answer, SolveError> {
    let t0 = Instant::now();
    let prepared = region::prepare_with(root, terms, ast, events)?;
    // A dead root is the whole answer on any program: every node is a fork of
    // root's fixpoint, and a fork only adds (L0). Route 1 visits root alone.
    let fork_only_adds = prepared.root_dead && !prepared.monotonicity.is_monotone();
    let n = prepared.candidates.len();
    let mut answer = if prepared.monotonicity.is_monotone() || prepared.root_dead {
        if opts.mode
            == (Mode {
                which: Which::All,
                ask: Ask::Describe,
            })
        {
            describe_route_1(root, terms, ast, prepared, t0, opts, events)?
        } else {
            let mut sp = Space::open(root, terms, ast, prepared, opts, events);
            let out = match (opts.mode.which, opts.mode.ask) {
                (Which::All, Ask::Find(n)) => sp.find_all(n),
                (Which::All, Ask::Single) => sp.single_all(),
                (Which::Minimal, ask) => sp.minimal(ask),
                (Which::Maximal, ask) => sp.maximal(ask),
                (Which::All, Ask::Describe) => unreachable!("route 1 above"),
            }?;
            sp.answer(opts.mode, Route::DownSet, out)
        }
    } else {
        flagged(root, terms, ast, prepared, t0, opts, events)?
    };
    if fork_only_adds {
        answer.licences = vec![Licence::ForkOnlyAdds];
        // Root is dead, so every node is, and the route read root alone.
        if let Some(w) = answer.warning.as_mut() {
            w.status = Status::RootDead {
                n,
                limit: opts.limit,
            };
        }
    }
    answer.wall = t0.elapsed();
    Ok(answer)
}

// ── The exploration: route 1's node, a cut per mode ───────────────────

/// What one commitment is, as the explorations remember it.
#[derive(Clone, Debug)]
struct Seen {
    dead: bool,
    owes: u32,
    /// Its state, numbered in the order states were first met.
    class: u32,
    closure: Box<[u32]>,
}

impl Seen {
    fn in_r(&self) -> bool {
        !self.dead && self.owes == 0
    }
}

/// What a mode found, before it is wrapped into an [`Answer`].
#[derive(Default)]
struct Found {
    states: Vec<State>,
    minimal: Vec<(Box<[u32]>, usize)>,
    exhausted: bool,
    verdict: Option<Verdict>,
    stopped: Option<Stopped>,
    licences: Vec<Licence>,
    /// Commitments of `R` met — `k_c`'s lower bound, where a run reports one.
    members: u64,
}

/// The prepared program, the node function, and the work done.
struct Space<'a> {
    root: &'a Kb,
    snapshot: Snapshot,
    terms: &'a mut Terms,
    ast: &'a Ast,
    memo: SharedMemo,
    axes: Axes,
    pool: Option<Pool>,
    facts: Vec<FactId>,
    candidates: Vec<Candidate>,
    monotonicity: Monotonicity,
    root_dead: bool,
    seen: FxHashMap<Box<[u32]>, Seen>,
    classes: FxHashMap<Box<[FactId]>, u32>,
    /// Root's node, once evaluated: what it owes, or why it is dead.
    root_node: Option<region::Node>,
    saturations: u64,
    spent: u64,
    budget: u64,
    deadline: Option<Instant>,
    events: &'a mut Events,
    /// On a flagged program: every node computed that holds a stale fact.
    stale: Vec<StaleNode>,
    /// On a flagged program within the limit: each node's record, so a run
    /// that read every node can say what the cube says of the status.
    records: Option<FxHashMap<Box<[u32]>, region::Node>>,
    /// Root's plans, what the attribution reads a node against.
    plans: Vec<std::sync::Arc<crate::plan::Plan>>,
}

/// `Ok(Err(stop))`: the step was not taken.
type Step<T> = Result<Result<T, Stopped>, SolveError>;

impl<'a> Space<'a> {
    fn open(
        root: &'a mut Kb,
        terms: &'a mut Terms,
        ast: &'a Ast,
        prepared: Prepared,
        opts: &Options,
        events: &'a mut Events,
    ) -> Space<'a> {
        let Prepared {
            snapshot,
            root_dead,
            candidates,
            hypothesised,
            monotonicity,
            memo,
            plans,
        } = prepared;
        let flagged = !monotonicity.is_monotone();
        let mut axes =
            Axes::new(root, &candidates, hypothesised, false).narrating(events.narration());
        if flagged {
            axes = axes.checking_stale(std::sync::Arc::new(StaleCheck::of(&plans)));
        }
        let within = candidates.len() <= opts.limit as usize && candidates.len() < 64;
        let facts = candidates.iter().map(|c| c.fact).collect();
        Space {
            root: root.sealed(),
            snapshot,
            terms,
            ast,
            memo,
            axes,
            pool: pool_of(opts.jobs),
            facts,
            candidates,
            monotonicity,
            root_dead,
            seen: FxHashMap::default(),
            classes: FxHashMap::default(),
            root_node: None,
            saturations: 0,
            spent: 0,
            budget: opts.budget,
            deadline: opts.deadline,
            events,
            stale: Vec::new(),
            records: (flagged && within).then(FxHashMap::default),
            plans,
        }
    }

    fn n(&self) -> usize {
        self.facts.len()
    }

    fn answer(self, mode: Mode, route: Route, found: Found) -> Answer {
        let (n, nodes, stale) = (self.n(), self.saturations, self.stale);
        let warning = (!self.monotonicity.is_monotone()).then(|| {
            Box::new(Warning {
                status: Status::NotRead { n },
                state: detector::State {
                    cube: false,
                    nodes,
                    stale,
                },
                paths: None,
            })
        });
        Answer {
            mode,
            candidates: self.candidates,
            monotonicity: self.monotonicity,
            root_dead: self.root_dead,
            route,
            licences: found.licences,
            states: found.states,
            minimal: found.minimal,
            shape: None,
            counts: None,
            described: None,
            exhausted: found.exhausted,
            verdict: found.verdict,
            stopped: found.stopped,
            saturations: self.saturations,
            facts: self.spent,
            wall: Duration::ZERO,
            warning,
        }
    }

    /// Whether a step of `nodes` new fixpoints fits: the deadline, then the
    /// budget against the run's facts per node so far (at least one).
    fn afford(&self, nodes: usize) -> Result<(), Stopped> {
        if self.deadline.is_some_and(|d| Instant::now() >= d) {
            return Err(Stopped::Deadline);
        }
        let per_node = (self.spent / self.saturations.max(1)).max(1);
        let projected = (nodes as u64).saturating_mul(per_node);
        if self.spent.saturating_add(projected) > self.budget {
            return Err(Stopped::Budget {
                spent: self.spent,
                projected,
            });
        }
        Ok(())
    }

    /// The nodes of `sets`, in order — each computed once per run, every
    /// later ask answered from memory. One step: afforded first.
    fn eval(&mut self, sets: &[Box<[u32]>]) -> Step<Vec<Seen>> {
        let mut fresh: Vec<&Box<[u32]>> = Vec::new();
        let mut uniq: FxHashSet<&[u32]> = FxHashSet::default();
        for s in sets {
            if !self.seen.contains_key(s) && uniq.insert(&s[..]) {
                fresh.push(s);
            }
        }
        if let Err(stop) = self.afford(fresh.len()) {
            return Ok(Err(stop));
        }
        let inputs: Vec<Vec<FactId>> = fresh
            .iter()
            .map(|s| s.iter().map(|&b| self.facts[b as usize]).collect())
            .collect();
        let outs = fan_out(
            self.pool.as_ref(),
            self.root,
            &self.snapshot,
            self.terms,
            self.ast,
            &self.memo,
            &self.axes,
            &inputs,
        )?;
        let mut step = crate::events::Stepped::default();
        for (s, mut o) in fresh.into_iter().zip(outs) {
            step.visited += 1;
            step.facts += o.state.len() as u64;
            if o.dead {
                step.dead += 1;
            } else if o.owes > 0 {
                step.owing += 1;
            } else {
                step.r += 1;
            }
            if self.events.on() || o.events.is_some() || self.events.watching() {
                let text = |bits: &[u32]| {
                    let mut c: Vec<String> = bits
                        .iter()
                        .map(|&b| self.candidates[b as usize].text.clone())
                        .collect();
                    c.sort();
                    c
                };
                let closure = if self.events.watching() {
                    text(&o.closure)
                } else {
                    Vec::new()
                };
                region::narrate_node(self.events, &mut o, text(s), || closure);
            }
            self.saturations += 1;
            if s.is_empty() {
                self.root_node = Some(o.node.clone());
            }
            if !o.stale.is_empty() {
                let status = (if o.dead { region::DEAD } else { 0 })
                    | (if o.owes > 0 { region::OWING } else { 0 });
                let node = self.name_of(s);
                self.stale.push(StaleNode {
                    node,
                    class: region::Class::of(status),
                    facts: std::mem::take(&mut o.stale),
                });
            }
            if let Some(records) = self.records.as_mut() {
                records.insert(s.clone(), o.node.clone());
            }
            self.spent += o.state.len() as u64;
            let next = self.classes.len() as u32;
            let class = *self.classes.entry(o.state).or_insert(next);
            self.seen.insert(
                s.clone(),
                Seen {
                    dead: o.dead,
                    owes: o.owes,
                    class,
                    closure: o.closure.into_boxed_slice(),
                },
            );
        }
        if step.visited > 0 {
            self.events.watch_step(&step);
        }
        Ok(Ok(sets.iter().map(|s| self.seen[s].clone()).collect()))
    }

    /// `{a, b}` or `∅`, in candidate order.
    fn name_of(&self, set: &[u32]) -> String {
        if set.is_empty() {
            return "∅".to_string();
        }
        let parts: Vec<&str> = set
            .iter()
            .map(|&b| self.candidates[b as usize].text.as_str())
            .collect();
        format!("{{{}}}", parts.join(", "))
    }

    /// What the cube says of the status, when this run read every node of it
    /// — a question decided early can still have read them all — and `None`
    /// when it did not.
    fn status_if_whole(&mut self) -> Result<Option<Status>, SolveError> {
        let n = self.n();
        let Some(records) = self.records.as_ref() else {
            return Ok(None);
        };
        if n >= 64 || self.seen.len() as u64 != 1u64 << n {
            return Ok(None);
        }
        let size = 1usize << n;
        let mut status = vec![0u8; size];
        let mut nodes = vec![region::Node::default(); size];
        for (set, seen) in &self.seen {
            let mask = set.iter().fold(0usize, |m, &b| m | 1 << b);
            status[mask] = (if seen.dead { region::DEAD } else { 0 })
                | (if seen.owes > 0 { region::OWING } else { 0 });
            if let Some(r) = records.get(set) {
                nodes[mask] = r.clone();
            }
        }
        let topology = region::Topology::of(n, &status);
        let sites = detector::site_texts(&self.monotonicity, self.terms);
        let root_plans = detector::RootPlans::of(&self.plans);
        let cube = detector::Cube {
            status: &status,
            nodes: &nodes,
            candidates: &self.candidates,
            topology: &topology,
        };
        let mut reader = region::Nodes::of_parts(
            self.root,
            self.snapshot.clone(),
            self.memo.clone(),
            self.terms,
            self.ast,
        );
        detector::status_of(&cube, &sites, &root_plans, &mut reader, self.ast).map(Some)
    }

    /// The layer after `layer`, from its alive (or kept) sets: the
    /// singletons after root, the prefix join after that.
    fn next_layer(&self, layer: usize, kept: &[Box<[u32]>]) -> Vec<Box<[u32]>> {
        if kept.is_empty() {
            Vec::new()
        } else if layer == 0 {
            (0..self.n() as u32).map(|b| Box::from([b])).collect()
        } else {
            join(kept)
        }
    }

    /// Climb from `start`, a node of `R`: add the lowest-index candidate
    /// whose extension is alive, until none is. Every extension of a node of
    /// `R` that is alive is in `R` (L2), and a candidate whose extension died
    /// below stays dead above (L1), so one ascending pass ends at a node of
    /// `R` whose every one-up extension is dead — maximal, by L4.
    fn climb(&mut self, start: &[u32]) -> Step<Box<[u32]>> {
        let mut top: Vec<u32> = start.to_vec();
        for b in 0..self.n() as u32 {
            if top.binary_search(&b).is_ok() {
                continue;
            }
            let mut up = top.clone();
            let at = up.partition_point(|&x| x < b);
            up.insert(at, b);
            let seen = match self.eval(&[up.into_boxed_slice()])? {
                Ok(v) => v.into_iter().next().expect("one"),
                Err(stop) => return Ok(Err(stop)),
            };
            if !seen.dead {
                top = seen.closure.to_vec();
            }
        }
        Ok(Ok(top.into_boxed_slice()))
    }

    /// `-n N` over all of `R`: layer order from `∅`, expanding every alive
    /// node, stopping at the `N`-th state of `R`.
    fn find_all(&mut self, want: u64) -> Result<Found, SolveError> {
        let mut f = Found {
            licences: vec![Licence::DeadUpward],
            ..Found::default()
        };
        let mut classes: FxHashSet<u32> = FxHashSet::default();
        let mut frontier: Vec<Box<[u32]>> = vec![Box::default()];
        let mut layer = 0;
        while !frontier.is_empty() {
            let mut alive = Vec::new();
            for chunk in doubling(&frontier) {
                let outs = match self.eval(chunk)? {
                    Ok(v) => v,
                    Err(stop) => {
                        f.stopped = Some(stop);
                        return Ok(f);
                    }
                };
                for (set, o) in chunk.iter().zip(outs) {
                    if o.dead {
                        continue;
                    }
                    alive.push(set.clone());
                    if o.in_r() && classes.insert(o.class) {
                        f.states.push(State {
                            set: set.clone(),
                            closure: o.closure.clone(),
                        });
                        if f.states.len() as u64 >= want {
                            return Ok(f);
                        }
                    }
                }
            }
            frontier = self.next_layer(layer, &alive);
            layer += 1;
        }
        f.exhausted = true;
        Ok(f)
    }

    /// The minimal line: layer order from `∅`, expanding **owing** nodes
    /// only. A node is generated only when every one-smaller subset owes, so
    /// every node of `R` it meets is minimal (L2), and every minimal one is
    /// met. `-n N` stops at its `N`-th state, `--single` at the second.
    fn minimal(&mut self, ask: Ask) -> Result<Found, SolveError> {
        let mut f = Found {
            licences: vec![Licence::DeadUpward, Licence::DischargedUpward],
            ..Found::default()
        };
        let mut index: FxHashMap<u32, usize> = FxHashMap::default();
        let mut any_alive = false;
        let mut frontier: Vec<Box<[u32]>> = vec![Box::default()];
        let mut layer = 0;
        while !frontier.is_empty() {
            let mut owing = Vec::new();
            for chunk in doubling(&frontier) {
                let outs = match self.eval(chunk)? {
                    Ok(v) => v,
                    Err(stop) => {
                        f.stopped = Some(stop);
                        return Ok(f);
                    }
                };
                for (set, o) in chunk.iter().zip(outs) {
                    if o.dead {
                        continue;
                    }
                    any_alive = true;
                    if o.owes > 0 {
                        owing.push(set.clone());
                        continue;
                    }
                    let next = f.states.len();
                    let at = *index.entry(o.class).or_insert(next);
                    if at == next {
                        f.states.push(State {
                            set: set.clone(),
                            closure: o.closure.clone(),
                        });
                    }
                    f.minimal.push((set.clone(), at));
                    match ask {
                        Ask::Find(want) if f.states.len() as u64 >= want => return Ok(f),
                        Ask::Single if f.states.len() >= 2 => {
                            f.verdict = Some(Verdict::Ambiguity);
                            f.states.truncate(2);
                            return Ok(f);
                        }
                        _ => {}
                    }
                }
            }
            frontier = self.next_layer(layer, &owing);
            layer += 1;
        }
        f.exhausted = true;
        if ask == Ask::Single {
            f.verdict = Some(match (f.states.len(), any_alive) {
                (1, _) => Verdict::Solution,
                (_, true) => Verdict::Open,
                (_, false) => Verdict::Contradiction,
            });
        }
        Ok(f)
    }

    /// `--single` over all of `R`: is `k_s = 1`?
    ///
    /// - **∅ ∈ R**: every alive node is in `R` (L2), so `k_s = 1` exactly
    ///   when no candidate outside root's closure is alive alone — at most
    ///   `n` saturations, stopping at the first that is.
    /// - **∅ ∉ R**: the minimal line, stopping when two of its nodes reach
    ///   different states; then the one state `s` must be maximal, since an
    ///   alive extension of it would be a second state of `R` (L2, L4).
    fn single_all(&mut self) -> Result<Found, SolveError> {
        let root = match self.eval(&[Box::default()])? {
            Ok(v) => v.into_iter().next().expect("one"),
            Err(stop) => {
                return Ok(Found {
                    stopped: Some(stop),
                    ..Found::default()
                });
            }
        };
        if root.dead {
            return Ok(Found {
                licences: vec![Licence::DeadUpward],
                exhausted: true,
                verdict: Some(Verdict::Contradiction),
                ..Found::default()
            });
        }
        let (mut f, s) = if root.in_r() {
            let f = Found {
                licences: vec![Licence::DeadUpward, Licence::DischargedUpward],
                states: vec![State {
                    set: Box::default(),
                    closure: root.closure.clone(),
                }],
                ..Found::default()
            };
            (f, root.closure.clone())
        } else {
            let mut f = self.minimal(Ask::Single)?;
            f.licences.push(Licence::LocalLines);
            if f.stopped.is_some() || f.verdict != Some(Verdict::Solution) {
                return Ok(f);
            }
            f.verdict = None;
            f.exhausted = false;
            f.minimal.clear();
            let s = f.states[0].closure.clone();
            (f, s)
        };
        // Every one-up extension of `s`'s closure: one alive is a second state.
        let ups: Vec<Box<[u32]>> = (0..self.n() as u32)
            .filter(|b| s.binary_search(b).is_err())
            .map(|b| {
                let mut up = s.to_vec();
                let at = up.partition_point(|&x| x < b);
                up.insert(at, b);
                up.into_boxed_slice()
            })
            .collect();
        for chunk in doubling(&ups) {
            let outs = match self.eval(chunk)? {
                Ok(v) => v,
                Err(stop) => {
                    f.stopped = Some(stop);
                    return Ok(f);
                }
            };
            if let Some((set, o)) = chunk.iter().zip(outs).find(|(_, o)| !o.dead) {
                f.states.push(State {
                    set: set.clone(),
                    closure: o.closure,
                });
                f.verdict = Some(Verdict::Ambiguity);
                return Ok(f);
            }
        }
        f.verdict = Some(Verdict::Solution);
        f.exhausted = true;
        Ok(f)
    }

    /// The maximal line: layer order from `∅` over the alive nodes **not
    /// under a maximal state found so far**, climbing from every node of `R`
    /// it meets. A node under a found maximum is alive (L1) and kept for the
    /// join without a visit. A maximal state not yet found lies under no
    /// found one, so the join reaches it. `-n N` stops at the `N`-th,
    /// `--single` at the second.
    fn maximal(&mut self, ask: Ask) -> Result<Found, SolveError> {
        let mut f = Found {
            licences: vec![
                Licence::DeadUpward,
                Licence::DischargedUpward,
                Licence::LocalLines,
            ],
            ..Found::default()
        };
        let mut tops: Vec<Box<[u32]>> = Vec::new();
        let under = |tops: &[Box<[u32]>], set: &[u32]| tops.iter().any(|t| is_subset(set, t));
        let mut any_alive = false;
        let mut frontier: Vec<Box<[u32]>> = vec![Box::default()];
        let mut layer = 0;
        while !frontier.is_empty() {
            let mut kept = Vec::new();
            for chunk in doubling(&frontier) {
                let todo: Vec<Box<[u32]>> =
                    chunk.iter().filter(|s| !under(&tops, s)).cloned().collect();
                if let Err(stop) = self.eval(&todo)? {
                    f.stopped = Some(stop);
                    return Ok(f);
                }
                for set in chunk {
                    if under(&tops, set) {
                        kept.push(set.clone());
                        continue;
                    }
                    let o = self.seen[set].clone();
                    if o.dead {
                        continue;
                    }
                    any_alive = true;
                    kept.push(set.clone());
                    if !o.in_r() {
                        continue;
                    }
                    let top = match self.climb(&o.closure)? {
                        Ok(t) => t,
                        Err(stop) => {
                            f.stopped = Some(stop);
                            return Ok(f);
                        }
                    };
                    tops.push(top.clone());
                    f.states.push(State {
                        set: top.clone(),
                        closure: top,
                    });
                    match ask {
                        Ask::Find(want) if f.states.len() as u64 >= want => return Ok(f),
                        Ask::Single if f.states.len() >= 2 => {
                            f.verdict = Some(Verdict::Ambiguity);
                            return Ok(f);
                        }
                        _ => {}
                    }
                }
            }
            frontier = self.next_layer(layer, &kept);
            layer += 1;
        }
        f.exhausted = true;
        if ask == Ask::Single {
            f.verdict = Some(match (f.states.len(), any_alive) {
                (1, _) => Verdict::Solution,
                (_, true) => Verdict::Open,
                (_, false) => Verdict::Contradiction,
            });
        }
        Ok(f)
    }

    /// A flagged program's cube in layer order, for the questions it can
    /// decide early: `-n N` and `--single`, over all of `R` or the minimal
    /// line. Nothing is skipped. A node of `R` is minimal when no proper
    /// subset is — every one was visited in an earlier layer.
    ///
    /// The cube is bounded by `--region-limit`, not by the facts budget: at
    /// `n ≤ limit` it may run to its end, and past it a run stops after
    /// `2^limit` nodes, which is what the whole cube at the limit costs.
    fn cube_in_order(&mut self, which: Which, ask: Ask, limit: u32) -> Result<Found, SolveError> {
        let mut f = Found {
            licences: vec![Licence::TheCube],
            ..Found::default()
        };
        let n = self.n();
        let max_nodes = (n > limit as usize).then(|| 1u64 << limit.min(63));
        let mut index: FxHashMap<u32, usize> = FxHashMap::default();
        let mut any_alive = false;
        let mut r_sets: FxHashSet<Box<[u32]>> = FxHashSet::default();
        for k in 0..=n {
            let layer = combinations(n, k);
            for chunk in doubling(&layer) {
                if max_nodes.is_some_and(|m| self.saturations + chunk.len() as u64 > m) {
                    f.stopped = Some(Stopped::Limit { n, limit });
                    f.members = r_sets.len() as u64;
                    return Ok(f);
                }
                let outs = match self.eval(chunk)? {
                    Ok(v) => v,
                    Err(stop) => {
                        f.stopped = Some(stop);
                        f.members = r_sets.len() as u64;
                        return Ok(f);
                    }
                };
                for (set, o) in chunk.iter().zip(outs) {
                    if o.dead {
                        continue;
                    }
                    any_alive = true;
                    if o.owes > 0 {
                        continue;
                    }
                    r_sets.insert(set.clone());
                    if which == Which::Minimal && proper_subsets(set).any(|s| r_sets.contains(&s)) {
                        continue;
                    }
                    let next = f.states.len();
                    let at = *index.entry(o.class).or_insert(next);
                    if at == next {
                        f.states.push(State {
                            set: set.clone(),
                            closure: o.closure.clone(),
                        });
                    }
                    if which == Which::Minimal {
                        f.minimal.push((set.clone(), at));
                    }
                    match ask {
                        Ask::Find(want) if f.states.len() as u64 >= want => {
                            f.members = r_sets.len() as u64;
                            return Ok(f);
                        }
                        Ask::Single if f.states.len() >= 2 => {
                            f.verdict = Some(Verdict::Ambiguity);
                            f.states.truncate(2);
                            return Ok(f);
                        }
                        _ => {}
                    }
                }
            }
        }
        f.members = r_sets.len() as u64;
        f.exhausted = true;
        if ask == Ask::Single {
            f.verdict = Some(match (f.states.len(), any_alive) {
                (1, _) => Verdict::Solution,
                (_, true) => Verdict::Open,
                (_, false) => Verdict::Contradiction,
            });
        }
        Ok(f)
    }

    /// A flagged program described past `--region-limit`, from the members
    /// [`Self::cube_in_order`] met. Each is in `R`. Every proper subset of a
    /// member was met in an earlier layer, so a member with no member of `R`
    /// below it is certified minimal, monotone or not. The maximal line needs
    /// the whole cube, and is not certified.
    fn partial_cube(&self) -> Described {
        // determinism-ok: sorted by size and candidates on the next line.
        let mut met: Vec<(&Box<[u32]>, &Seen)> = self.seen.iter().collect();
        met.sort_by(|a, b| (a.0.len(), a.0).cmp(&(b.0.len(), b.0)));
        let members: FxHashSet<&[u32]> = met
            .iter()
            .filter(|(_, s)| s.in_r())
            .map(|(set, _)| &set[..])
            .collect();
        let alive: FxHashSet<&[u32]> = met
            .iter()
            .filter(|(_, s)| !s.dead)
            .map(|(set, _)| &set[..])
            .collect();
        let mut minimal_commitments: Vec<Box<[u32]>> = Vec::new();
        let mut order: Vec<u32> = Vec::new();
        let mut by_class: FxHashMap<u32, described::State> = FxHashMap::default();
        for (set, s) in met.iter().filter(|(_, s)| s.in_r()) {
            let minimal = !proper_subsets(set).any(|p| members.contains(&p[..]));
            if minimal {
                minimal_commitments.push((*set).clone());
            }
            let st = by_class.entry(s.class).or_insert_with(|| {
                order.push(s.class);
                described::State {
                    rep: (*set).clone(),
                    closure: s.closure.clone(),
                    commitments: 0,
                    generators: Vec::new(),
                    minimal: false,
                    maximal: false,
                }
            });
            st.commitments += 1;
            st.minimal |= minimal;
            if !st.generators.iter().any(|g| is_subset(g, set)) {
                st.generators.push((*set).clone());
            }
        }
        let mut states: Vec<described::State> = order
            .iter()
            .map(|c| by_class.remove(c).expect("met"))
            .collect();
        states.sort_by(|a, b| (a.closure.len(), &a.closure).cmp(&(b.closure.len(), &b.closure)));
        let dead: Vec<&[u32]> = met
            .iter()
            .filter(|(_, s)| s.dead)
            .map(|(set, _)| &set[..])
            .collect();
        let minimal_dead: Vec<&[u32]> = dead
            .iter()
            .copied()
            .filter(|d| proper_subsets(d).all(|p| alive.contains(&p[..])))
            .collect();
        let root = self.seen.get(&[][..]);
        let node = self.root_node.clone().unwrap_or_default();
        Described {
            candidates: self.candidates.clone(),
            monotonicity: self.monotonicity.clone(),
            route: Route::Cube,
            exhausted: false,
            complete_up_to: None,
            stopped_before: None,
            root: described::Root {
                alive: root.is_some_and(|r| !r.dead) || (root.is_none() && !self.root_dead),
                owes: root.map_or(0, |r| r.owes),
                owed: node.owed,
                core: node.core,
                refuted_by: node.refuted_by,
                sources: node.sources,
            },
            any_alive: !alive.is_empty(),
            k_c: members.len() as u64,
            minimal_commitments,
            nogoods: minimal_dead.len() as u64,
            dead: minimal_dead
                .iter()
                .map(|d| described::Dead {
                    set: Box::from(*d),
                    core: Vec::new(),
                    refuted_by: Vec::new(),
                })
                .collect(),
            dead_alone: dead.iter().filter(|d| d.len() == 1).count() as u64,
            dead_alone_set: dead.iter().filter(|d| d.len() == 1).map(|d| d[0]).collect(),
            owing_met: met.iter().any(|(_, s)| s.owes > 0),
            dead_met: !dead.is_empty(),
            states,
            shape: None,
            frontier: Vec::new(),
            work: described::Work {
                saturations: self.saturations,
                facts: self.spent,
                layers: None,
            },
        }
    }
}

/// Steps of 1, 2, 4, … up to [`BATCH`] nodes.
fn doubling<T>(items: &[T]) -> impl Iterator<Item = &[T]> {
    let (mut start, mut size) = (0usize, 1usize);
    std::iter::from_fn(move || {
        if start >= items.len() {
            return None;
        }
        let end = (start + size).min(items.len());
        let chunk = &items[start..end];
        start = end;
        size = (size * 2).min(BATCH);
        Some(chunk)
    })
}

/// Every `k`-subset of `0..n`, lexicographically — route 1's visit order.
fn combinations(n: usize, k: usize) -> Vec<Box<[u32]>> {
    let mut out = Vec::new();
    if k > n {
        return out;
    }
    let mut c: Vec<u32> = (0..k as u32).collect();
    loop {
        out.push(c.clone().into_boxed_slice());
        let mut i = k;
        loop {
            if i == 0 {
                return out;
            }
            i -= 1;
            if (c[i] as usize) < n - k + i {
                break;
            }
            if i == 0 {
                return out;
            }
        }
        c[i] += 1;
        for j in i + 1..k {
            c[j] = c[j - 1] + 1;
        }
    }
}

/// Every proper subset of `set`.
fn proper_subsets(set: &[u32]) -> impl Iterator<Item = Box<[u32]>> + '_ {
    let k = set.len();
    (0..(1u64 << k) - 1).map(move |m| {
        set.iter()
            .enumerate()
            .filter(|(i, _)| m >> i & 1 == 1)
            .map(|(_, &x)| x)
            .collect()
    })
}

// ── Route 1 whole, and the cube whole ───────────────────────────────────

/// Describe all of `R` on a monotone program: route 1 unchanged. Where it
/// sees the cube contradict the static verdict, the cube answers instead.
fn describe_route_1(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    prepared: Prepared,
    t0: Instant,
    opts: &Options,
    events: &mut Events,
) -> Result<Answer, SolveError> {
    let d = downset::compute_prepared(
        root,
        terms,
        ast,
        prepared,
        t0,
        &downset::Options {
            budget: opts.budget,
            max_visits: u64::MAX,
            jobs: opts.jobs,
            deadline: opts.deadline,
        },
        events,
    )?;
    let saturations = d.points.len() as u64 + u64::from(d.top.is_some());
    let facts: u64 = d.layers.iter().map(|l| l.facts).sum::<u64>() + d.top_facts;
    let described = described::of_downset(&d);
    let mut states = Vec::new();
    let (mut k_c, mut k_s) = (0u64, 0u64);
    for (i, p) in d.points.iter().enumerate() {
        if p.in_r() {
            k_c += 1;
            if p.class as usize == i {
                k_s += 1;
                states.push(State {
                    set: p.set.clone(),
                    closure: p.closure.clone(),
                });
            }
        }
    }
    let set_of = |i: &u32| d.points[*i as usize].set.clone();
    let shape = d.description.as_ref().map(|x| Shape {
        empty_in_r: d.points.first().is_some_and(|p| p.in_r()),
        minimal: x.minimal.iter().map(set_of).collect(),
        maximal: x.maximal.iter().map(set_of).collect(),
        components: x.components.len(),
        subcubes: Some(x.subcubes.len()),
        nogoods: x.nogoods.len(),
        holes: None,
        merged: x.merged.len(),
    });
    let stopped = match &d.declined {
        None => None,
        Some(Declined::Budget {
            spent, projected, ..
        }) => Some(Stopped::Budget {
            spent: *spent,
            projected: *projected,
        }),
        Some(Declined::Contradicted { .. } | Declined::NotMonotone) => {
            // The static check called the program monotone and the cube says
            // otherwise: the theorem's premise failed, which is an engine
            // defect, never an answer. The corpus has no such program
            // (`route_1_is_the_oracle_on_every_monotone_program_it_reaches`).
            return Err(SolveError::Unsupported(
                "route 1 saw the cube contradict the static verdict; run `ein oracle` on this \
                 program"
                    .to_string(),
            ));
        }
    };
    // A flagged program reaches route 1 only with a dead root; the caller
    // sets the status.
    let warning = (!d.monotonicity.is_monotone()).then(|| {
        Box::new(Warning {
            status: Status::NotRead {
                n: d.candidates.len(),
            },
            state: detector::State {
                cube: false,
                nodes: saturations,
                stale: d.stale.clone(),
            },
            paths: None,
        })
    });
    Ok(Answer {
        mode: opts.mode,
        candidates: d.candidates,
        monotonicity: d.monotonicity,
        root_dead: d.root_dead,
        route: Route::DownSet,
        licences: vec![Licence::DeadUpward],
        exhausted: shape.is_some(),
        shape,
        counts: Some(Counts { k_c, k_s }),
        described: Some(Box::new(described)),
        states,
        minimal: Vec::new(),
        verdict: None,
        stopped,
        saturations,
        facts,
        wall: Duration::ZERO,
        warning,
    })
}

/// A flagged program: no licence but the cube.
fn flagged(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    prepared: Prepared,
    t0: Instant,
    opts: &Options,
    events: &mut Events,
) -> Result<Answer, SolveError> {
    let n = prepared.candidates.len();
    let early = matches!(
        (opts.mode.which, opts.mode.ask),
        (Which::All | Which::Minimal, Ask::Find(_) | Ask::Single)
    );
    let within = n <= opts.limit as usize && n < 64;
    if early || !within {
        // Layer order, stopping when the question is decided. Past the limit
        // a question that needs the whole cube stays undecided. The cube is
        // bounded by the limit, so the facts budget does not apply.
        let mut sp = Space::open(root, terms, ast, prepared, opts, events);
        sp.budget = u64::MAX;
        let which = if early { opts.mode.which } else { Which::All };
        let ask = if early {
            opts.mode.ask
        } else {
            Ask::Find(u64::MAX)
        };
        let mut found = sp.cube_in_order(which, ask, opts.limit.min(63))?;
        if !early {
            // Members of `R` found, printed as members and not as line
            // members: the lines are the whole cube's.
            found.exhausted = false;
            if found.stopped.is_none() {
                found.stopped = Some(Stopped::Limit {
                    n,
                    limit: opts.limit,
                });
            }
        }
        let describe_all = opts.mode
            == Mode {
                which: Which::All,
                ask: Ask::Describe,
            };
        let counts = describe_all.then_some(Counts {
            k_c: found.members,
            k_s: found.states.len() as u64,
        });
        let partial = describe_all.then(|| Box::new(sp.partial_cube()));
        // The cube's half: past the limit it is not known; within it, a run
        // that read every node says what the cube says.
        let status = if !within {
            Status::PastLimit {
                n,
                limit: opts.limit,
            }
        } else {
            match sp.status_if_whole()? {
                Some(s) => s,
                None => Status::NotRead { n },
            }
        };
        let whole = matches!(status, Status::Exercised { .. } | Status::Conservative);
        let mut answer = sp.answer(opts.mode, Route::Cube, found);
        if let Some(w) = answer.warning.as_mut() {
            w.status = status;
            w.state.cube = whole;
        }
        answer.counts = counts;
        answer.described = partial;
        return Ok(answer);
    }
    let outcome = region::compute_prepared(
        root,
        terms,
        ast,
        prepared,
        t0,
        &region::Options {
            limit: opts.limit,
            keep_states: false,
            jobs: opts.jobs,
            deadline: opts.deadline,
        },
        &mut |_| {},
        events,
    )?;
    let Outcome::Region(r) = outcome else {
        unreachable!("within the limit");
    };
    Ok(of_cube(opts.mode, *r))
}

/// The answer to `mode`, read off the whole cube — also how `ein oracle`
/// prints what it computed, in `ein solve`'s layout.
pub fn of_cube(mode: Mode, r: region::Region) -> Answer {
    let n = r.n();
    let set_of = |m: u64| -> Box<[u32]> { (0..n as u32).filter(|b| m >> b & 1 == 1).collect() };
    // Layer order, then lexicographic by candidate index: route 1's order.
    let ordered = |masks: &[u64]| -> Vec<u64> {
        let mut v = masks.to_vec();
        v.sort_by_key(|&m| (m.count_ones(), set_of(m)));
        v
    };
    let states_of = |masks: &[u64]| -> Vec<State> {
        let mut seen: FxHashSet<u64> = FxHashSet::default();
        ordered(masks)
            .into_iter()
            .filter(|&m| seen.insert(r.nodes[m as usize].class))
            .map(|m| State {
                set: set_of(m),
                closure: set_of(r.nodes[m as usize].closure),
            })
            .collect()
    };
    let t = &r.topology;
    let in_r: Vec<u64> = (0..1u64 << n).filter(|&m| r.in_r(m)).collect();
    let any_alive = r.status.iter().any(|s| s & region::DEAD == 0);
    let line: Vec<u64> = match mode.which {
        Which::All => in_r.clone(),
        Which::Minimal => t.minimal.clone(),
        Which::Maximal => t.maximal.clone(),
    };
    let mut states = states_of(&line);
    let mut minimal = Vec::new();
    if mode.which == Which::Minimal {
        let index: FxHashMap<u64, usize> = states
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let m = s.set.iter().fold(0u64, |m, &b| m | 1 << b);
                (r.nodes[m as usize].class, i)
            })
            .collect();
        minimal = ordered(&line)
            .into_iter()
            .map(|m| (set_of(m), index[&r.nodes[m as usize].class]))
            .collect();
    }
    let mut exhausted = true;
    let mut verdict = None;
    match mode.ask {
        Ask::Describe => {}
        Ask::Find(want) => {
            if (states.len() as u64) > want {
                states.truncate(want as usize);
                exhausted = false;
                let keep: FxHashSet<usize> = (0..want as usize).collect();
                minimal.retain(|(_, i)| keep.contains(i));
            }
        }
        Ask::Single => {
            verdict = Some(match (states.len(), any_alive) {
                (1, _) => Verdict::Solution,
                (0, true) => Verdict::Open,
                (0, false) => Verdict::Contradiction,
                _ => Verdict::Ambiguity,
            });
            if states.len() > 2 {
                states.truncate(2);
                exhausted = false;
            }
            minimal.clear();
        }
    }
    let describe_all = mode
        == Mode {
            which: Which::All,
            ask: Ask::Describe,
        };
    let classes: FxHashSet<u64> = in_r.iter().map(|&m| r.nodes[m as usize].class).collect();
    let mut per_class: FxHashMap<u64, u32> = FxHashMap::default();
    for &m in &in_r {
        *per_class.entry(r.nodes[m as usize].class).or_default() += 1;
    }
    let shape = describe_all.then(|| Shape {
        empty_in_r: r.in_r(0),
        minimal: ordered(&t.minimal).into_iter().map(set_of).collect(),
        maximal: ordered(&t.maximal).into_iter().map(set_of).collect(),
        components: t.components.len(),
        subcubes: t.subcubes.as_ref().map(Vec::len),
        nogoods: t.nogoods.len(),
        holes: Some(t.holes.len()),
        // determinism-ok: a count over the values, which no order reaches.
        merged: per_class.values().filter(|&&c| c > 1).count(),
    });
    let described = describe_all.then(|| Box::new(described::of_cube(&r)));
    // Facts beyond root: the non-candidates and the candidates each holds.
    let facts: u64 = r
        .nodes
        .iter()
        .map(|x| x.beyond as u64 + x.closure.count_ones() as u64)
        .sum();
    Answer {
        mode,
        route: Route::Cube,
        licences: vec![Licence::TheCube],
        states,
        minimal,
        counts: describe_all.then_some(Counts {
            k_c: t.size_r,
            k_s: classes.len() as u64,
        }),
        shape,
        described,
        exhausted,
        verdict,
        stopped: None,
        saturations: 1u64 << n,
        facts,
        wall: Duration::ZERO,
        warning: r.warning,
        candidates: r.candidates,
        monotonicity: r.monotonicity,
        root_dead: r.root_dead,
    }
}

// ── `--monotone`: a question about the program ──────────────────────────

/// `ein solve FILE --monotone`'s answer, a question about the **program**
/// rather than about `R` — M1e2 S1e2.1b.5, the form the user approved on
/// 2026-11-03 ((a1) of
/// [`detector_proposal.md`](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md)
/// § 2.2). Its work is root's fixpoint and the static check, and nothing
/// else: no node is built, so it has no `R` to disagree with `ein solve`
/// about, and its verdict and sites are the ones every surface prints, from
/// the same [`crate::monotone::classify`].
#[derive(Clone, Debug)]
pub struct Checked {
    /// The universe, as every route has it: how many candidates.
    pub n: usize,
    pub monotonicity: Monotonicity,
    /// Root's own fixpoint derives `(false)`.
    pub root_dead: bool,
    pub wall: Duration,
}

/// Answer `--monotone` about `root`, a **freshly loaded, unsaturated**
/// program: root's fixpoint, the universe and the static check, narrated into
/// `events` as every route narrates root.
pub fn check_monotone(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
) -> Result<Checked, SolveError> {
    let t0 = Instant::now();
    let p = region::prepare_with(root, terms, ast, events)?;
    Ok(Checked {
        n: p.candidates.len(),
        monotonicity: p.monotonicity,
        root_dead: p.root_dead,
        wall: t0.elapsed(),
    })
}
