//! **Every `{?ref}` a corpus or stdlib `:why` writes names something its form
//! can fill** — M1f S1f.5.14's T1f.5.14.5, the first of the stage's two
//! checks ([`09_natural_language.md` § 7](../../../../docs/kernel/ir/03-ein-lang/09_natural_language.md#7-what-holds-this-page)).
//!
//! [`ein_core::render_why`] leaves a reference nothing binds as written, so a
//! typo reaches a user's output as `{?huse}`; the effect is held where a
//! template is filled, by `ein-cli/tests/answer_round_trip.rs`'
//! `no_placeholder_reaches_an_output` and `corpus_cli.rs`' trace check, and
//! this holds the cause. Per form:
//!
//! - a **relation**'s references are positional, `{?1}` to `{?n}` for an
//!   `n`-place signature;
//! - a **rule**'s and an **hrule**'s name a variable the firing binds: a
//!   parameter, or a variable of a positive premise of its `:match` once the
//!   macros are expanded — not one only an `(absent …)` reads, which binds
//!   nothing, and in a top-level `(or …)` one every disjunct binds;
//! - a **macro**'s name a parameter.
//!
//! And **no `{x}` is a reference with its `?` missing**: a bare `{x}` is a
//! literal (`01_grammar.md` § `:why`), so `{1}` in a relation's `:why`, or
//! `{a}` in a rule's that binds `?a`, prints as written where a reference was
//! meant. When it was written (2026-11-10, at `5e4ecb93`) it found 66 over
//! the 893 templates it read: 58 such literals in 45 relation `:why`s over 13
//! files, `"{1} is a {2}"` for `"{?1} is a {?2}"`, and the rule
//! `knows-is-mutual`'s `"{?1} knows {?2}, …"` in
//! `examples/ein-bugs/wrong-answers/uncovered.ein` and its `-knows` twin,
//! which binds `?a` and `?b`: every one printed as written. The stage file's
//! § T1f.5.14.5 record names them; each was fixed in the commit that banked
//! this, and it is a regression guard since. The stage file expected 0 of 544;
//! a probe that comes back with findings is a result, as one that comes back
//! clean would have been.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_corpus::repo_root;
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, Node, NodeId, parse};

/// Every `.ein` the corpus and the stdlib hold.
fn files() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ["stdlib", "examples", "tests", "corpus"] {
        walk(&root.join(dir), &mut out);
    }
    out.sort();
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "ein") {
            out.push(p);
        }
    }
}

/// `:key`'s value among a form's arguments.
fn kw(ast: &Ast, form: NodeId, key: &str) -> Option<NodeId> {
    ast.form_args(form).iter().find_map(|&a| match ast.node(a) {
        Node::KwPair { key: k, value }
            if matches!(ast.node(k), Node::Keyword(s) if ast.sym(s) == key.trim_start_matches(':')) =>
        {
            Some(value)
        }
        _ => None,
    })
}

/// The variables `node` binds as a premise: every variable outside an
/// `(absent …)`; a top-level `(or …)`'s, those every disjunct binds.
fn binds(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::Var(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { .. } => match ast.head_name(node) {
            Some("absent") => {}
            Some("or") => {
                let mut each = ast.form_args(node).iter().map(|&d| {
                    let mut b = BTreeSet::new();
                    binds(ast, d, &mut b);
                    b
                });
                if let Some(first) = each.next() {
                    out.extend(each.fold(first, |acc, b| &acc & &b));
                }
            }
            _ => {
                if let Node::Var(s) = ast.node(match ast.node(node) {
                    Node::SForm { head, .. } => head,
                    _ => unreachable!(),
                }) {
                    out.insert(ast.sym(s).to_string());
                }
                for &a in ast.form_args(node) {
                    binds(ast, a, out);
                }
            }
        },
        _ => {}
    }
}

/// `{?ref}` and `{x}` in `text`: (references, literals that look like one).
fn references(text: &str) -> (Vec<String>, Vec<String>) {
    let (mut refs, mut bare) = (Vec::new(), Vec::new());
    let b = text.as_bytes();
    let mut i = 0;
    while i < b.len() {
        // `\u{…}` is an escape the parser has read, not a reference.
        let escape = i > 1 && b[i - 1] == b'u' && b[i - 2] == b'\\';
        if b[i] == b'{'
            && !escape
            && let Some(end) = text[i..].find('}')
        {
            let inner = &text[i + 1..i + end];
            if let Some(name) = inner.strip_prefix('?') {
                refs.push(name.to_string());
            } else if !inner.is_empty() && inner.chars().all(ein_core::text::is_name_char) {
                bare.push(inner.to_string());
            }
            i += end + 1;
            continue;
        }
        i += 1;
    }
    (refs, bare)
}

