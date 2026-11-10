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
use std::path::{Path, PathBuf};
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

/// **`render` draws the IR, after the loader has read it** — so it reports a
/// load error as every command does.
///
/// The rule views render the *parsed forms*, as written; nothing they draw is
/// the loaded `Kb`. Until M1f S1f.5.21 nothing built one either, and a file
/// the loader refused still rendered whatever rules it declared, which this
/// test held: on the load-negative fixture the diagnostic was about the missing
/// rules and not about the cycle, *a `kb load error` here would mean the
/// renderer had quietly acquired a dependency on the loader*. Since S1f.5.21 it
/// has, on purpose (W65, Q-M1f.71): a view that drew a program every other
/// command refused disagreed with them about whether it is one, so each view
/// reads the program through the loader first and refuses it with the same
/// line and exit.
#[test]
fn render_reports_a_load_error_as_every_command_does() {
    let r = ein(&["render", "rules", LOAD_NEGATIVE]);
    assert_eq!(r.code, 1);
    let solve = ein(&["solve", LOAD_NEGATIVE]);
    assert_eq!(
        r.err.lines().next(),
        solve.err.lines().next(),
        "the view and `ein solve` refuse the program with one line"
    );
    assert!(
        r.err.starts_with("kb load error: ") && r.err.contains("derivation cycle"),
        "the diagnostic is the loader's: {}",
        r.err
    );
    assert!(
        r.out.is_empty(),
        "the renderer drew a refused program: {}",
        r.out
    );
    // The control: a file that loads renders its rules, so the exit above is
    // about the load.
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

/// **A conclusion no fact can hold is refused at the parse, whether its rule
/// fires or not** — M1e2 S1e2.2.18, restated at M1f S1f.5.5's swap by the
/// grammar (G-14, Q-M1f.1 (b), and S1f.5.11's and S1f.5.2's rulings).
///
/// Until S1e2.2.18 the line was `TypeError: expected NestedPattern at :assert
/// top-level, got Atom` or `TypeError: an unrecognised `:assert` slot has no
/// value to store` under `ein saturate`: a Python class, a Python IR class's
/// name, and no word of which rule or what it wrote. From then until the swap
/// the four shapes below loaded, and the rule's first firing stopped the run
/// with one line in Ein's spelling naming the rule and the slot. Since the
/// swap each is a parse error at the place, every command exit 1, with the
/// file, line and column: a keyword pair in a conclusion (S1f.5.11), a bare
/// `_` (S1f.5.1), a name or a variable as a whole conclusion (G-14, the paper
/// grammar), and a string as a value (S1f.5.2), which S1e2.2.18 had made the
/// symbol it names.
#[test]
fn a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling() {
    let scratch = ein_corpus::Scratch::new("ein-not-a-fact");
    let cases = [
        (r#"(tag ?x b :source "x")"#, 48, "unexpected `:source`"),
        ("(tag ?x _)", 46, "unexpected `_`: _ (reserved)"),
        ("foo", 38, "unexpected `foo`: a name"),
        ("(and (tag ?x b) ?x)", 54, "unexpected `?x`: a variable"),
        (r#"(tag ?x "lbl")"#, 46, r#"unexpected `"lbl"`: a string"#),
    ];
    for (i, (assert, col, what)) in cases.iter().enumerate() {
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
        let want = format!("{file}:3:{col}: {what}");
        for cmd in ["saturate", "solve"] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, 1, "ein {cmd} on :assert {assert}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l == want),
                "ein {cmd} on :assert {assert}: no line {want:?}:\n{}",
                r.err
            );
            for python in ["TypeError", "NestedPattern", "SForm", "unrecognised"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
}

/// **A wildcard written as a relation is refused** — M1e2 S1e2.2.20 (W20), on
/// the user's ruling of 2026-11-05 (Q-M1e2.14 (c)), and at the parse since M1f
/// S1f.5.5's swap (S1f.5.1: a bare `_` is refused in every position).
///
/// One report, from every command that reads the program: `ein saturate`,
/// `ein solve` and `ein oracle` exit 1 and `ein test` 2. Until S1e2.2.20 a
/// wildcard head stopped compilation with `unbound relation head
/// ?Wildcard() in a premise of `SForm(…)``, the Python IR's spelling; from
/// then until the swap the loader refused it with one line naming the rule
/// and the slot. A range in a rule was refused here too, from S1e2.2.20
/// (Q-M1e2.13 (a)) until M1f S1f.5.3 retired the token; the test below holds
/// what it is since.
#[test]
fn a_wildcard_relation_in_a_rule_is_refused_at_load() {
    let scratch = ein_corpus::Scratch::new("ein-refused-at-load");
    let cases = [
        (":match (_ 2) :assert (hit r)", 20),
        (":match (box (_ 2)) :assert (hit r)", 25),
        (":match (absent (_ b)) :assert (hit r)", 28),
    ];
    for (i, (clauses, col)) in cases.iter().enumerate() {
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
        let want = format!("{file}:3:{col}: unexpected `_`: _ (reserved)");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {clauses}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l == want),
                "ein {cmd} on {clauses}: no line {want:?}:\n{}",
                r.err
            );
            for python in ["CompileError", "Wildcard()", "SForm"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
}

/// **A range is the name it spells** — M1f S1f.5.5's swap, on S1f.5.4's
/// ruling (the name rule) and S1f.5.3's (Q-M1e2.24: Ein has no numbers and no
/// ranges), with S1f.5.11's (closed keywords) in the same commit.
///
/// `1..5`, `1..3`, `1..*` and `2..*` are each one name, in a fact, a
/// declaration's signature, a rule's `:match` and `:assert`: the program
/// loads, and each reads, matches and prints as written. `:card 1..*` is
/// refused at the parse, at `:card`, a keyword no form takes; and a
/// `(config …)` flag takes `true` or `false`, so `1..5` there is a name in a
/// place that takes none, refused at the parse with its repairs. Until M1f
/// S1f.5.3 a range was a token (in a fact it stored the name `1..5`, as a
/// keyword's value it was dropped, and in a rule the loader refused it, M1e2
/// S1e2.2.20); from S1f.5.3 to the swap `1` was an `INT` and `..5` no token,
/// so every command refused each of these programs at the `.`, which this
/// test held as `a_range_is_no_token`.
#[test]
fn a_range_is_the_name_it_spells() {
    let scratch = ein_corpus::Scratch::new("ein-range-is-a-name");
    let pre = "(relation n Node :why \"n holds of {?1}\") (relation hit Node :why \"{?1} is hit\") (relation r Node Node :why \"r holds of {?1} and {?2}\")\n(n a)\n";
    let loads = [
        ("(r 1..5 a)", "  (r 1..5 a)\n"),
        (
            "(relation q 1..* Node :why \"q holds of {?1} and {?2}\")",
            "(relation q 1..* Node)\n",
        ),
        (
            "(n 1..3)\n(rule x () :match (n 1..3) :assert (hit a) :why \"n holds of 1..3, so a is hit\")",
            "  (hit a :rule x)\n",
        ),
        (
            "(rule x () :match (n ?v) :assert (r ?v 2..*) :why \"n holds of {?v}, so r holds of {?v} and 2..*\")",
            "  (r a 2..* :rule x)\n",
        ),
    ];
    for (i, (form, line)) in loads.iter().enumerate() {
        let file = scratch.path().join(format!("loads{i}.ein"));
        std::fs::write(&file, format!("{pre}{form}\n")).expect("the program is written");
        let file = file.to_string_lossy();
        let r = ein(&["saturate", "--dump", &file]);
        assert_eq!(r.code, 0, "ein saturate --dump on {form}:\n{}", r.err);
        assert!(r.out.contains(line), "{form}: no line {line:?}:\n{}", r.out);
        for (cmd, code) in [("solve", 0), ("oracle", 0)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {form}:\n{}", r.err);
        }
    }
    let refused = [
        (
            "(r a b :card 1..*)",
            8,
            "unexpected `:card`: a keyword this form does not take",
        ),
        (
            "(relation q Node Node :card 1..*)",
            23,
            "unexpected `:card`: a keyword this form does not take",
        ),
        (
            "(config :lattice-sanity-check 1..5)",
            31,
            "unexpected `1..5`: a name",
        ),
    ];
    for (i, (form, col, what)) in refused.iter().enumerate() {
        let file = scratch.path().join(format!("refused{i}.ein"));
        std::fs::write(&file, format!("{pre}{form}\n")).expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("{file}:3:{col}: {what}");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {form}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l == want),
                "ein {cmd} on {form}: no line {want:?}:\n{}",
                r.err
            );
        }
    }
}

/// **A string is natural-language text, and no value** — M1f S1f.5.2, on the
/// user's rulings of 2026-11-06 (Q-M1e2.23: *"I think we should allow use
/// strings only for NL - :why, :source, and other places where NL text or NL
/// text template with variables lives"*; *"(p "1") has to be refused"*) and
/// G-6 (a variable in a fact is refused).
///
/// Every command that reads a program refuses a string written as a value,
/// `(p "1")` first, with one line at the string, `FILE:L:C: unexpected `"1"`:
/// a string`, and S1f.5.2's note under it; and a variable in a fact, `(r ?x)`,
/// at the variable. Until S1f.5.5's swap `(p "1")` was the fact `(p 1)` (M1e2
/// S1e2.2.16) and `(r ?x)` held the name `?x`, printed `"?x"`. The same shapes
/// in every other place a value goes are rows of the grammar's table
/// (`ein-ir/tests/grammar_pages.rs`), and the three keywords' text loads and
/// reaches every surface that carries it
/// (`ein-cli/tests/natural_language_text.rs`).
#[test]
fn a_string_is_natural_language_and_no_value() {
    let scratch = ein_corpus::Scratch::new("ein-strings-are-text");
    let note = "a string is natural-language text, written only after :why, :source or \
                :goal-text: write a name bare";
    let pre = "(relation p T :why \"{?1} is a p\")\n(relation r T)\n";
    for (form, col, what, noted) in [
        ("(p \"1\")", 4, "unexpected `\"1\"`: a string", true),
        ("(p \"a\")", 4, "unexpected `\"a\"`: a string", true),
        ("(p (q \"x y\"))", 7, "unexpected `\"x y\"`: a string", true),
        (
            "(rule s () :match (p \"1\") :assert (r a))",
            22,
            "unexpected `\"1\"`: a string",
            true,
        ),
        (
            "(query :goal (p \"1\"))",
            17,
            "unexpected `\"1\"`: a string",
            true,
        ),
        ("(r ?x)", 4, "unexpected `?x`: a variable", false),
    ] {
        let file = scratch.path().join("refused.ein");
        std::fs::write(&file, format!("{pre}{form}\n")).expect("the program is written");
        let file = file.to_string_lossy();
        let want = format!("{file}:3:{col}: {what}");
        let quoted = what.split('`').nth(1).expect("the token");
        let want_note = format!("  note: {quoted} is a string, and {note}");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} on {form}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l == want),
                "ein {cmd} on {form}: no line {want:?}:\n{}",
                r.err
            );
            assert_eq!(
                r.err.lines().any(|l| l == want_note),
                noted,
                "ein {cmd} on {form}: the note {want_note:?}:\n{}",
                r.err
            );
        }
    }
}

