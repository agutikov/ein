//! `ein solve --print-final-*` — a solution KB as canonical s-expressions.
//!
//! A fact is *not* rendered here: the event protocol's fact renderer,
//! `ein_infer::events::sexpr`, is reused rather than duplicated.

use ein_core::{Kb, Symbol, Terms};
use ein_infer::events::sexpr;
use ein_ir::{Ast, Node, NodeId};

/// Relations the `(solve …)` `:hrules` clause targets — the hypothesis commitments.
///
/// Generic: the atoms named in the `:hrules` activators that are *declared
/// relations*, so type/object atoms drop out position-independently. Every
/// `:hrules` pair contributes, not just the first.
pub fn hypothesis_target_relations(ast: &Ast, terms: &Terms, kb: &Kb) -> Vec<Symbol> {
    let Some(solve) = kb.program().solve() else {
        return Vec::new();
    };
    let mut names: Vec<String> = Vec::new();
    for &pair in solve.kw_pairs.iter() {
        let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
            continue;
        };
        let Node::Keyword(name) = ast.node(key) else {
            continue;
        };
        if ast.sym(name) == "hrules" {
            collect_atoms(ast, value, &mut names);
        }
    }
    // The named atoms that are declared relations, in *registry* order,
    // because the caller only ever sorts it or tests membership.
    let mut out: Vec<Symbol> = Vec::new();
    for (sym, _) in kb.program().relations.iter() {
        if names.iter().any(|n| n == terms.sym(sym)) {
            out.push(sym);
        }
    }
    out
}

/// Every atom name in a node: an `Atom`'s own, an `SForm`'s head
/// when it is an atom, and recursively its arguments. Nothing else.
fn collect_atoms(ast: &Ast, node: NodeId, out: &mut Vec<String>) {
    match ast.node(node) {
        Node::Atom(name) => out.push(ast.sym(name).to_string()),
        Node::SForm { head, args } => {
            if let Some(name) = ast.atom_name(head) {
                out.push(name.to_string());
            }
            for &a in ast.args(args) {
                collect_atoms(ast, a, out);
            }
        }
        _ => {}
    }
}

/// The three `--print-final-*` modes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `-p` — everything the engine derived: the propositional residue.
    All,
    /// `-P` — `All` with the `(not …)` facts dropped: the positive residue.
    Positive,
    /// `-f` — only the `(solve …)` `:hrules` target facts, *whatever their origin*,
    /// so the given conditions count too.
    Hfacts,
}

/// Dump a slice of a solution kb's facts in canonical order.
pub fn print_final_state(terms: &Terms, kb: &Kb, mode: Mode, targets: &[Symbol]) {
    let (mut facts, label): (Vec<_>, String) = match mode {
        Mode::Hfacts => {
            let picked: Vec<_> = kb
                .facts()
                .filter(|&f| targets.contains(&terms.facts.get(f).0))
                .collect();
            let mut names: Vec<&str> = targets.iter().map(|&s| terms.sym(s)).collect();
            names.sort_unstable();
            let shown: Vec<String> = names.iter().map(|n| ein_core::repr::repr_str(n)).collect();
            (
                picked,
                format!(
                    "positive hypothesis-relation facts; :hrules [{}]",
                    shown.join(", ")
                ),
            )
        }
        _ => {
            let not = terms.syms.get("not");
            let picked: Vec<_> = kb
                .facts()
                .filter(|&f| is_derived(kb, terms, f))
                .filter(|&f| !(mode == Mode::Positive && Some(terms.facts.get(f).0) == not))
                .collect();
            (
                picked,
                if mode == Mode::Positive {
                    "derived facts, (not …) omitted".to_string()
                } else {
                    "derived facts".to_string()
                },
            )
        }
    };

    // Keyed on the relation name, then the args *rendered* rather than the
    // raw ids, so the order is the one a reader sees.
    let mut keyed: Vec<(&str, Vec<String>, ein_core::FactId)> = facts
        .drain(..)
        .map(|f| {
            let (rel, args) = terms.facts.get(f);
            let rendered = args
                .iter()
                .map(|a| ein_infer::events::sexpr_value(terms, *a))
                .collect();
            (terms.sym(rel), rendered, f)
        })
        .collect();
    keyed.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));

    println!("final-state facts ({label}; {} facts):", keyed.len());
    for (rel, args, _) in &keyed {
        // Note the unconditional separator: a nullary fact prints `(rel )`
        // here, where `sexpr` would spell it `(rel)`. This line has its own
        // join, and the difference is observable.
        println!("  ({rel} {})", args.join(" "));
    }
}

/// Derived — produced by the engine, i.e. it has provenance and that
/// provenance is not a `:source` ingestion. A fact with none is neither given
/// nor derived, so it drops out of every `--print-final-*` mode but `hfacts`.
fn is_derived(kb: &Kb, terms: &Terms, fact: ein_core::FactId) -> bool {
    match kb.primary(fact) {
        Some(p) => terms.provs.get(p).kind != ein_core::ProvKind::Source,
        None => false,
    }
}

/// The unsat-core dump `--print-final-*` prints when there is no model.
pub fn print_unsat_core(terms: &Terms, core: &[ein_core::FactId]) {
    let mut keyed: Vec<(&str, Vec<String>, String)> = core
        .iter()
        .map(|&f| {
            let (rel, args) = terms.facts.get(f);
            let rendered: Vec<String> = args
                .iter()
                .map(|a| ein_infer::events::sexpr_value(terms, *a))
                .collect();
            (terms.sym(rel), rendered, sexpr(terms, f))
        })
        .collect();
    keyed.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    println!();
    println!("unsat-core facts ({} facts):", keyed.len());
    for (_, _, text) in &keyed {
        println!("  {text}");
    }
}
