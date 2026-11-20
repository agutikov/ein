//! The search layer's semantics.
//!
//! Everything above the saturator and below the CLI: contradiction detection
//! ([`ein_infer::contradiction`]), the minimal-explanation search
//! ([`ein_infer::explain`]), guided hypothesis generation
//! ([`ein_infer::hypgen`]) and state identity ([`ein_infer::canon`]).
//!
//! | subject |
//! |---|
//! | the two contradiction shapes and the incremental check |
//! | `examples/saturation/`'s 24 per-rule demos |
//! | the OR-aware smallest frontier |
//! | `:hypothesis-relations` / `:no-hypothesis` |
//! | canonical model identity |
//! | the blind enumerator's solve |
//!
//! The routes' node function and licences are `region.rs`'s and
//! `explore.rs`'s tests. The two identity tests and the blind enumerator's
//! ask `--maximal`; the answer is `R`.
//!
//! Internals — the alternative-justification table, what a generator was
//! asked for — are not asserted; where one protects an observable, the
//! observable is what is asserted here.

use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use ein_core::{
    FactId, Justifications, Kb, ProvKind, SolverConfig, Symbol, Terms, Value, unsat_core,
};
use ein_infer::canon::state_key;
use ein_infer::contradiction::{Contradiction, detect};
use ein_infer::explain::{
    ExplanationBudget, minimal_contradiction_frontier, smallest_contradiction_frontier,
};
use ein_infer::hypgen::{HypGenStats, Skip};
use ein_infer::{Events, Firing, Saturator, Session, SharedMemo};
use ein_ir::{Ast, load_file, parse};
use rustc_hash::FxHashSet;

