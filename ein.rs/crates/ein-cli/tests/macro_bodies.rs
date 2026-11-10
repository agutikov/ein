//! **No path executes a malformed primitive a macro's expansion writes** —
//! M1f S1f.5.24, on the user's ruling of 2026-11-10 on Q-M1f.67: *"do we allow
//! invalid macro if it is not instantiated - the answer is yes, we allow, error
//! will raise later during either compilation of instantiation or even later,
//! the ultimate goal in this case is not allow execution of (eq <arg>)"*.
//!
//! A macro's body is a template the grammar never reads expanded, so until
//! S1f.5.24 what a call wrote was met where it was used, or not at all (W66 to
//! W69): `eq` or `absent` at another arity was refused by the compiler only
//! where it compiled the plan, so a rule only a commitment activates, an hrule,
//! an obligation and a rule no activator reaches loaded and some or every
//! command answered; a `not` of no fact and a connective where a value goes
//! were matched; `(eq k)`, `(false k)`, an `(or …)`, `(not k)` and a
//! conjunction as a value were concluded, by a firing or by the hypothesis
//! layer; and a conclusion that is no form stopped only the firing that met it.
//! Since then the loader holds a rule's clauses, after expansion, to the shapes
//! the grammar gives a written rule (`ein_ir::from_ir`'s
//! `refuse_malformed_primitives`, Q-M1f.80, Q-M1f.81), so each is refused at
//! instantiation, before any command compiles, fires, proposes or owes it.
//!
//! Every shape a macro's body can malform a primitive in, at every site a rule
//! is activated from (at root, only under a commitment, by no activator, as an
//! hrule and, for a `:match`, as an obligation), and the sixteen register
//! witnesses, each on every command that reads a program and every path that
//! fires a rule: `saturate`, every `solve` mode, `--events`, `oracle`, `test`
//! with a claim, the views, `kb save` and so a container. Each refuses with one
//! first line, the same on every command, and writes no container, and the
//! `--events` log holds no compilation, firing or debt of the rule. A macro
//! no rule calls loads, whatever its body, and every command answers it.

