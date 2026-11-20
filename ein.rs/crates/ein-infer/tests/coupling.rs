//! **The coupling is held to `N` and to `R`**: what ties the questions of a
//! program's cover together, against what a describe that exhausts says.
//!
//! [`ein_infer::structure::coupling`] reads the questions' hypergraph from
//! root's pair layer and from one node, every candidate alive alone
//! committed. A describe that exhausts states `N`, the minimal dead sets,
//! and every state of `R` with its generators, and shares no code with the
//! coupling past the node function. Two things must then be true:
//!
//! 1. **every member of `N` lies inside one edge**: an edge holds a question
//!    of each of its candidates. A member of two candidates may instead be
//!    two members of one question, a pair the theory excludes;
//! 2. **`R` is the product of its parts over the coupling's components**:
//!    for two commitments of `R` and a component, the one's candidates
//!    inside it with the other's outside it are a commitment of `R`. It is
//!    read on the states' closures and on their generators, which decide it
//!    for every commitment where dead is upward-closed and discharged is
//!    too. The components are over every edge, the debts' among them
//!    ([`ein_infer::structure::Debt`]): an instance root owes ties its
//!    payers in `R` as a conflict does. **Over the conflicts' and the
//!    derivations' components alone `R` is no product only where a debt's
//!    payers lie in two of them**, which the test holds too, and counts.
//!    A debt is an edge whether or not L5 licenses its instance: an
//!    unlicensed one's payers are those a single candidate's node shows.
//!
//! The converse of neither is claimed: an edge may be of a derivation that
//! never ends in a death.
//!
//! On every corpus file and regression witness the loader accepts, the static
//! check calls monotone and whose describe exhausts at the default
//! `--region-budget`, the coupling read at the same budget. The corpus's
//! `slow` entries are read under `EIN_CORPUS_SLOW=1`, as the other corpus
//! sweeps read them, but for the instances named here.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test coupling
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test coupling -- the_coupling_is_held_to_n_and_to_r
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::region::Cores;
use ein_infer::structure::{
    self, Coupling, CouplingOptions, CouplingReading, Cover, NodeStop, Question,
};
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn rel_of(p: &Path) -> String {
    p.strip_prefix(repo_root())
        .unwrap_or(p)
        .display()
        .to_string()
}

const L02: &str = "examples/lattice/02_genuine_3set_death.ein";
const SEND: &str = "examples/beyond_arith/send_more_money.ein";
const Z2: &str = "examples/zebra2.ein";

/// The instances every run reads, `slow` or not.
const INSTANCES: [&str; 3] = ["examples/zebra.ein", Z2, L02];

/// Every corpus file and regression witness, once; the `slow` entries that are
/// not [`INSTANCES`] where `slow_too` only.
fn files(slow_too: bool) -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !INSTANCES.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::regressions().case {
        files.push(repo_root().join(&row.witness));
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    files
}

fn read(rel: &str, reading: CouplingReading) -> (Cover, Coupling) {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
    let opts = CouplingOptions {
        reading,
        ..CouplingOptions::default()
    };
    structure::coupling(&mut kb, &mut terms, &ast, &opts).expect("the coupling")
}

// ── Held to N and to R ────────────────────────────────────────────────

/// What one program's comparison came to.
#[derive(Default)]
struct Held {
    failures: Vec<String>,
    /// The coupling and an exhausted describe were both read.
    compared: bool,
    /// The static check flags the program: nothing is compared.
    flagged: bool,
    /// Why the coupling was not whole, where it was not.
    unread: Option<String>,
    /// Members of `N` of two candidates, and of three or more.
    pairs: usize,
    three: usize,
    /// Of those of three or more, how many hold two candidates that no
    /// edge the pair layer alone gives holds a question of each of.
    past_the_pairs: usize,
    edges: usize,
    components: usize,
    /// The product was read: the conflicts and the derivations leave two
    /// components or more.
    factored: bool,
    /// The components over every edge are two or more.
    several: bool,
    /// The debts that are edges, and those of them whose payers lie in two
    /// of the conflicts' components.
    debts: usize,
    across: usize,
    /// Over the conflicts' components alone `R` is no product.
    finer_without_debts: bool,
    /// The debts that are edges whose instance L5 does not license.
    unlicensed: usize,
}

