//! The per-form IR → DOT renderer: `ein render program`, the program as
//! written, in three parts (Q-M1f.420).
//!
//! - **`kb`**: every fact and declaration the program states, as written,
//!   drawn as one graph by the KB view's code ([`crate::fact_graph`]), in
//!   the canonical form or folded (Q-M1f.419). A program with no rule, no
//!   import and no macro draws the same digraph as `ein render kb` of it,
//!   but for what the loader adds. The knowledge base is not split.
//! - **the rules**: a digraph per rule, hrule and macro, as `ein render rule`
//!   draws it ([`crate::rules`]).
//! - **`query`**: the queries, each its `:goal` drawn as a rule's match is,
//!   and the clauses beside it in a note, and the `(solve …)` clause, a note.
//!
//! A written `(trace …)` is drawn after them, as its own digraph. It is
//! drawn from the parsed forms, so imports are not resolved and a macro's
//! call stands where it is written; an `(import …)` and a `(config …)` draw
//! nothing.

use std::collections::{HashMap, HashSet};

use ein_ir::{Ast, Node, NodeId};

use crate::builder::Builder;
use crate::dot_util::{POLYGON_RANKSEP, multiline, quote, value_label};
use crate::fact_graph::{Fact, FactSet, Look, Under, fact_id};
use crate::kb_dot::{kb_digraph_open, short_source};
use crate::palette::hash_color;
use crate::rules::{RuleMode, goal_cluster, render_rule_form};

/// The rendering options `to_dot` threads through every form.
#[derive(Clone, Copy)]
pub struct DotOpts {
    pub rule_mode: RuleMode,
    pub trace_view: TraceView,
    /// Fold a fact or a pattern of three arrows that nothing drawn points
    /// at into one labelled arrow (Q-M1f.419).
    pub fold: bool,
}

impl Default for DotOpts {
    fn default() -> Self {
        DotOpts {
            rule_mode: RuleMode::SideBySide,
            trace_view: TraceView::PerStep,
            fold: false,
        }
    }
}

/// `render_trace`'s view, with friendly names alongside the legacy letters
/// (`a`/`per-step`, `b`/`aggregate`, `c`/`dag`).
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

/// The declarators: a top-level form headed by one of these is no fact.
const DECLARATORS: [&str; 8] = [
    "rule", "hrule", "macro", "query", "solve", "config", "import", "trace",
];

// ── node ids ───────────────────────────────────────────────────────

/// A DOT-safe quoted identifier for an atom-like node.
fn atom_id(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Var(s) => quote(&format!("?{}", ast.sym(s))),
        Node::Atom(s) => quote(ast.sym(s)),
        // Every caller hands over an atom-like node, so anything else is
        // unreachable.
        _ => panic!("not an atom-like node"),
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

/// Is `form` a fact the program states: a top-level form no declarator
/// heads? A written `(eq …)` or `(neq …)`, and `(not (eq …))`, which is
/// `(neq …)`, are checks and never stored, so no fact (Q-M1f.33, Q-M1f.31).
fn is_fact_form(ast: &Ast, form: NodeId) -> bool {
    let check = |f: NodeId| ast.head_name(f).is_some_and(ein_core::is_predicate);
    is_sform(ast, form)
        && !ast
            .head_name(form)
            .is_some_and(|h| DECLARATORS.contains(&h))
        && !check(form)
        && !(ast.head_name(form) == Some("not")
            && positional(ast, form).first().is_some_and(|g| check(*g)))
}

/// The text of a fact as an object: its relation and its arguments, the
/// clauses a written fact carries left out, so that a fact stated with a
/// source and the same fact under an arrow are one text. It is
/// [`value_label`]'s text for a fact that carries none.
fn fact_text(ast: &Ast, fact: NodeId) -> String {
    let head = ast.head_name(fact).unwrap_or("");
    let args: Vec<String> = positional(ast, fact)
        .iter()
        .map(|a| value_label(ast, *a))
        .collect();
    if args.is_empty() {
        format!("({head})")
    } else {
        format!("({head} {})", args.join(" "))
    }
}

// ── the knowledge base as written ──────────────────────────────────

/// How a fact the program states is drawn: by its relation, dashed where
/// it is written as derived (`:rule`), with the short text of its source or
/// `by <rule>` beside it, as the KB view draws the fact the loader makes of
/// it.
fn stated_look(ast: &Ast, fact: NodeId) -> Look {
    let head = ast.head_name(fact).unwrap_or("");
    let mut look = Look::plain(hash_color(head));
    let source = kw_get(ast, fact, "source").and_then(|v| match ast.node(v) {
        Node::Str(s) => Some(ast.sym(s).to_string()),
        _ => None,
    });
    let rule = kw_get(ast, fact, "rule").map(|v| value_label(ast, v));
    if rule.is_some() || kw_get(ast, fact, "using").is_some() {
        look.dashed = true;
    }
    look.note = match (source.filter(|s| !s.is_empty()), rule) {
        (Some(s), _) => Some(short_source(&s)),
        (None, Some(r)) => Some(format!("by {r}")),
        (None, None) => None,
    };
    look
}

struct Written<'a> {
    ast: &'a Ast,
    set: FactSet,
    at: HashMap<String, usize>,
    stated: HashSet<String>,
}

