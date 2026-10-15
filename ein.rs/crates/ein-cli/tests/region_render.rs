//! `ein render region DIR` — M1e2
//! [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md).
//!
//! The drawing of a `--dump-region` directory, held from the outside: every
//! view is DOT Graphviz accepts (`dot_wellformed.rs`'s standard), `auto` picks
//! the views the stage measured, and the overlay names what the register's
//! witnesses show — W1's unrecorded maximal node, W3's unsound lookahead, W2's
//! cascade moving the search's root, and a commitment whose saturation lands
//! on a line node counting as that node.

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

/// `ein solve FILE FLAGS --dump-region DIR`, which has to succeed.
fn dumped(file: &str, flags: &[&str], dir: &Path) {
    let d = dir.to_string_lossy();
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    args.extend(["--dump-region", &d]);
    let r = ein(&args);
    assert!(dir.join("walk.json").is_file(), "{file}: {}", r.err);
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

/// The DOT statement drawing node `mask`.
fn node_line(dot: &str, mask: i64) -> &str {
    let head = format!("  m{mask} [label=");
    dot.lines()
        .find(|l| l.starts_with(&head))
        .unwrap_or_else(|| panic!("node m{mask} is not drawn"))
}

/// The cube's bit of each candidate.
fn mask_of(dir: &Path, facts: &[&str]) -> i64 {
    let region = json(&dir.join("region.json"));
    let cands: Vec<&str> = region["candidates"]
        .as_array()
        .expect("candidates")
        .iter()
        .map(|c| c.as_str().expect("text"))
        .collect();
    facts.iter().fold(0, |acc, f| {
        let b = cands
            .iter()
            .position(|c| c == f)
            .unwrap_or_else(|| panic!("{f} is not a candidate of {cands:?}"));
        acc | 1 << b
    })
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
    ("examples/branching/15_lookahead_two_step_on.ein", &["-e"]),
    ("examples/lattice/04_obligation_trigger_hole.ein", &["-e"]),
    ("examples/ein-bugs/wrong-answers/naf-nowb.ein", &["-e"]),
    ("examples/ein-bugs/wrong-answers/forced-2.ein", &["-e"]),
    ("examples/ein-bugs/wrong-answers/owed-seats.ein", &["-e"]),
    ("examples/saturation/transitive/taxonomy.ein", &["-e"]),
    ("examples/branching/05_mini_zebra.ein", &["-e"]),
    ("examples/saturation/type-exclusivity/colors.ein", &[]),
    (
        "examples/lattice/02_genuine_3set_death.ein",
        &["-e", "--region-limit", "2"],
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

/// **W1, drawn.** On `branching/15` the maximal-line node `{(p A)}` holds no
/// recorded model's state: it is MISSING, red, and the recorded `{(p B),
/// (p C)}` — on the maximal line — is SPURIOUS against the minimal one only.
#[test]
fn w1_the_unrecorded_maximal_node_is_missing() {
    let root = scratch("w1");
    let dir = root.join("d");
    dumped(
        "examples/branching/15_lookahead_two_step_on.ein",
        &["-e"],
        &dir,
    );
    let dot = render(&dir, "full");
    let a = node_line(&dot, mask_of(&dir, &["(p A)"]));
    assert!(a.contains("MISSING (max)") && a.contains("#e31a1c"), "{a}");
    let bc = node_line(&dot, mask_of(&dir, &["(p B)", "(p C)"]));
    assert!(
        bc.contains("recorded model") && bc.contains("SPURIOUS (not min)"),
        "{bc}"
    );
    assert!(!bc.contains("not min, max"), "{bc}");
    let table = render(&dir, "table");
    assert!(
        table.contains(
            "- **maximal line** `{(p A)}`, `{(p B), (p C)}` — missing `{(p A)}`; spurious none"
        ),
        "{table}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **W3's mechanism, drawn.** On `naf-nowb` the lookahead kills `(p A)` before
/// the search, and the cube says `{(p A), (q A)}` is alive: the exclusion is
/// named unsound, with the superset it pruned, and the hole's repair edge
/// carries the fact that revives it.
#[test]
fn w3_an_unsound_exclusion_is_named_with_what_it_pruned() {
    let root = scratch("w3");
    let dir = root.join("d");
    dumped(
        "examples/ein-bugs/wrong-answers/naf-nowb.ein",
        &["-e"],
        &dir,
    );
    let dot = render(&dir, "full");
    let pa = mask_of(&dir, &["(p A)"]);
    let both = mask_of(&dir, &["(p A)", "(q A)"]);
    assert!(node_line(&dot, pa).contains("✗ unsound exclusion"));
    assert!(
        dot.contains(&format!(
            "  m{pa} -> m{both} [color=\"#ae017e\", style=dashed"
        )),
        "the unsound lookahead's edge to its alive superset:\n{dot}"
    );
    assert!(dot.contains("label=\"+(q A)\""), "the repair edge:\n{dot}");
    assert!(
        render(&dir, "table")
            .contains("1 unsound: lookahead `{(p A)}` (alive superset `{(p A), (q A)}`)")
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **W2, drawn.** The forced-positive cascade promotes `(q one)` before any
/// entering: `walk.json` times the promotion, the search's root moves onto
/// the dead `{(q one)}`, and `∅` — the whole of R — is missing from both lines.
#[test]
fn w2_the_cascade_moves_the_search_root() {
    let root = scratch("w2");
    let dir = root.join("d");
    dumped(
        "examples/ein-bugs/wrong-answers/forced-2.ein",
        &["-e"],
        &dir,
    );
    let walk = json(&dir.join("walk.json"));
    assert_eq!(walk["promotions"][0]["fact"], "(q one)");
    assert_eq!(walk["promotions"][0]["after_enterings"], 0);
    let dot = render(&dir, "full");
    let q = node_line(&dot, mask_of(&dir, &["(q one)"]));
    assert!(
        q.contains("root after the cascade") && q.contains("dead"),
        "{q}"
    );
    let empty = node_line(&dot, 0);
    assert!(empty.contains("MISSING (min, max)"), "{empty}");
    let _ = std::fs::remove_dir_all(&root);
}

/// **Saturation is a move inside the cube.** On `05_mini_zebra` the recorded
/// commitment derives a third candidate: its node is transient, a `sat` arrow
/// goes to the node its fixpoint is, that node is *reached by saturation* —
/// and, compared by state, the maximal line is recorded, not missing.
#[test]
fn a_recorded_commitment_counts_as_the_node_its_saturation_lands_on() {
    let root = scratch("moves");
    let dir = root.join("d");
    dumped("examples/branching/05_mini_zebra.ein", &["-e"], &dir);
    let region = json(&dir.join("region.json"));
    let walk = json(&dir.join("walk.json"));
    let at = walk["recorded"][0]["state_mask"]
        .as_i64()
        .expect("in the cube");
    let node = &region["region"]["nodes"][at as usize];
    let closure = node["closure"].as_i64().unwrap();
    assert_ne!(closure, at, "the recorded commitment's saturation moves it");
    let top = &region["region"]["nodes"][closure as usize];
    assert_eq!(top["maximal"], true);
    assert_eq!(top["class"], node["class"], "one state");
    let dot = render(&dir, "lines");
    assert!(
        dot.contains(&format!("  m{at} -> m{closure} [color=\"#08519c\"")),
        "the sat arrow:\n{dot}"
    );
    assert!(node_line(&dot, at).contains("style=\"filled,dashed\""));
    let landed = node_line(&dot, closure);
    assert!(landed.contains("reached by saturation"), "{landed}");
    assert!(!landed.contains("MISSING"), "{landed}");
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
