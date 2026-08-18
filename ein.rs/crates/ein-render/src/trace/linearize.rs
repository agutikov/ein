//! Linearise a lattice solve into a depth-ordered story — `ein.py`'s
//! `trace/linearize.py`.
//!
//! The engine emits an *unordered* commitment lattice; the narrative needs a
//! linear sequence. [`linearize`] turns a solve result into a [`Trace`]:
//!
//! - **the spine** — the firings of the primary solution commitment (smallest
//!   commitment first; the empty one is the unconditional root saturation),
//!   each a [`TraceStep`] carrying its premises, derived fact, rendered
//!   `:why`, quoted source sentences and an inline derivation slice;
//! - **reductios** — one per refuted commitment: "Suppose X. Then ⊥," closed
//!   by its lifted no-good;
//! - the closing lattice DAG and solution grid.
//!
//! **One shape departure.** ein.py reads a fact's provenance off the `Fact`
//! object, which travels with it out of a fork that no longer exists. Here
//! provenance lives in the KB, so `linearize` takes a `root` to resolve
//! against when there is no spine KB — the `Contradiction`-without-a-solution
//! case. It cannot change what is printed: the only provenance field read is
//! `:source`, which none but a load-time fact carries, and a load-time fact is
//! in every fork's layer stack.

use ein_core::{FactId, Kb, ProvKind, Terms};
use ein_infer::apriori::cmp_set;
use ein_infer::firing::Firing;
use ein_infer::solve::{DeadCommitment, LatticeProof, Solved};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::Ast;

use super::ast::{FactRef, TraceStep, fact_ref};
use super::relevance::relevant_firings;
use crate::dot_util::fact_label;
use crate::lattice_dag::{LatticeSource, LatticeView, render_lattice};
use crate::slice::{render_slice, render_solution, render_state};
use crate::why::render_why;

/// A refuted hypothesis — rendered as a foldable `<details>`.
pub struct Reductio {
    /// One line: what was assumed, and that it failed.
    pub summary: String,
    /// The assumed commitment, rendered.
    pub commitment: String,
    /// The lifted no-good.
    pub learned_clause: String,
    /// The contradiction slice, terminating in `⊥`.
    pub diagram: Option<String>,
}

/// A linearised solve, ready for markdown rendering.
#[derive(Default)]
pub struct Trace {
    pub steps: Vec<TraceStep>,
    pub reductios: Vec<Reductio>,
    pub summary: String,
    /// The primary solution's assumed hypotheses.
    pub commitment: String,
    pub solved: bool,
    pub n_solutions: usize,
    pub lattice_dot: Option<String>,
    pub solution_dot: Option<String>,
    pub full_kb_dot: Option<String>,
}

#[derive(Clone, Copy, Default)]
pub struct LinearizeOpts {
    pub diagrams: bool,
    pub full_kb_snapshots: bool,
    pub relevant: bool,
}

impl LinearizeOpts {
    /// ein.py's defaults: `diagrams=True`, the other two off.
    pub fn new() -> LinearizeOpts {
        LinearizeOpts {
            diagrams: true,
            full_kb_snapshots: false,
            relevant: false,
        }
    }
}

fn commitment_label(terms: &Terms, commitment: &[FactId]) -> String {
    if commitment.is_empty() {
        return "∅ (unconditional)".to_string();
    }
    let parts: Vec<String> = commitment.iter().map(|f| fact_label(terms, *f)).collect();
    format!("{{{}}}", parts.join(", "))
}

/// The node a derived fact is "about" — its first string argument.
fn target_entity(derived: &FactRef) -> Option<String> {
    derived.args.iter().find_map(|a| match a {
        super::ast::RefArg::Str(s) => Some(s.clone()),
        _ => None,
    })
}

