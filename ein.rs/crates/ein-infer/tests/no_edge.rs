//! **An extension that closes no coupling edge can die**: the two programs
//! that break the claim that it cannot, each held to the oracle.
//!
//! The claim: on a monotone program, an alive state `S` and a candidate `b`
//! alive alone give an alive `S ∪ {b}` wherever no edge of the coupling
//! read off the whole node through a question of `b` has every one of its
//! questions met by `S ∪ {b}` (a question is met by a set that holds one of
//! its members). Two readings of the edges are held:
//! [`structure::NodeCoupling`], the one the obligation tree reads, and the
//! structure's own (`structure::coupling`), with its edges one application
//! back.
//!
//! - `examples/lattice/15_union_label_dead.ein`: a derived fact's label is
//!   the union of the classes its ways need, so a death that needs one of
//!   two candidates is read as needing both. It breaks every reading, and the
//!   claim narrowed to a candidate in no pair with a member of `S` and in no
//!   question with one.
//! - `examples/lattice/16_pair_through_closure.ein`: every label one class,
//!   and a conflict of exactly two candidates, which the tree's reading makes
//!   a pair and no edge, met through a member the closure derives and no
//!   commitment holds.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test no_edge
//! ```

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::region::{self, Cores, Outcome, Region};
use ein_infer::structure::{self, CouplingOptions, NodeCoupling, Question};
use ein_ir::{Ast, load, parse};

const UNION: &str = "examples/lattice/15_union_label_dead.ein";
const PAIR: &str = "examples/lattice/16_pair_through_closure.ein";

fn loaded(rel: &str) -> (Kb, Terms, Ast) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let kb = load(&mut ast, &mut terms, &forms, Path::new(&path).parent()).expect("loads");
    (kb, terms, ast)
}

/// The whole cube, monotone.
fn oracle(rel: &str) -> Box<Region> {
    let (mut kb, mut terms, ast) = loaded(rel);
    let opts = region::Options {
        cores: Cores::READ_OUT,
        ..region::Options::default()
    };
    let Outcome::Region(g) =
        region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}).expect("the oracle runs")
    else {
        panic!("{rel}: within the oracle's limit");
    };
    assert!(g.monotonicity.is_monotone(), "{rel} is monotone");
    g
}

/// The cube's mask of the candidates named.
fn mask(g: &Region, texts: &[&str]) -> u64 {
    texts.iter().fold(0, |m, t| {
        let i = g
            .candidates
            .iter()
            .position(|c| c.text == *t)
            .unwrap_or_else(|| panic!("{t} is a candidate"));
        m | 1 << i
    })
}

fn dead(g: &Region, m: u64) -> bool {
    g.status[m as usize] & region::DEAD != 0
}

/// `S` is an alive state, `S ∪ {b}` is dead, and `b` is alive alone: the
/// premise and the conclusion the claim denies.
fn dies(g: &Region, s: &[&str], b: &str) {
    let sm = mask(g, s);
    let bm = mask(g, &[b]);
    assert!(!dead(g, sm), "{s:?} is alive");
    assert_eq!(g.nodes[sm as usize].closure, sm, "{s:?} is a state");
    assert!(!dead(g, bm), "{b} is alive alone");
    assert!(dead(g, sm | bm), "{s:?} with {b} is dead");
}

/// A candidate of `c` by its text.
fn at(c: &NodeCoupling, t: &str) -> u32 {
    c.candidates
        .iter()
        .position(|x| x.text == t)
        .unwrap_or_else(|| panic!("{t} is a candidate")) as u32
}

/// Whether some edge of `edges` that holds a question of `b` has every
/// question met by `x`, a question's members given by `members`.
fn closes(edges: &[Vec<u32>], of_b: &[u32], members: impl Fn(u32) -> Vec<u32>, x: &[u32]) -> bool {
    edges.iter().any(|e| {
        e.iter().any(|q| of_b.contains(q))
            && e.iter().all(|&q| members(q).iter().any(|m| x.contains(m)))
    })
}

