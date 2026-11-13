//! `R`, described: what every surface says about the region, built once —
//! M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! second step, for the read-out
//! [`read_out.md`](../../../../docs/kernel/inference/read_out.md) defines.
//!
//! A [`Described`] is read off route 1's down-set ([`of_downset`]) or off the
//! cube ([`of_cube`]), and nothing else computes one: the read-out, the
//! summary, the dump and the region claim's check all read the same value, so
//! no two of them can count `R` differently.
//!
//! **What it holds is what the run established, and no more.** On an
//! exhausted run every state of `R` is in [`Described::states`] and the
//! shape is known. On a partial run (route 1 stopped by its budget, the cube
//! past `--region-limit`) the states are the members met, each line flag is
//! set only where it is **certified**, and the shape is `None`: components,
//! sub-cubes and merged states need all of `R`
//! ([`read_out.md` § 6.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#62-a-partial-run-c3)).

use rustc_hash::{FxHashMap, FxHashSet};

use crate::downset::{DownSet, Point};
use crate::explore::Route;
use crate::monotone::Monotonicity;
use crate::region::{Candidate, DEAD, OWING, Region};
use crate::states::{self, StateRoute};

/// Root, the commitment `∅`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Root {
    /// Root's own fixpoint does not derive `(false)`.
    pub alive: bool,
    /// The obligation instances root's fixpoint leaves undischarged.
    pub owes: u32,
    /// Those instances, rendered through their rule's `:why`.
    pub owed: Vec<String>,
    /// A dead root's unsat core.
    pub core: Vec<String>,
    /// What the core's search established (M1e2 S1e2.2.23); `None` where no
    /// core was taken.
    pub core_status: Option<crate::explain::CoreStatus>,
    /// The rules whose firing produced root's contradiction.
    pub refuted_by: Vec<String>,
    /// The `:source` of each core fact that carries one.
    pub sources: Vec<String>,
}

impl Root {
    pub fn in_r(&self) -> bool {
        self.alive && self.owes == 0
    }
}

/// One state of `R`: a fixpoint, and the commitments of `R` that reach it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// A commitment that reaches it: the first in visit order (by size, then
    /// by candidate index), which is one of its minimal ones.
    pub rep: Box<[u32]>,
    /// The candidates its fixpoint holds.
    pub closure: Box<[u32]>,
    /// The commitments of `R` that reach it — those the run met, on a partial
    /// run. As a run reports a count: [`crate::count::CAP`] reads *this many
    /// or more*.
    pub commitments: u64,
    /// Its minimal commitments, in visit order: the third answer's
    /// `(merged … (or …))`. On a partial run, those met.
    pub generators: Vec<Box<[u32]>>,
    /// On the minimal line — certified, on a partial run.
    pub minimal: bool,
    /// On the maximal line — certified, on a partial run.
    pub maximal: bool,
}

/// An alive node that owes, with no alive node one fact up: where an empty
/// `R` of the owing kind stops.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Owing {
    pub set: Box<[u32]>,
    pub owes: u32,
    pub owed: Vec<String>,
}

/// A minimal dead set: dead, with every proper subset alive — and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dead {
    pub set: Box<[u32]>,
    /// Its unsat core, where the run took one: every member, under
    /// [`crate::region::Cores::nogoods`], the walk's core or one taken after
    /// the walk against its snapshot (M1f S1f.5.12).
    pub core: Vec<String>,
    /// What the core's search established; `None` where the run took no
    /// core, which no surface that prints `N` reads.
    pub core_status: Option<crate::explain::CoreStatus>,
    /// The rules whose firing produced its contradiction; empty where no
    /// core was taken.
    pub refuted_by: Vec<String>,
}

/// A node outside `R` between two members of it — the cube's alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hole {
    pub set: Box<[u32]>,
    pub dead: bool,
    pub core: Vec<String>,
    /// What the core's search established; `None` where none was taken.
    pub core_status: Option<crate::explain::CoreStatus>,
    pub owes: u32,
}

