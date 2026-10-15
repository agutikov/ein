//! Is a program's hypothesis lattice monotone? — M1e2
//! [S1e2.1a.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//!
//! The answer to a program is the region `R` of its hypothesis lattice, the
//! commitments that are alive and owe nothing
//! ([Q-M1e2.7](../../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).
//! Two lines describe `R` — its minimal and its maximal elements — only when
//! `R` has no **holes**: no node outside `R` between two nodes inside it. A
//! hole needs a commitment that grows and takes a conclusion away, or grows
//! and adds a debt. This module says, from the compiled plans alone, whether a
//! program can do either.
//!
//! ### The check
//!
//! Let `H` be the relations of the root candidate list — what a commitment
//! can add — and `D` the relations a commitment can **change**: `H`, closed
//! under *a saturation plan that reads a relation in `D` asserts into `D`*. A
//! stored negative `(not (R …))` is its own pseudo-relation `¬R`, because what
//! the generator proposes is `R`, and a `¬R` changes only if a rule derives it.
//!
//! Every relation a plan reads, at every `absent` depth, is then one of:
//!
//! | class | condition | growth of the relation can… |
//! |---|---|---|
//! | [`ReadClass::Static`] | not in `D` | nothing: it is complete at root and the same at every node |
//! | [`ReadClass::DynamicMonotone`] | in `D`, monotone polarity | only add a conclusion, or only pay a debt |
//! | [`ReadClass::DynamicNonMonotone`] | in `D`, non-monotone polarity | take a conclusion away, or add a debt |
//!
//! The polarity is the depth's parity, read differently by the two kinds of
//! rule. In a **saturation** rule a match is a conclusion, and a read at odd
//! depth (inside one `absent`) is non-monotone: W3's `bad`, W8's `derive-p`.
//! In an **obligation** rule a match is a *debt* — the rule is the negation of
//! its witness — and a read at even depth is non-monotone: its **trigger**, the
//! positive steps that say who owes. `total-owed`'s witness `absent` is odd
//! depth there, and a commitment that grows it can only pay.
//!
//! A [`Site`] is a non-monotone read, or a rule whose **activator** relation
//! is in `D`: a commitment can then compile plans root never had, which this
//! check cannot see.
//!
//! ### The theorem
//!
//! **A program with no site has a monotone lattice.** Every relation in `D` is
//! then derived only through positive reads of `D` and reads of static
//! relations, which have the same extension at every node; so each node's
//! fixpoint contains its subsets' (`dead` is upward-closed, `(false)` being a
//! fact), and every obligation's set of matches is fixed while its witnesses
//! only grow (*discharged* is upward-closed). Both closures together leave `R`
//! no holes, so its two lines describe it. That is a **sufficient** condition.
//! A site can be harmless in practice, and [`crate::region`]'s cube is what
//! says whether one was: it counts the violations it observes, and a
//! `Monotone` verdict beside an observed violation is a defect in this module.
//!
//! The probe that showed a hole needs no `absent` is
//! `examples/lattice/04_obligation_trigger_hole.ein`: `total-owed meal is-a`,
//! and a rule that derives `(is-a ?g Diner)` from a hypothesised seat.
//!
//! ### What it does not touch
//!
//! It reads compiled plans and the program's registries and nothing else — no
//! interning, no event, no counter — so a solve can ask it without moving.
//! *What* the non-monotone case means is
//! [P1e2.1b](../../../../plans/m1e2_correctness/p1e2.1b_the_non_monotone_lattice/README.md)'s,
//! and [`crate::solve::solve`]'s `non_monotone` is the stub it replaces.

use std::sync::Arc;

use ein_core::{Kb, Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::compile::SharedMemo;
use crate::plan::{Plan, PlanId, Slot, Span, Step};

/// A relation as the check sees it: `R`, or the stored negatives of `R`.
///
/// `name == kernel.not` with `negated` is the **wildcard** negative — a
/// `(not ?x)` pattern, which reads or writes every `¬R` at once.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rel {
    pub name: Symbol,
    pub negated: bool,
}

impl Rel {
    fn pos(name: Symbol) -> Rel {
        Rel {
            name,
            negated: false,
        }
    }

    fn neg(name: Symbol) -> Rel {
        Rel {
            name,
            negated: true,
        }
    }

