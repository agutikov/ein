//! **A debt root does not owe: what H holds for it** — the obligations
//! rung's reading of the activators a commitment can derive
//! ([`ein_infer::oblgen`], § *A debt root is not told of*; Q-M1f.307 (A), by
//! the static reading).
//!
//! H is taken once, at root. A rule can derive an obligation's activator at a
//! node, and that node then owes what root, which never held the activator,
//! proposed nothing for. The rung reads those activators off the rules and
//! root's plans, scans each one's plan at root, and repeats until nothing is
//! added. This file holds that reading on the register's witness and its
//! control and on seven programs written for it, each the witness with one
//! thing changed:
//!
//! 1. **the universe is the reading's**, fact for fact, with which activator
//!    each added candidate is there for and which rung built H;
//! 2. **R over that universe is the oracle's**: the cube
//!    ([`ein_infer::region::compute`], `ein oracle`'s library) classifies
//!    every commitment, its members are the ones written here, a bare
//!    describe ([`ein_infer::explore::run`]) counts the same R, and
//!    `--single` says the word it makes.
//!
//! **The reading is at relation grain, and inexact both ways.** Both cases
//! are here by name, each the answer the reading gives and not the answer a
//! reading over nodes would: [`OVER`], where no alive node derives the
//! activator and H still holds what would pay it, and [`UNDER`], where the
//! rule writes the activator with a variable and H gains nothing. So is the
//! shape it does not reach, [`HRULE_READS_H`]: no activator is derived there
//! at all.

use std::collections::BTreeSet;

use ein_core::{Kb, Terms};
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode as Question, Verdict, Which};
use ein_infer::oblgen::Mode;
use ein_infer::region::{self, Outcome};
use ein_ir::{Ast, load, parse};

