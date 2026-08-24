//! `ein test` — M1c
//! [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test)
//! T1c.1.3.5, **the tests for the tester**.
//!
//! A test runner that reports success on a broken expectation is the worst
//! possible outcome this phase has available to it, so every fixture below
//! that must *fail* is checked for failing — with the right exit code and the
//! right message — and not merely run. The
//! [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer)
//! precedent: the fuzzer's own three controls each failed once first.
//!
//! The comparison itself lives in `ein-infer`'s `expect_semantics`; what is
//! here is everything only a process can see — the exit code, the file
//! selection, the summary line, and the three ways a run can produce no
//! verdict at all.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("crates/<crate>/ is three below the root")
        .to_path_buf()
}

struct Run {
    code: i32,
    out: String,
    err: String,
}

impl Run {
    /// The last line of stdout is always the summary — the thing the stage
    /// calls "a summary line", and what a gate reads when it reads anything.
    fn summary(&self) -> &str {
        self.out.trim_end().lines().last().unwrap_or("")
    }
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

/// A scratch directory of `.ein` files the test owns and deletes.
struct Dir(PathBuf);

impl Dir {
    fn new(tag: &str) -> Dir {
        let dir = std::env::temp_dir().join(format!("ein-test-cli-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Dir(dir)
    }

    fn file(&self, name: &str, body: &str) -> String {
        let path = self.0.join(name);
        std::fs::write(&path, body).expect("writes");
        path.to_string_lossy().into_owned()
    }

    fn path(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const BODY: &str = "(relation p Thing Place)\n(p A H1)\n(p B H2)\n";
const TRUE_CLAIM: &str =
    "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H2)))\n";
const FALSE_CLAIM: &str =
    "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H3)))\n";

// ── The three corpus fixtures, which must hold ─────────────────────

/// One per verdict: `10_expect` is k = 1, `11_expect_ambiguity` is k = 2 and
/// `12_expect_false` is k = 0. `11` has no plain `solve` run in the corpus
/// because `-n 1` cannot check a k > 1 claim — under `test` it needs no flag,
/// which is the point of exhausting by default.
#[test]
fn the_three_feature_fixtures_hold() {
    for name in [
        "10_expect.ein",
        "11_expect_ambiguity.ein",
        "12_expect_false.ein",
    ] {
        let r = ein(&["test", &format!("examples/features/{name}")]);
        assert_eq!(r.code, 0, "{name}: {}{}", r.out, r.err);
        assert!(r.out.contains("ok "), "{name}: {}", r.out);
        assert!(
            r.summary().contains("1 held, 0 FAILED"),
            "{name}: {}",
            r.summary()
        );
    }
}

// ── A false claim fails, loudly ────────────────────────────────────

#[test]
fn a_false_claim_exits_one_and_names_the_fact() {
    let d = Dir::new("false");
    let f = d.file("wrong.ein", &format!("{BODY}{FALSE_CLAIM}"));
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(r.out.contains("(p B H3)"), "the missing fact: {}", r.out);
    assert!(r.summary().contains("0 held, 1 FAILED"), "{}", r.summary());
}

/// The case relation-closure exists for, and the derivation line M1c
/// T1c.1.3.3 added under it: a surplus fact, and the rule that put it there.
#[test]
fn a_surplus_fact_is_reported_with_the_rule_that_derived_it() {
    let d = Dir::new("surplus");
    let f = d.file(
        "mirror.ein",
        "(import std.algebra :symbols (symmetric))\n\
         (relation p Thing Thing)\n(symmetric p)\n(p A B)\n\
         (query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A B)))\n",
    );
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("(p B A)"), "the mirrored edge: {}", r.out);
    assert!(
        r.out.contains("derived by symmetric from (p A B)"),
        "and where it came from: {}",
        r.out
    );
}

/// The stage's acceptance in its own words: "a `:verdict` that came out
/// `Ambiguity` prints the k **and the models' query bindings**". A count on
/// its own sends the reader back to `solve -e` to find out what the second
/// model was.
#[test]
fn a_count_mismatch_names_the_models_it_found() {
    let r = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "--max-set-size",
        "5",
    ]);
    assert_eq!(
        r.code, 0,
        "the fixture holds as written: {}{}",
        r.out, r.err
    );

    // The same puzzle, claiming one model where there are two.
    let d = Dir::new("kmismatch");
    let src =
        std::fs::read_to_string(repo_root().join("examples/features/11_expect_ambiguity.ein"))
            .expect("the fixture");
    let one = src.replace(
        "  :expect (or (model (seat Ann S1) (seat Bob S2))
              (model (seat Ann S2) (seat Bob S1))))",
        "  :expect (model (seat Ann S1) (seat Bob S2)))",
    );
    assert_ne!(one, src, "the :expect block was rewritten");
    let f = d.file("one.ein", &one);
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("expected Solution with k = 1, got Ambiguity with k = 2"),
        "{}",
        r.out
    );
    assert!(r.out.contains("model 1 of 2"), "{}", r.out);
    assert!(r.out.contains("model 2 of 2"), "{}", r.out);
    assert!(
        r.out.contains("?who=Ann"),
        "the goal's own variables: {}",
        r.out
    );
}

