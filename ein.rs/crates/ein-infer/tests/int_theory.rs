//! **`std.int`, held to the signed integers and to the rule of the theory** —
//! M1f
//! [S1f.6.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md).
//!
//! `stdlib/int.ein` is the calculator's first extension: an integer is
//! `(int P N)`, two natural numbers of `std.arith`, and its value is P less
//! N. The module calls `std.arith`'s walks and does not edit it. Ein has no
//! numbers, so what holds the theory's answers is here, as
//! `arith_theory.rs` holds `std.arith`'s.
//!
//! | claim | where |
//! |---|---|
//! | every request of each operation in [`OPS`] over **every pair of integers written with parts from 0 to [`PART`]** is right: every combination of signs, simplified and not | [`every_pair_of_small_parts_is_right`] |
//! | requests over operands of every length up to the operation's longest are right, on a sample, in every combination of signs, with equal operands and operands one apart | [`requests_of_every_length_and_sign_are_right`] |
//! | an operand that is not simplified, and one written with high zeros, answers as its normal form does | [`an_operand_is_read_by_value`] |
//! | simplification and negation are right, at every length | [`simplification_and_negation_are_right`] |
//! | a request of every operation may name its operands | [`named_operands_work_for_every_request`] |
//! | a natural number is not an integer operand: the request is never answered | [`a_natural_number_is_not_an_integer_operand`] |
//! | **every answer is in normal form**, and each request has exactly one | all of them, through [`hold`] |
//! | what one request costs as a circuit: its firings and its depth | [`the_circuit_of_each_operation`] |
//! | **the rule of the theory**, on the module's text: every rule has one premise, so none joins two facts | [`every_rule_has_one_premise`] |
//! | no `absent`, `not`, `or`, guard or `(false)` | [`the_theory_writes_no_negation_and_no_guard`] |
//! | a division by zero derives `std.arith`'s flag, the request as written, and `(false)`, and no quotient | [`a_division_by_zero_is_the_flag_under_every_convention`] |
//! | the closing rule closes every relation and names every rule, and names `std.arith`'s entry | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//!
//! The oracle is `i128`. The *measured* half of the rule of the theory is
//! `arith_batch.rs`; the search-free claim, on the binary, is
//! `ein-cli/tests/arith_search_free.rs`.
//!
//! **Integer division is three rows of [`OPS`]** since M1f
//! [S1f.6.22](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.22_integer_division.md),
//! one a convention, each answered by a quotient and a remainder: Euclidean
//! against `i128`'s `div_euclid` and `rem_euclid`, its remainder a natural
//! number; truncated against `/` and `%`; floored against the pair computed
//! from them. A request whose divisor is zero is left out of every sample
//! and held by [`a_division_by_zero_is_the_flag_under_every_convention`].

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// Every integer written `(int P N)` with both parts from 0 to this is an
/// operand of [`every_pair_of_small_parts_is_right`]: 100 spellings of the
/// integers from −9 to 9, so 10 000 pairs an operation, every combination of
/// signs and of simplified and unsimplified operands.
const PART: u128 = 9;

const IMPORT: &str = "(import std.int :symbols (int-arith))\n";

// ── numbers and integers as the theory writes them ─────────────────

/// A string of decimal digits as a number of `std.arith`, high zeros kept.
fn written(digits: &str) -> String {
    let mut out = String::from("z");
    for ch in digits.chars() {
        out = format!("(n {out} {ch})");
    }
    out
}

/// The normal form of a natural number.
fn natural(v: u128) -> String {
    if v == 0 {
        "z".to_string()
    } else {
        written(&v.to_string())
    }
}

/// An integer as written, from its two parts.
fn pair(p: u128, n: u128) -> String {
    format!("(int {} {})", natural(p), natural(n))
}

/// The normal form of an integer: one part zero.
fn normal(v: i128) -> String {
    if v >= 0 {
        pair(v.unsigned_abs(), 0)
    } else {
        pair(0, v.unsigned_abs())
    }
}

