//! **`std.digits`, every row held to the integers** — M1f
//! [S1f.6.2](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.2_digits_and_tables.md).
//!
//! `stdlib/digits.ein` is the calculator's gates: seven tables of plain facts,
//! written by `utils/gen_arith_tables.py`. Ein has no numbers, so nothing in
//! the engine can say whether `(fa 7 5 0 2 1)` is true; a wrong row would be a
//! wrong answer in every program that adds 7 and 5, and no golden would know.
//! What holds the rows is this file, in three claims:
//!
//! | claim | where |
//! |---|---|
//! | every row agrees with Rust's integers, **none missing and none twice** | [`every_row_agrees_with_the_integers`] |
//! | the module is a library of facts: seven declarations, a `(__closed__ R)` fact for each, the rows, and nothing else | [`the_module_is_seven_closed_tables_and_nothing_else`] |
//! | the file is what the generator writes — the script's `--check`, which is how that check is in the gate | [`the_file_is_what_the_generator_writes`] |
//! | a flat import brings a listed table's rows, and the closing facts when `__closed__` is listed, from the **embedded** copy too | [`a_flat_import_brings_a_table_from_the_embedded_copy`] |
//!
//! The rows are computed twice, in Python by the generator and in Rust here,
//! and compared as sorted lists, so a missing row, an extra one and a
//! duplicate each fail with the first row that differs.

use std::collections::BTreeMap;
use std::process::{Command, Stdio};

use ein_corpus::repo_root;
use ein_ir::imports::Resolver;
use ein_ir::{Ast, Node, NodeId, parse};

/// The seven tables, in the order the module declares them.
const TABLES: [&str; 7] = ["fa", "fs", "md", "cl", "nz", "mid", "adj"];

type Row = Vec<String>;

fn row<const N: usize>(cells: [String; N]) -> Row {
    cells.to_vec()
}

fn d(n: i32) -> String {
    n.to_string()
}

/// What each table must hold, from Rust's integers alone.
fn expected() -> BTreeMap<&'static str, Vec<Row>> {
    let mut t: BTreeMap<&'static str, Vec<Row>> = BTreeMap::new();
    // fa: a + b + carry in = 10 × carry out + digit.
    // fs: a − b − borrow in = digit − 10 × borrow out.
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..2 {
                let s = a + b + c;
                t.entry("fa")
                    .or_default()
                    .push(row([d(a), d(b), d(c), d(s % 10), d(s / 10)]));
                let m: i32 = a - b - c;
                t.entry("fs").or_default().push(row([
                    d(a),
                    d(b),
                    d(c),
                    d(m.rem_euclid(10)),
                    d(i32::from(m < 0)),
                ]));
            }
            // md: a × b + carry in = 10 × carry out + digit, carry in 0 to 8.
            for c in 0..9 {
                let p = a * b + c;
                t.entry("md")
                    .or_default()
                    .push(row([d(a), d(b), d(c), d(p % 10), d(p / 10)]));
            }
            // cl: a differing digit overrules the order below; equal digits keep it.
            for below in ["lt", "eq", "gt"] {
                let order = match a.cmp(&b) {
                    std::cmp::Ordering::Less => "lt",
                    std::cmp::Ordering::Greater => "gt",
                    std::cmp::Ordering::Equal => below,
                };
                t.entry("cl").or_default().push(row([
                    below.to_string(),
                    d(a),
                    d(b),
                    order.to_string(),
                ]));
            }
        }
    }
    for k in 1..10 {
        t.entry("nz").or_default().push(row([d(k)]));
    }
    // mid: two bounds from 0 to 10, at least two apart, and the digit between.
    for lo in 0..=10 {
        for hi in 0..=10 {
            if hi - lo >= 2 {
                t.entry("mid")
                    .or_default()
                    .push(row([d(lo), d(hi), d((lo + hi) / 2)]));
            }
            if hi - lo == 1 {
                t.entry("adj").or_default().push(row([d(lo), d(hi)]));
            }
        }
    }
    for rows in t.values_mut() {
        rows.sort();
    }
    t
}

