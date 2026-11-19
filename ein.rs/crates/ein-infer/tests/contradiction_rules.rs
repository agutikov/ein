//! **`std.contradiction`'s rules fire on a fact beside its negation, and what
//! their firing adds is the fact `(false)`, never the verdict.**
//!
//! A state that holds a fact and its stored negation is dead by the kernel's
//! own reading, with no rule. `clash-0` to `clash-6` state that contradiction
//! as a rule, one a count of arguments
//! ([the module](../../../../stdlib/contradiction.ein)). So the claim their
//! firing fixtures carry, `(solve … :expect (false))`, holds with the rule
//! and without it, and no claim made of facts can tell the two apart: a dead
//! state has no model to list. What a fixture cannot state is asserted here,
//! in-process, as [`obligation_reports.rs`](obligation_reports.rs) does for
//! the owe counts:
//!
//! - **the firing**: each rule fires once on its fixture, its premises the
//!   pair and its conclusion `(false)`, a fact root then holds;
//! - **the verdict is not the rule's**: the same seven programs with the
//!   activator's line left out are dead all the same, and nothing fires;
//! - **silence**: where no fact meets its negation, all seven rules are
//!   activated and none fires;
//! - **a derived activator acts** as a written one does.
//!
//! The fixtures are `tests/stdlib/contradiction/`'s, and the coverage gate
//! reads the same firings ([`stdlib_coverage.rs`](stdlib_coverage.rs)).

use std::path::PathBuf;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::described::Described;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load_query, parse};

/// The firing fixture of `clash-n`.
fn fixture(n: usize) -> String {
    format!("tests/stdlib/contradiction/{:02}_clash_{n}.ein", n + 1)
}

/// `rel`'s text as `edit` leaves it, described as `ein solve` describes it:
/// the description, and every `fire` event of the run.
fn describe(rel: &str, edit: impl Fn(String) -> String) -> (Described, Vec<serde_json::Value>) {
    let path: PathBuf = repo_root().join(rel);
    let text = edit(std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}")));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut kb = load_query(&mut ast, &mut terms, &forms, path.parent(), 0)
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
    let mode = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let answer = explore::run_with(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(mode),
        &mut events,
    )
    .unwrap_or_else(|e| panic!("{rel}: {e}"));
    drop(events);
    let fires = buffer
        .to_string_lossy()
        .lines()
        .map(|l| serde_json::from_str::<serde_json::Value>(l).expect("an event is a JSON line"))
        // The firings that derive a fact: a firing that ends a state, the
        // rule's every program gets on the `(false)`, carries the word it
        // states and derives none.
        .filter(|e| e["e"] == "fire" && e.get("ein").is_none())
        .collect();
    let described = *answer
        .described
        .unwrap_or_else(|| panic!("{rel}: describe describes"));
    (described, fires)
}

/// `text` less every line that starts with `prefix`.
fn without(text: &str, prefix: &str) -> String {
    text.lines()
        .filter(|l| !l.starts_with(prefix))
        .map(|l| format!("{l}\n"))
        .collect()
}

fn texts(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .expect("a list")
        .iter()
        .map(|x| x.as_str().expect("a fact's text").to_string())
        .collect()
}

