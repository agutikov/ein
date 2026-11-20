//! What ends a state, as an explanation reads it, and the rules that refute
//! a fact beside one fact that holds it.
//!
//! **A state is dead where a rule that states `:ein inconsistent` fired**
//! ([`ein_core::Kb::ended`], Q-M1f.399): the record of its application,
//! which is no fact. The kernel reads no name to judge a state. A held
//! `(false)` ends a state by the rule every program gets, `std.core`'s,
//! which matches it and states the key (Q-M1f.551); a fact beside its stored
//! negation ends one by `std.contradiction`'s rules, which conclude `(false)`
//! from the two, where a program imports the module (Q-M1f.557). A program
//! that imports no such rule has no contradiction of that shape: a state may
//! hold a fact beside its negation and be alive.
//!
//! ### How an explanation reads a death: [`detect`]
//!
//! 1. **ended** — the firing of a rule the program wrote or imported that
//!    states the key: the rule is named, its premises explained.
//! 2. **pair** — a fact `X` beside a fact `(N X)` that holds it as its one
//!    argument, from which a firing concluded what the rule every program
//!    gets matched. An explanation reads through that rule
//!    ([`ein_core::Rule::implicit`]) to what its firing matched, and through
//!    a firing whose premises are such a pair to the pair itself, explained
//!    together: so `refuted by` names the rules that derived `X` and `(N X)`,
//!    and the core is the frontier of the two. A firing of the rule every
//!    program gets whose `(false)` some other derivation concluded too is
//!    read as the first shape as well, through its `(false)`.
//!
//! Each encodes a **branch failure** under the append-only KB: there is no
//! way to retract a fact or a firing, so the only resolution is for whatever
//! caused the conflict, typically a commitment, to be unwound. The result
//! **order** is fixed, the firings in the order they fired and, under each,
//! its pairs in the order the store records their derivations, because it
//! reaches the unsat core and the trace.
//!
//! ### What a rule refutes beside a fact: [`Pairs`]
//!
//! Read off the compiled plans, with no name: a plan whose disjunct is two
//! premises and nothing else, a fact `(R ?a …)` of distinct variables and a
//! fact `(N (R ?a …))` holding it as its one argument, and whose firing ends
//! a state, by stating the key or by concluding a fact a rule that states it
//! matches whole and alone. `std.contradiction`'s `clash-N`, activated for
//! `R`, is such a plan, `N` the relation `not`, and so is a program's own.
//! Where one is compiled, a state that holds a fact of `R` beside its
//! `(N …)` is dead whatever else it holds, which three economies read: a
//! candidate root so denies is not in `H` ([`crate::hypgen`]); a candidate an
//! alive state so denies is dead above it (L8, [`crate::known_dead`]); and
//! the agenda's first half ([`crate::saturator::AGENDA`]).

use ein_core::{FactId, Kb, ProvId, Symbol, Terms, Value};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::plan::{Plan, RelStep, Slot, Step};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Ended,
    Pair,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Ended => "ended",
            Kind::Pair => "pair",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contradiction {
    /// A rule that states `:ein inconsistent` fired: the record of the
    /// application ([`ein_core::Kb::ended`]).
    Ended(ProvId),
    /// The fact `X` beside `(N X)`, from which a firing concluded what the
    /// rule every program gets matched (the module's § How an explanation
    /// reads a death).
    Pair { positive: FactId, negative: FactId },
}

impl Contradiction {
    pub fn kind(&self) -> Kind {
        match self {
            Contradiction::Ended(_) => Kind::Ended,
            Contradiction::Pair { .. } => Kind::Pair,
        }
    }

    /// The fact that names the contradiction: for a pair the positive `X`,
    /// and for a firing that ended the state the first premise it matched,
    /// the `(false)` of a state that holds one, `None` where it matched no
    /// fact.
    /// It is the core search's tie-break and
    /// [`crate::explain::Explanation::target`].
    ///
    /// It is **not** what the core is the frontier of: that is
    /// [`Contradiction::sides`]. A core of this fact alone would leave out
    /// the derivation of a pair's `(N X)`, where a commitment typically is.
    pub fn witness(&self, terms: &Terms) -> Option<FactId> {
        match *self {
            Contradiction::Ended(p) => terms.provs.get(p).premises.first().copied(),
            Contradiction::Pair { positive, .. } => Some(positive),
        }
    }

