//! Route 1 — a monotone program's region, from its alive down-set. M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md).
//!
//! [`crate::region`] (route 2, *the oracle*) saturates all `2ⁿ` commitments.
//! When [`crate::monotone`] finds no site, the theorem there says `dead` is
//! upward-closed and *discharged* is upward-closed, so most of that work is
//! licensed away: a superset of a dead commitment is dead without being
//! visited. This route visits exactly what the theorem does not decide.
//!
//! ### The walk
//!
//! Layer by layer from `∅`, by the textbook prefix join over candidate
//! indices: a commitment of size `k + 1` is visited only when every one of its
//! `k`-subsets is alive, and is then a fork of root with the commitment written
//! in, saturated by saturation alone — [`crate::region`]'s own node, with no
//! lookahead, no forced positive, no kill cache, no writeback and no learned
//! clause. Root is saturated once and never written.
//!
//! What it visits is therefore the alive set plus **`N`**, the minimal dead
//! sets: a visited dead commitment has every proper subset alive. Every node
//! it does not visit is dead **by the theorem**, and the certificate of the
//! answer is the static verdict, `N`, and the minimal line (the alive members
//! of `E`). Its cost is `|alive| + |N|` saturations, against `2ⁿ`.
//!
//! ### What it checks of the theorem on the way
//!
//! The one violation a down-set walk can see is an alive, discharged
//! commitment with an alive, **owing** superset one fact up — *discharged*
//! failing to be upward-closed inside the alive set, which is also the only
//! way an alive node could be a hole. If it sees one, the static verdict is
//! contradicted and the route declines ([`Declined::Contradicted`]): the
//! answer is then the oracle's to give, or nobody's past its limit. A dead
//! node with an alive superset is invisible here by construction, which is why
//! `ein-infer/tests/region.rs` holds this route to the oracle node for node on
//! every monotone program the oracle reaches.
//!
//! ### Where it stops
//!
//! At a budget of visited commitments, checked before each layer, so a
//! declined run has visited whole layers `0 … k` and nothing past them. What
//! the route found then is exact up to size `k` and says nothing beyond it.

use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::monotone::Monotonicity;
use crate::region::{Axes, Candidate, Cost, DEAD, OWING, Prepared, fan_out, pool_of, prepare};
use crate::solve::SolveError;

/// Visited commitments past which the route declines: the oracle's cost at
/// its own limit, `2¹⁶` saturations. T1e2.1a.9.1 measured where it falls
/// (`route1_reach.md`).
pub const DEFAULT_BUDGET: u64 = 1 << 16;

/// Nodes per fan-out, as the oracle's.
const BATCH: usize = 1024;

/// How the route is asked.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// The most commitments it visits.
    pub budget: u64,
    /// Threads, with [`crate::region::Options::jobs`]' contract: any value
    /// gives the same record.
    pub jobs: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            budget: DEFAULT_BUDGET,
            jobs: 1,
        }
    }
}

/// One visited commitment.
#[derive(Clone, Debug)]
pub struct Point {
    /// The commitment, as candidate indices in ascending order.
    pub set: Box<[u32]>,
    /// [`DEAD`] / [`OWING`].
    pub status: u8,
    pub owes: u32,
    /// The owed instances, rendered through their `:why`.
    pub owed: Vec<String>,
    /// A dead point's unsat core — the reason it is in `N`.
    pub core: Vec<String>,
    pub refuted_by: Vec<String>,
    /// The first visited point with the same state, as an index into
    /// [`DownSet::points`].
    pub class: u32,
    /// The candidates its fixpoint holds — its class's top, `to`.
    pub closure: Box<[u32]>,
}

impl Point {
    pub fn in_r(&self) -> bool {
        self.status == 0
    }
}

/// What one layer held.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Layer {
    pub visited: u64,
    /// Members of `N` of this size.
    pub dead: u64,
    pub owing: u64,
    pub r: u64,
    /// Facts beyond root's, summed over the layer's fixpoints — the work, as
    /// a count that does not depend on the machine.
    pub facts: u64,
}

/// Why the route gave no description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Declined {
    /// The static check found a site: the theorem does not license the route.
    NotMonotone,
    /// Layer `layer` would have taken the visit count past the budget; it
    /// held `next` commitments.
    Budget { layer: usize, next: u64 },
    /// `from` is alive and discharged, `to` is `from` plus one fact, alive,
    /// and owes: the cube contradicts the static verdict. Both are indices
    /// into [`DownSet::points`].
    Contradicted { from: u32, to: u32 },
}

