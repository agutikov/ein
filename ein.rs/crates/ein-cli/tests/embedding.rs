//! T1a.9.4.3 — **the Rust embedding contract, executable.**
//!
//! [`docs/api/rust.md`](../../../../docs/api/rust.md) documents how to drive
//! the engine from another Rust program. Its worked example is not a code
//! block someone typed into a markdown file: it is the region of *this* file
//! between the two `// ─── page ───` markers, and
//! [`the_page_quotes_this_file`] compares the two texts. Since M1e a second
//! pair of markers, `// ─── prose ───`, does the same for the *paragraph* that
//! names these tests; on the page it is delimited by `<!-- prose -->`.
//!
//! That is the whole mechanism, and it is deliberately the cheapest one that
//! cannot rot. The substitute for the contract suite the deferred [S1a.9.2]
//! would have been is not a bigger suite; it is a page whose example is
//! compiled and run by `cargo test --workspace` and whose text is diffed
//! against what ran.
//!
//! [S1a.9.2]: ../../../../docs/history/m1a_rust/README.md#p1a9--release
//!
//! Five tests, five different failures:
//!
//! - the example does not compile → this file does not build;
//! - the example's numbers move → [`the_worked_example_runs`];
//! - a verdict arm the page shows is never taken →
//!   [`the_other_three_verdicts_are_reachable`];
//! - the page and the example drift apart → [`the_page_quotes_this_file`];
//! - the page's **prose about this file** drifts →
//!   [`the_page_quotes_this_files_prose_too`] and
//!   [`the_page_and_the_file_name_the_same_tests`].
//!
//! The last two are M1e `CD-M4`, and the finding was on this mechanism's own
//! page: the sentence naming the tests said
//! `the_other_two_verdicts_are_reachable` — a name from before `Open` arrived
//! — and sat **outside** the marked region, which is the one class of drift a
//! quote-this-file check structurally cannot catch. Widening the region is
//! only half a fix, and it is worth being explicit about which half:
//! a second marker makes the page and this file **one text**, so editing one
//! and not the other is loud; it does **not** make that text *true*, because
//! renaming a test and leaving the comment alone keeps the two in agreement
//! about a name neither of them still has. That is what
//! [`the_page_and_the_file_name_the_same_tests`] is for, and it is the one
//! that would have failed on the day of the rename.
//!
//! **`cargo fmt` is one of the things that can make them drift**, since the
//! marked region is ordinary code and the formatter has an opinion about it.
//! That is intended, not a flaw: the failure is loud and the fix is one
//! paste. The rule, and it is in `AGENTS.md` too — **edit the test, run it,
//! paste. Never edit the page's code block by hand.**

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_corpus::repo_root;

/// This test file, which three of the tests below read as text.
fn this_file() -> PathBuf {
    repo_root().join("ein.rs/crates/ein-cli/tests/embedding.rs")
}

/// The page these markers are quoted into.
fn page() -> String {
    std::fs::read_to_string(repo_root().join("docs/api/rust.md")).expect("the page")
}

// ─── page ───────────────────────────────────────────────────────────
use std::path::Path;

use ein_core::Terms;
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Verdict, Which};
use ein_ir::Ast;

