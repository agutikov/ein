//! **The grammar's pages** — M1f S1f.5.5, T1f.5.5.5.
//!
//! The generated parser is the grammar: `src/ein.l` and `src/ein.y`, which
//! `build.rs` compiles. This file keeps two things in step with them.
//!
//! - **The page.** `docs/kernel/ir/03-ein-lang/00_ebnf.md`'s two grammar
//!   blocks are written here, from the two files, read through grmtools'
//!   public readers (`lrlex`'s `LRNonStreamingLexerDef` and `cfgrammar`'s
//!   `YaccGrammar`), so the page cannot state a token or a production the
//!   parser does not have. The test fails when the page differs, and
//!   `EIN_BLESS=1` rewrites the two marked regions, as
//!   `ein-cli/tests/embedding.rs` does for `docs/api/rust.md`.
//! - **The grammar's table** ([`TABLE`]). One row for each shape the grammar
//!   refuses where the loader, the compiler or the firing refused it until the
//!   swap (T1f.5.5.4), with the parse error it is refused with, banked in
//!   `tests/golden/grammar_table.txt`. Each of those lines was deleted at
//!   T1f.5.5.5, every one shown unreachable first (the stage's § T5 record),
//!   and each row names the line it replaced ([`DELETED`]), so a deleted line
//!   with no row fails here. The lines the swap left reachable stay where they
//!   are, and [`the_lines_the_grammar_does_not_cover_are_still_reached`] holds
//!   a program reaching each. The four language stages add their positions as
//!   rows after this task: S1f.5.1's `_`, S1f.5.2's string, S1f.5.4's names
//!   and S1f.5.11's keywords, each in every place. S1f.5.4's are in, at the
//!   table's end (T1f.5.4.3): a token that is no name, in every place a name
//!   goes, where until the swap it loaded split or was refused at a character.
//!   S1f.5.2's follow (T1f.5.2.2), a string and a variable in a fact wherever
//!   a value goes, S1f.5.1's after them (T1f.5.1.3): a bare `_` and `?_` in
//!   each place of S1f.5.1's table and wherever a name or a variable goes, and
//!   S1f.5.11's last (T1f.5.11.2): a keyword each form does not take, one row
//!   a form, and a keyword pair in each place in a term.

use std::fmt::Write as _;
use std::str::FromStr as _;

use cfgrammar::Symbol;
use cfgrammar::yacc::YaccGrammar;
use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::{Ast, load, parse};
use lrlex::{DefaultLexerTypes, LRNonStreamingLexerDef, LexerDef};

// ── The page ───────────────────────────────────────────────────────

/// The page the blocks are written into.
const PAGE: &str = "docs/kernel/ir/03-ein-lang/00_ebnf.md";

