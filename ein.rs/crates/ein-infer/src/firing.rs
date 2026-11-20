//! Firing — `:assert` substitution, the derived fact, and the key that says
//! "this application already happened".
//!
//! One `Firing` is one rule application. A `:assert (and …)` concludes several
//! facts from it (a conjunction) and they **share one** [`Prov`]: one rule
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
///
/// **An application of a rule that states `:ein inconsistent` derives no
/// fact** ([`Firing::ends`], Q-M1f.399): `derived` is empty, and what it did
/// is end the state ([`end`]). Every other firing derives at least one.
#[derive(Clone, Debug)]
pub struct Firing {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
    pub bindings: Box<[(Symbol, Value)]>,
    pub derived: Box<[FactId]>,
    pub premises: Box<[FactId]>,
    pub redundant: bool,
    /// The rule states `:ein inconsistent`: the firing ended the state and
    /// wrote no fact; `redundant` where the KB already held the record of
    /// the same application.
    pub ends: bool,
}

/// What stops a firing that should never have been attempted.
///
/// **No conclusion reads a register its match left unbound**, so there is no
/// variant for one: the loader refuses a conclusion variable no premise or
/// parameter binds, and the compiler seeds every parameter from its
/// activator, a nested fact too (`parameter-nested-assert`). What
/// holds it is `resolve`'s assertion, and `ein-ir`'s
/// `fuzz_properties::no_firing_meets_an_unbound_variable`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FireError {
    /// A conclusion no fact can hold: a top-level `:assert` that is not a
    /// fact, or an argument that is not a value — a relation written as a
    /// variable the activator does not bind (a keyword pair, a range, a
    /// wildcard and a string are refused at the parse). One line in Ein's
    /// spelling, naming the rule and the slot as written
    /// (`firing::not_a_fact`).
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
///
/// **`None` is the negation of a name**, which is no fact (under the value
/// rule: *an object is an atom, or a fact a state could hold: a fact of
/// an ordinary relation, or the stored negation of a fact a state could
/// hold*). A conclusion `(not ?x)` is of a variable bound to a fact, a
/// stored negation among them, whose negation is a fact too (Q-M1f.391);
/// where a binding gives it a name there is nothing to conclude, so the
/// firing stores nothing for it, as a true `eq` conclusion adds nothing
/// ([`build_conclusion`]). The loader refuses the
/// shape wherever the program shows the variable can hold a name
/// (`ein_ir::from_ir`'s `refuse_negated_names`); this is what is left, a
/// binding only a run meets (Q-M1f.128). Stored, `(not k)` would be a fact
/// no program can write and no state file can reload.
pub fn build_fact(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    template: Slot,
) -> Result<Option<FactId>, FireError> {
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
        let Some(value) = resolve(terms, ast, plan, env, slot)? else {
            return Ok(None);
        };
        args.push(value);
    }
    if rel == terms.kernel.not && args.len() == 1 && args[0].as_fact().is_none() {
        return Ok(None);
    }
    Ok(Some(terms.intern_fact(rel, &args)?))
}