/// What every program here starts from: Ann, a Person; `likes`, which root
/// owes her a Food of; `drinks`, which nothing at root owes.
const HEAD: &str = "\
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why \"{?1} is a {?2}\")
(relation likes  Person Food :why \"{?1} likes {?2}\")
(relation drinks Person Drink :why \"{?1} drinks {?2}\")
";

/// The register's witness, `W26/oblig-derived-activator`: `arm` concludes the
/// second obligation's activator from any `likes` fact.
const WITNESS: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why \"{?p} likes {?f}, which switches on the debt that every Person drinks a Drink\")
(solve :no-hypothesis (is-a))
";

/// Its control: the activator also stated, so root owes both debts.
const CONTROL: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why \"{?p} likes {?f}, which switches on the debt that every Person drinks a Drink\")
(solve :no-hypothesis (is-a))
";

/// A second food, and `arm` fires on Soup alone: a member of R need not
/// derive the activator, so the reading's R is not the R of the program that
/// states it. `{(likes Ann Stew)}` is a member here.
const PARTIAL: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Stew Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p Soup) :assert (total-owed drinks is-a) :why \"{?p} likes Soup, which switches on the debt that every Person drinks a Drink\")
(solve :no-hypothesis (is-a))
";

/// **Over.** `arm` needs two foods of one person, which `one-food` refutes,
/// so no alive node derives the activator. The reading is at relation grain:
/// `arm` reads `likes`, which a commitment can change, so H holds the tea, and
/// two states of R hold a tea no node owes.
const OVER: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Stew Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (and (likes ?p Soup) (likes ?p Stew)) :assert (total-owed drinks is-a) :why \"{?p} likes Soup and Stew, which switches on the debt that every Person drinks a Drink\")
(rule one-food () :match (and (likes ?p ?f) (likes ?p ?g) (neq ?f ?g)) :assert (false) :why \"{?p} likes {?f} and {?g}, and a Person likes one Food\")
(solve :no-hypothesis (is-a))
";

/// **Under.** `arm` writes the owed relation as a variable, which a stated
/// fact binds. No `:assert` writes an activator that holds no variable, so
/// the reading finds none: H is root's, the node that likes Soup owes a
/// drink nothing pays, and R is empty.
const UNDER: &str = "\
(relation wanted T :why \"a debt over {?1} is wanted\")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(wanted drinks)
(total-owed likes is-a)
(rule arm () :match (and (likes ?p ?f) (wanted ?R) (relation ?R ?A ?B)) :assert (total-owed ?R is-a) :why \"{?p} likes {?f}, which switches on the debt over {?R}\")
(solve :no-hypothesis (is-a wanted))
";

/// The tea that pays the debt a node is told of switches on a second one,
/// over `eats`: the reading's second pass, since `drinks` is a relation a
/// commitment can change only once the first pass has proposed a fact of it.
const TWO_STEPS: &str = "\
(relation eats   Person Dish :why \"{?1} eats {?2}\")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink) (is-a Pie Dish)
(total-owed likes is-a)
(rule arm   () :match (likes ?p ?f)  :assert (total-owed drinks is-a) :why \"{?p} likes {?f}, which switches on the debt that every Person drinks a Drink\")
(rule arm-2 () :match (drinks ?p ?d) :assert (total-owed eats is-a)   :why \"{?p} drinks {?d}, which switches on the debt that every Person eats a Dish\")
(solve :no-hypothesis (is-a))
";

/// A chain of activators: `opens` derives `(serves K1)`, the activator of a
/// rule root has no plan of, and that rule's `:assert` writes the
/// obligation's. The relations a commitment can change have to be followed
/// through the rule root never compiled, as *forbid* follows them.
const CHAIN: &str = "\
(relation serves Kitchen :why \"kitchen {?1} serves\")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink) (is-a K1 Kitchen)
(total-owed likes is-a)
(rule opens () :match (likes ?p ?f) :assert (serves K1) :why \"{?p} likes {?f}, so kitchen K1 serves\")
(rule serves (?k) :match (is-a ?k Kitchen) :assert (total-owed drinks is-a) :why \"{?k} serves, which switches on the debt that every Person drinks a Drink\")
(solve :no-hypothesis (is-a serves))
";

/// The debt a node is told of is over a domain a commitment can change:
/// `more` derives a `kind` fact, and the later plan's guard scans `kind`. The
/// rung holds a later plan to its own decline, so the call declines and the
/// blind enumerator builds H.
const FLIP: &str = "\
(relation kind   T T :why \"{?1} is of kind {?2}\")
(is-a Ann Person) (is-a Soup Food)
(kind Ann Person) (kind Tea Drink)
(total-owed likes is-a)
(rule arm  () :match (likes ?p ?f) :assert (total-owed drinks kind) :why \"{?p} likes {?f}, which switches on the debt that every Person drinks a Drink\")
(rule more () :match (likes ?p ?f) :assert (kind Juice Drink) :why \"{?p} likes {?f}, so Juice is a Drink\")
(solve :no-hypothesis (is-a kind))
";

/// **Not reached.** Both debts are stated and no activator is derived: the
/// hrule that proposes a drink reads `likes`, which the other hrule proposes,
/// so it proposes at a node what it did not at root. The reading is the
/// obligations rung's and reads activators, so H is the hrules' at root and
/// R is empty; the static check calls the program monotone.
const HRULE_READS_H: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why \"guess: (likes {?a} {?b})?\")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why \"{?p} likes {?f}; guess: (drinks {?p} {?d})?\")
";

/// The witness under hrules that each propose a relation where its activator
/// holds: the hrule rung overrides the ladder, and is as it was.
const HRULE_DERIVED: &str = "\
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why \"{?p} likes {?f}, which switches on the debt that every Person drinks a Drink\")
(hrule guess-likes  () :match (and (total-owed likes is-a)  (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why \"likes is owed; guess: (likes {?a} {?b})?\")
(hrule guess-drinks () :match (and (total-owed drinks is-a) (is-a ?a Person) (is-a ?b Drink)) :assert (drinks ?a ?b) :why \"drinks is owed; guess: (drinks {?a} {?b})?\")
(solve :no-hypothesis (is-a))
";

const LIKES_SOUP: &str = "(likes Ann Soup)";
const LIKES_STEW: &str = "(likes Ann Stew)";
const TEA: &str = "(drinks Ann Tea)";
const PIE: &str = "(eats Ann Pie)";

/// One program, and what the reading gives on it.
struct Probe {
    name: &'static str,
    body: &'static str,
    /// The rung that built H.
    rung: Mode,
    /// H, in its order; `None` where it is the blind enumerator's, held by
    /// its size.
    universe: Result<&'static [&'static str], usize>,
    /// The later debts H holds a candidate for: the activator, the rules
    /// that conclude it, and the candidates added.
    later: &'static [(
        &'static str,
        &'static [&'static str],
        &'static [&'static str],
    )],
    /// R's commitments, each sorted; `None` past the oracle's limit.
    r: Option<&'static [&'static [&'static str]]>,
    /// Whether the static check flags the program.
    flagged: bool,
}

const PROBES: &[Probe] = &[
    Probe {
        name: "the witness",
        body: WITNESS,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, LIKES_SOUP]),
        later: &[("(total-owed drinks is-a)", &["arm"], &[TEA])],
        r: Some(&[&[TEA, LIKES_SOUP]]),
        flagged: true,
    },
    Probe {
        name: "the control",
        body: CONTROL,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, LIKES_SOUP]),
        later: &[],
        r: Some(&[&[TEA, LIKES_SOUP]]),
        flagged: true,
    },
    Probe {
        name: "partial: arm fires on one food of two",
        body: PARTIAL,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, LIKES_SOUP, LIKES_STEW]),
        later: &[("(total-owed drinks is-a)", &["arm"], &[TEA])],
        r: Some(&[
            &[LIKES_STEW],
            &[TEA, LIKES_SOUP],
            &[TEA, LIKES_STEW],
            &[TEA, LIKES_SOUP, LIKES_STEW],
        ]),
        flagged: true,
    },
    Probe {
        name: "over: no alive node derives the activator",
        body: OVER,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, LIKES_SOUP, LIKES_STEW]),
        later: &[("(total-owed drinks is-a)", &["arm"], &[TEA])],
        r: Some(&[
            &[LIKES_SOUP],
            &[LIKES_STEW],
            &[TEA, LIKES_SOUP],
            &[TEA, LIKES_STEW],
        ]),
        flagged: true,
    },
    Probe {
        name: "under: the rule writes the owed relation as a variable",
        body: UNDER,
        rung: Mode::Obligations,
        universe: Ok(&[LIKES_SOUP]),
        later: &[],
        r: Some(&[]),
        flagged: true,
    },
    Probe {
        name: "two steps: the tea that pays switches on a second debt",
        body: TWO_STEPS,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, PIE, LIKES_SOUP]),
        later: &[
            ("(total-owed drinks is-a)", &["arm"], &[TEA]),
            ("(total-owed eats is-a)", &["arm-2"], &[PIE]),
        ],
        r: Some(&[&[TEA, PIE, LIKES_SOUP]]),
        flagged: true,
    },
    Probe {
        name: "chain: the activator's rule is one root has no plan of",
        body: CHAIN,
        rung: Mode::Obligations,
        universe: Ok(&[TEA, LIKES_SOUP]),
        later: &[("(total-owed drinks is-a)", &["serves"], &[TEA])],
        r: Some(&[&[TEA, LIKES_SOUP]]),
        flagged: true,
    },
    Probe {
        name: "flip: the later debt's domain is one a commitment can change",
        body: FLIP,
        rung: Mode::Declined,
        universe: Err(24),
        later: &[],
        r: None,
        flagged: true,
    },
    Probe {
        name: "an hrule reads what the other proposes, no activator derived",
        body: HRULE_READS_H,
        rung: Mode::Hrules,
        universe: Ok(&[LIKES_SOUP]),
        later: &[],
        r: Some(&[]),
        flagged: false,
    },
    Probe {
        name: "the witness on the hrule rung",
        body: HRULE_DERIVED,
        rung: Mode::Hrules,
        universe: Ok(&[LIKES_SOUP]),
        later: &[],
        r: Some(&[]),
        flagged: true,
    },
];

