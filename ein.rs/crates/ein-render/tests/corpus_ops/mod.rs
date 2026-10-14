//! The corpus × op sweep, shared by the two tests that walk it.
//!
//! `id_order_invariance` runs every pair twice under different id spaces;
//! `corpus_shapes` runs every pair once and digests it. Both need the same
//! answer to "what are the observable surfaces, and how is one produced", and
//! a second copy of that list would be a second opinion about what the engine
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

/// Every observable surface the engine has. One op is one `(surface, mode)`
/// pair.
///
/// The list is every rendering built to be *comparable*: the frontend dumps,
/// the loaded KB's shape, root saturation's verbose event stream, the plan,
/// the match, hypothesis generation, the lattice, the NAF map, the solve,
/// commit and explain shapes, every DOT view, the trace and the
/// `--dump-states` tree. An id leak that reaches an output reaches one of
/// them, and one that reaches nothing here reaches nothing a user can see.
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
/// invisible to a sweep over `ein` processes: `parse`'s accept/reject *and
/// message*, the canonical and compact dumpers, and the three
/// import-resolution ops.
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
    /// The op's rendering, cut down to what is compared: narration out.
    ///
    /// The cut is `ein-narration`'s, so this sweep and every other caller
    /// cannot form different opinions about what a derivation is. `None` is a
    /// **rendered derivation compared for presence only** — how a `slice`
    /// view is treated.
    ///
    /// Under `EIN_NARRATION_STRICT=1` nothing is cut and **all 66** movements
    /// are reported, by op: it is not a configuration the suite passes, it is
    /// the measurement of what the relaxation covers.
    pub fn narrow(self, text: &str) -> Option<String> {
        if ein_narration::strict() {
            return Some(text.to_string());
        }
        Some(match self {
            Op::Dot(view) if ein_narration::is_narration(view) => return None,
            // The three ops the relaxation never touches: the frontend modes,
            // the loaded KB's shape, and root saturation's event stream. None
            // of them crosses a fork, so none of them carries narration, and
            // blanking a firing count here would only weaken the golden.
            Op::Ir(_) | Op::Load | Op::Saturate => text.to_string(),
            Op::Trace(mode) => ein_narration::blank_blocks(&format!("--- {mode}\n{text}"), "--- "),
            Op::Dump(_) => ein_narration::blank_blocks(text, "=== "),
            _ => ein_narration::blank(text),
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
    run_with(terms, path, op, 1)
}

/// The same, at a chosen job count — [S1a.7.5](../../../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)
/// T1a.7.5.3's axis.
///
/// It is a parameter and not a global because `jobs_invariance` runs the same
/// op at two job counts inside one process, and because the ops that *cannot*
/// differ by job count — every frontend one — should be visibly passing the
/// same 1 they always did.
pub fn run_with(terms: &mut Terms, path: &Path, op: Op, jobs: usize) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let base = path.parent();
    // A file that does not parse still has an observable — the message — and
    // it is what `examples/broken/*.ein` exist to pin. Every other op needs an
    // AST, so for those a parse failure is genuinely "nothing here".
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
    // corpus is a third of the files, and their messages are the answer.
    // Every other op below treats a load failure as "nothing here".
    if let Op::Load = op {
        return Some(match ein_ir::load(&mut ast, terms, &forms, base) {
            Ok(kb) => ein_core::shape(&kb, terms),
            Err(e) => format!("<refused> {}", e.0),
        });
    }
    let outcome = match op {
        Op::Dot(view) => dot_shape(&mut ast, terms, &forms, base, view, jobs),
        Op::Trace(mode) => trace_shape(&mut ast, terms, &forms, base, mode, jobs),
        Op::Dump(mode) => dump_shape(&mut ast, terms, &forms, base, mode, jobs),
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
                Op::Solve(mode) => ein_infer::solve_shape(&ast, terms, &mut kb, mode, jobs),
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
        // `parse` is the canonical dump of what parsed — which is how a
        // *parser* can be compared through a dumper at all.
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
                // One macro reading, the loader's, since M1e S1e.3.1 —
                // `CO-M4`. A program the loader refuses is refused here too,
                // with the same sentences, where the dump used to render the
                // expansion of whichever duplicate came first.
                let macros = collect_macros(ast, &f).map_err(|problems| problems.join("; "))?;
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
