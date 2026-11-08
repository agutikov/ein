//! Generates the parser from the grammar, `src/ein.l` and `src/ein.y`
//! (M1f S1f.5.5, T1f.5.5.3), with grmtools' `CTParserBuilder`: a conflict in
//! the grammar, or a token rule no production uses that `%expect-unused` does
//! not name, fails this build. Recovery is CPCT+, from the patched `lrpar`
//! the workspace vendors (F11).

use lrlex::CTLexerBuilder;
use lrpar::RecoveryKind;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/ein.l");
    println!("cargo::rerun-if-changed=src/ein.y");
    CTLexerBuilder::new()
        .lrpar_config(|c| {
            c.recoverer(RecoveryKind::CPCTPlus)
                .grammar_in_src_dir("ein.y")
                .expect("src/ein.y")
        })
        .lexer_in_src_dir("ein.l")
        .expect("src/ein.l")
        .build()
        .unwrap_or_else(|e| panic!("the grammar does not build: {e}"));
}
