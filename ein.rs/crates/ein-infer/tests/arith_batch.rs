//! **A batch of requests costs in proportion to its count** — the measured
//! half of the rule of `std.math.decimal-arith`.
//!
//! The rule of the theory is *no rule joins two facts on a number*, and
//! `arith_theory.rs` holds it on the module's text. This file holds what the
//! rule is **for**. The engine's fact index does not key on a nested fact,
//! so a premise looked up by a number walks
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
//!   arith_batch`**, which is **run by hand** (a change to a rule of the
//!   calculator runs it), it holds the claim itself: `candidates` per request
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
//! lets` candidates more, 200 000 for 500 requests beside 200 `let`s: each
//! operand would walk every `let`, because the index does not key the nested
//! fact it is asked with.
//!
//! A later operation joins this file by adding its request to [`REQUESTS`]
//! and its bound to [`BOUND`]. Each operation is measured in batches of its
//! own.
//!
//! `std.math.num9`, `std.math.int`, `std.math.rat`,
//! `std.math.binary-arith` and `std.math.gcd` have a test of
//! their own each, below, of the same shape; binary's holds the rule of the theory where most of its rules
//! are `std.math.positional`'s, activated with binary's relations, and the
//! stated exception where its `bin-let` is.

use ein_core::Terms;
use ein_core::counters;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, parse};

const IMPORT: &str = "(import std.math.decimal-arith :symbols (decimal-arith))\n";

/// The request relations a batch is made of, each with its result relation.
/// A batch is measured per operation, so that one operation's proportion is
/// not hidden in another's count.
const REQUESTS: [(&str, &str); 5] = [
    ("add", "sum-of"),
    ("sub", "diff-of"),
    ("cmp", "cmp-of"),
    ("mul", "product-of"),
    ("div", "quot-of"),
];

/// The digits of a request's second operand. Nine, as the first has, but
/// for a division: a dividend of nine digits by a divisor of four has a
/// quotient of five or six digits, where nine by nine has one. A number's
/// top digit is not zero, so no divisor is.
fn second_digits(op: &str) -> u32 {
    if op == "div" { 4 } else { 9 }
}

/// What a request of each operation may cost at the smallest batch, in
/// `candidates`: a bound a little above the measure, so that a scan of the
/// batch, which adds the batch's size to it, fails here as well as in the
/// proportion.
const BOUND: [f64; 5] = [250.0, 500.0, 1000.0, 20000.0, 40000.0];

/// Per-request work at a larger batch may exceed the smallest batch's by this
/// factor and no more. The operands differ from batch to batch, so the counts
/// are not identical; a join on a number multiplies them by the batch's ratio,
/// 4 and 16.
const TOLERANCE: f64 = 1.05;

/// What a request may cost in guard values at the smallest batch: a little
/// above the measure, 992 for two nine-digit operands. Ten for each digit,
/// `std.math.decimal`'s `numeral-digit`'s, read once a numeral; and for each
/// operand twice the program's
/// names, some two hundred in a program of the theory and numbers alone,
/// which is `operand-undeclared` asking whether the operand is a name
/// ([`a_number_operand_does_not_walk_the_lets`] holds how that grows).
const GUARD_BOUND: f64 = 1200.0;

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

/// `count` requests of `op` over two nine-digit numbers, beside `lets` named
/// constants nobody asks about. The requests are the same whatever `lets`
/// is: the constants are drawn from a stream of their own, and each is
/// written with a high zero, `(n (n z 0) …)`, so that no numeral of a
/// constant's is one of a request's. A constant nobody asks about is not
/// checked, and `std.math.decimal` checks a numeral an operation takes once,
/// whatever takes it (Q-M1f.404), so a numeral the two shared would cost the
/// program less than the two apart.
fn batch(op: &str, count: usize, lets: usize) -> String {
    let (mut rng, mut names) = (0x5146_0603_u64, 0x5146_0604_u64);
    let mut src = String::from(IMPORT);
    for i in 0..lets {
        let value = number(&mut names, 9).replacen("(n z ", "(n (n z 0) ", 1);
        src.push_str(&format!("(let c{i} {value})\n"));
    }
    for _ in 0..count {
        let (x, y) = (number(&mut rng, 9), number(&mut rng, second_digits(op)));
        src.push_str(&format!("({op} {x} {y})\n"));
    }
    src
}

