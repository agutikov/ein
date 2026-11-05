//! `ein render region DIR` — the cube drawn, with a region run on it.
//! M1e2 [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md)
//! drew the cube and the walk; since
//! [S1e2.1a.7](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md)
//! the run is a region run's (`--visited`, `--answer`). The legend is
//! [`04_dot_rendering.md` § Region rendering](../../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run).
//!
//! **The nodes are the pure cube**: every subset `C` of the root candidates
//! `H₁`, one rank per layer, `L0` at the top, and a thin edge for adding one
//! hypothesis. That cube is how the lattice would be if no rule derived a
//! hypothesised fact. Saturation is drawn **as a move inside it**: a node
//! whose fixpoint holds candidates it did not commit to has a saturation arrow
//! to the node those candidates make, `C ⇒ cl(C)`, and is drawn *transient*
//! (dashed), coloured by the state it lands in; an edge that adds a fact the
//! saturation already derived is *absorbed* (dotted), because it moves
//! nothing. No node is drawn twice — `sat(root + C)` is not a node of its own.
//! What a fixpoint holds that is not a coordinate is counted in the tooltip.
//!
//! One channel per question: **fill** is what the cube says (R by component,
//! owing, dead), **shape** is what the run did (octagon reported, box
//! visited, ellipse neither), **border** is the lines and the defects (double:
//! minimal line, heavy: maximal line, purple: hole, red: missing, orange: not
//! reached, magenta: spurious, the run against the cube, or an unsound
//! exclusion), and **edges** are classed by the statuses at their two ends.
//!
//! Four views ([`View`]), because a cube has `2ⁿ` nodes. The cutoffs are
//! measured, and the stage file says on what.

use std::fmt::Write as _;

use rustc_hash::FxHashSet;

use crate::dot_util::esc;
use crate::region_doc::{Cube, DEAD, OWING, Overlay, RegionDoc, RunDoc, Unvisited, status_word};

/// The whole cube is drawn up to this `n` — 64 nodes, 192 edges.
pub const FULL_MAX_N: usize = 6;
/// *R and its border* is drawn while it has at most this many nodes; past it,
/// the lines.
pub const BORDER_MAX_NODES: usize = 64;
/// A commitment of more facts than this is labelled by its bits.
const LABEL_MAX_FACTS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    /// [`View::Full`] up to [`FULL_MAX_N`], else [`View::Border`] while it has
    /// at most [`BORDER_MAX_NODES`] nodes, else [`View::Lines`].
    Auto,
    /// Every node and every edge of the cube.
    Full,
    /// `R`, its holes, every node one edge from `R`, the root, and what the
    /// run got wrong: a missing node, a spurious state, an unsound exclusion,
    /// a node it read otherwise than the cube. Hidden nodes are counted per
    /// layer.
    Border,
    /// The root, both lines, the holes and what the run got wrong, joined by
    /// inclusion: an edge spanning more than one fact says how many.
    Lines,
    /// Every node as a markdown table, one section per layer — at any `n`.
    Table,
}

