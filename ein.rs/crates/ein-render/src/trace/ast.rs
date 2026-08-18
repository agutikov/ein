//! The trace AST — `ein.py`'s `trace/ast.py`.
//!
//! A trace is itself IR: a list of `(step …)` forms carrying the rule name,
//! the premises it consumed, the derived edge, the bindings, and a generated
//! English explanation. [`trace_to_ir`] and [`parse_trace_steps`] round-trip
//! it through the parser as a `(trace …)` form — the same form
//! [`crate::ir_dot::render_trace`] draws.
//!
//! The serialisable core is `(n, rule, premises, derived, why)` plus the
//! bindings. `diagram` and `section` are render-time enrichments and come back
//! `None` after a round trip.
//!
//! **Why a fact here is not a `FactId`.** Everywhere else in the port a fact
//! is an interned id. A parsed trace is the exception: `parse_trace_steps`
//! rebuilds facts that may name relations and objects no KB ever held, so
//! there is nothing to intern them *into*. [`FactRef`] is therefore the owned
//! `(relation_name, args)` shape ein.py's `FactRef` alias describes, and
//! [`fact_ref`] converts one *out* of the KB where the linearizer needs it.

use ein_core::{FactId, Tag, Terms, Value};
use ein_ir::dump::escape_string_literal;
use ein_ir::{Ast, Node, NodeId, node_repr};

/// One argument of a [`FactRef`]: a string, an integer as its canonical
/// decimal text, or a nested fact.
///
/// `Str` and `Int` are distinct because the serialiser treats them
/// differently — an integer is written bare, a string is written bare only if
/// it is atom-safe.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RefArg {
    Str(String),
    Int(String),
    Fact(FactRef),
}

/// A fact reference: `(relation_name, args)`, mirroring the kernel's fact-id
/// shape.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FactRef {
    pub rel: String,
    pub args: Vec<RefArg>,
}

impl FactRef {
    /// The empty reference `("", ())` — what a `(step …)` with no `:derives`
    /// falls back to.
    pub fn empty() -> FactRef {
        FactRef {
            rel: String::new(),
            args: Vec::new(),
        }
    }

    /// The readable `rel(a, b, …)` label — `dot_util.fact_label` over this
    /// shape, recursing into nested references.
    pub fn label(&self) -> String {
        let parts: Vec<String> = self
            .args
            .iter()
            .map(|a| match a {
                RefArg::Str(s) => s.clone(),
                RefArg::Int(i) => i.clone(),
                RefArg::Fact(f) => f.label(),
            })
            .collect();
        let inner = parts.join(", ");
        if inner.is_empty() {
            self.rel.clone()
        } else {
            format!("{}({inner})", self.rel)
        }
    }
}

/// A KB fact as a [`FactRef`] — `(f.relation_name, f.args)`.
pub fn fact_ref(terms: &Terms, f: FactId) -> FactRef {
    let (rel, args) = terms.fact(f);
    FactRef {
        rel: terms.sym(rel).to_string(),
        args: args.iter().map(|a| ref_arg(terms, *a)).collect(),
    }
}

fn ref_arg(terms: &Terms, v: Value) -> RefArg {
    match v.tag() {
        Tag::Sym => RefArg::Str(terms.sym(v.as_sym().expect("tagged Sym")).to_string()),
        Tag::Int => RefArg::Int(terms.int_text(v.as_int().expect("tagged Int")).to_string()),
        Tag::Fact => RefArg::Fact(fact_ref(terms, v.as_fact().expect("tagged Fact"))),
    }
}

/// One narrated reasoning move — one rule firing.
#[derive(Clone, Debug)]
pub struct TraceStep {
    pub n: u64,
    pub rule: String,
    pub premises: Vec<FactRef>,
    pub derived: FactRef,
    /// Variable bindings in bind order — a Python `dict`, whose order is the
    /// order the matcher first bound each name.
    pub bindings: Vec<(String, String)>,
    pub why: String,
    /// The inline DOT slice — render-time, not serialised.
    pub diagram: Option<String>,
    /// The clustering key (target entity) — render-time, not serialised.
    pub section: Option<String>,
    /// Quoted source sentences.
    pub sources: Vec<String>,
    /// Whether the derivation depends on a hypothesis (commitment) fact.
    pub conditional: bool,
}

impl TraceStep {
    pub fn new(n: u64, rule: String, derived: FactRef) -> TraceStep {
        TraceStep {
            n,
            rule,
            premises: Vec::new(),
            derived,
            bindings: Vec::new(),
            why: String::new(),
            diagram: None,
            section: None,
            sources: Vec::new(),
            conditional: false,
        }
    }

