//! **`std.math.binary-arith` and `std.math.convert`, held to the integers**
//! ([04-stdlib/14-binary.md](../../../../docs/ein/user/04-reference/04-stdlib/14-binary.md), Q-M1f.321).
//!
//! `stdlib/math/binary-arith.ein` is school arithmetic on binary numerals,
//! `(b2 PREFIX BIT)` of `std.math.binary`, over the four tables of
//! `std.math.bits`; most of its rules are `std.math.positional`'s, the rules
//! both bases share, activated with binary's relations. Ein has no numbers,
//! so what holds the theory's answers is here, as `arith_theory.rs` holds
//! decimal's; the rule of the theory on the text, the closing rules and the
//! macro's call are `arith_theory.rs`'s, where every module of the calculator
//! is read.
//!
//! | claim | where |
//! |---|---|
//! | every request of each operation in [`OPS`] over two numbers up to [`SMALL`] is right | [`every_small_request_is_right`] |
//! | requests over operands of lengths around every power of two up to the operation's longest are right, with equal operands and operands one apart | [`requests_of_many_lengths_are_right`] |
//! | a carry or a borrow runs through every column and out of the top, at every length | [`a_carry_or_a_borrow_runs_the_whole_length`] |
//! | operands written with high zeros are read by value | [`high_zeros_are_read_by_value`] |
//! | a request of every operation may name its operands, `bin-let` | [`named_operands_work_for_every_request`] |
//! | **every answer is in normal form**, and each request has exactly one | all of them, through [`hold`] |
//! | a division by zero derives `std.math.arith`'s flag for the request and for its call, and `(false)`, and no quotient | [`a_division_by_zero_is_flagged_and_false`] |
//! | **a malformed numeral is flagged where it is written, `std.math.binary`'s `(not-binary …)`**, a bit where a number goes and a `bin-let` that cannot be read are the request's and the `bin-let`'s `(bin-malformed …)`, a name no `bin-let` states `(bin-undeclared …)`; each derives `(false)` and none is answered | [`a_malformed_request_is_flagged_once_and_never_answered`], [`an_undeclared_name_is_flagged_and_a_let_answers_it`], [`a_let_that_cannot_be_read_is_flagged`] |
//! | what stays silent, as the module's header lists it | [`what_stays_silent`] |
//! | what one request costs as a circuit: its firings and its depth, at decimal's example operands | [`the_circuit_of_each_operation`] |
//! | **`std.math.convert`**: a number to binary and back is right against Rust's integers, every number to 300 and a sample to 2^40, each in normal form, and a round trip is the number | [`conversions_are_right_both_ways`] |
//! | **both bases in one program**: the same requests asked in decimal and in binary answer the same values | [`both_bases_in_one_program_agree`] |
//!
//! The oracle is `u128`; the sampled operands have up to 64 bits, 60 for a
//! product, so no answer leaves it. The sample is drawn by the fixed-seed
//! generator `arith_theory.rs` writes, so a failure names operands that fail
//! again. The *measured* half of the rule of the theory, a batch that costs in
//! proportion to its count, is `arith_batch.rs`, on both bases; the
//! search-free claim, on the binary, is `ein-cli/tests/arith_search_free.rs`.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, parse};

/// Every pair of operands from 0 to this is asked of every operation: 65 ×
/// 65 requests each, every row of the full adder's and the full subtractor's
/// tables and the steps from one bit to seven.
const SMALL: u128 = 64;

/// The longest sampled operand, in bits: a sum of two stays in `u128`.
const MAX_BITS: u32 = 64;

const IMPORT: &str = "(import std.math.binary-arith :symbols (binary-arith))\n";

// ── numbers as the theory writes them ──────────────────────────────

/// A string of bits as a number of the theory, in reading order: `""` is
/// `z`, `"0101"` is `(b2 (b2 (b2 (b2 z 0) 1) 0) 1)`, high zero kept.
fn written(bits: &str) -> String {
    let mut out = String::from("z");
    for ch in bits.chars() {
        out = format!("(b2 {out} {ch})");
    }
    out
}

