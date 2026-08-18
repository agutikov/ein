//! The typed views over the graph: relations, rules, macros, and the
//! insertion-ordered registry they live in.
//!
//! ein.py's entities carry a `_kb` back-pointer so that `Relation.facts` and
//! friends can answer without an argument. This port has none — an accessor
//! takes `&Kb` — which is why the caveat `store.py` documents (a shared
//! entity's back-pointer sees the *root*'s facts, so `Relation.facts` on a
//! fork answers for the wrong KB) has nothing to reproduce.
//!
//! Since S1.7.23 there are no `Type` / `Instance` entities: the kernel imposes
//! no type system, so a puzzle's inheritance forest is just `is-a` facts.

use crate::intern::Symbol;
use crate::value::IntId;
use rustc_hash::FxHashMap;

/// A source position, as the frontend recorded it.
///
/// The frontend's own `Loc` names its file through a `FileId` into the AST's
/// file table; this is the same three numbers with the table left behind, so
/// the data model does not depend on the parser
/// ([design/12](../../../../plans/m1a_rust/design/12_toolchain_and_layout.md) §1
/// — everything depends on `ein-core`, and `ein-core` depends on nothing).
///
/// In practice almost every one of these is `None`: ein.py's `_topform` builds
/// its `SForm` without a `loc`, so a loader error that interpolates
/// `at {form.loc}` prints `at None` — Q-M1a.6, reproduced rather than fixed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Loc {
    pub file: u32,
    pub line: u32,
    pub col: u32,
}

/// A node in the program's syntax arena, opaque here.
///
/// A rule's `:match` / `:assert` clause stays as parsed until the compiler
/// lowers it to plan bytecode ([design/05](../../../../plans/m1a_rust/design/05_matcher.md) §2),
/// so the KB has to *hold* syntax without *knowing* it. It holds this
/// instead: the loader converts from the frontend's `NodeId` and the compiler
/// converts back, and neither direction costs anything.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExprRef(pub u32);

/// What a name is, in `NameRef`'s vocabulary.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NameCategory {
    Object,
    Relation,
    Rule,
}

impl NameCategory {
    /// The spelling ein.py uses — a `Literal["object", "relation", "rule"]`.
    pub fn as_str(self) -> &'static str {
        match self {
            NameCategory::Object => "object",
            NameCategory::Relation => "relation",
            NameCategory::Rule => "rule",
        }
    }
}

/// A relation declaration — `(relation Name T1 T2 … :kw v)`.
///
/// `signature` holds argument-position types **by name**: opaque atoms naming
/// whatever a puzzle declared, used only as object-exclusion metadata by
/// hypgen. An *empty* signature is legal (S1.22.4) and is deliberately not a
/// hypothesis target — the "declared domain relation" signal is signature
/// presence.
///
/// A relation entity is also created on the fly for open-world heads: the
/// property tags (`(symmetric co-located)`) are fact heads whose atom is the
/// name of a *rule*, and auto-creating a relation for them is what lets them
/// participate in the cross-reference indexes.
#[derive(Clone, Debug)]
pub struct Relation {
    pub name: Symbol,
    pub signature: Box<[Symbol]>,
    pub declared: bool,
    /// The `:why` render template. `None` is ein.py's `""`.
    pub why: Option<Symbol>,
    pub loc: Option<Loc>,
}

/// The structural view of a `:match` / `:assert` clause — `kb/pattern.py`.
///
/// No matching semantics: `expr` is the clause as parsed, and the two lists
/// are the pre-computed views the KB itself needs (`relation_names` feeds
/// `rules_by_relation`).
#[derive(Clone, Debug)]
pub struct Pattern {
    pub expr: ExprRef,
    pub variables: Box<[Symbol]>,
    pub relation_names: Box<[Symbol]>,
}

/// A rewrite rule — `(rule Name (?p1 ?p2 …) :match … :assert … …)`, or a
/// hypothesis rule, which is a rule by shape and lives in its own registry so
/// the saturator never fires it.
#[derive(Clone, Debug)]
pub struct Rule {
    pub name: Symbol,
    pub params: Box<[Symbol]>,
    pub match_: Option<Pattern>,
    pub assert_: Option<Pattern>,
    pub why: Option<Symbol>,
    /// An `Int` literal, so it is pooled rather than parsed: the grammar
    /// accepts any width and the saturator only ever compares priorities.
    pub priority: Option<IntId>,
    pub loc: Option<Loc>,
}

