//! The derivation-slice and KB-snapshot renderers — `ein.py`'s
//! `render/slice.py`.
//!
//! The trace does not want the whole 25-cell KB at each step. It wants, per
//! hypothesis, **just what that hypothesis touched** — a provenance cone: the
//! hypothesis facts, the KB facts the firings consumed, the rules that fired,
//! and the facts derived. That cone is [`render_slice`], embedded in each
//! trace section; fed a dead commitment's firing chain plus `contradiction`,
//! it becomes the refuted-branch slice terminating in `⊥`.
//!
//! Colour key: hypothesis / seed facts red, derived facts bold, negative
//! (eliminated-alternative) facts grey, rule nodes coloured by rule, the `⊥`
//! refutation node red.

use ein_core::{FactId, Kb, Tag, Terms, Value};
use ein_infer::firing::Firing;

use crate::dot_util::{digraph_open, fact_label, hashed_id, multiline, quote};
use crate::kb_dot::{KbDotOpts, to_dot};
use crate::palette::hash_color;
use ein_core::render_why;

const SEED_COLOUR: &str = "#d62728"; // red — hypothesis / seed facts, ⊥
const NEG_COLOUR: &str = "#7f7f7f"; // grey — eliminated-alternative facts

/// The content key behind a slice node id — stable across runs, and
/// deliberately **recursive**: it descends into nested fact arguments, which
/// the flat [`crate::dot_util::fact_key`] does not. That recursion is
/// load-bearing for these ids, which is why S1.7c.25 shared only the
/// hash-and-prefix tail rather than merging the two key builders.
fn key(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.fact(f);
    let parts: Vec<String> = args.iter().map(|a| arg_str(terms, *a)).collect();
    format!("{}|{}", terms.sym(rel), parts.join(","))
}

fn arg_str(terms: &Terms, a: Value) -> String {
    match a.tag() {
        Tag::Fact => key(terms, a.as_fact().expect("tagged Fact")),
        _ => terms.display(a),
    }
}

fn node_id(key: &str) -> String {
    hashed_id("f_", key, true)
}

