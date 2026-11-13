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
//! Since T1e2.1a.5.2 the modes print the read-out
//! `docs/kernel/inference/read_out.md` defines (since M1f S1f.5.16), whose
//! golden is its `read_out_examples.md` (`read_out_examples.rs`); the rows
//! here are read by label.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

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
    Scratch::new(&format!("ein-region-modes-{tag}"))
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
    // The nine the switch retired (`solve_interface.md` § 2.1) and the two
    // levers S1e2.1c.5 retired (the user's ruling of 2026-11-03), and only
    // those: every other option of `solve`'s is a mode's or a check.
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
        (vec!["--minimal", "-L"], "--no-lookahead"),
        (vec!["--minimal", "-K"], "--no-kill-cache"),
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
    let tmp = scratch("oracle");
    let dir = tmp.join("d");
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
}

/// **`--diff-models` compares by value** — M1f S1f.5.5, T1f.5.5.6 (S-15), the
/// user's ruling of 2026-11-07.
///
/// The file keeps its line format, and the facts between two `model` lines are
/// read by the generated parser and printed by the one printer, which prints
/// the states' facts too, so the same fact in another spelling matches. The
/// five rows of the stage's table, re-taken on `916d97ac` on the inventory's
/// probe: until this task the second to fourth matched nothing (`0 matched … 1
/// missing`), and the fifth, `(p a` with no `)`, was compared as written, with
/// no error. Since, a fact the parser refuses is a usage error, exit 2, before
/// the run, with every error recovery reaches. The probe's state held `(q "x
/// y")` too, a string, which S1f.5.2 refuses as a value; the state here holds
/// `(p a)` alone, which `closes p` compares. A fact that spans lines is the
/// sixth case.
#[test]
fn diff_models_reads_a_models_facts_as_ein() {
    let dir = scratch("diff-by-value");
    let program = dir.join("one.ein");
    std::fs::write(&program, "(relation p T :why \"p holds of {?1}\")\n(p a)\n")
        .expect("the program is written");
    let p = program.to_string_lossy().to_string();
    for (n, fact) in [
        "(p a)",
        "(p  a)",
        "( p a )",
        "(p a) ; the fact",
        "(p\n   a)",
    ]
    .iter()
    .enumerate()
    {
        let want = dir.join(format!("want-{n}.txt"));
        std::fs::write(&want, format!("closes p\nmodel\n{fact}\n")).expect("written");
        let r = ein(&["solve", &p, "--diff-models", &want.to_string_lossy()]);
        assert_eq!(r.code, 0, "{fact:?}: {}", r.err);
        assert!(
            r.out
                .contains("1 matched, 0 near, 0 missing, 0 spurious — the sets agree"),
            "{fact:?}:\n{}",
            r.out
        );
    }
    let bad = dir.join("unclosed.txt");
    std::fs::write(&bad, "closes p\nmodel\n(p a\n").expect("written");
    let b = bad.to_string_lossy().to_string();
    let r = ein(&["solve", &p, "--diff-models", &b]);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(r.out.is_empty(), "it ran:\n{}", r.out);
    assert!(
        r.err.starts_with(&format!(
            "error: --diff-models {b}: a model's facts do not parse\n"
        )),
        "{}",
        r.err
    );
    assert!(
        r.err
            .lines()
            .any(|l| l == format!("{b}:4:1: unexpected end of input")),
        "{}",
        r.err
    );
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
}

/// The walk's other options, taken by the modes — T1e2.1a.5.3's first step.
/// `-c` and `-t` add a block to stdout, `-v` and `--layer-progress` write the
/// route's steps to stderr, and `-y` adds its `paths` row and nothing else,
/// header included (C4): the spot check, unbounded, the user's ruling of
/// 2026-11-03 (M1e2 S1e2.1c.5), which retired `-L` and `-K` — refused, as
/// the switch's nine are, and held by `the_retired_options_are_refused_with_what_replaced_them`.
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
    let y = run(&["-y"]);
    let without = |t: &str| -> String {
        t.lines()
            .filter(|l| !l.starts_with("    paths "))
            .map(|l| format!("{l}\n"))
            .collect()
    };
    assert_eq!(
        (y.code, without(&y.out), &y.err),
        (bare.code, without(&bare.out), &bare.err),
        "-y moved the read-out past its own row"
    );
    assert!(
        y.out
            .contains("\n    paths           every parent path checked reaches the same state")
            && y.out.contains("(-y: every one-step parent)"),
        "-y on a monotone program prints its row: {}",
        y.out
    );
    let c = run(&["-c"]);
    assert!(
        c.out.starts_with("config (resolved)\n")
            && c.out
                .ends_with(&bare.out[bare.out.find('\n').unwrap() + 1..]),
        "{}",
        c.out
    );
    let cl = run(&["-c", "-y"]);
    assert!(
        cl.out.contains("lattice-sanity-check             true"),
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
}

