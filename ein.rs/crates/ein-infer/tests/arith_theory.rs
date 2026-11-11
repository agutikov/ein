//! **`std.arith`, held to the integers and to its own rule** — M1f
//! [S1f.6.3](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md).
//!
//! `stdlib/arith.ein` is school arithmetic written *in* Ein, over the digit
//! tables of `std.digits`. Ein has no numbers, so the engine cannot say
//! whether a sum is right, and a program under `tests/stdlib/arith/` can
//! claim a handful of sums and no more. What holds the theory is here.
//!
//! | claim | where |
//! |---|---|
//! | every request of each operation in [`OPS`] over two numbers up to [`SMALL`] is right | [`every_small_request_is_right`] |
//! | requests over operands of every length up to the operation's longest are right, on a sample, with equal operands and operands one apart at every length | [`requests_of_every_length_are_right`] |
//! | a carry or a borrow runs through every column and out of the top, at every length | [`a_carry_or_a_borrow_runs_the_whole_length`] |
//! | operands written with high zeros are read by value | [`high_zeros_are_read_by_value`] |
//! | a request of every operation may name its operands | [`named_operands_work_for_every_request`] |
//! | **every answer is in normal form**, and each request has exactly one | all five, through [`hold`] |
//! | **the rule of the theory**, on the module's text: no rule joins two facts on a number | [`no_rule_joins_two_facts_on_a_number`] |
//! | no `absent`, no `not`, no `or`: one stratum | [`the_theory_writes_no_negation`] |
//! | the closing rule closes every relation and names every rule | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//! | what one request costs as a circuit: its firings and its depth | [`the_circuit_of_one_addition`], [`the_circuit_of_one_subtraction_and_one_comparison`], [`the_circuit_of_one_multiplication`], [`the_circuit_of_one_division`] |
//! | a division by zero derives the flag, and the flag `(false)`, each by its own rule, and never a number | [`a_division_by_zero_is_flagged_and_false`] |
//! | division's two families of nine rules are alike but for the digit | [`the_rules_of_a_family_differ_by_their_digit_alone`] |
//!
//! The oracle is `u128`, which holds 38 decimal digits; the sampled operands
//! have up to [`MAX_DIGITS`] = 30, so no sum leaves it. The sample is drawn
//! by a fixed-seed generator written out below, so a failure names operands
//! that fail again.
//!
//! The *measured* half of the rule of the theory, a batch that costs in
//! proportion to its count, is `arith_batch.rs`; the search-free claim, on
//! the binary, is `ein-cli/tests/arith_search_free.rs`.
//!
//! **A later operation joins this file** by a row of [`OPS`], its request,
//! its result and what the integers say, and by a pin of its circuit; the
//! three static tests read the whole module and need no change to cover a new
//! rule. Subtraction and comparison joined at S1f.6.4, multiplication at
//! S1f.6.5, division at S1f.6.6.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// Every pair of operands from 0 to this is asked of every operation: 121 ×
/// 121 requests each, which cover every row of the full adder's and the full
/// subtractor's tables and the step from two digits to three.
const SMALL: u128 = 120;

/// The longest sampled operand, in digits.
const MAX_DIGITS: u32 = 30;

const IMPORT: &str = "(import std.arith :symbols (arith))\n";

// ── numbers as the theory writes them ──────────────────────────────

/// A string of decimal digits as a number of the theory, in reading order:
/// `""` is `z`, `"042"` is `(n (n (n z 0) 4) 2)`, high zero kept as written.
fn written(digits: &str) -> String {
    let mut out = String::from("z");
    for ch in digits.chars() {
        out = format!("(n {out} {ch})");
    }
    out
}

/// The normal form of a value: no high zero, and zero is `z`.
fn normal(v: u128) -> String {
    if v == 0 {
        "z".to_string()
    } else {
        written(&v.to_string())
    }
}

// ── a fixed-seed generator: the sample is the same on every run ────

/// SplitMix64. Twelve lines, no dependency, and the same stream everywhere.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /// A number of exactly `digits` digits, its top digit not zero; 0 for none.
    fn number(&mut self, digits: u32) -> u128 {
        let mut v: u128 = 0;
        for i in 0..digits {
            let d = if i == 0 {
                1 + self.below(9)
            } else {
                self.below(10)
            };
            v = v * 10 + u128::from(d);
        }
        v
    }
}

// ── running requests ───────────────────────────────────────────────

