//! **A saturation is bounded** (Q-M2.16): the clock, `-T`, and the cap,
//! `--max-steps`, reach inside every saturation a question about R computes,
//! root's first, so a program with no fixpoint ends.
//!
//! The witness is `tests/bounded/grow.ein`, one rule that nests its
//! conclusion one deeper than its premise: root's saturation has no fixpoint.
//! `tests/bounded/grow_node.ein` starts the same rule from a hypothesis, so
//! root reaches its fixpoint and the node of `(seed zero)` does not. Neither
//! is a corpus program: every corpus sweep saturates every corpus file, and
//! most of them through a reader no question's bound reaches (root's
//! saturation for the shape golden, `ein saturate`, `ein render kb`, the
//! oracle's cube), where a saturation has no bound of its own.
//!
//! The clock reaches a dead node's core search too, which is no saturation:
//! `tests/bounded/dead_link.ein`'s root dies on a link several rules derive,
//! and its core's search over every recorded derivation runs far past any
//! clock the tests set, where a question prints the core; a question that
//! prints none takes none, and answers in what root's saturation costs. It is
//! no corpus program either: a description prints its core, which no sweep
//! waits for. Two more fixtures put the same link where a core other than
//! root's is taken: `tests/bounded/dead_link_hole.ein`'s minimal hole on the
//! cube, which a description prints and a line mode does not, and
//! `tests/bounded/dead_link_whole.ein`'s whole node, which the obligation
//! tree reads beside its cache, and which `--events` and `--dump-lattice`
//! print and `-v` and `--layer-progress` do not.
//!
//! What each stop says is the pages' (`05-cli.md` § 3, `08-read-out.md`):
//!
//! - **the clock** ends the run as a `-T` abort always did: one line on
//!   stderr, nothing on stdout, exit 2, and a summary with no answer whose
//!   one diagnostic is the deadline's (Q-M2.9); `ein test` counts the file
//!   an `ERROR`, exit 2; the library's error is `SolveError::Deadline`;
//! - **the cap** ends it *not decided*, as a budget does: exit 0, the
//!   `exhausted` row or `--single`'s `verdict` and `decided` saying which
//!   saturation passed it, a `note: … partial — …` line, and the summary's
//!   `region.exhausted` false, `region.stopped` `{why: steps, steps, saturation}`
//!   and `verdict.word` `null`. Nothing of R is established, so the summary's
//!   `monotonicity` and `root` are `null`, no query is answered and no claim
//!   is checked, and its `diagnostics` are the `partial` note stderr printed;
//!   `ein test` counts the claim `NOT CHECKED`, exit 1.
//!
//! That a cap or a clock a run does not reach moves nothing is held over the
//! corpus by `lever_invariance.rs`' `no_config_flag_moves_a_line_of_any_mode`,
//! which asks every question again with no cap and a far clock, beside the
//! flags; here, through the binary, on the corpus's largest saturation.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use ein_core::Terms;
use ein_corpus::{Scratch, repo_root};
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_ir::Ast;
use serde_json::Value as J;

mod said;

/// The fixtures, beside this file.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/bounded")
        .join(name)
}

/// The cap the tests ask: far below the default, so a run ends in
/// milliseconds.
const CAP: &str = "1000";

/// A clock run must end well within this, whatever the machine: a stop at
/// the clock is a fraction of a second and a thousand-odd firings past it.
const PATIENCE: Duration = Duration::from_secs(30);

struct Run {
    code: i32,
    out: String,
    err: String,
    summary: Option<J>,
    wall: Duration,
}

fn ein(args: &[&str]) -> Run {
    let dir = Scratch::new("ein-bounded-saturation");
    let json = dir.join("summary.json");
    let mut argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    if matches!(args.first(), Some(&"solve" | &"oracle")) {
        argv.push("--json-summary".into());
        argv.push(json.to_string_lossy().into());
    }
    let t = Instant::now();
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(&argv)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let wall = t.elapsed();
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary: std::fs::read_to_string(&json)
            .ok()
            .map(|t| serde_json::from_str(&t).expect("the summary parses")),
        wall,
    }
}

