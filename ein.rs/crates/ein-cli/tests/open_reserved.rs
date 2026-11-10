//! **`open` is written only as a rule's verdict** — M1f S1f.5.23, on the
//! user's ruling of 2026-11-10 on the close review's finding 3 and on
//! Q-M1f.66 (*"Refuse it"*): `open` is the verdict a rule's `:assert` states,
//! `(open)` or `(open ?R)`, and is written nowhere else.
//!
//! Until S1f.5.23 `open` was a word a position gives a meaning to, a name
//! wherever a name goes (`ein.y`'s `Name`, `FactHead` and `ArgHead`), and the
//! loader refused it only in a rule's `:match`, inside a conclusion and beside
//! one (`validate_open`). So `(p (open x))` loaded (finding 3), and so did
//! `open` as a fact's relation, a stored negation's, a goal's or a claim's, an
//! activator binding a relation parameter to it, and `open` as a value
//! anywhere, which made it a name of the program the blind enumerator proposes
//! facts about (W70, W71); and a macro's body stated the verdict for a rule
//! (W72). Since then `open` is one of the words that are never a name
//! (`ein_ir::grammar::RESERVED_WORDS`), and `ein.y`'s `Assert` is the one
//! production that takes it, so every other place is refused at the parse, at
//! the `open`, with a note saying where it is written.
//!
//! Every place a program can write `open`, each on every command that reads a
//! program and every path that runs a rule: `saturate`, every `solve` mode,
//! `--events`, `oracle`, `test` with a claim, the views, `kb save` and so a
//! container. Each refuses with one first line at the `open`, the same on
//! every command, the note under it, and writes no container. The verdict
//! where a rule's or an hrule's `:assert` states it loads and every command
//! answers it; and a container that stores a program writing `open` as a name,
//! as one written before S1f.5.23 could, is refused where it is read.

#![cfg(feature = "einb")]

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

const HEAD: &str = "(relation a Thing :why \"{?1} is an a\")\n\
                    (relation p Thing :why \"{?1} is a p\")\n\
                    (relation q Thing :why \"{?1} is a q\")\n\
                    (is-a k Thing)\n\
                    (a k)\n";

/// A claim `ein test` checks, which holds of every program here that loads.
const CLAIM: &str = "(query :goal (is-a ?x Thing) :expect (model (is-a k Thing)))\n";

const LIKES: &str = "(relation likes Thing Thing :why \"{?1} likes {?2}\")\n";
const COPY: &str = "(rule copy (?R) :match (?R ?x) :assert (q ?x) :why \"{?x} is in {?R}\")\n";
const OWE: &str = "(macro owe (?R) (open ?R) :why \"one {?R} is owed\")\n";
const PP: &str = "(macro pp (?v) (p ?v) :why \"{?v} is a p\")\n";

/// A rule `r` over `m` and `a`.
fn rule(m: &str, a: &str) -> String {
    format!("(rule r () :match {m} :assert {a} :why \"r\")\n")
}

