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
    use std::io::ErrorKind;
    match std::fs::read_to_string(path) {
        Ok(text) => text,
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
                // `read_to_string` rejects non-UTF-8 the way `read_text` does.
                ErrorKind::InvalidData => {
                    format!("UnicodeDecodeError: 'utf-8' codec can't decode bytes in {shown}")
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

/// `_load_kb_or_exit` — parse + build a `Kb`, or print the failure.
///
/// `base_dir` is the puzzle's own directory, so file-relative `(import …)`
/// forms resolve against it (S1.8.A3).
pub fn load_kb_or_exit(ast: &mut Ast, terms: &mut Terms, path: &Path) -> Option<Kb> {
    let forms = parse_or_exit(ast, path)?;
    match ein_ir::load(ast, terms, &forms, path.parent()) {
        Ok(kb) => Some(kb),
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
pub fn saturate_error_line(e: &ein_infer::saturator::SaturateError) -> String {
    match e {
        ein_infer::saturator::SaturateError::Compile(c) => compile_error_line(c),
        other => other.to_string(),
    }
}
