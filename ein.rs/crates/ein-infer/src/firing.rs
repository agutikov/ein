//! Firing — `:assert` substitution, the derived fact, and the key that says
//! "this application already happened".
//!
//! One `Firing` is one rule application. A `:assert (and …)` concludes several
//! facts from it (A13 multi-assert) and they **share one** [`Prov`]: one rule
//! application fanning out to N derived nodes, not N applications.

use ein_core::repr::{ReprValue, repr, repr_str};
use ein_core::{FactId, Kb, NafRef, Overflow, Prov, ProvId, Symbol, Terms, Value};
use ein_ir::{Ast, Node};

use crate::plan::{Plan, Reg, Slot};

/// The bindings a firing is built from — a register-file snapshot plus the
/// trail that gives it an order.
///
/// Borrowed rather than owned because both producers have it already: the
/// matcher hands out a live view, and the saturator's queue entry holds a
/// snapshot it took at enqueue time.
#[derive(Clone, Copy)]
pub struct Env<'a> {
    pub regs: &'a [Value],
    /// Bound registers in **bind order** — what `Provenance.bindings` records.
    pub trail: &'a [Reg],
    pub premises: &'a [FactId],
}

impl<'a> Env<'a> {
    pub fn bindings(&self, plan: &'a Plan) -> impl Iterator<Item = (Symbol, Value)> + 'a {
        let (regs, trail) = (self.regs, self.trail);
        trail
            .iter()
            .map(move |&r| (plan.reg_names[r as usize], regs[r as usize]))
    }

    /// `{'name': value, …}` in bind order — what the unbound-`:assert`-var
    /// error quotes.
    fn dict_repr(&self, terms: &Terms, plan: &Plan) -> String {
        let items: Vec<String> = self
            .bindings(plan)
            .map(|(name, value)| {
                format!(
                    "{}: {}",
                    repr_str(terms.sym(name)),
                    repr(&terms.repr_value(value))
                )
            })
            .collect();
        format!("{{{}}}", items.join(", "))
    }
}

/// One rule application — successful or redundant.
///
/// `derived` holds the new facts, or (when `redundant`) the pre-existing ones
/// the matcher would have re-derived. The matcher still produced the binding,
/// which is pedagogically relevant for the trace; no second insertion happens.
#[derive(Clone, Debug)]
pub struct Firing {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
    pub bindings: Box<[(Symbol, Value)]>,
    pub derived: Box<[FactId]>,
    pub premises: Box<[FactId]>,
    pub redundant: bool,
}

/// What stops a firing that should never have been attempted.
///
/// **`UnboundVar` was a third variant until M1f S1f.5.18**, *unbound var
/// ?{name} in :assert — bindings: …*, which `ein saturate` printed as
/// `KeyError: "…"`: a conclusion read a register its match left unbound. Its
/// doc called it an invariant the matcher guaranteed, and the loader did not
/// check it, so a run met it at whichever node first fired such a rule, until
/// S1f.5.10 refused at load a conclusion variable no premise or parameter binds
/// (F13, W57). One path was left, a parameter its activator bound to a nested
/// fact, which the compiler did not seed (W33's cause), read only by a
/// conclusion (`W58/parameter-nested-assert`); S1f.5.18 seeds it and retired
/// the variant (Q-M1f.37). What holds it since is `resolve`'s assertion,
/// and `ein-ir`'s `fuzz_properties::no_firing_meets_an_unbound_variable`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FireError {
    /// A conclusion no fact can hold: a top-level `:assert` that is not a
    /// fact, or an argument that is not a value — a relation written as a
    /// variable the activator does not bind (a keyword pair, a range and a
    /// wildcard too, until M1f S1f.5.5's swap refused each at the parse).
    /// One line in Ein's spelling, naming the rule and the slot as
    /// written (`firing::not_a_fact`). It was *TypeError: expected
    /// NestedPattern at :assert top-level, got …* and *TypeError: an
    /// unrecognised `:assert` slot has no value to store* until M1e2
    /// S1e2.2.18, which also stopped a string reaching it.
    NotAFact(String),
    /// The interner filled up. Structurally possible, never seen.
    Overflow(Overflow),
}

impl std::fmt::Display for FireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FireError::NotAFact(m) => f.write_str(m),
            FireError::Overflow(o) => write!(f, "{o}"),
        }
    }
}

impl From<Overflow> for FireError {
    fn from(o: Overflow) -> FireError {
        FireError::Overflow(o)
    }
}