/// `(place, the program after HEAD)`: every place a program can write `open`
/// but a rule's whole `:assert`. The census of T1f.5.23.2, each place's
/// program as the census wrote it.
fn places() -> Vec<(&'static str, String)> {
    vec![
        // ── as a relation's name ──
        ("a fact's relation", "(open x)\n".into()),
        ("a nested fact's relation", "(p (open x))\n".into()),
        ("a stored negation's relation", "(not (open k))\n".into()),
        (
            "a nested negation's relation",
            "(p (not (open k)))\n".into(),
        ),
        (
            "a fact's :using",
            "(p k :rule copy :using ((open k)))\n".into(),
        ),
        (
            "a pattern's relation in a premise",
            rule("(and (a ?x) (p (open ?x)))", "(q ?x)"),
        ),
        (
            "the pattern a premise's not negates",
            rule("(and (a ?x) (not (open ?x)))", "(q ?x)"),
        ),
        (
            "a pattern's relation under an absent",
            rule("(and (a ?x) (absent (p (open ?x))))", "(q ?x)"),
        ),
        (
            "a guard's value, a pattern",
            rule("(and (a ?x) (neq ?x (open k)))", "(q ?x)"),
        ),
        (
            "a conclusion's argument, a pattern",
            rule("(a ?x)", "(q (open ?x))"),
        ),
        (
            "an hrule's conclusion argument, a pattern",
            "(hrule r () :match (a ?x) :assert (p (open ?x)) :why \"r\")\n".into(),
        ),
        ("a goal's pattern", "(query :goal (p (open ?x)))\n".into()),
        (
            "a claim's fact",
            "(query :goal (a ?x) :expect (model (a k) (open k)))\n".into(),
        ),
        (
            "a claim's nested fact",
            "(query :goal (p ?x) :expect (model (p (open k))))\n".into(),
        ),
        (
            "a region claim's fact",
            "(solve :expect (region (model (a k) (open k))))\n".into(),
        ),
        (
            "an activator's argument and a fact's relation",
            format!("{COPY}(copy open)\n(open k)\n"),
        ),
        ("an activator's argument", format!("{COPY}(copy open)\n")),
        (
            "a derived activator's argument",
            format!("{COPY}{}", rule("(a ?x)", "(copy open)")),
        ),
        (
            "an obligation's activator",
            "(rule r (?R) :match (and (a ?x) (absent (?R ?x ?y))) :assert (open ?R) :why \"r\")\n\
             (r open)\n"
                .into(),
        ),
        (
            "the verdict's argument",
            rule("(and (a ?x) (absent (q ?x)))", "(open open)"),
        ),
        // ── as a value ──
        ("a fact's argument", "(p open)\n".into()),
        ("a nested fact's argument", "(q (p open))\n".into()),
        (
            "a declaration's type",
            "(relation t open :why \"{?1} is a t\")\n".into(),
        ),
        ("an is-a's type", "(is-a j open)\n".into()),
        ("an is-a's object", "(is-a open Thing)\n".into()),
        (
            "a premise's argument",
            rule("(and (a ?x) (p open))", "(q ?x)"),
        ),
        (
            "an absent's argument",
            rule("(and (a ?x) (absent (p open)))", "(q ?x)"),
        ),
        (
            "a not's argument",
            rule("(and (a ?x) (not (p open)))", "(q ?x)"),
        ),
        (
            "a neq's value",
            rule("(and (a ?x) (neq ?x open))", "(q ?x)"),
        ),
        ("an eq's value", rule("(and (a ?x) (eq ?y open))", "(q ?y)")),
        ("a conclusion's argument", rule("(a ?x)", "(p open)")),
        (
            "an hrule's conclusion argument",
            "(hrule r () :match (a ?x) :assert (p open) :why \"r\")\n".into(),
        ),
        ("a goal's argument", "(query :goal (p open))\n".into()),
        (
            "a claim's argument",
            "(query :goal (p ?x) :expect (model (p open)))\n".into(),
        ),
        (
            "a claim's neq",
            "(query :goal (a ?x) :expect (model (a k) (neq k open)))\n".into(),
        ),
        (
            "a region claim's argument",
            "(solve :expect (region (model (a k) (p open))))\n".into(),
        ),
        (
            "a macro call's argument in a :match",
            format!("{PP}{}", rule("(and (a ?x) (pp open))", "(q ?x)")),
        ),
        (
            "a macro call's argument in an :assert",
            format!("{PP}{}", rule("(a ?x)", "(pp open)")),
        ),
        ("a fact's :rule", "(p k :rule open)\n".into()),
        (
            "a solve's :hypothesis-relations",
            "(solve :hypothesis-relations (open))\n".into(),
        ),
        (
            "a solve's :no-hypothesis",
            "(solve :no-hypothesis (open))\n".into(),
        ),
        ("a solve's :hrules", "(solve :hrules (open))\n".into()),
        (
            "a query's :over",
            "(query :goal (a ?x) :over open)\n".into(),
        ),
        ("an import's module", "(import open)\n".into()),
        (
            "an import's :symbols",
            "(import std.macro :symbols (open))\n".into(),
        ),
        (
            "a trace step's name",
            "(trace (step open :derives (a k)))\n".into(),
        ),
        (
            "a trace step's :rule",
            "(trace (step s1 :rule open :derives (a k)))\n".into(),
        ),
        (
            "a trace step's :derives",
            "(trace (step s1 :derives (open k)))\n".into(),
        ),
        // ── in a macro's body (Q-M1f.86) ──
        (
            "a macro's body, the verdict, called in an :assert",
            format!(
                "{LIKES}{OWE}{}",
                rule("(and (a ?x) (absent (likes ?x ?y)))", "(owe likes)")
            ),
        ),
        (
            "a macro's body, the verdict, called in a :match",
            format!("{OWE}{}", rule("(and (a ?x) (owe q))", "(q ?x)")),
        ),
        ("a macro's body no rule calls", OWE.into()),
        (
            "a macro's body, a value",
            format!(
                "(macro po (?v) (p open) :why \"open is a p\")\n{}",
                rule("(and (a ?x) (po ?x))", "(q ?x)")
            ),
        ),
    ]
}

