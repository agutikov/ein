//! The low-level DOT helpers every renderer shares.
//!
//! `ein.py`'s `render/dot_util.py`, and the reason it exists there is the
//! reason it exists here: S1.7c.25 collapsed four hand-rolled copies of the
//! node-id scheme onto one definition, and a port that re-hand-rolled them
//! would re-acquire the divergence that collapse removed.

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
