//! `ein render region DIR` — M1e2
//! [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md).
//!
//! The drawing of an `ein oracle --dump` directory, held from the outside:
//! every view is DOT Graphviz accepts (`dot_wellformed.rs`'s standard), and
//! `auto` picks the views the stage measured.
//!
//! Until M1e2 S1e2.1a.5 the directory was `ein solve --dump-region`'s and
//! carried the walk's `walk.json`, and four more tests here held the overlay
//! to what the register's witnesses showed — W1's unrecorded maximal node,
//! W3's unsound lookahead, W2's cascade moving the search's root, a commitment
//! whose saturation lands on a line node. No command runs the walk since the
//! switch, so nothing writes the file they read; drawing a region run on the
//! cube is [S1e2.1a.7](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md)'s,
//! and the four are in git history for it.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use ein_corpus::repo_root;
use serde_json::Value as J;

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
    let dir = std::env::temp_dir().join(format!("ein-region-render-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch");
    dir
}

/// `ein oracle FILE FLAGS --dump DIR`, which has to write the directory —
/// over the limit too, where the oracle refuses and `region.json` says why.
fn dumped(file: &str, flags: &[&str], dir: &Path) {
    let d = dir.to_string_lossy();
    let mut args = vec!["oracle", file];
    args.extend_from_slice(flags);
    args.extend(["--dump", &d]);
    let r = ein(&args);
    assert!(dir.join("region.json").is_file(), "{file}: {}", r.err);
}

fn render(dir: &Path, view: &str) -> String {
    let d = dir.to_string_lossy();
    let r = ein(&["render", "region", &d, "--view", view]);
    assert_eq!(r.code, 0, "render region {view}: {}", r.err);
    r.out
}

fn json(path: &Path) -> J {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The view `auto` resolved to, from the DOT's second line.
fn view_of(dot: &str) -> &str {
    dot.lines()
        .nth(1)
        .and_then(|l| l.split("view ").nth(1))
        .unwrap_or("?")
}

fn require_graphviz() {
    let ok = Command::new("dot")
        .arg("-V")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    assert!(
        ok,
        "`dot -V` does not run: this test needs Graphviz, the only authority \
         a DOT view has on being well-formed (dot_wellformed.rs)"
    );
}

fn graphviz_accepts(dot: &str) -> Result<(), String> {
    let mut child = Command::new("dot")
        .args(["-Tsvg", "-o", "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("dot runs");
    child
        .stdin
        .take()
        .expect("piped")
        .write_all(dot.as_bytes())
        .expect("dot reads");
    let out = child.wait_with_output().expect("dot finishes");
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

/// The programs every view is drawn for: a hole, an unsound lookahead, a
/// forced positive, saturation moving nodes, an owing root, the n = 9 border
/// view and the n = 8 lines view — and one directory over the limit.
const DRAWN: [(&str, &[&str]); 9] = [
    ("examples/branching/15_lookahead_two_step_on.ein", &[]),
    ("examples/lattice/04_obligation_trigger_hole.ein", &[]),
    ("examples/ein-bugs/wrong-answers/naf-nowb.ein", &[]),
    ("examples/ein-bugs/wrong-answers/forced-2.ein", &[]),
    ("examples/ein-bugs/wrong-answers/owed-seats.ein", &[]),
    ("examples/saturation/transitive/taxonomy.ein", &[]),
    ("examples/branching/05_mini_zebra.ein", &[]),
    ("examples/saturation/type-exclusivity/colors.ein", &[]),
    (
        "examples/lattice/02_genuine_3set_death.ein",
        &["--region-limit", "2"],
    ),
];

/// **Every view is DOT Graphviz accepts**, `table` is markdown with one
/// section per layer, and `auto` is the view the stage measured: the whole
/// cube to n = 6, R and its border to 64 nodes, the lines past that.
#[test]
fn every_view_is_drawn_and_auto_picks_the_measured_one() {
    require_graphviz();
    let root = scratch("views");
    let mut autos = Vec::new();
    for (i, (file, flags)) in DRAWN.iter().enumerate() {
        let dir = root.join(format!("{i}"));
        dumped(file, flags, &dir);
        for view in ["auto", "full", "border", "lines"] {
            let dot = render(&dir, view);
            if let Err(e) = graphviz_accepts(&dot) {
                panic!("{file} --view {view}: Graphviz refused it:\n{e}");
            }
            if view == "auto" {
                autos.push((*file, view_of(&dot).to_string()));
            }
        }
        let table = render(&dir, "table");
        assert!(table.starts_with("# Region · "), "{file}: {table}");
        let refused = json(&dir.join("region.json"))["region"].is_null();
        if !refused {
            let n = json(&dir.join("region.json"))["n"].as_u64().unwrap();
            assert_eq!(
                table.matches("\n## L").count() as u64,
                n + 1,
                "{file}: one section per layer"
            );
        }
    }
    let auto = |f: &str| {
        autos
            .iter()
            .find(|(x, _)| x.ends_with(f))
            .map(|(_, v)| v.as_str())
            .unwrap()
    };
    assert_eq!(auto("owed-seats.ein"), "full");
    assert_eq!(auto("05_mini_zebra.ein"), "border");
    assert_eq!(auto("colors.ein"), "lines");
    assert!(auto("02_genuine_3set_death.ein").starts_with("none"));
    let _ = std::fs::remove_dir_all(&root);
}

/// **A directory that is not a dump** is an error naming the file, exit 1.
#[test]
fn a_directory_without_the_files_is_an_error() {
    let root = scratch("empty");
    let r = ein(&["render", "region", &root.to_string_lossy()]);
    assert_eq!(r.code, 1);
    assert!(r.err.contains("region.json"), "{}", r.err);
    assert!(r.out.is_empty());
    let _ = std::fs::remove_dir_all(&root);
}
