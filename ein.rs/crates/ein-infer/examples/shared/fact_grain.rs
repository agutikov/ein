//! The static check at **fact grain** — M1e2
//! [S1e2.1b.1](../../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md)
//! T1e2.1b.1.2's prototype. It is computed **beside** [`monotone::classify`]
//! and read by no route: `site_census` prints it and
//! `ein-infer/tests/region.rs` holds it to the cube. Which grain the shipped
//! check reads at is the user's ruling (T1e2.1b.1.4), and until then it reads
//! at relation grain.
//!
//! ### What changes, and what does not
//!
//! The relation-grain check calls a read **dynamic** when its relation is in
//! `D`, the relations a commitment can change. Fact grain calls it dynamic
//! only when its **pattern** can match a fact a commitment can add: a
//! candidate, or a fact a saturation plan derives from one. Polarity is read
//! exactly as relation grain reads it, and so is everything else: a site at
//! fact grain is a relation-grain [`Site`](monotone::Site) one of whose reads
//! can match.
//!
//! ### `Δ̂`, the facts a commitment can add
//!
//! A node is root's fixpoint with the commitment written in and saturation
//! resumed, so a fork only adds. Let `Δ` be every fact some node holds beyond
//! root. A fact of `Δ` that is not a candidate was derived by a firing root
//! did not make; at root's fixpoint that firing's match was false, and it is
//! true at the node. A match is made of positive reads, which only grow, and
//! `absent`s; growth can turn it from false to true only through a read at
//! **even** `absent` depth that matches a fact root lacks. So every fact of
//! `Δ` is a candidate, or the conclusion of a saturation plan one of whose
//! even-depth reads matches a fact of `Δ`.
//!
//! `Δ̂` over-approximates `Δ` by that closure over **shapes**: a fact with
//! some arguments replaced by *any*. Its seeds are the candidates. A plan
//! whose even-depth read unifies with a shape in `Δ̂` adds each of its
//! conclusions, with the variables that read bound (a depth-0 read, which
//! shares the conclusion's registers) and *any* for the rest. Every shape is
//! kept only if no shape already there subsumes it, and nesting past
//! [`MAX_NESTING`] is widened to *any*, so the closure terminates.
//!
//! ### The claim the test holds
//!
//! **A read that no shape of `Δ̂` can match has the same extension at every
//! node**: whatever matches it is in root or in no node. So it is static, and
//! the module docs' theorem in `monotone.rs` applies at this grain: no
//! dynamic read at non-monotone polarity, no violation. `region.rs`'s
//! `fact_grain_never_calls_a_violated_cube_monotone` checks it on every cube
//! the oracle enumerates.
//!
//! ### Which node
//!
//! `Δ̂` is about the node `region.rs` builds today, the fork, where a fact
//! root derived is in every node, and the test holds it to that node. On a
//! program with **no** fact-grain site the from-scratch node should agree, by
//! the argument the relation-grain theorem makes: nothing a commitment adds
//! matches a read at non-monotone polarity, so no fact root derived can go
//! missing. That is an argument, not a test; comparing the two nodes is
//! S1e2.1b.2's. A **site's** verdict on a program that stays flagged is read
//! against the fork only: `features/07`'s `derive-q` reads `p` under an
//! `absent`, and `p` is static here because a fork never loses the `(p one)`
//! root derived.
//!
//! ### Where it is not complete
//!
//! An **activator** site is dynamic at fact grain when `Δ̂` holds a shape of
//! the rule's name at the rule's arity. A node can then compile a plan root
//! never had, whose conclusions `Δ̂` does not hold, so on such a program the
//! other sites' fact-grain verdicts are not to be trusted
//! ([`FactGrain::complete`] is false). The program stays flagged either way.

use std::sync::Arc;

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::monotone::{self, Monotonicity, Rel, SiteKind};
use ein_infer::plan::{Plan, Slot, Span, Step};
use rustc_hash::FxHashMap;

/// Nesting past this depth is widened to *any*.
pub const MAX_NESTING: usize = 4;

