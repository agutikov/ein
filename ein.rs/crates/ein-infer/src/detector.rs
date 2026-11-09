//! The detector: what a program the static check flags is warned of — M1e2
//! [S1e2.1b.5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md),
//! the design the user approved on 2026-11-03
//! ([`detector_proposal.md`](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md)).
//!
//! [`crate::monotone::classify`] answers *is this program monotone?* from the
//! compiled plans. On a program it flags, this module says more, in the words
//! of `docs/kernel/inference/monotonicity.md` § 7: which property of a node
//! each site puts at stake (the **state**, or the **status**), whether a
//! **cycle** runs through an `absent` over `D`, and whether the boundary's
//! **order** of admission can decide what a node holds. That is the static
//! half, [`Static`], read off root's plans at load and carried on
//! [`crate::monotone::Monotonicity`].
//!
//! The dynamic half, [`Warning`], says whether a site bit, and reads only what
//! the run computes anyway, with the **status** and the **state** reported
//! apart because the cube reads only the first:
//!
//! - **the status**: the cube's violations — holes, dead nodes with an alive
//!   superset, discharged nodes with an owing superset — each attributed to
//!   the sites that made it (S1e2.1b.3's rule, `status_of`; where it names
//!   none, since M1e2 S1e2.2.17, the derivation followed through each plan's
//!   activator, and where that names none either, [`by_words`] says so):
//!   *exercised* or *conservative*, and *not known* where the run did not read
//!   the cube;
//! - **the state**: every node the run computes is read for **stale facts**
//!   ([`stale_facts`], `node_census.md` § 4's check, in
//!   [`crate::region`]'s node function), dead nodes included, because W8's
//!   stale fact is in a dead node no run lists;
//! - **the paths**: the read-out's parent-path spot check
//!   ([`crate::region::Nodes::by_parent`]), at most [`SPOT_PARENTS`] parents
//!   of each state it prints of two commitments or more. It reads one-step
//!   parents only: on the probe `via_hole` and on `ACTIVATOR_UNDER_ABSENT` a
//!   node is dead along a two-step chain through an alive parent, and one-step
//!   parents see one of the two nodes (`node_census.md` § 11.2). Its row says
//!   what was checked, and claims nothing more.
//!
//! It decides nothing. What a node is on a flagged program, and what is
//! reported about it, are
//! [F21](../../../../plans/followups/f21_non_monotone_programs_decided.md)'s
//! D1–D7: no finding here chooses a node, refuses a program or moves `R`.
//!
//! ### Why it is gated, and why the gate costs nothing it should not
//!
//! Every check here runs on a program `classify` flags, and only there. No
//! option turns it on or off (the config principle, 2026-10-15): it runs by a
//! rule that reads the program, as describe's route does.
//!
//! **On an unflagged program the detector has nothing to find**, on every node
//! built by fork from root — which is how every shipped route builds one — and
//! on every split `sat(sat(P ∪ C1) ∪ (C \ C1))` from a fixpoint that already
//! holds root's. The argument is `monotonicity.md` § 5.1's theorem: a program
//! with no site reads no relation in `D` under an odd `absent`, so its graph
//! over `D` has no negative edge (no cycle through an `absent` over `D`), no
//! `absent` rule reads under an `absent` a relation in `D` another concludes
//! (no order finding), every relation in `D` grows with the commitment (no
//! fact's guard is falsified, so none is stale), and a static read is settled
//! at root, where every fork inherits its result (no path differs).
//!
//! **It does not hold from scratch.** A node built from the program with the
//! commitment written in re-derives root inside itself, and there an
//! equal-priority race over a static read can leave a stale fact on a program
//! the check calls monotone: `examples/lattice/11_fifo_gap_static.ein`, whose
//! `{(b one)}` holds a stale `(p one)` from scratch and none by fork (M1e2
//! S1e2.1b.7, `node_census.md` § 12.2). No shipped route builds from scratch,
//! so the gate is sound for what ships, and the scope is the claim.
//!
//! Under [`standard_of_proof.md`](../../../../docs/kernel/standard_of_proof.md)'s
//! rule 2 the argument is enough only while its premise is enforced, and the
//! premise is enforced by the tests of the stage's (c), which fail when it stops
//! being true:
//!
//! - `ein-infer/tests/region.rs::fact_grain_never_calls_a_violated_cube_monotone`
//!   holds a *Monotone* verdict to no cube violation on every corpus file and
//!   register witness within the oracle's limit, and `utils/region_synth.py`'s
//!   check on the random programs `ein-cli/tests/region_dump.rs` draws;
//! - `ein-infer/tests/detector.rs::a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split`
//!   builds every node of every monotone corpus cube with `n ≤ 10` by fork, by
//!   every split from a fixpoint that holds root's, and from scratch, and
//!   reads each for stale facts: one state per node, no stale fact, and
//!   scratch departing on `11_fifo_gap_static`'s `{(b one)}` alone; its
//!   sibling `…_on_random_programs` does the same on random programs;
//! - the same file holds an empty [`Static`] on every monotone corpus program,
//!   and the order check to no false negative on the FIFO races.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use ein_core::prov::{NafArg, NafRef};
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::SharedMemo;
use crate::error::SolveError;
use crate::events::{Events, sexpr};
use crate::match_::Matcher;
use crate::monotone::{self, Monotonicity, SiteKind};
use crate::obligations::{self, Owed};
use crate::plan::{Disjunct, NafGuard, Plan, Slot, Span, Step};
use crate::region::{Candidate, DEAD, Node, Nodes, OWING, Topology};
use crate::strata;