/// Split the text of a fact into its top-level parts: `(sum-of X Y S)` is
/// `["sum-of", X, Y, S]`, each a name or a parenthesised form.
fn parts(fact: &str) -> Vec<String> {
    let inner = fact
        .strip_prefix('(')
        .and_then(|f| f.strip_suffix(')'))
        .unwrap_or_else(|| panic!("not a form: {fact}"));
    let (mut out, mut depth, mut cur) = (Vec::new(), 0usize, String::new());
    for ch in inner.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ' ' if depth == 0 => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                continue;
            }
            _ => {}
        }
        cur.push(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Saturate `IMPORT` + `body` and return every fact of `relation`, as text.
fn run(body: &str, relation: &str) -> Vec<String> {
    let src = format!("{IMPORT}{body}");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<arith>")).expect("the requests parse");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the requests load");
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the theory compiles");
        sat.saturate(&mut s, None, &mut |_| {})
            .expect("the theory reaches its fixpoint");
    }
    let Some(rel) = terms.syms.get(relation) else {
        return Vec::new();
    };
    kb.facts_of(rel).map(|f| sexpr(&terms, f)).collect()
}

/// One operation of the theory, as the harness asks it: its request and
/// result relations, the longest operand it is sampled at, and what the
/// integers say the result's arguments after the operands are, each in the
/// theory's spelling.
struct Op {
    request: &'static str,
    result: &'static str,
    /// The longest sampled operand, in digits: the answer must stay in `u128`.
    max_digits: u32,
    /// `None` where the request has no answer: a division by zero, which
    /// the harness does not ask here ([`a_division_by_zero_is_flagged_and_false`]).
    answer: fn(u128, u128) -> Option<Vec<String>>,
}

/// The operations, in the order the stages added them. A later operation
/// joins the harness by a row here.
const OPS: [Op; 5] = [
    Op {
        request: "add",
        result: "sum-of",
        max_digits: MAX_DIGITS,
        answer: |a, b| Some(vec![normal(a + b)]),
    },
    // Two numbers: the difference where the first is not the smaller, and
    // what the first is short by where it is; the other is zero.
    Op {
        request: "sub",
        result: "diff-of",
        max_digits: MAX_DIGITS,
        answer: |a, b| {
            Some(vec![
                normal(a.saturating_sub(b)),
                normal(b.saturating_sub(a)),
            ])
        },
    },
    Op {
        request: "cmp",
        result: "cmp-of",
        max_digits: MAX_DIGITS,
        answer: |a, b| {
            Some(vec![
                match a.cmp(&b) {
                    std::cmp::Ordering::Less => "lt",
                    std::cmp::Ordering::Equal => "eq",
                    std::cmp::Ordering::Greater => "gt",
                }
                .to_string(),
            ])
        },
    },
    // A product of two numbers of 19 digits has at most 38, which `u128`
    // holds.
    Op {
        request: "mul",
        result: "product-of",
        max_digits: 19,
        answer: |a, b| Some(vec![normal(a * b)]),
    },
    // The quotient and the remainder. A zero divisor has no answer and is a
    // contradiction, so one such request would kill the whole program.
    Op {
        request: "div",
        result: "quot-of",
        max_digits: MAX_DIGITS,
        answer: |a, b| (b != 0).then(|| vec![normal(a / b), normal(a % b)]),
    },
];

/// An operand as written, and its value.
type Operand = (String, u128);

/// Ask every pair as one program of `op` requests and hold every answer:
/// **exactly one** result fact per request, repeating the operands as
/// written, its other arguments what the integers say, in normal form.
fn hold(op: &Op, cases: &[(Operand, Operand)]) {
    let mut want: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    let mut body = String::new();
    for ((x, a), (y, b)) in cases {
        let Some(answer) = (op.answer)(*a, *b) else {
            continue;
        };
        if want.insert((x.clone(), y.clone()), answer).is_none() {
            body.push_str(&format!("({} {x} {y})\n", op.request));
        }
    }
    let mut got: BTreeMap<(String, String), Vec<Vec<String>>> = BTreeMap::new();
    for fact in run(&body, op.result) {
        let p = parts(&fact);
        assert!(p.len() > 3, "a {} of another arity: {fact}", op.result);
        got.entry((p[1].clone(), p[2].clone()))
            .or_default()
            .push(p[3..].to_vec());
    }
    for (request, answer) in &want {
        let answers = got.remove(request).unwrap_or_default();
        assert_eq!(
            answers,
            std::slice::from_ref(answer),
            "({} {} {}): the integers say {answer:?}",
            op.request,
            request.0,
            request.1,
        );
    }
    assert!(
        got.is_empty(),
        "{}: answers to requests nobody made: {:?}",
        op.result,
        got.keys().collect::<Vec<_>>()
    );
}

