//! `:expect` through the CLI — M1c
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects).
//!
//! The comparison itself is `ein-infer`'s `expect_semantics` (a query's
//! classes) and `claim_round_trip` (the region claim); what is here is the part
//! a program can only observe from outside: **the exit code**, and the promise
//! that a claim under plain `solve` is checked, or noted as not checked, and
//! never ignored. Since M1e2 S1e2.1a.5 a query's `:expect` lists its
//! projection classes, and a claim about the whole state is the solve form's,
//! `(solve … :expect (region …))`.
//!
//! A checker that reports success on a broken expectation is the worst outcome
//! this phase has available to it, so the fixtures that must *fail* are checked
//! for failing — with the right code and the right message — and not merely
//! run. The [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-fuzzer)
//! precedent: the fuzzer's own three controls each failed once first.

use std::path::PathBuf;
use std::process::Command;

use ein_corpus::Scratch;

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

/// A scratch `.ein` the test owns, in a directory of its own that goes with
/// it ([`Scratch`]), beside whatever else the test writes. Its `Drop` used to
/// remove the file and leave the directory (M1e2 S1e2.3.5).
struct Fixture {
    dir: Scratch,
    path: PathBuf,
}

impl Fixture {
    fn new(tag: &str, body: &str) -> Fixture {
        let dir = Scratch::new("ein-expect-cli");
        let path = dir.join(format!("{tag}.ein"));
        std::fs::write(&path, body).expect("writes");
        Fixture { dir, path }
    }