/// One argument of a [`Shape`].
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Arg {
    /// Any value.
    Any,
    /// A name (or an integer, until M1f S1f.5.4 retired the int kind).
    Val(Value),
    /// A nested fact, `(not (likes A B))`'s one argument.
    Nested(Shape),
}

/// A fact, some of whose arguments may be *any*.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Shape {
    pub rel: Symbol,
    pub args: Box<[Arg]>,
}

impl Shape {
    /// The ground fact `f`.
    pub fn of_fact(terms: &Terms, f: FactId) -> Shape {
        let (rel, args) = terms.facts.get(f);
        Shape {
            rel,
            args: args
                .iter()
                .map(|&v| match v.as_fact() {
                    Some(inner) => Arg::Nested(Shape::of_fact(terms, inner)),
                    None => Arg::Val(v),
                })
                .collect(),
        }
    }

    /// `(rel a _ (inner …))`.
    pub fn render(&self, terms: &Terms) -> String {
        let mut s = format!("({}", terms.sym(self.rel));
        for a in self.args.iter() {
            s.push(' ');
            match a {
                Arg::Any => s.push('_'),
                Arg::Val(v) => s.push_str(&ein_infer::events::sexpr_value(terms, *v)),
                Arg::Nested(n) => s.push_str(&n.render(terms)),
            }
        }
        s.push(')');
        s
    }

    /// Every fact `other` stands for, `self` stands for too.
    fn subsumes(&self, other: &Shape) -> bool {
        self.rel == other.rel
            && self.args.len() == other.args.len()
            && self
                .args
                .iter()
                .zip(other.args.iter())
                .all(|(a, b)| match (a, b) {
                    (Arg::Any, _) => true,
                    (Arg::Val(x), Arg::Val(y)) => x == y,
                    (Arg::Nested(x), Arg::Nested(y)) => x.subsumes(y),
                    _ => false,
                })
    }

    /// Nesting past `depth` levels, widened to *any*.
    fn widened(self, depth: usize) -> Shape {
        Shape {
            rel: self.rel,
            args: self
                .args
                .into_vec()
                .into_iter()
                .map(|a| match a {
                    Arg::Nested(_) if depth == 0 => Arg::Any,
                    Arg::Nested(n) => Arg::Nested(n.widened(depth - 1)),
                    other => other,
                })
                .collect(),
        }
    }
}

/// One read a plan makes: [`monotone::reads_of`]'s relation and depth, and
/// the step that reads it.
#[derive(Clone, Copy, Debug)]
pub struct Read {
    pub rel: Rel,
    pub depth: u32,
    pub step_rel: Symbol,
    pub slots: Span,
}

/// Every read of `plan`, in [`monotone::reads_of`]'s order and with its
/// `(relation, depth)` — which this asserts, so the two walks cannot drift.
pub fn reads(plan: &Plan, terms: &Terms) -> Vec<Read> {
    fn walk(plan: &Plan, terms: &Terms, span: Span, depth: u32, out: &mut Vec<Read>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) if r.rel == terms.kernel.not => {
                    let before = out.len();
                    for slot in plan.slots(r.slots) {
                        if let Slot::Nested { rel, .. } = slot {
                            out.push(Read {
                                rel: Rel {
                                    name: *rel,
                                    negated: true,
                                },
                                depth,
                                step_rel: r.rel,
                                slots: r.slots,
                            });
                        }
                    }
                    if out.len() == before {
                        out.push(Read {
                            rel: Rel {
                                name: terms.kernel.not,
                                negated: true,
                            },
                            depth,
                            step_rel: r.rel,
                            slots: r.slots,
                        });
                    }
                }
                Step::Rel(r) => out.push(Read {
                    rel: Rel {
                        name: r.rel,
                        negated: false,
                    },
                    depth,
                    step_rel: r.rel,
                    slots: r.slots,
                }),
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
    let ours: Vec<(Rel, u32)> = out.iter().map(|r| (r.rel, r.depth)).collect();
    assert_eq!(
        ours,
        monotone::reads_of(plan, terms),
        "fact_grain::reads has drifted from monotone::reads_of"
    );
    out
}

/// A register's binding while one step is unified with one shape.
type Binds = FxHashMap<u16, Arg>;

