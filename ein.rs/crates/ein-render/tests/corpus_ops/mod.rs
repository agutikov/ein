//! The corpus × op sweep, shared by the two tests that walk it.
//!
//! `id_order_invariance` runs every pair once, digests it against the golden
//! and runs it again under a permuted id space; `jobs_invariance` runs the
//! pairs a job count reaches at `--jobs N`. Both need the same answer to "what are the observable
//! surfaces, and how is one produced", and a second copy of that list would be
//! a second opinion about what the engine renders.
//!
//! Every test binary that declares the module compiles the whole of it, so an
//! item only one of them calls reads as dead to the other — the standard cost
//! of a shared `tests/` module, and the reason for the blanket allow.
#![allow(dead_code)]

use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::Ast;
use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_render::shape::{PARSE_VIEWS, all_views, dot_shape, kb_dot_shape};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

mod searches;
pub use searches::{SWEEP_BUDGET, SWEEP_LIMIT, searches};

/// One fact of a state, as the `states` op digests it: the one printer's
/// text ([`Terms::fact_text`]) and its first recorded origin, `; source
/// "…"`, `; rule r` or `; hypothesis N`, nothing for a fact recorded with
/// none. The digest's own form: no surface of the binary prints a state's
/// facts with their origins.
fn state_fact(kb: &ein_core::Kb, terms: &Terms, f: ein_core::FactId) -> String {
    use ein_core::ProvKind;
    let text = terms.fact_text(f);
    let origin = kb
        .primary(f)
        .map(|p| terms.provs.get(p))
        .and_then(|p| match p.kind {
            ProvKind::Source => p
                .source
                .map(|s| terms.sym(s))
                .filter(|s| !s.is_empty())
                .map(|s| format!("source {}", ein_core::text::quote(s))),
            ProvKind::Rule => p
                .rule
                .map(|r| terms.sym(r))
                .filter(|r| !r.is_empty())
                .map(|r| format!("rule {}", ein_core::text::symbol(r))),
            ProvKind::Hypothesis => Some(format!("hypothesis {}", p.branch.unwrap_or(0))),
            ProvKind::Rejected => None,
        });
    match origin {
        Some(o) => format!("{text} ; {o}\n"),
        None => format!("{text}\n"),
    }
}

/// Every observable surface the engine has. One op is one `(surface, mode)`
/// pair.
///
/// The list is every rendering built to be *comparable*: the frontend dumps,
/// the loaded KB's shape, root saturation's verbose event stream, the plan,
/// the match, the NAF map, the explain shapes, every DOT view, and what
/// `ein solve` prints:
/// the read-out under each question, the region trace, and `ein render
/// lattice`'s drawing of the nodes a description visits; every core a run
/// reports, under each question ([`core_ops`]),
/// and `ein oracle --dump`'s cube, which hold the dead nodes' cores no line
/// of the read-out prints. H has no op of its own: the read-out's `universe`
/// rows and every answer hold it. An id leak that reaches an output reaches
/// one of these, and one that reaches nothing here reaches nothing a user can
/// see.
pub fn ops() -> Vec<Op> {
    let mut out = vec![
        Op::Load,
        Op::Saturate,
        Op::Plan,
        Op::Match,
        Op::Naf,
        Op::Explain(true),
        Op::Explain(false),
    ];
    out.extend(IR_MODES.iter().map(|m| Op::Ir(m)));
    out.extend(all_views().into_iter().map(Op::Dot));
    out.extend(READ_OUT_MODES.iter().map(|(m, _)| Op::ReadOut(m)));
    out.push(Op::RegionTrace);
    out.push(Op::States);
    out.push(Op::RegionLattice);
    out.extend(core_ops());
    out.push(Op::Oracle);
    out
}

