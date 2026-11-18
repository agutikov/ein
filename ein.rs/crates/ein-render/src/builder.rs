//! The digraph accumulator behind a written trace's step graph.
//!
//! Node ids are stored in their *quoted* DOT form. Re-declaring an id with no
//! attributes is a no-op; re-declaring it *with* attributes overrides. The
//! override keeps the node's original **position**, and nodes are emitted in
//! first-declaration order, so a `HashMap` here would scramble the file.

use std::collections::HashMap;

#[derive(Default)]
pub struct Builder {
    name: String,
    /// Ids in first-declaration order, with their current attribute string.
    nodes: Vec<(String, String)>,
    /// Id → its index in `nodes`. Lookups only, so its own order is not
    /// observable ([03-design/01-determinism-and-order.md § 9](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#9-the-tests-that-hold-it)).
    index: HashMap<String, usize>,
    edges: Vec<String>,
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
        // An empty attribute string emits no brackets — not the same as
        // `None`, but rendered the same, which is why this takes one `Option`
        // and checks both.
        if let Some(a) = attrs.filter(|a| !a.is_empty()) {
            line.push_str(&format!(" [{a}]"));
        }
        line.push(';');
        self.edges.push(line);
    }

    pub fn build(self) -> String {
        let mut lines: Vec<String> = Vec::new();
        for (id, attrs) in &self.nodes {
            if attrs.is_empty() {
                lines.push(format!("  {id};"));
            } else {
                lines.push(format!("  {id} [{attrs}];"));
            }
        }
        lines.extend(self.edges);
        format!("digraph {} {{\n{}\n}}", self.name, lines.join("\n"))
    }
}
