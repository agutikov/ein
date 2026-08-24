//! Checking a `:expect` against what the search actually answered.
//!
//! The shape is [`ein_ir::expect`]'s and the loader has already validated it;
//! this is the comparison. M1c
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
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
//! [M1c's thesis](../../../../docs/history/m1c_external_validation/README.md#the-thesis)
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
//!
//! # Who calls this
//!
//! Two commands, and the difference between them is exactly `exhausted`.
//! `ein solve` checks a query's `:expect` because ignoring one would be worse
//! than not having the keyword, and it stops at `-n 1` by default, so a
//! verdict-shaped claim there routinely comes back [`Outcome::NotChecked`].
//! `ein test` (M1c
//! [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test))
//! exhausts, has no flag not to, and never solves a query that carries no
//! `:expect` — so under it the only thing left that can truncate a run is the
//! lattice depth cap.

use ein_core::{FactId, Kb, ProvKind, Symbol, Terms};
use ein_ir::Ast;
use ein_ir::expect::{Expectation, Model};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::events::{sexpr, sexpr_value};
use crate::verdict::{Answer, Solution, Verdict, goal_bindings};

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
             bound, so nothing here is established. Either the run stopped at \
             -n, or the frontier is still alive at --max-set-size."
        )])
    }
}

/// A model, indexed the two ways the comparison asks about it.
struct Actual<'a> {
    /// The model itself, kept for the question a **surplus** fact raises next:
    /// not *that* it is there but *why*. M1c
    /// [T1c.1.3.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test).
    kb: &'a Kb,
    /// Every fact, rendered — what a listed `(not …)` is looked up in.
    all: FxHashSet<String>,
    /// Positive extent per relation name: rendering → the fact it renders.
    /// `(not X)` facts are not here — they are not `not`'s extent in any sense
    /// a test means. The `FactId` is the provenance handle and nothing else:
    /// the comparison itself is on the rendering, because two runs do not
    /// share an interner.
    by_relation: FxHashMap<String, FxHashMap<String, FactId>>,
}

impl<'a> Actual<'a> {
    fn of(terms: &Terms, kb: &'a Kb, not: Symbol) -> Self {
        let mut all = FxHashSet::default();
        let mut by_relation: FxHashMap<String, FxHashMap<String, FactId>> = FxHashMap::default();
        for f in kb.facts() {
            let rendered = sexpr(terms, f);
            let rel = terms.facts.rel(f);
            if rel != not {
                by_relation
                    .entry(terms.sym(rel).to_string())
                    .or_default()
                    .insert(rendered.clone(), f);
            }
            all.insert(rendered);
        }
        Actual {
            kb,
            all,
            by_relation,
        }
    }
}

/// Why this fact is in the model — one line, because "and where did *that*
/// come from" is `--trace`'s question and not this report's.
///
/// A surplus fact is the case [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
/// built relation-closure for, and the *next* thing its reader wants is the
/// rule that put it there — the `disjunctive-prune` guard bug this milestone
/// is written around was found exactly one step past "there is an extra fact
/// here". One level of premises, not a walk: the primary justification is what
/// the engine chose, and a reader who needs the rest reaches for `--trace`.
fn provenance(terms: &Terms, kb: &Kb, id: FactId) -> Option<String> {
    let prov = terms.provs.get(kb.primary(id)?);
    match prov.kind {
        ProvKind::Rule => {
            let rule = terms.sym(prov.rule?);
            if prov.premises.is_empty() {
                // A rule record with no premises is a synthetic engine
                // writeback, whose contract is that walks stop on it.
                return Some(format!("written back by {rule}"));
            }
            let premises: Vec<String> = prov.premises.iter().map(|&p| sexpr(terms, p)).collect();
            Some(format!("derived by {rule} from {}", premises.join(" ")))
        }
        ProvKind::Source => Some(match prov.source {
            Some(s) => format!("given by {}", terms.sym(s)),
            None => "in the program's own text".to_string(),
        }),
        ProvKind::Hypothesis => Some(match prov.branch {
            Some(b) => format!("hypothesised in branch {b}"),
            None => "hypothesised".to_string(),
        }),
        // Not reachable from a model — a rejected fact is not in one — and
        // named rather than swallowed, so it reads as a bug if it ever is.
        ProvKind::Rejected => Some("recorded as rejected".to_string()),
    }
}

