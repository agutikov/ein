//! **No surface recurses on a term's depth.** A rule that nests its
//! conclusion deeper than its premise makes a fact as deep as its firings
//! went, and every surface that prints a fact prints it whole, through one
//! walk of a term that keeps a stack of its own, never the call stack
//! (`ein_core::Terms::walk_term`; `03-design/03-data-model.md` § 3).
//!
//! | claim | test |
//! |---|---|
//! | `ein saturate --max-steps` stops `tests/bounded/grow.ein`, which has no fixpoint, at a hundred thousand firings, exit 0, its step-limit line naming the last firing's terms whole | [`the_step_limit_line_names_a_term_past_any_call_stack`] |
//! | a program whose fixpoint holds a fact far deeper than a call stack takes is answered, and its fact printed whole, by every surface that prints a fact: the read-out, `-p`, `--single`, `--json-summary`, `--events`, `--dump-answer`, `--dump-lattice`, `ein saturate --dump`, `ein oracle --dump`, `ein kb save` and the container, a refuted claim's block and `ein test --json-report` | [`every_surface_prints_a_fact_past_any_call_stack`] |
//! | the same surfaces, the knowledge-base view of the program and of a state it saved, and a trace with its snapshots, on a main thread of 256 KiB (`ulimit -s`), where a walk that recursed on the depth would overflow long before the depth the program reaches | [`no_surface_recurses_on_a_terms_depth`] |
//!
//! The programs are written here, few facts deep rather than many shallow:
//! a surface that prints every fact prints a chain of facts in the square of
//! its length, so the depth is reached in a few dozen firings, each nesting
//! its conclusion some hundreds of levels deeper, which the parser's limit of
//! 1 000 parentheses lets a rule write.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
    err: String,
}

/// `ein ARGS` in `dir`, under a main thread of `stack` KiB where one is given
/// (`ulimit -s`, which sets the stack of the process's main thread).
fn ein_in(dir: &Path, args: &[&str], stack: Option<u32>) -> Run {
    let bin = env!("CARGO_BIN_EXE_ein");
    let mut cmd = match stack {
        None => {
            let mut c = Command::new(bin);
            c.args(args);
            c
        }
        Some(kib) => {
            let mut c = Command::new("sh");
            c.arg("-c")
                .arg(format!("ulimit -s {kib} && exec \"$0\" \"$@\""))
                .arg(bin)
                .args(args);
            c
        }
    };
    let out = cmd
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// `zero` nested `depth` deep under `s`, as the one printer prints it.
fn nested(depth: usize) -> String {
    format!("{}zero{}", "(s ".repeat(depth), ")".repeat(depth))
}

/// A chain of `steps` steps and a rule that nests its count `per` levels
/// deeper at each, so that root's fixpoint holds `(at aN t)` with `t`
/// `steps × per` deep; `(solve :no-search)` keeps the blind enumerator off
/// the chain's names. Under `refuted`, the query claims no answer, so that
/// `ein test` refutes it and prints the answer it found.
fn program(steps: usize, per: usize, refuted: bool) -> String {
    let mut p = String::from(
        "(relation at N N :why \"{?3} is reached by {?4}\")\n\
         (relation step N N :why \"{?3} steps to {?4}\")\n\
         (rule deeper ()\n\
         \x20 :match (and (at ?a ?t) (step ?a ?b))\n",
    );
    p.push_str(&format!(
        "  :assert (at ?b {}?t{})\n",
        "(s ".repeat(per),
        ")".repeat(per)
    ));
    p.push_str("  :why \"a step nests its count deeper\")\n(at a0 zero)\n");
    for i in 0..steps {
        p.push_str(&format!("(step a{i} a{})\n", i + 1));
    }
    let expect = if refuted { " :expect (false)" } else { "" };
    p.push_str(&format!(
        "(query :goal (at a{steps} ?t) :goal-text \"{{?t}}\"{expect})\n(solve :no-search)\n"
    ));
    p
}

/// Whether `printed` holds a term `depth` deep whole, as the one printer
/// writes it.
fn written(printed: &str, depth: usize) -> bool {
    printed.contains(&nested(depth))
}

/// Whether `printed` holds it as a fact's label writes it, `s(s(…zero…))`:
/// a trace's text.
fn labelled(printed: &str, depth: usize) -> bool {
    printed.contains(&format!("{}zero{}", "s(".repeat(depth), ")".repeat(depth)))
}

/// Whether `printed` is a digraph that draws a term `depth` deep: a view draws
/// each nested fact as its own node, so its text holds no term whole.
fn drawn(printed: &str, depth: usize) -> bool {
    printed.starts_with("digraph") && printed.matches("shape=octagon").count() > depth
}

/// Whether the run wrote what it was asked to and printed nothing to read: a
/// container is bytes, and its read-out is the surface after it.
fn saved(_: &str, _: usize) -> bool {
    true
}

/// What a surface is asked, with `{OUT}` a directory of its own for what it
/// writes, the exit it takes, and what it prints of the deepest fact.
type Surface<'a> = (&'a str, &'a [&'a str], i32, fn(&str, usize) -> bool);

