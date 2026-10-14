//! S1a.5.1 acceptance — the checked-in DOT goldens, byte for byte.
//!
//! Fifteen files under `ein.rs/crates/ein-render/tests/golden/dot/`, plus
//! `kb_zebra_unified.dot` beside them, lock the bytes of every DOT emitter.
//! Each case builds a deterministic input and renders it. One is covered
//! elsewhere and is not repeated: `kb_provenance_dag` by `derivation_dot.rs`,
//! which landed with the provenance walk it renders.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-render --test golden_dot   # regenerate them
//! ```

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_corpus::{golden, golden_path, repo_root};
use ein_infer::commitment::Kind;
use ein_infer::firing::Firing;
use ein_infer::solve::{DeadCommitment, LatticeProof, LatticeStats, SolutionRecord};
use ein_ir::{Ast, parse};
use ein_render::ir_dot::{DotOpts, TraceView, render_query, render_trace, to_dot};
use ein_render::kb_dot::KbDotOpts;
use ein_render::lattice_dag::{LatticeSource, LatticeView, render_lattice};
use ein_render::rules::{RuleMode, render_rule_form, render_rules_forms};
use ein_render::slice::{render_slice, render_solution, render_state};

/// Compare `got` against `tests/golden/<rel>`, or rewrite it under
/// `EIN_BLESS=1`.
fn check(rel: &str, got: &str) {
    if let Some(e) = golden(&golden_path("ein-render", rel), got) {
        panic!("{e}");
    }
}

/// [`check`] for one of the goldens under `dot/`.
fn check_dot(name: &str, got: &str) {
    check(&format!("dot/{name}.dot"), got);
}

// ── deterministic input builders ───────────────────────────────────

fn parsed(text: &str) -> (Ast, Vec<ein_ir::NodeId>) {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, None).expect("the fixture parses");
    (ast, forms)
}

fn loaded(text: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, None).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// The KB behind three goldens: a binary fact, three `is-a` edges and a
/// ternary hyperedge — every node and edge path in the unified renderer.
const SMALL_KB: &str = concat!(
    "(is-a a Thing) (is-a b Thing) (is-a c Thing)\n",
    "(relation r Thing Thing)\n(relation tern Thing Thing Thing)\n",
    "(r a b :source \"(1)\")\n(tern a b c :source \"(2)\")\n",
);

const TRACE: &str = concat!(
    "(trace (step s1 :rule from-condition :using (c10) ",
    ":derives (lives-in Norwegian House-1))",
    " (step s2 :rule adjacent :using (and (lives-in Norwegian House-1)) ",
    ":derives (color-loc Blue House-2)))",
);

const CONSTRAINTS: &str = concat!(
    "(relation co-located Thing House)\n(symmetric co-located)\n",
    "(relation next-to House House)\n(transitive next-to)\n",
);

const RULE: &str = "(rule t () :match (r ?a ?b) :assert (r ?b ?a) :why \"t\")";
const SYMM: &str = concat!(
    "(rule symmetric (?rel) :match (?rel ?a ?b) ",
    ":assert (?rel ?b ?a) :why \"sym\")",
);

// ── the cases ──────────────────────────────────────────────────────

#[test]
fn the_per_form_ir_goldens_reproduce() {
    for (name, text) in [
        (
            "ir_to_dot_fact",
            "(co-located Norwegian House-1 :source \"(10)\")",
        ),
        (
            "ir_to_dot_neg",
            "(not (co-located Spaniard Coffee) :source \"(1)\")",
        ),
        (
            "ir_to_dot_reasoning",
            "(co-located Blue House-2 :rule square-fwd :using (c10))",
        ),
    ] {
        let (ast, forms) = parsed(text);
        check_dot(name, &to_dot(&ast, &forms, DotOpts::default()));
    }
}

#[test]
fn the_query_and_trace_goldens_reproduce() {
    let (ast, forms) = parsed("(query :goal (drinks Water ?h))");
    check_dot("ir_render_query", &render_query(&ast, forms[0]));

    let (ast, forms) = parsed(TRACE);
    check_dot(
        "ir_render_trace_a",
        &render_trace(&ast, forms[0], TraceView::PerStep),
    );
    check_dot(
        "ir_render_trace_dag",
        &render_trace(&ast, forms[0], TraceView::Dag),
    );
}

#[test]
fn the_rule_and_constraint_goldens_reproduce() {
    let (ast, forms) = parsed(RULE);
    check_dot(
        "render_rule",
        &render_rule_form(&ast, forms[0], RuleMode::SideBySide),
    );

    let (ast, forms) = parsed(&format!("{RULE}\n{SYMM}"));
    check_dot(
        "render_rules",
        &render_rules_forms(&ast, &forms, RuleMode::SideBySide),
    );

    let (ast, forms) = parsed(CONSTRAINTS);
    check_dot(
        "render_constraints",
        &ein_render::render_constraints(&ast, &forms, "constraints"),
    );
}

#[test]
fn the_kb_goldens_reproduce() {
    let (_ast, terms, kb) = loaded(SMALL_KB);
    check_dot(
        "kb_render_to_dot",
        &ein_render::kb_to_dot(&kb, &terms, &KbDotOpts::default()),
    );
    check_dot(
        "slice_render_state",
        &render_state(&kb, &terms, None, "snap"),
    );
    check_dot(
        "slice_render_solution",
        &render_solution(&kb, &terms, "solution"),
    );
}