/// An operand as written, and its value.
type Operand = (String, i128);

fn value(v: i128) -> Operand {
    (normal(v), v)
}

fn parts_of(p: u128, n: u128) -> Operand {
    (pair(p, n), p as i128 - n as i128)
}

// ── a fixed-seed generator: the sample is the same on every run ────

/// SplitMix64, as `arith_theory.rs` has it.
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

/// Split the text of a fact into its top-level parts.
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
    let forms = parse(&mut ast, &src, Some("<int>")).expect("the requests parse");
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

/// One operation over two integers: its request and result relations, the
/// longest operand it is sampled at, and what the signed integers say the
/// result's arguments after the operands are, in the theory's spelling: one
/// for most, and a quotient and a remainder, with a space between, for a
/// division.
struct Op {
    request: &'static str,
    result: &'static str,
    /// The longest sampled part, in digits: the answer must stay in `i128`.
    max_digits: u32,
    /// A division: a request whose second operand is zero has no answer,
    /// and is left out of what [`hold`] asks.
    divides: bool,
    answer: fn(i128, i128) -> String,
}

/// The floored quotient and remainder, from `i128`'s truncated pair: one
/// less in the quotient where a remainder is left and the signs differ.
fn floored(a: i128, b: i128) -> (i128, i128) {
    let (q, r) = (a / b, a % b);
    if r != 0 && (r < 0) != (b < 0) {
        (q - 1, r + b)
    } else {
        (q, r)
    }
}

const OPS: [Op; 7] = [
    Op {
        request: "int-add",
        result: "int-sum-of",
        max_digits: 30,
        divides: false,
        answer: |a, b| normal(a + b),
    },
    Op {
        request: "int-sub",
        result: "int-diff-of",
        max_digits: 30,
        divides: false,
        answer: |a, b| normal(a - b),
    },
    // The Euclidean remainder is a **natural number** of `std.arith`, in
    // normal form, and never an integer (the user's ruling of 2026-11-12).
    Op {
        request: "int-div-euclid",
        result: "int-quot-euclid-of",
        max_digits: 30,
        divides: true,
        answer: |a, b| {
            format!(
                "{} {}",
                normal(a.div_euclid(b)),
                natural(a.rem_euclid(b).unsigned_abs())
            )
        },
    },
    Op {
        request: "int-div-trunc",
        result: "int-quot-trunc-of",
        max_digits: 30,
        divides: true,
        answer: |a, b| format!("{} {}", normal(a / b), normal(a % b)),
    },
    Op {
        request: "int-div-floor",
        result: "int-quot-floor-of",
        max_digits: 30,
        divides: true,
        answer: |a, b| {
            let (q, r) = floored(a, b);
            format!("{} {}", normal(q), normal(r))
        },
    },
    // Two integers of 18 digits have a product of at most 36, which `i128`
    // holds, as it holds the two sums of products the circuit makes.
    Op {
        request: "int-mul",
        result: "int-product-of",
        max_digits: 18,
        divides: false,
        answer: |a, b| normal(a * b),
    },
    Op {
        request: "int-cmp",
        result: "int-cmp-of",
        max_digits: 30,
        divides: false,
        answer: |a, b| {
            match a.cmp(&b) {
                std::cmp::Ordering::Less => "lt",
                std::cmp::Ordering::Equal => "eq",
                std::cmp::Ordering::Greater => "gt",
            }
            .to_string()
        },
    },
];

