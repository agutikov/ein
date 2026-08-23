//! Import resolution, minimisation and macro expansion — S1a.1.3's
//! acceptance, without the oracle.
//!
//! The minimisation half matters more than it looks: the surviving declaration
//! set is observable through the plan cache and through firing order, so
//! dropping one declaration too many or too few is a T1/T2 failure several
//! phases later, on a fixture that will look unrelated.
//!
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
//! removes the second resolver:
//!
//! | was | now |
//! |---|---|
//! | three corpus-wide diffs (`resolve` / `minimize` / `expand`) | `corpus_shapes.md5`'s 107 `ir[resolve]` + 107 `ir[minimize]` + 107 `ir[expand]` lines — the same three ops through the same `dump_canonical` |
//! | `macro-names` against the oracle | [`stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list`], which re-parses `stdlib/macro.ein` at test time |
//! | the 14-module mangling diff | a checked-in table |
//! | the two-qualification cache probe | kept, minus its oracle half |
//! | the 12 `.expected` fixtures | unchanged — they never used the oracle |

use std::path::{Path, PathBuf};

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::dump::dump_canonical;
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, parse};

fn parse_file(ast: &mut Ast, path: &Path) -> Vec<ein_ir::NodeId> {
    let text = std::fs::read_to_string(path).expect("readable");
    parse(ast, &text, Some(path.to_str().expect("utf-8"))).expect("the corpus parses")
}

/// **`stdlib_macro_names` reads the module rather than a hard-coded list.**
///
/// The claim the function's name makes, checkable without a second engine: the
/// answer is exactly the `(macro …)` heads declared in `stdlib/macro.ein`,
/// re-derived here by parsing the file at test time. A list that drifted from
/// the module — the failure this exists for — fails on both sides of the
/// comparison at once.
#[test]
fn stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list() {
    let got = Resolver::new().stdlib_macro_names();
    let module = std::env::var("EIN_STDLIB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("stdlib"))
        .join("macro.ein");
    let text =
        std::fs::read_to_string(&module).unwrap_or_else(|e| panic!("{}: {e}", module.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, module.to_str()).expect("std.macro parses");
    let mut declared: Vec<String> = Vec::new();
    for &f in &forms {
        let ein_ir::Node::SForm { head, args } = ast.node(f) else {
            continue;
        };
        if ast.atom_name(head) != Some("macro") {
            continue;
        }
        let Some(&name) = ast.args(args).first() else {
            continue;
        };
        if let Some(name) = ast.atom_name(name) {
            declared.push(name.to_string());
        }
    }
    declared.sort();
    let mut got_sorted = got.clone();
    got_sorted.sort();
    assert_eq!(
        got_sorted, declared,
        "stdlib_macro_names does not answer what std/macro.ein declares"
    );
    assert!(got.contains(&"forall".to_string()), "{got:?}");
}

/// The checked-in messages, compared against the **fixture files** — the same
/// text `ein.py/tests/kb/test_load_negative.py` and the corpus sweep
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

/// One module, imported **twice under different qualifications** in a single
/// resolution — the shape T1a.6.5.3's module cache makes sharing-sensitive.
///
/// Since that task a module is parsed once per resolution and *both* importers
/// are handed the same nodes, so a qualification that rewrote a shared subtree
/// in place instead of building a new one would leak `m.` into the flat import
/// (or the other way round). `rename_atoms` builds; this is the test that says
/// so, and it compares against `ein.py`, which has no cache at all.
#[test]
fn one_module_under_two_qualifications_does_not_leak_either_way() {
    let base = repo_root().join("examples");
    let src = "(import std.macro :as m)\n(import std.macro :symbols (forall))\n";
    let mut ast = Ast::new();
    let forms = parse(&mut ast, src, Some("<string>")).expect("parses");
    let got = Resolver::new()
        .resolve_imports(&mut ast, &forms, Some(&base))
        .map(|f| dump_canonical(&ast, &f))
        .expect("resolves");
    // Both qualifications survive, each spelled its own way.
    assert!(
        got.contains("(macro m.forall"),
        "the aliased copy is missing:\n{got}"
    );
    assert!(
        got.contains("(macro forall"),
        "the flat copy is missing:\n{got}"
    );
    assert!(
        !got.contains("(macro m.m."),
        "a rename was applied twice:\n{got}"
    );
    // Both copies expand the same body: the rename touched the *heads* only,
    // so `m.forall` and `forall` differ in exactly one token. This is the half
    // that was ein.py's — a resolver with no cache cannot leak, so the whole
    // point was that the cached one produces the same text.
    let flat = got
        .lines()
        .find(|l| l.starts_with("(macro forall"))
        .expect("the flat copy");
    let aliased = got
        .lines()
        .find(|l| l.starts_with("(macro m.forall"))
        .expect("the aliased copy");
    assert_eq!(
        aliased.replacen("(macro m.forall", "(macro forall", 1),
        flat,
        "the two qualifications differ by more than the head"
    );
}

/// **Module-name → file-path mangling is `pathlib`'s**, and the places it is
/// surprising only show up in the "module not found" text.
///
/// `std.nope.`, `std..nope`, `a.` and `std.__x` are the interesting rows: an
/// empty segment and a trailing dot are not errors in `pathlib`, they are path
/// components, so the message names a file nobody would have guessed. Checked
/// in as a table because the message *is* the observable — resolution either
/// finds a module or says which file it looked for.
///
/// The path in each message is machine-specific, so the checkout root is
/// replaced by `{ROOT}` the way `examples/broken/load/`'s fixtures use
/// `{FILE}`.
#[test]
fn module_paths_are_mangled_the_way_pathlib_mangles_them() {
    const MODULES: [&str; 14] = [
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
    ];
    let root = repo_root();
    let base = root.join("examples");
    let mut out = String::new();
    let mut resolvable = 0;
    for module in MODULES {
        let src = format!("(import {module})");
        let mut ast = Ast::new();
        out.push_str(&format!("=== {src}\n"));
        let Ok(forms) = parse(&mut ast, &src, Some("<string>")) else {
            out.push_str("  <not a legal module name>\n");
            continue;
        };
        resolvable += 1;
        let answer = match Resolver::new().resolve_imports(&mut ast, &forms, Some(&base)) {
            Ok(f) => dump_canonical(&ast, &f),
            Err(e) => e.0,
        };
        for line in answer
            .replace(root.to_str().expect("utf-8"), "{ROOT}")
            .lines()
        {
            out.push_str(&format!("  {line}\n"));
        }
    }
    assert!(resolvable >= 6, "only {resolvable} module names parsed");
    if let Some(msg) = golden(&golden_path("ein-ir", "module_paths.txt"), &out) {
        panic!("{msg}");
    }
}
