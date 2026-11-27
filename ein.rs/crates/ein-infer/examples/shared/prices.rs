//! What a question about `R` costs, read against the structure of `H`: the
//! nodes a route visited, each put in one part of a split, and each way of
//! spending the structure simulated on them. Shared by the `price_list`
//! example and `tests/price_list.rs`.
//!
//! **How a route's nodes are read.** Through [`ein_infer::events::Watch`],
//! the reader `--dump-lattice` uses: every node a route computes is handed
//! to it on the committing thread, in visit order, with its commitment, what
//! it is, its closure and its facts beyond root. A watch sees a node and
//! changes none, and [`ask`] is run twice a cell, once plain and once
//! watched, so the two runs' work is compared on every cell.
//!
//! **The closures under a commitment.** A route extends a state by one
//! candidate, so a commitment `C` is read with what the run already knew
//! held under it: `K(C)`, `C` and the closures of the alive nodes visited
//! **before** it whose commitment lies inside it. On a monotone program
//! every fact of `K(C)` is in `C`'s fixpoint, so a dead set inside `K(C)`
//! is a reason `C` is dead that the run could have read before saturating
//! it. Nothing visited after `C` is read for it.
//!
//! **The split**, of a cell's saturations, each node in exactly one part:
//!
//! | part | the node |
//! |---|---|
//! | `in_r` | alive, owes nothing |
//! | `owing_owed_group` | alive, and owes an instance whose alternatives are an owed group |
//! | `owing_other` | alive and owing, no such instance |
//! | `dead_theory` | dead, `K` holds two members of one group: a pair the theory excludes |
//! | `dead_puzzle` | dead otherwise, `K` holds a pair root excludes that the theory does not |
//! | `dead_single` | dead otherwise, `K` holds a candidate dead alone |
//! | `dead_edge` | dead otherwise, and its core lies inside one edge of the coupling's whole-node reading (its commitment, where no core was read) |
//! | `dead_other` | dead otherwise |
//!
//! Which instances an alive node owes is read off its closure: a licensed
//! instance is owed exactly when none of its alternatives is in the closure,
//! which is the premise the obligation tree checks on every node it
//! registers.
//!
//! **The ways**, each a [`Delta`]: the saturations and facts it would have
//! added, by the object that costs them, and those it would have removed,
//! with the route's order held as it was. A way is applied on a monotone
//! program only: on one the static check flags no dead set says anything of
//! its supersets, a route reads no structure and pays for none.
//!
//! | way | what it reads | what it removes |
//! |---|---|---|
//! | `layer-whole` | root's pair layer, before the first node | a dead node whose `K` holds a candidate dead alone or a pair root excludes |
//! | `layer-by-class` | the same layer with L3 read over the classes of candidates with one closure: of the pairs between two classes one node | the same |
//! | `singles` | root's node and the singles | a dead node whose `K` holds a candidate dead alone |
//! | `denials` | the candidates each alive node's own fixpoint denies, a stored negation: read where the node is computed, no saturation | a dead node whose `K` holds a candidate that an alive node visited before it and inside it denies |
//! | `layer-by-need` | the same layer's nodes, each when first asked: before a node of two candidates or more, every pair of `K` not inside an alive closure the run holds, singles first, ascending, up to the first dead | the node, where a pair asked is dead |
//! | `death-pairs` | the same, asked only after a node of three or more has died | later dead nodes whose `K` holds a pair or a single learned |
//! | `death-core-pair` | the core of each death, where it is of one or two candidates; no saturation | later dead nodes whose `K` holds a core learned |
//! | `death-core` | the core of each death, of any size | the same |
//! | `theory` | the small KB's pair layer | a dead node whose `K` holds a pair the theory excludes or a candidate it kills alone |
//! | `theory-shared` | the same pairs, their cost paid once for every puzzle of the ontology and so not here: a bound | the same |
//! | `cover` | the small KB's layer and root's singles | `theory`'s, and a dead node whose `K` holds a candidate dead alone |
//! | `wipe-out` | the cover and what root owes | `cover`'s, and under `--single`, `-n 1` and `--maximal` any node whose `K` leaves an owed group no member: each dead alone or excluded by the theory with a member of `K` |
//! | `wipe-out-layer` | the same with root's pair layer | the same, a member also out where root excludes it with a member of `K` |
//! | `factor` | the coupling: both pair layers and the whole node | every node but the distinct projections of the visited nodes onto the components |
//!
//! `factor` is exact where every projection is itself a visited node, which
//! a describe's are; under a tree it is an estimate and says so.
//!
//! **What an alive node denies** is not in what a watch is handed, so it is
//! read apart ([`Denials`]): each alive node's fixpoint computed again by the
//! node function ([`ein_infer::region::Nodes`]) and asked, for each
//! candidate, whether its negation is stored. Those saturations are the
//! reader's and no cell's.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use ein_core::{Kb, Terms};
use ein_infer::events::{Events, Visited, Watch};
use ein_infer::explore::{self, Ask, Mode, Route, Stopped, Which};
use ein_infer::region::{Cores, Nodes};
use ein_infer::structure::{
    self, Coupling, CouplingReading, Cover, Edge, ExactlyOne, Owing, Theory, TheoryOptions,
};
use ein_ir::{Ast, load, parse};
use rustc_hash::{FxHashMap, FxHashSet};

// ── Sets ────────────────────────────────────────────────────────────────

/// A set of candidates.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bits(Vec<u64>);

impl Bits {
    pub fn new(n: usize) -> Bits {
        Bits(vec![0; n.div_ceil(64).max(1)])
    }

    pub fn of(n: usize, set: &[u32]) -> Bits {
        let mut b = Bits::new(n);
        for &x in set {
            b.insert(x);
        }
        b
    }

    pub fn insert(&mut self, b: u32) {
        self.0[b as usize / 64] |= 1 << (b % 64);
    }

    pub fn has(&self, b: u32) -> bool {
        self.0[b as usize / 64] >> (b % 64) & 1 == 1
    }

    pub fn or(&mut self, o: &Bits) {
        for (x, y) in self.0.iter_mut().zip(&o.0) {
            *x |= *y;
        }
    }

    pub fn meets(&self, o: &Bits) -> bool {
        self.0.iter().zip(&o.0).any(|(x, y)| x & y != 0)
    }

    pub fn within(&self, o: &Bits) -> bool {
        self.0.iter().zip(&o.0).all(|(x, y)| x & !y == 0)
    }

    pub fn list(&self) -> Vec<u32> {
        let mut out = Vec::new();
        for (w, &word) in self.0.iter().enumerate() {
            let mut x = word;
            while x != 0 {
                out.push(w as u32 * 64 + x.trailing_zeros());
                x &= x - 1;
            }
        }
        out
    }
}