    /// The facts the contradiction is made of, which the core search
    /// explains together ([`crate::explain::contradiction_frontier`]): a
    /// pair's `X` and its `(N X)`, the witness first, or the premises of the
    /// firing that ended the state, in the order its plan matched them.
    pub fn sides(&self, terms: &Terms) -> Vec<FactId> {
        match *self {
            Contradiction::Ended(p) => terms.provs.get(p).premises.to_vec(),
            Contradiction::Pair { positive, negative } => vec![positive, negative],
        }
    }

    /// The firing that ended the state, where the contradiction is one.
    pub fn ended(&self) -> Option<ProvId> {
        match *self {
            Contradiction::Ended(p) => Some(p),
            Contradiction::Pair { .. } => None,
        }
    }

    /// **The last steps of the contradiction's first-recorded derivation**:
    /// the primary of each fact it is made of, and for a firing that ended
    /// the state the firing itself, or, where its rule is of the module
    /// every program gets ([`ein_core::Rule::implicit`]), the primaries of
    /// the premises it matched: an explanation names the rules a program
    /// wrote or imported, and reads through such a rule to what its firing
    /// rests on.
    pub fn first_steps(&self, kb: &Kb, terms: &Terms) -> Vec<ProvId> {
        match *self {
            Contradiction::Ended(p) if !ended_by_an_implicit_rule(kb, terms, p) => vec![p],
            _ => self
                .sides(terms)
                .into_iter()
                .filter_map(|f| kb.primary(f))
                .collect(),
        }
    }
}

/// Whether the firing `prov` records is of a rule of the module every
/// program gets ([`ein_core::Rule::implicit`]), which an explanation reads
/// through.
pub fn ended_by_an_implicit_rule(kb: &Kb, terms: &Terms, prov: ProvId) -> bool {
    terms
        .provs
        .get(prov)
        .rule
        .and_then(|r| kb.program().rules.get(r))
        .is_some_and(|r| r.implicit)
}

/// The two facts of a firing whose premises are a fact and a fact holding
/// it as its one argument, the held one first; `None` for any other.
fn pair_of(terms: &Terms, premises: &[FactId]) -> Option<(FactId, FactId)> {
    let &[a, b] = premises else {
        return None;
    };
    let holds = |outer: FactId, inner: FactId| terms.facts.args(outer) == [Value::fact(inner)];
    if holds(b, a) {
        Some((a, b))
    } else if holds(a, b) {
        Some((b, a))
    } else {
        None
    }
}

/// Every contradiction in the KB, as an explanation reads it (the module's
/// § How an explanation reads a death): each firing that ended it, in the
/// order they fired, a firing of the rule every program gets read through to
/// the pairs what it matched was concluded from.
pub fn detect(kb: &Kb, terms: &Terms) -> Vec<Contradiction> {
    let mut out: Vec<Contradiction> = Vec::new();
    for &p in kb.ended() {
        if !ended_by_an_implicit_rule(kb, terms, p) {
            out.push(Contradiction::Ended(p));
            continue;
        }
        let mut pairs: Vec<Contradiction> = Vec::new();
        let mut through = true;
        for &f in terms.provs.get(p).premises.iter() {
            let ways = kb.justifications(f);
            through &= !ways.is_empty();
            for way in ways {
                match pair_of(terms, &terms.provs.get(way).premises) {
                    Some((positive, negative)) => {
                        let pair = Contradiction::Pair { positive, negative };
                        if !pairs.contains(&pair) {
                            pairs.push(pair);
                        }
                    }
                    None => through = false,
                }
            }
        }
        if !through || pairs.is_empty() {
            out.push(Contradiction::Ended(p));
        }
        out.extend(pairs);
    }
    out
}