/// **A bare `_` and `?_` name nothing, in every place** — M1f S1f.5.1's
/// T1f.5.1.3, on the user's ruling of 2026-11-06 (Q-M1e2.22: *"simple `_`
/// reserved and refused because of potential ambiguity, but remain valid in
/// any place in atom and variable names"*), at the parse since S1f.5.5's swap.
///
/// Every command that reads a program refuses each token in each place of
/// S1f.5.1's table: `ein saturate`, `solve`, `oracle`, `render program`,
/// `render rules`, `render constraints`, `render kb` and `kb save` exit 1 and
/// `ein test` 2, each with one line at the token, `FILE:L:C: unexpected `_`: _
/// (reserved)`, and S1f.5.1's note under it. Until the swap `_` was dropped
/// from a fact, a nested fact and an activator in silence (W-22, W29), matched
/// any value in a rule's pattern (W20), was read in a goal, and was refused at
/// load as a relation, a guard's argument or a premise; `?_` was refused at
/// the lexer. A module that writes one is refused where it is imported, with
/// the module's line, by every command that resolves a file-relative import:
/// `ein solve`, `oracle`, `test`, `render kb` and `kb save` (`ein saturate`
/// resolves none, and `render program`, `rules` and `constraints` draw the
/// file as written). The same tokens in every other place a name or a
/// variable goes are rows of the grammar's table
/// (`ein-ir/tests/grammar_pages.rs`), and a name that holds `_` anywhere else,
/// `_a_in_b_` or `?_______01`, is a name or a variable (`ein-ir`'s
/// `grammar.rs` tests).
#[test]
fn a_bare_underscore_names_nothing_in_every_place() {
    let scratch = ein_corpus::Scratch::new("ein-underscore-names-nothing");
    let pre = "(relation p T) (relation r T T) (relation h T) (relation hit T) (relation tag T T)\n\
               (rule q (?x) :match (p ?x) :assert (hit ?x))\n";
    // The place, as S1f.5.1's table names it, and line 3 with `{}` for the token.
    let places = [
        ("a fact's argument", "(r {} a)"),
        ("a nested fact's argument", "(r (f {} a) c)"),
        ("a nested fact's relation", "(h ({} b))"),
        ("an activator's argument", "(q {})"),
        ("a top-level form's head", "({} a)"),
        ("a relation's signature", "(relation s {} T)"),
        (
            "a premise's argument",
            "(rule w () :match (p {}) :assert (hit w))",
        ),
        (
            "a pattern's argument, under an absent",
            "(rule w () :match (and (p ?x) (absent (tag ?x {}))) :assert (hit ?x))",
        ),
        (
            "a premise's relation",
            "(rule w () :match ({} a) :assert (hit w))",
        ),
        (
            "a guard's argument",
            "(rule w () :match (and (p ?x) (eq ?x {})) :assert (hit ?x))",
        ),
        (
            "a premise",
            "(rule w () :match (and (p ?x) {}) :assert (hit ?x))",
        ),
        (
            "a conclusion's argument",
            "(rule w () :match (p ?x) :assert (tag ?x {}))",
        ),
        (
            "an hrule's conclusion",
            "(hrule g () :match (p ?x) :assert (tag ?x {}))",
        ),
        ("a query's goal", "(query :goal (hit {}))"),
        (
            "a claim's fact",
            "(query :goal (hit ?x) :expect (model (hit {})))",
        ),
    ];
    let commands: [(&[&str], i32); 8] = [
        (&["saturate"], 1),
        (&["solve"], 1),
        (&["oracle"], 1),
        (&["test"], 2),
        (&["render", "program"], 1),
        (&["render", "rules"], 1),
        (&["render", "constraints"], 1),
        (&["render", "kb"], 1),
    ];
    let note = |token: &str| {
        format!(
            "  note: {token} is reserved and names nothing: where any value will do, write a \
             variable used once"
        )
    };
    let refused_by = |argv: &[&str], code: i32, want: &str, token: &str, what: &str| {
        let r = ein(argv);
        assert_eq!(r.code, code, "ein {argv:?} on {what}:\n{}", r.err);
        assert!(
            r.err.lines().any(|l| l == want),
            "ein {argv:?} on {what}: no line {want:?}:\n{}",
            r.err
        );
        assert!(
            r.err.lines().any(|l| l == note(token)),
            "ein {argv:?} on {what}: no note:\n{}",
            r.err
        );
    };
    for (i, (place, form)) in places.iter().enumerate() {
        for token in ["_", "?_"] {
            let file = scratch.path().join(format!("{i}-{}.ein", token.len()));
            std::fs::write(&file, format!("{pre}{}\n", form.replace("{}", token)))
                .expect("the program is written");
            let file = file.to_string_lossy();
            let col = form.find("{}").expect("the token's place") + 1;
            let want = format!("{file}:3:{col}: unexpected `{token}`: {token} (reserved)");
            let what = format!("`{token}` as {place}");
            for (cmd, code) in &commands {
                let argv: Vec<&str> = cmd.iter().copied().chain([file.as_ref()]).collect();
                refused_by(&argv, *code, &want, token, &what);
            }
            #[cfg(feature = "einb")]
            {
                let out = scratch.path().join(format!("{i}-{}.einb", token.len()));
                let out = out.to_string_lossy();
                refused_by(&["kb", "save", &file, &out], 1, &want, token, &what);
            }
        }
    }
    // A module that writes one is refused where it is imported.
    let module = scratch.path().join("module.ein");
    std::fs::write(&module, "(relation p T T)\n(macro m (?a) (p ?a _))\n")
        .expect("the module is written");
    let main = scratch.path().join("main.ein");
    std::fs::write(
        &main,
        "(import module)\n(relation q T :why \"q holds of {?1}\")\n(q a)\n",
    )
    .expect("the program is written");
    let (module, main) = (module.to_string_lossy(), main.to_string_lossy());
    let want = format!("kb load error: {module}:2:21: unexpected `_`: _ (reserved)");
    let what = "`_` in an imported macro's body";
    for (cmd, code) in [
        (&["solve"][..], 1),
        (&["oracle"][..], 1),
        (&["test"][..], 2),
        (&["render", "kb"][..], 1),
    ] {
        let argv: Vec<&str> = cmd.iter().copied().chain([main.as_ref()]).collect();
        refused_by(&argv, code, &want, "_", what);
    }
    #[cfg(feature = "einb")]
    {
        let out = scratch.path().join("main.einb");
        refused_by(
            &["kb", "save", &main, &out.to_string_lossy()],
            1,
            &want,
            "_",
            what,
        );
    }
}