/// Check one query's expectation against the answer it got.
///
/// `exhausted` is the run's own `MonotonicStats::exhausted`, and it is not part
/// of the comparison — an expectation whose models are exactly the ones found
/// matches either way. What it decides is whether a match is a **verdict**:
/// `k` from a stopped search is a lower bound, so "expected 2, got 1" is
/// usually a run that stopped at `-n` or a frontier still alive at
/// `--max-set-size`, rather than a puzzle that is wrong.
pub fn check(
    ast: &Ast,
    terms: &mut Terms,
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
        // A `k = 0` from a **truncated** search is "no model within the cap",
        // not "proven unsat" — [`MonotonicStats::exhausted`]'s own words — so
        // it is the rescuable shortfall, the same one the `distinct.len() <
        // wanted.len()` arm below reports, arriving as a different verdict
        // because zero models is a verdict of its own. Calling it a failure
        // would refute a claim on the strength of a search that stopped.
        //
        // Found by M1c [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test),
        // where exhausting is the default and `--max-set-size` is therefore
        // the only thing left that can truncate one.
        if !exhausted {
            return Report::unchecked(vec![
                format!(
                    "expected {want}, got Contradiction — but the search was not \
                     exhausted, so k = 0 means \"no model within the cap\" and not \
                     \"no model\""
                ),
                "raise --max-set-size, or write `:expect (false)` if ⊥ really is \
                 the answer"
                    .into(),
            ]);
        }
        return Report::failed(vec![format!(
            "expected {want}, got Contradiction — write `:expect (false)` if that is the answer"
        )]);
    }

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
        // A count is not actionable on its own — "you said one model and I
        // found two" leaves the reader to go and run `solve -e` to see what
        // the second one was. So each model is projected through the query's
        // own `:goal`, which is the question the file asked and the smallest
        // rendering of a model that answers it.
        for (i, m) in distinct.iter().enumerate() {
            lines.push(format!(
                "  model {} of {}: {}",
                i + 1,
                distinct.len(),
                goal_row(ast, terms, m.kb)
            ));
        }
        // Too FEW models is the one shortfall a longer search could fix; too
        // many is a refutation whatever the search did next.
        if distinct.len() < wanted.len() && !exhausted {
            lines.push(
                "…and the search was not exhausted, so that k is a lower bound — \
                 exhaust it (`solve -e`; `test` always does) and raise \
                 --max-set-size if the frontier is capped"
                    .into(),
            );
            return Report::unchecked(lines);
        }
        return Report::failed(lines);
    }

    let mut lines = Vec::new();
    if matching(
        ast,
        &*terms,
        &wanted.iter().collect::<Vec<_>>(),
        &distinct,
        &mut lines,
    ) {
        Report::ok_if_exhausted(exhausted, "every listed model")
    } else {
        // A model that matches no expectation is a disagreement about content,
        // not about how far the search got.
        Report::failed(lines)
    }
}

/// One model as the answer to the query's own `:goal`, sorted.
///
/// **Sorted** because the row order is not observable: `defined_behaviour.md`
/// §6 files the goal row a solve *table* prints as under-determined — it moves
/// under a permuted id space — and a failure report that inherited that would
/// be a diagnostic nobody could diff. Keys inside a row are sorted for the same
/// reason, which is `summary.json`'s rule too.
fn goal_row(ast: &Ast, terms: &mut Terms, kb: &Kb) -> String {
    let rows = goal_bindings(ast, terms, kb, None);
    if rows.is_empty() {
        return "the :goal matches nothing in it".to_string();
    }
    let mut shown: Vec<String> = rows
        .iter()
        .map(|row| {
            let mut cells: Vec<String> = row
                .iter()
                .map(|(k, v)| format!("?{}={}", terms.sym(*k), sexpr_value(terms, *v)))
                .collect();
            cells.sort();
            cells.join(" ")
        })
        .collect();
    shown.sort();
    shown.dedup();
    shown.join("; ")
}

/// The distinct models among the branches, keyed the way `answer.rs` counts
/// `k` — by canonical state, so two branches that reached the same model are
/// one model here too.
fn distinct_models<'a>(terms: &Terms, models: &[&'a Solution]) -> Vec<Actual<'a>> {
    let not = terms.kernel.not;
    let mut keys: Vec<Box<[FactId]>> = Vec::new();
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
///
/// `terms` is threaded in for one reason: the report explains a surplus fact
/// and the two probes above it do not. Passing `None` there is not an
/// optimisation, it is the correctness of the *choice* — a provenance line is
/// still a line, and counting it would let the model with the noisiest
/// derivation win `min_by_key`.
fn matching(
    ast: &Ast,
    terms: &Terms,
    wanted: &[&Model],
    actual: &[Actual],
    lines: &mut Vec<String>,
) -> bool {
    let n = wanted.len();
    let fits: Vec<Vec<bool>> = wanted
        .iter()
        .map(|w| {
            actual
                .iter()
                .map(|a| explain(ast, w, a, None).is_empty())
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
                .min_by_key(|&j| explain(ast, wanted[i], &actual[j], None).len())
                .unwrap_or(0);
            let which = if n == 1 {
                String::new()
            } else {
                format!("expectation {} of {n}: ", i + 1)
            };
            for line in explain(ast, wanted[i], &actual[best], Some(terms)) {
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

fn augment(
    i: usize,
    fits: &[Vec<bool>],
    taken_by: &mut [Option<usize>],
    seen: &mut [bool],
) -> bool {
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
///
/// `why` is `Some` only on the reporting pass — see [`matching`]. With it, a
/// surplus fact is followed by the derivation that put it there.
fn explain(ast: &Ast, want: &Model, actual: &Actual, why: Option<&Terms>) -> Vec<String> {
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
        let empty = FxHashMap::default();
        let got = actual.by_relation.get(rel).unwrap_or(&empty);
        let mut missing: Vec<&String> = want_set.iter().filter(|f| !got.contains_key(*f)).collect();
        let mut surplus: Vec<(&String, FactId)> = got
            .iter()
            .filter(|(f, _)| !want_set.contains(*f))
            .map(|(f, &id)| (f, id))
            .collect();
        missing.sort();
        surplus.sort_by(|a, b| a.0.cmp(b.0));
        for f in missing {
            lines.push(format!(
                "{rel}: expected {f}, and the model has no such fact"
            ));
        }
        for (f, id) in surplus {
            lines.push(format!(
                "{rel}: the model also has {f}, which the expectation does not list \
                 (naming a relation closes it)"
            ));
            // The next question, answered where it is asked. `--trace` is the
            // rest of the story; this is the one step that says which rule to
            // go and look at.
            if let Some(terms) = why
                && let Some(how) = provenance(terms, actual.kb, id)
            {
                lines.push(format!("  …{f} is {how}"));
            }
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
