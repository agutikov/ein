//! `ein solve`'s region modes and `ein oracle`, through the binary — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4.
//!
//! What the modes *answer* is `ein-infer/tests/explore.rs`'s, held to the
//! oracle there. What is held here is the surface:
//! - the exit codes, with `--single` exiting the way a false `:expect` does;
//! - the refusals;
//! - `-j`;
//! - the oracle's directory, which `ein render region` draws with no run
//!   overlaid;
//! - and the walk, still the default until S1e2.1a.5.

use std::path::PathBuf;
use std::process::Command;

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
    let dir = std::env::temp_dir().join(format!("ein-region-modes-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

const THREE: &str = "examples/lattice/02_genuine_3set_death.ein";
const SQUARE: &str = "examples/saturation/square-bwd/floors.ein";

fn line<'a>(out: &'a str, key: &str) -> &'a str {
    out.lines()
        .find(|l| l.starts_with(key))
        .unwrap_or_else(|| panic!("no `{key}` line in:\n{out}"))
}

#[test]
fn describe_is_the_oracles_description_and_exits_0() {
    let r = ein(&["solve", THREE, "--region-budget", "524288"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(line(&r.out, "k_s").ends_with(" 7"), "{}", r.out);
    assert!(line(&r.out, "k_c").ends_with(" 7"), "{}", r.out);
    assert!(line(&r.out, "exhausted").ends_with("true"));
    let o = ein(&["oracle", THREE]);
    assert_eq!(o.code, 0, "{}", o.err);
    for key in [
        "k_s",
        "k_c",
        "∅ in R",
        "minimal ",
        "maximal ",
        "components",
        "N ",
    ] {
        assert_eq!(
            line(&r.out, key).split_whitespace().collect::<Vec<_>>(),
            line(&o.out, key).split_whitespace().collect::<Vec<_>>(),
            "{key}"
        );
    }
    assert!(line(&o.out, "the cube").contains("the static verdict holds"));
    assert!(line(&o.out, "route").contains("the cube"));
}

#[test]
fn single_exits_like_a_false_expectation() {
    // Three maximal states: Ambiguity, exit 1, the `:expect` line on stderr.
    let r = ein(&["solve", THREE, "--maximal", "--single"]);
    assert_eq!(r.code, 1);
    assert_eq!(
        line(&r.out, "verdict").split_whitespace().last(),
        Some("Ambiguity")
    );
    assert!(r.err.contains("--single FAILED"), "{}", r.err);
    // ⊤ is the square cube's one maximal state: Solution, exit 0, cheaply.
    let r = ein(&["solve", SQUARE, "--maximal", "--single", "-s"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert_eq!(
        line(&r.out, "verdict").split_whitespace().last(),
        Some("Solution")
    );
    assert!(r.err.is_empty(), "{}", r.err);
    // A budget that stops it first: NOT CHECKED, exit 1.
    let r = ein(&[
        "solve",
        "examples/lattice/06_friends_tables_everyone_sits.ein",
        "--single",
        "--region-budget",
        "1",
    ]);
    assert_eq!(r.code, 1);
    assert!(r.err.contains("--single NOT CHECKED"), "{}", r.err);
    assert!(line(&r.out, "stopped").contains("at the budget"));
}

#[test]
fn n_inside_a_mode_is_n_states_and_outside_it_is_still_the_walk() {
    let r = ein(&["solve", THREE, "--maximal", "-n", "2"]);
    assert_eq!(r.code, 0);
    assert_eq!(r.out.matches("\nstate ").count(), 2, "{}", r.out);
    assert!(line(&r.out, "answer").contains("more may exist"));
    // Until S1e2.1a.5 a run with no region flag is today's walk.
    let w = ein(&["solve", THREE, "-n", "2"]);
    assert!(!w.out.contains("region "), "{}", w.out);
    assert!(w.out.contains("verdict"), "{}", w.out);
}

#[test]
fn the_walks_options_are_refused_beside_a_mode() {
    for (args, name) in [
        (vec!["--maximal", "-e"], "--exhaustive"),
        (vec!["--single", "-m", "3"], "--max-set-size"),
        (vec!["--minimal", "-L"], "--no-lookahead"),
        (vec!["--region-budget", "10", "-p"], "--print-final-state"),
        (
            vec!["--maximal", "--json-summary", "/tmp/x.json"],
            "--json-summary",
        ),
    ] {
        let mut a = vec!["solve", THREE];
        a.extend(args.iter());
        let r = ein(&a);
        assert_eq!(r.code, 2, "{args:?}");
        assert!(
            r.err.contains(&format!("{name} is the walk's")),
            "{args:?}: {}",
            r.err
        );
    }
    // clap's own: the two lines exclude each other, and so do -n and --single.
    for args in [
        vec!["--minimal", "--maximal"],
        vec!["--single", "-n", "2"],
        vec!["--region-budget", "0"],
    ] {
        let mut a = vec!["solve", THREE];
        a.extend(args.iter());
        assert_eq!(ein(&a).code, 2, "{args:?}");
    }
}

#[test]
fn jobs_change_no_line_of_a_mode() {
    for mode in [
        &["--maximal"][..],
        &["--minimal"],
        &["--single"],
        &["-n", "3", "--region-budget", "524288"],
    ] {
        let run = |j: &str| {
            let mut a = vec![
                "solve",
                "examples/lattice/07_two_pairs_two_tables.ein",
                "-j",
                j,
            ];
            a.extend(mode.iter());
            ein(&a)
        };
        let (a, b) = (run("1"), run("4"));
        assert_eq!((a.code, &a.out), (b.code, &b.out), "{mode:?}");
    }
}

#[test]
fn the_oracles_directory_has_no_walk_and_draws_with_none() {
    let dir = scratch("oracle");
    let d = dir.to_string_lossy().to_string();
    let r = ein(&["oracle", THREE, "--dump", &d]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(dir.join("region.json").exists() && dir.join("region.md").exists());
    assert!(!dir.join("walk.json").exists());
    let md = std::fs::read_to_string(dir.join("region.md")).unwrap();
    assert!(!md.contains("This run's walk"), "{md}");
    let draw = ein(&["render", "region", &d]);
    assert_eq!(draw.code, 0, "{}", draw.err);
    assert!(draw.out.contains("no run overlaid"), "{}", draw.out);
    // A `--dump-region` directory dumped over it keeps its walk; the oracle
    // dumped over that one removes it.
    assert_eq!(ein(&["solve", THREE, "--dump-region", &d]).code, 0);
    assert!(dir.join("walk.json").exists());
    assert_eq!(ein(&["oracle", THREE, "--dump", &d]).code, 0);
    assert!(!dir.join("walk.json").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_oracle_refuses_past_its_limit() {
    let r = ein(&["oracle", THREE, "--region-limit", "2"]);
    assert_eq!(r.code, 2);
    assert!(r.err.contains("over --region-limit 2"), "{}", r.err);
}
