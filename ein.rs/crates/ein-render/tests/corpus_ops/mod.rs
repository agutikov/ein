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
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, parse};
use ein_render::shape::{SOLVE_VIEWS, all_views, dot_shape};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Every observable surface the engine has. One op is one `(surface, mode)`
/// pair.
///
/// The list is every rendering built to be *comparable*: the frontend dumps,
/// the loaded KB's shape, root saturation's verbose event stream, the plan,
/// the match, hypothesis generation, the NAF map, the explain shapes, every
/// DOT view, and — since M1e2 S1e2.1a.5's switch — what `ein solve` prints:
/// the read-out under each question, the region trace, and `ein render
/// lattice`'s drawing of the nodes a description visits. The walk's surfaces
/// (its solve table, trace, `--dump-states` tree, lattice proof and commit
/// shape) left with its CLI path. An id leak that reaches an output reaches
/// one of these, and one that reaches nothing here reaches nothing a user can
/// see.
pub fn ops() -> Vec<Op> {
    let mut out = vec![
        Op::Load,
        Op::Saturate,
        Op::Plan,
        Op::Match,
        Op::Hyp(false),
        Op::Hyp(true),
        Op::Naf,
        Op::Explain(true),
        Op::Explain(false),
    ];
    out.extend(IR_MODES.iter().map(|m| Op::Ir(m)));
    out.extend(
        all_views()
            .into_iter()
            .filter(|v| !SOLVE_VIEWS.contains(v))
            .map(Op::Dot),
    );
    out.extend(READ_OUT_MODES.iter().map(|(m, _)| Op::ReadOut(m)));
    out.push(Op::RegionTrace);
    out.push(Op::RegionLattice);
    out
}

/// The questions the sweep asks, as `ein solve`'s flags: describe, each line,
/// `--single` and `-n 2`.
pub const READ_OUT_MODES: [(&str, Mode); 5] = [
    (
        "describe",
        Mode {
            which: Which::All,
            ask: Ask::Describe,
        },
    ),
    (
        "maximal",
        Mode {
            which: Which::Maximal,
            ask: Ask::Describe,
        },
    ),
    (
        "minimal",
        Mode {
            which: Which::Minimal,
            ask: Ask::Describe,
        },
    ),
    (
        "single",
        Mode {
            which: Which::All,
            ask: Ask::Single,
        },
    ),
    (
        "n2",
        Mode {
            which: Which::All,
            ask: Ask::Find(2),
        },
    ),
];