/// The most general argument both stand for.
fn meet(a: &Arg, b: &Arg) -> Option<Arg> {
    match (a, b) {
        (Arg::Any, x) | (x, Arg::Any) => Some(x.clone()),
        (Arg::Val(x), Arg::Val(y)) => (x == y).then_some(Arg::Val(*x)),
        (Arg::Nested(x), Arg::Nested(y)) => {
            if x.rel != y.rel || x.args.len() != y.args.len() {
                return None;
            }
            let args: Option<Vec<Arg>> = x
                .args
                .iter()
                .zip(y.args.iter())
                .map(|(p, q)| meet(p, q))
                .collect();
            Some(Arg::Nested(Shape {
                rel: x.rel,
                args: args?.into_boxed_slice(),
            }))
        }
        _ => None,
    }
}

/// Can some fact `args` stands for match `slots`? The matcher's unifier
/// (`match_.rs`), over shapes: an arity check, a constant compared, a
/// register bound on first sight and compared after, an opaque slot never.
/// Registers bound by earlier steps are not known here, so they are free —
/// which only ever says *can* where the matcher might not.
fn unify(plan: &Plan, slots: &[Slot], args: &[Arg], binds: &mut Binds) -> bool {
    if slots.len() != args.len() {
        return false;
    }
    for (slot, arg) in slots.iter().zip(args) {
        let ok = match slot {
            Slot::Opaque(_) => false,
            Slot::Reg(r) => {
                let m = match binds.get(r) {
                    Some(b) => meet(b, arg),
                    None => Some(arg.clone()),
                };
                match m {
                    Some(m) => {
                        binds.insert(*r, m);
                        true
                    }
                    None => false,
                }
            }
            Slot::Const(v) => match arg {
                Arg::Any => true,
                Arg::Val(w) => v == w,
                // A constant that is itself a fact: compared by identity in
                // the matcher, and not resolved here.
                Arg::Nested(_) => v.as_fact().is_some(),
            },
            Slot::Nested { rel, slots: sub } => match arg {
                Arg::Any => {
                    let n = plan.slots(*sub).len();
                    let any: Vec<Arg> = vec![Arg::Any; n];
                    unify(plan, plan.slots(*sub), &any, binds)
                }
                Arg::Nested(s) => s.rel == *rel && unify(plan, plan.slots(*sub), &s.args, binds),
                Arg::Val(_) => false,
            },
        };
        if !ok {
            return false;
        }
    }
    true
}

/// A conclusion template, its registers read from `binds` and *any* where
/// unbound.
fn instantiate(plan: &Plan, slot: &Slot, binds: &Binds) -> Option<Shape> {
    fn arg(plan: &Plan, slot: &Slot, binds: &Binds) -> Arg {
        match slot {
            Slot::Reg(r) => binds.get(r).cloned().unwrap_or(Arg::Any),
            Slot::Const(v) if v.as_fact().is_none() => Arg::Val(*v),
            Slot::Const(_) | Slot::Opaque(_) => Arg::Any,
            Slot::Nested { rel, slots } => Arg::Nested(Shape {
                rel: *rel,
                args: plan
                    .slots(*slots)
                    .iter()
                    .map(|s| arg(plan, s, binds))
                    .collect(),
            }),
        }
    }
    // Only a nested pattern is a conclusion, as `monotone::writes_of` reads
    // them.
    match arg(plan, slot, binds) {
        Arg::Nested(s) => Some(s),
        _ => None,
    }
}

/// The fact-grain verdict for one program.
#[derive(Clone, Debug)]
pub struct FactGrain {
    /// `Δ̂`, as shapes, sorted by text.
    pub shapes: Vec<String>,
    /// Per site of the relation-grain verdict, in its order: a shape of `Δ̂`
    /// one of the site's reads can match — the site is still one at fact
    /// grain — or `None`.
    pub sites: Vec<Option<String>>,
    /// No activator site is dynamic at fact grain, so `Δ̂` holds every plan a
    /// node can run and each site's verdict above is the prototype's claim.
    pub complete: bool,
}

