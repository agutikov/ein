//! `ein solve`'s region modes and `ein oracle`, through the binary — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4.
//!
//! What the modes *answer* is `ein-infer/tests/explore.rs`'s, held to the
//! oracle there. What is held here is the surface:
//! - the exit codes: every answer exits 0, `--single`'s four words included
//!   (the user's ruling of 2026-10-16), and a stopped run says so on stderr;
//! - the refusals: the nine options S1e2.1a.5's switch retired, each with
//!   what replaced it;
//! - `-j`;
//! - the oracle's directory, which `ein render region` draws with no run
//!   overlaid.
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
fn n_is_n_states_of_the_part_of_r_asked_about() {
    let r = ein(&["solve", THREE, "--maximal", "-n", "2"]);
    assert_eq!(r.code, 0);
    assert_eq!(r.out.matches(";; state ").count(), 2, "{}", r.out);
    assert!(
        line(&r.out, "maximal line").contains("may hold more"),
        "{}",
        r.out
    );
    // Since S1e2.1a.5's switch a bare `-n` asks for N states of all of R.
    let w = ein(&["solve", THREE, "-n", "2"]);
    assert_eq!(w.code, 0);
    assert!(
        line(&w.out, "asked").starts_with("2 states of R (-n 2)"),
        "{}",
        w.out
    );
    assert_eq!(w.out.matches(";; state ").count(), 2, "{}", w.out);
}

