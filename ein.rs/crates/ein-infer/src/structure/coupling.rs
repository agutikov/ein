//! Coupling: what ties the questions of a cover together. The account is
//! in [`crate::structure`]'s header, § Coupling: what ties the questions.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use ein_core::{FactId, Kb, Terms};
use rustc_hash::{FxHashMap, FxHashSet};

use super::{
    Bits, Cover, ExactlyOne, Loaded, Mark, Options, Reading, Theory, TheoryOptions, small_kb,
    theory_in, with_space,
};
use crate::compile::SharedMemo;
use crate::error::SolveError;
use crate::events::Events;
use crate::explore::Space;
use crate::mechanisms::Mechanisms;
use crate::region::{Candidate, Supported};
use crate::saturator::Session;

mod node;
pub(crate) use node::node_coupling_in;
pub use node::{NodeCoupling, node_coupling};

/// **The bound on the whole node's saturation**, in rule applications,
/// redundant ones among them. A constant of the module and no lever. A node
/// that takes more is not read ([`NodeStop::Applications`]).
pub const NODE_APPLICATIONS: usize = 1 << 20;

/// **The bound on the rounds a label is settled in.** A fact's label is
/// read again while a premise's moved; past this many rounds the labels are
/// taken as they stand and [`Cut::unsettled`] says so.
pub const ROUNDS: u32 = 64;

/// **The bound on the ways one application is read**: the choices of one
/// way for each of its premises, as distinct sets of questions. Past it
/// they are read as one way that needs every question any of them names,
/// and counted ([`Cut::ways`]).
pub const WAYS: usize = 1 << 8;

/// **The bound on the descriptions of one way**: the choices of one
/// question for each candidate it reads. Past it the first in the order of
/// the sequences are kept, and the way counted ([`Cut::descriptions`]).
pub const DESCRIPTIONS: usize = 1 << 10;

/// What [`Coupling::domain`] says of a program the static check flags.
pub const COUPLING_FLAGGED: &str = "the static check flags this program: a tie is true of the \
                                    nodes it was read from, and nothing above them follows from \
                                    it";

/// One node of the hypergraph: a **question**.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Question {
    /// A group of the cover, by its place in [`Cover::groups`]: *which of
    /// these, if any*.
    Group(u32),
    /// A candidate in no group: *this one, or not*.
    Alone(u32),
}

/// How the coupling is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CouplingReading {
    /// **The pairs alone**: root's pair layer, and no node beyond it.
    Pairs,
    /// The pairs, and the whole node read as far as each application's own
    /// premises: an application that reads a derived fact is not read.
    Direct,
    /// The pairs, and the whole node with every derived premise followed
    /// back **one** application.
    OneStep,
    /// **The reading the owner takes**: the pairs, and the whole node with
    /// every derived premise followed back to the candidates.
    Node,
}

impl CouplingReading {
    pub fn as_str(self) -> &'static str {
        match self {
            CouplingReading::Pairs => "pairs",
            CouplingReading::Direct => "direct",
            CouplingReading::OneStep => "one-step",
            CouplingReading::Node => "node",
        }
    }

    /// The rounds its labels are settled in; `None` where no node is read.
    fn rounds(self) -> Option<u32> {
        match self {
            CouplingReading::Pairs => None,
            CouplingReading::Direct => Some(0),
            CouplingReading::OneStep => Some(1),
            CouplingReading::Node => Some(ROUNDS),
        }
    }
}

/// How [`coupling`] is asked.
#[derive(Clone, Copy, Debug)]
pub struct CouplingOptions {
    /// The budget, the jobs and the deadline of root's layer and of the
    /// small KB's, each read at it; the whole node is not read where its
    /// own facts beyond root pass the budget.
    pub layer: Options,
    pub reading: CouplingReading,
    /// Read the support of every pair root excludes
    /// ([`super::Support`]), so that a tie of the pair layer names the
    /// statements its death rests on.
    pub supports: bool,
}

impl Default for CouplingOptions {
    fn default() -> Self {
        CouplingOptions {
            layer: Options::default(),
            reading: CouplingReading::Node,
            supports: false,
        }
    }
}

/// What a pair of the pair layer says of its two candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TieKind {
    /// Root excludes the pair and the theory alone does not.
    Excludes,
    /// One member is in the closure of the other's node.
    Implies,
    /// The pair's node is alive and holds a fact neither member's does.
    Interacts,
}

impl TieKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TieKind::Excludes => "excludes",
            TieKind::Implies => "implies",
            TieKind::Interacts => "interacts",
        }
    }
}

/// **A tie of the pair layer**: two candidates, each alive alone, in no
/// question together, of which root's node of the pair says something
/// neither single's node does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tie {
    /// The two candidates, the lower first.
    pub pair: (u32, u32),
    pub kind: TieKind,
    /// For an exclusion, where supports were asked and this one was read:
    /// the puzzle's statements its death rests on, as places in
    /// [`Coupling::reasons`], ascending.
    pub stated: Option<Vec<u32>>,
}

/// What of the pair layer an edge covers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PairTies {
    pub excludes: u32,
    pub implies: u32,
    pub interacts: u32,
}

impl PairTies {
    pub fn all(&self) -> u32 {
        self.excludes + self.implies + self.interacts
    }
}

/// What of the whole node an edge covers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeTies {
    /// Ways to a contradiction: an application that concludes `(false)`,
    /// a fact beside its negation where a rule refutes the two, or one that
    /// ends the state.
    pub conflicts: u32,
    /// Derived facts that are no candidate, one of whose derivations needs
    /// a member of each of the edge's questions.
    pub derivations: u32,
    /// Ways to a candidate of the node from members of other questions.
    pub implications: u32,
}

impl NodeTies {
    pub fn all(&self) -> u32 {
        self.conflicts + self.derivations + self.implications
    }
}

/// What an edge is the edge of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    /// A conflict or a derivation needs a member of each of its questions.
    Tie,
    /// A debt needs a member of one of its questions.
    Debt,
    /// Both: a debt's questions, which a conflict or a derivation ties too.
    Both,
}

impl EdgeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EdgeKind::Tie => "a conflict or a derivation",
            EdgeKind::Debt => "a debt",
            EdgeKind::Both => "a conflict or a derivation, and a debt",
        }
    }
}

/// **A coupling**: two or more questions that bear on each other, a
/// hyperedge of the questions' hypergraph. Either some conflict or some
/// derivation needs a member of each of them ([`Self::pairs`],
/// [`Self::node`]), or some instance root owes needs a member of one of
/// them ([`Self::debts`]), or both.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    /// The questions, as places in [`Coupling::questions`], ascending.
    pub questions: Vec<u32>,
    /// The ties of the pair layer it was chosen for.
    pub pairs: PairTies,
    /// What of the whole node it was chosen for.
    pub node: NodeTies,
    /// The debts whose payers' questions these are, as places in
    /// [`Coupling::debts`], ascending: the edge's reason where it is a
    /// debt's.
    pub debts: Vec<u32>,
    /// **The puzzle's statements it rests on**: each as a place in
    /// [`Coupling::reasons`], with how many of the things the edge covers
    /// name it, the most named first. Of a tie, the statements of its
    /// support; of the whole node, the statements the last application of
    /// each way reads, directly or through root's own derivation. A debt
    /// names none: its reason is the instance.
    pub stated: Vec<(u32, u32)>,
}

impl Edge {
    pub fn arity(&self) -> usize {
        self.questions.len()
    }

    /// A conflict or a derivation ties its questions.
    pub fn is_tie(&self) -> bool {
        self.pairs.all() + self.node.all() > 0
    }