/// Build one conclusion as the KB takes it: the fact it states, or for an
/// `eq` / `neq` conclusion what its definition makes of it.
///
/// `eq` and `neq` are relations whose facts are computed and never stored
/// (Q-M1e2.26), so a rule that
/// concludes one is checked: one that holds, `(neq a b)`, adds nothing, and is
/// `None`; one that does not, `(neq a a)` or `(eq a w)`, cannot hold, and is
/// `(false)`, so its state is a contradiction, explained as one a rule
/// concluding `(false)` is — its core the premises the firing matched, and
/// `refuted by` the rule. Stored as an ordinary fact, a false one would sit
/// in the KB while a guard computed the opposite, and kill nothing.
pub fn build_conclusion(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    template: Slot,
) -> Result<Option<FactId>, FireError> {
    if let Some((eq, x, y)) = computed(terms, plan, template) {
        // A side that holds the negation of a name is no value, and the
        // conclusion nothing; the grammar writes no such side.
        let (Some(x), Some(y)) = (
            resolve(terms, ast, plan, env, x)?,
            resolve(terms, ast, plan, env, y)?,
        ) else {
            return Ok(None);
        };
        if (x == y) == eq {
            return Ok(None);
        }
        let r#false = terms.kernel.r#false;
        return Ok(Some(terms.intern_fact(r#false, &[])?));
    }
    build_fact(terms, ast, plan, env, template)
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
        // An argument no fact can hold, `_` or a keyword pair, is refused at
        // the parse: a conclusion's argument is a name, a variable or a fact
        // (`ein.y`'s `Term`), and a macro's body holds the same, so no slot
        // reaches here as one but a relation variable, the arm above.
        (None, Some(rel)) => unreachable!(
            "rule '{rule}': {written} as an argument of {} — the grammar's Term holds a name, \
             a variable or a fact",
            terms.sym(rel)
        ),
        // A whole conclusion that is no fact, `?y` or a name (Q-M1f.351): only a macro's
        // body writes one (`(macro id (?x) ?x)` called as a conclusion). A
        // line here would stop only the firing that met it, so a rule only a
        // commitment activates would load and some commands would answer it.
        // The loader
        // refuses it, after expansion, with the same words (`ein_ir::from_ir`'s
        // `refuse_malformed_primitives`), so no program reaches it.
        (None, None) => unreachable!(
            "rule '{rule}': its :assert concludes {written}, which is not a fact — the grammar \
             and the loader's refuse_malformed_primitives hold a conclusion to be a form"
        ),
    })
}

/// A slot's value, or `None` where it holds the negation of a name
/// ([`build_fact`]).
pub(crate) fn resolve(
    terms: &mut Terms,
    ast: &Ast,
    plan: &Plan,
    env: Env<'_>,
    slot: Slot,
) -> Result<Option<Value>, FireError> {
    match slot {
        Slot::Const(v) => Ok(Some(v)),
        Slot::Reg(r) => {
            let v = env.regs[r as usize];
            // No conclusion reads a register its match and its activator leave
            // unbound: the loader refuses a conclusion variable no premise or
            // parameter binds (`from_ir::refuse_unbound_conclusions`), and
            // the compiler seeds every parameter from its activator, a nested
            // fact too (`compile::Compiler::bind_activator`).
            assert!(
                !v.is_unbound(),
                "rule '{}' concludes ?{} unbound — bindings: {}; the loader binds every \
                 conclusion variable and the compiler seeds every parameter",
                terms.sym(plan.rule),
                terms.sym(plan.reg_names[r as usize]),
                env.dict_repr(terms, plan)
            );
            Ok(Some(v))
        }
        Slot::Nested { .. } => Ok(build_fact(terms, ast, plan, env, slot)?.map(Value::fact)),
        // `build_fact` refuses an opaque argument before it gets here.
        Slot::Opaque(_) => Err(not_a_fact(terms, ast, plan, slot, None)),
    }
}

/// Build every conclusion, write each, and record the application.
///
/// `absent` are the `(absent …)` queries that had to fail on the
/// closure/world boundary for this firing to be admitted. They ride
/// on the **same** provenance as the positive premises, so a conclusion
/// records the whole of what it depends on — positive and negative — rather
/// than only the facts it consumed. `universal` are the facts its
/// universals read ([`ein_core::Prov::universal`]).
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
        // A true `eq` / `neq` conclusion adds nothing (Q-M1e2.26).
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
        ends: false,
    }))
}

