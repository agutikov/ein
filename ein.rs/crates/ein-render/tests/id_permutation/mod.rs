//! **A permuted id space**: a `Terms` into which every name and every fact
//! of a finished run has been interned again in a shuffled order, so that a
//! second run of the same program differs from the first only in which
//! integer each name and each fact got.
//!
//! Shared by `#[path]`, since a test binary's functions are no crate's:
//! `id_order_invariance.rs` holds every rendering of a corpus program to it,
//! and `ein-infer/tests/structure_invariance.rs` the structure of the
//! hypothesis set.

use ein_core::{FactId, Symbol, Tag, Terms, Value};
use ein_infer::mt19937::Mt19937;
use rustc_hash::FxHashMap;

/// A `Terms` whose id space is a permutation of `after`'s.
///
/// Every name `after` holds is interned again in shuffled order, and every
/// **fact** is re-interned in an order shuffled
/// within its nesting depth — depth-blind would be wrong, since
/// `(not (color-loc Red House-1))` cannot be interned before the fact it
/// wraps, and a uniform offset would be no permutation at all: `FactId`s that
/// all move by the same amount keep their relative order, which is exactly
/// what a leak would read. `FACT_OFFSET` junk facts go in ahead of everything
/// so that even a file with one fact has its id displaced.
///
/// The kernel names keep ids 0–17 in both runs, because `Terms::new` interns
/// them before a caller can reach the table — that is not a gap in the
/// perturbation, it is what makes it legal: `Kernel` is a struct of `Symbol`s
/// the engine compares by identity, and an engine that read them out of the
/// table by number would be broken in a way no shuffle needs to prove.
pub fn permuted(after: &Terms, seed: i64) -> Terms {
    let mut rng = Mt19937::seeded(seed);
    let mut names: Vec<String> = (0..after.syms.len())
        .map(|i| after.syms.text(Symbol(i as u32)).to_string())
        .collect();
    rng.shuffle(&mut names);

    let mut terms = Terms::new();
    let junk = terms.intern_text("@id-order-probe").expect("room");
    for k in 0..FACT_OFFSET {
        let arg = terms.value_text(&format!("@probe-{k}")).expect("room");
        terms.intern_fact(junk, &[arg]).expect("room");
    }
    for name in &names {
        terms.intern_text(name).expect("room");
    }

    // Facts, by nesting depth so a wrapper never precedes what it wraps, and
    // shuffled inside each depth so the order within one is not the base run's.
    let n = after.facts.len();
    let mut depth = vec![0usize; n];
    for i in 0..n {
        let (_, args) = after.facts.get(FactId(i as u32));
        depth[i] = args
            .iter()
            .filter_map(|v| v.as_fact())
            .map(|f| depth[f.0 as usize] + 1)
            .max()
            .unwrap_or(0);
    }
    let mut by_depth: Vec<Vec<u32>> =
        vec![Vec::new(); depth.iter().copied().max().unwrap_or(0) + 1];
    for (i, d) in depth.iter().enumerate() {
        by_depth[*d].push(i as u32);
    }
    let mut moved_to: FxHashMap<u32, FactId> = FxHashMap::default();
    for bucket in &mut by_depth {
        rng.shuffle(bucket);
        for &i in bucket.iter() {
            let (rel, args) = after.facts.get(FactId(i));
            let rel_text = after.syms.text(rel).to_string();
            let args: Vec<Value> = args
                .iter()
                .map(|&v| remap(after, &mut terms, &moved_to, v))
                .collect();
            let rel = terms
                .intern_text(&rel_text)
                .expect("the relation name is already interned");
            let id = terms.intern_fact(rel, &args).expect("room");
            moved_to.insert(i, id);
        }
    }
    terms
}

/// One of `after`'s values, as the same value in `terms`.
///
/// By *text* for the two atomic shapes and by the depth-ordered map for the
/// nested one, so the proposition is the same proposition and only its id
/// moved. `UNBOUND` is not a value in a stored fact, and reaching this with
/// one would mean the store held a register sentinel.
fn remap(after: &Terms, terms: &mut Terms, moved: &FxHashMap<u32, FactId>, v: Value) -> Value {
    match v.tag() {
        Tag::Sym => {
            let text = after.syms.text(Symbol(v.payload())).to_string();
            terms.value_text(&text).expect("room")
        }
        Tag::Fact => Value::fact(moved[&v.payload()]),
    }
}

/// How far the fact space is displaced *before* the permutation. Small on
/// purpose: the point is that `FactId` 0 is not special, not that the store
/// scales.
const FACT_OFFSET: usize = 7;

/// How many ids the shuffle actually moved, and how many it *could* have —
/// the perturbation's own size, asserted so a run cannot pass because nothing
/// was permuted.
///
/// The kernel names are excluded from both halves: `Terms::new` interns them
/// before a caller can reach the table, so they sit at 0–17 in every run and
/// counting them as "unmoved" would understate every permutation by eighteen.
pub fn moved(after: &Terms, permuted: &Terms) -> (usize, usize) {
    let kernel = Terms::new().syms.len();
    let moved = (kernel..after.syms.len())
        .filter(|&i| {
            let name = after.syms.text(Symbol(i as u32));
            permuted.syms.get(name) != Some(Symbol(i as u32))
        })
        .count();
    (moved, after.syms.len() - kernel)
}
