//! **The cover is held by a control, and by what a cover is.**
//!
//! [`ein_infer::structure::Cover`] is `H` as the groups the theory makes of
//! it: the maximal sets of two or more candidates that pairwise exclude by
//! the theory alone. No relation is read to form one, so what the groups
//! *are* is held from outside:
//!
//! 1. **the declared control**: `zebra2.ein` declares its five `*-loc`
//!    relations bijective, and its groups are exactly the rows and the
//!    columns of those five, over all of `H`, every candidate in two. The
//!    rows and columns are read here off the candidates' texts, which is the
//!    reading by name the cover itself does not make;
//! 2. **two encodings, one cover**: `zebra.ein` states the same puzzle over
//!    one relation, each placement in both orientations, and its groups,
//!    folded onto their placements, are the same groups under the renaming;
//! 3. **one clue apart**: `zebra2-minus-15.ein`'s groups are its own rows and
//!    columns, and over the candidates both programs have they are
//!    `zebra2.ein`'s;
//! 4. **letters and digits**: SEND + MORE = MONEY's groups are a letter's
//!    digits and a digit's letters;
//! 5. **a conflict of three** forms no group, and its three candidates are
//!    in none;
//! 6. **what a cover is**, on every corpus program the pair layer's own
//!    default sweep reads: every group a clique of the theory's pairs and
//!    a maximal one, every pair of the theory inside a group, the two maps
//!    each other's inverse, nothing cut by the bound, and every pair of a
//!    group dead at root;
//! 7. **one cover by every entry**: over a program's prepared space with
//!    root's pairs and without them, and off a theory's share read whole;
//!    and at any job count;
//! 8. **what *exactly one* is**, on the same programs in the same reading:
//!    what root owes is L5's table as a route reads it, of the program
//!    loaded again; an instance stands as [`Cover::standing`] reads its
//!    alternatives; a group is owed by exactly the instances whose
//!    alternatives are its members; and a grid is a component whose two
//!    families each hold every member once
//!    ([`ein_infer::structure::ExactlyOne`]; its controls are
//!    `ein-infer/tests/exactly_one.rs`).
//!
//! The corpus's `slow` entries are not read by the sweep, but for the
//! instances named here, which every run reads.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test cover
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::structure::{
    self, Cover, CoverOptions, ExactlyOne, Mark, Owing, Standing, TheoryOptions,
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

fn cover_by(rel: &str, opts: &CoverOptions) -> Cover {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
    structure::cover(&mut kb, &mut terms, &ast, opts).expect("the cover")
}

/// The cover, root's pairs read too.
fn cover_of(rel: &str) -> Cover {
    cover_by(rel, &CoverOptions::default())
}

/// The cover with root read as far as its singles.
fn singles_alone() -> CoverOptions {
    CoverOptions {
        root_pairs: false,
        ..CoverOptions::default()
    }
}

/// A set of candidates, as their texts.
type Set = BTreeSet<String>;

fn texts(c: &Cover, set: &[u32]) -> Set {
    set.iter().map(|&b| c.text(b).to_string()).collect()
}

fn groups_of(c: &Cover) -> BTreeSet<Set> {
    c.groups.iter().map(|g| texts(c, g)).collect()
}

/// A `(rel x y)` candidate's three names.
fn parts(text: &str) -> Vec<&str> {
    text.trim_matches(|c| c == '(' || c == ')')
        .split_whitespace()
        .collect()
}

/// **The lines of `H`, read off the candidates' texts**: the candidates of
/// one relation that share their first name, or share their second, where
/// there are two or more. The reading by name the cover does not make, and
/// here what it is held to.
fn lines(c: &Cover) -> BTreeSet<Set> {
    let mut by: std::collections::BTreeMap<(String, usize, String), Set> = Default::default();
    for cand in &c.candidates {
        let p = parts(&cand.text);
        assert_eq!(p.len(), 3, "{}", cand.text);
        for k in [1, 2] {
            by.entry((p[0].to_string(), k, p[k].to_string()))
                .or_default()
                .insert(cand.text.clone());
        }
    }
    by.into_values().filter(|s| s.len() > 1).collect()
}

fn in_exactly_two(c: &Cover) -> bool {
    c.member_of.iter().all(|g| g.len() == 2)
}

/// **The declared control.** `zebra2.ein`'s five `*-loc` relations are
/// declared bijective, and its 36 groups are the rows and the columns of
/// those five over all 56 candidates of `H`, the 21 dead alone at root
/// among them: every candidate in exactly two, a value's houses and a
/// house's values. A row holds a member alive alone, so every group has an
/// open one.
///
/// And **root's own exclusion graph has none of them**: once the clues have
/// decided part of the puzzle, each row or column of two or more open
/// members is strictly inside a larger clique of root's pairs, which is why
/// a group is no clique of root's graph.
#[test]
fn the_groups_of_a_declared_bijection_are_its_rows_and_columns() {
    let c = cover_of("examples/zebra2.ein");
    assert!(c.complete() && c.monotone && c.small_monotone);
    assert_eq!(c.n(), 56);
    let relations: BTreeSet<&str> = c.candidates.iter().map(|x| parts(&x.text)[0]).collect();
    assert_eq!(relations.len(), 5, "{relations:?}");
    assert!(relations.iter().all(|r| r.ends_with("-loc")));

    let (groups, lines) = (groups_of(&c), lines(&c));
    assert_eq!(lines.len(), 36);
    assert_eq!(groups, lines, "the groups are the rows and the columns");
    assert!(in_exactly_two(&c), "every candidate in a row and a column");
    assert_eq!(c.in_no_group(), []);
    assert_eq!(c.killed_alone, []);
    assert_eq!(c.excluded.len(), 128);

    // The marks: root's own verdict on each candidate alone.
    let dead = c.marks.iter().filter(|&&m| m == Mark::DeadAlone).count();
    let alive = c.marks.iter().filter(|&&m| m == Mark::AliveAlone).count();
    assert_eq!((dead, alive), (21, 35));
    assert!((0..c.groups.len() as u32).all(|g| c.open(g).count() >= 1));

    // *At most one* holds of the program's own lattice.
    assert_eq!(c.alive_at_root, Some(Vec::new()));

    // The five relations are five components: a component is no group.
    assert_eq!(c.components.len(), 5);
    assert!(c.components.iter().all(|x| !c.groups.contains(x)));

    // Root's graph: its maximal cliques are many, and no row or column of
    // two or more open members is one of them.
    let root = c.root_cliques.as_ref().expect("root's pairs were read");
    assert!(root.cut.is_none() && root.cliques.len() > 36 * 10);
    let open_lines: Vec<Vec<u32>> = (0..c.groups.len() as u32)
        .map(|g| c.open(g).collect::<Vec<u32>>())
        .filter(|o| o.len() > 1)
        .collect();
    assert_eq!(open_lines.len(), 25);
    for line in &open_lines {
        assert!(!root.cliques.contains(line), "{:?}", texts(&c, line));
        assert!(
            root.cliques
                .iter()
                .any(|k| line.iter().all(|b| k.contains(b))),
            "{:?} excludes pairwise at root",
            texts(&c, line)
        );
    }
}

/// **Two encodings, one cover.** `zebra.ein` proposes the same 56
/// placements over one relation, with no relation to tell an attribute's
/// kind by, each in both orientations: the hrule's `(co-located X House-n)`
/// and its mirror `(co-located House-n X)`, which pays what a house owes and
/// joins `H` beside it (Q-M1f.570). Its groups, each candidate folded onto
/// its placement, are `zebra2.ein`'s under the renaming, an attribute's
/// houses and, at each house, the attributes **of one kind**, which no
/// candidate's text says.
///
/// **Each of them `2^k` times**: a candidate and its mirror exclude nothing
/// between them, and each excludes both orientations of every placement the
/// other excludes, so a group of `zebra2.ein`'s of `k` placements is `2^k`
/// groups here, one for each way round its members are written, each
/// holding a placement once: 352 groups, and no candidate in exactly two.
#[test]
fn one_puzzle_in_two_encodings_has_one_cover() {
    // A candidate's placement, `[x, House-n]`, whichever way round it is
    // written.
    let placement = |text: &str| -> [String; 2] {
        let p = parts(text);
        if p[1].starts_with("House-") {
            [p[2].to_string(), p[1].to_string()]
        } else {
            [p[1].to_string(), p[2].to_string()]
        }
    };
    let placements = |c: &Cover| -> BTreeSet<BTreeSet<[String; 2]>> {
        c.groups
            .iter()
            .map(|g| {
                let set: BTreeSet<[String; 2]> = g.iter().map(|&b| placement(c.text(b))).collect();
                assert_eq!(set.len(), g.len(), "a placement once in a group");
                set
            })
            .collect()
    };
    let (one, five) = (
        cover_of("examples/zebra.ein"),
        cover_of("examples/zebra2.ein"),
    );
    assert!(one.complete() && in_exactly_two(&five) && !in_exactly_two(&one));
    assert_eq!((one.n(), five.n()), (112, 56));
    assert_eq!(placements(&one), placements(&five));
    let blown: usize = five.groups.iter().map(|g| 1 << g.len()).sum();
    assert_eq!((one.groups.len(), blown), (352, 352));
    // A group is a line of the one relation only where it is written all one
    // way round, 72 of them, and of those 38 are: a house's line there holds
    // every kind of attribute.
    let one_way: BTreeSet<Set> = groups_of(&one)
        .into_iter()
        .filter(|g| {
            let ways: BTreeSet<bool> = g
                .iter()
                .map(|t| parts(t)[1].starts_with("House-"))
                .collect();
            ways.len() == 1
        })
        .collect();
    assert_eq!(one_way.len(), 72);
    let (groups, lines) = (groups_of(&one), lines(&one));
    assert!(groups.intersection(&lines).all(|g| one_way.contains(g)));
    assert_eq!(groups.intersection(&lines).count(), 38);
}

/// **One clue apart.** `zebra2-minus-15.ein` leaves 96 candidates and none
/// dead alone: its 46 groups are its rows and columns, every candidate in
/// two; and those of them that hold two or more of the 56 candidates
/// `zebra2.ein` has are, cut down to those candidates, `zebra2.ein`'s 36.
/// Root is read as far as its singles: the cover needs no pair of root's.
#[test]
fn two_programs_one_clue_apart_have_one_cover() {
    let minus = cover_by("examples/zebra2-minus-15.ein", &singles_alone());
    assert!(minus.complete() && in_exactly_two(&minus));
    assert_eq!((minus.n(), minus.groups.len()), (96, 46));
    assert_eq!(groups_of(&minus), lines(&minus));
    assert!(minus.marks.iter().all(|&m| m == Mark::AliveAlone));
    assert!(minus.root_cliques.is_none() && minus.alive_at_root.is_none());

    let whole = cover_by("examples/zebra2.ein", &singles_alone());
    let both: Set = whole.candidates.iter().map(|c| c.text.clone()).collect();
    let cut: BTreeSet<Set> = groups_of(&minus)
        .into_iter()
        .map(|g| g.intersection(&both).cloned().collect::<Set>())
        .filter(|g| g.len() > 1)
        .collect();
    assert_eq!(cut, groups_of(&whole));
}

/// **Letters and digits.** SEND + MORE = MONEY proposes `(val letter
/// digit)`: 18 groups, a letter's digits, eight of ten, and a digit's
/// letters, ten of eight, every candidate in two. `(val M 0)` and `(val S
/// 0)` are dead alone at root, by the puzzle's leading letters, and are
/// members all the same.
#[test]
fn a_cryptarithms_groups_are_its_letters_and_its_digits() {
    let c = cover_of("examples/beyond_arith/send_more_money.ein");
    assert!(c.complete() && in_exactly_two(&c));
    assert_eq!((c.n(), c.groups.len()), (80, 18));
    assert_eq!(groups_of(&c), lines(&c));
    let sizes = |k: usize| c.groups.iter().filter(|g| g.len() == k).count();
    assert_eq!((sizes(10), sizes(8)), (8, 10));
    let dead: Set = (0..c.n() as u32)
        .filter(|&b| c.marks[b as usize] == Mark::DeadAlone)
        .map(|b| c.text(b).to_string())
        .collect();
    assert_eq!(dead, Set::from(["(val M 0)".into(), "(val S 0)".into()]));
    assert_eq!(c.alive_at_root, Some(Vec::new()));
    // One component: every letter's row crosses every digit's column.
    assert_eq!(c.components.len(), 1);
}

/// **A conflict of three forms no group.** No pair excludes, at root or in
/// the theory, so there is no clique of two: three candidates alive alone,
/// each in no group, which is all a cover of pairs can say of them.
#[test]
fn a_conflict_of_three_is_in_no_group() {
    let c = cover_of("examples/lattice/02_genuine_3set_death.ein");
    assert!(c.complete());
    assert_eq!(c.groups, Vec::<Vec<u32>>::new());
    assert_eq!(c.in_no_group(), [0, 1, 2]);
    assert!(c.marks.iter().all(|&m| m == Mark::AliveAlone));
    assert_eq!(c.root_cliques.as_ref().map(|r| r.cliques.len()), Some(0));
    assert_eq!(c.steps, 0);
}

/// **What root owes, beside.** On the declared control every instance root
/// owes is licensed, and the alternatives of each are a group: every row is
/// owed by its value and every column by its house, so each of the 36
/// groups is one of which *some* member must hold. The cover did not read
/// the obligations to find them.
#[test]
fn an_owed_instances_alternatives_are_a_group_on_the_declared_control() {
    let rel = "examples/zebra2.ein";
    let c = cover_of(rel);
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
    let table = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast)
        .expect("root prepares")
        .expect("a monotone program that states an obligation");
    assert_eq!((table.instances.len(), table.licensed()), (36, 36));
    let owed: BTreeSet<u32> = table
        .instances
        .iter()
        .map(|i| {
            let alternatives = i.alternatives.as_ref().expect("licensed");
            match c.standing(alternatives) {
                Standing::Group(g) => g,
                other => panic!("{:?} stand as {other:?}", texts(&c, alternatives)),
            }
        })
        .collect();
    assert_eq!(owed.len(), 36, "each group is owed once");
}