impl View {
    pub fn parse(s: &str) -> Option<View> {
        Some(match s {
            "auto" => View::Auto,
            "full" => View::Full,
            "border" => View::Border,
            "lines" => View::Lines,
            "table" => View::Table,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            View::Auto => "auto",
            View::Full => "full",
            View::Border => "border",
            View::Lines => "lines",
            View::Table => "table",
        }
    }
}

/// What `auto` means for this region.
pub fn resolve(doc: &RegionDoc, run: &RunDoc, view: View) -> View {
    if view != View::Auto {
        return view;
    }
    if doc.cube.is_none() {
        return View::Lines;
    }
    if doc.n <= FULL_MAX_N {
        return View::Full;
    }
    let ov = Overlay::of(doc, run);
    if drawn(doc, &ov, View::Border).len() <= BORDER_MAX_NODES {
        View::Border
    } else {
        View::Lines
    }
}

/// Render `view` — DOT, or markdown for [`View::Table`].
pub fn render(doc: &RegionDoc, run: &RunDoc, view: View) -> String {
    let view = resolve(doc, run, view);
    let ov = Overlay::of(doc, run);
    if view == View::Table {
        return table(doc, run, &ov);
    }
    match &doc.cube {
        None => refused_dot(doc, run),
        Some(cube) => dot(doc, cube, run, &ov, view),
    }
}

// ── which nodes ─────────────────────────────────────────────────────

/// ` (1 site): status exercised · no stale fact` — what the title says of a
/// flagged program, as the read-out's warning does (M1e2 S1e2.1b.5).
fn flagged_words(doc: &RegionDoc) -> String {
    let n = doc.sites.len();
    let mut s = format!(" ({n} {})", if n == 1 { "site" } else { "sites" });
    if let Some(status) = &doc.status {
        s.push_str(&format!(": status {status}"));
        match doc.stale_nodes {
            Some(0) => s.push_str(" · no stale fact"),
            Some(1) => s.push_str(" · 1 node holds a stale fact"),
            Some(k) => s.push_str(&format!(" · {k} nodes hold a stale fact")),
            None => {}
        }
    }
    s
}

fn drawn(doc: &RegionDoc, ov: &Overlay, view: View) -> Vec<u64> {
    let Some(cube) = doc.cube.as_ref() else {
        return Vec::new();
    };
    let n = doc.n;
    let size = 1u64 << n;
    let in_r = |m: u64| cube.nodes[m as usize].status == 0;
    let mut keep: FxHashSet<u64> = FxHashSet::default();
    match view {
        View::Full | View::Auto | View::Table => keep.extend(0..size),
        View::Border | View::Lines => {
            keep.insert(0);
            keep.extend(cube.holes.iter().copied());
            keep.extend(cube.minimal.iter().copied());
            keep.extend(cube.maximal.iter().copied());
            // What the run got wrong is drawn whatever the view. What it got
            // right is on the lines, or in R.
            keep.extend(ov.missing.iter().copied());
            keep.extend(ov.spurious.iter().map(|x| x.0));
            keep.extend(ov.disagree.iter().copied());
            for &(d, a) in &ov.unsound {
                keep.insert(d);
                keep.insert(a);
            }
            for (m, mark) in ov.marks.iter().enumerate() {
                if mark.frontier.is_some() {
                    keep.insert(m as u64);
                }
            }
            if view == View::Border {
                for m in 0..size {
                    if in_r(m) || (0..n).any(|b| in_r(m ^ 1 << b)) {
                        keep.insert(m);
                    }
                }
            } else if cube.size_r == 0 {
                // No R to have lines: draw why — the minimal dead and the
                // minimal discharging sets.
                keep.extend(cube.nogoods.iter().copied());
                keep.extend(cube.discharging.iter().copied());
            }
            // Where saturation takes a node of a line, or a reported state:
            // the move the drawing is about. Not every border node's, which
            // would draw half the cube's dead nodes again. And the commitment
            // a drawn state was read from, so its arrow has a tail.
            let closures: Vec<u64> = keep
                .iter()
                .filter(|&&m| {
                    let x = &cube.nodes[m as usize];
                    ov.marks[m as usize].reported() || x.minimal || x.maximal
                })
                .map(|&m| cube.nodes[m as usize].closure)
                .collect();
            keep.extend(closures);
            let tails: Vec<u64> = (0..size)
                .filter(|&m| {
                    !ov.marks[m as usize].read_from.is_empty()
                        && keep.contains(&cube.nodes[m as usize].closure)
                })
                .collect();
            keep.extend(tails);
        }
    }
    // determinism-ok: sorted on the next line.
    let mut v: Vec<u64> = keep.into_iter().collect();
    v.sort_by_key(|m| (m.count_ones(), *m));
    v
}

// ── DOT ─────────────────────────────────────────────────────────────

/// R's fill, one per component.
/// None near owing's amber or dead's salmon, so a component never reads as a
/// status.
const COMPONENT_FILLS: [&str; 6] = [
    "#a1d99b", "#9ecae1", "#bcbddc", "#66c2a4", "#f1b6da", "#d9d9d9",
];
const OWING_FILL: &str = "#fee391";
const DEAD_FILL: &str = "#fcbba1";
const LINE_COLOUR: &str = "#006d2c";
const HOLE_COLOUR: &str = "#6a3d9a";
const MISSING_COLOUR: &str = "#e31a1c";
const SPURIOUS_COLOUR: &str = "#ae017e";
const SATURATION_COLOUR: &str = "#08519c";
const UNREACHED_COLOUR: &str = "#fd8d3c";

fn fill(cube: &Cube, m: u64) -> String {
    let node = &cube.nodes[m as usize];
    match (node.status & DEAD != 0, node.status & OWING != 0) {
        (false, false) => {
            let c = node.component.unwrap_or(0) as usize;
            if cube.components.len() <= 1 {
                COMPONENT_FILLS[0].to_string()
            } else {
                COMPONENT_FILLS[c % COMPONENT_FILLS.len()].to_string()
            }
        }
        (false, true) => OWING_FILL.to_string(),
        (true, false) => DEAD_FILL.to_string(),
        (true, true) => format!("{DEAD_FILL}:{OWING_FILL}"),
    }
}

/// An edge's class: the statuses at its two ends, `dead` before `owing`.
fn class_of(s: u8) -> &'static str {
    if s & DEAD != 0 {
        "dead"
    } else if s & OWING != 0 {
        "owing"
    } else {
        "R"
    }
}

/// `(colour, style, penwidth)` per edge class, and the sentence the legend
/// gives it.
pub const EDGE_CLASSES: [(&str, &str, &str, &str, &str); 9] = [
    ("R", "R", "#238b45", "solid", "inside R"),
    (
        "R",
        "owing",
        "#fd8d3c",
        "dashed",
        "adding the fact adds a debt",
    ),
    ("R", "dead", "#cb181d", "solid", "adding the fact kills"),
    (
        "owing",
        "R",
        "#2171b5",
        "solid",
        "adding the fact pays the last debt",
    ),
    ("owing", "owing", "#fdae6b", "solid", "still owing"),
    ("owing", "dead", "#cb181d", "dashed", "kills an owing node"),
    ("dead", "R", "#ae017e", "bold", "revives a dead node"),
    (
        "dead",
        "owing",
        "#ae017e",
        "dashed",
        "revives a dead node, owing",
    ),
    ("dead", "dead", "#bdbdbd", "solid", "dead to dead"),
];

fn edge_style(from: u8, to: u8) -> (&'static str, &'static str, f32) {
    let (a, b) = (class_of(from), class_of(to));
    let c = EDGE_CLASSES
        .iter()
        .find(|c| c.0 == a && c.1 == b)
        .expect("nine classes");
    let width = match (a, b) {
        ("R", "R") | ("owing", "R") => 2.0,
        ("dead", "R") => 2.5,
        ("dead", "dead") => 0.6,
        _ => 1.2,
    };
    (c.2, c.3, width)
}

fn label_lines(doc: &RegionDoc, m: u64) -> Vec<String> {
    let facts = doc.facts(m);
    if facts.is_empty() {
        return vec!["∅".to_string()];
    }
    if facts.len() > LABEL_MAX_FACTS {
        let bits: Vec<String> = (0..doc.n)
            .filter(|b| m >> b & 1 == 1)
            .map(|b| format!("#{b}"))
            .collect();
        return vec![format!("{} facts", facts.len()), bits.join(" ")];
    }
    facts.iter().map(|f| f.to_string()).collect()
}