// ── Only the work the expectations need runs ───────────────────────

/// **The acceptance criterion, checked where it can be observed.**
/// `features/04_open.ein` is the corpus's own example of "a run nobody can
/// finish is not coverage" — the entry whose lattice view blindly enumerates
/// an unbounded domain and which the sweep marks `slow` for it. It carries no
/// `:expect`, so `ein test` must never solve it, and the whole directory has
/// to come back in the time the three fixtures take.
#[test]
fn a_query_with_no_expectation_is_never_solved() {
    let started = std::time::Instant::now();
    let r = ein(&["test", "examples/features"]);
    let took = started.elapsed();
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.out.contains("(no expect)  examples/features/04_open.ein"),
        "{}",
        r.out
    );
    assert!(
        r.summary().contains("3 held") && r.summary().contains("9 files state no expectations"),
        "{}",
        r.summary()
    );
    // Generous by two orders of magnitude against the ~10 s `04_open` costs
    // the sweep, so this fails on the behaviour and not on the machine.
    assert!(
        took < std::time::Duration::from_secs(20),
        "the directory took {took:?} — something entered a search it was not asked to"
    );
}

// ── Directory mode and the summary ─────────────────────────────────

#[test]
fn a_directory_runs_every_file_and_the_worst_result_sets_the_code() {
    let d = Dir::new("dir");
    d.file("a_ok.ein", &format!("{BODY}{TRUE_CLAIM}"));
    d.file("b_bad.ein", &format!("{BODY}{FALSE_CLAIM}"));
    d.file("c_quiet.ein", BODY);
    let r = ein(&["test", &d.path()]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("ok "), "{}", r.out);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(r.out.contains("(no expect)"), "{}", r.out);
    assert!(
        r.summary()
            .contains("3 files, 2 expectations: 1 held, 1 FAILED, 0 not checked, 0 errors"),
        "{}",
        r.summary()
    );
    assert!(
        r.summary().contains("1 file states no expectations"),
        "{}",
        r.summary()
    );
}

