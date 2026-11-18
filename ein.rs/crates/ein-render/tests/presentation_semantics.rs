//! Presentation as **behaviour**, not as bytes.
//!
//! The bytes of every renderer are already pinned twice over:
//! `id_order_invariance.rs` digests the DOT views of every corpus entry
//! against `golden/corpus_shapes.md5`, and `golden_dot.rs` /
//! `golden_trace.rs` reproduce checked-in goldens.
//! What none of those say is *what the picture means* — a golden that changed
//! for a good reason is refreshed and the claim behind it is lost. These are
//! the claims: the ones a reader of the DOT would state in a sentence, each
//! written so that the wrong picture fails rather than a different one.
//!
//! The subjects:
//!
//! - the derivation-DAG trace view;
//! - `render rules` / `render rule --name` as *static file views*;
//! - the `→ ⊥` edge order;
//! - a derivation's three levels, and its palette's shades against black and
//!   white;
//! - the per-form dispatch's own shapes, and mode/view name resolution;
//! - the read-out's **dead hole**, which *forbid* keeps out of every program
//!   that loads, read at library level past the refusal.
//!
//! The read-out's English is held by `ein-cli/tests/acceptance_cli.rs`, and
//! the region trace's flags by `ein-cli/tests/region_modes.rs`.
//!
//! **On the subcommands.** `ein-render` cannot
//! depend on `ein-cli` — that is the dependency edge, backwards — so what is
//! asserted here is the body each subcommand is a five-line wrapper around
//! (`ein-cli/src/render.rs`): the
//! renderer call, its input, and the condition the subcommand turns into its
//! exit code. The exit codes and the two stderr sentences themselves are
//! surface, and `ein-cli/tests/help_surface.rs` is where that surface lives.

use ein_core::{FactId, Terms, Value};
use ein_ir::{Ast, NodeId, parse};
use ein_render::ir_dot::{DotOpts, TraceView, render_trace, to_dot};
use ein_render::render_constraints;
use ein_render::rules::{RuleMode, render_rule_form, render_rules_forms};
use ein_render::slice::render_slice;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

// ── fixtures ───────────────────────────────────────────────────────

/// Parse without loading — the *static file view* every `ein render` subcommand
/// except `lattice` takes, and the reason an imported rule is invisible to it.
fn parsed(text: &str) -> (Ast, Vec<NodeId>) {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
    (ast, forms)
}

fn source(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).expect("corpus file")
}

// ── the `(trace …)` views ──────────────────────────────────────────

const TRACE_CHAIN: &str = "\
(trace
  (step s1 :rule from-condition :using (c10) :derives (lives-in Norwegian House-1))
  (step s2 :rule exclusivity :using (s1) :derives (not (lives-in Norwegian House-2))))";

/// **trace-dag-chains-through-step-ids.** In the `dag` view a `:using` that
/// names a prior *step* draws from that step's derived **fact**.
///
/// This is what makes view (c) an explanation graph rather than a redrawn
/// step list: the reader follows facts, and a step id is an internal label
/// that should never become a premise. The failure it guards is not a crash
/// but a plausible-looking graph — `s1` as a fresh box pointing at `s2`,
/// which reads as "step s1 justifies this" instead of "the Norwegian being
/// in House-1 justifies this". Each step is a node of its own, and each fact
/// derived is its node in the canonical form, never a node labelled with its
/// text (Q-M1f.419).
#[test]
fn the_dag_trace_view_chains_through_derived_facts() {
    let (ast, forms) = parsed(TRACE_CHAIN);
    let dag = render_trace(&ast, forms[0], TraceView::Dag);
    let lives_in = ein_render::fact_graph::fact_id(
        "lives-in",
        &["Norwegian".to_string(), "House-1".to_string()],
    );
    assert!(
        dag.contains(&format!("  {lives_in} -> \"step s2\" [")),
        "s2 does not hang off s1's derived fact:\n{dag}"
    );
    assert!(
        dag.contains(&format!("  \"step s1\" -> {lives_in} [")),
        "{dag}"
    );
    assert!(
        dag.contains("\"step s2\" [shape=box, style=rounded, ")
            && dag.contains("penwidth=2, label=\"s2\\nexclusivity\"];"),
        "{dag}"
    );
    assert!(
        dag.contains(&format!(
            "  \"lives-in\" -> {lives_in} [xlabel=\"1\", dir=back"
        )),
        "the derived fact is its node, its first arrow at its relation:\n{dag}"
    );
    assert!(
        !dag.contains("\"s1\""),
        "the step id leaked into the graph as a premise:\n{dag}"
    );
    assert!(
        !dag.contains("label=\"("),
        "a node labelled with a fact's text:\n{dag}"
    );
    // The premise that is *not* a prior step is a name: `c10` is an input,
    // and the dag has nowhere else to root the chain.
    assert!(dag.contains("  \"c10\" -> \"step s1\" ["), "{dag}");
    // The per-step view is the other picture, and the contrast is total: its
    // spine is the step ids and it never names a fact at all. `s2` is the step
    // box; `s1` is drawn where it is *referenced*, as `s2`'s premise rectangle,
    // which is the one place the two views draw the same id differently.
    let per_step = render_trace(&ast, forms[0], TraceView::PerStep);
    assert!(
        per_step.contains("\"s2\" [shape=box, label=\"step: s2\"];"),
        "{per_step}"
    );
    assert!(
        per_step.contains("\"s1\" -> \"s2\" [style=dashed];"),
        "the step chain is not drawn:\n{per_step}"
    );
    assert!(
        !per_step.contains("lives-in"),
        "a derived fact leaked into the per-step view:\n{per_step}"
    );
    assert_ne!(dag, per_step);
}

