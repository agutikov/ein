//! **A state's `open` facts** (Q-M1f.566): what it owes, `(open R)`, and its
//! hypotheses, `(open F)`, written by a pass at each state's fixpoint into a
//! part of the state no fork inherits ([`ein_infer::open`]).
//!
//! Read off the node function every route reads a state with
//! ([`ein_infer::region::Nodes`]), against the oracle's cube
//! ([`ein_infer::region::compute`]):
//!
//! - the seats program holds `(open seats)` at each node that owes and at no
//!   other, neither state of R among them, and its four candidates as
//!   `(open F)` at root;
//! - a fork drops every `open` fact, a written one among them, and the
//!   empty commitment's state is root's, which holds the written ones;
//! - $H$ is the union of the writers (Q-M1f.570), and the blind enumerator
//!   writes only where nothing else does (Q-M1f.576);
//! - a rule that reads an `open` fact makes its program not monotone
//!   (Q-M1f.569), and so does a hypothesis first held above root, found by
//!   the static check and in flight (Q-M1f.574).

use std::path::PathBuf;

use ein_core::{FactId, Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::region::{self, Nodes, OWING, Options, Outcome};
use ein_ir::{Ast, load, load_file, parse};

fn load_rel(rel: &str) -> (Ast, Terms, Kb) {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    (ast, terms, kb)
}

fn load_text(text: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<test>")).expect("parses");
    let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
    (ast, terms, kb)
}

fn refused(text: &str) -> String {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    match parse(&mut ast, text, Some("<test>")) {
        Err(e) => e.to_string(),
        Ok(forms) => match load(&mut ast, &mut terms, &forms, None) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("loads: {text}"),
        },
    }
}

