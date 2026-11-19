//! **`std.debts`' requirements with no payment are what the kernel owes, at
//! every node, and `(debts-discharged)` is held exactly where nothing is
//! owed.**
//!
//! An obligation's verdict is no fact, so no rule reads what a state owes
//! ([08-obligations.md](../../../../docs/ein/user/03-syntax-and-semantics/08-obligations.md)).
//! [`std.debts`](../../../../stdlib/debts.ein) states the two halves of a debt
//! that can be facts, `(debt-due R J)` from an obligation's trigger and
//! `(debt-paid R J)` from the trigger with its witness, for the library's
//! four obligations, and `(debts-discharged)` where no requirement lacks its
//! payment. The claim that makes those facts worth reading is an invariance
//! between two readings of one state:
//!
//! ```text
//! the kernel's pass over the fixpoint      owes k instances
//! the module's rules, in the fixpoint      k requirements have no payment
//! ```
//!
//! and `(debts-discharged)` is held where `k = 0` and nowhere else. A
//! fixture's claim cannot state it: a claim is made of facts, the kernel's
//! side is none, and most of the nodes it is about are outside R. So it is
//! held here, in-process, by the node function every route shares
//! ([`ein_infer::region::Nodes`]): each node is root's fork with a
//! commitment written, saturated to quiescence, and both readings are taken
//! of that one fixpoint.
//!
//! Three sets of nodes: the whole cube of a small search, every node a
//! description of `examples/zebra2-obligations.ein` enters, and the roots of
//! the eight fixtures that hold the four obligations to their counts
//! ([`obligation_reports.rs`](obligation_reports.rs)).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ein_core::{FactId, Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::SharedMemo;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::region::{Candidate, Nodes};
use ein_ir::{Ast, load_query, parse};

/// What asks the module for everything it states.
const ASKED: &str = "\n(import std.debts)\n(debts-of-owed)\n";

/// One node, read both ways.
#[derive(Debug)]
struct Read {
    /// The kernel's: the obligation instances the fixpoint leaves owed.
    owes: usize,
    /// The module's: the requirements, and those of them with a payment.
    due: usize,
    paid: usize,
    discharged: bool,
}

impl Read {
    fn unpaid(&self) -> usize {
        self.due - self.paid
    }
}

/// The instances a fixpoint holds under `relation`: each fact's text less
/// its relation, so a requirement and its payment read alike.
fn instances(kb: &Kb, terms: &Terms, relation: &str) -> BTreeSet<String> {
    let Some(sym) = terms.syms.get(relation) else {
        return BTreeSet::new();
    };
    kb.facts_of(sym)
        .map(|f| {
            let text = terms.fact_text(f);
            text.strip_prefix(&format!("({relation} "))
                .unwrap_or_else(|| panic!("{text}: a fact of {relation}"))
                .to_string()
        })
        .collect()
}

fn read(kb: &Kb, terms: &mut Terms, ast: &Ast) -> Read {
    let owes =
        ein_infer::obligations::tally(kb, terms, ast, &SharedMemo::default(), &mut Events::off())
            .expect("the obligations compile")
            .total();
    let due = instances(kb, terms, "debt-due");
    let paid = instances(kb, terms, "debt-paid");
    assert!(
        paid.is_subset(&due),
        "a payment is of a requirement: {:?}",
        paid.difference(&due).collect::<Vec<_>>()
    );
    let discharged = terms
        .syms
        .get("debts-discharged")
        .is_some_and(|s| kb.facts_of(s).next().is_some());
    Read {
        owes,
        due: due.len(),
        paid: paid.len(),
        discharged,
    }
}

/// `rel`'s text with `more` after it, loaded; described; and then each of
/// the commitments `which` picks read both ways at its fixpoint.
///
/// `which` is given the candidates and the commitments the description
/// entered, each as candidate texts, and returns the commitments to read.
fn reads(
    rel: &str,
    more: &str,
    which: impl Fn(&[Candidate], Vec<Vec<String>>) -> Vec<Vec<String>>,
) -> Vec<(Vec<String>, Read)> {
    let path: PathBuf = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}")) + more;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut kb = load_query(&mut ast, &mut terms, &forms, path.parent(), 0)
        .unwrap_or_else(|e| panic!("{rel}: {e}"));

    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
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
    let described = answer
        .described
        .unwrap_or_else(|| panic!("{rel}: describe describes"));
    assert!(described.exhausted, "{rel}: R is not known");
    assert!(
        described.monotonicity.is_monotone(),
        "{rel}: the static check calls the program monotone with the module asked for"
    );
    let entered: Vec<Vec<String>> = buffer
        .to_string_lossy()
        .lines()
        .map(|l| serde_json::from_str::<serde_json::Value>(l).expect("an event is a JSON line"))
        .filter(|e| e["e"] == "enter")
        .map(|e| {
            e["commitment"]
                .as_array()
                .expect("an enter line lists its commitment")
                .iter()
                .map(|f| f.as_str().expect("a fact's text").to_string())
                .collect()
        })
        .collect();

    let candidates = described.candidates.clone();
    let by_text: BTreeMap<&str, FactId> = candidates
        .iter()
        .map(|c| (c.text.as_str(), c.fact))
        .collect();
    let sets = which(&candidates, entered);
    let mut nodes =
        Nodes::new(&mut kb, &mut terms, &ast, &candidates).unwrap_or_else(|e| panic!("{rel}: {e}"));
    sets.into_iter()
        .map(|set| {
            let facts: Vec<FactId> = set
                .iter()
                .map(|t| {
                    *by_text
                        .get(t.as_str())
                        .unwrap_or_else(|| panic!("{rel}: {t} is no candidate"))
                })
                .collect();
            let r = nodes
                .with_fixpoint(&facts, |kb, terms| read(kb, terms, &ast))
                .unwrap_or_else(|e| panic!("{rel}: {e}"));
            (set, r)
        })
        .collect()
}

