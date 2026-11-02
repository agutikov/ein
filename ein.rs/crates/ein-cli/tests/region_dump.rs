//! `ein oracle FILE --dump DIR` — M1e2
//! [S1e2.1a.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md)'s
//! door, which was `ein solve --dump-region` until S1e2.1a.5's switch moved it
//! to the oracle's own command.
//!
//! Held from the outside: the dump moves nothing the oracle prints; it agrees
//! node for node with the second route, `utils/region.py`, which classifies
//! each subset with one `ein saturate` process; its limit refuses, exit 2,
//! and warns when raised; and the `non_monotone` stub's line and block appear
//! exactly where the static check finds a site. The walk's half of the
//! directory, `walk.json`, went with the walk; drawing a region run on the
//! cube is S1e2.1a.7's.

use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use ein_corpus::repo_root;
use serde_json::Value as J;

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str], env: &[String]) -> Run {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(args).current_dir(repo_root());
    for kv in env {
        let (k, v) = kv.split_once('=').expect("NAME=value");
        cmd.env(k, v);
    }
    let out = cmd.output().expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-region-dump-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch");
    dir
}

fn json(path: &Path) -> J {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn dump(file: &str, flags: &[&str], dir: &Path) -> Run {
    let d = dir.to_string_lossy();
    let mut args = vec!["oracle", file];
    args.extend_from_slice(flags);
    args.extend(["--dump", &d]);
    ein(&args, &[])
}

/// The seven programs of Q-M1e2.7's tables.
const SEVEN: [&str; 7] = [
    "examples/branching/15_lookahead_two_step_on.ein",
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/ein-bugs/wrong-answers/forced-2-two.ein",
    "examples/ein-bugs/wrong-answers/owed-two-step.ein",
    "examples/ein-bugs/wrong-answers/owed-seats.ein",
    "examples/lattice/05_owed_seats_bounded.ein",
    "examples/ein-bugs/naf-upward-closure.ein",
];

/// **The dump moves nothing the oracle prints**: stdout, stderr, the exit
/// code and its summary, with `--dump` and without.
#[test]
fn the_flag_moves_no_answer() {
    let root = scratch("additive");
    for (i, file) in [
        "examples/branching/15_lookahead_two_step_on.ein",
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        "examples/ein-bugs/naf-upward-closure.ein",
    ]
    .iter()
    .enumerate()
    {
        let run = |with: bool| {
            let tag = if with { "with" } else { "without" };
            let s = root.join(format!("{i}-{tag}.json"));
            let d = root.join(format!("{i}-dir"));
            let (s, d) = (s.to_string_lossy(), d.to_string_lossy());
            let mut args = vec!["oracle", file, "--json-summary", &s];
            if with {
                args.extend(["--dump", &d]);
            }
            let r = ein(&args, &[]);
            let summary = std::fs::read_to_string(s.as_ref()).expect("summary");
            (r, summary)
        };
        let (a, sa) = run(false);
        let (b, sb) = run(true);
        assert_eq!(a.code, b.code, "{file}: exit code moved");
        let body = |s: &str| s.lines().skip(1).map(str::to_string).collect::<Vec<_>>();
        assert_eq!(body(&a.out), body(&b.out), "{file}: stdout moved");
        assert_eq!(a.err, b.err, "{file}: stderr moved");
        assert_eq!(sa, sb, "{file}: the summary moved");
    }
    let _ = std::fs::remove_dir_all(&root);
}

fn sets(v: &J) -> BTreeSet<Vec<String>> {
    v.as_array()
        .expect("a list of sets")
        .iter()
        .map(|s| {
            let mut f: Vec<String> = s
                .as_array()
                .expect("a set")
                .iter()
                .map(|x| x.as_str().expect("a fact").to_string())
                .collect();
            f.sort();
            f
        })
        .collect()
}

fn require_python3() {
    let ok = Command::new("python3")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success());
    assert!(
        ok,
        "python3 is not on PATH, and the second route needs it — this test fails \
         rather than skips, as cli_semantics' variant check does"
    );
}

/// A program where saturation **moves** a node inside the cube — a
/// commitment derives candidates it did not commit to — so the closure check
/// below is not vacuous: none of the seven does (S1e2.1a.2).
const MOVES: &str = "examples/saturation/transitive/taxonomy.ein";

/// A program whose dead nodes die by a **pair**, `X` beside `(not X)`, which
/// derives no `(false)` fact: 12 of its 32 nodes. `region.py` read only
/// `false` until M1e2 S1e2.1b.2, and called those 12 alive.
const PAIR: &str = "corpus/fuzz_findings/d3-unsat-core.ein";

