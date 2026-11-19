//! `ein show` — what a program holds, as Ein text.
//!
//! Where `ein render` draws, this lists: stdout is forms a `.ein` file could
//! hold and nothing else, so it pipes, and whatever is said about them is a
//! `note:` on stderr. One thing is shown today, `hyps`.

use std::path::Path;

use ein_core::Terms;
use ein_ir::Ast;

use crate::common::load_program;

/// `ein show hyps FILE` — the hypothesis set `H`, one candidate a line, as
/// the fact is written, every candidate, uncut, in the universe's canonical
/// order ([`ein_render::structure::hypotheses_text`]).
///
/// It is `H` as root's generator builds it, on whichever rung the program is
/// on: the set every route searches over, `ein solve -H` lists by relation
/// and `ein render h-space`'s document lists with what the structure says of
/// each ([`ein_infer::structure::hypotheses`], the one function all three
/// take it from). **It costs what building `H` costs**: root's saturation
/// and the generator's pass, and no node.
///
/// An empty `H` prints nothing and exits 0. A `note:` on stderr says which
/// rung built `H` and how many candidates it holds, after the `WARNING:` line
/// where the static check flags the program (Q-M1f.564). A program the loader
/// refuses is refused as every command refuses it, exit 1, and so is one the
/// engine refuses to read.
pub fn cmd_hyps(file: &str, load: &ein_ir::LoadOptions) -> i32 {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = load_program(&mut ast, &mut terms, Path::new(file), load) else {
        return 1;
    };
    crate::common::program_of_saved(&mut kb);
    let h = match ein_infer::structure::hypotheses(&mut kb, &mut terms, &ast) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    if !h.monotone {
        eprintln!("{}", ein_infer::detector::warning_line(file));
    }
    eprintln!(
        "note: {file}: {}",
        ein_render::structure::hypotheses_note(&h)
    );
    print!("{}", ein_render::structure::hypotheses_text(&h, &terms));
    0
}
