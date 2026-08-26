//! T1a.10.2.2 — **the `ein` binary as a tool**: exit codes, flag effects, the
//! `--events` protocol, and the shape of the three canonical Zebra files.
//!
//! Replaces four Python files, whose common subject is *the surface a caller
//! sees* rather than what the engine concluded:
//!
//! | Python original | what it owned |
//! |---|---|
//! | `tests/test_cli.py` | how a broken file is reported by each subcommand |
//! | `tests/test_solve_cli.py` | the stop policy, the diagnostic flags, `--json-summary` |
//! | `tests/test_events.py` | `--events`, whose spec is [`events.md`](../../../../docs/kernel/inference/events.md) |
//! | `tests/integration/test_zebra_parse.py` | that `zebra2.ein` and its two variants stay one encoding |
//!
//! | `tests/test_vscode_grammar.py` | the editor grammar's three closed name sets |
//!
//! Everything here runs the built binary (`CARGO_BIN_EXE_ein`) except four
//! claims that have no command line — `Events::off()`, root saturation, the
//! id-order perturbation and the grammar itself — and those say so where they
//! sit.
//!
//! **Nothing here re-asserts byte layout.** `ein-render`'s `corpus_shapes.md5`
//! owns the exact form of every line the CLI prints; a whitespace change must
//! fail there and be silent here, or a reader learns nothing from either. What
//! is asserted instead is *which* words, numbers and files appear, and the
//! relations between them: a counter that the `--stats` block and
//! `--json-summary` report differently is a real defect, and a column that
//! moved is not.
//!
//! Two of the ported claims came with a concession that turned out to be
//! false in this engine, and the tests are stronger than their originals as a
//! result — see [`schema_kinds`], which parses the event kinds out of
//! `EVENTS.md` rather than copying them, and
//! [`the_injected_clash_is_refuted_at_root_saturation`], whose Python original
//! lived behind an `EIN_RUN_SLOW` gate.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

use ein_core::{Kb, ProvKind, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level, sexpr};
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use serde_json::Value as J;

// ── plumbing ───────────────────────────────────────────────────────

/// A run of the binary, kept whole: the three things the "additive" claims
/// compare and every other test reads one of.
struct Run {
    code: i32,
    out: String,
    err: String,
}

impl Run {
    fn ok(&self) -> &Run {
        assert_eq!(self.code, 0, "expected success, stderr was:\n{}", self.err);
        self
    }

    /// The value of a `  <label>   <value>` row — the `--stats` block's shape.
    ///
    /// By label and trimmed rather than by column, because the columns are
    /// `corpus_shapes.md5`'s to pin.
    fn field(&self, label: &str) -> Option<String> {
        self.out
            .lines()
            .find(|l| l.trim_start().starts_with(label))
            .and_then(|l| l.trim_start().strip_prefix(label))
            .map(|v| v.split_whitespace().next().unwrap_or("").to_string())
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

/// A directory the test owns and deletes, for the flags that write files.
///
/// `tempfile` is not a dependency of this crate and one file per flag does not
/// justify adding it; the pid plus the caller's tag is unique enough for a
/// suite that runs its tests as threads of one process.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let dir =
            std::env::temp_dir().join(format!("ein-cli-semantics-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Scratch(dir)
    }

    fn at(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path} was not written: {e}"))
}

fn json_at(path: &str) -> J {
    serde_json::from_str(&read(path)).expect("the summary is JSON")
}

/// One `--events` log, as parsed objects.
fn events_of(path: &str) -> Vec<J> {
    read(path)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect()
}

fn kinds(events: &[J]) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for e in events {
        *out.entry(e["e"].as_str().unwrap_or("?").to_string())
            .or_insert(0) += 1;
    }
    out
}

fn corpus(rel: &str) -> PathBuf {
    repo_root().join(rel)
}

// ── the three canonical Zebra files ────────────────────────────────
//
// From `tests/integration/test_zebra_parse.py`. The acceptance gate reads its
// GAPS answer from `zebra2-minus-15.ein` and its CONTRADICTIONS answer from
// `ein-bugs/zebra2-bad.ein`, and both are only evidence about *zebra2* while
// they stay zebra2 ± one clue. Nothing else in the suite would notice if a
// rule drifted into one of them.

/// Condition (15) — the lone fact pinning Blue at House-2, and the one the
/// GAPS fixture drops.
const COND_15: &str = "(adjacent-via next-to nation-loc Norwegian color-loc Blue)";
/// The clue the CONTRADICTIONS fixture injects.
const INJECTED: &str = "(color-loc Green House-1)";

/// The nine `(relation …)` signatures the B1 encoding declares: the five
/// typed `*-loc` bijections, the two spatial relations, and the is-a pair.
///
/// The registry also holds auto-vivified heads (`co-located`, `bijective`, …)
/// with `declared = false`; those are incidental to how a puzzle is written,
/// so the contract is on the *declared* set, as ein.py's was.
const DECLARED: [&str; 9] = [
    "color-loc",
    "drink-loc",
    "is-a",
    "is-a*",
    "nation-loc",
    "next-to",
    "pet-loc",
    "right-of",
    "smoke-loc",
];

/// What "the same encoding" means, reduced to comparable sets.
struct Shape {
    declared: BTreeSet<String>,
    rules: BTreeSet<String>,
    /// The authored conditions — every `:source`-carrying fact.
    given: BTreeSet<String>,
    /// The un-annotated facts: schema, is-a enumerations, property tags.
    background: usize,
    rules_total: usize,
    has_query: bool,
}

/// Load a corpus file and classify its facts the way `ein saturate --dump`
/// buckets them: a fact is GIVEN when its primary provenance is a `source`
/// that carries a `:source` sentence, BACKGROUND when it has no provenance or
/// an unannotated `source`, DERIVED otherwise (none, before saturation).
fn shape_of(rel: &str) -> Shape {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &corpus(rel)).expect("the fixture loads");
    let (mut given, mut background) = (BTreeSet::new(), 0usize);
    for f in kb.facts() {
        match kb.primary(f) {
            Some(p) if terms.provs.get(p).kind == ProvKind::Source => {
                if terms.provs.get(p).source.is_some() {
                    given.insert(sexpr(&terms, f));
                } else {
                    background += 1;
                }
            }
            None => background += 1,
            Some(_) => {}
        }
    }
    let p = kb.program();
    Shape {
        declared: p
            .relations
            .iter()
            .filter(|(_, r)| r.declared)
            .map(|(n, _)| terms.sym(n).to_string())
            .collect(),
        rules: p.rules.keys().map(|n| terms.sym(n).to_string()).collect(),
        given,
        background,
        rules_total: p.rules.len(),
        has_query: p.query().is_some(),
    }
}

