//! **`std.rat`, held to exact fractions and to the rule of the theory** — M1f
//! [S1f.6.8](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md).
//!
//! `stdlib/rat.ein` is the calculator's second extension: a rational is
//! `(rat Z D)`, an integer of `std.int` over a natural number of `std.arith`
//! that is not zero, and its normal form is in lowest terms. The module calls
//! the two theories below it and edits neither. Ein has no numbers, so what
//! holds the theory's answers is here.
//!
//! | claim | where |
//! |---|---|
//! | every request of each operation in [`OPS`] over **every pair of the fractions with a numerator from −[`NUM`] to [`NUM`] and a denominator from 1 to [`DEN`]** is right | [`every_pair_of_small_fractions_is_right`] |
//! | requests over longer operands are right, on a sample, in every combination of signs | [`longer_fractions_are_right`] |
//! | an operand that is not in lowest terms, whose numerator is not simplified, or whose denominator has high zeros, answers as its normal form does | [`an_operand_is_read_by_value`] |
//! | simplification and negation are right | [`simplification_and_negation_are_right`] |
//! | a request of every operation may name its operands | [`named_operands_work_for_every_request`] |
//! | an integer or a natural number is not a rational operand | [`an_integer_is_not_a_rational_operand`] |
//! | **every answer is in lowest terms**, and each request has exactly one | all of them, through [`hold`] |
//! | **a zero denominator, and a division by zero, derive the flag and `(false)`**, the request as written, and never a number | [`a_zero_denominator_is_flagged_and_false`] |
//! | what one request costs as a circuit: its firings and its depth | [`the_circuit_of_each_operation`] |
//! | **the rule of the theory**, on the module's text: every rule has one premise | [`every_rule_has_one_premise`] |
//! | no `absent`, `not`, `or` or guard, and no rule of the module concludes `(false)` | [`the_theory_writes_no_negation_and_no_guard`] |
//! | the closing rule closes every relation and names every rule, and names the two entries below | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//!
//! The oracle is a pair of `i128` reduced by Euclid's algorithm, written out
//! below. The *measured* half of the rule of the theory is `arith_batch.rs`;
//! the search-free claim, on the binary, is
//! `ein-cli/tests/arith_search_free.rs`.
//!
//! **The sizes are small on purpose.** Lowest terms is a chain of divisions,
//! so one request of two one-digit fractions is some 450 firings, and a
//! stored fact costs about a kilobyte (E-26 of P1f.6's `findings.md`).
//! [`hold`] asks its cases in programs of [`CHUNK`] requests.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// The exhausting set: every fraction with a numerator from −`NUM` to `NUM`
/// over a denominator from 1 to [`DEN`], 28 spellings of 19 rationals,
/// against every other, under every operation.
const NUM: i128 = 3;
const DEN: i128 = 4;

/// The requests of one saturation.
const CHUNK: usize = 250;

const IMPORT: &str = "(import std.rat :symbols (rat-arith))\n";

// ── fractions, and how the theory writes them ──────────────────────

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

/// A fraction in lowest terms, its denominator positive: the oracle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Frac(i128, i128);

fn frac(n: i128, d: i128) -> Frac {
    assert!(d != 0);
    let g = gcd(n, d);
    let s = if d < 0 { -1 } else { 1 };
    Frac(s * n / g, s * d / g)
}

/// A string of decimal digits as a number of `std.arith`, high zeros kept.
fn written(digits: &str) -> String {
    let mut out = String::from("z");
    for ch in digits.chars() {
        out = format!("(n {out} {ch})");
    }
    out
}

fn natural(v: u128) -> String {
    if v == 0 {
        "z".to_string()
    } else {
        written(&v.to_string())
    }
}

/// A rational as written: P less N, over D.
fn spelt(p: u128, n: u128, d: u128) -> String {
    format!("(rat (int {} {}) {})", natural(p), natural(n), natural(d))
}

/// The normal form: lowest terms, the numerator with one part zero.
fn normal(f: Frac) -> String {
    if f.0 >= 0 {
        spelt(f.0.unsigned_abs(), 0, f.1.unsigned_abs())
    } else {
        spelt(0, f.0.unsigned_abs(), f.1.unsigned_abs())
    }
}

/// An operand as written, and its value.
type Operand = (String, Frac);