fn module_text() -> String {
    let path = repo_root().join("stdlib/digits.ein");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A top-level form as `(head, atom arguments)`; `None` for an argument that
/// is not a bare name, which no row of a table may hold.
fn flat(ast: &Ast, form: NodeId) -> (String, Option<Row>) {
    let head = ast.head_name(form).unwrap_or("?").to_string();
    let args: Option<Row> = ast
        .form_args(form)
        .iter()
        .map(|&a| ast.atom_name(a).map(str::to_string))
        .collect();
    (head, args)
}

/// Every top-level form of a form list, by head.
fn by_head(ast: &Ast, forms: &[NodeId]) -> BTreeMap<String, Vec<Row>> {
    let mut out: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for &f in forms {
        assert!(
            matches!(ast.node(f), Node::SForm { .. }),
            "a top-level form that is not a form"
        );
        let (head, args) = flat(ast, f);
        if head == "relation" {
            // `(relation fa Digit … :why "…")`: keep the name alone.
            let name = ast
                .form_args(f)
                .first()
                .and_then(|&a| ast.atom_name(a))
                .expect("a relation's name");
            out.entry(head).or_default().push(vec![name.to_string()]);
            continue;
        }
        let args =
            args.unwrap_or_else(|| panic!("a `{head}` form with an argument that is not a name"));
        out.entry(head).or_default().push(args);
    }
    out
}

#[test]
fn every_row_agrees_with_the_integers() {
    let mut ast = Ast::new();
    let forms =
        parse(&mut ast, &module_text(), Some("stdlib/digits.ein")).expect("std.digits parses");
    let got = by_head(&ast, &forms);
    let want = expected();
    let sizes: Vec<(&str, usize)> = TABLES.iter().map(|t| (*t, want[t].len())).collect();
    assert_eq!(
        sizes,
        [
            ("fa", 200),
            ("fs", 200),
            ("md", 900),
            ("cl", 300),
            ("nz", 9),
            ("mid", 45),
            ("adj", 10)
        ],
        "the integers' own table sizes — S1f.6.2's table"
    );
    for table in TABLES {
        let mut rows = got.get(table).cloned().unwrap_or_default();
        // Sorted and **not** deduplicated: a row written twice is a finding.
        rows.sort();
        let want = &want[table];
        if let Some(i) = (0..rows.len().max(want.len())).find(|&i| rows.get(i) != want.get(i)) {
            panic!(
                "std.digits' `{table}` differs from the integers at sorted row {i}: \
                 the module has {:?}, the integers say {:?} ({} rows against {})",
                rows.get(i),
                want.get(i),
                rows.len(),
                want.len()
            );
        }
    }
}

#[test]
fn the_module_is_seven_closed_tables_and_nothing_else() {
    let mut ast = Ast::new();
    let forms =
        parse(&mut ast, &module_text(), Some("stdlib/digits.ein")).expect("std.digits parses");
    let got = by_head(&ast, &forms);
    let tables: Vec<Row> = TABLES.iter().map(|t| vec![(*t).to_string()]).collect();
    assert_eq!(got["relation"], tables, "the declarations, in order");
    assert_eq!(
        got["__closed__"], tables,
        "one written `(__closed__ R)` per table: a row is never a hypothesis"
    );
    let mut heads: Vec<&str> = got.keys().map(String::as_str).collect();
    heads.sort_unstable();
    let mut want: Vec<&str> = TABLES.to_vec();
    want.extend(["relation", "__closed__"]);
    want.sort_unstable();
    assert_eq!(
        heads, want,
        "a library of facts: no rule, macro, import, query or solve form, \
         and no fact of an eighth relation"
    );
}

/// `utils/gen_arith_tables.py --check`, run here so that it is in the gate:
/// the per-commit tier runs `cargo test --workspace`, and its job installs a
/// `python3` for `cli_semantics.rs`'s `gen_zebra2_variants.py --check`, the
/// same shape (Q-M1f.145).
#[test]
fn the_file_is_what_the_generator_writes() {
    let probe = Command::new("python3")
        .args(["-c", "pass"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    assert!(
        matches!(probe, Ok(s) if s.success()),
        "`python3` does not run ({probe:?}): `stdlib/digits.ein` is generated and \
         `utils/gen_arith_tables.py --check` is the only check that compares its \
         bytes with the script's — a missing gate, not a missing convenience"
    );
    let out = Command::new("python3")
        .args(["utils/gen_arith_tables.py", "--check"])
        .current_dir(repo_root())
        .output()
        .expect("python3 ran a moment ago");
    assert!(
        out.status.success(),
        "stdlib/digits.ein is stale — run `python3 utils/gen_arith_tables.py` and \
         `python3 utils/stdlib_manifest.py --write`\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The tier a release binary uses. A listed table arrives with its rows, a
/// table not listed does not, and `__closed__` brings the module's seven
/// closing facts (Q-M1f.142: a fact of a kernel relation comes when its
/// relation's name is in the closure).
#[test]
fn a_flat_import_brings_a_table_from_the_embedded_copy() {
    let resolver = Resolver::with_stdlib(ein_ir::stdlib::Source::Embedded);
    let mut ast = Ast::new();
    let forms = parse(
        &mut ast,
        "(import std.digits :symbols (fa nz __closed__))\n",
        Some("<probe>"),
    )
    .expect("parses");
    let resolved = resolver
        .resolve_imports(&mut ast, &forms, None)
        .expect("std.digits resolves from the embedded copy");
    let got = by_head(&ast, &resolved);
    let want = expected();
    let mut fa = got["fa"].clone();
    fa.sort();
    assert_eq!(fa, want["fa"], "the full adder's 200 rows");
    let mut nz = got["nz"].clone();
    nz.sort();
    assert_eq!(nz, want["nz"]);
    assert_eq!(
        got["relation"],
        [["fa"], ["nz"]],
        "the two listed declarations"
    );
    assert_eq!(
        got["__closed__"].len(),
        7,
        "every closing fact of the module"
    );
    for other in ["fs", "md", "cl", "mid", "adj"] {
        assert!(
            !got.contains_key(other),
            "`{other}` was not listed and arrived"
        );
    }
}
