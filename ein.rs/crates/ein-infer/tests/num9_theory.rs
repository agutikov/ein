//! **`std.math.num9`, held to the integers and to its own rule** — M1f
//! [S1f.6.13](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.13_num9.md).
//!
//! `stdlib/math/num9.ein` is school arithmetic on numbers of nine digits, the
//! calculator's second module: wide rules, a premise a column, written by
//! `utils/gen_num9.py`. Ein has no numbers, so nothing in the engine can say
//! whether a sum is right. What holds the theory is here.
//!
//! | claim | where |
//! |---|---|
//! | **exhaustively, at a width small enough to exhaust:** the generator's output at width 2, every pair of operands, all five operations, 49 900 requests | [`width_two_is_right_on_every_pair`] |
//! | at the shipped width, a sample of operands of every length, the corners, and requests that overflow | [`width_nine_is_right_on_a_sample`] |
//! | **overflow is a flag and never a truncated number**: a request has exactly one of an answer and a flag | both, through [`hold`] |
//! | the conversions to and from `std.math.decimal-arith`'s numbers, at every count of digits; `big` past the width | [`conversions_are_right_at_every_length`] |
//! | a division by zero derives the flag, and `std.math.decimal-arith`'s rule `(false)` from it | [`a_division_by_zero_is_flagged_and_false`] |
//! | the file is what the generator writes: the script's `--check`, which is how that check is in the gate | [`the_file_is_what_the_generator_writes`] |
//! | **the rule of the theory**, on the module's text: no rule joins two facts on a number | [`no_rule_joins_two_facts_on_a_number`] |
//! | no `absent`, no `not`, no `or`, no guard, and no `(false)` of its own | [`the_theory_writes_no_negation`] |
//! | the closing rule closes every relation and names every rule | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//! | what one request costs as a circuit: its firings | [`the_circuit_of_each_operation`] |
//!
//! The oracle is `u64`. The measured half of the rule of the theory, a
//! batch that costs in proportion to its count, is `arith_batch.rs`; the
//! search-free claim, on the binary, is `ein-cli/tests/arith_search_free.rs`.

use std::collections::{BTreeMap, BTreeSet};
use std::process::{Command, Stdio};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// The shipped width.
const WIDTH: u32 = 9;

const IMPORT: &str = "(import std.math.num9 :symbols (arith9))\n";

// ── numbers as the two theories write them ─────────────────────────

/// `v` as a number of `w` digits: `(num9 0 0 0 1 2 3 0 7 8)`.
fn fixed(v: u64, w: u32) -> String {
    let digits = format!("{v:0>width$}", width = w as usize);
    assert_eq!(digits.len(), w as usize, "{v} does not fit {w} digits");
    let mut out = format!("(num{w}");
    for d in digits.chars() {
        out.push(' ');
        out.push(d);
    }
    out.push(')');
    out
}

/// A string of decimal digits as a number of `std.math.decimal-arith`, as written:
/// `""` is `z`, `"042"` is `(n (n (n z 0) 4) 2)`.
fn nested(digits: &str) -> String {
    let mut out = String::from("z");
    for d in digits.chars() {
        out = format!("(n {out} {d})");
    }
    out
}

/// `v` in `std.math.decimal-arith`'s normal form.
fn normal(v: u64) -> String {
    if v == 0 {
        "z".into()
    } else {
        nested(&v.to_string())
    }
}

