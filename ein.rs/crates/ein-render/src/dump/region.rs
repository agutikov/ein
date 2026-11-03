//! `ein oracle --dump DIR` — the solution region, the cube
//! ([`write_cube_dir`]).
//!
//! M1e2 [S1e2.1a.1](../../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//! Two files, documented in
//! [`region_dump.md`](../../../../../docs/kernel/inference/region_dump.md):
//!
//! - `region.json` — `ein_infer::region`'s cube: every node, its two bits and
//!   their reasons, `R`'s lines, holes, components, sub-cubes, `N`, `E`, the
//!   duality, the violations, and [`ein_infer::monotone`]'s verdict;
//! - `region.md` — the same for a reader, and the cost.
//!
//! A third, `walk.json`, was what the walk did with the lattice, written by
//! the retired `ein solve --dump-region`; it went with the walk at M1e2
//! S1e2.1c.5 (2026-11-03). A directory an older binary wrote may still hold
//! one, and [`write_cube_dir`] removes it.

use std::path::Path;

use ein_core::Terms;
use ein_infer::detector::{Status, Warning};
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{Class, Outcome, Region};
use rustc_hash::FxHashSet;

use super::json::{Json, dumps_indent};

pub const REGION_SCHEMA: &str = "ein-region/1";

/// What the CLI knows that the two halves do not.
pub struct RegionInputs<'a> {
    pub source: &'a str,
    pub limit: u32,
}

/// Write `region.json` and `region.md` into `dir`, with no walk: what
/// `ein oracle --dump DIR` asks for, M1e2 T1e2.1a.9.4. `ein render region`
/// draws such a directory with no run overlaid.
pub fn write_cube_dir(
    dir: &Path,
    oracle: &Outcome,
    oracle_terms: &Terms,
    inputs: &RegionInputs,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    // A previous `--dump-region` into the same directory leaves a walk this
    // run did not take: it goes, so the drawing cannot overlay it.
    match std::fs::remove_file(dir.join("walk.json")) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    std::fs::write(
        dir.join("region.json"),
        dumps_indent(&region_json(oracle, oracle_terms, inputs)) + "\n",
    )?;
    std::fs::write(
        dir.join("region.md"),
        region_md(oracle, oracle_terms, inputs),
    )?;
    Ok(())
}

// ── region.json ────────────────────────────────────────────────────

fn facts_of(r: &Region, mask: u64) -> Json {
    Json::Array(
        (0..r.n())
            .filter(|i| mask >> i & 1 == 1)
            .map(|i| Json::str(&r.candidates[i].text))
            .collect(),
    )
}

fn sets(r: &Region, masks: &[u64]) -> Json {
    Json::Array(masks.iter().map(|&m| facts_of(r, m)).collect())
}

fn strs(v: &[String]) -> Json {
    Json::Array(v.iter().map(Json::str).collect())
}

fn status_word(s: u8) -> &'static str {
    use ein_infer::region::{DEAD, OWING};
    match (s & DEAD != 0, s & OWING != 0) {
        (false, false) => "R",
        (false, true) => "owing",
        (true, false) => "dead",
        (true, true) => "dead+owing",
    }
}