/// The oracle's cube of `kb`: its candidates and every node's status.
fn cube(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> region::Region {
    let opts = Options::default();
    match region::compute(kb, terms, ast, &opts, &mut |_| {}).expect("computes") {
        Outcome::Region(r) => *r,
        _ => panic!("the cube is within the limit"),
    }
}

/// Each node's `open` facts, as text, sorted: `(open seats)`, `(open (p a))`.
fn open_at(ast: &Ast, terms: &mut Terms, kb: &mut Kb, sets: &[Vec<FactId>]) -> Vec<Vec<String>> {
    let cands: Vec<region::Candidate> = Vec::new();
    let mut nodes = Nodes::new(kb, terms, ast, &cands).expect("prepares");
    nodes
        .with_fixpoints(sets, |kbs, terms| {
            kbs.iter()
                .map(|k| {
                    let mut v: Vec<String> =
                        k.open_facts().iter().map(|&f| terms.fact_text(f)).collect();
                    v.sort();
                    v
                })
                .collect()
        })
        .expect("every fixpoint")
}

#[test]
fn the_seats_hold_what_they_owe_where_they_owe_it() {
    let rel = "examples/lattice/05_owed_seats_bounded.ein";
    let (ast, mut terms, mut kb) = load_rel(rel);
    let r = cube(&ast, &mut terms, &mut kb);
    let n = r.candidates.len();
    assert_eq!(n, 4, "the four seats");
    let sets: Vec<Vec<FactId>> = (0..1u64 << n)
        .map(|m| {
            (0..n)
                .filter(|i| m >> i & 1 == 1)
                .map(|i| r.candidates[i].fact)
                .collect()
        })
        .collect();
    let (ast, mut terms, mut kb) = load_rel(rel);
    let open = open_at(&ast, &mut terms, &mut kb, &sets);
    let mut owing = 0;
    for (m, facts) in open.iter().enumerate() {
        let owes = r.status[m] & OWING != 0;
        let held = facts.iter().any(|f| f == "(open seats)");
        assert_eq!(
            held, owes,
            "node {m}: (open seats) is held where the node owes: {facts:?}"
        );
        if r.in_r(m as u64) {
            assert!(
                facts.is_empty(),
                "a state of R holds no open fact: {facts:?}"
            );
        }
        owing += usize::from(owes);
    }
    assert_eq!(owing, 9, "the nine nodes that owe");
    assert_eq!(
        open[0],
        [
            "(open (seats Ann C1))",
            "(open (seats Ann C2))",
            "(open (seats Bob C1))",
            "(open (seats Bob C2))",
            "(open seats)",
        ],
        "root holds what it owes and its four candidates"
    );
}

#[test]
fn a_fork_drops_every_open_fact_a_written_one_among_them() {
    let text = "(relation p T :why \"{?3} is a p\")\n(relation q T :why \"{?3} is a q\")\n\
                (thing a) (thing b)\n(open (p a))\n\
                (rule ask () :match (thing ?x) :assert (open (q ?x)) :why \"is {?x} a q?\")\n";
    let (ast, mut terms, mut kb) = load_text(text);
    let r = cube(&ast, &mut terms, &mut kb);
    let q_a = r
        .candidates
        .iter()
        .find(|c| c.text == "(q a)")
        .expect("(q a) is a candidate")
        .fact;
    let mut at_root: Vec<String> = kb
        .open_facts()
        .iter()
        .map(|&f| terms.fact_text(f))
        .collect();
    at_root.sort();
    assert_eq!(
        at_root,
        ["(open (p a))", "(open (q a))", "(open (q b))"],
        "root holds the program's hypothesis and the rule's"
    );
    let open = open_at(&ast, &mut terms, &mut kb, &[vec![], vec![q_a]]);
    assert_eq!(
        open[0],
        ["(open (p a))", "(open (q a))", "(open (q b))"],
        "the empty commitment's state is root's: it holds the program's hypothesis and the rule's"
    );
    assert_eq!(
        open[1],
        ["(open (q a))", "(open (q b))"],
        "a state above root holds what the rule concludes there, (q a) among it though the state \
         holds (q a), and not the program's"
    );
}

/// **The empty commitment's state is root's**: the node every route builds
/// for ∅ holds every fact root holds, the `open` facts the program wrote
/// among them, each with its source, and nothing root does not; a node
/// above it holds none of the written ones. Here the program writes its two
/// hypotheses and a rule reads them, so the program is flagged: what is held
/// is the state the node is, fact for fact, and not what the program
/// answers.
#[test]
fn the_empty_commitments_state_is_roots() {
    let text = "(relation s T :why \"s holds of {?3}\")\n\
                (relation seen F :why \"{?3} is supposed\")\n\
                (thing a) (thing b)\n(open (s a)) (open (s b))\n\
                (rule noted () :match (open ?f) :assert (seen ?f) :why \"{?f} is supposed\")\n";
    let (ast, mut terms, mut kb) = load_text(text);
    let r = cube(&ast, &mut terms, &mut kb);
    let s_a = r
        .candidates
        .iter()
        .find(|c| c.text == "(s a)")
        .expect("(s a) is a candidate")
        .fact;
    let cands: Vec<region::Candidate> = Vec::new();
    let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &cands).expect("prepares");
    let facts = |k: &Kb, terms: &Terms| -> Vec<String> {
        let mut v: Vec<String> = k.facts().map(|f| terms.fact_text(f)).collect();
        v.sort();
        v
    };
    let (empty, above, sources) = nodes
        .with_fixpoints(&[vec![], vec![s_a]], |kbs, terms| {
            let sources: Vec<bool> = kbs[0]
                .open_facts()
                .iter()
                .filter(|&&f| terms.fact_text(f).starts_with("(open (s "))
                .map(|&f| {
                    kbs[0].primary(f).is_some_and(|p| {
                        terms.provs.get(p).kind == ein_core::prov::ProvKind::Source
                    })
                })
                .collect();
            (facts(&kbs[0], terms), facts(&kbs[1], terms), sources)
        })
        .expect("every fixpoint");
    drop(nodes);
    assert_eq!(empty, facts(&kb, &terms), "∅'s node is root's state");
    assert_eq!(
        sources,
        [true, true],
        "each written `open` fact keeps its source"
    );
    assert!(
        !above
            .iter()
            .any(|f| f == "(open (s a))" || f == "(open (s b))"),
        "a node above ∅ holds none of the written ones: {above:?}"
    );
}

#[test]
fn h_is_the_union_of_the_writers() {
    let rel = "examples/lattice/14_seats_nothing_proposes.ein";
    let (ast, mut terms, mut kb) = load_rel(rel);
    let r = cube(&ast, &mut terms, &mut kb);
    assert_eq!(
        r.candidates.len(),
        16,
        "the rule's twelve seats and the four the obligation proposes beside them"
    );
}

#[test]
fn the_blind_enumerator_writes_only_where_nothing_else_does() {
    let blind = "(relation p T :why \"{?3} is a p\")\n(thing a)\n";
    let (ast, mut terms, mut kb) = load_text(blind);
    let r = cube(&ast, &mut terms, &mut kb);
    assert_eq!(
        r.candidates
            .iter()
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>(),
        ["(p a)"],
        "nothing else writes: the blind enumerator's, over `a`, `thing` being a relation"
    );
    let ruled = format!(
        "{blind}(relation q T :why \"{{?3}} is a q\")\n\
         (rule ask () :match (thing ?x) :assert (open (q ?x)) :why \"is {{?x}} a q?\")\n"
    );
    let (ast, mut terms, mut kb) = load_text(&ruled);
    let r = cube(&ast, &mut terms, &mut kb);
    assert_eq!(
        r.candidates
            .iter()
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>(),
        ["(q a)"],
        "a rule writes, and the blind enumerator does not"
    );
}

