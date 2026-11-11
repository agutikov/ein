//! **A batch of requests costs in proportion to its count** — the measured
//! half of the rule of `std.arith`, M1f
//! [S1f.6.3](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md).
//!
//! The rule of the theory is *no rule joins two facts on a number*, and
//! `arith_theory.rs` holds it on the module's text. This file holds what the
//! rule is **for**. The engine's fact index does not key on a nested fact
//! (E-3 of P1f.6's `findings.md`), so a premise looked up by a number walks
//! its relation's whole extent: with one such join in the theory, every
//! request scans every other request's facts, and a batch costs the square of
//! its count while every answer stays right. Round 2's theory, written with
//! ordinary joins, took 3.0 s for 4 000 nine-digit sums where this one takes
//! about a tenth of a second.
//!
//! ## The measure: `candidates`, not the clock
//!
//! [`ein_core::counters::Counters::candidates`] counts the facts the matcher
//! *offered to a premise and unified against*: one per fact a lookup or a
//! scan tried. It is the engine's own count of the matcher's work, it is the
//! same on every machine, and it is exactly what a scan inflates.
//!
//! **The counters are compiled out by default**, so that the shipped binary
//! carries no increment on its hottest path. This file is therefore two
//! things:
//!
//! - under `cargo test --workspace` it is built without them, and holds the
//!   weaker claim that needs none: the *firings* per request do not grow with
//!   the batch, and every batch is answered;
//! - under **`cargo test -p ein-infer --features counters --test
//!   arith_batch`**, which is a step of the gate (`run_tests.sh`,
//!   `per-commit.yml`), it holds the claim itself: `candidates` per request
//!   at 1 000 and at 4 000 requests is what it is at 250, within
//!   [`TOLERANCE`].
//!
//! ## The stated exception, measured
//!
//! `operand-name` joins an operand with a `(let NAME NUMBER)` on the name,
//! and asks `(named ?x)` first: a fact with every argument bound is one
//! interned lookup, whatever the argument is, so an operand that is a number
//! stops there. [`a_number_operand_does_not_walk_the_lets`] holds that a
//! batch of number requests costs the same beside two hundred `let`s as
//! beside none. Without `(named ?x)` the same batch cost `2 × requests ×
//! lets` candidates more, 200 000 for 500 requests beside 200 `let`s
//! (2026-11-11): each operand walked every `let`, because the index does not
//! key the nested fact it was asked with.
//!
//! A later operation joins this file by adding its request to [`REQUESTS`].

use ein_core::Terms;
use ein_core::counters;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, parse};

const IMPORT: &str = "(import std.arith :symbols (arith))\n";

/// The request relations a batch is made of.
const REQUESTS: [&str; 1] = ["add"];

/// Per-request work at a larger batch may exceed the smallest batch's by this
/// factor and no more. The operands differ from batch to batch, so the counts
/// are not identical; a join on a number multiplies them by the batch's ratio,
/// 4 and 16.
const TOLERANCE: f64 = 1.05;

/// A number of exactly `digits` digits in the theory's spelling.
fn number(rng: &mut u64, digits: u32) -> String {
    let mut out = String::from("z");
    for i in 0..digits {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let r = (*rng >> 33) % if i == 0 { 9 } else { 10 };
        let d = if i == 0 { r + 1 } else { r };
        out = format!("(n {out} {d})");
    }
    out
}

/// `count` requests of each relation over two nine-digit numbers, beside
/// `lets` named constants nobody asks about. The requests are the same
/// whatever `lets` is: the constants are drawn from a stream of their own.
fn batch(count: usize, lets: usize) -> String {
    let (mut rng, mut names) = (0x5146_0603_u64, 0x5146_0604_u64);
    let mut src = String::from(IMPORT);
    for i in 0..lets {
        src.push_str(&format!("(let c{i} {})\n", number(&mut names, 9)));
    }
    for op in REQUESTS {
        for _ in 0..count {
            let (x, y) = (number(&mut rng, 9), number(&mut rng, 9));
            src.push_str(&format!("({op} {x} {y})\n"));
        }
    }
    src
}

