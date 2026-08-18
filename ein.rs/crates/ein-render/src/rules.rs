//! The rule → DOT renderer — `ein.py`'s `render/rules.py`.
//!
//! A `(rule Name (params…) :match … :assert … …)` as its *pattern →
//! conclusion* shape, in two modes: **sidebyside** (two clusters, the
//! readable view for a rule library) and **overlay** (match solid, assert
//! dashed, on one graph — the compact inline view).
//!
//! What it gets right that an edge dump does not: a per-panel id suffix that
//! never reaches the *label*, so `?a` appears as `?a` in both panels without
//! the two copies collapsing; guard predicates drawn as dotted undirected
//! `≠` / `=` links rather than as relation arrows, because they are computed
//! and not data; negation in red with a `¬` prefix; and a NAF `absent` guard
//! as its own `cluster_absent`, with the guard's binder-local variables
//! declared *inside* it and shared variables left outside — so "no such match
//! exists" survives instead of flattening into overlay arrows.

use ein_core::is_predicate;
use ein_ir::{Ast, Node, NodeId};

use crate::dot_util::{GROUND_SHAPE, HYPER_SHAPE, VAR_SHAPE, WILDCARD_ATTRS, quote, value_label};
use crate::palette::hash_color;

const NEG_COLOUR: &str = "#d62728"; // red — negated premises / conclusions
const GUARD_COLOUR: &str = "#888888"; // grey — NAF guard cluster chrome
const CONSTRAINT_COLOUR: &str = "#555555";

/// Constraint glyphs for the built-in guard predicates.
fn pred_glyph(name: &str) -> &str {
    match name {
        "neq" => "≠",
        "eq" => "=",
        other => other,
    }
}

/// Side-by-side LHS|RHS clusters (default) or the compact overlay. The legacy
/// single-letter names stay accepted.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RuleMode {
    SideBySide,
    Overlay,
}

impl RuleMode {
    pub fn parse(s: &str) -> Option<RuleMode> {
        match s {
            "a" | "sidebyside" | "side-by-side" => Some(RuleMode::SideBySide),
            "c" | "overlay" => Some(RuleMode::Overlay),
            _ => None,
        }
    }
}

// ── structural-head classification ─────────────────────────────────

/// The head atom's name, or `None` when the head is a `?var` / `_`.
fn head_name(ast: &Ast, expr: NodeId) -> Option<&str> {
    match ast.node(expr) {
        Node::SForm { head, .. } => match ast.node(head) {
            Node::Atom(s) => Some(ast.sym(s)),
            _ => None,
        },
        _ => None,
    }
}

fn is_guard(ast: &Ast, expr: NodeId) -> bool {
    matches!(head_name(ast, expr), Some("absent" | "forall"))
}

fn positional(ast: &Ast, expr: NodeId) -> Vec<NodeId> {
    ast.form_args(expr)
        .iter()
        .copied()
        .filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
        .collect()
}

fn form_head(ast: &Ast, expr: NodeId) -> NodeId {
    match ast.node(expr) {
        Node::SForm { head, .. } => head,
        _ => expr,
    }
}

fn is_sform(ast: &Ast, id: NodeId) -> bool {
    matches!(ast.node(id), Node::SForm { .. })
}

// ── node-occurrence analysis (for absent-cluster scoping) ──────────

/// Value-labels of every argument position in this clause tree.
///
/// Relation and predicate *heads* are edge labels, not nodes, so they are
/// excluded; the walk recurses through `and` / `or` / `not` / `absent` /
/// `forall` so a guard's inner argument nodes surface.
fn arg_nodes(ast: &Ast, expr: NodeId) -> Vec<String> {
    match head_name(ast, expr) {
        Some("and" | "or" | "absent" | "forall") => {
            let mut out = Vec::new();
            for child in ast.form_args(expr).to_vec() {
                if is_sform(ast, child) {
                    out.extend(arg_nodes(ast, child));
                }
            }
            out
        }
        Some("not") => match ast.form_args(expr).first().copied() {
            Some(first) if is_sform(ast, first) => arg_nodes(ast, first),
            _ => Vec::new(),
        },
        _ => positional(ast, expr)
            .iter()
            .map(|a| value_label(ast, *a))
            .collect(),
    }
}

/// First-seen order of argument-node labels across `clauses`.
fn ordered_nodes(ast: &Ast, clauses: &[NodeId]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for c in clauses {
        for n in arg_nodes(ast, *c) {
            if !seen.contains(&n) {
                seen.push(n);
            }
        }
    }
    seen
}

/// Where a node lives: at panel scope, or inside guard `n`'s cluster.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Home {
    Top,
    Guard(usize),
}