/// SplitMix64, written out so that a failure names operands that fail again.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number of exactly `digits` digits; 0 for no digit.
    fn number(&mut self, digits: u32) -> u64 {
        if digits == 0 {
            return 0;
        }
        let lo = 10u64.pow(digits - 1);
        lo + self.next() % (9 * lo)
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

struct Run {
    /// Every fact of each relation asked for, as text.
    facts: BTreeMap<String, Vec<String>>,
    /// Firings, the closing rule's left out.
    firings: usize,
    /// Whether root derived `(false)`.
    dead: bool,
}

/// Saturate `src` and read the facts of `relations` out of its fixpoint.
/// `closer` is the closing rule's name, whose firing is not counted.
fn run(src: &str, relations: &[&str], closer: &str) -> Run {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let src = &ein_corpus::listed(src);
    let forms = parse(&mut ast, src, Some("<num9>")).expect("the requests parse");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the requests load");
    let mut events = Events::off();
    let closer = ein_corpus::spellings(closer)
        .iter()
        .find_map(|n| terms.syms.get(n))
        .expect("the closing rule's name");
    // `std.math.decimal-arith`'s closing rule fires too since M1f S1f.6.35, the module
    // coming whole with `std.math.num9`'s import of it, and since S1f.6.38 those
    // of `std.math.decimal` and `std.math.arith`; they are no request's either.
    let lower: Vec<ein_core::Symbol> = [
        "std.math.decimal-arith.decimal-arith",
        "std.math.decimal.decimal",
        "std.math.arith.arith",
    ]
    .iter()
    .filter_map(|n| terms.syms.get(n))
    .collect();
    let mut firings = 0usize;
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
            if f.rule != closer && !lower.contains(&f.rule) {
                firings += 1;
            }
        })
        .expect("the theory reaches its fixpoint");
    }
    let mut facts = BTreeMap::new();
    for &r in relations {
        let found = ein_corpus::spellings(r)
            .iter()
            .find_map(|n| terms.syms.get(n))
            .map_or_else(Vec::new, |rel| {
                kb.facts_of(rel)
                    .map(|f| ein_corpus::unprefixed(&sexpr(&terms, f)))
                    .collect()
            });
        facts.insert(r.to_string(), found);
    }
    let dead = terms
        .syms
        .get("false")
        .is_some_and(|rel| kb.facts_of(rel).next().is_some());
    Run {
        facts,
        firings,
        dead,
    }
}

/// One operation, as the harness asks it at width `w`: its stem (`add` for
/// `add9`), its result's stem, and what the integers say: the result's
/// arguments after the operands, or `None` where the answer does not fit `w`
/// digits and the request must be flagged instead.
struct Op {
    request: &'static str,
    result: &'static str,
    answer: fn(u64, u64, u32) -> Option<Vec<String>>,
}

