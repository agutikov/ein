//! Is a program monotone? — M1e2
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
//! ### Which property it is about
//!
//! In the words of `docs/kernel/inference/monotonicity.md` § 7. The lattice,
//! the commitments over the candidates ordered by ⊆, is the domain, and it is
//! neither monotone nor not. What can be monotone are the functions on its
//! nodes: the **state**, `C ⊆ C′ ⇒ state(C) ⊆ state(C′)`, and the **status**,
//! `dead` and *discharged* upward-closed. A third and stronger property is the
//! program's: its consequence operator is monotone on every KB. A site says
//! the operator is not monotone over `D`, so the verdict approximates the
//! third property, and the theorem below gives the first two from it.
//!
//! **The verdict is relative to root.** A read of a relation outside `D` is
//! static because root's fixpoint completes it and a fork never re-derives it,
//! so *Monotone* says the operator is monotone on root's fixpoint extended by
//! commitments: on nodes built by fork from root, which is how every route
//! builds one. A node built from scratch re-derives root inside itself, and
//! there an equal-priority race over a static read can leave a stale fact on a
//! program this check calls monotone (`examples/lattice/11_fifo_gap_static.ein`,
//! M1e2 S1e2.1b.7).
//!
//! ### The theorem
//!
//! **A program with no site has a monotone state and a monotone status**, on
//! nodes built by fork from root. Every relation in `D` is then derived only
//! through positive reads of `D` and reads of static relations, which have the
//! same extension at every node; so each node's fixpoint contains its subsets'
//! (`dead` is upward-closed, `(false)` being a fact), and every obligation's
//! set of matches is fixed while its witnesses only grow (*discharged* is
//! upward-closed). Both closures together leave `R` no holes, so its two lines
//! describe it. That is a **sufficient** condition.
//! A site can be harmless in practice, and [`crate::region`]'s cube is what
//! says whether one was: it counts the violations it observes, and a
//! `Monotone` verdict beside an observed violation is a defect in this module.
//!
//! The probe that showed a hole needs no `absent` is
//! `examples/lattice/04_obligation_trigger_hole.ein`: `total-owed meal is-a`,
//! and a rule that derives `(is-a ?g Diner)` from a hypothesised seat.
//!
//! ### The grain
//!
//! The check reads at **relation grain**, and the theorem above is stated at
//! it: a read is dynamic when its *relation* is in `D`, whatever its pattern.
//! The user ruled that on 2026-11-02 (option (a) of
//! [S1e2.1b.1](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md#the-question-it-puts-to-the-user)),
//! over **fact grain**, where a read is dynamic only when a candidate, or a
//! fact derived from candidates, can match its pattern. Fact grain was
//! prototyped and held to the cube: it unflags 2 of the 22 flagged programs,
//! both past the oracle's limit, and no cube within it
//! (`site_census`, 2026-11-02,
//! [§ 5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md#5-the-grain)).
//! The prototype, `ein-infer/examples/shared/fact_grain.rs`, and its two tests
//! in `tests/region.rs` stay as evidence for S1e2.2.5's reading of
//! *extendable*; no route reads them.
//!
//! ### What it does not touch
//!
//! It reads compiled plans and the program's registries and nothing else — no
//! interning, no event, no counter — so a solve can ask it without moving.
//! *What* a node is on a program with a site is open:
//! [F21](../../../../plans/followups/f21_non_monotone_programs_decided.md)'s
//! D2. [P1e2.1b](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md)
//! detects, documents and warns: on a program with a site, [`crate::detector`]
//! reads what every surface warns of (S1e2.1b.5), and `ein solve --monotone`
//! prints this verdict alone.

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
            let names: Vec<std::borrow::Cow<'_, str>> = self
                .activator
                .iter()
                .map(|&a| ein_core::text::symbol(terms.sym(a)))
                .collect();
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
    /// The detector's static half, on a program with a site: what each site
    /// puts at stake, the cycles through an `absent` over `D`, and the order
    /// findings (M1e2 S1e2.1b.5, [`crate::detector`]). Empty on a program
    /// with none, which it never reads.
    pub detector: Box<crate::detector::Static>,
    /// The **negative cycles** of the saturation rules' dependency graph over
    /// every relation, not only `D` — M1f S1f.5.15's T4, the static
    /// non-stratifiability diagnostic ([`crate::strata::graph`]). Empty on a
    /// stratified program. Read on every program, flagged or not: a cycle
    /// through an `absent` among relations no commitment changes leaves `D`
    /// alone, so the static check calls the program monotone, and root's own
    /// fixpoint is still one of several, the one the boundary's admission order
    /// reaches. A diagnostic and not a refusal: the answer stands.
    pub negative_cycles: Vec<crate::strata::Cycle>,
}

impl Monotonicity {
    /// No site: the module docs' theorem applies.
    pub fn is_monotone(&self) -> bool {
        self.sites.is_empty()
    }

    /// Is `rel` in `D` — a relation some commitment can change? A relation
    /// outside it is static: complete at root, and the same at every node.
    /// L5's premise reads it (`crate::oblgen::alternatives`, M1e2 S1e2.1c.4).
    pub fn changes(&self, rel: Rel, terms: &Terms) -> bool {
        if rel.is_any_negative(terms) {
            return self.dynamic.iter().any(|x| x.negated);
        }
        self.dynamic.contains(&rel)
            || (rel.negated && self.dynamic.contains(&Rel::neg(terms.kernel.not)))
    }
}

