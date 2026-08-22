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
        SendProbe::<$t>(PhantomData).answer()
    };
}

macro_rules! is_sync {
    ($t:ty) => {
        SyncProbe::<$t>(PhantomData).answer()
    };
}

struct SendProbe<T>(PhantomData<T>);
struct SyncProbe<T>(PhantomData<T>);

trait Fallback {
    fn answer(&self) -> bool {
        false
    }
}

impl<T> Fallback for SendProbe<T> {}
impl<T> Fallback for SyncProbe<T> {}

impl<T: Send> SendProbe<T> {
    fn answer(&self) -> bool {
        true
    }
}

impl<T: Sync> SyncProbe<T> {
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

/// **The finding, and its fix.** `events::Buffer` was `Rc<RefCell<Vec<u8>>>`
/// when [T1a.7.1.4](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md#task-t1a714--kbcore--program-audit)
/// asked this question, so `Events` was the one thing on a worker's list that
/// could not cross a thread — which design/08 §3's "no shared queue" hid,
/// because a sink is not a queue.
///
/// The fix was not a lock. It is the shape the counters already have: a
/// per-worker buffer replayed at the **ordered commit**, so the stream a reader
/// sees is the sequential one
/// ([T1a.7.2.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md#task-t1a721--snapshot-and-fan-out)).
/// `Events::worker` is that buffer and `Events::replay` is the merge, and the
/// **ordinal is assigned at replay**: what a worker records is what happened,
/// and where it belongs in the run is the committing thread's to say.
///
/// So the assertion is inverted from what it was, and the two halves are both
/// load-bearing. `Send` is what a worker needs, because an `Events` is *moved*
/// into one. Not `Sync` is the other half and is not an omission: a sink two
/// threads could write at once is exactly the shared queue the design refuses.
#[test]
fn a_worker_can_hold_an_event_sink_and_two_cannot_share_one() {
    assert!(
        is_send!(ein_infer::events::Events),
        "`Events` stopped being `Send` — a worker cannot narrate without it"
    );
    assert!(is_send!(ein_infer::events::Buffer));
    assert!(
        !is_sync!(ein_infer::events::Events),
        "`Events` became `Sync`, so two threads could write one stream — \
         the design says a worker gets its own and the commit merges them"
    );
    // The control, and it is not decoration: a shim that answered `false` for
    // everything would pass the line above for the wrong reason.
    assert!(is_sync!(ein_core::Terms));
}

/// A worker's narration is the run's, in the run's order, with the run's
/// ordinals — the property `Events::replay` exists for.
#[test]
fn a_replayed_worker_narrates_into_the_streams_own_ordinals() {
    use ein_infer::events::{Buffer, Events, Level};

    let buf = Buffer::new();
    let mut run = Events::to(Box::new(buf.clone()), Level::Verbose);
    // The `run` event took ordinal 0.
    run.emit("a", |l| l.str("who", "committer"));

    let mut worker = run.worker();
    worker.emit("b", |l| l.str("who", "worker"));
    worker.emit("c", |l| l.str("who", "worker"));
    assert_eq!(worker.deferred(), 2, "a worker holds its lines");

    run.replay(worker);
    run.emit("d", |l| l.str("who", "committer"));

    let lines: Vec<String> = buf.to_string_lossy().lines().map(str::to_owned).collect();
    let kinds: Vec<String> = lines
        .iter()
        .map(|l| l.split('"').nth(3).expect("an \"e\" field").to_owned())
        .collect();
    assert_eq!(
        kinds,
        ["run", "a", "b", "c", "d"],
        "the worker's lines land where the commit put them, not where they ran"
    );
    for (i, line) in lines.iter().enumerate() {
        assert!(
            line.contains(&format!(r#""n": {i}"#)),
            "line {i} is not numbered {i}: {line}"
        );
    }
    // A worker whose run is not recording builds nothing at all.
    assert!(!Events::off().worker().on());
}
