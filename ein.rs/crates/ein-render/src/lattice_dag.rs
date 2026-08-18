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

/// One normalised lattice cell.
struct Cell {
    /// The representative commitment.
    rep: Vec<FactId>,
    commitments: Vec<Vec<FactId>>,
    verdict: Verdict,
    layer: usize,
    unsat_core: Vec<FactId>,
    learned_clause: Vec<FactId>,
}

fn make_cell(
    commitments: Vec<Vec<FactId>>,
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
fn commitment_repr(terms: &Terms, commitment: &[FactId]) -> String {
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

/// Normalise a proof into cells, and say whether a requested `full` view got
/// a real lattice (it never does — see the module docs).
fn proof_cells(terms: &Terms, proof: &LatticeProof, view: LatticeView) -> (Vec<Cell>, bool) {
    let mut cells: Vec<Cell> = proof
        .solutions
        .iter()
        .map(|s| {
            make_cell(
                vec![s.commitment.clone()],
                Verdict::Solution,
                Vec::new(),
                Vec::new(),
            )
        })
        .collect();
    cells.extend(proof.dead_commitments.iter().map(|d| {
        make_cell(
            vec![d.commitment.clone()],
            Verdict::Dead,
            d.unsat_core.clone(),
            d.learned_clause.clone(),
        )
    }));
    cells.extend(
        proof
            .alive_at_end
            .iter()
            .map(|a| make_cell(vec![a.clone()], Verdict::Alive, Vec::new(), Vec::new())),
    );
    (dedup(terms, cells), view != LatticeView::Full)
}

/// Collapse cells sharing a representative, keeping the highest-precedence
/// verdict and any enriching unsat core / learned clause.
fn dedup(terms: &Terms, cells: Vec<Cell>) -> Vec<Cell> {
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
    let mut keyed: Vec<(usize, String, Cell)> = by_rep
        .into_iter()
        .map(|c| (c.layer, commitment_repr(terms, &c.rep), c))
        .collect();
    keyed.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    keyed.into_iter().map(|(_, _, c)| c).collect()
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
    proof: &LatticeProof,
    view: LatticeView,
    name: &str,
) -> String {
    let (cells, full_ok) = proof_cells(terms, proof, view);

    let mut lines = digraph_open(name, Some("LR"), Some("fontname=\"Inter\", shape=box"));
    if view == LatticeView::Full && !full_ok {
        lines.push(
            "  // no stored lattice (store_lattice=False) — showing the solution frontier instead"
                .to_string(),
        );
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