fn value(v: u128) -> Operand {
    (normal(v), v)
}

// ── right, held to the integers ────────────────────────────────────

#[test]
fn every_small_request_is_right() {
    let mut cases = Vec::new();
    for a in 0..=SMALL {
        for b in 0..=SMALL {
            cases.push((value(a), value(b)));
        }
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

#[test]
fn requests_of_every_length_are_right() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0603);
        let mut cases = Vec::new();
        // Every pair of lengths from 0 to the operation's longest, twice.
        for la in 0..=op.max_digits {
            for lb in 0..=op.max_digits {
                for _ in 0..2 {
                    cases.push((value(rng.number(la)), value(rng.number(lb))));
                }
            }
        }
        // Equal operands, and operands one apart, at every length: the
        // difference of zero and of one, and the order decided in the units.
        for l in 1..=op.max_digits {
            let a = rng.number(l);
            cases.push((value(a), value(a)));
            cases.push((value(a), value(a - 1)));
            cases.push((value(a - 1), value(a)));
        }
        hold(op, &cases);
    }
}

#[test]
fn a_carry_or_a_borrow_runs_the_whole_length() {
    for op in &OPS {
        let mut cases = Vec::new();
        let mut nines: u128 = 0;
        for _ in 1..=op.max_digits.min(MAX_DIGITS) {
            nines = nines * 10 + 9;
            // 99…9 and 1, both ways, and 99…9 with itself: a sum one digit
            // longer.
            cases.push((value(nines), value(1)));
            cases.push((value(1), value(nines)));
            cases.push((value(nines), value(nines)));
            // 100…0 and 1, and 100…0 and 99…9: a borrow through every zero,
            // and an answer that loses every digit but one.
            cases.push((value(nines + 1), value(1)));
            cases.push((value(1), value(nines + 1)));
            cases.push((value(nines + 1), value(nines)));
            cases.push((value(nines), value(nines + 1)));
        }
        hold(op, &cases);
    }
}

#[test]
fn high_zeros_are_read_by_value() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0604);
        let pad = |rng: &mut Rng, v: u128| {
            let zeros = "0".repeat(rng.below(4) as usize);
            let digits = if v == 0 { String::new() } else { v.to_string() };
            (written(&format!("{zeros}{digits}")), v)
        };
        let mut cases = Vec::new();
        for _ in 0..1500 {
            let (la, lb) = (rng.below(13) as u32, rng.below(13) as u32);
            let (a, b) = (rng.number(la), rng.number(lb));
            let (x, y) = (pad(&mut rng, a), pad(&mut rng, b));
            cases.push((x, y));
        }
        // The same value, padded differently on the two sides.
        for _ in 0..200 {
            let l = rng.below(13) as u32;
            let a = rng.number(l);
            let (x, y) = (pad(&mut rng, a), pad(&mut rng, a));
            cases.push((x, y));
        }
        // Zero written as nothing but zeros, on either side and both.
        for zeros in ["0", "00", "000"] {
            cases.push(((written(zeros), 0), (written("7"), 7)));
            cases.push(((written("7"), 7), (written(zeros), 0)));
            cases.push(((written(zeros), 0), (written(zeros), 0)));
            cases.push(((written(zeros), 0), ("z".to_string(), 0)));
        }
        hold(op, &cases);
    }
}

/// A request may name an operand of every operation, and the answer repeats
/// the name: the same pairs, asked through `(let NAME NUMBER)`.
#[test]
fn named_operands_work_for_every_request() {
    let mut rng = Rng(0x5146_0605);
    let mut body = String::new();
    let mut names = Vec::new();
    for i in 0..40 {
        let l = rng.below(10) as u32;
        let v = rng.number(l);
        body.push_str(&format!("(let c{i} {})\n", normal(v)));
        names.push((format!("c{i}"), v));
    }
    for op in &OPS {
        let mut want = BTreeMap::new();
        let mut src = body.clone();
        for (x, a) in &names {
            for (y, b) in names.iter().step_by(7) {
                // By two names, and by a name and a number.
                let Some(answer) = (op.answer)(*a, *b) else {
                    continue;
                };
                src.push_str(&format!("({} {x} {y})\n", op.request));
                want.insert((x.clone(), y.clone()), answer.clone());
                src.push_str(&format!("({} {x} {})\n", op.request, normal(*b)));
                want.insert((x.clone(), normal(*b)), answer);
            }
        }
        let mut got = BTreeMap::new();
        for fact in run(&src, op.result) {
            let p = parts(&fact);
            assert!(
                got.insert((p[1].clone(), p[2].clone()), p[3..].to_vec())
                    .is_none(),
                "two answers: {fact}"
            );
        }
        assert_eq!(got, want, "{} by name", op.request);
    }
}

