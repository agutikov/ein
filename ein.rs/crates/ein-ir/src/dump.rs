//! The canonical IR printer — `ir/dump.py`, byte for byte.
//!
//! Three consumers make this a T3 surface rather than a convenience:
//! `ein-ir/tests/golden/from_ein_py/*.golden` (which *is* `dump_canonical(parse(f))`),
//! `--dump-states`, and the markdown trace. The width-driven line breaking is
//! where an off-by-one hides, so the rule is transcribed rather than
//! re-derived:
//!
//! > render compact; if `indent * len(INDENT) + len(compact) > width` **and**
//! > the node is a form **with** arguments, put the head on its own line, one
//! > argument per line indented one deeper, and append the `)` to the last.
//!
//! `len` there is Python's — a **character** count, not bytes. `zebra2.golden`
//! contains `⟹`, so the difference is not hypothetical.
//!
//! Note what does *not* break: a [`Node::KwPair`] is not a form, so it always
//! renders compact however long its value is. That is why a 100-character
//! `:why` template stays on one line.

use crate::ast::{Ast, Node, NodeId};

/// `_INDENT` — two spaces.
const INDENT: &str = "  ";
/// `_DEFAULT_WIDTH`.
pub const DEFAULT_WIDTH: usize = 80;

/// A `@`-prefixed head is synthetic (`@params`, `@empty`) and is printed
/// without a head at all.
fn is_headless(ast: &Ast, node: NodeId) -> bool {
    match ast.node(node) {
        Node::SForm { head, .. } => ast.atom_name(head).is_some_and(|n| n.starts_with('@')),
        _ => false,
    }
}

fn atom_text(ast: &Ast, node: NodeId) -> String {
    match ast.node(node) {
        Node::Atom(s) => ast.sym(s).to_string(),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        _ => "_".to_string(),
    }
}

/// `strings.escape_string_literal` — the full set (`\` `"` `\n` `\t` `\r`)
/// applied **in that order**, so a literal backslash-then-n survives as
/// `\\n` rather than collapsing into a newline escape.
///
/// The parser's unescape is the inverse, so `parse(escape(s))` recovers `s`.
pub fn escape_string_literal(s: &str) -> String {
    let body = s
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r");
    format!("\"{body}\"")
}

/// Single-line rendering of any node.
pub fn dump_compact(ast: &Ast, node: NodeId) -> String {
    let mut out = String::new();
    compact_into(ast, node, &mut out);
    out
}

fn compact_into(ast: &Ast, node: NodeId, out: &mut String) {
    match ast.node(node) {
        Node::Atom(_) | Node::Var(_) | Node::Wildcard => out.push_str(&atom_text(ast, node)),
        Node::Keyword(s) => {
            out.push(':');
            out.push_str(ast.sym(s));
        }
        Node::Str(s) => out.push_str(&escape_string_literal(ast.sym(s))),
        Node::Int(s) => out.push_str(ast.sym(s)),
        Node::Range { low, high } => {
            out.push_str(ast.sym(low));
            out.push_str("..");
            match high {
                Some(h) => out.push_str(ast.sym(h)),
                None => out.push('*'),
            }
        }
        Node::KwPair { key, value } => {
            compact_into(ast, key, out);
            out.push(' ');
            compact_into(ast, value, out);
        }
        Node::SForm { head, args } => {
            let args = ast.args(args).to_vec();
            let headless = is_headless(ast, node);
            out.push('(');
            if !headless {
                out.push_str(&atom_text(ast, head));
                if !args.is_empty() {
                    out.push(' ');
                }
            }
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                compact_into(ast, *a, out);
            }
            out.push(')');
        }
    }
}

