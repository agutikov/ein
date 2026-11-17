//! The per-`(rule, activator)` pattern compiler.
//!
//! Q29 picked *compile unit = per (rule, activator-binding) pair*: the
//! activator fact binds the rule's parameter list **before** matching begins,
//! so the compiler substitutes the parameters and bakes concrete relation
//! names into the program, with the slots lowered to [`Slot`]s and the
//! variables numbered
//! ([design/matcher.md § 2.1](../../../../docs/ein/ein.rs/design/matcher.md#21-the-compile-unit-and-its-activator)).
//!
//! The compiler is small and its **edge cases are semantics**. It never drops
//! a premise, because dropping one is unsound in one direction and incomplete
//! in the other: a plan whose premises all drop has no steps, which the
//! matcher accepts as one *vacuous* match, so the rule fires unconditionally.
//! A premise it cannot lower is a [`CompileError`] or, where the loader has
//! refused it first, an assertion.
//!
//! Two additions are pure metadata and change nothing about what a plan
//! matches: register numbering, and the candidate [`Probe`] list
//! ([`crate::plan`] documents both, including how static the probe can be).

use ein_core::entities::Rule;
use ein_core::{FactId, Kb, SlotKey, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::{Arc, Mutex};

use crate::plan::{
    Disjunct, GuardArg, GuardArgKind, MAX_REGS, NafGuard, Plan, PlanId, Probe, ProbeSrc, Reg,
    RelStep, Slot, Span, Step,
};
use crate::predicates;

/// A `:match` clause the compiler cannot lower to a faithful plan.
///
/// Each refuses a premise rather than drop it, or drop a whole disjunction.
/// The message is what a rule author sees, in Ein's words (Q-M1f.75): the
/// rule, the clause and the form as the program writes them, and where the
/// compiler knows the form's position, the file, the line and the column in
/// front, as a parse error places itself.
///
/// **No program reaches one.** The loader refuses a nested `(or …)`, a clause
/// past the register file and a goal's relation variable, and the grammar and
/// the loader's `refuse_malformed_primitives` hold `eq`'s and `absent`'s
/// arity, so those arms are assertions (Q-M1f.84). An activator of another
/// arity, a relation head bound to a fact, a rule's relation variable with no
/// activator, and a guard's argument that is a form holding a variable (which
/// the loader refuses in the same words, Q-M1f.30) are reached by a direct
/// caller of [`compile_rule`] only (Q-M1f.75).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompileError(pub String);

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CompileError {}

/// What a guard compares, as the loader's line and the compiler's say it
/// (Q-M1f.30).
pub const GUARD_VALUES: &str = "a guard compares two values, each a variable, a name, or a fact \
                                that holds no variable";

// ── Activators ─────────────────────────────────────────────────────

/// The activator facts authorising `rule` to apply — `Engine._activators_for`.
///
/// A parameter-less rule has *one* implicit activator, `None`: it applies once
/// over the KB. A parameterised one consults the **fork's**
/// `rule_apps_by_rule`, not the load-time KB's, because a fork derives
/// activators of its own during saturation, which the load-time KB's does not
/// hold.
///
/// Activators whose **arity** does not match the parameter list are
/// filtered out here. A rule name and a property relation may coincide (zebra2
/// derives the 1-ary marker `(total color-loc)` while `std.algebra`'s `total`
/// rule takes two parameters), and a fact that cannot bind the parameters does
/// not authorise anything. Passing it through left every parameter-headed
/// premise with an unbound head var, which the compiler then dropped —
/// silently, and in the vacuous direction.
pub fn activators_for(kb: &Kb, terms: &Terms, rule: &Rule) -> Vec<Option<FactId>> {
    let mut out = Vec::new();
    activators_into(kb, terms, rule, &mut out);
    out
}

/// The same, into a caller's buffer — which `Engine::compile_all` reuses
/// across every rule of the walk it repeats per hypgen call.
/// **Clears `out`**, so a caller may hand the same buffer over and over.
pub fn activators_into(kb: &Kb, terms: &Terms, rule: &Rule, out: &mut Vec<Option<FactId>>) {
    out.clear();
    if rule.params.is_empty() {
        out.push(None);
        return;
    }
    let n = rule.params.len();
    out.extend(
        kb.rule_apps_by_rule(rule.name)
            .filter(|&f| terms.facts.arity(f) == n)
            .map(Some),
    );
}

// ── The compile cache key ──────────────────────────────────────────

/// `(rule_name, the activator's arguments)` — the compile cache's key.
///
/// It keeps **all** the activator's arguments, as values, as
/// [`Plan::activator_args`] does (Q-M1f.43), and the engine interns *this*
/// list as [`crate::firing::BindingKey`]'s activator half, so the compile key
/// and the binding key are one list and a nested `Fact` reaches both
/// (Q-M1e.16); see `BindingKey`.
///
/// **A value, not its text**: two activators whose arguments are two values,
/// the names `7` and `07`, or a name and a nested fact, are two keys and two
/// plans, so neither loses its conclusions to the other (`W16`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlanKey {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
}

impl PlanKey {
    /// The key's arguments as text, each [`Terms::display`] of its value:
    /// what the `compile` event and the plan shape print.
    pub fn activator_text(&self, terms: &Terms) -> Vec<String> {
        self.activator.iter().map(|&v| terms.display(v)).collect()
    }
}

pub fn plan_key(terms: &Terms, rule: &Rule, activator: Option<FactId>) -> PlanKey {
    ein_core::counters::bump(|c| c.plan_key += 1);
    PlanKey {
        rule: rule.name,
        // The activator's arguments as the KB holds them. A fact's arguments
        // are interned values, so equal values are equal arguments, and
        // nothing here renders or interns.
        activator: match activator {
            None => Box::new([]),
            Some(f) => terms.facts.args(f).into(),
        },
    }
}

/// The **process-wide** `(rule, activator) → PlanId` memo
/// ([design/matcher.md § 7](../../../../docs/ein/ein.rs/design/matcher.md#7-the-compile-cache)).
///
/// A plan is a pure function of its key, so compiling it twice is waste: an
/// exhaustive `zebra2` makes 253 440 `compile_for` calls of which all but 19
/// are hits, plus 102 fresh `Engine`s that each recompile ~170 pairs from
/// scratch. Sharing the *plans* across forks removes that; what a fork may
/// **not** share is the compile cache's **iteration order**, which
/// the saturator's full enqueue pass walks — so each engine keeps its own ordered
/// `Vec<PlanId>` and this holds only the plans.
///
/// Append-only, which is what will let [design/parallelism.md](../../../../docs/ein/ein.rs/design/parallelism.md)
/// put it behind a shared lock without any invalidation story.
#[derive(Default, Debug)]
pub struct PlanMemo {
    /// `Arc` because a plan outlives the borrow of the memo that produced it:
    /// the saturator holds one while it mutates the engine around it, and
    /// [design/parallelism.md](../../../../docs/ein/ein.rs/design/parallelism.md) will
    /// share the same plans across threads without copying them.
    plans: Vec<Arc<Plan>>,
    by_key: FxHashMap<PlanKey, PlanId>,
}

impl PlanMemo {
    pub fn new() -> PlanMemo {
        PlanMemo::default()
    }

    pub fn get(&self, id: PlanId) -> &Plan {
        &self.plans[id.0 as usize]
    }

    /// A handle that outlives the borrow — see `PlanMemo::plans`.
    pub fn get_arc(&self, id: PlanId) -> Arc<Plan> {
        Arc::clone(&self.plans[id.0 as usize])
    }

    pub fn lookup(&self, key: &PlanKey) -> Option<PlanId> {
        self.by_key.get(key).copied()
    }

    pub fn len(&self) -> usize {
        self.plans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// The plan for one pair, compiling it if this process has not seen it.
    pub fn intern(
        &mut self,
        ast: &Ast,
        terms: &mut Terms,
        rule: &Rule,
        activator: Option<FactId>,
    ) -> Result<PlanId, CompileError> {
        let key = plan_key(terms, rule, activator);
        self.intern_keyed(key, ast, terms, rule, activator)
    }

    /// The same, for a caller that already has the key.
    ///
    /// [`Engine::compile_for`](crate::engine::Engine::compile_for) computes it
    /// to probe its own cache first, so recomputing it here would copy the
    /// activator's arguments again for an answer already in hand
    /// ([design/matcher.md § 7](../../../../docs/ein/ein.rs/design/matcher.md#7-the-compile-cache)).
    pub fn intern_keyed(
        &mut self,
        key: PlanKey,
        ast: &Ast,
        terms: &mut Terms,
        rule: &Rule,
        activator: Option<FactId>,
    ) -> Result<PlanId, CompileError> {
        if let Some(&id) = self.by_key.get(&key) {
            return Ok(id);
        }
        let plan = compile_rule(ast, terms, rule, activator)?;
        let id = PlanId(self.plans.len() as u32);
        self.plans.push(Arc::new(plan));
        self.by_key.insert(key, id);
        Ok(id)
    }
}

/// The memo, shared by every [`Engine`](crate::engine::Engine) of one run.
///
/// One handle lives on the [`Session`](crate::saturator::Session), so a fork's
/// saturator and every other engine of the run compile into the same table —
/// ([design/saturation.md § 4](../../../../docs/ein/ein.rs/design/saturation.md#4-a-node-resumes-roots-saturation)).
/// A `Mutex` rather than a `RefCell` because `Terms` is `Send + Sync` for
/// exactly this reason: the worker threads share the plans
/// ([design/parallelism.md § 3](../../../../docs/ein/ein.rs/design/parallelism.md#3-what-must-be-sync-and-how)).
/// The lock is taken **only on
/// an engine cache miss**, never on the read path — [`crate::Engine`] holds its own
/// `Arc<Plan>` per cached pair.
pub type SharedMemo = Arc<Mutex<PlanMemo>>;

// ── Compile ────────────────────────────────────────────────────────

/// Compile one `(rule, activator)` pair.
///
/// `activator` is `None` for a parameter-less rule such as
/// `type-exclusivity`; for a parameterised one the activator's arguments bind
/// the parameters positionally.
pub fn compile_rule(
    ast: &Ast,
    terms: &mut Terms,
    rule: &Rule,
    activator: Option<FactId>,
) -> Result<Plan, CompileError> {
    ein_core::counters::bump(|c| c.plan_compile += 1);
    Compiler::new(ast, terms).run(rule, activator)
}

/// The fact a form that holds no variable spells, interned as the loader
/// interns a fact a file writes: a name an argument, a form a nested fact.
/// `None` for a form with a variable, a string or a keyword in it at any
/// depth, a form whose head is no name, or a table that cannot number a new
/// proposition.
///
/// For a reader that asks what a rule's conclusion would write before any
/// state holds it: the obligations rung, of an activator a commitment can
/// derive ([`crate::oblgen`]).
pub(crate) fn ground_fact(ast: &Ast, terms: &mut Terms, node: NodeId) -> Option<FactId> {
    fn value(ast: &Ast, terms: &mut Terms, node: NodeId) -> Option<Value> {
        match ast.node(node) {
            Node::Atom(s) => terms.intern_text(ast.sym(s)).ok().map(Value::sym),
            Node::SForm { .. } => ground_fact(ast, terms, node).map(Value::fact),
            _ => None,
        }
    }
    let Node::SForm { head, args } = ast.node(node) else {
        return None;
    };
    let Node::Atom(h) = ast.node(head) else {
        return None;
    };
    let rel = terms.intern_text(ast.sym(h)).ok()?;
    let mut values = Vec::new();
    for &a in ast.args(args) {
        values.push(value(ast, terms, a)?);
    }
    terms.intern_fact(rel, &values).ok()
}

/// One register space: names in allocation order, and where each is projected
/// from when the space belongs to a guard sub-plan.
#[derive(Default)]
struct Space {
    names: Vec<Symbol>,
    index: FxHashMap<Symbol, Reg>,
    /// `None` everywhere in a plan space; in a sub-plan space, the parent
    /// register a scope variable is projected from.
    scope_of: Vec<Option<Reg>>,
}

impl Space {
    fn get(&self, name: Symbol) -> Option<Reg> {
        self.index.get(&name).copied()
    }

