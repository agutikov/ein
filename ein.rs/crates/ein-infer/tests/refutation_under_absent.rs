//! **A refutation resting on an `absent` a commitment can still fill** — M1e
//! [S1e.2.3](../../../../docs/history/m1e_review_processing/README.md#s1e23--q-m1e9s-containment-a-diagnostic-for-a-refutation-resting-on-an-absent)'s
//! containment for
//! [Q-M1e.9](../../../../docs/history/m1e_review_processing/open_questions.md),
//! refused since M1e2
//! [S1e2.2.5](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md).
//!
//! ## What is unsound
//!
//! `dead` is **not** upward-closed under `absent`. The premise
//! [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md)
//! states — *`X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the KB is append-only and
//! nothing retracts* — establishes that `sat` is **inflationary**, not that it
//! is monotone in its input, and `(absent P)` is exactly what separates the
//! two. A twenty-line probe has `{(p A)}` dead and `{(p A), (q A)}` alive.
//!
//! ## From a warning to a refusal
//!
//! [D4](../../../../docs/history/m1e_review_processing/search_soundness_probes/d4_q_m1e9_upward_closure.md)
//! ruled **B** in M1e: narrow the claim and diagnose it, and this file was the
//! census the warning's default was set from. The user ruled ***forbid*** on
//! 2026-11-02 (M1e2 S1e2.4.1): over a relation a commitment can still extend,
//! `(absent P)` ⟹ `(false)` or `(not …)` is an obligation written as a
//! refutation, and a program that writes it is refused, the message naming a
//! form that says what was meant. On 2026-11-04 the user ruled its F2, *via,
//! per activator*: a refutation resting on such an `absent` only through a
//! fact a rule derived under it counts too. [`ein_infer::forbid`] is the
//! check, and this file holds what it refuses, exactly.

use std::collections::BTreeSet;
use std::path::Path;

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::SolveError;
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load, parse};

// The programs written for P1e2.1b's censuses: the two activator shapes.
#[allow(dead_code)]
#[path = "../examples/shared/written.rs"]
mod written;

/// D4's probe, as a **corpus** fixture.
///
/// The original was `plans/m1e_review_processing/…/probes/naf_upward_closure.ein`,
/// and since M1e closed it is kept with S1e.1.1's other evidence at
/// `docs/history/m1e_review_processing/search_soundness_probes/probes/`. That
/// copy is a record and nothing runs it. A probe that decides a soundness
/// question belongs where the gate sweeps, so it is banked under
/// `examples/ein-bugs/` with the three sibling fixtures that also state today's
/// answer and are meant to break.
const PROBE: &str = "examples/ein-bugs/naf-upward-closure.ein";

/// **What *forbid* refuses on the corpus and the register, exactly** — M1e2
/// [S1e2.2.5](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md),
/// 2026-11-04. Until then this was S1e.2.3's census of the exposed set, the
/// rules the warning named: nine over seven entries on 2026-08-30, fourteen
/// over ten by M1e2 S1e2.4.1's re-count, whose `direct` reading (thirteen
/// over nine) the user ruled *forbid* on, with F2 *via, per activator*.
///
/// Each row is the program and the first clause of its refusal's line: the
/// rule, what it concludes, and what it rests on. The `direct` rows are W3's
/// four witnesses, `W1/alive-empty-interlayer` and the two `branching/`
/// refusal fixtures; the `via` rows are W8's `stale-naf-fork` pair and the
/// fuzz finding `d3-unsat-core`. `features/05` and `/12`, `direct` too, close
/// the relation their refutations read and load. The set is exact because
/// both directions are findings: a new row is a program written with the
/// shape, and a missing one is a rule rewritten or a refusal lost.
const REFUSED: &str = "\
corpus/fuzz_findings/d3-unsat-core.ein::rule 'fire-0' [activator T]: concludes (not \u{2026}) from r1, which rule 'fire-1' derives resting on (absent \u{2026}) over (not r1), a relation a commitment can still extend
examples/branching/13_lookahead_naf_world.ein::rule 'self-block': concludes (false) from (absent \u{2026}) over cand, a relation a commitment can still extend
examples/branching/14_lookahead_unjudgeable.ein::rule 'premature-signoff': concludes (false) from (absent \u{2026}) over signed, a relation a commitment can still extend
examples/ein-bugs/alive-empty-interlayer.ein::rule 'totality': concludes (false) from (absent \u{2026}) over p, a relation a commitment can still extend
examples/ein-bugs/naf-upward-closure.ein::rule 'bad': concludes (false) from (absent \u{2026}) over q, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/naf-nowb.ein::rule 'bad': concludes (false) from (absent \u{2026}) over q, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein::rule 'p-and-q-clash': concludes (false) from p, which rule 'derive-p' derives resting on (absent \u{2026}) over q, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/stale-naf-fork.ein::rule 'p-and-q-clash': concludes (false) from p, which rule 'derive-p' derives resting on (absent \u{2026}) over q, a relation a commitment can still extend
examples/lattice/synth/three_components.ein::rule 'dead-3': concludes (false) from (absent \u{2026}) over h, a relation a commitment can still extend
examples/lattice/synth/three_components.ein::rule 'dead-7': concludes (false) from (absent \u{2026}) over h, a relation a commitment can still extend
examples/lattice/synth/trigger_hole.ein::rule 'dead-1': concludes (false) from (absent \u{2026}) over h, a relation a commitment can still extend
examples/lattice/synth/trigger_hole.ein::rule 'dead-2': concludes (false) from (absent \u{2026}) over h, a relation a commitment can still extend
";