/// Ask whether one `.ein` file's answer is one state, and report it, as a
/// downstream crate would.
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

    // 3 — root's derivation, kept for step 5's trace. The run below finds
    //     root saturated and leaves it as it is; a caller that wants no trace
    //     skips this.
    let firings = ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut Events::off())
        .map_err(|e| e.to_string())?;

    // 4 — ask. A question is a `Mode`: which part of R (all of it, the
    //     minimal line, the maximal line) and what is asked of it (describe
    //     it, `Find(n)` states, or `Single`: is it exactly one state?). The
    //     read-out answers it and reads every state it lists — each state's
    //     `(model …)` and each query's projection classes over them.
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Single,
    });
    let r =
        ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())?;

    // 5 — read. `verdict` is `None` only when a budget stopped the run before
    //     it decided, so a caller that never sets one still has to name it.
    let mut out = format!("loaded {loaded_facts} facts\n");
    match r.answer.verdict {
        Some(Verdict::Solution) => {
            // R's one state: the positive facts of the candidates' relations,
            // and the query's rows in it — `(query …)` asked of the state, not
            // of root, which would answer nothing.
            out += &format!("one state: {} facts over H\n", r.states[0].model.len());
            for class in r.queries.iter().flat_map(|q| &q.classes) {
                for row in &class.rows {
                    for (var, val) in row {
                        out += &format!("  {var} = {val}\n");
                    }
                }
            }
        }
        Some(Verdict::Ambiguity) => out += "R holds two states or more\n",
        Some(Verdict::Contradiction) => out += "R is empty: every node is dead\n",
        // Alive nodes, every one of which still owes an obligation the program
        // stated (M1d S1d.2.6). Not a model and not a refutation — a caller
        // that branches on "R is empty" alone would call it unsat.
        Some(Verdict::Open) => out += "R is empty: every alive node owes\n",
        None => out += &format!("undecided: {:?}\n", r.answer.stopped),
    }
    // `exhausted`: the states listed are all of the part of R asked about.
    // Under `Single` that is every word but `Ambiguity`, which lists two.
    out += &format!(
        "saturations: {}, R listed whole: {}\n",
        r.answer.saturations, r.answer.exhausted
    );

    // 6 — explain. The markdown `ein solve FILE --single --trace FILE.md`
    //     writes: root's derivation, then each state's, from step 3's firings.
    let md = ein_render::trace::region::of_read_out(
        &r,
        &mut kb,
        &mut terms,
        &ast,
        &firings,
        &ein_render::read_out::Options {
            command: "solve",
            argv: "",
            file: &path.to_string_lossy(),
            budget: opts.budget,
            uncut: false,
        },
        ein_render::trace::region::Shape {
            diagrams: false,
            relevant: false,
            reorder: false,
            full_kb: false,
        },
    )?;
    let steps = md.lines().filter(|l| l.starts_with("### Step ")).count();
    out += &format!("trace: {steps} steps\n");
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
one state: 25 facts over H
  h_water = House-1
  h_zebra = House-5
  who_water = Norwegian
  who_zebra = Japanese
saturations: 45, R listed whole: true
trace: 569 steps
";
    assert_eq!(out, want);
}

// ─── prose ──────────────────────────────────────────────────────────
// The counts above are asserted by `the_worked_example_runs`, which takes the
// `Solution` arm; `the_other_three_verdicts_are_reachable` takes
// `Contradiction`, `Ambiguity` and `Open`, on three other files. So the `match`
// is five arms of which **four** have run. The fifth, `None`, needs a budget no
// example here sets, and it is in the page because a caller that never sets one
// still has to name it.
// ─── prose ──────────────────────────────────────────────────────────