/// Ask every pair as one program of `op` requests and hold every answer:
/// **exactly one** result fact per request, repeating the operands as
/// written, what follows them what the integers say, in normal form. A
/// division by zero is not asked.
fn hold(op: &Op, cases: &[(Operand, Operand)]) {
    let mut want: BTreeMap<(String, String), String> = BTreeMap::new();
    let mut body = String::new();
    for ((x, a), (y, b)) in cases {
        if op.divides && *b == 0 {
            continue;
        }
        if want
            .insert((x.clone(), y.clone()), (op.answer)(*a, *b))
            .is_none()
        {
            body.push_str(&format!("({} {x} {y})\n", op.request));
        }
    }
    let mut got: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for fact in run(&body, op.result) {
        let p = parts(&fact);
        assert_eq!(
            p.len(),
            if op.divides { 5 } else { 4 },
            "a {} of another arity: {fact}",
            op.result
        );
        got.entry((p[1].clone(), p[2].clone()))
            .or_default()
            .push(p[3..].join(" "));
    }
    for (request, answer) in &want {
        let answers = got.remove(request).unwrap_or_default();
        assert_eq!(
            answers,
            std::slice::from_ref(answer),
            "({} {} {}): the integers say {answer}",
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

// ── right, held to the integers ────────────────────────────────────

/// The exhausting width: every spelling of every integer from −9 to 9 with
/// parts of one digit, against every other, under every operation. Every
/// combination of signs is here, zero on either side, and every operand
/// that is not simplified, `(int 5 3)` beside `(int 2 z)`.
#[test]
fn every_pair_of_small_parts_is_right() {
    let mut operands = Vec::new();
    for p in 0..=PART {
        for n in 0..=PART {
            operands.push(parts_of(p, n));
        }
    }
    let mut cases = Vec::new();
    for x in &operands {
        for y in &operands {
            cases.push((x.clone(), y.clone()));
        }
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

#[test]
fn requests_of_every_length_and_sign_are_right() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0607);
        let mut cases = Vec::new();
        let signed = |rng: &mut Rng, v: u128| {
            if rng.below(2) == 0 {
                v as i128
            } else {
                -(v as i128)
            }
        };
        // Every pair of lengths from 0 to the operation's longest, in the
        // four combinations of signs.
        for la in (0..=op.max_digits).step_by(if op.max_digits > 20 { 2 } else { 1 }) {
            for lb in 0..=op.max_digits {
                let (a, b) = (rng.number(la) as i128, rng.number(lb) as i128);
                cases.push((value(a), value(b)));
                cases.push((value(-a), value(b)));
                cases.push((value(a), value(-b)));
                cases.push((value(-a), value(-b)));
            }
        }
        // Equal operands, operands one apart and opposite operands at every
        // length: a sum of zero, an order decided in the units.
        for l in 1..=op.max_digits {
            let magnitude = rng.number(l);
            let a = signed(&mut rng, magnitude);
            cases.push((value(a), value(a)));
            cases.push((value(a), value(a - 1)));
            cases.push((value(a - 1), value(a)));
            cases.push((value(a), value(-a)));
        }
        // Both parts long: an operand that is not simplified, at length.
        for _ in 0..200 {
            let l = 1 + rng.below(u64::from(op.max_digits)) as u32;
            let (p, n, q, m) = (rng.number(l), rng.number(l), rng.number(l), rng.number(l));
            cases.push((parts_of(p, n), parts_of(q, m)));
        }
        hold(op, &cases);
    }
}

/// The stage's task 3: *an unsimplified operand, such as (5, 3), must give
/// the same answers as (2, 0)*, and a part written with high zeros is read
/// by value.
#[test]
fn an_operand_is_read_by_value() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0608);
        let spell = |rng: &mut Rng, v: i128| -> Operand {
            // v = p − n with a common amount added to both parts, and each
            // part padded with up to three high zeros.
            let digits = rng.below(6) as u32;
            let extra = rng.number(digits);
            let (p, n) = if v >= 0 {
                (v.unsigned_abs() + extra, extra)
            } else {
                (extra, v.unsigned_abs() + extra)
            };
            let pad = |rng: &mut Rng, v: u128| {
                let zeros = "0".repeat(rng.below(4) as usize);
                let digits = if v == 0 { String::new() } else { v.to_string() };
                written(&format!("{zeros}{digits}"))
            };
            (format!("(int {} {})", pad(rng, p), pad(rng, n)), v)
        };
        let mut cases = Vec::new();
        for _ in 0..1200 {
            let (la, lb) = (rng.below(10) as u32, rng.below(10) as u32);
            let a = rng.number(la) as i128 * if rng.below(2) == 0 { 1 } else { -1 };
            let b = rng.number(lb) as i128 * if rng.below(2) == 0 { 1 } else { -1 };
            let (x, y) = (spell(&mut rng, a), spell(&mut rng, b));
            cases.push((x, y));
        }
        // The same integer, spelt differently on the two sides.
        for _ in 0..200 {
            let l = rng.below(10) as u32;
            let a = rng.number(l) as i128 * if rng.below(2) == 0 { 1 } else { -1 };
            let (x, y) = (spell(&mut rng, a), spell(&mut rng, a));
            cases.push((x, y));
        }
        hold(op, &cases);
    }
}

