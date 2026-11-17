//! The low-level DOT helpers every renderer shares.
//!
//! The node-id scheme has one definition here, so that no two renderers can
//! drift apart in how they name a node: a renderer that hand-rolled its own
//! would.

use ein_core::{FactId, Tag, Terms, Value};
use ein_ir::{Ast, Node, NodeId};
use md5::{Digest, Md5};

/// Escape DOT-special characters — backslash and double-quote — *without* the
/// surrounding quotes.
pub fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Quote a DOT identifier or label, escaping internal specials.
pub fn quote(s: &str) -> String {
    format!("\"{}\"", esc(s))
}

/// A content-addressed node id: `prefix` + `md5(seed)[:10]` in hex.
///
/// The caller owns `seed`: the flat [`fact_key`] and `render/slice`'s
/// *recursive* key are deliberately not merged — only this hash-and-prefix
/// tail is shared. `quoted` wraps the result in DOT quotes, which some
/// emitters do and others do not.
pub fn hashed_id(prefix: &str, seed: &str, quoted: bool) -> String {
    let digest = Md5::digest(seed.as_bytes());
    let mut id = String::with_capacity(prefix.len() + 10);
    id.push_str(prefix);
    for byte in &digest[..5] {
        id.push_str(&format!("{byte:02x}"));
    }
    if quoted { quote(&id) } else { id }
}

/// The flat content key behind a fact's node id: `rel|arg,arg`.
///
/// Not recursive — a nested fact argument is its s-expression, which is what
/// `ein-core`'s `Terms::display` renders, so callers pass the already-rendered
/// arguments, the one printer's text.
pub fn fact_key(relation_name: &str, args: &[String]) -> String {
    format!("{relation_name}|{}", args.join(","))
}

// ── The shape legend ───────────────────────────────────────────────
//
// `docs/ein/user/04-reference/12-rendering.md` § Node-shape legend. Named
// constants rather than literals because the per-form IR renderer and the
// rule renderer must agree, and a legend that lives in two string literals
// drifts.

pub const TYPE_SHAPE: &str = "box";
pub const INSTANCE_SHAPE: &str = "oval";
pub const GROUND_SHAPE: &str = "rectangle";
pub const HYPER_SHAPE: &str = "octagon";
pub const EQUALITY_SHAPE: &str = "doublecircle";
pub const VAR_SHAPE: &str = "diamond";

/// The rank separation of a digraph whose diamonds and octagons stand outside
/// any cluster: a rule's overlay, and a [`Builder`](crate::builder::Builder)
/// digraph that holds a hyperedge node.
///
/// An edge label makes `dot` halve the rank separation (`rank.c`,
/// `edgelabel_ranks`), and the default 0.5 in becomes 18 pt, the half-height
/// of a default node. On Ubuntu 26.04's x86-64-v3 build of Graphviz, whose
/// `libpathplan` orientation test is contracted into a fused multiply-add,
/// that puts the slanted side of a diamond or an octagon through exactly
/// collinear corners, and the spline router writes `triangulation failed`.
/// 0.55 in halves to 20 pt, and no corpus view gives the warning at it,
/// where the default gives it on several at every `nodesep` from 0.20 to
/// 0.40. The side-by-side rules draw theirs inside clusters and are not
/// refused, nor are the KB and constraint views. `dot_wellformed.rs` keeps
/// that true.
pub const POLYGON_RANKSEP: &str = "ranksep=0.55";

/// A quoted DOT label whose non-empty parts are joined by the two-character
/// `\n` line break, each escaped.
///
/// Empty parts are dropped, not rendered as blank lines — which is what lets
/// `render/slice` pass a `:why` that may be the empty string.
pub fn multiline(parts: &[&str]) -> String {
    let body: Vec<String> = parts
        .iter()
        .filter(|p| !p.is_empty())
        .map(|p| esc(p))
        .collect();
    format!("\"{}\"", body.join("\\n"))
}

/// The opening lines of a `digraph`, as the seed of the caller's line list.
///
/// Shared by the LR-family emitters (`slice`, `lattice_dag`, `constraints`).
/// The other preambles — `kb_dot`'s interleaved `fdp` comment, the derivation
/// DAG's and the rule renderer's bespoke headers, the `Builder`'s inline `{` —
/// differ too much to route here byte-identically.
pub fn digraph_open(name: &str, rankdir: Option<&str>, node_defaults: Option<&str>) -> Vec<String> {
    let mut out = vec![format!("digraph {name} {{")];
    if let Some(r) = rankdir {
        out.push(format!("  rankdir={r};"));
    }
    if let Some(n) = node_defaults {
        out.push(format!("  node [{n}];"));
    }
    out
}

/// A readable `rel(a, b, …)` label for a fact, recursing into nested fact
/// arguments (the Q40 relational-node idiom).
///
/// A nullary fact is its bare relation name — `""` inner, so no parentheses.
pub fn fact_label(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.fact(f);
    fact_label_parts(terms, terms.sym(rel), args)
}

