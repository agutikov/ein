//! **`std.math.decimal-arith`, held to the integers and to its own rule** — M1f
//! [S1f.6.3](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md).
//!
//! `stdlib/math/decimal-arith.ein` is school arithmetic written *in* Ein, over the digit
//! tables of `std.math.digits`. Ein has no numbers, so the engine cannot say
//! whether a sum is right, and a program under `tests/stdlib/math/decimal-arith/` can
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
//! | no `not`, no `or`, and **exactly one `absent`, over `let`**, which no rule derives; `eq` and `neq` in the checking rules alone; three rules conclude `(false)`, each from its flag | [`the_theory_writes_one_absent_over_let`] |
//! | the closing rule closes every relation and names every rule | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//! | what one request costs as a circuit: its firings and its depth | [`the_circuit_of_one_addition`], [`the_circuit_of_one_subtraction_and_one_comparison`], [`the_circuit_of_one_multiplication`], [`the_circuit_of_one_division`] |
//! | a division by zero derives the flag, and the flag `(false)`, each by its own rule, and never a number | [`a_division_by_zero_is_flagged_and_false`] |
//! | **a division by zero is flagged at the call, whoever made it** (S1f.6.24): `(div-call TAG X Y)` with a Y that is zero by value derives `(division-by-zero (div-call TAG X Y))` and `(false)`, under every shape of tag; a public request holds its flag and its call's | [`a_tagged_call_by_zero_is_flagged_whoever_made_it`] |
//! | division's two families of nine rules are alike but for the digit, and the ten rules from an operand's tag to its request but for the operation | [`the_rules_of_a_family_differ_by_their_digit_alone`], [`the_rules_from_a_tag_to_its_request_are_alike`] |
//! | **a malformed request has exactly one flag and no answer**, and the flag derives `(false)`; a well-formed one has no flag (S1f.6.12) | [`a_malformed_request_is_flagged_once_and_never_answered`], [`an_undeclared_name_is_flagged_and_a_let_answers_it`], [`a_let_that_cannot_be_read_is_flagged`], [`what_stays_silent`] |
//! | **the tagged calls are an interface** (S1f.6.30): `std.math.decimal-arith`'s, `std.math.int`'s and `std.math.rat`'s are declared with the shapes of [`TAGGED`], their headers and `stdlib/README.md` write them so, and every caller in the stdlib, `examples/` and `tests/` writes them so | [`the_tagged_calls_keep_their_shapes`] |
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
//! S1f.6.5, division at S1f.6.6. S1f.6.12 added the check of what a program
//! wrote, and restated the static tests: guards, a probe, a second join on a
//! name, one `absent` and three rules that conclude `(false)`, each named
//! here by a constant.

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