/// What the warning says is assumed, on every flagged program: the node is
/// built by fork from root, and what it should be is open.
pub const ASSUMES: &str = "every node is a fork of root (root's fixpoint, its commitment added, \
                           saturated); what a node is on this program is open: Q-M1e2.2, F21";

/// What a site's rule concludes, as the warning names it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Concludes {
    /// `(false)`: a refutation.
    False,
    /// A stored negative, `(not …)`: a refutation one edge up.
    Negative,
    /// A positive fact.
    Positive,
    /// An obligation's debt, `(open …)`.
    Open,
    /// An activator site whose rule root compiled no plan of: what it
    /// concludes is in a plan only a node compiles.
    NoRootPlan,
}

impl Concludes {
    pub fn as_str(self) -> &'static str {
        match self {
            Concludes::False => "(false)",
            Concludes::Negative => "(not …)",
            Concludes::Positive => "a positive fact",
            Concludes::Open => "(open …)",
            Concludes::NoRootPlan => "root compiled no plan of it",
        }
    }
}

/// Which property of a node a site puts at stake — `monotonicity.md` § 7.6's
/// table, read per site: an `absent` puts the **state** at stake, and `dead`
/// with it when it refutes; a trigger puts the **status** (`owes`); an
/// activator, either, through the plan a node compiles.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AtStake {
    State,
    StateAndDead,
    Owes,
    OwesThroughPlan,
    StateThroughPlan,
}

impl AtStake {
    /// The read-out's words.
    pub fn words(self) -> &'static str {
        match self {
            AtStake::State => "the state",
            AtStake::StateAndDead => "the state, and dead with it",
            AtStake::Owes => "the status: owes",
            AtStake::OwesThroughPlan => "the status: owes, through a plan a node compiles",
            AtStake::StateThroughPlan => "the state, through a plan a node compiles",
        }
    }

    /// The value whose [`Self::key`] is `key`.
    pub fn from_key(key: &str) -> Option<AtStake> {
        [
            AtStake::State,
            AtStake::StateAndDead,
            AtStake::Owes,
            AtStake::OwesThroughPlan,
            AtStake::StateThroughPlan,
        ]
        .into_iter()
        .find(|a| a.key() == key)
    }

    /// The summary's and the event's key.
    pub fn key(self) -> &'static str {
        match self {
            AtStake::State => "state",
            AtStake::StateAndDead => "state, dead",
            AtStake::Owes => "status: owes",
            AtStake::OwesThroughPlan => "status: owes, through a node's plan",
            AtStake::StateThroughPlan => "state, through a node's plan",
        }
    }
}

/// One site of [`Monotonicity::sites`], read further: what its rule concludes
/// and which property it puts at stake.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stake {
    /// The site, as [`monotone::Site::render`] writes it.
    pub site: String,
    pub kind: SiteKind,
    pub concludes: Concludes,
    pub at_stake: AtStake,
}

impl Stake {
    /// The `site` row: the site, what its rule concludes, what it puts at
    /// stake.
    pub fn text(&self) -> String {
        let what = if self.kind == SiteKind::Activator && self.concludes == Concludes::NoRootPlan {
            format!("; {}", self.concludes.as_str())
        } else if self.kind == SiteKind::Activator {
            format!(
                "; a plan root compiled of it concludes {}",
                self.concludes.as_str()
            )
        } else {
            format!(", concludes {}", self.concludes.as_str())
        };
        format!("{}{what} — at stake: {}", self.site, self.at_stake.words())
    }

    /// `rule [activator …]`: the site's text before its colon, what a status
    /// names.
    pub fn name(&self) -> &str {
        self.site.split(':').next().unwrap_or(&self.site)
    }
}

pub use crate::strata::Cycle;

/// An order finding: a deferred rule, `reader`, reads under an `absent` a
/// relation in `D` that another deferred rule, `writer`, concludes — directly,
/// or through rules that are not deferred. The boundary admits one parked
/// candidate a round in FIFO order, the written and arrival order (no rule
/// has a priority since M1f S1f.5.15), so the written order decides which
/// goes first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Race {
    pub reader: String,
    pub writer: String,
    /// The relations raced over, sorted.
    pub relations: Vec<String>,
}

impl Cycle {
    /// The `cycle` row.
    pub fn text(&self) -> String {
        format!(
            "{} through an absent — {} — {}",
            if self.odd { "odd" } else { "even" },
            self.negative.join(" · "),
            if self.odd {
                "a state with no stale fact need not exist"
            } else {
                "several states with no stale fact can exist"
            }
        )
    }
}

