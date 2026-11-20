//! Graphviz accepts every file the engine emits.
//!
//! A digest sweep (`corpus_shapes.md5`) proves every view still renders the
//! bytes it rendered. It cannot prove those bytes are *valid*: a golden of a
//! broken file reproduces just as well as a golden of a good one. So every
//! view of every corpus entry also goes through `dot -Tsvg`, which is the only
//! authority on whether the grammar and the attribute values are real.
//!
//! **Every corpus program outside `tests/`; under `EIN_CORPUS_SLOW=1`,
//! every one**: the programs under `tests/` are each one stdlib
//! rule's claim, drawn by the writers every other program is drawn by.
//! **A trace's drawings**, its slices, for the two programs
//! [`TRACE_WITNESSES`] names, at every tier: no view is a trace, and the
//! corpus's traces are too many slices to lay out per commit.
//!
//! **No layout is a claim here**: where Graphviz puts a node is a reader's
//! preference, and only its refusal is the DOT's fault (Q21).
//!
//! ## When Graphviz is the one that is broken
//!
//! Ubuntu 24.04 ships `graphviz 2.42.2-9ubuntu0.1`, which reports itself as
//! 2.43.0 and dates from 2019, where a working checkout is more likely to
//! have 12.x or 15.x. On that build five of `stdlib/slots.ein`'s
//! views kill `dot` outright, SIGABRT out of `malloc(): unaligned fastbin chunk
//! detected` or SIGSEGV, and the trigger is cross-graph: every digraph in those
//! views is accepted **on its own**, and two of them in one process are not.
//! Reduced, it needs a nested cluster holding a `constraint=false` edge, and
//! any further graph after it in the same stream — legal DOT that 15.x renders
//! without a word, and heap corruption in the layout's cleanup between graphs.
//!
//! So a *signal* is treated as a statement about Graphviz and a non-zero
//! *exit* as a statement about the bytes. When `dot` dies on a view, the view
//! is put again one digraph at a time: a DOT file is a sequence of graphs, so
//! if each digraph is accepted alone then the concatenation is valid by the
//! grammar and the crash was never about the file. If a lone digraph still
//! kills it, there is nothing left to split and the view fails — which is also
//! what a genuine rejection does, now naming the digraph rather than the view.
//!
//! **It fails rather than skips when Graphviz is missing.** A skip would be a
//! stderr line `cargo test` captures for a passing test, and a check that
//! reports a pass because its tool is absent is not a check.
//!
//! **Nor does it skip a view that errors**: a view whose renderer returned an
//! error and was passed over in silence would leave this test green over a
//! view it does not check, the engine the view runs deleted or not. A file
//! that does not parse has no view, one that does not
//! load has no KB view, and one whose root does not saturate has no
//! `kb-since`, the one view that saturates: those are the corpus's load- and
//! compile-error fixtures, and each is skipped by its reason. Every other
//! error fails the sweep.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root, under_tests};

/// `EIN_CORPUS_SLOW=1`: every corpus program; unset, every program outside
/// `tests/`.
fn slow_too() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
}
use ein_ir::Ast;
use ein_render::shape::{KB_VIEWS, all_views, dot_shape, kb_dot_shape};
use std::io::Write;
use std::process::{Command, Stdio};

fn require_graphviz() {
    let ok = Command::new("dot")
        .arg("-V")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    assert!(
        ok,
        "`dot -V` does not run: this test needs Graphviz (`apt-get install \
         graphviz`, `pacman -S graphviz`). It is the only authority the DOT \
         views have on being well-formed, so it is a missing gate rather than \
         a missing convenience."
    );
}

/// What Graphviz made of one DOT text.
enum Verdict {
    Accepted,
    /// It read the input and complained — a statement about the bytes.
    Rejected(String),
    /// It died of a signal — a statement about Graphviz.
    Crashed(String),
}