struct Work {
    /// Firings of the saturation, the closing rule's included.
    firings: u64,
    /// `candidates` over the saturation alone; 0 without the feature.
    candidates: u64,
    /// `predicate_value` over the saturation alone, the values its `eq` and
    /// `neq` premises were decided for; 0 without the feature.
    guards: u64,
    /// The answers stored.
    answers: usize,
}

fn saturate(src: &str, result: &str) -> Work {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let src = &ein_corpus::listed(src);
    let forms = parse(&mut ast, src, Some("<arith-batch>")).expect("the batch parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the batch loads");
    let mut events = Events::off();
    let mut firings = 0u64;
    let (candidates, guards);
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
        let counted = counters::snapshot();
        candidates = counted.candidates;
        guards = counted.predicate_value;
    }
    // The program writes the requests and not the result, so the result
    // prints by its full name.
    let answers = terms
        .syms
        .get(result)
        .or_else(|| {
            [
                "std.math.decimal-arith",
                "std.math.binary-arith",
                "std.math.num9",
                "std.math.int",
                "std.math.rat",
                "std.math.gcd",
            ]
            .iter()
            .find_map(|m| terms.syms.get(&format!("{m}.{result}")))
        })
        .map_or(0, |rel| kb.facts_of(rel).count());
    Work {
        firings,
        candidates,
        guards,
        answers,
    }
}

fn per_request(total: u64, count: usize) -> f64 {
    total as f64 / count as f64
}

#[test]
fn a_batch_costs_in_proportion_to_its_count() {
    let sizes = [250usize, 1000, 4000];
    let units: Vec<_> = REQUESTS.into_iter().zip(BOUND).collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&((op, result), bound)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch(op, n, 0), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            // The numbers themselves, for `-- --nocapture`.
            for (n, w) in sizes.iter().zip(&work) {
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request; \
                 {} guard values, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n),
                    w.guards,
                    per_request(w.guards, *n)
                );
            }
            let base_guards = per_request(work[0].guards, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of \
                     std.math.decimal-arith joins two facts on a number: the index does not key \
                     a nested fact, so that premise walks its whole relation. \
                     `arith_theory.rs` names the rule if \
                     it is written as two premises",
                        sizes[0]
                    );
                    // The second measure: a guard is no
                    // candidate, so a rule whose guard ranges over something
                    // that grows with the batch passes the count above.
                    let g = per_request(w.guards, *n);
                    assert!(
                        g <= base_guards * TOLERANCE,
                        "`{op}`: the guards' work per request grew with the batch: \
                     {base_guards:.1} values a request at {} and {g:.1} at {n}. A guard \
                     of std.math.decimal-arith ranges over something that grows with the requests; \
                     `candidates` does not see it",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                // A column's row is found among the rows its first digit keys:
                // the count is some tens a firing, and a scan of the batch would
                // add the batch's size to it.
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
                assert!(
                    base_guards > 0.0,
                    "the guards are counted and counted nothing"
                );
                assert!(
                    base_guards < GUARD_BOUND,
                    "`{op}`: {base_guards:.1} guard values a request at {} requests, over \
                 {GUARD_BOUND}",
                    sizes[0]
                );
            }
        },
    );
}

