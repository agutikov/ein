//! The rule → DOT renderer.
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
//! and not data; a stored negation as a red pattern of `not` (below); and a
//! NAF `absent` guard as its own `cluster_absent`, with the guard's
//! binder-local variables declared *inside* it and shared variables left
//! outside — so "no such match exists" survives instead of flattening into
//! overlay arrows.
//!
//! **A pattern is drawn as a fact is** (Q-M1f.419): an octagon with no name
//! in it, its first arrow, numbered `1`, at its relation, a box, or the
//! variable's diamond where the relation is a variable, and an arrow
//! numbered `2`, `3`, … at each argument. Folded (`fold`), a pattern of three
//! arrows that no drawn pattern points at is one arrow from its first
//! argument to its second, labelled with its relation, as a fact is folded.
//! A stored negation `(not P)` is a pattern of `not`, drawn red, whose second
//! arrow points at `P`'s node, as the reasoning pages draw a negation.
//!
//! **A pattern under an arrow of a pattern is its own node** (Q-M1f.395),
//! with its own arrows, so that what a nested pattern binds is drawn. It
//! stands in a dashed frame named for the relations of the patterns that
//! point at it (Q-M1f.398), and in no frame where it is also a premise, or a
//! literal of the conclusion: the pattern's own *held*; no octagon is
//! filled. A pattern that is pointed at is that node and no arrow, in the
//! panel it is pointed at in; the overlay is one drawing, so there a
//! conclusion that the match points at is the node the match draws.

use std::collections::{BTreeSet, HashMap, HashSet};

use ein_core::is_predicate;
use ein_ir::{Ast, Node, NodeId};

use crate::dot_util::{
    GROUND_SHAPE, HYPER_SHAPE, POLYGON_RANKSEP, VAR_SHAPE, ctx_frames, ctx_label, quote,
    value_label,
};
use crate::fact_graph::FACT_NODE;
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

/// The head atom's name, or `None` when the head is a `?var`.
///
/// No name of a theory's is read: a macro's call, `std.macro`'s `forall`
/// and `unknown` among them, is drawn as it is written, a pattern.
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
    head_name(ast, expr) == Some("absent")
}