/// A contradiction, an ambiguity and an **open** state take the other three
/// arms, so the `match` in the page is not four arms of which one has ever
/// executed.
///
/// The fourth arrived with M1d S1d.2.6 and is the one an embedder is most
/// likely to get wrong: R is empty there as it is for a `Contradiction`, so a
/// caller that branches on "no state" alone silently files it under *unsat*.
#[test]
fn the_other_three_verdicts_are_reachable() {
    let bad = run(&repo_root().join("examples/ein-bugs/zebra2-bad.ein")).expect("solve");
    assert!(bad.contains("R is empty: every node is dead"), "{bad}");
    let hints = run(&repo_root().join("examples/zebra2-hints.ein")).expect("solve");
    assert!(hints.contains("R holds two states or more"), "{hints}");
    assert!(hints.contains("R listed whole: false"), "{hints}");
    let owing = run(&repo_root().join("tests/stdlib/algebra/23_total_owed.ein")).expect("solve");
    assert!(
        owing.contains("R is empty: every alive node owes"),
        "{owing}"
    );
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
    let src = std::fs::read_to_string(this_file()).expect("this file");
    let mut parts = src.split(MARK);
    parts.next().expect("preamble");
    let example = parts.next().expect("an opening marker");
    assert!(parts.next().is_some(), "the closing marker is missing");
    assert!(
        example.contains("fn run("),
        "the marked region is not the example"
    );

    let page = page();
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
/// **The page quotes this file's prose too.** The paragraph that names these
/// tests is the region between the two `// ─── prose ───` markers with its
/// `// ` prefixes stripped, and the page carries it between `<!-- prose -->`
/// comments.
///
/// A separate test rather than a wider `page` region: that one exists to quote
/// a **code block**, and stretching it over prose would put a paragraph diff
/// and a compile-checked example behind the same failure message.
#[test]
fn the_page_quotes_this_files_prose_too() {
    const MARK: &str =
        "\n// ─── prose ──────────────────────────────────────────────────────────\n";
    let src = std::fs::read_to_string(this_file()).expect("this file");
    let mut parts = src.split(MARK);
    parts.next().expect("preamble");
    let region = parts.next().expect("an opening prose marker");
    assert!(
        parts.next().is_some(),
        "the closing prose marker is missing"
    );

    let prose: String = region
        .lines()
        .map(|l| l.strip_prefix("// ").unwrap_or(l.trim_end_matches("//")))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        prose.contains("the_worked_example_runs"),
        "the marked region is not the paragraph"
    );

    let page = page();
    let quoted = page
        .split("<!-- prose -->")
        .nth(1)
        .expect("a <!-- prose --> region in docs/api/rust.md");
    assert_eq!(
        quoted.trim_matches('\n'),
        prose.trim_matches('\n'),
        "docs/api/rust.md's <!-- prose --> region and this file's `// ─── prose ───` \
         region have diverged. They are one text: edit the comment, paste."
    );
}

/// **Every test name the page prints exists, and every test here is named.**
///
/// This is the check that would have caught `CD-M4` on the day it was made.
/// `the_other_two_verdicts_are_reachable` became
/// `the_other_three_verdicts_are_reachable` when the `Open` arm arrived, and
/// nothing anywhere resolved the old name against anything — not rustdoc (a
/// backtick is not an intra-doc link), not the quote-this-file diff (the
/// sentence was outside the region), and not the compiler (it is a string in a
/// markdown file).
///
/// Both directions, because a marker only makes the two texts *agree*:
///
/// - a name the page prints that is not a `fn` here — the page cites a test
///   that does not exist;
/// - a `#[test] fn` here that the page never names — a test was renamed and
///   the page still names it by the old name, which is the same failure read
///   from the other end, and it catches a rename whatever the new name looks
///   like.
///
/// The second direction is why the file's naming convention is not load-bearing:
/// this file's tests all begin `the_`, and the first direction reads the page
/// for that shape, but nothing would notice a test renamed *out* of the
/// convention if the closure were not checked from both sides.
#[test]
fn the_page_and_the_file_name_the_same_tests() {
    let src = std::fs::read_to_string(this_file()).expect("this file");
    let declared: BTreeSet<String> = src
        .lines()
        .filter_map(|l| l.trim().strip_prefix("fn "))
        .filter_map(|l| l.split_once('('))
        .map(|(name, _)| name.to_string())
        .filter(|n| n.starts_with("the_"))
        .collect();
    assert_eq!(declared.len(), 5, "{declared:?}");

    let page = page();
    let cited: BTreeSet<String> = page
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|t| t.starts_with("the_") && t.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
        .map(str::to_string)
        .collect();

    let phantom: Vec<_> = cited.difference(&declared).collect();
    assert!(
        phantom.is_empty(),
        "docs/api/rust.md names {phantom:?}, which is not a test in \
         ein-cli/tests/embedding.rs"
    );
    let unnamed: Vec<_> = declared.difference(&cited).collect();
    assert!(
        unnamed.is_empty(),
        "{unnamed:?} is a test here that docs/api/rust.md never names — if it \
         was renamed, the page still names the old one"
    );
}
