//! M1f S1f.5.15's T3 (T1f.5.15.3) — **the rule dependency graph, its strata,
//! and the band each rule's shape derives**, held on small programs and on
//! the corpus.
//!
//! `ein_infer::strata` builds the graph over the plans a run compiled, so each
//! test saturates root first, as `naf_deps` asks. The derived order
//! (`saturator::Order::Derived`) is held to derive what the declared order
//! derives on every corpus program: no order decides what a stratified
//! program derives (`naf_semantics::priority_does_not_decide_what_is_derivable`),
//! and the stage's T1 found no stored fact moved on the 511 with every
//! `:priority` taken out.
//!
//! ```text
//! cargo test -p ein-infer --test strata
//! cargo run --release -p ein-infer --example rule_bands   # the diff against :priority
//! ```

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use ein_core::{Kb, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::monotone;
use ein_infer::plan::Plan;
use ein_infer::saturator::{Order, Saturator, Session};
use ein_infer::strata::{self, Band};
use ein_ir::{Ast, load, parse};

/// `text` loaded and root saturated under `order`: the KB, the terms, and the
/// plans the run compiled. `None` where it does not load or saturate.
fn saturated(text: &str, dir: Option<&Path>, order: Order) -> Option<(Kb, Terms, Vec<Arc<Plan>>)> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, None).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, dir).ok()?;
    let memo = SharedMemo::default();
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?.with_order(order);
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    }
    let plans = monotone::plans_of(&memo);
    Some((kb, terms, plans))
}

fn bands(text: &str) -> BTreeMap<String, (Option<Band>, Option<u32>)> {
    let (kb, terms, plans) = saturated(text, None, Order::Declared).expect("saturates");
    let g = strata::graph(&kb, &terms, &plans);
    strata::rule_bands(&kb, &terms, &plans, &g)
        .into_iter()
        .map(|r| (terms.sym(r.rule).to_string(), (r.derived, r.stratum)))
        .collect()
}

/// **One rule of each band**, and the strata: a relation read under an
/// `absent` puts what it derives one stratum up.
#[test]
fn each_shape_derives_its_band_and_an_absent_read_is_a_stratum() {
    let b = bands(
        r#"
        (relation p T) (relation q T) (relation r T) (relation s T T)
        (import std.macro :symbols (forall))
        (p a) (p b) (s a b)
        (rule one () :match (p ?x) :assert (q ?x))
        (rule join () :match (and (p ?x) (s ?x ?y)) :assert (r ?y))
        (rule neg () :match (and (p ?x) (p ?y) (neq ?x ?y)) :assert (not (s ?y ?x)))
        (rule gate () :match (and (p ?x) (absent (r ?x))) :assert (r ?x))
        (rule bad () :match (and (q ?x) (r ?x) (s ?x ?x)) :assert (false))
        (rule survivor () :match (and (p ?x) (p ?y) (forall ?z (and (p ?z) (neq ?z ?y)) (not (s ?x ?z)))) :assert (s ?x ?y))
        "#,
    );
    assert_eq!(b["one"], (Some(Band::Propagate), Some(0)), "one premise");
    assert_eq!(b["join"].0, Some(Band::Derive), "a join");
    assert_eq!(
        b["neg"].0,
        Some(Band::Derive),
        "a stored negative from a join"
    );
    assert_eq!(
        b["gate"].0,
        Some(Band::Derive),
        "an absent that is no forall"
    );
    assert_eq!(b["bad"].0, Some(Band::Propagate), "a refutation");
    assert_eq!(
        b["survivor"].0,
        Some(Band::Eliminate),
        "a forall over negatives"
    );
    // `gate` reads `r` under an `absent` and writes it: a negative cycle, so
    // `r` keeps the stratum of what flows into it.
    let (kb, terms, plans) = saturated(
        "(relation p T) (relation q T) (p a)
         (rule g () :match (and (p ?x) (absent (q ?x))) :assert (q ?x))",
        None,
        Order::Declared,
    )
    .expect("saturates");
    let g = strata::graph(&kb, &terms, &plans);
    assert!(!g.stratified(), "{:?}", g.cycles);
    assert_eq!(g.cycles[0].negative, ["g: q →¬ q"]);
    let (kb, terms, plans) = saturated(
        "(relation p T) (relation q T) (relation r T) (p a)
         (rule g () :match (and (p ?x) (absent (q ?x))) :assert (r ?x))",
        None,
        Order::Declared,
    )
    .expect("saturates");
    let g = strata::graph(&kb, &terms, &plans);
    assert!(g.stratified());
    assert_eq!((g.stratum["q"], g.stratum["r"], g.depth()), (0, 1, 2));
}