/// **non-step-trace-events-render-as-ellipses.** Only a `step` is a box.
///
/// A `(trace …)` takes `step` alone, with its five keywords, so a trace that
/// writes another event — `branch-open`, `branch-close`, `contradiction`,
/// `symmetry-class` — is refused at the parse at it, and every event a trace
/// can hold is a box: no ellipse is drawn.
#[test]
fn only_a_step_event_is_a_box_the_rest_are_ellipses() {
    let refused = parse(
        &mut Ast::new(),
        "\
(trace
  (step s1 :rule from-condition :using (c10) :derives (lives-in Norwegian House-1))
  (branch-open b1 :using (c11))
  (branch-close b2)
  (contradiction x1)
  (symmetry-class sc1))",
        Some("<fixture>"),
    )
    .expect_err("a trace takes `step` alone");
    assert_eq!(
        (refused.position(), refused.errors[0].what.as_str()),
        ((3, 4), "unexpected `branch-open`: a name"),
        "{refused}"
    );
    let (ast, forms) = parsed(
        "\
(trace
  (step s1 :rule from-condition :using (c10) :derives (lives-in Norwegian House-1))
  (step s2 :using (c11)))",
    );
    let dot = render_trace(&ast, forms[0], TraceView::PerStep);
    for id in ["s1", "s2"] {
        assert!(
            dot.contains(&format!("\"{id}\" [shape=box, label=\"step: {id}\"];")),
            "{dot}"
        );
    }
    assert!(!dot.contains("ellipse"), "{dot}");
    // A premise is a dashed rectangle whichever step consumes it.
    assert!(dot.contains("\"c10\" [shape=rectangle];"), "{dot}");
    assert!(dot.contains("\"c11\" -> \"s2\" [style=dashed];"), "{dot}");
}

// ── the per-form IR dispatch ───────────────────────────────────────

/// **An equality fact is a fact like any other.** `(= a b)` is a fact of an
/// ordinary relation named `=` (no equality is a fact, so `=` is no
/// predicate), and no relation is drawn otherwise than another
/// (Q-M1f.421): its octagon's first arrow is at the atom `=`, and folded it
/// is the arrow `a -> b` labelled `=`, as `(r a b)` is.
#[test]
fn an_equality_fact_is_drawn_as_any_fact() {
    let (ast, forms) = parsed("(= a b :source \"(1)\")");
    let dot = to_dot(&ast, &forms, DotOpts::default());
    let id = ein_render::fact_graph::fact_id("=", &["a".to_string(), "b".to_string()]);
    assert!(
        dot.contains(&format!("  \"=\" -> {id} [xlabel=\"1\", dir=back")),
        "{dot}"
    );
    for (i, to) in ["a", "b"].iter().enumerate() {
        assert!(
            dot.contains(&format!("  {id} -> \"{to}\" [xlabel=\"{}\"", i + 2)),
            "{dot}"
        );
    }
    assert!(!dot.contains("doublecircle"), "{dot}");
    let folded = to_dot(
        &ast,
        &forms,
        DotOpts {
            fold: true,
            ..DotOpts::default()
        },
    );
    assert!(
        folded.contains("  \"a\" -> \"b\" [xlabel=\"= (1)\""),
        "{folded}"
    );
}

