//! **Every view draws a fact in the canonical form, or folded on request, and
//! every object once** (Q-M1f.419, Q-M1f.395).
//!
//! The canonical form is the graph as the reasoning pages define it: a fact
//! is an octagon with no name in it, its first arrow, numbered `1`, at its
//! relation's atom, a node, and its next arrows, `2`, `3`, …, at its
//! arguments. Folded, a fact of three arrows that no drawn fact points at is
//! one arrow from its first argument to its second, and a fact of two arrows
//! a loop on its argument, labelled with its relation's name (Q-M1f.425). A
//! first arrow written back, from the relation's atom to the fact with
//! `dir=back`, which lays the relation out above its facts, is read as it is
//! drawn: the fact's arrow at its relation. No view draws an octagon with a name in it, a node
//! labelled with a fact's text, or any relation otherwise than another
//! (Q-M1f.421): no box for what an `is-a` fact points at, no oval for what it
//! starts from, no empty arrowhead, no class node for `=`.
//!
//! A fact that a drawn fact points at is its own node, never an arrow beside
//! it. Where the knowledge base does not hold it, in a context only, it
//! stands in a dashed frame named for the relations of the facts that point
//! at it; a held one stands in no frame, whatever points at it, and no
//! octagon is filled (Q-M1f.398).
//!
//! Held here three ways: the writers on small fixtures, as claims a reader of
//! the DOT would state; every view of every corpus program in both forms,
//! by the rules any drawing keeps; and the program view's `kb` digraph
//! against the KB view of the same program (Q-M1f.420). What the bytes of
//! every corpus program's views are is `id_order_invariance.rs`'s, and that
//! Graphviz takes them `dot_wellformed.rs`'s.

use std::collections::HashSet;

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_ir::{Ast, NodeId, parse};
use ein_render::fact_graph::fact_id;
use ein_render::ir_dot::{DotOpts, render_kb, to_dot};
use ein_render::kb_dot::{KbDotOpts, to_dot as kb_to_dot};
use ein_render::render_constraints;
use ein_render::rules::{RuleMode, render_rule_form};
use ein_render::shape::{KB_VIEWS, all_views, dot_shape, kb_dot_shape};

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

fn kb_view(text: &str, fold: bool) -> String {
    let (mut ast, forms) = parsed(text);
    let mut terms = Terms::new();
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    kb_to_dot(
        &kb,
        &terms,
        &KbDotOpts {
            fold,
            ..KbDotOpts::default()
        },
    )
}

