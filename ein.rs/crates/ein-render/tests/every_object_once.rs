//! **Every drawing of facts draws an object once**
//! (Q-M1f.395).
//!
//! A fact that a drawn fact points at is its own node, an octagon with the
//! relation's name in it and its argument arrows numbered: never a node
//! labelled with the fact's text, and never an arrow beside its node. Where
//! the knowledge base does not hold it, in a context only, it stands in a
//! dashed frame named for the relations of the facts that point at it; a held
//! one stands in no frame, whatever points at it, and no octagon is filled
//! (Q-M1f.398). A held fact that nothing points at is drawn
//! as it was.
//!
//! The four writers that draw facts are held to it here on one small
//! knowledge base and one small rule, as claims a reader of the DOT would
//! state: the KB view (`ein render kb`), the program view (`ein render
//! program`), the rule view (`ein render rule`, both modes) and the
//! constraint view. What the bytes of every corpus program's views are is
//! `id_order_invariance.rs`'s, and that Graphviz takes them
//! `dot_wellformed.rs`'s.

use ein_core::Terms;
use ein_ir::{Ast, NodeId, parse};
use ein_render::ir_dot::{DotOpts, to_dot};
use ein_render::kb_dot::{KbDotOpts, to_dot as kb_to_dot};
use ein_render::render_constraints;
use ein_render::rules::{RuleMode, render_rule_form};

/// One fact, `(= 5 (+ 2 2))`, held and in the contexts of two relations;
/// one, `(+ 2 2)`, in a context only, not held; one, `(= 4 (+ 2 2))`, held
/// and under no arrow.
const KB: &str = "\
(think Alice (= 5 (+ 2 2)))
(says Bob (= 5 (+ 2 2)))
(= 5 (+ 2 2))
(= 4 (+ 2 2))
";

fn parsed(text: &str) -> (Ast, Vec<NodeId>) {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
    (ast, forms)
}

fn kb_view(text: &str) -> String {
    let (mut ast, forms) = parsed(text);
    let mut terms = Terms::new();
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    kb_to_dot(&kb, &terms, &KbDotOpts::default())
}

/// The lines of `dot` that declare a node: an id and its attributes, no
/// arrow.
fn node_lines(dot: &str) -> Vec<&str> {
    dot.lines()
        .map(str::trim)
        .filter(|l| l.contains(" [") && !l.contains(" -> "))
        .collect()
}

/// The id a node line declares.
fn id_of(line: &str) -> &str {
    line.split(" [").next().expect("an id")
}

/// The frame a node is declared in: the label of the `cluster_ctx_…`
/// subgraph whose braces hold its declaration.
fn frame_of(dot: &str, id: &str) -> Option<String> {
    let mut frame: Option<String> = None;
    let mut open = false;
    for line in dot.lines().map(str::trim) {
        if line.starts_with("subgraph cluster_ctx_") {
            open = true;
            frame = None;
        } else if open && line.starts_with("label=") {
            let label = line.split('"').nth(1).expect("a quoted label");
            frame = Some(label.to_string());
        } else if open && line == "}" {
            open = false;
            frame = None;
        } else if open && line.starts_with(&format!("{id} [")) {
            return frame;
        }
    }
    None
}

/// No node of the drawing is filled: *held* is *outside every frame*.
fn nothing_is_filled(dot: &str) {
    assert!(!dot.contains("filled"), "a filled node:\n{dot}");
}

/// No node of the drawing is labelled with a fact's text.
fn no_label_is_a_facts_text(dot: &str) {
    for line in dot.lines() {
        assert!(
            !line.contains("label=\"("),
            "a label that is a fact's text, or a parenthesised one a nested fact could be read \
             as: {line}"
        );
    }
}

