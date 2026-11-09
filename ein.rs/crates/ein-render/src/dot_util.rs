//! The low-level DOT helpers every renderer shares.
//!
//! It exists because S1.7c.25 collapsed four hand-rolled copies of the
//! node-id scheme onto one definition, and a renderer that re-hand-rolled one
//! would re-acquire the drift that collapse removed.

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
/// arguments (the one printer's text since M1e2 S1e2.3.8, where a nested fact
/// was its `repr`).
pub fn fact_key(relation_name: &str, args: &[String]) -> String {
    format!("{relation_name}|{}", args.join(","))
}

// ── The shape legend ───────────────────────────────────────────────
//
// `docs/kernel/ir/03-ein-lang/04_dot_rendering.md` § Node-shape legend. Named
// constants rather than literals because the per-form IR renderer and the
// rule renderer must agree, and a legend that lives in two string literals
// drifts.

pub const TYPE_SHAPE: &str = "box";
pub const INSTANCE_SHAPE: &str = "oval";
pub const GROUND_SHAPE: &str = "rectangle";
pub const HYPER_SHAPE: &str = "octagon";
pub const EQUALITY_SHAPE: &str = "doublecircle";
pub const VAR_SHAPE: &str = "diamond";
pub const WILDCARD_ATTRS: &str = "shape=diamond, style=dashed";

/// The rank separation of a digraph whose diamonds and octagons stand outside
/// any cluster: a rule's overlay, and a [`Builder`](crate::builder::Builder)
/// digraph that holds a hyperedge node. M1f S1f.5.17.
///
/// An edge label makes `dot` halve the rank separation (`rank.c`,
/// `edgelabel_ranks`), and the default 0.5 in becomes 18 pt, the half-height
/// of a default node. On Ubuntu 26.04's x86-64-v3 build of Graphviz, whose
/// `libpathplan` orientation test is contracted into a fused multiply-add,
/// that puts the slanted side of a diamond or an octagon through exactly
/// collinear corners, and the spline router writes `triangulation failed`.
/// 0.55 in halves to 20 pt. Every other `ranksep` tried gave no such warning
/// on any corpus view. The default gave one on 3 to 22 views at every
/// `nodesep` from 0.20 to 0.40. The side-by-side rules draw theirs inside
/// clusters and were never refused, nor were the KB and constraint views
/// (the stage file's § Step 1). `dot_wellformed.rs` keeps that true.
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
/// Total: a [`Node::KwPair`] is labelled `:key value`, the spelling
/// `ir_dot`'s `render_note` gives a `(query …)` block's keywords. A view's
/// own filter drops the keyword pairs of the form it draws, but this function
/// recurses into nested forms, and a keyword pair below the filter used to
/// panic here (*not a value node: KwPair*, exit 101 on `render constraints`
/// and `render program`). M1e2 S1e2.3.2, the user's ruling (a) of 2026-11-03;
/// the witness is `examples/ein-bugs/kwpair-below-the-filter.ein`.
pub fn value_label(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Atom(s) => ast.sym(s).to_string(),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        Node::Wildcard => "_".to_string(),
        Node::Keyword(s) => format!(":{}", ast.sym(s)),
        // The name a string is, as the one printer prints it (M1e2
        // S1e2.3.8): `"1"` and `"x y"` quoted, `"Red"` bare as `Red`.
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
        Node::KwPair { key, value } => {
            format!("{} {}", value_label(ast, key), value_label(ast, value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// M1e2 S1e2.3.2, ruling (a): a keyword pair below a form is labelled
    /// `:key value` at every depth, and the form as written is the label.
    /// The program was the fuzz witness's first step, the one that panicked;
    /// since M1f S1f.5.5's swap no keyword pair parses inside a term or a
    /// nested fact (S1f.5.11's ruling), so the witness is refused at its first
    /// `:rule`, and a pair reaches the arm only as a form's own, which the
    /// views filter (the review stop's item 11: the arm goes with S1f.5.11's
    /// T1f.5.11.2). Held here through a fact's own `:rule` and `:using`.
    #[test]
    fn a_keyword_pair_below_a_form_is_labelled_key_value() {
        let witness =
            "(tracex (step s1 :rule from-condition :using (c10) :derives (lives-in N H1)))";
        let refused = ein_ir::parse(&mut Ast::new(), witness, None).expect_err("refused");
        assert_eq!(refused.position(), (1, 18), "{refused}");
        let src = "(lives-in N H1 :rule from-condition :using ((c10)))";
        let mut ast = Ast::new();
        let top = ein_ir::parse(&mut ast, src, None).expect("the fact parses")[0];
        let pairs: Vec<String> = ast
            .form_args(top)
            .iter()
            .filter(|&&a| matches!(ast.node(a), Node::KwPair { .. }))
            .map(|&a| value_label(&ast, a))
            .collect();
        assert_eq!(pairs[0], ":rule from-condition");
        assert_eq!(pairs.len(), 2, "{pairs:?}");
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