/// A view with more edges than this is **not put to Graphviz at all**: it is
/// counted and skipped. What the test checks is the engine's DOT, not
/// Graphviz's layout of a large graph.
///
/// `dot`'s layered layout is superlinear in the edges, and a library of facts
/// is where that shows. `stdlib/math/digits.ein` is the
/// calculator's 1 664 table rows over eleven shared names, and every view of
/// it has some 7 900 edges, which neither `dot` nor `fdp`, the layout a KB
/// view suggests, lays out in minutes. Outside the tables, the wide rule
/// libraries and the saturated knowledge bases [`SKIPPING`] names, no corpus
/// view is past the bound.
///
/// **The bound is under the smallest table**: the KB view of a program that
/// imports `std.math.decimal-arith` holds `fa`'s 200 rows, some 1 200 edges.
/// So a view that holds a table is past the bound, and what `dot` lays out
/// is every view that holds none.
///
/// **What is given up:** nothing checks that the DOT text of a skipped view
/// is well-formed. The writers are the same functions that write the small
/// views, which is why the skip is safe. The sweep prints how many views it
/// skipped and each by name, with its edges, and holds the
/// directories they are in to [`SKIPPING`], so a skip is visible and one in
/// a directory that had none fails.
const DOT_LAYOUT_BOUND: usize = 1000;

/// The directories that hold a corpus file with a view past
/// [`DOT_LAYOUT_BOUND`]: the calculator's modules (`stdlib/math/`), the
/// programs that import them and so hold the digit tables,
/// and three rule libraries wide enough on their own: `stdlib/math/num9.ein`'s
/// wide rules, `examples/binary_arithmetic/five_operations.ein`, and
/// `std.bool.props` (`stdlib/bool/props.ein`, its
/// `kb-since` view, which holds what its rules derive) with the programs that
/// import it, under `tests/stdlib/bool/props/` and `examples/binary_logic/`,
/// where two more programs pass the bound by `std.bool.gates`' table and its
/// second layout; `tests/stdlib/math/decimal/`, whose programs hold the
/// tables; `tests/stdlib/math/convert/` and `tests/stdlib/math/binary-arith/`,
/// whose conversions and program of both bases hold decimal's; and
/// `tests/stdlib/math/gcd/`; and `tests/saved/`, whose standard library's
/// twin holds them. Every fact drawn as its octagon with an arrow to its
/// relation and to each argument (Q-M1f.419), a saturated knowledge base of a
/// few hundred facts is past the bound too: the `kb-since` view of the Zebra
/// programs (`examples/`, `examples/ein-bugs/`), of the programs of
/// `std.math.binary` (`tests/stdlib/math/binary/`) and of two of
/// `std.bool.inputs` (`tests/stdlib/bool/inputs/`). The calculator's
/// witnesses of a form at another count load, so they hold the tables too
/// (`examples/ein-bugs/wrong-answers/`).
/// `cargo test -p ein-render --test dot_wellformed
/// -- --nocapture` prints how many views are skipped, of how many files. A
/// view skipped anywhere else fails the sweep until its directory is named
/// here, which is where a bound that starts to skip a view it checked is
/// noticed. A run without `EIN_CORPUS_SLOW` reads no program under `tests/`,
/// and holds the directories it reads to the entries outside it; a run with
/// it, to every entry.
const SKIPPING: [&str; 23] = [
    "examples",
    "examples/arith",
    "examples/beyond_arith",
    "examples/binary_arithmetic",
    "examples/binary_logic",
    "examples/ein-bugs",
    "examples/ein-bugs/wrong-answers",
    "examples/word_problems",
    "stdlib/bool",
    "stdlib/math",
    "tests/saved",
    "tests/stdlib/bool/inputs",
    "tests/stdlib/bool/props",
    "tests/stdlib/math/binary",
    "tests/stdlib/math/binary-arith",
    "tests/stdlib/math/convert",
    "tests/stdlib/math/decimal",
    "tests/stdlib/math/decimal-arith",
    "tests/stdlib/math/digits",
    "tests/stdlib/math/gcd",
    "tests/stdlib/math/int",
    "tests/stdlib/math/num9",
    "tests/stdlib/math/rat",
];

/// The edges of a DOT text: its ` -> ` arrows.
fn edges(dot: &str) -> usize {
    dot.matches(" -> ").count()
}

