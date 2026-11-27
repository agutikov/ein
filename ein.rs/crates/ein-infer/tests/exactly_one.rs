//! ***Exactly one* is held by its controls.**
//!
//! [`ein_infer::structure::ExactlyOne`] reads what root owes against the
//! cover: a group whose members are exactly a licensed instance's
//! alternatives is *exactly one*, and a component of the theory's exclusion
//! graph whose groups are two families, every member in one group of each,
//! is a grid. Neither reads a relation or a declaration, so what they come
//! to is held from outside, by the programs the structure was read for:
//!
//! 1. **the declared control**: `zebra2.ein` declares five relations
//!    bijective. Five grids, one a relation, every row and every column an
//!    owed group: five bijections;
//! 2. **one relation, both orientations, no grid**: `zebra.ein` states the
//!    same puzzle over one relation through `std.slots`, and its `H` holds
//!    each placement in both orientations, the hypothesis rule's and the mirror that
//!    pays what a house owes. Every instance root owes is licensed and owes
//!    a group, the rows in one orientation and the columns in the other; but
//!    a candidate and its mirror exclude nothing between them, so a member
//!    is in more than two groups, and the five components, `zebra2.ein`'s
//!    five grids with each placement twice, are no grid;
//! 3. **one clue apart**: `zebra2-minus-15.ein`, five bijections over more
//!    candidates;
//! 4. **letters and digits**: one grid of eight by ten, the letters owed
//!    and the digits not;
//! 5. **where there is no *must***: a program that states no obligation, a
//!    conflict of three, a program the static check flags;
//! 6. **a member in more than two groups** is in no grid.
//!
//! What *exactly one* is on every corpus program is held with the cover, in
//! one reading (`ein-infer/tests/cover.rs`), and its two invariances with
//! the structure's (`structure_invariance.rs`).
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test exactly_one
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::structure::{
    self, Cover, CoverOptions, ExactlyOne, Mark, NoTable, NotAGrid, Owing, WitnessIs,
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

fn read_by(rel: &str, opts: &CoverOptions) -> (Cover, ExactlyOne) {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("the program loads");
    structure::exactly_one(&mut kb, &mut terms, &ast, opts).expect("the cover and what is owed")
}

fn read(rel: &str) -> (Cover, ExactlyOne) {
    read_by(rel, &CoverOptions::default())
}

/// A `(rel x y)` fact's three names.
fn parts(text: &str) -> Vec<&str> {
    text.trim_matches(|c| c == '(' || c == ')')
        .split_whitespace()
        .collect()
}

type Set = BTreeSet<String>;

fn texts(c: &Cover, set: &[u32]) -> Set {
    set.iter().map(|&b| c.text(b).to_string()).collect()
}

/// A grid's two families' sizes, the smaller first.
fn shape(x: &ExactlyOne, grid: usize) -> (usize, usize) {
    let f = &x.grids.grids[grid].families;
    (f[0].len().min(f[1].len()), f[0].len().max(f[1].len()))
}

/// The rules whose instances owe the groups of `family`.
fn owing_rules(x: &ExactlyOne, family: &[u32]) -> BTreeSet<String> {
    family
        .iter()
        .flat_map(|&g| x.owed_by[g as usize].iter())
        .map(|&k| x.owed.table()[k as usize].rule.clone())
        .collect()
}

/// The grids as sets of placements, each a pair of names with the relation
/// left out: what two encodings of one puzzle have in common.
fn placements(c: &Cover, x: &ExactlyOne) -> BTreeSet<BTreeSet<[String; 2]>> {
    x.grids
        .grids
        .iter()
        .map(|g| {
            c.components[g.component as usize]
                .iter()
                .map(|&b| {
                    let p = parts(c.text(b));
                    [p[1].to_string(), p[2].to_string()]
                })
                .collect()
        })
        .collect()
}

/// **The declared control.** `zebra2.ein` declares `color-loc`,
/// `drink-loc`, `nation-loc`, `pet-loc` and `smoke-loc` bijective. Found
/// with no relation read: five grids, each the candidates of one of the
/// five, three by three twice and four by four three times; every one of
/// the 36 groups owed by exactly one instance, whose alternatives are the
/// group's members, those dead alone at root among them; so every grid is a
/// bijection. In each grid one family is owed by the rule that owes a value
/// its house and the other by the rule that owes a house its value.
#[test]
fn a_declared_bijections_rows_and_columns_are_owed_groups() {
    let (c, x) = read("examples/zebra2.ein");
    assert!(c.complete() && c.monotone);
    assert_eq!(x.owed.root_owes, 36);
    assert_eq!(x.owed.table().len(), 36);
    assert!(x.owing.iter().all(|o| matches!(o, Owing::Group(_))));
    assert_eq!(x.owed_groups().len(), 36);
    assert_eq!(
        (x.unowed(), x.not_groups(), x.unlicensed()),
        (vec![], vec![], vec![])
    );
    assert!(x.owed_by.iter().all(|by| by.len() == 1));

    // The correspondence, in full: the alternatives are the members, the
    // members alive alone are the group's open ones, and every fact that
    // can witness the instance is a member or denied at root.
    let mut dead_members = 0;
    for (i, owing) in x.owed.table().iter().zip(&x.owing) {
        let Owing::Group(g) = owing else {
            unreachable!()
        };
        let alternatives = i.alternatives.as_ref().expect("licensed");
        assert_eq!(c.members(*g), &alternatives[..]);
        let open: Vec<u32> = c.open(*g).collect();
        assert!(!open.is_empty(), "{}", i.text);
        dead_members += alternatives.len() - open.len();
        let witnesses = i.witnesses.as_ref().expect("its witnesses");
        assert!(witnesses.iter().all(|w| w.is != WitnessIs::Unproposed));
        // A value has five houses and a house five values: every instance
        // has five witnesses, and root denies those that are no member.
        assert_eq!(witnesses.len(), 5, "{}", i.text);
        assert_eq!(alternatives.len() + i.denied as usize, 5, "{}", i.text);
    }
    assert!(dead_members > 0, "a member dead alone is an alternative");

    // The grids.
    assert_eq!(x.grids.grids.len(), 5);
    assert_eq!(x.grids.not, []);
    let mut shapes: Vec<(usize, usize)> = (0..5).map(|k| shape(&x, k)).collect();
    shapes.sort_unstable();
    assert_eq!(shapes, [(3, 3), (3, 3), (4, 4), (4, 4), (4, 4)]);
    let mut relations: Set = Set::new();
    for (k, g) in x.grids.grids.iter().enumerate() {
        assert!(x.is_bijection(k as u32), "grid {k}");
        let of: Set = c.components[g.component as usize]
            .iter()
            .map(|&b| parts(c.text(b))[0].to_string())
            .collect();
        assert_eq!(of.len(), 1, "grid {k} is of one relation: {of:?}");
        relations.extend(of);
        // One family is a value's houses and the other a house's values,
        // each owed by its own rule.
        let rules = [0, 1].map(|f| owing_rules(&x, &g.families[f]));
        assert!(rules.iter().all(|r| r.len() == 1), "{rules:?}");
        let both: Set = rules.iter().flatten().cloned().collect();
        assert_eq!(
            both,
            Set::from([
                "std.algebra.surjective-owed".to_string(),
                "std.algebra.total-owed".to_string()
            ])
        );
    }
    assert_eq!(
        relations,
        [
            "color-loc",
            "drink-loc",
            "nation-loc",
            "pet-loc",
            "smoke-loc"
        ]
        .map(String::from)
        .into()
    );
}

/// A `(co-located x y)` candidate's placement, `[attribute, house]`,
/// whichever way round it is written: a fact and its mirror are one
/// placement.
fn placement(text: &str) -> [String; 2] {
    let p = parts(text);
    if p[1].starts_with("House-") {
        [p[2].to_string(), p[1].to_string()]
    } else {
        [p[1].to_string(), p[2].to_string()]
    }
}

/// The candidate written the other way round.
fn mirror(text: &str) -> String {
    let p = parts(text);
    format!("({} {} {})", p[0], p[2], p[1])
}

/// **One relation, both orientations, no grid.** `zebra.ein` proposes the
/// same 56 placements over one relation, each in both orientations: the
/// hypothesis rule writes `(co-located v House-n)`, and `slot-owed-fill`, *a house has
/// no attribute of a kind yet*, writes what pays it, the mirror
/// `(co-located House-n v)`, which joins `H` beside the hypothesis rule's
/// (Q-M1f.570): 112 candidates, the 42 of them dead alone at root 21
/// placements twice, `zebra2.ein`'s 21 dead alone. `symmetric` brings each
/// of a candidate and its mirror from the other, so the two exclude nothing
/// between them, and each excludes both orientations of whatever the other
/// placement excludes.
///
/// Root owes 36 instances and **every one is licensed and owes a group**:
/// every fact that can witness one is a candidate or denied at root. The 18
/// of `slot-owed-room`, *an attribute is in no house yet*, each owe an
/// attribute's houses written the hypothesis rule's way; the 18 of `slot-owed-fill`
/// each owe a house's attributes of one kind written the mirror's way.
/// Folded onto their placements the 36 owed groups are `zebra2.ein`'s 36,
/// each once, found though one relation holds them all and no candidate's
/// text says an attribute's kind.
///
/// **And no grid**: a member of a line of `k` placements is in `2^(k−1)` of
/// the groups folded from it, one for each orientation of the line's other
/// members, so a member is in more than two groups. The five components are
/// `zebra2.ein`'s five grids under the renaming, each placement twice, and
/// each is no grid, its members in at most 16 groups where the grid is four
/// by four and 8 where it is three by three.
///
/// **What is true of the mirrors that pay a house**: one witness of
/// `slot-owed-fill` the mirror of each of the hypothesis rule's 56 candidates, and
/// each a candidate. By the theory alone each is brought by its own node
/// and its mirror's, and by no other. At root the 35 alive alone are in
/// both closures, and the 21 dead alone are held by no single's node.
#[test]
fn one_relation_owes_a_group_in_each_orientation_and_holds_no_grid() {
    let (c, x) = read("examples/zebra.ein");
    let (c2, x2) = read("examples/zebra2.ein");
    assert!(
        c.candidates
            .iter()
            .all(|k| parts(&k.text)[0] == "co-located")
    );
    // The 56 placements, each twice.
    let texts: BTreeSet<&str> = c.candidates.iter().map(|k| k.text.as_str()).collect();
    assert_eq!(texts.len(), 112);
    assert!(texts.iter().all(|t| texts.contains(mirror(t).as_str())));
    let folded: BTreeSet<[String; 2]> = texts.iter().map(|t| placement(t)).collect();
    let zebra2: BTreeSet<[String; 2]> = c2
        .candidates
        .iter()
        .map(|k| {
            let p = parts(&k.text);
            [p[1].to_string(), p[2].to_string()]
        })
        .collect();
    assert_eq!(folded, zebra2, "one universe, under the renaming");
    let dead: BTreeSet<u32> = (0..c.n() as u32)
        .filter(|&b| c.marks[b as usize] == Mark::DeadAlone)
        .collect();
    assert_eq!(dead.len(), 42);
    let dead_folded: BTreeSet<[String; 2]> = dead.iter().map(|&b| placement(c.text(b))).collect();
    let dead2: BTreeSet<[String; 2]> = (0..c2.n() as u32)
        .filter(|&b| c2.marks[b as usize] == Mark::DeadAlone)
        .map(|b| {
            let p = parts(c2.text(b));
            [p[1].to_string(), p[2].to_string()]
        })
        .collect();
    assert_eq!(
        dead_folded, dead2,
        "dead alone: zebra2.ein's, under the renaming"
    );
    let dead_texts: BTreeSet<&str> = dead.iter().map(|&b| c.text(b)).collect();
    assert!(
        dead_texts
            .iter()
            .all(|t| dead_texts.contains(mirror(t).as_str())),
        "a placement is dead alone in both orientations or in neither"
    );

    // Every instance licensed, and each owes a group in one orientation.
    assert_eq!((x.owed.root_owes, x.owed.table().len()), (36, 36));
    assert_eq!(x.owed_groups().len(), 36);
    assert_eq!(x.unowed().len(), 316);
    assert_eq!((x.unlicensed(), x.not_groups()), (vec![], vec![]));
    assert!(x.owed_by.iter().all(|by| by.len() <= 1));
    let mut by_rule: BTreeMap<String, usize> = BTreeMap::new();
    for (i, owing) in x.owed.table().iter().zip(&x.owing) {
        let Owing::Group(g) = owing else {
            panic!("{} stands as {owing:?}", i.text)
        };
        let alternatives = i.alternatives.as_ref().expect("licensed");
        assert_eq!(c.members(*g), &alternatives[..]);
        let mirrored = i.rule == "std.slots.slot-owed-fill";
        assert!(
            mirrored || i.rule == "std.slots.slot-owed-room",
            "{}",
            i.rule
        );
        assert!(
            alternatives
                .iter()
                .all(|&b| parts(c.text(b))[1].starts_with("House-") == mirrored),
            "{}",
            i.text
        );
        let witnesses = i.witnesses.as_ref().expect("its witnesses");
        assert!(witnesses.iter().all(|w| w.is != WitnessIs::Unproposed));
        *by_rule.entry(i.rule.clone()).or_default() += 1;
    }
    assert_eq!(by_rule.values().collect::<Vec<_>>(), [&18, &18]);
    let owed: BTreeSet<BTreeSet<[String; 2]>> = x
        .owed_groups()
        .iter()
        .map(|&g| c.members(g).iter().map(|&b| placement(c.text(b))).collect())
        .collect();
    let lines: BTreeSet<BTreeSet<[String; 2]>> = c2
        .groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|&b| {
                    let p = parts(c2.text(b));
                    [p[1].to_string(), p[2].to_string()]
                })
                .collect()
        })
        .collect();
    assert_eq!(owed, lines, "the owed groups are zebra2.ein's, folded");

    // No grid: the five components are zebra2.ein's grids, each placement
    // twice, and a member is in more than two groups.
    assert_eq!(x.grids.grids, []);
    let mut most: Vec<u32> = x
        .grids
        .not
        .iter()
        .map(|&(_, why)| match why {
            NotAGrid::MoreThanTwo { most } => most,
            other => panic!("a component is no grid for {other:?}"),
        })
        .collect();
    most.sort_unstable();
    assert_eq!(most, [8, 8, 16, 16, 16]);
    let components: BTreeSet<BTreeSet<[String; 2]>> = c
        .components
        .iter()
        .map(|k| {
            let set: BTreeSet<[String; 2]> = k.iter().map(|&b| placement(c.text(b))).collect();
            assert_eq!(set.len() * 2, k.len(), "each placement twice");
            set
        })
        .collect();
    assert_eq!(components, placements(&c2, &x2));

    // The mirrors that pay a house, by the nodes.
    let (mut mirrored, mut brought, mut by_nothing) = (BTreeSet::new(), 0, 0);
    for i in x.owed.table() {
        if i.rule != "std.slots.slot-owed-fill" {
            continue;
        }
        for w in i.witnesses.as_ref().expect("its witnesses") {
            match w.is {
                WitnessIs::Unproposed => panic!("{} is proposed by nothing", w.text),
                WitnessIs::Denied => {
                    assert!(w.held_at_root.is_empty() && w.held_in_theory.is_empty());
                }
                WitnessIs::Candidate(b) => {
                    // By the theory: its own node and its mirror's, and no
                    // other candidate's.
                    let held: BTreeSet<String> = w
                        .held_in_theory
                        .iter()
                        .map(|&h| c.text(h).to_string())
                        .collect();
                    assert_eq!(
                        held,
                        BTreeSet::from([w.text.clone(), mirror(&w.text)]),
                        "{}",
                        w.text
                    );
                    assert!(mirrored.insert(mirror(&w.text)), "{}", w.text);
                    // At root: in both closures where it is alive alone, and
                    // in no single's where it is dead alone.
                    if dead.contains(&b) {
                        assert!(w.held_at_root.is_empty(), "{}", w.text);
                        by_nothing += 1;
                    } else {
                        let at_root: BTreeSet<String> = w
                            .held_at_root
                            .iter()
                            .map(|&h| c.text(h).to_string())
                            .collect();
                        assert!(
                            at_root.contains(&w.text) && at_root.contains(&mirror(&w.text)),
                            "{}",
                            w.text
                        );
                        brought += 1;
                    }
                }
            }
        }
    }
    let guessed: BTreeSet<String> = texts
        .iter()
        .filter(|t| !parts(t)[1].starts_with("House-"))
        .map(|t| t.to_string())
        .collect();
    assert_eq!(
        mirrored, guessed,
        "one witness the mirror of each of the hypothesis rule's"
    );
    assert_eq!((brought, by_nothing), (35, 21));
}