/// The two variants are `zebra2.ein` ± exactly one `:source`d condition, and
/// identical to it in every other respect.
///
/// Asserted as a *relative* diff on purpose: an absolute golden of all three
/// files would have to be re-blessed whenever the canonical encoding gains a
/// rule, and the thing worth knowing is not what the fixtures contain but that
/// the three of them are one puzzle. The generator's own `--check` is stronger
/// still — it compares whole bytes, so it catches a drifted rule *body* where
/// the structural diff below compares only rule names — and it is run here
/// when a `python3` is available, because it lives under `examples/` and
/// outlives ein.py.
#[test]
fn the_zebra2_variants_are_zebra2_plus_or_minus_one_condition() {
    let z = shape_of("examples/zebra2.ein");
    let m = shape_of("examples/zebra2-minus-15.ein");
    let b = shape_of("examples/ein-bugs/zebra2-bad.ein");

    for (name, v) in [("zebra2-minus-15", &m), ("ein-bugs/zebra2-bad", &b)] {
        assert_eq!(v.declared, z.declared, "{name} declares other relations");
        assert_eq!(v.rules, z.rules, "{name} carries other rules");
        assert_eq!(
            v.background, z.background,
            "{name} changed a background fact, so it is not a thin diff"
        );
    }

    let dropped: Vec<&String> = z.given.difference(&m.given).collect();
    assert_eq!(dropped, [&COND_15.to_string()], "minus-15 drops only (15)");
    assert!(
        m.given.difference(&z.given).next().is_none(),
        "minus-15 must not add a condition"
    );
    let added: Vec<&String> = b.given.difference(&z.given).collect();
    assert_eq!(added, [&INJECTED.to_string()], "bad adds only the clash");
    assert!(
        z.given.difference(&b.given).next().is_none(),
        "bad must not drop a condition"
    );

    // The byte-level half. A checkout with no `python3` still runs everything
    // above; one that has it also learns whether a rule *body* drifted.
    let check = Command::new("python3")
        .arg("examples/gen_zebra2_variants.py")
        .arg("--check")
        .current_dir(repo_root())
        .output();
    match check {
        // 127 is "the interpreter could not run the script at all" — the same
        // situation as no `python3`, and it must not be reported as a stale
        // fixture. This is the one place in the workspace where a Python
        // process still runs, and after
        // [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
        // it is the *only* one: `PATH=<a python3 that exits 127> cargo test
        // --workspace` is 566 passed, and this line is why the count does not
        // drop by one.
        Ok(out) if out.status.code() == Some(127) => {
            eprintln!("skipped the generator's byte check: python3 exited 127")
        }
        Ok(out) => assert!(
            out.status.success(),
            "the on-disk variants are stale — run `python3 examples/gen_zebra2_variants.py`\n{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        ),
        Err(e) => eprintln!("skipped the generator's byte check: python3 did not run ({e})"),
    }
}

/// The landmarks of the canonical encoding: nine declared relations, a query
/// block, more than twenty rules, and eighteen authored conditions.
///
/// Eighteen rather than fifteen because condition (1) expands to four
/// `right-of` facts. The count is an anchor, not a property — its job is to
/// make a *silent* change to zebra2 loud, since the diff test above compares
/// the variants to zebra2 and would stay green if all three drifted together.
#[test]
fn the_canonical_encoding_keeps_its_landmarks() {
    for rel in [
        "examples/zebra2.ein",
        "examples/zebra2-minus-15.ein",
        "examples/ein-bugs/zebra2-bad.ein",
    ] {
        let s = shape_of(rel);
        let want: BTreeSet<String> = DECLARED.iter().map(|s| s.to_string()).collect();
        assert_eq!(s.declared, want, "{rel} declares a different B1 ontology");
        assert!(s.has_query, "{rel} has no query block to answer");
        assert!(
            s.rules_total > 20,
            "{rel} loaded only {} rules — a truncated rule library",
            s.rules_total
        );
    }
    let z = shape_of("examples/zebra2.ein");
    assert_eq!(z.given.len(), 18, "the 15 numbered conditions, (1) as four");
    assert!(
        z.given.contains(COND_15),
        "condition (15) is in the canonical"
    );
    assert!(
        !z.given.contains(INJECTED),
        "the canonical carries no clash"
    );
}

/// Root saturation, to the fixpoint, with no hypothesis at all — what `solve`
/// does before it branches.
fn root_saturated(rel: &str) -> (Terms, Kb) {
    use ein_infer::SharedMemo;
    use ein_infer::saturator::{Saturator, Session};

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &corpus(rel)).expect("the fixture loads");
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        ein_infer::emit_closed(&mut s).expect("the closed-world markers compile");
        let mut sat = Saturator::new(&mut s).expect("the rules compile");
        sat.saturate(&mut s, None, &mut |_| {})
            .expect("root saturation reaches its fixpoint");
    }
    (terms, kb)
}

/// `zebra2-bad.ein` is UNSAT at d=0: the injected clue contradicts the rules
/// before any hypothesis is made, while the canonical puzzle needs branching
/// to finish.
///
/// This is what makes the CONTRADICTIONS verdict cheap and its unsat core
/// tight — the search never has to prove unsatisfiability, it inherits it —
/// and it is the difference between the two fixtures being *a* difference and
/// being *the* one that matters. The Python original ran ~6 s and ~4 s under
/// CPython and lived behind an `EIN_RUN_SLOW` gate; here the two saturations
/// are a few milliseconds each, so the gate has no reason to exist.
#[test]
fn the_injected_clash_is_refuted_at_root_saturation() {
    let (terms, kb) = root_saturated("examples/ein-bugs/zebra2-bad.ein");
    let found = ein_infer::detect(&kb, &terms);
    let pairs: BTreeSet<String> = found
        .iter()
        .filter_map(|c| c.positive)
        .map(|f| sexpr(&terms, f))
        .collect();
    assert!(
        !found.is_empty(),
        "the injected clue no longer contradicts anything at root"
    );
    eprintln!("root contradictions on zebra2-bad: {pairs:?}");

    // The control, and the point: the shared encoding is not what breaks.
    let (terms, kb) = root_saturated("examples/zebra2.ein");
    let clean = ein_infer::detect(&kb, &terms);
    assert!(
        clean.is_empty(),
        "the canonical puzzle contradicts itself at root: {:?}",
        clean
            .iter()
            .map(|c| sexpr(&terms, c.witness()))
            .collect::<Vec<_>>()
    );
}

// ── The diagnostics — `tests/test_cli.py` ──────────────────────────

/// A file that parses and then fails to load, for the diagnostics below.
const LOAD_NEGATIVE: &str = "examples/broken/load/derivation_cycle.ein";
/// A file that does not parse.
const PARSE_NEGATIVE: &str = "examples/broken/unclosed_paren.ein";

/// **A load failure is *diagnosed*, not raised.** One line, on stderr, exit 1,
/// and no traceback.
///
/// The claim is about the boundary between an engine error and a user error:
/// `examples/broken/load/` is a corpus of *inputs a person can write*, and a
/// stack trace for one of them says the tool broke rather than that the file
/// did. Both engine subcommands go through the same path, which is why both
/// are asserted — `saturate` prints its own progress header first and then
/// fails identically.
#[test]
fn a_load_failure_is_one_diagnostic_line_and_a_non_zero_exit() {
    for cmd in ["solve", "saturate"] {
        let r = ein(&[cmd, LOAD_NEGATIVE]);
        assert_eq!(r.code, 1, "{cmd}: exit code");
        let lines: Vec<&str> = r.err.lines().filter(|l| !l.trim().is_empty()).collect();
        assert_eq!(
            lines.len(),
            1,
            "{cmd}: expected one diagnostic, got:\n{}",
            r.err
        );
        assert!(
            lines[0].starts_with("kb load error: "),
            "{cmd}: {}",
            lines[0]
        );
        assert!(
            lines[0].contains("derivation cycle"),
            "{cmd}: the message lost its reason: {}",
            lines[0]
        );
        for trace in ["panicked", "RUST_BACKTRACE", "Traceback"] {
            assert!(!r.err.contains(trace), "{cmd}: a {trace} reached the user");
        }
    }
}

/// **`render` views the IR, not the KB** — so it never reports a load error.
///
/// The rule views render the *parsed forms*; nothing builds a `KnowledgeBase`,
/// and a file that would be refused by the loader still renders whatever rules
/// it declares. On the load-negative fixture, which declares none, the
/// diagnostic is about the missing rules and not about the cycle — and that
/// difference is the claim: a `kb load error` here would mean the renderer had
/// quietly acquired a dependency on the loader.
#[test]
fn render_views_the_ir_and_never_reports_a_load_error() {
    let r = ein(&["render", "rules", LOAD_NEGATIVE]);
    assert_eq!(r.code, 1);
    assert!(
        r.err.contains(&format!("no rule forms in {LOAD_NEGATIVE}")),
        "the diagnostic is not about the missing rules: {}",
        r.err
    );
    assert!(
        !r.err.contains("kb load error") && !r.out.contains("kb load error"),
        "the renderer built a KB: {}{}",
        r.out,
        r.err
    );
    // The control: a file that *does* declare rules renders them, load-worthy
    // or not, so the exit above is about the file's contents.
    let ok = ein(&["render", "rules", "examples/branching/04_two_levels.ein"]);
    ok.ok();
    assert!(ok.out.contains("digraph"), "no DOT came out");
}

