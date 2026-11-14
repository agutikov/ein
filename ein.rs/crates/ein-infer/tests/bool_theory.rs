//! **`std.bool`, held to an enumeration of the sixteen columns** — M1f
//! [S1f.6.17](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md).
//!
//! `stdlib/bool/` is four modules: `std.bool.gates`, the sixteen Boolean
//! functions of two inputs as one table and as a relation each;
//! `std.bool.props`, thirty relations known of them, derived from the table
//! by rules; `std.bool.circuit`; and `std.bool.inputs`. Ein has no numbers
//! and no truth values of its own, so nothing in the engine can say whether
//! `(gate2 Orn 0 1 0)` is a right row or `(dual-of Andn Orn)` a right fact; a
//! wrong one would be a wrong answer in every program that reads it, and no
//! golden would know. What holds them is this file, against Rust's own
//! evaluation of each function and of each property's **definition**:
//!
//! | claim | where |
//! |---|---|
//! | the table is the sixteen columns, every row once, none missing and none twice, and each function's second name its column | [`the_table_is_the_sixteen_columns`] |
//! | the second layout, `(And A B OUT)`, is the table's rows and no other (the user's reply 19 (c)) | [`each_relation_of_a_function_is_its_rows`] |
//! | **every fact `std.bool.props` derives is the enumeration's**, relation by relation, and every fact the enumeration has is derived | [`every_property_is_its_definition`] |
//! | the three counts the user confirmed (reply 22 (a)), on the facts the rules derive: four swap pairs and eight functions their own swap; the duals `Andn`/`Orn` and `Randn`/`Rorn`; twelve functions up to the order of the inputs | [`the_three_corrected_counts`] |
//! | the one-sided identity and absorbing facts are derived (reply 21 (b)), eight a relation over seven functions, and their two-sided meets four | [`the_one_sided_forms_are_derived`] |
//! | each module's closing rule closes its relations and names every rule and the entry of the module below it | [`each_closing_rule_names_every_rule_and_closes_every_relation`] |
//! | the modules are monotone: no `absent`, no `not`, no `or`, no hrule and no macro; `neq` and `(false)` only in the two rules that refute | [`the_modules_write_no_negation`] |
//! | the generated files are what `utils/gen_gates.py` writes — the script's `--check`, which is how that check is in the gate | [`the_files_are_what_the_generator_writes`] |
//!
//! The generator computes the same relations in Python, from the columns,
//! and writes them as the claims of the programs under `tests/stdlib/bool/`;
//! this file computes them again from each property's definition in Rust,
//! and compares the facts the rules derive with both through the gate. So a
//! property is stated three times, in three languages: as a rule, in Python
//! and here.

use std::collections::{BTreeMap, BTreeSet};
use std::process::{Command, Stdio};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::sexpr;
use ein_infer::{Events, Saturator, Session, SharedMemo};
use ein_ir::{Ast, Node, NodeId, parse};

/// The sixteen names, each at its column read as a number of four bits, the
/// output at a b = 00 the highest: `Xor`'s column is 0110, six.
const NAMES: [&str; 16] = [
    "Zero", "And", "Andn", "Fst", "Randn", "Snd", "Xor", "Or", "Nor", "Xnor", "Nsnd", "Orn",
    "Nfst", "Rorn", "Nand", "One",
];

const BITS: [u8; 2] = [0, 1];

/// Function `f` of `a` and `b`, by its column.
fn out(f: usize, a: u8, b: u8) -> u8 {
    u8::from(f >> (3 - (2 * a + b)) & 1 == 1)
}

/// The function a two-input evaluation is, by its column.
fn named(g: impl Fn(u8, u8) -> u8) -> usize {
    let mut f = 0;
    for a in BITS {
        for b in BITS {
            f = f << 1 | usize::from(g(a, b));
        }
    }
    f
}

fn column(f: usize) -> [u8; 4] {
    [out(f, 0, 0), out(f, 0, 1), out(f, 1, 0), out(f, 1, 1)]
}

fn fact(rel: &str, args: &[String]) -> String {
    format!("({rel} {})", args.join(" "))
}

fn n(f: usize) -> String {
    NAMES[f].to_string()
}

fn bit(b: u8) -> String {
    b.to_string()
}