/// A fact's `:source` sentence, or `None` — `Fact.source`, which is the raw
/// field guarded by `kind == "source"`.
fn source_of<'a>(kb: &Kb, terms: &'a Terms, f: FactId) -> Option<&'a str> {
    let prov = terms.provs.get(kb.primary(f)?);
    if prov.kind != ProvKind::Source {
        return None;
    }
    // `if p.source` — Python's truthiness, so an empty sentence is dropped.
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
    // `firing.derived` is a tuple; the linear step records the *primary*
    // conclusion, and the per-step slice renders the full fan-out.
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
        .and_then(|kb| kb.program().rules.get(firing.rule))
        .and_then(|r| r.why)
        .map(|w| terms.sym(w).to_string())
        .filter(|w| !w.is_empty())
        .map(|template| {
            let bindings: Vec<(String, String)> = firing
                .bindings
                .iter()
                .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
                .collect();
            render_why(&template, &bindings)
        })
        .unwrap_or_default();
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
        // kept — which is what ein.py's `getattr(kb, "query", None)` does.
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

fn reductio(
    terms: &Terms,
    dc: &DeadCommitment,
    kb: Option<&Kb>,
    prov_kb: &Kb,
    diagrams: bool,
) -> Reductio {
    let commitment = commitment_label(terms, &dc.commitment);
    let mut clause: Vec<String> = dc
        .learned_clause
        .iter()
        .map(|f| fact_label(terms, *f))
        .collect();
    clause.sort();
    let mut core_sources: Vec<String> = dc
        .unsat_core
        .iter()
        .filter_map(|f| source_of(prov_kb, terms, *f).map(str::to_string))
        .collect();
    core_sources.sort();
    core_sources.dedup();
    let contradicts = if core_sources.is_empty() {
        String::new()
    } else {
        format!(" — contradicts {}", core_sources.join(", "))
    };
    let diagram = diagrams.then(|| {
        render_slice(
            terms,
            &dc.commitment,
            &[],
            kb,
            "reductio",
            Some((&dc.unsat_core, &dc.learned_clause)),
            None,
        )
    });
    Reductio {
        summary: format!(
            "Assumed {commitment}{contradicts} — refuted ({})",
            dc.kind.as_str()
        ),
        commitment,
        learned_clause: clause.join(", "),
        diagram,
    }
}