    pub fn kind(&self) -> EdgeKind {
        match (self.is_tie(), self.debts.is_empty()) {
            (true, true) => EdgeKind::Tie,
            (true, false) => EdgeKind::Both,
            (false, _) => EdgeKind::Debt,
        }
    }
}

/// **A debt that is an edge**: an instance root owes whose payers lie in
/// two questions or more, **whether or not L5 licenses it**. A state of `R`
/// holds one of its payers, so `R` is no product across them, whether or
/// not a conflict or a derivation ties them. L5's licence says whether a
/// search may confine itself to the instance's alternatives; it does not
/// say whether root owes the instance, and `R` is discharged either way. An
/// instance whose payers are all in one question is no edge: it is that
/// question's own, which is what makes an owed group *exactly one*.
///
/// **An unlicensed debt's payers are only those that were read**: the
/// candidates a single one of whose nodes shows a witness. A witness that
/// only two candidates together bring is not seen, nor one that is no
/// candidate and that nothing brings alone, so such an edge may be too
/// small. Nothing follows for a search from it beyond *do not take these
/// questions for independent*.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Debt {
    /// Its obligation rule and the instance, as [`super::OwedInstance`]
    /// has them.
    pub rule: String,
    pub instance: String,
    /// L5 licenses the instance: [`Self::payers`] are then its
    /// alternatives, all of what can witness it.
    pub licensed: bool,
    /// Its payers, alive alone at root, ascending. Where L5 licenses the
    /// instance, the candidates L5 reads as able to witness it; where it
    /// does not, **the payers read**: the candidates whose own node holds a
    /// fact that can witness it.
    pub payers: Vec<u32>,
    /// The questions of its payers, as places in [`Coupling::questions`],
    /// ascending: those that hold the most payers first, until every payer
    /// is in one, questions that hold as many chosen together. They are its
    /// edge's.
    pub questions: Vec<u32>,
}

/// Why the whole node was not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeStop {
    /// Root's own fixpoint holds a contradiction: there is no node to commit to.
    RootDead,
    /// Root's singles were not all read, so which candidates are alive
    /// alone is not known.
    SinglesUnread,
    /// No candidate is alive alone.
    Empty,
    /// The node took more than [`NODE_APPLICATIONS`] applications.
    Applications,
    /// The node's facts beyond root pass the layer's budget.
    Budget,
    /// The deadline passed before it.
    Deadline,
}

impl NodeStop {
    pub fn as_str(self) -> &'static str {
        match self {
            NodeStop::RootDead => "root is dead",
            NodeStop::SinglesUnread => "root's singles were not all read",
            NodeStop::Empty => "no candidate is alive alone",
            NodeStop::Applications => "the node took more applications than the bound",
            NodeStop::Budget => "the node's facts pass the budget",
            NodeStop::Deadline => "the deadline passed",
        }
    }
}

/// What reading the whole node cost.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeWork {
    /// Fixpoints computed: the one node.
    pub saturations: u64,
    /// The node's facts beyond root's.
    pub facts: u64,
    /// Rule applications the saturation made, redundant ones among them.
    pub applications: u64,
    /// The distinct derivation steps read off them and off the store: a
    /// conclusion beyond root with the premises one application gave it.
    pub steps: u64,
    /// Contradictions the node holds: `(false)` facts, and facts beside
    /// their negation.
    pub contradictions: u64,
    /// Rounds the labels were settled in.
    pub rounds: u32,
    /// Derived facts with ways of more than one kind, whose label is the
    /// union of the classes that describe them.
    pub merged: u64,
}

/// What the module's bounds cut of the whole node's reading.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cut {
    /// Applications of which more than [`WAYS`] ways were to be read.
    pub ways: u64,
    /// Ways of which more than [`DESCRIPTIONS`] descriptions were to be
    /// read.
    pub descriptions: u64,
    /// The labels were still moving after [`ROUNDS`] rounds.
    pub unsettled: bool,
}

impl Cut {
    pub fn is_none(&self) -> bool {
        *self == Cut::default()
    }
}

/// **The coupling of a program's questions**: the hypergraph whose nodes
/// are the cover's groups and the candidates in no group, and whose edges
/// are the sets of questions a conflict or a derivation needs a member of
/// each of.
///
/// Every candidate is its index in [`Self::candidates`], the universe in
/// rendered-text order; a question is its place in [`Self::questions`], the
/// groups in the cover's order and then the candidates in no group,
/// ascending. Each list is ascending or says its order, so a coupling no
/// bound cut does not depend on the interner's ids or on the job count.
#[derive(Clone, Debug)]
pub struct Coupling {
    pub candidates: Vec<Candidate>,
    pub reading: CouplingReading,
    /// The static verdict on the program.
    pub monotone: bool,
    /// **The nodes.**
    pub questions: Vec<Question>,
    /// For each candidate, the questions that hold it, ascending: its
    /// groups, or itself alone.
    pub question_of: Vec<Vec<u32>>,
    /// Root's pairs were all read: [`Self::ties`] are all of them.
    pub pairs_read: bool,
    /// **The ties of the pair layer**, by pair and then by kind.
    pub ties: Vec<Tie>,
    /// The puzzle's statements some tie or some edge names, as text, in
    /// the order the loader stored them.
    pub reasons: Vec<String>,
    /// **The edges**, in the order of their questions.
    pub edges: Vec<Edge>,
    /// **The edges read one application back**, under
    /// [`CouplingReading::Node`]: what [`CouplingReading::OneStep`] gives
    /// for [`Self::edges`], off the same node and the same pair layer. The
    /// whole node's labels are unions, so on a program whose derivations go
    /// round its edges come to one of many questions, and these are the
    /// finer ones that one absorbs. `None` under any other reading.
    pub one_step: Option<Vec<Edge>>,
    /// **The components**, over every edge: the candidates alive alone at
    /// root, each in one, two of them together where they share a question
    /// or an edge holds a question of each. Each ascending, in the order of
    /// their first members. They are what `R` is the product over.
    pub components: Vec<Vec<u32>>,
    /// **The components over the conflicts' and the derivations' edges
    /// alone**, the debts' left out: what no debt joins. Where a debt's
    /// payers lie in two of them ([`Self::debts_across`]), `R` is no
    /// product over these.
    pub conflict_components: Vec<Vec<u32>>,
    /// **The debts that are edges**, in the order of L5's table, each
    /// saying whether L5 licenses it. Empty where no table was read; and an
    /// unlicensed instance has no payers read, and so is none, where its
    /// rule names no witness, or where root's singles were not read, which
    /// [`Self::payers_read`] says.
    pub debts: Vec<Debt>,
    /// **The payers of every instance L5 does not license were read**: each
    /// such instance's witnesses' holders, off root's singles
    /// ([`super::Owed::singles_read`]). `false` where the table holds an
    /// unlicensed instance with witnesses and root's layer stopped before
    /// its last single: no such instance is then among [`Self::debts`] or
    /// an edge, [`Self::components`] may be finer than `R`'s parts, and
    /// [`Self::debts_across`] lists the licensed debts alone.
    pub payers_read: bool,
    /// The instances of L5's table it does not license, edges or not.
    pub unlicensed: u32,
    /// The candidates committed in the whole node: those alive alone at
    /// root. Empty where no node was read.
    pub committed: Vec<u32>,
    /// What the whole node cost, where it was saturated.
    pub node: Option<NodeWork>,
    /// Why the whole node was not read, where the reading asks one and it
    /// was not.
    pub node_stopped: Option<NodeStop>,
    /// What the bounds cut of it.
    pub cut: Cut,
}

