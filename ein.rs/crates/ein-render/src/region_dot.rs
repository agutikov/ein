//! `ein render region DIR` — the cube drawn, with this run's walk on it.
//! M1e2 [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md);
//! the legend is
//! [`04_dot_rendering.md` § Region rendering](../../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-the-walk-s1e21a2).
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
//! owing, dead), **shape** is what the search did (octagon recorded, box
//! stood on, ellipse never), **border** is the lines and the defects (double:
//! minimal line, heavy: maximal line, purple: hole, red: missing, magenta:
//! spurious or an unsound clause), and **edges** are classed by the statuses
//! at their two ends.
//!
//! Four views ([`View`]), because a cube has `2ⁿ` nodes. The cutoffs are
//! measured, and the stage file says on what.

use std::fmt::Write as _;

use rustc_hash::FxHashSet;

use crate::dot_util::esc;
use crate::region_doc::{Cube, DEAD, OWING, Overlay, RegionDoc, Skip, WalkDoc, status_word};

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
    /// `R`, its holes, every node one edge from `R`, the root, the recorded
    /// nodes and what an unsound exclusion pruned. Hidden nodes are counted
    /// per layer.
    Border,
    /// The root, both lines, the holes, the recorded nodes and what an unsound
    /// exclusion pruned, joined by inclusion: an edge spanning more than one
    /// fact says how many.
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
pub fn resolve(doc: &RegionDoc, walk: &WalkDoc, view: View) -> View {
    if view != View::Auto {
        return view;
    }
    if doc.cube.is_none() {
        return View::Lines;
    }
    if doc.n <= FULL_MAX_N {
        return View::Full;
    }
    let ov = Overlay::of(doc, walk);
    if drawn(doc, &ov, View::Border).len() <= BORDER_MAX_NODES {
        View::Border
    } else {
        View::Lines
    }
}

/// Render `view` — DOT, or markdown for [`View::Table`].
pub fn render(doc: &RegionDoc, walk: &WalkDoc, view: View) -> String {
    let view = resolve(doc, walk, view);
    let ov = Overlay::of(doc, walk);
    if view == View::Table {
        return table(doc, walk, &ov);
    }
    match &doc.cube {
        None => refused_dot(doc, walk),
        Some(cube) => dot(doc, cube, walk, &ov, view),
    }
}

