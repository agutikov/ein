//! T1a.10.2.2 — **the `ein` binary as a tool**: exit codes, flag effects, the
//! `--events` protocol, and the shape of the three canonical Zebra files.
//!
//! The common subject is *the surface a caller sees* rather than what the
//! engine concluded:
//!
//! | subject | what it owns |
//! |---|---|
//! | every subcommand | how a broken file is reported |
//! | `ein solve` | the stop policy, the diagnostic flags, `--json-summary` |
//! | the event stream | `--events`, whose spec is [`events.md`](../../../../docs/kernel/inference/events.md) |
//! | the Zebra files | that `zebra2.ein` and its two variants stay one encoding |
//! | the editor grammar | its three closed name sets |
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

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use ein_core::{Kb, ProvKind, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level, sexpr};
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
    ein_env(&[], args)
}

/// The same, with process environment. A **child** process is the only way to
/// set an `EIN_*` name in this file: cargo runs these tests as threads of one
/// binary, and a `set_var` here would be read by whatever else is mid-solve.
/// Its environment callers were the walk's, deleted at M1e2 S1e2.1c.5; the
/// stdlib-resolution test below still sets `EIN_STDLIB` through it.
fn ein_env(env: &[(&str, &str)], args: &[&str]) -> Run {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(args).current_dir(repo_root());
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("the `ein` binary runs");
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
// The acceptance gate reads its GAPS answer from `zebra2-minus-15.ein` and its CONTRADICTIONS answer from
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
/// so the contract is on the *declared* set.
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
/// too, through `python3`, because it lives under `examples/`.
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

    // The byte-level half — **required, not opportunistic**, since M1e
    // S1e.3.3's neighbour
    // [S1e.3.6](../../../../docs/history/m1e_review_processing/README.md#s1e36--tests-medium)
    // T1, the review's `TE-M1`.
    //
    // It used to `eprintln!` and pass when `python3` was absent or exited 127.
    // That is a shape the repo has already been bitten by — `dot_wellformed`
    // was converted for the same reason — and here it was worse than a lost
    // assertion: the structural diff above compares rule
    // *names*, so a drifted rule **body** in a generated variant is caught by
    // nothing else. A local gate on a machine without `python3` reported a
    // pass on exactly that, while `AGENTS.md` said flatly *`--check` is in the
    // gate*.
    //
    // 127 stays distinguished from a real failure, because a missing
    // interpreter and a stale fixture are different things to be told — but
    // both are now failures, and the message says which.
    require_python3();
    let out = Command::new("python3")
        .arg("examples/gen_zebra2_variants.py")
        .arg("--check")
        .current_dir(repo_root())
        .output()
        .expect("python3 ran a moment ago");
    assert!(
        out.status.success(),
        "the on-disk variants are stale — run `python3 examples/gen_zebra2_variants.py`\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The one external tool the workspace still needs besides Graphviz, required
/// the way `dot_wellformed::require_graphviz` requires that one.
///
/// Four `examples/` files are **generated** from `zebra2.ein` and
/// `gen_zebra2_variants.py --check` is what compares the bytes; without it the
/// only thing standing between a hand-edited variant and the gate is a
/// structural diff over rule *names*. So it is a missing gate rather than a
/// missing convenience, which is `require_graphviz`'s own sentence and is the
/// rule M1e S1e.3.6 T1 applied here.
fn require_python3() {
    let run = Command::new("python3")
        .arg("-c")
        .arg("pass")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match run {
        Ok(s) if s.success() => {}
        Ok(s) => panic!(
            "`python3 -c pass` exited {:?}: the interpreter on PATH cannot run \
             a script, so `examples/gen_zebra2_variants.py --check` cannot \
             either. Four examples/ files are generated and this is the only \
             check that compares their bytes — a missing gate, not a missing \
             convenience.",
            s.code()
        ),
        Err(e) => panic!(
            "`python3` does not run ({e}): four examples/ files are generated \
             from zebra2.ein and `gen_zebra2_variants.py --check` is the only \
             check that compares their bytes. Install a python3 — a missing \
             gate, not a missing convenience."
        ),
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
/// being *the* one that matters. The two saturations are a few milliseconds
/// each, so it runs in the default tier.
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

// ── The diagnostics ─────────────────────────────────────────────────

/// A file that parses and then fails to load, for the diagnostics below.
const LOAD_NEGATIVE: &str = "examples/broken/load/derivation_cycle.ein";
/// A file that does not parse.
const PARSE_NEGATIVE: &str = "examples/broken/unclosed_paren.ein";

/// **A load failure is *diagnosed*, not raised.** One line, on stderr, exit 1,
/// and no stack trace.
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

// ── The stop policy and the diagnostic flags ──────────────────────

/// The fixture the stop-policy claims are made on: two genuine models, so
/// "stop at one" and "exhaust" have different answers.
const TWO_MODELS: &str = "examples/branching/04_two_levels.ein";

/// The states a read-out lists: one `;; state i of k` heading each.
fn listed(r: &Run) -> usize {
    r.out
        .lines()
        .filter(|l| l.trim_start().starts_with(";; state ") && l.contains(" of "))
        .count()
}

/// **The question is the flags'**, and the four settings differ — M1e2
/// S1e2.1a.5's switch.
///
/// Bare `ein solve` describes all of R: seven states on this program,
/// exhausted, as one `(region …)`. `-n 2` lists two of them and says the part
/// may hold more. `--maximal` lists the two states nothing can be added to.
/// `--single` asks whether R is one state and answers the word. The short and
/// long spellings are one flag.
#[test]
fn the_question_is_the_flags() {
    let all = ein(&["solve", TWO_MODELS]);
    all.ok();
    assert_eq!(all.field("states (k_s)").as_deref(), Some("7"));
    assert_eq!(all.field("exhausted").as_deref(), Some("yes"));
    assert!(all.out.contains("  (region\n"), "{}", all.out);
    assert_eq!(listed(&all), 7);

    let n2 = ein(&["solve", TWO_MODELS, "-n", "2"]);
    n2.ok();
    assert_eq!(listed(&n2), 2);
    assert!(!n2.out.contains("(region"), "a cut is never a (region …)");

    let max = ein(&["solve", TWO_MODELS, "--maximal"]);
    max.ok();
    assert_eq!(listed(&max), 2);
    assert!(
        max.field("maximal line").is_some_and(|v| v == "2"),
        "{}",
        max.out
    );

    let single = ein(&["solve", TWO_MODELS, "--single"]);
    single.ok();
    assert!(
        single
            .out
            .contains("Ambiguity — R holds two states or more"),
        "the word did not reach stdout:\n{}",
        single.out
    );

    let long = ein(&["solve", TWO_MODELS, "--solutions", "2"]);
    let header = |r: &Run| r.out.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(header(&long), header(&n2), "-n and --solutions differ");
}

/// **`-n/--solutions` takes a count of one or more, and refuses everything
/// else** — M1e
/// [S1e.1.5](../../../../docs/history/m1e_review_processing/README.md#s1e15---n-0-q7),
/// the review's `Q7`.
///
/// `-n 0` used to be accepted, and what it *did* was `-n 1`. Neither reading a
/// user could mean was available — *record nothing* is not what it did, and
/// *no limit* is leaving `-n` out, which describes all of R since M1e2
/// S1e2.1a.5.
///
/// Two halves, and the second is the one that would rot quietly:
///
/// - the refusal, its exit code (**2**, a usage error) and its wording;
/// - a **non-integer** still gets `int_arg`'s message, unchanged, because
///   [`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)
///   quotes that message verbatim and uses `--solutions` as its example.
#[test]
fn solutions_takes_a_count_of_one_or_more_and_nothing_else() {
    // The positive control: the flag still works.
    for good in [1usize, 2] {
        let r = ein(&["solve", TWO_MODELS, "-n", &good.to_string()]);
        r.ok();
        assert_eq!(listed(&r), good, "-n {good} did not list {good} states");
    }

    // A negative needs the `=` spelling to reach the validator at all: `-n -1`
    // is clap's own *unexpected argument* — also exit 2.
    for bad in ["-n=0", "--solutions=0", "--solutions=-1", "--solutions=-7"] {
        let r = ein(&["solve", TWO_MODELS, bad]);
        assert_eq!(r.code, 2, "{bad} was not a usage error:\n{}", r.err);
        assert!(
            r.err.contains("solution count"),
            "{bad} was refused without saying why:\n{}",
            r.err
        );
        assert!(
            r.err.contains("1 or more") && r.err.contains("leave -n out"),
            "{bad}'s message names neither the range nor the other reading:\n{}",
            r.err
        );
        assert!(
            r.out.is_empty(),
            "{bad} printed to stdout before refusing:\n{}",
            r.out
        );
    }
    let spaced = ein(&["solve", TWO_MODELS, "-n", "-1"]);
    assert_eq!(
        spaced.code, 2,
        "`-n -1` reached the engine:\n{}",
        spaced.err
    );

    // §4's row, which is about the *value type* and not about the range.
    for bad in ["x", "1.5"] {
        let r = ein(&["solve", TWO_MODELS, "-n", bad]);
        assert_eq!(r.code, 2, "-n {bad} was not a usage error:\n{}", r.err);
        assert!(
            r.err
                .contains(&format!("invalid value '{bad}' for '--solutions <N>'")),
            "the documented usage-error line moved:\n{}",
            r.err
        );
        assert!(
            r.err.contains(&format!("invalid int value: '{bad}'")),
            "a non-integer must keep `int_arg`'s message — defined_behaviour.md \
             §4 quotes it:\n{}",
            r.err
        );
    }
}

/// **`-g/--progress-every` 0 means no node lines, and a negative is
/// refused** — M1e2 S1e2.3.4, the user's ruling (b) of 2026-11-03 on
/// [Q-M1e.17](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative).
///
/// The region modes clamped it with `.max(1)`, so `-v -g=0` and `-v -g=-3`
/// logged every node, as `-g 1` does, with no word (P1e2.3's S5). Now `-g 0`
/// is what `ein saturate --progress-every 0` is: the step lines and no node
/// line. A negative is a usage error, exit 2, with a stderr `error:` that says
/// why, in both spellings, since the option takes negative numbers as values.
/// stdout never moves: the option is stderr's.
#[test]
fn progress_every_is_zero_for_no_node_lines_and_refuses_a_negative() {
    const FIVE: &str = "examples/branching/05_mini_zebra.ein";
    let node = |e: &str| e.lines().filter(|l| l.starts_with("  node ")).count();
    let step = |e: &str| e.lines().filter(|l| l.contains(" visited — ")).count();
    let plain = ein(&["solve", FIVE]);
    let every = ein(&["solve", FIVE, "-v", "-g", "1"]);
    let none = ein(&["solve", FIVE, "-v", "-g", "0"]);
    for r in [&every, &none] {
        assert_eq!(r.code, 0, "{}", r.err);
        assert_eq!(r.out, plain.out, "-g moved stdout");
    }
    assert!(node(&every.err) > 1, "-g 1 logged no node:\n{}", every.err);
    assert_eq!(node(&none.err), 0, "-g 0 logged a node:\n{}", none.err);
    assert!(
        step(&none.err) > 0 && step(&none.err) == step(&every.err),
        "-g 0 must keep -v's step lines:\n{}",
        none.err
    );
    for bad in [
        &["-v", "-g=-1"][..],
        &["-v", "-g", "-1"],
        &["-v", "--progress-every=-3"],
    ] {
        let mut a = vec!["solve", FIVE];
        a.extend(bad);
        let r = ein(&a);
        assert_eq!(r.code, 2, "{bad:?} was not a usage error:\n{}", r.err);
        assert!(
            r.err.starts_with("error:") && r.err.contains("invalid progress interval"),
            "{bad:?} was refused without saying why:\n{}",
            r.err
        );
        assert!(r.out.is_empty(), "{bad:?} printed to stdout:\n{}", r.out);
    }
}

/// **`ein saturate --max-steps N` says where it stopped, in Ein's spelling,
/// and exits 0** — M1e2 S1e2.3.7's N3, the user's ruling of 2026-11-03.
///
/// The line spelt the last firing as a Python repr, `Firing(rule='transitive',
/// …, Fact(relation_name='co-located', …))`, sent the reader to a
/// `Saturator._last_firing` no crate defines, and said *without reaching fixed
/// point* on `colocation-chain.ein` at `--max-steps 1`, whose one firing is
/// the whole saturation. The help said it *raises SaturatorStepLimitError*.
/// A step limit leaves the KB where it stopped, which is what the flag asks
/// for, so nothing goes to stderr and the exit code is 0 (`read_out.md` § 9).
#[test]
fn the_step_limit_is_one_line_in_eins_spelling_and_exits_0() {
    const LINE: &str = "!! saturator step limit hit: ";
    const CHAIN: &str = "examples/saturation/transitive/colocation-chain.ein";
    let limit = |args: &[&str]| -> (Run, Vec<String>) {
        let mut a = vec!["saturate"];
        a.extend(args);
        let r = ein(&a);
        assert_eq!(r.code, 0, "{args:?}: {}", r.err);
        assert!(r.err.is_empty(), "{args:?} wrote to stderr:\n{}", r.err);
        let lines = r
            .out
            .lines()
            .filter(|l| l.starts_with(LINE))
            .map(|l| l[LINE.len()..].to_string())
            .collect();
        (r, lines)
    };
    let (one, l) = limit(&["--max-steps", "1", CHAIN]);
    assert_eq!(
        l,
        [
            "--max-steps 1 stopped saturation after 1 firing, before it checked for a fixpoint. \
             The last firing: (transitive co-located) with ?rel = co-located · ?a = A · ?b = B · \
             ?c = C derived (co-located A C) from (co-located A B) (co-located B C)"
        ]
    );
    assert!(one.out.contains("── per-rule firing breakdown at limit ──"));
    // The same one firing is the whole saturation: the line may not say the
    // KB fell short of its fixpoint.
    let (all, none) = limit(&[CHAIN]);
    assert!(none.is_empty(), "{none:?}");
    assert_eq!(all.field("total firings").as_deref(), Some("1"));
    assert_eq!(one.field("total firings").as_deref(), Some("1"));
    // A redundant last firing re-derived what the KB held.
    let (_, l) = limit(&["--max-steps", "5", "tests/stdlib/algebra/03_boolean.ein"]);
    assert_eq!(
        l,
        [
            "--max-steps 5 stopped saturation after 5 firings, before it checked for a fixpoint. \
             The last firing: (join-r owns rents either) with ?R1 = owns · ?R2 = rents · \
             ?R3 = either · ?a = Ann · ?b = Car re-derived (either Ann Car) from (rents Ann Car), \
             which the KB already held"
        ]
    );
    // 0 and a negative stop before the first firing, as typed.
    for (arg, n) in [("--max-steps=0", "0"), ("--max-steps=-3", "-3")] {
        let (_, l) = limit(&[arg, CHAIN]);
        assert_eq!(
            l,
            [format!(
                "--max-steps {n} stopped saturation after 0 firings, before it checked for a \
                 fixpoint; nothing fired, so there is no last firing"
            )]
        );
    }
    let help = ein(&["saturate", "--help"]);
    assert!(
        help.ok().out.contains("Not an error: the exit code is 0."),
        "{}",
        help.out
    );
    for text in [&one.out, &help.out] {
        for python in [
            "Firing(",
            "Fact(",
            "_last_firing",
            "SaturatorStepLimitError",
            "fixed point",
        ] {
            assert!(!text.contains(python), "`{python}` is back:\n{text}");
        }
    }
}

/// **A conclusion no fact can hold stops the run with one line in Ein's
/// spelling, the same from every command** — M1e2 S1e2.2.18.
///
/// The line was `TypeError: expected NestedPattern at :assert top-level, got
/// Atom` or `TypeError: an unrecognised `:assert` slot has no value to store`
/// under `ein saturate`, and the message alone under `ein solve`: a Python
/// class, a Python IR class's name, and no word of which rule or what it
/// wrote. A string reached it until S1e2.2.18 lowered one (W18), a range
/// until S1e2.2.20 refused one in a rule at load (W20, the next test), and a
/// keyword pair inside a nested conclusion and a relation written as a
/// variable no parameter binds, `(?x a b)` and `(tag a (?x b))`, until
/// S1e2.2.24 refused both at load (W24, the test after); these four shapes
/// still do, each a program the loader accepts and the rule's first firing
/// stops, exit 1 as before. The line names the rule and the slot as the
/// program writes it.
#[test]
fn a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling() {
    const ARG: &str = "and no fact can hold it — an argument is a name, a string, an int, a \
                       variable its :match binds, or a fact (relation argument …)";
    const TOP: &str = "which is not a fact — a conclusion is a fact (relation argument …), or \
                       an (and …) of facts";
    let scratch = ein_corpus::Scratch::new("ein-not-a-fact");
    let cases: [(&str, String); 4] = [
        (
            r#"(tag ?x b :source "x")"#,
            format!(r#"its :assert writes :source "x" as an argument of tag, {ARG}"#),
        ),
        (
            "(tag ?x _)",
            format!("its :assert writes _ as an argument of tag, {ARG}"),
        ),
        ("foo", format!("its :assert concludes foo, {TOP}")),
        (
            "(and (tag ?x b) ?x)",
            format!("its :assert concludes ?x, {TOP}"),
        ),
    ];
    for (i, (assert, line)) in cases.iter().enumerate() {
        let file = scratch.path().join(format!("{i}.ein"));
        std::fs::write(
            &file,
            format!(
                "(relation holds Node) (relation tag Node Node)\n(holds a)\n\
                 (rule s () :match (holds ?x) :assert {assert})\n"
            ),
        )
        .expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("rule 's': {line}");
        for cmd in ["saturate", "solve"] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, 1, "ein {cmd} on :assert {assert}:\n{}", r.err);
            assert_eq!(
                r.err.lines().rfind(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} on :assert {assert}"
            );
            for python in ["TypeError", "NestedPattern", "SForm", "unrecognised"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
    // A string reaches it no more: it is the symbol it names.
    let file = scratch.path().join("string.ein");
    std::fs::write(
        &file,
        "(relation holds Node) (relation tag Node Node)\n(holds a)\n\
         (rule s () :match (holds ?x) :assert (tag ?x \"lbl\"))\n",
    )
    .expect("the program is written");
    let r = ein(&["saturate", "--dump", &file.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("  (tag a lbl :rule s)\n"), "{}", r.out);
}

/// **A wildcard written as a relation is refused at load** — M1e2 S1e2.2.20
/// (W20), on the user's ruling of 2026-11-05 (Q-M1e2.14 (c)).
///
/// One line in Ein's spelling, naming the rule and the slot, from every
/// command that loads the program: `ein saturate`, `ein solve` and `ein
/// oracle` exit 1 and `ein test` 2, a load error's codes. Until then a
/// wildcard head stopped compilation with `unbound relation head
/// ?Wildcard() in a premise of `SForm(…)``, the Python IR's spelling. A range
/// in a rule was refused here too, from S1e2.2.20 (Q-M1e2.13 (a)) until M1f
/// S1f.5.3 retired the token; the test below holds what it is since.
#[test]
fn a_wildcard_relation_in_a_rule_is_refused_at_load() {
    const HEAD: &str = "with the wildcard _ as its relation — a relation is named by a name or \
                        by a parameter its activator binds, and _ stands for an argument";
    let scratch = ein_corpus::Scratch::new("ein-refused-at-load");
    let cases = [
        (
            ":match (_ 2) :assert (hit r)",
            format!("rule 'r': its :match writes (_ 2) {HEAD}"),
        ),
        (
            ":match (box (_ 2)) :assert (hit r)",
            format!("rule 'r': its :match writes (_ 2) {HEAD}"),
        ),
        (
            ":match (absent (_ b)) :assert (hit r)",
            format!("rule 'r': its :match writes (_ b) {HEAD}"),
        ),
    ];
    for (i, (clauses, line)) in cases.iter().enumerate() {
        let file = scratch.path().join(format!("{i}.ein"));
        std::fs::write(
            &file,
            format!(
                "(relation n Node) (relation hit Node) (relation box Node) (relation tag Node Node)\n\
                 (n 2) (box (n 2))\n(rule r () {clauses})\n"
            ),
        )
        .expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("kb load error: {line}");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {clauses}:\n{}", r.err);
            assert_eq!(
                r.err.lines().rfind(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} on {clauses}"
            );
            for python in ["CompileError", "Wildcard()", "SForm"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
}

/// **No range is read anywhere: `1..5` stops the parse at its `.`** — M1f
/// S1f.5.3, on the user's ruling of 2026-11-06 (Q-M1e2.24): Ein has no
/// numbers and no ranges. The `RANGE` token is gone, so `1` is an `INT` and
/// `..5` no token, in a fact, a keyword's value, a declaration, a rule's
/// `:match` and `:assert`, and a `(config …)`: every command that reads the
/// program refuses it with the parser's line at the `.`, exit 1 (`ein test`
/// 2). Until then a range was a token: in a fact it stored the name `1..5`,
/// as a keyword's value it was dropped, and in a rule the loader refused it
/// (M1e2 S1e2.2.20). From M1f S1f.5.4, `1..5` is one name.
#[test]
fn a_range_is_no_token() {
    let scratch = ein_corpus::Scratch::new("ein-no-range");
    let pre = "(relation n Node) (relation hit Node) (relation r Node Node)\n(n a)\n";
    let cases = [
        ("(r 1..5 a)", 5),
        ("(r a b :card 1..*)", 15),
        ("(relation q Node Node :card 1..*)", 30),
        ("(rule x () :match (n 1..3) :assert (hit a))", 23),
        ("(rule x () :match (n ?v) :assert (r ?v 2..*))", 41),
        ("(config :lattice-sanity-check 1..5)", 32),
    ];
    for (i, (form, col)) in cases.iter().enumerate() {
        let file = scratch.path().join(format!("{i}.ein"));
        std::fs::write(&file, format!("{pre}{form}\n")).expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("{file}:3:{col}: unexpected input");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {form}:\n{}", r.err);
            assert_eq!(
                r.err.lines().find(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} on {form}"
            );
        }
    }
}

/// **A rule shape that reads nothing is refused at load** — M1e2 S1e2.2.24
/// (W24), on the user's rulings of 2026-11-05 (Q-M1e2.16 (1), Q-M1e2.18).
///
/// Each of W24's seven witnesses, from every command that loads it: `ein
/// saturate`, `ein solve` and `ein oracle` exit 1 and `ein test` 2, with the
/// loader's line in Ein's spelling. Until then each loaded and read nothing,
/// in silence: a guard on `_`, a form, a keyword pair or a variable no
/// premise before it binds compared as a raw node or a null; a premise that is
/// not a form was dropped; a pattern inside a pattern whose relation no
/// parameter binds, or whose activator bound it to a nested fact, matched
/// nothing. The three shapes an `:assert` writes, which stopped the firing
/// with the line the test above holds, are refused at load too, and a
/// top-level `(?r b)` whose `CompileError` printed the Python IR's spelling.
/// A parameter a rule binds to a nested fact through a variable is the
/// compiler's (`ein-infer/tests/compile_semantics.rs`).
#[test]
fn a_rule_shape_that_reads_nothing_is_refused_at_load() {
    const GUARD: &str = "a guard compares two values, each a variable a premise before it \
                         binds, a name, a string or an integer";
    const WILD: &str = "and _ names no value — a guard compares two values, and _ stands for \
                        an argument of a pattern";
    const PREMISE: &str = "a premise is a pattern (relation argument …), a guard (eq a b) or \
                           (neq a b), an (absent …), or an (and …) or (or …) of premises";
    let witnesses: [(&str, String); 7] = [
        (
            "wild-in-guard",
            format!(
                "rule 'e': its :match writes (eq ?x _), {WILD}; \
                 rule 'n': its :match writes (neq ?x _), {WILD}; \
                 rule 'b': its :match writes (eq _ _), {WILD}"
            ),
        ),
        (
            "bare-premise",
            format!(
                "rule 'a': its :match is _, and _ is no premise — {PREMISE}; \
                 rule 'b': its :match writes _ as a premise of and, and _ is no premise — {PREMISE}; \
                 rule 'c': its :match is foo, and foo is no premise — {PREMISE}"
            ),
        ),
        (
            "nested-unbound-head",
            "rule 'r': its :match writes (?r b) with ?r as its relation, which no parameter \
             binds — a relation is named by a name or by a parameter its activator binds"
                .to_string(),
        ),
        (
            "form-in-guard",
            format!(
                "rule 'e': its :match writes (eq ?x (f a)), and the form (f a) is no value — {GUARD}; \
                 rule 'n': its :match writes (neq ?x (f a)), and the form (f a) is no value — {GUARD}"
            ),
        ),
        (
            "guard-before-binder",
            format!(
                "rule 'e': its :match writes (eq ?x a), and no premise before it binds ?x — {GUARD}; \
                 rule 'n': its :match writes (neq ?x a), and no premise before it binds ?x — {GUARD}"
            ),
        ),
        (
            "kwpair-in-guard",
            format!(
                "rule 'e': its :match writes (eq ?x :k a), and the keyword pair :k a is no value — {GUARD}"
            ),
        ),
        (
            "nested-fact-activator",
            "rule 'r': its parameter ?R names a relation, and (r (a b)) binds it to the fact \
             (a b) — a relation is named by a symbol, not a fact"
                .to_string(),
        ),
    ];
    for (name, line) in &witnesses {
        let file = format!("examples/ein-bugs/wrong-answers/{name}.ein");
        let want = format!("kb load error: {line}");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} {file}:\n{}", r.err);
            assert_eq!(
                r.err.lines().rfind(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} {file}"
            );
            for python in ["CompileError", "Wildcard()", "SForm", "Fact("] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
    // The `:assert` shapes the test above held at the firing, and the
    // top-level relation variable, in Ein's spelling at load.
    let scratch = ein_corpus::Scratch::new("ein-reads-nothing");
    for (i, (assert, line)) in [
        (
            "(tag ?x (box a :k v))",
            "its :assert writes :k v as an argument of box, and no fact can hold it — a \
             keyword pair is no value, so a pattern inside a pattern that holds one matches \
             nothing",
        ),
        (
            "(?x a b)",
            "its :assert writes (?x a b) with ?x as its relation, which no parameter binds — \
             a relation is named by a name or by a parameter its activator binds",
        ),
        (
            "(tag a (?x b))",
            "its :assert writes (?x b) with ?x as its relation, which no parameter binds — a \
             relation is named by a name or by a parameter its activator binds",
        ),
    ]
    .iter()
    .enumerate()
    {
        let file = scratch.path().join(format!("{i}.ein"));
        std::fs::write(
            &file,
            format!(
                "(relation holds Node) (relation tag Node Node)\n(holds a)\n\
                 (rule s () :match (holds ?x) :assert {assert})\n"
            ),
        )
        .expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("kb load error: rule 's': {line}");
        for cmd in ["saturate", "solve"] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, 1, "ein {cmd} on :assert {assert}:\n{}", r.err);
            assert_eq!(
                r.err.lines().rfind(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} on :assert {assert}"
            );
        }
    }
}

/// **`-j/--jobs` takes a count or the word `auto`, and refuses everything
/// else** — [S1a.7.5](../../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)
/// T1a.7.5.1.
///
/// What it asserts is the contract: the read-out at two jobs and at `auto` is
/// the one-job read-out, byte for byte, and `-j` is one of the options the
/// header leaves out for that reason. `--jobs 0` is refused on purpose: it is
/// the sentinel `auto` parses to, and letting a user type it would give one
/// meaning two spellings.
#[test]
fn jobs_takes_a_count_or_auto_and_nothing_else() {
    let one = ein(&["solve", TWO_MODELS]);
    one.ok();
    for spec in ["2", "auto"] {
        let r = ein(&["solve", TWO_MODELS, "--jobs", spec]);
        r.ok();
        assert_eq!(r.out, one.out, "--jobs {spec} moved the read-out");
    }
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

/// **The `work` row is the summary's `work` block**, and `-s` adds the wall
/// clock under it.
///
/// Two renderings of one count is exactly where a drift hides. Since M1e2
/// S1e2.1a.5 the walk's counters went with it, and what a run reports about
/// its own work is the read-out's `work` row — saturations, facts beyond
/// root, route-1 layers — which `--json-summary` writes as `work`.
#[test]
fn the_work_row_is_the_summarys_work_block() {
    let scratch = Scratch::new("stats");
    let path = scratch.at("summary.json");
    let r = ein(&["solve", TWO_MODELS, "-s", "--json-summary", &path]);
    r.ok();
    let j = json_at(&path);
    let row = r
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("work "))
        .expect("the work row")
        .to_string();
    let want = format!(
        "{} saturations · {} facts beyond root · {} layers",
        j["work"]["saturations"], j["work"]["facts"], j["work"]["layers"]
    );
    assert!(row.ends_with(&want), "{row} against {want}");
    assert!(r.field("wall").is_some(), "-s printed no wall-clock row");
}

/// **`--timing` covers every phase.** No cost may vanish from the accounting:
/// a phase that stopped being reported does not look slow, it looks free.
#[test]
fn the_timing_table_names_every_phase() {
    let r = ein(&["solve", TWO_MODELS, "--timing"]);
    r.ok();
    let table = r
        .out
        .split_once("\ntiming\n")
        .expect("no timing table in the output")
        .1;
    for phase in ["load", "route", "read-out", "print", "total"] {
        assert!(
            table.lines().any(|l| l.trim_start().starts_with(phase)),
            "the {phase:?} phase is missing from:\n{table}"
        );
    }
}

/// **The trace goes to a file and the answer to stdout.** They do not mix.
///
/// `--trace FILE` is a *file* flag, and the reason is redirection: the trace
/// is a markdown document with fenced DOT blocks in it, and a caller piping
/// `ein solve` into anything at all would otherwise get the document instead
/// of the read-out.
#[test]
fn the_trace_goes_to_a_file_and_the_answer_to_stdout() {
    let scratch = Scratch::new("trace");
    let path = scratch.at("trace.md");
    let r = ein(&["solve", TWO_MODELS, "--trace", &path]);
    r.ok();
    let trace = read(&path);
    assert!(trace.starts_with("# Region trace"), "{trace}");
    assert!(trace.contains("```dot"), "the trace has no fenced diagram");
    assert!(!r.out.contains("```"), "a fence reached stdout:\n{}", r.out);
    assert!(
        r.out.contains("states (k_s)"),
        "stdout lost the read-out:\n{}",
        r.out
    );
}

// ── `--json-summary` ────────────────────────────────────────────────

/// **The summary declares its schema, and its blocks have a fixed shape.**
///
/// One object per run, and `schema` first: a consumer that reads
/// `ein-summary/2` knows what the rest means, and one that reads something
/// else can refuse instead of guessing. `root.hypgen`'s first four keys are
/// the invariant behind them — `raw == emitted + Σ filtered` — and since M1e2
/// S1e2.2.8 `outside`, what the universe leaves out, is the fifth, and since
/// S1e2.2.10 `types_as_objects`, a type the untyped rung binds, the sixth.
#[test]
fn the_json_summary_declares_its_schema_and_its_block_shape() {
    let scratch = Scratch::new("schema");
    let path = scratch.at("summary.json");
    ein(&["solve", TWO_MODELS, "--json-summary", &path]).ok();
    let j = json_at(&path);
    assert_eq!(j["schema"], "ein-summary/2");
    assert_eq!(j["source"], TWO_MODELS);
    for block in ["asked", "region", "work", "root", "config", "leftover"] {
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
        [
            "emitted",
            "filtered",
            "outside",
            "pre_candidate",
            "raw",
            "types_as_objects",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<BTreeSet<String>>(),
        "root.hypgen's key set moved"
    );
    assert!(
        j["root"]["plans"].as_i64().unwrap_or(0) > 0,
        "a file with rules compiled no plans"
    );
    assert_eq!(j["region"]["k_s"], 7);
}

/// **An integer goal binding stays a JSON *number*** — at any width.
///
/// A goal binding is the *stored* argument, so an IR `INT` is an integer and
/// writing it as `"8"` is a type error a consumer cannot undo. S1a.6.6's
/// fuzzer found this on a two-line program: stdout was right, so only the
/// summary showed it. The wide literal is the second half — the IR's
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
    // Since M1e2 S1e2.1a.5 a goal's rows are its query's projection classes'.
    let rows = j["queries"][0]["classes"][0]["rows"]
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
/// The sibling of the integer case: a binding that is itself a fact is
/// written as its s-expression, and a run that binds one still writes its
/// summary rather than failing without one.
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
    let rows = j["queries"][0]["classes"][0]["rows"]
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
        "--maximal",
        "-p",
        "--json-summary",
        &scratch.at("s.json"),
        "--events",
        &scratch.at("e.jsonl"),
    ]);
    let without = ein(&["solve", TWO_MODELS, "--maximal", "-p"]);
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
    ein(&["solve", TWO_MODELS, "--json-summary", &a]).ok();
    ein(&["solve", TWO_MODELS, "--json-summary", &b]).ok();
    assert_eq!(read(&a), read(&b), "two runs wrote different bytes");

    let j = json_at(&a);
    let facts: Vec<&str> = j["region"]["states"][1]["facts"]
        .as_array()
        .expect("a state's facts")
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

/// **A stopped run still writes its summary**, and says why it stopped.
///
/// The run that most needs a machine-readable record is the one that did not
/// finish. Since M1e2 S1e2.1a.5 a budget stops a run with an answer — what it
/// established, and bounds — so it exits 0, writes one `note:` on stderr, and
/// its summary says `exhausted false` and names the budget. A `-T` abort is
/// the one stop that is not an answer: it exits 2 and writes no summary,
/// because nothing was established.
#[test]
fn a_stopped_run_writes_its_summary_and_says_why() {
    let scratch = Scratch::new("abort");
    let path = scratch.at("summary.json");
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "--region-budget",
        "1",
        "--json-summary",
        &path,
    ]);
    assert_eq!(r.code, 0, "a budget stop is an answer: {}", r.err);
    assert!(
        r.err.starts_with("note: ") && r.err.contains("partial"),
        "{}",
        r.err
    );
    let j = json_at(&path);
    assert_eq!(j["region"]["exhausted"], false);
    assert_eq!(j["region"]["stopped"]["why"], "budget");

    let deadline = scratch.at("deadline.json");
    let t = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-T",
        "0",
        "--json-summary",
        &deadline,
    ]);
    assert_eq!(t.code, 2, "a -T abort is exit 2, not {}", t.code);
    assert!(
        !PathBuf::from(&deadline).exists(),
        "a -T abort wrote a summary"
    );
}

// ── `--events` ──────────────────────────────────────────────────────

/// **The stream ends in `region`, and `n` is dense.**
///
/// Both halves are about being able to say *where* two logs diverge. The
/// sequence number is dense from 0 to the last event, so "the first difference
/// is at event k" identifies one event rather than a range; and the last event
/// is the `region` line, carrying what was asked, R as the run established
/// it and the work — so a truncated log is detectable and a complete one needs
/// no second artefact to interpret. It was the walk's `verdict` until M1e2
/// S1e2.1a.5.
#[test]
fn the_event_stream_is_dense_and_ends_in_the_region() {
    let scratch = Scratch::new("dense");
    let path = scratch.at("events.jsonl");
    ein(&["solve", TWO_MODELS, "--events", &path]).ok();
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
    assert_eq!(last["e"], "region");
    assert_eq!(last["asked"]["which"], "all");
    assert_eq!(last["region"]["k_s"], 7);
    assert_eq!(last["region"]["exhausted"], true);
    assert!(last["work"].is_object(), "no work on the region line");
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
/// A six-file cover rather than the whole corpus: the sweep is 0.06 s this
/// way and 40 s the other, and the question — *is any kind unreachable?* — is
/// answered as well by a cover as by a sweep. What the cover cannot do is
/// notice a kind that stopped being emitted by a file **not** listed here,
/// which is why each entry names its reason.
///
/// It had a third column, the environment a row needed, for the walk's tree;
/// a kind reachable only under an environment variable was a kind no
/// default-environment sweep can see, which is how `traversal` shipped at M1d
/// S1d.10.6 and reached M1e undocumented (`SE-M3`).
const EVENT_COVER: [(&str, &str); 6] = [
    (
        "examples/branching/01_saturate_only.ein",
        "the lifecycle and the deductive layer, `enter`, `layer` and `region`",
    ),
    (
        "examples/branching/04_two_levels.ein",
        "hyp / hypskip — how H is built, narrated since M1e2 S1e2.1a.5",
    ),
    (
        "examples/branching/12_typed_blind_solve.ein",
        "park / admit / retire — the NAF boundary, and alt",
    ),
    (
        "examples/features/06_symmetric_native.ein",
        "mirror — the native arg swap",
    ),
    (
        "examples/zebra2-obligations.ein",
        "rung and owe — the obligations rung choosing H, and the \
         post-fixpoint obligation pass",
    ),
    (
        "examples/lattice/04_obligation_trigger_hole.ein",
        "monotonicity — a site, an obligation's trigger. `naf-upward-closure.ein` \
         gave it, and `warn`, until M1e2 S1e2.2.5: *forbid* refuses that \
         program before the static check is narrated, and `warn` left the \
         schema with `(config :warn-derived-naf true)`, the flag that gated it. \
         The `alive-set-invariant` category had a row of its own, \
         `alive-set-fresh-name.ein`, until S1e2.2.4 seeded the universe with a \
         rule's constants",
    ),
];

/// Where the `--events` schema lives: the protocol is a product surface — a
/// debugging tool, and M20's likely feed.
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

/// **Every kind the schema defines is emitted by some corpus fixture.** The
/// walk's three, which no command emitted since M1e2 S1e2.1a.5 and this test
/// held unreached, left the schema with the walk at S1e2.1c.5.
///
/// A kind nothing reaches is a kind the engine can silently stop emitting
/// while the tier that should catch it stays green — the same shape of problem
/// as a test that skips. The parse of `EVENTS.md` is what keeps the cover
/// honest as the schema grows: a new row with no fixture behind it fails here.
#[test]
fn every_event_kind_the_schema_defines_is_reachable_from_the_corpus() {
    let scratch = Scratch::new("cover");
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for (i, (file, _why)) in EVENT_COVER.iter().enumerate() {
        let path = scratch.at(&format!("cover-{i}.jsonl"));
        // A budget, so the large fixtures stay fast; the kinds they are here
        // for are all emitted well before it.
        ein(&[
            "solve",
            file,
            "--region-budget",
            "4096",
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
    // The row count: the three payload tables have 17 kind cells (the walk's
    // `verdict` left at M1e2 S1e2.1a.5 and `region` arrived at T1e2.1a.5.2;
    // its `nogood`, `writeback` and `traversal` left at S1e2.1c.5, and `warn`
    // with its flag at S1e2.2.5), so a removed kind has to be typed here too.
    assert!(
        schema.len() >= 17,
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
/// six times the volume. A *redundant* firing is precisely what a change can
/// drop without changing any answer, so the level that hides them is the
/// level that hides the bug.
#[test]
fn the_events_level_gates_the_high_volume_kinds() {
    let scratch = Scratch::new("level");
    let (n, v) = (scratch.at("normal.jsonl"), scratch.at("verbose.jsonl"));
    ein(&["solve", TWO_MODELS, "--events", &n]).ok();
    ein(&[
        "solve",
        TWO_MODELS,
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
/// produce a *shorter log that looks complete*, and a comparison of two such
/// logs would pass because both sides were missing the same events.
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
/// The engine has no salted hash, so re-running a file proves nothing about
/// order; the question is posed in this engine's own terms, because its ids
/// are **assignment-ordered** — a `Symbol` is "how many distinct names had
/// been seen when this one arrived". Perturbing that is the experiment.
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
        // `ein solve FILE --events`: a describe, whose stream narrates root and
        // every node the route visits (the walk's stream until M1e2 S1e2.1c.5).
        let opts = ein_infer::explore::Options::new(ein_infer::explore::Mode {
            which: ein_infer::explore::Which::All,
            ask: ein_infer::explore::Ask::Describe,
        });
        ein_infer::explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
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

// ── The editor grammar ──────────────────────────────────────────────

/// The TextMate grammar mirrors three *closed* reserved-name sets in head
/// position. A stray copy that drifts from the registry is the failure mode
/// these four tests exist for.
const GRAMMAR: &str = "utils/vscode-ein/ein.tmLanguage.json";

/// The closed declarator set — P1.7c, plus S1.5.9's `macro`, S1.8.A2's
/// `import` and M1e2 S1e2.1a.4's `solve`. Source of truth:
/// [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md).
const DECLARATORS: [&str; 9] = [
    "config", "hrule", "import", "macro", "query", "relation", "rule", "solve", "trace",
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

/// Every `(a|b|c)` group of lowercase words in a regex —
/// `\(([a-z][a-z*-]*(?:\|[a-z][a-z*-]*)*)\)`, hand-scanned because
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

/// The `--layer-progress` rows on stderr: `layer N: V visited — D dead, O
/// owing, R in R · F facts`, each as its layer and its five counts.
fn layer_lines(err: &str) -> Vec<(u64, [u64; 5])> {
    let mut out = Vec::new();
    for l in err.lines() {
        let Some(rest) = l.strip_prefix("layer ") else {
            continue;
        };
        let nums: Vec<u64> = rest
            .split(|c: char| !c.is_ascii_digit())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse().ok())
            .collect();
        if let [layer, visited, dead, owing, r, facts] = nums[..] {
            out.push((layer, [visited, dead, owing, r, facts]));
        }
    }
    out
}

/// **The layer rows' arithmetic closes**, and adds up to the `work` row.
///
/// The flag exists so a reader can watch a run that takes minutes, and a
/// progress line nobody can add up is a progress line nobody can trust. Since
/// M1e2 S1e2.1a.5 a row is a route's layer — since S1e2.1c.2 the state
/// route's on a monotone program: root's, then one per state size extended.
/// Every node visited is dead, owing or in R, and the layers' visits and facts
/// are the run's saturations and facts.
#[test]
fn the_layer_progress_rows_add_up() {
    let r = ein(&["solve", "--layer-progress", TWO_MODELS]);
    assert_eq!(r.code, 0, "stderr:\n{}", r.err);
    let rows = layer_lines(&r.err);
    assert_eq!(rows.len(), 4, "four layers:\n{}", r.err);
    for (i, (layer, [visited, dead, owing, in_r, _])) in rows.iter().enumerate() {
        assert_eq!(*layer, i as u64, "the layers are not in order");
        assert_eq!(
            *visited,
            dead + owing + in_r,
            "layer {i}: a node is none of the three"
        );
    }
    let work = r
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("work "))
        .expect("the work row")
        .to_string();
    let visited: u64 = rows.iter().map(|(_, c)| c[0]).sum();
    let facts: u64 = rows.iter().map(|(_, c)| c[4]).sum();
    assert!(
        work.contains(&format!(
            "{visited} saturations · {facts} facts beyond root"
        )),
        "{work} against {visited} visits and {facts} facts"
    );
    assert!(
        rows.iter().any(|(_, c)| c[1] > 0),
        "the fixture has to kill something"
    );
}

/// **It is additive, and it is the layer half of `--verbose` alone.**
///
/// The flag changes what goes to *stderr* and nothing else, and what it
/// removes from `-v` is the per-node lines, which is what makes `-v` hard to
/// read while watching a layer.
#[test]
fn layer_progress_is_verbose_without_the_nodes() {
    let plain = ein(&["solve", TWO_MODELS]);
    let lp = ein(&["solve", "--layer-progress", TWO_MODELS]);
    let v = ein(&["solve", "-v", "-g", "1", TWO_MODELS]);
    assert_eq!(plain.out, lp.out, "--layer-progress moved stdout");
    assert_eq!(plain.out, v.out, "-v moved stdout");
    assert_eq!(plain.code, lp.code);
    assert!(
        plain.err.is_empty(),
        "the control wrote stderr: {}",
        plain.err
    );
    let node = |e: &str| e.lines().filter(|l| l.starts_with("  node ")).count();
    assert_eq!(node(&lp.err), 0, "--layer-progress narrated a node");
    assert!(
        node(&v.err) > 0,
        "-v narrated none — the contrast is vacuous"
    );
    assert_eq!(
        layer_lines(&lp.err),
        layer_lines(&v.err),
        "the two disagree about the layer half"
    );
}

// ── An empty count qualifies itself ────────────────────────────────

/// **A run that stopped does not say R is empty**, and one that exhausted
/// says which kind of empty it is — M1d T1d.10.5.2b's rule, read under R.
///
/// `saturation/type-exclusivity/pets.ein` is the fixture because the walk
/// called it *contradictory* at `-m 5` and it has 11 339 states. Stopped by
/// its budget before layer 1, a description has R's lower bounds and no
/// `empty` row, and a dead-alone count only of what it visited. The control,
/// `ein-bugs/zebra2-bad.ein`, is an exhausted empty R with a real one-fact
/// core, which keeps every word.
#[test]
fn a_stopped_run_does_not_report_r_empty() {
    let cut = ein(&[
        "solve",
        "--region-budget",
        "2",
        "examples/saturation/type-exclusivity/pets.ein",
    ]);
    assert_eq!(cut.code, 0);
    assert!(
        cut.out.contains("states (k_s)      1   (a lower bound"),
        "{}",
        cut.out
    );
    assert!(
        !cut.out.contains("\n  empty "),
        "a stopped run called R empty:\n{}",
        cut.out
    );
    assert!(
        cut.out
            .contains("dead alone not known: the run stopped before it visited each alone"),
        "{}",
        cut.out
    );
    assert!(
        cut.err.starts_with("note: ") && cut.err.contains("partial"),
        "{}",
        cut.err
    );

    let done = ein(&["solve", "examples/ein-bugs/zebra2-bad.ein"]);
    assert_eq!(done.code, 0);
    assert_eq!(done.field("exhausted").as_deref(), Some("yes"));
    assert!(
        done.out
            .contains("empty             every node is dead — root's own fixpoint derives (false)"),
        "the exhausted arm moved:\n{}",
        done.out
    );
    assert!(
        done.out
            .contains("unsat core        (color-loc Green House-1)")
    );
}

// ── M1e S1e.3.5 — EH-M2: the override proves itself ────────────────

/// **The refusal reaches a user through the environment**, which is the half
/// an in-process test cannot show.
///
/// `ein-ir`'s `an_override_without_the_marker_is_refused_by_name` drives
/// `Resolver::with_stdlib` directly; this is the wiring — `$EIN_STDLIB` is
/// read, the message names it, and the exit code is the loader's 1.
#[test]
fn a_stdlib_override_without_the_marker_is_refused_through_the_environment() {
    let scratch = Scratch::new("bad-stdlib");
    let dir = scratch.at("not-a-stdlib");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let r = ein_env(&[("EIN_STDLIB", &dir)], &["solve", "examples/zebra2.ein"]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(
        r.err.contains("$EIN_STDLIB") && r.err.contains("MANIFEST.sha256"),
        "{}",
        r.err
    );
    // The control: the repo's own stdlib, through the same variable.
    let real = repo_root().join("stdlib");
    let ok = ein_env(
        &[("EIN_STDLIB", &real.to_string_lossy())],
        &["solve", "examples/features/10_expect.ein"],
    );
    assert_eq!(ok.code, 0, "{}{}", ok.out, ok.err);
}

// ── The container sniff, in either feature set — M1e S1e.4.4, `EH-L2` ──

/// **A file that starts like a container but is not one is text in either
/// build.**
///
/// `common.rs` sniffed **five** bytes where `ein_einb::is_einb` requires
/// **eight**, so `EINB\0xyz` was *"a `.einb` container and this build has no
/// `einb` feature"* under `--no-default-features` and a UTF-8 or parse error
/// under the default one — two shipped feature sets disagreeing on garbage
/// input, on the surface a generated program or a pipeline hits first.
///
/// There is one predicate now (`common::looks_like_einb`) and one constant
/// (`common::EINB_MAGIC`, compared against `ein_einb::header::MAGIC` by
/// `einb_cli::the_two_magic_constants_agree`). This is the **behaviour** half,
/// and it is deliberately not in `einb_cli.rs`, which is
/// `#![cfg(feature = "einb")]`: what it asserts is that the two builds agree,
/// so it has to run in both.
///
/// The five-byte prefix is the whole point of the fixture. A file with the
/// **full** eight-byte magic legitimately parts ways between the two builds —
/// one opens it, the other says it cannot — and that divergence is the
/// feature, not the bug.
#[test]
fn a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build() {
    let dir = Scratch::new("einb-sniff");
    let path = dir.at("five.ein");
    // `EINB\0` — the old sniff's five bytes — and then not the magic.
    std::fs::write(&path, b"EINB\0xyz\n").expect("writes");

    for sub in ["solve", "saturate", "test"] {
        let r = ein(&[sub, &path]);
        assert!(
            !r.err.contains("has no `einb` feature"),
            "`ein {sub}` called a five-byte prefix a container: {:?}",
            r.err
        );
        assert!(
            r.err.contains("unexpected input"),
            "`ein {sub}` should have read it as text and failed to parse it: {:?}",
            r.err
        );
    }
}

/// **The summary escapes non-ASCII where the event stream does not** — M1e
/// S1e.4.8, `MA-L3`.
///
/// `--events` emits literal UTF-8, `--json-summary` emits `\uXXXX`. The
/// difference is **accepted**, with the reason at `summary::write` — and this
/// is what holds the premise the acceptance rests on, since the argument is
/// *every consumer parses the summary, so the encoding is invisible to all of
/// them*.
///
/// The carrier is an obligation `:why`, which is where a template string
/// reaches both artefacts of one run. The assertions are about **encoding**
/// and not about wording, so `stdlib/slots.ein`'s sentence can change freely.
#[test]
fn the_summary_escapes_non_ascii_where_the_event_stream_does_not() {
    let dir = Scratch::new("ensure-ascii");
    let summary = dir.at("s.json");
    let events = dir.at("e.jsonl");
    let r = ein(&[
        "solve",
        "tests/stdlib/slots/09_owed_room.ein",
        "--json-summary",
        &summary,
        "--events",
        &events,
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);

    let s = read(&summary);
    let e = read(&events);
    assert!(
        s.is_ascii(),
        "the summary is no longer ASCII-only — if that is intended, \
         `summary::write`'s doc comment is the thing to change first"
    );
    assert!(
        s.contains("\\u"),
        "nothing in the summary was escaped, so this test is vacuous: the \
         fixture stopped carrying a non-ASCII `:why`"
    );
    assert!(
        !e.is_ascii(),
        "the event stream stopped emitting literal UTF-8, which is what \
         `events::strings_escape_json_specials_and_keep_non_ascii` pins"
    );
}

// ── A keyword is a whole word ──────────────────────────────────────

/// **A name that starts with a keyword answers as its twin does, on every
/// command** — M1e2 S1e2.2.22 (W22), on the user's ruling of 2026-11-05 on
/// Q-M1e2.17.
///
/// Each of W22's fourteen witnesses writes a name that starts with a reserved
/// word (`neqw`, `orz`, `andz`, `notw`, `relationship`) in the place the
/// parser read the keyword in; its twin is the same program with the name's
/// first letter doubled (`nneqw`), a name that starts with no keyword and
/// sorts where the witness's does. By value: every command's exit code, its
/// stdout and the first line of its stderr, with the twin's path and name
/// read back as the witness's, the read-out's column padding collapsed and
/// `ein test`'s timing dropped, and `ein saturate --dump` from its dump on.
/// Until the stage the parser split the name, `(neqw ?x)` into the guard
/// `(neq w ?x)`, and every pair differed.
#[test]
fn a_name_that_starts_with_a_keyword_answers_as_its_twin_does() {
    const WITNESSES: [(&str, &str); 14] = [
        ("kw-neq-in-assert", "neqw"),
        ("kw-or-in-assert", "orz"),
        ("kw-and-in-assert", "andz"),
        ("kw-and-in-match", "andz"),
        ("kw-neq-in-match", "neqw"),
        ("kw-neq-in-absent", "neqw"),
        ("kw-neq-in-fact", "neqw"),
        ("kw-not-fact", "notw"),
        ("kw-relation-fact", "relationship"),
        ("kw-neq-in-goal", "neqw"),
        ("kw-or-in-hrule", "orz"),
        ("kw-or-in-obligation", "orz"),
        ("kw-and-in-expect", "andz"),
        ("kw-or-in-solve", "orz"),
    ];
    const RUNS: [&[&str]; 10] = [
        &["solve"],
        &["solve", "--maximal"],
        &["solve", "--minimal"],
        &["solve", "--single"],
        &["solve", "-n", "2"],
        &["solve", "--count"],
        &["solve", "-H"],
        &["oracle"],
        &["saturate", "--dump"],
        &["test"],
    ];
    // `(0.00 s)`, the one timing these runs print (`ein test`'s tally).
    let untimed = |s: &str| -> String {
        let mut out = String::new();
        let mut rest = s;
        while let Some(i) = rest.find('(') {
            out.push_str(&rest[..i]);
            let tail = &rest[i..];
            let close = tail.find(')').unwrap_or(0);
            let inner = &tail[1..close.max(1)];
            if close > 0
                && inner.ends_with(" s")
                && inner[..inner.len() - 2]
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '.')
            {
                rest = &tail[close + 1..];
            } else {
                out.push('(');
                rest = &tail[1..];
            }
        }
        out.push_str(rest);
        out
    };
    let norm = |s: &str, from: (&str, &str), to: (&str, &str), dump: bool| -> String {
        let s = s.replace(from.0, to.0).replace(from.1, to.1);
        let s = if dump {
            s.split_once("SATURATED KB DUMP")
                .map_or(s.clone(), |(_, d)| d.to_string())
        } else {
            s
        };
        untimed(&s)
            .lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let mut bad = Vec::new();
    for (witness, name) in WITNESSES {
        let w = format!("examples/ein-bugs/wrong-answers/{witness}.ein");
        let t = format!("examples/ein-bugs/wrong-answers/{witness}-twin.ein");
        let doubled = format!("{}{name}", &name[..1]);
        for run in RUNS {
            let argv = |file: &str| -> Vec<String> {
                let mut a = vec![run[0].to_string(), file.to_string()];
                a.extend(run[1..].iter().map(|s| s.to_string()));
                a
            };
            let aw = argv(&w);
            let at = argv(&t);
            let rw = ein(&aw.iter().map(String::as_str).collect::<Vec<_>>());
            let rt = ein(&at.iter().map(String::as_str).collect::<Vec<_>>());
            let dump = run.contains(&"--dump");
            let first = |e: &str| e.lines().find(|l| !l.is_empty()).unwrap_or("").to_string();
            let got = (
                rw.code,
                norm(&rw.out, ("", ""), ("", ""), dump),
                first(&rw.err),
            );
            let want = (
                rt.code,
                norm(&rt.out, (&t, &doubled), (&w, name), dump),
                first(&rt.err).replace(&t, &w).replace(&doubled, name),
            );
            if got != want {
                bad.push(format!(
                    "{witness} :: ein {}: exit {} (twin {}), stdout {}, stderr {:?} (twin {:?})",
                    run.join(" "),
                    got.0,
                    want.0,
                    if got.1 == want.1 { "alike" } else { "differs" },
                    got.2,
                    want.2
                ));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} runs differ from the twin's:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

// ── A core search cut at its budget says so — M1e2 S1e2.2.23 (W23) ────────

/// **A core whose search was cut says so on every surface that prints it, and
/// one whose search was not prints nothing new** — M1e2 S1e2.2.23, W23, on
/// the user's ruling of 2026-11-05 (Q-M1e2.16): the core search keeps its
/// budget, `explain::ExplanationBudget`, and a core it cut is qualified as
/// cut, at the cap it stopped at.
///
/// `core-search-cut.ein`'s `(A k)` has 256 environments, past the 64 per
/// fact the search keeps, and its root core is ten facts where eight
/// suffice; `core-search-cut-5.ein`, the same shape with five pairs, is under
/// the budget. Until S1e2.2.23 every surface printed the ten facts with
/// nothing to say the search was cut: `region.rs` kept the search's frontier
/// and dropped `Explanation::exhausted`.
#[test]
fn a_cut_core_says_so_on_every_surface_and_an_uncut_one_does_not() {
    const ROW: &str = "  core search       cut at its budget of 64 environments per fact: a smaller core may exist\n";
    const CELL: &str = "core search cut at its budget of 64 environments per fact";
    const TRACE: &str =
        "Core search: cut at its budget of 64 environments per fact: a smaller core may exist.";
    const TIP: &str =
        "core search: cut at its budget of 64 environments per fact: a smaller core may exist";
    let scratch = ein_corpus::Scratch::new("ein-core-cut");
    let checked = std::cell::Cell::new(0usize);
    for (file, word) in [
        (
            "examples/ein-bugs/wrong-answers/core-search-cut.ein",
            "cut-environments",
        ),
        (
            "examples/ein-bugs/wrong-answers/core-search-cut-5.ein",
            "within-budget",
        ),
    ] {
        let cut = word != "within-budget";
        let dir = scratch.join(if cut { "cut" } else { "under" });
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let at = |name: &str| dir.join(name).to_string_lossy().to_string();
        let says = |surface: &str, text: &str, needle: &str| {
            checked.set(checked.get() + 1);
            assert_eq!(
                text.contains(needle),
                cut,
                "{file}: {surface} {} {needle:?}",
                if cut { "does not print" } else { "prints" }
            );
            if !cut {
                assert!(
                    !text.contains("cut at its budget"),
                    "{file}: {surface} says a search under its budget was cut"
                );
            }
        };
        // The read-out, under each command that prints root's core.
        for args in [
            vec!["solve", file],
            vec!["solve", file, "-H"],
            vec!["oracle", file],
        ] {
            let r = ein(&args);
            r.ok();
            says(&args.join(" "), &r.out, ROW);
        }
        let status = |surface: &str, v: &J| {
            checked.set(checked.get() + 1);
            assert_eq!(v.as_str(), Some(word), "{file}: {surface}'s core_status");
        };
        // `--json-summary`: root's core.
        let summary = at("summary.json");
        ein(&["solve", file, "--json-summary", &summary]).ok();
        status(
            "--json-summary",
            &json_at(&summary)["region"]["core_status"],
        );
        // `--dump-answer`: N, the read-out uncut and the summary.
        let answer = at("answer");
        ein(&["solve", file, "--dump-answer", &answer]).ok();
        let nogoods = json_at(&format!("{answer}/nogoods.json"));
        status("nogoods.json", &nogoods[0]["core_status"]);
        says("answer.txt", &read(&format!("{answer}/answer.txt")), ROW);
        status(
            "answer.json",
            &json_at(&format!("{answer}/answer.json"))["region"]["core_status"],
        );
        // `--dump-lattice`, under each mode: every node the run visits.
        for mode in [None, Some("--maximal"), Some("--single")] {
            let lattice = at(&format!("lattice{}", mode.unwrap_or("")));
            let mut args = vec!["solve", file, "--dump-lattice", &lattice];
            args.extend(mode);
            ein(&args).ok();
            let nodes = json_at(&format!("{lattice}/nodes.json"));
            status("nodes.json", &nodes["nodes"][0]["core_status"]);
            says("nodes.md", &read(&format!("{lattice}/nodes.md")), CELL);
        }
        // `--events`: the `enter` line.
        let events = at("events.jsonl");
        ein(&["solve", file, "--events", &events]).ok();
        let enter: Vec<J> = events_of(&events)
            .into_iter()
            .filter(|e| e["e"] == "enter")
            .collect();
        assert_eq!(enter.len(), 1, "{file}: one node, root");
        status("--events' enter", &enter[0]["core_status"]);
        // `--trace`: N.
        let trace = at("trace.md");
        ein(&["solve", file, "--trace", &trace]).ok();
        says("--trace", &read(&trace), TRACE);
        // `ein oracle --dump`, and `ein render region` drawing it.
        let cube = at("cube");
        ein(&["oracle", file, "--dump", &cube]).ok();
        status(
            "region.json",
            &json_at(&format!("{cube}/region.json"))["region"]["nodes"][0]["core_status"],
        );
        says("region.md", &read(&format!("{cube}/region.md")), CELL);
        says(
            "ein render region",
            &ein(&["render", "region", &cube]).ok().out,
            TIP,
        );
        says(
            "ein render region --view table",
            &ein(&["render", "region", &cube, "--view", "table"])
                .ok()
                .out,
            CELL,
        );
    }
    // Not vacuous: nineteen surfaces a file.
    assert_eq!(checked.get(), 38, "the surfaces checked");
}

// ── A core the run did not keep says so — M1e2 S1e2.3.9 (N6) ─────────────

/// **A member of N a run past `--region-limit` met is printed with no core,
/// and says why; a run that did not stop prints each member's core** — M1e2
/// S1e2.3.9, P1e2.3's N6. Past the limit a describing run keeps each node it
/// met as its two bits (`Space::partial_cube`), so it has no core to print
/// for a member of N: `nogoods.json` writes the member with a `null` core,
/// `null` rules and `core_status: "not-kept"`, and `--trace`'s N says *Core:
/// not kept: the run stopped at `--region-limit`.* Until S1e2.3.9 both wrote
/// an empty core with no rule and no status, which reads as *refuted by
/// nothing*: `lattice/06_friends_tables.ein`, 68 of 68.
///
/// `past-limit-lines.ein` (W27's witness) is the control: past the default
/// limit its run stops with the 15 members of N met, each not kept, and with
/// `--region-limit 17` it exhausts and prints the same 15, each with a core.
#[test]
fn a_core_the_run_did_not_keep_says_so_and_no_member_of_n_has_an_empty_core() {
    const TRACE: &str = "Core: not kept: the run stopped at `--region-limit`.";
    let scratch = ein_corpus::Scratch::new("ein-core-not-kept");
    let mut met: BTreeMap<bool, BTreeSet<String>> = BTreeMap::new();
    for (i, (file, limit, stopped, members)) in [
        ("examples/lattice/06_friends_tables.ein", None, true, 68),
        (
            "examples/ein-bugs/wrong-answers/past-limit-lines.ein",
            None,
            true,
            15,
        ),
        (
            "examples/ein-bugs/wrong-answers/past-limit-lines.ein",
            Some("17"),
            false,
            15,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let run = format!(
            "{file} {}",
            limit.map_or(String::new(), |l| format!("--region-limit {l}"))
        );
        let answer = scratch
            .join(format!("answer{i}"))
            .to_string_lossy()
            .to_string();
        let trace = scratch
            .join(format!("trace{i}.md"))
            .to_string_lossy()
            .to_string();
        let mut args = vec!["solve", file, "--dump-answer", &answer, "--trace", &trace];
        if let Some(l) = limit {
            args.extend(["--region-limit", l]);
        }
        ein(&args).ok();
        let region = &json_at(&format!("{answer}/answer.json"))["region"];
        assert_eq!(region["exhausted"], !stopped, "{run}: exhausted");
        let nogoods = json_at(&format!("{answer}/nogoods.json"));
        let n = nogoods.as_array().expect("nogoods.json is an array");
        assert_eq!(n.len(), members, "{run}: the members of N met");
        for m in n {
            let (core, status, rules) = (&m["core"], &m["core_status"], &m["refuted_by"]);
            if stopped {
                assert_eq!(
                    (core, status.as_str(), rules),
                    (&J::Null, Some("not-kept"), &J::Null),
                    "{run}: {} is printed with a core it did not keep",
                    m["set"]
                );
            } else {
                assert!(
                    core.as_array().is_some_and(|c| !c.is_empty())
                        && status == "within-budget"
                        && rules.as_array().is_some_and(|r| !r.is_empty()),
                    "{run}: {} has the core {core} as {status} and the rules {rules}",
                    m["set"]
                );
            }
        }
        let t = read(&trace);
        assert_eq!(
            (t.matches(TRACE).count(), t.matches("Core: `").count()),
            if stopped { (members, 0) } else { (0, members) },
            "{run}: --trace's N"
        );
        assert!(!t.contains("not kept by this route"), "{run}: --trace");
        if file.contains("past-limit-lines") {
            met.entry(stopped)
                .or_default()
                .extend(n.iter().map(|m| m["set"].to_string()));
        }
    }
    // The control: the run that stops met every member the run that does not
    // stop prints a core for.
    assert_eq!(met[&true], met[&false], "past-limit-lines' N");
}
