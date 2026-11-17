//! What no node can hold: **licence L9**, a witness that nothing proposes
//! and nothing writes.
//!
//! L5 (`crate::tree`, § L5) branches an owing state on the alternatives of
//! one instance it owes, and `crate::oblgen::alternatives` licenses the
//! instance when every fact that can witness it is a candidate or has its
//! negative stored at root: the second kind is in no alive state, so the
//! candidates are all that can pay. A third kind of witness is in **no
//! state at all**: a fact that is no candidate, that root's fixpoint does not
//! hold, and whose relation no rule of the program writes and the kernel
//! does not mirror. This module says which relations those are
//! (`Unwritten`), and L9 counts such a witness with the denied ones: the
//! instance is licensed, its alternatives the candidates among its witnesses
//! ([`crate::oblgen::Instance::unheld`]).
//!
//! ### L9, the argument
//!
//! Under [`standard-of-proof.md`](../../../../docs/ein/standard-of-proof.md)'s
//! rule 2. Let the program be monotone (the static check, [`crate::monotone`]),
//! let root owe the instance `I`, let every read of `I`'s guard beside its
//! witness step be static (L5's premise, unchanged), and let `w` be a fact
//! that can witness `I`, no candidate, not denied at root, of a relation `R`
//! that `Unwritten` calls unwritten. Then no node's fixpoint holds `w`:
//!
//! 1. *A node is root's fixpoint, its commitment, and what saturation adds.*
//!    The node function forks root's fixpoint, writes the commitment's
//!    candidates and resumes root's saturation ([`crate::region`], § What a
//!    node is). A fact the saturation adds is the conclusion of a plan's
//!    firing, whose relation is the plan's own (a conclusion is a fact with
//!    a relation the plan names, `crate::firing::build_fact`), or the
//!    kernel's mirror of a fact of a relation some fact
//!    `(__symmetric__ R)` marks. Nothing else in a saturation writes a fact.
//! 2. *`w` is in neither of the first two.* Root owes `I`, and every other
//!    read of its guard is static, so no witness of `I` is in root's
//!    fixpoint; and `w` is no candidate, so no commitment writes it.
//! 3. *No plan a node runs writes `R`.* On a monotone program no rule's
//!    activator relation is one a commitment can change (an activator site
//!    flags the program), so the plans a node runs are plans root compiled,
//!    and `Unwritten::of` reads every one of them but an hrule's, which
//!    proposes and runs at no node: none writes `R`.
//! 4. *The kernel mirrors no fact of `R`.* Root's fixpoint holds no
//!    `(__symmetric__ R)`, and no plan writes `__symmetric__`, so no node
//!    holds one root does not.
//!
//! So `w` is in no fixpoint, alive or dead. In L5's argument a member `D`
//! of `R` above a state owing `I` holds a witness of `I` in its fixpoint;
//! that witness is not `w`, is not denied, and so is a candidate: one of the
//! alternatives, as L5 needs. ∎
//!
//! **It is L5 with one more kind of witness left out**, and named apart
//! because its premise is another: L5's rests on the negations root stores,
//! which a contradiction enforces in every node; L9's on what the program's
//! plans and the kernel can write, which the check below enforces.
//!
//! **An instance with no candidate among its witnesses is not L9's.** Its
//! alternatives would be none, and the branch would end where it owes; no
//! program of the corpus has such an instance and a candidate, so the
//! licence is taken only where it has an alternative to branch on, and an
//! instance with none stays unlicensed (L5's reason).
//!
//! ### The premise, enforced
//!
//! - **At load**: `Unwritten::of` reads the plans root compiled and root's
//!   marks, the same plans the static check reads; an instance with a
//!   witness of a relation some plan writes is not L9's, and keeps L5's
//!   reason. Every kernel relation is called written, whoever writes it.
//! - **On every node a route computes with the table**: the node function
//!   reports a fact of its fixpoint beyond root's, no candidate, of a
//!   relation L9 relied on (`crate::region::NodeOut::unheld`, by
//!   `breach`), and the route refuses the run; so does the whole node the
//!   tree's order reads, where it reads one. The premise failing is an
//!   engine defect, never an answer. The obligation tree's own check of
//!   L5's equivalence, *owed exactly when no alternative is in the closure*,
//!   holds of an instance L9 licenses as of any other (`crate::tree`).
//! - **Against the oracle**: on every monotone cube the oracle enumerates
//!   with an instance L9 licenses, no node holds such a fact, and every
//!   member of `R` above a state owing the instance holds one of its
//!   alternatives (`ein-infer/tests/unheld.rs`); and
//!   `ein-infer/tests/explore.rs` holds every mode to the oracle.
//! - **With and without**: every question on every corpus program and
//!   regression witness with an instance L9 licenses answers the same with it
//!   and without it (`ein-infer/tests/unheld.rs`).
//!
//! ### What it does not license
//!
//! A witness that is in the closure of a candidate, and so is no candidate
//! itself, does not stand for that candidate: a rule can derive it from two
//! candidates neither of which derives it alone, and a member of `R` then
//! pays the debt with no candidate whose own closure holds the witness
//! (`examples/lattice/13_witness_of_two.ein`). Its relation is written, so
//! L9 does not take it, and the instance stays unlicensed.
//!
//! ### Where only a route reads it
//!
//! A route's table takes L9's alternatives
//! ([`crate::oblgen::Alternatives::with_l9`]); describe's route reads no
//! table, and a reader of the structure reads L5's table as it is. The
//! read-out's `route` row names L9 where a branch took an instance it
//! licenses.
//!
//! ### The switch
//!
//! [`without`] makes every route's table opened on the calling thread while
//! it runs L5's alone: for the test that holds the two to one answer, and
//! for measuring what L9 saves. [`trusting`] calls every relation unwritten,
//! for the test that makes a node break the premise and shows the run
//! refuses. Neither is a lever: no `(config …)` field, no option of `ein` and
//! no `EIN_*` name reaches them.

