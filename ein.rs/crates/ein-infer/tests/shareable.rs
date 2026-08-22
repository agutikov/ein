//! T1a.7.1.4 — **the `KbCore` / `Program` audit, as assertions.**
//!
//! [S1a.7.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md)
//! asks for a confirmation that nothing a worker holds is secretly
//! single-threaded — a lazily-computed cache behind a `Cell`, an `Rc` that
//! crept in, a `RefCell` in a registry. The compiler already knows the answer
//! to that question for every type in the port; what was missing was somebody
//! asking it, in a file that fails when the answer changes.
//!
//! `ein_core::terms` has asked it of the intern tables since
//! [S1a.2.1](../../../../plans/m1a_rust/p1a.2_kb_core/s1a.2.1_interner_and_values.md),
//! for exactly this reason. This is the same assertion widened to everything
//! [design/08 §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
//! puts on a worker — **and to the one thing that fails it**, which is the
//! audit's only finding to date and is recorded rather than fixed, because
//! fixing it is [S1a.7.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)'s
//! ordered commit and not this stage's.

use std::marker::PhantomData;

fn shareable<T: Send + Sync>() {}

/// `T: Send`, as a runtime `bool`.
///
/// Rust has no stable way to *require* the absence of an auto trait, so this
/// is the standard specialisation shim: an inherent method that exists only
/// when the bound holds, and a blanket trait method that method resolution
/// falls back to when it does not. It is the only trick in this file, and it
/// is here because the alternative — a comment saying "note: `Buffer` is not
/// `Send`" — is what the stage is trying to stop relying on.
///
/// **A macro, not a function**, and that is the whole subtlety: inside a
/// `fn is_send<T>()` the parameter is opaque, so the inherent `impl` can never
/// be shown to apply and the fallback wins for *everything*, control included.
/// The type has to be concrete where the method is resolved.
macro_rules! is_send {
    ($t:ty) => {
        Probe::<$t>(PhantomData).answer()
    };
}

struct Probe<T>(PhantomData<T>);

trait Fallback {
    fn answer(&self) -> bool {
        false
    }
}

impl<T> Fallback for Probe<T> {}

impl<T: Send> Probe<T> {
    fn answer(&self) -> bool {
        true
    }
}

/// Everything a fan-out would hand a worker, or share between workers.
#[test]
fn every_structure_a_worker_touches_is_send_and_sync() {
    // The three intern tables and the KB they number propositions for.
    shareable::<ein_core::Terms>();
    shareable::<ein_core::Kb>();
    shareable::<ein_core::Program>();
    // The AST, which a fork reads to compile a plan it has not seen.
    shareable::<ein_ir::Ast>();
    // The per-engine plan list and the memo under it — design/06 § Win A.
    shareable::<ein_infer::Engine>();
    shareable::<ein_infer::SharedMemo>();
    // The options a layer's workers all read.
    shareable::<ein_infer::solve::SolveOptions>();
    shareable::<ein_core::config::SolverConfig>();
    // What a commitment attempt returns, which crosses back on the join.
    shareable::<ein_infer::commitment::CommitmentSetResult>();
}

/// **The finding.** `events::Buffer` is `Rc<RefCell<Vec<u8>>>`, so it is not
/// `Send` — a worker cannot hold an event sink, and design/08 §3's "no shared
/// queue" hid that, because a sink is not a queue.
///
/// The fix is not a lock. It is the shape the counters already have: a
/// per-worker buffer merged at the **ordered commit**, so the stream a reader
/// sees is the sequential one
/// ([T1a.7.2.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)).
/// Until that exists this test pins the state of affairs, so the day `Buffer`
/// becomes `Send` is a day somebody notices.
#[test]
fn the_event_sink_is_the_one_thing_that_is_not() {
    assert!(
        !is_send!(ein_infer::events::Buffer),
        "`events::Buffer` became `Send` — if that was deliberate, this test \
         and S1a.7.1 T1a.7.1.4's note in the stage doc both want deleting"
    );
    // The control: the shim answers `true` for something that is.
    assert!(is_send!(ein_core::Terms));
}
