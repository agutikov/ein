//! `ein solve --json-summary FILE` — the structured run summary,
//! `ein-summary/2`: the read-out as data
//! ([`08-read-out.md` § 11](../../../../docs/ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks)).
//!
//! Four properties: **additive** (writes a file, never stdout/stderr/the exit
//! code), **order-free** (every set-shaped observable sorted, so a difference
//! reports semantics rather than order), **self-describing** (field order
//! fixed by construction, `schema` versioned), and **one shape on every arm**:
//! the key set does not depend on the question or on what R turned out to
//! be, so a consumer switching on `schema` never tests for a key (*a field
//! that appears only sometimes is a field a consumer has to guess about*).
//! `ein-cli/tests/region_answer.rs` holds the key set on every arm, and
//! `summary_properties.rs` the region's identities.
//!
//! **It is written wherever the run got far enough to know where to write
//! it** (Q-M2.9): a run that answers, and one with no answer, refused at the
//! parse, at the load or at its node, or ended by the clock, whose answer
//! keys are `null` ([`build_unanswered`]). The last key, `diagnostics`, is
//! what the run said on stderr, as values, in its order: a refusal's
//! findings, the clock's abort, the static check's warning, the read-out's
//! notes, a failed artefact's line, the load's notes.

use ein_core::{Diagnostic, Kb, SolverConfig, Terms};
use ein_infer::SharedMemo;
use ein_infer::hypgen::{Drop, HypGenStats, Skip};
use ein_ir::Ast;
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