/// **Two routes agree.** The library oracle and `utils/region.py` on the
/// seven programs, [`MOVES`] and [`PAIR`], node for node: the status of every subset,
/// every alive node's closure (S1e2.1a.2), the two lines, the holes, the
/// components' sizes, the maximal sub-cubes, `N`, and the N side of the
/// duality. One of the seven has a hole.
#[test]
fn the_oracle_and_the_per_subset_route_agree_node_for_node() {
    require_python3();
    let root = scratch("routes");
    let mut moved = 0;
    for (i, file) in SEVEN.iter().chain([&MOVES, &PAIR]).enumerate() {
        let dir = root.join(format!("{i}"));
        let r = dump(file, &[], &dir);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        let region = json(&dir.join("region.json"));
        let cands: Vec<String> = region["candidates"]
            .as_array()
            .expect("candidates")
            .iter()
            .map(|c| c.as_str().expect("text").to_string())
            .collect();
        let mut cmd = Command::new("python3");
        cmd.arg("-I")
            .arg(repo_root().join("utils/region.py"))
            .arg("--json")
            .arg(repo_root().join(file))
            .args(&cands)
            .env("EIN_BIN", env!("CARGO_BIN_EXE_ein"));
        let out = cmd.output().expect("region.py runs");
        assert!(
            out.status.success(),
            "{file}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let py: J = serde_json::from_slice(&out.stdout).expect("region.py --json");

        let rg = &region["region"];
        // Every node: region.py says dead / owes n / ok, and reads no debt on
        // a dead node, so dead+owing is its dead.
        let ours: BTreeSet<(Vec<String>, String)> = rg["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .map(|n| {
                let mut f: Vec<String> = n["facts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                f.sort();
                let st = match n["status"].as_str().unwrap() {
                    "R" => "ok".to_string(),
                    "owing" => format!("owes {}", n["owes"]),
                    _ => "dead".to_string(),
                };
                (f, st)
            })
            .collect();
        let theirs: BTreeSet<(Vec<String>, String)> = py["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| {
                let f: Vec<String> = n["facts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                (f, n["status"].as_str().unwrap().to_string())
            })
            .collect();
        assert_eq!(ours, theirs, "{file}: the nodes differ");
        // Every alive node's closure: the candidates its fixpoint holds.
        let named = |mask: i64| -> Vec<String> {
            let mut f: Vec<String> = (0..cands.len())
                .filter(|b| mask >> b & 1 == 1)
                .map(|b| cands[b].clone())
                .collect();
            f.sort();
            f
        };
        let ours: BTreeSet<(Vec<String>, Vec<String>)> = rg["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|n| !n["status"].as_str().unwrap().starts_with("dead"))
            .map(|n| {
                if n["closure"] != n["mask"] {
                    moved += 1;
                }
                (
                    named(n["mask"].as_i64().unwrap()),
                    named(n["closure"].as_i64().unwrap()),
                )
            })
            .collect();
        let theirs: BTreeSet<(Vec<String>, Vec<String>)> = py["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|n| !n["closure"].is_null())
            .map(|n| {
                let f = |x: &J| -> Vec<String> {
                    x.as_array()
                        .unwrap()
                        .iter()
                        .map(|y| y.as_str().unwrap().to_string())
                        .collect()
                };
                (f(&n["facts"]), f(&n["closure"]))
            })
            .collect();
        assert_eq!(ours, theirs, "{file}: the closures differ");
        for key in ["minimal", "maximal", "holes", "nogoods"] {
            assert_eq!(sets(&rg[key]), sets(&py[key]), "{file}: {key}");
        }
        let mut sizes: Vec<i64> = rg["components"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["size"].as_i64().unwrap())
            .collect();
        sizes.sort();
        let theirs: Vec<i64> = py["components"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_i64().unwrap())
            .collect();
        assert_eq!(sizes, theirs, "{file}: components");
        let cubes = |v: &J, a: &str, b: &str| -> BTreeSet<(Vec<String>, Vec<String>)> {
            v.as_array()
                .unwrap()
                .iter()
                .map(|c| {
                    let side = |x: &J| {
                        let mut f: Vec<String> = x
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|y| y.as_str().unwrap().to_string())
                            .collect();
                        f.sort();
                        f
                    };
                    if a.is_empty() {
                        (side(&c[0]), side(&c[1]))
                    } else {
                        (side(&c[a]), side(&c[b]))
                    }
                })
                .collect()
        };
        assert_eq!(
            cubes(&rg["subcubes"], "from", "to"),
            cubes(&py["subcubes"], "", ""),
            "{file}: sub-cubes"
        );
        assert_eq!(
            rg["duality"]["maximal_holds"].as_bool(),
            py["duality_n"].as_bool(),
            "{file}: the N side of the duality"
        );
    }
    assert!(
        moved > 0,
        "no node moved: the closure check compared identities"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **Any region can be written as a program** (S1e2.1a.8): `utils/region_synth.py
/// --random` draws 300 targets over at most six candidates — half of them with
/// implications, so saturation moves nodes — writes each as a program, and
/// holds this binary's oracle to it node for node, with the static check
/// calling it monotone exactly when the target is. The one target it does not
/// draw, a dead root under a non-empty R, it refuses by name: every node is
/// built from root's fixpoint; what a node should be is F21's D2, open.
#[test]
fn a_random_region_written_as_a_program_comes_back() {
    require_python3();
    let out = Command::new("python3")
        .arg("-I")
        .arg(repo_root().join("utils/region_synth.py"))
        .args(["--random", "300", "--seed", "1", "--max-n", "6"])
        .env("EIN_BIN", env!("CARGO_BIN_EXE_ein"))
        .output()
        .expect("region_synth.py runs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{stdout}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(stdout.contains("300 targets, seed 1, 0 failed"), "{stdout}");
}

/// **Over the limit is a refusal**: exit 2, one stderr line naming the
/// option, and a directory whose `region.json` says why there is no region.
/// Under `ein solve`, where the cube was an artefact beside the walk, it was a
/// note; the oracle's whole answer is the cube.
#[test]
fn over_the_limit_the_oracle_refuses_and_says_why() {
    let root = scratch("limit");
    let file = "examples/lattice/02_genuine_3set_death.ein";
    let dir = root.join("d");
    let r = dump(file, &["--region-limit", "2"], &dir);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(r.out.is_empty(), "{}", r.out);
    let refusal: Vec<&str> = r.err.lines().filter(|l| l.starts_with("error:")).collect();
    assert_eq!(refusal.len(), 1, "{}", r.err);
    assert!(
        refusal[0].contains("3 root candidates") && refusal[0].contains("--region-limit 3"),
        "{}",
        r.err
    );
    let region = json(&dir.join("region.json"));
    assert!(region["region"].is_null());
    assert_eq!(region["refused"]["n"], 3);
    assert_eq!(region["refused"]["limit"], 2);
    assert!(!dir.join("walk.json").exists());
    let md = std::fs::read_to_string(dir.join("region.md")).expect("region.md");
    assert!(md.contains("## No region"), "{md}");
    let _ = std::fs::remove_dir_all(&root);
}

/// `n` candidates and no rule: every subset is in R.
fn wide_program(dir: &Path, n: usize) -> PathBuf {
    let mut s = String::from("(relation is-a T T)\n(relation cand T)\n(relation p T)\n");
    for i in 0..n {
        s.push_str(&format!("(is-a x{i} T) (cand x{i})\n"));
    }
    s.push_str("(hrule guess () :match (cand ?x) :assert (p ?x))\n(query :goal (cand ?x))\n");
    let path = dir.join(format!("wide-{n}.ein"));
    std::fs::write(&path, s).expect("write");
    path
}

/// The first `warning:` lines a raised limit prints, after which the run is
/// stopped — `2ⁿ` saturations are not this test's subject.
fn first_warning(file: &Path, n: usize, dir: &Path) -> Vec<String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("oracle")
        .arg(file)
        .args(["--region-limit", &n.to_string(), "--dump"])
        .arg(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("ein runs");
    let err = child.stderr.take().expect("stderr");
    let mut lines = Vec::new();
    for line in BufReader::new(err).lines() {
        let line = line.expect("utf-8");
        if line.starts_with("warning:") {
            lines.push(line);
            if n < 20 || lines.len() == 3 {
                break;
            }
        }
    }
    let _ = child.kill();
    let _ = child.wait();
    lines
}

/// **Raised, 17–19 warns on one line and 20 and more warns loudly**, each
/// with the cost measured on nodes spread over the cube.
#[test]
fn a_raised_limit_warns_with_the_projected_cost() {
    let root = scratch("warn");
    let w17 = first_warning(&wide_program(&root, 17), 17, &root.join("d17"));
    assert_eq!(w17.len(), 1, "{w17:?}");
    assert!(w17[0].contains("17 root candidates") && w17[0].contains("projected"));
    let w20 = first_warning(&wide_program(&root, 20), 20, &root.join("d20"));
    assert_eq!(w20.len(), 3, "{w20:?}");
    assert!(w20[0].contains("2^20 = 1048576 saturations"), "{w20:?}");
    assert!(
        w20[1].contains("µs per node") && w20[1].contains("projected"),
        "{w20:?}"
    );
    assert!(w20[2].contains("--region-limit 20"), "{w20:?}");
    // Not raised, 17 is refused.
    let r = ein(
        &[
            "oracle",
            &wide_program(&root, 17).to_string_lossy(),
            "--dump",
            &root.join("d17b").to_string_lossy(),
        ],
        &[],
    );
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(r.err.contains("over --region-limit 16"), "{}", r.err);
    assert!(!r.err.contains("warning:"), "{}", r.err);
    let _ = std::fs::remove_dir_all(&root);
}

/// `--states` keeps each node's state beyond root's.
#[test]
fn region_states_keeps_every_nodes_state() {
    let root = scratch("states");
    let dir = root.join("d");
    let file = "examples/lattice/04_obligation_trigger_hole.ein";
    let r = dump(file, &["--states"], &dir);
    assert_eq!(r.code, 0, "{}", r.err);
    let region = json(&dir.join("region.json"));
    let seated = region["region"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["facts"] == serde_json::json!(["(seats Ann C1)"]))
        .expect("the hole");
    let state: Vec<&str> = seated["state"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect();
    assert!(state.contains(&"(is-a Ann Diner)"), "{state:?}");
    assert_eq!(seated["status"], "owing");
    assert_eq!(seated["hole"], true);
    let without = root.join("e");
    dump(file, &[], &without);
    let region = json(&without.join("region.json"));
    assert!(region["region"]["nodes"][0].get("state").is_none());
    let _ = std::fs::remove_dir_all(&root);
}

/// **The warning** (M1e2 S1e2.1b.5, the wording the user approved on
/// 2026-11-03), which replaced S1e2.1a.1's stub: on a program with a site the
/// read-out's `lattice` block says what is assumed, each site and what it
/// puts at stake, whether it bit — the status and the state apart — and the
/// parent paths checked, and the summary's block carries the same; on a
/// monotone program the row is the one it was, and the block's detector keys
/// are null.
#[test]
fn the_warning_says_what_is_assumed_where_there_is_a_site() {
    let root = scratch("warning");
    let cases = [
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            false,
            "exercised",
        ),
        (
            "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
            false,
            "conservative",
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            false,
            "exercised",
        ),
        ("examples/zebra2.ein", true, ""),
        ("examples/branching/15_lookahead_two_step_on.ein", true, ""),
    ];
    for (i, (file, monotone, bit)) in cases.iter().enumerate() {
        let s = root.join(format!("{i}.json"));
        let r = ein(
            &[
                "solve",
                file,
                "--region-budget",
                "4096",
                "--json-summary",
                &s.to_string_lossy(),
            ],
            &[],
        );
        let lines: Vec<&str> = r.out.lines().collect();
        let at = lines
            .iter()
            .position(|l| l.starts_with("  lattice "))
            .unwrap_or_else(|| panic!("{file}: no lattice row:\n{}", r.out));
        let block: Vec<&str> = lines[at + 1..]
            .iter()
            .take_while(|l| l.starts_with("    "))
            .copied()
            .collect();
        let summary = json(&s);
        let m = &summary["monotonicity"];
        assert_eq!(m["monotone"], *monotone, "{file}");
        if *monotone {
            assert_eq!(
                lines[at], "  lattice           monotone (the static check) — R is convex: no hole",
                "{file}"
            );
            assert!(block.is_empty(), "{file}: {block:?}");
            for key in ["assumes", "status", "state", "paths"] {
                assert!(m[key].is_null(), "{file}: {key} is {}", m[key]);
            }
            continue;
        }
        assert!(
            lines[at].starts_with("  lattice           not monotone"),
            "{file}"
        );
        assert!(!lines[at].contains("assumes"), "{file}: {}", lines[at]);
        let label = |l: &&str| l.split_whitespace().next().unwrap_or("").to_string();
        let labels: Vec<String> = block.iter().map(label).collect();
        let sites = m["sites"].as_array().unwrap().len();
        assert_eq!(labels[0], "assumes", "{file}");
        assert_eq!(
            labels.iter().filter(|l| *l == "site").count(),
            sites,
            "{file}"
        );
        for row in ["status", "state", "paths"] {
            assert!(
                labels.iter().any(|l| l == row),
                "{file}: no {row} row in {block:?}"
            );
        }
        assert!(
            block.iter().any(|l| l.contains(&format!("({bit}"))),
            "{file}: the status is not {bit}: {block:?}"
        );
        assert_eq!(m["status"]["bit"], *bit, "{file}");
        assert!(m["state"]["stale_nodes"].is_u64(), "{file}");
        assert_eq!(m["assumes"]["node"], "fork from root", "{file}");
        assert!(!r.out.contains("monotone lattice"), "{file}");
    }
    let _ = std::fs::remove_dir_all(&root);
}
