//! M1e S1e.3.3 — **the M1 alive-set invariant, evaluated over the corpus.**
//!
//! [`ein_infer::invariant`] is the check; this is what it says, banked. The
//! finding it closes is `ST-M1`: the invariant licenses the per-KB `alive`
//! recompute, the `state_key` dedup that produces `k`, and — since M1d — the
//! tree's exhaustiveness-by-discharge argument, and until S1e.3.3 nothing
//! evaluated it.
//!
//! # Since M1e2 S1e2.2.4: the seed
//!
//! S1e.3.3 read clause 1, *rules assert no new objects*, against the names the
//! loaded **facts** mention, and found three programs that break it and a
//! fourth, `examples/ein-bugs/alive-set-fresh-name.ein`, on which breaking it
//! cost a model (M1e2's W4): the blind enumerator built its objects from the
//! KB's names, so a constant only a rule names entered no candidate. The user
//! ruled on 2026-10-16 that such a constant is a name of the program (Q-M1e.21
//! (b): *"rules are part of root, so rules can't assert new names"*), and
//! S1e2.2.4 seeds the universe with it. So the three readings S1e.3.3 named
//! read today:
//!
//! | | claim | corpus |
//! |---|---|---|
//! | **R0** | the constants **only a rule** mentions are seeded into the blind enumerator's objects | **3** programs have one (4 with the pair's twin), and each is seeded |
//! | **R1** | nothing a run derives names what the loaded program, its rules' constants included, did not | **0** |
//! | **R2** | nothing the **search** reaches names what **root's fixpoint** and the rules' constants did not | **0** |
//!
//! Until S1e2.2.4, R1 found `Ann` and `G` and R2 found `Ann`, because the
//! baseline was the facts'. A name R1 or R2 found now would be one no rule's
//! text predicts — the residue `interning.rs` names, a relation head written as
//! a parameter and bound to an integer — and the test that sweeps for it is
//! [`no_run_reaches_a_name_the_program_does_not_have`].