// ── What a cover is, over the corpus ──────────────────────────────────

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

/// What one program's cover is.
#[derive(Default)]
struct Held {
    failures: Vec<String>,
    candidates: usize,
    groups: usize,
    in_none: usize,
    overlapping: bool,
    flagged: bool,
    steps: u64,
    root_steps: u64,
    /// Instances of L5's table, those of them that owe a group, grids, and
    /// the grids every group of which is owed.
    instances: usize,
    owed_groups: usize,
    grids: usize,
    bijections: usize,
}

/// **What *exactly one* is**, of one program: `x` read against `c`, and
/// the table against the one a route reads of the program loaded again.
/// `obligations` says whether the program loads an obligation rule: one
/// that loads none has no table, and is not loaded a second time to say so.
fn exactly_one_held(
    path: &Path,
    obligations: bool,
    c: &Cover,
    x: &ExactlyOne,
    fail: &mut dyn FnMut(String),
) {
    // The table is the routes'.
    let table = if obligations {
        loaded(path)
            .and_then(|(mut kb, mut terms, ast)| {
                ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast).ok()
            })
            .flatten()
    } else {
        None
    };
    match (&table, &x.owed.instances) {
        (None, Err(_)) => {}
        (Some(t), Ok(mine)) => {
            if t.instances.len() != mine.len() {
                fail(format!(
                    "L5's table has {} instances and the owner's {}",
                    t.instances.len(),
                    mine.len()
                ));
            }
            for (k, (theirs, mine)) in t.instances.iter().zip(mine).enumerate() {
                let same = match (&theirs.alternatives, &mine.alternatives) {
                    (Ok(a), Ok(b)) => a[..] == b[..],
                    (Err(_), Err(_)) => true,
                    _ => false,
                };
                if !same || theirs.excluded != mine.denied {
                    fail(format!("instance {k} is not L5's: {}", mine.text));
                }
            }
        }
        (t, mine) => fail(format!(
            "a table is read by one and not the other: {:?} / {:?}",
            t.as_ref().map(|t| t.instances.len()),
            mine.as_ref().map(Vec::len)
        )),
    }
    let table = x.owed.table();
    if x.owing.len() != table.len() {
        fail("not one standing an instance".to_string());
        return;
    }
    let mut owed_by: Vec<Vec<u32>> = vec![Vec::new(); c.groups.len()];
    for (k, (i, owing)) in table.iter().zip(&x.owing).enumerate() {
        let agrees = match (&i.alternatives, owing) {
            (Ok(a), Owing::Group(g)) => {
                owed_by[*g as usize].push(k as u32);
                c.standing(a) == Standing::Group(*g) && c.members(*g) == &a[..]
            }
            (Ok(a), Owing::Inside(groups)) => c.standing(a) == Standing::Inside(groups.clone()),
            (Ok(a), Owing::NotExcluding { .. }) => {
                matches!(c.standing(a), Standing::NotExcluding { .. })
            }
            (Ok(a), Owing::One(b)) => a[..] == [*b],
            (Ok(a), Owing::None) => a.is_empty(),
            (Err(_), Owing::Unlicensed { payers, about }) => match (payers, about) {
                (Some(p), Some(g)) => {
                    c.members(*g) == &p[..] && x.about[*g as usize].contains(&(k as u32))
                }
                (None, Some(_)) => false,
                _ => true,
            },
            _ => false,
        };
        if !agrees {
            fail(format!(
                "instance {k} does not stand as it is said to: {} as {owing:?}",
                i.text
            ));
        }
        // A licensed instance's witnesses are its alternatives and what
        // root denies, and no other fact.
        if let (Ok(a), Ok(ws)) = (&i.alternatives, &i.witnesses) {
            use structure::WitnessIs;
            let mut candidates: Vec<u32> = ws
                .iter()
                .filter_map(|w| match w.is {
                    WitnessIs::Candidate(b) => Some(b),
                    _ => None,
                })
                .collect();
            candidates.sort_unstable();
            let denied = ws.iter().filter(|w| w.is == WitnessIs::Denied).count();
            if candidates != *a
                || denied != i.denied as usize
                || candidates.len() + denied != ws.len()
            {
                fail(format!(
                    "instance {k}'s witnesses are not its alternatives and what root denies"
                ));
            }
        }
        // An unlicensed one has a witness that is neither, or a reason that
        // names none.
        if let (Err(why), Ok(ws)) = (&i.alternatives, &i.witnesses) {
            use structure::WitnessIs;
            let unproposed = ws.iter().any(|w| w.is == WitnessIs::Unproposed);
            if why == structure::UNPROPOSED && !unproposed {
                fail(format!(
                    "instance {k} is said to have a witness no candidate proposes, and has none"
                ));
            }
        }
    }
    if owed_by != x.owed_by {
        fail("a group is not owed by the instances whose alternatives are its members".to_string());
    }
    let owed: Vec<u32> = (0..c.groups.len() as u32)
        .filter(|&g| x.is_owed(g))
        .collect();
    let mut both = owed.clone();
    both.extend(x.unowed());
    both.sort_unstable();
    if owed != x.owed_groups() || both != (0..c.groups.len() as u32).collect::<Vec<_>>() {
        fail(
            "the owed groups and the groups nothing owes are not all the groups, once".to_string(),
        );
    }
    // The grids: each a component, its two families each holding every
    // member exactly once, and two groups sharing at most one member.
    let grids = &x.grids;
    if *grids != c.grids() {
        fail("the grids are not the cover's".to_string());
    }
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    for (k, g) in grids.grids.iter().enumerate() {
        let members = &c.components[g.component as usize];
        seen.insert(g.component);
        for (f, family) in g.families.iter().enumerate() {
            let mut held: Vec<u32> = family
                .iter()
                .flat_map(|&g| c.members(g).iter().copied())
                .collect();
            held.sort_unstable();
            if family.is_empty() || held != *members {
                fail(format!(
                    "grid {k}'s family {f} does not hold every member once"
                ));
            }
            if family
                .iter()
                .any(|&g| grids.of_group[g as usize] != Some((k as u32, f as u8)))
            {
                fail(format!(
                    "grid {k}'s family {f} holds a group that does not name it"
                ));
            }
        }
        for &a in &g.families[0] {
            for &b in &g.families[1] {
                let shared = c
                    .members(a)
                    .iter()
                    .filter(|m| c.members(b).contains(m))
                    .count();
                if shared > 1 {
                    fail(format!(
                        "grid {k}'s groups {a} and {b} share {shared} members"
                    ));
                }
            }
        }
        if g.families[0].first() > g.families[1].first() {
            fail(format!("grid {k}'s first family is not its lowest group's"));
        }
        let share = x.share(k as u32);
        for (f, family) in g.families.iter().enumerate() {
            let owed = family.iter().filter(|&&g| x.is_owed(g)).count() as u32;
            if share[f].groups != family.len() as u32 || share[f].owed != owed {
                fail(format!(
                    "grid {k}'s family {f} is not owed what it is said to be"
                ));
            }
        }
    }
    for &(component, _) in &grids.not {
        if !seen.insert(component) {
            fail(format!("component {component} is a grid and is not one"));
        }
    }
    if seen.len() != c.components.len() {
        fail("a component is neither a grid nor said not to be".to_string());
    }
    for (g, of) in grids.of_group.iter().enumerate() {
        if let Some((k, f)) = of
            && !grids.grids[*k as usize].families[*f as usize].contains(&(g as u32))
        {
            fail(format!("group {g} names a grid that does not hold it"));
        }
    }
}