/// The stated exception costs a number operand one lookup: the same requests
/// beside `lets` named constants cost what they cost beside none, plus what
/// the constants cost themselves, a firing each, whatever the requests. Every
/// operation looks its operands up the same way, and each is held. A
/// program of neither costs something too, the loader stating the
/// numerals the theory's own rules write and `std.math.decimal` checking them,
/// which the two runs apart count twice: the program with no request and no
/// `let` is added to the one with both.
#[test]
fn a_number_operand_does_not_walk_the_lets() {
    if !cfg!(feature = "counters") {
        return;
    }
    let (requests, lets) = (500usize, 200usize);
    let units: Vec<_> = REQUESTS.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_number_operand_does_not_walk_the_lets",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result)| {
            let without = saturate(&batch(op, requests, 0), result);
            let with = saturate(&batch(op, requests, lets), result);
            let alone = saturate(&batch(op, 0, lets), result);
            let none = saturate(&batch(op, 0, 0), result);
            eprintln!(
                "{op}: {requests} requests: {} candidates beside no `let`, {} beside {lets}; \
             the {lets} `let`s alone, {}. Guard values: {}, {}, {}",
                without.candidates,
                with.candidates,
                alone.candidates,
                without.guards,
                with.guards,
                alone.guards
            );
            assert_eq!(
                with.candidates + none.candidates,
                without.candidates + alone.candidates,
                "{requests} `{op}` requests beside {lets} `let`s cost more than the requests \
             and the `let`s cost apart: a number operand is walking the `let` \
             facts. `operand-name` asks `(named ?x)` first so that it does not"
            );
            // **What the undeclared name costs, and it does grow with the
            // program's names**. To say
            // that an operand is a name, `operand-undeclared` ranges a variable
            // over the program's names and compares each with the operand: two
            // guard values a name, for each operand of each request, a number
            // too. A `let` adds its name to the program's, so the same requests
            // beside `lets` named constants cost exactly this much more, which
            // `candidates` does not see. It is pinned so that a second guard
            // over the names, or one inside the check's walk, a place a digit
            // (the investigation's b2), shows here as another multiple.
            assert_eq!(
                with.guards + none.guards,
                without.guards + alone.guards + (2 * 2 * requests * lets) as u64,
                "{requests} `{op}` requests beside {lets} `let`s: the guards' values are the \
             requests' and the `let`s' apart, and two for each operand and each `let`'s name"
            );
        },
    );
}

// ── std.math.num9 ───────────────────────────────────────────────────────

const IMPORT9: &str = "(import std.math.num9 :symbols (arith9))\n";

/// `std.math.num9`'s requests, each with its result relation and the digits of
/// its two operands, chosen so that every request has an answer and none
/// overflows: two numbers of eight digits have a sum of at most nine, and
/// two of four a product of at most eight.
const REQUESTS9: [(&str, &str, u32, u32); 5] = [
    ("add9", "sum9-of", 8, 8),
    ("sub9", "diff9-of", 9, 9),
    ("cmp9", "cmp9-of", 9, 9),
    ("mul9", "product9-of", 4, 4),
    ("div9", "quot9-of", 9, 4),
];

/// What a request of each may cost at the smallest batch, in `candidates`:
/// a little above the measure, as [`BOUND`] is.
const BOUND9: [f64; 5] = [BOUND9_ADD, BOUND9_SUB, BOUND9_CMP, BOUND9_MUL, BOUND9_DIV];
const BOUND9_ADD: f64 = 500.0;
const BOUND9_SUB: f64 = 500.0;
const BOUND9_CMP: f64 = 1200.0;
const BOUND9_MUL: f64 = 14000.0;
const BOUND9_DIV: f64 = 36000.0;

/// A number of exactly `digits` digits as `std.math.num9` writes it, nine digit
/// names with zeros in front.
fn number9(rng: &mut u64, digits: u32) -> String {
    let mut out = String::from("(num9");
    for _ in digits..9 {
        out.push_str(" 0");
    }
    for i in 0..digits {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let r = (*rng >> 33) % if i == 0 { 9 } else { 10 };
        let d = if i == 0 { r + 1 } else { r };
        out.push_str(&format!(" {d}"));
    }
    out.push(')');
    out
}