/// **wildcard-and-var-heads-render-as-their-labels.** A pattern head that is
/// not an atom still labels its edge.
///
/// `(?r ?a ?b)` is the relation-abstract pattern the language has, and it has
/// no relation *name* to colour or label an edge with. Dropping the clause —
/// the easy failure, since every other path reads a head atom — would render
/// a rule library in which the generic rules are blank, and the rules that
/// quantify over relations are the interesting ones. A bare `_`, as in
/// `(_ ?a ?b)`, is refused at the parse in every position, and so is a
/// variable as a whole conclusion, `:assert ?a` (Q-M1f.351), so the rule
/// whose picture is held concludes a fact.
#[test]
fn a_wildcard_or_variable_pattern_head_still_labels_its_edge() {
    let refused = parse(
        &mut Ast::new(),
        "(rule w () :match (_ ?a ?b) :assert ?a :why \"w\")",
        Some("<fixture>"),
    )
    .expect_err("a bare `_` and a variable conclusion are refused");
    assert_eq!(refused.position(), (1, 20), "{refused}");
    let (ast, forms) = parsed("(rule v (?r) :match (?r ?a ?b) :assert (q ?a) :why \"v\")");
    // In the canonical form the relation is the variable's node, under the
    // pattern's first arrow.
    let var = render_rule_form(&ast, forms[0], RuleMode::Overlay, false);
    assert!(
        var.contains("\"?r\" -> \"h1_r\" [xlabel=\"1\", dir=back"),
        "the rule parameter is not the node under the first arrow:\n{var}"
    );
    assert!(
        var.contains("\"?r\" [label=\"?r\", shape=diamond];"),
        "{var}"
    );
    // Folded, it labels the arrow.
    let var = render_rule_form(&ast, forms[0], RuleMode::Overlay, true);
    assert!(
        var.contains("\"?a\" -> \"?b\" [label=\"?r\""),
        "the rule parameter did not become the edge label:\n{var}"
    );
    // Variable endpoints keep their `?` names — they are the nodes, not holes.
    assert!(
        var.contains("\"?a\" [label=\"?a\", shape=diamond];"),
        "{var}"
    );
    assert!(
        var.contains("\"?b\" [label=\"?b\", shape=diamond];"),
        "{var}"
    );
}

/// **mode-names-alias-and-an-unknown-one-is-refused** (absorbs
/// **trace-view-aliases**). Every rendering mode has two names and no third.
///
/// The single letters are the original CLI spelling and the words are what
/// `--rule-mode` / `--view` document; both have to keep working, and they have
/// to resolve to the *same* rendering rather than to two that merely look
/// alike — so the aliases are compared by rendered output, not by name. The
/// refusal is the other half: a mode parser that fell back to its default
/// would silently hand a user the side-by-side picture they explicitly asked
/// not to have.
#[test]
fn mode_and_view_names_alias_and_an_unknown_one_is_refused() {
    let (ast, forms) = parsed("(rule x () :match (r ?a ?b) :assert (r ?b ?a) :why \"x\")");
    let rule = forms[0];
    let render = |name: &str| {
        render_rule_form(
            &ast,
            rule,
            RuleMode::parse(name).unwrap_or_else(|| panic!("{name} is a rule mode")),
            false,
        )
    };
    assert_eq!(render("sidebyside"), render("a"));
    assert_eq!(render("side-by-side"), render("a"));
    assert_eq!(render("overlay"), render("c"));
    assert_ne!(render("a"), render("c"));
    assert!(RuleMode::parse("bogus").is_none());
    assert!(RuleMode::parse("b").is_none(), "`b` is a *trace* view name");

    let (ast, forms) = parsed(TRACE_CHAIN);
    let trace = forms[0];
    let view = |name: &str| {
        render_trace(
            &ast,
            trace,
            TraceView::parse(name).unwrap_or_else(|| panic!("{name} is a trace view")),
        )
    };
    assert_eq!(view("per-step"), view("a"));
    assert_eq!(view("aggregate"), view("b"));
    assert_eq!(view("dag"), view("c"));
    assert_ne!(view("dag"), view("per-step"));
    assert!(TraceView::parse("nope").is_none());
}

