//! `ein render region DIR [--visited DIR] [--answer DIR]` — M1e2
//! [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md)
//! and [S1e2.1a.7](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md).
//!
//! The drawing of an `ein oracle --dump` directory with a region run on it,
//! held from the outside: every view is DOT Graphviz accepts
//! (`dot_wellformed.rs`'s standard), with and without a run, and `auto` picks
//! the views S1e2.1a.2 measured.
//!
//! Until M1e2 S1e2.1a.5 the run was the walk's (`walk.json`), and four tests
//! here held the overlay to what the register's witnesses showed. S1e2.1a.7
//! brought them back against a region run's dumps (`--dump-lattice`,
//! `--dump-answer`): W1's maximal node that no reported state holds is
//! MISSING, W3's hole and an exclusion the cube refutes, W2's root that a
//! region run never moves, and a state that counts as the node its
//! saturation lands on. A correct engine misses nothing, so where a test
//! needs a wrong answer it writes one — the walk's, from the register — into
//! a copy of the run's dump, which is the input the comparison is for.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use ein_corpus::{Scratch, repo_root};
use ein_render::dump::region_lattice::{self, VisitedNode, Visits};
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

fn scratch(tag: &str) -> Scratch {
    Scratch::new(&format!("ein-region-render-{tag}"))
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

/// What a region run visits, as `--dump-lattice`'s watch collects it.
struct Collect(Arc<Mutex<Visits>>);

impl ein_infer::events::Watch for Collect {
    fn node(&mut self, v: &ein_infer::events::Visited<'_>) {
        self.0.lock().expect("one watcher").nodes.push(VisitedNode {
            commitment: v.commitment.to_vec(),
            kind: v.kind,
            owes: v.owes,
            owed: v.owed.to_vec(),
            core: v.core.to_vec(),
            refuted_by: v.refuted_by.to_vec(),
            closure: v.closure.to_vec(),
            facts: v.facts,
        });
    }

    fn step(&mut self, s: &ein_infer::events::Stepped) {
        let mut v = self.0.lock().expect("one watcher");
        let before = v.steps.iter().map(|(x, _)| x.visited as usize).sum();
        v.steps.push((*s, before));
    }
}

/// `file`'s cube in `cube` and its `--maximal` run's `answer/` and
/// `lattice/` in `run`, written by the writers `ein oracle --dump` and `ein
/// solve --maximal --dump-answer --dump-lattice` call, at library level and
/// past *forbid*'s refusal (`forbid::studying`, M1e2 S1e2.2.5), which no CLI
/// path lifts. W3's dead hole is the shape the refusal names, and its drawing
/// is what the test below holds; `ein render region` reads the directories
/// and refuses nothing.
fn studied(file: &str, cube: &Path, run: &Path) {
    let loaded = || {
        let path = repo_root().join(file);
        let text = std::fs::read_to_string(&path).expect(file);
        let mut ast = ein_ir::Ast::new();
        let mut terms = ein_core::Terms::new();
        let forms = ein_ir::parse(&mut ast, &text, path.to_str()).expect("parses");
        let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
        (kb, terms, ast, text)
    };
    ein_infer::forbid::studying(|| {
        let (mut kb, mut terms, ast, _) = loaded();
        let out = ein_infer::region::compute(
            &mut kb,
            &mut terms,
            &ast,
            &ein_infer::region::Options::default(),
            &mut |_| {},
        )
        .expect("studied, the cube is computed");
        let inputs = ein_render::dump::region::RegionInputs {
            source: file,
            limit: ein_infer::region::DEFAULT_LIMIT,
        };
        ein_render::dump::write_cube_dir(cube, &out, &terms, &inputs).expect("the cube dir");

        let (mut kb, mut terms, ast, text) = loaded();
        let visits = Arc::new(Mutex::new(Visits::default()));
        let mut events =
            ein_infer::events::Events::off().watched(Box::new(Collect(visits.clone())));
        let opts = ein_infer::explore::Options::new(ein_infer::explore::Mode {
            which: ein_infer::explore::Which::Maximal,
            ask: ein_infer::explore::Ask::Describe,
        });
        let r = ein_infer::read_out::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
            .expect("studied, the run answers");
        let candidates: Vec<String> = r.answer.candidates.iter().map(|c| c.text.clone()).collect();
        let v = visits.lock().expect("the run is done").clone();
        let lattice = region_lattice::Inputs {
            source: file,
            argv: "ein solve FILE --maximal",
            route: "2, the cube",
            candidates: &candidates,
        };
        let dir = run.join("lattice");
        std::fs::create_dir_all(&dir).expect("lattice/");
        region_lattice::write(&dir, &v, &lattice).expect("lattice/");
        let config = kb.program().config.clone().unwrap_or_default();
        let summary = ein_cli::summary::text(
            &ein_cli::summary::build_region(&ast, &mut terms, &mut kb, &r, file, &config)
                .expect("the summary"),
        );
        let inputs = ein_render::dump::region_answer::Inputs {
            source: file,
            program: &text,
            summary: &summary,
            answer_txt: "",
            forms: &[],
        };
        let dir = run.join("answer");
        std::fs::create_dir_all(&dir).expect("answer/");
        ein_render::dump::region_answer::write(&dir, &r, &mut kb, &mut terms, &ast, &inputs)
            .expect("answer/");
    });
}

/// `ein solve FILE FLAGS --dump-answer DIR/answer --dump-lattice
/// DIR/lattice`: the run's side of an overlay.
fn ran(file: &str, flags: &[&str], dir: &Path) {
    let (a, l) = (dir.join("answer"), dir.join("lattice"));
    let (a, l) = (a.to_string_lossy(), l.to_string_lossy());
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    args.extend(["--dump-answer", &a, "--dump-lattice", &l]);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{file}: {}", r.err);
}

/// The cube in `dir`, with the run whose `answer/` and `lattice/` are in
/// `run` drawn on it when one is given.
fn render_with(dir: &Path, run: Option<&Path>, view: &str) -> String {
    let d = dir.to_string_lossy().into_owned();
    let mut args: Vec<String> = vec!["render".into(), "region".into(), d];
    if let Some(run) = run {
        args.extend([
            "--visited".into(),
            run.join("lattice").to_string_lossy().into_owned(),
            "--answer".into(),
            run.join("answer").to_string_lossy().into_owned(),
        ]);
    }
    args.extend(["--view".into(), view.into()]);
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let r = ein(&argv);
    assert_eq!(r.code, 0, "render region {view}: {}", r.err);
    r.out
}

/// The DOT statement drawing node `mask`.
fn node_line(dot: &str, mask: i64) -> &str {
    let head = format!("  m{mask} [label=");
    dot.lines()
        .find(|l| l.starts_with(&head))
        .unwrap_or_else(|| panic!("node m{mask} is not drawn:\n{dot}"))
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

/// Rewrite one JSON file of a dump in place.
fn doctor(path: &Path, edit: impl FnOnce(&mut J)) {
    let mut j = json(path);
    edit(&mut j);
    std::fs::write(path, serde_json::to_string(&j).expect("json")).expect("write");
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
/// view and the n = 8 lines view — and one directory over the limit. The
/// third was `naf-nowb`'s dead hole until *forbid* refused it (M1e2
/// S1e2.2.5); the W3 test draws that one from a library-level fixture.
const DRAWN: [(&str, &[&str]); 9] = [
    ("examples/branching/15_lookahead_two_step_on.ein", &[]),
    ("examples/lattice/04_obligation_trigger_hole.ein", &[]),
    ("examples/lattice/08_absent_witness_hole.ein", &[]),
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

/// **Every view is DOT Graphviz accepts**, with no run on the cube and with
/// a bare `ein solve` on it; `table` is markdown with one section per layer
/// and an anchor per node; and `auto` is the view the stage measured: the
/// whole cube to n = 6, R and its border to 64 nodes, the lines past that.
#[test]
fn every_view_is_drawn_and_auto_picks_the_measured_one() {
    require_graphviz();
    let root = scratch("views");
    let mut autos = Vec::new();
    for (i, (file, flags)) in DRAWN.iter().enumerate() {
        let dir = root.join(format!("{i}"));
        let cube = dir.join("region");
        dumped(file, flags, &cube);
        ran(file, flags, &dir);
        for run in [None, Some(dir.as_path())] {
            for view in ["auto", "full", "border", "lines"] {
                let dot = render_with(&cube, run, view);
                if let Err(e) = graphviz_accepts(&dot) {
                    panic!(
                        "{file} --view {view} (run: {}): Graphviz refused it:\n{e}",
                        run.is_some()
                    );
                }
                if view == "auto" && run.is_none() {
                    autos.push((*file, view_of(&dot).to_string()));
                }
                assert_eq!(
                    dot.contains("no run overlaid"),
                    run.is_none(),
                    "{file} --view {view}: the title says whether a run is drawn"
                );
            }
            let table = render_with(&cube, run, "table");
            assert!(table.starts_with("# Region · "), "{file}: {table}");
            let refused = json(&cube.join("region.json"))["region"].is_null();
            if !refused {
                let n = json(&cube.join("region.json"))["n"].as_u64().unwrap();
                assert_eq!(
                    table.matches("\n## L").count() as u64,
                    n + 1,
                    "{file}: one section per layer"
                );
                assert_eq!(
                    table.matches("<a id=\"m").count() as u64,
                    1 << n,
                    "{file}: one anchor per node"
                );
                assert_eq!(
                    table.contains("| run |"),
                    run.is_some(),
                    "{file}: the run column"
                );
                if run.is_some() {
                    // A correct engine's run, against the cube that holds it.
                    for line in [
                        "- **missing** none",
                        "- **spurious** none",
                        "- **≠ the cube** none",
                    ] {
                        assert!(table.contains(line), "{file}: no `{line}`:\n{table}");
                    }
                }
            } else if run.is_some() {
                assert!(
                    table.contains("with no cube to hold it to"),
                    "{file}: {table}"
                );
            }
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
}

/// **W1, drawn.** On `branching/15` `--maximal` reports both nodes of the
/// maximal line, and nothing is missing. The walk reported only `{(p B),
/// (p C)}` and called it exhausted (the register's `wrong`): written into the
/// run's answer, `{(p A)}` is MISSING, red, in the drawing and the table.
/// Bare `ein solve` reports all of R — five states, five octagons.
#[test]
fn w1_a_node_of_the_part_asked_that_no_state_holds_is_missing() {
    let root = scratch("w1");
    let file = "examples/branching/15_lookahead_two_step_on.ein";
    let cube = root.join("region");
    dumped(file, &[], &cube);
    let a = mask_of(&cube, &["(p A)"]);
    let max = root.join("max");
    ran(file, &["--maximal"], &max);
    let dot = render_with(&cube, Some(&max), "full");
    let pa = node_line(&dot, a);
    assert!(
        pa.contains("shape=octagon") && pa.contains("state 1"),
        "{pa}"
    );
    assert!(!dot.contains("\\nMISSING"), "{dot}");
    let table = render_with(&cube, Some(&max), "table");
    assert!(
        table.contains("**Compared with the maximal line**"),
        "{table}"
    );
    assert!(table.contains("- **missing** none"), "{table}");

    // The walk's answer, as the register records it.
    doctor(&max.join("answer/index.json"), |j| {
        let states = j["region"]["states"].as_array_mut().expect("states");
        states.retain(|s| s["facts"] != serde_json::json!(["(p A)"]));
        assert_eq!(states.len(), 1);
    });
    let dot = render_with(&cube, Some(&max), "full");
    let pa = node_line(&dot, a);
    assert!(pa.contains("MISSING") && pa.contains("#e31a1c"), "{pa}");
    assert!(
        pa.contains("shape=box") && pa.contains("\\nvisited"),
        "visited, and reported by nothing: {pa}"
    );
    let table = render_with(&cube, Some(&max), "table");
    assert!(
        table.contains(&format!("- **missing** [`{{(p A)}}`](#m{a})")),
        "{table}"
    );
    assert!(
        table.contains(&format!(
            "<a id=\"m{a}\"></a>`{{(p A)}}` | R · max · **missing**"
        )),
        "{table}"
    );

    let all = root.join("all");
    ran(file, &[], &all);
    let dot = render_with(&cube, Some(&all), "full");
    assert_eq!(dot.matches("shape=octagon").count(), 5, "{dot}");
    assert!(
        !dot.contains("\\nMISSING") && !dot.contains("\\nSPURIOUS"),
        "{dot}"
    );
}

/// **W3, drawn.** `naf-nowb` is not monotone: `{(p A)}` is a hole, dead
/// under its `absent` guard, and `{(p A), (q A)}` above it is alive. *Forbid*
/// refuses the program since M1e2 S1e2.2.5, so its directories are written by
/// [`studied`], a library-level fixture, and the drawing is held as it was. The
/// region run takes the cube (route 2), visits every node and reports the
/// alive superset — the hole carries its repair edge, and nothing is missing
/// or read otherwise than the cube. A run that pruned above the dead `{(p
/// A)}`, as licence L1 would on a monotone program and as the walk's lookahead
/// did here, is drawn with the exclusion named unsound and the alive node it
/// left unvisited.
#[test]
fn w3_the_hole_is_drawn_and_an_exclusion_the_cube_refutes_is_named() {
    let root = scratch("w3");
    let file = "examples/ein-bugs/wrong-answers/naf-nowb.ein";
    let cube = root.join("region");
    let run = root.join("run");
    studied(file, &cube, &run);
    let pa = mask_of(&cube, &["(p A)"]);
    let both = mask_of(&cube, &["(p A)", "(q A)"]);
    let dot = render_with(&cube, Some(&run), "full");
    assert!(dot.contains("label=\"+(q A)\""), "the repair edge:\n{dot}");
    assert!(node_line(&dot, pa).contains("hole"), "{dot}");
    assert!(node_line(&dot, both).contains("state 1"), "{dot}");
    assert!(
        !dot.contains("\\nMISSING") && !dot.contains("\\n≠ the cube"),
        "{dot}"
    );
    let table = render_with(&cube, Some(&run), "table");
    assert!(table.contains("- **visited** 4 of 4 nodes"), "{table}");

    // The run had pruned above the dead singleton.
    doctor(&run.join("lattice/nodes.json"), |j| {
        let nodes = j["nodes"].as_array_mut().expect("nodes");
        nodes.retain(|n| n["commitment"] != serde_json::json!(["(p A)", "(q A)"]));
    });
    let dot = render_with(&cube, Some(&run), "full");
    assert!(node_line(&dot, pa).contains("✗ unsound exclusion"), "{dot}");
    assert!(
        dot.contains(&format!(
            "  m{pa} -> m{both} [color=\"#ae017e\", style=dashed"
        )),
        "the exclusion's edge to the alive node it left:\n{dot}"
    );
    let table = render_with(&cube, Some(&run), "table");
    assert!(
        table.contains(&format!(
            "1 of them alive in the cube: **unsound** [`{{(p A)}}`](#m{pa}) visited dead, \
             [`{{(p A), (q A)}}`](#m{both}) above it alive"
        )),
        "{table}"
    );
}

/// **W2, drawn.** The walk's forced-positive cascade promoted `(q one)` into
/// root before any entering, moved the search's root onto the dead `{(q
/// one)}`, and left `∅` — all of R — missing. A region run never writes root:
/// it visits `∅` and `{(q one)}`, reads them as the cube does, and reports
/// `∅`.
#[test]
fn w2_a_region_run_never_moves_root() {
    let root = scratch("w2");
    let file = "examples/ein-bugs/wrong-answers/forced-2.ein";
    let cube = root.join("region");
    dumped(file, &[], &cube);
    let run = root.join("run");
    ran(file, &["--maximal"], &run);
    let dot = render_with(&cube, Some(&run), "full");
    let empty = node_line(&dot, 0);
    assert!(
        empty.contains("state 1") && empty.contains("shape=octagon"),
        "{empty}"
    );
    let q = node_line(&dot, mask_of(&cube, &["(q one)"]));
    assert!(
        q.contains("dead") && q.contains("visited") && q.contains("shape=box"),
        "{q}"
    );
    for word in ["\\nMISSING", "root after the cascade", "\\n≠ the cube"] {
        assert!(!dot.contains(word), "{word}:\n{dot}");
    }
}

/// **Saturation is a move inside the cube.** On `05_mini_zebra` the describing
/// run reads its maximal state from `{(co-located Bob Coffee), (co-located Bob
/// Dog)}`, whose saturation derives `(co-located Coffee Dog)`: the commitment
/// is transient with a `sat` arrow to its closure, the closure is the state,
/// on the maximal line, and — compared by state — nothing is missing.
#[test]
fn a_reported_state_counts_as_the_node_its_saturation_lands_on() {
    let root = scratch("moves");
    let file = "examples/branching/05_mini_zebra.ein";
    let cube = root.join("region");
    dumped(file, &[], &cube);
    let run = root.join("run");
    ran(file, &[], &run);
    let index = json(&run.join("answer/index.json"));
    let state = index["region"]["states"]
        .as_array()
        .expect("states")
        .iter()
        .position(|s| s["from"] != s["closure"])
        .expect("a state read from a commitment its saturation moves");
    let s = &index["region"]["states"][state];
    let strs = |v: &J| -> Vec<String> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect()
    };
    let (from, closure) = (strs(&s["from"]), strs(&s["closure"]));
    let from = mask_of(&cube, &from.iter().map(String::as_str).collect::<Vec<_>>());
    let closure = mask_of(
        &cube,
        &closure.iter().map(String::as_str).collect::<Vec<_>>(),
    );
    let region = json(&cube.join("region.json"));
    assert_eq!(region["region"]["nodes"][closure as usize]["maximal"], true);
    let dot = render_with(&cube, Some(&run), "lines");
    assert!(
        dot.contains(&format!("  m{from} -> m{closure} [color=\"#08519c\"")),
        "the sat arrow:\n{dot}"
    );
    let tail = node_line(&dot, from);
    assert!(
        tail.contains("style=\"filled,rounded,dashed\"")
            || tail.contains("style=\"filled,dashed\""),
        "{tail}"
    );
    assert!(
        tail.contains(&format!("from: state {}", state + 1)),
        "{tail}"
    );
    let landed = node_line(&dot, closure);
    assert!(landed.contains(&format!("state {}", state + 1)), "{landed}");
    assert!(
        landed.contains("shape=octagon") && !landed.contains("MISSING"),
        "{landed}"
    );
}

/// **The table's head names at most fifty nodes a list**, and how many
/// there are. `syntax/equality.ein`'s R is its whole 2¹² cube, so `-n 1`
/// leaves 4 095 nodes *not reached*; each row's status says so, and the head
/// links the first fifty. It was 2⁶ and 63 until M1e2 S1e2.2.14 made
/// `House-1`, written only inside `(color House-1)`, a candidate object. The gallery's `houses` (2²⁰, all R, stopped by the
/// budget) listed a million before the cap.
#[test]
fn the_head_names_at_most_fifty_nodes_a_list() {
    let root = scratch("cap");
    let file = "examples/syntax/equality.ein";
    let cube = root.join("region");
    dumped(file, &[], &cube);
    let run = root.join("run");
    ran(file, &["-n", "1"], &run);
    let table = render_with(&cube, Some(&run), "table");
    let line = table
        .lines()
        .find(|l| l.starts_with("- **not reached**"))
        .unwrap_or_else(|| panic!("no not-reached line:\n{table}"));
    assert_eq!(line.matches("](#m").count(), 50, "{line}");
    assert!(
        line.contains(", … 4045 more: 4095 in all, and each row's status says *not reached*"),
        "{line}"
    );
    assert_eq!(table.matches(" · not reached |").count(), 4095);
}

/// **A run of another program** is refused, exit 1: its commitments are not
/// the cube's nodes. So is an answer directory with no `index.json`.
#[test]
fn a_run_over_another_universe_is_an_error() {
    let root = scratch("other");
    let cube = root.join("region");
    dumped(
        "examples/branching/15_lookahead_two_step_on.ein",
        &[],
        &cube,
    );
    let run = root.join("run");
    ran("examples/ein-bugs/wrong-answers/forced-2.ein", &[], &run);
    let (c, l, a) = (
        cube.to_string_lossy().into_owned(),
        run.join("lattice").to_string_lossy().into_owned(),
        run.join("answer").to_string_lossy().into_owned(),
    );
    for args in [
        vec!["render", "region", &c, "--visited", &l],
        vec!["render", "region", &c, "--answer", &a],
    ] {
        let r = ein(&args);
        assert_eq!(r.code, 1, "{args:?}: {}", r.out);
        assert!(
            r.err.contains("not a run of the program the cube is of"),
            "{}",
            r.err
        );
        assert!(r.out.is_empty());
    }
    let r = ein(&["render", "region", &c, "--answer", &l]);
    assert_eq!(r.code, 1);
    assert!(r.err.contains("index.json"), "{}", r.err);
}

/// **A directory that is not a dump** is an error naming the file, exit 1.
#[test]
fn a_directory_without_the_files_is_an_error() {
    let root = scratch("empty");
    let r = ein(&["render", "region", &root.to_string_lossy()]);
    assert_eq!(r.code, 1);
    assert!(r.err.contains("region.json"), "{}", r.err);
    assert!(r.out.is_empty());
}
