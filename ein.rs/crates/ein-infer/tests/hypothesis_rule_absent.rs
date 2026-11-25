//! **A rule that concludes a hypothesis reads its `absent` premises**
//! (Q-M1f.685): a match of a rule that concludes `(open F)` is one whose
//! `(absent …)` premises hold against the state, as a match of a rule that
//! concludes a fact is.
//!
//! The function that reads a hypothesis rule's matches
//! (`ein_infer::HypothesisRules::candidates`) builds $H$ for every reader, the
//! oracle's cube among them, so agreeing with the oracle proves nothing about
//! it. It is held here by a **second route** that does not read it: a rule
//! that concludes a fact over the same `:match`, `(s …)` beside `(open (r
//! …))`, which the saturator fires, judging its guards at the boundary. On
//! each program below, monotone by the static check and reading under its
//! `absent`s only relations no commitment changes:
//!
//! - at root, $H$'s facts of `r` are the facts of `s` root holds, renamed;
//! - at every node of the oracle's cube, the node's `open` facts of `r` are
//!   its facts of `s`, renamed: the pass that writes a state's `open` facts
//!   reads the match as the saturator does;
//! - one rule that concludes both, `(and (s ?x) (open (r ?x)))`, gives both
//!   of the same matches.
//!
//! And where the `absent` reads a relation a commitment can change, the
//! hypothesis's own, a state above root holds fewer hypotheses than root, and
//! none root does not, so the static check names no site there and the
//! program is monotone (`a_commitment_takes_a_hypothesis_away_and_adds_none`).

use std::path::PathBuf;