/// The two requests with one operand: `(int-simplify X)` and `(int-neg X)`.
#[test]
fn simplification_and_negation_are_right() {
    let mut rng = Rng(0x5146_0609);
    let mut operands: Vec<Operand> = Vec::new();
    for p in 0..=30u128 {
        for n in 0..=30u128 {
            operands.push(parts_of(p, n));
        }
    }
    for l in 0..=30u32 {
        for _ in 0..4 {
            let lb = rng.below(31) as u32;
            operands.push(parts_of(rng.number(l), rng.number(lb)));
        }
        let a = rng.number(l);
        operands.push(parts_of(a, a));
        operands.push(parts_of(a + 1, a));
        operands.push(parts_of(a, a + 1));
    }
    // High zeros in either part, and a zero that is nothing but zeros.
    for (p, n, v) in [
        ("007", "012", -5i128),
        ("00", "", 0),
        ("", "000", 0),
        ("0100", "01", 99),
    ] {
        operands.push((format!("(int {} {})", written(p), written(n)), v));
    }
    type Unary = (&'static str, &'static str, fn(i128) -> i128);
    let unary: [Unary; 2] = [
        ("int-simplify", "int-simple-of", |v| v),
        ("int-neg", "int-neg-of", |v| -v),
    ];
    for (request, result, f) in unary {
        let mut want: BTreeMap<String, String> = BTreeMap::new();
        let mut body = String::new();
        for (x, v) in &operands {
            if want.insert(x.clone(), normal(f(*v))).is_none() {
                body.push_str(&format!("({request} {x})\n"));
            }
        }
        let mut got: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for fact in run(&body, result) {
            let p = parts(&fact);
            assert_eq!(p.len(), 3, "{fact}");
            got.entry(p[1].clone()).or_default().push(p[2].clone());
        }
        for (x, answer) in &want {
            assert_eq!(
                got.remove(x).unwrap_or_default(),
                std::slice::from_ref(answer),
                "({request} {x})"
            );
        }
        assert!(got.is_empty(), "{result}: answers nobody asked for");
    }
}