    pub fn derived_label(&self) -> String {
        self.derived.label()
    }

    pub fn premise_labels(&self) -> Vec<String> {
        self.premises.iter().map(FactRef::label).collect()
    }
}

// ── IR round-trip: out ─────────────────────────────────────────────

/// Whether a string can be written as a bare atom rather than a literal.
///
/// ein.py is `s and all(c.isalnum() or c in "-_*?." for c in s) and not
/// s[0].isdigit()`. `isalnum` / `isdigit` are Python's *Unicode* predicates;
/// `char::is_alphanumeric` / `is_numeric` are the closest Rust equivalents and
/// agree with them over every character the grammar admits in a symbol.
fn atom_safe(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if !s
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '*' | '?' | '.'))
    {
        return false;
    }
    !s.chars().next().expect("non-empty").is_numeric()
}

fn arg_to_sexpr(a: &RefArg) -> String {
    match a {
        RefArg::Fact(f) => fact_to_sexpr(f),
        RefArg::Int(i) => i.clone(),
        RefArg::Str(s) => {
            if atom_safe(s) {
                s.clone()
            } else {
                escape_string_literal(s)
            }
        }
    }
}

fn fact_to_sexpr(f: &FactRef) -> String {
    let inner: Vec<String> = f.args.iter().map(arg_to_sexpr).collect();
    if inner.is_empty() {
        format!("({})", f.rel)
    } else {
        format!("({} {})", f.rel, inner.join(" "))
    }
}

/// One `(step …)` S-expression line.
pub fn step_to_ir(step: &TraceStep) -> String {
    let mut parts = vec![format!("(step s{} :rule {}", step.n, step.rule)];
    if !step.premises.is_empty() {
        let using = if step.premises.len() == 1 {
            fact_to_sexpr(&step.premises[0])
        } else {
            let inner: Vec<String> = step.premises.iter().map(fact_to_sexpr).collect();
            format!("(and {})", inner.join(" "))
        };
        parts.push(format!(":using {using}"));
    }
    parts.push(format!(":derives {}", fact_to_sexpr(&step.derived)));
    if !step.bindings.is_empty() {
        let binds: Vec<String> = step
            .bindings
            .iter()
            .map(|(k, v)| format!("?{k} {}", arg_to_sexpr(&RefArg::Str(v.clone()))))
            .collect();
        parts.push(format!(":bind ({})", binds.join(" ")));
    }
    if !step.why.is_empty() {
        parts.push(format!(":why {}", escape_string_literal(&step.why)));
    }
    format!("{})", parts.join(" "))
}

/// The whole trace as a `(trace …)` IR form, which round-trips through the
/// parser.
pub fn trace_to_ir(steps: &[TraceStep]) -> String {
    if steps.is_empty() {
        return "(trace)".to_string();
    }
    let body: Vec<String> = steps.iter().map(step_to_ir).collect();
    format!("(trace\n  {})", body.join("\n  "))
}

// ── IR round-trip: back ────────────────────────────────────────────

/// One IR scalar's Python value: `Atom` → its name, `Int` / `String` → their
/// value, anything else → `str(x)` — the dataclass repr.
fn atom_or_value(ast: &Ast, id: NodeId) -> RefArg {
    match ast.node(id) {
        Node::Atom(s) => RefArg::Str(ast.sym(s).to_string()),
        Node::Str(s) => RefArg::Str(ast.sym(s).to_string()),
        Node::Int(s) => RefArg::Int(ast.sym(s).to_string()),
        _ => RefArg::Str(node_repr(ast, id)),
    }
}

fn sform_to_factref(ast: &Ast, form: NodeId) -> FactRef {
    let (head, args) = match ast.node(form) {
        Node::SForm { head, args } => (head, ast.args(args).to_vec()),
        _ => return FactRef::empty(),
    };
    let rel = match ast.node(head) {
        Node::Atom(s) => ast.sym(s).to_string(),
        _ => node_repr(ast, head),
    };
    let mut out = Vec::new();
    for a in args {
        match ast.node(a) {
            Node::KwPair { .. } => continue,
            Node::SForm { .. } => out.push(RefArg::Fact(sform_to_factref(ast, a))),
            _ => out.push(atom_or_value(ast, a)),
        }
    }
    FactRef { rel, args: out }
}

