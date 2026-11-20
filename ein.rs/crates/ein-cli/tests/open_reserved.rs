//! **`open` is the kernel's where it reads it, and a name elsewhere**
//! (Q-M1f.66): at the head of a written fact, a premise or a conclusion the
//! kernel reads what a state owes, `(open R)`, and a hypothesis, `(open F)`
//! (Q-M1f.566); everywhere else `open` is a name, and under an ordinary
//! relation the head of a fact the kernel does not read (Q-M1f.571).
//!
//! It stays refused where a program would declare it or make it more than
//! the kernel's, and in the shapes the kernel reads it in that it does not
//! take (Q-M1f.568): the bare `(open)`, `open` or `absent` directly under it,
//! `(absent (absent P))`, a written `(open x)` with `x` neither a fact nor a
//! relation, an obligation made to owe `open`, a declarator, a list or an
//! import that names it, and a claim's fact of it. Each place is refused on
//! every command that reads a program and every path that runs a rule,
//! `saturate`, every `solve` mode, `--events`, `oracle`, `test` with a claim,
//! the views, `kb save` and so a container, with one first line, the same on
//! every command, and no container written. Every other place loads; the
//! verdict and a hypothesis where a rule concludes one load and every command
//! answers them, their container too; and a container whose stored program
//! nests `open` under `open` is refused where it is read.

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

const HEAD: &str = "(relation a Thing :why \"{?3} is an a\")\n\
                    (relation p Thing :why \"{?3} is a p\")\n\
                    (relation q Thing :why \"{?3} is a q\")\n\
                    (is-a k Thing)\n\
                    (a k)\n";

/// A claim `ein test` checks, which holds of every program here that loads.
const CLAIM: &str = "(query :goal (is-a ?x Thing) :expect (model (is-a k Thing)))\n";

const LIKES: &str = "(relation likes Thing Thing :why \"{?3} likes {?4}\")\n";
const COPY: &str = "(rule copy (?R) :match (?R ?x) :assert (q ?x) :why \"{?x} is in {?R}\")\n";
const OWE: &str = "(macro owe (?R) (open ?R) :why \"one {?R} is owed\")\n";
const PP: &str = "(macro pp (?v) (p ?v) :why \"{?v} is a p\")\n";

/// A rule `r` over `m` and `a`.
fn rule(m: &str, a: &str) -> String {
    format!("(rule r () :match {m} :assert {a} :why \"r\")\n")
}