    /// A new register, past the last.
    ///
    /// **The file holds [`MAX_REGS`]**, and the loader refuses a `:match` or a
    /// `:goal` that writes more distinct variables than that, a rule's
    /// parameters among them (`ein_ir::from_ir`'s `refuse_wide_matches`,
    /// `W63`), which bounds each register file a plan has. So no program
    /// reaches the bound here, and it is an invariant (Q-M1f.84): a rule no
    /// state compiles would never meet a compiler's error.
    fn push(&mut self, name: Symbol, from: Option<Reg>) -> Result<Reg, CompileError> {
        assert!(
            self.names.len() < MAX_REGS,
            "a register file holds {MAX_REGS} variables, and the loader refuses a :match or a \
             :goal that writes more (ein_ir::from_ir's refuse_wide_matches)"
        );
        let reg = self.names.len() as Reg;
        self.names.push(name);
        self.scope_of.push(from);
        self.index.insert(name, reg);
        Ok(reg)
    }
}

struct Compiler<'a> {
    ast: &'a Ast,
    terms: &'a mut Terms,
    // Who a refusal names, `rule 'r'` or `query`, and its clause, `:match` or
    // `:goal`: set by `run`.
    who: String,
    clause: &'static str,
    // Arenas.
    steps: Vec<Step>,
    slots: Vec<Slot>,
    guards: Vec<NafGuard>,
    probes: Vec<Probe>,
    shared: Vec<Symbol>,
    guard_args: Vec<GuardArg>,
    guard_keys: Vec<u32>,
    // The plan's register space, seeds first.
    plan_space: Space,
    seed: Vec<(Symbol, Value)>,
    // The guard sub-plan currently being compiled, with the scope it projects.
    sub: Option<Space>,
    sub_scope: FxHashSet<Symbol>,
    // `known_vars` — one set for the whole disjunct, polluted by
    // `(absent …)` sub-plans too.
    known: FxHashSet<Symbol>,
    // `split_naf`'s `bound` — the variables of the *positive* premises seen so
    // far, seeded with the rule's parameters. A guard's scope is a snapshot.
    bound: FxHashSet<Symbol>,
    // The conjunctions being compiled, innermost last: what each has bound so
    // far, what its relation premises bind, and the `eq` / `neq` guards
    // waiting for one of those ([`Seq`]).
    seqs: Vec<Seq>,
}

/// One conjunction being compiled — a disjunct's body, or an `(absent …)`'s —
/// and where its `eq` / `neq` guards go: a premise means the same in any
/// order of its conjuncts (Q-M1e2.26).
///
/// A guard whose every variable is bound where it is written is a test there.
/// One
/// with a variable a **relation premise of the same conjunction** binds waits
/// until that premise has, and is a test after it: `(and (neq ?x a) (p ?x))`
/// is `(and (p ?x) (neq ?x a))`. One with a variable **nothing** in the
/// conjunction binds binds it where it is written, ranging over the program's
/// names (the matcher's guard step), and counts as binding it from there on, as
/// a relation premise does — for an `(absent …)` after it among them.
#[derive(Default)]
struct Seq {
    /// Bound before the next premise: the parameters, the scope an `(absent
    /// …)` projects, and what this conjunction's premises bound so far.
    bound: FxHashSet<Symbol>,
    /// What a relation premise of this conjunction binds, anywhere in it.
    later: FxHashSet<Symbol>,
    /// Guards waiting for a relation premise, in the order written, each with
    /// its variables.
    pending: Vec<(Step, Vec<Symbol>)>,
}