use ein_core::{FactId, Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::region::{self, Nodes, Options, Outcome};
use ein_ir::{Ast, load, load_file, parse};

/// The corpus witness: one `:match`, `(and (p ?x) (absent (q ?x)))`, under a
/// rule that concludes `(s ?x)` and one that concludes `(open (r ?x))`.
const WITNESS: &str = "examples/ein-bugs/wrong-answers/hypothesis-rule-absent.ein";

const DECLARED: &str = r#"
(relation p T :why "{?3} is p")
(relation q T :why "{?3} is q")
(relation r T :why "{?3} is r")
(relation s T :why "{?3} is s")
"#;

/// Each program's own forms, after [`DECLARED`]: a name, and the text.
const SHAPES: &[(&str, &str)] = &[
    (
        // The `absent` reads a relation a rule derives at root.
        "derived",
        r#"
(relation t T :why "{?3} is t")
(p a) (p b) (t a)
(rule mk () :match (t ?x) :assert (q ?x) :why "{?x} is t, so q")
(rule guess () :match (and (p ?x) (absent (q ?x))) :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (and (p ?x) (absent (q ?x))) :assert (s ?x) :why "{?x} is s")
"#,
    ),
    (
        // A nested `absent`, a `forall`'s shape: every e of `?x` is ok.
        "nested",
        r#"
(relation e T T :why "{?3} leads to {?4}")
(relation ok T :why "{?3} is ok")
(p a) (p b) (e a c) (e b d) (ok c)
(rule guess () :match (and (p ?x) (absent (and (e ?x ?y) (absent (ok ?y)))))
  :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (and (p ?x) (absent (and (e ?x ?y) (absent (ok ?y)))))
  :assert (s ?x) :why "{?x} is s")
"#,
    ),
    (
        // Two disjuncts, one guarded and one not.
        "disjuncts",
        r#"
(relation k T :why "{?3} is k")
(p a) (p b) (p c) (q a) (q b) (k b)
(rule guess () :match (or (and (p ?x) (absent (q ?x))) (and (p ?x) (k ?x)))
  :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (or (and (p ?x) (absent (q ?x))) (and (p ?x) (k ?x)))
  :assert (s ?x) :why "{?x} is s")
"#,
    ),
    (
        // The `absent` before the premise that binds its variable: is there
        // no q at all? There is one, so nothing matches; the written `open`
        // fact keeps the blind enumerator out of H.
        "before-its-binding",
        r#"
(relation u T :why "{?3} is u")
(p a) (p b) (q a)
(open (u z))
(rule guess () :match (and (absent (q ?y)) (p ?x)) :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (and (absent (q ?y)) (p ?x)) :assert (s ?x) :why "{?x} is s")
"#,
    ),
    (
        // `std.macro`'s `unknown`: neither the fact nor its stored negation.
        "unknown",
        r#"
(import std.macro :symbols (unknown))
(p a) (p b) (p c) (q a) (not (q b))
(rule guess () :match (and (p ?x) (unknown (q ?x))) :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (and (p ?x) (unknown (q ?x))) :assert (s ?x) :why "{?x} is s")
"#,
    ),
    (
        // One rule that concludes a fact and a hypothesis.
        "both",
        r#"
(p a) (p b) (q a)
(rule both () :match (and (p ?x) (absent (q ?x))) :assert (and (s ?x) (open (r ?x)))
  :why "{?x} is s, and suppose it is r")
"#,
    ),
];

fn load_rel(rel: &str) -> (Ast, Terms, Kb) {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    (ast, terms, kb)
}

fn load_text(name: &str, text: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
    let kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{name}: {e}"));
    (ast, terms, kb)
}

/// The oracle's cube.
fn cube(name: &str, ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> region::Region {
    match region::compute(kb, terms, ast, &Options::default(), &mut |_| {}) {
        Ok(Outcome::Region(r)) => *r,
        _ => panic!("{name}: the cube is within the limit"),
    }
}

/// `(s b)` as `(r b)`: a fact of the rule that concludes a fact, as the
/// hypothesis the same match gives.
fn renamed(text: &str) -> Option<String> {
    text.strip_prefix("(s ").map(|rest| format!("(r {rest}"))
}

/// `(open (r b))`'s hypothesis, `(r b)`; `None` for an `open` fact of another
/// relation.
fn hypothesis(text: &str) -> Option<String> {
    let inner = text.strip_prefix("(open ")?.strip_suffix(')')?;
    inner.starts_with("(r ").then(|| inner.to_string())
}

/// At each node of `sets`, its `open` facts of `r` and its facts of `s`
/// renamed, each sorted.
fn at_nodes(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    sets: &[Vec<FactId>],
) -> Vec<(Vec<String>, Vec<String>)> {
    let mut nodes = Nodes::new(kb, terms, ast, &[]).expect("prepares");
    nodes
        .with_fixpoints(sets, |kbs, terms| {
            kbs.iter()
                .map(|k| {
                    let mut open: Vec<String> = k
                        .open_facts()
                        .iter()
                        .filter_map(|&f| hypothesis(&terms.fact_text(f)))
                        .collect();
                    let mut matched: Vec<String> = k
                        .facts()
                        .filter_map(|f| renamed(&terms.fact_text(f)))
                        .collect();
                    open.sort();
                    matched.sort();
                    (open, matched)
                })
                .collect()
        })
        .expect("every fixpoint")
}

/// $H$ held to the match, at root and at every node of the cube; returns
/// $H$'s facts of `r`, sorted.
fn held_to_the_match(name: &str, load: &dyn Fn() -> (Ast, Terms, Kb)) -> Vec<String> {
    let (ast, mut terms, mut kb) = load();
    let r = cube(name, &ast, &mut terms, &mut kb);
    assert!(
        r.monotonicity.is_monotone(),
        "{name}: written monotone, its sites {:?}",
        r.monotonicity.sites
    );
    let mut h: Vec<String> = r
        .candidates
        .iter()
        .map(|c| c.text.clone())
        .filter(|t| t.starts_with("(r "))
        .collect();
    h.sort();
    let n = r.candidates.len();
    let sets: Vec<Vec<FactId>> = (0..1u64 << n)
        .map(|m| {
            (0..n)
                .filter(|i| m >> i & 1 == 1)
                .map(|i| r.candidates[i].fact)
                .collect()
        })
        .collect();
    let (ast, mut terms, mut kb) = load();
    let nodes = at_nodes(&ast, &mut terms, &mut kb, &sets);
    let (_, at_root) = &nodes[0];
    assert_eq!(
        &h, at_root,
        "{name}: H's facts of r are the facts of s root holds, renamed"
    );
    for (m, (open, matched)) in nodes.iter().enumerate() {
        assert_eq!(
            open, matched,
            "{name}, node {m}: its open facts of r are its facts of s, renamed"
        );
    }
    // Nothing refutes and nothing is owed: R is every commitment.
    assert!(
        (0..1u64 << n).all(|m| r.in_r(m)),
        "{name}: every commitment is alive and owes nothing"
    );
    h
}

#[test]
fn the_witness_proposes_what_its_match_gives() {
    let h = held_to_the_match(WITNESS, &|| load_rel(WITNESS));
    assert_eq!(h, ["(r b)"], "(q a) is written: the match holds of b alone");
}

#[test]
fn each_shape_proposes_what_its_match_gives() {
    let expected: &[(&str, &[&str])] = &[
        ("derived", &["(r b)"]),
        ("nested", &["(r a)"]),
        ("disjuncts", &["(r b)", "(r c)"]),
        ("before-its-binding", &[]),
        ("unknown", &["(r c)"]),
        ("both", &["(r b)"]),
    ];
    for (name, body) in SHAPES {
        let text = format!("{DECLARED}{body}");
        let h = held_to_the_match(name, &|| load_text(name, &text));
        let want = expected
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, w)| *w)
            .expect("each shape states its H");
        assert_eq!(h, want, "{name}: H by hand");
    }
}