/// A set of pairs of candidates.
#[derive(Clone)]
pub struct PairSet {
    rows: Vec<Bits>,
}

impl PairSet {
    pub fn new(n: usize) -> PairSet {
        PairSet {
            rows: vec![Bits::new(n); n],
        }
    }

    pub fn of(n: usize, pairs: &[(u32, u32)]) -> PairSet {
        let mut p = PairSet::new(n);
        for &(a, b) in pairs {
            p.insert(a, b);
        }
        p
    }

    pub fn insert(&mut self, a: u32, b: u32) {
        self.rows[a as usize].insert(b);
        self.rows[b as usize].insert(a);
    }

    pub fn has(&self, a: u32, b: u32) -> bool {
        self.rows[a as usize].has(b)
    }

    /// The candidates paired with `a`.
    pub fn row(&self, a: u32) -> &Bits {
        &self.rows[a as usize]
    }

    /// `k` holds both members of some pair.
    pub fn any_in(&self, list: &[u32], k: &Bits) -> bool {
        list.iter().any(|&a| self.rows[a as usize].meets(k))
    }
}

/// Sets of candidates, each ascending, asked for those inside a given set.
struct Trie {
    edges: FxHashMap<(u32, u32), u32>,
    ends: Vec<Option<u32>>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            edges: FxHashMap::default(),
            ends: vec![None],
        }
    }

    fn insert(&mut self, set: &[u32], id: u32) {
        let mut at = 0u32;
        for &x in set {
            let next = self.ends.len() as u32;
            let to = *self.edges.entry((at, x)).or_insert(next);
            if to == next {
                self.ends.push(None);
            }
            at = to;
        }
        self.ends[at as usize].get_or_insert(id);
    }

    /// `f` of every stored set inside `set`; stops where `f` says so.
    fn subsets(&self, set: &[u32], f: &mut impl FnMut(u32) -> bool) -> bool {
        let mut stack: Vec<(u32, usize)> = vec![(0, 0)];
        while let Some((at, from)) = stack.pop() {
            if let Some(id) = self.ends[at as usize]
                && f(id)
            {
                return true;
            }
            for (j, &x) in set.iter().enumerate().skip(from) {
                if let Some(&to) = self.edges.get(&(at, x)) {
                    stack.push((to, j + 1));
                }
            }
        }
        false
    }

    fn any_subset(&self, set: &[u32]) -> bool {
        self.subsets(set, &mut |_| true)
    }
}

// ── The structure, read once a program ──────────────────────────────────

/// One node of root's pair layer.
#[derive(Clone, Debug)]
pub struct LayerNode {
    pub dead: bool,
    pub facts: u64,
    pub closure: Box<[u32]>,
}

/// A program's structure, as the owner reads it, and root's pair layer node
/// by node.
pub struct Read {
    pub n: usize,
    pub texts: Vec<String>,
    index: Arc<FxHashMap<String, u32>>,
    pub theory: Theory,
    pub cover: Option<Cover>,
    pub exact: Option<ExactlyOne>,
    pub coupling: Option<Coupling>,
    /// Root's layer: root's node, every single and every pair the licences
    /// leave, each by its commitment less the candidates root holds.
    pub layer: FxHashMap<Box<[u32]>, LayerNode>,
    /// The layer read node by node is the one the theory's share holds, list
    /// for list and in work, and its nodes' count and facts are its work.
    pub layer_same: bool,
    pub at_root: Bits,
    pub dead_alone: Bits,
    /// The pairs root excludes; the theory's, over all of `H`; and root's
    /// that are not the theory's.
    pub root_pairs: PairSet,
    pub theory_pairs: PairSet,
    pub puzzle_pairs: PairSet,
    /// The candidates the theory kills alone.
    pub killed_alone: Bits,
    /// Each licensed instance of what root owes: its alternatives, and the
    /// group they are where they are one.
    pub licensed: Vec<(Bits, Option<u32>)>,
    /// Seconds: the structure whole, and the layer read node by node.
    pub wall: f64,
    pub wall_layer: f64,
}

pub fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// What the watch keeps of a run.
#[derive(Default)]
struct Kept {
    nodes: Vec<Node>,
    /// Texts no candidate has, in a commitment or a closure.
    unmapped: u64,
}

struct Collect {
    index: Arc<FxHashMap<String, u32>>,
    kept: Arc<Mutex<Kept>>,
}

impl Collect {
    fn set(&self, texts: &[String]) -> (Box<[u32]>, u64) {
        let mut miss = 0;
        let mut out: Vec<u32> = texts
            .iter()
            .filter_map(|t| {
                let b = self.index.get(t).copied();
                miss += u64::from(b.is_none());
                b
            })
            .collect();
        out.sort_unstable();
        out.dedup();
        (out.into_boxed_slice(), miss)
    }
}

impl Watch for Collect {
    fn node(&mut self, v: &Visited<'_>) {
        let kind = match v.kind {
            "dead" => Kind::Dead,
            "owing" => Kind::Owing,
            _ => Kind::InR,
        };
        let (set, miss) = self.set(v.commitment);
        let (closure, miss_closure) = if kind == Kind::Dead {
            (Box::default(), 0)
        } else {
            self.set(v.closure)
        };
        // A core names the facts a death rests on: stated ones, which every
        // node holds, and committed ones. Its candidates are what is kept.
        let core = (kind == Kind::Dead && v.core_status.is_some())
            .then(|| self.set(v.core).0)
            .filter(|c| !c.is_empty());
        let mut kept = self.kept.lock().expect("one thread watches");
        kept.unmapped += miss + miss_closure;
        kept.nodes.push(Node {
            set,
            kind,
            owes: v.owes,
            facts: v.facts,
            closure,
            core_read: kind == Kind::Dead && v.core_status.is_some(),
            core,
        });
    }
}

