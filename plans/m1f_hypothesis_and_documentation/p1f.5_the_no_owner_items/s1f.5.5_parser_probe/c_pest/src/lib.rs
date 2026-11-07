#![forbid(unsafe_code)]
use common::{Err,Node}; use pest::{Parser,iterators::Pair}; use pest_derive::Parser;
#[derive(Parser)] #[grammar="ein.pest"] pub struct P;
fn node<'a>(p:Pair<'a,Rule>)->Node<'a>{ let o=p.as_span().start() as u32; match p.as_rule(){
 Rule::var=>Node::Var(p.as_str(),o), Rule::text=>Node::Str(p.as_str(),o),
 Rule::program|Rule::relation|Rule::rule|Rule::hrule|Rule::params|Rule::hrules|Rule::using_and|Rule::query|Rule::solve|Rule::names|Rule::config|Rule::trace|Rule::step|Rule::macro_|Rule::macro_params|Rule::import|Rule::not_fact|Rule::fact|Rule::fact_term|Rule::and|Rule::or|Rule::not|Rule::neq|Rule::pattern
   =>Node::List(p.into_inner().filter(|x|x.as_rule()!=Rule::EOI).map(node).collect()),
 r=>{ let s=p.as_str(); if s.starts_with(':'){Node::Kw(s,o)} else { let _=r; Node::Name(s,o)} } } }
pub fn parse(s:&str)->Result<Vec<Node<'_>>,Err>{
  match P::parse(Rule::program,s){
   Ok(mut it)=>{ let p=it.next().unwrap(); match node(p){Node::List(v)=>Ok(v),_=>unreachable!()} }
   Err(e)=>{ let offset=match e.location{pest::error::InputLocation::Pos(p)=>p,pest::error::InputLocation::Span((a,_))=>a};
     let expected=match &e.variant{pest::error::ErrorVariant::ParsingError{positives,..}=>positives.iter().map(|r|format!("{r:?}")).collect(),_=>vec![]};
     Err(Err{offset,expected,msg:String::new()}) } } }