// ── the static file views (`ein render rules` / `rule` / `constraints`) ──

/// The `(rule …)` / `(hrule …)` forms of a parsed file, in file order — the
/// `rule_forms` the `render` subcommands filter on.
fn rule_forms(ast: &Ast, forms: &[NodeId]) -> Vec<NodeId> {
    forms
        .iter()
        .copied()
        .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
        .collect()
}

fn rule_named(ast: &Ast, forms: &[NodeId], name: &str) -> Option<NodeId> {
    rule_forms(ast, forms)
        .into_iter()
        .find(|r| ast.form_args(*r).first().and_then(|a| ast.atom_name(*a)) == Some(name))
}

/// **cli-render-static-views.** `render rules` draws the rules *in the file*,
/// one digraph each — imports and all their contents excluded.
///
/// The static view is a decision, not an omission: `ein render rules
/// zebra2.ein` answers "what does this file say", and resolving
/// `(import std.algebra …)` would answer a different question with a picture
/// three times the size. The cost is that `symmetric` / `transitive` /
/// `includes` are invisible here, and asserting their absence is what keeps
/// the decision from being quietly reversed. An empty render is the reason the
/// subcommand needs its own guard — a file with no rule forms would otherwise
/// print a blank line and exit 0.
#[test]
fn render_rules_is_a_static_view_of_the_forms_the_file_itself_holds() {
    let text = source("examples/zebra2.ein");
    let (ast, forms) = parsed(&text);
    let rules = rule_forms(&ast, &forms);
    assert!(rules.len() > 5, "zebra2 has a rule library to draw");
    let dot = render_rules_forms(&ast, &rules, RuleMode::SideBySide, false);
    assert_eq!(
        dot.matches("digraph ").count(),
        rules.len(),
        "one digraph per rule form"
    );
    for r in &rules {
        let name = ast
            .form_args(*r)
            .first()
            .and_then(|a| ast.atom_name(*a))
            .expect("a named rule");
        assert!(
            dot.contains(&format!("rule_{}_lhs_rhs", name.replace('-', "_"))),
            "{name} is missing from the library"
        );
    }
    assert!(
        text.contains("(import std.algebra"),
        "the fixture is supposed to import its algebra"
    );
    for imported in ["rule_symmetric_lhs_rhs", "rule_transitive_lhs_rhs"] {
        assert!(
            !dot.contains(imported),
            "{imported} was resolved through the import — this is a static view"
        );
    }

    let constraints = render_constraints(&ast, &forms, "constraints", false);
    assert!(constraints.starts_with("digraph constraints {"));

    // The guard's condition: no rule forms → nothing to draw at all.
    let (empty_ast, empty_forms) = parsed("(relation r A B)\n(r a b :source \"(1)\")");
    let none = rule_forms(&empty_ast, &empty_forms);
    assert!(none.is_empty());
    assert_eq!(
        render_rules_forms(&empty_ast, &none, RuleMode::SideBySide, false),
        ""
    );
}

/// **cli-render-rule-by-name.** `--name X` draws X alone, and `--rule-mode
/// overlay` draws it as one graph instead of two panels.
///
/// `co-located` is the by-name target because it is puzzle-local — four
/// parameters puts it past relation algebra's three-variable ceiling, so it
/// cannot migrate into `std.algebra` and stop being addressable. The digraph
/// name is the assertion that matters: it carries the sanitised rule name, so
/// a lookup that matched `co-located-fanout` by prefix, or a mode that ignored
/// its argument, both fail here rather than producing a picture of the wrong
/// rule under the right title.
#[test]
fn a_rule_renders_by_name_and_the_overlay_drops_the_panels() {
    let text = source("examples/zebra2.ein");
    let (ast, forms) = parsed(&text);
    let rule = rule_named(&ast, &forms, "co-located").expect("zebra2 defines co-located inline");

    let side = render_rule_form(&ast, rule, RuleMode::SideBySide, false);
    assert!(
        side.starts_with("digraph rule_co_located_lhs_rhs {"),
        "{side}"
    );
    assert_eq!(
        side.matches("digraph ").count(),
        1,
        "more than one rule drawn"
    );
    assert!(side.contains("subgraph cluster_lhs"), "{side}");
    assert!(side.contains("subgraph cluster_rhs"), "{side}");

    let overlay = render_rule_form(&ast, rule, RuleMode::Overlay, false);
    assert!(
        overlay.starts_with("digraph rule_co_located_overlay {"),
        "{overlay}"
    );
    assert!(!overlay.contains("cluster_lhs"), "{overlay}");
    assert!(!overlay.contains("cluster_rhs"), "{overlay}");

    // The neighbours a prefix match would have caught, and the miss the
    // subcommand turns into its exit-1.
    assert!(rule_named(&ast, &forms, "co-located-fanout").is_some());
    assert!(rule_named(&ast, &forms, "no-such-rule").is_none());
    assert!(
        rule_named(&ast, &forms, "symmetric").is_none(),
        "an imported rule is not addressable by --name in a static view"
    );
}

