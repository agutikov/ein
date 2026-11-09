//! Every question `ein solve` asks about `R`, each answered with the least
//! work it needs — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4, the design the user approved on 2026-10-16
//! ([`solve_interface.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)).
//!
//! A question is a [`Mode`]: **which** part of `R` (all of it, the minimal
//! line, the maximal line) and **what is asked** (describe it, find `N` of
//! its states, or expect exactly one; or, over all of `R` only, count it).
//! [`run`] answers it. `--count` (M1e2 T1e2.1c.4.6) is describe's question
//! with the listing left out: [`run`] answers it by describe's route, and
//! the read-out ([`crate::read_out`]) reads no state for it.
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
//! **Describe** on a monotone program takes **the state route**
//! ([`crate::states`], M1e2 S1e2.1c.2): every alive state once, its one-up
//! extensions saturated, `R` read off the states. **Every other question on a
//! monotone program** takes the least of the same lattice of states it needs
//! (`crate::tree`, M1e2 S1e2.1c.4): the **obligation tree**, where an owing
//! state branches on one owed instance's alternatives (L5), for `-n`,
//! `--single` and a first state of either line; a **climb** and **dualise
//! and advance** for the maximal line; a **shrink** and the duality against
//! the maximal owing states for the minimal line. Until S1e2.1c.4 they took
//! route 1 ([`crate::downset`]) with a cut per mode. A flagged program takes
//! **route 2**, the cube: in layer order where the question can be decided
//! early (`-n`, `--single`, over all of `R` or the minimal line), and whole
//! ([`crate::region`]) where it cannot. Past `--region-limit` a question that
//! needs the whole cube meets the members of `R` in layer order up to the
//! limit; describe certifies what it can of them (`Space::partial_cube`),
//! and since M1e2 S1e2.2.26 (W27) a line mode answers from the same
//! description, so it certifies what describe certifies and no more. A
//! program whose root is dead is decided by route 1 in one saturation (L0,
//! L1). **Since M1f S1f.5.6 a minimal question where ∅ is in `R` is decided
//! at root on every route**, a flagged program's within the limit or past it
//! (`Space::empty_state`): ∅ is below every commitment, so the minimal line
//! is {∅}, on the user's ruling of 2026-11-07 on S-16.
//!
//! ### Describe's route — a decision taken without a ruling
//!
//! **The rule reads the program and nothing else: the static check's
//! verdict.** Monotone, describe takes the state route; flagged, it takes the
//! cube as before, or route 1 when root is dead. No option chooses it.
//!
//! The argument is that the state route never does more than route 1. The
//! alive commitments it saturates are distinct, so they are at most route 1's;
//! a dead extension it saturates is pruned from every superset (L1), and
//! the commitments that are a state already are read, not saturated (L3).
//! That bounds the alive half and not the dead half, so the premise is
//! measured and held: over the 181 monotone programs the corpus holds it
//! saturates fewer nodes on 32 and the same on the rest, and derives no more
//! facts on any (`alive_set_reach.md` § 9), and
//! `the_state_route_never_does_more_than_route_1` fails on the first program
//! where it would, at which point this rule is revisited. Where both routes
//! finish, their read-outs are identical apart from the `route` and `work`
//! rows (`ein-infer/tests/explore.rs`).

use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::SharedMemo;
use crate::described::{self, Described};
use crate::detector::{self, StaleCheck, StaleNode, Status, Warning};
use crate::downset::{self, Declined, is_subset};
use crate::error::SolveError;
use crate::events::Events;
use crate::monotone::Monotonicity;
use crate::region::{
    self, Axes, Candidate, Cores, Outcome, Pool, Prepared, RootStats, fan_out, pool_of,
};
use crate::saturator::Snapshot;

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
    /// How many states and commitments does it hold? `--count`, M1e2
    /// T1e2.1c.4.6, the user's ruling of 2026-11-03: over all of `R` only,
    /// since k_s and k_c are `R`'s. [`run`] answers it by describe's route,
    /// so its counts are describe's by construction, and the read-out reads
    /// no state ([`crate::read_out`]).
    Count,
}