const IMPORT: &str = "(import std.math.decimal-arith :symbols (decimal-arith))\n";

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
    let src = ein_corpus::listed(&format!("{IMPORT}{body}"));
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
    let Some(rel) = ein_corpus::spellings(relation)
        .iter()
        .find_map(|n| terms.syms.get(n))
    else {
        return Vec::new();
    };
    kb.facts_of(rel)
        .map(|f| ein_corpus::unprefixed(&sexpr(&terms, f)))
        .collect()
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
/// Since S1f.6.24 the request's call is flagged beside it,
/// `(division-by-zero (div-call (quotient X Y) U V))`, U and V the numbers the
/// operands stand for: **two flags for one public division**, exactly.
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
            let number = |o: &String| {
                if o == "none" {
                    "z".to_string()
                } else {
                    o.clone()
                }
            };
            want.insert(format!(
                "(division-by-zero (div-call (quotient {x} {y}) {} {}))",
                number(x),
                number(y)
            ));
        }
        body.push_str(&format!("(div {x} some)\n(div {x} (n z 3))\n"));
    }
    let flags: BTreeSet<String> = run(&body, "division-by-zero").into_iter().collect();
    assert_eq!(
        flags, want,
        "the flags are the requests that divide by zero, and the call of each"
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

/// **A division by zero is flagged at the call, whoever made it** (S1f.6.24;
/// the user's ruling of 2026-11-12: *"(div ?x 0) must emit division-by-zero
/// no matter where it happens"*). Until the stage a tagged call with a zero
/// divisor derived nothing, and each theory over this one wrote a rule of its
/// own.
///
/// The calls here are written as facts, which the theory computes with as
/// with a derived one. The tags are every shape the modules and the examples
/// use: a bare name; `std.math.decimal-arith`'s own; `std.math.int`'s, its request and the two
/// signs; `std.math.rat`'s three; and a tag of a program's own that holds numbers.
/// Each call with a divisor that is zero by value, `z` or written with
/// digits, has exactly one flag, the call as asserted, and no answer; each
/// call with another divisor is answered and not flagged.
#[test]
fn a_tagged_call_by_zero_is_flagged_whoever_made_it() {
    let tags = [
        "mine".to_string(),
        "(quotient (n z 7) nobody)".into(),
        "(int-div-m (int-div-of (int-div-trunc a b)) pos neg (n z 3))".into(),
        "(rat-gcd-rem (rat-g (rat-add a b) pos (n z 6) (n z 4)) (n z 4))".into(),
        "(rat-q-num (rat-add a b) neg (n z 4) (n z 2))".into(),
        "(rat-q-den (rat-add a b) pos (n z 3))".into(),
        "(tr-div (n (n z 1) 7) (n z 0))".into(),
    ];
    let zeros = ["z".to_string(), written("0"), written("000")];
    let dividends = ["z".to_string(), written("7"), written("120")];
    for tag in &tags {
        let mut body = String::new();
        let mut want = BTreeSet::new();
        for x in &dividends {
            for y in &zeros {
                // The tag holds the divisor too, so that each call has a tag
                // of its own, as a caller's has.
                body.push_str(&format!("(div-call (at {tag} {x} {y}) {x} {y})\n"));
                want.insert(format!(
                    "(division-by-zero (div-call (at {tag} {x} {y}) {x} {y}))"
                ));
            }
            body.push_str(&format!("(div-call (at {tag} {x} three) {x} (n z 3))\n"));
        }
        let flags: BTreeSet<String> = run(&body, "division-by-zero").into_iter().collect();
        assert_eq!(flags, want, "the flags are the calls that divide by zero");
        assert_eq!(run(&body, "false"), ["(false)"], "the flag derives (false)");
        let answers = run(&body, "div-done");
        assert_eq!(
            answers.len(),
            dividends.len(),
            "the calls that do not divide by zero are each answered: {answers:?}"
        );
        for fact in &answers {
            assert!(fact.contains(" three) "), "a call by zero answered: {fact}");
        }
        // The tag itself, bare, as a caller with one division writes it.
        // `std.math.decimal-arith`'s own tag is a public request's, so its call has the
        // request's flag beside it, whoever wrote the call.
        let bare = format!("(div-call {tag} (n z 7) z)\n");
        let mut want = vec![format!("(division-by-zero (div-call {tag} (n z 7) z))")];
        if tag.starts_with("(quotient ") {
            want.insert(0, "(division-by-zero (div (n z 7) nobody))".into());
        }
        assert_eq!(run(&bare, "division-by-zero"), want);
        assert_eq!(run(&bare, "false"), ["(false)"]);
    }
    // No flag and no contradiction without a zero divisor.
    let fine = "(div-call mine (n z 7) (n z 3))\n(div-call (other) z (n z 3))\n";
    assert!(run(fine, "division-by-zero").is_empty());
    assert!(run(fine, "false").is_empty());
    assert_eq!(run(fine, "div-done").len(), 2);
}

// ── malformed requests (S1f.6.12) ──────────────────────────────────

/// A well-formed operand that is not zero, to stand beside a malformed one.
const FINE: &str = "(n z 5)";

/// Every flag, answer and contradiction of one program: the facts of
/// `malformed`, of `undeclared`, of `division-by-zero` and of `false`, and
/// the requests that have a result fact, as `(REQUEST X Y)`.
struct Outcome {
    malformed: BTreeSet<String>,
    undeclared: BTreeSet<String>,
    division_by_zero: Vec<String>,
    contradiction: Vec<String>,
    answered: BTreeSet<String>,
}

fn outcome(body: &str) -> Outcome {
    let mut answered = BTreeSet::new();
    for op in &OPS {
        for fact in run(body, op.result) {
            let p = parts(&fact);
            answered.insert(format!("({} {} {})", op.request, p[1], p[2]));
        }
    }
    Outcome {
        malformed: run(body, "malformed").into_iter().collect(),
        undeclared: run(body, "undeclared").into_iter().collect(),
        division_by_zero: run(body, "division-by-zero"),
        contradiction: run(body, "false"),
        answered,
    }
}

/// **A malformed request is flagged once, and never answered** (S1f.6.12,
/// the user's rulings of 2026-11-11 and 2026-11-12). Every operation, either
/// operand, every kind of the set the investigation called b1:
///
/// - a digit's place holding a name that is no digit, at the units and a
///   place up, `z` and the bound `10` among the names, or holding a fact;
/// - a number that ends in a digit where `z` goes, or in a named constant;
/// - a digit where a number goes;
/// - the same through a name: a `let` whose number is malformed, and a name
///   that stands for another name.
///
/// Each derives `(malformed REQUEST)`, the request as written, and nothing
/// else flags; the flag derives `(false)`; and no malformed request has a
/// result fact. The well-formed requests of the same program are answered
/// and not flagged: the saturator runs root to its fixpoint whatever it
/// derives.
#[test]
fn a_malformed_request_is_flagged_once_and_never_answered() {
    let lets = "(let price (n z 5))\n(let broken (n (n z x) 1))\n(let alias price)\n";
    let bad = [
        "(n (n z 1) x)",
        "(n (n z x) 1)",
        "(n (n (n z 4) z) 2)",
        "(n z 10)",
        "(n z (n z 1))",
        "(n (n z (pair 1 2)) 3)",
        "(n 1 2)",
        "(n 0 2)",
        "(n (n 3 1) 2)",
        "(n price 1)",
        "(n (n price 0) 1)",
        "7",
        "0",
        "broken",
        "alias",
    ];
    for op in &OPS {
        let mut body = String::from(lets);
        let mut want = BTreeSet::new();
        for b in bad {
            for request in [
                format!("({} {b} {FINE})", op.request),
                format!("({} {FINE} {b})", op.request),
                format!("({} {b} {b})", op.request),
            ] {
                body.push_str(&request);
                body.push('\n');
                want.insert(format!("(malformed {request})"));
            }
        }
        // Beside them, requests the theory can read.
        let fine = [
            format!("({} {FINE} {FINE})", op.request),
            format!("({} price {FINE})", op.request),
            format!("({} (n (n z 0) 7) price)", op.request),
        ];
        for request in &fine {
            body.push_str(request);
            body.push('\n');
        }
        let got = outcome(&body);
        assert_eq!(
            got.malformed, want,
            "{}: the flags are the malformed requests, each once",
            op.request
        );
        assert!(
            got.undeclared.is_empty() && got.division_by_zero.is_empty(),
            "{}: no other flag: {:?} {:?}",
            op.request,
            got.undeclared,
            got.division_by_zero
        );
        assert_eq!(
            got.contradiction,
            ["(false)"],
            "{}: the flag derives (false)",
            op.request
        );
        assert_eq!(
            got.answered,
            fine.iter().cloned().collect::<BTreeSet<_>>(),
            "{}: the requests answered are the well-formed ones, and no malformed one",
            op.request
        );
    }
}

/// **A name no `let` states is undeclared** (the user's reply of 2026-11-12
/// to round 2's item 5, (b)): `(undeclared NAME REQUEST)`, the request as
/// written, and `(false)` from it; one flag a request, for the operand that
/// is looked up first; and never `(malformed …)`. **The same program with
/// the `let`s stated is answered and has no flag**: the premise is an
/// `absent` over `let`, which nothing derives, so the one state there is
/// has either the `let` or the flag.
#[test]
fn an_undeclared_name_is_flagged_and_a_let_answers_it() {
    for op in &OPS {
        let r = op.request;
        let requests = format!(
            "({r} foo {FINE})\n({r} {FINE} foo)\n({r} foo bar)\n({r} foo foo)\n({r} known foo)\n"
        );
        let known = "(let known (n (n z 1) 2))\n";
        let got = outcome(&format!("{known}{requests}({r} known {FINE})\n"));
        let want: BTreeSet<String> = [
            format!("(undeclared foo ({r} foo {FINE}))"),
            format!("(undeclared foo ({r} {FINE} foo))"),
            format!("(undeclared foo ({r} foo bar))"),
            format!("(undeclared foo ({r} foo foo))"),
            format!("(undeclared foo ({r} known foo))"),
        ]
        .into_iter()
        .collect();
        assert_eq!(got.undeclared, want, "{r}: one flag a request");
        assert!(got.malformed.is_empty(), "{r}: {:?}", got.malformed);
        assert_eq!(got.contradiction, ["(false)"], "{r}");
        assert_eq!(
            got.answered,
            [format!("({r} known {FINE})")].into_iter().collect(),
            "{r}: an undeclared request is never answered"
        );

        // The `let`s stated: every request answered, nothing flagged.
        let stated = format!("{known}(let foo (n z 7))\n(let bar (n z 3))\n{requests}");
        let got = outcome(&stated);
        assert!(
            got.undeclared.is_empty() && got.malformed.is_empty() && got.contradiction.is_empty(),
            "{r}: a `let` removes the flag: {:?} {:?}",
            got.undeclared,
            got.malformed
        );
        assert_eq!(got.answered.len(), 5, "{r}: each request is answered");
    }
}

/// **A `let` that cannot be read** is flagged as the `let`, whether or not a
/// request uses the name: a name stated twice with two numbers, each of the
/// two; and `z` or a digit as a constant's name. Until S1f.6.12 each gave
/// two answers to one request. A name stated twice with one number is one
/// fact, and fine.
#[test]
fn a_let_that_cannot_be_read_is_flagged() {
    let body = "(let price (n z 5))\n(let price (n z 7))\n\
                (let same (n z 4))\n(let same (n z 4))\n\
                (let z (n z 5))\n(let 3 (n z 5))\n(let 0 (n z 5))\n\
                (let fine (n (n z 1) 0))\n";
    let got = outcome(body);
    let want: BTreeSet<String> = [
        "(malformed (let price (n z 5)))",
        "(malformed (let price (n z 7)))",
        "(malformed (let z (n z 5)))",
        "(malformed (let 3 (n z 5)))",
        "(malformed (let 0 (n z 5)))",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    assert_eq!(got.malformed, want);
    assert_eq!(got.contradiction, ["(false)"]);
    assert!(got.undeclared.is_empty());
    // Each alone is a contradiction, and a program without one is none.
    for (one, flags) in [
        ("(let price (n z 5))\n(let price (n z 7))\n", 2),
        ("(let z (n z 5))\n", 1),
        ("(let 3 (n z 5))\n", 1),
        ("(let 0 (n z 5))\n", 1),
    ] {
        let got = outcome(one);
        assert_eq!(got.malformed.len(), flags, "{one}");
        assert_eq!(got.contradiction, ["(false)"], "{one}");
    }
    let got = outcome("(let same (n z 4))\n(let same (n z 4))\n(add same same)\n");
    assert!(got.malformed.is_empty() && got.contradiction.is_empty());
    assert_eq!(got.answered.len(), 1);
}

/// **What stays silent** (the user's reply 6 (a) of 2026-11-12: no general
/// report of unanswered requests; *a flag means malformed, no flag does not
/// mean well-formed*). Each of these is neither flagged nor answered, and
/// the module's header and `stdlib/README.md` name them:
///
/// - a number that ends in a name that is neither a digit nor a constant;
/// - a fact of another relation where a number goes, as an operand or
///   inside a number, the theory's own accumulator among them;
/// - a name that stands for such a thing.
///
/// A test, so that the list on the page is the list the theory has: a rule
/// that starts flagging one of them fails here, and the page is then wrong.
#[test]
fn what_stays_silent() {
    let silent = [
        "(n end 1)",
        "(n (n end 1) 2)",
        "(pair 1 2)",
        "(n (pair 1 2) 3)",
        "(b 1 z)",
        "odd",
    ];
    for op in &OPS {
        let mut body = String::from("(let odd (pair 1 2))\n");
        for x in silent {
            body.push_str(&format!("({} {x} {FINE})\n", op.request));
            body.push_str(&format!("({} {FINE} {x})\n", op.request));
        }
        let got = outcome(&body);
        assert!(
            got.malformed.is_empty()
                && got.undeclared.is_empty()
                && got.division_by_zero.is_empty()
                && got.contradiction.is_empty(),
            "{}: flagged: {:?} {:?}",
            op.request,
            got.malformed,
            got.undeclared
        );
        assert!(
            got.answered.is_empty(),
            "{}: {:?}",
            op.request,
            got.answered
        );
    }
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
    let src = ein_corpus::listed(&format!("{IMPORT}{body}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<arith>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closer = terms
        .syms
        .get("decimal-arith")
        .expect("the closing rule's name");
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
/// answer: 21 firings, one chain, every call carrying its continuation.
/// **And the check of its operands** (S1f.6.12), a firing a place, six and
/// four: 31 firings. The check runs beside the walk and hands nothing on, so
/// the depth is the chain's, 21, as it was. Every pin of the four tests
/// below is its chain and the digits of the operands that are found.
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
        (31, 21),
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
        (37, 27),
        "999999999 + 1: (firings, depth)"
    );
    // Named operands cost what numbers cost, a lookup and a check either way,
    // and each `let` one firing that says its name is one.
    assert_eq!(
        circuit(&format!("(let a {six})\n(let b {four})\n(add a b)\n")),
        (33, 21),
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
        (38, 21),
        "123078 - 4567: (firings, depth)"
    );
    assert_eq!(
        circuit(&format!("(sub {four} {six})\n")),
        (38, 21),
        "4567 - 123078: (firings, depth)"
    );
    // Equal numbers: both walks end with no borrow, each drops its six
    // zeros and hands over zero, and the two answer rules say the one fact.
    assert_eq!(
        circuit(&format!("(sub {six} {six})\n")),
        (47, 20),
        "123078 - 123078: (firings, depth)"
    );
    assert_eq!(
        circuit(&format!("(cmp {six} {four})\n")),
        (23, 13),
        "123078 against 4567: (firings, depth)"
    );
}

/// **One multiplication, as a circuit**, and how it grows with the two
/// lengths. The multiplier is turned round, a firing a digit; then each of
/// its digits is a row, a walk the length of the multiplicand with its strip
/// and turn, and, after the first, an addition walk the length of the sum so
/// far. So the size grows as the product of the two lengths, and the circuit
/// is still one chain: its depth is its size less the check of the two
/// operands, a firing a digit (S1f.6.12).
#[test]
fn the_circuit_of_one_multiplication() {
    let mul = |x: &str, y: &str| circuit(&format!("(mul {} {})\n", written(x), written(y)));
    assert_eq!(mul("123", "456"), (78, 72), "123 x 456: (firings, depth)");
    assert_eq!(
        mul("123078", "123078"),
        (239, 227),
        "123078 x 123078: (firings, depth)"
    );
    // The two lengths: a multiplier of one digit is one row; each digit more
    // is a row and an addition.
    assert_eq!(mul("999999", "9"), (35, 28), "six digits by one");
    assert_eq!(mul("9", "999999"), (135, 128), "one digit by six");
    assert_eq!(mul("999999", "999999"), (250, 238), "six digits by six");
    assert_eq!(
        mul("999999999999", "999999"),
        (388, 370),
        "twelve digits by six"
    );
    assert_eq!(
        mul("999999", "999999999999"),
        (574, 556),
        "six digits by twelve"
    );
}

/// **One division, as a circuit.** 127645 divided by 25: the divisor's nine
/// multiples, each a row of two columns; the dividend's six digits turned
/// round; then a step a digit, each three trials, and a trial a subtraction
/// with its strip and turn where it fits. One chain, so its depth is its
/// size less the check of the two operands, six firings and two (S1f.6.12).
/// A divisor of one digit and a dividend of one are the two ends.
#[test]
fn the_circuit_of_one_division() {
    let div = |x: &str, y: &str| circuit(&format!("(div {} {})\n", written(x), written(y)));
    assert_eq!(
        div("127645", "25"),
        (228, 220),
        "127645 / 25: (firings, depth)"
    );
    assert_eq!(div("7", "3"), (86, 84), "7 / 3");
    assert_eq!(div("999999", "7"), (236, 229), "six digits by one");
    assert_eq!(div("999999999999", "7"), (407, 394), "twelve digits by one");
    assert_eq!(
        div("999999999999", "777777"),
        (673, 655),
        "twelve digits by six"
    );
    // A division by zero stops at the divisor's normal form: the operands,
    // one row, its strip, the flag and the contradiction; and the dividend's
    // six places are checked beside it. Since S1f.6.24 the call is flagged
    // beside the request and is a contradiction too: two firings more, no
    // deeper.
    assert_eq!(div("127645", ""), (18, 10), "127645 / 0");
}

// ── the module's text ──────────────────────────────────────────────

/// The seven tables of `std.math.digits`: a premise over one is a lookup by digits.
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

/// The second rule that joins two facts, **on a name** as the first does
/// (S1f.6.12): a constant named twice is two `let`s of one name, which the
/// fact index finds by the name; the numbers are compared by a guard and
/// looked up by nothing.
const NAME_TWICE: (&str, [&str; 3]) = ("let-twice", ["let", "let", "neq"]);

/// A premise that is **one probe**: a relation of one argument, asked with
/// its argument bound by the premise that holds the numbers, which the
/// engine answers by one interned lookup whatever the argument is, a number
/// too (E-3's note). `nz` is a table and is held as one; `named` is the
/// theory's own.
const PROBES: [&str; 1] = ["named"];

/// The computed relations: a guard compares two values and looks nothing up.
const GUARDS: [&str; 2] = ["eq", "neq"];

/// The rules that write a guard, and no other may (S1f.6.12): the ten
/// digits in `check-digit`; *a name, not `z` and not a digit* in
/// `operand-undeclared`; two different numbers in `let-twice`.
const GUARDED: [&str; 3] = ["check-digit", "operand-undeclared", "let-twice"];

/// The one rule that reads an `absent`, and the relation it is over, which a
/// program writes and no rule of the module derives.
const ABSENT: (&str, &str) = ("operand-undeclared", "let");

/// The rules that conclude `(false)`, each with the one premise it has: its
/// flag.
const FALSE_RULES: [(&str, &str); 3] = [
    ("division-by-zero-is-false", "division-by-zero"),
    ("malformed-is-false", "malformed"),
    ("undeclared-is-false", "undeclared"),
];

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/math/decimal-arith.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.math.decimal-arith parses");
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

    /// A variable's name, without its `?`.
    fn var(&self, node: NodeId) -> Option<&str> {
        match self.ast.node(node) {
            Node::Var(v) => Some(self.ast.sym(v)),
            _ => None,
        }
    }

    /// Every variable at any depth under `node`.
    fn vars<'a>(&'a self, node: NodeId, out: &mut Vec<&'a str>) {
        out.extend(self.var(node));
        if matches!(self.ast.node(node), Node::SForm { .. }) {
            for &a in self.ast.form_args(node) {
                self.vars(a, out);
            }
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
/// premise that is not a row of `std.math.digits`; that premise holds the numbers,
/// and a table row holds none: its arguments are digits, names or variables,
/// never a form. The two exceptions join on a **name** and are named in
/// [`NAME_LOOKUP`] and [`NAME_TWICE`].
///
/// Since S1f.6.12 a rule may also hold guards, which compare and look
/// nothing up, and whose arguments are variables or names, never a form; a
/// probe of [`PROBES`], on a variable the state premise binds; and, in the
/// one rule [`ABSENT`] names, the `absent`, which
/// [`the_theory_writes_one_absent_over_let`] holds.
#[test]
fn no_rule_joins_two_facts_on_a_number() {
    let m = module();
    let rules = m.declared("rule");
    assert!(
        rules.len() >= 18,
        "std.math.decimal-arith parsed to {} rules",
        rules.len()
    );
    let mut exceptions_seen = 0;
    for (name, rule) in &rules {
        let mut state = Vec::new();
        if let Some((_, want)) = [NAME_LOOKUP, NAME_TWICE].iter().find(|(n, _)| n == name) {
            let premises: Vec<String> = m
                .conjuncts(m.clause(*rule, "match"))
                .iter()
                .map(|&p| m.ast.head_name(p).unwrap_or("?").to_string())
                .collect();
            assert_eq!(
                premises, want,
                "rule {name}: the stated exception has these premises in this order"
            );
            exceptions_seen += 1;
            continue;
        }
        let premises = m.conjuncts(m.clause(*rule, "match"));
        for &premise in &premises {
            let head = m
                .ast
                .head_name(premise)
                .unwrap_or_else(|| panic!("rule {name}: a premise with no relation"))
                .to_string();
            if GUARDS.contains(&head.as_str()) {
                for &arg in m.ast.form_args(premise) {
                    assert!(
                        !matches!(m.ast.node(arg), Node::SForm { .. }),
                        "rule {name}: a guard compares variables and names, and this one \
                         holds a form"
                    );
                }
            } else if head == "absent" {
                assert_eq!(
                    name, ABSENT.0,
                    "rule {name} reads an `absent`: {} alone does",
                    ABSENT.0
                );
            } else if PROBES.contains(&head.as_str()) {
                // One argument, a variable, and the premise that holds the
                // numbers comes first and binds it.
                let args = m.ast.form_args(premise);
                let var = match args {
                    [a] => m.var(*a),
                    _ => None,
                }
                .unwrap_or_else(|| panic!("rule {name}: `{head}` is asked of one variable"));
                let first = premises[0];
                let mut bound = Vec::new();
                m.vars(first, &mut bound);
                assert!(
                    first != premise && bound.contains(&var),
                    "rule {name}: `({head} ?{var})` is one probe only where the first \
                     premise binds ?{var}"
                );
            } else if TABLES.contains(&head.as_str()) {
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
            "rule {name} joins {state:?}: a rule of std.math.decimal-arith has ONE premise \
             that holds numbers, and its other premises are rows of std.math.digits. \
             A second one changes no answer and makes every batch of requests \
             cost the square of its count (E-3 of P1f.6's findings.md); carry \
             what the rule needs in the call's tag instead"
        );
    }
    assert_eq!(
        exceptions_seen, 2,
        "an exception this file names is no longer in the module: {NAME_LOOKUP:?}, \
         {NAME_TWICE:?}"
    );
}

/// **One `absent`, over `let`; no `not`, no `or`** (S1f.6.12; until then the
/// module wrote no `absent` at all). The one is `operand-undeclared`'s: an
/// operand that is a name and that no `let` states. `let` is a relation a
/// program writes and no rule of the module concludes, and the closing rule
/// closes it, so no commitment can add the fact the premise found absent:
/// the loader's *forbid* does not refuse the theory, the static check calls
/// a program that imports it monotone, and `ein saturate --dump`'s
/// `;; Strata` line says *stratified, 2 strata*, the rule that reads the
/// `absent` above the rest (`arith_search_free.rs` holds all three on the
/// binary).
///
/// **Guards in the checking rules alone:** `eq` and `neq` are computed and
/// monotone, and only the rules of [`GUARDED`] write one.
///
/// **Three rules conclude `(false)`**, each from its flag and nothing else:
/// a division by zero, a malformed request, an undeclared name.
#[test]
fn the_theory_writes_one_absent_over_let() {
    let m = module();
    let mut absents = Vec::new();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        let mut concluded = Vec::new();
        m.heads(m.clause(rule, "assert"), &mut concluded);
        for banned in ["not", "or"] {
            assert!(
                !heads.iter().chain(&concluded).any(|h| h == banned),
                "rule {name} writes `{banned}`"
            );
        }
        assert!(
            !concluded
                .iter()
                .any(|h| h == "absent" || GUARDS.contains(&h.as_str())),
            "rule {name} concludes a guard or an `absent`"
        );
        assert_eq!(
            heads.iter().any(|h| GUARDS.contains(&h.as_str())),
            GUARDED.contains(&name.as_str()),
            "the rules that write a guard are {GUARDED:?}: {name}"
        );
        // The `absent`s, each with the relation it is over.
        for premise in m.conjuncts(m.clause(rule, "match")) {
            if m.ast.head_name(premise) == Some("absent") {
                let inner = m.ast.form_args(premise);
                assert_eq!(inner.len(), 1, "rule {name}: an `absent` over one pattern");
                absents.push((
                    name.clone(),
                    m.ast.head_name(inner[0]).unwrap_or("?").to_string(),
                ));
            }
        }
        assert_eq!(
            heads.iter().filter(|h| *h == "absent").count(),
            usize::from(name == ABSENT.0),
            "rule {name}: `absent` is read by {} alone, once, as a premise of its own",
            ABSENT.0
        );
        // No rule derives the relation the `absent` is over: a conclusion's
        // own head, not a fact it holds, as `(malformed (let …))` does.
        assert!(
            !m.conjuncts(m.clause(rule, "assert"))
                .iter()
                .any(|&c| m.ast.head_name(c) == Some(ABSENT.1)),
            "rule {name} concludes `{}`, which the theory's `absent` is over: \
             a program writes it and no rule derives it",
            ABSENT.1
        );
        // `(false)` is concluded from a flag, by the flag's own rule.
        let flag = FALSE_RULES.iter().find(|(r, _)| *r == name);
        assert_eq!(
            concluded.iter().any(|h| h == "false"),
            flag.is_some(),
            "`(false)` is concluded by {FALSE_RULES:?} and by no other rule: {name}"
        );
        if let Some((_, flag)) = flag {
            let premises: Vec<&str> = m
                .conjuncts(m.clause(rule, "match"))
                .iter()
                .filter_map(|&p| m.ast.head_name(p))
                .collect();
            assert_eq!(premises, [*flag], "{name} reads its flag and nothing else");
        }
    }
    assert_eq!(
        absents,
        [(ABSENT.0.to_string(), ABSENT.1.to_string())],
        "the theory reads exactly one `absent`, over `{}`",
        ABSENT.1
    );
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.math.decimal-arith guesses nothing and expands nothing"
    );
}

/// `decimal-arith` is the theory's one entry. A relation it does not close is one the
/// search may guess facts of; a rule it does not name is one
/// `(import std.math.decimal-arith :symbols (decimal-arith))` leaves behind, in silence. Both are
/// easy to forget when a rule is added, and neither changes an answer the day
/// it is forgotten.
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "decimal-arith")
        .expect("the closing rule, `decimal-arith`");
    assert!(
        m.conjuncts(m.clause(*closer, "match")).is_empty(),
        "`decimal-arith` fires at root, on nothing"
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
            other => panic!("`decimal-arith` asserts a {other:?}"),
        }
    }

    // The tables the module imports, read off its `(import std.math.digits …)`.
    let mut tables = BTreeSet::new();
    for &f in &m.forms {
        if m.ast.head_name(f) != Some("import") {
            continue;
        }
        let args = m.ast.form_args(f);
        assert_eq!(
            m.ast.atom_name(args[0]),
            Some("std.math.digits"),
            "std.math.decimal-arith imports std.math.digits and nothing else"
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
        "the import lists tables of std.math.digits: {tables:?}"
    );

    let mut want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    want_closed.extend(tables);
    assert_eq!(
        closed, want_closed,
        "`decimal-arith` closes every relation the module declares and every table it imports"
    );
    let want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "decimal-arith")
        .collect();
    assert_eq!(
        named, want_named,
        "`decimal-arith` names every other rule of the module"
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
        [("arith-rule".to_string(), vec!["decimal-arith".to_string()])],
        "the module writes one fact, the closing rule's own name"
    );
}