impl Read {
    /// The structure of the program at `path`, loaded twice: once for the
    /// owner's reading, the theory's share with the cover, what root owes
    /// and the coupling, and once for root's layer node by node.
    pub fn of(path: &Path) -> Result<Read, String> {
        let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
        let t = Instant::now();
        let (theory, coupled) = structure::theory_coupled(
            &mut kb,
            &mut terms,
            &ast,
            &TheoryOptions::default(),
            CouplingReading::Node,
        )
        .map_err(|e| e.to_string())?;
        let wall = t.elapsed().as_secs_f64();
        drop((kb, terms, ast));
        let exact = theory.exactly_one();
        let (cover, coupling) = match coupled {
            Some((c, k)) => (Some(c), Some(k)),
            None => (exact.as_ref().map(|(c, _)| c.clone()), None),
        };
        let exact = exact.map(|(_, x)| x);

        let l = &theory.root;
        let n = l.n();
        let texts: Vec<String> = l.candidates.iter().map(|c| c.text.clone()).collect();
        let index: Arc<FxHashMap<String, u32>> = Arc::new(
            texts
                .iter()
                .enumerate()
                .map(|(i, t)| (t.clone(), i as u32))
                .collect(),
        );
        let at_root = Bits::of(n, &l.at_root);

        // Root's layer again, node by node.
        let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
        let kept = Arc::new(Mutex::new(Kept::default()));
        let mut events = Events::off().watched(Box::new(Collect {
            index: index.clone(),
            kept: kept.clone(),
        }));
        let t = Instant::now();
        let again = structure::pair_layer_with(
            &mut kb,
            &mut terms,
            &ast,
            &structure::Options::default(),
            &mut events,
        )
        .map_err(|e| e.to_string())?;
        let wall_layer = t.elapsed().as_secs_f64();
        drop(events);
        drop((kb, terms, ast));
        let kept = std::mem::take(&mut *kept.lock().expect("the run is over"));
        let mut layer: FxHashMap<Box<[u32]>, LayerNode> = FxHashMap::default();
        let mut facts = 0u64;
        for x in &kept.nodes {
            facts += x.facts;
            layer.insert(
                less(&x.set, &at_root),
                LayerNode {
                    dead: x.kind == Kind::Dead,
                    facts: x.facts,
                    closure: x.closure.clone(),
                },
            );
        }
        let layer_same = kept.unmapped == 0
            && again.work == l.work
            && again.dead_alone == l.dead_alone
            && again.excluded == l.excluded
            && again.implies == l.implies
            && kept.nodes.len() as u64 == l.work.saturations
            && layer.len() == kept.nodes.len()
            && facts == l.work.facts;

        let root_pairs = PairSet::of(n, &l.excluded);
        let theory_list: &[(u32, u32)] = cover.as_ref().map_or(&[], |c| &c.excluded);
        let theory_pairs = PairSet::of(n, theory_list);
        let mut puzzle_pairs = PairSet::new(n);
        for &(a, b) in &l.excluded {
            if !theory_pairs.has(a, b) {
                puzzle_pairs.insert(a, b);
            }
        }
        let killed_alone = Bits::of(n, cover.as_ref().map_or(&[], |c| &c.killed_alone));
        let licensed = exact.as_ref().map_or_else(Vec::new, |x| {
            x.owed
                .table()
                .iter()
                .zip(&x.owing)
                .filter_map(|(i, how)| {
                    let alts = i.alternatives.as_ref().ok()?;
                    let group = match how {
                        Owing::Group(g) => Some(*g),
                        _ => None,
                    };
                    Some((Bits::of(n, alts), group))
                })
                .collect()
        });
        Ok(Read {
            n,
            index,
            dead_alone: Bits::of(n, &l.dead_alone),
            texts,
            cover,
            exact,
            coupling,
            layer,
            layer_same,
            at_root,
            root_pairs,
            theory_pairs,
            puzzle_pairs,
            killed_alone,
            licensed,
            wall,
            wall_layer,
            theory,
        })
    }

    pub fn monotone(&self) -> bool {
        self.theory.root.monotone
    }

    /// What the small KB's layer cost, in its own root's terms.
    pub fn small_work(&self) -> [u64; 2] {
        self.theory
            .small
            .as_ref()
            .map_or([0, 0], |s| [s.work.saturations, s.work.facts])
    }

    /// What the coupling's whole node cost.
    pub fn node_work(&self) -> [u64; 2] {
        self.coupling
            .as_ref()
            .and_then(|c| c.node)
            .map_or([0, 0], |w| [w.saturations, w.facts])
    }

    /// Root's layer by its two halves: root's node with the singles, and
    /// the pairs.
    pub fn layer_work(&self) -> [[u64; 2]; 2] {
        let mut out = [[0u64; 2]; 2];
        // determinism-ok: a sum over the values.
        for (set, x) in &self.layer {
            let half = usize::from(set.len() >= 2);
            out[half][0] += 1;
            out[half][1] += x.facts;
        }
        out
    }
}

/// `set` without the candidates of `drop`.
fn less(set: &[u32], drop: &Bits) -> Box<[u32]> {
    set.iter().copied().filter(|&b| !drop.has(b)).collect()
}

// ── A question, asked ───────────────────────────────────────────────────

/// The six questions `ein solve` asks: a key, the flags, the mode.
pub fn questions() -> [(&'static str, &'static str, Mode); 6] {
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Dead,
    Owing,
    InR,
}

/// One node a route visited, its candidates by index.
#[derive(Clone, Debug)]
pub struct Node {
    pub set: Box<[u32]>,
    pub kind: Kind,
    pub owes: u32,
    /// Its fixpoint's facts beyond root's.
    pub facts: u64,
    /// An alive node's closure.
    pub closure: Box<[u32]>,
    /// A dead node's core was read.
    pub core_read: bool,
    /// The candidates of its core, where one was read and names any: with
    /// what the program states, committing them is enough for the death.
    pub core: Option<Box<[u32]>>,
}

/// What one run answered and what it cost.
#[derive(Clone, Debug)]
pub struct Run {
    pub saturations: u64,
    pub facts: u64,
    pub wall: f64,
    pub route: Route,
    pub exhausted: bool,
    pub verdict: Option<&'static str>,
    pub stopped: Option<Stopped>,
    /// The commitments of the states the run lists.
    pub states: Vec<Box<[u32]>>,
}

impl Run {
    /// Two runs of one question answered alike and did the same work.
    pub fn same(&self, o: &Run) -> bool {
        self.saturations == o.saturations
            && self.facts == o.facts
            && self.route == o.route
            && self.exhausted == o.exhausted
            && self.verdict == o.verdict
            && self.stopped.is_some() == o.stopped.is_some()
            && self.states == o.states
    }
}

/// Ask `mode` of the program at `path`, loaded again, at the default budget
/// with the options `ein solve` gives the question; `watch` hands every node
/// to a reader and takes a core at every death, as `--dump-lattice` does.
/// The nodes visited, in visit order, and how many texts no candidate has.
pub fn ask(
    path: &Path,
    key: &str,
    mode: Mode,
    watch: Option<&Read>,
) -> Result<(Run, Vec<Node>, u64), String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
    let opts = explore::Options {
        cores: Cores {
            root: true,
            visited: watch.is_some(),
            nogoods: false,
            holes: true,
        },
        nogoods: key != "count",
        ..explore::Options::new(mode)
    };
    let kept = Arc::new(Mutex::new(Kept::default()));
    let mut events = match watch {
        None => Events::off(),
        Some(r) => Events::off().watched(Box::new(Collect {
            index: r.index.clone(),
            kept: kept.clone(),
        })),
    };
    let t = Instant::now();
    let a = explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
        .map_err(|e| e.to_string())?;
    let wall = t.elapsed().as_secs_f64();
    drop(events);
    let run = Run {
        saturations: a.saturations,
        facts: a.facts,
        wall,
        route: a.route,
        exhausted: a.exhausted,
        verdict: a.verdict.map(|v| v.word()),
        stopped: a.stopped.clone(),
        states: a.states.iter().map(|s| s.set.clone()).collect(),
    };
    let kept = std::mem::take(&mut *kept.lock().expect("the run is over"));
    Ok((run, kept.nodes, kept.unmapped))
}

