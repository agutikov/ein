//! The dumper's two checked-in goldens, and `parse ∘ dump` as a fixed point —
//! S1a.1.2's acceptance.
//!
//! `ein.rs/crates/ein-ir/tests/golden/zebra2.golden` *is*
//! `dump_canonical(parse(zebra2.ein))`: 293 lines of deep nesting, long `:why`
//! templates and non-ASCII, which makes it the dumper's hardest fixture;
//! `zebra.golden` is the same for the classic encoding. The corpus-wide
//! `dump_canonical` / `dump_compact` checks are `corpus_shapes.md5`'s 111
//! `ir[parse]` and 107 `ir[dump-compact]` lines.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-ir --test dump_goldens
//! ```

use ein_corpus::{corpus_files, golden, golden_path, repo_root};
use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::{Ast, parse};

fn parse_file(ast: &mut Ast, path: &std::path::Path) -> Vec<ein_ir::NodeId> {
    let text = std::fs::read_to_string(path).expect("readable");
    parse(ast, &text, Some(path.to_str().expect("utf-8"))).expect("the corpus parses")
}

#[test]
fn the_goldens_are_reproduced() {
    let root = repo_root();
    for (source, name) in [
        ("examples/zebra2.ein", "zebra2.golden"),
        ("examples/zebra.ein", "zebra.golden"),
    ] {
        let mut ast = Ast::new();
        let forms = parse_file(&mut ast, &root.join(source));
        let got = dump_canonical(&ast, &forms);
        if let Some(msg) = golden(&golden_path("ein-ir", name), &got) {
            panic!("{msg}");
        }
    }
}

/// `parse(dump(parse(x))) == parse(x)`, structurally — the property `Loc`'s
/// side table exists to make true.
///
/// Over every corpus file the parser accepts. Until S1f.5.5's swap that was
/// every file outside `examples/broken/`; since, the refusal witnesses of the
/// rulings the grammar carries (W29's, W32's, W35's, W18's, …) are refused at
/// the parse too, wherever they live, and are left out by the parse rather
/// than by their directory.
#[test]
fn dump_then_parse_is_a_fixed_point() {
    let (mut read, mut refused) = (0usize, 0usize);
    for path in corpus_files() {
        let source = std::fs::read_to_string(&path).expect("readable");
        let mut ast = Ast::new();
        let Ok(once) = parse(&mut ast, &source, Some(path.to_str().expect("utf-8"))) else {
            refused += 1;
            continue;
        };
        read += 1;
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
    assert!(
        read >= 300 && refused < read / 4,
        "{read} corpus files read, {refused} refused — the sweep stopped looking"
    );
}