/// `count` distinct requests: two short operands can be drawn twice, and a
/// request stated twice is one fact with one answer.
fn batch9(op: &str, count: usize, first: u32, second: u32) -> String {
    let mut rng = 0x5146_0613_u64;
    let mut src = String::from(IMPORT9);
    let mut seen = std::collections::BTreeSet::new();
    while seen.len() < count {
        let request = format!(
            "({op} {} {})\n",
            number9(&mut rng, first),
            number9(&mut rng, second)
        );
        if seen.insert(request.clone()) {
            src.push_str(&request);
        }
    }
    src
}

/// **The second module keeps the rule of the theory, and this is what it
/// buys.** A `num9` that joined a division's state with the divisor's
/// multiples on the divisor, a nested fact, would have quadratic batches.
/// The shipped module computes the nine multiples in one
/// firing and carries them in the state, and computes a product's rows
/// inside its step rule, so no rule has a second premise that holds a
/// number (`num9_theory.rs` holds that on the text). Here: a request of each
/// operation costs the same `candidates` at 1 000 and at 4 000 requests as
/// at 250, and the same firings.
#[test]
fn a_batch_of_nine_digit_requests_costs_in_proportion_to_its_count() {
    let sizes = [250usize, 1000, 4000];
    let units: Vec<_> = REQUESTS9.into_iter().zip(BOUND9).collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_of_nine_digit_requests_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&((op, result, first, second), bound)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch9(op, n, first, second), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n)
                );
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of std.math.num9 \
                     joins two facts on a number; \
                     `num9_theory.rs` names the rule if it is written as two premises",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
            }
        },
    );
}

// ── std.math.int ────────────────────────────────────────────────────────

const IMPORT_INT: &str = "(import std.math.int :symbols (int-arith))\n";

/// `std.math.int`'s requests over two integers, each with its result relation and
/// what a request may cost at the smallest batch, in `candidates`: a little
/// above the measure, as [`BOUND`] is; and the digits of an operand's parts,
/// six, and four for a product, which is four of `std.math.decimal-arith`'s. Every third
/// operand is written with both parts, not simplified. The three divisions
/// are of four-digit parts, a divisor that is zero left out.
const REQUESTS_INT: [(&str, &str, f64, u32); 7] = [
    ("int-add", "int-sum-of", BOUND_INT_ADD, 6),
    ("int-sub", "int-diff-of", BOUND_INT_ADD, 6),
    ("int-mul", "int-product-of", BOUND_INT_MUL, 4),
    ("int-cmp", "int-cmp-of", BOUND_INT_CMP, 6),
    ("int-div-euclid", "int-quot-euclid-of", BOUND_INT_DIV, 4),
    ("int-div-trunc", "int-quot-trunc-of", BOUND_INT_DIV, 4),
    ("int-div-floor", "int-quot-floor-of", BOUND_INT_DIV, 4),
];
const BOUND_INT_DIV: f64 = 5500.0;
const BOUND_INT_ADD: f64 = 700.0;
const BOUND_INT_MUL: f64 = 6000.0;
const BOUND_INT_CMP: f64 = 1300.0;

/// An integer of the batch: a part of `digits` digits on one side, and on
/// every third draw on both.
fn integer(rng: &mut u64, digits: u32) -> String {
    let shape = {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (*rng >> 33) % 3
    };
    let a = number(rng, digits);
    match shape {
        0 => format!("(int {a} z)"),
        1 => format!("(int z {a})"),
        _ => format!("(int {a} {})", number(rng, digits)),
    }
}