impl Coupling {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Candidate `b`, as the program writes it.
    pub fn text(&self, b: u32) -> &str {
        &self.candidates[b as usize].text
    }

    /// The reading asked was read whole: root's pairs, the payers of every
    /// instance L5 does not license, and the node where the reading asks
    /// one, with nothing cut.
    pub fn complete(&self) -> bool {
        self.pairs_read && self.payers_read && self.node_stopped.is_none() && self.cut.is_none()
    }

    /// What the coupling's statements are about, where that is not the
    /// whole lattice: [`COUPLING_FLAGGED`] on a program the static check
    /// flags.
    pub fn domain(&self) -> Option<&'static str> {
        (!self.monotone).then_some(COUPLING_FLAGGED)
    }

    /// The questions that are a candidate in no group, as places in
    /// [`Self::questions`].
    pub fn alone(&self) -> impl Iterator<Item = u32> + '_ {
        (0..self.questions.len() as u32)
            .filter(|&q| matches!(self.questions[q as usize], Question::Alone(_)))
    }

    /// **The edge a set of candidates lies inside**: the first edge of a
    /// conflict or a derivation that holds a question of every member.
    /// `None` where none does. A debt's edge alone is not read: it says
    /// that one of its payers must hold, and nothing of a conflict.
    pub fn edge_over(&self, set: &[u32]) -> Option<u32> {
        (0..self.edges.len() as u32).find(|&e| {
            if !self.edges[e as usize].is_tie() {
                return false;
            }
            let questions = &self.edges[e as usize].questions;
            set.iter().all(|&b| {
                self.question_of[b as usize]
                    .iter()
                    .any(|q| questions.binary_search(q).is_ok())
            })
        })
    }

    /// The members of `set` share one question.
    pub fn in_one_question(&self, set: &[u32]) -> bool {
        let Some((&first, rest)) = set.split_first() else {
            return false;
        };
        self.question_of[first as usize].iter().any(|q| {
            rest.iter()
                .all(|&b| self.question_of[b as usize].binary_search(q).is_ok())
        })
    }

    /// The component candidate `b` is in, as a place in
    /// [`Self::components`]; `None` for a candidate not alive alone.
    pub fn component_of(&self, b: u32) -> Option<u32> {
        (0..self.components.len() as u32)
            .find(|&k| self.components[k as usize].binary_search(&b).is_ok())
    }

    /// **The debts no conflict joins**: those whose payers lie in two or
    /// more of [`Self::conflict_components`], as places in [`Self::debts`].
    pub fn debts_across(&self) -> Vec<u32> {
        (0..self.debts.len() as u32)
            .filter(|&d| !within(&self.conflict_components, &self.debts[d as usize].payers))
            .collect()
    }
}

/// `payers` lie in one of `components`.
fn within(components: &[Vec<u32>], payers: &[u32]) -> bool {
    components
        .iter()
        .any(|k| payers.iter().all(|b| k.binary_search(b).is_ok()))
}

// ── The cover of a set of ways by classes ─────────────────────────────

/// A set of questions, ascending.
type Class = Vec<u32>;

/// One way to a conclusion, as the questions it needs: `derived`, those its
/// derived premises' labels name, and `own`, the candidates it reads as
/// they are, each standing for one of its questions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Way {
    derived: Class,
    own: Vec<u32>,
}

/// The classes that describe `way`: its derived questions with one question
/// of each candidate it reads. At most [`DESCRIPTIONS`], the rest counted.
fn descriptions(way: &Way, question_of: &[Vec<u32>], cut: &mut Cut) -> Vec<Class> {
    let mut out: BTreeSet<Class> = BTreeSet::from([way.derived.clone()]);
    for &c in &way.own {
        let mut next: BTreeSet<Class> = BTreeSet::new();
        for class in &out {
            for &q in &question_of[c as usize] {
                let mut with = class.clone();
                if let Err(at) = with.binary_search(&q) {
                    with.insert(at, q);
                }
                next.insert(with);
            }
        }
        if next.len() > DESCRIPTIONS {
            cut.descriptions += 1;
            next = next.into_iter().take(DESCRIPTIONS).collect();
        }
        out = next;
    }
    out.into_iter().collect()
}

/// How many ways of the whole node each class describes: what decides
/// between two classes that describe as many of one fact's.
type Weights = FxHashMap<Class, u32>;

/// **The cover**: classes chosen so that every item is described by one of
/// them, each with the items it was chosen for, in the order chosen. A
/// class of one question goes before any other, since a way one question
/// describes is that question's own; then the class that describes the
/// most items not yet covered; among equals the one of the greater
/// `weight`, then the one of fewer questions. **Classes equal in all of
/// that are chosen together**, so the choice does not follow the order the
/// questions are numbered in, which is the universe's and moves with how a
/// name is spelled.
fn cover_of(items: &[Vec<Class>], weight: &Weights) -> Vec<(Class, Vec<u32>)> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Key {
        single: bool,
        count: u32,
        weight: u32,
        shorter: std::cmp::Reverse<usize>,
        lower: std::cmp::Reverse<Class>,
    }
    let mut of_class: BTreeMap<&Class, Vec<u32>> = BTreeMap::new();
    for (k, classes) in items.iter().enumerate() {
        for class in classes {
            of_class.entry(class).or_default().push(k as u32);
        }
    }
    let key = |class: &Class, count: u32| Key {
        single: class.len() <= 1,
        count,
        weight: weight.get(class).copied().unwrap_or(0),
        shorter: std::cmp::Reverse(class.len()),
        lower: std::cmp::Reverse(class.clone()),
    };
    let mut heap: BinaryHeap<Key> = of_class
        .iter()
        .map(|(class, held)| key(class, held.len() as u32))
        .collect();
    let mut covered = vec![false; items.len()];
    let mut left = items.iter().filter(|c| !c.is_empty()).count();
    let mut out: Vec<(Class, Vec<u32>)> = Vec::new();
    while left > 0 {
        let Some(top) = heap.pop() else {
            break;
        };
        let class = top.lower.0;
        let live: Vec<u32> = of_class[&class]
            .iter()
            .copied()
            .filter(|&k| !covered[k as usize])
            .collect();
        if live.is_empty() {
            continue;
        }
        if live.len() as u32 != top.count {
            // Stale: some of its items were covered since.
            heap.push(key(&class, live.len() as u32));
            continue;
        }
        // Every class that stands where this one does, read against the
        // same items.
        let mut chosen = vec![(class, live)];
        while heap.peek().is_some_and(|next| {
            (next.single, next.count, next.weight, next.shorter.0)
                == (top.single, top.count, top.weight, top.shorter.0)
        }) {
            let class = heap.pop().expect("peeked").lower.0;
            let live: Vec<u32> = of_class[&class]
                .iter()
                .copied()
                .filter(|&k| !covered[k as usize])
                .collect();
            if live.len() as u32 == top.count {
                chosen.push((class, live));
            } else if !live.is_empty() {
                heap.push(key(&class, live.len() as u32));
            }
        }
        chosen.sort();
        for (_, live) in &chosen {
            for &k in live {
                if !covered[k as usize] {
                    covered[k as usize] = true;
                    left -= 1;
                }
            }
        }
        out.extend(chosen);
    }
    out
}

// ── The whole node ────────────────────────────────────────────────────

/// One derivation step of the whole node: a conclusion beyond root, the
/// premises beyond root one application gave it, and the puzzle's
/// statements that application reads.
struct Step {
    target: u32,
    premises: Vec<u32>,
    reasons: Vec<u32>,
}

