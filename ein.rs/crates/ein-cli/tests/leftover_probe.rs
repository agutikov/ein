//! M1d S1d.3.1 — **the leftover-open probe, and the claim that it is a read.**
//!
//! One test in a file of its own, for
//! [`obligation_rung_control.rs`](../../ein-infer/tests/obligation_rung_control.rs)'s
//! reason and by its rule: `EIN_LEFTOVER` is read from the process
//! environment, so nothing that asserts the default may share a binary — or a
//! `--test-threads` — with something that sets it. Cargo gives each
//! `tests/*.rs` its own process; one `#[test]` inside it is the rest of the
//! serialisation, and it is why the three claims below are three sections
//! rather than three functions.
//!
//! What the probe answers is the question a state's model cannot be asked:
//! how many facts the **blind** enumerator would still propose there, outside
//! H. Since M1e2 S1e2.1a.5 it is read per state the read-out lists
//! (`read_out.md` § 12) — a question about the universe, not about a route.
//! Running it on a fork of the state's fixpoint is what makes it a read, and
//! *that* is the claim worth a test, because it is the one that would fail
//! silently. Since M1e2 S1e2.3.1 (the user's ruling of 2026-11-03) the pass is
//! **config-free**, as H's is: no lookahead filter drops a candidate before it
//! is counted, and § 3 holds that too.

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load_file};
use serde_json::Value as J;

/// One entry's summary, under `--maximal`, as `--json-summary` writes it.
fn summary(rel: &str) -> J {
    let path = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let config = kb.program().config.clone().unwrap_or_default();
    let opts = explore::Options::new(Mode {
        which: Which::Maximal,
        ask: Ask::Describe,
    });
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    let json = ein_cli::summary::build_region(&ast, &mut terms, &mut kb, &r, rel, &config)
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    serde_json::from_str(&ein_render::dump::json::dumps_indent(&json)).expect("summary is JSON")
}

fn listed(summary: &J) -> u64 {
    summary["region"]["states"]
        .as_array()
        .expect("states")
        .iter()
        .filter(|s| s["listed"] == true)
        .count() as u64
}

fn counts(summary: &J, key: &str) -> Vec<u64> {
    summary["leftover"][key]
        .as_array()
        .expect("leftover array")
        .iter()
        .map(|n| n.as_u64().expect("a count"))
        .collect()
}

#[test]
fn the_leftover_probe_is_off_by_default_is_a_read_and_means_what_it_says() {
    // ── 1. Off unless it is on, and emitted either way ──────────────
    //
    // A field that appears only sometimes is a field a consumer has to guess
    // about — the rule the `owes` and `config` blocks already follow — so
    // `taken` carries the state and the two arrays are empty rather than
    // absent. Read before any write to the environment.
    let closed_default = summary("examples/branching/12_typed_blind_solve.ein");
    assert_eq!(closed_default["leftover"]["taken"], J::Bool(false));
    assert!(counts(&closed_default, "states").is_empty());

    let before = summary("examples/branching/06_lookahead_on.ein");

    // SAFETY: this binary holds exactly one test, so nothing else in the
    // process reads the environment while it is written. `EIN_LEFTOVER` is
    // read once per summary, so the write has to land before `build_region`
    // and not during it.
    unsafe { std::env::set_var("EIN_LEFTOVER", "1") };
    let after = summary("examples/branching/06_lookahead_on.ein");
    let closed = summary("examples/saturation/square-fwd/houses.ein");
    let typed = summary("examples/branching/12_typed_blind_solve.ein");
    let open = summary("examples/branching/05_mini_zebra.ein");
    unsafe { std::env::remove_var("EIN_LEFTOVER") };

    // ── 2. The probe changes nothing but its own block ──────────────
    //
    // The property that makes it a measurement rather than a different
    // engine, taken on a multi-model entry so that the model *set* is in
    // scope: the pass writes into its fork, and if the fork were the state the
    // writes would move what the read-out lists. Everything outside
    // `leftover` — R, every state, the work, the root block — is compared
    // whole, so a probe that leaks is caught wherever it leaks.
    assert_eq!(after["leftover"]["taken"], J::Bool(true), "the lever took");
    let mut a = before.clone();
    let mut b = after.clone();
    a.as_object_mut().expect("object").remove("leftover");
    b.as_object_mut().expect("object").remove("leftover");
    assert_eq!(a, b, "the probe moved something outside its own block");
    assert_eq!(
        counts(&after, "states").len() as u64,
        listed(&after),
        "one row per listed state, index-aligned with region.states"
    );

    // ── 3. What the count means ─────────────────────────────────────
    //
    // The two shapes a corpus entry comes in, and the contrast is the whole
    // point of the number:
    //
    // * `saturation/square-fwd/houses`' R is its whole cube, so its maximal
    //   line is the one state that commits every candidate, and every
    //   candidate the blind enumerator can build there is already a fact.
    //   **Zero** — the state is a complete assignment, and the open- and
    //   closed-world readings agree.
    // * `branching/05_mini_zebra` does not close `co-located`, and its
    //   **unique** maximal state still leaves dozens of atoms undecided: every
    //   pair the blind enumerator builds and no rule decides. Under an
    //   open-world reading that state is 2ⁿ models; the puzzle means the
    //   closed-world one, and nothing in the file says so. (`zebra2`'s one
    //   state leaves 8 203 config-free, at seconds per run; M1d's 3 678 was
    //   the count through the walk's lookahead, until 2026-11-03.)
    //
    // And the pass is config-free (M1e2 S1e2.3.1). `branching/12_typed_blind_solve`
    // decides every well-typed `color-of` atom at each state of its maximal
    // line, and leaves the type-wrong fillers — `(color-of Red Blue)` — that
    // its `typecheck-arg-*` rules refute only once one is added. The walk's
    // one-step lookahead dropped exactly those, and the count read 0; H keeps
    // them, so the count outside H does too.
    //
    // No half is a golden — the bounds are `= 0` and `> 0`, not the counts
    // this engine reports today.
    let rows = counts(&closed, "states");
    assert_eq!(rows.len() as u64, listed(&closed));
    assert!(
        !rows.is_empty() && rows.iter().all(|&n| n == 0),
        "a state that commits every candidate leaves nothing open: {rows:?}"
    );
    let rows = counts(&typed, "states");
    assert_eq!(rows.len() as u64, listed(&typed));
    assert!(
        !rows.is_empty() && rows.iter().all(|&n| n > 0),
        "the probe dropped what a refutation would only find on adding it — a \
         filter H's pass does not apply: {rows:?}"
    );
    let rows = counts(&open, "states");
    assert_eq!(rows.len(), 1, "mini_zebra's maximal line is one state");
    assert!(
        rows[0] > 0,
        "an open domain leaves the enumerator something: {rows:?}"
    );
}