fn batch_int(op: &str, count: usize, digits: u32) -> String {
    let mut rng = 0x5146_0607_u64;
    let mut src = String::from(IMPORT_INT);
    let mut seen = std::collections::BTreeSet::new();
    while seen.len() < count {
        let (x, y) = (integer(&mut rng, digits), integer(&mut rng, digits));
        // A division by zero is not a request of the batch: a divisor
        // written with two equal parts.
        if op.starts_with("int-div-") {
            let inner = y.trim_start_matches("(int ");
            let inner = &inner[..inner.len() - 1];
            let half = inner.len() / 2;
            if inner.len() % 2 == 1 && inner[..half] == inner[half + 1..] {
                continue;
            }
        }
        let request = format!("({op} {x} {y})\n");
        if seen.insert(request.clone()) {
            src.push_str(&request);
        }
    }
    src
}

/// **The integers keep the rule of the theory.** Every rule of `std.math.int` has
/// one premise (`int_theory.rs` holds that on the text), and each step is a
/// walk of `std.math.decimal-arith` under a tag of the module's. Here: a request of each
/// operation costs the same `candidates` and the same firings at 1 000 and
/// at 4 000 requests as at 250.
#[test]
fn a_batch_of_integer_requests_costs_in_proportion_to_its_count() {
    let sizes = [250usize, 1000, 4000];
    let units: Vec<_> = REQUESTS_INT.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_of_integer_requests_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result, bound, digits)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch_int(op, n, digits), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n)
                );
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of std.math.int \
                     joins two facts on a number; \
                     `int_theory.rs` names the rule if it is written as two premises",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
            }
        },
    );
}

// ── std.math.rat ────────────────────────────────────────────────────────

const IMPORT_RAT: &str = "(import std.math.rat :symbols (rat-arith))\n";

/// `std.math.rat`'s requests over two rationals, each with its result relation
/// and what a request may cost at the smallest batch, in `candidates`: a
/// little above the measure, as [`BOUND`] is.
const REQUESTS_RAT: [(&str, &str, f64); 5] = [
    ("rat-add", "rat-sum-of", BOUND_RAT_ADD),
    ("rat-sub", "rat-diff-of", BOUND_RAT_ADD),
    ("rat-mul", "rat-product-of", BOUND_RAT_MUL),
    ("rat-div", "rat-quot-of", BOUND_RAT_MUL),
    ("rat-cmp", "rat-cmp-of", BOUND_RAT_CMP),
];
// An operation divides nothing (Q-M1f.183): a sum or a difference is some
// 670 candidates a request at 100, a product 502 and a quotient 549, as this
// test measures them.
const BOUND_RAT_ADD: f64 = 750.0;
const BOUND_RAT_MUL: f64 = 650.0;
const BOUND_RAT_CMP: f64 = 900.0;

/// The batches of `std.math.rat`. Smaller than the others': a request put in
/// lowest terms is a chain of divisions, some hundreds of firings, and a
/// stored fact costs about a kilobyte. An operation divides nothing, and the
/// sizes are kept, so that the measures compare; `std.math.gcd`'s batch,
/// which the chain is, uses them too.
const SIZES_RAT: [usize; 3] = [100, 400, 1200];

/// A rational of the batch: a numerator from 1 to 9 and either sign, never
/// zero, so that it may divide, over a denominator from 2 to 9. Two-digit
/// terms are more than twice the firings, 2.4 million for 1 600 sums.
fn rational(rng: &mut u64) -> String {
    let mut draw = |n: u64| {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (*rng >> 33) % n
    };
    let spell = |v: u64| {
        let mut out = String::from("z");
        for ch in v.to_string().chars() {
            out = format!("(n {out} {ch})");
        }
        out
    };
    let (num, den, negative) = (1 + draw(9), 2 + draw(8), draw(2) == 1);
    if negative {
        format!("(rat (int z {}) {})", spell(num), spell(den))
    } else {
        format!("(rat (int {} z) {})", spell(num), spell(den))
    }
}

