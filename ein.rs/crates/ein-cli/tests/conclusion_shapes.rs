//! **What a rule may conclude** — M1f S1f.6.37, E-63 and W94, the user's
//! question of 2026-11-14: *"The rule can assert some arbitrary complext fact
//! built from simple facts with `and` and `or` - how engine handles this?"*
//!
//! A rule concludes **a conjunction of literals** (facts and stored negations),
//! `(false)`, a check (`eq`, `neq`, `(not (eq …))`), `()` or an obligation,
//! and nothing else: no disjunction, and no compound fact. An `(and …)` is read
//! flat at any depth, as a premise is (`ein_ir::Ast::conjuncts`; the user's
//! ruling of 2026-11-14, *"ok, flatten is fine, continue"*), so `(and (r a)
//! (and (p a) (q a)))` concludes what `(and (r a) (p a) (q a))` concludes, and
//! no fact is ever headed `and`. Until S1f.6.37 the compiler split the top
//! `(and …)` alone and stored the inner one as one fact headed `and` (W94).
//!
//! Each row of the stage file's table is pinned, here or where the row's kind of
//! answer is pinned:
//!
//! | the conclusion | where |
//! |---|---|
//! | a fact, a stored negation, an `(and …)` of them, an `and` inside an `and`, `(false)`, a check, `()`, `(open)` | [`every_shape_a_rule_concludes`], through `ein saturate --dump` and `ein solve` |
//! | an `and` inside an `and`, written and through a macro | [`an_and_inside_an_and_is_read_flat`], [`a_macros_and_inside_an_and_is_read_flat`], [`the_witnesses_answer_as_their_flat_twins`]: the same answer, facts and read-out as the flat conclusion (the user's rule of 2026-11-14, *verify by invariance*) |
//! | `(or …)`, an `(or …)` inside an `(and …)`, a connective as a conclusion's argument or under its `not` | refused at the parse: the grammar's table, `ein-ir/tests/grammar_pages.rs` (S1f.6.37's rows at its end), and [`no_disjunction_and_no_compound_fact_is_a_conclusion`] here |
//! | the same written by a macro's expansion | refused at load: `defined_behaviour.md` § 4.16, `ein-ir`'s `from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes` |
//! | an hrule's `(and …)` of more than one conjunct, written or through a macro | refused at load (Q-M1f.271's default, E-65, W95): `defined_behaviour.md` § 4.29, [`an_hrule_concludes_one_fact`], on every command |
//!
//! ```text
//! cargo test -p ein-cli --test conclusion_shapes
//! ```

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

/// Every program's declarations: four relations, each closed, so nothing is
/// guessed and R is root's alone.
const DECLS: &str = "(relation p Thing :why \"p {?1}\")\n\
                     (relation q Thing :why \"q {?1}\")\n\
                     (relation r Thing :why \"r {?1}\")\n\
                     (__closed__ p) (__closed__ q) (__closed__ r)\n";

/// One query per relation, so the read-out prints what root holds.
const QUERIES: &str = "(query :goal (p ?x))\n(query :goal (q ?x))\n(query :goal (r ?x))\n";

/// A program of one rule that fires once at root, concluding `assert`.
fn program(assert: &str) -> String {
    format!("{DECLS}(rule c () :match () :assert {assert} :why \"the shape\")\n{QUERIES}")
}

/// What one run of `ein` left: its exit code, stdout and stderr, and the
/// `--json-summary` a solve wrote.
struct Run {
    code: i32,
    stdout: String,
    stderr: String,
    summary: Option<serde_json::Value>,
}

/// `ein <args> FILE` with `text` written as `FILE` in a fresh directory, run
/// there, so that every program's read-out names the same path.
fn ein(text: &str, args: &[&str]) -> Run {
    let scratch = Scratch::new("ein-conclusion-shapes");
    let dir = scratch.path();
    std::fs::write(dir.join("c.ein"), text).expect("the program is written");
    let mut argv: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    argv.push("c.ein".to_string());
    if args[0] == "solve" {
        argv.push("--json-summary".to_string());
        argv.push("s.json".to_string());
    }
    run_in(dir, &argv)
}

