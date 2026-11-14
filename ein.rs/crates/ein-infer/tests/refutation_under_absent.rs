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
/// fuzz finding `d3-unsat-core`, a corpus entry since M1e2 S1e2.2.12
/// promoted it (2026-11-04); the three `per activator` rows are W16's second
/// shape, banked by S1e2.2.16 the same day: the symbol control, and its int
/// and nested-fact twins, which loaded until W16's fix, *forbid* reading an
/// activator with no symbol argument as none; and W17's family (M1e2
/// S1e2.2.17, 2026-11-04): `forbid-chain-0`, its control, an activator `mk`
/// derives resting on an `absent` compiling a refutation root has no plan of,
/// refused since S1e2.2.5 because its activator relation is in D, and its
/// five witnesses, which loaded until *forbid* followed the chain past D:
/// one and two links longer, the `absent` in a plan a node compiles (over a
/// relation, and over a parameter's), and over a relation only such a plan
/// writes, `x`, in D⁺ and not in D.
/// Since M1f S1f.5.5's swap the int twin's activator is the name `1`, a run
/// of digits being a name (S1f.5.4's ruling), so its line names it,
/// `[activator 1]`, as the symbol control's names `a`. Since M1f S1f.5.18
/// the nested twin's names its fact, `[activator (box a)]`, where it named
/// none: `Plan::activator_args` keeps every argument (Q-M1f.43).
/// `features/05` and `/12`, `direct` too, close
/// the relation their refutations read and load. The set is exact because
/// both directions are findings: a new row is a program written with the
/// shape, and a missing one is a rule rewritten or a refusal lost.
const REFUSED: &str = "\
examples/branching/13_lookahead_naf_world.ein::rule 'self-block': concludes (false) from (absent \u{2026}) over cand, a relation a commitment can still extend
examples/branching/14_lookahead_unjudgeable.ein::rule 'premature-signoff': concludes (false) from (absent \u{2026}) over signed, a relation a commitment can still extend
examples/ein-bugs/activator-all-int.ein::rule 'alarm' [activator 1]: concludes (false) from its activator, which rule 'watch' derives resting on (absent \u{2026}) over checked, a relation a commitment can still extend
examples/ein-bugs/activator-all-nested.ein::rule 'alarm' [activator (box a)]: concludes (false) from its activator, which rule 'watch' derives resting on (absent \u{2026}) over checked, a relation a commitment can still extend
examples/ein-bugs/activator-all-symbol.ein::rule 'alarm' [activator a]: concludes (false) from its activator, which rule 'watch' derives resting on (absent \u{2026}) over checked, a relation a commitment can still extend
examples/ein-bugs/alive-empty-interlayer.ein::rule 'totality': concludes (false) from (absent \u{2026}) over p, a relation a commitment can still extend
examples/ein-bugs/d3-unsat-core.ein::rule 'fire-0' [activator T]: concludes (not \u{2026}) from r1, which rule 'fire-1' derives resting on (absent \u{2026}) over (not r1), a relation a commitment can still extend
examples/ein-bugs/naf-upward-closure.ein::rule 'bad': concludes (false) from (absent \u{2026}) over q, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-0.ein::rule 'q': concludes (false) from its activator, which rule 'mk' derives resting on (absent \u{2026}) over p2, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-1.ein::rule 'r2': concludes (false) from its activator, which rule 'q' derives resting on (absent \u{2026}) over p2, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-2.ein::rule 'r3': concludes (false) from its activator, which rule 'r2' derives resting on (absent \u{2026}) over p2, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-node-absent.ein::rule 'r2': concludes (false) from its activator, which rule 'q' derives resting on (absent \u{2026}) over p2, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-node-writes.ein::rule 'chk': concludes (false) from (absent \u{2026}) over x, a relation a commitment can still extend
examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein::rule 'bad': concludes (false) from y, which rule 'watch' derives resting on (absent \u{2026}) over the relation its parameter ?R names, and a commitment can derive its activator, so a node can compile it over a relation a commitment can still extend
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