/// Every relation `span` of `plan` reads, nested `absent`s included, less the
/// top-level step at `skip` — the reads a guard makes beside its witness
/// (M1e2 S1e2.1c.4's L5).
pub fn reads_in(plan: &Plan, span: Span, skip: Option<usize>, terms: &Terms) -> Vec<Rel> {
    let mut out = Vec::new();
    for (i, step) in plan.steps(span).iter().enumerate() {
        if skip == Some(i) {
            continue;
        }
        match step {
            Step::Rel(r) if r.rel == terms.kernel.not => {
                let before = out.len();
                for slot in plan.slots(r.slots) {
                    if let Slot::Nested { rel, .. } = slot {
                        out.push(Rel::neg(*rel));
                    }
                }
                if out.len() == before {
                    out.push(Rel::neg(terms.kernel.not));
                }
            }
            Step::Rel(r) => out.push(Rel::pos(r.rel)),
            Step::Absent { sub } => out.extend(reads_in(plan, *sub, None, terms)),
            Step::Guard { .. } => {}
        }
    }
    out
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
    let saturation: Vec<(&Plan, &[(Rel, u32)])> = kinded
        .iter()
        .zip(&reads)
        .filter(|((kind, _), _)| *kind == PlanKind::Saturation)
        .map(|((_, plan), reads)| (*plan, reads.as_slice()))
        .collect();
    // determinism-ok: the seed of a set, closed to a fixpoint; nothing reads
    // its order.
    let seed = hypothesised.iter().copied().map(Rel::pos);
    let dynamic = close(seed, &saturation, &[], terms);

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
    // The detector's static half, gated on the verdict just reached: a program
    // with no site gets none and pays one test (M1e2 S1e2.1b.5).
    out.detector = Box::new(crate::detector::of_program(kb, terms, plans, &out));
    // Whether a stratification exists at all (M1f S1f.5.15's T4), on every
    // program: the graph is the one the detector restricts to `D`.
    out.negative_cycles = crate::strata::graph(kb, terms, plans).cycles;
    out
}

/// `D` over `plans` from the seed `hypothesised`: the relations a commitment
/// to facts of those relations can change — the seed, closed under *a
/// saturation plan that reads a relation in `D` asserts into `D`*.
///
/// The one computation of `D`. [`classify`] seeds it with `H`'s relations;
/// the obligations rung seeds it with the relations it would propose, and
/// declines on a guard that scans one (`crate::oblgen::generate`, M1e2
/// S1e2.2.15), before `H` exists to seed [`classify`] with.
pub(crate) fn dynamic(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    hypothesised: impl IntoIterator<Item = Symbol>,
) -> FxHashSet<Rel> {
    let program = kb.program();
    let saturation: Vec<(&Plan, Vec<(Rel, u32)>)> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| (&**p, reads_of(p, terms)))
        .collect();
    let saturation: Vec<(&Plan, &[(Rel, u32)])> =
        saturation.iter().map(|(p, r)| (*p, r.as_slice())).collect();
    // determinism-ok: the seed of a set, closed to a fixpoint; nothing reads
    // its order.
    let seed = hypothesised.into_iter().map(Rel::pos);
    close(seed, &saturation, &[], terms)
}

/// `D` followed through the rules root has no plan of — M1e2
/// [S1e2.2.17](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.17_forbid_activator_chain.md),
/// W17: the relations *forbid* calls extendable.
///
/// `D` is closed over the plans root compiled. A rule with parameters whose
/// activator relation is in `D` is an activator site: a node compiles a plan
/// of it root never had, and what that plan writes a commitment can change
/// too, though `D` does not hold it, nor what a rule activated by it writes.
/// `open` is every rule with parameters, as *its activator relation, the
/// relations its pattern writes*; the result is `m`'s `D` closed under
/// `saturation` (root's plans, `D`'s own rule) and under each such rule whose
/// activator relation it holds. It is `D` on a program with no activator site.
/// [`crate::forbid`] reads it; the static check and every other reader of `D`
/// do not, so no site, no rung decline and no summary moves with it.
pub(crate) fn through_activators(
    m: &Monotonicity,
    saturation: &[(&Plan, &[(Rel, u32)])],
    open: &[(Rel, &[Rel])],
    terms: &Terms,
) -> FxHashSet<Rel> {
    close(m.dynamic.iter().copied(), saturation, open, terms)
}

/// The one closure of `D`: `seed`, closed under *a saturation plan that reads
/// a relation in it asserts into it* and, for each `(activator, writes)` of
/// `open`, *a rule whose activator relation is in it writes `writes` into
/// it*. [`dynamic`] and [`classify`] pass no `open`; [`through_activators`]
/// passes the rules with parameters.
fn close(
    seed: impl IntoIterator<Item = Rel>,
    saturation: &[(&Plan, &[(Rel, u32)])],
    open: &[(Rel, &[Rel])],
    terms: &Terms,
) -> FxHashSet<Rel> {
    // determinism-ok: a set built from a set; nothing reads its order.
    let mut dynamic: FxHashSet<Rel> = seed.into_iter().collect();
    loop {
        let mut grew = false;
        for (plan, reads) in saturation {
            if !reads.iter().any(|&(r, _)| in_set(&dynamic, r, terms)) {
                continue;
            }
            for w in writes_of(plan, terms) {
                grew |= dynamic.insert(w);
            }
        }
        for &(activator, writes) in open {
            if !in_set(&dynamic, activator, terms) {
                continue;
            }
            for &w in writes {
                grew |= dynamic.insert(w);
            }
        }
        if !grew {
            break;
        }
    }
    dynamic
}

/// Is `r` in `set`, the wildcard negative read both ways? `set` is a `D`
/// [`dynamic`] returned.
pub(crate) fn in_set(set: &FxHashSet<Rel>, r: Rel, terms: &Terms) -> bool {
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
