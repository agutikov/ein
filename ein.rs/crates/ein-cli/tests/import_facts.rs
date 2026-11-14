//! **An import brings a module's facts, on every command** — M1f S1f.6.11
//! (W88; Q-M1f.135 to .144), and since S1f.6.35 the whole module.
//!
//! `(import M :symbols (…))` kept the listed declarations and their closure
//! and dropped every fact of `M`, in silence, so a library of facts, an
//! ontology or a table, arrived empty, and a relation `M` only states facts
//! of could not be listed at all. The user ruled on 2026-11-11 that
//! *importing facts is mandatory*. From S1f.6.11 a name in the selection's
//! closure selected the module's declaration of it, if any, and every
//! top-level fact whose relation it is; since S1f.6.35 every import brings
//! its module whole and a list only names (*"in all cases we import
//! everything, the difference only in names"*, the user, 2026-11-14).
//!
//! What an import brings, form for form, is `ein-ir`'s
//! `imports_semantics.rs`. This file holds the other half of the acceptance,
//! *on every command that loads a program*: each register witness, given a
//! claim, answers on every command what its **twin** answers, the same
//! program with the module written in place of the import, under the names
//! the program prints, and its saved container answers what the file does.
//! One tool, consistent answers.

#![cfg(feature = "einb")]

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn ein_in(dir: &Path, args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// Every command that loads a program, as the argv before and after the
/// path; `{out}` is a fresh file in the scratch directory.
///
/// The four views that parse and never load (`render rules`, `rule`,
/// `constraints` and `program`) are not here: on a file each draws the
/// program as written, its imports unresolved, and on a container the
/// resolved program it stores (Q-M1f.17), so an import of any kind is not the
/// forms written in place there, and no fact is drawn by the first two.
const COMMANDS: [(&[&str], &[&str]); 12] = [
    (&["saturate"], &[]),
    (&["saturate"], &["--dump"]),
    (&["solve"], &[]),
    (&["solve", "--minimal"], &[]),
    (&["solve", "--maximal"], &[]),
    (&["solve", "--single"], &[]),
    (&["solve", "--count"], &[]),
    (&["oracle"], &[]),
    (&["test"], &[]),
    (&["render", "lattice"], &[]),
    (&["render", "kb"], &[]),
    (&["kb", "save"], &["{out}"]),
];

/// What two runs of one command are compared on: the path each was handed
/// aside, and the lines that say what the run read and how long it took, each
/// number a line times masked (`program_read.rs`'s).
fn comparable(text: &str, path: &str) -> String {
    let mut out = Vec::new();
    for line in text.replace(path, "{FILE}").lines() {
        let t = line.trim_start();
        let size = t.ends_with(" chars") || t.ends_with(" bytes, a .einb container");
        if ["input:", "parse:", "kb load:", "open:"]
            .iter()
            .any(|p| t.starts_with(p))
            || (size && t.starts_with(|c: char| c.is_ascii_digit()))
            || t.starts_with("wall ")
        {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let masked: Vec<&str> = words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let timed = words
                    .get(i + 1)
                    .is_some_and(|n| matches!(*n, "ms" | "s" | "s)"));
                if timed && w.trim_start_matches('(').parse::<f64>().is_ok() {
                    "#"
                } else {
                    w
                }
            })
            .collect();
        out.push(masked.join(" "));
    }
    out.join("\n")
}

/// A register witness of W88: its file, its module, the names its import
/// lists, the claim its goal's rows make.
struct Witness {
    file: &'static str,
    module: &'static str,
    listed: &'static [&'static str],
    claim: &'static str,
}

fn witnesses() -> Vec<Witness> {
    vec![
        Witness {
            file: "import-facts-flat",
            module: "import-facts-library",
            listed: &["is-a"],
            claim: "(model (is-a Milk Drink) (is-a Tea Drink))",
        },
        Witness {
            file: "import-facts-rule",
            module: "import-facts-library",
            listed: &["liquids"],
            claim: "(model (liquid Milk) (liquid Tea))",
        },
        Witness {
            file: "import-facts-undeclared",
            module: "import-facts-library",
            listed: &["likes"],
            claim: "(model (likes Tea Milk))",
        },
        Witness {
            file: "import-facts-table",
            module: "import-facts-table-module",
            listed: &["add2", "add", "num2", "sum-of"],
            claim: "(model (sum-of (num2 0 9) (num2 0 1) (num2 1 0)))",
        },
    ]
}

/// **What the import brings, written in place** (M1f S1f.6.35): the module
/// whole, its comments and its `(solve …)` left out, each name it declares
/// that the program does not list under its full name, `module.unused`, as
/// the program prints it. A dotted name the main file declares where no
/// module of the program has its prefix is the program's own
/// (`defined_behaviour.md` § 4.28), so the twin loads. Strings are left as
/// they are.
fn in_place(text: &str, module: &str, listed: &[&str]) -> String {
    let declared: Vec<&str> = text
        .lines()
        .filter_map(|l| {
            ["(relation ", "(rule ", "(hrule ", "(macro "]
                .iter()
                .find_map(|d| l.strip_prefix(d))?
                .split_whitespace()
                .next()
        })
        .filter(|n| !listed.contains(n))
        .collect();
    let mut out = String::new();
    for line in text
        .lines()
        .filter(|l| !l.starts_with(';') && !l.starts_with("(solve "))
    {
        let mut in_string = false;
        let mut token = String::new();
        let flush = |token: &mut String, out: &mut String| {
            if declared.contains(&token.as_str()) {
                out.push_str(module);
                out.push('.');
            }
            out.push_str(token);
            token.clear();
        };
        for c in line.chars() {
            if in_string {
                out.push(c);
                in_string = c != '"';
            } else if c == '"' || c == '(' || c == ')' || c.is_whitespace() {
                flush(&mut token, &mut out);
                out.push(c);
                in_string = c == '"';
            } else {
                token.push(c);
            }
        }
        flush(&mut token, &mut out);
        out.push('\n');
    }
    out
}

