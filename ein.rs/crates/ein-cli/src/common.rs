//! Shared loaders, and **the one read of a program from a path** (M1f
//! S1f.5.5, T1f.5.5.13).
//!
//! Every command that takes a program reads it here: [`read_program`] reads
//! the bytes and tells a `.einb` container from UTF-8 text by the magic bytes
//! (through `ein_ir::read` since M1f S1f.5.22, which the import resolver
//! reads a module through too), [`load_held`] loads either (a text against its own directory, so a
//! file-relative `(import …)` resolves; a container opened), and
//! [`parse_held`] parses either without loading (a container's stored
//! program). Until that task `ein saturate` read text only and loaded it with
//! no base directory, and the `render` views that parse and `ein kb save` read
//! text only, so each refused a `.einb` another command answered (W40) and
//! `saturate` a file-relative import (W41); and a file that is not UTF-8 was
//! refused with a Python line, `UnicodeDecodeError: …`, by a `process::exit`
//! that took `ein test`'s whole selection down with it, exit 1.
//!
//! The convention: nothing here exits. A failure comes back as the text the
//! command prints to stderr, and the exit code is the command's to choose —
//! 1 for `solve`, `saturate`, `oracle`, `render` and `kb save`, and for `ein
//! test` an `ERROR` row and 2 (`defined_behaviour.md` § 4).

use std::path::Path;
use std::time::{Duration, Instant};

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

pub use ein_ir::read::Held;

/// Read a program from `path`: the one read every command makes of one,
/// [`ein_ir::read::read_program`], its failure the line the command prints.
///
/// The read moved to `ein-ir` at M1f S1f.5.22, so that the import resolver
/// reads an imported module through it too and refuses one as this refuses
/// the program it is given, naming the module's path (the close review's
/// finding 2, W74): a file that cannot be read gives one line naming the
/// error class, the errno text and the quoted path, and a file that is
/// neither a `.einb` container nor UTF-8 gives
/// [`ein_ir::read::not_utf8_line`].
pub fn read_program(path: &Path) -> Result<Held, String> {
    ein_ir::read::read_program(path).map_err(|e| e.line)
}

/// A program path's text, for the one caller that wants the text and not the
/// loaded program: `--dump-answer`'s inlining (`solve::inlined_program`).
pub struct ProgramText {
    pub text: String,
    /// The name its positions are parsed under.
    pub name: Option<String>,
    /// Where its file-relative imports resolve; `None` for a container's.
    pub base: Option<std::path::PathBuf>,
}