// ── Scaffolding ────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// Parse + load an inline fixture.
fn kb_of(src: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<fixture>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// Run to the fixpoint, returning the firings in order.
fn saturate(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Vec<Firing> {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("the fixture compiles");
    let mut out = Vec::new();
    sat.saturate(&mut s, None, &mut |f| out.push(f.clone()))
        .expect("the fixture saturates");
    out
}

/// Borrow a `Session` for the length of one call.
fn in_session<R>(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    f: impl FnOnce(&mut Session<'_>) -> R,
) -> R {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    f(&mut s)
}

/// Intern `(rel arg…)` — the proposition, whether or not any KB believes it.
fn fact(terms: &mut Terms, rel: &str, args: &[&str]) -> FactId {
    let r = terms.intern_text(rel).expect("room for a relation name");
    let mut vals = Vec::with_capacity(args.len());
    for a in args {
        vals.push(Value::sym(
            terms.intern_text(a).expect("room for an argument"),
        ));
    }
    terms.intern_fact(r, &vals).expect("room for a fact")
}

/// A KB's facts as sorted s-expressions — readable, and comparable across two
/// `Terms` arenas where a `FactId` is not.
fn sexprs(terms: &Terms, facts: impl IntoIterator<Item = FactId>) -> BTreeSet<String> {
    facts
        .into_iter()
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect()
}

/// The distinct relation names of a fact set, sorted.
fn rel_names(terms: &Terms, facts: &[FactId]) -> Vec<String> {
    let mut out: Vec<String> = facts
        .iter()
        .map(|&f| terms.sym(terms.facts.rel(f)).to_string())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// The rule that recorded a fact's primary derivation, if any.
fn primary_rule(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
    let p = kb.primary(f)?;
    terms.provs.get(p).rule.map(|r| terms.sym(r).to_string())
}

fn primary_kind(kb: &Kb, terms: &Terms, f: FactId) -> Option<ProvKind> {
    kb.primary(f).map(|p| terms.provs.get(p).kind)
}

/// A pair's `X`, or `None` for a contradiction that is no pair.
fn positive(c: &Contradiction) -> Option<FactId> {
    match *c {
        Contradiction::Pair { positive, .. } => Some(positive),
        _ => None,
    }
}

/// A pair's `(not X)`, or `None` for a contradiction that is no pair.
fn negative(c: &Contradiction) -> Option<FactId> {
    match *c {
        Contradiction::Pair { negative, .. } => Some(negative),
        _ => None,
    }
}

/// The union of every contradiction witness's premise closure, under one
/// reading of the OR-node.
fn union_core(kb: &Kb, terms: &Terms, how: Justifications) -> FxHashSet<FactId> {
    let w: Vec<FactId> = detect(kb, terms)
        .iter()
        .filter_map(|c| c.witness(terms))
        .collect();
    unsat_core(kb, terms, &w, how).into_iter().collect()
}

// ── Contradiction detection ────────────────────────────────────────

/// A pair is a contradiction however each side entered the KB, where the
/// program imports the rules that say so (`std.contradiction`, Q-M1f.557).
///
/// `(sits A S1)` and `(sits B S1)` are both authored clues; `one-per-seat`
/// derives the negation of each from the other. A detector that skipped a
/// pair whose facts sat in different knowledge layers would report **zero**
/// contradictions on this KB, flatly inconsistent with its own input.
/// The corpus-scale instance of the same bug is
/// `examples/ein-bugs/zebra2-bad.ein`, whose `Contradiction` verdict
/// `acceptance.rs` owns; this is the unit-level pin, and it checks the
/// provenance of both sides so that a detector which had merely stopped
/// looking at layers could not pass it by accident.
#[test]
fn mutual_negation_is_detected_whatever_either_sides_origin() {
    let (ast, mut terms, mut kb) = kb_of(
        "(import std.contradiction)\n\
         (relation sits Person Seat :why \"{?3} sits in seat {?4}\")\n(relation is-a Thing Thing :why \"{?3} is a {?4}\")\n\
         (is-a A Person) (is-a B Person)\n\
         (rule one-per-seat ()\n\
         \x20 :match  (and (sits ?p1 ?s) (is-a ?p2 Person) (neq ?p1 ?p2))\n\
         \x20 :assert (not (sits ?p2 ?s))\n\
         \x20 :why \"{?s} is taken by {?p1}\")\n\
         (sits A S1 :source \"clue (1)\")\n(sits B S1 :source \"clue (2)\")\n",
    );
    saturate(&ast, &mut terms, &mut kb);

    let found = detect(&kb, &terms);
    assert_eq!(
        found.len(),
        2,
        "{:?}",
        sexprs(&terms, found.iter().filter_map(negative))
    );
    assert_eq!(
        sexprs(&terms, found.iter().filter_map(positive)),
        ["(sits A S1)", "(sits B S1)"]
            .map(str::to_string)
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    for c in &found {
        let (x, not_x) = (positive(c).expect("a pair"), negative(c).expect("a pair"));
        assert_eq!(primary_kind(&kb, &terms, x), Some(ProvKind::Source));
        assert_eq!(
            primary_rule(&kb, &terms, not_x).as_deref(),
            Some("one-per-seat"),
            "the negative is rule-derived"
        );
    }
}

/// `detect` reports one `Contradiction` per distinct `(X, ¬X)` pair, not the
/// first one it meets.
///
/// The unsat core of a `k = 0` verdict is the union over every recorded dead
/// commitment's core, so a detector that stopped at the first pair would
/// narrow the explanation to whichever conflict the extent order happened to
/// put first — an answer that changes with the input's line order.
#[test]
fn every_pair_is_reported_not_only_the_first() {
    let (terms, kb) = {
        let (ast, mut terms, mut kb) = kb_of(
            "(import std.contradiction)\n(relation r T T :why \"r holds of {?3} and {?4}\")\n\
             (r A B) (not (r A B))\n(r C D) (not (r C D))\n(r E F) (not (r E F))\n",
        );
        saturate(&ast, &mut terms, &mut kb);
        (terms, kb)
    };
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 3);
    assert_eq!(
        sexprs(&terms, found.iter().filter_map(positive)),
        ["(r A B)", "(r C D)", "(r E F)"]
            .map(str::to_string)
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

/// The inner positive of a `(not …)` is matched by the structural identity of
/// its own nested-fact argument, one level further in than a flat `(p one)`.
///
/// `(not (hypothesis (co-located N H2)))` is the shape Q40 introduced and the
/// one the hypothesis-contradiction idiom actually writes. The detector's
/// lookup is `kb.contains(inner)`, so it is right at any depth *provided*
/// interning is structural all the way down — which is precisely what a
/// pointer-equality or top-level-only comparison would get wrong.
#[test]
fn a_nested_positive_pairs_through_structural_identity() {
    let (ast, mut terms, mut kb) = kb_of(
        "(import std.contradiction)\n\
         (relation hypothesis T :why \"{?3} is hypothesised\")\n(relation co-located T T :why \"{?3} is co-located with {?4}\")\n\
         (hypothesis (co-located Norwegian House-2))\n\
         (not (hypothesis (co-located Norwegian House-2)))\n",
    );
    saturate(&ast, &mut terms, &mut kb);
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        ein_infer::events::sexpr(&terms, positive(&found[0]).expect("a pair")),
        "(hypothesis (co-located Norwegian House-2))"
    );
    assert_eq!(found[0].kind(), ein_infer::contradiction::Kind::Pair);
}

/// A `(not 5)` — a negation whose argument is not a fact — yields no
/// contradiction and no panic.
///
/// Neither the loader nor the matcher produces this shape, which is exactly
/// why it needs a test: nothing but a hand-built fact exercises it, and the
/// tolerant reading is a decision (Q40 / R9) rather than an accident.
#[test]
fn a_malformed_negation_is_tolerated_rather_than_crashing() {
    let (_ast, mut terms, mut kb) =
        kb_of("(relation r T T :why \"r holds of {?3} and {?4}\")\n(r A B :source \"(1)\")\n");
    let not = terms.kernel.not;
    // `5` is a name: no fact.
    let five = terms.value_text("5").expect("room");
    let malformed = terms.intern_fact(not, &[five]).expect("room");
    kb.add_and_index_fact(&mut terms, not, &[five], None)
        .expect("room");

    assert!(kb.contains(malformed), "the fixture is in the KB");
    assert!(detect(&kb, &terms).is_empty());
    assert!(!ein_infer::has_contradiction(&kb, &terms));
}

/// `witness()` is the positive for a pair, and for a firing that ended the
/// state the first fact it matched: of a state that holds `(false)`, that
/// fact — in both shapes a fact the KB holds, so the unsat-core walk has a
/// derivation to seed from.
///
/// **A held `(false)` and a pair are contradictions by a rule's firing and
/// by nothing else**: the loaded KB, which no rule has run over, holds both
/// and no contradiction; saturated, `std.contradiction`'s `clash-2` has
/// concluded `(false)` from the pair, and the rule every program gets has
/// matched it and ended the state: of a stated `(false)` the firing is read
/// as itself, and of one the pair gave as the pair.
///
/// No ein.rs test called `witness()` before this one: every caller reaches it
/// through a digest, where returning the *negative* of a pair would still
/// produce a plausible-looking core (the negation's own premises) and would
/// only be wrong in that it explains the rebuttal instead of the claim.
#[test]
fn witness_is_the_positive_for_a_pair_and_the_matched_fact_for_an_end() {
    let (ast, mut terms, mut kb) = kb_of(
        "(import std.contradiction)\n\
         (relation r T T :why \"r holds of {?3} and {?4}\")\n(r A B :source \"(1)\")\n(not (r A B) :source \"(2)\")\n\
         (false :source \"(3)\")\n",
    );
    let loaded = detect(&kb, &terms);
    assert!(loaded.is_empty(), "{loaded:?}");

    saturate(&ast, &mut terms, &mut kb);
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 1, "{found:?}");

    let ended = found[0];
    assert_eq!(ended.kind(), ein_infer::contradiction::Kind::Ended);
    assert_eq!(positive(&ended), None);
    let falsum = ended.witness(&terms).expect("the fact it matched");
    assert_eq!(ein_infer::events::sexpr(&terms, falsum), "(false)");
    assert_eq!(ended.sides(&terms), [falsum]);
    let record = terms.provs.get(ended.ended().expect("its record"));
    assert_eq!(
        record.rule.map(|r| terms.sym(r)),
        Some("std.core.inconsistent")
    );

    // The same pair with no `(false)` stated: the death is the pair's.
    let (ast2, mut terms2, mut kb2) = kb_of(
        "(import std.contradiction)\n\
         (relation r T T :why \"r holds of {?3} and {?4}\")\n(r A B :source \"(1)\")\n(not (r A B) :source \"(2)\")\n",
    );
    saturate(&ast2, &mut terms2, &mut kb2);
    let pairs = detect(&kb2, &terms2);
    assert_eq!(pairs.len(), 1, "{pairs:?}");
    let pair = pairs[0];
    assert_eq!(pair.kind(), ein_infer::contradiction::Kind::Pair);
    assert_eq!(pair.witness(&terms2), positive(&pair));
    assert_ne!(pair.witness(&terms2), negative(&pair));
    for c in &pairs {
        let witness = c.witness(&terms2).expect("a fact");
        assert!(kb2.contains(witness), "the witness is believed");
        assert!(
            !smallest_contradiction_frontier(&kb2, &terms2, Some(&[witness])).is_empty(),
            "and it seeds a frontier"
        );
    }

    for c in &found {
        let witness = c.witness(&terms).expect("a fact");
        assert!(kb.contains(witness), "the witness is believed");
        assert!(
            !smallest_contradiction_frontier(&kb, &terms, Some(&[witness])).is_empty(),
            "and it seeds a frontier"
        );
    }
}

/// Replaying a saturation firing by firing, the first firing that ends the
/// state is exactly the firing at which a full scan first becomes non-empty.
///
/// This is the equivalence the fail-fast fork rests on, and it is a
/// claim about a *running* saturation rather than a static KB: the stop
/// reads the firing, and a firing that ended no state while a scan found a
/// contradiction would be a firing late during the run — which is the one
/// moment at which the answer is used.
#[test]
fn fail_fast_agrees_with_the_post_hoc_scan_over_a_real_saturation() {
    let (ast, mut terms, mut kb) = kb_of(
        "(import std.contradiction)\n\
         (rule h1-implies-x () :match (h1 ?x) :assert (x ?x) :why \"h1 holds of {?x}, so x holds of {?x}\")\n\
         (rule h2-forbids-x () :match (h2 ?x) :assert (not (x ?x)) :why \"h2 holds of {?x}, so x does not\")\n\
         (relation h1 T :why \"h1 holds of {?3}\") (relation h2 T :why \"h2 holds of {?3}\") (relation x T :why \"x holds of {?3}\")\n(is-a a T)\n(h1 a) (h2 a)\n",
    );
    let (first_incremental, first_scan) = in_session(&ast, &mut terms, &mut kb, |s| {
        let mut sat = Saturator::new(s).expect("compiles");
        let (mut incremental, mut scan, mut i) = (None, None, 0usize);
        while let Some(firing) = sat.step(s).expect("saturates") {
            i += 1;
            if scan.is_none() && !detect(s.kb, s.terms).is_empty() {
                scan = Some(i);
            }
            if incremental.is_none() && !firing.redundant && firing.ends {
                incremental = Some(i);
            }
        }
        (incremental, scan)
    });
    assert!(first_incremental.is_some(), "the fixture must die");
    assert_eq!(first_incremental, first_scan);
}

// ── The per-rule demos ─────────────────────────────────────────────

/// The eight rule directories, each with three scenarios.
const DEMO_RULES: [&str; 8] = [
    "hypothesis-contradiction",
    "implies",
    "square-bwd",
    "square-fwd",
    "square-unique",
    "symmetric",
    "transitive",
    "type-exclusivity",
];

/// `(<rule name>, <path>)` for every demo, sorted.
fn demos() -> Vec<(String, PathBuf)> {
    let root = repo_root().join("examples/saturation");
    let mut out = Vec::new();
    for rule in DEMO_RULES {
        let dir = root.join(rule);
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
            .map(|e| e.expect("dir entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "ein"))
            .collect();
        files.sort();
        out.extend(files.into_iter().map(|p| (rule.to_string(), p)));
    }
    out
}

fn demo_id(path: &Path) -> String {
    let stem = path.file_stem().expect("a file").to_string_lossy();
    let dir = path
        .parent()
        .and_then(Path::file_name)
        .expect("a directory")
        .to_string_lossy();
    format!("{dir}/{stem}")
}

/// `examples/saturation/` is exactly eight rule directories of three scenarios
/// — 24 files, no more and no fewer.
///
/// The layout is load-bearing rather than tidy: every other demo test derives
/// the rule name it expects from the *directory* name, so a demo filed under
/// the wrong rule, or a ninth rule added without scenarios, would make those
/// tests weaker without making them fail.
#[test]
fn the_demo_directory_layout_is_eight_rules_times_three_scenarios() {
    let root = repo_root().join("examples/saturation");
    let mut dirs: Vec<String> = std::fs::read_dir(&root)
        .expect("examples/saturation")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.is_dir())
        .map(|p| {
            p.file_name()
                .expect("a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    dirs.sort();
    assert_eq!(dirs, DEMO_RULES);

    let found = demos();
    assert_eq!(found.len(), 24, "8 rules × 3 scenarios");
    for rule in DEMO_RULES {
        assert_eq!(
            found.iter().filter(|(r, _)| r == rule).count(),
            3,
            "{rule}/ should hold three scenarios"
        );
    }
}

/// Each demo's directory name is a rule name, and saturating the demo fires
/// that rule.
///
/// The demos are documentation that runs: their whole claim is "this is what
/// rule X looks like doing its job". A demo that saturates fine while never
/// firing the rule it is filed under still renders, still passes the corpus
/// digests and teaches the reader the wrong thing.
#[test]
fn each_demo_fires_the_rule_its_directory_names() {
    for (rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let firings = saturate(&ast, &mut terms, &mut kb);
        let seen: Vec<&str> = firings.iter().map(|f| terms.sym(f.rule)).collect();
        assert!(
            seen.contains(&rule.as_str()),
            "{}: {rule:?} did not fire; observed {seen:?}",
            demo_id(&path)
        );
    }
}

/// At least one of the named rule's firings concludes a **genuinely derived**
/// fact rather than re-deriving an authored one.
///
/// The two are indistinguishable from a firing count. `symmetric` matches
/// `(rel A B)`, derives `(rel B A)`, then matches `(rel B A)` and re-derives
/// `(rel A B)` — which already exists, so the insert dedupes back onto the
/// authored fact and keeps its `source` provenance. A demo whose *every*
/// firing bounced that way would fire the rule and show nothing, so the
/// provenance of the conclusion is what has to be checked.
#[test]
fn each_demo_produces_a_genuinely_derived_fact() {
    for (rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let firings = saturate(&ast, &mut terms, &mut kb);
        let kinds: Vec<Option<ProvKind>> = firings
            .iter()
            .filter(|f| terms.sym(f.rule) == rule)
            .filter_map(|f| f.derived.first().copied())
            .map(|d| primary_kind(&kb, &terms, d))
            .collect();
        assert!(
            kinds.contains(&Some(ProvKind::Rule)),
            "{}: {rule:?} fired but derived nothing new; provenance kinds {kinds:?}",
            demo_id(&path)
        );
    }
}

/// Every demo carries a `(query … :goal …)` naming the fact its rule is
/// expected to produce.
///
/// The goal is the demo's assertion in the file itself — the machine-readable
/// half of the prose header — and it is what lets a reader (or the trace
/// renderer) check the demo against its own claim rather than against this
/// test's idea of it.
#[test]
fn each_demo_carries_a_query_goal() {
    for (_rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let query = kb
            .program()
            .query()
            .unwrap_or_else(|| panic!("{}: no (query …) block", demo_id(&path)));
        assert!(
            ein_infer::query_value(&ast, query, "goal").is_some(),
            "{}: (query …) has no :goal",
            demo_id(&path)
        );
    }
}

// ── The smallest frontier ──────────────────────────────────────────

/// One fact with two derivations, plus one clash — the R3 report's E3 fixture.
///
/// `(X a)` follows from `(A a) ∧ (B a)` via `join` and from `(C a)` alone via
/// `chain`; `(X a) ∧ (Y a)` is absurd. Only the two rules' written order
/// moves between instantiations, and the first written fires first (FIFO,
/// Q-M1f.313) — so which
/// derivation wins the dedup race and becomes `(X a)`'s *primary* provenance
/// flips with it.
fn two_derivations(join_first: bool) -> String {
    let join = "(rule join ()\n  :match (and (A ?o) (B ?o)) :assert (X ?o)\n\
                \x20 :why \"A and B both hold of {?o}, so X holds of {?o}\")\n";
    let chain = "(rule chain ()\n  :match (C ?o) :assert (X ?o)\n\
                 \x20 :why \"C holds of {?o}, so X holds of {?o}\")\n";
    let (first, second) = if join_first {
        (join, chain)
    } else {
        (chain, join)
    };
    format!(
        "(relation A T :why \"A holds of {{?3}}\")\n(relation B T :why \"B holds of {{?3}}\")\n(relation C T :why \"C holds of {{?3}}\")\n(relation X T :why \"X holds of {{?3}}\")\n(relation Y T :why \"Y holds of {{?3}}\")\n\
         {first}{second}\
         (rule clash ()\n  :match (and (X ?o) (Y ?o)) :assert (false)\n\
         \x20 :why \"X and Y both hold of {{?o}}, which cannot be\")\n\
         (A a :source \"(A)\")\n(B a :source \"(B)\")\n(C a :source \"(C)\")\n\
         (Y a :source \"(Y)\")\n"
    )
}

/// A single functional clash — one `(false)` witness whose frontier is the
/// clashing pair.
const FUNCTIONAL: &str = "\
(rule functional (?R)
  :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
  :assert (false)
  :why    \"{?R} is functional, yet gives {?a} both {?b} and {?c}\")
(relation R T T :why \"R holds of {?3} and {?4}\")
(functional R)
(R x One :source \"(1)\")
(R x Two :source \"(2)\")
";

fn saturated(src: &str) -> (Ast, Terms, Kb) {
    let (ast, mut terms, mut kb) = kb_of(src);
    saturate(&ast, &mut terms, &mut kb);
    (ast, terms, kb)
}

/// Flipping the two rules' written order flips which derivation is recorded
/// primary and leaves the
/// reported frontier identical.
///
/// With one justification per fact, the frontier would be `{C, Y}` when
/// `chain` fired first and `{A, B, Y}` when `join` did — even though the
/// two-fact explanation still exists in the second run and nothing about the
/// puzzle has changed. The store records the loser as an alternative
/// (Q18) and the search walks the AND/OR graph,
/// so what a reader is told no longer depends on a dedup race. The primary
/// provenance is asserted here *because* it still flips: that is what shows
/// the fix is in the search rather than in which derivation wins.
#[test]
fn the_reported_frontier_does_not_depend_on_which_derivation_fired_first() {
    for (join_first, expected_primary) in [(false, "chain"), (true, "join")] {
        let (_ast, terms, kb) = saturated(&two_derivations(join_first));
        let frontier = smallest_contradiction_frontier(&kb, &terms, None);
        assert_eq!(
            rel_names(&terms, &frontier),
            ["C", "Y"],
            "join written first: {join_first}"
        );

        let mut terms = terms;
        let x_a = fact(&mut terms, "X", &["a"]);
        assert_eq!(
            primary_rule(&kb, &terms, x_a).as_deref(),
            Some(expected_primary),
            "the dedup race still has a winner"
        );
        let alts: Vec<String> = kb
            .alternatives(x_a)
            .iter()
            .filter_map(|&p| terms.provs.get(p).rule)
            .map(|r| terms.sym(r).to_string())
            .collect();
        assert_eq!(alts.len(), 1, "the loser is retained: {alts:?}");
        assert_ne!(alts[0], expected_primary);
    }
}

/// An explanation names only facts the **OR-aware** premise closure reaches,
/// and on the flipped run it is deliberately *not* a subset of the
/// primary-only union core.
///
/// Half a soundness envelope is not one. Every explanation is inside the
/// `All` closure — that is what makes it a real set of derivation leaves — but
/// an implementation that quietly returned `unsat_core(witnesses, Primary)`
/// would satisfy that half on every input. The second assertion is what
/// separates them: in the `join`-first run the primary-only core is
/// `{A, B, Y}`, and reporting `{C, Y}` means stepping *outside* it. That
/// over-report is exactly what the AND/OR search rules out.
#[test]
fn an_explanation_is_a_subset_of_the_or_aware_closure_and_not_of_the_primary_only_union() {
    let (_ast, terms, kb) = saturated(FUNCTIONAL);
    let frontier = smallest_contradiction_frontier(&kb, &terms, None);
    assert!(!frontier.is_empty(), "a single clash still explains itself");
    let primary = union_core(&kb, &terms, Justifications::Primary);
    assert!(
        frontier.iter().all(|f| primary.contains(f)),
        "with one derivation per fact the two readings coincide"
    );

    let (_ast, terms, kb) = saturated(&two_derivations(true));
    let frontier = smallest_contradiction_frontier(&kb, &terms, None);
    let all = union_core(&kb, &terms, Justifications::All);
    assert!(
        frontier.iter().all(|f| all.contains(f)),
        "sound: {:?} ⊄ {:?}",
        sexprs(&terms, frontier.iter().copied()),
        // The assertion above is `⊆`, which has no order; the set below is
        // only rendered once it has already failed, and nothing reads it.
        // determinism-ok: an `⊆` assertion's failure message.
        sexprs(&terms, all.iter().copied())
    );
    let primary = union_core(&kb, &terms, Justifications::Primary);
    assert!(
        !frontier.iter().all(|f| primary.contains(f)),
        "the union core is {:?} — an explanation confined to it would be the \
         pre-S1.21.7 over-report",
        sexprs(&terms, primary.iter().copied())
    );
}

/// An unbudgeted search reports exhaustion; the same search under
/// `max_rounds = 1` says it did not, and stays sound.
///
/// Minimum-cardinality source frontier over an AND/OR graph is worst-case
/// exponential, so every truncation is legitimate — but a truncated search
/// that still claimed `exhausted` would present "possibly not smallest" as
/// "proven smallest", and there is nothing downstream that could tell the
/// difference. `rounds > 0` and a named `target` are the other half: an
/// implementation that returned the recorded fallback would also be sound and
/// would also be `exhausted = false`, and it is the fallback this budget is
/// *not* supposed to reach.
#[test]
fn a_full_search_reports_exhaustion_and_a_budgeted_one_admits_it_did_not() {
    let (_ast, terms, kb) = saturated(&two_derivations(true));

    let full = minimal_contradiction_frontier(&kb, &terms, None, &ExplanationBudget::default());
    assert!(full.exhausted);
    assert!(full.target.is_some());
    assert!(full.rounds > 0);
    assert_eq!(rel_names(&terms, &full.frontier), ["C", "Y"]);

    let tight = minimal_contradiction_frontier(
        &kb,
        &terms,
        None,
        &ExplanationBudget {
            max_rounds: 1,
            ..ExplanationBudget::default()
        },
    );
    assert!(!tight.exhausted, "a capped search must admit it was capped");
    let envelope = union_core(&kb, &terms, Justifications::All);
    assert!(
        tight.frontier.iter().all(|f| envelope.contains(f)),
        "a truncated frontier is still a real set of derivation leaves"
    );
}

/// Turning alternative recording off restores the pre-S1.21.7 answer — the
/// gate moves the *frontier*, not merely a counter.
///
/// A config flag that only stopped a table from filling would be untestable
/// from the outside and indistinguishable from dead configuration. Here the
/// same saturated KB reports `{A, B, Y}` with the flag off and `{C, Y}` with
/// it on, which is what makes it a real off switch: the alternatives are the
/// only reason the smaller explanation is findable at all.
#[test]
fn turning_alternative_recording_off_restores_the_primary_only_frontier() {
    let src = two_derivations(true);

    let (ast, mut terms, mut kb) = kb_of(&src);
    kb.program_mut().config = Some(SolverConfig {
        record_alternative_justifications: false,
        ..SolverConfig::default()
    });
    saturate(&ast, &mut terms, &mut kb);
    assert!(
        !kb.has_alternative_justifications(),
        "the alternatives table stays empty"
    );
    assert_eq!(
        rel_names(&terms, &smallest_contradiction_frontier(&kb, &terms, None)),
        ["A", "B", "Y"],
        "the recorded-primary answer"
    );

    let (_ast, terms, kb) = saturated(&src);
    assert!(
        kb.has_alternative_justifications(),
        "the control records them"
    );
    assert_eq!(
        rel_names(&terms, &smallest_contradiction_frontier(&kb, &terms, None)),
        ["C", "Y"]
    );
}

// ── Guided hypothesis generation ───────────────────────────────────

/// Two declared relations over two objects — the blind enumerator proposes
/// both unless a query keyword narrows it.
const TWO_REL: &str = "(relation co-located T T :why \"{?3} is co-located with {?4}\")\n(relation likes T T :why \"{?3} likes {?4}\")\n\
                       (is-a Thing T)\n(is-a A Thing) (is-a B Thing)\n";

/// The candidates H's pass emits (`generate_config_free`), plus its stats.
fn candidates(src: &str) -> (BTreeSet<String>, Vec<String>, HypGenStats) {
    let (ast, mut terms, mut kb) = kb_of(src);
    let mut stats = HypGenStats::new();
    let mut out = Vec::new();
    in_session(&ast, &mut terms, &mut kb, |s| {
        ein_infer::hypgen::generate_config_free(s, &mut stats, &mut |f| {
            out.push(f);
            ControlFlow::Continue(())
        })
        .expect("generates")
    });
    let rels = rel_names(&terms, &out);
    (sexprs(&terms, out), rels, stats)
}

/// A `:hypothesis-relations` / `:no-hypothesis` value is a list, and a bare
/// name is refused at the parse: the grammar takes a list, the one form.
///
/// Until the swap a bare SYMBOL meant exactly what the one-element list
/// means, which this test held because no corpus file uses the bare form and
/// nothing else in the suite would notice if `coerce_relation_names` stopped
/// reading an `Atom` value and fell through to the "keyword absent" branch —
/// which for a whitelist is *unrestricted*, the filter silently turning itself
/// off. Since the swap the bare form does not parse, so it cannot turn the
/// filter off either: it is refused at the name, whose repair is the list.
#[test]
fn lattice_relation_keyword_accepts_a_bare_symbol() {
    let goal = "(query :goal (co-located A B))";
    for (bare, list, col) in [
        (
            ":hypothesis-relations co-located",
            ":hypothesis-relations (co-located)",
            30,
        ),
        (":no-hypothesis likes", ":no-hypothesis (likes)", 23),
    ] {
        let name = bare.split_whitespace().nth(1).expect("a name");
        let refused = parse(
            &mut Ast::new(),
            &format!("{TWO_REL}(solve {bare}) {goal}"),
            None,
        )
        .expect_err("a bare name is refused at the parse");
        assert_eq!(
            refused.errors[0].what,
            format!("unexpected `{name}`: a name"),
            "{bare}: {refused}"
        );
        assert_eq!(refused.position(), (5, col), "{bare}: {refused}");
        assert_eq!(
            refused.errors[0].repairs[0],
            format!("Insert `(`, Shift `{name}`, Insert `)`"),
            "{bare}: the first repair is the list: {refused}"
        );
        let (list_facts, list_rels, _) = candidates(&format!("{TWO_REL}(solve {list}) {goal}"));
        assert!(!list_facts.is_empty(), "{list}: nothing generated at all");
        assert_eq!(list_rels, ["co-located"], "{list}");
    }
}

/// A relation named by both `:hypothesis-relations` and `:no-hypothesis`
/// yields nothing, and the skip is attributed to the **blacklist**.
///
/// Which counter it lands in is the observable that says the two filters are
/// ordered rather than merely both applied. If the blacklist ran first, the
/// relation would never reach the whitelist test and the skip would be
/// reported as `no_hypothesis_relation` too — so the discriminating assertion
/// is the *other* counter staying at zero, which pins that the whitelist
/// admitted the relation and the blacklist then took it away.
#[test]
fn no_hypothesis_overrides_the_whitelist() {
    let (facts, rels, stats) = candidates(&format!(
        "{TWO_REL}(solve :hypothesis-relations (co-located likes) :no-hypothesis (likes)) \
         (query :goal (co-located A B))"
    ));
    assert!(!facts.is_empty());
    assert_eq!(rels, ["co-located"], "likes is excluded");
    assert!(
        stats.pre_candidate[Skip::NoHypothesisRelation as usize] > 0,
        "the blacklist is what skipped it"
    );
    assert_eq!(
        stats.pre_candidate[Skip::RelationNotWhitelisted as usize],
        0,
        "the whitelist admitted it first"
    );
}

// ── Canonical state identity ───────────────────────────────────────

/// `ein solve FILE --maximal`: R's maximal line, exhausted, each state's
/// fixpoint handed to `read` as the node function every route uses builds it
/// ([`ein_infer::region::Nodes`]).
fn maximal_line<R>(rel: &str, read: impl FnOnce(&mut [Kb], &mut Terms) -> R) -> R {
    use ein_infer::explore::{self, Ask, Mode, Which};
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let a = explore::run(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(Mode {
            which: Which::Maximal,
            ask: Ask::Describe,
        }),
    )
    .unwrap_or_else(|e| panic!("{rel} solves: {e}"));
    assert!(a.exhausted, "{rel}: the maximal line is not known");
    let sets: Vec<Vec<FactId>> = a
        .states
        .iter()
        .map(|s| {
            s.set
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect()
        })
        .collect();
    let mut nodes =
        ein_infer::region::Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).expect("prepares");
    nodes.with_fixpoints(&sets, read).expect("saturates")
}

/// Solution-node identity is the canonical fact list itself, never a digest of
/// it — so two distinct closed models can never merge.
///
/// `state_key` *returns* the sorted fact list and every identity site compares lists, so there is no hash in the
/// path to collide — which is the claim, and the first assertion below is how
/// it is checked rather than argued. The second is the consequence the bug
/// report (REVIEW_M1-01 §1) was written about: under hash-as-identity
/// `branching/04` collapsed to a falsely-certified `Solution`, so `k == 2`
/// with two genuinely different fact sets is the observable that would have
/// caught it.
#[test]
fn state_key_is_identity_not_a_digest() {
    maximal_line("examples/branching/04_two_levels.ein", |states, terms| {
        assert_eq!(states.len(), 2, "two maximal states");
        for kb in states.iter() {
            let key = state_key(kb);
            let mut facts: Vec<FactId> = kb.facts().collect();
            facts.sort_unstable();
            assert_eq!(&*key, &facts[..], "the key is the representation");
        }
        assert_ne!(
            sexprs(terms, states[0].facts()),
            sexprs(terms, states[1].facts()),
            "two states that were never merged hold different facts"
        );
    });
}

/// Two different puzzles' root closures are different states, and their keys
/// differ.
///
/// Both are loaded into **one** `Terms`, so the two key vectors are written in
/// the same alphabet and a difference between them is a difference between the
/// KBs rather than between two interning orders. Loading `zebra2` twice is the
/// control: without it, "the keys differ" would also be satisfied by a key
/// that carried arena-local noise.
#[test]
fn distinct_closures_have_distinct_keys() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let root = repo_root();
    let mut key_of = |rel: &str| {
        let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        saturate(&ast, &mut terms, &mut kb);
        state_key(&kb)
    };
    let zebra2 = key_of("examples/zebra2.ein");
    let again = key_of("examples/zebra2.ein");
    let minus_15 = key_of("examples/zebra2-minus-15.ein");

    assert!(!zebra2.is_empty());
    assert_eq!(zebra2, again, "one fixture, two runs, one closure");
    assert_ne!(zebra2, minus_15);
}

// ── The blind enumerator's solve ───────────────────────────────────

const BLIND: &str = "examples/branching/12_typed_blind_solve.ein";

/// A puzzle with no a rule that concludes `(open F)` solves on the blind combinatorial path: its
/// maximal line is the puzzle's two models, the two colourings of H2 and H3
/// that the anchor `(color-of H1 Red)` leaves. The walk, asked for its first
/// model, reported one; R holds both.
///
/// The acceptance program (`zebra2`) states its hypotheses with a rule, so it
/// never enters the blind branch of the generator at all. Without a fixture
/// that both takes that path *and* converges, a change to the blind
/// enumerator would be a silent change: the demos that run blind abort at
/// `k = 0`, so they cannot witness "still solves". The `hypotheses.is_empty()`
/// assertion is what keeps this test on the path it names — a rule that
/// concludes a hypothesis added to the fixture would make the solve pass
/// while testing nothing.
#[test]
fn the_blind_path_solves_to_its_two_models() {
    let k = maximal_line(BLIND, |states, _| states.len());
    assert_eq!(k, 2, "blind-path fixture must reach its two maximal states");

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(BLIND)).expect("loads");
    assert!(
        kb.program().hypotheses.is_empty(),
        "the fixture must stay blind-path"
    );
}

/// Each recovered model is a total House → Colour bijection, not a partial
/// dead-end certified as complete.
///
/// `complete` is the emptiness test on the open-hypothesis set, so a KB can be
/// "complete" for reasons that have nothing to do with the puzzle being
/// answered — every remaining candidate filtered away, say. Counting the
/// positives and checking both projections is total is what distinguishes a
/// model from a stuck search: three cells, three houses, three colours, and
/// the authored anchor still standing.
#[test]
fn each_blind_model_is_a_total_bijection() {
    let models: Vec<Vec<(String, String)>> = maximal_line(BLIND, |states, terms| {
        let color_of: Symbol = terms.syms.get("color-of").expect("interned");
        states
            .iter()
            .map(|kb| {
                kb.facts_of(color_of)
                    .map(|f| {
                        let args = terms.facts.args(f);
                        assert_eq!(args.len(), 2, "color-of is binary");
                        let name = |v: Value| terms.sym(v.as_sym().expect("a symbol")).to_string();
                        (name(args[0]), name(args[1]))
                    })
                    .collect()
            })
            .collect()
    });
    assert_eq!(models.len(), 2);
    for cells in models {
        assert_eq!(
            cells.len(),
            3,
            "a bijection has exactly 3 positives: {cells:?}"
        );
        let houses: BTreeSet<&str> = cells.iter().map(|(h, _)| h.as_str()).collect();
        let colours: BTreeSet<&str> = cells.iter().map(|(_, c)| c.as_str()).collect();
        assert_eq!(houses, ["H1", "H2", "H3"].into_iter().collect());
        assert_eq!(colours, ["Blue", "Green", "Red"].into_iter().collect());
        assert!(
            cells.contains(&("H1".to_string(), "Red".to_string())),
            "the authored anchor is respected: {cells:?}"
        );
    }
}