/// `--trace`'s three shaping flags under a mode: `--relevant` keeps a slice,
/// `--reorder` groups each section by entity, `--full-kb-snapshots` appends
/// each state's KB. None of them moves stdout.
#[test]
fn the_trace_takes_its_three_shaping_flags() {
    let zebra = "examples/branching/05_mini_zebra.ein";
    let dir = scratch("trace-shape");
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
}

/// **A step names every conclusion of its firing**, in the order the rule
/// writes them (M1f S1f.6.25, E-5 of P1f.6's `findings.md`). Until that stage
/// the *Derives* line, and `--reorder`'s arrow, named the first alone: a
/// second conclusion was stored, held by the state and read by later steps,
/// and no step said where it came from. Each conclusion carries its own
/// statement, its relation's `:why`; a rule of one conclusion prints the line
/// it always did.
#[test]
fn a_trace_step_names_every_conclusion_of_its_firing() {
    let dir = scratch("trace-conclusions");
    let file = dir.join("two.ein");
    std::fs::write(
        &file,
        "(relation p Thing :why \"{?1} is p\")\n\
         (relation q Thing :why \"{?1} is q\")\n\
         (relation r Thing Thing :why \"{?1} is related to {?2}\")\n\
         (p a)\n\
         (rule many () :match (p ?x)\n\
           :assert (and (q ?x) (flag ?x) (not (r ?x b)) (r ?x ?x))\n\
           :why \"{?x} is q, flagged and related to itself\")\n\
         (rule one () :match (flag ?x) :assert (seen ?x) :why \"{?x} is seen\")\n\
         (solve :hypothesis-relations ())\n",
    )
    .unwrap();
    let file = file.to_string_lossy().to_string();
    let trace = |extra: &[&str]| {
        let p = dir.join("t.md").to_string_lossy().to_string();
        let mut a = vec!["solve", &file, "--no-diagrams", "--trace", &p];
        a.extend(extra.iter());
        let r = ein(&a);
        assert_eq!(r.code, 0, "{}", r.err);
        std::fs::read_to_string(&p).unwrap()
    };
    let plain = trace(&[]);
    assert!(
        plain.contains(
            "Derives `q(a)` — a is q; `flag(a)`; `not(r(a, b))`; `r(a, a)` — a is related to a\n"
        ),
        "{plain}"
    );
    // One conclusion: the line as it was, a full stop where no statement ends it.
    assert!(plain.contains("Derives `seen(a)`.\n"), "{plain}");
    let reorder = trace(&["--reorder"]);
    assert!(
        reorder.contains("· `many` → `q(a)`, `flag(a)`, `not(r(a, b))`, `r(a, a)` — "),
        "{reorder}"
    );
    assert!(reorder.contains("· `one` → `seen(a)` — "), "{reorder}");
}

/// `n` items, one open relation over them, and optionally the rule *if any
/// item is chosen every item is*: with it two states, ∅ and ⊤, and 2ⁿ
/// commitments; without it the whole cube, 2ⁿ states.
fn items(n: usize, all: bool) -> String {
    let mut s = String::from("(relation chosen Thing :why \"{?1} is chosen\")\n");
    if all {
        s.push_str(
            "(rule all () :match (and (chosen ?x) (item ?y)) :assert (chosen ?y)\n  \
             :why \"{?x} is chosen, so {?y} is\")\n",
        );
    }
    for i in 1..=n {
        s.push_str(&format!("(item x{i})\n"));
    }
    s
}

