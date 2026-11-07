#![forbid(unsafe_code)]
pub mod ein_grammar; pub mod ein_grammar_trait; pub mod ein_parser;
/// Accept or reject, and where. The typed tree parol builds is what is timed; it is not walked into `Node`.
pub fn accepts(s:&str)->Result<usize,String>{ let mut g=ein_grammar::EinGrammar::default();
  match ein_parser::parse(s,"x.ein",&mut g){ Ok(_)=> if g.refused {Err("refused token".into())} else {Ok(g.n)}, Err(e)=>Err(format!("{e:?}")) } }
