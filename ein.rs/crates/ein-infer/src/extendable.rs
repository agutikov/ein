//! `D⁺`: the relations a commitment can change, followed through the rules
//! root has no plan of
//! ([03-absent-and-naf.md](../../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md#a-refutation-resting-on-an-absent)).
//!
//! The static check's `D` ([`crate::monotone`]) is closed over the plans root
//! compiled. A rule with parameters whose activator relation is in `D` is an
//! activator site: a node compiles a plan of it root never had, and what that
//! plan writes a commitment can change too, though `D` does not hold it, nor
//! what a rule activated by it writes. `D⁺` follows those rules as well, each
//! read from its pattern (`monotone::through_activators`), so a chain of
//! activators that first appears at a node is followed. It is `D` on a
//! program with no activator site.
//!
//! A relation of `D⁺` is called **extendable**: a commitment can extend its
//! held extent. The reading is at relation grain and decides nothing: it
//! refuses no program, chooses no node and moves no answer.
//!
//! ### Its readers
//!
//! - [`derivable_activators`]: which activators of an obligation rule a
//!   commitment can derive, so that H holds what a debt root does not owe
//!   would propose ([`crate::oblgen`]). Its `D⁺` is seeded with the relations
//!   of what the obligations rung proposes.
//! - [`relations`]: `D⁺` over the static check's `D`, sorted, for a reader
//!   that studies which relations a commitment can change.
//!
//! Both read root's saturation plans, and every rule with parameters from its
//! pattern, once (`extendable_over`); neither runs a saturation, interns a
//! name or narrates an event.

use std::sync::Arc;

use ein_core::{Kb, Rule, Symbol, Terms};
use ein_ir::{Ast, Node, NodeId};

use crate::monotone::{self, Monotonicity, Rel};
use crate::plan::Plan;

/// `D⁺` over the static check's `D`, sorted by label. `plans` are root's,
/// `m` the static check over them. It is `D` on a program with no activator
/// site.
pub fn relations(
    kb: &Kb,
    terms: &Terms,
    ast: &Ast,
    plans: &[Arc<Plan>],
    m: &Monotonicity,
) -> Vec<Rel> {
    let program = kb.program();
    let saturation: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let reads: Vec<Vec<(Rel, u32)>> = saturation
        .iter()
        .map(|p| monotone::reads_of(p, terms))
        .collect();
    let writes = pattern_writes(kb, terms, ast);
    // determinism-ok: sorted by label on the next line.
    let mut out: Vec<Rel> = extendable_over(
        m.dynamic.iter().copied(),
        &saturation,
        &reads,
        &writes,
        terms,
    )
    .into_iter()
    .collect();
    out.sort_by_key(|r| r.label(terms));
    out
}

/// One activator of an obligation rule that a commitment can derive
/// ([`derivable_activators`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Derivable {
    /// The obligation rule the conclusion would activate.
    pub obligation: Symbol,
    /// The conclusion, a form that holds no variable.
    pub form: NodeId,
    /// The saturation rule whose `:assert` writes it.
    pub by: Symbol,
}