// ── which nodes ─────────────────────────────────────────────────────

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
            keep.insert(ov.search_root);
            keep.extend(cube.holes.iter().copied());
            keep.extend(cube.minimal.iter().copied());
            keep.extend(cube.maximal.iter().copied());
            for (m, mark) in ov.marks.iter().enumerate() {
                if mark.recorded.is_some() {
                    keep.insert(m as u64);
                }
            }
            for e in ov.unsound() {
                keep.insert(e.mask);
                keep.extend(e.alive_superset);
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
            // Where saturation takes a node the search stood on, or a node of
            // a line: the move the drawing is about. Not every border node's,
            // which would draw half the cube's dead nodes again.
            let closures: Vec<u64> = keep
                .iter()
                .filter(|&&m| {
                    let x = &cube.nodes[m as usize];
                    ov.marks[m as usize].stood() || x.minimal || x.maximal
                })
                .map(|&m| cube.nodes[m as usize].closure)
                .collect();
            keep.extend(closures);
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
    let missing: Vec<&str> = [("min", &ov.missing[0]), ("max", &ov.missing[1])]
        .iter()
        .filter(|(_, v)| v.contains(&m))
        .map(|(l, _)| *l)
        .collect();
    let spurious: Vec<&str> = [("min", &ov.spurious[0]), ("max", &ov.spurious[1])]
        .iter()
        .filter(|(_, v)| v.contains(&m))
        .map(|(l, _)| *l)
        .collect();
    if !missing.is_empty() {
        label.push(format!("MISSING ({})", missing.join(", ")));
    }
    if !spurious.is_empty() {
        label.push(format!("SPURIOUS (not {})", spurious.join(", ")));
    }
    let unsound = ov.unsound().any(|e| e.mask == m);
    if unsound {
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
    tip.push(format!("walk: {}", if tag.is_empty() { "—" } else { &tag }));
    if let Some((_, why)) = &mark.skipped {
        tip.push(format!("  {why}"));
    }
    if let (Some(f), false) = (mark.reached_from, mark.stood()) {
        tip.push(format!("  saturation of {} lands here", doc.name(f)));
    }
    if let Some(d) = &mark.disagrees {
        tip.push(format!("≠ {d}"));
    }
    for e in ov.unsound().filter(|e| e.mask == m) {
        tip.push(format!(
            "✗ {} excluded every superset, and {} is alive",
            e.kinds.join("+"),
            doc.name(e.alive_superset.unwrap_or_default())
        ));
    }

    let shape = if mark.recorded.is_some() {
        "octagon"
    } else if mark.visited() {
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
    let (colour, width) = if !missing.is_empty() {
        (MISSING_COLOUR, 3.0)
    } else if !spurious.is_empty() || unsound || mark.disagrees.is_some() {
        (SPURIOUS_COLOUR, 3.0)
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

fn dot(doc: &RegionDoc, cube: &Cube, walk: &WalkDoc, ov: &Overlay, view: View) -> String {
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
                format!("NOT monotone ({} sites)", doc.sites.len())
            }
        ),
        format!(
            "walk: {} k = {} exhausted = {} · view {} ({} of {} nodes)",
            walk.verdict,
            walk.k,
            walk.exhausted,
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
    // What an unsound exclusion pruned.
    for e in ov.unsound() {
        if let Some(w) = e.alive_superset
            && w != e.mask
        {
            let _ = writeln!(
                out,
                "  m{} -> m{w} [color=\"{SPURIOUS_COLOUR}\", style=dashed, penwidth=2, \
                 constraint=false, label=\"✗ alive\", fontsize=8, fontcolor=\"{SPURIOUS_COLOUR}\", \
                 tooltip=\"{} excluded {} and every superset; {} is alive\"];",
                e.mask,
                e.kinds.join("+"),
                esc(&doc.name(e.mask)),
                esc(&doc.name(w))
            );
        }
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
            "the run recorded this node's state (model or open)",
        ),
        (
            "rounded box",
            "the search stood here (root, an entering), or reached it by saturation",
        ),
        (
            "ellipse",
            "the search never stood here; why is under the facts",
        ),
        ("double border", "R's minimal line"),
        ("heavy green border", "R's maximal line"),
        ("purple border", "a hole: outside R, between two members"),
        (
            "red border",
            "MISSING: on a line, and no recorded model is its state",
        ),
        (
            "magenta border",
            "SPURIOUS, an unsound exclusion, or the search against the cube",
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
    if ov.unsound().next().is_some() {
        row(
            &mut key,
            format!("<FONT COLOR=\"{SPURIOUS_COLOUR}\"><B>╌▶</B></FONT> ✗ alive"),
            "an exclusion the cube refutes, to the smallest alive superset it pruned",
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
    let mut notes: Vec<String> = vec![
        "lines compared: both — which line is the answer is S1e2.1a.3's ruling (a)".to_string(),
    ];
    for (i, line) in ["minimal", "maximal"].iter().enumerate() {
        let on = if i == 0 { &cube.minimal } else { &cube.maximal };
        notes.push(format!(
            "{line} line ({}): {} — missing {}; spurious {}",
            on.len(),
            listed(doc, on),
            listed(doc, &ov.missing[i]),
            listed(doc, &ov.spurious[i])
        ));
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
    let unsound: Vec<_> = ov.unsound().collect();
    for e in unsound.iter().take(LEGEND_LIST_MAX) {
        notes.push(format!(
            "✗ unsound {}: {} — {} is alive",
            e.kinds.join("+"),
            short(doc, e.mask),
            short(doc, e.alive_superset.unwrap_or_default())
        ));
    }
    if unsound.len() > LEGEND_LIST_MAX {
        notes.push(format!(
            "… {} more unsound exclusions (--view table)",
            unsound.len() - LEGEND_LIST_MAX
        ));
    }
    if !ov.outside_recorded.is_empty() {
        notes.push(format!(
            "recorded outside the cube: {}",
            ov.outside_recorded.join("; ")
        ));
    }
    if !doc.monotone {
        notes.push(format!(
            "not monotone — sites: {}; the lines assume a monotone lattice (P1e2.1b)",
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
         <TR><TD ALIGN=\"LEFT\"><B>legend</B> — fill: the cube · shape: the search · border: the lines and the defects</TD></TR>\
         <TR><TD ALIGN=\"LEFT\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"3\">{}</TABLE></TD></TR>\
         <TR><TD ALIGN=\"LEFT\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"3\">{notes}</TABLE></TD></TR>\
         </TABLE>>];",
        key.join("")
    );
    let _ = writeln!(out, "  {{ rank=sink; legend; }}");
    let _ = writeln!(out, "  L{last_band} -> legend [style=invis];");
}

fn refused_dot(doc: &RegionDoc, walk: &WalkDoc) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "digraph region {{");
    let _ = writeln!(out, "  // ein render region — view none (no cube)");
    let lines = vec![
        format!("region · {}", doc.source),
        format!("n = {} — no cube", doc.n),
        doc.refused.clone().unwrap_or_default(),
        format!(
            "walk: {} k = {} exhausted = {} · {} enterings",
            walk.verdict,
            walk.k,
            walk.exhausted,
            walk.enterings.len()
        ),
    ];
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

fn table(doc: &RegionDoc, walk: &WalkDoc, ov: &Overlay) -> String {
    let mut out = format!("# Region · {} — every node\n\n", doc.source);
    let Some(cube) = doc.cube.as_ref() else {
        let _ = writeln!(
            out,
            "No cube: {}.\n",
            doc.refused.as_deref().unwrap_or("the oracle did not run")
        );
        return out;
    };
    let n = doc.n;
    let _ = writeln!(
        out,
        "n = {n} · |R| = {} of {} · {} · walk: **{}**, k = {}, exhausted = {}\n",
        cube.size_r,
        1u64 << n,
        if doc.monotone {
            "monotone".to_string()
        } else {
            format!("**not monotone** ({} sites)", doc.sites.len())
        },
        walk.verdict,
        walk.k,
        walk.exhausted
    );
    let names = |ms: &[u64]| -> String {
        if ms.is_empty() {
            "none".to_string()
        } else {
            ms.iter()
                .map(|&m| format!("`{}`", doc.name(m)))
                .collect::<Vec<_>>()
                .join(", ")
        }
    };
    let _ = writeln!(
        out,
        "Lines compared: **both** — which line is the answer is S1e2.1a.3's ruling (a).\n"
    );
    for (i, line) in ["minimal", "maximal"].iter().enumerate() {
        let _ = writeln!(
            out,
            "- **{line} line** {} — missing {}; spurious {}",
            names(if i == 0 { &cube.minimal } else { &cube.maximal }),
            names(&ov.missing[i]),
            names(&ov.spurious[i])
        );
    }
    let _ = writeln!(out, "- **holes** {}", names(&cube.holes));
    let unsound: Vec<String> = ov
        .unsound()
        .map(|e| {
            format!(
                "{} `{}` (alive superset `{}`)",
                e.kinds.join("+"),
                doc.name(e.mask),
                doc.name(e.alive_superset.unwrap_or_default())
            )
        })
        .collect();
    let _ = writeln!(
        out,
        "- **exclusions** {} checked, {} unsound{}",
        ov.exclusions.len(),
        unsound.len(),
        if unsound.is_empty() {
            String::new()
        } else {
            format!(": {}", unsound.join("; "))
        }
    );
    let moved = cube
        .nodes
        .iter()
        .enumerate()
        .filter(|(m, x)| x.closure != *m as u64)
        .count();
    let _ = writeln!(
        out,
        "- **saturation moves** {moved} of {} nodes to a larger one",
        1u64 << n
    );
    let mut skips: Vec<(Skip, usize)> = Vec::new();
    for mark in &ov.marks {
        if let Some((s, _)) = &mark.skipped {
            match skips.iter_mut().find(|x| x.0 == *s) {
                Some(x) => x.1 += 1,
                None => skips.push((*s, 1)),
            }
        }
    }
    let visited = ov.marks.iter().filter(|m| m.visited()).count();
    let _ = writeln!(
        out,
        "- **the walk** stood on {visited} nodes{}\n",
        skips
            .iter()
            .map(|(s, c)| format!("; {c} {}", s.as_str()))
            .collect::<String>()
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
        out.push_str("| node | status | why | saturation | walk |\n|---|---|---|---|---|\n");
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
            if ov.missing[0].contains(&m) {
                status.push("**missing (min)**".into());
            }
            if ov.missing[1].contains(&m) {
                status.push("**missing (max)**".into());
            }
            if ov.spurious[0].contains(&m) {
                status.push("**spurious (not min)**".into());
            }
            if ov.spurious[1].contains(&m) {
                status.push("**spurious (not max)**".into());
            }
            let mut why = Vec::new();
            if !node.core.is_empty() {
                why.push(format!(
                    "core {} by {}",
                    node.core.join(" "),
                    node.refuted_by.join(", ")
                ));
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
            let mut w = vec![mark.tag()];
            if let Some((_, d)) = &mark.skipped {
                w.push(d.clone());
            }
            if let (Some(f), false) = (mark.reached_from, mark.stood()) {
                w.push(format!("from `{}`", doc.name(f)));
            }
            if let Some(d) = &mark.disagrees {
                w.push(format!("**≠ {d}**"));
            }
            for e in ov.unsound().filter(|e| e.mask == m) {
                w.push(format!(
                    "**✗ unsound {}: `{}` alive**",
                    e.kinds.join("+"),
                    doc.name(e.alive_superset.unwrap_or_default())
                ));
            }
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} | {} |",
                md(&doc.name(m)),
                md(&status.join(" · ")),
                md(&why.join("; ")),
                md(&sat.join("; ")),
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