/// The normal form of a value: no high zero, and zero is `z`.
fn normal(v: u128) -> String {
    if v == 0 {
        "z".to_string()
    } else {
        written(&format!("{v:b}"))
    }
}

/// The normal form of a value in decimal, `std.math.decimal`'s numeral.
fn decimal(v: u128) -> String {
    if v == 0 {
        return "z".to_string();
    }
    let mut out = String::from("z");
    for ch in v.to_string().chars() {
        out = format!("(n {out} {ch})");
    }
    out
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

    /// A number of exactly `bits` bits, its top bit 1; 0 for none.
    fn number(&mut self, bits: u32) -> u128 {
        if bits == 0 {
            return 0;
        }
        let low = (u128::from(self.next()) << 64) | u128::from(self.next());
        let mask = if bits >= 128 {
            u128::MAX
        } else {
            (1u128 << bits) - 1
        };
        (low & mask) | (1u128 << (bits - 1))
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

/// Saturate `import` + `body` and return every fact of each of `relations`,
/// as text, by relation.
fn run_all(import: &str, body: &str, relations: &[&str]) -> BTreeMap<String, Vec<String>> {
    let src = ein_corpus::listed(&format!("{import}{body}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<binary>")).expect("the requests parse");
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
            let facts = ein_corpus::spellings(relation)
                .iter()
                .find_map(|n| terms.syms.get(n))
                .map(|rel| {
                    kb.facts_of(rel)
                        .map(|f| ein_corpus::unprefixed(&sexpr(&terms, f)))
                        .collect()
                })
                .unwrap_or_default();
            (relation.to_string(), facts)
        })
        .collect()
}

/// Saturate `IMPORT` + `body` and return every fact of `relation`, as text.
fn run(body: &str, relation: &str) -> Vec<String> {
    run_all(IMPORT, body, &[relation])
        .remove(relation)
        .unwrap_or_default()
}

/// One operation of the theory: its request and result relations, the
/// longest operand it is sampled at, and what the integers say the result's
/// arguments after the operands are, each in the theory's spelling.
struct Op {
    request: &'static str,
    result: &'static str,
    max_bits: u32,
    /// `None` where the request has no answer: a division by zero.
    answer: fn(u128, u128) -> Option<Vec<String>>,
}

/// The five operations, as `arith_theory.rs`' rows, each name with `bin-`.
const OPS: [Op; 5] = [
    Op {
        request: "bin-add",
        result: "bin-sum-of",
        max_bits: MAX_BITS,
        answer: |a, b| Some(vec![normal(a + b)]),
    },
    Op {
        request: "bin-sub",
        result: "bin-diff-of",
        max_bits: MAX_BITS,
        answer: |a, b| {
            Some(vec![
                normal(a.saturating_sub(b)),
                normal(b.saturating_sub(a)),
            ])
        },
    },
    Op {
        request: "bin-cmp",
        result: "bin-cmp-of",
        max_bits: MAX_BITS,
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
    // A product of two numbers of 60 bits has at most 120, which `u128` holds.
    Op {
        request: "bin-mul",
        result: "bin-product-of",
        max_bits: 60,
        answer: |a, b| Some(vec![normal(a * b)]),
    },
    Op {
        request: "bin-div",
        result: "bin-quot-of",
        max_bits: MAX_BITS,
        answer: |a, b| (b != 0).then(|| vec![normal(a / b), normal(a % b)]),
    },
];

/// An operand as written, and its value.
type Operand = (String, u128);

fn value(v: u128) -> Operand {
    (normal(v), v)
}

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

/// The lengths sampled: every one to nine, and each side of every power of
/// two to 64, where a carry and a borrow change how far they run.
const LENGTHS: [u32; 24] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 15, 16, 17, 23, 31, 32, 33, 40, 47, 48, 59, 60, 63, 64,
];

