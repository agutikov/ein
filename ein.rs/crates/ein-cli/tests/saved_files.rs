//! **A saved state is saved and loaded as itself** (Q-M1f.392), on the
//! programs that show it, through the binary.
//!
//! A state `ein solve FILE --dump-answer DIR` writes is a **saved file**: the
//! program as it was read, every module its load read, each as it was read,
//! and the choices made, one `(commit …)` (Q-M1f.400, Q-M1f.401); a load
//! derives the state again, and **a saved file is a node of its program's
//! lattice** (Q-M1f.402). Here, on `tests/saved/`:
//!
//! - solve, save, load: one solution, the same facts with the same origins,
//!   a rule's fact built with a fact inside it, a choice with one inside it,
//!   and the standard library's twin, and both invariances as the commands
//!   that check them (`cmp` and `ein saturate --dump`);
//! - a saved file reads no other file: a local module and the program gone
//!   from beside it, and `$EIN_STDLIB` pointing nowhere;
//! - its run is the part of its program's R above it, its universe H less
//!   the choices, with a `commit` row, and the program's claims NOT CHECKED,
//!   exit 0 under `ein solve` and `ein test` (Q-M1f.403); `ein show hyps`
//!   lists the program's H;
//! - the refusals: a choice that is no candidate of H, one named twice, a
//!   wrapper in a program, a saved file of another shape, an import it does
//!   not carry, and `ein kb save` of a node; and the note on a carried module
//!   of the standard library that is not the binary's.
//!
//! Every corpus program's every state file is held to the invariances by
//! `answer_round_trip.rs`, and every mode on a saved file to the oracle by
//! `ein-infer/tests/explore.rs`' `every_mode_is_the_oracle_above_a_saved_node`.

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein_with(args: &[&std::ffi::OsStr], env: &[(&str, &str)]) -> Run {
    let mut c = Command::new(env!("CARGO_BIN_EXE_ein"));
    c.args(args).current_dir(repo_root());
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn ein(args: &[&std::ffi::OsStr]) -> Run {
    ein_with(args, &[])
}

/// `ein …`, each argument a string.
macro_rules! ein {
    ($($a:expr),* $(,)?) => { ein(&[$(std::ffi::OsStr::new(&$a)),*]) };
}

/// The value of the read-out row `label`.
fn row<'a>(out: &'a str, label: &str) -> Option<&'a str> {
    out.lines().find_map(|l| {
        let rest = l.trim_start().strip_prefix(label)?;
        rest.starts_with(' ').then(|| rest.trim())
    })
}

/// `ein saturate --dump`'s KB section of `path`.
fn dumped(path: &Path) -> String {
    let r = ein!("saturate", "--dump", path.as_os_str());
    assert_eq!(r.code, 0, "{}: {}", path.display(), r.err);
    let at = r.out.find("\n=====").expect("a KB section");
    r.out[at..].to_string()
}

/// `ein solve FILE --dump-answer DIR`, and the state files it wrote, in
/// order.
fn saved(file: &str, dir: &Path) -> Vec<PathBuf> {
    let r = ein!("solve", file, "--dump-answer", dir.as_os_str());
    assert_eq!(r.code, 0, "{file}: {}", r.err);
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir.join("models"))
        .map(|d| d.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    v.sort();
    v
}

/// The two invariances on one saved file, as the commands that check them:
/// `ein solve F --no-search --dump-answer D` writes `D/root.ein`, which is F
/// byte for byte, and `ein saturate --dump` prints the same KB section of
/// both.
fn both_ways(file: &Path, scratch: &Path) {
    let again = scratch.join(format!(
        "again-{}",
        file.file_stem().unwrap_or_default().to_string_lossy()
    ));
    let r = ein!(
        "solve",
        file.as_os_str(),
        "--no-search",
        "--dump-answer",
        again.as_os_str()
    );
    assert_eq!(r.code, 0, "{}: {}", file.display(), r.err);
    let root = again.join("root.ein");
    assert_eq!(
        std::fs::read(file).expect("saved"),
        std::fs::read(&root).expect("saved again"),
        "save → load → save: {} again is not the same bytes",
        file.display()
    );
    assert_eq!(
        dumped(file),
        dumped(&root),
        "load → save → load: {} saved again loads to another dump",
        file.display()
    );
}