// ── division by zero ───────────────────────────────────────────────

/// **Division by zero, in two steps** (the user's ruling of 2026-11-11). A
/// request whose divisor is zero, written `z`, with digits, or by a name,
/// derives the flag `(division-by-zero (div X Y))`, the request as written;
/// the flag derives `(false)`; and the request is never answered with a
/// number. Requests whose divisor is not zero are not flagged.
///
/// The saturator here runs root to its fixpoint whatever it derives, so the
/// other requests of the program are answered in root's state beside the
/// contradiction; what a *solve* makes of a dead root, an empty R, is
/// `ein-cli/tests/arith_search_free.rs`'s.
#[test]
fn a_division_by_zero_is_flagged_and_false() {
    let zeros = ["z".to_string(), written("0"), written("000"), "none".into()];
    let dividends = ["z".to_string(), written("7"), written("120"), "none".into()];
    let mut body = String::from("(let none z)\n(let some (n z 3))\n");
    let mut want = BTreeSet::new();
    for x in &dividends {
        for y in &zeros {
            body.push_str(&format!("(div {x} {y})\n"));
            want.insert(format!("(division-by-zero (div {x} {y}))"));
        }
        body.push_str(&format!("(div {x} some)\n(div {x} (n z 3))\n"));
    }
    let flags: BTreeSet<String> = run(&body, "division-by-zero").into_iter().collect();
    assert_eq!(
        flags, want,
        "the flags are the requests that divide by zero"
    );
    assert_eq!(run(&body, "false"), ["(false)"], "the flag derives (false)");
    for fact in run(&body, "quot-of") {
        let p = parts(&fact);
        assert!(
            !zeros.contains(&p[2]),
            "a division by zero answered with a number: {fact}"
        );
    }
    assert_eq!(
        run(&body, "quot-of").len(),
        2 * dividends.len(),
        "the requests that do not divide by zero are each answered"
    );
    // No flag and no contradiction without a zero divisor.
    let fine = "(div (n z 7) (n z 3))\n(div z (n z 3))\n";
    assert!(run(fine, "division-by-zero").is_empty());
    assert!(run(fine, "false").is_empty());
}

// ── the circuit's measure ──────────────────────────────────────────

/// What one request costs, read off its firings: how many there are, the
/// circuit's **size**, and the longest chain of them, its **depth**, the
/// rounds a synchronous circuit would take. Ein's saturator is an agenda with
/// no rounds of its own, so the depth is computed here: a fact a firing
/// derives is one deeper than the deepest premise it read, and a written
/// fact, a table row among them, is at depth 0.
///
/// The closing rule's one firing is not the request's and is left out.
fn circuit(body: &str) -> (usize, usize) {
    let src = format!("{IMPORT}{body}");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<arith>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closer = terms.syms.get("arith").expect("the closing rule's name");
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the theory compiles");
        sat.saturate(&mut s, None, &mut |f| {
            if f.rule == closer {
                return;
            }
            firings += 1;
            let depth = 1 + f
                .premises
                .iter()
                .map(|p| depth_of.get(p).copied().unwrap_or(0))
                .max()
                .unwrap_or(0);
            for &d in f.derived.iter() {
                depth_of.entry(d).or_insert(depth);
            }
            deepest = deepest.max(depth);
        })
        .expect("the theory reaches its fixpoint");
    }
    (firings, deepest)
}

