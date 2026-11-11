//! **`not` of a name is the negation of no fact** — M1f S1f.7.4 (W84,
//! Q-M1f.127 and Q-M1f.128), under the value rule (Q-M1f.113, the user's
//! ruling of 2026-11-11: *a value is a name, or a fact a state could hold: an
//! atom of an ordinary relation, or its negation*).
//!
//! The grammar takes `(not ?x)` as a conclusion, for a variable bound to a
//! fact. Until S1f.7.4 a rule whose variable held a name stored `(not k)`,
//! which no program can write and whose `--dump-answer` state file did not
//! reload, and an hrule proposed it, so R held two states that differed in a
//! fact nothing can read.
//!
//! Two halves, both held here. **The loader** refuses the conclusion wherever
//! the program shows that the variable can hold a name (`ein_ir::from_ir`'s
//! `refuse_negated_names`): one line, the same on every command. **The
//! firing** builds no fact for the negation of a name
//! (`ein_infer::firing::build_fact`), so a binding the loader does not read,
//! through the relation of a parameter whose activator a rule derives, stores
//! nothing and proposes nothing, no state file holds one, and `ein solve`
//! counts what `ein oracle` counts. Beside them, what must keep
//! loading: `(not ?h)` of a variable bound to facts alone, as the three
//! `examples/saturation/hypothesis-contradiction/` programs write it.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
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
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// Two guests, of whom Ann has paid.
const HEAD: &str = "(relation paid Guest :why \"{?1} has paid\")\n\
     (relation seen Guest :why \"{?1} was seen\")\n\
     (is-a Ann Guest) (is-a Bob Guest)\n\
     (paid Ann)\n";

const TAIL: &str = "not takes a pattern (relation argument …) or a variable bound to a fact: it is \
                    the stored negation of one fact, and a name is no fact; negation as failure is \
                    (absent …)";

/// `(shape, the forms after HEAD, what the first line of stderr holds)`.
const REFUSED: [(&str, &str, &str); 18] = [
    (
        "a-written-fact",
        "(rule r () :match (paid ?g) :assert (not ?g) :why \"w\")",
        "rule 'r': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where \
         the fact (paid Ann) holds Ann — not takes",
    ),
    (
        "an-hrule",
        "(hrule r () :match (paid ?g) :assert (not ?g) :why \"w\")",
        "hrule 'r': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where \
         the fact (paid Ann) holds Ann — not takes",
    ),
    (
        "one-of-two-conclusions",
        "(rule r () :match (paid ?g) :assert (and (seen ?g) (not ?g)) :why \"w\")",
        "rule 'r': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g",
    ),
    (
        "a-typed-premise",
        "(rule r () :match (is-a ?g Guest) :assert (not ?g) :why \"w\")",
        "(is-a ?g Guest) binds ?g where the fact (is-a Ann Guest) holds Ann",
    ),
    (
        "a-fact-inside-a-fact",
        "(says Bob (owes Ann))\n(rule r () :match (owes ?g) :assert (not ?g) :why \"w\")",
        "(owes ?g) binds ?g where the fact (says Bob (owes Ann)) holds Ann",
    ),
    (
        "a-position-that-holds-both",
        "(claim (paid Bob)) (claim Ann)\n\
         (rule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
        "(claim ?c) binds ?c where the fact (claim Ann) holds Ann",
    ),
    (
        "a-declaration",
        "(rule r () :match (relation ?R ?T) :assert (not ?R) :why \"w\")",
        "(relation ?R ?T) binds ?R where a declaration names a relation or a type",
    ),
    (
        "a-parameter",
        "(r Bob)\n(rule r (?g) :match () :assert (not ?g) :why \"w\")",
        "?g is its parameter, which the fact (r Bob) binds to Bob",
    ),
    (
        "an-hrules-activator",
        "(hrule r (?g) :match () :assert (not ?g) :why \"w\")\n(solve :hrules (r (Bob)))",
        "hrule 'r': its :assert writes (not ?g), and ?g can be a name: ?g is its parameter, \
         which (solve :hrules (r (Bob))) binds to Bob",
    ),
    (
        "another-rules-conclusion",
        "(rule mk () :match (is-a ?g Guest) :assert (met ?g) :why \"m\")\n\
         (rule r () :match (met ?g) :assert (not ?g) :why \"w\")",
        "(met ?g) binds ?g where rule 'mk' writes (met ?g)",
    ),
    (
        "an-hrules-conclusion",
        "(hrule mk () :match (is-a ?g Guest) :assert (seen ?g) :why \"m\")\n\
         (rule r () :match (seen ?g) :assert (not ?g) :why \"w\")",
        "(seen ?g) binds ?g where hrule 'mk' writes (seen ?g)",
    ),
    (
        "a-guard-alone",
        "(rule r () :match (neq ?g Ann) :assert (not ?g) :why \"w\")",
        "(neq ?g Ann) binds ?g over the program's names, nothing else binding it",
    ),
    // A pattern whose relation is the rule's parameter, read as the relation
    // its written activator names.
    (
        "a-parameters-relation",
        "(r paid)\n(rule r (?R) :match (?R ?g) :assert (not ?g) :why \"w\")",
        "(?R ?g) binds ?g where the fact (paid Ann) holds Ann",
    ),
    (
        "an-hrules-parameters-relation",
        "(hrule r (?R) :match (?R ?g) :assert (not ?g) :why \"w\")\n(solve :hrules (r (paid)))",
        "hrule 'r': its :assert writes (not ?g), and ?g can be a name: (?R ?g) binds ?g where \
         the fact (paid Ann) holds Ann",
    ),
    (
        "a-name-written-through-a-parameter",
        "(mk met)\n(rule mk (?R) :match (is-a ?g Guest) :assert (?R ?g) :why \"m\")\n\
         (rule r () :match (met ?g) :assert (not ?g) :why \"w\")",
        "(met ?g) binds ?g where rule 'mk' writes (?R ?g)",
    ),
    // No fact of `seen` is written: a commitment states one, of a guest.
    (
        "a-declared-relation",
        "(rule r () :match (seen ?g) :assert (not ?g) :why \"w\")",
        "(seen ?g) binds ?g where a commitment can hold a name, seen being a declared relation \
         the hypotheses are taken from",
    ),
    (
        "a-declared-relation-among-the-listed",
        "(rule r () :match (seen ?g) :assert (not ?g) :why \"w\")\n\
         (solve :hypothesis-relations (seen))",
        "(seen ?g) binds ?g where a commitment can hold a name",
    ),
    (
        "a-macros-not",
        "(macro neg (?a) (not ?a) :why \"m\")\n\
         (rule r () :match (paid ?g) :assert (neg ?g) :why \"w\")",
        "rule 'r': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where \
         the fact (paid Ann) holds Ann",
    ),
];

