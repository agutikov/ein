//! `ein render` — DOT views of rules / constraints / the search lattice.
//!
//! Every body is the same three steps: load, call one renderer, write it with
//! a trailing newline. The renderers themselves are `ein-render`'s
//! ([S1a.5.1](../../../../docs/history/m1a_rust/README.md#s1a51--dot-renderers));
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
    eprintln!("no rule named {} in {file}", ein_core::repr::repr_str(name));
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

/// `ein render program` — the program **as written**, by
/// [`ein_render::ir_to_dot`]: parsed and never loaded, so `examples/broken/`
/// draws too. M1e2 S1e2.1.3, answering Q-M1e.20.
///
/// Imports are not resolved: the stdlib's forms would drown the program's own.
pub fn cmd_program(file: &str, mode: RuleMode, levi: bool) -> i32 {
    let mut ast = Ast::new();
    let Some(forms) = parse_or_exit(&mut ast, Path::new(file)) else {
        return 1;
    };
    let opts = ein_render::DotOpts {
        rule_mode: mode,
        levi,
        ..ein_render::DotOpts::default()
    };
    let dot = ein_render::ir_to_dot(&ast, &forms, opts);
    if dot.trim().is_empty() {
        eprintln!("no renderable forms in {file}");
        return 1;
    }
    emit(&dot);
    0
}

/// `ein render kb` — one KB as one digraph, by [`ein_render::kb_to_dot`]:
/// `file` loaded and saturated. M1e2 S1e2.1.3, answering Q-M1e.20.
///
/// **A model is a file**, not a `--state model:N` here. `ein solve
/// --dump-answer` writes each recorded state as a program that loads and
/// saturates to itself, so `render kb DIR/models/02.ein --since DIR/root.ein`
/// draws model 2 with what its commitment added in bold — from the one solve
/// whose answer the directory holds, rather than from a second solve this
/// command would have to be handed the same flags for.
pub fn cmd_kb(
    file: &str,
    since: Option<&str>,
    colour_by: ein_render::ColourBy,
    include_types: bool,
    include_instances: bool,
) -> i32 {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(kb) = loaded_and_saturated(&mut ast, &mut terms, file) else {
        return 1;
    };
    // One `Terms` for both, so a fact is the same `FactId` in each and the
    // renderer's `since` comparison is by identity.
    let before = match since {
        Some(path) => match loaded_and_saturated(&mut ast, &mut terms, path) {
            Some(kb) => Some(kb),
            None => return 1,
        },
        None => None,
    };
    let opts = ein_render::KbDotOpts {
        colour_by,
        include_types,
        include_instances,
        since: before.as_ref(),
        name: "kb",
    };
    emit(&ein_render::kb_to_dot(&kb, &terms, &opts));
    0
}

fn loaded_and_saturated(ast: &mut Ast, terms: &mut Terms, file: &str) -> Option<ein_core::Kb> {
    let mut kb = load_any_or_exit(ast, terms, Path::new(file))?;
    if let Err(e) = ein_infer::saturate_events(ast, terms, &mut kb) {
        eprintln!("{file}: {e}");
        return None;
    }
    Some(kb)
}