impl Declined {
    pub fn as_str(&self) -> &'static str {
        match self {
            Declined::NotMonotone => "not-monotone",
            Declined::Budget { .. } => "budget",
            Declined::Contradicted { .. } => "contradicted",
        }
    }
}

/// One connected piece of `R` under one-fact edges.
#[derive(Clone, Debug)]
pub struct Component {
    pub size: u64,
    pub minimal: Vec<u32>,
    pub maximal: Vec<u32>,
}

/// A state with more than one generating commitment: its minimal generators
/// (`from`) and the candidates its fixpoint holds (`to`). For a monotone
/// program the class is the union of `[from … to]` over the generators
/// ([Q-M1e2.7 § The third answer](../../../../plans/m1e2_correctness/open_questions.md#the-third-answer-2026-10-16)).
#[derive(Clone, Debug)]
pub struct Merged {
    /// The class's first visited point.
    pub class: u32,
    /// How many commitments of `R` have this state.
    pub size: u64,
    pub generators: Vec<u32>,
    pub to: Box<[u32]>,
}

/// `R`, described — every list an index into [`DownSet::points`], in visit
/// order.
#[derive(Clone, Debug, Default)]
pub struct Description {
    /// `|R|` in commitments.
    pub k_c: u64,
    /// `|R|` in states.
    pub k_s: u64,
    pub minimal: Vec<u32>,
    pub maximal: Vec<u32>,
    /// `N`: every visited dead point, each minimal by construction.
    pub nogoods: Vec<u32>,
    pub components: Vec<Component>,
    /// `[a … b]` with `a` on the minimal line and `b` on the maximal one —
    /// the maximal sub-cubes, since `R` is convex.
    pub subcubes: Vec<(u32, u32)>,
    pub merged: Vec<Merged>,
}

/// What the route did.
#[derive(Clone, Debug)]
pub struct DownSet {
    pub candidates: Vec<Candidate>,
    pub monotonicity: Monotonicity,
    pub root_dead: bool,
    /// Every visited commitment, in visit order: by size, then
    /// lexicographically by candidate index.
    pub points: Vec<Point>,
    pub layers: Vec<Layer>,
    /// The wall clock of each layer's saturations, beside [`Self::layers`].
    pub layer_wall: Vec<Duration>,
    /// `None` when the route described `R`.
    pub declined: Option<Declined>,
    /// Present exactly when [`Self::declined`] is `None`.
    pub description: Option<Description>,
    pub cost: Cost,
}

impl DownSet {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// `{a, b}` or `∅`, in candidate order.
    pub fn name(&self, point: u32) -> String {
        let set = &self.points[point as usize].set;
        if set.is_empty() {
            return "∅".to_string();
        }
        let parts: Vec<&str> = set
            .iter()
            .map(|&b| self.candidates[b as usize].text.as_str())
            .collect();
        format!("{{{}}}", parts.join(", "))
    }

    /// The point as an `n`-bit mask, for comparison with the oracle — `n < 64`
    /// only.
    pub fn mask(&self, point: u32) -> u64 {
        self.points[point as usize]
            .set
            .iter()
            .fold(0, |m, &b| m | 1 << b)
    }
}