// ── the definitions ────────────────────────────────────────────────

fn commutative(f: usize) -> bool {
    BITS.iter()
        .all(|&a| BITS.iter().all(|&b| out(f, a, b) == out(f, b, a)))
}

fn assoc_at(f: usize, a: u8, b: u8, c: u8) -> bool {
    out(f, out(f, a, b), c) == out(f, a, out(f, b, c))
}

fn reads_first(f: usize) -> bool {
    BITS.iter().any(|&b| out(f, 0, b) != out(f, 1, b))
}

fn reads_second(f: usize) -> bool {
    BITS.iter().any(|&a| out(f, a, 0) != out(f, a, 1))
}

/// Raising an input never lowers the output.
fn monotone(f: usize) -> bool {
    BITS.iter().all(|&a| {
        BITS.iter().all(|&b| {
            BITS.iter().all(|&c| {
                BITS.iter()
                    .all(|&d| !(a <= c && b <= d) || out(f, a, b) <= out(f, c, d))
            })
        })
    })
}

/// A sum modulo 2 of the inputs and a constant: some `c0 ^ c1·a ^ c2·b`. The
/// definition, not the parity of the column that the rule reads.
fn affine(f: usize) -> bool {
    (0..8u8).any(|cs| {
        let (c0, c1, c2) = (cs & 1, cs >> 1 & 1, cs >> 2 & 1);
        BITS.iter().all(|&a| {
            BITS.iter()
                .all(|&b| out(f, a, b) == c0 ^ (c1 & a) ^ (c2 & b))
        })
    })
}

fn dual(f: usize) -> usize {
    named(|a, b| 1 - out(f, 1 - a, 1 - b))
}

/// Every relation `std.bool.props` derives, from the definitions.
fn expected() -> BTreeMap<&'static str, BTreeSet<String>> {
    let mut e: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    let mut put = |rel: &'static str, args: Vec<String>| {
        e.entry(rel).or_default().insert(fact(rel, &args));
    };
    for f in 0..16 {
        let [o00, o01, o10, o11] = column(f);
        put("column", vec![n(f), bit(o00), bit(o01), bit(o10), bit(o11)]);
        put("swap-of", vec![n(f), n(named(|a, b| out(f, b, a)))]);
        put(
            "complement-of",
            vec![n(f), n(named(|a, b| 1 - out(f, a, b)))],
        );
        put("dual-of", vec![n(f), n(dual(f))]);
        put(
            "left-negated",
            vec![n(f), n(named(|a, b| out(f, 1 - a, b)))],
        );
        put(
            "right-negated",
            vec![n(f), n(named(|a, b| out(f, a, 1 - b)))],
        );
        let mut all_points = true;
        for a in BITS {
            for b in BITS {
                for c in BITS {
                    if assoc_at(f, a, b, c) {
                        put("assoc-at", vec![n(f), bit(a), bit(b), bit(c)]);
                    } else {
                        all_points = false;
                    }
                }
            }
        }
        let one =
            |rel: &'static str, holds: bool, put: &mut dyn FnMut(&'static str, Vec<String>)| {
                if holds {
                    put(rel, vec![n(f)]);
                }
            };
        one("commutative", commutative(f), &mut put);
        one("associative", all_points, &mut put);
        one(
            "idempotent",
            BITS.iter().all(|&a| out(f, a, a) == a),
            &mut put,
        );
        one("reads-first", reads_first(f), &mut put);
        one("reads-second", reads_second(f), &mut put);
        one("ignores-first", !reads_first(f), &mut put);
        one("ignores-second", !reads_second(f), &mut put);
        let (z, o, m, af, sd) = (
            out(f, 0, 0) == 0,
            out(f, 1, 1) == 1,
            monotone(f),
            affine(f),
            dual(f) == f,
        );
        one("preserves-zero", z, &mut put);
        one("preserves-one", o, &mut put);
        one("monotone", m, &mut put);
        one("affine", af, &mut put);
        one("self-dual", sd, &mut put);
        one("escapes-zero", !z, &mut put);
        one("escapes-one", !o, &mut put);
        one("escapes-monotone", !m, &mut put);
        one("escapes-affine", !af, &mut put);
        one("escapes-self-dual", !sd, &mut put);
        one("complete-alone", !z && !o && !m && !af && !sd, &mut put);
        for e in BITS {
            let right_id = BITS.iter().all(|&a| out(f, a, e) == a);
            let left_id = BITS.iter().all(|&a| out(f, e, a) == a);
            let right_abs = BITS.iter().all(|&a| out(f, a, e) == e);
            let left_abs = BITS.iter().all(|&a| out(f, e, a) == e);
            for (rel, holds) in [
                ("right-identity-of", right_id),
                ("left-identity-of", left_id),
                ("identity-of", right_id && left_id),
                ("right-absorbing-of", right_abs),
                ("left-absorbing-of", left_abs),
                ("absorbing-of", right_abs && left_abs),
            ] {
                if holds {
                    put(rel, vec![n(f), bit(e)]);
                }
            }
        }
    }
    e
}

