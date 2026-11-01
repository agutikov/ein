//! T1a.10.1.3 — **the summary, banked as properties**, since M1e2
//! S1e2.1a.5's switch over `ein-summary/2`.
//!
//! `summary.json` carries what the run established about R, on every `solve`
//! cell of the corpus. The obvious check — the numbers against a golden —
//! banks the *value* and loses the *reason*, and reproducing 22 is a
//! tautology with a file behind it. So what is banked here is the arithmetic
//! instead: the identities that make a description of R *coherent*, which
//! hold for any correct run and do not have to be looked up. A regression that
//! moves one number without the others fails here; one that moves them all
//! consistently is a semantic change, and the oracle's test
//! (`ein-infer/tests/explore.rs`) is what catches that.
//!
//! The walk's identities (enterings, clause accounting, `solution_nodes`) went
//! with the walk's summary, `ein-summary/1`. The region's are
//! `read_out.md` § 13's: k_c ≥ k_s, each line no larger than R, the
//! components summing to k_c, an empty R exactly when k_s = 0 and of the kind
//! root says, a query's classes partitioning the states it is over.
//!
//! The file's **first** test is about the frame the numbers arrive in:
//! [`the_summary_has_one_shape_on_every_arm`] holds every arm — each question,
//! each kind of R, a partial run, the cube — to one shape at every depth.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load_file};
use serde_json::Value as J;

/// The counters that are **zero on every corpus cell**, and why.
///
/// | counter | why it is zero |
/// |---|---|
/// | `root.saturator.naf_dropped` | structurally 0 since S1.21.8 moved NAF to the closure/world boundary: a guard is parked or admitted, never dropped, and reporting anything else means the boundary was rebuilt wrong |
const STRUCTURAL_ZEROS: [&str; 1] = ["root.saturator.naf_dropped"];

/// The `config` block's key set, exactly: the seven flags a program can set
/// since M1e2 S1e2.1a.6 retired `enable-symmetric-mirror`.
const CONFIG_KEYS: [&str; 7] = [
    "enable-fail-fast-fork",
    "enable-lookahead-kill-cache",
    "enable-path-nogoods",
    "enable-pre-branch-lookahead",
    "lattice-sanity-check",
    "record-alternative-justifications",
    "warn-derived-naf",
];

/// The questions a corpus `solve` cell asks, and the budget each runs under:
/// small, so that the sweep is quick and partial runs are among its cells.
const REGIMES: [(&str, Mode); 5] = [
    (
        "describe",
        Mode {
            which: Which::All,
            ask: Ask::Describe,
        },
    ),
    (
        "maximal",
        Mode {
            which: Which::Maximal,
            ask: Ask::Describe,
        },
    ),
    (
        "minimal",
        Mode {
            which: Which::Minimal,
            ask: Ask::Describe,
        },
    ),
    (
        "single",
        Mode {
            which: Which::All,
            ask: Ask::Single,
        },
    ),
    (
        "n3",
        Mode {
            which: Which::All,
            ask: Ask::Find(3),
        },
    ),
];

const BUDGET: u64 = 1 << 14;

/// The largest cube a flagged program gets in the sweep: the cube is bounded
/// by its limit and not by the budget, and 2¹⁶ saturations a cell would be
/// most of the sweep's cost for no identity it does not already check at 2¹⁰.
const LIMIT: u32 = 10;

/// One cell: load, run, and hand back the `summary.json` the CLI would have
/// written — the same `summary::build_region`, so what is checked is the
/// artefact a consumer reads.
fn summary(path: &std::path::Path, mode: Mode, budget: u64) -> Option<J> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    let config = kb.program().config.clone().unwrap_or_default();
    let mut opts = explore::Options::new(mode);
    opts.budget = budget;
    opts.limit = LIMIT;
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).ok()?;
    let json = ein_cli::summary::build_region(
        &ast,
        &mut terms,
        &mut kb,
        &r,
        &path.to_string_lossy(),
        &config,
    )
    .ok()?;
    serde_json::from_str(&ein_render::dump::json::dumps_indent(&json)).ok()
}

// ── The schema's shape, on every arm ────────────────────────────────

/// One cell per **arm** of the read-out, and what makes it that arm, asserted
/// before the shape is, so a file that stopped being its arm is noticed.
const SHAPE_COVER: [(&str, &str, u64); 8] = [
    ("examples/branching/05_mini_zebra.ein", "describe", 1 << 19),
    (
        "tests/stdlib/algebra/09_irreflexive_violated.ein",
        "describe",
        1 << 19,
    ),
    (
        "tests/stdlib/closure/03_closed_and_owing.ein",
        "describe",
        1 << 19,
    ),
    (
        "examples/saturation/square-bwd/floors.ein",
        "describe",
        4096,
    ),
    (
        "examples/lattice/06_friends_tables.ein",
        "describe",
        1 << 19,
    ),
    (
        "examples/lattice/02_genuine_3set_death.ein",
        "maximal",
        1 << 19,
    ),
    (
        "examples/lattice/02_genuine_3set_death.ein",
        "single",
        1 << 19,
    ),
    ("examples/lattice/02_genuine_3set_death.ein", "n3", 1 << 19),
];

