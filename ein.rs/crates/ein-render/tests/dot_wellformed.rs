//! S1a.5.1 acceptance — Graphviz accepts every file the engine emits.
//!
//! A digest sweep (`corpus_shapes.md5`) proves every view still renders the
//! bytes it rendered. It cannot prove those bytes are *valid*: a golden of a
//! broken file reproduces just as well as a golden of a good one. So every
//! view of every corpus entry also goes through `dot -Tsvg`, which is the only
//! authority on whether the grammar and the attribute values are real.
//!
//! ## When Graphviz is the one that is broken
//!
//! `ubuntu-24.04` — the per-commit runner — ships `graphviz 2.42.2-9ubuntu0.1`,
//! which reports itself as 2.43.0 and dates from 2019; a working checkout is
//! more likely to have 12.x or 15.x. On that build five of `stdlib/slots.ein`'s
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
//! **It fails rather than skips when Graphviz is missing.** It used to skip —
//! on a stderr line `cargo test` captures for a passing test. A check that
//! reports a pass because its tool is absent is not a check, and "loudly" was
//! the word doing the lying.
//!
//! **Nor does it skip a view that errors** (M1e2 S1e2.1c.5). Until then a view
//! whose renderer returned an error was passed over in silence, so deleting
//! the engine a view runs would have left this test green over views it no
//! longer checked. A file that does not parse has no view, one that does not
//! load has no KB view, and one whose root does not saturate has no
//! `kb-since`, the one view that saturates: those are the corpus's load- and
//! compile-error fixtures, and each is skipped by its reason. Every other
//! error fails the sweep.
//!
//! **The walk's three views are gone**, `lattice`, `lattice-full` and
//! `slice`: they rendered the walk's proof, and went with the walk at M1e2
//! S1e2.1c.5 (2026-11-03). Until that commit this test left them out by name,
//! so that a view erroring once its engine was gone could not pass here.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_ir::{Ast, parse};
use ein_render::shape::{KB_VIEWS, all_views, dot_shape};
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

/// A view with more edges than this is laid out by [`BOUNDED_LAYOUT`], not
/// by `dot`'s own layout.
///
/// `dot`'s layered layout is superlinear in the edges, and a library of facts
/// is where that shows. `stdlib/digits.ein` (M1f S1f.6.2) is the calculator's
/// 1 664 table rows over eleven shared names, and every view of it has some
/// 7 900 edges: `dot` 16.0.0 had not drawn one after five minutes, and `fdp`,
/// the layout a KB view suggests, not after 100 s. Table by table, the KB view
/// of `fa` alone (1 014 edges) takes `dot` 0.5 s, `fa` and `fs` (2 021) 3.2 s,
/// with `cl` (3 227) 9.9 s, and `md` alone (4 514) 23.7 s (2026-11-11). The
/// largest view the corpus had before that module has 361 edges, and its
/// slowest took 0.43 s, so no view that `dot` drew then is past the bound.
///
/// **The bound is under the smallest table** since M1f S1f.6.3 (it was 1 500
/// for a stage): the KB view of a program that imports `std.arith` holds
/// `fa`'s 200 rows, some 1 200 edges, and `dot` takes 0.55 s on each of its
/// six KB views, where the bounded layout takes 0.1 s. One such program is
/// affordable and a suite of them is not, and a later operation's programs
/// hold `md`'s 900 rows. So a view that holds a table is past the bound, and
/// what `dot` lays out is what it laid out before the tables.
///
/// What a view past the bound is still held to is this file's claim, that
/// **Graphviz accepts the bytes**: the same parser reads the same grammar,
/// and the same renderer writes the same attributes to SVG and warns of the
/// same unknown shape, arrow, style and colour (tried on each). What it is
/// not held to is `dot`'s own layout of it, which nobody can wait for.
/// Not a skip: every view still goes to Graphviz, and one it complains of
/// fails.
const DOT_LAYOUT_BOUND: usize = 1000;

