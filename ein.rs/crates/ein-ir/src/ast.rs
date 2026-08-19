//! The IR abstract syntax tree — an arena, not a tree of pointers.
//!
//! ein.py's nodes are frozen dataclasses whose `loc` field is
//! `field(compare=False)`, which is the single reason
//! `parse(dump(parse(x))) == parse(x)` holds. ein.rs reproduces that by
//! keeping positions in a **side table**: [`Node`] carries no `Loc`, so
//! structural equality ([`Ast::eq_nodes`]) cannot see one
//! ([design/04](../../../../plans/m1a_rust/design/04_ir_frontend.md) §3).
//!
//! Everything is `u32`-indexed into three parallel arenas — nodes, their
//! locations, and a flat argument list. No `Rc`, no recursive `Drop`, and a
//! subtree copy (macro expansion) is a walk over integers rather than a graph
//! clone.
//!
//! Two ein.py quirks are *deliberately* reproduced here rather than fixed,
//! because they are observable and the harness diffs bytes:
//!
//! - **Synthetic heads.** `()` lowers to a form headed `@empty` and a
//!   `rule`/`macro` parameter list to one headed `@params`; the dumper prints
//!   any `@`-headed form without its head.
//! - **Top-level forms carry no `loc`.** `_topform`, `relation_decl`,
//!   `generic_fact`, `eq_fact`, `not_form`, … all build their `SForm` without
//!   passing one, and only `generic_list` sets `loc=head.loc`. A loader error
//!   that interpolates `at {form.loc}` therefore prints `at None`, and so must
//!   ein.rs (Q-M1a.6 — a real usability bug, fixed in *both* after parity).

use rustc_hash::FxHashMap;

pub use ein_core::pyrepr::canonical_int;
use ein_core::pyrepr::repr_str;

/// An interned string: an atom name, a variable name, a string body, or the
/// canonical decimal text of an integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SymId(pub u32);

/// An index into [`Ast`]'s node arena.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(pub u32);

/// An index into [`Ast`]'s file-name table. `Loc` stores one instead of a
/// string so a `Loc` stays 12 bytes and a whole import tree shares one copy of
/// each path.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FileId(pub u32);

/// A source position: **1-based** line and column, in characters, taken from
/// the token's first character — what Lark's `propagate_positions` yields and
/// what `ir/ast.py::_loc` records.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Loc {
    pub file: FileId,
    pub line: u32,
    pub col: u32,
}

/// A form's arguments: a contiguous slice of [`Ast::args`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArgSpan {
    pub start: u32,
    pub len: u32,
}

impl ArgSpan {
    pub const EMPTY: ArgSpan = ArgSpan { start: 0, len: 0 };

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// One node. No `Loc` (see the module docs) and deliberately **no derived
/// `PartialEq`**: comparing two `Node`s by their child *ids* would answer
/// "same subtree object", where every caller means "same shape". Use
/// [`Ast::eq_nodes`].
#[derive(Clone, Copy, Debug)]
pub enum Node {
    /// `SYMBOL`, and the `=` of an `eq_fact` (`EQ` is a *named* terminal in
    /// the Lark grammar precisely so it survives token filtering and arrives
    /// as `Atom("=")` — in both the fact and the list-head position).
    Atom(SymId),
    /// `?name` — the `?` is not stored.
    Var(SymId),
    /// `:name` — the `:` is not stored. Only ever a [`Node::KwPair`] key.
    Keyword(SymId),
    /// `_`.
    Wildcard,
    /// A double-quoted string, stored **unescaped**.
    Str(SymId),
    /// An integer, stored as its *canonical decimal text* — `007` and `7`
    /// intern to the same symbol, exactly as `Int(value=int(tok))` collapses
    /// them, and the width is unbounded as Python's is
    /// ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §3).
    Int(SymId),
    /// `low..high`, or `low..*` when `high` is `None`. Both bounds are
    /// canonical decimal text, for the same reason as [`Node::Int`].
    Range { low: SymId, high: Option<SymId> },
    /// `:key value`. `key` is a [`Node::Keyword`] node so it keeps its own
    /// `Loc`, as ein.py's `KwPair.key` does.
    KwPair { key: NodeId, value: NodeId },
    /// `(head arg…)`. `head` is an `Atom` for every real form and may be a
    /// `Var` or `Wildcard` inside a pattern interior (`(?rel ?a ?b)`).
    SForm { head: NodeId, args: ArgSpan },
}

/// String interner. Never iterated — only `intern` and index lookups — so the
/// hash map's order cannot reach an observable
/// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §9),
/// and it may therefore be hashed by whatever is fastest. SipHash was **7 %**
/// of a `parse/zebra2` profile for a table of 157 entries (T1a.6.5.2).
#[derive(Default, Debug)]
struct StrTable {
    strings: Vec<String>,
    index: FxHashMap<Box<str>, SymId>,
}

impl StrTable {
    fn intern(&mut self, s: &str) -> SymId {
        ein_core::counters::bump(|k| k.intern += 1);
        if let Some(&id) = self.index.get(s) {
            return id;
        }
        ein_core::counters::bump(|k| k.intern_miss += 1);
        let id = SymId(self.strings.len() as u32);
        self.strings.push(s.to_string());
        self.index.insert(s.into(), id);
        id
    }

