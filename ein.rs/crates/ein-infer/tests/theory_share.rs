//! **The theory's share is held by invariances**, and by the program it is a
//! share of.
//!
//! [`ein_infer::structure::theory`] reads a program twice over one universe:
//! root's pair layer, and the pair layer of the **small KB**, the program
//! with the puzzle's statements taken out by
//! [`ein_infer::structure::small_kb`]. What the small KB kills is *of the
//! theory*. Nothing declares which statements are the puzzle's, so the rule
//! is held by what must be true of any right one:
//!
//! 1. **the declared control**: `zebra2.ein` declares its five `*-loc`
//!    relations bijective, and the theory excludes exactly the pairs of `H`
//!    along a row or a column of one of them;
//! 2. **one clue apart, one theory**: `zebra2.ein` and `zebra2-minus-15.ein`
//!    differ by one condition, so the theory decides alike of the candidates
//!    both have;
//! 3. **two encodings, one theory**: `zebra.ein` and `zebra2.ein` state one
//!    puzzle over one `co-located` relation and over five, and the theory
//!    excludes the same pairs of placements under the renaming, each in the
//!    four orientations the one relation writes it in;
//! 4. **a smaller program derives no more**: what is dead in the small KB is
//!    dead at root, and the small KB's fixpoint is inside root's, on every
//!    corpus program the pair layer's own default sweep reads;
//! 5. **the two readings of a pair agree**: the support of each pair root
//!    excludes ([`ein_infer::explain::support_frontier`]) lies inside the
//!    small KB exactly where the small KB kills the pair, and the small KB
//!    with the support's other statements put back kills it, by a node inside
//!    the program's own.
//!
//! The corpus's `slow` entries are not read by the sweep, but for the
//! instances named here, which every run reads.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test theory_share
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::structure::{self, Deaths, PairLayer, Theory, TheoryOptions};
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

fn theory_of(rel: &str, opts: &TheoryOptions) -> Theory {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
    structure::theory(&mut kb, &mut terms, &ast, opts).expect("the theory's share")
}

/// The small KB's layer alone: root read as far as its universe.
fn small_alone() -> TheoryOptions {
    TheoryOptions {
        root_pairs: false,
        ..TheoryOptions::default()
    }
}

/// Root's layer, the small KB's, every support and its check.
fn checked() -> TheoryOptions {
    TheoryOptions {
        supports: true,
        check: true,
        ..TheoryOptions::default()
    }
}

fn small(t: &Theory) -> &PairLayer {
    t.small.as_ref().expect("the small KB has a layer")
}

/// A `(rel value house)` candidate's three names.
fn parts(text: &str) -> Vec<&str> {
    text.trim_matches(|c| c == '(' || c == ')')
        .split_whitespace()
        .collect()
}

