//! `ein kb` — the container's CLI surface (T1a.8.1.7).
//!
//! One verb, `save`, because the other direction is not a command: a `.einb`
//! is read *wherever a `.ein` path is accepted*, by
//! [`crate::common::read_program`], which sniffs the magic bytes. There is
//! deliberately no `ein kb load`, and no `.einb → .ein` exporter — the stage's
//! own note says why, and `--dump-states` already prints a KB as text.

use std::path::{Path, PathBuf};

use ein_core::Terms;
use ein_einb::{KbState, SaveOptions, Source};
use ein_ir::Ast;

use crate::common::{How, load_held, read_program};

/// `ein kb save <file> <out.einb> [--saturate]`.
///
/// `file` is read as every command reads a program (M1f S1f.5.5,
/// T1f.5.5.13): a `.ein` is parsed and loaded against its own directory, and a
/// `.einb` is opened and saved again, its sources and its state carried over,
/// where until then it was refused as text that is not UTF-8 (W40).
pub fn cmd_save(file: &str, out: &str, saturate: bool, load: &ein_ir::LoadOptions) -> i32 {
    let path = PathBuf::from(file);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let loaded = match read_program(&path)
        .and_then(|held| load_held(&mut ast, &mut terms, &path, held, 0, load))
    {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let (mut kb, forms) = (loaded.kb, loaded.forms);
    // The default is the **loaded** KB, and that is the whole reason `ein solve
    // x.einb` can be byte-identical to `ein solve x.ein`: the solve starts from
    // the same place. `--saturate` banks the fixpoint instead, which is faster
    // to open and is a different starting point — the root's derivations are
    // already there, so a trace of that run has nothing to say about them. A
    // container saved again keeps the state it was opened in, a saturated one
    // saturated, unless its derived state was not believed; a `Solved` one is
    // `Saturated`, since this command writes no `SOLUTIONS`.
    let (kept, sources) = match &loaded.how {
        How::Text { .. } => (KbState::Loaded, Source::of(&path).into_iter().collect()),
        How::Container {
            meta,
            derived_dropped,
            ..
        } => (
            match meta.state {
                _ if *derived_dropped => KbState::Loaded,
                KbState::Solved => KbState::Saturated,
                s => s,
            },
            meta.sources.clone(),
        ),
    };
    let state = if saturate {
        if let Err(e) = ein_infer::saturate_events(&ast, &mut terms, &mut kb) {
            eprintln!("{}", crate::common::saturate_error_line(&e));
            return 1;
        }
        KbState::Saturated
    } else {
        kept
    };
    let opts = SaveOptions {
        state,
        sources,
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
