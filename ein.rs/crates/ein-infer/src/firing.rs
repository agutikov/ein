//! Firing — `:assert` substitution, the derived fact, and the key that says
//! "this application already happened".
//!
//! One `Firing` is one rule application. A `:assert (and …)` concludes several
//! facts from it (A13 multi-assert) and they **share one** [`Prov`]: one rule
//! application fanning out to N derived nodes, not N applications.

use ein_core::pyrepr::{PyValue, repr, repr_str};
use ein_core::{FactId, Kb, NafRef, Overflow, Prov, ProvId, Symbol, Terms, Value};

use crate::plan::{Plan, Reg, Slot};

/// The bindings a firing is built from — a register-file snapshot plus the
/// trail that gives it an order.
///
/// Borrowed rather than owned because both producers have it already: the
/// matcher hands out a live view, and the saturator's queue entry holds a
/// snapshot it took at enqueue time (ein.py's `dict(bindings)`).
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

    /// `str(bindings)` — the dict repr the unbound-`:assert`-var error quotes.
    fn dict_repr(&self, terms: &Terms, plan: &Plan) -> String {
        let items: Vec<String> = self
            .bindings(plan)
            .map(|(name, value)| {
                format!(
                    "{}: {}",
                    repr_str(terms.sym(name)),
                    repr(&terms.py_value(value))
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
    pub activator: Box<[Symbol]>,
    pub bindings: Box<[(Symbol, Value)]>,
    pub derived: Box<[FactId]>,
    pub premises: Box<[FactId]>,
    pub redundant: bool,
}

/// What stops a firing that should never have been attempted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FireError {
    /// ein.py raises `KeyError(f"unbound var ?{name} in :assert — bindings: …")`.
    /// The matcher guarantees no unbound var reaches an `:assert` template, so
    /// this is an invariant violation rather than a rejected input — which is
    /// why it is loud in both implementations.
    UnboundVar(String),
    /// `TypeError(f"expected NestedPattern at :assert top-level, got …")`.
    NotAFact(String),
    /// The interner filled up. Structurally possible, never seen.
    Overflow(Overflow),
}

impl std::fmt::Display for FireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FireError::UnboundVar(m) | FireError::NotAFact(m) => f.write_str(m),
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
    plan: &Plan,
    env: Env<'_>,
    template: Slot,
) -> Result<FactId, FireError> {
    let Slot::Nested { rel, slots } = template else {
        return Err(FireError::NotAFact(format!(
            "expected NestedPattern at :assert top-level, got {}",
            slot_type_name(template)
        )));
    };
    let mut args: Vec<Value> = Vec::with_capacity(slots.len());
    for i in slots.range() {
        args.push(resolve(terms, plan, env, plan.slots[i])?);
    }
    Ok(terms.intern_fact(rel, &args)?)
}

fn resolve(terms: &mut Terms, plan: &Plan, env: Env<'_>, slot: Slot) -> Result<Value, FireError> {
    match slot {
        Slot::Const(v) => Ok(v),
        Slot::Reg(r) => {
            let v = env.regs[r as usize];
            if v.is_unbound() {
                return Err(FireError::UnboundVar(format!(
                    "unbound var ?{} in :assert — bindings: {}",
                    terms.sym(plan.reg_names[r as usize]),
                    env.dict_repr(terms, plan)
                )));
            }
            Ok(v)
        }
        Slot::Nested { .. } => Ok(Value::fact(build_fact(terms, plan, env, slot)?)),
        // ein.py's `resolve_leaf` returns the node itself, which lands in
        // `Fact.args` as an IR node — a shape nothing downstream can read.
        // Refusing is the difference, and it is one the loader's validator
        // already makes unreachable.
        Slot::Opaque(_) => Err(FireError::NotAFact(
            "an unrecognised `:assert` slot has no value to store".to_string(),
        )),
    }
}

fn slot_type_name(slot: Slot) -> &'static str {
    match slot {
        // The name Python's `type(template).__name__` gives, for the shapes a
        // template can actually be.
        Slot::Reg(_) => "Var",
        Slot::Const(_) => "Atom",
        Slot::Nested { .. } => "NestedPattern",
        Slot::Opaque(_) => "SForm",
    }
}

