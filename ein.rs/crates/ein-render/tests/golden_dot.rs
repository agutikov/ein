//! S1a.5.1 acceptance — the checked-in DOT goldens, byte for byte.
//!
//! `ein.py/tests/render/test_golden_dot.py` locked the current bytes of every
//! DOT emitter onto fifteen files under `ein.py/tests/golden/dot/`, plus
//! `kb_zebra_unified.dot` beside them. Those *committed* files are the
//! fixture here: a port that shipped its own copy of the expected bytes would
//! prove only that it agrees with itself.
//!
//! Each case rebuilds ein.py's deterministic input against the port's data
//! model and renders it. Two are already covered elsewhere and are not
//! repeated: `kb_provenance_dag` by `derivation_dot.rs`, which landed with
//! the provenance walk it renders.

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::commitment::Kind;
use ein_infer::firing::Firing;
use ein_infer::solve::{DeadCommitment, LatticeProof, LatticeStats, SolutionRecord};
use ein_ir::{Ast, parse};
use ein_oracle::repo_root;
use ein_render::ir_dot::{DotOpts, TraceView, render_query, render_trace, to_dot};
use ein_render::kb_dot::KbDotOpts;
use ein_render::lattice_dag::{LatticeSource, LatticeView, render_lattice};
use ein_render::rules::{RuleMode, render_rule_form, render_rules_forms};
use ein_render::slice::{render_slice, render_solution, render_state};
use std::path::PathBuf;

fn golden(name: &str) -> String {
    let path: PathBuf = repo_root()
        .join("ein.py/tests/golden/dot")
        .join(format!("{name}.dot"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
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
        assert_eq!(
            to_dot(&ast, &forms, DotOpts::default()),
            golden(name),
            "{name}"
        );
    }
}

#[test]
fn the_query_and_trace_goldens_reproduce() {
    let (ast, forms) = parsed("(query :goal (drinks Water ?h))");
    assert_eq!(render_query(&ast, forms[0]), golden("ir_render_query"));

    let (ast, forms) = parsed(TRACE);
    assert_eq!(
        render_trace(&ast, forms[0], TraceView::PerStep),
        golden("ir_render_trace_a")
    );
    assert_eq!(
        render_trace(&ast, forms[0], TraceView::Dag),
        golden("ir_render_trace_dag")
    );
}

#[test]
fn the_rule_and_constraint_goldens_reproduce() {
    let (ast, forms) = parsed(RULE);
    assert_eq!(
        render_rule_form(&ast, forms[0], RuleMode::SideBySide),
        golden("render_rule")
    );

    let (ast, forms) = parsed(&format!("{RULE}\n{SYMM}"));
    assert_eq!(
        render_rules_forms(&ast, &forms, RuleMode::SideBySide),
        golden("render_rules")
    );

    let (ast, forms) = parsed(CONSTRAINTS);
    assert_eq!(
        ein_render::render_constraints(&ast, &forms, "constraints"),
        golden("render_constraints")
    );
}

#[test]
fn the_kb_goldens_reproduce() {
    let (_ast, terms, kb) = loaded(SMALL_KB);
    assert_eq!(
        ein_render::kb_to_dot(&kb, &terms, &KbDotOpts::default()),
        golden("kb_render_to_dot")
    );
    assert_eq!(
        render_state(&kb, &terms, None, "snap"),
        golden("slice_render_state")
    );
    assert_eq!(
        render_solution(&kb, &terms, "solution"),
        golden("slice_render_solution")
    );
}

/// `examples/zebra.ein` through the unified renderer — `kb_zebra_unified.dot`,
/// the one golden that lives beside `dot/` rather than inside it, and the only
/// one built from a real puzzle rather than a hand-written fixture.
#[test]
fn the_zebra_unified_golden_reproduces() {
    let path = repo_root().join("examples/zebra.ein");
    let text = std::fs::read_to_string(&path).expect("the puzzle is checked in");
    // `base_dir = None`, as ein.py's fixture does: `std.*` imports resolve
    // regardless, and nothing the renderer prints carries a file name.
    let (_ast, terms, kb) = loaded(&text);
    let want =
        std::fs::read_to_string(repo_root().join("ein.py/tests/golden/kb_zebra_unified.dot"))
            .expect("the golden is checked in");
    assert_eq!(
        ein_render::kb_to_dot(&kb, &terms, &KbDotOpts::default()),
        want
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
    assert_eq!(
        render_slice(&terms, &[seed], &firings, None, "slice", None, None),
        golden("slice_render_slice")
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
        }],
        // This fixture renders the *lattice*, which does not read root's own
        // saturation.
        root_firings: Vec::new(),
        dead_commitments: vec![
            dead(pb, vec![pb, qb], Kind::DeadPost),
            dead(pc, vec![pc], Kind::DeadPre),
        ],
        alive_at_end: Vec::new(),
        learned_nogoods: Vec::new(),
        stats: LatticeStats::default(),
    };
    assert_eq!(
        render_lattice(
            &terms,
            LatticeSource::Proof(&proof),
            LatticeView::Solution,
            "lattice",
        ),
        golden("lattice_render")
    );
}