    fn get(&self, id: SymId) -> &str {
        &self.strings[id.0 as usize]
    }
}

/// The arena: nodes, their positions, their arguments, and the two string
/// tables everything indexes into.
///
/// One `Ast` holds a whole *program* — including every module an
/// `(import …)` pulls in — so structural comparison across files
/// ([`Ast::eq_nodes`], which import de-duplication needs) is a comparison of
/// interned ids rather than of strings.
#[derive(Default, Debug)]
pub struct Ast {
    nodes: Vec<Node>,
    locs: Vec<Option<Loc>>,
    args: Vec<NodeId>,
    strs: StrTable,
    files: Vec<String>,
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Strings and files ──────────────────────────────────────────

    pub fn intern(&mut self, s: &str) -> SymId {
        self.strs.intern(s)
    }

    pub fn sym(&self, id: SymId) -> &str {
        self.strs.get(id)
    }

    /// Register a source file name. `None` becomes `"<string>"` — the name
    /// `parse(text)` records when no filename is given.
    pub fn intern_file(&mut self, name: Option<&str>) -> FileId {
        let name = name.unwrap_or("<string>");
        if let Some(i) = self.files.iter().position(|f| f == name) {
            return FileId(i as u32);
        }
        self.files.push(name.to_string());
        FileId((self.files.len() - 1) as u32)
    }

    pub fn file(&self, id: FileId) -> &str {
        &self.files[id.0 as usize]
    }

    // ── Nodes ──────────────────────────────────────────────────────

    pub fn push(&mut self, node: Node, loc: Option<Loc>) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        self.locs.push(loc);
        id
    }

    pub fn node(&self, id: NodeId) -> Node {
        self.nodes[id.0 as usize]
    }

    pub fn loc(&self, id: NodeId) -> Option<Loc> {
        self.locs[id.0 as usize]
    }

    /// `(nodes, args, symbols)` — what the arenas hold. The instrument
    /// [`frontend_cost`](../../ein-infer/examples/frontend_cost.rs) reports it
    /// next to the allocation count, so "an arena grew" and "the program is
    /// bigger" stay distinguishable.
    pub fn arena_sizes(&self) -> (usize, usize, usize) {
        (self.nodes.len(), self.args.len(), self.strs.strings.len())
    }

    pub fn args(&self, span: ArgSpan) -> &[NodeId] {
        let start = span.start as usize;
        &self.args[start..start + span.len as usize]
    }

    /// Copy `ids` into the argument arena and return the span naming them.
    pub fn alloc_args(&mut self, ids: &[NodeId]) -> ArgSpan {
        if ids.is_empty() {
            return ArgSpan::EMPTY;
        }
        let start = self.args.len() as u32;
        self.args.extend_from_slice(ids);
        ArgSpan {
            start,
            len: ids.len() as u32,
        }
    }