impl FactGrain {
    /// No site survives.
    pub fn is_monotone(&self) -> bool {
        self.sites.iter().all(Option::is_none)
    }
}

/// `Δ̂`, and each of `relation`'s sites read at fact grain. `candidates` is
/// the universe; `plans` every plan root compiled; `relation` the
/// relation-grain verdict over the same plans.
pub fn classify(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    candidates: &[FactId],
    relation: &Monotonicity,
) -> FactGrain {
    let program = kb.program();
    let saturation: Vec<(&Plan, Vec<Read>)> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| (&**p, reads(p, terms)))
        .collect();

    // Δ̂, by relation, each list an antichain under `subsumes`.
    let mut delta: FxHashMap<Symbol, Vec<Shape>> = FxHashMap::default();
    let insert = |delta: &mut FxHashMap<Symbol, Vec<Shape>>, s: &Shape| -> bool {
        let v = delta.entry(s.rel).or_default();
        if v.iter().any(|g| g.subsumes(s)) {
            return false;
        }
        v.retain(|x| !s.subsumes(x));
        v.push(s.clone());
        true
    };
    let mut work: Vec<Shape> = Vec::new();
    for &f in candidates {
        let s = Shape::of_fact(terms, f);
        if insert(&mut delta, &s) {
            work.push(s);
        }
    }
    while let Some(s) = work.pop() {
        for (plan, rs) in &saturation {
            for r in rs
                .iter()
                .filter(|r| r.depth % 2 == 0 && r.step_rel == s.rel)
            {
                let mut binds = Binds::default();
                if !unify(plan, plan.slots(r.slots), &s.args, &mut binds) {
                    continue;
                }
                // A read inside an `absent` has registers of its own.
                if r.depth != 0 {
                    binds.clear();
                }
                for a in plan.asserts.iter() {
                    let Some(out) = instantiate(plan, a, &binds) else {
                        continue;
                    };
                    let out = out.widened(MAX_NESTING);
                    if insert(&mut delta, &out) {
                        work.push(out);
                    }
                }
            }
        }
    }

    // Can `read`'s pattern match a shape of Δ̂? The first one it can, as text.
    let matched = |plan: &Plan, r: &Read| -> Option<String> {
        let shapes = delta.get(&r.step_rel)?;
        let mut hits: Vec<String> = shapes
            .iter()
            .filter(|s| unify(plan, plan.slots(r.slots), &s.args, &mut Binds::default()))
            .map(|s| s.render(terms))
            .collect();
        hits.sort();
        hits.into_iter().next()
    };
    let arity_of = |name: Symbol| -> usize {
        program
            .rules
            .get(name)
            .or_else(|| program.obligations.get(name))
            .map(|r| r.params.len())
            .expect("an activator site names a rule or an obligation")
    };
    let mut complete = true;
    let sites: Vec<Option<String>> = relation
        .sites
        .iter()
        .map(|site| match site.kind {
            SiteKind::Activator => {
                let arity = arity_of(site.rule);
                let mut hits: Vec<String> = delta
                    .get(&site.rule)
                    .into_iter()
                    .flatten()
                    .filter(|s| s.args.len() == arity)
                    .map(|s| s.render(terms))
                    .collect();
                hits.sort();
                let hit = hits.into_iter().next();
                if hit.is_some() {
                    complete = false;
                }
                hit
            }
            SiteKind::Absent | SiteKind::ObligationTrigger => {
                let mut hits: Vec<String> = plans
                    .iter()
                    .filter(|p| p.rule == site.rule && p.activator_args == site.activator)
                    .flat_map(|p| {
                        reads(p, terms)
                            .into_iter()
                            .filter(|r| r.rel == site.relation && r.depth == site.depth)
                            .filter_map(|r| matched(p, &r))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                hits.sort();
                hits.into_iter().next()
            }
        })
        .collect();

    // determinism-ok: flattened into text and sorted on the next line.
    let mut shapes: Vec<String> = delta.values().flatten().map(|s| s.render(terms)).collect();
    shapes.sort();
    FactGrain {
        shapes,
        sites,
        complete,
    }
}
