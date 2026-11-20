//! **The engine's four names are reserved** (Q-M1f.7: the dunder is never
//! bound).
//!
//! | name | where a program may write it | what holds it |
//! |---|---|---|
//! | `<forced-positive>`, `<monotonic-unconditional>`, `<lookahead-dies-immediately>` | nowhere: they are the rule names the engine writes for a conclusion no rule drew, with no meaning a program can give them | the grammar's `WRITEBACK` token, which no production takes |
//! | `__closed__` | wherever a name is **used**, as a relation or as a value (`(imply1 functional __closed__)`), and nowhere a name is **bound**: a declared relation, a rule's, a hypothesis rule's or a macro's name, an import's `:as` | `ein_core::RESERVED`, the loader's *shadows a reserved kernel name* |
//!
//! ```text
//! cargo test -p ein-ir --test engine_names
//! ```

use ein_ir::Ast;
use ein_ir::grammar::{self, ParseError};

/// The engine's three writeback rule names (`ein_core::terms::ENGINE`).
const WRITEBACKS: [&str; 3] = [
    ein_core::terms::FORCED_POSITIVE,
    ein_core::terms::MONOTONIC_UNCONDITIONAL,
    ein_core::terms::LOOKAHEAD_DIES,
];

/// The kernel's dunder.
const DUNDERS: [&str; 1] = [ein_core::terms::CLOSED];

/// What the parser says of a writeback name where it stands.
const WRITEBACK_REFUSED: &str = "the engine's writeback rule name (reserved)";

/// Parse `text` and load it: `Ok(())`, or every line it is refused with.
fn load(text: &str) -> Result<(), String> {
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let forms = grammar::parse(&mut ast, text, Some("probe.ein")).map_err(|e| e.to_string())?;
    ein_ir::load(&mut ast, &mut terms, &forms, None)
        .map(|_| ())
        .map_err(|e| e.0)
}

/// The parser's report on `text`, which must refuse it.
fn refusal(text: &str) -> ParseError {
    grammar::parse(&mut Ast::new(), text, Some("probe.ein")).expect_err("the parser refuses it")
}

/// The 1-based line and column of each place `name` is written in `text`.
fn places(text: &str, name: &str) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        for (at, _) in line.match_indices(name) {
            let col = line[..at].chars().count() + 1;
            out.push((i as u32 + 1, col as u32));
        }
    }
    out
}

/// **A writeback name is refused wherever a program writes it.** One program
/// per position, each name in turn: the parser reports an error at the name,
/// in its own words.
#[test]
fn a_writeback_name_is_refused_wherever_a_program_writes_it() {
    let positions = [
        ("a fact's relation", "(W a)"),
        ("a fact's argument", "(relation p T)\n(p W)"),
        ("a nested fact's relation", "(relation p T)\n(p (W a))"),
        ("a negated fact", "(not (W a))"),
        ("a declared relation", "(relation W T)"),
        (
            "a rule's name",
            "(relation p T)\n(relation q T)\n(rule W () :match (p ?x) :assert (q ?x))",
        ),
        (
            "a hypothesis rule's name",
            "(relation p T)\n(relation q T)\n(rule W () :match (p ?x) :assert (open (q ?x)))",
        ),
        ("a macro's name", "(relation p T)\n(macro W (?x) (p ?x))"),
        (
            "a premise's relation",
            "(relation q T)\n(rule r () :match (W ?x) :assert (q ?x))",
        ),
        (
            "a premise's argument",
            "(relation p T)\n(relation q T)\n(rule r () :match (p W) :assert (q a))",
        ),
        (
            "a conclusion's relation",
            "(relation p T)\n(rule r () :match (p ?x) :assert (W ?x))",
        ),
        (
            "a conclusion's argument",
            "(relation p T)\n(relation q T)\n(rule r () :match (p ?x) :assert (q W))",
        ),
        ("a goal", "(query :goal (W ?x))"),
        (
            "a claim's fact",
            "(relation p T)\n(p a)\n(query :goal (p ?x) :expect (model (p W)))",
        ),
        ("a `:no-hypothesis` list", "(solve :no-hypothesis (W))"),
        ("a fact's `:rule`", "(relation q T)\n(q a :rule W)"),
        ("an import's `:as`", "(import std.macro :as W)"),
        ("an import's `:symbols`", "(import std.macro :symbols (W))"),
    ];
    for name in WRITEBACKS {
        for (position, template) in positions {
            let text = template.replace('W', name);
            let at = places(&text, name);
            assert_eq!(at.len(), 1, "{position}: {text}");
            let report = refusal(&text);
            assert!(
                report
                    .errors
                    .iter()
                    .any(|e| (e.line, e.col) == at[0] && e.what.ends_with(WRITEBACK_REFUSED)),
                "{name} as {position}: the parser's report is not at {:?}:\n{report}",
                at[0]
            );
        }
    }
}