/// A clause's arguments: every one, since a term holds no keyword pair
/// (`ein.y`'s `Args` and `Terms`; Q-M1f.14), so there is nothing to filter.
fn positional(ast: &Ast, expr: NodeId) -> Vec<NodeId> {
    ast.form_args(expr).to_vec()
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

/// The label of a pattern's relation: an atom's name or a variable's `?v`.
fn head_label(ast: &Ast, clause: NodeId) -> String {
    value_label(ast, form_head(ast, clause))
}

// ── the patterns under an arrow ────────────────────────────────────

/// What one drawing of a rule draws under an arrow: every pattern that a
/// drawn pattern points at, by its text, in the order it is first met, with
/// the relations of the patterns that point at it; and the texts of the
/// drawing's premises and of its conclusion's literals, which are what a
/// pattern has for *held*. `fold` is the drawing's form.
///
/// One drawing is a panel of the side-by-side view, or the whole overlay.
#[derive(Default)]
struct Scene {
    nested: Vec<(NodeId, BTreeSet<String>)>,
    at: HashMap<String, usize>,
    premises: HashSet<String>,
    conclusions: HashSet<String>,
    fold: bool,
}

impl Scene {
    fn is_nested(&self, ast: &Ast, clause: NodeId) -> bool {
        self.at.contains_key(&value_label(ast, clause))
    }

    /// A pattern drawn as one labelled arrow: of three arrows, folded, and
    /// pointed at by no drawn pattern.
    fn folds(&self, ast: &Ast, clause: NodeId) -> bool {
        self.fold && positional(ast, clause).len() == 2 && !self.is_nested(ast, clause)
    }

    /// Everything `form` points at is pointed at by a drawn pattern, `by`
    /// the name its arrows are read under.
    fn walk(&mut self, ast: &Ast, form: NodeId, by: &str) {
        for arg in positional(ast, form) {
            if !is_sform(ast, arg) {
                continue;
            }
            let text = value_label(ast, arg);
            let i = match self.at.get(&text) {
                Some(&i) => i,
                None => {
                    let i = self.nested.len();
                    self.at.insert(text, i);
                    self.nested.push((arg, BTreeSet::new()));
                    let inner = head_label(ast, arg);
                    self.walk(ast, arg, &inner);
                    i
                }
            };
            self.nested[i].1.insert(by.to_string());
        }
    }

    /// One clause of a pattern, read as the drawing reads it. `held` is the
    /// set the clause's own text joins where it is a premise or a literal:
    /// none under an absence. A `(not P)` is a pattern of `not`, which points
    /// at `P`.
    fn clause(&mut self, ast: &Ast, clause: NodeId, conclusion: bool, positive: bool) {
        match head_name(ast, clause) {
            Some("and" | "or") => {
                for child in ast.form_args(clause).to_vec() {
                    if is_sform(ast, child) {
                        self.clause(ast, child, conclusion, positive);
                    }
                }
            }
            Some("absent") => {
                for child in ast.form_args(clause).to_vec() {
                    if is_sform(ast, child) {
                        self.clause(ast, child, conclusion, false);
                    }
                }
            }
            // A guard's arguments are under its arrows too, and a fact it
            // compares is drawn as its node; the guard itself is no pattern.
            Some(hn) if is_predicate(hn) => {
                let by = head_label(ast, clause);
                self.walk(ast, clause, &by);
            }
            _ => {
                if positive {
                    let text = value_label(ast, clause);
                    if conclusion {
                        self.conclusions.insert(text);
                    } else {
                        self.premises.insert(text);
                    }
                }
                let by = head_label(ast, clause);
                self.walk(ast, clause, &by);
            }
        }
    }

    /// The nodes of what stands under an arrow, and of everything under the
    /// arrows of a pattern that stands there, at any depth, in the order
    /// met: a nested pattern is a node, and so are its relation and each
    /// variable and name it points at.
    fn value_nodes(&self, ast: &Ast, arg: NodeId, out: &mut Vec<String>) {
        out.push(value_label(ast, arg));
        if is_sform(ast, arg) {
            out.push(head_label(ast, arg));
            for inner in positional(ast, arg) {
                self.value_nodes(ast, inner, out);
            }
        }
    }

    /// The labels of every node a clause tree draws: a pattern's relation,
    /// where it is drawn as its octagon, and its arguments. The walk recurses
    /// through `and` / `or` / `absent`, so a guard's inner nodes
    /// surface; a predicate's two ends are nodes and its name is not.
    fn arg_nodes(&self, ast: &Ast, expr: NodeId) -> Vec<String> {
        let mut out = Vec::new();
        match head_name(ast, expr) {
            Some("and" | "or" | "absent") => {
                for child in ast.form_args(expr).to_vec() {
                    if is_sform(ast, child) {
                        out.extend(self.arg_nodes(ast, child));
                    }
                }
            }
            Some(hn) if is_predicate(hn) => {
                for a in positional(ast, expr) {
                    self.value_nodes(ast, a, &mut out);
                }
            }
            _ => {
                if !self.folds(ast, expr) {
                    out.push(head_label(ast, expr));
                }
                for a in positional(ast, expr) {
                    self.value_nodes(ast, a, &mut out);
                }
            }
        }
        out
    }

    /// First-seen order of node labels across `clauses`.
    fn ordered_nodes(&self, ast: &Ast, clauses: &[NodeId]) -> Vec<String> {
        let mut seen: Vec<String> = Vec::new();
        for c in clauses {
            for n in self.arg_nodes(ast, *c) {
                if !seen.contains(&n) {
                    seen.push(n);
                }
            }
        }
        seen
    }

    /// A node lives in a guard cluster iff it appears in exactly one guard
    /// and nowhere in the top-level clauses — that makes it the guard's
    /// binder-local node. Everything shared stays at the top so its edges
    /// cross into the cluster rather than pulling the node inside.
    fn node_homes(&self, ast: &Ast, top: &[NodeId], guards: &[NodeId]) -> Vec<(String, Home)> {
        let top_nodes = self.ordered_nodes(ast, top);
        let guard_nodes: Vec<Vec<String>> =
            guards.iter().map(|g| self.arg_nodes(ast, *g)).collect();
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
}

/// The scene of one drawing: `matched` and `asserted` are the patterns it
/// draws, either of which a panel leaves out.
fn scene(ast: &Ast, matched: Option<NodeId>, asserted: Option<NodeId>, fold: bool) -> Scene {
    let mut s = Scene {
        fold,
        ..Scene::default()
    };
    for (pattern, conclusion) in [(matched, false), (asserted, true)] {
        let Some(pattern) = pattern else { continue };
        for clause in ast.conjuncts(pattern) {
            if is_sform(ast, clause) && head_name(ast, clause) != Some("#empty") {
                s.clause(ast, clause, conclusion, true);
            }
        }
    }
    s
}

/// Where a node lives: at panel scope, or inside guard `n`'s cluster.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Home {
    Top,
    Guard(usize),
}

fn home_of(homes: &[(String, Home)], label: &str) -> Option<Home> {
    homes.iter().find(|(n, _)| n == label).map(|(_, h)| *h)
}

// ── shapes / ids ───────────────────────────────────────────────────

fn shape_attrs(nodelabel: &str) -> String {
    // A variable is a diamond, anything else a box; `_` is refused at the
    // parse, so no label is a wildcard's.
    if nodelabel.starts_with('?') {
        format!("shape={VAR_SHAPE}")
    } else {
        format!("shape={GROUND_SHAPE}")
    }
}

/// Quoted DOT id: the clean label plus a per-panel disambiguating suffix.
fn nid(nodelabel: &str, suffix: &str) -> String {
    quote(&format!("{nodelabel}{suffix}"))
}

/// The colour of a pattern of the relation labelled `head`: red for a
/// stored negation's, and by the relation's name for any other.
fn pattern_colour(head: &str, negative: bool) -> &'static str {
    if negative || head == "not" {
        NEG_COLOUR
    } else {
        hash_color(head.trim_start_matches('?'))
    }
}

