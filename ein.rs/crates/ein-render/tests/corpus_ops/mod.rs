//! The corpus × op sweep, shared by the two tests that walk it.
//!
//! `id_order_invariance` runs every pair twice under different id spaces;
//! `corpus_shapes` runs every pair once and digests it. Both need the same
//! answer to "what are the observable surfaces, and how is one produced", and
//! a second copy of that list would be a second opinion about what the port
//! renders.
//!
//! Every test binary that declares the module compiles the whole of it, so an
//! item only one of them calls reads as dead to the other — the standard cost
//! of a shared `tests/` module, and the reason for the blanket allow.
#![allow(dead_code)]

use ein_core::Terms;
use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, parse};
use ein_render::shape::{DUMP_MODES, TRACE_MODES, all_views, dot_shape, dump_shape, trace_shape};
use std::path::Path;

/// Every observable surface the port has, named the way its parity test names
/// it. One op is one `(surface, mode)` pair.
///
/// The list is the union of what the corpus-wide parity sweeps compare —
/// `parse_parity`, `dump_parity` (both of them), `imports_parity`,
/// `compile_parity`, `match_parity`, `saturate_parity`, `hypgen_parity`,
/// `dot_parity`, `trace_parity` and `ein-render`'s `dump_parity` — because
/// those are the renderings that were built to be *comparable*. An id leak
/// that reaches an output reaches one of them, and one that reaches nothing
/// here reaches nothing a user can see.
///
/// `Load` and `Saturate` were added by
/// [S1a.10.2](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md).
/// They are the two surfaces whose *only* owner was a differential sweep —
/// `load_parity`'s `kb-shape` and `saturate_parity`'s verbose event stream —
/// and the ledger's answer to "and then what asserts it" was this manifest.
/// They were blessed while the two engines still agreed on both, which is the
/// whole reason the blessing had to happen before the removal and not after.
pub fn ops() -> Vec<Op> {
    let mut out = vec![
        Op::Load,
        Op::Saturate,
        Op::Plan,
        Op::Match,
        Op::Hyp(false),
        Op::Hyp(true),
        Op::Lattice,
        Op::Naf,
        Op::Solve("default"),
        Op::Solve("exhaustive"),
        Op::Solve("shuffled"),
        Op::Commit(true),
        Op::Commit(false),
        Op::Explain(true),
        Op::Explain(false),
    ];
    out.extend(IR_MODES.iter().map(|m| Op::Ir(m)));
    out.extend(all_views().into_iter().map(Op::Dot));
    out.extend(TRACE_MODES.iter().map(|m| Op::Trace(m)));
    out.extend(DUMP_MODES.iter().map(|m| Op::Dump(m)));
    out
}

/// The frontend surfaces, which have no CLI of their own and are therefore
/// invisible to a harness that compares two `ein` processes: `parse`'s
/// accept/reject *and message*, the canonical and compact dumpers, and the
/// three resolution ops `imports_parity` sweeps. Named as `ir_oracle.py`
/// named them — the script is gone since S1a.10.4, but the ledger's rows are
/// written against its vocabulary and renaming the ops would orphan them.
pub const IR_MODES: [&str; 5] = ["parse", "dump-compact", "resolve", "minimize", "expand"];