#[test]
fn a_rule_that_reads_open_is_flagged() {
    let text = "(relation p T :why \"{?3} is a p\")\n(thing a)\n\
                (rule ask () :match (thing ?x) :assert (open (p ?x)) :why \"is {?x} a p?\")\n\
                (rule asked () :match (open (p ?x)) :assert (asked ?x) :why \"{?x} is asked\")\n";
    let (ast, mut terms, mut kb) = load_text(text);
    let r = cube(&ast, &mut terms, &mut kb);
    assert!(!r.monotonicity.is_monotone());
    assert!(
        r.monotonicity
            .sites
            .iter()
            .any(|s| s.render(&terms) == "asked: open-read reads open at absent depth 0"),
        "{:?}",
        r.monotonicity.sites
    );
}

#[test]
fn a_hypothesis_first_held_above_root_is_found() {
    let text = "(relation chosen D :why \"{?3} is chosen\")\n(relation with D E :why \"{?3} comes with {?4}\")\n\
                (is-a Tea Drink) (is-a Sugar Extra)\n\
                (rule ask-drink () :match (is-a ?d Drink) :assert (open (chosen ?d)) :why \"is {?d} chosen?\")\n\
                (rule ask-extra () :match (and (chosen ?d) (is-a ?e Extra)) :assert (open (with ?d ?e)) :why \"{?d} with {?e}?\")\n";
    let (ast, mut terms, mut kb) = load_text(text);
    // The check in flight is made where the pass runs: at every state under
    // `--in-flight` (Q-M1f.656), and by default at none of the cube's nodes,
    // which nothing prints and whose rules read no `open` fact (Q-M1f.655).
    let r = ein_infer::open::at_every_state(true, || cube(&ast, &mut terms, &mut kb));
    assert!(
        r.monotonicity.sites.iter().any(|s| s
            .render(&terms)
            .starts_with("ask-extra: hypothesis reads chosen")),
        "the static check: {:?}",
        r.monotonicity.sites
    );
    assert_eq!(
        r.monotonicity.in_flight.as_deref(),
        Some("(with Tea Sugar)"),
        "and in flight"
    );
    let (ast, mut terms, mut kb) = load_text(text);
    let r = cube(&ast, &mut terms, &mut kb);
    assert_eq!(
        r.monotonicity.in_flight, None,
        "by default no state of the cube writes its `open` facts, and none is checked"
    );
    assert!(
        !r.monotonicity.sites.is_empty(),
        "the static check flags it either way"
    );
}

/// **The pass runs where a state is used** (Q-M1f.655): a node of the cube
/// that nothing prints writes no `open` fact, and its status is the one it
/// has with the pass at every state (Q-M1f.656). The state itself, read
/// through the node function every printer reads, holds them either way
/// ([`the_seats_hold_what_they_owe_where_they_owe_it`]).
#[test]
fn the_pass_runs_where_a_state_is_used() {
    let rel = "examples/lattice/05_owed_seats_bounded.ein";
    let (ast, mut terms, mut kb) = load_rel(rel);
    let by_default = cube(&ast, &mut terms, &mut kb);
    let (ast, mut terms, mut kb) = load_rel(rel);
    let everywhere = ein_infer::open::at_every_state(true, || cube(&ast, &mut terms, &mut kb));
    assert_eq!(
        by_default.status, everywhere.status,
        "{rel}: every node's status is the same with the pass at every state and without"
    );
    assert_eq!(by_default.candidates.len(), everywhere.candidates.len());
}

#[test]
fn the_kernels_words_nest_as_ruled() {
    let pre = "(relation p T :why \"{?3} is a p\")\n(p a)\n";
    for (shape, text) in [
        ("(open)", format!("{pre}(open)")),
        ("(open (open P))", format!("{pre}(open (open (p a)))")),
        ("(open (absent P))", format!("{pre}(open (absent (p a)))")),
        (
            "(absent (absent P))",
            format!(
                "{pre}(rule r () :match (and (p ?x) (absent (absent (p ?x)))) :assert (q ?x) :why \"r\")"
            ),
        ),
    ] {
        let e = refused(&text);
        assert!(
            e.contains("unexpected"),
            "{shape} is refused at the parse: {e}"
        );
    }
    for text in [
        format!("{pre}(think a (open (p a)))"),
        format!("{pre}(think a (absent (p a)))"),
        format!("{pre}(not (open (p a)))"),
        format!("{pre}(open (not (p b)))"),
        format!(
            "{pre}(rule r () :match (and (p ?x) (absent (open (p ?x)))) :assert (q ?x) :why \"r\")"
        ),
    ] {
        load_text(&text);
    }
}
