//! A program's region, as sets of closures — shared by `walk_vs_region` and
//! `walk_census`, which put the walk's recorded states on it.
//!
//! R is known when route 1 describes it under [`BUDGET`] visits, or, for a
//! program the static check flags, when the oracle enumerates it at
//! `n ≤ region::DEFAULT_LIMIT`.

use std::collections::BTreeSet;
use std::path::Path;

use ein_core::{Kb, Terms};
use ein_infer::downset;
use ein_infer::region::{self, Outcome};
use ein_ir::{Ast, load, parse};
use rustc_hash::FxHashMap;

/// Route 1's budget here: enough for `zebra.ein` (270 912 visits).
pub const BUDGET: u64 = 1 << 20;

/// A commitment or a closure: candidate indices, ascending.
pub type State = Vec<u32>;

pub fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// R and its neighbours, as sets of closures.
#[derive(Default)]
pub struct Known {
    pub route: u8,
    pub monotone: bool,
    pub candidates: Vec<String>,
    pub k_c: u64,
    pub alive: u64,
    /// Every state of R.
    pub r: BTreeSet<State>,
    /// The states of R's maximal line.
    pub max: BTreeSet<State>,
    pub min: BTreeSet<State>,
    /// Alive, owing, and no alive commitment one fact up: what the walk can
    /// record as an open state.
    pub open_max: BTreeSet<State>,
    pub alive_states: u64,
    pub visited: u64,
    /// The status of every commitment the route decided; any other is dead by
    /// the theorem (route 1) or does not occur (route 2 holds them all).
    pub status: FxHashMap<State, u8>,
    /// Route 2 with `keep_states` only: each state of R and of its maximal
    /// line as its **positive facts beyond root's**, by text — the oracle's
    /// states, for a program whose closure may not determine its state.
    pub r_facts: Option<BTreeSet<Vec<String>>>,
    pub max_facts: Option<BTreeSet<Vec<String>>>,
    /// Root's fixpoint, by text — what a recorded state is taken beyond.
    pub root_facts: Option<BTreeSet<String>>,
}

/// The positive facts of `facts` that root's fixpoint does not hold.
pub fn beyond(facts: &[String], root: &BTreeSet<String>) -> Vec<String> {
    let mut v: Vec<String> = facts
        .iter()
        .filter(|f| !f.starts_with("(not ") && !root.contains(*f))
        .cloned()
        .collect();
    v.sort();
    v.dedup();
    v
}

pub fn known(path: &Path, jobs: usize, keep_states: bool) -> Option<Known> {
    let (mut kb, mut terms, ast) = loaded(path)?;
    let d = downset::compute(
        &mut kb,
        &mut terms,
        &ast,
        &downset::Options {
            budget: u64::MAX,
            max_visits: BUDGET,
            jobs,
        },
    )
    .ok()?;
    let candidates: Vec<String> = d.candidates.iter().map(|c| c.text.clone()).collect();
    if let Some(desc) = &d.description {
        let mut k = Known {
            route: 1,
            monotone: true,
            candidates,
            k_c: desc.k_c,
            visited: d.points.len() as u64,
            ..Known::default()
        };
        k.status = d
            .points
            .iter()
            .map(|p| (p.set.to_vec(), p.status))
            .collect();
        let index: FxHashMap<&[u32], usize> = d
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (&p.set[..], i))
            .collect();
        let alive = |s: &[u32]| {
            index
                .get(s)
                .is_some_and(|&i| d.points[i].status & region::DEAD == 0)
        };
        let mut alive_states: BTreeSet<State> = BTreeSet::new();
        for p in &d.points {
            if p.status & region::DEAD != 0 {
                continue;
            }
            k.alive += 1;
            alive_states.insert(p.closure.to_vec());
            if p.in_r() {
                k.r.insert(p.closure.to_vec());
            } else {
                let up = (0..k.candidates.len() as u32)
                    .filter(|b| p.set.binary_search(b).is_err())
                    .any(|b| {
                        let mut s = p.set.to_vec();
                        let at = s.partition_point(|&x| x < b);
                        s.insert(at, b);
                        alive(&s)
                    });
                if !up {
                    k.open_max.insert(p.closure.to_vec());
                }
            }
        }
        k.alive_states = alive_states.len() as u64;
        for &i in &desc.maximal {
            k.max.insert(d.points[i as usize].closure.to_vec());
        }
        for &i in &desc.minimal {
            k.min.insert(d.points[i as usize].closure.to_vec());
        }
        return Some(k);
    }
    if d.n() > region::DEFAULT_LIMIT as usize || d.monotonicity.is_monotone() {
        return None;
    }
    let (mut kb, mut terms, ast) = loaded(path)?;
    let opts = region::Options {
        jobs,
        keep_states,
        ..region::Options::default()
    };
    let Ok(Outcome::Region(r)) = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {})
    else {
        return None;
    };
    let n = r.n();
    let bits = |m: u64| -> State { (0..n as u32).filter(|b| m >> b & 1 == 1).collect() };
    let mut k = Known {
        route: 2,
        monotone: false,
        candidates,
        k_c: r.topology.size_r,
        visited: 1 << n,
        ..Known::default()
    };
    let mut alive_states: BTreeSet<State> = BTreeSet::new();
    for m in 0..(1u64 << n) {
        let st = r.status[m as usize];
        k.status.insert(bits(m), st);
        if st & region::DEAD != 0 {
            continue;
        }
        k.alive += 1;
        let closure = bits(r.nodes[m as usize].closure);
        alive_states.insert(closure.clone());
        if st == 0 {
            k.r.insert(closure);
        } else {
            let up = (0..n)
                .any(|b| m >> b & 1 == 0 && r.status[(m | 1 << b) as usize] & region::DEAD == 0);
            if !up {
                k.open_max.insert(closure);
            }
        }
    }
    k.alive_states = alive_states.len() as u64;
    for &m in &r.topology.maximal {
        k.max.insert(bits(r.nodes[m as usize].closure));
    }
    for &m in &r.topology.minimal {
        k.min.insert(bits(r.nodes[m as usize].closure));
    }
    if let Some(states) = &r.states {
        // `region::compute` leaves root at its fixpoint.
        let root: BTreeSet<String> = kb
            .facts()
            .map(|f| ein_infer::events::sexpr(&terms, f))
            .collect();
        let pos = |m: u64| beyond(&states[m as usize], &root);
        k.r_facts = Some((0..1u64 << n).filter(|&m| r.in_r(m)).map(pos).collect());
        k.max_facts = Some(r.topology.maximal.iter().map(|&m| pos(m)).collect());
        k.root_facts = Some(root);
    }
    Some(k)
}

/// The candidates a recorded KB holds, by text.
pub fn closure_of(kb: &Kb, terms: &Terms, index: &FxHashMap<&str, u32>) -> State {
    let mut v: Vec<u32> = kb
        .facts()
        .filter_map(|f| {
            index
                .get(ein_infer::events::sexpr(terms, f).as_str())
                .copied()
        })
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}
