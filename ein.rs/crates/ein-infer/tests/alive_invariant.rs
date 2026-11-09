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
//! S1e2.2.4 seeds the universe with it. S1e2.2.14 widened the seed to every
//! name of the program, on the user's ruling of 2026-11-04 on Q-M1e2.9 (a): a
//! constant in a rule's `:match`, and one a fact names only inside a nested
//! fact, as well (M1e2's W15). So the three readings S1e.3.3 named read today:
//!
//! | | claim | corpus |
//! |---|---|---|
//! | **R0** | every name of the program is among the blind enumerator's objects, the ones the names index does not hold included | **37** names on **16** programs the index does not hold (S1e2.2.4: 3 on 3), and each is a candidate object |
//! | **R1** | nothing a run derives names what the loaded program, its rules' constants included, did not | **0** |
//! | **R2** | nothing the **search** reaches names what **root's fixpoint** and the rules' constants did not | **0** |
//!
//! Until S1e2.2.4, R1 found `Ann` and `G` and R2 found `Ann`, because the
//! baseline was the facts'. A name R1 or R2 found now would be one no rule's
//! text predicts — the residue `interning.rs` named, a relation head written
//! as a parameter and bound to an integer, until M1f S1f.5.4 retired the int
//! kind (a run of digits is a name since S1f.5.5's swap) — and the test that
//! sweeps for it is [`no_run_reaches_a_name_the_program_does_not_have`].

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

