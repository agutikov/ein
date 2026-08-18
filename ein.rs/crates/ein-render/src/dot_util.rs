//! The low-level DOT helpers every renderer shares.
//!
//! `ein.py`'s `render/dot_util.py`, and the reason it exists there is the
//! reason it exists here: S1.7c.25 collapsed four hand-rolled copies of the
//! node-id scheme onto one definition, and a port that re-hand-rolled them
//! would re-acquire the divergence that collapse removed.

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
/// Not recursive — a nested fact argument stringifies through `str(a)`, which
/// for a frozen dataclass is its `repr`. `ein-core`'s `Terms::display` is that
/// rendering, so callers pass the already-rendered arguments.
pub fn fact_key(relation_name: &str, args: &[String]) -> String {
    format!("{relation_name}|{}", args.join(","))
}

// ── The shape legend ───────────────────────────────────────────────
//
// `docs/kernel/ir/03-ein-lang/04_dot_rendering.md` § Node-shape legend. Named
// constants rather than literals for the same reason ein.py names them: the
// per-form IR renderer and the rule renderer must agree, and a legend that
// lives in two string literals drifts.

pub const TYPE_SHAPE: &str = "box";
pub const INSTANCE_SHAPE: &str = "oval";
pub const GROUND_SHAPE: &str = "rectangle";
pub const HYPER_SHAPE: &str = "octagon";
pub const EQUALITY_SHAPE: &str = "doublecircle";
pub const VAR_SHAPE: &str = "diamond";
pub const WILDCARD_ATTRS: &str = "shape=diamond, style=dashed";

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
/// Shared by the LR-family emitters (`render/slice`, `render/lattice_dag`,
/// `render/constraints`). The other preambles — `kb/render`'s interleaved
/// `fdp` comment, the derivation DAG's and the rule renderer's bespoke
/// headers, the `_Builder`'s inline `{` — diverge too much to route here
/// byte-identically, and ein.py does not try either.
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
/// Panics on a [`Node::KwPair`], as ein.py's `TypeError` does: a keyword pair
/// is not a value, and every caller filters them out before getting here.
pub fn value_label(ast: &Ast, id: NodeId) -> String {
    match ast.node(id) {
        Node::Atom(s) => ast.sym(s).to_string(),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        Node::Wildcard => "_".to_string(),
        Node::Keyword(s) => format!(":{}", ast.sym(s)),
        Node::Str(s) => ast.sym(s).to_string(),
        Node::Int(s) => ast.sym(s).to_string(),
        Node::Range { low, high } => {
            let high = high.map_or_else(|| "*".to_string(), |h| ast.sym(h).to_string());
            format!("{}..{}", ast.sym(low), high)
        }
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
        Node::KwPair { .. } => panic!("not a value node: KwPair"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_node_id_is_the_first_ten_hex_digits_of_the_md5() {
        // The three ids in `ein.py/tests/golden/dot/kb_provenance_dag.dot`.
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