/// The clusters of one rule's drawing, `cluster_absent_…`, each with the
/// lines from its opening to its closing brace, in the order opened.
fn absent_clusters(dot: &str) -> Vec<(String, Vec<String>)> {
    let lines: Vec<&str> = dot.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(id) = line.trim().strip_prefix("subgraph cluster_absent_") else {
            continue;
        };
        let mut depth = 0i32;
        let mut body = Vec::new();
        for l in &lines[i..] {
            depth += l.matches('{').count() as i32 - l.matches('}').count() as i32;
            body.push(l.trim().to_string());
            if depth == 0 {
                break;
            }
        }
        out.push((id.trim_end_matches(" {").to_string(), body));
    }
    out
}

/// **An `absent` is a cluster at any depth.** `(absent (and G (absent (not
/// B))))`, which `std.macro`'s `forall` expands to, and `(absent (and G (not
/// B)))` say two things: no `G` whose `B` is not stored negated, and no `G`
/// whose `B` is. Drawn alike, the inner absence read as a red pattern, the
/// picture says the second of the first. So the inner `absent` is a cluster
/// inside the outer one, holding what it alone names, the pattern of `not`
/// and the box `not`, and the two rules draw differently; a pattern under an
/// `absent` is drawn in its relation's colour, as anywhere, and only `not`'s
/// is red. An `absent` in an arm of an `(or …)` is a cluster too.
#[test]
fn an_absent_under_an_absent_is_a_cluster_inside_it() {
    let text = "\
(rule two-absences (?R ?isa)
  :match  (and (?isa ?b ?B)
               (absent (and (?isa ?o ?B) (neq ?o ?b) (absent (not (?R ?a ?o))))))
  :assert (?R ?a ?b))
(rule one-absence (?R ?isa)
  :match  (and (?isa ?b ?B)
               (absent (and (?isa ?o ?B) (neq ?o ?b) (not (?R ?a ?o)))))
  :assert (?R ?a ?b))
(rule unmarked (?g)
  :match  (and (group ?g) (absent (and (member ?y ?g) (absent (marked ?y)))))
  :assert (done ?g))
