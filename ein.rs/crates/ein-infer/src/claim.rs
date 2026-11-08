//! A `(solve … :expect …)` claim, checked against R — M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
//!
//! The shape is [`ein_ir::claim`]'s and the loader has validated it. This is
//! the comparison, and it never runs the walk: R comes from route 1 on a
//! monotone program and from the cube on a flagged one, the two routes
//! `ein solve` describes R by ([`crate::explore`]). Each state's facts are its
//! fixpoint's, root's included, read by the oracle's own node.
//!
//! The rules (`region_examples.md` § 1, accepted 2026-10-16):
//!
//! - every relation the claim names positively is closed, across the whole
//!   claim, and a listed `(not …)` is checked for presence;
//! - the `(model …)`s are R's states, one to one;
//! - `(merged …)` is all or nothing: with one, every state reached by more
//!   than one commitment is listed, with exactly its minimal commitments;
//! - `(owing …)` says R is empty and lists the alive, owing nodes with no
//!   alive node one fact up;
//! - `(false)` says every node is dead, and `(region)` that R is empty.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use ein_ir::claim::{Claim, Region as Stated};
use ein_ir::expect::Model;

use crate::described::{self, Described};
use crate::error::SolveError;
use crate::events::sexpr;
use crate::expect::{Outcome, Report};
use crate::explore::Route;
use crate::region::{self, Axes, fan_out, pool_of, prepare};
use crate::{downset, region::Candidate};

/// How a claim is checked: the budgets of the two routes.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Route 1's, in facts beyond root.
    pub budget: u64,
    /// The largest `n` the cube is computed for, on a flagged program.
    pub limit: u32,
    pub jobs: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            budget: downset::DEFAULT_BUDGET,
            limit: region::DEFAULT_LIMIT,
            jobs: 1,
        }
    }
}

/// What the check found, with what it ran.
#[derive(Debug)]
pub struct Checked {
    pub report: Report,
    /// `route 1` or `route 2`; `None` when no route described R.
    pub route: Option<&'static str>,
    pub monotone: bool,
    pub k_s: u64,
    pub k_c: u64,
}

/// One state of R: a commitment that reaches it, how many commitments of R
/// do, and the minimal ones.
struct State {
    rep: Vec<u32>,
    commitments: u64,
    generators: Vec<Vec<u32>>,
}

/// R, as the claim needs it.
struct Lattice {
    candidates: Vec<Candidate>,
    route: &'static str,
    monotone: bool,
    any_alive: bool,
    states: Vec<State>,
    /// Alive, owing, and no alive node one fact up.
    frontier: Vec<Vec<u32>>,
}

/// Check `claim` against the R of `root`, a **freshly loaded, unsaturated**
/// program, as [`crate::explore::run`] asks.
pub fn check(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    claim: &Claim,
    opts: &Options,
) -> Result<Checked, SolveError> {
    let lattice = match lattice(root, terms, ast, opts)? {
        Ok(l) => l,
        Err((why, monotone)) => {
            return Ok(Checked {
                report: Report {
                    outcome: Outcome::NotChecked,
                    lines: vec![why],
                },
                route: None,
                monotone,
                k_s: 0,
                k_c: 0,
            });
        }
    };
    let k_s = lattice.states.len() as u64;
    let k_c = lattice.states.iter().map(|s| s.commitments).sum();
    let report = compare(root, terms, ast, claim, &lattice, opts)?;
    Ok(Checked {
        report,
        route: Some(lattice.route),
        monotone: lattice.monotone,
        k_s,
        k_c,
    })
}

/// Check `claim` against `d`, an exhausted description of `root`'s R that a
/// run already computed — what `ein solve` does, so that the claim and the
/// read-out are about one R and the program is solved once.
pub fn check_described(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    claim: &Claim,
    d: &Described,
    opts: &Options,
) -> Result<Report, SolveError> {
    debug_assert!(d.exhausted, "a claim is checked against all of R");
    compare(root, terms, ast, claim, &of_described(d.clone()), opts)
}