/// **A parse error names the file it came from** — every subcommand.
///
/// The regression is specific: `saturate` used to parse without passing
/// `filename=`, so every location in its diagnostics read `<string>` and no
/// editor could jump to one. The path on the command line is the only name the
/// user gave, so it is the only name the diagnostic may use.
#[test]
fn a_parse_error_names_the_file_from_every_subcommand() {
    for args in [
        vec!["solve", PARSE_NEGATIVE],
        vec!["saturate", PARSE_NEGATIVE],
        vec!["render", "rules", PARSE_NEGATIVE],
    ] {
        let r = ein(&args);
        assert_ne!(r.code, 0, "{args:?}: a broken file parsed");
        assert!(
            r.err.contains(PARSE_NEGATIVE),
            "{args:?}: the diagnostic does not name the file:\n{}",
            r.err
        );
        assert!(
            !r.err.contains("<string>"),
            "{args:?}: the file became <string>:\n{}",
            r.err
        );
    }
}

// ── The stop policy and the diagnostic flags — `test_solve_cli.py` ─

/// The fixture the stop-policy claims are made on: two genuine models, so
/// "stop at one" and "exhaust" have different answers.
const TWO_MODELS: &str = "examples/branching/04_two_levels.ein";

/// **The stop policy is honoured**, and the three settings differ.
///
/// Default is "one model, and do not claim it is the only one":
/// `exhausted false` is what keeps `k = 1` from reading as uniqueness.
/// `--exhaustive` runs the lattice out and reports both models. `--solutions
/// N` is the middle setting, and on a two-model puzzle `-n 2` reaches the
/// same `k` as `-e` while still not certifying it — which is the distinction
/// a caller most often gets wrong.
#[test]
fn the_stop_policy_is_honoured() {
    let first = ein(&["solve", TWO_MODELS, "-s"]);
    first.ok();
    assert_eq!(first.field("solutions (k)").as_deref(), Some("1"));
    assert_eq!(first.field("exhausted").as_deref(), Some("false"));

    let exhaustive = ein(&["solve", TWO_MODELS, "-e", "-s"]);
    exhaustive.ok();
    assert_eq!(exhaustive.field("solutions (k)").as_deref(), Some("2"));
    assert_eq!(exhaustive.field("exhausted").as_deref(), Some("true"));
    assert!(
        exhaustive.out.to_lowercase().contains("ambiguous"),
        "the verdict word did not reach stdout:\n{}",
        exhaustive.out
    );

    let n2 = ein(&["solve", TWO_MODELS, "-n", "2", "-s"]);
    n2.ok();
    assert_eq!(n2.field("solutions (k)").as_deref(), Some("2"));
    assert_eq!(
        n2.field("exhausted").as_deref(),
        Some("false"),
        "-n 2 certified a puzzle it did not exhaust"
    );
    // The short and long spellings are the same flag. Compared with the `wall`
    // row dropped: it is wall-clock, and it is the one line in the block that
    // differs run to run for reasons that have nothing to do with the flag.
    let long = ein(&["solve", TWO_MODELS, "--solutions", "2", "--stats"]);
    let without_wall = |r: &Run| -> String {
        r.out
            .lines()
            .filter(|l| !l.trim_start().starts_with("wall"))
            .collect::<Vec<&str>>()
            .join("\n")
    };
    assert_eq!(
        without_wall(&long),
        without_wall(&n2),
        "-n/-s and --solutions/--stats differ"
    );
}

/// **`-j/--jobs` takes a count or the word `auto`, and refuses everything
/// else** — [S1a.7.5](../../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)
/// T1a.7.5.1.
///
/// `auto` is [`std::thread::available_parallelism`], so the test cannot assert
/// a number; what it asserts is that the flag resolves to *something the block
/// reports* and that the answer is the default run's. `--jobs 0` is refused on
/// purpose: it is the sentinel `auto` parses to, and letting a user type it
/// would give one meaning two spellings — the more so because "0 threads"
/// reads as "none" at least as often as "all of them".
#[test]
fn jobs_takes_a_count_or_auto_and_nothing_else() {
    let one = ein(&["solve", TWO_MODELS, "-e"]);
    one.ok();
    for spec in ["2", "auto"] {
        let r = ein(&["solve", TWO_MODELS, "-e", "--jobs", spec, "--stats"]);
        r.ok();
        assert_eq!(
            r.out.contains("verdict"),
            one.out.contains("verdict"),
            "--jobs {spec} changed the answer"
        );
        // The block exists only above one job, and it names what was asked.
        assert!(
            r.out.contains("\njobs\n"),
            "--jobs {spec} printed no jobs block:\n{}",
            r.out
        );
        let asked: usize = r
            .out
            .lines()
            .find_map(|l| l.trim().strip_prefix("workers"))
            .and_then(|l| l.split("of ").nth(1))
            .and_then(|l| l.split_whitespace().next())
            .and_then(|n| n.parse().ok())
            .expect("the workers row names the job count");
        assert!(asked >= 2, "--jobs {spec} resolved to {asked}");
    }
    // …and the default prints no block at all, which is what keeps every
    // `--stats` run in the repo byte-identical to what it was.
    let plain = ein(&["solve", TWO_MODELS, "-e", "--stats"]);
    plain.ok();
    assert!(
        !plain.out.contains("\njobs\n"),
        "--jobs 1 printed a jobs block"
    );

    for bad in ["0", "xyz", "1.5"] {
        let r = ein(&["solve", TWO_MODELS, "--jobs", bad]);
        assert_ne!(r.code, 0, "--jobs {bad} was accepted");
        assert!(
            r.err.contains("job count"),
            "--jobs {bad} was refused without saying why:\n{}",
            r.err
        );
    }
}

/// **The `--stats` block reports the engine's counters**, and they are the
/// same numbers `--json-summary` writes.
///
/// Two renderings of one counter set is exactly where a drift hides: a block
/// that reads a stale copy, or a summary assembled from a different snapshot,
/// is invisible in either alone. What is compared is the *values*, by label —
/// the block's column layout belongs to `corpus_shapes.md5`.
#[test]
fn the_stats_block_reports_the_same_counters_as_the_json_summary() {
    let scratch = Scratch::new("stats");
    let path = scratch.at("summary.json");
    let r = ein(&["solve", TWO_MODELS, "-e", "-s", "--json-summary", &path]);
    r.ok();
    let j = json_at(&path);

    assert_eq!(
        r.field("solutions (k)").as_deref(),
        Some(j["stats"]["solution_nodes"].as_i64().unwrap().to_string()).as_deref()
    );
    assert_eq!(
        r.field("exhausted").as_deref(),
        Some(j["stats"]["exhausted"].as_bool().unwrap().to_string()).as_deref()
    );
    for (label, key) in [
        ("layers_explored", "layers_explored"),
        ("saturate_count", "saturate_count"),
    ] {
        assert_eq!(
            r.field(label).as_deref(),
            Some(j["stats"][key].as_i64().unwrap().to_string()).as_deref(),
            "{label} disagrees"
        );
    }
    // The entering breakdown is one row: `36 (alive=20 dead_pre=0 dead_post=16)`.
    let enterings = r
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("enterings"))
        .expect("the enterings row")
        .to_string();
    for (key, label) in [
        ("enterings_alive", "alive"),
        ("enterings_dead_pre", "dead_pre"),
        ("enterings_dead_post", "dead_post"),
    ] {
        let want = format!("{label}={}", j["stats"][key].as_i64().unwrap());
        assert!(enterings.contains(&want), "{want} missing from {enterings}");
    }
    assert!(
        enterings.contains(&j["stats"]["enterings_total"].as_i64().unwrap().to_string()),
        "the total is missing from {enterings}"
    );
    let nogoods = r
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("nogoods"))
        .expect("the nogoods row")
        .to_string();
    for (key, label) in [
        ("nogoods_emitted", "emitted"),
        ("nogoods_subsumed", "subsumed"),
    ] {
        let want = format!("{label}={}", j["stats"][key].as_i64().unwrap());
        assert!(nogoods.contains(&want), "{want} missing from {nogoods}");
    }
    assert!(r.field("wall").is_some(), "no wall-clock row");
}