/// A set of candidates, a bit each.
fn bits(n: usize, set: &[u32]) -> Vec<u64> {
    let mut out = vec![0u64; n.div_ceil(64)];
    for &b in set {
        out[b as usize / 64] |= 1 << (b % 64);
    }
    out
}

fn inside(a: &[u64], b: &[u64]) -> bool {
    a.iter().zip(b).all(|(x, y)| x & !y == 0)
}

/// A part `R` is not the product over, and the mix that shows it.
type Finer = (Vec<u32>, Vec<u32>);

/// The most mixes one program is read on.
const MIXES: usize = 1 << 22;

/// Both claims for `path`; `None` where the program does not load, is
/// refused, or its describe does not exhaust.
fn held(path: &Path) -> Option<Held> {
    let rel = rel_of(path);
    let (mut kb, mut terms, ast) = loaded(path)?;
    // N is read off describe by the state route, which computes it where
    // the tree and the classes, describe's default, do not.
    let opts = explore::Options {
        cores: Cores::READ_OUT,
        describe_route: explore::DescribeRoute::States,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let answer = explore::run(&mut kb, &mut terms, &ast, &opts).ok()?;
    let d = answer.described.as_deref()?;
    if !d.exhausted || d.nogoods_skipped {
        return None;
    }
    drop((kb, terms));
    let mut out = Held::default();
    if !answer.monotonicity.is_monotone() {
        out.flagged = true;
        return Some(out);
    }
    let (mut kb, mut terms, ast) = loaded(path)?;
    let c = match structure::coupling(&mut kb, &mut terms, &ast, &CouplingOptions::default()) {
        Ok((_, c)) => c,
        Err(e) => {
            out.failures.push(format!(
                "{rel}: a describe answers and the coupling does not: {e}"
            ));
            return Some(out);
        }
    };
    if c.n() != d.n()
        || c.candidates
            .iter()
            .zip(&d.candidates)
            .any(|(a, b)| a.text != b.text)
    {
        out.failures.push(format!(
            "{rel}: the coupling and the describe read two universes"
        ));
        return Some(out);
    }
    // A dead root and a universe with no candidate alive alone have no node
    // to read and no member of N past one candidate.
    let nothing_to_read = matches!(c.node_stopped, Some(NodeStop::RootDead | NodeStop::Empty));
    if !c.pairs_read || c.cut.unsettled || (c.node_stopped.is_some() && !nothing_to_read) {
        out.unread = Some(format!(
            "pairs read: {}, node: {:?}, cut: {:?}",
            c.pairs_read, c.node_stopped, c.cut
        ));
        return Some(out);
    }
    out.compared = true;
    out.edges = c.edges.len();
    out.components = c.components.len();
    let names = |set: &[u32]| -> Vec<&str> { set.iter().map(|&b| c.text(b)).collect() };

    // 1. Every member of N lies inside one edge.
    let of_pairs: Vec<usize> = (0..c.edges.len())
        .filter(|&e| c.edges[e].pairs.all() > 0)
        .collect();
    let inside_a_pair_edge = |set: &[u32]| {
        of_pairs.iter().any(|&e| {
            set.iter().all(|&b| {
                c.question_of[b as usize]
                    .iter()
                    .any(|q| c.edges[e].questions.binary_search(q).is_ok())
            })
        })
    };
    for dead in &d.dead {
        match dead.set.len() {
            0 | 1 => {}
            2 => {
                out.pairs += 1;
                if !c.in_one_question(&dead.set) && c.edge_over(&dead.set).is_none() {
                    out.failures.push(format!(
                        "{rel}: {:?} is a minimal dead pair, in no question together and inside \
                         no edge",
                        names(&dead.set)
                    ));
                }
            }
            _ => {
                out.three += 1;
                // Some two of its members the pair layer ties by no edge.
                let apart = dead.set.iter().enumerate().any(|(k, &a)| {
                    dead.set[k + 1..]
                        .iter()
                        .any(|&b| !inside_a_pair_edge(&[a, b]))
                });
                out.past_the_pairs += usize::from(apart);
                if c.edge_over(&dead.set).is_none() {
                    out.failures.push(format!(
                        "{rel}: {:?} is a minimal dead set and no edge holds a question of each \
                         of its members",
                        names(&dead.set)
                    ));
                }
            }
        }
    }

    // 2. R is the product of its parts over the components.
    out.debts = c.debts.len();
    out.across = c.debts_across().len();
    out.unlicensed = c.debts.iter().filter(|d| !d.licensed).count();
    if c.conflict_components.len() < 2 {
        return Some(out);
    }
    let n = c.n();
    let states: Vec<(Vec<u64>, Vec<Vec<u64>>)> = d
        .states
        .iter()
        .map(|s| {
            (
                bits(n, &s.closure),
                s.generators.iter().map(|g| bits(n, g)).collect(),
            )
        })
        .collect();
    // A closure and a generator are commitments of R as they are; any
    // other set is one where it lies between a state's generator and its
    // closure.
    let known: std::collections::HashSet<&Vec<u64>> = states
        .iter()
        .flat_map(|s| std::iter::once(&s.0).chain(s.1.iter()))
        .collect();
    let in_r = |x: &Vec<u64>| {
        known.contains(x)
            || states.iter().any(|(closure, generators)| {
                inside(x, closure) && generators.iter().any(|g| inside(g, x))
            })
    };
    let all: Vec<&Vec<u64>> = states.iter().flat_map(|s| s.1.iter()).collect();
    let tops: Vec<&Vec<u64>> = states.iter().map(|s| &s.0).collect();
    // The first part `R` is not the product over, with the mix that shows
    // it: every part of a closure inside the part with every part of a
    // closure outside it, and the same of the generators. `Err` where that
    // is more mixes than the test reads.
    let finer = |parts: &[Vec<u32>]| -> Result<Option<Finer>, usize> {
        let mut read = 0usize;
        for part in parts {
            let k = bits(n, part);
            for side in [&tops, &all] {
                let cut = |keep: bool| -> BTreeSet<Vec<u64>> {
                    side.iter()
                        .map(|a| {
                            (0..k.len())
                                .map(|w| if keep { a[w] & k[w] } else { a[w] & !k[w] })
                                .collect()
                        })
                        .collect()
                };
                let (within, without) = (cut(true), cut(false));
                read += within.len() * without.len();
                if read > MIXES {
                    return Err(read);
                }
                for a in &within {
                    for b in &without {
                        let mix: Vec<u64> = a.iter().zip(b).map(|(x, y)| x | y).collect();
                        if !in_r(&mix) {
                            let set: Vec<u32> = (0..n as u32)
                                .filter(|&x| mix[x as usize / 64] >> (x % 64) & 1 == 1)
                                .collect();
                            return Ok(Some((part.clone(), set)));
                        }
                    }
                }
            }
        }
        Ok(None)
    };
    out.several = c.components.len() >= 2;
    match finer(&c.components) {
        Ok(None) => out.factored = true,
        Ok(Some((part, set))) => out.failures.push(format!(
            "{rel}: the component {:?} is finer than R: {:?} is one commitment of R inside it \
             and another outside, and is not in R",
            names(&part),
            names(&set)
        )),
        Err(mixes) => {
            out.unread = Some(format!(
                "{} states over {} components are {mixes} mixes or more, past what the test reads",
                states.len(),
                c.components.len()
            ));
        }
    }
    // And over the conflicts' components alone, but where a debt's payers
    // lie in two of them.
    match finer(&c.conflict_components) {
        Ok(None) => {}
        Ok(Some((part, set))) => {
            out.finer_without_debts = true;
            if out.across == 0 {
                out.failures.push(format!(
                    "{rel}: over the conflicts' components R is no product at {:?}, by {:?}, and \
                     no debt's payers lie in two of them",
                    names(&part),
                    names(&set)
                ));
            }
        }
        Err(mixes) => {
            out.unread = Some(format!(
                "{} states over {} components of conflicts are {mixes} mixes or more, past what \
                 the test reads",
                states.len(),
                c.conflict_components.len()
            ));
        }
    }
    Some(out)
}

/// **The coupling is held to `N` and to `R`** on every corpus program
/// whose describe exhausts at the default budget.
#[test]
fn the_coupling_is_held_to_n_and_to_r() {
    // The corpus's `slow` entries too, where the run asks for them.
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    let results = ein_corpus::pool::map(
        "coupling::held",
        &files,
        |p| (rel_of(p), String::new()),
        |p| held(p),
    );
    let mut failures: Vec<String> = Vec::new();
    let mut unread: Vec<String> = Vec::new();
    let (mut compared, mut flagged, mut pairs, mut three, mut with_three) = (0, 0, 0, 0, 0);
    let (mut past, mut with_past, mut factored, mut edges) = (0, 0, 0, 0);
    let (mut debts, mut with_debts, mut across, mut several) = (0, 0, 0, 0);
    let mut finer: Vec<String> = Vec::new();
    let mut joined: Vec<String> = Vec::new();
    let (mut unlicensed, mut with_unlicensed) = (0, 0);
    for (file, got) in files.iter().zip(results) {
        let Some(h) = got else { continue };
        failures.extend(h.failures);
        compared += usize::from(h.compared);
        flagged += usize::from(h.flagged);
        pairs += h.pairs;
        three += h.three;
        with_three += usize::from(h.three > 0);
        past += h.past_the_pairs;
        with_past += usize::from(h.past_the_pairs > 0);
        factored += usize::from(h.factored);
        edges += h.edges;
        debts += h.debts;
        with_debts += usize::from(h.debts > 0);
        across += h.across;
        several += usize::from(h.several);
        if h.across > 0 {
            joined.push(rel_of(file));
        }
        if h.finer_without_debts {
            finer.push(rel_of(file));
        }
        unlicensed += h.unlicensed;
        with_unlicensed += usize::from(h.unlicensed > 0);
        if let Some(why) = h.unread {
            unread.push(format!("{}: {why}", rel_of(file)));
        }
    }
    assert!(
        failures.is_empty(),
        "{} disagreement(s) between the coupling and a describe:\n{}",
        failures.len(),
        failures.join("\n")
    );
    // A describe exhausted at the default budget and the coupling was not
    // read whole: named, and none is expected.
    assert!(
        unread.is_empty(),
        "the coupling was not read whole where a describe exhausts:\n{}",
        unread.join("\n")
    );
    assert!(compared > 100, "only {compared} programs compared");
    assert!(with_three > 0 && factored > 0, "nothing was held");
    println!(
        "coupling: {compared} programs compared ({flagged} flagged and not compared), {edges} \
         edges; N: {pairs} members of two candidates, {three} of three or more on {with_three} \
         programs, each inside one edge; {past} of those on {with_past} programs hold two \
         candidates the pair layer ties by no edge; R is the product of its parts over the \
         components on the {factored} programs the conflicts and the derivations leave two \
         components or more of, {several} of them with two or more over every edge; {debts} debts \
         are edges, on {with_debts} programs, {unlicensed} of them of an instance L5 does not \
         license, on {with_unlicensed}, and {across} have payers in two of the conflicts' \
         components, on {}: {joined:?}; over the conflicts' components alone R is no product on \
         {}: {finer:?}",
        joined.len(),
        finer.len()
    );
}

// ── The controls ──────────────────────────────────────────────────────

/// **A conflict of three is one coupling.** No pair of the three candidates
/// excludes, implies or interacts, so the pair layer ties nothing and each
/// candidate is a question and a component of its own; the whole node shows
/// the one application that reads all three.
#[test]
fn a_conflict_of_three_is_one_coupling() {
    let (cover, pairs) = read(L02, CouplingReading::Pairs);
    assert!(cover.groups.is_empty());
    assert_eq!(
        pairs.questions,
        [Question::Alone(0), Question::Alone(1), Question::Alone(2)]
    );
    assert!(pairs.ties.is_empty() && pairs.edges.is_empty());
    assert_eq!(pairs.components.len(), 3);
    assert!(pairs.node.is_none());
    for reading in [
        CouplingReading::Direct,
        CouplingReading::OneStep,
        CouplingReading::Node,
    ] {
        let (_, c) = read(L02, reading);
        assert!(c.complete(), "{reading:?}");
        assert_eq!(c.edges.len(), 1, "{reading:?}");
        let e = &c.edges[0];
        assert_eq!(e.questions, [0, 1, 2]);
        assert_eq!((e.pairs.all(), e.node.conflicts, e.node.all()), (0, 1, 1));
        assert_eq!(c.components, [vec![0, 1, 2]]);
        assert_eq!(c.edge_over(&[0, 1, 2]), Some(0));
        let node = c.node.expect("the node was read");
        // The conflict's rule, and the one that ends the state on its
        // `(false)`.
        assert_eq!((node.saturations, node.applications), (1, 2));
    }
}

/// The question whose members are `(val LETTER d)` for every digit.
fn letter(cover: &Cover, c: &Coupling, name: char) -> u32 {
    let of = |b: u32| c.text(b).starts_with(&format!("(val {name} "));
    (0..c.questions.len() as u32)
        .find(|&q| match c.questions[q as usize] {
            Question::Group(g) => cover.members(g).iter().all(|&b| of(b)),
            Question::Alone(b) => of(b),
        })
        .unwrap_or_else(|| panic!("no question of {name}"))
}

/// **A cryptarithm shows its columns.** The pair layer ties the two letters
/// of the units column and nothing else: the carry into every other column
/// waits for them. One step back from the node's contradictions gives the
/// units column whole. The whole node gives every column with the letters
/// its carry comes from, each naming the statement of the column it is.
#[test]
fn a_cryptarithm_shows_its_columns() {
    let columns = |cover: &Cover, c: &Coupling| -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = c
            .edges
            .iter()
            .map(|e| {
                let letters: String = "DEMNORSY"
                    .chars()
                    .filter(|&l| e.questions.contains(&letter(cover, c, l)))
                    .collect();
                // Every question of the edge is a letter's.
                assert_eq!(letters.len(), e.arity(), "{e:?}");
                let stated = e.stated.first().expect("a statement");
                (letters, c.reasons[stated.0 as usize].clone())
            })
            .collect();
        out.sort();
        out
    };
    let pair = |a: &str, b: &str| (a.to_string(), b.to_string());

    let (cover, pairs) = read(SEND, CouplingReading::Pairs);
    assert_eq!((cover.groups.len(), pairs.questions.len()), (18, 18));
    assert_eq!(pairs.edges.len(), 1);
    let e = &pairs.edges[0];
    assert_eq!(
        e.questions,
        [letter(&cover, &pairs, 'D'), letter(&cover, &pairs, 'E')]
    );
    assert_eq!((e.pairs.interacts, e.pairs.all()), (90, 90));

    let (cover, one) = read(SEND, CouplingReading::OneStep);
    assert_eq!(
        columns(&cover, &one),
        [
            pair("DE", "(column c1 D E Y)"),
            pair("DEY", "(column c1 D E Y)")
        ]
    );

    let (cover, node) = read(SEND, CouplingReading::Node);
    assert!(node.complete());
    assert_eq!(
        columns(&cover, &node),
        [
            pair("DE", "(column c1 D E Y)"),
            pair("DEMNORS", "(column c4 S M O)"),
            pair("DENOR", "(column c3 E O N)"),
            pair("DENR", "(column c2 N R E)"),
            pair("DEY", "(column c1 D E Y)"),
        ]
    );
    // The top column's second statement is the carry written as a letter.
    let top = node.edges.iter().find(|e| e.arity() == 7).expect("c4");
    assert_eq!(node.reasons[top.stated[1].0 as usize], "(carried c4 M)");
    assert_eq!(node.components.len(), 1);
    assert_eq!(node.node.expect("the node").saturations, 1);
}