/// A request may name an operand, through `std.arith`'s `let`, and the
/// answer repeats the name.
#[test]
fn named_operands_work_for_every_request() {
    let mut rng = Rng(0x5146_060A);
    let mut body = String::new();
    let mut names = Vec::new();
    for i in 0..30 {
        let (lp, ln) = (rng.below(8) as u32, rng.below(8) as u32);
        let (p, n) = (rng.number(lp), rng.number(ln));
        body.push_str(&format!("(let c{i} {})\n", pair(p, n)));
        names.push((format!("c{i}"), p as i128 - n as i128));
    }
    for op in &OPS {
        let mut want = BTreeMap::new();
        let mut src = body.clone();
        for (x, a) in &names {
            for (y, b) in names.iter().step_by(7) {
                if op.divides && *b == 0 {
                    continue;
                }
                // By two names, and by a name and an integer written out.
                src.push_str(&format!("({} {x} {y})\n", op.request));
                want.insert((x.clone(), y.clone()), (op.answer)(*a, *b));
                src.push_str(&format!("({} {x} {})\n", op.request, normal(*b)));
                want.insert((x.clone(), normal(*b)), (op.answer)(*a, *b));
            }
        }
        let mut got = BTreeMap::new();
        for fact in run(&src, op.result) {
            let p = parts(&fact);
            assert!(
                got.insert((p[1].clone(), p[2].clone()), p[3..].join(" "))
                    .is_none(),
                "two answers: {fact}"
            );
        }
        assert_eq!(got, want, "{} by name", op.request);
    }
    let mut src = body.clone();
    for (x, _) in &names {
        src.push_str(&format!("(int-neg {x})\n(int-simplify {x})\n"));
    }
    let negs: BTreeSet<String> = run(&src, "int-neg-of").into_iter().collect();
    let simples: BTreeSet<String> = run(&src, "int-simple-of").into_iter().collect();
    for (x, v) in &names {
        assert!(negs.contains(&format!("(int-neg-of {x} {})", normal(-*v))));
        assert!(simples.contains(&format!("(int-simple-of {x} {})", normal(*v))));
    }
    assert_eq!((negs.len(), simples.len()), (names.len(), names.len()));
}

/// The module's stated limit (Q-M1f.175): an operand of an integer request is
/// an integer, or a name of one. A natural number in an operand's place is
/// answered by silence, as any malformed operand of the calculator is today
/// (S1f.6.12's investigation), and the well-formed request beside it is
/// answered.
#[test]
fn a_natural_number_is_not_an_integer_operand() {
    let five = written("5");
    let body = format!(
        "(let n5 {five})\n\
         (int-add {five} (int z z))\n(int-add (int z z) {five})\n(int-add n5 (int z z))\n\
         (int-neg {five})\n(int-mul (int (n z 2) z) {five})\n\
         (int-add (int (n z 2) z) (int (n z 3) z))\n"
    );
    assert_eq!(
        run(&body, "int-sum-of"),
        ["(int-sum-of (int (n z 2) z) (int (n z 3) z) (int (n z 5) z))"]
    );
    assert!(run(&body, "int-neg-of").is_empty());
    assert!(run(&body, "int-product-of").is_empty());
    assert!(run(&body, "false").is_empty());
}

/// **A division by zero, under every convention**: the divisor is zero by
/// value, written `(int z z)`, with both parts equal, with digits, or by
/// name. `int-div-by-zero` derives `std.arith`'s flag with the request as
/// written, `std.arith`'s rule derives `(false)`, and the request has no
/// quotient; the request beside it is answered.
///
/// **One flag a request, and it is the request's** (S1f.6.24). `std.arith`
/// flags a division at its tagged call since that stage, whoever made it;
/// this module sees a divisor that is zero by its shape before it calls, so
/// no call with a zero divisor is made and no `(division-by-zero (div-call
/// …))` stands beside the request's flag. The equality below is exact.
#[test]
fn a_division_by_zero_is_the_flag_under_every_convention() {
    let zeros = [
        "(int z z)".to_string(),
        pair(3, 3),
        format!("(int {} {})", written("00"), written("0")),
        "nothing".to_string(),
    ];
    for op in OPS.iter().filter(|op| op.divides) {
        for zero in &zeros {
            for dividend in [normal(7), normal(-7), normal(0), pair(5, 12)] {
                let body = format!(
                    "(let nothing (int z z))\n({} {dividend} {zero})\n({} {} {})\n",
                    op.request,
                    op.request,
                    normal(-7),
                    normal(2)
                );
                assert_eq!(
                    run(&body, "division-by-zero"),
                    [format!(
                        "(division-by-zero ({} {dividend} {zero}))",
                        op.request
                    )],
                    "{body}"
                );
                assert_eq!(run(&body, "false"), ["(false)"], "{body}");
                let answers = run(&body, op.result);
                assert_eq!(answers.len(), 1, "{body}: {answers:?}");
                assert!(
                    answers[0].starts_with(&format!(
                        "({} {} {} ",
                        op.result,
                        normal(-7),
                        normal(2)
                    )),
                    "{body}: {answers:?}"
                );
            }
        }
    }
}