/// **The declared control.** `zebra2.ein`'s five `*-loc` relations are
/// declared bijective, so *rules, ontology and the two facts* exclude two
/// candidates of one relation with the same value or the same house, and
/// nothing else: every other exclusion needs a condition. The theory's
/// pairs are those 128, over all 56 candidates root leaves, the 21 dead
/// alone at root among them; no candidate is dead alone in the small KB;
/// and of the pairs root excludes, the theory's are the 45 along a row or a
/// column.
#[test]
fn the_theory_excludes_a_declared_bijections_rows_and_columns() {
    let t = theory_of("examples/zebra2.ein", &TheoryOptions::default());
    let (l, s) = (&t.root, small(&t));
    assert!(s.monotone && s.complete() && !s.root_dead);
    assert_eq!(
        s.dead_alone,
        [],
        "no candidate is dead alone in the small KB"
    );
    assert!(t.against_root.is_empty() && t.small_beyond_root == 0);

    let n = l.n() as u32;
    let mut lines: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut relations: BTreeSet<&str> = BTreeSet::new();
    for a in 0..n {
        for b in a + 1..n {
            let (x, y) = (parts(l.text(a)), parts(l.text(b)));
            assert!(x.len() == 3 && x[0].ends_with("-loc"), "{}", l.text(a));
            if x[0] == y[0] && (x[1] == y[1]) != (x[2] == y[2]) {
                lines.insert((a, b));
                relations.insert(x[0]);
            }
        }
    }
    assert_eq!(relations.len(), 5, "{relations:?}");
    assert_eq!((l.n(), lines.len()), (56, 128));
    let theory: BTreeSet<(u32, u32)> = s.excluded.iter().copied().collect();
    assert_eq!(
        theory, lines,
        "the theory's pairs are the rows and the columns"
    );

    // Of the pairs root excludes: along a line, the theory's; off one, the
    // puzzle's.
    let root: BTreeSet<(u32, u32)> = l.excluded.iter().copied().collect();
    for (&p, of_theory) in l.excluded.iter().zip(t.of_the_theory()) {
        assert_eq!(
            of_theory,
            lines.contains(&p),
            "{} {}",
            l.text(p.0),
            l.text(p.1)
        );
    }
    let along = root.intersection(&lines).count();
    assert_eq!((root.len(), along), (327, 45));
    // The other lines hold a candidate dead alone at root, which is why root
    // calls none of them an exclusion.
    let dead = Deaths::of(l);
    assert!(
        lines
            .difference(&root)
            .all(|&(a, b)| dead.single(a) || dead.single(b))
    );

    // What the small KB drops is the givens, the clues and the row of
    // houses; what it keeps of the candidates' objects is their types, which
    // the hrule reads to build `H`.
    let dropped: Vec<&str> = t
        .stated
        .iter()
        .filter(|s| !s.kept && s.origin == structure::Origin::Fact)
        .map(|s| s.text.as_str())
        .collect();
    assert_eq!(dropped.len(), 18, "{dropped:#?}");
    for kept in [
        "(bijective color-loc)",
        "(is-a Red Color)",
        "(symmetric next-to)",
    ] {
        assert!(
            t.stated.iter().any(|s| s.kept && s.text == kept),
            "{kept} is the theory's"
        );
    }
    for gone in [
        "(drink-loc Milk House-3)",
        "(co-located nation-loc Englishman color-loc Red)",
        "(adjacent-via next-to nation-loc Norwegian color-loc Blue)",
        "(right-of House-2 House-1)",
    ] {
        assert!(dropped.contains(&gone), "{gone} is the puzzle's");
    }
}

/// What the theory says of the candidates `keep` holds: the candidates dead
/// alone in the small KB and the pairs it excludes, as texts.
fn share(t: &Theory, keep: &dyn Fn(&str) -> bool) -> (BTreeSet<String>, BTreeSet<[String; 2]>) {
    let s = small(t);
    let alone = s
        .dead_alone
        .iter()
        .map(|&b| s.text(b).to_string())
        .filter(|x| keep(x))
        .collect();
    let pairs = s
        .excluded
        .iter()
        .map(|&(a, b)| [s.text(a).to_string(), s.text(b).to_string()])
        .filter(|[a, b]| keep(a) && keep(b))
        .map(|mut p| {
            p.sort();
            p
        })
        .collect();
    (alone, pairs)
}

/// **One clue apart, one theory.** `zebra2-minus-15.ein` is `zebra2.ein`
/// with condition (15) deleted and nothing else, so the two have one theory:
/// over the candidates both propose, the small KB of each kills the same
/// singles and excludes the same pairs. Root does not: one clue fewer leaves
/// 96 candidates where the other leaves 56, and excludes other pairs.
#[test]
fn two_programs_one_clue_apart_have_one_theory() {
    let whole = theory_of("examples/zebra2.ein", &small_alone());
    let minus = theory_of("examples/zebra2-minus-15.ein", &small_alone());
    let of = |t: &Theory| -> BTreeSet<String> {
        t.root.candidates.iter().map(|c| c.text.clone()).collect()
    };
    let (a, b) = (of(&whole), of(&minus));
    let both: BTreeSet<&String> = a.intersection(&b).collect();
    assert_eq!((a.len(), b.len(), both.len()), (56, 96, 56));
    let keep = |x: &str| both.contains(&x.to_string());
    let (one, other) = (share(&whole, &keep), share(&minus, &keep));
    assert_eq!(one.1.len(), 128);
    assert_eq!(
        one, other,
        "the theory's share over the candidates both have"
    );
    // And the under-determined program's own: every row and column of its
    // 96, each pair of them excluded at root too.
    assert_eq!(small(&minus).excluded.len(), 318);
    assert_eq!(small(&minus).dead_alone, []);
}