impl<'a> Compiler<'a> {
    fn new(ast: &'a Ast, terms: &'a mut Terms) -> Compiler<'a> {
        Compiler {
            ast,
            terms,
            who: String::new(),
            clause: ":match",
            steps: Vec::new(),
            slots: Vec::new(),
            guards: Vec::new(),
            probes: Vec::new(),
            shared: Vec::new(),
            guard_args: Vec::new(),
            guard_keys: Vec::new(),
            plan_space: Space::default(),
            seed: Vec::new(),
            sub: None,
            sub_scope: FxHashSet::default(),
            known: FxHashSet::default(),
            bound: FxHashSet::default(),
            seqs: Vec::new(),
        }
    }

    fn run(mut self, rule: &Rule, activator: Option<FactId>) -> Result<Plan, CompileError> {
        // A query's goal is compiled as a parameter-less rule named `#query`
        // (`verdict::goal_bindings`), so a refusal of one names the query.
        if rule.name == self.terms.kernel.query_rule {
            self.who = "query".to_string();
            self.clause = ":goal";
        } else {
            self.who = format!("rule '{}'", self.terms.sym(rule.name));
        }
        let activator_args = self.bind_activator(rule, activator)?;

        // A top-level `(or …)` lowers to one step-tuple per disjunct:
        // `steps` is the first, `extra_match_plans` the rest, and the matcher
        // runs them all. Each disjunct compiles with the same activator
        // bindings and its own fresh body-var scope.
        let bodies: Vec<NodeId> = match &rule.match_ {
            None => vec![],
            Some(p) if self.is_empty(NodeId(p.expr.0)) => vec![],
            Some(p) => self.match_disjuncts(NodeId(p.expr.0)),
        };
        let mut disjuncts: Vec<Disjunct> = Vec::new();
        if bodies.is_empty() {
            // `:match ()`, the empty conjunction, which is true → one empty
            // disjunct, which the matcher accepts as a single vacuous match:
            // the plan fires once, with only its parameters bound (`W31`).
            disjuncts.push(Disjunct {
                steps: Span::EMPTY,
                guards: Span::EMPTY,
                guard_key: Span::EMPTY,
                n_premises: 0,
                n_slots: 0,
            });
        }
        for body in bodies {
            disjuncts.push(self.compile_disjunct(body)?);
        }

        // The `:assert` clause: an `(and …)` lowers to one template per
        // conjunct, at any depth (`assert_conjuncts`). Var slots stay
        // unbound — `fire` fills them; Atom / nested slots stay literal.
        //
        // `:assert ()`, the empty conclusion, asserts nothing: no template, so
        // an application derives nothing and is no firing, as one whose every
        // conclusion holds; no fact is stored for it, and no name joins the
        // program's names or H (`W34`).
        let mut asserts: Vec<Slot> = Vec::new();
        if let Some(p) = &rule.assert_
            && !self.is_empty(NodeId(p.expr.0))
        {
            for c in self.assert_conjuncts(NodeId(p.expr.0)) {
                let slot = self.slot(c)?;
                asserts.push(slot);
            }
        }

        Ok(Plan {
            rule: rule.name,
            activator_args,
            activator,
            seed: self.seed.into_boxed_slice(),
            disjuncts: disjuncts.into_boxed_slice(),
            asserts: asserts.into_boxed_slice(),
            why: rule.why,
            n_regs: self.plan_space.names.len() as Reg,
            reg_names: self.plan_space.names.into_boxed_slice(),
            steps: self.steps.into_boxed_slice(),
            slots: self.slots.into_boxed_slice(),
            guards: self.guards.into_boxed_slice(),
            probes: self.probes.into_boxed_slice(),
            shared: self.shared.into_boxed_slice(),
            guard_args: self.guard_args.into_boxed_slice(),
            guard_keys: self.guard_keys.into_boxed_slice(),
        })
    }

    /// Bind the rule's parameters from the activator, positionally, and
    /// return its every argument, the compile key's list
    /// ([`Plan::activator_args`]).
    ///
    /// Every argument binds its parameter, a name or a nested fact alike: a
    /// nested fact is a value, which a pattern matches and a conclusion
    /// writes. The seed is what `slot` substitutes from and what `split_naf`
    /// seeds every guard's scope with, and a relation head written as a
    /// parameter the activator binds to a fact meets the fact there and is
    /// refused (`head_not_a_name`).
    ///
    /// **A nested fact binds its parameter** as a name does, so the seed is
    /// as long as the parameter list and no parameter is a free variable of
    /// the plan: `(act (f a))` over `(p a) (p b) (p (f a))` fires `(p (f a))`
    /// alone (`W33`), and a conclusion that reads the parameter alone has its
    /// value (`W58`).
    fn bind_activator(
        &mut self,
        rule: &Rule,
        activator: Option<FactId>,
    ) -> Result<Box<[Value]>, CompileError> {
        let Some(fact) = activator else {
            return Ok(Box::new([]));
        };
        let args: Vec<Value> = self.terms.facts.get(fact).1.to_vec();
        if rule.params.len() != args.len() {
            // Leaving `bindings` empty would drop every premise headed by a
            // parameter, and a plan whose premises all drop fires
            // unconditionally. Both drivers filter mismatched activators
            // before they get here (`activators_for`, `hrule.Hrules`); a
            // direct caller that constructs the pair anyway gets told, in
            // Ein's words: the activator as a fact and the parameters as the
            // rule writes them.
            let fact = crate::events::sexpr(self.terms, fact);
            let params: Vec<String> = rule
                .params
                .iter()
                .map(|&p| format!("?{}", self.terms.sym(p)))
                .collect();
            return Err(CompileError(format!(
                "{}: its activator {fact} has {} argument{}, and it declares {} parameter{} ({}) \
                 — an activator binds a rule's parameters one for one, so this one activates \
                 nothing",
                self.who,
                args.len(),
                if args.len() == 1 { "" } else { "s" },
                rule.params.len(),
                if rule.params.len() == 1 { "" } else { "s" },
                params.join(" "),
            )));
        }
        for (&p, &a) in rule.params.iter().zip(args.iter()) {
            match self.plan_space.get(p) {
                // A repeated parameter name keeps the first position and
                // takes the last value.
                Some(reg) => self.seed[reg as usize].1 = a,
                None => {
                    self.plan_space.push(p, None)?;
                    self.seed.push((p, a));
                }
            }
        }
        Ok(args.into_boxed_slice())
    }

    // ── Disjuncts and conjuncts ────────────────────────────────────

    /// A top-level `(or d1 … dm)` `:match` → `[d1, …, dm]`; else `[expr]`.
    ///
    /// Every argument is a disjunct (`ein.y`'s `Connective`): no keyword pair
    /// parses inside a term, so there is nothing beside them to filter.
    fn match_disjuncts(&self, expr: NodeId) -> Vec<NodeId> {
        if self.head_is(expr, "or") {
            return self.ast.form_args(expr).to_vec();
        }
        vec![expr]
    }

    /// An `:assert`'s conjuncts, `(and …)` flattened at any depth:
    /// [`ein_ir::Ast::conjuncts`], the one reading every reader of a
    /// conclusion shares, so `(and (r a) (and (p a) (q a)))` is three
    /// templates, as `(and (r a) (p a) (q a))` is, and none is headed `and`.
    ///
    /// The grammar's `Conclusion` nests, and [`premise`](Self::premise)
    /// flattens a premise's at any depth, so an `and` inside a conclusion,
    /// written or left by a macro's call as one conjunct, is its members and
    /// never one fact headed `and` (`W94`).
    fn assert_conjuncts(&self, expr: NodeId) -> Vec<NodeId> {
        self.ast.conjuncts(expr)
    }

    fn head_is(&self, node: NodeId, name: &str) -> bool {
        matches!(self.ast.node(node), Node::SForm { .. }) && self.ast.head_name(node) == Some(name)
    }

    /// `()`: the parser's form over the synthetic [`ein_core::text::EMPTY_HEAD`],
    /// which the grammar writes only as a whole `:match` (the empty
    /// conjunction) or a whole `:assert` (the empty conclusion).
    fn is_empty(&self, node: NodeId) -> bool {
        self.head_is(node, ein_core::text::EMPTY_HEAD) && self.ast.form_args(node).is_empty()
    }

    fn compile_disjunct(&mut self, body: NodeId) -> Result<Disjunct, CompileError> {
        self.known.clear();
        self.bound = self.seed.iter().map(|(name, _)| *name).collect();
        let guards_start = self.guards.len() as u32;
        let mut steps: Vec<Step> = Vec::new();
        let bound: FxHashSet<Symbol> = self.seed.iter().map(|(name, _)| *name).collect();
        self.open_seq(body, bound);
        let compiled = self.premise(body, true, &mut steps);
        self.close_seq(&mut steps);
        compiled?;
        let n_premises = steps.iter().filter(|s| matches!(s, Step::Rel(_))).count() as u16;
        let span = self.push_steps(steps);
        let guards = Span {
            start: guards_start,
            len: self.guards.len() as u32 - guards_start,
        };
        Ok(Disjunct {
            steps: span,
            guards,
            guard_key: self.encode_guards(guards),
            n_premises,
            n_slots: self.rel_steps(span),
        })
    }

    fn push_steps(&mut self, steps: Vec<Step>) -> Span {
        let start = self.steps.len() as u32;
        self.steps.extend(steps);
        Span {
            start,
            len: self.steps.len() as u32 - start,
        }
    }

    // ── Premises ───────────────────────────────────────────────────

    /// Compile one premise into zero or more steps.
    ///
    /// `top` is "this premise is a direct member of the disjunct's step
    /// sequence", which survives `(and …)` flattening and is what decides
    /// whether an `(absent …)` is **lifted** to a [`NafGuard`] or
    /// stays an inline [`Step::Absent`] inside a negative query.
    fn premise(
        &mut self,
        node: NodeId,
        top: bool,
        out: &mut Vec<Step>,
    ) -> Result<(), CompileError> {
        match self.ast.node(node) {
            Node::SForm { .. } => {}
            // A keyword pair, `:where (neq ?a ?b)` beside a conjunction's
            // members, is refused inside a term at the parse (Q-M1e2.28), so
            // none reaches a conjunction here.
            Node::KwPair { .. } => {
                unreachable!(
                    "a premise is a pattern or a connective, never a keyword pair (ein.y's Premise)"
                )
            }
            // A premise that is not a form — an atom, a variable, `_`, a
            // string. The loader refuses each in a rule (`W24`), so no
            // program it accepts reaches this arm. Were the premise dropped,
            // `:match _` would fire with no premise at all,
            // `(and (hit ?y) _)` on `(hit ?y)` alone, and `(absent ?x)` would
            // be the empty sub-plan below, a `CompileError` in the Python IR's
            // spelling.
            _ => return Ok(()),
        }
        let head = match self.ast.node(node) {
            Node::SForm { head, .. } => head,
            _ => unreachable!(),
        };
        let head_name: Option<String> = match self.ast.node(head) {
            Node::Atom(s) => Some(self.ast.sym(s).to_string()),
            _ => None,
        };
        let args = self.ast.form_args(node).to_vec();

        // `(absent P)` — explicit negation-as-failure.
        //
        // **Exactly one argument.** `absent`
        // is one of the three kernel meta-primitives the grammar does not
        // shape-pin ([`grammar.md` §2] names four of seven), so what its arity
        // is checked by is whatever reads it — and where nothing does,
        // `(absent)` falls through to the ordinary relation path and
        // retired the rule in silence, while `(absent A B)` read `args[0]` and
        // dropped the rest, firing a guard weaker than the one written.
        //
        // [`grammar.md` §2]: ../../../../docs/ein/user/reference/grammar.md
        //
        // **An invariant** (`W66`): the grammar takes `absent` at one
        // premise, and what a macro's expansion writes is held to the
        // grammar's shapes at load (`ein_ir::from_ir`'s
        // `refuse_malformed_primitives`, Q-M1f.80), so no program reaches
        // another arity here. A line of the compiler would be met only where
        // the plan is compiled, so a rule only a commitment activates would
        // load and some commands would answer it (Q-M1f.84).
        if head_name.as_deref() == Some("absent") {
            assert!(
                args.len() == 1,
                "{}: absent takes one premise, which the grammar and the loader's \
                 refuse_malformed_primitives hold",
                ein_ir::dump_compact(self.ast, node)
            );
            return self.absent(node, args[0], top, out);
        }

        // `(forall …)` / `(unknown …)` are **not** compiler sugar: they are
        // ein-lang `(macro …)` declarations expanded at LOAD time, so
        // by now they are already `(absent (and G (absent B)))` /
        // `(and (absent P) (absent (not P)))`.

        // `(and P1 P2 …)` — flatten into sibling premises of the same plan.
        if head_name.as_deref() == Some("and") {
            for child in args {
                self.premise(child, top, out)?;
            }
            return Ok(());
        }

        // A *top-level* `(or …)` was split by `match_disjuncts` before this
        // ran, so what reaches here is nested. Dropping it would not make the
        // shape unsupported, it would make it silently WRONG in both
        // polarities: `(and (a ?x) (or (p ?x) (q ?x)))` would fire with
        // neither p nor q in the KB, and `(absent (or (p ?x) (q ?x)))` would
        // never fire with neither, permanently (the empty sub-plan makes the
        // guard fail, and a failing monotone guard retires its candidate).
        //
        // A line of the compiler would be met only where a plan is compiled,
        // so some commands would answer a program others stop on (`W62`).
        // The loader refuses one, after macro expansion, so no program
        // reaches this arm (Q-M1f.9, Q-M1f.84).
        if head_name.as_deref() == Some("or") {
            unreachable!(
                "an (or …) is a whole :match or :goal, and the loader refuses one written as a \
                 premise (ein_ir::from_ir's refuse_nested_disjunctions)"
            );
        }

        // A registered built-in predicate — `eq` / `neq`. Its args stay **raw
        // IR nodes**: see [`GuardArg`].
        //
        // The arity is the predicate's own ([`predicates::Pred::arity`]) and
        // is checked **here**, at compile time, and not in the matcher: a
        // check there would hold `(eq ?x)` only from below, at `guard_holds`,
        // and `(eq ?x A B)` would pass on `args[0..2]` with the tail dropped.
        // The grammar shape-pins `NeqForm` and `eq` is an ordinary
        // `GenericList`, so `eq`'s arity is what needs the check.
        //
        // `eq` is the relation wherever it is written (Q-M1e2.26), so a
        // pattern whose relation is a parameter its activator binds to `eq`,
        // `(?R ?x ?y)` under `(r eq)`, is this guard, and `(not (eq a b))`,
        // the stored negation of a fact whose truth is computed, is `(neq a
        // b)` (Q-M1f.31). As ordinary patterns over stored facts, which no
        // state holds, they would match nothing (`W51`, `W52`).
        //
        // **An invariant** (`W66`): a written guard is shaped by the grammar,
        // one a macro's expansion writes is refused at load (`ein_ir::from_ir`'s
        // `refuse_malformed_primitives`), and one whose relation is a
        // parameter its activator binds to `eq` is refused at load
        // (`refuse_eq_at_another_arity`, `W60`), so no program reaches another
        // arity here.
        if let Some((pred, args)) = self.guard_form(node) {
            assert!(
                args.len() == pred.arity(),
                "{}: {} compares two values, which the grammar and the loader's \
                 refuse_malformed_primitives and refuse_eq_at_another_arity hold",
                ein_ir::dump_compact(self.ast, node),
                pred.as_str()
            );
            let start = self.guard_args.len() as u32;
            for &arg in &args {
                let ga = self.guard_arg(node, arg)?;
                self.guard_args.push(ga);
            }
            let step = Step::Guard {
                pred,
                args: Span {
                    start,
                    len: self.guard_args.len() as u32 - start,
                },
            };
            // Where it goes is [`Seq`]'s: here, or after the relation premise
            // that binds its variable.
            let mut vars: Vec<Symbol> = Vec::new();
            for &arg in &args {
                if let Node::Var(s) = self.ast.node(arg) {
                    let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                    if !vars.contains(&name) {
                        vars.push(name);
                    }
                }
            }
            match self.seqs.last_mut() {
                Some(q)
                    if vars
                        .iter()
                        .any(|v| q.later.contains(v) && !q.bound.contains(v)) =>
                {
                    q.pending.push((step, vars));
                }
                _ => {
                    out.push(step);
                    self.bind_guard_vars(&vars);
                }
            }
            return Ok(());
        }

        // An ordinary relation pattern. `(not P)` is one too: relation `not`
        // with the inner expression as a nested arg, matching stored `(not P)`
        // facts. `(absent P)` is how you ask for negation as failure.
        self.relation(node, head, &args, out)
    }

    fn absent(
        &mut self,
        node: NodeId,
        body: NodeId,
        top: bool,
        out: &mut Vec<Step>,
    ) -> Result<(), CompileError> {
        if !top {
            // A guard nested inside another guard's sub-plan — what a `forall`
            // desugars to. It is part of the negative query, not of the
            // closure, and the boundary evaluates it as one unit, in the
            // enclosing sub-plan's binding environment.
            let mut sub: Vec<Step> = Vec::new();
            // What the enclosing conjunction has bound where this is written is
            // bound in it, and what it binds stays in it.
            let bound = self
                .seqs
                .last()
                .map(|q| q.bound.clone())
                .unwrap_or_default();
            self.open_seq(body, bound);
            let compiled = self.premise(body, false, &mut sub);
            self.close_seq(&mut sub);
            compiled?;
            assert!(!sub.is_empty(), "{}", self.empty_sub_plan(node));
            let span = self.push_steps(sub);
            out.push(Step::Absent { sub: span });
            return Ok(());
        }

        // A top-level guard, lifted out of the closure plan. Its
        // scope is the variables bound by the positive premises that
        // *preceded* it, seeded with the rule's parameters — which is what
        // preserves the distinction between `(and (absent (P ?x)) (Q ?x))`
        // ("is there no P at all?") and `(and (Q ?x) (absent (P ?x)))`
        // ("is there no P for *this* x?") once the guard is judged at the end.
        //
        // Seeding with the parameters is not cosmetic: a predicate `Guard`
        // inside an `(absent …)` is compiled from the raw IR nodes, so an
        // `(eq ?y ?PARAM)` under a guard would otherwise resolve `?PARAM` to
        // nothing, the negative query would find nothing, and the guard would
        // pass when it must fail.
        // `sort_names` totally orders this on the next line: symbols are
        // interned, so distinct ids have distinct text and the comparison has
        // no ties for the set's iteration order to break. What reaches
        // `NafGuard.scope` is a function of *which* variables are bound, not
        // of how the set was built.
        // determinism-ok: sorted by name before it can reach `NafGuard.scope`.
        let mut scope: Vec<Symbol> = self.bound.iter().copied().collect();
        self.sort_names(&mut scope);
        self.sub_scope = self.bound.clone();
        self.sub = Some(Space::default());

        let mut sub: Vec<Step> = Vec::new();
        self.open_seq(body, self.bound.clone());
        let compiled = self.premise(body, false, &mut sub);
        self.close_seq(&mut sub);
        let space = self.sub.take().expect("the sub-space is still mine");
        self.sub_scope.clear();
        compiled?;
        assert!(!sub.is_empty(), "{}", self.empty_sub_plan(node));
        let span = self.push_steps(sub);

        let mut watched = self.watched_relations(span);
        self.sort_names(&mut watched);
        // Anti-monotone unless the query holds another `(absent …)`: a purely
        // positive query's match set only grows, so once it finds a match it
        // finds one forever. A `forall`'s nested absent can flip from failing
        // to *passing* as the KB grows, so it must stay parked.
        let monotone = !self
            .steps(span)
            .iter()
            .any(|s| matches!(s, Step::Absent { .. }));
        self.guards.push(NafGuard {
            sub: span,
            n_regs: space.names.len() as Reg,
            n_slots: self.rel_steps(span),
            reg_names: space.names.into_boxed_slice(),
            scope_of: space.scope_of.into_boxed_slice(),
            scope: scope.into_boxed_slice(),
            watched: watched.into_boxed_slice(),
            monotone,
        });
        Ok(())
    }

    /// Where `node` is written, `<file>:<line>:<col>: `, as a parse error
    /// places itself; nothing where the parser gave it no position.
    fn place(&self, node: NodeId) -> String {
        match self.ast.loc(node) {
            Some(loc) => format!("{}: ", ein_ir::loc_repr(self.ast, Some(loc))),
            None => String::new(),
        }
    }

    /// A relation head written as a parameter, bound by the activator to a
    /// value that is not a name: a nested `Fact`, which names no relation
    /// (`W24`, Q-M1e2.18). The parameter is seeded with the fact, as a value
    /// parameter is, and the head meets it there. The loader refuses the
    /// activators it can read, a fact or a literal in an `:assert`; this is
    /// the one a rule derives through a variable, which only a run binds.
    ///
    /// Ein has no int kind: a run of digits is a name, which binds like any
    /// other, so a value is a name or a fact and this is a fact.
    ///
    /// **No program reaches it**: the loader refuses every activator a
    /// program writes or a rule could derive that binds a relation parameter
    /// to a fact, and a candidate is a name. A direct caller of
    /// [`compile_rule`] with such an activator does, and reads the line in
    /// Ein's words (Q-M1f.75).
    fn head_not_a_name(&self, node: NodeId, head: NodeId, v: Value) -> CompileError {
        debug_assert!(v.as_fact().is_some(), "a value that is no name is a fact");
        CompileError(format!(
            "{}{}: its {} writes {}, and its activator binds {} to the fact {} — a relation is \
             named by a symbol, not a fact",
            self.place(node),
            self.who,
            self.clause,
            ein_ir::dump_compact(self.ast, node),
            ein_ir::dump_compact(self.ast, head),
            crate::events::sexpr_value(self.terms, v),
        ))
    }

    /// What an `(absent …)` whose sub-plan compiled to no steps would be.
    ///
    /// An empty sub-plan is not "a query that finds nothing", it is a query
    /// that matches VACUOUSLY: running no steps yields one match, so `holds`
    /// is true and the guard fails against every possible KB. `monotone` is
    /// then true, so the candidate would be retired permanently on its first
    /// judgement, in silence. It is an assertion: an `absent` takes one premise
    /// (`ein.y`'s `Connective`), a premise that is no form is the loader's
    /// refusal (`from_ir`'s `NotAForm`, which a macro's body can still reach),
    /// and every form a premise can be compiles to a step, so no program
    /// reaches one; `(absent :k v)`, a keyword pair as the premise, is a parse
    /// error (Q-M1e2.28).
    fn empty_sub_plan(&self, node: NodeId) -> String {
        format!(
            "`(absent …)` sub-plan compiled to no steps in {} — the guard \
             would fail against every KB and its candidates would be retired \
             permanently.",
            ein_ir::dump_compact(self.ast, node)
        )
    }

    fn relation(
        &mut self,
        node: NodeId,
        head: NodeId,
        args: &[NodeId],
        out: &mut Vec<Step>,
    ) -> Result<(), CompileError> {
        let rel = match self.ast.node(head) {
            Node::Atom(s) => {
                let name = self.ast.sym(s).to_string();
                self.intern(&name)
            }
            Node::Var(s) => {
                let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                match self.seed_value(name) {
                    Some(v) if v.as_sym().is_none() => {
                        return Err(self.head_not_a_name(node, head, v));
                    }
                    Some(v) => {
                        // The relation's own name, its raw text: what it
                        // interns as, and not how it prints.
                        let text = self.terms.sym(v.as_sym().expect("a name")).to_string();
                        self.intern(&text)
                    }
                    // An error, never a dropped premise. An unbound head var
                    // reaches here only when the activator failed to bind the
                    // rule's parameter, and a dropped premise is silently
                    // wrong: if it was the plan's only one, the matcher yields
                    // one vacuous match and the rule fires unconditionally —
                    // a rule with a ground `:assert` would store its
                    // conclusion off an arity-mismatched activator.
                    //
                    // The loader refuses a head variable no parameter binds
                    // (`W24`), and a parameter bound to a nested fact is
                    // seeded with it and refused just above, so no program the
                    // loader accepts reaches this arm. A goal has no
                    // activator, and the loader refuses a goal's relation
                    // variable at every depth (`refuse_no_fact_holds`, `W64`),
                    // so only a direct caller compiling a rule with
                    // parameters and no activator reaches this line, in Ein's
                    // words (Q-M1f.75).
                    None => {
                        return Err(CompileError(format!(
                            "{}{}: its {} writes {} with ?{} as its relation, and no activator \
                             binds it — a relation written as a variable is a parameter its \
                             activator binds, so compile the rule with one",
                            self.place(node),
                            self.who,
                            self.clause,
                            ein_ir::dump_compact(self.ast, node),
                            self.ast.sym(s),
                        )));
                    }
                }
            }
            // A premise's relation is a name or a variable (`ein.y`'s
            // `PremiseHead`): a bare `_` is refused at the parse
            // (Q-M1e2.22), so the arm is an invariant (Q-M1f.9).
            _ => unreachable!("a premise's relation is a name or a variable (ein.y's PremiseHead)"),
        };

        // Lowered into a local first, then appended in one block: `slot`
        // pushes a nested pattern's own children into the same arena, so
        // pushing as we go would interleave them into this step's span.
        let mut lowered: Vec<Slot> = Vec::with_capacity(args.len());
        for &a in args {
            let slot = self.slot(a)?;
            lowered.push(slot);
        }
        let start = self.slots.len() as u32;
        self.slots.extend(lowered);
        let slots = Span {
            start,
            len: self.slots.len() as u32 - start,
        };

        // Variables of these slots that an earlier premise already mentioned.
        // The unifier does not need this — it treats Scan and Join identically
        // — but it is in the compiled shape, so the diff sees it.
        let mut vars: Vec<Symbol> = Vec::new();
        self.collect_vars(slots, &mut vars);
        let mut shared_names: Vec<Symbol> = Vec::new();
        for &name in &vars {
            if self.known.contains(&name) && !shared_names.contains(&name) {
                shared_names.push(name);
            }
        }
        self.sort_names(&mut shared_names);
        let shared_start = self.shared.len() as u32;
        self.shared.extend(&shared_names);
        let shared = Span {
            start: shared_start,
            len: self.shared.len() as u32 - shared_start,
        };

        // `known_vars` grows *after* the shared check, and it is shared by the
        // whole disjunct — including `(absent …)` sub-plans, whose variables
        // therefore make a later top-level premise a `Join`. One set is
        // threaded through the whole body.
        for &name in &vars {
            self.known.insert(name);
        }
        // `split_naf`'s accumulator, which is *not* `known_vars`: it collects
        // from the **top-level positive** premises only, and it is what a
        // guard's scope is a snapshot of. A variable first bound inside an
        // `(absent …)` sub-plan pollutes `known_vars` (so a later premise
        // becomes a `Join`) but is not in scope for anything.
        if self.sub.is_none() {
            self.bound.extend(vars.iter().copied());
        }

        let probe = self.probes_for(slots);
        out.push(Step::Rel(RelStep {
            join: !shared.is_empty(),
            rel,
            slots,
            probe,
            shared,
        }));
        // The guards that waited for these variables go right after it.
        if let Some(q) = self.seqs.last_mut() {
            q.bound.extend(vars.iter().copied());
        }
        self.flush_ready(out);
        Ok(())
    }

    // ── Where a guard goes ─────────────────────────────────────────

    /// `node` as an `eq` / `neq` guard, with the arguments it compares, or
    /// `None` for a premise that is not one: `(eq a b)` and `(neq a b)`, a
    /// pattern whose relation is a parameter its activator binds to `eq`, and
    /// `(not P)` over any of these, which is the other predicate over the same
    /// arguments (Q-M1f.31).
    fn guard_form(&mut self, node: NodeId) -> Option<(predicates::Pred, Vec<NodeId>)> {
        let Node::SForm { head, args } = self.ast.node(node) else {
            return None;
        };
        let args = self.ast.args(args).to_vec();
        let name: Option<String> = match self.ast.node(head) {
            Node::Atom(s) => Some(self.ast.sym(s).to_string()),
            Node::Var(s) => {
                let var = self.terms.intern_text(self.ast.sym(s)).expect("room");
                self.seed_value(var)
                    .and_then(|v| v.as_sym())
                    .map(|v| self.terms.sym(v).to_string())
            }
            _ => None,
        };
        match name.as_deref() {
            Some("not") if args.len() == 1 => {
                let (pred, inner) = self.guard_form(args[0])?;
                let other = match pred {
                    predicates::Pred::Eq => predicates::Pred::Neq,
                    predicates::Pred::Neq => predicates::Pred::Eq,
                };
                Some((other, inner))
            }
            Some(h) => predicates::get(h).map(|p| (p, args)),
            None => None,
        }
    }

    /// Open a conjunction: `bound` is what is bound where it starts.
    fn open_seq(&mut self, body: NodeId, bound: FxHashSet<Symbol>) {
        let mut later = FxHashSet::default();
        self.relation_vars(body, &mut later);
        self.seqs.push(Seq {
            bound,
            later,
            pending: Vec::new(),
        });
    }

    /// Close the innermost conjunction. A guard still waiting has nothing
    /// left to wait for — every variable it waited on is one a relation
    /// premise of the conjunction binds, so none is left when the premises
    /// are — but a compile error part-way leaves some, and they go last.
    fn close_seq(&mut self, out: &mut Vec<Step>) {
        let q = self.seqs.pop().expect("a conjunction is open");
        out.extend(q.pending.into_iter().map(|(step, _)| step));
    }

    /// Emit, in the order written, every waiting guard whose variables a
    /// relation premise has now bound; each binds the rest of its variables.
    fn flush_ready(&mut self, out: &mut Vec<Step>) {
        loop {
            let Some(q) = self.seqs.last_mut() else {
                return;
            };
            let at = q.pending.iter().position(|(_, vars)| {
                vars.iter()
                    .all(|v| !q.later.contains(v) || q.bound.contains(v))
            });
            let Some(at) = at else {
                return;
            };
            let (step, vars) = q.pending.remove(at);
            out.push(step);
            self.bind_guard_vars(&vars);
        }
    }

    /// A guard emitted: its variables are bound from here on, in the
    /// conjunction and, at a disjunct's top level, for an `(absent …)`'s scope.
    fn bind_guard_vars(&mut self, vars: &[Symbol]) {
        if let Some(q) = self.seqs.last_mut() {
            q.bound.extend(vars.iter().copied());
        }
        if self.sub.is_none() {
            self.bound.extend(vars.iter().copied());
        }
    }

    /// The variables a **relation premise** of the conjunction `node` binds:
    /// a pattern's, `(not P)`'s included, through `(and …)`, and not an
    /// `(absent …)`'s, a guard's or a nested `(or …)`'s (which the compiler
    /// refuses).
    fn relation_vars(&mut self, node: NodeId, out: &mut FxHashSet<Symbol>) {
        let Node::SForm { head, args } = self.ast.node(node) else {
            return;
        };
        let head_name = match self.ast.node(head) {
            Node::Atom(s) => Some(self.ast.sym(s).to_string()),
            _ => None,
        };
        if self.guard_form(node).is_some() {
            return;
        }
        match head_name.as_deref() {
            Some("and") => {
                for a in self.ast.args(args).to_vec() {
                    self.relation_vars(a, out);
                }
            }
            Some("absent") | Some("or") => {}
            _ => {
                for a in self.ast.args(args).to_vec() {
                    self.vars_of(a, out);
                }
            }
        }
    }

    fn vars_of(&mut self, node: NodeId, out: &mut FxHashSet<Symbol>) {
        match self.ast.node(node) {
            Node::Var(s) => {
                let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                out.insert(name);
            }
            Node::SForm { args, .. } => {
                for a in self.ast.args(args).to_vec() {
                    self.vars_of(a, out);
                }
            }
            _ => {}
        }
    }

    // ── Slots ──────────────────────────────────────────────────────

    /// Lower one IR slot node.
    ///
    /// A `Var` the activator bound becomes a `Const`, so the unifier treats it
    /// uniformly with a literal.
    /// An `Atom` becomes the symbol it names, as the loader holds it in a fact
    /// (`from_ir::atomic_value`), at any depth, in a `:match` and an `:assert`
    /// alike; `1` is the symbol `1`. A string in a pattern and a bare `_` are
    /// refused at the parse, so neither reaches a slot.
    /// An `SForm` becomes a nested pattern, with its head substituted from the
    /// binding when it is a bound `Var`; one whose head is a parameter the
    /// activator bound to a nested fact is refused (`head_not_a_name`).
    /// An `SForm` whose head is a variable the activator does not bind is
    /// returned as-is and compared by equality, which is the "unrecognised
    /// shape" safety net and which never matches anything; in an `:assert` it
    /// stops the firing. No program the loader accepts reaches it in a
    /// `:match` (`W24`): the loader refuses a relation variable no parameter
    /// binds, in a rule.
    /// A keyword pair inside a term is refused at the parse (Q-M1e2.28), so
    /// the `KwPair` arm is unreachable.
    ///
    /// **An opaque node is a silent wrong answer** wherever a program can
    /// reach one: it matches nothing, so a rule over it never fires and an
    /// `(absent …)` over it always holds. That is why every shape that would
    /// be one — a string (`W18`), a wildcard or a range (`W20`, Q-M1e2.22), a
    /// keyword pair inside a pattern and a relation variable no parameter
    /// binds (`W24`) — is refused before it gets here.
    fn slot(&mut self, node: NodeId) -> Result<Slot, CompileError> {
        match self.ast.node(node) {
            Node::Var(s) => {
                let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                match self.seed_value(name) {
                    Some(v) => Ok(Slot::Const(v)),
                    None => Ok(Slot::Reg(self.var_reg(name)?)),
                }
            }
            Node::Atom(s) => {
                let name = self.ast.sym(s).to_string();
                Ok(Slot::Const(Value::sym(self.intern(&name))))
            }
            Node::Str(_) => unreachable!("a pattern holds no string (ein.y's Arg and Term)"),
            Node::SForm { head, args } => {
                let rel = match self.ast.node(head) {
                    Node::Atom(s) => {
                        let name = self.ast.sym(s).to_string();
                        Some(self.intern(&name))
                    }
                    Node::Var(s) => {
                        let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                        match self.seed_value(name) {
                            Some(v) if v.as_sym().is_none() => {
                                return Err(self.head_not_a_name(node, head, v));
                            }
                            v => v.map(|v| {
                                // The relation's own name, as above.
                                let text = self.terms.sym(v.as_sym().expect("a name")).to_string();
                                self.intern(&text)
                            }),
                        }
                    }
                    _ => None,
                };
                // An unusable head is the safety net: the validator catches
                // malformed slots at load time, and what still gets here stays
                // an opaque node.
                let Some(rel) = rel else {
                    return Ok(Slot::Opaque(node));
                };
                // Every argument is a slot, here as at a premise's top level
                // (no keyword pair parses inside a term).
                let children: Vec<NodeId> = self.ast.args(args).to_vec();
                let mut lowered = Vec::with_capacity(children.len());
                for c in children {
                    lowered.push(self.slot(c)?);
                }
                let start = self.slots.len() as u32;
                self.slots.extend(lowered);
                Ok(Slot::Nested {
                    rel,
                    slots: Span {
                        start,
                        len: self.slots.len() as u32 - start,
                    },
                })
            }
            Node::KwPair { .. } => {
                unreachable!("a pattern's argument is no keyword pair (ein.y's Arg and Term)")
            }
            _ => Ok(Slot::Opaque(node)),
        }
    }

    /// One argument of the guard `guard`, as the matcher reads it.
    ///
    /// A variable is its register, bound where the guard is judged or bound by
    /// it ([`Seq`]); a name is the symbol it names, as in `slot`; and a fact
    /// that holds no variable is the nested fact it spells, a value as it is in
    /// `(p (f a))` (`form-in-guard`). The loader interns every such fact a
    /// guard writes
    /// (`from_ir::intern_program_names`), so this finds it under a lent table
    /// too.
    ///
    /// **A form that holds a variable is refused**, at load in a rule
    /// (`from_ir::refuse_no_fact_holds`) and here in a goal: it is a pattern,
    /// and the value a match would build from it may be one no store holds,
    /// which no register can be bound to (Q-M1f.30). A string, a bare `_` and
    /// a keyword pair are refused at the parse, so no guard compares one.
    fn guard_arg(&mut self, guard: NodeId, node: NodeId) -> Result<GuardArg, CompileError> {
        let kind = match self.ast.node(node) {
            Node::Var(s) => {
                let name = self.terms.intern_text(self.ast.sym(s)).expect("room");
                GuardArgKind::Reg(self.var_reg(name)?)
            }
            Node::Atom(s) => {
                let name = self.ast.sym(s).to_string();
                GuardArgKind::Const(Value::sym(self.intern(&name)))
            }
            Node::Str(_) => unreachable!("a guard compares no string (ein.y's Guard and Term)"),
            Node::SForm { .. } => match self.ground_value(node) {
                Some(v) => GuardArgKind::Const(v),
                None => {
                    return Err(CompileError(format!(
                        "{}: its {} writes {}, and the form {} holds a variable — {}",
                        self.who,
                        self.clause,
                        ein_ir::dump_compact(self.ast, guard),
                        ein_ir::dump_compact(self.ast, node),
                        GUARD_VALUES
                    )));
                }
            },
            Node::KwPair { .. } | Node::Keyword(_) => {
                unreachable!("a guard compares no keyword (ein.y's Guard and Term)")
            }
        };
        Ok(GuardArg { kind, node })
    }

    /// The value a form that holds no variable spells: the nested fact, as the
    /// loader interns it in a fact (`from_ir::fact_args`). `None` for a form
    /// with a variable in it, at any depth.
    fn ground_value(&mut self, node: NodeId) -> Option<Value> {
        match self.ast.node(node) {
            Node::Atom(s) => {
                let name = self.ast.sym(s).to_string();
                Some(Value::sym(self.intern(&name)))
            }
            Node::SForm { head, args } => {
                let rel = match self.ast.node(head) {
                    Node::Atom(s) => {
                        let name = self.ast.sym(s).to_string();
                        self.intern(&name)
                    }
                    _ => return None,
                };
                let mut values = Vec::new();
                for a in self.ast.args(args).to_vec() {
                    values.push(self.ground_value(a)?);
                }
                Some(
                    self.terms
                        .value_fact(rel, &values)
                        .expect("the loader interns every fact a guard writes"),
                )
            }
            _ => None,
        }
    }

    // ── Registers ──────────────────────────────────────────────────

    fn seed_value(&self, name: Symbol) -> Option<Value> {
        self.seed.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
    }

    /// The register a variable resolves to in the space being compiled.
    ///
    /// In a guard sub-plan a variable that is **in scope** is projected from
    /// the parent's register; one that is not starts free, whatever the parent
    /// has since bound it to. That is `world.project` decided at compile time.
    fn var_reg(&mut self, name: Symbol) -> Result<Reg, CompileError> {
        if self.sub.is_some() {
            if let Some(reg) = self.sub.as_ref().expect("checked").get(name) {
                return Ok(reg);
            }
            let from = if self.sub_scope.contains(&name) {
                // Every scope name has a plan register by now: scope is the
                // parameters plus the variables of preceding positive premises.
                Some(self.plan_reg(name)?)
            } else {
                None
            };
            return self.sub.as_mut().expect("checked").push(name, from);
        }
        self.plan_reg(name)
    }

    fn plan_reg(&mut self, name: Symbol) -> Result<Reg, CompileError> {
        match self.plan_space.get(name) {
            Some(reg) => Ok(reg),
            None => self.plan_space.push(name, None),
        }
    }

    fn intern(&mut self, name: &str) -> Symbol {
        self.terms.intern_text(name).expect("room for a name")
    }

    fn sort_names(&self, names: &mut [Symbol]) {
        names.sort_by(|&a, &b| self.terms.sym(a).cmp(self.terms.sym(b)));
    }

    // ── Walks over compiled shapes ─────────────────────────────────

    fn steps(&self, span: Span) -> &[Step] {
        &self.steps[span.range()]
    }

    /// Every variable named by `slots`, in left-to-right depth-first order —
    /// which is the order the matcher binds them, and therefore the order
    /// `Provenance.bindings` records.
    fn collect_vars(&self, slots: Span, out: &mut Vec<Symbol>) {
        for i in slots.range() {
            match self.slots[i] {
                Slot::Reg(r) => out.push(self.reg_name(r)),
                Slot::Nested { slots, .. } => self.collect_vars(slots, out),
                Slot::Const(_) | Slot::Opaque(_) => {}
            }
        }
    }

    fn reg_name(&self, r: Reg) -> Symbol {
        match &self.sub {
            Some(space) => space.names[r as usize],
            None => self.plan_space.names[r as usize],
        }
    }

    // ── The structural guard key ───────────────────────────────────
    //
    // Guards compare **by value**, so two `(or …)` disjuncts asking the same
    // negative question collapse in `seen`. Encoding the guards into a flat
    // `u32` vector gives that without any structural comparison at run time.
    //
    // Everything guard equality reaches is encoded and nothing else:
    // `Scan` vs `Join` and a `Join`'s `shared_vars`
    // included, register *indices* rather than variable names (identical
    // sub-plans allocate registers in the same encounter order, so the indices
    // agree exactly where the names do), and raw IR nodes **structurally**,
    // because two `(neq ?p ?q)` nodes at different source positions are equal
    // guards and must key alike.

    fn encode_guards(&mut self, guards: Span) -> Span {
        let start = self.guard_keys.len() as u32;
        for i in guards.range() {
            let g = &self.guards[i];
            let (sub, scope, watched, monotone) =
                (g.sub, g.scope.clone(), g.watched.clone(), g.monotone);
            self.guard_keys.push(monotone as u32);
            self.guard_keys.push(scope.len() as u32);
            self.guard_keys.extend(scope.iter().map(|s| s.0));
            self.guard_keys.push(watched.len() as u32);
            self.guard_keys.extend(watched.iter().map(|s| s.0));
            self.encode_steps(sub);
        }
        Span {
            start,
            len: self.guard_keys.len() as u32 - start,
        }
    }

    fn encode_steps(&mut self, span: Span) {
        self.guard_keys.push(span.len);
        for i in span.range() {
            match self.steps[i] {
                Step::Rel(r) => {
                    self.guard_keys.push(0);
                    self.guard_keys.push(r.join as u32);
                    self.guard_keys.push(r.rel.0);
                    let shared: Vec<u32> =
                        self.shared[r.shared.range()].iter().map(|s| s.0).collect();
                    self.guard_keys.push(shared.len() as u32);
                    self.guard_keys.extend(shared);
                    self.encode_slots(r.slots);
                }
                Step::Guard { pred, args } => {
                    self.guard_keys.push(1);
                    self.guard_keys.push(pred as u32);
                    self.guard_keys.push(args.len);
                    let nodes: Vec<NodeId> = self.guard_args[args.range()]
                        .iter()
                        .map(|a| a.node)
                        .collect();
                    for n in nodes {
                        self.encode_node(n);
                    }
                }
                Step::Absent { sub } => {
                    self.guard_keys.push(2);
                    self.encode_steps(sub);
                }
            }
        }
    }

    fn encode_slots(&mut self, span: Span) {
        self.guard_keys.push(span.len);
        for i in span.range() {
            match self.slots[i] {
                Slot::Reg(r) => {
                    self.guard_keys.push(0);
                    self.guard_keys.push(r as u32);
                }
                Slot::Const(v) => {
                    self.guard_keys.push(1);
                    self.guard_keys.push(v.bits());
                }
                Slot::Nested { rel, slots } => {
                    self.guard_keys.push(2);
                    self.guard_keys.push(rel.0);
                    self.encode_slots(slots);
                }
                Slot::Opaque(n) => {
                    self.guard_keys.push(3);
                    self.encode_node(n);
                }
            }
        }
    }

    /// An IR node by **shape**, not by arena position — the AST interns its
    /// strings, so two structurally equal nodes encode identically.
    fn encode_node(&mut self, node: NodeId) {
        match self.ast.node(node) {
            Node::Atom(s) => self.guard_keys.extend([0, s.0]),
            Node::Var(s) => self.guard_keys.extend([1, s.0]),
            Node::Keyword(s) => self.guard_keys.extend([2, s.0]),
            Node::Str(_) => unreachable!("a guard compares no string (ein.y's Guard and Term)"),
            // 3 to 6 are unused: no node of those kinds exists (a wildcard, a
            // string value, an int, a range).
            Node::KwPair { key, value } => {
                self.guard_keys.push(7);
                self.encode_node(key);
                self.encode_node(value);
            }
            Node::SForm { head, args } => {
                self.guard_keys.push(8);
                self.encode_node(head);
                let children: Vec<NodeId> = self.ast.args(args).to_vec();
                self.guard_keys.push(children.len() as u32);
                for c in children {
                    self.encode_node(c);
                }
            }
        }
    }

    /// Relation steps in a span, counting into nested `(absent …)` queries.
    fn rel_steps(&self, span: Span) -> u16 {
        self.steps(span)
            .iter()
            .map(|s| match s {
                Step::Rel(_) => 1,
                Step::Absent { sub } => self.rel_steps(*sub),
                Step::Guard { .. } => 0,
            })
            .sum()
    }

    /// Every relation a guard sub-plan reads, through nested guards too — the
    /// boundary's invalidation key.
    ///
    /// A `(not (R …))` pattern contributes `"not"`, which is right: the query
    /// reads the *stored negative* fact, and that is the relation whose growth
    /// can change the answer.
    fn watched_relations(&self, span: Span) -> Vec<Symbol> {
        let mut out: Vec<Symbol> = Vec::new();
        let mut stack = vec![span];
        while let Some(span) = stack.pop() {
            for step in self.steps(span) {
                match step {
                    Step::Rel(r) => {
                        if !out.contains(&r.rel) {
                            out.push(r.rel);
                        }
                    }
                    Step::Absent { sub } => stack.push(*sub),
                    Step::Guard { .. } => {}
                }
            }
        }
        out
    }

    /// The ordered probe candidates for one relation step — see [`Probe`].
    ///
    /// The top-level walk is the left-to-right candidate scan, with a
    /// **second level**: a `Nested` slot still contributes no key of its
    /// own, but each of *its* slots does, appended at the outer slot's
    /// position. The list stays in left-to-right order and the runtime still
    /// takes the first usable entry, so wherever the top-level scan would have
    /// narrowed, this narrows on the same slot; the nested entries are reached
    /// only when nothing before them was usable — which, for a
    /// `(not (R …))` premise, is always.
    fn probes_for(&mut self, slots: Span) -> Span {
        let start = self.probes.len() as u32;
        for (i, idx) in slots.range().enumerate() {
            match self.slots[idx] {
                Slot::Const(v) => {
                    self.probes.push(Probe {
                        slot: i as u16,
                        inner: SlotKey::DIRECT,
                        src: ProbeSrc::Const(v),
                    });
                    // The candidate scan stops at the first known value, so no
                    // later slot can ever be the one it narrows on.
                    break;
                }
                Slot::Reg(r) => self.probes.push(Probe {
                    slot: i as u16,
                    inner: SlotKey::DIRECT,
                    src: ProbeSrc::Reg(r),
                }),
                Slot::Nested { slots: inner, .. } => {
                    for (j, inner_idx) in inner.range().enumerate() {
                        let src = match self.slots[inner_idx] {
                            Slot::Const(v) => ProbeSrc::Const(v),
                            Slot::Reg(r) => ProbeSrc::Reg(r),
                            // A doubly-nested pattern would need a two-level
                            // key the index does not hold; an opaque slot has
                            // no value at all.
                            Slot::Nested { .. } | Slot::Opaque(_) => continue,
                        };
                        self.probes.push(Probe {
                            slot: i as u16,
                            inner: j as u16,
                            src,
                        });
                    }
                }
                // An opaque slot has no value at all.
                Slot::Opaque(_) => {}
            }
        }
        Span {
            start,
            len: self.probes.len() as u32 - start,
        }
    }
}

