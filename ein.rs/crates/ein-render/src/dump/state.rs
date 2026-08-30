//! `MonotonicDumper` and `ProgressDumper` — `ein.py`'s
//! `inference/monotonic/state_dump.py`.
//!
//! Per-layer filesystem snapshots of a solve, and the live `-v` view over the
//! same hooks. The layout:
//!
//! ```text
//! dump/<puzzle>-<ts>/
//!    00_root_initial.ein           ← root before any enterings
//!    00_timeline.jsonl             ← chronological event log
//!    layers/
//!        layer_01_pre.ein          ← root.kb at layer 1 start
//!        layer_01_post.ein         ← root.kb at layer 1 end
//!        …
//!    summary.json                  ← final stats + verdict
//! ```
//!
//! Reading `00_timeline.jsonl` linearly tells the whole search story; the
//! per-layer snapshots show what root had accumulated at each boundary. No
//! per-commitment folders — that is [`super::lattice`]'s job.

use std::io::Write;
use std::path::Path;

use ein_core::{FactId, Kb, Terms};
use ein_infer::solve::{Dumper, EnteringInfo, LayerCensus, MonotonicStats};
use ein_infer::verdict::Answer;

use super::json::Json;
use super::serialise::{Timeline, kb_to_ein_text};

/// `out_dir = None` skips every filesystem write — the hooks still fire but
/// produce no on-disk artefacts, which is what lets [`ProgressDumper`] reuse
/// the lifecycle stream without paying for the dump.
pub struct MonotonicDumper {
    pub timeline: Timeline,
}

impl MonotonicDumper {
    pub fn new(out_dir: Option<&Path>) -> std::io::Result<MonotonicDumper> {
        Ok(MonotonicDumper {
            timeline: Timeline::new(out_dir)?,
        })
    }
}

/// A commitment as the timeline's list of `{relation, args}` dicts.
pub fn commitment_json(terms: &Terms, commitment: &[FactId]) -> Json {
    Json::Array(
        commitment
            .iter()
            .map(|f| {
                let (rel, args) = terms.fact(*f);
                Json::obj(vec![
                    ("relation", Json::str(terms.sym(rel))),
                    (
                        "args",
                        Json::Array(args.iter().map(|a| Json::str(terms.display(*a))).collect()),
                    ),
                ])
            })
            .collect(),
    )
}

/// Every field of [`MonotonicStats`], in its declaration order — the order
/// `dataclasses.fields` yields, and what `summary.json` sorts.
pub fn stats_json(stats: &MonotonicStats) -> Vec<(&'static str, Json)> {
    let b = &stats.base;
    vec![
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
    ]
}

impl Dumper for MonotonicDumper {
    fn root_initial(&mut self, kb: &Kb, terms: &Terms) {
        if let Some(dir) = self.timeline.out_dir.clone() {
            let _ = std::fs::write(dir.join("00_root_initial.ein"), kb_to_ein_text(kb, terms));
        }
        self.timeline.root_initial(kb);
    }

    /// The beginning of a layer's candidate loop.
    ///
    /// The `pre` file is written **here**, not by the previous `layer_end`:
    /// the spec had `layer_end` writing the next layer's, which leaves a stray
    /// `layer_(N+1)_pre.ein` when layer N is the last one.
    fn layer_start(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize) {
        if let Some(dir) = self.timeline.out_dir.clone() {
            let _ = std::fs::write(
                dir.join("layers").join(format!("layer_{layer:02}_pre.ein")),
                kb_to_ein_text(kb, terms),
            );
        }
        self.timeline.layer_start(layer, n_alive);
    }

    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        self.timeline
            .entering(layer, commitment, terms, outcome, info);
    }

    fn layer_end(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize, n_next: usize) {
        if let Some(dir) = self.timeline.out_dir.clone() {
            let _ = std::fs::write(
                dir.join("layers")
                    .join(format!("layer_{layer:02}_post.ein")),
                kb_to_ein_text(kb, terms),
            );
        }
        self.timeline.layer_end(layer, kb, n_alive, n_next);
    }

    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {
        self.timeline.summary(verdict.as_str(), stats_json(stats));
    }

    fn close(&mut self) {
        self.timeline.close();
    }
}