/// A `(rel x House-n)` or `(rel House-n x)` candidate's placement,
/// `[x, House-n]`: a fact and its mirror are one placement.
fn placement(text: &str) -> [String; 2] {
    let p = parts(text);
    if p[1].starts_with("House-") {
        [p[2].to_string(), p[1].to_string()]
    } else {
        [p[1].to_string(), p[2].to_string()]
    }
}

/// How many times each of `items` occurs.
fn counted<T: Ord>(items: impl IntoIterator<Item = T>) -> std::collections::BTreeMap<T, usize> {
    let mut out = std::collections::BTreeMap::new();
    for x in items {
        *out.entry(x).or_default() += 1;
    }
    out
}

/// **Two encodings, one theory.** `zebra.ein` states the puzzle over one
/// symmetric `co-located` relation whose structure `std.slots` gives, and
/// `zebra2.ein` over five relations declared bijective. Root proposes the
/// same 56 placements in both: `(R-loc X House-n)` in `zebra2.ein`, and in
/// `zebra.ein` the hrule's `(co-located X House-n)` beside its mirror
/// `(co-located House-n X)`, which pays what a house owes and joins `H`
/// beside it (Q-M1f.570), each placement twice. The theory excludes the
/// same 128 pairs of placements, each in its four orientations, 512 pairs,
/// and never a candidate with its mirror: which statements are the
/// puzzle's does not turn on how a relation is named or on which module's
/// rules read it.
#[test]
fn the_same_puzzle_in_two_encodings_has_one_theory() {
    let placements = |t: &Theory| -> (Vec<[String; 2]>, Vec<[[String; 2]; 2]>) {
        let s = small(t);
        let key = |b: u32| placement(s.text(b));
        assert_eq!(s.dead_alone, []);
        (
            (0..s.n() as u32).map(key).collect(),
            s.excluded
                .iter()
                .map(|&(a, b)| {
                    let mut p = [key(a), key(b)];
                    p.sort();
                    p
                })
                .collect(),
        )
    };
    let one = placements(&theory_of("examples/zebra.ein", &small_alone()));
    let five = placements(&theory_of("examples/zebra2.ein", &small_alone()));
    let (one, five) = (
        (counted(one.0), counted(one.1)),
        (counted(five.0), counted(five.1)),
    );
    assert_eq!((five.0.len(), five.1.len()), (56, 128));
    assert!(five.0.values().chain(five.1.values()).all(|&n| n == 1));
    assert!(
        one.0.keys().eq(five.0.keys()),
        "one universe, under the renaming"
    );
    assert!(one.0.values().all(|&n| n == 2), "each placement twice");
    assert!(
        one.1.keys().eq(five.1.keys()),
        "one theory, under the renaming"
    );
    assert!(
        one.1.values().all(|&n| n == 4),
        "each pair in its four orientations"
    );
    assert!(
        one.1.keys().all(|[a, b]| a != b),
        "no candidate with its mirror"
    );
}

/// The instances every run reads, `slow` or not, as the pair layer's sweep
/// reads them.
const INSTANCES: [&str; 3] = [
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/lattice/02_genuine_3set_death.ein",
];

/// Every corpus file and regression witness but the `slow` entries that are
/// not [`INSTANCES`], once.
fn files() -> Vec<PathBuf> {
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
    files.retain(|f| !slow.contains(f));
    files
}

/// What one program's two layers said of each other.
#[derive(Default)]
struct Held {
    failures: Vec<String>,
    candidates: usize,
    dropped: usize,
    theory_pairs: usize,
    theory_singles: usize,
    flagged: bool,
}