/// `S ∪ {b}` closes no edge of the tree's reading through `b`, and, where
/// `narrowed`, is in no pair with a member of `S` and in no question with
/// one.
fn closes_none_node(c: &NodeCoupling, s: &[&str], b: &str, narrowed: bool) {
    let b = at(c, b);
    let mut x: Vec<u32> = s.iter().map(|t| at(c, t)).collect();
    x.push(b);
    let members = |q: u32| c.questions[q as usize].clone();
    assert!(
        !closes(&c.edges, &c.question_of[b as usize], members, &x),
        "an edge of the tree's reading through {} is met by {x:?}: {:?}",
        c.text(b),
        c.edges
    );
    if narrowed {
        assert!(
            !c.pairs
                .iter()
                .any(|&(p, q)| (p == b && x.contains(&q)) || (q == b && x.contains(&p))),
            "no pair of {} with a member of S: {:?}",
            c.text(b),
            c.pairs
        );
        assert!(
            !c.question_of[b as usize]
                .iter()
                .any(|&q| c.questions[q as usize]
                    .iter()
                    .any(|&m| m != b && x.contains(&m))),
            "no question of {} holds a member of S",
            c.text(b)
        );
    }
}

fn node(rel: &str) -> NodeCoupling {
    let (mut kb, mut terms, ast) = loaded(rel);
    let c = structure::node_coupling(&mut kb, &mut terms, &ast, &structure::Options::default())
        .expect("the node is read");
    assert!(c.complete(), "{rel}: read whole, nothing cut");
    c
}

/// **A label that is a union reads a death as needing a question it does
/// not.** `(flag X)` comes from `(a-prop X)` alone or from `(b-prop X)` alone,
/// so its label is both questions and the conflict's edge is all four:
/// `{c, d}` is an alive state, `{a, c, d}` is dead, and no edge through `a`
/// is met by it, in the tree's reading, in the structure's, and in the
/// structure's one application back.
#[test]
fn a_union_label_reads_a_death_as_needing_a_question_it_does_not() {
    let g = oracle(UNION);
    let s = ["(c-prop X)", "(d-prop X)"];
    for b in ["(a-prop X)", "(b-prop X)"] {
        dies(&g, &s, b);
    }

    let c = node(UNION);
    assert!(
        c.node.expect("read").merged > 0,
        "a fact's label is a union of two classes"
    );
    for b in ["(a-prop X)", "(b-prop X)"] {
        closes_none_node(&c, &s, b, true);
    }

    let (mut kb, mut terms, ast) = loaded(UNION);
    let (cover, k) = structure::coupling(&mut kb, &mut terms, &ast, &CouplingOptions::default())
        .expect("the structure is read");
    assert!(k.complete(), "the structure's coupling is read whole");
    let members = |q: u32| match k.questions[q as usize] {
        Question::Group(g) => cover.groups[g as usize].clone(),
        Question::Alone(b) => vec![b],
    };
    let at = |t: &str| {
        k.candidates
            .iter()
            .position(|x| x.text == t)
            .expect("a candidate") as u32
    };
    let edges: Vec<Vec<u32>> = k.edges.iter().map(|e| e.questions.clone()).collect();
    let back: Vec<Vec<u32>> = k
        .one_step
        .as_ref()
        .expect("kept beside")
        .iter()
        .map(|e| e.questions.clone())
        .collect();
    for b in ["(a-prop X)", "(b-prop X)"] {
        let mut x: Vec<u32> = s.iter().map(|t| at(t)).collect();
        x.push(at(b));
        let of_b = &k.question_of[at(b) as usize];
        assert!(
            !closes(&edges, of_b, members, &x),
            "an edge of the structure's coupling through {b} is met: {edges:?}"
        );
        assert!(
            !closes(&back, of_b, members, &x),
            "an edge one application back through {b} is met: {back:?}"
        );
    }
}

/// **A pair read over commitments misses a member the closure brings.**
/// Every label of the whole node is one class; `{p, q}` is one rule's
/// conflict of two candidates, a pair and no edge. `{r}` is an alive state,
/// `{p, r}` is dead, since it derives `q`, and it meets no pair, no edge
/// through `p` and no question with `p` of the tree's reading.
#[test]
fn a_pair_read_over_commitments_misses_a_member_the_closure_brings() {
    let g = oracle(PAIR);
    dies(&g, &["(r-prop X)"], "(p-prop X)");
    let q = mask(&g, &["(q-prop X)"]);
    let pr = mask(&g, &["(p-prop X)", "(r-prop X)"]);
    assert_eq!(
        g.nodes[pr as usize].closure & q,
        q,
        "the node of {{p, r}} holds q"
    );

    let c = node(PAIR);
    assert_eq!(c.node.expect("read").merged, 0, "no label is a union");
    let (p, qq) = (at(&c, "(p-prop X)"), at(&c, "(q-prop X)"));
    assert!(
        c.pairs.contains(&(p.min(qq), p.max(qq))),
        "p with q is a pair: {:?}",
        c.pairs
    );
    closes_none_node(&c, &["(r-prop X)"], "(p-prop X)", true);
}