/// **One clue apart.** `zebra2-minus-15.ein` leaves 96 candidates, none
/// dead alone: five bijections, five by five three times and four by four
/// twice, every one of its 46 groups owed. Root is read as far as its
/// singles, which is all the reading needs.
#[test]
fn an_under_determined_puzzle_has_its_five_bijections() {
    let (c, x) = read_by(
        "examples/zebra2-minus-15.ein",
        &CoverOptions {
            root_pairs: false,
            ..CoverOptions::default()
        },
    );
    assert_eq!((c.n(), c.groups.len()), (96, 46));
    assert_eq!(x.owed_groups().len(), 46);
    assert_eq!(x.grids.grids.len(), 5);
    let mut shapes: Vec<(usize, usize)> = (0..5).map(|k| shape(&x, k)).collect();
    shapes.sort_unstable();
    assert_eq!(shapes, [(4, 4), (4, 4), (5, 5), (5, 5), (5, 5)]);
    assert!((0..5).all(|k| x.is_bijection(k)));
    assert!(x.owed.singles_read && x.owed.theory_read);
}

/// **Letters and digits.** SEND + MORE = MONEY is one grid, eight groups
/// of ten against ten groups of eight. A letter owes a digit: the eight
/// groups of ten are owed, each by one instance. Nothing owes a digit a
/// letter, there being ten digits and eight letters: the ten groups of
/// eight are *at most one*, and the grid is no bijection.
#[test]
fn a_cryptarithms_letters_are_owed_and_its_digits_are_not() {
    let (c, x) = read("examples/beyond_arith/send_more_money.ein");
    assert_eq!(x.grids.grids.len(), 1);
    assert_eq!(x.grids.not, []);
    assert_eq!(shape(&x, 0), (8, 10));
    let families = &x.grids.grids[0].families;
    let (letters, digits) = if families[0].len() == 8 {
        (&families[0], &families[1])
    } else {
        (&families[1], &families[0])
    };
    assert!(letters.iter().all(|&g| c.members(g).len() == 10));
    assert!(digits.iter().all(|&g| c.members(g).len() == 8));
    assert_eq!(x.owed_groups(), *letters);
    assert_eq!(x.unowed(), *digits);
    assert!(x.owed_by.iter().all(|by| by.len() <= 1));
    assert_eq!((x.owed.table().len(), x.unlicensed().len()), (8, 0));
    assert!(!x.is_bijection(0));
    // Every group nothing owes has an open member: at most one of them is
    // a statement about states the search visits.
    assert!(digits.iter().all(|&g| c.open(g).count() >= 6));
}