/// **One addition, as a circuit.** 123078 + 4567: two operands looked up
/// (5 firings with the request's own), six columns, the end of the walk, the
/// check for a high zero, six digits turned round, the hand-over and the
/// answer. A circuit of design K is a sequence, every call carrying its
/// continuation, so its depth is its size.
///
/// The numbers are pinned so that a change to the wiring shows as a change
/// to them: `cargo test -p ein-infer --test arith_theory the_circuit` is the
/// command that re-takes them.
#[test]
fn the_circuit_of_one_addition() {
    let six = written("123078");
    let four = written("4567");
    assert_eq!(
        circuit(&format!("(add {six} {four})\n")),
        (21, 21),
        "123078 + 4567: (firings, depth)"
    );
    // The carry out of the top column goes straight to the turn: nine columns,
    // ten digits turned, and no strip.
    assert_eq!(
        circuit(&format!(
            "(add {} {})\n",
            written("999999999"),
            written("1")
        )),
        (27, 27),
        "999999999 + 1: (firings, depth)"
    );
    // Named operands cost what numbers cost, a lookup either way, and each
    // `let` one firing that says its name is one.
    assert_eq!(
        circuit(&format!("(let a {six})\n(let b {four})\n(add a b)\n")),
        (23, 21),
        "the same sum by name"
    );
}

/// **One subtraction and one comparison, as circuits.** A subtraction is two
/// walks side by side, so its depth is less than its size: 123078 − 4567 is
/// 5 firings to ask and find the operands, then one walk of six columns that
/// ends with a borrow (7 firings) beside one of six columns, its end, the
/// check for a high zero, six digits turned, the hand-over and the answer
/// (16). The reverse request costs the same: the two walks exchange roles.
/// A comparison builds no number: 5 firings for the operands, a column a
/// firing, the end and the answer.
#[test]
fn the_circuit_of_one_subtraction_and_one_comparison() {
    let six = written("123078");
    let four = written("4567");
    assert_eq!(
        circuit(&format!("(sub {six} {four})\n")),
        (28, 21),
        "123078 - 4567: (firings, depth)"
    );
    assert_eq!(
        circuit(&format!("(sub {four} {six})\n")),
        (28, 21),
        "4567 - 123078: (firings, depth)"
    );
    // Equal numbers: both walks end with no borrow, each drops its six
    // zeros and hands over zero, and the two answer rules say the one fact.
    assert_eq!(
        circuit(&format!("(sub {six} {six})\n")),
        (35, 20),
        "123078 - 123078: (firings, depth)"
    );
    assert_eq!(
        circuit(&format!("(cmp {six} {four})\n")),
        (13, 13),
        "123078 against 4567: (firings, depth)"
    );
}

/// **One multiplication, as a circuit**, and how it grows with the two
/// lengths. The multiplier is turned round, a firing a digit; then each of
/// its digits is a row, a walk the length of the multiplicand with its strip
/// and turn, and, after the first, an addition walk the length of the sum so
/// far. So the size grows as the product of the two lengths, and the circuit
/// is still one chain: its depth is its size.
#[test]
fn the_circuit_of_one_multiplication() {
    let mul = |x: &str, y: &str| circuit(&format!("(mul {} {})\n", written(x), written(y)));
    assert_eq!(mul("123", "456"), (72, 72), "123 x 456: (firings, depth)");
    assert_eq!(
        mul("123078", "123078"),
        (227, 227),
        "123078 x 123078: (firings, depth)"
    );
    // The two lengths: a multiplier of one digit is one row; each digit more
    // is a row and an addition.
    assert_eq!(mul("999999", "9"), (28, 28), "six digits by one");
    assert_eq!(mul("9", "999999"), (128, 128), "one digit by six");
    assert_eq!(mul("999999", "999999"), (238, 238), "six digits by six");
    assert_eq!(
        mul("999999999999", "999999"),
        (370, 370),
        "twelve digits by six"
    );
    assert_eq!(
        mul("999999", "999999999999"),
        (556, 556),
        "six digits by twelve"
    );
}

/// **One division, as a circuit.** 127645 divided by 25: the divisor's nine
/// multiples, each a row of two columns; the dividend's six digits turned
/// round; then a step a digit, each three trials, and a trial a subtraction
/// with its strip and turn where it fits. One chain, so its depth is its
/// size. A divisor of one digit and a dividend of one are the two ends.
#[test]
fn the_circuit_of_one_division() {
    let div = |x: &str, y: &str| circuit(&format!("(div {} {})\n", written(x), written(y)));
    assert_eq!(
        div("127645", "25"),
        (220, 220),
        "127645 / 25: (firings, depth)"
    );
    assert_eq!(div("7", "3"), (84, 84), "7 / 3");
    assert_eq!(div("999999", "7"), (229, 229), "six digits by one");
    assert_eq!(div("999999999999", "7"), (394, 394), "twelve digits by one");
    assert_eq!(
        div("999999999999", "777777"),
        (655, 655),
        "twelve digits by six"
    );
    // A division by zero stops at the divisor's normal form: the operands,
    // one row, its strip, the flag and the contradiction.
    assert_eq!(div("127645", ""), (10, 10), "127645 / 0");
}