/// **Every name of the program the names index does not hold, corpus-wide**,
/// at load, as `(path, [rendered…])` — what [`invariant::seeded`] reports.
///
/// Until M1e2 S1e2.2.4 these were the corpus's breaches; S1e2.2.4 made a
/// rule's `:assert` constant a name of the program (`Ann`, `G`, `Z`), and
/// S1e2.2.14, on the user's ruling of 2026-11-04 on Q-M1e2.9 (a), a constant
/// in a rule's `:match` (`eq_arity_high`'s `A` and `B`, `match-only`'s `Z`)
/// and one a fact names only inside a nested fact (the rest). Each is a
/// candidate object.
///
/// - `mixed-type-hypothesis.ein` — `Ann`, in an `hrule`'s `:assert`. That
///   query drives the hrule rung, whose candidates already named `Ann`; the
///   seed reaches its blind probe (`EIN_LEFTOVER`) alone.
/// - `07_schroder.ein` — `G`, in the probe rule `probe-undecided`, which fires
///   in **root** saturation, so `G` was a name at root before the seed too;
///   and `C`, `D`, inside its stored negatives.
/// - `wrong-answers/fresh-name-default.ein` — `Z`, M1e2's W4 in isolation.
/// - `wrong-answers/match-only.ein` and `negative-only.ein` — `Z`, M1e2's W15:
///   before S1e2.2.14 no candidate named it, and R was {∅} where the
///   `-declared` controls' four states are.
/// - `wrong-answers/oblig-fresh-name.ein`, `-two.ein` and
///   `oblig-stdlib-total-owed.ein` — `Z`, in `special`'s `:assert`, M1e2's
///   W14 (S1e2.2.15): an object of the blind enumerator, which the obligations
///   rung those programs take does not consult.
/// - the three `*-disproved`, `syntax/equality` and `syntax/arg-kinds` — the
///   corpus programs whose H that moved (W15's rows; `arg-kinds`' `a` and `b`
///   inside `(r (nested a b) c)`, its solve scoped since M1f S1f.5.5's swap,
///   R {∅}).
/// - since M1f S1f.5.5's swap, a run of digits a name (S1f.5.4's ruling):
///   `stdlib/elim.ein`'s `0` and `1`, `:match` constants of
///   `domain-elimination` that were ints until then; `syntax/relation-decls`'
///   `7`, inside its `ontology` fact; and `wrong-answers/range-in-match.ein`'s
///   `1..3`, a `:match` constant the parser refused from S1f.5.3 to the swap.
/// - `ein-bugs/binding-key-{both,swapped,nested-only}.ein` — `Y`, inside an
///   activator's nested `(src Y)`, M1e2's W7 family (S1e2.2.7): H is
///   `(holds Y)`, as W15 made it on the pair, which S1e2.2.7 promoted.
/// - `wrong-answers/or-one-key-twin.ein` — `w`, in `m1`'s and `s`'s
///   `:assert`, M1f's W39 (S1f.5.1's T1f.5.1.3): the witness beside it names a
///   rule `w`, which makes `w` a name of the program there, and the twin,
///   whose rules are `w1` and `w2`, names it in no fact.
/// - `ein-bugs/activator-all-nested.ein` — `a`, inside `(item (box a))`,
///   M1e2's W16 (S1e2.2.16): its one candidate is the hrule's `(checked
///   (box a))`, and the name is no candidate's.
/// - the rest — nested names on programs with no relation the blind rung
///   proposes over, so H stays empty; `22_symmetric_negative`'s `C` is a
///   name at root's fixpoint too, `(undecided B C)`.
///
/// Nine of M1e2's W22 witnesses (`wrong-answers/kw-*.ein`) stood here from
/// T1e2.2.22.1 until the stage's fix the same day (2026-11-05): `w` or `z`,
/// the rest of a name the parser split after a keyword, `(neqw ?x)` read as
/// `(neq w ?x)`, which made `w` a name of the program. A keyword is a whole
/// word since, and `neqw` the name it spells.
///
/// Two of M1e2's W24 witnesses stood here from T1e2.2.24.1 until the stage's
/// fix the same day (2026-11-05): `wrong-answers/bare-premise.ein`'s `z`, a
/// rule's `:assert` constant, and `nested-unbound-head.ein`'s `b` and `z`. The
/// loader refuses both programs since, so neither loads here.
///
/// Four of the witnesses M1f S1f.5.0 banked on 2026-11-08 stand here, each a
/// name only a rule writes: W30's `assert-neq-w-x.ein` and
/// `assert-eq-x-w.ein`, `w` in a concluded `neq` or `eq`, which makes `(p w)`
/// a candidate; and W31's `empty-match-fact.ein` and `empty-match-hrule.ein`,
/// `b` in a conclusion no firing reaches. Each owning stage says whether its
/// fix moves its line.
///
/// Two of the witnesses M1f S1f.5.7 banked on 2026-11-09 (T1f.5.7.1) stand
/// here: W52's `relvar-eq-assert.ein`, `w` in a conclusion whose relation a
/// parameter binds to `eq`, as W30's `assert-eq-x-w.ein`'s; and W54's
/// `goal-nested-var.ein`, `a` and `b` inside its stored negations.
///
/// **Left at M1f S1f.5.5's swap (T1f.5.5.4)**, each refused at the parse since
/// and so loading here no more: `broken/compile/eq_arity_high.ein`'s `A` and
/// `B` (moved to `examples/broken/`, `eq` taking two terms), the moved
/// `kwpair-below-the-filter.ein`'s four step names (S1f.5.11), W18's
/// `str-in-assert.ein` and `str-in-hrule.ein`'s `lbl` and W28's
/// `classes-int-name.ein`'s `"1"` (strings, S1f.5.2), W32's
/// `kwpair-in-pattern.ein` and `kwpair-in-assert.ein`'s `v` (S1f.5.11), and
/// W29's `underscore-nested-fact-arg.ein`'s `a` (S1f.5.1).
const SEEDED: [(&str, &[&str]); 30] = [
    (
        "examples/ein-bugs/activator-all-nested.ein",
        &["`a`, in (item (box a))"],
    ),
    (
        "examples/ein-bugs/binding-key-both.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/binding-key-nested-only.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/binding-key-swapped.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/mixed-type-hypothesis.ein",
        &["`Ann`, asserted by hrule `guess`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/assert-eq-x-w.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/assert-neq-w-x.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/empty-match-fact.ein",
        &["`b`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/empty-match-hrule.ein",
        &["`b`, asserted by hrule `h`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/fresh-name-default.ein",
        &["`Z`, asserted by rule `spawn`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/goal-nested-var.ein",
        &["`a`, in (not (p a))", "`b`, in (not (p b))"],
    ),
    (
        "examples/ein-bugs/wrong-answers/match-only.ein",
        &["`Z`, matched by rule `reach-z`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/negative-only.ein",
        &["`Z`, in (not (q Z A))"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-fresh-name.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/or-one-key-twin.ein",
        &["`w`, asserted by rule `m1`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/range-in-match.ein",
        &["`1..3`, matched by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/relvar-eq-assert.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/saturation/hypothesis-contradiction/coloc-disproved.ein",
        &[
            "`House-2`, in (hypothesis (co-located Norwegian House-2))",
            "`Norwegian`, in (hypothesis (co-located Norwegian House-2))",
        ],
    ),
    (
        "examples/saturation/hypothesis-contradiction/next-to-disproved.ein",
        &[
            "`X`, in (hypothesis (next-to X Y))",
            "`Y`, in (hypothesis (next-to X Y))",
        ],
    ),
    (
        "examples/saturation/hypothesis-contradiction/right-of-disproved.ein",
        &[
            "`A`, in (hypothesis (right-of A B))",
            "`B`, in (hypothesis (right-of A B))",
        ],
    ),
    (
        "examples/syntax/arg-kinds.ein",
        &["`a`, in (r (nested a b) c)", "`b`, in (r (nested a b) c)"],
    ),
    (
        "examples/syntax/equality.ein",
        &["`House-1`, in (= (color House-1) Red)"],
    ),
    (
        "examples/syntax/negation.ein",
        &["`a`, in (not (r a b))", "`b`, in (not (r a b))"],
    ),
    (
        "examples/syntax/relation-decls.ein",
        &[
            "`7`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R2`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R3`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R4`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`T`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`U`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`V`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
        ],
    ),
    (
        "examples/syntax/wrappers.ein",
        &[
            "`T`, in (ontology (relation r T T) (is-a a T))",
            "`a`, in (ontology (relation r T T) (is-a a T))",
            "`b`, in (facts (r a b))",
            "`c`, in (reasoning (r b c))",
            "`r`, in (ontology (relation r T T) (is-a a T))",
        ],
    ),
    (
        "stdlib/elim.ein",
        &[
            "`0`, matched by rule `domain-elimination`",
            "`1`, matched by rule `domain-elimination`",
        ],
    ),
    (
        "tests/stdlib/algebra/07_schroder.ein",
        &[
            "`C`, in (not (t A C))",
            "`D`, in (not (t D F))",
            "`G`, matched by rule `probe-undecided`",
        ],
    ),
    (
        "tests/stdlib/algebra/22_symmetric_negative.ein",
        &["`C`, in (not (sits-with A C))"],
    ),
];

/// The pair S1e.3.3 added, which is in `SEEDED`'s shape and not in `SEEDED`:
/// it is checked by [`the_pair_answers_one_region`] instead, where the
/// answers are the claim.
const PAIR: [&str; 2] = [
    "examples/ein-bugs/alive-set-fresh-name.ein",
    "examples/ein-bugs/alive-set-fresh-name-declared.ein",
];

/// **R0 — every name of the program is a candidate object, over every corpus
/// program.**
///
/// Free: it reads the loaded facts at every depth and the registered rules'
/// `:match` and `:assert` constants once, at load, and never runs the search.
/// That is what makes it a statement about the *program*: a rule that would
/// name a constant only on an input nobody has written is seeded all the
/// same. What it lists is what the list adds over the names index; every name
/// it lists is an object here, so every one is held to the candidates.
#[test]
fn every_name_of_the_program_is_a_candidate_object() {
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
        .flat_map(|(p, names)| names.iter().map(|n| (p.to_string(), n.to_string())))
        .collect();
    want.sort();
    assert_eq!(
        found, want,
        "the corpus programs with a name the names index does not hold moved. \
         A new one is a program the seed reaches; a missing one means a fact now \
         names it at top level, and this list has to say so."
    );
}

/// **The breach a register row holds today** (M1f S1f.5.0, 2026-11-08):
/// W34, `:assert ()` stored as a fact whose head is the parser's synthetic
/// `#empty` (`@empty` until M1f T1f.5.5.8), a name no fact or rule writes,
/// which then joins H as an object.
/// It is the row's own wrong answer (`W34/assert-empty`), so it is named
/// rather than failed, and held both ways: S1f.5.9 empties the list in the
/// commit that fixes W34.
const KNOWN_R1: [(&str, &str); 2] = [
    (
        "examples/ein-bugs/wrong-answers/assert-empty.ein",
        "new-object: `\"#empty\"` in (q \"#empty\")",
    ),
    (
        "examples/ein-bugs/wrong-answers/assert-empty.ein",
        "new-relation: `\"#empty\"` in ()",
    ),
];

/// **R1 and R2 — no run reaches a name the program does not have.**
///
/// The dynamic half exists to confirm what the static half assumes: a
/// *variable* leaf can only carry a value the KB already holds, so nothing but
/// a rule's constant can introduce a name — and a rule's constant is a name of
/// the program. Measured over every state a bounded describe meets on the
/// corpus, the scan finds **nothing**, against either baseline, but for the
/// one breach a register row holds ([`KNOWN_R1`], W34's). Until M1e2
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
    r1.sort();
    let mut known: Vec<(String, String)> = KNOWN_R1
        .iter()
        .map(|(p, b)| (p.to_string(), b.to_string()))
        .collect();
    known.sort();
    assert_eq!(
        r1, known,
        "R1 — a run derived a name that is neither a loaded fact's nor a rule's \
         constant: a value no rule's text predicts, which the seed cannot hold \
         (KNOWN_R1 names W34's, open, which S1f.5.9 fixes)"
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