(rule either ()
  :match  (or (and (p ?x) (q ?x)) (and (p ?x) (absent (r ?x))))
  :assert (q ?x))";
    let (ast, forms) = parsed(text);
    let draw = |name: &str| {
        let rule = rule_named(&ast, &forms, name).expect(name);
        render_rule_form(&ast, rule, RuleMode::SideBySide, false)
    };
    let (two, one) = (draw("two-absences"), draw("one-absence"));
    let unnamed =
        |dot: &str, name: &str| dot.replace(&name.replace('-', "_"), "").replace(name, "");
    assert_ne!(
        unnamed(&two, "two-absences"),
        unnamed(&one, "one-absence"),
        "two rules that mean different things are drawn alike"
    );

    // One absence: one cluster, the pattern of `not` in it.
    let clusters = absent_clusters(&one);
    assert_eq!(clusters.len(), 1, "{one}");
    assert!(
        clusters[0]
            .1
            .iter()
            .any(|l| l == "\"not_L\" [label=\"not\", shape=box];"),
        "{one}"
    );

    // Two: the inner cluster inside the outer, holding the box `not` and the
    // pattern of `not`; the outer holds them only by holding the inner.
    let clusters = absent_clusters(&two);
    let ids: Vec<&str> = clusters.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, ["L_0", "L_0_0"], "{two}");
    let (outer, inner) = (&clusters[0].1, &clusters[1].1);
    assert!(
        outer.windows(inner.len()).any(|w| w == inner.as_slice()),
        "the inner absence is not inside the outer one: {two}"
    );
    let not_box = "\"not_L\" [label=\"not\", shape=box];";
    assert!(inner.iter().any(|l| l == not_box), "{two}");
    assert_eq!(two.matches(not_box).count(), 1, "{two}");
    assert!(
        inner
            .iter()
            .any(|l| l.starts_with("\"not_L\" -> \"h")
                && l.contains("_not_L\" [xlabel=\"1\", dir=back")),
        "{two}"
    );
    // `?o` is named by both absences, so it is the outer one's.
    let o = "\"?o_L\" [label=\"?o\", shape=diamond];";
    assert!(
        outer.iter().any(|l| l == o) && !inner.iter().any(|l| l == o),
        "{two}"
    );

    // `forall` written out: the pattern of `marked` in the inner cluster, in
    // its relation's colour, not red.
    let unmarked = draw("unmarked");
    let clusters = absent_clusters(&unmarked);
    assert_eq!(clusters.len(), 2, "{unmarked}");
    let marked = clusters[1]
        .1
        .iter()
        .find(|l| l.starts_with("\"h") && l.contains("_marked_L\" [shape=octagon"))
        .unwrap_or_else(|| panic!("the pattern of marked is in the inner cluster: {unmarked}"));
    assert!(!marked.contains("#d62728"), "{marked}");

    // An `absent` in an arm of an `(or …)`: its cluster, `r` in it.
    let either = draw("either");
    let clusters = absent_clusters(&either);
    assert_eq!(clusters.len(), 1, "{either}");
    assert!(
        clusters[0]
            .1
            .iter()
            .any(|l| l == "\"r_L\" [label=\"r\", shape=box];"),
        "{either}"
    );
}

// ── the refutation slice ───────────────────────────────────────────

/// **bottom-edges-sorted-by-repr.** The `→ ⊥` edges are in `repr` order,
/// whatever order the detector collected the core in.
///
/// It matters because this DOT block is embedded
/// verbatim in the `--trace` markdown, so an edge order that followed a hash
/// set's iteration order would make one puzzle produce two different trace
/// files across runs. `repr` rather than a native comparison because a core
/// can hold facts with mixed argument types, which have no total order
/// otherwise ([01-knowledge-base.md § The text of an object](../../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md#the-text-of-an-object)).
/// Six facts, fed in scrambled order, so a collection order that
/// happens to match sorted order is 1-in-720 rather than a coin flip.
#[test]
fn the_bottom_edges_of_a_refutation_are_sorted_not_collection_ordered() {
    let mut terms = Terms::new();
    let rel = terms.intern_text("co-located").expect("room");
    let mut fact = |colour: &str, house: &str| -> FactId {
        let args: Vec<Value> = [colour, house]
            .iter()
            .map(|a| terms.value_text(a).expect("room"))
            .collect();
        terms.intern_fact(rel, &args).expect("room")
    };
    // Deliberately not sorted, and not reverse-sorted either.
    let core: Vec<FactId> = [
        ("Red", "H1"),
        ("Yellow", "H5"),
        ("Blue", "H3"),
        ("Ivory", "H4"),
        ("Green", "H2"),
        ("Amber", "H6"),
    ]
    .iter()
    .map(|(c, h)| fact(c, h))
    .collect();

    let dot = render_slice(
        &terms,
        &[],
        &[],
        None,
        "reductio",
        Some((&core, &[])),
        None,
        false,
    );

    // Read the order back off the graph: the node ids are content hashes of
    // the facts, which no label carries (Q-M1f.419).
    let id_of =
        |c: &str, h: &str| ein_render::hashed_id("f_", &format!("co-located|{c},{h}"), true);
    let got: Vec<String> = dot
        .lines()
        .filter_map(|l| {
            l.trim()
                .strip_suffix("-> \"⊥\" [color=\"#d62728\", penwidth=2];")
        })
        .map(|src| src.trim().to_string())
        .collect();
    let want: Vec<String> = [
        ("Amber", "H6"),
        ("Blue", "H3"),
        ("Green", "H2"),
        ("Ivory", "H4"),
        ("Red", "H1"),
        ("Yellow", "H5"),
    ]
    .iter()
    .map(|(c, h)| id_of(c, h))
    .collect();
    assert_eq!(got, want, "the ⊥ edges are not in repr order:\n{dot}");
}

