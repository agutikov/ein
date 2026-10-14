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
//! ein solve FILE --dump-lattice DIR …                        every commitment tested
//! ```
//!
//! The renderers' own standard is `ein-render/tests/dot_wellformed.rs`: what
//! they emit, `dot` accepts. This extends it from the renderer to the door —
//! the bytes the *command* prints go through `dot -Tsvg` — and holds the
//! one claim each door adds of its own.

use std::io::Write;
use std::path::{Path, PathBuf};
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
    // The third door writes files, and the DOTs among them are the lattice's.
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-e",
        "--dump-lattice",
        &lattice.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "--dump-lattice: {}", r.err);
    assert!(lattice.join("proof_summary.json").is_file());
    assert!(lattice.join("enterings").is_dir());
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
/// wrote: a model's state against root's, one `Terms` for both, so a fact
/// root also held is drawn plain and one it lacked is drawn at `penwidth=3`.
#[test]
fn render_kb_since_root_emboldens_what_the_commitment_added() {
    let dir = scratch("since").join("answer");
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let r = ein(&["solve", file, "-e", "--dump-answer", &dir.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    let model = dir.join("models/01.ein");
    let root = dir.join("root.ein");
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
    // Model 1 seats two guests: two `seats` edges root did not have.
    assert_eq!(
        bold.iter().filter(|l| l.contains("seats")).count(),
        2,
        "the bold edges are not the commitment's:\n{}",
        bold.join("\n")
    );
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
}

/// The clock fields, masked — `golden_dump.rs`'s list.
fn mask_clock(text: &str) -> String {
    let mut out = text.to_string();
    for key in ["\"ts_ms\": ", "\"elapsed_seconds\": ", "\"wall_seconds\": "] {
        while let Some(at) = out.find(key) {
            let start = at + key.len();
            let end = out[start..]
                .find(|c: char| !matches!(c, '0'..='9' | '.' | 'e' | 'E' | '+' | '-'))
                .map_or(out.len(), |j| start + j);
            out.replace_range(start..end, "<ts>");
            out.replace_range(at..at + 1, "\u{1}");
        }
        out = out.replace('\u{1}', "\"");
    }
    out
}

fn tree(dir: &Path) -> Vec<(String, String)> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
        for e in std::fs::read_dir(dir).expect("readdir").flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(base, &p, out);
            } else {
                let rel = p.strip_prefix(base).unwrap().to_string_lossy().into_owned();
                out.push((
                    rel,
                    mask_clock(&std::fs::read_to_string(&p).unwrap_or_default()),
                ));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

/// **The CLI's tree is the library's tree.** `golden_dump.rs` pins what a
/// `LatticeDumper` writes when a Rust caller hands it to `solve`; this runs the
/// same fixture through the new flag and through the library call, and the two
/// directories are the same files with the same bytes, clocks aside.
#[test]
fn dump_lattice_writes_the_tree_the_library_writes() {
    use ein_core::Terms;
    use ein_infer::solve::{SolveOptions, solve};
    use ein_ir::{Ast, load_file};

    let file = "examples/lattice/01_subset_pruned.ein";
    let dir = scratch("lattice");
    let cli = dir.join("cli");
    let r = ein(&[
        "solve",
        file,
        "-e",
        "-m",
        "3",
        "--dump-lattice",
        &cli.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);

    let lib = dir.join("lib");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(file)).expect("it loads");
    let mut dumper = ein_render::dump::LatticeDumper::new(Some(&lib)).expect("a dumper");
    let config = kb.program().config.clone().unwrap_or_default();
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        config: Some(config),
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    solve(&mut kb, &mut terms, &ast, &mut events, &mut dumper, &opts).expect("it solves");

    let (a, b) = (tree(&cli), tree(&lib));
    let names = |t: &[(String, String)]| t.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>();
    assert_eq!(names(&a), names(&b), "the two trees hold different files");
    for ((name, x), (_, y)) in a.iter().zip(&b) {
        assert_eq!(x, y, "{name} differs between the CLI and the library");
    }
    assert!(
        a.iter().any(|(n, _)| n.starts_with("enterings/")),
        "no enterings/"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **One dumper per solve**, so the flags that each choose one are refused
/// together with a reason, at exit 2 — where `-t` used to take `--dump-states`'
/// directory and drop it.
#[test]
fn the_flags_that_choose_a_dumper_are_refused_together() {
    let dir = scratch("conflict");
    let d = dir.join("d").to_string_lossy().into_owned();
    for other in [
        vec!["--dump-states", "x"],
        vec!["-v"],
        vec!["--layer-progress"],
        vec!["-t"],
    ] {
        let mut args = vec!["solve", "examples/zebra2.ein", "--dump-lattice", &d];
        args.extend(other.iter().copied());
        let r = ein(&args);
        assert_eq!(r.code, 2, "{other:?}: {}", r.err);
        assert!(r.err.contains("--dump-lattice"), "{other:?}: {}", r.err);
        assert!(r.out.is_empty(), "{other:?} ran anyway");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// **`-t` with `--dump-states` writes the dump.** It took the timing arm and
/// wrote nothing, exit 0, until M1e2 S1e2.1.3; the tree is now the one a plain
/// `--dump-states` writes, file for file.
#[test]
fn timing_and_dump_states_both_happen() {
    let dir = scratch("timing");
    let (with, plain) = (dir.join("with"), dir.join("plain"));
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-t",
        "--dump-states",
        &with.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.contains("root saturation"),
        "the timing table is gone"
    );
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "--dump-states",
        &plain.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let names = |d: &Path| tree(d).into_iter().map(|(n, _)| n).collect::<Vec<_>>();
    assert_eq!(names(&with), names(&plain));
    assert!(!names(&with).is_empty(), "-t --dump-states wrote nothing");
    let _ = std::fs::remove_dir_all(&dir);
}