/// A commitment rendered as its hypothesis facts — `(color-loc Green House-4)`,
/// space-joined at layer ≥ 2, and `∅` for the empty set.
pub fn fmt_commitment(terms: &Terms, commitment: &[FactId]) -> String {
    if commitment.is_empty() {
        return "∅".to_string();
    }
    commitment
        .iter()
        .map(|f| {
            let (rel, args) = terms.fact(*f);
            let inner: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
            // Not `Terms::compact`, whose trailing space is unconditional:
            // this one drops the space for a nullary fact.
            if inner.is_empty() {
                format!("({})", terms.sym(rel))
            } else {
                format!("({} {})", terms.sym(rel), inner.join(" "))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The live progress emitter for slow runs — the `-v` view.
///
/// A [`MonotonicDumper`] by composition rather than by inheritance, so passing
/// an `out_dir` still writes the full filesystem log alongside the live lines.
/// Every `progress_every`-th entering prints; layer boundaries, each solution
/// node and the final summary always print.
pub struct ProgressDumper {
    inner: MonotonicDumper,
    stream: Box<dyn Write>,
    progress_every: u64,
    /// `--layer-progress`: the three layer lines and nothing per entering.
    /// A 618 076-entering run logs 6 180 lines at the default
    /// `--progress-every`, which is a firehose to read a *layer* out of.
    layers_only: bool,
    label: String,
    enterings: u64,
    /// Distinct solution-node states, deduped by canonical `state_key` — the
    /// verdict's `k`, not the raw count of solution-outcome events.
    node_keys: Vec<Box<[FactId]>>,
    /// Phase wall-clock, so this doubles as the CLI's `--timing` source and
    /// `-v` composes with `-t`.
    pub t0: std::time::Instant,
    pub t_root: Option<std::time::Instant>,
    pub t_end: Option<std::time::Instant>,
    pub root_facts: usize,
    /// Root-saturation progress is surfaced only once saturation runs past a
    /// second, then at most once a second — so a fast root adds no noise ahead
    /// of the search, which is the progress that matters.
    last_sat_say: std::time::Instant,
}

impl ProgressDumper {
    pub fn new(
        out_dir: Option<&Path>,
        stream: Box<dyn Write>,
        progress_every: u64,
        label: &str,
    ) -> std::io::Result<ProgressDumper> {
        ProgressDumper::with_volume(out_dir, stream, progress_every, label, false)
    }

    /// [`ProgressDumper::new`], plus the switch that silences the per-entering
    /// line. `layers_only` is `--layer-progress`; `false` is `--verbose`.
    pub fn with_volume(
        out_dir: Option<&Path>,
        stream: Box<dyn Write>,
        progress_every: u64,
        label: &str,
        layers_only: bool,
    ) -> std::io::Result<ProgressDumper> {
        let now = std::time::Instant::now();
        Ok(ProgressDumper {
            inner: MonotonicDumper::new(out_dir)?,
            stream,
            progress_every,
            layers_only,
            label: label.to_string(),
            enterings: 0,
            node_keys: Vec::new(),
            t0: now,
            t_root: None,
            t_end: None,
            root_facts: 0,
            last_sat_say: now,
        })
    }

    fn say(&mut self, msg: &str) {
        let _ = writeln!(self.stream, "{msg}");
        let _ = self.stream.flush();
    }

    /// `f"{elapsed:5.0f}s"` — the elapsed-seconds column.
    fn el(&self) -> String {
        format!("{:5.0}s", self.inner.timeline.elapsed())
    }

    fn head(&self) -> String {
        if self.label.is_empty() {
            String::new()
        } else {
            format!("[{}] ", self.label)
        }
    }
}

impl Dumper for ProgressDumper {
    fn root_saturating(&mut self, n_firings: usize) {
        // Quiet while root saturation is fast; speak only when it is slow
        // enough to look like a hang.
        let now = std::time::Instant::now();
        if now.duration_since(self.last_sat_say).as_secs_f64() < 1.0 {
            return;
        }
        self.last_sat_say = now;
        let (head, el) = (self.head(), self.el());
        self.say(&format!(
            "{head}  saturating root: {n_firings} firings  ({el})"
        ));
    }

    fn root_initial(&mut self, kb: &Kb, terms: &Terms) {
        self.inner.root_initial(kb, terms);
        self.t_root = Some(std::time::Instant::now());
        self.root_facts = kb.n_facts();
        let (head, el, n) = (self.head(), self.el(), kb.n_facts());
        self.say(&format!("{head}root saturated: {n} facts  ({el})"));
    }

    fn layer_start(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize) {
        self.inner.layer_start(layer, kb, terms, n_alive);
        let (el, n) = (self.el(), kb.n_facts());
        self.say(&format!(
            "  layer {layer}: alive={n_alive} root_facts={n}  ({el})"
        ));
    }

    /// The generation half — what the prefix join proposed and what the
    /// learned clauses took off it, before a single fork.
    fn layer_generated(&mut self, layer: u32, c: &LayerCensus) {
        let el = self.el();
        let filt = if c.joined == 0 {
            String::from("—")
        } else {
            format!("{:.1}%", 100.0 * c.dropped_nogood as f64 / c.joined as f64)
        };
        self.say(&format!(
            "  layer {layer} gen:  frontier={} joined={} −dead={} −clause={} ({filt}) \
             cand={}  ({el})",
            c.frontier, c.joined, c.dropped_dead, c.dropped_nogood, c.candidates
        ));
    }

    /// The testing half — the same row the `layer` event carries, for a reader
    /// who has no event stream.
    fn layer_census(&mut self, layer: u32, c: &LayerCensus) {
        let el = self.el();
        let dead = c.dead_pre + c.dead_post;
        // `alive_enterings` counts every consistent fork; only the ones that
        // are *not* already complete reach the next frontier, so the gap is
        // this layer's solution-outcome enterings — and `models` is what they
        // collapse to under `state_key`. On `zebra2` layer 1 that is 13 → 1.
        let complete = c.alive_enterings.saturating_sub(c.next);
        self.say(&format!(
            "  layer {layer} test: entered={} alive={} complete={complete} models={} \
             dead={dead} dead_pre={} clauses={} subsumed={} writebacks={}  ({el})",
            c.entered,
            c.alive_enterings,
            c.models,
            c.dead_pre,
            c.nogoods_emitted,
            c.nogoods_subsumed,
            c.writebacks
        ));
    }

    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        self.inner.entering(layer, commitment, terms, outcome, info);
        self.enterings += 1;
        if outcome == "solution"
            && let Some(kb) = info.kb
        {
            let key = ein_infer::canon::state_key(kb);
            if !self.node_keys.contains(&key) {
                self.node_keys.push(key);
            }
        }
        if self.layers_only {
            return;
        }
        if outcome == "solution" || self.enterings.is_multiple_of(self.progress_every) {
            let (el, e, n) = (self.el(), self.enterings, self.node_keys.len());
            let c = fmt_commitment(terms, commitment);
            self.say(&format!(
                "    e={e:>5} layer={layer}  {c}  -> {outcome:<9} solution-nodes={n}  ({el})"
            ));
        }
    }

    fn layer_end(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize, n_next: usize) {
        self.inner.layer_end(layer, kb, terms, n_alive, n_next);
        let (el, e, n) = (self.el(), self.enterings, self.node_keys.len());
        self.say(&format!(
            "  layer {layer} done: survivors={n_next} enterings={e} solution-nodes={n}  ({el})"
        ));
    }

    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {
        self.inner.summary(verdict, stats);
        self.t_end = Some(std::time::Instant::now());
        let (el, k, ex, total) = (
            self.el(),
            stats.solution_nodes,
            if stats.exhausted { "True" } else { "False" },
            stats.base.enterings_total,
        );
        let kind = verdict.as_str();
        self.say(&format!(
            "  => {kind}  k={k}  exhausted={ex}  enterings={total}  ({el})"
        ));
    }

    fn close(&mut self) {
        self.inner.close();
    }
}
