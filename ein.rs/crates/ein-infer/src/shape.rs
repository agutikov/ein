//! Every plan a KB compiles, as one deterministic text — the S1a.3.1 diff.
//!
//! A `JoinPlan` has no CLI surface, so `ein-conformance` cannot see one: it
//! compares two `ein` binaries, and nothing either of them prints exposes a
//! step sequence, a guard's scope, or a `watched` set. So the compiler is
//! compared the way the loader was at
//! [S1a.2.3](../../../../plans/m1a_rust/p1a.2_kb_core/s1a.2.3_loader_and_provenance.md):
//! both implementations render the same text and the texts are diffed
//! (`utils/ir_oracle.py`'s `plan-shape` op is the other half).
//!
//! Two rules make the text comparable, and they are the same two the KB shape
//! settled on:
//!
//! - **Values are rendered with `repr`**, so the atom `7` prints as `'7'` and
//!   the integer `7` as `7`, and a slot that changed shape cannot hide.
//! - **Sets are rendered sorted** — `watched`, `scope` and a step's shared
//!   variables are `frozenset`s in ein.py, whose order is not reproducible
//!   even run to run.
//!
//! What is deliberately *not* in the text: registers and probes. They are the
//! port's own metadata, ein.py has nothing to compare them against, and the
//! claim that they change no behaviour is checked where it is made — by the
//! matcher's debug assertion that its probe choice is `_candidates`' choice
//! (S1a.3.2).

use ein_core::entities::Rule;
use ein_core::pyrepr::{PyValue, repr, repr_str};
use ein_core::{FactId, Kb, Symbol, Terms};
use ein_ir::{Ast, node_repr};
use rustc_hash::FxHashMap;

use crate::compile::{
    CompileError, activators_for, asserted_relation, naf_relation_refs, negated_relation, plan_key,
};
use crate::match_::{Match, Matcher};
use crate::plan::{GuardArgKind, NafGuard, Plan, Slot, Span, Step};