/// One class of a fact's label, with the puzzle's statements the last
/// applications of the ways it covers read.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    class: Class,
    reasons: Vec<u32>,
}

/// The whole node, as a derivation graph over its facts beyond root.
struct Graph {
    /// For each fact beyond root, the candidate it is, where it is one of
    /// those committed.
    own: Vec<Option<u32>>,
    /// The steps, by target.
    steps: Vec<Vec<Step>>,
    /// The facts that are `(false)`.
    falsity: Vec<u32>,
    /// The firings that ended the state, each as a step with no target:
    /// what it read beyond root, and the statements root's side rests on.
    ends: Vec<Step>,
    /// A fact beside its negation.
    pairs: Vec<(u32, u32)>,
    /// A fact root holds beside its negation beyond root, or the reverse:
    /// the side beyond root.
    half_pairs: Vec<u32>,
    work: NodeWork,
}

/// Which two candidates no alive state holds together, as root's pair layer
/// and the cover say it: a pair in one question, and a pair root excludes.
struct Clash(Vec<Bits>);

impl Clash {
    fn of(cover: &Cover, root: &super::PairLayer) -> Clash {
        let n = cover.n();
        let mut rows = vec![Bits::new(n); n];
        let dead = root
            .against_l3
            .iter()
            .filter(|x| x.2)
            .map(|&(a, b, _)| (a, b));
        for (a, b) in cover
            .excluded
            .iter()
            .chain(&root.excluded)
            .copied()
            .chain(dead)
        {
            rows[a as usize].set(b);
            rows[b as usize].set(a);
        }
        Clash(rows)
    }

    /// No pair is known to clash: what a reading without the pair layer
    /// and the cover has.
    fn none(n: usize) -> Clash {
        Clash(vec![Bits::new(n); n])
    }

    /// Some two of `own` clash.
    fn within(&self, own: &[u32]) -> bool {
        own.iter()
            .enumerate()
            .any(|(k, &a)| own[k + 1..].iter().any(|&b| self.0[a as usize].has(b)))
    }
}

/// What a way is a way to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum To {
    /// A derived fact, which a later step may read. A way that reads two
    /// candidates no alive state holds together is no state's, and is not
    /// read: what only it derives is derived in no alive node.
    Fact,
    /// A contradiction. The one way kept that reads such a pair is the
    /// pair's own, two candidates and nothing derived: that is the pair's
    /// death, and anything larger that holds the pair is not minimal.
    Contradiction,
}

fn union(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i] < b[j]) {
            out.push(a[i]);
            i += 1;
        } else if i == a.len() || b[j] < a[i] {
            out.push(b[j]);
            j += 1;
        } else {
            out.push(a[i]);
            i += 1;
            j += 1;
        }
    }
    out
}

/// What reading the node needs of the theory's share.
struct Puzzle<'t> {
    /// A stated fact's place in [`Theory::stated`].
    place: FxHashMap<FactId, u32>,
    theory: &'t Theory,
}

impl Puzzle<'_> {
    /// **The puzzle's statements a root fact rests on**: itself where the
    /// program states it and the small KB drops it, and otherwise those the
    /// derivation root recorded first for it reaches, through its premises,
    /// its activator and what its universals read.
    fn reasons(
        &self,
        root: &Kb,
        terms: &Terms,
        fact: FactId,
        memo: &mut FxHashMap<FactId, Vec<u32>>,
    ) -> Vec<u32> {
        if let Some(known) = memo.get(&fact) {
            return known.clone();
        }
        let mut out: BTreeSet<u32> = BTreeSet::new();
        let mut seen: FxHashSet<FactId> = FxHashSet::default();
        let mut stack = vec![fact];
        while let Some(f) = stack.pop() {
            if !seen.insert(f) {
                continue;
            }
            if let Some(&k) = self.place.get(&f) {
                if !self.theory.stated[k as usize].kept {
                    out.insert(k);
                }
                continue;
            }
            let Some(p) = root.primary(f) else {
                continue;
            };
            let prov = terms.provs.get(p);
            stack.extend(prov.premises.iter().copied());
            stack.extend(prov.universal.iter().copied());
            stack.extend(prov.activator);
        }
        let out: Vec<u32> = out.into_iter().collect();
        memo.insert(fact, out.clone());
        out
    }
}

impl Graph {
    /// The whole node `kb`, a fork of `root`, with the applications that
    /// reached it: every application's premises, what its universals read
    /// and the activator its plan was compiled from, and beside them every
    /// justification the store kept.
    fn of(
        root: &Kb,
        kb: &Kb,
        terms: &Terms,
        firings: &[Supported],
        committed: &FxHashMap<FactId, u32>,
        puzzle: Option<&Puzzle<'_>>,
        refuted: &crate::contradiction::Pairs,
    ) -> Graph {
        let mut index: FxHashMap<FactId, u32> = FxHashMap::default();
        let mut own: Vec<Option<u32>> = Vec::new();
        let mut facts = 0u64;
        for f in kb.facts() {
            if !root.contains(f) {
                index.insert(f, own.len() as u32);
                own.push(committed.get(&f).copied());
                facts += 1;
            }
        }
        let mut memo: FxHashMap<FactId, Vec<u32>> = FxHashMap::default();
        let mut seen: FxHashSet<(u32, Vec<u32>)> = FxHashSet::default();
        let mut steps: Vec<Vec<Step>> = (0..own.len()).map(|_| Vec::new()).collect();
        let mut count = 0u64;
        let mut add = |targets: &[FactId], read: &mut dyn Iterator<Item = FactId>| {
            let mut premises: Vec<u32> = Vec::new();
            let mut reasons: Vec<u32> = Vec::new();
            for p in read {
                match index.get(&p) {
                    Some(&k) => premises.push(k),
                    None => {
                        if let Some(puzzle) = puzzle {
                            reasons = union(&reasons, &puzzle.reasons(root, terms, p, &mut memo));
                        }
                    }
                }
            }
            premises.sort_unstable();
            premises.dedup();
            for d in targets {
                let Some(&target) = index.get(d) else {
                    continue;
                };
                if premises.contains(&target) || !seen.insert((target, premises.clone())) {
                    continue;
                }
                count += 1;
                steps[target as usize].push(Step {
                    target,
                    premises: premises.clone(),
                    reasons: reasons.clone(),
                });
            }
        };
        for Supported {
            firing: f,
            universal,
        } in firings
        {
            // The activator the plan was compiled from: the fact
            // `(rule args…)`, a premise wherever the node holds it.
            let activator = (!f.activator.is_empty())
                .then(|| terms.probe_fact(f.rule, &f.activator))
                .flatten()
                .filter(|&a| kb.contains(a));
            let mut read = f
                .premises
                .iter()
                .chain(universal.iter())
                .copied()
                .chain(activator);
            add(&f.derived, &mut read);
        }
        // determinism-ok: the steps are a set, sorted below.
        for &f in index.keys() {
            for p in kb.justifications(f) {
                let prov = terms.provs.get(p);
                if prov.rule.is_none() && prov.premises.is_empty() {
                    continue;
                }
                let mut read = prov
                    .premises
                    .iter()
                    .chain(prov.universal.iter())
                    .copied()
                    .chain(prov.activator.filter(|&a| kb.contains(a)));
                add(&[f], &mut read);
            }
        }
        for of_one in &mut steps {
            of_one.sort_by(|a, b| a.premises.cmp(&b.premises));
        }
        let (mut falsity, mut pairs, mut half_pairs) = (Vec::new(), Vec::new(), Vec::new());
        let mut ends: Vec<Step> = Vec::new();
        // Every pair the rules refute, and not the ones the store's records
        // name: the reading does not depend on what the store keeps.
        let contradictions = crate::contradiction::detect_all(kb, terms, refuted);
        for c in &contradictions {
            use crate::contradiction::Contradiction;
            match *c {
                // A rule of the module every program gets is read through:
                // what its firing matched is what the contradiction is.
                Contradiction::Ended(p)
                    if crate::contradiction::ended_by_an_implicit_rule(kb, terms, p) =>
                {
                    let prov = terms.provs.get(p);
                    falsity.extend(prov.premises.iter().filter_map(|f| index.get(f).copied()));
                }
                Contradiction::Ended(p) => {
                    let prov = terms.provs.get(p);
                    let (mut premises, mut reasons) = (Vec::new(), Vec::new());
                    let read = prov
                        .premises
                        .iter()
                        .chain(prov.universal.iter())
                        .copied()
                        .chain(prov.activator.filter(|&a| kb.contains(a)));
                    for f in read {
                        match index.get(&f) {
                            Some(&k) => premises.push(k),
                            None => {
                                if let Some(puzzle) = puzzle {
                                    reasons =
                                        union(&reasons, &puzzle.reasons(root, terms, f, &mut memo));
                                }
                            }
                        }
                    }
                    premises.sort_unstable();
                    premises.dedup();
                    // A firing that read nothing beyond root is root's own.
                    if !premises.is_empty() {
                        ends.push(Step {
                            target: u32::MAX,
                            premises,
                            reasons,
                        });
                    }
                }
                Contradiction::Pair { positive, negative } => {
                    match (index.get(&positive), index.get(&negative)) {
                        (Some(&a), Some(&b)) => pairs.push((a, b)),
                        (Some(&a), None) | (None, Some(&a)) => half_pairs.push(a),
                        (None, None) => {}
                    }
                }
            }
        }
        Graph {
            own,
            steps,
            falsity,
            ends,
            pairs,
            half_pairs,
            work: NodeWork {
                saturations: 1,
                facts,
                applications: firings.len() as u64,
                steps: count,
                contradictions: contradictions.len() as u64,
                rounds: 0,
                merged: 0,
            },
        }
    }

