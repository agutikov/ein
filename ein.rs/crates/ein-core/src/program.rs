//! The registries — everything a load produces that facts do not.
//!
//! ein.py keeps these on the `KnowledgeBase` and shares them **by reference**
//! across every fork and snapshot, because they are immutable once the loader
//! finishes. Here they are an `Arc<Program>` that every `Kb` holds, which says
//! the same thing in the type system: a fork cannot write one, and sharing one
//! costs a refcount.
//!
//! The `add_*` methods are the loader's, and their rules are load-time
//! semantics rather than bookkeeping — first-declaration-wins for rules,
//! declared-wins-over-open-world for relations.

use crate::config::SolverConfig;
use crate::entities::{Macro, NameCategory, Query, Registry, Relation, Rule};
use crate::intern::Symbol;
use crate::terms::Terms;

#[derive(Default, Debug)]
pub struct Program {
    pub relations: Registry<Relation>,
    pub rules: Registry<Rule>,
    /// Hypothesis rules (S1.5.6b) — a `Rule` by shape, kept out of `rules` so
    /// the saturator, which walks `rules`, never fires one. `hypgen` is the
    /// only consumer. Rules and hrules share **one** name-space.
    pub hrules: Registry<Rule>,
    pub macros: Registry<Macro>,
    pub query: Option<Query>,
    pub config: Option<SolverConfig>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a relation; the *declared* flag wins over open-world.
    ///
    /// An open-world entry that is later declared is upgraded **in place** —
    /// the registry position is kept, because `hypgen._raw_candidates`
    /// enumerates in insertion order.
    pub fn add_relation(&mut self, rel: Relation) {
        match self.relations.get(rel.name) {
            None => {
                self.relations.insert_new(rel.name, rel);
            }
            Some(existing) if rel.declared && !existing.declared => {
                self.relations.replace(rel.name, rel);
            }
            Some(_) => {}
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert_new(rule.name, rule);
    }

    pub fn add_hrule(&mut self, rule: Rule) {
        self.hrules.insert_new(rule.name, rule);
    }

    /// A name's `NameRef` category.
    ///
    /// `relation` and `rule` are the only two kernel forms hardcoded as
    /// `"relation"`. Since S1.7.6, `type` and `instance` are **not** special:
    /// they categorise as relations only when a puzzle declares them.
    /// `hrules` are deliberately absent — a name declared only as an hrule
    /// falls through to `"object"`, as it does in ein.py.
    pub fn categorise(&self, terms: &Terms, name: Symbol) -> NameCategory {
        if name == terms.kernel.relation || name == terms.kernel.rule {
            return NameCategory::Relation;
        }
        if self.relations.contains(name) {
            return NameCategory::Relation;
        }
        if self.rules.contains(name) {
            return NameCategory::Rule;
        }
        NameCategory::Object
    }
}