/// **A declared bijection's rows and columns are the questions**, with no
/// candidate alone, and one puzzle ties them all into one component.
#[test]
fn a_puzzle_with_one_answer_is_one_component() {
    let (cover, c) = read(Z2, CouplingReading::Node);
    assert!(c.complete());
    assert_eq!((cover.groups.len(), c.questions.len()), (36, 36));
    assert_eq!(c.alone().count(), 0);
    assert_eq!(c.components.len(), 1);
    assert_eq!(c.components[0], c.committed);
    // Every tie is of two candidates no question holds together.
    for t in &c.ties {
        assert!(!c.in_one_question(&[t.pair.0, t.pair.1]), "{t:?}");
    }
}

/// **A debt is an edge.** Root owes each of two people a seat, and no
/// conflict and no derivation ties one's seats to the other's: the
/// conflicts leave every candidate a component of its own, and `R` holds
/// no commitment that seats one and not the other. Each instance's payers
/// are candidates in no group, so its edge is of their questions, its
/// reason the instance.
#[test]
fn a_debt_is_an_edge() {
    let rel = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let (cover, c) = read(rel, CouplingReading::Node);
    assert!(cover.groups.is_empty() && c.complete());
    assert!(!c.debts.is_empty());
    assert_eq!(c.debts_across().len(), c.debts.len());
    for (k, d) in c.debts.iter().enumerate() {
        assert!(d.payers.len() >= 2, "{d:?}");
        let e = c
            .edges
            .iter()
            .find(|e| e.questions == d.questions)
            .expect("the debt's edge");
        assert!(e.debts.contains(&(k as u32)));
        assert_eq!(e.kind(), structure::EdgeKind::Debt, "{e:?}");
        assert!(e.stated.is_empty());
        // Its questions are its payers', each a candidate alone.
        let of: Vec<u32> = d
            .payers
            .iter()
            .map(|&b| c.question_of[b as usize][0])
            .collect();
        assert_eq!(of, d.questions);
        // No conflict reads a debt's edge.
        assert_eq!(c.edge_over(&d.payers), None);
    }
    assert!(c.components.len() < c.conflict_components.len());
    assert_eq!(c.conflict_components.len(), c.committed.len());

    // A letter owes a digit, and its ten digits are one question: no edge.
    let (_, send) = read(SEND, CouplingReading::Node);
    assert!(send.debts.is_empty() && send.unlicensed == 0);
    assert!(
        send.edges
            .iter()
            .all(|e| e.kind() == structure::EdgeKind::Tie)
    );
    assert_eq!(send.components, send.conflict_components);
}

