//! A derivation as steps — one [`TraceStep`] per firing, carrying its
//! premises, the fact it derived, its rendered `:why`, the source sentences it
//! quotes and an inline derivation slice — and an owing state's debts as
//! lines. The region trace ([`super::region`]) tells root's derivation and
//! each state's with them.

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

/// Whether a step carries its drawing, and in which form: the canonical, or
/// folded (`--fold`, Q-M1f.419).
#[derive(Clone, Copy, Debug)]
pub struct Diagrams {
    pub drawn: bool,
    pub fold: bool,
}

#[allow(clippy::too_many_arguments)]
fn step_from_firing(
    terms: &Terms,
    n: u64,
    firing: &Firing,
    kb: Option<&Kb>,
    prov_kb: &Kb,
    diagrams: Diagrams,
    conditional: bool,
) -> TraceStep {
    // `firing.derived` is a tuple: the step is headed by its first
    // conclusion, which names its section, and carries the others in
    // `more` (Q-M1f.214); the per-step slice renders the fan-out.
    // A firing that ended the state derived nothing: its step carries the
    // word its rule gave the engine.
    let derived = firing.derived.first().map(|&d| fact_ref(terms, d));
    let mut step = TraceStep::new(n, terms.sym(firing.rule).to_string(), derived);
    step.ein = firing
        .ends
        .then(|| ein_core::EngineWord::Inconsistent.as_str().to_string());
    step.premises = firing
        .premises
        .iter()
        .map(|p| fact_ref(terms, *p))
        .collect();
    step.premise_statements = firing
        .premises
        .iter()
        .map(|p| kb.and_then(|kb| crate::sentence::statement(kb.program(), terms, *p)))
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
    step.statement = firing
        .derived
        .first()
        .and_then(|&d| kb.and_then(|kb| crate::sentence::statement(kb.program(), terms, d)));
    step.more = firing
        .derived
        .iter()
        .skip(1)
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
    step.section = step.derived.as_ref().and_then(target_entity);
    step.conditional = conditional;
    if diagrams.drawn {
        step.diagram = Some(render_slice(
            terms,
            &[],
            std::slice::from_ref(firing),
            kb,
            &format!("step{n}"),
            None,
            None,
            diagrams.fold,
        ));
    }
    step
}

/// The steps of `firings`, numbered from 1: what a region trace tells for
/// root and for each state.
pub fn steps_of(
    ast: &Ast,
    terms: &Terms,
    firings: &[Firing],
    kb: &Kb,
    diagrams: bool,
    commitment: &[FactId],
) -> Vec<TraceStep> {
    let diagrams = Diagrams {
        drawn: diagrams,
        fold: false,
    };
    steps_of_with(ast, terms, firings, kb, diagrams, commitment, false)
}

/// [`steps_of`], and with `relevant` (`--relevant`) only the goal-relevant
/// slice ([`relevant_firings`]).
pub fn steps_of_with(
    ast: &Ast,
    terms: &Terms,
    firings: &[Firing],
    kb: &Kb,
    diagrams: Diagrams,
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
    diagrams: Diagrams,
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