/// Several paths at once, and a nested directory: the shell's glob expansion
/// is the same argv, so this is what a glob does too.
#[test]
fn several_paths_and_a_nested_directory_are_one_selection() {
    let d = Dir::new("nested");
    d.file("top.ein", &format!("{BODY}{TRUE_CLAIM}"));
    std::fs::create_dir_all(d.0.join("sub")).expect("a subdirectory");
    d.file("sub/deep.ein", &format!("{BODY}{TRUE_CLAIM}"));
    // Not a `.ein`, so a directory walk must not pick it up.
    d.file("notes.txt", "not a program");
    let r = ein(&["test", &d.path(), "examples/features/10_expect.ein"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.summary().starts_with("3 files, 3 expectations: 3 held"),
        "{}",
        r.summary()
    );
}

/// …and a program named twice is one program. A summary line that counted the
/// argv rather than the corpus would say "2 files" here.
#[test]
fn a_program_named_twice_is_counted_once() {
    let d = Dir::new("dedup");
    let f = d.file("once.ein", &format!("{BODY}{TRUE_CLAIM}"));
    let r = ein(&["test", &d.path(), &f, &f]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.summary().starts_with("1 file, 1 expectation: 1 held"),
        "{}",
        r.summary()
    );
}

/// A walk does not follow a symlinked directory, because a link back up the
/// tree is an infinite walk and a gate command that hangs is the worst way to
/// fail. Naming one explicitly still works — that is asking for it.
#[cfg(unix)]
#[test]
fn a_symlink_loop_does_not_hang_the_walk() {
    let d = Dir::new("loop");
    d.file("one.ein", &format!("{BODY}{TRUE_CLAIM}"));
    std::os::unix::fs::symlink(&d.0, d.0.join("self")).expect("a loop");
    let r = ein(&["test", &d.path()]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.summary().starts_with("1 file, 1 expectation: 1 held"),
        "{}",
        r.summary()
    );
}

/// `-q` prints the summary and whatever was not ok, and nothing else.
#[test]
fn quiet_keeps_the_failures_and_drops_the_passes() {
    let d = Dir::new("quiet");
    d.file("a_ok.ein", &format!("{BODY}{TRUE_CLAIM}"));
    d.file("b_bad.ein", &format!("{BODY}{FALSE_CLAIM}"));
    let r = ein(&["test", &d.path(), "-q"]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(!r.out.contains("a_ok.ein"), "{}", r.out);
    assert!(r.out.contains("b_bad.ein"), "{}", r.out);
}

/// `-v` says what held, with the verdict and k — the line that turns a green
/// run into evidence rather than an absence of red.
#[test]
fn verbose_reports_the_verdict_of_a_passing_query() {
    let r = ein(&["test", "examples/features/11_expect_ambiguity.ein", "-v"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains("holds (Ambiguity, k = 2)"), "{}", r.out);
}

// ── The three ways a run produces no verdict ───────────────────────

/// A load error is **2**, not 1: 1 means "an expectation is false", and a
/// runner that cannot tell a broken file from a false claim is the failure
/// this file exists to prevent.
#[test]
fn a_load_error_is_two_and_not_a_failed_expectation() {
    let r = ein(&["test", "examples/broken/load/expect_unknown_relation.ein"]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(
        r.out.lines().next().is_some_and(|l| l.starts_with("ERROR")),
        "{}",
        r.out
    );
    assert!(r.err.contains("kb load error"), "{}", r.err);
    assert!(
        r.summary().contains("0 FAILED") && r.summary().contains("1 error"),
        "a broken file is an error and not a false claim: {}",
        r.summary()
    );
}

/// An error anywhere dominates a failure: "every expectation was checked and
/// some are false" would be a lie about a run in which one file never loaded.
#[test]
fn an_error_dominates_a_failure_in_the_exit_code() {
    let d = Dir::new("dominates");
    d.file("a_bad.ein", &format!("{BODY}{FALSE_CLAIM}"));
    d.file("b_broken.ein", "(relation p Thing Place)\n(p A H1\n");
    let r = ein(&["test", &d.path()]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.summary().contains("1 FAILED"), "{}", r.summary());
    assert!(r.summary().contains("1 error"), "{}", r.summary());
}

/// A budget abort is neither a pass nor a failure — nothing was established
/// either way — so it is an ERROR and takes 2.
#[test]
fn a_budget_abort_is_an_error_and_never_a_pass() {
    let r = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "-E",
        "1",
    ]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.out.contains("ERROR"), "{}", r.out);
    assert!(r.out.contains("aborted before an answer"), "{}", r.out);
    assert!(!r.out.contains("holds"), "{}", r.out);
}

/// **A selection that checked nothing is not green.** M1c's acceptance says a
/// missing tool is reported and never skipped past; the same rule applies to a
/// corpus with no tests in it, which is how a coverage gate silently passes.
#[test]
fn a_selection_with_no_expectations_is_reported_not_passed() {
    let r = ein(&["test", "examples/features/04_open.ein"]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.out.contains("(no expect)"), "{}", r.out);
    assert!(r.err.contains("nothing to check"), "{}", r.err);
}

/// A directory with no `.ein` in it at all is the same finding one step
/// earlier, and it must not be an empty green run.
#[test]
fn an_empty_directory_is_refused() {
    let d = Dir::new("empty");
    let r = ein(&["test", &d.path()]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.err.contains("no .ein files"), "{}", r.err);
}

#[test]
fn a_path_that_does_not_exist_is_refused() {
    let r = ein(&["test", "examples/nope-not-here.ein"]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.err.starts_with("error: "), "{}", r.err);
}

// ── Exhausting is the behaviour, not a flag ────────────────────────

/// `solve` on the k = 2 fixture reports k = 1 at its `-n 1` default and cannot
/// check the claim; `test` answers it with no flag at all. Both are right —
/// `solve` is asked for an answer and `test` is asked whether a claim holds —
/// and this is the difference stated as a test.
#[test]
fn test_exhausts_where_solve_stops_at_one() {
    let file = "examples/features/11_expect_ambiguity.ein";
    let stopped = ein(&["solve", file]);
    assert_eq!(stopped.code, 1, "{}{}", stopped.out, stopped.err);
    assert!(stopped.out.contains("NOT CHECKED"), "{}", stopped.out);

    let tested = ein(&["test", file]);
    assert_eq!(tested.code, 0, "{}{}", tested.out, tested.err);
}

/// …and the one thing that can still truncate an exhaustive run — the lattice
/// depth cap — comes back NOT CHECKED rather than FAILED, with the cap named.
/// A `k = 0` from a capped search is "no model within the cap".
#[test]
fn a_capped_frontier_is_not_checked_and_says_which_cap() {
    let r = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "-m",
        "1",
    ]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("NOT CHECKED"), "{}", r.out);
    assert!(r.out.contains("--max-set-size"), "{}", r.out);
    assert!(r.summary().contains("1 not checked"), "{}", r.summary());
}

