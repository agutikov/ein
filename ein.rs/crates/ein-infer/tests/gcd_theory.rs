//! **`std.math.gcd`, held to Rust's integers and to the rule of the theory**
//! ([04-stdlib.md](../../../../docs/ein/user/04-reference/04-stdlib.md), Q-M1f.336).
//!
//! `stdlib/math/gcd.ein` answers `(gcd X Y)` by `(gcd-of X Y G)` and `(lcm X
//! Y)` by `(lcm-of X Y L)`, on natural numbers of `std.math.decimal-arith`, by
//! Euclid's chain over that theory's tagged division. Ein has no numbers, so
//! what holds the module's answers is here.
//!
//! | claim | where |
//! |---|---|
//! | both requests over **every pair from 0 to [`BOUND`]** are right | [`every_pair_to_a_bound_is_right`] |
//! | requests over operands of one to nineteen digits, on a sample, and on the neighbours of the Fibonacci numbers, where the chain is longest, are right | [`long_operands_are_right`] |
//! | **zero in each place**, written `z` or with digits, and gcd(0, 0) = 0 and lcm(0, 0) = 0 (Q-M1f.299) | [`zero_in_each_place`] |
//! | equal operands, written alike or differently | [`equal_operands`] |
//! | an operand written with high zeros is read by value | [`high_zeros_are_read_by_value`] |
//! | a request may name its operands | [`named_operands_work_for_both_requests`] |
//! | the tagged call, from a rule of a program's own, with high zeros | [`the_tagged_call_answers_under_its_tag`] |
//! | **exactly one answer a request, in normal form, and no division by zero**, whatever the operands | all of them, through [`hold`] |
//! | an operand's fault is flagged on the request, and `(false)` | [`an_operands_fault_is_flagged_on_the_request`] |
//! | an integer or a rational is no operand here, and is answered by silence | [`what_is_not_an_operand`] |
//! | what one request costs as a circuit: its firings and its depth | [`the_circuit_of_each_request`] |
//! | **the rule of the theory**, on the module's text: every rule has one premise | [`every_rule_has_one_premise`] |
//! | no `absent`, `not`, `or`, guard or `(false)`, and no rule of the module derives `division-by-zero` | [`the_theory_writes_no_negation_and_no_guard`] |
//! | the closing rule closes every relation and names every rule, and the entries below | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//!
//! The oracle is Rust's `u128` and Euclid's algorithm, written out below. The
//! *measured* half of the rule of the theory is `arith_batch.rs`; the
//! search-free claim, on the binary, is `ein-cli/tests/arith_search_free.rs`;
//! the call's shape is `arith_theory.rs`' `the_tagged_calls_keep_their_shapes`.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// The exhausting set: every pair of numbers from 0 to `BOUND`, both ways
/// round, under both requests.
const BOUND: u128 = 30;

/// The requests of one saturation.
const CHUNK: usize = 250;

const IMPORT: &str = "(import std.math.gcd :symbols (gcd))\n";

// ── numbers, and how the theory writes them ────────────────────────

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

/// A string of decimal digits as a number of `std.math.decimal-arith`, high
/// zeros kept.
fn written(digits: &str) -> String {
    let mut out = String::from("z");
    for ch in digits.chars() {
        out = format!("(n {out} {ch})");
    }
    out
}

/// A number in normal form: no high zero, and zero `z`.
fn normal(v: u128) -> String {
    if v == 0 {
        "z".to_string()
    } else {
        written(&v.to_string())
    }
}

/// An operand as written, and its value.
type Operand = (String, u128);