/// **The activators of an obligation rule that a commitment can derive**, by
/// the rules as written and root's plans alone: every conclusion that holds
/// no variable, heads an obligation rule with parameters and has as many
/// arguments as that rule has parameters, in the `:assert` of a saturation
/// rule that reads a relation of `D⁺` or is activated by one. In the rules'
/// load order, and a rule's conclusions as it writes them.
///
/// `D⁺` is the module's, by its one reading ([`extendable_over`]): `seed`,
/// the relations of what a commitment can add, closed under root's saturation
/// plans and followed through the rules root has no plan of. A rule *reads a
/// relation of `D⁺`* where a plan root compiled of it does, at any `absent`
/// depth, and *is activated by one* where it has parameters and its own name
/// is in `D⁺`: a node can then compile a plan of it root never had.
///
/// It is a reading at relation grain, and inexact both ways. **Over**: a rule
/// that reads a relation a commitment can change need fire at no alive node.
/// **Under**: a conclusion that writes the activator with a variable in it,
/// a parameter of its own rule among them, is not read, whatever binds the
/// variable. The obligations rung reads it for what a debt root does not owe
/// would propose ([`crate::oblgen`]); it runs no saturation, interns nothing
/// and narrates nothing.
pub(crate) fn derivable_activators(
    kb: &Kb,
    terms: &Terms,
    ast: &Ast,
    plans: &[Arc<Plan>],
    seed: impl IntoIterator<Item = Rel>,
) -> Vec<Derivable> {
    let program = kb.program();
    // No obligation rule a fact can activate: nothing to derive, and nothing
    // read.
    if !program.obligations.values().any(|r| !r.params.is_empty()) {
        return Vec::new();
    }
    let saturation: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let reads: Vec<Vec<(Rel, u32)>> = saturation
        .iter()
        .map(|p| monotone::reads_of(p, terms))
        .collect();
    let writes = pattern_writes(kb, terms, ast);
    let extendable = extendable_over(seed, &saturation, &reads, &writes, terms);
    let in_d = |r: Rel| monotone::in_set(&extendable, r, terms);
    // The rules a plan of root's reads `D⁺` through. Membership only: the
    // walk below is the registry's.
    let reading: rustc_hash::FxHashSet<Symbol> = saturation
        .iter()
        .zip(&reads)
        .filter(|(_, rs)| rs.iter().any(|&(r, _)| in_d(r)))
        .map(|(p, _)| p.rule)
        .collect();
    let mut out = Vec::new();
    for rule in program.rules.values() {
        let activated = !rule.params.is_empty() && in_d(Rel::pos(rule.name));
        if !(activated || reading.contains(&rule.name)) {
            continue;
        }
        let Some(assert) = rule.assert_.as_ref() else {
            continue;
        };
        for c in ast.conjuncts(NodeId(assert.expr.0)) {
            let Some(obligation) = ast
                .head_name(c)
                .and_then(|n| terms.syms.get(n))
                .and_then(|n| program.obligations.get(n))
            else {
                continue;
            };
            if obligation.params.is_empty()
                || ast.form_args(c).len() != obligation.params.len()
                || !holds_no_variable(ast, c)
            {
                continue;
            }
            out.push(Derivable {
                obligation: obligation.name,
                form: c,
                by: rule.name,
            });
        }
    }
    out
}

/// Does `node` spell a value: a name, or a form of names and such forms?
fn holds_no_variable(ast: &Ast, node: NodeId) -> bool {
    match ast.node(node) {
        Node::Atom(_) => true,
        Node::SForm { head, .. } => {
            matches!(ast.node(head), Node::Atom(_))
                && ast
                    .form_args(node)
                    .iter()
                    .all(|&a| holds_no_variable(ast, a))
        }
        _ => false,
    }
}

/// Every rule with parameters, with the relations its pattern's `:assert`
/// writes: what a plan a node compiles of it can conclude.
fn pattern_writes<'k>(kb: &'k Kb, terms: &Terms, ast: &Ast) -> Vec<(&'k Rule, Vec<Rel>)> {
    kb.program()
        .rules
        .values()
        .filter(|r| !r.params.is_empty())
        .map(|r| {
            let mut writes = Vec::new();
            if let Some(a) = r.assert_.as_ref() {
                heads(ast, terms, NodeId(a.expr.0), &mut writes);
            }
            (r, writes)
        })
        .collect()
}

/// `D⁺` over root's saturation plans and the rules with parameters, from
/// `seed`: the static check's `D` for [`relations`], the relations of what
/// the obligations rung proposes for [`derivable_activators`].
fn extendable_over(
    seed: impl IntoIterator<Item = Rel>,
    saturation: &[&Plan],
    reads: &[Vec<(Rel, u32)>],
    writes: &[(&Rule, Vec<Rel>)],
    terms: &Terms,
) -> rustc_hash::FxHashSet<Rel> {
    let plans: Vec<(&Plan, &[(Rel, u32)])> = saturation
        .iter()
        .zip(reads)
        .map(|(p, r)| (*p, r.as_slice()))
        .collect();
    let open: Vec<(Rel, &[Rel])> = writes
        .iter()
        .map(|(r, w)| (Rel::pos(r.name), w.as_slice()))
        .collect();
    monotone::through_activators(seed, &plans, &open, terms)
}

/// The relations an `:assert` writes, `(false)` included, one per conjunct
/// ([`ein_ir::Ast::conjuncts`], the conclusion read flat at any depth, as the
/// compiler reads it); a head written as a parameter names
/// nothing here.
fn heads(ast: &Ast, terms: &Terms, node: NodeId, out: &mut Vec<Rel>) {
    for c in ast.conjuncts(node) {
        let Node::SForm { head, .. } = ast.node(c) else {
            continue;
        };
        match ast.atom_name(head) {
            Some("not") => {
                for &a in ast.form_args(c) {
                    if let Some(name) = ast.head_name(a).and_then(|n| terms.syms.get(n)) {
                        out.push(Rel {
                            name,
                            negated: true,
                        });
                    } else {
                        out.push(Rel {
                            name: terms.kernel.not,
                            negated: true,
                        });
                    }
                }
            }
            Some(name) => {
                if let Some(name) = terms.syms.get(name) {
                    out.push(Rel::pos(name));
                }
            }
            None => {}
        }
    }
}