/// `None` where the program does not load or its run ends in an error: it
/// has no layer to take a share of.
fn held(path: &Path) -> Option<Held> {
    let rel = rel_of(path);
    let (mut kb, mut terms, ast) = loaded(path)?;
    let t = structure::theory(&mut kb, &mut terms, &ast, &TheoryOptions::default()).ok()?;
    let mut out = Held {
        candidates: t.root.n(),
        dropped: t.dropped().len(),
        flagged: !t.root.monotone,
        ..Held::default()
    };
    let s = match &t.small {
        Ok(s) => s,
        Err(e) => {
            out.failures
                .push(format!("{rel}: the small KB has no layer: {e}"));
            return Some(out);
        }
    };
    out.theory_pairs = s.excluded.len();
    out.theory_singles = s.dead_alone.len();
    if !s.complete() {
        out.failures.push(format!(
            "{rel}: the budget stopped the small KB's layer: {:?}",
            s.stopped
        ));
    }
    if s.root_dead && !t.root.root_dead {
        out.failures.push(format!(
            "{rel}: the small KB's root is dead and root's is not"
        ));
    }
    if t.small_beyond_root > 0 {
        out.failures.push(format!(
            "{rel}: the small KB's fixpoint holds {} facts root's does not",
            t.small_beyond_root
        ));
    }
    for &a in &t.against_root.singles {
        out.failures.push(format!(
            "{rel}: {} is dead alone in the small KB and alive alone at root",
            t.root.text(a)
        ));
    }
    for &(a, b) in &t.against_root.pairs {
        out.failures.push(format!(
            "{rel}: {{{}, {}}} is dead in the small KB and alive at root",
            t.root.text(a),
            t.root.text(b)
        ));
    }
    Some(out)
}

/// **A smaller program derives no more.** The small KB is the program over
/// fewer of its statements, so its fixpoint is inside root's and what it
/// kills root kills: a candidate dead alone in it is dead alone at root, and
/// a pair dead in it is dead at root. One that is not rests on the absence
/// of a statement the small KB dropped, and the rule that dropped it does
/// not fit that program.
#[test]
fn the_theory_derives_no_more_than_the_program() {
    let files = files();
    let results = ein_corpus::pool::map(
        "theory_share::held",
        &files,
        |p| (rel_of(p), String::new()),
        |p| held(p),
    );
    let mut failures: Vec<String> = Vec::new();
    let (mut read, mut with_candidates, mut dropping, mut with_pairs, mut pairs) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut with_singles, mut flagged) = (0usize, 0usize);
    for h in results.into_iter().flatten() {
        failures.extend(h.failures);
        read += 1;
        with_candidates += usize::from(h.candidates > 0);
        dropping += usize::from(h.dropped > 0);
        with_pairs += usize::from(h.theory_pairs > 0);
        pairs += h.theory_pairs;
        with_singles += usize::from(h.theory_singles > 0);
        flagged += usize::from(h.flagged);
    }
    assert!(
        failures.is_empty(),
        "{} place(s) where the small KB says more than the program:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "theory_share: {read} programs read, {with_candidates} with a candidate, {dropping} \
         whose small KB drops a statement, {with_pairs} with a pair the theory excludes \
         ({pairs} pairs), {with_singles} with a candidate the theory kills alone, {flagged} \
         flagged"
    );
    // Not vacuous: programs of each kind.
    assert!(read >= 300, "only {read} programs read");
    assert!(dropping >= 60, "only {dropping} small KBs drop a statement");
    assert!(
        with_pairs >= 30,
        "only {with_pairs} programs with a theory's pair"
    );
    assert!(flagged >= 10, "only {flagged} flagged programs read");
}

/// **The two readings of a pair agree**, on `rel`: every pair root excludes
/// has a support; the support lies inside the small KB exactly where the
/// small KB kills the pair; and the small KB with the support's other
/// statements put back kills the pair by a node that holds no fact the
/// program's own node of the pair does not. Returns the pairs of the theory
/// and of the puzzle.
fn the_readings_agree(rel: &str) -> (u64, u64) {
    the_readings_agree_on(&theory_of(rel, &checked()), rel)
}