/// R, by route 1 on a monotone program and by the cube otherwise — or why no
/// route described it, with the static verdict.
fn lattice(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<Result<Lattice, (String, bool)>, SolveError> {
    let t0 = Instant::now();
    let prepared = prepare(root, terms, ast)?;
    let monotone = prepared.monotonicity.is_monotone();
    let n = prepared.candidates.len();
    // Describe's route, as `ein solve` takes it (`explore`'s § Describe's
    // route): the state route on a monotone program, route 1 on a flagged one
    // whose root is dead (`explore::Licence::ForkOnlyAdds`), the cube else.
    // A claim states no core, so the run takes none but root's (M1e2
    // S1e2.2.21, *minimum work per question*).
    let route_opts = downset::Options {
        budget: opts.budget,
        max_visits: u64::MAX,
        jobs: opts.jobs,
        deadline: None,
        cores: false,
    };
    if monotone {
        let s = crate::states::compute_prepared(
            root,
            terms,
            ast,
            prepared,
            t0,
            &route_opts,
            &mut crate::events::Events::off(),
        )?;
        match &s.declined {
            None => {
                let d = crate::states::describe(&s)?;
                return Ok(Ok(of_described(described::of_states(&s, &d))));
            }
            Some(downset::Declined::Budget { layer, .. }) => {
                return Ok(Err((
                    format!(
                        "R was not described: the state route stopped at its budget of {} \
                         facts, before layer {layer}",
                        opts.budget
                    ),
                    true,
                )));
            }
            // A node contradicted the static verdict: take the cube.
            Some(_) => {}
        }
    } else if prepared.root_dead {
        let d = downset::compute_prepared(
            root,
            terms,
            ast,
            prepared,
            t0,
            &route_opts,
            &mut crate::events::Events::off(),
        )?;
        if d.declined.is_none() {
            return Ok(Ok(of_described(described::of_downset(&d))));
        }
    }
    if n > opts.limit as usize {
        return Ok(Err((
            format!(
                "R was not described: the program is not monotone and n = {n} is past \
                 the oracle's limit of {}",
                opts.limit
            ),
            monotone,
        )));
    }
    // A fresh preparation: the one above went to a route, or was never used.
    let prepared = prepare(root, terms, ast)?;
    let outcome = region::compute_prepared(
        root,
        terms,
        ast,
        prepared,
        t0,
        &region::Options {
            limit: opts.limit,
            keep_states: false,
            jobs: opts.jobs,
            deadline: None,
            cores: false,
        },
        &mut |_| {},
        &mut crate::events::Events::off(),
    )?;
    Ok(match outcome {
        region::Outcome::Region(r) => Ok(of_described(described::of_cube(&r))),
        region::Outcome::Refused { n, limit, .. } => Err((
            format!("R was not described: n = {n} is past the oracle's limit of {limit}"),
            monotone,
        )),
    })
}

/// R, printed as the claim that states it — the form `ein solve` prints and a
/// program reads back (ruling 1 of 2026-10-16: one form, both directions). It
/// names the candidates' relations, so every state is told apart. `None` when
/// no route described R.
///
/// `(false)` when no node is alive; `(region (owing …))` when R is empty and
/// alive nodes owe; otherwise every state, then a `(merged …)` for every state
/// more than one commitment reaches.
pub fn print(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<Option<String>, SolveError> {
    let Ok(l) = lattice(root, terms, ast, opts)? else {
        return Ok(None);
    };
    if !l.any_alive {
        return Ok(Some("(false)".to_string()));
    }
    let named: BTreeSet<String> = l
        .candidates
        .iter()
        .map(|c| relation_of(&c.text).to_string())
        .collect();
    let model = |s: &Seen| describe(s, &named);
    if l.states.is_empty() {
        let frontier = fixpoints(root, terms, ast, &l.candidates, &l.frontier, opts)?;
        let mut items: Vec<String> = frontier.iter().map(model).collect();
        items.sort();
        return Ok(Some(format!("(region (owing {}))", items.join(" "))));
    }
    let reps: Vec<Vec<u32>> = l.states.iter().map(|s| s.rep.clone()).collect();
    let states = fixpoints(root, terms, ast, &l.candidates, &reps, opts)?;
    let mut items: Vec<String> = states.iter().map(model).collect();
    items.sort();
    let mut merged: Vec<String> = Vec::new();
    for (st, seen) in l.states.iter().zip(&states) {
        if st.commitments < 2 {
            continue;
        }
        let mut gens: Vec<String> = st
            .generators
            .iter()
            .map(|g| {
                let mut facts: Vec<String> = g
                    .iter()
                    .map(|&b| l.candidates[b as usize].text.clone())
                    .collect();
                facts.sort();
                if facts.is_empty() {
                    "(model)".to_string()
                } else {
                    format!("(model {})", facts.join(" "))
                }
            })
            .collect();
        gens.sort();
        let from = if gens.len() == 1 {
            gens.remove(0)
        } else {
            format!("(or {})", gens.join(" "))
        };
        merged.push(format!("(merged {} {from})", model(seen)));
    }
    merged.sort();
    items.extend(merged);
    Ok(Some(format!("(region {})", items.join(" "))))
}

/// R as the claim reads it, from the one description every surface reads
/// ([`crate::described`]).
fn of_described(d: Described) -> Lattice {
    Lattice {
        route: match d.route {
            Route::DownSet => "route 1",
            Route::Cube => "route 2",
            Route::States => "state route",
            Route::Tree => "obligation tree",
        },
        monotone: d.monotonicity.is_monotone(),
        any_alive: d.any_alive,
        states: d
            .states
            .into_iter()
            .map(|s| State {
                rep: s.rep.to_vec(),
                commitments: s.commitments,
                generators: s.generators.into_iter().map(|g| g.to_vec()).collect(),
            })
            .collect(),
        frontier: d.frontier.into_iter().map(|o| o.set.to_vec()).collect(),
        candidates: d.candidates,
    }
}

/// Every listed fact as the checker reads it.
struct Want {
    closes: BTreeMap<String, BTreeSet<String>>,
    negatives: Vec<String>,
}

fn want(ast: &Ast, m: &Model) -> Want {
    let mut closes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut negatives = Vec::new();
    for &f in &m.facts {
        let f = ein_ir::expect::fact(ast, f);
        if f.negated {
            negatives.push(f.rendered);
        } else {
            closes
                .entry(f.relation.to_string())
                .or_default()
                .insert(f.rendered);
        }
    }
    Want { closes, negatives }
}

/// A state's fixpoint, as the comparison reads it.
struct Seen {
    all: BTreeSet<String>,
    by_rel: BTreeMap<String, BTreeSet<String>>,
}

fn relation_of(fact: &str) -> &str {
    fact.trim_start_matches('(')
        .split([' ', ')'])
        .next()
        .unwrap_or("")
}

fn seen(facts: BTreeSet<String>) -> Seen {
    let mut by_rel: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for f in &facts {
        let r = relation_of(f);
        if r != "not" {
            by_rel.entry(r.to_string()).or_default().insert(f.clone());
        }
    }
    Seen { all: facts, by_rel }
}

/// `w` against `s`, every relation in `closed` at its complete extent.
fn fits(w: &Want, s: &Seen, closed: &BTreeSet<String>) -> bool {
    let none = BTreeSet::new();
    closed
        .iter()
        .all(|rel| w.closes.get(rel).unwrap_or(&none) == s.by_rel.get(rel).unwrap_or(&none))
        && w.negatives.iter().all(|n| s.all.contains(n))
}

/// A perfect matching between listed models and states, or `None`.
fn matching(wants: &[Want], states: &[Seen], closed: &BTreeSet<String>) -> Option<Vec<usize>> {
    if wants.len() != states.len() {
        return None;
    }
    let mut owner: Vec<Option<usize>> = vec![None; states.len()];
    fn augment(
        i: usize,
        wants: &[Want],
        states: &[Seen],
        closed: &BTreeSet<String>,
        seen: &mut [bool],
        owner: &mut [Option<usize>],
    ) -> bool {
        for j in 0..states.len() {
            if seen[j] || !fits(&wants[i], &states[j], closed) {
                continue;
            }
            seen[j] = true;
            if owner[j].is_none_or(|k| augment(k, wants, states, closed, seen, owner)) {
                owner[j] = Some(i);
                return true;
            }
        }
        false
    }
    for i in 0..wants.len() {
        let mut seen = vec![false; states.len()];
        if !augment(i, wants, states, closed, &mut seen, &mut owner) {
            return None;
        }
    }
    // `owner[j]` is the model state j matched; the inverse is what the
    // caller wants.
    let mut of_model = vec![0; wants.len()];
    for (j, o) in owner.iter().enumerate() {
        if let Some(i) = o {
            of_model[*i] = j;
        }
    }
    Some(of_model)
}

/// The fixpoints of `sets`, root's facts included, rendered.
fn fixpoints(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    candidates: &[Candidate],
    sets: &[Vec<u32>],
    opts: &Options,
) -> Result<Vec<Seen>, SolveError> {
    if sets.is_empty() {
        return Ok(Vec::new());
    }
    let prepared = prepare(root, terms, ast)?;
    let axes = Axes::new(root, candidates, prepared.hypothesised.clone(), true).taking_cores(false);
    let facts: Vec<Vec<FactId>> = sets
        .iter()
        .map(|s| s.iter().map(|&b| candidates[b as usize].fact).collect())
        .collect();
    let pool = pool_of(opts.jobs);
    let outs = fan_out(
        pool.as_ref(),
        root,
        &prepared.snapshot,
        terms,
        ast,
        &prepared.memo,
        &axes,
        &facts,
    )?;
    let root_text: BTreeSet<String> = root.facts().map(|f| sexpr(terms, f)).collect();
    Ok(outs
        .into_iter()
        .map(|o| {
            let mut all = root_text.clone();
            all.extend(o.delta);
            seen(all)
        })
        .collect())
}

/// Where exactly one state of R and one listed model are left over, the facts
/// they differ in over the closed relations — and each fact the state holds
/// and the model does not list, with the rule that put it there. That is M1c
/// T1c.1.3.3's line, kept when the query's claim of a state became the region
/// claim (M1e2 S1e2.1a.5): the step after "there is an extra fact here" is
/// where its reader goes next.
fn one_off(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    l: &Lattice,
    states: &[Seen],
    wants: &[Want],
    closed: &BTreeSet<String>,
) -> Result<Vec<String>, SolveError> {
    let lost: Vec<usize> = (0..states.len())
        .filter(|&i| !wants.iter().any(|w| fits(w, &states[i], closed)))
        .collect();
    let extra: Vec<usize> = (0..wants.len())
        .filter(|&j| !states.iter().any(|s| fits(&wants[j], s, closed)))
        .collect();
    let ([i], [j]) = (lost.as_slice(), extra.as_slice()) else {
        return Ok(Vec::new());
    };
    let (s, w) = (&states[*i], &wants[*j]);
    let none = BTreeSet::new();
    let mut surplus: Vec<String> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for rel in closed {
        let have = s.by_rel.get(rel).unwrap_or(&none);
        let listed = w.closes.get(rel).unwrap_or(&none);
        for f in listed.difference(have) {
            lines.push(format!(
                "  the model lists {f}, which the state does not hold"
            ));
        }
        surplus.extend(have.difference(listed).cloned());
    }
    if surplus.is_empty() {
        return Ok(lines);
    }
    let facts: Vec<FactId> = l.states[*i]
        .rep
        .iter()
        .map(|&b| l.candidates[b as usize].fact)
        .collect();
    let mut nodes = region::Nodes::new(root, terms, ast, &l.candidates)?;
    let why: Vec<(String, Option<String>)> = nodes.with_fixpoint(&facts, |kb, terms| {
        surplus
            .iter()
            .map(|f| {
                let id = kb.facts().find(|&id| sexpr(terms, id) == *f);
                (
                    f.clone(),
                    id.and_then(|id| crate::expect::provenance(terms, kb, id)),
                )
            })
            .collect()
    })?;
    for (f, w) in why {
        lines.push(format!(
            "  the state holds {f}, which the model does not list"
        ));
        if let Some(w) = w {
            lines.push(format!("    {w}"));
        }
    }
    Ok(lines)
}

fn describe(s: &Seen, closed: &BTreeSet<String>) -> String {
    let facts: Vec<&String> = closed
        .iter()
        .filter_map(|r| s.by_rel.get(r))
        .flatten()
        .collect();
    if facts.is_empty() {
        "(model)".to_string()
    } else {
        format!(
            "(model {})",
            facts
                .iter()
                .map(|f| f.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

fn compare(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    claim: &Claim,
    l: &Lattice,
    opts: &Options,
) -> Result<Report, SolveError> {
    let k_s = l.states.len();
    let failed = |lines: Vec<String>| Report {
        outcome: Outcome::Failed,
        lines,
    };
    let held = Report {
        outcome: Outcome::Held,
        lines: Vec::new(),
    };
    let stated = match claim {
        Claim::False => {
            return Ok(if !l.any_alive {
                held
            } else if k_s > 0 {
                failed(vec![format!(
                    "(false) says every node is dead, and R has {k_s} state{}",
                    if k_s == 1 { "" } else { "s" }
                )])
            } else {
                failed(vec![
                    "(false) says every node is dead; R is empty, but alive nodes owe — \
                     that is `(region (owing …))`"
                        .to_string(),
                ])
            });
        }
        Claim::Region(r) => r,
    };
    // The relations the claim closes: every one a model, a `<to>` or an
    // owing model names positively.
    let mut closed: BTreeSet<String> = BTreeSet::new();
    let models_named = stated
        .models
        .iter()
        .chain(stated.merged.iter().map(|m| &m.to))
        .chain(stated.owing.iter());
    for m in models_named {
        closed.extend(want(ast, m).closes.into_keys());
    }
    if !stated.owing.is_empty() || stated.models.is_empty() {
        return owing_or_empty(root, terms, ast, stated, l, &closed, opts);
    }
    if k_s == 0 {
        return Ok(failed(vec![format!(
            "the claim lists {} state{} and R is empty",
            stated.models.len(),
            if stated.models.len() == 1 { "" } else { "s" }
        )]));
    }
    let reps: Vec<Vec<u32>> = l.states.iter().map(|s| s.rep.clone()).collect();
    let states = fixpoints(root, terms, ast, &l.candidates, &reps, opts)?;
    let wants: Vec<Want> = stated.models.iter().map(|m| want(ast, m)).collect();
    let Some(of_model) = matching(&wants, &states, &closed) else {
        let mut lines = vec![format!(
            "the claim lists {} state{} and R has {k_s}",
            wants.len(),
            if wants.len() == 1 { "" } else { "s" }
        )];
        if wants.len() == k_s {
            lines[0] = "R's states and the listed models do not match one to one".to_string();
        }
        for s in &states {
            if !wants.iter().any(|w| fits(w, s, &closed)) {
                lines.push(format!(
                    "  a state of R no model lists: {}",
                    describe(s, &closed)
                ));
            }
        }
        for (w, m) in wants.iter().zip(&stated.models) {
            if !states.iter().any(|s| fits(w, s, &closed)) {
                let text: Vec<String> = m
                    .facts
                    .iter()
                    .map(|&f| ein_ir::expect::render(ast, f))
                    .collect();
                lines.push(format!(
                    "  a listed model no state of R is: (model {})",
                    text.join(" ")
                ));
            }
        }
        lines.extend(one_off(root, terms, ast, l, &states, &wants, &closed)?);
        return Ok(failed(lines));
    };
    if stated.merged.is_empty() {
        return Ok(held);
    }
    // `(merged …)`, all or nothing: every state with more than one commitment,
    // with exactly its minimal ones.
    let text = |set: &[u32]| -> BTreeSet<String> {
        set.iter()
            .map(|&b| l.candidates[b as usize].text.clone())
            .collect()
    };
    let mut lines = Vec::new();
    let mut covered = vec![false; k_s];
    for m in &stated.merged {
        let w = want(ast, &m.to);
        let Some(i) = (0..stated.models.len()).find(|&i| fits(&w, &states[of_model[i]], &closed))
        else {
            continue; // the loader holds every `<to>` to a listed model
        };
        let st = &l.states[of_model[i]];
        covered[of_model[i]] = true;
        let minimal: BTreeSet<BTreeSet<String>> = st.generators.iter().map(|s| text(s)).collect();
        let from: BTreeSet<BTreeSet<String>> = m
            .from
            .iter()
            .map(|g| {
                g.facts
                    .iter()
                    .map(|&f| ein_ir::expect::render(ast, f))
                    .collect()
            })
            .collect();
        if st.commitments < 2 {
            lines.push(format!(
                "a (merged …) names {}, which one commitment alone reaches",
                describe(&states[of_model[i]], &closed)
            ));
        } else if from != minimal {
            lines.push(format!(
                "{} is reached by {} commitments, and its minimal ones are not the listed ones",
                describe(&states[of_model[i]], &closed),
                st.commitments
            ));
        }
    }
    for (j, st) in l.states.iter().enumerate() {
        if st.commitments > 1 && !covered[j] {
            lines.push(format!(
                "{} is reached by {} commitments and no (merged …) lists it",
                describe(&states[j], &closed),
                st.commitments
            ));
        }
    }
    Ok(if lines.is_empty() {
        held
    } else {
        failed(lines)
    })
}

fn owing_or_empty(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    stated: &Stated,
    l: &Lattice,
    closed: &BTreeSet<String>,
    opts: &Options,
) -> Result<Report, SolveError> {
    let k_s = l.states.len();
    let failed = |lines: Vec<String>| Report {
        outcome: Outcome::Failed,
        lines,
    };
    if k_s > 0 {
        return Ok(failed(vec![format!(
            "the claim says R is empty, and R has {k_s} state{}",
            if k_s == 1 { "" } else { "s" }
        )]));
    }
    if stated.owing.is_empty() {
        // `(region)`: R is empty, of either kind.
        return Ok(Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        });
    }
    if !l.any_alive {
        return Ok(failed(vec![
            "(owing …) says alive nodes owe, and every node is dead — that is `(false)`"
                .to_string(),
        ]));
    }
    let states = fixpoints(root, terms, ast, &l.candidates, &l.frontier, opts)?;
    let wants: Vec<Want> = stated.owing.iter().map(|m| want(ast, m)).collect();
    Ok(match matching(&wants, &states, closed) {
        Some(_) => Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        },
        None => {
            let mut lines = vec![format!(
                "(owing …) lists {} state{}, and the alive owing nodes with no alive node one \
                 fact up are {}",
                wants.len(),
                if wants.len() == 1 { "" } else { "s" },
                states.len()
            )];
            for s in &states {
                if !wants.iter().any(|w| fits(w, s, closed)) {
                    lines.push(format!(
                        "  an owing state no model lists: {}",
                        describe(s, closed)
                    ));
                }
            }
            failed(lines)
        }
    })
}