/// **`--timing` covers every phase.** No cost may vanish from the accounting.
///
/// The table is the only place a caller can see *where* a slow run went, and
/// its value is entirely in being exhaustive: a phase that stopped being
/// reported does not look slow, it looks free. The eight names are the
/// flag's own help text, which is the contract.
#[test]
fn the_timing_table_names_every_phase() {
    let r = ein(&["solve", TWO_MODELS, "-e", "--timing"]);
    r.ok();
    let table = r
        .out
        .split_once("timing (ms)")
        .expect("no timing table in the output")
        .1;
    for phase in [
        "parse",
        "kb load",
        "compile",
        "root saturation",
        "hypothesis search",
        "per hypothesis",
        "solve",
        "end-to-end",
    ] {
        assert!(
            table.lines().any(|l| l.trim_start().starts_with(phase)),
            "the {phase:?} phase is missing from:\n{table}"
        );
    }
    assert!(
        !r.out.contains("timing (ms)\n\n"),
        "the table header has no rows under it"
    );
}

/// **The trace goes to a file and the answer to stdout.** They do not mix.
///
/// `--trace FILE` is a *file* flag, and the reason is redirection: the trace
/// is a markdown document with fenced DOT blocks in it, and a caller piping
/// `ein solve` into anything at all would otherwise get the document instead
/// of the table. So stdout keeps the solve table and carries no fence.
#[test]
fn the_trace_goes_to_a_file_and_the_answer_to_stdout() {
    let scratch = Scratch::new("trace");
    let path = scratch.at("trace.md");
    let r = ein(&["solve", TWO_MODELS, "-e", "--trace", &path]);
    r.ok();
    let trace = read(&path);
    assert!(trace.contains("```"), "the trace has no fenced block");
    assert!(
        trace.len() > 200,
        "the trace is {} bytes — it is not a document",
        trace.len()
    );
    assert!(!r.out.contains("```"), "a fence reached stdout:\n{}", r.out);
    assert!(
        r.out.contains("solutions (k)"),
        "stdout lost the solve table:\n{}",
        r.out
    );
}

// ── `--json-summary` — `test_solve_cli.py` ─────────────────────────

/// **The summary declares its schema, and its blocks have a fixed shape.**
///
/// One object per run, and `schema` first: a consumer that reads
/// `ein-summary/1` knows what the rest means, and one that reads something
/// else can refuse instead of guessing. `root.hypgen`'s four keys are the
/// invariant behind them — `raw == emitted + Σ filtered` — so a fifth key or a
/// missing one changes what the block *means*, not just what it contains.
#[test]
fn the_json_summary_declares_its_schema_and_its_block_shape() {
    let scratch = Scratch::new("schema");
    let path = scratch.at("summary.json");
    ein(&["solve", TWO_MODELS, "-e", "--json-summary", &path]).ok();
    let j = json_at(&path);
    assert_eq!(j["schema"], "ein-summary/1");
    assert_eq!(j["source"], TWO_MODELS);
    for block in ["verdict", "stats", "root", "config"] {
        assert!(j[block].is_object(), "the {block} block is missing");
    }
    let hypgen = j["root"]["hypgen"]
        .as_object()
        .expect("the hypgen block")
        .keys()
        .cloned()
        .collect::<BTreeSet<String>>();
    assert_eq!(
        hypgen,
        ["emitted", "filtered", "pre_candidate", "raw"]
            .iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<String>>(),
        "root.hypgen's key set moved"
    );
    assert!(
        j["root"]["plans"].as_i64().unwrap_or(0) > 0,
        "a file with rules compiled no plans"
    );
    assert_eq!(j["verdict"]["type"], "Ambiguity");
}

/// **An integer goal binding stays a JSON *number*** — at any width.
///
/// A goal binding is the *stored* argument, so an IR `INT` is an integer and
/// writing it as `"8"` is a type error a consumer cannot undo. S1a.6.6's
/// fuzzer found this from the other side, on a two-line program, after five
/// phases of byte parity had not: stdout was identical on both engines, so
/// only the summary showed it. The wide literal is the second half — the IR's
/// `INT` is unbounded, so the value has to be carried as an exact numeric
/// literal rather than clamped to an `i64`.
#[test]
fn an_integer_goal_binding_stays_a_json_number() {
    let scratch = Scratch::new("int-binding");
    let path = scratch.at("summary.json");
    ein(&[
        "solve",
        "examples/ein-bugs/int-goal-binding.ein",
        "--json-summary",
        &path,
    ])
    .ok();
    let text = read(&path);
    let j: J = serde_json::from_str(&text).expect("the summary is JSON");
    let rows = j["verdict"]["solutions"][0]["goal_bindings"]
        .as_array()
        .expect("the goal bindings");
    let bound: BTreeMap<String, String> = rows
        .iter()
        .map(|r| {
            let x = r["x"].as_str().expect("?x is an atom").to_string();
            assert!(
                r["y"].is_number() && !r["y"].is_string(),
                "?y = {} is not a number",
                r["y"]
            );
            (x, r["y"].to_string())
        })
        .collect();
    assert_eq!(bound["o3"], "8");
    assert_eq!(bound["o4"], "-7");
    assert_eq!(bound.len(), 3, "a binding went missing: {bound:?}");

    // The wide one is checked against the **bytes**, because a `serde_json`
    // built without `arbitrary_precision` — this workspace's — reads it back
    // as an `f64` and hands out `1e23`. That is the reader's limit, not the
    // writer's, and what the claim is about is what was written: the IR's
    // `INT` is unbounded, so the summary carries the digits.
    assert!(
        text.contains(r#""y": 99999999999999999999999"#),
        "the wide literal was clamped or quoted:\n{text}"
    );
}

/// **A nested-fact goal binding renders as an s-expression**, and the summary
/// is still written.
///
/// The sibling of the integer case, and the one where **ein.py was the engine
/// that was wrong**: `json.dumps` cannot serialise a `Fact`, so the oracle
/// raised and produced no summary at all. A crash is not a semantics worth
/// preserving, so ein.py was changed to match ein.rs — which is worth
/// recording here, because it is the only place in the port where the arrow
/// pointed that way.
#[test]
fn a_nested_fact_goal_binding_renders_as_an_s_expression() {
    let scratch = Scratch::new("fact-binding");
    let path = scratch.at("summary.json");
    let r = ein(&[
        "solve",
        "examples/ein-bugs/fact-goal-binding.ein",
        "--json-summary",
        &path,
    ]);
    r.ok();
    let j = json_at(&path);
    let rows = j["verdict"]["solutions"][0]["goal_bindings"]
        .as_array()
        .expect("the goal bindings");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["x"], "(u0 o1)", "the nested fact is not an s-expr");
    assert_eq!(rows[0]["y"], "o2");
}

/// **Writing an artefact changes nothing else.** Both flags, one claim.
///
/// `--json-summary` and `--events` exist so that a caller can read a run
/// without perturbing it, and that is only true if the run is *identical* with
/// them and without: same stdout, same stderr, same exit code. One invocation
/// can then answer every question at once, which is why both flags are carried
/// together here. `-p` for a large stdout and **not** `-s`, whose
/// `wall` row is wall-clock and differs run to run for reasons that have
/// nothing to do with the flags.
#[test]
fn recording_an_artefact_changes_nothing_else() {
    let scratch = Scratch::new("additive");
    let with = ein(&[
        "solve",
        TWO_MODELS,
        "-m",
        "2",
        "-p",
        "--json-summary",
        &scratch.at("s.json"),
        "--events",
        &scratch.at("e.jsonl"),
    ]);
    let without = ein(&["solve", TWO_MODELS, "-m", "2", "-p"]);
    assert_eq!(with.out, without.out, "stdout moved");
    assert_eq!(with.err, without.err, "stderr moved");
    assert_eq!(with.code, without.code, "the exit code moved");
    assert!(
        !read(&scratch.at("s.json")).is_empty() && !read(&scratch.at("e.jsonl")).is_empty(),
        "the flags wrote nothing, so the comparison is vacuous"
    );
}

