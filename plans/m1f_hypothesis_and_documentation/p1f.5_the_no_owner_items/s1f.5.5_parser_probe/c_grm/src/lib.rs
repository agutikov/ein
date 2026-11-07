#![forbid(unsafe_code)]
use common::{Err,Node}; use lrlex::lrlex_mod; use lrpar::{lrpar_mod,LexError,LexParseError,Lexeme,NonStreamingLexer};
lrlex_mod!("ein.l"); lrpar_mod!("ein.y");
pub fn parse(s:&str)->Result<Vec<Node<'_>>,Err>{ let d=ein_l::lexerdef(); let lx=d.lexer(s); let (r,e)=ein_y::parse(&lx);
  if let Some(x)=e.first(){ let off=match x{LexParseError::LexError(l)=>l.span().start(),LexParseError::ParseError(p)=>p.lexeme().span().start(),_=>0}; let _=lx.span_str(lrpar::Span::new(0,0)); return Err(Err{offset:off,expected:vec![],msg:String::new()}); }
  r.ok_or(Err{offset:0,expected:vec![],msg:"none".into()}) }