/// **A run of digits is a name in every place a name goes** — M1f S1f.5.4's
/// T1f.5.4.3, on the user's rulings of 2026-11-06 (*"digits are not numbers,
/// digits are plain names … (p 1) and (p 01) are VERY DIFFERENT facts"*) and
/// 2026-11-07 (F8: a label is a name like any other).
///
/// One program writes a run of digits in each place: a fact's argument and
/// its relation, a nested fact, a stored negation, a declaration's name and
/// its signature, a rule's name, a premise's relation and argument, a guard's
/// argument, an `absent`'s, a conclusion's argument, and a claim's fact.
/// `ein saturate --dump` prints each as written and holds each pair apart:
/// `1` and `01`, `7` and `-7` and `07` are five names, the rule `1` fires on
/// `(p 1)` and not on `(p 01)`, the guard `(neq ?x 1)` lets `01` through and
/// `(absent (n 7))` holds beside `(n 01)`. A claim that names `1` where the
/// state holds `01` is refuted, and `ein test` exits 1; naming `01` it holds.
/// Until the swap `(p 1) (p 01)` stored one fact, the int `1`; since
/// T1f.5.4.3 there is no int kind to read a run of digits as. A token that is
/// no name, in the same places, is refused at the parse: the grammar's table
/// (`ein-ir/tests/grammar_pages.rs`).
#[test]
fn a_run_of_digits_is_a_name_in_every_place() {
    let scratch = ein_corpus::Scratch::new("ein-digits-are-names");
    let program = "(relation p T :why \"p holds of {?1}\") (relation n T :why \"n holds of {?1}\") (relation q T T :why \"q holds of {?1} and {?2}\")\n\
                   (relation 1 T :why \"1 holds of {?1}\") (relation 01 T :why \"01 holds of {?1}\")\n\
                   (relation r 1 01 :why \"r holds of {?1} and {?2}\")\n\
                   (p 1) (p 01) (p -7) (p 7)\n\
                   (1 a) (01 a)\n\
                   (p (q 1 01))\n\
                   (not (n 1)) (n 01)\n\
                   (rule 1 () :match (p 1) :assert (n a) :why \"p holds of 1, so n holds of a\")\n\
                   (rule 01 () :match (1 ?x) :assert (q ?x 01) :why \"1 holds of {?x}, so q holds of {?x} and 01\")\n\
                   (rule g () :match (and (p ?x) (neq ?x 1) (neq ?x 7)) :assert (q g ?x) :why \"p holds of {?x}, which is neither 1 nor 7, so q holds of g and {?x}\")\n\
                   (rule b () :match (absent (n 7)) :assert (q b 07) :why \"n does not hold of 7, so q holds of b and 07\")\n\
                   (solve :no-hypothesis (p n q 1 01 r))\n";
    let file = scratch.path().join("digits.ein");
    std::fs::write(&file, program).expect("the program is written");
    let file = file.to_string_lossy();
    let r = ein(&["saturate", "--dump", &file]);
    assert_eq!(r.code, 0, "ein saturate --dump:\n{}", r.err);
    for line in [
        // A declaration's name and its signature.
        "(relation 1 T)\n",
        "(relation 01 T)\n",
        "(relation r 1 01)\n",
        // A fact's argument: four names, not two ints.
        "  (p 1)\n",
        "  (p 01)\n",
        "  (p -7)\n",
        "  (p 7)\n",
        // A fact's relation, a nested fact, a stored negation beside its twin.
        "  (1 a)\n",
        "  (01 a)\n",
        "  (p (q 1 01))\n",
        "  (not (n 1))\n",
        "  (n 01)\n",
        // A rule's name.
        ";;   1  :params ()",
        ";;   01  :params ()",
        // A premise's argument, `(p 1)` and not `(p 01)`, and relation.
        "  (n a :rule 1)\n",
        "  (q a 01 :rule 01)\n",
        // A guard's argument: `1` and `7` are left out, `01` and `-7` are not.
        "  (q g 01 :rule g)\n",
        "  (q g -7 :rule g)\n",
        // An absent's argument, and a conclusion's.
        "  (q b 07 :rule b)\n",
    ] {
        assert!(r.out.contains(line), "no line {line:?}:\n{}", r.out);
    }
    for line in ["(q g 1 ", "(q g 7 ", "(n a :rule 01)", "(q 1 01 :rule 01)"] {
        assert!(!r.out.contains(line), "a line {line:?}:\n{}", r.out);
    }
    // A claim's fact: `(q a 01)` is the state's, `(q a 1)` is no fact of it.
    for (claim, code) in [("(model (q a 01))", 0), ("(model (q a 1))", 1)] {
        let file = scratch.path().join(format!("claim{code}.ein"));
        std::fs::write(
            &file,
            format!("{program}(query :goal (q a ?x) :expect {claim})\n"),
        )
        .expect("the program is written");
        let r = ein(&["test", &file.to_string_lossy()]);
        assert_eq!(r.code, code, "ein test on {claim}:\n{}{}", r.out, r.err);
    }
}

