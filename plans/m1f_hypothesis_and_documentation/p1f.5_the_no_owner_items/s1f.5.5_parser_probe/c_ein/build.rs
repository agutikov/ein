//! The final grammar, `../grammar/ein.l` and `ein.y`, compiled with CPCT+
//! recovery on (S1f.5.5, F11). A conflict fails this build.
use lrlex::CTLexerBuilder;
use lrpar::RecoveryKind;
fn main() {
    let g = concat!(env!("CARGO_MANIFEST_DIR"), "/../grammar");
    let out = std::env::var("OUT_DIR").unwrap();
    let y_out = format!("{out}/ein.y.rs");
    CTLexerBuilder::new()
        .lrpar_config(move |c| {
            c.recoverer(RecoveryKind::CPCTPlus)
                .grammar_path(format!("{g}/ein.y"))
                .output_path(&y_out)
                .mod_name("ein_y")
        })
        .lexer_path(format!("{g}/ein.l"))
        .output_path(format!("{out}/ein.l.rs"))
        .mod_name("ein_l")
        .build()
        .unwrap();
    for f in ["ein.l", "ein.y", "build_ast.rs"] {
        println!("cargo:rerun-if-changed={g}/{f}");
    }
}
