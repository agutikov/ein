//! Import resolution, minimisation and macro expansion — S1a.1.3's
//! acceptance.
//!
//! The minimisation half matters more than it looks: the surviving declaration
//! set is observable through the plan cache and through firing order, so
//! dropping one declaration too many or too few is a wrong answer several
//! phases later, on a fixture that will look unrelated.
//!
//! Where each part is checked:
//!
//! | claim | where |
//! |---|---|
//! | the three corpus-wide ops (`resolve` / `minimize` / `expand`) | `corpus_shapes.md5`'s 107 `ir[resolve]` + 107 `ir[minimize]` + 107 `ir[expand]` lines, through `dump_canonical` |
//! | `std.macro`'s exported names | [`stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list`], which re-parses `stdlib/macro.ein` at test time |
//! | the 14-module mangling | a checked-in table |
//! | one module under two qualifications | [`one_module_under_two_qualifications_does_not_leak_either_way`] |
//! | what each import form brings of a module's relations, rules, macros and facts | [`what_each_import_form_brings_of_a_module`] (M1f S1f.6.11's table) |
//! | a facts library reached by two flat paths | [`a_diamond_of_flat_imports_yields_each_fact_once`] |
//! | the 9 `.expected` fixtures of the resolver and the expander | [`the_import_and_macro_failures_are_byte_identical`] (12 until M1f S1f.5.5's swap moved three to `examples/broken/`, refused at the parse) |

use std::path::{Path, PathBuf};

