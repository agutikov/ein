use crate::ein_grammar_trait::{EinGrammarTrait,Program,Refused}; use parol_runtime::Result;
#[derive(Debug,Default)] pub struct EinGrammar<'t>{ pub n:usize, pub refused:bool, _p:std::marker::PhantomData<&'t str> }
impl<'t> EinGrammarTrait<'t> for EinGrammar<'t>{
 fn program(&mut self,_a:&Program<'t>)->Result<()>{ self.n+=1; Ok(()) }
 fn refused(&mut self,_a:&Refused<'t>)->Result<()>{ self.refused=true; Ok(()) } }
