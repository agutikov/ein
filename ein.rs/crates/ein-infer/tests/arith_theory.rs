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
//! | every sum of two numbers up to [`SMALL`] is right | [`every_small_sum_is_right`] |
//! | sums of operands of every length up to [`MAX_DIGITS`] digits are right, on a sample | [`sums_of_every_length_are_right`] |
//! | a carry runs through every column and out of the top, at every length | [`a_carry_runs_the_whole_length`] |
//! | operands written with high zeros are read by value | [`high_zeros_are_read_by_value`] |
//! | **every answer is in normal form**, and each request has exactly one | all four, through [`sums`] |
//! | **the rule of the theory**, on the module's text: no rule joins two facts on a number | [`no_rule_joins_two_facts_on_a_number`] |
//! | no `absent`, no `not`, no `or`: one stratum | [`the_theory_writes_no_negation`] |
//! | the closing rule closes every relation and names every rule | [`the_closing_rule_names_every_rule_and_closes_every_relation`] |
//! | what one addition costs as a circuit: its firings and its depth | [`the_circuit_of_one_addition`] |
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
//! **A later operation joins this file** by adding its request and result
//! relations to [`run`]'s callers and its cases beside these; the three
//! static tests read the whole module and need no change to cover a new rule.

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// Every pair of operands from 0 to this is added: 121 × 121 sums, which
/// cover every row of the full adder's table and the step from two digits to
/// three.
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

/// Ask every `(x, y)` as one program and hold every answer: **exactly one**
/// `sum-of` per request, repeating the operands as written, its sum the
/// normal form of `want`. `cases` is `(x as written, y as written, x + y)`.
fn sums(cases: &[(String, String, u128)]) {
    let mut want: BTreeMap<(String, String), u128> = BTreeMap::new();
    let mut body = String::new();
    for (x, y, sum) in cases {
        if want.insert((x.clone(), y.clone()), *sum).is_none() {
            body.push_str(&format!("(add {x} {y})\n"));
        }
    }
    let mut got: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for fact in run(&body, "sum-of") {
        let p = parts(&fact);
        assert_eq!(p.len(), 4, "a sum-of of another arity: {fact}");
        got.entry((p[1].clone(), p[2].clone()))
            .or_default()
            .push(p[3].clone());
    }
    for (request, sum) in &want {
        let answers = got.remove(request).unwrap_or_default();
        assert_eq!(
            answers,
            [normal(*sum)],
            "(add {} {}): the integers say {sum}, in normal form {}",
            request.0,
            request.1,
            normal(*sum)
        );
    }
    assert!(
        got.is_empty(),
        "answers to requests nobody made: {:?}",
        got.keys().collect::<Vec<_>>()
    );
}

// ── right, held to the integers ────────────────────────────────────

#[test]
fn every_small_sum_is_right() {
    let mut cases = Vec::new();
    for a in 0..=SMALL {
        for b in 0..=SMALL {
            cases.push((normal(a), normal(b), a + b));
        }
    }
    sums(&cases);
}

#[test]
fn sums_of_every_length_are_right() {
    let mut rng = Rng(0x5146_0603);
    let mut cases = Vec::new();
    // Every pair of lengths from 0 to MAX_DIGITS, twice: 31 × 31 × 2.
    for la in 0..=MAX_DIGITS {
        for lb in 0..=MAX_DIGITS {
            for _ in 0..2 {
                let (a, b) = (rng.number(la), rng.number(lb));
                cases.push((normal(a), normal(b), a + b));
            }
        }
    }
    sums(&cases);
}

#[test]
fn a_carry_runs_the_whole_length() {
    let mut cases = Vec::new();
    let mut nines: u128 = 0;
    for _ in 1..=MAX_DIGITS {
        nines = nines * 10 + 9;
        // 99…9 + 1, 1 + 99…9, and 99…9 + 99…9: the answer is one digit longer.
        cases.push((normal(nines), normal(1), nines + 1));
        cases.push((normal(1), normal(nines), nines + 1));
        cases.push((normal(nines), normal(nines), nines + nines));
    }
    sums(&cases);
}

#[test]
fn high_zeros_are_read_by_value() {
    let mut rng = Rng(0x5146_0604);
    let pad = |rng: &mut Rng, v: u128| {
        let zeros = "0".repeat(rng.below(4) as usize);
        let digits = if v == 0 { String::new() } else { v.to_string() };
        written(&format!("{zeros}{digits}"))
    };
    let mut cases = Vec::new();
    for _ in 0..1500 {
        let (la, lb) = (rng.below(13) as u32, rng.below(13) as u32);
        let (a, b) = (rng.number(la), rng.number(lb));
        let (x, y) = (pad(&mut rng, a), pad(&mut rng, b));
        cases.push((x, y, a + b));
    }
    // Zero written as nothing but zeros, on either side and both.
    for zeros in ["0", "00", "000"] {
        cases.push((written(zeros), written("7"), 7));
        cases.push((written("7"), written(zeros), 7));
        cases.push((written(zeros), written(zeros), 0));
        cases.push((written(zeros), "z".to_string(), 0));
    }
    sums(&cases);
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
        for banned in ["absent", "not", "or", "neq", "false"] {
            assert!(
                !heads.iter().any(|h| h == banned),
                "rule {name} writes `{banned}`"
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