/// The thirty relations and the step, in the module's order.
const PROPS: [&str; 31] = [
    "column",
    "swap-of",
    "complement-of",
    "dual-of",
    "left-negated",
    "right-negated",
    "commutative",
    "associative",
    "idempotent",
    "reads-first",
    "reads-second",
    "ignores-first",
    "ignores-second",
    "right-identity-of",
    "left-identity-of",
    "identity-of",
    "right-absorbing-of",
    "left-absorbing-of",
    "absorbing-of",
    "preserves-zero",
    "preserves-one",
    "monotone",
    "affine",
    "self-dual",
    "escapes-zero",
    "escapes-one",
    "escapes-monotone",
    "escapes-affine",
    "escapes-self-dual",
    "complete-alone",
    "assoc-at",
];

// ── running a program ──────────────────────────────────────────────

/// Saturate `src` and return every fact of each relation of `relations`, as
/// text with every stdlib prefix taken off.
fn derive(src: &str, relations: &[&str]) -> BTreeMap<String, BTreeSet<String>> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<bool>")).expect("the program parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the program loads");
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the modules compile");
        sat.saturate(&mut s, None, &mut |_| {})
            .expect("the modules reach their fixpoint");
    }
    relations
        .iter()
        .map(|&r| {
            let facts = ein_corpus::spellings(r)
                .iter()
                .find_map(|s| terms.syms.get(s))
                .map(|rel| {
                    kb.facts_of(rel)
                        .map(|f| ein_corpus::unprefixed(&sexpr(&terms, f)))
                        .collect()
                })
                .unwrap_or_default();
            (r.to_string(), facts)
        })
        .collect()
}

/// What a program that imports `std.bool.props`, and so the table, derives.
fn derived_props() -> BTreeMap<String, BTreeSet<String>> {
    derive("(import std.bool.props)\n", &PROPS)
}

// ── the module's text ──────────────────────────────────────────────

struct Module {
    ast: Ast,
    forms: Vec<NodeId>,
}

