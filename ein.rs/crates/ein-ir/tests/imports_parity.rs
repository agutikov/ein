//! S1a.1.3 acceptance — import resolution, minimisation and macro expansion
//! produce the same forms and the same failures as `ein.py`.
//!
//! The minimisation half matters more than it looks: the surviving
//! declaration set is observable through `len(engine.cache)` and through
//! firing order, so dropping one declaration too many or too few is a T1/T2
//! failure several phases later, on a fixture that will look unrelated.

use std::path::{Path, PathBuf};

use ein_ir::dump::dump_canonical;
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, parse};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};

fn parse_file(ast: &mut Ast, path: &Path) -> Vec<ein_ir::NodeId> {
    let text = std::fs::read_to_string(path).expect("readable");
    parse(ast, &text, Some(path.to_str().expect("utf-8"))).expect("the corpus parses")
}

/// The corpus entries that reach the resolver: everything that parses.
fn resolvable() -> Vec<PathBuf> {
    corpus_files()
        .into_iter()
        .filter(|p| !p.parent().is_some_and(|d| d.ends_with("broken")))
        .collect()
}

/// Run one of the three resolution ops on both sides and compare.
fn compare_op(op: &str) {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip(op);
    };
    let mut bad = Vec::new();
    for path in resolvable() {
        let base = path.parent().map(Path::to_path_buf);
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, &path);
        let resolver = Resolver::new();
        // `Err` carries the message, whichever layer raised it — a load
        // error from the resolver or a `MacroError` from the expander. The
        // oracle reports both the same way, and so must this.
        let got: Result<String, String> = match op {
            "resolve" => resolver
                .resolve_imports(&mut ast, &forms, base.as_deref())
                .map(|f| dump_canonical(&ast, &f))
                .map_err(|e| e.0),
            "minimize" => resolver
                .resolve_and_minimize(&mut ast, &forms, base.as_deref())
                .map(|f| dump_canonical(&ast, &f))
                .map_err(|e| e.0),
            "expand" => resolver
                .resolve_imports(&mut ast, &forms, base.as_deref())
                .map_err(|e| e.0)
                .and_then(|f| {
                    let macros = collect_macros(&ast, &f);
                    expand_rule_clauses(&mut ast, &f, &macros)
                        .map(|expanded| dump_canonical(&ast, &expanded))
                        .map_err(|e| e.0)
                }),
            _ => unreachable!("op"),
        };
        let want = py.file(op, &path);
        let same = match (&got, &want) {
            (Ok(a), Answer::Ok(b)) => a == b,
            (Err(e), Answer::Err { msg, .. }) => e == msg,
            _ => false,
        };
        if !same {
            let detail = match (&got, &want) {
                (Ok(a), Answer::Ok(b)) => first_difference(a, b),
                (Err(e), _) => format!("ein.rs Err({e:?}), ein.py {want:?}"),
                (Ok(_), Answer::Err { msg, .. }) => format!("ein.rs accepted; ein.py {msg:?}"),
            };
            bad.push(format!("{}\n  {detail}", path.display()));
        }
    }
    assert!(
        bad.is_empty(),
        "{} file(s) differ under `{op}`:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

#[test]
fn the_corpus_resolves_identically() {
    compare_op("resolve");
}

#[test]
fn the_corpus_minimizes_identically() {
    compare_op("minimize");
}

#[test]
fn the_corpus_expands_identically() {
    compare_op("expand");
}

#[test]
fn stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list");
    };
    let got = Resolver::new().stdlib_macro_names();
    let want = py.ask(serde_json::json!({"op": "macro-names"}));
    assert_eq!(got.join("\n"), want.unwrap());
    assert!(got.contains(&"forall".to_string()), "{got:?}");
}

