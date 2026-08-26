//! T1a.10.2.2 — presentation as **behaviour**, not as bytes.
//!
//! The bytes of every renderer are already pinned twice over: `corpus_shapes.rs`
//! digests all seventeen DOT views of every corpus entry, and `golden_dot.rs` /
//! `golden_trace.rs` / `golden_slice.rs` reproduce ein.py's committed goldens.
//! What none of those say is *what the picture means* — a golden that changed
//! for a good reason is refreshed and the claim behind it is lost. These are
//! the claims: the ones a reader of the DOT would state in a sentence, each
//! written so that the wrong picture fails rather than a different one.
//!
//! Replaces the semantic half of five Python files, all deleted with ein.py at
//! [P1a.10](../../../../docs/history/m1a_rust/README.md#p1a10--one-implementation):
//!
//! | Python | subject kept here |
//! |---|---|
//! | `ein.py/tests/render/test_lattice_dag.py` | the lattice under a shuffled traversal, the no-good back-edge, the derivation-DAG trace view |
//! | `ein.py/tests/render/test_rules_dot.py` | `render rules` / `render rule --name` as *static file views* |
//! | `ein.py/tests/render/test_slice_dot.py` | the `→ ⊥` edge order (hazard H4) |
//! | `ein.py/tests/test_ir_to_dot.py` | the per-form dispatch's own shapes, and mode/view name resolution |
//! | `ein.py/tests/trace/test_answer.py`, `tests/trace/test_render.py` | where the English comes from, and what the four `--trace` flags do |
//!
//! **On the five entries the work-list called `cli-…`.** `ein-render` cannot
//! depend on `ein-cli` — that is the dependency edge, backwards — so what is
//! asserted here is the body each subcommand is a five-line wrapper around
//! (`ein-cli/src/render.rs`, `ein-cli/src/solve.rs::write_trace`): the
//! renderer call, its input, and the condition the subcommand turns into its
//! exit code. The exit codes and the two stderr sentences themselves are
//! surface, and `ein-cli/tests/help_parity.rs` is where that surface lives.

use ein_core::{FactId, Kb, SolverConfig, Terms, Value};
use ein_infer::events::{Buffer, Level};
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::verdict::{Answer, Solution, Verdict};
use ein_infer::{Events, Solved};
use ein_ir::{Ast, NodeId, load_file, parse};
use ein_render::ir_dot::{DotOpts, TraceView, render_trace, to_dot};
use ein_render::lattice_dag::{LatticeSource, LatticeView, render_lattice};
use ein_render::rules::{RuleMode, render_rule_form, render_rules_forms};
use ein_render::slice::render_slice;
use ein_render::trace::{LinearizeOpts, Mode, linearize, render_markdown};
use ein_render::{ModelsForm, render_answer, render_constraints, render_solution_table};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// The verdict colours `render/lattice_dag` fills a cell with.
const SOLUTION_GREEN: &str = "fillcolor=\"#e8f6e8\"";
const DEAD_RED: &str = "fillcolor=\"#fdeaea\"";

// ── fixtures ───────────────────────────────────────────────────────

/// Parse without loading — the *static file view* every `ein render` subcommand
/// except `lattice` takes, and the reason an imported rule is invisible to it.
fn parsed(text: &str) -> (Ast, Vec<NodeId>) {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
    (ast, forms)
}

fn loaded(text: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    (ast, terms, kb)
}

fn source(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).expect("corpus file")
}

/// A solved corpus file, with everything the renderers need kept alive.
struct Run {
    ast: Ast,
    terms: Terms,
    kb: Kb,
    solved: Solved,
}

impl Run {
    fn lattice(&self, view: LatticeView) -> String {
        let proof = self.solved.proof.as_ref().expect("store_lattice was on");
        render_lattice(&self.terms, LatticeSource::Proof(proof), view, "lattice")
    }
}

/// `ein render lattice` / `ein solve --trace`'s solve: `store_lattice` on and
/// the subcommand's own `--max-set-size 3`.
fn solve_file(rel: &str, store_lattice: bool, seed: Option<i64>) -> Run {
    solve_logged(rel, store_lattice, seed).0
}

