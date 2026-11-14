//! **What a file writes, stated at load** — M1f S1f.6.38, E1, the user's ruling
//! of 2026-11-14 on Q-M1f.272 (`defined_behaviour.md` § 4.30).
//!
//! The loader states every fact nested in a fact a file writes, and the ones a
//! rule's or an hrule's conclusion holds where they hold no variable
//! (Q-M1f.275 (a)), as `(written T)`, the kernel relation a representation
//! module reads to check its own terms where they are written. Each is derived
//! by the loader's step `#written` from the fact it was read in, so a core that
//! rests on one names the fact as written.
//!
//! | claim | where |
//! |---|---|
//! | what is stated, once a value, the outer before the inner, from a fact, a negation, a module's fact and a rule's conclusion, and from nothing else | [`what_the_loader_states`] |
//! | a program does not write `written`: a fact of it, a negation of one and a conclusion of one are refused at load, on every command, and no declarator binds it | [`a_program_does_not_write_written`] |
//! | a core that rests on a stated fact names the fact as written; one stated from a rule's text rests on none | [`a_core_names_the_fact_as_written`] |
//! | a stated fact is no candidate and no name, and no read-out prints one: with no rule reading `written` a program answers as it did without them | [`the_answer_is_as_it_was`] |
//!
//! ```text
//! cargo test -p ein-cli --test written_facts
//! ```

use std::path::Path;
use std::process::Command;

use ein_corpus::Scratch;

/// What one run of `ein` left.
struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn run_in(dir: &Path, argv: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(argv)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// `ein <args> w.ein`, with `files` written in a fresh directory, `w.ein`
/// first.
fn ein(files: &[(&str, &str)], args: &[&str]) -> Run {
    let scratch = Scratch::new("ein-written-facts");
    let dir = scratch.path();
    for (name, text) in files {
        std::fs::write(dir.join(name), text).expect("the program is written");
    }
    let mut argv: Vec<&str> = args.to_vec();
    argv.push("w.ein");
    run_in(dir, &argv)
}

/// The `(written …)` lines of `ein saturate --dump`, in its order.
fn stated(dump: &str) -> Vec<String> {
    dump.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("(written "))
        .map(str::to_string)
        .collect()
}

#[test]
fn what_the_loader_states() {
    let module = "(relation m Thing :why \"m {?1}\")\n(m (from-module a))\n";
    let program = "(import mod :symbols (m))\n\
         (relation p Thing :why \"p {?1}\")\n\
         (relation q Thing Thing :why \"q {?1} {?2}\")\n\
         (__closed__ p) (__closed__ q)\n\
         (p (q a (k b)))\n\
         (p (q a (k b)))\n\
         (not (p (q c (k d))))\n\
         (p (not (q e (k f))))\n\
         (eq (g h) (g h))\n\
         (q flat flat)\n\
         (rule r () :match (p ?x) :assert (and (s ?x (k b) (m (j e))) (s ?x (m ?x) (h (i f)))) \
           :why \"r\")\n\
         (hrule hr () :match (p ?x) :assert (t (u v)) :why \"hr\")\n";
    let run = ein(
        &[("w.ein", program), ("mod.ein", module)],
        &["saturate", "--dump"],
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let step = " :rule \"#written\")";
    let want: Vec<String> = [
        // The module's fact comes first, as the import inlines it.
        "(from-module a)",
        // Each fact a file writes, once a value, the outer before the inner.
        "(q a (k b))",
        "(k b)",
        // A negation's denied fact is no term: what its arguments nest is.
        "(q c (k d))",
        "(k d)",
        // A negation nested in a fact is a term, and its denied fact is not.
        "(not (q e (k f)))",
        "(k f)",
        // A rule's conclusion, where a nested fact holds no variable; an
        // hrule's too. `(m ?x)` holds one and is stated by nothing.
        "(k b)",
        "(m (j e))",
        "(j e)",
        "(h (i f))",
        "(i f)",
        "(u v)",
    ]
    .iter()
    .map(|t| format!("(written {t}{step}"))
    .collect();
    // `(k b)` is stated once, from the fact that holds it first.
    let mut want_once: Vec<String> = Vec::new();
    for w in want {
        if !want_once.contains(&w) {
            want_once.push(w);
        }
    }
    let mut got = stated(&run.stdout);
    let mut sorted_got = got.clone();
    sorted_got.sort();
    let mut sorted_want = want_once.clone();
    sorted_want.sort();
    assert_eq!(
        sorted_got, sorted_want,
        "what the loader states\n{}",
        run.stdout
    );
    // The order: the facts a file writes first, in their order, then the
    // rules' conclusions in the registries' order.
    got.retain(|l| l.contains("(from-module") || l.contains("(q a") || l.contains("(u v)"));
    assert_eq!(
        got,
        [
            "(written (from-module a) :rule \"#written\")",
            "(written (q a (k b)) :rule \"#written\")",
            "(written (u v) :rule \"#written\")",
        ]
    );
    // The denied fact itself is stated by nothing.
    assert!(!run.stdout.contains("(written (p (q c"), "{}", run.stdout);
    // An `eq` fact is checked and never stored: what it nests is stated by
    // nothing.
    assert!(!run.stdout.contains("(written (g h)"), "{}", run.stdout);
}

#[test]
fn a_program_does_not_write_written() {
    let decls = "(relation p Thing :why \"p {?1}\")\n";
    let tail = " — written is the loader's: it states (written T) for each fact nested in a fact \
                a file writes, and a program does not write it";
    for (body, at, says) in [
        ("(written (p a))\n", "w.ein:2:", "(written (p a))"),
        (
            "(not (written (p a)))\n",
            "w.ein:2:",
            "(not (written (p a)))",
        ),
        (
            "(rule r () :match (p ?x) :assert (and (q ?x) (written (p ?x))) :why \"r\")\n",
            "w.ein:2:",
            "rule 'r': its :assert writes (written (p ?x))",
        ),
        (
            "(hrule h () :match () :assert (written (p a)) :why \"h\")\n",
            "w.ein:2:",
            "hrule 'h': its :assert writes (written (p a))",
        ),
    ] {
        let text = format!("{decls}{body}");
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
            let run = ein(&[("w.ein", &text)], args);
            let code = if args[0] == "test" { 2 } else { 1 };
            assert_eq!(run.code, code, "{body}: `ein {}`", args.join(" "));
            let first = run.stderr.lines().next().unwrap_or("");
            assert!(
                first.starts_with(&format!("kb load error: {at}"))
                    && first.ends_with(&format!("{says}{tail}")),
                "{body}: `ein {}`'s first line: {first}",
                args.join(" ")
            );
        }
    }
    // No declarator binds it.
    let run = ein(
        &[("w.ein", "(relation written T :why \"w {?1}\")\n")],
        &["solve"],
    );
    assert_eq!(run.code, 1);
    assert!(
        run.stderr
            .contains("relation 'written' shadows a reserved kernel name"),
        "{}",
        run.stderr
    );
    // A rule reads it, and so does a goal.
    let run = ein(
        &[(
            "w.ein",
            "(relation p Thing :why \"p {?1}\")\n(__closed__ p)\n(p (q a))\n\
             (rule seen () :match (written (q ?x)) :assert (r ?x) :why \"{?x} is seen\")\n\
             (query :goal (written (q ?x)) :expect (model (written (q a))))\n",
        )],
        &["solve"],
    );
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}