#[test]
fn the_kb_view_draws_a_fact_in_two_contexts_once() {
    let dot = kb_view(KB);
    // The KB view writes a relation's name in an octagon in parentheses, so
    // its labels are read by what stands after the opening one.
    let nodes = node_lines(&dot);
    let equalities: Vec<&&str> = nodes.iter().filter(|l| l.contains("label=\"(=)")).collect();
    assert_eq!(
        equalities.len(),
        1,
        "one node for the one fact of = that a fact points at:\n{dot}"
    );
    let e = id_of(equalities[0]);
    // Held, and in the contexts of `says` and `think`: in no frame, and no
    // octagon is filled.
    assert_eq!(
        frame_of(&dot, e),
        None,
        "a held fact is in no frame:\n{dot}"
    );
    nothing_is_filled(&dot);
    // And no arrow of that fact: nothing leaves `5` under the label `=`.
    assert!(
        !dot.lines()
            .any(|l| l.trim_start().starts_with("\"5\" -> ") && l.contains("label=\"=\"")),
        "the fact is its node and no arrow:\n{dot}"
    );
    // Both facts that point at it point at the one node.
    for from in ["\"Alice\"", "\"Bob\""] {
        assert!(
            dot.lines()
                .any(|l| l.trim_start().starts_with(&format!("{from} -> {e} "))),
            "{from} points at the node {e}:\n{dot}"
        );
    }
    // Its own arrows are numbered, to a name and to the fact under it.
    let sum = id_of(
        nodes
            .iter()
            .find(|l| l.contains("label=\"(+)"))
            .expect("the node of (+ 2 2)"),
    );
    assert!(
        dot.contains(&format!("{e} -> \"5\" [label=\"#1\"")),
        "{dot}"
    );
    assert!(
        dot.contains(&format!("{e} -> {sum} [label=\"#2\"")),
        "{dot}"
    );
    // `(+ 2 2)` is in a context only, not held: one node, in the frame of
    // the relation that points at it, and the one frame of the drawing.
    let plus: Vec<&&str> = nodes.iter().filter(|l| l.contains("label=\"(+)")).collect();
    assert_eq!(plus.len(), 1, "{dot}");
    assert_eq!(frame_of(&dot, sum).as_deref(), Some("= ctx"), "{dot}");
    assert_eq!(dot.matches("subgraph cluster_ctx_").count(), 1, "{dot}");
    // `(= 4 (+ 2 2))` is held and nothing points at it: an arrow, as it was,
    // to the node of the fact under its third arrow.
    assert!(
        dot.contains(&format!("\"4\" -> {sum} [label=\"=\"")),
        "a held fact that nothing points at is an arrow:\n{dot}"
    );
    // No node is named or labelled by a fact's text.
    for text in ["(= 5", "(= 4", "(+ 2"] {
        assert!(
            !dot.contains(text),
            "the text {text} is in the drawing:\n{dot}"
        );
    }
}

#[test]
fn a_knowledge_base_whose_facts_under_an_arrow_are_all_held_has_no_frame() {
    // `(owns Bob Dog)` is held, and Alice thinks it and Carol says it: one
    // octagon in no frame, and the drawing has none.
    let dot =
        kb_view("(owns Bob Dog)\n(think Alice (owns Bob Dog))\n(says Carol (owns Bob Dog))\n");
    let owns: Vec<&str> = node_lines(&dot)
        .into_iter()
        .filter(|l| l.contains("label=\"(owns)"))
        .collect();
    assert_eq!(owns.len(), 1, "{dot}");
    assert!(!dot.contains("cluster_ctx_"), "{dot}");
    assert!(
        !dot.contains("newrank"),
        "a view with no frame ranks as it did:\n{dot}"
    );
    nothing_is_filled(&dot);
    // Two facts of one relation: the one held stands outside, the one only
    // thought alone in `think ctx`.
    let dot = kb_view(
        "(owns Bob Dog)\n(think Alice (owns Bob Dog))\n(think Dan (owns Bob Dog))\n\
         (think Dan (owns Bob Cat))\n",
    );
    let owns: Vec<&str> = node_lines(&dot)
        .into_iter()
        .filter(|l| l.contains("label=\"(owns)"))
        .collect();
    assert_eq!(owns.len(), 2, "{dot}");
    let framed: Vec<Option<String>> = owns.iter().map(|l| frame_of(&dot, id_of(l))).collect();
    assert_eq!(framed, [None, Some("think ctx".to_string())], "{dot}");
    nothing_is_filled(&dot);
}

