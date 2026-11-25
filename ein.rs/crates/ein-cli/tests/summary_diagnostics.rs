//! **What a run says, as values** (Q-M2.9): the run summary's
//! `diagnostics`, a refused program's summary, the clock's, and the report's
//! rows in `error`.
//!
//! The summary is written wherever a run gets far enough to know where to
//! write it: a run that answers, and one with no answer, refused at the parse,
//! at the load or by the judgement of its hypothesis mechanisms, or ended by
//! the clock, whose answer keys are `null`. Its last key, `diagnostics`, is
//! what the run printed on stderr, in order, each `{kind, severity, message,
//! place}` (`said::agree`). What is held here:
//!
//! | claim | test |
//! |---|---|
//! | every refused fixture under `examples/broken/` writes a summary with no answer, whose findings are each of a kind `07-diagnostics.md` names, and whose messages hold its `.expected` | [`every_refused_fixture_says_its_findings`] |
//! | on every corpus program, root alone, the summary's diagnostics are what stderr printed (every declared `solve` run of the corpus is held to the same by `corpus_cli.rs`) | [`every_corpus_program_says_on_stderr_what_its_summary_lists`] |
//! | a `-T 0` run writes a summary with no answer whose one diagnostic is the deadline's, under `ein solve`, `ein oracle` and `ein test` | [`the_clock_writes_a_summary_whose_one_diagnostic_is_the_deadlines`] |
//! | a row of `ein test --json-report` in `error` says why, and no other row carries diagnostics | [`a_report_row_in_error_says_why`] |
//! | the page names a diagnostic's four fields, as the summary writes them | [`the_page_names_a_diagnostics_fields`] |

mod said;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, corpus_files, repo_root};
use serde_json::Value as J;

/// The keys of an answer, `null` in a summary with no answer.
const ANSWER_KEYS: [&str; 8] = [
    "region",
    "verdict",
    "queries",
    "claim",
    "work",
    "monotonicity",
    "leftover",
    "root",
];

struct Run {
    code: i32,
    out: String,
    err: String,
    summary: Option<J>,
}

/// `ein ARGS --json-summary DIR/summary.json`, from the repository's root.
fn ein(args: &[&str], dir: &Path) -> Run {
    let json = dir.join("summary.json");
    let _ = std::fs::remove_file(&json);
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .arg("--json-summary")
        .arg(&json)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary: std::fs::read_to_string(&json)
            .ok()
            .map(|t| serde_json::from_str(&t).expect("the summary parses")),
    }
}

/// The kinds `07-diagnostics.md` writes beside its sections: the refusals'.
fn refusal_kinds() -> BTreeSet<String> {
    let page =
        std::fs::read_to_string(repo_root().join("docs/ein/user/04-reference/07-diagnostics.md"))
            .expect("07-diagnostics.md");
    let mut out = BTreeSet::new();
    for line in page.lines() {
        let Some(rest) = line.strip_prefix("**Kind:**") else {
            continue;
        };
        for (i, part) in rest.split('`').enumerate() {
            if i % 2 == 1 && ein_core::Kind::of_name(part).is_some() {
                out.insert(part.to_string());
            }
        }
    }
    assert!(out.len() > 20, "only {} kinds on the page", out.len());
    out
}

/// Whether `s` is a summary with no answer: every answer key `null`.
fn unanswered(s: &J) -> bool {
    ANSWER_KEYS.iter().all(|k| s[k].is_null())
}