/// The same, with the verbose event stream captured — the only view of the
/// *traversal* as opposed to the answer.
fn solve_logged(rel: &str, store_lattice: bool, seed: Option<i64>) -> (Run, String) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        store_lattice,
        config: seed.map(|s| SolverConfig {
            lattice_order_seed: Some(s),
            ..SolverConfig::default()
        }),
        ..SolveOptions::default()
    };
    let solved =
        solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).expect("solves");
    let log = buffer.to_string_lossy();
    (
        Run {
            ast,
            terms,
            kb,
            solved,
        },
        log,
    )
}

/// The commitments the search entered, in the order it entered them.
fn enter_order(log: &str) -> Vec<String> {
    log.lines()
        .filter(|l| l.contains("\"enter\""))
        .map(|l| l.to_string())
        .collect()
}

// ── the commitment lattice ─────────────────────────────────────────

/// **lattice-render-is-seed-invariant.** A shuffled traversal draws the same
/// lattice.
///
/// `lattice-order-seed` permutes each layer's candidate order, so the engine
/// meets the same commitments in a different sequence — and the picture must
/// not know. It would know if any cell's identity, order or verdict came from
/// *when* it was visited rather than from *what* it is; the renderer instead
/// keys every cell on `repr` of its representative commitment. The seeded run's
/// `enter` stream is compared too, and that half is what stops this from
/// passing vacuously: if the shuffle were a no-op the traversal would be
/// identical and equal pictures would prove nothing.
#[test]
fn a_shuffled_traversal_draws_the_same_lattice() {
    for rel in [
        "examples/branching/04_two_levels.ein",
        "examples/lattice/03_state_hash_collision.ein",
    ] {
        let (base, base_log) = solve_logged(rel, true, None);
        let picture = base.lattice(LatticeView::Solution);
        let mut moved = false;
        for seed in [0, 3, 9] {
            let (run, log) = solve_logged(rel, true, Some(seed));
            assert_eq!(
                run.lattice(LatticeView::Solution),
                picture,
                "{rel}: the rendered lattice moved under lattice-order-seed {seed}"
            );
            moved |= enter_order(&log) != enter_order(&base_log);
        }
        assert!(
            moved,
            "{rel}: no seed changed the traversal — the invariance above is vacuous"
        );
    }
}

