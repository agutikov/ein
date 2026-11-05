//! *Forbid* — M1e2
//! [S1e2.2.5](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md):
//! a refutation may not rest on an `(absent P)` over a relation a commitment
//! can still extend.
//!
//! The user ruled it on 2026-11-02
//! ([S1e2.4.1 § 5](../../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)):
//! over such a relation, `(absent P)` ⟹ `(false)` or `(not …)` is an
//! obligation written as a refutation. It judges a state that has not yet
//! decided `P` as if `P` were false, so it says *contradiction* where the
//! truth is *not yet*, and `dead` stops being upward-closed (Q-M1e.9: a
//! commitment of `P` lifts it). A program that writes it is refused, and the
//! message names a form that says what was meant. Over a relation no
//! commitment can extend the same shape is closed-world reasoning, and stays
//! legal.
//!
//! ### What counts
//!
//! *Extendable* is F1's reading (b), at relation grain: the relation is in
//! `D⁺`, the static check's `D` ([`crate::monotone`]), the relations a
//! commitment can change, followed through the rules root has no plan of
//! (`monotone::through_activators`, M1e2 S1e2.2.17). A refutation is a
//! saturation plan whose `:assert` writes `(false)` or a stored negative. It
//! is refused when it rests on such an `absent`:
//!
//! | cause | the plan… |
//! |---|---|
//! | [`Cause::Direct`] | reads a relation in `D⁺` inside an odd number of `absent`s |
//! | [`Cause::Via`] | reads positively a relation in `N`: one a rule derives resting on such an `absent` (F2, *via*, ruled 2026-11-04) |
//! | [`Cause::Activator`] | is compiled from an activator in `N` (F2, *per activator*) |
//! | [`Cause::Parameter`] | is a rule with parameters whose activator a commitment can derive, and reads a parameter's relation inside an odd number of `absent`s: a node can compile it over a relation in `D⁺` (F3) |
//!
//! The check reads root's plans, and every **open** rule from its pattern: a
//! rule with parameters whose activator relation is in `D⁺`, since a node can
//! compile a plan of it that root never had. `N` is the relations a plan of
//! root's or an open rule writes when it reads, inside an odd number of
//! `absent`s, a relation in `D⁺` (an open rule's parameter's relation too,
//! whatever a node binds it to), closed under *a plan that reads `N`
//! positively, or is compiled from an activator in `N`, writes `N`*. On a
//! program the static check calls monotone it reads nothing: every cause
//! needs a site.
//!
//! **Until M1e2 S1e2.2.17** (W17) *extendable* was `D` and an open rule's
//! activator relation had to be in `D`, so a chain of activators past the
//! first plan root never compiled was not followed, and an open rule's own
//! `absent` tainted nothing it derived: a refutation whose activator chain
//! first appears at a node loaded with a dead hole
//! (`examples/ein-bugs/wrong-answers/forbid-chain-*.ein`). `D⁺` is `D` on a
//! program with no activator site.
//!
//! ### Where it runs
//!
//! In [`crate::region`]'s `prepare`, after root's fixpoint and the static
//! check, so every question about a commitment refuses alike: `ein solve` in
//! every mode, `ein test`, `ein oracle`, `ein render lattice`, and every
//! `(solve :expect)` check (F3). `ein saturate` asks about no commitment and
//! does not refuse. The error is [`crate::SolveError::Refused`], one line per
//! refusing rule, and a load error's exit code.
//!
//! ### Studying the shape
//!
//! [`studying`] lifts the refusal on the calling thread for the duration of a
//! closure. No CLI path calls it. It is how the tests and the censuses that
//! study the shape *forbid* names (the dead hole of `naf-upward-closure.ein`,
//! which the read-out still has a line for) reach it as a library-level
//! fixture.

use std::cell::Cell;
use std::sync::Arc;

use ein_core::{Kb, Rule, Symbol, Terms};
use ein_ir::{Ast, Node, NodeId};

use crate::monotone::{self, Monotonicity, Rel};
use crate::plan::Plan;

/// What a refused rule concludes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Concludes {
    False,
    Not,
}