/// **What each alive node denies**: for a commitment, the candidates whose
/// negation its fixpoint stores. Kept across a program's cells, since the
/// questions meet many of the same nodes.
#[derive(Default)]
pub struct Denials {
    of: FxHashMap<Box<[u32]>, Bits>,
    /// Fixpoints computed for it.
    pub saturations: u64,
}

impl Denials {
    /// Read the alive nodes of `nodes` not read yet, of the program at
    /// `path` loaded again. An error where the program's universe is not
    /// `r`'s.
    pub fn read(&mut self, path: &Path, r: &Read, nodes: &[Node]) -> Result<(), String> {
        let missing: Vec<&Node> = nodes
            .iter()
            .filter(|x| x.kind != Kind::Dead && !self.of.contains_key(&x.set))
            .collect();
        if missing.is_empty() {
            return Ok(());
        }
        let (mut kb, mut terms, ast) = loaded(path).ok_or("does not load")?;
        // Root alone: the universe, as every route builds it.
        let root = explore::Options {
            no_search: true,
            cores: Cores::READ_OUT,
            ..explore::Options::new(Mode {
                which: Which::All,
                ask: Ask::Describe,
            })
        };
        let a = explore::run(&mut kb, &mut terms, &ast, &root).map_err(|e| e.to_string())?;
        if a.candidates.len() != r.n || a.candidates.iter().zip(&r.texts).any(|(c, t)| c.text != *t)
        {
            return Err("another universe".to_string());
        }
        let facts: Vec<ein_core::FactId> = a.candidates.iter().map(|c| c.fact).collect();
        let mut reader =
            Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).map_err(|e| e.to_string())?;
        for x in missing {
            let set: Vec<ein_core::FactId> = x.set.iter().map(|&b| facts[b as usize]).collect();
            let denied = reader
                .with_fixpoint(&set, |kb, terms| {
                    let mut d = Bits::new(r.n);
                    for (b, &f) in facts.iter().enumerate() {
                        if terms
                            .probe_fact(terms.kernel.not, &[ein_core::Value::fact(f)])
                            .is_some_and(|n| kb.contains(n))
                        {
                            d.insert(b as u32);
                        }
                    }
                    d
                })
                .map_err(|e| e.to_string())?;
            self.saturations += 1;
            self.of.insert(x.set.clone(), denied);
        }
        Ok(())
    }

    fn get(&self, set: &[u32]) -> Option<&Bits> {
        self.of.get(set)
    }
}

// ── The split and the ways ──────────────────────────────────────────────

pub const PARTS: [&str; 8] = [
    "in_r",
    "owing_owed_group",
    "owing_other",
    "dead_theory",
    "dead_puzzle",
    "dead_single",
    "dead_edge",
    "dead_other",
];

pub const WAYS: [&str; 14] = [
    "layer-whole",
    "layer-by-class",
    "singles",
    "denials",
    "layer-by-need",
    "death-pairs",
    "death-core-pair",
    "death-core",
    "theory",
    "theory-shared",
    "cover",
    "wipe-out",
    "wipe-out-layer",
    "factor",
];

/// The objects a way pays for.
pub const SINGLES: &str = "root's singles";
pub const PAIRS: &str = "root's pairs";
pub const SMALL: &str = "the small KB's layer";
pub const NODE: &str = "the coupling's node";

/// What one way of spending would have done to a cell: `[saturations,
/// facts]` added, by object, and removed.
#[derive(Clone, Debug, Default)]
pub struct Delta {
    /// The way was applied: the program is monotone and the objects it
    /// reads were read.
    pub applies: bool,
    /// The count is what the same route would have done with its order
    /// held; `false` where it is an estimate, which `note` explains.
    pub exact: bool,
    pub added: [u64; 2],
    pub by: BTreeMap<&'static str, [u64; 2]>,
    pub removed: [u64; 2],
    /// Counts the way keeps beside: pairs asked, cores read, and so on.
    pub counts: BTreeMap<&'static str, u64>,
    pub note: Option<&'static str>,
    /// Facts are not known for it.
    pub no_facts: bool,
    /// A cost the way has is not counted: what it removes is a bound on
    /// what it saves.
    pub bound: bool,
}

impl Delta {
    fn charge(&mut self, object: &'static str, sat: u64, facts: u64) {
        self.added[0] += sat;
        self.added[1] += facts;
        let e = self.by.entry(object).or_default();
        e[0] += sat;
        e[1] += facts;
    }

    fn refund(&mut self, object: &'static str, sat: u64, facts: u64) {
        self.added[0] -= sat;
        self.added[1] -= facts;
        let e = self.by.entry(object).or_default();
        e[0] -= sat;
        e[1] -= facts;
    }

    fn remove(&mut self, x: &Node) {
        self.removed[0] += 1;
        self.removed[1] += x.facts;
    }

    fn count(&mut self, what: &'static str, by: u64) {
        *self.counts.entry(what).or_default() += by;
    }

    /// The cell's work with the way, in unit `u`: 0 saturations, 1 facts.
    pub fn with(&self, base: u64, u: usize) -> u64 {
        (base + self.added[u]).saturating_sub(self.removed[u])
    }
}

/// What a best order could not go under, read off the leaves a `--single`
/// or a `-n 1` found.
#[derive(Clone, Debug)]
pub struct Ceiling {
    /// States of `R` the run lists.
    pub leaves: usize,
    /// Root's node and one node a candidate of the leaves' commitments:
    /// the fewest nodes of any tree that reaches them.
    pub floor: u64,
    /// The run decided its question.
    pub decided: bool,
}

