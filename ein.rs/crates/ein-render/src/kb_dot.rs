//! The unified KB → DOT renderer.
//!
//! One `digraph` over the whole knowledge base, **fusing** entity identity
//! across forms: `Norwegian` is emitted once and participates in its type
//! edge, its authored `(co-located Norwegian House-1 :source "(10)")` fact
//! edge, and any inferred edge. That is the 2021 prototype's `linked.svg` —
//! types, instances and inferences on one canvas, not stacked tiles.
//!
//! **No head is special-cased.** The type-box / instance-oval split is read
//! from the puzzle's own `is-a` facts; a puzzle that spells membership some
//! other way gets ordinary fact rendering, not a second convention.
//!
//! **Every object is drawn once** (Q-M1f.395). A fact that a
//! drawn fact points at is its own node, an octagon with its argument arrows
//! numbered, whatever its count and held or not: never a node labelled with
//! the fact's text, and never an arrow beside its node. Where the knowledge
//! base does not hold it, in a context only, it stands in a dashed frame
//! named for the relations of the facts that point at it (Q-M1f.398;
//! [`dot_util::ctx_frames`](crate::dot_util::ctx_frames)); a held one stands
//! in no frame, and no octagon is filled. A held fact that nothing points at
//! is drawn as it was: an arrow, a loop, or an octagon outside every frame.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use ein_core::{FactId, Kb, Prov, ProvKind, Tag, Terms, Value};

use crate::dot_util::{FRAMED_RANKING, ctx_frames, ctx_label, fact_key, hashed_id, quote};
use crate::palette::hash_color;

/// Per-relation deterministic colour (the default), per-origin colour, or no
/// colour at all.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColourBy {
    Relation,
    Origin,
    None,
}

impl ColourBy {
    pub fn parse(s: &str) -> Option<ColourBy> {
        match s {
            "relation" => Some(ColourBy::Relation),
            "origin" => Some(ColourBy::Origin),
            "none" => Some(ColourBy::None),
            _ => None,
        }
    }
}

/// The keyword surface of `kb.to_dot`.
pub struct KbDotOpts<'a> {
    pub colour_by: ColourBy,
    pub include_types: bool,
    pub include_instances: bool,
    /// A prior KB to diff against: facts present here but absent from it are
    /// drawn at `penwidth=3` — the transition highlight, "this step added E".
    pub since: Option<&'a Kb>,
    pub name: &'a str,
}

impl Default for KbDotOpts<'_> {
    fn default() -> Self {
        KbDotOpts {
            colour_by: ColourBy::Relation,
            include_types: true,
            include_instances: true,
            since: None,
            name: "kb",
        }
    }
}

/// A stable DOT identifier for an octagon hyperedge node — the shared
/// `f_<md5[:10]>` scheme, keyed by `(relation_name, args)`.
fn fact_node_id(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.fact(f);
    let args: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
    hashed_id("f_", &fact_key(terms.sym(rel), &args), false)
}

/// The short `(N)` inside `"condition (N)"`, or the original string when
/// there is no parenthesised group.
///
/// The group is the *first* `(` followed by at least one non-`)` character
/// and then a `)`.
fn short_source(source: &str) -> String {
    for (i, c) in source.char_indices() {
        if c != '(' {
            continue;
        }
        let rest = &source[i + 1..];
        if let Some(close) = rest.find(')')
            && close > 0
        {
            return format!("({})", &rest[..close]);
        }
        // `[^)]+` needs at least one character and a closing `)`; when this
        // `(` has neither, the regex engine retries from the next one.
    }
    source.to_string()
}

/// The `(type-names, instance-names)` the renderer draws, read from the
/// puzzle's own `(is-a Child Parent)` facts.
///
/// The Child→Parent edge is *not* returned: the `is-a` facts draw their own
/// type edge in the per-fact pass.
fn schema_nodes(kb: &Kb, terms: &Terms) -> (Vec<String>, Vec<String>) {
    let mut types: Vec<String> = Vec::new();
    let mut children: Vec<String> = Vec::new();
    let mut parents: Vec<String> = Vec::new();
    let Some(is_a) = terms.syms.get("is-a") else {
        return (types, Vec::new());
    };
    let push = |v: &mut Vec<String>, s: &str| {
        if !v.iter().any(|x| x == s) {
            v.push(s.to_string());
        }
    };
    for f in kb.facts() {
        let (rel, args) = terms.fact(f);
        // `_two_strs`: at least two args, the first two both `str`.
        if rel != is_a || args.len() < 2 || args[0].tag() != Tag::Sym || args[1].tag() != Tag::Sym {
            continue;
        }
        let (child, parent) = (terms.display(args[0]), terms.display(args[1]));
        push(&mut children, &child);
        push(&mut parents, &parent);
        push(&mut types, &parent);
    }
    let insts: Vec<String> = children
        .into_iter()
        .filter(|c| !parents.contains(c) && !types.contains(c))
        .collect();
    (types, insts)
}

