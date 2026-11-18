//! A set of facts drawn as the knowledge base's graph: the one drawing of
//! facts that the KB view, the program view's `kb` digraph and the
//! constraint view share, so that a fact is drawn the same in each.
//!
//! **A fact is drawn in the canonical form** (Q-M1f.419): an octagon with no
//! name in it, its first arrow, numbered `1`, at its relation's atom, and an
//! arrow numbered `2`, `3`, … at each argument in turn, as
//! `docs/ein/reasoning/01-static-ir/01-knowledge-base.md` § The graph draws
//! one. The relation is a node labelled with its name, one node for every
//! fact of it in the drawing, and so is every other atom: a box, whatever
//! relation points at it. No relation is drawn otherwise than another
//! (Q-M1f.421).
//!
//! **Folded** (`fold`), a fact of three arrows, a relation and two
//! arguments, that no drawn fact points at is one arrow from its first
//! argument to its second, its relation's name written on it. Nothing else
//! is folded: a fact of two arrows stays an octagon, and so does a fact that
//! a drawn fact points at, which is a node.
//!
//! **Every word on an arrow is its `xlabel`**: an arrow's number, and a folded
//! arrow's relation, placed beside it once the graph is laid out. A `label` is
//! a node of `dot`'s layout, and one `label` in a digraph makes `dot` lay out
//! every arrow over two ranks, so that a knowledge base of some hundreds of
//! facts in long chains, which it lays out in seconds without one, takes it
//! minutes with one.
//!
//! **Every object is drawn once** (Q-M1f.395). A fact that a drawn fact
//! points at is its node, and no arrow beside it. Where it is in a context
//! only, pointed at and not held, it stands in a dashed frame named for the
//! relations of the facts that point at it (Q-M1f.398;
//! [`ctx_frames`]); a held one stands in no frame, and no octagon is filled.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::dot_util::{FRAMED_RANKING, HYPER_SHAPE, ctx_frames, ctx_label, quote};

/// The attributes every fact's octagon carries: a small node with no name in
/// it, as the reasoning pages draw one.
pub const FACT_NODE: &str = "label=\"\", width=0.3, height=0.3, fixedsize=true";

/// What stands under an arrow of a fact: a name, or a fact of the set.
#[derive(Clone, Debug)]
pub enum Under {
    Name(String),
    Fact(usize),
}

/// How a fact is marked: its colour, dashed where a rule derived it, a note
/// beside it (a source's short id, `by <rule>`), and a width where it is new.
#[derive(Clone, Debug)]
pub struct Look {
    pub colour: &'static str,
    pub dashed: bool,
    pub note: Option<String>,
    pub penwidth: Option<u32>,
}

impl Look {
    /// A fact drawn plain in `colour`: solid, with no note.
    pub fn plain(colour: &'static str) -> Look {
        Look {
            colour,
            dashed: false,
            note: None,
            penwidth: None,
        }
    }

    fn style(&self) -> &'static str {
        if self.dashed { "dashed" } else { "solid" }
    }

    fn width(&self) -> String {
        self.penwidth
            .map_or(String::new(), |p| format!(", penwidth={p}"))
    }
}

/// One fact of a drawing.
#[derive(Clone, Debug)]
pub struct Fact {
    /// Its node's DOT id, as written: [`fact_id`].
    pub id: String,
    /// Its relation's name, the label of the node its first arrow points at.
    pub relation: String,
    pub args: Vec<Under>,
    /// Held: drawn in no frame. A fact only pointed at and not held stands
    /// in the frame of what points at it.
    pub held: bool,
    pub look: Look,
}

/// The facts of one drawing: every fact a drawn fact points at is among
/// them, and `drawn` is the held facts the view draws, in its order.
#[derive(Default, Debug)]
pub struct FactSet {
    pub facts: Vec<Fact>,
    pub drawn: Vec<usize>,
}

/// The DOT id of a fact's node: `f_` and ten hex digits of the md5 of its
/// relation's name and its arguments' texts, the one scheme
/// ([`crate::dot_util::hashed_id`]) every view of a knowledge base names a
/// fact by.
pub fn fact_id(relation: &str, args: &[String]) -> String {
    crate::dot_util::hashed_id("f_", &crate::dot_util::fact_key(relation, args), false)
}

impl FactSet {
    fn under_id(&self, u: &Under) -> String {
        match u {
            Under::Name(n) => quote(n),
            Under::Fact(i) => self.facts[*i].id.clone(),
        }
    }