fn batch_rat(op: &str, count: usize) -> String {
    let mut rng = 0x5146_0608_u64;
    let mut src = String::from(IMPORT_RAT);
    let mut seen = std::collections::BTreeSet::new();
    while seen.len() < count {
        let request = format!("({op} {} {})\n", rational(&mut rng), rational(&mut rng));
        if seen.insert(request.clone()) {
            src.push_str(&request);
        }
    }
    src
}

/// **The rationals keep the rule of the theory.** Every rule of `std.math.rat`
/// has one premise (`rat_theory.rs` holds that on the text): the cross
/// products and the products of the parts are each a call of
/// `std.math.decimal-arith` or `std.math.int` under a tag, and an operation
/// divides nothing. Here: a request of each operation
/// costs the same `candidates` and the same firings at 400 and at 1 200
/// requests as at 100.
#[test]
fn a_batch_of_rational_requests_costs_in_proportion_to_its_count() {
    let sizes = SIZES_RAT;
    let units: Vec<_> = REQUESTS_RAT.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_of_rational_requests_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result, bound)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch_rat(op, n), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n)
                );
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE_RAT,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE_RAT,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of std.math.rat \
                     joins two facts on a number; \
                     `rat_theory.rs` names the rule if it is written as two premises",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
            }
        },
    );
}

/// What a request of `std.math.rat` costs depends on its operands more than a sum
/// of two nine-digit numbers does: the lengths of the cross products vary, as
/// the length of Euclid's chain does in `std.math.gcd`'s. A hundred requests
/// are a small sample of that, so the per-request
/// work at a larger batch may exceed the smallest's by this factor; a join on
/// a number multiplies it by the batches' ratio, 4 and 12.
const TOLERANCE_RAT: f64 = 1.15;

// ── std.math.binary-arith ───────────────────────────────────────────────

const IMPORT_BIN: &str = "(import std.math.binary-arith :symbols (binary-arith))\n";

/// Binary's requests, each with its result relation and what a request may
/// cost at the smallest batch, in `candidates`, a little above the measure:
/// thirty-bit operands, about nine decimal digits, as decimal's batches have
/// them; a divisor of thirteen bits, about four; and a
/// product of two twenty-bit numbers.
const REQUESTS_BIN: [(&str, &str, f64, [usize; 3]); 5] = [
    ("bin-add", "bin-sum-of", 600.0, SIZES_BIN),
    ("bin-sub", "bin-diff-of", 1200.0, SIZES_BIN),
    ("bin-cmp", "bin-cmp-of", 500.0, SIZES_BIN),
    ("bin-mul", "bin-product-of", 30000.0, SIZES_BIN_LONG),
    ("bin-div", "bin-quot-of", 30000.0, SIZES_BIN_LONG),
];

/// The batches of a walk: as decimal's.
const SIZES_BIN: [usize; 3] = [250, 1000, 4000];

/// The batches of a product and a quotient, a walk a bit of the multiplier
/// or the dividend: a batch of 4 000 of either took most of a minute of the
/// gate; the ratios, 4 and 12, are what a join would multiply by.
const SIZES_BIN_LONG: [usize; 3] = [100, 400, 1200];

/// A number of exactly `bits` bits in binary's spelling, its top bit 1.
fn number_bin(rng: &mut u64, bits: u32) -> String {
    let mut out = String::from("z");
    for i in 0..bits {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let b = if i == 0 { 1 } else { (*rng >> 33) % 2 };
        out = format!("(b2 {out} {b})");
    }
    out
}