impl Written<'_> {
    fn add(&mut self, fact: NodeId) -> usize {
        let text = fact_text(self.ast, fact);
        if let Some(&i) = self.at.get(&text) {
            return i;
        }
        let ast = self.ast;
        let head = ast.head_name(fact).unwrap_or("").to_string();
        let mut texts: Vec<String> = Vec::new();
        let mut under: Vec<Under> = Vec::new();
        for a in positional(ast, fact) {
            texts.push(value_label(ast, a));
            under.push(if is_sform(ast, a) {
                Under::Fact(self.add(a))
            } else {
                Under::Name(value_label(ast, a))
            });
        }
        let i = self.set.facts.len();
        self.set.facts.push(Fact {
            id: fact_id(&head, &texts),
            look: Look::plain(hash_color(&head)),
            relation: head,
            args: under,
            held: self.stated.contains(&text),
        });
        self.at.insert(text, i);
        i
    }
}

/// The facts and declarations of `facts` as a [`FactSet`], in the order the
/// loader reads them: the declarations, then the other facts, each in the
/// order written. A fact written twice is one fact, drawn as written first.
pub fn written_fact_set(ast: &Ast, facts: &[NodeId]) -> FactSet {
    let facts: Vec<NodeId> = facts
        .iter()
        .copied()
        .filter(|f| is_fact_form(ast, *f))
        .collect();
    let declarations = facts
        .iter()
        .copied()
        .filter(|f| ast.head_name(*f) == Some("relation"));
    let others = facts
        .iter()
        .copied()
        .filter(|f| ast.head_name(*f) != Some("relation"));
    let drawn: Vec<NodeId> = declarations.chain(others).collect();
    fact_set_of(ast, &facts, &drawn)
}

/// The facts `drawn`, in their order, as a [`FactSet`], with every fact
/// they point at: a fact is held where it is one of `stated`, the facts
/// the program states, and stands in the frame of its context where not.
pub(crate) fn fact_set_of(ast: &Ast, stated: &[NodeId], drawn: &[NodeId]) -> FactSet {
    let mut w = Written {
        ast,
        set: FactSet::default(),
        at: HashMap::new(),
        stated: stated
            .iter()
            .filter(|f| is_fact_form(ast, **f))
            .map(|f| fact_text(ast, *f))
            .collect(),
    };
    let mut seen: HashSet<usize> = HashSet::new();
    for &f in drawn {
        let i = w.add(f);
        if seen.insert(i) {
            w.set.facts[i].look = stated_look(ast, f);
            w.set.drawn.push(i);
        }
    }
    w.set
}

/// The digraph `kb`: the facts the program states, drawn by the KB view's
/// code; `""` where it states none.
pub fn render_kb(ast: &Ast, forms: &[NodeId], fold: bool) -> String {
    let set = written_fact_set(ast, forms);
    if set.drawn.is_empty() {
        return String::new();
    }
    let mut lines = kb_digraph_open("kb");
    lines.extend(set.body(fold));
    lines.push("}".to_string());
    lines.join("\n")
}