/// **The ten rules from an operand's tag to its request are one rule
/// written ten times** (S1f.6.12): `bad-OP-first` and `bad-OP-second` for
/// each of the five operations, alike but for the operation's name, its
/// sign in the sentence, and the second tag's third argument.
#[test]
fn the_rules_from_a_tag_to_its_request_are_alike() {
    let path = repo_root().join("stdlib/math/decimal-arith.ein");
    let text = std::fs::read_to_string(&path).expect("std.math.decimal-arith");
    let rule = |name: &str| -> String {
        let start = text
            .find(&format!("(rule {name} ()"))
            .unwrap_or_else(|| panic!("no rule {name}"));
        let end = text[start..].find("\n\n").map_or(text.len(), |e| start + e);
        text[start..end].to_string()
    };
    for (op, sign) in [
        ("add", "+"),
        ("sub", "-"),
        ("cmp", "compared with"),
        ("mul", "x"),
        ("div", "divided by"),
    ] {
        for (place, tag) in [("first", "?x ?y"), ("second", "?x ?y ?u")] {
            let want = format!(
                "(rule bad-{op}-{place} ()\n  \
                 :match  (bad-operand ({op}-{place} {tag}) ?f)\n  \
                 :assert (bad-request ({op} ?x ?y) ?f)\n  \
                 :why    \"the {place} operand of {{?x}} {sign} {{?y}} is at fault: {{?f}}\")"
            );
            assert_eq!(rule(&format!("bad-{op}-{place}")), want);
        }
    }
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
    let path = repo_root().join("stdlib/math/decimal-arith.ein");
    let text = std::fs::read_to_string(&path).expect("std.math.decimal-arith");
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

// ── the tagged calls: an interface ─────────────────────────────────

/// **The tagged calls,** as each module's header and `stdlib/README.md` write
/// them (M1f S1f.6.30's T1; the user's *"(a)"* of 2026-11-12 to round 2's
/// item 41): `(module, as written, the declaration's types)`. As written, a
/// capital stands where a caller writes a value of its own, and a name where
/// every caller writes that name: a walk starts at `z`, a subtraction at `0`
/// and a comparison at `eq`; an addition's carry in is the caller's, a bit
/// in `examples/binary_arithmetic/conv.ein`. Each module's calls come first
/// and then the facts that answer them; `std.math.int`'s comparison is answered
/// by `std.math.decimal-arith`'s `order-is`.
const TAGGED: [(&str, &str, &str); 21] = [
    (
        "decimal-arith",
        "(add-walk TAG X Y C z)",
        "Tag Number Number Digit Digits",
    ),
    (
        "decimal-arith",
        "(sub-walk TAG X Y 0 z)",
        "Tag Number Number Digit Digits",
    ),
    (
        "decimal-arith",
        "(cmp-walk TAG X Y eq)",
        "Tag Number Number Order",
    ),
    ("decimal-arith", "(mul-call TAG X Y)", "Tag Number Number"),
    ("decimal-arith", "(div-call TAG X Y)", "Tag Number Number"),
    ("decimal-arith", "(operand TAG X)", "Tag Operand"),
    ("decimal-arith", "(done TAG S)", "Tag Number"),
    ("decimal-arith", "(short TAG)", "Tag"),
    ("decimal-arith", "(order-is TAG O)", "Tag Order"),
    ("decimal-arith", "(div-done TAG Q R)", "Tag Number Number"),
    ("decimal-arith", "(operand-is TAG N)", "Tag Number"),
    ("int", "(int-norm TAG P N)", "Tag Number Number"),
    (
        "int",
        "(int-add-call TAG P1 N1 P2 N2)",
        "Tag Number Number Number Number",
    ),
    (
        "int",
        "(int-mul-call TAG P1 N1 P2 N2)",
        "Tag Number Number Number Number",
    ),
    (
        "int",
        "(int-cmp-call TAG P1 N1 P2 N2)",
        "Tag Number Number Number Number",
    ),
    ("int", "(int-done TAG INTEGER)", "Tag Integer"),
    ("rat", "(rat-norm TAG P N D)", "Tag Number Number Number"),
    (
        "rat",
        "(rat-mul-call TAG P1 N1 D1 P2 N2 D2)",
        "Tag Number Number Number Number Number Number",
    ),
    (
        "rat",
        "(rat-cross TAG P1 N1 D1 P2 N2 D2)",
        "Tag Number Number Number Number Number Number",
    ),
    ("rat", "(rat-done TAG RATIONAL)", "Tag Rational"),
    (
        "rat",
        "(rat-cross-is TAG POSITIVE NEGATIVE)",
        "Tag Number Number",
    ),
];

/// A tagged call as written: its relation, and each argument's place, `None`
/// where a caller writes its own value and the name every caller writes.
fn tagged_shape(written: &str) -> (&str, Vec<Option<&str>>) {
    let inner = written.trim_start_matches('(').trim_end_matches(')');
    let mut words = inner.split_whitespace();
    let relation = words.next().expect("a relation");
    let places = words
        .map(|w| (!w.starts_with(|c: char| c.is_ascii_uppercase())).then_some(w))
        .collect();
    (relation, places)
}

/// Whether `text` writes `name` at the head of a form, bare or after a
/// qualified import's prefix: `(name ` or `.name ` and the like. A cheap
/// filter before a parse, and generous: what it lets through is parsed.
fn mentions(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(i, _)| {
        let before = text[..i].chars().next_back();
        let after = text[i + name.len()..].chars().next();
        matches!(before, Some('(' | '.')) && after.is_some_and(|c| c.is_whitespace() || c == ')')
    })
}

