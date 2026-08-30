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
    let macros = collect_macros(&ast, &resolved).expect("the fixture declares no bad macro");
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

// ── M1e S1e.2.1 — CO-H2: one reserved-name list ────────────────────

/// A scratch module directory one test owns and deletes.
///
/// Tagged per test for `primitive_arity.rs`'s reason: these run as threads of
/// one binary, and an untagged directory means the second `new()` deletes the
/// first test's modules out from under it — which reads as *"module not
/// found"* in a test whose whole subject is what a module may declare.
struct Modules(PathBuf);

impl Modules {
    fn new(tag: &str) -> Modules {
        let dir = std::env::temp_dir().join(format!("ein-reserved-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Modules(dir)
    }

    /// Write `m.ein` and load `src` against it — `Ok(())`, or the message.
    fn load(&self, module: &str, src: &str) -> Result<(), String> {
        std::fs::write(self.0.join("m.ein"), module).expect("the module is written");
        let mut ast = Ast::new();
        let mut terms = ein_core::Terms::new();
        let forms = parse(&mut ast, src, Some("<probe>")).map_err(|e| format!("parse: {e}"))?;
        ein_ir::load(&mut ast, &mut terms, &forms, Some(&self.0))
            .map(|_| ())
            .map_err(|e| e.0)
    }
}

impl Drop for Modules {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// One declaration of `name` under `decl`, as a module body.
fn declaration(decl: &str, name: &str) -> String {
    let body = match decl {
        "rule" | "hrule" => {
            format!("({decl} {name} ()\n  :match (p ?x)\n  :assert (q ?x)\n  :why \"d\")")
        }
        "relation" => format!("(relation {name} T)"),
        "macro" => format!("(macro {name} (?x) (p ?x))"),
        other => unreachable!("{other}"),
    };
    format!("(relation p T)\n(relation q T)\n{body}\n")
}

/// **A declarator may not bind a reserved name, by whichever route it
/// arrives** — M1e S1e.2.1, `CO-H2`.
///
/// Thirty-two cells: four declarators × two names × four import routes. The
/// names are `open`, which is `ein-core`'s ninth and was the drift, and
/// `absent`, which is in every copy of the list there has ever been and is the
/// control — the finding is precisely that the two behaved differently.
///
/// **Eight of the thirty-two used to load with exit 0.** `imports.rs` carried
/// its own `RESERVED_NAMES: [&str; 8]` and `qualify()` filtered against it, so
/// a name it did not know was prefixed — `m.open` — before the loader could
/// object, and `qualify()`'s own doc comment stated the opposite intent. Two
/// of the four routes go through `qualify()`, hence 4 declarators × 2 routes.
/// The comment beside the list even predicted the fix (*"P1a.3 brings the
/// registries over and this becomes a query against them"*); it never
/// happened, and what closed it is one `ein_core::is_reserved` call.
///
/// **The fourth route is this test's own finding.** The review named three —
/// direct, flat `:symbols`, qualified — and `:as` is a fourth, taking the same
/// `qualify()` path with a different prefix. It was equally broken.
#[test]
fn reserved_names_are_reserved_through_every_import_route() {
    let m = Modules::new("routes");
    let mut loaded: Vec<String> = Vec::new();
    for decl in ["rule", "hrule", "relation", "macro"] {
        for name in ["open", "absent"] {
            let module = declaration(decl, name);
            for (route, src) in [
                ("direct", format!("{module}(p A)\n")),
                ("symbols", format!("(import m :symbols ({name}))\n(p A)\n")),
                ("qualified", "(import m)\n(p A)\n".to_string()),
                ("aliased", "(import m :as z)\n(p A)\n".to_string()),
            ] {
                let cell = format!("{decl} {name} via {route}");
                match m.load(&module, &src) {
                    Ok(()) => loaded.push(cell),
                    Err(e) => assert!(
                        e.contains("shadows a reserved kernel name"),
                        "{cell}: refused, but not as a reserved name: {e}"
                    ),
                }
            }
        }
    }
    assert!(
        loaded.is_empty(),
        "{} of 32 cells bound a reserved name:\n  {}",
        loaded.len(),
        loaded.join("\n  ")
    );
}

/// The same four routes over a name that is **not** reserved, so the test
/// above is a guard and not a blanket refusal.
///
/// It also pins what qualification does with the name it is allowed to touch:
/// flat and direct keep it, `(import m)` prefixes it with the module name and
/// `:as z` with the alias. A fix that refused everything, or renamed nothing,
/// passes the previous test and fails this one.
#[test]
fn an_unreserved_name_still_qualifies_through_all_four_routes() {
    let m = Modules::new("control");
    let module = declaration("macro", "opennnn");
    for (route, src) in [
        ("direct", format!("{module}(p A)\n")),
        (
            "symbols",
            "(import m :symbols (opennnn))\n(p A)\n".to_string(),
        ),
        ("qualified", "(import m)\n(p A)\n".to_string()),
        ("aliased", "(import m :as z)\n(p A)\n".to_string()),
    ] {
        m.load(&module, &src)
            .unwrap_or_else(|e| panic!("{route}: an unreserved name must load: {e}"));
    }
}

/// **There is one reserved-name list, and it is [`ein_core::RESERVED`]** —
/// M1e S1e.2.1's other half.
///
/// The test the finding asked for is *"assert the two lists are one"*, which a
/// shared constant makes trivial and therefore worth stating as a behaviour
/// instead: every name in `ein_core::RESERVED` is unbindable, checked one name
/// at a time through the route that used to leak. A re-forked list fails here
/// the moment the two disagree, and it fails naming the name.
///
/// Two outcomes count as reserved, and the split is the grammar's rather than
/// the loader's: `and`, `neq`, `not` and `or` are SYMBOL-excluded by the lexer
/// ([`crate::lex`]'s own list, which genuinely differs and is
/// [`SE-L2`](../../../../plans/m1e_review_processing/p1e.4_low/s1e.4.2_semantics.md)),
/// so a module declaring one never parses; the rest reach the loader and are
/// refused there. What no name may do is **load**.
#[test]
fn every_ein_core_reserved_name_is_unbindable_through_a_qualified_import() {
    let m = Modules::new("all-names");
    let mut bound: Vec<&str> = Vec::new();
    let mut refused_at_parse: Vec<&str> = Vec::new();
    for name in ein_core::RESERVED {
        let module = declaration("macro", name);
        match m.load(&module, "(import m)\n") {
            Ok(()) => bound.push(name),
            // The module's own parse error, surfaced through `resolve_imports`
            // as a `LoadError` — the lexer refused the name a `SYMBOL` before
            // any declaration existed to check.
            Err(e) if e.contains("unexpected input") => refused_at_parse.push(name),
            Err(e) => assert!(
                e.contains("shadows a reserved kernel name"),
                "{name}: refused, but not as a reserved name: {e}"
            ),
        }
    }
    assert!(
        bound.is_empty(),
        "reserved names a qualified import still binds: {bound:?}"
    );
    assert_eq!(
        refused_at_parse,
        ["and", "neq", "not", "or"],
        "the lexer's SYMBOL exclusions are not the four SE-L2 names"
    );
}

// ── The third resolution tier — M1e `CO-M5` ────────────────────────

/// **A `std.*` module may import only `std.*` modules**, and the refusal is
/// the same in all three tiers.
///
/// `Resolver::locate` derives a module's identity — and the `base_dir` for its
/// *own* imports — from `std::fs::canonicalize(&display)`. Under the embedded
/// root `display` is `<embedded>/x.ein`, which is not a path, so the
/// canonicalisation fails silently and `base_dir` comes back `None`. A stdlib
/// module with a file-relative import therefore resolves under a checkout and
/// under `$EIN_STDLIB`, and fails **only** under the embedded copy — that is,
/// only in an installed binary, and never in this harness, which always sets
/// the override.
///
/// Refusing the shape is what makes the three tiers agree. The stdlib does not
/// contain one today; this builds one, because a check nothing can trip is not
/// a check.
#[test]
fn a_stdlib_module_may_not_import_a_file_relative_one() {
    let dir = std::env::temp_dir().join(format!("ein-stdlib-rel-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch stdlib");
    std::fs::write(dir.join("MANIFEST.sha256"), "").expect("the marker");
    std::fs::write(dir.join("sibling.ein"), "(relation s T)\n").expect("the sibling");
    std::fs::write(dir.join("hasrel.ein"), "(import sibling)\n(relation h T)\n")
        .expect("the offender");

    let resolver = Resolver::with_stdlib(ein_ir::stdlib::Source::Override(dir.clone()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, "(import std.hasrel)\n", Some("<probe>")).expect("parses");
    let err = resolver
        .resolve_imports(&mut ast, &forms, None)
        .expect_err("a std module importing a file-relative one is refused");
    assert!(
        err.0.contains("may import only std.* modules"),
        "the refusal has to say what is wrong: {}",
        err.0
    );
    // The control: the same shape one level out is *fine*, because a puzzle
    // importing a file-relative module is the ordinary case.
    let mut ast = Ast::new();
    let forms = parse(&mut ast, "(import sibling)\n", Some("<probe>")).expect("parses");
    resolver
        .resolve_imports(&mut ast, &forms, Some(&dir))
        .expect("a file-relative import from a file is not a stdlib import");
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The embedded stdlib resolves**, which nothing here had ever asked.
///
/// Three tiers — `$EIN_STDLIB`, the checkout walk, the `include_dir!` copy —
/// and the third is the one a release binary uses and the one the harness can
/// never reach, because it always sets the override
/// (`ein-ir/src/stdlib.rs`'s own note). So the tier with no coverage was the
/// tier that ships. This is the smallest thing that exercises it end to end: a
/// program importing a stdlib module, resolved against `Source::Embedded`, and
/// the declaration it brings in has to arrive.
#[test]
fn the_embedded_stdlib_is_a_resolution_tier_that_works() {
    let resolver = Resolver::with_stdlib(ein_ir::stdlib::Source::Embedded);
    let mut ast = Ast::new();
    let forms = parse(&mut ast, "(import std.algebra)\n", Some("<probe>")).expect("parses");
    let resolved = resolver
        .resolve_imports(&mut ast, &forms, None)
        .expect("std.algebra resolves from the embedded copy");
    assert!(
        !resolved.is_empty(),
        "the embedded module resolved to nothing"
    );
    // Qualified, the way an unaliased import is — so this also pins that the
    // embedded path goes through `qualify` like the other two tiers.
    let names: Vec<String> = resolved
        .iter()
        .filter_map(|&f| {
            let ein_ir::Node::SForm { head, args } = ast.node(f) else {
                return None;
            };
            let _ = head;
            ast.args(args)
                .first()
                .and_then(|&a| ast.atom_name(a))
                .map(str::to_string)
        })
        .collect();
    assert!(
        names.iter().any(|n| n.starts_with("std.algebra.")),
        "nothing arrived under the module's prefix: {names:?}"
    );
}
