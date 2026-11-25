//! Shared loaders, and **the one read of a program from a path**.
//!
//! Every command that takes a program reads it here: [`read_program`] reads
//! the bytes and tells a `.einb` container from UTF-8 text by the magic bytes
//! (through `ein_ir::read`, which the import resolver reads a module through
//! too), [`load_held`] loads either (a text against its own directory, so a
//! file-relative `(import …)` resolves; a container opened), and
//! [`parse_held`] parses either without loading (a container's stored
//! program). So no command refuses a `.einb` another answers or a
//! file-relative import another resolves, and a file that is not UTF-8
//! is one line, never a process exit that takes `ein test`'s whole selection
//! down with it.
//!
//! The convention: nothing here exits. A failure comes back as the text the
//! command prints to stderr, and the exit code is the command's to choose —
//! 1 for `solve`, `saturate`, `oracle`, `render` and `kb save`, and for `ein
//! test` an `ERROR` row and 2 (`docs/ein/user/04-reference/07-diagnostics.md` § 2).

use std::path::Path;
use std::time::{Duration, Instant};

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

pub use ein_ir::read::Held;

/// Read a program from `path`: the one read every command makes of one,
/// [`ein_ir::read::read_program`], its failure the line the command prints.
///
/// The read is `ein-ir`'s, so that the import resolver reads an imported
/// module through it too and refuses one as this refuses the program it is
/// given, naming the module's path: a file that cannot be read gives one line naming the
/// error class, the errno text and the quoted path, and a file that is
/// neither a `.einb` container nor UTF-8 gives
/// [`ein_ir::read::not_utf8_line`].
pub fn read_program(path: &Path) -> Result<Held, String> {
    // A hypothesis a state above root holds that root does not makes the
    // program not monotone, found in flight (Q-M1f.574): the warning is said
    // at once, once, on stderr, the static check having found no site.
    let file = path.display().to_string();
    ein_infer::open::set_in_flight_hook(Some(Box::new(move |_| {
        // Once a file, whatever runs of it the command makes.
        static SAID: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
        let mut said = SAID.lock().unwrap_or_else(|e| e.into_inner());
        if !said.contains(&file) {
            said.push(file.clone());
            eprintln!("{}", ein_infer::detector::warning_line(&file));
        }
    })));
    ein_ir::read::read_program(path).map_err(|e| e.line)
}

/// A program loaded from a path, and how it was read.
pub struct Loaded {
    pub kb: Kb,
    /// Its top-level forms in the `Ast`: as written, for a text, or as the
    /// container stores them, resolved and import-flattened.
    pub forms: Vec<NodeId>,
    pub how: How,
}

/// How [`load_held`] read a program, and what each step cost.
pub enum How {
    /// A text, parsed and then loaded against its own directory.
    Text { parse: Duration, load: Duration },
    /// A `.einb` container, opened; a build with no `einb` feature refuses
    /// one, and never builds this.
    #[cfg_attr(not(feature = "einb"), allow(dead_code))]
    Container {
        open: Duration,
        /// What the file says it was made from.
        #[cfg(feature = "einb")]
        meta: Box<ein_einb::Meta>,
        /// The file's derived sections were not believed, and the KB is the
        /// loader's output from the stored program.
        #[cfg(feature = "einb")]
        derived_dropped: bool,
    },
}

/// What a command's argv asks of the load: a `:why` on every declaration, the
/// default, unless `--no-require-why` (Q-M1f.62), on every command that loads
/// a program. A
/// command that does not define it loads as the default does.
pub fn load_options(m: &clap::ArgMatches) -> ein_ir::LoadOptions {
    ein_ir::LoadOptions {
        require_why: m
            .try_get_one::<bool>("no-require-why")
            .ok()
            .flatten()
            .copied()
            != Some(true),
    }
}

/// The refusal of a container by a build with no `einb` feature.
#[cfg(not(feature = "einb"))]
fn no_einb_line(path: &Path) -> String {
    format!(
        "kb load error: {} is a .einb container and this build has no `einb` feature",
        path.display()
    )
}