/// What one form's `:why` cannot fill, as findings.
fn check_form(ast: &Ast, form: NodeId, file: &str, out: &mut Vec<String>) {
    let Some(head) = ast.head_name(form) else {
        return;
    };
    if !matches!(head, "relation" | "rule" | "hrule" | "macro") {
        return;
    }
    let args = ast.form_args(form);
    let name = args
        .first()
        .and_then(|&n| ast.atom_name(n))
        .unwrap_or("?")
        .to_string();
    let Some(why) = kw(ast, form, ":why") else {
        return;
    };
    let Node::Str(s) = ast.node(why) else { return };
    let text = ast.sym(s).to_string();
    let (refs, bare) = references(&text);
    let at = |what: String| format!("{file}: ({head} {name}) {what} in {text:?}");
    match head {
        "relation" => {
            let arity = args
                .iter()
                .skip(1)
                .take_while(|&&a| !matches!(ast.node(a), Node::KwPair { .. }))
                .count();
            for r in &refs {
                if !r
                    .parse::<usize>()
                    .is_ok_and(|n| n >= 1 && (arity == 0 || n <= arity))
                {
                    out.push(at(format!("{{?{r}}} is no position of its {arity}")));
                }
            }
            for b in bare
                .iter()
                .filter(|b| b.chars().all(|c| c.is_ascii_digit()))
            {
                out.push(at(format!("{{{b}}} is {{?{b}}} with its `?` missing")));
            }
        }
        _ => {
            let mut bound = BTreeSet::new();
            if let Some(&params) = args.get(1) {
                for &p in ast.form_args(params) {
                    if let Node::Var(v) = ast.node(p) {
                        bound.insert(ast.sym(v).to_string());
                    }
                }
            }
            if head != "macro"
                && let Some(m) = kw(ast, form, ":match")
            {
                binds(ast, m, &mut bound);
            }
            for r in &refs {
                if !bound.contains(r) {
                    out.push(at(format!("{{?{r}}} names nothing it binds")));
                }
            }
            for b in bare.iter().filter(|b| bound.contains(*b)) {
                out.push(at(format!("{{{b}}} is {{?{b}}} with its `?` missing")));
            }
        }
    }
}

#[test]
fn every_template_reference_names_what_its_form_fills() {
    let root = repo_root();
    let mut findings = Vec::new();
    let (mut files_read, mut templates) = (0usize, 0usize);
    for path in files() {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let mut ast = Ast::new();
        // A fixture refused at the parse has no form the loader reads.
        let Ok(forms) = parse(&mut ast, &text, Some(&rel)) else {
            continue;
        };
        files_read += 1;
        // Expand the program's own macros and those its imports bring, as the
        // loader does, so a `forall`'s variables read as the `absent` they are.
        let resolved = Resolver::new()
            .resolve_imports(&mut ast, &forms, path.parent())
            .unwrap_or_else(|_| forms.clone());
        let expanded = collect_macros(&ast, &resolved)
            .ok()
            .and_then(|m| expand_rule_clauses(&mut ast, &forms, &m).ok())
            .unwrap_or_else(|| forms.clone());
        for &f in &expanded {
            if kw(&ast, f, ":why").is_some() {
                templates += 1;
            }
            check_form(&ast, f, &rel, &mut findings);
        }
    }
    assert!(files_read > 300, "the sweep read {files_read} files");
    assert!(templates > 800, "the sweep read {templates} templates");
    assert!(
        findings.is_empty(),
        "{} template references their form cannot fill:\n{}",
        findings.len(),
        findings.join("\n")
    );
}

#[test]
fn the_reader_finds_what_it_is_for() {
    assert_eq!(
        references("{?a} and {b} and \\u{416}"),
        (vec!["a".into()], vec!["b".into()])
    );
    let mut ast = Ast::new();
    let forms = parse(
        &mut ast,
        "(relation r T T :why \"{?1} {?3} {2}\")\n\
         (rule k (?p) :match (and (r ?a ?b) (absent (r ?b ?c))) :assert (r ?b ?a) :why \"{?p} {?a} {?c} {b}\")",
        Some("t.ein"),
    )
    .expect("parses");
    let mut out = Vec::new();
    for &f in &forms {
        check_form(&ast, f, "t.ein", &mut out);
    }
    assert_eq!(out.len(), 4, "{out:#?}");
    assert!(out[0].contains("{?3} is no position of its 2"));
    assert!(out[1].contains("{2} is {?2} with its `?` missing"));
    assert!(out[2].contains("{?c} names nothing it binds"));
    assert!(out[3].contains("{b} is {?b} with its `?` missing"));
}