/// Both readings agree at every node of `nodes`.
fn hold(rel: &str, nodes: &[(Vec<String>, Read)]) {
    for (set, r) in nodes {
        assert_eq!(
            r.unpaid(),
            r.owes,
            "{rel}: at {{{}}} the kernel owes {} and {} of {} requirements have no payment",
            set.join(", "),
            r.owes,
            r.unpaid(),
            r.due,
        );
        assert_eq!(
            r.discharged,
            r.owes == 0,
            "{rel}: at {{{}}}, which owes {}, (debts-discharged) is {}",
            set.join(", "),
            r.owes,
            if r.discharged { "held" } else { "not held" },
        );
    }
}

/// **The whole cube of a search: sixteen nodes, alive and dead, each read
/// both ways.**
///
/// `tests/stdlib/debts/10_discharged_over_r.ein` seats two guests at two
/// chairs under `total-owed` and `surjective-owed`, with `functional` and
/// `injective` beside them, so its four candidates' sixteen subsets hold
/// every kind of node: owing, in R, dead and owing, dead and discharged. The
/// requirements are root's, four, and a commitment adds payments.
#[test]
fn at_every_node_of_a_cube_the_unpaid_requirements_are_what_the_node_owes() {
    let rel = "tests/stdlib/debts/10_discharged_over_r.ein";
    let nodes = reads(rel, "", |candidates, _| {
        assert_eq!(candidates.len(), 4, "{rel}: four seatings to choose among");
        (0..1u32 << candidates.len())
            .map(|mask| {
                candidates
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask >> i & 1 == 1)
                    .map(|(_, c)| c.text.clone())
                    .collect()
            })
            .collect()
    });
    assert_eq!(nodes.len(), 16);
    hold(rel, &nodes);
    // The claim is not vacuous: every node requires the same four, and the
    // nodes owe four, two, one and nothing.
    assert!(nodes.iter().all(|(_, r)| r.due == 4), "{rel}: {nodes:?}");
    let owes: BTreeSet<usize> = nodes.iter().map(|(_, r)| r.owes).collect();
    assert_eq!(owes, BTreeSet::from([0, 1, 2, 4]), "{rel}");
    assert_eq!(
        nodes.iter().filter(|(_, r)| r.discharged).count(),
        7,
        "{rel}: the two matchings and the five dead sets above them"
    );
}

/// **Every node a description enters, on a puzzle: the five bijections of
/// `examples/zebra2-obligations.ein`.**
///
/// The program with the module imported and asked for. Its obligations are
/// derived, `(bijective R)` fanned out by `std.bijection`, so the module's
/// activators are derived from derived activators.
#[test]
fn at_every_node_a_description_enters_the_unpaid_requirements_are_what_it_owes() {
    let rel = "examples/zebra2-obligations.ein";
    let nodes = reads(rel, ASKED, |_, entered| entered);
    hold(rel, &nodes);
    let root = nodes
        .iter()
        .find(|(set, _)| set.is_empty())
        .unwrap_or_else(|| panic!("{rel}: the description enters root"));
    assert!(
        root.1.owes > 0 && root.1.paid > 0,
        "{rel}: root owes, and some of what it requires is paid: {:?}",
        root.1
    );
    assert!(
        nodes.iter().any(|(_, r)| r.owes == 0) && nodes.len() > 1,
        "{rel}: the description enters nodes that owe nothing"
    );
}

/// **The roots of the four obligations' fixtures: each owes what its fixture
/// states, and that many requirements have no payment.**
///
/// The eight programs `obligation_reports.rs` holds to their counts, each
/// with the module imported and asked for.
#[test]
fn each_obligations_fixture_requires_and_pays_what_it_owes() {
    let cases: [(&str, usize); 8] = [
        ("tests/stdlib/algebra/23_total_owed.ein", 1),
        ("tests/stdlib/algebra/24_total_owed_satisfied.ein", 0),
        ("tests/stdlib/algebra/25_surjective_owed.ein", 1),
        ("tests/stdlib/algebra/26_surjective_owed_satisfied.ein", 0),
        ("tests/stdlib/slots/09_owed_room.ein", 3),
        ("tests/stdlib/slots/10_owed_room_satisfied.ein", 0),
        ("tests/stdlib/slots/11_owed_fill.ein", 3),
        ("tests/stdlib/slots/12_owed_fill_satisfied.ein", 0),
    ];
    for (rel, want) in cases {
        let nodes = reads(rel, ASKED, |_, _| vec![Vec::new()]);
        hold(rel, &nodes);
        let root = &nodes[0].1;
        assert_eq!(root.owes, want, "{rel}: what root owes");
        assert!(root.due > 0, "{rel}: the obligation requires something");
    }
}

/// **A state that owes holds its debt's two halves: `09_debts_owing.ein`
/// owes Ann a food, and Ann's is the requirement with no payment.**
#[test]
fn an_owing_root_holds_the_requirement_it_has_not_paid() {
    let rel = "tests/stdlib/debts/09_debts_owing.ein";
    let nodes = reads(rel, "", |_, _| vec![Vec::new()]);
    hold(rel, &nodes);
    let root = &nodes[0].1;
    assert_eq!(
        (root.owes, root.due, root.paid, root.discharged),
        (1, 2, 1, false),
        "{rel}"
    );
}