use ein_core::Value;
use ein_corpus::{Scratch, golden, golden_path, repo_root};
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
/// The claim the function's name makes, checked directly: the answer is
/// exactly the `(macro …)` heads declared in `stdlib/macro.ein`, re-derived
/// here by parsing the file at test time. A list that drifted from
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
/// text the corpus sweep holds the binary to.
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
    // compared when the loader lands (P1a.2). Since M1f S1f.5.5's swap
    // (T1f.5.5.4) `import_alias_not_a_name` (a string, S1f.5.2),
    // `import_symbols_empty` and `import_symbols_not_a_list` (the paper
    // grammar's `:symbols`, a list of one name or more) are refused at the
    // parse, and moved to `examples/broken/`, where
    // `grammar_decisions.rs` holds their reports.
    for name in [
        "import_as_and_symbols",
        "import_bare_std",
        "import_conflicting_definitions",
        "import_cycle",
        "import_cycle_b",
        "import_module_not_found",
        "import_relative_not_found",
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

    // The macro fixture's message is a `MacroError`; the loader prefixes it
    // with `({head} {name}): ` and the prefix arrives with the loader. Asserting the composition here is what
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
/// so.
#[test]
fn one_module_under_two_qualifications_does_not_leak_either_way() {
    let base = repo_root().join("examples");
    let src = "(import std.macro :as m)\n(import std.macro :symbols (forall))\n";
    let mut ast = Ast::new();
    let forms = parse(&mut ast, src, Some("<string>")).expect("parses");
    // One form a line: since M1f S1f.5.5's swap the stdlib's macros carry a
    // `:why` (the brief's item 4), so the canonical dump breaks each over
    // several lines; the compact one keeps each on one.
    let got = Resolver::new()
        .resolve_imports(&mut ast, &forms, Some(&base))
        .map(|f| {
            f.iter()
                .map(|&n| ein_ir::dump::dump_compact(&ast, n))
                .collect::<Vec<_>>()
                .join("\n")
        })
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
    // so `m.forall` and `forall` differ in exactly one token. A resolver with
    // no cache could not leak, so the point is that the cached one produces
    // the text an uncached one would.
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

/// **Module-name → file-path mangling**, and the places it is surprising only
/// show up in the "module not found" text.
///
/// `std.nope.`, `std..nope`, `a.` and `std.__x` are the interesting rows: an
/// empty segment — a doubled or a trailing dot — is not an error, it is
/// dropped, so the message names a file nobody would have guessed. Checked
/// in as a table because the message *is* the observable — resolution either
/// finds a module or says which file it looked for.
///
/// The path in each message is machine-specific, so the checkout root is
/// replaced by `{ROOT}` the way `examples/broken/load/`'s fixtures use
/// `{FILE}`.
#[test]
fn module_names_mangle_into_these_file_paths() {
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
            format!(
                "({decl} {name} ()\n  :match (p ?x)\n  :assert (q ?x)\n  :why \"p holds of {{?x}}, so q is taken to hold of {{?x}}\")"
            )
        }
        "relation" => format!(
            "(relation {name} T :why \"the relation this module declares holds of {{?1}}\")"
        ),
        "macro" => format!("(macro {name} (?x) (p ?x) :why \"p holds of {{?x}}\")"),
        other => unreachable!("{other}"),
    };
    format!(
        "(relation p T :why \"p holds of {{?1}}\")\n(relation q T :why \"q holds of {{?1}}\")\n{body}\n"
    )
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
///
/// **Since M1f S1f.5.5's swap (T1f.5.5.4) `open` and `absent` are refused at
/// the parse wherever a declarator binds one**, each a word a position gives
/// a meaning to and no name a declarator takes (the paper grammar's default
/// 1): in the program on the direct route, in the module on the other three.
/// The loader's route-independence is held since by a third name the grammar
/// reads as a name, the kernel dunder `__closed__` (T1f.5.5.11), refused as a
/// reserved name through every route: forty-eight cells. **Since M1f S1f.5.23
/// `open` is no name at all** (the user's ruling on Q-M1f.66), so the flat
/// route's `:symbols (open)` is refused at the program's own parse, before the
/// module is read.
#[test]
fn reserved_names_are_reserved_through_every_import_route() {
    let m = Modules::new("routes");
    let mut loaded: Vec<String> = Vec::new();
    for decl in ["rule", "hrule", "relation", "macro"] {
        for name in ["open", "absent", "__closed__"] {
            let module = declaration(decl, name);
            for (route, src) in [
                ("direct", format!("{module}(p A)\n")),
                ("symbols", format!("(import m :symbols ({name}))\n(p A)\n")),
                ("qualified", "(import m)\n(p A)\n".to_string()),
                ("aliased", "(import m :as z)\n(p A)\n".to_string()),
            ] {
                let cell = format!("{decl} {name} via {route}");
                let want = if name.starts_with("__") {
                    "shadows a reserved kernel name".to_string()
                } else if name == "open" && route == "symbols" {
                    "<probe>:1:21: unexpected `open`\n  note: open is reserved".to_string()
                } else {
                    format!(":3:{}: unexpected `{name}`", decl.len() + 3)
                };
                match m.load(&module, &src) {
                    Ok(()) => loaded.push(cell),
                    Err(e) => assert!(
                        e.contains(&want),
                        "{cell}: refused, but not with {want:?}: {e}"
                    ),
                }
            }
        }
    }
    assert!(
        loaded.is_empty(),
        "{} of 48 cells bound a reserved name:\n  {}",
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
/// the loader's. Until M1f S1f.5.5's swap `and`, `neq`, `not` and `or` were
/// SYMBOL-excluded by the hand-written lexer (its own list, which genuinely
/// differed and is
/// [`SE-L2`](../../../../docs/history/m1e_review_processing/README.md#s1e42--semantics-low)),
/// so a module declaring one never parsed, and the rest reached the loader.
/// Since the swap (T1f.5.5.4) every name of the list that is a token of its
/// own is refused at the parse where a declarator binds it, the four words
/// that are never a name and the five a position gives a meaning to and no
/// declarator takes (`absent`, `eq`, `false`, `open`, `relation`); the two
/// kernel dunders, names the grammar reads, reach the loader and are refused
/// there (T1f.5.5.11). What no name may do is **load**.
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
            // as a `LoadError` — the grammar refused the name where a
            // declarator binds one, before any declaration existed to check.
            Err(e) if e.contains(&format!(":3:8: unexpected `{name}`")) => {
                refused_at_parse.push(name)
            }
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
        [
            "absent", "and", "eq", "false", "neq", "not", "open", "or", "relation"
        ],
        "the grammar's refusals are not every token of the list"
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
    let dir = Scratch::new("ein-stdlib-rel");
    std::fs::write(dir.join("MANIFEST.sha256"), "").expect("the marker");
    std::fs::write(dir.join("sibling.ein"), "(relation s T)\n").expect("the sibling");
    std::fs::write(dir.join("hasrel.ein"), "(import sibling)\n(relation h T)\n")
        .expect("the offender");

    let resolver = Resolver::with_stdlib(ein_ir::stdlib::Source::Override(dir.to_path_buf()));
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
}

/// **The embedded stdlib resolves**, which nothing here had ever asked.
///
/// Three tiers — `$EIN_STDLIB`, the checkout walk, the `include_dir!` copy —
/// and the third is the one a release binary uses and the one the harness can
/// never reach, because it always sets the override. So the tier with no
/// coverage was the tier that ships. This is the smallest thing that exercises it end to end: a
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

/// **A `$EIN_STDLIB` that cannot prove itself is refused at the first
/// `std.*` import** — M1e S1e.3.5, `EH-M2`.
///
/// The override was the only one of the three tiers taken on faith: the
/// checkout walk requires `MANIFEST.sha256` because *a directory called
/// `stdlib/` proves nothing*, and the highest-precedence source skipped that
/// test. What a typo'd or stale override produced was *"module not found at
/// &lt;path&gt;/algebra.ein"* — true, and it names the module rather than the
/// variable that chose the directory, so the cost of the mistake was the
/// diagnosis and not the wrong answer.
///
/// Driven through `with_stdlib` rather than the environment: a `#[test]` is a
/// thread of a shared binary, and a `set_var` there is a write every other
/// test can see.
#[test]
fn an_override_without_the_marker_is_refused_by_name() {
    let dir = Scratch::new("ein-bad-stdlib");
    // A module is *there*, so this is a refusal on the marker and not a
    // "module not found" wearing a new coat.
    std::fs::write(dir.join("algebra.ein"), "(relation q T)\n").expect("write");

    let resolver = Resolver::with_stdlib(ein_ir::stdlib::Source::Override(dir.to_path_buf()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, "(import std.algebra)\n", Some("<probe>")).expect("parses");
    let err = resolver
        .resolve_imports(&mut ast, &forms, None)
        .expect_err("a markerless override is refused");
    assert!(
        err.0.contains("$EIN_STDLIB") && err.0.contains("MANIFEST.sha256"),
        "the refusal names neither the variable nor what is missing: {}",
        err.0
    );
    assert!(
        !err.0.contains("module not found"),
        "still the old message: {}",
        err.0
    );

    // …and a program that imports nothing from the stdlib is not refused for
    // the shape of a variable it never reads.
    let mut ast = Ast::new();
    let forms = parse(&mut ast, "(relation p T)\n(p A)\n", Some("<probe>")).expect("parses");
    assert!(resolver.resolve_imports(&mut ast, &forms, None).is_ok());
}

/// **The route that inlines a puzzle's imports into a standalone program.**
///
/// `stdlib/README.md` offered `ein ir parse --resolve` for this until M1e
/// `CD-M5`; the whole `ein ir` subcommand went at P1.11 (2026-06-16,
/// `8378ad7`), and what replaced it is a library call with no CLI in front of
/// it. So the README now shows three functions, and this is the test that says
/// they compose — because a documented route nothing runs is what `CD-M5` was.
///
/// The claim is not just that the dump has no `(import …)` left in it: it is
/// that the standalone text is the **same program**. Minimisation drops
/// imported declarations nothing reaches, and the surviving set is observable
/// through the plan cache and firing order, so "one declaration too few" is a
/// failure several phases away on a fixture that will look unrelated. Loading
/// both and comparing the KBs is the cheap way to say it here.
#[test]
fn the_inlining_route_the_stdlib_readme_documents_round_trips() {
    let puzzle = repo_root().join("examples/zebra2.ein");
    let base = puzzle.parent();

    // The three calls, in the order the README prints them.
    let mut ast = Ast::new();
    let forms = parse_file(&mut ast, &puzzle);
    let inlined = ein_ir::resolve_and_minimize(&mut ast, &forms, base).expect("resolves");
    let standalone = dump_canonical(&ast, &inlined);

    assert!(
        !standalone.contains("(import "),
        "the dump still imports something"
    );
    // A `:why` template only `std.algebra` writes — the rule's *name* is no
    // evidence, since the dump line-breaks a form wider than 80 columns and
    // `(rule\n  symmetric` is what it actually prints.
    assert!(
        standalone.contains(r#":why "{?rel} is symmetric"#),
        "std.algebra's rules were not inlined"
    );

    // …and it is the same program. `base_dir` is `None` on the reload, which
    // is the point of a standalone file: nothing left to resolve against.
    let mut terms = ein_core::Terms::new();
    let want = ein_ir::load(&mut ast, &mut terms, &forms, base).expect("the original loads");

    let mut ast2 = Ast::new();
    let forms2 = parse(&mut ast2, &standalone, Some("<inlined>")).expect("the dump re-parses");
    let mut terms2 = ein_core::Terms::new();
    let got = ein_ir::load(&mut ast2, &mut terms2, &forms2, None).expect("the dump loads");

    // `<KnowledgeBase relations=N rules=N facts=N>` — the three counts
    // minimisation can get wrong, in one comparison.
    assert_eq!(format!("{got:?}"), format!("{want:?}"));
}

// ── M1f S1f.6.11: a flat import brings a module's facts ──────────────────────

/// The ontology library of S1f.6.11's table: a declared relation with two
/// facts, one carrying a `:source`; a relation nothing mentions; a fact and a
/// stored negation of a relation the module does not declare; two rules over
/// `is-a`; a macro; and the three forms that are neither declaration nor fact.
const ONTO: &str = "(relation is-a Thing Type :why \"{?1} is a {?2}\")\n\
     (relation unused Thing :why \"{?1} is never mentioned\")\n\
     (is-a Tea Drink)\n\
     (is-a Milk Drink :source \"the ontology\")\n\
     (likes Tea Milk)\n\
     (not (likes Milk Tea))\n\
     (rule things () :match (is-a ?x Drink) :assert (thing ?x) :why \"{?x} is a thing\")\n\
     (rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{?x} is a liquid\")\n\
     (macro both (?a ?b) (and ?a ?b) :why \"{?a} and {?b}\")\n\
     (query :goal (is-a ?x Drink))\n\
     (solve :hypothesis-relations ())\n\
     (config :record-alternative-justifications false)\n";

/// Each form `src` resolves to against `modules`, one a line, in order, with
/// every `:why` dropped so a row reads as the form it is.
fn resolved(modules: &[(&str, &str)], src: &str) -> Result<Vec<String>, String> {
    let scratch = Scratch::new("ein-import-facts");
    for (name, text) in modules {
        std::fs::write(scratch.path().join(format!("{name}.ein")), text).expect("written");
    }
    let mut ast = Ast::new();
    let forms = parse(&mut ast, src, Some("<program>")).map_err(|e| e.to_string())?;
    let out = Resolver::new()
        .resolve_imports(&mut ast, &forms, Some(scratch.path()))
        .map_err(|e| e.0)?;
    Ok(out
        .iter()
        .map(|&n| {
            let line = ein_ir::dump::dump_compact(&ast, n);
            match line.find(" :why ") {
                Some(i) => format!("{})", &line[..i]),
                None => line,
            }
        })
        .collect())
}

/// **What each import form brings of a module** — M1f S1f.6.11's table, a
/// case per cell (W88).
///
/// Until the stage `:symbols` kept the listed declarations and their closure
/// and dropped every fact, in silence, so a library of facts arrived empty;
/// and a relation the module only states facts of could not be listed. The
/// user ruled on 2026-11-11 that *importing facts is mandatory*. A qualified
/// import is unmoved (point (b)): it keeps every form and renames each
/// declared name wherever it stands, so the module's `is-a` facts are
/// `o.is-a` facts, and a library of facts is imported flat.
#[test]
fn what_each_import_form_brings_of_a_module() {
    let onto = [("onto", ONTO)];
    let lines = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    // Whole-module and aliased: every form, the declared names renamed
    // wherever they stand, the relation heading a fact among them; an
    // undeclared relation (`likes`, and the `thing` a rule asserts) and every
    // constant as written. The query, the solve form and the config come too.
    for (import, p) in [("(import onto)", "onto"), ("(import onto :as o)", "o")] {
        assert_eq!(
            resolved(&onto, import).expect("resolves"),
            lines(&[
                &format!("(relation {p}.is-a Thing Type)"),
                &format!("(relation {p}.unused Thing)"),
                &format!("({p}.is-a Tea Drink)"),
                &format!("({p}.is-a Milk Drink :source \"the ontology\")"),
                "(likes Tea Milk)",
                "(not (likes Milk Tea))",
                &format!("(rule {p}.things () :match ({p}.is-a ?x Drink) :assert (thing ?x))"),
                &format!("(rule {p}.liquids () :match ({p}.is-a ?x Drink) :assert (liquid ?x))"),
                &format!("(macro {p}.both (?a ?b) (and ?a ?b))"),
                &format!("(query :goal ({p}.is-a ?x Drink))"),
                "(solve :hypothesis-relations ())",
                "(config :record-alternative-justifications false)",
            ]),
            "{import}"
        );
    }
    // A listed relation: its declaration and its facts, as written and in the
    // module's order, a `:source` kept. Nothing else: not the relation
    // nothing names, not `likes`, not the rules that would fire on the facts
    // (point (f)), not the macro, and never a query, a solve form or a config.
    assert_eq!(
        resolved(&onto, "(import onto :symbols (is-a))").expect("resolves"),
        lines(&[
            "(relation is-a Thing Type)",
            "(is-a Tea Drink)",
            "(is-a Milk Drink :source \"the ontology\")",
        ])
    );
    // A listed rule: the closure follows the names it writes, so `is-a`'s
    // declaration comes, as it always did, and `is-a`'s facts with it.
    assert_eq!(
        resolved(&onto, "(import onto :symbols (things))").expect("resolves"),
        lines(&[
            "(relation is-a Thing Type)",
            "(is-a Tea Drink)",
            "(is-a Milk Drink :source \"the ontology\")",
            "(rule things () :match (is-a ?x Drink) :assert (thing ?x))",
        ])
    );
    // A relation the module only states facts of may be listed, and a stored
    // negation is a fact of the relation it negates.
    assert_eq!(
        resolved(&onto, "(import onto :symbols (likes))").expect("resolves"),
        lines(&["(likes Tea Milk)", "(not (likes Milk Tea))"])
    );
    // A macro and a relation with no fact: declarations alone, as before.
    assert_eq!(
        resolved(&onto, "(import onto :symbols (both unused))").expect("resolves"),
        lines(&[
            "(relation unused Thing)",
            "(macro both (?a ?b) (and ?a ?b))"
        ])
    );
    // Everything listed, in the module's order whatever the list's.
    assert_eq!(
        resolved(
            &onto,
            "(import onto :symbols (both likes liquids unused things))"
        )
        .expect("resolves"),
        lines(&[
            "(relation is-a Thing Type)",
            "(relation unused Thing)",
            "(is-a Tea Drink)",
            "(is-a Milk Drink :source \"the ontology\")",
            "(likes Tea Milk)",
            "(not (likes Milk Tea))",
            "(rule things () :match (is-a ?x Drink) :assert (thing ?x))",
            "(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x))",
            "(macro both (?a ?b) (and ?a ?b))",
        ])
    );
    // A name the module neither declares nor states a fact of is still
    // refused: `thing` is written only in a rule's `:assert`, and `Tea` only
    // as a value.
    for name in ["thing", "Tea", "nope"] {
        assert_eq!(
            resolved(&onto, &format!("(import onto :symbols ({name}))")),
            Err(format!(
                "(import onto :symbols …) — not provided by the module: {name} at None"
            )),
            "{name}"
        );
    }
}

/// **The closure follows the names a kept declaration writes, and not the
/// names inside a kept fact** (Q-M1f.141), and a fact of a kernel relation
/// comes as any fact does (Q-M1f.142).
#[test]
fn a_selection_follows_declarations_and_not_the_names_inside_a_fact() {
    let m = "(relation owns Person Thing :why \"{?1} owns {?2}\")\n\
         (relation cat Thing :why \"{?1} is a cat\")\n\
         (owns Ann cat)\n\
         (cat Tom)\n\
         (__closed__ owns)\n\
         (__closed__ cat)\n\
         (rule close () :match () :assert (__closed__ pets) :why \"pets is closed\")\n";
    let lines = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    // `(owns Ann cat)` names `cat`, a declared relation with a fact; neither
    // comes with `owns`, and neither does `(__closed__ owns)`.
    assert_eq!(
        resolved(&[("m", m)], "(import m :symbols (owns))").expect("resolves"),
        lines(&["(relation owns Person Thing)", "(owns Ann cat)"])
    );
    // The marker is a relation's name like another: listed, it brings the
    // module's `(__closed__ …)` facts; and a kept rule that writes it does.
    assert_eq!(
        resolved(&[("m", m)], "(import m :symbols (owns __closed__))").expect("resolves"),
        lines(&[
            "(relation owns Person Thing)",
            "(owns Ann cat)",
            "(__closed__ owns)",
            "(__closed__ cat)",
        ])
    );
    assert_eq!(
        resolved(&[("m", m)], "(import m :symbols (close))").expect("resolves"),
        lines(&[
            "(__closed__ owns)",
            "(__closed__ cat)",
            "(rule close () :match () :assert (__closed__ pets))",
        ])
    );
}

/// **A diamond of flat imports of one facts library yields each fact once**
/// (M1f S1f.6.11's T4).
///
/// A flat diamond over *declarations* always collapsed, identical ones being
/// one (`dedup_declarations`). A fact has no name to collapse on, and a
/// module is parsed once per resolution, so a statement that arrives a second
/// time is the same node and is kept once, where it first arrived. The same
/// library imported flat twice by one program is the same case. A fact the
/// program itself writes beside the library's is another statement and stays,
/// for the loader, to which a fact stated twice is one fact.
#[test]
fn a_diamond_of_flat_imports_yields_each_fact_once() {
    let modules = [
        ("onto", ONTO),
        (
            "left",
            "(import onto :symbols (is-a))\n(is-a Cola Drink)\n\
             (rule fizzy () :match (is-a ?x Drink) :assert (fizzy ?x) :why \"{?x} is fizzy\")\n",
        ),
        (
            "right",
            "(import onto :symbols (is-a likes))\n(is-a Water Drink)\n\
             (rule wet () :match (is-a ?x Drink) :assert (wet ?x) :why \"{?x} is wet\")\n",
        ),
    ];
    let lines = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    // Each side re-exports the library's facts: they are facts of the side.
    let got = resolved(
        &modules,
        "(import left :symbols (fizzy))\n(import right :symbols (wet likes))\n(is-a Tea Drink)\n",
    )
    .expect("resolves");
    assert_eq!(
        got,
        lines(&[
            "(relation is-a Thing Type)",
            "(is-a Tea Drink)",
            "(is-a Milk Drink :source \"the ontology\")",
            "(is-a Cola Drink)",
            "(rule fizzy () :match (is-a ?x Drink) :assert (fizzy ?x))",
            "(likes Tea Milk)",
            "(not (likes Milk Tea))",
            "(is-a Water Drink)",
            "(rule wet () :match (is-a ?x Drink) :assert (wet ?x))",
            // The program's own statement of a fact the library states.
            "(is-a Tea Drink)",
        ])
    );
    // One library, imported flat twice by one program.
    assert_eq!(
        resolved(
            &modules,
            "(import onto :symbols (is-a))\n(import onto :symbols (things))\n"
        )
        .expect("resolves"),
        lines(&[
            "(relation is-a Thing Type)",
            "(is-a Tea Drink)",
            "(is-a Milk Drink :source \"the ontology\")",
            "(rule things () :match (is-a ?x Drink) :assert (thing ?x))",
        ])
    );
    // And loaded, the diamond's program holds each fact once.
    let scratch = Scratch::new("ein-import-facts-diamond");
    for (name, text) in modules {
        std::fs::write(scratch.path().join(format!("{name}.ein")), text).expect("written");
    }
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let forms = parse(
        &mut ast,
        "(import left :symbols (fizzy))\n(import right :symbols (wet likes))\n",
        Some("<program>"),
    )
    .expect("parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, Some(scratch.path())).expect("loads");
    let mut facts: Vec<String> = kb
        .facts()
        .map(|f| terms.display(Value::fact(f)))
        .filter(|f| !f.starts_with("(relation "))
        .collect();
    facts.sort();
    assert_eq!(
        facts,
        lines(&[
            "(is-a Cola Drink)",
            "(is-a Milk Drink)",
            "(is-a Tea Drink)",
            "(is-a Water Drink)",
            "(likes Tea Milk)",
            "(not (likes Milk Tea))",
        ])
    );
}
