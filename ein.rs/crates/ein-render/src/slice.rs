//! The derivation-slice and KB-snapshot renderers.
//!
//! The trace does not want the whole 25-cell KB at each step. It wants, per
//! hypothesis, **just what that hypothesis touched** — a provenance cone: the
//! hypothesis facts, the KB facts the firings consumed, the rules that fired,
//! and the facts derived. That cone is [`render_slice`], embedded in each
//! trace section; fed a dead commitment's firing chain plus `contradiction`,
//! it becomes the refuted-branch slice terminating in `⊥`.
//!
//! Colour key: hypothesis / seed facts red, negative (eliminated-alternative)
//! facts grey, each step and each fact it derives in its rule's hue, the `⊥`
//! refutation node red. Each fact is drawn as its node in the canonical form,
//! or folded on request, as every view draws one (Q-M1f.419), and laid out in
//! layers as a knowledge base is ([`crate::fact_graph`]): a relation above
//! its facts, a fact above its arguments.
//!
//! **The derivation stands out** (Q-M1f.428), at three levels: the fact a
//! step adds, its octagon and its arrows, is the brightest, in the
//! [`Hue::derived`] shade of its rule's hue at [`DERIVED_WIDTH`]; the step's
//! border and the derivation's arrows, from a premise to a step, from a step
//! to what it derives, are its [`Hue::explanation`] shade, less bright, at
//! [`DERIVATION_WIDTH`]; everything else is Graphviz's default black at the
//! default width. No shade of [`DERIVATION_PALETTE`] is near black.
//!
//! **No arrow goes to or from an arrow** (Q-M1f.429): a fact a derivation's
//! arrow touches, a premise, a fact derived, a fact of the core or of the
//! no-good, is its octagon in either form, and only a fact no such arrow
//! touches is folded.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use ein_core::{FactId, Kb, TermStep, Terms, Value};
use ein_infer::firing::Firing;

use crate::dot_util::{
    FRAMED_RANKING, HYPER_SHAPE, POLYGON_RANKSEP, ctx_frames, ctx_label, digraph_open, hashed_id,
    multiline, quote,
};
use crate::fact_graph::{FACT_NODE, first_arrow, folds};
use crate::kb_dot::{KbDotOpts, to_dot};

const SEED_COLOUR: &str = "#d62728"; // red — hypothesis / seed facts, ⊥
const NEG_COLOUR: &str = "#7f7f7f"; // grey — eliminated-alternative facts

/// The width of a derivation's arrows and of its steps' borders: twice
/// Graphviz's default.
pub const DERIVATION_WIDTH: &str = "penwidth=2";

/// The width of a fact a derivation adds, its octagon and its arrows: three
/// times Graphviz's default, a step wider than the derivation's arrows.
pub const DERIVED_WIDTH: u32 = 3;

/// Graphviz's default colour, written where a fact's look must name one: the
/// rest of a derivation's drawing, which no step derives.
pub const DEFAULT_INK: &str = "black";

/// One rule's hue, in two shades of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hue {
    /// The derivation's explanation: a step's border and label, and its
    /// arrows, from a premise to the step and from the step to what it
    /// derives.
    pub explanation: &'static str,
    /// The fact the derivation adds, its octagon and its arrows: the same
    /// hue, brighter.
    pub derived: &'static str,
}

/// The hues of a derivation's steps, one per rule, fixed by its name. Each
/// shade is clear of black, at a contrast of more than 3.5 against black,
/// and readable on white: the explanation's, which a step's label is written
/// in, at 5.5 or more, the derived fact's, a line, at 3 or more. None is red,
/// the commitment's and the core's colour, or grey, a negation's.
pub const DERIVATION_PALETTE: [Hue; 6] = [
    Hue {
        explanation: "#0e61cc",
        derived: "#418cfa",
    }, // blue
    Hue {
        explanation: "#11706e",
        derived: "#1c9e9a",
    }, // teal
    Hue {
        explanation: "#117622",
        derived: "#1ca433",
    }, // green
    Hue {
        explanation: "#98530f",
        derived: "#d37619",
    }, // orange
    Hue {
        explanation: "#951be7",
        derived: "#b46afa",
    }, // purple
    Hue {
        explanation: "#af2b94",
        derived: "#f240cd",
    }, // magenta
];