impl Concludes {
    pub fn as_str(self) -> &'static str {
        match self {
            Concludes::False => "(false)",
            Concludes::Not => "(not …)",
        }
    }
}

/// Why a refutation is refused — the module docs' table.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cause {
    /// The plan reads `over`, in `D`, at odd `absent` depth `depth`.
    Direct { over: Rel, depth: u32 },
    /// The plan reads `premise` positively, and rule `by` derives it resting
    /// on an `absent` over `over`.
    Via {
        premise: Rel,
        by: Symbol,
        over: Over,
    },
    /// The plan's activator is in `N`: rule `by` derives it resting on an
    /// `absent` over `over`.
    Activator { by: Symbol, over: Over },
    /// A rule with parameters, whose activator a commitment can derive, reads
    /// the relation its parameter `param` names at odd `absent` depth.
    Parameter { param: Symbol },
}

/// What the `absent` a fact of `N` rests on reads: a relation in `D⁺`, or
/// the relation an open rule's parameter names, which a node binds — read
/// conservatively, as [`Cause::Parameter`] is (M1e2 S1e2.2.17).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Over {
    Rel(Rel),
    Param(Symbol),
}

impl Over {
    /// `(absent …) over R, a relation a commitment can still extend`, or, for
    /// a parameter, what [`Cause::Parameter`]'s line has always said.
    fn text(self, terms: &Terms) -> String {
        match self {
            Over::Rel(r) => format!("(absent …) over {}, {EXTENDABLE}", r.label(terms)),
            Over::Param(p) => format!(
                "(absent …) over the relation its parameter ?{} names, and a commitment can \
                 derive its activator, so a node can compile it over {EXTENDABLE}",
                terms.sym(p)
            ),
        }
    }

    /// What the line says the state has not yet decided.
    fn undecided(self, terms: &Terms) -> String {
        match self {
            Over::Rel(r) => r.label(terms),
            Over::Param(_) => "that relation".to_string(),
        }
    }
}

const EXTENDABLE: &str = "a relation a commitment can still extend";

/// One refused refutation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Refusal {
    pub rule: Symbol,
    /// The activator's arguments, as [`Plan::activator_args`] has them; empty
    /// for a rule without parameters, and for [`Cause::Parameter`], which is
    /// about the rule rather than one plan.
    pub activator: Box<[Symbol]>,
    pub concludes: Concludes,
    pub cause: Cause,
}

impl Refusal {
    /// One line: the rule, what it rests on, and the forms that say what was
    /// meant (F4).
    pub fn text(&self, terms: &Terms) -> String {
        let rule = terms.sym(self.rule);
        let act = if self.activator.is_empty() {
            String::new()
        } else {
            let a: Vec<std::borrow::Cow<'_, str>> = self
                .activator
                .iter()
                .map(|&s| ein_core::text::symbol(terms.sym(s)))
                .collect();
            format!(" [activator {}]", a.join(" "))
        };
        let concl = self.concludes.as_str();
        let (from, rel) = match &self.cause {
            Cause::Direct { over, .. } => (Over::Rel(*over).text(terms), over.label(terms)),
            Cause::Via { premise, by, over } => (
                format!(
                    "{}, which rule '{}' derives resting on {}",
                    premise.label(terms),
                    terms.sym(*by),
                    over.text(terms)
                ),
                over.undecided(terms),
            ),
            Cause::Activator { by, over } => (
                format!(
                    "its activator, which rule '{}' derives resting on {}",
                    terms.sym(*by),
                    over.text(terms)
                ),
                over.undecided(terms),
            ),
            Cause::Parameter { param } => (
                Over::Param(*param).text(terms),
                Over::Param(*param).undecided(terms),
            ),
        };
        // A stored negative's relation is closed by closing its positive.
        let close = match &self.cause {
            Cause::Direct { over, .. }
            | Cause::Via {
                over: Over::Rel(over),
                ..
            }
            | Cause::Activator {
                over: Over::Rel(over),
                ..
            } if over.name != terms.kernel.not => {
                format!("(solve :no-hypothesis ({}))", terms.sym(over.name))
            }
            _ => "(solve :no-hypothesis (…))".to_string(),
        };
        format!(
            "rule '{rule}'{act}: concludes {concl} from {from} — so it calls a state that has not yet decided {rel} a \
             contradiction, and a commitment would lift it (dead is not upward-closed under \
             absent; forbidden since M1e2 S1e2.2.5). Write what it means: close the \
             relation, {close}, if no commitment is meant to extend it; state a requirement \
             as a decision or an obligation, a yes/no attribute that must be decided \
             (functional + total-owed) or (open ?R) with hrules that propose it; or conclude \
             from a stored negative, (not …), as std.algebra's total does"
        )
    }
}