/// A node lives in a guard cluster iff it appears in exactly one guard and
/// nowhere in the top-level clauses — that makes it the guard's binder-local
/// variable. Everything shared stays at the top so its edges cross into the
/// cluster rather than pulling the node inside.
fn node_homes(ast: &Ast, top: &[NodeId], guards: &[NodeId]) -> Vec<(String, Home)> {
    let mut top_nodes: Vec<String> = Vec::new();
    for c in top {
        for n in arg_nodes(ast, *c) {
            if !top_nodes.contains(&n) {
                top_nodes.push(n);
            }
        }
    }
    let guard_nodes: Vec<Vec<String>> = guards.iter().map(|g| arg_nodes(ast, *g)).collect();
    let mut every: Vec<String> = top_nodes.clone();
    for gs in &guard_nodes {
        for n in gs {
            if !every.contains(n) {
                every.push(n.clone());
            }
        }
    }
    every
        .into_iter()
        .map(|n| {
            let in_guards: Vec<usize> = guard_nodes
                .iter()
                .enumerate()
                .filter(|(_, gs)| gs.contains(&n))
                .map(|(i, _)| i)
                .collect();
            let home = if !top_nodes.contains(&n) && in_guards.len() == 1 {
                Home::Guard(in_guards[0])
            } else {
                Home::Top
            };
            (n, home)
        })
        .collect()
}

fn home_of(homes: &[(String, Home)], label: &str) -> Option<Home> {
    homes.iter().find(|(n, _)| n == label).map(|(_, h)| *h)
}

// ── shapes / ids ───────────────────────────────────────────────────

fn shape_attrs(nodelabel: &str) -> String {
    if nodelabel == "_" {
        WILDCARD_ATTRS.to_string()
    } else if nodelabel.starts_with('?') {
        format!("shape={VAR_SHAPE}")
    } else {
        format!("shape={GROUND_SHAPE}")
    }
}

/// Quoted DOT id: the clean label plus a per-panel disambiguating suffix.
fn nid(nodelabel: &str, suffix: &str) -> String {
    quote(&format!("{nodelabel}{suffix}"))
}

// ── the renderer ───────────────────────────────────────────────────

/// Accumulates one rule's DOT, panel by panel. The hyperedge counter is
/// per-*rule*, not per-panel, so an n-ary clause on the right keeps counting
/// from where the left panel stopped.
#[derive(Default)]
struct RuleRenderer {
    hcount: u32,
}

/// A panel's node declarations, in first-declaration order.
type Decls = Vec<(String, String)>;

fn decls_setdefault(decls: &mut Decls, key: String, value: String) {
    if !decls.iter().any(|(k, _)| *k == key) {
        decls.push((key, value));
    }
}

impl RuleRenderer {
    fn fresh_hyper(&mut self, label: &str, suffix: &str) -> String {
        self.hcount += 1;
        quote(&format!("h{}_{label}{suffix}", self.hcount))
    }

    fn clause_lines(
        &mut self,
        ast: &Ast,
        clause: NodeId,
        suffix: &str,
        dashed: bool,
        negative: bool,
    ) -> Vec<String> {
        match head_name(ast, clause) {
            Some("and" | "or") => {
                let mut out = Vec::new();
                for child in ast.form_args(clause).to_vec() {
                    if is_sform(ast, child) {
                        out.extend(self.clause_lines(ast, child, suffix, dashed, negative));
                    }
                }
                out
            }
            Some("not") => match ast.form_args(clause).first().copied() {
                Some(inner) if is_sform(ast, inner) => {
                    self.clause_lines(ast, inner, suffix, dashed, true)
                }
                _ => Vec::new(),
            },
            Some(hn) if is_predicate(hn) => vec![self.constraint_line(ast, clause, suffix)],
            // An `absent` / `forall` reached here is nested inside another
            // clause — render its body inline as forbidden. The top-level
            // case is a cluster, handled by `panel`.
            Some("absent" | "forall") => {
                let mut out = Vec::new();
                for child in ast.form_args(clause).to_vec() {
                    if is_sform(ast, child) {
                        out.extend(self.clause_lines(ast, child, suffix, dashed, true));
                    }
                }
                out
            }
            _ => self.relation_lines(ast, clause, suffix, dashed, negative),
        }
    }