/// **cli-render-lattice-runs-a-solve.** The body of `ein render lattice`: a
/// solve, and a `digraph lattice` whose dead cells keep their no-good.
///
/// The interesting half is `lattice/01_subset_pruned`, where the `{a, b}`
/// commitment dies while the puzzle as a whole is satisfiable. A renderer that
/// drew only the surviving frontier would produce a picture with nothing wrong
/// in it and lose the one thing the lattice is for — that a branch was tried,
/// refuted, and lifted into a clause that pruned its supersets.
#[test]
fn the_lattice_keeps_a_refuted_commitment_in_a_satisfiable_puzzle() {
    let solutions = solve_file("examples/branching/04_two_levels.ein", true, None);
    let dot = solutions.lattice(LatticeView::Solution);
    assert!(
        dot.starts_with("digraph lattice {"),
        "not a lattice digraph"
    );
    assert!(dot.contains(SOLUTION_GREEN), "no solution cell is green");

    let pruned = solve_file("examples/lattice/01_subset_pruned.ein", true, None);
    assert!(
        matches!(
            pruned.solved.answer,
            Answer::Verdict(Verdict::Solution(_) | Verdict::Ambiguity(_))
        ),
        "the fixture is supposed to be satisfiable overall"
    );
    let dot = pruned.lattice(LatticeView::Solution);
    assert!(dot.contains(DEAD_RED), "the {{a, b}} death is not drawn");
    let backedges: Vec<&str> = dot
        .lines()
        .filter(|l| l.contains("label=\"no-good\"") && l.contains("style=dashed"))
        .collect();
    assert!(
        !backedges.is_empty(),
        "a dead commitment drew no dashed no-good back-edge:\n{dot}"
    );
    assert!(
        dot.contains("unsat-core"),
        "a dead cell carries no unsat-core tooltip"
    );
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
/// that should never become a node. The failure it guards is not a crash but
/// a plausible-looking graph — `s1` as a fresh rectangle pointing at `s2`'s
/// conclusion, which reads as "step s1 justifies this" instead of "the
/// Norwegian being in House-1 justifies this".
#[test]
fn the_dag_trace_view_chains_through_derived_facts() {
    let (ast, forms) = parsed(TRACE_CHAIN);
    let dag = render_trace(&ast, forms[0], TraceView::Dag);
    assert!(
        dag.contains(
            "\"(lives-in Norwegian House-1)\" -> \"(not (lives-in Norwegian House-2))\" \
             [label=\"exclusivity\"];"
        ),
        "s2 does not hang off s1's derived fact, labelled by its rule:\n{dag}"
    );
    assert!(
        !dag.contains("\"s1\""),
        "the step id leaked into the graph as a node:\n{dag}"
    );
    // The premise that is *not* a prior step keeps its rectangle: `c10` is an
    // input, and the dag has nowhere else to root the chain.
    assert!(dag.contains("\"c10\" [shape=rectangle];"), "{dag}");
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
/// The trace AST is open — `branch-open`, `branch-close`, `contradiction`,
/// `symmetry-class` are events with the same `(head name :using …)` shape as a
/// step. Drawing them identically would say they are derivations, which is the
/// one thing they are not: nothing follows from a branch closing. The ellipse
/// and the `"{kind}: {name}"` label are how the reader tells narration from
/// inference.
#[test]
fn only_a_step_event_is_a_box_the_rest_are_ellipses() {
    let (ast, forms) = parsed(
        "\
(trace
  (step s1 :rule from-condition :using (c10) :derives (lives-in Norwegian House-1))
  (branch-open b1 :using (c11))
  (branch-close b2)
  (contradiction x1)
  (symmetry-class sc1))",
    );
    let dot = render_trace(&ast, forms[0], TraceView::PerStep);
    assert!(
        dot.contains("\"s1\" [shape=box, label=\"step: s1\"];"),
        "{dot}"
    );
    for (id, kind) in [
        ("b1", "branch-open"),
        ("b2", "branch-close"),
        ("x1", "contradiction"),
        ("sc1", "symmetry-class"),
    ] {
        assert!(
            dot.contains(&format!(
                "\"{id}\" [shape=ellipse, label=\"{kind}: {id}\"];"
            )),
            "{kind} is not an ellipse labelled by its kind:\n{dot}"
        );
    }
    // A premise is a dashed rectangle whichever kind of event consumes it.
    assert!(dot.contains("\"c10\" [shape=rectangle];"), "{dot}");
    assert!(dot.contains("\"c11\" -> \"b1\" [style=dashed];"), "{dot}");
}

// ── the per-form IR dispatch ───────────────────────────────────────

/// **equality-fact-renders-as-an-equality-class.** `(= a b)` draws a class,
/// not an arrow.
///
/// Every other binary fact collapses to `a -> b` labelled by its relation, and
/// `=` deliberately does not: equality is symmetric and transitive, so an
/// arrow would assert a direction the fact does not have. The double-circle
/// node with an edge to each side is the class, and it is the one node shape
/// no corpus file produces — which is exactly why the byte goldens do not
/// cover it and this does.
#[test]
fn an_equality_fact_draws_a_class_node_rather_than_an_arrow() {
    let (ast, forms) = parsed("(= a b :source \"(1)\")");
    let dot = to_dot(&ast, &forms, DotOpts::default());
    assert!(
        dot.contains("\"eq_a_b\" [shape=doublecircle, label=\"=\"];"),
        "{dot}"
    );
    assert!(dot.contains("\"eq_a_b\" -> \"a\";"), "{dot}");
    assert!(dot.contains("\"eq_a_b\" -> \"b\";"), "{dot}");
    assert!(
        !dot.contains("\"a\" -> \"b\""),
        "equality drew a directed edge between the sides:\n{dot}"
    );
    // The contrast: an ordinary binary fact *is* the collapsed arrow.
    let (ast, forms) = parsed("(r a b :source \"(1)\")");
    let plain = to_dot(&ast, &forms, DotOpts::default());
    assert!(plain.contains("\"a\" -> \"b\""), "{plain}");
    assert!(!plain.contains("doublecircle"), "{plain}");
}

/// **wildcard-and-var-heads-render-as-their-labels.** A pattern head that is
/// not an atom still labels its edge.
///
/// `(_ ?a ?b)` and `(?r ?a ?b)` are the two relation-abstract patterns the
/// language has, and both have no relation *name* to colour or label an edge
/// with. Dropping the clause — the easy failure, since every other path reads
/// a head atom — would render a rule library in which the generic rules are
/// blank, and the rules that quantify over relations are the interesting ones.
#[test]
fn a_wildcard_or_variable_pattern_head_still_labels_its_edge() {
    let (ast, forms) = parsed(
        "(rule w () :match (_ ?a ?b) :assert ?a :why \"w\")\n\
         (rule v (?r) :match (?r ?a ?b) :assert ?a :why \"v\")",
    );
    let wildcard = render_rule_form(&ast, forms[0], RuleMode::Overlay);
    assert!(
        wildcard.contains("\"?a\" -> \"?b\" [label=\"_\""),
        "the wildcard head did not become the edge label:\n{wildcard}"
    );
    let var = render_rule_form(&ast, forms[1], RuleMode::Overlay);
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
    let dot = render_rules_forms(&ast, &rules, RuleMode::SideBySide);
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

    let constraints = render_constraints(&ast, &forms, "constraints");
    assert!(constraints.starts_with("digraph constraints {"));

    // The guard's condition: no rule forms → nothing to draw at all.
    let (empty_ast, empty_forms) = parsed("(relation r A B)\n(r a b :source \"(1)\")");
    let none = rule_forms(&empty_ast, &empty_forms);
    assert!(none.is_empty());
    assert_eq!(
        render_rules_forms(&empty_ast, &none, RuleMode::SideBySide),
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

    let side = render_rule_form(&ast, rule, RuleMode::SideBySide);
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

    let overlay = render_rule_form(&ast, rule, RuleMode::Overlay);
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

// ── the refutation slice ───────────────────────────────────────────

/// **bottom-edges-sorted-by-repr.** The `→ ⊥` edges are in `repr` order,
/// whatever order the detector collected the core in.
///
/// M1a hazard H4, and the reason it mattered: this DOT block is embedded
/// verbatim in the `--trace` markdown, so an edge order that followed a hash
/// set's iteration order made one puzzle produce two different trace files
/// across runs. `repr` rather than a native comparison because a core can hold
/// facts with mixed argument types, which have no total order otherwise
/// (Q-M1a.4). Six facts, fed in scrambled order, so a collection order that
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

    let dot = render_slice(&terms, &[], &[], None, "reductio", Some((&core, &[])), None);

    // Read the order back off the graph: the node ids are content hashes, so
    // the labels are what a reader sees.
    let mut label_of: Vec<(String, String)> = Vec::new();
    for line in dot.lines() {
        let line = line.trim();
        if let Some((id, rest)) = line.split_once(" [label=\"")
            && let Some((label, _)) = rest.split_once('"')
        {
            label_of.push((id.to_string(), label.to_string()));
        }
    }
    let got: Vec<&str> = dot
        .lines()
        .filter_map(|l| l.trim().strip_suffix("-> \"⊥\" [color=\"#d62728\"];"))
        .map(|src| {
            let src = src.trim();
            label_of
                .iter()
                .find(|(id, _)| id == src)
                .map(|(_, label)| label.as_str())
                .expect("every edge source is a declared node")
        })
        .collect();
    assert_eq!(
        got,
        [
            "co-located(Amber, H6)",
            "co-located(Blue, H3)",
            "co-located(Green, H2)",
            "co-located(Ivory, H4)",
            "co-located(Red, H1)",
            "co-located(Yellow, H5)",
        ],
        "the ⊥ edges are not in repr order:\n{dot}"
    );
}

// ── the answer: every English word comes from the puzzle ───────────

/// A complete model written down as facts, with the two template sources: a
/// per-relation `:why` (positional `{?1}` / `{?2}`) and the query's own
/// `:goal-text` (named after the goal's variables). Nothing is solved — the
/// renderer's input is a model, and building one directly keeps the claim
/// about *rendering*.
const ZEBRA_SHAPED: &str = r#"
(relation drink-loc  Drink       House :why "{?1} is drunk in {?2}")
(relation nation-loc Nationality House :why "the {?1} lives in {?2}")
(relation pet-loc    Pet         House :why "{?2} keeps the {?1}")
(drink-loc  Water     House-1)
(nation-loc Norwegian House-1)
(pet-loc    Zebra     House-5)
(nation-loc Japanese  House-5)
(query
  :goal (and (drink-loc  Water      ?h_water)
             (nation-loc ?who_water ?h_water)
             (pet-loc    Zebra      ?h_zebra)
             (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns the zebra in {?h_zebra}")
"#;

/// The same shape in an unrelated vocabulary: different relation names,
/// different variable names, and one relation with no `:why` at all.
const OWNER_SHAPED: &str = r#"
(relation drink-loc Drink House :why "{?1} drunk at {?2}")
(relation owner-loc Owner House)
(drink-loc Water  House-1)
(owner-loc Zaphod House-1)
(query
  :goal (and (drink-loc Water ?h) (owner-loc ?who ?h))
  :goal-text "{?who} drinks the water")
"#;

/// A model, packaged as the `Solution` the renderers take.
fn answer_of(kb: &mut Kb) -> Answer {
    Answer::Verdict(Verdict::Solution(Solution {
        kb: kb.snapshot(),
        trace: Vec::new(),
    }))
}

/// **goal-text-headline.** The one-line answer is the query's own template,
/// bound to the goal's own variables.
///
/// There is no relation→verb table anywhere in the engine, and this is the
/// test that would find one: the two fixtures use different relations,
/// different variable names and different sentences, and both must come out
/// right. A hardcoded projection — "the nation-loc of the water house" — would
/// pass on the zebra-shaped fixture and fail on `?who` / `owner-loc`, which is
/// precisely the pair. The no-template case pins the other end: absent a
/// `:goal-text` the headline is a neutral `Solved.`, never invented prose.
#[test]
fn the_headline_is_the_querys_own_goal_text() {
    let (ast, mut terms, mut kb) = loaded(ZEBRA_SHAPED);
    let answer = answer_of(&mut kb);
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, true),
        "The Norwegian drinks water in House-1; the Japanese owns the zebra in House-5"
    );

    let (ast, mut terms, mut kb) = loaded(OWNER_SHAPED);
    let answer = answer_of(&mut kb);
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, true),
        "Zaphod drinks the water"
    );

    let (ast, mut terms, mut kb) = loaded(
        "(relation drink-loc Drink House :why \"{?1} at {?2}\")\n\
         (drink-loc Water House-1)\n\
         (query :goal (drink-loc Water ?h))",
    );
    let answer = answer_of(&mut kb);
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, true),
        "Solved."
    );
}

