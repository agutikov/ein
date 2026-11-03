//! The **M1 alive-set invariant**, as a predicate the engine can evaluate.
//!
//! `docs/kernel/inference/README.md` § M1 invariant states three clauses —
//! rules assert no new objects, no new relations, and hypotheses connect
//! existing names only — and concludes that the candidates are a pure
//! function of the closed KB. Three mechanisms cited that conclusion as their
//! warrant: the per-KB `compute_alive` recompute, the
//! [`state_key`](crate::canon::state_key) dedup that produces `k`, and — since
//! M1d — the tree traversal's exhaustiveness-by-discharge argument. Until M1e
//! S1e.3.3 **nothing evaluated it**, which is `ST-M1`.
//!
//! # Since M1e2 S1e2.2.4: a rule's constant is a name of the program
//!
//! S1e.3.3 read clause 1 against the names the loaded **facts** mention, and
//! found three programs whose rule asserts a constant no fact names. The user
//! ruled on 2026-10-16 that such a constant is not new
//! ([Q-M1e.21](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about)
//! (b)): *"rules are part of root, so rules can't assert new names"*. So
//! [`rule_constants`] is the one list of what a rule can introduce, and two
//! readers take it: [`Universe::of`], whose baseline is every name of the
//! program, and [`crate::hypgen::candidate_objects`], which **seeds** the
//! blind enumerator's objects with it. `examples/ein-bugs/alive-set-fresh-name.ein`
//! is the program on which a rule's constant missing from the candidates cost
//! a model (M1e2's W4).
//!
//! What a rule can still introduce that no static set predicts is a relation
//! head written as a parameter and bound to an **integer**, whose decimal text
//! the compiler interns as a name. `ein-infer/tests/interning.rs` names it,
//! and no corpus program has one.
//!
//! This is not F5's typed form, which would make a violation unrepresentable.
//! It is the cheap one: two set memberships per fact, against a baseline taken
//! at load.
//!
//! # The three questions the operational form has to answer
//!
//! **What is the baseline?** The *names the loaded program reaches*, not the
//! interner's length. Those are different sets: `Ann` in
//! `examples/ein-bugs/mixed-type-hypothesis.ein` appears in an `hrule`'s
//! `:assert` and in no fact, so `ein_ir::from_ir::intern_program_names`
//! interns it at **load** — the interner never grows afterwards
//! (`ein-infer/tests/interning.rs`). So the baseline is [`Universe::of`]:
//! every symbol the loaded facts mention, at any depth, every constant a
//! rule's `:assert` names, and the relation registry and its signatures.
//!
//! **What is a new relation?** A fact head that is not in
//! [`ein_core::Program::relations`]. That registry is *declared ∪
//! auto-vivified*, exactly the invariant's own phrase: `from_ir` vivifies an
//! undeclared head at load, and nothing can vivify one afterwards, because
//! `Program` is an `Arc` every fork shares and `program_mut` panics once it is
//! shared. A derived fact with an unregistered head is therefore a fact about a
//! relation that does not exist.
//!
//! **When does it run?** At a fixpoint, and the honest scope is *every*
//! fixpoint, because the warrant is used per KB. What it costs is
//! [`Universe::breaches`]'s two bit tests per symbol against a baseline sized
//! by the interner, and the caller decides how often to pay it. No route pays
//! it: `ein-infer/tests/alive_invariant.rs` runs it over every state a bounded
//! describe meets on the whole corpus, and finds nothing.

use ein_core::bitset::BitSet;
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};

/// What the loaded program may name, and what it may name a fact *about*.
///
/// Two bitsets over [`Symbol`], so a membership test is a bit test rather than
/// a hash lookup: the check runs once per fixpoint and a fixpoint can hold
/// thousands of facts.
#[derive(Clone, Debug, Default)]
pub struct Universe {
    /// Every symbol the loaded facts mention, at any depth, every constant a
    /// rule's `:assert` names, plus the relation registry's names and the
    /// symbols in their signatures.
    objects: BitSet,
    /// The relation registry as of load — declared ∪ auto-vivified.
    relations: BitSet,
    /// `terms.syms.len()` when the baseline was taken. A symbol numbered at or
    /// above this did not exist at load at all, which is a stronger fact than
    /// *not in the universe* and is worth reporting as one.
    interned: u32,
}

/// One fact that names something the loaded program did not.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Breach {
    /// The fact that carries the name.
    pub fact: FactId,
    /// The offending symbol.
    pub name: Symbol,
    pub kind: BreachKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreachKind {
    /// Clause 1 / 3 — an argument names an object the program never had.
    NewObject,
    /// Clause 2 — the fact's head is not a relation the program registered.
    NewRelation,
}

impl BreachKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BreachKind::NewObject => "new-object",
            BreachKind::NewRelation => "new-relation",
        }
    }
}

