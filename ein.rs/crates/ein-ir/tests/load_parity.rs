//! S1a.2.3 acceptance — every corpus file loads to the same KB, and every way
//! of failing to load says the same thing.
//!
//! The KB has no CLI surface, so `ein-conformance` cannot see any of this:
//! `ein_core::shape` renders the registries, the fact list and the seven
//! indexes as one deterministic text, `utils/ir_oracle.py`'s `kb-shape` op
//! renders the same text from ein.py, and the two are diffed. A load *failure*
//! is compared the same way, which is what extends the accumulated-message
//! check from the eighteen fixtures to the whole corpus.

use ein_core::{Terms, shape};
use ein_ir::{Ast, load_file};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use std::path::{Path, PathBuf};

/// What ein.rs answers, in the oracle's vocabulary.
fn rust_shape(path: &Path) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    match load_file(&mut ast, &mut terms, path) {
        Ok(kb) => {
            kb.check_layering(&terms)
                .expect("layering holds after load");
            Answer::Ok(shape(&kb, &terms))
        }
        Err(e) => Answer::Err {
            kind: "KBLoadError".into(),
            msg: e.0,
        },
    }
}

#[test]
fn the_whole_corpus_loads_to_the_same_kb() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_whole_corpus_loads_to_the_same_kb");
    };
    let files = corpus_files();
    assert!(files.len() >= 90, "only {} corpus files found", files.len());
    let (mut bad, mut loaded, mut rejected) = (Vec::new(), 0, 0);
    for path in &files {
        let got = rust_shape(path);
        let want = py.file("kb-shape", path);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                loaded += 1;
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => {
                rejected += 1;
                if a != b {
                    bad.push(format!("{name}\n  ein.py: {b}\n  ein.rs: {a}"));
                }
            }
            (Answer::Ok(_), Answer::Err { msg, .. }) => {
                bad.push(format!("{name}\n  ein.py rejected: {msg}\n  ein.rs loaded"))
            }
            (Answer::Err { msg, .. }, Answer::Ok(_)) => {
                bad.push(format!("{name}\n  ein.py loaded\n  ein.rs rejected: {msg}"))
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} files differ:\n\n{}",
        bad.len(),
        files.len(),
        bad.join("\n\n")
    );
    assert!(loaded >= 60, "only {loaded} files actually loaded");
    assert!(rejected >= 20, "only {rejected} files were rejected");
}

/// The first differing line, with a little context — a whole-shape diff of
/// `zebra2` is unreadable.
fn first_difference(got: &str, want: &str) -> String {
    let (g, w): (Vec<&str>, Vec<&str>) = (got.lines().collect(), want.lines().collect());
    for i in 0..g.len().max(w.len()) {
        let (a, b) = (g.get(i).copied(), w.get(i).copied());
        if a != b {
            let from = i.saturating_sub(2);
            let context: Vec<String> = (from..i).map(|j| format!("    {}", w[j])).collect();
            return format!(
                "  line {}:\n{}\n  ein.py: {}\n  ein.rs: {}",
                i + 1,
                context.join("\n"),
                b.unwrap_or("<end>"),
                a.unwrap_or("<end>")
            );
        }
    }
    "  identical?".to_string()
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
        "fact_layer_kwarg",
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
/// what is compared is what a puzzle author would see.
#[test]
fn config_coercion_agrees_on_every_path() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("config_coercion_agrees_on_every_path");
    };
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
    let mut bad = Vec::new();
    for body in bodies {
        let got = rust_shape_text(body);
        let want = py.text("kb-shape", body, None);
        let same = match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => a == b,
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => a == b,
            _ => false,
        };
        if !same {
            bad.push(format!("{body}\n  ein.py: {want:?}\n  ein.rs: {got:?}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
}

fn rust_shape_text(text: &str) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = match ein_ir::parse(&mut ast, text, None) {
        Ok(forms) => forms,
        Err(e) => {
            return Answer::Err {
                kind: "IRParseError".into(),
                msg: e.to_string(),
            };
        }
    };
    match ein_ir::load(&mut ast, &mut terms, &forms, None) {
        Ok(kb) => Answer::Ok(shape(&kb, &terms)),
        Err(e) => Answer::Err {
            kind: "KBLoadError".into(),
            msg: e.0,
        },
    }
}

/// Errors **accumulate**, and the order is the pass order — macros, then
/// relations, then rules, then facts, then config, then the unimported-macro
/// guard, then the derivation-cycle check. A reordered `; `-joined message is
/// the failure mode this pins.
#[test]
fn load_errors_accumulate_in_pass_order() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("load_errors_accumulate_in_pass_order");
    };
    let programs = [
        // One error per pass, declared in an order that is not the pass order.
        "(x a :layer fact)\n(rule r () :match (x ?a))\n(relation)\n\
         (macro m (?a) (rel ?a))\n(macro m (?a) (other ?a))\n(config :nope true)",
        // Several within one pass keep their form order.
        "(relation)\n(relation eq)\n(relation dup)\n(relation dup)\n(relation r (A B))",
        // …and within the fact pass.
        "(relation r T T)\n(r a :layer fact)\n(r b :layer reasoning)",
        // A rule error does not stop the following rules from loading.
        "(rule absent () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (z ?a))",
        // A macro rejected for its name is *not* registered, so the rule that
        // invokes it keeps the unexpanded form rather than expanding it.
        "(macro absent (?p) (rel ?p))\n(rule r () :match (absent ?a) :assert (y ?a))",
    ];
    let mut bad = Vec::new();
    for program in programs {
        let got = rust_shape_text(program);
        let want = py.text("kb-shape", program, None);
        let same = match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => a == b,
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => a == b,
            _ => false,
        };
        if !same {
            bad.push(format!("{program}\n  ein.py: {want:?}\n  ein.rs: {got:?}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
}