/// One question: `--minimal` / `--maximal` × describe / `-n N` / `--single`,
/// or `--count` over all of `R`.
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
    /// `-y`: the parent-path spot check, unbounded and on every program — read
    /// by the read-out ([`crate::read_out`]), not by a route, so no answer and
    /// no `work` count depends on it. The program's own `(config
    /// :lattice-sanity-check true)` turns it on as well (M1e2 S1e2.1c.5).
    pub sanity: bool,
    /// Which dead nodes' unsat cores the run takes beyond root's and, on the
    /// cube, the minimal holes', which the read-out and the summary print
    /// ([`Cores`]): every one the walk visits (`--dump-lattice`, `--events`),
    /// and after the walk each member of `N` a description holds
    /// (`--dump-answer`, `--trace`). M1e2 S1e2.2.21 and M1f S1f.5.12, the
    /// user's rulings of 2026-11-05 and 2026-11-08, *minimum work per
    /// question*: every core taken is the one an eager walk takes, and no
    /// answer depends on it.
    pub cores: Cores,
}

impl Options {
    pub fn new(mode: Mode) -> Self {
        Options {
            mode,
            budget: downset::DEFAULT_BUDGET,
            limit: region::DEFAULT_LIMIT,
            jobs: 1,
            deadline: None,
            sanity: false,
            cores: Cores::ALL,
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
    /// The state route: a monotone program's alive states, each at most once,
    /// and `R` read off them ([`crate::states`]) — describe's since
    /// S1e2.1c.2, and the modes' over states (`crate::tree`) since
    /// S1e2.1c.4 where no owed instance's alternatives were branched on.
    States,
    /// The obligation tree (`crate::tree`, M1e2 S1e2.1c.4): the modes' route
    /// over states on a monotone program, where an owing state branched on one
    /// owed instance's alternatives (L5).
    Tree,
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
    /// L3: a commitment's state is its closure's — so the state route visits
    /// states, not commitments, and counts `k_c` over them as
    /// `f(T) = 2^|T| − Σ f(S)` over the alive states `S ⊊ T`
    /// ([`crate::states`], M1e2 S1e2.1c.2).
    Closure,
    /// L4: a node of `R` is maximal when every one-up extension is dead.
    LocalLines,
    /// L5: an owed instance's alternatives are exhaustive — an owing state
    /// branches on one owed instance's alternatives, and one whose every
    /// alternative is dead has no member of `R` above it
    /// (`crate::tree`, M1e2 S1e2.1c.4).
    Exhaustive,
    /// L7: every node of the cube, nothing skipped.
    TheCube,
}

impl Licence {
    pub fn label(self) -> &'static str {
        match self {
            Licence::ForkOnlyAdds => "L0 a fork only adds: root is dead, so every node is",
            Licence::DeadUpward => "L1 dead is upward-closed",
            Licence::DischargedUpward => "L2 discharged is upward-closed",
            Licence::Closure => "L3 a commitment's state is its closure's",
            Licence::LocalLines => "L4 the lines are local",
            Licence::Exhaustive => "L5 an owed instance's alternatives are exhaustive",
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
    /// Past `--region-limit` on a program the static check flags, under
    /// `--minimal` or `--maximal`: describe's partial description of the
    /// members the run met (`Space::partial_cube`), whose line members are
    /// [`Self::states`]. So a line mode certifies what describe certifies, by
    /// the one rule: a member met with no member of `R` met below it is
    /// minimal, and the maximal line, which needs the whole cube, is not
    /// known. The read-out counts the other members met and does not list
    /// them, and `--dump-answer` writes each (M1e2 S1e2.2.26, W27, the user's
    /// ruling of 2026-11-05 on Q-M1e2.19). `None` on every other run.
    pub partial: Option<Box<Described>>,
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
    /// What root's saturation and the universe's generator pass counted, as
    /// every route starts from them: `--json-summary`'s `root` block.
    pub root: RootStats,
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
    if opts.mode.ask == Ask::Count {
        // `--count` is describe's question with the listing left out, so it
        // takes describe's route with describe's mode, and only the answer's
        // label says what was asked: no path of its own to disagree by.
        if opts.mode.which != Which::All {
            return Err(SolveError::Unsupported(
                "--count is over all of R only: k_s and k_c are R's, and a bare `ein solve FILE` \
                 describes R, the lines' sizes among it"
                    .to_string(),
            ));
        }
        let describe = Options {
            mode: Mode {
                which: Which::All,
                ask: Ask::Describe,
            },
            ..*opts
        };
        let mut answer = run_with(root, terms, ast, &describe, events)?;
        answer.mode = opts.mode;
        return Ok(answer);
    }
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
            // Describe's route: the module's § Describe's route.
            if prepared.monotonicity.is_monotone() {
                describe_states(root, terms, ast, prepared, t0, opts, events)?
            } else {
                describe_route_1(root, terms, ast, prepared, t0, opts, events)?
            }
        } else {
            // The modes' routes over states (`crate::tree`, S1e2.1c.4); a dead
            // root is the whole answer, in one saturation (L0, L1).
            let mut sp = Space::open(root, terms, ast, prepared, opts, events);
            let out = if sp.root_dead {
                sp.dead_root(opts.mode)?
            } else {
                crate::tree::answer(&mut sp, opts.mode)?
            };
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
pub(crate) struct Seen {
    pub(crate) dead: bool,
    pub(crate) owes: u32,
    /// Its state, numbered in the order states were first met.
    pub(crate) class: u32,
    pub(crate) closure: Box<[u32]>,
    /// The instances of L5's table it owes ([`crate::region::NodeOut::owed`]),
    /// when the run carries one.
    pub(crate) owed: Box<[u32]>,
}

impl Seen {
    pub(crate) fn in_r(&self) -> bool {
        !self.dead && self.owes == 0
    }
}

/// What a mode found, before it is wrapped into an [`Answer`].
#[derive(Default)]
pub(crate) struct Found {
    pub(crate) states: Vec<State>,
    pub(crate) minimal: Vec<(Box<[u32]>, usize)>,
    pub(crate) exhausted: bool,
    pub(crate) verdict: Option<Verdict>,
    pub(crate) stopped: Option<Stopped>,
    pub(crate) licences: Vec<Licence>,
    /// Commitments of `R` met — `k_c`'s lower bound, where a run reports one.
    pub(crate) members: u64,
    /// The route that answered, when it is not the one the caller names.
    pub(crate) route: Option<Route>,
}

/// The prepared program, the node function, and the work done.
pub(crate) struct Space<'a> {
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
    /// What root's saturation and the universe's pass counted.
    root_stats: RootStats,
}

/// `Ok(Err(stop))`: the step was not taken.
pub(crate) type Step<T> = Result<Result<T, Stopped>, SolveError>;

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
            root: root_stats,
            root_owes: _,
        } = prepared;
        let flagged = !monotonicity.is_monotone();
        let mut axes = Axes::new(root, &candidates, hypothesised, false)
            .narrating(events.narration())
            .taking_cores(opts.cores.visited);
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
            root_stats,
        }
    }

    pub(crate) fn n(&self) -> usize {
        self.facts.len()
    }

    fn answer(self, mode: Mode, route: Route, found: Found) -> Answer {
        let route = found.route.unwrap_or(route);
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
            partial: None,
            exhausted: found.exhausted,
            verdict: found.verdict,
            stopped: found.stopped,
            saturations: self.saturations,
            facts: self.spent,
            wall: Duration::ZERO,
            warning,
            root: self.root_stats,
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
    pub(crate) fn eval(&mut self, sets: &[Box<[u32]>]) -> Step<Vec<Seen>> {
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
                    owed: o.owed.into_boxed_slice(),
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

    /// A dead root decides every mode in one saturation: no node is alive
    /// (L1, and on a flagged program L0), so every part of `R` is empty.
    fn dead_root(&mut self, mode: Mode) -> Result<Found, SolveError> {
        if let Err(stop) = self.eval(&[Box::default()])? {
            return Ok(Found {
                stopped: Some(stop),
                ..Found::default()
            });
        }
        let licences = match mode.which {
            Which::All => vec![Licence::DeadUpward],
            Which::Minimal => vec![Licence::DeadUpward, Licence::DischargedUpward],
            Which::Maximal => vec![
                Licence::DeadUpward,
                Licence::DischargedUpward,
                Licence::LocalLines,
            ],
        };
        Ok(Found {
            licences,
            exhausted: true,
            verdict: (mode.ask == Ask::Single).then_some(Verdict::Contradiction),
            ..Found::default()
        })
    }

    /// **The ∅ rule** (M1f S1f.5.6, the user's ruling of 2026-11-07 on S-16,
    /// *"b, use the ∅ rule in both"*): the minimal line is the members of `R`
    /// with no smaller member, and ∅ is below every commitment, so where ∅ is
    /// in `R` the minimal line is {∅} and no other state is on it. Whether ∅
    /// is in `R` is root's fixpoint alone, so every minimal question is
    /// decided by root's node, one saturation, on every program, monotone or
    /// not, within `--region-limit` or past it; no licence about the program
    /// is used. Root's node is computed for what the read-out prints of it,
    /// its closure and its stale facts; it is root's fixpoint, so it is in
    /// `R` exactly when root is alive and owes nothing, and a node that says
    /// otherwise is an engine defect, refused.
    fn empty_state(&mut self, ask: Ask) -> Result<Found, SolveError> {
        let root = match self.eval(&[Box::default()])? {
            Ok(mut v) => v.pop().expect("one node"),
            Err(stop) => {
                return Ok(Found {
                    licences: vec![Licence::TheCube],
                    stopped: Some(stop),
                    ..Found::default()
                });
            }
        };
        if !root.in_r() {
            return Err(SolveError::Unsupported(
                "root's node is not in R, and root's fixpoint is alive and owes nothing — an \
                 engine defect; run `ein oracle` on this program"
                    .to_string(),
            ));
        }
        Ok(Found {
            licences: vec![Licence::TheCube],
            states: vec![State {
                set: Box::default(),
                closure: root.closure,
            }],
            minimal: vec![(Box::default(), 0)],
            exhausted: true,
            verdict: (ask == Ask::Single).then_some(Verdict::Solution),
            members: 1,
            ..Found::default()
        })
    }

    /// L5's table ([`crate::oblgen::alternatives`]), read at root's fixpoint
    /// on a monotone program that states an obligation, and handed to the
    /// node function so every node names the instances it owes. `None` on
    /// any other program.
    pub(crate) fn alternatives(
        &mut self,
    ) -> Result<Option<std::sync::Arc<crate::oblgen::Alternatives>>, SolveError> {
        if !self.monotonicity.is_monotone() || self.root.program().obligations.is_empty() {
            return Ok(None);
        }
        let candidates: FxHashMap<FactId, u32> = self
            .facts
            .iter()
            .enumerate()
            .map(|(i, &f)| (f, i as u32))
            .collect();
        let table = std::sync::Arc::new(crate::oblgen::alternatives(
            self.root,
            self.terms,
            self.ast,
            &self.memo,
            &self.monotonicity,
            &candidates,
        )?);
        self.axes = self.axes.clone().owing_of(table.clone());
        Ok(Some(table))
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
    /// the whole cube, and is not certified. Describe's, and since M1e2
    /// S1e2.2.26 (W27) every line mode's past the limit too
    /// (`certified_line`): the one rule, so the two cannot disagree.
    ///
    /// With `nogoods`, each member of `N` met is given its core after the
    /// walk, through the node function against the walk's snapshot (M1f
    /// S1f.5.12, P-15 (c)): the walk keeps a node as its two bits, so a member
    /// is taken then even where the walk took a core. Without it a member
    /// carries none, and no surface that prints `N` reads it.
    fn partial_cube(&mut self, nogoods: bool) -> Result<Described, SolveError> {
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
        // N's cores, after the walk (M1f S1f.5.12). Root's is root's node's.
        let taken: Vec<region::Node> = if nogoods {
            let sets: Vec<Vec<FactId>> = minimal_dead
                .iter()
                .filter(|d| !d.is_empty())
                .map(|d| d.iter().map(|&b| self.facts[b as usize]).collect())
                .collect();
            region::cores_after(
                self.pool.as_ref(),
                self.root,
                &self.snapshot,
                self.terms,
                self.ast,
                &self.memo,
                &self.axes,
                &sets,
            )?
        } else {
            Vec::new()
        };
        let mut taken = taken.into_iter();
        let root = self.seen.get(&[][..]);
        let node = self.root_node.clone().unwrap_or_default();
        let nogoods_met: Vec<described::Dead> = minimal_dead
            .iter()
            .map(|d| {
                let why = if d.is_empty() {
                    Some(node.clone())
                } else if nogoods {
                    taken.next()
                } else {
                    None
                };
                match why {
                    Some(w) => described::Dead {
                        set: Box::from(*d),
                        core: w.core,
                        core_status: w.core_status,
                        refuted_by: w.refuted_by,
                    },
                    None => described::Dead {
                        set: Box::from(*d),
                        core: Vec::new(),
                        core_status: None,
                        refuted_by: Vec::new(),
                    },
                }
            })
            .collect();
        Ok(Described {
            candidates: self.candidates.clone(),
            monotonicity: self.monotonicity.clone(),
            route: Route::Cube,
            exhausted: false,
            complete_up_to: None,
            stopped_before: None,
            states_partial: None,
            root: described::Root {
                alive: root.is_some_and(|r| !r.dead) || (root.is_none() && !self.root_dead),
                owes: root.map_or(0, |r| r.owes),
                owed: node.owed,
                core: node.core,
                core_status: node.core_status,
                refuted_by: node.refuted_by,
                sources: node.sources,
            },
            any_alive: !alive.is_empty(),
            k_c: members.len() as u64,
            minimal_commitments,
            nogoods: minimal_dead.len() as u64,
            dead: nogoods_met,
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
        })
    }
}

/// Steps of 1, 2, 4, … up to [`BATCH`] nodes.
pub(crate) fn doubling<T>(items: &[T]) -> impl Iterator<Item = &[T]> {
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
    let mut prepared = prepared;
    let root_stats = std::mem::take(&mut prepared.root);
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
            cores: opts.cores,
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
        partial: None,
        states,
        minimal: Vec::new(),
        verdict: None,
        stopped,
        saturations,
        facts,
        wall: Duration::ZERO,
        warning,
        root: root_stats,
    })
}