/// The layout past [`DOT_LAYOUT_BOUND`]: `fdp`, the one every KB view names
/// as its *suggested layout*, stopped after one iteration, with no overlap
/// removal and straight edges. The layout is not the subject, and this one
/// takes about two seconds on the largest view there is, where `osage` took
/// seven and `twopi` five.
const BOUNDED_LAYOUT: [&str; 4] = ["-Kfdp", "-Gmaxiter=1", "-Goverlap=true", "-Gsplines=false"];

/// The edges of a DOT text: its ` -> ` arrows.
fn edges(dot: &str) -> usize {
    dot.matches(" -> ").count()
}

/// Feed one DOT text to Graphviz, `-Tsvg`, discarding the SVG: `dot`'s
/// layout, or [`BOUNDED_LAYOUT`] past [`DOT_LAYOUT_BOUND`].
fn graphviz(dot: &str) -> Verdict {
    let mut args = vec!["-Tsvg", "-o", "/dev/null"];
    if edges(dot) > DOT_LAYOUT_BOUND {
        args.extend(BOUNDED_LAYOUT);
    }
    let mut child = Command::new("dot")
        .args(&args)
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

#[test]
fn graphviz_accepts_every_view_of_every_corpus_file() {
    require_graphviz();
    let views = all_views();
    let (mut bad, mut checked) = (Vec::new(), 0usize);
    // Views that only Graphviz's own cross-graph bug objected to, kept for the
    // summary line: the check reached a verdict on them, it just took a second
    // question to get there.
    let mut resplit: Vec<String> = Vec::new();
    for path in &corpus_files() {
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        // A file that does not parse has no view, one that does not load has
        // no KB view, and one whose root does not saturate has no `kb-since`:
        // the corpus's load- and compile-error fixtures.
        let (loads, saturates) = {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
                continue;
            };
            match ein_ir::load(&mut ast, &mut terms, &forms, path.parent()) {
                Ok(mut kb) => (
                    true,
                    ein_infer::region::root_firings(
                        &mut kb,
                        &mut terms,
                        &ast,
                        &mut ein_infer::Events::off(),
                    )
                    .is_ok(),
                ),
                Err(_) => (false, false),
            }
        };
        for view in &views {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let forms = parse(&mut ast, &text, path.to_str()).expect("it parsed above");
            let out = match dot_shape(&mut ast, &mut terms, &forms, path.parent(), view) {
                Ok(out) => out,
                Err(_) if !loads && KB_VIEWS.contains(view) => continue,
                Err(_) if !saturates && *view == "kb-since" => continue,
                Err(e) => {
                    bad.push(format!("{name} [{view}]\n  the view errored: {e}"));
                    continue;
                }
            };
            let dot = digraphs_only(&out);
            // An empty view is not a broken one: a file with no rules renders
            // no rule library, and `(config …)` renders nothing at all.
            if dot.trim().is_empty() {
                continue;
            }
            checked += 1;
            let why = match graphviz(&dot) {
                Verdict::Accepted => continue,
                Verdict::Rejected(why) => why,
                Verdict::Crashed(why) => {
                    // Ask again, one digraph at a time. All accepted, and more
                    // than one of them, is the answer that the stream was fine
                    // and `dot` was not.
                    let parts = digraphs(&dot);
                    let per: Vec<String> = parts
                        .iter()
                        .enumerate()
                        .filter_map(|(i, part)| match graphviz(part) {
                            Verdict::Accepted => None,
                            Verdict::Rejected(w) | Verdict::Crashed(w) => {
                                Some(format!("digraph {i}: {w}"))
                            }
                        })
                        .collect();
                    if per.is_empty() && parts.len() > 1 {
                        resplit.push(format!("{name} [{view}] ({} digraphs)", parts.len()));
                        continue;
                    }
                    format!("{why}\n  {}", per.join("\n  "))
                }
            };
            bad.push(format!("{name} [{view}]\n  {why}"));
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {checked} views are not valid DOT:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("graphviz: {checked} non-empty views accepted");
    if !resplit.is_empty() {
        eprintln!(
            "  {} of them digraph by digraph, `dot` having died on the stream: {}",
            resplit.len(),
            resplit.join(", ")
        );
    }
    assert!(checked >= 500, "only {checked} views reached Graphviz");
}