#[test]
fn a_core_names_the_fact_as_written() {
    let program = "(relation p Thing :why \"p {?1}\")\n(__closed__ p)\n\
                   (p (bad a))\n\
                   (rule no-bad () :match (written (bad ?x)) :assert (false) \
                     :why \"{?x} is written bad\")\n";
    let run = ein(&[("w.ein", program)], &["solve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let core: Vec<&str> = run
        .stdout
        .lines()
        .filter_map(|l| l.trim().strip_prefix("unsat core"))
        .map(str::trim)
        .collect();
    assert_eq!(core, ["(p (bad a))"], "{}", run.stdout);
    assert!(
        run.stdout
            .lines()
            .any(|l| l.trim() == "refuted by        no-bad"),
        "{}",
        run.stdout
    );
    // Stated from a rule's text, which no fact holds: the core is ∅.
    let program = "(relation p Thing :why \"p {?1}\")\n(__closed__ p)\n\
                   (relation s Thing :why \"s {?1}\")\n(__closed__ s)\n\
                   (rule never () :match (s ?x) :assert (p (pair (bad ?x) (bad b))) :why \"never\")\n\
                   (rule no-bad () :match (written (bad ?x)) :assert (false) \
                     :why \"{?x} is written bad\")\n";
    let run = ein(&[("w.ein", program)], &["solve"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert!(
        run.stdout
            .lines()
            .any(|l| l.trim().starts_with("unsat core") && l.contains('∅')),
        "{}",
        run.stdout
    );
}

#[test]
fn the_answer_is_as_it_was() {
    // A program whose nested fact nothing reads answers as it did before
    // S1f.6.38: 16 states over 4 candidates, no `written` fact among them
    // (the release binary of `8340baf7` answers the same, 2026-11-14). No
    // read-out names `written`.
    let nested = "(relation p Thing :why \"p {?1}\")\n(relation q Thing :why \"q {?1}\")\n\
                  (p (box a)) (q b)\n\
                  (rule r () :match (p ?x) :assert (q c) :why \"r\")\n\
                  (query :goal (q ?x))\n";
    let run = ein(&[("w.ein", nested)], &["solve", "--count"]);
    assert_eq!(run.code, 0, "{}", run.stderr);
    for want in ["states (k_s)      16", "universe          4 candidates"] {
        assert!(run.stdout.contains(want), "{want}\n{}", run.stdout);
    }
    for args in [&["solve"][..], &["solve", "--maximal"], &["oracle"]] {
        let run = ein(&[("w.ein", nested)], args);
        assert_eq!(run.code, 0, "{}", run.stderr);
        assert!(
            !run.stdout.contains("written"),
            "`ein {}` prints a stated fact:\n{}",
            args.join(" "),
            run.stdout
        );
    }
}
