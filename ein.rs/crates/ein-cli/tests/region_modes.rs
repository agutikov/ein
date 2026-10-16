//! `ein solve`'s region modes and `ein oracle`, through the binary — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4.
//!
//! What the modes *answer* is `ein-infer/tests/explore.rs`'s, held to the
//! oracle there. What is held here is the surface:
//! - the exit codes: every answer exits 0, `--single`'s four words included
//!   (the user's ruling of 2026-10-16), and a stopped run says so on stderr;
//! - the refusals;
//! - `-j`;
//! - the oracle's directory, which `ein render region` draws with no run
//!   overlaid;
//! - and the walk, still the default until S1e2.1a.5.
//!
//! Since T1e2.1a.5.2 the modes print the read-out of `read_out.md`, whose
//! golden is `read_out_examples.md` (`read_out_examples.rs`); the rows here
//! are read by label.

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

/// The row labelled `key`, its value only.
fn line<'a>(out: &'a str, key: &str) -> &'a str {
    out.lines()
        .find(|l| l.trim_start().starts_with(key))
        .map(|l| l.trim_start()[key.len()..].trim_start())
        .unwrap_or_else(|| panic!("no `{key}` row in:\n{out}"))
}

/// The `(region …)` block: from its first line to the first blank one.
fn region_block(out: &str) -> Vec<&str> {
    out.lines()
        .skip_while(|l| l.trim_start() != "(region")
        .take_while(|l| !l.is_empty())
        .collect()
}