/// [`detect`] for a reader that must not depend on which derivations the
/// store records (`:record-alternative-justifications`, whose one exemption
/// is the core): the firings that ended the state, a firing of the rule
/// every program gets kept where what it matched was first derived by no
/// pair, and then **every** pair `pairs` refutes that the KB holds, in the
/// KB's order, rather than the pairs the recorded derivations name. The
/// structure's node reading takes it ([`crate::structure`]).
pub fn detect_all(kb: &Kb, terms: &Terms, pairs: &Pairs) -> Vec<Contradiction> {
    let mut out: Vec<Contradiction> = Vec::new();
    for &p in kb.ended() {
        if !ended_by_an_implicit_rule(kb, terms, p) {
            out.push(Contradiction::Ended(p));
            continue;
        }
        let by_a_pair = terms.provs.get(p).premises.iter().all(|&f| {
            kb.primary(f)
                .is_some_and(|way| pair_of(terms, &terms.provs.get(way).premises).is_some())
        });
        if !by_a_pair {
            out.push(Contradiction::Ended(p));
        }
    }
    for negative in kb.facts() {
        let &[inner] = terms.facts.args(negative) else {
            continue;
        };
        let Some(positive) = inner.as_fact() else {
            continue;
        };
        let (wrapper, _) = terms.fact(negative);
        let (rel, args) = terms.fact(positive);
        if kb.contains(positive)
            && pairs
                .by_fact
                .get(&(rel, args.len()))
                .is_some_and(|ns| ns.contains(&wrapper))
        {
            out.push(Contradiction::Pair { positive, negative });
        }
    }
    out
}

/// Whether the KB holds a contradiction: whether a rule that states
/// `:ein inconsistent` fired on it.
pub fn has_contradiction(kb: &Kb, _terms: &Terms) -> bool {
    kb.is_inconsistent()
}

/// **The rules that refute a fact beside one fact that holds it**, read off
/// the compiled plans (the module's § What a rule refutes beside a fact).
#[derive(Clone, Debug, Default)]
pub struct Pairs {
    /// `(R, count)` → each `N` such that a fact of `R` of that many
    /// arguments beside `(N fact)` is refuted, in the plans' order.
    by_fact: FxHashMap<(Symbol, usize), Vec<Symbol>>,
    /// `N` → every `R` it is read with, at any count.
    by_wrapper: FxHashMap<Symbol, FxHashSet<Symbol>>,
    /// For each plan [`Pairs::of`] read, in its order: whether the plan is
    /// such a rule.
    refuting: Vec<bool>,
    /// For each plan [`Pairs::of`] read, in its order: where it is such a
    /// rule and its match is the pair and nothing else, one disjunct, the
    /// pair's shape, which the saturator matches by a probe.
    shapes: Vec<Option<PairShape>>,
}

/// **A plan whose match is a pair and nothing else**: one disjunct, no
/// `(absent …)`, two premises, a fact `(R ?a …)` of `count` distinct
/// variables at the step `held`, and `(wrapper (R ?a …))` at the other, and a
/// firing that ends a state ([`Pairs::shape_at`]). `std.contradiction`'s
/// `clash-N`, activated for `R`, is one, `wrapper` the relation `not`.
///
/// Its matches need no walk: a new fact `F` of `R` matches it where the state
/// holds `(wrapper F)`, a new `(wrapper G)` where the state holds `G`, one
/// probe each ([`PairShape::each_match`]), which the saturator asks in place
/// of a seeded match (`saturator`'s § A pair matched by a probe).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PairShape {
    /// The relation of the fact that holds the other, as the rule's shape
    /// names it: no name the kernel knows.
    pub wrapper: Symbol,
    /// The relation of the held fact.
    pub rel: Symbol,
    /// The held fact's count of arguments.
    pub count: usize,
    /// The step of the held fact, 0 or 1; the other is the fact holding it.
    pub held: usize,
}

impl PairShape {
    /// The premises, in step order, of each match of the plan that `fact`
    /// takes part in, handed to `each` in the order a seeded match finds
    /// them: the plan's steps in order, `fact` at each it can stand at.
    ///
    /// At the held step, a fact of `rel` and `count` arguments, where `kb`
    /// holds `(wrapper fact)`: the fact store's own probe of that
    /// proposition, which builds nothing, then `Kb::contains`. At the other,
    /// `(wrapper G)` with `G` a fact of `rel` and `count` arguments, where
    /// `kb` holds `G`. A fact of another relation or count is turned away by
    /// a compare.
    pub fn each_match(
        &self,
        kb: &Kb,
        terms: &Terms,
        fact: FactId,
        mut each: impl FnMut([FactId; 2]),
    ) {
        let (rel, args) = terms.facts.get(fact);
        for step in 0..2 {
            if step == self.held {
                if rel == self.rel
                    && args.len() == self.count
                    && let Some(holding) = terms.facts.probe(self.wrapper, &[Value::fact(fact)])
                    && kb.contains(holding)
                {
                    each(self.premises(fact, holding));
                }
            } else if rel == self.wrapper
                && let [held] = args
                && let Some(held) = held.as_fact()
                && terms.facts.rel(held) == self.rel
                && terms.facts.args(held).len() == self.count
                && kb.contains(held)
            {
                each(self.premises(held, fact));
            }
        }
    }