/// Every command that reads a program, and the exit it gives one it refuses.
const COMMANDS: [(&[&str], i32); 20] = [
    (&["saturate"], 1),
    (&["saturate", "--dump"], 1),
    (&["solve"], 1),
    (&["solve", "--minimal"], 1),
    (&["solve", "--maximal"], 1),
    (&["solve", "--single"], 1),
    (&["solve", "--count"], 1),
    (&["solve", "-n", "1"], 1),
    (&["solve", "--events", "{events}"], 1),
    (&["oracle"], 1),
    (&["test"], 2),
    (&["test", "--events", "{events}"], 2),
    (&["render", "lattice"], 1),
    (&["render", "kb"], 1),
    (&["render", "program"], 1),
    (&["render", "rules"], 1),
    (&["render", "rule", "--name", "r"], 1),
    (&["render", "constraints"], 1),
    (&["kb", "save"], 1),
    (&["kb", "save", "--saturate"], 1),
];

fn argv(cmd: &[&str], file: &Path, einb: &Path, events: &Path) -> Vec<String> {
    let mut args: Vec<String> = cmd
        .iter()
        .map(|a| match *a {
            "{events}" => events.display().to_string(),
            a => a.to_string(),
        })
        .collect();
    args.push(file.display().to_string());
    if cmd[0] == "kb" {
        args.push(einb.display().to_string());
    }
    args
}

/// The note every refusal at an `open` carries (`ein_ir::grammar::OPEN_NOTE`).
const NOTE: &str = "  note: open is reserved: it is the verdict a rule's :assert states, written \
                    as the whole :assert, (open) or (open ?R), and nowhere else, so it names no \
                    relation and no value";

