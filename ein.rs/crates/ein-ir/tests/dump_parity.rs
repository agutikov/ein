//! S1a.1.2 acceptance — the dumper is byte-identical to `ir/dump.py`, and
//! `parse ∘ dump` is the identity on the AST.
//!
//! `ein.py/tests/golden/zebra2.golden` *is* `dump_canonical(parse(zebra2.ein))`,
//! so this is the gate that says the port produces the same text a checked-in
//! artefact already pins — 293 lines of deep nesting, long `:why` templates
//! and non-ASCII.

use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::{Ast, parse};
use ein_oracle::{IR_ORACLE, Oracle, corpus_files, repo_root, skip};

fn parse_file(ast: &mut Ast, path: &std::path::Path) -> Vec<ein_ir::NodeId> {
    let text = std::fs::read_to_string(path).expect("readable");
    parse(ast, &text, Some(path.to_str().expect("utf-8"))).expect("the corpus parses")
}

#[test]
fn the_corpus_dumps_identically() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_corpus_dumps_identically");
    };
    let files = corpus_files();
    let mut bad = Vec::new();
    for path in &files {
        // `broken/` is parse-negative for the four at the top level; the
        // load-negative fixtures under it parse fine and are included.
        if path.parent().is_some_and(|p| p.ends_with("broken")) {
            continue;
        }
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, path);
        let got = dump_canonical(&ast, &forms);
        let want = py.file("parse", path).unwrap().to_string();
        if got != want {
            bad.push(format!(
                "{}\n{}",
                path.display(),
                first_difference(&got, &want)
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} files differ:\n{}",
        bad.len(),
        files.len(),
        bad.join("\n")
    );
}

#[test]
fn the_corpus_dumps_compactly_identically() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_corpus_dumps_compactly_identically");
    };
    let mut bad = Vec::new();
    for path in corpus_files() {
        if path.parent().is_some_and(|p| p.ends_with("broken")) {
            continue;
        }
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, &path);
        let got: Vec<String> = forms.iter().map(|f| dump_compact(&ast, *f)).collect();
        let want = py.file("compact", &path).unwrap().to_string();
        if got.join("\n") != want {
            bad.push(format!(
                "{}\n{}",
                path.display(),
                first_difference(&got.join("\n"), &want)
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} file(s) differ:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// The checked-in artefacts, compared directly rather than through the oracle
/// — so the test still means something if the oracle and the golden ever part
/// company.
#[test]
fn the_goldens_are_reproduced() {
    let root = repo_root();
    for (source, golden) in [
        ("examples/zebra2.ein", "ein.py/tests/golden/zebra2.golden"),
        ("examples/zebra.ein", "ein.py/tests/golden/zebra.golden"),
    ] {
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, &root.join(source));
        let got = dump_canonical(&ast, &forms);
        let want = std::fs::read_to_string(root.join(golden)).expect("golden");
        assert_eq!(got, want, "{golden}\n{}", first_difference(&got, &want));
    }
}

/// `parse(dump(parse(x))) == parse(x)`, structurally — the property `Loc`'s
/// side table exists to make true.
#[test]
fn dump_then_parse_is_a_fixed_point() {
    for path in corpus_files() {
        if path.parent().is_some_and(|p| p.ends_with("broken")) {
            continue;
        }
        let mut ast = Ast::new();
        let once = parse_file(&mut ast, &path);
        let text = dump_canonical(&ast, &once);
        let twice = parse(&mut ast, &text, Some("<dumped>")).expect("a dump re-parses");
        assert_eq!(
            once.len(),
            twice.len(),
            "{}: form count moved",
            path.display()
        );
        for (a, b) in once.iter().zip(&twice) {
            assert!(
                ast.eq_nodes(*a, *b),
                "{}: {} != {}",
                path.display(),
                dump_compact(&ast, *a),
                dump_compact(&ast, *b)
            );
        }
        // And the text is a fixed point too, which is the observable half.
        assert_eq!(text, dump_canonical(&ast, &twice), "{}", path.display());
    }
}

/// The same property on the *other* side, over the same inputs — a shared
/// generator is only shared if both implementations actually run it.
#[test]
fn dump_then_parse_is_a_fixed_point_in_ein_py_too() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("dump_then_parse_is_a_fixed_point_in_ein_py_too");
    };
    for path in corpus_files() {
        if path.parent().is_some_and(|p| p.ends_with("broken")) {
            continue;
        }
        let once = py.file("parse", &path).unwrap().to_string();
        let twice = py
            .text("parse", &once, Some("<dumped>"))
            .unwrap()
            .to_string();
        assert_eq!(once, twice, "{}", path.display());
    }
}

/// Name the first line that differs — a 293-line diff reported as "not equal"
/// is not a report.
fn first_difference(got: &str, want: &str) -> String {
    for (i, (a, b)) in got.lines().zip(want.lines()).enumerate() {
        if a != b {
            return format!("  line {}\n    ein.py: {b:?}\n    ein.rs: {a:?}", i + 1);
        }
    }
    format!(
        "  same {} line(s), then ein.py has {} and ein.rs {}",
        got.lines().count().min(want.lines().count()),
        want.lines().count(),
        got.lines().count()
    )
}