/// The static check, as both files print it.
pub fn monotonicity_json(m: &Monotonicity, terms: &Terms) -> Json {
    let counts = |c: &ein_infer::monotone::ReadCounts| {
        Json::obj(vec![
            ("static", Json::int(c.static_ as i64)),
            ("dynamic_monotone", Json::int(c.dynamic_monotone as i64)),
            (
                "dynamic_non_monotone",
                Json::int(c.dynamic_non_monotone as i64),
            ),
        ])
    };
    let mut pairs = vec![
        ("monotone", Json::Bool(m.is_monotone())),
        (
            "hypothesised",
            Json::Array(
                m.hypothesised
                    .iter()
                    .map(|&s| Json::str(terms.sym(s)))
                    .collect(),
            ),
        ),
        (
            "dynamic",
            Json::Array(
                m.dynamic
                    .iter()
                    .map(|r| Json::str(r.label(terms)))
                    .collect(),
            ),
        ),
        (
            "plans",
            Json::obj(vec![
                ("saturation", Json::int(m.plans.0 as i64)),
                ("obligation", Json::int(m.plans.1 as i64)),
            ]),
        ),
        ("reads", counts(&m.reads)),
        ("absent_reads", counts(&m.absent_reads)),
        (
            "sites",
            Json::Array(
                m.sites
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        let mut site = vec![
                            ("rule", Json::str(terms.sym(s.rule))),
                            (
                                "activator",
                                Json::Array(
                                    s.activator
                                        .iter()
                                        .map(|&a| Json::str(terms.sym(a)))
                                        .collect(),
                                ),
                            ),
                            ("kind", Json::str(s.kind.as_str())),
                            ("relation", Json::str(s.relation.label(terms))),
                            ("depth", Json::int(s.depth as i64)),
                        ];
                        // The detector's reading of the site (M1e2
                        // S1e2.1b.5): additive, and on a flagged program only.
                        if let Some(stake) = m.detector.stakes.get(i) {
                            site.push(("concludes", Json::str(stake.concludes.as_str())));
                            site.push(("at_stake", Json::str(stake.at_stake.key())));
                        }
                        Json::obj(site)
                    })
                    .collect(),
            ),
        ),
    ];
    pairs.extend(static_half(m));
    Json::obj(pairs)
}