/// `file` refused on every command at an `open`: one first line, the same on
/// each, `<file>:<line>:<col>: unexpected `open``, where the file writes
/// `open`, the note under it, no container written and nothing in the
/// `--events` log.
fn refused_everywhere(file: &Path, dir: &Path) -> Vec<String> {
    let mut bad = Vec::new();
    let text = std::fs::read_to_string(repo_root().join(file)).expect("read");
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let mut first: Option<String> = None;
    for (cmd, code) in COMMANDS {
        let _ = std::fs::remove_file(&events);
        let args = argv(cmd, file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein(&argv);
        let mut lines = r.err.lines();
        let line = lines.next().unwrap_or("").to_string();
        if r.code != code {
            bad.push(format!(
                "ein {argv:?}: exit {}, want {code}: {line}",
                r.code
            ));
            continue;
        }
        let at = line
            .strip_prefix(&format!("{}:", file.display()))
            .and_then(|l| l.strip_suffix(": unexpected `open`"))
            .and_then(|l| l.split_once(':'))
            .and_then(|(l, c)| Some((l.parse::<usize>().ok()?, c.parse::<usize>().ok()?)));
        let written = at.is_some_and(|(l, c)| {
            text.lines().nth(l - 1).is_some_and(|t| {
                t.chars()
                    .skip(c - 1)
                    .collect::<String>()
                    .starts_with("open")
            })
        });
        if !written {
            bad.push(format!(
                "ein {argv:?}: {line:?}, wants {}:<line>:<col>: unexpected `open`, at an `open`",
                file.display()
            ));
        }
        if lines.next() != Some(NOTE) {
            bad.push(format!("ein {argv:?}: no note under {line:?}"));
        }
        match &first {
            None => first = Some(line.clone()),
            Some(f) if *f != line => bad.push(format!(
                "ein {argv:?}: {line:?}, where the first command printed {f:?}"
            )),
            Some(_) => {}
        }
        if einb.exists() {
            bad.push(format!(
                "ein {argv:?} wrote a container of a refused program"
            ));
            let _ = std::fs::remove_file(&einb);
        }
        if std::fs::read_to_string(&events).is_ok_and(|t| t.contains("\"e\": \"fire\"")) {
            bad.push(format!("ein {argv:?} fired a rule"));
        }
    }
    bad
}

#[test]
fn open_is_refused_wherever_it_is_not_the_verdict() {
    let scratch = Scratch::new("ein-open-reserved");
    let mut bad = Vec::new();
    let places = places();
    for (i, (place, body)) in places.iter().enumerate() {
        let file = scratch.path().join(format!("place-{i:02}.ein"));
        let claim = if body.contains("(query") { "" } else { CLAIM };
        std::fs::write(&file, format!("{HEAD}{body}{claim}")).expect("written");
        for b in refused_everywhere(&file, scratch.path()) {
            bad.push(format!("{place}: {b}"));
        }
    }
    assert_eq!(places.len(), 52, "every place the census wrote");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's twenty-four witnesses, W70 to W72, each read from its file,
/// as `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-open-witnesses");
    let dir = Path::new("examples/ein-bugs/wrong-answers");
    let mut witnesses: Vec<_> = std::fs::read_dir(repo_root().join(dir))
        .expect("the witnesses")
        .filter_map(|e| {
            let name = e.ok()?.file_name().to_str()?.to_string();
            (name.starts_with("open-") && name.ends_with(".ein")).then(|| dir.join(name))
        })
        .collect();
    witnesses.sort();
    assert_eq!(
        witnesses.len(),
        24,
        "W70's seven, W71's fourteen and W72's three"
    );
    let mut bad = Vec::new();
    for w in &witnesses {
        for b in refused_everywhere(w, scratch.path()) {
            bad.push(format!("{}: {b}", w.display()));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The verdict loads where a rule's or an hrule's `:assert` states it**,
/// `(open)`, `(open ?R)` with `?R` a parameter and `(open likes)` naming a
/// relation, and every command answers it, its container too.
#[test]
fn the_verdict_loads_where_a_rule_concludes_it() {
    let scratch = Scratch::new("ein-open-verdict");
    let owed = "(likes k k)\n";
    for (form, rule) in [
        (
            "(open)",
            "(rule r () :match (and (a ?x) (absent (likes ?x ?y))) :assert (open) :why \"r\")\n",
        ),
        (
            "(open likes)",
            "(rule r () :match (and (a ?x) (absent (likes ?x ?y))) :assert (open likes) :why \"r\")\n",
        ),
        (
            "(open ?R)",
            "(rule r (?R) :match (and (a ?x) (absent (?R ?x ?y))) :assert (open ?R) :why \"r\")\n\
             (r likes)\n",
        ),
        (
            "(open likes), an hrule's",
            "(hrule r () :match (and (a ?x) (absent (likes ?x ?y))) :assert (open likes) :why \"r\")\n",
        ),
    ] {
        let file = scratch.path().join("verdict.ein");
        std::fs::write(&file, format!("{HEAD}{LIKES}{owed}{rule}{CLAIM}")).expect("written");
        let einb = scratch.path().join("verdict.einb");
        let events = scratch.path().join("events.jsonl");
        let _ = std::fs::remove_file(&einb);
        for (cmd, _) in COMMANDS {
            let args = argv(cmd, &file, &einb, &events);
            let argv: Vec<&str> = args.iter().map(String::as_str).collect();
            let r = ein(&argv);
            assert_eq!(r.code, 0, "{form}: ein {argv:?}: {}", r.err);
        }
        assert!(einb.exists(), "{form}: kb save wrote no container");
        let einb = einb.display().to_string();
        for cmd in ["saturate", "solve", "oracle", "test"] {
            let r = ein(&[cmd, &einb]);
            assert_eq!(r.code, 0, "{form}: ein {cmd} the container: {}", r.err);
        }
    }
}

/// **A container whose stored program writes `open` as a name is refused
/// where it is read**, as one `ein kb save` wrote before M1f S1f.5.23 could
/// be: the stored program is read back through the parser (Q-M1f.15), which
/// refuses it at the `open`. Written through `ein_einb::save`, the stored
/// program a tree the parser no longer builds, `(p open)`.
#[test]
fn a_container_that_stores_open_as_a_name_is_refused() {
    let scratch = Scratch::new("ein-open-container");
    let dir = scratch.path();
    let mut ast = ein_ir::Ast::new();
    let mut terms = ein_core::Terms::new();
    let held = ein_ir::parse(
        &mut ast,
        "(relation p Thing :why \"{?1} is a p\") (is-a k Thing)",
        Some("k.ein"),
    )
    .expect("it parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &held, None).expect("it loads");
    let mut forms = held.clone();
    let p = ast.atom("p", None);
    let open = ast.atom("open", None);
    forms.push(ast.sform(p, &[open], None));
    ein_einb::save(
        &dir.join("o.einb"),
        &kb,
        &terms,
        &mut ast,
        &forms,
        Some(dir),
        &ein_einb::SaveOptions::default(),
    )
    .expect("a container is written");
    let einb = dir.join("o.einb").display().to_string();
    for cmd in ["saturate", "solve", "oracle", "render"] {
        let args: Vec<&str> = if cmd == "render" {
            vec!["render", "kb", &einb]
        } else {
            vec![cmd, &einb]
        };
        let r = ein(&args);
        assert_eq!(r.code, 1, "{args:?}: {}", r.err);
        assert!(
            r.err.starts_with(&format!(
                "kb load error: .einb PROGRAM does not load: <program stored in {einb}>:"
            )) && r.err.contains(": unexpected `open`"),
            "{args:?}: {}",
            r.err
        );
    }
}