/// A file with one of each: the status column says **FAILED**, because a claim
/// shown to be false is a stronger thing to report than one nobody could
/// check. Both are exit 1, so this is about which word the column carries.
#[test]
fn a_failure_outranks_an_unchecked_one_in_the_file_label() {
    let d = Dir::new("mixed");
    let searched = "(relation instance Thing Type)\n\
         (instance Ann Person) (instance Bob Person)\n\
         (relation seat Person Slot)\n\
         (rule one-slot (?R) :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c)) \
          :assert (false) :priority 250)\n\
         (rule one-person (?R) :match (and (?R ?a ?c) (?R ?b ?c) (neq ?a ?b)) \
          :assert (false) :priority 250)\n\
         (one-slot seat) (one-person seat)\n\
         (hrule guess (?x ?y) :match (instance ?x Person) :assert (seat ?x ?y))\n";
    let f = d.file(
        "mixed.ein",
        &format!(
            "{BODY}{searched}{FALSE_CLAIM}\
             (query :goal (seat ?w ?s) :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
              :expect (or (model (seat Ann S1) (seat Bob S2)) \
                          (model (seat Ann S2) (seat Bob S1))))\n"
        ),
    );
    // At the default depth both queries answer and only the first is wrong.
    let deep = ein(&["test", &f]);
    assert_eq!(deep.code, 1, "{}{}", deep.out, deep.err);
    assert!(
        deep.out
            .lines()
            .next()
            .is_some_and(|l| l.starts_with("FAILED")),
        "{}",
        deep.out
    );

    // At a cap of 1 the second is NOT CHECKED and the first is still FAILED.
    let capped = ein(&["test", &f, "-m", "1"]);
    assert_eq!(capped.code, 1, "{}{}", capped.out, capped.err);
    assert!(capped.out.contains("NOT CHECKED"), "{}", capped.out);
    assert!(
        capped
            .out
            .lines()
            .next()
            .is_some_and(|l| l.starts_with("FAILED")),
        "the stronger finding heads the file: {}",
        capped.out
    );
    assert!(
        capped.summary().contains("1 FAILED") && capped.summary().contains("1 not checked"),
        "{}",
        capped.summary()
    );
}

// ── The artefact flags ─────────────────────────────────────────────