/// **Every refused fixture says its findings.** Each `.ein` under
/// `examples/broken/` that a run refuses exits 1 and writes a summary with no
/// answer, `asked` the question and `config` `null`, whose diagnostics are its
/// findings, at least one, each an `error` of a kind `07-diagnostics.md`
/// names, and their messages, joined as the refusal joins them, hold the line
/// its `.expected` banks; and they are what stderr printed.
#[test]
fn every_refused_fixture_says_its_findings() {
    let root = repo_root();
    let kinds = refusal_kinds();
    let mut files: Vec<PathBuf> = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .expect("a directory")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().and_then(|x| x.to_str()) == Some("ein") {
                out.push(p);
            }
        }
    }
    walk(&root.join("examples/broken"), &mut files);
    let stdlib = root.join("stdlib");
    let tmp = Scratch::new("ein-summary-refused");
    let found: Vec<Result<bool, String>> = ein_corpus::pool::map(
        "summary_diagnostics_refused",
        &files.iter().enumerate().collect::<Vec<_>>(),
        |(_, p)| (p.display().to_string(), "solve".to_string()),
        |(i, path)| {
            let dir = tmp.join(format!("{i:03}"));
            std::fs::create_dir_all(&dir).expect("a scratch directory");
            let file = path.to_str().expect("utf-8");
            let r = ein(&["solve", file], &dir);
            let at = path
                .strip_prefix(&root)
                .unwrap_or(path)
                .display()
                .to_string();
            if r.code == 0 {
                return Ok(false);
            }
            let s = r
                .summary
                .ok_or_else(|| format!("{at}: refused, exit {}, and no summary", r.code))?;
            if r.code != 1 || !r.out.is_empty() || !unanswered(&s) || !s["config"].is_null() {
                return Err(format!(
                    "{at}: exit {}, stdout {:?}, a summary with an answer or a config:\n{s}",
                    r.code, r.out
                ));
            }
            if s["asked"]["ask"] != "describe" {
                return Err(format!("{at}: asked {}", s["asked"]));
            }
            said::agree(&s, &r.err).map_err(|e| format!("{at}: {e}"))?;
            let ds: Vec<ein_core::Diagnostic> = s["diagnostics"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|v| said::diagnostic(v).map_err(|e| format!("{at}: {e}")))
                .collect::<Result<_, _>>()?;
            if ds.is_empty() {
                return Err(format!("{at}: refused with no finding"));
            }
            for d in &ds {
                if d.severity != ein_core::Severity::Error || !kinds.contains(d.kind.name()) {
                    return Err(format!(
                        "{at}: a finding of {} {}, which is no refusal the page names",
                        d.severity.name(),
                        d.kind.name()
                    ));
                }
            }
            let expected = path.with_extension("expected");
            let want = std::fs::read_to_string(&expected)
                .map_err(|e| format!("{at}: {e}"))?
                .trim()
                .replace("{FILE}", file)
                .replace(
                    "{DIR}",
                    path.parent().and_then(Path::to_str).expect("utf-8"),
                )
                .replace("{STDLIB}", stdlib.to_str().expect("utf-8"));
            let messages: Vec<&str> = ds.iter().map(|d| d.message.as_str()).collect();
            if !messages.join("; ").contains(&want) && !messages.join("\n").contains(&want) {
                return Err(format!(
                    "{at}: the findings do not hold the .expected line\n  want {want:?}\n  got  {messages:?}"
                ));
            }
            Ok(true)
        },
    );
    let mut refused = 0usize;
    let mut bad = Vec::new();
    for f in found {
        match f {
            Ok(true) => refused += 1,
            Ok(false) => {}
            Err(e) => bad.push(e),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(refused >= 60, "only {refused} fixtures were refused");
}

/// **On every corpus program, the summary lists what stderr printed.** Root
/// alone, `--no-search`, the cheapest question every program takes: a
/// refusal's findings, the static check's warning, the claims root does not
/// decide, the load's notes, each a value of the one shape, in stderr's
/// order, and nothing on stderr that no diagnostic carries. Every `solve`
/// run the corpus declares is held to the same by `corpus_cli.rs`.
#[test]
fn every_corpus_program_says_on_stderr_what_its_summary_lists() {
    let files = corpus_files();
    let root = repo_root();
    let tmp = Scratch::new("ein-summary-corpus");
    let found: Vec<Result<(), String>> = ein_corpus::pool::map(
        "summary_diagnostics_corpus",
        &files.iter().enumerate().collect::<Vec<_>>(),
        |(_, p)| {
            (
                p.strip_prefix(&root).unwrap_or(p).display().to_string(),
                "solve --no-search".to_string(),
            )
        },
        |(i, path)| {
            let dir = tmp.join(format!("{i:04}"));
            std::fs::create_dir_all(&dir).expect("a scratch directory");
            let at = path
                .strip_prefix(&root)
                .unwrap_or(path)
                .display()
                .to_string();
            let r = ein(&["solve", &at, "--no-search"], &dir);
            let _ = std::fs::remove_dir_all(&dir);
            let s = r
                .summary
                .ok_or_else(|| format!("{at}: exit {}, no summary\n{}", r.code, r.err))?;
            said::agree(&s, &r.err).map_err(|e| format!("{at}: {e}"))
        },
    );
    let bad: Vec<String> = found.into_iter().filter_map(Result::err).collect();
    assert!(
        bad.is_empty(),
        "{} programs:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **The clock writes a summary whose one diagnostic is the deadline's**:
/// `-T 0` ends the run at its first read of the clock, exit 2, nothing on
/// stdout, the abort's line on stderr, and a summary with no answer, its
/// `asked` the question, its `config` the program's, and one diagnostic,
/// the deadline's, as stderr printed it. `ein oracle` says its own line, and
/// `ein test` the line `ein solve` says, on stderr, beside the file's `ERROR`
/// row on stdout, and lists it in the file's summary as stderr printed it.
#[test]
fn the_clock_writes_a_summary_whose_one_diagnostic_is_the_deadlines() {
    let file = "examples/lattice/02_genuine_3set_death.ein";
    let tmp = Scratch::new("ein-summary-deadline");
    let deadline = |message: &str| {
        serde_json::json!([{
            "kind": "deadline",
            "severity": "error",
            "message": message,
            "place": null,
        }])
    };
    let decided = "--max-time 0 s reached before the question was decided";
    for (question, ask) in [
        (&[][..], "describe"),
        (&["--single"][..], "single"),
        (&["--count"][..], "count"),
        (&["--no-search"][..], "no-search"),
    ] {
        let mut args = vec!["solve", file, "-T", "0"];
        args.extend(question);
        let r = ein(&args, tmp.path());
        let at = args.join(" ");
        assert_eq!(r.code, 2, "{at}: {}", r.err);
        assert!(r.out.is_empty(), "{at}: {}", r.out);
        assert_eq!(r.err, format!("** aborted: {decided} **\n"), "{at}");
        let s = r.summary.unwrap_or_else(|| panic!("{at}: no summary"));
        assert!(unanswered(&s), "{at}: {s}");
        assert_eq!(s["asked"]["ask"], ask, "{at}");
        assert!(s["config"].is_object(), "{at}: {}", s["config"]);
        assert_eq!(s["diagnostics"], deadline(decided), "{at}");
        said::agree(&s, &r.err).unwrap_or_else(|e| panic!("{at}: {e}"));
    }
    let r = ein(&["oracle", file, "-T", "0"], tmp.path());
    let done = "--max-time 0 s reached before the cube was done";
    assert_eq!(r.code, 2, "oracle: {}", r.err);
    let s = r.summary.expect("oracle -T 0: a summary");
    assert!(unanswered(&s), "oracle: {s}");
    assert_eq!(s["diagnostics"], deadline(done), "oracle");
    said::agree(&s, &r.err).unwrap_or_else(|e| panic!("oracle: {e}"));
    // `ein test`: the file states a claim, so a question is asked of it.
    let claimed = "examples/lattice/18_two_candidates.ein";
    let report = tmp.join("report.json");
    let r = ein(
        &[
            "test",
            claimed,
            "-T",
            "0",
            "--json-report",
            report.to_str().expect("utf-8"),
        ],
        tmp.path(),
    );
    assert_eq!(r.code, 2, "test: {}{}", r.out, r.err);
    assert!(r.out.starts_with("ERROR "), "test: {}", r.out);
    assert_eq!(r.err, format!("** aborted: {decided} **\n"), "test");
    let s = r.summary.expect("test -T 0: a summary");
    assert!(unanswered(&s), "test: {s}");
    assert_eq!(s["diagnostics"], deadline(decided), "test");
    said::agree(&s, &r.err).unwrap_or_else(|e| panic!("test: {e}"));
    let rows: J =
        serde_json::from_str(&std::fs::read_to_string(&report).expect("the report")).expect("JSON");
    let rows = rows["rows"].as_array().expect("rows");
    assert!(
        rows.iter()
            .filter(|r| !r["expect"].is_null())
            .all(|r| r["outcome"] == "error" && r["diagnostics"] == deadline(decided)),
        "test: every row of a claim is an error that says the deadline: {rows:?}"
    );
}

/// **A report row in `error` says why**: a file the run refused has one row,
/// `error`, whose diagnostics are its findings, as `ein solve`'s summary
/// lists them; every other row's diagnostics are `null`.
#[test]
fn a_report_row_in_error_says_why() {
    let tmp = Scratch::new("ein-report-diagnostics");
    let report = tmp.join("report.json");
    let refused = "examples/broken/unclosed_paren.ein";
    let held = "examples/lattice/18_two_candidates.ein";
    let r = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args([
            "test",
            refused,
            held,
            "--json-report",
            report.to_str().expect("utf-8"),
        ])
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    assert_eq!(r.status.code(), Some(2));
    let doc: J =
        serde_json::from_str(&std::fs::read_to_string(&report).expect("the report")).expect("JSON");
    let solved = ein(&["solve", refused], tmp.path());
    let want = solved.summary.expect("the refusal's summary")["diagnostics"].clone();
    let rows = doc["rows"].as_array().expect("rows");
    let mut errors = 0;
    for row in rows {
        if row["outcome"] == "error" {
            errors += 1;
            assert_eq!(row["path"], refused, "{row}");
            assert_eq!(row["diagnostics"], want, "{row}");
            assert_eq!(row["diagnostics"][0]["kind"], "parse", "{row}");
        } else {
            assert!(row["diagnostics"].is_null(), "{row}");
        }
    }
    assert_eq!(errors, 1, "{rows:?}");
}

/// **The page names a diagnostic's fields**: `08-read-out.md`'s table of
/// the `diagnostics` key lists the keys a diagnostic of the summary has, and
/// a place's.
#[test]
fn the_page_names_a_diagnostics_fields() {
    let page =
        std::fs::read_to_string(repo_root().join("docs/ein/user/04-reference/08-read-out.md"))
            .expect("08-read-out.md");
    let block = page
        .split_once("<!-- diagnostics -->")
        .and_then(|(_, rest)| rest.split_once("<!-- /diagnostics -->"))
        .map(|(b, _)| b)
        .expect("a <!-- diagnostics --> table");
    let named: Vec<String> = block
        .lines()
        .filter(|l| l.starts_with("| `"))
        .filter_map(|l| l.split('`').nth(1).map(str::to_string))
        .collect();
    let tmp = Scratch::new("ein-diagnostic-fields");
    let r = ein(&["solve", "examples/broken/unclosed_paren.ein"], tmp.path());
    let s = r.summary.expect("a summary");
    let keys: Vec<String> = s["diagnostics"][0]
        .as_object()
        .expect("a diagnostic")
        .keys()
        .cloned()
        .collect();
    let place: Vec<String> = s["diagnostics"][0]["place"]
        .as_object()
        .expect("a place")
        .keys()
        .map(|k| format!("place.{k}"))
        .collect();
    // As sets: a JSON object's keys come back sorted.
    let mut want = keys;
    want.extend(place);
    want.sort();
    let mut named = named;
    named.sort();
    assert_eq!(named, want, "the page's table against a diagnostic's keys");
}
