//! T1a.9.4.3 — **the Rust embedding contract, executable.**
//!
//! [`docs/api/rust.md`](../../../../docs/api/rust.md) documents how to drive
//! the engine from another Rust program. Its worked example is not a code
//! block someone typed into a markdown file: it is the region of *this* file
//! between the two `// ─── page ───` markers, and
//! [`the_page_quotes_this_file`] compares the two texts.
//!
//! That is the whole mechanism, and it is deliberately the cheapest one that
//! cannot rot. `docs/api/`'s five Python pages were a *good* contract — five
//! steps, a worked example with real numbers, per-symbol tables — and they
//! still went stale, because nothing ran them. The substitute for the
//! contract suite the deferred [S1a.9.2] would have been is not a bigger
//! suite; it is a page whose example is compiled and run by
//! `cargo test --workspace` and whose text is diffed against what ran.
//!
//! [S1a.9.2]: ../../../../docs/history/m1a_rust/README.md#p1a9--release
//!
//! Three tests, three different failures:
//!
//! - the example does not compile → this file does not build;
//! - the example's numbers move → [`the_worked_example_runs`];
//! - the page and the example drift apart → [`the_page_quotes_this_file`].
//!
//! **`cargo fmt` is one of the things that can make them drift**, since the
//! marked region is ordinary code and the formatter has an opinion about it.
//! That is intended, not a flaw: the failure is loud and the fix is one
//! paste. The rule, and it is in `AGENTS.md` too — **edit the test, run it,
//! paste. Never edit the page's code block by hand.**

use ein_corpus::repo_root;

// ─── page ───────────────────────────────────────────────────────────
use std::path::Path;

use ein_core::Terms;
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict, goal_bindings};
use ein_ir::Ast;
use ein_render::{LinearizeOpts, Mode, linearize, render_markdown};

/// Solve one `.ein` file and report the model, as a downstream crate would.
fn run(path: &Path) -> Result<String, String> {
    // 1 — the two arenas. `Ast` owns the parsed forms, `Terms` the interned
    //     symbols and values; both outlive the KB and are passed by `&mut`
    //     everywhere, because loading and solving both intern.
    let mut ast = Ast::new();
    let mut terms = Terms::new();

    // 2 — load: parse, expand macros, resolve `(import std.*)` against the
    //     file's directory, and build the KB. `ein_ir::parse` + `ein_ir::load`
    //     is the same thing in two steps, for a caller that has the text.
    let mut kb = ein_ir::load_file(&mut ast, &mut terms, path).map_err(|e| e.to_string())?;
    let loaded_facts = kb.n_facts();

    // 3 — solve. One entry, and the verdict is *read* from the result rather
    //     than chosen: `stop_after: Some(1)` is the fast path, `None`
    //     exhausts the lattice and certifies unique / ambiguous / unsat.
    //     `store_lattice` is what step 5 needs and nothing else.
    let opts = SolveOptions {
        stop_after: Some(1),
        store_lattice: true,
        ..SolveOptions::default()
    };
    let solved: Solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut Events::off(),
        &mut NoDumper,
        &opts,
    )
    .map_err(|e| e.to_string())?;

    // 4 — read. `Answer::Aborted` is deliberately outside `Verdict`, so a
    //     caller that never sets a budget still has to name it once.
    let mut out = format!("loaded {loaded_facts} facts\n");
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(model)) => {
            // The model is the solution's **own** KB — root plus what the
            // winning commitment derived. Projecting the goal over `kb` would
            // ask the question of the unsolved root and answer nothing.
            out += &format!("model: {} facts\n", model.kb.n_facts());
            for row in goal_bindings(&ast, &mut terms, &model.kb, None) {
                for (var, val) in row {
                    out += &format!("  {} = {}\n", terms.sym(var), terms.display(val));
                }
            }
        }
        Answer::Verdict(Verdict::Ambiguity(models)) => {
            out += &format!("{} distinct models\n", models.len());
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            out += &format!("unsat, core of {} facts\n", unsat_core.len());
        }
        Answer::Aborted { reason } => out += &format!("aborted: {reason}\n"),
    }
    out += &format!(
        "k = {}, exhausted = {}\n",
        solved.stats.solution_nodes, solved.stats.exhausted
    );

    // 5 — explain. The markdown derivation `--trace` writes, from the lattice
    //     proof step 3 asked for.
    let trace = linearize(&ast, &terms, &kb, &solved, LinearizeOpts::new());
    let md = render_markdown(&trace, Mode::Engine, false);
    out += &format!("trace: {} steps\n", trace.steps.len());
    let _ = md;
    Ok(out)
}
// ─── page ───────────────────────────────────────────────────────────

/// The example runs, and its numbers are the page's numbers.
///
/// Not a golden: five lines asserted by hand is cheaper to read than a banked
/// file, and every one of them is quoted in the page's output block.
#[test]
fn the_worked_example_runs() {
    let out = run(&repo_root().join("examples/zebra2.ein")).expect("solve zebra2");
    let want = "\
loaded 84 facts
model: 444 facts
  h_water = House-1
  who_water = Norwegian
  h_zebra = House-5
  who_zebra = Japanese
k = 1, exhausted = false
trace: 244 steps
";
    assert_eq!(out, want);
}

/// A contradiction and an ambiguity take the other two arms, so the `match`
/// in the page is not three arms of which one has ever executed.
#[test]
fn the_other_two_verdicts_are_reachable() {
    let bad = run(&repo_root().join("examples/ein-bugs/zebra2-bad.ein")).expect("solve");
    assert!(bad.contains("unsat, core of "), "{bad}");
    let hints = run(&repo_root().join("examples/zebra2-hints.ein")).expect("solve");
    assert!(hints.contains("h_zebra = House-5"), "{hints}");
}

/// **The page quotes this file.** Both texts are the region between the two
/// `// ─── page ───` markers; a change to either that is not made to the
/// other fails here rather than shipping a documented API that does not
/// compile.
#[test]
fn the_page_quotes_this_file() {
    // The whole line, because the module doc above mentions the short form
    // and a substring split would cut there first.
    const MARK: &str =
        "\n// ─── page ───────────────────────────────────────────────────────────\n";
    let src = std::fs::read_to_string(repo_root().join("ein.rs/crates/ein-cli/tests/embedding.rs"))
        .expect("this file");
    let mut parts = src.split(MARK);
    parts.next().expect("preamble");
    let example = parts.next().expect("an opening marker");
    assert!(parts.next().is_some(), "the closing marker is missing");
    assert!(
        example.contains("fn run("),
        "the marked region is not the example"
    );

    let page = std::fs::read_to_string(repo_root().join("docs/api/rust.md")).expect("the page");
    let block = page
        .split("```rust\n")
        .nth(1)
        .expect("a ```rust block in docs/api/rust.md")
        .split("\n```")
        .next()
        .expect("its close");

    assert_eq!(
        block, example,
        "docs/api/rust.md's first ```rust block and this file's marked region \
         have diverged. They are one text: edit the test, run it, paste."
    );
}
