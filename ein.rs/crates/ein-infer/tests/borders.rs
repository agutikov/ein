//! **The border route describes `R` as the state route does** — the route
//! [`ein_infer::borders`] measures, which no question of `ein solve` takes:
//! `explore::describe_by(…, Route::Borders)` and the price list's `--borders`
//! alone call it. On a few programs, small enough for every commit, each
//! field of its description that is `R`'s or around it is the state route's,
//! with `N` taken and without, and the cube's where the oracle reaches. The
//! corpus-wide reading of the same comparison is the price list's.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::described::Described;
use ein_infer::explore::{self, Ask, Mode, Route, Which};
use ein_infer::region;
use ein_ir::{Ast, load, parse};

/// Monotone programs that search: ∅ in `R`, and root owing with one state of
/// `R` or several, merged states, an empty frontier and a dead candidate.
const PROGRAMS: [&str; 7] = [
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/lattice/07_two_pairs_two_tables.ein",
    "examples/lattice/06_friends_tables_everyone_sits.ein",
    "examples/lattice/14_seats_nothing_proposes.ein",
    "examples/beyond_arith/cryptarithm.ein",
    "examples/binary_logic/knights.ein",
    "tests/stdlib/bool/inputs/01_in_drives.ein",
];

fn loaded(path: &Path) -> (Kb, Terms, Ast) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    (kb, terms, ast)
}

fn describe(path: &Path, route: Route, nogoods: bool) -> Described {
    let (mut kb, mut terms, ast) = loaded(path);
    let opts = explore::Options {
        cores: region::Cores::READ_OUT,
        nogoods,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let a = explore::describe_by(&mut kb, &mut terms, &ast, &opts, route)
        .unwrap_or_else(|e| panic!("{path:?} by {route:?}: {e}"));
    assert_eq!(a.route, route, "{path:?}");
    *a.described.expect("a description")
}

/// The fields of `b` that are `R`'s or around it and differ from `a`'s. The
/// run's own (the route, the work, how a stopped run is partial, whether a
/// node met owes) are not compared, nor `N`'s members where neither took it.
fn differ(a: &Described, b: &Described) -> Vec<&'static str> {
    let mut d = Vec::new();
    let texts =
        |x: &Described| -> Vec<String> { x.candidates.iter().map(|c| c.text.clone()).collect() };
    if texts(a) != texts(b) {
        d.push("candidates");
    }
    if a.exhausted != b.exhausted {
        d.push("exhausted");
    }
    if a.root != b.root || a.any_alive != b.any_alive {
        d.push("root");
    }
    if a.states != b.states {
        d.push("states");
    }
    if a.k_c != b.k_c {
        d.push("k_c");
    }
    if a.minimal_commitments != b.minimal_commitments {
        d.push("minimal_commitments");
    }
    if a.nogoods_skipped != b.nogoods_skipped {
        d.push("nogoods_skipped");
    }
    if !(a.nogoods_skipped && b.nogoods_skipped)
        && (a.nogoods != b.nogoods
            || !a
                .dead
                .iter()
                .map(|x| &x.set)
                .eq(b.dead.iter().map(|x| &x.set)))
    {
        d.push("N");
    }
    if a.dead_alone != b.dead_alone || a.dead_alone_set != b.dead_alone_set {
        d.push("dead_alone");
    }
    if a.dead_met != b.dead_met {
        d.push("dead_met");
    }
    let shape = |x: &Described| {
        x.shape
            .as_ref()
            .map(|s| (s.components.clone(), s.subcubes.clone()))
    };
    if shape(a) != shape(b) {
        d.push("shape");
    }
    if a.frontier != b.frontier {
        d.push("frontier");
    }
    d
}

#[test]
fn the_border_route_describes_r_as_the_state_route_does() {
    let mut bad: Vec<String> = Vec::new();
    for rel in PROGRAMS {
        let path = repo_root().join(rel);
        for nogoods in [true, false] {
            let s = describe(&path, Route::States, nogoods);
            let b = describe(&path, Route::Borders, nogoods);
            assert!(s.exhausted && b.exhausted, "{rel}: both routes finish");
            let d = differ(&s, &b);
            if !d.is_empty() {
                bad.push(format!("{rel}, N taken {nogoods}: {d:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn the_border_route_is_the_oracle_where_it_reaches() {
    let mut read = 0;
    for rel in PROGRAMS {
        let path = repo_root().join(rel);
        let (mut kb, mut terms, ast) = loaded(&path);
        let o = region::Options {
            limit: region::DEFAULT_LIMIT,
            keep_states: false,
            jobs: 1,
            deadline: None,
            cores: region::Cores::READ_OUT,
        };
        let region::Outcome::Region(r) =
            region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {}).expect("the oracle")
        else {
            continue;
        };
        read += 1;
        let cube = ein_infer::described::of_cube(&r);
        let b = describe(&path, Route::Borders, true);
        let d = differ(&cube, &b);
        assert!(d.is_empty(), "{rel}: {d:?} differ from the oracle's");
    }
    assert!(read >= 3, "the oracle reaches {read} of the programs");
}
