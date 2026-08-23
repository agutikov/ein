//! Checking a `:expect` against what the search actually answered.
//!
//! The shape is [`ein_ir::expect`]'s and the loader has already validated it;
//! this is the comparison. M1c
//! [S1c.1.2](../../../../plans/m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md)
//! T1c.1.2.4.
//!
//! # Relation-closure
//!
//! **Naming a relation asserts its complete extent.** If an expectation
//! mentions `pet-loc` at all, the `pet-loc` facts it lists are the model's
//! whole `pet-loc` extent — not a subset — and relations it never mentions are
//! unconstrained.
//!
//! That rule is the design, and it sits between two useless extremes. A
//! per-fact assertion cannot catch a **surplus** fact: the 23 spurious models
//! of `zebra2-minus-15` that
//! [M1c's thesis](../../../../plans/m1c_external_validation/README.md#the-thesis)
//! is written around were surplus — Chesterfields and the Fox in one house —
//! and a `:derives`-style check passes on every one of them. A whole-state
//! golden goes the other way and pins 250 facts of `is-a*` and activator noise
//! that no test means to assert. Closure is exact on what the test is about
//! and silent on the rest.
//!
//! Two consequences worth stating, because they are decisions and not
//! omissions:
//!
//! - **Stored negatives are not closed.** Closing `pet-loc` says nothing about
//!   the extent of `(not (pet-loc …))`. A `(not …)` listed in a model is
//!   checked for *presence*, like any other fact, so a test can pin one
//!   deliberately; what it cannot do is drag in the negative-completion rules'
//!   entire output, which on a Zebra puzzle is most of the model.
//! - **Facts compare by content**, as rendered s-expressions, never by
//!   `FactId` — `fork_audit`'s reason: two runs do not share an interner, and
//!   an expectation is written by a human in the first place.
//!
//! # The verdict is implied
//!
//! `(model …)` expects one model, `(or …)` expects that many, `(false)`
//! expects `Contradiction`. There is no separate `:verdict` or `:k` to
//! disagree with the models beside it, so "says `Solution` and lists two
//! models" is not a test one can write.
//!
//! The three are peers, because the three verdicts are: `k = 0`, `k = 1` and
//! `k > 1` are all answers in ein, read off the result rather than chosen by a
//! flag ([`01_grammar.md` § Query](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query)).
//! An expectation that could only state one of them would be a form for
//! *solvable* puzzles, which is a third of what the engine is for.
//!
//! # What this form can state and cannot verify
//!
//! `(or M₁ … M_k)` is two claims: every `Mᵢ` is a model — found by searching —
//! and there is no `M_{k+1}`, which is established only by **exhausting the
//! lattice**. [`Outcome::NotChecked`] is the second half made honest, not
//! solved: `zebra2-minus-15`'s 32 models are all found by depth 3 and depths 4
//! and 5 exist only to prove there are no more, so its answer can be written
//! here and verified on no machine. And a *puzzle* cannot state the claim at
//! all — `(or A B)` in a `:match` is a disjunction over premises, and nothing
//! in the rule language quantifies over models.
//!
//! That is [P1d.4](../../../../plans/m1d_satisfiability/p1d.4_model_set_closure/README.md)
//! / [Q-M1d.7](../../../../plans/m1d_satisfiability/open_questions.md#q-m1d7--may-a-program-require-its-own-model-count),
//! and it is deliberately not decided here.

use ein_core::{Kb, Symbol, Terms};
use ein_ir::Ast;
use ein_ir::expect::{Expectation, Model};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::events::sexpr;
use crate::verdict::{Answer, Solution, Verdict};

/// Three outcomes, not two.
///
/// **An expectation is a claim about the *exhausted* answer**, because the
/// verdict it names is: `Solution` means one model and no other, `(or …)` with
/// k disjuncts means k and no k+1-th, `(false)` means every branch died. A
/// search that stopped early establishes a *lower bound* on k, which confirms
/// none of those — so "the counts happen to agree" is not a pass, and calling
/// it one would be a green result for a claim nobody checked.
///
/// It only bites where more searching could have changed the answer.
/// `found > claimed` is a genuine failure whether or not the lattice was
/// exhausted, and so is a model that disagrees with the expectation it was
/// matched to: no amount of further search unfinds them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// The claim holds, and the search that established it was exhausted.
    Held,
    /// The claim is false, and more searching would not rescue it.
    Failed,
    /// The search stopped early; the claim was neither confirmed nor refuted.
    NotChecked,
}

/// What the check found. `lines` is empty exactly when [`Outcome::Held`].
#[derive(Debug)]
pub struct Report {
    pub outcome: Outcome,
    /// One line per disagreement, in the order they were found — the loader's
    /// convention, and what a person debugging a rule reads.
    pub lines: Vec<String>,
}