/// The `order` rows: one per finding, a mutual pair as one.
pub fn order_texts(order: &[Race]) -> Vec<String> {
    let pairs: BTreeSet<(&str, &str)> = order
        .iter()
        .map(|r| (r.reader.as_str(), r.writer.as_str()))
        .collect();
    let mut said: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut out = Vec::new();
    for r in order {
        let (reader, writer) = (r.reader.as_str(), r.writer.as_str());
        if said.contains(&(writer, reader)) {
            continue;
        }
        said.insert((reader, writer));
        let mutual = pairs.contains(&(writer, reader));
        out.push(if mutual {
            format!(
                "{reader} and {writer} each read under an absent what the other concludes: which \
                 the boundary admits first is the order they are written in"
            )
        } else {
            format!(
                "{reader} reads {} under an absent and {writer} concludes it: which the boundary \
                 admits first is the order they are written in",
                r.relations.join(", ")
            )
        });
    }
    out
}

/// The static half of the detector, on a flagged program; empty on an
/// unflagged one, which it never reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Static {
    /// One per [`Monotonicity::sites`], in its order.
    pub stakes: Vec<Stake>,
    /// The cycles through an `absent` over `D`, sorted by their relations.
    pub cycles: Vec<Cycle>,
    /// The order findings, sorted by reader then writer.
    pub order: Vec<Race>,
}

/// The static half, from root's plans — what `classify` read — and its
/// verdict. **Gated**: a program with no site gets [`Static::default`] and no
/// work beyond the test.
pub fn of_program(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>], m: &Monotonicity) -> Static {
    if m.is_monotone() {
        return Static::default();
    }
    let program = kb.program();
    let stakes = m
        .sites
        .iter()
        .map(|s| {
            let mine: Vec<&Arc<Plan>> = plans.iter().filter(|p| p.rule == s.rule).collect();
            let concludes = if s.kind == SiteKind::ObligationTrigger {
                Concludes::Open
            } else if mine.is_empty() {
                Concludes::NoRootPlan
            } else {
                mine.iter()
                    .find_map(|p| refutes(p, terms))
                    .unwrap_or(Concludes::Positive)
            };
            let at_stake = match s.kind {
                SiteKind::Absent if concludes == Concludes::Positive => AtStake::State,
                SiteKind::Absent => AtStake::StateAndDead,
                SiteKind::ObligationTrigger => AtStake::Owes,
                SiteKind::Activator if program.obligations.contains(s.rule) => {
                    AtStake::OwesThroughPlan
                }
                SiteKind::Activator => AtStake::StateThroughPlan,
            };
            Stake {
                site: s.render(terms),
                kind: s.kind,
                concludes,
                at_stake,
            }
        })
        .collect();
    let dynamic: BTreeSet<String> = m.dynamic.iter().map(|r| r.label(terms)).collect();
    Static {
        stakes,
        cycles: cycles_over(kb, terms, plans, &dynamic),
        order: order(kb, terms, plans, &dynamic),
    }
}

/// `(false)` or `(not …)` when one of `plan`'s conclusions refutes.
fn refutes(plan: &Plan, terms: &Terms) -> Option<Concludes> {
    let w = monotone::writes_of(plan, terms);
    if w.iter()
        .any(|r| !r.negated && r.name == terms.kernel.r#false)
    {
        Some(Concludes::False)
    } else if w.iter().any(|r| r.negated) {
        Some(Concludes::Negative)
    } else {
        None
    }
}

/// The saturation rules' dependency graph over `D` (`node_census.md` § 5's):
/// [`strata::edges`] restricted to the relations in `D`. Each strongly
/// connected component holding a negative edge, with its parity.
fn cycles_over(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    dynamic: &BTreeSet<String>,
) -> Vec<Cycle> {
    let edges = strata::edges(kb, terms, plans);
    let over_d: Vec<&strata::Edge> = edges
        .iter()
        .filter(|(a, b, _, _)| dynamic.contains(a) && dynamic.contains(b))
        .collect();
    let mut out = strata::components(&over_d);
    out.sort_by(|a, b| a.relations.cmp(&b.relations));
    out
}

/// The order check. A saturation rule is **deferred** when a disjunct carries
/// an `absent` guard: the boundary admits its candidates one a round, in
/// FIFO order. A deferred rule A that reads under an `absent` a
/// relation in `D` which a deferred rule B concludes — directly, or through
/// rules that are not deferred, which fire before the boundary speaks — can be
/// admitted before B and keep a fact B's conclusion then falsifies (the FIFO
/// gap, `monotonicity.md` § 7.3). At one priority the written order decides
/// which goes first; with A's priority earlier, A goes first in every order
/// (`fifo_gap_priority`, the proposal's § 7). B earlier is the stratified
/// order.
fn order(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>], dynamic: &BTreeSet<String>) -> Vec<Race> {
    let program = kb.program();
    let sat: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let deferred = |p: &Plan| p.disjuncts.iter().any(|d| !d.guards.is_empty());
    // What each relation reaches through rules that are not deferred.
    let mut forward: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in sat.iter().filter(|p| !deferred(p)) {
        let w: Vec<String> = monotone::writes_of(p, terms)
            .into_iter()
            .map(|r| r.label(terms))
            .collect();
        for (r, d) in monotone::reads_of(p, terms) {
            if d % 2 == 0 {
                forward
                    .entry(r.label(terms))
                    .or_default()
                    .extend(w.iter().cloned());
            }
        }
    }
    let reach = |start: Vec<String>| -> BTreeSet<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut stack = start;
        while let Some(x) = stack.pop() {
            if seen.insert(x.clone())
                && let Some(next) = forward.get(&x)
            {
                stack.extend(next.iter().cloned());
            }
        }
        seen
    };
    let mut out: BTreeMap<(String, String), Race> = BTreeMap::new();
    for a in sat.iter().filter(|p| deferred(p)) {
        let reads: BTreeSet<String> = monotone::reads_of(a, terms)
            .into_iter()
            .filter(|(_, d)| d % 2 == 1)
            .map(|(r, _)| r.label(terms))
            .filter(|l| dynamic.contains(l))
            .collect();
        for b in sat.iter().filter(|p| deferred(p) && p.rule != a.rule) {
            let concludes = reach(
                monotone::writes_of(b, terms)
                    .into_iter()
                    .map(|r| r.label(terms))
                    .collect(),
            );
            let raced: Vec<String> = reads.intersection(&concludes).cloned().collect();
            if raced.is_empty() {
                continue;
            }
            let key = (terms.sym(a.rule).to_string(), terms.sym(b.rule).to_string());
            let race = out.entry(key.clone()).or_insert_with(|| Race {
                reader: key.0.clone(),
                writer: key.1.clone(),
                relations: Vec::new(),
            });
            for r in raced {
                if !race.relations.contains(&r) {
                    race.relations.push(r);
                }
            }
            race.relations.sort();
        }
    }
    out.into_values().collect()
}

