//! T1a.10.1.3 — **the T1 counters, banked as properties.**
//!
//! `ein-conformance --tier T1` compared `summary.json` between two engines:
//! every counter the engine reports about its own work, on every `solve` cell
//! of the corpus. [P1a.10](../../../../plans/m1a_rust/p1a.10_single_implementation/README.md)
//! retires the second operand, and the obvious replacement — check the numbers
//! in against a golden — banks the *value* and loses the *reason*. A counter
//! golden rots into "whatever it was last time"; two engines agreeing on 22
//! was evidence, one engine reproducing 22 is a tautology with a file behind
//! it.
//!
//! So what is banked here is the arithmetic instead: the identities that make
//! a counter set *coherent*, which hold for any correct run of any engine and
//! do not have to be looked up. A regression that moves one number without
//! moving the others fails here; one that moves them all consistently is a
//! semantic change and is
//! [P1c.1](../../../../plans/m1c_external_validation/p1c.1_stdlib_conformance/README.md)'s
//! to catch. That division is deliberate and it is on the ledger.
//!
//! Every identity below was **measured over the whole corpus before it was
//! written down** — 176 solve cells, every entry including the `slow` ones,
//! under `ein solve --json-summary`. Two of them are not identities but
//! *reasons*, and they are the interesting ones: see [`STRUCTURAL_ZEROS`].

use ein_core::{Kb, SolverConfig, Terms};
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use ein_oracle::{corpus_files, repo_root};
use serde_json::Value as J;

/// The counters that are **zero on every corpus cell**, and why — the half of
/// this file that a golden could not have said.
///
/// | counter | why it is zero |
/// |---|---|
/// | `root.saturator.naf_dropped` | structurally 0 since S1.21.8 moved NAF to the closure/world boundary: a guard is parked or admitted, never dropped. design/01 §2 says ein.rs reporting anything else means the boundary was rebuilt wrong |
/// | `stats.enterings_dead_pre` | a fork is `dead-pre` when `contradiction::detect` fires on the hypothesis facts *alone*, which needs the commitment to hold some `X` and `(not X)`. Hypgen only ever proposes positives (`hypgen::generate` builds candidates from relation extents) and drops any whose negation is already believed (`negated_fact`), and `apriori::filter_candidate` re-drops a candidate whose element left `alive` — which is exactly what the singleton writeback does when it writes `(not h)`. So no commitment can carry a pair, and every death is `dead-post` |
/// | `stats.nogoods_subsumed` | the counter's other half: `handle_dead` emits a clause per death and counts it `subsumed` when the store already implies it. No corpus entry reaches a death whose clause is subsumed |
///
/// The first two are **claims about the engine** and stay zero unless
/// something changes on purpose. The third is a **claim about the corpus** and
/// is the weaker one: it says nobody has written the fixture, not that the
/// path is unreachable. Recorded here rather than in a comment because
/// `ein-conformance` compared all three on 505 cells and learned nothing from
/// any of them — two zeroes agree for the wrong reason — and that is precisely
/// what an oracle's departure must not silently inherit.
const STRUCTURAL_ZEROS: [&str; 3] = [
    "root.saturator.naf_dropped",
    "stats.enterings_dead_pre",
    "stats.nogoods_subsumed",
];

/// The `config` block's key set, exactly.
///
/// T1 read this block as part of `summary.json`; a key that stopped being
/// emitted was a difference. With one engine it is a list, and the list is
/// here so that adding a `SolverConfig` field without reporting it fails.
const CONFIG_KEYS: [&str; 17] = [
    "candidate-order-seed",
    "enable-fail-fast-fork",
    "enable-forced-positive",
    "enable-lookahead-kill-cache",
    "enable-path-nogoods",
    "enable-pre-branch-lookahead",
    "enable-singleton-writeback",
    "enable-symmetric-mirror",
    "hypgen-obj-weight",
    "hypgen-rel-weight",
    "hypgen-scoring",
    "lattice-order",
    "lattice-order-seed",
    "lattice-sanity-check",
    "print-alive",
    "record-alternative-justifications",
    "warn-derived-naf",
];

/// The regimes a corpus `solve` cell runs under, reduced to what changes the
/// counters. The manifest's own run matrix is `solve`, `solve -e`, `solve -n
/// 3`, `solve -m 2`, `solve -p -s`, plus the `-L` / `-K` levers; `-p -s` and
/// `-m 2` are presentation and budget, and the rest are these.
///
/// `no-path-nogoods` is **not** in the manifest — the CLI has no lever for it
/// (Q-M1a.16) — and it is here because it is the one regime that falsifies
/// [`nogood_accounting`]: with the clause store off, a death emits nothing and
/// the identity has to be conditional rather than universal. A conditional
/// nobody ever runs the other side of is an untested branch.
const REGIMES: [(&str, u64, bool); 5] = [
    // name, max_enterings, exhaustive
    ("default", 300, false),
    ("exhaustive", 300, true),
    ("n3", 300, false),
    ("no-lookahead", 300, true),
    ("no-path-nogoods", 300, true),
];