/// **A count past what a run reports is an answer** — M1f S1f.6.25, E-22 of
/// P1f.6's `findings.md`, W92 (Q-M1f.218).
///
/// Until that stage the state route counted a state's commitments in `u128`
/// and summed R's into `u64`, and where either overflowed a bare `ein solve`,
/// `--count` and `--json-summary` printed *a state's commitments pass 2^128:
/// the state route cannot count them* and exited 1, with no read-out, though
/// every commitment was classified; from 2⁶³ the summary wrote the count as a
/// negative number. The count is exact now, and a surface carries it up to
/// 2⁶³ − 1, which reads *this many or more*.
#[test]
fn a_count_past_the_cap_is_an_answer_on_every_surface() {
    const CAP: &str = "9 223 372 036 854 775 807";
    const CAP_JSON: u64 = 9_223_372_036_854_775_807;
    let dir = scratch("count-cap");
    let write = |name: &str, text: String| -> String {
        let p = dir.join(name);
        std::fs::write(&p, text).unwrap();
        p.to_string_lossy().to_string()
    };
    // Below the cap a count is exact, as it was: 2⁶² commitments.
    let below = write("below.ein", items(62, true));
    let r = ein(&["solve", &below]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out
            .contains("  commitments (k_c) 4 611 686 018 427 387 904\n"),
        "{}",
        r.out
    );
    // 2⁶³, which the read-out printed and the summary wrote negative; 2⁶⁴,
    // the smallest that was an exit 1; and 2¹³⁰, one state's past `u128`.
    for n in [63, 64, 130] {
        let file = write(&format!("all{n}.ein"), items(n, true));
        let summary = dir.join(format!("s{n}.json")).to_string_lossy().to_string();
        let answer = dir.join(format!("a{n}")).to_string_lossy().to_string();
        let r = ein(&[
            "solve",
            &file,
            "--json-summary",
            &summary,
            "--dump-answer",
            &answer,
        ]);
        assert_eq!((r.code, r.err.as_str()), (0, ""), "{n}");
        for row in [
            "  states (k_s)      2\n".to_string(),
            format!("  commitments (k_c) {CAP} or more   (2^63 − 1: the run counts no further)\n"),
            "  exhausted         yes — every commitment is classified: this is all of R\n"
                .to_string(),
            "  components        1\n".to_string(),
            format!("· maximal · {CAP} or more commitments, {n} minimal"),
        ] {
            assert!(r.out.contains(&row), "{n}: no {row:?} in\n{}", r.out);
        }
        // The `(merged …)` comment, in the read-out written uncut: stdout
        // stops at a hundred lines, which 130 candidates pass.
        let uncut = std::fs::read_to_string(format!("{answer}/answer.txt")).unwrap();
        let merged = format!(";; state 2 is reached by {CAP} or more commitments; its {n} minimal");
        assert!(uncut.contains(&merged), "{n}: no {merged:?} in\n{uncut}");
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&summary).unwrap()).unwrap();
        assert_eq!(doc["region"]["k_s"], 2, "{n}");
        assert_eq!(doc["region"]["k_c"], CAP_JSON, "{n}");
        assert_eq!(doc["region"]["exhausted"], true, "{n}");
        assert_eq!(doc["region"]["components"][0], CAP_JSON, "{n}");
        let state = std::fs::read_to_string(format!("{answer}/models/02.ein")).unwrap();
        assert!(
            state.contains(";;; reached by: 9223372036854775807 or more commitments\n"),
            "{n}: {}",
            &state[..state.len().min(600)]
        );
        // `--count` is describe's question with the listing left out.
        let c = ein(&["solve", &file, "--count"]);
        assert_eq!((c.code, c.err.as_str()), (0, ""), "{n}");
        assert!(
            c.out.contains(&format!(
                "  commitments (k_c) {CAP} or more   (2^63 − 1: the run counts no further)\n"
            )),
            "{n}: {}",
            c.out
        );
    }
    // The whole cube on a partial run: R is decided by ∅ and ⊤, its count is
    // 2⁶⁴, and the run stops at its budget, which is its own row and note.
    let cube = write("cube64.ein", items(64, false));
    let r = ein(&["solve", &cube, "--region-budget", "2000"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.contains(&format!(
            "  commitments (k_c) {CAP} or more   (2^63 − 1: the run counts no further)\n"
        )),
        "{}",
        r.out
    );
    assert!(
        r.out.contains("  exhausted         no — R is decided"),
        "{}",
        r.out
    );
}