/// **What is not a writeback name is not refused as one**: the name in a
/// string or a comment, and a longer run that starts with one, which the name
/// rule reads as the name it spells.
#[test]
fn a_writeback_name_in_a_string_or_a_comment_or_a_longer_name_is_not_one() {
    for name in WRITEBACKS {
        let text = format!(
            "(relation p T :why \"{{?3}} is not {name}\")\n; {name}\n#| {name} |#\n(p a)\n"
        );
        load(&text).unwrap_or_else(|e| panic!("{name} in a string or a comment: {e}"));
        let longer = format!("(relation p T :why \"p holds of {{?3}}\")\n(p {name}x)\n");
        load(&longer).unwrap_or_else(|e| panic!("{name}x is a name: {e}"));
        assert!(!grammar::is_one_name(name), "{name} is a NAME of the lexer");
        assert!(grammar::is_one_name(&format!("{name}x")));
    }
}

/// **A probe, refused at its three writeback names**. It declares a
/// relation named `<forced-positive>`,
/// names two rules after the other two, and asks for the dunder's behaviour as
/// the relation of a fact, which Q-M1f.7 keeps legal. The
/// parser reports every error recovery reaches: one at each place a writeback
/// name is written, and none on the dunder's line, not only the first `<`,
/// at 5:11.
#[test]
fn a_probe_is_refused_at_its_three_writeback_names() {
    let probe = ";; A probe: the engine's writeback rule names and the
;; kernel's dunder are names under the name rule. A program writes each.
(relation p T :why \"{?3} is p\")
(relation hit T :why \"{?3} is hit\")
(relation <forced-positive> T :why \"{?3} forced\")
(relation s T T :why \"{?3} s {?4}\")
(rule <lookahead-dies-immediately> () :match (p ?x) :assert (hit ?x) :why \"a rule a program named after the kill-cache writeback\")
(rule <monotonic-unconditional> () :match (hit ?x) :assert (<forced-positive> ?x) :why \"and one named after the singleton writeback\")
(p a)
(s a b)
(__closed__ hit)
";
    let report = refusal(probe);
    let mut written: Vec<(u32, u32)> = WRITEBACKS
        .iter()
        .flat_map(|name| places(probe, name))
        .collect();
    written.sort();
    assert_eq!(
        written.len(),
        4,
        "the probe writes a writeback name four times"
    );
    let reported: Vec<(u32, u32)> = report.errors.iter().map(|e| (e.line, e.col)).collect();
    for e in &report.errors {
        assert!(
            written.contains(&(e.line, e.col)) && e.what.ends_with(WRITEBACK_REFUSED),
            "an error that is not at a writeback name:\n{report}"
        );
    }
    for name in WRITEBACKS {
        assert!(
            places(probe, name).iter().any(|at| reported.contains(at)),
            "{name} is not reported:\n{report}"
        );
    }
    assert!(
        report.errors.iter().all(|e| e.line < 9),
        "an error on the dunder's line:\n{report}"
    );
}

/// **A dunder is refused wherever a program binds it** (Q-M1f.7): a
/// declared relation, a rule's, a hypothesis rule's or a macro's name, and an import's
/// `:as`, each dunder in each, with the loader's line.
#[test]
fn a_dunder_is_refused_wherever_a_program_binds_it() {
    let positions = [
        (
            "relation",
            "(relation D T :why \"D holds of {?3}\")",
            "relation 'D' shadows a reserved kernel name",
        ),
        (
            "rule",
            "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n(rule D () :match (p ?x) :assert (q ?x) :why \"p holds of {?x}, so q holds of {?x}\")",
            "rule 'D' shadows a reserved kernel name",
        ),
        (
            "hypothesis rule",
            "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n(rule D () :match (p ?x) :assert (open (q ?x)) :why \"p holds of {?x}: guess that q holds of {?x}\")",
            "rule 'D' shadows a reserved kernel name",
        ),
        (
            "macro",
            "(relation p T :why \"p holds of {?3}\")\n(macro D (?x) (p ?x) :why \"p holds of {?x}\")",
            "macro 'D' shadows a reserved kernel name",
        ),
        (
            "an import's `:as`",
            "(import std.macro :as D)",
            "alias 'D' shadows a reserved kernel name",
        ),
    ];
    for name in DUNDERS {
        for (position, template, line) in positions {
            let text = format!("{}\n", template.replace('D', name));
            let want = line.replace('D', name);
            match load(&text) {
                Ok(()) => panic!("{name} as {position} loads"),
                Err(e) => assert!(e.contains(&want), "{name} as {position}: {e}"),
            }
        }
    }
}

/// **`__symmetric__` is no kernel name**: the kernel mirrors no relation, so
/// the name is a program's like any other, which a program may bind in each
/// place the kernel's dunder is refused, and a fact of it means what the
/// program's rules make it mean.
#[test]
fn a_name_the_kernel_does_not_read_binds_as_any_name_does() {
    let name = "__symmetric__";
    for (position, template) in [
        ("relation", "(relation D T :why \"D holds of {?3}\")"),
        (
            "rule",
            "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n(rule D () :match (p ?x) :assert (q ?x) :why \"p holds of {?x}, so q holds of {?x}\")",
        ),
        ("an import's `:as`", "(import std.macro :as D)"),
    ] {
        let text = format!("{}\n", template.replace('D', name));
        load(&text).unwrap_or_else(|e| panic!("{name} as {position}: {e}"));
    }
}

/// **A dunder is legal wherever a program uses it** (Q-M1f.7): as the
/// relation of a fact, a nested fact, a premise, a conclusion, a goal and a
/// claim's fact, and as a value: an argument, `std.algebra`'s documented
/// `(imply1 functional __closed__)` and a `:no-hypothesis` list. One program.
#[test]
fn a_dunder_is_legal_wherever_a_program_uses_it() {
    let text = "(import std.algebra :symbols (imply1))
(relation p T T :why \"p holds of {?3} and {?4}\")
(relation q T :why \"q holds of {?3}\")
(relation knows T T :why \"{?3} knows {?4}\")
(relation functional T :why \"{?3} is functional\")
(relation foo T T :why \"foo holds of {?3} and {?4}\")
(p a b)
(__closed__ p)
(q (__closed__ p))
(q __closed__)
(imply1 functional __closed__)
(functional foo)
(rule r () :match (__closed__ ?R) :assert (q ?R) :why \"the extension of {?R} is closed, so q holds of {?R}\")
(rule s () :match (p ?x ?y) :assert (__closed__ ?x) :why \"p holds of {?x} and {?y}, so the extension of {?x} is closed\")
(solve :no-hypothesis (__closed__))
(query :goal (__closed__ ?R) :expect (model (__closed__ p)))
";
    load(text).unwrap_or_else(|e| panic!("{e}"));
}