/// **Every set-shaped observable in the summary is sorted.**
///
/// Two runs are byte-identical, so a `diff` of two summaries reports semantics
/// rather than iteration order — which is what makes the summary usable as a
/// regression artefact at all. The two places it could leak are the model's
/// own fact list and the `facts_by_relation` histogram's keys, and both are
/// checked rather than inferred from the byte equality: identical *wrong*
/// order twice would satisfy the first assertion alone.
#[test]
fn the_json_summary_is_order_free_and_reproducible() {
    let scratch = Scratch::new("order-free");
    let (a, b) = (scratch.at("a.json"), scratch.at("b.json"));
    ein(&["solve", TWO_MODELS, "-m", "3", "-e", "--json-summary", &a]).ok();
    ein(&["solve", TWO_MODELS, "-m", "3", "-e", "--json-summary", &b]).ok();
    assert_eq!(read(&a), read(&b), "two runs wrote different bytes");

    let j = json_at(&a);
    let facts: Vec<&str> = j["verdict"]["solutions"][0]["facts"]
        .as_array()
        .expect("the model's facts")
        .iter()
        .map(|f| f.as_str().expect("a fact is text"))
        .collect();
    let mut sorted = facts.clone();
    sorted.sort();
    assert_eq!(facts, sorted, "the model's facts are not sorted");
    let by_rel: Vec<&String> = j["root"]["facts_by_relation"]
        .as_object()
        .expect("the histogram")
        .keys()
        .collect();
    let mut sorted = by_rel.clone();
    sorted.sort();
    assert_eq!(by_rel, sorted, "facts_by_relation is not sorted by key");
}

/// **An abort still writes its summary**, and says which budget ran out.
///
/// The run that most needs a machine-readable record is the one that did not
/// finish, and the exit code is the caller's signal: 2 is "budget", distinct
/// from 1's "your file is wrong" and 0's "here is the answer". The `reason`
/// names the budget *and its value*, so a caller can raise the right one
/// instead of guessing which of `-E` / `-T` / `-m` it hit.
#[test]
fn an_abort_still_writes_its_summary() {
    let scratch = Scratch::new("abort");
    let path = scratch.at("summary.json");
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-e",
        "-E",
        "3",
        "--json-summary",
        &path,
    ]);
    assert_eq!(r.code, 2, "an abort is exit 2, not {}", r.code);
    let j = json_at(&path);
    assert_eq!(j["verdict"]["type"], "Aborted");
    assert_eq!(j["verdict"]["exhausted"], false);
    assert_eq!(j["verdict"]["reason"], "max-enterings (3) reached");
    assert_eq!(
        j["stats"]["enterings_total"], 3,
        "the partial counters are not the ones it stopped at"
    );
}

// ── `--events` — `tests/test_events.py` ────────────────────────────

/// **The stream ends in the verdict, and `n` is dense.**
///
/// Both halves are about being able to say *where* two logs diverge. The
/// sequence number is dense from 0 to the last event, so "the first difference
/// is at event k" identifies one event rather than a range; and the last event
/// is the `verdict`, carrying the type, `k`, `exhausted`, every counter and
/// the models — so a truncated log is detectable and a complete one needs no
/// second artefact to interpret.
#[test]
fn the_event_stream_is_dense_and_ends_in_the_verdict() {
    let scratch = Scratch::new("dense");
    let path = scratch.at("events.jsonl");
    ein(&["solve", TWO_MODELS, "-e", "--events", &path]).ok();
    let events = events_of(&path);
    assert!(events.len() > 100, "only {} events", events.len());

    let ns: Vec<i64> = events
        .iter()
        .map(|e| e["n"].as_i64().expect("every event carries n"))
        .collect();
    assert_eq!(
        ns,
        (0..events.len() as i64).collect::<Vec<i64>>(),
        "the sequence is not dense from 0"
    );
    assert_eq!(events[0]["e"], "run", "the stream does not open with `run`");

    let last = events.last().expect("a last event");
    assert_eq!(last["e"], "verdict");
    assert_eq!(last["type"], "Ambiguity");
    assert_eq!(last["k"], 2);
    assert_eq!(last["exhausted"], true);
    assert!(last["counters"].is_object(), "no counters on the verdict");
    assert_eq!(
        last["models"].as_array().map(Vec::len),
        Some(2),
        "the verdict does not carry both models"
    );
}

/// **`Events::off()` is inert** — it formats nothing and does not count.
///
/// This is the one events claim with no command line: the guard at the top of
/// `emit` returns *before* the closure runs, so a call site that omits its
/// `if events.on()` test is still correct — the payload is never built and the
/// sequence number never advances. That matters because the sequence number is
/// the protocol's own index: a counter that ticked when nothing was written
/// would make two logs of the same run disagree on where an event is.
#[test]
fn events_off_formats_nothing_and_does_not_count() {
    let mut off = Events::off();
    assert!(!off.on() && !off.verbose());
    let mut built = false;
    off.emit("fire", |l| {
        built = true;
        l.str("rule", "never");
    });
    assert!(!built, "the payload closure ran with no sink");
    assert_eq!(off.seq(), 0, "the sequence advanced with no sink");

    // The control: with a sink, the same call does both.
    let buffer = Buffer::new();
    let mut on = Events::to(Box::new(buffer.clone()), Level::Normal);
    assert_eq!(on.seq(), 1, "`to` emits the schema's `run` event first");
    on.emit("fire", |l| l.str("rule", "always"));
    assert_eq!(on.seq(), 2);
    let text = buffer.to_string_lossy();
    assert!(text.contains("\"rule\": \"always\""), "{text}");
    assert_eq!(text.lines().count(), 2);
}

/// The corpus fixtures that between them emit every kind
/// [`events.md`](../../../../docs/kernel/inference/events.md) defines, and the
/// kind each is here for.
///
/// A five-file cover rather than the whole corpus: the sweep is 0.06 s this
/// way and 40 s the other, and the question — *is any kind unreachable?* — is
/// answered as well by a cover as by a sweep. What the cover cannot do is
/// notice a kind that stopped being emitted by a file **not** listed here,
/// which is why each entry names its reason.
const EVENT_COVER: [(&str, &str); 6] = [
    (
        "examples/branching/01_saturate_only.ein",
        "the lifecycle and the deductive layer",
    ),
    (
        "examples/branching/02_one_dead_one_alive.ein",
        "enter / nogood / alt",
    ),
    (
        "examples/branching/07_lookahead_off.ein",
        "writeback — the singleton (not h)",
    ),
    (
        "examples/branching/12_typed_blind_solve.ein",
        "park / admit / retire — the NAF boundary",
    ),
    (
        "examples/features/06_symmetric_native.ein",
        "mirror — the native arg swap",
    ),
    (
        "tests/stdlib/algebra/23_total_owed.ein",
        "owe — the post-fixpoint obligation pass, which no other cover file \
         activates",
    ),
];

/// Where the `--events` schema lives. It was `conformance/EVENTS.md` until
/// S1a.10.3: the protocol is a product surface — a debugging tool, and
/// M20's likely feed — and it outlived both the directory named after the
/// two-engine harness and the tier that was its first reader.
const EVENTS_DOC: &str = "docs/kernel/inference/events.md";

/// Every event kind the schema names, read out of the document itself.
///
/// Parsed rather than copied: the schema is the contract and a second list
/// here would be the thing that drifts. The kind cells are the first column of
/// the three payload tables — and only those, since the envelope table above
/// them describes `e` and `n`, the two fields every line carries. `park` /
/// `admit` / `retire` share one row, so a cell may name three kinds.
const PAYLOAD_SECTIONS: [&str; 3] = ["### Lifecycle", "### Deductive layer", "### Search layer"];

fn schema_kinds() -> BTreeSet<String> {
    let doc = std::fs::read_to_string(repo_root().join(EVENTS_DOC)).expect(EVENTS_DOC);
    let mut kinds = BTreeSet::new();
    let mut in_payload = false;
    for line in doc.lines() {
        let line = line.trim();
        if line.starts_with("#") {
            in_payload = PAYLOAD_SECTIONS.contains(&line);
            continue;
        }
        if !in_payload || !line.starts_with("| `") {
            continue;
        }
        let cell = line[1..].split('|').next().unwrap_or("").trim();
        for name in cell.split('/') {
            let name = name.trim().trim_matches('`').trim();
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase()) && name != "e" {
                kinds.insert(name.to_string());
            }
        }
    }
    kinds
}

