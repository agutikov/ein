//! The container's size and its cold open, timed — [03-design/09-binary-format.md § 7](../../../../docs/ein/ein.rs/03-design/09-binary-format.md#7-tests).
//!
//! **Alone in its test binary, on purpose.** `cargo test` runs the tests of one
//! binary in parallel threads and the binaries one after another, so a timed
//! open beside other tests shares the CPUs with them and measures them. Beside
//! the corpus round trips of `roundtrip.rs` it read several times its own
//! cost on a four-CPU machine, and past its ceiling on the CI runner, with the
//! code unchanged; alone it reads the open (`taskset -c 0-3 cargo test
//! --profile gate -p ein-einb --test open_budget -- --nocapture`, against the
//! same with `--test roundtrip` when the test lived there). Nothing else goes
//! in this file.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_einb::{KbState, OpenOptions, SaveOptions, save_to_vec};
use ein_ir::{Ast, parse};

/// Load a program the way the CLI does.
fn load(path: &Path) -> Option<(Ast, Terms, Kb, Vec<ein_ir::NodeId>)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((ast, terms, kb, forms))
}

/// Two ceilings: a saturated `zebra2` under 80 KB, and a cold open whose
/// median is under 6 ms in a build without debug assertions and under 20 ms
/// in one with them.
///
/// The open is timed as a *median* of eleven, because a single one on a shared
/// runner measures the runner, and alone in its test binary (the module's
/// head says why). It is still the whole job — digest check,
/// re-intern, re-parse of `PROGRAM`, registry rebuild, fact replay, index
/// rebuild — and not a subset chosen to make the number. The re-parse by the
/// generated parser is most of it; keeping the parsed tree in the container
/// would be a format bump.
#[test]
fn a_saturated_zebra2_is_under_80_kb_and_opens_cold_under_6_ms_or_20_with_assertions() {
    let path = repo_root().join("examples/zebra2.ein");
    let (mut ast, mut terms, mut kb, forms) = load(&path).expect("zebra2 loads");
    ein_infer::saturate_events(&ast, &mut terms, &mut kb).expect("saturate");
    let bytes = save_to_vec(
        &kb,
        &terms,
        &mut ast,
        &forms,
        path.parent(),
        &SaveOptions {
            state: KbState::Saturated,
            ..SaveOptions::default()
        },
    )
    .expect("save");
    // Every import brings its module whole, so the container stores
    // `std.algebra`, `std.bijection` and `std.macro` whole, with their rules
    // and `std.algebra`'s lemmas. `ein kb save -s examples/zebra2.ein` writes
    // the file, and the line below prints each section's bytes.
    assert!(
        bytes.len() < 80 * 1024,
        "a saturated zebra2 is {} bytes, over 80 KB",
        bytes.len()
    );

    let breakdown: Vec<String> = ein_einb::section_sizes(&bytes)
        .expect("a readable section table")
        .iter()
        .map(|(k, n)| format!("{k:?}={n}"))
        .collect();

    let mut times: Vec<f64> = Vec::new();
    for _ in 0..11 {
        let mut fresh = Terms::new();
        let t = std::time::Instant::now();
        let opened =
            ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default()).expect("open");
        times.push(t.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(opened.kb.n_facts());
    }
    times.sort_by(f64::total_cmp);
    let median = times[times.len() / 2];
    // The tight ceiling is about the **shipped** engine. A build with debug
    // assertions on (`cargo test`'s dev profile, `opt-level = 1`, or the
    // `gate` profile) runs on a shared machine, so gating the tight number on
    // it would be measuring the runner. The number is printed either way,
    // which is what a regression is read off (`cargo test [--release] -p
    // ein-einb --test open_budget -- --nocapture`); what
    // differs is the ceiling that fails the test.
    let budget = if cfg!(debug_assertions) { 20.0 } else { 6.0 };
    eprintln!(
        "einb: saturated zebra2 = {} bytes ({}), {} facts, cold open {median:.3} ms",
        bytes.len(),
        breakdown.join(" "),
        kb.n_facts()
    );
    // The message names the budget, the profile and the suspect: a wall
    // clock this tight can fail for the machine's sake, and a failure that
    // said only what it took would not say which.
    assert!(
        median < budget,
        "cold open took {median:.3} ms against a {budget} ms {} budget \
         — a real regression, or machine load?",
        if cfg!(debug_assertions) {
            "dev"
        } else {
            "release"
        }
    );
}