/// Build one conclusion from its template, without writing it.
///
/// The saturator builds every conclusion *before* deciding whether the firing
/// is productive — that is what makes the redundancy check cheap — so this is
/// deliberately separate from [`fire`].
pub fn build_fact(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    template: Slot,
) -> Result<FactId, FireError> {
    let Slot::Nested { rel, slots } = template else {
        return Err(not_a_fact(terms, ast, plan, template, None));
    };
    let mut args: Vec<Value> = Vec::with_capacity(slots.len());
    for i in slots.range() {
        let slot = plan.slots[i];
        // Storing the node itself would put an IR node in a fact's args — a
        // shape nothing downstream can read — so this refuses.
        if let Slot::Opaque(_) = slot {
            return Err(not_a_fact(terms, ast, plan, slot, Some(rel)));
        }
        args.push(resolve(terms, ast, plan, env, slot)?);
    }
    Ok(terms.intern_fact(rel, &args)?)
}

/// Build one conclusion as the KB takes it: the fact it states, or for an
/// `eq` / `neq` conclusion what its definition makes of it.
///
/// `eq` and `neq` are relations whose facts are computed and never stored
/// (M1f S1f.5.7, the user's ruling of 2026-11-07 on G-21), so a rule that
/// concludes one is checked: one that holds, `(neq a b)`, adds nothing, and is
/// `None`; one that does not, `(neq a a)` or `(eq a w)`, cannot hold, and is
/// `(false)`, so its state is a contradiction, explained as one a rule
/// concluding `(false)` is — its core the premises the firing matched, and
/// `refuted by` the rule. Until S1f.5.7 the conclusion was stored as an
/// ordinary fact, so a false one sat in the KB while a guard computed the
/// opposite, and killed nothing (W30).
pub fn build_conclusion(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    template: Slot,
) -> Result<Option<FactId>, FireError> {
    if let Some((eq, x, y)) = computed(terms, plan, template) {
        let x = resolve(terms, ast, plan, env, x)?;
        let y = resolve(terms, ast, plan, env, y)?;
        if (x == y) == eq {
            return Ok(None);
        }
        let r#false = terms.kernel.r#false;
        return Ok(Some(terms.intern_fact(r#false, &[])?));
    }
    build_fact(terms, ast, plan, env, template).map(Some)
}

/// A conclusion of a relation whose facts are computed: `(eq x y)`, `(neq x
/// y)`, and `(not P)` over one of them, which is the other (Q-M1f.31) —
/// whether it is `eq` the template states, and the two sides.
pub fn computed(terms: &Terms, plan: &Plan, template: Slot) -> Option<(bool, Slot, Slot)> {
    let Slot::Nested { rel, slots } = template else {
        return None;
    };
    let k = &terms.kernel;
    if rel == k.not && slots.len() == 1 {
        let (eq, x, y) = computed(terms, plan, plan.slots[slots.start as usize])?;
        return Some((!eq, x, y));
    }
    if (rel == k.eq || rel == k.neq) && slots.len() == 2 {
        let at = slots.start as usize;
        return Some((rel == k.eq, plan.slots[at], plan.slots[at + 1]));
    }
    None
}

/// The line a conclusion no fact can hold stops the run with, in Ein's
/// spelling: the rule, and the slot as the program writes it. `within` is the
/// relation whose argument it is, or `None` for a top-level conclusion.
fn not_a_fact(
    terms: &Terms,
    ast: &Ast,
    plan: &Plan,
    slot: Slot,
    within: Option<Symbol>,
) -> FireError {
    let rule = terms.sym(plan.rule);
    let written = match slot {
        Slot::Const(v) => terms.display(v),
        Slot::Reg(r) => format!("?{}", terms.sym(plan.reg_names[r as usize])),
        Slot::Nested { .. } => String::from("a fact"),
        Slot::Opaque(node) => ein_ir::dump::dump_compact(ast, node),
    };
    let unbound_head = match slot {
        Slot::Opaque(node) => match ast.node(node) {
            Node::SForm { head, .. } => Some(ein_ir::dump::dump_compact(ast, head)),
            _ => None,
        },
        _ => None,
    };
    FireError::NotAFact(match (unbound_head, within) {
        (Some(head), _) => format!(
            "rule '{rule}': its :assert writes {written}, whose relation {head} \
             its activator does not bind — a relation written as a variable is \
             one of the rule's parameters, which its activator binds to a name"
        ),
        // An argument no fact can hold, `_` or a keyword pair: refused at
        // load until M1e2 S1e2.2.18, and with a line here (*writes … as an
        // argument of …, and no fact can hold it*) until M1f S1f.5.5's T5.
        // Since the swap a conclusion's argument is a name, a variable or a
        // fact (`ein.y`'s `Term`), and a macro's body holds the same, so no
        // slot reaches here as one but a relation variable, the arm above.
        (None, Some(rel)) => unreachable!(
            "rule '{rule}': {written} as an argument of {} — the grammar's Term holds a name, \
             a variable or a fact",
            terms.sym(rel)
        ),
        (None, None) => format!(
            "rule '{rule}': its :assert concludes {written}, which is not a fact \
             — a conclusion is a fact (relation argument …), or an (and …) of \
             facts"
        ),
    })
}

fn resolve(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    slot: Slot,
) -> Result<Value, FireError> {
    match slot {
        Slot::Const(v) => Ok(v),
        Slot::Reg(r) => {
            let v = env.regs[r as usize];
            // No conclusion reads a register its match and its activator leave
            // unbound: the loader refuses a conclusion variable no premise or
            // parameter binds (M1f S1f.5.10, `from_ir::refuse_unbound_conclusions`),
            // and the compiler seeds every parameter from its activator, a
            // nested fact too (S1f.5.18, `compile::Compiler::bind_activator`),
            // which retired `FireError::UnboundVar` and its `KeyError` line.
            assert!(
                !v.is_unbound(),
                "rule '{}' concludes ?{} unbound — bindings: {}; the loader binds every \
                 conclusion variable and the compiler seeds every parameter",
                terms.sym(plan.rule),
                terms.sym(plan.reg_names[r as usize]),
                env.dict_repr(terms, plan)
            );
            Ok(v)
        }
        Slot::Nested { .. } => Ok(Value::fact(build_fact(terms, ast, plan, env, slot)?)),
        // `build_fact` refuses an opaque argument before it gets here.
        Slot::Opaque(_) => Err(not_a_fact(terms, ast, plan, slot, None)),
    }
}

/// Build every conclusion, write each, and record the application.
///
/// `absent` are the `(absent …)` queries that had to fail on the
/// closure/world boundary for this firing to be admitted (S1.21.8). They ride
/// on the **same** provenance as the positive premises, so a conclusion
/// records the whole of what it depends on — positive and negative — rather
/// than only the facts it consumed. `universal` are the facts its
/// universals read ([`ein_core::Prov::universal`], M1e2 S1e2.2.21).
pub fn fire(
    kb: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    absent: Box<[NafRef]>,
    universal: Box<[FactId]>,
) -> Result<Option<Firing>, FireError> {
    if plan.asserts.is_empty() {
        return Ok(None);
    }
    let bindings: Box<[(Symbol, Value)]> = env.bindings(plan).collect();
    let prov = new_prov(terms, plan, env, bindings.clone(), absent, universal);
    let mut derived: Vec<FactId> = Vec::with_capacity(plan.asserts.len());
    for i in 0..plan.asserts.len() {
        // A true `eq` / `neq` conclusion adds nothing (M1f S1f.5.7, G-21).
        let Some(id) = build_conclusion(terms, ast, plan, env, plan.asserts[i])? else {
            continue;
        };
        let (rel, args) = terms.facts.get(id);
        let args = args.to_vec();
        // Dedup is by `(relation, args)`, so an already-known conclusion is
        // returned and indexed once — a *partially* novel multi-assert still
        // records every fact.
        let added = kb.add_and_index_fact(terms, rel, &args, Some(prov))?;
        derived.push(added.id());
    }
    if derived.is_empty() {
        // Every conclusion an `eq` / `neq` that holds: no firing (the
        // saturator does not reach here with one, `Saturator::apply`).
        return Ok(None);
    }
    Ok(Some(Firing {
        rule: plan.rule,
        activator: plan.activator_args.clone(),
        bindings,
        derived: derived.into_boxed_slice(),
        premises: env.premises.into(),
        redundant: false,
    }))
}

/// One `Prov` per application, shared by every conclusion.
pub fn new_prov(
    terms: &mut Terms,
    plan: &Plan,
    env: Env<'_>,
    bindings: Box<[(Symbol, Value)]>,
    absent: Box<[NafRef]>,
    universal: Box<[FactId]>,
) -> ProvId {
    let mut prov = Prov::from_rule(plan.rule, env.premises.into(), None);
    prov.bindings = bindings;
    prov.absent = absent;
    prov.activator = plan.activator;
    prov.universal = universal;
    terms.provs.push(prov)
}

/// The identity of a rule application: `(rule, activator, bindings)`.
///
/// Keyed on the set of `(name, value)` bindings, it would be a hashed set of
/// pairs built per match — 445 k of them on an exhaustive `zebra2`. Here it is
/// the **register file**, which is the same information: registers are a bijection with variable names within a plan, an
/// unbound one carries a sentinel no `Value` can forge, and the vector's order
/// is fixed by the plan rather than by the match.
///
/// `activator` is an id the engine interns for the plan's compile key,
/// [`crate::compile::PlanKey::activator`] — **every** argument of the
/// activator, as a value — which keeps a key two words plus one small vector.
/// Until M1e2 S1e2.2.16 that list was each argument's text, so the int `1`
/// and the string `"1"` were one id and one plan, and the second activator's
/// conclusions were lost (W16).
/// So within one engine an id names one plan, and the plans that share
/// `(rule, activator)` are one plan with one register layout, the invariant a
/// value vector needs to be comparable. `Engine::check_layout` asserts it in a
/// debug build.
///
/// **Until M1e2 S1e2.2.7 the id interned `plan.activator_args`**, the
/// activator's **symbol** arguments. An `int` binds its parameter, so it
/// seeds a register and arrives in `values`, and two activators differing
/// only in one had two keys (M1e
/// [S1e.1.4](../../../../docs/history/m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3),
/// which refuted Q-M1a.8's claim with
/// `rule_semantics::activators_differing_only_by_an_int_argument_both_fire`).
/// A nested `Fact` binds nothing and reached neither half: two activators
/// differing only there shared a key, and the dropped application was a
/// duplicate. But an `int` in the position another activator gives a nested
/// `Fact` put two plans with **two register layouts** in one space, a release
/// build compared them as one, and a derivation was lost with no diagnostic
/// ([Q-M1e.16](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one),
/// M1e2's W7). The id is the compile key's list since, which was Q-M1e.16's
/// first fix: `rule_semantics::an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`
/// is its regression, and two nested-`Fact` activators now each fire, the
/// second redundantly (`…_differing_only_by_a_nested_fact_argument_both_fire`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BindingKey {
    pub rule: Symbol,
    pub activator: ActivatorId,
    pub values: Box<[Value]>,
}

/// An interned compile key's argument list, [`crate::compile::PlanKey::activator`]
/// — every argument of the activator, since M1e2 S1e2.2.7, and as values, not
/// their text, since S1e2.2.16.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ActivatorId(pub u32);

impl BindingKey {
    pub fn new(plan: &Plan, activator: ActivatorId, regs: &[Value]) -> BindingKey {
        ein_core::counters::bump(|c| c.binding_key += 1);
        BindingKey {
            rule: plan.rule,
            activator,
            values: regs[..plan.n_regs as usize].into(),
        }
    }
}

/// Each binding's display text — what the trace prints. Kept as a function rather than done at record time,
/// because not doing it is the whole point: the redundant-firing path is the
/// hottest in the engine and stringifying there is the most expensive part of
/// recording ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §7).
pub fn rendered_bindings(terms: &Terms, bindings: &[(Symbol, Value)]) -> Vec<(String, String)> {
    bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
        .collect()
}

/// The `ReprValue` rendering of a whole binding environment — for the error
/// paths and the event log.
pub fn bindings_repr(terms: &Terms, bindings: &[(Symbol, Value)]) -> Vec<(String, ReprValue)> {
    bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.repr_value(*v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::Plan;
    use ein_core::Kb;
    use std::ops::ControlFlow;

    fn setup(src: &str) -> (ein_ir::Ast, Terms, Kb, Plan) {
        let mut ast = ein_ir::Ast::new();
        let mut terms = Terms::new();
        let forms = ein_ir::parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = ein_ir::from_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let plan = crate::compile_rule(&ast, &mut terms, &rule, None).expect("compiles");
        (ast, terms, kb, plan)
    }

    /// One rule application, N conclusions, **one** provenance — A13's
    /// multi-assert is one node fanning out, not N applications.
    #[test]
    fn a_multi_assert_shares_one_provenance() {
        let (ast, mut terms, mut kb, plan) = setup(
            "(relation a Thing)\n(relation b Thing)\n\
             (rule r ()\n  :match (a ?x)\n  :assert (and (b ?x) (not (a ?x))))\n\
             (a thing1)\n",
        );
        let mut matcher = crate::Matcher::new();
        let mut envs: Vec<(Vec<Value>, Vec<Reg>, Vec<FactId>)> = Vec::new();
        matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
            envs.push((
                m.regs().to_vec(),
                m.bindings().map(|_| 0).collect::<Vec<Reg>>(),
                m.premises().to_vec(),
            ));
            ControlFlow::Continue(())
        });
        assert_eq!(envs.len(), 1);
        let (regs, _, prems) = &envs[0];
        let trail: Vec<Reg> = (0..plan.n_regs).collect();
        let env = Env {
            regs,
            trail: &trail,
            premises: prems,
        };
        let firing = fire(
            &mut kb,
            &mut terms,
            &ast,
            &plan,
            env,
            Box::new([]),
            Box::new([]),
        )
        .expect("fires")
        .expect("has an assert");
        assert_eq!(firing.derived.len(), 2);
        let provs: Vec<_> = firing
            .derived
            .iter()
            .map(|&f| kb.primary(f).expect("recorded"))
            .collect();
        assert_eq!(provs[0], provs[1], "both conclusions share one Prov");
        let prov = terms.provs.get(provs[0]);
        assert_eq!(prov.premises.as_ref(), prems.as_slice());
        assert!(!prov.bindings.is_empty());
    }

    /// A conclusion writes a parameter its activator binds to a nested fact,
    /// as one bound to a name: M1f S1f.5.18 (W58). Until then the compiler did
    /// not seed the parameter, and the conclusion read an unbound register,
    /// which stopped the run with `FireError::UnboundVar`'s *unbound var ?never
    /// in :assert — bindings: {'x': 'thing1'}*, the line this test held, the
    /// plan compiled with no activator; since S1f.5.18 that variant is gone,
    /// and no driver compiles a rule with parameters but from an activator.
    #[test]
    fn a_parameter_bound_to_a_nested_fact_is_concluded() {
        let src = "(relation a Thing)\n(relation b A B)\n\
                   (rule r (?then)\n  :match (a ?x)\n  :assert (b ?x ?then))\n\
                   (a thing1)\n(r (f thing2))\n";
        let mut ast = ein_ir::Ast::new();
        let mut terms = Terms::new();
        let forms = ein_ir::parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = ein_ir::from_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let activator = crate::activators_for(&kb, &terms, &rule)[0];
        let plan = crate::compile_rule(&ast, &mut terms, &rule, activator).expect("compiles");
        assert_eq!(
            plan.activator_args
                .iter()
                .map(|&v| terms.display(v))
                .collect::<Vec<_>>(),
            ["(f thing2)"],
            "the activator's every argument"
        );
        let mut matcher = crate::Matcher::new();
        let mut envs: Vec<(Vec<Value>, Vec<Reg>)> = Vec::new();
        matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
            envs.push((m.regs().to_vec(), Vec::new()));
            std::ops::ControlFlow::Continue(())
        });
        assert_eq!(envs.len(), 1, "one match, of (a thing1)");
        let (regs, trail) = &envs[0];
        let env = Env {
            regs,
            trail,
            premises: &[],
        };
        let fact = build_fact(&mut terms, &ast, &plan, env, plan.asserts[0]).expect("a fact");
        assert_eq!(terms.fact_text(fact), "(b thing1 (f thing2))");
    }

    /// The key is the register file, and it is the set of bindings by another
    /// name: same bindings collide, a different value does not, and an
    /// *unbound* register is distinct from every bound one.
    #[test]
    fn a_binding_key_is_the_binding_set_by_another_name() {
        let (_ast, mut terms, _kb, plan) = setup(
            "(relation a Thing)\n(relation b A B)\n\
             (rule r ()\n  :match (a ?x)\n  :assert (b ?x ?x))\n",
        );
        let n = plan.n_regs as usize;
        let (v1, v2) = (
            terms.value_text("one").expect("room"),
            terms.value_text("two").expect("room"),
        );
        let mk = |v: Value| {
            let mut regs = vec![Value::UNBOUND; n];
            regs[0] = v;
            BindingKey::new(&plan, ActivatorId(0), &regs)
        };
        assert_eq!(mk(v1), mk(v1));
        assert_ne!(mk(v1), mk(v2));
        let unbound = BindingKey::new(&plan, ActivatorId(0), &vec![Value::UNBOUND; n]);
        assert_ne!(mk(v1), unbound);
        // …and a different activator is a different application, which is what
        // keeps two activators of one rule from masking each other.
        let other = BindingKey::new(&plan, ActivatorId(1), &{
            let mut regs = vec![Value::UNBOUND; n];
            regs[0] = v1;
            regs
        });
        assert_ne!(mk(v1), other);
    }
}