    // ── Builders ───────────────────────────────────────────────────

    pub fn atom(&mut self, name: &str, loc: Option<Loc>) -> NodeId {
        let s = self.intern(name);
        self.push(Node::Atom(s), loc)
    }

    pub fn sform(&mut self, head: NodeId, args: &[NodeId], loc: Option<Loc>) -> NodeId {
        let span = self.alloc_args(args);
        self.push(Node::SForm { head, args: span }, loc)
    }

    /// `(name arg…)` with a **synthetic** head — the `Atom` carries no `Loc`,
    /// which is what every `_topform`-built form does in ein.py.
    pub fn sform_named(&mut self, name: &str, args: &[NodeId], loc: Option<Loc>) -> NodeId {
        let head = self.atom(name, None);
        self.sform(head, args, loc)
    }

    // ── Accessors the later passes want ────────────────────────────

    /// The name of an `Atom` node, or `None` for anything else.
    pub fn atom_name(&self, id: NodeId) -> Option<&str> {
        match self.node(id) {
            Node::Atom(s) => Some(self.sym(s)),
            _ => None,
        }
    }

    /// The `Atom` head-name of a form, or `None` when the node is not a form
    /// or its head is a `Var`/`Wildcard`.
    pub fn head_name(&self, id: NodeId) -> Option<&str> {
        match self.node(id) {
            Node::SForm { head, .. } => self.atom_name(head),
            _ => None,
        }
    }

    /// A form's arguments, or an empty slice for a non-form.
    pub fn form_args(&self, id: NodeId) -> &[NodeId] {
        match self.node(id) {
            Node::SForm { args, .. } => self.args(args),
            _ => &[],
        }
    }

    // ── Structural equality ────────────────────────────────────────

    /// Structural equality, ignoring positions — ein.py's `IRNode.__eq__`
    /// with `loc` excluded from `compare`.
    ///
    /// Import de-duplication compares two declarations that came from
    /// different files, so this must be a *shape* comparison; the `Loc` side
    /// table is what keeps it honest.
    pub fn eq_nodes(&self, a: NodeId, b: NodeId) -> bool {
        if a == b {
            return true;
        }
        match (self.node(a), self.node(b)) {
            (Node::Atom(x), Node::Atom(y)) => x == y,
            (Node::Var(x), Node::Var(y)) => x == y,
            (Node::Keyword(x), Node::Keyword(y)) => x == y,
            (Node::Wildcard, Node::Wildcard) => true,
            (Node::Str(x), Node::Str(y)) => x == y,
            (Node::Int(x), Node::Int(y)) => x == y,
            (Node::Range { low: l1, high: h1 }, Node::Range { low: l2, high: h2 }) => {
                l1 == l2 && h1 == h2
            }
            (Node::KwPair { key: k1, value: v1 }, Node::KwPair { key: k2, value: v2 }) => {
                self.eq_nodes(k1, k2) && self.eq_nodes(v1, v2)
            }
            (Node::SForm { head: h1, args: a1 }, Node::SForm { head: h2, args: a2 }) => {
                if a1.len != a2.len || !self.eq_nodes(h1, h2) {
                    return false;
                }
                (0..a1.len as usize).all(|i| {
                    let x = self.args[a1.start as usize + i];
                    let y = self.args[a2.start as usize + i];
                    self.eq_nodes(x, y)
                })
            }
            _ => false,
        }
    }

    // ── Arena bookkeeping (the parser backtracks) ──────────────────

    /// A watermark the parser restores to when an alternative fails. Nothing
    /// a failed alternative built can be referenced afterwards, so rolling the
    /// arenas back is both safe and what keeps them tight.
    pub(crate) fn mark(&self) -> (usize, usize) {
        (self.nodes.len(), self.args.len())
    }

    pub(crate) fn rollback(&mut self, mark: (usize, usize)) {
        self.nodes.truncate(mark.0);
        self.locs.truncate(mark.0);
        self.args.truncate(mark.1);
    }
}