fn module(name: &'static str) -> Module {
    let path = repo_root().join(format!("stdlib/bool/{name}.ein"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).unwrap_or_else(|e| panic!("{name}: {e:?}"));
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

    /// Every top-level fact of `relation`, as written.
    fn facts(&self, relation: &str) -> Vec<String> {
        self.forms
            .iter()
            .filter(|&&f| self.ast.head_name(f) == Some(relation))
            .map(|&f| {
                let args: Vec<String> = self
                    .ast
                    .form_args(f)
                    .iter()
                    .map(|&a| {
                        self.ast
                            .atom_name(a)
                            .unwrap_or_else(|| panic!("{relation}: a nested argument"))
                            .to_string()
                    })
                    .collect();
                fact(relation, &args)
            })
            .collect()
    }

    /// The first name each `(import …)` form writes.
    fn imports(&self) -> Vec<String> {
        self.forms
            .iter()
            .filter(|&&f| self.ast.head_name(f) == Some("import"))
            .map(|&f| {
                self.ast
                    .atom_name(self.ast.form_args(f)[0])
                    .expect("a module")
                    .to_string()
            })
            .collect()
    }
}

// ── the claims ─────────────────────────────────────────────────────

/// **The table**, as written in `stdlib/bool/gates.ein`: 64 rows, four a
/// function, each the function's column at its inputs, none missing and none
/// twice, compared as sorted lists so that a duplicate fails as a missing row
/// does; each function's `column-name` its column; and the three small
/// tables the property rules read.
#[test]
fn the_table_is_the_sixteen_columns() {
    let m = module("gates");
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    let mut rows = Vec::new();
    let mut names = Vec::new();
    for f in 0..16 {
        for a in BITS {
            for b in BITS {
                rows.push(fact("gate2", &[n(f), bit(a), bit(b), bit(out(f, a, b))]));
            }
        }
        let col: String = column(f).iter().map(|b| b.to_string()).collect();
        names.push(fact("column-name", &[n(f), format!("f{col}")]));
    }
    assert_eq!(sorted(m.facts("gate2")), sorted(rows), "gate2's 64 rows");
    assert_eq!(
        sorted(m.facts("column-name")),
        sorted(names),
        "each function's second name is its column"
    );
    assert_eq!(sorted(m.facts("inv")), ["(inv 0 1)", "(inv 1 0)"]);
    assert_eq!(sorted(m.facts("le")), ["(le 0 0)", "(le 0 1)", "(le 1 1)"]);
    assert_eq!(sorted(m.facts("bit")), ["(bit 0)", "(bit 1)"]);
    // Every column has a name, and no name two columns.
    let distinct: BTreeSet<&str> = NAMES.iter().copied().collect();
    assert_eq!(distinct.len(), 16, "sixteen names, one a column");
}

/// **The second layout** (reply 19 (c)): a program that imports the module
/// derives `(And A B OUT)` exactly where `(gate2 And A B OUT)` holds, for all
/// sixteen, and no other fact of those relations.
#[test]
fn each_relation_of_a_function_is_its_rows() {
    let got = derive("(import std.bool.gates)\n", &NAMES);
    for f in 0..16 {
        let want: BTreeSet<String> = BITS
            .iter()
            .flat_map(|&a| {
                BITS.iter()
                    .map(move |&b| fact(NAMES[f], &[bit(a), bit(b), bit(out(f, a, b))]))
            })
            .collect();
        assert_eq!(got[NAMES[f]], want, "{}'s four rows", NAMES[f]);
    }
}

/// **Every property, against its definition.** The rules of
/// `std.bool.props` read the 64 rows; the enumeration reads the columns and
/// each property's definition: `affine` as *some* `c0 ^ c1·a ^ c2·b`, not the
/// parity the rule reads off `Xor`'s rows, `monotone` over every pair of
/// points, `self-dual` as its own dual. Relation by relation, the derived
/// facts are the enumeration's, none more and none fewer.
#[test]
fn every_property_is_its_definition() {
    let got = derived_props();
    let want = expected();
    assert_eq!(
        want.keys().copied().collect::<BTreeSet<_>>(),
        PROPS.iter().copied().collect::<BTreeSet<_>>(),
        "the enumeration covers the module's relations"
    );
    for rel in PROPS {
        let w = want.get(rel).cloned().unwrap_or_default();
        let g = &got[rel];
        let missing: Vec<&String> = w.difference(g).collect();
        let extra: Vec<&String> = g.difference(&w).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "{rel}: the rules miss {missing:?} and derive {extra:?} beyond the definition"
        );
    }
}