#[test]
fn describe_is_the_oracles_description_and_exits_0() {
    let r = ein(&["solve", THREE, "--region-budget", "524288"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(line(&r.out, "states (k_s)"), "7", "{}", r.out);
    assert_eq!(line(&r.out, "commitments (k_c)"), "7", "{}", r.out);
    assert!(line(&r.out, "exhausted").starts_with("yes"));
    let o = ein(&["oracle", THREE]);
    assert_eq!(o.code, 0, "{}", o.err);
    // Route 1 and the cube describe one R in one read-out: every row but the
    // route and the work, and the region itself, line for line.
    for key in [
        "states (k_s)",
        "commitments (k_c)",
        "exhausted",
        "∅",
        "minimal line",
        "maximal line",
        "components",
        "sub-cubes",
        "merged",
        "N",
        "universe",
    ] {
        assert_eq!(line(&r.out, key), line(&o.out, key), "{key}");
    }
    assert_eq!(region_block(&r.out), region_block(&o.out));
    assert!(!region_block(&r.out).is_empty(), "{}", r.out);
    assert!(line(&o.out, "the cube").contains("the static verdict holds"));
    assert!(line(&o.out, "route").contains("the cube"));
    assert!(line(&o.out, "E").contains("minimal discharging set"));
}

#[test]
fn every_answer_exits_0_and_a_stopped_run_says_so() {
    // Three maximal states: Ambiguity, an answer like any other — exit 0,
    // nothing on stderr.
    let r = ein(&["solve", THREE, "--maximal", "--single"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        line(&r.out, "verdict").starts_with("Ambiguity"),
        "{}",
        r.out
    );
    assert!(r.err.is_empty(), "{}", r.err);
    // ⊤ is the square cube's one maximal state: Solution, exit 0, cheaply.
    let r = ein(&["solve", SQUARE, "--maximal", "--single", "-s"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(line(&r.out, "verdict").starts_with("Solution"), "{}", r.out);
    assert!(r.err.is_empty(), "{}", r.err);
    // A budget that stops it first: not decided, exit 0, and one `note:`.
    let r = ein(&[
        "solve",
        "examples/lattice/06_friends_tables_everyone_sits.ein",
        "--single",
        "--region-budget",
        "1",
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.err.starts_with("note: "), "{}", r.err);
    assert!(
        r.err.contains("partial — stopped at --region-budget 1"),
        "{}",
        r.err
    );
    assert_eq!(r.err.lines().count(), 1, "{}", r.err);
    assert!(
        line(&r.out, "verdict").starts_with("not decided"),
        "{}",
        r.out
    );
    assert!(
        line(&r.out, "decided").contains("--region-budget 1"),
        "{}",
        r.out
    );
}

#[test]
fn n_inside_a_mode_is_n_states_and_outside_it_is_still_the_walk() {
    let r = ein(&["solve", THREE, "--maximal", "-n", "2"]);
    assert_eq!(r.code, 0);
    assert_eq!(r.out.matches(";; state ").count(), 2, "{}", r.out);
    assert!(
        line(&r.out, "maximal line").contains("may hold more"),
        "{}",
        r.out
    );
    // Until S1e2.1a.5 a run with no region flag is today's walk.
    let w = ein(&["solve", THREE, "-n", "2"]);
    assert!(!w.out.contains("asked "), "{}", w.out);
    assert!(w.out.contains("verdict"), "{}", w.out);
}

#[test]
fn the_walks_options_are_refused_beside_a_mode() {
    for (args, name) in [
        (vec!["--maximal", "-e"], "--exhaustive"),
        (vec!["--single", "-m", "3"], "--max-set-size"),
        (vec!["--minimal", "-L"], "--no-lookahead"),
        (
            vec!["--region-budget", "10", "--events", "/tmp/x.jsonl"],
            "--events",
        ),
        (vec!["--maximal", "--trace", "/tmp/x.md"], "--trace"),
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

/// `--models key|grid`, `-p/-P/-f` and `--diff-models` over the listed states
/// — T1e2.1a.5.2's sixth step, `read_out.md` § 8.3 and § 8.6. Each block
/// follows the read-out, which it leaves as it was, and names its set.
#[test]
fn the_listed_states_print_as_asked() {
    let run = |extra: &[&str]| {
        let mut a = vec!["solve", THREE];
        a.extend(extra.iter());
        ein(&a)
    };
    let check = |extra: &[&str], want: &[&str]| {
        let mode: Vec<&str> = extra
            .iter()
            .take_while(|a| !["--models", "-P", "-p", "-f", "--diff-models"].contains(a))
            .copied()
            .collect();
        let (bare, r) = (run(&mode), run(extra));
        assert_eq!(r.code, 0, "{extra:?}: {}", r.err);
        // The header names the flags, which change what is printed; below
        // it the read-out is as it was.
        let body = |o: &str| {
            o.split_once('\n')
                .map_or(String::new(), |x| x.1.to_string())
        };
        assert!(
            body(&r.out).starts_with(&body(&bare.out)),
            "{extra:?}: the read-out moved"
        );
        for w in want {
            assert!(r.out.contains(w), "{extra:?}: no `{w}` in\n{}", r.out);
        }
    };
    check(
        &["--region-budget", "524288", "--models", "key"],
        &[
            "models — the determining key of every state of R",
            "7 rows, one per state. Not every state is maximal",
        ],
    );
    check(
        &["--maximal", "--models", "key"],
        &["its R is that\n    state alone"],
    );
    check(
        &["--maximal", "-P"],
        &[
            "── state 1 of 3 ──",
            "── state 3 of 3 ──",
            "final-state facts",
        ],
    );
    let dir = scratch("diff");
    std::fs::create_dir_all(&dir).unwrap();
    let want = dir.join("want.txt");
    std::fs::write(
        &want,
        "closes a-prop b-prop c-prop\nmodel\n(a-prop X)\n(b-prop X)\nmodel\n(a-prop X)\n(c-prop X)\n",
    )
    .unwrap();
    let w = want.to_string_lossy().to_string();
    check(
        &["--maximal", "--diff-models", &w],
        &[
            "2 matched, 0 near, 0 missing, 1 spurious",
            "+ spurious: state 3 of 3",
        ],
    );
    // A file it cannot read stops the run before it starts, as the walk's does.
    let r = run(&["--maximal", "--diff-models", "/nonexistent/want.txt"]);
    assert_eq!(r.code, 2);
    assert!(
        r.out.is_empty() && r.err.starts_with("error: --diff-models"),
        "{}",
        r.err
    );
    let _ = std::fs::remove_dir_all(&dir);
    // `--models grid` per state, here with the reason a program without a
    // row type gets; the cells themselves are `models_grid.rs`'s.
    let g = ein(&[
        "solve",
        "examples/branching/04_two_levels.ein",
        "--maximal",
        "--models",
        "grid",
    ]);
    assert_eq!(g.code, 0, "{}", g.err);
    assert!(
        g.out.contains("models — the states listed, each as a grid"),
        "{}",
        g.out
    );
    assert!(
        g.out.contains("── state 2 of 2 ──") && g.out.contains("grid — none:"),
        "{}",
        g.out
    );
}
