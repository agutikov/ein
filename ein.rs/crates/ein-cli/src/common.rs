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
//! The convention: nothing here exits. A failure comes back as a
//! [`Refusal`], the text the command prints to stderr and its findings as
//! values, and the exit code is the command's to choose — 1 for `solve`,
//! `saturate`, `oracle`, `render` and `kb save`, and for `ein test` an `ERROR`
//! row and 2 (`docs/ein/user/04-reference/07-diagnostics.md` § 2).
//!
//! **What a run says on stderr is kept as values** (Q-M2.9): every
//! diagnostic a command prints goes through [`say`], or [`refuse`] for a
//! refusal's findings, and the load's notes through [`keep_notes`], so that
//! the run summary's `diagnostics` ([`said`]) are the lines stderr printed,
//! in their order.

use std::path::Path;
use std::time::{Duration, Instant};

use ein_core::{Diagnostic, Kb, Kind, Terms};
use ein_ir::{Ast, NodeId};

pub use ein_ir::read::Held;

/// **A program refused**, as a command meets it: the text it prints on
/// stderr, byte for byte what it printed before its findings were values,
/// and the findings, each with its kind and its place (Q-M2.9).
#[derive(Clone, Debug)]
pub struct Refusal {
    /// What stderr prints: a load's findings `; `-joined behind `kb load
    /// error: `, a parse's report, a judgement's conflicts one a line.
    pub line: String,
    /// The refusal's findings.
    pub findings: Vec<Diagnostic>,
}

impl Refusal {
    /// A `.einb` container refused: the stored program's refusal, its
    /// findings, or the container's own, which a build cannot read
    /// ([`Kind::Unreadable`]).
    #[cfg(feature = "einb")]
    fn of_einb(e: ein_einb::EinbError) -> Refusal {
        let line = format!("kb load error: {e}");
        let findings = match &e {
            ein_einb::EinbError::Program(k) => k.findings().to_vec(),
            _ => vec![Diagnostic::error(Kind::Unreadable, e.to_string())],
        };
        Refusal { line, findings }
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.line)
    }
}

impl From<Refusal> for String {
    fn from(r: Refusal) -> String {
        r.line
    }
}

/// **What the run has said on stderr, as values**, in the order it was
/// printed: every diagnostic [`say`] and [`refuse`] printed. The load's
/// notes are kept apart until the run ends ([`keep_notes`]).
static SAID: std::sync::Mutex<Vec<Diagnostic>> = std::sync::Mutex::new(Vec::new());

/// **Print a diagnostic on stderr**, as its line
/// ([`Diagnostic::line`]), and keep it for the run summary.
pub fn say(d: Diagnostic) {
    eprintln!("{}", d.line());
    SAID.lock().unwrap_or_else(|e| e.into_inner()).push(d);
}

/// **Print a refusal on stderr**, its line, and keep its findings for the
/// run summary.
pub fn refuse(r: &Refusal) {
    eprintln!("{}", r.line);
    SAID.lock()
        .unwrap_or_else(|e| e.into_inner())
        .extend(r.findings.iter().cloned());
}

/// **What the run said, for the summary** (Q-M2.9): every diagnostic
/// printed so far, in its order, then the load's notes not yet printed,
/// which [`say_notes`] prints when the run ends. So a summary written before
/// the run ends lists what stderr holds once it has.
pub fn said() -> Vec<Diagnostic> {
    let mut out = SAID.lock().unwrap_or_else(|e| e.into_inner()).clone();
    out.extend(
        NOTES
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending
            .iter()
            .cloned(),
    );
    out
}

/// Read a program from `path`: the one read every command makes of one,
/// [`ein_ir::read::read_program`], its failure the line the command prints.
///
/// The read is `ein-ir`'s, so that the import resolver reads an imported
/// module through it too and refuses one as this refuses the program it is
/// given, naming the module's path: a file that cannot be read gives one line naming the
/// error class, the errno text and the quoted path, and a file that is
/// neither a `.einb` container nor UTF-8 gives
/// [`ein_ir::read::not_utf8_line`].
pub fn read_program(path: &Path) -> Result<Held, Refusal> {
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
            say(ein_infer::detector::warning(&file));
        }
    })));
    ein_ir::read::read_program(path).map_err(|e| Refusal {
        findings: vec![e.diagnostic()],
        line: e.line,
    })
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