const OPS: [Op; 5] = [
    Op {
        request: "add",
        result: "sum",
        answer: |a, b, w| (a + b < 10u64.pow(w)).then(|| vec![fixed(a + b, w)]),
    },
    // Two numbers, and one of them zero: a difference never overflows.
    Op {
        request: "sub",
        result: "diff",
        answer: |a, b, w| {
            Some(vec![
                fixed(a.saturating_sub(b), w),
                fixed(b.saturating_sub(a), w),
            ])
        },
    },
    Op {
        request: "cmp",
        result: "cmp",
        answer: |a, b, _| {
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
    Op {
        request: "mul",
        result: "product",
        answer: |a, b, w| (a * b < 10u64.pow(w)).then(|| vec![fixed(a * b, w)]),
    },
    // A zero divisor is a contradiction and is never asked here.
    Op {
        request: "div",
        result: "quot",
        answer: |a, b, w| Some(vec![fixed(a / b, w), fixed(a % b, w)]),
    },
];

/// Ask every pair of every operation as one program at width `w`, with
/// `theory` in front of the requests, and hold every request to the
/// integers: **exactly one** result fact where the answer fits, repeating
/// the operands as written, and no flag; **exactly one** `(overflow
/// REQUEST)` where it does not, and no result fact: never a truncated
/// number. And no answer and no flag for a request nobody made.
fn hold(theory: &str, w: u32, cases: &[(u64, u64)]) {
    let name = |stem: &str, of: bool| format!("{stem}{w}{}", if of { "-of" } else { "" });
    let mut body = String::from(theory);
    let mut want: BTreeMap<(String, String, String), Option<Vec<String>>> = BTreeMap::new();
    for op in &OPS {
        let request = name(op.request, false);
        for &(a, b) in cases {
            if op.request == "div" && b == 0 {
                continue;
            }
            let (x, y) = (fixed(a, w), fixed(b, w));
            let key = (request.clone(), x.clone(), y.clone());
            if want.insert(key, (op.answer)(a, b, w)).is_none() {
                body.push_str(&format!("({request} {x} {y})\n"));
            }
        }
    }
    let results: Vec<String> = OPS.iter().map(|op| name(op.result, true)).collect();
    let mut relations: Vec<&str> = results.iter().map(String::as_str).collect();
    relations.push("overflow");
    let got = run(&body, &relations, &format!("arith{w}"));
    assert!(!got.dead, "width {w}: root derived (false)");

    let mut answers: BTreeMap<(String, String, String), Vec<Vec<String>>> = BTreeMap::new();
    for op in &OPS {
        for fact in &got.facts[&name(op.result, true)] {
            let p = parts(fact);
            assert!(p.len() > 3, "a result of another arity: {fact}");
            answers
                .entry((name(op.request, false), p[1].clone(), p[2].clone()))
                .or_default()
                .push(p[3..].to_vec());
        }
    }
    let mut flags: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for fact in &got.facts["overflow"] {
        let p = parts(fact);
        assert_eq!(p.len(), 2, "a flag names one request: {fact}");
        let r = parts(&p[1]);
        assert_eq!(r.len(), 3, "a request has two operands: {fact}");
        *flags
            .entry((r[0].clone(), r[1].clone(), r[2].clone()))
            .or_default() += 1;
    }
    for (request, answer) in &want {
        let answered = answers.remove(request).unwrap_or_default();
        let flagged = flags.remove(request).unwrap_or(0);
        match answer {
            Some(a) => {
                assert_eq!(
                    answered,
                    std::slice::from_ref(a),
                    "{request:?}: the integers say {a:?}"
                );
                assert_eq!(flagged, 0, "{request:?} fits and is flagged");
            }
            None => {
                assert!(
                    answered.is_empty(),
                    "{request:?} does not fit {w} digits and has a number: {answered:?}"
                );
                assert_eq!(
                    flagged, 1,
                    "{request:?} does not fit and is not flagged once"
                );
            }
        }
    }
    assert!(
        answers.is_empty() && flags.is_empty(),
        "answers or flags for requests nobody made: {:?} {:?}",
        answers.keys().collect::<Vec<_>>(),
        flags.keys().collect::<Vec<_>>()
    );
}

/// `python3 utils/gen_num9.py ARGS`, its stdout.
fn generator(args: &[&str]) -> std::process::Output {
    let probe = Command::new("python3")
        .args(["-c", "pass"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    assert!(
        matches!(probe, Ok(s) if s.success()),
        "`python3` does not run ({probe:?}): `stdlib/math/num9.ein` is generated, and \
         `utils/gen_num9.py` is what this file holds it to — a missing gate, not a \
         missing convenience"
    );
    Command::new("python3")
        .arg("utils/gen_num9.py")
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("python3 ran a moment ago")
}

// ── right, held to the integers ────────────────────────────────────

/// **Exhaustive.** The generator's output at width 2 is the shipped theory
/// with two columns for nine: the same rules, a premise a column, the same
/// three places a product overflows and the same ten step rules of a
/// division. Every pair of operands from 00 to 99, every operation: 10 000
/// requests each, 9 900 for division, whose divisor is never zero. So every
/// row of every table the theory reads is used, and every way a carry, a
/// borrow or an overflow can meet another is met.
#[test]
fn width_two_is_right_on_every_pair() {
    let out = generator(&["--width", "2", "--stdout"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let theory = String::from_utf8(out.stdout).expect("utf-8");
    assert!(
        theory.contains("(rule add2-fits ") && theory.contains("(rule div2-step-9 "),
        "the generator's width 2 is the theory over `num2`"
    );
    let cases: Vec<(u64, u64)> = (0..100)
        .flat_map(|a| (0..100).map(move |b| (a, b)))
        .collect();
    hold(&theory, 2, &cases);
}

/// **The shipped module, on a sample.** Operands of every pair of lengths
/// from 0 to 9 digits, four draws a pair by a fixed-seed generator; equal
/// operands and operands one apart at every length; and the corners, where
/// a sum or a product is the last that fits or the first that does not.
#[test]
fn width_nine_is_right_on_a_sample() {
    let mut rng = Rng(0x5146_0613);
    let mut cases: Vec<(u64, u64)> = Vec::new();
    for la in 0..=WIDTH {
        for lb in 0..=WIDTH {
            for _ in 0..4 {
                cases.push((rng.number(la), rng.number(lb)));
            }
        }
        let a = rng.number(la);
        cases.push((a, a));
        cases.push((a, a.saturating_sub(1)));
        cases.push((a.saturating_sub(1), a));
    }
    let top = 10u64.pow(WIDTH) - 1;
    cases.extend([
        (top, 0),
        (top, 1),
        (top - 1, 1),
        (1, top),
        (top, top),
        (0, 0),
        (500_000_000, 500_000_000),
        (499_999_999, 500_000_000),
        (31_622, 31_622),
        (31_623, 31_623),
        (500_000_000, 2),
        (2, 500_000_000),
        (111_111_111, 9),
        (111_111_112, 9),
        (123_078, 123_078),
        (99_999, 10_000),
        (100_000, 10_000),
        (top, 7),
        (864_197_523, 7),
        (top, 123_456_789),
    ]);
    hold(IMPORT, WIDTH, &cases);
}

/// **The conversions, at every count of digits.** `(to9 X)` of a number
/// written with up to nine digits is `(as9 X W)`, W the same digits behind
/// zeros, high zeros of X read as written; of one written with more,
/// `(big X)` and no `as9`. `(from9 W)` is `(of9 W X)`, X in normal form.
/// One answer each, and none nobody asked for.
#[test]
fn conversions_are_right_at_every_length() {
    let mut rng = Rng(0x5146_0614);
    let mut body = String::from(IMPORT);
    let mut want_as: BTreeMap<String, String> = BTreeMap::new();
    let mut want_big: BTreeSet<String> = BTreeSet::new();
    let mut want_of: BTreeMap<String, String> = BTreeMap::new();
    for digits in 0..=12u32 {
        for _ in 0..6 {
            // A number of `digits` digits, and the same behind up to two high zeros.
            let mut written = String::new();
            for i in 0..digits {
                let d = if i == 0 {
                    1 + rng.next() % 9
                } else {
                    rng.next() % 10
                };
                written.push_str(&d.to_string());
            }
            for pad in 0..=2usize {
                let w = format!("{}{written}", "0".repeat(pad));
                let x = nested(&w);
                if w.len() <= WIDTH as usize {
                    let v: u64 = if w.is_empty() {
                        0
                    } else {
                        w.parse().expect("digits")
                    };
                    want_as.insert(x.clone(), fixed(v, WIDTH));
                } else {
                    want_big.insert(x.clone());
                }
                body.push_str(&format!("(to9 {x})\n"));
            }
            if digits <= WIDTH {
                let v: u64 = if written.is_empty() {
                    0
                } else {
                    written.parse().expect("digits")
                };
                want_of.insert(fixed(v, WIDTH), normal(v));
                body.push_str(&format!("(from9 {})\n", fixed(v, WIDTH)));
            }
        }
    }
    // A zero inside and at the end is kept.
    for v in [100_200u64, 10, 100_000_000, 909_090_909] {
        want_of.insert(fixed(v, WIDTH), normal(v));
        body.push_str(&format!("(from9 {})\n", fixed(v, WIDTH)));
    }
    let got = run(&body, &["as9", "big", "of9"], "arith9");
    let pairs = |rel: &str| -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        for fact in &got.facts[rel] {
            let p = parts(fact);
            assert_eq!(p.len(), 3, "{fact}");
            assert!(
                out.insert(p[1].clone(), p[2].clone()).is_none(),
                "two answers for {}",
                p[1]
            );
        }
        out
    };
    assert_eq!(pairs("as9"), want_as, "(as9 X W)");
    assert_eq!(pairs("of9"), want_of, "(of9 W X)");
    let big: BTreeSet<String> = got.facts["big"]
        .iter()
        .map(|f| parts(f)[1].clone())
        .collect();
    assert_eq!(big, want_big, "(big X)");
}

/// **Division by zero is a contradiction, in two steps,** as in `std.math.decimal-arith`
/// and by its rule: `div9-by-zero` derives the flag, naming the request as
/// written, and `division-by-zero-is-false` derives `(false)`. No quotient
/// is stored for the request. Beside it, a division that is not by zero is
/// not flagged.
#[test]
fn a_division_by_zero_is_flagged_and_false() {
    for a in [0u64, 7, 127_645, 999_999_999] {
        let request = format!("(div9 {} {})", fixed(a, WIDTH), fixed(0, WIDTH));
        let got = run(
            &format!("{IMPORT}{request}\n"),
            &["division-by-zero", "quot9-of"],
            "arith9",
        );
        assert!(got.dead, "{request}: root derives (false)");
        assert_eq!(
            got.facts["division-by-zero"],
            [format!("(division-by-zero {request})")],
            "the flag names the request as written"
        );
        assert!(
            got.facts["quot9-of"].is_empty(),
            "{request}: a division by zero is never answered with a number"
        );
    }
    let got = run(
        &format!("{IMPORT}(div9 {} {})\n", fixed(0, WIDTH), fixed(7, WIDTH)),
        &["division-by-zero", "quot9-of"],
        "arith9",
    );
    assert!(!got.dead && got.facts["division-by-zero"].is_empty());
    assert_eq!(got.facts["quot9-of"].len(), 1, "0 divided by 7 is answered");
}

/// **What one request costs.** A sum, a difference and a comparison are one
/// firing, a premise a column; a difference of equal numbers is two, both
/// subtractors concluding the same fact. A product is eleven: a start, a
/// step for each of nine digits, an end. A quotient is eleven: the start
/// that computes the divisor's multiples, nine steps, an end. A conversion
/// is one. A product that overflows stops where it does.
///
/// Pinned, so that a change to the wiring shows: `cargo test -p ein-infer
/// --test num9_theory the_circuit` re-takes them.
#[test]
fn the_circuit_of_each_operation() {
    let n = |v| fixed(v, WIDTH);
    let cases = [
        (format!("(add9 {} {})", n(123_078), n(4_567)), 1),
        (format!("(sub9 {} {})", n(123_078), n(4_567)), 1),
        (format!("(sub9 {} {})", n(25), n(25)), 2),
        (format!("(cmp9 {} {})", n(123_078), n(4_567)), 1),
        (format!("(mul9 {} {})", n(123), n(456)), 11),
        (format!("(div9 {} {})", n(127_645), n(25)), 11),
        (format!("(to9 {})", normal(123_078)), 1),
        (format!("(from9 {})", n(123_078)), 1),
        (format!("(add9 {} {})", n(999_999_999), n(1)), 1),
        // 123078 x 123078: the start, seven steps, to where the sum so far has
        // a top digit and two digits of the multiplier are still to come, and
        // the flag.
        (format!("(mul9 {} {})", n(123_078), n(123_078)), 9),
    ];
    for (request, firings) in cases {
        let got = run(&format!("{IMPORT}{request}\n"), &[], "arith9");
        assert_eq!(got.firings, firings, "{request}");
    }
}

/// `utils/gen_num9.py --check`, run here so that it is in the gate: the
/// per-commit tier runs `cargo test --workspace`, and its job installs a
/// `python3`, as for `utils/gen_arith_tables.py --check` (Q-M1f.145).
#[test]
fn the_file_is_what_the_generator_writes() {
    let out = generator(&["--check"]);
    assert!(
        out.status.success(),
        "stdlib/math/num9.ein is stale — run `python3 utils/gen_num9.py` and \
         `python3 utils/stdlib_manifest.py --write`\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

// ── the module's text ──────────────────────────────────────────────

/// The tables of `std.math.digits` the module reads.
const TABLES: [&str; 6] = ["fa", "fs", "md", "cl", "nz", "adj"];

/// What the module takes from `std.math.decimal-arith`: the number's relation, for the
/// conversions, and the flag with the rule from it to `(false)`.
/// What the module takes from the modules below it, by module (M1f S1f.6.38:
/// `n` is `std.math.decimal`'s, `z`, the flag of a division by zero and its
/// rule `std.math.arith`'s; `std.math.decimal-arith` comes whole, its import
/// bare).
const FROM_ARITH: [(&str, &[&str]); 3] = [
    ("std.math.decimal-arith", &[]),
    ("std.math.decimal", &["n"]),
    (
        "std.math.arith",
        &["z", "division-by-zero", "division-by-zero-is-false"],
    ),
];

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module() -> Module {
    let path = repo_root().join("stdlib/math/num9.ein");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.math.num9 parses");
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

    /// The names a `(import MODULE :symbols (…))` of the module lists.
    fn imported(&self, module: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for &f in &self.forms {
            if self.ast.head_name(f) != Some("import") {
                continue;
            }
            let args = self.ast.form_args(f);
            if self.ast.atom_name(args[0]) != Some(module) {
                continue;
            }
            for &a in &args[1..] {
                if let Node::KwPair { key, value } = self.ast.node(a) {
                    assert_eq!(self.keyword(key), Some("symbols"));
                    out.extend(self.ast.head_name(value).map(str::to_string));
                    out.extend(
                        self.ast
                            .form_args(value)
                            .iter()
                            .filter_map(|&t| self.ast.atom_name(t).map(str::to_string)),
                    );
                }
            }
        }
        out
    }
}

/// **The rule of the theory, kept by the second module too.** Every rule
/// has at most one premise that is not a row of `std.math.digits`; that premise
/// holds the numbers, and a table row holds none. It is what the prototype's
/// division broke, joining its state with the divisor's multiples on the
/// divisor, so that 4 000 divisions took 8.1 s: here the multiples travel in
/// the state (`arith_batch.rs` measures the proportion).
#[test]
fn no_rule_joins_two_facts_on_a_number() {
    let m = module();
    let rules = m.declared("rule");
    assert!(
        rules.len() >= 40,
        "std.math.num9 parsed to {} rules",
        rules.len()
    );
    for (name, rule) in &rules {
        let mut state = Vec::new();
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
            "rule {name} joins {state:?}: a rule of std.math.num9 has ONE premise that \
             holds numbers, and its other premises are rows of std.math.digits (E-3 of \
             P1f.6's findings.md)"
        );
    }
}

/// No `absent`, no `not`, no `or`, no guard: the theory is monotone and has
/// one stratum. And it concludes no `(false)` of its own: a division by zero
/// derives `std.math.decimal-arith`'s flag, and `std.math.decimal-arith`'s rule does the rest.
#[test]
fn the_theory_writes_no_negation() {
    let m = module();
    for (name, rule) in m.declared("rule") {
        let mut heads = Vec::new();
        m.heads(m.clause(rule, "match"), &mut heads);
        m.heads(m.clause(rule, "assert"), &mut heads);
        for banned in ["absent", "not", "or", "neq", "eq", "false"] {
            assert!(
                !heads.iter().any(|h| h == banned),
                "rule {name} writes `{banned}`"
            );
        }
    }
    assert!(
        m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
        "std.math.num9 guesses nothing and expands nothing"
    );
}

/// `arith9` is the theory's one entry: it closes every relation the module
/// declares, every table it reads and the two relations it takes from the
/// modules below, `n` and `division-by-zero`, and names every other rule of
/// the module and the one it takes. Its own name is the module's one written
/// fact (E-11).
#[test]
fn the_closing_rule_names_every_rule_and_closes_every_relation() {
    let m = module();
    let rules = m.declared("rule");
    let (_, closer) = rules
        .iter()
        .find(|(n, _)| n == "arith9")
        .expect("the closing rule, `arith9`");
    assert!(
        m.conjuncts(m.clause(*closer, "match")).is_empty(),
        "`arith9` fires at root, on nothing"
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
            Some("num9-rule") => assert!(named.insert(arg.clone()), "named twice: {arg}"),
            other => panic!("`arith9` asserts a {other:?}"),
        }
    }
    let tables = m.imported("std.math.digits");
    assert_eq!(
        tables,
        TABLES.iter().map(|t| t.to_string()).collect(),
        "the tables the module imports"
    );
    for (module, names) in FROM_ARITH {
        assert_eq!(
            m.imported(module),
            names.iter().map(|t| t.to_string()).collect(),
            "what the module takes from {module}"
        );
    }

    let mut want_closed: BTreeSet<String> =
        m.declared("relation").into_iter().map(|(n, _)| n).collect();
    want_closed.extend(tables);
    want_closed.extend(["n".to_string(), "division-by-zero".to_string()]);
    assert_eq!(
        closed, want_closed,
        "`arith9` closes every relation the module declares, every table it \
         imports, and the two relations it takes from the modules below"
    );
    let mut want_named: BTreeSet<String> = rules
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n != "arith9")
        .collect();
    want_named.insert("division-by-zero-is-false".to_string());
    assert_eq!(
        named, want_named,
        "`arith9` names every other rule of the module, and the rule it takes"
    );

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
        [("num9-rule".to_string(), vec!["arith9".to_string()])],
        "the module writes one fact, the closing rule's own name"
    );
}