    fn is_any_negative(self, terms: &Terms) -> bool {
        self.negated && self.name == terms.kernel.not
    }

    /// `R`, `(not R)`, or `(not ?)` for the wildcard.
    pub fn label(self, terms: &Terms) -> String {
        if self.is_any_negative(terms) {
            "(not ?)".to_string()
        } else if self.negated {
            format!("(not {})", terms.sym(self.name))
        } else {
            terms.sym(self.name).to_string()
        }
    }
}

/// What kind of rule a read belongs to — its polarity depends on it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanKind {
    Saturation,
    Obligation,
}

impl PlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PlanKind::Saturation => "saturation",
            PlanKind::Obligation => "obligation",
        }
    }
}

/// One read's class — the module docs' table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReadClass {
    Static,
    DynamicMonotone,
    DynamicNonMonotone,
}

impl ReadClass {
    pub fn as_str(self) -> &'static str {
        match self {
            ReadClass::Static => "static",
            ReadClass::DynamicMonotone => "dynamic-monotone",
            ReadClass::DynamicNonMonotone => "dynamic-non-monotone",
        }
    }

    fn of(kind: PlanKind, depth: u32, dynamic: bool) -> ReadClass {
        if !dynamic {
            return ReadClass::Static;
        }
        let bad = match kind {
            PlanKind::Saturation => !depth.is_multiple_of(2),
            PlanKind::Obligation => depth.is_multiple_of(2),
        };
        if bad {
            ReadClass::DynamicNonMonotone
        } else {
            ReadClass::DynamicMonotone
        }
    }
}

/// Why a [`Site`] is one.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SiteKind {
    /// A saturation rule reads a relation in `D` inside an odd number of
    /// `absent`s: growing it can take the rule's conclusion away.
    Absent,
    /// An obligation rule reads a relation in `D` at even depth — its trigger:
    /// growing it can add a debt.
    ObligationTrigger,
    /// The rule's activator relation is in `D`, so a node can compile a plan
    /// of this rule that root never had, and nothing here reads it.
    Activator,
}

impl SiteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SiteKind::Absent => "absent",
            SiteKind::ObligationTrigger => "obligation-trigger",
            SiteKind::Activator => "activator",
        }
    }
}

/// A read that can make the lattice non-monotone.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Site {
    pub rule: Symbol,
    /// The activator's arguments, as [`Plan::activator_args`] has them; empty
    /// for an activator site, which is about the rule rather than one plan.
    pub activator: Box<[Symbol]>,
    pub kind: SiteKind,
    pub relation: Rel,
    /// The `absent` depth of the read: 0 for a top-level premise.
    pub depth: u32,
}

impl Site {
    /// `rule [activator a b]: kind reads R at depth d` — one line, for a
    /// report.
    pub fn render(&self, terms: &Terms) -> String {
        let act = if self.activator.is_empty() {
            String::new()
        } else {
            let names: Vec<&str> = self.activator.iter().map(|&a| terms.sym(a)).collect();
            format!(" [activator {}]", names.join(" "))
        };
        match self.kind {
            SiteKind::Activator => format!(
                "{}: its activator relation is one a commitment can change",
                terms.sym(self.rule)
            ),
            _ => format!(
                "{}{act}: {} reads {} at absent depth {}",
                terms.sym(self.rule),
                self.kind.as_str(),
                self.relation.label(terms),
                self.depth
            ),
        }
    }
}

/// Every read of a relation inside an `absent`, classified — the user's
/// question of 2026-10-15, *does any `absent` cause non-monotonicity, or only
/// some?*, answered per program.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AbsentRead {
    pub rule: Symbol,
    pub activator: Box<[Symbol]>,
    pub kind: PlanKind,
    pub relation: Rel,
    pub depth: u32,
    pub class: ReadClass,
}

/// How many reads of each class, over every plan the check read.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ReadCounts {
    pub static_: usize,
    pub dynamic_monotone: usize,
    pub dynamic_non_monotone: usize,
}

impl ReadCounts {
    fn bump(&mut self, class: ReadClass) {
        match class {
            ReadClass::Static => self.static_ += 1,
            ReadClass::DynamicMonotone => self.dynamic_monotone += 1,
            ReadClass::DynamicNonMonotone => self.dynamic_non_monotone += 1,
        }
    }
}