#[derive(Clone, Copy)]
pub enum Op {
    Ir(&'static str),
    /// The KB after load: registries, fact list and the seven indexes.
    Load,
    /// Root saturation's whole `--events` stream at `Level::Verbose` — every
    /// firing including the redundant ones, plus the ABSENT / CLASH / SUMMARY
    /// tail.
    Saturate,
    Plan,
    Match,
    Hyp(bool),
    Lattice,
    Naf,
    Solve(&'static str),
    Commit(bool),
    Explain(bool),
    Dot(&'static str),
    Trace(&'static str),
    Dump(&'static str),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Ir(m) => write!(f, "ir[{m}]"),
            Op::Load => write!(f, "load"),
            Op::Saturate => write!(f, "saturate"),
            Op::Plan => write!(f, "plan"),
            Op::Match => write!(f, "match"),
            Op::Hyp(c) => write!(f, "hyp{}", if *c { "+closed" } else { "" }),
            Op::Lattice => write!(f, "lattice"),
            Op::Naf => write!(f, "naf"),
            Op::Solve(m) => write!(f, "solve[{m}]"),
            Op::Commit(ff) => write!(f, "commit{}", if *ff { "" } else { "-nofailfast" }),
            Op::Explain(a) => write!(f, "explain{}", if *a { "" } else { "-noalts" }),
            Op::Dot(v) => write!(f, "dot[{v}]"),
            Op::Trace(m) => write!(f, "trace[{m}]"),
            Op::Dump(m) => write!(f, "dump[{m}]"),
        }
    }
}

impl Op {
    /// The op's rendering, cut down to what the parity contract compares.
    ///
    /// Exactly the cut `trace_parity`, `dump_parity` and `dot_parity` apply,
    /// reached through the same crate, so this test and the gate cannot form
    /// different opinions about what a derivation is. `None` is a **rendered
    /// derivation compared for presence only** — `dot_parity`'s treatment of a
    /// `slice` view.
    ///
    /// Under `EIN_PARITY_STRICT=1` nothing is cut and **all 66** movements are
    /// reported, by op — which is the same thing that flag does to
    /// `trace_parity` and `dump_parity`: it is not a configuration the suite
    /// passes, it is the measurement of what the relaxation covers
    /// (design/01 §5).
    pub fn narrow(self, text: &str) -> Option<String> {
        if ein_parity::strict() {
            return Some(text.to_string());
        }
        Some(match self {
            Op::Dot(view) if ein_parity::is_narration(view) => return None,
            // The three ops the relaxation never touched, because their
            // parity tests compared bytes: `ir_oracle.py`'s frontend modes,
            // `load_parity`'s `kb-shape`, and `saturate_parity`'s event
            // stream. Blanking a firing count here would weaken a self-golden
            // to be lenient towards an engine that is not there.
            Op::Ir(_) | Op::Load | Op::Saturate => text.to_string(),
            Op::Trace(mode) => ein_parity::blank_blocks(&format!("--- {mode}\n{text}"), "--- "),
            Op::Dump(_) => ein_parity::blank_blocks(text, "=== "),
            _ => ein_parity::blank(text),
        })
    }
}

/// Run one op over one file with a caller-supplied `Terms`, so the caller
/// decides what is already interned when the file arrives.
///
/// `None` is "this file has nothing for this op" — it does not parse, does not
/// load, or the op refuses it. A refusal is not interesting here: what is
/// compared is two runs of the *same* op, so a file that refuses in both runs
/// agrees trivially and a file that refuses in only one is a difference, which
/// is why the error text is returned rather than swallowed.
pub fn run(terms: &mut Terms, path: &Path, op: Op) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let base = path.parent();
    // A file that does not parse still has an observable — the message — and
    // it is the one T3 compared on `examples/broken/*.ein`. Every other op
    // needs an AST, so for those a parse failure is genuinely "nothing here".
    let forms = match parse(&mut ast, &text, path.to_str()) {
        Ok(forms) => forms,
        Err(e) => {
            return matches!(op, Op::Ir("parse")).then(|| format!("<refused> {e}"));
        }
    };
    if let Op::Ir(mode) = op {
        return Some(ir_op(&mut ast, &forms, base, mode));
    }
    // `Load` is the one op whose *failure* is its answer: the load-negative
    // corpus is a third of the files, and their messages are what
    // `load_parity` compared. Every other op below treats a load failure as
    // "nothing here".
    if let Op::Load = op {
        return Some(match ein_ir::load(&mut ast, terms, &forms, base) {
            Ok(kb) => ein_core::shape(&kb, terms),
            Err(e) => format!("<refused> {}", e.0),
        });
    }
    let outcome = match op {
        Op::Dot(view) => dot_shape(&mut ast, terms, &forms, base, view),
        Op::Trace(mode) => trace_shape(&mut ast, terms, &forms, base, mode),
        Op::Dump(mode) => dump_shape(&mut ast, terms, &forms, base, mode),
        _ => {
            let mut kb = ein_ir::load(&mut ast, terms, &forms, base).ok()?;
            match op {
                Op::Plan => ein_infer::plan_shape(&ast, terms, &kb).map_err(|e| e.to_string()),
                Op::Match => ein_infer::match_shape(&ast, terms, &kb).map_err(|e| e.to_string()),
                Op::Hyp(closed) => ein_infer::hyp_shape_with(&ast, terms, &mut kb, closed),
                Op::Lattice => {
                    ein_infer::lattice_shape(&ast, terms, &mut kb).map_err(|e| e.to_string())
                }
                Op::Naf => ein_infer::naf_map(&ast, terms, &mut kb).map_err(|e| e.to_string()),
                Op::Saturate => {
                    ein_infer::saturate_events(&ast, terms, &mut kb).map_err(|e| e.to_string())
                }
                Op::Solve(mode) => ein_infer::solve_shape(&ast, terms, &mut kb, mode),
                Op::Commit(ff) => {
                    ein_infer::commit_shape(&ast, terms, &mut kb, ff).map_err(|e| e.to_string())
                }
                Op::Explain(alts) => {
                    ein_infer::explain_shape(&ast, terms, &mut kb, alts).map_err(|e| e.to_string())
                }
                Op::Ir(_) | Op::Load | Op::Dot(_) | Op::Trace(_) | Op::Dump(_) => {
                    unreachable!("handled above")
                }
            }
        }
    };
    Some(match outcome {
        Ok(text) => text,
        Err(msg) => format!("<refused> {msg}"),
    })
}

/// One frontend op, as text — the rendering on success, `<refused> …` with the
/// message on failure, which is the observable either way.
fn ir_op(ast: &mut Ast, forms: &[ein_ir::NodeId], base: Option<&Path>, mode: &str) -> String {
    let r = Resolver::new();
    let out: Result<String, String> = match mode {
        // `parse` is the canonical dump of what parsed — the same op
        // `ir_oracle.py` answered under that name, and the reason
        // `dump_parity` could compare a *parser* through a dumper at all.
        "parse" => Ok(dump_canonical(ast, forms)),
        "dump-compact" => Ok(forms
            .iter()
            .map(|f| dump_compact(ast, *f))
            .collect::<Vec<_>>()
            .join("\n")),
        "resolve" => r
            .resolve_imports(ast, forms, base)
            .map(|f| dump_canonical(ast, &f))
            .map_err(|e| e.0),
        "minimize" => r
            .resolve_and_minimize(ast, forms, base)
            .map(|f| dump_canonical(ast, &f))
            .map_err(|e| e.0),
        "expand" => r
            .resolve_imports(ast, forms, base)
            .map_err(|e| e.0)
            .and_then(|f| {
                let macros = collect_macros(ast, &f);
                expand_rule_clauses(ast, &f, &macros)
                    .map(|x| dump_canonical(ast, &x))
                    .map_err(|e| e.0)
            }),
        other => unreachable!("unknown ir mode {other}"),
    };
    match out {
        Ok(text) => text,
        Err(msg) => format!("<refused> {msg}"),
    }
}