/// The `absent` reads the hypothesis's own relation, which a commitment
/// changes: root proposes `(r a)` and `(r b)`; a state that holds `(r a)`
/// proposes `(r b)` alone, and no state proposes what root does not, so the
/// static check names no site (a hypothesis rule's site is a read at even
/// depth, where growing what is read adds a match) and $R$ is every
/// commitment over root's $H$.
#[test]
fn a_commitment_takes_a_hypothesis_away_and_adds_none() {
    let text = format!(
        "{DECLARED}{}",
        r#"
(p a) (p b)
(rule guess () :match (and (p ?x) (absent (r ?x))) :assert (open (r ?x)) :why "suppose {?x} is r")
"#
    );
    let (ast, mut terms, mut kb) = load_text("own-relation", &text);
    let r = cube("own-relation", &ast, &mut terms, &mut kb);
    assert!(
        r.monotonicity.is_monotone(),
        "no site: {:?}",
        r.monotonicity.sites
    );
    let h: Vec<&str> = r.candidates.iter().map(|c| c.text.as_str()).collect();
    assert_eq!(h, ["(r a)", "(r b)"], "root proposes both");
    assert!((0..4).all(|m| r.in_r(m)), "R is every commitment");
    let sets: Vec<Vec<FactId>> = (0..4u64)
        .map(|m| {
            (0..2)
                .filter(|i| m >> i & 1 == 1)
                .map(|i| r.candidates[i].fact)
                .collect()
        })
        .collect();
    let (ast, mut terms, mut kb) = load_text("own-relation", &text);
    let open: Vec<Vec<String>> = at_nodes(&ast, &mut terms, &mut kb, &sets)
        .into_iter()
        .map(|(open, _)| open)
        .collect();
    assert_eq!(
        open,
        [
            vec!["(r a)".to_string(), "(r b)".to_string()],
            vec!["(r b)".to_string()],
            vec!["(r a)".to_string()],
            vec![],
        ],
        "each state proposes what its match gives, none of what it holds"
    );
}