/// **Every kind the schema defines is emitted by some corpus fixture.**
///
/// A kind nothing reaches is a kind the engine can silently stop emitting
/// while the tier that should catch it stays green — the same shape of problem
/// as a test that skips. Four of the seventeen were unreached by any ein.rs
/// test when this was written (`load`, `verdict`, `nogood`, `writeback`), and
/// the parse of `EVENTS.md` is what keeps the cover honest as the schema
/// grows: a new row with no fixture behind it fails here.
#[test]
fn every_event_kind_the_schema_defines_is_reachable_from_the_corpus() {
    let scratch = Scratch::new("cover");
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for (i, (file, _why)) in EVENT_COVER.iter().enumerate() {
        let path = scratch.at(&format!("cover-{i}.jsonl"));
        // A budget, so the two large fixtures stay fast; the kinds they are
        // here for are all emitted well before it.
        ein(&[
            "solve",
            file,
            "-e",
            "-E",
            "60",
            "--events",
            &path,
            "--events-level",
            "verbose",
        ]);
        for e in events_of(&path) {
            *seen
                .entry(e["e"].as_str().unwrap_or("?").to_string())
                .or_insert(0) += 1;
        }
    }
    let schema = schema_kinds();
    assert!(
        schema.len() >= 18,
        "EVENTS.md parsed to only {} kinds — the table shape moved: {schema:?}",
        schema.len()
    );
    let missing: Vec<&String> = schema.iter().filter(|k| !seen.contains_key(*k)).collect();
    assert!(
        missing.is_empty(),
        "no corpus fixture emits {missing:?}; the cover reached {:?}",
        seen.keys().collect::<Vec<_>>()
    );
    let extra: Vec<&String> = seen.keys().filter(|k| !schema.contains(*k)).collect();
    assert!(extra.is_empty(), "{extra:?} is emitted but not documented");
}

/// **`--events-level` gates the high-volume kinds**, and nothing else.
///
/// `normal` is the level a shipping run can afford; `verbose` adds the
/// redundant firings and the pre-candidate hypothesis skips, which is roughly
/// six times the volume. T2 compared at `verbose` for one reason: a *redundant*
/// firing is precisely what a port drops without changing any answer, so the
/// level that hides them is the level that hides the bug.
#[test]
fn the_events_level_gates_the_high_volume_kinds() {
    let scratch = Scratch::new("level");
    let (n, v) = (scratch.at("normal.jsonl"), scratch.at("verbose.jsonl"));
    ein(&["solve", TWO_MODELS, "-e", "--events", &n]).ok();
    ein(&[
        "solve",
        TWO_MODELS,
        "-e",
        "--events",
        &v,
        "--events-level",
        "verbose",
    ])
    .ok();
    let (normal, verbose) = (events_of(&n), events_of(&v));
    let (kn, kv) = (kinds(&normal), kinds(&verbose));

    assert_eq!(
        kn.get("hypskip"),
        None,
        "normal emitted a pre-candidate skip"
    );
    assert!(kv.get("hypskip").is_some_and(|&n| n > 0), "verbose did not");
    let redundant = |events: &[J]| {
        events
            .iter()
            .filter(|e| e["e"] == "fire" && e["redundant"] == true)
            .count()
    };
    assert_eq!(redundant(&normal), 0, "normal emitted a redundant firing");
    assert!(redundant(&verbose) > 0, "verbose emitted none");
    assert!(
        verbose.len() > normal.len(),
        "verbose ({}) is not longer than normal ({})",
        verbose.len(),
        normal.len()
    );
    // And the levels agree on everything else: normal is a subsequence of
    // verbose by kind, not a different run.
    for (kind, count) in &kn {
        assert!(
            kv.get(kind).copied().unwrap_or(0) >= *count,
            "verbose lost {kind}: {count} → {:?}",
            kv.get(kind)
        );
    }
}

/// **`--events-level` is a closed set.** An unknown level is refused.
///
/// The failure it prevents is silent: a typo that fell back to `normal` would
/// produce a *shorter log that looks complete*, and a T2 comparison against it
/// would pass because both sides were missing the same events.
#[test]
fn an_unknown_events_level_is_refused_by_name() {
    let scratch = Scratch::new("bad-level");
    let path = scratch.at("events.jsonl");
    let r = ein(&[
        "solve",
        TWO_MODELS,
        "--events",
        &path,
        "--events-level",
        "loud",
    ]);
    assert_ne!(r.code, 0, "an unknown level was accepted");
    assert!(
        r.err.contains("loud"),
        "the diagnostic does not name the offending value:\n{}",
        r.err
    );
    for known in ["normal", "verbose"] {
        assert!(
            r.err.contains(known),
            "the diagnostic does not list {known}:\n{}",
            r.err
        );
    }
}

/// **The event stream does not depend on the order ids were assigned in.**
///
/// The Python original ran the same file under `PYTHONHASHSEED=0` and `=42`,
/// because ein.py's `hash()` is salted and a set iterated at an instrumented
/// site would come out reordered. ein.rs has no salted hash, so re-running it
/// proves nothing; the question survives in this engine's own terms, because
/// its ids are **assignment-ordered** — a `Symbol` is "how many distinct names
/// had been seen when this one arrived". Perturbing that is the same
/// experiment.
///
/// Here the perturbation is the cheap half: every name the file interns is
/// pre-interned in reverse discovery order, so the whole non-kernel symbol
/// space is renumbered before the loader sees a byte. The full-strength
/// version — names, integer literals and the fact space all permuted, over the
/// whole corpus and every rendering — is
/// `ein-render/tests/id_order_invariance.rs`; what is added here is the one
/// observable that sweep does not render, the **event stream**.
#[test]
fn the_event_stream_does_not_depend_on_interning_order() {
    fn stream(rel: &str, reverse: bool) -> String {
        let path = corpus(rel);
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        if reverse {
            // Discover the names by loading once, then renumber them.
            let mut scout_ast = Ast::new();
            let mut scout = Terms::new();
            load_file(&mut scout_ast, &mut scout, &path).expect("the fixture loads");
            let kernel = Terms::new().syms.len();
            for i in (kernel..scout.syms.len()).rev() {
                terms
                    .intern_text(scout.syms.text(Symbol(i as u32)))
                    .expect("room");
            }
            assert!(
                terms.syms.len() > kernel + 4,
                "the perturbation renumbered almost nothing"
            );
        }
        let mut kb = load_file(&mut ast, &mut terms, &path).expect("the fixture loads");
        let buffer = Buffer::new();
        let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
        let opts = SolveOptions {
            stop_after: None,
            max_set_size: 3,
            store_lattice: false,
            on_budget: OnBudget::Verdict,
            max_enterings: Some(60),
            ..SolveOptions::default()
        };
        solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
            .expect("the fixture solves");
        buffer.to_string_lossy()
    }

    for rel in [TWO_MODELS, "examples/branching/12_typed_blind_solve.ein"] {
        let plain = stream(rel, false);
        assert!(plain.lines().count() > 20, "{rel}: too short to compare");
        assert_eq!(
            plain,
            stream(rel, false),
            "{rel}: two identical runs disagree, so the engine is not deterministic"
        );
        assert_eq!(
            plain,
            stream(rel, true),
            "{rel}: the stream moved when the id space was renumbered"
        );
    }
}

// ── The editor grammar — `tests/test_vscode_grammar.py` ────────────

/// The TextMate grammar mirrors three *closed* reserved-name sets in head
/// position. A stray copy that drifts from the registry is the failure mode
/// these four tests exist for.
const GRAMMAR: &str = "utils/vscode-ein/ein.tmLanguage.json";