/// Load what [`read_program`] read, about query number `query` — a program
/// holds every `(query …)` it writes, in both formats, because a container
/// stores its program as text and therefore carries every query the source
/// had.
///
/// A text is parsed under its path and loaded with its own directory as the
/// base, so a file-relative `(import …)` resolves against it on every
/// command. A container is opened: the fork is on the
/// **magic bytes** and not on the extension, so a container renamed
/// `puzzle.ein` is still a container. `ast` must be empty, because a
/// container carries its own program and replaces it.
///
/// A container whose *inputs* have moved is opened anyway and said so on
/// stderr. There is nothing else to do — the caller was handed the `.einb` and
/// not the `.ein`, so "re-parse the source" (`09-binary-format.md` § 4.3) is not available to
/// it — and the one thing that must not happen is believing it silently.
pub fn load_held(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    held: Held,
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Result<Loaded, String> {
    match held {
        Held::Text(text) => {
            // A program, or a saved file, its program read against the
            // modules it carries (`ein_ir::saved`): one read and one load,
            // so the parse is timed with it.
            let t = Instant::now();
            let loaded = ein_ir::saved::load_text(
                ast,
                terms,
                &text,
                path.to_str(),
                path.parent(),
                query,
                load,
            )
            .map_err(|e| e.to_string())?;
            let (parse, loading) = (loaded.parse, t.elapsed().saturating_sub(loaded.parse));
            judge_mechanisms(ast, terms, &loaded.kb, |ast, terms| {
                ein_ir::saved::load_text(
                    ast,
                    terms,
                    &text,
                    path.to_str(),
                    path.parent(),
                    query,
                    load,
                )
                .ok()
                .map(|l| l.kb)
            })?;
            Ok(Loaded {
                kb: loaded.kb,
                forms: loaded.forms,
                how: How::Text {
                    parse,
                    load: loading,
                },
            })
        }
        #[cfg(feature = "einb")]
        Held::Container(bytes) => open_einb(ast, terms, path, &bytes, query, load),
        #[cfg(not(feature = "einb"))]
        Held::Container(_) => {
            let _ = (terms, query, load);
            Err(no_einb_line(path))
        }
    }
}

/// **What the load noticed and did not refuse**: each note's line,
/// `note: <message>` ([`ein_core::Diagnostic::line`]), kept for
/// [`say_notes`], on every command, since every command that loads a program
/// loads it through [`load_held`]: the hypothesis mechanisms' notes
/// (Q-M1f.242). A command may load one file several times, once a query and
/// once a question, and each line carries its file and position, so a line
/// is kept the first time the process meets it and never again. No answer
/// and no exit code moves.
fn keep_notes(notes: &[ein_core::Diagnostic]) {
    let mut w = NOTES.lock().unwrap_or_else(|e| e.into_inner());
    for note in notes {
        let line = note.line();
        if !w.said.contains(&line) {
            w.said.push(line.clone());
            w.pending.push(line);
        }
    }
}

/// The load's note lines: every one met, and those not yet printed.
struct Notes {
    said: Vec<String>,
    pending: Vec<String>,
}

static NOTES: std::sync::Mutex<Notes> = std::sync::Mutex::new(Notes {
    said: Vec::new(),
    pending: Vec::new(),
});

/// **The load's notes, said when a command's run of a program ends**
/// ([`keep_notes`]): at the end, so that a flagged program's `WARNING` line
/// stays stderr's first (Q-M1f.564, Q-M1f.580). No answer and no exit code
/// moves.
pub fn say_notes() {
    let pending = {
        let mut w = NOTES.lock().unwrap_or_else(|e| e.into_inner());
        std::mem::take(&mut w.pending)
    };
    for line in pending {
        eprintln!("{line}");
    }
}

/// **A conflict between the hypothesis mechanisms refuses the program**
/// (Q-M1f.245), on every command, since
/// every command that loads a program loads it here: one line per conflict,
/// each `kb load error: …`, at the command's load-error exit. A list that
/// filters no candidate, or a name another declaration already says, is one
/// `note:` line, as [`keep_notes`] keeps the loader's, and moves nothing.
///
/// The judgement is the library's, [`ein_infer::mechanisms::judge_loaded`]:
/// derived closures and derived obligation activators exist only at root's
/// fixpoint, so it saturates a second load of the program, `again`, and
/// leaves the command's own tables as they were.
fn judge_mechanisms(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    again: impl FnOnce(&mut Ast, &mut Terms) -> Option<Kb>,
) -> Result<(), String> {
    match ein_infer::mechanisms::judge_loaded(ast, terms, kb, again) {
        Ok(notes) => {
            keep_notes(&notes);
            Ok(())
        }
        Err(conflicts) => Err(conflicts
            .iter()
            .map(|d| format!("kb load error: {d}"))
            .collect::<Vec<_>>()
            .join("\n")),
    }
}

/// Parse what [`read_program`] read, without loading it: a text as written,
/// or the program a container stores, parsed under
/// `ein_einb::program_name`'s name. What `ein render rules`, `rule`,
/// `constraints` and `program` read.
pub fn parse_held(ast: &mut Ast, path: &Path, held: Held) -> Result<Vec<NodeId>, String> {
    match held {
        // A saved file's program, as written.
        Held::Text(text) => {
            ein_ir::saved::parse_text(ast, &text, path.to_str()).map_err(|e| e.to_string())
        }
        #[cfg(feature = "einb")]
        Held::Container(bytes) => {
            let text =
                ein_einb::program_of(&bytes, true).map_err(|e| format!("kb load error: {e}"))?;
            let name = ein_einb::program_name(Some(&path.display().to_string()));
            ein_ir::parse(ast, &text, Some(&name))
                .map_err(|e| format!("kb load error: {}", ein_einb::EinbError::Program(e.into())))
        }
        #[cfg(not(feature = "einb"))]
        Held::Container(_) => Err(no_einb_line(path)),
    }
}

/// [`read_program`] and [`load_held`], about query number `query`, with the
/// failure printed to stderr: `None` is the caller's load-error exit.
pub fn load_program_query(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Option<Kb> {
    match read_program(path).and_then(|held| load_held(ast, terms, path, held, query, load)) {
        Ok(loaded) => Some(loaded.kb),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

/// [`load_program_query`], about the first query.
pub fn load_program(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    load: &ein_ir::LoadOptions,
) -> Option<Kb> {
    load_program_query(ast, terms, path, 0, load)
}

/// [`read_program`] and [`parse_held`], with the failure printed to stderr.
pub fn parse_program(ast: &mut Ast, path: &Path) -> Option<Vec<NodeId>> {
    match read_program(path).and_then(|held| parse_held(ast, path, held)) {
        Ok(forms) => Some(forms),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

#[cfg(feature = "einb")]
fn open_einb(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    bytes: &[u8],
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Result<Loaded, String> {
    let t = Instant::now();
    let opts = || ein_einb::OpenOptions {
        // The paths the file names, re-hashed where they still exist. One that
        // does not claims nothing: a container shipped without its source is
        // the normal case, not a stale one.
        sources: ein_einb::meta_of(bytes)
            .map(|m| {
                m.sources
                    .iter()
                    .filter_map(|s| ein_einb::Source::of(Path::new(&s.path)))
                    .collect()
            })
            .unwrap_or_default(),
        query,
        load: *load,
        // Every position in the stored program names the container
        // (Q-M1f.15), not `<string>`.
        name: Some(path.display().to_string()),
        ..ein_einb::OpenOptions::default()
    };
    let opened =
        ein_einb::open_bytes(bytes, terms, &opts()).map_err(|e| format!("kb load error: {e}"))?;
    judge_mechanisms(&opened.ast, terms, &opened.kb, |ast, terms| {
        let again = ein_einb::open_bytes(bytes, terms, &opts()).ok()?;
        *ast = again.ast;
        Some(again.kb)
    })?;
    match opened.freshness {
        ein_einb::Freshness::Fresh => {}
        why => {
            let stale = ein_core::Diagnostic::warning(
                ein_core::Kind::StaleContainer,
                format!(
                    "{} was written under different inputs ({why:?}){}",
                    path.display(),
                    if opened.derived_dropped {
                        " — its derived state was dropped and the program re-loaded"
                    } else {
                        ""
                    }
                ),
            );
            eprintln!("{}", stale.line());
        }
    }
    *ast = opened.ast;
    Ok(Loaded {
        kb: opened.kb,
        forms: opened.forms,
        how: How::Container {
            open: t.elapsed(),
            meta: Box::new(opened.meta),
            derived_dropped: opened.derived_dropped,
        },
    })
}

/// **A saved file read as its program** (Q-M1f.402): its `(commit …)` set
/// aside, so that what a command reads is the program's own H and its
/// structure, of which the node is a commitment. What `ein show hyps` and
/// `ein render h-space` read; a run of `ein solve` or `ein oracle` reads the
/// node, and the part of R above it.
pub fn program_of_saved(kb: &mut Kb) {
    if kb.program().commit.is_some() {
        kb.program_mut().commit = None;
    }
}

/// The flat top-level `(rule …)` declarations.
pub fn rule_forms(ast: &Ast, forms: &[NodeId]) -> Vec<NodeId> {
    forms
        .iter()
        .copied()
        .filter(|f| ast.head_name(*f) == Some("rule"))
        .collect()
}

/// A `SaturateError` as the line the run ends with.
///
/// A class is named qualified, and a conclusion no fact can hold names no
/// class: it is the rule and the slot as the program writes them, the line
/// `ein solve` prints too.
///
/// No line is an unbound `:assert` variable's: the loader refuses a
/// conclusion's variable nothing binds
/// ([`07-diagnostics.md` § 2.13](../../../../docs/ein/user/04-reference/07-diagnostics.md#213-a-conclusions-variable-nothing-binds);
/// `examples/ein-bugs/unbound-assert-var.ein`), and a parameter its activator
/// binds to a nested fact read only by a conclusion is seeded
/// (`parameter-nested-assert`), so `ein_infer::firing::FireError` has no
/// such case.
pub fn saturate_error_line(e: &ein_infer::saturator::SaturateError) -> String {
    use ein_infer::firing::FireError;
    use ein_infer::saturator::SaturateError;
    match e {
        // The compiler's line as `ein solve` prints it, in Ein's words.
        SaturateError::Compile(c) => c.to_string(),
        // Ein's spelling: the rule and the slot as the program writes them,
        // the line `ein solve` prints too.
        SaturateError::Fire(FireError::NotAFact(m)) => m.clone(),
        SaturateError::StepLimit(m) => {
            format!("ein.inference.saturator.SaturatorStepLimitError: {m}")
        }
        other => other.to_string(),
    }
}