    /// The held fact and the one holding it, in step order.
    fn premises(&self, held: FactId, holding: FactId) -> [FactId; 2] {
        if self.held == 0 {
            [held, holding]
        } else {
            [holding, held]
        }
    }
}

impl Pairs {
    /// The pairs `plans` refute.
    pub fn of<'a>(plans: impl IntoIterator<Item = &'a Plan>, terms: &Terms) -> Pairs {
        let plans: Vec<&Plan> = plans.into_iter().collect();
        // What ends a state matched whole and alone: a plan that states the
        // key over one premise of distinct variables.
        // determinism-ok: a set asked for membership alone.
        let mut alone: FxHashSet<(Symbol, usize)> = FxHashSet::default();
        for plan in plans.iter().filter(|p| p.ends) {
            if let [d] = &*plan.disjuncts
                && d.guards.is_empty()
                && let [Step::Rel(r)] = plan.steps(d.steps)
                && distinct_variables(plan.slots(r.slots))
            {
                alone.insert((r.rel, r.slots.len()));
            }
        }
        let ends = |plan: &Plan| {
            plan.ends
                || plan.asserts.iter().any(|slot| match *slot {
                    Slot::Nested { rel, slots } => {
                        crate::firing::computed(terms, plan, *slot).is_none()
                            && alone.contains(&(rel, slots.len()))
                    }
                    _ => false,
                })
        };
        let mut out = Pairs::default();
        for plan in plans {
            out.refuting.push(false);
            out.shapes.push(None);
            if !ends(plan) {
                continue;
            }
            for d in plan.disjuncts.iter() {
                if !d.guards.is_empty() {
                    continue;
                }
                let [Step::Rel(a), Step::Rel(b)] = plan.steps(d.steps) else {
                    continue;
                };
                let Some((wrapper, rel, count, held)) = wrapped(plan, a, b)
                    .map(|(w, r, n)| (w, r, n, 0))
                    .or_else(|| wrapped(plan, b, a).map(|(w, r, n)| (w, r, n, 1)))
                else {
                    continue;
                };
                let of = out.by_fact.entry((rel, count)).or_default();
                if !of.contains(&wrapper) {
                    of.push(wrapper);
                }
                out.by_wrapper.entry(wrapper).or_default().insert(rel);
                *out.refuting.last_mut().expect("pushed above") = true;
                if plan.disjuncts.len() == 1 {
                    *out.shapes.last_mut().expect("pushed above") = Some(PairShape {
                        wrapper,
                        rel,
                        count,
                        held,
                    });
                }
            }
        }
        out
    }

    /// The shape of the `at`-th plan [`Pairs::of`] read, where it refutes a
    /// pair and its match is that pair and nothing else ([`PairShape`]).
    pub fn shape_at(&self, at: usize) -> Option<PairShape> {
        self.shapes.get(at).copied().flatten()
    }

    /// Whether the `at`-th plan [`Pairs::of`] read refutes a pair: its
    /// firing ends the state on the two facts alone, so the agenda fires it
    /// as it fires a rule that states the key, before every other
    /// ([`crate::saturator::AGENDA`]).
    pub fn refutes_at(&self, at: usize) -> bool {
        self.refuting.get(at).copied().unwrap_or(false)
    }

    /// No plan refutes a pair.
    pub fn is_empty(&self) -> bool {
        self.by_fact.is_empty()
    }

    /// Whether `kb` holds a fact that, beside `fact`, a rule refutes: a
    /// `(N fact)` for an `N` some plan reads with `fact`'s relation and
    /// count. No fact is interned to ask.
    pub fn denies(&self, kb: &Kb, terms: &Terms, fact: FactId) -> bool {
        let (rel, args) = terms.fact(fact);
        self.by_fact.get(&(rel, args.len())).is_some_and(|ns| {
            ns.iter().any(|&n| {
                terms
                    .probe_fact(n, &[Value::fact(fact)])
                    .is_some_and(|w| kb.contains(w))
            })
        })
    }

    /// Whether some plan refutes a fact of `rel` beside `(wrapper fact)`.
    pub fn pairs(&self, wrapper: Symbol, rel: Symbol) -> bool {
        self.by_wrapper
            .get(&wrapper)
            .is_some_and(|r| r.contains(&rel))
    }

    /// Every `N` some plan refutes a fact beside `(N fact)` of, in no
    /// order: for a reader that collects into a set.
    pub fn wrappers(&self) -> impl Iterator<Item = Symbol> + '_ {
        // determinism-ok: the one caller collects what it reads into a set.
        self.by_wrapper.keys().copied()
    }

    /// Every relation some plan refutes a fact of beside `(wrapper fact)`;
    /// `None` where `wrapper` is no such relation.
    pub fn wrapped_by(&self, wrapper: Symbol) -> Option<&FxHashSet<Symbol>> {
        self.by_wrapper.get(&wrapper)
    }

    /// What the conclusion `slot` of `plan` is as the side of a pair that
    /// holds the other: `(N (R …))` as `(N, Some(R))`, and `(N ?x)`, of any
    /// fact, as `(N, None)`; `None` where it is neither, or `N` wraps no
    /// relation of a pair.
    pub fn holding_side(&self, plan: &Plan, slot: Slot) -> Option<(Symbol, Option<Symbol>)> {
        let Slot::Nested {
            rel: wrapper,
            slots,
        } = slot
        else {
            return None;
        };
        let rels = self.by_wrapper.get(&wrapper)?;
        match plan.slots(slots) {
            [Slot::Nested { rel, .. }] if rels.contains(rel) => Some((wrapper, Some(*rel))),
            [Slot::Reg(_)] => Some((wrapper, None)),
            _ => None,
        }
    }
}

