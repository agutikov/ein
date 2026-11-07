use lrlex::CTLexerBuilder; use lrpar::RecoveryKind;
fn main(){ CTLexerBuilder::new().lrpar_config(|c| c.recoverer(RecoveryKind::None).grammar_in_src_dir("ein.y").unwrap()).lexer_in_src_dir("ein.l").unwrap().build().unwrap(); }