// ── the renderer ───────────────────────────────────────────────────

/// Accumulates one rule's DOT, panel by panel. The hyperedge counter is
/// per-*rule*, not per-panel, so an n-ary clause on the right keeps counting
/// from where the left panel stopped.
#[derive(Default)]
struct RuleRenderer {
    hcount: u32,
    /// The ids of the nested patterns already drawn: the overlay draws its
    /// two patterns on one graph, and a pattern both hold is drawn once.
    drawn: HashSet<String>,
}

/// A panel's node declarations, in first-declaration order.
type Decls = Vec<(String, String)>;

/// The node of a pattern under an arrow, as [`RuleRenderer::nested_lines`]
/// draws it.
struct Nested {
    /// The label of its frame, `None` where the pattern is held in the
    /// drawing: a premise, or a literal of the conclusion.
    frame: Option<String>,
    id: String,
    attrs: String,
    arrows: Vec<String>,
}

fn decls_setdefault(decls: &mut Decls, key: String, value: String) {
    if !decls.iter().any(|(k, _)| *k == key) {
        decls.push((key, value));
    }
}

/// A pattern's numbered arrows from its octagon `from`: the first at its
/// relation, the next at its arguments.
fn pattern_arrows(
    ast: &Ast,
    clause: NodeId,
    from: &str,
    suffix: &str,
    colour: &str,
    edge_style: &str,
) -> Vec<String> {
    let mut targets = vec![head_label(ast, clause)];
    targets.extend(positional(ast, clause).iter().map(|a| value_label(ast, *a)));
    targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "{from} -> {} [xlabel=\"{}\", color=\"{colour}\"{edge_style}];",
                nid(t, suffix),
                i + 1
            )
        })
        .collect()
}