/// **Where there is no *must*.** A program that states no obligation has
/// no table, so nothing is owed whatever its groups; a conflict of three
/// has no group to owe; and a program the static check flags has no table
/// either, L5 being read on a monotone program.
#[test]
fn a_program_with_no_table_owes_no_group() {
    // The blind enumerator's: ten groups, each a component of its own, all
    // of members dead alone.
    let (c, x) = read("examples/branching/06_lookahead_on.ein");
    assert_eq!(x.owed.instances, Err(NoTable::NoObligation));
    assert_eq!((c.groups.len(), x.unowed().len()), (10, 10));
    assert!(x.owing.is_empty() && x.grids.grids.is_empty());
    assert_eq!(x.grids.not.len(), 10);
    assert!(
        x.grids
            .not
            .iter()
            .all(|&(_, why)| why == NotAGrid::OneGroup)
    );
    assert!((0..10).all(|g| c.open(g).count() == 0));

    let (c, x) = read("examples/lattice/02_genuine_3set_death.ein");
    assert!(c.groups.is_empty() && x.grids.grids.is_empty() && x.grids.not.is_empty());
    assert_eq!(x.owed.instances, Err(NoTable::NoObligation));

    let (c, x) = read("examples/lattice/06_friends_tables.ein");
    assert!(!c.monotone);
    assert_eq!(x.owed.instances, Err(NoTable::Flagged));
    assert_eq!(x.unowed().len(), c.groups.len());
}