/// One cell, priced.
#[derive(Clone, Debug, Default)]
pub struct Priced {
    /// `[saturations, facts]` a part, in [`PARTS`]' order.
    pub split: [[u64; 2]; 8],
    /// Counts beside the split.
    pub detail: BTreeMap<&'static str, u64>,
    pub ways: Vec<(&'static str, Delta)>,
    pub ceiling: Option<Ceiling>,
    /// Each node's part, in visit order.
    pub parts: Vec<u8>,
}

impl Priced {
    pub fn part(&self, name: &str) -> [u64; 2] {
        self.split[PARTS.iter().position(|p| *p == name).expect("a part")]
    }
}

/// `set` lies inside one edge of `among` that a conflict or a derivation
/// gave: an edge holds a question of every member.
fn edge_over(c: &Coupling, among: &[Edge], set: &[u32]) -> bool {
    among.iter().any(|e| {
        e.is_tie()
            && set.iter().all(|&b| {
                c.question_of[b as usize]
                    .iter()
                    .any(|q| e.questions.binary_search(q).is_ok())
            })
    })
}

/// What a way knows dead before any node, and which of the layer's nodes it
/// has computed.
struct Known<'a> {
    singles: &'a Bits,
    pairs: Vec<&'a PairSet>,
    /// The layer's nodes it holds: 2 all, 1 root's node and the singles, 0
    /// none, 3 those of `held`.
    have: u8,
    held: Option<&'a FxHashSet<Box<[u32]>>>,
    /// It reads what root owes.
    wipe: bool,
}

impl Known<'_> {
    fn dead(&self, list: &[u32], k: &Bits) -> bool {
        k.meets(self.singles) || self.pairs.iter().any(|p| p.any_in(list, k))
    }

    fn holds(&self, set: &[u32]) -> bool {
        match self.have {
            2 => true,
            1 => set.len() <= 1,
            3 => self.held.is_some_and(|h| h.contains(set)),
            _ => false,
        }
    }
}

/// One cell's nodes, read.
struct Cx<'a> {
    r: &'a Read,
    nodes: &'a [Node],
    ks: &'a [Bits],
    lists: &'a [Vec<u32>],
    visited: &'a FxHashSet<Box<[u32]>>,
}

impl Cx<'_> {
    /// A way that knows `known` before the first node: what it adds, the
    /// layer's nodes it holds that the route did not visit and the small
    /// KB's layer where `small_paid`; and what it removes.
    fn eager(&self, known: &Known<'_>, small_paid: bool, wipe_pairs: Option<&PairSet>) -> Delta {
        let r = self.r;
        let mut d = Delta {
            applies: true,
            exact: true,
            ..Delta::default()
        };
        // determinism-ok: sums over the layer's nodes.
        for (set, x) in &r.layer {
            if known.holds(set) && !self.visited.contains(set) {
                d.charge(if set.len() >= 2 { PAIRS } else { SINGLES }, 1, x.facts);
            }
        }
        if small_paid {
            let small = r.small_work();
            d.charge(SMALL, small[0], small[1]);
        }
        for (i, x) in self.nodes.iter().enumerate() {
            let set = less(&x.set, &r.at_root);
            if known.holds(&set) && r.layer.contains_key(&set) {
                continue;
            }
            let (k, list) = (&self.ks[i], &self.lists[i]);
            if x.kind == Kind::Dead && known.dead(list, k) {
                d.remove(x);
                d.count("known dead", 1);
                continue;
            }
            if known.wipe
                && let Some(pairs) = wipe_pairs
            {
                // The candidates out above this node: dead alone, or
                // excluded with a member of K.
                let mut gone = known.singles.clone();
                for &b in list {
                    gone.or(pairs.row(b));
                }
                let wiped = r
                    .licensed
                    .iter()
                    .any(|(alts, g)| g.is_some() && !alts.meets(k) && alts.within(&gone));
                if wiped {
                    d.remove(x);
                    d.count("an owed group left no member", 1);
                    if x.kind != Kind::Dead {
                        d.count("of them alive", 1);
                    }
                }
            }
        }
        d
    }
}

