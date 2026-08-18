//! The per-form IR → DOT renderer — `ein.py`'s `ir/to_dot.py`.
//!
//! Implements `docs/kernel/ir/03-ein-lang/04_dot_rendering.md`. The default
//! view is **compact**: a binary fact `(rel a b)` collapses to one labelled,
//! relation-coloured arrow, and a unary fact to a labelled self-loop (the
//! predicate-as-subset idiom, S1.22.4). `levi = true` keeps the canonical
//! Levi-bipartite octagon for both; every other arity renders bipartite in
//! either mode, because DOT has no native hyperedge.
//!
//! **Where this port departs in shape, not in bytes.** ein.py's `to_dot`
//! re-groups a flat program by synthesising wrapper `SForm`s — `SForm(head=
//! Atom("facts"), args=…)` — and recursing. Synthesising a node here would
//! mean a `&mut Ast` in a renderer, so the wrappers are *slices* instead:
//! [`render_facts_forms`] and friends take the children directly and the
//! public per-form entry points unwrap their argument into one. The emitted
//! bytes are the wrapper's, because the wrapper only ever contributed the
//! digraph name.

use ein_ir::{Ast, Node, NodeId};

use crate::builder::Builder;
use crate::dot_util::{
    EQUALITY_SHAPE, GROUND_SHAPE, INSTANCE_SHAPE, TYPE_SHAPE, VAR_SHAPE, WILDCARD_ATTRS, quote,
    value_label,
};
use crate::palette::hash_color;
use crate::rules::{RuleMode, render_rule_form, render_rules_forms};

/// The rendering options `to_dot` threads through every form.
#[derive(Clone, Copy)]
pub struct DotOpts {
    pub rule_mode: RuleMode,
    pub trace_view: TraceView,
    pub levi: bool,
}

impl Default for DotOpts {
    fn default() -> Self {
        DotOpts {
            rule_mode: RuleMode::SideBySide,
            trace_view: TraceView::PerStep,
            levi: false,
        }
    }
}

/// `render_trace`'s view, with ein.py's friendly names alongside the legacy
/// letters (`a`/`per-step`, `b`/`aggregate`, `c`/`dag`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraceView {
    PerStep,
    Aggregate,
    Dag,
}

impl TraceView {
    pub fn parse(s: &str) -> Option<TraceView> {
        match s {
            "a" | "per-step" => Some(TraceView::PerStep),
            "b" | "aggregate" => Some(TraceView::Aggregate),
            "c" | "dag" => Some(TraceView::Dag),
            _ => None,
        }
    }
}

// ── node ids and shapes ────────────────────────────────────────────

/// A DOT-safe quoted identifier for an atom-like node.
fn atom_id(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Var(s) => quote(&format!("?{}", ast.sym(s))),
        Node::Wildcard => quote("_"),
        Node::Atom(s) => quote(ast.sym(s)),
        // ein.py reads `node.name` unconditionally on the fall-through, so
        // anything else is an `AttributeError` there and unreachable here.
        _ => panic!("not an atom-like node"),
    }
}

fn is_atom_like(ast: &Ast, id: NodeId) -> bool {
    matches!(ast.node(id), Node::Atom(_) | Node::Var(_) | Node::Wildcard)
}

/// Shape attrs for an atom-like arg appearing in a fact or pattern.
fn atom_arg_attrs(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Var(_) => format!("shape={VAR_SHAPE}"),
        Node::Wildcard => WILDCARD_ATTRS.to_string(),
        _ => format!("shape={GROUND_SHAPE}"),
    }
}

/// Resolve an atom-like or `SForm` value to a DOT node id.
fn atom_id_for_value(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Atom(_) | Node::Var(_) | Node::Wildcard => atom_id(ast, id),
        Node::Str(s) => quote(ast.sym(s)),
        Node::Int(s) => quote(ast.sym(s)),
        Node::Range { .. } | Node::SForm { .. } => quote(&value_label(ast, id)),
        Node::Keyword(_) | Node::KwPair { .. } => {
            panic!("cannot use as DOT node id")
        }
    }
}

fn emit_atom(b: &mut Builder, ast: &Ast, id: NodeId) {
    if is_atom_like(ast, id) {
        let attrs = atom_arg_attrs(ast, id);
        b.node(&atom_id(ast, id), Some(&attrs));
    }
}