fn emit_type_node(name: &str) -> String {
    format!("  {} [shape=box, label={}];", quote(name), quote(name))
}

fn emit_instance_node(name: &str) -> String {
    format!("  {} [shape=oval, label={}];", quote(name), quote(name))
}

/// The dashed empty-arrow type edge.
fn emit_is_a_edge(child: &str, parent: &str, penwidth: Option<u32>) -> String {
    let pw = penwidth.map_or(String::new(), |p| format!(", penwidth={p}"));
    format!("  {child} -> {parent} [style=dashed, arrowhead=empty, label=\"is-a\"{pw}];")
}

struct Styling {
    colour: &'static str,
    style: &'static str,
    label_extra: Option<String>,
    penwidth: Option<u32>,
}

/// The DOT id of what stands under an argument arrow: a name's, quoted, or
/// the node of the fact that stands there.
fn arg_id(terms: &Terms, a: Value) -> String {
    match a.as_fact() {
        Some(g) => fact_node_id(terms, g),
        None => quote(&terms.display(a)),
    }
}

fn emit_binary_fact(terms: &Terms, f: FactId, s: &Styling) -> String {
    let (rel, args) = terms.fact(f);
    let mut label_parts = vec![terms.sym(rel).to_string()];
    if let Some(extra) = &s.label_extra {
        label_parts.push(extra.clone());
    }
    let mut attrs = vec![
        format!("label={}", quote(&label_parts.join(" "))),
        format!("color=\"{}\"", s.colour),
        format!("fontcolor=\"{}\"", s.colour),
        format!("style={}", s.style),
    ];
    if let Some(p) = s.penwidth {
        attrs.push(format!("penwidth={p}"));
    }
    format!(
        "  {} -> {} [{}];",
        arg_id(terms, args[0]),
        arg_id(terms, args[1]),
        attrs.join(", ")
    )
}

/// A unary fact as a labelled **self-loop** on its single argument — the
/// compact view's convention for the predicate-as-subset idiom, and the
/// degenerate case of the binary collapse with source == target.
fn emit_unary_fact(terms: &Terms, f: FactId, s: &Styling) -> String {
    let (rel, args) = terms.fact(f);
    let mut label_parts = vec![terms.sym(rel).to_string()];
    if let Some(extra) = &s.label_extra {
        label_parts.push(extra.clone());
    }
    let mut attrs = vec![
        format!("label={}", quote(&label_parts.join(" "))),
        format!("color=\"{}\"", s.colour),
        format!("fontcolor=\"{}\"", s.colour),
        format!("style={}", s.style),
    ];
    if let Some(p) = s.penwidth {
        attrs.push(format!("penwidth={p}"));
    }
    let only = quote(&terms.display(args[0]));
    format!("  {only} -> {only} [{}];", attrs.join(", "))
}

/// A fact's octagon: its node's declaration, with no indent.
fn hyperedge_node(terms: &Terms, f: FactId, s: &Styling) -> String {
    let nid = fact_node_id(terms, f);
    let (rel, _) = terms.fact(f);
    let mut head_label = format!("({})", terms.sym(rel));
    if let Some(extra) = &s.label_extra {
        head_label.push_str(&format!("\\n{extra}"));
    }
    let pw = s
        .penwidth
        .map_or(String::new(), |p| format!(", penwidth={p}"));
    let (colour, style) = (s.colour, s.style);
    format!(
        "{nid} [shape=octagon, label={}, color=\"{colour}\", fontcolor=\"{colour}\", \
         style={style}{pw}];",
        quote(&head_label)
    )
}

/// The numbered argument arrows of a fact's octagon.
fn hyperedge_arrows(terms: &Terms, f: FactId, s: &Styling) -> Vec<String> {
    let nid = fact_node_id(terms, f);
    let (_, args) = terms.fact(f);
    let pw = s
        .penwidth
        .map_or(String::new(), |p| format!(", penwidth={p}"));
    let (colour, style) = (s.colour, s.style);
    args.iter()
        .enumerate()
        .map(|(i, a)| {
            format!(
                "  {nid} -> {} [label=\"#{}\", color=\"{colour}\", style={style}{pw}];",
                arg_id(terms, *a),
                i + 1
            )
        })
        .collect()
}