/// Premises from a `:using` value — an `(and …)` of facts, or one fact.
fn parse_using(ast: &Ast, val: NodeId) -> Vec<FactRef> {
    let Node::SForm { head, args } = ast.node(val) else {
        return Vec::new();
    };
    // `val.head.name` — an AttributeError in ein.py when the head is a `?var`,
    // and unreachable, because a `:using` is always written by `step_to_ir`.
    let is_and = matches!(ast.node(head), Node::Atom(s) if ast.sym(s) == "and");
    if is_and {
        return ast
            .args(args)
            .iter()
            .filter(|a| matches!(ast.node(**a), Node::SForm { .. }))
            .map(|a| sform_to_factref(ast, *a))
            .collect();
    }
    vec![sform_to_factref(ast, val)]
}

/// Variable bindings from a `:bind (?v value …)` flat pair list.
fn parse_bindings(ast: &Ast, val: NodeId) -> Vec<(String, String)> {
    let Node::SForm { head, args } = ast.node(val) else {
        return Vec::new();
    };
    let mut items = vec![head];
    items.extend_from_slice(ast.args(args));
    let mut out: Vec<(String, String)> = Vec::new();
    let mut i = 0;
    while i + 1 < items.len() {
        // `getattr(items[i], "name", None)` — only `Atom` / `Var` / `Keyword`
        // carry a `name`, and an empty one is falsy, so it is skipped too.
        let name = match ast.node(items[i]) {
            Node::Atom(s) | Node::Var(s) | Node::Keyword(s) => Some(ast.sym(s).to_string()),
            _ => None,
        };
        if let Some(name) = name.filter(|n| !n.is_empty()) {
            let value = match atom_or_value(ast, items[i + 1]) {
                RefArg::Str(s) => s,
                RefArg::Int(s) => s,
                RefArg::Fact(f) => f.label(),
            };
            match out.iter_mut().find(|(k, _)| *k == name) {
                Some(slot) => slot.1 = value,
                None => out.push((name, value)),
            }
        }
        i += 2;
    }
    out
}

fn kw_get(ast: &Ast, form: NodeId, key: &str) -> Option<NodeId> {
    let mut found = None;
    for a in ast.form_args(form) {
        if let Node::KwPair { key: k, value } = ast.node(*a)
            && let Node::Keyword(s) = ast.node(k)
            && ast.sym(s) == key
        {
            // Last wins — `kw_map` builds a dict.
            found = Some(value);
        }
    }
    found
}

/// One `(step …)` form → a [`TraceStep`], the serialisable core only.
fn parse_step(ast: &Ast, ev: NodeId, default_n: u64) -> TraceStep {
    let name = ast
        .form_args(ev)
        .iter()
        .find_map(|a| match ast.node(*a) {
            Node::Atom(s) => Some(ast.sym(s).to_string()),
            _ => None,
        })
        .unwrap_or_else(|| "s0".to_string());
    let tail: String = name.chars().skip(1).collect();
    let n = if !tail.is_empty() && tail.chars().all(|c| c.is_numeric()) {
        tail.parse::<u64>().unwrap_or(default_n)
    } else {
        default_n
    };
    let rule = match kw_get(ast, ev, "rule") {
        None => String::new(),
        Some(v) => match ast.node(v) {
            Node::Atom(s) => ast.sym(s).to_string(),
            _ => node_repr(ast, v),
        },
    };
    let is_sform = |id: NodeId| matches!(ast.node(id), Node::SForm { .. });
    let mut step = TraceStep::new(
        n,
        rule,
        kw_get(ast, ev, "derives")
            .filter(|v| is_sform(*v))
            .map_or_else(FactRef::empty, |v| sform_to_factref(ast, v)),
    );
    step.premises = kw_get(ast, ev, "using")
        .filter(|v| is_sform(*v))
        .map_or_else(Vec::new, |v| parse_using(ast, v));
    step.bindings = kw_get(ast, ev, "bind")
        .filter(|v| is_sform(*v))
        .map_or_else(Vec::new, |v| parse_bindings(ast, v));
    step.why = match kw_get(ast, ev, "why") {
        Some(v) => match ast.node(v) {
            Node::Str(s) => ast.sym(s).to_string(),
            _ => String::new(),
        },
        None => String::new(),
    };
    step
}

/// Reconstruct `TraceStep`s from a parsed `(trace …)` form.
pub fn parse_trace_steps(ast: &Ast, form: NodeId) -> Vec<TraceStep> {
    let mut steps: Vec<TraceStep> = Vec::new();
    for ev in ast.form_args(form).to_vec() {
        if ast.head_name(ev) == Some("step") {
            let n = steps.len() as u64 + 1;
            steps.push(parse_step(ast, ev, n));
        }
    }
    steps
}