/// **The whole node's reading holds the edges one application back too**,
/// the ones [`CouplingReading::OneStep`] gives, off the same node and the
/// same pair layer.
#[test]
fn the_node_keeps_the_edges_one_step_back() {
    let edges = |c: &Coupling, edges: &[structure::Edge]| -> Vec<String> {
        edges
            .iter()
            .map(|e| {
                let stated: Vec<(&str, u32)> = e
                    .stated
                    .iter()
                    .map(|&(k, n)| (c.reasons[k as usize].as_str(), n))
                    .collect();
                format!("{:?}", (&e.questions, e.pairs, e.node, &e.debts, stated))
            })
            .collect()
    };
    for rel in [
        L02,
        SEND,
        Z2,
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
    ] {
        let (_, node) = read(rel, CouplingReading::Node);
        let (_, one) = read(rel, CouplingReading::OneStep);
        assert!(one.one_step.is_none());
        let kept = node.one_step.as_ref().expect("under the whole node");
        assert_eq!(edges(&node, kept), edges(&one, &one.edges), "{rel}");
    }
    // On a grid of bijections one step back lists couplings of three or
    // more that the whole node's one class absorbs.
    let (_, c) = read(Z2, CouplingReading::Node);
    let three = |edges: &[structure::Edge]| edges.iter().filter(|e| e.arity() >= 3).count();
    assert!(three(c.one_step.as_ref().expect("kept")) > three(&c.edges));
}

