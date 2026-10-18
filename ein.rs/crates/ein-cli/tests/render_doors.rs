//! M1e2 S1e2.1.3 — **three renderers get a door**, and Q-M1e.20 is answered.
//!
//! [Q-M1e.20](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable)
//! counted renderers that were built, tested and reachable from no command:
//! `ein_render::kb_dot`, the `LatticeDumper` tree, and — `utils/README.md`'s
//! third — `ein_render::ir_dot`. Its open question was *who wants these*, and
//! M1e2's answer is the reviewer of a wrong answer. So:
//!
//! ```text
//! ein render program FILE [--levi] [--rule-mode …]          the program as written
//! ein render kb FILE [--since FILE] [--colour-by …] …        a KB, one digraph
//! ein solve FILE --dump-lattice DIR …                        every node the run visited
//! ein render lattice FILE [--view full]                      those nodes drawn
//! ```
//!
//! The third door was the walk's `LatticeDumper` tree until M1e2 S1e2.1a.5's
//! switch; it is a region run's visited nodes since (`ein-lattice/2`,
//! [`lattice_dump.md`](../../../../docs/kernel/inference/lattice_dump.md)), and
//! `ein render lattice` draws them.
//!
//! The renderers' own standard is `ein-render/tests/dot_wellformed.rs`: what
//! they emit, `dot` accepts. This extends it from the renderer to the door —
//! the bytes the *command* prints go through `dot -Tsvg` — and holds the
//! one claim each door adds of its own.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use ein_corpus::repo_root;

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

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-render-doors-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch");
    dir
}