/// **Beside an hrule, the first remedy names the hrule** — M1f S1f.6.34, E-47.
///
/// E-47's witness (S1f.6.28's T4): *what is not listed is zero* adopted for
/// `has`, an hrule `maybe-apples` supposing Alice has an apple, and `(solve
/// :hrules (maybe-apples) :no-hypothesis (has))`. *Forbid* refused it with four
/// lines whose first remedy was `(solve :no-hypothesis (has))`, the form the
/// program already wrote, and one that does nothing where an hrule proposes
/// the relation: no list filters an hrule's candidates. Every command refuses
/// the program at load since, for the two conflicts it writes
/// (`ein-cli/tests/mechanism_conflicts.rs`); the library's routes, which do
/// not run that check, still reach *forbid*, and its lines name the hrule.
/// Without the list the remedy is the same; the twin without `listed_is_all`
/// reads no `absent` over `has`, and *forbid* refuses nothing of it.
#[test]
fn beside_an_hrule_the_remedy_names_the_hrule_and_not_the_list() {
    let scratch = ein_corpus::Scratch::new("ein-e47-remedy");
    for module in ["words", "gaps", "listed_is_all"] {
        std::fs::copy(
            repo_root().join(format!("examples/word_problems/{module}.ein")),
            scratch.path().join(format!("{module}.ein")),
        )
        .expect("a word-problem module");
    }
    let program = |listed: bool, list: bool| {
        format!(
            "(import words :symbols (words has asked together))\n\
             (import std.math.decimal :symbols (n))\n(import std.math.arith :symbols (z))\n\
             (import gaps :symbols (gaps))\n{}\
             (has Bob apples (n z 5) now)\n(has Alice pears (n z 3) now)\n\
             (asked (together Bob Alice apples now))\n\
             (hrule maybe-apples () :match (has ?p pears ?v ?t) :assert (has ?p apples (n z 1) ?t) \
             :why \"suppose {{?p}} has 1 apple as well\")\n\
             (solve :hrules (maybe-apples){})\n",
            if listed {
                "(import listed_is_all :symbols (listed-is-all))\n"
            } else {
                ""
            },
            if list { " :no-hypothesis (has)" } else { "" },
        )
    };
    let mut bad = Vec::new();
    for (listed, list, refused) in [
        (true, true, true),
        (true, false, true),
        (false, false, false),
    ] {
        let path = scratch.path().join(format!("w-{listed}-{list}.ein"));
        let text = program(listed, list);
        std::fs::write(&path, &text).expect("written");
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
        let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
        let out = explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(DESCRIBE));
        match (out, refused) {
            (Err(SolveError::Refused(m)), true) => {
                for line in m.lines() {
                    for want in [
                        "over has, a relation a commitment can still extend",
                        "let no hrule propose has, if no commitment is meant to extend it — \
                         hrule 'maybe-apples' proposes it, and no list filters an hrule's \
                         candidates",
                    ] {
                        if !line.contains(want) {
                            bad.push(format!(
                                "listed {listed}, list {list}: {line:?} lacks {want:?}"
                            ));
                        }
                    }
                    if line.contains(":no-hypothesis") {
                        bad.push(format!(
                            "listed {listed}, list {list}: names the list: {line:?}"
                        ));
                    }
                }
                if m.lines().count() != 4 {
                    bad.push(format!(
                        "listed {listed}, list {list}: {} lines:\n{m}",
                        m.lines().count()
                    ));
                }
            }
            (Ok(_), false) => {}
            (out, _) => bad.push(format!(
                "listed {listed}, list {list}: {:?}",
                out.map(|a| a.counts.map(|c| c.k_s))
            )),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **With no hrule, an obligation's relation is no list's to close either** —
/// M1f S1f.6.34. Where an obligation owes the relation the refutation reads
/// under `absent`, `(solve :no-hypothesis (R))` is a conflict (O + NH,
/// `defined_behaviour.md` § 4.27), so the first remedy names the obligation;
/// where none owes it, the list, as before.
#[test]
fn beside_an_obligation_the_remedy_names_the_obligation() {
    let program = |owed: bool| {
        let (import, activator) = if owed {
            (
                "(import std.algebra :symbols (total-owed))\n",
                "(total-owed likes is-a)\n",
            )
        } else {
            ("", "")
        };
        format!(
            "{import}(relation likes Person Food :why \"{{?1}} likes {{?2}}\")\n\
             (is-a Ann Person) (is-a Soup Food) (is-a Stew Food)\n{activator}\
             (rule hungry () :match (and (is-a ?p Person) (absent (likes ?p Soup))) \
             :assert (false) :why \"{{?p}} does not like Soup\")\n\
             (solve :no-hypothesis (is-a))\n"
        )
    };
    for (owed, want, not) in [
        (
            true,
            "let no obligation owe likes, if no commitment is meant to extend it — obligation \
             'total-owed' owes it, and no list may filter what an obligation owes",
            ":no-hypothesis",
        ),
        (
            false,
            "close the relation, (solve :no-hypothesis (likes))",
            "let no obligation owe",
        ),
    ] {
        let text = program(owed);
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, Some("<owed>")).expect("it parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("it loads");
        let Err(SolveError::Refused(m)) =
            explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(DESCRIBE))
        else {
            panic!("owed {owed}: not refused");
        };
        assert!(m.contains(want), "owed {owed}: {m}");
        assert!(!m.contains(not), "owed {owed}: names {not:?}: {m}");
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