fn value(v: u128) -> Operand {
    (normal(v), v)
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

    /// A number of exactly `digits` digits, its top digit not zero.
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

/// Saturate `IMPORT` + `body` and return every fact of each of `relations`,
/// as text, in their order.
fn run_all(body: &str, relations: &[&str]) -> Vec<Vec<String>> {
    let src = ein_corpus::listed(&format!("{IMPORT}{body}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<gcd>")).expect("the requests parse");
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
    relations
        .iter()
        .map(|relation| {
            let Some(rel) = ein_corpus::spellings(relation)
                .iter()
                .find_map(|n| terms.syms.get(n))
            else {
                return Vec::new();
            };
            kb.facts_of(rel)
                .map(|f| ein_corpus::unprefixed(&sexpr(&terms, f)))
                .collect()
        })
        .collect()
}

fn run(body: &str, relation: &str) -> Vec<String> {
    run_all(body, &[relation]).remove(0)
}

/// One request over two numbers: its request and result relations, and what
/// Rust's integers say the result's last argument is.
struct Op {
    request: &'static str,
    result: &'static str,
    answer: fn(u128, u128) -> u128,
}

const OPS: [Op; 2] = [
    Op {
        request: "gcd",
        result: "gcd-of",
        answer: gcd,
    },
    Op {
        request: "lcm",
        result: "lcm-of",
        answer: lcm,
    },
];

/// Ask every pair, [`CHUNK`] requests a program, and hold every answer:
/// **exactly one** result fact per request, repeating the operands as
/// written, its last argument what the integers say, in normal form. No
/// program derives `division-by-zero` or `(false)`: no request of the module
/// divides by zero.
fn hold(op: &Op, cases: &[(Operand, Operand)]) {
    let mut want: BTreeMap<(String, String), String> = BTreeMap::new();
    for ((x, a), (y, b)) in cases {
        want.insert((x.clone(), y.clone()), normal((op.answer)(*a, *b)));
    }
    let requests: Vec<(&(String, String), &String)> = want.iter().collect();
    for chunk in requests.chunks(CHUNK) {
        let body: String = chunk
            .iter()
            .map(|((x, y), _)| format!("({} {x} {y})\n", op.request))
            .collect();
        let [answers, flags, falses] =
            <[Vec<String>; 3]>::try_from(run_all(&body, &[op.result, "division-by-zero", "false"]))
                .expect("three relations");
        assert!(
            flags.is_empty() && falses.is_empty(),
            "a request of std.math.gcd divides by zero: {flags:?} {falses:?}"
        );
        let mut got: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        for fact in answers {
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
}

// ── right, held to Rust's integers ─────────────────────────────────

/// The exhausting set: 31 numbers, 961 pairs a request, zero on either side
/// and on both, every pair both ways round, equal operands, and every pair
/// whose one operand divides the other.
#[test]
fn every_pair_to_a_bound_is_right() {
    let mut cases = Vec::new();
    for a in 0..=BOUND {
        for b in 0..=BOUND {
            cases.push((value(a), value(b)));
        }
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

/// Longer operands: one to eight digits a side in every combination of
/// lengths, a few of nineteen, whose product `u128` still holds, pairs that
/// share a common factor of twelve digits, and neighbours of the Fibonacci
/// numbers, on which Euclid's chain is longest (Lamé): F(41) and F(40) take
/// thirty-nine steps.
#[test]
fn long_operands_are_right() {
    let mut rng = Rng(0x5146_0632);
    let mut cases = Vec::new();
    for la in 1..=8u32 {
        for lb in 1..=8u32 {
            let (a, b) = (rng.number(la), rng.number(lb));
            cases.push((value(a), value(b)));
        }
    }
    for _ in 0..4 {
        let (a, b) = (rng.number(19), rng.number(19));
        cases.push((value(a), value(b)));
    }
    // A common factor of twelve digits, times two of six.
    for _ in 0..4 {
        let (k, a, b) = (rng.number(12), rng.number(6), rng.number(6));
        cases.push((value(k * a), value(k * b)));
    }
    let mut fib = vec![1u128, 1];
    while fib.len() < 41 {
        let n = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(n);
    }
    for i in [10usize, 25, 40] {
        cases.push((value(fib[i]), value(fib[i - 1])));
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

/// **Zero in each place** (Q-M1f.299, the stage's default): gcd(X, 0) and
/// gcd(0, X) are X, gcd(0, 0) is zero; lcm with a zero is zero, lcm(0, 0)
/// too. Zero is written `z`, `(n z 0)` and `(n (n z 0) 0)`; the other operand
/// with and without high zeros.
#[test]
fn zero_in_each_place() {
    let zeros: Vec<Operand> = vec![("z".to_string(), 0), (written("0"), 0), (written("00"), 0)];
    let others: Vec<Operand> = vec![
        value(1),
        value(7),
        value(10),
        value(1071),
        (written("007"), 7),
        (written("0100"), 100),
    ];
    let mut cases = Vec::new();
    for zero in &zeros {
        for x in &others {
            cases.push((zero.clone(), x.clone()));
            cases.push((x.clone(), zero.clone()));
        }
        for other in &zeros {
            cases.push((zero.clone(), other.clone()));
        }
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

/// Equal operands: gcd(X, X) and lcm(X, X) are X, whether the two are
/// written alike or one with high zeros.
#[test]
fn equal_operands() {
    let mut cases = Vec::new();
    for v in [1u128, 2, 9, 10, 12, 99, 1071, 123_456_789] {
        cases.push((value(v), value(v)));
        cases.push((value(v), (written(&format!("00{v}")), v)));
        cases.push(((written(&format!("0{v}")), v), value(v)));
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

/// An operand written with high zeros is read by value, and the answer is
/// in normal form: the chain's last divisor is never an operand as written.
#[test]
fn high_zeros_are_read_by_value() {
    let mut rng = Rng(0x5146_0633);
    let mut cases = Vec::new();
    for _ in 0..60 {
        let (la, lb) = (1 + rng.below(4) as u32, 1 + rng.below(4) as u32);
        let (a, b) = (rng.number(la), rng.number(lb));
        let za = "0".repeat(rng.below(3) as usize);
        let zb = "0".repeat(rng.below(3) as usize);
        cases.push((
            (written(&format!("{za}{a}")), a),
            (written(&format!("{zb}{b}")), b),
        ));
    }
    // One divides the other: the answer is an operand's value, and never the
    // operand as written.
    for (a, b) in [(6u128, 12u128), (12, 6), (5, 5), (3, 300)] {
        cases.push((
            (written(&format!("00{a}")), a),
            (written(&format!("0{b}")), b),
        ));
    }
    for op in &OPS {
        hold(op, &cases);
    }
}

/// A request may name an operand, through `std.math.decimal-arith`'s `let`,
/// and the answer repeats the name.
#[test]
fn named_operands_work_for_both_requests() {
    let mut rng = Rng(0x5146_0634);
    let mut body = String::new();
    let mut names = Vec::new();
    for i in 0..10 {
        let len = 1 + rng.below(3) as u32;
        let v = if i == 0 { 0 } else { rng.number(len) };
        body.push_str(&format!("(let c{i} {})\n", normal(v)));
        names.push((format!("c{i}"), v));
    }
    for op in &OPS {
        let mut want = BTreeMap::new();
        let mut src = body.clone();
        for (x, a) in &names {
            for (y, b) in names.iter().step_by(3) {
                let answer = normal((op.answer)(*a, *b));
                // By two names, and by a name and a number written out.
                src.push_str(&format!("({} {x} {y})\n", op.request));
                want.insert((x.clone(), y.clone()), answer.clone());
                src.push_str(&format!("({} {} {x})\n", op.request, normal(*b)));
                want.insert((normal(*b), x.clone()), answer);
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
        assert!(run(&src, "false").is_empty(), "{} by name", op.request);
    }
}

/// **The tagged call** (Q-M1f.300): `(gcd-call TAG X Y)` from a rule of a
/// program's own, its numbers high zeros and all, answered under the tag by
/// `(gcd-done TAG G)`, G in normal form, one a call; and no request's result
/// is derived from a call.
#[test]
fn the_tagged_call_answers_under_its_tag() {
    let pairs: [(&str, &str, u128); 6] = [
        ("1071", "462", 21),
        ("0012", "018", 6),
        ("0", "05", 5),
        ("00", "0", 0),
        ("7", "7", 7),
        ("462", "1071", 21),
    ];
    let mut body =
        String::from("(relation pair Thing :why \"the pair {?1}\")\n(__closed__ pair)\n");
    let mut want = BTreeSet::new();
    for (i, (x, y, g)) in pairs.iter().enumerate() {
        body.push_str(&format!(
            "(gcd-call (pair p{i}) {} {})\n",
            written(x),
            written(y)
        ));
        want.insert(format!("(gcd-done (pair p{i}) {})", normal(*g)));
    }
    let [done, of, flags] =
        <[Vec<String>; 3]>::try_from(run_all(&body, &["gcd-done", "gcd-of", "division-by-zero"]))
            .expect("three relations");
    assert_eq!(done.into_iter().collect::<BTreeSet<_>>(), want);
    assert!(of.is_empty(), "a call is no request: {of:?}");
    assert!(flags.is_empty(), "{flags:?}");
}

/// **An operand's fault is flagged on the request** (as `std.math.int`'s and
/// `std.math.rat`'s): a digit, the digit 0, or a name no `let`
/// states, in either place of either request, is `std.math.decimal-arith`'s
/// flag on the request as written, and `(false)`; the request is never
/// answered.
#[test]
fn an_operands_fault_is_flagged_on_the_request() {
    let fine = "(n z 6)";
    for op in &OPS {
        for (bad, flag) in [
            ("7", "malformed"),
            ("0", "malformed"),
            ("foo", "undeclared"),
        ] {
            for request in [
                format!("({} {bad} {fine})", op.request),
                format!("({} {fine} {bad})", op.request),
            ] {
                let body = format!("{request}\n");
                let [flags, falses, answers] =
                    <[Vec<String>; 3]>::try_from(run_all(&body, &[flag, "false", op.result]))
                        .expect("three relations");
                let want = if flag == "malformed" {
                    format!("(malformed {request})")
                } else {
                    format!("(undeclared foo {request})")
                };
                assert_eq!(flags, [want], "{request}");
                assert_eq!(falses, ["(false)"], "{request}");
                assert!(answers.is_empty(), "{request} is flagged and answered");
            }
        }
    }
}

/// **Natural numbers only** (the stage: no gcd of integers or rationals): an
/// integer or a rational where a number goes is answered by silence, as
/// `std.math.decimal-arith`'s requests answer one, and the request beside it
/// is answered.
#[test]
fn what_is_not_an_operand() {
    let body = "(gcd (int (n z 4) z) (n z 6))\n(lcm (n z 6) (rat (int (n z 1) z) (n z 2)))\n\
                (gcd (n z 4) (n z 6))\n";
    let src = format!(
        "(import std.math.int :symbols (int))\n(import std.math.rat :symbols (rat))\n{body}"
    );
    let [of, lcm_of, falses] =
        <[Vec<String>; 3]>::try_from(run_all(&src, &["gcd-of", "lcm-of", "false"]))
            .expect("three relations");
    assert_eq!(of, ["(gcd-of (n z 4) (n z 6) (n z 2))"]);
    assert!(lcm_of.is_empty(), "{lcm_of:?}");
    assert!(falses.is_empty(), "{falses:?}");
}

// ── the circuit's measure ──────────────────────────────────────────

/// Firings and depth of one request, as `arith_theory.rs` measures them; the
/// four closing rules' firings are left out.
fn circuit(body: &str) -> (usize, usize) {
    let src = ein_corpus::listed(&format!("{IMPORT}{body}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<gcd>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closers: Vec<_> = ["decimal-arith", "gcd", "decimal", "arith"]
        .iter()
        .map(|entry| {
            ein_corpus::spellings(entry)
                .iter()
                .find_map(|n| terms.syms.get(n))
                .unwrap_or_else(|| panic!("the closing rule {entry}"))
        })
        .collect();
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

/// **Each request as a circuit.** A step of Euclid's chain is a long
/// division, which computes its divisor's nine multiples first; the least
/// common multiple is the chain, one product and one division more. The
/// firings count `std.math.decimal`'s check of each numeral the program
/// writes, as every circuit of the calculator's does.
///
/// `cargo test -p ein-infer --test gcd_theory the_circuit -- --nocapture`
/// re-takes them.
#[test]
fn the_circuit_of_each_request() {
    let n = |v: u128| normal(v);
    let measured = [
        circuit(&format!("(gcd {} {})\n", n(1071), n(462))),
        circuit(&format!("(gcd {} {})\n", n(462), n(1071))),
        circuit(&format!("(lcm {} {})\n", n(1071), n(462))),
        circuit(&format!("(gcd {} {})\n", n(5), n(6))),
        circuit(&format!("(gcd {} z)\n", n(7))),
        circuit("(gcd z z)\n"),
        circuit("(lcm z z)\n"),
        circuit(&format!("(gcd {} {})\n", n(102_334_155), n(63_245_986))),
    ];
    eprintln!(
        "gcd 1071 462, gcd 462 1071, lcm 1071 462, gcd 5 6, gcd 7 0, gcd 0 0, lcm 0 0, \
         gcd F(40) F(39): {measured:?}"
    );
    assert_eq!(measured, CIRCUITS, "(firings, depth) of each");
}

/// The pinned measures, in the order [`the_circuit_of_each_request`] asks.
const CIRCUITS: [(usize, usize); 8] = [
    (553, 553),
    (742, 742),
    (866, 866),
    (257, 257),
    (17, 17),
    (14, 14),
    (14, 14),
    (10337, 10337),
];

// ── the module's text ──────────────────────────────────────────────

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/math/gcd.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.math.gcd parses");
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
/// rule of `std.math.gcd` but the closing rule has exactly **one** premise. It
/// reads no table and joins nothing: all it needs travels in the call's tag
/// (the index does not key a nested fact, so a second premise looked up by a
/// number would make a batch cost the square of its count).
#[test]
fn every_rule_has_one_premise() {
    let m = module();
    let rules = m.declared("rule");
    assert!(
        rules.len() >= 18,
        "std.math.gcd parsed to {} rules",
        rules.len()
    );
    for (name, rule) in &rules {
        let premises = m.conjuncts(m.clause(*rule, "match"));
        if name == "gcd" {
            assert!(
                premises.is_empty(),
                "`gcd`, the closing rule, fires at root, on nothing"
            );
            continue;
        }
        assert_eq!(
            premises.len(),
            1,
            "rule {name} has {} premises: a rule of std.math.gcd has ONE, and carries \
             what it needs in the call's tag",
            premises.len()
        );
    }
}

/// No `absent`, no `not`, no `or`, no guard and no `(false)`: the module is
/// monotone, has one stratum of its own, and divides by nothing that may be
/// zero, so no rule of it derives `std.math.arith`'s flag. An operand's fault
/// goes to `std.math.decimal-arith`'s `bad-request`, whose rules flag it.
#[test]
fn the_theory_writes_no_negation_and_no_guard() {
    let m = module();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        m.heads(m.clause(rule, "assert"), &mut heads);
        for banned in [
            "absent",
            "not",
            "or",
            "neq",
            "eq",
            "false",
            "division-by-zero",
        ] {
            assert!(
                !heads.iter().any(|h| h == banned),
                "rule {name} writes `{banned}`"
            );
        }
    }
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.math.gcd guesses nothing and expands nothing"
    );
}

/// `gcd` is the module's one entry. It closes every relation the module
/// declares, names every other rule, and names the entries of the modules it
/// imports, `decimal-arith`, `decimal` and `arith`. The module writes no fact
/// of its own.
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "gcd")
        .expect("the closing rule, `gcd`");
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
            Some("gcd-rule") => assert!(named.insert(arg.clone()), "named twice: {arg}"),
            other => panic!("`gcd` asserts a {other:?}"),
        }
    }
    let want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        closed, want_closed,
        "`gcd` closes every relation the module declares"
    );
    let mut want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "gcd")
        .collect();
    for entry in ["decimal-arith", "decimal", "arith"] {
        want_named.insert(entry.to_string());
    }
    assert_eq!(
        named, want_named,
        "`gcd` names every other rule of the module, and the entries below"
    );
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
    assert_eq!(
        imports,
        [
            "std.math.decimal-arith",
            "std.math.decimal",
            "std.math.arith"
        ]
    );
    assert!(
        m.forms
            .iter()
            .all(|&f| matches!(m.ast.head_name(f), Some("relation" | "rule" | "import"))),
        "the module writes no fact"
    );
}