/// Every core a run reports, under each of [`READ_OUT_MODES`]'s questions.
/// The read-out prints root's core and four minimal holes';
/// what it does not print, every member of `N` and every dead node a route
/// visited, a run writes into `--dump-answer`'s `nogoods.json` and
/// `--dump-lattice`'s `nodes.json`, each with the rules that refuted it, and
/// these ops hold them: a dead node's `refuted_by` and its core are read off
/// the order its fork's derivations arrive in, which is where an id order
/// would show (`fork-delta-order`,
/// `examples/ein-bugs/fork-delta-order.ein`).
pub fn core_ops() -> Vec<Op> {
    READ_OUT_MODES.iter().map(|(m, _)| Op::Cores(m)).collect()
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

/// The frontend surfaces, which have no CLI of their own and are therefore
/// invisible to a sweep over `ein` processes: `parse`'s accept/reject *and
/// message*, the canonical and compact dumpers, and the two
/// import-resolution ops. There is no tree-shake op: an import brings its
/// module whole, and a `--dump-answer` state file writes the program as
/// `resolve` reads it.
pub const IR_MODES: [&str; 4] = ["parse", "dump-compact", "resolve", "expand"];

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
    Naf,
    Explain(bool),
    Dot(&'static str),
    /// `ein solve`'s stdout, stderr lines and exit code under one of
    /// [`READ_OUT_MODES`].
    ReadOut(&'static str),
    /// `ein solve --trace`'s markdown, under describe.
    RegionTrace,
    /// `ein solve --dump-answer`'s state files, under describe, each fact with
    /// its provenance and the fact in English. The rendering reads a fact's
    /// first recorded justification, and so is held to the same invariance
    /// as the rest of the answer.
    States,
    /// `ein render lattice`'s DOT.
    RegionLattice,
    /// Under one of [`READ_OUT_MODES`]: the summary's `region` block
    /// (`region.core`, every hole's), `--dump-answer`'s `nogoods.json` (each
    /// member of `N`'s core) and `--dump-lattice`'s `nodes.json` (each visited
    /// node's), each core beside the rules that refuted it — [`core_ops`].
    Cores(&'static str),
    /// `ein oracle FILE --dump DIR`'s `region.json` and `region.md`, at the
    /// sweep's cube limit: every node of the cube, each dead one's core.
    Oracle,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Ir(m) => write!(f, "ir[{m}]"),
            Op::Load => write!(f, "load"),
            Op::Saturate => write!(f, "saturate"),
            Op::Plan => write!(f, "plan"),
            Op::Match => write!(f, "match"),
            Op::Naf => write!(f, "naf"),
            Op::Explain(a) => write!(f, "explain{}", if *a { "" } else { "-noalts" }),
            Op::Dot(v) => write!(f, "dot[{v}]"),
            Op::ReadOut(m) => write!(f, "read-out[{m}]"),
            Op::RegionTrace => write!(f, "region-trace"),
            Op::States => write!(f, "states"),
            Op::RegionLattice => write!(f, "region-lattice"),
            Op::Cores(m) => write!(f, "cores[{m}]"),
            Op::Oracle => write!(f, "oracle"),
        }
    }
}

impl Op {
    /// **Whether the op takes a job count**: the ones [`run_with`] hands to a
    /// region route, which fans a step's nodes out — the five read-outs, the
    /// region trace, the states, the lattice, the five core sets and the
    /// oracle. `run_with` dispatches on this, so
    /// an op outside it never sees the count, and `jobs_invariance` runs
    /// these alone at `--jobs N`: every other op, the DOT views among them,
    /// runs no search, and is one computation at any job count.
    pub fn takes_jobs(self) -> bool {
        matches!(
            self,
            Op::ReadOut(_)
                | Op::RegionTrace
                | Op::States
                | Op::RegionLattice
                | Op::Cores(_)
                | Op::Oracle
        )
    }

    /// The op's rendering, cut down to what is compared: narration out.
    ///
    /// The cut is `ein-narration`'s, so this sweep and every other caller
    /// cannot form different opinions about what a derivation is. `None` is a
    /// **rendered derivation compared for presence only** — how a `slice`
    /// view is treated.
    ///
    /// Under `EIN_NARRATION_STRICT=1` nothing is cut and every movement is
    /// reported, by op: the measurement of what the relaxation covers. Under a
    /// permuted interner nothing moves, since a resumed fork's delta is in the
    /// order its facts were written and not in id order: `id_order_invariance`
    /// passes strict, and asserts that nothing moves either way.
    pub fn narrow(self, text: &str) -> Option<String> {
        if ein_narration::strict() {
            return Some(text.to_string());
        }
        Some(match self {
            Op::Dot(view) if ein_narration::is_narration(view) => return None,
            // The region trace is a derivation told step by step, which two
            // runs may narrate differently, so it is compared for presence
            // only, as a rendered derivation is (`ein_narration::is_narration`).
            // The golden digests it whole, in one id space.
            Op::RegionTrace => return None,
            // The three ops the relaxation never touches: the frontend modes,
            // the loaded KB's shape, and root saturation's event stream. None
            // of them crosses a fork, so none of them carries narration, and
            // blanking a firing count here would only weaken the golden.
            Op::Ir(_) | Op::Load | Op::Saturate | Op::States => text.to_string(),
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

/// The same, at a chosen job count — the axis of
/// [03-design/08-parallelism.md § 1](../../../../../docs/ein/ein.rs/03-design/08-parallelism.md#1-the-contract).
///
/// It is a parameter and not a global because `jobs_invariance` runs the same
/// op at two job counts inside one process. **Only an op [`Op::takes_jobs`]
/// receives it**: [`run_loaded`] dispatches on that
/// predicate, and every other op is rendered by code that has no job count to
/// read, so the set the count reaches is read off this code and not written
/// down beside it.
///
/// This parses and loads the file into `terms`, every time: what a permuted
/// id space needs, whose point is the load into it. A sweep that runs many
/// ops over one program in the ordinary id space loads it once instead,
/// through [`Loads`].
pub fn run_with(terms: &mut Terms, path: &Path, op: Op, jobs: usize) -> Option<String> {
    if !op.loads() {
        return run_unloaded(path, op);
    }
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let kb = load_text(&mut ast, terms, &text, path)?;
    run_loaded(&ast, terms, kb, path, op, jobs)
}

/// A program's text loaded as every command loads it, `ein_ir::saved`'s
/// one read, so the KB carries the text a state file saves and a saved
/// file is the node it names: `None` where it does not parse, and the load's
/// refusal where it does not load.
fn load_text(
    ast: &mut Ast,
    terms: &mut Terms,
    text: &str,
    path: &Path,
) -> Option<Result<ein_core::Kb, ein_ir::KbLoadError>> {
    let opts = ein_ir::LoadOptions::default();
    match ein_ir::saved::load_text(ast, terms, text, path.to_str(), path.parent(), 0, &opts) {
        Ok(loaded) => Some(Ok(loaded.kb)),
        Err(ein_ir::saved::TextError::Parse(_)) => None,
        Err(ein_ir::saved::TextError::Load(e)) => Some(Err(e)),
    }
}

impl Op {
    /// **Whether the op reads the loaded program**: every op but the frontend
    /// modes and the DOT views drawn off the parse. These are what [`Loads`]
    /// runs on a copy of one load; the others never
    /// load, and touch no `Terms`.
    pub fn loads(self) -> bool {
        match self {
            Op::Ir(_) => false,
            Op::Dot(view) => !PARSE_VIEWS.contains(&view),
            _ => true,
        }
    }
}

/// The ops that do not load: the frontend modes and the DOT views drawn off
/// the parse, each over a parse of its own.
pub fn run_unloaded(path: &Path, op: Op) -> Option<String> {
    debug_assert!(!op.loads(), "{op} loads: run_loaded's");
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let base = path.parent();
    // A file that does not parse still has an observable — the message — and
    // it is what `examples/broken/*.ein` exist to pin. Every other op needs an
    // AST, so for those a parse failure is genuinely "nothing here". A saved
    // file's forms are the program it carries, as the binary draws them.
    let forms = match ein_ir::saved::parse_text(&mut ast, &text, path.to_str()) {
        Ok(forms) => forms,
        Err(e) => {
            return matches!(op, Op::Ir("parse")).then(|| format!("<refused> {e}"));
        }
    };
    let outcome = match op {
        Op::Ir(mode) => return Some(ir_op(&mut ast, &forms, base, mode)),
        Op::Dot(view) => dot_shape(&mut ast, &mut Terms::new(), &forms, base, view),
        _ => unreachable!("{op} loads: run_loaded's"),
    };
    Some(match outcome {
        Ok(text) => text,
        Err(msg) => format!("<refused> {msg}"),
    })
}

/// **Every op that reads the loaded program**, over a load already made: the
/// AST and `Terms` the load left, and its KB or its refusal. The region ops
/// take the job count, through [`region_op`]; nothing else can read it.
pub fn run_loaded(
    ast: &Ast,
    terms: &mut Terms,
    kb: Result<ein_core::Kb, ein_ir::KbLoadError>,
    path: &Path,
    op: Op,
    jobs: usize,
) -> Option<String> {
    debug_assert!(op.loads(), "{op} does not load: run_unloaded's");
    // `Load` is the one op whose *failure* is its answer: the load-negative
    // corpus is a third of the files, and their messages are the answer. The
    // KB views say why they have no KB in the loader's words too. Every other
    // op below treats a load failure as "nothing here".
    let outcome = match op {
        Op::Load => {
            return Some(match kb {
                Ok(kb) => ein_core::shape(&kb, terms),
                Err(e) => format!("<refused> {}", e.0),
            });
        }
        Op::Dot(view) => match kb {
            Ok(mut kb) => kb_dot_shape(ast, terms, &mut kb, view),
            Err(e) => Err(e.to_string()),
        },
        _ => {
            let mut kb = kb.ok()?;
            if op.takes_jobs() {
                region_op(ast, terms, &mut kb, path, op, jobs)
            } else {
                match op {
                    Op::Plan => ein_infer::plan_shape(ast, terms, &kb).map_err(|e| e.to_string()),
                    Op::Match => ein_infer::match_shape(ast, terms, &kb).map_err(|e| e.to_string()),
                    Op::Naf => ein_infer::naf_map(ast, terms, &mut kb).map_err(|e| e.to_string()),
                    Op::Saturate => {
                        ein_infer::saturate_events(ast, terms, &mut kb).map_err(|e| e.to_string())
                    }
                    Op::Explain(alts) => ein_infer::explain_shape(ast, terms, &mut kb, alts)
                        .map_err(|e| e.to_string()),
                    Op::Ir(_)
                    | Op::Load
                    | Op::Dot(_)
                    | Op::ReadOut(_)
                    | Op::RegionTrace
                    | Op::States
                    | Op::RegionLattice
                    | Op::Cores(_)
                    | Op::Oracle => unreachable!("handled above"),
                }
            }
        }
    };
    Some(match outcome {
        Ok(text) => text,
        Err(msg) => format!("<refused> {msg}"),
    })
}

/// One corpus program as one load left it: what [`Loads`] keeps, and copies
/// for each op. `None` where the file could not be read or did not parse: no
/// op that loads has anything there.
type Loaded = Option<Parsed>;

/// The AST and `Terms` after the load, and the load's KB or refusal.
struct Parsed {
    ast: Ast,
    terms: Terms,
    kb: Result<ein_core::Kb, ein_ir::KbLoadError>,
}

fn load_once(path: &Path) -> Loaded {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_text(&mut ast, &mut terms, &text, path)?;
    Some(Parsed { ast, terms, kb })
}

/// One program's place in [`Loads`]: its load while a unit still needs it,
/// and whether it searches, once asked.
struct Slot {
    /// Units of this program not yet finished.
    left: AtomicUsize,
    loaded: Mutex<Option<Arc<Loaded>>>,
    searches: OnceLock<bool>,
}

/// **Each program loaded once, for every op a sweep runs over it**: the
/// first unit of a program
/// that needs the load makes it, every op runs on a copy of it (`Ast`,
/// `Terms` and `Kb` are `Clone` for this), and the last unit of the program
/// drops it, so the sweep holds the loads of the programs in flight and no
/// more. A copy reads as the load it was taken from, ids and all, so every op
/// renders what it renders over a load of its own; the golden is the proof,
/// the same either way, and `id_order_invariance`'s permuted runs, which
/// load afresh into the permuted id space, compare against a run made this
/// way.
///
/// Import resolution, which reads and parses every stdlib module a program
/// imports, is most of what an op that loads costs, and a program's ops
/// share one.
pub struct Loads {
    files: Vec<PathBuf>,
    slots: Vec<Slot>,
}

impl Loads {
    /// The sweep's programs, and how many units of each it runs: a unit takes
    /// its program with [`Loads::unit`] once, and the program's load is
    /// dropped when the last one is done.
    pub fn new(files: &[PathBuf], units_of: &[usize]) -> Loads {
        assert_eq!(files.len(), units_of.len(), "one count a program");
        Loads {
            files: files.to_vec(),
            slots: units_of
                .iter()
                .map(|&n| Slot {
                    left: AtomicUsize::new(n),
                    loaded: Mutex::new(None),
                    searches: OnceLock::new(),
                })
                .collect(),
        }
    }

    /// The program `f`, for one unit: dropping it is the unit's end.
    pub fn unit(&self, f: usize) -> Unit<'_> {
        Unit { loads: self, f }
    }
}

/// One unit's hold on its program in [`Loads`].
pub struct Unit<'a> {
    loads: &'a Loads,
    f: usize,
}

impl Unit<'_> {
    fn slot(&self) -> &Slot {
        &self.loads.slots[self.f]
    }

    pub fn path(&self) -> &Path {
        &self.loads.files[self.f]
    }

    /// The program's load: made by the first unit that asks, and made again
    /// only if a unit asks after the last one was done.
    fn loaded(&self) -> Arc<Loaded> {
        let mut held = self.slot().loaded.lock().unwrap_or_else(|e| e.into_inner());
        held.get_or_insert_with(|| Arc::new(load_once(self.path())))
            .clone()
    }

    /// **[`run_with`] at `jobs`, on a copy of the program's one load**, in
    /// the ordinary id space: the rendering, and the `Terms` the op left,
    /// which a permutation is built from. An op that does not load runs as
    /// [`run_with`] runs it, on a `Terms` of its own.
    pub fn run(&self, op: Op, jobs: usize) -> (Option<String>, Terms) {
        let mut fresh = Terms::new();
        if !op.loads() {
            let out = run_with(&mut fresh, self.path(), op, jobs);
            return (out, fresh);
        }
        let loaded = self.loaded();
        let Some(Parsed { ast, terms, kb }) = &*loaded else {
            return (None, fresh);
        };
        let (ast, mut terms, kb) = (ast.clone(), terms.clone(), kb.clone());
        drop(loaded);
        let out = run_loaded(&ast, &mut terms, kb, self.path(), op, jobs);
        (out, terms)
    }

    /// **Whether the program searches**: its describe, at the sweep's budget
    /// and cube limit, saturates more than root ([`searches`]). Asked once a
    /// program.
    pub fn searches(&self) -> bool {
        *self.slot().searches.get_or_init(|| {
            let loaded = self.loaded();
            let Some(Parsed {
                ast,
                terms,
                kb: Ok(kb),
            }) = &*loaded
            else {
                return false;
            };
            searches(ast, &mut terms.clone(), &mut kb.clone())
        })
    }
}

impl Drop for Unit<'_> {
    fn drop(&mut self) {
        if self.slot().left.fetch_sub(1, Ordering::AcqRel) == 1 {
            *self.slot().loaded.lock().unwrap_or_else(|e| e.into_inner()) = None;
        }
    }
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
                core_status: v.core_status,
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
        Op::ReadOut(m) | Op::Cores(m) => {
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
    // Every core a surface below prints: `--dump-lattice`'s visited nodes',
    // `--dump-answer`'s and `--trace`'s members of N.
    opts.cores = ein_infer::region::Cores::ALL;
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
                    fold: false,
                },
            )
        }
        Op::States => {
            // The state files, from the call the binary makes: each is the
            // program as read and the commitment, of which the commitment is
            // compared; and the state it loads to, each fact with its origin
            // ([`state_fact`]), read through the node function every route
            // uses.
            let r = ein_infer::read_out::run(kb, terms, ast, &opts).map_err(|e| e.to_string())?;
            let scratch = ein_corpus::Scratch::new("ein-corpus-ops-states");
            let inputs = ein_render::dump::region_answer::Inputs {
                source: &file,
                summary: "",
                answer_txt: "",
                forms: &[],
            };
            ein_render::dump::region_answer::write(scratch.path(), &r, kb, &inputs)?;
            let candidates = r.answer.candidates.clone();
            let mut nodes = ein_infer::region::Nodes::new(kb, terms, ast, &candidates)
                .map_err(|e| e.to_string())?;
            let width = r.states.len().max(1).to_string().len().max(2);
            let mut out = String::new();
            for (i, s) in r.states.iter().enumerate() {
                let name = format!("{:0width$}.ein", i + 1);
                let text = std::fs::read_to_string(scratch.path().join("models").join(&name))
                    .map_err(|e| e.to_string())?;
                let commit = text.rsplit_once("\n(commit\n").map_or("", |(_, c)| c);
                let facts: Vec<_> = s.rep.iter().map(|&b| candidates[b as usize].fact).collect();
                let state = nodes
                    .with_fixpoint(&facts, |kb, terms| {
                        let relation = terms.kernel.relation;
                        let mut state: Vec<_> = kb
                            .facts()
                            .filter(|&f| terms.facts.get(f).0 != relation)
                            .collect();
                        state.sort_by_cached_key(|&f| ein_infer::events::sexpr(terms, f));
                        state
                            .iter()
                            .map(|&f| state_fact(kb, terms, f))
                            .collect::<String>()
                    })
                    .map_err(|e| e.to_string())?;
                out.push_str(&format!("--- {name}\n(commit\n{commit}{state}"));
            }
            Ok(out)
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
                    route: route_words(&a),
                    candidates: &candidates,
                },
                false,
            ))
        }
        Op::Cores(m) => {
            // What `--json-summary`, `--dump-answer` and `--dump-lattice`
            // write, from the same calls the binary makes.
            use ein_render::dump::json::{Json, dumps_indent};
            let visits = Arc::new(Mutex::new(Default::default()));
            let mut events =
                ein_infer::events::Events::off().watched(Box::new(Keep(visits.clone())));
            let r = ein_infer::read_out::run_with(kb, terms, ast, &opts, &mut events)
                .map_err(|e| e.to_string())?;
            let region = ein_render::dump::region_answer::summary_blocks(&r)
                .into_iter()
                .find(|(k, _)| *k == "region")
                .map_or(Json::Null, |(_, v)| v);
            let nogoods = ein_render::dump::region_answer::nogoods(&r)?;
            let v = visits.lock().expect("done").clone();
            let candidates: Vec<String> =
                r.answer.candidates.iter().map(|c| c.text.clone()).collect();
            let a = argv(m);
            let nodes = ein_render::dump::region_lattice::json(
                &v,
                &ein_render::dump::region_lattice::Inputs {
                    source: &file,
                    argv: &a,
                    route: route_words(&r.answer),
                    candidates: &candidates,
                },
            );
            Ok(format!(
                "--- region\n{}\n--- nogoods.json\n{}\n--- nodes.json\n{}\n",
                dumps_indent(&region),
                dumps_indent(&nogoods),
                dumps_indent(&nodes)
            ))
        }
        Op::Oracle => {
            let o = ein_infer::region::Options {
                limit: SWEEP_LIMIT,
                keep_states: false,
                jobs,
                deadline: None,
                cores: ein_infer::region::Cores::ALL,
            };
            let outcome = ein_infer::region::compute(kb, terms, ast, &o, &mut |_| {})
                .map_err(|e| e.to_string())?;
            let inputs = ein_render::dump::RegionInputs {
                source: &file,
                limit: SWEEP_LIMIT,
            };
            // `## Cost`'s line is wall-clock, the one line the page says
            // differs between two runs: kept as a placeholder, so a page that
            // lost it still differs.
            let md: Vec<String> = ein_render::dump::region::region_md(&outcome, terms, &inputs)
                .lines()
                .map(|l| {
                    if l.ends_with("differ between two runs.") {
                        "<the cost line: wall-clock>".to_string()
                    } else {
                        l.to_string()
                    }
                })
                .collect();
            Ok(format!(
                "--- region.json\n{}\n--- region.md\n{}\n",
                ein_render::dump::json::dumps_indent(&ein_render::dump::region::region_json(
                    &outcome, terms, &inputs
                )),
                md.join("\n")
            ))
        }
        _ => unreachable!("a region op"),
    }
}