#[test]
fn a_knowledge_base_with_no_fact_under_an_arrow_has_no_frame() {
    let dot = kb_view("(lives-in Norwegian House-1)\n(owns Norwegian Dog)\n(triple a b c)\n");
    assert!(!dot.contains("cluster_ctx_"), "{dot}");
    assert!(!dot.contains("filled"), "{dot}");
    assert!(!dot.contains("// facts in a context"), "{dot}");
}

#[test]
fn the_program_view_draws_a_fact_under_an_arrow_as_its_node() {
    let (ast, forms) = parsed(KB);
    for levi in [false, true] {
        let dot = to_dot(
            &ast,
            &forms,
            DotOpts {
                levi,
                ..DotOpts::default()
            },
        );
        no_label_is_a_facts_text(&dot);
        nothing_is_filled(&dot);
        // One node for the fact two facts point at, named for its relation,
        // in no frame since the program states it.
        let e = "\"(= 5 (+ 2 2))\"";
        let declared: Vec<&str> = node_lines(&dot)
            .into_iter()
            .filter(|l| id_of(l) == e)
            .collect();
        assert_eq!(declared.len(), 1, "levi={levi}:\n{dot}");
        assert!(declared[0].contains("shape=octagon, label=\"=\""), "{dot}");
        assert_eq!(frame_of(&dot, e), None, "{dot}");
        assert!(
            dot.contains(&format!("{e} -> \"5\" [label=\"1\"]")),
            "{dot}"
        );
        assert!(
            dot.contains(&format!("{e} -> \"(+ 2 2)\" [label=\"2\"]")),
            "{dot}"
        );
        // The fact under it is in a context only, not stated.
        let sum = node_lines(&dot)
            .into_iter()
            .find(|l| id_of(l) == "\"(+ 2 2)\"")
            .expect("the node of (+ 2 2)");
        assert!(sum.contains("label=\"+\""), "{dot}");
        assert_eq!(
            frame_of(&dot, "\"(+ 2 2)\"").as_deref(),
            Some("= ctx"),
            "{dot}"
        );
    }
}

#[test]
fn a_negation_of_a_fact_that_is_a_node_points_at_the_node() {
    let (ast, forms) = parsed("(think Alice (likes Bob Carol))\n(not (likes Bob Carol))\n");
    let dot = to_dot(&ast, &forms, DotOpts::default());
    let g = "\"(likes Bob Carol)\"";
    // The denied fact is drawn once, as the node `think` points at, and the
    // negation is a node that points at it too; no dashed arrow draws the
    // fact a second time.
    assert_eq!(
        frame_of(&dot, g).as_deref(),
        Some("not, think ctx"),
        "{dot}"
    );
    assert!(
        dot.lines()
            .any(|l| l.contains("_not\" -> ") && l.contains(g)),
        "{dot}"
    );
    assert!(
        !dot.lines()
            .any(|l| l.trim_start().starts_with("\"Bob\" -> \"Carol\"")),
        "{dot}"
    );
    // Alone, a negation is drawn as it was: the denied fact, dashed.
    let (ast, forms) = parsed("(not (likes Bob Carol))\n");
    let alone = to_dot(&ast, &forms, DotOpts::default());
    assert!(alone.contains("\"Bob\" -> \"Carol\""), "{alone}");
    assert!(!alone.contains("cluster_ctx_"), "{alone}");
}

const RULES: &str = "\
(rule trust ()
  :match  (and (reliable ?p) (think ?p (= ?a ?b)))
  :assert (= ?a ?b)
  :why    \"{?p} is reliable\")
