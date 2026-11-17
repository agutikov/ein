//! The digraph accumulator behind the per-form IR renderer.
//!
//! Node ids are stored in their *quoted* DOT form. Re-declaring an id with no
//! attributes is a no-op; re-declaring it *with* attributes overrides, which
//! is how a name that first appears as a generic ground atom is refined when
//! the ontology later calls it a type. The override keeps the node's original
//! **position**, and nodes are emitted in first-declaration order, so a
//! `HashMap` here would scramble the file.
//!
//! A digraph that holds a hyperedge node sets
//! [`POLYGON_RANKSEP`].
//!
//! A node may be declared **in a frame**, the frame of a context
//! ([`ctx_frames`]): it is then written inside that
//! frame's subgraph, after the nodes that stand in none and before the
//! edges, and a digraph with no framed node is written as it always was.

use std::collections::HashMap;

use crate::dot_util::{FRAMED_RANKING, HYPER_SHAPE, POLYGON_RANKSEP, ctx_frames, quote};

#[derive(Default)]
pub struct Builder {
    name: String,
    /// Ids in first-declaration order, with their current attribute string.
    nodes: Vec<(String, String)>,
    /// Id → its index in `nodes`. Lookups only, so its own order is not
    /// observable ([03-design/01-determinism-and-order.md § 9](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#9-the-tests-that-hold-it)).
    index: HashMap<String, usize>,
    edges: Vec<String>,
    hcount: u32,
    /// Id → the label of the frame it is declared in. Lookups only.
    frames: HashMap<String, String>,
}

impl Builder {
    pub fn new(name: &str) -> Builder {
        Builder {
            name: name.to_string(),
            ..Builder::default()
        }
    }

    /// Declare a node. `None` attributes leave an existing declaration alone.
    pub fn node(&mut self, node_id: &str, attrs: Option<&str>) {
        match self.index.get(node_id) {
            Some(&i) => {
                if let Some(a) = attrs {
                    self.nodes[i].1 = a.to_string();
                }
            }
            None => {
                self.index.insert(node_id.to_string(), self.nodes.len());
                self.nodes
                    .push((node_id.to_string(), attrs.unwrap_or("").to_string()));
            }
        }
    }

    /// Declare a node in the frame labelled `frame`: a fact that a drawn
    /// fact points at. A node is in one frame, the first it is given.
    pub fn framed(&mut self, node_id: &str, attrs: &str, frame: &str) {
        self.node(node_id, Some(attrs));
        self.frames
            .entry(node_id.to_string())
            .or_insert_with(|| frame.to_string());
    }

    pub fn edge(&mut self, src: &str, dst: &str, attrs: Option<&str>) {
        let mut line = format!("  {src} -> {dst}");
        // An empty attribute string emits no brackets — not the same as
        // `None`, but rendered the same, which is why this takes one `Option`
        // and checks both.
        if let Some(a) = attrs.filter(|a| !a.is_empty()) {
            line.push_str(&format!(" [{a}]"));
        }
        line.push(';');
        self.edges.push(line);
    }

    /// Mint and declare a fresh hyperedge (Levi list-node) id.
    pub fn fresh_h(&mut self, label: &str) -> String {
        self.hcount += 1;
        let node_id = quote(&format!("h_{}_{}", self.hcount, label));
        self.node(
            &node_id,
            Some(&format!("shape={HYPER_SHAPE}, label=\"{label}\"")),
        );
        node_id
    }

    pub fn build(self) -> String {
        // A hyperedge is an octagon, and `fresh_h` and `framed` are the only
        // doors to one.
        let mut lines: Vec<String> = Vec::new();
        if self.hcount > 0 || !self.frames.is_empty() {
            lines.push(format!("  {POLYGON_RANKSEP};"));
        }
        if !self.frames.is_empty() {
            lines.push(format!("  {FRAMED_RANKING};"));
        }
        let decl = |id: &str, attrs: &str| {
            if attrs.is_empty() {
                format!("{id};")
            } else {
                format!("{id} [{attrs}];")
            }
        };
        let mut framed: Vec<(String, String)> = Vec::new();
        for (id, attrs) in &self.nodes {
            match self.frames.get(id) {
                Some(frame) => framed.push((frame.clone(), decl(id, attrs))),
                None => lines.push(format!("  {}", decl(id, attrs))),
            }
        }
        lines.extend(ctx_frames("", "  ", &framed));
        lines.extend(self.edges);
        format!("digraph {} {{\n{}\n}}", self.name, lines.join("\n"))
    }
}