/// The checked-in messages, compared against the **fixture files** — the same
/// text `ein.py/tests/kb/test_load_negative.py` and the conformance runner
/// hold ein.py to.
#[test]
fn the_import_and_macro_failures_are_byte_identical() {
    let root = repo_root();
    let dir = root.join("examples/broken/load");
    let stdlib = std::env::var("EIN_STDLIB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("stdlib"));

    let expand_placeholders = |text: String, path: &Path| {
        text.trim_end()
            .replace("{FILE}", path.to_str().expect("utf-8"))
            .replace("{DIR}", dir.to_str().expect("utf-8"))
            .replace("{STDLIB}", stdlib.to_str().expect("utf-8"))
    };

    // Only the fixtures whose message this phase owns. The rest —
    // relation/rule/config/derivation, the duplicate and reserved macro
    // names, the S1.8a.f20 guard's wiring — are the loader's, and are
    // compared when the loader lands (P1a.2).
    for name in [
        "import_alias_not_a_name",
        "import_as_and_symbols",
        "import_bare_std",
        "import_conflicting_definitions",
        "import_cycle",
        "import_cycle_b",
        "import_module_not_found",
        "import_relative_not_found",
        "import_symbols_empty",
        "import_symbols_not_a_list",
        "import_symbols_not_provided",
    ] {
        let path = dir.join(format!("{name}.ein"));
        let expected = expand_placeholders(
            std::fs::read_to_string(dir.join(format!("{name}.expected")))
                .expect("an .expected beside every fixture"),
            &path,
        );
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, &path);
        let err = Resolver::new()
            .resolve_imports(&mut ast, &forms, Some(&dir))
            .expect_err(&format!("{name} must fail"));
        assert_eq!(err.0, expected, "{name}");
    }

    // The macro fixture's message is a `MacroError`; the `({head} {name}): `
    // prefix around it is the loader's `errors.append(f"({head} {name}): {e}")`
    // and arrives with the loader. Asserting the composition here is what
    // makes the remaining half a one-line wiring job rather than a discovery.
    let path = dir.join("macro_arity_mismatch.ein");
    let expected = expand_placeholders(
        std::fs::read_to_string(dir.join("macro_arity_mismatch.expected")).expect("expected"),
        &path,
    );
    let mut ast = Ast::new();
    let forms = parse_file(&mut ast, &path);
    let resolved = Resolver::new()
        .resolve_imports(&mut ast, &forms, Some(&dir))
        .expect("resolves");
    let macros = collect_macros(&ast, &resolved);
    let err = expand_rule_clauses(&mut ast, &resolved, &macros).expect_err("arity mismatch");
    assert_eq!(format!("(rule x): {}", err.0), expected);
}

/// Module-name → file-path mangling is `pathlib`'s, and the places it is
/// surprising (an empty segment, a trailing dot, a bare `std`) only show up in
/// the "module not found" text. Compare the text.
#[test]
fn module_paths_are_mangled_the_way_pathlib_mangles_them() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("module_paths_are_mangled_the_way_pathlib_mangles_them");
    };
    let base = repo_root().join("examples");
    let mut bad = Vec::new();
    let mut checked = 0;
    for module in [
        "std.nope",
        "std.a.b",
        "std.a.b.c",
        "nope",
        "nope.missing",
        "a.b.c",
        "std",
        "std.a-b",
        "std.a*b",
        "x--y",
        "std.nope.",
        "std..nope",
        "a.",
        "std.__x",
    ] {
        let src = format!("(import {module})");
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &src, Some("<string>")) else {
            continue; // not a legal module name — the parser's business
        };
        let got = Resolver::new()
            .resolve_imports(&mut ast, &forms, Some(&base))
            .map(|f| dump_canonical(&ast, &f));
        // Hand the oracle a file in `examples/` so its base_dir matches.
        let want = py.ask(serde_json::json!({
            "op": "resolve",
            "text": src,
            "filename": base.join("probe.ein").to_str(),
        }));
        // A `text` request carries no base_dir on the Python side, so a
        // file-relative import fails differently there; those are covered by
        // the fixture test above.
        if matches!(&want, Answer::Err { msg, .. } if msg.contains("base directory")) {
            continue;
        }
        checked += 1;
        let same = match (&got, &want) {
            (Err(e), Answer::Err { msg, .. }) => &e.0 == msg,
            (Ok(a), Answer::Ok(b)) => a == b,
            _ => false,
        };
        if !same {
            bad.push(format!(
                "(import {module})\n  ein.py: {want:?}\n  ein.rs: {got:?}"
            ));
        }
    }
    assert!(checked >= 6, "only {checked} module names were comparable");
    assert!(
        bad.is_empty(),
        "{} module name(s) differ:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

fn first_difference(got: &str, want: &str) -> String {
    for (i, (a, b)) in got.lines().zip(want.lines()).enumerate() {
        if a != b {
            return format!("line {}: {a:?} (ein.py has {b:?})", i + 1);
        }
    }
    format!(
        "same {} line(s), then ein.py has {} and ein.rs {}",
        got.lines().count().min(want.lines().count()),
        want.lines().count(),
        got.lines().count()
    )
}