    fn relation_lines(
        &mut self,
        ast: &Ast,
        clause: NodeId,
        suffix: &str,
        dashed: bool,
        negative: bool,
    ) -> Vec<String> {
        let head_label = value_label(ast, form_head(ast, clause));
        let pos = positional(ast, clause);
        let colour = if negative {
            NEG_COLOUR
        } else {
            hash_color(head_label.trim_start_matches('?'))
        };
        let label = if negative {
            format!("¬{head_label}")
        } else {
            head_label.clone()
        };
        let mut attrs = vec![
            format!("label={}", quote(&label)),
            format!("color=\"{colour}\""),
            format!("fontcolor=\"{colour}\""),
        ];
        if dashed {
            attrs.push("style=dashed".to_string());
        }
        let attr_s = attrs.join(", ");
        if pos.len() == 2 {
            return vec![format!(
                "{} -> {} [{attr_s}];",
                nid(&value_label(ast, pos[0]), suffix),
                nid(&value_label(ast, pos[1]), suffix)
            )];
        }
        // n-ary (or arity 0/1): a Levi octagon list-node + role edges.
        let h = self.fresh_hyper(head_label.trim_start_matches('?'), suffix);
        let mut lines = vec![format!(
            "{h} [shape={HYPER_SHAPE}, label={}, color=\"{colour}\", fontcolor=\"{colour}\"];",
            quote(&label)
        )];
        let edge_style = if dashed { ", style=dashed" } else { "" };
        for (i, arg) in pos.iter().enumerate() {
            lines.push(format!(
                "{h} -> {} [label=\"{}\", color=\"{colour}\"{edge_style}];",
                nid(&value_label(ast, *arg), suffix),
                i + 1
            ));
        }
        lines
    }

    fn constraint_line(&mut self, ast: &Ast, clause: NodeId, suffix: &str) -> String {
        let head = form_head(ast, clause);
        let head_name = match ast.node(head) {
            Node::Atom(s) => ast.sym(s).to_string(),
            _ => value_label(ast, head),
        };
        let glyph = pred_glyph(&head_name).to_string();
        let pos = positional(ast, clause);
        if pos.len() != 2 {
            // Defensive — `eq` / `neq` are binary.
            return format!("// constraint {}", value_label(ast, clause));
        }
        format!(
            "{} -> {} [label=\"{glyph}\", dir=none, style=dotted, \
             color=\"{CONSTRAINT_COLOUR}\", fontcolor=\"{CONSTRAINT_COLOUR}\", constraint=false];",
            nid(&value_label(ast, pos[0]), suffix),
            nid(&value_label(ast, pos[1]), suffix)
        )
    }

    /// One pattern → (node-decl map, ordered body lines).
    ///
    /// Top-home nodes come back in the decl map, for the caller to emit at
    /// panel scope; guard-local nodes are declared inside their
    /// `cluster_absent` block within the body lines.
    fn panel(
        &mut self,
        ast: &Ast,
        pattern: Option<NodeId>,
        suffix: &str,
        dashed: bool,
    ) -> (Decls, Vec<String>) {
        let mut decls: Decls = Vec::new();
        let mut body: Vec<String> = Vec::new();
        let Some(pattern) = pattern else {
            return (decls, body);
        };
        let clauses: Vec<NodeId> = if head_name(ast, pattern) == Some("and") {
            ast.form_args(pattern)
                .iter()
                .copied()
                .filter(|c| is_sform(ast, *c))
                .collect()
        } else {
            vec![pattern]
        };
        let top: Vec<NodeId> = clauses
            .iter()
            .copied()
            .filter(|c| !is_guard(ast, *c))
            .collect();
        let guards: Vec<NodeId> = clauses
            .iter()
            .copied()
            .filter(|c| is_guard(ast, *c))
            .collect();
        let homes = node_homes(ast, &top, &guards);

        // Panel-scope node declarations (shared / top-home nodes).
        for nl in ordered_nodes(ast, &clauses) {
            if home_of(&homes, &nl) == Some(Home::Top) {
                decls_setdefault(
                    &mut decls,
                    nid(&nl, suffix),
                    format!("label={}, {}", quote(&nl), shape_attrs(&nl)),
                );
            }
        }

        for c in &top {
            body.extend(self.clause_lines(ast, *c, suffix, dashed, false));
        }

        // Each guard → its own cluster (local decls + inner lines).
        for (gi, guard) in guards.iter().enumerate() {
            let kind = head_name(ast, *guard).unwrap_or("absent").to_string();
            let scope = suffix.trim_matches('_');
            let scope = if scope.is_empty() { "o" } else { scope };
            let cid = format!("cluster_{kind}_{scope}_{gi}");
            let glyph = if kind == "absent" { "∄" } else { "∀" };
            let mut block = vec![
                format!("subgraph {cid} {{"),
                format!(
                    "  label=\"{kind} ({glyph})\"; style=\"dashed,rounded\"; \
                     color=\"{GUARD_COLOUR}\"; fontcolor=\"{GUARD_COLOUR}\";"
                ),
            ];
            for nl in ordered_nodes(ast, &[*guard]) {
                if home_of(&homes, &nl) == Some(Home::Guard(gi)) {
                    block.push(format!(
                        "  {} [label={}, {}];",
                        nid(&nl, suffix),
                        quote(&nl),
                        shape_attrs(&nl)
                    ));
                }
            }
            for child in ast.form_args(*guard).to_vec() {
                if is_sform(ast, child) {
                    for ln in self.clause_lines(ast, child, suffix, dashed, false) {
                        block.push(format!("  {ln}"));
                    }
                }
            }
            block.push("}".to_string());
            body.extend(block);
        }
        (decls, body)
    }
}