/// **The fixtures both ways**: `features/07_unstratifiable.ein` has a
/// negative cycle, `features/01_not_and_absent.ein` (an `absent` over a
/// declared-only relation) has none.
#[test]
fn the_unstratifiable_fixture_has_a_negative_cycle_and_its_sibling_none() {
    let root = ein_corpus::repo_root();
    for (file, stratified) in [
        ("examples/features/07_unstratifiable.ein", false),
        ("examples/features/01_not_and_absent.ein", true),
    ] {
        let path = root.join(file);
        let text = std::fs::read_to_string(&path).expect("reads");
        let (kb, terms, plans) =
            saturated(&text, path.parent(), Order::Declared).expect("saturates");
        let g = strata::graph(&kb, &terms, &plans);
        assert_eq!(g.stratified(), stratified, "{file}: {:?}", g.cycles);
    }
}

/// **The stdlib's bands, where its shape speaks plainly**, on the stdlib's own
/// test of domain elimination (`tests/stdlib/`, not a puzzle: TE-L2): a
/// declaration's fan-out and a refutation propagate, a stored negative from a
/// join derives, and survivor forcing eliminates.
#[test]
fn the_stdlib_rules_derive_the_bands_their_shapes_give() {
    let (kb, terms, plans) = {
        let path = ein_corpus::repo_root().join("tests/stdlib/bijection/02_domain_elimination.ein");
        let text = std::fs::read_to_string(&path).expect("reads");
        saturated(&text, path.parent(), Order::Declared).expect("saturates")
    };
    let g = strata::graph(&kb, &terms, &plans);
    let got: BTreeMap<String, Option<Band>> = strata::rule_bands(&kb, &terms, &plans, &g)
        .into_iter()
        .map(|r| (terms.sym(r.rule).to_string(), r.derived))
        .collect();
    for (rule, band) in [
        ("bijective-setup", Band::Propagate),
        ("functional", Band::Propagate),
        ("functional-negative", Band::Derive),
        ("domain-elimination", Band::Eliminate),
    ] {
        assert_eq!(got.get(rule).copied().flatten(), Some(band), "{rule}");
    }
}

/// **The derived order derives what the declared order derives**, on every
/// corpus program whose root saturates: the same stored facts, the order of
/// the agenda and of the boundary aside.
#[test]
fn the_derived_order_derives_what_the_declared_order_derives() {
    let mut compared = 0usize;
    let mut bad = Vec::new();
    for path in ein_corpus::corpus_files() {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let facts = |order: Order| -> Option<Vec<String>> {
            let (kb, terms, _) = saturated(&text, path.parent(), order)?;
            let mut v: Vec<String> = kb.facts().map(|f| terms.fact_text(f)).collect();
            v.sort();
            Some(v)
        };
        let (Some(a), Some(b)) = (facts(Order::Declared), facts(Order::Derived)) else {
            continue;
        };
        compared += 1;
        if a != b {
            bad.push(path.display().to_string());
        }
    }
    assert!(
        bad.is_empty(),
        "the stored facts moved with the order: {bad:?}"
    );
    assert!(compared >= 300, "{compared} programs compared");
}