/// A pattern macro — `(macro NAME (?p…) BODY)`.
///
/// Consumed at load time only: the loader expands each rule clause's
/// invocations before compiling, and nothing reads the registry afterwards.
/// It is kept as an inspectable record, shared by reference across forks like
/// the other registries.
#[derive(Clone, Debug)]
pub struct Macro {
    pub name: Symbol,
    pub params: Box<[Symbol]>,
    pub body: ExprRef,
    pub loc: Option<Loc>,
}

/// A `(query …)` block, keeping its raw kw-pairs — interpretation is the
/// hypothesis loop's.
#[derive(Clone, Debug, Default)]
pub struct Query {
    pub kw_pairs: Box<[ExprRef]>,
}

/// An insertion-ordered map from [`Symbol`] to `V` — a Python `dict`, whose
/// order is a language guarantee and is observable through
/// `hypgen._raw_candidates` and `Engine.compile_all`
/// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §2).
#[derive(Clone, Debug)]
pub struct Registry<V> {
    entries: Vec<(Symbol, V)>,
    index: FxHashMap<Symbol, u32>,
}

impl<V> Default for Registry<V> {
    fn default() -> Self {
        Registry {
            entries: Vec::new(),
            index: FxHashMap::default(),
        }
    }
}

impl<V> Registry<V> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert under a name that is not taken; `false` (and no change) when it
    /// is — the first declaration wins, as `add_rule` / `add_hrule` do.
    pub fn insert_new(&mut self, name: Symbol, value: V) -> bool {
        if self.index.contains_key(&name) {
            return false;
        }
        self.index.insert(name, self.entries.len() as u32);
        self.entries.push((name, value));
        true
    }

    /// Overwrite in place, **keeping the original position** — what
    /// `add_relation`'s declared-wins upgrade needs, since a re-declaration
    /// must not move the relation in iteration order.
    pub fn replace(&mut self, name: Symbol, value: V) {
        match self.index.get(&name) {
            Some(&at) => self.entries[at as usize].1 = value,
            None => {
                self.insert_new(name, value);
            }
        }
    }

    pub fn get(&self, name: Symbol) -> Option<&V> {
        self.index
            .get(&name)
            .map(|&at| &self.entries[at as usize].1)
    }

    pub fn get_mut(&mut self, name: Symbol) -> Option<&mut V> {
        match self.index.get(&name) {
            Some(&at) => Some(&mut self.entries[at as usize].1),
            None => None,
        }
    }

    pub fn contains(&self, name: Symbol) -> bool {
        self.index.contains_key(&name)
    }

    /// Entries in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (Symbol, &V)> {
        self.entries.iter().map(|(k, v)| (*k, v))
    }

    /// Names in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.entries.iter().map(|(k, _)| *k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<V: Default> Registry<V> {
    /// The entry for `name`, creating an empty one at the end if absent.
    pub fn entry(&mut self, name: Symbol) -> &mut V {
        if !self.index.contains_key(&name) {
            self.insert_new(name, V::default());
        }
        self.get_mut(name).expect("just inserted")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_registry_iterates_in_insertion_order() {
        let mut r: Registry<u32> = Registry::new();
        assert!(r.insert_new(Symbol(7), 70));
        assert!(r.insert_new(Symbol(1), 10));
        assert!(!r.insert_new(Symbol(7), 99), "first declaration wins");
        assert_eq!(r.get(Symbol(7)), Some(&70));
        assert_eq!(r.keys().collect::<Vec<_>>(), vec![Symbol(7), Symbol(1)]);
    }

    #[test]
    fn replace_upgrades_without_moving() {
        // `add_relation`'s declared-wins rule replaces the registry entry; an
        // open-world relation that is later declared must keep its place, or
        // `hypgen._raw_candidates` would enumerate in a different order.
        let mut r: Registry<&str> = Registry::new();
        r.insert_new(Symbol(1), "open");
        r.insert_new(Symbol(2), "other");
        r.replace(Symbol(1), "declared");
        assert_eq!(
            r.iter().collect::<Vec<_>>(),
            vec![(Symbol(1), &"declared"), (Symbol(2), &"other")]
        );
    }
}