/// Multi-line rendering: compact when it fits the column budget, otherwise one
/// argument per line.
pub fn dump_pretty(ast: &Ast, node: NodeId, indent: usize, width: usize) -> String {
    let compact = dump_compact(ast, node);
    let cur = indent * INDENT.len();
    let Node::SForm { head, args } = ast.node(node) else {
        return compact;
    };
    if cur + compact.chars().count() <= width {
        return compact;
    }
    if args.is_empty() {
        return compact; // `(head)` always fits, or has no breaking room
    }
    let head = if is_headless(ast, node) {
        String::new()
    } else {
        atom_text(ast, head)
    };
    let open = format!("({head}");
    let pad = INDENT.repeat(indent + 1);
    let body: Vec<String> = ast
        .args(args)
        .to_vec()
        .iter()
        .map(|a| format!("{pad}{}", dump_pretty(ast, *a, indent + 1, width)))
        .collect();
    format!("{open}\n{})", body.join("\n"))
}

/// `dump_canonical` over an iterable: forms separated by a blank line, with a
/// trailing newline when there is anything at all.
pub fn dump_canonical(ast: &Ast, nodes: &[NodeId]) -> String {
    dump_canonical_width(ast, nodes, DEFAULT_WIDTH)
}

pub fn dump_canonical_width(ast: &Ast, nodes: &[NodeId], width: usize) -> String {
    if nodes.is_empty() {
        return String::new();
    }
    let chunks: Vec<String> = nodes
        .iter()
        .map(|n| dump_pretty(ast, *n, 0, width))
        .collect();
    format!("{}\n", chunks.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    fn round(src: &str) -> String {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, src, None).expect("parses");
        dump_canonical(&ast, &forms)
    }

    #[test]
    fn escaping_is_applied_in_python_order() {
        assert_eq!(escape_string_literal("a\nb"), "\"a\\nb\"");
        assert_eq!(escape_string_literal("a\\nb"), "\"a\\\\nb\"");
        assert_eq!(escape_string_literal("q\"q"), "\"q\\\"q\"");
        assert_eq!(escape_string_literal("t\tr\r"), "\"t\\tr\\r\"");
    }

    #[test]
    fn synthetic_heads_print_without_one() {
        assert_eq!(
            round("(rule r () :match X :assert Y)"),
            "(rule r () :match X :assert Y)\n"
        );
        assert_eq!(round("(x ())"), "(x ())\n");
        assert_eq!(
            round("(rule r (?a ?b) :match X)"),
            "(rule r (?a ?b) :match X)\n"
        );
    }

    #[test]
    fn forms_are_blank_line_separated_with_a_trailing_newline() {
        assert_eq!(round("(a)(b)"), "(a)\n\n(b)\n");
        assert_eq!(round(""), "");
        assert_eq!(round("   ; nothing\n"), "");
    }

    #[test]
    fn a_form_breaks_only_when_it_is_a_form_with_arguments() {
        let mut ast = Ast::new();
        let src = "(relation very-long-relation-name-here A B :why \"a template long enough to overflow\")";
        let forms = parse(&mut ast, src, None).expect("parses");
        let out = dump_pretty(&ast, forms[0], 0, DEFAULT_WIDTH);
        assert!(
            out.starts_with("(relation\n  very-long-relation-name-here\n"),
            "{out}"
        );
        // The kw-pair is not a form, so it stays on one line however long.
        assert!(
            out.contains("  :why \"a template long enough to overflow\")"),
            "{out}"
        );
    }

    #[test]
    fn the_width_budget_counts_characters_not_bytes() {
        // 20 `⟹` are 20 characters and 60 bytes; at width 40 the form fits on
        // one line only if the budget is counted the way Python counts it.
        let arrows = "⟹".repeat(20);
        let src = format!("(a \"{arrows}\")");
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &src, None).expect("parses");
        assert_eq!(
            dump_pretty(&ast, forms[0], 0, 40),
            format!("(a \"{arrows}\")")
        );
        assert!(dump_pretty(&ast, forms[0], 0, 20).starts_with("(a\n"));
    }
}
