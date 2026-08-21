//! `ein kb` — the container's CLI surface (T1a.8.1.7).
//!
//! One verb, `save`, because the other direction is not a command: a `.einb`
//! is read *wherever a `.ein` path is accepted*, by
//! [`crate::common::load_any_or_exit`], which sniffs the magic bytes. There is
//! deliberately no `ein kb load`, and no `.einb → .ein` exporter — the stage's
//! own note says why, and `--dump-states` already prints a KB as text.
//!
//! This is a **new surface**: `.einb` has no ein.py counterpart, so nothing
//! here is reproducing an `argparse` parser.

use std::path::{Path, PathBuf};

use ein_core::Terms;
use ein_einb::{KbState, SaveOptions, Source};
use ein_ir::Ast;

use crate::common::read_text_or_crash;

/// `ein kb save <file.ein> <out.einb> [--saturate]`.
pub fn cmd_save(file: &str, out: &str, saturate: bool) -> i32 {
    let path = PathBuf::from(file);
    let text = read_text_or_crash(&path);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = match ein_ir::parse(&mut ast, &text, path.to_str()) {
        Ok(forms) => forms,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let mut kb = match ein_ir::load(&mut ast, &mut terms, &forms, path.parent()) {
        Ok(kb) => kb,
        Err(e) => {
            eprintln!("kb load error: {e}");
            return 1;
        }
    };
    // The default is the **loaded** KB, and that is the whole reason `ein solve
    // x.einb` can be byte-identical to `ein solve x.ein`: the solve starts from
    // the same place. `--saturate` banks the fixpoint instead, which is faster
    // to open and is a different starting point — the root's derivations are
    // already there, so a trace of that run has nothing to say about them.
    let state = if saturate {
        if let Err(e) = ein_infer::saturate_events(&ast, &mut terms, &mut kb) {
            eprintln!("{}", crate::common::saturate_error_line(&e));
            return 1;
        }
        KbState::Saturated
    } else {
        KbState::Loaded
    };
    let opts = SaveOptions {
        state,
        sources: Source::of(&path).into_iter().collect(),
        solutions: None,
    };
    match ein_einb::save(
        Path::new(out),
        &kb,
        &terms,
        &mut ast,
        &forms,
        path.parent(),
        &opts,
    ) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("kb save error: {e}");
            1
        }
    }
}