/// **The three counts the user confirmed** (reply 22 (a) of 2026-11-12, to
/// three corrections of the stage file's hand counts), on what the rules
/// derive: four swap pairs, and eight functions their own swap; the duals
/// `Andn`/`Orn` and `Randn`/`Rorn`, where the stage file had written the
/// complements; and twelve functions up to the order of the inputs.
#[test]
fn the_three_corrected_counts() {
    let got = derived_props();
    let pairs = |rel: &str| -> Vec<(String, String)> {
        got[rel]
            .iter()
            .map(|f| {
                let inner = f.trim_start_matches('(').trim_end_matches(')');
                let parts: Vec<&str> = inner.split(' ').collect();
                (parts[1].to_string(), parts[2].to_string())
            })
            .collect()
    };
    let swaps = pairs("swap-of");
    let mut two: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    let mut own: BTreeSet<String> = BTreeSet::new();
    for (f, g) in &swaps {
        if f == g {
            own.insert(f.clone());
        } else {
            two.insert([f.clone(), g.clone()].into_iter().collect());
        }
    }
    let want_pairs: BTreeSet<BTreeSet<String>> = [
        ["Andn", "Randn"],
        ["Fst", "Snd"],
        ["Nfst", "Nsnd"],
        ["Orn", "Rorn"],
    ]
    .iter()
    .map(|p| p.iter().map(|s| s.to_string()).collect())
    .collect();
    assert_eq!(two, want_pairs, "four swap pairs");
    assert_eq!(
        own,
        ["And", "Nand", "Nor", "One", "Or", "Xnor", "Xor", "Zero"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        "eight functions their own swap"
    );
    assert_eq!(
        two.len() + own.len(),
        12,
        "twelve functions up to the order of the inputs"
    );

    let duals: BTreeSet<(String, String)> = pairs("dual-of").into_iter().collect();
    let p = |a: &str, b: &str| (a.to_string(), b.to_string());
    for (a, b) in [("Andn", "Orn"), ("Randn", "Rorn")] {
        assert!(
            duals.contains(&p(a, b)) && duals.contains(&p(b, a)),
            "{a} and {b} are each other's dual"
        );
    }
    for (a, b) in [("Andn", "Rorn"), ("Randn", "Orn")] {
        assert!(
            !duals.contains(&p(a, b)),
            "{a} and {b} are complements, not duals"
        );
        let complements: BTreeSet<(String, String)> = pairs("complement-of").into_iter().collect();
        assert!(
            complements.contains(&p(a, b)),
            "{a} and {b} are complements"
        );
    }
}

/// **The one-sided forms** (reply 21 (b)): each of the four one-sided
/// relations is derived, eight facts over seven functions, since `Fst` or
/// `Snd` has both bits on its free side; the two-sided forms are their
/// meets, four each. `Andn`, a and not b, has a right identity, 0, and no
/// left one.
#[test]
fn the_one_sided_forms_are_derived() {
    let got = derived_props();
    for (one_sided, two_sided) in [
        (["right-identity-of", "left-identity-of"], "identity-of"),
        (["right-absorbing-of", "left-absorbing-of"], "absorbing-of"),
    ] {
        for rel in one_sided {
            let functions: BTreeSet<String> = got[rel]
                .iter()
                .map(|f| f.split(' ').nth(1).expect("a function").to_string())
                .collect();
            assert_eq!(
                (got[rel].len(), functions.len()),
                (8, 7),
                "{rel}: facts, functions"
            );
        }
        let meet: BTreeSet<String> = got[one_sided[0]]
            .iter()
            .map(|f| f.replacen(one_sided[0], two_sided, 1))
            .filter(|f| got[one_sided[1]].contains(&f.replacen(two_sided, one_sided[1], 1)))
            .collect();
        assert_eq!(got[two_sided], meet, "{two_sided} is both sides");
        assert_eq!(got[two_sided].len(), 4, "{two_sided}: four facts");
    }
    assert!(got["right-identity-of"].contains("(right-identity-of Andn 0)"));
    assert!(
        !got["left-identity-of"]
            .iter()
            .any(|f| f.starts_with("(left-identity-of Andn ")),
        "Andn has no left identity"
    );
}

/// **The closing rules.** Each module's entry closes every relation the
/// module declares, but `std.bool.inputs`' `in`, the hypothesis; names every
/// other rule of the module, and the entry of the module it imports; and the
/// module writes its entry's own name as a fact, as the calculator's
/// modules do (E-11).
#[test]
fn each_closing_rule_names_every_rule_and_closes_every_relation() {
    for (name, entry, named_by, below, open) in [
        ("gates", "gates", "gates-rule", None, None),
        ("props", "props", "props-rule", Some("gates"), None),
        ("circuit", "circuit", "circuit-rule", Some("gates"), None),
        (
            "inputs",
            "inputs",
            "inputs-rule",
            Some("circuit"),
            Some("in"),
        ),
    ] {
        let m = module(name);
        let rules = m.declared("rule");
        let (_, closer) = rules
            .iter()
            .find(|(n, _)| n == entry)
            .unwrap_or_else(|| panic!("{name}: the closing rule `{entry}`"));
        assert!(
            m.conjuncts(m.clause(*closer, "match")).is_empty(),
            "{name}: `{entry}` fires at root, on nothing"
        );
        let (mut closed, mut listed) = (BTreeSet::new(), BTreeSet::new());
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
                Some(h) if h == named_by => {
                    assert!(listed.insert(arg.clone()), "named twice: {arg}");
                }
                other => panic!("{name}: `{entry}` asserts a {other:?}"),
            }
        }
        let want_closed: BTreeSet<String> = m
            .declared("relation")
            .into_iter()
            .map(|(n, _)| n)
            .filter(|n| Some(n.as_str()) != open)
            .collect();
        assert_eq!(
            closed,
            want_closed,
            "{name}: `{entry}` closes every relation of the module{}",
            open.map(|o| format!(" but `{o}`")).unwrap_or_default()
        );
        let mut want_named: BTreeSet<String> = rules
            .iter()
            .map(|(n, _)| n.clone())
            .filter(|n| n != entry)
            .collect();
        if let Some(b) = below {
            want_named.insert(b.to_string());
        }
        assert_eq!(
            listed,
            want_named,
            "{name}: `{entry}` names every other rule{}",
            below.map(|b| format!(", and `{b}`")).unwrap_or_default()
        );
        assert_eq!(
            m.facts(named_by),
            [fact(named_by, &[entry.to_string()])],
            "{name}: the module writes its entry's name, once"
        );
        let want_imports: &[&str] = match name {
            "gates" => &[],
            "props" | "circuit" => &["std.bool.gates"],
            _ => &["std.bool.circuit", "std.algebra"],
        };
        assert_eq!(m.imports(), want_imports, "{name}'s imports");
    }
}