/// What loads: `(not ?x)` of a variable the program shows no name for.
const ACCEPTED: [(&str, &str); 10] = [
    // A declared relation held out of the hypotheses holds what is written.
    (
        "a-declared-relation-held-out",
        "(relation claim Claim :why \"{?1} is claimed\")\n(claim (paid Bob))\n\
         (solve :no-hypothesis (claim))\n\
         (rule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-declared-relation-not-listed",
        "(relation claim Claim :why \"{?1} is claimed\")\n(claim (paid Bob))\n\
         (solve :hypothesis-relations (paid))\n\
         (rule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-declared-relation-beside-an-hrule",
        "(relation claim Claim :why \"{?1} is claimed\")\n(claim (paid Bob))\n\
         (hrule guess () :match (is-a ?g Guest) :assert (seen ?g) :why \"g\")\n\
         (rule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-position-that-holds-facts",
        "(claim (paid Bob))\n(rule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
    ),
    (
        "two-premises-one-holding-no-name",
        "(claim (paid Bob))\n\
         (rule r () :match (and (paid ?g) (claim ?c)) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-premise-under-absent",
        "(claim (paid Bob))\n\
         (rule r () :match (and (claim ?c) (absent (paid ?c))) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-guard-beside-a-premise",
        "(claim (paid Bob))\n\
         (rule r () :match (and (claim ?c) (neq ?c Ann)) :assert (not ?c) :why \"w\")",
    ),
    (
        "a-parameter-bound-to-a-fact",
        "(r (paid Bob))\n(rule r (?c) :match () :assert (not ?c) :why \"w\")",
    ),
    (
        "an-hrule-over-facts",
        "(claim (paid Bob))\n(hrule r () :match (claim ?c) :assert (not ?c) :why \"w\")",
    ),
    (
        "the-negation-of-a-pattern",
        "(rule r () :match (is-a ?g Guest) :assert (not (seen ?g)) :why \"w\")",
    ),
];

/// Every command that reads a program, the file last, each with the exit a
/// load error takes (`ein test`: 2); `value_rule.rs`'s list.
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

