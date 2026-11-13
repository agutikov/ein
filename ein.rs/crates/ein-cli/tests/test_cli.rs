//! `ein test` — M1c
//! [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test)
//! T1c.1.3.5, **the tests for the tester**.
//!
//! A test runner that reports success on a broken expectation is the worst
//! possible outcome this phase has available to it, so every fixture below
//! that must *fail* is checked for failing — with the right exit code and the
//! right message — and not merely run. The
//! [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-fuzzer)
//! precedent: the fuzzer's own three controls each failed once first.
//!
//! The comparisons themselves live in `ein-infer` — a query's classes in
//! `expect_semantics`, a region claim in `claim_round_trip` and `tests/region/`;
//! what is here is everything only a process can see — the exit code, the file
//! selection, the summary line, the one run a file gets in the mode its
//! claims need (M1e2 S1e2.1a.5), and the ways a run can decide nothing.

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

/// The facts every program here is about, with the lattice they are asked
/// over: nothing guessed, so R is root alone.
const FACTS: &str =
    "(relation p Thing Place :why \"p holds of {?1} and {?2}\")\n(p A H1)\n(p B H2)\n";
/// A region claim that holds — R is one state, both facts — and one that does
/// not. One `(solve …)` per file since M1e2 S1e2.1a.4, and since S1e2.1a.5 a
/// claim about the state is the region claim's.
const TRUE_CLAIM: &str = "(solve :no-hypothesis (p) :expect (region (model (p A H1) (p B H2))))\n\
                          (query :goal (p A ?h))\n";
const FALSE_CLAIM: &str = "(solve :no-hypothesis (p) :expect (region (model (p A H1) (p B H3))))\n\
                           (query :goal (p A ?h))\n";
/// The same lattice with no claim, for files whose claims are queries'.
const BODY: &str = "(relation p Thing Place :why \"p holds of {?1} and {?2}\")\n(p A H1)\n(p B H2)\n(solve :no-hypothesis (p))\n";
/// A query's claim lists its projection classes: `(p A ?h)` has one, `(p A H1)`.
const TRUE_QUERY: &str = "(query :goal (p A ?h) :expect (model (p A H1)))\n";
const FALSE_QUERY: &str = "(query :goal (p A ?h) :expect (model (p A H2)))\n";

fn holds() -> String {
    format!("{FACTS}{TRUE_CLAIM}")
}

fn fails() -> String {
    format!("{FACTS}{FALSE_CLAIM}")
}

// ── The corpus fixtures that state their own answer, which must hold ──

/// One per shape since M1e2 S1e2.1a.5: `10_expect` claims R is one state,
/// `11_expect_ambiguity` that its maximal line has two projection classes
/// (`:over maximal`), and `12_expect_false` that every node is dead. Each
/// `ein test` run is the mode its claim needs and no flag.
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
    let f = d.file("wrong.ein", &fails());
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(
        r.out
            .contains("the model lists (p B H3), which the state does not hold"),
        "the missing fact: {}",
        r.out
    );
    assert!(r.summary().contains("0 held, 1 FAILED"), "{}", r.summary());
}

/// The case relation-closure exists for, and the derivation line M1c
/// T1c.1.3.3 added under it: a surplus fact, and the rule that put it there.
/// Since M1e2 S1e2.1a.5 it is the region claim's, which kept the line.
#[test]
fn a_surplus_fact_is_reported_with_the_rule_that_derived_it() {
    let d = Dir::new("surplus");
    let f = d.file(
        "mirror.ein",
        "(import std.algebra :symbols (symmetric))\n\
         (relation p Thing Thing :why \"p holds of {?1} and {?2}\")\n(symmetric p)\n(p A B)\n\
         (solve :no-hypothesis (p) :expect (region (model (p A B))))\n\
         (query :goal (p A ?h))\n",
    );
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("the state holds (p B A), which the model does not list"),
        "the mirrored edge: {}",
        r.out
    );
    assert!(
        r.out.contains("derived by symmetric from (p A B)"),
        "and where it came from: {}",
        r.out
    );
}

/// A query claim that lists one class where the query has two says how many
/// it lists, and which class it did not list. Since M1f S1f.6.27 the claim
/// is asked `--maximal --single`, which stops at the line's second state:
/// that state's class refutes the claim, and the lines say that the run did
/// not exhaust the line. Until then the description said *the query has 2*.
#[test]
fn a_class_mismatch_names_the_class_it_did_not_list() {
    let r = ein(&["test", "examples/features/11_expect_ambiguity.ein"]);
    assert_eq!(
        r.code, 0,
        "the fixture holds as written: {}{}",
        r.out, r.err
    );

    // The same puzzle, claiming one seating where the maximal line has two.
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
    for line in [
        "the claim lists 1 class, and the maximal line has a class it does not list: \
         (model (seat Ann S2) (seat Bob S1))",
        "the run asked whether its maximal line is one state and did not exhaust the maximal \
         line: what it found refutes the claim, and is not every difference",
    ] {
        assert!(r.out.contains(line), "{line}:\n{}", r.out);
    }
    // The description of the same file says every difference, as it did.
    let described = ein(&["solve", &f, "--maximal"]);
    assert_eq!(described.code, 1, "{}{}", described.out, described.err);
    assert!(
        described.out.contains(
            "the claim lists 1 class, the query has 2: \
             (model (seat Ann S2) (seat Bob S1)) is not listed"
        ),
        "{}",
        described.out
    );
}

// ── Only the work the expectations need runs ───────────────────────

