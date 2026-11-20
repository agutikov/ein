//! What a candidate of the blind enumerator names that is no object of the
//! puzzle: the marks `object_census` defines and `rung_census` reads.
//!
//! A candidate is read against root's fixpoint and the main file as written.
//! Each set is a column of `object_census`, whose module docs say what each
//! one holds; `E17` is `K1 ∪ K2 ∪ R1 ∪ R2`, `E54` is `IN ∪ IR`, `any` their
//! union.

use std::collections::BTreeSet;

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::events::sexpr;
use ein_ir::{Ast, Node, NodeId};
use rustc_hash::FxHashSet;

/// The marked sets, in column order.
pub const SETS: [&str; N] = ["K1", "K2", "R1", "R2", "E17", "IN", "IR", "E54", "any"];
pub const N: usize = 9;
pub const K1: usize = 0;
pub const K2: usize = 1;
pub const R1: usize = 2;
pub const R2: usize = 3;
pub const E17: usize = 4;
pub const IN: usize = 5;
pub const IR: usize = 6;
pub const E54: usize = 7;
pub const ANY: usize = 8;
/// What the main file writes, read before any import is resolved.
#[derive(Default)]
pub struct MainFile {
    pub imports: bool,
    /// Every name in any form, an import's `:symbols` list included.
    pub spelled: BTreeSet<String>,
    /// [`Self::spelled`] less what only a `(query …)` writes.
    pub names: BTreeSet<String>,
}

fn atoms(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::Atom(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { head, .. } => {
            atoms(ast, head, out);
            for &a in ast.form_args(node) {
                atoms(ast, a, out);
            }
        }
        Node::KwPair { value, .. } => atoms(ast, value, out),
        _ => {}
    }
}

pub fn main_file(ast: &Ast, forms: &[NodeId]) -> MainFile {
    let mut m = MainFile::default();
    for &f in forms {
        match ast.head_name(f) {
            Some("import") => {
                m.imports = true;
                for &a in ast.form_args(f) {
                    let Node::KwPair { key, value } = ast.node(a) else {
                        continue;
                    };
                    if matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "symbols") {
                        let mut listed = BTreeSet::new();
                        atoms(ast, value, &mut listed);
                        m.names.extend(listed.iter().cloned());
                        m.spelled.extend(listed);
                    }
                }
            }
            Some("query") => atoms(ast, f, &mut m.spelled),
            _ => {
                let mut own = BTreeSet::new();
                atoms(ast, f, &mut own);
                m.names.extend(own.iter().cloned());
                m.spelled.extend(own);
            }
        }
    }
    m
}

/// Every symbol in an argument position of a fact, at any depth.
fn arg_symbols(terms: &Terms, fact: FactId, out: &mut Vec<Symbol>) {
    fn walk(terms: &Terms, v: Value, out: &mut Vec<Symbol>) {
        if let Some(s) = v.as_sym() {
            out.push(s);
        } else if let Some(f) = v.as_fact() {
            for &a in terms.facts.args(f) {
                walk(terms, a, out);
            }
        }
    }
    for &a in terms.facts.args(fact) {
        walk(terms, a, out);
    }
}

/// A name no program binds: the reserved words, the structural primitives,
/// the predicates, and what the loader or the engine makes up.
pub fn kernel_name(text: &str) -> bool {
    ein_core::terms::RESERVED.contains(&text)
        || ein_core::terms::STRUCTURAL.contains(&text)
        || ein_core::terms::PREDICATES.contains(&text)
        || ein_core::text::is_internal(text)
        || text.starts_with('<')
}

/// A candidate list, each candidate marked.
#[derive(Default)]
pub struct Marked {
    /// The names each set marks, sorted (`E17`, `E54` and `any` left empty).
    pub names: [BTreeSet<String>; N],
    /// Each candidate's text, and the sets it is in.
    pub marks: Vec<(String, [bool; N])>,
}

impl Marked {
    pub fn count(&self, set: usize) -> usize {
        self.marks.iter().filter(|(_, m)| m[set]).count()
    }
}

/// What the marks are read against: root's fixpoint and the main file.
pub struct Reader<'a> {
    kb: &'a Kb,
    terms: &'a Terms,
    main: &'a MainFile,
    /// The names of the program: every constant in an argument position.
    written: FxHashSet<Symbol>,
    /// Every relation a rule's `:match` or `:assert` names.
    rule_heads: FxHashSet<Symbol>,
}

impl<'a> Reader<'a> {
    /// The reader of one program: `kb` at root's fixpoint, `main` its main
    /// file as written.
    pub fn new(kb: &'a Kb, terms: &'a Terms, ast: &Ast, main: &'a MainFile) -> Reader<'a> {
        let mut rule_heads: FxHashSet<Symbol> = FxHashSet::default();
        for reg in [
            &kb.program().rules,
            &kb.program().hypotheses,
            &kb.program().obligations,
        ] {
            for (_, rule) in reg.iter() {
                for p in [rule.match_.as_ref(), rule.assert_.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    rule_heads.extend(p.relation_names.iter().copied());
                }
            }
        }
        Reader {
            kb,
            terms,
            main,
            written: ein_infer::invariant::program_names(kb, terms, ast)
                .into_iter()
                .map(|n| n.name)
                .collect(),
            rule_heads,
        }
    }

    /// Each candidate with the sets it is in.
    pub fn mark(&self, candidates: &[FactId]) -> Marked {
        let mut out = Marked::default();
        for &c in candidates {
            let mut m = [false; N];
            let rel = self.terms.sym(self.terms.facts.get(c).0).to_string();
            if !self.main.spelled.contains(&rel) {
                m[IR] = true;
                out.names[IR].insert(rel);
            }
            let mut args = Vec::new();
            arg_symbols(self.terms, c, &mut args);
            for s in args {
                let text = self.terms.sym(s).to_string();
                let value = self.written.contains(&s);
                let set = if kernel_name(&text) {
                    Some(if value { K2 } else { K1 })
                } else if self.kb.name_as_head(s).next().is_some() || self.rule_heads.contains(&s) {
                    Some(if value { R2 } else { R1 })
                } else {
                    None
                };
                if let Some(set) = set {
                    m[set] = true;
                    out.names[set].insert(text.clone());
                }
                if !self.main.names.contains(&text) {
                    m[IN] = true;
                    out.names[IN].insert(text);
                }
            }
            m[E17] = m[K1] || m[K2] || m[R1] || m[R2];
            m[E54] = m[IN] || m[IR];
            m[ANY] = m[E17] || m[E54];
            out.marks.push((sexpr(self.terms, c), m));
        }
        out
    }
}