use ein_core::{FactId, Kb, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::invariant::{self, Universe};
use ein_infer::region::Nodes;
use ein_ir::{Ast, load_file};

/// `ein solve FILE`, bounded for a sweep: a describe that stops at a small
/// budget, and a flagged program's cube only where it is small, lists the
/// states it met — what this file means by *what a run reaches*.
fn describe_options() -> explore::Options {
    let mut o = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    o.budget = 20_000;
    o.limit = 10;
    o
}

/// Every state the answer met and every owing frontier node, as the facts
/// that reach it.
fn reached(a: &explore::Answer) -> Vec<Vec<FactId>> {
    let Some(d) = a.described.as_deref() else {
        return Vec::new();
    };
    let facts = |set: &[u32]| -> Vec<FactId> {
        set.iter().map(|&b| a.candidates[b as usize].fact).collect()
    };
    d.states
        .iter()
        .map(|s| facts(&s.rep))
        .chain(d.frontier.iter().map(|o| facts(&o.set)))
        .collect()
}

/// **Every constant only a rule mentions, corpus-wide**, as `(path, rendered)`.
///
/// Until M1e2 S1e2.2.4 this was the corpus's breach set; it is the same three
/// names, and each is now a name of the program the universe holds.
///
/// - `mixed-type-hypothesis.ein` — `Ann`, in an `hrule`'s `:assert`. That
///   query drives the hrule rung, whose candidates already named `Ann`; the
///   seed reaches its blind probe (`EIN_LEFTOVER`) alone.
/// - `07_schroder.ein` — `G`, in the probe rule `probe-undecided`'s `:assert`.
///   It fires in **root** saturation, so `G` was a name at root before the
///   seed too.
/// - `wrong-answers/fresh-name-default.ein` — `Z`, M1e2's W4 in isolation
///   (`corpus/wrong_answers.toml`, `W4/fresh-name-default`): `spawn` fires
///   only on a committed `q`, so before the seed no candidate named `Z` and R
///   was {∅} where `{(q A Z), (q B Z)}` is the maximal state.
const SEEDED: [(&str, &str); 3] = [
    (
        "examples/ein-bugs/mixed-type-hypothesis.ein",
        "`Ann`, asserted by hrule `guess`",
    ),
    (
        "examples/ein-bugs/wrong-answers/fresh-name-default.ein",
        "`Z`, asserted by rule `spawn`",
    ),
    (
        "tests/stdlib/algebra/07_schroder.ein",
        "`G`, asserted by rule `probe-undecided`",
    ),
];

/// The pair S1e.3.3 added, which is in `SEEDED`'s shape and not in `SEEDED`:
/// it is checked by [`the_pair_answers_one_region`] instead, where the
/// answers are the claim.
const PAIR: [&str; 2] = [
    "examples/ein-bugs/alive-set-fresh-name.ein",
    "examples/ein-bugs/alive-set-fresh-name-declared.ein",
];

/// **R0 — every constant only a rule mentions is seeded, over every corpus
/// program.**
///
/// Free: it reads the registered rules' `:assert` constants once, at load, and
/// never runs the search. That is what makes it a statement about the
/// *program*: a rule that would name a constant only on an input nobody has
/// written is seeded all the same.
#[test]
fn the_names_only_a_rule_mentions_are_seeded() {
    let mut found: Vec<(String, String)> = Vec::new();
    let mut programs = 0usize;
    for path in &corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        programs += 1;
        if PAIR.contains(&rel.as_str()) {
            continue;
        }
        let objects = ein_infer::candidate_objects(&kb, &terms, &ast);
        for c in invariant::seeded(&kb, &terms, &ast) {
            assert!(
                objects.contains(&c.name),
                "{rel}: {} is a name of the program and not among the blind \
                 enumerator's objects {:?} — the seed is gone",
                c.render(&terms),
                objects.iter().map(|&o| terms.sym(o)).collect::<Vec<_>>()
            );
            found.push((rel.clone(), c.render(&terms)));
        }
    }
    assert!(
        programs >= 160,
        "only {programs} corpus files loaded — the sweep stopped looking"
    );
    found.sort();
    let mut want: Vec<(String, String)> = SEEDED
        .iter()
        .map(|(p, b)| (p.to_string(), b.to_string()))
        .collect();
    want.sort();
    assert_eq!(
        found, want,
        "the corpus programs with a constant only a rule mentions moved. A new \
         one is a program the seed reaches; a missing one means a fact now \
         names it, and this list has to say so."
    );
}

/// **R1 and R2 — no run reaches a name the program does not have.**
///
/// The dynamic half exists to confirm what the static half assumes: a
/// *variable* leaf can only carry a value the KB already holds, so nothing but
/// a rule's constant can introduce a name — and a rule's constant is a name of
/// the program. Measured over every state a bounded describe meets on the
/// corpus, the scan finds **nothing**, against either baseline. Until M1e2
/// S1e2.2.4 the baselines left the rules' constants out, and R1 found `Ann`
/// and `G`, R2 `Ann`.
#[test]
fn no_run_reaches_a_name_the_program_does_not_have() {
    let mut r1: Vec<(String, String)> = Vec::new();
    let mut r2: Vec<(String, String)> = Vec::new();
    let mut solved = 0usize;
    for path in &corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(mut kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        let loaded = Universe::of(&kb, &terms, &ast);
        let at_load = kb.n_facts();
        // The route saturates root in place, once, and writes nothing else
        // into it: what root holds after the run is root's fixpoint.
        let Ok(answer) = explore::run(&mut kb, &mut terms, &ast, &describe_options()) else {
            continue;
        };
        let at_root = kb.n_facts();
        let root_universe = Universe::of(&kb, &terms, &ast);

        let collect = |u: &Universe,
                       k: &ein_core::Kb,
                       terms: &Terms,
                       from: usize,
                       into: &mut Vec<(String, String)>| {
            let mut b = Vec::new();
            u.breaches(k, terms, from, &mut b);
            for x in b {
                let row = (rel.clone(), x.render(terms));
                if !into.contains(&row) {
                    into.push(row);
                }
            }
        };
        collect(&loaded, &kb, &terms, at_load, &mut r1);

        solved += 1;
        let sets = reached(&answer);
        let Ok(mut nodes) = Nodes::new(&mut kb, &mut terms, &ast, &answer.candidates) else {
            continue;
        };
        nodes
            .with_fixpoints(&sets, |kbs: &mut [Kb], terms: &mut Terms| {
                for k in kbs.iter() {
                    collect(&loaded, k, terms, at_load, &mut r1);
                    collect(&root_universe, k, terms, at_root, &mut r2);
                }
            })
            .unwrap_or_else(|e| panic!("{rel}: {e}"));
    }
    assert!(
        solved >= 155,
        "only {solved} corpus files reached a solve — the sweep stopped looking"
    );
    assert_eq!(
        r1,
        Vec::<(String, String)>::new(),
        "R1 — a run derived a name that is neither a loaded fact's nor a rule's \
         constant: a value no rule's text predicts, which the seed cannot hold"
    );
    assert_eq!(
        r2,
        Vec::<(String, String)>::new(),
        "R2 — the search reached a name root's fixpoint and the rules' constants \
         do not hold, so the candidates taken at root cannot be about it"
    );
}