#[test]
fn requests_of_many_lengths_are_right() {
    for op in &OPS {
        let mut rng = Rng(0x5146_1616);
        let lengths: Vec<u32> = LENGTHS
            .iter()
            .copied()
            .filter(|&l| l <= op.max_bits)
            .collect();
        let mut cases = Vec::new();
        for &la in &lengths {
            for &lb in &lengths {
                cases.push((value(rng.number(la)), value(rng.number(lb))));
            }
        }
        // Equal operands, and operands one apart: the difference of zero and
        // of one, and the order decided in the units.
        for &l in lengths.iter().filter(|&&l| l > 0) {
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
        let mut ones: u128 = 0;
        for _ in 1..=op.max_bits {
            ones = (ones << 1) | 1;
            // 11…1 and 1, both ways, and 11…1 with itself: a sum one bit
            // longer; 100…0 and 1, and 100…0 and 11…1: a borrow through
            // every zero.
            for (a, b) in [
                (ones, 1),
                (1, ones),
                (ones, ones),
                (ones + 1, 1),
                (1, ones + 1),
                (ones + 1, ones),
                (ones, ones + 1),
            ] {
                cases.push((value(a), value(b)));
            }
        }
        hold(op, &cases);
    }
}

#[test]
fn high_zeros_are_read_by_value() {
    for op in &OPS {
        let mut rng = Rng(0x5146_1617);
        let pad = |rng: &mut Rng, v: u128| {
            let zeros = "0".repeat(rng.below(4) as usize);
            let bits = if v == 0 {
                String::new()
            } else {
                format!("{v:b}")
            };
            (written(&format!("{zeros}{bits}")), v)
        };
        let mut cases = Vec::new();
        for _ in 0..600 {
            let (la, lb) = (rng.below(25) as u32, rng.below(25) as u32);
            let (a, b) = (rng.number(la), rng.number(lb));
            let (x, y) = (pad(&mut rng, a), pad(&mut rng, b));
            cases.push((x, y));
        }
        for _ in 0..100 {
            let l = rng.below(25) as u32;
            let a = rng.number(l);
            let (x, y) = (pad(&mut rng, a), pad(&mut rng, a));
            cases.push((x, y));
        }
        for zeros in ["0", "00", "000"] {
            cases.push(((written(zeros), 0), (written("111"), 7)));
            cases.push(((written("111"), 7), (written(zeros), 0)));
            cases.push(((written(zeros), 0), (written(zeros), 0)));
            cases.push(((written(zeros), 0), ("z".to_string(), 0)));
        }
        hold(op, &cases);
    }
}

/// A request may name an operand of every operation, and the answer repeats
/// the name: the same pairs, asked through `(bin-let NAME NUMBER)`.
#[test]
fn named_operands_work_for_every_request() {
    let mut rng = Rng(0x5146_1618);
    let mut body = String::new();
    let mut names = Vec::new();
    for i in 0..40 {
        let l = rng.below(30) as u32;
        let v = rng.number(l);
        body.push_str(&format!("(bin-let c{i} {})\n", normal(v)));
        names.push((format!("c{i}"), v));
    }
    for op in &OPS {
        let mut want = BTreeMap::new();
        let mut src = body.clone();
        for (x, a) in &names {
            for (y, b) in names.iter().step_by(7) {
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

/// **Division by zero, in two steps**, as in decimal: a request whose divisor
/// is zero, written `z`, with bits, or by a name, derives `std.math.arith`'s
/// flag for the request as written and for its call, and `(false)` from
/// them; and it is never answered with a number. Requests whose divisor is
/// not zero are not flagged.
#[test]
fn a_division_by_zero_is_flagged_and_false() {
    let zeros = ["z".to_string(), written("0"), written("000"), "none".into()];
    let dividends = [
        "z".to_string(),
        written("111"),
        written("1100"),
        "none".into(),
    ];
    let mut body = String::from("(bin-let none z)\n(bin-let some (b2 (b2 z 1) 1))\n");
    let mut want = BTreeSet::new();
    for x in &dividends {
        for y in &zeros {
            body.push_str(&format!("(bin-div {x} {y})\n"));
            want.insert(format!("(division-by-zero (bin-div {x} {y}))"));
            let number = |o: &String| {
                if o == "none" {
                    "z".to_string()
                } else {
                    o.clone()
                }
            };
            want.insert(format!(
                "(division-by-zero (bin-div-call (bin-quotient {x} {y}) {} {}))",
                number(x),
                number(y)
            ));
        }
        body.push_str(&format!(
            "(bin-div {x} some)\n(bin-div {x} (b2 (b2 z 1) 1))\n"
        ));
    }
    let flags: BTreeSet<String> = run(&body, "division-by-zero").into_iter().collect();
    assert_eq!(
        flags, want,
        "the flags are the requests that divide by zero, and the call of each"
    );
    assert_eq!(run(&body, "false"), ["(false)"], "the flag derives (false)");
    let answers = run(&body, "bin-quot-of");
    for fact in &answers {
        let p = parts(fact);
        assert!(
            !zeros.contains(&p[2]),
            "a division by zero answered with a number: {fact}"
        );
    }
    assert_eq!(answers.len(), 2 * dividends.len());
    let fine = "(bin-div (b2 (b2 (b2 z 1) 1) 1) (b2 (b2 z 1) 1))\n(bin-div z (b2 z 1))\n";
    assert!(run(fine, "division-by-zero").is_empty());
    assert!(run(fine, "false").is_empty());
}

// ── malformed requests ─────────────────────────────────────────────

/// A well-formed operand that is not zero, to stand beside a malformed one.
const FINE: &str = "(b2 (b2 z 1) 0)";

/// Every flag, answer and contradiction of one program.
struct Outcome {
    malformed: BTreeSet<String>,
    not_binary: BTreeSet<String>,
    undeclared: BTreeSet<String>,
    division_by_zero: Vec<String>,
    contradiction: Vec<String>,
    answered: BTreeSet<String>,
}

fn outcome(body: &str) -> Outcome {
    let mut relations = vec![
        "bin-malformed",
        "not-binary",
        "bin-undeclared",
        "division-by-zero",
        "false",
    ];
    relations.extend(OPS.iter().map(|op| op.result));
    let mut got = run_all(IMPORT, body, &relations);
    let mut answered = BTreeSet::new();
    for op in &OPS {
        for fact in got.remove(op.result).unwrap_or_default() {
            let p = parts(&fact);
            answered.insert(format!("({} {} {})", op.request, p[1], p[2]));
        }
    }
    let mut take = |r: &str| got.remove(r).unwrap_or_default();
    Outcome {
        malformed: take("bin-malformed").into_iter().collect(),
        not_binary: take("not-binary").into_iter().collect(),
        undeclared: take("bin-undeclared").into_iter().collect(),
        division_by_zero: take("division-by-zero"),
        contradiction: take("false"),
        answered,
    }
}

/// **A malformed numeral is flagged where an operation takes it, a malformed
/// operand at its request, and neither is answered**, as in decimal (Q-M1f.268,
/// Q-M1f.404,
/// defaulted: as far as decimal flags). A bit's place holding a name that is
/// no bit, a decimal digit among them, or holding a fact; a numeral that ends
/// in a bit where `z` goes, or in a named constant; the same in a `bin-let`'s
/// value. Each derives `std.math.binary`'s `(not-binary NUMERAL THING)` on the
/// smallest numeral at fault, once. **A bit where a number goes** is the
/// operand's fault, `(bin-malformed REQUEST)`; **a name for a name** is the
/// `bin-let`'s.
#[test]
fn a_malformed_request_is_flagged_once_and_never_answered() {
    let lets = "(bin-let mask (b2 z 1))\n(bin-let broken (b2 (b2 z x) 1))\n(bin-let alias mask)\n";
    let numerals = [
        ("(b2 (b2 z 1) x)", "(not-binary (b2 (b2 z 1) x) x)"),
        ("(b2 (b2 z 2) 1)", "(not-binary (b2 z 2) 2)"),
        ("(b2 (b2 (b2 z 1) z) 0)", "(not-binary (b2 (b2 z 1) z) z)"),
        ("(b2 z (b2 z 1))", "(not-binary (b2 z (b2 z 1)) (b2 z 1))"),
        ("(b2 1 0)", "(not-binary (b2 1 0) 1)"),
        ("(b2 0 1)", "(not-binary (b2 0 1) 0)"),
        ("(b2 (b2 1 1) 0)", "(not-binary (b2 1 1) 1)"),
        ("(b2 mask 1)", "(not-binary (b2 mask 1) mask)"),
        ("broken", "(not-binary (b2 z x) x)"),
        ("alias", ""),
    ];
    for op in &OPS {
        let mut body = String::from(lets);
        let mut want_numerals = BTreeSet::new();
        let mut want_requests = BTreeSet::new();
        want_requests.insert("(bin-malformed (bin-let alias mask))".to_string());
        for (b, flag) in numerals {
            if !flag.is_empty() {
                want_numerals.insert(flag.to_string());
            }
            for request in [
                format!("({} {b} {FINE})", op.request),
                format!("({} {FINE} {b})", op.request),
            ] {
                body.push_str(&request);
                body.push('\n');
            }
        }
        for b in ["1", "0"] {
            for request in [
                format!("({} {b} {FINE})", op.request),
                format!("({} {FINE} {b})", op.request),
            ] {
                body.push_str(&request);
                body.push('\n');
                want_requests.insert(format!("(bin-malformed {request})"));
            }
        }
        let fine = [
            format!("({} {FINE} {FINE})", op.request),
            format!("({} mask {FINE})", op.request),
        ];
        for request in &fine {
            body.push_str(request);
            body.push('\n');
        }
        let got = outcome(&body);
        assert_eq!(
            got.not_binary, want_numerals,
            "{}: the numerals' flags",
            op.request
        );
        assert_eq!(
            got.malformed, want_requests,
            "{}: the requests' flags",
            op.request
        );
        assert!(
            got.undeclared.is_empty() && got.division_by_zero.is_empty(),
            "{}: no other flag: {:?} {:?}",
            op.request,
            got.undeclared,
            got.division_by_zero
        );
        assert_eq!(got.contradiction, ["(false)"], "{}", op.request);
        assert_eq!(
            got.answered,
            fine.iter().cloned().collect::<BTreeSet<_>>(),
            "{}: the requests answered are the well-formed ones",
            op.request
        );
    }
    // A numeral no operation takes is not checked (Q-M1f.404): asked about
    // by no request, it has no flag, and nothing is a contradiction.
    let got = outcome("(bin-let unused (b2 (b2 z 1) 2))\n");
    assert!(got.not_binary.is_empty(), "{:?}", got.not_binary);
    assert!(got.contradiction.is_empty(), "{:?}", got.contradiction);
}

/// **A name no `bin-let` states is undeclared**, a decimal digit among the
/// names: `(bin-undeclared NAME REQUEST)` and `(false)`; one flag a request;
/// and the same program with the `bin-let`s stated is answered.
#[test]
fn an_undeclared_name_is_flagged_and_a_let_answers_it() {
    for op in &OPS {
        let r = op.request;
        let requests = format!("({r} foo {FINE})\n({r} {FINE} 7)\n({r} known foo)\n");
        let known = "(bin-let known (b2 (b2 z 1) 1))\n";
        let got = outcome(&format!("{known}{requests}({r} known {FINE})\n"));
        let want: BTreeSet<String> = [
            format!("(bin-undeclared foo ({r} foo {FINE}))"),
            format!("(bin-undeclared 7 ({r} {FINE} 7))"),
            format!("(bin-undeclared foo ({r} known foo))"),
        ]
        .into_iter()
        .collect();
        assert_eq!(got.undeclared, want, "{r}: one flag a request");
        assert!(got.malformed.is_empty(), "{r}: {:?}", got.malformed);
        assert_eq!(got.contradiction, ["(false)"], "{r}");
        assert_eq!(
            got.answered,
            [format!("({r} known {FINE})")].into_iter().collect(),
            "{r}"
        );
        let stated =
            format!("{known}(bin-let foo (b2 z 1))\n(bin-let 7 (b2 (b2 z 1) 1))\n{requests}");
        let got = outcome(&stated);
        assert!(
            got.undeclared.is_empty() && got.malformed.is_empty() && got.contradiction.is_empty(),
            "{r}: a `bin-let` removes the flag: {:?} {:?}",
            got.undeclared,
            got.malformed
        );
        assert_eq!(got.answered.len(), 3, "{r}: each request is answered");
    }
}

/// **A `bin-let` that cannot be read**: a name stated twice with two numbers,
/// each of the two; `z` or a bit as a constant's name; a bit or another named
/// constant as what it names.
#[test]
fn a_let_that_cannot_be_read_is_flagged() {
    let body = "(bin-let mask (b2 z 1))\n(bin-let mask (b2 (b2 z 1) 1))\n\
                (bin-let same (b2 z 1))\n(bin-let same (b2 z 1))\n\
                (bin-let z (b2 z 1))\n(bin-let 1 (b2 z 1))\n(bin-let 0 (b2 z 1))\n\
                (bin-let one 1)\n(bin-let none 0)\n(bin-let fine (b2 (b2 z 1) 0))\n(bin-let alias fine)\n";
    let got = outcome(body);
    let want: BTreeSet<String> = [
        "(bin-malformed (bin-let mask (b2 z 1)))",
        "(bin-malformed (bin-let mask (b2 (b2 z 1) 1)))",
        "(bin-malformed (bin-let z (b2 z 1)))",
        "(bin-malformed (bin-let 1 (b2 z 1)))",
        "(bin-malformed (bin-let 0 (b2 z 1)))",
        "(bin-malformed (bin-let one 1))",
        "(bin-malformed (bin-let none 0))",
        "(bin-malformed (bin-let alias fine))",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    assert_eq!(got.malformed, want);
    assert_eq!(got.contradiction, ["(false)"]);
    assert!(got.undeclared.is_empty());
    let got = outcome("(bin-let same (b2 z 1))\n(bin-let same (b2 z 1))\n(bin-add same same)\n");
    assert!(got.malformed.is_empty() && got.contradiction.is_empty());
    assert_eq!(got.answered.len(), 1);
}

/// **What stays silent**, as the headers of `std.math.binary` and
/// `std.math.binary-arith` list it: a prefix that is a name neither `z`, a
/// bit nor a named constant, a decimal digit among them; a fact of another
/// relation where a number goes, a decimal number and the theory's own
/// accumulator among them; a name that stands for such a thing.
#[test]
fn what_stays_silent() {
    let silent = [
        "(b2 end 1)",
        "(b2 7 1)",
        "(pair 1 0)",
        "(b2 (pair 1 0) 1)",
        "(bin-b 1 z)",
        "(n z 5)",
        "odd",
    ];
    for op in &OPS {
        let mut body = String::from("(bin-let odd (pair 1 0))\n");
        for x in silent {
            body.push_str(&format!("({} {x} {FINE})\n", op.request));
            body.push_str(&format!("({} {FINE} {x})\n", op.request));
        }
        let got = outcome(&body);
        assert!(
            got.malformed.is_empty()
                && got.not_binary.is_empty()
                && got.undeclared.is_empty()
                && got.division_by_zero.is_empty()
                && got.contradiction.is_empty(),
            "{}: flagged: {:?} {:?} {:?}",
            op.request,
            got.malformed,
            got.not_binary,
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

/// What one request costs, read off its firings, as `arith_theory.rs`'s
/// `circuit` reads it: its size and its depth, the closing rules' firings
/// left out.
fn circuit(body: &str) -> (usize, usize) {
    let src = ein_corpus::listed(&format!("{IMPORT}{body}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<binary>")).expect("the request parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the request loads");
    let mut events = Events::off();
    let mut depth_of: BTreeMap<ein_core::FactId, usize> = BTreeMap::new();
    let (mut firings, mut deepest) = (0usize, 0usize);
    let closers: Vec<ein_core::Symbol> = ["binary-arith", "binary", "arith"]
        .iter()
        .flat_map(|n| ein_corpus::spellings(n))
        .filter_map(|n| terms.syms.get(&n))
        .collect();
    assert!(closers.len() >= 3, "the three closing rules' names");
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

/// **Each operation as a circuit**, at the operands `arith_theory.rs` pins
/// decimal's at, written in binary: 123078 is 17 bits and 4567 is 13, so a
/// walk is three times decimal's six columns. The numbers are pinned so that
/// a change to the wiring shows as a change to them: `cargo test -p
/// ein-infer --test binary_theory the_circuit` re-takes them. Binary's firings
/// are its walks' and its operands' checks': `std.math.binary`'s walk of each
/// numeral an operand stands for, a firing a bit and one for the label
/// (Q-M1f.404), beside the chain, and deeper than it where a division by zero
/// stops the chain early. Against decimal's 35, 42, 42, 27, 82, 235 and 22
/// (`arith_theory.rs`): a walk is three times as long, a product's row needs
/// no walk of its own, and a quotient's bit is one subtraction where
/// decimal's digit is up to four.
#[test]
fn the_circuit_of_each_operation() {
    let n = |v: u128| normal(v);
    let pins = [
        (
            format!("(bin-add {} {})", n(123_078), n(4_567)),
            "123078 + 4567",
        ),
        (
            format!("(bin-sub {} {})", n(123_078), n(4_567)),
            "123078 - 4567",
        ),
        (
            format!("(bin-sub {} {})", n(4_567), n(123_078)),
            "4567 - 123078",
        ),
        (
            format!("(bin-cmp {} {})", n(123_078), n(4_567)),
            "123078 against 4567",
        ),
        (format!("(bin-mul {} {})", n(123), n(456)), "123 x 456"),
        (format!("(bin-div {} {})", n(127_645), n(25)), "127645 / 25"),
        (format!("(bin-div {} z)", n(127_645)), "127645 / 0"),
    ];
    let got: Vec<(&str, (usize, usize))> = pins
        .iter()
        .map(|(body, what)| (*what, circuit(&format!("{body}\n"))))
        .collect();
    assert_eq!(
        got,
        [
            ("123078 + 4567", (77, 43)),
            ("123078 - 4567", (95, 43)),
            ("4567 - 123078", (95, 43)),
            ("123078 against 4567", (58, 24)),
            ("123 x 456", (140, 120)),
            ("127645 / 25", (274, 246)),
            ("127645 / 0", (33, 21)),
        ],
        "(firings, depth)"
    );
}

// ── std.math.convert ───────────────────────────────────────────────

/// **A conversion each way, right against the integers**: every number from
/// 0 to 300, each power of two to 2^40 and either side of it, and a sample
/// to 2^40, asked of `std.math.convert` in one program: `(to-bin X)` is
/// answered by `(bin-of X B)`, B the number in binary's normal form, and
/// `(to-dec B)` by `(dec-of B X)`, in decimal's. **A round trip is the
/// number**: the binary the first answers is asked back, and its answer is
/// the decimal the program wrote.
#[test]
fn conversions_are_right_both_ways() {
    let mut values: BTreeSet<u128> = (0..=300).collect();
    for k in 1..=40 {
        values.extend([(1u128 << k) - 1, 1u128 << k, (1u128 << k) + 1]);
    }
    let mut rng = Rng(0x5146_1619);
    for _ in 0..30 {
        let l = 1 + rng.below(40) as u32;
        values.insert(rng.number(l));
    }
    let mut body = String::new();
    for &v in &values {
        body.push_str(&format!(
            "(to-bin {})\n(to-dec {})\n",
            decimal(v),
            normal(v)
        ));
    }
    let got = run_all(
        "(import std.math.convert :symbols (convert))\n",
        &body,
        &["bin-of", "dec-of"],
    );
    let bin_of: BTreeSet<String> = got["bin-of"].iter().cloned().collect();
    let dec_of: BTreeSet<String> = got["dec-of"].iter().cloned().collect();
    let want_bin: BTreeSet<String> = values
        .iter()
        .map(|&v| format!("(bin-of {} {})", decimal(v), normal(v)))
        .collect();
    let want_dec: BTreeSet<String> = values
        .iter()
        .map(|&v| format!("(dec-of {} {})", normal(v), decimal(v)))
        .collect();
    assert_eq!(bin_of, want_bin, "to binary, every value once and right");
    assert_eq!(dec_of, want_dec, "to decimal, every value once and right");
    assert_eq!(got["bin-of"].len(), values.len(), "one answer a request");
    assert_eq!(got["dec-of"].len(), values.len(), "one answer a request");
    // The round trip: what `(to-bin X)` answers, asked back, is X.
    let mut back = String::new();
    for fact in &got["bin-of"] {
        back.push_str(&format!("(to-dec {})\n", parts(fact)[2]));
    }
    let round = run_all(
        "(import std.math.convert :symbols (convert))\n",
        &back,
        &["dec-of"],
    );
    let round: BTreeSet<String> = round["dec-of"]
        .iter()
        .map(|f| parts(f)[2].clone())
        .collect();
    let written: BTreeSet<String> = values.iter().map(|&v| decimal(v)).collect();
    assert_eq!(round, written, "a round trip is the number");
}

/// **Both bases in one program**: each imported
/// whole, they share `z`, `0`, `1`, `std.math.arith` and the shared rules,
/// each activated once a base. The same requests asked in decimal and in
/// binary answer the same values, each in its own base, and neither base
/// answers the other's.
#[test]
fn both_bases_in_one_program_agree() {
    let import = "(import std.math.decimal-arith :symbols (decimal-arith))\n\
                  (import std.math.binary-arith :symbols (binary-arith))\n";
    let mut rng = Rng(0x5146_161a);
    let mut body = String::new();
    let mut want = BTreeMap::new();
    for _ in 0..60 {
        let (la, lb) = (1 + rng.below(30) as u32, 1 + rng.below(30) as u32);
        let (a, b) = (rng.number(la), rng.number(lb));
        body.push_str(&format!(
            "(add {} {})\n(bin-add {} {})\n(mul {} {})\n(bin-mul {} {})\n",
            decimal(a),
            decimal(b),
            normal(a),
            normal(b),
            decimal(a),
            decimal(b),
            normal(a),
            normal(b)
        ));
        want.insert(("sum-of", decimal(a), decimal(b)), decimal(a + b));
        want.insert(("bin-sum-of", normal(a), normal(b)), normal(a + b));
        want.insert(("product-of", decimal(a), decimal(b)), decimal(a * b));
        want.insert(("bin-product-of", normal(a), normal(b)), normal(a * b));
    }
    let results = ["sum-of", "bin-sum-of", "product-of", "bin-product-of"];
    let got = run_all(import, &body, &results);
    let mut seen = BTreeMap::new();
    for r in results {
        for fact in &got[r] {
            let p = parts(fact);
            assert!(
                seen.insert((r, p[1].clone(), p[2].clone()), p[3].clone())
                    .is_none(),
                "two answers: {fact}"
            );
        }
    }
    assert_eq!(seen, want, "each base answers its own requests, and right");
}