/// **ambiguity-headline-qualifies-its-own-count.** M1d
/// [S1d.3.3](../../../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
/// T1d.3.3.2, on the two surfaces the CLI does not reach.
///
/// A model count is a **claim about a set**, and `exhausted` is what licenses
/// it. The `Solution` headline has carried that distinction since ein.py; the
/// verdict that reports a set carried none, which is the wrong way round —
/// `k = 1` unqualified guesses at uniqueness, `k = 4` unqualified is a wrong
/// number. `branching/02_one_dead_one_alive.ein` is the corpus's proof: at
/// depth 3 the search finds 4 models and does not exhaust; the sharper case
/// is `saturation/type-exclusivity/colors.ein`, which says **5** at the
/// default cap and has **9** at `-m 6`.
///
/// Both proofless surfaces are checked, because they are written in two
/// places and the trace's summary is the one no corpus digest covers — the
/// five `Ambiguity` entries all exhaust at the shape sweep's depth, so
/// `trace[no-proof]` renders the *other* branch of it.
#[test]
fn an_unexhausted_ambiguity_says_the_count_is_a_lower_bound() {
    let run = solve_file("examples/branching/02_one_dead_one_alive.ein", false, None);
    assert!(!run.solved.stats.exhausted, "depth 3 should not exhaust");
    let mut terms = run.terms;
    let answer = &run.solved.answer;
    assert_eq!(
        render_answer(&run.ast, &mut terms, &run.kb, answer, false),
        "Ambiguous — at least 4 distinct complete models; \
         the search did not exhaust the lattice."
    );
    // The same answer rendered as though the lattice had been exhausted: one
    // sentence, and it is the only one entitled to say *the* models.
    assert_eq!(
        render_answer(&run.ast, &mut terms, &run.kb, answer, true),
        "Ambiguous — 4 distinct complete models; the puzzle is under-determined."
    );

    let trace = linearize(&run.ast, &terms, &run.kb, &run.solved, LinearizeOpts::new());
    assert_eq!(
        trace.summary,
        "Ambiguous — at least 4 models (showing one); the search did not exhaust."
    );

    // And the exhausted counterpart, so the assertion above is about the
    // qualifier and not about the file.
    let deep = solve_file("examples/lattice/02_genuine_3set_death.ein", false, None);
    assert!(deep.solved.stats.exhausted);
    let trace = linearize(
        &deep.ast,
        &deep.terms,
        &deep.kb,
        &deep.solved,
        LinearizeOpts::new(),
    );
    assert_eq!(trace.summary, "Ambiguous — 3 models (showing one).");
}