/// A step's hue: [`DERIVATION_PALETTE`] indexed by the rule's name, by the
/// same stable digest as a relation's colour ([`crate::palette::hash_color`]).
pub fn derivation_hue(rule: &str) -> Hue {
    DERIVATION_PALETTE[crate::palette::stable_index(rule, DERIVATION_PALETTE.len())]
}

/// The content key behind a slice node id — stable across runs, and
/// deliberately **deep**: it descends into nested fact arguments, at any
/// depth, which the flat [`crate::dot_util::fact_key`] does not. That descent
/// is load-bearing for these ids, which is why only the hash-and-prefix tail,
/// [`crate::dot_util::hashed_id`], is shared and the two key builders are not
/// merged.
fn key(terms: &Terms, f: FactId) -> String {
    // `rel|a,b`, a nested fact's argument its own key, at any depth
    // ([`Terms::walk_term`]).
    let mut out = String::new();
    terms.walk_term(Value::fact(f), |step| match step {
        TermStep::Name(s) => out.push_str(&ein_core::text::symbol(terms.sym(s))),
        TermStep::Open(rel, _) => {
            out.push_str(terms.sym(rel));
            out.push('|');
        }
        TermStep::Arg(i) => {
            if i > 0 {
                out.push(',');
            }
        }
        TermStep::Close(..) => {}
    });
    out
}

fn node_id(key: &str) -> String {
    hashed_id("f_", key, true)
}

/// The marks of one fact of a cone: red where the commitment holds it, the
/// derived shade of its step's hue and [`DERIVED_WIDTH`] where a firing
/// added it, grey where it is a stored negation, and heavier where `since`
/// lacks it.
struct Marks {
    colour: Option<&'static str>,
    penwidth: Option<u32>,
}

impl Marks {
    fn attrs(&self) -> String {
        let mut out = String::new();
        if let Some(c) = self.colour {
            out.push_str(&format!(", color=\"{c}\", fontcolor=\"{c}\""));
        }
        if let Some(p) = self.penwidth {
            out.push_str(&format!(", penwidth={p}"));
        }
        out
    }

    fn edge_attrs(&self) -> String {
        let mut out = Vec::new();
        if let Some(c) = self.colour {
            out.push(format!("color=\"{c}\""));
        }
        if let Some(p) = self.penwidth {
            out.push(format!("penwidth={p}"));
        }
        out.join(", ")
    }
}

/// The node of what stands under an arrow: a name's box, or a fact's node.
fn under_id(terms: &Terms, a: Value) -> String {
    match a.as_fact() {
        Some(g) => node_id(&key(terms, g)),
        None => quote(&terms.display(a)),
    }
}