/// A non-binary fact: one octagon node plus a labelled edge per argument.
/// Arity 0 lands here too, with no participants.
fn emit_hyperedge(terms: &Terms, f: FactId, s: &Styling) -> Vec<String> {
    let mut lines = vec![format!("  {}", hyperedge_node(terms, f, s))];
    lines.extend(hyperedge_arrows(terms, f, s));
    lines
}

// ── per-fact decisions ─────────────────────────────────────────────

fn prov_of<'a>(kb: &Kb, terms: &'a Terms, f: FactId) -> Option<&'a Prov> {
    kb.primary(f).map(|p| terms.provs.get(p))
}

fn is_derived(kb: &Kb, terms: &Terms, f: FactId) -> bool {
    prov_of(kb, terms, f).is_some_and(|p| p.kind != ProvKind::Source)
}

/// Rule-application meta-facts — a head that names a `Rule` — are suppressed
/// as meta rather than data; so are `not`-headed facts, the stored
/// negations, which this view does not draw.
fn suppress(kb: &Kb, terms: &Terms, f: FactId) -> bool {
    let (rel, _) = terms.fact(f);
    kb.program().rules.get(rel).is_some() || rel == terms.kernel.not
}

fn pick_colour(kb: &Kb, terms: &Terms, f: FactId, colour_by: ColourBy) -> &'static str {
    match colour_by {
        ColourBy::Relation => {
            let (rel, _) = terms.fact(f);
            hash_color(terms.sym(rel))
        }
        ColourBy::Origin => {
            if is_derived(kb, terms, f) {
                "#1f77b4"
            } else if prov_of(kb, terms, f)
                .is_some_and(|p| p.kind == ProvKind::Source && p.source.is_some())
            {
                "#000000"
            } else {
                "#444444"
            }
        }
        ColourBy::None => "#000000",
    }
}

/// The bit after the relation name on the edge label: an authored condition
/// contributes its short source id, an engine derivation `by <rule-name>`,
/// and a background assumption nothing.
fn label_extra(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
    let prov = prov_of(kb, terms, f)?;
    if prov.kind == ProvKind::Source
        && let Some(s) = prov.source
    {
        // An empty `:source` falls through to the rule branch exactly as an
        // absent one does.
        let text = terms.sym(s);
        if !text.is_empty() {
            return Some(short_source(text));
        }
    }
    if prov.kind == ProvKind::Rule
        && let Some(r) = prov.rule
    {
        return Some(format!("by {}", terms.sym(r)));
    }
    None
}

/// How a held fact is drawn: by its relation or its origin, dashed where a
/// rule derived it, with the short text of where it comes from.
fn styling(kb: &Kb, terms: &Terms, f: FactId, colour_by: ColourBy, new: bool) -> Styling {
    Styling {
        colour: pick_colour(kb, terms, f, colour_by),
        style: if is_derived(kb, terms, f) {
            "dashed"
        } else {
            "solid"
        },
        label_extra: label_extra(kb, terms, f),
        penwidth: if new { Some(3) } else { None },
    }
}

fn emit_fact_line(
    kb: &Kb,
    terms: &Terms,
    f: FactId,
    colour_by: ColourBy,
    new: bool,
) -> Vec<String> {
    let s = styling(kb, terms, f, colour_by, new);
    let (rel, args) = terms.fact(f);
    if terms.sym(rel) == "is-a" && args.len() == 2 {
        return vec![emit_is_a_edge(
            &arg_id(terms, args[0]),
            &arg_id(terms, args[1]),
            s.penwidth,
        )];
    }
    if args.len() == 2 {
        return vec![emit_binary_fact(terms, f, &s)];
    }
    if args.len() == 1 && args[0].tag() == Tag::Sym {
        return vec![emit_unary_fact(terms, f, &s)];
    }
    emit_hyperedge(terms, f, &s)
}

// ── facts in a context ─────────────────────────────────────────────