/// **A member in more than two groups is in no grid.** A rule that seats
/// two friends at one table ties a friend's seats to another's, so a
/// candidate is in nine groups, eleven or thirteen: one component, and no
/// grid. The program owes every friend a seat, and `H` holds every seat of
/// every friend, 25: the hypothesis rule proposes the seats at a table the friend's
/// group fits, and the obligation every seat that would pay the friend's
/// debt, which joins `H` beside the hypothesis rule's (Q-M1f.570). So each of the
/// five instances is licensed, every seat that can witness it a candidate,
/// and owes a group, the friend's five seats.
#[test]
fn a_member_in_more_than_two_groups_is_in_no_grid() {
    let (c, x) = read("examples/lattice/06_friends_tables_everyone_sits.ein");
    assert!(c.monotone);
    assert_eq!(c.n(), 25);
    assert_eq!(c.components.len(), 1);
    assert!(x.grids.grids.is_empty());
    assert!(matches!(
        x.grids.not[..],
        [(0, NotAGrid::MoreThanTwo { most: 13 })]
    ));
    let mut in_groups: Vec<usize> = c.member_of.iter().map(Vec::len).collect();
    in_groups.sort_unstable();
    in_groups.dedup();
    assert_eq!(in_groups, [9, 11, 13]);
    assert_eq!(x.owed.table().len(), 5);
    assert_eq!(x.owed_groups().len(), 5);
    assert_eq!(x.unlicensed(), []);
    for (i, owing) in x.owed.table().iter().zip(&x.owing) {
        let Owing::Group(g) = owing else {
            panic!("{} stands as {owing:?}", i.text)
        };
        let members = texts(&c, c.members(*g));
        let friend = parts(members.first().expect("a member"))[1].to_string();
        assert_eq!(
            members,
            ["B1", "B2", "B3", "S1", "S2"]
                .map(|s| format!("(sits {friend} {s})"))
                .into(),
            "{}",
            i.text
        );
        let witnesses = i.witnesses.as_ref().expect("its witnesses");
        assert!(
            witnesses
                .iter()
                .all(|w| matches!(w.is, WitnessIs::Candidate(_))),
            "{}",
            i.text
        );
    }
    // The groups nothing owes are named, each with its members.
    assert_eq!(x.unowed().len(), 50);
    assert!(
        x.unowed()
            .iter()
            .all(|&g| !texts(&c, c.members(g)).is_empty())
    );
}