// ── a derivation's three levels ─────────────────────────────────────

/// **A derivation is drawn at three levels** (Q-M1f.428): the fact a step
/// derives, its octagon and every arrow of it, is the brightest, in the
/// derived shade of the step's hue and wider than the rest; the step and the
/// derivation's arrows, from each premise and to what it derives, are the
/// same hue's explanation shade at twice the default width; a premise no
/// step derives is the rest, Graphviz's default black at the default width,
/// its octagon and its arrows naming no colour and no width.
#[test]
fn a_derivation_is_drawn_at_three_levels() {
    use ein_render::slice::{DERIVED_WIDTH, derivation_hue};
    let mut terms = Terms::new();
    let atom = |terms: &mut Terms, n: &str| terms.value_text(n).expect("room");
    let (a, b, c) = (
        atom(&mut terms, "a"),
        atom(&mut terms, "b"),
        atom(&mut terms, "c"),
    );
    let rel = terms.intern_text("r").expect("room");
    let ab = terms.intern_fact(rel, &[a, b]).expect("room");
    let bc = terms.intern_fact(rel, &[b, c]).expect("room");
    let ac = terms.intern_fact(rel, &[a, c]).expect("room");
    let firing = ein_infer::firing::Firing {
        rule: terms.intern_text("triangle").expect("room"),
        activator: Box::new([]),
        bindings: Box::new([]),
        derived: Box::new([ac]),
        premises: Box::new([ab, bc]),
        redundant: false,
    };
    let dot = render_slice(&terms, &[], &[firing], None, "step", None, None, false);
    let hue = derivation_hue("triangle");
    assert_ne!(hue.explanation, hue.derived);
    let id = |x: &str, y: &str| ein_render::hashed_id("f_", &format!("r|{x},{y}"), true);
    let step = "\"fire0_triangle\"";
    // A fact's own lines: its octagon and its arrows, not the step's.
    let lines_of = |f: &str| -> Vec<&str> {
        dot.lines()
            .filter(|l| !l.contains(step))
            .filter(|l| {
                let l = l.trim();
                l.starts_with(&format!("{f} ")) || l.contains(&format!("-> {f} [xlabel"))
            })
            .collect()
    };
    // The fact derived: its octagon and its three arrows.
    let derived = lines_of(&id("a", "c"));
    assert_eq!(derived.len(), 4, "{dot}");
    for l in &derived {
        assert!(
            l.contains(&format!("color=\"{}\"", hue.derived))
                && l.contains(&format!("penwidth={DERIVED_WIDTH}")),
            "the derived fact is not the brightest and widest: {l}\n{dot}"
        );
    }
    // The step and the derivation's three arrows.
    let explanation: Vec<&str> = dot.lines().filter(|l| l.contains(step)).collect();
    assert_eq!(explanation.len(), 4, "{dot}");
    for l in &explanation {
        assert!(
            l.contains(&format!("color=\"{}\"", hue.explanation)) && l.contains("penwidth=2"),
            "the derivation's explanation is not its hue's other shade at width 2: {l}\n{dot}"
        );
    }
    // Each premise, its octagon and its arrows: no colour and no width.
    for (x, y) in [("a", "b"), ("b", "c")] {
        let premise = lines_of(&id(x, y));
        assert_eq!(premise.len(), 4, "{dot}");
        for l in &premise {
            assert!(
                !l.contains("color=") && !l.contains("penwidth"),
                "a premise is marked: {l}\n{dot}"
            );
        }
    }
}