/// **relation-why-positional-table.** Each goal conjunct is rendered through
/// its relation's `:why`, with `{?1}` / `{?2}` bound *positionally*.
///
/// Positional and not by name, because a `:why` belongs to the relation and
/// knows nothing about the variables a query happened to use. `pet-loc`'s
/// template reverses its slots (`{?2} keeps the {?1}`), which is the assertion
/// that separates a positional substitution from one that guessed
/// subject-then-object. The untemplated relation falls back to its raw IR
/// s-expression in the same column — an honest "no words for this" rather than
/// a sentence the puzzle never authorised.
#[test]
fn the_table_renders_each_conjunct_through_its_relations_why() {
    let (ast, mut terms, mut kb) = loaded(ZEBRA_SHAPED);
    let answer = answer_of(&mut kb);
    let table = render_solution_table(
        &ast,
        &mut terms,
        &kb,
        &answer,
        Some(1),
        true,
        None,
        ModelsForm::List,
    )
    .expect("the goal compiles");
    for sentence in [
        "Water is drunk in House-1",
        "the Norwegian lives in House-1",
        "House-5 keeps the Zebra",
        "the Japanese lives in House-5",
    ] {
        assert!(table.contains(sentence), "missing {sentence:?}:\n{table}");
    }

    let (ast, mut terms, mut kb) = loaded(OWNER_SHAPED);
    let answer = answer_of(&mut kb);
    let table = render_solution_table(
        &ast,
        &mut terms,
        &kb,
        &answer,
        Some(1),
        true,
        None,
        ModelsForm::List,
    )
    .expect("the goal compiles");
    assert!(table.contains("Water drunk at House-1"), "{table}");
    assert!(
        table.contains("(owner-loc Zaphod House-1)"),
        "an untemplated relation invented prose instead of falling back:\n{table}"
    );
}

