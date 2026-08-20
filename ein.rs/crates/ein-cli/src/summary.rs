//! `ein solve --json-summary FILE` — the structured run summary (S1a.0.1).
//!
//! The Rust half of `ein/cli/_summary.py`. This file was the parity contract's
//! first two tiers — **T0** (the answer is the same) and **T1** (every counter
//! the engine reports about its own work is the same). The harness that read
//! it retired at S1a.10.3; what reads it now is
//! `ein-cli/tests/summary_properties.rs`, which holds the same fields to
//! thirteen arithmetic identities instead of to a second engine.
//!
//! Three properties, all still worth having: **additive** (writes a file,
//! never stdout/stderr/the exit code), **order-free** (every set-shaped
//! observable sorted, so a difference reports semantics rather than order),
//! and **self-describing** (field order fixed by construction, `schema`
//! versioned).

use ein_core::{Kb, SolverConfig, Terms, Value};
use ein_infer::SharedMemo;
use ein_infer::events::{sexpr, sexpr_value};
use ein_infer::hypgen::{Drop, HypGenStats, Skip};
use ein_infer::solve::MonotonicStats;
use ein_infer::verdict::{Answer, Verdict, goal_bindings};
use ein_ir::Ast;
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

const SCHEMA: &str = "ein-summary/1";

/// A KB's facts as sorted s-expressions — the model, as a set.
fn facts(terms: &Terms, kb: &Kb) -> Json {
    let mut out: Vec<String> = kb.facts().map(|f| sexpr(terms, f)).collect();
    out.sort();
    Json::Array(out.into_iter().map(Json::Str).collect())
}

/// One goal binding, as `json.dumps` writes ein.py's.
///
/// `goal_bindings` returns the **stored** argument, and ein.py stores an IR
/// `INT` as a Python `int` — so `json.dumps` writes it as a *number*. Every
/// binding used to go through `sexpr_value`, which made it a string, and
/// `summary.json` is T0: `(query :goal (r1 ?x ?y))` over `(r1 o3 8)` was a
/// verdict-level difference on a two-line program. Found by
/// [S1a.6.6](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s
/// fuzzer, 2026-08-20; `examples/ein-bugs/int-goal-binding.ein` is the
/// fixture. A symbol stays a string and a nested fact its s-expression.
fn binding_value(terms: &Terms, v: Value) -> Json {
    match v.as_int() {
        Some(id) => {
            let text = terms.int_text(id);
            text.parse::<i64>()
                .map_or_else(|_| Json::BigInt(text.to_string()), Json::Int)
        }
        None => Json::Str(sexpr_value(terms, v)),
    }
}

/// `(query :goal …)` binding rows, sorted; each row's keys sorted too.
fn bindings(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Json {
    let rows = goal_bindings(ast, terms, kb, None);
    let mut shown: Vec<Vec<(String, Value)>> = rows
        .iter()
        .map(|row| {
            let mut r: Vec<(String, Value)> = row
                .iter()
                .map(|(k, v)| (terms.sym(*k).to_string(), *v))
                .collect();
            r.sort_by(|a, b| a.0.cmp(&b.0));
            r
        })
        .collect();
    // Rows are ordered by their rendered form, which is what ein.py's
    // `sorted(rows, key=…)` compares.
    shown.sort_by_key(|r| {
        r.iter()
            .map(|(k, v)| (k.clone(), sexpr_value(terms, *v)))
            .collect::<Vec<_>>()
    });
    Json::Array(
        shown
            .into_iter()
            .map(|r| {
                Json::Object(
                    r.into_iter()
                        .map(|(k, v)| (k, binding_value(terms, v)))
                        .collect(),
                )
            })
            .collect(),
    )
}

/// The T0 observables: what was proved, and the model(s) that prove it.
fn verdict_block(ast: &Ast, terms: &mut Terms, answer: &Answer) -> Vec<(String, Json)> {
    let mut core: Vec<String> = Vec::new();
    let branches: Vec<&ein_infer::Solution> = match answer {
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            core = unsat_core.iter().map(|&f| sexpr(terms, f)).collect();
            core.sort();
            Vec::new()
        }
        Answer::Verdict(Verdict::Solution(s)) => vec![s],
        Answer::Verdict(Verdict::Ambiguity(bs)) => bs.iter().collect(),
        Answer::Aborted { .. } => Vec::new(),
    };
    // Sorted by model, not left in branch order: which of k models is found
    // first is a *traversal* fact, and `--shuffle` reorders it while proving
    // the answer unchanged.
    let mut solutions: Vec<(Vec<String>, Json)> = branches
        .iter()
        .map(|b| {
            let f = facts(terms, &b.kb);
            let key = match &f {
                Json::Array(items) => items
                    .iter()
                    .map(|i| match i {
                        Json::Str(s) => s.clone(),
                        _ => String::new(),
                    })
                    .collect(),
                _ => Vec::new(),
            };
            let g = bindings(ast, terms, &b.kb);
            (key, Json::obj(vec![("facts", f), ("goal_bindings", g)]))
        })
        .collect();
    solutions.sort_by(|a, b| a.0.cmp(&b.0));
    vec![
        (
            "unsat_core".to_string(),
            Json::Array(core.into_iter().map(Json::Str).collect()),
        ),
        (
            "solutions".to_string(),
            Json::Array(solutions.into_iter().map(|(_, v)| v).collect()),
        ),
    ]
}