/// `inner` a fact of distinct variables, and `outer` a fact holding it as
/// its one argument: `(outer's relation, inner's relation, inner's count)`.
fn wrapped(plan: &Plan, inner: &RelStep, outer: &RelStep) -> Option<(Symbol, Symbol, usize)> {
    let held = plan.slots(inner.slots);
    if !distinct_variables(held) {
        return None;
    }
    match plan.slots(outer.slots) {
        [Slot::Nested { rel, slots }] if *rel == inner.rel && plan.slots(*slots) == held => {
            Some((outer.rel, inner.rel, held.len()))
        }
        _ => None,
    }
}

/// Every slot a variable, no two the same.
fn distinct_variables(slots: &[Slot]) -> bool {
    slots.iter().enumerate().all(|(i, s)| match s {
        Slot::Reg(r) => !slots[..i].contains(&Slot::Reg(*r)),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::SharedMemo;
    use crate::events::Events;
    use crate::saturator::{Saturator, Session};
    use ein_ir::{Ast, from_ir::load, parse};

    fn saturated(src: &str) -> (Terms, Kb, Vec<std::sync::Arc<Plan>>) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let memo = SharedMemo::default();
        let mut ev = Events::off();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: memo.clone(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        }
        let plans = crate::monotone::plans_of(&memo);
        (terms, kb, plans)
    }

    /// A held `(false)` is a fact like any other: the kernel reads no
    /// contradiction off it, nor off a fact beside its negation. What ends a
    /// state that holds it is the firing of the rule every program gets,
    /// which `a_pair_with_no_rule_over_it_is_no_contradiction` saturates.
    #[test]
    fn a_held_false_is_no_contradiction_by_its_name() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let src = "(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n(false)\n";
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        assert!(detect(&kb, &terms).is_empty());
        assert!(!has_contradiction(&kb, &terms));
    }

    const CLASH: &str =
        "(rule clash-1 (?R) :match (and (?R ?a) (not (?R ?a))) :assert (false) :why \"c\")\n";

    /// A fact beside its stored negation is no contradiction of the
    /// kernel's: with no rule over the two, the state is alive; a held
    /// `(false)` ends it, by the rule every program gets.
    #[test]
    fn a_pair_with_no_rule_over_it_is_no_contradiction() {
        let (terms, kb, plans) =
            saturated("(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n");
        assert!(!has_contradiction(&kb, &terms));
        assert!(detect(&kb, &terms).is_empty());
        assert!(Pairs::of(plans.iter().map(|p| &**p), &terms).is_empty());

        let (terms, kb, _) = saturated(
            "(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n(false)\n",
        );
        assert!(has_contradiction(&kb, &terms));
        let found = detect(&kb, &terms);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].kind(), Kind::Ended, "a stated (false) is no pair");
    }

    /// With a rule that concludes `(false)` from the two, the state is dead,
    /// an explanation reads the death as the pair, and the rule is read as
    /// one that refutes a fact of `p` beside its negation.
    #[test]
    fn a_pair_a_rule_refutes_is_read_as_the_pair() {
        let src = format!(
            "{CLASH}(relation p Thing :why \"p holds of {{?3}}\")\n(clash-1 p)\n(p one)\n(not (p one))\n(p two)\n"
        );
        let (terms, kb, plans) = saturated(&src);
        assert!(has_contradiction(&kb, &terms));
        let found = detect(&kb, &terms);
        assert_eq!(found.len(), 1, "{found:?}");
        let Contradiction::Pair { positive, negative } = found[0] else {
            panic!("{found:?}")
        };
        assert_eq!(terms.facts.args(negative), [Value::fact(positive)]);
        assert_eq!(found[0].sides(&terms), vec![positive, negative]);
        assert_eq!(Some(positive), found[0].witness(&terms));

        let pairs = Pairs::of(plans.iter().map(|p| &**p), &terms);
        let p = terms.syms.get("p").expect("interned");
        let not = terms.syms.get("not").expect("interned");
        assert!(pairs.pairs(not, p));
        assert!(pairs.denies(&kb, &terms, positive));
        let two = kb
            .facts_of(p)
            .find(|&f| f != positive)
            .expect("(p two) is held");
        assert!(
            !pairs.denies(&kb, &terms, two),
            "nothing holds (not (p two))"
        );
    }

    /// A rule over a pair that also reads a third fact refutes no pair
    /// alone, and neither does one that concludes a fact nothing ends a
    /// state on.
    #[test]
    fn only_a_pair_alone_that_ends_a_state_is_read() {
        let (terms, _, plans) = saturated(
            "(relation p Thing :why \"p holds of {?3}\")\n(relation q Thing :why \"q {?3}\")\n\
             (rule three () :match (and (p ?a) (not (p ?a)) (q ?a)) :assert (false) :why \"t\")\n\
             (rule noted () :match (and (p ?a) (not (p ?a))) :assert (q ?a) :why \"n\")\n(p one)\n",
        );
        assert!(Pairs::of(plans.iter().map(|p| &**p), &terms).is_empty());
    }

    /// The shape of each plan `src` compiles, by its rule's name.
    fn shapes(src: &str) -> (Terms, Kb, Vec<(String, Option<PairShape>)>) {
        let (terms, kb, plans) = saturated(src);
        let pairs = Pairs::of(plans.iter().map(|p| &**p), &terms);
        let shapes = plans
            .iter()
            .enumerate()
            .map(|(at, p)| (terms.sym(p.rule).to_string(), pairs.shape_at(at)))
            .collect();
        (terms, kb, shapes)
    }

    fn shape_of(shapes: &[(String, Option<PairShape>)], rule: &str) -> Option<PairShape> {
        shapes
            .iter()
            .find(|(name, _)| name == rule)
            .unwrap_or_else(|| panic!("no plan of {rule}"))
            .1
    }

    /// A plan whose match is the pair and nothing else has a shape, its held
    /// fact at the step it is written at; one that refutes a pair in one of
    /// two disjuncts refutes it and has none, so it is seeded.
    #[test]
    fn a_pair_alone_has_a_shape_and_its_held_step() {
        let (terms, _, shapes) = shapes(
            "(relation p A B :why \"p holds of {?3} and {?4}\")\n(relation q Thing :why \"q {?3}\")\n\
             (rule first () :match (and (p ?a ?b) (not (p ?a ?b))) :assert (false) :why \"f\")\n\
             (rule second () :match (and (not (q ?a)) (q ?a)) :ein inconsistent :why \"s\")\n\
             (rule either () :match (or (and (q ?a) (not (q ?a))) (and (p ?a ?b) (not (p ?a ?b)))) :assert (false) :why \"e\")\n\
             (p one two)\n",
        );
        let (p, q) = (terms.syms.get("p").unwrap(), terms.syms.get("q").unwrap());
        let not = terms.syms.get("not").unwrap();
        assert_eq!(
            shape_of(&shapes, "first"),
            Some(PairShape {
                wrapper: not,
                rel: p,
                count: 2,
                held: 0
            })
        );
        assert_eq!(
            shape_of(&shapes, "second"),
            Some(PairShape {
                wrapper: not,
                rel: q,
                count: 1,
                held: 1
            })
        );
        assert_eq!(
            shape_of(&shapes, "either"),
            None,
            "two disjuncts are seeded"
        );
    }

    /// A pair's probe finds the match from either side: a new fact where the
    /// state holds the fact holding it, a new holding fact where the state
    /// holds the fact it holds, and a fact of another count or relation is
    /// turned away.
    #[test]
    fn a_pair_is_found_from_either_side_by_a_probe() {
        let (terms, kb, shapes) = shapes(
            "(relation p Thing :why \"p holds of {?3}\")\n(relation q Thing :why \"q {?3}\")\n\
             (rule first () :match (and (p ?a) (not (p ?a))) :assert (false) :why \"f\")\n\
             (p one)\n(not (p one))\n(p two)\n(p one two)\n(not (q one))\n(q one)\n",
        );
        let shape = shape_of(&shapes, "first").expect("a pair alone");
        let fact = |rel: &str, args: &[&str]| {
            let rel = terms.syms.get(rel).unwrap();
            let args: Vec<Value> = args
                .iter()
                .map(|a| Value::sym(terms.syms.get(a).unwrap()))
                .collect();
            terms.probe_fact(rel, &args).expect("interned")
        };
        let not = terms.syms.get("not").unwrap();
        let p_one = fact("p", &["one"]);
        let not_p_one = terms.probe_fact(not, &[Value::fact(p_one)]).unwrap();
        let q_one = fact("q", &["one"]);
        let not_q_one = terms.probe_fact(not, &[Value::fact(q_one)]).unwrap();
        let found = |f: FactId| {
            let mut out = Vec::new();
            shape.each_match(&kb, &terms, f, |premises| out.push(premises));
            out
        };
        assert_eq!(found(p_one), [[p_one, not_p_one]]);
        assert_eq!(found(not_p_one), [[p_one, not_p_one]]);
        assert!(
            found(fact("p", &["two"])).is_empty(),
            "nothing holds (not (p two))"
        );
        assert!(
            found(fact("p", &["one", "two"])).is_empty(),
            "another count"
        );
        assert!(found(not_q_one).is_empty(), "another relation");
        assert!(found(q_one).is_empty(), "another relation");
    }

    /// A wrapper may be the held fact's own relation: a fact of it stands at
    /// either step, and the saturation, which the gate's build holds to a
    /// seeded match fact by fact, ends the state.
    #[test]
    fn a_wrapper_of_its_own_relation_is_probed_at_both_steps() {
        let src = "(relation w Thing :why \"w {?3}\")\n\
                   (rule own () :match (and (w ?a) (w (w ?a))) :assert (false) :why \"o\")\n\
                   (w x)\n(w (w x))\n";
        let (terms, kb, shapes) = shapes(src);
        let w = terms.syms.get("w").unwrap();
        assert_eq!(
            shape_of(&shapes, "own"),
            Some(PairShape {
                wrapper: w,
                rel: w,
                count: 1,
                held: 0
            })
        );
        assert!(has_contradiction(&kb, &terms));
        let found = detect(&kb, &terms);
        let [Contradiction::Pair { positive, negative }] = found[..] else {
            panic!("{found:?}")
        };
        assert_eq!(terms.facts.args(negative), [Value::fact(positive)]);
    }
}