/// A grammar file, as `build.rs` reads it.
fn source(name: &str) -> String {
    let path = repo_root().join("ein.rs/crates/ein-ir/src").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn grammar() -> YaccGrammar<u32> {
    YaccGrammar::<u32>::from_str(&source("ein.y")).expect("ein.y reads through cfgrammar")
}

/// A token as the page writes it: one named for its text quoted, a class of
/// tokens bare.
fn token(name: &str) -> String {
    if name.chars().all(|c| c.is_ascii_uppercase()) {
        name.to_string()
    } else {
        format!("'{name}'")
    }
}

/// `ein.l`, rule by rule, read through `lrlex`: each token's regular
/// expression, the start conditions it is tried in and the one it enters, and
/// the name an error gives it (`ein.y`'s `%epp`).
fn lexical_block() -> String {
    type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;
    let def = Def::from_str(&source("ein.l")).expect("ein.l reads through lrlex");
    let states: Vec<&str> = def.iter_start_states().map(|s| s.name()).collect();
    let state = |id: usize| -> &str { states.get(id).copied().expect("a declared start state") };
    let grm = grammar();
    let epp = |name: &str| -> Option<String> {
        let t = grm.token_idx(name)?;
        grm.token_epp(t)
            .filter(|e| *e != name)
            .map(|e| format!("\"{e}\""))
    };
    let rows: Vec<(String, String, String)> = def
        .iter_rules()
        .map(|r| {
            let lhs = r.name().map_or_else(|| "(skipped)".to_string(), token);
            let mut notes: Vec<String> = Vec::new();
            if !r.start_states().is_empty() {
                let s: Vec<&str> = r.start_states().iter().map(|&i| state(i)).collect();
                notes.push(format!("in {}", s.join(", ")));
            }
            if let Some((to, _)) = r.target_state() {
                notes.push(format!("then {}", state(to)));
            }
            notes.extend(r.name().and_then(epp));
            (lhs, r.re_str().to_string(), notes.join("; "))
        })
        .collect();
    let width = rows
        .iter()
        .map(|r| r.0.chars().count())
        .filter(|&w| w <= 12)
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    writeln!(
        out,
        "/* ein-ir/src/ein.l, read by lrlex: a token's regular expression, the\n   \
         start conditions it is tried in, the one it enters, and the name an\n   \
         error gives it. Written by ein-ir/tests/grammar_pages.rs. */"
    )
    .unwrap();
    for s in def.iter_start_states().filter(|s| s.name() != "INITIAL") {
        writeln!(out, "%x {}", s.name()).unwrap();
    }
    for (lhs, re, notes) in rows {
        let pad = " ".repeat(width.saturating_sub(lhs.chars().count()));
        if notes.is_empty() {
            writeln!(out, "{lhs}{pad} ::= {re}").unwrap();
        } else {
            writeln!(out, "{lhs}{pad} ::= {re}    /* {notes} */").unwrap();
        }
    }
    out
}

/// `ein.y`, rule by rule, read through `cfgrammar`: each rule's alternatives,
/// a token as [`token`] writes it.
fn phrase_block() -> String {
    const WIDTH: usize = 96;
    let grm = grammar();
    let sym = |s: &Symbol<u32>| -> String {
        match s {
            Symbol::Rule(r) => grm.rule_name_str(*r).to_string(),
            Symbol::Token(t) => token(grm.token_name(*t).expect("a named token")),
        }
    };
    let mut out = String::new();
    writeln!(
        out,
        "/* ein-ir/src/ein.y, read by cfgrammar: every rule and its alternatives.\n   \
         Written by ein-ir/tests/grammar_pages.rs. The start rule is Program. */"
    )
    .unwrap();
    let start = grm.start_rule_idx();
    for r in grm.iter_rules() {
        if r == start || grm.implicit_rule() == Some(r) {
            continue;
        }
        let name = grm.rule_name_str(r);
        let alts: Vec<String> = grm
            .rule_to_prods(r)
            .iter()
            .map(|&p| {
                let syms: Vec<String> = grm.prod(p).iter().map(sym).collect();
                if syms.is_empty() {
                    "/* empty */".to_string()
                } else {
                    syms.join(" ")
                }
            })
            .collect();
        // Alternatives fill a line, and each line past the first starts `|`
        // under the `::=`.
        let indent = " ".repeat(name.chars().count() + 1);
        let mut line = format!("{name} ::= {}", alts[0]);
        for a in &alts[1..] {
            if line.chars().count() + 3 + a.chars().count() > WIDTH {
                writeln!(out, "{line}").unwrap();
                line = format!("{indent}  | {a}");
            } else {
                write!(line, " | {a}").unwrap();
            }
        }
        writeln!(out, "{line}").unwrap();
    }
    out
}

/// The page with each marked region holding `blocks`' text, fenced.
fn page_with(page: &str, blocks: &[(&str, String)]) -> String {
    let mut out = page.to_string();
    for (name, body) in blocks {
        let open = format!("<!-- generated: {name} -->\n");
        let close = format!("<!-- /generated: {name} -->");
        let a = out
            .find(&open)
            .unwrap_or_else(|| panic!("{PAGE} has no {open:?} marker"))
            + open.len();
        let b = a + out[a..]
            .find(&close)
            .unwrap_or_else(|| panic!("{PAGE} has no {close:?} marker"));
        out.replace_range(a..b, &format!("```text\n{body}```\n"));
    }
    out
}

/// **`00_ebnf.md`'s blocks are the grammar's.** Each is generated from the
/// file `build.rs` compiles; a page that differs fails, and `EIN_BLESS=1`
/// writes the blocks into it.
#[test]
fn the_pages_grammar_blocks_are_the_parsers() {
    let path = repo_root().join(PAGE);
    let page = std::fs::read_to_string(&path).expect("the page reads");
    let want = page_with(
        &page,
        &[("ein.l", lexical_block()), ("ein.y", phrase_block())],
    );
    if want == page {
        return;
    }
    if std::env::var_os("EIN_BLESS").is_some() {
        std::fs::write(&path, &want).expect("the page is written");
        return;
    }
    let first = page
        .lines()
        .zip(want.lines())
        .position(|(a, b)| a != b)
        .map_or(0, |i| i + 1);
    panic!(
        "{PAGE}'s grammar blocks are not ein.l and ein.y as the parser reads them \
         (first difference at line {first}); EIN_BLESS=1 rewrites them"
    );
}

/// Every token `ein.y` names is a rule of `ein.l`, so the lexical block lists
/// every token the phrase block writes.
#[test]
fn every_token_the_phrase_block_writes_the_lexical_block_defines() {
    type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;
    let def = Def::from_str(&source("ein.l")).expect("ein.l reads");
    let lexed: Vec<&str> = def.iter_rules().filter_map(|r| r.name()).collect();
    let grm = grammar();
    for t in grm.iter_tidxs() {
        if let Some(name) = grm.token_name(t) {
            assert!(
                lexed.contains(&name),
                "ein.y's token {name:?} has no rule in ein.l"
            );
        }
    }
}

// ── The grammar's table ────────────────────────────────────────────

/// A line the loader, the compiler or the firing refused a program with until
/// M1f S1f.5.5's T5 deleted it, the grammar having refused that shape at the
/// parse since the swap: its id, the file it was in, and its sentence as the
/// program read it.
const DELETED: &[(&str, &str, &str)] = &[
    (
        "top-level-form",
        "from_ir.rs",
        "unexpected top-level form: …",
    ),
    (
        "top-level-claim",
        "from_ir.rs",
        "a top-level (region …) — a region claim belongs under (solve … :expect …)",
    ),
    (
        "solve-keyword",
        "from_ir.rs",
        "(solve …): unknown keyword :k — one of :hrules :hypothesis-relations :no-hypothesis :expect",
    ),
    (
        "query-lattice-keyword",
        "from_ir.rs",
        "(query …): :hrules chooses the lattice and belongs on (solve …) since M1e2 S1e2.1a.4 — one program, one lattice",
    ),
    (
        "query-keyword",
        "from_ir.rs",
        "(query …): unknown keyword :k — one of :goal :goal-text :over :expect :mode (QUERY_KEYWORDS)",
    ),
    (
        "relation-name",
        "from_ir.rs",
        "(relation) needs a name at …",
    ),
    (
        "relation-malformed",
        "from_ir.rs",
        "malformed (relation) at …",
    ),
    (
        "rule-name-params",
        "from_ir.rs",
        "(rule) needs name + params at …",
    ),
    ("rule-malformed", "from_ir.rs", "malformed (rule …) at …"),
    (
        "wild-head",
        "from_ir.rs",
        "Found::WildHead: … writes (_ a) with the wildcard _ as its relation",
    ),
    (
        "nested-kw",
        "from_ir.rs",
        "Found::NestedKw: … writes :k v as an argument of p, and no fact can hold it",
    ),
    (
        "wild-guard",
        "from_ir.rs",
        "Found::WildGuard: … writes (eq ?x _), and _ names no value",
    ),
    (
        "guard-kwpair",
        "from_ir.rs",
        "Found::GuardNode: … writes (eq ?x :k a), and the keyword pair :k a is no value",
    ),
    ("fact-head", "from_ir.rs", "fact with non-atom head at …"),
    (
        "config-kw-pairs",
        "from_ir.rs",
        "(config …): (config …) body expects kw_pairs, got …",
    ),
    (
        "config-retired",
        "from_ir.rs",
        "(config …): config flag :k was retired at M1e2 …: … (a note on the parse error since the swap)",
    ),
    (
        "config-unknown",
        "from_ir.rs",
        "(config …): unknown config flag :k (expected one of: …)",
    ),
    (
        "config-bool",
        "from_ir.rs",
        "(config …): config flag :k expects true/false, got …",
    ),
    (
        "expect-atom-false",
        "expect.rs",
        ":expect false — ⊥ is spelled `(false)`, as it is in every `:assert`",
    ),
    (
        "expect-atom",
        "expect.rs",
        ":expect all — expected `(false)`, `(model …)` or `(or (model …) …)`",
    ),
    (
        "expect-not-a-form",
        "expect.rs",
        ":expect — expected `(false)`, `(model …)` or `(or (model …) …)`, at a value no form",
    ),
    (
        "expect-head",
        "expect.rs",
        ":expect — expected …, at a form whose head is no name",
    ),
    (
        "expect-false-args",
        "expect.rs",
        ":expect (false …) — ⊥ takes no arguments",
    ),
    (
        "expect-or-empty",
        "expect.rs",
        ":expect (or) — an empty disjunction expects nothing",
    ),
    (
        "expect-or-item-not-a-form",
        "expect.rs",
        ":expect (or …) — every disjunct must be a `(model …)`, at a disjunct no form",
    ),
    (
        "expect-or-item-not-model",
        "expect.rs",
        ":expect (or …) — every disjunct must be a `(model …)`",
    ),
    (
        "expect-and",
        "expect.rs",
        ":expect (and …) — a model is `(model …)`; `and` is a connective …",
    ),
    (
        "expect-head-other",
        "expect.rs",
        ":expect (models …) — expected `(false)`, `(model …)` or `(or (model …) …)`",
    ),
    (
        "expect-not-arity",
        "expect.rs",
        ":expect — `(not …)` takes exactly one fact",
    ),
    (
        "expect-fact-not-a-form",
        "expect.rs",
        ":expect — `3` is not a fact",
    ),
    (
        "expect-fact-head",
        "expect.rs",
        ":expect — `(?p A)` has no relation name in head position",
    ),
    (
        "expect-var",
        "expect.rs",
        ":expect — `?x` is a variable; an expectation is an answer, not a pattern",
    ),
    (
        "expect-wildcard",
        "expect.rs",
        ":expect — `_` is a wildcard; an expectation is an answer, not a pattern",
    ),
    (
        "expect-nested-head",
        "expect.rs",
        ":expect — a nested fact needs a relation name in head position",
    ),
    (
        "expect-arg-other",
        "expect.rs",
        ":expect — `:k v` is not a fact argument",
    ),
    (
        "over-not-a-name",
        "expect.rs",
        "(query …): :over (all) — one of all, minimal, maximal, at a value no name",
    ),
    (
        "claim-not-a-form",
        "claim.rs",
        "(solve … :expect …) — expected `(false)` or `(region …)`",
    ),
    (
        "claim-false-args",
        "claim.rs",
        "(solve … :expect (false …)) — ⊥ takes no arguments",
    ),
    (
        "claim-model-or-or",
        "claim.rs",
        "(solve … :expect (model …)) — a region is stated as `(region (model …) …)`; `(model …)` is a query's answer",
    ),
    (
        "claim-head-other",
        "claim.rs",
        "(solve … :expect (models …)) — (solve … :expect …) — expected `(false)` or `(region …)`",
    ),
    (
        "merged-arity",
        "claim.rs",
        "(merged …) — expected `(merged (model …) <from>)`, a state and its minimal commitments",
    ),
    (
        "merged-to",
        "claim.rs",
        "(merged …) — its first part is a `(model …)`, one of the states the region lists",
    ),
    (
        "merged-or-item",
        "claim.rs",
        "(merged …) — every commitment in its `(or …)` is a `(model …)`",
    ),
    (
        "merged-from",
        "claim.rs",
        "(merged …) — its second part is a `(model …)`, or an `(or (model …) …)` of them",
    ),
    (
        "owing-empty",
        "claim.rs",
        "(owing) — it lists the alive nodes that owe; an empty R with nothing listed is `(region)`",
    ),
    (
        "owing-item",
        "claim.rs",
        "(owing …) — every item is a `(model …)`",
    ),
    (
        "region-item",
        "claim.rs",
        "(region …) — `(p A)` is not `(model …)`, `(merged …)` or `(owing …)`",
    ),
    (
        "import-module",
        "imports.rs",
        "malformed (import …) — missing module name at …",
    ),
    (
        "import-as",
        "imports.rs",
        "(import m :as …) — alias must be a bare name at …",
    ),
    (
        "import-symbols-list",
        "imports.rs",
        "(import m :symbols …) — expected a (name …) list at …",
    ),
    (
        "import-symbols-empty",
        "imports.rs",
        "(import m :symbols ()) — empty list at …",
    ),
    (
        "macro-arity",
        "macros.rs",
        "(macro) needs name + params + body at …",
    ),
    ("macro-malformed", "macros.rs", "malformed (macro …) at …"),
    (
        "conclusion-argument",
        "ein-infer firing.rs",
        "rule 'r': its :assert writes _ as an argument of p, and no fact can hold it — …",
    ),
    (
        "absent-empty",
        "ein-infer compile.rs",
        "`(absent …)` sub-plan compiled to no steps in … — the guard would fail against every KB …",
    ),
];

/// One refusal: the place in the grammar, the deleted lines that refused the
/// shape before the swap (empty where none did: the shape loaded, or was
/// dropped, and the row is a ruling's), and the program.
struct Row {
    place: &'static str,
    was: &'static [&'static str],
    program: &'static str,
}