thread_local! {
    static STUDYING: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with the refusal lifted on this thread — the module docs'
/// *Studying the shape*. Nested calls and a panic inside `f` leave the flag as
/// they found it.
pub fn studying<T>(f: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            STUDYING.with(|s| s.set(self.0));
        }
    }
    let _restore = Restore(STUDYING.with(|s| s.replace(true)));
    f()
}

/// Is the refusal lifted on this thread?
pub fn is_studying() -> bool {
    STUDYING.with(Cell::get)
}

/// Every refused refutation of a program, sorted by rule text and then by
/// activator. `plans` are root's, `m` the static check over them.
pub fn check(
    kb: &Kb,
    terms: &Terms,
    ast: &Ast,
    plans: &[Arc<Plan>],
    m: &Monotonicity,
) -> Vec<Refusal> {
    // Every cause needs a site: a monotone program pays one test.
    if m.is_monotone() {
        return Vec::new();
    }
    let program = kb.program();
    let saturation: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    // A saturation plan has an activator exactly when its rule has
    // parameters: `activators_for` hands a rule with parameters only facts
    // of its arity, each binding every parameter's position. Not
    // `!p.activator_args.is_empty()`, which until M1e2 S1e2.2.16 this was:
    // that list keeps the activator's symbol arguments only, so an activator
    // whose every argument is an int or a nested fact read as none, its plan's
    // cause was never looked for, and a refutation resting on it through
    // `absent` loaded (W16, `examples/ein-bugs/activator-all-int.ein`).
    let has_activator = |p: &Plan| {
        program
            .rules
            .get(p.rule)
            .is_some_and(|r| !r.params.is_empty())
    };
    let reads: Vec<Vec<(Rel, u32)>> = saturation
        .iter()
        .map(|p| monotone::reads_of(p, terms))
        .collect();
    // Every rule with parameters, read from its pattern, and `D⁺`, the
    // relations a commitment can extend: `D` followed through the plans a
    // node can compile that root never had (M1e2 S1e2.2.17, W17). Until then
    // this read `D` itself, whose closure follows root's plans only.
    let bodies = bodies_of(kb, terms, ast);
    let extendable = extendable_over(m, &saturation, &reads, &bodies, terms);
    let in_d = |r: Rel| monotone::in_set(&extendable, r, terms);
    let odd_d = |rs: &[(Rel, u32)]| -> Option<(Rel, u32)> {
        let mut v: Vec<(Rel, u32)> = rs
            .iter()
            .copied()
            .filter(|&(r, d)| d % 2 == 1 && in_d(r))
            .collect();
        v.sort_by(|a, b| a.0.label(terms).cmp(&b.0.label(terms)).then(a.1.cmp(&b.1)));
        v.into_iter().next()
    };
    // The open rules: those with parameters a node can compile a plan of
    // that root never had, their activator relation in `D⁺`. Until M1e2
    // S1e2.2.17, in `D`, so a chain of activators past the first was not
    // followed (`forbid-chain-1.ein`'s `r2`).
    let open_rules: Vec<(&Rule, &Body)> = bodies
        .iter()
        .filter(|(r, _)| in_d(pos(r.name)))
        .map(|(r, b)| (*r, b))
        .collect();

    // N, each relation with the rule that derives it and the `absent` it
    // rests on: seeded by every plan of root's, and since M1e2 S1e2.2.17 by
    // every open rule, that reads inside an odd number of `absent`s a
    // relation in `D⁺` — or, an open rule, its parameter's relation, which a
    // node binds (`forbid-chain-node-absent.ein`, `-param-absent.ein`).
    let mut n = Tainted::default();
    for (p, rs) in saturation.iter().zip(&reads) {
        if let Some((over, _)) = odd_d(rs) {
            for w in monotone::writes_of(p, terms) {
                if w.name != terms.kernel.r#false {
                    n.insert(w, p.rule, Over::Rel(over));
                }
            }
        }
    }
    for (rule, body) in &open_rules {
        let mut odd: Vec<(Rel, u32)> = body
            .reads
            .iter()
            .filter_map(|&(r, d)| match r {
                Read::Rel(r) if d % 2 == 1 && in_d(r) => Some((r, d)),
                _ => None,
            })
            .collect();
        odd.sort_by(|a, b| a.0.label(terms).cmp(&b.0.label(terms)).then(a.1.cmp(&b.1)));
        let over = odd.first().map(|&(r, _)| Over::Rel(r)).or_else(|| {
            body.reads.iter().find_map(|&(r, d)| match r {
                Read::Param(p) if d % 2 == 1 => Some(Over::Param(p)),
                _ => None,
            })
        });
        if let Some(over) = over {
            for &w in &body.writes {
                if w.name != terms.kernel.r#false {
                    n.insert(w, rule.name, over);
                }
            }
        }
    }
    loop {
        let mut grew = false;
        for (p, rs) in saturation.iter().zip(&reads) {
            let from = rs
                .iter()
                .filter(|&&(_, d)| d % 2 == 0)
                .find_map(|&(r, _)| n.get(r, terms))
                .or_else(|| {
                    has_activator(p)
                        .then(|| n.get(pos(p.rule), terms))
                        .flatten()
                });
            if let Some((_, over)) = from {
                for w in monotone::writes_of(p, terms) {
                    if w.name != terms.kernel.r#false {
                        grew |= n.insert(w, p.rule, over);
                    }
                }
            }
        }
        for (rule, body) in &open_rules {
            let from = body
                .reads
                .iter()
                .filter(|&&(_, d)| d % 2 == 0)
                .find_map(|&(r, _)| match r {
                    Read::Rel(r) => n.get(r, terms),
                    Read::Param(_) => None,
                })
                .or_else(|| n.get(pos(rule.name), terms));
            if let Some((_, over)) = from {
                for &w in &body.writes {
                    if w.name != terms.kernel.r#false {
                        grew |= n.insert(w, rule.name, over);
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }

    let mut out: Vec<Refusal> = Vec::new();
    for (p, rs) in saturation.iter().zip(&reads) {
        let Some(concludes) = refutes(&monotone::writes_of(p, terms), terms) else {
            continue;
        };
        let cause = if let Some((over, depth)) = odd_d(rs) {
            Cause::Direct { over, depth }
        } else if let Some((premise, (by, over))) = rs
            .iter()
            .filter(|&&(_, d)| d % 2 == 0)
            .find_map(|&(r, _)| n.get(r, terms).map(|o| (r, o)))
        {
            Cause::Via { premise, by, over }
        } else if let Some((by, over)) = has_activator(p)
            .then(|| n.get(pos(p.rule), terms))
            .flatten()
        {
            Cause::Activator { by, over }
        } else {
            continue;
        };
        out.push(Refusal {
            rule: p.rule,
            activator: p.activator_args.clone(),
            concludes,
            cause,
        });
    }
    for (rule, body) in &open_rules {
        if out.iter().any(|r| r.rule == rule.name) {
            continue;
        }
        let Some(concludes) = refutes(&body.writes, terms) else {
            continue;
        };
        let param = body.reads.iter().find_map(|&(r, d)| match r {
            Read::Param(p) if d % 2 == 1 => Some(p),
            _ => None,
        });
        let direct = body.reads.iter().find_map(|&(r, d)| match r {
            Read::Rel(r) if d % 2 == 1 && in_d(r) => Some((r, d)),
            _ => None,
        });
        let via = body.reads.iter().find_map(|&(r, d)| match r {
            Read::Rel(r) if d % 2 == 0 => n.get(r, terms).map(|o| (r, o)),
            _ => None,
        });
        let cause = if let Some((over, depth)) = direct {
            Cause::Direct { over, depth }
        } else if let Some(param) = param {
            Cause::Parameter { param }
        } else if let Some((premise, (by, over))) = via {
            Cause::Via { premise, by, over }
        } else if let Some((by, over)) = n.get(pos(rule.name), terms) {
            Cause::Activator { by, over }
        } else {
            continue;
        };
        out.push(Refusal {
            rule: rule.name,
            activator: Box::new([]),
            concludes,
            cause,
        });
    }
    out.sort_by(|a, b| {
        terms
            .syms
            .cmp_text(a.rule, b.rule)
            .then_with(|| cmp_args(terms, &a.activator, &b.activator))
    });
    out.dedup();
    out
}

/// The relations *forbid* calls extendable, `D⁺`, sorted by label: the static
/// check's `D` followed through the rules root has no plan of
/// (`monotone::through_activators`, M1e2 S1e2.2.17). `plans` are root's, `m`
/// the static check over them. For a reader that studies the shape *forbid*
/// names, as `ein-infer/tests/region.rs`' test of F21's D6 (b) does; it is
/// `D` on a program with no activator site.
pub fn extendable(
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
    let bodies = bodies_of(kb, terms, ast);
    // determinism-ok: sorted by label on the next line.
    let mut out: Vec<Rel> = extendable_over(m, &saturation, &reads, &bodies, terms)
        .into_iter()
        .collect();
    out.sort_by_key(|r| r.label(terms));
    out
}

/// Every rule with parameters, with its pattern read.
fn bodies_of<'k>(kb: &'k Kb, terms: &Terms, ast: &Ast) -> Vec<(&'k Rule, Body)> {
    kb.program()
        .rules
        .values()
        .filter(|r| !r.params.is_empty())
        .map(|r| (r, Body::of(ast, terms, r)))
        .collect()
}