// ── The dynamic half ────────────────────────────────────────────────

/// What the cube says of the **status**, `dead` and `owes`: whether the
/// flagged sites broke an upward closure on this program's own lattice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// The cube shows a violation: non-monotone here, by the sites the
    /// violations are attributed to (S1e2.1b.3's rule).
    Exercised {
        by: Vec<String>,
        holes: u64,
        dead_not_upward_closed: u64,
        discharged_not_upward_closed: u64,
    },
    /// The cube shows none: monotone here, the flags conservative.
    Conservative,
    /// The cube is past `--region-limit`.
    PastLimit { n: usize, limit: u32 },
    /// Within the limit, and this run did not read every node.
    NotRead { n: usize },
    /// Root is dead, so every node is (L0), and the run read root alone.
    RootDead { n: usize, limit: u32 },
}

impl Status {
    /// `exercised`, `conservative` or `not known`: the summary's word.
    pub fn word(&self) -> &'static str {
        match self {
            Status::Exercised { .. } => "exercised",
            Status::Conservative => "conservative",
            _ => "not known",
        }
    }

    /// The cube's holes, where the status read the whole cube.
    pub fn holes(&self) -> Option<u64> {
        match self {
            Status::Exercised { holes, .. } => Some(*holes),
            Status::Conservative => Some(0),
            _ => None,
        }
    }
}

/// The sites an `Exercised` status names, as its row says them: `by a, b`,
/// or, where the attribution reaches none, why (M1e2 S1e2.2.17).
pub fn by_words(by: &[String]) -> String {
    if by.is_empty() {
        "by no site: the violation's derivation, each plan's activator followed, reaches none"
            .to_string()
    } else {
        format!("by {}", by.join(", "))
    }
}

/// One node holding a stale fact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaleNode {
    /// `{a, b}` or `∅`, in candidate order.
    pub node: String,
    /// What the node is: in R, owing or dead.
    pub class: crate::region::Class,
    /// Its stale facts, `fact ← rule (absent …)`, sorted.
    pub facts: Vec<String>,
}

/// What the stale check read: every node the run computed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// The run computed the whole cube.
    pub cube: bool,
    /// How many nodes it computed, and the check read.
    pub nodes: u64,
    /// The nodes that hold a stale fact, in the order the run computed them.
    pub stale: Vec<StaleNode>,
}

/// The parent-path spot check: for each state the read-out prints of two
/// commitments or more, up to [`SPOT_PARENTS`] of its parents `C \ {h}`, or
/// every one of them under `-y` (M1e2 S1e2.1c.5, the user's ruling of
/// 2026-11-03), which runs it on a monotone program too.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Paths {
    /// Parents read per printed state: [`SPOT_PARENTS`], or `None` under
    /// `-y`, every one.
    pub bound: Option<usize>,
    /// Printed states of two commitments or more.
    pub states: u64,
    /// Parents checked: alive, built, `h` written, saturated.
    pub parents: u64,
    /// Parents skipped because they are dead, as `-y` skipped one.
    pub dead: u64,
    pub saturations: u64,
    /// `{state} by way of its parent without h`, one per path that reaches
    /// another state.
    pub differ: Vec<String>,
    /// When the read-out prints no state of two commitments or more and the
    /// run met some it does not print: how many states it met, and what
    /// checking theirs would cost at most.
    pub unprinted: Option<(u64, u64)>,
}

/// The dynamic half: whether a flagged site bit, in the status (the cube) and
/// in the state (stale facts, parent paths). Reported apart, because the cube
/// reads only the status: five programs it calls conservative have a broken
/// state (`monotonicity.md` § 7.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    pub status: Status,
    pub state: State,
    /// Filled by the read-out, which knows what it prints.
    pub paths: Option<Paths>,
}

/// How many parents of each printed state the spot check reads — the user's
/// approval of 2026-11-03. `-y` lifts it ([`Paths::bound`]).
pub const SPOT_PARENTS: usize = 3;

// ── The stale check ─────────────────────────────────────────────────

