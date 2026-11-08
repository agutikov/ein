//! **Seeded mutants of the corpus** — S1f.5.5's census generator (`c_ein
//! mutate`, 2026-11-08), kept here so that the tests and the measurements that
//! read it draw the same inputs (M1f S1f.5.5, T1f.5.5.3).
//!
//! Every `.ein` under `examples/`, `stdlib/`, `tests/` and `corpus/` that is
//! under 6 000 bytes is a source; a mutant is one source with one to three
//! edits, each deleting a character, inserting a fragment, or replacing a
//! character with one. The fragments are the census's: the characters and
//! words a token can start or end with, a keyword pair, `()` and `(and)`. The
//! stream is xorshift64 from a fixed state, so `(n, seed)` names one set of
//! mutants on every machine, as long as the corpus is the same.

use std::path::PathBuf;

use crate::repo_root;

/// What an edit may insert, or replace a character with: the census's
/// alphabet.
const FRAGMENTS: &[&str] = &[
    "(", ")", "_", "?", "?x", ":", "\"", "\"a\"", " ", "\n", ";", "#|", "|#", "1", "a", "-", "=",
    "not", "rule", ":why", ":match", "$", "*", ".", "..", "\\", "?_", "01", ":k v", "(and)", "()",
];

/// `text` with every `:priority` clause taken out: the keyword and the run of
/// digits after it, a `-` allowed. What a corpus file reads as once S1f.5.15
/// has deleted the keyword, which the generated parser's grammar does not have
/// (the user, 2026-11-08); a comment or a string that writes one loses it too,
/// which reads the same to both parsers.
pub fn without_priority(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(":priority") {
        let after = &rest[at + ":priority".len()..];
        let value = after.trim_start();
        let digits = value.strip_prefix('-').unwrap_or(value);
        let n = digits.len()
            - digits
                .trim_start_matches(|c: char| c.is_ascii_digit())
                .len();
        let ends_a_word = after.len() > value.len() && n > 0;
        out.push_str(&rest[..at]);
        if ends_a_word {
            rest = &digits[n..];
        } else {
            out.push_str(":priority");
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// The census's source files: every corpus `.ein` under 6 000 bytes, sorted
/// by path, with its text.
pub fn sources() -> Vec<(PathBuf, String)> {
    let root = repo_root();
    let mut files = Vec::new();
    for dir in ["examples", "stdlib", "tests", "corpus"] {
        files.extend(crate::ein_files_under(&root.join(dir)));
    }
    files.sort();
    files
        .into_iter()
        .filter_map(|p| {
            let text = std::fs::read_to_string(&p).ok()?;
            (text.len() < 6000).then_some((p, text))
        })
        .collect()
}

/// `n` mutants of `sources`, drawn from the stream `seed` names (zero is
/// taken as the census's own state). Each is its source's index and its text.
pub fn mutants(sources: &[(PathBuf, String)], n: usize, seed: u64) -> Vec<(usize, String)> {
    let mut x = if seed == 0 {
        0x9E37_79B9_7F4A_7C15
    } else {
        seed
    };
    let mut r = move |m: usize| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        (x % (m as u64)) as usize
    };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let which = r(sources.len());
        let mut t = sources[which].1.clone();
        for _ in 0..(1 + r(3)) {
            let idx: Vec<usize> = t.char_indices().map(|c| c.0).collect();
            if idx.is_empty() {
                break;
            }
            let i = idx[r(idx.len())];
            let c = t[i..].chars().next().expect("a character at a char index");
            match r(3) {
                0 => t.replace_range(i..i + c.len_utf8(), ""),
                1 => t.insert_str(i, FRAGMENTS[r(FRAGMENTS.len())]),
                _ => t.replace_range(i..i + c.len_utf8(), FRAGMENTS[r(FRAGMENTS.len())]),
            }
        }
        out.push((which, t));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::without_priority;

    #[test]
    fn a_priority_clause_is_taken_out_and_nothing_else() {
        assert_eq!(
            without_priority("(rule r () :match (p) :assert (q) :priority 400)"),
            "(rule r () :match (p) :assert (q) )"
        );
        assert_eq!(
            without_priority(":priority -5\n:priority x"),
            "\n:priority x"
        );
        assert_eq!(without_priority(":priorityless 3"), ":priorityless 3");
    }
}