/// **Solve, save, load: one solution, the same facts.** A rule builds
/// `(note (bad a))`, a fact with a fact inside it no file writes: the run has
/// one state, its saved file is the program and no choice, and the file
/// loads to one state, root, holding the note as `make` derived it and no
/// `(false)`: the program's root, fact for fact and origin for origin.
#[test]
fn a_rule_built_fact_is_saved_and_loaded_as_one_solution() {
    let scratch = Scratch::new("ein-saved-01");
    let program = "tests/saved/01_a_rule_builds_a_fact.ein";
    let files = saved(program, &scratch.join("a"));
    assert_eq!(files.len(), 1, "one state");
    let text = std::fs::read_to_string(&files[0]).expect("saved");
    assert!(
        text.starts_with(ein_ir::saved::FIRST_LINE)
            && text.contains(
                &std::fs::read_to_string(repo_root().join(program)).expect("the program")
            )
            && !text.contains("(commit"),
        "the file is the program as written and no choice:\n{text}"
    );
    let r = ein!("solve", files[0].as_os_str());
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(row(&r.out, "states (k_s)"), Some("1"), "{}", r.out);
    let dump = dumped(&files[0]);
    assert!(
        dump.contains("(note (bad a) :rule make)") && !dump.contains("(false"),
        "{dump}"
    );
    assert_eq!(
        dump,
        dumped(&repo_root().join(program)),
        "the load is the program's root"
    );
    both_ways(&files[0], scratch.path());
}

/// **A choice with a fact inside it is committed, not written.** The state
/// `{(note (bad a))}` is saved as the program and `(commit (note (bad a)))`;
/// loaded, the choice is committed after root's fixpoint, its origin a
/// hypothesis, and `flag` derives `(flagged a)` from it: the run is the
/// part of the program's R above the node, one state, ∅ of its lattice, over
/// H less the choice, with the `commit` row, and the program's claims NOT
/// CHECKED, exit 0.
#[test]
fn a_choice_is_saved_as_a_commitment_and_loaded_as_the_node() {
    let scratch = Scratch::new("ein-saved-02");
    let program = "tests/saved/02_a_choice_with_a_fact_inside_it.ein";
    let files = saved(program, &scratch.join("a"));
    assert_eq!(files.len(), 2, "root and one state");
    let text = std::fs::read_to_string(&files[1]).expect("saved");
    assert!(
        text.ends_with(")\n(commit\n  (note (bad a)))\n"),
        "the state is the program and its choice:\n{text}"
    );
    let dump = dumped(&files[1]);
    assert!(
        dump.contains("(note (bad a) :hypothesis 0)") && dump.contains("(flagged a :rule flag)"),
        "{dump}"
    );
    let r = ein!("solve", files[1].as_os_str());
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(row(&r.out, "states (k_s)"), Some("1"), "{}", r.out);
    assert!(
        row(&r.out, "∅").is_some_and(|v| v.starts_with("in R"))
            && row(&r.out, "commit").is_some_and(|v| v.ends_with("— (note (bad a))"))
            && r.out.contains("NOT CHECKED — the claim is the program's"),
        "{}",
        r.out
    );
    let t = ein!("test", files[1].as_os_str());
    assert_eq!(
        t.code, 0,
        "a node's claims are not checked, exit 0:\n{}",
        t.out
    );
    assert!(t.out.contains("0 FAILED, 2 not checked"), "{}", t.out);
    // H is the program's: the node's choice among it.
    let h = ein!("show", "hyps", files[1].as_os_str());
    assert_eq!(
        (h.code, h.out.as_str()),
        (0, "(note (bad a))\n"),
        "{}",
        h.err
    );
    for f in &files {
        both_ways(f, scratch.path());
    }
}