    /// **The ways of one step**: one choice for each premise, the
    /// candidate it is or a class of its label, as distinct sets. `None`
    /// where a premise has neither yet. With `inherit` a way carries the
    /// statements of the classes chosen, which is how a fact beside its
    /// negation, no application's conclusion, names what it rests on.
    fn ways(
        &self,
        premises: &[u32],
        labels: &[Vec<Element>],
        (to, inherit): (To, bool),
        (question_of, clash): (&[Vec<u32>], &Clash),
        cut: &mut Cut,
    ) -> Option<Vec<(Way, Vec<u32>)>> {
        let mut acc: BTreeMap<Way, Vec<u32>> = BTreeMap::from([(
            Way {
                derived: Vec::new(),
                own: Vec::new(),
            },
            Vec::new(),
        )]);
        for &p in premises {
            let (own, label) = (self.own[p as usize], &labels[p as usize]);
            if own.is_none() && label.is_empty() {
                return None;
            }
            let mut next: BTreeMap<Way, Vec<u32>> = BTreeMap::new();
            for (way, reasons) in &acc {
                if let Some(c) = own {
                    let mut with = way.clone();
                    if let Err(at) = with.own.binary_search(&c) {
                        with.own.insert(at, c);
                    }
                    if to == To::Contradiction || !clash.within(&with.own) {
                        let held = next.entry(with).or_default();
                        *held = union(held, reasons);
                    }
                }
                for e in label {
                    let with = Way {
                        derived: union(&way.derived, &e.class),
                        own: way.own.clone(),
                    };
                    let held = next.entry(with).or_default();
                    *held = union(held, reasons);
                    if inherit {
                        *held = union(held, &e.reasons);
                    }
                }
            }
            if next.len() > WAYS {
                // Past the bound the ways are read as one that needs every
                // question any of them names: coarser, and still a
                // description of each.
                cut.ways += 1;
                let mut all = (Class::new(), Vec::new());
                for (way, reasons) in next {
                    all.0 = union(&all.0, &way.derived);
                    for &c in &way.own {
                        all.0 = union(&all.0, &question_of[c as usize]);
                    }
                    all.1 = union(&all.1, &reasons);
                }
                next = BTreeMap::from([(
                    Way {
                        derived: all.0,
                        own: Vec::new(),
                    },
                    all.1,
                )]);
            }
            acc = next;
        }
        Some(
            acc.into_iter()
                .filter(|(way, _)| {
                    to == To::Fact
                        || (way.own.len() == 2 && way.derived.is_empty())
                        || !clash.within(&way.own)
                })
                .collect(),
        )
    }

    /// The ways of fact `t`'s steps from the labels as they stand, each as
    /// the classes that describe it, with the statements its step reads.
    fn items(
        &self,
        t: u32,
        labels: &[Vec<Element>],
        read: (&[Vec<u32>], &Clash),
        cut: &mut Cut,
    ) -> Items {
        let question_of = read.0;
        let mut items = Items::default();
        for step in &self.steps[t as usize] {
            let Some(ways) = self.ways(&step.premises, labels, (To::Fact, false), read, cut) else {
                continue;
            };
            for (way, _) in ways {
                items.classes.push(descriptions(&way, question_of, cut));
                items.reasons.push(step.reasons.clone());
            }
        }
        items
    }

    /// The labels, settled in at most `rounds` rounds. Each round reads
    /// the ways of every fact one of whose premises moved in the round
    /// before, from the labels of that round, and then every fact's label
    /// as the cover of its ways, a class that describes as many of them as
    /// another going first where it describes more ways of the whole node.
    /// So the result does not depend on the order the applications came
    /// in. With them, the labels as the first round left them: every
    /// derived premise followed back one application.
    fn labels(
        &mut self,
        rounds: u32,
        read: (&[Vec<u32>], &Clash),
        cut: &mut Cut,
    ) -> (Vec<Vec<Element>>, Vec<Vec<Element>>) {
        let n = self.own.len();
        let mut labels: Vec<Vec<Element>> = vec![Vec::new(); n];
        let mut first: Option<Vec<Vec<Element>>> = None;
        let mut users: Vec<Vec<u32>> = vec![Vec::new(); n];
        let is_false: FxHashSet<u32> = self.falsity.iter().copied().collect();
        for of_one in &self.steps {
            for step in of_one {
                for &p in &step.premises {
                    users[p as usize].push(step.target);
                }
            }
        }
        // Nothing reads `(false)` as a premise: its ways are read once, at
        // the end.
        let derived: Vec<u32> = (0..n as u32)
            .filter(|t| !self.steps[*t as usize].is_empty() && !is_false.contains(t))
            .collect();
        let mut dirty: BTreeSet<u32> = derived.iter().copied().collect();
        let mut items: Vec<Items> = (0..n).map(|_| Items::default()).collect();
        let mut weight: Weights = Weights::default();
        let mut several = vec![false; n];
        let mut moved = true;
        while moved {
            if self.work.rounds == rounds {
                cut.unsettled = rounds == ROUNDS;
                break;
            }
            self.work.rounds += 1;
            for &t in &dirty {
                let new = self.items(t, &labels, read, cut);
                for class in items[t as usize].classes.iter().flatten() {
                    if let Some(w) = weight.get_mut(class) {
                        *w -= 1;
                    }
                }
                for class in new.classes.iter().flatten() {
                    *weight.entry(class.clone()).or_default() += 1;
                }
                items[t as usize] = new;
            }
            dirty.clear();
            moved = false;
            let mut next: Vec<(u32, Vec<Element>)> = Vec::new();
            for &t in &derived {
                let (new, more) = items[t as usize].label(&weight);
                several[t as usize] = more;
                if new != labels[t as usize] {
                    next.push((t, new));
                }
            }
            for (t, new) in next {
                labels[t as usize] = new;
                moved = true;
                dirty.extend(
                    users[t as usize]
                        .iter()
                        .copied()
                        .filter(|u| !is_false.contains(u)),
                );
            }
            if self.work.rounds == 1 {
                first = Some(labels.clone());
            }
        }
        self.work.merged = several.iter().filter(|&&x| x).count() as u64;
        let first = first.unwrap_or_else(|| labels.clone());
        (labels, first)
    }
}