/// What the stale check reads a node against: root's plans, by rule. Built
/// once per run, on a flagged program only.
#[derive(Debug, Default)]
pub struct StaleCheck {
    by_rule: FxHashMap<Symbol, Vec<Arc<Plan>>>,
}

impl StaleCheck {
    /// Over `plans`, root's: what `classify` read.
    pub fn of(plans: &[Arc<Plan>]) -> StaleCheck {
        let mut by_rule: FxHashMap<Symbol, Vec<Arc<Plan>>> = FxHashMap::default();
        for p in plans {
            by_rule.entry(p.rule).or_default().push(p.clone());
        }
        StaleCheck { by_rule }
    }
}

/// Every fact of `kb` whose primary derivation's `absent` guard `kb` itself
/// now falsifies, as `fact ← rule (absent …)`, sorted — `node_census.md`
/// § 4's check. For each fact whose primary provenance records `absent`
/// patterns, the plan and the disjunct that record exactly those patterns,
/// with the saturator's registers rebuilt from the provenance's bindings; then
/// the engine's matcher, [`Matcher::holds`], asks whether the guard's query now
/// has a match. Where no plan reproduces the recorded patterns (a plan only a
/// node compiled), the recorded patterns are read directly.
pub fn stale_facts(kb: &Kb, terms: &Terms, ast: &Ast, check: &StaleCheck) -> Vec<String> {
    let mut m = Matcher::new();
    let mut out = Vec::new();
    for f in kb.facts() {
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        if prov.absent.is_empty() {
            continue;
        }
        let Some(rule) = prov.rule else { continue };
        let mut verdict: Option<bool> = None;
        if let Some(plans) = check.by_rule.get(&rule) {
            'plans: for plan in plans {
                let regs = regs_of(plan, &prov.bindings);
                for d in plan.disjuncts.iter() {
                    if d.guards.is_empty() || refs_of(plan, d, &regs).as_slice() != &*prov.absent {
                        continue;
                    }
                    verdict = Some(
                        plan.guards(d.guards)
                            .iter()
                            .any(|g| m.holds(kb, terms, ast, plan, g, &regs)),
                    );
                    break 'plans;
                }
            }
        }
        let falsified =
            verdict.unwrap_or_else(|| prov.absent.iter().any(|r| naf_holds(kb, terms, r)));
        if falsified {
            let guards: Vec<String> = prov
                .absent
                .iter()
                .map(|r| format!("(absent {})", naf_text(terms, r.rel, &r.args)))
                .collect();
            out.push(format!(
                "{} ← {} {}",
                sexpr(terms, f),
                terms.sym(rule),
                guards.join(" ")
            ));
        }
    }
    out.sort();
    out
}

/// The saturator's registers for `plan`, rebuilt from a provenance's bindings.
fn regs_of(plan: &Plan, bindings: &[(Symbol, Value)]) -> Vec<Value> {
    let mut regs = vec![Value::UNBOUND; plan.n_regs as usize];
    for (i, (_, v)) in plan.seed.iter().enumerate() {
        regs[i] = *v;
    }
    for (name, v) in bindings {
        if let Some(r) = plan.reg_names.iter().position(|n| n == name) {
            regs[r] = *v;
        }
    }
    regs
}

/// The `absent` patterns `d`'s guards record under `regs` — what the
/// saturator writes into a provenance's `absent`.
fn refs_of(plan: &Plan, d: &Disjunct, regs: &[Value]) -> Vec<NafRef> {
    fn ground(plan: &Plan, g: &NafGuard, slot: &Slot, regs: &[Value]) -> NafArg {
        match slot {
            Slot::Reg(r) => match g.scope_of[*r as usize] {
                Some(parent) => {
                    let v = regs[parent as usize];
                    if v.is_unbound() || v.as_fact().is_some() {
                        NafArg::Free
                    } else {
                        NafArg::Value(v)
                    }
                }
                None => NafArg::Free,
            },
            Slot::Const(v) => NafArg::Value(*v),
            Slot::Nested { rel, slots } => NafArg::Nested {
                rel: *rel,
                args: plan
                    .slots(*slots)
                    .iter()
                    .map(|s| ground(plan, g, s, regs))
                    .collect(),
            },
            Slot::Opaque(_) => NafArg::Free,
        }
    }
    fn collect(plan: &Plan, g: &NafGuard, span: Span, regs: &[Value], out: &mut Vec<NafRef>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) => out.push(NafRef {
                    rel: r.rel,
                    args: plan
                        .slots(r.slots)
                        .iter()
                        .map(|s| ground(plan, g, s, regs))
                        .collect(),
                }),
                Step::Absent { sub } => collect(plan, g, *sub, regs, out),
                Step::Guard { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for g in plan.guards(d.guards) {
        collect(plan, g, g.sub, regs, &mut out);
    }
    let mut seen: Vec<NafRef> = Vec::new();
    for r in out {
        if !seen.contains(&r) {
            seen.push(r);
        }
    }
    seen
}

/// A recorded pattern matches a fact of `kb`.
fn naf_holds(kb: &Kb, terms: &Terms, r: &NafRef) -> bool {
    fn args_match(terms: &Terms, args: &[Value], pat: &[NafArg]) -> bool {
        args.len() == pat.len()
            && args.iter().zip(pat).all(|(v, p)| match p {
                NafArg::Free => true,
                NafArg::Value(x) => v == x,
                NafArg::Nested { rel, args: ps } => v.as_fact().is_some_and(|f| {
                    let (r2, a2) = terms.facts.get(f);
                    r2 == *rel && args_match(terms, a2, ps)
                }),
            })
    }
    kb.facts_of(r.rel)
        .any(|f| args_match(terms, terms.facts.args(f), &r.args))
}

fn naf_text(terms: &Terms, rel: Symbol, args: &[NafArg]) -> String {
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

// ── The cube's half: the status, attributed ─────────────────────────

/// What the attribution reads of a cube: its status bits, each node's
/// record, its candidates and its topology.
pub(crate) struct Cube<'a> {
    pub status: &'a [u8],
    pub nodes: &'a [Node],
    pub candidates: &'a [Candidate],
    pub topology: &'a Topology,
}

impl Cube<'_> {
    fn n(&self) -> usize {
        self.candidates.len()
    }

    fn in_r(&self, mask: u64) -> bool {
        self.status[mask as usize] == 0
    }

    fn facts_of(&self, mask: u64) -> Vec<FactId> {
        (0..self.n())
            .filter(|b| mask >> b & 1 == 1)
            .map(|b| self.candidates[b].fact)
            .collect()
    }

    /// The largest proper subset of `mask` in R: the node a hole's debt is new
    /// against.
    fn r_below(&self, mask: u64) -> Option<u64> {
        let mut best: Option<u64> = None;
        let mut sub = mask;
        loop {
            sub = sub.wrapping_sub(1) & mask;
            if sub == mask {
                break;
            }
            if self.in_r(sub) && best.is_none_or(|b| sub.count_ones() > b.count_ones()) {
                best = Some(sub);
            }
            if sub == 0 {
                break;
            }
        }
        best
    }
}