/// The check's answer for one program.
#[derive(Clone, Default, Debug)]
pub struct Monotonicity {
    /// `H`, sorted by text.
    pub hypothesised: Vec<Symbol>,
    /// `D`, sorted by label.
    pub dynamic: Vec<Rel>,
    /// How many plans were read, of each kind.
    pub plans: (usize, usize),
    /// Every read, by class.
    pub reads: ReadCounts,
    /// The reads inside an `absent` (depth ≥ 1), by class.
    pub absent_reads: ReadCounts,
    /// Every read inside an `absent`, in site order.
    pub absents: Vec<AbsentRead>,
    /// The non-monotone reads and the activator sites, sorted and distinct.
    /// Empty is the theorem's premise.
    pub sites: Vec<Site>,
}

impl Monotonicity {
    /// No site: the module docs' theorem applies.
    pub fn is_monotone(&self) -> bool {
        self.sites.is_empty()
    }
}

/// Every plan the memo holds — what a run compiled.
///
/// The memo is append-only and keyed by `(rule, activator)`, so this is one
/// plan per pair the run has seen, in compile order.
pub fn plans_of(memo: &SharedMemo) -> Vec<Arc<Plan>> {
    let m = memo.lock().expect("no compiler panicked");
    (0..m.len()).map(|i| m.get_arc(PlanId(i as u32))).collect()
}

/// Classify `plans` against `hypothesised` (`H`).
///
/// `kb` supplies the program's registries — which rules are saturation rules,
/// which are obligations, which have activators. A plan of an `hrule`, or of
/// anything else the memo holds, is not read: an hrule proposes, and what it
/// proposes is already `H`.
pub fn classify(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    hypothesised: &FxHashSet<Symbol>,
) -> Monotonicity {
    let program = kb.program();
    let kinded: Vec<(PlanKind, &Plan)> = plans
        .iter()
        .filter_map(|p| {
            if program.rules.contains(p.rule) {
                Some((PlanKind::Saturation, &**p))
            } else if program.obligations.contains(p.rule) {
                Some((PlanKind::Obligation, &**p))
            } else {
                None
            }
        })
        .collect();
    let reads: Vec<Vec<(Rel, u32)>> = kinded.iter().map(|(_, p)| reads_of(p, terms)).collect();

    // D — `H`, closed under "a saturation plan that reads D asserts into D".
    // determinism-ok: a set built from a set; nothing reads its order.
    let mut dynamic: FxHashSet<Rel> = hypothesised.iter().map(|&r| Rel::pos(r)).collect();
    loop {
        let mut grew = false;
        for ((kind, plan), reads) in kinded.iter().zip(&reads) {
            if *kind != PlanKind::Saturation
                || !reads.iter().any(|&(r, _)| in_set(&dynamic, r, terms))
            {
                continue;
            }
            for w in writes_of(plan, terms) {
                grew |= dynamic.insert(w);
            }
        }
        if !grew {
            break;
        }
    }

    let mut out = Monotonicity {
        plans: (
            kinded
                .iter()
                .filter(|(k, _)| *k == PlanKind::Saturation)
                .count(),
            kinded
                .iter()
                .filter(|(k, _)| *k == PlanKind::Obligation)
                .count(),
        ),
        ..Monotonicity::default()
    };
    for ((kind, plan), reads) in kinded.iter().zip(&reads) {
        for &(rel, depth) in reads {
            let class = ReadClass::of(*kind, depth, in_set(&dynamic, rel, terms));
            out.reads.bump(class);
            if depth >= 1 {
                out.absent_reads.bump(class);
                out.absents.push(AbsentRead {
                    rule: plan.rule,
                    activator: plan.activator_args.clone(),
                    kind: *kind,
                    relation: rel,
                    depth,
                    class,
                });
            }
            if class == ReadClass::DynamicNonMonotone {
                out.sites.push(Site {
                    rule: plan.rule,
                    activator: plan.activator_args.clone(),
                    kind: match kind {
                        PlanKind::Saturation => SiteKind::Absent,
                        PlanKind::Obligation => SiteKind::ObligationTrigger,
                    },
                    relation: rel,
                    depth,
                });
            }
        }
    }
    // Activator sites are about **rules**, not plans: the plan a commitment
    // would compile is exactly the one the memo does not hold.
    for rule in program.rules.values().chain(program.obligations.values()) {
        if !rule.params.is_empty() && in_set(&dynamic, Rel::pos(rule.name), terms) {
            out.sites.push(Site {
                rule: rule.name,
                activator: Box::new([]),
                kind: SiteKind::Activator,
                relation: Rel::pos(rule.name),
                depth: 0,
            });
        }
    }

    let order = |a: &Site, b: &Site| {
        terms
            .syms
            .cmp_text(a.rule, b.rule)
            .then_with(|| cmp_args(terms, &a.activator, &b.activator))
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.relation.label(terms).cmp(&b.relation.label(terms)))
            .then_with(|| a.depth.cmp(&b.depth))
    };
    out.sites.sort_by(order);
    out.sites.dedup();
    out.absents.sort_by(|a, b| {
        terms
            .syms
            .cmp_text(a.rule, b.rule)
            .then_with(|| cmp_args(terms, &a.activator, &b.activator))
            .then_with(|| a.relation.label(terms).cmp(&b.relation.label(terms)))
            .then_with(|| a.depth.cmp(&b.depth))
    });
    out.absents.dedup();

    // determinism-ok: sorted by text on the next line, before anyone reads it.
    let mut h: Vec<Symbol> = hypothesised.iter().copied().collect();
    h.sort_by(|&a, &b| terms.syms.cmp_text(a, b));
    out.hypothesised = h;
    // determinism-ok: sorted by label on the next line.
    let mut d: Vec<Rel> = dynamic.into_iter().collect();
    d.sort_by_key(|r| r.label(terms));
    out.dynamic = d;
    out
}