/// `ein oracle`'s five options from `ein solve` — T1e2.1a.5.3's first step,
/// `solve_interface.md` § 2.2.
#[test]
fn the_oracle_takes_the_read_outs_options() {
    let dir = scratch("oracle-options");
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
        // `ein-bugs/naf-upward-closure.ein` was this row until *forbid*
        // refused it (M1e2 S1e2.2.5): `--monotone` takes the universe and runs
        // the static check, and the refusal is read there. Its refusal is held
        // below.
        (
            "examples/lattice/08_absent_witness_hole.ein",
            format!(
                "  monotone          no — 1 site\n{assumes}\
                 \x20   site            soup-unless-allergic: absent reads allergic at absent \
                 depth 1, concludes a positive fact — at stake: the state\n\
                 \x20   bit             not computed — the cube is 2^3 = 8 saturations: `ein \
                 oracle FILE` computes it, and so does a bare `ein solve FILE`\n{}",
                tail("3 candidates")
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
                 other concludes: which the boundary admits first is the \
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
    let refused = ein(&[
        "solve",
        "examples/ein-bugs/naf-upward-closure.ein",
        "--monotone",
    ]);
    assert_eq!(refused.code, 1, "{}", refused.out);
    assert!(refused.out.is_empty(), "{}", refused.out);
    assert!(refused.err.starts_with("rule 'bad': "), "{}", refused.err);
    for other in ["--minimal", "--maximal", "--single", "-n 2"] {
        let mut args = vec!["solve", THREE, "--monotone"];
        args.extend(other.split(' '));
        let r = ein(&args);
        assert_eq!(r.code, 2, "--monotone {other}: {}", r.out);
        assert!(r.err.contains("cannot be used with"), "{other}: {}", r.err);
    }
    let dir = scratch("monotone");
    let summary = dir.join("s.json");
    let answer = dir.join("answer");
    let r = ein(&[
        "solve",
        "examples/lattice/08_absent_witness_hole.ein",
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
    assert_eq!(s["monotonicity"]["sites"][0]["at_stake"], "state");
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.txt")).expect("written"),
        r.out
    );
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.json")).expect("written"),
        std::fs::read_to_string(&summary).expect("written")
    );
}

/// **`ein solve FILE --count`** — M1e2 T1e2.1c.4.6, the user's ruling of
/// 2026-11-03: R's two counts and the rows that qualify them, describe's
/// route, nothing listed. The block on `pets` is T1e2.1c.4.5's draft, the one
/// the user approved, byte for byte. Every option that asks what `--count`
/// does not answer is a usage error naming the command that does; a claim the
/// program states is not checked, and one row and a `note:` per claim say so;
/// `--dump-answer` writes the read-out and the summary and no state, and
/// clears what a describe's dump left; the levers, `-t`, `--events` and
/// `--dump-lattice` are taken. What its counts are is held to describe's and
/// the oracle's elsewhere (`read_out.rs`, `explore.rs`, `lever_invariance.rs`).
/// Since M1e2 S1e2.2.10 the block carries `universe`'s `types` row: `pets.ein`'s
/// blind enumerator binds `Pet`, a type, as an object.
#[test]
fn the_count_question_is_describes_counts() {
    let pets = "examples/saturation/type-exclusivity/pets.ein";
    let r = ein(&["solve", pets, "--count"]);
    assert_eq!((r.code, r.err.as_str()), (0, ""), "{}", r.err);
    assert_eq!(
        r.out,
        format!(
            "solve · {pets} --count\n\
             ──────────────────────────────────────────────────────────────\n\
             \x20 asked             how many states and commitments R holds (--count)\n\
             \x20 states (k_s)      11 339\n\
             \x20 commitments (k_c) 11 339\n\
             \x20 exhausted         yes — every commitment is classified: this is all of R\n\
             \x20 ∅                 in R — root is alive and owes nothing\n\
             \x20 lattice           monotone (the static check) — R is convex: no hole\n\
             \x20 universe          15 candidates, 0 dead alone\n\
             \x20   types           Pet — on the right of an is-a, and 9 candidates of \
             co-located, is-a name it as an object: the blind enumerator is untyped, and no \
             signature names it\n\
             \x20 route             the state route, every alive state once — L1 dead is \
             upward-closed · L3 a commitment's state is its closure's · L4 the lines are local\n\
             \x20 work              11 351 saturations · 76 743 facts beyond root · 14 layers\n"
        )
    );
    // `-y` reads no state under `--count` either: its row says what checking
    // the states met would cost, and nothing else moves. `-t` adds its block.
    let l = ein(&["solve", pets, "--count", "-y"]);
    let rest: String = l
        .out
        .lines()
        .filter(|x| !x.starts_with("    paths "))
        .map(|x| format!("{x}\n"))
        .collect();
    assert_eq!((l.code, &rest, &l.err), (r.code, &r.out, &r.err), "-y");
    assert!(
        l.out.contains("    paths           not checked"),
        "{}",
        l.out
    );
    let t = ein(&["solve", pets, "--count", "-t"]);
    assert!(
        t.out.contains("\n  read-out ") && t.out.contains("none: --count reads no state"),
        "{}",
        t.out
    );

    // What it does not answer, each refused with the command that does.
    for (other, names) in [
        ("--minimal", "a bare `ein solve FILE` describes R"),
        ("--maximal", "a bare `ein solve FILE` describes R"),
        ("-n 2", "`ein solve FILE -n N`"),
        ("--single", "`ein solve FILE --single`"),
        ("--monotone", "`ein solve FILE --monotone`"),
        ("--models key", "`ein solve FILE --models FORM`"),
        ("--models list", "`ein solve FILE --models FORM`"),
        ("--diff-models x", "`ein solve FILE --diff-models FILE`"),
        ("-p", "`ein solve FILE -p`"),
        ("-P", "`ein solve FILE -P`"),
        ("-f", "`ein solve FILE -f`"),
        ("-H", "`ein solve FILE -H`"),
        ("--trace x.md", "`ein solve FILE --trace FILE.md`"),
    ] {
        let mut args = vec!["solve", THREE, "--count"];
        args.extend(other.split(' '));
        let x = ein(&args);
        assert_eq!((x.code, x.out.as_str()), (2, ""), "--count {other}");
        assert!(
            x.err.starts_with("error: --count ") && x.err.contains(names),
            "--count {other}: {}",
            x.err
        );
    }

    // A claim is not checked, and says so: one row, a note per claim, exit 0.
    let claimed = "tests/region/03_merged_or.ein";
    let c = ein(&["solve", claimed, "--count"]);
    assert_eq!(c.code, 0, "{}", c.err);
    assert_eq!(
        line(&c.out, "claims"),
        "not checked — 1 claim, the (solve :expect): --count reads no state, so it checks no \
         claim; a bare `ein solve FILE` checks it"
    );
    assert_eq!(
        c.err,
        format!(
            "note: {claimed}: (solve :expect) NOT CHECKED — --count reads no state, so it checks \
             no claim: a bare `ein solve FILE` checks it\n"
        )
    );
    assert!(
        !c.out.contains("(region") && !c.out.contains("query "),
        "{}",
        c.out
    );

    // A flagged program keeps the warning, and its spot check is not run
    // (`naf-upward-closure` until *forbid* refused it, M1e2 S1e2.2.5).
    let flagged = ein(&[
        "solve",
        "examples/lattice/08_absent_witness_hole.ein",
        "--count",
    ]);
    assert_eq!(flagged.code, 0, "{}", flagged.err);
    for row in ["assumes", "site", "status", "state", "paths"] {
        assert!(
            flagged
                .out
                .lines()
                .any(|l| l.starts_with(&format!("    {row} "))),
            "no `{row}` row: {}",
            flagged.out
        );
    }
    assert!(line(&flagged.out, "paths").starts_with("not checked — "));
    assert!(!flagged.out.contains("minimal hole"), "{}", flagged.out);

    // The summary and the answer directory: the read-out and the summary, no
    // state, and nothing a describe's dump left behind.
    let dir = scratch("count");
    let summary = dir.join("s.json");
    let answer = dir.join("answer");
    let a = answer.to_str().unwrap();
    assert_eq!(ein(&["solve", claimed, "--dump-answer", a]).code, 0);
    assert!(answer.join("index.json").exists() && answer.join("models").exists());
    let d = ein(&[
        "solve",
        claimed,
        "--count",
        "--json-summary",
        summary.to_str().unwrap(),
        "--dump-answer",
        a,
    ]);
    assert_eq!((d.code, &d.out, &d.err), (c.code, &c.out, &c.err));
    let mut written: Vec<String> = std::fs::read_dir(&answer)
        .expect("written")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    written.sort();
    assert_eq!(written, ["answer.json", "answer.txt"]);
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.txt")).expect("written"),
        c.out
    );
    assert_eq!(
        std::fs::read_to_string(answer.join("answer.json")).expect("written"),
        std::fs::read_to_string(&summary).expect("written")
    );
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).expect("written")).expect("JSON");
    assert_eq!(s["asked"]["ask"], "count");
    assert_eq!(
        (s["region"]["k_s"].as_u64(), s["region"]["k_c"].as_u64()),
        (Some(4), Some(8))
    );
    assert!(s["region"]["states"].is_null() && s["region"]["frontier"].is_null());
    assert_eq!(s["claim"]["outcome"], "not-checked");
}

