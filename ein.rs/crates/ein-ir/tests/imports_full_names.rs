//! Imports by full name — M1f S1f.6.35, the user's model (h) of 2026-11-13 and
//! its ruling of 2026-11-14 (*"in all cases we import everything, the
//! difference only in names"*), at the resolver.
//!
//! | claim | where |
//! |---|---|
//! | the user's dependency graph, row by row, as `import_clashes.md` § 7's *under (h)* column says | [`the_users_dependency_graph`] |
//! | a short name is local to the file that binds it, and renames nothing | [`a_short_name_is_local_to_its_file`] |
//! | a module's name is a path from the program file's directory, for a module's own imports too | [`a_module_is_found_from_the_programs_directory`] |
//! | the refusals, one each | [`the_refusals`] |
//! | what prints: the main file's spelling, the full name otherwise, never two names as one | [`what_prints`] |
//!
//! What the answers are, R and claims, under every import form, is
//! `ein-cli`'s `import_invariance.rs`; the calculator's acceptance,
//! `imports_by_full_name.rs`.

use ein_corpus::Scratch;
use ein_ir::imports::Resolver;
use ein_ir::{Ast, parse};

/// Each form `src` resolves to against `modules`, one a line, every `:why`
/// dropped; or the refusal.
fn resolved(modules: &[(&str, &str)], src: &str) -> Result<Vec<String>, String> {
    let scratch = Scratch::new("ein-full-names");
    for (name, text) in modules {
        let path = scratch.path().join(format!("{name}.ein"));
        std::fs::create_dir_all(path.parent().expect("a directory")).expect("made");
        std::fs::write(path, text).expect("written");
    }
    let mut ast = Ast::new();
    let forms = parse(&mut ast, src, Some("p.ein")).map_err(|e| e.to_string())?;
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

fn lines(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// The coordinator's scratch modules of § 7: `A` with a relation in, a rule
/// out, an entry and a seed; `B`, `C` and `E` each over the one below by its
/// entry, passing its output on.
const A: &str = "(relation a-in Thing :why \"in\")\n\
     (relation a-out Thing :why \"out\")\n\
     (a-in seed)\n\
     (rule a-step () :match (a-in ?x) :assert (a-out ?x) :why \"step\")\n\
     (rule a () :match () :assert (and (__closed__ a-in) (__closed__ a-out)) :why \"entry\")\n";

fn over(up: &str, lo: &str) -> String {
    let (u, l) = (up.to_lowercase(), lo.to_lowercase());
    format!(
        "(import {lo} :symbols ({l} {l}-out))\n\
         (relation {u}-out Thing :why \"out\")\n\
         (rule {u}-step () :match ({l}-out ?x) :assert ({u}-out ?x) :why \"step\")\n\
         (rule {u} () :match () :assert (__closed__ {u}-out) :why \"entry\")\n"
    )
}

/// **The user's dependency graph** (*"A <import- B <import- C <import- Prog,
/// A <import- Prog, A <import- E <import- Prog"*), each row of § 7's table.
#[test]
fn the_users_dependency_graph() {
    let b = over("B", "A");
    let c = over("C", "B");
    let e = over("E", "A");
    let graph = [
        ("A", A),
        ("B", b.as_str()),
        ("C", c.as_str()),
        ("E", e.as_str()),
    ];
    let count = |got: &[String], head: &str| got.iter().filter(|l| l.starts_with(head)).count();

    // All flat: one copy of A, by full name; the program's `(a-in mine)`
    // joins A's because its import of A lists `a-in`.
    let got = resolved(
        &graph,
        "(import C :symbols (c c-out))\n(import A :symbols (a a-in a-out))\n\
         (import E :symbols (e e-out))\n(a-in mine)\n",
    )
    .expect("resolves");
    assert_eq!(count(&got, "(rule A.a-step "), 1, "{got:#?}");
    assert_eq!(count(&got, "(a-in seed)"), 1, "{got:#?}");
    assert!(got.contains(&"(a-in mine)".to_string()), "{got:#?}");
    assert!(
        got.contains(&"(rule A.a-step () :match (a-in ?x) :assert (a-out ?x))".to_string()),
        "{got:#?}"
    );

    // C alone, writing `a-in` and `a-out`: the program's own (shadowing);
    // A's are `A.a-in` and `A.a-out`, A brought by B's import.
    let got = resolved(&graph, "(import C :symbols (c c-out))\n(a-in mine)\n").expect("resolves");
    assert!(got.contains(&"(A.a-in seed)".to_string()), "{got:#?}");
    assert!(got.contains(&"(a-in mine)".to_string()), "{got:#?}");

    // Its own `a-out` declared beside C alone: loads, its own.
    let got = resolved(
        &graph,
        "(import C :symbols (c c-out))\n(relation a-out Thing :why \"mine\")\n",
    )
    .expect("loads");
    assert_eq!(count(&got, "(relation a-out "), 1, "{got:#?}");
    assert_eq!(count(&got, "(relation A.a-out "), 1, "{got:#?}");

    // A aliased, the rest flat: one copy, printed under the alias.
    let got = resolved(
        &graph,
        "(import A :as qa)\n(import C :symbols (c c-out))\n(import E :symbols (e e-out))\n",
    )
    .expect("resolves");
    assert_eq!(count(&got, "(rule qa.a-step "), 1, "{got:#?}");
    assert_eq!(count(&got, "(qa.a-in seed)"), 1, "{got:#?}");
    assert!(
        got.contains(&"(rule B.b-step () :match (qa.a-out ?x) :assert (B.b-out ?x))".to_string()),
        "{got:#?}"
    );

    // `(import C :as cc)` beside a flat A: one copy of A; `cc.` aliases C's
    // own names.
    let got =
        resolved(&graph, "(import C :as cc)\n(import A :symbols (a a-out))\n").expect("resolves");
    assert_eq!(count(&got, "(rule A.a-step "), 1, "{got:#?}");
    assert!(
        got.contains(&"(rule cc.c-step () :match (B.b-out ?x) :assert (cc.c-out ?x))".to_string()),
        "{got:#?}"
    );

    // An alias inside an alias: the defining module's full name.
    let bq = "(import A :as aa)\n(relation bq-out Thing :why \"out\")\n\
              (rule bq-step () :match (aa.a-out ?x) :assert (bq-out ?x) :why \"step\")\n";
    let got =
        resolved(&[("A", A), ("Bq", bq)], "(import Bq :as bb)\n(A.a-out x)\n").expect("resolves");
    assert!(
        got.contains(
            &"(rule bb.bq-step () :match (A.a-out ?x) :assert (bb.bq-out ?x))".to_string()
        ),
        "{got:#?}"
    );
    assert!(got.contains(&"(A.a-out x)".to_string()), "{got:#?}");

    // A cycle X → Y → X: refused, as it always was.
    let err = resolved(
        &[
            ("X", "(import Y :symbols (y))\n(relation x T :why \"x\")\n"),
            ("Y", "(import X :symbols (x))\n(relation y T :why \"y\")\n"),
        ],
        "(import X :symbols (x))\n",
    )
    .expect_err("a cycle");
    assert!(err.starts_with("import cycle: "), "{err}");

    // `(A.a-in x)` with A imported flat: A's own `a-in`, by its full name,
    // printed by the program's short name.
    let got = resolved(&graph, "(import A :symbols (a a-in))\n(A.a-in x)\n").expect("resolves");
    assert!(got.contains(&"(a-in x)".to_string()), "{got:#?}");

    // A declaration under an imported module's full name: refused (Q-M1f.239).
    let err = resolved(
        &graph,
        "(import A :symbols (a))\n(relation A.mine T :why \"m\")\n",
    )
    .expect_err("refused");
    assert!(err.contains("Q-M1f.239"), "{err}");
}

/// **A short name is local to the file that binds it, and renames nothing**:
/// a module's own short names never reach its importer (no re-export), and a
/// listed name is the module's own declaration under its full name.
#[test]
fn a_short_name_is_local_to_its_file() {
    let lower = "(relation ask T :why \"ask\")\n(ask x)\n";
    let upper = "(import lower :symbols (ask))\n(relation got T :why \"got\")\n\
                 (rule up () :match (ask ?x) :assert (got ?x) :why \"up\")\n";
    let m = [("lower", lower), ("upper", upper)];
    // The program writes `ask` without listing it: its own, beside lower's.
    let got = resolved(&m, "(import upper :symbols (got))\n(ask y)\n").expect("resolves");
    assert_eq!(
        got,
        lines(&[
            "(relation lower.ask T)",
            "(lower.ask x)",
            "(relation got T)",
            "(rule upper.up () :match (lower.ask ?x) :assert (got ?x))",
            "(ask y)",
        ])
    );
    // Listing `ask` from upper, which only imports it: not provided.
    let err = resolved(&m, "(import upper :symbols (ask))\n").expect_err("no re-export");
    assert!(err.contains("not provided by the module: ask"), "{err}");
}

/// **A module's name is a path from the program file's directory**, a dot
/// per directory, for the imports a module writes too (the user, 2026-11-13:
/// *"(import word-theory.gaps :as gaps) having global names like
/// word-theory.gaps.gaps"*).
#[test]
fn a_module_is_found_from_the_programs_directory() {
    let words = "(relation holds T :why \"h\")\n";
    let gaps = "(import word-theory.words :symbols (holds))\n(relation gaps T :why \"g\")\n\
                (rule fill () :match (holds ?x) :assert (gaps ?x) :why \"f\")\n";
    let m = [("word-theory/words", words), ("word-theory/gaps", gaps)];
    let got =
        resolved(&m, "(import word-theory.gaps :as gaps)\n(gaps.gaps a)\n").expect("resolves");
    assert_eq!(
        got,
        lines(&[
            "(relation word-theory.words.holds T)",
            "(relation gaps.gaps T)",
            "(rule gaps.fill () :match (word-theory.words.holds ?x) :assert (gaps.gaps ?x))",
            "(gaps.gaps a)",
        ])
    );
    // The module's own import written from its own directory is not found:
    // it is read from the program's.
    let gaps_local = "(import words :symbols (holds))\n(relation gaps T :why \"g\")\n";
    let err = resolved(
        &[
            ("word-theory/words", words),
            ("word-theory/gaps", gaps_local),
        ],
        "(import word-theory.gaps :as gaps)\n",
    )
    .expect_err("found from the program's directory");
    assert!(
        err.contains("(import words) — module not found at"),
        "{err}"
    );
}

/// **The refusals**, each a section of `defined_behaviour.md` § 4.
#[test]
fn the_refusals() {
    let m = "(relation done T :why \"d\")\n(relation sum T :why \"s\")\n";
    let n = "(relation done T :why \"n's done\")\n";
    let mods = [("m", m), ("n", n)];
    let refused = |src: &str, mods: &[(&str, &str)]| resolved(mods, src).expect_err(src);
    // One short name, two full names, by two imports (Q-M1f.234).
    let e = refused(
        "(import m :symbols (done))\n(import n :symbols (done))\n",
        &mods,
    );
    assert!(
        e.contains("'done' is bound to two full names in the program, m.done and n.done"),
        "{e}"
    );
    // …and by an import and the file's own declaration.
    let e = refused(
        "(import m :symbols (done))\n(relation done T :why \"mine\")\n",
        &mods,
    );
    assert!(e.contains("done (its own declaration) and m.done"), "{e}");
    // One short name bound to one full name twice is not an error.
    resolved(
        &mods,
        "(import m :symbols (done))\n(import m :symbols (done sum))\n",
    )
    .expect("loads");
    // An alias prefixing two modules in one file.
    let e = refused("(import m :as x)\n(import n :as x)\n", &mods);
    assert!(e.contains("'x' prefixes two modules in one file"), "{e}");
    // A prefixed name that names nothing (Q-M1f.251): by alias, by full name.
    let e = refused("(import m :as x)\n(x.nope a)\n", &mods);
    assert!(
        e.contains("'x.nope' names nothing: m declares no 'nope'"),
        "{e}"
    );
    let e = refused("(import m :symbols (done))\n(m.nope a)\n", &mods);
    assert!(
        e.contains("'m.nope' names nothing: m declares no 'nope'"),
        "{e}"
    );
    // A full name of a module no import brings (Q-M1f.238), with an import of
    // another module and with none.
    let e = refused("(import m :symbols (done))\n(n.done a)\n", &mods);
    assert!(
        e.contains("'n.done' is a full name of n, which no import of the program brings"),
        "{e}"
    );
    let e = refused("(std.math.decimal-arith.add a b)\n", &mods);
    assert!(
        e.contains("'std.math.decimal-arith.add' is a full name of std.math.decimal-arith"),
        "{e}"
    );
    // …and a name the program declares is its own, whatever its dots: a
    // state file declares the full names it was loaded with.
    resolved(
        &mods,
        "(relation std.math.decimal-arith.add T T :why \"a\")\n(std.math.decimal-arith.add a b)\n",
    )
    .expect("a state file's declarations");
    // A declaration of the program under an imported module's full name
    // (Q-M1f.239).
    let e = refused(
        "(import m :symbols (done))\n(relation m.mine T :why \"x\")\n",
        &mods,
    );
    assert!(e.contains("Q-M1f.239"), "{e}");
    // A module's declared name with a dot (Q-M1f.241).
    let e = refused(
        "(import d :symbols (x))\n",
        &[(
            "d",
            "(relation x T :why \"x\")\n(relation y.z T :why \"y\")\n",
        )],
    );
    assert!(e.contains("d declares 'y.z', a name with a dot"), "{e}");
    // An empty segment or a `/` in a module's name (Q-M1f.252).
    for module in ["..m", "m.", "a/b", "../m"] {
        let e = refused(&format!("(import {module})\n"), &mods);
        assert!(e.contains("(Q-M1f.252)"), "{module}: {e}");
    }
}

/// **What prints**: the main file's listed short name, else its first alias's
/// spelling, else the full name (Q-M1f.237, Q-M1f.253); a spelling another
/// file writes as a name of its own is not taken.
#[test]
fn what_prints() {
    let m = "(relation done T :why \"d\")\n(relation sum T :why \"s\")\n(done a)\n(sum a)\n";
    let mods = [("m", m)];
    assert_eq!(
        resolved(&mods, "(import m :symbols (done))\n(import m :as k)\n").expect("resolves"),
        lines(&[
            "(relation done T)",
            "(relation k.sum T)",
            "(done a)",
            "(k.sum a)"
        ])
    );
    assert_eq!(
        resolved(&mods, "(import m)\n").expect("resolves"),
        lines(&[
            "(relation m.done T)",
            "(relation m.sum T)",
            "(m.done a)",
            "(m.sum a)"
        ])
    );
    // `w` writes an undeclared `done` of its own: the program's listed `done`
    // is m's, and prints in full, so that the two never read as one.
    let w = "(done b)\n";
    assert_eq!(
        resolved(
            &[("m", m), ("w", w)],
            "(import m :symbols (done))\n(import w)\n"
        )
        .expect("resolves"),
        lines(&[
            "(relation m.done T)",
            "(relation m.sum T)",
            "(m.done a)",
            "(m.sum a)",
            "(done b)"
        ])
    );
}