/// **The standard library's twin, and a file that reads no other.** The
/// state of `03` carries the five modules its imports read, each as the
/// binary's standard library holds it, and loads with `$EIN_STDLIB` pointing
/// nowhere, to the program's root; and a program with a local module, saved,
/// loads with neither the program nor the module beside it.
#[test]
fn a_saved_file_reads_no_other_file() {
    let scratch = Scratch::new("ein-saved-03");
    let program = "tests/saved/03_the_standard_library.ein";
    let files = saved(program, &scratch.join("a"));
    assert_eq!(files.len(), 1, "one state");
    let text = std::fs::read_to_string(&files[0]).expect("saved");
    let modules: Vec<&str> = text
        .lines()
        .filter_map(|l| l.strip_prefix("(module "))
        .collect();
    assert_eq!(
        modules,
        [
            "std.math.arith",
            "std.math.decimal",
            "std.math.decimal-arith",
            "std.math.digits",
            "std.math.positional"
        ]
    );
    for m in &modules {
        let path = repo_root()
            .join("stdlib")
            .join(m.trim_start_matches("std.").replace('.', "/"))
            .with_extension("ein");
        let module = std::fs::read_to_string(&path).expect("the module");
        assert!(
            text.contains(&format!("(module {m}\n{module}\n)\n")),
            "{m} is carried as written"
        );
    }
    let want = dumped(&repo_root().join(program));
    assert_eq!(dumped(&files[0]), want, "the load is the program's root");
    let nowhere = scratch.join("no-stdlib");
    let r = ein_with(
        &["saturate".as_ref(), "--dump".as_ref(), files[0].as_os_str()],
        &[("EIN_STDLIB", nowhere.to_str().expect("utf-8"))],
    );
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.ends_with(want.as_str()),
        "with no standard library to read"
    );
    both_ways(&files[0], scratch.path());

    // A local module, and the program, gone from beside the saved file.
    let dir = scratch.join("local");
    std::fs::create_dir_all(&dir).expect("a directory");
    std::fs::write(
        dir.join("shelf.ein"),
        "(relation on-shelf Thing :why \"{?3} is on the shelf\")\n(on-shelf book)\n",
    )
    .expect("the module");
    let p = dir.join("p.ein");
    std::fs::write(
        &p,
        "(import shelf :symbols (on-shelf))\n\
         (relation read Thing :why \"{?3} is read\")\n\
         (rule reads () :match (on-shelf ?x) :assert (read ?x) :why \"{?x} on the shelf is read\")\n",
    )
    .expect("the program");
    let files = saved(p.to_str().expect("utf-8"), &dir.join("a"));
    assert!(!files.is_empty());
    let want = dumped(&files[0]);
    std::fs::remove_file(dir.join("shelf.ein")).expect("removed");
    std::fs::remove_file(&p).expect("removed");
    assert_eq!(dumped(&files[0]), want, "the module comes from the file");
    assert!(want.contains("(read book :rule reads)"), "{want}");
}

/// A saved file of `02`'s state with its `(commit …)` replaced.
fn with_commit(scratch: &Path, name: &str, commit: &str) -> PathBuf {
    let files = saved(
        "tests/saved/02_a_choice_with_a_fact_inside_it.ein",
        &scratch.join(format!("of-{name}")),
    );
    let text = std::fs::read_to_string(&files[1]).expect("saved");
    let (program, _) = text.rsplit_once("(commit\n").expect("a commitment");
    let path = scratch.join(format!("{name}.ein"));
    std::fs::write(&path, format!("{program}{commit}")).expect("written");
    path
}