use std::path::{Path, PathBuf};
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
                    (relation s Thing :why \"{?1} is an s\")\n\
                    (relation p Thing :why \"{?1} is a p\")\n\
                    (relation q Thing :why \"{?1} is a q\")\n\
                    (is-a k Thing)\n\
                    (a k)\n";

/// A claim `ein test` checks, which holds of every program here that loads.
const CLAIM: &str = "(query :goal (a ?x) :expect (model (a k)))\n";

/// `(shape, macro declarations, the call, its clause, what the line says)`;
/// the clause is `:match` or `:assert`.
const SHAPES: [(&str, &str, &str, &str, &str); 18] = [
    (
        "eq-literal",
        "(macro one (?a) (eq ?a) :why \"{?a} equals\")",
        "(one ?x)",
        ":match",
        "writes (eq ?x), and eq takes exactly 2 arguments, not 1",
    ),
    (
        "eq-through-a-parameter",
        "(macro app (?R ?a) (?R ?a) :why \"{?R} of {?a}\")",
        "(app eq ?x)",
        ":match",
        "writes (eq ?x), and eq takes exactly 2 arguments, not 1",
    ),
    (
        "absent-literal",
        "(macro two (?a ?b) (absent ?a ?b) :why \"neither {?a} nor {?b}\")",
        "(two (p ?x) (q ?x))",
        ":match",
        "writes (absent (p ?x) (q ?x)), and absent takes exactly 1 argument, not 2",
    ),
    (
        "absent-through-a-parameter",
        "(macro app2 (?R ?a ?b) (?R ?a ?b) :why \"{?R} of {?a} and {?b}\")",
        "(app2 absent (p ?x) (q ?x))",
        ":match",
        "writes (absent (p ?x) (q ?x)), and absent takes exactly 1 argument, not 2",
    ),
    (
        "not-of-a-conjunction",
        "(macro neg (?a) (not ?a) :why \"not {?a}\")",
        "(absent (neg (and (p ?x) (q ?x))))",
        ":match",
        "writes (not (and (p ?x) (q ?x))), and not takes a pattern",
    ),
    (
        "unknown-of-a-conjunction",
        "(import std.macro :symbols (unknown))",
        "(unknown (and (p ?x) (q ?x)))",
        ":match",
        "writes (not (and (p ?x) (q ?x))), and not takes a pattern",
    ),
    (
        "not-of-a-variable",
        "(macro neg (?a) (not ?a) :why \"not {?a}\")",
        "(absent (neg ?x))",
        ":match",
        "writes (not ?x), and not takes a pattern (relation argument …), not ?x",
    ),
    (
        "neq-of-a-connective",
        "(macro diff (?a ?b) (neq ?a ?b) :why \"{?a} is not {?b}\")",
        "(diff ?x (and (p k)))",
        ":match",
        "writes (neq ?x (and (p k))), and (and (p k)) is a connective where a value goes",
    ),
    (
        "eq-conclusion",
        "(macro one (?a) (eq ?a) :why \"{?a} equals\")",
        "(one ?x)",
        ":assert",
        "writes (eq ?x), and eq takes exactly 2 arguments, not 1",
    ),
    (
        "eq-conclusion-through-a-parameter",
        "(macro app (?R ?a) (?R ?a) :why \"{?R} of {?a}\")",
        "(app eq ?x)",
        ":assert",
        "writes (eq ?x), and eq takes exactly 2 arguments, not 1",
    ),
    (
        "false-with-an-argument",
        "(macro bad (?a) (false ?a) :why \"{?a} is impossible\")",
        "(bad ?x)",
        ":assert",
        "writes (false ?x), and false takes no argument, not 1",
    ),
    (
        "false-through-a-parameter",
        "(macro app (?R ?a) (?R ?a) :why \"{?R} of {?a}\")",
        "(app false ?x)",
        ":assert",
        "writes (false ?x), and false takes no argument, not 1",
    ),
    (
        "or-conclusion",
        "(macro either (?a) (or (p ?a) (q ?a)) :why \"{?a} is a p or a q\")",
        "(either ?x)",
        ":assert",
        "writes (or (p ?x) (q ?x)), and an (or …) is no conclusion",
    ),
    (
        "not-of-a-name",
        "(macro neg (?a) (not ?a) :why \"not {?a}\")",
        "(neg k)",
        ":assert",
        "writes (not k), and not takes a pattern (relation argument …) or a variable, not k",
    ),
    (
        "not-of-a-conjunction-concluded",
        "(macro nboth (?a) (not (and (p ?a) (q ?a))) :why \"not both for {?a}\")",
        "(nboth ?x)",
        ":assert",
        "writes (not (and (p ?x) (q ?x))), and not takes a pattern (relation argument …) or a \
         variable",
    ),
    (
        "conjunction-as-a-value",
        "(macro wrap (?a) (q (and (p ?a))) :why \"q of {?a}\")",
        "(wrap ?x)",
        ":assert",
        "writes (q (and (p ?x))), and (and (p ?x)) is a connective where a value goes",
    ),
    (
        "bare-variable-conclusion",
        "(macro id (?x) ?x :why \"{?x} itself\")",
        "(id ?x)",
        ":assert",
        "concludes ?x, which is not a fact",
    ),
    (
        "bare-name-conclusion",
        "(macro id (?x) ?x :why \"{?x} itself\")",
        "(id k)",
        ":assert",
        "concludes k, which is not a fact",
    ),
];

/// Where the rule using the call is activated from: `(site, who, text)`, or
/// `None` where the site takes no such clause (an obligation's `:assert` is
/// `(open …)`).
fn site(site: &str, call: &str, clause: &str) -> Option<(&'static str, String)> {
    let m = |anchor: &str| {
        if clause == ":match" {
            format!("(and {anchor} {call})")
        } else {
            anchor.to_string()
        }
    };
    let a = if clause == ":assert" { call } else { "(p ?x)" };
    Some(match site {
        "root" => (
            "rule 'r'",
            format!(
                "(rule r ()\n  :match  {}\n  :assert {a}\n  :why \"r of {{?x}}\")\n",
                m("(a ?x)")
            ),
        ),
        "commitment" => (
            "rule 'r'",
            format!(
                "(rule mk () :match (s ?x) :assert (r a) :why \"{{?x}} is an s, so r is activated\")\n\
                 (rule r (?R)\n  :match  {}\n  :assert {a}\n  :why \"r of {{?x}}\")\n",
                m("(?R ?x)")
            ),
        ),
        "never" => (
            "rule 'r'",
            format!(
                "(rule r (?R)\n  :match  {}\n  :assert {a}\n  :why \"r of {{?x}}\")\n",
                m("(?R ?x)")
            ),
        ),
        "hrule" => (
            "hrule 'r'",
            format!(
                "(hrule r ()\n  :match  {}\n  :assert {a}\n  :why \"guess of {{?x}}\")\n\
                 (solve :hrules (r))\n",
                m("(a ?x)")
            ),
        ),
        "obligation" if clause == ":match" => (
            "rule 'r'",
            format!(
                "(rule r ()\n  :match  {}\n  :assert (open)\n  :why \"owed for {{?x}}\")\n",
                m("(a ?x)")
            ),
        ),
        _ => return None,
    })
}

const SITES: [&str; 5] = ["root", "commitment", "never", "hrule", "obligation"];

/// Every command that reads a program and every path that fires a rule, the
/// file last; `{einb}` and `{events}` are the run's scratch paths. Each with
/// the exit a load error takes (`ein test`: 2).
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

/// The rule's own steps in an `--events` log: its plan compiled, a firing,
/// a debt. A run refused at load has none.
fn rule_steps(events: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(events) else {
        return Vec::new();
    };
    text.lines()
        .filter(|l| {
            ["\"compile\"", "\"fire\"", "\"owe\""]
                .iter()
                .any(|k| l.contains(&format!("\"e\": {k}")))
                && l.contains("\"rule\": \"r\"")
        })
        .map(str::to_string)
        .collect()
}

/// `file` refused on every command with one first line, the same on each,
/// `want` in it, no container written and no step of the rule taken.
fn refused_everywhere(file: &Path, who: &str, clause: &str, want: &str, dir: &Path) -> Vec<String> {
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
        let fits = line.starts_with(&format!("kb load error: {}:", file.display()))
            && line.contains(&format!(": {who}: its {clause} {want}"));
        if !fits {
            bad.push(format!(
                "ein {argv:?}: {line:?}\n  wants kb load error: {}:<line>:<col>: {who}: its \
                 {clause} {want} …",
                file.display()
            ));
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
        let steps = rule_steps(&events);
        if !steps.is_empty() {
            bad.push(format!("ein {argv:?} took the rule's steps: {steps:?}"));
        }
    }
    bad
}

#[test]
fn every_malformed_primitive_is_refused_before_any_path_runs_it() {
    let scratch = Scratch::new("ein-macro-bodies");
    let mut bad = Vec::new();
    let mut programs = 0usize;
    for (shape, macros, call, clause, want) in SHAPES {
        for s in SITES {
            let Some((who, rule)) = site(s, call, clause) else {
                continue;
            };
            let file = scratch.path().join(format!("{shape}--{s}.ein"));
            std::fs::write(&file, format!("{HEAD}{macros}\n{rule}{CLAIM}")).expect("written");
            programs += 1;
            for b in refused_everywhere(&file, who, clause, want, scratch.path()) {
                bad.push(format!("{shape} at {s}: {b}"));
            }
        }
    }
    assert_eq!(programs, 8 * 5 + 10 * 4, "every shape at every site");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's sixteen witnesses (W66 to W69) and the load-negative fixture
/// S1f.5.21 left at compile, each read from its file, as `wrong_answers.rs`
/// reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-macro-witnesses");
    let witnesses: [(&str, &str, &str, &str); 17] = [
        (
            "macro-eq-under-commitment",
            "rule 'r'",
            ":match",
            "writes (eq ?x)",
        ),
        (
            "macro-absent-hrule",
            "hrule 'h'",
            ":match",
            "writes (absent (p ?x) (q ?x))",
        ),
        (
            "macro-eq-obligation",
            "rule 'o'",
            ":match",
            "writes (eq ?x)",
        ),
        (
            "macro-absent-never-compiled",
            "rule 'r'",
            ":match",
            "writes (absent (p ?x) (q ?x))",
        ),
        (
            "macro-not-conjunction",
            "rule 'r'",
            ":match",
            "writes (not (and (p ?x) (q ?x)))",
        ),
        (
            "macro-neq-connective",
            "rule 'r'",
            ":match",
            "writes (neq ?x (and (p k)))",
        ),
        (
            "macro-neq-connective-hrule",
            "hrule 'h'",
            ":match",
            "writes (neq ?x (and (p k)))",
        ),
        (
            "macro-not-conjunction-obligation",
            "rule 'o'",
            ":match",
            "writes (not (and (p ?x) (q ?x)))",
        ),
        (
            "macro-eq-conclusion",
            "rule 'r'",
            ":assert",
            "writes (eq ?x)",
        ),
        (
            "macro-false-argument",
            "rule 'r'",
            ":assert",
            "writes (false ?x)",
        ),
        (
            "macro-or-conclusion",
            "rule 'r'",
            ":assert",
            "writes (or (p ?x) (q ?x))",
        ),
        (
            "macro-not-name-conclusion",
            "rule 'r'",
            ":assert",
            "writes (not k)",
        ),
        (
            "macro-connective-value",
            "rule 'r'",
            ":assert",
            "writes (q (and (p ?x)))",
        ),
        (
            "macro-eq-candidate",
            "hrule 'h'",
            ":assert",
            "writes (eq ?x)",
        ),
        (
            "macro-bare-conclusion-under-commitment",
            "rule 'r'",
            ":assert",
            "concludes ?x",
        ),
        (
            "macro-bare-conclusion-hrule",
            "hrule 'h'",
            ":assert",
            "concludes ?x",
        ),
        ("eq_in_a_macro_body", "rule 'r'", ":match", "writes (eq ?x)"),
    ];
    let mut bad = Vec::new();
    for (name, who, clause, want) in witnesses {
        let file = if name.starts_with("macro-") {
            PathBuf::from(format!("examples/ein-bugs/wrong-answers/{name}.ein"))
        } else {
            PathBuf::from(format!("examples/broken/load/{name}.ein"))
        };
        assert!(repo_root().join(&file).is_file(), "{}", file.display());
        for b in refused_everywhere(&file, who, clause, want, scratch.path()) {
            bad.push(format!("{name}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A macro no rule calls loads, whatever its body** — the user's ruling on
/// Q-M1f.67: every malformed body above, declared and called by no rule, beside
/// a rule that loads, is answered by every command, exit 0, the claim held.
#[test]
fn an_unused_malformed_macro_loads() {
    let scratch = Scratch::new("ein-macro-unused");
    let mut macros: Vec<&str> = SHAPES.iter().map(|s| s.1).collect();
    macros.sort();
    macros.dedup();
    let file = scratch.path().join("unused.ein");
    std::fs::write(
        &file,
        format!(
            "{HEAD}{}\n(rule r () :match (a ?x) :assert (p ?x) :why \"{{?x}} is an a, so a p\")\n{CLAIM}",
            macros.join("\n")
        ),
    )
    .expect("written");
    let einb = scratch.path().join("x.einb");
    let events = scratch.path().join("events.jsonl");
    for (cmd, _) in COMMANDS {
        let args = argv(cmd, &file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein(&argv);
        assert_eq!(r.code, 0, "ein {argv:?}:\n{}", r.err);
    }
    // The container a used one could not be: written, and answered as the file.
    for cmd in ["saturate", "solve", "oracle", "test"] {
        let r = ein(&[cmd, &einb.display().to_string()]);
        assert_eq!(r.code, 0, "ein {cmd} on the container:\n{}", r.err);
    }
}