impl Report {
    /// Held — and *only* held. A caller that treats `NotChecked` as success is
    /// the failure mode this enum exists to prevent, so the question has one
    /// spelling.
    pub fn passed(&self) -> bool {
        self.outcome == Outcome::Held
    }

    fn ok() -> Self {
        Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        }
    }

    fn failed(lines: Vec<String>) -> Self {
        Report {
            outcome: Outcome::Failed,
            lines,
        }
    }

    fn unchecked(lines: Vec<String>) -> Self {
        Report {
            outcome: Outcome::NotChecked,
            lines,
        }
    }

    /// The verdict-shaped claims: held only if the search was exhausted.
    fn ok_if_exhausted(exhausted: bool, what: &str) -> Self {
        if exhausted {
            return Report::ok();
        }
        Report::unchecked(vec![format!(
            "{what} matches, but the search was not exhausted — k is a lower \
             bound, so nothing here is established. Pass --exhaustive."
        )])
    }
}

/// A model, indexed the two ways the comparison asks about it.
struct Actual {
    /// Every fact, rendered — what a listed `(not …)` is looked up in.
    all: FxHashSet<String>,
    /// Positive extent per relation name, rendered. `(not X)` facts are not
    /// here: they are not `not`'s extent in any sense a test means.
    by_relation: FxHashMap<String, FxHashSet<String>>,
}

impl Actual {
    fn of(terms: &Terms, kb: &Kb, not: Symbol) -> Self {
        let mut all = FxHashSet::default();
        let mut by_relation: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
        for f in kb.facts() {
            let rendered = sexpr(terms, f);
            let rel = terms.facts.rel(f);
            if rel != not {
                by_relation
                    .entry(terms.sym(rel).to_string())
                    .or_default()
                    .insert(rendered.clone());
            }
            all.insert(rendered);
        }
        Actual { all, by_relation }
    }
}

/// Check one query's expectation against the answer it got.
///
/// `exhausted` is the run's own `MonotonicStats::exhausted`. It is not part of
/// the comparison — an expectation whose models are exactly the ones found
/// holds either way — but it is what turns the *disagreement* into something
/// actionable: `k` from a stopped search is a lower bound, so "expected 2, got
/// 1" usually means `--exhaustive` was not passed rather than that the puzzle
/// is wrong.
pub fn check(
    ast: &Ast,
    terms: &Terms,
    expectation: &Expectation,
    answer: &Answer,
    exhausted: bool,
) -> Report {
    let verdict = match answer {
        Answer::Verdict(v) => v,
        Answer::Aborted { reason } => {
            return Report::failed(vec![format!(
                "expected {}, but the run did not finish ({reason})",
                expectation.verdict_name()
            )]);
        }
    };
    let want = expectation.verdict_name();
    let models: Vec<&Solution> = match verdict {
        Verdict::Contradiction { .. } => Vec::new(),
        Verdict::Solution(s) => vec![s],
        Verdict::Ambiguity(bs) => bs.iter().collect(),
    };
    if matches!(expectation, Expectation::Contradiction) {
        return if matches!(verdict, Verdict::Contradiction { .. }) {
            // A `Contradiction` from a stopped search is Q-M1d.6's open
            // question — ten corpus entries already say it — so this does not
            // take a position on it: it declines to call the claim checked.
            Report::ok_if_exhausted(exhausted, "Contradiction")
        } else {
            Report::failed(vec![format!(
                "expected (false) — Contradiction — got {} with {} model{}",
                verdict.as_str(),
                models.len(),
                if models.len() == 1 { "" } else { "s" }
            )])
        };
    }
    if matches!(verdict, Verdict::Contradiction { .. }) {
        return Report::failed(vec![format!(
            "expected {want}, got Contradiction — write `:expect (false)` if that is the answer"
        )]);
    }
    let _ = exhausted;

    // Models are compared as a **set**: the order a search happens to find
    // them in is exactly what S1a.7.0's invariance tests assert is not
    // observable, so a sequence comparison would pin something the engine
    // does not promise.
    let distinct = distinct_models(terms, &models);
    let wanted = expectation.models();
    if distinct.len() != wanted.len() {
        let mut lines = vec![format!(
            "expected {want} with k = {}, got {} with k = {}",
            wanted.len(),
            verdict.as_str(),
            distinct.len()
        )];
        // Too FEW models is the one shortfall a longer search could fix; too
        // many is a refutation whatever the search did next.
        if distinct.len() < wanted.len() && !exhausted {
            lines.push(
                "…and the search was not exhausted, so that k is a lower bound — \
                 pass --exhaustive (or -n) before believing it"
                    .into(),
            );
            return Report::unchecked(lines);
        }
        return Report::failed(lines);
    }

    let mut lines = Vec::new();
    if matching(ast, &wanted.iter().collect::<Vec<_>>(), &distinct, &mut lines) {
        Report::ok_if_exhausted(exhausted, "every listed model")
    } else {
        // A model that matches no expectation is a disagreement about content,
        // not about how far the search got.
        Report::failed(lines)
    }
}