/// The ways to one fact: each as the classes that describe it, and the
/// statements its step reads.
#[derive(Default)]
struct Items {
    classes: Vec<Vec<Class>>,
    reasons: Vec<Vec<u32>>,
}

impl Items {
    /// **The label they give: one class**, the union of the classes their
    /// cover chose, with whether it chose more than one. A fact with ways
    /// of two kinds is read as needing a member of every question either
    /// kind names: coarser than the ways, and every one of them is still
    /// described, since the class holds a question of each candidate each
    /// reads. Labels kept apart do not settle on a program whose
    /// derivations go round, and one class a fact does.
    fn label(&self, weight: &Weights) -> (Vec<Element>, bool) {
        let chosen = cover_of(&self.classes, weight);
        let several = chosen.len() > 1;
        let merged = chosen
            .into_iter()
            .fold(None, |all: Option<Element>, (class, held)| {
                let reasons = held
                    .iter()
                    .fold(Vec::new(), |r, &k| union(&r, &self.reasons[k as usize]));
                Some(match all {
                    None => Element { class, reasons },
                    Some(e) => Element {
                        class: union(&e.class, &class),
                        reasons: union(&e.reasons, &reasons),
                    },
                })
            });
        (merged.into_iter().collect(), several)
    }
}

/// A tie before it is listed: its pair, and what root says of it.
type TieKey = ((u32, u32), TieKind);

/// What an edge is being built from.
#[derive(Clone, Default)]
struct EdgeAcc {
    pairs: PairTies,
    node: NodeTies,
    debts: Vec<u32>,
    stated: BTreeMap<u32, u32>,
}

impl EdgeAcc {
    fn name(&mut self, reasons: &[u32]) {
        for &r in reasons {
            *self.stated.entry(r).or_default() += 1;
        }
    }
}

/// The edges of the whole node, into `edges`: the cover of its ways to a
/// contradiction, the cover of its ways to each committed candidate, and
/// the classes of every other derived fact's label. A class of fewer than
/// two questions is no edge.
fn node_edges(
    g: &Graph,
    labels: &[Vec<Element>],
    read: (&[Vec<u32>], &Clash),
    edges: &mut BTreeMap<Class, EdgeAcc>,
    cut: &mut Cut,
) {
    let question_of = read.0;
    // The ways to a contradiction.
    let mut items: Vec<Vec<Class>> = Vec::new();
    let mut reasons: Vec<Vec<u32>> = Vec::new();
    let mut take = |ways: Option<Vec<(Way, Vec<u32>)>>, own: &[u32], cut: &mut Cut| {
        for (way, inherited) in ways.into_iter().flatten() {
            items.push(descriptions(&way, question_of, cut));
            reasons.push(union(own, &inherited));
        }
    };
    for &t in &g.falsity {
        for step in &g.steps[t as usize] {
            let ways = g.ways(
                &step.premises,
                labels,
                (To::Contradiction, false),
                read,
                cut,
            );
            take(ways, &step.reasons, cut);
        }
    }
    for step in &g.ends {
        let ways = g.ways(
            &step.premises,
            labels,
            (To::Contradiction, false),
            read,
            cut,
        );
        take(ways, &step.reasons, cut);
    }
    for &(a, b) in &g.pairs {
        let ways = g.ways(&[a, b], labels, (To::Contradiction, true), read, cut);
        take(ways, &[], cut);
    }
    for &a in &g.half_pairs {
        let ways = g.ways(&[a], labels, (To::Contradiction, true), read, cut);
        take(ways, &[], cut);
    }
    for (class, held) in cover_of(&items, &Weights::default()) {
        if class.len() < 2 {
            continue;
        }
        let e = edges.entry(class).or_default();
        e.node.conflicts += held.len() as u32;
        for k in held {
            e.name(&reasons[k as usize]);
        }
    }

    // The ways to a committed candidate, and every other fact's label.
    let is_false: FxHashSet<u32> = g.falsity.iter().copied().collect();
    let mut items: Vec<Vec<Class>> = Vec::new();
    let mut reasons: Vec<&Vec<u32>> = Vec::new();
    for (t, label) in labels.iter().enumerate() {
        if is_false.contains(&(t as u32)) {
            continue;
        }
        match g.own[t] {
            Some(c) => {
                for e in label {
                    let way = Way {
                        derived: e.class.clone(),
                        own: vec![c],
                    };
                    items.push(descriptions(&way, question_of, cut));
                    reasons.push(&e.reasons);
                }
            }
            None => {
                for e in label.iter().filter(|e| e.class.len() >= 2) {
                    let edge = edges.entry(e.class.clone()).or_default();
                    edge.node.derivations += 1;
                    edge.name(&e.reasons);
                }
            }
        }
    }
    for (class, held) in cover_of(&items, &Weights::default()) {
        if class.len() < 2 {
            continue;
        }
        let e = edges.entry(class).or_default();
        e.node.implications += held.len() as u32;
        for k in held {
            e.name(reasons[k as usize]);
        }
    }
}

