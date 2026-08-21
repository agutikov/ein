//! The loader's contract — S1a.2.3's acceptance, without the oracle.
//!
//! The KB has no CLI surface, so `ein-conformance` never saw any of this:
//! `ein_core::shape` renders the registries, the fact list and the seven
//! indexes as one deterministic text, and until
//! [S1a.10.2](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
//! this file diffed that text against `utils/ir_oracle.py`'s `kb-shape` op on
//! every corpus file. What replaced each half:
//!
//! | was | now |
//! |---|---|
//! | the corpus-wide `kb-shape` diff | `ein-render`'s `corpus_shapes.md5`, 107 `::load` lines, carrying this test's `loaded >= 60` floor |
//! | `kb.check_layering` riding inside the diff's helper | [`layering_holds_after_every_load`], its own sweep |
//! | the 32-body `(config …)` coercion diff | [`the_config_coercion_table_is_stable`], against a checked-in table |
//! | the accumulated-message diff | [`load_errors_accumulate_in_pass_order`], the same way |
//! | the 18 `.expected` fixtures | unchanged — it never used the oracle |
//!
//! The two tables were **blessed from a tree where the differential half was
//! green**: `the_whole_corpus_loads_to_the_same_kb` and
//! `config_coercion_agrees_on_every_path` both passed against a live ein.py in
//! the commit that wrote them, so what is checked in is text ein.py signed off
//! on. That is the same provenance argument `corpus_shapes.md5` makes, and it
//! is the only one available once the second engine is gone.

use ein_core::{Terms, shape};
use ein_corpus::{corpus_files, golden, golden_path, repo_root};
use ein_ir::{Ast, load_file};
use std::path::{Path, PathBuf};

/// What one load answered, in the vocabulary the oracle used — kept because
/// the tables below are *rendered* in it, and a renamed variant would
/// invalidate a checked-in golden for no reason.
enum Answer {
    Ok(String),
    Err { kind: &'static str, msg: String },
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Ok(text) => write!(f, "{text}"),
            Answer::Err { kind, msg } => write!(f, "<{kind}> {msg}"),
        }
    }
}

/// One file, loaded.
fn rust_shape(path: &Path) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    match load_file(&mut ast, &mut terms, path) {
        Ok(kb) => Answer::Ok(shape(&kb, &terms)),
        Err(e) => Answer::Err {
            kind: "KBLoadError",
            msg: e.0,
        },
    }
}

/// **Layering holds after every load.**
///
/// `check_layering` was an `expect` inside the differential helper, which made
/// it a *precondition of the comparison* rather than a claim — a file whose
/// layering broke would have panicked out of a test named for something else.
/// Here it is the claim: every fact's layer is consistent with its provenance
/// on every corpus file that loads, before any rule has run.
#[test]
fn layering_holds_after_every_load() {
    let mut loaded = 0;
    for path in &corpus_files() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        kb.check_layering(&terms)
            .unwrap_or_else(|e| panic!("{name}: layering broken after load: {e:?}"));
        loaded += 1;
    }
    assert!(loaded >= 60, "only {loaded} corpus files loaded");
}