struct Work {
    /// Firings of the saturation, the closing rule's included.
    firings: u64,
    /// `candidates` over the saturation alone; 0 without the feature.
    candidates: u64,
    /// The answers stored.
    answers: usize,
}

fn saturate(src: &str) -> Work {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<arith-batch>")).expect("the batch parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the batch loads");
    let mut events = Events::off();
    let mut firings = 0u64;
    let candidates;
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the theory compiles");
        counters::reset();
        sat.saturate(&mut s, None, &mut |_| firings += 1)
            .expect("the batch reaches its fixpoint");
        candidates = counters::snapshot().candidates;
    }
    let answers = terms
        .syms
        .get("sum-of")
        .map_or(0, |rel| kb.facts_of(rel).count());
    Work {
        firings,
        candidates,
        answers,
    }
}

fn per_request(total: u64, count: usize) -> f64 {
    total as f64 / (count * REQUESTS.len()) as f64
}

#[test]
fn a_batch_costs_in_proportion_to_its_count() {
    let sizes = [250usize, 1000, 4000];
    let work: Vec<Work> = sizes.iter().map(|&n| saturate(&batch(n, 0))).collect();
    for (n, w) in sizes.iter().zip(&work) {
        assert_eq!(w.answers, n * REQUESTS.len(), "{n} requests, each answered");
    }
    let base_firings = per_request(work[0].firings, sizes[0]);
    let base = per_request(work[0].candidates, sizes[0]);
    // The numbers themselves, for `-- --nocapture`: the stage records quote them.
    for (n, w) in sizes.iter().zip(&work) {
        eprintln!(
            "{n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request",
            w.firings,
            per_request(w.firings, *n),
            w.candidates,
            per_request(w.candidates, *n)
        );
    }
    for (n, w) in sizes.iter().zip(&work).skip(1) {
        let firings = per_request(w.firings, *n);
        assert!(
            firings <= base_firings * TOLERANCE,
            "firings per request grew with the batch: {base_firings:.1} at {} \
             and {firings:.1} at {n}",
            sizes[0]
        );
        if cfg!(feature = "counters") {
            let c = per_request(w.candidates, *n);
            assert!(
                c <= base * TOLERANCE,
                "the matcher's work per request grew with the batch: {base:.1} \
                 candidates a request at {} and {c:.1} at {n}. A rule of \
                 std.arith joins two facts on a number: the index does not key \
                 a nested fact, so that premise walks its whole relation (E-3 \
                 of P1f.6's findings.md). `arith_theory.rs` names the rule if \
                 it is written as two premises",
                sizes[0]
            );
        }
    }
    if cfg!(feature = "counters") {
        assert!(base > 0.0, "the counters are on and counted nothing");
        // A request is 27 firings, and a column's row is found among the
        // twenty its first digit keys: the count is in the hundreds, and a
        // scan of the batch would add the batch's size to it.
        assert!(
            base < 250.0,
            "{base:.1} candidates a request at {} requests",
            sizes[0]
        );
    }
}

/// The stated exception costs a number operand one lookup: the same requests
/// beside `lets` named constants cost what they cost beside none, plus what
/// the constants cost themselves, a firing each, whatever the requests.
#[test]
fn a_number_operand_does_not_walk_the_lets() {
    if !cfg!(feature = "counters") {
        return;
    }
    let (requests, lets) = (500usize, 200usize);
    let without = saturate(&batch(requests, 0));
    let with = saturate(&batch(requests, lets));
    let alone = saturate(&batch(0, lets));
    eprintln!(
        "{requests} requests: {} candidates beside no `let`, {} beside {lets}; \
         the {lets} `let`s alone, {}",
        without.candidates, with.candidates, alone.candidates
    );
    assert_eq!(
        with.candidates,
        without.candidates + alone.candidates,
        "{requests} requests beside {lets} `let`s cost more than the requests \
         and the `let`s cost apart: a number operand is walking the `let` \
         facts. `operand-name` asks `(named ?x)` first so that it does not"
    );
}