/// Is `r` in `set`, the wildcard negative read both ways?
fn in_set(set: &FxHashSet<Rel>, r: Rel, terms: &Terms) -> bool {
    if r.is_any_negative(terms) {
        // determinism-ok: `any` — the answer is the same in every order.
        return set.iter().any(|x| x.negated);
    }
    set.contains(&r) || (r.negated && set.contains(&Rel::neg(terms.kernel.not)))
}

/// Every relation `plan` reads, with the `absent` depth it is read at.
///
/// Top-level `absent`s are the disjunct's [`crate::plan::NafGuard`]s (depth
/// 1); a nested one is a [`Step::Absent`] inside a guard (depth + 1), which is
/// what `forall` desugars to. Built-in predicates read no relation.
pub fn reads_of(plan: &Plan, terms: &Terms) -> Vec<(Rel, u32)> {
    fn walk(plan: &Plan, terms: &Terms, span: Span, depth: u32, out: &mut Vec<(Rel, u32)>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) if r.rel == terms.kernel.not => {
                    let before = out.len();
                    for slot in plan.slots(r.slots) {
                        if let Slot::Nested { rel, .. } = slot {
                            out.push((Rel::neg(*rel), depth));
                        }
                    }
                    if out.len() == before {
                        out.push((Rel::neg(terms.kernel.not), depth));
                    }
                }
                Step::Rel(r) => out.push((Rel::pos(r.rel), depth)),
                Step::Absent { sub } => walk(plan, terms, *sub, depth + 1, out),
                Step::Guard { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for d in plan.disjuncts.iter() {
        walk(plan, terms, d.steps, 0, &mut out);
        for g in plan.guards(d.guards) {
            walk(plan, terms, g.sub, 1, &mut out);
        }
    }
    out
}

/// Every relation `plan`'s `:assert` writes, `(false)` included — every
/// conclusion, not only the first (A13 multi-assert).
pub fn writes_of(plan: &Plan, terms: &Terms) -> Vec<Rel> {
    let mut out = Vec::new();
    for slot in plan.asserts.iter() {
        let Slot::Nested { rel, slots } = slot else {
            continue;
        };
        if *rel == terms.kernel.not {
            let before = out.len();
            for s in plan.slots(*slots) {
                if let Slot::Nested { rel, .. } = s {
                    out.push(Rel::neg(*rel));
                }
            }
            if out.len() == before {
                out.push(Rel::neg(terms.kernel.not));
            }
        } else {
            out.push(Rel::pos(*rel));
        }
    }
    out
}

/// Element-wise by text, a prefix first — `naf_deps`'s order for the same
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