fn dot_label(lines: &[String]) -> String {
    lines.iter().map(|l| esc(l)).collect::<Vec<_>>().join("\\n")
}

fn subcubes_of(cube: &Cube, m: u64) -> Vec<usize> {
    cube.subcubes
        .as_ref()
        .map(|v| {
            v.iter()
                .enumerate()
                .filter(|(_, (a, b))| m & a == *a && m & !b == 0)
                .map(|(i, _)| i + 1)
                .collect()
        })
        .unwrap_or_default()
}

fn node_dot(out: &mut String, doc: &RegionDoc, cube: &Cube, ov: &Overlay, m: u64) {
    let node = &cube.nodes[m as usize];
    let mark = &ov.marks[m as usize];
    let mut label = label_lines(doc, m);
    let mut status = status_word(node.status).to_string();
    if node.owes > 0 {
        status = format!("{status} {}", node.owes);
    }
    let mut lines_on = Vec::new();
    if node.minimal {
        lines_on.push("min");
    }
    if node.maximal {
        lines_on.push("max");
    }
    if node.hole {
        lines_on.push("hole");
    }
    if !lines_on.is_empty() {
        status = format!("{status} · {}", lines_on.join(" · "));
    }
    label.push(status);
    let tag = mark.tag();
    if !tag.is_empty() {
        label.push(tag.clone());
    }
    // A dead fixpoint holds everything, so its "move" says nothing: the
    // tooltip keeps it, the drawing does not.
    let moves = node.closure != m && node.status & DEAD == 0;
    if moves {
        label.push(format!(
            "⇒ +{} by saturation",
            (node.closure & !m).count_ones()
        ));
    }
    let missing = ov.missing.contains(&m);
    let unreached = ov.unreached.contains(&m);
    let spurious = ov.is_spurious(m);
    if missing {
        label.push("MISSING".to_string());
    }
    if unreached {
        label.push("not reached".to_string());
    }
    if spurious.is_some() {
        label.push("SPURIOUS".to_string());
    }
    let unsound = ov.is_unsound(m);
    if unsound.is_some() {
        label.push("✗ unsound exclusion".to_string());
    }
    let local = cube.local_max.contains(&m) || cube.local_min.contains(&m);
    if local {
        label.push("≠ local test".to_string());
    }
    if mark.disagrees.is_some() {
        label.push("≠ the cube".to_string());
    }

    // The tooltip: everything the node's reason is.
    let mut tip: Vec<String> = vec![format!("{} — {}", doc.name(m), status_word(node.status))];
    if !node.core.is_empty() {
        tip.push(format!("core: {}", node.core.join(" ")));
        if let Some(q) = node.core_status.and_then(|c| c.qualifier()) {
            tip.push(format!("core search: {q}"));
        }
        tip.push(format!("refuted by: {}", node.refuted_by.join(", ")));
    }
    if !node.absent_premises.is_empty() {
        tip.push(format!("rests on: {}", node.absent_premises.join(", ")));
    }
    for o in &node.owed {
        tip.push(format!("owes: {o}"));
    }
    if node.closure != m {
        tip.push(format!(
            "saturation derives {} → {}",
            doc.facts(node.closure & !m).join(", "),
            doc.name(node.closure)
        ));
    }
    if node.excluded != 0 {
        tip.push(format!(
            "derives the negation of {}",
            doc.facts(node.excluded).join(", ")
        ));
    }
    if node.beyond > 0 {
        tip.push(format!(
            "{} derived facts are not coordinates ({} over a candidate relation)",
            node.beyond, node.beyond_hypothesised
        ));
    }
    if node.class != m {
        tip.push(format!("same state as {}", doc.name(node.class)));
    }
    let subs = subcubes_of(cube, m);
    if !subs.is_empty() {
        tip.push(format!(
            "in sub-cube {}",
            subs.iter()
                .map(|s| format!("S{s}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if ov.run {
        tip.push(format!("run: {}", if tag.is_empty() { "—" } else { &tag }));
        if let Some(k) = &mark.visited {
            tip.push(format!("  visited, read {k}"));
        }
        if let Some(Unvisited::AboveDead(d)) = mark.unvisited {
            tip.push(format!("  not visited: above the dead {}", doc.name(d)));
        }
        if missing {
            tip.push(format!(
                "MISSING: a node of {}, and no state the run reported is its state",
                ov.part
            ));
        }
        if unreached {
            tip.push(format!(
                "not reached: a node of {}, and the run stopped before reporting its state",
                ov.part
            ));
        }
        if let Some(w) = spurious {
            tip.push(format!("SPURIOUS: reported, and {w}"));
        }
    }
    if let Some(d) = &mark.disagrees {
        tip.push(format!("≠ {d}"));
    }
    if let Some(a) = unsound {
        tip.push(format!(
            "✗ the run visited it dead and not {}, which is above it and alive in the cube",
            doc.name(a)
        ));
    }

    let shape = if mark.reported() {
        "octagon"
    } else if mark.visited.is_some() {
        "box"
    } else {
        "ellipse"
    };
    let mut style = vec!["filled"];
    if shape == "box" {
        style.push("rounded");
    }
    if moves {
        style.push("dashed");
    }
    let (colour, width) = if missing {
        (MISSING_COLOUR, 3.0)
    } else if spurious.is_some() || unsound.is_some() || mark.disagrees.is_some() {
        (SPURIOUS_COLOUR, 3.0)
    } else if unreached {
        (UNREACHED_COLOUR, 2.5)
    } else if node.hole {
        (HOLE_COLOUR, 2.5)
    } else if node.maximal {
        (LINE_COLOUR, 3.0)
    } else if node.minimal {
        (LINE_COLOUR, 1.5)
    } else {
        ("#525252", 1.0)
    };
    let mut fillc = fill(cube, m);
    if moves && !fillc.contains(':') {
        fillc.push_str("99");
    }
    let _ = writeln!(
        out,
        "  m{m} [label=\"{}\", shape={shape}, style=\"{}\", fillcolor=\"{fillc}\", \
         color=\"{colour}\", penwidth={width}{}, tooltip=\"{}\"];",
        dot_label(&label),
        style.join(","),
        if node.minimal { ", peripheries=2" } else { "" },
        dot_label(&tip),
    );
}

fn dot(doc: &RegionDoc, cube: &Cube, run: &RunDoc, ov: &Overlay, view: View) -> String {
    let n = doc.n;
    let nodes = drawn(doc, ov, view);
    let shown: FxHashSet<u64> = nodes.iter().copied().collect();
    let mut out = String::new();
    let _ = writeln!(out, "digraph region {{");
    let _ = writeln!(out, "  // ein render region — view {}", view.as_str());
    let title = vec![
        format!("region · {}", doc.source),
        format!(
            "n = {n} · |R| = {} of {} · {}",
            cube.size_r,
            1u64 << n,
            if doc.monotone {
                "monotone".to_string()
            } else {
                format!("NOT monotone{}", flagged_words(doc))
            }
        ),
        format!(
            "{} · view {} ({} of {} nodes)",
            run.line(),
            view.as_str(),
            nodes.len(),
            1u64 << n
        ),
    ];
    let _ = writeln!(
        out,
        "  graph [rankdir=TB, labelloc=t, fontsize=12, nodesep=0.25, ranksep=0.6, \
         newrank=true, label=\"{}\"];",
        dot_label(&title)
    );
    let _ = writeln!(out, "  node [fontsize=10, fontname=\"Helvetica\"];");
    let _ = writeln!(out, "  edge [arrowsize=0.6];");

    // The rank column: one label per layer, with what the view hides — and
    // one label for a run of layers with nothing drawn, so a view of a few
    // nodes in a deep cube is not mostly empty ranks.
    let mut drawn_at = vec![0u64; n + 1];
    for m in &nodes {
        drawn_at[m.count_ones() as usize] += 1;
    }
    let mut hidden = vec![[0u64; 3]; n + 1];
    for m in 0..1u64 << n {
        if !shown.contains(&m) {
            let c = match class_of(cube.nodes[m as usize].status) {
                "R" => 0,
                "owing" => 1,
                _ => 2,
            };
            hidden[m.count_ones() as usize][c] += 1;
        }
    }
    // (first layer, last layer) of each band, in order.
    let mut bands: Vec<(usize, usize)> = Vec::new();
    for k in 0..=n {
        match bands.last_mut() {
            Some(b) if drawn_at[k] == 0 && drawn_at[b.1] == 0 => b.1 = k,
            _ => bands.push((k, k)),
        }
    }
    let band_of = |k: usize| {
        bands
            .iter()
            .position(|b| b.0 <= k && k <= b.1)
            .expect("every layer")
    };
    for (i, &(a, b)) in bands.iter().enumerate() {
        let total: u64 = (a..=b).map(|k| binom(n, k)).sum();
        let [r, o, d] = (a..=b).fold([0u64; 3], |acc, k| {
            [
                acc[0] + hidden[k][0],
                acc[1] + hidden[k][1],
                acc[2] + hidden[k][2],
            ]
        });
        let name = if a == b {
            format!("L{a}")
        } else {
            format!("L{a}–L{b}")
        };
        let mut text = vec![format!("{name} · {total}")];
        if r + o + d > 0 {
            text.push(format!("{} drawn", total - r - o - d));
            text.push(format!("hidden: {r} R, {o} owing, {d} dead"));
        }
        let _ = writeln!(
            out,
            "  L{i} [shape=plaintext, fontsize=9, label=\"{}\"];",
            dot_label(&text)
        );
    }
    for i in 1..bands.len() {
        let _ = writeln!(out, "  L{} -> L{i} [style=invis];", i - 1);
    }

    for &m in &nodes {
        node_dot(&mut out, doc, cube, ov, m);
    }
    for (i, _) in bands.iter().enumerate() {
        let ids: Vec<String> = nodes
            .iter()
            .filter(|m| band_of(m.count_ones() as usize) == i)
            .map(|m| format!("m{m}"))
            .collect();
        if !ids.is_empty() {
            let _ = writeln!(out, "  {{ rank=same; L{i}; {} }}", ids.join("; "));
        }
    }
    let last_band = bands.len() - 1;

    // The cube's edges, or — in the lines view — inclusion between drawn nodes.
    let repairs: Vec<(u64, usize, u64)> = cube.repairs.clone();
    let edge = |out: &mut String, a: u64, c: u64| {
        let (sa, sc) = (cube.nodes[a as usize].status, cube.nodes[c as usize].status);
        let added = c & !a;
        if added.count_ones() == 1 {
            let b = added.trailing_zeros() as usize;
            let fact = &doc.candidates[b];
            let absorbed = cube.nodes[a as usize].closure & added != 0;
            let tip = format!(
                "{} + {} → {}: {} → {}{}",
                doc.name(a),
                fact,
                doc.name(c),
                class_of(sa),
                class_of(sc),
                if absorbed {
                    " (absorbed: saturation already derived it)"
                } else {
                    ""
                }
            );
            if absorbed {
                let _ = writeln!(
                    out,
                    "  m{a} -> m{c} [color=\"#969696\", style=dotted, arrowhead=none, \
                     penwidth=0.8, tooltip=\"{}\"];",
                    esc(&tip)
                );
                return;
            }
            let (colour, style, width) = edge_style(sa, sc);
            let repair = repairs
                .iter()
                .any(|&(h, rb, y)| h == a && rb == b && y == c);
            let _ = writeln!(
                out,
                "  m{a} -> m{c} [color=\"{colour}\", style={style}, penwidth={width}{}, tooltip=\"{}\"];",
                if repair {
                    format!(
                        ", label=\"+{}\", fontcolor=\"{HOLE_COLOUR}\", fontsize=9",
                        esc(fact)
                    )
                } else {
                    String::new()
                },
                esc(&tip)
            );
        } else {
            let _ = writeln!(
                out,
                "  m{a} -> m{c} [color=\"#969696\", style=dashed, label=\"+{}\", \
                 fontsize=8, fontcolor=\"#636363\", tooltip=\"{} ⊂ {}\"];",
                added.count_ones(),
                esc(&doc.name(a)),
                esc(&doc.name(c))
            );
        }
    };
    if view == View::Lines {
        for (i, &a) in nodes.iter().enumerate() {
            for &c in &nodes[i + 1..] {
                if a & c != a || a == c {
                    continue;
                }
                let between = nodes
                    .iter()
                    .any(|&b| b != a && b != c && a & b == a && b & c == b);
                if !between {
                    edge(&mut out, a, c);
                }
            }
        }
    } else {
        for &a in &nodes {
            for b in 0..n {
                let c = a | 1 << b;
                if c != a && shown.contains(&c) {
                    edge(&mut out, a, c);
                }
            }
        }
    }

    // Saturation, as a move inside the cube.
    for &a in &nodes {
        let cl = cube.nodes[a as usize].closure;
        if cl != a && cube.nodes[a as usize].status & DEAD == 0 && shown.contains(&cl) {
            let _ = writeln!(
                out,
                "  m{a} -> m{cl} [color=\"{SATURATION_COLOUR}\", penwidth=2.2, arrowhead=vee, \
                 constraint=false, label=\"sat\", fontsize=8, fontcolor=\"{SATURATION_COLOUR}\", \
                 tooltip=\"sat(root + {}) holds {}\"];",
                esc(&doc.name(a)),
                esc(&doc.facts(cl & !a).join(", "))
            );
        }
    }
    // What an unsound exclusion left unvisited.
    for &(d, a) in &ov.unsound {
        let _ = writeln!(
            out,
            "  m{d} -> m{a} [color=\"{SPURIOUS_COLOUR}\", style=dashed, penwidth=2, \
             constraint=false, label=\"✗ alive\", fontsize=8, fontcolor=\"{SPURIOUS_COLOUR}\", \
             tooltip=\"the run visited {} dead and not {}, which is above it and alive\"];",
            esc(&doc.name(d)),
            esc(&doc.name(a))
        );
    }

    legend(&mut out, doc, cube, ov, view, last_band);
    let _ = writeln!(out, "}}");
    out
}

fn html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// A node as the legend names it: its facts while they are short, its bits
/// once the drawing labels nodes by bit.
fn short(doc: &RegionDoc, m: u64) -> String {
    if doc.n <= LABEL_MAX_FACTS {
        return doc.name(m);
    }
    if m == 0 {
        return "∅".to_string();
    }
    let bits: Vec<String> = (0..doc.n)
        .filter(|b| m >> b & 1 == 1)
        .map(|b| format!("#{b}"))
        .collect();
    format!("{{{}}}", bits.join(" "))
}

/// At most this many entries of one list in the legend; the table has all.
const LEGEND_LIST_MAX: usize = 12;
/// A legend note is wrapped at this many characters.
const LEGEND_WRAP: usize = 110;

fn listed(doc: &RegionDoc, ms: &[u64]) -> String {
    if ms.is_empty() {
        return "none".to_string();
    }
    let mut v: Vec<String> = ms
        .iter()
        .take(LEGEND_LIST_MAX)
        .map(|&m| short(doc, m))
        .collect();
    if ms.len() > LEGEND_LIST_MAX {
        v.push(format!(
            "… {} more (--view table)",
            ms.len() - LEGEND_LIST_MAX
        ));
    }
    v.join(", ")
}

/// A note as HTML, wrapped at word boundaries.
fn wrapped(text: &str) -> String {
    let mut lines: Vec<String> = vec![String::new()];
    for word in text.split(' ') {
        let cur = lines.last_mut().expect("one line");
        if !cur.is_empty() && cur.chars().count() + 1 + word.chars().count() > LEGEND_WRAP {
            lines.push(word.to_string());
        } else {
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(word);
        }
    }
    lines
        .iter()
        .map(|l| html(l))
        .collect::<Vec<_>>()
        .join("<BR ALIGN=\"LEFT\"/>")
        + "<BR ALIGN=\"LEFT\"/>"
}

fn legend(
    out: &mut String,
    doc: &RegionDoc,
    cube: &Cube,
    ov: &Overlay,
    view: View,
    last_band: usize,
) {
    // The key: two columns, never wider than its own rows.
    let mut key: Vec<String> = Vec::new();
    let row = |key: &mut Vec<String>, a: String, b: &str| {
        key.push(format!(
            "<TR><TD ALIGN=\"LEFT\">{a}</TD><TD ALIGN=\"LEFT\">{}</TD></TR>",
            html(b)
        ));
    };
    for (c, what) in [
        (
            COMPONENT_FILLS[0],
            "R: alive and discharged (one colour per component)",
        ),
        (OWING_FILL, "owing: alive, an obligation unwitnessed"),
        (DEAD_FILL, "dead: derives (false); dead+owing is both"),
    ] {
        key.push(format!(
            "<TR><TD BGCOLOR=\"{c}\"> </TD><TD ALIGN=\"LEFT\">{}</TD></TR>",
            html(what)
        ));
    }
    for (a, b) in [
        (
            "dashed outline",
            "transient: saturation moves it (its sat arrow); it is the state it lands on",
        ),
        (
            "octagon",
            "a state the run reported, or a node of an empty R's owing frontier",
        ),
        ("rounded box", "a node the run visited"),
        (
            "ellipse",
            "a node the run did not visit; why is under the facts",
        ),
        ("double border", "R's minimal line"),
        ("heavy green border", "R's maximal line"),
        ("purple border", "a hole: outside R, between two members"),
        (
            "red border",
            "MISSING: in the part of R the run asked, no reported state is its state, and the run says it is complete",
        ),
        (
            "orange border",
            "not reached: the same, on a run that stopped (a bound, not a defect)",
        ),
        (
            "magenta border",
            "SPURIOUS (a reported state outside the part asked), the run's reading against the cube's, or an unsound exclusion",
        ),
    ] {
        row(&mut key, format!("<I>{}</I>", html(a)), b);
    }
    for (from, to, colour, style, what) in EDGE_CLASSES {
        row(
            &mut key,
            format!("<FONT COLOR=\"{colour}\"><B>━━</B></FONT> {from} → {to} <I>{style}</I>"),
            what,
        );
    }
    row(
        &mut key,
        "<FONT COLOR=\"#969696\"><B>┈┈</B></FONT> absorbed <I>dotted</I>".to_string(),
        "adds a fact saturation already derived: no move",
    );
    row(
        &mut key,
        format!("<FONT COLOR=\"{SATURATION_COLOUR}\"><B>━▶</B></FONT> sat"),
        "C ⇒ cl(C): the candidates sat(root + C) holds",
    );
    if !cube.repairs.is_empty() {
        row(
            &mut key,
            format!("<FONT COLOR=\"{HOLE_COLOUR}\">+fact</FONT>"),
            "a repair edge: the fact that takes a hole back into R",
        );
    }
    if !ov.unsound.is_empty() {
        row(
            &mut key,
            format!("<FONT COLOR=\"{SPURIOUS_COLOUR}\"><B>╌▶</B></FONT> ✗ alive"),
            "from a dead node the run visited to an unvisited node above it the cube says is alive",
        );
    }
    if view == View::Lines {
        row(
            &mut key,
            "<FONT COLOR=\"#969696\"><B>╌╌</B></FONT> +k".to_string(),
            "inclusion across k facts, the nodes between hidden",
        );
    }

    // The notes: one column, wrapped.
    let mut notes: Vec<String> = Vec::new();
    if ov.answered {
        notes.push(format!(
            "compared with {}, by state — ruling (a): the answer is R, and a run answers the part of it its question asked",
            ov.part
        ));
        let spurious: Vec<u64> = ov.spurious.iter().map(|x| x.0).collect();
        notes.push(format!(
            "missing {}; spurious {}{}",
            listed(doc, &ov.missing),
            listed(doc, &spurious),
            if ov.unreached.is_empty() {
                String::new()
            } else {
                format!("; not reached {}", listed(doc, &ov.unreached))
            }
        ));
    }
    for (i, line) in ["minimal", "maximal"].iter().enumerate() {
        let on = if i == 0 { &cube.minimal } else { &cube.maximal };
        notes.push(format!("{line} line ({}): {}", on.len(), listed(doc, on)));
    }
    match &cube.subcubes {
        Some(v) => {
            for (i, (a, b)) in v.iter().enumerate().take(LEGEND_LIST_MAX) {
                notes.push(format!(
                    "sub-cube S{} [{} … {}], dimension {}{}",
                    i + 1,
                    short(doc, *a),
                    short(doc, *b),
                    (a ^ b).count_ones(),
                    if a ^ b != 0 {
                        format!(", free {}", short(doc, a ^ b))
                    } else {
                        String::new()
                    }
                ));
            }
            if v.len() > LEGEND_LIST_MAX {
                notes.push(format!(
                    "… {} more sub-cubes (--view table)",
                    v.len() - LEGEND_LIST_MAX
                ));
            }
        }
        None => notes.push("sub-cubes: not computed (holes, past the work bound)".to_string()),
    }
    for &h in cube.minimal_holes.iter().take(LEGEND_LIST_MAX) {
        let node = &cube.nodes[h as usize];
        let why = if !node.absent_premises.is_empty() {
            format!("its absent guard {}", node.absent_premises.join(", "))
        } else if !node.owed.is_empty() {
            format!("no absent guard; it owes {}", node.owed.join("; "))
        } else {
            "no absent guard and no debt recorded".to_string()
        };
        notes.push(format!("minimal hole {}: {why}", short(doc, h)));
    }
    for &(d, a) in ov.unsound.iter().take(LEGEND_LIST_MAX) {
        notes.push(format!(
            "✗ unsound exclusion: {} visited dead — {} above it is alive and was not visited",
            short(doc, d),
            short(doc, a)
        ));
    }
    if ov.unsound.len() > LEGEND_LIST_MAX {
        notes.push(format!(
            "… {} more unsound exclusions (--view table)",
            ov.unsound.len() - LEGEND_LIST_MAX
        ));
    }
    if !ov.disagree.is_empty() {
        notes.push(format!(
            "≠ the cube: the run read {} otherwise",
            listed(doc, &ov.disagree)
        ));
    }
    if !ov.outside.is_empty() {
        notes.push(format!(
            "reported outside the cube: {}",
            ov.outside.join("; ")
        ));
    }
    if !doc.monotone {
        notes.push(format!(
            "not monotone — sites: {}; nodes by fork from root, what a node is here is open (F21)",
            doc.sites.join("; ")
        ));
    }
    if doc.n > LABEL_MAX_FACTS {
        for (i, c) in doc.candidates.iter().enumerate() {
            notes.push(format!("#{i} {c}"));
        }
    }
    let notes: String = notes
        .iter()
        .map(|t| {
            format!(
                "<TR><TD ALIGN=\"LEFT\" BALIGN=\"LEFT\">{}</TD></TR>",
                wrapped(t)
            )
        })
        .collect();
    let _ = writeln!(
        out,
        "  legend [shape=plaintext, label=<<TABLE BORDER=\"0\" CELLSPACING=\"6\">\
         <TR><TD ALIGN=\"LEFT\"><B>legend</B> — fill: the cube · shape: the run · border: the lines and the defects</TD></TR>\
         <TR><TD ALIGN=\"LEFT\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"3\">{}</TABLE></TD></TR>\
         <TR><TD ALIGN=\"LEFT\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"3\">{notes}</TABLE></TD></TR>\
         </TABLE>>];",
        key.join("")
    );
    let _ = writeln!(out, "  {{ rank=sink; legend; }}");
    let _ = writeln!(out, "  L{last_band} -> legend [style=invis];");
}

fn refused_dot(doc: &RegionDoc, run: &RunDoc) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "digraph region {{");
    let _ = writeln!(out, "  // ein render region — view none (no cube)");
    let mut lines = vec![
        format!("region · {}", doc.source),
        format!("n = {} — no cube", doc.n),
        doc.refused.clone().unwrap_or_default(),
        run.line(),
    ];
    // The run alone: what it reported, with nothing to hold it to.
    for s in run.states.iter().take(LEGEND_LIST_MAX) {
        lines.push(format!(
            "state {}: {}",
            s.number,
            if s.closure.is_empty() {
                "∅".to_string()
            } else {
                format!("{{{}}}", s.closure.join(", "))
            }
        ));
    }
    if run.states.len() > LEGEND_LIST_MAX {
        lines.push(format!(
            "… {} more states (--view table)",
            run.states.len() - LEGEND_LIST_MAX
        ));
    }
    let _ = writeln!(
        out,
        "  note [shape=note, fontsize=10, label=\"{}\"];",
        dot_label(&lines)
    );
    let _ = writeln!(out, "}}");
    out
}

fn binom(n: usize, k: usize) -> u64 {
    let mut r = 1u64;
    for i in 0..k {
        r = r * (n - i) as u64 / (i + 1) as u64;
    }
    r
}

// ── the table ───────────────────────────────────────────────────────

fn md(s: &str) -> String {
    s.replace('|', "\\|")
}

/// A node as a link to its row: every row carries `<a id="mMASK">`, so the
/// table's head links its lists into the table, and a page that splits the
/// table by layer can follow the anchor.
fn node_link(doc: &RegionDoc, m: u64) -> String {
    format!("[`{}`](#m{m})", md(&doc.name(m)))
}

/// The table's head names at most this many nodes per list. Every row's
/// status or run cell carries the same mark, so the rest are a search away;
/// a run a budget stopped on a 2²⁰ cube that is all R leaves a million nodes
/// *not reached*, and the head listed every one (S1e2.1a.7's gallery, 2026-11-01).
pub const HEAD_LIST_MAX: usize = 50;

/// At most [`HEAD_LIST_MAX`] items, and how many there are when more.
fn capped(items: Vec<String>, rows_say: &str) -> String {
    let n = items.len();
    if n == 0 {
        return "none".to_string();
    }
    let mut out = items
        .into_iter()
        .take(HEAD_LIST_MAX)
        .collect::<Vec<_>>()
        .join(", ");
    if n > HEAD_LIST_MAX {
        let _ = write!(
            out,
            ", … {} more: {n} in all, and each row's {rows_say}",
            n - HEAD_LIST_MAX
        );
    }
    out
}

fn links(doc: &RegionDoc, ms: &[u64], rows_say: &str) -> String {
    capped(ms.iter().map(|&m| node_link(doc, m)).collect(), rows_say)
}

fn table(doc: &RegionDoc, run: &RunDoc, ov: &Overlay) -> String {
    let mut out = format!("# Region · {} — every node\n\n", doc.source);
    let Some(cube) = doc.cube.as_ref() else {
        let _ = writeln!(
            out,
            "No cube: {}.\n",
            doc.refused.as_deref().unwrap_or("the oracle did not run")
        );
        if run.present() {
            let _ = writeln!(out, "{} — with no cube to hold it to:\n", run.line());
            for s in &run.states {
                let _ = writeln!(
                    out,
                    "- state {}: `{}`{}",
                    s.number,
                    md(&if s.closure.is_empty() {
                        "∅".to_string()
                    } else {
                        format!("{{{}}}", s.closure.join(", "))
                    }),
                    if s.listed { "" } else { " (dump only)" }
                );
            }
            out.push('\n');
        }
        return out;
    };
    let n = doc.n;
    let _ = writeln!(
        out,
        "n = {n} · |R| = {} of {} · {} · {}\n",
        cube.size_r,
        1u64 << n,
        if doc.monotone {
            "monotone".to_string()
        } else {
            format!("**not monotone**{}", flagged_words(doc))
        },
        md(&run.line())
    );
    let names = |ms: &[u64], rows_say: &str| -> String {
        capped(
            ms.iter()
                .map(|&m| format!("`{}`", md(&doc.name(m))))
                .collect(),
            rows_say,
        )
    };
    if ov.answered {
        let _ = writeln!(
            out,
            "**Compared with {}**, by state: ruling (a), the answer is R, and a run \
             answers the part of it its question asked ({}).\n",
            ov.part,
            run.asked()
        );
        let _ = writeln!(
            out,
            "- **missing** {}",
            links(doc, &ov.missing, "status says **missing**")
        );
        let spurious: Vec<String> = ov
            .spurious
            .iter()
            .map(|(m, why)| format!("{} ({why})", node_link(doc, *m)))
            .collect();
        let _ = writeln!(
            out,
            "- **spurious** {}",
            capped(spurious, "status says **spurious**")
        );
        if !run.exhausted {
            let _ = writeln!(
                out,
                "- **not reached** {} — the run stopped ({}): a bound, not a defect",
                links(doc, &ov.unreached, "status says *not reached*"),
                run.stopped.as_deref().unwrap_or("?")
            );
        }
    } else if !run.present() {
        let _ = writeln!(
            out,
            "No run overlaid: `ein render region DIR --visited … --answer …` draws one.\n"
        );
    }
    if ov.run {
        let _ = writeln!(
            out,
            "- **≠ the cube** {}",
            links(doc, &ov.disagree, "run cell says **≠**")
        );
    }
    if run.visited {
        let unsound: Vec<String> = ov
            .unsound
            .iter()
            .map(|&(d, a)| {
                format!(
                    "{} visited dead, {} above it alive",
                    node_link(doc, d),
                    node_link(doc, a)
                )
            })
            .collect();
        let n_unsound = unsound.len();
        let _ = writeln!(
            out,
            "- **visited** {} of {} nodes; {} not visited above a dead node the run \
             visited (licence L1), {} of them alive in the cube{}",
            ov.marks.iter().filter(|m| m.visited.is_some()).count(),
            1u64 << n,
            ov.above_dead,
            n_unsound,
            if unsound.is_empty() {
                String::new()
            } else {
                format!(
                    ": **unsound** {}",
                    capped(unsound, "run cell says **✗ unsound**")
                )
            }
        );
    }
    if !ov.outside.is_empty() {
        let _ = writeln!(
            out,
            "- **reported outside the cube** {}",
            md(&ov.outside.join("; "))
        );
    }
    for (i, line) in ["minimal", "maximal"].iter().enumerate() {
        let _ = writeln!(
            out,
            "- **{line} line** {}",
            names(
                if i == 0 { &cube.minimal } else { &cube.maximal },
                if i == 0 {
                    "status says `min`"
                } else {
                    "status says `max`"
                }
            )
        );
    }
    let _ = writeln!(
        out,
        "- **holes** {}",
        names(&cube.holes, "status says `hole`")
    );
    let moved = cube
        .nodes
        .iter()
        .enumerate()
        .filter(|(m, x)| x.closure != *m as u64)
        .count();
    let _ = writeln!(
        out,
        "- **saturation moves** {moved} of {} nodes to a larger one\n",
        1u64 << n
    );
    out.push_str("| bit | candidate |\n|---:|---|\n");
    for (i, c) in doc.candidates.iter().enumerate() {
        let _ = writeln!(out, "| {i} | `{}` |", md(c));
    }
    out.push('\n');
    for k in 0..=n {
        let size = binom(n, k);
        let _ = writeln!(
            out,
            "## L{k} — {size} node{}\n",
            if size == 1 { "" } else { "s" }
        );
        if ov.run {
            out.push_str("| node | status | why | saturation | run |\n|---|---|---|---|---|\n");
        } else {
            out.push_str("| node | status | why | saturation |\n|---|---|---|---|\n");
        }
        for m in (0..1u64 << n).filter(|m| m.count_ones() as usize == k) {
            let node = &cube.nodes[m as usize];
            let mark = &ov.marks[m as usize];
            let mut status = vec![status_word(node.status).to_string()];
            if node.minimal {
                status.push("min".into());
            }
            if node.maximal {
                status.push("max".into());
            }
            if node.hole {
                status.push("hole".into());
            }
            if ov.missing.contains(&m) {
                status.push("**missing**".into());
            }
            if ov.unreached.contains(&m) {
                status.push("not reached".into());
            }
            if let Some(w) = ov.is_spurious(m) {
                status.push(format!("**spurious** ({w})"));
            }
            let mut why = Vec::new();
            if !node.core.is_empty() {
                why.push(format!(
                    "core {} by {}",
                    node.core.join(" "),
                    node.refuted_by.join(", ")
                ));
                if let Some(at) = node.core_status.and_then(|c| c.cut_at()) {
                    why.push(format!("core search cut at its budget of {at}"));
                }
            }
            if !node.absent_premises.is_empty() {
                why.push(format!("rests on {}", node.absent_premises.join(", ")));
            }
            if !node.owed.is_empty() {
                why.push(format!("owes {}", node.owed.join("; ")));
            }
            let mut sat = Vec::new();
            if node.closure != m {
                sat.push(format!("⇒ `{}`", doc.name(node.closure)));
            }
            if node.excluded != 0 {
                sat.push(format!("¬ {}", doc.facts(node.excluded).join(", ")));
            }
            if node.beyond > 0 {
                sat.push(format!(
                    "+{} beyond ({} over H)",
                    node.beyond, node.beyond_hypothesised
                ));
            }
            if node.class != m {
                sat.push(format!("≡ `{}`", doc.name(node.class)));
            }
            let cells = [
                format!("<a id=\"m{m}\"></a>`{}`", md(&doc.name(m))),
                md(&status.join(" · ")),
                md(&why.join("; ")),
                md(&sat.join("; ")),
            ];
            if !ov.run {
                let _ = writeln!(out, "| {} |", cells.join(" | "));
                continue;
            }
            let mut w = vec![mark.tag()];
            if let Some(k) = &mark.visited
                && mark.reported()
            {
                w.push(format!("visited, read {k}"));
            }
            if let Some(Unvisited::AboveDead(d)) = mark.unvisited {
                w.push(format!("above the dead `{}`", doc.name(d)));
            }
            if let Some(d) = &mark.disagrees {
                w.push(format!("**≠ {d}**"));
            }
            if let Some(a) = ov.is_unsound(m) {
                w.push(format!(
                    "**✗ unsound: `{}` above it is alive and was not visited**",
                    doc.name(a)
                ));
            }
            let _ = writeln!(
                out,
                "| {} | {} |",
                cells.join(" | "),
                md(&w
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" — "))
            );
        }
        out.push('\n');
    }
    out
}