(rule noted ()
  :match  (and (think ?p (= ?a ?b)) (= ?a ?b))
  :assert (right ?p)
  :why    \"{?p} is right\")
";

fn rule(ast: &Ast, forms: &[NodeId], name: &str) -> NodeId {
    forms
        .iter()
        .copied()
        .find(|f| {
            ast.form_args(*f)
                .first()
                .is_some_and(|a| ein_render::value_label(ast, *a) == name)
        })
        .expect("the rule")
}

#[test]
fn the_rule_view_draws_what_a_nested_pattern_binds() {
    let (ast, forms) = parsed(RULES);
    let dot = render_rule_form(&ast, rule(&ast, &forms, "trust"), RuleMode::SideBySide);
    no_label_is_a_facts_text(&dot);
    nothing_is_filled(&dot);
    // In the match panel the pattern under `think`'s arrow is a node named
    // for its relation, in the frame `think ctx`, and its two variables are
    // drawn, each under a numbered arrow.
    let u = "\"(= ?a ?b)_L\"";
    let declared: Vec<&str> = node_lines(&dot)
        .into_iter()
        .filter(|l| id_of(l) == u)
        .collect();
    assert_eq!(declared.len(), 1, "{dot}");
    assert!(declared[0].contains("shape=octagon, label=\"=\""), "{dot}");
    assert_eq!(
        frame_of(&dot, u).as_deref(),
        Some("think ctx"),
        "no premise: {dot}"
    );
    for (i, v) in ["\"?a_L\"", "\"?b_L\""].iter().enumerate() {
        assert!(
            node_lines(&dot).iter().any(|l| id_of(l) == *v),
            "the variable {v} is drawn:\n{dot}"
        );
        assert!(
            dot.contains(&format!("{u} -> {v} [label=\"{}\"", i + 1)),
            "{dot}"
        );
    }
    assert!(
        dot.contains(&format!("\"?p_L\" -> {u} [label=\"think\"")),
        "{dot}"
    );
    // In the assert panel nothing points at the conclusion: an arrow.
    assert!(dot.contains("\"?a_R\" -> \"?b_R\" [label=\"=\""), "{dot}");

    // The overlay is one drawing: the conclusion is the node the match
    // points at, dashed, as what the conclusion adds, and in no frame, and no
    // arrow draws it again.
    let overlay = render_rule_form(&ast, rule(&ast, &forms, "trust"), RuleMode::Overlay);
    nothing_is_filled(&overlay);
    let node = node_lines(&overlay)
        .into_iter()
        .find(|l| id_of(l) == "\"(= ?a ?b)\"")
        .expect("the node");
    assert!(node.contains("style=dashed"), "{overlay}");
    assert_eq!(frame_of(&overlay, "\"(= ?a ?b)\""), None, "{overlay}");
    assert!(!overlay.contains("cluster_ctx_"), "{overlay}");
    assert!(
        !overlay.contains("\"?a\" -> \"?b\" [label=\"=\""),
        "{overlay}"
    );

    // A pattern that is a premise and under an arrow of another is one node,
    // in no frame, and no arrow.
    let noted = render_rule_form(&ast, rule(&ast, &forms, "noted"), RuleMode::SideBySide);
    nothing_is_filled(&noted);
    let node = node_lines(&noted)
        .into_iter()
        .find(|l| id_of(l) == "\"(= ?a ?b)_L\"")
        .expect("the node");
    assert!(!node.contains("style="), "{noted}");
    assert_eq!(frame_of(&noted, "\"(= ?a ?b)_L\""), None, "{noted}");
    assert!(!noted.contains("cluster_ctx_"), "{noted}");
    assert!(
        !noted.contains("\"?a_L\" -> \"?b_L\" [label=\"=\""),
        "{noted}"
    );
}

#[test]
fn the_constraint_view_draws_a_fact_under_an_arrow_as_its_node() {
    let (ast, forms) = parsed("(act (f a))\n(pair (f a) (g b c))\n(f a)\n");
    let dot = render_constraints(&ast, &forms, "constraints");
    let f = "\"(f a)\"";
    let declared: Vec<&str> = node_lines(&dot)
        .into_iter()
        .filter(|l| id_of(l) == f)
        .collect();
    assert_eq!(declared.len(), 1, "{dot}");
    // Named for its relation, its unary property a badge as on any node, in
    // no frame since the program states it.
    assert!(declared[0].contains("shape=octagon, label=\"f"), "{dot}");
    assert_eq!(frame_of(&dot, f), None, "{dot}");
    nothing_is_filled(&dot);
    assert!(dot.contains(&format!("{f} -> \"a\" [label=\"1\"")), "{dot}");
    // One the program does not state is in the frame of what points at it.
    let g = "\"(g b c)\"";
    assert_eq!(frame_of(&dot, g).as_deref(), Some("pair ctx"), "{dot}");
    assert!(dot.contains(&format!("{g} -> \"c\" [label=\"2\"")), "{dot}");
}