    fn path(&self) -> String {
        self.path.to_string_lossy().into_owned()
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

/// Both claim forms: a query's that lists a class the query does not have,
/// and a region claim whose model misses a fact the state holds, which it
/// names — naming a relation closes it.
#[test]
fn a_false_expectation_exits_one_and_names_the_fact() {
    let q = Fixture::new(
        "wrong-class",
        &format!(
            "{BODY}(solve :no-hypothesis (p)) (query :goal (p A ?h) :expect (model (p A H2)))\n"
        ),
    );
    let r = ein(&["solve", &q.path()]);
    assert_eq!(r.code, 1, "a false claim is a failure: {}{}", r.out, r.err);
    assert!(r.out.contains(":expect           FAILED"), "{}", r.out);
    assert!(
        r.out
            .contains("(model (p A H1)) is a class the claim does not list"),
        "{}",
        r.out
    );
    assert!(r.err.contains("query :expect FAILED"), "{}", r.err);

    let s = Fixture::new(
        "surplus",
        &format!("{BODY}(solve :no-hypothesis (p) :expect (region (model (p A H1))))\n"),
    );
    let r = ein(&["solve", &s.path()]);
    assert_eq!(r.code, 1, "a false claim is a failure: {}{}", r.out, r.err);
    assert!(r.out.contains("(solve :expect)   FAILED"), "{}", r.out);
    assert!(
        r.out
            .contains("the state holds (p B H2), which the model does not list"),
        "the surplus fact: {}",
        r.out
    );
}

#[test]
fn a_true_expectation_exits_zero() {
    let f = Fixture::new(
        "holds",
        &format!(
            "{BODY}(solve :no-hypothesis (p) :expect (region (model (p A H1) (p B H2))))\n\
             (query :goal (p A ?h) :expect (model (p A H1)))\n"
        ),
    );
    let r = ein(&["solve", &f.path()]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains(":expect           holds"), "{}", r.out);
    assert!(r.out.contains("(solve :expect)   holds"), "{}", r.out);
}

/// The four refusals of a malformed `(query …)`, each with its message
/// checked in beside it. This asserts the CLI reaches them at all — the
/// messages themselves are the `load-negative` and `parse-negative` corpus
/// groups'. Since M1f S1f.5.5's swap two are the grammar's (an unknown
/// keyword, S1f.5.11; a variable in a claim's fact, S1f.5.2's G-6), refused at
/// the parse and moved to `examples/broken/` (Q-M1f.2 (a)), their `.expected`
/// the parse report with the file written `{FILE}`; the other two stay the
/// loader's.
#[test]
fn the_load_negatives_are_refused_with_their_banked_message() {
    for (dir, name) in [
        ("examples/broken", "expect_unknown_keyword"),
        ("examples/broken/load", "expect_unknown_relation"),
        ("examples/broken/load", "expect_omits_the_goal"),
        ("examples/broken", "expect_is_a_pattern"),
    ] {
        let ein_path = format!("{dir}/{name}.ein");
        let want = std::fs::read_to_string(repo_root().join(format!("{dir}/{name}.expected")))
            .expect("a .expected beside the fixture")
            .replace("{FILE}", &ein_path);
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

/// **`NOT CHECKED` is a note under `solve` and a failure under `ein test`**
/// — M1e2 S1e2.1a.5's split, on the corpus's own fixture.
///
/// `examples/features/11_expect_ambiguity.ein` claims the maximal line's two
/// classes. Asked for one state, `ein solve -n 1` did not decide that: it
/// answers what it was asked, says on stderr — one line, naming the file —
/// that the claim went unchecked, and exits 0, because an answer is not an
/// error. `ein test` runs the question the claim is about, and it holds.
/// Before the switch plain `solve` stopped at `-n 1` by default and exited 1
/// here (M1d S1d.4.3).
#[test]
fn not_checked_is_a_note_under_solve_and_decided_by_test() {
    let file = "examples/features/11_expect_ambiguity.ein";
    let r = ein(&["solve", file, "-n", "1"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains(":expect           NOT CHECKED"), "{}", r.out);
    assert!(
        r.out
            .contains("the claim is about the maximal line, and this run asked 1 of its states"),
        "it says why the claim went unchecked: {}",
        r.out
    );
    assert!(
        r.err.starts_with("note: ")
            && r.err.contains("NOT CHECKED")
            && r.err.contains("11_expect_ambiguity.ein"),
        "{:?}",
        r.err
    );
    assert_eq!(
        r.err.lines().count(),
        1,
        "one line, not a second copy: {:?}",
        r.err
    );

    let plain = ein(&["solve", file]);
    assert_eq!(plain.code, 0, "{}{}", plain.out, plain.err);
    assert!(
        plain.out.contains(":expect           holds"),
        "{}",
        plain.out
    );
    let t = ein(&["test", file]);
    assert_eq!(t.code, 0, "{}{}", t.out, t.err);
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
             (solve :no-hypothesis (p))\n\
             (query :goal (p A ?h) :expect (model (p A H1)))\n\
             (query :goal (p B ?h) :expect (model (p B H3)))\n"
        ),
    );
    let r = ein(&["solve", &f.path()]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(r.out.contains("query 1 of 2"), "{}", r.out);
    assert!(r.out.contains("query 2 of 2"), "{}", r.out);
    assert!(r.out.contains("holds"), "the first one holds: {}", r.out);
    assert!(r.out.contains("FAILED"), "{}", r.out);
    assert!(r.out.contains("(p B H3)"), "{}", r.out);
    assert!(r.err.contains("query 2 of 2 :expect FAILED"), "{}", r.err);
}

/// **A file with several queries is one run, so one path holds it** — since
/// M1e2 S1e2.1a.5. The walk solved once per query and refused an artefact flag
/// over two of them, because one path could not hold two runs; a region run
/// computes R once and reads each query off it, so the summary's `queries`
/// carries both and the event stream is the one run's.
#[test]
fn an_artefact_flag_over_several_queries_writes_the_one_run() {
    let f = Fixture::new(
        "artefact",
        &format!(
            "{BODY}\
             (solve :no-hypothesis (p))\n\
             (query :goal (p A ?h) :expect (model (p A H1)))\n\
             (query :goal (p B ?h) :expect (model (p B H2)))\n"
        ),
    );
    let out = f.dir.join("summary.json");
    let r = ein(&["solve", &f.path(), "--json-summary", &out.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    let j: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).expect("the summary")).unwrap();
    let queries = j["queries"].as_array().expect("queries");
    assert_eq!(queries.len(), 2);
    assert!(
        queries.iter().all(|q| q["expect"]["outcome"] == "held"),
        "{j}"
    );
}

/// A single-query file is untouched by any of it: the flags still work, which
/// is the whole corpus.
#[test]
fn one_query_keeps_its_artefacts() {
    let dir = Scratch::new("ein-expect-one");
    let out = dir.join("events.jsonl");
    let r = ein(&[
        "solve",
        "examples/features/10_expect.ein",
        "--events",
        &out.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(out.exists(), "the event log was written");
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
             (solve :no-hypothesis (p))\n\
             (query :goal (p A ?h) :expect (model (p A H1)))\n\
             (query :goal (p B ?h) :expect (model (p B H3)))\n"
        ),
    );
    let container = f.dir.join("einb.einb");
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
}