/// Describe all of `R` by `route` — the state route or route 1 — whatever
/// the rule would pick: for the tests that hold two routes to one read-out.
/// `ein solve` never calls it; describe's route is the rule's (the module's
/// § Describe's route). A flagged program is refused unless its root is dead.
pub fn describe_by(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    route: Route,
) -> Result<Answer, SolveError> {
    let t0 = Instant::now();
    let prepared = region::prepare_with(root, terms, ast, &mut Events::off())?;
    let mut answer = match route {
        Route::States if prepared.monotonicity.is_monotone() => {
            describe_states(root, terms, ast, prepared, t0, opts, &mut Events::off())?
        }
        Route::DownSet if prepared.monotonicity.is_monotone() || prepared.root_dead => {
            describe_route_1(root, terms, ast, prepared, t0, opts, &mut Events::off())?
        }
        _ => {
            return Err(SolveError::Unsupported(format!(
                "describe_by: route {route:?} does not take this program"
            )));
        }
    };
    answer.wall = t0.elapsed();
    Ok(answer)
}

/// Describe all of `R` on a monotone program by the state route: every alive
/// state once, `R` read off the states. Where a node contradicts the static
/// verdict, the run refuses, as route 1's does.
fn describe_states(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    prepared: Prepared,
    t0: Instant,
    opts: &Options,
    events: &mut Events,
) -> Result<Answer, SolveError> {
    let mut prepared = prepared;
    let root_stats = std::mem::take(&mut prepared.root);
    let (s, after) = crate::states::compute_resumable(
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
            cores: opts.cores,
        },
        events,
    )?;
    let stopped = match &s.declined {
        None => None,
        Some(Declined::Budget {
            spent, projected, ..
        }) => Some(Stopped::Budget {
            spent: *spent,
            projected: *projected,
        }),
        Some(Declined::Contradicted { .. } | Declined::NotMonotone) => {
            return Err(SolveError::Unsupported(
                "the state route saw a node contradict the static verdict; run `ein oracle` on \
                 this program"
                    .to_string(),
            ));
        }
    };
    let d = crate::states::describe(&s)?;
    let mut described = described::of_states(&s, &d);
    // N's cores, after the walk (M1f S1f.5.12): the route reads `N` off its
    // maximal alive states, so a member it did not saturate, or saturated
    // without taking a core, is taken now, through the node function against
    // the walk's snapshot.
    if opts.cores.nogoods
        && let Some(after) = after.as_ref()
    {
        let at: Vec<usize> = (0..described.dead.len())
            .filter(|&i| {
                let x = &described.dead[i];
                x.core_status.is_none() && !x.set.is_empty()
            })
            .collect();
        let sets: Vec<Vec<FactId>> = at
            .iter()
            .map(|&i| {
                described.dead[i]
                    .set
                    .iter()
                    .map(|&b| s.candidates[b as usize].fact)
                    .collect()
            })
            .collect();
        let taken = after.cores(root.sealed(), terms, ast, &sets)?;
        for (i, node) in at.into_iter().zip(taken) {
            let x = &mut described.dead[i];
            x.core = node.core;
            x.core_status = node.core_status;
            x.refuted_by = node.refuted_by;
        }
    }
    let mut licences = vec![Licence::DeadUpward, Licence::Closure, Licence::LocalLines];
    if d.cube && !d.exhausted {
        // ∅ and ⊤ in R decide that R is the cube: L2 is what says every
        // node above ∅ is discharged.
        licences.insert(1, Licence::DischargedUpward);
    }
    let set_of = |i: u32| s.states[i as usize].closure.clone();
    let shape = d.exhausted.then(|| Shape {
        empty_in_r: !s.root.dead && s.root.owes == 0,
        minimal: d.minimal.iter().map(|(g, _)| g.clone()).collect(),
        maximal: d.maximal.iter().map(|&i| set_of(i)).collect(),
        components: d.components.as_ref().map_or(0, Vec::len),
        subcubes: d.subcubes.as_ref().map(Vec::len),
        nogoods: d.nogoods.as_ref().map_or(0, Vec::len),
        holes: None,
        merged: d
            .r
            .iter()
            .filter(|&&i| d.commitments[i as usize].is_some_and(|c| c > 1))
            .count(),
    });
    let states =
        d.r.iter()
            .map(|&i| {
                let st = &s.states[i as usize];
                State {
                    set: d.generators[i as usize]
                        .first()
                        .cloned()
                        .unwrap_or_else(|| st.from.clone()),
                    closure: st.closure.clone(),
                }
            })
            .collect();
    Ok(Answer {
        mode: opts.mode,
        candidates: s.candidates.clone(),
        monotonicity: s.monotonicity.clone(),
        root_dead: s.root.dead,
        route: Route::States,
        licences,
        exhausted: d.exhausted,
        shape,
        counts: Some(Counts {
            k_c: described.k_c,
            k_s: described.k_s(),
        }),
        described: Some(Box::new(described)),
        partial: None,
        states,
        minimal: Vec::new(),
        verdict: None,
        stopped,
        saturations: s.saturations(),
        facts: s.facts(),
        wall: Duration::ZERO,
        warning: None,
        root: root_stats,
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
    // The ∅ rule (M1f S1f.5.6): a minimal question, under any ask, where ∅
    // is in `R` — root is alive here, so where it owes nothing — is decided
    // by root's node, within the limit or past it (`Space::empty_state`).
    let empty_in_r = opts.mode.which == Which::Minimal && prepared.root_owes == 0;
    if early || !within || empty_in_r {
        // Layer order, stopping when the question is decided. Past the limit
        // a question that needs the whole cube stays undecided. The cube is
        // bounded by the limit, so the facts budget does not apply.
        let mut sp = Space::open(root, terms, ast, prepared, opts, events);
        sp.budget = u64::MAX;
        let mut found = if empty_in_r {
            sp.empty_state(opts.mode.ask)?
        } else {
            let which = if early { opts.mode.which } else { Which::All };
            let ask = if early {
                opts.mode.ask
            } else {
                Ask::Find(u64::MAX)
            };
            sp.cube_in_order(which, ask, opts.limit.min(63))?
        };
        if !early && !empty_in_r {
            // Members of `R` found, the members met in layer order up to the
            // limit: the lines are the whole cube's, and only what describe
            // certifies of them is a line member (below).
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
        // A line mode past the limit (`!early` here is past it) answers from
        // describe's partial description, by its rule, so the two certify the
        // same line members (M1e2 S1e2.2.26, W27; Q-M1e2.19). Until then it
        // labelled every member met with its line.
        let line_past_limit = !early && !empty_in_r && opts.mode.which != Which::All;
        // Only describe prints `N`, so only describe takes its cores.
        let partial = if describe_all || line_past_limit {
            Some(Box::new(
                sp.partial_cube(describe_all && opts.cores.nogoods)?,
            ))
        } else {
            None
        };
        if line_past_limit && let Some(d) = partial.as_deref() {
            let (states, minimal) = certified_line(d, opts.mode);
            found.states = states;
            found.minimal = minimal;
        }
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
        if describe_all {
            answer.described = partial;
        } else {
            answer.partial = partial;
        }
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
            cores: opts.cores,
        },
        &mut |_| {},
        events,
    )?;
    let Outcome::Region(r) = outcome else {
        unreachable!("within the limit");
    };
    Ok(of_cube(opts.mode, *r))
}

/// A minimal commitment and the index of its state in a mode's states, as
/// [`Answer::minimal`] holds them.
type MinimalAt = (Box<[u32]>, usize);

/// A line mode's states past `--region-limit` on a flagged program, read off
/// describe's partial description `d` ([`Space::partial_cube`]), so that the
/// mode certifies what describe certifies and nothing more (M1e2 S1e2.2.26,
/// W27). On the minimal line, the states `d` marks minimal, each with the
/// minimal commitments that reach it, in layer order: the order the minimal
/// line has within the limit ([`of_cube`]). The maximal line needs the whole
/// cube, so `d` certifies no state on it, and none is listed. `-n N` keeps the
/// first `N`; `--single` reaches here on the maximal line alone, which it
/// cannot decide.
fn certified_line(d: &Described, mode: Mode) -> (Vec<State>, Vec<MinimalAt>) {
    let mut states: Vec<State> = Vec::new();
    let mut minimal: Vec<MinimalAt> = Vec::new();
    match mode.which {
        Which::Minimal => {
            // Each minimal commitment is a generator of its own state, and of
            // no other: nothing in `R` is below it, so no earlier generator.
            let mut of: FxHashMap<&[u32], usize> = FxHashMap::default();
            for (i, s) in d.states.iter().enumerate().filter(|(_, s)| s.minimal) {
                for g in &s.generators {
                    of.insert(g, i);
                }
            }
            let mut at: FxHashMap<usize, usize> = FxHashMap::default();
            // `minimal_commitments` is in layer order, then by candidates.
            for m in &d.minimal_commitments {
                let Some(&i) = of.get(&m[..]) else {
                    continue;
                };
                let next = states.len();
                let k = *at.entry(i).or_insert(next);
                if k == next {
                    states.push(State {
                        set: m.clone(),
                        closure: d.states[i].closure.clone(),
                    });
                }
                minimal.push((m.clone(), k));
            }
        }
        Which::Maximal => {
            for s in d.states.iter().filter(|s| s.maximal) {
                states.push(State {
                    set: s.rep.clone(),
                    closure: s.closure.clone(),
                });
            }
        }
        Which::All => {}
    }
    if let Ask::Find(want) = mode.ask {
        states.truncate(usize::try_from(want).unwrap_or(usize::MAX));
        minimal.retain(|(_, k)| (*k as u64) < want);
    }
    (states, minimal)
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
        // `--count` reaches the cube as describe: `run` asks describe.
        Ask::Describe | Ask::Count => {}
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
        partial: None,
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
        root: r.root,
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
    /// What root's saturation and the universe's pass counted, as every route
    /// has them: `--json-summary`'s `root` block.
    pub root: RootStats,
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
        root: p.root,
    })
}