/// **Each rule fires once on its fixture: the pair is its premises, `(false)`
/// its conclusion.**
///
/// One rule a file, so the one firing of the run is the rule's.
#[test]
fn each_clash_rule_fires_once_and_its_premises_are_the_pair() {
    for n in 0..=6 {
        let rel = fixture(n);
        let rule = format!("clash-{n}");
        let (d, fires) = describe(&rel, |t| t);
        assert_eq!(fires.len(), 1, "{rel}: one firing, the rule's: {fires:?}");
        let fire = &fires[0];
        assert_eq!(fire["rule"], rule.as_str(), "{rel}");
        assert_eq!(
            fire["redundant"], false,
            "{rel}: the firing derives a new fact"
        );
        assert_eq!(
            texts(&fire["derived"]),
            ["(false)"],
            "{rel}: what the rule concludes"
        );
        let premises = texts(&fire["premises"]);
        assert_eq!(premises.len(), 2, "{rel}: the pair: {premises:?}");
        assert_eq!(
            premises[1],
            format!("(not {})", premises[0]),
            "{rel}: a fact and its negation"
        );
        assert!(!d.root.alive, "{rel}: root is dead");
        // Root now holds two contradictions over one core, the pair and the
        // `(false)` the rule derived from it, and which of them the
        // explanation is taken over is the explanation's choice: its
        // tie-break is the witnesses' rank, a fact's text against `(false)`.
        // So the row reads the rule or `(given)`, and nothing else.
        assert!(
            d.root.refuted_by == [rule.clone()] || d.root.refuted_by == ["(given)"],
            "{rel}: refuted by the rule, or by the pair as given: {:?}",
            d.root.refuted_by
        );
    }
}

/// **The verdict is the kernel's: without the activator the same root is
/// dead, and no rule fires.**
///
/// Each fixture with its `(clash-N …)` line left out. The module is still
/// imported, so its rules are loaded and none is activated.
#[test]
fn without_the_activator_the_pair_is_dead_all_the_same_and_nothing_fires() {
    for n in 0..=6 {
        let rel = fixture(n);
        let activator = format!("(clash-{n} ");
        let (d, fires) = describe(&rel, |t| without(&t, &activator));
        assert!(fires.is_empty(), "{rel}: nothing fires: {fires:?}");
        assert!(!d.root.alive, "{rel}: root is dead by the kernel's reading");
        assert!(
            d.root.refuted_by.iter().all(|r| !r.contains("clash")),
            "{rel}: no rule of the module refutes it: {:?}",
            d.root.refuted_by
        );
    }
}

/// **Loaded, activated and silent: where no fact meets its negation, none of
/// the seven fires.**
#[test]
fn no_clash_rule_fires_where_no_fact_meets_its_negation() {
    let rel = "tests/stdlib/contradiction/08_clash_satisfied.ein";
    let (d, fires) = describe(rel, |t| t);
    assert!(fires.is_empty(), "{rel}: nothing fires: {fires:?}");
    assert!(d.root.in_r(), "{rel}: root is alive and owes nothing");
}

/// **A derived activator acts: under `(clash-of-declared)` a declared
/// relation's pair is refuted by the rule of its count.**
///
/// `09_clash_of_declared.ein` as written is alive, and only the rules that
/// derive activators fire. With the denial of the fact it holds written in,
/// `clash-of-2`'s `(clash-2 r2)` has a pair to read, and `clash-2` fires on
/// it.
#[test]
fn a_derived_activator_fires_its_rule() {
    let rel = "tests/stdlib/contradiction/09_clash_of_declared.ein";
    let (d, fires) = describe(rel, |t| t);
    assert!(d.root.in_r(), "{rel}: as written, alive");
    assert!(
        !fires.is_empty()
            && fires
                .iter()
                .all(|f| f["rule"].as_str().unwrap().contains("clash-of-")),
        "{rel}: only the rules that derive activators fire: {fires:?}"
    );

    let (d, fires) = describe(rel, |t| t + "\n(not (r2 a b))\n");
    assert!(!d.root.alive, "{rel}: with the denial, dead");
    let clash: Vec<&serde_json::Value> = fires.iter().filter(|f| f["rule"] == "clash-2").collect();
    assert_eq!(clash.len(), 1, "{rel}: clash-2 fires once");
    assert_eq!(texts(&clash[0]["activator"]), ["r2"], "{rel}: for r2");
    assert_eq!(
        texts(&clash[0]["premises"]),
        ["(r2 a b)", "(not (r2 a b))"],
        "{rel}: on the pair"
    );
}