/// `None` where the program does not load or the engine refuses it: it has
/// no layer to cover.
fn held(path: &Path) -> Option<Held> {
    let rel = rel_of(path);
    let (mut kb, mut terms, ast) = loaded(path)?;
    let obligations = !kb.program().obligations.is_empty();
    let (c, x) =
        structure::exactly_one(&mut kb, &mut terms, &ast, &CoverOptions::default()).ok()?;
    let n = c.n() as u32;
    let mut out = Held {
        instances: x.owed.table().len(),
        owed_groups: x.owed_groups().len(),
        grids: x.grids.grids.len(),
        bijections: (0..x.grids.grids.len() as u32)
            .filter(|&k| x.is_bijection(k))
            .count(),
        candidates: c.n(),
        groups: c.groups.len(),
        in_none: c.in_no_group().len(),
        overlapping: c.member_of.iter().any(|g| g.len() > 1),
        flagged: !c.monotone,
        steps: c.steps,
        root_steps: c.root_cliques.as_ref().map_or(0, |r| r.steps),
        ..Held::default()
    };
    let mut fail = |what: String| out.failures.push(format!("{rel}: {what}"));
    if !c.complete() {
        fail(format!(
            "the cover is not whole: cut {:?}, stopped {:?}",
            c.cut, c.stopped
        ));
    }
    match &c.root_cliques {
        None => fail("root's pairs were not all read".to_string()),
        Some(r) if r.cut.is_some() => fail(format!("the bound cut root's cliques: {:?}", r.cut)),
        Some(_) => {}
    }
    // *At most one* is the program's: no pair of the theory is alive at
    // root.
    if let Some(alive) = &c.alive_at_root {
        for &(a, b) in alive {
            fail(format!(
                "{{{}, {}}} is the theory's pair and alive at root",
                c.text(a),
                c.text(b)
            ));
        }
    }
    let pairs: BTreeSet<(u32, u32)> = c.excluded.iter().copied().collect();
    let joined = |a: u32, b: u32| pairs.contains(&(a.min(b), a.max(b)));
    let mut inside: BTreeSet<(u32, u32)> = BTreeSet::new();
    for (g, members) in c.groups.iter().enumerate() {
        if members.len() < 2 || !members.is_sorted() {
            fail(format!("group {g} is no ascending set of two or more"));
        }
        for (k, &a) in members.iter().enumerate() {
            for &b in &members[k + 1..] {
                if !joined(a, b) || !c.excludes(a, b) {
                    fail(format!(
                        "group {g} holds {} and {}, which the theory does not exclude",
                        c.text(a),
                        c.text(b)
                    ));
                }
                inside.insert((a, b));
            }
            if !c.groups_of(a).contains(&(g as u32)) {
                fail(format!(
                    "group {g} holds {}, whose groups do not name it",
                    c.text(a)
                ));
            }
        }
        if let Some(more) =
            (0..n).find(|b| !members.contains(b) && members.iter().all(|&a| joined(a, *b)))
        {
            fail(format!(
                "group {g} is not maximal: {} excludes every member",
                c.text(more)
            ));
        }
        if c.standing(members) != Standing::Group(g as u32) {
            fail(format!("group {g} does not stand as itself"));
        }
        if !c
            .components
            .iter()
            .any(|x| members.iter().all(|b| x.contains(b)))
        {
            fail(format!("group {g} is in no one component"));
        }
    }
    if !c.groups.is_sorted() {
        fail("the groups are not in order".to_string());
    }
    for b in 0..n {
        for &g in c.groups_of(b) {
            if !c.members(g).contains(&b) {
                fail(format!(
                    "{} names group {g}, which does not hold it",
                    c.text(b)
                ));
            }
        }
    }
    // Every pair of the theory is inside a group, so two candidates exclude
    // by the theory exactly where they share one.
    if inside != pairs {
        fail(format!(
            "{} of the theory's {} pairs are inside no group",
            pairs.difference(&inside).count(),
            pairs.len()
        ));
    }
    let with_pair: BTreeSet<u32> = pairs.iter().flat_map(|&(a, b)| [a, b]).collect();
    let in_none: Vec<u32> = (0..n).filter(|b| !with_pair.contains(b)).collect();
    if c.in_no_group() != in_none {
        fail("the candidates in no group are not those no pair of the theory holds".to_string());
    }
    if c.killed_alone.iter().any(|b| with_pair.contains(b)) {
        fail("a candidate the theory kills alone is in a pair of it".to_string());
    }
    exactly_one_held(path, obligations, &c, &x, &mut fail);
    Some(out)
}

