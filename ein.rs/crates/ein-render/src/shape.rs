//! Every DOT view of a file, as one text — the S1a.5.1 diff.
//!
//! The renderers *do* have a CLI surface, but not all of it and not in every
//! combination: `ein render` reaches four of them, `kb.to_dot`'s keyword
//! surface is library-only, and the slice cones are reachable only through a
//! `--trace` that [S1a.5.2](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.2_trace_and_answer.md)
//! has not built yet. So the renderers are compared the way the loader and
//! the compiler were: both implementations render the same views over the
//! same corpus and the texts are diffed (`utils/ir_oracle.py`'s `dot-shape`
//! op is the other half).
//!
//! Unlike `plan-shape` or `kb-shape` this invents no rendering of its own.
//! Every view calls a renderer entry point exactly as a CLI subcommand or the
//! trace calls it, so what the diff compares is the artefact a user sees —
//! which is the whole point of the byte gate.

use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_infer::SharedMemo;
use ein_ir::{Ast, NodeId};

use crate::ir_dot::{DotOpts, TraceView, to_dot, to_dot_form};
use crate::kb_dot::{ColourBy, KbDotOpts};
use crate::lattice_dag::{LatticeSource, LatticeView, render_lattice};
use crate::rules::{RuleMode, render_rules_forms};

/// The views that need only the parsed forms.
pub const PARSE_VIEWS: [&str; 8] = [
    "ir",
    "ir-levi",
    "ir-overlay",
    "ir-trace-dag",
    "ir-forms",
    "rules",
    "rules-overlay",
    "constraints",
];

/// The views that need a loaded KB.
pub const KB_VIEWS: [&str; 6] = [
    "kb",
    "kb-origin",
    "kb-none",
    "kb-no-types",
    "kb-no-instances",
    "kb-since",
];

/// The views that run the engine.
pub const SOLVE_VIEWS: [&str; 3] = ["lattice", "lattice-full", "slice"];

/// The modes of the `--dump-states` diff —
/// [S1a.5.3](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.3_state_dumps.md).
pub const DUMP_MODES: [&str; 5] = ["monotonic", "lattice", "progress", "abort", "snapshot"];

/// The modes of the trace / answer diff — [S1a.5.2](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.2_trace_and_answer.md).
pub const TRACE_MODES: [&str; 3] = ["trace", "answer", "no-proof"];

pub fn all_views() -> Vec<&'static str> {
    PARSE_VIEWS
        .iter()
        .chain(KB_VIEWS.iter())
        .chain(SOLVE_VIEWS.iter())
        .copied()
        .collect()
}

fn parse_view(ast: &Ast, forms: &[NodeId], view: &str) -> Result<String, String> {
    let opts = |rule_mode, trace_view, levi| DotOpts {
        rule_mode,
        trace_view,
        levi,
    };
    let default = DotOpts::default();
    Ok(match view {
        "ir" => to_dot(ast, forms, default),
        "ir-levi" => to_dot(
            ast,
            forms,
            opts(RuleMode::SideBySide, TraceView::PerStep, true),
        ),
        "ir-overlay" => to_dot(
            ast,
            forms,
            opts(RuleMode::Overlay, TraceView::PerStep, false),
        ),
        "ir-trace-dag" => to_dot(
            ast,
            forms,
            opts(RuleMode::SideBySide, TraceView::Dag, false),
        ),
        "ir-forms" => {
            // One digraph per top-level form, through the single-node
            // dispatch — the only way `(config …)`'s empty string is
            // reachable.
            let mut out: Vec<String> = Vec::new();
            for (i, f) in forms.iter().enumerate() {
                let head = ast.head_name(*f).unwrap_or("?");
                out.push(format!("--- {i} {head}\n{}", to_dot_form(ast, *f, default)));
            }
            out.join("\n")
        }
        "rules" | "rules-overlay" => {
            let mode = if view.ends_with("overlay") {
                RuleMode::Overlay
            } else {
                RuleMode::SideBySide
            };
            let rules: Vec<NodeId> = forms
                .iter()
                .copied()
                .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
                .collect();
            render_rules_forms(ast, &rules, mode)
        }
        "constraints" => crate::constraints::render_constraints(ast, forms, "constraints"),
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

fn kb_view(ast: &Ast, terms: &mut Terms, kb: &mut Kb, view: &str) -> Result<String, String> {
    let d = KbDotOpts::default;
    Ok(match view {
        "kb" => crate::kb_dot::to_dot(kb, terms, &d()),
        "kb-origin" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                colour_by: ColourBy::Origin,
                ..d()
            },
        ),
        "kb-none" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                colour_by: ColourBy::None,
                name: "plain",
                ..d()
            },
        ),
        "kb-no-types" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                include_types: false,
                ..d()
            },
        ),
        "kb-no-instances" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                include_instances: false,
                ..d()
            },
        ),
        "kb-since" => {
            let root = kb.snapshot();
            saturate(ast, terms, kb)?;
            crate::kb_dot::to_dot(
                kb,
                terms,
                &KbDotOpts {
                    since: Some(&root),
                    name: "state",
                    ..d()
                },
            )
        }
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