/// `n` over `d` as written, with `n`'s sign in the numerator's parts and
/// nothing reduced.
fn over(n: i128, d: i128) -> Operand {
    let text = if n >= 0 {
        spelt(n.unsigned_abs(), 0, d.unsigned_abs())
    } else {
        spelt(0, n.unsigned_abs(), d.unsigned_abs())
    };
    (text, frac(n, d))
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
    let forms = parse(&mut ast, &src, Some("<rat>")).expect("the requests parse");
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

/// One operation over two rationals: its request and result relations, and
/// what exact fractions say the result's last argument is, in the theory's
/// spelling; `None` where the request has no answer, a division by zero.
struct Op {
    request: &'static str,
    result: &'static str,
    answer: fn(Frac, Frac) -> Option<String>,
}

const OPS: [Op; 5] = [
    Op {
        request: "rat-add",
        result: "rat-sum-of",
        answer: |a, b| Some(normal(frac(a.0 * b.1 + b.0 * a.1, a.1 * b.1))),
    },
    Op {
        request: "rat-sub",
        result: "rat-diff-of",
        answer: |a, b| Some(normal(frac(a.0 * b.1 - b.0 * a.1, a.1 * b.1))),
    },
    Op {
        request: "rat-mul",
        result: "rat-product-of",
        answer: |a, b| Some(normal(frac(a.0 * b.0, a.1 * b.1))),
    },
    // A divisor that is zero has no answer and is a contradiction, so one
    // such request would kill the whole program.
    Op {
        request: "rat-div",
        result: "rat-quot-of",
        answer: |a, b| (b.0 != 0).then(|| normal(frac(a.0 * b.1, a.1 * b.0))),
    },
    Op {
        request: "rat-cmp",
        result: "rat-cmp-of",
        answer: |a, b| {
            Some(
                match (a.0 * b.1).cmp(&(b.0 * a.1)) {
                    std::cmp::Ordering::Less => "lt",
                    std::cmp::Ordering::Equal => "eq",
                    std::cmp::Ordering::Greater => "gt",
                }
                .to_string(),
            )
        },
    },
];

/// Ask every pair, [`CHUNK`] requests a program, and hold every answer:
/// **exactly one** result fact per request, repeating the operands as
/// written, its last argument what the fractions say, in lowest terms. No
/// program derives `(false)`.
fn hold(op: &Op, cases: &[(Operand, Operand)]) {
    let mut want: BTreeMap<(String, String), String> = BTreeMap::new();
    for ((x, a), (y, b)) in cases {
        if let Some(answer) = (op.answer)(*a, *b) {
            want.insert((x.clone(), y.clone()), answer);
        }
    }
    let requests: Vec<(&(String, String), &String)> = want.iter().collect();
    for chunk in requests.chunks(CHUNK) {
        let body: String = chunk
            .iter()
            .map(|((x, y), _)| format!("({} {x} {y})\n", op.request))
            .collect();
        let mut got: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        for fact in run(&body, op.result) {
            let p = parts(&fact);
            assert_eq!(p.len(), 4, "a {} of another arity: {fact}", op.result);
            got.entry((p[1].clone(), p[2].clone()))
                .or_default()
                .push(p[3].clone());
        }
        for (request, answer) in chunk {
            let answers = got.remove(*request).unwrap_or_default();
            assert_eq!(
                answers,
                std::slice::from_ref(*answer),
                "({} {} {}): the fractions say {answer}",
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
}

// ── right, held to exact fractions ─────────────────────────────────

/// The exhausting set: 28 operands, 784 pairs an operation, every
/// combination of signs, zero on either side, and every fraction that is not
/// in lowest terms, 2/4 beside 1/2 and 3/3 beside 1/1.
#[test]
fn every_pair_of_small_fractions_is_right() {
    let mut operands = Vec::new();
    for n in -NUM..=NUM {
        for d in 1..=DEN {
            operands.push(over(n, d));
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

/// Longer operands: numerators and denominators of one to four digits in the
/// four combinations of signs, a few of twelve digits, whose cross products
/// have twenty-four, and the neighbours of the Fibonacci numbers, on which
/// Euclid's chain is longest.
#[test]
fn longer_fractions_are_right() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0608);
        let mut cases = Vec::new();
        for la in 1..=4u32 {
            for lb in 1..=4u32 {
                let (a, b) = (rng.number(la) as i128, rng.number(lb) as i128);
                let (c, d) = (rng.number(lb) as i128, rng.number(la) as i128);
                let sign = |k: u64| if k == 0 { 1 } else { -1 };
                let (s, t) = (sign(rng.below(2)), sign(rng.below(2)));
                cases.push((over(s * a, b), over(t * c, d)));
            }
        }
        for _ in 0..3 {
            let (a, b, c, d) = (
                rng.number(12) as i128,
                rng.number(12) as i128,
                rng.number(12) as i128,
                rng.number(12) as i128,
            );
            cases.push((over(-a, b), over(c, d)));
        }
        let fib = [
            1i128, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987,
        ];
        for w in fib.windows(3) {
            cases.push((over(w[1], w[0]), over(-w[2], w[1])));
        }
        hold(op, &cases);
    }
}

/// The stage's task 3: *an unsimplified operand, such as 2/4, must answer as
/// 1/2*. Here an operand is spelt with a common factor in both terms, with a
/// common amount in both parts of its numerator, and with high zeros in its
/// denominator.
#[test]
fn an_operand_is_read_by_value() {
    for op in &OPS {
        let mut rng = Rng(0x5146_0618);
        let spell = |rng: &mut Rng, n: i128, d: i128| -> Operand {
            let k = 1 + rng.below(6) as i128;
            let extra = rng.below(8) as u128;
            let (p, m) = if n >= 0 {
                ((n * k).unsigned_abs() + extra, extra)
            } else {
                (extra, (n * k).unsigned_abs() + extra)
            };
            let zeros = "0".repeat(rng.below(3) as usize);
            let den = written(&format!("{zeros}{}", d * k));
            (
                format!("(rat (int {} {}) {den})", natural(p), natural(m)),
                frac(n, d),
            )
        };
        let mut cases = Vec::new();
        for _ in 0..120 {
            let (a, b) = (rng.below(13) as i128 - 6, 1 + rng.below(9) as i128);
            let (c, d) = (rng.below(13) as i128 - 6, 1 + rng.below(9) as i128);
            let (x, y) = (spell(&mut rng, a, b), spell(&mut rng, c, d));
            cases.push((x, y));
        }
        // The same rational, spelt differently on the two sides.
        for _ in 0..30 {
            let (a, b) = (rng.below(13) as i128 - 6, 1 + rng.below(9) as i128);
            let (x, y) = (spell(&mut rng, a, b), spell(&mut rng, a, b));
            cases.push((x, y));
        }
        hold(op, &cases);
    }
}

/// The two requests with one operand: `(rat-simplify X)` and `(rat-neg X)`.
#[test]
fn simplification_and_negation_are_right() {
    let mut rng = Rng(0x5146_0628);
    let mut operands: Vec<Operand> = Vec::new();
    for n in -12i128..=12 {
        for d in 1..=12i128 {
            operands.push(over(n, d));
        }
    }
    for l in 1..=10u32 {
        let (a, b) = (rng.number(l) as i128, rng.number(l) as i128);
        operands.push(over(a, b));
        operands.push(over(-a, b));
        // A common factor of the two terms' own size.
        let k = rng.number(l.min(6)) as i128;
        operands.push(over(a * k, b * k));
    }
    // A numerator that is not simplified, a denominator with high zeros, and
    // zero written every way.
    for (p, n, d, v) in [
        ("5", "3", "4", frac(2, 4)),
        ("3", "5", "04", frac(-2, 4)),
        ("7", "7", "9", frac(0, 1)),
        ("", "", "001", frac(0, 1)),
        ("00", "0", "12", frac(0, 1)),
        ("012", "", "0018", frac(2, 3)),
    ] {
        operands.push((
            format!("(rat (int {} {}) {})", written(p), written(n), written(d)),
            v,
        ));
    }
    type Unary = (&'static str, &'static str, fn(Frac) -> Frac);
    let unary: [Unary; 2] = [
        ("rat-simplify", "rat-simple-of", |v| v),
        ("rat-neg", "rat-neg-of", |v| Frac(-v.0, v.1)),
    ];
    for (request, result, f) in unary {
        let mut want: BTreeMap<String, String> = BTreeMap::new();
        for (x, v) in &operands {
            want.insert(x.clone(), normal(f(*v)));
        }
        let requests: Vec<(&String, &String)> = want.iter().collect();
        for chunk in requests.chunks(CHUNK) {
            let body: String = chunk
                .iter()
                .map(|(x, _)| format!("({request} {x})\n"))
                .collect();
            let mut got: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for fact in run(&body, result) {
                let p = parts(&fact);
                assert_eq!(p.len(), 3, "{fact}");
                got.entry(p[1].clone()).or_default().push(p[2].clone());
            }
            for (x, answer) in chunk {
                assert_eq!(
                    got.remove(*x).unwrap_or_default(),
                    std::slice::from_ref(*answer),
                    "({request} {x})"
                );
            }
            assert!(got.is_empty(), "{result}: answers nobody asked for");
        }
    }
}

/// A request may name an operand, through `std.arith`'s `let`, and the
/// answer repeats the name.
#[test]
fn named_operands_work_for_every_request() {
    let mut rng = Rng(0x5146_0638);
    let mut body = String::new();
    let mut names = Vec::new();
    for i in 0..12 {
        let (n, d) = (rng.below(41) as i128 - 20, 1 + rng.below(20) as i128);
        let (text, v) = over(n, d);
        body.push_str(&format!("(let c{i} {text})\n"));
        names.push((format!("c{i}"), v));
    }
    for op in &OPS {
        let mut want = BTreeMap::new();
        let mut src = body.clone();
        for (x, a) in &names {
            for (y, b) in names.iter().step_by(4) {
                let Some(answer) = (op.answer)(*a, *b) else {
                    continue;
                };
                // By two names, and by a name and a rational written out.
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
                got.insert((p[1].clone(), p[2].clone()), p[3].clone())
                    .is_none(),
                "two answers: {fact}"
            );
        }
        assert_eq!(got, want, "{} by name", op.request);
    }
}

/// The module's stated limit (Q-M1f.181): an operand of a rational request
/// is a rational, or a name of one. An integer or a natural number in an
/// operand's place is answered by silence, and the well-formed request
/// beside it is answered.
#[test]
fn an_integer_is_not_a_rational_operand() {
    let (five, half) = ("(int (n z 5) z)", normal(frac(1, 2)));
    let body = format!(
        "(let i5 {five})\n(let n5 (n z 5))\n\
         (rat-add {five} {half})\n(rat-add {half} (n z 5))\n(rat-add i5 {half})\n\
         (rat-add n5 {half})\n(rat-neg {five})\n(rat-mul {half} {five})\n\
         (rat-add {half} {half})\n"
    );
    assert_eq!(
        run(&body, "rat-sum-of"),
        [format!("(rat-sum-of {half} {half} {})", normal(frac(1, 1)))]
    );
    assert!(run(&body, "rat-neg-of").is_empty());
    assert!(run(&body, "rat-product-of").is_empty());
    assert!(run(&body, "division-by-zero").is_empty());
    assert!(run(&body, "false").is_empty());
}

// ── a zero denominator, and a division by zero ─────────────────────

/// **A zero denominator is a division by zero, in two steps**, as
/// `std.arith`'s is (the user's ruling of 2026-11-11). A request with an
/// operand whose denominator is zero, written `z`, with digits, or under a
/// name, and a `rat-div` by a rational that is zero, however that is
/// written, derive `(division-by-zero REQUEST)`, the request as written;
/// `std.arith`'s rule derives `(false)`; and the request is never answered.
/// The requests beside them are answered and not flagged.
///
/// **One flag a request, and it is the request's** (S1f.6.24). `std.arith`
/// flags a division at its tagged call since that stage, whoever made it.
/// None of this module's rules is replaced by it: a request with a
/// denominator of zero performs no division, and a `rat-div` by a rational
/// that is zero is seen at the divisor's numerator, before any call; the
/// module's own calls, Euclid's chain and lowest terms, divide by numbers
/// that are not zero. So the set below holds no `(division-by-zero
/// (div-call …))`, and the equality is exact.
///
/// The saturator here runs root to its fixpoint whatever it derives; what a
/// *solve* makes of a dead root is `ein-cli/tests/arith_search_free.rs`'s.
#[test]
fn a_zero_denominator_is_flagged_and_false() {
    let half = normal(frac(1, 2));
    let bad = [
        "(rat (int (n z 3) z) z)".to_string(),
        "(rat (int z (n z 3)) (n z 0))".to_string(),
        "(rat (int z z) (n (n z 0) 0))".to_string(),
        "nothing".to_string(),
    ];
    let zeros = [
        "(rat (int z z) (n z 5))".to_string(),
        "(rat (int (n z 4) (n z 4)) (n z 7))".to_string(),
        "(rat (int (n z 0) z) (n z 1))".to_string(),
        "nil".to_string(),
    ];
    let mut body =
        String::from("(let nothing (rat (int (n z 1) z) z))\n(let nil (rat (int z z) (n z 3)))\n");
    let mut want = BTreeSet::new();
    let binary = ["rat-add", "rat-sub", "rat-mul", "rat-div", "rat-cmp"];
    for x in &bad {
        for op in binary {
            // A zero denominator in the first place, in the second, in both.
            for request in [
                format!("({op} {x} {half})"),
                format!("({op} {half} {x})"),
                format!("({op} {x} {x})"),
            ] {
                body.push_str(&format!("{request}\n"));
                want.insert(format!("(division-by-zero {request})"));
            }
        }
        for op in ["rat-simplify", "rat-neg"] {
            body.push_str(&format!("({op} {x})\n"));
            want.insert(format!("(division-by-zero ({op} {x}))"));
        }
    }
    for y in &zeros {
        body.push_str(&format!("(rat-div {half} {y})\n"));
        want.insert(format!("(division-by-zero (rat-div {half} {y}))"));
        // A rational that is zero is an operand like any other of the rest.
        body.push_str(&format!("(rat-add {half} {y})\n(rat-mul {y} {half})\n"));
    }
    body.push_str(&format!("(rat-div {half} {half})\n"));
    let flags: BTreeSet<String> = run(&body, "division-by-zero").into_iter().collect();
    assert_eq!(flags, want, "the flags are the requests with no value");
    assert_eq!(run(&body, "false"), ["(false)"], "the flag derives (false)");
    let flagged = |fact: &str| {
        let p = parts(fact);
        bad.contains(&p[1]) || bad.contains(&p[2])
    };
    for result in [
        "rat-sum-of",
        "rat-diff-of",
        "rat-product-of",
        "rat-quot-of",
        "rat-cmp-of",
    ] {
        for fact in run(&body, result) {
            assert!(!flagged(&fact), "a request with no value answered: {fact}");
        }
    }
    assert!(run(&body, "rat-simple-of").is_empty());
    assert!(run(&body, "rat-neg-of").is_empty());
    assert_eq!(
        run(&body, "rat-quot-of"),
        [format!(
            "(rat-quot-of {half} {half} {})",
            normal(frac(1, 1))
        )],
        "the one division whose divisor is not zero"
    );
    assert_eq!(run(&body, "rat-sum-of").len(), zeros.len());
    assert_eq!(run(&body, "rat-product-of").len(), zeros.len());
    // No flag and no contradiction without one.
    let fine = format!("(rat-div {half} {half})\n(rat-add {half} {})\n", zeros[0]);
    assert!(run(&fine, "division-by-zero").is_empty());
    assert!(run(&fine, "false").is_empty());
}

// ── the circuit's measure ──────────────────────────────────────────

/// Firings and depth of one request, as `arith_theory.rs` measures them; the
/// three closing rules' firings are left out.
fn circuit(body: &str) -> (usize, usize) {
    let src = format!("{IMPORT}{body}");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<rat>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closers = [
        terms.syms.get("arith").expect("std.arith's closing rule"),
        terms.syms.get("int-arith").expect("std.int's closing rule"),
        terms.syms.get("rat-arith").expect("std.rat's closing rule"),
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

/// **Each operation as a circuit.** Lowest terms is most of every request
/// but a comparison: the numerator simplified, Euclid's chain, a division a
/// step, each computing its divisor's nine multiples first, and two more
/// divisions by what the chain finds. A comparison is the cross products
/// and one walk, and builds nothing.
///
/// `cargo test -p ein-infer --test rat_theory the_circuit` re-takes them.
#[test]
fn the_circuit_of_each_operation() {
    let r = |n: i128, d: i128| over(n, d).0;
    let measured = [
        circuit(&format!("(rat-simplify {})\n", r(2, 4))),
        circuit(&format!("(rat-simplify {})\n", r(1071, 462))),
        circuit(&format!("(rat-neg {})\n", r(-3, 4))),
        circuit(&format!("(rat-add {} {})\n", r(1, 2), r(1, 3))),
        circuit(&format!("(rat-sub {} {})\n", r(1, 2), r(1, 3))),
        circuit(&format!("(rat-mul {} {})\n", r(2, 3), r(3, 4))),
        circuit(&format!("(rat-div {} {})\n", r(1, 2), r(-3, 4))),
        circuit(&format!("(rat-cmp {} {})\n", r(1, 2), r(1, 3))),
        circuit(&format!("(rat-add {} {})\n", r(123, 456), r(789, 1011))),
        circuit(&format!("(rat-cmp {} {})\n", r(123, 456), r(789, 1011))),
    ];
    eprintln!(
        "simplify 2/4, simplify 1071/462, neg, add, sub, mul, div, cmp, \
         add 123/456 + 789/1011, cmp the same: {measured:?}"
    );
    assert_eq!(measured, CIRCUITS, "(firings, depth) of each");
}

/// The pinned measures, in the order [`the_circuit_of_each_operation`] asks.
const CIRCUITS: [(usize, usize); 10] = [
    (334, 332),
    (867, 862),
    (435, 433),
    (492, 490),
    (417, 415),
    (459, 457),
    (494, 490),
    (76, 76),
    (2640, 2633),
    (270, 270),
];

// ── the module's text ──────────────────────────────────────────────

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/rat.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.rat parses");
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
/// rule of `std.rat` but the closing rule has exactly **one** premise. It
/// reads no table itself and joins nothing: all it needs travels in the
/// call's tag (E-3 of P1f.6's `findings.md` is why).
#[test]
fn every_rule_has_one_premise() {
    let m = module();
    let rules = m.declared("rule");
    assert!(rules.len() >= 50, "std.rat parsed to {} rules", rules.len());
    for (name, rule) in &rules {
        let premises = m.conjuncts(m.clause(*rule, "match"));
        if name == "rat-arith" {
            assert!(premises.is_empty(), "`rat-arith` fires at root, on nothing");
            continue;
        }
        assert_eq!(
            premises.len(),
            1,
            "rule {name} has {} premises: a rule of std.rat has ONE, and carries \
             what it needs in the call's tag",
            premises.len()
        );
    }
}

/// No `absent`, no `not`, no `or` and no guard: the module is monotone and
/// has one stratum. It concludes no `(false)` either: a zero denominator
/// derives `std.arith`'s flag, by one of the four rules named here, and
/// `std.arith`'s rule derives the contradiction.
#[test]
fn the_theory_writes_no_negation_and_no_guard() {
    let m = module();
    let mut flaggers = BTreeSet::new();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        let mut concluded = Vec::new();
        m.heads(m.clause(rule, "assert"), &mut concluded);
        if concluded.iter().any(|h| h == "division-by-zero") {
            flaggers.insert(name.clone());
        }
        heads.extend(concluded);
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
    assert_eq!(
        flaggers.into_iter().collect::<Vec<_>>(),
        [
            "rat-div-zero",
            "rat-zero-first",
            "rat-zero-only",
            "rat-zero-second"
        ],
        "the rules that derive the flag"
    );
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.rat guesses nothing and expands nothing"
    );
}

/// `rat-arith` is the theory's one entry. It closes every relation the
/// module declares, names every other rule, and names `int-arith` and
/// `arith`, the entries of the theories below, without which a selection
/// leaves their rules behind in silence (E-20 of P1f.6's `findings.md`).
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "rat-arith")
        .expect("the closing rule, `rat-arith`");
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
            Some("rat-rule") => assert!(named.insert(arg.clone()), "named twice: {arg}"),
            other => panic!("`rat-arith` asserts a {other:?}"),
        }
    }
    let want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        closed, want_closed,
        "`rat-arith` closes every relation the module declares"
    );
    let mut want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "rat-arith")
        .collect();
    want_named.insert("arith".to_string());
    want_named.insert("int-arith".to_string());
    assert_eq!(
        named, want_named,
        "`rat-arith` names every other rule of the module, `int-arith` and `arith`"
    );

    // The module imports the two theories below it, each by its entry.
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
    assert_eq!(imports, ["std.arith", "std.int"]);

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
        [("rat-rule".to_string(), vec!["rat-arith".to_string()])],
        "the module writes one fact, the closing rule's own name"
    );
}