/// The witness with its claim, and its twin: the module ([`in_place`])
/// written where the import stood. Comments are dropped from both, so a line
/// and a column are the same in neither and nothing compared names one.
fn programs(w: &Witness, dir: &Path) -> (String, String) {
    let from = repo_root().join("examples/ein-bugs/wrong-answers");
    let read = |name: &str| {
        std::fs::read_to_string(from.join(format!("{name}.ein")))
            .unwrap_or_else(|e| panic!("{name}.ein: {e}"))
    };
    std::fs::write(dir.join(format!("{}.ein", w.module)), read(w.module)).expect("the module");
    let body: String = read(w.file)
        .lines()
        .filter(|l| !l.starts_with(";;;"))
        .map(|l| format!("{l}\n"))
        .collect();
    let claimed = body.replace(" :goal-text ", &format!(" :expect {} :goal-text ", w.claim));
    assert_ne!(
        claimed, body,
        "{}: the witness asks its goal with a :goal-text",
        w.file
    );
    let import = claimed
        .lines()
        .find(|l| l.starts_with("(import "))
        .unwrap_or_else(|| panic!("{}: the witness imports its module on one line", w.file))
        .to_string();
    let twin = claimed.replace(
        &format!("{import}\n"),
        &in_place(&read(w.module), w.module, w.listed),
    );
    let (file, twin_file) = (format!("{}.ein", w.file), format!("{}-twin.ein", w.file));
    std::fs::write(dir.join(&file), &claimed).expect("the witness");
    std::fs::write(dir.join(&twin_file), &twin).expect("the twin");
    (file, twin_file)
}

fn argv<'a>(cmd: &(&'a [&'a str], &'a [&'a str]), file: &'a str, out: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = cmd.0.to_vec();
    v.push(file);
    v.extend(cmd.1.iter().map(|a| if *a == "{out}" { out } else { *a }));
    v
}

#[test]
fn a_flat_import_of_facts_answers_as_the_facts_written_in_place_on_every_command() {
    let scratch = Scratch::new("ein-import-facts");
    let mut bad = Vec::new();
    let mut runs = 0usize;
    for w in witnesses() {
        let dir = scratch.path().join(w.file);
        std::fs::create_dir_all(&dir).expect("a directory");
        let (file, twin) = programs(&w, &dir);
        let einb = format!("{}.einb", w.file);
        let saved = ein_in(&dir, &["kb", "save", &file, &einb]);
        assert_eq!(saved.code, 0, "{}: kb save: {}", w.file, saved.stderr);
        for (i, cmd) in COMMANDS.iter().enumerate() {
            let outs: Vec<String> = ["ein", "twin", "einb"]
                .iter()
                .map(|k| format!("out-{i}-{k}.einb"))
                .collect();
            let got = ein_in(&dir, &argv(cmd, &file, &outs[0]));
            let want = ein_in(&dir, &argv(cmd, &twin, &outs[1]));
            let boxed = ein_in(&dir, &argv(cmd, &einb, &outs[2]));
            runs += 3;
            let name = format!("{}: ein {}", w.file, cmd.0.join(" "));
            // Every answer exits 0; a line mode says on stderr that it checks
            // no claim about all of R, the same sentence on each.
            let said = |r: &Run, path: &str| {
                format!(
                    "exit {}\n{}\n{}",
                    r.code,
                    comparable(&r.stdout, path),
                    comparable(&r.stderr, path)
                )
            };
            let (g, t, b) = (said(&got, &file), said(&want, &twin), said(&boxed, &einb));
            if got.code != 0 {
                bad.push(format!("{name}: {g}"));
            }
            if g != t {
                bad.push(format!(
                    "{name}: the import answers\n{g}\n— and the facts written in place\n{t}"
                ));
            }
            if g != b {
                bad.push(format!(
                    "{name}: the file answers\n{g}\n— and its container\n{b}"
                ));
            }
        }
    }
    assert!(runs > 0);
    assert!(
        bad.is_empty(),
        "{} of {runs} runs disagree:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
}

/// The claim each witness's goal makes holds: `ein test` checks it, exit 0.
/// The same claim over the library's facts fails where the import brings
/// none, which is what the stage fixed.
#[test]
fn the_claims_over_imported_facts_hold() {
    let scratch = Scratch::new("ein-import-facts-claims");
    for w in witnesses() {
        let dir = scratch.path().join(w.file);
        std::fs::create_dir_all(&dir).expect("a directory");
        let (file, _) = programs(&w, &dir);
        let r = ein_in(&dir, &["test", &file]);
        assert_eq!(r.code, 0, "{}: {}{}", w.file, r.stdout, r.stderr);
        assert!(
            r.stdout.contains("1 held, 0 FAILED"),
            "{}: {}",
            w.file,
            r.stdout
        );
    }
}
