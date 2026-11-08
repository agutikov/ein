//! Shared loaders.
//!
//! The sentinel convention: print the failure to stderr and return `None`,
//! letting the caller `return 1`. Nothing here exits on a *load*
//! error, which is why the exit code is the caller's to choose.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

/// A file read as UTF-8 — including how it *fails*.
///
/// A missing or unreadable file is not an argument error: the run prints one
/// line naming the error class, the errno text and the quoted path
/// (`FileNotFoundError: [Errno 2] No such file or directory: '<path>'`, and
/// its two siblings below) and exits 1
/// ([`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)).
pub fn read_text_or_crash(path: &Path) -> String {
    let bytes = read_bytes_or_crash(path);
    match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => {
            // What `read_to_string` used to report here before the bytes had
            // to be sniffed for a `.einb` magic first: the decode is the same
            // one, moved a line later.
            let shown = ein_core::repr::repr_str(&path.display().to_string());
            eprintln!("UnicodeDecodeError: 'utf-8' codec can't decode bytes in {shown}");
            std::process::exit(1);
        }
    }
}

/// The same read, undecoded — what a caller that may have been handed a binary
/// container needs before it can tell which it is.
pub fn read_bytes_or_crash(path: &Path) -> Vec<u8> {
    use std::io::ErrorKind;
    match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            let shown = ein_core::repr::repr_str(&path.display().to_string());
            let line = match e.kind() {
                ErrorKind::NotFound => {
                    format!("FileNotFoundError: [Errno 2] No such file or directory: {shown}")
                }
                ErrorKind::PermissionDenied => {
                    format!("PermissionError: [Errno 13] Permission denied: {shown}")
                }
                ErrorKind::IsADirectory => {
                    format!("IsADirectoryError: [Errno 21] Is a directory: {shown}")
                }
                _ => format!("OSError: {e}: {shown}"),
            };
            eprintln!("{line}");
            std::process::exit(1);
        }
    }
}