/// The value of the read-out row `label`, at the top level.
fn row<'a>(out: &'a str, label: &str) -> Option<&'a str> {
    out.lines().find_map(|l| {
        let rest = l.strip_prefix("  ")?.strip_prefix(label)?;
        rest.starts_with(' ').then(|| rest.trim())
    })
}

const ABORT: &str = "** aborted: --max-time 0.3 s reached before the question was decided **\n";

/// The abort's message, as its diagnostic holds it.
const DECIDED: &str = "--max-time 0.3 s reached before the question was decided";

/// Whether `r` wrote a summary with no answer, its one diagnostic the
/// deadline's, `message`, as stderr printed it.
fn aborted(r: &Run, message: &str) -> bool {
    r.summary.as_ref().is_some_and(|s| {
        [
            "region",
            "verdict",
            "queries",
            "claim",
            "work",
            "monotonicity",
            "leftover",
            "root",
        ]
        .iter()
        .all(|k| s[k].is_null())
            && s["diagnostics"]
                == serde_json::json!([{
                    "kind": "deadline",
                    "severity": "error",
                    "message": message,
                    "place": null,
                }])
            && said::agree(s, &r.err).is_ok()
    })
}

/// A cap no clock run reaches before its clock, whatever the machine: the
/// default cap is reached in about a second on a fast one, so a clock run
/// lifts it, and the clock alone ends the run.
const NO_CAP: &str = "1000000000000";