// ── the query ──────────────────────────────────────────────────────

/// The clauses of a `(query …)` or `(solve …)` as a note's lines: each
/// keyword and its value as written, a flag alone.
fn note_lines(ast: &Ast, form: NodeId, skip: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    for a in ast.form_args(form) {
        if let Node::KwPair { key, value } = ast.node(*a)
            && let Node::Keyword(s) = ast.node(key)
        {
            let k = ast.sym(s);
            if k == skip {
                continue;
            }
            // A flag, `:no-search`, has no value: the parser gives it the
            // empty form.
            if ast.head_name(value) == Some("#empty") && positional(ast, value).is_empty() {
                parts.push(format!(":{k}"));
            } else {
                parts.push(format!(":{k} {}", value_label(ast, value)));
            }
        }
    }
    parts
}

/// The digraph `query`: each `(query …)` a cluster, its `:goal` drawn as a
/// rule's match is and its other clauses in a note, and the `(solve …)`
/// clause a note; `""` where the program has neither.
pub fn render_queries(ast: &Ast, forms: &[NodeId], fold: bool) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut n = 0;
    for &f in forms {
        match ast.head_name(f) {
            Some("query") => {
                n += 1;
                let rest = note_lines(ast, f, "goal");
                match kw_get(ast, f, "goal").filter(|g| is_sform(ast, *g)) {
                    Some(goal) => {
                        let mut cluster = goal_cluster(ast, goal, n, fold);
                        // The note stands inside the query's cluster, before
                        // its closing brace.
                        let close = cluster.pop().expect("a cluster closes");
                        if !rest.is_empty() {
                            let parts: Vec<&str> = rest.iter().map(String::as_str).collect();
                            cluster.push(format!(
                                "    \"query_{n}\" [shape=note, label={}];",
                                multiline(&parts)
                            ));
                        }
                        cluster.push(close);
                        lines.extend(cluster);
                    }
                    None => {
                        let parts: Vec<&str> = rest.iter().map(String::as_str).collect();
                        lines.push(format!(
                            "  \"query_{n}\" [shape=note, label={}];",
                            if parts.is_empty() {
                                quote("query")
                            } else {
                                multiline(&parts)
                            }
                        ));
                    }
                }
            }
            Some("solve") => {
                let rest = note_lines(ast, f, "");
                let parts: Vec<&str> = rest.iter().map(String::as_str).collect();
                let label = if parts.is_empty() {
                    quote("solve")
                } else {
                    multiline(&parts)
                };
                lines.push(format!("  \"solve\" [shape=note, label={label}];"));
            }
            _ => {}
        }
    }
    if lines.is_empty() {
        return String::new();
    }
    let mut out = vec!["digraph query {".to_string(), "  rankdir=TB;".to_string()];
    out.extend(lines);
    out.push("}".to_string());
    out.join("\n")
}

/// `(query …)` alone — the query digraph of one form.
pub fn render_query(ast: &Ast, form: NodeId) -> String {
    render_queries(ast, &[form], false)
}

// ── a written trace ────────────────────────────────────────────────

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