/// Apply a rule that states `:ein inconsistent` (Q-M1f.399): record the
/// application on the KB, which is then inconsistent
/// ([`ein_core::Kb::is_inconsistent`]), and write no fact.
///
/// The record is what a derived fact's would be, the rule, the premises the
/// match read, `absent` and `universal` as [`fire`] takes them, so a core is
/// taken over it as over a derivation ([`mod@crate::explain`]). A KB that already
/// holds the record of this application keeps one, and the firing is
/// redundant.
pub fn end(
    kb: &mut Kb,
    terms: &mut Terms,
    plan: &Plan,
    env: Env<'_>,
    absent: Box<[NafRef]>,
    universal: Box<[FactId]>,
) -> Firing {
    debug_assert!(plan.ends, "only a rule that states the key ends a state");
    let bindings: Box<[(Symbol, Value)]> = env.bindings(plan).collect();
    let prov = new_prov(terms, plan, env, bindings.clone(), absent, universal);
    let new = kb.end(terms, prov);
    Firing {
        rule: plan.rule,
        activator: plan.activator_args.clone(),
        bindings,
        derived: Box::new([]),
        premises: env.premises.into(),
        redundant: !new,
        ends: true,
    }
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
/// activator, as a value, not its text — which keeps a key two words
/// plus one small vector. So within one engine an id names one plan, and the
/// plans that share
/// `(rule, activator)` are one plan with one register layout, the invariant a
/// value vector needs to be comparable. `Engine::check_layout` asserts it in a
/// debug build.
///
/// **The id keys on every activator argument** (Q-M1e.16): a key on part of
/// them would put two plans with **two register layouts** in one space,
/// where a release build compares them as one and a derivation is lost with
/// no diagnostic. `rule_semantics` holds it: activators differing
/// only by a name argument both fire
/// (`activators_differing_only_by_an_int_argument_both_fire`), two
/// nested-`Fact` activators each fire, the second redundantly
/// (`…_differing_only_by_a_nested_fact_argument_both_fire`), and a name beside
/// a nested `Fact` in one position loses no derivation
/// (`an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BindingKey {
    pub rule: Symbol,
    pub activator: ActivatorId,
    pub values: Box<[Value]>,
}

/// An interned compile key's argument list, [`crate::compile::PlanKey::activator`]
/// — every argument of the activator, as values, not their text.
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
/// recording ([03-design/03-data-model.md § 8](../../../../docs/ein/ein.rs/03-design/03-data-model.md#8-provenance)).
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
            "(relation a Thing :why \"a holds of {?3}\")\n(relation b Thing :why \"b holds of {?3}\")\n\
             (rule r ()\n  :match (a ?x)\n  :assert (and (b ?x) (not (a ?x))) :why \"a holds of {?x}, so b holds of {?x} and a does not\")\n\
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
    /// as one bound to a name: the compiler seeds the parameter, so
    /// the conclusion never reads an unbound register, and no driver compiles
    /// a rule with parameters but from an activator.
    #[test]
    fn a_parameter_bound_to_a_nested_fact_is_concluded() {
        let src = "(relation a Thing :why \"a holds of {?3}\")\n(relation b A B :why \"b holds of {?3} and {?4}\")\n\
                   (rule r (?then)\n  :match (a ?x)\n  :assert (b ?x ?then) :why \"a holds of {?x}, so b holds of {?x} and {?then}\")\n\
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
        let fact = build_fact(&mut terms, &ast, &plan, env, plan.asserts[0])
            .expect("a fact")
            .expect("of no negated name");
        assert_eq!(terms.fact_text(fact), "(b thing1 (f thing2))");
    }

    /// The key is the register file, and it is the set of bindings by another
    /// name: same bindings collide, a different value does not, and an
    /// *unbound* register is distinct from every bound one.
    #[test]
    fn a_binding_key_is_the_binding_set_by_another_name() {
        let (_ast, mut terms, _kb, plan) = setup(
            "(relation a Thing :why \"a holds of {?3}\")\n(relation b A B :why \"b holds of {?3} and {?4}\")\n\
             (rule r ()\n  :match (a ?x)\n  :assert (b ?x ?x) :why \"a holds of {?x}, so b holds of {?x} and {?x}\")\n",
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
