//! `LatticeDumper` — `ein.py`'s `inference/monotonic/_lattice_dump.py`.
//!
//! Every commitment tested at every layer, with the firings each one emitted,
//! survivors and casualties alike: the exhaustive audit trail. Sibling of
//! [`super::state::MonotonicDumper`], sharing the lifecycle-hook pattern and
//! adding the proof sections.
//!
//! ```text
//! out_dir/
//! ├── 00_root_initial.ein   ← root before any hypothesis
//! ├── 00_timeline.jsonl     ← lifecycle event log
//! ├── layers/layer_NN/{pre,post}.ein
//! ├── enterings/layer_NN/<C-slug>/
//! │       commitment.json · outcome.txt · firings.jsonl (non-dead-pre)
//! │       kb.ein (solution) · unsat_core.jsonl + learned_clause.json (dead)
//! ├── kb_index/layer_NN/kb_<i>/…   ← under a stored lattice
//! ├── proof_summary.json    ← top-level proof index
//! └── summary.json          ← cumulative stats
//! ```
//!
//! Sub-folders are created lazily on first write, so the layout reflects what
//! actually happened rather than what could have.
//!
//! **`kb_index/` never materialises**, and that is ein.py's behaviour, not a
//! gap: `LatticeProof.kb_index` is written only by a DAG builder via
//! `_record_setnode`, and nothing on the shipping path calls one — the same
//! fact that makes `render lattice --view full` always take its fallback
//! ([S1a.5.1](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.1_dot_renderers.md)).
//! So `proof_summary.json` carries an empty `kb_index` list, which is exactly
//! what ein.py writes.

use std::io::Write;
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, Terms};
use ein_infer::commitment::Kind;
use ein_infer::solve::{Dumper, EnteringInfo, LatticeProof, MonotonicStats};
use ein_infer::verdict::Answer;

use super::json::{Json, dumps, dumps_indent, dumps_indent_sorted};
use super::serialise::{Timeline, fact_summary, firing_to_json, kb_to_ein_text};
use super::state::{commitment_json, stats_json};