/// **The acceptance criterion, checked where it can be observed.**
/// `features/04_open.ein` is the corpus's own example of "a run nobody can
/// finish is not coverage" — the entry whose lattice view blindly enumerates
/// an unbounded domain and which the sweep marks `slow` for it. It carries no
/// `:expect`, so `ein test` must never solve it, and the whole directory has
/// to come back in the time the eight fixtures take. (Three until M1e S1e.3.1
/// added `13_mixed_solution_and_open.ein`, whose claim is `CO-M2`'s witness,
/// four until M1f S1f.5.2's T1f.5.2.2 added `14_unicode_text.ein` and
/// `15_unicode_escapes.ein`, the two examples of natural-language text, and
/// six until M1f S1f.5.7's T1f.5.7.2 added `16_eq_neq_goal.ein`, nine claims
/// about `eq` and `neq` in a goal, and seven until its T1f.5.7.3 added
/// `17_eq_neq_premise.ein`, four about them in a premise.)
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
        r.summary().contains("19 held") && r.summary().contains("9 files state no expectations"),
        "{}",
        r.summary()
    );
    // **The margin, measured** — M1e S1e.4.5, `TE-L1`. The comment here used
    // to price it against `04_open`'s ~10 s in the *sweep*, which is a
    // different operation: what this runs is `ein test examples/features`,
    // and it is **15 ms** on this box (2026-09-01, three runs, dev binary).
    // So the ceiling is ~1 300× rather than the two orders of magnitude
    // claimed, and this assertion cannot fail on machine load — it fires only
    // if something entered a search it was not asked to, which is the
    // behaviour it exists for. It is the one wall-clock assertion in the
    // workspace whose margin makes gating pointless; the tight one is
    // `corpus_cli`'s recorded-cost band, which is behind `EIN_CORPUS_SLOW`.
    assert!(
        took < std::time::Duration::from_secs(20),
        "the directory took {took:?} against ~15 ms — something entered a \
         search it was not asked to (or, at 1 300× the measured cost, the \
         machine is in trouble)"
    );
}

// ── Directory mode and the summary ─────────────────────────────────

#[test]
fn a_directory_runs_every_file_and_the_worst_result_sets_the_code() {
    let d = Dir::new("dir");
    d.file("a_ok.ein", &holds());
    d.file("b_bad.ein", &fails());
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
    d.file("top.ein", &holds());
    std::fs::create_dir_all(d.0.join("sub")).expect("a subdirectory");
    d.file("sub/deep.ein", &holds());
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
    let f = d.file("once.ein", &holds());
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
    d.file("one.ein", &holds());
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
    d.file("a_ok.ein", &holds());
    d.file("b_bad.ein", &fails());
    let r = ein(&["test", &d.path(), "-q"]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(!r.out.contains("a_ok.ein"), "{}", r.out);
    assert!(r.out.contains("b_bad.ein"), "{}", r.out);
}

/// `-v` says what held, with what the run found: R's size and route for a
/// region claim, the set and its classes for a query's. Both counts of the
/// walk's header (`k` and `recorded`, M1e S1e.3.4) went with it at M1e2
/// S1e2.1a.5, and the owing fixture that parted them now claims R empty.
#[test]
fn verbose_reports_what_the_run_found() {
    let r = ein(&["test", "examples/features/11_expect_ambiguity.ein", "-v"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("holds (--maximal -n 3: the maximal line, 2 classes over 2 states)"),
        "{}",
        r.out
    );

    let open = ein(&["test", "tests/stdlib/slots/03_fill.ein", "-v"]);
    assert_eq!(open.code, 0, "{}{}", open.out, open.err);
    assert!(
        open.out
            .contains("solve · :expect (region …) — holds (state route, k_s = 0, k_c = 0)"),
        "an owing R is described, and the header says how big:\n{}",
        open.out
    );
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
    d.file("a_bad.ein", &fails());
    d.file("b_broken.ein", "(relation p Thing Place)\n(p A H1\n");
    let r = ein(&["test", &d.path()]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.summary().contains("1 FAILED"), "{}", r.summary());
    assert!(r.summary().contains("1 error"), "{}", r.summary());
}

/// A `-T` abort is neither a pass nor a failure — nothing was established
/// either way — so it is an ERROR and takes 2. The walk's `-E` retired at M1e2
/// S1e2.1a.5; a budget now answers in part (below), and only the wall clock
/// aborts.
#[test]
fn a_deadline_abort_is_an_error_and_never_a_pass() {
    // The 2¹⁶ region of `lattice/07`, claimed whole: no deadline of 0 s
    // reaches it.
    let d = Dir::new("deadline");
    let src =
        std::fs::read_to_string(repo_root().join("examples/lattice/07_two_pairs_two_tables.ein"))
            .expect("the fixture");
    let f = d.file(
        "slow.ein",
        &format!("{src}\n(query :goal (sits ?p ?s) :expect (false))\n"),
    );
    let r = ein(&["test", &f, "-T", "0"]);
    assert_eq!(r.code, 2, "{}{}", r.out, r.err);
    assert!(r.out.contains("ERROR"), "{}", r.out);
    assert!(r.out.contains("aborted before an answer"), "{}", r.out);
    assert!(!r.out.contains("holds"), "{}", r.out);
    let e = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "-E",
        "1",
    ]);
    assert_eq!(e.code, 2, "{}{}", e.out, e.err);
    assert!(e.err.contains("-E/--max-enterings is retired"), "{}", e.err);
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

// ── The mode a claim needs, and nothing more ───────────────────────

/// `ein test` runs the mode a file's claims need (`solve_interface.md`
/// § 2.3): `11`'s claim is about the maximal line, so it runs `--maximal`.
/// `ein solve` checks the same claim against what it was asked: the
/// description holds it, and `--minimal` cannot decide it, which is a `note:`
/// and exit 0 under `solve` — the user's ruling of 2026-10-16 — where `ein
/// test`, whose whole job is checking, says NOT CHECKED and exits 1.
#[test]
fn test_runs_the_mode_a_claim_needs() {
    let file = "examples/features/11_expect_ambiguity.ein";
    let described = ein(&["solve", file]);
    assert_eq!(described.code, 0, "{}{}", described.out, described.err);
    assert!(
        described.out.contains(":expect           holds"),
        "{}",
        described.out
    );

    let minimal = ein(&["solve", file, "--minimal"]);
    assert_eq!(minimal.code, 0, "{}{}", minimal.out, minimal.err);
    assert!(minimal.out.contains("NOT CHECKED"), "{}", minimal.out);
    assert!(minimal.err.starts_with("note: "), "{}", minimal.err);

    let tested = ein(&["test", file]);
    assert_eq!(tested.code, 0, "{}{}", tested.out, tested.err);
}

/// …and the one thing that can still stop a run short — its budget — comes
/// back NOT CHECKED rather than FAILED, saying the run did not exhaust what
/// the claim is about.
#[test]
fn a_budget_stopped_run_is_not_checked_and_says_why() {
    let r = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "--region-budget",
        "1",
    ]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("NOT CHECKED"), "{}", r.out);
    assert!(
        r.out
            .contains("the claim is about the maximal line, and this run did not exhaust it"),
        "{}",
        r.out
    );
    assert!(r.summary().contains("1 not checked"), "{}", r.summary());
}