/// A relation declaration's name, and the rest of it as text: its types and
/// its `:why`, which is what says that two declarations are one.
fn declaration(ast: &Ast, form: NodeId) -> Option<(String, Vec<String>)> {
    if ast.head_name(form) != Some("relation") {
        return None;
    }
    let args = ast.form_args(form);
    let name = ast.atom_name(*args.first()?)?.to_string();
    let rest = args[1..]
        .iter()
        .map(|&a| ein_ir::dump_compact(ast, a))
        .collect();
    Some((name, rest))
}

/// **The tagged calls keep their shapes** (M1f S1f.6.30's T1). `std.math.decimal-arith`'s
/// five calls and their four answers and its operand look-up, and `std.math.int`'s and `std.math.rat`'s calls
/// with theirs, are what a theory built on the calculator calls: an
/// interface since the user's reply of 2026-11-12 to round 2's item 41. Held
/// three ways:
///
/// - **the declarations:** each module declares each of its rows of
///   [`TAGGED`] with the row's types, so a change of shape is a change of
///   this table;
/// - **the pages:** the module's header and `stdlib/README.md` write each
///   row as [`TAGGED`] does, so the page a caller reads is the shape held;
/// - **every caller:** every module of the stdlib and every program under
///   `examples/` and `tests/` that brings a module's declaration of a row,
///   flat or under a prefix, writes every form of that relation with the
///   row's count of arguments; and where a rule's conclusion, or a written
///   fact, *starts* a walk (a rule whose `:match` reads no fact of that walk),
///   it writes the row's names in their places, `z`, `0` or `eq`.
///
/// The loader refuses a form of a declared relation with another count of
/// arguments in a program it loads (S1f.6.23); this holds the count against
/// the interface, by name, in every file, and the start values, which no
/// loader sees. A file that does not parse, or whose imports do not resolve,
/// calls nothing and is not read.
#[test]
fn the_tagged_calls_keep_their_shapes() {
    let root = repo_root();
    let readme = std::fs::read_to_string(root.join("stdlib/README.md")).expect("stdlib/README.md");
    // ── the declarations, and the pages ──
    let mut declared: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for module in ["decimal-arith", "int", "rat"] {
        let path = root.join(format!("stdlib/math/{module}.ein"));
        let text = std::fs::read_to_string(&path).expect("a module");
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("a module parses");
        let decls: BTreeMap<String, Vec<String>> =
            forms.iter().filter_map(|&f| declaration(&ast, f)).collect();
        for &(m, written, types) in TAGGED.iter().filter(|t| t.0 == module) {
            let (relation, places) = tagged_shape(written);
            let rest = decls
                .get(relation)
                .unwrap_or_else(|| panic!("std.math.{m} declares no {relation}"));
            let got: Vec<&str> = rest
                .iter()
                .filter(|r| !r.starts_with(':'))
                .map(String::as_str)
                .collect();
            assert_eq!(
                got.join(" "),
                types,
                "std.math.{m}'s {relation} is declared with other types than the interface's"
            );
            assert_eq!(
                places.len(),
                got.len(),
                "{written}: its count against {types}"
            );
            assert!(
                text.contains(written),
                "std.math.{m}'s header does not write {written}"
            );
            assert!(
                readme.contains(written),
                "stdlib/README.md does not write {written}"
            );
            declared.insert(relation, rest.clone());
        }
    }
    // ── every caller ──
    let shape: BTreeMap<&str, (&str, Vec<Option<&str>>)> = TAGGED
        .iter()
        .map(|t| {
            let (relation, places) = tagged_shape(t.1);
            (relation, (t.1, places))
        })
        .collect();
    let mut bad: Vec<String> = Vec::new();
    let mut read: BTreeMap<String, usize> = BTreeMap::new();
    let mut started: BTreeMap<&str, usize> = BTreeMap::new();
    for path in ein_corpus::corpus_files() {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if !shape.keys().any(|r| mentions(&text, r)) {
            continue;
        }
        let file = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
            continue;
        };
        let Ok(resolved) = ein_ir::resolve_imports(&mut ast, &forms, path.parent()) else {
            continue;
        };
        // The interface as this program holds it: a relation it declares
        // whose name is a row's, bare or after a prefix, and whose
        // declaration is the module's.
        let mut ours: BTreeMap<String, &str> = BTreeMap::new();
        for &f in &resolved {
            let Some((name, rest)) = declaration(&ast, f) else {
                continue;
            };
            let bare = name.rsplit('.').next().unwrap_or(&name);
            if let Some((&relation, module_rest)) = declared.get_key_value(bare)
                && *module_rest == rest
            {
                ours.insert(name.clone(), relation);
            }
        }
        if ours.is_empty() {
            continue;
        }
        let mut seen = 0usize;
        let mut check = |ast: &Ast, node: NodeId, starts: bool, bad: &mut Vec<String>| {
            let Some(head) = ast.head_name(node) else {
                return;
            };
            let Some(&relation) = ours.get(head) else {
                return;
            };
            let (written, places) = &shape[relation];
            let args = ast.form_args(node);
            seen += 1;
            if args.len() != places.len() {
                bad.push(format!(
                    "{file}: {} has {} arguments where {written} has {}",
                    ein_ir::dump_compact(ast, node),
                    args.len(),
                    places.len()
                ));
                return;
            }
            if starts && places.iter().any(Option::is_some) {
                *started.entry(relation).or_default() += 1;
                for (&a, place) in args.iter().zip(places) {
                    if let Some(want) = place
                        && ast.atom_name(a) != Some(*want)
                    {
                        bad.push(format!(
                            "{file}: {} starts a walk with {} where {written} writes {want}",
                            ein_ir::dump_compact(ast, node),
                            ein_ir::dump_compact(ast, a)
                        ));
                    }
                }
            }
        };
        fn every(ast: &Ast, node: NodeId, out: &mut Vec<NodeId>) {
            match ast.node(node) {
                Node::SForm { .. } => {
                    out.push(node);
                    for &a in ast.form_args(node) {
                        every(ast, a, out);
                    }
                }
                Node::KwPair { value, .. } => every(ast, value, out),
                _ => {}
            }
        }
        for &f in &forms {
            let head = ast.head_name(f).unwrap_or("");
            if matches!(head, "import" | "relation" | "macro") {
                continue;
            }
            // What starts a walk: a written fact, or a conclusion of a rule
            // that reads no fact of the same walk.
            let mut starts: BTreeSet<NodeId> = BTreeSet::new();
            if matches!(head, "rule" | "hrule") {
                let mut read_heads: Vec<String> = Vec::new();
                for &a in ast.form_args(f) {
                    if let Node::KwPair { key, value } = ast.node(a)
                        && matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "match")
                    {
                        let mut nodes = Vec::new();
                        every(&ast, value, &mut nodes);
                        read_heads.extend(
                            nodes
                                .iter()
                                .filter_map(|&n| ast.head_name(n).map(str::to_string)),
                        );
                    }
                }
                for &a in ast.form_args(f) {
                    if let Node::KwPair { key, value } = ast.node(a)
                        && matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "assert")
                    {
                        let conjuncts = match ast.head_name(value) {
                            Some("and") => ast.form_args(value).to_vec(),
                            _ => vec![value],
                        };
                        starts.extend(conjuncts.into_iter().filter(|&c| {
                            ast.head_name(c)
                                .is_some_and(|h| !read_heads.iter().any(|r| r == h))
                        }));
                    }
                }
            } else if !matches!(head, "query" | "solve" | "config" | "trace") {
                starts.insert(f);
            }
            let mut nodes = Vec::new();
            every(&ast, f, &mut nodes);
            for n in nodes {
                check(&ast, n, starts.contains(&n), &mut bad);
            }
        }
        if seen > 0 {
            read.insert(file, seen);
        }
    }
    assert!(
        bad.is_empty(),
        "a tagged call out of its shape:\n{}",
        bad.join("\n")
    );
    // Not vacuous: the callers the pages name are read, and every walk is
    // started somewhere.
    for caller in [
        "stdlib/math/decimal-arith.ein",
        "stdlib/math/int.ein",
        "stdlib/math/rat.ein",
        "examples/beyond_arith/casting_out_nines.ein",
        "examples/binary_arithmetic/conv.ein",
    ] {
        assert!(read.contains_key(caller), "{caller} was not read: {read:?}");
    }
    for walk in ["add-walk", "sub-walk", "cmp-walk"] {
        assert!(
            started.get(walk).is_some_and(|&n| n > 0),
            "no start of {walk} was read"
        );
    }
    eprintln!(
        "the tagged calls: {} forms in {} files; walks started: {started:?}\n{read:#?}",
        read.values().sum::<usize>(),
        read.len()
    );
}