fn loaded(p: &Probe) -> (Kb, Terms, Ast) {
    let text = format!("{HEAD}{}", p.body);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, Some(p.name)).unwrap_or_else(|e| panic!("{}: {e}", p.name));
    let kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{}: {e}", p.name));
    (kb, terms, ast)
}

/// **The universe is the reading's on every probe**: the rung that built H,
/// H fact for fact, and for each later debt its activator, the rules that
/// conclude it and the candidates it added.
#[test]
fn the_universe_is_the_static_readings_on_every_probe() {
    for p in PROBES {
        let (mut kb, mut terms, ast) = loaded(p);
        let checked = explore::check_monotone(&mut kb, &mut terms, &ast, &mut Events::off())
            .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        assert_eq!(
            checked.root.hypgen.rung.mode, p.rung,
            "{}: the rung",
            p.name
        );
        assert_eq!(
            !checked.monotonicity.is_monotone(),
            p.flagged,
            "{}: the static check",
            p.name
        );
        let later: Vec<(&str, Vec<&str>, Vec<&str>)> = checked
            .root
            .later
            .iter()
            .map(|l| {
                (
                    l.activator.as_str(),
                    l.by.iter().map(String::as_str).collect(),
                    l.added.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        let want: Vec<(&str, Vec<&str>, Vec<&str>)> = p
            .later
            .iter()
            .map(|(a, by, added)| (*a, by.to_vec(), added.to_vec()))
            .collect();
        assert_eq!(later, want, "{}: the later debts", p.name);
        // The rung's own counts: what root owes is counted as it was, and a
        // later debt apart.
        let rung = checked.root.hypgen.rung;
        let read = checked.root.hypgen.later.len() as u64;
        assert_eq!(rung.later_debts, read, "{}: later_debts", p.name);
        if p.rung == Mode::Obligations {
            assert_eq!(
                checked.root.hypgen.raw,
                rung.candidates + rung.later_candidates,
                "{}: every candidate built is a debt's of root's or a later one's",
                p.name
            );
        }

        let (mut kb, mut terms, ast) = loaded(p);
        let out = region::compute(
            &mut kb,
            &mut terms,
            &ast,
            &region::Options::default(),
            &mut |_| {},
        )
        .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        let candidates: Vec<&str> = match &out {
            Outcome::Region(r) => r.candidates.iter().map(|c| c.text.as_str()).collect(),
            Outcome::Refused { candidates, .. } => {
                candidates.iter().map(|c| c.text.as_str()).collect()
            }
        };
        match p.universe {
            Ok(h) => assert_eq!(candidates, h, "{}: H", p.name),
            Err(n) => assert_eq!(candidates.len(), n, "{}: |H|", p.name),
        }
        assert_eq!(
            candidates.len(),
            checked.n,
            "{}: one H on both paths",
            p.name
        );
    }
}

/// **R over that universe is the oracle's.** The cube classifies every
/// commitment over H; its members are the ones written for the probe, the
/// over- and under-cases as the reading gives them; a bare describe counts
/// the same R, in states and in commitments; and `--single` says the word
/// that R makes.
#[test]
fn r_over_the_readings_universe_is_the_oracles() {
    for p in PROBES {
        let Some(want) = p.r else {
            // Past the oracle's limit: the blind enumerator's H, which the
            // decline hands the program to, is 2^24 nodes.
            continue;
        };
        let (mut kb, mut terms, ast) = loaded(p);
        let out = region::compute(
            &mut kb,
            &mut terms,
            &ast,
            &region::Options::default(),
            &mut |_| {},
        )
        .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        let Outcome::Region(r) = out else {
            panic!("{}: the cube is within the limit", p.name);
        };
        let n = r.n();
        let members: BTreeSet<Vec<&str>> = (0..1u64 << n)
            .filter(|&m| r.in_r(m))
            .map(|m| {
                (0..n)
                    .filter(|i| m >> i & 1 == 1)
                    .map(|i| r.candidates[i].text.as_str())
                    .collect()
            })
            .collect();
        let want: BTreeSet<Vec<&str>> = want.iter().map(|c| c.to_vec()).collect();
        assert_eq!(members, want, "{}: R's commitments, by the cube", p.name);
        let states: BTreeSet<u64> = (0..1u64 << n)
            .filter(|&m| r.in_r(m))
            .map(|m| r.nodes[m as usize].class)
            .collect();

        let (mut kb, mut terms, ast) = loaded(p);
        let describe = Question {
            which: Which::All,
            ask: Ask::Describe,
        };
        let a = explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(describe))
            .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        assert!(a.exhausted, "{}: describe exhausts R", p.name);
        let counts = a
            .counts
            .unwrap_or_else(|| panic!("{}: describe counts R", p.name));
        assert_eq!(
            (counts.k_c, counts.k_s),
            (members.len() as u64, states.len() as u64),
            "{}: describe's R is the cube's",
            p.name
        );

        // `--single`'s word is the one the cube's R makes: every probe with
        // an empty R has an alive root that owes, the owing kind.
        let (mut kb, mut terms, ast) = loaded(p);
        let single = Question {
            which: Which::All,
            ask: Ask::Single,
        };
        let a = explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(single))
            .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        let word = match states.len() {
            0 => Verdict::Open,
            1 => Verdict::Solution,
            _ => Verdict::Ambiguity,
        };
        assert_eq!(a.verdict, Some(word), "{}: --single's verdict", p.name);
    }
}

/// **A program that reads no later debt narrates the `rung` line it always
/// narrated**, and one that reads one says so in two fields more: the
/// control's line has no `later_debts`, the witness's counts one debt and one
/// candidate beside what root owes.
#[test]
fn the_rung_line_counts_a_later_debt_apart() {
    let rung_line = |p: &Probe| -> String {
        let (mut kb, mut terms, ast) = loaded(p);
        let buffer = ein_infer::events::Buffer::new();
        let mut events = Events::to(Box::new(buffer.clone()), ein_infer::events::Level::Normal);
        explore::check_monotone(&mut kb, &mut terms, &ast, &mut events)
            .unwrap_or_else(|e| panic!("{}: {e}", p.name));
        buffer
            .to_string_lossy()
            .lines()
            .find(|l| l.contains("\"rung\""))
            .unwrap_or_else(|| panic!("{}: the rung narrates", p.name))
            .to_string()
    };
    let witness = rung_line(&PROBES[0]);
    assert!(
        witness.contains("\"owed\": 1")
            && witness.contains("\"candidates\": 1")
            && witness.contains("\"later_debts\": 1")
            && witness.contains("\"later_candidates\": 1"),
        "{witness}"
    );
    let control = rung_line(&PROBES[1]);
    assert!(
        control.contains("\"owed\": 2")
            && control.contains("\"candidates\": 2")
            && !control.contains("later"),
        "{control}"
    );
}