/// Every surface that prints a fact and costs no more than the text it
/// prints.
const SURFACES: [Surface; 8] = [
    (
        "the read-out and its artefacts",
        &[
            "solve",
            "p.ein",
            "--json-summary",
            "{OUT}/s.json",
            "--events",
            "{OUT}/e.jsonl",
            "--dump-answer",
            "{OUT}/answer",
            "--dump-lattice",
            "{OUT}/lattice",
        ],
        0,
        written,
    ),
    ("the final state", &["solve", "p.ein", "-p"], 0, written),
    ("one state", &["solve", "p.ein", "--single"], 0, written),
    (
        "ein saturate",
        &["saturate", "p.ein", "--dump", "--events", "{OUT}/e.jsonl"],
        0,
        written,
    ),
    (
        "the oracle",
        &["oracle", "p.ein", "--dump", "{OUT}/cube"],
        0,
        written,
    ),
    ("a container", &["kb", "save", "p.ein", "p.einb"], 0, saved),
    ("the container's read-out", &["solve", "p.einb"], 0, written),
    (
        "a refuted claim",
        &[
            "test",
            "refuted.ein",
            "--diff",
            "full",
            "--json-report",
            "{OUT}/r.json",
        ],
        1,
        written,
    ),
];

/// What a surface printed: its stdout, and every file it wrote under its
/// directory.
fn printed(r: &Run, out: &Path) -> String {
    let mut text = r.out.clone();
    let mut stack = vec![out.to_path_buf()];
    while let Some(d) = stack.pop() {
        for p in std::fs::read_dir(&d)
            .expect("a directory")
            .filter_map(|e| e.ok().map(|e| e.path()))
        {
            if p.is_dir() {
                stack.push(p);
            } else {
                text.push_str(&String::from_utf8_lossy(&std::fs::read(&p).expect("read")));
            }
        }
    }
    text
}

/// Ask every surface of [`SURFACES`], then `extra`, of the program of
/// `steps` steps of `per` levels, under `stack`; each must exit as it does on
/// any program, crash nowhere, and print the deepest fact whole, a term
/// `steps × per` deep, as it prints a fact.
fn every_surface(steps: usize, per: usize, stack: Option<u32>, extra: &[Surface]) {
    let scratch = Scratch::new("ein-deep-terms");
    let dir = scratch.path();
    std::fs::write(dir.join("p.ein"), program(steps, per, false)).expect("written");
    std::fs::write(dir.join("refuted.ein"), program(steps, per, true)).expect("written");
    let depth = steps * per;
    let mut bad = Vec::new();
    for (k, &(name, args, code, holds)) in SURFACES.iter().chain(extra).enumerate() {
        let out = dir.join(format!("out-{k}"));
        std::fs::create_dir_all(&out).expect("a directory");
        let at = out.to_str().expect("a path").to_string();
        let args: Vec<String> = args.iter().map(|a| a.replace("{OUT}", &at)).collect();
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein_in(dir, &args, stack);
        if r.code != code || r.err.contains("overflow") || r.err.contains("panicked") {
            bad.push(format!(
                "{name}: exit {}, its own is {code}; stderr {:?}",
                r.code,
                r.err.chars().take(300).collect::<String>()
            ));
        } else if !holds(&printed(&r, &out), depth) {
            bad.push(format!(
                "{name}: printed no fact {depth} deep whole, on stdout or in what it wrote"
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The step-limit line names a term past any call stack.** `grow.ein`
/// nests its conclusion one level deeper at each firing and has no fixpoint;
/// stopped at a hundred thousand firings, its last binds `?x` to a term
/// 99 999 deep, which the line prints whole, at exit 0.
#[test]
fn the_step_limit_line_names_a_term_past_any_call_stack() {
    let grow = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/bounded/grow.ein");
    let r = ein_in(
        &repo_root(),
        &[
            "saturate",
            "--max-steps",
            "100000",
            grow.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(r.code, 0, "{}", r.err.chars().take(300).collect::<String>());
    let want = format!(
        "!! saturator step limit hit: --max-steps 100000 stopped saturation after 100000 \
         firings, before it checked for a fixpoint. The last firing: (grow) with ?x = {} \
         derived (num {}) from (num {})",
        nested(99_999),
        nested(100_000),
        nested(99_999)
    );
    assert!(
        r.out.lines().any(|l| l == want),
        "no step-limit line naming the last firing whole"
    );
}

/// **Every surface prints a fact past any call stack**: forty steps of 990
/// levels, a fact 39 600 deep, on the main thread's own stack.
#[test]
fn every_surface_prints_a_fact_past_any_call_stack() {
    every_surface(40, 990, None, &[]);
}

/// **No surface recurses on a term's depth**: twenty steps of a hundred
/// levels, a fact 2 000 deep, on a main thread of 256 KiB, where a walk that
/// recursed on the depth overflowed at a fraction of it; and beside the
/// surfaces above, the knowledge-base view of the program and of the state
/// its read-out saved, and a trace with its knowledge-base snapshots, which
/// draw every nested fact as its node, at a cost that grows with the square of
/// the depth.
#[test]
fn no_surface_recurses_on_a_terms_depth() {
    every_surface(
        20,
        100,
        Some(256),
        &[
            (
                "the knowledge-base view",
                &["render", "kb", "p.ein"],
                0,
                drawn,
            ),
            (
                "a saved state's view",
                &["render", "kb", "out-0/answer/models/01.ein"],
                0,
                drawn,
            ),
            (
                "a trace with its snapshots",
                &["solve", "p.ein", "-r", "{OUT}/trace.md", "-F"],
                0,
                labelled,
            ),
        ],
    );
}