/// `count` requests of `op` over binary numbers, beside `lets` named
/// constants nobody asks about, each written with a high zero so that no
/// numeral of a constant's is one of a request's, as [`batch`] writes
/// decimal's.
fn batch_bin(op: &str, count: usize, lets: usize) -> String {
    let (mut rng, mut names) = (0x5146_1616_u64, 0x5146_1617_u64);
    let mut src = String::from(IMPORT_BIN);
    for i in 0..lets {
        let value = number_bin(&mut names, 30).replacen("(b2 z ", "(b2 (b2 z 0) ", 1);
        src.push_str(&format!("(bin-let c{i} {value})\n"));
    }
    // A product of two thirty-bit numbers is thirty walks of up to sixty bits
    // a request, and its facts hold numbers as long: twenty bits a side keeps
    // the batch of products in proportion to the others' time.
    let (first, second) = match op {
        "bin-mul" => (20, 20),
        "bin-div" => (30, 13),
        _ => (30, 30),
    };
    for _ in 0..count {
        let (x, y) = (number_bin(&mut rng, first), number_bin(&mut rng, second));
        src.push_str(&format!("({op} {x} {y})\n"));
    }
    src
}

#[test]
fn a_batch_of_binary_requests_costs_in_proportion_to_its_count() {
    let units: Vec<_> = REQUESTS_BIN.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_of_binary_requests_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result, bound, sizes)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch_bin(op, n, 0), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request; \
                 {} guard values, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n),
                    w.guards,
                    per_request(w.guards, *n)
                );
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            let base_guards = per_request(work[0].guards, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of \
                     std.math.binary-arith or of std.math.positional joins two facts on a \
                     number; `arith_theory.rs` names the rule",
                        sizes[0]
                    );
                    let g = per_request(w.guards, *n);
                    assert!(
                        g <= base_guards * TOLERANCE,
                        "`{op}`: the guards' work per request grew with the batch: \
                     {base_guards:.1} values a request at {} and {g:.1} at {n}",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
                assert!(
                    base_guards < GUARD_BOUND,
                    "`{op}`: {base_guards:.1} guard values a request at {} requests, over \
                 {GUARD_BOUND}",
                    sizes[0]
                );
            }
        },
    );
}

/// The stated exception in binary: `operand-name`, `std.math.positional`'s,
/// asks `(bin-named ?x)` first, so the same requests beside `bin-let`s cost
/// what they cost beside none, plus the constants' own; and
/// `bin-operand-undeclared` costs two guard values a name of the program for
/// each operand, as decimal's does.
#[test]
fn a_binary_number_operand_does_not_walk_the_lets() {
    if !cfg!(feature = "counters") {
        return;
    }
    let (requests, lets) = (500usize, 200usize);
    let units: Vec<_> = REQUESTS_BIN.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_binary_number_operand_does_not_walk_the_lets",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result, _, _)| {
            let without = saturate(&batch_bin(op, requests, 0), result);
            let with = saturate(&batch_bin(op, requests, lets), result);
            let alone = saturate(&batch_bin(op, 0, lets), result);
            let none = saturate(&batch_bin(op, 0, 0), result);
            eprintln!(
                "{op}: {requests} requests: {} candidates beside no `bin-let`, {} beside {lets}; \
             the {lets} `bin-let`s alone, {}. Guard values: {}, {}, {}",
                without.candidates,
                with.candidates,
                alone.candidates,
                without.guards,
                with.guards,
                alone.guards
            );
            assert_eq!(
                with.candidates + none.candidates,
                without.candidates + alone.candidates,
                "{requests} `{op}` requests beside {lets} `bin-let`s cost more than the requests \
             and the `bin-let`s cost apart: a number operand is walking the `bin-let` facts"
            );
            assert_eq!(
                with.guards + none.guards,
                without.guards + alone.guards + (2 * 2 * requests * lets) as u64,
                "{requests} `{op}` requests beside {lets} `bin-let`s: two guard values for each \
             operand and each `bin-let`'s name"
            );
        },
    );
}

// ── std.math.gcd ────────────────────────────────────────────────────────

const IMPORT_GCD: &str = "(import std.math.gcd :symbols (gcd))\n";

/// `std.math.gcd`'s two requests, each with its result relation and what a
/// request may cost at the smallest batch, in `candidates`: a little above
/// the measure, as [`BOUND`] is.
const REQUESTS_GCD: [(&str, &str, f64); 2] =
    [("gcd", "gcd-of", BOUND_GCD), ("lcm", "lcm-of", BOUND_LCM)];