const fn row(place: &'static str, was: &'static [&'static str], program: &'static str) -> Row {
    Row {
        place,
        was,
        program,
    }
}

/// Declarations every rule row's program reads, so that a row's only error is
/// its own.
const RULES: &str = "(relation n T) (relation p T) (relation q T) (relation hit T)\n\
                     (relation box T) (relation tag T T)\n";

/// **The grammar's table.** Its rows were the tests of the lines T5 deleted:
/// `from_ir`'s unit tests of the wildcard relation, the shapes that read
/// nothing, the verdict atom's placements, an unknown query keyword, `:mode`, a
/// malformed `:expect` and the accumulating passes; `expect`'s of a claim's
/// shape; `ein-infer`'s `claim_semantics`, `compile_semantics` (an empty
/// `absent`, the config refusals, the wrapped declaration) and `naf_semantics`
/// (the retired flag). The outcome of each is banked in
/// `tests/golden/grammar_table.txt`.
const TABLE: &[Row] = &[
    // ── The top level ──
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "((p) a)",
    ),
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "(?x a)",
    ),
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "(\"s\" a)",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(relation p T) (p A) (region (model (p A)))",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(relation p T) (p A) (owing (model))",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(merged (model) (model))",
    ),
    // ── Declarations ──
    row("a relation's name", &["relation-name"], "(relation)"),
    row(
        "a relation's name",
        &["relation-name"],
        "(relation)\n(relation dup)\n(relation dup)\n(rule absent () :match (x ?a) :assert (y ?a))",
    ),
    row(
        "a relation's signature",
        &["relation-malformed"],
        "(relation R (T1 T2))",
    ),
    row(
        "a relation's signature",
        &["relation-malformed"],
        "(relation r ?A)",
    ),
    row(
        "a relation's name",
        &["relation-malformed"],
        "(relation (r) A)",
    ),
    row("a relation's name", &[], "(relation open A B)"),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(rule x)",
    ),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(hrule x)",
    ),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(rule)",
    ),
    row(
        "a rule's name",
        &["rule-malformed"],
        "(rule (x) () :match (a) :assert (b))",
    ),
    row(
        "a rule's parameters",
        &["rule-malformed"],
        "(rule x y :match (a) :assert (b))",
    ),
    row(
        "a rule's parameters",
        &["rule-malformed"],
        "(rule x :match (a) :assert (b))",
    ),
    row(
        "a rule's name",
        &[],
        "(relation r A B)\n(rule open () :match (r ?a ?b) :assert (r ?b ?a))",
    ),
    row(
        "a macro's parameters and body",
        &["macro-arity"],
        "(macro foo (?p))",
    ),
    row("a macro's parameters and body", &["macro-arity"], "(macro)"),
    row("a macro's name", &["macro-malformed"], "(macro ?x ?y)"),
    row(
        "a macro's parameters",
        &["macro-malformed"],
        "(macro foo () (rel ?a))",
    ),
    row(
        "a macro's name",
        &["macro-malformed"],
        "(macro (f) (?p) (r ?p))",
    ),
    row("a macro's name", &[], "(macro open (?P) (absent ?P))"),
    // ── Imports ──
    row("an import's module", &["import-module"], "(import)"),
    row(
        "an import's module",
        &["import-module"],
        "(import (std) :as m)",
    ),
    row(
        "an import's alias",
        &["import-as"],
        "(import std.macro :as \"m\")",
    ),
    row(
        "an import's alias",
        &["import-as"],
        "(import std.macro :as (m))",
    ),
    row(
        "an import's symbols",
        &["import-symbols-list"],
        "(import std.macro :symbols forall)",
    ),
    row(
        "an import's symbols",
        &["import-symbols-empty"],
        "(import std.macro :symbols ())",
    ),
    // ── A rule's :match ──
    row(
        "a premise's relation",
        &["wild-head"],
        "(rule w () :match (_ 2) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &["wild-head"],
        "(rule w () :match (box (_ 2)) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &["wild-head"],
        "(rule w () :match (absent (not (_ b))) :assert (hit w))",
    ),
    row(
        "a pattern's relation, in a guard",
        &["wild-head"],
        "(rule w () :match (and (n ?x) (eq ?x (_ 2))) :assert (hit w))",
    ),
    row(
        "a conclusion's relation",
        &["wild-head"],
        "(rule w () :match (n ?x) :assert (box (_ ?x)))",
    ),
    row(
        "a premise's relation, in an hrule",
        &["wild-head"],
        "(hrule h () :match (_ ?x) :assert (tag ?x ?x))",
    ),
    row(
        "a premise's relation and a conclusion's",
        &["wild-head"],
        "(rule r () :match (and (n ?x) (_ 2)) :assert (box (_ ?x)))",
    ),
    row(
        "a pattern's argument",
        &[],
        "(rule r () :match (and (n _) (box (tag _ _))) :assert (hit r))",
    ),
    row(
        "a pattern's argument, under an absent",
        &[],
        "(rule owe () :match (and (n ?x) (absent (tag ?x _))) :assert (open tag))",
    ),
    row(
        "a pattern's argument",
        &["nested-kw"],
        "(rule r () :match (box (p a :k v)) :assert (hit a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (n ?x :where v) :assert (hit r))",
    ),
    row(
        "a guard's argument",
        &["wild-guard"],
        "(rule e () :match (and (p ?x) (eq ?x _)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument, under an absent",
        &["wild-guard"],
        "(rule e () :match (and (p ?x) (absent (and (q ?y) (neq ?y _)))) :assert (hit ?x))",
    ),
    row(
        "a guard's argument, in an hrule",
        &["wild-guard"],
        "(hrule h () :match (and (p ?x) (eq ?x _)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument",
        &["guard-kwpair"],
        "(rule e () :match (and (p ?x) (eq ?x :k a)) :assert (hit ?x))",
    ),
    row("a :match", &[], "(rule r () :match _ :assert (hit a))"),
    row("a :match", &[], "(rule r () :match foo :assert (hit a))"),
    row(
        "a conjunct",
        &[],
        "(rule r () :match (and (p ?x) ?x) :assert (hit ?x))",
    ),
    row(
        "a disjunct",
        &[],
        "(rule r () :match (or 3 (q ?y)) :assert (hit a))",
    ),
    row(
        "an absent's premise",
        &[],
        "(rule r () :match (and (p ?x) (absent \"s\")) :assert (hit ?x))",
    ),
    row(
        "a conjunct, in an hrule",
        &[],
        "(hrule h () :match (and (p ?x) _) :assert (hit ?x))",
    ),
    row(
        "an absent's premise",
        &["absent-empty"],
        "(rule vacuous ()\n  :match (and (p ?x) (absent :k v))\n  :assert (hit ?x))",
    ),
    row(
        "an absent's premise",
        &["absent-empty"],
        "(rule vacuous ()\n  :match (and (p ?x) (absent ?x))\n  :assert (hit ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R) :match (and (relation ?R ?A ?B) (open ?R)) :assert (r a b))",
    ),
    // ── A rule's :assert ──
    row(
        "a conclusion's argument",
        &["nested-kw", "conclusion-argument"],
        "(rule r () :match (p ?x) :assert (box (p ?x :k v)))",
    ),
    row(
        "a conclusion's argument",
        &["conclusion-argument"],
        "(rule s () :match (p ?x) :assert (tag ?x _))",
    ),
    row(
        "a conclusion's argument",
        &["conclusion-argument"],
        "(rule s () :match (p ?x) :assert (tag ?x \"lbl\"))",
    ),
    row(
        "a conclusion's keyword",
        &[],
        "(rule s () :match (p ?x) :assert (tag ?x b :source \"x\"))",
    ),
    row("an :assert", &[], "(rule s () :match (p ?x) :assert foo)"),
    row(
        "a conclusion",
        &[],
        "(rule s () :match (p ?x) :assert (and (tag ?x b) ?x))",
    ),
    row(
        "an (open …)'s arguments",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (open ?b (?isa ?b ?B) (?R ?a ?b)))",
    ),
    row(
        "a conclusion beside (open …)",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (and (open ?R) (r ?a b)))",
    ),
    row(
        "an (open …)'s argument",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (open (r ?a ?b)))",
    ),
    // ── A (query …) ──
    row(
        "a query's keyword",
        &["query-keyword"],
        "(relation p T P)\n(p A H)\n(query :goal (p A ?h) :expct none)",
    ),
    row(
        "a query's keyword",
        &["query-keyword"],
        "(query :goal (a ?x) :mode all)",
    ),
    row(
        "a query's keyword",
        &["query-keyword"],
        "(query :goal (a ?x))\n(query :goal (b ?x) :mode all)\n(config :print-alive true)",
    ),
    row(
        "a query's keyword",
        &["query-lattice-keyword"],
        "(query :goal (a ?x) :hrules (h))",
    ),
    row(
        "a query's keyword",
        &["query-lattice-keyword"],
        "(query :goal (a ?x) :no-hypothesis (p))",
    ),
    row(
        "a query's :over",
        &["over-not-a-name"],
        "(query :goal (a ?x) :over (all))",
    ),
    row(
        "a query's :over",
        &["over-not-a-name"],
        "(query :goal (a ?x) :over ?x)",
    ),
    row(
        "a query's claim",
        &["expect-atom"],
        "(query :goal (p ?x) :expect all)",
    ),
    row(
        "a query's claim",
        &["expect-atom"],
        "(query :goal (p ?x) :expect none)",
    ),
    row(
        "a query's claim",
        &["expect-atom-false"],
        "(query :goal (p ?x) :expect false)",
    ),
    row(
        "a query's claim",
        &["expect-not-a-form"],
        "(query :goal (p ?x) :expect ?x)",
    ),
    row(
        "a query's claim",
        &["expect-not-a-form"],
        "(query :goal (p ?x) :expect \"s\")",
    ),
    row(
        "a query's claim",
        &["expect-head"],
        "(query :goal (p ?x) :expect (?m (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-false-args"],
        "(query :goal (p ?x) :expect (false A))",
    ),
    row(
        "a query's claim",
        &["expect-and"],
        "(query :goal (p ?x) :expect (and (p A) (p B)))",
    ),
    row(
        "a query's claim",
        &["expect-and"],
        "(query :goal (p ?x) :expect (and (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-head-other"],
        "(query :goal (p ?x) :expect (models (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-or-empty"],
        "(query :goal (p ?x) :expect (or))",
    ),
    row(
        "a query's claim",
        &["expect-or-item-not-model"],
        "(query :goal (p ?x) :expect (or (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-or-item-not-a-form"],
        "(query :goal (p ?x) :expect (or a))",
    ),
    row(
        "a claim's fact",
        &["expect-not-arity"],
        "(query :goal (p ?x) :expect (model (not (p A) (p B))))",
    ),
    row(
        "a claim's fact",
        &["expect-fact-not-a-form"],
        "(query :goal (p ?x) :expect (model 3))",
    ),
    row(
        "a claim's fact",
        &["expect-fact-head"],
        "(query :goal (p ?x) :expect (model (?p A)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-var"],
        "(query :goal (p ?x) :expect (model (p ?x)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-wildcard"],
        "(query :goal (p ?x) :expect (model (p _)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-nested-head"],
        "(query :goal (p ?x) :expect (model (p (?q A))))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-arg-other"],
        "(query :goal (p ?x) :expect (model (p :k v)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-arg-other"],
        "(query :goal (p ?x) :expect (model (p \"s\")))",
    ),
    // ── A (solve …) ──
    row(
        "a solve's keyword",
        &["solve-keyword"],
        "(solve :hrule (h))",
    ),
    row(
        "a solve's keyword",
        &["solve-keyword"],
        "(solve :goal (p ?x))",
    ),
    row(
        "a solve's claim",
        &["claim-not-a-form"],
        "(solve :expect region)",
    ),
    row(
        "a solve's claim",
        &["claim-false-args"],
        "(solve :expect (false A))",
    ),
    row(
        "a solve's claim",
        &["claim-model-or-or"],
        "(relation p T) (p A) (solve :expect (model (p A)))",
    ),
    row(
        "a solve's claim",
        &["claim-model-or-or"],
        "(solve :expect (or (model) (model)))",
    ),
    row(
        "a solve's claim",
        &["claim-head-other"],
        "(solve :expect (models))",
    ),
    row(
        "a region's item",
        &["region-item"],
        "(relation p T) (p A) (solve :expect (region (p A)))",
    ),
    row(
        "a region's item",
        &["merged-arity"],
        "(relation p T) (p A) (solve :expect (region (merged (model (p A)))))",
    ),
    row(
        "a region's item",
        &["merged-to"],
        "(solve :expect (region (model) (merged (p A) (model))))",
    ),
    row(
        "a region's item",
        &["merged-from"],
        "(solve :expect (region (model) (merged (model) (p A))))",
    ),
    row(
        "a region's item",
        &["merged-or-item"],
        "(solve :expect (region (model) (merged (model) (or (p A)))))",
    ),
    row(
        "a region's item",
        &["owing-empty"],
        "(relation p T) (p A) (solve :expect (region (owing)))",
    ),
    row(
        "a region's item",
        &["owing-item"],
        "(solve :expect (region (owing (p A))))",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p A) (solve :expect (region (model (p ?x))))",
    ),
    // ── A (config …) ──
    row("a config's body", &["config-kw-pairs"], "(config true)"),
    row(
        "a config's body",
        &["config-kw-pairs"],
        "(config lattice-sanity-check true)",
    ),
    row(
        "a config's keyword",
        &["config-unknown"],
        "(config :nope 1)",
    ),
    row(
        "a config's keyword",
        &["config-unknown"],
        "(config :lattice-sanity true)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :lattice-order-seed 7)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :candidate-order-seed 5)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :print-alive true)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :lattice-order lex)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :hypgen-rel-weight 2)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-singleton-writeback false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-forced-positive false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-symmetric-mirror false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :warn-derived-naf true)",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :lattice-sanity-check 7)",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :record-alternative-justifications \"false\")",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :lattice-sanity-check TRUE)",
    ),
    // ── S1f.5.4's names: a token that is no name, where a name goes ──
    // The name rule (the user, 2026-11-07, on F2 and F8): a name is a run of
    // ASCII letters, digits, connectors and decorators up to a separator, and
    // `?` and `:` only in front of a variable and a keyword. So a token that
    // runs on past one of those, or holds a character no name has, is no name
    // in any place one goes. Until the swap `a?x` loaded as `a` and `?x`, `"a"b`
    // as a string and a name, and `a:b`, `a,b` and `Жук` were refused at the
    // character. A run of digits in each place is a name, which
    // `ein-cli/tests/cli_semantics.rs`' `a_run_of_digits_is_a_name_in_every_place`
    // holds.
    row("a fact's argument", &[], "(relation p T) (p a?x)"),
    row("a fact's argument", &[], "(relation p T) (p Жук)"),
    row("a fact's relation", &[], "(a?x b)"),
    row(
        "a nested fact's argument",
        &[],
        "(relation p T) (p (q a:b))",
    ),
    row("a stored negation's argument", &[], "(not (p a,b))"),
    row("a relation's name", &[], "(relation a?x T)"),
    row("a relation's signature", &[], "(relation p a:b)"),
    row(
        "a rule's name",
        &[],
        "(rule a?x () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule r () :match (a?x ?y) :assert (q ?y))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p a?x) :assert (q a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p ?x?y) :assert (q ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x a#b)) :assert (q ?x))",
    ),
    row(
        "an absent's premise's argument",
        &[],
        "(rule r () :match (absent (p a|b)) :assert (q a))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (p ?x) :assert (q a,b))",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p \"a\"b)))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (a\\b))",
    ),
    row("an import's alias", &[], "(import std.macro :as a?x)"),
    row("a macro's name", &[], "(macro a?x (?a) (p ?a))"),
    // ── S1f.5.2's strings: a string is natural-language text, and no value ──
    // The user, 2026-11-06 (Q-M1e2.23): a string is written only after `:why`,
    // `:source` or `:goal-text`, *"(p "1") has to be refused"*; and G-6, ruled
    // the same day: a variable in a fact is refused. Until the swap a string
    // anywhere else was the name it spells (`(p "a")` was `(p a)`, M1e2
    // S1e2.2.16 and S1e2.2.18), and a variable in a fact the name `?x`, printed
    // `"?x"`. Each is refused at the parse, a string with S1f.5.2's note; the
    // rows above that write a string (a form's head, an import's alias, an
    // `absent`'s premise, a conclusion's argument, a conclusion's `:source`,
    // an `:expect`, a claim's fact, a config flag's value) hold it in their
    // places. The three keywords' text loads and reaches every surface that
    // carries it, which `ein-cli/tests/natural_language_text.rs` holds, and
    // every command refuses `(p "1")` with one line and the note, which
    // `ein-cli/tests/cli_semantics.rs`' `a_string_is_natural_language_and_no_value`
    // holds.
    row("a fact's argument", &[], "(relation p T) (p \"1\")"),
    row("a fact's argument", &[], "(relation p T) (p \"a\")"),
    row("a fact's argument", &[], "(relation p T) (p \"Жук\")"),
    row(
        "a nested fact's argument",
        &[],
        "(relation p T) (p (q \"a\"))",
    ),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p \"a\"))",
    ),
    row(
        "a fact's argument",
        &[],
        "(relation is-a T T) (is-a \"x\" T)",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation link T) (rule chase (?r) :match (link ?x) :assert (?r ?x)) (chase \"link\")",
    ),
    row("a fact's :rule", &[], "(relation p T) (p a :rule \"r\")"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p a :rule r :using ((q \"b\")))",
    ),
    row("a fact's argument", &[], "(relation r T) (r ?x)"),
    row("a nested fact's argument", &[], "(relation p T) (p (q ?x))"),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p ?x))",
    ),
    row("a relation's name", &[], "(relation \"p\" T)"),
    row("a relation's signature", &[], "(relation p \"T\")"),
    row(
        "a rule's name",
        &[],
        "(rule \"r\" () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (\"x\") :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule r () :match (\"p\" ?x) :assert (q ?x))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p \"1\") :assert (q a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (box (p \"a\")) :assert (q a))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (eq ?x \"a\")) :assert (q ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x \"1\")) :assert (q ?x))",
    ),
    row(
        "an absent's premise's argument",
        &[],
        "(rule r () :match (absent (p \"a\")) :assert (q a))",
    ),
    row(
        "a stored negation's premise",
        &[],
        "(rule r () :match (not (p \"a\")) :assert (q a))",
    ),
    row(
        "a conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (\"q\" ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (p ?x) :assert (and (q ?x) (not (p \"a\"))))",
    ),
    row(
        "an open conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (open \"q\"))",
    ),
    row(
        "an hrule's premise",
        &[],
        "(hrule g () :match (p \"a\") :assert (q a))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (q \"a\"))",
    ),
    row(
        "a rule's keyword",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x) :priority \"5\")",
    ),
    row(
        "a query's goal",
        &[],
        "(relation p T) (p a) (query :goal (p \"a\"))",
    ),
    row(
        "a query's :over",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :over \"all\")",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p \"1\")))",
    ),
    row(
        "a claim's neq",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (neq \"a\" b)))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p \"a\"))))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (\"p\"))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :hypothesis-relations (\"p\"))",
    ),
    row("a solve's hrules", &[], "(solve :hrules (\"g\"))"),
    row("a solve's hrules", &[], "(solve :hrules (g (\"a\")))"),
    row(
        "a config flag's value",
        &[],
        "(config :lattice-sanity-check \"true\")",
    ),
    row("a macro's name", &[], "(macro \"m\" (?a) (p ?a))"),
    row("a macro's parameters", &[], "(macro m (\"a\") (p ?a))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a \"b\"))"),
    row("an import's module", &[], "(import \"std.macro\")"),
    row(
        "an import's symbols",
        &[],
        "(import std.macro :symbols (\"forall\"))",
    ),
    row("a trace step's name", &[], "(trace (step \"s1\" :rule r))"),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (p \"a\")))",
    ),
    row(
        "a fact's keyword",
        &[],
        "(relation p T) (p a :why \"text\")",
    ),
    row(
        "a relation's keyword",
        &[],
        "(relation p T :source \"text\")",
    ),
    row(
        "a query's keyword",
        &[],
        "(relation p T) (query :goal (p ?x) :why \"text\")",
    ),
    // ── S1f.5.1's `_`: a bare `_` and `?_` name nothing ──
    // The user, 2026-11-06 (Q-M1e2.22): *"simple `_` reserved and refused
    // because of potential ambiguity, but remain valid in any place in atom and
    // variable names"*; `?_` with it (S1f.5.1's default 1: two `?_` in one rule
    // would bind one value, the opposite of what the spelling suggests). Each is
    // a token no production takes (`ein.l`'s `UNDERSCORE` and `VARUNDERSCORE`),
    // refused at the parse with S1f.5.1's note, in every place of S1f.5.1's
    // table and every place a name or a variable goes. Until the swap `_` was
    // dropped from a fact, an activator and a nested fact (W-22, W29), matched
    // any value in a rule's pattern (W20), and was refused at load as a
    // relation, a guard's argument or a premise (the rows above that write it,
    // T5's); `?_` was refused at the lexer. Every command that reads a program
    // refuses both, which `ein-cli/tests/cli_semantics.rs`'
    // `a_bare_underscore_names_nothing_in_every_place` holds.
    row("a fact's argument", &[], "(relation r T T) (r _ a)"),
    row("a fact's argument", &[], "(relation r T T) (r ?_ a)"),
    row(
        "a nested fact's argument",
        &[],
        "(relation r T T) (r (f _ a) c)",
    ),
    row(
        "a nested fact's argument",
        &[],
        "(relation r T T) (r (f ?_ a) c)",
    ),
    row("a nested fact's relation", &[], "(relation h T) (h (_ b))"),
    row("a nested fact's relation", &[], "(relation h T) (h (?_ b))"),
    row("a fact's argument", &[], "(relation is-a T T) (is-a _ T)"),
    row("a fact's argument", &[], "(relation is-a T T) (is-a ?_ T)"),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p _))",
    ),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p ?_))",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation p T) (relation hit T) (rule q (?x) :match (p ?x) :assert (hit ?x)) (q _)",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation p T) (relation hit T) (rule q (?x) :match (p ?x) :assert (hit ?x)) (q ?_)",
    ),
    row("a fact's :rule", &[], "(relation p T) (p a :rule _)"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p a :rule r :using ((q ?_)))",
    ),
    row("a top-level form's head", &[], "(_ a)"),
    row("a top-level form's head", &[], "(?_ a)"),
    row("a relation's name", &[], "(relation _ T)"),
    row("a relation's name", &[], "(relation ?_ T)"),
    row("a relation's signature", &[], "(relation r _ T)"),
    row("a relation's signature", &[], "(relation r ?_ T)"),
    row(
        "a rule's name",
        &[],
        "(rule _ () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's name",
        &[],
        "(rule ?_ () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (_) :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (?_) :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (n ?_) :assert (hit r))",
    ),
    row(
        "a pattern's argument",
        &[],
        "(rule r () :match (box (tag ?_ a)) :assert (hit r))",
    ),
    row(
        "a pattern's argument, under an absent",
        &[],
        "(rule r () :match (and (n ?x) (absent (tag ?x ?_))) :assert (hit ?x))",
    ),
    row(
        "a premise's argument, in an hrule",
        &[],
        "(hrule h () :match (n _) :assert (hit a))",
    ),
    row(
        "a premise's argument, in an hrule",
        &[],
        "(hrule h () :match (n ?_) :assert (hit a))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule w () :match (?_ 2) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &[],
        "(rule w () :match (box (?_ 2)) :assert (hit w))",
    ),
    row(
        "a premise's relation, in an hrule",
        &[],
        "(hrule h () :match (?_ ?x) :assert (tag ?x ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule e () :match (and (p ?x) (eq ?x ?_)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule e () :match (and (p ?x) (neq ?_ ?x)) :assert (hit ?x))",
    ),
    row("a :match", &[], "(rule r () :match ?_ :assert (hit a))"),
    row(
        "a conjunct",
        &[],
        "(rule r () :match (and (p ?x) ?_) :assert (hit ?x))",
    ),
    row(
        "a disjunct",
        &[],
        "(rule r () :match (or _ (q ?y)) :assert (hit a))",
    ),
    row(
        "an absent's premise",
        &[],
        "(rule r () :match (and (p ?x) (absent _)) :assert (hit ?x))",
    ),
    row(
        "a conclusion's relation",
        &[],
        "(rule w () :match (n ?x) :assert (?_ ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule s () :match (p ?x) :assert (tag ?x ?_))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule s () :match (p ?x) :assert (and (hit ?x) (not (tag ?x _))))",
    ),
    row("a conclusion", &[], "(rule s () :match (p ?x) :assert _)"),
    row("a conclusion", &[], "(rule s () :match (p ?x) :assert ?_)"),
    row(
        "an open conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (open _))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (tag ?x _))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (tag ?x ?_))",
    ),
    row(
        "a query's goal",
        &[],
        "(relation holds T) (holds a) (query :goal (holds _))",
    ),
    row(
        "a query's goal",
        &[],
        "(relation holds T) (holds a) (query :goal (holds ?_))",
    ),
    row(
        "a query's :over",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :over _)",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p ?_)))",
    ),
    row(
        "a claim's neq",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (neq _ b)))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p _))))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p ?_))))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (_))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :hypothesis-relations (?_))",
    ),
    row("a solve's hrules", &[], "(solve :hrules (_))"),
    row(
        "a config flag's value",
        &[],
        "(config :lattice-sanity-check _)",
    ),
    row("a macro's name", &[], "(macro _ (?a) (p ?a))"),
    row("a macro's parameters", &[], "(macro m (?_) (p a))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a _))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a ?_))"),
    row("an import's module", &[], "(import _)"),
    row("an import's alias", &[], "(import std.macro :as _)"),
    row(
        "an import's symbols",
        &[],
        "(import std.macro :symbols (?_))",
    ),
    row("a trace step's name", &[], "(trace (step _ :rule r))"),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (p ?_)))",
    ),
    // ── S1f.5.11's keywords: each form takes a fixed set, and a term none ──
    // The user, 2026-11-07 (Q-M1e2.28, D-15, G-15, G-22): *"there are fixed set
    // of allowed kwargs, if parser sees :k or :G :notes or whatever else
    // unknown kwarg - it is an error"*, and *"yes, plan to remove `:where`"*.
    // Each form's keywords are its tokens in `ein.y` (S1f.5.11's § The lists),
    // and no production of a term or a nested fact takes one, so a keyword a
    // form does not list, `:where`, `:cardinality` and `:mode` among them, is
    // refused at the parse, and so is any keyword pair inside a term. Until
    // the swap only `query`, `solve` and `config` refused an unknown keyword
    // (their rows are T5's, above); every other form, and every term, dropped
    // one in silence: a `:match`'s or a goal's pair was dropped by the compiler
    // (Q32), a macro call's by the expander's arity filter, both retired by
    // S1f.5.11's T1f.5.11.2, and an annotation's by the loader. W32's seven
    // rows were the wrong answers. One row for each form, then one for each
    // place in a term; `fuzz_properties`' last property holds the lists on
    // every program the fuzzer's stream accepts.
    row(
        "a relation's keyword",
        &[],
        "(relation s Node :cardinality 1..1)",
    ),
    row(
        "a rule's keyword",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x) :where (neq ?x a))",
    ),
    row(
        "an hrule's keyword",
        &[],
        "(hrule h () :match (p ?x) :assert (q ?x) :zz 1)",
    ),
    row("a fact's keyword", &[], "(relation q T) (q c :zz 1)"),
    row(
        "a stored negation's keyword",
        &[],
        "(relation q T) (not (q c) :k v)",
    ),
    row("a query's keyword", &[], "(query :goal (p ?x) :G 1)"),
    row("a solve's keyword", &[], "(solve :notes (p))"),
    row("a config's keyword", &[], "(config :zz true)"),
    row("an import's keyword", &[], "(import std.macro :zz 1)"),
    row("a macro's keyword", &[], "(macro m (?a) (p ?a) :k v)"),
    row("a trace's keyword", &[], "(trace :k v)"),
    row(
        "a trace step's keyword",
        &[],
        "(trace (step s1 :rule r :derives (q a) :zz 1))",
    ),
    row(
        "a :match's and",
        &[],
        "(rule r () :match (and (p ?x) :where (neq ?x a)) :assert (q ?x))",
    ),
    row(
        "a :match's and",
        &[],
        "(relation r T T)\n(rule old-where (?r)\n  :match (and (?r ?a ?b) :where (neq ?a ?b))\n  :assert (?r ?b ?a)\n  :why \"ow\")\n(old-where r)",
    ),
    row(
        "a :match's or",
        &[],
        "(rule r () :match (or (p ?x) (q ?x) :k v) :assert (hit ?x))",
    ),
    row(
        "a pattern",
        &[],
        "(rule r () :match (p ?x :k v) :assert (q ?x))",
    ),
    row(
        "a pattern, in an hrule",
        &[],
        "(hrule h () :match (p ?x :k v) :assert (q ?x))",
    ),
    row(
        "an absent",
        &[],
        "(rule r () :match (and (p ?x) (absent (q ?x) :k v)) :assert (hit ?x))",
    ),
    row(
        "a premise's stored negation",
        &[],
        "(rule r () :match (and (p ?x) (not (q ?x) :k v)) :assert (hit ?x))",
    ),
    row(
        "a guard",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x a :k v)) :assert (hit ?x))",
    ),
    row(
        "an :assert's and",
        &[],
        "(rule r () :match (p ?x) :assert (and (q ?x) :k v))",
    ),
    row(
        "a conclusion",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x :k v))",
    ),
    row(
        "a conclusion's stored negation",
        &[],
        "(rule r () :match (p ?x) :assert (not (q ?x) :k v))",
    ),
    row(
        "a :goal's and",
        &[],
        "(relation p T) (p a) (p b)\n(query :goal (and (p ?x) :zz 1) :expect (model (p a) (p b)))",
    ),
    row("a :goal's pattern", &[], "(query :goal (p ?x :zz 1))"),
    row(
        "a macro call in a :match",
        &[],
        "(import std.macro :symbols (forall))\n(rule r () :match (and (p ?x) (forall (q ?y) (p ?y) :k v)) :assert (hit ?x))",
    ),
    row(
        "a macro call in an :assert",
        &[],
        "(macro both (?a) (and (q ?a) (hit ?a)))\n(rule r () :match (p ?x) :assert (both ?x :k v))",
    ),
    row("a macro's body", &[], "(macro m (?a) (p ?a :k v))"),
    row(
        "a nested fact",
        &[],
        "(relation facts F) (relation r T T)\n(facts (r a b :source \"(1)\"))",
    ),
    row(
        "a nested fact in a :using",
        &[],
        "(relation q T) (q c :rule r :using ((p c :source \"s\")))",
    ),
    row(
        "a claim's model",
        &[],
        "(query :goal (p ?x) :expect (model (p a) (p b) :zz 1))",
    ),
    row(
        "a region claim",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p a)) :zz 1))",
    ),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (q a :k v)))",
    ),
];