/// Feed one DOT text to Graphviz, `-Tsvg`, discarding the SVG.
fn graphviz(dot: &str) -> Verdict {
    let mut child = Command::new("dot")
        .args(["-Tsvg", "-o", "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("graphviz was there a moment ago");
    child
        .stdin
        .take()
        .expect("piped")
        .write_all(dot.as_bytes())
        .expect("dot reads its input");
    let out = child.wait_with_output().expect("dot exits");
    let err = String::from_utf8_lossy(&out.stderr);
    match out.status.code() {
        // Graphviz exits 0 on a warning, so the stderr text is the real signal.
        Some(0) if err.trim().is_empty() => Verdict::Accepted,
        Some(_) => Verdict::Rejected(format!("exit {}: {}", out.status, err.trim())),
        // `code()` is `None` exactly when a signal killed it.
        None => Verdict::Crashed(format!("{}: {}", out.status, err.trim())),
    }
}

/// The view's digraphs, one per element.
///
/// A DOT file is a *sequence* of graphs, so a cut wherever the brace depth
/// returns to zero yields texts that are each a whole DOT file. Quoted strings
/// are tracked because a label may hold a brace; comments and HTML-like
/// labels — the other two hiding places — are not emitted by any view.
fn digraphs(view: &str) -> Vec<String> {
    let (mut out, mut cur) = (Vec::new(), String::new());
    let (mut depth, mut quoted, mut escaped) = (0i32, false, false);
    for ch in view.chars() {
        cur.push(ch);
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if quoted => escaped = true,
            '"' => quoted = !quoted,
            '{' if !quoted => depth += 1,
            '}' if !quoted => {
                depth -= 1;
                if depth == 0 {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => {}
        }
    }
    // What is left after the last `}` is the newline that ended it — hand it to
    // the graph it belongs to, so the parts are the view cut up rather than the
    // view minus its whitespace. Anything with content in it is a part of its
    // own: an unterminated graph is exactly what `dot` should be asked about.
    match out.last_mut() {
        Some(last) if cur.trim().is_empty() => last.push_str(&cur),
        _ if !cur.is_empty() => out.push(cur),
        _ => {}
    }
    out
}

/// The DOT inside one view.
///
/// Two things in a view are the shape op's and not Graphviz's: the `--- …`
/// separator lines that `ir-forms` and `slice` interleave between digraphs,
/// and the `NO PROOF` sentinel a budget-aborted solve returns instead of a
/// lattice. Strip the first, and let the second read as an empty view — what
/// is left is a stream of digraphs, which `dot` reads happily.
fn digraphs_only(view: &str) -> String {
    if view.trim() == "NO PROOF" {
        return String::new();
    }
    view.lines()
        .filter(|l| !l.starts_with("--- "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The classification, against the real tool: a complaint is about the bytes
/// and must still fail the sweep, whatever the crash arm does with a signal.
#[test]
fn a_complaint_is_a_rejection_and_not_a_crash() {
    require_graphviz();
    assert!(matches!(graphviz("digraph { a -> b; }"), Verdict::Accepted));
    assert!(matches!(graphviz("digraph { a -> }"), Verdict::Rejected(_)));
}

/// The splitter's two hazards, pinned: the cut is at depth zero rather than at
/// every `}`, and a brace inside a label is text and not depth.
#[test]
fn a_view_splits_into_whole_digraphs() {
    let nested = "digraph a { subgraph cluster_c { x; } }\ndigraph b { y; }\n";
    assert_eq!(digraphs(nested).len(), 2);
    assert_eq!(digraphs(nested).concat(), nested);

    let braced = "digraph a { n [label=\"{\"]; }\ndigraph b { m [label=\"\\\"}\"]; }\n";
    assert_eq!(digraphs(braced).len(), 2);
    assert_eq!(digraphs(braced).concat(), braced);

    // One graph stays one, which is what stops the crash fallback from
    // claiming a split it did not make.
    assert_eq!(digraphs("digraph a { x; }\n").len(), 1);
}

/// One view of one corpus file, as drawing it left it.
enum Drawn {
    /// The renderer refused it, past the reasons a fixture has.
    Errored(String),
    /// Empty, or skipped by its reason: nothing to put to Graphviz.
    Nothing,
    /// Past [`DOT_LAYOUT_BOUND`], with its edges: put to nothing.
    Skipped(usize),
    /// The DOT to put to Graphviz.
    Dot(String),
}

/// Every view of one corpus file, drawn: the engine's half of the sweep,
/// one unit a file, so that the file is loaded once.
struct FileViews {
    /// The file, from the repo's root.
    name: String,
    /// Its directory, from the repo's root.
    dir: String,
    /// Each view in `views`' order.
    drawn: Vec<(&'static str, Drawn)>,
}

/// Every view of one corpus file, drawn and not yet put to Graphviz.
fn draw_file(path: &std::path::Path, views: &[&'static str]) -> FileViews {
    let rel = path.strip_prefix(repo_root()).unwrap_or(path);
    let mut file = FileViews {
        name: rel.display().to_string(),
        dir: rel
            .parent()
            .map(|d| d.display().to_string())
            .unwrap_or_default(),
        drawn: Vec::new(),
    };
    let Ok(text) = std::fs::read_to_string(path) else {
        return file;
    };
    // A file that does not parse has no view, one that does not load has
    // no KB view, and one whose root does not saturate has no `kb-since` and
    // no constraint view, each drawn of root's fixpoint:
    // the corpus's load- and compile-error fixtures. The file is loaded once
    // and every KB view drawn from a copy of the load, as
    // `id_order_invariance` digests them.
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Ok(forms) = ein_ir::saved::parse_text(&mut ast, &text, path.to_str()) else {
        return file;
    };
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent());
    let (loads, saturates) = match &kb {
        Ok(kb) => (
            true,
            ein_infer::region::root_firings(
                &mut kb.clone(),
                &mut terms.clone(),
                &ast,
                &mut ein_infer::Events::off(),
            )
            .is_ok(),
        ),
        Err(_) => (false, false),
    };
    for view in views.iter() {
        let drawn = if KB_VIEWS.contains(view) {
            match &kb {
                Ok(kb) => kb_dot_shape(&ast, &mut terms.clone(), &mut kb.clone(), view),
                Err(e) => Err(e.to_string()),
            }
        } else {
            let mut ast = Ast::new();
            let forms =
                ein_ir::saved::parse_text(&mut ast, &text, path.to_str()).expect("it parsed above");
            dot_shape(&mut ast, &mut Terms::new(), &forms, path.parent(), view)
        };
        let out = match drawn {
            Ok(out) => out,
            Err(_) if !loads && KB_VIEWS.contains(view) => {
                file.drawn.push((view, Drawn::Nothing));
                continue;
            }
            Err(_)
                if !saturates
                    && matches!(*view, "kb-since" | "constraints" | "constraints-fold") =>
            {
                file.drawn.push((view, Drawn::Nothing));
                continue;
            }
            Err(e) => {
                file.drawn.push((view, Drawn::Errored(e)));
                continue;
            }
        };
        let dot = digraphs_only(&out);
        // An empty view is not a broken one: a file with no rules renders
        // no rule library, and `(config …)` renders nothing at all.
        let drawn = if dot.trim().is_empty() {
            Drawn::Nothing
        } else if edges(&dot) > DOT_LAYOUT_BOUND {
            Drawn::Skipped(edges(&dot))
        } else {
            Drawn::Dot(dot)
        };
        file.drawn.push((view, drawn));
    }
    file
}

/// What Graphviz made of one view: accepted, accepted digraph by digraph
/// once `dot` had died on the stream (`Ok(true)`), or why not.
fn put_to_graphviz(dot: &str) -> Result<bool, String> {
    match graphviz(dot) {
        Verdict::Accepted => Ok(false),
        Verdict::Rejected(why) => Err(why),
        Verdict::Crashed(why) => {
            // Ask again, one digraph at a time. All accepted, and more
            // than one of them, is the answer that the stream was fine
            // and `dot` was not.
            let parts = digraphs(dot);
            let per: Vec<String> = parts
                .iter()
                .enumerate()
                .filter_map(|(i, part)| match graphviz(part) {
                    Verdict::Accepted => None,
                    Verdict::Rejected(w) | Verdict::Crashed(w) => Some(format!("digraph {i}: {w}")),
                })
                .collect();
            if per.is_empty() && parts.len() > 1 {
                return Ok(true);
            }
            Err(format!("{why}\n  {}", per.join("\n  ")))
        }
    }
}

/// The sweep, **on the pool** (`ein_corpus::pool::map`), in two passes: each
/// file's views drawn, a unit a file, so that it is loaded once; then each
/// view put to Graphviz, a unit a (file, view), so that one file's views are
/// laid out side by side, its `dot` processes being independent. What is
/// reported is gathered in the files' and the views' order, so the report
/// reads the same at any `EIN_TEST_JOBS`.
#[test]
fn graphviz_accepts_every_view_of_every_corpus_file() {
    require_graphviz();
    let views = all_views();
    let slow = slow_too();
    let root = repo_root();
    let rel = |f: &std::path::Path| f.strip_prefix(&root).unwrap_or(f).display().to_string();
    let files: Vec<std::path::PathBuf> = corpus_files()
        .into_iter()
        .filter(|f| slow || !under_tests(f.strip_prefix(&root).unwrap_or(f)))
        .collect();
    let drawn = ein_corpus::pool::map(
        "dot_wellformed",
        &files,
        |f| (rel(f), "views".to_string()),
        |f| draw_file(f, &views),
    );
    let units: Vec<(usize, usize)> = drawn
        .iter()
        .enumerate()
        .flat_map(|(i, file)| {
            file.drawn
                .iter()
                .enumerate()
                .filter(|(_, (_, d))| matches!(d, Drawn::Dot(_)))
                .map(move |(j, _)| (i, j))
        })
        .collect();
    let verdicts = ein_corpus::pool::map(
        "dot_wellformed",
        &units,
        |&(i, j)| (drawn[i].name.clone(), drawn[i].drawn[j].0.to_string()),
        |&(i, j)| match &drawn[i].drawn[j].1 {
            Drawn::Dot(dot) => put_to_graphviz(dot),
            _ => unreachable!("a unit is a view drawn"),
        },
    );
    let mut verdict = units.iter().zip(verdicts);
    let (mut bad, mut checked, mut resplit) = (Vec::new(), 0usize, Vec::new());
    let (mut skipped, mut skipped_files) = (Vec::new(), 0usize);
    let mut skipping = std::collections::BTreeSet::new();
    for file in &drawn {
        let name = &file.name;
        let mut skips = false;
        for (view, d) in &file.drawn {
            match d {
                Drawn::Nothing => {}
                Drawn::Errored(e) => bad.push(format!("{name} [{view}]\n  the view errored: {e}")),
                Drawn::Skipped(edges) => {
                    skips = true;
                    skipped.push((format!("{name} [{view}]"), *edges));
                }
                Drawn::Dot(dot) => {
                    let (_, said) = verdict.next().expect("a verdict a view drawn");
                    checked += 1;
                    match said {
                        Ok(false) => {}
                        Ok(true) => resplit.push(format!(
                            "{name} [{view}] ({} digraphs)",
                            digraphs(dot).len()
                        )),
                        Err(why) => bad.push(format!("{name} [{view}]\n  {why}")),
                    }
                }
            }
        }
        if skips {
            skipped_files += 1;
            skipping.insert(file.dir.clone());
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {checked} views are not valid DOT:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!(
        "graphviz{}: {checked} non-empty views of {} files accepted, {} skipped past \
         {DOT_LAYOUT_BOUND} edges ({skipped_files} files)",
        if slow {
            " [every corpus program]"
        } else {
            " [outside tests/]"
        },
        files.len(),
        skipped.len()
    );
    for (view, edges) in &skipped {
        eprintln!("  skipped: {view} ({edges} edges)");
    }
    if !resplit.is_empty() {
        eprintln!(
            "  {} of them digraph by digraph, `dot` having died on the stream: {}",
            resplit.len(),
            resplit.join(", ")
        );
    }
    assert!(checked >= 500, "only {checked} views reached Graphviz");
    let want: Vec<&str> = SKIPPING
        .iter()
        .copied()
        .filter(|d| slow || !under_tests(std::path::Path::new(d)))
        .collect();
    assert_eq!(
        skipping.iter().map(String::as_str).collect::<Vec<_>>(),
        want,
        "the directories with a view past {DOT_LAYOUT_BOUND} edges, which is \
         skipped and not put to Graphviz, are not the ones `SKIPPING` names \
         (`-- --nocapture` lists every skipped view)"
    );
}

// ── A trace's drawings ───────────────────────────────────────────────

/// The corpus's region trace, drawn as `ein solve FILE --trace OUT.md` draws
/// it: the op the shape golden digests.
mod corpus_ops;

/// The programs whose traces' slices are put to Graphviz per commit: the two
/// smallest of the calculator's programs a slice of which Graphviz refused
/// while slices were laid out left to right (*triangulation failed*, *lost
/// edge*), one in each form. A trace is not a view: [`all_views`] has none,
/// and the corpus's traces hold some hundred and sixty thousand slices, past
/// any per-commit budget, so these two stand for them. A slice under `--fold`
/// is the canonical one, every fact of it the step's premise or conclusion,
/// which is never folded (Q-M1f.429).
const TRACE_WITNESSES: [&str; 2] = [
    "examples/arith/div.ein",
    "examples/binary_arithmetic/first.ein",
];

/// **Every drawing of the witnesses' traces is accepted by Graphviz**: each
/// inline `dot` block of the region trace, a step's slice, put to `dot`, on
/// the pool, a unit a block. Graphviz's refusal is a fault of the DOT's, as
/// for every view; where it lays a slice out is no claim of this test.
#[test]
fn graphviz_accepts_every_slice_of_the_witnesses_traces() {
    require_graphviz();
    let root = repo_root();
    let mut blocks: Vec<(&str, usize, String)> = Vec::new();
    for rel in TRACE_WITNESSES {
        let md = corpus_ops::run(
            &mut Terms::new(),
            &root.join(rel),
            corpus_ops::Op::RegionTrace,
        )
        .unwrap_or_else(|| panic!("{rel}: no trace"));
        let before = blocks.len();
        for (i, rest) in md.split("```dot\n").skip(1).enumerate() {
            let dot = rest.split("```").next().expect("a block").to_string();
            blocks.push((rel, i + 1, dot));
        }
        assert!(blocks.len() > before, "{rel}: a trace with no drawing");
    }
    let verdicts = ein_corpus::pool::map(
        "dot_wellformed::traces",
        &blocks,
        |(rel, i, _)| (rel.to_string(), format!("slice {i}")),
        |(_, _, dot)| put_to_graphviz(dot),
    );
    let bad: Vec<String> = blocks
        .iter()
        .zip(verdicts)
        .filter_map(|((rel, i, _), said)| said.err().map(|why| format!("{rel} slice {i}\n  {why}")))
        .collect();
    eprintln!(
        "dot_wellformed: {} slices of {} traces put to Graphviz",
        blocks.len(),
        TRACE_WITNESSES.len()
    );
    assert!(
        bad.is_empty(),
        "{} slices refused by Graphviz:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

// ── The structure's graphs ────────────────────────────────────────────

// What *a program searches* means is the determinism sweeps' one definition,
// `corpus_ops::searches`.

/// The lines of one of the structure's graphs: its ` -- ` strokes. They are
/// undirected graphs, so [`edges`] counts none of them.
fn strokes(dot: &str) -> usize {
    dot.matches(" -- ").count()
}

/// **The graphs of `ein render h-space` past [`DOT_LAYOUT_BOUND`]**, each
/// by its program and its view: not put to Graphviz, as a view of a program
/// is not. The list is exact, a graph and not a directory: one past the
/// bound that is not named fails the sweep, and so does a name whose graph
/// is no longer past it. The under-determined zebra's exclusion graph is the
/// pairs of its 96 candidates, and its edges read one application back are
/// mostly of two questions; the two programs whose `R` passes 2^64
/// commitments tie every candidate to every other. A run without
/// `EIN_CORPUS_SLOW` reads no `slow` entry, and holds the list to the others.
const STRUCTURE_SKIPPING: [(&str, &str); 8] = [
    (
        "examples/ein-bugs/wrong-answers/count-past-2-128.ein",
        "coupling",
    ),
    (
        "examples/ein-bugs/wrong-answers/count-past-2-128.ein",
        "one-step",
    ),
    (
        "examples/ein-bugs/wrong-answers/count-past-2-64.ein",
        "coupling",
    ),
    (
        "examples/ein-bugs/wrong-answers/count-past-2-64.ein",
        "one-step",
    ),
    ("examples/zebra2-minus-15-obligations.ein", "exclusion"),
    ("examples/zebra2-minus-15-obligations.ein", "one-step"),
    ("examples/zebra2-minus-15.ein", "exclusion"),
    ("examples/zebra2-minus-15.ein", "one-step"),
];

/// What the sweep found of one program's structure.
#[derive(Default)]
struct StructureSweep {
    /// The program searches, and its structure was read.
    read: bool,
    bad: Vec<String>,
    /// Graphs put to Graphviz.
    checked: usize,
    /// Graphs past [`DOT_LAYOUT_BOUND`]: the view and its lines.
    skipped: Vec<(&'static str, usize)>,
    /// What was stopped, cut or left unread, a sentence each: empty where
    /// the structure is whole.
    unread: Vec<String>,
    /// What reading it cost: root's layer's saturations, the small KB's and
    /// the whole node's; then root's layer's facts beyond root and the small
    /// KB's beyond its own root.
    work: [u64; 5],
}

/// The four graphs of one program that searches, put to Graphviz.
fn sweep_structure(path: &std::path::Path) -> StructureSweep {
    let mut out = StructureSweep::default();
    let rel = path.strip_prefix(repo_root()).unwrap_or(path);
    let name = rel.display().to_string();
    let Ok(text) = std::fs::read_to_string(path) else {
        return out;
    };
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Ok(forms) = ein_ir::saved::parse_text(&mut ast, &text, path.to_str()) else {
        return out;
    };
    let Ok(mut kb) = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()) else {
        return out;
    };
    if !corpus_ops::searches(&ast, &mut terms.clone(), &mut kb.clone()) {
        return out;
    }
    let asked = ein_infer::structure::WholeOptions::default();
    let whole = match ein_infer::structure::whole(&mut kb, &mut terms, &ast, &asked) {
        Ok(w) => w,
        Err(e) => {
            out.bad
                .push(format!("{name}\n  the structure was refused: {e}"));
            return out;
        }
    };
    out.read = true;
    out.unread = ein_render::structure::unread(&whole);
    let (root, small) = (&whole.theory.root, whole.theory.small.as_ref().ok());
    let node = whole.parts.as_ref().and_then(|p| p.coupling.node);
    out.work = [
        root.work.saturations,
        small.map_or(0, |s| s.work.saturations),
        node.map_or(0, |n| n.saturations),
        root.work.facts,
        small.map_or(0, |s| s.work.facts),
    ];
    // The exclusion graph draws a pair of the theory's own list, or a pair
    // root excludes that is alive in the small KB. A pair root excludes that
    // is dead in the small KB by a member the theory kills alone is in
    // neither, and would be drawn by no stroke: there is none where the
    // small KB derives no more than the program, which is held here and not
    // argued.
    let theory: std::collections::BTreeSet<(u32, u32)> =
        whole.theory.excluded().iter().copied().collect();
    let undrawn = root
        .excluded
        .iter()
        .zip(whole.theory.of_the_theory())
        .filter(|(pair, of_theory)| *of_theory && !theory.contains(pair))
        .count();
    if undrawn > 0 {
        out.bad.push(format!(
            "{name} [exclusion]\n  {undrawn} pairs root excludes are in neither stroke: dead in \
             the small KB by a member it kills alone"
        ));
    }
    for graph in ein_render::structure::graphs(&whole, &name) {
        let lines = strokes(&graph.dot);
        if lines != graph.lines {
            out.bad.push(format!(
                "{name} [{}]\n  the graph says {} lines and draws {lines}",
                graph.name, graph.lines
            ));
        }
        if lines > DOT_LAYOUT_BOUND {
            out.skipped.push((graph.name, lines));
            continue;
        }
        out.checked += 1;
        match graphviz(&graph.dot) {
            Verdict::Accepted => {}
            Verdict::Rejected(why) | Verdict::Crashed(why) => {
                out.bad.push(format!("{name} [{}]\n  {why}", graph.name));
            }
        }
    }
    out
}

/// **Graphviz accepts every graph `ein render h-space` writes**, of every
/// corpus program that searches: the exclusion graph, the cover, the
/// questions' hypergraph and its edges read one application back, each one
/// graph, each put to `dot -Tsvg` with nothing to say of it.
///
/// Without `EIN_CORPUS_SLOW`, the programs that search outside `tests/` but
/// for the corpus's `slow` entries; under `EIN_CORPUS_SLOW=1`, every one that
/// searches. A program that does not search has no candidate, and
/// its four graphs are a title each, which the view's own test draws. A
/// graph past [`DOT_LAYOUT_BOUND`] is skipped as a view is, and named
/// ([`STRUCTURE_SKIPPING`]). Nothing here holds a picture: the SVG is
/// discarded, a layout being Graphviz's to choose, and the DOT is what the
/// determinism sweeps hold (`ein-infer/tests/structure_invariance.rs`). And
/// every structure read is held whole at the default budget: one with a
/// layer stopped, a cover cut or a reading refused fails here by name.
#[test]
fn graphviz_accepts_the_structure_graphs_of_every_program_that_searches() {
    require_graphviz();
    let slow = slow_too();
    let root = repo_root();
    let slow_entries: std::collections::BTreeSet<String> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| e.path.clone())
        .collect();
    let rel = |f: &std::path::Path| f.strip_prefix(&root).unwrap_or(f).display().to_string();
    let files: Vec<std::path::PathBuf> = corpus_files()
        .into_iter()
        .filter(|f| {
            slow || !(under_tests(f.strip_prefix(&root).unwrap_or(f))
                || slow_entries.contains(&rel(f)))
        })
        .collect();
    let found = ein_corpus::pool::map(
        "dot_wellformed",
        &files,
        |f| (rel(f), "structure".to_string()),
        |f| sweep_structure(f),
    );
    let (mut bad, mut read, mut checked) = (Vec::new(), 0usize, 0usize);
    let mut skipped: Vec<(String, &'static str, usize)> = Vec::new();
    let mut not_whole: Vec<String> = Vec::new();
    let mut work = [0u64; 5];
    for (file, sweep) in files.iter().zip(found) {
        bad.extend(sweep.bad);
        read += usize::from(sweep.read);
        checked += sweep.checked;
        for (total, of) in work.iter_mut().zip(sweep.work) {
            *total += of;
        }
        skipped.extend(sweep.skipped.into_iter().map(|(v, n)| (rel(file), v, n)));
        if !sweep.unread.is_empty() {
            not_whole.push(format!("{}\n  {}", rel(file), sweep.unread.join("\n  ")));
        }
    }
    // The graphs drawn are of whole structures: at the default budget no
    // program read here has a layer stopped, a cover cut or a reading
    // refused. One that does is named, with what it left unread.
    assert!(
        not_whole.is_empty(),
        "{} of the {read} structures read are not whole at the default budget:\n\n{}",
        not_whole.len(),
        not_whole.join("\n\n")
    );
    assert!(
        bad.is_empty(),
        "{} of the structure's graphs are not valid DOT:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!(
        "graphviz, the structure's graphs{}: {read} programs that search, {checked} graphs \
         accepted, {} skipped past {DOT_LAYOUT_BOUND} lines",
        if slow {
            " [every corpus program]"
        } else {
            " [outside tests/, the slow entries aside]"
        },
        skipped.len()
    );
    for (program, view, lines) in &skipped {
        eprintln!("  skipped: {program} [{view}] ({lines} lines)");
    }
    // What reading them cost, in the read-out's units: the sweep's own
    // work, which a run prints so that no page has to remember it.
    eprintln!(
        "  the {read} structures: root's layers {} saturations and {} facts beyond root, the \
         small KBs' {} saturations and {} facts beyond their own roots, {} whole nodes",
        work[0], work[3], work[1], work[4], work[2]
    );
    assert!(read >= 100, "only {read} programs' structures were read");
    assert_eq!(
        checked + skipped.len(),
        4 * read,
        "four graphs a program that searches"
    );
    let mut want: Vec<(&str, &str)> = STRUCTURE_SKIPPING
        .iter()
        .copied()
        .filter(|(p, _)| slow || !slow_entries.contains(*p))
        .collect();
    want.sort_unstable();
    let mut got: Vec<(&str, &str)> = skipped.iter().map(|(p, v, _)| (p.as_str(), *v)).collect();
    got.sort_unstable();
    assert_eq!(
        got, want,
        "the structure's graphs past {DOT_LAYOUT_BOUND} lines, which are skipped and not put \
         to Graphviz, are not the ones `STRUCTURE_SKIPPING` names (`-- --nocapture` lists \
         every skipped graph)"
    );
}