/// The closed declarator set — P1.7c, plus S1.5.9's `macro` and S1.8.A2's
/// `import`. Source of truth:
/// [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md).
const DECLARATORS: [&str; 8] = [
    "config", "hrule", "import", "macro", "query", "relation", "rule", "trace",
];

/// The wrapper heads S1.7c.4 removed. They are ordinary fact heads now and
/// must be highlighted by none of the three reserved scopes.
const REMOVED_WRAPPERS: [&str; 4] = ["ontology", "facts", "reasoning", "rules"];

/// The names a pattern highlights under `scope`, over the whole grammar.
///
/// A head keyword reaches the grammar as a lowercase alternation in capture
/// group 2 of a `begin` / `match` regex — `(\()\s*(relation)\b…` for one name,
/// `(query|config|trace|import)` for a set. Group 1 is the literal paren and
/// group 3 is the declared *name*, and neither can be mistaken for the
/// alternation: both start with a character that is not `[a-z]`.
fn names_for_scope(node: &J, scope: &str, found: &mut BTreeSet<String>) {
    match node {
        J::Object(map) => {
            let caps = map.get("beginCaptures").or_else(|| map.get("captures"));
            let is_head = caps
                .and_then(|c| c.get("2"))
                .and_then(|c| c.get("name"))
                .and_then(J::as_str)
                == Some(scope);
            if is_head {
                let regex = map
                    .get("begin")
                    .or_else(|| map.get("match"))
                    .and_then(J::as_str)
                    .unwrap_or("");
                found.extend(lowercase_alternations(regex));
            }
            for v in map.values() {
                names_for_scope(v, scope, found);
            }
        }
        J::Array(items) => {
            for v in items {
                names_for_scope(v, scope, found);
            }
        }
        _ => {}
    }
}

/// Every `(a|b|c)` group of lowercase words in a regex — the Python original's
/// `r"\(([a-z][a-z*-]*(?:\|[a-z][a-z*-]*)*)\)"`, hand-scanned because
/// `ein-cli` has no regex dependency and one alternation form does not justify
/// acquiring one.
fn lowercase_alternations(regex: &str) -> Vec<String> {
    let bytes: Vec<char> = regex.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != '(' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let Some(len) = bytes[start..].iter().position(|&c| c == ')') else {
            break;
        };
        let body: String = bytes[start..start + len].iter().collect();
        let words: Vec<&str> = body.split('|').collect();
        let wordlike = |w: &&str| {
            !w.is_empty()
                && w.starts_with(|c: char| c.is_ascii_lowercase())
                && w.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '*' || c == '-')
        };
        if words.iter().all(wordlike) {
            out.extend(words.iter().map(|w| w.to_string()));
        }
        i = start + len + 1;
    }
    out
}

fn grammar() -> J {
    serde_json::from_str(&read(&corpus(GRAMMAR).to_string_lossy())).expect("the grammar is JSON")
}

/// **The checked-in grammar is the *ein* grammar.** It parses, it claims
/// `source.ein`, and it claims the `ein` file type.
///
/// The cheapest of the four and the one that fails first: a grammar that does
/// not parse is not applied by the editor at all, and there is no error
/// anywhere — the file simply stops colouring, which nobody reports as a bug.
#[test]
fn the_textmate_grammar_is_the_ein_grammar() {
    let g = grammar();
    assert_eq!(g["scopeName"], "source.ein");
    assert!(
        g["fileTypes"]
            .as_array()
            .expect("fileTypes")
            .iter()
            .any(|t| t == "ein"),
        "the grammar does not claim .ein files"
    );
    assert!(g["patterns"].is_array(), "the grammar has no patterns");
}

/// **The highlighted declarators are exactly the closed set.**
///
/// Closed is the operative word: the eight are the only heads the *loader*
/// treats as declarations, so a ninth in the grammar promises a syntax that
/// does not exist and a missing one leaves a real declarator looking like an
/// ordinary fact. `06_reserved_names.md` is the source of truth and the list
/// here mirrors it.
#[test]
fn the_grammars_declarators_are_the_closed_set() {
    let mut found = BTreeSet::new();
    names_for_scope(&grammar(), "keyword.control.declarator.ein", &mut found);
    assert_eq!(
        found,
        DECLARATORS
            .iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<String>>(),
        "the grammar's declarator set drifted"
    );
}

/// **The grammar tracks the primitive and predicate registries.**
///
/// Both lists live in the engine — `ein_core::STRUCTURAL` and
/// `ein_infer::predicates::names()` — and the grammar is a *copy* of them,
/// which is the whole reason this test exists. Adding a predicate to the
/// engine without adding it to the grammar has no symptom a developer would
/// notice: the new name just fails to colour, in an editor, on someone else's
/// machine.
///
/// **One name is highlighted that `STRUCTURAL` does not hold**: `open`, the
/// verdict atom (M1d S1d.2.3). It is in `RESERVED` — no declarator may bind
/// it — but deliberately not in `STRUCTURAL`, because a structural primitive
/// is rule-*body* vocabulary the compiler, matcher or detector reads, and
/// `open` is none of those: it appears only as an `:assert` conclusion and is
/// never stored. An editor still has to colour it like `(false)`, its dual,
/// so the grammar's set is `STRUCTURAL ∪ {open}` and the assertion says so by
/// construction rather than by listing — a tenth primitive added to the
/// engine still fails this test.
#[test]
fn the_grammar_tracks_the_primitive_and_predicate_registries() {
    let g = grammar();
    let mut primitives = BTreeSet::new();
    names_for_scope(&g, "keyword.control.primitive.ein", &mut primitives);
    let expected: BTreeSet<String> = ein_core::STRUCTURAL
        .iter()
        .map(|s| s.to_string())
        .chain(std::iter::once("open".to_string()))
        .collect();
    assert!(
        ein_core::RESERVED.contains(&"open") && !ein_core::STRUCTURAL.contains(&"open"),
        "`open` is reserved but not a rule-body primitive — if that changed, \
         this test's exception is the thing to revisit"
    );
    assert_eq!(
        primitives, expected,
        "the grammar's primitives are not ein_core::STRUCTURAL + the verdict atom"
    );

    let mut predicates = BTreeSet::new();
    names_for_scope(&g, "keyword.operator.predicate.ein", &mut predicates);
    assert_eq!(
        predicates,
        ein_infer::predicates::names()
            .iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<String>>(),
        "the grammar's predicates are not predicates::names()"
    );
}

/// **The removed block wrappers are not keywords.**
///
/// `ontology` / `facts` / `reasoning` / `rules` were block heads until
/// S1.7c.4 and are ordinary relation names now — a file may declare
/// `(relation facts …)` and mean it. Highlighting one as a keyword would tell
/// the reader the language still has block structure, which is the specific
/// regression this guards.
#[test]
fn the_removed_block_wrappers_are_not_highlighted_as_keywords() {
    let g = grammar();
    for scope in [
        "keyword.control.declarator.ein",
        "keyword.control.primitive.ein",
        "keyword.operator.predicate.ein",
    ] {
        let mut found = BTreeSet::new();
        names_for_scope(&g, scope, &mut found);
        for wrapper in REMOVED_WRAPPERS {
            assert!(
                !found.contains(wrapper),
                "{wrapper:?} is still highlighted as {scope}"
            );
        }
    }
    // Non-vacuity: `rule` is a declarator and `rules` is not, so the check is
    // about whole names rather than about prefixes.
    let mut declarators = BTreeSet::new();
    names_for_scope(&g, "keyword.control.declarator.ein", &mut declarators);
    assert!(declarators.contains("rule") && !declarators.contains("rules"));
}

// ── `--layer-progress` — M1d P1d.10 ────────────────────────────────

/// Parse the `layer N gen:` / `layer N test:` / `layer N done:` lines into
/// `(label, {key: value})`, which is what the two tests below assert on.
fn layer_lines(err: &str) -> Vec<(String, std::collections::BTreeMap<String, u64>)> {
    let mut out = Vec::new();
    for l in err.lines() {
        let t = l.trim_start();
        let Some(rest) = t.strip_prefix("layer ") else {
            continue;
        };
        let mut it = rest.split_whitespace();
        let (Some(n), Some(what)) = (it.next(), it.next()) else {
            continue;
        };
        let mut kv = std::collections::BTreeMap::new();
        for tok in it {
            if let Some((k, v)) = tok.split_once('=')
                && let Ok(v) = v.parse::<u64>()
            {
                kv.insert(k.to_string(), v);
            }
        }
        out.push((format!("{n} {what}"), kv));
    }
    out
}