/// **A rule shape that reads nothing is refused** — M1e2 S1e2.2.24 (W24), on
/// the user's rulings of 2026-11-05 (Q-M1e2.16 (1), Q-M1e2.18), and since M1f
/// S1f.5.5's swap at the parse wherever the grammar states the place.
///
/// Each of W24's seven witnesses, from every command that loads it: `ein
/// saturate`, `ein solve` and `ein oracle` exit 1 and `ein test` 2. Until
/// S1e2.2.24 each loaded and read nothing, in silence: a guard on `_`, a form,
/// a keyword pair or a variable no premise before it binds compared as a raw
/// node or a null; a premise that is not a form was dropped; a pattern inside
/// a pattern whose relation no parameter binds, or whose activator bound it to
/// a nested fact, matched nothing. From then the loader refused each with a
/// line in Ein's spelling. Since the swap three are the grammar's, each a
/// parse error at the place with the file, line and column (`_` as a guard's
/// argument and as a premise, S1f.5.1; `foo` as a premise, the paper
/// grammar's; a keyword pair as a guard's argument, S1f.5.11), and two,
/// which a context-free grammar does not see, keep the loader's line. A
/// parameter a rule binds to a nested fact through a variable is the
/// compiler's (`ein-infer/tests/compile_semantics.rs`).
///
/// **Since M1f S1f.5.7 (T1f.5.7.3) two of the seven load**, on the user's
/// ruling of 2026-11-07 on Q-M1e2.26: `form-in-guard`, a fact that holds no
/// variable as a guard's argument, which is a value, and
/// `guard-before-binder`, a guard written before the premise that binds its
/// variable, which waits for it. A form that holds a variable stays refused
/// (Q-M1f.30), with its own line.
#[test]
fn a_rule_shape_that_reads_nothing_is_refused_at_load() {
    const GUARD: &str = "a guard compares two values, each a variable, a name, or a fact \
                         that holds no variable";
    let at_the_parse: [(&str, &[&str]); 3] = [
        (
            "wild-in-guard",
            &[
                "35:38: unexpected `_`: _ (reserved)",
                "36:39: unexpected `_`: _ (reserved)",
                "37:35: unexpected `_`: _ (reserved)",
            ],
        ),
        (
            "bare-premise",
            &[
                "37:19: unexpected `_`: _ (reserved)",
                "38:33: unexpected `_`: _ (reserved)",
                "39:19: unexpected `foo`: a name",
            ],
        ),
        (
            "kwpair-in-guard",
            &["33:38: unexpected `:k`: a keyword this form does not take"],
        ),
    ];
    for (name, lines) in &at_the_parse {
        let file = format!("examples/ein-bugs/wrong-answers/{name}.ein");
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} {file}:\n{}", r.err);
            for line in *lines {
                let want = format!("{file}:{line}");
                assert!(
                    r.err.lines().any(|l| l == want),
                    "ein {cmd} {file}: no line {want:?}:\n{}",
                    r.err
                );
            }
        }
    }
    let at_load: [(&str, String); 2] = [
        (
            "nested-unbound-head",
            "rule 'r': its :match writes (?r b) with ?r as its relation, which no parameter \
             binds — a relation is named by a name or by a parameter its activator binds"
                .to_string(),
        ),
        (
            "nested-fact-activator",
            "rule 'r': its parameter ?R names a relation, and (r (a b)) binds it to the fact \
             (a b) — a relation is named by a symbol, not a fact"
                .to_string(),
        ),
    ];
    for (name, line) in &at_load {
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
    // Since S1f.5.7 the two shapes its ruling answers load, on every command.
    for name in ["form-in-guard", "guard-before-binder"] {
        let file = format!("examples/ein-bugs/wrong-answers/{name}.ein");
        for cmd in ["saturate", "solve", "oracle"] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, 0, "ein {cmd} {file}:\n{}", r.err);
        }
    }
    // A form that holds a variable stays refused (Q-M1f.30), in a rule and in
    // a query's goal, one line per form.
    let scratch = ein_corpus::Scratch::new("ein-guard-pattern");
    for (src, want) in [
        (
            "(rule e () :match (and (p ?x) (q ?y) (eq ?x (f ?y))) :assert (hit ?x) :why \"p holds of {?x}, q of {?y}, and {?x} is (f {?y}), so {?x} is hit\")",
            format!(
                "kb load error: rule 'e': its :match writes (eq ?x (f ?y)), and the form (f ?y) \
                 holds a variable — {GUARD}"
            ),
        ),
        (
            "(query :goal (and (q ?y) (neq ?x (f ?y))))",
            format!(
                "kb load error: query: its :goal writes (neq ?x (f ?y)), and the form (f ?y) \
                 holds a variable — {GUARD}"
            ),
        ),
    ] {
        let file = scratch.path().join("pattern.ein");
        std::fs::write(
            &file,
            format!(
                "(relation p Node :why \"p holds of {{?1}}\") (relation q Node :why \"q holds of {{?1}}\") (relation f Node :why \"f holds of {{?1}}\") (relation hit Node :why \"{{?1}} is hit\")\n\
                 (p a) (q a)\n{src}\n"
            ),
        )
        .expect("the program is written");
        let file = file.to_string_lossy();
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, code, "ein {cmd} {src}:\n{}", r.err);
            assert_eq!(
                r.err.lines().rfind(|l| !l.is_empty()),
                Some(want.as_str()),
                "ein {cmd} {src}"
            );
        }
    }
    // The `:assert` shapes the test above held at the firing, and the
    // top-level relation variable: a keyword pair inside a nested conclusion
    // at the parse since the swap (S1f.5.11), the two relation variables no
    // parameter binds in Ein's spelling at load.
    let scratch = ein_corpus::Scratch::new("ein-reads-nothing");
    let file = scratch.path().join("kw.ein");
    std::fs::write(
        &file,
        "(relation holds Node) (relation tag Node Node)\n(holds a)\n\
         (rule s () :match (holds ?x) :assert (tag ?x (box a :k v)))\n",
    )
    .expect("the program is written");
    let file = file.to_string_lossy();
    let want = format!("{file}:3:53: unexpected `:k`: a keyword this form does not take");
    for cmd in ["saturate", "solve"] {
        let r = ein(&[cmd, &file]);
        assert_eq!(r.code, 1, "ein {cmd} {file}:\n{}", r.err);
        assert!(r.err.lines().any(|l| l == want), "{}", r.err);
    }
    for (i, (assert, line)) in [
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
                "(relation holds Node :why \"{{?1}} holds\") (relation tag Node Node :why \"{{?1}} has the tag {{?2}}\")\n(holds a)\n\
                 (rule s () :match (holds ?x) :assert {assert} :why \"{{?x}} holds, so s concludes a fact that names {{?x}} as a relation\")\n"
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

/// **A conclusion's variable nothing binds is refused at load, by every
/// command, with one line** — M1f S1f.5.10, on the user's ruling of 2026-11-07
/// on P1f.5's F13 (*"yes, file F13 with the refusal"*).
///
/// W57's four witnesses and the regression fixture, each on every command that
/// loads a program: exit 1 (`ein test`: 2), and the loader's line, one per
/// variable, naming the rule, the variable and the two explicit spellings, in
/// Ein's words. Until S1f.5.10 the first and the fixture stopped every command
/// at root's firing, *unbound var ?y in :assert — bindings: {'x': 'a'}*; the
/// second stopped `solve` and `oracle` while `saturate` and `solve --minimal`
/// answered; and the third and the fourth answered on every command. What
/// loads: a variable a typed premise, an `eq` guard or a parameter binds. And
/// W58's witness, a parameter its activator binds to a nested fact, which
/// stopped the run with the same line until S1f.5.18 seeded the parameter
/// (Q-M1f.37), answers on every command.
#[test]
fn a_conclusion_variable_nothing_binds_is_refused_at_load() {
    let line = |head: &str, name: &str, var: &str, place: &str| {
        format!(
            "{head} '{name}': its :assert writes ?{var}, which no premise or parameter \
             binds{place} — to conclude it of every object of a type, bind it with a premise \
             such as (is-a ?{var} T), or of every name of the program with (eq ?{var} ?{var})"
        )
    };
    let refused: [(&str, String); 5] = [
        (
            "examples/ein-bugs/wrong-answers/assert-var-stops.ein",
            line("rule", "r", "y", ""),
        ),
        (
            "examples/ein-bugs/wrong-answers/assert-var-under-commitment.ein",
            line("rule", "r", "y", ""),
        ),
        (
            "examples/ein-bugs/wrong-answers/hrule-assert-var.ein",
            line("hrule", "h", "y", ""),
        ),
        (
            "examples/ein-bugs/wrong-answers/assert-var-never-fires.ein",
            line("rule", "r", "y", ""),
        ),
        (
            "examples/ein-bugs/unbound-assert-var.ein",
            format!(
                "{}; {}",
                line(
                    "rule",
                    "fire-3",
                    "v0",
                    " where its :match holds through (r1 ?v1 ?v3)"
                ),
                line(
                    "rule",
                    "fire-3",
                    "v1",
                    " where its :match holds through (r0 ?v0 ?v2)"
                )
            ),
        ),
    ];
    let scratch = ein_corpus::Scratch::new("ein-unbound-conclusion");
    let einb = scratch.path().join("x.einb").to_string_lossy().into_owned();
    for (file, want) in &refused {
        let want = format!("kb load error: {want}");
        for (args, code) in [
            (vec!["saturate", "--dump", file], 1),
            (vec!["solve", file], 1),
            (vec!["solve", "--minimal", file], 1),
            (vec!["oracle", file], 1),
            (vec!["test", file], 2),
            (vec!["render", "lattice", file], 1),
            (vec!["render", "kb", file], 1),
            (vec!["kb", "save", file, &einb], 1),
        ] {
            let r = ein(&args);
            assert_eq!(r.code, code, "ein {args:?}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l.trim() == want),
                "ein {args:?}: no line {want:?}:\n{}",
                r.err
            );
            for python in ["KeyError", "bindings: {", "unbound var"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
    // What loads: a variable a typed premise, an `eq` guard or a parameter
    // binds — the acceptance's typed premise among them.
    let ok = scratch.path().join("ok.ein");
    std::fs::write(
        &ok,
        "(relation p Node :why \"p holds of {?1}\") (relation q Node :why \"q holds of {?1}\") (relation is-a Node Node :why \"{?1} is a {?2}\")\n\
         (p a) (is-a h1 House)\n\
         (rule typed () :match (and (p ?x) (is-a ?y House)) :assert (q ?y) :why \"p holds of {?x} and {?y} is a House, so q holds of {?y}\")\n\
         (rule every () :match (and (p ?x) (eq ?y ?y)) :assert (q ?y) :why \"p holds of {?x} and {?y} is a name of the program, so q holds of {?y}\")\n\
         (rule param (?y) :match (p ?x) :assert (q ?y) :why \"p holds of {?x}, so q holds of {?y}\")\n(param b)\n",
    )
    .expect("writes");
    let r = ein(&["saturate", "--dump", &ok.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    // `typed` concludes `(q h1)`; `every`, every name of the program, `House`
    // and `b` among them, `b` the parameter's too.
    for fact in ["(q h1 :rule typed)", "(q b :rule", "(q House :rule every)"] {
        assert!(r.out.contains(fact), "no {fact}:\n{}", r.out);
    }
    // W58: the one path S1f.5.10 left, a parameter its activator binds to a
    // nested fact, which the compiler did not seed (W33's cause) until
    // S1f.5.18; every command answers since, root holding `(hit (f a))`.
    let w58 = "examples/ein-bugs/wrong-answers/parameter-nested-assert.ein";
    for args in [
        vec!["saturate", "--dump", w58],
        vec!["solve", w58],
        vec!["solve", "--minimal", w58],
        vec!["oracle", w58],
    ] {
        let r = ein(&args);
        assert_eq!(r.code, 0, "ein {args:?}:\n{}", r.err);
        for python in ["KeyError", "bindings: {", "unbound var"] {
            assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
        }
    }
    let r = ein(&["saturate", "--dump", w58]);
    assert!(r.out.contains("(hit (f a) :rule act)"), "{}", r.out);
}

/// **A rule that could derive an activator binding a relation parameter to a
/// fact is refused at load, by every command, with one line** — M1f S1f.5.18,
/// on the user's ruling of 2026-11-08 on Q-M1f.1 (b).
///
/// The two `g14` programs, as refusal fixtures under `examples/broken/load/`
/// and as W59's witnesses, each on every command that loads a program: exit 1
/// (`ein test`: 2) and the loader's line, naming the deriving rule, the
/// activated rule and the parameter. Until S1f.5.18 the first stopped every
/// command where `r`'s plan was compiled, with the compiler's *the relation
/// head ?R of (?R b) … is bound to the fact (a b)*, and the second answered.
/// What loads: the zebra2 fan-out and the stdlib's, which pass a relation's
/// name on through a rule's parameter and through a marker.
#[test]
fn a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load() {
    let want = "kb load error: rule 'mk': its :assert writes (r ?f), whose ?f is rule 'r''s \
                parameter ?R, which names a relation, and (item ?f) can bind ?f to a fact, as an \
                argument of the declared relation item — a relation is named by a symbol, not a \
                fact";
    let scratch = ein_corpus::Scratch::new("ein-derived-activator");
    let einb = scratch.path().join("x.einb").to_string_lossy().into_owned();
    for file in [
        "examples/broken/load/derived_activator.ein",
        "examples/broken/load/derived_activator_never.ein",
        "examples/ein-bugs/wrong-answers/derived-activator.ein",
        "examples/ein-bugs/wrong-answers/derived-activator-never.ein",
    ] {
        for (args, code) in [
            (vec!["saturate", "--dump", file], 1),
            (vec!["solve", file], 1),
            (vec!["solve", "--minimal", file], 1),
            (vec!["oracle", file], 1),
            (vec!["test", file], 2),
            (vec!["render", "lattice", file], 1),
            (vec!["render", "kb", file], 1),
            (vec!["kb", "save", file, &einb], 1),
        ] {
            let r = ein(&args);
            assert_eq!(r.code, code, "ein {args:?}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l.trim() == want),
                "ein {args:?}: no line {want:?}:\n{}",
                r.err
            );
            assert!(
                !r.err.contains("is bound to the fact"),
                "the compiler's line is back:\n{}",
                r.err
            );
        }
    }
    // What loads: zebra2's `co-located-fanout`, whose `?R1` a rule's relation
    // parameter binds, and the stdlib's `bijective-setup` and `typecheck-setup`,
    // which read the markers `(bijective ?R)` and `(typecheck-hierarchy ?isa)`.
    for (args, code) in [
        (vec!["saturate", "examples/zebra2.ein"], 0),
        (
            vec!["test", "tests/stdlib/bijection/01_setup_and_negatives.ein"],
            0,
        ),
    ] {
        let r = ein(&args);
        assert_eq!(r.code, code, "ein {args:?}:\n{}", r.err);
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

/// **A goal binding of digits is a JSON *string*** — at any width, and as
/// written. Restated at M1f S1f.5.5's swap, on S1f.5.4's Q4 (a), decided
/// 2026-11-08.
///
/// Until the swap a goal binding was the *stored* argument, so an IR `INT` was
/// an integer and writing it as `"8"` was a type error a consumer could not
/// undo: S1a.6.6's fuzzer found it on a two-line program, and the wide literal
/// was carried as an exact numeric literal rather than clamped to an `i64`.
/// Since the swap a run of digits is a name (S1f.5.4's ruling), and every name
/// is a JSON string, so that `(p 007)` and `(p 7)`, two names, stay two in the
/// summary.
#[test]
fn a_goal_binding_of_digits_is_a_json_string() {
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
            let x = r["x"].as_str().expect("?x is a name").to_string();
            let y = r["y"]
                .as_str()
                .unwrap_or_else(|| panic!("?y = {} is not a string", r["y"]));
            (x, y.to_string())
        })
        .collect();
    assert_eq!(bound["o3"], "8");
    assert_eq!(bound["o4"], "-7");
    assert_eq!(bound["o5"], "99999999999999999999999");
    assert_eq!(bound.len(), 3, "a binding went missing: {bound:?}");
    assert!(
        text.contains(r#""y": "99999999999999999999999""#),
        "the wide name was not written as written:\n{text}"
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
/// There is one predicate now (`ein_ir::read::looks_like_einb`, `ein-cli`'s
/// `common` until M1f S1f.5.22) and one constant (`ein_ir::read::EINB_MAGIC`,
/// compared against `ein_einb::header::MAGIC` by
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
        // The parse error's first line, since M1f S1f.5.5's swap the token
        // it found (`unexpected input` until then).
        assert!(
            r.err.contains(&format!("{path}:1:1: unexpected `EINB")),
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

// ── Every member of N has its core, past the limit too — M1f S1f.5.12 ─────

/// **Every member of N a describing run prints has its core, past
/// `--region-limit` too, and it is the core the walk takes eagerly** — M1f
/// S1f.5.12, the user's ruling of 2026-11-08 on P-15, (c). `--dump-answer`
/// and `--trace` ask for N's cores, so the run takes none during its walk and
/// a core for each member of N after it, through the node function against
/// the walk's snapshot. Until S1f.5.12 a run past the limit printed each
/// member it met with a `null` core and rules beside `core_status:
/// "not-kept"`, and `--trace` said *Core: not kept: the run stopped at
/// `--region-limit`.* (M1e2 S1e2.3.9): `lattice/06_friends_tables.ein`, 68 of
/// 68, and `past-limit-lines.ein`, 15 of 15.
///
/// `past-limit-lines.ein` (W27's witness) is the control: past the default
/// limit its run stops with the 15 members of N met, and with
/// `--region-limit 17` it exhausts and prints the same 15 with the same
/// cores, statuses and rules, which are the cores `--dump-lattice` writes for
/// the same sets, every node's core taken as the walk visits it.
#[test]
fn every_member_of_n_has_its_core_past_the_limit_too() {
    let scratch = ein_corpus::Scratch::new("ein-core-n-after-the-walk");
    let mut printed: BTreeMap<bool, BTreeMap<String, J>> = BTreeMap::new();
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
            assert!(
                core.as_array().is_some_and(|c| !c.is_empty())
                    && status == "within-budget"
                    && rules.as_array().is_some_and(|r| !r.is_empty()),
                "{run}: {} has the core {core} as {status} and the rules {rules}",
                m["set"]
            );
        }
        let t = read(&trace);
        assert_eq!(
            t.matches("Core: `").count(),
            members,
            "{run}: --trace's N gives every member its core"
        );
        assert!(!t.contains("not kept"), "{run}: --trace");
        if file.contains("past-limit-lines") {
            printed
                .entry(stopped)
                .or_default()
                .extend(n.iter().map(|m| (m["set"].to_string(), m.clone())));
        }
    }
    // The control: the run that stops met every member the run that does not
    // stop prints, and prints each with the same core.
    assert_eq!(printed[&true], printed[&false], "past-limit-lines' N");
    // And each is the eager core: `--dump-lattice` takes every visited dead
    // node's core as the walk visits it.
    let lattice = scratch.join("lattice").to_string_lossy().to_string();
    ein(&[
        "solve",
        "examples/ein-bugs/wrong-answers/past-limit-lines.ein",
        "--region-limit",
        "17",
        "--dump-lattice",
        &lattice,
    ])
    .ok();
    let nodes = json_at(&format!("{lattice}/nodes.json"));
    let mut eager = 0;
    for node in nodes["nodes"].as_array().expect("nodes") {
        let set = node["commitment"].to_string();
        if let Some(m) = printed[&true].get(&set) {
            eager += 1;
            for f in ["core", "core_status", "refuted_by"] {
                assert_eq!(m[f], node[f], "past-limit-lines: {set}'s {f}");
            }
        }
    }
    assert_eq!(eager, 15, "past-limit-lines: N's members in nodes.json");
}

/// **A goal's empty row says whether the goal has a variable** — M1f S1f.5.7
/// (S-18): *the goal holds: it has no variable* is printed only for a goal
/// with no variable, `(neq a b)`; a goal whose every variable is an `(absent
/// …)`'s own binds none, and says so; and a goal whose guard has a free
/// variable prints a row per value, where it printed *it has no variable*
/// until S1f.5.7.
#[test]
fn a_goals_empty_row_says_whether_the_goal_has_a_variable() {
    let scratch = ein_corpus::Scratch::new("ein-goal-row");
    let file = scratch.path().join("goal.ein");
    std::fs::write(
        &file,
        "(relation p Node :why \"p holds of {?1}\") (relation q Node :why \"q holds of {?1}\")\n(p a)\n\
         (query :goal (neq a b))\n\
         (query :goal (absent (q ?x)))\n\
         (query :goal (neq w ?y))\n",
    )
    .expect("the program is written");
    let r = ein(&["solve", &file.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    let block = |i: usize| -> String {
        let start = r
            .out
            .find(&format!("query {i} of 3"))
            .expect("the query's block");
        let rest = &r.out[start..];
        let end = rest[1..].find("query ").map_or(rest.len(), |e| e + 1);
        rest[..end].to_string()
    };
    assert!(
        block(1).contains("the goal holds: it has no variable"),
        "{}",
        block(1)
    );
    assert!(
        block(2).contains("the goal holds: it binds no variable"),
        "{}",
        block(2)
    );
    let third = block(3);
    assert!(!third.contains("the goal holds"), "{third}");
    for name in ["Node", "a", "p", "q"] {
        assert!(third.contains(&format!("?y = {name}")), "{third}");
    }
    assert!(!third.contains("?y = w"), "{third}");
}

/// **An empty `:match` fires, and an empty conclusion asserts nothing, on every
/// command** — M1f S1f.5.9, on the user's ruling of 2026-11-07 on P1f.5's F12
/// (*"1 fire; 2 same; 3 reject (and) while it is the same as () and `and`
/// should have arguments"*).
///
/// W31's four witnesses and W34's, each on every command that answers a
/// program: `()` is the empty conjunction, which is true, so a rule whose
/// `:match` is `()` fires once at root, or once per activator fact, with only
/// its parameters bound, and an hrule's proposes its conclusion likewise; and
/// `:assert ()` concludes nothing. Until S1f.5.9 `()` was a premise over the
/// parser's synthetic `#empty`, which no fact holds, so each rule never fired
/// and the hrule proposed nothing, in silence (W31), and `:assert ()` stored a
/// fact under that head, whose name joined H (W34).
#[test]
fn an_empty_match_fires_and_an_empty_conclusion_asserts_nothing() {
    let w = |name: &str| format!("examples/ein-bugs/wrong-answers/{name}.ein");
    // Root's fixpoint, as `ein saturate --dump` lists it.
    for (file, holds, lacks) in [
        (w("empty-match-false"), &["(false :rule r)"][..], &[][..]),
        (w("empty-match-fact"), &["(q b :rule r)", "(q a)"], &[]),
        (w("empty-match-activator"), &["(hit a :rule r)"], &[]),
        (w("assert-empty"), &["(q a)"], &["( :rule r)", "#empty"]),
    ] {
        let r = ein(&["saturate", "--dump", &file]);
        assert_eq!(r.code, 0, "{file}:\n{}", r.err);
        for fact in holds {
            assert!(
                r.out.lines().any(|l| l.trim() == *fact),
                "{file}: no {fact}:\n{}",
                r.out
            );
        }
        for not in lacks {
            assert!(!r.out.contains(not), "{file}: {not} is back:\n{}", r.out);
        }
    }
    // R, by describe and by the oracle: the state counts.
    for (file, states) in [
        (w("empty-match-false"), "0"),
        (w("empty-match-fact"), "4"),
        (w("empty-match-activator"), "2"),
        (w("empty-match-hrule"), "2"),
        (w("assert-empty"), "1"),
    ] {
        for cmd in ["solve", "oracle"] {
            let r = ein(&[cmd, &file]);
            assert_eq!(r.code, 0, "ein {cmd} {file}:\n{}", r.err);
            let line = r
                .out
                .lines()
                .find(|l| l.trim_start().starts_with("states (k_s)"))
                .unwrap_or_else(|| panic!("ein {cmd} {file}: no states line:\n{}", r.out));
            assert_eq!(
                line.split_whitespace().nth(2),
                Some(states),
                "ein {cmd} {file}: {line}"
            );
        }
    }
    // The dead root: the empty core, `r` its rule, on every text surface
    // that prints a core (Q-M1f.48), where until S1f.5.9 none was empty.
    let empty = format!(
        "{}, {}",
        ein_infer::explain::EMPTY_CORE,
        ein_infer::explain::EMPTY_CORE_GLOSS
    );
    let scratch = ein_corpus::Scratch::new("ein-empty-core");
    let dir = scratch.path();
    let trace = dir.join("t.md").to_string_lossy().into_owned();
    let lattice = dir.join("lattice").to_string_lossy().into_owned();
    let oracle = dir.join("oracle").to_string_lossy().into_owned();
    let r = ein(&[
        "solve",
        &w("empty-match-false"),
        "--trace",
        &trace,
        "--dump-lattice",
        &lattice,
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let row = |label: &str| -> Option<String> {
        r.out.lines().find_map(|l| {
            l.trim_start()
                .strip_prefix(label)
                .map(|v| v.trim().to_string())
        })
    };
    assert_eq!(
        row("unsat core").as_deref(),
        Some(empty.as_str()),
        "{}",
        r.out
    );
    assert_eq!(row("refuted by").as_deref(), Some("r"), "{}", r.out);
    let t = std::fs::read_to_string(&trace).expect("the trace");
    assert!(t.contains(&format!("Core: {empty}.")), "{t}");
    assert!(!t.contains("not kept by this route"), "{t}");
    let n = std::fs::read_to_string(dir.join("lattice/nodes.md")).expect("nodes.md");
    assert!(n.contains("| core ∅ |"), "{n}");
    let r = ein(&["oracle", &w("empty-match-false"), "--dump", &oracle]);
    assert_eq!(r.code, 0, "{}", r.err);
    let md = std::fs::read_to_string(dir.join("oracle/region.md")).expect("region.md");
    assert!(md.contains("core ∅ by r"), "{md}");
    // The hrule proposes its conclusion: H is `(q b)`.
    let r = ein(&["solve", "-H", &w("empty-match-hrule")]);
    assert!(r.out.lines().any(|l| l.trim() == "(q b)"), "{}", r.out);
}

/// **A relation parameter that can hold `eq` and heads a form of another arity
/// is refused at load, by every command, with one line** — M1f S1f.5.9's
/// T1f.5.9.4, the shape S1f.5.18 found (Q-M1f.46), and a written fact of `eq`
/// at another arity beside it (Q-M1f.47).
///
/// W60's four witnesses and W61's, each on every command that loads a program:
/// exit 1 (`ein test`: 2), and the loader's line in Ein's words, where until
/// S1f.5.9 the first stopped every command where the plan was compiled, with
/// the compiler's arity line in the Python IR's spelling; the second and the
/// third stopped `ein solve` while `ein saturate` answered; and the fourth and
/// W61's stored a fact of `eq` at one argument. What loads: the same parameter
/// heading a form of two arguments, `eq`'s guard (S1f.5.7), and a declared
/// relation of another arity, which matches nothing, as the pattern written
/// directly does.
#[test]
fn eq_at_another_arity_is_refused_at_load() {
    let w = |name: &str| format!("examples/ein-bugs/wrong-answers/{name}.ein");
    let tail = "— eq is the relation of equality, which holds between exactly two values";
    let refused: [(String, String); 5] = [
        (
            w("eq-arity-premise"),
            format!(
                "rule 'r': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (r \
                 eq) binds ?R to eq {tail}"
            ),
        ),
        (
            w("eq-arity-under-commitment"),
            format!(
                "rule 'r': its parameter ?R heads (?R ?x), a form of 1 argument, and ?R can be \
                 bound to eq by the fact (mark eq), through rule 'mk''s conclusion (r ?f) {tail}"
            ),
        ),
        (
            w("eq-arity-hrule"),
            format!(
                "hrule 'h': its parameter ?R heads (?R ?x), a form of 1 argument, and (solve \
                 :hrules (h (eq))) binds ?R to eq {tail}"
            ),
        ),
        (
            w("eq-arity-conclusion"),
            format!(
                "rule 'r': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (r \
                 eq) binds ?R to eq {tail}"
            ),
        ),
        (
            w("fact-eq-arity"),
            format!("the fact (eq a) writes eq with 1 argument {tail}"),
        ),
    ];
    let scratch = ein_corpus::Scratch::new("ein-eq-arity");
    let einb = scratch.path().join("x.einb").to_string_lossy().into_owned();
    for (file, want) in &refused {
        let want = format!("kb load error: {want}");
        for (args, code) in [
            (vec!["saturate", "--dump", file.as_str()], 1),
            (vec!["solve", file], 1),
            (vec!["solve", "--minimal", file], 1),
            (vec!["oracle", file], 1),
            (vec!["test", file], 2),
            (vec!["render", "lattice", file], 1),
            (vec!["render", "kb", file], 1),
            (vec!["kb", "save", file, &einb], 1),
        ] {
            let r = ein(&args);
            assert_eq!(r.code, code, "ein {args:?}:\n{}", r.err);
            assert!(
                r.err.lines().any(|l| l.trim() == want),
                "ein {args:?}: no line {want:?}:\n{}",
                r.err
            );
            for python in ["CompileError", "SForm(", "takes exactly"] {
                assert!(!r.err.contains(python), "`{python}` is back:\n{}", r.err);
            }
        }
    }
    // What loads: two arguments, the guard; and a declared binary relation
    // under the same one-argument pattern, which matches nothing.
    let ok = scratch.path().join("ok.ein");
    std::fs::write(
        &ok,
        "(relation p Node :why \"p holds of {?1}\") (relation q Node Node :why \"q holds of {?1} and {?2}\") (relation hit Node :why \"{?1} is hit\")\n\
         (p a) (q a b)\n\
         (rule two (?R) :match (and (p ?x) (?R ?x ?x)) :assert (hit ?x) :why \"p holds of {?x} and {?R} holds of {?x} and {?x}, so {?x} is hit\")\n(two eq)\n\
         (rule one (?R) :match (?R ?x) :assert (hit ?x) :why \"{?R} holds of {?x}, so {?x} is hit\")\n(one q)\n",
    )
    .expect("writes");
    let r = ein(&["saturate", "--dump", &ok.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("(hit a :rule two)"), "{}", r.out);
    assert!(!r.out.contains(":rule one"), "{}", r.out);
}

/// What the Python IR's spelling and the compiler's history put on stderr until
/// M1f S1f.5.21, none of which a command prints since (the close review's
/// finding 1): the class, the IR's repr, a position as `Loc(…)`, a stage id and
/// *the M1 compiler*.
const PYTHON_SPELLINGS: [&str; 7] = [
    "CompileError",
    "SForm(",
    "Atom(name=",
    "Var(name=",
    "Loc(",
    "S1.8.",
    "M1 compiler",
];

/// Every command that reads a program, with the file last and `{einb}` the
/// container `kb save` writes, and the exit it gives a program it refuses.
fn every_command(file: &str, einb: &str) -> Vec<(Vec<String>, i32)> {
    let cmds: [(&[&str], i32); 17] = [
        (&["saturate"], 1),
        (&["saturate", "--dump"], 1),
        (&["solve"], 1),
        (&["solve", "--minimal"], 1),
        (&["solve", "--maximal"], 1),
        (&["solve", "--single"], 1),
        (&["solve", "--count"], 1),
        (&["solve", "-n", "1"], 1),
        (&["oracle"], 1),
        (&["test"], 2),
        (&["render", "lattice"], 1),
        (&["render", "kb"], 1),
        (&["render", "program"], 1),
        (&["render", "rules"], 1),
        (&["render", "rule", "--name", "r"], 1),
        (&["render", "constraints"], 1),
        (&["kb", "save"], 1),
    ];
    cmds.iter()
        .map(|(c, code)| {
            let mut args: Vec<String> = c.iter().map(|s| s.to_string()).collect();
            args.push(file.to_string());
            if c == &["kb", "save"] {
                args.push(einb.to_string());
            }
            (args, *code)
        })
        .collect()
}

/// **A refusal the compiler made reads the same on every command, in Ein's
/// words, and is made at load** — M1f S1f.5.21, on the user's ruling on the
/// close review's finding 1 (*"new stage in p1f.5"*): W62, W63, W64 and W65.
///
/// Each witness on every command that reads a program, seventeen of them, the
/// views and `kb save` among them: exit 1 (`ein test`: 2) and one first line,
/// the loader's, behind `kb load error: `, with none of the Python IR's
/// spelling on stderr. Until S1f.5.21 the compiler refused each where it
/// compiled the plan, printed four ways (bare, behind
/// `ein.inference.compile.CompileError:`, behind the file's name, and `solve`
/// with the class where `oracle` printed it bare), while a command that never
/// compiled the plan answered: a commitment's activator, an hrule, a goal, a
/// rule no activator reached, and the four views and `kb save` always.
///
/// The last compile refusal a program reached, `(eq ?x)` in a macro's body,
/// is the loader's since M1f S1f.5.24 (W66, Q-M1f.80): one line on every
/// command, its position the macro body's, the views and `kb save` refusing it
/// too. Until then it was refused only where the rule was compiled, and the
/// views and `kb save` accepted the program; every shape a macro's body
/// malforms is held on every command by `macro_bodies.rs`.
#[test]
fn a_compile_refusal_reads_the_same_on_every_command() {
    let scratch = ein_corpus::Scratch::new("ein-compile-refusal");
    let einb = scratch.path().join("x.einb").to_string_lossy().into_owned();
    let nested = |who: &str, clause: &str, split: &str| {
        format!(
            "{who}: its {clause} writes (or (p ?x) (q ?x)) as a premise of and — an (or …) is a \
             whole {clause}, each of its premises one way it holds: lift it to the top of the \
             {clause}, or {split}"
        )
    };
    let rule = nested("rule 'r'", ":match", "split the rule");
    let goal = nested("query", ":goal", "ask two queries");
    let head = |form: &str| {
        format!(
            "query: its :goal writes {form} with ?R as its relation, which nothing binds — a \
             relation is named by a name, or by a rule's parameter its activator binds, and a \
             query has no activator"
        )
    };
    let w = |name: &str| format!("examples/ein-bugs/wrong-answers/{name}.ein");
    let witnesses: Vec<(String, String)> = vec![
        (w("nested-or-stops"), rule.clone()),
        (w("nested-or-under-commitment"), rule.clone()),
        (
            w("nested-or-hrule"),
            nested("hrule 'h'", ":match", "split the rule"),
        ),
        (w("nested-or-never-compiled"), rule.clone()),
        (w("nested-or-goal"), goal.clone()),
        (w("nested-or-goal-no-state"), goal),
        (
            w("wide-match-stops"),
            "rule 'wide': its :match writes 257 distinct variables, and a rule binds at most \
             256 — split the rule"
                .to_string(),
        ),
        (
            w("wide-match-never-compiled"),
            "rule 'wide': its :match writes 258 distinct variables, its parameters among them, \
             and a rule binds at most 256 — split the rule"
                .to_string(),
        ),
        (
            w("wide-goal"),
            "query: its :goal writes 257 distinct variables, and a goal binds at most 256 — ask \
             two queries"
                .to_string(),
        ),
        (
            "examples/ein-bugs/query-goal-free-head.ein".to_string(),
            head("(?R Rex Animal)"),
        ),
        (
            "examples/ein-bugs/query-goal-free-head-unsat.ein".to_string(),
            head("(?R a b)"),
        ),
        (
            "examples/broken/load/nested_or.ein".to_string(),
            rule.replace("rule 'r'", "rule 'nested-disjunction'"),
        ),
        (
            "examples/broken/load/eq_in_a_macro_body.ein".to_string(),
            "examples/broken/load/eq_in_a_macro_body.ein:23:18: rule 'r': its :match writes \
             (eq ?x), and eq takes exactly 2 arguments, not 1 — it compares two values, so \
             write one guard per pair"
                .to_string(),
        ),
    ];
    for (file, line) in &witnesses {
        let want = format!("kb load error: {line}");
        for (args, code) in every_command(file, &einb) {
            let argv: Vec<&str> = args.iter().map(String::as_str).collect();
            let r = ein(&argv);
            assert_eq!(r.code, code, "ein {argv:?}:\n{}", r.err);
            assert_eq!(r.err.lines().next(), Some(want.as_str()), "ein {argv:?}");
            for python in PYTHON_SPELLINGS {
                assert!(
                    !r.err.contains(python),
                    "ein {argv:?}: `{python}` is back:\n{}",
                    r.err
                );
            }
            assert!(
                !Path::new(&einb).exists(),
                "ein {argv:?} wrote a container of a refused program"
            );
        }
    }
}
