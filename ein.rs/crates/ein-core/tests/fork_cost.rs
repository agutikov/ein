//! S1a.2.2 acceptance — a fork costs the same whether the KB holds ten facts
//! or ten thousand.
//!
//! The claim design/03 §5 makes is not "forking got faster" — ein.py's fork is
//! already 0.003 s over 206 calls. It is that a fork stops being *proportional
//! to the KB*, because [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
//! wants hundreds live at once and design/05's beta-memories are only
//! affordable if a fork does not copy them. A counting allocator is how that
//! claim is checked rather than asserted: it measures both the number of
//! allocations and the bytes, so an O(|facts|) copy that happens to be *one*
//! allocation — a bitset clone, say — cannot pass.

use ein_core::{Kb, Program, Relation, Terms, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static ALLOCS: Cell<usize> = const { Cell::new(0) };
    static BYTES: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

/// Counts on the *calling thread*, so the other tests in this binary — which
/// `cargo test` runs in parallel — cannot perturb the measurement.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note(new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn note(size: usize) {
    // `try_with` because a thread tearing down its TLS still allocates.
    let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
    let _ = BYTES.try_with(|c| c.set(c.get() + size));
}

fn counters() -> (usize, usize) {
    (ALLOCS.with(|c| c.get()), BYTES.with(|c| c.get()))
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// A KB of `n` distinct facts over one relation, indexed as saturation would
/// index them.
fn kb_with(n: u32) -> (Terms, Kb) {
    let mut terms = Terms::new();
    let mut program = Program::new();
    let rel = terms.intern_text("co-located").expect("room");
    program.add_relation(Relation {
        name: rel,
        signature: Box::new([]),
        declared: true,
        why: None,
        loc: None,
    });
    let mut kb = Kb::new(program);
    for i in 0..n {
        let args: Vec<Value> = vec![
            terms.value_text(&format!("Person-{i}")).expect("room"),
            terms.value_int(&i.to_string()).expect("room"),
        ];
        kb.add_and_index_fact(&mut terms, rel, &args, None)
            .expect("room");
    }
    (terms, kb)
}

/// `(allocations, bytes)` charged to one `fork()`.
fn one_fork(n: u32) -> (usize, usize) {
    let (_terms, mut kb) = kb_with(n);
    let before = counters();
    let child = kb.fork();
    let after = counters();
    assert_eq!(child.n_facts(), n as usize);
    (after.0 - before.0, after.1 - before.1)
}

#[test]
fn a_fork_costs_the_same_at_ten_facts_and_at_ten_thousand() {
    let small = one_fork(10);
    let large = one_fork(10_000);
    assert_eq!(
        small, large,
        "fork cost moved with the fact count: {small:?} vs {large:?}"
    );
    // Sealing the parent's top layer is one allocation and cloning the layer
    // vector is another; anything much beyond that is a copy that crept in.
    assert!(
        small.0 <= 4,
        "a fork should be a handful of allocations, not {}",
        small.0
    );
}

/// Ten nested forks, each adding a fact — `(per-fork bytes, deepest branch
/// size)`.
fn deep_fork_costs(n: u32) -> (Vec<usize>, usize) {
    let (mut terms, mut kb) = kb_with(n);
    let rel = terms.intern_text("co-located").expect("room");
    let mut branches: Vec<Kb> = Vec::new();
    let mut costs: Vec<usize> = Vec::new();
    let mut current = kb.fork();
    for i in 0..10u32 {
        let args = vec![
            terms.value_text(&format!("Branch-{i}")).expect("room"),
            terms.value_int(&i.to_string()).expect("room"),
        ];
        current
            .add_and_index_fact(&mut terms, rel, &args, None)
            .expect("room");
        let before = counters();
        let next = current.fork();
        costs.push(counters().1 - before.1);
        branches.push(std::mem::replace(&mut current, next));
    }
    for (i, branch) in branches.iter().enumerate() {
        assert_eq!(branch.n_facts(), n as usize + i + 1);
        branch.check_layering(&terms).expect("layering holds");
    }
    (costs, current.n_facts())
}

#[test]
fn a_deep_branch_pays_for_its_depth_and_not_for_the_facts_below_it() {
    // The cost of a fork does grow — by one pointer per sealed layer, because
    // the layer vector is cloned. What it must not do is grow with the KB
    // underneath it, which is the whole point of sharing the layers.
    let (small, small_facts) = deep_fork_costs(100);
    let (large, large_facts) = deep_fork_costs(10_000);
    assert_eq!(
        small, large,
        "per-fork bytes moved with the fact count: {small:?} vs {large:?}"
    );
    assert_eq!(small_facts, 110);
    assert_eq!(large_facts, 10_010);
    let growth = small.last().expect("ten forks") - small.first().expect("ten forks");
    assert!(
        growth < 1024,
        "ten levels of depth should cost pointers, not kilobytes: {small:?}"
    );
}
