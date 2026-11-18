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
//! **Laid out in layers** (Q21): a fact's first arrow is written from its
//! relation's atom to the fact, `"rel" -> f [xlabel="1", dir=back]`, so that
//! it is drawn as it is, from the fact to the relation, and `dot` ranks the
//! relation above the fact; its next arrows go down to its arguments. The
//! drawing writes its layers too (`FactSet::layers`), objects and facts in
//! turn, a band of facts as deep as its facts nest: `relation` on top, the
//! declarations, the relations and their labels, the relations' facts, the
//! plain objects, where no fact nests. Only the graph's structure is fixed;
//! the layers are a preference, and no test holds them.
//!
//! **Folded** (`fold`), a fact of three arrows, a relation and two
//! arguments, that no drawn fact points at is one arrow from its first
//! argument to its second, its relation's name written on it; a fact of two
//! arrows, a relation and one argument, that no drawn fact points at is a
//! loop on its argument, its relation's name written on it (Q-M1f.425).
//! Nothing else is folded: a fact of one arrow, or of four or more, stays an
//! octagon, and so does a fact that a drawn fact points at, which is a node.
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

/// A fact's first arrow, numbered `1`, from the fact's node `fact` to its
/// relation's node `relation`, as one DOT line with no indent: written from
/// the relation to the fact and drawn back, so that `dot` ranks the
/// relation above the fact and the arrow still points at the relation.
/// `attrs` are the arrow's other attributes, each after a `, `.
pub fn first_arrow(fact: &str, relation: &str, attrs: &str) -> String {
    format!("{relation} -> {fact} [xlabel=\"1\", dir=back{attrs}];")
}

/// Whether a fact of `args` arguments folds, where nothing drawn points at
/// it: a fact of three arrows into one arrow, of two into a loop
/// (Q-M1f.425).
pub fn folds(args: usize) -> bool {
    args == 1 || args == 2
}

/// The attributes of a layer's anchor: a node drawn as nothing.
const ANCHOR: &str = "style=invis, label=\"\", width=0, height=0, fixedsize=true";

