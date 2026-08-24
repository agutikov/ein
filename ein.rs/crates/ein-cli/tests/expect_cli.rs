//! `:expect` through the CLI — M1c
//! [S1c.1.2](../../../../plans/m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md).
//!
//! The comparison itself is `ein-infer`'s `expect_semantics`; what is here is
//! the part a program can only observe from outside: **the exit code**, and the
//! promise that a query carrying `:expect` under plain `solve` is either
//! checked or refused, and never ignored.
//!
//! A checker that reports success on a broken expectation is the worst outcome
//! this phase has available to it, so the fixtures that must *fail* are checked
//! for failing — with the right code and the right message — and not merely
//! run. The [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer)
//! precedent: the fuzzer's own three controls each failed once first.

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

/// A scratch `.ein` the test owns and deletes.
struct Fixture(PathBuf);

impl Fixture {
    fn new(tag: &str, body: &str) -> Fixture {
        let dir = std::env::temp_dir().join(format!("ein-expect-cli-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let path = dir.join(format!("{tag}.ein"));
        std::fs::write(&path, body).expect("writes");
        Fixture(path)
    }

    fn path(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

const BODY: &str = "(relation p Thing Place)\n(p A H1)\n(p B H2)\n";

// ── The corpus fixture, which must hold ────────────────────────────

#[test]
fn the_feature_fixture_holds_and_says_so() {
    let r = ein(&["solve", "examples/features/10_expect.ein"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains(":expect"), "{}", r.out);
    assert!(r.out.contains("holds"), "{}", r.out);
}

// ── A false claim is exit 1, not a note ────────────────────────────

#[test]
fn a_false_expectation_exits_one_and_names_the_fact() {
    let f = Fixture::new(
        "surplus",
        &format!("{BODY}(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))\n"),
    );
    let r = ein(&["solve", &f.path()]);
    assert_eq!(r.code, 1, "a false claim is a failure: {}{}", r.out, r.err);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(r.out.contains("(p B H2)"), "the surplus fact: {}", r.out);
}

#[test]
fn a_true_expectation_exits_zero() {
    let f = Fixture::new(
        "holds",
        &format!(
            "{BODY}(query :goal (p A ?h) :no-hypothesis (p) \
             :expect (model (p A H1) (p B H2)))\n"
        ),
    );
    let r = ein(&["solve", &f.path()]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains("holds"), "{}", r.out);
}

/// The four load-time refusals, each with its message checked in beside it.
/// This asserts the CLI reaches them at all — the messages themselves are the
/// `load-negative` corpus group's.
#[test]
fn the_load_negatives_are_refused_with_their_banked_message() {
    for name in [
        "expect_unknown_keyword",
        "expect_unknown_relation",
        "expect_omits_the_goal",
        "expect_is_a_pattern",
    ] {
        let ein_path = format!("examples/broken/load/{name}.ein");
        let want = std::fs::read_to_string(
            repo_root().join(format!("examples/broken/load/{name}.expected")),
        )
        .expect("a .expected beside the fixture");
        let r = ein(&["solve", &ein_path]);
        assert_eq!(r.code, 1, "{name}: {}{}", r.out, r.err);
        assert!(
            r.err.contains(want.trim()),
            "{name}: wanted {:?} in {:?}",
            want.trim(),
            r.err
        );
    }
}

// ── Several queries in one file ────────────────────────────────────

/// Both queries run, and the second's failure is what sets the exit code —
/// the discard this stage removed would have made this file exit 0.
#[test]
fn every_query_runs_and_any_failure_sets_the_code() {
    let f = Fixture::new(
        "two",
        &format!(
            "{BODY}\
             (query :goal (p A ?h) :no-hypothesis (p) \
              :expect (model (p A H1) (p B H2)))\n\
             (query :goal (p B ?h) :no-hypothesis (p) \
              :expect (model (p A H1) (p B H3)))\n"
        ),
    );
    let r = ein(&["solve", &f.path()]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("query 1 of 2"), "{}", r.out);
    assert!(r.out.contains("query 2 of 2"), "{}", r.out);
    assert!(r.out.contains("holds"), "the first one holds: {}", r.out);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(r.out.contains("(p B H3)"), "{}", r.out);
}

/// One path cannot hold two runs, so an artefact flag over a two-question file
/// is refused rather than silently overwritten — the same reasoning that
/// removed the last-query-wins discard in the first place.
#[test]
fn an_artefact_flag_over_several_queries_is_refused() {
    let f = Fixture::new(
        "artefact",
        &format!(
            "{BODY}\
             (query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H2)))\n\
             (query :goal (p B ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H2)))\n"
        ),
    );
    let out = std::env::temp_dir().join(format!("ein-expect-cli-{}.jsonl", std::process::id()));
    let r = ein(&["solve", &f.path(), "--events", &out.to_string_lossy()]);
    assert_eq!(r.code, 2, "a usage refusal: {}{}", r.out, r.err);
    assert!(r.err.contains("--events names one path"), "{}", r.err);
    assert!(!out.exists(), "and nothing was written");
    let _ = std::fs::remove_file(&out);
}

/// A single-query file is untouched by any of it: the flags still work, which
/// is the whole corpus.
#[test]
fn one_query_keeps_its_artefacts() {
    let out = std::env::temp_dir().join(format!("ein-expect-one-{}.jsonl", std::process::id()));
    let r = ein(&[
        "solve",
        "examples/features/10_expect.ein",
        "--events",
        &out.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(out.exists(), "the event log was written");
    let _ = std::fs::remove_file(&out);
}

// ── Dumps and round-trips like every other keyword ─────────────────

/// A `.einb` stores its program as **canonical text** and re-parses it on
/// open, so opening one is a `dump_canonical → parse` round-trip of the whole
/// file. Running the container has to give the same answer as running the
/// source — which is what says `:expect` dumps, and that the container honours
/// the query index rather than always rebuilding about the first block.
#[cfg(feature = "einb")]
#[test]
fn the_container_round_trips_expect_and_the_query_index() {
    let f = Fixture::new(
        "einb",
        &format!(
            "{BODY}\
             (query :goal (p A ?h) :no-hypothesis (p) \
              :expect (model (p A H1) (p B H2)))\n\
             (query :goal (p B ?h) :no-hypothesis (p) \
              :expect (model (p A H1) (p B H3)))\n"
        ),
    );
    let container = std::env::temp_dir().join(format!("ein-expect-{}.einb", std::process::id()));
    let out = container.to_string_lossy().into_owned();
    let saved = ein(&["kb", "save", &f.path(), &out]);
    assert_eq!(saved.code, 0, "{}{}", saved.out, saved.err);

    let source = ein(&["solve", &f.path()]);
    let binary = ein(&["solve", &out]);
    assert_eq!(binary.code, source.code, "{}{}", binary.out, binary.err);
    assert_eq!(binary.code, 1, "query 2 is deliberately wrong");
    // Both queries reached the container's reader: query 1 holds and query 2
    // does not, which a reader stuck on block 1 could not produce.
    assert!(binary.out.contains("query 2 of 2"), "{}", binary.out);
    assert!(binary.out.contains("holds"), "{}", binary.out);
    assert!(binary.out.contains("(p B H3)"), "{}", binary.out);
    let _ = std::fs::remove_file(&container);
}