/// **What a cover is**, on every program the sweep reads: the bound cut
/// nothing, of the groups or of root's cliques; every group is a maximal
/// clique of the theory's pairs, and every pair of the theory is inside
/// one; the two maps are each other's inverse; and no pair of the theory is
/// alive at root.
///
/// **And what *exactly one* is**, of the same reading: what root owes is
/// L5's table as a route reads it; each instance stands as its alternatives
/// stand to the cover; a group is owed by the instances whose alternatives
/// are exactly its members; and each grid is a component whose two families
/// each hold every member once.
#[test]
fn every_cover_is_whole_and_its_groups_are_the_maximal_cliques() {
    let files = files();
    let results = ein_corpus::pool::map(
        "cover::held",
        &files,
        |p| (rel_of(p), String::new()),
        |p| held(p),
    );
    let mut failures: Vec<String> = Vec::new();
    let (mut read, mut with_groups, mut groups, mut overlapping) = (0usize, 0usize, 0usize, 0);
    let (mut in_none, mut flagged, mut steps) = (0usize, 0usize, 0u64);
    let (mut instances, mut owed, mut grids, mut bijections) = (0usize, 0usize, 0usize, 0usize);
    let mut longest = String::new();
    for (file, got) in files.iter().zip(results) {
        let Some(h) = got else { continue };
        if h.steps.max(h.root_steps) > steps {
            longest = rel_of(file);
        }
        instances += h.instances;
        owed += h.owed_groups;
        grids += h.grids;
        bijections += h.bijections;
        failures.extend(h.failures);
        read += 1;
        with_groups += usize::from(h.groups > 0);
        groups += h.groups;
        overlapping += usize::from(h.overlapping);
        in_none += usize::from(h.candidates > 0 && h.in_none > 0);
        flagged += usize::from(h.flagged);
        steps = steps.max(h.steps).max(h.root_steps);
    }
    assert!(
        failures.is_empty(),
        "{} place(s) where a cover is not what it says:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "cover: {read} programs read, {with_groups} with a group ({groups} groups), \
         {overlapping} with a candidate in two or more, {in_none} with a candidate in none, \
         {flagged} flagged; the longest enumeration {steps} calls of {}, {longest}'s; exactly \
         one: {instances} instances root owes, {owed} owed groups, {grids} grids, {bijections} \
         of them bijections",
        structure::CLIQUE_STEPS
    );
    // Not vacuous, and well inside the bound: the longest enumeration is of
    // the maximal cliques of root's graph on the one-relation zebra, every
    // placement in both orientations, and is under a quarter of it.
    assert!(read >= 300, "only {read} programs read");
    assert!(
        with_groups >= 40,
        "only {with_groups} programs with a group"
    );
    assert!(
        overlapping >= 15,
        "only {overlapping} with overlapping groups"
    );
    assert!(flagged >= 10, "only {flagged} flagged programs read");
    assert!(
        steps * 4 < structure::CLIQUE_STEPS,
        "{longest}: {steps} calls"
    );
    assert!(instances >= 150, "only {instances} instances root owes");
    assert!(owed >= 60, "only {owed} owed groups");
    assert!(grids >= 20, "only {grids} grids");
    assert!(bijections >= 5, "only {bijections} bijections");
}