/// [`the_readings_agree`], of `rel`'s theory read [`checked`].
fn the_readings_agree_on(t: &Theory, rel: &str) -> (u64, u64) {
    let l = &t.root;
    assert_eq!(t.supports.len(), l.excluded.len(), "{rel}");
    let r = t.readings();
    assert_eq!(
        (r.unread, r.disagree, r.theory_by_another_way),
        (0, 0, 0),
        "{rel}: {r:?}"
    );
    for (&(a, b), s) in l.excluded.iter().zip(&t.supports) {
        let name = format!("{rel}: {{{}, {}}}", l.text(a), l.text(b));
        assert!(s.read && s.other.is_empty(), "{name}: {s:?}");
        // A pair each of whose members is alive alone dies of both.
        assert_eq!(s.commitments, [a, b], "{name}");
        assert!(!s.rules.is_empty(), "{name}");
        let c = s.check.as_ref().expect("checked");
        assert!(
            c.unread.is_none() && c.dead && !c.root_dead && c.beyond == 0,
            "{name}: {c:?}"
        );
    }
    (r.theory_both, r.puzzle_both)
}

/// The declared control's 327 excluded pairs: 45 of the theory, 282 of the
/// puzzle, each by the conditions its support names.
#[test]
fn the_readings_agree_on_the_declared_control() {
    assert_eq!(the_readings_agree("examples/zebra2.ein"), (45, 282));
}

/// The one-relation encoding's 1 656: 224 of the theory, 1 432 of the
/// puzzle. `H` holds each placement in both orientations, so they are 414
/// pairs of placements, each in its four orientations and all four of one
/// reading: 56 of the theory, 358 of the puzzle.
#[test]
fn the_readings_agree_on_one_relation() {
    let rel = "examples/zebra.ein";
    let t = theory_of(rel, &checked());
    assert_eq!(the_readings_agree_on(&t, rel), (224, 1432));
    let mut folded: std::collections::BTreeMap<[[String; 2]; 2], (usize, usize)> =
        Default::default();
    for (&(a, b), of_theory) in t.root.excluded.iter().zip(t.of_the_theory()) {
        let mut key = [placement(t.root.text(a)), placement(t.root.text(b))];
        key.sort();
        let seen = folded.entry(key).or_default();
        seen.0 += 1;
        seen.1 += usize::from(of_theory);
    }
    assert_eq!(folded.len(), 414);
    assert!(
        folded
            .values()
            .all(|&(n, theory)| n == 4 && (theory == 0 || theory == 4))
    );
    assert_eq!(
        folded.values().filter(|&&(_, theory)| theory == 4).count(),
        56
    );
}

/// **A support's check is read where root's layer passed its budget.** The
/// check compares the small KB's node of a pair with the program's own node
/// of it, which root's layer computed, and reads that node off the cache,
/// which asks no budget. A layer's last step is afforded by a projection and
/// may carry it past its budget: one fact under what root's layer spends on
/// the one-relation zebra, the layer reads every pair, every support is read,
/// and every one is checked, each check the one of the default budget. The
/// budget is read off the program's own spend.
#[test]
fn a_supports_check_is_read_where_roots_layer_passed_its_budget() {
    let rel = "examples/zebra.ein";
    let whole = theory_of(rel, &checked());
    let spent = whole.root.work.facts;
    let mut asked = checked();
    asked.layer.budget = spent - 1;
    let at = theory_of(rel, &asked);
    assert!(
        at.root.complete() && at.root.pairs.left == 0,
        "{:?}",
        at.root.stopped
    );
    assert_eq!(at.root.work.facts, spent, "its last step passed the budget");
    let checks = |t: &Theory| -> Vec<Option<structure::Check>> {
        t.supports.iter().map(|s| s.check.clone()).collect()
    };
    let checked_at = checks(&at);
    assert_eq!(checked_at.len(), at.root.excluded.len(), "a support a pair");
    assert!(
        checked_at
            .iter()
            .all(|c| c.as_ref().is_some_and(|c| c.unread.is_none())),
        "a support left unchecked: {:?}",
        checked_at
            .iter()
            .find(|c| c.as_ref().is_none_or(|c| c.unread.is_some()))
    );
    assert_eq!(
        checked_at,
        checks(&whole),
        "each check is the default budget's"
    );
    assert_eq!(at.readings(), whole.readings());
}