/// A site, rendered once, before the node function borrows `Terms`.
pub(crate) struct SiteText {
    kind: SiteKind,
    label: String,
    rule: Symbol,
    rule_text: String,
    activator: Box<[Value]>,
    /// `rule [activator …]`: what the status row names.
    name: String,
}

pub(crate) fn site_texts(m: &Monotonicity, terms: &Terms) -> Vec<SiteText> {
    m.sites
        .iter()
        .map(|s| {
            let text = s.render(terms);
            SiteText {
                kind: s.kind,
                label: s.relation.label(terms),
                rule: s.rule,
                rule_text: terms.sym(s.rule).to_string(),
                activator: s.activator.clone(),
                name: text.split(':').next().unwrap_or(&text).to_string(),
            }
        })
        .collect()
}

/// Root's plans, as `(rule, activator)`: a plan a node compiles is one not
/// here.
pub(crate) struct RootPlans {
    pairs: FxHashSet<(Symbol, Box<[Value]>)>,
    rules: FxHashSet<Symbol>,
}

impl RootPlans {
    pub(crate) fn of(plans: &[Arc<Plan>]) -> RootPlans {
        RootPlans {
            pairs: plans
                .iter()
                .map(|p| (p.rule, p.activator_args.clone()))
                .collect(),
            rules: plans.iter().map(|p| p.rule).collect(),
        }
    }
}

/// `(absent (not (q A)))`'s relation, as [`monotone::Rel::label`] writes it.
fn absent_label(premise: &str) -> String {
    let inner = premise
        .strip_prefix("(absent (")
        .unwrap_or(premise)
        .trim_end_matches(')');
    let mut words = inner.split_whitespace();
    match words.next() {
        Some("not") => {
            let r = words
                .next()
                .map(|w| w.trim_start_matches('(').trim_end_matches(')'))
                .unwrap_or("?");
            if r.is_empty() || r == "_" {
                "(not ?)".to_string()
            } else {
                format!("(not {r})")
            }
        }
        Some(r) => r.to_string(),
        None => "?".to_string(),
    }
}

/// The sites a dead node falls to: the activator site of a refuting rule root
/// never compiled; else the `absent` sites over a premise's relation, those
/// whose rule refuted it where there are any (S1e2.1b.3's rule, the
/// `attribution` example's `dead`). Where it names none, [`status_of`] asks
/// [`dead_chain_sites`] (M1e2 S1e2.2.17). *Refuting* reads the first
/// contradiction's rules (`Node::first_rules`), which the node keeps whether
/// or not its core is taken; a dead node's printed `refuted by` follows its
/// core since M1e2 S1e2.3.10, and the detector's labels are F21's.
fn dead_sites(sites: &[SiteText], root: &RootPlans, node: &Node, named: &mut BTreeSet<String>) {
    let mut found = false;
    for s in sites.iter().filter(|s| s.kind == SiteKind::Activator) {
        if node.first_rules.contains(&s.rule_text) && !root.rules.contains(&s.rule) {
            named.insert(s.name.clone());
            found = true;
        }
    }
    if found {
        return;
    }
    let labels: BTreeSet<String> = node
        .absent_premises
        .iter()
        .map(|p| absent_label(p))
        .collect();
    let over: Vec<&SiteText> = sites
        .iter()
        .filter(|s| s.kind == SiteKind::Absent)
        .filter(|s| {
            labels.contains(&s.label)
                || (s.label == "(not ?)" && labels.iter().any(|x| x.starts_with("(not ")))
        })
        .collect();
    let refuting: Vec<&&SiteText> = over
        .iter()
        .filter(|s| node.first_rules.contains(&s.rule_text))
        .collect();
    if !refuting.is_empty() {
        named.extend(refuting.iter().map(|s| s.name.clone()));
    } else {
        named.extend(over.iter().map(|s| s.name.clone()));
    }
}