/// What only all of `R` has.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    /// The components under one-fact edges, as their sizes in commitments.
    pub components: Vec<u64>,
    /// The maximal sub-cubes' dimensions. `None` where the cube's interval
    /// search passed its work bound.
    pub subcubes: Option<Vec<usize>>,
    /// `E`, the minimal discharging sets — the cube's alone.
    pub discharging: Option<u64>,
    /// The holes: how many, and the minimal ones. `None` on route 1, where a
    /// monotone program has none by the theorem.
    pub holes: Option<(u64, Vec<Hole>)>,
}

/// The work a run did, as counts that do not depend on the machine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    /// Fixpoints computed.
    pub saturations: u64,
    /// Facts beyond root's, summed over them.
    pub facts: u64,
    /// Route 1's completed layers.
    pub layers: Option<usize>,
}

/// `R`, as far as the run established it.
#[derive(Clone, Debug)]
pub struct Described {
    pub candidates: Vec<Candidate>,
    pub monotonicity: Monotonicity,
    pub route: Route,
    /// Every commitment is classified: [`Self::states`] is all of `R`.
    pub exhausted: bool,
    /// On a partial route-1 run: every member of `R` reached by a commitment
    /// of at most this many candidates was visited.
    pub complete_up_to: Option<usize>,
    /// On a route-1 run its budget stopped: the layer it did not take, and
    /// that layer's commitments. On a state-route run, the same for its
    /// layers, whose commitments are the extensions it did not saturate.
    pub stopped_before: Option<(usize, u64)>,
    /// On a state-route run its budget stopped: how it is partial.
    pub states_partial: Option<StatesPartial>,
    pub root: Root,
    /// Some node is alive.
    pub any_alive: bool,
    /// The states met, in canonical order: by closure size, then by the
    /// closure's candidates in the universe's order. The order depends on
    /// no route, lever or `-j`.
    pub states: Vec<State>,
    /// The commitments of `R` met: `k_c`, or its lower bound. As a run
    /// reports a count: [`crate::count::CAP`] reads *this many or more* (M1f
    /// S1f.6.25), which is not the lower bound of a partial run.
    pub k_c: u64,
    /// The minimal line as commitments, in visit order: the minimal elements
    /// of `R` — those certified, on a partial run. Its smallest is the least
    /// guess.
    pub minimal_commitments: Vec<Box<[u32]>>,
    /// `N`, the minimal dead sets — those met, on a partial run.
    pub nogoods: u64,
    /// The same sets, each with why it is dead, in visit order.
    pub dead: Vec<Dead>,
    /// Candidates dead alone, which no state can hold.
    pub dead_alone: u64,
    /// Which they are, as candidate indices — those met, on a partial run.
    pub dead_alone_set: Vec<u32>,
    /// Some node met owes: the program states an obligation that bites.
    pub owing_met: bool,
    /// Some node met is dead.
    pub dead_met: bool,
    /// Exhausted runs only.
    pub shape: Option<Shape>,
    /// When `R` is empty and some node is alive: every alive owing node with
    /// no alive node one fact up, in visit order.
    pub frontier: Vec<Owing>,
    pub work: Work,
}