/// Every `MonotonicStats` field, in declaration order — the base counters
/// lead, as `_BaseStats` does.
fn stats_block(stats: &MonotonicStats) -> Json {
    let b = &stats.base;
    Json::obj(vec![
        ("enterings_total", Json::int(b.enterings_total as i64)),
        ("enterings_alive", Json::int(b.enterings_alive as i64)),
        ("enterings_dead_pre", Json::int(b.enterings_dead_pre as i64)),
        (
            "enterings_dead_post",
            Json::int(b.enterings_dead_post as i64),
        ),
        ("facts_merged", Json::int(b.facts_merged as i64)),
        ("forced_positives", Json::int(b.forced_positives as i64)),
        ("saturate_count", Json::int(b.saturate_count as i64)),
        ("layers_explored", Json::int(b.layers_explored as i64)),
        ("nogoods_emitted", Json::int(b.nogoods_emitted as i64)),
        ("nogoods_subsumed", Json::int(b.nogoods_subsumed as i64)),
        ("solution_nodes", Json::int(stats.solution_nodes as i64)),
        ("exhausted", Json::Bool(stats.exhausted)),
    ])
}

/// A sparse counter block: only the keys that fired, in sorted key order —
/// which for both enums is discriminant order.
fn sparse(counts: &[u64; 4], name: impl Fn(usize) -> &'static str) -> Json {
    Json::Object(
        counts
            .iter()
            .enumerate()
            .filter(|(_, n)| **n > 0)
            .map(|(i, n)| (name(i).to_string(), Json::int(*n as i64)))
            .collect(),
    )
}

/// Root-saturation observables, computed on a fork so the caller's KB is
/// untouched: how big the fixpoint is, how many plans it took, what the
/// boundary did, and what hypgen would enumerate from it.
///
/// It runs on the **live** event log, not a silent one: ein.py's summary is
/// built after `_events.verdict` and before `_events.finish`, so its second
/// root saturation is recorded — 92 further events on a fixture whose whole
/// solve is 96. A silent one here would make every `--events --json-summary`
/// run diverge at T2, which is how this was found.
fn root_block(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    events: &mut ein_infer::events::Events,
) -> Result<Json, String> {
    use ein_infer::hypgen::generate;
    use ein_infer::saturator::{Saturator, Session};
    use std::ops::ControlFlow;

    let mut root = kb.fork();
    let (n_facts, by_rel, naf, hstats) = {
        let mut s = Session {
            kb: &mut root,
            terms,
            ast,
            events,
            memo: SharedMemo::default(),
        };
        ein_infer::closed::emit_closed(&mut s).map_err(crate::common::compile_error_line)?;
        let mut sat = Saturator::new(&mut s).map_err(|e| crate::common::saturate_error_line(&e))?;
        sat.saturate(&mut s, None, &mut |_| {})
            .map_err(|e| crate::common::saturate_error_line(&e))?;
        let mut hstats = HypGenStats::new();
        generate(&mut s, &mut hstats, &mut |_| ControlFlow::Continue(()))
            .map_err(crate::common::compile_error_line)?;
        let naf = [
            sat.naf_rounds,
            sat.naf_admitted,
            sat.naf_retired,
            sat.naf_dropped,
        ];
        let mut by_rel: std::collections::BTreeMap<String, i64> = Default::default();
        for f in s.kb.facts() {
            let rel = s.terms.facts.get(f).0;
            *by_rel.entry(s.terms.sym(rel).to_string()).or_default() += 1;
        }
        (s.kb.n_facts(), by_rel, naf, hstats)
    };

    let mut engine = ein_infer::Engine::new();
    let fork = kb.fork();
    engine
        .compile_all(ast, terms, &fork, events)
        .map_err(crate::common::compile_error_line)?;

    Ok(Json::obj(vec![
        ("facts", Json::int(n_facts as i64)),
        (
            "facts_by_relation",
            Json::Object(by_rel.into_iter().map(|(k, v)| (k, Json::int(v))).collect()),
        ),
        ("plans", Json::int(engine.len() as i64)),
        (
            "saturator",
            Json::obj(vec![
                ("naf_rounds", Json::int(naf[0] as i64)),
                ("naf_admitted", Json::int(naf[1] as i64)),
                ("naf_retired", Json::int(naf[2] as i64)),
                ("naf_dropped", Json::int(naf[3] as i64)),
            ]),
        ),
        (
            "hypgen",
            Json::obj(vec![
                ("raw", Json::int(hstats.raw as i64)),
                ("emitted", Json::int(hstats.emitted as i64)),
                (
                    "filtered",
                    sparse(&hstats.filtered, |i| {
                        [
                            Drop::FactAlreadyExists,
                            Drop::LookaheadKilled,
                            Drop::NegatedFact,
                            Drop::SeenInCall,
                        ][i]
                            .as_str()
                    }),
                ),
                (
                    "pre_candidate",
                    sparse(&hstats.pre_candidate, |i| {
                        [
                            Skip::ClosedRelation,
                            Skip::NoHypothesisRelation,
                            Skip::RelationNotWhitelisted,
                            Skip::SelfEdge,
                        ][i]
                            .as_str()
                    }),
                ),
            ]),
        ),
    ]))
}

