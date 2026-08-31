//! The **M1 alive-set invariant**, as a predicate the engine can evaluate.
//!
//! `docs/kernel/inference/README.md` § M1 invariant states three clauses —
//! rules assert no new objects, no new relations, and hypotheses connect
//! existing names only — and concludes that `alive` is a pure function of the
//! closed KB. Three shipped mechanisms cite that conclusion as their warrant:
//! the per-KB `compute_alive` recompute, the
//! [`state_key`](crate::canon::state_key) dedup that produces `k`, and — since
//! M1d — the tree traversal's exhaustiveness-by-discharge argument. Until M1e
//! S1e.3.3 **nothing evaluated it**, which is `ST-M1`.
//!
//! This is not F5's typed form, which would make a violation unrepresentable.
//! It is the cheap one: two set memberships per fact, against a baseline taken
//! at load.
//!
//! # The three questions the operational form has to answer
//!
//! **What is the baseline?** The *names the loaded program reaches*, not the
//! interner's length. Those are different sets and the difference is the whole
//! finding: `Ann` in `examples/ein-bugs/mixed-type-hypothesis.ein` appears in
//! an `hrule`'s `:assert` and in no fact, so `ein_ir::from_ir::intern_program_names`
//! interns it at **load** — the interner never grows afterwards
//! (`ein-infer/tests/interning.rs`) and the invariant is broken all the same,
//! because the KB gained a name its ontology never had. So the baseline is
//! [`Universe::of`]: every symbol the loaded facts mention, at any depth, plus
//! the relation registry and its signatures.
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
//! by the interner, and the caller decides how often to pay it. What ships is
//! cheaper still: `solve.rs`'s `Run::phase1` runs the **static** half — the
//! rules' `:assert` constants, once, at load — because that answers for every
//! run the program could have rather than for the one that happened, and
//! because it finds every breach the scan finds on the whole corpus
//! (`ein-infer/tests/alive_invariant.rs`).

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
    /// Every symbol the loaded facts mention, at any depth, plus the relation
    /// registry's names and the symbols in their signatures.
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
    /// Clause 1 / 3 — an argument names an object the ontology never had.
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
    /// `<kind>: <name> in <fact>` — the one line a `warn` event and a debug
    /// assertion both want.
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

impl Universe {
    /// The baseline, taken from a **loaded** KB — before any saturation.
    ///
    /// Taking it after root saturation would make the check vacuous at root:
    /// whatever saturation invented would already be in the baseline. The
    /// caller is the loader's neighbour, not the search's.
    pub fn of(kb: &Kb, terms: &Terms) -> Universe {
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

    /// Every **constant** a rule's `:assert` can put into a KB that the
    /// baseline does not have — the invariant read as what it says, *rules*
    /// don't assert facts whose args introduce new names.
    ///
    /// This is the check that matters and it is free: it runs once, over the
    /// program, and it is **total** — it answers for every run the program
    /// could have, where a post-fixpoint scan answers only for the run that
    /// happened. All three violations M1e S1e.3.3 found are here, and the
    /// corpus sweep found none the sweep could see and this could not
    /// (`ein-infer/tests/alive_invariant.rs`).
    ///
    /// A **variable** leaf contributes nothing: a variable is bound by the
    /// match, so it can only carry a value the KB already holds. That is the
    /// induction the dynamic half exists to confirm rather than to assume.
    ///
    /// `fact` on each breach is the `FactId` of nothing — there is no fact
    /// yet — so this reports the rule and the name instead.
    pub fn rule_breaches(&self, kb: &Kb, terms: &Terms, ast: &Ast) -> Vec<RuleBreach> {
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
                    let Some(sym) = terms.syms.get(text) else {
                        return;
                    };
                    let known = if head {
                        self.relations.contains(sym.0)
                    } else {
                        self.objects.contains(sym.0)
                    };
                    if known || seen.contains(&sym) {
                        return;
                    }
                    seen.push(sym);
                    out.push(RuleBreach {
                        rule: name,
                        which,
                        name: sym,
                        kind: if head {
                            BreachKind::NewRelation
                        } else {
                            BreachKind::NewObject
                        },
                    });
                });
            }
        }
        out
    }
}

/// A rule that can introduce a name the loaded program does not have.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RuleBreach {
    /// The declarator's name.
    pub rule: Symbol,
    /// `rule` / `hrule` / `obligation` — which registry it came from.
    pub which: &'static str,
    pub name: Symbol,
    pub kind: BreachKind,
}

impl RuleBreach {
    pub fn render(&self, terms: &Terms) -> String {
        format!(
            "{}: `{}`, asserted by {} `{}`",
            self.kind.as_str(),
            terms.sym(self.name),
            self.which,
            terms.sym(self.rule)
        )
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
