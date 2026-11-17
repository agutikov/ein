//! The trace AST.
//!
//! A derivation's steps as `steps_of` narrates them: each carries the rule
//! name, the premises it consumed, the derived edge, the bindings, and a
//! generated English explanation. `diagram` and `section` are render-time
//! enrichments.
//!
//! **A trace is written, never read back**: nothing writes these steps as a
//! `(trace …)` form or parses one into them. A `(trace …)` form a program
//! writes is drawn by [`crate::ir_dot::render_trace`].
//!
//! **Why a fact here is not a `FactId`.** Everywhere else in the engine a
//! fact is an interned id. [`FactRef`] is an owned `(relation_name, args)`
//! pair, which [`fact_ref`] converts *out* of the KB where the linearizer
//! needs it, so a step outlives the store it was read from.

use ein_core::{FactId, Tag, Terms, Value};

/// One argument of a [`FactRef`]: a name or a nested fact. A run of digits
/// is a name.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RefArg {
    Str(String),
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
    /// shape, recursing into nested references. A name prints as the one
    /// printer prints it: bare where it lexes as itself, else
    /// quoted, so `holds("x y")` is one argument and `holds(1)` a name.
    pub fn label(&self) -> String {
        let parts: Vec<String> = self
            .args
            .iter()
            .map(|a| match a {
                RefArg::Str(s) => ein_core::text::symbol(s).into_owned(),
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
    /// Variable bindings in bind order — the order the matcher first bound
    /// each name.
    pub bindings: Vec<(String, String)>,
    pub why: String,
    /// What the derived fact says: its relation's `:why`, positional. `None`
    /// where the relation carries none.
    pub statement: Option<String>,
    /// The firing's other conclusions, in the order the rule writes them,
    /// each with its statement: a firing stores every conclusion of its
    /// rule, and a step names each (Q-M1f.214). Empty for a rule of one
    /// conclusion.
    pub more: Vec<(FactRef, Option<String>)>,
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
            statement: None,
            more: Vec::new(),
            diagram: None,
            section: None,
            sources: Vec::new(),
            conditional: false,
        }
    }

    pub fn derived_label(&self) -> String {
        self.derived.label()
    }

    /// Every conclusion's label, the first one first.
    pub fn derived_labels(&self) -> Vec<String> {
        std::iter::once(&self.derived)
            .chain(self.more.iter().map(|(f, _)| f))
            .map(FactRef::label)
            .collect()
    }

    pub fn premise_labels(&self) -> Vec<String> {
        self.premises.iter().map(FactRef::label).collect()
    }
}