/// The static rows of a read-out — `assumes`, `site`, `…`, `cycle`, `order`
/// — in order: what `--monotone` prints after its `monotone` row, and what a
/// flagged read-out's `lattice` block prints before its `status`.
fn static_rows(out: &str) -> Vec<&str> {
    out.lines()
        .filter(|l| {
            ["assumes", "site", "…", "cycle", "order"].iter().any(|k| {
                l.strip_prefix("    ")
                    .is_some_and(|r| r.starts_with(&format!("{k} ")))
            })
        })
        .collect()
}

/// **One verdict** — M1e2 S1e2.1b.5's test (c) 5. On every corpus program,
/// `ein solve FILE --monotone`'s verdict and sites are byte-equal to a
/// read-out's `lattice` block (a describe stopped at once: `--region-limit
/// 0 --region-budget 1`, which prints the block as every run does), to both
/// runs' summaries' `monotonicity` blocks, and to `ein test`'s report wherever
/// a `(solve … :expect …)` claim gives the file a row with `monotone`. One
/// function, `monotone::classify`, and no surface that disagrees with it.
#[test]
fn one_verdict_on_every_corpus_program() {
    let dir = scratch("one-verdict");
    let report = dir.join("report.json");
    let r = ein(&[
        "test",
        "examples",
        "tests",
        "stdlib",
        "--json-report",
        report.to_str().unwrap(),
    ]);
    // `examples/broken/` exits 2, as a load error does under `ein test`; the
    // report is written whatever the code.
    assert!(report.exists(), "{}", r.err);
    let rows: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&report).expect("written")).expect("JSON");
    let tested: std::collections::BTreeMap<String, bool> = rows["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter_map(|row| {
            let m = row["region"]["monotone"].as_bool()?;
            Some((row["path"].as_str()?.to_string(), m))
        })
        .collect();
    let files: Vec<String> = ein_corpus::corpus_files()
        .iter()
        .map(|p| {
            p.strip_prefix(repo_root())
                .unwrap_or(p)
                .display()
                .to_string()
        })
        .collect();
    let failures = std::sync::Mutex::new(Vec::<String>::new());
    let unread = std::sync::Mutex::new(Vec::<String>::new());
    let (checked, flagged, refused) = (
        std::sync::atomic::AtomicUsize::new(0),
        std::sync::atomic::AtomicUsize::new(0),
        std::sync::atomic::AtomicUsize::new(0),
    );
    let jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(8));
    std::thread::scope(|scope| {
        for w in 0..jobs {
            let (files, dir, tested) = (&files, &dir, &tested);
            let (failures, checked, flagged, unread, refused) =
                (&failures, &checked, &flagged, &unread, &refused);
            scope.spawn(move || {
                for (i, file) in files.iter().enumerate().skip(w).step_by(jobs) {
                    let (a, b) = (
                        dir.join(format!("{i}-m.json")),
                        dir.join(format!("{i}-d.json")),
                    );
                    let asked = ein(&[
                        "solve",
                        file,
                        "--monotone",
                        "--json-summary",
                        a.to_str().unwrap(),
                    ]);
                    if asked.code != 0 {
                        // A file that does not load: no verdict to compare.
                        // A refused one (M1e2 S1e2.2.5) is refused alike by
                        // the read-out, with the same lines.
                        if asked.err.starts_with("rule '") {
                            let d = ein(&["solve", file, "--region-limit", "0"]);
                            if (d.code, &d.err) != (asked.code, &asked.err) {
                                failures.lock().expect("one writer at a time").push(format!(
                                    "{file}: `--monotone` refuses and the read-out exits {}: {}",
                                    d.code, d.err
                                ));
                            }
                            refused.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        continue;
                    }
                    let described = ein(&[
                        "solve",
                        file,
                        "--region-limit",
                        "0",
                        "--region-budget",
                        "1",
                        "--json-summary",
                        b.to_str().unwrap(),
                    ]);
                    // A refuted claim exits 1 and prints its read-out
                    // whole (W28's witnesses, M1e2 S1e2.3.8), so it is read.
                    if described.code != 0 && !described.out.contains("\n  lattice ") {
                        // A read-out that errs prints none: a goal the
                        // compiler rejected ended the run where a state is
                        // answered from it (`read_out.rs`), and `--monotone`
                        // answers no query; the loader refuses such a goal
                        // since M1f S1f.5.21, so none does.
                        unread
                            .lock()
                            .expect("one writer at a time")
                            .push(file.clone());
                        continue;
                    }
                    let mut bad = Vec::new();
                    let monotone = line(&asked.out, "monotone").starts_with("yes");
                    let Some(lattice) = described
                        .out
                        .lines()
                        .find_map(|l| l.strip_prefix("  lattice "))
                        .map(str::trim_start)
                    else {
                        failures.lock().expect("one writer at a time").push(format!(
                            "{file}: the read-out has no lattice row (exit {}): {}",
                            described.code, described.err
                        ));
                        continue;
                    };
                    if lattice.starts_with("monotone") != monotone {
                        bad.push(format!(
                            "verdict: `--monotone` {monotone}, lattice row `{lattice}`"
                        ));
                    }
                    if !monotone {
                        flagged.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let sites = line(&asked.out, "monotone").trim_start_matches("no — ");
                        if !lattice
                            .starts_with(&format!("not monotone: {sites} (the static check)"))
                        {
                            bad.push(format!("sites: `{sites}` against `{lattice}`"));
                        }
                    }
                    if static_rows(&asked.out) != static_rows(&described.out) {
                        bad.push(format!(
                            "static rows:\n{}\nagainst\n{}",
                            static_rows(&asked.out).join("\n"),
                            static_rows(&described.out).join("\n")
                        ));
                    }
                    let block = |p: &std::path::Path| -> serde_json::Value {
                        let s: serde_json::Value =
                            serde_json::from_str(&std::fs::read_to_string(p).expect("a summary"))
                                .expect("JSON");
                        let m = &s["monotonicity"];
                        serde_json::json!([
                            m["monotone"],
                            m["sites"],
                            m["assumes"],
                            m["cycles"],
                            m["order"]
                        ])
                    };
                    if block(&a) != block(&b) {
                        bad.push("the summaries' monotonicity blocks differ".to_string());
                    }
                    if let Some(&m) = tested.get(file.as_str())
                        && m != monotone
                    {
                        bad.push(format!("`ein test`'s row says monotone = {m}"));
                    }
                    checked.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if !bad.is_empty() {
                        failures
                            .lock()
                            .expect("one writer at a time")
                            .push(format!("{file}: {}", bad.join("; ")));
                    }
                }
            });
        }
    });
    let failures = failures.into_inner().expect("joined");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    let mut unread = unread.into_inner().expect("joined");
    unread.sort();
    assert!(
        unread.is_empty(),
        "the programs whose read-out errs, and so has no lattice block to compare: {unread:?}. \
         None since M1f S1f.5.21, whose loader refuses a goal's relation variable (W64): until \
         then `query-goal-free-head.ein` and `features/02_star_in_identifiers.ein`, whose \
         `--monotone` answered where the read-out stopped on the goal, the compiler's line; and \
         `W57/assert-var-under-commitment` a third from M1f S1f.5.10's T1f.5.10.1 until \
         T1f.5.10.2 refused it at load"
    );
    let (checked, flagged, refused) = (
        checked.into_inner(),
        flagged.into_inner(),
        refused.into_inner(),
    );
    // 2026-11-04 (M1e2 S1e2.2.5): *forbid* refuses nine corpus programs, which
    // were checked and flagged until then, and `features/05` and `/12` closed
    // a relation and are monotone; 26 flagged before.
    assert!(
        checked > 190 && flagged >= 14 && refused >= 9,
        "{checked} checked, {flagged} flagged, {refused} refused"
    );
    assert!(
        tested.values().any(|m| !m),
        "no flagged program carries a solve claim: the `ein test` half reads nothing"
    );
}