/// Two cryptarithms. SEND + MORE = MONEY's 609 excluded pairs are a letter's
/// two digits or a digit's two letters, all the theory's; the smaller one's
/// 215 hold 71 that its one column excludes. In both the puzzle's
/// statements are a module's tables and the program's columns alike, and
/// the small KB kills no candidate alone: *S begins a number* is the
/// puzzle's, so the theory's pairs are every letter's and every digit's,
/// the two candidates root kills alone among them.
#[test]
fn the_readings_agree_on_the_cryptarithms() {
    assert_eq!(
        the_readings_agree("examples/beyond_arith/send_more_money.ein"),
        (609, 0)
    );
    assert_eq!(
        the_readings_agree("examples/beyond_arith/cryptarithm.ein"),
        (144, 71)
    );
    let t = theory_of(
        "examples/beyond_arith/send_more_money.ein",
        &TheoryOptions::default(),
    );
    assert_eq!(t.root.dead_alone.len(), 2);
    assert_eq!(small(&t).dead_alone, []);
    // 8 letters of 10 digits and 10 digits of 8 letters.
    assert_eq!(small(&t).excluded.len(), 8 * 45 + 10 * 28);
}

/// **A clue through a name no candidate holds.** Four friends in two groups
/// sit at two tables: `(member Ann P1)` and `(at S1 T1)` each name one of
/// the candidates' objects and a group or a table, and a rule joins them.
/// The program's hrule reads `is-a` alone to build `H`, so they are the
/// puzzle's: the theory's pairs are a friend's two seats and a seat's two
/// friends, 48, and the 16 pairs root excludes besides are friends at two
/// tables.
#[test]
fn a_clue_through_a_name_no_candidate_holds_is_the_puzzles() {
    let rel = "examples/lattice/07_two_pairs_two_tables.ein";
    assert_eq!(the_readings_agree(rel), (48, 16));
    let t = theory_of(rel, &TheoryOptions::default());
    assert_eq!(small(&t).excluded.len(), 48);
    for (text, kept) in [
        ("(member Ann P1)", false),
        ("(at S1 T1)", false),
        ("(is-a Ann Friend)", true),
        ("(is-a T1 Table)", true),
        ("(functional sits)", true),
    ] {
        let s = t.stated.iter().find(|s| s.text == text).expect(text);
        assert_eq!(s.kept, kept, "{text}: {}", s.clause);
    }
}

/// **Where no pair excludes there is no share to take**: the conflict of
/// three, whose theory and whose program both leave every pair alive.
#[test]
fn a_conflict_of_three_has_no_theorys_pair() {
    let t = theory_of("examples/lattice/02_genuine_3set_death.ein", &checked());
    assert_eq!(t.root.excluded, []);
    assert_eq!(small(&t).excluded, []);
    assert_eq!(small(&t).dead_alone, []);
    assert!(t.supports.is_empty() && t.dropped().is_empty());
}

/// **Root read as far as its universe** gives the small KB's layer as the
/// whole reading does, and says of root's own layer that it stopped by
/// asking.
#[test]
fn the_small_kbs_layer_needs_root_s_universe_alone() {
    let rel = "examples/zebra2-hints.ein";
    let whole = theory_of(rel, &TheoryOptions::default());
    let alone = theory_of(rel, &small_alone());
    let said = |l: &PairLayer| {
        format!(
            "{:?}",
            (
                &l.dead_alone,
                &l.implies,
                &l.equivalent,
                &l.excluded,
                &l.interacting,
                l.work
            )
        )
    };
    assert!(small(&whole).excluded.len() > 10);
    assert_eq!(said(small(&whole)), said(small(&alone)));
    assert_eq!(
        alone.root.stopped,
        Some(ein_infer::explore::Stopped::NoSearch)
    );
    assert_eq!(alone.root.singles_left, alone.root.n() as u64);
    assert!(alone.root.excluded.is_empty() && alone.against_root.is_empty());
    let texts =
        |t: &Theory| -> Vec<String> { t.root.candidates.iter().map(|c| c.text.clone()).collect() };
    assert_eq!(texts(&whole), texts(&alone));
}
