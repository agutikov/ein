//! `ein solve --dump-region DIR` — the solution region and this run's walk.
//!
//! M1e2 [S1e2.1a.1](../../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//! Three files, documented in
//! [`region_dump.md`](../../../../../docs/kernel/inference/region_dump.md):
//!
//! - `region.json` — `ein_infer::region`'s cube: every node, its two bits and
//!   their reasons, `R`'s lines, holes, components, sub-cubes, `N`, `E`, the
//!   duality, the violations, and [`ein_infer::monotone`]'s verdict;
//! - `walk.json` — what this run did with the lattice: the universe it started
//!   from, every entering, the recorded nodes, the learned clauses and the
//!   facts the search wrote at root, each commitment matched to the cube's
//!   mask by fact text;
//! - `region.md` — the two for a reader, and the cost.
//!
//! The cube and the walk come from **two loads** of one program — the
//! oracle's own, so that its interning cannot renumber the solve's — and are
//! joined by rendered fact text, which is why every commitment here is text.

use std::path::Path;

use ein_core::{FactId, Kb, Terms};
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{Candidate, Class, Outcome, Region};
use ein_infer::solve::Solved;
use ein_infer::verdict::{Answer, Verdict};
use rustc_hash::{FxHashMap, FxHashSet};

use super::json::{Json, dumps_indent};

pub const REGION_SCHEMA: &str = "ein-region/1";
pub const WALK_SCHEMA: &str = "ein-walk/1";

/// What the CLI knows that the two halves do not.
pub struct RegionInputs<'a> {
    pub source: &'a str,
    pub limit: u32,
    /// The run's `--max-set-size` and `-n`: the walk's bounds, which say why a
    /// node was never generated.
    pub max_set_size: u32,
    pub stop_after: Option<u64>,
}