/// A row's program: a rule row's declarations before it.
fn text(r: &Row) -> String {
    if r.program.contains("(rule ") || r.program.contains("(hrule ") {
        if r.program.contains("(relation ") {
            r.program.to_string()
        } else {
            format!("{RULES}{}", r.program)
        }
    } else {
        r.program.to_string()
    }
}

/// **Every shape the table holds is refused at the parse**, with the report
/// `tests/golden/grammar_table.txt` banks: every error, its notes, its ranked
/// repairs. Before the swap each was refused by the line its row names, or
/// loaded.
#[test]
fn every_row_of_the_grammars_table_is_refused_at_the_parse() {
    let mut out = String::new();
    for (i, r) in TABLE.iter().enumerate() {
        let program = text(r);
        let report = match parse(&mut Ast::new(), &program, Some("<row>")) {
            Ok(_) => panic!("row {i} ({}) parses: {program}", r.place),
            Err(e) => e.to_string(),
        };
        let was = if r.was.is_empty() {
            "—".to_string()
        } else {
            r.was.join(", ")
        };
        writeln!(out, "## {i}. {} (was: {was})", r.place).unwrap();
        for line in program.lines() {
            writeln!(out, "    {line}").unwrap();
        }
        writeln!(out, "{report}\n").unwrap();
    }
    let path = golden_path("ein-ir", "grammar_table.txt");
    if let Some(diff) = golden(&path, &out) {
        panic!("{diff}");
    }
}