/// The text of a program path: a text as written, named by its path and
/// resolved against its directory; or the program a `.einb` stores, the
/// resolved and import-flattened text `ein kb save` wrote, named
/// `<program stored in x.einb>` (`ein_einb::program_name`) and with nothing
/// left to resolve. Until M1f S1f.5.12 (T1f.5.12.5, W43) a container was
/// refused here, and `ein solve x.einb --dump-answer DIR` exited 2 where the
/// `.ein` answered.
pub fn program_text(path: &Path) -> Result<ProgramText, String> {
    match read_program(path)? {
        Held::Text(text) => Ok(ProgramText {
            text,
            name: path.to_str().map(str::to_string),
            base: path.parent().map(Path::to_path_buf),
        }),
        #[cfg(feature = "einb")]
        Held::Container(bytes) => Ok(ProgramText {
            text: ein_einb::program_of(&bytes, true).map_err(|e| format!("kb load error: {e}"))?,
            name: Some(ein_einb::program_name(Some(&path.display().to_string()))),
            base: None,
        }),
        #[cfg(not(feature = "einb"))]
        Held::Container(_) => Err(no_einb_line(path)),
    }
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
/// default since M1f T1f.5.14.7, unless `--no-require-why`, on every command
/// that loads a program (T1f.5.14.6 gave it `--require-why`, default off). A
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

/// Load what [`read_program`] read, about query number `query` — M1c
/// S1c.1.2's plural `(query …)`, in both formats, because a container stores
/// its program as text and therefore carries every query the source had.
///
/// A text is parsed under its path and loaded with its own directory as the
/// base, so a file-relative `(import …)` resolves against it (S1.8.A3) on
/// every command. A container is opened (T1a.8.1.7): the fork is on the
/// **magic bytes** and not on the extension, so a container renamed
/// `puzzle.ein` is still a container. `ast` must be empty, because a
/// container carries its own program and replaces it.
///
/// A container whose *inputs* have moved is opened anyway and said so on
/// stderr. There is nothing else to do — the caller was handed the `.einb` and
/// not the `.ein`, so "re-parse the source" (design/10 §4) is not available to
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
            let t = Instant::now();
            let forms = ein_ir::parse(ast, &text, path.to_str()).map_err(|e| e.to_string())?;
            let parse = t.elapsed();
            let t = Instant::now();
            let kb = ein_ir::load_query_with(ast, terms, &forms, path.parent(), query, load)
                .map_err(|e| format!("kb load error: {e}"))?;
            Ok(Loaded {
                kb,
                forms,
                how: How::Text {
                    parse,
                    load: t.elapsed(),
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

/// Parse what [`read_program`] read, without loading it: a text as written,
/// or the program a container stores, parsed under
/// `ein_einb::program_name`'s name. What `ein render rules`, `rule`,
/// `constraints` and `program` read.
pub fn parse_held(ast: &mut Ast, path: &Path, held: Held) -> Result<Vec<NodeId>, String> {
    match held {
        Held::Text(text) => ein_ir::parse(ast, &text, path.to_str()).map_err(|e| e.to_string()),
        #[cfg(feature = "einb")]
        Held::Container(bytes) => {
            let text =
                ein_einb::program_of(&bytes, true).map_err(|e| format!("kb load error: {e}"))?;
            let name = ein_einb::program_name(Some(&path.display().to_string()));
            ein_ir::parse(ast, &text, Some(&name)).map_err(|e| {
                format!(
                    "kb load error: {}",
                    ein_einb::EinbError::Program(e.to_string())
                )
            })
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
    let opts = ein_einb::OpenOptions {
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
        // Every position in the stored program names the container (M1f
        // T1f.5.5.13), where it named `<string>`.
        name: Some(path.display().to_string()),
        ..ein_einb::OpenOptions::default()
    };
    let opened =
        ein_einb::open_bytes(bytes, terms, &opts).map_err(|e| format!("kb load error: {e}"))?;
    match opened.freshness {
        ein_einb::Freshness::Fresh => {}
        why => eprintln!(
            "warning: {} was written under different inputs ({why:?}){}",
            path.display(),
            if opened.derived_dropped {
                " — its derived state was dropped and the program re-loaded"
            } else {
                ""
            }
        ),
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

/// The flat top-level `(rule …)` / `(hrule …)` declarations.
pub fn rule_forms(ast: &Ast, forms: &[NodeId]) -> Vec<NodeId> {
    forms
        .iter()
        .copied()
        .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
        .collect()
}

/// A `SaturateError` as the line the run ends with.
///
/// A class is named qualified, and a conclusion no fact can hold names no
/// class: it is the rule and the slot as the program writes them, the line
/// `ein solve` prints too (it was `TypeError: …` until M1e2 S1e2.2.18).
///
/// **`KeyError` was the other unqualified class until M1f S1f.5.18**: an
/// unbound `:assert` variable ended the run as `KeyError: "unbound var ?v1 in
/// :assert — bindings: {…}"`, quotes and all, the message quoted the way
/// [`repr_str`](ein_core::repr::repr_str) quotes a string. The message was
/// printed alone until 2026-08-20, when
/// [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-fuzzer)'s
/// fuzzer produced the first input that reaches it.
/// `examples/ein-bugs/unbound-assert-var.ein` was the fixture until M1f
/// S1f.5.10, which refuses its program at load (F13); the one input left, a
/// parameter its activator binds to a nested fact read only by a conclusion
/// (`W58/parameter-nested-assert`), answers since S1f.5.18 seeds the
/// parameter, which retired `ein_infer::firing::FireError`'s `UnboundVar` and
/// this line ([`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)).
pub fn saturate_error_line(e: &ein_infer::saturator::SaturateError) -> String {
    use ein_infer::firing::FireError;
    use ein_infer::saturator::SaturateError;
    match e {
        // The compiler's line as `ein solve` prints it, in Ein's words, since
        // M1f S1f.5.21; it was `ein.inference.compile.CompileError: <message>`
        // here and the message alone there until then (the close review's
        // finding 1).
        SaturateError::Compile(c) => c.to_string(),
        // Ein's spelling since M1e2 S1e2.2.18: the rule and the slot as the
        // program writes them, the line `ein solve` prints too.
        SaturateError::Fire(FireError::NotAFact(m)) => m.clone(),
        SaturateError::StepLimit(m) => {
            format!("ein.inference.saturator.SaturatorStepLimitError: {m}")
        }
        other => other.to_string(),
    }
}