/// Build a [`Trace`] from a solve result.
pub fn linearize(
    ast: &Ast,
    terms: &Terms,
    root: &Kb,
    solved: &Solved,
    opts: LinearizeOpts,
) -> Trace {
    let proof: Option<&LatticeProof> = solved.proof.as_ref();

    // ── A monotonic Solution with no proof: the trace *is* solution.trace. ──
    if let (Answer::Verdict(Verdict::Solution(s)), None) = (&solved.answer, proof) {
        let steps = build_steps(
            ast,
            terms,
            &s.trace,
            Some(&s.kb),
            &s.kb,
            opts.diagrams,
            opts.relevant,
            &[],
        );
        let kept = if opts.relevant {
            format!(" ({} of {} relevant)", steps.len(), s.trace.len())
        } else {
            String::new()
        };
        return Trace {
            summary: format!("Solved in {} steps (unconditional){kept}.", steps.len()),
            steps,
            commitment: "∅ (unconditional)".to_string(),
            solved: true,
            n_solutions: 1,
            solution_dot: opts
                .diagrams
                .then(|| render_solution(&s.kb, terms, "solution")),
            full_kb_dot: opts
                .full_kb_snapshots
                .then(|| render_state(&s.kb, terms, None, "state")),
            ..Trace::default()
        };
    }

    // ── Ambiguity / Contradiction with no proof. ──
    if let (Answer::Verdict(Verdict::Ambiguity(branches)), None) = (&solved.answer, proof) {
        let first = branches.first();
        let kb = first.map(|f| &f.kb);
        let steps = match first {
            Some(f) => build_steps(
                ast,
                terms,
                &f.trace,
                Some(&f.kb),
                &f.kb,
                opts.diagrams,
                opts.relevant,
                &[],
            ),
            None => Vec::new(),
        };
        return Trace {
            steps,
            summary: format!("Ambiguous — {} models (showing one).", branches.len()),
            commitment: "∅ (unconditional)".to_string(),
            solved: false,
            n_solutions: branches.len(),
            solution_dot: match (opts.diagrams, kb) {
                (true, Some(kb)) => Some(render_solution(kb, terms, "solution")),
                _ => None,
            },
            ..Trace::default()
        };
    }

    if let (Answer::Verdict(Verdict::Contradiction { unsat_core }), None) = (&solved.answer, proof)
    {
        let mut core: Vec<String> = unsat_core
            .iter()
            .filter_map(|f| source_of(root, terms, *f).map(str::to_string))
            .collect();
        core.sort();
        core.dedup();
        let label = if core.is_empty() {
            format!("{} facts", unsat_core.len())
        } else {
            core.join(", ")
        };
        return Trace {
            summary: format!("Contradiction — no model; unsat core: {label}."),
            commitment: "—".to_string(),
            solved: false,
            n_solutions: 0,
            ..Trace::default()
        };
    }

    // ── The proof path. ──
    let solutions: &[_] = proof.map_or(&[], |p| &p.solutions);
    let deads: &[_] = proof.map_or(&[], |p| &p.dead_commitments);

    // Primary solution = the smallest commitment; the empty one sorts first.
    let primary = solutions.iter().min_by(|a, b| {
        (a.commitment.len())
            .cmp(&b.commitment.len())
            .then_with(|| cmp_set(terms, &a.commitment, &b.commitment))
    });
    let spine_kb = primary.map(|p| &p.kb);
    let prov_kb = spine_kb.unwrap_or(root);

    let mut steps: Vec<TraceStep> = Vec::new();
    let mut n_firings = 0usize;
    if let Some(p) = primary {
        n_firings = p.firings.len();
        steps = build_steps(
            ast,
            terms,
            &p.firings,
            spine_kb,
            prov_kb,
            opts.diagrams,
            opts.relevant,
            &p.commitment,
        );
    }

    let reductios: Vec<Reductio> = deads
        .iter()
        .map(|dc| reductio(terms, dc, spine_kb, prov_kb, opts.diagrams))
        .collect();

    let solved_flag = primary.is_some();
    let commitment = match primary {
        Some(p) => commitment_label(terms, &p.commitment),
        None => "—".to_string(),
    };
    let pruned = if opts.relevant && solved_flag {
        format!(" (pruned to {} of {n_firings} firings)", steps.len())
    } else {
        String::new()
    };
    let summary = if solved_flag {
        format!(
            "Solved in {} steps; commitment {commitment}; {} solution(s), {} refuted{pruned}.",
            steps.len(),
            solutions.len(),
            reductios.len()
        )
    } else {
        format!(
            "No solution — {} commitments refuted ({} dead).",
            reductios.len(),
            deads.len()
        )
    };

    // The solution sub-DAG when there is a survivor; the full lattice for unsat.
    let lattice_dot = match (opts.diagrams, proof) {
        (true, Some(p)) => Some(render_lattice(
            terms,
            LatticeSource::Proof(p),
            if solved_flag {
                LatticeView::Solution
            } else {
                LatticeView::Full
            },
            "lattice",
        )),
        _ => None,
    };

    Trace {
        steps,
        reductios,
        summary,
        commitment,
        solved: solved_flag,
        n_solutions: solutions.len(),
        lattice_dot,
        solution_dot: match (opts.diagrams, spine_kb) {
            (true, Some(kb)) => Some(render_solution(kb, terms, "solution")),
            _ => None,
        },
        full_kb_dot: match (opts.full_kb_snapshots, spine_kb) {
            (true, Some(kb)) => Some(render_state(kb, terms, None, "state")),
            _ => None,
        },
    }
}
