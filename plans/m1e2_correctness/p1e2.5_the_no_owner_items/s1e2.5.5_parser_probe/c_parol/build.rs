use parol::build::Builder;
fn main(){ println!("cargo:rerun-if-changed=ein.par");
 if let Err(e)=Builder::with_explicit_output_dir("src").grammar_file("ein.par").expanded_grammar_output_file("ein-exp.par").parser_output_file("ein_parser.rs").actions_output_file("ein_grammar_trait.rs").user_type_name("EinGrammar").user_trait_module_name("ein_grammar").trim_parse_tree().generate_parser(){ panic!("{e:?}"); } }