// ── the circuit's measure ──────────────────────────────────────────

/// Firings and depth of one request, as `arith_theory.rs` measures them; the
/// two closing rules' firings are left out.
fn circuit(body: &str) -> (usize, usize) {
    let src = format!("{IMPORT}{body}");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<int>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closers = [
        terms.syms.get("arith").expect("std.arith's closing rule"),
        terms.syms.get("int-arith").expect("std.int's closing rule"),
    ];
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
            if closers.contains(&f.rule) {
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

/// **Each operation as a circuit**, on the operands `arith_theory.rs` pins
/// `std.arith`'s on, with a sign: 123078 and −4567. A sum is two addition
/// walks and the two subtraction walks of the simplification, which run side
/// by side, so its depth is less than its size. A product is four of
/// `std.arith`'s products, of which three have a zero operand here, two
/// sums and the simplification. A division is two simplifications,
/// `std.arith`'s division of the sizes and, where the convention adjusts,
/// an addition walk and a subtraction walk: −7 by 2, and −127645 by 25,
/// under each convention.
///
/// `cargo test -p ein-infer --test int_theory the_circuit` re-takes them.
#[test]
fn the_circuit_of_each_operation() {
    let (a, b) = (normal(123_078), normal(-4567));
    let measured = [
        circuit(&format!("(int-simplify {})\n", pair(123_078, 4567))),
        circuit(&format!("(int-neg {b})\n")),
        circuit(&format!("(int-add {a} {b})\n")),
        circuit(&format!("(int-sub {a} {b})\n")),
        circuit(&format!("(int-cmp {a} {b})\n")),
        circuit(&format!("(int-mul {} {})\n", normal(123), normal(-456))),
        circuit(&format!(
            "(int-mul {} {})\n",
            normal(-123_078),
            normal(-123_078)
        )),
        circuit(&format!(
            "(int-mul {} {})\n",
            pair(123 + 77, 77),
            pair(1000, 1456)
        )),
        circuit(&format!("(int-div-euclid {} {})\n", normal(-7), normal(2))),
        circuit(&format!("(int-div-trunc {} {})\n", normal(-7), normal(2))),
        circuit(&format!("(int-div-floor {} {})\n", normal(-7), normal(2))),
        circuit(&format!(
            "(int-div-euclid {} {})\n",
            normal(-127_645),
            normal(25)
        )),
        circuit(&format!(
            "(int-div-trunc {} {})\n",
            normal(-127_645),
            normal(25)
        )),
        circuit(&format!(
            "(int-div-floor {} {})\n",
            normal(-127_645),
            normal(25)
        )),
    ];
    eprintln!(
        "simplify, neg, add, sub, cmp, mul 123 x -456, mul -123078 squared, mul unsimplified, \
         -7 / 2 Euclidean, truncated, floored, -127645 / 25 Euclidean, truncated, floored: \
         {measured:?}"
    );
    assert_eq!(measured, CIRCUITS, "(firings, depth) of each");
}

/// The pinned measures, in the order [`the_circuit_of_each_operation`] asks.
const CIRCUITS: [(usize, usize); 14] = [
    (28, 21),
    (22, 17),
    (59, 52),
    (50, 43),
    (33, 33),
    (142, 136),
    (342, 330),
    (413, 406),
    (124, 120),
    (112, 108),
    (124, 120),
    (279, 269),
    (259, 249),
    (279, 269),
];

// ── the module's text ──────────────────────────────────────────────

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/int.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.int parses");
    Module { ast, forms }
}

impl Module {
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

    fn keyword(&self, key: NodeId) -> Option<&str> {
        match self.ast.node(key) {
            Node::Keyword(k) => Some(self.ast.sym(k)),
            _ => None,
        }
    }

    fn conjuncts(&self, clause: NodeId) -> Vec<NodeId> {
        match self.ast.head_name(clause) {
            Some("and") => self.ast.form_args(clause).to_vec(),
            Some(h) if h == ein_core::text::EMPTY_HEAD => Vec::new(),
            _ => vec![clause],
        }
    }

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

/// **The rule of the theory, on the text**, in its strongest form: every
/// rule of `std.int` but the closing rule has exactly **one** premise. It
/// reads no table of `std.digits` itself and joins nothing: all it needs
/// travels in the call's tag. A second premise that looks a fact up by a
/// number would change no answer and make every batch cost the square of its
/// count (E-3 of P1f.6's `findings.md`).
#[test]
fn every_rule_has_one_premise() {
    let m = module();
    let rules = m.declared("rule");
    assert!(rules.len() >= 30, "std.int parsed to {} rules", rules.len());
    for (name, rule) in &rules {
        let premises = m.conjuncts(m.clause(*rule, "match"));
        if name == "int-arith" {
            assert!(premises.is_empty(), "`int-arith` fires at root, on nothing");
            continue;
        }
        assert_eq!(
            premises.len(),
            1,
            "rule {name} has {} premises: a rule of std.int has ONE, and carries \
             what it needs in the call's tag",
            premises.len()
        );
    }
}

/// No `absent`, no `not`, no `or`, no guard, and no `(false)`: the module is
/// monotone, has one stratum, and never refutes. A contradiction in a
/// program of integers is `std.arith`'s, a division by zero: since S1f.6.22
/// the module derives that theory's flag, `(division-by-zero REQUEST)`, and
/// `std.arith`'s rule concludes `(false)` from it.
#[test]
fn the_theory_writes_no_negation_and_no_guard() {
    let m = module();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        m.heads(m.clause(rule, "assert"), &mut heads);
        for banned in ["absent", "not", "or", "neq", "false"] {
            assert!(
                !heads.iter().any(|h| h == banned),
                "rule {name} writes `{banned}`"
            );
        }
        // `eq` is the order's name, a value: it may not head a form.
        assert!(
            !heads.iter().any(|h| h == "eq"),
            "rule {name} writes an `eq` guard"
        );
    }
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.int guesses nothing and expands nothing"
    );
}