/// The distinct models among the branches, keyed the way `answer.rs` counts
/// `k` — by canonical state, so two branches that reached the same model are
/// one model here too.
fn distinct_models(terms: &Terms, models: &[&Solution]) -> Vec<Actual> {
    let not = terms.kernel.not;
    let mut keys: Vec<Box<[ein_core::FactId]>> = Vec::new();
    let mut out = Vec::new();
    for s in models {
        let key = crate::canon::state_key(&s.kb);
        if keys.contains(&key) {
            continue;
        }
        keys.push(key);
        out.push(Actual::of(terms, &s.kb, not));
    }
    out
}

/// Is there a perfect matching between expectations and models?
///
/// Kuhn's augmenting-path algorithm, because greedy is wrong: two
/// expectations can each be satisfied by two models, and pairing them the
/// first way that fits can strand the third. The sets are the size of a `k`, so
/// the cubic bound is not a cost.
///
/// On failure `lines` gets the *first* expectation that matched nothing,
/// explained against the model closest to it — an unmatched expectation is
/// what a person needs to see, and the whole bipartite story is not.
fn matching(ast: &Ast, wanted: &[&Model], actual: &[Actual], lines: &mut Vec<String>) -> bool {
    let n = wanted.len();
    let fits: Vec<Vec<bool>> = wanted
        .iter()
        .map(|w| {
            actual
                .iter()
                .map(|a| explain(ast, w, a).is_empty())
                .collect()
        })
        .collect();
    let mut taken_by: Vec<Option<usize>> = vec![None; actual.len()];
    for i in 0..n {
        let mut seen = vec![false; actual.len()];
        if !augment(i, &fits, &mut taken_by, &mut seen) {
            // Report against whichever model this expectation is closest to,
            // which is the one a reader will have been looking at.
            let best = (0..actual.len())
                .min_by_key(|&j| explain(ast, wanted[i], &actual[j]).len())
                .unwrap_or(0);
            let which = if n == 1 {
                String::new()
            } else {
                format!("expectation {} of {n}: ", i + 1)
            };
            for line in explain(ast, wanted[i], &actual[best]) {
                lines.push(format!("{which}{line}"));
            }
            if lines.is_empty() {
                // Every model this expectation fits was needed by another.
                lines.push(format!(
                    "{which}matches a model that another expectation also claims — \
                     the {n} expectations are not distinct"
                ));
            }
            return false;
        }
    }
    true
}

fn augment(i: usize, fits: &[Vec<bool>], taken_by: &mut [Option<usize>], seen: &mut [bool]) -> bool {
    for j in 0..taken_by.len() {
        if !fits[i][j] || seen[j] {
            continue;
        }
        seen[j] = true;
        let free = match taken_by[j] {
            None => true,
            Some(other) => augment(other, fits, taken_by, seen),
        };
        if free {
            taken_by[j] = Some(i);
            return true;
        }
    }
    false
}

/// Why this model does not satisfy this expectation; empty when it does.
///
/// Two checks, and only the first is closure: every relation the expectation
/// names positively must have *exactly* the listed extent, and every listed
/// `(not …)` must be present.
fn explain(ast: &Ast, want: &Model, actual: &Actual) -> Vec<String> {
    let mut listed: FxHashMap<&str, FxHashSet<String>> = FxHashMap::default();
    let mut negatives: Vec<String> = Vec::new();
    for &node in &want.facts {
        // The loader rejected anything `expect::fact` refuses, so a failure
        // here cannot come from a program.
        let Ok(f) = ein_ir::expect::fact(ast, node) else {
            continue;
        };
        if f.negated {
            negatives.push(f.rendered);
        } else {
            listed.entry(f.relation).or_default().insert(f.rendered);
        }
    }
    let mut lines = Vec::new();
    // determinism-ok: the relations are re-sorted here precisely so the
    // report does not inherit a hash map's order.
    let mut relations: Vec<&str> = listed.keys().copied().collect();
    relations.sort_unstable();
    for rel in relations {
        let want_set = &listed[rel];
        let empty = FxHashSet::default();
        let got = actual.by_relation.get(rel).unwrap_or(&empty);
        let mut missing: Vec<&String> = want_set.difference(got).collect();
        let mut surplus: Vec<&String> = got.difference(want_set).collect();
        missing.sort();
        surplus.sort();
        for f in missing {
            lines.push(format!("{rel}: expected {f}, and the model has no such fact"));
        }
        for f in surplus {
            lines.push(format!(
                "{rel}: the model also has {f}, which the expectation does not list \
                 (naming a relation closes it)"
            ));
        }
    }
    negatives.sort();
    for f in &negatives {
        if !actual.all.contains(f) {
            lines.push(format!("expected {f}, and the model does not carry it"));
        }
    }
    lines
}