/// `file` refused on every command with one stderr, the same on each: its
/// first line placed in `shown`, holding `first` and ending as every such
/// refusal ends, and no container written.
fn refused_everywhere(file: &Path, shown: &str, first: &str, dir: &Path) -> Vec<String> {
    let mut bad = Vec::new();
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let mut seen: Option<String> = None;
    for (cmd, code) in COMMANDS {
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
        if !line.starts_with(&format!("kb load error: {shown}:"))
            || !line.contains(first)
            || !line.ends_with(TAIL)
        {
            bad.push(format!(
                "ein {argv:?}: {:?}\n  wants a first line at {shown} holding {first:?}",
                r.err
            ));
        }
        match &seen {
            None => seen = Some(r.err.clone()),
            Some(s) if *s != r.err => bad.push(format!(
                "ein {argv:?}: {:?}, where the first command printed {s:?}",
                r.err
            )),
            Some(_) => {}
        }
        if einb.exists() {
            bad.push(format!(
                "ein {argv:?} wrote a container of a refused program"
            ));
            let _ = std::fs::remove_file(&einb);
        }
    }
    bad
}

#[test]
fn a_conclusion_negating_a_name_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-negated-names");
    let mut bad = Vec::new();
    for (shape, body, first) in REFUSED {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{body}\n")).expect("written");
        let shown = file.display().to_string();
        for b in refused_everywhere(&file, &shown, first, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's seven witnesses (W84), each read from its file, as
/// `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-negated-names-witnesses");
    let witnesses: [(&str, &str); 7] = [
        (
            "not-name-concluded",
            "rule 'deny': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g \
             where the fact (paid Ann) holds Ann",
        ),
        (
            "not-name-hrule-concluded",
            "hrule 'suppose': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds \
             ?g where the fact (paid Ann) holds Ann",
        ),
        (
            "not-name-parameter",
            "?g is its parameter, which the fact (deny Bob) binds to Bob",
        ),
        (
            "not-name-macro",
            "rule 'deny': its :assert writes (not ?g), and ?g can be a name",
        ),
        (
            "not-name-guard",
            "(neq ?g Ann) binds ?g over the program's names",
        ),
        (
            "not-name-derived",
            "(seen ?g) binds ?g where rule 'greet' writes (seen ?g)",
        ),
        (
            "not-name-under-commitment",
            "(seen ?g) binds ?g where a commitment can hold a name, seen being a declared \
             relation the hypotheses are taken from",
        ),
    ];
    let mut bad = Vec::new();
    for (stem, first) in witnesses {
        let shown = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        for b in refused_everywhere(Path::new(&shown), &shown, first, scratch.path()) {
            bad.push(format!("{stem}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What `(not ?x)` may still conclude, and the three corpus programs that
/// conclude it of a hypothesis, each storing the negation it did.
#[test]
fn the_negation_of_a_bound_fact_loads() {
    let scratch = Scratch::new("ein-negated-names-accepted");
    let mut bad = Vec::new();
    for (shape, body) in ACCEPTED {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{body}\n")).expect("written");
        let f = file.display().to_string();
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let runs: [Vec<&str>; 5] = [
            vec!["saturate", &f],
            vec!["solve", "--count", &f],
            vec!["render", "program", &f],
            vec!["render", "kb", &f],
            vec!["kb", "save", &f, &einb],
        ];
        for argv in runs {
            let r = ein(&argv);
            if r.code != 0 {
                bad.push(format!(
                    "{shape}: ein {argv:?}: exit {}: {}",
                    r.code,
                    r.err.lines().next().unwrap_or("")
                ));
            }
        }
    }
    for (stem, stored) in [
        ("coloc-disproved", "(not (co-located Norwegian House-2)"),
        ("next-to-disproved", "(not (next-to "),
        ("right-of-disproved", "(not (right-of "),
    ] {
        let f = format!("examples/saturation/hypothesis-contradiction/{stem}.ein");
        let r = ein(&["saturate", "--dump", &f]);
        if r.code != 0
            || !r.out.contains(stored)
            || !r.out.contains(":rule hypothesis-contradiction")
        {
            bad.push(format!(
                "{stem}: exit {}, and the dump holds no {stored:?} by the rule: {}",
                r.code,
                r.err.lines().next().unwrap_or("")
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// `states (k_s)      N` of a read-out.
fn states(out: &str) -> Option<usize> {
    out.lines()
        .find_map(|l| l.trim_start().strip_prefix("states (k_s)"))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|n| n.parse().ok())
}

/// A binding the loader does not read: the firing stores no negation of a
/// name and an hrule proposes none, a stored negation of a fact beside it is
/// kept, no state file holds one, and `ein solve` counts what `ein oracle`
/// counts (Q-M1f.128). Each program reaches the name through the relation of
/// a parameter whose activator a rule derives through a variable, `(marks
/// ?R)` then `(deny ?R)`.
///
/// None is a corpus program, so none is a register row: a state file writes
/// the derived activator as a fact, the loader then reads the parameter's
/// relation through it and refuses the file, and `answer_round_trip.rs` holds
/// every corpus program's state files to reload (F25's R-1).
#[test]
fn a_name_only_a_run_meets_is_not_negated() {
    let scratch = Scratch::new("ein-negated-names-unforeseen");
    let mut bad = Vec::new();
    const ON: &str = "(rule on () :match (marks ?R) :assert (deny ?R) :why \"o\")\n";
    // (shape, the forms after HEAD and ON, R's states)
    let cases: [(&str, &str, usize); 3] = [
        // `(paid Ann)` binds `?g` to a name, `(claim (paid Bob))` to a fact.
        (
            "a-derived-activators-relation",
            "(claim (paid Bob))\n(marks paid) (marks claim)\n\
             (rule deny (?R) :match (?R ?g) :assert (not ?g) :why \"w\")\n\
             (solve :hypothesis-relations ())",
            1,
        ),
        // An hrule over `met`, which `deny` fills with a guest and the
        // program with a fact: its one candidate is `(not (paid Bob))`.
        (
            "an-hrule-over-a-derived-fact",
            "(met (paid Bob))\n(marks met)\n\
             (rule deny (?R) :match (paid ?g) :assert (?R ?g) :why \"w\")\n\
             (hrule suppose () :match (met ?c) :assert (not ?c) :why \"s\")",
            2,
        ),
        // A rule that concludes a fact beside the negation: the fact stays.
        (
            "beside-a-fact",
            "(marks paid)\n\
             (rule deny (?R) :match (?R ?g) :assert (and (seen ?g) (not ?g)) :why \"w\")\n\
             (solve :hypothesis-relations ())",
            1,
        ),
    ];
    for (shape, body, want) in cases {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{ON}{body}\n")).expect("written");
        let f = file.display().to_string();
        let einb = scratch.path().join(format!("{shape}.einb"));
        let saved = ein(&["kb", "save", &f, &einb.display().to_string()]);
        if saved.code != 0 {
            bad.push(format!("{shape}: kb save: {}", saved.err));
            continue;
        }
        for program in [f.clone(), einb.display().to_string()] {
            let sat = ein(&["saturate", "--dump", &program]);
            if sat.code != 0 || sat.out.contains("(not Ann") || sat.out.contains("(not Bob") {
                bad.push(format!(
                    "{shape}: ein saturate --dump {program}: exit {}, or it stores the negation \
                     of a name: {}",
                    sat.code,
                    sat.err.lines().next().unwrap_or("")
                ));
            }
            let solve = ein(&["solve", &program]);
            let oracle = ein(&["oracle", &program]);
            let (s, o) = (states(&solve.out), states(&oracle.out));
            if solve.code != 0 || oracle.code != 0 || s != Some(want) || o != Some(want) {
                bad.push(format!(
                    "{shape}: {program}: solve counts {s:?} (exit {}), oracle {o:?} (exit {}), \
                     want {want}",
                    solve.code, oracle.code
                ));
            }
        }
        // No state `--dump-answer` writes holds the negation of a name.
        let dir = scratch.path().join(format!("{shape}-answer"));
        let dump = ein(&["solve", &f, "--dump-answer", &dir.display().to_string()]);
        if dump.code != 0 {
            bad.push(format!("{shape}: --dump-answer: {}", dump.err));
            continue;
        }
        let mut files: Vec<_> = std::fs::read_dir(dir.join("models"))
            .expect("the dump's models")
            .map(|e| e.expect("an entry").path())
            .collect();
        files.sort();
        if files.len() != want {
            bad.push(format!("{shape}: {} state files, want {want}", files.len()));
        }
        for state in files {
            let text = std::fs::read_to_string(&state).expect("a state file");
            if text.contains("(not Ann") || text.contains("(not Bob)") {
                bad.push(format!(
                    "{shape}: {} holds the negation of a name",
                    state.display()
                ));
            }
        }
    }
    // The fact beside the negation, and the fact's negation, are stored.
    let file = scratch.path().join("beside-a-fact.ein");
    let sat = ein(&["saturate", "--dump", &file.display().to_string()]);
    if !sat.out.contains("(seen Ann :rule deny") {
        bad.push("beside-a-fact: (seen Ann) is not stored".to_string());
    }
    let file = scratch.path().join("a-derived-activators-relation.ein");
    let sat = ein(&["saturate", "--dump", &file.display().to_string()]);
    if !sat.out.contains("(not (paid Bob) :rule deny") {
        bad.push("a-derived-activators-relation: (not (paid Bob)) is not stored".to_string());
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