/// The refusal of a container by a build with no `einb` feature: a file
/// this build cannot read ([`Kind::Unreadable`]).
#[cfg(not(feature = "einb"))]
fn no_einb(path: &Path) -> Refusal {
    let message = format!(
        "{} is a .einb container and this build has no `einb` feature",
        path.display()
    );
    Refusal {
        line: format!("kb load error: {message}"),
        findings: vec![Diagnostic::error(Kind::Unreadable, message)],
    }
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
) -> Result<Loaded, Refusal> {
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
            .map_err(|e| Refusal {
                line: e.to_string(),
                findings: e.findings(),
            })?;
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
            Err(no_einb(path))
        }
    }
}

/// **What the load noticed and did not refuse**: each note, kept as a value
/// for [`say_notes`], which prints its line, `note: <message>`
/// ([`ein_core::Diagnostic::line`]), and for the run summary ([`said`]), on
/// every command, since every command that loads a program loads it through
/// [`load_held`]: the hypothesis mechanisms' notes (Q-M1f.242). A command may
/// load one file several times, once a query and once a question, and each
/// line carries its file and position, so a note is kept the first time the
/// process meets it and never again. No answer and no exit code moves.
fn keep_notes(notes: &[ein_core::Diagnostic]) {
    let mut w = NOTES.lock().unwrap_or_else(|e| e.into_inner());
    for note in notes {
        let line = note.line();
        if !w.said.contains(&line) {
            w.said.push(line);
            w.pending.push(note.clone());
        }
    }
}

/// The load's notes: every line met, and the notes not yet printed.
struct Notes {
    said: Vec<String>,
    pending: Vec<ein_core::Diagnostic>,
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
    for note in pending {
        eprintln!("{}", note.line());
    }
}

/// **A conflict between the hypothesis mechanisms refuses the program**
/// (Q-M1f.245), on every command, since
/// every command that loads a program loads it here: one line per conflict,
/// each `kb load error: …`, at the command's load-error exit, the conflicts
/// the refusal's findings. A list that
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
) -> Result<(), Refusal> {
    match ein_infer::mechanisms::judge_loaded(ast, terms, kb, again) {
        Ok(notes) => {
            keep_notes(&notes);
            Ok(())
        }
        Err(conflicts) => Err(Refusal {
            line: conflicts
                .iter()
                .map(|d| format!("kb load error: {d}"))
                .collect::<Vec<_>>()
                .join("\n"),
            findings: conflicts,
        }),
    }
}

/// Parse what [`read_program`] read, without loading it: a text as written,
/// or the program a container stores, parsed under
/// `ein_einb::program_name`'s name. What `ein render rules`, `rule`,
/// `constraints` and `program` read.
pub fn parse_held(ast: &mut Ast, path: &Path, held: Held) -> Result<Vec<NodeId>, Refusal> {
    match held {
        // A saved file's program, as written.
        Held::Text(text) => {
            ein_ir::saved::parse_text(ast, &text, path.to_str()).map_err(|e| Refusal {
                line: e.to_string(),
                findings: e.findings(),
            })
        }
        #[cfg(feature = "einb")]
        Held::Container(bytes) => {
            let text = ein_einb::program_of(&bytes, true).map_err(Refusal::of_einb)?;
            let name = ein_einb::program_name(Some(&path.display().to_string()));
            ein_ir::parse(ast, &text, Some(&name))
                .map_err(|e| Refusal::of_einb(ein_einb::EinbError::Program(e.into())))
        }
        #[cfg(not(feature = "einb"))]
        Held::Container(_) => Err(no_einb(path)),
    }
}

/// [`read_program`] and [`load_held`], about query number `query`, with the
/// refusal printed to stderr ([`refuse`]) and handed back: the caller's
/// load-error exit, and what its summary says.
pub fn load_or_refuse(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Result<Kb, Refusal> {
    match read_program(path).and_then(|held| load_held(ast, terms, path, held, query, load)) {
        Ok(loaded) => Ok(loaded.kb),
        Err(e) => {
            refuse(&e);
            Err(e)
        }
    }
}

/// [`load_or_refuse`], the refusal printed: `None` is the caller's
/// load-error exit.
pub fn load_program_query(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Option<Kb> {
    load_or_refuse(ast, terms, path, query, load).ok()
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

/// [`read_program`] and [`parse_held`], with the refusal printed to stderr.
pub fn parse_program(ast: &mut Ast, path: &Path) -> Option<Vec<NodeId>> {
    match read_program(path).and_then(|held| parse_held(ast, path, held)) {
        Ok(forms) => Some(forms),
        Err(e) => {
            refuse(&e);
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
) -> Result<Loaded, Refusal> {
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
    let opened = ein_einb::open_bytes(bytes, terms, &opts()).map_err(Refusal::of_einb)?;
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
            say(stale);
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