/// The refusal of `text` (read from `at`), one row per refusing rule: the
/// line up to its em dash. Empty for a program that does not load, or loads
/// and is not refused.
fn refusal(rel: &str, text: &str, at: &Path) -> Vec<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Ok(forms) = parse(&mut ast, text, at.to_str()) else {
        return Vec::new();
    };
    let Ok(mut kb) = load(&mut ast, &mut terms, &forms, at.parent()) else {
        return Vec::new();
    };
    // Root's fixpoint, the universe and the static check, where the refusal
    // runs — `--monotone`'s question, and no route.
    match explore::check_monotone(&mut kb, &mut terms, &ast, &mut Events::off()) {
        Err(SolveError::Refused(m)) => m
            .lines()
            .map(|l| {
                let head = l.split(" \u{2014} ").next().unwrap_or(l);
                format!("{rel}::{head}")
            })
            .collect(),
        _ => Vec::new(),
    }
}

const DESCRIBE: Mode = Mode {
    which: Which::All,
    ask: Ask::Describe,
};

#[test]
fn the_refused_set_is_exactly_these() {
    let mut files = corpus_files();
    for row in ein_corpus::register().case {
        files.push(repo_root().join(&row.witness));
    }
    files.sort();
    files.dedup();
    let mut got: BTreeSet<String> = BTreeSet::new();
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        got.extend(refusal(&rel, &text, &path));
    }
    let want: BTreeSet<String> = REFUSED.lines().map(str::to_string).collect();
    let missing: Vec<&String> = want.difference(&got).collect();
    let extra: Vec<&String> = got.difference(&want).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "the refused set moved.\n  no longer refused: {missing:#?}\n  newly refused: {extra:#?}"
    );
}

/// **The two written activator programs** (`examples/shared/written.rs`),
/// which no corpus program has: F3's read of a rule with parameters refuses
/// `ACTIVATOR_REFUTES` at load, where no plan of `watch` exists at root, and
/// F2's *per activator* refuses `ACTIVATOR_UNDER_ABSENT`, whose refutation
/// reads no `absent` and whose activator rests on one.
#[test]
fn the_written_activator_programs_are_refused() {
    let at = repo_root().join("written.ein");
    let refutes = refusal("w", written::ACTIVATOR_REFUTES, &at);
    assert_eq!(
        refutes,
        [
            "w::rule 'watch': concludes (false) from (absent \u{2026}) over the relation its \
          parameter ?R names, and a commitment can derive its activator, so a node can \
          compile it over a relation a commitment can still extend"
        ]
    );
    let under = refusal("w", written::ACTIVATOR_UNDER_ABSENT, &at);
    assert_eq!(
        under,
        [
            "w::rule 'watch': concludes (false) from its activator, which rule 'act' derives \
          resting on (absent \u{2026}) over q, a relation a commitment can still extend"
        ]
    );
}