/// A sparse counter block: only the keys that fired, in sorted key order —
/// which for both enums is discriminant order.
fn sparse(counts: &[u64], name: impl Fn(usize) -> &'static str) -> Json {
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
/// every route starts from them.
///
/// **It runs nothing.** `kb` is root's fixpoint already: every writer reaches
/// the summary after [`ein_infer::region`]'s `prepare_with` has saturated root
/// in place, and no route writes into root after it. `stats` is what that same
/// pass counted ([`ein_infer::region::RootStats`]): the plans root's saturator
/// compiled, its NAF boundary's counters, and the config-free generator pass
/// whose `emitted` is H's size. So the block is config-free, no lever moves it,
/// and `summary_properties.rs` holds `facts` to root's fixpoint and `emitted`
/// to `region.universe.candidates`: a pass of its own would count what it
/// wrote as root's.
fn root_block(terms: &Terms, kb: &Kb, stats: &ein_infer::region::RootStats) -> Json {
    let mut by_rel: std::collections::BTreeMap<String, i64> = Default::default();
    for f in kb.facts() {
        let rel = terms.facts.get(f).0;
        *by_rel.entry(terms.sym(rel).to_string()).or_default() += 1;
    }
    let h = &stats.hypgen;
    let mut hypgen = vec![
        ("raw", Json::int(h.raw as i64)),
        ("emitted", Json::int(h.emitted as i64)),
        (
            "filtered",
            sparse(&h.filtered, |i| {
                [Drop::FactAlreadyExists, Drop::NegatedFact, Drop::SeenInCall][i].as_str()
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
    ];
    // What H holds for debts root does not owe and a commitment can bring:
    // only where it holds a candidate for one, so every other program's
    // block is what it was.
    if !stats.later.is_empty() {
        hypgen.push((
            "later_debts",
            ein_render::dump::region::later_json(&stats.later),
        ));
    }
    // What the universe leaves out (Q-M1e2.3): on every run, `[]` when
    // nothing.
    hypgen.push((
        "outside",
        ein_render::dump::region::outside_json(&stats.outside),
    ));
    Json::obj(vec![
        ("facts", Json::int(kb.n_facts() as i64)),
        (
            "facts_by_relation",
            Json::Object(by_rel.into_iter().map(|(k, v)| (k, Json::int(v))).collect()),
        ),
        // Root's standing (Q-M1f.255): what `--no-search` reports, and what
        // every run reaches first.
        ("dead", Json::Bool(stats.dead)),
        ("owes", Json::int(i64::from(stats.owes))),
        (
            "owed",
            Json::Array(stats.owed.iter().map(|w| Json::str(w.as_str())).collect()),
        ),
        // A saved file's node (Q-M1f.402): the candidates of H its
        // `(commit …)` names, `[]` for a program.
        (
            "commit",
            Json::Array(
                stats
                    .commit
                    .iter()
                    .map(|c| Json::str(c.text.as_str()))
                    .collect(),
            ),
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
        ("hypgen", Json::obj(hypgen)),
    ])
}

/// What the **blind** enumerator would still propose at one recorded state.
///
/// The probe of
/// [what a state of R leaves open](../../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#5-a-state-of-r-is-a-model-of-the-program-as-h-scopes-it)
/// takes: `complete` means *the writers propose nothing*, the writers being
/// whichever the program has. A node the hypothesis rules or the obligations
/// called complete may still have facts the blind enumerator would guess at,
/// and their number is the difference between "one model" and "2ⁿ models"
/// when the reading is open-world.
///
/// **It is a read, and that is the whole design.** The pass is config-free
/// (no lookahead filter, so no `(not h)` kill is written) and writes nothing
/// but the interning of its candidates; it still runs on a fork of the
/// state's KB, dropped on the next line, so that no pass this probe makes can
/// reach the state, at one generation call per *listed state*.
///
/// The event log is silent for the same reason `--events` output is a
/// contract: a probe nobody asked for must not narrate into a stream somebody
/// is diffing.
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
        .map_err(|e| e.to_string())?;
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

/// `EIN_LEFTOVER=1`'s block for a region run (`06-config.md`'s `EIN_LEFTOVER`
/// row): what the
/// **blind** enumerator would still propose at each state the read-out
/// listed, outside H — a question about the universe, not about a route.
///
/// Off unless the variable is `1`: a blind generation pass per state would
/// sit in every corpus summary of
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
    // A run a saturation stopped at the cap lists no state.
    let cut = matches!(
        r.answer.stopped,
        Some(ein_infer::explore::Stopped::Steps { .. })
    );
    if taken && !cut {
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

/// The resolved config: the two flags a program can set
/// (`ein_core::config::FLAGS`), kebab-cased, in declaration order —
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

/// The summary of a region run, `ein-summary/2`, its answer blocks defined by
/// [`08-read-out.md` § 11](../../../../docs/ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks).
/// The read-out's blocks (`asked`, `region`, `verdict`, `queries`, `claim`,
/// `work`) from [`ein_render::dump::region_answer::summary_blocks`], then the
/// program's: the static check, `EIN_LEFTOVER`'s block, root's fixpoint and H
/// (`root_block`) and the resolved config, and last what the run said,
/// `diagnostics`. Every key on every run.
///
/// `kb` is the root the run answered from, at its fixpoint.
pub fn build_region(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    r: &ein_infer::read_out::ReadOut,
    source: &str,
    config: &SolverConfig,
    diagnostics: &[Diagnostic],
) -> Result<Json, String> {
    let mut pairs: Vec<(&str, Json)> = vec![
        (
            "schema",
            Json::str(ein_render::dump::region_answer::SUMMARY),
        ),
        ("source", Json::str(source)),
    ];
    pairs.extend(ein_render::dump::region_answer::summary_blocks(r));
    // A run a saturation stopped at the cap read neither the static check nor
    // root's fixpoint: both blocks are `null`, as a block that does not apply
    // is (`explore`'s § A saturation is bounded).
    let cut = matches!(
        r.answer.stopped,
        Some(ein_infer::explore::Stopped::Steps { .. })
    );
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
    pairs.push(("monotonicity", if cut { Json::Null } else { m }));
    // Read before the probe forks the states, though the probe leaves root as
    // it is: the block is root as the routes left it.
    let root = if cut {
        Json::Null
    } else {
        root_block(terms, kb, &r.answer.root)
    };
    pairs.push(("leftover", leftover_block(ast, terms, kb, r)?));
    pairs.push(("root", root));
    pairs.push(("config", config_block(config)));
    pairs.push((
        "diagnostics",
        ein_render::dump::region_answer::diagnostics(diagnostics),
    ));
    Ok(Json::obj(pairs))
}

/// The summary of `ein solve --monotone`, `ein-summary/2`: every key a
/// region run's summary has, `asked.ask` = `"monotone"`, no
/// region, no verdict, no query answered, root's one saturation as the work,
/// the static check's block, and what the run said. Its `root` block is a region run's, so
/// `root.hypgen.emitted` is the universe its stdout prints.
pub fn build_monotone(
    terms: &Terms,
    kb: &Kb,
    checked: &ein_infer::explore::Checked,
    source: &str,
    config: &SolverConfig,
    diagnostics: &[Diagnostic],
) -> Json {
    let taken = matches!(std::env::var("EIN_LEFTOVER").as_deref(), Ok("1"));
    Json::obj(vec![
        (
            "schema",
            Json::str(ein_render::dump::region_answer::SUMMARY),
        ),
        ("source", Json::str(source)),
        ("asked", asked_monotone()),
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
        (
            "diagnostics",
            ein_render::dump::region_answer::diagnostics(diagnostics),
        ),
    ])
}

/// **The summary of a run with no answer**, `ein-summary/2` (Q-M2.9): a
/// program refused at the parse, at the load or at its node, or a run the
/// clock ended. Every key a region run's summary has, and every key of the
/// answer `null`: `region`, `verdict`, `queries`, `claim`, `work`,
/// `monotonicity`, `leftover` and `root`. `asked` is what the command asked,
/// where it chose a question, and `config` the program's where it loaded;
/// `diagnostics` say why there is no answer.
pub fn build_unanswered(
    source: &str,
    asked: Option<Json>,
    config: Option<&SolverConfig>,
    diagnostics: &[Diagnostic],
) -> Json {
    Json::obj(vec![
        (
            "schema",
            Json::str(ein_render::dump::region_answer::SUMMARY),
        ),
        ("source", Json::str(source)),
        ("asked", asked.unwrap_or(Json::Null)),
        ("region", Json::Null),
        ("verdict", Json::Null),
        ("queries", Json::Null),
        ("claim", Json::Null),
        ("work", Json::Null),
        ("monotonicity", Json::Null),
        ("leftover", Json::Null),
        ("root", Json::Null),
        ("config", config.map_or(Json::Null, config_block)),
        (
            "diagnostics",
            ein_render::dump::region_answer::diagnostics(diagnostics),
        ),
    ])
}

/// The `asked` block of `ein solve --monotone`: a question about the
/// program, no part of R.
pub fn asked_monotone() -> Json {
    Json::obj(vec![
        ("which", Json::Null),
        ("ask", Json::str("monotone")),
        ("n", Json::Null),
    ])
}

/// The summary as JSON, two-space indented, **ASCII-only**, with a trailing
/// newline.
///
/// # Escaped here, literal in the event stream
///
/// `dumps_indent` escapes every non-ASCII character, so `ein solve
/// examples/zebra.ein --json-summary` emits `\u2014` where the *same run*'s
/// `--events` emits a literal em dash: `ein-infer`'s event writer leaves
/// non-ASCII text as it is (`events.rs`, pinned by
/// `strings_escape_json_specials_and_keep_non_ascii`). The difference is
/// **kept, not removed**:
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