/// They still work, because a failing expectation is exactly when someone
/// wants the stream — over a selection that is one run.
#[test]
fn the_artefact_flags_work_over_one_run() {
    let d = Dir::new("artefacts");
    let events = d.0.join("events.jsonl");
    let summary = d.0.join("summary.json");
    let r = ein(&[
        "test",
        "examples/features/10_expect.ein",
        "--events",
        &events.to_string_lossy(),
        "--json-summary",
        &summary.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    let log = std::fs::read_to_string(&events).expect("the event log was written");
    assert!(log.lines().count() > 2, "{log}");
    let json = std::fs::read_to_string(&summary).expect("the summary was written");
    assert!(json.contains("\"schema\": \"ein-summary/1\""), "{json}");
}

/// One path cannot hold two runs, so each is refused over a selection that is
/// more than one — before anything is written, and whether the several runs
/// come from several files or from several queries in one.
#[test]
fn an_artefact_flag_over_more_than_one_run_is_refused() {
    let d = Dir::new("refused");
    d.file("a.ein", &format!("{BODY}{TRUE_CLAIM}"));
    d.file("b.ein", &format!("{BODY}{TRUE_CLAIM}"));
    let out = d.0.join("events.jsonl");

    let many_files = ein(&["test", &d.path(), "--events", &out.to_string_lossy()]);
    assert_eq!(many_files.code, 2, "{}{}", many_files.out, many_files.err);
    assert!(
        many_files.err.contains("--events names one path"),
        "{}",
        many_files.err
    );
    assert!(!out.exists(), "and nothing was written");

    let two_queries = d.file("two.ein", &format!("{BODY}{TRUE_CLAIM}{FALSE_CLAIM}"));
    let r = ein(&["test", &two_queries, "--events", &out.to_string_lossy()]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.err.contains("states 2 expectations"), "{}", r.err);
    assert!(!out.exists(), "and nothing was written");
}

// ── Several queries in one file ────────────────────────────────────

#[test]
fn every_expectation_in_a_file_is_checked_and_the_failure_is_located() {
    let d = Dir::new("queries");
    let f = d.file("two.ein", &format!("{BODY}{TRUE_CLAIM}{FALSE_CLAIM}"));
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("query 2 of 2"), "{}", r.out);
    assert!(
        !r.out.contains("query 1 of 2"),
        "the passing one is quiet: {}",
        r.out
    );
    assert!(
        r.summary()
            .contains("1 file, 2 expectations: 1 held, 1 FAILED"),
        "{}",
        r.summary()
    );
}

/// A file whose *only* expectation is on its second query: the first is never
/// solved, and the second is found rather than skipped with it.
#[test]
fn an_expectation_on_a_later_query_is_reached() {
    let d = Dir::new("later");
    let f = d.file(
        "later.ein",
        &format!("{BODY}(query :goal (p A ?h) :no-hypothesis (p))\n{FALSE_CLAIM}"),
    );
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("query 2 of 2"), "{}", r.out);
    assert!(
        r.summary().contains("1 file, 1 expectation"),
        "{}",
        r.summary()
    );
}

// ── The container ──────────────────────────────────────────────────

/// Every command that takes a `.ein` path takes a `.einb` too, dispatching on
/// the magic bytes. A directory walk still collects `.ein` only, so a
/// directory holding both does not run each program twice.
#[cfg(feature = "einb")]
#[test]
fn a_named_container_is_tested_and_a_walk_ignores_one() {
    let d = Dir::new("einb");
    let src = d.file("holds.ein", &format!("{BODY}{TRUE_CLAIM}"));
    let container = d.0.join("holds.einb");
    let saved = ein(&["kb", "save", &src, &container.to_string_lossy()]);
    assert_eq!(saved.code, 0, "{}{}", saved.out, saved.err);

    let named = ein(&["test", &container.to_string_lossy()]);
    assert_eq!(named.code, 0, "{}{}", named.out, named.err);
    assert!(
        named.summary().starts_with("1 file, 1 expectation"),
        "{}",
        named.summary()
    );

    // The directory now holds both; the walk sees one program.
    let walked = ein(&["test", &d.path()]);
    assert_eq!(walked.code, 0, "{}{}", walked.out, walked.err);
    assert!(
        walked.summary().starts_with("1 file, 1 expectation"),
        "{}",
        walked.summary()
    );
}