/// The W27 witness: a flagged program of 17 candidates, past the default
/// `--region-limit`, whose R the oracle gives at 17 (three states, minimal line
/// {∅}).
const PAST: &str = "examples/ein-bugs/wrong-answers/past-limit-lines.ein";

/// **Where ∅ is in R, every minimal question stops at root, and says only ∅** —
/// M1f S1f.5.6, the user's ruling of 2026-11-07 on S-16, *"b, use the ∅ rule
/// in both"*: ∅ is below every commitment, so where it is in R the minimal
/// line is {∅}, decided after one saturation, on any program and at any
/// limit. The three rows of the stage's table, through the binary, on the W27
/// witness past the limit, where describe certifies {∅} and until S1f.5.6
/// `--single` said *not decided*, `-n N` *as asked* or a lower bound, and
/// `--minimal` *exhausted no*, each after 2^16 saturations but `-n 1`.
#[test]
fn where_empty_is_in_r_every_minimal_question_stops_at_root() {
    let d = ein(&["solve", PAST]);
    assert_eq!(d.code, 0, "{}", d.err);
    assert_eq!(
        line(&d.out, "∅"),
        "in R — root is alive and owes nothing",
        "{}",
        d.out
    );
    assert!(
        line(&d.out, "exhausted").starts_with("no — not monotone"),
        "{}",
        d.out
    );
    let line_rows = [
        ("minimal line", "1 state — all of them"),
        ("exhausted", "yes — this is all of the minimal line"),
    ];
    let single_rows = [
        ("verdict", "Solution — the minimal line is one state"),
        ("decided", "yes — the minimal line holds no other state"),
    ];
    for (args, rows) in [
        (&["--minimal", "-n", "1"][..], line_rows),
        (&["--minimal", "-n", "2"][..], line_rows),
        (&["--minimal", "--single"][..], single_rows),
        (&["--minimal"][..], line_rows),
    ] {
        let mut argv = vec!["solve", PAST];
        argv.extend_from_slice(args);
        let r = ein(&argv);
        let tag = args.join(" ");
        assert_eq!(r.code, 0, "{tag}: {}", r.err);
        // Decided, so no `note: … partial` on stderr.
        assert!(r.err.is_empty(), "{tag}: {}", r.err);
        for (k, v) in rows {
            assert_eq!(line(&r.out, k), v, "{tag}:\n{}", r.out);
        }
        assert_eq!(
            line(&r.out, "work"),
            "1 saturation · 0 facts beyond root",
            "{tag}"
        );
        assert_eq!(r.out.matches(";; state ").count(), 1, "{tag}:\n{}", r.out);
        assert!(
            r.out.contains(";; state 1 of 1 — ∅, root itself · minimal"),
            "{tag}:\n{}",
            r.out
        );
    }
}

