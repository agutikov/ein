//! Goal-relevant trace pruning — `ein.py`'s `trace/relevance.py`.
//!
//! The full firing log is the engine's *complete saturation*: for zebra2, 560
//! firings of which 517 merely re-derive facts already present and most of the
//! rest are closure bookkeeping a human never writes down. A human walkthrough
//! is ~20 moves.
//!
//! [`relevant_firings`] recovers that human-scale slice by a **provenance
//! backtrack** from the solution, exactly as one would do by hand: seed with
//! the solved assignment (the goal's relations and the `:hrules` targets),
//! keep only firings on a provenance path to a seed, drop the redundant
//! re-derivations, and flag as *conditional* every firing whose derivation
//! transitively consumes a commitment fact — the rest is the unconditional
//! spine.
//!
//! Every set here is consulted by membership only, so none of their iteration
//! orders reaches the output; the *kept list* keeps the firings' own order,
//! which is what the trace numbers.

use ein_core::{FactId, Kb, Terms};
use ein_infer::firing::Firing;
use ein_infer::query_value;
use ein_ir::{Ast, Node, NodeId};
use rustc_hash::FxHashSet;

/// Relation names the puzzle solves for — the query goal's relations plus its
/// `:hrules` targets. Empty when there is no query.
fn solution_relations(ast: &Ast, kb: &Kb, terms: &Terms) -> FxHashSet<ein_core::Symbol> {
    let mut rels: FxHashSet<ein_core::Symbol> = FxHashSet::default();
    let Some(query) = kb.program().query.as_ref() else {
        return rels;
    };
    let mut names: Vec<String> = Vec::new();
    if let Some(goal) = query_value(ast, query, "goal") {
        collect_goal_relations(ast, goal, &mut names);
    }
    if let Some(hrules) = query_value(ast, query, "hrules")
        && let Node::SForm { args, .. } = ast.node(hrules)
    {
        // `(<activator> (R T1 T2) …)` — each triple's head is a relation.
        for triple in ast.args(args) {
            if let Node::SForm { head, .. } = ast.node(*triple)
                && let Node::Atom(s) = ast.node(head)
            {
                names.push(ast.sym(s).to_string());
            }
        }
    }
    for name in names {
        if let Some(sym) = terms.syms.get(&name) {
            rels.insert(sym);
        }
    }
    rels
}

fn collect_goal_relations(ast: &Ast, form: NodeId, rels: &mut Vec<String>) {
    let Node::SForm { head, args } = ast.node(form) else {
        return;
    };
    let head_name = match ast.node(head) {
        Node::Atom(s) => Some(ast.sym(s).to_string()),
        _ => None,
    };
    match head_name.as_deref() {
        Some("and" | "or" | "not") => {
            for a in ast.args(args).to_vec() {
                collect_goal_relations(ast, a, rels);
            }
        }
        Some(name) => rels.push(name.to_string()),
        None => {}
    }
}

fn seed_keys(ast: &Ast, kb: &Kb, terms: &Terms) -> FxHashSet<FactId> {
    let rels = solution_relations(ast, kb, terms);
    if rels.is_empty() {
        return FxHashSet::default();
    }
    kb.facts()
        .filter(|f| {
            let (rel, args) = terms.fact(*f);
            rels.contains(&rel) && args.len() == 2
        })
        .collect()
}

/// `[(firing, conditional)]` for the goal-relevant slice.
///
/// Firings keep their original order; each derived fact appears once, at its
/// first non-redundant derivation.
pub fn relevant_firings<'a>(
    ast: &Ast,
    terms: &Terms,
    firings: &'a [Firing],
    kb: &Kb,
    commitment: &[FactId],
) -> Vec<(&'a Firing, bool)> {
    let seeds = seed_keys(ast, kb, terms);

    // Backward provenance cone from the seeds over the firing graph. A firing
    // concludes *several* facts, so it is indexed under each: the cone reaches
    // it through any conclusion.
    let mut by_derived: rustc_hash::FxHashMap<FactId, Vec<&Firing>> = Default::default();
    for f in firings {
        for d in f.derived.iter() {
            by_derived.entry(*d).or_default().push(f);
        }
    }
    let mut needed: FxHashSet<FactId> = FxHashSet::default();
    let mut stack: Vec<FactId> = seeds.into_iter().collect();
    while let Some(k) = stack.pop() {
        if !needed.insert(k) {
            continue;
        }
        if let Some(fs) = by_derived.get(&k) {
            for f in fs {
                stack.extend(f.premises.iter().copied());
            }
        }
    }

    // Conditional-fact closure: seeded by the commitment, grown forward.
    let mut conditional: FxHashSet<FactId> = commitment.iter().copied().collect();
    for f in firings {
        if f.premises.iter().any(|p| conditional.contains(p)) {
            for d in f.derived.iter() {
                conditional.insert(*d);
            }
        }
    }

    let mut kept: Vec<(&Firing, bool)> = Vec::new();
    let mut seen: FxHashSet<FactId> = FxHashSet::default();
    for f in firings {
        if f.redundant {
            continue;
        }
        // Keep the firing once if any conclusion is needed and not yet shown.
        let fresh: Vec<FactId> = f
            .derived
            .iter()
            .copied()
            .filter(|d| needed.contains(d) && !seen.contains(d))
            .collect();
        if fresh.is_empty() {
            continue;
        }
        seen.extend(fresh);
        kept.push((f, f.derived.iter().any(|d| conditional.contains(d))));
    }
    kept
}