impl Breach {
    /// `<kind>: <name> in <fact>` — the one line a report wants.
    pub fn render(&self, terms: &Terms) -> String {
        format!(
            "{}: `{}` in {}",
            self.kind.as_str(),
            terms.sym(self.name),
            crate::events::sexpr(terms, self.fact)
        )
    }
}

fn walk_value(terms: &Terms, v: Value, out: &mut impl FnMut(Symbol)) {
    match v.tag() {
        ein_core::value::Tag::Sym => out(v.as_sym().expect("tagged Sym")),
        ein_core::value::Tag::Int => {}
        ein_core::value::Tag::Fact => {
            let f = v.as_fact().expect("tagged Fact");
            let (rel, args) = terms.facts.get(f);
            out(rel);
            for &a in args {
                walk_value(terms, a, out);
            }
        }
    }
}

/// A constant a registered rule's `:assert` names as an **object**, in an
/// argument position — a name of the program, which a rule may assert
/// (Q-M1e.21 (b), the user's ruling of 2026-10-16).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RuleConstant {
    /// The declarator's name.
    pub rule: Symbol,
    /// `rule` / `hrule` / `obligation` — which registry it came from.
    pub which: &'static str,
    pub name: Symbol,
}

impl RuleConstant {
    /// `` `Z`, asserted by rule `spawn` `` — what a report names it by.
    pub fn render(&self, terms: &Terms) -> String {
        format!(
            "`{}`, asserted by {} `{}`",
            terms.sym(self.name),
            self.which,
            terms.sym(self.rule)
        )
    }
}

/// **Every constant a rule's `:assert` can put into a KB as an object** — the
/// one list of what a rule can introduce, in registry order, once per rule.
///
/// Free, and **total**: it reads the registered rules once, over the program,
/// so it answers for every run the program could have rather than for the one
/// that happened. All three registries, because an `hrule` is a rule by shape
/// and its constants are names of the program as a rule's are.
///
/// A **variable** leaf contributes nothing: a variable is bound by the match,
/// or by the activator, so it can only carry a value a fact already holds.
/// The one exception is a relation head written as a parameter, which the
/// compiler names by the bound value's text: an integer there is the residue
/// the module docs name. A **head** contributes nothing here: a relation an
/// `:assert` names is the registry's business ([`Universe::of`] reads it off
/// `relation_names`).
pub fn rule_constants(kb: &Kb, terms: &Terms, ast: &Ast) -> Vec<RuleConstant> {
    let mut out = Vec::new();
    for (which, reg) in [
        ("rule", &kb.program().rules),
        ("hrule", &kb.program().hrules),
        ("obligation", &kb.program().obligations),
    ] {
        for (name, rule) in reg.iter() {
            let Some(a) = rule.assert_.as_ref() else {
                continue;
            };
            let mut seen: Vec<Symbol> = Vec::new();
            walk_constants(ast, NodeId(a.expr.0), true, &mut |text, head| {
                if head {
                    return;
                }
                // Every leaf of a registered rule is interned at load
                // (`from_ir::intern_program_names`), so a miss names nothing.
                let Some(sym) = terms.syms.get(text) else {
                    return;
                };
                if seen.contains(&sym) {
                    return;
                }
                seen.push(sym);
                out.push(RuleConstant {
                    rule: name,
                    which,
                    name: sym,
                });
            });
        }
    }
    out
}

/// The constants **only a rule** mentions: [`rule_constants`] that no loaded
/// fact, relation or signature names. What the seed adds to the blind
/// enumerator's objects over what the facts gave it — on the corpus `Ann`,
/// `G` and `Z` (`ein-infer/tests/alive_invariant.rs`).
pub fn seeded(kb: &Kb, terms: &Terms, ast: &Ast) -> Vec<RuleConstant> {
    let facts = Universe::of_facts(kb, terms);
    rule_constants(kb, terms, ast)
        .into_iter()
        .filter(|c| !facts.objects.contains(c.name.0))
        .collect()
}

impl Universe {
    /// The baseline, taken from a **loaded** KB — before any saturation: every
    /// name of the program.
    ///
    /// Taking it after root saturation would make the check vacuous at root:
    /// whatever saturation invented would already be in the baseline. The
    /// caller is the loader's neighbour, not the search's.
    pub fn of(kb: &Kb, terms: &Terms, ast: &Ast) -> Universe {
        let mut u = Universe::of_facts(kb, terms);
        for c in rule_constants(kb, terms, ast) {
            u.objects.insert(c.name.0);
        }
        u
    }