const BOUND_GCD: f64 = 6500.0;
const BOUND_LCM: f64 = 8300.0;

/// A number of the batch: one to three digits, its top digit not zero, so
/// that the chains are of every length a pair of such numbers has.
fn number_gcd(rng: &mut u64) -> String {
    let mut draw = |n: u64| {
        *rng = rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (*rng >> 33) % n
    };
    let digits = 1 + draw(3);
    let mut out = String::from("z");
    for i in 0..digits {
        let d = if i == 0 { 1 + draw(9) } else { draw(10) };
        out = format!("(n {out} {d})");
    }
    out
}

fn batch_gcd(op: &str, count: usize) -> String {
    let mut rng = 0x5146_0632_u64;
    let mut src = String::from(IMPORT_GCD);
    let mut seen = std::collections::BTreeSet::new();
    while seen.len() < count {
        let request = format!("({op} {} {})\n", number_gcd(&mut rng), number_gcd(&mut rng));
        if seen.insert(request.clone()) {
            src.push_str(&request);
        }
    }
    src
}

/// **The greatest common divisor keeps the rule of the theory.** Every rule
/// of `std.math.gcd` has one premise (`gcd_theory.rs` holds that on the
/// text): the two walks to normal form, Euclid's chain and the least common
/// multiple's product and division are each a call of `std.math.decimal-arith`
/// under a tag. Here: a request of each kind costs the same `candidates` and
/// the same firings at 400 and at 1 200 requests as at 100, within the
/// rationals' tolerance, since the chain's length varies from pair to pair as
/// it does there.
#[test]
fn a_batch_of_gcd_requests_costs_in_proportion_to_its_count() {
    let sizes = SIZES_RAT;
    let units: Vec<_> = REQUESTS_GCD.into_iter().collect();
    ein_corpus::pool::map(
        "arith_batch::a_batch_of_gcd_requests_costs_in_proportion_to_its_count",
        &units,
        |u| (format!("{:?}", u.0), String::new()),
        |&(op, result, bound)| {
            let work: Vec<Work> = sizes
                .iter()
                .map(|&n| saturate(&batch_gcd(op, n), result))
                .collect();
            for (n, w) in sizes.iter().zip(&work) {
                assert_eq!(w.answers, *n, "{n} `{op}` requests, each answered");
                eprintln!(
                    "{op}: {n} requests: {} firings, {:.2} a request; {} candidates, {:.2} a request",
                    w.firings,
                    per_request(w.firings, *n),
                    w.candidates,
                    per_request(w.candidates, *n)
                );
            }
            let base_firings = per_request(work[0].firings, sizes[0]);
            let base = per_request(work[0].candidates, sizes[0]);
            for (n, w) in sizes.iter().zip(&work).skip(1) {
                let firings = per_request(w.firings, *n);
                assert!(
                    firings <= base_firings * TOLERANCE_RAT,
                    "`{op}`: firings per request grew with the batch: {base_firings:.1} at {} \
                 and {firings:.1} at {n}",
                    sizes[0]
                );
                if cfg!(feature = "counters") {
                    let c = per_request(w.candidates, *n);
                    assert!(
                        c <= base * TOLERANCE_RAT,
                        "`{op}`: the matcher's work per request grew with the batch: {base:.1} \
                     candidates a request at {} and {c:.1} at {n}. A rule of std.math.gcd \
                     joins two facts on a number; \
                     `gcd_theory.rs` names the rule if it is written as two premises",
                        sizes[0]
                    );
                }
            }
            if cfg!(feature = "counters") {
                assert!(base > 0.0, "the counters are on and counted nothing");
                assert!(
                    base < bound,
                    "`{op}`: {base:.1} candidates a request at {} requests, over {bound}",
                    sizes[0]
                );
            }
        },
    );
}