impl Described {
    /// `k_s`, or its lower bound.
    pub fn k_s(&self) -> u64 {
        self.states.len() as u64
    }

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

/// Canonical order: closure size, then the closure's candidates.
fn canonical(states: &mut [State]) {
    states.sort_by(|a, b| (a.closure.len(), &a.closure).cmp(&(b.closure.len(), &b.closure)));
}

fn root_of(p: Option<&Point>, root_dead: bool) -> Root {
    match p {
        Some(p) => Root {
            alive: p.status & DEAD == 0,
            owes: p.owes,
            owed: p.owed.clone(),
            core: p.core.clone(),
            core_status: p.core_status,
            refuted_by: p.refuted_by.clone(),
            sources: p.sources.clone(),
        },
        None => Root {
            alive: !root_dead,
            ..Root::default()
        },
    }
}

/// `set ∪ {b}`, sorted, for `b ∉ set`.
fn with(set: &[u32], b: u32) -> Vec<u32> {
    let mut s = set.to_vec();
    let at = s.partition_point(|&x| x < b);
    s.insert(at, b);
    s
}

/// Route 1's down-set, described: exhausted when the route gave a
/// description, partial when its budget stopped it.
pub fn of_downset(d: &DownSet) -> Described {
    let points = &d.points;
    let n = d.n() as u32;
    let index: FxHashMap<&[u32], u32> = points
        .iter()
        .enumerate()
        .map(|(i, p)| (&p.set[..], i as u32))
        .collect();
    let desc = d.description.as_ref();
    let exhausted = desc.is_some();

    // Each class's commitments of `R` met.
    let mut size: FxHashMap<u32, u64> = FxHashMap::default();
    for p in points.iter().filter(|p| p.in_r()) {
        *size.entry(p.class).or_default() += 1;
    }
    let minimal_points: Vec<u32> = match desc {
        Some(x) => x.minimal.clone(),
        None => minimal_met(points, &index),
    };
    let minimal: FxHashSet<u32> = minimal_points
        .iter()
        .map(|&i| points[i as usize].class)
        .collect();
    let maximal: FxHashSet<u32> = match desc {
        Some(x) => x
            .maximal
            .iter()
            .map(|&i| points[i as usize].class)
            .collect(),
        None => maximal_met(d, &index),
    };
    let mut generators: FxHashMap<u32, Vec<Box<[u32]>>> = FxHashMap::default();
    match desc {
        Some(x) => {
            for m in &x.merged {
                generators.insert(
                    m.class,
                    m.generators
                        .iter()
                        .map(|&g| points[g as usize].set.clone())
                        .collect(),
                );
            }
        }
        None => {
            // A member met is a minimal commitment of its state when no
            // one-down subset reaches the same state: route 1 visits every
            // subset of a visited point, and its classes are convex.
            for (i, p) in points.iter().enumerate() {
                if !p.in_r() || p.class as usize == i {
                    continue;
                }
                let mut probe = Vec::new();
                let below = (0..p.set.len()).any(|k| {
                    crate::downset::without(&p.set, k, &mut probe);
                    index.get(&probe[..]).is_some_and(|&j| {
                        points[j as usize].in_r() && points[j as usize].class == p.class
                    })
                });
                if !below {
                    generators
                        .entry(p.class)
                        .or_insert_with(|| vec![points[p.class as usize].set.clone()])
                        .push(p.set.clone());
                }
            }
        }
    }
    let mut states: Vec<State> = points
        .iter()
        .enumerate()
        .filter(|(i, p)| p.in_r() && p.class as usize == *i)
        .map(|(_, p)| State {
            rep: p.set.clone(),
            closure: p.closure.clone(),
            commitments: size[&p.class],
            generators: generators
                .remove(&p.class)
                .unwrap_or_else(|| vec![p.set.clone()]),
            minimal: minimal.contains(&p.class),
            maximal: maximal.contains(&p.class),
        })
        .collect();
    // determinism-ok: a sum over the values, which no order reaches.
    let mut k_c: u64 = size.values().sum();
    let mut work = Work {
        saturations: points.len() as u64,
        facts: d.layers.iter().map(|l| l.facts).sum(),
        layers: Some(d.layers.len()),
    };
    // L6 on a partial run: one saturation of ⊤ certifies the trivial maximal
    // line when ⊤ is in R.
    if let Some(top) = &d.top {
        work.saturations += 1;
        work.facts += d.top_facts;
        if top.in_r() {
            match states.iter_mut().find(|s| s.closure.len() == n as usize) {
                Some(s) => s.maximal = true,
                None => {
                    k_c += 1;
                    states.push(State {
                        rep: top.set.clone(),
                        closure: top.closure.clone(),
                        commitments: 1,
                        generators: Vec::new(),
                        minimal: false,
                        maximal: true,
                    });
                }
            }
        }
    }
    canonical(&mut states);

    let shape = desc.map(|x| Shape {
        components: x.components.iter().map(|c| c.size).collect(),
        subcubes: Some(
            x.subcubes
                .iter()
                .map(|&(lo, hi)| points[hi as usize].set.len() - points[lo as usize].set.len())
                .collect(),
        ),
        discharging: None,
        holes: None,
    });
    let any_alive = points.iter().any(|p| p.status & DEAD == 0);
    let frontier = if exhausted && states.is_empty() && any_alive {
        let alive: FxHashSet<&[u32]> = points
            .iter()
            .filter(|p| p.status & DEAD == 0)
            .map(|p| &p.set[..])
            .collect();
        points
            .iter()
            .filter(|p| p.status & DEAD == 0 && p.status & OWING != 0)
            .filter(|p| {
                !(0..n)
                    .filter(|b| p.set.binary_search(b).is_err())
                    .any(|b| alive.contains(&with(&p.set, b)[..]))
            })
            .map(|p| Owing {
                set: p.set.clone(),
                owes: p.owes,
                owed: p.owed.clone(),
            })
            .collect()
    } else {
        Vec::new()
    };
    // A dead root leaves every candidate dead alone without visiting one: on
    // a monotone program by L1, and on any by L0 (a fork only adds).
    let dead_alone_set: Vec<u32> = if d.root_dead {
        (0..n).collect()
    } else {
        points
            .iter()
            .filter(|p| p.set.len() == 1 && p.status & DEAD != 0)
            .map(|p| p.set[0])
            .collect()
    };
    let dead_alone = dead_alone_set.len() as u64;
    // …and no hole, since a hole sits between two nodes of R.
    let shape = match shape {
        Some(mut sh) if d.root_dead => {
            sh.holes = Some((0, Vec::new()));
            Some(sh)
        }
        sh => sh,
    };
    Described {
        candidates: d.candidates.clone(),
        monotonicity: d.monotonicity.clone(),
        route: Route::DownSet,
        exhausted,
        complete_up_to: (!exhausted).then(|| d.layers.len().saturating_sub(1)),
        stopped_before: match d.declined {
            Some(crate::downset::Declined::Budget { layer, next, .. }) => Some((layer, next)),
            _ => None,
        },
        states_partial: None,
        root: root_of(points.first(), d.root_dead),
        any_alive,
        k_c,
        minimal_commitments: minimal_points
            .iter()
            .map(|&i| points[i as usize].set.clone())
            .collect(),
        nogoods: points.iter().filter(|p| p.status & DEAD != 0).count() as u64,
        // Route 1 visits a set only when every subset is alive, so each dead
        // point is minimal.
        dead: points
            .iter()
            .filter(|p| p.status & DEAD != 0)
            .map(|p| Dead {
                set: p.set.clone(),
                core: p.core.clone(),
                core_status: p.core_status,
                refuted_by: p.refuted_by.clone(),
            })
            .collect(),
        dead_alone,
        dead_alone_set,
        owing_met: points.iter().any(|p| p.owes > 0) || d.top.as_ref().is_some_and(|t| t.owes > 0),
        dead_met: points.iter().any(|p| p.status & DEAD != 0),
        states,
        shape,
        frontier,
        work,
    }
}

/// The points a partial route-1 run certifies minimal in `R`: a member whose
/// every one-down subset was visited and is outside `R`. Route 1 visits
/// every subset of a point it visits, and outside `R` is upward closed for
/// a monotone program, so no smaller member lies further down.
fn minimal_met(points: &[Point], index: &FxHashMap<&[u32], u32>) -> Vec<u32> {
    let mut probe = Vec::new();
    (0..points.len() as u32)
        .filter(|&i| {
            let p = &points[i as usize];
            p.in_r()
                && (0..p.set.len()).all(|k| {
                    crate::downset::without(&p.set, k, &mut probe);
                    index
                        .get(&probe[..])
                        .is_some_and(|&j| !points[j as usize].in_r())
                })
        })
        .collect()
}

/// The classes a partial route-1 run certifies maximal: a member `C` below
/// the last complete layer whose every extension by a candidate outside its
/// closure is dead — visited dead, or above a dead point and so never
/// generated. `C ∪ {x}` has the state of `cl(C) ∪ {x}`, so that is L4 for
/// the state.
fn maximal_met(d: &DownSet, index: &FxHashMap<&[u32], u32>) -> FxHashSet<u32> {
    let last = d.layers.len();
    if last < 2 {
        return FxHashSet::default();
    }
    let n = d.n() as u32;
    d.points
        .iter()
        .filter(|p| p.in_r() && p.set.len() + 1 < last)
        .filter(|p| {
            (0..n)
                .filter(|b| p.closure.binary_search(b).is_err())
                .all(|b| {
                    index
                        .get(&with(&p.set, b)[..])
                        .is_none_or(|&j| d.points[j as usize].status & DEAD != 0)
                })
        })
        .map(|p| p.class)
        .collect()
}

/// How a state-route description its budget stopped is partial — M1e2
/// S1e2.1c.2. Route 1's partial run is complete up to commitments of a size
/// ([`Described::complete_up_to`]); the state route's is complete up to
/// **states** of a size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatesPartial {
    /// Every alive state of at most this many candidates was met and
    /// extended. `None` when the route stopped before extending any.
    pub up_to: Option<usize>,
    /// The state size the route did not extend, and that layer's extensions.
    pub stopped_at: (usize, u64),
    /// `R` is decided anyway: ∅ and ⊤ are both in it, so it is the whole cube
    /// (L1, L2). `k_c`, both lines, the components, the sub-cubes and `N` are
    /// exact; `k_s` and the merged states are not.
    pub cube: bool,
    /// `N` is known: ⊤ is alive, so no node is dead.
    pub nogoods_known: bool,
}