/// What a coupling says.
fn said(c: &Coupling) -> String {
    format!(
        "{:?}",
        (
            (&c.questions, &c.question_of, &c.ties, &c.reasons),
            (&c.edges, &c.one_step, &c.debts, c.unlicensed),
            (&c.components, &c.conflict_components, &c.committed),
            (c.node, c.node_stopped, c.cut)
        )
    )
}

/// **The coupling is the same at any job count**: the pair layers are, and
/// the whole node is one fork.
#[test]
fn the_coupling_is_the_same_at_any_job_count() {
    for rel in [L02, SEND, "examples/branching/05_mini_zebra.ein"] {
        let at = |jobs: usize| {
            let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("loads");
            let opts = CouplingOptions {
                layer: structure::Options {
                    jobs,
                    ..structure::Options::default()
                },
                ..CouplingOptions::default()
            };
            let (_, c) = structure::coupling(&mut kb, &mut terms, &ast, &opts).expect("coupling");
            said(&c)
        };
        assert_eq!(at(1), at(4), "{rel}");
    }
}

/// **A dead root has no node to read**, and the coupling says so.
#[test]
fn a_dead_root_has_no_coupling() {
    let (_, c) = read("examples/ein-bugs/alt-cap-core.ein", CouplingReading::Node);
    assert_eq!(c.node_stopped, Some(NodeStop::RootDead));
    assert!(c.ties.is_empty() && c.edges.is_empty() && c.components.is_empty());
    assert!(c.node.is_none() && c.committed.is_empty());
    assert!(!c.complete());
}