/// Parse a file, printing a parse error to stderr.
pub fn parse_or_exit(ast: &mut Ast, path: &Path) -> Option<Vec<NodeId>> {
    let text = read_text_or_crash(path);
    match ein_ir::parse(ast, &text, path.to_str()) {
        Ok(forms) => Some(forms),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

/// `EINB\0` plus three pad bytes — the container's magic, and the **only**
/// sniff in this crate.
///
/// A second copy of `ein_einb::header::MAGIC`, because `ein-einb` is an
/// optional dependency and a `--no-default-features` build must still
/// recognise a container in order to refuse it. That is the case
/// [`AR-M1`](../../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium)'s
/// rule covers explicitly — *compared by a test, since a shared crate for
/// eight bytes is worse than the problem* — and the test is
/// `einb_cli::the_two_magic_constants_agree`.
///
/// It sniffed **five** bytes until M1e S1e.4.2's sibling stage (`EH-L2`), so
/// `EINB\0xyz` was *"a `.einb` container and this build has no `einb`
/// feature"* in the light build and a UTF-8 error in the default one — two
/// shipped feature sets disagreeing on garbage input.
pub const EINB_MAGIC: [u8; 8] = *b"EINB\0\0\0\0";

/// Whether these bytes open a `.einb` container, in **either** build.
///
/// The one predicate, so the two feature arms below cannot answer it
/// differently.
pub fn looks_like_einb(bytes: &[u8]) -> bool {
    bytes.len() >= EINB_MAGIC.len() && bytes[..EINB_MAGIC.len()] == EINB_MAGIC
}

/// A `Kb` from a path, in **either** format.
///
/// `base_dir` is the puzzle's own directory, so file-relative `(import …)`
/// forms resolve against it (S1.8.A3).
///
/// The `.einb` fork (T1a.8.1.7) is on the **magic bytes** and not on the
/// extension: a container renamed `puzzle.ein` is still a container, and
/// refusing to notice would be a parse error about a file that is not text.
/// `ast` must be empty, because a container carries its own program and
/// replaces it.
///
/// A container whose *inputs* have moved is opened anyway and said so on
/// stderr. There is nothing else to do — the caller was handed the `.einb` and
/// not the `.ein`, so "re-parse the source" (design/10 §4) is not available to
/// it — and the one thing that must not happen is believing it silently.
pub fn load_any_or_exit(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    load: &ein_ir::LoadOptions,
) -> Option<Kb> {
    load_any_query_or_exit(ast, terms, path, 0, load)
}

/// What a command's argv asks of the load: `--require-why`, on every command
/// that loads a program (M1f T1f.5.14.6). A command that does not define it
/// loads as the default does.
pub fn load_options(m: &clap::ArgMatches) -> ein_ir::LoadOptions {
    ein_ir::LoadOptions {
        require_why: m.try_get_one::<bool>("require-why").ok().flatten().copied() == Some(true),
    }
}

/// [`load_any_or_exit`], about query number `query` — M1c S1c.1.2's plural
/// `(query …)`, in both formats, because a container stores its program as
/// text and therefore carries every query the source had.
pub fn load_any_query_or_exit(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    query: usize,
    load: &ein_ir::LoadOptions,
) -> Option<Kb> {
    let bytes = read_bytes_or_crash(path);
    if looks_like_einb(&bytes) {
        #[cfg(feature = "einb")]
        return open_einb(ast, terms, path, &bytes, query, load);
        #[cfg(not(feature = "einb"))]
        {
            eprintln!(
                "kb load error: {} is a .einb container and this build has no `einb` feature",
                path.display()
            );
            return None;
        }
    }
    let text = match String::from_utf8(bytes) {
        Ok(t) => t,
        Err(_) => {
            let shown = ein_core::repr::repr_str(&path.display().to_string());
            eprintln!("UnicodeDecodeError: 'utf-8' codec can't decode bytes in {shown}");
            std::process::exit(1);
        }
    };
    let forms = match ein_ir::parse(ast, &text, path.to_str()) {
        Ok(forms) => forms,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };
    match ein_ir::load_query_with(ast, terms, &forms, path.parent(), query, load) {
        Ok(kb) => Some(kb),
        Err(e) => {
            eprintln!("kb load error: {e}");
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
) -> Option<Kb> {
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
        ..ein_einb::OpenOptions::default()
    };
    match ein_einb::open_bytes(bytes, terms, &opts) {
        Ok(opened) => {
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
            Some(opened.kb)
        }
        Err(e) => {
            eprintln!("kb load error: {e}");
            None
        }
    }
}

/// The flat top-level `(rule …)` / `(hrule …)` declarations.
pub fn rule_forms(ast: &Ast, forms: &[NodeId]) -> Vec<NodeId> {
    forms
        .iter()
        .copied()
        .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
        .collect()
}

/// How a run *fails* on a rule that will not compile.
///
/// The error reaches `solve` and `saturate` alike, and both end on the one
/// line `ein.inference.compile.CompileError: <message>` and exit 1 — the
/// class named, then the message
/// ([`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)).
pub fn compile_error_line(msg: impl std::fmt::Display) -> String {
    format!("ein.inference.compile.CompileError: {msg}")
}

/// A `SaturateError` as the line the run ends with.
///
/// `KeyError` is named bare and the other classes qualified, and a conclusion
/// no fact can hold names no class: it is the rule and the slot as the program
/// writes them, the line `ein solve` prints too (it was `TypeError: …` until
/// M1e2 S1e2.2.18). `KeyError`'s message is quoted the way
/// [`repr_str`](ein_core::repr::repr_str) quotes a string — so an unbound
/// `:assert` variable ends the run as
/// `KeyError: "unbound var ?v1 in :assert — bindings: {…}"`, quotes and all
/// ([`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)).
/// The message was printed alone until 2026-08-20, when
/// [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-fuzzer)'s
/// fuzzer produced the first input that reaches it; no corpus file had ever
/// hit it. `examples/ein-bugs/unbound-assert-var.ein` is the fixture.
pub fn saturate_error_line(e: &ein_infer::saturator::SaturateError) -> String {
    use ein_infer::firing::FireError;
    use ein_infer::saturator::SaturateError;
    match e {
        SaturateError::Compile(c) => compile_error_line(c),
        SaturateError::Fire(FireError::UnboundVar(m)) => {
            format!("KeyError: {}", ein_core::repr::repr_str(m))
        }
        // Ein's spelling since M1e2 S1e2.2.18: the rule and the slot as the
        // program writes them, the line `ein solve` prints too.
        SaturateError::Fire(FireError::NotAFact(m)) => m.clone(),
        SaturateError::StepLimit(m) => {
            format!("ein.inference.saturator.SaturatorStepLimitError: {m}")
        }
        other => other.to_string(),
    }
}