/// Route 1 on `root`, a **freshly loaded, unsaturated** program nothing else
/// will use, as [`crate::region::compute`] asks.
pub fn compute(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<DownSet, SolveError> {
    let t_root = Instant::now();
    let Prepared {
        snapshot,
        root_dead,
        candidates,
        hypothesised,
        monotonicity,
        memo,
    } = prepare(root, terms, ast)?;
    let mut out = DownSet {
        candidates,
        monotonicity,
        root_dead,
        points: Vec::new(),
        layers: Vec::new(),
        layer_wall: Vec::new(),
        declined: None,
        description: None,
        cost: Cost::default(),
    };
    if !out.monotonicity.is_monotone() {
        out.declined = Some(Declined::NotMonotone);
        out.cost.root = t_root.elapsed();
        return Ok(out);
    }
    let axes = Axes::new(root, &out.candidates, hypothesised, false);
    let facts: Vec<FactId> = out.candidates.iter().map(|c| c.fact).collect();
    let root = root.sealed();
    out.cost.root = t_root.elapsed();

    let t_nodes = Instant::now();
    let pool = pool_of(opts.jobs);
    let mut index: FxHashMap<Box<[u32]>, u32> = FxHashMap::default();
    let mut classes: FxHashMap<Box<[FactId]>, u32> = FxHashMap::default();
    let mut frontier: Vec<Box<[u32]>> = vec![Box::default()];
    while !frontier.is_empty() {
        let layer = out.layers.len();
        let visited = out.points.len() as u64;
        if visited + frontier.len() as u64 > opts.budget {
            out.declined = Some(Declined::Budget {
                layer,
                next: frontier.len() as u64,
            });
            break;
        }
        let t_layer = Instant::now();
        let mut stats = Layer::default();
        let mut alive: Vec<Box<[u32]>> = Vec::new();
        for chunk in frontier.chunks(BATCH) {
            let sets: Vec<Vec<FactId>> = chunk
                .iter()
                .map(|s| s.iter().map(|&b| facts[b as usize]).collect())
                .collect();
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
            for (o, set) in outs.into_iter().zip(chunk) {
                let id = out.points.len() as u32;
                stats.facts += o.state.len() as u64;
                let class = *classes.entry(o.state).or_insert(id);
                let status = (if o.dead { DEAD } else { 0 }) | (if o.owes > 0 { OWING } else { 0 });
                stats.visited += 1;
                if o.dead {
                    stats.dead += 1;
                } else {
                    alive.push(set.clone());
                    if o.owes > 0 {
                        stats.owing += 1;
                    } else {
                        stats.r += 1;
                    }
                }
                index.insert(set.clone(), id);
                out.points.push(Point {
                    set: set.clone(),
                    status,
                    owes: o.owes,
                    owed: o.node.owed,
                    core: o.node.core,
                    refuted_by: o.node.refuted_by,
                    class,
                    closure: o.closure.into_boxed_slice(),
                });
            }
        }
        out.layers.push(stats);
        out.layer_wall.push(t_layer.elapsed());
        frontier = if layer == 0 {
            if alive.is_empty() {
                Vec::new()
            } else {
                (0..out.n() as u32).map(|b| Box::from([b])).collect()
            }
        } else {
            join(&alive)
        };
    }
    out.cost.nodes = t_nodes.elapsed();

    let t_topo = Instant::now();
    if let Some((from, to)) = contradiction(&out.points, &index) {
        out.declined = Some(Declined::Contradicted { from, to });
    }
    if out.declined.is_none() {
        out.description = Some(describe(out.n(), &out.points, &index));
    }
    out.cost.topology = t_topo.elapsed();
    Ok(out)
}

/// The next layer: every `(k+1)`-set whose `k`-subsets are all in `alive`,
/// which is sorted lexicographically and holds sets of one size `k ≥ 1`.
/// The output is sorted the same way.
fn join(alive: &[Box<[u32]>]) -> Vec<Box<[u32]>> {
    let is_alive: FxHashSet<&[u32]> = alive.iter().map(|s| &s[..]).collect();
    let mut out = Vec::new();
    let mut probe: Vec<u32> = Vec::new();
    for (i, a) in alive.iter().enumerate() {
        let k = a.len();
        for b in &alive[i + 1..] {
            // Sorted, so once the prefix differs no later set shares it.
            if a[..k - 1] != b[..k - 1] {
                break;
            }
            let mut c: Vec<u32> = Vec::with_capacity(k + 1);
            c.extend_from_slice(a);
            c.push(b[k - 1]);
            // The two subsets that dropped `a`'s or `b`'s last element are `b`
            // and `a`; the rest drop one prefix element each.
            let all = (0..k - 1).all(|drop| {
                probe.clear();
                probe.extend(
                    c.iter()
                        .enumerate()
                        .filter(|&(j, _)| j != drop)
                        .map(|(_, &x)| x),
                );
                is_alive.contains(&probe[..])
            });
            if all {
                out.push(c.into_boxed_slice());
            }
        }
    }
    out
}

/// An alive, discharged point with an alive owing point one fact above it,
/// the first in visit order — `None` when the walk saw none.
fn contradiction(points: &[Point], index: &FxHashMap<Box<[u32]>, u32>) -> Option<(u32, u32)> {
    let mut probe: Vec<u32> = Vec::new();
    for (q, p) in points.iter().enumerate() {
        if p.status != OWING {
            continue;
        }
        for drop in 0..p.set.len() {
            without(&p.set, drop, &mut probe);
            if let Some(&s) = index.get(&probe[..])
                && points[s as usize].status == 0
            {
                return Some((s, q as u32));
            }
        }
    }
    None
}

fn without(set: &[u32], drop: usize, into: &mut Vec<u32>) {
    into.clear();
    into.extend(
        set.iter()
            .enumerate()
            .filter(|&(j, _)| j != drop)
            .map(|(_, &x)| x),
    );
}

fn is_subset(a: &[u32], b: &[u32]) -> bool {
    let mut j = 0;
    for &x in a {
        while j < b.len() && b[j] < x {
            j += 1;
        }
        if j == b.len() || b[j] != x {
            return false;
        }
        j += 1;
    }
    true
}

/// `R` from the visited points: every node outside them is dead by the
/// theorem, and no hole exists because [`contradiction`] found none.
fn describe(n: usize, points: &[Point], index: &FxHashMap<Box<[u32]>, u32>) -> Description {
    let in_r = |set: &[u32]| index.get(set).is_some_and(|&i| points[i as usize].in_r());
    let r: Vec<u32> = (0..points.len() as u32)
        .filter(|&i| points[i as usize].in_r())
        .collect();
    let mut d = Description {
        k_c: r.len() as u64,
        ..Description::default()
    };
    let mut probe: Vec<u32> = Vec::new();
    for &i in &r {
        let set = &points[i as usize].set;
        if (0..set.len()).all(|drop| {
            without(set, drop, &mut probe);
            !in_r(&probe)
        }) {
            d.minimal.push(i);
        }
        let up = (0..n as u32)
            .filter(|b| set.binary_search(b).is_err())
            .any(|b| {
                probe.clear();
                probe.extend_from_slice(set);
                let at = probe.partition_point(|&x| x < b);
                probe.insert(at, b);
                in_r(&probe)
            });
        if !up {
            d.maximal.push(i);
        }
    }
    d.nogoods = (0..points.len() as u32)
        .filter(|&i| points[i as usize].status & DEAD != 0)
        .collect();

    // Components: union-find over the one-fact edges inside R, each found
    // from its larger end.
    let mut parent: FxHashMap<u32, u32> = r.iter().map(|&i| (i, i)).collect();
    fn find(parent: &mut FxHashMap<u32, u32>, x: u32) -> u32 {
        let mut root = x;
        while parent[&root] != root {
            root = parent[&root];
        }
        let mut y = x;
        while parent[&y] != root {
            let next = parent[&y];
            parent.insert(y, root);
            y = next;
        }
        root
    }
    for &i in &r {
        let set = &points[i as usize].set;
        for drop in 0..set.len() {
            without(set, drop, &mut probe);
            if let Some(&j) = index.get(&probe[..])
                && points[j as usize].in_r()
            {
                let (a, b) = (find(&mut parent, i), find(&mut parent, j));
                if a != b {
                    parent.insert(a.max(b), a.min(b));
                }
            }
        }
    }
    let mut ids: FxHashMap<u32, usize> = FxHashMap::default();
    for &i in &r {
        let root = find(&mut parent, i);
        let next = ids.len();
        let c = *ids.entry(root).or_insert(next);
        if c == d.components.len() {
            d.components.push(Component {
                size: 0,
                minimal: Vec::new(),
                maximal: Vec::new(),
            });
        }
        d.components[c].size += 1;
    }
    for &i in &d.minimal {
        let c = ids[&find(&mut parent, i)];
        d.components[c].minimal.push(i);
    }
    for &i in &d.maximal {
        let c = ids[&find(&mut parent, i)];
        d.components[c].maximal.push(i);
    }

    for &a in &d.minimal {
        for &b in &d.maximal {
            if is_subset(&points[a as usize].set, &points[b as usize].set) {
                d.subcubes.push((a, b));
            }
        }
    }

    // States, and the classes with several generators.
    let mut members: FxHashMap<u32, Vec<u32>> = FxHashMap::default();
    let mut order: Vec<u32> = Vec::new();
    for &i in &r {
        let class = points[i as usize].class;
        let v = members.entry(class).or_default();
        if v.is_empty() {
            order.push(class);
        }
        v.push(i);
    }
    d.k_s = order.len() as u64;
    for class in order {
        let v = &members[&class];
        if v.len() < 2 {
            continue;
        }
        let generators: Vec<u32> = v
            .iter()
            .copied()
            .filter(|&i| {
                let set = &points[i as usize].set;
                (0..set.len()).all(|drop| {
                    without(set, drop, &mut probe);
                    index
                        .get(&probe[..])
                        .is_none_or(|&j| points[j as usize].class != class)
                })
            })
            .collect();
        d.merged.push(Merged {
            class,
            size: v.len() as u64,
            generators,
            to: points[class as usize].closure.clone(),
        });
    }
    d
}
