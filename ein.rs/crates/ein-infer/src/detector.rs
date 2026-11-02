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
//! being true. T1e2.1b.5.6 puts them in `ein-infer/tests/detector.rs`; until
//! then the static half's own test there holds an empty [`Static`] on every
//! monotone corpus program.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use ein_core::{Kb, Terms};

use crate::monotone::{self, Monotonicity, SiteKind};
use crate::plan::Plan;
use crate::saturator::DEFAULT_PRIORITY;

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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stake {
    pub concludes: Concludes,
    pub at_stake: AtStake,
}

/// A strongly connected component of the saturation rules' dependency graph
/// over `D` that holds a negative edge: a cycle through an `absent`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cycle {
    /// Its relations, as [`monotone::Rel::label`] writes them, sorted.
    pub relations: Vec<String>,
    /// Its negative edges, `rule: a →¬ b`, sorted.
    pub negative: Vec<String>,
    /// Some cycle in it has an odd number of negative edges: a state with no
    /// stale fact need not exist. Even: there can be several.
    pub odd: bool,
}

/// An order finding: a deferred rule, `reader`, reads under an `absent` a
/// relation in `D` that another deferred rule, `writer`, concludes — directly,
/// or through rules that are not deferred. The boundary admits one parked
/// candidate a round in (priority, FIFO) order, so at one priority the written
/// order decides which goes first, and with the reader's priority earlier the
/// reader goes first in every order. The writer earlier is the stratified
/// order, and is no finding.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Race {
    pub reader: String,
    pub writer: String,
    /// The relations raced over, sorted.
    pub relations: Vec<String>,
    pub reader_priority: i64,
    pub writer_priority: i64,
}

impl Race {
    /// Both at one priority: which is admitted first is the written order.
    pub fn at_one_priority(&self) -> bool {
        self.reader_priority == self.writer_priority
    }
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

/// A labelled edge of the dependency graph: from, to, negative, the rule.
type Edge = (String, String, bool, String);

/// The saturation rules' dependency graph over `D` (`node_census.md` § 5's):
/// an edge from each relation a plan reads to each it writes, negative at odd
/// `absent` depth, and a positive one from an activator relation. Each
/// strongly connected component holding a negative edge, with its parity.
fn cycles_over(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    dynamic: &BTreeSet<String>,
) -> Vec<Cycle> {
    let program = kb.program();
    let sat: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let mut labels: BTreeSet<String> = BTreeSet::new();
    for p in &sat {
        for w in monotone::writes_of(p, terms) {
            labels.insert(w.label(terms));
        }
        for (r, _) in monotone::reads_of(p, terms) {
            labels.insert(r.label(terms));
        }
    }
    // The wildcard negative reads every stored negative the graph names.
    let negatives: Vec<String> = labels
        .iter()
        .filter(|l| l.starts_with("(not ") && *l != "(not ?)")
        .cloned()
        .collect();
    let mut edges: BTreeSet<Edge> = BTreeSet::new();
    for p in &sat {
        let rule = terms.sym(p.rule).to_string();
        let writes: Vec<String> = monotone::writes_of(p, terms)
            .into_iter()
            .map(|w| w.label(terms))
            .collect();
        for (r, depth) in monotone::reads_of(p, terms) {
            let from = r.label(terms);
            let froms = if from == "(not ?)" {
                negatives.clone()
            } else {
                vec![from]
            };
            for f in froms {
                for w in &writes {
                    edges.insert((f.clone(), w.clone(), depth % 2 == 1, rule.clone()));
                }
            }
        }
    }
    for rule in program.rules.values() {
        if rule.params.is_empty() {
            continue;
        }
        let act = terms.sym(rule.name).to_string();
        for p in sat.iter().filter(|p| p.rule == rule.name) {
            for w in monotone::writes_of(p, terms) {
                edges.insert((act.clone(), w.label(terms), false, act.clone()));
            }
        }
    }
    let over_d: Vec<&Edge> = edges
        .iter()
        .filter(|(a, b, _, _)| dynamic.contains(a) && dynamic.contains(b))
        .collect();
    let mut out = components(&over_d);
    out.sort_by(|a, b| a.relations.cmp(&b.relations));
    out
}

/// The strongly connected components of `edges` with a negative edge inside.
fn components(edges: &[&Edge]) -> Vec<Cycle> {
    let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
    for (a, b, _, _) in edges {
        let k = ids.len();
        ids.entry(a.as_str()).or_insert(k);
        let k = ids.len();
        ids.entry(b.as_str()).or_insert(k);
    }
    let mut names = vec![""; ids.len()];
    for (s, &i) in &ids {
        names[i] = s;
    }
    let n = ids.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b, _, _) in edges {
        adj[ids[a.as_str()]].push(ids[b.as_str()]);
    }
    let (comp, ncomp) = scc(&adj);
    let mut out = Vec::new();
    for c in 0..ncomp {
        let inside: Vec<&Edge> = edges
            .iter()
            .copied()
            .filter(|(a, b, _, _)| comp[ids[a.as_str()]] == c && comp[ids[b.as_str()]] == c)
            .collect();
        if !inside.iter().any(|e| e.2) {
            continue;
        }
        let members: Vec<usize> = (0..n).filter(|&v| comp[v] == c).collect();
        // A 2-colouring by negative-edge parity: a conflict is an odd cycle.
        let mut parity: BTreeMap<usize, bool> = BTreeMap::new();
        parity.insert(members[0], false);
        let mut odd = false;
        let mut changed = true;
        while changed {
            changed = false;
            for (a, b, neg, _) in inside.iter().copied() {
                let (a, b) = (ids[a.as_str()], ids[b.as_str()]);
                if let Some(&pa) = parity.get(&a) {
                    let want = pa ^ neg;
                    match parity.get(&b) {
                        None => {
                            parity.insert(b, want);
                            changed = true;
                        }
                        Some(&pb) if pb != want => odd = true,
                        _ => {}
                    }
                }
            }
        }
        let mut relations: Vec<String> = members.iter().map(|&v| names[v].to_string()).collect();
        relations.sort();
        let negative: BTreeSet<String> = inside
            .iter()
            .filter(|e| e.2)
            .map(|(a, b, _, r)| format!("{r}: {a} →¬ {b}"))
            .collect();
        out.push(Cycle {
            relations,
            negative: negative.into_iter().collect(),
            odd,
        });
    }
    out
}