/// `repr(node)` — the dataclass `repr` two loader messages interpolate.
///
/// `(config foo)` reports `got Atom(name='foo')`, and an ill-typed flag value
/// reports the node it got; both are text a puzzle author reads, so the
/// nesting and the one-tuple comma have to be right.
pub fn node_repr(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Atom(s) => format!("Atom(name={})", repr_str(ast.sym(s))),
        Node::Var(s) => format!("Var(name={})", repr_str(ast.sym(s))),
        Node::Keyword(s) => format!("Keyword(name={})", repr_str(ast.sym(s))),
        Node::Wildcard => "Wildcard()".to_string(),
        Node::Str(s) => format!("String(value={})", repr_str(ast.sym(s))),
        // `Int` and `Range` carry Python *integers*, so they print unquoted.
        Node::Int(s) => format!("Int(value={})", ast.sym(s)),
        Node::Range { low, high } => format!(
            "Range(low={}, high={})",
            ast.sym(low),
            high.map_or("None".to_string(), |h| ast.sym(h).to_string())
        ),
        Node::KwPair { key, value } => format!(
            "KwPair(key={}, value={})",
            node_repr(ast, key),
            node_repr(ast, value)
        ),
        Node::SForm { head, args } => {
            let args = ast.args(args);
            let mut inner: String = args
                .iter()
                .map(|&a| node_repr(ast, a))
                .collect::<Vec<_>>()
                .join(", ");
            if args.len() == 1 {
                inner.push(',');
            }
            format!("SForm(head={}, args=({inner}))", node_repr(ast, head))
        }
    }
}

/// `f"{loc}"` — the dataclass `repr` a loader message interpolates.
///
/// `None` for a **top-level** form, which is what makes ein.py's loader
/// messages end in `at None`: `_topform`, `relation_decl`, `generic_fact`,
/// `eq_fact`, … all build their `SForm` without a `loc` and only
/// `generic_list` passes one. Q-M1a.6 — a real usability bug, fixed in both
/// implementations *after* parity, because fixing it during the port would
/// break T3 and hide regressions.
pub fn loc_repr(ast: &Ast, loc: Option<Loc>) -> String {
    match loc {
        None => "None".to_string(),
        Some(l) => format!(
            "Loc(file={}, line={}, col={})",
            ein_core::pyrepr::repr_str(ast.file(l.file)),
            l.line,
            l.col
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_equality_ignores_position() {
        let mut ast = Ast::new();
        let f = ast.intern_file(Some("a.ein"));
        let here = Some(Loc {
            file: f,
            line: 1,
            col: 1,
        });
        let there = Some(Loc {
            file: f,
            line: 9,
            col: 9,
        });
        let a = ast.atom("x", here);
        let b = ast.atom("x", there);
        let c = ast.atom("y", here);
        assert!(ast.eq_nodes(a, b));
        assert!(!ast.eq_nodes(a, c));

        let fa = ast.sform_named("rule", &[a], here);
        let fb = ast.sform_named("rule", &[b], there);
        let fc = ast.sform_named("rule", &[c], here);
        assert!(ast.eq_nodes(fa, fb));
        assert!(!ast.eq_nodes(fa, fc));
    }

    #[test]
    fn a_loc_renders_as_pythons_dataclass_repr() {
        let mut ast = Ast::new();
        let f = ast.intern_file(Some("examples/zebra2.ein"));
        assert_eq!(loc_repr(&ast, None), "None");
        assert_eq!(
            loc_repr(
                &ast,
                Some(Loc {
                    file: f,
                    line: 6,
                    col: 20
                })
            ),
            "Loc(file='examples/zebra2.ein', line=6, col=20)"
        );
    }

    #[test]
    fn rollback_restores_the_arena() {
        let mut ast = Ast::new();
        let keep = ast.atom("keep", None);
        let mark = ast.mark();
        let drop = ast.atom("drop", None);
        ast.sform(drop, &[drop, drop], None);
        ast.rollback(mark);
        assert_eq!(ast.mark(), mark);
        assert_eq!(ast.atom_name(keep), Some("keep"));
    }
}