/// The detector's static half, after the static check's fields (M1e2
/// S1e2.1b.5): what is assumed, the cycles through an `absent` over D, the
/// order findings. Every key on every run, as `ein-summary/2` has one shape:
/// on a monotone program the detector reads nothing, so `assumes` is `null`
/// and the two lists are empty.
fn static_half(m: &Monotonicity) -> Vec<(&'static str, Json)> {
    let d = &m.detector;
    vec![
        (
            "assumes",
            if m.is_monotone() {
                Json::Null
            } else {
                Json::obj(vec![
                    ("node", Json::str("fork from root")),
                    ("open", Json::str("Q-M1e2.2, F21")),
                ])
            },
        ),
        (
            "cycles",
            Json::Array(
                d.cycles
                    .iter()
                    .map(|c| {
                        Json::obj(vec![
                            ("relations", strs(&c.relations)),
                            ("negative", strs(&c.negative)),
                            ("odd", Json::Bool(c.odd)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "order",
            Json::Array(
                d.order
                    .iter()
                    .map(|r| {
                        Json::obj(vec![
                            ("reader", Json::str(&r.reader)),
                            ("writer", Json::str(&r.writer)),
                            ("relations", strs(&r.relations)),
                            (
                                "priorities",
                                Json::Array(vec![
                                    Json::int(r.reader_priority),
                                    Json::int(r.writer_priority),
                                ]),
                            ),
                            ("at_one_priority", Json::Bool(r.at_one_priority())),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]
}

/// The detector's dynamic half (M1e2 S1e2.1b.5), after the static half:
/// `status`, `state` and `paths`, each `null` where the run has none — on a
/// monotone program, which the detector never reads, and `paths` where no
/// read-out ran. Every key on every run, as `ein-summary/2` has one shape.
pub fn warning_json(w: Option<&Warning>) -> Vec<(&'static str, Json)> {
    warning_json_with(w, None)
}

/// The spot check's `paths` object: what it read, and what differed. `bound`
/// is `null` under `-y`, which reads every parent (M1e2 S1e2.1c.5).
pub fn paths_json(p: &ein_infer::detector::Paths) -> Json {
    Json::obj(vec![
        ("bound", p.bound.map_or(Json::Null, |b| Json::int(b as i64))),
        ("states", Json::int(p.states as i64)),
        ("parents", Json::int(p.parents as i64)),
        ("dead_parents", Json::int(p.dead as i64)),
        ("saturations", Json::int(p.saturations as i64)),
        ("differ", strs(&p.differ)),
        (
            "unprinted",
            p.unprinted.map_or(Json::Null, |(met, cost)| {
                Json::obj(vec![
                    ("members", Json::int(met as i64)),
                    ("saturations", Json::int(cost as i64)),
                ])
            }),
        ),
    ])
}

/// [`warning_json`], with `-y`'s spot check on a monotone program, which
/// has no warning: `status` and `state` stay `null`, and `paths` is the
/// check's.
pub fn warning_json_with(
    w: Option<&Warning>,
    spot: Option<&ein_infer::detector::Paths>,
) -> Vec<(&'static str, Json)> {
    let Some(w) = w else {
        return vec![
            ("status", Json::Null),
            ("state", Json::Null),
            ("paths", spot.map_or(Json::Null, paths_json)),
        ];
    };
    let n_limit = |why: &str, n: usize, limit: Option<u32>| {
        let mut v = vec![
            ("read", Json::Null),
            ("bit", Json::str("not known")),
            ("why", Json::str(why)),
            ("n", Json::int(n as i64)),
        ];
        if let Some(l) = limit {
            v.push(("limit", Json::int(l as i64)));
        }
        Json::obj(v)
    };
    let status = match &w.status {
        Status::Exercised {
            by,
            holes,
            dead_not_upward_closed,
            discharged_not_upward_closed,
        } => Json::obj(vec![
            ("read", Json::str("the cube")),
            ("bit", Json::str("exercised")),
            ("by", strs(by)),
            ("holes", Json::int(*holes as i64)),
            (
                "dead_not_upward_closed",
                Json::int(*dead_not_upward_closed as i64),
            ),
            (
                "discharged_not_upward_closed",
                Json::int(*discharged_not_upward_closed as i64),
            ),
        ]),
        Status::Conservative => Json::obj(vec![
            ("read", Json::str("the cube")),
            ("bit", Json::str("conservative")),
            ("by", Json::Array(Vec::new())),
            ("holes", Json::int(0)),
            ("dead_not_upward_closed", Json::int(0)),
            ("discharged_not_upward_closed", Json::int(0)),
        ]),
        Status::PastLimit { n, limit } => n_limit("past --region-limit", *n, Some(*limit)),
        Status::NotRead { n } => n_limit("the run did not read every node", *n, None),
        Status::RootDead { n, limit } => n_limit("root is dead (L0)", *n, Some(*limit)),
    };
    let state = Json::obj(vec![
        (
            "read",
            Json::str(if w.state.cube {
                "the cube"
            } else {
                "every node the run computed"
            }),
        ),
        ("nodes", Json::int(w.state.nodes as i64)),
        ("stale_nodes", Json::int(w.state.stale.len() as i64)),
        (
            "stale",
            Json::Array(
                w.state
                    .stale
                    .iter()
                    .map(|s| {
                        Json::obj(vec![
                            ("node", Json::str(&s.node)),
                            ("class", Json::str(s.class.as_str())),
                            ("facts", strs(&s.facts)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]);
    let paths = w.paths.as_ref().map_or(Json::Null, paths_json);
    vec![("status", status), ("state", state), ("paths", paths)]
}

fn absents_json(m: &Monotonicity, terms: &Terms) -> Json {
    Json::Array(
        m.absents
            .iter()
            .map(|a| {
                Json::obj(vec![
                    ("rule", Json::str(terms.sym(a.rule))),
                    (
                        "activator",
                        Json::Array(
                            a.activator
                                .iter()
                                .map(|&x| Json::str(terms.sym(x)))
                                .collect(),
                        ),
                    ),
                    ("kind", Json::str(a.kind.as_str())),
                    ("relation", Json::str(a.relation.label(terms))),
                    ("depth", Json::int(a.depth as i64)),
                    ("class", Json::str(a.class.as_str())),
                ])
            })
            .collect(),
    )
}

fn region_json(oracle: &Outcome, terms: &Terms, inputs: &RegionInputs) -> Json {
    let (candidates, monotonicity) = match oracle {
        Outcome::Region(r) => (&r.candidates, &r.monotonicity),
        Outcome::Refused {
            candidates,
            monotonicity,
            ..
        } => (candidates, monotonicity),
    };
    let mut m = monotonicity_json(monotonicity, terms);
    if let Json::Object(pairs) = &mut m {
        let warning = match oracle {
            Outcome::Region(r) => r.warning.as_deref(),
            Outcome::Refused { .. } => None,
        };
        // The cube's half and the state: what `ein render region` labels.
        if warning.is_some() {
            pairs.extend(
                warning_json(warning)
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v)),
            );
        }
        pairs.push(("absents".to_string(), absents_json(monotonicity, terms)));
    }
    let head = vec![
        ("schema", Json::str(REGION_SCHEMA)),
        ("source", Json::str(inputs.source)),
        ("n", Json::int(candidates.len() as i64)),
        ("limit", Json::int(inputs.limit as i64)),
        (
            "candidates",
            Json::Array(candidates.iter().map(|c| Json::str(&c.text)).collect()),
        ),
        ("monotonicity", m),
    ];
    let mut pairs = head;
    match oracle {
        Outcome::Refused { n, limit, .. } => {
            pairs.push(("region", Json::Null));
            pairs.push((
                "refused",
                Json::obj(vec![
                    ("n", Json::int(*n as i64)),
                    ("limit", Json::int(*limit as i64)),
                    ("note", Json::str(refusal_note(*n, *limit))),
                ]),
            ));
        }
        Outcome::Region(r) => {
            pairs.push(("region", region_block(r)));
            pairs.push(("refused", Json::Null));
        }
    }
    Json::obj(pairs)
}

/// The refusal's sentence — the stderr note and `region.md` say the same.
pub fn refusal_note(n: usize, limit: u32) -> String {
    format!(
        "{n} root candidates, over --region-limit {limit}: the region is 2^{n} \
         saturations and was not computed; pass --region-limit {n} to compute it"
    )
}

fn region_block(r: &Region) -> Json {
    let t = &r.topology;
    let mins: FxHashSet<u64> = t.minimal.iter().copied().collect();
    let maxs: FxHashSet<u64> = t.maximal.iter().copied().collect();
    // determinism-ok: `t.holes` is a Vec in mask order; this set is membership only.
    let holes: FxHashSet<u64> = t.holes.iter().copied().collect();
    let violation = |v: &ein_infer::region::Violation| {
        Json::obj(vec![
            ("count", Json::int(v.count as i64)),
            (
                "witness",
                match v.witness {
                    Some((a, b)) => Json::Array(vec![facts_of(r, a), facts_of(r, b)]),
                    None => Json::Null,
                },
            ),
        ])
    };
    let nodes: Vec<Json> = (0..r.status.len() as u64)
        .map(|m| {
            let i = m as usize;
            let node = &r.nodes[i];
            let comp = t.component_of[i];
            let mut pairs = vec![
                ("mask", Json::int(m as i64)),
                ("layer", Json::int(m.count_ones() as i64)),
                ("facts", facts_of(r, m)),
                ("status", Json::str(status_word(r.status[i]))),
                ("minimal", Json::Bool(mins.contains(&m))),
                ("maximal", Json::Bool(maxs.contains(&m))),
                ("hole", Json::Bool(holes.contains(&m))),
                (
                    "component",
                    if comp == u32::MAX {
                        Json::Null
                    } else {
                        Json::int(comp as i64)
                    },
                ),
                ("class", Json::int(node.class as i64)),
                ("closure", Json::int(node.closure as i64)),
                ("excluded", Json::int(node.excluded as i64)),
                ("beyond", Json::int(node.beyond as i64)),
                (
                    "beyond_hypothesised",
                    Json::int(node.beyond_hypothesised as i64),
                ),
                ("owes", Json::int(node.owes as i64)),
                ("owed", strs(&node.owed)),
                ("core", strs(&node.core)),
                ("refuted_by", strs(&node.refuted_by)),
                ("absent_premises", strs(&node.absent_premises)),
            ];
            if let Some(states) = &r.states {
                pairs.push(("state", strs(&states[i])));
            }
            Json::obj(pairs)
        })
        .collect();
    Json::obj(vec![
        ("root_dead", Json::Bool(r.root_dead)),
        (
            "assumes_monotone",
            Json::Bool(!r.monotonicity.is_monotone()),
        ),
        ("size", Json::int(t.size_r as i64)),
        ("minimal", sets(r, &t.minimal)),
        ("maximal", sets(r, &t.maximal)),
        ("holes", sets(r, &t.holes)),
        ("minimal_holes", sets(r, &t.minimal_holes)),
        (
            "repairs",
            Json::Array(
                t.repairs
                    .iter()
                    .map(|&(h, b, y)| {
                        Json::obj(vec![
                            ("hole", facts_of(r, h)),
                            ("adds", Json::str(&r.candidates[b].text)),
                            ("into", facts_of(r, y)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "components",
            Json::Array(
                t.components
                    .iter()
                    .map(|c| {
                        Json::obj(vec![
                            ("size", Json::int(c.size as i64)),
                            ("minimal", sets(r, &c.minimal)),
                            ("maximal", sets(r, &c.maximal)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "subcubes",
            match &t.subcubes {
                None => Json::Null,
                Some(v) => Json::Array(
                    v.iter()
                        .map(|&(a, b)| {
                            Json::obj(vec![
                                ("from", facts_of(r, a)),
                                ("to", facts_of(r, b)),
                                ("dimension", Json::int((a ^ b).count_ones() as i64)),
                            ])
                        })
                        .collect(),
                ),
            },
        ),
        ("nogoods", sets(r, &t.nogoods)),
        ("discharging", sets(r, &t.discharging)),
        (
            "duality",
            Json::obj(vec![
                ("minimal_holds", Json::Bool(t.duality.minimal_holds)),
                ("maximal_holds", Json::Bool(t.duality.maximal_holds)),
                ("minimal_from_e", sets(r, &t.duality.minimal_from_e)),
                ("maximal_from_n", sets(r, &t.duality.maximal_from_n)),
            ]),
        ),
        ("local_max", sets(r, &t.local_max)),
        ("local_min", sets(r, &t.local_min)),
        (
            "violations",
            Json::obj(vec![
                (
                    "dead_not_upward_closed",
                    violation(&t.dead_not_upward_closed),
                ),
                (
                    "discharged_not_upward_closed",
                    violation(&t.discharged_not_upward_closed),
                ),
            ]),
        ),
        (
            "edges",
            Json::Array(
                t.edges
                    .iter()
                    .map(|&((a, b), n)| {
                        Json::obj(vec![
                            ("from", Json::str(a.as_str())),
                            ("to", Json::str(b.as_str())),
                            ("count", Json::int(n as i64)),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("nodes", Json::Array(nodes)),
    ])
}

// ── region.md ──────────────────────────────────────────────────────

fn md_set(r: &Region, m: u64) -> String {
    format!("`{}`", r.name(m))
}

fn md_sets(r: &Region, ms: &[u64]) -> String {
    if ms.is_empty() {
        return "none".to_string();
    }
    ms.iter()
        .map(|&m| md_set(r, m))
        .collect::<Vec<_>>()
        .join(", ")
}

fn region_md(oracle: &Outcome, _terms: &Terms, inputs: &RegionInputs) -> String {
    let mut out = format!("# Region · {}\n\n", inputs.source);
    let (candidates, m) = match oracle {
        Outcome::Region(r) => (&r.candidates, &r.monotonicity),
        Outcome::Refused {
            candidates,
            monotonicity,
            ..
        } => (candidates, monotonicity),
    };
    out.push_str(&format!(
        "The lattice is over **{} root candidates** — the generator's list at root's \
         fixpoint with no config switch applied (`docs/kernel/inference/region_dump.md`).\n\n",
        candidates.len()
    ));
    if m.is_monotone() {
        out.push_str(&format!(
            "**Monotone by construction**: no read a commitment can undo \
             ({} saturation and {} obligation plans read; {} reads inside an `absent`, \
             {} of them over a relation a commitment changes, all at monotone polarity).\n\n",
            m.plans.0,
            m.plans.1,
            m.absent_reads.static_ + m.absent_reads.dynamic_monotone,
            m.absent_reads.dynamic_monotone
        ));
    } else {
        // The warning, as the read-out's `lattice` block says it (M1e2
        // S1e2.1b.5, approved 2026-11-03).
        out.push_str(&format!(
            "**Not monotone: {} site{}.** A commitment can take a conclusion away or add a \
             debt, so the region may have holes and its two lines may not describe it. \
             Every node below is a fork of root: root's fixpoint, its commitment added, \
             saturated. What a node is on such a program is open (Q-M1e2.2, F21), and a line \
             marked *(nodes by fork)* is computed over the nodes so built.\n\n",
            m.sites.len(),
            if m.sites.len() == 1 { "" } else { "s" }
        ));
        for s in &m.detector.stakes {
            out.push_str(&format!("- {}\n", s.text()));
        }
        for c in &m.detector.cycles {
            out.push_str(&format!("- the cycle: {}\n", c.text()));
        }
        for o in ein_infer::detector::order_texts(&m.detector.order) {
            out.push_str(&format!("- the order: {o}\n"));
        }
        if let Outcome::Region(r) = oracle
            && let Some(w) = r.warning.as_deref()
        {
            let status = match &w.status {
                Status::Exercised {
                    by,
                    holes,
                    dead_not_upward_closed,
                    discharged_not_upward_closed,
                } => format!(
                    "non-monotone here (exercised, by {}) — {}",
                    by.join(", "),
                    crate::read_out::violation_counts(
                        *holes,
                        *dead_not_upward_closed,
                        *discharged_not_upward_closed
                    )
                ),
                s => crate::read_out::status_text(s),
            };
            out.push_str(&format!("- the status: {status}\n"));
            out.push_str(&format!(
                "- the state: {}\n",
                crate::read_out::state_text(&w.state)
            ));
        }
        out.push('\n');
    }
    out.push_str("| bit | candidate |\n|---:|---|\n");
    for (i, c) in candidates.iter().enumerate() {
        out.push_str(&format!("| {i} | `{}` |\n", c.text));
    }
    out.push('\n');

    let r = match oracle {
        Outcome::Refused { n, limit, .. } => {
            out.push_str(&format!(
                "## No region\n\n{}.\n\n",
                refusal_note(*n, *limit)
            ));
            return out;
        }
        Outcome::Region(r) => r,
    };
    let t = &r.topology;
    let assumes = if m.is_monotone() {
        ""
    } else {
        " *(nodes by fork)*"
    };
    out.push_str("## R — alive and discharged\n\n");
    out.push_str(&format!(
        "- **|R| = {}** of 2^{} = {} nodes{}{}\n",
        t.size_r,
        r.n(),
        r.status.len(),
        if r.root_dead { "; root is dead" } else { "" },
        ""
    ));
    out.push_str(&format!(
        "- **minimal line**: {}{assumes}\n",
        md_sets(r, &t.minimal)
    ));
    out.push_str(&format!(
        "- **maximal line**: {}{assumes}\n",
        md_sets(r, &t.maximal)
    ));
    out.push_str(&format!("- **holes**: {}\n", md_sets(r, &t.holes)));
    if !t.repairs.is_empty() {
        let reps: Vec<String> = t
            .repairs
            .iter()
            .map(|&(h, b, y)| {
                format!(
                    "{} + `{}` → {}",
                    md_set(r, h),
                    r.candidates[b].text,
                    md_set(r, y)
                )
            })
            .collect();
        out.push_str(&format!("- **repair edges**: {}\n", reps.join("; ")));
        for &h in &t.minimal_holes {
            let node = &r.nodes[h as usize];
            let why = if !node.absent_premises.is_empty() {
                format!("rests on {}", node.absent_premises.join(", "))
            } else if !node.owed.is_empty() {
                format!("owes: {}", node.owed.join("; "))
            } else {
                "no `absent` premise and no debt recorded".to_string()
            };
            out.push_str(&format!("  - minimal hole {}: {why}\n", md_set(r, h)));
        }
    }
    let sizes: Vec<String> = t.components.iter().map(|c| c.size.to_string()).collect();
    out.push_str(&format!(
        "- **components**: {} (sizes {}){assumes}\n",
        t.components.len(),
        if sizes.is_empty() {
            "—".to_string()
        } else {
            sizes.join(", ")
        }
    ));
    match &t.subcubes {
        Some(v) => {
            let cubes: Vec<String> = v
                .iter()
                .map(|&(a, b)| {
                    format!("[{} … {}] dim {}", r.name(a), r.name(b), (a ^ b).count_ones())
                })
                .collect();
            out.push_str(&format!(
                "- **maximal sub-cubes**: {}\n",
                if cubes.is_empty() { "none".to_string() } else { cubes.join(", ") }
            ));
        }
        None => out.push_str("- **maximal sub-cubes**: not computed (R has holes and the interval search passed its work bound)\n"),
    }
    out.push_str(&format!(
        "- **N**, minimal dead sets: {}\n",
        md_sets(r, &t.nogoods)
    ));
    out.push_str(&format!(
        "- **E**, minimal discharging sets: {}\n",
        md_sets(r, &t.discharging)
    ));
    let side = |holds: bool, from: &[u64]| {
        if holds {
            "holds".to_string()
        } else {
            format!("fails — gives {}", md_sets(r, from))
        }
    };
    out.push_str(&format!(
        "- **duality**: minimal line from E {}; maximal line from N {}\n",
        side(t.duality.minimal_holds, &t.duality.minimal_from_e),
        side(t.duality.maximal_holds, &t.duality.maximal_from_n)
    ));
    let viol = |v: &ein_infer::region::Violation, what: &str| match v.witness {
        None => format!("{what}: none"),
        Some((a, b)) => format!(
            "{what}: {} (e.g. {} ⊂ {})",
            v.count,
            md_set(r, a),
            md_set(r, b)
        ),
    };
    out.push_str(&format!(
        "- **upward closure**: {}; {}\n",
        viol(
            &t.dead_not_upward_closed,
            "dead nodes with an alive superset"
        ),
        viol(
            &t.discharged_not_upward_closed,
            "discharged nodes with an owing superset"
        )
    ));
    if m.is_monotone()
        && (t.dead_not_upward_closed.count > 0 || t.discharged_not_upward_closed.count > 0)
    {
        out.push_str(
            "- **⚠ the static check says monotone and the cube disagrees** — a defect in \
             `ein_infer::monotone`\n",
        );
    }
    if !t.local_max.is_empty() || !t.local_min.is_empty() {
        out.push_str(&format!(
            "- **local ≠ global**: maximal {}; minimal {}\n",
            md_sets(r, &t.local_max),
            md_sets(r, &t.local_min)
        ));
    }
    let edges: Vec<String> = t
        .edges
        .iter()
        .map(|&((a, b), n)| format!("{}→{} {n}", a.as_str(), b.as_str()))
        .collect();
    out.push_str(&format!("- **edges**: {}\n\n", edges.join(", ")));

    if r.n() <= 6 {
        out.push_str("## Every node\n\n| layer | node | status | why |\n|---:|---|---|---|\n");
        let mut order: Vec<u64> = (0..r.status.len() as u64).collect();
        order.sort_by_key(|m| (m.count_ones(), *m));
        for mk in order {
            let node = &r.nodes[mk as usize];
            let mut why: Vec<String> = Vec::new();
            if !node.core.is_empty() {
                why.push(format!(
                    "core {} by {}",
                    node.core.join(" "),
                    node.refuted_by.join(", ")
                ));
            }
            if !node.owed.is_empty() {
                why.push(format!("owes {}", node.owed.join("; ")));
            }
            if node.class != mk {
                why.push(format!("same state as {}", r.name(node.class)));
            }
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                mk.count_ones(),
                md_set(r, mk),
                Class::of(r.status[mk as usize]).as_str(),
                why.join("; ").replace('|', "\\|")
            ));
        }
        out.push('\n');
    } else {
        out.push_str(&format!(
            "Every node is in `region.json` (`region.nodes`, {} records).\n\n",
            r.status.len()
        ));
    }
    out.push_str(&format!(
        "## Cost\n\nroot {:.3} ms · {} nodes {:.3} ms ({:.1} µs each) · topology {:.3} ms. \
         These are the only lines of this directory that differ between two runs.\n",
        r.cost.root.as_secs_f64() * 1e3,
        r.status.len(),
        r.cost.nodes.as_secs_f64() * 1e3,
        r.cost.nodes.as_secs_f64() * 1e6 / r.status.len() as f64,
        r.cost.topology.as_secs_f64() * 1e3,
    ));
    out
}