/// A commitment as a filesystem-safe slug.
///
/// The empty commitment is `root`; a singleton is the bare fact slug; several
/// join with `+`. Within one fact the arguments join with `_`, and a literal
/// `_` in an identifier becomes `-` so the field separator stays unambiguous.
pub fn commitment_slug(terms: &Terms, commitment: &[FactId]) -> String {
    if commitment.is_empty() {
        return "root".to_string();
    }
    let field = |s: &str| s.to_lowercase().replace('_', "-");
    commitment
        .iter()
        .map(|f| {
            let (rel, args) = terms.fact(*f);
            let mut parts = vec![field(terms.sym(rel))];
            parts.extend(args.iter().map(|a| field(&terms.display(*a))));
            parts.join("_")
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn factid_json(terms: &Terms, f: FactId) -> Json {
    let (rel, args) = terms.fact(f);
    Json::obj(vec![
        ("relation", Json::str(terms.sym(rel))),
        (
            "args",
            Json::Array(args.iter().map(|a| Json::str(terms.display(*a))).collect()),
        ),
    ])
}

pub struct LatticeDumper {
    pub timeline: Timeline,
}

impl LatticeDumper {
    pub fn new(out_dir: Option<&Path>) -> std::io::Result<LatticeDumper> {
        Ok(LatticeDumper {
            timeline: Timeline::new(out_dir)?,
        })
    }

    /// `layers/layer_NN/` — created lazily.
    fn layer_dir(&self, layer: u32) -> Option<PathBuf> {
        let dir = self.timeline.out_dir.as_ref()?;
        let p = dir.join("layers").join(format!("layer_{layer:02}"));
        std::fs::create_dir_all(&p).ok()?;
        Some(p)
    }

    /// `enterings/layer_NN/<slug>/` — created lazily.
    fn entering_dir(&self, layer: u32, terms: &Terms, commitment: &[FactId]) -> Option<PathBuf> {
        let dir = self.timeline.out_dir.as_ref()?;
        let p = dir
            .join("enterings")
            .join(format!("layer_{layer:02}"))
            .join(commitment_slug(terms, commitment));
        std::fs::create_dir_all(&p).ok()?;
        Some(p)
    }
}

impl Dumper for LatticeDumper {
    fn root_initial(&mut self, kb: &Kb, terms: &Terms) {
        if let Some(dir) = self.timeline.out_dir.clone() {
            let _ = std::fs::write(dir.join("00_root_initial.ein"), kb_to_ein_text(kb, terms));
        }
        self.timeline.emit(
            "root_initial",
            vec![("facts", Json::int(kb.n_facts() as i64))],
        );
    }

    fn layer_start(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize) {
        if let Some(dir) = self.layer_dir(layer) {
            let _ = std::fs::write(dir.join("pre.ein"), kb_to_ein_text(kb, terms));
        }
        self.timeline.emit(
            "layer_start",
            vec![
                ("layer", Json::int(layer as i64)),
                ("alive_size", Json::int(n_alive as i64)),
            ],
        );
    }

    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        self.timeline.emit(
            "entering",
            vec![
                ("layer", Json::int(layer as i64)),
                ("outcome", Json::str(outcome)),
                ("commitment", commitment_json(terms, commitment)),
                ("facts_merged", Json::int(info.facts_merged as i64)),
                ("nogood_emitted", Json::Bool(info.nogood_emitted)),
                ("nogood_subsumed", Json::Bool(info.nogood_subsumed)),
                ("kind", Json::str(info.kind.as_str())),
                ("firings", Json::int(info.firings.len() as i64)),
                ("unsat_core_size", Json::int(info.unsat_core.len() as i64)),
            ],
        );

        let Some(folder) = self.entering_dir(layer, terms, commitment) else {
            return;
        };
        let _ = std::fs::write(
            folder.join("commitment.json"),
            dumps_indent(&commitment_json(terms, commitment)),
        );
        let _ = std::fs::write(folder.join("outcome.txt"), outcome);

        if info.kind != Kind::DeadPre {
            // Both alive and dead-post have saturated forks whose firings are
            // meaningful per-hypothesis emissions.
            if let Ok(mut fp) = std::fs::File::create(folder.join("firings.jsonl")) {
                for firing in info.firings {
                    let _ = writeln!(fp, "{}", dumps(&firing_to_json(terms, firing)));
                }
            }
        }

        if outcome == "solution"
            && let Some(kb) = info.kb
        {
            let _ = std::fs::write(folder.join("kb.ein"), kb_to_ein_text(kb, terms));
        }

        if outcome == "dead-pre" || outcome == "dead-post" {
            if let Ok(mut fp) = std::fs::File::create(folder.join("unsat_core.jsonl")) {
                // `sorted`: the core is a set, and its raw iteration order
                // would put these lines in a `PYTHONHASHSEED`-dependent order
                // (hazard H4 — the same bug as `render/slice`'s `⊥` edges, in
                // the other artefact).
                let mut core: Vec<(String, FactId)> = info
                    .unsat_core
                    .iter()
                    .map(|f| (ein_core::pyrepr::repr(&terms.py_fact(*f)), *f))
                    .collect();
                core.sort();
                for (_, f) in core {
                    let _ = writeln!(fp, "{}", dumps(&fact_summary(terms, f)));
                }
            }
            // The learned clause is the commitment, sorted by
            // `(relation_name, tuple(map(str, args)))`.
            let mut clause: Vec<FactId> = commitment.to_vec();
            clause.sort_by_key(|f| {
                let (rel, args) = terms.fact(*f);
                (
                    terms.sym(rel).to_string(),
                    args.iter().map(|a| terms.display(*a)).collect::<Vec<_>>(),
                )
            });
            let _ = std::fs::write(
                folder.join("learned_clause.json"),
                dumps_indent(&Json::Array(
                    clause.iter().map(|f| factid_json(terms, *f)).collect(),
                )),
            );
        }
    }

    fn layer_end(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize, n_next: usize) {
        if let Some(dir) = self.layer_dir(layer) {
            let _ = std::fs::write(dir.join("post.ein"), kb_to_ein_text(kb, terms));
        }
        self.timeline.emit(
            "layer_end",
            vec![
                ("layer", Json::int(layer as i64)),
                ("facts", Json::int(kb.n_facts() as i64)),
                ("alive_size", Json::int(n_alive as i64)),
                ("survived_count", Json::int(n_next as i64)),
            ],
        );
    }

    fn proof_summary(&mut self, proof: &LatticeProof, terms: &Terms) {
        let Some(dir) = self.timeline.out_dir.clone() else {
            return;
        };
        let entry = |slug: String, layer: u32, commitment: &[FactId], kind: Option<&str>| {
            let mut pairs = vec![
                ("slug", Json::str(slug.clone())),
                ("layer", Json::int(layer as i64)),
            ];
            if let Some(kind) = kind {
                pairs.push(("kind", Json::str(kind)));
            }
            pairs.push(("commitment", commitment_json(terms, commitment)));
            pairs.push((
                "path",
                Json::str(format!("enterings/layer_{layer:02}/{slug}")),
            ));
            Json::obj(pairs)
        };
        let summary = Json::obj(vec![
            (
                "solutions",
                Json::Array(
                    proof
                        .solutions
                        .iter()
                        .map(|s| {
                            entry(
                                commitment_slug(terms, &s.commitment),
                                s.layer,
                                &s.commitment,
                                None,
                            )
                        })
                        .collect(),
                ),
            ),
            (
                "dead_commitments",
                Json::Array(
                    proof
                        .dead_commitments
                        .iter()
                        .map(|d| {
                            entry(
                                commitment_slug(terms, &d.commitment),
                                d.layer,
                                &d.commitment,
                                Some(d.kind.as_str()),
                            )
                        })
                        .collect(),
                ),
            ),
            // Empty by construction — see the module docs.
            ("kb_index", Json::Array(Vec::new())),
            (
                "alive_at_end",
                Json::Array(
                    proof
                        .alive_at_end
                        .iter()
                        .map(|c| commitment_json(terms, c))
                        .collect(),
                ),
            ),
            (
                "learned_nogoods_count",
                Json::int(proof.learned_nogoods.len() as i64),
            ),
            ("stats", lattice_stats_json(proof)),
        ]);
        let _ = std::fs::write(
            dir.join("proof_summary.json"),
            dumps_indent_sorted(&summary),
        );
        self.timeline.emit(
            "proof_summary",
            vec![
                ("solutions", Json::int(proof.solutions.len() as i64)),
                (
                    "dead_commitments",
                    Json::int(proof.dead_commitments.len() as i64),
                ),
                ("kb_index_size", Json::int(0)),
            ],
        );
    }

    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {
        self.timeline.summary(verdict.as_str(), stats_json(stats));
    }

    fn close(&mut self) {
        self.timeline.close();
    }
}

/// `LatticeStats`'s fields, in declaration order.
///
/// ein.py's `LatticeStats` *inherits* `MonotonicStats`, so `dataclasses.fields`
/// yields the base's ten first and its own three after; the port composes
/// rather than inherits, so the flattening is written out.
fn lattice_stats_json(proof: &LatticeProof) -> Json {
    let s = &proof.stats;
    let b = &s.base;
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
        ("solutions_found", Json::int(s.solutions_found as i64)),
        ("state_key_merges", Json::int(s.state_key_merges as i64)),
        ("elapsed_seconds", Json::Float(s.elapsed_seconds)),
    ])
}