/// **The layer rows' arithmetic closes.**
///
/// The flag exists so a reader can watch a search that runs for minutes, and a
/// progress line nobody can add up is a progress line nobody can trust. Three
/// identities, per layer, and each names a different half of the loop:
///
/// - `joined − dropped_dead − dropped_nogood = candidates` — generation
/// - `entered = alive + dead` — testing
/// - `alive = complete + survivors` — where the survivors went, which is the
///   one the census row could not state on its own: `alive_enterings` counts
///   every consistent fork and only the incomplete ones reach the next
///   frontier
///
/// `zebra2` is the fixture because it is the corpus's pruning case — layer 1
/// kills 32 of 56 and completes 13 more, so every term above is non-zero.
#[test]
fn the_layer_progress_rows_add_up() {
    let r = ein(&["solve", "-e", "--layer-progress", "examples/zebra2.ein"]);
    assert_eq!(r.code, 0, "stderr:\n{}", r.err);
    let rows = layer_lines(&r.err);
    let generated: Vec<_> = rows.iter().filter(|(l, _)| l.ends_with("gen:")).collect();
    let test: Vec<_> = rows.iter().filter(|(l, _)| l.ends_with("test:")).collect();
    let done: Vec<_> = rows.iter().filter(|(l, _)| l.ends_with("done:")).collect();
    assert_eq!(
        generated.len(),
        2,
        "two layers, two generation rows:\n{}",
        r.err
    );
    assert_eq!(test.len(), generated.len());
    assert_eq!(done.len(), generated.len());

    let mut saw_a_death = false;
    let mut saw_a_completion = false;
    for i in 0..generated.len() {
        let (g, t, d) = (&generated[i].1, &test[i].1, &done[i].1);
        let at = |m: &std::collections::BTreeMap<String, u64>, k: &str| {
            *m.get(k)
                .unwrap_or_else(|| panic!("no {k} in layer {i} of:\n{}", r.err))
        };
        assert_eq!(
            at(g, "joined") - at(g, "−dead") - at(g, "−clause"),
            at(g, "cand"),
            "layer {i}: the join and the two filters do not reach `cand`"
        );
        assert_eq!(
            at(t, "entered"),
            at(t, "alive") + at(t, "dead"),
            "layer {i}: an entering was neither alive nor dead"
        );
        assert_eq!(
            at(t, "alive"),
            at(t, "complete") + at(d, "survivors"),
            "layer {i}: a consistent fork was neither complete nor a survivor"
        );
        assert_eq!(
            at(g, "cand"),
            at(t, "entered"),
            "layer {i}: a candidate was generated and not entered"
        );
        saw_a_death |= at(t, "dead") > 0;
        saw_a_completion |= at(t, "complete") > 0;
    }
    assert!(saw_a_death, "the fixture has to kill something");
    assert!(saw_a_completion, "…and complete something");
}

/// **It is additive, and it is the layer half of `--verbose` alone.**
///
/// Two claims in one run because they are the same claim from two sides: the
/// flag changes what goes to *stderr* and nothing else, and what it removes
/// from `-v` is the per-entering firehose — 6 180 lines on a 618 076-entering
/// run at the default `--progress-every`, which is what makes `-v` unusable
/// for watching a layer.
#[test]
fn layer_progress_is_verbose_without_the_enterings() {
    let plain = ein(&["solve", "-e", "-s", "examples/zebra2.ein"]);
    let lp = ein(&[
        "solve",
        "-e",
        "-s",
        "--layer-progress",
        "examples/zebra2.ein",
    ]);
    let v = ein(&["solve", "-e", "-s", "-v", "examples/zebra2.ein"]);

    // `wall` is the one volatile row of `--stats`, and it is volatile under
    // any two runs — the claim is about every other byte.
    let steady = |o: &str| {
        o.lines()
            .filter(|l| !l.trim_start().starts_with("wall"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        steady(&plain.out),
        steady(&lp.out),
        "--layer-progress moved stdout"
    );
    assert_eq!(plain.code, lp.code);
    assert!(
        plain.err.is_empty(),
        "the control wrote stderr: {}",
        plain.err
    );

    let entering = |e: &str| {
        e.lines()
            .filter(|l| l.trim_start().starts_with("e="))
            .count()
    };
    assert_eq!(
        entering(&lp.err),
        0,
        "--layer-progress narrated an entering"
    );
    assert!(
        entering(&v.err) > 0,
        "-v narrated none — the contrast is vacuous"
    );
    assert_eq!(
        layer_lines(&lp.err).len(),
        layer_lines(&v.err).len(),
        "the two disagree about the layer half"
    );
}

// ── The `k = 0` verdicts qualify themselves — M1d T1d.10.5.2b ──────

/// **A refutation needs the lattice exhausted, and says so when it did not
/// get it.**
///
/// [S1d.3.3](../../../../plans/m1d_satisfiability/p1d.3_model_sets/s1d.3.3_the_verdict.md)
/// made `exhausted = true` ⇒ *these are the models* normative and gave the
/// qualifier to `Solution` and `Ambiguity`, leaving `Contradiction` — where
/// the problem is a *word* and not a number — to
/// [Q-M1d.1](../../../../plans/m1d_satisfiability/open_questions.md). This is
/// that word.
///
/// `saturation/type-exclusivity/pets.ein` is the fixture because it makes the
/// old sentence flatly false: *the constraints are contradictory* at `-m 5`
/// and `-m 8`, **35 models** at `-m 10`. `ein-bugs/zebra2-bad.ein` is the
/// non-vacuity control — an exhausted refutation with a real one-fact core,
/// which must keep every word it had.
///
/// The claim channel needed no change and is asserted next door:
/// `expect_semantics::a_contradiction_from_a_truncated_search_is_not_checked`
/// has always answered `NOT CHECKED` here. What this pins is that the *verdict*
/// channel now agrees with it instead of contradicting it.
#[test]
fn a_truncated_k0_is_not_reported_as_a_refutation() {
    let cut = ein(&[
        "solve",
        "-e",
        "-m",
        "5",
        "-s",
        "examples/saturation/type-exclusivity/pets.ein",
    ]);
    assert_eq!(cut.code, 0);
    assert_eq!(cut.field("exhausted").as_deref(), Some("false"));
    let verdict = cut
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("verdict"))
        .unwrap_or_else(|| panic!("no verdict line in:\n{}", cut.out));
    assert!(
        !verdict.contains("contradictory"),
        "a search that stopped at -m 5 called a 35-model program contradictory: {verdict}"
    );
    assert!(
        verdict.contains("did not exhaust"),
        "the verdict does not say why its zero is a zero: {verdict}"
    );
    assert!(
        cut.out
            .contains("(none found — the search did not exhaust)"),
        "the count carries no qualifier:\n{}",
        cut.out
    );
    // An unsat core explains why a program has *no model*, which is exactly
    // what this run did not show — so the block is named for what it holds.
    assert!(
        cut.out.contains("refuted so far") && !cut.out.contains("unsat core"),
        "a truncated run still calls its deaths an unsat core:\n{}",
        cut.out
    );

    // Non-vacuity: an exhausted refutation keeps every word.
    let done = ein(&["solve", "-e", "-s", "examples/ein-bugs/zebra2-bad.ein"]);
    assert_eq!(done.code, 0);
    assert_eq!(done.field("exhausted").as_deref(), Some("true"));
    assert!(
        done.out
            .contains("verdict         No solution — the constraints are contradictory"),
        "the exhausted arm moved:\n{}",
        done.out
    );
    assert!(done.out.contains("unsat core (1 facts)"));
    assert!(
        !done.out.contains("did not exhaust"),
        "the exhausted arm grew a qualifier it has no use for:\n{}",
        done.out
    );
}