/// Price one cell: `nodes` are what `run` visited, in its order.
pub fn price(r: &Read, key: &str, run: &Run, nodes: &[Node], denials: Option<&Denials>) -> Priced {
    let n = r.n;
    let mut out = Priced::default();
    let monotone = r.monotone();
    let tree = matches!(key, "single" | "n1" | "minimal" | "maximal");

    // K, and each alive node's closure.
    let empty = Bits::new(n);
    let mut closures: Vec<Bits> = Vec::with_capacity(nodes.len());
    let mut ks: Vec<Bits> = Vec::with_capacity(nodes.len());
    // What the alive nodes visited before a node and inside it deny.
    let mut denied_under: Vec<Bits> = Vec::with_capacity(nodes.len());
    let mut under = Trie::new();
    for (i, x) in nodes.iter().enumerate() {
        let mut k = Bits::of(n, &x.set);
        let mut denied = Bits::new(n);
        under.subsets(&x.set, &mut |j| {
            k.or(&closures[j as usize]);
            if let Some(d) = denials.and_then(|d| d.get(&nodes[j as usize].set)) {
                denied.or(d);
            }
            false
        });
        ks.push(k);
        denied_under.push(denied);
        if x.kind == Kind::Dead {
            closures.push(empty.clone());
        } else {
            closures.push(Bits::of(n, &x.closure));
            under.insert(&x.set, i as u32);
        }
    }
    let lists: Vec<Vec<u32>> = ks.iter().map(Bits::list).collect();

    // The split.
    let one_step: &[Edge] = r
        .coupling
        .as_ref()
        .and_then(|c| c.one_step.as_deref())
        .unwrap_or(&[]);
    let mut owes_licensed = vec![false; nodes.len()];
    for (i, x) in nodes.iter().enumerate() {
        let (k, list) = (&ks[i], &lists[i]);
        let part = match x.kind {
            Kind::InR => 0,
            Kind::Owing => {
                let cl = &closures[i];
                let mut group = false;
                let mut owed = 0u32;
                for (alts, g) in &r.licensed {
                    if !alts.meets(cl) {
                        owed += 1;
                        group |= g.is_some();
                    }
                }
                owes_licensed[i] = owed > 0;
                if owed == 0 {
                    *out.detail.entry("owing_no_licensed_instance").or_default() += 1;
                }
                if owed > x.owes {
                    *out.detail
                        .entry("owed_read_off_closure_exceeds")
                        .or_default() += 1;
                }
                if group { 1 } else { 2 }
            }
            Kind::Dead => {
                *out.detail.entry("dead").or_default() += 1;
                if x.core_read {
                    *out.detail.entry("cores_read").or_default() += 1;
                }
                if x.core_read && x.core.is_none() {
                    *out.detail.entry("cores_naming_no_candidate").or_default() += 1;
                }
                if let Some(c) = &x.core {
                    if c.len() < x.set.len() {
                        *out.detail
                            .entry("core_smaller_than_commitment")
                            .or_default() += 1;
                    }
                    if c.len() <= 2 {
                        *out.detail.entry("core_of_two_or_fewer").or_default() += 1;
                    }
                }
                if r.theory_pairs.any_in(list, k) {
                    3
                } else if r.puzzle_pairs.any_in(list, k) {
                    4
                } else if k.meets(&r.dead_alone) {
                    5
                } else {
                    *out.detail.entry("dead_otherwise").or_default() += 1;
                    let mut inside = false;
                    if let Some(c) = &r.coupling {
                        let by_core = x.core.as_deref();
                        let node_core = by_core.map(|s| edge_over(c, &c.edges, s));
                        let step_core = by_core.map(|s| edge_over(c, one_step, s));
                        let node_set = edge_over(c, &c.edges, &x.set);
                        let step_set = edge_over(c, one_step, &x.set);
                        let mut tick = |name: &'static str, yes: bool| {
                            if yes {
                                *out.detail.entry(name).or_default() += 1;
                            }
                        };
                        tick("core_in_one_edge_node", node_core == Some(true));
                        tick("core_in_one_edge_one_step", step_core == Some(true));
                        tick("commitment_in_one_edge_node", node_set);
                        tick("commitment_in_one_edge_one_step", step_set);
                        inside = node_core.unwrap_or(node_set);
                        tick("in_one_edge_one_step", step_core.unwrap_or(step_set));
                    }
                    if inside { 6 } else { 7 }
                }
            }
        };
        if x.kind != Kind::Dead && denied_under[i].meets(k) {
            // An alive node over a candidate a node under it denies: the
            // licence the way rests on does not hold.
            *out.detail.entry("alive_and_denied_under").or_default() += 1;
        }
        if x.kind == Kind::Dead && denied_under[i].meets(k) {
            *out.detail.entry("dead_and_denied_under").or_default() += 1;
            let name = match part {
                3 => "denied_under_of_dead_theory",
                4 => "denied_under_of_dead_puzzle",
                5 => "denied_under_of_dead_single",
                6 => "denied_under_of_dead_edge",
                _ => "denied_under_of_dead_other",
            };
            *out.detail.entry(name).or_default() += 1;
        }
        out.split[part][0] += 1;
        out.split[part][1] += x.facts;
        out.parts.push(part as u8);
    }

    // The nodes a tree built by branching on every extension, at a state
    // that owes and owes no licensed instance.
    if tree && monotone {
        let mut by_closure: FxHashMap<&[u32], usize> = FxHashMap::default();
        let mut every = 0u64;
        for (i, x) in nodes.iter().enumerate() {
            let parent = (0..x.set.len()).find_map(|drop| {
                let mut p: Vec<u32> = x.set.to_vec();
                p.remove(drop);
                by_closure.get(&p[..]).copied()
            });
            if let Some(p) = parent
                && nodes[p].kind == Kind::Owing
                && !owes_licensed[p]
            {
                every += 1;
            }
            if x.kind != Kind::Dead {
                by_closure.entry(&x.closure[..]).or_insert(i);
            }
        }
        out.detail
            .insert("above_a_state_owing_no_licensed_instance", every);
    }

    // The ceiling.
    if matches!(key, "single" | "n1") {
        let mut all: FxHashSet<u32> = FxHashSet::default();
        for s in &run.states {
            all.extend(s.iter().copied().filter(|&b| !r.at_root.has(b)));
        }
        out.ceiling = Some(Ceiling {
            leaves: run.states.len(),
            floor: 1 + all.len() as u64,
            decided: run.stopped.is_none(),
        });
    }

    // The ways.
    let visited: FxHashSet<Box<[u32]>> = nodes.iter().map(|x| less(&x.set, &r.at_root)).collect();
    let have_small = r.theory.small.is_ok() && r.cover.is_some();
    let all_root: Vec<&PairSet> = vec![&r.root_pairs];
    let cx = Cx {
        r,
        nodes,
        ks: &ks,
        lists: &lists,
        visited: &visited,
    };
    let eager = |known: Known<'_>, small_paid: bool, wipe_pairs: Option<&PairSet>| {
        cx.eager(&known, small_paid, wipe_pairs)
    };

    let off = Delta::default;
    let mut ways: Vec<(&'static str, Delta)> = Vec::new();
    let layer_read = r.layer_same && r.theory.root.complete();
    // The pair layer whole.
    ways.push((
        "layer-whole",
        if monotone && layer_read {
            eager(
                Known {
                    singles: &r.dead_alone,
                    pairs: all_root.clone(),
                    have: 2,
                    held: None,
                    wipe: false,
                },
                false,
                None,
            )
        } else {
            off()
        },
    ));
    // The same layer with L3 read over the classes: two candidates with
    // one closure have one state, so of the pairs between two classes one
    // node is read, a visited one where the route visits any.
    let by_class: FxHashSet<Box<[u32]>> = {
        let mut class: Vec<u32> = (0..n as u32).collect();
        for c in &r.theory.root.equivalent {
            for &b in c {
                class[b as usize] = c[0];
            }
        }
        // determinism-ok: sorted on the next line.
        let mut keys: Vec<&Box<[u32]>> = r.layer.keys().collect();
        keys.sort();
        let mut held: FxHashSet<Box<[u32]>> = FxHashSet::default();
        let mut of: BTreeMap<(u32, u32), &Box<[u32]>> = BTreeMap::new();
        for k in keys {
            if k.len() != 2 {
                held.insert(k.clone());
                continue;
            }
            let (x, y) = (class[k[0] as usize], class[k[1] as usize]);
            let at = of.entry((x.min(y), x.max(y))).or_insert(k);
            if !visited.contains(*at) && visited.contains(k) {
                *at = k;
            }
        }
        held.extend(of.into_values().cloned());
        held
    };
    ways.push((
        "layer-by-class",
        if monotone && layer_read {
            eager(
                Known {
                    singles: &r.dead_alone,
                    pairs: all_root.clone(),
                    have: 3,
                    held: Some(&by_class),
                    wipe: false,
                },
                false,
                None,
            )
        } else {
            off()
        },
    ));
    ways.push((
        "singles",
        if monotone && layer_read {
            eager(
                Known {
                    singles: &r.dead_alone,
                    pairs: Vec::new(),
                    have: 1,
                    held: None,
                    wipe: false,
                },
                false,
                None,
            )
        } else {
            off()
        },
    ));
    ways.push((
        "denials",
        if monotone && denials.is_some() {
            let mut d = Delta {
                applies: true,
                exact: true,
                ..Delta::default()
            };
            for (i, x) in nodes.iter().enumerate() {
                if x.kind == Kind::Dead && denied_under[i].meets(&ks[i]) {
                    d.remove(x);
                }
            }
            d.count(
                "alive nodes that deny a candidate",
                nodes
                    .iter()
                    .filter(|x| {
                        x.kind != Kind::Dead
                            && denials
                                .and_then(|d| d.get(&x.set))
                                .is_some_and(|d| d.list().iter().any(|&b| !r.dead_alone.has(b)))
                    })
                    .count() as u64,
            );
            d
        } else {
            off()
        },
    ));
    ways.push((
        "layer-by-need",
        if monotone && layer_read {
            lazy(r, nodes, &ks, &lists, false)
        } else {
            off()
        },
    ));
    ways.push((
        "death-pairs",
        if monotone && layer_read {
            lazy(r, nodes, &ks, &lists, true)
        } else {
            off()
        },
    ));
    for (name, most) in [("death-core-pair", 2usize), ("death-core", usize::MAX)] {
        ways.push((
            name,
            if monotone {
                cores(nodes, &lists, most)
            } else {
                off()
            },
        ));
    }
    for (name, paid) in [("theory", true), ("theory-shared", false)] {
        let mut d = if monotone && have_small {
            eager(
                Known {
                    singles: &r.killed_alone,
                    pairs: vec![&r.theory_pairs],
                    have: 0,
                    held: None,
                    wipe: false,
                },
                paid,
                None,
            )
        } else {
            off()
        };
        if !paid && d.applies {
            d.bound = true;
            d.note = Some(
                "a bound: the small KB's layer is paid once for every puzzle of one ontology, \
                 and none of it is counted here",
            );
        }
        ways.push((name, d));
    }
    ways.push((
        "cover",
        if monotone && have_small && layer_read {
            eager(
                Known {
                    singles: &r.dead_alone,
                    pairs: vec![&r.theory_pairs],
                    have: 1,
                    held: None,
                    wipe: false,
                },
                true,
                None,
            )
        } else {
            off()
        },
    ));
    let wipes = matches!(key, "single" | "n1" | "maximal");
    for (name, have) in [("wipe-out", 1u8), ("wipe-out-layer", 2u8)] {
        let mut both = r.theory_pairs.clone();
        if have == 2 {
            for (a, row) in (0..n as u32).map(|a| (a, r.root_pairs.row(a))) {
                for b in row.list() {
                    both.insert(a, b);
                }
            }
        }
        let mut d = if monotone && have_small && layer_read && r.exact.is_some() {
            eager(
                Known {
                    singles: &r.dead_alone,
                    pairs: vec![&both],
                    have,
                    held: None,
                    wipe: wipes,
                },
                true,
                Some(&both),
            )
        } else {
            off()
        };
        if d.applies && !wipes {
            d.note = Some(
                "no node is skipped for an owed group here: the question reads the alive \
                 states that owe",
            );
        }
        ways.push((name, d));
    }
    ways.push((
        "factor",
        if monotone && have_small && layer_read && r.coupling.is_some() {
            factor(r, nodes, tree)
        } else {
            off()
        },
    ));
    out.ways = ways;
    out
}