/// **certify-hint-on-templated-headline.** The uniqueness hint rides on a
/// rendered headline and only on a rendered headline.
///
/// `exhausted` is the difference between "here is a model" and "here is the
/// only model", and the hint is the whole of that distinction in the one-line
/// answer — dropping it would let a first-solution run read as a proof. It is
/// appended to the *templated* sentence and not to the neutral `Solved.`,
/// because `Solved.` already promises nothing and a hint on it would advertise
/// a flag that changes nothing the reader can see.
#[test]
fn the_certify_hint_rides_on_the_templated_headline() {
    let (ast, mut terms, mut kb) = loaded(OWNER_SHAPED);
    let answer = answer_of(&mut kb);
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, false),
        "Zaphod drinks the water  (a solution — pass --exhaustive to certify uniqueness)"
    );
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, true),
        "Zaphod drinks the water"
    );

    let (ast, mut terms, mut kb) = loaded(
        "(relation drink-loc Drink House :why \"{?1} at {?2}\")\n\
         (drink-loc Water House-1)\n\
         (query :goal (drink-loc Water ?h))",
    );
    let answer = answer_of(&mut kb);
    assert_eq!(
        render_answer(&ast, &mut terms, &kb, &answer, false),
        "Solved.",
        "the hint attached itself to the neutral headline"
    );
}

// ── the markdown trace ─────────────────────────────────────────────