fn run_in(dir: &Path, argv: &[String]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(argv)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    let summary = std::fs::read_to_string(dir.join("s.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("the summary parses"));
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
    }
}

/// Root's derived facts as `ein saturate --dump` lists them, in its order,
/// each with its `:rule` taken off.
fn derived(dump: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in dump.lines() {
        if line.starts_with(";; ── DERIVED") {
            inside = true;
            continue;
        }
        if inside {
            let l = line.trim();
            if l.is_empty() || (l.starts_with(";;") && !l.starts_with(";;   ")) {
                break;
            }
            if l.starts_with(";;") {
                continue;
            }
            out.push(match l.find(" :rule ") {
                Some(i) => format!("{})", &l[..i]),
                None => l.to_string(),
            });
        }
    }
    out
}

/// What root is, from a solve's summary: `alive`, `dead` or `owing`.
fn root(s: &serde_json::Value) -> &'static str {
    if s["root"]["dead"] == true {
        "dead"
    } else if s["root"]["owes"].as_u64().unwrap_or(0) > 0 {
        "owing"
    } else {
        "alive"
    }
}

/// The relations root holds a fact of, from a solve's summary.
fn root_relations(s: &serde_json::Value) -> BTreeSet<String> {
    s["root"]["facts_by_relation"]
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default()
}

/// **Every stored shape of the table**: what root derives and what root is.
#[test]
fn every_shape_a_rule_concludes() {
    // (conclusion, root's derived facts, root)
    let rows: &[(&str, &[&str], &str)] = &[
        ("(p a)", &["(p a)"], "alive"),
        ("(not (q a))", &["(not (q a))"], "alive"),
        (
            "(and (p a) (not (q a)))",
            &["(p a)", "(not (q a))"],
            "alive",
        ),
        (
            "(and (r a) (and (p a) (q a)))",
            &["(r a)", "(p a)", "(q a)"],
            "alive",
        ),
        (
            "(and (r a) (and (p a) (not (q a))))",
            &["(r a)", "(p a)", "(not (q a))"],
            "alive",
        ),
        // `(false)` refutes the state, and is stored as the refutation.
        ("(false)", &["(false)"], "dead"),
        ("(and (p a) (false))", &["(p a)", "(false)"], "dead"),
        // A check holds and stores nothing, or kills the state.
        ("(eq a a)", &[], "alive"),
        ("(eq a b)", &["(false)"], "dead"),
        ("(neq a b)", &[], "alive"),
        ("(not (eq a b))", &[], "alive"),
        (
            "(and (p a) (and (eq a a) (q a)))",
            &["(p a)", "(q a)"],
            "alive",
        ),
        // The empty conclusion, and the bare obligation, which root owes.
        ("()", &[], "alive"),
        ("(open)", &[], "owing"),
    ];
    for &(assert, facts, status) in rows {
        let text = program(assert);
        let dump = ein(&text, &["saturate", "--dump"]);
        assert_eq!(
            dump.code, 0,
            "{assert}: saturate --dump exits 0\n{}",
            dump.stderr
        );
        let mut got = derived(&dump.stdout);
        got.sort();
        let mut want: Vec<&str> = facts.to_vec();
        want.sort();
        assert_eq!(got, want, "{assert}: root's derived facts");
        let solve = ein(&text, &["solve"]);
        assert_eq!(solve.code, 0, "{assert}: solve exits 0\n{}", solve.stderr);
        let s = solve.summary.expect("solve writes its summary");
        assert_eq!(root(&s), status, "{assert}: root");
        assert!(
            !root_relations(&s).contains("and"),
            "{assert}: no fact is headed and"
        );
        let k = s["region"]["k_s"].as_u64();
        assert_eq!(
            k,
            Some(u64::from(status == "alive")),
            "{assert}: R is {{∅}} or empty"
        );
    }
}

/// An `(open ?R)` whose `?R` its activator binds: an obligation, which
/// derives nothing, and a state that owes it is out of R.
#[test]
fn an_obligation_concludes_nothing() {
    let text = "(relation item Node :why \"{?1} is an item\")\n\
                (relation tag Node Node :why \"{?1} has the tag {?2}\")\n\
                (item a)\n\
                (rule owed (?R) :match (and (relation ?R ?A ?B) (item ?a) (absent (?R ?a ?b))) \
                 :assert (open ?R) :why \"{?a} has no {?R} yet\")\n\
                (owed tag)\n";
    let dump = ein(text, &["saturate", "--dump"]);
    assert_eq!(dump.code, 0, "{}", dump.stderr);
    assert!(
        derived(&dump.stdout).is_empty(),
        "an obligation derives nothing"
    );
    let solve = ein(text, &["solve"]);
    let s = solve.summary.expect("solve writes its summary");
    assert_eq!(root(&s), "owing", "root owes the tag");
}