/// **The answer as documents**, under describe: every block of the summary
/// the read-out writes (`--json-summary`'s `asked`, `region`, `verdict`,
/// `queries`, `claim` and `work`), `--dump-answer`'s `nogoods.json` and
/// `--dump-lattice`'s `nodes.json`, from one run. No layout: what a sweep
/// compares across two spellings of one program, whose names differ in
/// length, reads this rather than the read-out's text.
pub fn answer(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut ein_core::Kb,
    path: &Path,
) -> Result<String, String> {
    use ein_render::dump::json::{Json, dumps_indent};
    let file = path.to_string_lossy().to_string();
    let mut opts = explore::Options::new(READ_OUT_MODES[0].1);
    opts.budget = SWEEP_BUDGET;
    opts.limit = SWEEP_LIMIT;
    opts.cores = ein_infer::region::Cores::ALL;
    let visits = Arc::new(Mutex::new(Default::default()));
    let mut events = ein_infer::events::Events::off().watched(Box::new(Keep(visits.clone())));
    let r = ein_infer::read_out::run_with(kb, terms, ast, &opts, &mut events)
        .map_err(|e| e.to_string())?;
    let blocks = Json::obj(ein_render::dump::region_answer::summary_blocks(&r));
    let nogoods = ein_render::dump::region_answer::nogoods(&r)?;
    let v = visits.lock().expect("done").clone();
    let candidates: Vec<String> = r.answer.candidates.iter().map(|c| c.text.clone()).collect();
    let nodes = ein_render::dump::region_lattice::json(
        &v,
        &ein_render::dump::region_lattice::Inputs {
            source: &file,
            argv: &file,
            route: route_words(&r.answer),
            candidates: &candidates,
        },
    );
    Ok(format!(
        "--- summary\n{}\n--- nogoods.json\n{}\n--- nodes.json\n{}\n",
        dumps_indent(&blocks),
        dumps_indent(&nogoods),
        dumps_indent(&nodes)
    ))
}