/// **The minimal-line row says no clause beside ∅** — M1f S1f.5.6, the user's
/// ruling of 2026-11-07 on S-17 and F1, *"a, drop the clause"*. Until then
/// describe's row read `1 state — ∅, the trivial line: no obligation is
/// stated` wherever no node the run met owed, whatever the program states:
/// on the `tests/stdlib/` programs that state an obligation their root
/// discharges, on the F1 probe and on its control alike, and on
/// `past-limit-lines` at `--region-limit 1` but not at `2`. Where ∅ is in R
/// the row reads `1 state — ∅` on every program and at every limit, the `∅`
/// row above it saying why, and `region.minimal.trivial` reads *∅ is in R*.
#[test]
fn the_minimal_line_row_names_empty_and_no_clause() {
    let dir = scratch("minimal-line-row");
    let mut cases: Vec<(String, Vec<&str>)> = [
        "tests/stdlib/algebra/24_total_owed_satisfied.ein",
        "tests/stdlib/algebra/26_surjective_owed_satisfied.ein",
        "tests/stdlib/bijection/06_blind_enumeration.ein",
        "tests/stdlib/closure/02_closed_and_satisfied.ein",
        "tests/stdlib/slots/01_partition.ein",
        "tests/stdlib/slots/08_spatial_adjacent.ein",
        "tests/stdlib/slots/10_owed_room_satisfied.ein",
        "tests/stdlib/slots/12_owed_fill_satisfied.ein",
        "examples/ein-bugs/minimal-line-stated-obligation.ein",
        "examples/ein-bugs/minimal-line-stated-obligation-control.ein",
    ]
    .into_iter()
    .map(|f| (f.to_string(), Vec::new()))
    .collect();
    cases.push((PAST.to_string(), vec!["--region-limit", "1"]));
    cases.push((PAST.to_string(), vec!["--region-limit", "2"]));
    for (i, (file, extra)) in cases.iter().enumerate() {
        let json = dir.join(format!("s{i}.json"));
        let json = json.to_str().expect("utf-8");
        let mut argv = vec!["solve", file.as_str()];
        argv.extend(extra.iter().copied());
        argv.extend(["--json-summary", json]);
        let r = ein(&argv);
        let tag = format!("{file} {}", extra.join(" "));
        assert_eq!(r.code, 0, "{tag}: {}", r.err);
        assert_eq!(
            line(&r.out, "∅"),
            "in R — root is alive and owes nothing",
            "{tag}:\n{}",
            r.out
        );
        assert_eq!(
            line(&r.out, "minimal line"),
            "1 state — ∅",
            "{tag}:\n{}",
            r.out
        );
        let s: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(json).expect("a summary"))
                .expect("the summary parses");
        assert_eq!(s["region"]["root"], "in-r", "{tag}");
        assert_eq!(s["region"]["minimal"]["trivial"], true, "{tag}");
    }
    // Where ∅ is not in R the field is false: R empty, every alive node owing.
    let json = dir.join("owing.json");
    let json = json.to_str().expect("utf-8");
    let r = ein(&[
        "solve",
        "tests/stdlib/closure/03_closed_and_owing.ein",
        "--json-summary",
        json,
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(json).expect("a summary"))
            .expect("the summary parses");
    assert_ne!(s["region"]["root"], "in-r");
    assert_eq!(s["region"]["minimal"]["trivial"], false);
}
