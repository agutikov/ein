//! The conclusion census — M1f
//! [S1f.6.37](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.37_conclusion_shapes.md):
//! every rule's, hrule's and obligation's `:assert` in the corpus, **after
//! macro expansion**, by shape.
//!
//! A conclusion is `()`, an obligation `(open …)`, one conjunct, or an
//! `(and …)` of conjuncts; a conjunct is a fact (a pattern), a stored
//! negation `(not …)`, `(false)` or a check (`eq`, `neq`, `(not (eq …))`).
//! The census counts each shape and lists every conclusion that writes an
//! `and` inside an `and` (E-63), which the loader reads flattened at any depth
//! since S1f.6.37 ([`ein_ir::Ast::conjuncts`]).
//!
//! ```sh
//! cargo run --release -p ein-infer --example conclusion_census            # corpus_files() and the register's witnesses
//! cargo run --release -p ein-infer --example conclusion_census -- FILE…
//! cargo run --release -p ein-infer --example conclusion_census -- --unloaded   # + each file that does not load
//! ```
//!
//! A rule a module declares is loaded with every program that imports it, so
//! a rule is counted once by its full name and its conclusion's text. A file
//! that does not parse or load (the negative fixtures) is counted apart. It
//! writes nothing, and no route reads it.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ein_ir::{Ast, Node, NodeId};

/// A conjunct's kind.
fn conjunct_kind(ast: &Ast, node: NodeId) -> &'static str {
    match ast.head_name(node) {
        _ if !matches!(ast.node(node), Node::SForm { .. }) => "no form",
        Some("and") => "and",
        Some("false") => "(false)",
        Some("eq" | "neq") => "check",
        Some("not") => match ast.form_args(node).first() {
            Some(&p) if matches!(ast.head_name(p), Some("eq" | "neq")) => "check",
            _ => "stored negation",
        },
        Some("open") => "open",
        _ => "fact",
    }
}

/// How deep `and` nests under `node`: 0 for no `and`, 1 for a flat one.
fn and_depth(ast: &Ast, node: NodeId) -> usize {
    if ast.head_name(node) != Some("and") {
        return 0;
    }
    1 + ast
        .form_args(node)
        .iter()
        .map(|&c| and_depth(ast, c))
        .max()
        .unwrap_or(0)
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let list_unloaded = args.iter().any(|a| a == "--unloaded");
    args.retain(|a| a != "--unloaded");
    let files: Vec<PathBuf> = if args.is_empty() {
        census::selection()
    } else {
        args.iter().map(PathBuf::from).collect()
    };
    // (which, full name, conclusion as text) → (shape, nested depth, first file)
    let mut seen: BTreeMap<(String, String, String), (String, usize, String)> = BTreeMap::new();
    let (mut programs, mut unloaded) = (0usize, 0usize);
    for path in &files {
        let Some((kb, terms, ast)) = census::loaded(path) else {
            unloaded += 1;
            if list_unloaded {
                println!("not loaded: {}", census::relative(path));
            }
            continue;
        };
        programs += 1;
        for (which, reg) in [
            ("rule", &kb.program().rules),
            ("hrule", &kb.program().hrules),
            ("obligation", &kb.program().obligations),
        ] {
            for (name, rule) in reg.iter() {
                let Some(a) = rule.assert_.as_ref() else {
                    continue;
                };
                let node = NodeId(a.expr.0);
                let text = ein_ir::dump_compact(&ast, node);
                let key = (which.to_string(), terms.sym(name).to_string(), text);
                if seen.contains_key(&key) {
                    continue;
                }
                let empty = ast.head_name(node) == Some(ein_core::text::EMPTY_HEAD)
                    && ast.form_args(node).is_empty();
                let shape = if empty {
                    "()".to_string()
                } else if ast.head_name(node) == Some("open") {
                    "(open …)".to_string()
                } else {
                    let kinds: BTreeSet<&str> = ast
                        .conjuncts(node)
                        .into_iter()
                        .map(|c| conjunct_kind(&ast, c))
                        .collect();
                    let kinds: Vec<&str> = kinds.into_iter().collect();
                    if ast.head_name(node) == Some("and") {
                        format!(
                            "(and …) of {} conjuncts: {}",
                            ast.conjuncts(node).len(),
                            kinds.join(", ")
                        )
                    } else {
                        format!("one conjunct: {}", kinds.join(", "))
                    }
                };
                let depth = and_depth(&ast, node);
                seen.insert(key, (shape, depth, census::relative(path)));
            }
        }
    }
    println!(
        "{} files: {programs} loaded, {unloaded} not (negative fixtures); {} distinct conclusions",
        files.len(),
        seen.len()
    );
    // which → shape (the and's conjunct count bucketed) → count
    let mut table: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut by_depth: BTreeMap<(String, usize), usize> = BTreeMap::new();
    for ((which, _, _), (shape, depth, _)) in &seen {
        let shape = match shape.split_once(" of ") {
            Some((head, rest)) => {
                let (n, kinds) = rest.split_once(" conjuncts: ").unwrap_or((rest, ""));
                let n: usize = n.parse().unwrap_or(0);
                let bucket = if n <= 4 {
                    n.to_string()
                } else {
                    "5+".to_string()
                };
                format!("{head} of {bucket} conjuncts: {kinds}")
            }
            None => shape.clone(),
        };
        *table.entry((which.clone(), shape)).or_default() += 1;
        *by_depth.entry((which.clone(), *depth)).or_default() += 1;
    }
    println!("\nby shape (after expansion):");
    for ((which, shape), n) in &table {
        println!("  {which:<10} {n:>6}  {shape}");
    }
    println!("\nby the depth `and` nests to (0: no and, 1: a flat and, 2+: an and inside an and):");
    for ((which, depth), n) in &by_depth {
        println!("  {which:<10} {n:>6}  depth {depth}");
    }
    let nested: Vec<_> = seen.iter().filter(|(_, (_, d, _))| *d >= 2).collect();
    println!(
        "\nconclusions that write an and inside an and: {}",
        nested.len()
    );
    for ((which, name, text), (_, depth, file)) in nested {
        println!("  {which} {name} (depth {depth}), {file}: {text}");
    }
}