/// The pair layer asked node by node: before each node of two candidates or
/// more (`at_death` false), or only after a node of three or more has died.
fn lazy(r: &Read, nodes: &[Node], ks: &[Bits], lists: &[Vec<u32>], at_death: bool) -> Delta {
    let n = r.n;
    let mut d = Delta {
        applies: true,
        exact: true,
        ..Delta::default()
    };
    // The layer's nodes computed by asking, which the route has not visited
    // yet; and every node the run holds.
    let mut asked: FxHashSet<Box<[u32]>> = FxHashSet::default();
    let mut have: FxHashSet<Box<[u32]>> = FxHashSet::default();
    let mut alive_known = PairSet::new(n);
    let mut dead_single = Bits::new(n);
    let mut dead_pair = PairSet::new(n);

    struct Asker<'a> {
        r: &'a Read,
        asked: &'a mut FxHashSet<Box<[u32]>>,
        have: &'a mut FxHashSet<Box<[u32]>>,
        alive_known: &'a mut PairSet,
        dead_single: &'a mut Bits,
        dead_pair: &'a mut PairSet,
    }
    impl<'a> Asker<'a> {
        /// The layer's node of `set`, computed now where the run does not
        /// hold it.
        fn node(&mut self, d: &mut Delta, set: &[u32]) -> Option<&'a LayerNode> {
            let r: &'a Read = self.r;
            let x = r.layer.get(set)?;
            if self.have.insert(set.into()) {
                self.asked.insert(set.into());
                d.charge(if set.len() >= 2 { PAIRS } else { SINGLES }, 1, x.facts);
                d.count("layer nodes asked", 1);
            }
            Some(x)
        }

        /// Ask the pairs of `list` the run does not know alive, ascending,
        /// singles first, up to the first dead one.
        fn pairs(&mut self, d: &mut Delta, list: &[u32], skip: Option<(u32, u32)>) -> bool {
            for (i, &a) in list.iter().enumerate() {
                for &b in &list[i + 1..] {
                    if self.alive_known.has(a, b) || skip == Some((a, b)) {
                        continue;
                    }
                    let mut implied = false;
                    for (s, other) in [(a, b), (b, a)] {
                        if self.dead_single.has(s) {
                            continue;
                        }
                        match self.node(d, &[s]) {
                            Some(x) if x.dead => self.dead_single.insert(s),
                            Some(x) => implied |= x.closure.binary_search(&other).is_ok(),
                            None => {}
                        }
                    }
                    if self.dead_single.has(a) || self.dead_single.has(b) {
                        return true;
                    }
                    if implied {
                        self.alive_known.insert(a, b);
                        continue;
                    }
                    match self.node(d, &[a, b]) {
                        Some(x) if x.dead => {
                            self.dead_pair.insert(a, b);
                            return true;
                        }
                        _ => self.alive_known.insert(a, b),
                    }
                }
            }
            false
        }
    }

    for (i, x) in nodes.iter().enumerate() {
        let set = less(&x.set, &r.at_root);
        let (k, full) = (&ks[i], &lists[i]);
        let list: Vec<u32> = full
            .iter()
            .copied()
            .filter(|&b| !r.at_root.has(b))
            .collect();
        let mark = |alive_known: &mut PairSet| {
            if x.kind != Kind::Dead {
                for (i, &a) in x.closure.iter().enumerate() {
                    for &b in &x.closure[i + 1..] {
                        alive_known.insert(a, b);
                    }
                }
            }
        };
        if asked.remove(&set) {
            // The route's own node, computed earlier by asking: counted
            // once, as the route's.
            let cost = r.layer[&set].facts;
            d.refund(if set.len() >= 2 { PAIRS } else { SINGLES }, 1, cost);
            mark(&mut alive_known);
            continue;
        }
        let mut dead = k.meets(&dead_single) || dead_pair.any_in(&list, k);
        let mut ask = Asker {
            r,
            asked: &mut asked,
            have: &mut have,
            alive_known: &mut alive_known,
            dead_single: &mut dead_single,
            dead_pair: &mut dead_pair,
        };
        if !dead && !at_death && set.len() >= 2 {
            let skip = (set.len() == 2).then(|| (set[0], set[1]));
            dead = ask.pairs(&mut d, &list, skip);
        }
        if dead {
            if x.kind == Kind::Dead {
                d.remove(x);
            } else {
                // A pair dead under an alive node: the licences do not hold.
                d.count("alive over a dead pair", 1);
            }
            continue;
        }
        ask.have.insert(set.clone());
        if x.kind == Kind::Dead {
            match set.len() {
                1 => ask.dead_single.insert(set[0]),
                2 => ask.dead_pair.insert(set[0], set[1]),
                _ if at_death => {
                    d.count("deaths asked about", 1);
                    if ask.pairs(&mut d, &list, None) {
                        d.count("deaths with a dead pair found", 1);
                    }
                }
                _ => {}
            }
        }
        mark(&mut alive_known);
    }
    d
}