/// A dead node [`dead_sites`] names no site of — M1e2 S1e2.2.17: its first
/// contradiction's derivation, followed through each plan's activator (F2's
/// *per activator*, read for a label). A derivation does not record the
/// activator its plan was compiled from, so where a chain of activators runs
/// at a node, the refuting rule is no site and its premises carry no
/// `absent`: on W17's witnesses, studied past *forbid*'s refusal, the rule
/// above named nothing, and the status row read *by* and an empty list.
fn dead_chain_sites(
    nodes: &mut Nodes<'_>,
    sites: &[SiteText],
    root: &RootPlans,
    facts: &[FactId],
) -> Result<BTreeSet<String>, SolveError> {
    nodes.with_fixpoints(&[facts.to_vec()], |kbs, terms| {
        let kb = &kbs[0];
        let Some(first) = crate::contradiction::detect(kb, terms).into_iter().next() else {
            return BTreeSet::new();
        };
        let seeds: Vec<FactId> = [first.positive, Some(first.negative)]
            .into_iter()
            .flatten()
            .collect();
        chain_sites(sites, root, &chain_of(kb, terms, &seeds))
    })
}

/// What a derivation rests on, each plan's activator a premise: the rules
/// that derived a fact on it, and the relations of the `absent` premises
/// recorded on it, as [`monotone::Rel::label`] writes them.
struct Chain {
    rules: FxHashSet<Symbol>,
    absent: BTreeSet<String>,
}

/// The derivations of `seeds` in `kb`, walked through every fact's primary
/// derivation and, for a fact a rule with parameters derived, through that
/// rule's applications in `kb`: the activators its plans were compiled from,
/// which provenance does not name, so every one of them (the
/// `examples/shared/taint.rs` walk's reading).
fn chain_of(kb: &Kb, terms: &Terms, seeds: &[FactId]) -> Chain {
    // determinism-ok: membership only; nothing reads its order.
    let mut rules: FxHashSet<Symbol> = FxHashSet::default();
    let mut absent = BTreeSet::new();
    // determinism-ok: membership only; nothing reads its order.
    let mut seen: FxHashSet<FactId> = FxHashSet::default();
    let mut stack: Vec<FactId> = seeds.to_vec();
    while let Some(f) = stack.pop() {
        if !seen.insert(f) {
            continue;
        }
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        for r in prov.absent.iter() {
            absent.insert(naf_rel_label(terms, r));
        }
        stack.extend(prov.premises.iter().copied());
        let Some(rule) = prov.rule else { continue };
        rules.insert(rule);
        let program = kb.program();
        let has_params = program
            .rules
            .get(rule)
            .or_else(|| program.obligations.get(rule))
            .is_some_and(|r| !r.params.is_empty());
        if has_params {
            stack.extend(kb.rule_apps_by_rule(rule));
        }
    }
    Chain { rules, absent }
}

/// An `absent` premise's relation: `q`, `(not q)`, or `(not ?)`.
fn naf_rel_label(terms: &Terms, r: &NafRef) -> String {
    if r.rel == terms.kernel.not {
        return match r.args.first() {
            Some(NafArg::Nested { rel, .. }) => format!("(not {})", terms.sym(*rel)),
            _ => "(not ?)".to_string(),
        };
    }
    terms.sym(r.rel).to_string()
}

/// The sites a chain names: the activator sites of the rules on it root never
/// compiled; else the `absent` sites over its premises' relations.
fn chain_sites(sites: &[SiteText], root: &RootPlans, chain: &Chain) -> BTreeSet<String> {
    let acts: BTreeSet<String> = sites
        .iter()
        .filter(|s| s.kind == SiteKind::Activator)
        .filter(|s| chain.rules.contains(&s.rule) && !root.rules.contains(&s.rule))
        .map(|s| s.name.clone())
        .collect();
    if !acts.is_empty() {
        return acts;
    }
    sites
        .iter()
        .filter(|s| s.kind == SiteKind::Absent)
        .filter(|s| {
            chain.absent.contains(&s.label)
                || (s.label == "(not ?)" && chain.absent.iter().any(|x| x.starts_with("(not ")))
        })
        .map(|s| s.name.clone())
        .collect()
}