/// **cli-solve-trace-writes-the-file.** The trace is a document with a
/// numbered spine, and the spine exists because `--trace` turns
/// `store_lattice` on.
///
/// That coupling is the interesting half. `solve` keeps its proof only when
/// asked, so the same run without it still answers — and narrates nothing: no
/// root saturation section, no refuted branches, no lattice. The `wrote OUT (N
/// steps, M refuted)` line the CLI prints to stderr is a format over exactly
/// the two fields compared here, and it is the one thing that would look
/// identical while being empty.
#[test]
fn the_trace_is_a_document_whose_spine_comes_from_the_stored_lattice() {
    let run = solve_file("examples/branching/04_two_levels.ein", true, None);
    let trace = linearize(
        &run.ast,
        &run.terms,
        &run.kb,
        &run.solved,
        LinearizeOpts::new(),
    );
    let md = render_markdown(&trace, Mode::Engine, true);
    assert!(
        md.starts_with("# Solution trace\n"),
        "{}",
        &md[..80.min(md.len())]
    );
    assert!(
        md.contains("\n## Step 1 — "),
        "no numbered first step:\n{md}"
    );
    assert!(!trace.steps.is_empty(), "no derivation spine");
    assert!(!trace.reductios.is_empty(), "no refuted branch to narrate");
    assert!(trace.lattice_dot.is_some(), "no commitment lattice");

    let bare = solve_file("examples/branching/04_two_levels.ein", false, None);
    assert!(bare.solved.proof.is_none(), "store_lattice was off");
    let thin = linearize(
        &bare.ast,
        &bare.terms,
        &bare.kb,
        &bare.solved,
        LinearizeOpts::new(),
    );
    assert!(
        thin.reductios.is_empty() && thin.root_steps.is_empty() && thin.lattice_dot.is_none(),
        "a proofless solve narrated a proof"
    );
}

/// **cli-trace-shaping-flags.** Each of the four `--trace` shapers changes the
/// document in its own way.
///
/// They are four independent knobs on one renderer, and the failure they guard
/// is a flag that parses and then does nothing — the kind that survives a byte
/// golden of the default output indefinitely. So each is asserted by the
/// difference it makes: no fenced `dot` blocks at all, a summary that says how
/// much was pruned, entity headings instead of step headings, and the whole-KB
/// section appended. `--relevant` also has to *shrink* the trace; a prune that
/// kept everything would still say "pruned to".
#[test]
fn each_trace_shaping_flag_changes_the_document() {
    let run = solve_file("examples/branching/04_two_levels.ein", true, None);
    let trace = |opts: LinearizeOpts| linearize(&run.ast, &run.terms, &run.kb, &run.solved, opts);

    let full = trace(LinearizeOpts::new());
    let default_md = render_markdown(&full, Mode::Engine, true);
    assert!(
        default_md.contains("```dot"),
        "the default trace has diagrams"
    );

    // --no-diagrams
    let plain = trace(LinearizeOpts {
        diagrams: false,
        ..LinearizeOpts::new()
    });
    let plain_md = render_markdown(&plain, Mode::Engine, false);
    assert!(
        !plain_md.contains("```dot"),
        "a dot block survived --no-diagrams"
    );

    // --relevant
    let pruned = trace(LinearizeOpts {
        diagrams: false,
        relevant: true,
        ..LinearizeOpts::new()
    });
    assert!(
        pruned.summary.contains("pruned to"),
        "the summary does not report the prune: {}",
        pruned.summary
    );
    assert!(
        pruned.steps.len() < full.steps.len(),
        "the relevant slice ({}) is no smaller than the full one ({})",
        pruned.steps.len(),
        full.steps.len()
    );

    // --reorder
    let reordered = render_markdown(&plain, Mode::Reorder, false);
    assert!(reordered.contains("\n## About "), "no by-entity grouping");
    assert!(
        !plain_md.contains("\n## About "),
        "engine order grouped by entity"
    );

    // --full-kb-snapshots
    let snapshot = trace(LinearizeOpts {
        diagrams: false,
        full_kb_snapshots: true,
        ..LinearizeOpts::new()
    });
    let snapshot_md = render_markdown(&snapshot, Mode::Engine, false);
    assert!(
        snapshot_md.contains("## Full KB (final state)"),
        "no whole-KB section"
    );
    assert!(
        !plain_md.contains("## Full KB"),
        "the KB snapshot is not opt-in"
    );
}