impl RuleRenderer {
    fn fresh_hyper(&mut self, label: &str, suffix: &str) -> String {
        self.hcount += 1;
        quote(&format!("h{}_{label}{suffix}", self.hcount))
    }

    fn clause_lines(
        &mut self,
        ast: &Ast,
        scene: &Scene,
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
                        out.extend(self.clause_lines(ast, scene, child, suffix, dashed, negative));
                    }
                }
                out
            }
            Some(hn) if is_predicate(hn) => vec![self.constraint_line(ast, clause, suffix)],
            // An `absent` reached here is nested inside another clause —
            // render its body inline as forbidden. The top-level case is a
            // cluster, handled by `panel`.
            Some("absent") => {
                let mut out = Vec::new();
                for child in ast.form_args(clause).to_vec() {
                    if is_sform(ast, child) {
                        out.extend(self.clause_lines(ast, scene, child, suffix, dashed, true));
                    }
                }
                out
            }
            // A pattern that a drawn pattern points at is its node, drawn
            // with the scene's, and no arrow.
            _ if scene.is_nested(ast, clause) => Vec::new(),
            _ => self.relation_lines(ast, scene, clause, suffix, dashed, negative),
        }
    }

    /// The node of one pattern under an arrow, as its frame, none where the
    /// pattern is held in the drawing, its id and its attributes, and its
    /// numbered arrows; nothing where the drawing has it already.
    fn nested_lines(
        &mut self,
        ast: &Ast,
        scene: &Scene,
        label: &str,
        suffix: &str,
        dashed: bool,
    ) -> Option<Nested> {
        let &i = scene.at.get(label)?;
        let (pattern, by) = &scene.nested[i];
        let id = nid(label, suffix);
        if !self.drawn.insert(id.clone()) {
            return None;
        }
        let colour = pattern_colour(&head_label(ast, *pattern), false);
        // In no frame where the pattern is a premise, nor where it is a
        // literal of the conclusion, dashed there where it is no premise, as
        // what the conclusion adds; in the frame of what points at it where
        // it is neither, in a context only.
        let (frame, style) = if scene.premises.contains(label) {
            (None, "")
        } else if scene.conclusions.contains(label) {
            (None, ", style=dashed")
        } else {
            (Some(ctx_label(by)), "")
        };
        let attrs = format!("shape={HYPER_SHAPE}, {FACT_NODE}, color=\"{colour}\"{style}");
        let edge_style = if dashed { ", style=dashed" } else { "" };
        let arrows = pattern_arrows(ast, *pattern, &id, suffix, colour, edge_style);
        Some(Nested {
            frame,
            id,
            attrs,
            arrows,
        })
    }

    fn relation_lines(
        &mut self,
        ast: &Ast,
        scene: &Scene,
        clause: NodeId,
        suffix: &str,
        dashed: bool,
        negative: bool,
    ) -> Vec<String> {
        let head = head_label(ast, clause);
        let pos = positional(ast, clause);
        let colour = pattern_colour(&head, negative);
        let edge_style = if dashed { ", style=dashed" } else { "" };
        if scene.folds(ast, clause) {
            return vec![format!(
                "{} -> {} [label={}, color=\"{colour}\", fontcolor=\"{colour}\"{edge_style}];",
                nid(&value_label(ast, pos[0]), suffix),
                nid(&value_label(ast, pos[1]), suffix),
                quote(&head)
            )];
        }
        // The canonical form: an octagon, its first arrow at the relation.
        let h = self.fresh_hyper(head.trim_start_matches('?'), suffix);
        let mut lines = vec![format!(
            "{h} [shape={HYPER_SHAPE}, {FACT_NODE}, color=\"{colour}\"{edge_style}];"
        )];
        lines.extend(pattern_arrows(ast, clause, &h, suffix, colour, edge_style));
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

    /// One pattern → (node-decl map, ordered body lines, framed nodes).
    ///
    /// Top-home nodes come back in the decl map, for the caller to emit at
    /// panel scope; guard-local nodes are declared inside their
    /// `cluster_absent` block within the body lines. A pattern under an
    /// arrow in a context only comes back as a framed node, `(frame,
    /// declaration)`, for the caller to write its frames, and one held in the
    /// drawing as a top-home node; one that is an absence's own is declared,
    /// framed or not, inside the absence's block.
    fn panel(
        &mut self,
        ast: &Ast,
        scene: &Scene,
        pattern: Option<NodeId>,
        suffix: &str,
        dashed: bool,
    ) -> (Decls, Vec<String>, Vec<(String, String)>) {
        let mut decls: Decls = Vec::new();
        let mut body: Vec<String> = Vec::new();
        let mut framed: Vec<(String, String)> = Vec::new();
        let Some(pattern) = pattern else {
            return (decls, body, framed);
        };
        // The clause's conjuncts, an `(and …)` read flat at any depth
        // (`ein_ir::Ast::conjuncts`): the compiler's reading of a premise and
        // of a conclusion alike.
        // `()`, the empty pattern, is true and draws nothing.
        let clauses: Vec<NodeId> = ast
            .conjuncts(pattern)
            .into_iter()
            .filter(|c| is_sform(ast, *c) && head_name(ast, *c) != Some("#empty"))
            .collect();
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
        let homes = scene.node_homes(ast, &top, &guards);

        // Panel-scope node declarations (shared / top-home nodes), and the
        // arrows of the patterns under an arrow among them, after the
        // clauses'.
        let mut nested_arrows: Vec<String> = Vec::new();
        for nl in scene.ordered_nodes(ast, &clauses) {
            if home_of(&homes, &nl) != Some(Home::Top) {
                continue;
            }
            if scene.at.contains_key(&nl) {
                if let Some(n) = self.nested_lines(ast, scene, &nl, suffix, dashed) {
                    match n.frame {
                        Some(frame) => framed.push((frame, format!("{} [{}];", n.id, n.attrs))),
                        None => decls_setdefault(&mut decls, n.id, n.attrs),
                    }
                    nested_arrows.extend(n.arrows);
                }
                continue;
            }
            decls_setdefault(
                &mut decls,
                nid(&nl, suffix),
                format!("label={}, {}", quote(&nl), shape_attrs(&nl)),
            );
        }

        for c in &top {
            body.extend(self.clause_lines(ast, scene, *c, suffix, dashed, false));
        }
        body.extend(nested_arrows);

        // Each guard → its own cluster (local decls + inner lines).
        for (gi, guard) in guards.iter().enumerate() {
            let kind = "absent";
            let scope = suffix.trim_matches('_');
            let scope = if scope.is_empty() { "o" } else { scope };
            let cid = format!("cluster_{kind}_{scope}_{gi}");
            let glyph = "∄";
            let mut block = vec![
                format!("subgraph {cid} {{"),
                format!(
                    "  label=\"{kind} ({glyph})\"; style=\"dashed,rounded\"; \
                     color=\"{GUARD_COLOUR}\"; fontcolor=\"{GUARD_COLOUR}\";"
                ),
            ];
            let mut local_framed: Vec<(String, String)> = Vec::new();
            let mut local_arrows: Vec<String> = Vec::new();
            for nl in scene.ordered_nodes(ast, &[*guard]) {
                if home_of(&homes, &nl) != Some(Home::Guard(gi)) {
                    continue;
                }
                if scene.at.contains_key(&nl) {
                    if let Some(n) = self.nested_lines(ast, scene, &nl, suffix, dashed) {
                        let decl = format!("{} [{}];", n.id, n.attrs);
                        match n.frame {
                            Some(frame) => local_framed.push((frame, decl)),
                            None => block.push(format!("  {decl}")),
                        }
                        local_arrows.extend(n.arrows);
                    }
                    continue;
                }
                block.push(format!(
                    "  {} [label={}, {}];",
                    nid(&nl, suffix),
                    quote(&nl),
                    shape_attrs(&nl)
                ));
            }
            block.extend(ctx_frames(
                &format!("{kind}_{scope}_{gi}_"),
                "  ",
                &local_framed,
            ));
            for child in ast.form_args(*guard).to_vec() {
                if is_sform(ast, child) {
                    for ln in self.clause_lines(ast, scene, child, suffix, dashed, false) {
                        block.push(format!("  {ln}"));
                    }
                }
            }
            block.extend(local_arrows.iter().map(|ln| format!("  {ln}")));
            block.push("}".to_string());
            body.extend(block);
        }
        (decls, body, framed)
    }
}