use std::cell::Cell;

use ein_core::{FactId, Kb, Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::compile::SharedMemo;

thread_local! {
    static WITHOUT: Cell<bool> = const { Cell::new(false) };
    static TRUSTING: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with L9 left out of every route's table opened on this thread:
/// L5's alone (the module's § The switch). Restored when `f` returns or
/// unwinds.
pub fn without<T>(f: impl FnOnce() -> T) -> T {
    with_flag(&WITHOUT, f)
}

/// Whether [`without`] is running on this thread.
pub fn is_without() -> bool {
    WITHOUT.with(Cell::get)
}

/// Run `f` with every relation called unwritten, whatever the program's
/// plans write: **the premise broken on purpose**, so that a test can show
/// a node that holds such a witness refusing the run (the module's § The
/// switch). Restored when `f` returns or unwinds.
pub fn trusting<T>(f: impl FnOnce() -> T) -> T {
    with_flag(&TRUSTING, f)
}

fn with_flag<T>(flag: &'static std::thread::LocalKey<Cell<bool>>, f: impl FnOnce() -> T) -> T {
    struct Restore(&'static std::thread::LocalKey<Cell<bool>>, bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            self.0.with(|s| s.set(self.1));
        }
    }
    let _restore = Restore(flag, flag.with(|s| s.replace(true)));
    f()
}

/// **The relations a node can hold a fact of that neither root nor its
/// commitment holds**: those a plan root compiled writes, an hrule's apart,
/// those the kernel mirrors, and every kernel relation. Every other relation
/// is *unwritten*: a fact of it that root does not hold and that is no
/// candidate is in no node's fixpoint, on a monotone program (the module's
/// § L9).
#[derive(Clone, Debug, Default)]
pub(crate) struct Unwritten {
    written: FxHashSet<Symbol>,
    /// Some plan writes `__symmetric__`, so a node may mirror a relation
    /// root does not mark: every relation is written.
    every: bool,
}

impl Unwritten {
    /// From root's fixpoint `kb`, on which the static check ran, and `memo`,
    /// which holds every plan root compiled.
    pub(crate) fn of(kb: &Kb, terms: &Terms, memo: &SharedMemo) -> Unwritten {
        let program = kb.program();
        let k = &terms.kernel;
        // determinism-ok: a set asked for membership alone.
        let mut written: FxHashSet<Symbol> =
            [k.not, k.r#false, k.open, k.closed, k.symmetric, k.written]
                .into_iter()
                .collect();
        let mut every = false;
        for plan in crate::monotone::plans_of(memo) {
            // An hrule proposes candidates and runs at no node.
            if program.hrules.contains(plan.rule) && !program.rules.contains(plan.rule) {
                continue;
            }
            for w in crate::monotone::writes_of(&plan, terms) {
                // A stored negation is a fact of `not`, which is written
                // already; the relation it denies is not.
                if w.negated {
                    continue;
                }
                every |= w.name == k.symmetric;
                written.insert(w.name);
            }
        }
        for f in kb.facts_of(k.symmetric) {
            if let Some(r) = terms.facts.args(f).first().and_then(|v| v.as_sym()) {
                written.insert(r);
            }
        }
        Unwritten { written, every }
    }

    /// No node holds a fact of `rel` that root does not hold and that is no
    /// candidate.
    pub(crate) fn is_unwritten(&self, rel: Symbol) -> bool {
        TRUSTING.with(Cell::get) || (!self.every && !self.written.contains(&rel))
    }
}

/// **L9's premise, read at one node**: a fact of `fixpoint` that `root`'s
/// fixpoint does not hold, that is no candidate, and whose relation is one
/// of `relations`, rendered; the first by text. `None` where there is none,
/// which is every node while the premise holds.
pub(crate) fn breach(
    fixpoint: &Kb,
    root: &Kb,
    terms: &Terms,
    relations: &[Symbol],
    is_candidate: impl Fn(FactId) -> bool,
) -> Option<String> {
    relations
        .iter()
        .flat_map(|&rel| fixpoint.facts_of(rel))
        .filter(|&f| !root.contains(f) && !is_candidate(f))
        .map(|f| crate::events::sexpr(terms, f))
        .min()
}

/// The run's refusal where [`breach`] found `fact`.
pub(crate) fn refusal(fact: &str) -> crate::error::SolveError {
    crate::error::SolveError::Unsupported(format!(
        "L9: a node holds `{fact}`, which no candidate proposes, root does not hold and the \
         program's plans do not write — the licence's premise failed; run `ein oracle` on this \
         program"
    ))
}