fn saturate(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<(), String> {
    let mut events = ein_infer::events::Events::off();
    let mut s = ein_infer::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).map_err(|e| e.to_string())?;
    sat.saturate(&mut s, None, &mut |_| {})
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn solve_view(ast: &Ast, terms: &mut Terms, kb: &mut Kb, view: &str) -> Result<String, String> {
    use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};

    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(60),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved =
        solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())?;
    let Some(proof) = solved.proof.as_ref() else {
        return Ok("NO PROOF".to_string());
    };
    Ok(match view {
        "lattice" => render_lattice(
            terms,
            LatticeSource::Proof(proof),
            LatticeView::Solution,
            "lattice",
        ),
        // Always the fallback-with-a-note path: `solve` never populates a
        // per-commitment SetNode DAG, which is what `--view full`'s help says.
        "lattice-full" => render_lattice(
            terms,
            LatticeSource::Proof(proof),
            LatticeView::Full,
            "lattice",
        ),
        // The three shapes `trace.linearize` builds, with its own arguments:
        // the whole-commitment cone, the per-firing step diagram, and the
        // reductio.
        "slice" => {
            let mut out: Vec<String> = Vec::new();
            for (i, s) in proof.solutions.iter().enumerate() {
                out.push(format!("--- solution {i}"));
                out.push(crate::slice::render_slice(
                    terms,
                    &s.commitment,
                    &s.firings,
                    Some(&s.kb),
                    &format!("sol{i}"),
                    None,
                    None,
                ));
                for (n, firing) in s.firings.iter().take(5).enumerate() {
                    out.push(format!("--- step {i}.{}", n + 1));
                    out.push(crate::slice::render_slice(
                        terms,
                        &[],
                        std::slice::from_ref(firing),
                        Some(&s.kb),
                        &format!("step{}", n + 1),
                        None,
                        None,
                    ));
                }
            }
            for (i, d) in proof.dead_commitments.iter().enumerate() {
                out.push(format!("--- dead {i}"));
                out.push(crate::slice::render_slice(
                    terms,
                    &d.commitment,
                    &[],
                    Some(kb),
                    "reductio",
                    Some((&d.unsat_core, &d.learned_clause)),
                    None,
                ));
            }
            out.join("\n")
        }
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

/// Render one view of a parsed program.
pub fn dot_shape(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    view: &str,
) -> Result<String, String> {
    if PARSE_VIEWS.contains(&view) {
        return parse_view(ast, forms, view);
    }
    let mut kb = ein_ir::load(ast, terms, forms, base_dir).map_err(|e| e.to_string())?;
    if KB_VIEWS.contains(&view) {
        return kb_view(ast, terms, &mut kb, view);
    }
    solve_view(ast, terms, &mut kb, view)
}

// ── The trace and answer surface ───────────────────────────────────

/// The bounded solve a trace mode runs.
///
/// `first` is the *fast* regime, and it is what the `trace` mode wants: a
/// puzzle that stops at its first solution reaches one in a dozen enterings
/// and hands the renderer a spine of several hundred firings, where the
/// exhaustive regime spends its whole budget and aborts with nothing to
/// narrate. The exhaustive regime is what the `answer` mode wants, for the
/// opposite reason: `Ambiguity`, `Contradiction` and `Aborted` are only
/// reachable there, and they are three of the table's four shapes.
fn solve_for_trace(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    store_lattice: bool,
    first: bool,
) -> Result<ein_infer::solve::Solved, String> {
    use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
    let opts = SolveOptions {
        stop_after: if first { Some(1) } else { None },
        max_set_size: 3,
        max_enterings: Some(if first { 300 } else { 60 }),
        on_budget: OnBudget::Verdict,
        store_lattice,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())
}

/// The four `solve --trace` flags, as one value.
#[derive(Clone, Copy, Default)]
struct Flags {
    diagrams: bool,
    full_kb: bool,
    relevant: bool,
    reorder: bool,
}

fn trace_markdown(
    ast: &Ast,
    terms: &Terms,
    root: &Kb,
    solved: &ein_infer::solve::Solved,
    flags: Flags,
) -> String {
    let trace = crate::trace::linearize(
        ast,
        terms,
        root,
        solved,
        crate::trace::LinearizeOpts {
            diagrams: flags.diagrams,
            full_kb_snapshots: flags.full_kb,
            relevant: flags.relevant,
        },
    );
    crate::trace::render_markdown(
        &trace,
        if flags.reorder {
            crate::trace::Mode::Reorder
        } else {
            crate::trace::Mode::Engine
        },
        flags.diagrams,
    )
}

/// Render one mode of the trace / answer surface.
pub fn trace_shape(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    mode: &str,
) -> Result<String, String> {
    let mut kb = ein_ir::load(ast, terms, forms, base_dir).map_err(|e| e.to_string())?;

    if mode == "no-proof" {
        let solved = solve_for_trace(ast, terms, &mut kb, false, false)?;
        return Ok(trace_markdown(
            ast,
            terms,
            &kb,
            &solved,
            Flags {
                diagrams: true,
                ..Flags::default()
            },
        ));
    }

    if mode == "answer" {
        let solved = solve_for_trace(ast, terms, &mut kb, true, false)?;
        let mut out: Vec<String> = Vec::new();
        for exhausted in [true, false] {
            out.push(format!("--- answer exhausted={}", py_bool(exhausted)));
            out.push(crate::answer::render_answer(
                ast,
                terms,
                &kb,
                &solved.answer,
                exhausted,
            ));
            out.push(format!("--- table exhausted={}", py_bool(exhausted)));
            out.push(crate::answer::render_solution_table(
                ast,
                terms,
                &kb,
                &solved.answer,
                Some(solved.stats.solution_nodes),
                exhausted,
                Some("<source>"),
            )?);
        }
        // The exhaustive regime's own trace: this is where the unsat and
        // many-solution lattice shapes are, and `trace` never sees one.
        out.push("--- markdown exhaustive".to_string());
        out.push(trace_markdown(
            ast,
            terms,
            &kb,
            &solved,
            Flags {
                diagrams: true,
                ..Flags::default()
            },
        ));
        return Ok(out.join("\n"));
    }

    let solved = solve_for_trace(ast, terms, &mut kb, true, true)?;
    let f = |diagrams, full_kb, relevant, reorder| Flags {
        diagrams,
        full_kb,
        relevant,
        reorder,
    };
    let flags = [
        ("default", f(true, false, false, false)),
        ("no-diagrams", f(false, false, false, false)),
        ("full-kb", f(true, true, false, false)),
        ("reorder", f(false, false, false, true)),
        ("relevant", f(false, false, true, false)),
        ("relevant-reorder", f(false, false, true, true)),
    ];
    let mut out: Vec<String> = Vec::new();
    for (name, flags) in flags {
        out.push(format!("--- markdown {name}"));
        out.push(trace_markdown(ast, terms, &kb, &solved, flags));
    }
    // The round-trip is a *property*, and its witness is a text both
    // implementations can print: dump the steps as IR, parse them back, dump
    // again, and show both. Equal halves mean the round-trip held.
    let steps = crate::trace::linearize(
        ast,
        terms,
        &kb,
        &solved,
        crate::trace::LinearizeOpts {
            diagrams: false,
            full_kb_snapshots: false,
            relevant: false,
        },
    )
    .steps;
    let ir = crate::trace::trace_to_ir(&steps);
    let mut reparse = Ast::new();
    // A parse failure is ein.py's `IRParseError`, propagating out of the op —
    // so it propagates here too rather than degrading to an empty trace.
    let forms = ein_ir::parse(&mut reparse, &ir, None).map_err(|e| e.to_string())?;
    let again = match forms.first() {
        Some(&f) => crate::trace::trace_to_ir(&crate::trace::parse_trace_steps(&reparse, f)),
        None => "(trace)".to_string(),
    };
    out.push("--- ir".to_string());
    out.push(ir.clone());
    out.push("--- ir-reparsed".to_string());
    out.push(again.clone());
    out.push(format!(
        "--- round-trip {}",
        if ir == again { "ok" } else { "DIFFERS" }
    ));
    Ok(out.join("\n"))
}

/// `str(bool)` — `True` / `False`, because the separator lines interpolate one.
fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

// ── The `--dump-states` tree ───────────────────────────────────────

/// A directory as one text: every file, sorted by path, with its bytes.
///
/// There is no way to diff a tree over a line protocol, so the tree is
/// rendered. The rendering is deliberately dumb — it invents nothing — so a
/// missing file, an extra file, a renamed directory and a changed byte all
/// read the same way.
fn render_tree(root: &Path) -> String {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_files(root, &mut files);
    files.sort();
    let mut out: Vec<String> = Vec::new();
    for path in files {
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        out.push(format!("=== {rel}"));
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        if rel.starts_with("enterings/") {
            // **A fork's own dump is narration** — `ein-parity`'s rule, and
            // the one place it is applied where the artefact is *produced*
            // rather than where it is compared.
            //
            // That exception is measured, not assumed: `zebra2-hints` writes
            // **6.6 MiB** of per-entering dumps against 84 KiB for the rest of
            // the tree, so rendering them into a shape that
            // [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)'s
            // normalisation immediately throws away would push hundreds of
            // megabytes through the oracle's JSON-Lines pipe, twice, to
            // compare nothing. `dump_parity` re-checks that this marker is
            // exactly `ein_parity::NARRATED`, so the decision still has one
            // owner. `utils/ir_oracle.py::_render_tree` is the other half.
            //
            // The **file set** is still compared exactly — a missing, renamed
            // or empty per-commitment dump still fails — and everything
            // outside `enterings/` is compared byte for byte: the root
            // snapshot, every layer dump, the timeline and `summary.json`.
            // Not even "empty or not": a resumed fork whose delta triggers
            // nothing writes an *empty* `firings.jsonl` where ein.py's writes
            // its re-derivation of root, so non-emptiness is the divergence
            // too. What replaces the byte check is `utils/fork_delta_verify.py`
            // — every fork's fact set, fact by fact, across 3.2 M enterings —
            // and an ein.rs golden
            // ([S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md)).
            out.push("=== <narrated>".to_string());
            continue;
        }
        out.extend(text.lines().map(normalise_dump_line));
        if !text.is_empty() && !text.ends_with('\n') {
            out.push("=== (no trailing newline)".to_string());
        }
    }
    out.join("\n")
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect_files(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// Blank the clock readings — value, not presence, so a record that lost its
/// `ts_ms` still fails.
///
/// The clocks, and only the clocks: they are on the
/// [normalisation list](../../../../plans/m1a_rust/design/01_parity_contract.md) §5
/// because no two runs can agree on them. The per-entering `firings` count
/// used to be blanked here too and is not any more — it is
/// [D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it),
/// which is a *comparison* decision and now lives with the rest of them in
/// `ein-parity`. A renderer that decides what a diff will look at is a
/// renderer with an opinion about the contract.
///
/// `utils/ir_oracle.py`'s `_normalise_dump_line` does the same on the Python
/// side; the two lists are maintained together.
fn normalise_dump_line(line: &str) -> String {
    let mut out = line.to_string();
    for key in ["ts_ms", "elapsed_seconds"] {
        let needle = format!("\"{key}\": ");
        let mut from = 0;
        while let Some(i) = out[from..].find(&needle).map(|j| from + j) {
            let start = i + needle.len();
            let end = out[start..]
                .find(|c: char| !matches!(c, '0'..='9' | '.' | 'e' | 'E' | '+' | '-'))
                .map_or(out.len(), |j| start + j);
            out.replace_range(start..end, "<ts>");
            from = start + "<ts>".len();
        }
    }
    // The progress view's `(   12s)` elapsed column.
    normalise_elapsed(&out)
}

fn normalise_elapsed(line: &str) -> String {
    // `\(\s*\d+s\)` → `(<el>)`.
    let bytes: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '(' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] == ' ' {
                j += 1;
            }
            let digits = j;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > digits && j + 1 < bytes.len() && bytes[j] == 's' && bytes[j + 1] == ')' {
                out.push_str("(<el>)");
                i = j + 2;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

/// A fresh directory under the system temp dir, removed by the caller.
fn temp_dir(tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!("ein-dump-{}-{tag}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    p
}

/// Render one mode of the state-dump surface.
pub fn dump_shape(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    mode: &str,
) -> Result<String, String> {
    use ein_infer::solve::{OnBudget, SolveError, SolveOptions, solve};

    let mut kb = ein_ir::load(ast, terms, forms, base_dir).map_err(|e| e.to_string())?;
    if mode == "snapshot" {
        return snapshot_shape(ast, terms, &mut kb);
    }

    let tmp = temp_dir(mode);
    let out_dir = tmp.join("states");
    let buffer = ein_infer::events::Buffer::new();
    let abort = mode == "abort";
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(if abort { 3 } else { 60 }),
        on_budget: if abort {
            OnBudget::Raise
        } else {
            OnBudget::Verdict
        },
        store_lattice: matches!(mode, "lattice" | "abort"),
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let mut monotonic;
    let mut lattice;
    let mut progress;
    let dumper: &mut dyn ein_infer::solve::Dumper = match mode {
        "monotonic" => {
            monotonic =
                crate::dump::MonotonicDumper::new(Some(&out_dir)).map_err(|e| e.to_string())?;
            &mut monotonic
        }
        "progress" => {
            // `out_dir` too: `-v` and `--dump-states` compose, and the live
            // view *is* the file dumper plus a stream, so this is the one mode
            // that exercises both at once.
            progress =
                crate::dump::ProgressDumper::new(Some(&out_dir), Box::new(buffer.clone()), 3, "p")
                    .map_err(|e| e.to_string())?;
            &mut progress
        }
        _ => {
            lattice = crate::dump::LatticeDumper::new(Some(&out_dir)).map_err(|e| e.to_string())?;
            &mut lattice
        }
    };
    let aborted = match solve(&mut kb, terms, ast, &mut events, dumper, &opts) {
        Ok(_) => false,
        Err(SolveError::Budget { .. }) if abort => true,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tmp);
            return Err(e.to_string());
        }
    };
    let mut out = vec![
        format!("ABORTED {}", py_bool(aborted)),
        render_tree(&out_dir),
    ];
    if mode == "progress" {
        out.push("=== <stderr>".to_string());
        out.extend(buffer.to_string_lossy().lines().map(normalise_dump_line));
    }
    let _ = std::fs::remove_dir_all(&tmp);
    Ok(out.join("\n"))
}

/// The `LatticeSnapshot` projection, plus the lattice DOT rendered *from a
/// snapshot* rather than from the live proof.
///
/// The two renders are not the same picture and are not meant to be: a
/// snapshot's `solutions` are post-saturation state keys, so its solution view
/// draws whole states where the proof's draws commitments. What matters is
/// that both implementations draw the same one.
fn snapshot_shape(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<String, String> {
    use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};

    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(60),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved =
        solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())?;
    let Some(proof) = solved.proof.as_ref() else {
        return Ok("NO PROOF".to_string());
    };
    let snap = crate::dump::lattice_snapshot(&solved.answer, proof, kb, terms);

    let show = |sets: Vec<&[ein_core::FactId]>| {
        let mut rendered: Vec<(String, String)> = sets
            .into_iter()
            .map(|s| {
                // Already in `repr` order — the snapshot stores it that way,
                // which is the whole point of `repr_sorted`. Re-sorting the
                // *text* here would hide a key that was not.
                let ids: Vec<String> = s
                    .iter()
                    .map(|f| ein_infer::events::sexpr(terms, *f))
                    .collect();
                (
                    // A *canonical* key's repr, not a commitment's — the two
                    // shapes differ on a nested fact, and this is the sort
                    // order ein.py's `sorted(sets, key=repr)` uses here.
                    crate::dump::snapshot::canon_key_repr(terms, s),
                    format!("{{{}}}", ids.join(" ")),
                )
            })
            .collect();
        rendered.sort();
        format!(
            "[{}]",
            rendered
                .into_iter()
                .map(|(_, r)| r)
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let out = vec![
        format!(
            "SNAPSHOT verdict={} nodes={}",
            snap.verdict_kind,
            snap.nodes_by_state_key.len()
        ),
        format!("  root_state_key {}", show(vec![&snap.root_state_key])),
        format!(
            "  solutions      {}",
            show(snap.solutions.iter().map(|s| &s[..]).collect())
        ),
        // Rendered in full, and *compared* as a count: a dead commitment's
        // `state_key` is the fork's state at the firing that killed it —
        // `enable_fail_fast_fork` stops there rather than at a fixpoint — so
        // it is a dying fork's stopping point, which `ein-parity` blanks.
        // The blanking is that crate's job and not this renderer's; with
        // fail-fast off the keys agree, which is what says this is the prefix
        // and not a difference of conflict.
        format!(
            "  deads          {}",
            show(snap.deads.iter().map(|s| &s[..]).collect())
        ),
        format!(
            "  alive_at_end   {}",
            show(snap.alive_at_end.iter().map(|s| &s[..]).collect())
        ),
        // The two lattice DOTs rendered *from the snapshot* key their dead
        // nodes — id and label both — on the dead commitment's `state_key`,
        // which is the field above, and the DAG *merges* dead commitments by
        // that key, so even the node and edge counts move. Rendered in full
        // here and named as a derivation in `ein-parity`'s closed list, which
        // is what elides them. The renderer itself is still byte-compared,
        // through `dot_parity`'s `lattice` and `lattice-full` views, which
        // read a `LatticeProof` and label their dead nodes by *commitment*
        // rather than by state.
        "=== dot solution".to_string(),
        render_lattice(
            terms,
            LatticeSource::Snapshot(&snap),
            LatticeView::Solution,
            "lattice",
        ),
        "=== dot full".to_string(),
        render_lattice(
            terms,
            LatticeSource::Snapshot(&snap),
            LatticeView::Full,
            "lattice",
        ),
    ];
    Ok(out.join("\n"))
}