/// `D⁺` over root's saturation plans and the rules with parameters.
fn extendable_over(
    m: &Monotonicity,
    saturation: &[&Plan],
    reads: &[Vec<(Rel, u32)>],
    bodies: &[(&Rule, Body)],
    terms: &Terms,
) -> rustc_hash::FxHashSet<Rel> {
    let plans: Vec<(&Plan, &[(Rel, u32)])> = saturation
        .iter()
        .zip(reads)
        .map(|(p, r)| (*p, r.as_slice()))
        .collect();
    let open: Vec<(Rel, &[Rel])> = bodies
        .iter()
        .map(|(r, b)| (pos(r.name), b.writes.as_slice()))
        .collect();
    monotone::through_activators(m, &plans, &open, terms)
}

/// The refusals as [`crate::SolveError::Refused`]'s message: one line each.
pub fn message(refusals: &[Refusal], terms: &Terms) -> String {
    refusals
        .iter()
        .map(|r| r.text(terms))
        .collect::<Vec<_>>()
        .join("\n")
}

fn pos(name: Symbol) -> Rel {
    Rel {
        name,
        negated: false,
    }
}

fn refutes(writes: &[Rel], terms: &Terms) -> Option<Concludes> {
    if writes
        .iter()
        .any(|r| !r.negated && r.name == terms.kernel.r#false)
    {
        Some(Concludes::False)
    } else if writes.iter().any(|r| r.negated) {
        Some(Concludes::Not)
    } else {
        None
    }
}

/// `N`, in insertion order — the first derivation found is the one a message
/// names.
#[derive(Default)]
struct Tainted(Vec<(Rel, Symbol, Over)>);

impl Tainted {
    /// `r ∈ N`, the wildcard negative read both ways, as
    /// [`Monotonicity::changes`] reads `D`.
    fn get(&self, r: Rel, terms: &Terms) -> Option<(Symbol, Over)> {
        let any_neg = |x: Rel| x.negated && x.name == terms.kernel.not;
        self.0
            .iter()
            .find(|&&(x, _, _)| x == r || (r.negated && any_neg(x)) || (any_neg(r) && x.negated))
            .map(|&(_, by, over)| (by, over))
    }

    fn insert(&mut self, r: Rel, by: Symbol, over: Over) -> bool {
        if self.0.iter().any(|&(x, _, _)| x == r) {
            return false;
        }
        self.0.push((r, by, over));
        true
    }
}

/// A read in a rule's pattern: a relation, or the relation a parameter names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Read {
    Rel(Rel),
    Param(Symbol),
}

