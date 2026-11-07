#![forbid(unsafe_code)]
use common::{Err,Node}; use lalrpop_util::{lalrpop_mod,ParseError};
lalrpop_mod!(ein);
pub struct Parser(ein::ProgramParser);
pub fn new()->Parser{Parser(ein::ProgramParser::new())}
pub fn parse(s:&str)->Result<Vec<Node<'_>>,Err>{ new().go(s) }
impl Parser{ pub fn go<'a>(&self,s:&'a str)->Result<Vec<Node<'a>>,Err>{ self.0.parse(s,s).map_err(|e|match e{
  ParseError::InvalidToken{location}=>Err{offset:location,expected:vec![],msg:"invalid token".into()},
  ParseError::UnrecognizedEof{location,expected}=>Err{offset:location,expected,msg:"eof".into()},
  ParseError::UnrecognizedToken{token:(l,_,_),expected}=>Err{offset:l,expected,msg:String::new()},
  ParseError::ExtraToken{token:(l,_,_)}=>Err{offset:l,expected:vec![],msg:"extra".into()},
  ParseError::User{..}=>Err{offset:0,expected:vec![],msg:"user".into()} }) } }
