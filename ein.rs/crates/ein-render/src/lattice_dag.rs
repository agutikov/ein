//! The commitment-lattice / proof-DAG renderer — `ein.py`'s
//! `render/lattice_dag.py`.
//!
//! The engine produces no ordered search tree; it produces a **set-indexed
//! commitment lattice**. Each visited commitment is a set of hypothesis facts;
//! commitments relate by subset/cover (the Apriori prefix join) and collapse
//! by post-saturation `state_key`. [`render_lattice`] draws that lattice as a
//! DAG — the analogue of the old tree view, but a partial order. Verdict
//! colours: alive grey, dead red, solution green.
//!
//! **On `view = "full"`.** ein.py's full view reads `LatticeProof.kb_index` —
//! the per-commitment `SetNode` DAG — and falls back to the solution frontier,
//! with a note, when it is empty. It is *always* empty: `solve`'s own proof
//! packaging never writes it (only a DAG builder would, via `_record_setnode`,
//! and nothing in the shipping path calls one), which is what the `render
//! lattice --view full` help text tells the user. So the fallback is not an
//! edge case here, it is the behaviour; this port has no `kb_index` and emits
//! the note, which is byte-for-byte what ein.py emits.

use ein_core::pyrepr::{PyValue, repr};
use ein_core::{FactId, Terms};
use ein_infer::solve::LatticeProof;

use crate::dot_util::{digraph_open, fact_label, hashed_id, multiline, quote};
use crate::dump::snapshot::LatticeSnapshot;

/// Why `--view full` draws the solution frontier — the note the DOT carries,
/// and the clause `ein render lattice --view full`'s help text carries.
///
/// It read *"no stored lattice (store_lattice=False) — showing the solution
/// frontier instead"* until M1e S1e.4.6 (`CD-L3`), byte-for-byte what ein.py
/// emitted. That sentence is **false twice over**, not merely stale in an
/// alien spelling: every caller that can reach this line sets `store_lattice =
/// true` (`ein-cli`'s `cmd_lattice`, and both of `shape.rs`'s), and
/// `LatticeProof` has no `kb_index` field for any setting of that flag to
/// populate — so no value of `store_lattice` changes the outcome. The real
/// reason is the one `--view`'s own help gives, and the two surfaces a user
/// meets in one session now say the same thing.
///
/// Keeping ein.py's bytes was an argument whose premise left with the oracle
/// at S1a.10.5. What it cost to stop is a **named re-bless**, priced before it
/// was taken: 271 digest rows of `corpus_shapes.md5` and one line of
/// `dump_snapshot_subset-pruned.txt`, every one of them this single line.
pub const FULL_VIEW_FALLBACK: &str =
    "solve stores no per-commitment SetNode DAG — showing the solution frontier instead";

/// `verdict → (border, fill)`.
fn verdict_style(verdict: Verdict) -> (&'static str, &'static str) {
    match verdict {
        Verdict::Alive => ("#7f7f7f", "#eeeeee"),
        Verdict::Dead => ("#d62728", "#fdeaea"),
        Verdict::Solution => ("#2ca02c", "#e8f6e8"),
    }
}

/// Precedence when two cells merge onto one representative.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Verdict {
    Alive = 0,
    Dead = 1,
    Solution = 2,
}

/// `full` — every visited commitment / state; `solution` — the surviving
/// commitments plus the dead siblings pruned at each layer, the small sub-DAG
/// the trace embeds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LatticeView {
    Full,
    Solution,
}

impl LatticeView {
    pub fn parse(s: &str) -> Option<LatticeView> {
        match s {
            "full" => Some(LatticeView::Full),
            "solution" => Some(LatticeView::Solution),
            _ => None,
        }
    }
}