/// The blocks whose keys come from the **data** and not from the schema: a
/// relation name, a filter's name, a pre-candidate skip's name. Their key sets
/// differ per program by construction and are not part of the shape.
///
/// `region.stopped` is a tagged union, its keys the kind's (`why`): a budget's,
/// the cube's limit, the deadline.
const SPARSE_BLOCKS: [&str; 6] = [
    "stopped",
    "by_relation",
    "facts_by_relation",
    "filtered",
    "pre_candidate",
    "states_by_relation",
];

/// A summary's **shape**: every object's key list, at its path. Arrays are
/// opaque, and a `null` is a value — `verdict` is one on every run but
/// `--single`'s — so what is compared is the frame a consumer switches on.
fn shape(v: &J) -> std::collections::BTreeMap<String, Vec<String>> {
    fn walk(v: &J, path: &str, out: &mut std::collections::BTreeMap<String, Vec<String>>) {
        let J::Object(map) = v else { return };
        out.insert(path.to_string(), map.keys().cloned().collect());
        for (k, child) in map {
            if SPARSE_BLOCKS.contains(&k.as_str()) {
                continue;
            }
            walk(child, &format!("{path}.{k}"), out);
        }
    }
    let mut out = Default::default();
    walk(v, "", &mut out);
    out
}