/// **The clock ends a saturation with no fixpoint, root's and a node's**, on
/// every question, as `-T` ends a run: one line on stderr, nothing on stdout,
/// exit 2, and a summary with no answer whose one diagnostic is the
/// deadline's. `ein oracle` ends the same way, with its own line.
#[test]
fn the_clock_ends_a_saturation_with_no_fixpoint() {
    let grow = fixture("grow.ein");
    let node = fixture("grow_node.ein");
    let mut failures = Vec::new();
    for (file, question) in [
        (&grow, &[][..]),
        (&grow, &["--single"][..]),
        (&grow, &["--no-search"][..]),
        (&grow, &["--count"][..]),
        (&node, &[][..]),
        (&node, &["--single"][..]),
    ] {
        let mut args = vec![
            "solve",
            file.to_str().expect("a path"),
            "-T",
            "0.3",
            "--max-steps",
            NO_CAP,
        ];
        args.extend(question);
        let r = ein(&args);
        let at = args.join(" ");
        if r.code != 2 || r.err != ABORT || !r.out.is_empty() || !aborted(&r, DECIDED) {
            failures.push(format!(
                "{at}: exit {}, stdout {:?}, stderr {:?}, summary {:?}",
                r.code, r.out, r.err, r.summary
            ));
        }
        if r.wall > PATIENCE {
            failures.push(format!("{at}: ended after {:?}", r.wall));
        }
    }
    let r = ein(&["oracle", grow.to_str().expect("a path"), "-T", "0.3"]);
    if r.code != 2
        || r.err != "** aborted: --max-time 0.3 s reached before the cube was done **\n"
        || r.wall > PATIENCE
        || !aborted(&r, "--max-time 0.3 s reached before the cube was done")
    {
        failures.push(format!(
            "oracle -T 0.3: exit {}, stderr {:?}, after {:?}, summary {:?}",
            r.code, r.err, r.wall, r.summary
        ));
    }
    // `ein test` asks a file that states a claim: the node's.
    let r = ein(&[
        "test",
        node.to_str().expect("a path"),
        "-T",
        "0.3",
        "--max-steps",
        NO_CAP,
    ]);
    if r.code != 2 || !r.out.starts_with("ERROR ") || r.wall > PATIENCE {
        failures.push(format!(
            "test -T 0.3: exit {}, stdout {:?}, after {:?}",
            r.code, r.out, r.wall
        ));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **The clock ends a dead root's core search where the question prints the
/// core**, the one computation a dead node's record needs that its saturation
/// does not: `dead_link.ein`'s root saturates in some twenty-five thousand
/// firings and dies, and the search for its core folds millions of
/// environments over every derivation of a link several rules derive. A
/// description and `--no-search` print root's core, and so does every surface
/// that prints a visited node's (`--events`, `--dump-lattice`), under any
/// question; each of them, `ein oracle`, and `ein test` writing the file's
/// summary end at the clock as `-T` ends a run anywhere; the library's error
/// is [`ein_infer::SolveError::Deadline`]. The same program with
/// `(config :record-alternative-justifications false)`, the search offered
/// one derivation a fact, answers with no clock, its root dead and its claim
/// held: what the clock stopped was the search, and the program's answer is
/// no other.
#[test]
fn the_clock_ends_a_dead_roots_core_search() {
    let file = fixture("dead_link.ein");
    let path = file.to_str().expect("a path");
    let dir = Scratch::new("ein-bounded-dead-link");
    let events = dir.join("events.jsonl");
    let events = events.to_str().expect("a path");
    let lattice = dir.join("lattice");
    let lattice = lattice.to_str().expect("a path");
    let mut failures = Vec::new();
    for question in [
        &[][..],
        &["--no-search"][..],
        &["--single", "--events", events][..],
        &["--count", "--dump-lattice", lattice][..],
    ] {
        let mut args = vec!["solve", path, "-T", "0.3"];
        args.extend(question);
        let r = ein(&args);
        let at = args.join(" ");
        if r.code != 2 || r.err != ABORT || !r.out.is_empty() || !aborted(&r, DECIDED) {
            failures.push(format!(
                "{at}: exit {}, stdout {:?}, stderr {:?}, summary {:?}",
                r.code, r.out, r.err, r.summary
            ));
        }
        if r.wall > PATIENCE {
            failures.push(format!("{at}: ended after {:?}", r.wall));
        }
    }
    let r = ein(&["oracle", path, "-T", "0.3"]);
    if r.code != 2
        || r.err != "** aborted: --max-time 0.3 s reached before the cube was done **\n"
        || r.wall > PATIENCE
    {
        failures.push(format!(
            "oracle -T 0.3: exit {}, stderr {:?}, after {:?}",
            r.code, r.err, r.wall
        ));
    }
    let summary = dir.join("test.json");
    let r = ein(&[
        "test",
        path,
        "-T",
        "0.3",
        "--json-summary",
        summary.to_str().expect("a path"),
    ]);
    if r.code != 2 || !r.out.starts_with("ERROR ") || r.wall > PATIENCE {
        failures.push(format!(
            "test -T 0.3 --json-summary: exit {}, stdout {:?}, after {:?}",
            r.code, r.out, r.wall
        ));
    }
    let t = Instant::now();
    match run(
        &file,
        Mode {
            which: Which::All,
            ask: Ask::Describe,
        },
        |o| {
            o.deadline = Some(Instant::now() + Duration::from_millis(200));
        },
    ) {
        Err(ein_infer::SolveError::Deadline) if t.elapsed() < PATIENCE => {}
        other => failures.push(format!(
            "the library, describe: {:?} after {:?}",
            other.map(|a| a.verdict),
            t.elapsed()
        )),
    }
    // The program with the lever, which no clock stops.
    let lever = lever_twin(&dir, &file);
    let lever = lever.to_str().expect("a path");
    let r = ein(&["solve", lever, "--single"]);
    if r.code != 0 || row(&r.out, "verdict").is_none_or(|v| !v.starts_with("Contradiction")) {
        failures.push(format!(
            "with the lever, --single: exit {}, stdout {:?}, stderr {:?}",
            r.code, r.out, r.err
        ));
    }
    let r = ein(&["test", lever]);
    if r.code != 0 || !r.out.starts_with("ok ") {
        failures.push(format!(
            "with the lever, test: exit {}, stdout {:?}, stderr {:?}",
            r.code, r.out, r.err
        ));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// `file` with `(config :record-alternative-justifications false)`, written
/// into `dir`: the search offered one derivation a fact.
fn lever_twin(dir: &Scratch, file: &Path) -> PathBuf {
    let lever = dir.join("dead_link_one_justification.ein");
    let text = std::fs::read_to_string(file).expect("the fixture reads");
    std::fs::write(
        &lever,
        format!("{text}(config :record-alternative-justifications false)\n"),
    )
    .expect("the copy writes");
    lever
}

/// **A question that prints no core takes no dead root's core, and answers
/// within the clock** (Q-M1f.29, Q-M1e2.31): a mode prints a part of `R` and
/// `--count` how many, and neither prints root's core, so neither searches
/// for it. On `dead_link.ein`, whose root's core search runs far past the
/// clock, `--single`, `--count`, `-n 2`, `--minimal` and `--maximal` under
/// `-T 0.3` each answer, exit 0, in what root's saturation costs, the
/// summary's `region.core` `null`, and their `region` and `verdict` the lever's
/// twin's, which takes the search's one derivation a fact; so does `ein test`
/// with no summary, whose claim reads no core; and the library's `--single`
/// and `--count` under a deadline of 200 ms.
#[test]
fn a_question_that_prints_no_core_answers_a_dead_root_within_the_clock() {
    let file = fixture("dead_link.ein");
    let path = file.to_str().expect("a path");
    let dir = Scratch::new("ein-bounded-dead-link-modes");
    let lever = lever_twin(&dir, &file);
    let lever = lever.to_str().expect("a path");
    let mut failures = Vec::new();
    for question in [
        &["--single"][..],
        &["--count"][..],
        &["-n", "2"][..],
        &["--minimal"][..],
        &["--maximal"][..],
    ] {
        let mut args = vec!["solve", path, "-T", "0.3"];
        args.extend(question);
        let at = args.join(" ");
        let r = ein(&args);
        let mut twin = vec!["solve", lever];
        twin.extend(question);
        let t = ein(&twin);
        let blocks = |r: &Run| {
            r.summary
                .as_ref()
                .map(|s| (s["region"].clone(), s["verdict"].clone()))
        };
        if r.code != 0 || t.code != 0 || r.wall > PATIENCE {
            failures.push(format!(
                "{at}: exit {} after {:?}, the twin's exit {}; stderr {:?}",
                r.code, r.wall, t.code, r.err
            ));
            continue;
        }
        match (blocks(&r), blocks(&t)) {
            (Some(a), Some(b)) if a == b && a.0["core"].is_null() => {}
            (a, b) => failures.push(format!("{at}: region and verdict {a:?}, the twin's {b:?}")),
        }
    }
    let r = ein(&["test", path, "-T", "0.3"]);
    if r.code != 0 || !r.out.starts_with("ok ") || r.wall > PATIENCE {
        failures.push(format!(
            "test -T 0.3: exit {}, stdout {:?}, after {:?}",
            r.code, r.out, r.wall
        ));
    }
    for ask in [Ask::Single, Ask::Count] {
        let mode = Mode {
            which: Which::All,
            ask,
        };
        let t = Instant::now();
        match run(&file, mode, |o| {
            o.deadline = Some(Instant::now() + Duration::from_millis(200));
        }) {
            Ok(a) if a.root_dead && a.exhausted && t.elapsed() < PATIENCE => {}
            other => failures.push(format!(
                "the library, {mode:?}: {:?} after {:?}",
                other.map(|a| (a.verdict, a.exhausted)),
                t.elapsed()
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The `region` and `verdict` blocks of a run's summary: its answer, which
/// no clock it is within and no watch moves.
fn answer_blocks(r: &Run) -> Option<(J, J)> {
    r.summary
        .as_ref()
        .map(|s| (s["region"].clone(), s["verdict"].clone()))
}

/// **A line mode takes no minimal hole's core, and a watch no whole node's**
/// (Q-M1e2.31, Q-M1f.29): a core is taken only where a surface prints one.
///
/// `dead_link_hole.ein` is a flagged program within the limit whose one
/// minimal hole dies on a link several rules derive. A line mode prints no
/// hole, so `--maximal` and `--minimal`, which read the whole cube, and
/// `--maximal` under `-n 2` and `--single`, answer under `-T 2`, exit 0, their
/// `region` and `verdict` the lever's twin's; a description and `--count`,
/// whose read-out and summary print the hole's core, and `--maximal` with
/// `--dump-lattice`, which prints every dead node's, end at the clock.
///
/// `dead_link_whole.ein` is a monotone program whose whole node, which the
/// obligation tree reads under `--single` and `-n 2`, dies on such a link.
/// `-v` and `--layer-progress` print a node's commitment and kind and no
/// core, so under `-T 5` each answers, exit 0, its read-out and its `region`
/// and `verdict` those of the run with no watch.
#[test]
fn a_line_mode_takes_no_holes_core_and_a_watch_no_whole_nodes() {
    let mut failures = Vec::new();
    let hole = fixture("dead_link_hole.ein");
    let path = hole.to_str().expect("a path");
    let dir = Scratch::new("ein-bounded-dead-link-hole");
    let lever = lever_twin(&dir, &hole);
    let lever = lever.to_str().expect("a path");
    for question in [
        &["--maximal"][..],
        &["--minimal"][..],
        &["--maximal", "-n", "2"][..],
        &["--maximal", "--single"][..],
    ] {
        let mut args = vec!["solve", path, "-T", "2"];
        args.extend(question);
        let at = args.join(" ");
        let r = ein(&args);
        let mut twin = vec!["solve", lever];
        twin.extend(question);
        let t = ein(&twin);
        if r.code != 0 || t.code != 0 || r.wall > PATIENCE {
            failures.push(format!(
                "{at}: exit {} after {:?}, the twin's exit {}; stderr {:?}",
                r.code, r.wall, t.code, r.err
            ));
            continue;
        }
        match (answer_blocks(&r), answer_blocks(&t)) {
            (Some(a), Some(b)) if a == b && a.0["holes"].is_null() => {}
            (a, b) => failures.push(format!("{at}: region and verdict {a:?}, the twin's {b:?}")),
        }
    }
    let lattice = dir.join("lattice");
    let lattice = lattice.to_str().expect("a path");
    for question in [
        &[][..],
        &["--count"][..],
        &["--maximal", "--dump-lattice", lattice][..],
    ] {
        let mut args = vec!["solve", path, "-T", "0.3"];
        args.extend(question);
        let r = ein(&args);
        let at = args.join(" ");
        if r.code != 2 || r.err != ABORT || !r.out.is_empty() || !aborted(&r, DECIDED) {
            failures.push(format!(
                "{at}: exit {}, stdout {:?}, stderr {:?}, summary {:?}",
                r.code, r.out, r.err, r.summary
            ));
        }
        if r.wall > PATIENCE {
            failures.push(format!("{at}: ended after {:?}", r.wall));
        }
    }
    let whole = fixture("dead_link_whole.ein");
    let path = whole.to_str().expect("a path");
    for (question, watch) in [
        (&["--single"][..], "-v"),
        (&["--single"][..], "--layer-progress"),
        (&["-n", "2"][..], "-v"),
    ] {
        let mut plain = vec!["solve", path, "-T", "5"];
        plain.extend(question);
        let mut args = plain.clone();
        args.push(watch);
        let at = args.join(" ");
        let r = ein(&args);
        let p = ein(&plain);
        // Past the header, which echoes the command line.
        let body = |r: &Run| r.out.lines().skip(1).collect::<Vec<_>>().join("\n");
        if r.code != 0 || p.code != 0 || r.wall > PATIENCE {
            failures.push(format!(
                "{at}: exit {} after {:?}, with no watch exit {}; stderr {:?}",
                r.code,
                r.wall,
                p.code,
                r.err.lines().last()
            ));
        } else if body(&r) != body(&p) || answer_blocks(&r) != answer_blocks(&p) {
            failures.push(format!("{at}: not the answer of the run with no watch"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **The cap ends a saturation with no fixpoint**, root's and a node's, on
/// every question, *not decided*: exit 0, the rows that say which saturation
/// passed it, a `note: … partial` line, and a summary that establishes
/// nothing of R.
#[test]
fn the_cap_ends_a_saturation_with_no_fixpoint() {
    let mut failures = Vec::new();
    for (name, which) in [("grow.ein", "root"), ("grow_node.ein", "node")] {
        let file = fixture(name);
        let what = format!(
            "{} saturation needed more than --max-steps {CAP} firings, and was stopped before \
             its fixpoint",
            if which == "root" {
                "root's"
            } else {
                "a node's"
            }
        );
        for question in [
            &[][..],
            &["--single"][..],
            &["-n", "2"][..],
            &["--maximal"][..],
            &["--count"][..],
        ] {
            let mut args = vec!["solve", file.to_str().expect("a path"), "--max-steps", CAP];
            args.extend(question);
            let at = format!("{name} {}", question.join(" "));
            let r = ein(&args);
            if r.code != 0 {
                failures.push(format!("{at}: exit {}\n{}", r.code, r.err));
                continue;
            }
            let single = question == ["--single"];
            let rows_say = if single {
                row(&r.out, "verdict") == Some("not decided — the run stopped first")
                    && row(&r.out, "decided")
                        == Some(format!("no — {what}: nothing of R is established").as_str())
            } else {
                row(&r.out, "exhausted")
                    == Some(format!("no — {what}: nothing of R is established").as_str())
            };
            if !rows_say {
                failures.push(format!("{at}: the rows do not say the stop:\n{}", r.out));
            }
            if !r
                .err
                .lines()
                .any(|l| l.starts_with("note: ") && l.contains(&format!("partial — {what}")))
            {
                failures.push(format!("{at}: no partial note:\n{}", r.err));
            }
            let Some(s) = r.summary else {
                failures.push(format!("{at}: no summary"));
                continue;
            };
            let stopped = serde_json::json!({
                "why": "steps",
                "steps": CAP.parse::<u64>().expect("a number"),
                "saturation": which,
            });
            if s["region"]["exhausted"] != J::Bool(false)
                || s["region"]["stopped"] != stopped
                || s["region"]["root"] != J::Null
                || s["region"]["universe"] != J::Null
                || s["region"]["route"] != J::Null
                || s["monotonicity"] != J::Null
                || s["root"] != J::Null
            {
                failures.push(format!(
                    "{at}: the summary's region {} monotonicity {} root {}",
                    s["region"], s["monotonicity"], s["root"]
                ));
            }
            if single && s["verdict"]["word"] != J::Null {
                failures.push(format!("{at}: verdict.word {}", s["verdict"]["word"]));
            }
            // The lines beside the answer, as values: the partial note, and
            // the node's claim, not checked.
            let kinds: Vec<&str> = s["diagnostics"]
                .as_array()
                .map(|d| d.iter().filter_map(|x| x["kind"].as_str()).collect())
                .unwrap_or_default();
            let want: &[&str] = if which == "root" {
                &["partial"]
            } else {
                &["partial", "claim-not-checked"]
            };
            if kinds != want {
                failures.push(format!(
                    "{at}: the summary's diagnostics {}",
                    s["diagnostics"]
                ));
            }
            if let Err(e) = said::agree(&s, &r.err) {
                failures.push(format!("{at}: {e}"));
            }
        }
    }
    // `--no-search` on a root with no fixpoint: root is not known.
    let grow = fixture("grow.ein");
    let r = ein(&[
        "solve",
        grow.to_str().expect("a path"),
        "--max-steps",
        CAP,
        "--no-search",
    ]);
    if r.code != 0
        || !row(&r.out, "R").is_some_and(|v| v.starts_with("not searched, and root not known — "))
        || r.summary.as_ref().map(|s| &s["region"]["stopped"]["why"]) != Some(&J::from("steps"))
    {
        failures.push(format!(
            "--no-search: exit {}\n{}\n{}",
            r.code, r.out, r.err
        ));
    }
    // `ein test`: the node's claim is not checked, as past any budget.
    let node = fixture("grow_node.ein");
    let r = ein(&["test", node.to_str().expect("a path"), "--max-steps", CAP]);
    if r.code != 1 || !r.out.starts_with("NOT CHECKED ") || !r.out.contains("--max-steps 1000") {
        failures.push(format!("test: exit {}\n{}", r.code, r.out));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The library's run of `file` under `mode` and `opts`' bound.
fn run(
    file: &Path,
    mode: Mode,
    bound: impl FnOnce(&mut explore::Options),
) -> Result<explore::Answer, ein_infer::SolveError> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = ein_ir::load_file(&mut ast, &mut terms, file).expect("loads");
    let mut opts = explore::Options::new(mode);
    bound(&mut opts);
    explore::run(&mut kb, &mut terms, &ast, &opts)
}

/// **The library says each stop as the binary does**: the cap is an answer,
/// [`Stopped::Steps`], not decided, with nothing listed; the clock is
/// [`ein_infer::SolveError::Deadline`], inside root's saturation and inside a
/// node's; and [`explore::Options::new`] takes the default cap.
#[test]
fn the_library_says_each_stop() {
    let describe = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let single = Mode {
        which: Which::All,
        ask: Ask::Single,
    };
    assert_eq!(
        explore::Options::new(describe).max_steps,
        Some(explore::DEFAULT_MAX_STEPS)
    );
    for (name, root) in [("grow.ein", true), ("grow_node.ein", false)] {
        for mode in [describe, single] {
            let a = run(&fixture(name), mode, |o| o.max_steps = Some(1000))
                .unwrap_or_else(|e| panic!("{name}: {e}"));
            assert_eq!(
                a.stopped,
                Some(Stopped::Steps { cap: 1000, root }),
                "{name}"
            );
            assert!(!a.exhausted && a.verdict.is_none() && a.states.is_empty());
            assert!(a.candidates.is_empty() && a.described.is_none(), "{name}");
            let t = Instant::now();
            let e = run(&fixture(name), mode, |o| {
                o.deadline = Some(Instant::now() + Duration::from_millis(200));
                o.max_steps = None;
            })
            .expect_err("the clock ends it");
            assert!(matches!(e, ein_infer::SolveError::Deadline), "{name}: {e}");
            assert!(
                t.elapsed() < PATIENCE,
                "{name}: ended after {:?}",
                t.elapsed()
            );
        }
    }
}

/// **A program with a fixpoint is the same under a cap it does not reach**:
/// the corpus's largest saturation, `laws.ein`'s, and the puzzle, through
/// the binary, under a cap just above what they need and under the default,
/// byte for byte but the header that echoes the option.
#[test]
fn a_cap_a_run_does_not_reach_moves_nothing() {
    for (file, cap) in [
        ("examples/beyond_arith/laws.ein", "20000"),
        ("examples/zebra2.ein", "1000"),
    ] {
        for question in [&[][..], &["--single"][..], &["--maximal", "-n", "2"][..]] {
            let mut plain = vec!["solve", file];
            plain.extend(question);
            let mut capped = plain.clone();
            capped.extend(["--max-steps", cap]);
            let (a, b) = (ein(&plain), ein(&capped));
            let header = |t: &str| t.lines().skip(1).collect::<Vec<_>>().join("\n");
            assert_eq!(
                (a.code, header(&a.out), &a.err, &a.summary),
                (b.code, header(&b.out), &b.err, &b.summary),
                "{file} {question:?} under --max-steps {cap}"
            );
        }
    }
}

/// `--max-steps` takes one or more firings, as `--region-budget` takes one or
/// more facts: a usage error, exit 2, otherwise.
#[test]
fn the_cap_is_one_firing_or_more() {
    for bad in ["0", "-3", "many"] {
        let r = ein(&["solve", "examples/zebra2.ein", "--max-steps", bad]);
        assert_eq!(r.code, 2, "--max-steps {bad}: {}", r.err);
        let r = ein(&["test", "examples/zebra2.ein", "--max-steps", bad]);
        assert_eq!(r.code, 2, "test --max-steps {bad}: {}", r.err);
    }
}