/// The sites an owing node `at` falls to, by each debt new against `below`: a
/// trigger site of the instance; else the activator site of an obligation
/// whose plan root never compiled; else an `absent` site whose rule derived a
/// fact `below` holds and `at` does not (the `attribution` example's
/// `owing`); else, since M1e2 S1e2.2.17, the sites [`chain_sites`] names on
/// the obligation's activator's derivation.
fn owing_sites(
    nodes: &mut Nodes<'_>,
    ast: &Ast,
    sites: &[SiteText],
    root: &RootPlans,
    at: &[FactId],
    below: &[FactId],
    named: &mut BTreeSet<String>,
) -> Result<(), SolveError> {
    let found = nodes.with_fixpoints(&[below.to_vec(), at.to_vec()], |kbs, terms| {
        let owed = |kb: &Kb, terms: &mut Terms| -> Vec<Owed> {
            obligations::tally(kb, terms, ast, &SharedMemo::default(), &mut Events::off())
                .map(|o| o.instances().to_vec())
                .unwrap_or_default()
        };
        let lo = owed(&kbs[0], terms);
        let hi = owed(&kbs[1], terms);
        let lost: FxHashSet<Symbol> = kbs[0]
            .facts()
            .filter(|&f| !kbs[1].contains(f))
            .filter_map(|f| kbs[0].primary(f))
            .filter_map(|p| terms.provs.get(p).rule)
            .collect();
        let mut out: BTreeSet<String> = BTreeSet::new();
        for o in hi.iter().filter(|o| !lo.contains(o)) {
            let triggers: Vec<&SiteText> = sites
                .iter()
                .filter(|s| s.kind == SiteKind::ObligationTrigger)
                .filter(|s| s.rule == o.rule && s.activator == o.activator)
                .collect();
            if !triggers.is_empty() {
                out.extend(triggers.iter().map(|s| s.name.clone()));
                continue;
            }
            if !root.pairs.contains(&(o.rule, o.activator.clone())) {
                let acts: Vec<&SiteText> = sites
                    .iter()
                    .filter(|s| s.kind == SiteKind::Activator && s.rule == o.rule)
                    .collect();
                if !acts.is_empty() {
                    out.extend(acts.iter().map(|s| s.name.clone()));
                    continue;
                }
            }
            let lost_sites: Vec<&SiteText> = sites
                .iter()
                .filter(|s| s.kind == SiteKind::Absent && lost.contains(&s.rule))
                .collect();
            if !lost_sites.is_empty() {
                out.extend(lost_sites.iter().map(|s| s.name.clone()));
                continue;
            }
            // None of the three (M1e2 S1e2.2.17): the obligation's activator,
            // followed to the plans that derived it, as a dead node's
            // refutation is (`lattice/12_activator_chain_hole.ein`, whose
            // `total-owed` activator a plan of `serves` derives).
            let mut seeds: Vec<FactId> = kbs[1]
                .rule_apps_by_rule(o.rule)
                // The activator's every argument, a nested fact among them
                // (M1f S1f.5.18, Q-M1f.43): until then the owed instance kept
                // its symbol arguments only, and so did this match.
                .filter(|&f| *terms.facts.get(f).1 == *o.activator)
                .collect();
            if seeds.is_empty() {
                seeds = kbs[1].rule_apps_by_rule(o.rule).collect();
            }
            out.extend(chain_sites(sites, root, &chain_of(&kbs[1], terms, &seeds)));
        }
        out
    })?;
    named.extend(found);
    Ok(())
}

/// The status a whole cube shows, with the sites its violations are
/// attributed to: each minimal hole, and the smallest witness of each
/// violation — `site_census`'s reading, reduced to the names.
pub(crate) fn status_of(
    cube: &Cube<'_>,
    sites: &[SiteText],
    root: &RootPlans,
    nodes: &mut Nodes<'_>,
    ast: &Ast,
) -> Result<Status, SolveError> {
    let t = cube.topology;
    let (h, d, o) = (
        t.holes.len() as u64,
        t.dead_not_upward_closed.count,
        t.discharged_not_upward_closed.count,
    );
    if h == 0 && d == 0 && o == 0 {
        return Ok(Status::Conservative);
    }
    let mut items: Vec<(u64, Option<u64>)> = t.minimal_holes.iter().map(|&m| (m, None)).collect();
    if let Some((a, b)) = t.dead_not_upward_closed.witness {
        items.push((a, Some(b)));
    }
    if let Some((a, b)) = t.discharged_not_upward_closed.witness {
        items.push((a, Some(b)));
    }
    let mut named: BTreeSet<String> = BTreeSet::new();
    for (mask, up) in items {
        let status = cube.status[mask as usize];
        match (up, status & DEAD == 0) {
            // A discharged node with an owing superset: the debt new at the
            // superset, against the node.
            (Some(b), true) => owing_sites(
                nodes,
                ast,
                sites,
                root,
                &cube.facts_of(b),
                &cube.facts_of(mask),
                &mut named,
            )?,
            _ if status & DEAD != 0 => {
                let mut here: BTreeSet<String> = BTreeSet::new();
                dead_sites(sites, root, &cube.nodes[mask as usize], &mut here);
                if here.is_empty() {
                    here = dead_chain_sites(nodes, sites, root, &cube.facts_of(mask))?;
                }
                named.extend(here);
            }
            _ if status & OWING != 0 => {
                let below = cube
                    .r_below(mask)
                    .map(|b| cube.facts_of(b))
                    .unwrap_or_default();
                owing_sites(
                    nodes,
                    ast,
                    sites,
                    root,
                    &cube.facts_of(mask),
                    &below,
                    &mut named,
                )?
            }
            _ => {}
        }
    }
    Ok(Status::Exercised {
        by: named.into_iter().collect(),
        holes: h,
        dead_not_upward_closed: d,
        discharged_not_upward_closed: o,
    })
}
