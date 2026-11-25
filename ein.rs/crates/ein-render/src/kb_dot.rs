//! The unified KB → DOT renderer.
//!
//! One `digraph` over the whole knowledge base, every fact it holds, **fusing**
//! entity identity across forms: `Norwegian` is one node, under the arrows of
//! every fact that names it, given or derived.
//!
//! **No relation is special-cased** (Q-M1f.421): every fact is drawn by
//! [`crate::fact_graph`], in the canonical form, or folded on request
//! (Q-M1f.419), `is-a` and `relation` as any other, and every atom is a box.
//! A stored negation and a fact that applies a rule are facts the knowledge
//! base holds, and are drawn as any other.
//!
//! **Every object is drawn once** (Q-M1f.395): a fact that a drawn fact points
//! at is its node, in the frame of its context where the knowledge base does
//! not hold it (Q-M1f.398).

use std::collections::{HashMap, HashSet};

use ein_core::{FactId, Kb, Prov, ProvKind, Terms, Value};

use crate::dot_util::POLYGON_RANKSEP;
use crate::fact_graph::{Fact, FactSet, Look, Under, fact_id};
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
    /// Fold a fact of three arrows that no drawn fact points at into one
    /// labelled arrow, and a fact of two into a loop (Q-M1f.425); by default
    /// every fact is its octagon.
    pub fold: bool,
    /// A prior KB to diff against: facts present here but absent from it are
    /// drawn at `penwidth=3` — the transition highlight, "this step added E".
    pub since: Option<&'a Kb>,
    /// **A slice** (Q-M1f.527): the held facts drawn, in the knowledge
    /// base's order, in place of every fact it holds, each with what it
    /// points at, at any depth; `ein_infer::slice`'s selections. `None`
    /// draws every fact.
    pub only: Option<&'a [FactId]>,
    pub name: &'a str,
}

impl Default for KbDotOpts<'_> {
    fn default() -> Self {
        KbDotOpts {
            colour_by: ColourBy::Relation,
            fold: false,
            since: None,
            only: None,
            name: "kb",
        }
    }
}

/// The short `(N)` inside `"condition (N)"`, or the original string when
/// there is no parenthesised group.
///
/// The group is the *first* `(` followed by at least one non-`)` character
/// and then a `)`.
pub fn short_source(source: &str) -> String {
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

// ── per-fact decisions ─────────────────────────────────────────────

fn prov_of<'a>(kb: &Kb, terms: &'a Terms, f: FactId) -> Option<&'a Prov> {
    kb.primary(f).map(|p| terms.provs.get(p))
}

fn is_derived(kb: &Kb, terms: &Terms, f: FactId) -> bool {
    prov_of(kb, terms, f).is_some_and(|p| p.kind != ProvKind::Source)
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

/// The note beside a fact: an authored condition's short source id, an
/// engine derivation's `by <rule-name>`, and a background assumption none.
fn note(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
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

/// How a fact is drawn: by its relation or its origin, dashed where a rule
/// derived it, with the short text of where it comes from beside it. A fact
/// the knowledge base does not hold, in a context only, has no origin.
fn look(
    kb: &Kb,
    terms: &Terms,
    f: FactId,
    opts: &KbDotOpts,
    since: Option<&HashSet<FactId>>,
) -> Look {
    if !kb.contains(f) {
        return Look::plain(pick_colour(kb, terms, f, opts.colour_by));
    }
    Look {
        colour: pick_colour(kb, terms, f, opts.colour_by),
        dashed: is_derived(kb, terms, f),
        note: note(kb, terms, f),
        penwidth: since.is_some_and(|k| !k.contains(&f)).then_some(3),
    }
}

/// The knowledge base's facts as a [`FactSet`]: every fact it holds, or the
/// slice [`KbDotOpts::only`] keeps, drawn, in its order, and every fact one
/// of them points at, at any depth.
fn fact_set(kb: &Kb, terms: &Terms, opts: &KbDotOpts) -> FactSet {
    let since: Option<HashSet<FactId>> = opts.since.map(|s| s.facts().collect());
    let mut set = FactSet::default();
    let mut at: HashMap<FactId, usize> = HashMap::new();
    let drawn: Box<dyn Iterator<Item = FactId>> = match opts.only {
        Some(only) => Box::new(only.iter().copied()),
        None => Box::new(kb.facts()),
    };
    for f in drawn {
        let i = add(kb, terms, opts, since.as_ref(), &mut set, &mut at, f);
        set.drawn.push(i);
    }
    set
}

fn add(
    kb: &Kb,
    terms: &Terms,
    opts: &KbDotOpts,
    since: Option<&HashSet<FactId>>,
    set: &mut FactSet,
    at: &mut HashMap<FactId, usize>,
    f: FactId,
) -> usize {
    if let Some(&i) = at.get(&f) {
        return i;
    }
    // A fact is added after every fact it points at, in the order of its
    // arguments, each once: a stack of the facts being added and how many of
    // their arguments are done, not the call stack, so a fact nested at any
    // depth is drawn.
    let mut adding: Vec<(FactId, usize)> = vec![(f, 0)];
    while let Some((g, done)) = adding.last_mut() {
        let g = *g;
        let args = terms.fact(g).1;
        let next =
            (*done..args.len()).find(|&j| args[j].as_fact().is_some_and(|h| !at.contains_key(&h)));
        if let Some(j) = next {
            *done = j + 1;
            adding.push((args[j].as_fact().expect("a fact"), 0));
            continue;
        }
        adding.pop();
        let (rel, args) = terms.fact(g);
        let texts: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
        let under: Vec<Under> = args
            .iter()
            .map(|a: &Value| match a.as_fact() {
                Some(h) => Under::Fact(at[&h]),
                None => Under::Name(terms.display(*a)),
            })
            .collect();
        let i = set.facts.len();
        set.facts.push(Fact {
            id: fact_id(terms.sym(rel), &texts),
            relation: terms.sym(rel).to_string(),
            args: under,
            held: kb.contains(g),
            look: look(kb, terms, g, opts, since),
        });
        at.insert(g, i);
    }
    at[&f]
}

/// The opening lines of a knowledge base's digraph, the KB view's and the
/// program view's `kb` alike: laid out by `dot` in layers from the top, each
/// relation above its facts and each fact above its arguments
/// ([`crate::fact_graph`]). Every fact is an octagon outside any cluster,
/// so the digraph sets [`POLYGON_RANKSEP`], as a rule's overlay does.
pub fn kb_digraph_open(name: &str) -> Vec<String> {
    vec![
        format!("digraph {name} {{"),
        "  rankdir=TB;".to_string(),
        format!("  {POLYGON_RANKSEP};"),
        "  node [fontname=\"Inter\", shape=box];".to_string(),
    ]
}

// ── the entry point ────────────────────────────────────────────────

/// Render a knowledge base as a unified Graphviz digraph.
///
/// A slice that keeps no fact ([`KbDotOpts::only`]) is a digraph with no node,
/// which says so.
pub fn to_dot(kb: &Kb, terms: &Terms, opts: &KbDotOpts) -> String {
    let mut lines = kb_digraph_open(opts.name);
    if opts.only.is_some_and(<[FactId]>::is_empty) {
        lines.push("  // the slice keeps no fact".to_string());
    }
    lines.extend(fact_set(kb, terms, opts).body(opts.fold));
    lines.push("}".to_string());
    lines.join("\n")
}