/// View (c) — the derivation a written trace records: each step a node of
/// its own, from its premises to the fact it derives, and each fact derived
/// drawn as its node in the canonical form (Q-M1f.419), never a node
/// labelled with its text. A premise that names an earlier step is the fact
/// that step derived; any other is a name.
fn render_trace_dag(ast: &Ast, form: NodeId) -> String {
    let mut w = Written {
        ast,
        set: FactSet::default(),
        at: HashMap::new(),
        stated: HashSet::new(),
    };
    // What the trace derives is what it holds: a fact under an arrow of a
    // derived fact is in a context only where no step derives it.
    for ev in ast.form_args(form).to_vec() {
        if let Some(d) = kw_get(ast, ev, "derives").filter(|v| is_sform(ast, *v)) {
            w.stated.insert(fact_text(ast, d));
        }
    }
    let mut steps: Vec<String> = Vec::new();
    let mut edges: Vec<String> = Vec::new();
    // step name → the node id of the fact it derives, in first-seen order.
    let mut derived_by_step: Vec<(String, String)> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut drawn: HashSet<usize> = HashSet::new();
    for ev in ast.form_args(form).to_vec() {
        if !is_sform(ast, ev) {
            continue;
        }
        let Some(step_name) = leading_symbol(ast, ev).map(str::to_string) else {
            continue;
        };
        let rule = kw_get(ast, ev, "rule").map(|v| value_label(ast, v));
        let step_id = quote(&format!("step {step_name}"));
        let label = match &rule {
            Some(r) => multiline(&[&step_name, r]),
            None => quote(&step_name),
        };
        steps.push(format!(
            "  {step_id} [shape=box, style=rounded, label={label}];"
        ));
        if let Some(using) = kw_get(ast, ev, "using").filter(|v| is_sform(ast, *v)) {
            for premise in trace_premises(ast, using) {
                let Node::Atom(s) = ast.node(premise) else {
                    continue;
                };
                let pname = ast.sym(s).to_string();
                let pid = match derived_by_step.iter().find(|(k, _)| *k == pname) {
                    Some((_, v)) => v.clone(),
                    None => {
                        if !names.contains(&pname) {
                            names.push(pname.clone());
                        }
                        atom_id(ast, premise)
                    }
                };
                edges.push(format!("  {pid} -> {step_id};"));
            }
        }
        if let Some(d) = kw_get(ast, ev, "derives").filter(|v| is_sform(ast, *v)) {
            let i = w.add(d);
            if drawn.insert(i) {
                let head = ast.head_name(d).unwrap_or("");
                w.set.facts[i].look = Look {
                    penwidth: Some(2),
                    ..Look::plain(hash_color(head))
                };
                w.set.drawn.push(i);
            }
            let did = w.set.facts[i].id.clone();
            edges.push(format!("  {step_id} -> {did} [style=bold];"));
            match derived_by_step.iter_mut().find(|(k, _)| *k == step_name) {
                Some(slot) => slot.1 = did,
                None => derived_by_step.push((step_name.clone(), did)),
            }
        }
    }
    let mut lines = vec![
        "digraph trace {".to_string(),
        format!("  {POLYGON_RANKSEP};"),
        "  node [shape=box];".to_string(),
    ];
    lines.extend(w.set.body(false));
    lines.push(String::new());
    lines.push("  // steps".to_string());
    lines.extend(steps);
    lines.extend(edges);
    lines.push("}".to_string());
    lines.join("\n")
}

// ── top-level dispatch ─────────────────────────────────────────────

/// One top-level form → its digraph, or `""` for one that draws nothing,
/// `(config …)` and `(import …)`.
pub fn to_dot_form(ast: &Ast, form: NodeId, opts: DotOpts) -> String {
    match ast.head_name(form).unwrap_or("") {
        "rule" | "hrule" | "macro" => render_rule_form(ast, form, opts.rule_mode, opts.fold),
        "query" | "solve" => render_queries(ast, &[form], opts.fold),
        "trace" => render_trace(ast, form, opts.trace_view),
        "config" | "import" => String::new(),
        // A fact: the knowledge base of that one fact.
        _ => render_kb(ast, &[form], opts.fold),
    }
}

/// A whole program → its three parts, `kb`, a digraph per rule, hrule and
/// macro, and `query`, then a written trace's digraph; those that draw
/// nothing are left out, and the rest joined by blank lines.
pub fn to_dot(ast: &Ast, forms: &[NodeId], opts: DotOpts) -> String {
    let forms: Vec<NodeId> = forms
        .iter()
        .copied()
        .filter(|f| is_sform(ast, *f))
        .collect();
    let mut rendered: Vec<String> = vec![render_kb(ast, &forms, opts.fold)];
    for &f in &forms {
        if matches!(ast.head_name(f), Some("rule" | "hrule" | "macro")) {
            rendered.push(render_rule_form(ast, f, opts.rule_mode, opts.fold));
        }
    }
    rendered.push(render_queries(ast, &forms, opts.fold));
    for &f in &forms {
        if ast.head_name(f) == Some("trace") {
            rendered.push(render_trace(ast, f, opts.trace_view));
        }
    }
    rendered.retain(|c| !c.is_empty());
    rendered.join("\n\n")
}
