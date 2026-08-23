//! S1a.3.2 acceptance — the inner loop allocates nothing.
//!
//! The claim is not "matching got faster". It is that the *per-candidate* cost
//! stopped being an allocation: ein.py's `_bind_arg` returns
//! `{**bindings, name: arg}` on every successful bind, which is a fresh dict
//! per bound variable per candidate fact at every level of the join
//! ([design/05](../../../../docs/history/m1a_rust/design/05_matcher.md) §1). A
//! counting allocator is how that is checked rather than asserted: run the same
//! plan over a KB twice the size, and the allocation count must not move.

use ein_core::{Terms, Value};
use ein_ir::{Ast, from_ir::load, parse};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::ops::ControlFlow;

thread_local! {
    static ALLOCS: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

/// Counts on the *calling thread*, so the other tests `cargo test` runs in
/// parallel cannot perturb the measurement.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note();
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note();
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note();
        unsafe { System.realloc(ptr, layout, new_size) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn note() {
    let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// A chain `(edge n-1 n)` of `n` edges, and a self-joining rule over it.
fn chain(n: usize) -> String {
    let mut s = String::from("(relation edge A B)\n(relation path A B)\n");
    s.push_str(
        "(rule walk ()\n  :match (and (edge ?a ?b) (edge ?b ?c))\n  :assert (path ?a ?c))\n",
    );
    for i in 0..n {
        s.push_str(&format!("(edge n{i} n{})\n", i + 1));
    }
    s
}

/// `(matches, allocations)` for one full run over a chain of `n` edges.
fn measure(n: usize) -> (usize, usize) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let text = chain(n);
    let forms = parse(&mut ast, &text, Some("<chain>")).expect("parses");
    let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
    let rule = kb.program().rules.values().next().expect("a rule").clone();
    let plan = ein_infer::compile_rule(&ast, &mut terms, &rule, None).expect("compiles");

    let mut matcher = ein_infer::Matcher::new();
    let mut warm = 0usize;
    // The first run sizes the register file, the trail and the premise slots.
    // Those are one allocation apiece and they are not the claim.
    matcher.run(&kb, &terms, &ast, &plan, &mut |_| {
        warm += 1;
        ControlFlow::Continue(())
    });

    let mut matches = 0usize;
    let before = ALLOCS.with(|c| c.get());
    matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
        // Touch what a real consumer touches: the callback must be able to
        // read bindings and premises without the matcher having built them.
        matches += m.bindings().count() + m.premises().len();
        ControlFlow::Continue(())
    });
    let allocs = ALLOCS.with(|c| c.get()) - before;
    (matches, allocs)
}

#[test]
fn the_inner_loop_does_not_allocate() {
    let (small_matches, small) = measure(64);
    let (large_matches, large) = measure(256);
    assert!(
        large_matches > small_matches * 3,
        "the larger chain must do more work: {small_matches} vs {large_matches}"
    );
    assert_eq!(
        (small, large),
        (0, 0),
        "matching allocated: {small} for {small_matches} units of work, \
         {large} for {large_matches}"
    );
}

/// The trail is the bind order, and it survives backtracking: a register bound
/// on a candidate that later fails must be unbound again, or the next
/// candidate compares against a stale value and the match set shrinks silently.
#[test]
fn backtracking_leaves_no_binding_behind() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let text = chain(8);
    let forms = parse(&mut ast, &text, Some("<chain>")).expect("parses");
    let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
    let rule = kb.program().rules.values().next().expect("a rule").clone();
    let plan = ein_infer::compile_rule(&ast, &mut terms, &rule, None).expect("compiles");
    let mut matcher = ein_infer::Matcher::new();
    let mut seen: Vec<Vec<Value>> = Vec::new();
    matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
        seen.push(m.bindings().map(|(_, v)| v).collect());
        assert_eq!(m.bindings().count(), 3, "?a ?b ?c, in bind order");
        ControlFlow::Continue(())
    });
    // Seven edges chain into six two-step walks, each with distinct endpoints.
    assert_eq!(seen.len(), 7);
    let mut sorted = seen.clone();
    sorted.sort_by_key(|v| v.iter().map(|x| x.bits()).collect::<Vec<_>>());
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        seen.len(),
        "a stale binding duplicated a match"
    );
}
