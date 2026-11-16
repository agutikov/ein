//! The loader's contract.
//!
//! The KB has no CLI surface of its own: `ein_core::shape` renders the
//! registries, the fact list and the seven indexes as one deterministic text.
//! Where each part of the contract is checked:
//!
//! | claim | where |
//! |---|---|
//! | every corpus file loads to the same KB | `ein-render`'s `corpus_shapes.md5`, 107 `::load` lines, carrying this test's `loaded >= 60` floor |
//! | layering holds after a load | [`layering_holds_after_every_load`], its own sweep |
//! | the 32-body `(config …)` coercion | [`the_config_coercion_table_is_stable`], against a checked-in table |
//! | the accumulated message | [`load_errors_accumulate_in_pass_order`], the same way |
//! | 16 of the `.expected` fixtures | [`the_load_negative_fixtures_are_byte_identical`] |
//! | `require_why` refuses a relation, rule or hrule with no `:why`, one line each, and loads it without the flag | [`require_why_refuses_each_form_without_one`] |
//! | a macro is asked for a `:why` too, since its parser reads one | [`a_macro_needs_a_why_only_where_its_parser_reads_one`] |
//! | `require_why` decides only whether a corpus program loads: one it lets through loads to the same KB, and it refuses none | [`require_why_decides_loading_only`] |
//! | the default requires a `:why`, and `require_why: false` loads without one (Q-M1f.62) | [`the_default_requires_a_why_and_false_loads_without`] |

use ein_core::{Terms, shape};
use ein_corpus::{corpus_files, golden, golden_path, repo_root};
use ein_ir::{Ast, load_file};
use std::path::{Path, PathBuf};

/// What one load answered — the vocabulary the tables below are *rendered*
/// in, so a renamed variant would invalidate a checked-in golden for no
/// reason.
enum Answer {
    Ok(String),
    Err { kind: &'static str, msg: String },
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Ok(text) => write!(f, "{text}"),
            Answer::Err { kind, msg } => write!(f, "<{kind}> {msg}"),
        }
    }
}

/// One file, loaded.
fn load_shape(path: &Path) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    match load_file(&mut ast, &mut terms, path) {
        Ok(kb) => Answer::Ok(shape(&kb, &terms)),
        Err(e) => Answer::Err {
            kind: "KBLoadError",
            msg: e.0,
        },
    }
}

/// **Layering holds after every load.**
///
/// `check_layering` is the claim here, not a precondition of some other
/// comparison — a file whose layering broke panics out of a test named for
/// exactly that. Every fact's layer is consistent with its provenance on
/// every corpus file that loads, before any rule has run.
#[test]
fn layering_holds_after_every_load() {
    let mut loaded = 0;
    for path in &corpus_files() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        kb.check_layering(&terms)
            .unwrap_or_else(|e| panic!("{name}: layering broken after load: {e:?}"));
        loaded += 1;
    }
    assert!(loaded >= 60, "only {loaded} corpus files loaded");
}