/// A rule's pattern, read as [`monotone::reads_of`] reads a plan: every
/// relation with its `absent` depth, and every relation its `:assert` writes.
struct Body {
    reads: Vec<(Read, u32)>,
    writes: Vec<Rel>,
}

impl Body {
    fn of(ast: &Ast, terms: &Terms, rule: &Rule) -> Body {
        let mut reads = Vec::new();
        if let Some(m) = rule.match_.as_ref() {
            walk(ast, terms, rule, NodeId(m.expr.0), 0, &mut reads);
        }
        let mut writes = Vec::new();
        if let Some(a) = rule.assert_.as_ref() {
            heads(ast, terms, NodeId(a.expr.0), &mut writes);
        }
        Body { reads, writes }
    }
}

/// The relation a head node names: `Some(Read)`, or `None` for a head that
/// names nothing a commitment can reach (a wildcard, an unknown name).
fn head_read(ast: &Ast, terms: &Terms, rule: &Rule, head: NodeId, negated: bool) -> Option<Read> {
    match ast.node(head) {
        Node::Atom(s) => terms
            .syms
            .get(ast.sym(s))
            .map(|name| Read::Rel(Rel { name, negated })),
        Node::Var(s) => {
            let v = terms.syms.get(ast.sym(s))?;
            rule.params.contains(&v).then_some(Read::Param(v))
        }
        _ => None,
    }
}