/// Tarjan's strongly connected components: each vertex's component, and how
/// many there are.
fn scc(adj: &[Vec<usize>]) -> (Vec<usize>, usize) {
    struct T<'a> {
        adj: &'a [Vec<usize>],
        index: Vec<usize>,
        low: Vec<usize>,
        on: Vec<bool>,
        stack: Vec<usize>,
        comp: Vec<usize>,
        next: usize,
        ncomp: usize,
    }
    impl T<'_> {
        fn visit(&mut self, v: usize) {
            self.index[v] = self.next;
            self.low[v] = self.next;
            self.next += 1;
            self.stack.push(v);
            self.on[v] = true;
            for k in 0..self.adj[v].len() {
                let w = self.adj[v][k];
                if self.index[w] == usize::MAX {
                    self.visit(w);
                    self.low[v] = self.low[v].min(self.low[w]);
                } else if self.on[w] {
                    self.low[v] = self.low[v].min(self.index[w]);
                }
            }
            if self.low[v] == self.index[v] {
                loop {
                    let w = self.stack.pop().expect("on the stack");
                    self.on[w] = false;
                    self.comp[w] = self.ncomp;
                    if w == v {
                        break;
                    }
                }
                self.ncomp += 1;
            }
        }
    }
    let n = adj.len();
    let mut t = T {
        adj,
        index: vec![usize::MAX; n],
        low: vec![0; n],
        on: vec![false; n],
        stack: Vec::new(),
        comp: vec![usize::MAX; n],
        next: 0,
        ncomp: 0,
    };
    for v in 0..n {
        if t.index[v] == usize::MAX {
            t.visit(v);
        }
    }
    (t.comp, t.ncomp)
}

/// The order check. A saturation rule is **deferred** when a disjunct carries
/// an `absent` guard: the boundary admits its candidates one a round, in
/// (priority, FIFO) order. A deferred rule A that reads under an `absent` a
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
    let priority = |p: &Plan| -> i64 {
        program
            .rules
            .get(p.rule)
            .and_then(|r| r.priority)
            .and_then(|i| terms.ints.value(i))
            .unwrap_or(DEFAULT_PRIORITY)
    };
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
            let (pa, pb) = (priority(a), priority(b));
            if pa > pb {
                continue;
            }
            let key = (terms.sym(a.rule).to_string(), terms.sym(b.rule).to_string());
            let race = out.entry(key.clone()).or_insert_with(|| Race {
                reader: key.0.clone(),
                writer: key.1.clone(),
                relations: Vec::new(),
                reader_priority: pa,
                writer_priority: pb,
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