/// **The refusal is lifted only by `forbid::studying`**, the library-level
/// fixture's way past it, which no CLI path calls: inside it D4's probe
/// answers the R it had (three states, a dead hole), and after it the probe
/// is refused again.
#[test]
fn studying_lifts_the_refusal_and_only_while_it_runs() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let run = || {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
        let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
        explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(DESCRIBE))
    };
    assert!(matches!(run(), Err(SolveError::Refused(_))));
    let studied = ein_infer::forbid::studying(run).expect("studied, it answers");
    assert_eq!(studied.counts.map(|c| c.k_s), Some(3));
    assert!(!ein_infer::forbid::is_studying());
    assert!(matches!(run(), Err(SolveError::Refused(_))));
}

/// D4's probe, banked: refused, and the message names the replacements —
/// F4's three forms. The probe is a **mis-encoded obligation**: it says *a
/// world with `p` and without `q` is false* and never says *`q` is
/// required*. That reading is the user's, 2026-08-28, and it is why the
/// message has to say *instead* and not only *don't*.
#[test]
fn the_probe_is_refused_and_the_message_names_the_replacements() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let Err(SolveError::Refused(m)) =
        explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(DESCRIBE))
    else {
        panic!("the probe is not refused");
    };
    assert_eq!(m.lines().count(), 1, "one line, one rule: {m}");
    for want in [
        "rule 'bad'",
        "concludes (false)",
        "over q",
        "a relation a commitment can still extend",
        // The three forms the ruling measured.
        "(solve :no-hypothesis (q))",
        "functional + total-owed",
        "(open ?R)",
        "a stored negative",
        "total does",
    ] {
        assert!(m.contains(want), "the message does not say {want:?}:\n{m}");
    }
}

/// **The refusal is the stream's end, and no warning precedes it.** Until
/// M1e2 S1e2.2.5 the fixture's own `(config :warn-derived-naf true)` made S1e.2.3's
/// `RefutationUnderAbsentWarning` part of its recorded output, a `warn` line
/// after root's saturation; that sentence is the refusal's now, the flag is
/// retired and the fixture's line went. `ein solve FILE --events` narrates
/// root's saturation and then stops: no `warn` line, no `monotonicity` line,
/// no node.
#[test]
fn the_fixtures_stream_ends_at_the_refusal() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let buf = ein_infer::events::Buffer::new();
    let mut events = Events::to(Box::new(buf.clone()), ein_infer::events::Level::Normal);
    let opts = ein_infer::explore::Options::new(DESCRIBE);
    let out = ein_infer::explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events);
    assert!(
        matches!(out, Err(SolveError::Refused(_))),
        "the fixture is refused (M1e2 S1e2.2.5)"
    );
    let stream = buf.to_string_lossy();
    for kind in ["warn", "monotonicity", "node"] {
        assert!(
            !stream.contains(&format!("\"e\": \"{kind}\"")),
            "the run narrated a {kind} line before its refusal:\n{stream}"
        );
    }
    assert!(
        stream.contains("\"e\": \"quiesce\""),
        "root's saturation is narrated:\n{stream}"
    );
}

/// The one existing hazard signal stays silent here, and that is the finding
/// it was filed as: the stratification advisory (`warn_derived_naf`'s other
/// half, retired with it at M1e2 S1e2.2.5; the library's
/// `derived_naf_warnings` since) watches an `(absent …)` over a
/// **rule-derived** relation, and `q` is only ever proposed by the generator.
/// Two adjacent hazards, and until S1e.2.3 the engine had a warning for one.
#[test]
fn the_stratification_warning_does_not_cover_this() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let mut events = Events::off();
    let mut s = ein_infer::saturator::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
        memo: Default::default(),
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).expect("a saturator");
    sat.saturate(&mut s, None, &mut |_| {})
        .expect("it saturates");
    assert!(
        ein_infer::naf_deps::derived_naf_warnings(&sat.engine, s.terms).is_empty(),
        "the stratification warning fired on a hypothesis-eligible watch"
    );
}
