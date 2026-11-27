//! **The coupling read from the whole node alone**: one saturation, no pair
//! layer and no cover. What a route can afford to read while it runs.
//!
//! [`super::Coupling`] reads its questions off the cover, whose groups are
//! cliques of the pairs a smaller program excludes, and drops the ways no
//! alive state has by root's pair layer. Both layers cost thousands of
//! saturations on a program whose search is tens. This reading takes
//! neither:
//!
//! - **a question is what root owes**: the alternatives of an instance L5
//!   licenses, *which of these*, read off L5's table, which a route has
//!   before its first node; every other candidate is a question alone;
//! - **the node** is the one [`super::Coupling`] reads, with **every**
//!   candidate of `H` committed, since which are alive alone is a layer's
//!   to say; saturated to quiescence though dead, every application read
//!   and every derived premise followed back to the candidates, the labels
//!   settled as the full reading settles them;
//! - **an edge** is a set of two or more questions a conflict or a
//!   derivation of that node needs a member of each of, the cover of the
//!   ways by classes as the full reading takes it.
//!
//! ### What it does not see
//!
//! - **A conflict of exactly two candidates is no edge.** With no cover a
//!   pair two candidates of one group make, *different letters, different
//!   digits*, cannot be told from a pair the puzzle's statements make: both
//!   are one application over two candidates. They are counted
//!   ([`NodeCoupling::pairs`]) and join the components, and no edge is made
//!   of one, so the edges are of what no pair shows.
//! - **A way no alive state has is read.** Two candidates that exclude each
//!   other are both committed and may derive a fact together; the full
//!   reading drops such a way by the pair layer, this one keeps it, so an
//!   edge here may be coarser than the full reading's and is never finer for
//!   that reason.
//! - **A candidate dead alone is committed** and its derivations are read
//!   like any other's.
//! - **The questions are the debts' and not the groups'**: a group nothing
//!   owes, *who is 5* where only the letters owe a digit, is no question
//!   here, and each of its members is read under the questions it is an
//!   alternative of, or alone.
//!
//! Every candidate is its index in the space's universe, in rendered-text
//! order, and the questions are in the order of their members, so the
//! reading does not depend on the interner's ids or on the order a program
//! states its facts in.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::{FactId, Kb, Terms};
use rustc_hash::FxHashMap;

use super::{
    Clash, Class, Cut, EdgeAcc, Graph, NODE_APPLICATIONS, NodeStop, NodeWork, ROUNDS, node_edges,
};
use crate::error::SolveError;
use crate::explore::Space;
use crate::oblgen::Alternatives;
use crate::region::Candidate;
use crate::structure::{Options, with_space};

/// **The coupling of what root owes, read from one node** (the module's
/// docs).
#[derive(Clone, Debug)]
pub struct NodeCoupling {
    /// The candidates of `H`, the universe in rendered-text order: every
    /// candidate below is its index here.
    pub candidates: Vec<Candidate>,
    /// **The questions**, each its members ascending: first the alternative
    /// sets of the instances L5 licenses, in the order of their members,
    /// each set once; then every candidate in none of them, alone.
    pub questions: Vec<Vec<u32>>,
    /// How many of [`Self::questions`] are a licensed instance's: the first
    /// that many.
    pub owed: usize,
    /// For each candidate, the questions that hold it, ascending.
    pub question_of: Vec<Vec<u32>>,
    /// **The edges**: each the questions, ascending, a conflict or a
    /// derivation of the node needs a member of each of. In the order of
    /// their questions.
    pub edges: Vec<Vec<u32>>,
    /// The edges read one application back, off the same node.
    pub one_step: Vec<Vec<u32>>,
    /// **The pairs**: two candidates one application concludes a
    /// contradiction from, with nothing derived between. No edge is made of
    /// one.
    pub pairs: Vec<(u32, u32)>,
    /// The candidates, each in one component: two are together where they
    /// share a question, an edge holds a question of each, or they are a
    /// pair. Each ascending, in the order of their first members.
    pub components: Vec<Vec<u32>>,
    /// The node derives a contradiction.
    pub dead: bool,
    /// What the node cost, where it was saturated: one saturation, its
    /// facts beyond root's.
    pub node: Option<NodeWork>,
    /// Why it was not read.
    pub node_stopped: Option<NodeStop>,
    /// What the module's bounds cut of the reading.
    pub cut: Cut,
    /// **The node's core**, where it is dead and the run that read it is
    /// narrated to a surface that prints every visited node's core
    /// (`--events`, `--dump-lattice`; `Space::takes_whole_core`): its facts,
    /// the search's status and the rules it rests on, as every dead node a
    /// route visits reports them, so that such a surface prints this one as
    /// it prints the others. `None` elsewhere, a watch that prints no core
    /// among them (`-v`, `--layer-progress`): a core is taken only where a
    /// surface prints one.
    pub core: Option<(Vec<String>, crate::explain::CoreStatus, Vec<String>)>,
}