/// The loader's own eighteen fixtures, against the committed `.expected`
/// files — the same text `ein.py/tests/kb/test_load_negative.py` holds ein.py
/// to. The eleven import ones landed with S1a.1.3.
#[test]
fn the_load_negative_fixtures_are_byte_identical() {
    let root = repo_root();
    let dir = root.join("examples/broken/load");
    let stdlib = std::env::var("EIN_STDLIB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("stdlib"));

    let mut bad = Vec::new();
    for name in [
        "config_bad_value",
        "config_unknown_flag",
        "derivation_cycle",
        "hrule_duplicate_name",
        "hrule_reserved_name",
        "macro_arity_mismatch",
        "macro_duplicate",
        "macro_reserved_name",
        "relation_duplicate",
        "relation_malformed",
        "relation_needs_a_name",
        "relation_reserved_name",
        "rule_duplicate_name",
        "rule_half_declarators",
        "rule_missing_assert",
        "rule_missing_match",
        "rule_reserved_name",
        "unimported_std_macro",
    ] {
        let path = dir.join(format!("{name}.ein"));
        let expected = std::fs::read_to_string(dir.join(format!("{name}.expected")))
            .expect("an .expected beside every fixture")
            .trim_end()
            .replace("{FILE}", path.to_str().expect("utf-8"))
            .replace("{DIR}", dir.to_str().expect("utf-8"))
            .replace("{STDLIB}", stdlib.to_str().expect("utf-8"));
        match rust_shape(&path) {
            Answer::Err { msg, .. } if msg == expected => {}
            Answer::Err { msg, .. } => {
                bad.push(format!("{name}\n  want: {expected}\n  got:  {msg}"))
            }
            Answer::Ok(_) => bad.push(format!("{name}: loaded, but must fail")),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// `SolverConfig.from_kw_pairs` — every coercion path and every message,
/// driven through a `(config …)` body rather than through the Rust API, so
/// what is checked is what a puzzle author would see.
///
/// A table rather than a diff: the thirty-two bodies below were chosen to hit
/// every branch of the coercion — the four field kinds, CPython's `int()` /
/// `float()` tolerances (`"1_000"`, `" 7 "`, and the `"1_"` / `"1.5"` it
/// refuses for an int), a malformed body, an unknown flag, and the
/// last-one-wins rule — and each one's *answer* is the assertion. Rendered as
/// one file so a change reads as a diff of the affected bodies rather than as
/// thirty-two separate failures.
#[test]
fn the_config_coercion_table_is_stable() {
    let bodies = [
        // Booleans: the atoms, the strings, and the wrong shapes.
        "(config :print-alive true)",
        "(config :print-alive false)",
        "(config :print-alive \"true\")",
        "(config :print-alive TRUE)",
        "(config :print-alive 1)",
        "(config :print-alive maybe)",
        "(config :print-alive ?x)",
        "(config :print-alive 1..5)",
        "(config :print-alive (nested a))",
        // Strings: an atom and a string coerce, an int does not.
        "(config :hypgen-scoring most-constrained)",
        "(config :hypgen-scoring \"popularity\")",
        "(config :hypgen-scoring 7)",
        "(config :lattice-order score-sum)",
        // Ints: literals, strings, and CPython's tolerances.
        "(config :candidate-order-seed 7)",
        "(config :candidate-order-seed -1)",
        "(config :candidate-order-seed 007)",
        "(config :candidate-order-seed \"7\")",
        "(config :candidate-order-seed \" 7 \")",
        "(config :candidate-order-seed \"1_000\")",
        "(config :candidate-order-seed \"1_\")",
        "(config :candidate-order-seed \"1.5\")",
        "(config :candidate-order-seed true)",
        "(config :lattice-order-seed 3)",
        // Floats.
        "(config :hypgen-rel-weight 2)",
        "(config :hypgen-rel-weight \"1.5\")",
        "(config :hypgen-rel-weight \"1_0.5\")",
        "(config :hypgen-obj-weight \"nope\")",
        // The shape of the body itself, and the unknown-flag enumeration.
        "(config foo)",
        "(config :nope true)",
        "(config)",
        // Last one wins, and an earlier repeat of a key loses to a later.
        "(config :print-alive true) (config :warn-derived-naf true)",
        "(config :print-alive true :print-alive false)",
    ];
    let mut out = String::new();
    for body in bodies {
        out.push_str(&format!("=== {body}\n{}", indented(&rust_shape_text(body))));
    }
    assert_eq!(bodies.len(), 32, "the table lost a coercion path");
    if let Some(msg) = golden(&golden_path("ein-ir", "config_coercion.txt"), &out) {
        panic!("{msg}");
    }
}

/// One answer, every line indented — so a `=== ` header always starts a row
/// and a multi-line diagnostic (a parse error carries its caret) cannot be
/// mistaken for the next body.
fn indented(answer: &Answer) -> String {
    answer
        .to_string()
        .lines()
        .map(|l| format!("  {l}\n"))
        .collect()
}

fn rust_shape_text(text: &str) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = match ein_ir::parse(&mut ast, text, None) {
        Ok(forms) => forms,
        Err(e) => {
            return Answer::Err {
                kind: "IRParseError",
                msg: e.to_string(),
            };
        }
    };
    match ein_ir::load(&mut ast, &mut terms, &forms, None) {
        Ok(kb) => Answer::Ok(shape(&kb, &terms)),
        Err(e) => Answer::Err {
            kind: "KBLoadError",
            msg: e.0,
        },
    }
}

/// Errors **accumulate**, and the order is the pass order — macros, then
/// relations, then rules, then facts, then config, then the unimported-macro
/// guard, then the derivation-cycle check. A reordered `; `-joined message is
/// the failure mode this pins, and it is invisible in any single fixture:
/// each of the programs below declares its errors in an order that is *not*
/// the pass order, so a loader that reported them in encounter order would
/// produce plausible messages and fail here.
///
/// **The fact pass is absent from these programs because it cannot fail on
/// anything the grammar accepts.** Its one error — a non-atom head — is
/// reachable only by hand-building an AST, since `(?R a b)` is an
/// `IRParseError`. Facts either parse or load.
#[test]
fn load_errors_accumulate_in_pass_order() {
    let programs = [
        // One error per pass, declared in an order that is not the pass order.
        "(rule r () :match (x ?a))\n(relation)\n\
         (macro m (?a) (rel ?a))\n(macro m (?a) (other ?a))\n(config :nope true)",
        // Several within one pass keep their form order.
        "(relation)\n(relation eq)\n(relation dup)\n(relation dup)\n(relation r (A B))",
        // A rule error does not stop the following rules from loading.
        "(rule absent () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (z ?a))",
        // A macro rejected for its name is *not* registered, so the rule that
        // invokes it keeps the unexpanded form rather than expanding it.
        "(macro absent (?p) (rel ?p))\n(rule r () :match (absent ?a) :assert (y ?a))",
    ];
    let mut out = String::new();
    for program in programs {
        out.push_str(&format!(
            "=== {}\n{}",
            program.replace('\n', " ⏎ "),
            indented(&rust_shape_text(program))
        ));
    }
    assert_eq!(programs.len(), 4, "the table lost a program");
    if let Some(msg) = golden(&golden_path("ein-ir", "load_error_order.txt"), &out) {
        panic!("{msg}");
    }
}