    /// [`Universe::of`] without the rules' constants: what the facts, the
    /// registry and the kernel name. M1e S1e.3.3's baseline, and what
    /// [`seeded`] is measured against.
    fn of_facts(kb: &Kb, terms: &Terms) -> Universe {
        let mut u = Universe {
            objects: BitSet::new(),
            relations: BitSet::new(),
            interned: terms.syms.len() as u32,
        };
        for name in kb.program().relations.keys() {
            u.relations.insert(name.0);
            u.objects.insert(name.0);
        }
        for (_, rel) in kb.program().relations.iter() {
            for &ty in rel.signature.iter() {
                u.objects.insert(ty.0);
            }
        }
        // **…∪ rule-assertable**, which is a widening of the review's
        // *declared ∪ auto-vivified* and the stage's first finding. `from_ir`
        // vivifies an undeclared **fact** head and not an undeclared
        // **rule-`:assert`** head, so a relation a rule derives and no fact
        // states is in no registry at all: **49** such names over 33 corpus
        // files, all but one a stdlib activator (`total` derived from
        // `(bijective …)`, `slot-endpoint-fwd`, `converse-illtyped-dom`, …;
        // the exception is `examples/features/01_not_and_absent.ein`'s own
        // `explicitly-dislikes`). Reading the registry as the whole answer
        // would make the check fire on the standard library on its first
        // run, which is a check that says nothing.
        //
        // `Pattern::relation_names` is the head set already computed at load,
        // with the structural primitives excluded — so this is free and it is
        // the *same* list `rules_by_relation` is built from.
        for reg in [
            &kb.program().rules,
            &kb.program().hrules,
            &kb.program().obligations,
        ] {
            for (_, rule) in reg.iter() {
                let Some(a) = rule.assert_.as_ref() else {
                    continue;
                };
                for &r in a.relation_names.iter() {
                    u.relations.insert(r.0);
                    u.objects.insert(r.0);
                }
            }
        }
        // The kernel's own vocabulary: a stored `(not X)` and a stored
        // `(false)` are facts whose head is reserved and therefore never in
        // the registry, and `__closed__` is written by `emit_closed` after the
        // program is sealed. None of them is a name a *program* introduced.
        for name in ein_core::terms::STRUCTURAL
            .iter()
            .chain(ein_core::terms::PREDICATES.iter())
            .chain(ein_core::terms::ENGINE.iter())
            .chain(ein_core::terms::RESERVED.iter())
        {
            if let Some(sym) = terms.syms.get(name) {
                u.relations.insert(sym.0);
                u.objects.insert(sym.0);
            }
        }
        for f in kb.facts() {
            let (rel, args) = terms.facts.get(f);
            u.objects.insert(rel.0);
            for &a in args {
                walk_value(terms, a, &mut |s| {
                    u.objects.insert(s.0);
                });
            }
        }
        u
    }

    /// Was this symbol numbered after the baseline was taken?
    ///
    /// The interner does not grow during a search
    /// (`ein-infer/tests/interning.rs`), so this is `false` for every breach
    /// the corpus can produce — which is the point of reporting it separately
    /// from *not in the universe*.
    pub fn interned_late(&self, name: Symbol) -> bool {
        name.0 >= self.interned
    }

    /// Every fact of `kb` that names something the baseline did not, appended
    /// to `out` in `kb.facts()` order.
    ///
    /// `from` is a fact-count mark: [`Kb::n_facts`] as of the last check, so a
    /// caller that has already checked a prefix pays only for the delta. A
    /// KB's fact list only ever grows and `push_fact` appends, which is the
    /// same property [`Kb::written_since_saturation`] rests on.
    pub fn breaches(&self, kb: &Kb, terms: &Terms, from: usize, out: &mut Vec<Breach>) {
        for f in kb.facts_from(from) {
            let (rel, args) = terms.facts.get(f);
            if !self.relations.contains(rel.0) {
                out.push(Breach {
                    fact: f,
                    name: rel,
                    kind: BreachKind::NewRelation,
                });
            }
            for &a in args {
                walk_value(terms, a, &mut |s| {
                    if !self.objects.contains(s.0) {
                        out.push(Breach {
                            fact: f,
                            name: s,
                            kind: BreachKind::NewObject,
                        });
                    }
                });
            }
        }
    }

    /// How many names the baseline holds — the report's denominator.
    pub fn len(&self) -> (usize, usize) {
        (self.objects.len(), self.relations.len())
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

/// Every constant leaf of an `:assert` expression, with a flag for *this one
/// is a fact head*.
///
/// The structural primitives are transparent: `(and (p A) (not (q B)))`
/// asserts about `p` and `q`, not about `and` and `not`. A keyword pair
/// contributes its value only — `:priority 250` names no relation — and a
/// variable contributes nothing at all.
fn walk_constants(ast: &Ast, node: NodeId, head_pos: bool, f: &mut impl FnMut(&str, bool)) {
    match ast.node(node) {
        Node::Atom(s) => f(ast.sym(s), head_pos),
        Node::SForm { head, .. } => {
            let structural = ast
                .atom_name(head)
                .is_some_and(|n| ein_core::terms::STRUCTURAL.contains(&n));
            if let Some(name) = ast.atom_name(head)
                && !structural
            {
                f(name, true);
            }
            for &a in ast.form_args(node) {
                walk_constants(ast, a, false, f);
            }
        }
        Node::KwPair { value, .. } => walk_constants(ast, value, false, f),
        _ => {}
    }
}