impl NodeCoupling {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Candidate `b`, as the program writes it.
    pub fn text(&self, b: u32) -> &str {
        &self.candidates[b as usize].text
    }

    /// The node was read whole, nothing cut.
    pub fn complete(&self) -> bool {
        self.node_stopped.is_none() && self.cut.is_none()
    }

    /// The candidates of an edge: the members of its questions, ascending.
    pub fn members(&self, edge: &[u32]) -> Vec<u32> {
        let mut out: BTreeSet<u32> = BTreeSet::new();
        for &q in edge {
            out.extend(&self.questions[q as usize]);
        }
        out.into_iter().collect()
    }
}

/// **[`NodeCoupling`] over a program's prepared space**, `table` L5's as the
/// space reads it. One saturation of the node function's own fork of root,
/// which is no node of the routes' cache and is charged to no budget here:
/// [`NodeCoupling::node`] is its cost, the caller's to count.
///
/// **Within `max_facts`**: the node is saturated only while the candidates
/// committed and the facts its applications have derived are that many or
/// fewer. Past it nothing is read, [`NodeCoupling::node_stopped`] is
/// [`NodeStop::Budget`] and [`NodeCoupling::node`] holds what was derived
/// before the stop, nothing where the candidates alone are more: a caller
/// that can afford only so large a node pays no more than it said.
pub(crate) fn node_coupling_in(
    sp: &mut Space<'_>,
    table: Option<&Alternatives>,
    max_facts: u64,
) -> Result<NodeCoupling, SolveError> {
    let n = sp.n();
    let owed_sets: BTreeSet<Vec<u32>> = table
        .into_iter()
        .flat_map(|t| &t.instances)
        .filter_map(|i| i.alternatives.as_ref().ok())
        .filter(|a| !a.is_empty())
        .map(|a| a.to_vec())
        .collect();
    let mut questions: Vec<Vec<u32>> = owed_sets.into_iter().collect();
    let owed = questions.len();
    let mut question_of: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (q, members) in questions.iter().enumerate() {
        for &b in members {
            question_of[b as usize].push(q as u32);
        }
    }
    for b in 0..n as u32 {
        if question_of[b as usize].is_empty() {
            question_of[b as usize].push(questions.len() as u32);
            questions.push(vec![b]);
        }
    }
    let mut out = NodeCoupling {
        candidates: sp.candidates().to_vec(),
        questions,
        owed,
        question_of,
        edges: Vec::new(),
        one_step: Vec::new(),
        pairs: Vec::new(),
        components: Vec::new(),
        dead: false,
        node: None,
        node_stopped: None,
        cut: Cut::default(),
        core: None,
    };
    if sp.root_is_dead() {
        out.node_stopped = Some(NodeStop::RootDead);
    } else if n == 0 {
        out.node_stopped = Some(NodeStop::Empty);
    }
    if out.node_stopped.is_none() && (n as u64) > max_facts {
        // The candidates alone are more than the caller can pay for.
        out.node_stopped = Some(NodeStop::Budget);
        out.node = Some(NodeWork::default());
    }
    if out.node_stopped.is_none() {
        let committed: Vec<u32> = (0..n as u32).collect();
        let facts: FxHashMap<FactId, u32> = sp
            .candidates()
            .iter()
            .enumerate()
            .map(|(b, c)| (c.fact, b as u32))
            .collect();
        let takes_core = sp.takes_whole_core();
        // The relations L9's premise is read over, where a route's table
        // relies on it: the whole node is a node the route computes.
        let unheld: &[ein_core::Symbol] = table.map_or(&[], |t| &t.unheld_relations);
        let refuted = crate::contradiction::Pairs::of(sp.plans().iter().map(|p| &**p), sp.terms());
        let (root_kb, _, _) = sp.parts();
        let read = sp.supported_within(
            &committed,
            NODE_APPLICATIONS,
            max_facts - n as u64,
            |kb: &Kb, terms: &mut Terms, firings| {
                let breach =
                    crate::unheld::breach(kb, root_kb, terms, unheld, |f| facts.contains_key(&f));
                let core = if takes_core {
                    crate::region::core_of_fixpoint(kb, terms)
                } else {
                    None
                };
                (
                    Graph::of(root_kb, kb, terms, firings, &facts, None, &refuted),
                    core,
                    breach,
                )
            },
        )?;
        if let Ok((_, _, Some(fact))) = &read {
            return Err(crate::unheld::refusal(fact));
        }
        let read = read.map(|(g, core, _)| (g, core));
        match read {
            Err(derived) => {
                out.node_stopped = Some(if derived > max_facts - n as u64 {
                    NodeStop::Budget
                } else {
                    NodeStop::Applications
                });
                out.node = Some(NodeWork {
                    saturations: 1,
                    facts: derived,
                    ..NodeWork::default()
                });
            }
            Ok((mut g, core)) => {
                out.dead = g.work.contradictions > 0;
                if out.dead {
                    out.core = core;
                }
                // The pairs: a contradiction one application concludes from
                // two candidates. Counted, and read for no edge.
                let mut pairs: BTreeSet<(u32, u32)> = BTreeSet::new();
                {
                    let Graph {
                        own,
                        steps,
                        falsity,
                        ..
                    } = &mut g;
                    for &t in falsity.iter() {
                        steps[t as usize].retain(|step| {
                            let [p, q] = step.premises[..] else {
                                return true;
                            };
                            match (own[p as usize], own[q as usize]) {
                                (Some(a), Some(b)) => {
                                    pairs.insert((a.min(b), a.max(b)));
                                    false
                                }
                                _ => true,
                            }
                        });
                    }
                }
                out.pairs = pairs.into_iter().collect();
                let clash = Clash::none(n);
                let read = (&out.question_of[..], &clash);
                let mut cut = Cut::default();
                let (labels, first) = g.labels(ROUNDS, read, &mut cut);
                let listed = |edges: BTreeMap<Class, EdgeAcc>| -> Vec<Vec<u32>> {
                    edges.into_keys().filter(|c| c.len() >= 2).collect()
                };
                let mut shallow: BTreeMap<Class, EdgeAcc> = BTreeMap::new();
                node_edges(&g, &first, read, &mut shallow, &mut cut);
                let mut edges: BTreeMap<Class, EdgeAcc> = BTreeMap::new();
                node_edges(&g, &labels, read, &mut edges, &mut cut);
                out.one_step = listed(shallow);
                out.edges = listed(edges);
                out.node = Some(g.work);
                out.cut = cut;
            }
        }
    }
    out.components = components(&out);
    Ok(out)
}