/// The loader's own fixtures, against the committed `.expected` files.
///
/// **The four `reserved_open_*`** are one declaration reached four ways:
/// written directly, imported flat via `:symbols`, imported qualified,
/// imported under `:as`. Four files rather than one because the route is what
/// can differ: a module's declaration takes its full name, and one that did
/// so against a copy of the reserved list of its own would let a reserved
/// name load by two of the routes. `open` is a word a position gives a
/// meaning to and no name a declarator takes, so `(macro open …)` is refused
/// at the parse, and each of the four reports that error in
/// `reserved_open_direct.ein`, the module every route reaches; the loader's
/// route-independence is held by `imports_semantics`' cells over a kernel
/// dunder, which the grammar reads as a name.
#[test]
fn the_load_negative_fixtures_are_byte_identical() {
    let root = repo_root();
    let dir = root.join("examples/broken/load");
    let stdlib = std::env::var("EIN_STDLIB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("stdlib"));

    let mut bad = Vec::new();
    // Nine more are refused at the parse, under `examples/broken/`, where
    // `grammar_decisions.rs` holds their reports: `config_bad_value`,
    // `config_unknown_flag`, `empty_absent`, `hrule_reserved_name`,
    // `macro_reserved_name`, `relation_malformed`, `relation_needs_a_name`,
    // `relation_reserved_name` and `rule_reserved_name`; and `(mirror 3)`
    // names the relation `3`, so it has no fixture here.
    // `eq_in_a_macro_body` (W66): the loader holds a macro's expansion to the
    // grammar's shapes, its line the macro body's position first.
    for name in [
        "derivation_cycle",
        "eq_in_a_macro_body",
        "hrule_duplicate_name",
        "macro_arity_mismatch",
        "macro_duplicate",
        "relation_duplicate",
        "reserved_open_aliased",
        "reserved_open_direct",
        "reserved_open_qualified",
        "reserved_open_symbols",
        "rule_duplicate_name",
        "rule_half_declarators",
        "rule_missing_assert",
        "rule_missing_match",
        "unbound_head",
        "unimported_std_macro",
    ] {
        let path = dir.join(format!("{name}.ein"));
        let expected = std::fs::read_to_string(dir.join(format!("{name}.expected")))
            .expect("an .expected beside every fixture")
            .trim_end()
            .replace("{FILE}", path.to_str().expect("utf-8"))
            .replace("{DIR}", dir.to_str().expect("utf-8"))
            .replace("{STDLIB}", stdlib.to_str().expect("utf-8"));
        match load_shape(&path) {
            Answer::Err { msg, .. } if msg == expected => {}
            Answer::Err { msg, .. } => {
                bad.push(format!("{name}\n  want: {expected}\n  got:  {msg}"))
            }
            Answer::Ok(_) => bad.push(format!("{name}: loaded, but must fail")),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The `(config …)` coercion — every coercion path and every message,
/// driven through a `(config …)` body rather than through the Rust API, so
/// what is checked is what a puzzle author would see.
///
/// A table rather than a diff: the thirty-two bodies below were chosen to hit
/// every branch of the coercion — the four field kinds, the integer and
/// float coercions' tolerances (`"1_000"`, `" 7 "`, and the `"1_"` / `"1.5"`
/// an int refuses), a malformed body, an unknown flag, and the
/// last-one-wins rule — and each one's *answer* is the assertion. Rendered as
/// one file so a change reads as a diff of the affected bodies rather than as
/// thirty-two separate failures.
///
/// **Most rows are parse errors**, each a path a body can take: a
/// `(config …)` flag takes `true` or `false` and nothing else, so a string, a
/// name of digits or any other name, a variable, a nested fact and a keyword
/// no form takes are refused at the parse, each with its repairs, and a
/// retired flag with its note. Only `true`, `false`, the empty block and the
/// last-one-wins rows reach the loader; the coercion's other branches are
/// reached by no program.
#[test]
fn the_config_coercion_table_is_stable() {
    let bodies = [
        // Booleans: the atoms, the strings, and the wrong shapes.
        "(config :lattice-sanity-check true)",
        "(config :lattice-sanity-check false)",
        "(config :lattice-sanity-check \"true\")",
        "(config :lattice-sanity-check TRUE)",
        "(config :lattice-sanity-check 1)",
        "(config :lattice-sanity-check maybe)",
        "(config :lattice-sanity-check ?x)",
        "(config :lattice-sanity-check 1..5)",
        "(config :lattice-sanity-check (nested a))",
        // Every flag a program can set is a bool, and the string, int and
        // float flags below are retired: their rows pin the refusal, which
        // names why, and the coercions they would reach are the container's
        // alone.
        // Strings: an atom and a string coerce, an int does not.
        "(config :hypgen-scoring most-constrained)",
        "(config :hypgen-scoring \"popularity\")",
        "(config :hypgen-scoring 7)",
        "(config :lattice-order score-sum)",
        // Ints: literals, strings, and the coercion's tolerances.
        "(config :candidate-order-seed 7)",
        "(config :candidate-order-seed -1)",
        "(config :candidate-order-seed 007)",
        "(config :candidate-order-seed \"7\")",
        "(config :candidate-order-seed \" 7 \")",
        "(config :candidate-order-seed \"1_000\")",
        "(config :candidate-order-seed \"1_\")",
        "(config :candidate-order-seed \"1.5\")",
        "(config :candidate-order-seed true)",
        "(config :lattice-order-seed 3)",
        // Floats.
        "(config :hypgen-rel-weight 2)",
        "(config :hypgen-rel-weight \"1.5\")",
        "(config :hypgen-rel-weight \"1_0.5\")",
        "(config :hypgen-obj-weight \"nope\")",
        // The shape of the body itself, and the unknown-flag enumeration.
        "(config foo)",
        "(config :nope true)",
        "(config)",
        // Last one wins, and an earlier repeat of a key loses to a later.
        "(config :lattice-sanity-check true) (config :enable-path-nogoods false)",
        "(config :lattice-sanity-check true :lattice-sanity-check false)",
    ];
    let mut out = String::new();
    for body in bodies {
        out.push_str(&format!("=== {body}\n{}", indented(&load_shape_text(body))));
    }
    assert_eq!(bodies.len(), 32, "the table lost a coercion path");
    if let Some(msg) = golden(&golden_path("ein-ir", "config_coercion.txt"), &out) {
        panic!("{msg}");
    }
}

/// One answer, every line indented — so a `=== ` header always starts a row
/// and a multi-line diagnostic (a parse error carries its caret) cannot be
/// mistaken for the next body.
fn indented(answer: &Answer) -> String {
    answer
        .to_string()
        .lines()
        .map(|l| format!("  {l}\n"))
        .collect()
}

fn load_shape_text(text: &str) -> Answer {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = match ein_ir::parse(&mut ast, text, None) {
        Ok(forms) => forms,
        Err(e) => {
            return Answer::Err {
                kind: "IRParseError",
                msg: e.to_string(),
            };
        }
    };
    match ein_ir::load(&mut ast, &mut terms, &forms, None) {
        Ok(kb) => Answer::Ok(shape(&kb, &terms)),
        Err(e) => Answer::Err {
            kind: "KBLoadError",
            msg: e.0,
        },
    }
}

/// Errors **accumulate**, and the order is the pass order — macros, then
/// relations, then rules, then facts, then config, then the unimported-macro
/// guard, then the derivation-cycle check. A reordered `; `-joined message is
/// the failure mode this pins, and it is invisible in any single fixture:
/// each of the programs below declares its errors in an order that is *not*
/// the pass order, so a loader that reported them in encounter order would
/// produce plausible messages and fail here.
///
/// **The fact pass is absent from these programs because it cannot fail on
/// anything the grammar accepts.** Its one error — a non-atom head — is
/// reachable only by hand-building an AST, since `(?R a b)` is a parse
/// error. Facts either parse or load.
///
/// **The first four programs are parse errors**: `(relation)`, `(relation
/// eq)`, `(relation r (A B))`, a rule or a macro named `absent` and an
/// unknown `(config …)` flag are the grammar's, and the parse reports every
/// one recovery reaches before the loader runs. The fifth holds the pass order with
/// errors the loader still owns: a rule with no `:assert`, a relation declared
/// twice, a macro declared twice and a derivation cycle.
#[test]
fn load_errors_accumulate_in_pass_order() {
    let programs = [
        // One error per pass, declared in an order that is not the pass order.
        "(rule r () :match (x ?a))\n(relation)\n\
         (macro m (?a) (rel ?a))\n(macro m (?a) (other ?a))\n(config :nope true)",
        // Several within one pass keep their form order.
        "(relation)\n(relation eq)\n(relation dup)\n(relation dup)\n(relation r (A B))",
        // A rule error does not stop the following rules from loading.
        "(rule absent () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (y ?a))\n\
         (rule ok () :match (x ?a) :assert (z ?a))",
        // A macro rejected for its name is *not* registered, so the rule that
        // invokes it keeps the unexpanded form rather than expanding it.
        "(macro absent (?p) (rel ?p))\n(rule r () :match (absent ?a) :assert (y ?a))",
        // One loader error per pass.
        // Each form carries its `:why`, which the default requires, so the
        // loader's errors are the only lines.
        "(r a b :rule x :using ((r c d)))\n(r c d :rule x :using ((r a b)))\n\
         (rule r () :match (x ?a) :why \"x holds of {?a}, and with no :assert the rule concludes nothing\")\n\
         (relation dup :why \"dup holds\")\n(relation dup :why \"dup holds\")\n\
         (macro m (?a) (rel ?a) :why \"rel holds of {?a}\")\n(macro m (?a) (other ?a) :why \"other holds of {?a}\")",
    ];
    let mut out = String::new();
    for program in programs {
        out.push_str(&format!(
            "=== {}\n{}",
            program.replace('\n', " ⏎ "),
            indented(&load_shape_text(program))
        ));
    }
    assert_eq!(programs.len(), 5, "the table lost a program");
    if let Some(msg) = golden(&golden_path("ein-ir", "load_error_order.txt"), &out) {
        panic!("{msg}");
    }
}

// ── `:why` required at load ────────────────────────────────────────────

/// One program's load, under `require_why` or not, from text.
fn load_text_with(text: &str, require_why: bool) -> Result<String, String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, text, None).map_err(|e| e.to_string())?;
    let opts = ein_ir::LoadOptions { require_why };
    ein_ir::load_with(&mut ast, &mut terms, &forms, None, &opts)
        .map(|kb| shape(&kb, &terms))
        .map_err(|e| e.0)
}

/// The line the loader refuses a form with.
fn missing(head: &str, name: &str) -> String {
    format!(
        "{head} '{name}' has no :why \"…\", which this load requires of every {}",
        ein_ir::WHY_REQUIRED_OF
    )
}

/// **With `require_why` on, a relation, a rule and an hrule without a `:why`
/// are each refused with one line naming the form; with it off, each loads**
/// (Q-M1f.62): the grammar takes a `:why` as optional, and the loader's flag
/// requires it. A `:why` that is not a string is no template; the grammar
/// refuses it at the parse.
#[test]
fn require_why_refuses_each_form_without_one() {
    // Every form carries its `:why`; each case takes one away.
    let whole = [
        "(relation owns Person Thing :why \"{?1} owns {?2}\")",
        "(relation has Person Thing :why \"{?1} has {?2}\")",
        "(rule keep () :match (owns ?p ?x) :assert (has ?p ?x) :why \"{?p} keeps {?x}\")",
        "(hrule guess () :match (has ?p ?x) :assert (owns ?p ?x) :why \"suppose {?p} owns {?x}\")",
        "(owns Ann box)",
    ];
    let with = whole.join("\n");
    let loaded = load_text_with(&with, true).expect("every form carries its :why");
    assert_eq!(Ok(loaded), load_text_with(&with, false));

    let cases = [
        (0, "(relation owns Person Thing)", "relation", "owns"),
        (
            2,
            "(rule keep () :match (owns ?p ?x) :assert (has ?p ?x))",
            "rule",
            "keep",
        ),
        (
            3,
            "(hrule guess () :match (has ?p ?x) :assert (owns ?p ?x))",
            "hrule",
            "guess",
        ),
    ];
    // A `:why` that is no string is no fourth case: the grammar refuses it at
    // the parse.
    assert!(
        ein_ir::parse(
            &mut Ast::new(),
            "(relation has Person Thing :why has)",
            None
        )
        .is_err()
    );
    for (at, without, head, name) in cases {
        let mut forms = whole;
        forms[at] = without;
        let program = forms.join("\n");
        assert_eq!(
            load_text_with(&program, true),
            Err(missing(head, name)),
            "{without}"
        );
        assert!(load_text_with(&program, false).is_ok(), "{without}");
    }

    // One line per form, in pass order, beside the loader's other refusals.
    let none = "(rule keep () :match (owns ?p ?x) :assert (has ?p ?x))\n\
                (relation owns Person Thing)\n(relation owns Person Thing :why \"o\")\n\
                (hrule guess () :match (has ?p ?x) :assert (owns ?p ?x))";
    let refused = load_text_with(none, true).expect_err("refused");
    let lines: Vec<&str> = refused.split("; ").collect();
    assert_eq!(lines.len(), 4, "{refused}");
    assert_eq!(lines[0], missing("relation", "owns"));
    assert!(
        lines[1].starts_with("duplicate relation 'owns' at "),
        "{refused}"
    );
    assert_eq!(
        lines[2..],
        [missing("rule", "keep"), missing("hrule", "guess")]
    );
}

/// **A macro is asked for its `:why`**, as a relation, a rule and an hrule
/// are (Q-M1f.59).
#[test]
fn a_macro_needs_a_why_only_where_its_parser_reads_one() {
    let load = |text: &str| {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = ein_ir::parse(&mut ast, text, None).expect("parses");
        let opts = ein_ir::LoadOptions { require_why: true };
        ein_ir::load_with(&mut ast, &mut terms, &forms, None, &opts)
            .map(|_| ())
            .map_err(|e| e.0)
    };
    let with = "(macro owns (?p ?x) (has ?p ?x) :why \"{?p} owns {?x}\")";
    let without = "(macro owns (?p ?x) (has ?p ?x))";
    assert_eq!(load(with), Ok(()));
    assert_eq!(load(without), Err(missing("macro", "owns")));
}

/// **The default requires it**: `load`, the
/// entry with no options, refuses a declaration with no `:why`, and
/// `require_why: false`, the CLI's `--no-require-why`, loads it (Q-M1f.62).
#[test]
fn the_default_requires_a_why_and_false_loads_without() {
    assert!(ein_ir::LoadOptions::default().require_why);
    let text = "(relation p T)\n(p a)";
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, text, Some("default.ein")).expect("parses");
    let refused = ein_ir::load(&mut ast, &mut terms, &forms, None)
        .expect_err("the default refuses a relation with no :why");
    assert!(
        refused.0.contains("relation 'p' has no :why"),
        "{}",
        refused.0
    );
    assert!(load_text_with(text, false).is_ok());
}

/// **`require_why` decides only whether a program loads** (Q-M1f.62). Over
/// every corpus program: one that does not load without the flag does not
/// load with it; one the flag refuses is refused by `has no :why` lines and
/// nothing else; and one it lets through loads to the same KB, registries,
/// facts and indexes, as without it. The stdlib's modules are all let
/// through: every relation, rule and macro of them carries its `:why`.
///
/// **And it refuses none**: every corpus program carries its `:why`s
/// (Q-M1f.57), so every program that loads without the flag loads with it,
/// and the flag can be the default. A program written without a `:why` fails
/// here, by name.
#[test]
fn require_why_decides_loading_only() {
    let (mut same, mut refused) = (0, 0);
    // A file a unit on the corpus sweeps' pool; a unit's
    // `(same, refused)`, summed in order.
    let files = corpus_files();
    let found = ein_corpus::pool::map(
        "load_semantics::require_why",
        &files,
        |p| {
            (
                p.strip_prefix(repo_root())
                    .unwrap_or(p)
                    .display()
                    .to_string(),
                String::new(),
            )
        },
        |path| -> (usize, usize) {
            let (mut same, mut refused) = (0usize, 0usize);
            let text = std::fs::read_to_string(path).expect("reads");
            let load = |require_why: bool| {
                let mut ast = Ast::new();
                let mut terms = Terms::new();
                let forms =
                    ein_ir::parse(&mut ast, &text, path.to_str()).map_err(|e| e.to_string())?;
                let opts = ein_ir::LoadOptions { require_why };
                ein_ir::load_with(&mut ast, &mut terms, &forms, path.parent(), &opts)
                    .map(|kb| shape(&kb, &terms))
                    .map_err(|e| e.0)
            };
            let rel = path.strip_prefix(repo_root()).unwrap_or(path).display();
            let Ok(without) = load(false) else {
                assert!(load(true).is_err(), "{rel}");
                return (same, refused);
            };
            match load(true) {
                Ok(with) => {
                    assert!(with == without, "{rel}: the flag moved the KB it loads");
                    same += 1;
                }
                Err(e) => {
                    for line in e.split("; ") {
                        assert!(
                            line.contains(" has no :why \"…\", which this load requires of every "),
                            "{rel}: the flag refused with {line}"
                        );
                    }
                    eprintln!("{rel}: {e}");
                    refused += 1;
                }
            }
            if path.starts_with(repo_root().join("stdlib")) {
                assert!(load(true).is_ok(), "{rel}: a stdlib module is refused");
            }
            (same, refused)
        },
    );
    for (s, r) in found {
        same += s;
        refused += r;
    }
    // Every corpus program carries its `:why`s (Q-M1f.57), and the sweep
    // is looking at something.
    assert!(
        same > 300 && refused == 0,
        "{same} let through, {refused} refused: a corpus program lacks a :why"
    );
    eprintln!("{same} let through, {refused} refused");
}

// ── A template names every variable it can reach ──

/// **A template that writes a reference to a variable holding `}` is refused,
/// and one that reaches its variables loads** (W37, Q-M1f.11).
///
/// A reference reads a variable's name up to its first `}`
/// (`ein_core::why::render_why`), and `}` is one of the name rule's
/// characters, so `?a}b` is a variable no reference can name: `{?a}b}` is the
/// reference `{?a}` and the text `b}`. Each place a template names a scope's
/// variables, a rule's and an hrule's `:why`, a macro's and a query's
/// `:goal-text`, is refused with one line where it writes one; the same
/// program loads with a reference to a variable it can reach (`?n*`, W37's),
/// or with `?a}b` named by no reference, and a reference to no variable at
/// all, `{?a}`, stays text as before.
#[test]
fn a_template_naming_a_variable_no_reference_reaches_is_refused() {
    let pre = "(relation p T) (relation q T)\n(p a)\n";
    let line = |what: &str, clause: &str| {
        format!(
            "{what}: its {clause} writes {{?a}}b}}, a reference to ?a}}b, which no reference \
             can name — a reference reads a variable's name up to its first }}"
        )
    };
    for (form, what, clause) in [
        (
            "(rule r () :match (p ?a}b) :assert (q ?a}b) :why \"{?a}b} is a q\")",
            "rule 'r'",
            ":why",
        ),
        (
            "(hrule h () :match (p ?a}b) :assert (q ?a}b) :why \"guess {?a}b}\")",
            "hrule 'h'",
            ":why",
        ),
        (
            "(macro m (?a}b) (p ?a}b) :why \"{?a}b} is a p\")",
            "macro 'm'",
            ":why",
        ),
        (
            "(query :goal (p ?a}b) :goal-text \"{?a}b} is a p\")",
            "query",
            ":goal-text",
        ),
    ] {
        let text = format!("{pre}{form}\n");
        let refused = load_text_with(&text, false).expect_err(form);
        assert!(
            refused.contains(&line(what, clause)),
            "{form}: no line {:?} in\n{refused}",
            line(what, clause)
        );
        for reaching in [
            form.replace("{?a}b}", "{?a}"),
            form.replace("?a}b", "?n*"),
            form.replace("{?a}b}", "it"),
        ] {
            load_text_with(&format!("{pre}{reaching}\n"), false)
                .unwrap_or_else(|e| panic!("{reaching}: {e}"));
        }
    }
}