/// One file with a claim that fails and one nobody can check: a false region
/// claim, and a query claim on a query with no `:goal`, which no run answers.
fn mixed() -> String {
    format!("{}(query :expect (model))\n", fails())
}

/// A file with one of each: the status column says **FAILED**, because a claim
/// shown to be false is a stronger thing to report than one nobody could
/// check. Both are exit 1, so this is about which word the column carries.
#[test]
fn a_failure_outranks_an_unchecked_one_in_the_file_label() {
    let d = Dir::new("mixed");
    let f = d.file("mixed.ein", &mixed());
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("NOT CHECKED"), "{}", r.out);
    assert!(
        r.out
            .lines()
            .next()
            .is_some_and(|l| l.starts_with("FAILED")),
        "the stronger finding heads the file: {}",
        r.out
    );
    assert!(
        r.summary().contains("1 FAILED") && r.summary().contains("1 not checked"),
        "{}",
        r.summary()
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
    assert!(json.contains("\"schema\": \"ein-summary/2\""), "{json}");
}

/// One path cannot hold two runs, so each is refused over a selection of
/// more than one file — before anything is written. A file is **one** run
/// since M1e2 S1e2.1a.5, whatever it claims, so a file with several claims
/// takes the flag.
#[test]
fn an_artefact_flag_over_more_than_one_run_is_refused() {
    let d = Dir::new("refused");
    d.file("a.ein", &holds());
    d.file("b.ein", &holds());
    let out = d.0.join("events.jsonl");

    let many_files = ein(&["test", &d.path(), "--events", &out.to_string_lossy()]);
    assert_eq!(many_files.code, 2, "{}{}", many_files.out, many_files.err);
    assert!(
        many_files.err.contains("--events names one path"),
        "{}",
        many_files.err
    );
    assert!(!out.exists(), "and nothing was written");

    let sub = Dir::new("refused-one");
    let two = sub.file(
        "two.ein",
        &format!("{FACTS}(solve :no-hypothesis (p) :expect (region (model (p A H1) (p B H2))))\n{TRUE_QUERY}"),
    );
    let r = ein(&["test", &two, "--events", &out.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    let log = std::fs::read_to_string(&out).expect("one run, one stream");
    assert_eq!(
        log.lines()
            .filter(|l| l.contains("\"e\": \"region\""))
            .count(),
        1,
        "{log}"
    );
}

// ── Several queries in one file ────────────────────────────────────

#[test]
fn every_expectation_in_a_file_is_checked_and_the_failure_is_located() {
    let d = Dir::new("queries");
    let f = d.file("two.ein", &format!("{BODY}{TRUE_QUERY}{FALSE_QUERY}"));
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
        &format!("{BODY}(query :goal (p A ?h))\n{FALSE_QUERY}"),
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
    let src = d.file("holds.ein", &holds());
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

// ── The report ─────────────────────────────────────────────────────

/// `--json-report` over a selection, parsed.
fn report(args: &[&str], out: &std::path::Path) -> serde_json::Value {
    let mut argv: Vec<&str> = vec!["test"];
    argv.extend_from_slice(args);
    let path = out.to_string_lossy().into_owned();
    argv.extend_from_slice(&["--json-report", &path]);
    let r = ein(&argv);
    let text = std::fs::read_to_string(out)
        .unwrap_or_else(|e| panic!("no report at {path} ({e}): {}{}", r.out, r.err));
    serde_json::from_str(&text).expect("the report parses")
}

/// **The row set accounts for the selection, file for file.**
///
/// The census this exists for asks a *fraction* — how much of the corpus makes
/// a claim about its own model set — and a report that listed only the
/// numerator could not answer it. So a query with no `:expect` gets a row, a
/// file with no `(query …)` gets a row, and a file that did not load gets a
/// row saying so.
#[test]
fn every_file_of_the_selection_has_a_row() {
    let d = Dir::new("report-rows");
    let r = report(&["examples/features"], &d.0.join("r.json"));
    assert_eq!(r["schema"], "ein-test-report/5");
    let rows = r["rows"].as_array().expect("rows");
    let files: std::collections::BTreeSet<&str> =
        rows.iter().map(|x| x["path"].as_str().unwrap()).collect();
    assert_eq!(
        files.len(),
        r["tally"]["files"].as_u64().unwrap() as usize,
        "one file per row set"
    );
    let claims = rows.iter().filter(|x| !x["expect"].is_null()).count();
    assert_eq!(claims, r["tally"]["held"].as_u64().unwrap() as usize);
}

/// **The shape is read off the loaded program, and that is the whole point.**
///
/// `10_expect.ein`'s header comment documents the `(or …)` form on line 12 and
/// its `:expect` is a `(model …)`; a grep for `:expect (or` finds it and is
/// wrong. That mistake was made — it is what M1d S1d.4.1's first-hour
/// reconnaissance reported, and correcting it took the corpus's count of
/// set-closure claims from two to **one**
/// ([the census](../../../../docs/history/m1d_satisfiability/closure_census.md)).
#[test]
fn the_shape_comes_from_the_program_and_not_from_the_text() {
    let d = Dir::new("report-shape");
    let r = report(&["examples/features"], &d.0.join("r.json"));
    let shape = |name: &str| -> String {
        r["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["path"].as_str().unwrap().ends_with(name))
            .unwrap_or_else(|| panic!("no row for {name}"))["expect"]["shape"]
            .as_str()
            .unwrap_or("null")
            .to_string()
    };
    // Since M1e2 S1e2.1a.5 `10` and `12` state their claims as region claims,
    // whose row is the file's first; `11`'s is its query's.
    assert_eq!(
        shape("10_expect.ein"),
        "region",
        "the comment is not the claim"
    );
    assert_eq!(shape("11_expect_ambiguity.ein"), "or");
    assert_eq!(shape("12_expect_false.ein"), "false");
    assert_eq!(shape("04_open.ein"), "null", "no `:expect`, no shape");

    let row = |name: &str, place: &str| -> serde_json::Value {
        r["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["path"].as_str().unwrap().ends_with(name) && x["place"] == place)
            .cloned()
            .unwrap()
    };
    let amb = row("11_expect_ambiguity.ein", "query");
    assert_eq!(amb["expect"]["models"], 2, "the claim lists two classes");
    assert_eq!(amb["expect"]["over"], "maximal");
    assert_eq!(amb["answer"]["classes"], 2);
    assert_eq!(amb["answer"]["exact"], true);
    assert_eq!(amb["answer"]["over"], "maximal");
    // What a query's goal asks about — the first factor of the write cost of
    // a claim a file does not yet carry.
    assert_eq!(
        row("10_expect.ein", "query")["goal_relations"][0],
        "next-to"
    );
    assert_eq!(row("10_expect.ein", "solve")["region"]["k_s"], 1);
}

/// **Additive**: same stdout, same stderr, same exit code, and — the one that
/// matters here — the same work. A query stating nothing is still never
/// solved, so its row carries no run.
#[test]
fn the_report_changes_nothing_but_writes_a_file() {
    let d = Dir::new("report-additive");
    let out = d.0.join("r.json");
    let bare = ein(&["test", "examples/features", "-v"]);
    let with = ein(&[
        "test",
        "examples/features",
        "-v",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    assert_eq!(bare.code, with.code);
    assert_eq!(bare.err, with.err);
    // The summary line carries the wall clock, which is not an observable.
    let strip = |r: &Run| {
        r.out
            .lines()
            .filter(|l| !l.contains(" held, "))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(strip(&bare), strip(&with));

    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).unwrap()).unwrap();
    let open = doc["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["path"].as_str().unwrap().ends_with("04_open.ein"))
        .unwrap()
        .clone();
    assert_eq!(open["outcome"], "no-expect");
    assert!(
        open["answer"].is_null(),
        "it must not have been solved: {open}"
    );
}

/// It is the report of a *run*, not of a query, so it takes any selection —
/// which is the difference from `--json-summary`, refused over more than one
/// run three tests above. One invocation over the three corpus roots is the
/// whole of M1d S1d.4.1's first two tables.
#[test]
fn the_report_takes_a_selection_of_many_runs() {
    let d = Dir::new("report-many");
    let r = report(
        &["tests/stdlib/algebra", "examples/features"],
        &d.0.join("r.json"),
    );
    assert!(r["tally"]["files"].as_u64().unwrap() > 30, "{}", r["tally"]);
    assert!(r["tally"]["held"].as_u64().unwrap() > 20, "{}", r["tally"]);
}

/// A file that did not load states nothing, **because a claim is a property of
/// a program**. Three fixtures under `examples/broken/load/` contain the token
/// `:expect` and are refused by the loader; counting them as claims would put
/// the loader's own negatives in the numerator of "what fraction of the corpus
/// claims a model set".
#[test]
fn a_refused_file_carries_no_claim() {
    let d = Dir::new("report-refused");
    let r = report(
        &["examples/broken/load/expect_unknown_relation.ein"],
        &d.0.join("r.json"),
    );
    let row = &r["rows"][0];
    assert_eq!(row["outcome"], "error");
    assert_eq!(row["queries"], 0);
    assert!(row["expect"].is_null(), "{row}");
}

/// **The rows line up with the queries**, which is the one place a row set can
/// silently go wrong: rows are indexed from the file's base offset, so a
/// second file in the selection whose *second* query is the one that claims
/// would land its outcome on the first file's row if the offset were dropped.
#[test]
fn a_files_rows_are_indexed_from_its_own_base() {
    let d = Dir::new("report-offset");
    d.file("a_quiet.ein", BODY);
    d.file(
        "b_two.ein",
        &format!("{BODY}(query :goal (p A ?h))\n{TRUE_QUERY}"),
    );
    let r = report(&[&d.path()], &d.0.join("r.json"));
    let rows = r["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 3, "{rows:?}");
    assert!(rows[0]["path"].as_str().unwrap().contains("a_quiet"));
    // No `(query …)` at all, which is a different thing from a query that
    // claims nothing — every `stdlib/*.ein` is one, and the census's
    // denominator needs to be able to tell them apart.
    assert_eq!(rows[0]["outcome"], "no-query");
    assert_eq!(rows[0]["queries"], 0);
    assert!(rows[0]["answer"].is_null());

    assert_eq!(rows[1]["query"], 1);
    assert_eq!(rows[1]["queries"], 2);
    assert_eq!(
        rows[1]["outcome"], "no-expect",
        "the query that claims nothing"
    );
    assert!(
        rows[1]["answer"].is_null(),
        "and it was not solved: {}",
        rows[1]
    );

    assert_eq!(rows[2]["query"], 2);
    assert_eq!(rows[2]["outcome"], "held");
    assert_eq!(rows[2]["expect"]["shape"], "model");
    assert_eq!(rows[2]["answer"]["classes"], 1);
}

// ── The two surfaces that report one run ───────────────────────────

/// The header line `ein test -v` prints for a claim, **rebuilt from that
/// claim's report row**.
///
/// The direction is the whole point: every number comes from the row, under
/// the row's names, so a header that printed one of them under the other's
/// label fails here. That is exactly what `SE-M1` was — `k = {}` carried
/// `stats.solution_nodes`, which parts from `Verdict::k` on every `Open`
/// entry, so the human-facing and the machine-facing surfaces of one run
/// disagreed by name on twelve of the corpus's programs.
fn header_for(row: &serde_json::Value) -> String {
    let verb = match row["outcome"].as_str().expect("an outcome") {
        "held" => "holds",
        "failed" => "FAILED",
        "not-checked" => "NOT CHECKED",
        other => panic!("a row carrying a run came out {other}: {row}"),
    };
    // A `(solve … :expect …)` claim's row (M1e2 S1e2.1a.4): its run is R, by
    // the route that described it, and its header says so.
    if row["place"] == "solve" {
        let e = &row["expect"];
        let shape = match e["shape"].as_str().expect("a shape") {
            "false" => "(false)",
            _ if e["models"] == 0 && e["owing"] == 0 => "(region)",
            _ => "(region …)",
        };
        let region = &row["region"];
        return format!(
            "  solve · :expect {shape} — {verb} ({}, k_s = {}, k_c = {})",
            region["route"].as_str().expect("a route"),
            region["k_s"],
            region["k_c"],
        );
    }
    let goal = row["goal"].as_str().unwrap_or("?");
    let (q, n) = (
        row["query"].as_u64().expect("a query index"),
        row["queries"].as_u64().expect("a query count"),
    );
    let where_ = if n == 1 {
        format!(":goal {goal}")
    } else {
        format!("query {q} of {n} · :goal {goal}")
    };
    let a = &row["answer"];
    if a.is_null() {
        return format!("  {where_} — {verb}");
    }
    let over = match a["over"].as_str().expect("an over") {
        "all" => "all of R",
        "minimal" => "the minimal line",
        "maximal" => "the maximal line",
        _ => "the states listed",
    };
    let classes = a["classes"].as_u64().expect("classes");
    // The question the claim was read off, and the one asked first where it
    // left the claim undecided (M1f S1f.6.27).
    let route = row["route"]
        .as_str()
        .expect("an answered claim has a route");
    let after = match row["fell_back_from"].as_str() {
        Some(first) => format!(", after {first}"),
        None => String::new(),
    };
    format!(
        "  {where_} — {verb} ({route}{after}: {over}, {} over {} states)",
        if classes == 1 {
            "1 class".to_string()
        } else {
            format!("{classes} classes")
        },
        a["states"],
    )
}

/// The query headers of a `-v` run: two leading spaces, where a file's status
/// line has none and a disagreement's detail line has four.
fn headers(out: &str) -> Vec<String> {
    out.lines()
        .filter(|l| l.starts_with("  ") && !l.starts_with("    "))
        .map(str::to_string)
        .collect()
}

/// **The verbose header and the report row are one run, said twice** — M1e
/// S1e.3.2, `SE-M1`.
///
/// The finding was one label and [S1e.3.4] fixed it; this is the half that
/// makes the *class* closed rather than the instance. `ein test` publishes
/// what a run found through two surfaces — a line a human reads and a row a
/// census parses — and nothing compared them, which is how the header came to
/// print the search's count under the verdict's name and stay that way from
/// S1c.1.3 to M1e.
///
/// The whole corpus in one invocation, because it costs 0.04 s and because the
/// interesting cells are the ones a hand-picked cover would not have thought
/// to include: **13 of the 68 checked queries have `k != solution_nodes`**,
/// and every one of them is a program that states an obligation.
///
/// [S1e.3.4]: ../../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium
#[test]
fn the_verbose_header_and_the_report_row_agree_field_for_field() {
    let d = Dir::new("cross-surface");
    let out = d.0.join("r.json");
    let r = ein(&[
        "test",
        "examples",
        "tests",
        "stdlib",
        "-v",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");

    // Every row that carries a claim, in walk order — which is print order,
    // so the two sequences are comparable position for position.
    let ran: Vec<&serde_json::Value> = doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|x| !x["expect"].is_null() && x["outcome"] != "error")
        .collect();

    let observed = headers(&r.out);
    let expected: Vec<String> = ran.iter().map(|row| header_for(row)).collect();
    assert_eq!(
        observed.len(),
        expected.len(),
        "the run printed {} claim headers and the report carries {} claims",
        observed.len(),
        expected.len()
    );
    // The **first** disagreement, not all of them: eighty lines of context
    // bury the one line that is the finding.
    if let Some((i, (got, want))) = observed
        .iter()
        .zip(&expected)
        .enumerate()
        .find(|(_, (a, b))| a != b)
    {
        let row = ran[i];
        panic!(
            "claim {i} of the selection reports two different runs\n  \
             header: {got}\n  row:    {want}\n  from:   {}",
            row["path"]
        );
    }

    // …and it is not vacuous: both places, and every shape.
    assert!(
        expected.len() >= 75,
        "only {} claims ran — the selection stopped covering the corpus",
        expected.len()
    );
    let shapes: std::collections::BTreeSet<&str> = ran
        .iter()
        .filter_map(|x| x["expect"]["shape"].as_str())
        .collect();
    assert!(
        // No corpus query claims a single class; `(or …)` is the query's.
        ["false", "region", "or"].iter().all(|s| shapes.contains(s)),
        "the selection reached only {shapes:?}; the header is per place and shape"
    );
    let owing = ran
        .iter()
        .filter(|x| x["expect"]["owing"].as_u64().unwrap_or(0) > 0)
        .count();
    assert!(owing >= 10, "only {owing} owing claims in the selection");
}

/// The same correspondence on the two outcomes the corpus does not contain,
/// and on the multi-query header form — a claim that is **false** and one
/// nobody could **check**, in one file.
///
/// `held` is 68 of 68 in the sweep above by construction: the gate is green.
/// A test that only ever saw `holds` would not notice a verb that stopped
/// being rendered, and the row's `outcome` is the field with the most words.
#[test]
fn the_two_surfaces_agree_on_a_failure_and_on_an_unchecked_claim() {
    let d = Dir::new("cross-surface-red");
    let f = d.file("mixed.ein", &mixed());
    let out = d.0.join("r.json");
    let r = ein(&["test", &f, "-v", "--json-report", &out.to_string_lossy()]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    let ran: Vec<&serde_json::Value> = doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|x| !x["expect"].is_null())
        .collect();
    let outcomes: Vec<&str> = ran.iter().filter_map(|x| x["outcome"].as_str()).collect();
    assert_eq!(
        outcomes,
        vec!["failed", "not-checked"],
        "the fixture no longer produces one of each: {}",
        r.out
    );
    let expected: Vec<String> = ran.iter().map(|row| header_for(row)).collect();
    assert_eq!(headers(&r.out), expected, "{}{}", r.out, r.err);
    // The multi-query form, spelled out once: a file with two queries names
    // which of them the line is about, and the row is where that number comes
    // from.
    assert!(
        expected[1].starts_with("  query 2 of 2 · :goal "),
        "{expected:?}"
    );
}

// ── What H leaves out, on both surfaces — M1e2 S1e2.3.10 ───────────

/// Each claim header of a `-v` run with the lines printed under it: two
/// leading spaces for the header, four for its lines.
fn claim_blocks(out: &str) -> Vec<(String, Vec<String>)> {
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    for l in out.lines() {
        if l.starts_with("    ") {
            if let Some(b) = blocks.last_mut() {
                b.1.push(l.trim_start().to_string());
            }
        } else if l.starts_with("  ") {
            blocks.push((l.to_string(), Vec::new()));
        }
    }
    blocks
}

/// Every claim's scope lines against its row: `outside the universe: …`
/// where the row's `outside` names a relation, each of them named in the
/// line, and `a type as an object: …` where its `types_as_objects` names an
/// object, each named in the line; neither line where the row's list is
/// empty. Rows in walk order, which is print order. The failures.
fn scope_disagreements(out: &str, doc: &serde_json::Value) -> Vec<String> {
    let rows: Vec<&serde_json::Value> = doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|x| !x["expect"].is_null() && x["outcome"] != "error")
        .collect();
    let blocks = claim_blocks(out);
    let mut bad = Vec::new();
    if rows.len() != blocks.len() {
        bad.push(format!(
            "{} claim headers printed, {} claims in the report",
            blocks.len(),
            rows.len()
        ));
        return bad;
    }
    for (row, (header, lines)) in rows.iter().zip(&blocks) {
        let line =
            |prefix: &str| -> Option<&String> { lines.iter().find(|l| l.starts_with(prefix)) };
        let names = |v: &serde_json::Value, key: Option<&str>| -> Vec<String> {
            v.as_array()
                .unwrap_or_else(|| panic!("a list where {row} has a run"))
                .iter()
                .map(|x| match key {
                    Some(k) => x[k].as_str().expect("a name").to_string(),
                    None => x.as_str().expect("a name").to_string(),
                })
                .collect()
        };
        let outside = names(&row["outside"], Some("relation"));
        let objects = names(&row["types_as_objects"]["objects"], None);
        for (what, prefix, named) in [
            ("outside", "outside the universe: ", &outside),
            ("types_as_objects", "a type as an object: ", &objects),
        ] {
            match (line(prefix), named.is_empty()) {
                (None, true) => {}
                (Some(l), false) if named.iter().all(|n| l.contains(n.as_str())) => {}
                (l, _) => bad.push(format!(
                    "{header} ({}): the text prints {l:?}, the row's {what} is {named:?}",
                    row["path"]
                )),
            }
        }
    }
    bad
}

/// **The report's row carries what H leaves out, wherever `ein test`
/// prints it beside a claim** — M1e2 S1e2.3.10, P1e2.3's N8.
///
/// Beside each claim it prints, `ein test` says what the universe leaves
/// out (`outside the universe: …`, M1e2 S1e2.2.8) and what the untyped rung
/// bound as an object (`a type as an object: …`, S1e2.2.10). Until
/// S1e2.3.10 the report's rows carried neither: `tests/region/07_cube.ein`
/// printed `outside the universe: is-a …` and its row's `region` was `{route,
/// monotone, k_s, k_c, ms, status, stale_nodes, paths_differ}`. Since
/// `ein-test-report/4` every row carries `outside` and `types_as_objects`,
/// the summary's `root.hypgen` fields under its names and in its shapes,
/// `null` where no run checked the row's claim. The corpus prints the first
/// beside one claim and the second beside none (2026-11-06), so the second
/// is held here on `branching/02`'s program with two claims added: a solve
/// claim and a query's, each false, whose lines print without `-v` too.
#[test]
fn the_report_row_carries_what_h_leaves_out_where_the_text_prints_it() {
    let d = Dir::new("report-scope");
    let program =
        std::fs::read_to_string(repo_root().join("examples/branching/02_one_dead_one_alive.ein"))
            .expect("branching/02 reads");
    let typed = d.file(
        "typed.ein",
        &format!("{program}\n(solve :expect (false))\n(query :goal (co-located Red ?h) :expect (false))\n"),
    );
    let out = d.0.join("r.json");
    let r = ein(&[
        "test",
        "tests/region/07_cube.ein",
        &typed,
        "-v",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    assert_eq!(
        r.code, 1,
        "the typed program's claims are false: {}{}",
        r.out, r.err
    );
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    assert_eq!(doc["schema"], "ein-test-report/5");
    let bad = scope_disagreements(&r.out, &doc);
    assert!(bad.is_empty(), "{}\n{}", bad.join("\n"), r.out);

    let rows = doc["rows"].as_array().expect("rows");
    let row = |path: &str, place: &str, query: u64| -> &serde_json::Value {
        rows.iter()
            .find(|x| {
                x["path"].as_str().unwrap().ends_with(path)
                    && x["place"] == place
                    && x["query"] == query
            })
            .unwrap_or_else(|| panic!("no {place} row {query} for {path}"))
    };
    // `tests/region/07`: the inventory's S-4 witness.
    let cube = row("07_cube.ein", "solve", 0);
    assert_eq!(
        cube["outside"],
        serde_json::json!([{"relation": "is-a", "arity": 2, "why": "no-hrule"}]),
        "{cube}"
    );
    assert_eq!(
        cube["types_as_objects"],
        serde_json::json!({"objects": [], "relations": [], "candidates": 0})
    );
    assert!(
        r.out.contains("    outside the universe: is-a — "),
        "{}",
        r.out
    );
    // Its query states no claim and no run checked it: `null`.
    let silent = row("07_cube.ein", "query", 1);
    assert_eq!(silent["outcome"], "no-expect");
    assert!(silent["outside"].is_null() && silent["types_as_objects"].is_null());
    // The typed program, both claims: `Color, House` bound as objects.
    for (place, query) in [("solve", 0), ("query", 2)] {
        let t = row("typed.ein", place, query);
        assert_eq!(t["outcome"], "failed", "{t}");
        assert_eq!(
            t["types_as_objects"]["objects"],
            serde_json::json!(["Color", "House"]),
            "{t}"
        );
        assert_eq!(t["outside"], serde_json::json!([]), "{t}");
    }
    assert_eq!(
        r.out
            .matches("    a type as an object: Color, House")
            .count(),
        2,
        "{}",
        r.out
    );
}

/// **The same over the corpus**: every claim `ein test -v` prints, its scope
/// lines against its row, both directions (M1e2 S1e2.3.10).
#[test]
fn every_corpus_claims_scope_lines_are_its_rows() {
    let d = Dir::new("report-scope-corpus");
    let out = d.0.join("r.json");
    let r = ein(&[
        "test",
        "examples",
        "tests",
        "stdlib",
        "-v",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    let bad = scope_disagreements(&r.out, &doc);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // Not vacuous: `tests/region/07` prints the one (2026-11-06).
    assert!(
        doc["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .any(|x| x["outside"].as_array().is_some_and(|o| !o.is_empty())),
        "no corpus claim's row names a relation outside the universe"
    );
}

// ── The question a claim is asked — M1f S1f.6.27 ───────────────────

/// `tests/region/08_query_classes.ein`'s program, less its query: three
/// candidates and R of seven states, ∅ among them, of which three hold
/// `(a-prop X)`.
fn seven_states() -> String {
    let src = std::fs::read_to_string(repo_root().join("tests/region/08_query_classes.ein"))
        .expect("the fixture");
    let at = src.find("(query").expect("its query");
    src[..at].to_string()
}

/// The claim rows of a report, in order.
fn claim_rows(doc: &serde_json::Value) -> Vec<&serde_json::Value> {
    doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|x| !x["expect"].is_null())
        .collect()
}

/// **A claim is asked the least question that decides it, and the surfaces
/// say which** (T1f.6.27.3): `--single` for a claim of one class, `-n k+1`
/// for one of k, the line's flag before it for a claim over a line, and
/// describe for a solve claim. `-v` names the question before what the run
/// found, and `ein-test-report/5`'s row carries it as `route`.
#[test]
fn a_claim_is_asked_the_least_question_and_the_surfaces_say_which() {
    let d = Dir::new("routes");
    let out = d.0.join("r.json");
    let r = ein(&[
        "test",
        "examples/beyond_arith/cryptarithm.ein",
        "examples/beyond_arith/equation_two_unknowns.ein",
        "examples/features/11_expect_ambiguity.ein",
        "examples/beyond_arith/equation_refuted.ein",
        "examples/features/04_open.ein",
        "-v",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    for line in [
        "— holds (--single: all of R, 1 class over 1 states)",
        "— holds (-n 4: all of R, 3 classes over 3 states)",
        "— holds (--maximal -n 3: the maximal line, 2 classes over 2 states)",
        "solve · :expect (false) — holds (state route, k_s = 0, k_c = 0)",
    ] {
        assert!(r.out.contains(line), "{line}:\n{}", r.out);
    }
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    assert_eq!(doc["schema"], "ein-test-report/5");
    let routes: Vec<(&str, &serde_json::Value)> = claim_rows(&doc)
        .iter()
        .map(|x| (x["route"].as_str().expect("a route"), &x["fell_back_from"]))
        .collect();
    assert_eq!(
        routes.iter().map(|(r, _)| *r).collect::<Vec<_>>(),
        vec!["--single", "-n 4", "--maximal -n 3", "describe"],
        "{}",
        r.out
    );
    assert!(routes.iter().all(|(_, f)| f.is_null()), "none fell back");
    // A row no run checked names no question, and carries both keys.
    let silent: Vec<&serde_json::Value> = doc["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|x| x["expect"].is_null())
        .collect();
    assert!(!silent.is_empty(), "04_open states no claim");
    for row in silent {
        let o = row.as_object().expect("a row");
        assert!(
            o["route"].is_null() && o["fell_back_from"].is_null(),
            "{row}"
        );
        assert!(o.contains_key("route") && o.contains_key("fell_back_from"));
    }
}

/// **The stage's point, on a corpus program**: a claim the least question
/// decides is held where describing R stops at the budget. I + BB = ILL at
/// a budget of 1 200 facts: `--single` shows the one state and no other
/// within it, and describe, which visits every alive partial assignment,
/// does not finish, so until S1f.6.27 the claim was NOT CHECKED, exit 1.
#[test]
fn a_claim_on_a_search_is_held_where_describing_r_stops() {
    let file = "examples/beyond_arith/cryptarithm.ein";
    let r = ein(&["test", file, "--region-budget", "1200", "-v"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("— holds (--single: all of R, 1 class over 1 states)"),
        "{}",
        r.out
    );
    // The same budget does stop the description, which `ein solve` says and
    // exits 0 on: the claim is a note there, not checked.
    let described = ein(&["solve", file, "--region-budget", "1200"]);
    assert_eq!(described.code, 0, "{}{}", described.out, described.err);
    assert!(
        described.out.contains("NOT CHECKED") && described.err.contains("note: "),
        "{}{}",
        described.out,
        described.err
    );
}

/// **What the least question leaves undecided is asked the describing one**:
/// two classes over seven states, where `-n 3` finds three states in the two
/// classes and decides nothing. The claim holds as it did, off the
/// description, and both surfaces say the runner fell back.
#[test]
fn an_undecided_claim_falls_back_to_the_description() {
    let d = Dir::new("fallback");
    let out = d.0.join("r.json");
    let summary = d.0.join("s.json");
    let r = ein(&[
        "test",
        "tests/region/08_query_classes.ein",
        "-v",
        "--json-report",
        &out.to_string_lossy(),
        "--json-summary",
        &summary.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("— holds (describe, after -n 3: all of R, 2 classes over 7 states)"),
        "{}",
        r.out
    );
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    let row = claim_rows(&doc)[0];
    assert_eq!(row["route"], "describe", "{row}");
    assert_eq!(row["fell_back_from"], "-n 3", "{row}");
    assert_eq!(row["answer"]["exact"], true, "{row}");
    // The one-run artefact is the last question asked.
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).expect("a summary")).expect("JSON");
    assert_eq!(s["asked"]["ask"], "describe", "{}", s["asked"]);

    // And where no fall-back is asked, it is the question's own run.
    let one = ein(&[
        "test",
        "examples/beyond_arith/cryptarithm.ein",
        "--json-summary",
        &summary.to_string_lossy(),
    ]);
    assert_eq!(one.code, 0, "{}{}", one.out, one.err);
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).expect("a summary")).expect("JSON");
    assert_eq!(s["asked"]["ask"], "single", "{}", s["asked"]);
}

/// **A state found refutes a claim that does not list its class**, whatever
/// the run left unvisited (Q-M1f.222): one class claimed where R's seven
/// states fall in two. `--single` stops at its second state, and the first,
/// ∅, gives `(model)`, which the claim does not list. The lines say that the
/// run did not exhaust R, so they are what refutes and not every difference.
#[test]
fn a_state_found_refutes_a_claim_that_does_not_list_its_class() {
    let d = Dir::new("refuted");
    let f = d.file(
        "one_of_two.ein",
        &format!(
            "{}(query :goal (a-prop ?x) :expect (model (a-prop X)))\n",
            seven_states()
        ),
    );
    let out = d.0.join("r.json");
    let r = ein(&["test", &f, "-v", "--json-report", &out.to_string_lossy()]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    for line in [
        "— FAILED (--single: all of R, 2 classes over 2 states)",
        "the claim lists 1 class, and all of R has a class it does not list: (model)",
        "the run asked whether it is one state (--single) and did not exhaust all of R: what it \
         found refutes the claim, and is not every difference",
    ] {
        assert!(r.out.contains(line), "{line}:\n{}", r.out);
    }
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    let row = claim_rows(&doc)[0];
    assert_eq!(row["outcome"], "failed", "{row}");
    assert_eq!(row["route"], "--single", "{row}");
    assert_eq!(row["answer"]["exact"], false, "a lower bound: {row}");

    // `ein solve` under the same question keeps its note and its exit 0.
    let solved = ein(&["solve", &f, "--single"]);
    assert_eq!(solved.code, 0, "{}{}", solved.out, solved.err);
    assert!(
        solved.out.contains("NOT CHECKED") && solved.err.starts_with("note: "),
        "{}{}",
        solved.out,
        solved.err
    );
    // And the description fails the claim with every difference, as before.
    let described = ein(&["solve", &f]);
    assert_eq!(described.code, 1, "{}{}", described.out, described.err);
}

/// **A claim still not checked says what was asked first, and why that did
/// not decide** (T1f.6.27.4): the describing run's line as it was, and one
/// under it. Exit 1, as before.
#[test]
fn a_claim_still_not_checked_says_what_was_asked_first() {
    // The question stopped at the budget, and so did the description.
    let stopped = ein(&[
        "test",
        "examples/features/11_expect_ambiguity.ein",
        "--region-budget",
        "1",
    ]);
    assert_eq!(stopped.code, 1, "{}{}", stopped.out, stopped.err);
    for line in [
        "the claim is about the maximal line, and this run did not exhaust it",
        "asked first: --maximal -n 3, which stopped at --region-budget",
    ] {
        assert!(stopped.out.contains(line), "{line}:\n{}", stopped.out);
    }
    // The question ran to its end and found only states in the classes
    // listed; the description then stopped at the budget.
    let d = Dir::new("asked-first");
    let out = d.0.join("r.json");
    let within = ein(&[
        "test",
        "tests/region/08_query_classes.ein",
        "--region-budget",
        "4",
        "--json-report",
        &out.to_string_lossy(),
    ]);
    assert_eq!(within.code, 1, "{}{}", within.out, within.err);
    for line in [
        "the claim is about all of R, and this run did not exhaust it",
        "asked first: -n 3, which found 3 states, each in a class the claim lists, and did not \
         exhaust what the claim is about",
    ] {
        assert!(within.out.contains(line), "{line}:\n{}", within.out);
    }
    let doc: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("a report")).expect("JSON");
    let row = claim_rows(&doc)[0];
    assert_eq!(row["outcome"], "not-checked", "{row}");
    assert_eq!(row["route"], "describe", "{row}");
    assert_eq!(row["fell_back_from"], "-n 3", "{row}");
}

/// **`(solve … :expect (false))` is refuted at root**: it says every node is
/// dead, so an alive root refutes it whether or not the description
/// exhausts. Exhausted, the line says what R holds, as before; stopped at
/// the budget it was NOT CHECKED until S1f.6.27 and is FAILED, at root.
#[test]
fn the_regions_false_is_refuted_at_an_alive_root() {
    let d = Dir::new("false-at-root");
    let body = seven_states();
    let at = body.find("(solve").expect("its (solve …)");
    // The form ends `…(c-prop Item)))`: the claim goes before its last `)`.
    let close = at + body[at..].find(")))").expect("the form's end") + 2;
    let f = d.file(
        "alive.ein",
        &format!("{} :expect (false){}", &body[..close], &body[close..]),
    );
    let whole = ein(&["test", &f]);
    assert_eq!(whole.code, 1, "{}{}", whole.out, whole.err);
    assert!(
        whole
            .out
            .contains("(false) says every node is dead, and R has 7 states"),
        "{}",
        whole.out
    );
    let stopped = ein(&["test", &f, "--region-budget", "1"]);
    assert_eq!(stopped.code, 1, "{}{}", stopped.out, stopped.err);
    assert!(
        stopped.summary().contains("1 FAILED, 0 not checked"),
        "{}",
        stopped.out
    );
    assert!(
        stopped
            .out
            .contains("(false) says every node is dead, and root is alive"),
        "{}",
        stopped.out
    );
}

/// **The pages name the schema the binary writes.** `--json-report`'s
/// version is said on two kernel pages and in the option's help; until M1f
/// S1f.6.27 nothing held them to the report, and `defined_behaviour.md` said
/// `/2` while the binary wrote `/4`. Any other version one of them names is
/// one it says came before.
#[test]
fn the_pages_name_the_reports_schema() {
    let d = Dir::new("report-schema");
    let doc = report(&["examples/features/10_expect.ein"], &d.0.join("r.json"));
    let schema = doc["schema"].as_str().expect("a schema").to_string();
    let (name, version) = schema.split_once('/').expect("name/version");
    let version: u32 = version.parse().expect("a number");
    for page in [
        "docs/kernel/defined_behaviour.md",
        "docs/kernel/configuration.md",
    ] {
        let text = std::fs::read_to_string(repo_root().join(page)).expect("the page");
        assert!(
            text.contains(&format!("`{schema}`")),
            "{page} does not name {schema}"
        );
        for later in version + 1..version + 4 {
            assert!(
                !text.contains(&format!("{name}/{later}")),
                "{page} names {name}/{later}, past the binary's {schema}"
            );
        }
    }
    let help = ein(&["test", "--help"]);
    assert!(help.out.contains(&schema), "--help: {}", help.out);
}