fn config_for(regime: &str, kb: &Kb) -> SolverConfig {
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    match regime {
        "no-lookahead" => cfg.enable_pre_branch_lookahead = false,
        "no-path-nogoods" => cfg.enable_path_nogoods = false,
        _ => {}
    }
    cfg
}

/// One cell: load, solve, and hand back the `summary.json` the CLI would have
/// written — the same `summary::build`, so what is checked is the artefact T1
/// read and not a private view of the same numbers.
fn summary(path: &std::path::Path, regime: &str, budget: u64, exhaustive: bool) -> Option<J> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    let config = config_for(regime, &kb);
    let opts = SolveOptions {
        stop_after: if exhaustive {
            None
        } else if regime == "n3" {
            Some(3)
        } else {
            Some(1)
        },
        max_set_size: 5,
        config: Some(config.clone()),
        max_enterings: Some(budget),
        on_budget: OnBudget::Verdict,
        store_lattice: false,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).ok()?;
    let json = ein_cli::summary::build(
        &ast,
        &mut terms,
        &mut kb,
        &solved.answer,
        &solved.stats,
        &config,
        &path.to_string_lossy(),
        &mut events,
    )
    .ok()?;
    serde_json::from_str(&ein_render::dump::json::dumps_indent(&json)).ok()
}

fn u(v: &J, path: &str) -> i64 {
    let mut cur = v;
    for key in path.split('.') {
        cur = cur
            .get(key)
            .unwrap_or_else(|| panic!("summary has no {path}"));
    }
    cur.as_i64()
        .unwrap_or_else(|| panic!("{path} is not an integer: {cur}"))
}

