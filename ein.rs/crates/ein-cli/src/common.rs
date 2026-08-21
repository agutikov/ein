//! Shared loaders — the Rust half of `ein/cli/_common.py`.
//!
//! The sentinel convention is ein.py's: print the failure to stderr and return
//! `None`, letting the caller `return 1`. Nothing here exits on a *load*
//! error, which is why the exit code is the caller's to choose.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

/// `Path.read_text(encoding="utf-8")` — including how it *fails*.
///
/// ein.py calls it unguarded, so a missing file is not an argument error but a
/// `FileNotFoundError` traceback and exit 1: the `crash-parity` group of
/// [Q-M1a.14](../../../../plans/m1a_rust/open_questions.md#q-m1a14--crash-parity).
/// The harness compares the exception class off the last stderr line
/// (`tier::exception_class`), so the class is named, with CPython's `OSError`
/// text for the errnos a CLI actually meets.
pub fn read_text_or_crash(path: &Path) -> String {
    let bytes = read_bytes_or_crash(path);
    match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => {
            // What `read_text` raises, and what `read_to_string` used to raise
            // here before the bytes had to be sniffed for a `.einb` magic
            // first: the decode is the same one, moved a line later.
            let shown = ein_core::pyrepr::repr_str(&path.display().to_string());
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
            let shown = ein_core::pyrepr::repr_str(&path.display().to_string());
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

/// `_parse_or_exit` — parse a file, printing a parse error to stderr.
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

/// `_load_kb_or_exit` — a `Kb` from a path, in **either** format.
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
pub fn load_any_or_exit(ast: &mut Ast, terms: &mut Terms, path: &Path) -> Option<Kb> {
    let bytes = read_bytes_or_crash(path);
    #[cfg(feature = "einb")]
    if ein_einb::is_einb(&bytes) {
        return open_einb(ast, terms, path, &bytes);
    }
    #[cfg(not(feature = "einb"))]
    if bytes.starts_with(b"EINB\0") {
        eprintln!(
            "kb load error: {} is a .einb container and this build has no `einb` feature",
            path.display()
        );
        return None;
    }
    let text = match String::from_utf8(bytes) {
        Ok(t) => t,
        Err(_) => {
            let shown = ein_core::pyrepr::repr_str(&path.display().to_string());
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
    match ein_ir::load(ast, terms, &forms, path.parent()) {
        Ok(kb) => Some(kb),
        Err(e) => {
            eprintln!("kb load error: {e}");
            None
        }
    }
}

#[cfg(feature = "einb")]
fn open_einb(ast: &mut Ast, terms: &mut Terms, path: &Path, bytes: &[u8]) -> Option<Kb> {
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

/// `_rule_forms` — the flat top-level `(rule …)` / `(hrule …)` declarations.
pub fn rule_forms(ast: &Ast, forms: &[NodeId]) -> Vec<NodeId> {
    forms
        .iter()
        .copied()
        .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
        .collect()
}

/// How ein.py *fails* on an error it does not catch.
///
/// `CompileError` propagates out of `solve` and `saturate` alike, so CPython
/// prints a traceback whose last line is
/// `ein.inference.compile.CompileError: <message>`. The `crash-parity` group
/// compares the exit code plus the class off that line
/// ([Q-M1a.14](../../../../plans/m1a_rust/open_questions.md#q-m1a14--crash-parity)),
/// and P1a.3 already brought the message body to parity — so naming the class
/// is what makes the whole line match.
pub fn compile_error_line(msg: impl std::fmt::Display) -> String {
    format!("ein.inference.compile.CompileError: {msg}")
}

/// A `SaturateError` as the line ein.py's traceback would end with.
///
/// CPython prints a builtin exception by its bare name and anything else by
/// its qualified one, and `KeyError`'s `str` is the **repr** of its key — so
/// an unbound `:assert` variable ends a traceback as
/// `KeyError: "unbound var ?v1 in :assert — bindings: {…}"`, quotes and all.
/// ein.rs printed the message alone until 2026-08-20, when
/// [S1a.6.6](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s
/// fuzzer produced the first input that reaches it —
/// [Q-M1a.14](../../../../plans/m1a_rust/open_questions.md#q-m1a14--crash-parity)
/// named this exact case ("a `KeyError` from an unbound `:assert` var is
/// caught nowhere") and no corpus file had ever hit it.
/// `examples/ein-bugs/unbound-assert-var.ein` is the fixture.
pub fn saturate_error_line(e: &ein_infer::saturator::SaturateError) -> String {
    use ein_infer::firing::FireError;
    use ein_infer::saturator::SaturateError;
    match e {
        SaturateError::Compile(c) => compile_error_line(c),
        SaturateError::Fire(FireError::UnboundVar(m)) => {
            format!("KeyError: {}", ein_core::pyrepr::repr_str(m))
        }
        SaturateError::Fire(FireError::NotAFact(m)) => format!("TypeError: {m}"),
        SaturateError::StepLimit(m) => {
            format!("ein.inference.saturator.SaturatorStepLimitError: {m}")
        }
        other => other.to_string(),
    }
}