/// The one-relation zebra, each placement in both orientations, every
/// instance of its table licensed.
const Z1: &str = "examples/zebra.ein";

/// [`Z1`]'s structure at the default budget, read once.
fn z1_whole() -> &'static structure::Whole {
    static WHOLE: std::sync::OnceLock<structure::Whole> = std::sync::OnceLock::new();
    WHOLE.get_or_init(|| whole_at(Z1, structure::Options::default().budget))
}

/// Every object of `rel`'s structure, each pair layer read at `budget`.
fn whole_at(rel: &str, budget: u64) -> structure::Whole {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("loads");
    let asked = structure::WholeOptions {
        layer: structure::Options {
            budget,
            ..structure::Options::default()
        },
        supports: false,
    };
    structure::whole(&mut kb, &mut terms, &ast, &asked).expect("a structure")
}

/// The witnesses of `x`'s table that some single's node holds: at root, and
/// by the theory alone.
fn holders(x: &structure::ExactlyOne) -> (usize, usize) {
    let witnesses = x
        .owed
        .table()
        .iter()
        .flat_map(|i| i.witnesses.iter().flatten());
    let (mut at_root, mut in_theory) = (0, 0);
    for w in witnesses {
        at_root += usize::from(!w.held_at_root.is_empty());
        in_theory += usize::from(!w.held_in_theory.is_empty());
    }
    (at_root, in_theory)
}

