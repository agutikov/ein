//! The dumper's two checked-in goldens, and `parse ∘ dump` as a fixed point —
//! S1a.1.2's acceptance, without the oracle.
//!
//! `ein.rs/crates/ein-ir/tests/golden/from_ein_py/zebra2.golden` *is* `dump_canonical(parse(zebra2.ein))`
//! as **ein.py wrote it**, checked in years before the port: 293 lines of deep
//! nesting, long `:why` templates and non-ASCII. That makes it the last
//! independent provenance the repo has, which is why
//! [the ledger §4](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#4-what-the-removal-must-relocate)
//! lists it among the files [S1a.10.5](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
//! must `git mv` rather than re-bless — a golden regenerated from ein.rs would
//! say "ein.rs reproduces itself".
//!
//! What S1a.10.2 removed from this file is the corpus-wide `dump_canonical` /
//! `dump_compact` diffs, now `corpus_shapes.md5`'s 111 `ir[parse]` and 107
//! `ir[dump-compact]` lines, and ein.py's own fixed-point check — a test that
//! ran no ein.rs code at all, whose subject was `ein/ir/dump.py`.

use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::{Ast, parse};
use ein_oracle::{corpus_files, repo_root};

fn parse_file(ast: &mut Ast, path: &std::path::Path) -> Vec<ein_ir::NodeId> {
    let text = std::fs::read_to_string(path).expect("readable");
    parse(ast, &text, Some(path.to_str().expect("utf-8"))).expect("the corpus parses")
}

#[test]
fn the_goldens_are_reproduced() {
    let root = repo_root();
    for (source, golden) in [
        (
            "examples/zebra2.ein",
            "ein.rs/crates/ein-ir/tests/golden/from_ein_py/zebra2.golden",
        ),
        (
            "examples/zebra.ein",
            "ein.rs/crates/ein-ir/tests/golden/from_ein_py/zebra.golden",
        ),
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
