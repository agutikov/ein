//! **What *a program searches* means**, and the budget and cube limit it is
//! read at: one definition.
//!
//! Shared by `#[path]`, since a test tree's functions are no crate's:
//! `corpus_ops` scopes the determinism sweeps by it per commit,
//! `ein-infer/examples/pair_layer.rs` its census's tables, and
//! `ein-infer/tests/structure_invariance.rs` its own sweep.

use ein_core::{Kb, Terms};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::Ast;

/// The sweep's budget and cube limit: small, so that every file is quick and
/// partial runs, budget- and limit-stopped, are among what is compared.
pub const SWEEP_BUDGET: u64 = 1 << 14;
pub const SWEEP_LIMIT: u32 = 10;

/// **A program searches when its describe saturates more than root**, at the
/// sweep's budget and cube limit ([`SWEEP_BUDGET`], [`SWEEP_LIMIT`]), one
/// job: the programs whose region routes have more than one node to fan out
/// and more than one fixpoint to intern into, which is where a job count and
/// an id order can reach an output past root's. The determinism sweeps run
/// over these per commit; computed from the program,
/// never a list. A program that does not load does not search.
pub fn searches(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> bool {
    let mut opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    opts.budget = SWEEP_BUDGET;
    opts.limit = SWEEP_LIMIT;
    opts.jobs = 1;
    ein_infer::read_out::run(kb, terms, ast, &opts).is_ok_and(|r| r.answer.saturations > 1)
}