/// `examples/zebra.ein` through the unified renderer — `kb_zebra_unified.dot`,
/// the one golden that lives beside `dot/` rather than inside it, and the only
/// one built from a real puzzle rather than a hand-written fixture.
#[test]
fn the_zebra_unified_golden_reproduces() {
    let path = repo_root().join("examples/zebra.ein");
    let text = std::fs::read_to_string(&path).expect("the puzzle is checked in");
    // `base_dir = None`: `std.*` imports resolve regardless, and nothing the
    // renderer prints carries a file name.
    let (_ast, terms, kb) = loaded(&text);
    check(
        "kb_zebra_unified.dot",
        &ein_render::kb_to_dot(&kb, &terms, &KbDotOpts::default()),
    );
}

// ── the two synthetic engine fixtures ──────────────────────────────

fn sym(terms: &mut Terms, s: &str) -> Symbol {
    terms.intern_text(s).expect("room")
}

fn fact(terms: &mut Terms, rel: &str, args: &[Value]) -> FactId {
    let rel = sym(terms, rel);
    terms.intern_fact(rel, args).expect("room")
}

fn atom(terms: &mut Terms, s: &str) -> Value {
    terms.value_text(s).expect("room")
}

/// The slice cone: a hypothesis `co-located(Blue, H3)` plus three firings —
/// one plain, one deriving a `not`-wrapped fact, one consuming two of them.
#[test]
fn the_slice_golden_reproduces() {
    let mut terms = Terms::new();
    let (blue, h3) = (atom(&mut terms, "Blue"), atom(&mut terms, "H3"));
    let seed = fact(&mut terms, "co-located", &[blue, h3]);

    let negated = |terms: &mut Terms, colour: &str| {
        let (c, h) = (atom(terms, colour), atom(terms, "H3"));
        let rel = sym(terms, "co-located");
        let inner = terms.value_fact(rel, &[c, h]).expect("room");
        fact(terms, "not", &[inner])
    };
    let neg_red = negated(&mut terms, "Red");
    let neg_green = negated(&mut terms, "Green");

    let mirror = {
        let (h3, blue) = (atom(&mut terms, "H3"), atom(&mut terms, "Blue"));
        fact(&mut terms, "co-located", &[h3, blue])
    };
    let yellow_h1 = {
        let (y, h1) = (atom(&mut terms, "Yellow"), atom(&mut terms, "H1"));
        fact(&mut terms, "co-located", &[y, h1])
    };

    let firing = |terms: &mut Terms, rule: &str, premises: &[FactId], derived: FactId| Firing {
        rule: sym(terms, rule),
        activator: Box::new([]),
        bindings: Box::new([]),
        derived: Box::new([derived]),
        premises: premises.to_vec().into_boxed_slice(),
        redundant: false,
    };
    let firings = vec![
        firing(&mut terms, "symmetric", &[seed], mirror),
        firing(&mut terms, "type-exclusivity", &[seed], neg_red),
        firing(
            &mut terms,
            "domain-elimination",
            &[neg_red, neg_green],
            yellow_h1,
        ),
    ];
    check_dot(
        "slice_render_slice",
        &render_slice(&terms, &[seed], &firings, None, "slice", None, None),
    );
}

/// A three-cell lattice: one solution and two dead commitments, one of which
/// carries a two-fact unsat core. No cell's representative is empty, which is
/// what makes the root node the un-styled `root`, not `∅ root`.
#[test]
fn the_lattice_golden_reproduces() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, " ", None).expect("empty parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("empty loads");

    let p = |terms: &mut Terms, rel: &str, arg: &str| {
        let a = atom(terms, arg);
        fact(terms, rel, &[a])
    };
    let (pa, pb, pc) = (
        p(&mut terms, "p", "a"),
        p(&mut terms, "p", "b"),
        p(&mut terms, "p", "c"),
    );
    let qb = p(&mut terms, "q", "b");

    let dead = |commitment: FactId, core: Vec<FactId>, kind: Kind| DeadCommitment {
        commitment: vec![commitment],
        unsat_core: core,
        learned_clause: vec![commitment],
        layer: 1,
        kind,
        state_key: Box::new([]),
    };
    let proof = LatticeProof {
        solutions: vec![SolutionRecord {
            commitment: vec![pa],
            kb: kb.snapshot(),
            firings: Vec::new(),
            layer: 1,
            // The lattice view does not read a node's tally.
            owes: Default::default(),
        }],
        // This fixture renders the *lattice*, which does not read root's own
        // saturation.
        root_firings: Vec::new(),
        root_initial: Vec::new(),
        dead_commitments: vec![
            dead(pb, vec![pb, qb], Kind::DeadPost),
            dead(pc, vec![pc], Kind::DeadPre),
        ],
        alive_at_end: Vec::new(),
        learned_nogoods: Vec::new(),
        stats: LatticeStats::default(),
    };
    check_dot(
        "lattice_render",
        &render_lattice(
            &terms,
            LatticeSource::Proof(&proof),
            LatticeView::Solution,
            "lattice",
        ),
    );
}