fn walk(
    ast: &Ast,
    terms: &Terms,
    rule: &Rule,
    node: NodeId,
    depth: u32,
    out: &mut Vec<(Read, u32)>,
) {
    let Node::SForm { head, .. } = ast.node(node) else {
        return;
    };
    let args = ast.form_args(node);
    match ast.atom_name(head) {
        Some("and") | Some("or") => {
            for &a in args {
                walk(ast, terms, rule, a, depth, out);
            }
        }
        Some("absent") => {
            for &a in args {
                walk(ast, terms, rule, a, depth + 1, out);
            }
        }
        Some("not") => {
            for &a in args {
                if let Node::SForm { head: inner, .. } = ast.node(a)
                    && let Some(r) = head_read(ast, terms, rule, inner, true)
                {
                    out.push((r, depth));
                }
            }
        }
        Some(name) if crate::predicates::is_predicate(name) => {}
        _ => {
            if let Some(r) = head_read(ast, terms, rule, head, false) {
                out.push((r, depth));
            }
        }
    }
}

/// The relations an `:assert` writes, `(false)` included; a head written as
/// a parameter names nothing here.
fn heads(ast: &Ast, terms: &Terms, node: NodeId, out: &mut Vec<Rel>) {
    let Node::SForm { head, .. } = ast.node(node) else {
        return;
    };
    match ast.atom_name(head) {
        Some("and") => {
            for &a in ast.form_args(node) {
                heads(ast, terms, a, out);
            }
        }
        Some("not") => {
            for &a in ast.form_args(node) {
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
                out.push(pos(name));
            }
        }
        None => {}
    }
}

/// Element-wise by text, a prefix first — `monotone`'s order for the same
/// records.
fn cmp_args(terms: &Terms, a: &[Symbol], b: &[Symbol]) -> std::cmp::Ordering {
    for (x, y) in a.iter().zip(b.iter()) {
        let o = terms.syms.cmp_text(*x, *y);
        if o != std::cmp::Ordering::Equal {
            return o;
        }
    }
    a.len().cmp(&b.len())
}
