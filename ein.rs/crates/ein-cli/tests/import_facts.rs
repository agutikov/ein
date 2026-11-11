//! **A flat import brings a module's facts, on every command** — M1f
//! S1f.6.11 (W88; Q-M1f.135 to .144).
//!
//! `(import M :symbols (…))` kept the listed declarations and their closure
//! and dropped every fact of `M`, in silence, so a library of facts, an
//! ontology or a table, arrived empty, and a relation `M` only states facts
//! of could not be listed at all. The user ruled on 2026-11-11 that
//! *importing facts is mandatory*. Since the stage a name in the selection's
//! closure selects the module's declaration of it, if any, and every
//! top-level fact whose relation it is (`ein_ir::imports`' `select`).
//!
//! What a selection brings, form for form, is `ein-ir`'s
//! `imports_semantics.rs`. This file holds the other half of the acceptance,
//! *on every command that loads a program*: each register witness, given a
//! claim, answers on every command what its **twin** answers, the same
//! program with the selected forms written in place of the import, and its
//! saved container answers what the file does. One tool, consistent answers.

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

const IS_A: &str = "(relation is-a Thing Type :why \"{?1} is a {?2}\")\n\
     (is-a Tea Drink)\n\
     (is-a Milk Drink :source \"the ontology\")\n";

/// A register witness of W88: its file, its module, the forms its import
/// selects, as the module writes them and in its order, the claim its goal's
/// rows make.
struct Witness {
    file: &'static str,
    module: &'static str,
    selected: String,
    claim: &'static str,
}

fn witnesses() -> Vec<Witness> {
    vec![
        Witness {
            file: "import-facts-flat",
            module: "import-facts-library",
            selected: IS_A.to_string(),
            claim: "(model (is-a Milk Drink) (is-a Tea Drink))",
        },
        Witness {
            file: "import-facts-rule",
            module: "import-facts-library",
            selected: format!(
                "{IS_A}(rule liquids ()\n  :match  (is-a ?x Drink)\n  :assert (liquid ?x)\n  \
                 :why    \"{{?x}} is a drink, so {{?x}} is a liquid\")\n"
            ),
            claim: "(model (liquid Milk) (liquid Tea))",
        },
        Witness {
            file: "import-facts-undeclared",
            module: "import-facts-library",
            selected: "(likes Tea Milk)\n(not (likes Milk Tea))\n".to_string(),
            claim: "(model (likes Tea Milk))",
        },
        Witness {
            file: "import-facts-table",
            module: "import-facts-table-module",
            selected: "(relation fa Digit Digit Digit Digit Digit :why \"{?1} + {?2} + carry {?3} is digit {?4}, carry {?5}\")\n\
                 (relation num2 Digit Digit :why \"the number {?1}{?2}\")\n\
                 (relation add Number Number :why \"asked: {?1} + {?2}\")\n\
                 (relation sum-of Number Number Number :why \"{?1} + {?2} = {?3}\")\n\
                 (fa 9 1 0 0 1)\n\
                 (fa 0 0 1 1 0)\n\
                 (rule add2 ()\n  :match  (and (add (num2 ?a1 ?a0) (num2 ?b1 ?b0))\n               \
                 (fa ?a0 ?b0 0 ?s0 ?c1)\n               (fa ?a1 ?b1 ?c1 ?s1 0))\n  \
                 :assert (sum-of (num2 ?a1 ?a0) (num2 ?b1 ?b0) (num2 ?s1 ?s0))\n  \
                 :why    \"{?a1}{?a0} + {?b1}{?b0} is {?s1}{?s0}, column by column\")\n"
                .to_string(),
            claim: "(model (sum-of (num2 0 9) (num2 0 1) (num2 1 0)))",
        },
    ]
}

/// The witness with its claim, and its twin: the forms the import selects
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
    let twin = claimed.replace(&format!("{import}\n"), &w.selected);
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