// ── the module's text ──────────────────────────────────────────────

/// The seven tables of `std.digits`: a premise over one is a lookup by digits.
const TABLES: [&str; 7] = ["fa", "fs", "md", "cl", "nz", "mid", "adj"];

/// The one rule that may join other facts, and the premises it has, **in
/// this order**: `operand-name` looks a `(let NAME NUMBER)` up by its name.
/// `(named ?x)` is first after the operand because every argument of it is
/// bound, which the engine answers by one lookup whatever `?x` is, a number
/// too; only a name reaches the `let`, which the fact index finds by the
/// name. With the two exchanged, or `named` left out, every number operand
/// walks every `let` (`arith_batch.rs` measures it). The module's header
/// states the exception; adding a rule here is a decision about the theory's
/// cost, not a way to make a test pass.
const NAME_LOOKUP: (&str, [&str; 3]) = ("operand-name", ["operand", "named", "let"]);

/// The one rule that concludes `(false)`.
const FALSE_RULE: &str = "division-by-zero-is-false";

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/arith.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.arith parses");
    Module { ast, forms }
}

impl Module {
    /// `(name, form)` of every top-level form headed `kind`.
    fn declared(&self, kind: &str) -> Vec<(String, NodeId)> {
        self.forms
            .iter()
            .filter(|&&f| self.ast.head_name(f) == Some(kind))
            .map(|&f| {
                let name = self
                    .ast
                    .form_args(f)
                    .first()
                    .and_then(|&a| self.ast.atom_name(a))
                    .unwrap_or_else(|| panic!("a `{kind}` with no name"));
                (name.to_string(), f)
            })
            .collect()
    }

    /// The value of a rule's `:match` or `:assert`.
    fn clause(&self, rule: NodeId, keyword: &str) -> NodeId {
        self.ast
            .form_args(rule)
            .iter()
            .find_map(|&a| match self.ast.node(a) {
                Node::KwPair { key, value } if self.keyword(key) == Some(keyword) => Some(value),
                _ => None,
            })
            .unwrap_or_else(|| panic!("a rule with no {keyword}"))
    }

    /// A key pair's keyword, without its colon.
    fn keyword(&self, key: NodeId) -> Option<&str> {
        match self.ast.node(key) {
            Node::Keyword(k) => Some(self.ast.sym(k)),
            _ => None,
        }
    }

    /// A clause's conjuncts: `(and a b)` is `a`, `b`; `()` is none; anything
    /// else is itself.
    fn conjuncts(&self, clause: NodeId) -> Vec<NodeId> {
        match self.ast.head_name(clause) {
            Some("and") => self.ast.form_args(clause).to_vec(),
            Some(h) if h == ein_core::text::EMPTY_HEAD => Vec::new(),
            _ => vec![clause],
        }
    }

    /// Every form head at any depth under `node`, `node`'s own included.
    fn heads(&self, node: NodeId, out: &mut Vec<String>) {
        if let Node::KwPair { value, .. } = self.ast.node(node) {
            self.heads(value, out);
            return;
        }
        if !matches!(self.ast.node(node), Node::SForm { .. }) {
            return;
        }
        if let Some(h) = self.ast.head_name(node) {
            out.push(h.to_string());
        }
        for &a in self.ast.form_args(node) {
            self.heads(a, out);
        }
    }
}