/// **`(and A (and B C))` concludes what `(and A B C)` concludes**: the same
/// facts, the same answer, the same read-out, in every way an `and` nests.
#[test]
fn an_and_inside_an_and_is_read_flat() {
    let flat = "(and (r a) (p a) (q a))";
    for nested in [
        "(and (r a) (and (p a) (q a)))",
        "(and (r a) (and (p a) (and (q a))))",
        "(and (and (r a) (p a)) (q a))",
        "(and (and (and (r a))) (and (p a) (q a)))",
    ] {
        same_answer(&program(flat), &program(nested), nested);
    }
    // In another order the facts are the same set, and the answer the same.
    let nested = "(and (and (p a) (q a)) (r a))";
    let a = ein(&program(flat), &["saturate", "--dump"]);
    let b = ein(&program(nested), &["saturate", "--dump"]);
    let set = |r: &Run| derived(&r.stdout).into_iter().collect::<BTreeSet<_>>();
    assert_eq!(set(&a), set(&b), "{nested}: the same facts");
    let (a, b) = (
        ein(&program(flat), &["solve"]),
        ein(&program(nested), &["solve"]),
    );
    assert_eq!(a.stdout, b.stdout, "{nested}: the same read-out");
}

/// **A macro's call that expands to an `(and …)`, written as one conjunct**,
/// concludes what the expansion written out does, and what the flat
/// conclusion does.
#[test]
fn a_macros_and_inside_an_and_is_read_flat() {
    let both = "(macro both (?x) (and (p ?x) (q ?x)) :why \"both p and q of {?x}\")\n";
    let called = format!("{both}{}", program("(and (r a) (both a))"));
    let deeper = format!(
        "{both}(macro all (?x) (and (r ?x) (both ?x)) :why \"all of {{?x}}\")\n{}",
        program("(all a)")
    );
    let written = format!("{both}{}", program("(and (r a) (and (p a) (q a)))"));
    let flat = format!("{both}{}", program("(and (r a) (p a) (q a))"));
    same_answer(&flat, &called, "a call inside an and");
    same_answer(&flat, &deeper, "a call of a macro whose body calls one");
    same_answer(&flat, &written, "the expansion written out");
}

/// **The register's two witnesses of W94 answer as their flat twins**, the
/// conclusion written `(and (r a) (p a) (q a))` and nothing else changed.
#[test]
fn the_witnesses_answer_as_their_flat_twins() {
    for (file, from, to) in [
        (
            "examples/ein-bugs/wrong-answers/and-in-and.ein",
            ":assert (and (r a) (and (p a) (q a)))",
            ":assert (and (r a) (p a) (q a))",
        ),
        (
            "examples/ein-bugs/wrong-answers/and-in-and-macro.ein",
            ":assert (and (r a) (both a))",
            ":assert (and (r a) (p a) (q a))",
        ),
    ] {
        let text = std::fs::read_to_string(repo_root().join(file)).expect("the witness reads");
        assert_eq!(text.matches(from).count(), 1, "{file} writes {from} once");
        let twin = text.replace(from, to);
        same_answer(&twin, &text, file);
        let solve = ein(&text, &["solve"]);
        assert_eq!(solve.code, 0, "{file}: both claims hold\n{}", solve.stdout);
    }
}

/// `a` and `b`, two texts of one program, give the same `ein saturate --dump`
/// (the derived facts in order, and every count), the same `ein solve`
/// read-out and stderr, and the same summary.
fn same_answer(a: &str, b: &str, what: &str) {
    let (da, db) = (
        ein(a, &["saturate", "--dump"]),
        ein(b, &["saturate", "--dump"]),
    );
    assert_eq!(da.code, db.code, "{what}: saturate's exit");
    assert_eq!(
        derived(&da.stdout),
        derived(&db.stdout),
        "{what}: the derived facts"
    );
    assert!(
        !derived(&db.stdout).iter().any(|f| f.starts_with("(and ")),
        "{what}: no fact is headed and"
    );
    let counts = |r: &Run| -> Vec<String> {
        r.stdout
            .lines()
            .skip_while(|l| !l.starts_with("── state AFTER"))
            .take_while(|l| !l.starts_with("====="))
            .filter(|l| !l.contains(" ms") && !l.starts_with("saturate:"))
            .map(str::to_string)
            .collect()
    };
    assert_eq!(
        counts(&da),
        counts(&db),
        "{what}: the state after saturation"
    );
    let (sa, sb) = (ein(a, &["solve"]), ein(b, &["solve"]));
    assert_eq!(sa.code, sb.code, "{what}: solve's exit");
    assert_eq!(sa.stdout, sb.stdout, "{what}: the read-out");
    assert_eq!(sa.stderr, sb.stderr, "{what}: stderr");
    let strip = |s: Option<serde_json::Value>| -> serde_json::Value {
        let mut s = s.expect("solve writes its summary");
        if let Some(o) = s.as_object_mut() {
            o.remove("work");
        }
        s
    };
    assert_eq!(strip(sa.summary), strip(sb.summary), "{what}: the summary");
}