/// The core of each death learned, where it is of at most `most`
/// candidates: a later dead node whose `K` holds one is not saturated.
fn cores(nodes: &[Node], lists: &[Vec<u32>], most: usize) -> Delta {
    let mut d = Delta {
        applies: true,
        exact: true,
        ..Delta::default()
    };
    let mut learned = Trie::new();
    let mut seen: FxHashSet<&[u32]> = FxHashSet::default();
    for (i, x) in nodes.iter().enumerate() {
        if x.kind != Kind::Dead {
            continue;
        }
        if learned.any_subset(&lists[i]) {
            d.remove(x);
            continue;
        }
        d.count("cores read", 1);
        if let Some(c) = &x.core
            && c.len() <= most
            && seen.insert(&c[..])
        {
            learned.insert(c, i as u32);
            d.count("cores learned", 1);
        }
    }
    d
}

/// The components solved apart: the nodes a run would visit are the
/// distinct projections of the visited nodes onto the components.
fn factor(r: &Read, nodes: &[Node], tree: bool) -> Delta {
    let c = r.coupling.as_ref().expect("asked with a coupling");
    let mut d = Delta {
        applies: true,
        exact: true,
        ..Delta::default()
    };
    let mut of: Vec<Option<u32>> = vec![None; r.n];
    for (k, members) in c.components.iter().enumerate() {
        for &b in members {
            of[b as usize] = Some(k as u32);
        }
    }
    d.count("components", c.components.len() as u64);
    // Each visited node's facts, by its commitment.
    let facts: FxHashMap<Box<[u32]>, u64> = nodes
        .iter()
        .map(|x| (less(&x.set, &r.at_root), x.facts))
        .collect();
    let mut kept: FxHashSet<Box<[u32]>> = FxHashSet::default();
    kept.insert(Box::default());
    let apart = c.components.len() >= 2;
    for x in nodes {
        let set = less(&x.set, &r.at_root);
        if !apart {
            // One component: the run is the route's own.
            kept.insert(set);
            continue;
        }
        // A candidate in no component is a part of its own.
        let mut parts: BTreeMap<(u8, u32), Vec<u32>> = BTreeMap::new();
        for &b in set.iter() {
            let key = match of[b as usize] {
                Some(k) => (0, k),
                None => (1, b),
            };
            parts.entry(key).or_default().push(b);
        }
        for (_, p) in parts {
            kept.insert(p.into_boxed_slice());
        }
    }
    let mut kept_facts = 0u64;
    let mut unvisited = 0u64;
    // determinism-ok: sums.
    for set in &kept {
        match facts.get(set) {
            Some(f) => kept_facts += f,
            None => unvisited += 1,
        }
    }
    d.count("nodes kept", kept.len() as u64);
    d.count("projections that are no visited node", unvisited);
    let total: [u64; 2] = [
        nodes.len() as u64,
        nodes.iter().map(|x| x.facts).sum::<u64>(),
    ];
    d.removed[0] = total[0].saturating_sub(kept.len() as u64);
    if unvisited == 0 {
        d.removed[1] = total[1].saturating_sub(kept_facts);
    } else {
        d.no_facts = true;
    }
    if kept.len() as u64 > total[0] {
        d.charge("projections", kept.len() as u64 - total[0], 0);
    }
    if unvisited > 0 || (tree && apart) {
        d.exact = false;
        d.note = Some(
            "an estimate: a tree's order over one component need not be its order under \
             another's state, and a projection that is no visited node has no facts read",
        );
    }
    // The coupling as built reads both pair layers and the node.
    // determinism-ok: sums over the layer's nodes.
    for (set, x) in &r.layer {
        if !kept.contains(set) {
            d.charge(if set.len() >= 2 { PAIRS } else { SINGLES }, 1, x.facts);
        }
    }
    let small = r.small_work();
    d.charge(SMALL, small[0], small[1]);
    let node = r.node_work();
    d.charge(NODE, node[0], node[1]);
    d
}

/// The six cells of one program, each asked plain and then watched.
pub struct Cell {
    pub key: &'static str,
    pub flags: &'static str,
    /// The plain run, or why there is none.
    pub run: Result<Run, String>,
    /// The watched run's wall, cores taken at every death.
    pub wall_watched: f64,
    /// The watched run answered as the plain one and did its work, and the
    /// nodes handed to the watch are its saturations and its facts.
    pub same_work: bool,
    pub unmapped: u64,
    pub nodes: Vec<Node>,
    pub priced: Priced,
}

pub fn cell(
    path: &Path,
    r: &Read,
    key: &'static str,
    flags: &'static str,
    mode: Mode,
    denials: &mut Denials,
) -> Cell {
    let plain = ask(path, key, mode, None);
    let mut c = Cell {
        key,
        flags,
        run: plain.map(|(run, _, _)| run),
        wall_watched: 0.0,
        same_work: false,
        unmapped: 0,
        nodes: Vec::new(),
        priced: Priced::default(),
    };
    let Ok(run) = &c.run else {
        return c;
    };
    if let Ok((watched, nodes, unmapped)) = ask(path, key, mode, Some(r)) {
        c.wall_watched = watched.wall;
        c.same_work = watched.same(run)
            && nodes.len() as u64 == run.saturations
            && nodes.iter().map(|x| x.facts).sum::<u64>() == run.facts;
        c.unmapped = unmapped;
        // What the alive nodes deny, on a monotone program, where a dead
        // set says something of its supersets.
        let read = r.monotone() && denials.read(path, r, &nodes).is_ok();
        c.priced = price(r, key, run, &nodes, read.then_some(&*denials));
        c.nodes = nodes;
    }
    c
}