/// The components of the candidates: together where they share a question,
/// an edge holds a question of each, or they are a pair.
fn components(c: &NodeCoupling) -> Vec<Vec<u32>> {
    fn find(parent: &mut [u32], mut a: u32) -> u32 {
        while parent[a as usize] != a {
            parent[a as usize] = parent[parent[a as usize] as usize];
            a = parent[a as usize];
        }
        a
    }
    let n = c.n() as u32;
    let mut parent: Vec<u32> = (0..n).collect();
    let mut unite = |a: u32, b: u32| {
        let (x, y) = (find(&mut parent, a), find(&mut parent, b));
        if x != y {
            parent[x.max(y) as usize] = x.min(y);
        }
    };
    for members in &c.questions {
        for w in members.windows(2) {
            unite(w[0], w[1]);
        }
    }
    for e in &c.edges {
        for w in e.windows(2) {
            unite(c.questions[w[0] as usize][0], c.questions[w[1] as usize][0]);
        }
    }
    for &(a, b) in &c.pairs {
        unite(a, b);
    }
    let mut by_root: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for b in 0..n {
        by_root.entry(find(&mut parent, b)).or_default().push(b);
    }
    by_root.into_values().collect()
}

/// [`NodeCoupling`] of `root`, a **freshly loaded, unsaturated** program
/// nothing else will use: `node_coupling_in` over a space opened for it,
/// with L5's table as that space reads it.
pub fn node_coupling(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &ein_ir::Ast,
    opts: &Options,
) -> Result<NodeCoupling, SolveError> {
    with_space(root, terms, ast, opts, None, |sp| {
        let table = sp.alternatives()?;
        node_coupling_in(sp, table.as_deref(), opts.budget)
    })
}