/// The route's words, as the binary's `--dump-lattice` and `ein render
/// lattice` print them (`ein-cli`'s `modes::route_words`).
fn route_words(answer: &explore::Answer) -> &'static str {
    let describe =
        answer.mode.which == Which::All && matches!(answer.mode.ask, Ask::Describe | Ask::Count);
    match answer.route {
        explore::Route::DownSet => "1, the alive down-set",
        explore::Route::Cube => "2, the cube",
        explore::Route::States if describe => "the state route, every alive state once",
        explore::Route::States => "the state route, each state at most once",
        explore::Route::Tree => "the obligation tree, over states",
        explore::Route::Root => "none, root alone (--no-search)",
        explore::Route::TreeClasses => "the tree and the classes",
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
        "expand" => r
            .resolve_imports(ast, forms, base)
            .map_err(|e| e.0)
            .and_then(|f| {
                // One macro reading, the loader's: a program the loader
                // refuses is refused here too, with the same sentences, and
                // never rendered as the expansion of whichever duplicate
                // came first.
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

/// **The pair's name in the golden**, `corpus_shapes.md5`,
/// `<repo-relative path>::<op>`.
pub fn pair_name(rel: &Path, op: Op) -> String {
    format!("{}::{op}", rel.display())
}

/// Sixteen hex digits of `md5`, the same digest and the same truncation
/// `render::hashed_id` uses for a DOT node id. Not a security claim: the
/// question is whether a rendering changed, and 64 bits answers it.
///
/// One definition for the sweeps: `id_order_invariance` banks
/// it, and `jobs_invariance` holds a `--jobs N` run to what it banked.
pub fn digest(text: &str) -> String {
    use md5::Digest;
    let mut h = md5::Md5::new();
    h.update(text.as_bytes());
    format!("{:x}", h.finalize())[..16].to_string()
}

/// One manifest line. The line count rides along because it is the cheapest
/// thing that turns "it moved" into "it grew by 40 lines", which is often the
/// whole diagnosis.
pub fn manifest_line(name: &str, text: &str) -> String {
    format!("{}  {:>7}L  {name}", digest(text), text.lines().count())
}

/// The checkout directory, out of the rendering.
///
/// Two of the sweep's surfaces quote a *resolved* path back at the reader: a
/// `ParseError` names the file it failed in, and `import cycle: …` names the
/// chain, which `Resolver::locate` canonicalises so that two spellings of one
/// file are one node in the cycle graph. Digested raw, both would bank the
/// blessing machine's checkout into a checked-in golden — green where it was
/// blessed and red on every other checkout.
///
/// `corpus_cli` gets the same normalisation for free by running the binary
/// with the repo as its working directory and naming its cells relatively;
/// this sweep calls the crates in-process with the absolute path
/// `corpus_files` hands it, so it does the substitution itself. Only the
/// root goes: a rendering that starts naming a *different* file still moves,
/// because everything below `<repo>` is still in the digest. The golden is
/// of this text, so a sweep that holds a run to the golden digests the same.
pub fn portable(root: &str, text: &str) -> String {
    text.replace(root, "<repo>")
}

/// **The golden as `name → digest`**: what a `--jobs 1` run
/// of each pair rendered when it was banked, the bytes [`digest`] reads,
/// narration and all, the checkout made [`portable`].
///
/// A sweep that finds its run's digest here has the `--jobs 1` rendering's
/// digest without computing it. The premise is that the golden is today's,
/// and `id_order_invariance` is what fails when it is not;
/// a pair whose digest differs, or that the golden lacks, computes the run.
/// Empty if the file is missing.
pub fn golden_digests() -> std::collections::BTreeMap<String, String> {
    let path = ein_corpus::golden_path("ein-render", "corpus_shapes.md5");
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let (left, name) = l.split_once("L  ")?;
            let digest = left.split_whitespace().next()?;
            Some((name.to_string(), digest.to_string()))
        })
        .collect()
}