// ── field extraction ───────────────────────────────────────────────

/// `(name, match-expr, assert-expr)` from a `(rule …)` form; a `(macro …)`'s
/// body is its match, and it concludes nothing.
fn extract(ast: &Ast, rule: NodeId) -> (String, Option<NodeId>, Option<NodeId>) {
    let mut name = "anon".to_string();
    let mut match_expr = None;
    let mut assert_expr = None;
    let is_macro = ast.head_name(rule) == Some("macro");
    let mut positional_at = 0;
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
            // `(macro <name> (<params>) <body> …)`: the body is the third
            // argument.
            Node::SForm { .. } if is_macro => {
                positional_at += 1;
                if positional_at == 2 {
                    match_expr = Some(arg);
                }
            }
            _ => {}
        }
    }
    (name, match_expr, assert_expr)
}

// ── public entry points ────────────────────────────────────────────

/// One `(rule …)`, `(hrule …)` or `(macro …)` form as a DOT digraph, every
/// pattern in the canonical form, or folded where `fold`.
pub fn render_rule_form(ast: &Ast, rule: NodeId, mode: RuleMode, fold: bool) -> String {
    let (name, match_expr, assert_expr) = extract(ast, rule);
    let safe = name.replace(['-', ' ', '.'], "_");
    if ast.head_name(rule) == Some("macro") {
        return render_macro(ast, &safe, &name, match_expr, fold);
    }
    match mode {
        RuleMode::SideBySide => render_sidebyside(ast, &safe, &name, match_expr, assert_expr, fold),
        RuleMode::Overlay => render_overlay(ast, &safe, &name, match_expr, assert_expr, fold),
    }
}

