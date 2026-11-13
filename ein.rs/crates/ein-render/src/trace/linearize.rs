//! A derivation as steps — one [`TraceStep`] per firing, carrying its
//! premises, the fact it derived, its rendered `:why`, the source sentences it
//! quotes and an inline derivation slice — and an owing state's debts as
//! lines. The region trace ([`super::region`]) tells root's derivation and
//! each state's with them.
//!
//! The walk's linearizer, which turned one solve's lattice proof into a story
//! with reductios and a closing lattice DAG, went with the walk at M1e2
//! S1e2.1c.5 (2026-11-03); the module keeps its name.

use ein_core::{FactId, Kb, ProvKind, Terms};
use ein_infer::firing::Firing;
use ein_infer::obligations::Owes;
use ein_ir::Ast;

use super::ast::{FactRef, TraceStep, fact_ref};
use super::relevance::relevant_firings;
use crate::slice::render_slice;

/// One outstanding obligation as a trace line: its `:why`, or — for a rule
/// that declares none — the rule and its bindings, so a debt is never a blank
/// bullet.
pub fn owe_lines(terms: &Terms, owes: &Owes) -> Vec<String> {
    owes.instances()
        .iter()
        .map(|i| {
            if !i.why.is_empty() {
                return i.why.clone();
            }
            let b: Vec<String> = i
                .bindings
                .iter()
                .map(|(k, v)| format!("?{}={}", terms.sym(*k), terms.display(*v)))
                .collect();
            format!("{} ({})", terms.sym(i.rule), b.join(" "))
        })
        .collect()
}

/// The node a derived fact is "about" — its first string argument.
fn target_entity(derived: &FactRef) -> Option<String> {
    derived.args.iter().find_map(|a| match a {
        super::ast::RefArg::Str(s) => Some(s.clone()),
        _ => None,
    })
}

/// A fact's `:source` sentence, or `None` — the provenance's raw `source`
/// field, read only when its kind is `Source`.
fn source_of<'a>(kb: &Kb, terms: &'a Terms, f: FactId) -> Option<&'a str> {
    let prov = terms.provs.get(kb.primary(f)?);
    if prov.kind != ProvKind::Source {
        return None;
    }
    // An empty sentence is dropped.
    prov.source.map(|s| terms.sym(s)).filter(|s| !s.is_empty())
}

#[allow(clippy::too_many_arguments)]
fn step_from_firing(
    terms: &Terms,
    n: u64,
    firing: &Firing,
    kb: Option<&Kb>,
    prov_kb: &Kb,
    diagrams: bool,
    conditional: bool,
) -> TraceStep {
    // `firing.derived` is a tuple: the step is headed by its first
    // conclusion, which names its section, and carries the others in
    // `more` (M1f S1f.6.25, E-5); the per-step slice renders the fan-out.
    let derived = fact_ref(terms, firing.derived[0]);
    let mut step = TraceStep::new(n, terms.sym(firing.rule).to_string(), derived);
    step.premises = firing
        .premises
        .iter()
        .map(|p| fact_ref(terms, *p))
        .collect();
    step.bindings = firing
        .bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
        .collect();
    step.why = kb
        .and_then(|kb| {
            crate::sentence::rule_sentence(kb.program(), terms, firing.rule, &firing.bindings)
        })
        .unwrap_or_default();
    step.statement =
        kb.and_then(|kb| crate::sentence::statement(kb.program(), terms, firing.derived[0]));
    step.more = firing.derived[1..]
        .iter()
        .map(|d| {
            let says = kb.and_then(|kb| crate::sentence::statement(kb.program(), terms, *d));
            (fact_ref(terms, *d), says)
        })
        .collect();
    step.sources = firing
        .premises
        .iter()
        .filter_map(|p| source_of(prov_kb, terms, *p).map(str::to_string))
        .collect();
    step.section = target_entity(&step.derived);
    step.conditional = conditional;
    if diagrams {
        step.diagram = Some(render_slice(
            terms,
            &[],
            std::slice::from_ref(firing),
            kb,
            &format!("step{n}"),
            None,
            None,
        ));
    }
    step
}

/// The steps of `firings`, numbered from 1, as the walk's trace builds them:
/// what a region trace tells for root and for each state.
pub fn steps_of(
    ast: &Ast,
    terms: &Terms,
    firings: &[Firing],
    kb: &Kb,
    diagrams: bool,
    commitment: &[FactId],
) -> Vec<TraceStep> {
    steps_of_with(ast, terms, firings, kb, diagrams, commitment, false)
}

/// [`steps_of`], and with `relevant` (`--relevant`) only the goal-relevant
/// slice, as the walk's trace prunes it.
pub fn steps_of_with(
    ast: &Ast,
    terms: &Terms,
    firings: &[Firing],
    kb: &Kb,
    diagrams: bool,
    commitment: &[FactId],
    relevant: bool,
) -> Vec<TraceStep> {
    build_steps(
        ast,
        terms,
        firings,
        Some(kb),
        kb,
        diagrams,
        relevant,
        commitment,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_steps(
    ast: &Ast,
    terms: &Terms,
    firings: &[Firing],
    kb: Option<&Kb>,
    prov_kb: &Kb,
    diagrams: bool,
    relevant: bool,
    commitment: &[FactId],
) -> Vec<TraceStep> {
    if relevant {
        // `relevant_firings` needs a KB to read the query goal from; with no
        // spine KB there is no goal, so the seed set is empty and nothing is
        // kept.
        let Some(kb) = kb else { return Vec::new() };
        let kept = relevant_firings(ast, terms, firings, kb, commitment);
        return kept
            .into_iter()
            .enumerate()
            .map(|(i, (f, cond))| {
                step_from_firing(terms, i as u64 + 1, f, Some(kb), prov_kb, diagrams, cond)
            })
            .collect();
    }
    firings
        .iter()
        .enumerate()
        .map(|(i, f)| step_from_firing(terms, i as u64 + 1, f, kb, prov_kb, diagrams, false))
        .collect()
}