    /// The facts that a drawn fact points at, in the order first met, each
    /// with the relations of the facts that point at it. A fact found is
    /// drawn as its node, so what it points at is pointed at by a drawn fact
    /// too.
    fn in_contexts(&self) -> Vec<(usize, BTreeSet<String>)> {
        let mut found: Vec<(usize, BTreeSet<String>)> = Vec::new();
        let mut at: HashMap<usize, usize> = HashMap::new();
        let mut walked: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = self.drawn.iter().copied().collect();
        while let Some(f) = queue.pop_front() {
            if !walked.insert(f) {
                continue;
            }
            let fact = &self.facts[f];
            for a in &fact.args {
                let Under::Fact(g) = a else { continue };
                let i = *at.entry(*g).or_insert_with(|| {
                    found.push((*g, BTreeSet::new()));
                    queue.push_back(*g);
                    found.len() - 1
                });
                found[i].1.insert(fact.relation.clone());
            }
        }
        found
    }

    /// A fact's octagon: its node's declaration, with no indent.
    fn node(&self, f: usize) -> String {
        let fact = &self.facts[f];
        let look = &fact.look;
        let note = look
            .note
            .as_ref()
            .map_or(String::new(), |n| format!(", xlabel={}", quote(n)));
        format!(
            "{} [shape={HYPER_SHAPE}, {FACT_NODE}, color=\"{c}\", fontcolor=\"{c}\", style={}{}{note}];",
            fact.id,
            look.style(),
            look.width(),
            c = look.colour,
        )
    }

    /// A fact's arrows, numbered from the first, at its relation's atom.
    fn arrows(&self, f: usize) -> Vec<String> {
        let fact = &self.facts[f];
        let look = &fact.look;
        let tail = format!(
            "color=\"{}\", style={}{}",
            look.colour,
            look.style(),
            look.width()
        );
        let mut out = vec![format!(
            "  {} -> {} [xlabel=\"1\", {tail}];",
            fact.id,
            quote(&fact.relation)
        )];
        for (i, a) in fact.args.iter().enumerate() {
            out.push(format!(
                "  {} -> {} [xlabel=\"{}\", {tail}];",
                fact.id,
                self.under_id(a),
                i + 2
            ));
        }
        out
    }

    /// A fact of three arrows as one arrow from its first argument to its
    /// second, labelled with its relation's name and its note: the arrow's
    /// `xlabel`, as a number is, since one `label` in a digraph makes `dot`
    /// lay out every arrow over two ranks.
    fn folded(&self, f: usize) -> String {
        let fact = &self.facts[f];
        let look = &fact.look;
        let mut label = fact.relation.clone();
        if let Some(n) = &look.note {
            label.push(' ');
            label.push_str(n);
        }
        format!(
            "  {} -> {} [xlabel={}, color=\"{c}\", fontcolor=\"{c}\", style={}{}];",
            self.under_id(&fact.args[0]),
            self.under_id(&fact.args[1]),
            quote(&label),
            look.style(),
            look.width(),
            c = look.colour,
        )
    }

    /// The drawing's lines, each indented as a digraph's body: the drawn
    /// facts that nothing points at, then, under `// facts in a context`,
    /// the facts a drawn fact points at, the held ones in no frame and the
    /// others in their frames.
    pub fn body(&self, fold: bool) -> Vec<String> {
        let nested = self.in_contexts();
        let is_nested: HashSet<usize> = nested.iter().map(|(g, _)| *g).collect();
        let mut lines: Vec<String> = Vec::new();
        if !self.drawn.is_empty() {
            lines.push(String::new());
            lines.push("  // facts".to_string());
        }
        for &f in &self.drawn {
            // A fact that a fact points at is its node, below, and no arrow.
            if is_nested.contains(&f) {
                continue;
            }
            if fold && self.facts[f].args.len() == 2 {
                lines.push(self.folded(f));
            } else {
                lines.push(format!("  {}", self.node(f)));
                lines.extend(self.arrows(f));
            }
        }
        if nested.is_empty() {
            return lines;
        }
        lines.push(String::new());
        lines.push("  // facts in a context".to_string());
        let mut held: Vec<String> = Vec::new();
        let mut members: Vec<(String, String)> = Vec::new();
        for (g, by) in &nested {
            if self.facts[*g].held {
                held.push(format!("  {}", self.node(*g)));
            } else {
                members.push((ctx_label(by), self.node(*g)));
            }
        }
        if !members.is_empty() {
            lines.push(format!("  {FRAMED_RANKING};"));
        }
        lines.extend(held);
        lines.extend(ctx_frames("", "  ", &members));
        for (g, _) in &nested {
            lines.extend(self.arrows(*g));
        }
        lines
    }
}