fn program_view(text: &str, fold: bool) -> String {
    let (ast, forms) = parsed(text);
    to_dot(
        &ast,
        &forms,
        DotOpts {
            fold,
            ..DotOpts::default()
        },
    )
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

/// The ids of the octagons a drawing declares.
fn octagons(dot: &str) -> Vec<&str> {
    node_lines(dot)
        .into_iter()
        .filter(|l| l.contains("shape=octagon"))
        .map(id_of)
        .collect()
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

/// The arrows of a drawing: `(from, to, label)`, the label an arrow's number
/// or its `label`, `""` where it has neither. An arrow written `dir=back` is
/// read as it is drawn, from its head to its tail: the layout is free, and
/// only the arrows are the drawing's structure (Q21).
fn arrows(dot: &str) -> Vec<(&str, &str, &str)> {
    dot.lines()
        .map(str::trim)
        .filter_map(|l| {
            let (from, rest) = l.split_once(" -> ")?;
            let (to, attrs) = match rest.split_once(" [") {
                Some((to, attrs)) => (to, attrs),
                None => (rest.trim_end_matches(';'), ""),
            };
            // An arrow's number is its `xlabel`; a folded arrow's relation,
            // and a guard's glyph, its `label`.
            let label = attrs
                .strip_prefix("xlabel=\"")
                .or_else(|| attrs.strip_prefix("label=\""))
                .or_else(|| attrs.split_once(", label=\"").map(|(_, l)| l))
                .and_then(|l| l.split_once('"'))
                .map_or("", |(l, _)| l);
            if attrs.contains("dir=back") {
                Some((to, from, label))
            } else {
                Some((from, to, label))
            }
        })
        .collect()
}

/// Whether the drawing has an arrow from `from` to `to` numbered or labelled
/// `label`, read as drawn ([`arrows`]).
fn has_arrow(dot: &str, from: &str, to: &str, label: &str) -> bool {
    arrows(dot)
        .iter()
        .any(|(f, t, l)| *f == from && *t == to && *l == label)
}

/// **The rules a drawing of facts keeps**, in the form it is drawn in,
/// read off its DOT: every octagon has no name in it and an arrow `1`, at
/// its relation; no label is a fact's text, and no arrow is numbered with a
/// `#`; no relation is drawn otherwise than another; and the arrows of the
/// form: in the canonical form no labelled arrow but a numbered one or a
/// guard's undirected link, and folded no octagon of three arrows that
/// nothing points at.
fn holds_the_rules(dot: &str, fold: bool) -> Result<(), String> {
    let all = arrows(dot);
    for line in node_lines(dot) {
        if line.contains("shape=octagon") && !line.contains("label=\"\"") {
            return Err(format!("an octagon with a name in it: {line}"));
        }
    }
    for line in dot.lines() {
        let mut rest = line;
        while let Some(at) = rest.find("label=\"(") {
            if !rest[..at].ends_with('x') {
                return Err(format!("a label that is a fact's text: {line}"));
            }
            rest = &rest[at + 1..];
        }
        for mark in [
            "doublecircle",
            "shape=oval",
            "arrowhead=empty",
            "\\n«",
            "label=\"#",
        ] {
            if line.contains(mark) {
                return Err(format!("{mark} drawn: {line}"));
            }
        }
    }
    for id in octagons(dot) {
        if !all.iter().any(|(f, _, l)| *f == id && *l == "1") {
            return Err(format!("the octagon {id} has no arrow 1 at a relation"));
        }
    }
    for line in dot.lines().map(str::trim).filter(|l| l.contains(" -> ")) {
        if line.contains("dir=back") && !line.contains("xlabel=\"1\"") {
            return Err(format!(
                "an arrow drawn back that is no first arrow: {line}"
            ));
        }
        let Some(&(from, to, label)) = arrows(line).first() else {
            continue;
        };
        let numbered = !label.is_empty() && label.chars().all(|c| c.is_ascii_digit());
        if !fold
            && !label.is_empty()
            && !numbered
            && !line.contains("dir=none")
            && label != "lifts to"
        {
            return Err(format!(
                "a labelled arrow {from} -> {to} in the canonical form: {line}"
            ));
        }
    }
    if fold {
        for id in octagons(dot) {
            let mut out: Vec<&str> = all
                .iter()
                .filter(|(f, _, l)| *f == id && !l.is_empty())
                .map(|(_, _, l)| *l)
                .collect();
            out.sort_unstable();
            let pointed_at = all.iter().any(|(_, t, _)| *t == id);
            if (out == ["1", "2", "3"] || out == ["1", "2"]) && !pointed_at {
                return Err(format!(
                    "a fact of {} arrows that nothing points at, not folded: {id}",
                    out.len()
                ));
            }
        }
    }
    Ok(())
}

/// No node of the drawing is filled: *held* is *outside every frame*.
fn nothing_is_filled(dot: &str) {
    assert!(!dot.contains("filled"), "a filled node:\n{dot}");
}

#[test]
fn the_kb_view_draws_a_fact_in_two_contexts_once() {
    for fold in [false, true] {
        let dot = kb_view(KB, fold);
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("fold={fold}: {e}\n{dot}"));
        let e = fact_id("=", &["5".into(), "(+ 2 2)".into()]);
        let sum = fact_id("+", &["2".into(), "2".into()]);
        // One octagon for the one fact of = that facts point at, held and in
        // the contexts of `says` and `think`: in no frame, its first arrow at
        // the relation's atom, its next at the name and at the fact under it.
        assert_eq!(
            octagons(&dot).iter().filter(|id| **id == e).count(),
            1,
            "{dot}"
        );
        assert_eq!(
            frame_of(&dot, &e),
            None,
            "a held fact is in no frame:\n{dot}"
        );
        for (i, to) in ["\"=\"", "\"5\"", &sum].iter().enumerate() {
            assert!(
                has_arrow(&dot, &e, to, &(i + 1).to_string()),
                "fold={fold}:\n{dot}"
            );
        }
        nothing_is_filled(&dot);
        // Both facts that point at it point at the one node: arrows from
        // their octagons, or folded, from the name.
        for (rel, who) in [("think", "Alice"), ("says", "Bob")] {
            let f = fact_id(rel, &[who.into(), "(= 5 (+ 2 2))".into()]);
            let arrow = if fold {
                format!("  \"{who}\" -> {e} [xlabel=\"{rel}\"")
            } else {
                format!("  {f} -> {e} [xlabel=\"3\"")
            };
            assert!(dot.contains(&arrow), "fold={fold}: {arrow}\n{dot}");
        }
        // `(+ 2 2)` is in a context only, not held: one node, in the frame of
        // the relation that points at it, and the one frame of the drawing.
        assert_eq!(frame_of(&dot, &sum).as_deref(), Some("= ctx"), "{dot}");
        assert_eq!(dot.matches("subgraph cluster_ctx_").count(), 1, "{dot}");
        // `(= 4 (+ 2 2))` is held and nothing points at it: its octagon, or
        // folded, one arrow labelled with its relation.
        let four = fact_id("=", &["4".into(), "(+ 2 2)".into()]);
        if fold {
            assert!(
                dot.contains(&format!("  \"4\" -> {sum} [xlabel=\"=\"")),
                "{dot}"
            );
            assert!(!dot.contains(&four), "{dot}");
        } else {
            assert!(has_arrow(&dot, &four, "\"=\"", "1"), "{dot}");
        }
        // No node is named or labelled by a fact's text.
        for text in ["(= 5", "(= 4", "(+ 2"] {
            assert!(
                !dot.contains(text),
                "the text {text} is in the drawing:\n{dot}"
            );
        }
    }
}

#[test]
fn a_knowledge_base_whose_facts_under_an_arrow_are_all_held_has_no_frame() {
    // `(owns Bob Dog)` is held, and Alice thinks it and Carol says it: one
    // octagon in no frame, and the drawing has none.
    let owns = fact_id("owns", &["Bob".into(), "Dog".into()]);
    for fold in [false, true] {
        let dot = kb_view(
            "(owns Bob Dog)\n(think Alice (owns Bob Dog))\n(says Carol (owns Bob Dog))\n",
            fold,
        );
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
        assert_eq!(
            octagons(&dot).iter().filter(|id| **id == owns).count(),
            1,
            "{dot}"
        );
        assert!(!dot.contains("cluster_ctx_"), "{dot}");
        assert!(
            !dot.contains("newrank"),
            "a view with no frame ranks as it did:\n{dot}"
        );
        nothing_is_filled(&dot);
    }
    // Two facts of one relation: the one held stands outside, the one only
    // thought alone in `think ctx`.
    let dot = kb_view(
        "(owns Bob Dog)\n(think Alice (owns Bob Dog))\n(think Dan (owns Bob Dog))\n\
         (think Dan (owns Bob Cat))\n",
        false,
    );
    let cat = fact_id("owns", &["Bob".into(), "Cat".into()]);
    assert_eq!(frame_of(&dot, &owns), None, "{dot}");
    assert_eq!(frame_of(&dot, &cat).as_deref(), Some("think ctx"), "{dot}");
    nothing_is_filled(&dot);
}

#[test]
fn a_knowledge_base_with_no_fact_under_an_arrow_has_no_frame() {
    for fold in [false, true] {
        let dot = kb_view(
            "(lives-in Norwegian House-1)\n(owns Norwegian Dog)\n(triple a b c)\n",
            fold,
        );
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
        assert!(!dot.contains("cluster_ctx_"), "{dot}");
        assert!(!dot.contains("// facts in a context"), "{dot}");
        // The fact of four arrows is an octagon in both forms; folded, the
        // two of three arrows are arrows.
        let triple = fact_id("triple", &["a".into(), "b".into(), "c".into()]);
        assert!(has_arrow(&dot, &triple, "\"triple\"", "1"), "{dot}");
        assert_eq!(
            dot.contains("  \"Norwegian\" -> \"Dog\" [xlabel=\"owns\""),
            fold,
            "{dot}"
        );
    }
}

/// **No relation is drawn otherwise than another** (Q-M1f.421): `is-a` and
/// `relation` are drawn as any relation, and every atom alike.
#[test]
fn is_a_and_relation_are_ordinary_relations() {
    let text = "(relation owns Person Pet :why \"{?3} owns {?4}\")\n\
                (is-a Bob Person)\n(is-a Dog Pet)\n(owns Bob Dog)\n";
    for fold in [false, true] {
        for dot in [kb_view(text, fold), program_view(text, fold)] {
            holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
            assert!(!dot.contains("shape=oval"), "{dot}");
            assert!(!dot.contains("// types"), "{dot}");
            let decl = fact_id("relation", &["owns".into(), "Person".into(), "Pet".into()]);
            assert!(has_arrow(&dot, &decl, "\"relation\"", "1"), "{dot}");
            let bob = fact_id("is-a", &["Bob".into(), "Person".into()]);
            if fold {
                assert!(
                    dot.contains("  \"Bob\" -> \"Person\" [xlabel=\"is-a\""),
                    "{dot}"
                );
            } else {
                assert!(has_arrow(&dot, &bob, "\"is-a\"", "1"), "{dot}");
            }
        }
    }
}

#[test]
fn the_program_view_draws_a_fact_under_an_arrow_as_its_node() {
    for fold in [false, true] {
        let dot = program_view(KB, fold);
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
        nothing_is_filled(&dot);
        assert!(dot.starts_with("digraph kb {"), "{dot}");
        assert!(!dot.contains("digraph ontology"), "{dot}");
        assert!(!dot.contains("digraph facts"), "{dot}");
        // One node for the fact two facts point at, in no frame since the
        // program states it.
        let e = fact_id("=", &["5".into(), "(+ 2 2)".into()]);
        assert_eq!(
            octagons(&dot).iter().filter(|id| **id == e).count(),
            1,
            "{dot}"
        );
        assert_eq!(frame_of(&dot, &e), None, "{dot}");
        // The fact under it is in a context only, not stated.
        let sum = fact_id("+", &["2".into(), "2".into()]);
        assert_eq!(frame_of(&dot, &sum).as_deref(), Some("= ctx"), "{dot}");
    }
}

#[test]
fn a_negation_is_a_fact_of_not_that_points_at_the_fact_it_denies() {
    let g = fact_id("likes", &["Bob".into(), "Carol".into()]);
    let n = fact_id("not", &["(likes Bob Carol)".into()]);
    for fold in [false, true] {
        let dot = program_view(
            "(think Alice (likes Bob Carol))\n(not (likes Bob Carol))\n",
            fold,
        );
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
        // The denied fact is drawn once, the node `think` and `not` point at.
        assert_eq!(
            frame_of(&dot, &g).as_deref(),
            Some("not, think ctx"),
            "{dot}"
        );
        if fold {
            // A fact of two arrows that nothing points at: a loop on the
            // fact it denies (Q-M1f.425).
            assert!(has_arrow(&dot, &g, &g, "not"), "{dot}");
            assert!(!octagons(&dot).contains(&n.as_str()), "{dot}");
        } else {
            assert!(has_arrow(&dot, &n, "\"not\"", "1"), "{dot}");
            assert!(has_arrow(&dot, &n, &g, "2"), "{dot}");
        }
        assert!(!dot.contains("\"Bob\" -> \"Carol\""), "{dot}");
    }
    // Alone, too: a negation is a fact of `not`, and the fact it denies is in
    // its context.
    let alone = program_view("(not (likes Bob Carol))\n", false);
    assert_eq!(frame_of(&alone, &g).as_deref(), Some("not ctx"), "{alone}");
}

const RULES: &str = "\
(rule trust ()
  :match  (and (reliable ?p) (think ?p (= ?a ?b)))
  :assert (= ?a ?b)
  :why    \"{?p} is reliable\")
(rule noted ()
  :match  (and (think ?p (= ?a ?b)) (= ?a ?b) (absent (not (= ?b ?a))))
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
fn the_rule_view_draws_a_pattern_as_a_fact() {
    let (ast, forms) = parsed(RULES);
    for fold in [false, true] {
        for mode in [RuleMode::SideBySide, RuleMode::Overlay] {
            for name in ["trust", "noted"] {
                let dot = render_rule_form(&ast, rule(&ast, &forms, name), mode, fold);
                holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{name}: {e}\n{dot}"));
                nothing_is_filled(&dot);
            }
        }
    }
    let dot = render_rule_form(
        &ast,
        rule(&ast, &forms, "trust"),
        RuleMode::SideBySide,
        false,
    );
    // In the match panel the pattern under `think`'s arrow is a node in the
    // frame `think ctx`, its first arrow at the relation's box and the next
    // two at its variables, so what it binds is drawn.
    let u = "\"(= ?a ?b)_L\"";
    assert_eq!(
        octagons(&dot).iter().filter(|id| **id == u).count(),
        1,
        "{dot}"
    );
    assert_eq!(
        frame_of(&dot, u).as_deref(),
        Some("think ctx"),
        "no premise: {dot}"
    );
    for (i, v) in ["\"=_L\"", "\"?a_L\"", "\"?b_L\""].iter().enumerate() {
        assert!(has_arrow(&dot, u, v, &(i + 1).to_string()), "{dot}");
    }
    assert!(dot.contains("\"=_L\" [label=\"=\", shape=box];"), "{dot}");
    // Folded, the pattern of `think` that nothing points at is an arrow, and
    // the conclusion too.
    let folded = render_rule_form(
        &ast,
        rule(&ast, &forms, "trust"),
        RuleMode::SideBySide,
        true,
    );
    assert!(
        folded.contains(&format!("\"?p_L\" -> {u} [label=\"think\"")),
        "{folded}"
    );
    assert!(
        folded.contains("\"?a_R\" -> \"?b_R\" [label=\"=\""),
        "{folded}"
    );

    // The overlay is one drawing: the conclusion is the node the match
    // points at, dashed, as what the conclusion adds, and in no frame.
    let overlay = render_rule_form(&ast, rule(&ast, &forms, "trust"), RuleMode::Overlay, true);
    let node = node_lines(&overlay)
        .into_iter()
        .find(|l| id_of(l) == "\"(= ?a ?b)\"")
        .expect("the node");
    assert!(node.contains("style=dashed"), "{overlay}");
    assert_eq!(frame_of(&overlay, "\"(= ?a ?b)\""), None, "{overlay}");
    assert!(
        !overlay.contains("\"?a\" -> \"?b\" [label=\"=\""),
        "{overlay}"
    );

    // A pattern that is a premise and under an arrow of another is one node,
    // in no frame; the negation under the absence is a red fact of `not`
    // whose second arrow is at the pattern it denies.
    let noted = render_rule_form(
        &ast,
        rule(&ast, &forms, "noted"),
        RuleMode::SideBySide,
        false,
    );
    assert_eq!(frame_of(&noted, "\"(= ?a ?b)_L\""), None, "{noted}");
    assert!(
        noted.lines().any(|l| l.contains("\"not_L\" -> ")
            && l.contains("_not_L\" [xlabel=\"1\", dir=back, color=\"#d62728\"")),
        "{noted}"
    );
    assert!(
        noted
            .lines()
            .any(|l| l.contains("_not_L\" -> \"(= ?b ?a)_L\" [xlabel=\"2\"")),
        "{noted}"
    );
}

/// **The constraint view keeps the facts about relations** (Q-M1f.523,
/// Q-M1f.530): of root's knowledge base, every fact with a relation of the
/// program directly under an argument arrow, each drawn as a fact, with what
/// it points at. `r`, `f` and `g` are relations of the program, each written
/// first in a tuple; `act`'s fact and `(f a)` hold none, and `(pair …)` one
/// only under an arrow of a fact under its own.
#[test]
fn the_constraint_view_draws_a_property_as_a_fact() {
    let text = "(r x y)\n(symmetric r)\n(f a)\n(act (f a))\n(pair (f a) (g b c))\n\
                (about r (f a))\n(about g (g b c))\n(pair x (h r))\n";
    let (mut ast, forms) = parsed(text);
    let mut terms = Terms::new();
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    for fold in [false, true] {
        let dot = render_constraints(&kb, &terms, "constraints", fold);
        holds_the_rules(&dot, fold).unwrap_or_else(|e| panic!("{e}\n{dot}"));
        nothing_is_filled(&dot);
        // A property of a relation is a fact about it, never a word in its
        // node: an octagon whose second arrow is at the relation.
        let sym = fact_id("symmetric", &["r".into()]);
        if fold {
            // Folded, a fact of two arrows is a loop on its argument.
            assert!(has_arrow(&dot, "\"r\"", "\"r\"", "symmetric"), "{dot}");
        } else {
            assert!(has_arrow(&dot, &sym, "\"r\"", "2"), "{dot}");
        }
        // A fact the knowledge base holds, pointed at, is in no frame; one it
        // does not hold, in the frame of what points at it.
        let f = fact_id("f", &["a".into()]);
        assert_eq!(frame_of(&dot, &f), None, "{dot}");
        let g = fact_id("g", &["b".into(), "c".into()]);
        assert_eq!(frame_of(&dot, &g).as_deref(), Some("about ctx"), "{dot}");
        assert!(
            dot.contains(&format!("  {g} -> \"c\" [xlabel=\"3\"")),
            "{dot}"
        );
        // A fact about no relation is not kept, and neither is one whose
        // relation stands only under an arrow of a fact under its arrows.
        for (rel, args) in [
            ("r", vec!["x", "y"]),
            ("act", vec!["(f a)"]),
            ("pair", vec!["(f a)", "(g b c)"]),
            ("pair", vec!["x", "(h r)"]),
        ] {
            let args: Vec<String> = args.into_iter().map(str::to_string).collect();
            let id = fact_id(rel, &args);
            assert!(!dot.contains(&id), "({rel} …) is kept:\n{dot}");
        }
    }
}

/// **A written check is no fact the constraint view draws** (Q-M1f.531): a
/// check the program writes as a fact, `(eq a w)` or `(not (eq a a))`, is no
/// fact the knowledge base holds, and the view draws the knowledge base.
#[test]
fn the_constraint_view_draws_no_written_check() {
    for check in ["(eq a w)", "(not (eq a a))"] {
        let text = format!("(relation p Node :why \"{{?3}} is p\")\n(p a)\n{check}\n");
        let (mut ast, forms) = parsed(&text);
        let mut terms = Terms::new();
        let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
        let dot = render_constraints(&kb, &terms, "constraints", false);
        assert!(!dot.contains("\"eq\""), "{check} is drawn:\n{dot}");
        assert!(!dot.contains("\"not\""), "{check} is drawn:\n{dot}");
        let declared = fact_id("relation", &["p".into(), "Node".into()]);
        assert!(dot.contains(&declared), "{dot}");
    }
}

/// The program view's `kb` digraph, as the KB view's code draws it, less the
/// facts the loader adds beside a declaration, `(relation r)`: its octagon
/// and every arrow that ends at it, or folded, its loop on `r`.
fn kb_less_the_loaders(kb: &str, ast: &Ast, forms: &[NodeId]) -> String {
    let names: Vec<String> = forms
        .iter()
        .filter(|f| ast.head_name(**f) == Some("relation"))
        .filter_map(|f| {
            let args = ast.form_args(*f);
            let labels = args
                .iter()
                .filter(|a| !matches!(ast.node(**a), ein_ir::Node::KwPair { .. }))
                .count();
            (labels > 1).then(|| ein_render::value_label(ast, args[0]))
        })
        .collect();
    let added: HashSet<String> = names
        .iter()
        .map(|r| fact_id("relation", std::slice::from_ref(r)))
        .collect();
    let loops: Vec<String> = names
        .iter()
        .map(|r| {
            let r = ein_render::dot_util::quote(r);
            format!("{r} -> {r} [xlabel=\"relation\"")
        })
        .collect();
    kb.lines()
        .filter_map(|l| {
            let t = l.trim();
            // A layer's group lists the loader's facts among its members.
            if let Some(members) = t.strip_prefix("{ rank=same; ") {
                let kept: Vec<&str> = members
                    .trim_end_matches('}')
                    .split_whitespace()
                    .filter(|m| !added.contains(m.trim_end_matches(';')))
                    .collect();
                return (!kept.is_empty())
                    .then(|| format!("  {{ rank=same; {} }}", kept.join(" ")));
            }
            let (from, rest) = t.split_once(" -> ").unwrap_or((t, ""));
            let to = rest.split([' ', '[']).next().unwrap_or("");
            let from = from.split([' ', '[']).next().unwrap_or("");
            (!added.contains(from)
                && !added.contains(to)
                && !loops.iter().any(|p| t.starts_with(p)))
            .then(|| l.to_string())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// What `ein render kb` draws of a file: loaded, saturated. With it, the
/// parsed program, and whether saturation derived a fact, as the check of a
/// written `(eq …)` can with no rule.
struct KbCommand {
    dot: String,
    ast: Ast,
    forms: Vec<NodeId>,
    derived: bool,
}

fn kb_command(text: &str, base: Option<&std::path::Path>, fold: bool) -> Option<KbCommand> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, None).ok()?;
    let mut terms = Terms::new();
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, base).ok()?;
    let loaded = kb.n_facts();
    ein_infer::saturate_events(&ast, &mut terms, &mut kb).ok()?;
    let dot = kb_to_dot(
        &kb,
        &terms,
        &KbDotOpts {
            fold,
            ..KbDotOpts::default()
        },
    );
    Some(KbCommand {
        dot,
        derived: kb.n_facts() != loaded,
        ast,
        forms,
    })
}

/// **The program view's `kb` is the KB view of the program** (Q-M1f.420):
/// for a program with no rule, no import and no macro, the digraph `kb`
/// that `ein render program` draws is what `ein render kb` draws of it, but
/// for what the loader adds, in both forms.
#[test]
fn the_program_views_kb_is_the_kb_views_drawing() {
    let text = "\
(relation owns Person Pet :why \"{?3} owns {?4}\")
(relation lives Person :why \"{?3} lives\")
(owns Bob Dog :source \"condition (3)\")
(claims Carol (owns Bob Cat))
(not (owns Ann Dog))
(lives Ann :source \"x\")
(owns Bob Dog)
(empty)
(ontology (relation r T T) (is-a a T))
";
    for fold in [false, true] {
        let k = kb_command(text, None, fold).expect("loads");
        assert!(!k.derived);
        let program = render_kb(&k.ast, &k.forms, fold);
        assert_eq!(
            program,
            kb_less_the_loaders(&k.dot, &k.ast, &k.forms),
            "fold={fold}"
        );
    }
}

/// Every corpus program the claim is about: one with no rule, hypothesis rule, macro
/// or import, that loads and saturates, and is no saved file.
#[test]
fn the_program_views_kb_is_the_kb_views_drawing_on_the_corpus() {
    let root = repo_root();
    let files = corpus_files();
    let found = ein_corpus::pool::map(
        "every_object_once::kb",
        &files,
        |f| {
            (
                f.strip_prefix(&root).unwrap_or(f).display().to_string(),
                "kb".to_string(),
            )
        },
        |path| -> Option<Result<(), String>> {
            let text = std::fs::read_to_string(path).ok()?;
            if ein_ir::saved::split(&text, path.to_str()).ok()?.is_some() {
                return None;
            }
            for fold in [false, true] {
                let k = kb_command(&text, path.parent(), fold)?;
                // A written check, `(eq …)`, `(neq …)` or `(not (eq …))`, is
                // read by the loader as its verdict, `(false)` where it fails.
                let check = |f: NodeId| {
                    k.ast.head_name(f).is_some_and(ein_core::is_predicate)
                        || (k.ast.head_name(f) == Some("not")
                            && k.ast.form_args(f).first().is_some_and(|g| {
                                k.ast.head_name(*g).is_some_and(ein_core::is_predicate)
                            }))
                };
                if k.derived
                    || k.forms.iter().any(|f| {
                        check(*f)
                            || matches!(k.ast.head_name(*f), Some("rule" | "macro" | "import"))
                    })
                {
                    return None;
                }
                let program = render_kb(&k.ast, &k.forms, fold);
                let kb = kb_less_the_loaders(&k.dot, &k.ast, &k.forms);
                // A program that states no fact draws no `kb`, and its KB
                // view draws none either.
                let same = if program.is_empty() {
                    !kb.contains(" -> ")
                } else {
                    program == kb
                };
                if !same {
                    return Some(Err(format!("{} fold={fold}", path.display())));
                }
            }
            Some(Ok(()))
        },
    );
    let checked = found.iter().filter(|r| matches!(r, Some(Ok(())))).count();
    let bad: Vec<String> = found
        .into_iter()
        .flatten()
        .filter_map(Result::err)
        .collect();
    assert!(checked > 20, "only {checked} corpus programs read");
    assert!(
        bad.is_empty(),
        "the program view's kb is not the KB view's:\n{}",
        bad.join("\n")
    );
}

/// **Every view of every corpus program** keeps the rules, in the form it is
/// drawn in: the `-fold` views folded, every other one canonical.
#[test]
fn every_view_of_every_corpus_program_keeps_the_rules() {
    let root = repo_root();
    let files = corpus_files();
    let views = all_views();
    let found = ein_corpus::pool::map(
        "every_object_once::views",
        &files,
        |f| {
            (
                f.strip_prefix(&root).unwrap_or(f).display().to_string(),
                "views".to_string(),
            )
        },
        |path| -> Vec<String> {
            let Ok(text) = std::fs::read_to_string(path) else {
                return Vec::new();
            };
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let Ok(forms) = ein_ir::saved::parse_text(&mut ast, &text, path.to_str()) else {
                return Vec::new();
            };
            let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent());
            let mut bad = Vec::new();
            for view in &views {
                let drawn = if KB_VIEWS.contains(view) {
                    match &kb {
                        Ok(kb) => kb_dot_shape(&ast, &mut terms.clone(), &mut kb.clone(), view),
                        Err(_) => continue,
                    }
                } else {
                    let mut ast = Ast::new();
                    let forms = ein_ir::saved::parse_text(&mut ast, &text, path.to_str())
                        .expect("it parsed above");
                    dot_shape(&mut ast, &mut Terms::new(), &forms, path.parent(), view)
                };
                let Ok(dot) = drawn else { continue };
                if let Err(e) = holds_the_rules(&dot, view.ends_with("-fold")) {
                    bad.push(format!("{} [{view}]: {e}", path.display()));
                }
            }
            bad
        },
    );
    let bad: Vec<String> = found.into_iter().flatten().collect();
    assert!(
        bad.is_empty(),
        "{} views break the rules:\n{}",
        bad.len(),
        bad.join("\n")
    );
}