// ── One cover by every entry ──────────────────────────────────────────

/// What a cover says, but for what reading root's pairs adds.
fn said(c: &Cover) -> String {
    format!(
        "{:?}",
        (
            c.candidates.iter().map(|x| &x.text).collect::<Vec<_>>(),
            (&c.marks, &c.killed_alone, &c.excluded),
            (&c.groups, &c.member_of, &c.components),
            (c.steps, c.cut, c.monotone, c.small_monotone, &c.stopped),
            c.small_work,
        )
    )
}

/// And what reading root's pairs adds.
fn said_of_root(c: &Cover) -> String {
    format!("{:?}", (&c.alive_at_root, &c.root_cliques, c.root_work))
}

/// **One cover by every entry.** Over the program's prepared space with
/// root's pairs, the same with root read as far as its singles, and off the
/// theory's share read whole: one set of groups, one pair of maps, one mark
/// a candidate. Reading root's pairs adds root's cliques and the check that
/// the theory's pairs are dead at root, and changes nothing else. On a
/// monotone program, on a flagged one, and on one whose root is dead.
#[test]
fn the_cover_is_the_same_by_every_entry() {
    for rel in [
        "examples/zebra2-hints.ein",
        "examples/branching/08_hypothesis_relation_whitelist.ein",
        "examples/beyond_arith/cryptarithm.ein",
        "examples/saturation/square-unique/cul-de-sac.ein",
        "examples/branching/01_saturate_only.ein",
    ] {
        let with_pairs = cover_of(rel);
        let alone = cover_by(rel, &singles_alone());
        assert_eq!(said(&with_pairs), said(&alone), "{rel}");
        assert!(
            with_pairs.root_cliques.is_some() && alone.root_cliques.is_none(),
            "{rel}"
        );
        assert!(alone.alive_at_root.is_none(), "{rel}");
        // The singles cost less than the layer: no pair's node is read.
        assert!(
            alone.root_work.saturations <= with_pairs.root_work.saturations,
            "{rel}"
        );

        let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
        let t = structure::theory(&mut kb, &mut terms, &ast, &TheoryOptions::default())
            .expect("the theory's share");
        let off_theory = t.cover().expect("the small KB has a layer");
        assert_eq!(said(&with_pairs), said(&off_theory), "{rel}");
        assert_eq!(
            said_of_root(&with_pairs),
            said_of_root(&off_theory),
            "{rel}"
        );
    }
    // A program none of whose pairs root needs to read has groups all the
    // same: the pairs are the small KB's.
    let dead = cover_of("examples/branching/01_saturate_only.ein");
    assert!(!dead.groups.is_empty());
    assert!(dead.marks.iter().all(|&m| m == Mark::DeadAlone));
    assert!((0..dead.groups.len() as u32).all(|g| dead.open(g).count() == 0));
    // And a flagged program says what its cover is about.
    let flagged = cover_of("examples/lattice/06_friends_tables.ein");
    assert!(!flagged.monotone && !flagged.groups.is_empty());
    assert_eq!(flagged.domain(), Some(structure::COVER_FLAGGED));
    assert_eq!(cover_of("examples/zebra2-hints.ein").domain(), None);
}

/// **The job count moves nothing**: the cover of a monotone program and of
/// a flagged one, at one thread and at four.
#[test]
fn the_cover_is_the_same_at_any_job_count() {
    for rel in [
        "examples/zebra2-hints.ein",
        "examples/branching/08_hypothesis_relation_whitelist.ein",
        "examples/lattice/06_friends_tables.ein",
    ] {
        let one = cover_of(rel);
        let four = cover_by(
            rel,
            &CoverOptions {
                layer: structure::Options {
                    jobs: 4,
                    ..structure::Options::default()
                },
                ..CoverOptions::default()
            },
        );
        assert!(!one.groups.is_empty(), "{rel}: the cover has groups");
        assert_eq!(said(&one), said(&four), "{rel}");
        assert_eq!(said_of_root(&one), said_of_root(&four), "{rel}");
    }
}