/// Render one hypothesis's provenance cone as an inline `dot` block.
///
/// `commitment` is the hypothesis fact set (drawn red); `firings` are the
/// surviving-path firings, each a rule node between its premises and its
/// derived facts, labelled with the rendered `:why` when `kb` carries the
/// rule. Only facts in the cone appear — never the whole KB.
///
/// `contradiction` is a dead commitment's `(unsat_core, learned_clause)`: the
/// core facts point at a `⊥` node tagged with the lifted no-good. `since`
/// thickens facts absent from that prior KB.
pub fn render_slice(
    terms: &Terms,
    commitment: &[FactId],
    firings: &[Firing],
    kb: Option<&Kb>,
    name: &str,
    contradiction: Option<(&[FactId], &[FactId])>,
    since: Option<&Kb>,
) -> String {
    let since_keys: Option<Vec<String>> = since.map(|s| s.facts().map(|f| key(terms, f)).collect());

    let seed_keys: Vec<String> = commitment.iter().map(|f| key(terms, *f)).collect();
    let derived_keys: Vec<String> = firings
        .iter()
        .flat_map(|f| f.derived.iter())
        .map(|d| key(terms, *d))
        .collect();

    // key → the full declaration line, in first-touch order.
    let mut node_decls: Vec<(String, String)> = Vec::new();
    let mut edges: Vec<String> = Vec::new();
    let mut firing_nodes: Vec<String> = Vec::new();

    let touch = |decls: &mut Vec<(String, String)>, f: FactId| -> String {
        let k = key(terms, f);
        let nid = node_id(&k);
        if !decls.iter().any(|(dk, _)| *dk == k) {
            let (rel, _) = terms.fact(f);
            let negative = terms.sym(rel) == "not";
            let seed = seed_keys.contains(&k);
            let derived = derived_keys.contains(&k);
            let mut attrs = vec![
                format!("label={}", quote(&fact_label(terms, f))),
                "shape=box".to_string(),
            ];
            if seed {
                attrs.push(format!("color=\"{SEED_COLOUR}\""));
                attrs.push(format!("fontcolor=\"{SEED_COLOUR}\""));
                attrs.push("style=\"rounded,filled\"".to_string());
                attrs.push("fillcolor=\"#fdeaea\"".to_string());
            } else if negative {
                attrs.push(format!("color=\"{NEG_COLOUR}\""));
                attrs.push(format!("fontcolor=\"{NEG_COLOUR}\""));
                attrs.push("style=rounded".to_string());
            } else {
                attrs.push("style=rounded".to_string());
            }
            if derived {
                attrs.push("penwidth=2".to_string()); // bold — newly derived
            }
            if since_keys.as_ref().is_some_and(|sk| !sk.contains(&k)) {
                attrs.push("penwidth=3".to_string()); // transition highlight
            }
            decls.push((k, format!("  {nid} [{}];", attrs.join(", "))));
        }
        nid
    };

    // Seeds first, so they declare red even when they are also a premise.
    for f in commitment {
        touch(&mut node_decls, *f);
    }

    // Each firing → a rule node between its premises and its derived facts.
    for (idx, firing) in firings.iter().enumerate() {
        let rule_name = terms.sym(firing.rule).to_string();
        let fnode = quote(&format!("fire{idx}_{rule_name}"));
        let colour = hash_color(&rule_name);
        let why = kb
            .and_then(|kb| kb.program().rules.get(firing.rule))
            .and_then(|r| r.why)
            .map(|w| terms.sym(w).to_string())
            .filter(|w| !w.is_empty())
            .map(|template| {
                let bindings: Vec<(String, String)> = firing
                    .bindings
                    .iter()
                    .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
                    .collect();
                render_why(&template, &bindings)
            })
            .unwrap_or_default();
        let style = if firing.redundant {
            "rounded,dashed"
        } else {
            "rounded,bold"
        };
        firing_nodes.push(format!(
            "  {fnode} [shape=box, style=\"{style}\", color=\"{colour}\", \
             fontcolor=\"{colour}\", label={}];",
            multiline(&[&rule_name, &why])
        ));
        for p in firing.premises.iter() {
            let pid = touch(&mut node_decls, *p);
            edges.push(format!("  {pid} -> {fnode} [color=\"{colour}\"];"));
        }
        // One application fans out to each derived fact.
        for d in firing.derived.iter() {
            let did = touch(&mut node_decls, *d);
            edges.push(format!(
                "  {fnode} -> {did} [color=\"{colour}\", style=bold];"
            ));
        }
    }

    // Refuted branch → ⊥ tied to the unsat core, tagged with the no-good.
    if let Some((unsat_core, learned_clause)) = contradiction {
        let bottom = quote("⊥");
        firing_nodes.push(format!(
            "  {bottom} [shape=doublecircle, color=\"{SEED_COLOUR}\", \
             fontcolor=\"{SEED_COLOUR}\", label=\"⊥\"];"
        ));
        // `sorted`, because `unsat_core` is a `set[Fact]`: iterating it raw
        // made the `-> ⊥` edge order depend on `PYTHONHASHSEED`, and this
        // block lands verbatim in `--trace` output (hazard H4). `key=repr`
        // matches `inference.explain`'s convention and is total over mixed
        // argument types, where a bare `sorted` would raise (Q-M1a.4).
        let mut core: Vec<(String, FactId)> = unsat_core
            .iter()
            .map(|f| (ein_core::pyrepr::repr(&terms.py_fact(*f)), *f))
            .collect();
        core.sort();
        for (_, f) in core {
            let cid = touch(&mut node_decls, f);
            edges.push(format!("  {cid} -> {bottom} [color=\"{SEED_COLOUR}\"];"));
        }
        if !learned_clause.is_empty() {
            let ng = quote("learned-nogood");
            let mut labels: Vec<String> = learned_clause
                .iter()
                .map(|f| fact_label(terms, *f))
                .collect();
            labels.sort();
            let clause = labels.join(" ∧ ");
            firing_nodes.push(format!(
                "  {ng} [shape=note, label={}];",
                multiline(&["learned no-good", &clause])
            ));
            edges.push(format!(
                "  {bottom} -> {ng} [style=dashed, label=\"lifts to\"];"
            ));
        }
    }

    let mut lines = digraph_open(name, Some("LR"), Some("fontname=\"Inter\""));
    lines.extend(node_decls.into_iter().map(|(_, line)| line));
    lines.extend(firing_nodes);
    lines.extend(edges);
    lines.push("}".to_string());
    lines.join("\n")
}

/// The complete KB graph at a moment — flag-gated behind
/// `--full-kb-snapshots`. `since` thickens the facts absent from that prior
/// KB ("this step added E").
pub fn render_state(kb: &Kb, terms: &Terms, since: Option<&Kb>, name: &str) -> String {
    to_dot(
        kb,
        terms,
        &KbDotOpts {
            since,
            name,
            ..KbDotOpts::default()
        },
    )
}

/// The solved-state graph — the trace's closing answer view.
pub fn render_solution(kb: &Kb, terms: &Terms, name: &str) -> String {
    to_dot(
        kb,
        terms,
        &KbDotOpts {
            name,
            ..KbDotOpts::default()
        },
    )
}