/// **Monotone**: no `absent`, no `not` and no `or` in any rule of the four
/// modules, so each is one stratum (a program that imports `std.bool.inputs`
/// holds `std.algebra`'s `total-owed`, whose `absent` is the obligation's);
/// no hrule and no macro, nothing guessed but what an obligation proposes;
/// and `neq` and `(false)` only in the two rules that refute, `one-value`
/// and `required`.
#[test]
fn the_modules_write_no_negation() {
    let refuting = ["one-value", "required"];
    let mut seen = Vec::new();
    for name in ["gates", "props", "circuit", "inputs"] {
        let m = module(name);
        assert!(
            m.declared("hrule").is_empty() && m.declared("macro").is_empty(),
            "{name} guesses nothing and expands nothing"
        );
        for (rule, node) in m.declared("rule") {
            let mut heads = Vec::new();
            m.heads(m.clause(node, "match"), &mut heads);
            m.heads(m.clause(node, "assert"), &mut heads);
            for banned in ["absent", "not", "or", "eq"] {
                assert!(
                    !heads.iter().any(|h| h == banned),
                    "{name}: rule {rule} writes `{banned}`"
                );
            }
            let refutes = heads.iter().any(|h| h == "false" || h == "neq");
            assert_eq!(
                refutes,
                refuting.contains(&rule.as_str()),
                "{name}: rule {rule}: `neq` and `(false)` are the refuting rules' alone"
            );
            if refutes {
                seen.push(rule);
            }
        }
    }
    seen.sort();
    assert_eq!(seen, ["one-value", "required"]);
}

/// `utils/gen_gates.py --check`, run here so that it is in the gate: the
/// per-commit tier runs `cargo test --workspace`, and its job installs a
/// `python3` for `cli_semantics.rs`'s `gen_zebra2_variants.py --check`, the
/// same shape as `ein-ir/tests/digit_tables.rs`' (Q-M1f.145).
#[test]
fn the_files_are_what_the_generator_writes() {
    let probe = Command::new("python3")
        .args(["-c", "pass"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    assert!(
        matches!(probe, Ok(s) if s.success()),
        "`python3` does not run ({probe:?}): `stdlib/bool/gates.ein`'s table and the programs \
         under `tests/stdlib/bool/` are generated, and `utils/gen_gates.py --check` is the only \
         check that compares their bytes with the script's — a missing gate, not a missing \
         convenience"
    );
    let out = Command::new("python3")
        .args(["utils/gen_gates.py", "--check"])
        .current_dir(repo_root())
        .output()
        .expect("python3 ran a moment ago");
    assert!(
        out.status.success(),
        "a file `utils/gen_gates.py` writes is stale — run it, then \
         `python3 utils/stdlib_manifest.py --write`\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