/// [`fact_label`] with the relation and arguments given separately, for the
/// callers that hold a row rather than an id.
pub fn fact_label_parts(terms: &Terms, relation_name: &str, args: &[Value]) -> String {
    let parts: Vec<String> = args
        .iter()
        .map(|a| match a.tag() {
            Tag::Fact => fact_label(terms, a.as_fact().expect("tagged Fact")),
            _ => terms.display(*a),
        })
        .collect();
    let inner = parts.join(", ");
    if inner.is_empty() {
        relation_name.to_string()
    } else {
        format!("{relation_name}({inner})")
    }
}

/// A human-readable single-line label for an IR *value* node — what an edge
/// label or a constraint operand shows.
///
/// A value holds no [`Node::KwPair`] (`ein.y`'s `FactArg`, `Arg` and `Term`):
/// a keyword pair is a form's own clause, which each view reads or filters
/// before it labels the form's arguments, and no keyword pair parses inside a
/// term or a nested fact, so the arm for one is unreachable and names the
/// grammar (Q-M1f.14). A pair below a form,
/// `examples/broken/kwpair-below-the-filter.ein`, is refused at the parse.
pub fn value_label(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Atom(s) => ast.sym(s).to_string(),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        Node::Keyword(s) => format!(":{}", ast.sym(s)),
        // A string is natural-language text, which reaches this label only
        // as a keyword's value, `:goal-text "…"` in a query's note. It is
        // labelled as the one printer prints a name: quoted, and bare where
        // it lexes as one name.
        Node::Str(s) => ein_core::text::symbol(ast.sym(s)).into_owned(),
        Node::SForm { head, args } => {
            let inner: Vec<String> = ast
                .args(args)
                .iter()
                .map(|a| value_label(ast, *a))
                .collect();
            let head = value_label(ast, head);
            if inner.is_empty() {
                format!("({head})")
            } else {
                format!("({head} {})", inner.join(" "))
            }
        }
        Node::KwPair { .. } => {
            unreachable!("a value holds no keyword pair (ein.y's FactArg, Arg and Term)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **No keyword pair reaches a value's label.** The fuzz witness's first
    /// step is refused at the parse, at its first `:rule`: no keyword pair
    /// parses inside a term or a nested fact. A pair is a form's own clause,
    /// and each view reads
    /// the fact's arguments without it: a fact carrying its own `:rule` and
    /// `:using` is drawn by both views that drew the witness, its arguments
    /// labelled and its pairs read as provenance.
    #[test]
    fn no_keyword_pair_reaches_a_values_label() {
        let witness =
            "(tracex (step s1 :rule from-condition :using (c10) :derives (lives-in N H1)))";
        let refused = ein_ir::parse(&mut Ast::new(), witness, None).expect_err("refused");
        assert_eq!(refused.position(), (1, 18), "{refused}");
        let src = "(lives-in (house N) H1 :rule from-condition :using ((c10)))";
        let mut ast = Ast::new();
        let forms = ein_ir::parse(&mut ast, src, None).expect("the fact parses");
        let program = crate::ir_to_dot(&ast, &forms, crate::DotOpts::default());
        assert!(program.contains("(house N)"), "{program}");
        let constraints = crate::render_constraints(&ast, &forms, "witness");
        assert!(constraints.starts_with("digraph"), "{constraints}");
        let labels: Vec<String> = ast
            .form_args(forms[0])
            .iter()
            .filter(|&&a| !matches!(ast.node(a), Node::KwPair { .. }))
            .map(|&a| value_label(&ast, a))
            .collect();
        assert_eq!(labels, ["(house N)", "H1"]);
    }

    #[test]
    fn the_node_id_is_the_first_ten_hex_digits_of_the_md5() {
        // The three ids in `ein-render/tests/golden/dot/kb_provenance_dag.dot`.
        assert_eq!(hashed_id("f_", "q|a,c", false), "f_0b8a036bc4");
        assert_eq!(hashed_id("f_", "p|a,b", false), "f_69510aab53");
        assert_eq!(hashed_id("f_", "p|b,c", false), "f_b34b2c73fa");
        assert_eq!(hashed_id("n_", "", true), "\"n_d41d8cd98f\"");
    }

    #[test]
    fn escaping_covers_the_backslash_as_well_as_the_quote() {
        assert_eq!(esc(r#"a\b"c"#), r#"a\\b\"c"#);
        assert_eq!(quote("plain"), "\"plain\"");
    }

    #[test]
    fn a_fact_key_is_flat() {
        assert_eq!(
            fact_key("co-located", &["Norwegian".into(), "House-1".into()]),
            "co-located|Norwegian,House-1"
        );
        assert_eq!(fact_key("q", &[]), "q|");
    }
}