/// **A layer that read its singles has its witnesses' holders read,
/// whatever it spent.** The holders are read off the singles a layer left
/// in the cache, and nothing is computed for them, so no budget is asked.
/// On the one-relation zebra 76 of the witnesses of what root owes are
/// held by a single's node at root, and 112 by the theory alone, every
/// witness that is a candidate. A layer's last step is afforded by a
/// projection and may carry it past its budget: one fact under what root's
/// layer spends, the layer reads every pair, the holders are read, and
/// *exactly one* and the coupling are the ones of the default budget,
/// complete. And one fact under what the small KB's layer spends, that
/// layer reads every pair, and every witness's holders, at root and by the
/// theory, and every instance's standing are the default budget's. Each
/// budget is read off the program's own spend.
#[test]
fn a_layer_that_read_its_singles_has_its_holders_read() {
    let rel = Z1;
    let whole = z1_whole();
    let parts = whole.parts.as_ref().expect("a cover");
    assert_eq!(holders(&parts.exactly_one), (76, 112), "{rel}");
    assert!(parts.coupling.complete());

    // One fact under what root's layer spends.
    let spent = whole.theory.root.work.facts;
    let at = whole_at(rel, spent - 1);
    let root = &at.theory.root;
    assert!(
        root.complete() && root.pairs.left == 0,
        "{:?}",
        root.stopped
    );
    assert_eq!(root.work.facts, spent, "its last step passed the budget");
    let p = at.parts.as_ref().expect("a cover");
    assert!(p.exactly_one.owed.singles_read && p.exactly_one.owed.theory_read);
    assert!(p.coupling.complete());
    assert_eq!(said(&p.coupling), said(&parts.coupling));
    assert_eq!(p.exactly_one, parts.exactly_one);

    // One fact under what the small KB's layer spends.
    let small_spent = whole.theory.small.as_ref().expect("a layer").work.facts;
    assert!(small_spent < spent, "the small KB's layer costs root's now");
    let at = whole_at(rel, small_spent - 1);
    let small = at.theory.small.as_ref().expect("a layer");
    assert!(
        small.complete() && small.pairs.left == 0,
        "{:?}",
        small.stopped
    );
    assert_eq!(small.work.facts, small_spent);
    let p = at.parts.as_ref().expect("a cover");
    assert!(p.exactly_one.owed.theory_read && p.exactly_one.owed.singles_read);
    assert_eq!(p.exactly_one.owed, parts.exactly_one.owed);
    assert_eq!(p.exactly_one.owing, parts.exactly_one.owing);
    // Root's layer stopped at that budget, and the coupling with it.
    assert!(!at.theory.root.complete() && !p.coupling.complete());
}