/// **The pair answers one region.**
///
/// The two files differ by the single fact `(seen Z)`, over a relation nothing
/// else mentions. It changes no rule, no constraint and no goal: all it does
/// is put `Z` in a fact.
///
/// Until M1e2 S1e2.2.4 that one fact was the difference between two answers:
/// the twin's R was `{∅}`, because a rule's constant was in no candidate (W4,
/// `corpus/wrong_answers.toml`), and the control's maximal state was
/// `{(q A Z), (q B Z)}`. Since the seed, `Z` is a name of both programs, the
/// fact names it in one of them, and the two describe the same R: four
/// states, that one the maximal.
#[test]
fn the_pair_answers_one_region() {
    let run = |rel: &str, which: Which| {
        let path = repo_root().join(rel);
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb =
            load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        let seeded: Vec<String> = invariant::seeded(&kb, &terms, &ast)
            .iter()
            .map(|c| c.render(&terms))
            .collect();
        let a = explore::run(
            &mut kb,
            &mut terms,
            &ast,
            &explore::Options::new(Mode {
                which,
                ask: Ask::Describe,
            }),
        )
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(a.exhausted, "{rel}: the answer is not known whole");
        let mut states: Vec<Vec<String>> = a
            .states
            .iter()
            .map(|s| {
                let mut v: Vec<String> = s
                    .closure
                    .iter()
                    .map(|&b| a.candidates[b as usize].text.clone())
                    .filter(|t| t.starts_with("(q "))
                    .collect();
                v.sort();
                v
            })
            .collect();
        states.sort();
        (seeded, states)
    };

    let (twin_seeded, twin_line) = run(PAIR[0], Which::Maximal);
    let (control_seeded, control_line) = run(PAIR[1], Which::Maximal);

    // The same rule, the same constant, and only the twin's needs the seed.
    assert_eq!(
        twin_seeded,
        vec!["`Z`, asserted by rule `spawn`".to_string()],
        "the fixture stopped naming Z in its rule alone"
    );
    assert!(
        control_seeded.is_empty(),
        "the control names Z in `(seen Z)`, so its rule's constant is a fact's \
         name too and nothing is seeded. It reported {control_seeded:?}"
    );

    let maximal = vec![vec!["(q A Z)".to_string(), "(q B Z)".to_string()]];
    assert_eq!(
        twin_line, maximal,
        "the twin's maximal line is the state naming Z: the seed is what puts \
         its two candidates in H"
    );
    assert_eq!(control_line, maximal, "the control's maximal state moved");

    // All of R, not only the line: one fact names Z, and nothing else differs.
    let (_, twin_all) = run(PAIR[0], Which::All);
    let (_, control_all) = run(PAIR[1], Which::All);
    assert_eq!(
        twin_all.len(),
        4,
        "the twin's R: ∅, the two singletons, both"
    );
    assert_eq!(twin_all, control_all, "the pair describes two regions");
}