/// The id of the anchor of the `i`-th layer, top first.
fn anchor(i: usize) -> String {
    format!("\"layer {i}\"")
}

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
            "  {}",
            first_arrow(&fact.id, &quote(&fact.relation), &format!(", {tail}"))
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
    /// second, and a fact of two as a loop on its argument, labelled with its
    /// relation's name and its note: the arrow's `xlabel`, as a number is,
    /// since one `label` in a digraph makes `dot` lay out every arrow over
    /// two ranks. A folded arrow ranks nothing (`constraint=false`): its
    /// arguments stand in the layer where the fact's octagon would put them,
    /// the canonical drawing's, and the arrow runs within that layer.
    fn folded(&self, f: usize) -> String {
        let fact = &self.facts[f];
        let look = &fact.look;
        let mut label = fact.relation.clone();
        if let Some(n) = &look.note {
            label.push(' ');
            label.push_str(n);
        }
        let from = self.under_id(&fact.args[0]);
        let to = fact.args.get(1).map_or(from.clone(), |a| self.under_id(a));
        format!(
            "  {from} -> {to} [xlabel={}, color=\"{c}\", fontcolor=\"{c}\", style={}{}, constraint=false];",
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
        // The octagons, in the order drawn: the facts not folded, then the
        // facts a fact points at.
        let mut octagons: Vec<usize> = Vec::new();
        let mut folded: Vec<usize> = Vec::new();
        for &f in &self.drawn {
            // A fact that a fact points at is its node, below, and no arrow.
            if is_nested.contains(&f) {
                continue;
            }
            if fold && folds(self.facts[f].args.len()) {
                lines.push(self.folded(f));
                folded.push(f);
            } else {
                lines.push(format!("  {}", self.node(f)));
                lines.extend(self.arrows(f));
                octagons.push(f);
            }
        }
        octagons.extend(nested.iter().map(|(g, _)| *g));
        let layers = self.layers(&octagons, &folded);
        if nested.is_empty() {
            lines.extend(layers);
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
        lines.extend(layers);
        lines
    }

    /// The drawing's layers, as `rank=same` groups, one a line, top first:
    /// the objects and the facts in turn, a sandwich, and a band of facts
    /// as deep as its facts nest (Q21). A fact stands in the band under its
    /// relation's layer, or under the band of a fact that points at it, the
    /// lower; in its band, one rank below the deepest fact of the band that
    /// points at it, so the facts nothing points at share the band's top rank
    /// and a fact under an arrow stands below the fact whose arrow it is
    /// under. A name under a fact's arrow stands in the layer under the
    /// fact's band; a name under no fact's argument arrow, a relation and
    /// nothing else, in the layer above its highest fact. Read off the arrows
    /// alone, no name; where the arrows go round (a fact of a relation whose
    /// name is under its own arrows) there are no layers to write, and none
    /// is written. A folded fact is no node and no group holds it, but it
    /// places its names as its octagon would, in the layer under the band it
    /// would stand in, so that a folded arrow runs within a layer; a fact its
    /// arrow ends at stands in that band too, on the band's top rank, where
    /// the folded fact's octagon is not.
    fn layers(&self, octagons: &[usize], folded: &[usize]) -> Vec<String> {
        let is_octagon: HashSet<usize> = octagons.iter().copied().collect();
        let every: Vec<usize> = octagons.iter().chain(folded).copied().collect();
        let mut pointers: HashMap<usize, Vec<usize>> = HashMap::new();
        for &f in &every {
            for a in &self.facts[f].args {
                if let Under::Fact(g) = a
                    && is_octagon.contains(g)
                {
                    pointers.entry(*g).or_default().push(f);
                }
            }
        }
        let mut layer: HashMap<&str, usize> = HashMap::new();
        let mut band: HashMap<usize, (usize, usize)> = every.iter().map(|&f| (f, (0, 0))).collect();
        let mut settled = false;
        for _ in 0..(2 * every.len() + 2) {
            let mut moved = false;
            for &f in &every {
                let fact = &self.facts[f];
                let ptrs = pointers.get(&f).map_or(&[][..], Vec::as_slice);
                let b = ptrs
                    .iter()
                    .map(|p| band[p].0)
                    .chain([
                        band[&f].0,
                        layer.get(fact.relation.as_str()).copied().unwrap_or(0),
                    ])
                    .max()
                    .unwrap_or(0);
                let d = ptrs
                    .iter()
                    .filter(|p| is_octagon.contains(p) && band[p].0 == b)
                    .map(|p| band[p].1 + 1)
                    .max()
                    .unwrap_or(0);
                if band[&f] != (b, d) {
                    band.insert(f, (b, d));
                    moved = true;
                }
                for a in &fact.args {
                    if let Under::Name(n) = a {
                        let l = layer.entry(n.as_str()).or_insert(0);
                        if *l < b + 1 {
                            *l = b + 1;
                            moved = true;
                        }
                    }
                }
            }
            if !moved {
                settled = true;
                break;
            }
        }
        if !settled || every.is_empty() {
            return Vec::new();
        }
        // A fact of such a relation, a relation under no argument arrow,
        // that holds no fact and that no fact points at stands in the band
        // just above its names, not wherever its relation is.
        for &f in octagons {
            let fact = &self.facts[f];
            if layer.contains_key(fact.relation.as_str())
                || pointers.contains_key(&f)
                || fact.args.iter().any(|a| matches!(a, Under::Fact(_)))
            {
                continue;
            }
            let lowest = fact
                .args
                .iter()
                .filter_map(|a| match a {
                    Under::Name(n) => layer.get(n.as_str()).copied(),
                    Under::Fact(_) => None,
                })
                .min();
            if let Some(l) = lowest
                && l > band[&f].0 + 1
            {
                band.insert(f, (l - 1, 0));
            }
        }
        // A relation under no argument arrow, which no layer holds yet,
        // stands just above its highest fact.
        let mut above: HashMap<&str, usize> = HashMap::new();
        for &f in octagons {
            let r = self.facts[f].relation.as_str();
            if !layer.contains_key(r) {
                let b = band[&f].0;
                above.entry(r).and_modify(|l| *l = (*l).min(b)).or_insert(b);
            }
        }
        layer.extend(above);
        // The groups, each in the order its members are first drawn.
        let mut groups: Vec<((usize, usize, usize), Vec<String>)> = Vec::new();
        let mut put = |key: (usize, usize, usize), id: String| match groups
            .iter_mut()
            .find(|(k, _)| *k == key)
        {
            Some((_, ids)) if !ids.contains(&id) => ids.push(id),
            Some(_) => {}
            None => groups.push((key, vec![id])),
        };
        for &f in octagons {
            let fact = &self.facts[f];
            put((layer[fact.relation.as_str()], 0, 0), quote(&fact.relation));
            for a in &fact.args {
                if let Under::Name(n) = a {
                    put((layer[n.as_str()], 0, 0), quote(n));
                }
            }
        }
        for &f in folded {
            for a in &self.facts[f].args {
                if let Under::Name(n) = a {
                    put((layer[n.as_str()], 0, 0), quote(n));
                }
            }
        }
        for &f in octagons {
            let (b, d) = band[&f];
            put((b, 1, d), self.facts[f].id.clone());
        }
        groups.sort_by_key(|(k, _)| *k);
        // A group says only that its members share a rank; which group is
        // above which is said by a chain of invisible anchors, one in each
        // group, since two groups no arrow joins would otherwise share a
        // rank. An anchor's id holds a space, which no name does.
        let mut out = vec![String::new(), "  // layers".to_string()];
        for i in 0..groups.len() {
            out.push(format!("  {} [{ANCHOR}];", anchor(i)));
        }
        for i in 1..groups.len() {
            out.push(format!(
                "  {} -> {} [style=invis];",
                anchor(i - 1),
                anchor(i)
            ));
        }
        out.extend(groups.iter().enumerate().map(|(i, (_, ids))| {
            let members: Vec<String> = ids.iter().map(|i| format!("{i};")).collect();
            format!("  {{ rank=same; {}; {} }}", anchor(i), members.join(" "))
        }));
        out
    }
}
