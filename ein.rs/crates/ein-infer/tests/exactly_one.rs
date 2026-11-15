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
//! 2. **one relation, five grids, half of each owed**: `zebra.ein` states the
//!    same puzzle over one relation through `std.slots`. The same five
//!    grids; of each, the family *an attribute's houses* is owed, and the
//!    family *a house's attributes of one kind* is owed by nothing L5
//!    licenses: an instance is about each of those groups whose witnesses
//!    are the mirrored facts no candidate proposes;
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
    self, Cover, CoverOptions, ExactlyOne, Mark, NoTable, NotAGrid, Owing, UNPROPOSED, WitnessIs,
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

/// **One relation, five grids, half of each owed.** `zebra.ein` proposes
/// the same 56 placements over one relation: the same five grids as
/// `zebra2.ein`'s under the renaming, found though one relation holds them
/// all and no candidate's text says an attribute's kind.
///
/// Root owes 36 instances. The 18 of `slot-owed-room`, *an attribute is in
/// no house yet*, are licensed and each owes a group: in every grid one
/// family whole. The 18 of `slot-owed-fill`, *a house has no attribute of a
/// kind yet*, L5 does not license: a fact that can witness one is
/// `(co-located House-n v)`, and the candidates are `(co-located v
/// House-n)`. Each is about one of the 18 groups nothing owes, the other
/// family of every grid.
///
/// **What is true of those witnesses**: 56 of them are no candidate and
/// not denied, one the mirror of each candidate. By the theory alone each
/// is brought by its mirror's node and by no other. At root the 38 whose
/// mirror is alive alone are in that candidate's closure, and the 18 whose
/// mirror is dead alone are held by no single's node.
#[test]
fn one_relation_holds_five_grids_and_half_of_each_is_owed() {
    let (c, x) = read("examples/zebra.ein");
    let (c2, x2) = read("examples/zebra2.ein");
    assert_eq!(x.grids.grids.len(), 5);
    assert_eq!(x.grids.not, []);
    assert_eq!(placements(&c, &x), placements(&c2, &x2));
    assert!(
        c.candidates
            .iter()
            .all(|k| parts(&k.text)[0] == "co-located")
    );

    assert_eq!((x.owed.root_owes, x.owed.table().len()), (36, 36));
    assert_eq!(x.owed_groups().len(), 18);
    assert_eq!(x.unowed().len(), 18);
    assert_eq!(x.unlicensed().len(), 18);
    assert_eq!(x.not_groups(), []);
    for (i, owing) in x.owed.table().iter().zip(&x.owing) {
        match owing {
            Owing::Group(_) => assert_eq!(i.rule, "std.slots.slot-owed-room"),
            Owing::Unlicensed { about, .. } => {
                assert_eq!(i.rule, "std.slots.slot-owed-fill");
                assert_eq!(i.alternatives, Err(UNPROPOSED.to_string()));
                assert!(about.is_some(), "{}", i.text);
            }
            other => panic!("{} stands as {other:?}", i.text),
        }
    }
    // In every grid one family is owed whole, and every group of the other
    // has an unlicensed instance about it and is owed by nothing.
    for k in 0..5u32 {
        let mut share = x.share(k).to_vec();
        share.sort_by_key(|s| s.owed);
        assert_eq!((share[0].owed, share[0].about), (0, share[0].groups));
        assert_eq!((share[1].owed, share[1].about), (share[1].groups, 0));
        assert_eq!(share[0].groups, share[1].groups, "as many rows as columns");
        assert!(!x.is_bijection(k));
    }
    // The 18 groups nothing owes are the 18 an instance is about, one each.
    let about: Vec<u32> = (0..c.groups.len() as u32)
        .filter(|&g| !x.about[g as usize].is_empty())
        .collect();
    assert_eq!(about, x.unowed());
    assert!(x.about.iter().all(|a| a.len() <= 1));

    // The witnesses, by the nodes.
    let dead: BTreeSet<u32> = (0..c.n() as u32)
        .filter(|&b| c.marks[b as usize] == Mark::DeadAlone)
        .collect();
    assert_eq!(dead.len(), 18);
    let mut mirrored: BTreeMap<u32, usize> = BTreeMap::new();
    let (mut brought, mut by_nothing, mut whole) = (0, 0, 0);
    for &k in &x.unlicensed() {
        let i = &x.owed.table()[k as usize];
        for w in i.witnesses.as_ref().expect("its witnesses") {
            match w.is {
                WitnessIs::Candidate(_) => panic!("{} is a candidate", w.text),
                WitnessIs::Denied => {
                    assert!(w.held_at_root.is_empty() && w.held_in_theory.is_empty());
                }
                WitnessIs::Unproposed => {
                    // By the theory: its mirror, and no other candidate.
                    let [b] = w.held_in_theory[..] else {
                        panic!("{} is brought by {:?}", w.text, w.held_in_theory)
                    };
                    let (p, q) = (parts(&w.text), parts(c.text(b)));
                    assert_eq!((p[0], p[1], p[2]), (q[0], q[2], q[1]));
                    *mirrored.entry(b).or_default() += 1;
                    // At root: in its mirror's closure where that is alive
                    // alone, and in no single's where it is dead alone.
                    if dead.contains(&b) {
                        assert!(w.held_at_root.is_empty(), "{}", w.text);
                        by_nothing += 1;
                    } else {
                        assert!(w.held_at_root.contains(&b), "{}", w.text);
                        brought += 1;
                    }
                }
            }
        }
        let (all, by) = x.brought(k).expect("read");
        whole += usize::from(all == by);
    }
    assert_eq!(
        mirrored.len(),
        56,
        "one witness the mirror of each candidate"
    );
    assert!(mirrored.values().all(|&n| n == 1));
    assert_eq!((brought, by_nothing), (38, 18));
    assert_eq!(whole, 5, "instances every such witness of which is brought");
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
/// candidate is in nine groups or ten: one component, and no grid. The
/// program that owes every friend a seat has two owed groups, the two
/// friends every seat of whom its hrule proposes; of the three others a
/// seat can witness the debt that no candidate proposes, that root does not
/// deny, and that no single's node brings, at root or by the theory: such
/// an instance is about no group.
#[test]
fn a_member_in_more_than_two_groups_is_in_no_grid() {
    let (c, x) = read("examples/lattice/06_friends_tables_everyone_sits.ein");
    assert!(c.monotone);
    assert_eq!(c.components.len(), 1);
    assert!(x.grids.grids.is_empty());
    assert!(matches!(
        x.grids.not[..],
        [(0, NotAGrid::MoreThanTwo { most: 10 })]
    ));
    assert_eq!(x.owed.table().len(), 5);
    assert_eq!(x.owed_groups().len(), 2);
    assert_eq!(x.unlicensed().len(), 3);
    for &k in &x.unlicensed() {
        assert!(matches!(
            x.owing[k as usize],
            Owing::Unlicensed { about: None, .. }
        ));
        let i = &x.owed.table()[k as usize];
        let unproposed: Vec<&structure::Witness> = i
            .witnesses
            .as_ref()
            .expect("its witnesses")
            .iter()
            .filter(|w| w.is == WitnessIs::Unproposed)
            .collect();
        assert_eq!(unproposed.len(), 2, "{}", i.text);
        assert!(
            unproposed
                .iter()
                .all(|w| w.held_at_root.is_empty() && w.held_in_theory.is_empty())
        );
        assert_eq!(x.brought(k), Some((2, 0)));
    }
    // The groups nothing owes are named, each with its members.
    assert_eq!(x.unowed().len(), 24);
    assert!(
        x.unowed()
            .iter()
            .all(|&g| !texts(&c, c.members(g)).is_empty())
    );
}