/// **The rule of the theory, on the text.** Every rule has at most one
/// premise that is not a row of `std.digits`; that premise holds the numbers,
/// and a table row holds none: its arguments are digits, names or variables,
/// never a form. The one exception is named in [`NAME_LOOKUP`].
#[test]
fn no_rule_joins_two_facts_on_a_number() {
    let m = module();
    let rules = m.declared("rule");
    assert!(
        rules.len() >= 18,
        "std.arith parsed to {} rules",
        rules.len()
    );
    let mut exception_seen = false;
    for (name, rule) in &rules {
        let mut state = Vec::new();
        if name == NAME_LOOKUP.0 {
            let premises: Vec<String> = m
                .conjuncts(m.clause(*rule, "match"))
                .iter()
                .map(|&p| m.ast.head_name(p).unwrap_or("?").to_string())
                .collect();
            assert_eq!(
                premises, NAME_LOOKUP.1,
                "rule {name}: the stated exception has these premises in this order"
            );
            exception_seen = true;
            continue;
        }
        for premise in m.conjuncts(m.clause(*rule, "match")) {
            let head = m
                .ast
                .head_name(premise)
                .unwrap_or_else(|| panic!("rule {name}: a premise with no relation"))
                .to_string();
            if TABLES.contains(&head.as_str()) {
                for &arg in m.ast.form_args(premise) {
                    assert!(
                        !matches!(m.ast.node(arg), Node::SForm { .. }),
                        "rule {name}: a row of `{head}` is looked up by digits, \
                         and this premise holds a form"
                    );
                }
            } else {
                state.push(head);
            }
        }
        assert!(
            state.len() <= 1,
            "rule {name} joins {state:?}: a rule of std.arith has ONE premise \
             that holds numbers, and its other premises are rows of std.digits. \
             A second one changes no answer and makes every batch of requests \
             cost the square of its count (E-3 of P1f.6's findings.md); carry \
             what the rule needs in the call's tag instead"
        );
    }
    assert!(
        exception_seen,
        "the exception this file names is no longer in the module: {NAME_LOOKUP:?}"
    );
}

/// No `absent`, no `not`, no `or` anywhere in a rule: the theory is monotone
/// and has one stratum, since a cycle through an `absent` is the only thing
/// that makes a second (`ein saturate --dump`'s `;; Strata` line says
/// *stratified, 1 stratum* of any program that imports it).
#[test]
fn the_theory_writes_no_negation() {
    let m = module();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        m.heads(m.clause(rule, "assert"), &mut heads);
        for banned in ["absent", "not", "or", "neq"] {
            assert!(
                !heads.iter().any(|h| h == banned),
                "rule {name} writes `{banned}`"
            );
        }
        // One rule concludes `(false)`, from the flag and nothing else: a
        // program that divides by zero is a contradiction (the user's ruling
        // of 2026-11-11). It reads no `absent`, so the theory is monotone
        // still, and no other rule may conclude it.
        let concludes_false = heads.iter().any(|h| h == "false");
        assert_eq!(
            concludes_false,
            name == FALSE_RULE,
            "`(false)` is concluded by {FALSE_RULE} and by no other rule: {name}"
        );
        if name == FALSE_RULE {
            let premises: Vec<&str> = m
                .conjuncts(m.clause(rule, "match"))
                .iter()
                .filter_map(|&p| m.ast.head_name(p))
                .collect();
            assert_eq!(
                premises,
                ["division-by-zero"],
                "{FALSE_RULE} reads the flag"
            );
        }
    }
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.arith guesses nothing and expands nothing"
    );
}