/// Render one hypothesis's provenance cone as an inline `dot` block.
///
/// `commitment` is the hypothesis fact set (drawn red); `firings` are the
/// surviving-path firings, each a rule node between its premises and its
/// derived facts, labelled with the rendered `:why` when `kb` carries the
/// rule. Only facts in the cone appear — never the whole KB.
///
/// **Each fact is drawn as its node** (Q-M1f.419): an octagon with no name in
/// it, its first arrow at its relation's atom and the next at its
/// arguments, never a node labelled with its text; a fact one of them points
/// at is its node too, in the frame of its context where `kb` does not hold
/// it. Under `fold`, a fact of three arrows that no fact of the cone points
/// at is one arrow from its first argument to its second, labelled with its
/// relation, and a fact of two arrows a loop on its argument, where no
/// derivation's arrow touches it: a premise, a fact derived, a fact of the
/// core or of the no-good is always its octagon, so that no arrow goes to or
/// from an arrow (Q-M1f.429).
///
/// `contradiction` is a dead commitment's `(unsat_core, learned_clause)`: the
/// core facts point at a `⊥` node, and a note, `learned no-good`, points at
/// each fact of the lifted no-good. `since` thickens facts absent from that
/// prior KB.
#[allow(clippy::too_many_arguments)]
pub fn render_slice(
    terms: &Terms,
    commitment: &[FactId],
    firings: &[Firing],
    kb: Option<&Kb>,
    name: &str,
    contradiction: Option<(&[FactId], &[FactId])>,
    since: Option<&Kb>,
    fold: bool,
) -> String {
    let since_keys: Option<HashSet<String>> =
        since.map(|s| s.facts().map(|f| key(terms, f)).collect());
    let seed_keys: HashSet<String> = commitment.iter().map(|f| key(terms, *f)).collect();
    // Each fact a firing adds, in the brighter shade of the first such
    // firing's hue. A redundant firing adds nothing: what it derives was
    // there before it.
    let mut derived_shade: HashMap<String, &'static str> = HashMap::new();
    for firing in firings.iter().filter(|f| !f.redundant) {
        let shade = derivation_hue(terms.sym(firing.rule)).derived;
        for d in firing.derived.iter() {
            derived_shade.entry(key(terms, *d)).or_insert(shade);
        }
    }

    // The facts of the cone, in first-touch order: the seeds first, so they
    // are drawn red even when they are also a premise.
    let mut cone: Vec<FactId> = Vec::new();
    let mut in_cone: HashSet<String> = HashSet::new();
    let mut touch = |f: FactId| {
        if in_cone.insert(key(terms, f)) {
            cone.push(f);
        }
    };
    commitment.iter().for_each(|f| touch(*f));
    for firing in firings {
        firing.premises.iter().for_each(|p| touch(*p));
        firing.derived.iter().for_each(|d| touch(*d));
    }
    // Sorted, because `unsat_core` is a set: its raw iteration order is not
    // a property of the data, and this block lands verbatim in `--trace`
    // output (hazard H4). Sorting by `repr` matches the explainer's
    // convention and is total over mixed argument types
    // (`docs/ein/reasoning/01-static-ir/01-knowledge-base.md` § The text of an object).
    let sorted = |facts: &[FactId]| -> Vec<FactId> {
        let mut v: Vec<(String, FactId)> = facts
            .iter()
            .map(|f| (terms.repr_fact_text(*f), *f))
            .collect();
        v.sort();
        v.dedup();
        v.into_iter().map(|(_, f)| f).collect()
    };
    let (core, clause) = match contradiction {
        Some((c, l)) => (sorted(c), sorted(l)),
        None => (Vec::new(), Vec::new()),
    };
    core.iter().for_each(|f| touch(*f));
    clause.iter().for_each(|f| touch(*f));
    // The facts a derivation's arrow touches: never folded, since an arrow
    // goes only to or from a node.
    let touched: HashSet<String> = firings
        .iter()
        .flat_map(|f| f.premises.iter().chain(f.derived.iter()))
        .chain(core.iter())
        .chain(clause.iter())
        .map(|f| key(terms, *f))
        .collect();

    // What a fact of the cone points at, at any depth, with the relations
    // that point at it: a node, never folded.
    let mut nested: Vec<(FactId, BTreeSet<String>)> = Vec::new();
    let mut at: HashMap<String, usize> = HashMap::new();
    let mut queue: VecDeque<FactId> = cone.iter().copied().collect();
    let mut walked: HashSet<String> = HashSet::new();
    while let Some(f) = queue.pop_front() {
        if !walked.insert(key(terms, f)) {
            continue;
        }
        let (rel, args) = terms.fact(f);
        for g in args.iter().filter_map(|a| a.as_fact()) {
            let k = key(terms, g);
            let i = *at.entry(k).or_insert_with(|| {
                nested.push((g, BTreeSet::new()));
                queue.push_back(g);
                nested.len() - 1
            });
            nested[i].1.insert(terms.sym(rel).to_string());
        }
    }
    let marks = |f: FactId| -> Marks {
        let k = key(terms, f);
        let (rel, _) = terms.fact(f);
        let derived = derived_shade.get(&k).copied();
        let colour = if seed_keys.contains(&k) {
            Some(SEED_COLOUR)
        } else if derived.is_some() {
            derived
        } else if terms.sym(rel) == "not" {
            Some(NEG_COLOUR)
        } else {
            None
        };
        let penwidth = if since_keys.as_ref().is_some_and(|sk| !sk.contains(&k)) {
            Some(3) // transition highlight
        } else if derived.is_some() {
            Some(DERIVED_WIDTH)
        } else {
            None
        };
        Marks { colour, penwidth }
    };
    let is_nested = |f: FactId| at.contains_key(&key(terms, f));
    let octagon = |f: FactId| -> (String, Vec<String>) {
        let id = node_id(&key(terms, f));
        let m = marks(f);
        let decl = format!("{id} [shape={HYPER_SHAPE}, {FACT_NODE}{}];", m.attrs());
        let (rel, args) = terms.fact(f);
        let tail = m.edge_attrs();
        let tail = if tail.is_empty() {
            String::new()
        } else {
            format!(", {tail}")
        };
        let mut arrows = vec![format!(
            "  {}",
            first_arrow(&id, &quote(terms.sym(rel)), &tail)
        )];
        for (i, a) in args.iter().enumerate() {
            arrows.push(format!(
                "  {id} -> {} [xlabel=\"{}\"{tail}];",
                under_id(terms, *a),
                i + 2
            ));
        }
        (decl, arrows)
    };

    let mut lines = digraph_open(name, Some("TB"), Some("fontname=\"Inter\", shape=box"));
    lines.push(format!("  {POLYGON_RANKSEP};"));
    let mut arrows: Vec<String> = Vec::new();
    for &f in &cone {
        if is_nested(f) {
            continue;
        }
        let (rel, args) = terms.fact(f);
        if fold && folds(args.len()) && !touched.contains(&key(terms, f)) {
            // One labelled arrow from the first argument to the second, or
            // a loop on the one argument.
            let m = marks(f);
            let mut attrs = format!("label={}", quote(terms.sym(rel)));
            if let Some(c) = m.colour {
                attrs.push_str(&format!(", fontcolor=\"{c}\""));
            }
            let tail = m.edge_attrs();
            if !tail.is_empty() {
                attrs.push_str(&format!(", {tail}"));
            }
            let from = under_id(terms, args[0]);
            let to = args.get(1).map_or(from.clone(), |a| under_id(terms, *a));
            arrows.push(format!("  {from} -> {to} [{attrs}];"));
            continue;
        }
        let (decl, a) = octagon(f);
        lines.push(format!("  {decl}"));
        arrows.extend(a);
    }
    // The facts a fact of the cone points at: a node each, in the frame of
    // its context where the state does not hold it.
    let mut framed: Vec<(String, String)> = Vec::new();
    for (g, by) in &nested {
        let (decl, a) = octagon(*g);
        if kb.is_some_and(|kb| !kb.contains(*g)) {
            framed.push((ctx_label(by), decl));
        } else {
            lines.push(format!("  {decl}"));
        }
        arrows.extend(a);
    }
    if !framed.is_empty() {
        lines.push(format!("  {FRAMED_RANKING};"));
        lines.extend(ctx_frames("", "  ", &framed));
    }

    let mut steps: Vec<String> = Vec::new();
    let mut edges: Vec<String> = Vec::new();
    // Each firing → a rule node between its premises and its derived facts.
    for (idx, firing) in firings.iter().enumerate() {
        let rule_name = terms.sym(firing.rule).to_string();
        let fnode = quote(&format!("fire{idx}_{rule_name}"));
        let colour = derivation_hue(&rule_name).explanation;
        let why = kb
            .and_then(|kb| {
                crate::sentence::rule_sentence(kb.program(), terms, firing.rule, &firing.bindings)
            })
            .unwrap_or_default();
        let style = if firing.redundant {
            "rounded,dashed"
        } else {
            "rounded"
        };
        steps.push(format!(
            "  {fnode} [shape=box, style=\"{style}\", color=\"{colour}\", \
             fontcolor=\"{colour}\", {DERIVATION_WIDTH}, label={}];",
            multiline(&[&rule_name, &why])
        ));
        for p in firing.premises.iter() {
            let pid = node_id(&key(terms, *p));
            edges.push(format!(
                "  {pid} -> {fnode} [color=\"{colour}\", {DERIVATION_WIDTH}];"
            ));
        }
        // One application fans out to each derived fact.
        for d in firing.derived.iter() {
            let did = node_id(&key(terms, *d));
            edges.push(format!(
                "  {fnode} -> {did} [color=\"{colour}\", {DERIVATION_WIDTH}];"
            ));
        }
        // An application that ended the state derived no fact: it points at
        // the word its rule gave the engine, which is no node of the KB.
        if firing.ends {
            let word = quote(&format!(
                ":ein {}",
                ein_core::EngineWord::Inconsistent.as_str()
            ));
            steps.push(format!(
                "  {word} [shape=plaintext, fontcolor=\"{colour}\", label={word}];"
            ));
            edges.push(format!(
                "  {fnode} -> {word} [color=\"{colour}\", {DERIVATION_WIDTH}];"
            ));
        }
    }

    // Refuted branch → ⊥ tied to the unsat core, and the no-good it lifts
    // to, a note that points at each of its facts.
    if contradiction.is_some() {
        let bottom = quote("⊥");
        steps.push(format!(
            "  {bottom} [shape=doublecircle, color=\"{SEED_COLOUR}\", \
             fontcolor=\"{SEED_COLOUR}\", {DERIVATION_WIDTH}, label=\"⊥\"];"
        ));
        for f in &core {
            let cid = node_id(&key(terms, *f));
            edges.push(format!(
                "  {cid} -> {bottom} [color=\"{SEED_COLOUR}\", {DERIVATION_WIDTH}];"
            ));
        }
        // The no-good's arrows are the refutation's, in the core's red: no
        // arrow of a derivation is black.
        if !clause.is_empty() {
            let ng = quote("learned-nogood");
            steps.push(format!("  {ng} [shape=note, label=\"learned no-good\"];"));
            edges.push(format!(
                "  {bottom} -> {ng} [style=dashed, color=\"{SEED_COLOUR}\", {DERIVATION_WIDTH}, label=\"lifts to\"];"
            ));
            for f in &clause {
                let fid = node_id(&key(terms, *f));
                edges.push(format!(
                    "  {ng} -> {fid} [style=dashed, color=\"{SEED_COLOUR}\", {DERIVATION_WIDTH}];"
                ));
            }
        }
    }

    lines.extend(steps);
    lines.extend(arrows);
    lines.extend(edges);
    lines.push("}".to_string());
    lines.join("\n")
}

/// The complete KB graph at a moment — flag-gated behind
/// `--full-kb-snapshots`. `since` thickens the facts absent from that prior
/// KB ("this step added E").
pub fn render_state(kb: &Kb, terms: &Terms, since: Option<&Kb>, name: &str, fold: bool) -> String {
    to_dot(
        kb,
        terms,
        &KbDotOpts {
            since,
            name,
            fold,
            ..KbDotOpts::default()
        },
    )
}

/// The solved-state graph — the trace's closing answer view.
pub fn render_solution(kb: &Kb, terms: &Terms, name: &str, fold: bool) -> String {
    to_dot(
        kb,
        terms,
        &KbDotOpts {
            name,
            fold,
            ..KbDotOpts::default()
        },
    )
}