/// The state route's run, described: exhausted when it extended every alive
/// state, partial when its budget stopped it.
pub fn of_states(s: &StateRoute, d: &states::Description) -> Described {
    let n = s.n();
    let exhausted = d.exhausted;
    // determinism-ok: `d.minimal` and `d.maximal` are vectors in canonical
    // order, collected into sets that are only looked up.
    let minimal_states: FxHashSet<u32> = d.minimal.iter().map(|(_, i)| *i).collect();
    // determinism-ok: as above.
    let maximal: FxHashSet<u32> = d.maximal.iter().copied().collect();
    let mut k_c = d.k_c;
    let mut states: Vec<State> =
        d.r.iter()
            .map(|&i| {
                let st = &s.states[i as usize];
                let generators = d.generators[i as usize].clone();
                State {
                    rep: generators
                        .first()
                        .cloned()
                        .unwrap_or_else(|| st.from.clone()),
                    closure: st.closure.clone(),
                    commitments: d.commitments[i as usize].unwrap_or(st.met),
                    generators,
                    minimal: minimal_states.contains(&i),
                    maximal: maximal.contains(&i),
                }
            })
            .collect();
    // ⊤ saturated once on a partial run, as route 1 does: in R, it certifies
    // the trivial maximal line.
    if let Some(top) = &s.top
        && !top.dead
        && top.owes == 0
    {
        let all: Box<[u32]> = (0..n as u32).collect();
        match states.iter_mut().find(|x| x.closure.len() == n) {
            Some(x) => x.maximal = true,
            None => {
                if !d.cube {
                    k_c = crate::count::capped_add(k_c, 1);
                }
                states.push(State {
                    rep: all.clone(),
                    closure: all,
                    commitments: 1,
                    generators: Vec::new(),
                    minimal: false,
                    maximal: true,
                });
            }
        }
    }
    canonical(&mut states);
    let shape = match (&d.components, &d.subcubes) {
        (Some(c), Some(cubes)) => Some(Shape {
            components: c.clone(),
            subcubes: Some(cubes.iter().map(|(a, b)| b.len() - a.len()).collect()),
            discharging: None,
            holes: s.root.dead.then(|| (0, Vec::new())),
        }),
        _ => None,
    };
    // N where the run knows it; on a partial run, the members it certified.
    let nogoods: Vec<Box<[u32]>> = d.nogoods.clone().unwrap_or_else(|| d.nogoods_met.clone());
    let dead: Vec<Dead> = nogoods
        .iter()
        .map(|set| {
            let saturated = s.dead.iter().find(|x| x.set == *set);
            Dead {
                set: set.clone(),
                core: saturated.map(|x| x.core.clone()).unwrap_or_default(),
                core_status: saturated.and_then(|x| x.core_status),
                refuted_by: saturated.map(|x| x.refuted_by.clone()).unwrap_or_default(),
            }
        })
        .collect();
    // A candidate is dead alone when root's state plus it is dead: the first
    // layer saturates every such extension, and a dead root makes every one.
    let first = s.states.first();
    let dead_alone_set: Vec<u32> = if s.root.dead {
        (0..n as u32).collect()
    } else {
        (0..n as u32)
            .filter(|&b| {
                first.is_some_and(|r| {
                    !r.bits.contains(b) && s.dead.iter().any(|x| x.set == super_with(&r.closure, b))
                })
            })
            .collect()
    };
    let frontier = d
        .frontier
        .iter()
        .map(|&i| {
            let st = &s.states[i as usize];
            Owing {
                set: st.closure.clone(),
                owes: st.owes,
                owed: st.owed.clone(),
            }
        })
        .collect();
    let partial = match &s.declined {
        Some(crate::downset::Declined::Budget { next, .. }) if !exhausted => Some(StatesPartial {
            up_to: s.complete_up_to,
            stopped_at: (s.stopped_size.unwrap_or(0), *next),
            cube: d.cube,
            nogoods_known: d.nogoods.is_some(),
        }),
        _ => None,
    };
    let any_dead = !s.dead.is_empty() || s.pruned > 0 || s.top.as_ref().is_some_and(|t| t.dead);
    Described {
        candidates: s.candidates.clone(),
        monotonicity: s.monotonicity.clone(),
        route: Route::States,
        exhausted,
        complete_up_to: None,
        stopped_before: match s.declined {
            Some(crate::downset::Declined::Budget { layer, next, .. }) => Some((layer, next)),
            _ => None,
        },
        states_partial: partial,
        root: Root {
            alive: !s.root.dead,
            owes: s.root.owes,
            owed: s.root.owed.clone(),
            core: s.root.core.clone(),
            core_status: s.root.core_status,
            refuted_by: s.root.refuted_by.clone(),
            sources: s.root.sources.clone(),
        },
        any_alive: !s.root.dead,
        k_c,
        // determinism-ok: `d.minimal` is a vector, by size and then candidates.
        minimal_commitments: d.minimal.iter().map(|(g, _)| g.clone()).collect(),
        nogoods: nogoods.len() as u64,
        dead,
        dead_alone: dead_alone_set.len() as u64,
        dead_alone_set,
        owing_met: s.states.iter().any(|x| x.owes > 0)
            || s.dead.iter().any(|x| x.owes > 0)
            || s.top.as_ref().is_some_and(|t| t.owes > 0),
        dead_met: any_dead,
        states,
        shape,
        frontier,
        work: Work {
            saturations: s.saturations(),
            facts: s.facts(),
            layers: Some(s.layers.len()),
        },
    }
}

