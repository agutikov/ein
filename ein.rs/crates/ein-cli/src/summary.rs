//! `ein solve --json-summary FILE` — the structured run summary, since M1e2
//! S1e2.1a.5's switch `ein-summary/2`: the read-out as data
//! ([`read_out.md` § 8.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#81---json-summary-ein-summary2)).
//!
//! Four properties: **additive** (writes a file, never stdout/stderr/the exit
//! code), **order-free** (every set-shaped observable sorted, so a difference
//! reports semantics rather than order), **self-describing** (field order
//! fixed by construction, `schema` versioned), and **one shape on every arm**:
//! the key set does not depend on the question or on what R turned out to
//! be, so a consumer switching on `schema` never tests for a key (*a field
//! that appears only sometimes is a field a consumer has to guess about*).
//! `ein-cli/tests/region_answer.rs` holds the key set on every arm, and
//! `summary_properties.rs` the region's identities. `ein-summary/1`, the
//! walk's, went with the walk's CLI path.

use ein_core::{Kb, SolverConfig, Terms};
use ein_infer::SharedMemo;
use ein_infer::hypgen::{Drop, HypGenStats, Skip};
use ein_ir::Ast;
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

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

/// `--json-summary`'s `root` block: root's fixpoint and how H was built, as
/// every route starts from them — M1e2
/// [S1e2.3.1](../../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md),
/// the user's ruling (a) of 2026-11-03.
///
/// **It runs nothing.** `kb` is root's fixpoint already: every writer reaches
/// the summary after [`ein_infer::region`]'s `prepare_with` has saturated root
/// in place, and no route writes into root after it. `stats` is what that same
/// pass counted ([`ein_infer::region::RootStats`]): the plans root's saturator
/// compiled, its NAF boundary's counters, and the config-free generator pass
/// whose `emitted` is H's size. So the block is config-free, no lever moves it,
/// and `summary_properties.rs` holds `facts` to root's fixpoint and `emitted`
/// to `region.universe.candidates`. Until that stage the block ran its own
/// pass on a fork — `emit_closed`'s markers, a second saturation and the
/// walk's generator with the lookahead and its kill cache on — and counted
/// what that pass wrote as root's (27 facts where root holds 22 on
/// `branching/02_one_dead_one_alive.ein`, and 30 candidates where H holds 48).
fn root_block(terms: &Terms, kb: &Kb, stats: &ein_infer::region::RootStats) -> Json {
    let mut by_rel: std::collections::BTreeMap<String, i64> = Default::default();
    for f in kb.facts() {
        let rel = terms.facts.get(f).0;
        *by_rel.entry(terms.sym(rel).to_string()).or_default() += 1;
    }
    let h = &stats.hypgen;
    Json::obj(vec![
        ("facts", Json::int(kb.n_facts() as i64)),
        (
            "facts_by_relation",
            Json::Object(by_rel.into_iter().map(|(k, v)| (k, Json::int(v))).collect()),
        ),
        ("plans", Json::int(stats.plans as i64)),
        (
            "saturator",
            Json::obj(vec![
                ("naf_rounds", Json::int(stats.naf[0] as i64)),
                ("naf_admitted", Json::int(stats.naf[1] as i64)),
                ("naf_retired", Json::int(stats.naf[2] as i64)),
                ("naf_dropped", Json::int(stats.naf[3] as i64)),
            ]),
        ),
        (
            "hypgen",
            Json::obj(vec![
                ("raw", Json::int(h.raw as i64)),
                ("emitted", Json::int(h.emitted as i64)),
                (
                    "filtered",
                    sparse(&h.filtered, |i| {
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
                    sparse(&h.pre_candidate, |i| {
                        [
                            Skip::ClosedRelation,
                            Skip::NoHypothesisRelation,
                            Skip::RelationNotWhitelisted,
                            Skip::SelfEdge,
                        ][i]
                            .as_str()
                    }),
                ),
                // What the universe leaves out (M1e2 S1e2.2.8, Q-M1e2.3 (a)):
                // on every run, `[]` when nothing.
                (
                    "outside",
                    ein_render::dump::region::outside_json(&stats.outside),
                ),
                // What the untyped rung bound as an object (M1e2 S1e2.2.10,
                // Q-M1e.12 (a)): on every run, empty when nothing.
                (
                    "types_as_objects",
                    ein_render::dump::region::types_json(&stats.types_as_objects),
                ),
            ]),
        ),
    ])
}

/// What the **blind** enumerator would still propose at one recorded state.
///
/// The probe M1d [P1d.2 handed forward] and
/// [S1d.3.1](../../../../docs/history/m1d_satisfiability/README.md#s1d31--what-the-models-differ-in)
/// takes: `complete` means *the active rung proposes nothing*, and the active
/// rung is whichever of the three the program earned. A node the hrule rung or
/// the obligations rung called complete may still have facts the blind
/// enumerator would guess at, and their number is the difference between "one
/// model" and "2ⁿ models" when the reading is open-world.
///
/// **It is a read, and that is the whole design.** The pass is config-free
/// since M1e2 S1e2.3.1 (the user's ruling of 2026-11-03: no lookahead filter,
/// so no `(not h)` kill is written) and writes nothing but the interning of
/// its candidates; it still runs on a fork of the state's KB, dropped on the
/// next line, so that no pass this probe makes can reach the state. P1d.2
/// declined this measurement because the pass it had in mind ran against the
/// live node; the fork is the difference between the two, and it costs one
/// generation call per *listed state*.
///
/// The event log is silent for the same reason `--events` output is a
/// contract: a probe nobody asked for must not narrate into a stream somebody
/// is diffing.
///
/// [P1d.2 handed forward]: `docs/history/m1d_satisfiability/hypotheses_from_obligations.md`
fn leftover_at(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
) -> Result<(u64, Vec<(String, Json)>), String> {
    use ein_infer::hypgen::generate_blind;
    use ein_infer::saturator::Session;
    use std::ops::ControlFlow;

    let mut silent = ein_infer::events::Events::off();
    let mut fork = kb.fork();
    let mut stats = HypGenStats::new();
    let mut emitted: Vec<ein_core::FactId> = Vec::new();
    {
        let mut s = Session {
            kb: &mut fork,
            terms,
            ast,
            events: &mut silent,
            memo: SharedMemo::default(),
        };
        generate_blind(&mut s, &mut stats, &mut |f| {
            emitted.push(f);
            ControlFlow::Continue(())
        })
        .map_err(crate::common::compile_error_line)?;
    }
    // Attributed, because a bare count cannot answer the question the count is
    // *for*: whether the leftover is the program failing to close a domain it
    // means to close, or a relation it never meant to decide. On `zebra2` the
    // answer is the second, and only the split says so.
    let mut by_rel: std::collections::BTreeMap<String, i64> = Default::default();
    for f in emitted {
        let rel = terms.facts.get(f).0;
        *by_rel.entry(terms.sym(rel).to_string()).or_default() += 1;
    }
    Ok((
        stats.emitted,
        by_rel.into_iter().map(|(k, n)| (k, Json::int(n))).collect(),
    ))
}

/// `EIN_LEFTOVER=1`'s block for a region run — M1d S1d.3.1's probe, re-read
/// per listed state at M1e2 S1e2.1a.5 (`read_out.md` § 12): what the
/// **blind** enumerator would still propose at each state the read-out
/// listed, outside H — a question about the universe, not about a route.
///
/// Off unless the variable is `1`, for the reasons the walk's had: a blind
/// generation pass per state would sit in every corpus summary of
/// `cargo test`, and a `(config …)` knob would re-bless every shape golden.
/// The block is emitted either way, `taken` saying which, and `states` and
/// `states_by_relation` are index-aligned with `region.states`' listed ones.
fn leftover_block(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    r: &ein_infer::read_out::ReadOut,
) -> Result<Json, String> {
    let taken = matches!(std::env::var("EIN_LEFTOVER").as_deref(), Ok("1"));
    let mut counts: Vec<Json> = Vec::new();
    let mut by_relation: Vec<Json> = Vec::new();
    if taken {
        let a = &r.answer;
        let sets: Vec<Vec<ein_core::FactId>> = r
            .listed()
            .map(|(_, s)| {
                s.rep
                    .iter()
                    .map(|&b| a.candidates[b as usize].fact)
                    .collect()
            })
            .collect();
        let mut nodes = ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates)
            .map_err(|e| e.to_string())?;
        let found = nodes
            .with_fixpoints(&sets, |kbs, terms| {
                kbs.iter_mut()
                    .map(|kb| leftover_at(ast, terms, kb))
                    .collect::<Result<Vec<_>, String>>()
            })
            .map_err(|e| e.to_string())??;
        for (n, rel) in found {
            counts.push(Json::int(n as i64));
            by_relation.push(Json::Object(rel));
        }
    }
    Ok(Json::obj(vec![
        ("taken", Json::Bool(taken)),
        ("states", Json::Array(counts)),
        ("states_by_relation", Json::Array(by_relation)),
    ]))
}

/// The resolved config: the two flags a program can set since M1e2
/// S1e2.2.5 (`ein_core::config::FLAGS`), kebab-cased, in declaration order —
/// the same fields `--dump-config` prints.
fn config_block(cfg: &SolverConfig) -> Json {
    Json::obj(vec![
        ("lattice-sanity-check", Json::Bool(cfg.lattice_sanity_check)),
        (
            "record-alternative-justifications",
            Json::Bool(cfg.record_alternative_justifications),
        ),
    ])
}

/// The summary of a region run, `ein-summary/2` — M1e2 T1e2.1a.5.2,
/// [`read_out.md` § 8.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#81---json-summary-ein-summary2).
/// The read-out's blocks (`asked`, `region`, `verdict`, `queries`, `claim`,
/// `work`) from [`ein_render::dump::region_answer::summary_blocks`], then the
/// program's: the static check, `EIN_LEFTOVER`'s block, root's fixpoint and H
/// (`root_block`) and the resolved config. Every key on every run.
///
/// `kb` is the root the run answered from, at its fixpoint.
pub fn build_region(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    r: &ein_infer::read_out::ReadOut,
    source: &str,
    config: &SolverConfig,
) -> Result<Json, String> {
    let mut pairs: Vec<(&str, Json)> = vec![
        (
            "schema",
            Json::str(ein_render::dump::region_answer::SUMMARY),
        ),
        ("source", Json::str(source)),
    ];
    pairs.extend(ein_render::dump::region_answer::summary_blocks(r));
    let mut m = ein_render::dump::region::monotonicity_json(&r.answer.monotonicity, terms);
    if let Json::Object(fields) = &mut m {
        fields.extend(
            ein_render::dump::region::warning_json_with(
                r.answer.warning.as_deref(),
                r.spot.as_ref(),
            )
            .into_iter()
            .map(|(k, v)| (k.to_string(), v)),
        );
    }
    pairs.push(("monotonicity", m));
    // Read before the probe forks the states, though the probe leaves root as
    // it is: the block is root as the routes left it.
    let root = root_block(terms, kb, &r.answer.root);
    pairs.push(("leftover", leftover_block(ast, terms, kb, r)?));
    pairs.push(("root", root));
    pairs.push(("config", config_block(config)));
    Ok(Json::obj(pairs))
}

/// The summary of `ein solve --monotone`, `ein-summary/2` — M1e2 S1e2.1b.5:
/// every key a region run's summary has, `asked.ask` = `"monotone"`, no
/// region, no verdict, no query answered, root's one saturation as the work,
/// and the static check's block. Its `root` block is a region run's, so
/// `root.hypgen.emitted` is the universe its stdout prints.
pub fn build_monotone(
    terms: &Terms,
    kb: &Kb,
    checked: &ein_infer::explore::Checked,
    source: &str,
    config: &SolverConfig,
) -> Json {
    let taken = matches!(std::env::var("EIN_LEFTOVER").as_deref(), Ok("1"));
    Json::obj(vec![
        (
            "schema",
            Json::str(ein_render::dump::region_answer::SUMMARY),
        ),
        ("source", Json::str(source)),
        (
            "asked",
            Json::obj(vec![
                ("which", Json::Null),
                ("ask", Json::str("monotone")),
                ("n", Json::Null),
            ]),
        ),
        ("region", Json::Null),
        ("verdict", Json::Null),
        ("queries", Json::Array(Vec::new())),
        ("claim", Json::Null),
        (
            "work",
            Json::obj(vec![
                ("saturations", Json::int(1)),
                ("facts", Json::int(0)),
                ("layers", Json::Null),
            ]),
        ),
        ("monotonicity", {
            // The static half only: `--monotone` reads no node, so the
            // dynamic keys are null, and present because the shape is one.
            let mut m = ein_render::dump::region::monotonicity_json(&checked.monotonicity, terms);
            if let Json::Object(fields) = &mut m {
                fields.extend(
                    ein_render::dump::region::warning_json(None)
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v)),
                );
            }
            m
        }),
        (
            "leftover",
            Json::obj(vec![
                ("taken", Json::Bool(taken)),
                ("states", Json::Array(Vec::new())),
                ("states_by_relation", Json::Array(Vec::new())),
            ]),
        ),
        ("root", root_block(terms, kb, &checked.root)),
        ("config", config_block(config)),
    ])
}