impl Coupling {
    /// The coupling of the program `theory` and `cover` were read of, over
    /// its prepared space: the ties off root's pair layer, and where
    /// `opts.reading` asks, the whole node saturated through `sp`.
    fn read(
        sp: &mut Space<'_>,
        theory: &Theory,
        cover: &Cover,
        opts: &CouplingOptions,
    ) -> Result<Coupling, SolveError> {
        let n = cover.n();
        let root = &theory.root;

        // The questions.
        let mut questions: Vec<Question> = (0..cover.groups.len() as u32)
            .map(Question::Group)
            .collect();
        let mut question_of: Vec<Vec<u32>> = cover.member_of.clone();
        for b in 0..n as u32 {
            if question_of[b as usize].is_empty() {
                question_of[b as usize].push(questions.len() as u32);
                questions.push(Question::Alone(b));
            }
        }

        // The puzzle's statements, by place.
        let puzzle = Puzzle {
            place: theory
                .stated
                .iter()
                .enumerate()
                .map(|(k, s)| (s.fact, k as u32))
                .collect(),
            theory,
        };
        let mut edges: BTreeMap<Class, EdgeAcc> = BTreeMap::new();

        // The ties of the pair layer: what root says of a pair that no
        // question holds.
        let alive = |b: u32| cover.marks[b as usize] == Mark::AliveAlone;
        let apart = |a: u32, b: u32| {
            alive(a)
                && alive(b)
                && !cover.excludes(a, b)
                && !question_of[a as usize]
                    .iter()
                    .any(|q| question_of[b as usize].binary_search(q).is_ok())
        };
        let mut ties: BTreeMap<TieKey, Option<Vec<u32>>> = BTreeMap::new();
        for (k, &(a, b)) in root.excluded.iter().enumerate() {
            if apart(a, b) {
                let stated = theory.supports.get(k).filter(|s| s.read).map(|s| {
                    s.stated
                        .iter()
                        .copied()
                        .filter(|&k| !theory.stated[k as usize].kept)
                        .collect()
                });
                ties.insert(((a, b), TieKind::Excludes), stated);
            }
        }
        for &(a, b) in &root.implies {
            if apart(a, b) {
                ties.insert(((a.min(b), a.max(b)), TieKind::Implies), None);
            }
        }
        for &(a, b) in &root.interacting {
            if apart(a, b) {
                ties.insert(((a, b), TieKind::Interacts), None);
            }
        }
        let mut cut = Cut::default();
        let items: Vec<Vec<Class>> = ties
            .keys()
            .map(|&((a, b), _)| {
                descriptions(
                    &Way {
                        derived: Vec::new(),
                        own: vec![a, b],
                    },
                    &question_of,
                    &mut cut,
                )
            })
            .collect();
        let tie_list: Vec<(&TieKey, &Option<Vec<u32>>)> = ties.iter().collect();
        for (class, held) in cover_of(&items, &Weights::default()) {
            let e = edges.entry(class).or_default();
            for k in held {
                let (&(_, kind), stated) = tie_list[k as usize];
                match kind {
                    TieKind::Excludes => e.pairs.excludes += 1,
                    TieKind::Implies => e.pairs.implies += 1,
                    TieKind::Interacts => e.pairs.interacts += 1,
                }
                if let Some(stated) = stated {
                    e.name(stated);
                }
            }
        }

        // The debts: every instance root owes whose payers lie in two
        // questions or more. L5's alternatives where it licenses the
        // instance, and otherwise the candidates whose own node holds one
        // of its witnesses.
        let committed: Vec<u32> = (0..n as u32).filter(|&b| alive(b)).collect();
        let mut debts: Vec<Debt> = Vec::new();
        let mut unlicensed = 0u32;
        // An unlicensed instance's payers are its witnesses' holders, which
        // are read where root's singles were.
        let payers_read = theory.owed.singles_read
            || !theory
                .owed
                .table()
                .iter()
                .any(|i| i.alternatives.is_err() && i.witnesses.is_ok());
        for i in theory.owed.table() {
            let licensed = i.alternatives.is_ok();
            unlicensed += u32::from(!licensed);
            let mut payers: Vec<u32> = match (&i.alternatives, &i.witnesses) {
                (Ok(alternatives), _) => alternatives.clone(),
                (Err(_), Ok(witnesses)) if theory.owed.singles_read => witnesses
                    .iter()
                    .flat_map(|w| w.held_at_root.iter().copied())
                    .collect(),
                _ => Vec::new(),
            };
            payers.retain(|&b| alive(b));
            payers.sort_unstable();
            payers.dedup();
            let one_question = payers.split_first().is_some_and(|(&first, rest)| {
                question_of[first as usize].iter().any(|q| {
                    rest.iter()
                        .all(|&b| question_of[b as usize].binary_search(q).is_ok())
                })
            });
            if payers.len() < 2 || one_question {
                continue;
            }
            let items: Vec<Vec<Class>> = payers
                .iter()
                .map(|&b| question_of[b as usize].iter().map(|&q| vec![q]).collect())
                .collect();
            let over = cover_of(&items, &Weights::default())
                .into_iter()
                .fold(Class::new(), |all, (class, _)| union(&all, &class));
            edges
                .entry(over.clone())
                .or_default()
                .debts
                .push(debts.len() as u32);
            debts.push(Debt {
                rule: i.rule.clone(),
                instance: i.text.clone(),
                licensed,
                payers,
                questions: over,
            });
        }

        // The whole node.
        let (mut node, mut node_stopped) = (None, None);
        let mut one_step: Option<BTreeMap<Class, EdgeAcc>> = None;
        if let Some(rounds) = opts.reading.rounds() {
            node_stopped = if root.root_dead {
                Some(NodeStop::RootDead)
            } else if root.singles_left != 0 {
                Some(NodeStop::SinglesUnread)
            } else if committed.is_empty() {
                Some(NodeStop::Empty)
            } else if opts
                .layer
                .deadline
                .is_some_and(|d| std::time::Instant::now() >= d)
            {
                Some(NodeStop::Deadline)
            } else {
                None
            };
            if node_stopped.is_none() {
                let facts: FxHashMap<FactId, u32> = committed
                    .iter()
                    .map(|&b| (cover.candidates[b as usize].fact, b))
                    .collect();
                let refuted =
                    crate::contradiction::Pairs::of(sp.plans().iter().map(|p| &**p), sp.terms());
                let (root_kb, _, _) = sp.parts();
                let read = sp.supported(&committed, NODE_APPLICATIONS, |kb, terms, firings| {
                    Graph::of(root_kb, kb, terms, firings, &facts, Some(&puzzle), &refuted)
                })?;
                match read {
                    None => node_stopped = Some(NodeStop::Applications),
                    Some(g) if g.work.facts > opts.layer.budget => {
                        node = Some(g.work);
                        node_stopped = Some(NodeStop::Budget);
                    }
                    Some(mut g) => {
                        let clash = Clash::of(cover, root);
                        let read = (&question_of[..], &clash);
                        let (labels, first) = g.labels(rounds, read, &mut cut);
                        if opts.reading == CouplingReading::Node {
                            let mut shallow = edges.clone();
                            node_edges(&g, &first, read, &mut shallow, &mut cut);
                            one_step = Some(shallow);
                        }
                        node_edges(&g, &labels, read, &mut edges, &mut cut);
                        node = Some(g.work);
                    }
                }
            }
        }
        if opts.reading == CouplingReading::Node && one_step.is_none() {
            // No node was read: one application back is the pairs and the
            // debts, as the whole node is.
            one_step = Some(edges.clone());
        }

        // The statements named, in the loader's order, and the edges.
        let mut named: BTreeSet<u32> = BTreeSet::new();
        for stated in ties.values().flatten() {
            named.extend(stated);
        }
        for e in edges
            .values()
            .chain(one_step.iter().flat_map(|m| m.values()))
        {
            named.extend(e.stated.keys());
        }
        let place: BTreeMap<u32, u32> = named
            .iter()
            .enumerate()
            .map(|(at, &k)| (k, at as u32))
            .collect();
        let reasons: Vec<String> = named
            .iter()
            .map(|&k| theory.stated[k as usize].text.clone())
            .collect();
        let ties: Vec<Tie> = ties
            .into_iter()
            .map(|((pair, kind), stated)| Tie {
                pair,
                kind,
                stated: stated.map(|s| s.iter().map(|k| place[k]).collect()),
            })
            .collect();
        let listed = |edges: BTreeMap<Class, EdgeAcc>| -> Vec<Edge> {
            edges
                .into_iter()
                .map(|(questions, acc)| {
                    let mut stated: Vec<(u32, u32)> =
                        acc.stated.iter().map(|(k, &c)| (place[k], c)).collect();
                    stated.sort_by_key(|&(k, c)| (std::cmp::Reverse(c), k));
                    Edge {
                        questions,
                        pairs: acc.pairs,
                        node: acc.node,
                        debts: acc.debts,
                        stated,
                    }
                })
                .collect()
        };
        let edges = listed(edges);
        let one_step = one_step.map(listed);

        // The components: the candidates alive alone, together where they
        // share a question or an edge holds a question of each.
        fn find(parent: &mut [u32], mut a: u32) -> u32 {
            while parent[a as usize] != a {
                parent[a as usize] = parent[parent[a as usize] as usize];
                a = parent[a as usize];
            }
            a
        }
        let mut open: Vec<Vec<u32>> = vec![Vec::new(); questions.len()];
        for &b in &committed {
            for &q in &question_of[b as usize] {
                open[q as usize].push(b);
            }
        }
        let components_over = |with_debts: bool| -> Vec<Vec<u32>> {
            let mut parent: Vec<u32> = (0..n as u32).collect();
            let unite = |parent: &mut Vec<u32>, a: u32, b: u32| {
                let (x, y) = (find(parent, a), find(parent, b));
                if x != y {
                    parent[x.max(y) as usize] = x.min(y);
                }
            };
            for members in &open {
                for w in members.windows(2) {
                    unite(&mut parent, w[0], w[1]);
                }
            }
            for e in edges.iter().filter(|e| with_debts || e.is_tie()) {
                let mut first: Option<u32> = None;
                for &q in &e.questions {
                    if let Some(&b) = open[q as usize].first() {
                        match first {
                            None => first = Some(b),
                            Some(a) => unite(&mut parent, a, b),
                        }
                    }
                }
            }
            let mut by_root: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
            for &b in &committed {
                by_root.entry(find(&mut parent, b)).or_default().push(b);
            }
            by_root.into_values().collect()
        };
        let components = components_over(true);
        let conflict_components = components_over(false);

        let node_read = node.is_some() && node_stopped.is_none();
        Ok(Coupling {
            candidates: cover.candidates.clone(),
            reading: opts.reading,
            monotone: root.monotone,
            questions,
            question_of,
            pairs_read: root.complete(),
            ties,
            reasons,
            edges,
            one_step,
            components,
            conflict_components,
            debts,
            payers_read,
            unlicensed,
            committed: if node_read { committed } else { Vec::new() },
            node,
            node_stopped,
            cut,
        })
    }
}