/// **What is refused**, each at the command's load-error exit: a choice that
/// is no candidate of the program's H, or one named twice, on every command
/// that reads the node; a wrapper in a program; a saved file of another
/// shape; an import the file does not carry; `ein kb save` of a node. And a
/// carried module of the standard library that is not the binary's is
/// loaded as carried, with a `note:`.
#[test]
fn what_a_saved_file_is_refused_for() {
    let scratch = Scratch::new("ein-saved-refused");
    let no_candidate = with_commit(scratch.path(), "no-candidate", "(commit (note (bad b)))\n");
    let twice = with_commit(
        scratch.path(),
        "twice",
        "(commit (note (bad a)) (note (bad a)))\n",
    );
    for (file, line) in [
        (
            &no_candidate,
            "(commit …) names (note (bad b)), which is no candidate of its program's H — a saved \
             file's choices are candidates of H, which `ein show hyps` lists (Q-M1f.400)",
        ),
        (
            &twice,
            "(commit …) names (note (bad a)) twice — a commitment is a set of candidates of H, \
             each named once (Q-M1f.400)",
        ),
    ] {
        for args in [
            vec!["solve"],
            vec!["solve", "--no-search"],
            vec!["oracle"],
            vec!["saturate"],
            vec!["render", "kb"],
        ] {
            let mut argv: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_ref()).collect();
            argv.push(file.as_os_str());
            let r = ein(&argv);
            assert!(
                r.code == 1 && r.err.lines().any(|l| l == line),
                "{args:?} {}: exit {}\n{}",
                file.display(),
                r.code,
                r.err
            );
        }
    }
    let write = |name: &str, text: &str| {
        let p = scratch.join(name);
        std::fs::write(&p, text).expect("written");
        p
    };
    let cases = [
        (
            write("wrapper.ein", "(item a)\n(commit (item a))\n"),
            "kb load error: a program holds a (commit …) at None — (program …), (module …) and \
             (commit …) are a saved file's, which opens with its (program …)",
        ),
        (
            write(
                "second-commit.ein",
                "(program\n(item a)\n)\n(commit (item a))\n(commit (item a))\n",
            ),
            "kb load error: a second (commit …) at None — a saved file is one (program …), then \
             the (module …)s it carries, each once, then at most one (commit …), and nothing else",
        ),
        (
            write("a-fact.ein", "(program\n(item a)\n)\n(item b)\n"),
            "kb load error: a form that is no (program …), (module …) or (commit …) at None — a \
             saved file is one (program …), then the (module …)s it carries, each once, then at \
             most one (commit …), and nothing else",
        ),
    ];
    for (file, line) in &cases {
        let r = ein!("solve", file.as_os_str());
        assert_eq!((r.code, r.err.trim_end()), (1, *line), "{}", file.display());
    }
    let not_carried = write("not-carried.ein", "(program\n(import m)\n(item a)\n)\n");
    let r = ein!("solve", not_carried.as_os_str());
    assert!(
        r.code == 1
            && r.err.contains(&format!(
                "(import m) — {} carries no module m",
                not_carried.display()
            )),
        "{}",
        r.err
    );
    let r = ein!(
        "kb",
        "save",
        twice.as_os_str(),
        scratch.join("x.einb").as_os_str()
    );
    assert!(
        r.code == 1 && r.err.starts_with("kb save error: ") && r.err.contains("a node of"),
        "{}",
        r.err
    );
    // A carried module of the standard library that is not the binary's.
    let files = saved(
        "tests/saved/03_the_standard_library.ein",
        &scratch.join("of-03"),
    );
    let text = std::fs::read_to_string(&files[0]).expect("saved");
    let moved = write(
        "moved.ein",
        &text.replace(
            "(module std.math.arith\n",
            "(module std.math.arith\n;; moved\n",
        ),
    );
    let r = ein!("solve", moved.as_os_str());
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(
        r.err.trim_end(),
        format!(
            "note: {}: the std.math.arith it carries is not this binary's standard library's; \
             the text it carries is loaded",
            moved.display()
        )
    );
}

/// **`tests/saved/04_a_saved_node.ein` is `02`'s state saved**, byte for
/// byte what `ein solve tests/saved/02_… --dump-answer DIR` writes as
/// `models/02.ein`: the saved file the corpus reads as a program, every sweep
/// over the corpus reading it as the node it names. A change to `02` or to
/// the format fails here; save it again.
#[test]
fn the_corpus_saved_node_is_its_programs_save() {
    let scratch = Scratch::new("ein-saved-04");
    let files = saved(
        "tests/saved/02_a_choice_with_a_fact_inside_it.ein",
        &scratch.join("a"),
    );
    let corpus = repo_root().join("tests/saved/04_a_saved_node.ein");
    assert_eq!(
        std::fs::read_to_string(&files[1]).expect("saved"),
        std::fs::read_to_string(&corpus).expect("the corpus file"),
        "tests/saved/04_a_saved_node.ein is not 02's state as `ein solve --dump-answer` saves it: \
         copy DIR/models/02.ein over it"
    );
    let t = ein!("test", corpus.as_os_str());
    assert_eq!(t.code, 0, "{}", t.out);
}