/// **`ein-summary/2` has one shape.** Every object's keys, at every path two
/// arms both have, are the same, and the top level and the `region` block are
/// on every arm.
#[test]
fn the_summary_has_one_shape_on_every_arm() {
    let mode_of = |name: &str| {
        REGIMES
            .iter()
            .find(|(n, _)| *n == name)
            .expect("a regime")
            .1
    };
    let mut shapes: Vec<(String, std::collections::BTreeMap<String, Vec<String>>)> = Vec::new();
    for (rel, regime, budget) in SHAPE_COVER {
        let s = summary(&repo_root().join(rel), mode_of(regime), budget)
            .unwrap_or_else(|| panic!("{rel} did not produce a summary"));
        assert_eq!(s["schema"], "ein-summary/2", "{rel}");
        shapes.push((format!("{rel} [{regime}]"), shape(&s)));
    }
    let (first, base) = &shapes[0];
    let mut bad = Vec::new();
    for (who, sh) in &shapes[1..] {
        for path in [
            "",
            ".region",
            ".asked",
            ".work",
            ".root",
            ".config",
            ".leftover",
        ] {
            if sh.get(path) != base.get(path) {
                bad.push(format!(
                    "{who}: {path:?} is {:?}, {first} has {:?}",
                    sh.get(path),
                    base.get(path)
                ));
            }
        }
        for (path, keys) in sh {
            for (_, other) in &shapes {
                if let Some(k) = other.get(path)
                    && k != keys
                {
                    bad.push(format!("{who}: {path:?} is {keys:?}, elsewhere {k:?}"));
                }
            }
        }
    }
    bad.sort();
    bad.dedup();
    assert!(
        bad.is_empty(),
        "the schema has more than one shape:\n  {}",
        bad.join("\n  ")
    );
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
fn the_description_is_coherent_on_every_corpus_cell() {
    let mut bad: Vec<String> = Vec::new();
    let mut cells = 0usize;
    let mut described = 0usize;
    let mut peak: std::collections::BTreeMap<&str, i64> = Default::default();
    let counters = [
        "work.saturations",
        "work.facts",
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
        for (regime, mode) in REGIMES {
            let Some(s) = summary(path, mode, BUDGET) else {
                continue;
            };
            cells += 1;
            let at = format!("{} [{regime}]", rel.display());
            let mut check = |name: &str, ok: bool| {
                if !ok {
                    bad.push(format!("{at}: {name}"));
                }
            };
            let r = &s["region"];
            let states = r["states"].as_array().cloned().unwrap_or_default();
            let listed = states.iter().filter(|x| x["listed"] == true).count() as i64;
            let exhausted = r["exhausted"] == true;

            // ── what a description of R is made of ─────────────────────
            if regime == "describe" && exhausted {
                described += 1;
                let (k_s, k_c) = (u(&s, "region.k_s"), u(&s, "region.k_c"));
                check("k_c >= k_s", k_c >= k_s);
                check("one state per k_s", states.len() as i64 == k_s);
                check("an exhausted description lists every state", listed == k_s);
                check(
                    "R is empty exactly when k_s = 0",
                    (k_s == 0) == !r["empty"].is_null(),
                );
                check(
                    "R is empty of the dead kind exactly when root is dead",
                    (r["empty"] == "dead") == (r["root"] == "dead"),
                );
                if k_s > 0 {
                    let (mn, mx) = (
                        u(&s, "region.minimal.states"),
                        u(&s, "region.maximal.states"),
                    );
                    check("each line holds a state, and no more than R", {
                        (1..=k_s).contains(&mn) && (1..=k_s).contains(&mx)
                    });
                    check(
                        "the components sum to k_c",
                        r["components"]
                            .as_array()
                            .map(|c| c.iter().filter_map(J::as_i64).sum::<i64>())
                            == Some(k_c),
                    );
                    check(
                        "∅ in R makes the minimal line {∅}",
                        r["root"] != "in-r" || mn == 1,
                    );
                    let merged = states
                        .iter()
                        .filter(|x| x["commitments"].as_i64().unwrap_or(1) > 1)
                        .count() as i64;
                    check(
                        "merged counts the states reached by more than one commitment",
                        u(&s, "region.merged.states") == merged,
                    );
                    let commitments: i64 = states
                        .iter()
                        .filter_map(|x| x["commitments"].as_i64())
                        .sum();
                    check("the states' commitments sum to k_c", commitments == k_c);
                }
            }
            // ── what a mode answered ───────────────────────────────────
            match regime {
                "n3" => check("-n 3 lists at most three states", listed <= 3),
                "single" => {
                    check("--single lists at most two states", listed <= 2);
                    check(
                        "a decided --single carries its word",
                        !exhausted || s["verdict"].is_object(),
                    );
                }
                "maximal" | "minimal" => {
                    let line = if regime == "maximal" {
                        "maximal"
                    } else {
                        "minimal"
                    };
                    check(
                        "a line mode's states are on its line",
                        states.iter().all(|x| {
                            x["lines"]
                                .as_array()
                                .is_some_and(|l| l.iter().any(|w| w == line))
                        }),
                    );
                }
                _ => {}
            }
            // ── the queries, over the states the run reached ───────────
            for q in s["queries"].as_array().cloned().unwrap_or_default() {
                let classes = q["classes"].as_array().cloned().unwrap_or_default();
                let mut seen: Vec<i64> = classes
                    .iter()
                    .flat_map(|c| {
                        c["states"]
                            .as_array()
                            .cloned()
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|i| i.as_i64())
                    })
                    .collect();
                let n = seen.len();
                seen.sort_unstable();
                seen.dedup();
                check("a state is in one class of a query", seen.len() == n);
                if q["over"] == "all" && q["exact"] == true && regime == "describe" {
                    check(
                        "the classes over all of R partition it",
                        n as i64 == states.len() as i64,
                    );
                }
            }
            // ── the work, root, and the config ─────────────────────────
            check("a run saturates", u(&s, "work.saturations") >= 1);
            let by_rel: i64 = s["root"]["facts_by_relation"]
                .as_object()
                .map(|m| m.values().filter_map(J::as_i64).sum())
                .unwrap_or(-1);
            check(
                "root.facts == sum(root.facts_by_relation)",
                u(&s, "root.facts") == by_rel,
            );
            let filtered: i64 = s["root"]["hypgen"]["filtered"]
                .as_object()
                .map(|m| m.values().filter_map(J::as_i64).sum())
                .unwrap_or(0);
            check(
                "hypgen.raw == emitted + sum(filtered)",
                u(&s, "root.hypgen.raw") == u(&s, "root.hypgen.emitted") + filtered,
            );
            let keys: Vec<&str> = s["config"]
                .as_object()
                .map(|m| m.keys().map(String::as_str).collect())
                .unwrap_or_default();
            check("the config block reports every flag", keys == CONFIG_KEYS);
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
        "{} of {cells} cells break an identity:\n  {}",
        bad.len(),
        bad.iter()
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n  ")
    );
    assert!(
        cells >= 900,
        "only {cells} cells ran — the sweep stopped looking"
    );
    assert!(
        described >= 150,
        "only {described} exhausted descriptions, so R's identities hold for few programs"
    );
    let mut silent: Vec<&str> = peak
        .iter()
        .filter(|&(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect();
    silent.sort_unstable();
    assert_eq!(
        silent,
        STRUCTURAL_ZEROS.to_vec(),
        "the counters that never fire are not the ones with a written reason"
    );
    eprintln!("summary properties: {cells} cells, {described} exhausted descriptions");
}
