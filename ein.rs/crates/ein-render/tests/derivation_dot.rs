//! S1a.2.4 acceptance — `DerivationDAG.to_dot` byte-for-byte.
//!
//! The fixture is the one `ein.py/tests/render/test_golden_dot.py` captured
//! `kb_provenance_dag.dot` from, rebuilt against the port's data model. The
//! golden is the *committed* one, read from `ein.py/`: a port that ships its
//! own copy of the expected bytes proves only that it agrees with itself.

use ein_core::{Justifications, Kb, Program, Prov, Relation, Terms, Value, build_derivation_dag};
use ein_oracle::repo_root;
use ein_render::derivation_dag_to_dot;

/// `(relation p T T) (relation q T T) (is-a a T) … (p a b :source "(1)")
/// (p b c :source "(2)")`, plus a `(q a c)` derived by `triangle` from both.
fn fixture() -> (Terms, Kb, ein_core::FactId) {
    let mut terms = Terms::new();
    let mut program = Program::new();
    let t = terms.intern_text("T").expect("room");
    for name in ["p", "q"] {
        let name = terms.intern_text(name).expect("room");
        program.add_relation(Relation {
            name,
            signature: Box::new([t, t]),
            declared: true,
            why: None,
            loc: None,
        });
    }
    let is_a = terms.intern_text("is-a").expect("room");
    program.add_relation(Relation {
        name: is_a,
        signature: Box::new([]),
        declared: false,
        why: None,
        loc: None,
    });
    let mut kb = Kb::new(program);

    let add = |kb: &mut Kb, terms: &mut Terms, rel: &str, args: &[&str], source: Option<&str>| {
        let rel = terms.intern_text(rel).expect("room");
        let args: Vec<Value> = args
            .iter()
            .map(|a| terms.value_text(a).expect("room"))
            .collect();
        let source = source.map(|s| terms.intern_text(s).expect("room"));
        let prov = terms.provs.push(Prov::from_source(source, None));
        kb.add_and_index_fact(terms, rel, &args, Some(prov))
            .expect("room")
            .id()
    };
    for name in ["a", "b", "c"] {
        add(&mut kb, &mut terms, "is-a", &[name, "T"], None);
    }
    let p_ab = add(&mut kb, &mut terms, "p", &["a", "b"], Some("(1)"));
    let p_bc = add(&mut kb, &mut terms, "p", &["b", "c"], Some("(2)"));

    let q = terms.intern_text("q").expect("room");
    let triangle = terms.intern_text("triangle").expect("room");
    let args = [
        terms.value_text("a").expect("room"),
        terms.value_text("c").expect("room"),
    ];
    let prov = terms
        .provs
        .push(Prov::from_rule(triangle, Box::new([p_ab, p_bc]), None));
    let derived = kb
        .add_and_index_fact(&mut terms, q, &args, Some(prov))
        .expect("room")
        .id();
    (terms, kb, derived)
}

#[test]
fn the_derivation_dag_renders_the_committed_golden() {
    let (terms, kb, derived) = fixture();
    let dag = build_derivation_dag(&kb, &terms, derived, Justifications::Primary);
    let got = derivation_dag_to_dot(&dag, &kb, &terms);
    let golden = repo_root()
        .join("ein.rs/crates/ein-render/tests/golden/from_ein_py/dot/kb_provenance_dag.dot");
    let want = std::fs::read_to_string(&golden).expect("the committed golden");
    assert_eq!(got, want, "\n--- ein.rs ---\n{got}\n--- ein.py ---\n{want}");
}

#[test]
fn an_or_graph_draws_a_diamond_per_justification() {
    let (mut terms, mut kb, derived) = fixture();
    // A second derivation of `(q a c)`, from one premise instead of two.
    let other = terms.intern_text("shortcut").expect("room");
    let premise = kb.facts().next().expect("a fact");
    let alt = terms
        .provs
        .push(Prov::from_rule(other, Box::new([premise]), None));
    assert!(kb.record_justification(&terms, derived, alt));

    let dag = build_derivation_dag(&kb, &terms, derived, Justifications::All);
    assert!(dag.is_or_graph());
    let dot = derivation_dag_to_dot(&dag, &kb, &terms);
    assert_eq!(dot.matches("shape=diamond").count(), 2, "{dot}");
    assert!(dot.contains(" -> j0;"), "{dot}");
    assert!(dot.contains("j1 -> "), "{dot}");

    // The primary-only view is unchanged by the alternative existing.
    let primary = build_derivation_dag(&kb, &terms, derived, Justifications::Primary);
    assert!(!primary.is_or_graph());
    assert!(!derivation_dag_to_dot(&primary, &kb, &terms).contains("diamond"));
}