/// The theory's share a coupling asks: root's pairs, and the supports
/// where asked.
fn theory_asked(opts: &CouplingOptions) -> TheoryOptions {
    TheoryOptions {
        layer: opts.layer,
        root_pairs: true,
        supports: opts.supports,
        check: false,
    }
}

/// **The cover and the coupling over a program's prepared space**,
/// computed once a program, as `cover_in` computes the cover: root's
/// singles and pairs are read through `sp`, the routes' node cache, the
/// small KB's layer in a space of its own, and the whole node, where
/// `opts.reading` asks it, is the node function's own fork of `sp`'s root,
/// saturated once with every candidate alive alone committed. That node is
/// not a node of the cache and is charged to no budget of `sp`'s:
/// [`Coupling::node`] is its cost. Nothing is written that a node reads.
/// An error where the small KB has no layer.
pub(crate) fn coupling_in(
    sp: &mut Space<'_>,
    loaded: &Loaded,
    opts: &CouplingOptions,
) -> Result<(Cover, Coupling), SolveError> {
    let t = theory_in(sp, loaded, Reading::Pairs, &theory_asked(opts), &small_kb)?;
    let Some(cover) = t.cover() else {
        return Err(SolveError::Unsupported(format!(
            "the small KB has no layer, so the program has no questions: {}",
            t.small.as_ref().err().map_or("", String::as_str)
        )));
    };
    let coupling = Coupling::read(sp, &t, &cover, opts)?;
    Ok((cover, coupling))
}

/// The cover and the coupling of `root`, a **freshly loaded, unsaturated**
/// program nothing else will use: `coupling_in` over a space opened for
/// it, at `opts.layer`'s budget.
pub fn coupling(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &ein_ir::Ast,
    opts: &CouplingOptions,
) -> Result<(Cover, Coupling), SolveError> {
    let loaded = Loaded::of(root);
    with_space(root, terms, ast, &opts.layer, None, |sp| {
        coupling_in(sp, &loaded, opts)
    })
}

/// **The theory's share with the coupling beside it**, of `root`, a
/// freshly loaded, unsaturated program nothing else will use: what
/// [`super::theory`] reads at `opts`, and over the same space the cover and
/// the coupling by `reading`, for a reader that wants all three of one
/// reading of root. `None` where the small KB has no layer or root's pairs
/// were not asked.
pub fn theory_coupled(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &ein_ir::Ast,
    opts: &TheoryOptions,
    reading: CouplingReading,
) -> Result<(Theory, Option<(Cover, Coupling)>), SolveError> {
    let loaded = Loaded::of(root);
    with_space(root, terms, ast, &opts.layer, None, |sp| {
        let how = if opts.root_pairs {
            Reading::Pairs
        } else {
            Reading::Universe
        };
        let t = theory_in(sp, &loaded, how, opts, &small_kb)?;
        let coupled = match t.cover() {
            Some(cover) if opts.root_pairs => {
                let asked = CouplingOptions {
                    layer: opts.layer,
                    reading,
                    supports: opts.supports,
                };
                let coupling = Coupling::read(sp, &t, &cover, &asked)?;
                Some((cover, coupling))
            }
            _ => None,
        };
        Ok((t, coupled))
    })
}

/// How [`whole`] is asked.
#[derive(Clone, Copy, Debug, Default)]
pub struct WholeOptions {
    /// The budget, the jobs and the deadline of root's layer and of the
    /// small KB's, each read at it, as [`TheoryOptions::layer`].
    pub layer: Options,
    /// Read the support of every pair root excludes ([`super::Support`]).
    pub supports: bool,
}

/// What [`Whole`] holds past the theory's share, where the small KB has a
/// layer: the three objects read against it.
#[derive(Clone, Debug)]
pub struct Parts {
    pub cover: Cover,
    pub exactly_one: ExactlyOne,
    pub coupling: Coupling,
}

/// **Every object of the structure, of one reading of root**: the theory's
/// share with root's pair layer and the small KB's, and over the same space
/// the cover, *exactly one* and the coupling under the reading the owner
/// takes ([`CouplingReading::Node`], which keeps the edges one application
/// back beside its own). Each layer is read once.
#[derive(Clone, Debug)]
pub struct Whole {
    pub theory: Theory,
    /// `None` where the small KB has no layer ([`Theory::small`] says why):
    /// the program then has no groups and no questions by this reading.
    pub parts: Option<Parts>,
    /// **What the program declares, beside**: the hypothesis mechanisms
    /// ([`crate::mechanisms::read`]) at root's fixpoint, after the reading
    /// and of the same root. The message where a plan does not compile.
    /// Nothing above is read from them.
    pub declared: Result<Mechanisms, String>,
}

/// [`Whole`], of `root`, a **freshly loaded, unsaturated** program nothing
/// else will use: [`theory_coupled`] with root's pairs asked and no check,
/// [`ExactlyOne::of`] the cover it found and what root owes, and the
/// mechanisms read of the root it left. For a reader that wants the whole
/// structure of one program and pays each pair layer once.
pub fn whole(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &ein_ir::Ast,
    opts: &WholeOptions,
) -> Result<Whole, SolveError> {
    let asked = TheoryOptions {
        layer: opts.layer,
        root_pairs: true,
        supports: opts.supports,
        check: false,
    };
    let (theory, coupled) = theory_coupled(root, terms, ast, &asked, CouplingReading::Node)?;
    let parts = coupled.map(|(cover, coupling)| {
        let exactly_one = ExactlyOne::of(&cover, theory.owed.clone());
        Parts {
            cover,
            exactly_one,
            coupling,
        }
    });
    let declared = {
        let mut off = Events::off();
        let mut s = Session {
            kb: root,
            terms,
            ast,
            events: &mut off,
            memo: SharedMemo::default(),
        };
        crate::mechanisms::read(&mut s).map_err(|e| e.to_string())
    };
    Ok(Whole {
        theory,
        parts,
        declared,
    })
}