/// **Each deleted line has a row**, and each row names only lines that were.
#[test]
fn every_deleted_line_is_a_row_of_the_table() {
    let ids: Vec<&str> = DELETED.iter().map(|d| d.0).collect();
    for r in TABLE {
        for w in r.was {
            assert!(
                ids.contains(w),
                "row {:?} names {w:?}, no deleted line",
                r.program
            );
        }
    }
    for (id, file, line) in DELETED {
        assert!(
            TABLE.iter().any(|r| r.was.contains(id)),
            "{file}'s deleted line {id:?} ({line}) has no row"
        );
    }
}

// ── What stays the loader's ────────────────────────────────────────

/// The load error `text` is refused with: it parses, and the loader refuses it.
fn load_error(text: &str) -> String {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<loader>")).unwrap_or_else(|e| panic!("{text}\n{e}"));
    match load(&mut ast, &mut Terms::new(), &forms, None) {
        Ok(_) => panic!("{text} loads"),
        Err(e) => e.0,
    }
}

/// **The lines the grammar does not cover are still reached**, each by a
/// program the grammar accepts — which is why S1f.5.5's T5 kept them, where the
/// census counted them the grammar's (`s1f.5.5_census.md` § The refusal lines).
///
/// - **A reserved name a declarator binds**: the kernel's two dunders are names
///   (Q-M1f.7 (b), T1f.5.5.11), so the loader refuses `__closed__` bound by a
///   relation, a rule, a macro or an import's `:as`; the other reserved words
///   are tokens no declarator takes, refused at the parse (the table above).
/// - **`(open …)`'s placement, arity and argument**: a pattern's argument may be
///   any name, `open` among them (`ein.y`'s `ArgHead`), and a macro's body is a
///   template, which a conclusion invoking it expands into any of the shapes.
/// - **A premise that is not a form** (`Found::NotAForm`): a macro whose body
///   is a bare parameter puts its argument where a premise goes.
///
/// The compiler's `absent` and `eq` arities and the firing's whole conclusion
/// that is no fact are reached the same way, through a macro's body, and held
/// in `ein-infer/tests/compile_semantics.rs`.
#[test]
fn the_lines_the_grammar_does_not_cover_are_still_reached() {
    for (text, want) in [
        (
            "(relation __closed__ T)",
            "relation '__closed__' shadows a reserved kernel name",
        ),
        (
            "(relation p T) (rule __symmetric__ () :match (p ?x) :assert (p ?x))",
            "rule '__symmetric__' shadows a reserved kernel name",
        ),
        (
            "(macro __closed__ (?x) (p ?x))",
            "macro '__closed__' shadows a reserved kernel name",
        ),
        (
            "(import std.macro :as __closed__)",
            "alias '__closed__' shadows a reserved kernel name",
        ),
        (
            "(relation p T) (relation hit T)\n(rule r () :match (p (open ?y)) :assert (hit a))",
            "`(open …)` is a verdict about the KB and is legal only in :assert",
        ),
        (
            "(macro both (?r) (and (open ?r) (hit a)))\n(relation p T) (relation hit T)\n\
             (rule r (?R) :match (and (p ?x) (absent (?R ?x ?y))) :assert (both ?R))",
            "a rule asserting `open` may assert nothing else",
        ),
        (
            "(macro o2 (?r) (open ?r ?r))\n(relation p T) (relation hit T)\n\
             (rule r (?R) :match (and (p ?x) (absent (?R ?x ?y))) :assert (o2 ?R))",
            "`open` takes the incomplete relation and nothing else",
        ),
        (
            "(macro o1 (?r) (open ?r))\n(relation p T) (relation hit T)\n\
             (rule r (?R) :match (and (p ?x) (absent (?R ?x ?y))) :assert (o1 (p ?x)))",
            "`open`'s argument names a relation",
        ),
        (
            "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) \
             (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (not (open ?R)))",
            "`(open …)` is a whole conclusion, not a term inside one",
        ),
        (
            "(macro id (?x) ?x)\n(relation p T) (relation hit T)\n(rule r () :match (id foo) :assert (hit a))",
            "rule 'r': its :match is foo, and foo is no premise",
        ),
        (
            "(macro all (?b ?G ?B) (absent (and ?G (absent ?B))))\n(relation hit T)\n\
             (rule r () :match (all ?x foo bar) :assert (hit a))",
            "rule 'r': its :match writes foo as a premise of and, and foo is no premise",
        ),
    ] {
        let got = load_error(text);
        assert!(got.contains(want), "{text}\nwants {want:?}, got {got:?}");
    }
}