// ── form-shape helpers ─────────────────────────────────────────────

fn positional(ast: &Ast, form: NodeId) -> Vec<NodeId> {
    ast.form_args(form)
        .iter()
        .copied()
        .filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
        .collect()
}

fn is_sform(ast: &Ast, id: NodeId) -> bool {
    matches!(ast.node(id), Node::SForm { .. })
}

/// The keyword names on a form, for [`render_group`].
fn kw_names(ast: &Ast, form: NodeId) -> Vec<&str> {
    ast.form_args(form)
        .iter()
        .filter_map(|a| match ast.node(*a) {
            Node::KwPair { key, .. } => match ast.node(key) {
                Node::Keyword(s) => Some(ast.sym(s)),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// The `{keyword: value}` map of a form, last-wins on a repeated key.
fn kw_map(ast: &Ast, form: NodeId) -> Vec<(&str, NodeId)> {
    let mut out: Vec<(&str, NodeId)> = Vec::new();
    for a in ast.form_args(form) {
        if let Node::KwPair { key, value } = ast.node(*a)
            && let Node::Keyword(s) = ast.node(key)
        {
            let name = ast.sym(s);
            match out.iter_mut().find(|(k, _)| *k == name) {
                Some(slot) => slot.1 = value,
                None => out.push((name, value)),
            }
        }
    }
    out
}

fn kw_get(ast: &Ast, form: NodeId, key: &str) -> Option<NodeId> {
    kw_map(ast, form)
        .into_iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v)
}

/// The first `Atom` argument's name — `SForm.leading_symbol`.
fn leading_symbol(ast: &Ast, form: NodeId) -> Option<&str> {
    ast.form_args(form).iter().find_map(|a| match ast.node(*a) {
        Node::Atom(s) => Some(ast.sym(s)),
        _ => None,
    })
}

// ── one fact ───────────────────────────────────────────────────────

fn emit_fact(b: &mut Builder, ast: &Ast, fact: NodeId, derived: bool, levi: bool) {
    let head = ast.head_name(fact).unwrap_or("");
    let pos = positional(ast, fact);

    // Equality fact → double-circle equality class.
    if head == "=" && pos.len() == 2 {
        let (a, c) = (pos[0], pos[1]);
        let eq_id = quote(&format!(
            "eq_{}_{}",
            value_label(ast, a),
            value_label(ast, c)
        ));
        b.node(
            &eq_id,
            Some(&format!("shape={EQUALITY_SHAPE}, label=\"=\"")),
        );
        emit_atom(b, ast, a);
        emit_atom(b, ast, c);
        let (ida, idc) = (atom_id_for_value(ast, a), atom_id_for_value(ast, c));
        b.edge(&eq_id, &ida, None);
        b.edge(&eq_id, &idc, None);
        return;
    }

    // Membership fact → dashed is-a edge (UML-style). Presentation knowledge,
    // not kernel reasoning: `is-a` is an ordinary relation the renderer
    // happens to know how to draw.
    if head == "is-a" && pos.len() == 2 {
        let (ent, typ) = (pos[0], pos[1]);
        if is_atom_like(ast, ent) {
            b.node(&atom_id(ast, ent), Some(&format!("shape={INSTANCE_SHAPE}")));
        }
        if is_atom_like(ast, typ) {
            b.node(&atom_id(ast, typ), Some(&format!("shape={TYPE_SHAPE}")));
        }
        let (ide, idt) = (atom_id_for_value(ast, ent), atom_id_for_value(ast, typ));
        b.edge(
            &ide,
            &idt,
            Some("style=dashed, arrowhead=empty, label=\"is-a\""),
        );
        return;
    }

    // Negative fact → recurse into the wrapped expression, mark dashed.
    if head == "not" && pos.len() == 1 && is_sform(ast, pos[0]) {
        emit_fact(b, ast, pos[0], true, levi);
        return;
    }

    // Compact: a unary relation is a relation-coloured self-loop on its
    // single argument — the degenerate case of the binary collapse, with
    // source == target. `not` is excluded; its own encoding is above.
    if !levi && pos.len() == 1 && head != "not" && is_atom_like(ast, pos[0]) {
        let a = pos[0];
        emit_atom(b, ast, a);
        let colour = hash_color(head);
        let style = if derived { "dashed" } else { "solid" };
        let id = atom_id_for_value(ast, a);
        b.edge(
            &id,
            &id,
            Some(&format!(
                "label=\"{head}\", color=\"{colour}\", fontcolor=\"{colour}\", style={style}"
            )),
        );
        return;
    }

    // Compact: a binary relation is one relation-coloured arrow.
    if !levi && pos.len() == 2 {
        let (a, c) = (pos[0], pos[1]);
        emit_atom(b, ast, a);
        emit_atom(b, ast, c);
        let colour = hash_color(head);
        let style = if derived { "dashed" } else { "solid" };
        let (ida, idc) = (atom_id_for_value(ast, a), atom_id_for_value(ast, c));
        b.edge(
            &ida,
            &idc,
            Some(&format!(
                "label=\"{head}\", color=\"{colour}\", fontcolor=\"{colour}\", style={style}"
            )),
        );
        return;
    }

    // Levi-bipartite octagon: every remaining arity, plus binary under
    // `levi`. Provenance is implicit on the hyperedge node, not drawn.
    let style = if derived { ", style=dashed" } else { "" };
    let h_id = b.fresh_h(head);
    for (i, arg) in pos.iter().enumerate() {
        emit_atom(b, ast, *arg);
        let id = atom_id_for_value(ast, *arg);
        b.edge(&h_id, &id, Some(&format!("label=\"{}\"{style}", i + 1)));
    }
}

// ── the form renderers ─────────────────────────────────────────────

/// `(ontology …)` — the UML-ish type / instance / relation schema.
///
/// Type and relation declarations render identically in both modes (they are
/// schema, already direct labelled edges); only implicit facts honour `levi`.
pub fn render_ontology_forms(ast: &Ast, decls: &[NodeId], levi: bool) -> String {
    let mut b = Builder::new("ontology");
    for decl in decls {
        if !is_sform(ast, *decl) {
            continue;
        }
        let head = ast.head_name(*decl).unwrap_or("");
        if head == "relation" {
            // `(relation Name T1 T2 … [kw…])` — flat args post-R10.
            let args = ast.form_args(*decl).to_vec();
            let Some(&first) = args.first() else { continue };
            if !matches!(ast.node(first), Node::Atom(_) | Node::Var(_)) {
                continue;
            }
            let rel_name = value_label(ast, first);
            let sig: Vec<NodeId> = args[1..]
                .iter()
                .copied()
                .filter(|a| is_atom_like(ast, *a))
                .collect();
            if sig.len() >= 2 {
                let (src, dst) = (sig[0], sig[1]);
                b.node(&atom_id(ast, src), Some(&format!("shape={TYPE_SHAPE}")));
                b.node(&atom_id(ast, dst), Some(&format!("shape={TYPE_SHAPE}")));
                let (ids, idd) = (atom_id_for_value(ast, src), atom_id_for_value(ast, dst));
                b.edge(
                    &ids,
                    &idd,
                    Some(&format!("label=\"{rel_name}\", style=dashed")),
                );
            }
        } else {
            emit_fact(&mut b, ast, *decl, false, levi);
        }
    }
    b.build()
}

/// `(facts …)` / `(reasoning …)` — the latter with `derived`, which dashes
/// every edge.
pub fn render_facts_forms(ast: &Ast, facts: &[NodeId], derived: bool, levi: bool) -> String {
    let name = if derived { "reasoning" } else { "facts" };
    let mut b = Builder::new(name);
    for fact in facts {
        if is_sform(ast, *fact) {
            emit_fact(&mut b, ast, *fact, derived, levi);
        }
    }
    b.build()
}

/// `(query …)` — the keyword args as one compact note node.
pub fn render_query(ast: &Ast, form: NodeId) -> String {
    let mut b = Builder::new("query");
    let mut parts: Vec<String> = Vec::new();
    for a in ast.form_args(form) {
        if let Node::KwPair { key, value } = ast.node(*a)
            && let Node::Keyword(s) = ast.node(key)
        {
            parts.push(format!(":{} {}", ast.sym(s), value_label(ast, value)));
        }
    }
    let label = if parts.is_empty() {
        "query".to_string()
    } else {
        parts.join("\\n")
    };
    b.node(
        &quote("query"),
        Some(&format!("shape=note, label=\"{label}\"")),
    );
    b.build()
}

/// Premise refs in a `:using` clause.
///
/// `(and c10 c15)` → `[c10, c15]` (the `and` head is the combinator);
/// `(c10)` → `[c10]` (the head *is* the premise).
fn trace_premises(ast: &Ast, using: NodeId) -> Vec<NodeId> {
    let Node::SForm { head, args } = ast.node(using) else {
        return Vec::new();
    };
    let head_is_and = matches!(ast.node(head), Node::Atom(s) if ast.sym(s) == "and");
    let mut out: Vec<NodeId> = Vec::new();
    if !head_is_and && matches!(ast.node(head), Node::Atom(_)) {
        out.push(head);
    }
    out.extend(
        ast.args(args)
            .iter()
            .copied()
            .filter(|a| matches!(ast.node(*a), Node::Atom(_))),
    );
    out
}

/// `(trace …)` — the step graph, or the derivation DAG under `view = dag`.
pub fn render_trace(ast: &Ast, form: NodeId, view: TraceView) -> String {
    if view == TraceView::Dag {
        return render_trace_dag(ast, form);
    }
    let mut b = Builder::new("trace");
    for ev in ast.form_args(form).to_vec() {
        if !is_sform(ast, ev) {
            continue;
        }
        let kind = ast.head_name(ev).unwrap_or("").to_string();
        let Some(step_name) = leading_symbol(ast, ev).map(str::to_string) else {
            continue;
        };
        let step_id = quote(&step_name);
        let shape = if kind == "step" { "box" } else { "ellipse" };
        b.node(
            &step_id,
            Some(&format!("shape={shape}, label=\"{kind}: {step_name}\"")),
        );
        if let Some(using) = kw_get(ast, ev, "using")
            && is_sform(ast, using)
        {
            for premise in trace_premises(ast, using) {
                let pid = atom_id(ast, premise);
                b.node(&pid, Some("shape=rectangle"));
                b.edge(&pid, &step_id, Some("style=dashed"));
            }
        }
    }
    b.build()
}

/// View (c) — derived-fact nodes linked back to their `:using` premises.
fn render_trace_dag(ast: &Ast, form: NodeId) -> String {
    let mut b = Builder::new("trace");
    // step name → its derived-fact node id, in first-seen order. Lookups
    // only; the emission order is the `for ev` loop's.
    let mut derived_by_step: Vec<(String, String)> = Vec::new();
    for ev in ast.form_args(form).to_vec() {
        if !is_sform(ast, ev) {
            continue;
        }
        let Some(step_name) = leading_symbol(ast, ev).map(str::to_string) else {
            continue;
        };
        let rule = kw_get(ast, ev, "rule").map(|v| value_label(ast, v));
        let derives = kw_get(ast, ev, "derives").filter(|v| is_sform(ast, *v));
        let using = kw_get(ast, ev, "using").filter(|v| is_sform(ast, *v));

        let dnode = match derives {
            Some(d) => {
                let dlabel = value_label(ast, d);
                let dnode = quote(&dlabel);
                b.node(
                    &dnode,
                    Some(&format!("shape=box, style=bold, label={}", quote(&dlabel))),
                );
                dnode
            }
            None => {
                let dnode = quote(&step_name);
                b.node(
                    &dnode,
                    Some(&format!("shape=box, label={}", quote(&step_name))),
                );
                dnode
            }
        };
        match derived_by_step.iter_mut().find(|(k, _)| *k == step_name) {
            Some(slot) => slot.1 = dnode.clone(),
            None => derived_by_step.push((step_name.clone(), dnode.clone())),
        }
        let edge_attrs = rule.map(|r| format!("label=\"{r}\""));
        if let Some(using) = using {
            for premise in trace_premises(ast, using) {
                let pname = match ast.node(premise) {
                    Node::Atom(s) => ast.sym(s).to_string(),
                    _ => continue,
                };
                let known = derived_by_step
                    .iter()
                    .find(|(k, _)| *k == pname)
                    .map(|(_, v)| v.clone());
                let pid = match &known {
                    Some(v) => v.clone(),
                    None => {
                        let id = atom_id(ast, premise);
                        b.node(&id, Some("shape=rectangle"));
                        id
                    }
                };
                b.edge(&pid, &dnode, edge_attrs.as_deref());
            }
        }
    }
    b.build()
}

// ── top-level dispatch ─────────────────────────────────────────────

/// Which DOT group a flat fact form belongs to, from its provenance
/// kw-pairs: `:rule` / `:using` → reasoning, `:source` → fact, else ontology.
///
/// S1.22.1b: render-bucket names, not knowledge layers — the `Layer` enum is
/// gone. The grouping survives only because three sub-graphs read better than
/// one.
fn render_group(ast: &Ast, form: NodeId) -> &'static str {
    let kw = kw_names(ast, form);
    if kw.contains(&"rule") || kw.contains(&"using") {
        "reasoning"
    } else if kw.contains(&"source") {
        "fact"
    } else {
        "ontology"
    }
}

/// One top-level form → its digraph, or `""` for `(config …)`.
pub fn to_dot_form(ast: &Ast, form: NodeId, opts: DotOpts) -> String {
    let args = ast.form_args(form).to_vec();
    match ast.head_name(form).unwrap_or("") {
        "ontology" => render_ontology_forms(ast, &args, opts.levi),
        "facts" => render_facts_forms(ast, &args, false, opts.levi),
        "reasoning" => render_facts_forms(ast, &args, true, opts.levi),
        "rules" => render_rules_forms(ast, &args, opts.rule_mode),
        "rule" | "hrule" => render_rule_form(ast, form, opts.rule_mode),
        "query" => render_query(ast, form),
        "trace" => render_trace(ast, form, opts.trace_view),
        // Solver knobs — no graph structure to render.
        "config" => String::new(),
        // A flat fact: render it through its group's view, as a singleton.
        _ => match render_group(ast, form) {
            "ontology" => render_ontology_forms(ast, &[form], opts.levi),
            "fact" => render_facts_forms(ast, &[form], false, opts.levi),
            _ => render_facts_forms(ast, &[form], true, opts.levi),
        },
    }
}

/// A whole program → one `digraph` per rendered group, joined by blank lines.
///
/// A flat program is **re-grouped** by head and provenance kw-pairs back into
/// the views the renderers draw: `rule` / `hrule` → a rule library;
/// `relation` + un-annotated facts → the ontology; `:source`d → facts;
/// `:rule`-derived → reasoning. The deprecated wrapper forms still render
/// directly, in the order they appear, *after* the regrouped chunks.
pub fn to_dot(ast: &Ast, forms: &[NodeId], opts: DotOpts) -> String {
    let forms: Vec<NodeId> = forms
        .iter()
        .copied()
        .filter(|f| is_sform(ast, *f))
        .collect();
    let mut rules: Vec<NodeId> = Vec::new();
    let mut ontology: Vec<NodeId> = Vec::new();
    let mut facts: Vec<NodeId> = Vec::new();
    let mut reasoning: Vec<NodeId> = Vec::new();
    let mut chunks: Vec<String> = Vec::new();
    for f in forms {
        match ast.head_name(f).unwrap_or("") {
            "ontology" | "facts" | "reasoning" | "rules" | "query" | "trace" | "config" => {
                chunks.push(to_dot_form(ast, f, opts));
            }
            "rule" | "hrule" => rules.push(f),
            "relation" => ontology.push(f),
            _ => match render_group(ast, f) {
                "ontology" => ontology.push(f),
                "fact" => facts.push(f),
                _ => reasoning.push(f),
            },
        }
    }
    let mut rendered: Vec<String> = Vec::new();
    if !rules.is_empty() {
        rendered.push(render_rules_forms(ast, &rules, opts.rule_mode));
    }
    if !ontology.is_empty() {
        rendered.push(render_ontology_forms(ast, &ontology, opts.levi));
    }
    if !facts.is_empty() {
        rendered.push(render_facts_forms(ast, &facts, false, opts.levi));
    }
    if !reasoning.is_empty() {
        rendered.push(render_facts_forms(ast, &reasoning, true, opts.levi));
    }
    rendered.extend(chunks);
    rendered.retain(|c| !c.is_empty());
    rendered.join("\n\n")
}