/// The resolved `SolverConfig`, kebab-cased, in declaration order — the same
/// field order `--dump-config` prints.
fn config_block(cfg: &SolverConfig) -> Json {
    Json::obj(vec![
        (
            "enable-pre-branch-lookahead",
            Json::Bool(cfg.enable_pre_branch_lookahead),
        ),
        (
            "enable-lookahead-kill-cache",
            Json::Bool(cfg.enable_lookahead_kill_cache),
        ),
        ("hypgen-scoring", Json::str(cfg.hypgen_scoring.clone())),
        ("hypgen-rel-weight", Json::Float(cfg.hypgen_rel_weight)),
        ("hypgen-obj-weight", Json::Float(cfg.hypgen_obj_weight)),
        ("print-alive", Json::Bool(cfg.print_alive)),
        ("warn-derived-naf", Json::Bool(cfg.warn_derived_naf)),
        ("candidate-order-seed", Json::int(cfg.candidate_order_seed)),
        ("lattice-sanity-check", Json::Bool(cfg.lattice_sanity_check)),
        ("lattice-order", Json::str(cfg.lattice_order.clone())),
        (
            "lattice-order-seed",
            match cfg.lattice_order_seed {
                Some(v) => Json::int(v),
                None => Json::Null,
            },
        ),
        ("enable-path-nogoods", Json::Bool(cfg.enable_path_nogoods)),
        (
            "enable-symmetric-mirror",
            Json::Bool(cfg.enable_symmetric_mirror),
        ),
        (
            "enable-singleton-writeback",
            Json::Bool(cfg.enable_singleton_writeback),
        ),
        (
            "enable-forced-positive",
            Json::Bool(cfg.enable_forced_positive),
        ),
        (
            "record-alternative-justifications",
            Json::Bool(cfg.record_alternative_justifications),
        ),
        (
            "enable-fail-fast-fork",
            Json::Bool(cfg.enable_fail_fast_fork),
        ),
    ])
}

/// The summary object for a completed solve.
#[allow(clippy::too_many_arguments)]
pub fn build(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    answer: &Answer,
    stats: &MonotonicStats,
    config: &SolverConfig,
    source: &str,
    events: &mut ein_infer::events::Events,
) -> Result<Json, String> {
    let mut verdict = vec![
        ("type".to_string(), Json::str(answer.as_str())),
        ("k".to_string(), Json::int(stats.solution_nodes as i64)),
        ("exhausted".to_string(), Json::Bool(stats.exhausted)),
    ];
    verdict.extend(verdict_block(ast, terms, answer));
    Ok(Json::obj(vec![
        ("schema", Json::str(SCHEMA)),
        ("source", Json::str(source)),
        ("verdict", Json::Object(verdict)),
        ("stats", stats_block(stats)),
        ("root", root_block(ast, terms, kb, events)?),
        ("config", config_block(config)),
    ]))
}

/// The summary for a run cut short by `--max-time` / `--max-enterings`.
#[allow(clippy::too_many_arguments)]
pub fn build_aborted(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    reason: &str,
    stats: &MonotonicStats,
    config: &SolverConfig,
    source: &str,
    events: &mut ein_infer::events::Events,
) -> Result<Json, String> {
    Ok(Json::obj(vec![
        ("schema", Json::str(SCHEMA)),
        ("source", Json::str(source)),
        (
            "verdict",
            Json::obj(vec![
                ("type", Json::str("Aborted")),
                ("k", Json::int(stats.solution_nodes as i64)),
                ("exhausted", Json::Bool(stats.exhausted)),
                ("reason", Json::str(reason)),
                ("unsat_core", Json::Array(Vec::new())),
                ("solutions", Json::Array(Vec::new())),
            ]),
        ),
        ("stats", stats_block(stats)),
        ("root", root_block(ast, terms, kb, events)?),
        ("config", config_block(config)),
    ]))
}

/// `json.dumps(summary, indent=2, ensure_ascii=False)` with a trailing
/// newline.
pub fn write(path: &str, summary: &Json) -> std::io::Result<()> {
    std::fs::write(path, dumps_indent(summary) + "\n")
}