/// Write `region.json`, `walk.json` and `region.md` into `dir`.
pub fn write_region_dir(
    dir: &Path,
    oracle: &Outcome,
    oracle_terms: &Terms,
    walk_terms: &Terms,
    root: &Kb,
    solved: &Solved,
    inputs: &RegionInputs,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let candidates: &[Candidate] = match oracle {
        Outcome::Region(r) => &r.candidates,
        Outcome::Refused { candidates, .. } => candidates,
    };
    let bits: FxHashMap<&str, usize> = candidates
        .iter()
        .enumerate()
        .map(|(i, c)| (c.text.as_str(), i))
        .collect();
    let walk = Walk::of(walk_terms, root, solved, &bits);
    std::fs::write(
        dir.join("region.json"),
        dumps_indent(&region_json(oracle, oracle_terms, inputs)) + "\n",
    )?;
    std::fs::write(
        dir.join("walk.json"),
        dumps_indent(&walk.json(inputs)) + "\n",
    )?;
    std::fs::write(
        dir.join("region.md"),
        region_md(oracle, oracle_terms, &walk, inputs),
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
    Json::obj(vec![
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
                    .map(|s| {
                        Json::obj(vec![
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
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
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

// ── walk.json ──────────────────────────────────────────────────────

/// A commitment of the walk, as text and, where every fact is a coordinate
/// of the cube, as its mask.
struct Set {
    facts: Vec<String>,
    mask: Option<u64>,
}

impl Set {
    fn of(terms: &Terms, ids: &[FactId], bits: &FxHashMap<&str, usize>) -> Set {
        let facts: Vec<String> = ids
            .iter()
            .map(|&f| ein_infer::events::sexpr(terms, f))
            .collect();
        let mask = facts
            .iter()
            .try_fold(0u64, |acc, f| bits.get(f.as_str()).map(|&b| acc | 1 << b));
        Set { facts, mask }
    }

    fn json(&self) -> Json {
        Json::obj(vec![
            ("facts", strs(&self.facts)),
            (
                "mask",
                match self.mask {
                    Some(m) => Json::int(m as i64),
                    None => Json::Null,
                },
            ),
        ])
    }
}

fn mask_json(m: Option<u64>) -> Json {
    match m {
        Some(m) => Json::int(m as i64),
        None => Json::Null,
    }
}

/// One refuted commitment of the walk.
struct Dead {
    set: Set,
    layer: u32,
    outcome: &'static str,
    clause: Vec<String>,
    core: Vec<String>,
}

/// What the run did with the lattice.
struct Walk {
    verdict: &'static str,
    k: usize,
    exhausted: bool,
    monotone: bool,
    proposed: Vec<String>,
    alive0: Vec<String>,
    /// Each entering, its layer, its outcome, and the cube node it stood on:
    /// the commitment plus every promotion before it.
    entered: Vec<(Set, u32, &'static str, Option<u64>)>,
    /// Each recorded state, as what, its debt, and its cube node.
    recorded: Vec<(Set, &'static str, usize, Option<u64>)>,
    /// The forced positives, in order, and after how many enterings.
    promotions: Vec<(String, usize)>,
    dead: Vec<Dead>,
    nogoods: Vec<Set>,
    alive_at_end: Vec<Set>,
    root_writes: Vec<String>,
    has_proof: bool,
}

impl Walk {
    fn of(terms: &Terms, root: &Kb, solved: &Solved, bits: &FxHashMap<&str, usize>) -> Walk {
        let text = |ids: &[FactId]| -> Vec<String> {
            ids.iter()
                .map(|&f| ein_infer::events::sexpr(terms, f))
                .collect()
        };
        let k = solved.answer.k(solved.stats.solution_nodes);
        let mut w = Walk {
            verdict: solved.answer.as_str(),
            k,
            exhausted: solved.stats.exhausted,
            monotone: solved.monotonicity.is_monotone(),
            proposed: text(&solved.universe.proposed),
            alive0: text(&solved.universe.alive0),
            entered: Vec::new(),
            recorded: Vec::new(),
            promotions: Vec::new(),
            dead: Vec::new(),
            nogoods: Vec::new(),
            alive_at_end: Vec::new(),
            root_writes: Vec::new(),
            has_proof: solved.proof.is_some(),
        };
        let Some(proof) = solved.proof.as_ref() else {
            return w;
        };
        let open = matches!(&solved.answer, Answer::Verdict(Verdict::Open { .. }));
        // The root each entering stood on: the promotions before it, as cube
        // bits. A promotion that is not a coordinate leaves no bit, and the
        // node is then still the commitment's own.
        let promoted_before = |at: usize| -> u64 {
            proof
                .promotions
                .iter()
                .filter(|p| p.after_enterings <= at)
                .filter_map(|p| bits.get(ein_infer::events::sexpr(terms, p.fact).as_str()))
                .fold(0u64, |acc, &b| acc | 1 << b)
        };
        for p in &proof.promotions {
            w.promotions
                .push((ein_infer::events::sexpr(terms, p.fact), p.after_enterings));
        }
        for (i, e) in proof.entered.iter().enumerate() {
            let set = Set::of(terms, &e.commitment, bits);
            let at = set.mask.map(|m| m | promoted_before(i));
            w.entered.push((set, e.layer, e.kind.as_str(), at));
        }
        for s in &proof.solutions {
            let kind = if open || (solved.owes.in_scope() && !s.owes.is_empty()) {
                "open"
            } else {
                "model"
            };
            // Where its entering stood, or — for root, which has none — on the
            // root every promotion moved.
            let at = proof
                .entered
                .iter()
                .rposition(|e| {
                    e.commitment == s.commitment && e.kind == ein_infer::commitment::Kind::Alive
                })
                .unwrap_or(proof.entered.len());
            let set = Set::of(terms, &s.commitment, bits);
            let node = set.mask.map(|m| m | promoted_before(at));
            w.recorded.push((set, kind, s.owes.total(), node));
        }
        for d in &proof.dead_commitments {
            w.dead.push(Dead {
                set: Set::of(terms, &d.commitment, bits),
                layer: d.layer,
                outcome: d.kind.as_str(),
                clause: text(&d.learned_clause),
                core: text(&d.unsat_core),
            });
        }
        for g in &proof.learned_nogoods {
            w.nogoods.push(Set::of(terms, g, bits));
        }
        for a in &proof.alive_at_end {
            w.alive_at_end.push(Set::of(terms, a, bits));
        }
        let initial: FxHashSet<FactId> = proof.root_initial.iter().copied().collect();
        let mut writes: Vec<String> = root
            .facts()
            .filter(|f| !initial.contains(f))
            .map(|f| ein_infer::events::sexpr(terms, f))
            .collect();
        writes.sort();
        w.root_writes = writes;
        w
    }

    fn json(&self, inputs: &RegionInputs) -> Json {
        Json::obj(vec![
            ("schema", Json::str(WALK_SCHEMA)),
            ("source", Json::str(inputs.source)),
            (
                "verdict",
                Json::obj(vec![
                    ("type", Json::str(self.verdict)),
                    ("k", Json::int(self.k as i64)),
                    ("exhausted", Json::Bool(self.exhausted)),
                    ("monotone", Json::Bool(self.monotone)),
                ]),
            ),
            (
                "bounds",
                Json::obj(vec![
                    ("max_set_size", Json::int(inputs.max_set_size as i64)),
                    (
                        "solutions",
                        match inputs.stop_after {
                            Some(n) => Json::int(n as i64),
                            None => Json::Null,
                        },
                    ),
                ]),
            ),
            (
                "universe",
                Json::obj(vec![
                    ("proposed", strs(&self.proposed)),
                    ("alive0", strs(&self.alive0)),
                ]),
            ),
            (
                "promotions",
                Json::Array(
                    self.promotions
                        .iter()
                        .map(|(f, at)| {
                            Json::obj(vec![
                                ("fact", Json::str(f)),
                                ("after_enterings", Json::int(*at as i64)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "enterings",
                Json::Array(
                    self.entered
                        .iter()
                        .map(|(s, layer, kind, at)| {
                            Json::obj(vec![
                                ("commitment", s.json()),
                                ("layer", Json::int(*layer as i64)),
                                ("outcome", Json::str(*kind)),
                                ("state_mask", mask_json(*at)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "recorded",
                Json::Array(
                    self.recorded
                        .iter()
                        .map(|(s, kind, owes, at)| {
                            Json::obj(vec![
                                ("commitment", s.json()),
                                ("as", Json::str(*kind)),
                                ("owes", Json::int(*owes as i64)),
                                ("state_mask", mask_json(*at)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "dead",
                Json::Array(
                    self.dead
                        .iter()
                        .map(|d| {
                            Json::obj(vec![
                                ("commitment", d.set.json()),
                                ("layer", Json::int(d.layer as i64)),
                                ("outcome", Json::str(d.outcome)),
                                ("learned_clause", strs(&d.clause)),
                                ("core", strs(&d.core)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "learned_nogoods",
                Json::Array(self.nogoods.iter().map(Set::json).collect()),
            ),
            (
                "alive_at_end",
                Json::Array(self.alive_at_end.iter().map(Set::json).collect()),
            ),
            ("root_writes", strs(&self.root_writes)),
        ])
    }
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

fn region_md(oracle: &Outcome, terms: &Terms, walk: &Walk, inputs: &RegionInputs) -> String {
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
        out.push_str(&format!(
            "**Not monotone: {} site{}.** A commitment can take a conclusion away or add a \
             debt, so the region may have holes and its two lines may not describe it. \
             Everything below that assumes a monotone lattice says so; what the region \
             means here is P1e2.1b's.\n\n",
            m.sites.len(),
            if m.sites.len() == 1 { "" } else { "s" }
        ));
        for s in &m.sites {
            out.push_str(&format!("- {}\n", s.render(terms)));
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
            out.push_str(&walk_md(None, walk));
            return out;
        }
        Outcome::Region(r) => r,
    };
    let t = &r.topology;
    let assumes = if m.is_monotone() {
        ""
    } else {
        " *(assumes a monotone lattice)*"
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
    out.push_str(&walk_md(Some(r), walk));
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

fn walk_md(r: Option<&Region>, walk: &Walk) -> String {
    let mut out = String::from("## This run's walk\n\n");
    out.push_str(&format!(
        "Verdict **{}**, k = {}, exhausted = {}.\n\n",
        walk.verdict, walk.k, walk.exhausted
    ));
    if let Some(r) = r {
        let cube: Vec<&str> = r.candidates.iter().map(|c| c.text.as_str()).collect();
        let proposed: Vec<&str> = walk.proposed.iter().map(String::as_str).collect();
        let mut a = cube.clone();
        a.sort();
        let mut b = proposed.clone();
        b.sort();
        if a != b {
            out.push_str(&format!(
                "- **the run's own universe differs from the cube's**: it proposed {} \
                 candidates against the cube's {}\n",
                proposed.len(),
                cube.len()
            ));
        }
    }
    // determinism-ok: `walk.alive0` is a Vec; this set is membership only.
    let alive0: FxHashSet<&str> = walk.alive0.iter().map(String::as_str).collect();
    let written: FxHashSet<&str> = walk.root_writes.iter().map(String::as_str).collect();
    let (promoted, dropped): (Vec<&String>, Vec<&String>) = walk
        .proposed
        .iter()
        .filter(|f| !alive0.contains(f.as_str()))
        .partition(|f| written.contains(f.as_str()));
    let tick = |v: &[&String]| {
        v.iter()
            .map(|f| format!("`{f}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    out.push_str(&format!(
        "- **alive₀**: {} of the {} proposed\n",
        walk.alive0.len(),
        walk.proposed.len(),
    ));
    if !promoted.is_empty() {
        out.push_str(&format!(
            "  - promoted to root by the forced-positive cascade: {}\n",
            tick(&promoted)
        ));
    }
    if !dropped.is_empty() {
        out.push_str(&format!(
            "  - dropped before the search (the lookahead, or a root negative it wrote): {}\n",
            tick(&dropped)
        ));
    }
    if !walk.has_proof {
        out.push_str("- the run kept no proof, so its enterings are not listed\n\n");
        return out;
    }
    out.push_str(&format!(
        "- **{} enterings**, {} dead; {} learned clause{}; {} fact{} written at root\n",
        walk.entered.len(),
        walk.dead.len(),
        walk.nogoods.len(),
        if walk.nogoods.len() == 1 { "" } else { "s" },
        walk.root_writes.len(),
        if walk.root_writes.len() == 1 { "" } else { "s" },
    ));
    for f in &walk.root_writes {
        out.push_str(&format!("  - `{f}`\n"));
    }
    out.push_str("- **recorded**:\n");
    if walk.recorded.is_empty() {
        out.push_str("  - nothing\n");
    }
    for (s, kind, owes, at) in &walk.recorded {
        let place = match (r, *at) {
            (Some(r), Some(m)) => {
                let t = &r.topology;
                // By state: a commitment whose saturation derives a
                // candidate holds the state of a larger node.
                let class = r.nodes[m as usize].class;
                let on_line =
                    |line: &[u64]| line.iter().any(|&x| r.nodes[x as usize].class == class);
                let mut on = Vec::new();
                if on_line(&t.minimal) {
                    on.push("the minimal line");
                }
                if on_line(&t.maximal) {
                    on.push("the maximal line");
                }
                format!(
                    "{}{} in the cube, {}",
                    if Some(m) != s.mask {
                        format!("as the state {}: ", r.name(m))
                    } else {
                        String::new()
                    },
                    Class::of(r.status[m as usize]).as_str(),
                    if on.is_empty() {
                        "on neither line".to_string()
                    } else {
                        format!("on {}", on.join(" and "))
                    }
                )
            }
            (Some(_), None) => "outside the cube (a fact the root list does not have)".to_string(),
            (None, _) => "no region to place it in".to_string(),
        };
        out.push_str(&format!(
            "  - {} `{{{}}}`{} — {place}\n",
            kind,
            s.facts.join(", "),
            if *owes > 0 {
                format!(", owes {owes}")
            } else {
                String::new()
            }
        ));
    }
    if let Some(r) = r {
        // The overlay's question in one line each — which nodes of each line
        // the run recorded. Which line is *the* answer is S1e2.1a.3's ruling
        // (a), so both are listed.
        // By state, as `ein render region` compares: a line node is recorded
        // when a recorded model holds its fixpoint.
        let recorded: FxHashSet<u64> = walk
            .recorded
            .iter()
            .filter(|x| x.1 == "model")
            .filter_map(|x| x.3)
            .map(|m| r.nodes[m as usize].class)
            .collect();
        for (line, masks) in [
            ("minimal", &r.topology.minimal),
            ("maximal", &r.topology.maximal),
        ] {
            let missing: Vec<u64> = masks
                .iter()
                .copied()
                .filter(|&m| !recorded.contains(&r.nodes[m as usize].class))
                .collect();
            out.push_str(&format!(
                "- **{line} line not recorded**: {}\n",
                md_sets(r, &missing)
            ));
        }
    }
    out.push('\n');
    out
}
