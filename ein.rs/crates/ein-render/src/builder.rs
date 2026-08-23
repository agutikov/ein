//! The digraph accumulator behind the per-form IR renderer — `ir/to_dot`'s
//! `_Builder`.
//!
//! Node ids are stored in their *quoted* DOT form. Re-declaring an id with no
//! attributes is a no-op; re-declaring it *with* attributes overrides, which
//! is how a name that first appears as a generic ground atom is refined when
//! the ontology later calls it a type. The override keeps the node's original
//! **position** — ein.py's `self._nodes[node_id] = attrs` on a `dict`, and the
//! emission order is the dict's, so a `HashMap` here would scramble the file.

use std::collections::HashMap;

use crate::dot_util::{HYPER_SHAPE, quote};

#[derive(Default)]
pub struct Builder {
    name: String,
    /// Ids in first-declaration order, with their current attribute string.
    nodes: Vec<(String, String)>,
    /// Id → its index in `nodes`. Lookups only, so its own order is not
    /// observable ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §9).
    index: HashMap<String, usize>,
    edges: Vec<String>,
    hcount: u32,
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

    pub fn edge(&mut self, src: &str, dst: &str, attrs: Option<&str>) {
        let mut line = format!("  {src} -> {dst}");
        // ein.py tests the *string*, so an empty attribute string is falsy
        // and emits no brackets — not the same as `None`, but rendered the
        // same, which is why this takes one `Option` and checks both.
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
        let mut lines: Vec<String> = self
            .nodes
            .iter()
            .map(|(id, attrs)| {
                if attrs.is_empty() {
                    format!("  {id};")
                } else {
                    format!("  {id} [{attrs}];")
                }
            })
            .collect();
        lines.extend(self.edges);
        format!("digraph {} {{\n{}\n}}", self.name, lines.join("\n"))
    }
}