/// What [`render_lattice`] draws.
///
/// A proof is the richer input: its dead commitments carry the unsat core and
/// the learned clause. A snapshot is the permutation-invariant projection, and
/// it draws a *different picture on purpose* — its solutions and deads are
/// post-saturation **state keys**, so each cell's "commitment" is a whole
/// state. The preferred input when determinism matters, and the one whose
/// picture is stable across `lattice_order_seed`.
pub enum LatticeSource<'a> {
    Proof(&'a LatticeProof),
    Snapshot(&'a LatticeSnapshot),
}

/// One normalised lattice cell.
struct Cell {
    /// The representative commitment.
    rep: Vec<FactId>,
    /// `repr(rep)`, computed **by the caller**, because the two sources spell
    /// it differently and the difference is load-bearing: a proof's
    /// commitments carry raw `Fact.args`, where a nested fact reprs as
    /// `Fact(relation_name=…)`; a snapshot's state keys carry
    /// `canon._hashable_args`, where the same fact reprs as a tuple. The order
    /// of the whole diagram follows this string.
    rep_repr: String,
    commitments: Vec<Vec<FactId>>,
    verdict: Verdict,
    layer: usize,
    unsat_core: Vec<FactId>,
    learned_clause: Vec<FactId>,
}

fn make_cell(
    commitments: Vec<Vec<FactId>>,
    rep_repr: String,
    verdict: Verdict,
    unsat_core: Vec<FactId>,
    learned_clause: Vec<FactId>,
) -> Cell {
    // `min(commits, key=lambda c: (len(c), c))` — every construction site
    // passes exactly one commitment, so the representative is that one.
    let rep = commitments.first().cloned().unwrap_or_default();
    let layer = rep.len();
    Cell {
        rep,
        rep_repr,
        commitments,
        verdict,
        layer,
        unsat_core,
        learned_clause,
    }
}

/// `repr` of a commitment — a tuple of `(relation_name, args)` id tuples.
///
/// `repr`, not the raw tuple, because a representative can hold mixed-argument
/// facts, which have no native total order (P1.21 R1).
pub fn commitment_repr(terms: &Terms, commitment: &[FactId]) -> String {
    let items: Vec<PyValue> = commitment
        .iter()
        .map(|f| {
            let (rel, args) = terms.fact(*f);
            PyValue::Tuple(vec![
                PyValue::Str(terms.sym(rel).to_string()),
                PyValue::Tuple(args.iter().map(|a| terms.py_value(*a)).collect()),
            ])
        })
        .collect();
    repr(&PyValue::Tuple(items))
}

fn snapshot_cells(terms: &Terms, snap: &LatticeSnapshot, view: LatticeView) -> (Vec<Cell>, bool) {
    // Every sort here is by `repr`: state keys and label sets are tuples of
    // heterogeneous tuples with no native total order. `dedup` re-sorts the
    // survivors the same way, so the pre-sort only decides which of two equal
    // representatives wins — and two equal representatives in the same verdict
    // class are the same cell.
    let cell = |c: &[FactId], v: Verdict| {
        make_cell(
            vec![c.to_vec()],
            crate::dump::snapshot::canon_key_repr(terms, c),
            v,
            Vec::new(),
            Vec::new(),
        )
    };
    let mut cells: Vec<Cell> = snap
        .solutions
        .iter()
        .map(|c| cell(c, Verdict::Solution))
        .collect();
    cells.extend(snap.deads.iter().map(|c| cell(c, Verdict::Dead)));
    cells.extend(snap.alive_at_end.iter().map(|c| cell(c, Verdict::Alive)));
    (cells, view != LatticeView::Full)
}

/// Normalise a proof into cells, and say whether a requested `full` view got
/// a real lattice (it never does — see the module docs).
fn proof_cells(terms: &Terms, proof: &LatticeProof, view: LatticeView) -> (Vec<Cell>, bool) {
    let mut cells: Vec<Cell> = proof
        .solutions
        .iter()
        .map(|s| {
            make_cell(
                vec![s.commitment.clone()],
                commitment_repr(terms, &s.commitment),
                Verdict::Solution,
                Vec::new(),
                Vec::new(),
            )
        })
        .collect();
    cells.extend(proof.dead_commitments.iter().map(|d| {
        make_cell(
            vec![d.commitment.clone()],
            commitment_repr(terms, &d.commitment),
            Verdict::Dead,
            d.unsat_core.clone(),
            d.learned_clause.clone(),
        )
    }));
    cells.extend(proof.alive_at_end.iter().map(|a| {
        make_cell(
            vec![a.clone()],
            commitment_repr(terms, a),
            Verdict::Alive,
            Vec::new(),
            Vec::new(),
        )
    }));
    (cells, view != LatticeView::Full)
}

/// Collapse cells sharing a representative, keeping the highest-precedence
/// verdict and any enriching unsat core / learned clause.
fn dedup(cells: Vec<Cell>) -> Vec<Cell> {
    let mut by_rep: Vec<Cell> = Vec::new();
    for c in cells {
        match by_rep.iter().position(|p| p.rep == c.rep) {
            None => by_rep.push(c),
            Some(i) => {
                let prev = &by_rep[i];
                // `max((prev, c), key=rank)` returns the *first* maximum, so
                // `prev` wins a tie.
                let verdict = if c.verdict > prev.verdict {
                    c.verdict
                } else {
                    prev.verdict
                };
                let mut commitments = prev.commitments.clone();
                for cm in &c.commitments {
                    if !commitments.contains(cm) {
                        commitments.push(cm.clone());
                    }
                }
                // `prev.x or c.x` — an empty set is falsy, so it falls through.
                let unsat_core = if prev.unsat_core.is_empty() {
                    c.unsat_core.clone()
                } else {
                    prev.unsat_core.clone()
                };
                let learned_clause = if prev.learned_clause.is_empty() {
                    c.learned_clause.clone()
                } else {
                    prev.learned_clause.clone()
                };
                by_rep[i] = Cell {
                    rep: c.rep,
                    rep_repr: c.rep_repr,
                    commitments,
                    verdict,
                    layer: c.layer,
                    unsat_core,
                    learned_clause,
                };
            }
        }
    }
    // Deterministic order: by `(layer, repr(representative))`.
    by_rep.sort_by(|a, b| (a.layer, &a.rep_repr).cmp(&(b.layer, &b.rep_repr)));
    by_rep
}

// ── labels / ids ───────────────────────────────────────────────────

fn commit_label(terms: &Terms, commitment: &[FactId]) -> String {
    if commitment.is_empty() {
        return "∅".to_string();
    }
    let parts: Vec<String> = commitment.iter().map(|f| fact_label(terms, *f)).collect();
    format!("{{{}}}", parts.join(", "))
}

fn cell_id(terms: &Terms, rep: &[FactId]) -> String {
    if rep.is_empty() {
        return quote("root");
    }
    let seed: Vec<String> = rep.iter().map(|f| fact_label(terms, *f)).collect();
    hashed_id("n_", &seed.join("|"), true)
}

/// A proper-subset test over two commitments, `set(p) < set(rep)`.
fn is_proper_subset(p: &[FactId], rep: &[FactId]) -> bool {
    let sub = p.iter().all(|x| rep.contains(x));
    // Both are canonical (sorted, deduplicated), so a length comparison is
    // the strictness test.
    sub && p.len() < rep.len()
}

// ── the renderer ───────────────────────────────────────────────────

/// Render the commitment lattice as an inline `dot` block.
pub fn render_lattice(
    terms: &Terms,
    source: LatticeSource<'_>,
    view: LatticeView,
    name: &str,
) -> String {
    let (cells, full_ok) = match source {
        LatticeSource::Proof(proof) => proof_cells(terms, proof, view),
        LatticeSource::Snapshot(snap) => snapshot_cells(terms, snap, view),
    };
    let cells = dedup(cells);

    let mut lines = digraph_open(name, Some("LR"), Some("fontname=\"Inter\", shape=box"));
    if view == LatticeView::Full && !full_ok {
        lines.push(format!("  // {FULL_VIEW_FALLBACK}"));
    }

    let root_cell = cells.iter().find(|c| c.rep.is_empty());
    let non_root: Vec<&Cell> = cells.iter().filter(|c| !c.rep.is_empty()).collect();

    // Root node (layer 0), reusing a root cell's verdict colour if present.
    match root_cell {
        Some(c) => {
            let (border, fill) = verdict_style(c.verdict);
            lines.push(format!(
                "  {} [label={}, style=filled, color=\"{border}\", fillcolor=\"{fill}\"];",
                quote("root"),
                multiline(&["∅ root", "(saturation)"])
            ));
        }
        None => lines.push(format!(
            "  {} [label={}, style=filled, color=\"#7f7f7f\", fillcolor=\"#eeeeee\"];",
            quote("root"),
            multiline(&["root", "(saturation)"])
        )),
    }

    // Cell nodes.
    for cell in &non_root {
        let nid = cell_id(terms, &cell.rep);
        let (border, fill) = verdict_style(cell.verdict);
        let extra = cell.commitments.len() - 1;
        let head = commit_label(terms, &cell.rep);
        let tail = format!("(+{extra} ≡ same state)");
        let label_parts: Vec<&str> = if extra > 0 {
            vec![head.as_str(), tail.as_str()]
        } else {
            vec![head.as_str()]
        };
        let mut attrs = vec![
            format!("label={}", multiline(&label_parts)),
            "style=filled".to_string(),
            format!("color=\"{border}\""),
            format!("fillcolor=\"{fill}\""),
        ];
        if !cell.unsat_core.is_empty() {
            let mut core: Vec<String> = cell
                .unsat_core
                .iter()
                .map(|f| fact_label(terms, *f))
                .collect();
            core.sort();
            let mut note = format!("unsat-core: {}", core.join(", "));
            if !cell.learned_clause.is_empty() {
                let mut clause: Vec<String> = cell
                    .learned_clause
                    .iter()
                    .map(|f| fact_label(terms, *f))
                    .collect();
                clause.sort();
                note.push_str(&format!(" | no-good: {}", clause.join(", ")));
            }
            attrs.push(format!("tooltip={}", quote(&note)));
        }
        lines.push(format!("  {nid} [{}];", attrs.join(", ")));
    }

    // Cover edges (subset, differing by one) plus root fallbacks.
    let mut reps_by_layer: Vec<(usize, Vec<Vec<FactId>>)> = Vec::new();
    for cell in &non_root {
        match reps_by_layer.iter_mut().find(|(l, _)| *l == cell.layer) {
            Some(slot) => slot.1.push(cell.rep.clone()),
            None => reps_by_layer.push((cell.layer, vec![cell.rep.clone()])),
        }
    }
    let layer_reps = |layer: usize| -> Vec<Vec<FactId>> {
        reps_by_layer
            .iter()
            .find(|(l, _)| *l == layer)
            .map(|(_, r)| r.clone())
            .unwrap_or_default()
    };
    let parents_of = |cell: &Cell| -> Vec<Vec<FactId>> {
        if cell.layer == 0 {
            return Vec::new();
        }
        layer_reps(cell.layer - 1)
            .into_iter()
            .filter(|p| is_proper_subset(p, &cell.rep))
            .collect()
    };
    for cell in &non_root {
        let parents = parents_of(cell);
        if !parents.is_empty() {
            for p in parents {
                lines.push(format!(
                    "  {} -> {};",
                    cell_id(terms, &p),
                    cell_id(terms, &cell.rep)
                ));
            }
        } else {
            // No shown immediate parent — hang it off the root.
            let style = if cell.layer == 1 {
                ""
            } else {
                " [style=dotted]"
            };
            lines.push(format!(
                "  {} -> {}{style};",
                quote("root"),
                cell_id(terms, &cell.rep)
            ));
        }
    }

    // Dead nodes: a dashed back-edge labelled with the lifted no-good.
    for cell in &non_root {
        if cell.verdict == Verdict::Dead && !cell.learned_clause.is_empty() {
            let parents = parents_of(cell);
            let target = match parents.first() {
                Some(p) => cell_id(terms, p),
                None => quote("root"),
            };
            lines.push(format!(
                "  {} -> {target} [style=dashed, color=\"#d62728\", constraint=false, \
                 label=\"no-good\"];",
                cell_id(terms, &cell.rep)
            ));
        }
    }

    // Rank alignment per layer.
    let mut layers: Vec<usize> = reps_by_layer.iter().map(|(l, _)| *l).collect();
    layers.sort();
    for layer in layers {
        let ids: Vec<String> = layer_reps(layer)
            .iter()
            .map(|r| cell_id(terms, r))
            .collect();
        lines.push(format!("  {{rank=same; {}}}", ids.join(" ")));
    }

    lines.push("}".to_string());
    lines.join("\n")
}