/// Compile every `(rule, activator)` pair in `Engine.compile_all` order and
/// render the lot.
///
/// The order is `kb.rules` in registry (insertion) order × that rule's
/// activators in `rule_apps_by_rule` order, which is the order the compile
/// cache is built in — and the cache's iteration order is observable through
/// `_enqueue_pass`'s full pass, so it is part of what this diff checks.
pub fn plan_shape(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Result<String, CompileError> {
    plan_shape_with(ast, terms, kb, true)
}

/// [`plan_shape`], optionally without `activators_for`'s S1.22.0 **arity**
/// filter.
///
/// Nothing in the engine compiles an unfiltered pair — both drivers filter
/// first, which is exactly why `compile_rule`'s arity error is otherwise
/// unreachable. `filter_activators: false` is how its fixture reaches it, on
/// both sides (`plan-shape` takes the same flag).
pub fn plan_shape_with(
    ast: &Ast,
    terms: &mut Terms,
    kb: &Kb,
    filter_activators: bool,
) -> Result<String, CompileError> {
    let rules: Vec<Rule> = kb.program().rules.values().cloned().collect();
    let mut out = String::new();
    for rule in &rules {
        let activators = if filter_activators {
            activators_for(kb, terms, rule)
        } else if rule.params.is_empty() {
            vec![None]
        } else {
            kb.rule_apps_by_rule(rule.name).map(Some).collect()
        };
        for activator in activators {
            let key = plan_key(terms, rule, activator);
            let plan = crate::compile::compile_rule(ast, terms, rule, activator)?;
            render_plan(&mut out, ast, terms, &plan, &key.activator);
        }
    }
    // Lines are `"\n".join`ed on the Python side, so there is no trailing one.
    if out.ends_with('\n') {
        out.pop();
    }
    Ok(out)
}

/// Every match every plan produces over `kb` — the S1a.3.2 diff.
///
/// Two sweeps per plan, because the matcher has two entry shapes and they owe
/// each other an identity: the full run, and a `run_seeded` at **every fact in
/// the KB**, which is what forces the premise-order contract — a seeded match's
/// provenance must read exactly like a full run's, seeded fact at its own
/// step's position.
///
/// Bindings go out in bind order (the trail's order, which is
/// `Provenance.bindings`') and premises as fact **positions**, so an order or
/// identity difference names itself rather than showing up as a wall of
/// re-rendered facts.
pub fn match_shape(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Result<String, CompileError> {
    let rules: Vec<Rule> = kb.program().rules.values().cloned().collect();
    let facts: Vec<FactId> = kb.facts().collect();
    let at: FxHashMap<FactId, usize> = facts.iter().enumerate().map(|(i, &f)| (f, i)).collect();
    let mut matcher = Matcher::new();
    let mut out = String::new();
    for rule in &rules {
        for activator in activators_for(kb, terms, rule) {
            let key = plan_key(terms, rule, activator);
            let plan = crate::compile::compile_rule(ast, terms, rule, activator)?;
            let key_repr = repr(&PyValue::Tuple(
                key.activator
                    .iter()
                    .map(|&s| PyValue::Str(terms.sym(s).to_string()))
                    .collect(),
            ));
            out.push_str(&format!("PLAN {} key={key_repr}\n", terms.sym(plan.rule)));
            for d in 0..plan.disjuncts.len() {
                matcher.run_one(kb, terms, ast, &plan, d, &mut |m| {
                    out.push_str(&format!("  RUN D{d} {}\n", match_text(terms, &at, m)));
                    std::ops::ControlFlow::Continue(())
                });
            }
            for (j, &fact) in facts.iter().enumerate() {
                matcher.run_seeded(kb, terms, ast, &plan, fact, &mut |m| {
                    out.push_str(&format!(
                        "  SEED {j} D{} {}\n",
                        m.disjunct,
                        match_text(terms, &at, m)
                    ));
                    std::ops::ControlFlow::Continue(())
                });
            }
        }
    }
    if out.ends_with('\n') {
        out.pop();
    }
    Ok(out)
}

fn match_text(terms: &Terms, at: &FxHashMap<FactId, usize>, m: &Match<'_>) -> String {
    let bindings: Vec<String> = m
        .bindings()
        .map(|(name, value)| {
            format!(
                "({}, {})",
                repr_str(terms.sym(name)),
                repr(&terms.py_value(value))
            )
        })
        .collect();
    let premises: Vec<String> = m.premises().iter().map(|f| at[f].to_string()).collect();
    format!("b=[{}] p=[{}]", bindings.join(", "), premises.join(", "))
}

fn render_plan(out: &mut String, ast: &Ast, terms: &Terms, plan: &Plan, key: &[Symbol]) {
    let key_repr = repr(&PyValue::Tuple(
        key.iter()
            .map(|&s| PyValue::Str(terms.sym(s).to_string()))
            .collect(),
    ));
    let args_repr = repr(&PyValue::Tuple(
        plan.activator_args
            .iter()
            .map(|&s| PyValue::Str(terms.sym(s).to_string()))
            .collect(),
    ));
    let why = plan.why.map(|s| terms.sym(s)).unwrap_or("");
    out.push_str(&format!(
        "PLAN {} key={key_repr} args={args_repr} why={}\n",
        terms.sym(plan.rule),
        repr_str(why),
    ));
    for (name, value) in plan.seed.iter() {
        out.push_str(&format!(
            "  SEED {} {}\n",
            terms.sym(*name),
            repr(&terms.py_value(*value))
        ));
    }
    for (i, d) in plan.disjuncts.iter().enumerate() {
        out.push_str(&format!("  D{i} STEPS {}\n", d.steps.len()));
        render_steps(out, ast, terms, plan, &plan.reg_names, d.steps, 2);
        for (j, g) in plan.guards(d.guards).iter().enumerate() {
            render_guard(out, ast, terms, plan, g, i, j);
        }
    }
    for (i, t) in plan.asserts.iter().enumerate() {
        out.push_str(&format!(
            "  ASSERT {i} {}\n",
            slot_text(ast, terms, plan, &plan.reg_names, t)
        ));
    }
    out.push_str(&format!(
        "  ASSERTED {} NEGATED {}\n",
        opt_name(terms, asserted_relation(plan, terms)),
        opt_name(terms, negated_relation(plan, terms)),
    ));
    let refs = naf_relation_refs(plan, terms);
    if !refs.is_empty() {
        let rendered: Vec<String> = refs
            .iter()
            .map(|(r, neg)| format!("({}, {})", repr_str(terms.sym(*r)), py_bool(*neg)))
            .collect();
        out.push_str(&format!("  NAFREFS [{}]\n", rendered.join(", ")));
    }
}

fn render_guard(
    out: &mut String,
    ast: &Ast,
    terms: &Terms,
    plan: &Plan,
    g: &NafGuard,
    disjunct: usize,
    index: usize,
) {
    out.push_str(&format!(
        "  D{disjunct} GUARD {index} scope=({}) watched=({}) monotone={}\n",
        names(terms, &g.scope),
        names(terms, &g.watched),
        py_bool(g.monotone),
    ));
    render_steps(out, ast, terms, plan, &g.reg_names, g.sub, 2);
}

fn render_steps(
    out: &mut String,
    ast: &Ast,
    terms: &Terms,
    plan: &Plan,
    regs: &[Symbol],
    span: Span,
    depth: usize,
) {
    let pad = "  ".repeat(depth);
    for step in plan.steps(span) {
        match step {
            Step::Rel(r) => {
                let slots: Vec<String> = plan
                    .slots(r.slots)
                    .iter()
                    .map(|s| slot_text(ast, terms, plan, regs, s))
                    .collect();
                let kind = if r.join { "JOIN" } else { "SCAN" };
                let shared = if r.join {
                    format!(" shared=({})", names(terms, plan.shared(r.shared)))
                } else {
                    String::new()
                };
                out.push_str(&format!(
                    "{pad}{kind} {}{shared} [{}]\n",
                    terms.sym(r.rel),
                    slots.join(" ")
                ));
            }
            Step::Guard { pred, args } => {
                let rendered: Vec<String> = plan
                    .guard_args(*args)
                    .iter()
                    .map(|a| node_repr(ast, a.node))
                    .collect();
                out.push_str(&format!(
                    "{pad}GUARD {} [{}]\n",
                    pred.as_str(),
                    rendered.join(", ")
                ));
            }
            Step::Absent { sub } => {
                out.push_str(&format!("{pad}ABSENT {}\n", sub.len()));
                render_steps(out, ast, terms, plan, regs, *sub, depth + 1);
            }
        }
    }
}

/// One compiled slot, in the vocabulary the Python renderer uses.
fn slot_text(ast: &Ast, terms: &Terms, plan: &Plan, regs: &[Symbol], slot: &Slot) -> String {
    match slot {
        Slot::Reg(r) => format!("?{}", terms.sym(regs[*r as usize])),
        Slot::Const(v) => repr(&terms.py_value(*v)),
        Slot::Nested { rel, slots } => {
            let mut s = format!("({}", terms.sym(*rel));
            for inner in plan.slots(*slots) {
                s.push(' ');
                s.push_str(&slot_text(ast, terms, plan, regs, inner));
            }
            s.push(')');
            s
        }
        Slot::Opaque(node) => node_repr(ast, *node),
    }
}

fn names(terms: &Terms, syms: &[Symbol]) -> String {
    syms.iter()
        .map(|&s| terms.sym(s))
        .collect::<Vec<_>>()
        .join(" ")
}

fn opt_name(terms: &Terms, sym: Option<Symbol>) -> String {
    match sym {
        Some(s) => repr_str(terms.sym(s)),
        None => "None".to_string(),
    }
}

fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

/// A register's name, for the unbound-`:assert`-var message and for tests.
pub fn reg_name<'a>(terms: &'a Terms, plan: &Plan, reg: crate::plan::Reg) -> &'a str {
    terms.sym(plan.reg_names[reg as usize])
}

/// A guard argument as ein.py stores it — the raw IR node.
pub fn guard_arg_text(ast: &Ast, node: ein_ir::NodeId, _kind: GuardArgKind) -> String {
    node_repr(ast, node)
}
