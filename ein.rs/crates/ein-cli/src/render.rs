//! `ein render` — DOT views of rules / constraints / the search lattice.
//!
//! The Rust half of `ein/cli/render.py`. Every body is the same three steps:
//! load, call one renderer, write it with a trailing newline. The renderers
//! themselves reached byte parity at
//! [S1a.5.1](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.1_dot_renderers.md);
//! what this adds is the surface around them.

use std::path::Path;

use ein_core::Terms;
use ein_ir::Ast;
use ein_render::{
    LatticeSource, LatticeView, RuleMode, render_constraints, render_lattice, render_rule_form,
    render_rules_forms,
};

use crate::common::{load_any_or_exit, parse_or_exit, rule_forms};

/// stdout with the trailing newline every `render` body writes after the DOT.
fn emit(dot: &str) {
    println!("{dot}");
}

pub fn cmd_rules(file: &str, mode: RuleMode) -> i32 {
    let mut ast = Ast::new();
    let Some(forms) = parse_or_exit(&mut ast, Path::new(file)) else {
        return 1;
    };
    let rules = rule_forms(&ast, &forms);
    if rules.is_empty() {
        eprintln!("no rule forms in {file}");
        return 1;
    }
    emit(&render_rules_forms(&ast, &rules, mode));
    0
}

pub fn cmd_rule(file: &str, name: &str, mode: RuleMode) -> i32 {
    let mut ast = Ast::new();
    let Some(forms) = parse_or_exit(&mut ast, Path::new(file)) else {
        return 1;
    };
    for r in rule_forms(&ast, &forms) {
        // The rule's name is its *first argument*, and it has to be an atom —
        // a `(rule "x" …)` is not addressable by `--name`.
        let first = ast.form_args(r).first().copied();
        if first.and_then(|a| ast.atom_name(a)) == Some(name) {
            emit(&render_rule_form(&ast, r, mode));
            return 0;
        }
    }
    eprintln!(
        "no rule named {} in {file}",
        ein_core::pyrepr::repr_str(name)
    );
    1
}

pub fn cmd_constraints(file: &str) -> i32 {
    let mut ast = Ast::new();
    let Some(forms) = parse_or_exit(&mut ast, Path::new(file)) else {
        return 1;
    };
    emit(&render_constraints(&ast, &forms, "constraints"));
    0
}

/// Unlike the static views this *runs the engine*: the lattice DAG comes from
/// `solve`'s `LatticeProof` (`store_lattice`).
pub fn cmd_lattice(file: &str, view: LatticeView, max_set_size: i64) -> i32 {
    use ein_infer::solve::{NoDumper, SolveOptions, solve};

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = load_any_or_exit(&mut ast, &mut terms, Path::new(file)) else {
        return 1;
    };
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: max_set_size.max(0) as u32,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved = match solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let Some(proof) = solved.proof.as_ref() else {
        eprintln!("solve produced no LatticeProof for {file}");
        return 1;
    };
    emit(&render_lattice(
        &terms,
        LatticeSource::Proof(proof),
        view,
        "lattice",
    ));
    0
}