#[test]
fn the_retired_options_are_refused_with_what_replaced_them() {
    // The nine the switch retired (`solve_interface.md` § 2.1), and only
    // those: every other option of `solve`'s is a mode's.
    for (args, name) in [
        (vec!["--maximal", "-e"], "--exhaustive"),
        (vec!["--single", "-m", "3"], "--max-set-size"),
        (vec!["--minimal", "-E", "9"], "--max-enterings"),
        (vec!["--minimal", "-o", "score-sum"], "--lattice-order"),
        (vec!["--minimal", "-z"], "--shuffle"),
        (vec!["--minimal", "-d", "3"], "--seed"),
        (
            vec!["--maximal", "--dump-states", "/tmp/x"],
            "--dump-states",
        ),
        (
            vec!["--maximal", "--dump-region", "/tmp/x"],
            "--dump-region",
        ),
        (vec!["--maximal", "--region-states"], "--region-states"),
    ] {
        let mut a = vec!["solve", THREE];
        a.extend(args.iter());
        let r = ein(&a);
        assert_eq!(r.code, 2, "{args:?}");
        // The sentence names the option and what replaced it.
        assert!(
            r.err.starts_with("error: ")
                && r.err.contains(name)
                && (r.err.contains(" is retired") || r.err.contains(" moved")),
            "{args:?}: {}",
            r.err
        );
        // …with or without a region flag beside it.
        let mut bare = vec!["solve", THREE];
        bare.extend(args.iter().skip(1));
        assert_eq!(ein(&bare).err, r.err, "{args:?}");
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
    // A stale `walk.json`, from a `--dump-region` before the switch, does not
    // survive the oracle dumped over it.
    std::fs::write(dir.join("walk.json"), "{}").unwrap();
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

/// `-H` and `--trace` under a region mode — T1e2.1a.5.2's seventh step,
/// `read_out.md` § 8.4 and § 8.6.
#[test]
fn the_universe_and_the_trace() {
    let zebra = "examples/branching/05_mini_zebra.ein";
    let r = ein(&["solve", zebra, "--region-budget", "524288", "-H"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out
            .contains("H — the universe: 9 candidates over 1 relation, 6 dead alone"),
        "{}",
        r.out
    );
    assert_eq!(r.out.matches("   dead alone\n").count(), 6, "{}", r.out);
    let m = ein(&["solve", zebra, "--maximal", "-H"]);
    assert!(m.out.contains("dead alone not known"), "{}", m.out);

    let dir = scratch("trace");
    std::fs::create_dir_all(&dir).unwrap();
    let md = dir.join("t.md");
    let p = md.to_string_lossy().to_string();
    let bare = ein(&["solve", zebra, "--region-budget", "524288"]);
    let t = ein(&["solve", zebra, "--region-budget", "524288", "--trace", &p]);
    assert_eq!(
        (t.code, &t.out, &t.err),
        (bare.code, &bare.out, &bare.err),
        "--trace moved the read-out"
    );
    let text = std::fs::read_to_string(&md).unwrap();
    for want in [
        "# Region trace",
        "  states (k_s)      5",
        "## Root — 24 steps",
        "## State 5 of 5 — 3 candidates · maximal · 4 commitments, 3 minimal",
        "Assuming **(co-located Bob Coffee), (co-located Bob Dog)**.",
        "## N — 6 minimal dead sets",
    ] {
        assert!(text.contains(want), "no `{want}` in the trace:\n{text}");
    }
    // An empty R that owes is traced through its frontier.
    let o = ein(&[
        "solve",
        "tests/stdlib/closure/03_closed_and_owing.ein",
        "--region-budget",
        "524288",
        "--trace",
        &p,
    ]);
    assert_eq!(o.code, 0, "{}", o.err);
    let text = std::fs::read_to_string(&md).unwrap();
    assert!(
        text.contains("## Owing — 1 node") && text.contains("- **∅** owes"),
        "{text}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// `--events FILE` under a region mode — T1e2.1a.5.2's seventh step,
/// `read_out.md` § 8.6: `run` and `load`, root's own events, one `enter` per
/// node visited after that node's events, a `layer` per route-1 layer, and
/// one `region` line last. The same stream at one job and at four, and the
/// read-out it leaves as it was.
#[test]
fn the_event_stream_is_the_runs_and_jobs_move_no_line() {
    let zebra = "examples/branching/05_mini_zebra.ein";
    let dir = scratch("events");
    std::fs::create_dir_all(&dir).unwrap();
    let path = |j: &str| dir.join(format!("{j}.jsonl")).to_string_lossy().to_string();
    let bare = ein(&["solve", zebra, "--region-budget", "524288"]);
    let mut streams = Vec::new();
    for j in ["1", "4"] {
        let p = path(j);
        let r = ein(&[
            "solve",
            zebra,
            "--region-budget",
            "524288",
            "-j",
            j,
            "--events",
            &p,
        ]);
        assert_eq!(
            (r.code, &r.out, &r.err),
            (bare.code, &bare.out, &bare.err),
            "-j {j}"
        );
        let lines: Vec<serde_json::Value> = std::fs::read_to_string(&p)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
            .collect();
        streams.push(lines);
    }
    let kinds = |s: &[serde_json::Value], k: &str| s.iter().filter(|e| e["e"] == k).count();
    let one = &streams[0];
    assert_eq!(one[0]["e"], "run");
    assert_eq!(one[1]["e"], "load");
    let last = one.last().unwrap();
    assert_eq!(last["e"], "region");
    assert_eq!(last["region"]["k_s"], 5);
    assert_eq!(
        kinds(one, "enter") as u64,
        last["work"]["saturations"].as_u64().unwrap()
    );
    assert_eq!(
        kinds(one, "layer") as u64,
        last["work"]["layers"].as_u64().unwrap()
    );
    assert!(
        kinds(one, "fire") > 0,
        "root and the nodes narrate their firings"
    );
    // Every line but `run`, whose argv names the -j, is the same at four jobs.
    assert_eq!(one[1..], streams[1][1..]);
    let _ = std::fs::remove_dir_all(&dir);
}

/// The walk's other options, taken by the modes — T1e2.1a.5.3's first step.
/// `-c` and `-t` add a block to stdout, `-v` and `--layer-progress` write the
/// route's steps to stderr, and the levers `-L`, `-K` and `-y` change nothing
/// printed, header included (C4): no route reads them until P1e2.1c.
#[test]
fn the_modes_take_the_walks_other_options() {
    let zebra = "examples/branching/05_mini_zebra.ein";
    let base = ["solve", zebra, "--region-budget", "524288"];
    let run = |extra: &[&str]| {
        let mut a = base.to_vec();
        a.extend(extra.iter());
        ein(&a)
    };
    let bare = run(&[]);
    assert_eq!(bare.code, 0, "{}", bare.err);
    for lever in [&["-L"][..], &["-K"], &["-y"], &["-L", "-K", "-y"]] {
        let r = run(lever);
        assert_eq!(
            (r.code, &r.out, &r.err),
            (bare.code, &bare.out, &bare.err),
            "{lever:?} moved the read-out"
        );
    }
    let c = run(&["-c"]);
    assert!(
        c.out.starts_with("config (resolved)\n")
            && c.out
                .ends_with(&bare.out[bare.out.find('\n').unwrap() + 1..]),
        "{}",
        c.out
    );
    let cl = run(&["-c", "-L"]);
    assert!(
        cl.out.contains("enable-pre-branch-lookahead      false"),
        "-c shows the lever: {}",
        cl.out
    );
    let t = run(&["-t"]);
    assert!(
        t.out.contains("\ntiming\n") && t.out.contains("  route "),
        "{}",
        t.out
    );
    // Progress goes to stderr, one line per route-1 layer, and stdout stands.
    let p = run(&["--layer-progress"]);
    assert_eq!(p.out, bare.out, "--layer-progress moved stdout");
    let layers: usize = line(&bare.out, "work")
        .split(" · ")
        .find_map(|w| w.strip_suffix(" layers"))
        .and_then(|n| n.trim().parse().ok())
        .expect("a layers count");
    assert_eq!(
        p.err.lines().filter(|l| l.starts_with("layer ")).count(),
        layers,
        "{}",
        p.err
    );
    let v = run(&["-v", "-g", "1"]);
    assert_eq!(v.out, bare.out, "-v moved stdout");
    let saturations: usize = line(&bare.out, "work")
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .expect("a saturations count");
    assert_eq!(
        v.err.lines().filter(|l| l.starts_with("  node ")).count(),
        saturations,
        "{}",
        v.err
    );
}

/// `--dump-lattice DIR` under a mode: every node the route visited, in visit
/// order, the same at one job and at four, and additive.
#[test]
fn the_lattice_dump_is_every_node_visited() {
    let dir = scratch("lattice");
    let mut dumps = Vec::new();
    for (j, mode) in [("1", "--single"), ("4", "--single"), ("1", "--maximal")] {
        let d = dir.join(format!("{j}{mode}"));
        let p = d.to_string_lossy().to_string();
        let bare = ein(&["solve", SQUARE, mode, "-j", j]);
        let r = ein(&["solve", SQUARE, mode, "-j", j, "--dump-lattice", &p]);
        assert_eq!(
            (r.code, &r.out, &r.err),
            (bare.code, &bare.out, &bare.err),
            "{mode} -j {j}: the dump moved the read-out"
        );
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(d.join("nodes.json")).unwrap()).unwrap();
        assert_eq!(doc["schema"], "ein-lattice/2");
        let saturations: u64 = line(&r.out, "work")
            .split(' ')
            .next()
            .and_then(|n| n.replace('\u{a0}', "").parse().ok())
            .expect("a saturations count");
        assert_eq!(doc["visited"].as_u64(), Some(saturations), "{mode}");
        assert_eq!(doc["nodes"].as_array().unwrap().len() as u64, saturations);
        let stepped: u64 = doc["steps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["visited"].as_u64().unwrap())
            .sum();
        assert!(stepped <= saturations, "{mode}");
        assert!(
            std::fs::read_to_string(d.join("nodes.md"))
                .unwrap()
                .starts_with("# Visited nodes — `examples/saturation/square-bwd/floors.ein`")
        );
        dumps.push(doc);
    }
    assert_eq!(dumps[0], dumps[1], "-j moved the dump");
    let _ = std::fs::remove_dir_all(&dir);
}

/// `--trace`'s three shaping flags under a mode: `--relevant` keeps a slice,
/// `--reorder` groups each section by entity, `--full-kb-snapshots` appends
/// each state's KB. None of them moves stdout.
#[test]
fn the_trace_takes_its_three_shaping_flags() {
    let zebra = "examples/branching/05_mini_zebra.ein";
    let dir = scratch("trace-shape");
    std::fs::create_dir_all(&dir).unwrap();
    let trace = |extra: &[&str]| {
        let p = dir.join("t.md").to_string_lossy().to_string();
        let mut a = vec!["solve", zebra, "--single", "--no-diagrams", "--trace", &p];
        a.extend(extra.iter());
        let r = ein(&a);
        assert_eq!(r.code, 0, "{}", r.err);
        (r.out, std::fs::read_to_string(&p).unwrap())
    };
    let (out, plain) = trace(&[]);
    let (o1, relevant) = trace(&["--relevant"]);
    let (o2, reorder) = trace(&["--reorder"]);
    let (o3, full) = trace(&["--full-kb-snapshots"]);
    assert!(
        out == o1 && out == o2 && out == o3,
        "a trace flag moved stdout"
    );
    let steps = |t: &str| t.matches("### Step ").count() + t.matches("**Step ").count();
    assert!(steps(&relevant) < steps(&plain), "{relevant}");
    assert_eq!(steps(&reorder), steps(&plain));
    assert!(reorder.contains("### About "), "{reorder}");
    assert_eq!(
        full.matches("**The state's whole KB.**").count(),
        2,
        "{full}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// `ein oracle`'s five options from `ein solve` — T1e2.1a.5.3's first step,
/// `solve_interface.md` § 2.2.
#[test]
fn the_oracle_takes_the_read_outs_options() {
    let dir = scratch("oracle-options");
    std::fs::create_dir_all(&dir).unwrap();
    let bare = ein(&["oracle", THREE]);
    assert_eq!(bare.code, 0, "{}", bare.err);
    let p = dir.join("s.json").to_string_lossy().to_string();
    let s = ein(&["oracle", THREE, "--json-summary", &p]);
    assert_eq!((s.code, &s.out, &s.err), (bare.code, &bare.out, &bare.err));
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
    assert_eq!(doc["schema"], "ein-summary/2");
    assert_eq!(doc["region"]["k_s"], 7);
    let k = ein(&["oracle", THREE, "--models", "key"]);
    // Below the header, which names the flag, the read-out is the same.
    let body = |s: &str| s[s.find('\n').unwrap()..].to_string();
    assert!(
        body(&k.out).starts_with(&body(&bare.out))
            && k.out.contains("models — the determining key of"),
        "{}",
        k.out
    );
    let pp = ein(&["oracle", THREE, "-P"]);
    assert_eq!(pp.out.matches("── state ").count(), 7, "{}", pp.out);
    let t = ein(&[
        "oracle",
        "examples/lattice/07_two_pairs_two_tables.ein",
        "-T",
        "0",
    ]);
    assert_eq!(t.code, 2, "{}", t.err);
    assert!(t.err.contains("--max-time"), "{}", t.err);
    let _ = std::fs::remove_dir_all(&dir);
}

/// **`ein solve FILE --monotone`**, the explicit check the user approved on
/// 2026-11-03 (M1e2 S1e2.1b.5, `detector_proposal.md` § 2.2): a question
/// about the program, answered by root's fixpoint and the static check. The
/// blocks are the proposal's drafted ones, byte for byte (its monotone one on
/// `lattice/02` rather than zebra, which this file does not anchor on); every
/// verdict exits 0; it is exclusive with the questions about R; and its
/// summary says what was asked and describes no region.
#[test]
fn the_monotone_question_is_about_the_program() {
    let header = |file: &str| {
        format!(
            "solve · {file} --monotone\n\
             ──────────────────────────────────────────────────────────────\n\
             \x20 asked             is the program monotone? — the static check, on root's fixpoint\n"
        )
    };
    let tail = |n: &str| {
        format!(
            "  universe          {n}\n\
             \x20 route             none — a question about the program: root's fixpoint, then the static check\n\
             \x20 work              1 saturation, root's · 0 facts beyond root\n"
        )
    };
    let assumes = "    assumes         every node is a fork of root (root's fixpoint, its commitment \
                   added, saturated); what a node is on this program is open: Q-M1e2.2, F21\n";
    let cases: Vec<(&str, String)> = vec![
        (
            THREE,
            format!(
                "  monotone          yes — no site: dead and discharged are upward-closed, so R \
                 has no hole and its two lines describe it\n{}",
                tail("3 candidates")
            ),
        ),
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            format!(
                "  monotone          no — 1 site\n{assumes}\
                 \x20   site            bad: absent reads q at absent depth 1, concludes (false) — \
                 at stake: the state, and dead with it\n\
                 \x20   bit             not computed — the cube is 2^2 = 4 saturations: `ein \
                 oracle FILE` computes it, and so does a bare `ein solve FILE`\n{}",
                tail("2 candidates")
            ),
        ),
        (
            "examples/features/07_unstratifiable.ein",
            format!(
                "  monotone          no — 2 sites\n{assumes}\
                 \x20   site            derive-p: absent reads q at absent depth 1, concludes a \
                 positive fact — at stake: the state\n\
                 \x20   site            derive-q: absent reads p at absent depth 1, concludes a \
                 positive fact — at stake: the state\n\
                 \x20   cycle           even through an absent — derive-p: q →¬ p · derive-q: p \
                 →¬ q — several states with no stale fact can exist\n\
                 \x20   order           derive-p and derive-q each read under an absent what the \
                 other concludes, both at priority 200: which the boundary admits first is the \
                 order they are written in\n\
                 \x20   bit             not computed — the cube is 2^1 = 2 saturations: `ein \
                 oracle FILE` computes it, and so does a bare `ein solve FILE`\n{}",
                tail("1 candidate")
            ),
        ),
        (
            "examples/lattice/06_friends_tables.ein",
            format!(
                "  monotone          no — 1 site\n{assumes}\
                 \x20   site            group-seated-owed: obligation-trigger reads sits at \
                 absent depth 0, concludes (open …) — at stake: the status: owes\n\
                 \x20   bit             not computed — the cube is 2^19 nodes, past \
                 --region-limit 16\n{}",
                tail("19 candidates")
            ),
        ),
    ];
    for (file, body) in &cases {
        let r = ein(&["solve", file, "--monotone"]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        assert_eq!(r.out, format!("{}{body}", header(file)), "{file}");
        assert_eq!(r.err, "", "{file}");
    }
    for other in ["--minimal", "--maximal", "--single", "-n 2"] {
        let mut args = vec!["solve", THREE, "--monotone"];
        args.extend(other.split(' '));
        let r = ein(&args);
        assert_eq!(r.code, 2, "--monotone {other}: {}", r.out);
        assert!(r.err.contains("cannot be used with"), "{other}: {}", r.err);
    }
    let dir = scratch("monotone");
    std::fs::create_dir_all(&dir).expect("scratch");
    let summary = dir.join("s.json");
    let answer = dir.join("answer");
    let r = ein(&[
        "solve",
        "examples/ein-bugs/naf-upward-closure.ein",
        "--monotone",
        "--json-summary",
        summary.to_str().unwrap(),
        "--dump-answer",
        answer.to_str().unwrap(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).expect("written")).expect("JSON");
    assert_eq!(s["asked"]["ask"], "monotone");
    assert!(s["region"].is_null());
    assert_eq!(s["work"]["saturations"], 1);
    assert_eq!(s["monotonicity"]["monotone"], false);
    assert_eq!(s["monotonicity"]["sites"][0]["at_stake"], "state, dead");
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.txt")).expect("written"),
        r.out
    );
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.json")).expect("written"),
        std::fs::read_to_string(&summary).expect("written")
    );
    let _ = std::fs::remove_dir_all(&dir);
}