// ── Plan introspection (pure walks; no KB) ─────────────────────────

/// The positive relation a plan's `:assert` concludes, or `None`.
///
/// A plan whose assert template is `(R …)` with head not `not` proves `R` is
/// rule-derivable.
pub fn asserted_relation(plan: &Plan, terms: &Terms) -> Option<Symbol> {
    match plan.assert_template()? {
        Slot::Nested { rel, .. } if rel != terms.kernel.not => Some(rel),
        _ => None,
    }
}

/// The relation `R` a plan's `:assert` *negates* via `(not (R …))`, or `None`.
///
/// The dual of [`asserted_relation`]: it answers "does some rule derive a
/// `(not (R …))` fact?", which is what an `(absent (not (R …)))` guard — a
/// `forall` / totality NAF — watches.
pub fn negated_relation(plan: &Plan, terms: &Terms) -> Option<Symbol> {
    match plan.assert_template()? {
        Slot::Nested { rel, slots } if rel == terms.kernel.not => {
            plan.slots(slots).iter().find_map(|s| match s {
                Slot::Nested { rel, .. } => Some(*rel),
                _ => None,
            })
        }
        _ => None,
    }
}

/// `(relation, negated)` for every relation the plan's guards watch, in walk
/// order, duplicates included — the caller dedupes.
///
/// Recurses through nested guards (the `forall` macro expands to
/// `(absent (and G (absent B)))`, and Q-S1.7.4.B says enter both levels). A
/// `(not (R …))` sub-pattern yields `(R, true)`: the genuine watched relation
/// is the nested one. Activator-bound head vars are already substituted, so
/// the names here are concrete and per-activator.
pub fn naf_relation_refs(plan: &Plan, terms: &Terms) -> Vec<(Symbol, bool)> {
    fn walk(plan: &Plan, terms: &Terms, span: Span, out: &mut Vec<(Symbol, bool)>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) if r.rel == terms.kernel.not => {
                    for slot in plan.slots(r.slots) {
                        if let Slot::Nested { rel, .. } = slot {
                            out.push((*rel, true));
                        }
                    }
                }
                Step::Rel(r) => out.push((r.rel, false)),
                Step::Absent { sub } => walk(plan, terms, *sub, out),
                Step::Guard { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for d in plan.disjuncts.iter() {
        for g in plan.guards(d.guards) {
            walk(plan, terms, g.sub, &mut out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{GuardArgKind, ProbeSrc};
    use ein_ir::{from_ir::load, parse};

    /// Compile the first rule of `src` against the first activator
    /// `activators_for` offers it — the engine's own choice.
    fn compile(src: &str) -> (Ast, Terms, Plan) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let activator = activators_for(&kb, &terms, &rule)
            .into_iter()
            .next()
            .expect("an activator");
        let plan = compile_rule(&ast, &mut terms, &rule, activator).expect("compiles");
        (ast, terms, plan)
    }

    /// The key is the activator's values, so two activators that are two
    /// facts are two keys and two plans, and neither loses its conclusions
    /// (`W16`): two names of digits, `7` and `07`, and a name from a nested
    /// fact. The key's text is each value's `display`, the one printer's.
    #[test]
    fn plan_key_is_the_activators_values() {
        let src = "(relation edge A B :why \"there is an edge from {?1} to {?2}\")\n(rule walk (?R ?N)\n  :match (?R ?a ?b)\n  :assert (?R ?b ?a) :why \"{?R} holds of {?a} and {?b}, so {?R} holds of {?b} and {?a}\")\n\
                   (walk edge 7)\n(walk edge 07)\n(walk edge seven)\n(walk edge seven)\n\
                   (walk edge (edge A B))\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let acts = activators_for(&kb, &terms, &rule);
        assert_eq!(acts.len(), 4, "`seven` twice is one fact");
        let keys: Vec<PlanKey> = acts.iter().map(|&a| plan_key(&terms, &rule, a)).collect();
        let distinct: FxHashSet<&PlanKey> = keys.iter().collect();
        assert_eq!(distinct.len(), 4, "one key per activator: {keys:?}");
        let mut text: Vec<Vec<String>> = keys.iter().map(|k| k.activator_text(&terms)).collect();
        text.sort();
        assert_eq!(
            text,
            [
                ["edge", "(edge A B)"],
                ["edge", "07"],
                ["edge", "7"],
                ["edge", "seven"]
            ]
        );
        let no_params = "(relation edge A B :why \"there is an edge from {?1} to {?2}\")\n(rule turn ()\n  :match (edge ?a ?b)\n  :assert (edge ?b ?a) :why \"there is an edge from {?a} to {?b}, so there is one from {?b} to {?a}\")\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, no_params, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        assert!(plan_key(&terms, &rule, None).activator.is_empty());
    }

    fn reg_named(terms: &Terms, plan: &Plan, name: &str) -> Reg {
        plan.reg_names
            .iter()
            .position(|&s| terms.sym(s) == name)
            .unwrap_or_else(|| panic!("no register for ?{name}")) as Reg
    }

    /// The activator binding is materialised into the register file, in
    /// `bindings_seed` order — which is the order `Provenance.bindings` starts
    /// with, since the seed is bound first.
    #[test]
    fn the_seed_takes_the_first_registers_in_binding_order() {
        let (_, terms, plan) = compile(
            "(relation edge A B :why \"there is an edge from {?1} to {?2}\")\n\
             (rule walk (?R ?T)\n  :match (?R ?a ?b)\n  :assert (?T ?a ?b) :why \"{?R} holds of {?a} and {?b}, so {?T} holds of {?a} and {?b}\")\n\
             (walk edge edge)\n",
        );
        let seeded: Vec<&str> = plan.seed.iter().map(|(name, _)| terms.sym(*name)).collect();
        assert_eq!(seeded, ["R", "T"]);
        assert_eq!(terms.sym(plan.reg_names[0]), "R");
        assert_eq!(terms.sym(plan.reg_names[1]), "T");
        // …and a seeded parameter is a *constant* in a slot, never a register:
        // `slot` substitutes it before the matcher ever sees it.
        assert!(matches!(
            plan.steps(plan.disjuncts[0].steps)[0],
            Step::Rel(RelStep { rel, .. }) if terms.sym(rel) == "edge"
        ));
    }

    /// A lifted guard is exactly as strong as the guard written in place.
    ///
    /// `(and (absent (block ?x)) (a ?x))` asks "is there no block at all?" and
    /// `(and (a ?x) (absent (block ?x)))` asks "is there no block for *this*
    /// x?" — the whole reason `NafGuard.scope` exists. Both are judged at the
    /// end, so the distinction has to survive as the projection.
    #[test]
    fn a_guards_scope_decides_which_question_it_asks() {
        let before = "(relation a Thing :why \"a holds of {?1}\")\n(relation block Thing :why \"there is a block for {?1}\")\n\
             (rule r ()\n  :match (and (absent (block ?x)) (a ?x))\n  :assert (a ?x) :why \"a holds of {?x} and there is no block at all, so a holds of {?x}\")\n";
        let after = "(relation a Thing :why \"a holds of {?1}\")\n(relation block Thing :why \"there is a block for {?1}\")\n\
             (rule r ()\n  :match (and (a ?x) (absent (block ?x)))\n  :assert (a ?x) :why \"a holds of {?x} and there is no block for {?x}, so a holds of {?x}\")\n";

        let (_, terms, plan) = compile(before);
        let g = &plan.guards[0];
        assert!(g.scope.is_empty(), "nothing precedes the guard");
        assert_eq!(
            g.scope_of.as_ref(),
            [None],
            "?x is free in the query — it ranges over every block"
        );

        let (_, terms2, plan2) = compile(after);
        let g2 = &plan2.guards[0];
        assert_eq!(
            g2.scope.iter().map(|&s| terms2.sym(s)).collect::<Vec<_>>(),
            ["x"]
        );
        assert_eq!(
            g2.scope_of.as_ref(),
            [Some(reg_named(&terms2, &plan2, "x"))],
            "?x is projected from the parent — the query is about this x"
        );
        let _ = terms;
    }

    /// A predicate guard's arguments are raw IR nodes, so a *parameter* inside
    /// an `(absent …)` resolves through the runtime environment — which is why
    /// `split_naf` seeds every scope with the rule's parameters. Resolve it to
    /// nothing and the negative query finds nothing and the guard passes when
    /// it must fail.
    #[test]
    fn a_parameter_under_a_guard_resolves_through_the_seed() {
        let (_, terms, plan) = compile(
            "(relation p A B :why \"p holds of {?1} and {?2}\")\n\
             (rule r (?K)\n\
             \x20 :match (and (p ?a ?b) (absent (and (p ?a ?y) (neq ?y ?K))))\n\
             \x20 :assert (p ?b ?a) :why \"p holds of {?a} and {?b}, and of {?a} with no second but {?K}, so p holds of {?b} and {?a}\")\n\
             (r sentinel)\n",
        );
        let g = &plan.guards[0];
        assert!(
            g.scope.iter().any(|&s| terms.sym(s) == "K"),
            "the parameter must be in scope for the guard"
        );
        let sub = plan.steps(g.sub);
        let Step::Guard { args, .. } = sub
            .iter()
            .find(|s| matches!(s, Step::Guard { .. }))
            .unwrap()
        else {
            unreachable!()
        };
        let k = plan.guard_args(*args)[1];
        let GuardArgKind::Reg(r) = k.kind else {
            panic!("?K did not resolve to a register: {k:?}")
        };
        assert_eq!(
            g.scope_of[r as usize],
            Some(reg_named(&terms, &plan, "K")),
            "?K must project from the seed register, not start free"
        );
    }

    /// The probe list is the candidate scan, pre-filtered: constants end it
    /// (the scan stops at the first known value), registers stay in it (they
    /// may be unbound, or hold a nested fact), and nested patterns are skipped
    /// because the index has no key for one.
    #[test]
    fn the_probe_list_is_the_candidates_scan_narrowed() {
        let (_, terms, plan) = compile(
            "(relation p A B C :why \"p holds of {?1}, {?2} and {?3}\")\n(relation q A :why \"q holds of {?1}\")\n\
             (rule r ()\n  :match (and (q ?a) (p ?a marker ?c))\n  :assert (q ?c) :why \"q holds of {?a} and p of {?a}, marker and {?c}, so q holds of {?c}\")\n",
        );
        let steps = plan.steps(plan.disjuncts[0].steps);
        let Step::Rel(second) = steps[1] else {
            panic!("expected a relation step")
        };
        let probes = plan.probes(second.probe);
        assert_eq!(probes.len(), 2, "?a then the literal, and nothing past it");
        assert_eq!(probes[0].slot, 0);
        assert!(matches!(probes[0].src, ProbeSrc::Reg(_)));
        assert_eq!(probes[1].slot, 1);
        assert!(matches!(probes[1].src, ProbeSrc::Const(_)));
        let _ = terms;
    }

    /// A nested pattern has no key of its own and two of its slots'. Without
    /// them this premise would compile to an empty probe list and walk `not`'s
    /// whole extent, most of an exhaustive `zebra`'s candidates.
    #[test]
    fn a_nested_pattern_probes_one_level_in() {
        let (_, _terms, plan) = compile(
            "(relation likes A B :why \"{?1} likes {?2}\")\n\
             (rule r ()\n  :match (not (likes ?a ?b))\n  :assert (likes ?b ?a) :why \"{?a} does not like {?b}, so {?b} likes {?a}\")\n",
        );
        let steps = plan.steps(plan.disjuncts[0].steps);
        let Step::Rel(first) = steps[0] else {
            panic!("expected a relation step")
        };
        let probes = plan.probes(first.probe);
        assert_eq!(
            probes.len(),
            2,
            "one per inner slot, and none for the outer"
        );
        for (i, p) in probes.iter().enumerate() {
            assert_eq!(p.slot, 0, "the outer position is the nested argument");
            assert_eq!(p.inner, i as u16);
            assert!(matches!(p.src, ProbeSrc::Reg(_)));
        }
    }

    /// The same premise with a variable used once inside it, where a program
    /// might want a wildcard (a bare `_` is refused, Q-M1e2.22): a variable is
    /// a register, and a register stays a probe, since it may be unbound, so
    /// the nested premise probes on both inner slots.
    #[test]
    fn a_nested_pattern_with_no_keyable_slot_has_no_probe() {
        let (_, _terms, plan) = compile(
            "(relation likes A B :why \"{?1} likes {?2}\")\n\
             (rule r ()\n  :match (not (likes ?any ?b))\n  :assert (likes ?b ?b) :why \"{?any} does not like {?b}, so {?b} likes {?b}\")\n",
        );
        let steps = plan.steps(plan.disjuncts[0].steps);
        let Step::Rel(first) = steps[0] else {
            panic!("expected a relation step")
        };
        let probes = plan.probes(first.probe);
        assert_eq!(probes.len(), 2, "`?any`, then `?b`");
        for (i, p) in probes.iter().enumerate() {
            assert_eq!((p.slot, p.inner), (0, i as u16));
            assert!(matches!(p.src, ProbeSrc::Reg(_)));
        }
    }

    /// A bare `_` is refused at the parse, in every position (Q-M1e2.22,
    /// `W20`): the program is refused at each of its four `_`, and a variable
    /// used once matches any value instead.
    #[test]
    fn a_wildcard_matches_any_value_and_binds_nothing() {
        let src = "(relation likes A B)\n\
                   (rule r ()\n  :match (and (likes _ ?b) (likes ?b _) (likes _ _))\n  :assert (likes ?b ?b))\n";
        let refused = parse(&mut Ast::new(), src, Some("<test>")).expect_err("refused");
        let at: Vec<(u32, u32)> = refused.errors.iter().map(|e| (e.line, e.col)).collect();
        assert_eq!(at, [(3, 22), (3, 38), (3, 48)], "{refused}");
        assert!(
            refused
                .errors
                .iter()
                .all(|e| e.what == "unexpected `_`: _ (reserved)"),
            "{refused}"
        );
        let (_, terms, plan) = compile(
            "(relation likes A B :why \"{?1} likes {?2}\")\n\
             (rule r ()\n  :match (and (likes ?u ?b) (likes ?b ?v) (likes ?w ?z))\n  :assert (likes ?b ?b) :why \"{?u} likes {?b}, {?b} likes {?v} and {?w} likes {?z}, so {?b} likes {?b}\")\n",
        );
        let mut names: Vec<&str> = plan.reg_names.iter().map(|&s| terms.sym(s)).collect();
        names.sort_unstable();
        assert_eq!(
            names,
            ["b", "u", "v", "w", "z"],
            "a variable takes a register"
        );
    }

    /// A string is natural-language text, written only after `:why`,
    /// `:source` or `:goal-text`, so a string in a pattern and in an `:assert`
    /// is refused at the parse, each with its note (`W18`); the name written
    /// bare is the key.
    #[test]
    fn a_string_in_a_pattern_is_the_symbol_it_names() {
        let src = "(relation likes A B)\n\
                   (rule r ()\n  :match (not (likes \"lit\" ?b))\n  :assert (likes ?b \"lit\"))\n";
        let refused = parse(&mut Ast::new(), src, Some("<test>")).expect_err("refused");
        let at: Vec<(u32, u32)> = refused.errors.iter().map(|e| (e.line, e.col)).collect();
        assert_eq!(at, [(3, 22), (4, 21)], "{refused}");
        assert!(
            refused.errors.iter().all(|e| e.notes.len() == 1
                && e.notes[0]
                    .starts_with("\"lit\" is a string, and a string is natural-language text")),
            "{refused}"
        );
        let (_, mut terms, plan) = compile(
            "(relation likes A B :why \"{?1} likes {?2}\")\n\
             (rule r ()\n  :match (not (likes lit ?b))\n  :assert (likes ?b lit) :why \"lit does not like {?b}, so {?b} likes lit\")\n",
        );
        let lit = Value::sym(terms.intern_text("lit").expect("room"));
        let steps = plan.steps(plan.disjuncts[0].steps);
        let Step::Rel(first) = steps[0] else {
            panic!("expected a relation step")
        };
        let probes = plan.probes(first.probe);
        assert_eq!(probes.len(), 2, "the name, then `?b`");
        assert_eq!((probes[0].slot, probes[0].inner), (0, 0));
        assert!(matches!(probes[0].src, ProbeSrc::Const(v) if v == lit));
        assert_eq!((probes[1].slot, probes[1].inner), (0, 1));
        let Slot::Nested { slots, .. } = plan.asserts[0] else {
            panic!("a fact template")
        };
        assert!(
            matches!(plan.slots(slots)[1], Slot::Const(v) if v == lit),
            "the `:assert`'s name is the same symbol"
        );
    }

    /// `known_vars` is one set for the whole disjunct, and an `(absent …)`
    /// sub-plan writes into it — so a variable first seen inside a guard makes
    /// a *later* top-level premise a `Join`. It is informational, and it is in
    /// the compiled shape, so it is kept rather than tidied.
    #[test]
    fn a_guards_variables_leak_into_the_scan_join_labelling() {
        let (_, terms, plan) = compile(
            "(relation a Thing :why \"a holds of {?1}\")\n(relation b Thing :why \"b holds of {?1}\")\n(relation block Thing :why \"there is a block for {?1}\")\n\
             (rule r ()\n  :match (and (a ?p) (absent (block ?q)) (b ?q))\n  :assert (b ?p) :why \"a holds of {?p}, b of {?q}, and there is no block at all, so b holds of {?p}\")\n",
        );
        let steps = plan.steps(plan.disjuncts[0].steps);
        assert_eq!(
            steps.len(),
            2,
            "the guard is lifted out of the closure plan"
        );
        let Step::Rel(second) = steps[1] else {
            panic!("expected a relation step")
        };
        assert!(
            second.join,
            "?q was first seen inside the guard, so `(b ?q)` is a Join"
        );
        assert_eq!(
            plan.shared(second.shared)
                .iter()
                .map(|&s| terms.sym(s))
                .collect::<Vec<_>>(),
            ["q"]
        );
        // …and it is a Join over a variable the *guard* cannot see: `?q` is
        // not in the guard's scope, so the query ranges over every block.
        assert!(plan.guards[0].scope_of.iter().all(|s| s.is_none()));
    }

    /// A top-level `(or …)` is one plan with several disjuncts, each with its
    /// own guards — the pairing that keeps a guard from being judged in a
    /// disjunct it was not written in.
    #[test]
    fn each_disjunct_carries_its_own_guards() {
        let (_, terms, plan) = compile(
            "(relation a Thing :why \"a holds of {?1}\")\n(relation block Thing :why \"there is a block for {?1}\")\n(relation other Thing :why \"other holds of {?1}\")\n\
             (rule r ()\n\
             \x20 :match (or (and (a ?x) (absent (block ?x)))\n\
             \x20             (and (a ?x) (absent (other ?x))))\n\
             \x20 :assert (a ?x) :why \"a holds of {?x}, and there is no block for {?x} or other does not hold of {?x}, so a holds of {?x}\")\n",
        );
        assert_eq!(plan.disjuncts.len(), 2);
        let watched: Vec<Vec<&str>> = plan
            .disjuncts
            .iter()
            .map(|d| {
                plan.guards(d.guards)
                    .iter()
                    .flat_map(|g| g.watched.iter().map(|&s| terms.sym(s)))
                    .collect()
            })
            .collect();
        assert_eq!(watched, [["block"], ["other"]]);
        assert!(plan.has_naf());
    }

    /// `(absent (and G (absent B)))` — the `forall` shape. The inner guard is
    /// part of the negative query, not of the closure, so it is **not** lifted
    /// and it makes the outer one non-monotone: adding a `B` makes the inner
    /// fail and the outer pass, so a candidate it rejects is waiting, not dead.
    #[test]
    fn a_nested_absent_stays_in_the_query_and_costs_monotonicity() {
        let (_, terms, plan) = compile(
            "(relation player Thing :why \"{?1} is a player\")\n(relation beats A B :why \"{?1} beats {?2}\")\n\
             (rule undefeated ()\n\
             \x20 :match (and (player ?p)\n\
             \x20              (absent (and (player ?q) (neq ?p ?q) (absent (beats ?p ?q)))))\n\
             \x20 :assert (player ?p) :why \"{?p} is a player who beats every other player, so {?p} is a player\")\n",
        );
        assert_eq!(plan.guards.len(), 1, "only the top-level guard is lifted");
        let g = &plan.guards[0];
        assert!(!g.monotone);
        assert_eq!(
            g.watched.iter().map(|&s| terms.sym(s)).collect::<Vec<_>>(),
            ["beats", "player"],
            "watched reaches through the nested guard, sorted"
        );
        assert!(
            plan.steps(g.sub)
                .iter()
                .any(|s| matches!(s, Step::Absent { .. })),
            "the inner absent is a step of the query"
        );
    }

    /// A `:assert (and …)` concludes several facts from one application, and
    /// the introspection helpers read the *first* template — the
    /// `assert_template` back-compat reader.
    #[test]
    fn a_multi_assert_keeps_every_template_and_answers_for_the_first() {
        let (_, terms, plan) = compile(
            "(relation a Thing :why \"a holds of {?1}\")\n(relation b Thing :why \"b holds of {?1}\")\n\
             (rule r ()\n  :match (a ?x)\n  :assert (and (b ?x) (not (a ?x))) :why \"a holds of {?x}, so b holds of {?x} and a does not\")\n",
        );
        assert_eq!(plan.asserts.len(), 2);
        assert_eq!(
            asserted_relation(&plan, &terms).map(|s| terms.sym(s)),
            Some("b")
        );
        assert_eq!(
            negated_relation(&plan, &terms),
            None,
            "the first is not a `not`"
        );
    }

    #[test]
    fn a_negated_assert_names_the_relation_it_negates() {
        let (_, terms, plan) = compile(
            "(relation a Thing :why \"a holds of {?1}\")\n\
             (rule r ()\n  :match (a ?x)\n  :assert (not (a ?x)) :why \"a holds of {?x}, so a does not hold of {?x}\")\n",
        );
        assert_eq!(asserted_relation(&plan, &terms), None);
        assert_eq!(
            negated_relation(&plan, &terms).map(|s| terms.sym(s)),
            Some("a")
        );
    }

    /// The memo is what makes Win A exact: one compile per distinct
    /// `(rule, activator)` pair, for the whole process.
    #[test]
    fn the_memo_compiles_a_pair_once() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let src = "(relation edge A B :why \"there is an edge from {?1} to {?2}\")\n\
                   (rule walk (?R ?T)\n  :match (?R ?a ?b)\n  :assert (?T ?a ?b) :why \"{?R} holds of {?a} and {?b}, so {?T} holds of {?a} and {?b}\")\n\
                   (walk edge edge)\n";
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let activator = activators_for(&kb, &terms, &rule)[0];
        let mut memo = PlanMemo::new();
        let a = memo.intern(&ast, &mut terms, &rule, activator).expect("ok");
        let b = memo.intern(&ast, &mut terms, &rule, activator).expect("ok");
        assert_eq!(a, b);
        assert_eq!(memo.len(), 1);
    }
}