/// `int-arith` is the theory's one entry. It closes every relation the
/// module declares, names every other rule, and names `arith`, `std.arith`'s
/// entry, without which a selection leaves the lower theory's rules behind
/// in silence (E-20 of P1f.6's `findings.md`).
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "int-arith")
        .expect("the closing rule, `int-arith`");
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
            Some("int-rule") => assert!(named.insert(arg.clone()), "named twice: {arg}"),
            other => panic!("`int-arith` asserts a {other:?}"),
        }
    }
    let want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        closed, want_closed,
        "`int-arith` closes every relation the module declares"
    );
    let mut want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "int-arith")
        .collect();
    want_named.insert("arith".to_string());
    assert_eq!(
        named, want_named,
        "`int-arith` names every other rule of the module, and `arith`"
    );

    // The module imports `std.arith` by its entry and nothing else.
    let imports: Vec<String> = m
        .forms
        .iter()
        .filter(|&&f| m.ast.head_name(f) == Some("import"))
        .map(|&f| {
            m.ast
                .atom_name(m.ast.form_args(f)[0])
                .expect("a module")
                .to_string()
        })
        .collect();
    assert_eq!(imports, ["std.arith"]);

    // Its own name is a written fact, the module's only one (E-11).
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
        [("int-rule".to_string(), vec!["int-arith".to_string()])],
        "the module writes one fact, the closing rule's own name"
    );
}