fn decl_lines(decls: &Decls) -> Vec<String> {
    decls
        .iter()
        .map(|(nid, attrs)| format!("{nid} [{attrs}];"))
        .collect()
}

/// One pattern drawn as a panel draws it, inside `subgraph cluster_<id>`
/// labelled `label`, each line indented by `indent`.
fn cluster(
    r: &mut RuleRenderer,
    ast: &Ast,
    pattern: Option<NodeId>,
    asserted: bool,
    fold: bool,
    (id, label, suffix): (&str, &str, &str),
    indent: &str,
) -> Vec<String> {
    let scene = if asserted {
        scene(ast, None, pattern, fold)
    } else {
        scene(ast, pattern, None, fold)
    };
    let (decls, body, framed) = r.panel(ast, &scene, pattern, suffix, asserted);
    let inner = format!("{indent}  ");
    let mut out = vec![format!(
        "{indent}subgraph cluster_{id} {{ label={};",
        quote(label)
    )];
    out.extend(decl_lines(&decls).iter().map(|ln| format!("{inner}{ln}")));
    out.extend(ctx_frames(
        &format!("{}_", suffix.trim_matches('_')),
        &inner,
        &framed,
    ));
    out.extend(body.iter().map(|ln| format!("{inner}{ln}")));
    out.push(format!("{indent}}}"));
    out
}