/// `arith` is the theory's one entry. A relation it does not close is one the
/// search may guess facts of; a rule it does not name is one
/// `(import std.arith :symbols (arith))` leaves behind, in silence. Both are
/// easy to forget when a rule is added, and neither changes an answer the day
/// it is forgotten.
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "arith")
        .expect("the closing rule, `arith`");
    assert!(
        m.conjuncts(m.clause(*closer, "match")).is_empty(),
        "`arith` fires at root, on nothing"
    );
    let (mut closed, mut named) = (BTreeSet::new(), BTreeSet::new());
    for c in m.conjuncts(m.clause(*closer, "assert")) {
        let arg = m
            .ast
            .form_args(c)
            .first()
            .and_then(|&a| m.ast.atom_name(a))
            .expect("one name")
            .to_string();
        match m.ast.head_name(c) {
            Some("__closed__") => assert!(closed.insert(arg.clone()), "closed twice: {arg}"),
            Some("arith-rule") => assert!(named.insert(arg.clone()), "named twice: {arg}"),
            other => panic!("`arith` asserts a {other:?}"),
        }
    }

    // The tables the module imports, read off its `(import std.digits …)`.
    let mut tables = BTreeSet::new();
    for &f in &m.forms {
        if m.ast.head_name(f) != Some("import") {
            continue;
        }
        let args = m.ast.form_args(f);
        assert_eq!(
            m.ast.atom_name(args[0]),
            Some("std.digits"),
            "std.arith imports std.digits and nothing else"
        );
        for &a in &args[1..] {
            if let Node::KwPair { key, value } = m.ast.node(a) {
                assert_eq!(m.keyword(key), Some("symbols"));
                // `(fa nz)`: a form whose head is the first name.
                tables.extend(m.ast.head_name(value).map(str::to_string));
                tables.extend(
                    m.ast
                        .form_args(value)
                        .iter()
                        .filter_map(|&t| m.ast.atom_name(t).map(str::to_string)),
                );
            }
        }
    }
    assert!(
        tables.iter().all(|t| TABLES.contains(&t.as_str())) && !tables.is_empty(),
        "the import lists tables of std.digits: {tables:?}"
    );

    let mut want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    want_closed.extend(tables);
    assert_eq!(
        closed, want_closed,
        "`arith` closes every relation the module declares and every table it imports"
    );
    let want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "arith")
        .collect();
    assert_eq!(
        named, want_named,
        "`arith` names every other rule of the module"
    );

    // Its own name is a written fact, the module's only one: what keeps the
    // closing rule in a program that is inlined and tree-shaken, as a
    // `--dump-answer` state file's is (E-11 of P1f.6's findings.md;
    // `ein-cli`'s `answer_round_trip.rs` is what fails without it).
    let facts: Vec<(String, Vec<String>)> = m
        .forms
        .iter()
        .filter(|&&f| {
            !matches!(
                m.ast.head_name(f),
                Some("relation" | "rule" | "import") | None
            )
        })
        .map(|&f| {
            (
                m.ast.head_name(f).expect("a head").to_string(),
                m.ast
                    .form_args(f)
                    .iter()
                    .filter_map(|&a| m.ast.atom_name(a).map(str::to_string))
                    .collect(),
            )
        })
        .collect();
    assert_eq!(
        facts,
        [("arith-rule".to_string(), vec!["arith".to_string()])],
        "the module writes one fact, the closing rule's own name"
    );
}

/// **A family of rules is one rule written nine times.** Division tries a
/// multiple of the divisor by a rule a multiple, `div-try-1` to `div-try-9`,
/// since a rule cannot pick the J-th of nine by a digit without joining on a
/// number; and it computes the multiples by a chain, `div-multiple-3` to
/// `div-multiple-9`. They are written by hand, so this holds that they are
/// alike: each rule's text is the family's first with its digit, and the
/// variable that names its multiple, exchanged.
#[test]
fn the_rules_of_a_family_differ_by_their_digit_alone() {
    let path = repo_root().join("stdlib/arith.ein");
    let text = std::fs::read_to_string(&path).expect("std.arith");
    let rule = |name: &str| -> String {
        let start = text
            .find(&format!("(rule {name} ()"))
            .unwrap_or_else(|| panic!("no rule {name}"));
        let end = text[start..].find("\n\n").map_or(text.len(), |e| start + e);
        text[start..end].to_string()
    };
    // The trials: `div-try-J` reads `(mid ?lo ?hi J)`, tags its trial J, and
    // subtracts `?mJ`. The list of the nine multiples is the same in each.
    let list = "?m1 ?m2 ?m3 ?m4 ?m5 ?m6 ?m7 ?m8 ?m9";
    let shape = |j: u32| {
        rule(&format!("div-try-{j}"))
            .replace(list, "<multiples>")
            .replace(&format!("?m{j}"), "<multiple>")
            .replace(&j.to_string(), "<digit>")
    };
    let first = shape(1);
    assert_eq!(first.matches("<digit>").count(), 5, "{first}");
    assert_eq!(first.matches("<multiple>").count(), 2, "{first}");
    for j in 2..=9 {
        assert_eq!(shape(j), first, "div-try-{j} against div-try-1");
    }
    // The chain: `div-multiple-J` reads J−1 times the divisor out of the tag
    // `div-m(J−1)`, which holds the multiples before it, and asks for J.
    for j in 3..=9u32 {
        let known: Vec<String> = (1..j - 1).map(|i| format!("?m{i}")).collect();
        let known = known.join(" ");
        let before = if known.is_empty() {
            String::new()
        } else {
            format!(" {known}")
        };
        let want = format!(
            "(rule div-multiple-{j} ()\n  \
             :match  (done (div-m{p} ?k ?u{before}) ?m{p})\n  \
             :assert (mul-walk (div-m{j} ?k ?u{before} ?m{p}) ?m1 {j} 0 z)\n  \
             :why    \"{p} times the divisor is {{?m{p}}}: next, {j} times the divisor\")",
            p = j - 1
        );
        assert_eq!(rule(&format!("div-multiple-{j}")), want);
    }
}