/// `dot -Tsvg` over the text, as `dot_wellformed.rs` does: Graphviz is a hard
/// dependency of the gate, so its absence fails rather than skips.
fn dot_accepts(what: &str, text: &str) {
    let mut child = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Graphviz `dot` on PATH — a hard dependency of the gate");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(text.as_bytes())
        .expect("dot reads its input");
    let out = child.wait_with_output().expect("dot runs");
    assert!(
        out.status.success(),
        "{what}: dot refused it:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn every_door_prints_dot_that_dot_draws() {
    let dir = scratch("dot");
    let lattice = dir.join("lattice");
    for (what, args) in [
        (
            "render program",
            vec!["render", "program", "examples/zebra2.ein"],
        ),
        (
            "render program --levi",
            vec!["render", "program", "examples/zebra2.ein", "--levi"],
        ),
        ("render kb", vec!["render", "kb", "examples/zebra2.ein"]),
        (
            "render kb --colour-by origin",
            vec![
                "render",
                "kb",
                "examples/zebra2.ein",
                "--colour-by",
                "origin",
            ],
        ),
    ] {
        let r = ein(&args);
        assert_eq!(r.code, 0, "{what}: {}", r.err);
        assert!(r.out.contains("digraph"), "{what} printed no digraph");
        dot_accepts(what, &r.out);
    }
    // The third door writes files, and the fourth draws the same nodes.
    for view in ["solution", "full"] {
        let r = ein(&[
            "render",
            "lattice",
            "examples/ein-bugs/wrong-answers/owed-seats.ein",
            "--view",
            view,
        ]);
        assert_eq!(r.code, 0, "render lattice --view {view}: {}", r.err);
        dot_accepts(&format!("render lattice --view {view}"), &r.out);
    }
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-n",
        "1",
        "--dump-lattice",
        &lattice.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "--dump-lattice: {}", r.err);
    assert!(lattice.join("nodes.json").is_file());
    assert!(lattice.join("nodes.md").is_file());
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The program is drawn before it is loaded**, so a program the loader
/// refuses still has a picture — which is the case a reviewer of a broken
/// fixture needs most.
#[test]
fn render_program_draws_what_the_loader_refuses() {
    let file = "examples/broken/load/derivation_cycle.ein";
    assert_ne!(ein(&["saturate", file]).code, 0, "{file} loads now");
    let r = ein(&["render", "program", file]);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts(file, &r.out);
}

/// **`--since` draws what a commitment added**, from the files `--dump-answer`
/// wrote: a state against ∅'s, one `Terms` for both, so a fact ∅ also held is
/// drawn plain and one it lacked is drawn at `penwidth=3`. Since M1e2
/// S1e2.1a.5 ∅'s file is `models/01.ein` where ∅ is in R — the walk wrote a
/// `root.ein` beside its models.
#[test]
fn render_kb_since_root_emboldens_what_the_commitment_added() {
    let dir = scratch("since").join("answer");
    let file = "examples/features/11_expect_ambiguity.ein";
    let r = ein(&["solve", file, "--dump-answer", &dir.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    // State 1 is ∅; state 6 seats both guests.
    let root = dir.join("models/01.ein");
    let model = dir.join("models/06.ein");
    let plain = ein(&["render", "kb", &model.to_string_lossy()]);
    assert_eq!(plain.code, 0, "{}", plain.err);
    assert!(
        !plain.out.contains("penwidth=3"),
        "nothing to embolden without --since"
    );
    let r = ein(&[
        "render",
        "kb",
        &model.to_string_lossy(),
        "--since",
        &root.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts("render kb --since", &r.out);
    let bold: Vec<&str> = r.out.lines().filter(|l| l.contains("penwidth=3")).collect();
    // State 6 seats two guests: two `seat` edges ∅ did not have.
    assert_eq!(
        bold.iter().filter(|l| l.contains("seat")).count(),
        2,
        "the bold edges are not the commitment's:\n{}",
        bold.join("\n")
    );
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
}

/// **The dump is the run's nodes.** `nodes.json` lists, in visit order,
/// exactly the nodes the run's `--events` stream enters, with the kind each
/// was classified as — so the drawing and the stream are one account of the
/// run. The walk's door was held to the library's `LatticeDumper` tree file
/// for file; that tree has no CLI caller since the switch.
#[test]
fn dump_lattice_lists_the_nodes_the_run_entered() {
    let dir = scratch("lattice");
    let d = dir.join("d");
    let ev = dir.join("e.jsonl");
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let r = ein(&[
        "solve",
        file,
        "--dump-lattice",
        &d.to_string_lossy(),
        "--events",
        &ev.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let nodes: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(d.join("nodes.json")).unwrap()).unwrap();
    assert_eq!(nodes["schema"], "ein-lattice/2");
    let entered = std::fs::read_to_string(&ev)
        .unwrap()
        .lines()
        .filter(|l| l.contains("\"e\": \"enter\""))
        .count();
    assert_eq!(
        nodes["visited"], entered as u64,
        "the dump and the stream disagree"
    );
    assert_eq!(nodes["nodes"].as_array().unwrap().len(), entered);
    let in_r = nodes["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| n["kind"] == "in-r")
        .count();
    assert_eq!(in_r, 7, "owed-seats' R is seven states, each visited once");
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The dump moves no answer, beside any other flag.** The walk had one
/// dumper per solve and refused `--dump-lattice` with `-v`, `--layer-progress`,
/// `-t` and `--dump-states`; a region run's dump is a reader of the run, so
/// it combines with each and stdout is the same with it and without.
/// `--dump-states` is retired, and says what replaced it.
#[test]
fn the_dump_combines_with_the_other_flags() {
    let dir = scratch("combine");
    let d = dir.join("d").to_string_lossy().into_owned();
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    for other in [vec!["-v"], vec!["--layer-progress"], vec!["-s"]] {
        let mut with = vec!["solve", file, "--dump-lattice", &d];
        with.extend(other.iter().copied());
        let mut without = vec!["solve", file];
        without.extend(other.iter().copied());
        let (a, b) = (ein(&with), ein(&without));
        assert_eq!((a.code, b.code), (0, 0), "{other:?}: {}", a.err);
        let steady = |o: &str| {
            o.lines()
                .filter(|l| !l.trim_start().starts_with("wall"))
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(steady(&a.out), steady(&b.out), "{other:?} moved stdout");
        assert_eq!(a.err, b.err, "{other:?} moved stderr");
    }
    let r = ein(&["solve", file, "-t", "--dump-lattice", &d]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("\ntiming\n"), "the timing table is gone");
    let r = ein(&["solve", file, "--dump-states", &d]);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(
        r.err.contains("-D/--dump-states is retired") && r.err.contains("--dump-lattice DIR"),
        "{}",
        r.err
    );
    assert!(r.out.is_empty(), "a retired flag ran anyway");
    let _ = std::fs::remove_dir_all(&dir);
}