fn render_sidebyside(
    ast: &Ast,
    safe: &str,
    name: &str,
    match_expr: Option<NodeId>,
    assert_expr: Option<NodeId>,
    fold: bool,
) -> String {
    let mut r = RuleRenderer::default();
    // Each panel is a drawing of its own: what a pattern points at is read
    // in the panel it is drawn in.
    let mut out = vec![
        format!("digraph rule_{safe}_lhs_rhs {{"),
        "  rankdir=TB;".to_string(),
        format!("  label={}; labelloc=t;", quote(name)),
    ];
    out.extend(cluster(
        &mut r,
        ast,
        match_expr,
        false,
        fold,
        ("lhs", "match", "_L"),
        "  ",
    ));
    out.extend(cluster(
        &mut r,
        ast,
        assert_expr,
        true,
        fold,
        ("rhs", "assert", "_R"),
        "  ",
    ));
    out.push("}".to_string());
    out.join("\n")
}

/// A macro: its body, drawn as a rule's match is, in one panel.
fn render_macro(ast: &Ast, safe: &str, name: &str, body: Option<NodeId>, fold: bool) -> String {
    let mut r = RuleRenderer::default();
    let mut out = vec![
        format!("digraph macro_{safe} {{"),
        "  rankdir=TB;".to_string(),
        format!("  label={}; labelloc=t;", quote(&format!("macro {name}"))),
    ];
    out.extend(cluster(
        &mut r,
        ast,
        body,
        false,
        fold,
        ("body", "body", "_B"),
        "  ",
    ));
    out.push("}".to_string());
    out.join("\n")
}

/// A query's `:goal`, drawn as a rule's match is, in a cluster of the query
/// digraph: `n` numbers the query among the program's.
pub fn goal_cluster(ast: &Ast, goal: NodeId, n: usize, fold: bool) -> Vec<String> {
    let mut r = RuleRenderer::default();
    let id = format!("goal_{n}");
    let suffix = format!("_Q{n}");
    cluster(
        &mut r,
        ast,
        Some(goal),
        false,
        fold,
        (&id, "query", &suffix),
        "  ",
    )
}

fn render_overlay(
    ast: &Ast,
    safe: &str,
    name: &str,
    match_expr: Option<NodeId>,
    assert_expr: Option<NodeId>,
    fold: bool,
) -> String {
    let mut r = RuleRenderer::default();
    // The overlay is one drawing of both patterns: a conclusion that the
    // match points at is the node the match draws.
    let both = scene(ast, match_expr, assert_expr, fold);
    let (m_decls, m_body, mut framed) = r.panel(ast, &both, match_expr, "", false);
    let (a_decls, a_body, a_framed) = r.panel(ast, &both, assert_expr, "", true);
    framed.extend(a_framed);
    let mut merged = m_decls.clone();
    for (k, v) in a_decls {
        decls_setdefault(&mut merged, k, v);
    }
    // No cluster holds the diamonds and octagons here, unlike the side-by-side
    // view's, so the overlay sets the rank separation
    // (`dot_util::POLYGON_RANKSEP`).
    let mut out = vec![
        format!("digraph rule_{safe}_overlay {{"),
        format!("  label={}; labelloc=t;", quote(name)),
        format!("  {POLYGON_RANKSEP};"),
    ];
    out.extend(decl_lines(&merged).iter().map(|ln| format!("  {ln}")));
    out.extend(ctx_frames("o_", "  ", &framed));
    out.extend(m_body.iter().map(|ln| format!("  {ln}")));
    out.extend(a_body.iter().map(|ln| format!("  {ln}")));
    out.push("}".to_string());
    out.join("\n")
}

/// A rule library — one digraph per child rule, joined by blank lines.
pub fn render_rules_forms(ast: &Ast, rules: &[NodeId], mode: RuleMode, fold: bool) -> String {
    let chunks: Vec<String> = rules
        .iter()
        .filter(|r| is_sform(ast, **r))
        .map(|r| render_rule_form(ast, *r, mode, fold))
        .collect();
    chunks.join("\n\n")
}