/// Build every conclusion, write each, and record the application.
///
/// `absent` are the `(absent …)` queries that had to fail on the
/// closure/world boundary for this firing to be admitted (S1.21.8). They ride
/// on the **same** provenance as the positive premises, so a conclusion
/// records the whole of what it depends on — positive and negative — rather
/// than only the facts it consumed.
pub fn fire(
    kb: &mut Kb,
    terms: &mut Terms,
    plan: &Plan,
    env: Env<'_>,
    absent: Box<[NafRef]>,
) -> Result<Option<Firing>, FireError> {
    if plan.asserts.is_empty() {
        return Ok(None);
    }
    let bindings: Box<[(Symbol, Value)]> = env.bindings(plan).collect();
    let prov = new_prov(terms, plan, env, bindings.clone(), absent);
    let mut derived: Vec<FactId> = Vec::with_capacity(plan.asserts.len());
    for i in 0..plan.asserts.len() {
        let id = build_fact(terms, plan, env, plan.asserts[i])?;
        let (rel, args) = terms.facts.get(id);
        let args = args.to_vec();
        // Dedup is by `(relation, args)`, so an already-known conclusion is
        // returned and indexed once — a *partially* novel multi-assert still
        // records every fact.
        let added = kb.add_and_index_fact(terms, rel, &args, Some(prov))?;
        derived.push(added.id());
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
) -> ProvId {
    let mut prov = Prov::from_rule(plan.rule, env.premises.into(), None);
    prov.bindings = bindings;
    prov.absent = absent;
    terms.provs.push(prov)
}

/// The identity of a rule application: `(rule, activator, bindings)`.
///
/// ein.py's `_binding_key` is
/// `(rule_name, plan.activator_args, frozenset(bindings.items()))`, and it
/// costs 2.7 s over 445 k calls on an exhaustive `zebra2` — a frozenset of
/// tuples, built per match. Here it is the **register file**, which is the same
/// information: registers are a bijection with variable names within a plan, an
/// unbound one carries a sentinel no `Value` can forge, and the vector's order
/// is fixed by the plan rather than by the match.
///
/// `activator` is an id the engine interns for `plan.activator_args`, which
/// keeps a key two words plus one small vector. Note that `activator_args`
/// holds the activator's **string** arguments only, while the *compile* cache
/// key stringifies all of them — so two activators differing only in an `int`
/// argument share a binding key. That is Q-M1a.8, reproduced rather than fixed.
///
/// **The invariant this leans on:** all plans sharing `(rule, activator)` have
/// the same register layout, so their value vectors are comparable. It holds
/// because the layout is a function of the rule and of *which* parameters the
/// activator bound, and it is asserted in debug builds where the plan list is
/// built (`Engine::register`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BindingKey {
    pub rule: Symbol,
    pub activator: ActivatorId,
    pub values: Box<[Value]>,
}

/// An interned `plan.activator_args`.
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

/// `str(v)` for each binding — what `Provenance.bindings` stores in ein.py and
/// what the trace prints. Kept as a function rather than done at record time,
/// because not doing it is the whole point: the redundant-firing path is the
/// hottest in the engine and stringifying there is the most expensive part of
/// recording ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §7).
pub fn rendered_bindings(terms: &Terms, bindings: &[(Symbol, Value)]) -> Vec<(String, String)> {
    bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
        .collect()
}

/// The `PyValue` rendering of a whole binding environment — for the error
/// paths and the event log.
pub fn bindings_py(terms: &Terms, bindings: &[(Symbol, Value)]) -> Vec<(String, PyValue)> {
    bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.py_value(*v)))
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
        let firing = fire(&mut kb, &mut terms, &plan, env, Box::new([]))
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

    /// The matcher guarantees no unbound var reaches an `:assert`, so one that
    /// does is an invariant violation — and it says so with ein.py's text,
    /// bindings dict included.
    #[test]
    fn an_unbound_assert_var_quotes_the_bindings() {
        let (_ast, mut terms, _kb, plan) = setup(
            "(relation a Thing)\n(relation b A B)\n\
             (rule r ()\n  :match (a ?x)\n  :assert (b ?x ?never))\n",
        );
        let mut regs = vec![Value::UNBOUND; plan.n_regs as usize];
        let x = plan
            .reg_names
            .iter()
            .position(|&s| terms.sym(s) == "x")
            .expect("?x");
        regs[x] = terms.value_text("thing1").expect("room");
        let env = Env {
            regs: &regs,
            trail: &[x as Reg],
            premises: &[],
        };
        let err = build_fact(&mut terms, &plan, env, plan.asserts[0]).expect_err("unbound");
        assert_eq!(
            err.to_string(),
            "unbound var ?never in :assert — bindings: {'x': 'thing1'}"
        );
    }

    /// The key is the register file, and it is the frozenset ein.py builds:
    /// same bindings collide, a different value does not, and an *unbound*
    /// register is distinct from every bound one.
    #[test]
    fn a_binding_key_is_the_frozenset_by_another_name() {
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
