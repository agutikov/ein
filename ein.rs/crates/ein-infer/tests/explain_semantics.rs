//! The justification graph and the saturator's own contract — T1a.10.2.2.
//!
//! Replaces five Python files, whose common subject is *what a derivation is*:
//!
//! | Python | what it owned |
//! |---|---|
//! | `tests/inference/test_explain.py` | the AND/OR label search over recorded justifications |
//! | `tests/inference/test_infer_closure.py` | `std.closure`'s derived `(__closed__ R)` and its hand-off to hypgen |
//! | `tests/inference/test_saturator.py` | the fixpoint contract: idempotence, stalling, priority bands |
//! | `tests/inference/test_saturator_fork_parity.py` | a fork saturates to what the same program saturates to directly |
//! | `tests/inference/test_why.py` | the `:why` reference character class |
//!
//! Two things had to be re-aimed rather than translated.
//!
//! **The Python tests read `kb._alt_justifications` directly.** The table is a
//! Rust private too, so what is asserted here is the observable it exists to
//! serve — [`Kb::justifications`], the fact's OR-node, and what the search
//! makes of it.
//!
//! **A fork means two different things.** [D3](../../../../docs/history/m1a_rust/divergences.md)
//! is about `try_commitment_set` forking a *saturated* root, where ein.rs
//! **resumes** the parent's saturation instead of re-deriving its closure — so
//! "the fork derived everything root did" is deliberately false there, and the
//! firing counts differ by design. Every fork below is a fork of an
//! **unsaturated** KB driven by a fresh [`Saturator`], which is the shape the
//! Python parity tests used and the one where the two arms must agree
//! firing-for-firing. Nothing here resumes a snapshot.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;
use std::path::PathBuf;

use ein_core::walks::{self, Justifications};
use ein_core::{FactId, Kb, Prov, ProvKind, Symbol, Terms, Value};
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explain::{Explanation, ExplanationBudget, explain, minimal_contradiction_frontier};
use ein_infer::hypgen::{HypGenStats, Skip};
use ein_infer::saturator::{Saturator, Session};
use ein_infer::{CLOSED, SharedMemo};
use ein_ir::{Ast, dump_canonical, parse};

// ── Fixtures ───────────────────────────────────────────────────────

/// Two facts, each derivable two ways, clashing.
///
/// The per-fact best picks are `X ← C` at size 1 and `Z ← {A,B}` (or `{D,E}`)
/// at size 2 — union 3 — but sharing `{A, B}` across **both** derivations
/// costs 2. Verbatim from `test_explain.py`, because the numbers are the
/// point.
const SHARED_OPTIMUM: &str = r#"
(relation A T) (relation B T) (relation C T) (relation D T) (relation E T)
(relation X T) (relation Z T)
(rule x-join () :match (and (A ?o) (B ?o)) :assert (X ?o) :why "xj" :priority 100)
(rule x-chain () :match (C ?o) :assert (X ?o) :why "xc" :priority 110)
(rule z-join () :match (and (A ?o) (B ?o)) :assert (Z ?o) :why "zj" :priority 120)
(rule z-pair () :match (and (D ?o) (E ?o)) :assert (Z ?o) :why "zp" :priority 130)
(rule clash () :match (and (X ?o) (Z ?o)) :assert (false) :why "clash" :priority 300)
(A a :source "(A)") (B a :source "(B)") (C a :source "(C)")
(D a :source "(D)") (E a :source "(E)")
"#;

/// `mirror` derives its own premise's swap, so once `(P x y)` is itself
/// derived the two P-facts justify each other — a genuine 2-cycle in the
/// justification graph. A `:source` fact would not cycle: givens are frontier
/// terminals and take no alternatives.
const CYCLIC: &str = r#"
(relation S T T) (relation P T T) (relation Q T)
(rule seed () :match (S ?a ?b) :assert (P ?a ?b) :why "s" :priority 90)
(rule mirror () :match (P ?a ?b) :assert (P ?b ?a) :why "m" :priority 100)
(rule clash () :match (and (P y x) (Q x)) :assert (false) :why "c" :priority 300)
(S x y :source "(S)") (Q x :source "(Q)")
"#;

const CONSISTENT: &str = "(relation R T T)\n(R x One :source \"(1)\")\n";