/// The facts that a drawn fact points at, in the order they are first met,
/// each with the relations of the facts that point at it.
///
/// A drawn fact is one of `drawn`, the held facts the view draws, or a fact
/// one of them points at, at any depth: such a fact is drawn as its node, so
/// what it points at is pointed at by a drawn fact too. The order is the
/// order of `drawn` and, under one fact, of its arguments, so it does not
/// depend on the ids the facts were interned under.
fn in_contexts(terms: &Terms, drawn: &[FactId]) -> Vec<(FactId, BTreeSet<String>)> {
    let mut found: Vec<(FactId, BTreeSet<String>)> = Vec::new();
    let mut at: HashMap<FactId, usize> = HashMap::new();
    let mut walked: HashSet<FactId> = HashSet::new();
    let mut queue: VecDeque<FactId> = drawn.iter().copied().collect();
    while let Some(f) = queue.pop_front() {
        if !walked.insert(f) {
            continue;
        }
        let (rel, args) = terms.fact(f);
        for g in args.iter().filter_map(|a| a.as_fact()) {
            let i = *at.entry(g).or_insert_with(|| {
                found.push((g, BTreeSet::new()));
                queue.push_back(g);
                found.len() - 1
            });
            found[i].1.insert(terms.sym(rel).to_string());
        }
    }
    found
}

// ── the entry point ────────────────────────────────────────────────

/// Render a knowledge base as a unified Graphviz digraph.
pub fn to_dot(kb: &Kb, terms: &Terms, opts: &KbDotOpts) -> String {
    let since_keys: Option<HashSet<FactId>> = opts.since.map(|s| s.facts().collect());
    let mut lines: Vec<String> = vec![
        format!("digraph {} {{", opts.name),
        // `fdp` is the layout engine; the comment is the hint that lets
        // `render_examples.sh` pick it for kb outputs.
        "  // suggested layout: fdp".to_string(),
        "  rankdir=BT;".to_string(),
        "  node [fontname=\"Inter\"];".to_string(),
    ];

    let (schema_types, schema_insts) = schema_nodes(kb, terms);
    let type_set: Vec<String> = if opts.include_types {
        schema_types
    } else {
        Vec::new()
    };
    let mut type_names = type_set.clone();
    type_names.sort();
    // Skip ovals for a name already drawn as a type box.
    let mut inst_names: Vec<String> = if opts.include_instances {
        schema_insts
            .into_iter()
            .filter(|n| !type_set.contains(n))
            .collect()
    } else {
        Vec::new()
    };
    inst_names.sort();
    if !type_names.is_empty() {
        lines.push("  // types".to_string());
        lines.extend(type_names.iter().map(|n| emit_type_node(n)));
    }
    if !inst_names.is_empty() {
        lines.push("  // instances".to_string());
        lines.extend(inst_names.iter().map(|n| emit_instance_node(n)));
    }

    if kb.n_facts() > 0 {
        lines.push(String::new());
        lines.push("  // facts".to_string());
    }
    let drawn: Vec<FactId> = kb.facts().filter(|f| !suppress(kb, terms, *f)).collect();
    let nested = in_contexts(terms, &drawn);
    let is_nested: HashSet<FactId> = nested.iter().map(|(g, _)| *g).collect();
    let is_new = |f: FactId| since_keys.as_ref().is_some_and(|k| !k.contains(&f));
    for &f in &drawn {
        // A fact that a fact points at is its node, below, and no arrow.
        if is_nested.contains(&f) {
            continue;
        }
        lines.extend(emit_fact_line(kb, terms, f, opts.colour_by, is_new(f)));
    }
    if !nested.is_empty() {
        lines.push(String::new());
        lines.push("  // facts in a context".to_string());
        // A fact in a context is drawn as a held fact is where the knowledge
        // base holds it, and stands in no frame; one it does not hold, in a
        // context only, has no origin and stands in the frame of the
        // relations that point at it.
        let style_of = |g: FactId| {
            if kb.contains(g) {
                styling(kb, terms, g, opts.colour_by, is_new(g))
            } else {
                Styling {
                    colour: pick_colour(kb, terms, g, opts.colour_by),
                    style: "solid",
                    label_extra: None,
                    penwidth: None,
                }
            }
        };
        let mut held: Vec<String> = Vec::new();
        let mut members: Vec<(String, String)> = Vec::new();
        for (g, by) in &nested {
            let decl = hyperedge_node(terms, *g, &style_of(*g));
            if kb.contains(*g) {
                held.push(format!("  {decl}"));
            } else {
                members.push((ctx_label(by), decl));
            }
        }
        if !members.is_empty() {
            lines.push(format!("  {FRAMED_RANKING};"));
        }
        lines.extend(held);
        lines.extend(ctx_frames("", "  ", &members));
        for (g, _) in &nested {
            lines.extend(hyperedge_arrows(terms, *g, &style_of(*g)));
        }
    }
    lines.push("}".to_string());
    lines.join("\n")
}