fn super_with(set: &[u32], b: u32) -> Box<[u32]> {
    let mut v = set.to_vec();
    let at = v.partition_point(|&x| x < b);
    v.insert(at, b);
    v.into_boxed_slice()
}

/// `g ⊆ m`, as masks.
fn below(g: u64, m: u64) -> bool {
    g & m == g
}

/// The whole cube, described.
pub fn of_cube(r: &Region) -> Described {
    let n = r.n();
    let set_of = |m: u64| -> Box<[u32]> { (0..n as u32).filter(|&b| m >> b & 1 == 1).collect() };
    let t = &r.topology;
    let cube = 1u64 << n;
    // Each class's members of `R`, by size then by candidate index — the
    // order route 1 visits in.
    let mut members: Vec<u64> = (0..cube).filter(|&m| r.in_r(m)).collect();
    members.sort_by_key(|&m| (m.count_ones(), set_of(m)));
    let mut by_class: FxHashMap<u64, Vec<u64>> = FxHashMap::default();
    let mut order: Vec<u64> = Vec::new();
    for &m in &members {
        let class = r.nodes[m as usize].class;
        by_class
            .entry(class)
            .or_insert_with(|| {
                order.push(class);
                Vec::new()
            })
            .push(m);
    }
    let minimal: FxHashSet<u64> = t
        .minimal
        .iter()
        .map(|&m| r.nodes[m as usize].class)
        .collect();
    let maximal: FxHashSet<u64> = t
        .maximal
        .iter()
        .map(|&m| r.nodes[m as usize].class)
        .collect();
    let mut states: Vec<State> = order
        .iter()
        .map(|class| {
            let ms = &by_class[class];
            // Minimal members, in increasing size: a member with a smaller
            // member below it has a minimal one below it, found first.
            let mut gens: Vec<u64> = Vec::new();
            for &m in ms {
                if !gens.iter().any(|&g| below(g, m)) {
                    gens.push(m);
                }
            }
            State {
                rep: set_of(ms[0]),
                closure: set_of(r.nodes[ms[0] as usize].closure),
                commitments: ms.len() as u64,
                generators: gens.into_iter().map(set_of).collect(),
                minimal: minimal.contains(class),
                maximal: maximal.contains(class),
            }
        })
        .collect();
    canonical(&mut states);
    let alive = |m: u64| r.status[m as usize] & DEAD == 0;
    let any_alive = (0..cube).any(alive);
    let frontier = if states.is_empty() && any_alive {
        let mut v: Vec<u64> = (0..cube)
            .filter(|&m| alive(m) && r.status[m as usize] & OWING != 0)
            .filter(|&m| !(0..n).any(|b| m >> b & 1 == 0 && alive(m | 1 << b)))
            .collect();
        v.sort_by_key(|&m| (m.count_ones(), set_of(m)));
        v.into_iter()
            .map(|m| Owing {
                set: set_of(m),
                owes: r.nodes[m as usize].owes,
                owed: r.nodes[m as usize].owed.clone(),
            })
            .collect()
    } else {
        Vec::new()
    };
    let holes = t
        .minimal_holes
        .iter()
        .map(|&h| {
            let node = &r.nodes[h as usize];
            Hole {
                set: set_of(h),
                dead: r.status[h as usize] & DEAD != 0,
                core: node.core.clone(),
                core_status: node.core_status,
                owes: node.owes,
            }
        })
        .collect();
    let mut line: Vec<u64> = t.minimal.clone();
    line.sort_by_key(|&m| (m.count_ones(), set_of(m)));
    let root = &r.nodes[0];
    Described {
        candidates: r.candidates.clone(),
        monotonicity: r.monotonicity.clone(),
        route: Route::Cube,
        exhausted: true,
        complete_up_to: None,
        stopped_before: None,
        states_partial: None,
        root: Root {
            alive: !r.root_dead && alive(0),
            owes: root.owes,
            owed: root.owed.clone(),
            core: root.core.clone(),
            core_status: root.core_status,
            refuted_by: root.refuted_by.clone(),
            sources: root.sources.clone(),
        },
        any_alive,
        k_c: t.size_r,
        minimal_commitments: line.into_iter().map(set_of).collect(),
        nogoods: t.nogoods.len() as u64,
        dead: {
            let mut v: Vec<u64> = t.nogoods.clone();
            v.sort_by_key(|&m| (m.count_ones(), set_of(m)));
            v.into_iter()
                .map(|m| Dead {
                    set: set_of(m),
                    core: r.nodes[m as usize].core.clone(),
                    core_status: r.nodes[m as usize].core_status,
                    refuted_by: r.nodes[m as usize].refuted_by.clone(),
                })
                .collect()
        },
        dead_alone: (0..n).filter(|&b| !alive(1u64 << b)).count() as u64,
        dead_alone_set: (0..n as u32).filter(|&b| !alive(1u64 << b)).collect(),
        owing_met: r.status.iter().any(|s| s & OWING != 0),
        dead_met: r.status.iter().any(|s| s & DEAD != 0),
        states,
        shape: Some(Shape {
            components: t.components.iter().map(|c| c.size).collect(),
            subcubes: t.subcubes.as_ref().map(|v| {
                v.iter()
                    .map(|&(lo, hi)| (hi.count_ones() - lo.count_ones()) as usize)
                    .collect()
            }),
            discharging: Some(t.discharging.len() as u64),
            holes: Some((t.holes.len() as u64, holes)),
        }),
        frontier,
        work: Work {
            saturations: cube,
            facts: r
                .nodes
                .iter()
                .map(|x| x.beyond as u64 + x.closure.count_ones() as u64)
                .sum(),
            layers: None,
        },
    }
}
