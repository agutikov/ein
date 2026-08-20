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
//! | `tests/test_events.py` | `--events`, whose spec is [`conformance/EVENTS.md`](../../../../conformance/EVENTS.md) |
//! | `tests/integration/test_zebra_parse.py` | that `zebra2.ein` and its two variants stay one encoding |
//!
//! Everything here runs the built binary (`CARGO_BIN_EXE_ein`) except three
//! claims that have no command line — `Events::off()`, root saturation, and
//! the id-order perturbation — and those say so where they sit.
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
//! result — see [`SCHEMA_KINDS`] and
//! [`the_injected_clash_is_refuted_at_root_saturation`].

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use ein_core::{FactId, IntId, Kb, ProvKind, Symbol, Tag, Terms, Value};
use ein_infer::events::{Buffer, Events, Level, sexpr};
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use ein_oracle::repo_root;
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
            .map(|v| v.trim().split_whitespace().next().unwrap_or("").to_string())
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
        let dir = std::env::temp_dir().join(format!("ein-cli-semantics-{}-{tag}", std::process::id()));
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
        *out.entry(e["e"].as_str().unwrap_or("?").to_string()).or_insert(0) += 1;
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
        has_query: p.query.is_some(),
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
    assert!(z.given.contains(COND_15), "condition (15) is in the canonical");
    assert!(!z.given.contains(INJECTED), "the canonical carries no clash");
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