// ── field extraction ───────────────────────────────────────────────

/// `(name, match-expr, assert-expr)` from a `(rule …)` form.
fn extract(ast: &Ast, rule: NodeId) -> (String, Option<NodeId>, Option<NodeId>) {
    let mut name = "anon".to_string();
    let mut match_expr = None;
    let mut assert_expr = None;
    for (i, arg) in ast.form_args(rule).to_vec().into_iter().enumerate() {
        match ast.node(arg) {
            Node::Atom(s) if i == 0 => name = ast.sym(s).to_string(),
            Node::KwPair { key, value } => {
                let key = match ast.node(key) {
                    Node::Keyword(s) => ast.sym(s),
                    _ => continue,
                };
                if key == "match" && is_sform(ast, value) {
                    match_expr = Some(value);
                } else if key == "assert" && is_sform(ast, value) {
                    assert_expr = Some(value);
                }
            }
            _ => {}
        }
    }
    (name, match_expr, assert_expr)
}

// ── public entry points ────────────────────────────────────────────

/// One `(rule …)` form as a DOT digraph.
pub fn render_rule_form(ast: &Ast, rule: NodeId, mode: RuleMode) -> String {
    let (name, match_expr, assert_expr) = extract(ast, rule);
    let safe = name.replace(['-', ' '], "_");
    match mode {
        RuleMode::SideBySide => render_sidebyside(ast, &safe, &name, match_expr, assert_expr),
        RuleMode::Overlay => render_overlay(ast, &safe, &name, match_expr, assert_expr),
    }
}

fn decl_lines(decls: &Decls) -> Vec<String> {
    decls
        .iter()
        .map(|(nid, attrs)| format!("{nid} [{attrs}];"))
        .collect()
}

fn render_sidebyside(
    ast: &Ast,
    safe: &str,
    name: &str,
    match_expr: Option<NodeId>,
    assert_expr: Option<NodeId>,
) -> String {
    let mut r = RuleRenderer::default();
    let (m_decls, m_body) = r.panel(ast, match_expr, "_L", false);
    let (a_decls, a_body) = r.panel(ast, assert_expr, "_R", true);
    let mut out = vec![
        format!("digraph rule_{safe}_lhs_rhs {{"),
        "  rankdir=TB;".to_string(),
        format!("  label={}; labelloc=t;", quote(name)),
        "  subgraph cluster_lhs { label=\"match\";".to_string(),
    ];
    out.extend(decl_lines(&m_decls).iter().map(|ln| format!("    {ln}")));
    out.extend(m_body.iter().map(|ln| format!("    {ln}")));
    out.push("  }".to_string());
    out.push("  subgraph cluster_rhs { label=\"assert\";".to_string());
    out.extend(decl_lines(&a_decls).iter().map(|ln| format!("    {ln}")));
    out.extend(a_body.iter().map(|ln| format!("    {ln}")));
    out.push("  }".to_string());
    out.push("}".to_string());
    out.join("\n")
}

fn render_overlay(
    ast: &Ast,
    safe: &str,
    name: &str,
    match_expr: Option<NodeId>,
    assert_expr: Option<NodeId>,
) -> String {
    let mut r = RuleRenderer::default();
    let (m_decls, m_body) = r.panel(ast, match_expr, "", false);
    let (a_decls, a_body) = r.panel(ast, assert_expr, "", true);
    let mut merged = m_decls.clone();
    for (k, v) in a_decls {
        decls_setdefault(&mut merged, k, v);
    }
    let mut out = vec![
        format!("digraph rule_{safe}_overlay {{"),
        format!("  label={}; labelloc=t;", quote(name)),
    ];
    out.extend(decl_lines(&merged).iter().map(|ln| format!("  {ln}")));
    out.extend(m_body.iter().map(|ln| format!("  {ln}")));
    out.extend(a_body.iter().map(|ln| format!("  {ln}")));
    out.push("}".to_string());
    out.join("\n")
}

/// A rule library — one digraph per child rule, joined by blank lines.
pub fn render_rules_forms(ast: &Ast, rules: &[NodeId], mode: RuleMode) -> String {
    let chunks: Vec<String> = rules
        .iter()
        .filter(|r| is_sform(ast, **r))
        .map(|r| render_rule_form(ast, *r, mode))
        .collect();
    chunks.join("\n\n")
}