/// **No disjunction and no compound fact is a conclusion**: each is refused at
/// the parse, one line, on the command that reads the program. The grammar's
/// table banks each report (`ein-ir/tests/grammar_pages.rs`).
#[test]
fn no_disjunction_and_no_compound_fact_is_a_conclusion() {
    for assert in [
        "(or (p a) (q a))",
        "(and (p a) (or (q a) (r a)))",
        "(and (r a) (and (p a) (or (q a) (r a))))",
        "(p (and (q a) (r a)))",
        "(p (or (q a) (r a)))",
        "(not (and (p a) (q a)))",
        "(not (or (p a) (q a)))",
    ] {
        let run = ein(&program(assert), &["saturate"]);
        assert_eq!(run.code, 1, "{assert}: refused\n{}", run.stdout);
        let first = run.stderr.lines().next().unwrap_or("");
        assert!(
            first.starts_with("c.ein:") && first.contains("unexpected"),
            "{assert}: refused at the parse, at the connective: {first}"
        );
    }
}

/// **An hrule proposes one fact per match**, so its conclusion, read flat, is
/// one conjunct, and one of more is refused at load, on every command, one
/// line placed where the `:assert` is written (M1f S1f.6.37, Q-M1f.271's
/// default, `defined_behaviour.md` § 4.29). Until then the hypothesis layer
/// proposed the first conjunct alone and dropped the others in silence (E-65,
/// W95). An `(and …)` of one conjunct is one conclusion, and loads.
#[test]
fn an_hrule_concludes_one_fact() {
    let both = "(macro both (?x) (and (p ?x) (q ?x)) :why \"both p and q of {?x}\")\n";
    let hrule = |assert: &str| {
        // Its relations open: an hrule proposing a closed one is refused too
        // (`defined_behaviour.md` § 4.27).
        let decls = DECLS.lines().take(3).collect::<Vec<_>>().join("\n");
        format!("{decls}\n{both}(hrule h () :match () :assert {assert} :why \"the guess\")\n")
    };
    let line = |written: &str, n: usize| {
        format!(
            "hrule 'h': its :assert writes {written}, {n} conclusions — an hrule proposes one \
             fact per match, each a commitment of its own: write one hrule per conclusion"
        )
    };
    for (assert, written, n) in [
        ("(and (p a) (q a))", "(and (p a) (q a))", 2),
        (
            "(and (and (p a) (q a)) (r a))",
            "(and (and (p a) (q a)) (r a))",
            3,
        ),
        ("(both a)", "(and (p a) (q a))", 2),
        ("(and (r a) (both a))", "(and (r a) (and (p a) (q a)))", 3),
    ] {
        let text = hrule(assert);
        let want = line(written, n);
        for args in [
            &["saturate"][..],
            &["saturate", "--dump"],
            &["solve"],
            &["solve", "--maximal"],
            &["solve", "--count"],
            &["oracle"],
            &["render", "rules"],
            &["render", "kb"],
            &["test"],
        ] {
            let run = ein(&text, args);
            let code = if args[0] == "test" { 2 } else { 1 };
            assert_eq!(
                run.code,
                code,
                "{assert}: `ein {}` refuses it",
                args.join(" ")
            );
            let first = run.stderr.lines().next().unwrap_or("");
            assert!(
                first.starts_with("kb load error: c.ein:") && first.ends_with(&want),
                "{assert}: `ein {}`'s first line: {first}",
                args.join(" ")
            );
        }
    }
    // One conjunct, written bare or in an `(and …)`, proposes its fact.
    for assert in ["(p a)", "(and (p a))", "(and (and (p a)))"] {
        let run = ein(&hrule(assert), &["solve"]);
        assert_eq!(run.code, 0, "{assert}: loads\n{}", run.stderr);
        let s = run.summary.expect("solve writes its summary");
        assert_eq!(
            s["region"]["k_s"].as_u64(),
            Some(2),
            "{assert}: R is ∅ and {{(p a)}}"
        );
    }
}
