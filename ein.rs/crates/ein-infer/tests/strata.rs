//! M1f S1f.5.15's T3 and T4 — **the rule dependency graph and its strata**,
//! held on small programs and on the stage's two fixtures.
//!
//! `ein_infer::strata` builds the graph over the plans a run compiled, so each
//! test saturates root first, as `naf_deps` asks. The band T3 derived from a
//! rule's shape went with `:priority` (the user's ruling of 2026-11-08 on
//! Q-M1f.6: *"plain FIFO"*); what the graph is for since is the
//! stratification diagnostic and the stratum `ein saturate --dump` prints.
//!
//! ```text
//! cargo test -p ein-infer --test strata
//! ```

use std::path::Path;
use std::sync::Arc;

use ein_core::{Kb, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::monotone;
use ein_infer::plan::Plan;
use ein_infer::saturator::{Saturator, Session};
use ein_infer::strata;
use ein_ir::{Ast, load, parse};

/// `text` loaded and root saturated: the KB, the terms, and the plans the run
/// compiled. `None` where it does not load or saturate.
fn saturated(text: &str, dir: Option<&Path>) -> Option<(Kb, Terms, Vec<Arc<Plan>>)> {
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
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    }
    let plans = monotone::plans_of(&memo);
    Some((kb, terms, plans))
}

/// **An `absent` read is a stratum, and a relation read under an `absent` by
/// the rule that writes it is a negative cycle.**
#[test]
fn an_absent_read_is_a_stratum_and_a_self_read_is_a_negative_cycle() {
    let (kb, terms, plans) = saturated(
        "(relation p T) (relation q T) (p a)
         (rule g () :match (and (p ?x) (absent (q ?x))) :assert (q ?x))",
        None,
    )
    .expect("saturates");
    let g = strata::graph(&kb, &terms, &plans);
    assert!(!g.stratified(), "{:?}", g.cycles);
    assert_eq!(g.cycles[0].negative, ["g: q →¬ q"]);
    assert!(g.cycles[0].odd);

    let (kb, terms, plans) = saturated(
        "(relation p T) (relation q T) (relation r T) (relation s T) (p a)
         (rule one () :match (p ?x) :assert (s ?x))
         (rule g () :match (and (s ?x) (absent (q ?x))) :assert (r ?x))",
        None,
    )
    .expect("saturates");
    let g = strata::graph(&kb, &terms, &plans);
    assert!(g.stratified(), "{:?}", g.cycles);
    assert_eq!(
        (
            g.stratum["p"],
            g.stratum["s"],
            g.stratum["q"],
            g.stratum["r"],
            g.depth()
        ),
        (0, 0, 0, 1, 2)
    );
    let rules: Vec<(String, Option<u32>)> = strata::rule_strata(&kb, &terms, &plans, &g)
        .into_iter()
        .map(|(r, k)| (terms.sym(r).to_string(), k))
        .collect();
    assert_eq!(
        rules,
        [("one".to_string(), Some(0)), ("g".to_string(), Some(1))]
    );
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
        let (kb, terms, plans) = saturated(&text, path.parent()).expect("saturates");
        let g = strata::graph(&kb, &terms, &plans);
        assert_eq!(g.stratified(), stratified, "{file}: {:?}", g.cycles);
    }
}