/// **Where the payers cannot be read the coupling is not complete, and says
/// which reading it lacks**: [`Coupling::complete`] reads
/// [`Coupling::payers_read`], and a coupling whole in everything else is not
/// complete without it. The payers read are those of an instance L5 does
/// not license, and both zebras' tables hold none: at a budget of one fact
/// root's layer stops among its singles and no witness's holders are read,
/// and nothing is lacking of the payers. Each coupling is incomplete by its
/// pairs.
#[test]
fn holders_unread_leave_the_coupling_incomplete() {
    let mut c = z1_whole().parts.as_ref().expect("a cover").coupling.clone();
    assert!(c.payers_read && c.complete());
    c.payers_read = false;
    assert!(c.pairs_read && c.node_stopped.is_none() && c.cut.is_none());
    assert!(!c.complete());

    // Every instance licensed: nothing is lacking of the payers.
    let at = whole_at(Z1, 1);
    assert!(
        at.theory.root.singles_left > 0,
        "the layer read its singles"
    );
    let p = at.parts.as_ref().expect("a cover");
    assert!(!p.exactly_one.owed.singles_read && !p.exactly_one.owed.theory_read);
    assert_eq!(
        holders(&p.exactly_one),
        (0, 0),
        "no witness's holders are read"
    );
    assert!(p.coupling.unlicensed == 0 && p.coupling.payers_read);
    assert!(!p.coupling.complete(), "its pairs were not read");

    let at = whole_at(Z2, 1);
    assert!(at.theory.root.singles_left > 0);
    let p = at.parts.as_ref().expect("a cover");
    assert!(!p.exactly_one.owed.singles_read);
    assert!(p.coupling.unlicensed == 0 && p.coupling.payers_read);
    assert!(!p.coupling.complete(), "its pairs were not read");
}
