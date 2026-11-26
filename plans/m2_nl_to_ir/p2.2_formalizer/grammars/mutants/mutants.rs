//! S2.2.3's mutants of the whole grammar's hand-written rules: each a change
//! to one rule of `einaf/grammars/ein.gbnf`'s text, and which of the three
//! checks with no model sees it: the corpus (a file the parser and the
//! mutant part ways on), the parser's banked table of refused shapes (a row
//! the mutant takes), and texts sampled from the mutant (one the parser
//! refuses, 2 000 from the seed test 2 uses).
//!
//! Run by copying this file to `einaf/tests/zz_mutants.rs` and
//! `cargo test --manifest-path einaf/Cargo.toml --profile gate --test zz_mutants -- --nocapture`;
//! the file is not left in the package.

use einaf::grammar::gbnf::{Compiled, Rng};
use einaf::kernel;

/// Each mutant: its name, the rule's text, and what replaces it.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("a name may be a word", "tok-name ::= ", "tok-name ::= tok-name-rest-plus"),
    ("`?_` is a variable", "tok-var ::= ", "tok-var ::= \"?\" tok-var-rest-plus"),
    ("a string may be glued to `#`", "gap-text ::= ", "gap-text ::= gap-word"),
    ("a line comment need not end its line", "comment-line-ended ::= ", "comment-line-ended ::= comment-line"),
    ("two words need no separator", "gap-word ::= ", "gap-word ::= gap"),
];

fn mutate(text: &str, rule: &str, with: &str) -> String {
    let start = text.find(&format!("\n{rule}")).expect("the rule") + 1;
    let end = text[start..].find("\n\n").map_or(text.len(), |e| start + e);
    let extra = "\ntok-name-rest-plus ::= [!$-'*+\\--9<->@-\\[\\]-{}~]+\ntok-var-rest-plus ::= [!$-'*+\\--9<->@-\\[\\]-{}~]+";
    format!("{}{}{}{}", &text[..start], with, &text[end..], extra)
}

fn table() -> Vec<String> {
    let path = ein_corpus::repo_root().join("ein.rs/crates/ein-ir/tests/golden/grammar_table.txt");
    let mut rows = Vec::new();
    let (mut prog, mut inside) = (String::new(), false);
    for line in std::fs::read_to_string(path).unwrap().lines() {
        if line.starts_with("## ") {
            prog.clear();
            inside = true;
        } else if inside {
            if let Some(l) = line.strip_prefix("    ") {
                prog.push_str(l);
                prog.push('\n');
            } else {
                inside = false;
                rows.push(prog.clone());
            }
        }
    }
    rows
}

#[test]
fn which_check_sees_each_mutant() {
    let whole = einaf::grammar::whole_text();
    let files: Vec<(String, String, bool)> = ein_corpus::corpus_files()
        .into_iter()
        .filter_map(|p| {
            let t = std::fs::read_to_string(&p).ok()?;
            let ok = kernel::parse(&t, "f").forms.is_some();
            Some((p.display().to_string(), t, ok))
        })
        .collect();
    let rows = table();
    for (name, rule, with) in MUTANTS {
        let m = Compiled::parse(&mutate(&whole, rule, with)).expect("a mutant reads");
        let corpus = files
            .iter()
            .filter(|(_, t, ok)| m.accepts("program", t).unwrap().accepted() != *ok)
            .count();
        let taken = rows
            .iter()
            .filter(|r| m.accepts("program", r).unwrap().accepted())
            .count();
        let mut rng = Rng::new(20_261_126);
        let mut refused = 0;
        for i in 0..2_000 {
            let root = if i % 2 == 0 { "root" } else { "program" };
            let t = m.sample(root, &mut rng, 24, 1_200, einaf::grammar::FAVOURED).unwrap();
            if kernel::parse(&t, "s").forms.is_none() {
                refused += 1;
            }
        }
        println!("{name}: the corpus {corpus} files part ways; the table {taken} rows taken; the samples {refused} of 2000 refused");
    }
}