/// `(place, the program after HEAD)`: every place a program writes `open` in
/// and is refused, each place's program.
fn refused_places() -> Vec<(&'static str, String)> {
    vec![
        ("the bare (open), written", "(open)\n".into()),
        ("the bare (open), concluded", rule("(a ?x)", "(open)")),
        ("open under open, written", "(open (open (p k)))\n".into()),
        ("open under open, concluded", rule("(a ?x)", "(open (open (p ?x)))")),
        ("absent under open, written", "(open (absent (p k)))\n".into()),
        ("absent under open, concluded", rule("(a ?x)", "(open (absent (p ?x)))")),
        (
            "absent under absent",
            rule("(and (a ?x) (absent (absent (p ?x))))", "(q ?x)"),
        ),
        ("a written (open x), x a name", "(open x)\n".into()),
        (
            "an activator's argument and a written (open x)",
            format!("{COPY}(copy open)\n(open k)\n"),
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
        (
            "what is owed beside a conclusion",
            "(rule r (?R) :match (and (a ?x) (absent (?R ?x ?y))) :assert (and (open ?R) (q ?x)) :why \"r\")\n\
             (r likes)\n"
                .into(),
        ),
        (
            "a claim's fact",
            "(query :goal (a ?x) :expect (model (a k) (open k)))\n".into(),
        ),
        (
            "a region claim's fact",
            "(solve :expect (region (model (a k) (open k))))\n".into(),
        ),
        (
            "a relation's name",
            "(relation open T :why \"{?3} is open\")\n".into(),
        ),
        (
            "a rule's name",
            "(rule open () :match (a ?x) :assert (q ?x) :why \"r\")\n".into(),
        ),
        ("a macro's name", "(macro open (?x) (p ?x) :why \"m\")\n".into()),
        (
            "a solve's :hypothesis-relations",
            "(solve :hypothesis-relations (open))\n".into(),
        ),
        (
            "a solve's :no-hypothesis",
            "(solve :no-hypothesis (open))\n".into(),
        ),
        ("a solve's :hrules", "(solve :hrules (open))\n".into()),
        ("an import's module", "(import open)\n".into()),
        (
            "an import's :symbols",
            "(import std.macro :symbols (open))\n".into(),
        ),
    ]
}

/// `(place, the program after HEAD)`: every place a program writes `open` in
/// as a name, or under an ordinary relation, each of which loads.
fn named_places() -> Vec<(&'static str, String)> {
    vec![
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
            "a guard's value, a pattern",
            rule("(and (a ?x) (neq ?x (open k)))", "(q ?x)"),
        ),
        (
            "a conclusion's argument, a pattern",
            rule("(a ?x)", "(q (open ?x))"),
        ),
        ("a goal's pattern", "(query :goal (p (open ?x)))\n".into()),
        (
            "a claim's nested fact",
            "(query :goal (p ?x) :expect (model (p (open k))))\n".into(),
        ),
        ("an activator's argument", format!("{COPY}(copy open)\n")),
        ("a fact's argument", "(p open)\n".into()),
        ("a nested fact's argument", "(q (p open))\n".into()),
        (
            "a declaration's type",
            "(relation t open :why \"{?3} is a t\")\n".into(),
        ),
        ("an is-a's object", "(is-a open Thing)\n".into()),
        (
            "a premise's argument",
            rule("(and (a ?x) (p open))", "(q ?x)"),
        ),
        ("an eq's value", rule("(and (a ?x) (eq ?y open))", "(q ?y)")),
        ("a conclusion's argument", rule("(a ?x)", "(p open)")),
        ("a fact's :rule", "(p k :rule open)\n".into()),
        (
            "a trace step's name",
            "(trace (step open :derives (a k)))\n".into(),
        ),
        (
            "a macro call's argument in a :match",
            format!("{PP}{}", rule("(and (a ?x) (pp open))", "(q ?x)")),
        ),
        (
            "a macro's body, the verdict, called in an :assert",
            format!(
                "{LIKES}{OWE}{}",
                rule("(and (a ?x) (absent (likes ?x ?y)))", "(owe likes)")
            ),
        ),
        ("a macro's body no rule calls", OWE.into()),
        (
            "a premise headed open",
            rule("(and (a ?x) (open (p ?x)))", "(q ?x)"),
        ),
        (
            "an absent of a premise headed open",
            rule("(and (a ?x) (absent (open (p ?x))))", "(q ?x)"),
        ),
        ("a written hypothesis", "(open (p k))\n".into()),
        (
            "a written hypothesis of a negation",
            "(open (not (p k)))\n".into(),
        ),
        (
            "a hypothesis beside a fact",
            rule("(a ?x)", "(and (open (p ?x)) (q ?x))"),
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

/// `file` refused on every command: one first line, the same on each, no
/// container written and no rule fired in the `--events` log.
fn refused_everywhere(file: &Path, dir: &Path) -> Vec<String> {
    let mut bad = Vec::new();
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let mut first: Option<String> = None;
    for (cmd, code) in COMMANDS {
        let _ = std::fs::remove_file(&events);
        let args = argv(cmd, file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein(&argv);
        let line = r.err.lines().next().unwrap_or("").to_string();
        if r.code != code {
            bad.push(format!(
                "ein {argv:?}: exit {}, want {code}: {line}",
                r.code
            ));
            continue;
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

/// `file` loads: `ein saturate` answers it, exit 0.
fn loads(file: &Path) -> Option<String> {
    let path = file.display().to_string();
    let r = ein(&["saturate", &path]);
    (r.code != 0).then(|| format!("ein saturate: exit {}: {}", r.code, r.err))
}

#[test]
fn open_is_refused_where_a_program_would_make_it_more_than_the_kernels() {
    let scratch = Scratch::new("ein-open-reserved");
    let mut bad = Vec::new();
    let places = refused_places();
    for (i, (place, body)) in places.iter().enumerate() {
        let file = scratch.path().join(format!("place-{i:02}.ein"));
        let claim = if body.contains("(query") { "" } else { CLAIM };
        std::fs::write(&file, format!("{HEAD}{body}{claim}")).expect("written");
        for b in refused_everywhere(&file, scratch.path()) {
            bad.push(format!("{place}: {b}"));
        }
    }
    assert_eq!(places.len(), 22, "every place the census wrote");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn open_is_a_name_where_the_kernel_does_not_read_it() {
    let scratch = Scratch::new("ein-open-named");
    let mut bad = Vec::new();
    for (i, (place, body)) in named_places().iter().enumerate() {
        let file = scratch.path().join(format!("named-{i:02}.ein"));
        let claim = if body.contains("(query") { "" } else { CLAIM };
        std::fs::write(&file, format!("{HEAD}{body}{claim}")).expect("written");
        if let Some(b) = loads(&file) {
            bad.push(format!("{place}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The regressions' twenty-four witnesses of `open` written as a name, each
/// read from its file: four refused on every command, for what the kernel
/// does not take, and twenty that load.
#[test]
fn the_registers_witnesses_are_refused_or_load() {
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
    assert_eq!(witnesses.len(), 24);
    const REFUSED: [&str; 4] = [
        "open-fact-head.ein",
        "open-obligation-parameter.ein",
        "open-relation-parameter.ein",
        "open-solve-list.ein",
    ];
    let mut bad = Vec::new();
    for w in &witnesses {
        let name = w.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if REFUSED.contains(&name) {
            for b in refused_everywhere(w, scratch.path()) {
                bad.push(format!("{}: {b}", w.display()));
            }
        } else if let Some(b) = loads(w) {
            bad.push(format!("{}: {b}", w.display()));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The verdict loads where a rule's or an hrule's `:assert` states it**,
/// `(open ?R)` with `?R` a parameter and `(open likes)` naming a relation, and
/// so does a hypothesis a rule concludes, `(open (likes ?x ?x))`, and every
/// command answers each, its container too.
#[test]
fn the_verdict_loads_where_a_rule_concludes_it() {
    let scratch = Scratch::new("ein-open-verdict");
    let owed = "(likes k k)\n";
    for (form, rule) in [
        (
            "(open (likes ?x ?x))",
            "(rule r () :match (a ?x) :assert (open (likes ?x ?x)) :why \"r\")\n",
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

/// **A container whose stored program nests `open` under `open` is refused
/// where it is read**: the stored program is read back through the parser
/// (Q-M1f.15), which refuses it at the inner `open` (Q-M1f.568). Written
/// through `ein_einb::save`, the stored program a tree the parser does not
/// build, `(open (open k))`.
#[test]
fn a_container_that_stores_open_under_open_is_refused() {
    let scratch = Scratch::new("ein-open-container");
    let dir = scratch.path();
    let mut ast = ein_ir::Ast::new();
    let mut terms = ein_core::Terms::new();
    let held = ein_ir::parse(
        &mut ast,
        "(relation p Thing :why \"{?3} is a p\") (is-a k Thing)",
        Some("k.ein"),
    )
    .expect("it parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &held, None).expect("it loads");
    let mut forms = held.clone();
    let k = ast.atom("k", None);
    let p = ast.atom("p", None);
    let pk = ast.sform(p, &[k], None);
    let open = ast.atom("open", None);
    let inner = ast.sform(open, &[pk], None);
    let open2 = ast.atom("open", None);
    forms.push(ast.sform(open2, &[inner], None));
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
