//! The markdown pieces a trace's steps are rendered with: a step's premises
//! line and its inline `dot` block — the region trace's
//! ([`super::region`]).
//!
//! The walk's trace renderer — one narrative per solve, its reductios and its
//! closing lattice DAG — went with the walk at M1e2 S1e2.1c.5 (2026-11-03).

use super::ast::TraceStep;

pub(crate) fn dot_block(dot: Option<&String>) -> Vec<String> {
    match dot.filter(|d| !d.is_empty()) {
        None => Vec::new(),
        Some(d) => vec![
            "```dot".to_string(),
            d.clone(),
            "```".to_string(),
            String::new(),
        ],
    }
}

/// A natural-language text as markdown, for the trace's paragraphs, quotes
/// and list items: each line break it holds, `\n` and `\r` alike, a **hard
/// break** (two spaces before it), and each line after it led by `lead`, its
/// block's own marker (`"> "` in a quote, the item's indent in a list), so the
/// text keeps its lines where the trace is rendered. Until M1f S1f.5.28 the
/// break was written as it stands, which markdown reads as a soft break,
/// joining the text's lines into one (its item 5, Q-M1f.109; the read-out's
/// own rows break at their column, S1f.5.16's Q-M1f.19). A `\r\n` pair is one
/// break, not two with an empty line between them, which markdown would read
/// as a new paragraph. A text with no break is itself.
pub(crate) fn md_text(text: &str, lead: &str) -> String {
    if !text.contains(['\n', '\r']) {
        return text.to_string();
    }
    text.replace("\r\n", "\n")
        .split(['\n', '\r'])
        .collect::<Vec<_>>()
        .join(&format!("  \n{lead}"))
}

pub(crate) fn premises_line(step: &TraceStep) -> String {
    let labels = step.premise_labels();
    if !labels.is_empty() {
        let quoted: Vec<String> = labels.iter().map(|p| format!("`{p}`")).collect();
        let mut base = format!("Premises: {}", quoted.join(", "));
        if !step.sources.is_empty() {
            base.push_str(&format!(" — from {}", sources(step)));
        }
        return base;
    }
    if !step.sources.is_empty() {
        // No derived premise — the source *is* what was given.
        return format!("Premises: from {}", sources(step));
    }
    "Premises: —".to_string()
}

/// The premises' `:source`s, each as [`md_text`] writes a text in a paragraph.
fn sources(step: &TraceStep) -> String {
    step.sources
        .iter()
        .map(|t| md_text(t, ""))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::md_text;

    #[test]
    fn each_break_is_one_hard_break() {
        assert_eq!(md_text("a b", "> "), "a b");
        assert_eq!(md_text("a\nb", "> "), "a  \n> b");
        assert_eq!(md_text("a\rb", ""), "a  \nb");
        // A CRLF pair is one break: no empty line, which would end the block.
        assert_eq!(md_text("a\r\nb", "> "), "a  \n> b");
        assert_eq!(md_text("a\n\nb", ""), "a  \n  \nb");
    }
}