#[test]
fn the_counter_set_is_coherent_on_every_corpus_cell() {
    let mut bad: Vec<String> = Vec::new();
    let mut cells = 0usize;
    // Non-vacuity for the two unsat-core implications: at least one cell has
    // to report a core, or both hold because none ever does.
    let mut cored = 0usize;
    // Every counter has to be non-zero *somewhere*, or it agrees for the wrong
    // reason — the discipline `hypgen_parity`'s
    // `every_filter_and_skip_fires_somewhere_in_the_corpus` already keeps for
    // the hypgen block, extended here to the stats block, which nothing
    // asserted before.
    let mut peak: std::collections::BTreeMap<&str, i64> = Default::default();
    let counters = [
        "stats.enterings_total",
        "stats.enterings_alive",
        "stats.enterings_dead_pre",
        "stats.enterings_dead_post",
        "stats.facts_merged",
        "stats.forced_positives",
        "stats.saturate_count",
        "stats.layers_explored",
        "stats.nogoods_emitted",
        "stats.nogoods_subsumed",
        "stats.solution_nodes",
        "root.facts",
        "root.plans",
        "root.saturator.naf_rounds",
        "root.saturator.naf_admitted",
        "root.saturator.naf_retired",
        "root.saturator.naf_dropped",
        "root.hypgen.raw",
        "root.hypgen.emitted",
    ];

    for path in &corpus_files() {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        for (regime, budget, exhaustive) in REGIMES {
            let Some(s) = summary(path, regime, budget, exhaustive) else {
                continue;
            };
            cells += 1;
            let at = format!("{} [{regime}]", rel.display());
            let mut check = |name: &str, ok: bool| {
                if !ok {
                    bad.push(format!("{at}: {name}"));
                }
            };

            // ── the verdict block agrees with the stats block ──────────
            check(
                "verdict.k == stats.solution_nodes",
                u(&s, "verdict.k") == u(&s, "stats.solution_nodes"),
            );
            check(
                "verdict.exhausted == stats.exhausted",
                s["verdict"]["exhausted"] == s["stats"]["exhausted"],
            );
            check(
                "len(verdict.solutions) == verdict.k",
                s["verdict"]["solutions"].as_array().map_or(0, Vec::len) as i64
                    == u(&s, "verdict.k"),
            );
            let kind = s["verdict"]["type"].as_str().unwrap_or("?");
            let k = u(&s, "verdict.k");
            let core = s["verdict"]["unsat_core"].as_array().map_or(0, Vec::len);
            check(
                "a verdict's type and its k agree",
                match kind {
                    "Solution" => k == 1,
                    "Ambiguity" => k >= 2,
                    "Contradiction" => k == 0,
                    _ => true,
                },
            );
            // An unsat core is the *answer* of a contradictory root and is
            // reported for nothing else — the one place the core reaches
            // stdout, which is why design/01 §5 excludes it from every
            // relaxation. Stated as two implications rather than as an
            // equivalence, because `Contradiction` covers two situations and
            // only one of them has anything to blame: `k = 0` with the lattice
            // exhausted is "no model exists", and `k = 0` with `exhausted
            // false` is "no model **within the cap**", which is not a
            // refutation and carries an empty core.
            //
            // The equivalence stood for 440 cells and was falsified the day
            // S1a.10.2 added `examples/syntax/equality.ein` to the corpus: two
            // `=` forms, no rule, no hypothesis that ever completes, and a
            // depth cap that cuts at layer 5 with every commitment still
            // alive. Nothing died, so nothing is blamed.
            let deaths = u(&s, "stats.enterings_dead_pre") + u(&s, "stats.enterings_dead_post");
            check(
                "an unsat core is reported for nothing but a Contradiction",
                core == 0 || kind == "Contradiction",
            );
            check(
                "a Contradiction that refuted something blames something",
                !(kind == "Contradiction" && deaths > 0) || core > 0,
            );
            if core > 0 {
                cored += 1;
            }

            // ── the search's own arithmetic ────────────────────────────
            check(
                "enterings_total == alive + dead_pre + dead_post",
                u(&s, "stats.enterings_total")
                    == u(&s, "stats.enterings_alive")
                        + u(&s, "stats.enterings_dead_pre")
                        + u(&s, "stats.enterings_dead_post"),
            );
            // Root itself is a solution node when the puzzle needs no
            // hypothesis at all — 47 of 176 cells, which is why the bound is
            // `+ 1` and not equality with `alive`.
            check(
                "solution_nodes <= enterings_alive + 1",
                u(&s, "stats.solution_nodes") <= u(&s, "stats.enterings_alive") + 1,
            );
            check(
                "a search that entered anything explored a layer",
                u(&s, "stats.layers_explored") >= 1 || u(&s, "stats.enterings_total") == 0,
            );
            check("root was saturated", u(&s, "stats.saturate_count") >= 1);

            // ── the clause store's accounting ──────────────────────────
            // `handle_dead` emits one clause per death and counts it as
            // landed or subsumed, so the two counters partition the deaths —
            // but only while the store is on, which is what the
            // `no-path-nogoods` regime exists to run the other side of.
            let deaths = u(&s, "stats.enterings_dead_pre") + u(&s, "stats.enterings_dead_post");
            let clauses = u(&s, "stats.nogoods_emitted") + u(&s, "stats.nogoods_subsumed");
            check(
                "emitted + subsumed accounts for every death",
                if s["config"]["enable-path-nogoods"] == J::Bool(true) {
                    clauses == deaths
                } else {
                    clauses == 0
                },
            );

            // ── the root block ─────────────────────────────────────────
            let by_rel: i64 = s["root"]["facts_by_relation"]
                .as_object()
                .map(|m| m.values().filter_map(J::as_i64).sum())
                .unwrap_or(-1);
            check(
                "root.facts == sum(root.facts_by_relation)",
                u(&s, "root.facts") == by_rel,
            );
            // Every candidate the generator built is either emitted or dropped
            // for exactly one named reason. `pre_candidate.*` counts what was
            // refused *before* a candidate existed, so it is outside `raw` —
            // the two blocks are the two halves of the pipeline and only the
            // second is conserved.
            let filtered: i64 = s["root"]["hypgen"]["filtered"]
                .as_object()
                .map(|m| m.values().filter_map(J::as_i64).sum())
                .unwrap_or(0);
            check(
                "hypgen.raw == emitted + sum(filtered)",
                u(&s, "root.hypgen.raw") == u(&s, "root.hypgen.emitted") + filtered,
            );

            // ── the config block, key for key ──────────────────────────
            let keys: Vec<&str> = s["config"]
                .as_object()
                .map(|m| m.keys().map(String::as_str).collect())
                .unwrap_or_default();
            check("the config block reports every lever", keys == CONFIG_KEYS);

            for name in counters {
                let v = u(&s, name);
                check(&format!("{name} is not negative"), v >= 0);
                let e = peak.entry(name).or_insert(0);
                *e = (*e).max(v);
            }
        }
    }

    assert!(
        bad.is_empty(),
        "{} of {cells} cells break a counter identity:\n  {}",
        bad.len(),
        bad.iter()
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n  ")
    );
    assert!(
        cells >= 300,
        "only {cells} cells ran — the sweep stopped looking"
    );
    assert!(
        cored >= 5,
        "only {cored} cells reported an unsat core, so both core implications \
         hold for the wrong reason"
    );

    // The zeroes, asserted as zeroes with a reason, and everything else
    // asserted to have fired at least once. A counter that is zero everywhere
    // and *not* on the list is a coverage hole that used to be invisible: T1
    // compared it 505 times and both sides said 0.
    let mut silent: Vec<&str> = peak
        .iter()
        .filter(|&(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect();
    silent.sort_unstable();
    let mut expected: Vec<&str> = STRUCTURAL_ZEROS.to_vec();
    expected.sort_unstable();
    assert_eq!(
        silent, expected,
        "the counters that never fire are not the ones with a written reason"
    );
    eprintln!(
        "counter properties: {cells} cells, {} counters, {} of them zero everywhere by design",
        counters.len(),
        expected.len()
    );
}