/// The sweep's budget and cube limit: small, so that every file is quick and
/// partial runs, budget- and limit-stopped, are among what is compared.
pub const SWEEP_BUDGET: u64 = 1 << 14;
pub const SWEEP_LIMIT: u32 = 10;

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
    Naf,
    Explain(bool),
    Dot(&'static str),
    /// `ein solve`'s stdout, stderr lines and exit code under one of
    /// [`READ_OUT_MODES`].
    ReadOut(&'static str),
    /// `ein solve --trace`'s markdown, under describe.
    RegionTrace,
    /// `ein render lattice`'s DOT.
    RegionLattice,
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
            Op::Naf => write!(f, "naf"),
            Op::Explain(a) => write!(f, "explain{}", if *a { "" } else { "-noalts" }),
            Op::Dot(v) => write!(f, "dot[{v}]"),
            Op::ReadOut(m) => write!(f, "read-out[{m}]"),
            Op::RegionTrace => write!(f, "region-trace"),
            Op::RegionLattice => write!(f, "region-lattice"),
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
            // The region trace is a derivation told step by step, which two
            // runs may narrate differently — the walk's `markdown` was the same
            // and compared for presence only (`ein_narration::is_narration`).
            // `corpus_shapes` still digests it whole, in one id space.
            Op::RegionTrace => return None,
            // The three ops the relaxation never touches: the frontend modes,
            // the loaded KB's shape, and root saturation's event stream. None
            // of them crosses a fork, so none of them carries narration, and
            // blanking a firing count here would only weaken the golden.
            Op::Ir(_) | Op::Load | Op::Saturate => text.to_string(),
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
        Op::ReadOut(_) | Op::RegionTrace | Op::RegionLattice => {
            let mut kb = ein_ir::load(&mut ast, terms, &forms, base).ok()?;
            region_op(&ast, terms, &mut kb, path, op, jobs)
        }
        _ => {
            let mut kb = ein_ir::load(&mut ast, terms, &forms, base).ok()?;
            match op {
                Op::Plan => ein_infer::plan_shape(&ast, terms, &kb).map_err(|e| e.to_string()),
                Op::Match => ein_infer::match_shape(&ast, terms, &kb).map_err(|e| e.to_string()),
                Op::Hyp(closed) => ein_infer::hyp_shape_with(&ast, terms, &mut kb, closed),
                Op::Naf => ein_infer::naf_map(&ast, terms, &mut kb).map_err(|e| e.to_string()),
                Op::Saturate => {
                    ein_infer::saturate_events(&ast, terms, &mut kb).map_err(|e| e.to_string())
                }
                Op::Explain(alts) => {
                    ein_infer::explain_shape(&ast, terms, &mut kb, alts).map_err(|e| e.to_string())
                }
                Op::Ir(_)
                | Op::Load
                | Op::Dot(_)
                | Op::ReadOut(_)
                | Op::RegionTrace
                | Op::RegionLattice => {
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

/// A watch that keeps what a route visits, for `RegionLattice`.
struct Keep(Arc<Mutex<ein_render::dump::region_lattice::Visits>>);

impl ein_infer::events::Watch for Keep {
    fn node(&mut self, v: &ein_infer::events::Visited<'_>) {
        self.0.lock().expect("one thread").nodes.push(
            ein_render::dump::region_lattice::VisitedNode {
                commitment: v.commitment.to_vec(),
                kind: v.kind,
                owes: v.owes,
                owed: v.owed.to_vec(),
                core: v.core.to_vec(),
                refuted_by: v.refuted_by.to_vec(),
                closure: v.closure.to_vec(),
                facts: v.facts,
            },
        );
    }
}

/// One of `ein solve`'s surfaces, rendered as the binary renders it: the
/// read-out under a question (stdout, then its stderr lines and exit code),
/// the region trace, or `ein render lattice`'s DOT.
fn region_op(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut ein_core::Kb,
    path: &Path,
    op: Op,
    jobs: usize,
) -> Result<String, String> {
    let file = path.to_string_lossy().to_string();
    let mode = match op {
        Op::ReadOut(m) => {
            READ_OUT_MODES
                .iter()
                .find(|(n, _)| *n == m)
                .expect("a mode")
                .1
        }
        _ => READ_OUT_MODES[0].1,
    };
    let mut opts = explore::Options::new(mode);
    opts.budget = SWEEP_BUDGET;
    opts.limit = SWEEP_LIMIT;
    opts.jobs = jobs;
    let argv = |m: &str| format!("{file} --{m}");
    fn options<'a>(argv: &'a str, file: &'a str) -> ein_render::read_out::Options<'a> {
        ein_render::read_out::Options {
            command: "solve",
            argv,
            file,
            budget: SWEEP_BUDGET,
            uncut: false,
        }
    }
    match op {
        Op::ReadOut(m) => {
            let r = ein_infer::read_out::run(kb, terms, ast, &opts).map_err(|e| e.to_string())?;
            let a = argv(m);
            let text = ein_render::read_out::render(&r, &options(&a, &file));
            Ok(format!(
                "{}--- stderr\n{}\n--- exit {}\n",
                text.stdout,
                text.stderr.join("\n"),
                text.code
            ))
        }
        Op::RegionTrace => {
            let mut events = ein_infer::events::Events::off();
            let firings = ein_infer::region::root_firings(kb, terms, ast, &mut events)
                .map_err(|e| e.to_string())?;
            let r = ein_infer::read_out::run_with(kb, terms, ast, &opts, &mut events)
                .map_err(|e| e.to_string())?;
            ein_render::trace::region::of_read_out(
                &r,
                kb,
                terms,
                ast,
                &firings,
                &options(&file, &file),
                ein_render::trace::region::Shape {
                    diagrams: true,
                    relevant: false,
                    reorder: false,
                    full_kb: false,
                },
            )
        }
        Op::RegionLattice => {
            let visits = Arc::new(Mutex::new(Default::default()));
            let mut events =
                ein_infer::events::Events::off().watched(Box::new(Keep(visits.clone())));
            let a =
                explore::run_with(kb, terms, ast, &opts, &mut events).map_err(|e| e.to_string())?;
            let v = visits.lock().expect("done").clone();
            let candidates: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
            Ok(ein_render::dump::region_lattice::dot(
                &v,
                &ein_render::dump::region_lattice::Inputs {
                    source: &file,
                    argv: &file,
                    route: match a.route {
                        explore::Route::DownSet => "1, the alive down-set",
                        explore::Route::Cube => "2, the cube",
                        explore::Route::States => "the state route, every alive state once",
                        explore::Route::Tree => "the obligation tree, over states",
                    },
                    candidates: &candidates,
                },
                false,
            ))
        }
        _ => unreachable!("a region op"),
    }
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