/// **A redundant firing adds nothing**: what it derives was there before
/// it, so its conclusion is drawn as the rest, while its step, dashed, and
/// its arrows are the derivation's explanation.
#[test]
fn a_redundant_firings_conclusion_is_not_drawn_as_added() {
    use ein_render::slice::derivation_hue;
    let mut terms = Terms::new();
    let (a, b) = (
        terms.value_text("a").expect("room"),
        terms.value_text("b").expect("room"),
    );
    let rel = terms.intern_text("r").expect("room");
    let ab = terms.intern_fact(rel, &[a, b]).expect("room");
    let ba = terms.intern_fact(rel, &[b, a]).expect("room");
    let firing = ein_infer::firing::Firing {
        rule: terms.intern_text("mirror").expect("room"),
        activator: Box::new([]),
        bindings: Box::new([]),
        derived: Box::new([ab]),
        premises: Box::new([ba]),
        redundant: true,
    };
    let dot = render_slice(&terms, &[], &[firing], None, "step", None, None, false);
    let hue = derivation_hue("mirror");
    assert!(
        !dot.contains(hue.derived),
        "a redundant firing added a fact:\n{dot}"
    );
    assert!(
        dot.contains("style=\"rounded,dashed\"") && dot.contains(hue.explanation),
        "{dot}"
    );
}

/// WCAG's relative luminance of a `#rrggbb` colour.
fn luminance(hex: &str) -> f64 {
    let channel = |i: usize| {
        let v = f64::from(u8::from_str_radix(&hex[i..i + 2], 16).expect("hex")) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5)
}

/// **No shade of a derivation is near black, and each reads on white**
/// (Q-M1f.428): every shade at a contrast of more than 3.5 against black;
/// the explanation's, in which a step's label is written, at 5.5 or more
/// against white, the derived fact's, a line, at 3 or more; and the derived
/// fact's the brighter of its hue's two.
#[test]
fn no_shade_of_the_derivation_palette_is_near_black() {
    let against_black = |c: &str| (luminance(c) + 0.05) / 0.05;
    let against_white = |c: &str| 1.05 / (luminance(c) + 0.05);
    for hue in ein_render::slice::DERIVATION_PALETTE {
        for shade in [hue.explanation, hue.derived] {
            assert!(against_black(shade) > 3.5, "{shade} is near black");
        }
        assert!(
            against_white(hue.explanation) >= 5.5,
            "{} does not read on white",
            hue.explanation
        );
        assert!(
            against_white(hue.derived) >= 3.0,
            "{} does not read on white",
            hue.derived
        );
        assert!(
            luminance(hue.derived) > luminance(hue.explanation),
            "{hue:?}: the derived fact's shade is not the brighter"
        );
    }
}

// ── the read-out's dead hole ───────────────────────────────────────

/// **A dead hole is read out as dead, with what it died of.** The read-out
/// has a line for a minimal hole of each kind, `— owes n` and `— dead: core`,
/// and the cube's status counts dead nodes with an alive superset. *Forbid*
/// refuses every program of the corpus with a dead hole — the witnesses of
/// the regressions `naf-upward-closure`, `naf-nowb-L`, `three-components` and
/// `trigger-hole` among them — so no `ein solve` prints the second kind,
/// and the line is held here: `naf-upward-closure.ein`, read
/// through `forbid::studying`, the library-level way past the refusal that no
/// CLI path takes. The program is refused outside it, which is the other half.
#[test]
fn a_dead_hole_is_read_out_as_dead_with_its_core() {
    let rel = "examples/ein-bugs/naf-upward-closure.ein";
    let path = repo_root().join(rel);
    let text = source(rel);
    let run = || {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
        let opts = ein_infer::explore::Options::new(ein_infer::explore::Mode {
            which: ein_infer::explore::Which::All,
            ask: ein_infer::explore::Ask::Describe,
        });
        ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
    };
    assert!(matches!(run(), Err(ein_infer::SolveError::Refused(_))));
    let r = ein_infer::forbid::studying(run).expect("studied, it answers");
    let out = ein_render::read_out::render(
        &r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: rel,
            file: rel,
            budget: ein_infer::downset::DEFAULT_BUDGET,
            uncut: false,
        },
    )
    .stdout;
    let lines: Vec<&str> = out.lines().collect();
    assert!(
        lines.contains(&"    minimal hole    {(p A)} — dead: (p A)"),
        "{out}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with("    status ")
            && l.contains("(exercised, by bad)")
            && l.contains("1 hole · 1 dead node with an alive superset")),
        "{out}"
    );
    assert!(
        lines.contains(&"  lattice           not monotone: 1 site (the static check) — 1 hole"),
        "{out}"
    );
}