/// One symmetric activator over one edge — the smallest program with a
/// fixpoint worth reaching.
const MIRRORED_EDGE: &str = r#"
(rule sym (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100)
(relation r T T) (sym r)
(r A B :source "(1)")
"#;

/// `(full ?r)` iff every cell of `?r` is blocked — a `forall`, i.e. a nested
/// `absent`, whose guard is non-monotone. That is the one shape whose
/// candidates park, get re-judged and are never retired, so it is the shape
/// that makes the NAF boundary's own state observable.
const FORALL: &str = r#"
(relation row T)
(relation cell T T)
(relation blocked T)
(relation full T)
(relation done T)
(rule all-blocked ()
  :match  (and (row ?r) (absent (and (cell ?r ?c) (absent (blocked ?c)))))
  :assert (full ?r) :why "every cell of {?r} is blocked" :priority 200)
(rule finish ()
  :match  (and (full ?r) (absent (done ?r)))
  :assert (done ?r) :why "close {?r}" :priority 300)
(row R1 :source "(1)") (row R2 :source "(2)")
(cell R1 C1 :source "(3)") (cell R1 C2 :source "(4)")
(cell R2 C3 :source "(5)")
(blocked C1 :source "(6)") (blocked C2 :source "(7)")
"#;

const IMPORT_CLOSURE: &str = "(import std.closure :symbols (infer-closure))\n";

// ── Harness ────────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// The three arenas an engine call needs, kept together.
struct Fixture {
    ast: Ast,
    terms: Terms,
    kb: Kb,
}

fn load_text(text: &str) -> Fixture {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    Fixture { ast, terms, kb }
}

/// What a saturation looks like from outside the saturator.
///
/// Everything here is either a public observable of [`Saturator`] or a field
/// of the `--events` protocol — deliberately, because the Python originals
/// reached into `sat._parked` and `sat._park_stamp`, and a Rust test that
/// reached into the Rust equivalents would pin this build rather than the
/// language.
#[derive(Debug, Default, PartialEq, Eq)]
struct Run {
    /// `(rule, redundant, derived)` per firing, in order.
    firings: Vec<(Symbol, bool, Vec<FactId>)>,
    rounds: u32,
    admitted: u32,
    retired: u32,
    dropped: u32,
    /// Candidates still parked when the boundary last spoke — the `n_parked`
    /// field of the final `quiesce` event.
    parked: i64,
}

impl Run {
    fn productive(&self) -> impl Iterator<Item = &(Symbol, bool, Vec<FactId>)> {
        self.firings.iter().filter(|(_, redundant, _)| !redundant)
    }

    /// The rule names, resolved once the arena is no longer borrowed.
    fn productive_rules(&self, terms: &Terms) -> Vec<String> {
        self.productive()
            .map(|(rule, _, _)| terms.sym(*rule).to_string())
            .collect()
    }
}

/// Saturate `kb` to its fixpoint, recording the sequence and the boundary's
/// account of itself.
fn saturate_kb(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Run {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let mut firings = Vec::new();
    let (rounds, admitted, retired, dropped) = {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the fixture compiles");
        sat.saturate(&mut s, None, &mut |f| {
            firings.push((f.rule, f.redundant, f.derived.to_vec()));
        })
        .expect("the fixture saturates");
        (
            sat.naf_rounds,
            sat.naf_admitted,
            sat.naf_retired,
            sat.naf_dropped,
        )
    };
    Run {
        firings,
        rounds,
        admitted,
        retired,
        dropped,
        parked: last_quiesce_parked(&buffer.to_string_lossy()),
    }
}

/// `n_parked` from the last `quiesce` event, or -1 if the run never quiesced.
fn last_quiesce_parked(events: &str) -> i64 {
    events
        .lines()
        .filter(|l| l.contains("\"e\": \"quiesce\""))
        .filter_map(|l| {
            let at = l.find("\"n_parked\": ")? + "\"n_parked\": ".len();
            let rest = &l[at..];
            let end = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            rest[..end].parse::<i64>().ok()
        })
        .next_back()
        .unwrap_or(-1)
}

impl Fixture {
    fn saturate(&mut self) -> Run {
        saturate_kb(&self.ast, &mut self.terms, &mut self.kb)
    }

    fn sym(&self, name: &str) -> Symbol {
        self.terms
            .syms
            .get(name)
            .unwrap_or_else(|| panic!("{name} was never interned"))
    }

    /// The id of an already-believed fact, by its spelling.
    fn fact(&mut self, rel: &str, args: &[&str]) -> FactId {
        let id = self.intern(rel, args);
        assert!(
            self.kb.contains(id),
            "({rel} {}) is not in the KB",
            args.join(" ")
        );
        id
    }

    /// Intern a fact **without** believing it — the id exists, the KB does not
    /// hold it.
    fn intern(&mut self, rel: &str, args: &[&str]) -> FactId {
        let rel = self.terms.intern_text(rel).expect("room");
        let args: Vec<Value> = args
            .iter()
            .map(|a| self.terms.value_text(a).expect("room"))
            .collect();
        self.terms.intern_fact(rel, &args).expect("room")
    }

    /// The relation names of a fact set, as a multiset-free set — what the
    /// Python `_names` helper returned.
    fn names(&self, facts: &[FactId]) -> BTreeSet<String> {
        facts
            .iter()
            .map(|&f| self.terms.sym(self.terms.fact(f).0).to_string())
            .collect()
    }

    /// Every recorded justification of `fact`, as `rule` names.
    fn rules_deriving(&self, fact: FactId) -> BTreeSet<String> {
        self.kb
            .justifications(fact)
            .iter()
            .filter_map(|&p| self.terms.provs.get(p).rule)
            .map(|r| self.terms.sym(r).to_string())
            .collect()
    }

    fn contradiction_witnesses(&self) -> Vec<FactId> {
        ein_infer::contradiction::detect(&self.kb, &self.terms)
            .iter()
            .map(|c| c.witness())
            .collect()
    }

    fn explain_contradiction(&self, budget: &ExplanationBudget) -> Explanation {
        minimal_contradiction_frontier(&self.kb, &self.terms, None, budget)
    }
}

// ── The label search ───────────────────────────────────────────────

/// **the-search-finds-a-shared-optimum-a-greedy-pick-would-miss.**
///
/// This is the whole reason `explain.rs` exists beside
/// [`walks::unsat_core`]. Both clashing facts have two derivations; picking
/// each fact's own smallest one gives `{C} ∪ {A,B}` = 3 facts, and no per-fact
/// walk can do better, because the saving is *between* the facts — `{A, B}`
/// derives both. Only a search over combinations of justifications sees it, so
/// a frontier of 2 is the assertion that the AND/OR label search is really
/// running.
#[test]
fn the_search_finds_a_shared_optimum_a_greedy_pick_would_miss() {
    let mut f = load_text(SHARED_OPTIMUM);
    f.saturate();

    // Both alternatives really were recorded — otherwise there is only one
    // combination and the search has nothing to choose between.
    let x = f.fact("X", &["a"]);
    let z = f.fact("Z", &["a"]);
    assert_eq!(
        f.rules_deriving(x),
        ["x-chain", "x-join"].map(String::from).into(),
        "X's OR-node lost a derivation"
    );
    assert_eq!(
        f.rules_deriving(z),
        ["z-join", "z-pair"].map(String::from).into(),
        "Z's OR-node lost a derivation"
    );

    let result = f.explain_contradiction(&ExplanationBudget::default());
    assert!(
        result.exhausted,
        "the search gave up on a five-fact program"
    );
    assert_eq!(
        f.names(&result.frontier),
        ["A", "B"].map(String::from).into()
    );
    assert_eq!(
        result.len(),
        2,
        "a greedy per-fact pick returns 3 ({{C}} ∪ {{A,B}}); this must be 2"
    );
}

/// **the-search-terminates-and-grounds-out-on-a-cyclic-justification-graph.**
///
/// Once re-derivations are recorded, `(P x y)` and `(P y x)` justify each
/// other through the symmetric mirror — an ordinary situation, not a corner
/// case. A search that walked premises until it hit a leaf would loop, and one
/// that broke the loop carelessly could label a fact with *itself*, reporting a
/// conclusion as its own reason. The label fixpoint cannot: at the moment a
/// fact's own label is still empty it contributes nothing, so only the givens
/// can enter a frontier.
#[test]
fn the_search_terminates_and_grounds_out_on_a_cyclic_justification_graph() {
    let mut f = load_text(CYCLIC);
    f.saturate();

    let pxy = f.fact("P", &["x", "y"]);
    let pyx = f.fact("P", &["y", "x"]);
    assert_eq!(
        f.rules_deriving(pxy),
        ["mirror", "seed"].map(String::from).into(),
        "the cycle is not there: (P x y) must also be re-derivable by mirror"
    );
    assert_eq!(f.rules_deriving(pyx), ["mirror"].map(String::from).into());

    let result = f.explain_contradiction(&ExplanationBudget::default());
    assert!(result.exhausted);
    assert_eq!(
        f.names(&result.frontier),
        ["Q", "S"].map(String::from).into()
    );
    for &fact in &result.frontier {
        let kind =
            f.kb.primary(fact)
                .map(|p| f.terms.provs.get(p).kind)
                .expect("a frontier fact has provenance");
        assert_eq!(
            kind,
            ProvKind::Source,
            "a derived fact reached the frontier"
        );
    }
}

/// **a-consistent-kb-and-an-underivable-target-explain-to-nothing.**
///
/// Two ways of asking for an explanation that does not exist, and neither may
/// invent one. A consistent KB has no witness at all, so the result carries no
/// target; an interned-but-unbelieved fact *is* a legal target, and the graph
/// grounds it out rather than treating the dangling id as an assumption — which
/// is the failure that would put a fact the KB never held into a user's core.
#[test]
fn a_consistent_kb_and_an_underivable_target_explain_to_nothing() {
    let mut f = load_text(CONSISTENT);
    f.saturate();

    let result = f.explain_contradiction(&ExplanationBudget::default());
    assert!(result.is_empty(), "a consistent KB explained something");
    assert_eq!(result.target, None);

    let orphan = f.intern("R", &["nope", "nope"]);
    assert!(
        !f.kb.contains(orphan),
        "the fixture believes its own orphan"
    );
    let result = explain(&f.kb, &f.terms, &[orphan], &ExplanationBudget::default());
    assert!(
        result.frontier.is_empty(),
        "an underivable target was given a frontier: {:?}",
        f.names(&result.frontier)
    );
}

/// **a-given-explains-itself.**
///
/// A leaf is labelled `{{itself}}`, so explaining a clue returns the clue.
/// Nothing in the corpus digest reaches this: `explain_shape` only ever
/// targets rule-derived facts, so without a test the base case of the whole
/// fixpoint is unpinned — and the plausible wrong answers (an empty frontier,
/// or the fact's own re-derivation) are both silent.
#[test]
fn a_given_explains_itself() {
    let mut f = load_text(CONSISTENT);
    f.saturate();
    let given = f.fact("R", &["x", "One"]);
    let result = explain(&f.kb, &f.terms, &[given], &ExplanationBudget::default());
    assert_eq!(result.frontier, vec![given]);
    assert_eq!(result.target, Some(given));
}

/// **every-budget-cap-stays-sound-and-reports-itself.**
///
/// Minimum-cardinality over an AND/OR graph is exponential, so the search is
/// capped on four axes — and a cap that could return a *wrong* frontier would
/// be worse than one that returned none. Truncation only ever discards
/// environments, so each survivor is still a real set of derivation leaves:
/// the frontier stays inside the OR-aware premise closure under every cap.
/// The second half is the honesty requirement — `exhausted` is false whenever
/// the answer might not be the minimum, so the caller can tell "smallest" from
/// "smallest I got to".
#[test]
fn every_budget_cap_stays_sound_and_reports_itself() {
    let mut f = load_text(SHARED_OPTIMUM);
    f.saturate();

    let witnesses = f.contradiction_witnesses();
    let envelope: BTreeSet<FactId> =
        walks::unsat_core(&f.kb, &f.terms, &witnesses, Justifications::All)
            .into_iter()
            .collect();
    assert!(!envelope.is_empty(), "the fixture has no contradiction");

    let mut truncated_at_least_once = false;
    for budget in [
        ExplanationBudget {
            max_rounds: 1,
            ..ExplanationBudget::default()
        },
        ExplanationBudget {
            max_environments: 1,
            ..ExplanationBudget::default()
        },
        ExplanationBudget {
            max_facts: 2,
            ..ExplanationBudget::default()
        },
    ] {
        let result = f.explain_contradiction(&budget);
        for fact in &result.frontier {
            assert!(
                envelope.contains(fact),
                "{budget:?} returned {:?}, which no recorded derivation reaches",
                f.names(&result.frontier)
            );
        }
        assert!(
            !result.exhausted || result.len() == 2,
            "{budget:?} claimed a {}-fact frontier is the minimum",
            result.len()
        );
        truncated_at_least_once |= !result.exhausted;
    }
    assert!(
        truncated_at_least_once,
        "no cap actually truncated — the soundness claim would be vacuous"
    );
}

/// **max-env-size-turns-the-search-into-a-decision-question.**
///
/// The other caps trade completeness for time; this one changes the question.
/// At 2 the 2-fact explanation comes back; at 1 the answer is *empty*, and
/// that empty is a real answer — "there is no explanation of at most one
/// given" — not a truncated 2-fact one. A cap that silently returned a
/// 1-element prefix of the answer instead would be unsound in the worst way:
/// a frontier that does not force the target.
#[test]
fn max_env_size_turns_the_search_into_a_decision_question() {
    let mut f = load_text(SHARED_OPTIMUM);
    f.saturate();

    let at_two = f.explain_contradiction(&ExplanationBudget {
        max_env_size: Some(2),
        ..ExplanationBudget::default()
    });
    assert_eq!(at_two.len(), 2);
    assert_eq!(
        f.names(&at_two.frontier),
        ["A", "B"].map(String::from).into()
    );

    let at_one = f.explain_contradiction(&ExplanationBudget {
        max_env_size: Some(1),
        ..ExplanationBudget::default()
    });
    assert!(
        at_one.frontier.is_empty(),
        "no explanation of size <= 1 exists, but the search returned {:?}",
        f.names(&at_one.frontier)
    );
}

/// **a-rebuild-preserves-the-alternative-justifications.**
///
/// Every other index a KB holds is a projection of its fact list, so
/// `rebuild_indexes` recomputes them. The justification tables are not: they
/// record derivations the engine *attempted*, and the second derivation of a
/// fact leaves no trace in the fact set at all — re-deriving `(X a)` adds
/// nothing to `facts`. So a rebuild that recomputed them would silently drop
/// every alternative, the label search would fall back to one derivation per
/// fact, and the answer would get bigger without anything failing.
/// `incremental_indexing_and_a_rebuild_agree` cannot see this: it records no
/// alternative.
#[test]
fn a_rebuild_preserves_the_alternative_justifications() {
    let mut f = load_text(SHARED_OPTIMUM);
    f.saturate();

    let before: BTreeMap<FactId, Vec<String>> =
        f.kb.facts()
            .map(|id| {
                let rules: Vec<String> =
                    f.kb.justifications(id)
                        .iter()
                        .filter_map(|&p| f.terms.provs.get(p).rule)
                        .map(|r| f.terms.sym(r).to_string())
                        .collect();
                (id, rules)
            })
            .collect();
    assert!(
        f.kb.has_alternative_justifications(),
        "nothing recorded an alternative — the test would be vacuous"
    );
    let alternatives = before.values().filter(|r| r.len() > 1).count();
    assert_eq!(alternatives, 2, "X and Z are the two OR-nodes");

    f.kb.rebuild_indexes(&f.terms);

    let after: BTreeMap<FactId, Vec<String>> =
        f.kb.facts()
            .map(|id| {
                let rules: Vec<String> =
                    f.kb.justifications(id)
                        .iter()
                        .filter_map(|&p| f.terms.provs.get(p).rule)
                        .map(|r| f.terms.sym(r).to_string())
                        .collect();
                (id, rules)
            })
            .collect();
    assert_eq!(before, after, "a rebuild lost recorded derivations");

    // And the search still finds the shared optimum through them.
    let result = f.explain_contradiction(&ExplanationBudget::default());
    assert_eq!(
        f.names(&result.frontier),
        ["A", "B"].map(String::from).into()
    );
}

// ── `std.closure` ──────────────────────────────────────────────────

/// Does `r` carry `(__closed__ r)` after saturation?
fn closes(props: &str, imported: bool) -> bool {
    let src = format!(
        "{}(relation r A B)\n{props}\n",
        if imported { IMPORT_CLOSURE } else { "" }
    );
    let mut f = load_text(&src);
    f.saturate();
    let (Some(closed), Some(r)) = (f.terms.syms.get(CLOSED), f.terms.syms.get("r")) else {
        return false;
    };
    f.kb.facts_of(closed)
        .any(|id| f.terms.fact(id).1.first().and_then(|v| v.as_sym()) == Some(r))
}

/// **infer-closure-needs-functional-and-total.**
///
/// `functional ∧ total` is an *operational* witness — a total function's
/// extension is fixed by saturation, so speculating about it is wasted search.
/// Either marker alone is too weak and closing on it would prune branches the
/// answer depends on, which is a completeness bug that shows up as "no
/// solution" on a solvable puzzle. And the import is the only gate: there is no
/// config flag, so a program that never asked for the rule must never get it.
#[test]
fn infer_closure_needs_functional_and_total() {
    assert!(closes("(functional r) (total r)", true));
    assert!(!closes("(functional r)", true), "functional alone closed r");
    assert!(!closes("(total r)", true), "total alone closed r");
    assert!(
        !closes("(functional r) (total r)", false),
        "the rule fired without being imported — the import is the opt-in"
    );
}

/// Candidate hypothesis facts, and the pre-candidate skip counters.
fn hypotheses(f: &mut Fixture) -> (Vec<FactId>, HypGenStats) {
    let mut events = Events::off();
    let mut stats = HypGenStats::new();
    let mut out = Vec::new();
    {
        let mut s = Session {
            kb: &mut f.kb,
            terms: &mut f.terms,
            ast: &f.ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        ein_infer::hypgen::generate(&mut s, &mut stats, &mut |fid| {
            out.push(fid);
            ControlFlow::Continue(())
        })
        .expect("hypgen compiles");
    }
    (out, stats)
}

/// **a-derived-closure-marker-silences-hypgen.**
///
/// The hand-off is by *fact*, not by provenance: hypgen looks up
/// `(__closed__ R)` and does not care whether a human authored it,
/// `emit_closed` wrote it, or `infer-closure` derived it during saturation.
/// That is what makes `std.closure` an ordinary rule rather than an engine
/// mode — and it is only checkable end to end, because the same program
/// *without* the import produces the candidates this one suppresses.
#[test]
fn a_derived_closure_marker_silences_hypgen() {
    let objects = "(relation r Thing Thing)\n(functional r) (total r)\n\
                   (is-a a Thing :source \"(1)\") (is-a b Thing :source \"(2)\")\n";

    let mut open = load_text(objects);
    open.saturate();
    let (candidates, _) = hypotheses(&mut open);
    let r = open.sym("r");
    let open_r = candidates
        .iter()
        .filter(|&&c| open.terms.fact(c).0 == r)
        .count();
    assert!(
        open_r > 0,
        "the blind enumerator proposes nothing about r even unclosed — \
         the closed arm below would prove nothing"
    );

    let mut closed = load_text(&format!("{IMPORT_CLOSURE}{objects}"));
    closed.saturate();
    let (candidates, stats) = hypotheses(&mut closed);
    let r = closed.sym("r");
    assert!(
        !candidates.iter().any(|&c| closed.terms.fact(c).0 == r),
        "a closed relation still yielded candidates"
    );
    assert!(
        stats.pre_candidate[Skip::ClosedRelation as usize] > 0,
        "r was skipped for some other reason than being closed: {:?}",
        stats.report_lines()
    );
}

/// **an-identical-re-declaration-collapses.**
///
/// Import is idempotent, and it has to be: two modules may pull the same
/// dependency, and the diamond must collapse rather than trip a duplicate-name
/// error. The dedup key is the *body*, not the name — so this is the half that
/// must load, and a differing body next to it is the half that must not. Both
/// arms are here because either one alone is satisfied by a broken rule: "never
/// error" and "always error" each pass one of them.
#[test]
fn an_identical_re_declaration_collapses() {
    let text = ein_ir::stdlib::resolve_default()
        .read("closure.ein")
        .expect("the stdlib is resolvable from a test");
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, Some("closure.ein")).expect("the stdlib parses");
    let verbatim = dump_canonical(&ast, &forms);

    let mut f = load_text(&format!(
        "{IMPORT_CLOSURE}{verbatim}(relation r A B)\n(functional r) (total r)\n"
    ));
    let name = f.sym("infer-closure");
    assert!(
        f.kb.program().rules.get(name).is_some(),
        "the re-declared rule is gone"
    );
    assert_eq!(
        f.kb.program()
            .rules
            .iter()
            .filter(|(n, _)| *n == name)
            .count(),
        1
    );
    // One rule, and it still fires.
    f.saturate();
    let closed = f.sym(CLOSED);
    assert_eq!(f.kb.n_facts_of(closed), 1);

    // The control: same name, different body.
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let differing = format!(
        "{IMPORT_CLOSURE}(rule infer-closure () :match (functional ?R) \
         :assert (__closed__ ?R) :priority 90)\n"
    );
    let forms = parse(&mut ast, &differing, Some("<fixture>")).expect("parses");
    let err = ein_ir::load(&mut ast, &mut terms, &forms, None)
        .expect_err("a differing redefinition must not load");
    assert!(
        err.0.contains("conflicting definitions of 'infer-closure'"),
        "unexpected error: {}",
        err.0
    );
}

// ── The fixpoint contract ──────────────────────────────────────────

/// **saturation-is-idempotent.**
///
/// The fixpoint has to be a *fixpoint*: a second `saturate()` on a saturator
/// that already reached one does no work. It is the property every caller
/// above the saturator assumes — the fork loop calls it after writing a
/// commitment and expects to pay only for the delta — and the way to break it
/// is to leave a candidate in the queue whose dedup key was never recorded, so
/// it re-fires forever.
#[test]
fn saturation_is_idempotent() {
    let mut f = load_text(MIRRORED_EDGE);
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut f.kb,
        terms: &mut f.terms,
        ast: &f.ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("compiles");
    let first = sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    assert!(first > 0, "the first pass must do something");
    let second = sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    assert_eq!(
        second, 0,
        "re-saturating a fixpoint produced {second} firings"
    );
}

/// **is-stalled-reopens-after-a-write-outside-step.**
///
/// `is_stalled` is the primitive a fork loop consults to decide whether to keep
/// going, and the reason it cannot simply report "the queue is empty" is this
/// test's third phase: a hypothesis arrives as a *direct write* to the KB, not
/// as a firing, so nothing in the saturator's own flow knows a new match may
/// exist. It therefore forces an enqueue pass before answering — and a stalled
/// saturator that stayed stalled would silently stop the search one write short
/// of the answer.
#[test]
fn is_stalled_reopens_after_a_write_outside_step() {
    let mut f = load_text(MIRRORED_EDGE);
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut f.kb,
        terms: &mut f.terms,
        ast: &f.ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("compiles");
    assert!(
        !sat.is_stalled(&mut s).expect("judges"),
        "work was available before the first step"
    );
    sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    assert!(sat.is_stalled(&mut s).expect("judges"), "not at a fixpoint");

    // A fact written straight into the KB, the way a commitment arrives.
    let rel = s.terms.intern_text("r").expect("room");
    let args = [
        s.terms.value_text("X").expect("room"),
        s.terms.value_text("Y").expect("room"),
    ];
    let prov = s.terms.provs.push(Prov::from_hypothesis(1, None));
    s.kb.add_and_index_fact(s.terms, rel, &args, Some(prov))
        .expect("room");

    assert!(
        !sat.is_stalled(&mut s).expect("judges"),
        "a direct write left the saturator stalled — the mirror of (r X Y) \
         is derivable and nothing would derive it"
    );
}

/// **an-activator-with-an-empty-extent-saturates-to-nothing.**
///
/// A rule with a properly-instantiated activator and no facts to match is not
/// the same case as a rule with no activator: the first has a compiled plan and
/// an empty extent, the second has no plan at all. Both saturate to nothing, so
/// the only way to tell them apart is that this one reaches a stalled fixpoint
/// with a plan in hand — and a matcher that mishandled an empty relation would
/// show up here as a firing, a panic, or a saturation that never stalls.
#[test]
fn an_activator_with_an_empty_extent_saturates_to_nothing() {
    let mut f = load_text(
        "(rule s (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why \"s\" :priority 100)\n\
         (relation r T T) (s r)\n",
    );
    let run = f.saturate();
    assert!(run.firings.is_empty(), "an empty extent produced firings");

    let mut events = Events::off();
    let mut s = Session {
        kb: &mut f.kb,
        terms: &mut f.terms,
        ast: &f.ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("compiles");
    assert!(
        sat.is_stalled(&mut s).expect("judges"),
        "a rule with nothing to match is stalled from the start"
    );
}

/// **priority-bands-order-the-firing-sequence.**
///
/// Bands are advisory for *soundness* since S1.21.8 — `(absent …)` is judged on
/// the closure boundary either way — but they still decide the shape of every
/// trace, and elimination reading negatives the propagate band has not produced
/// yet would weaken the state it writes. The claim has two halves and the
/// second is the one a naive implementation gets wrong: the bands are not a
/// one-way sweep. A fact produced by a *higher* band re-enters the lower one
/// immediately, ahead of the higher band's own remaining candidates — a global
/// priority queue, not a pipeline.
#[test]
fn priority_bands_order_the_firing_sequence() {
    let mut f = load_text(
        "(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)\n\
         \x20 :why \"s\" :priority 100)\n\
         (rule transitive (?rel)\n\
         \x20 :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))\n\
         \x20 :assert (?rel ?a ?c) :why \"t\" :priority 200)\n\
         (rule type-exclusivity (?R)\n\
         \x20 :match (and (is-a ?a ?T) (is-a ?b ?T) (neq ?a ?b))\n\
         \x20 :assert (not (?R ?a ?b)) :why \"x\" :priority 300)\n\
         (is-a Red Color) (is-a Blue Color)\n\
         (relation r T T) (symmetric r) (transitive r) (type-exclusivity r)\n\
         (r A B :source \"(1)\") (r B C :source \"(2)\")\n",
    );
    let run = f.saturate();
    let rules = run.productive_rules(&f.terms);
    assert!(!rules.is_empty(), "nothing was derived");

    let first_of = |name: &str| rules.iter().position(|r| r == name);
    let (sym, trans, excl) = (
        first_of("symmetric"),
        first_of("transitive"),
        first_of("type-exclusivity"),
    );
    assert_eq!(sym, Some(0), "the 100 band must open: {rules:?}");
    let trans = trans.expect("transitive must fire");
    let excl = excl.expect("type-exclusivity must fire");
    assert!(trans < excl, "300 preempted 200: {rules:?}");
    assert!(
        rules[..trans].iter().all(|r| r == "symmetric"),
        "a 200-band firing before the 100 band drained: {rules:?}"
    );
    // The re-entry: transitive's conclusion opens a symmetric match, and that
    // match runs before the rest of the transitive candidates.
    assert_eq!(
        rules.get(trans + 1).map(String::as_str),
        Some("symmetric"),
        "the fact the 200 band produced did not re-enter the 100 band: {rules:?}"
    );

    // The scale arm: the same discipline on a real puzzle.
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = ein_ir::load_file(
        &mut ast,
        &mut terms,
        &repo_root().join("examples/zebra.ein"),
    )
    .expect("zebra.ein loads");
    let run = saturate_kb(&ast, &mut terms, &mut kb);
    let rules = run.productive_rules(&terms);
    let propagate = [
        "slot-partition-setup",
        "slot-spatial-setup",
        "symmetric-negative-setup",
        "symmetric",
        "symmetric-negative",
        "includes",
    ];
    let elimination = ["slot-elimination", "slot-fill"];
    assert!(
        propagate.contains(&rules[0].as_str()),
        "zebra's first productive firing was {}, not propagate-band",
        rules[0]
    );
    assert!(
        rules.iter().any(|r| elimination.contains(&r.as_str())),
        "zebra's root saturation must reach the elimination band — it is what \
         places Yellow and Water in House-1"
    );
}

// ── Fork vs direct ─────────────────────────────────────────────────

/// **fork-and-direct-saturation-agree.**
///
/// The bug this pins was real and quiet: plan compilation asked the *rule* for
/// its activators, and a rule's activator list came from the KB it was loaded
/// into — the parent — so on a fork every rule whose activator fact is derived
/// at runtime lost its plan, and five `disjunctive-prune` firings plus one
/// cascade simply did not happen. Nothing failed; the fork just knew less.
/// zebra2 is the fixture because its `adjacent-via-fwd` / `disjunctive-prune`
/// activators are exactly that shape.
///
/// The fork here is of an **unsaturated** KB with a fresh saturator, so
/// [D3](../../../../docs/history/m1a_rust/divergences.md) — a fork *resuming* a
/// saturated root's snapshot, where the re-derivation is deliberately skipped —
/// does not apply, and the firing counts are required to match exactly.
#[test]
fn fork_and_direct_saturation_agree() {
    let path = repo_root().join("examples/zebra2.ein");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    // One arena, so a `FactId` means the same thing in both arms.
    let mut direct = ein_ir::load_file(&mut ast, &mut terms, &path).expect("zebra2 loads");
    let mut parent = ein_ir::load_file(&mut ast, &mut terms, &path).expect("zebra2 loads");
    let mut forked = parent.fork();

    emit_closed_on(&ast, &mut terms, &mut direct);
    emit_closed_on(&ast, &mut terms, &mut forked);
    let a = saturate_kb(&ast, &mut terms, &mut direct);
    let b = saturate_kb(&ast, &mut terms, &mut forked);

    assert_eq!(
        a.firings.len(),
        b.firings.len(),
        "firing-count divergence: direct={}, fork={}",
        a.firings.len(),
        b.firings.len()
    );
    assert_eq!(
        counts_by_relation(&direct, &terms),
        counts_by_relation(&forked, &terms),
        "fact-count divergence by relation"
    );
    assert_eq!(
        ein_infer::state_key(&direct),
        ein_infer::state_key(&forked),
        "the two arms reached different states"
    );
    assert!(
        a.firings.len() > 100,
        "only {} firings — zebra2 stopped exercising this",
        a.firings.len()
    );
}

/// **fork-parity-extends-to-the-naf-boundary.**
///
/// The fact set is not all a saturation carries: since S1.21.8 there is a
/// second phase with state of its own — the parked candidates, their watch
/// stamps, and the round counters — and a fork that agreed about the facts
/// while disagreeing about *those* would diverge on the next write, not on this
/// one. The fixture is a `forall`, because a nested `absent` is the one guard
/// shape that can flip fail→pass: its candidates park, get re-judged, and are
/// never retired. The non-vacuity assertions are load-bearing — on a fixture
/// that never reaches the boundary, this test passes while asserting nothing.
#[test]
fn fork_parity_extends_to_the_naf_boundary() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, FORALL, Some("<forall>")).expect("parses");
    let mut direct = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    let mut parent = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    let mut forked = parent.fork();

    let a = saturate_kb(&ast, &mut terms, &mut direct);
    let b = saturate_kb(&ast, &mut terms, &mut forked);
    assert_eq!(a, b, "fork and direct disagree about the boundary");
    assert_eq!(ein_infer::state_key(&direct), ein_infer::state_key(&forked));

    assert!(
        a.rounds > 1,
        "the boundary spoke once — nothing was re-judged"
    );
    assert!(a.admitted > 0, "no candidate was ever admitted");
    assert!(
        a.parked > 0,
        "nothing stayed parked: R2 must remain a standing question"
    );
    let full = terms.syms.get("full").expect("interned");
    let derived: Vec<String> = direct
        .facts_of(full)
        .map(|f| ein_infer::events::sexpr(&terms, f))
        .collect();
    assert_eq!(derived, ["(full R1)"], "the forall itself is wrong");
}

/// **a-runtime-derived-activator-gets-a-plan-on-a-fork.**
///
/// The minimal reproducer of the same bug, and the sharper statement of it: the
/// `target` rule's activator `(target X)` does not exist when saturation
/// starts — `meta-derive` concludes it — so the rule has to be compiled
/// *during* the run, from the KB the run is over. Read the parent's activator
/// map instead and `(done X)` is never derived; nothing else about the fork
/// looks wrong.
#[test]
fn a_runtime_derived_activator_gets_a_plan_on_a_fork() {
    let mut f = load_text(
        "(rule meta-derive ()\n\
         \x20 :match  (trigger ?x)\n\
         \x20 :assert (target ?x)\n\
         \x20 :why    \"trigger implies a target activator\"\n\
         \x20 :priority 100)\n\
         (rule target (?x)\n\
         \x20 :match  (trigger ?y)\n\
         \x20 :assert (done ?y)\n\
         \x20 :why    \"trigger fires target\"\n\
         \x20 :priority 200)\n\
         (relation trigger T)\n(relation done T)\n(relation target T)\n\
         (trigger X :source \"(1)\")\n",
    );
    let mut forked = f.kb.fork();
    saturate_kb(&f.ast, &mut f.terms, &mut forked);

    let done = f.terms.syms.get("done").expect("interned");
    let facts: Vec<String> = forked
        .facts_of(done)
        .map(|id| ein_infer::events::sexpr(&f.terms, id))
        .collect();
    assert_eq!(
        facts,
        ["(done X)"],
        "the runtime-derived activator (target X) never got a plan on the fork"
    );
}

// ── `:why` references ──────────────────────────────────────────────

/// **why-reference-names-admit-hyphens-and-underscores.**
///
/// `{?some-var}` has to reach the renderer as a *reference*, and two separate
/// things have to admit the hyphen for that: the IR's VAR production
/// (`\?[A-Za-z][A-Za-z0-9_*-]*`) and the binding key the firing records, which
/// is what the substitution looks the name up in. This asserts the half
/// `ein-infer` owns — a hyphenated and an underscored variable bind, and their
/// provenance entries are spelled exactly as the template spells them. If the
/// lexer stopped at the `-`, `?some` and `-var` would not even parse as one
/// token; if the binding were normalised, `{?some-var}` would render as
/// literal text and the trace would silently show template source to a user.
/// The substitution itself lives in `ein-render::why`, which `ein-infer` does
/// not depend on.
#[test]
fn a_why_reference_name_may_carry_hyphens_and_underscores() {
    let mut f = load_text(
        "(relation p T)\n(relation q T)\n\
         (rule r () :match (p ?some-var) :assert (q ?some-var)\n\
         \x20 :why \"{?some-var} and {?some_var2}\" :priority 100)\n\
         (p A :source \"(1)\")\n",
    );
    f.saturate();

    let derived = f.fact("q", &["A"]);
    let prov =
        f.kb.primary(derived)
            .expect("derived facts have provenance");
    let bindings =
        ein_infer::firing::rendered_bindings(&f.terms, &f.terms.provs.get(prov).bindings);
    assert_eq!(
        bindings,
        vec![("some-var".to_string(), "A".to_string())],
        "the binding key is not spelled the way the template references it"
    );

    // And the template survived the loader with its references intact.
    let rule = f.sym("r");
    let why =
        f.kb.program()
            .rules
            .get(rule)
            .and_then(|r| r.why)
            .map(|w| f.terms.sym(w).to_string())
            .expect("the rule has a :why");
    assert!(
        why.contains("{?some-var}"),
        "the template was rewritten: {why}"
    );
}

// ── Shared helpers ─────────────────────────────────────────────────

fn emit_closed_on(ast: &Ast, terms: &mut Terms, kb: &mut Kb) {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    ein_infer::emit_closed(&mut s).expect("the closure pass compiles");
}

fn counts_by_relation(kb: &Kb, terms: &Terms) -> BTreeMap<String, usize> {
    let mut out: BTreeMap<String, usize> = BTreeMap::new();
    for f in kb.facts() {
        *out.entry(terms.sym(terms.fact(f).0).to_string())
            .or_default() += 1;
    }
    out
}