/// The summary as JSON, two-space indented, **ASCII-only**, with a trailing
/// newline.
///
/// # Escaped here, literal in the event stream — M1e S1e.4.8
///
/// `dumps_indent` escapes every non-ASCII character, so `ein solve
/// examples/zebra.ein --json-summary` emits `\u2014` where the *same run*'s
/// `--events` emits a literal em dash: `ein-infer`'s event writer leaves
/// non-ASCII text as it is (`events.rs`, pinned by
/// `strings_escape_json_specials_and_keep_non_ascii`). `MA-L3` reported the
/// difference, and it is **accepted rather than changed**:
///
/// - the summary is documented as a **parsed**, self-describing artefact and
///   every reader in the tree parses it, so `json.loads` and `serde_json` give
///   identical values from either encoding — the difference is invisible to
///   every consumer that exists;
/// - an ASCII-only artefact survives a pipeline, a locale and a terminal that
///   an em dash does not, which is a real virtue for the surface an embedder
///   is pointed at.
///
/// What holds the premise is
/// `cli_semantics::the_summary_escapes_non_ascii_where_the_event_stream_does_not`.
/// The alternative, if a consumer ever wants the bytes: thread a non-escaping
/// switch through `dump/json.rs`'s writer and point **this function alone** at
/// it — `--dump-states` and `test --json-report` must keep the default, since
/// they escape too and a blanket change would move every output they have.
pub fn write(path: &str, summary: &Json) -> std::io::Result<()> {
    std::fs::write(path, text(summary))
}

/// The bytes [`write()`] writes — which `ein solve --dump-answer`'s
/// `answer.json` also is, so the two are one spelling rather than two.
pub fn text(summary: &Json) -> String {
    dumps_indent(summary) + "\n"
}
