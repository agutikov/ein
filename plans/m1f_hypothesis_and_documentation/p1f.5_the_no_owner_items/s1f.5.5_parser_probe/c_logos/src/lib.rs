#![forbid(unsafe_code)]
#![allow(non_camel_case_types)]
use common::{Err,Node}; use lalrpop_util::{lalrpop_mod,ParseError}; use logos::Logos;
#[derive(logos::Logos,Debug,Clone,Copy,PartialEq)]
#[logos(skip r"[\s\u{1c}-\u{1f}]+")]
#[logos(skip(r";[^\n]*",allow_greedy=true))]
#[logos(skip(r"#\|([^|]|\|+[^|#])*\|+#", priority=20))]
pub enum Tok<'a>{
  #[token("(",priority=30)] LP,
  #[token(")",priority=30)] RP,
  #[token("not",priority=30)] W_not,
  #[token("and",priority=30)] W_and,
  #[token("or",priority=30)] W_or,
  #[token("neq",priority=30)] W_neq,
  #[token("rule",priority=30)] W_rule,
  #[token("hrule",priority=30)] W_hrule,
  #[token("query",priority=30)] W_query,
  #[token("solve",priority=30)] W_solve,
  #[token("config",priority=30)] W_config,
  #[token("trace",priority=30)] W_trace,
  #[token("macro",priority=30)] W_macro,
  #[token("import",priority=30)] W_import,
  #[token("relation",priority=30)] W_relation,
  #[token("step",priority=30)] W_step,
  #[token(":why",priority=30)] K_why,
  #[token(":source",priority=30)] K_source,
  #[token(":goal-text",priority=30)] K_goal_text,
  #[token(":match",priority=30)] K_match,
  #[token(":assert",priority=30)] K_assert,
  #[token(":priority",priority=30)] K_priority,
  #[token(":goal",priority=30)] K_goal,
  #[token(":expect",priority=30)] K_expect,
  #[token(":over",priority=30)] K_over,
  #[token(":no-hypothesis",priority=30)] K_no_hypothesis,
  #[token(":hrules",priority=30)] K_hrules,
  #[token(":hypothesis-relations",priority=30)] K_hypothesis_relations,
  #[token(":symbols",priority=30)] K_symbols,
  #[token(":as",priority=30)] K_as,
  #[token(":rule",priority=30)] K_rule,
  #[token(":using",priority=30)] K_using,
  #[token(":derives",priority=30)] K_derives,
  #[token("_",priority=30)] Reserved,
  #[regex(r"[0-9]+",slice,allow_greedy=true,priority=25)] Digits(&'a str),
  #[regex(r"[!$%&'*+,\-./0-9<=>@A-Z\[\\\]^_`a-z{}~]+",slice,allow_greedy=true,priority=10)] Word(&'a str),
  #[regex(r"\?[!$%&'*+,\-./0-9<=>@A-Z\[\\\]^_`a-z{}~]+",slice,allow_greedy=true,priority=10)] Var(&'a str),
  #[regex(r":[!$%&'*+,\-./0-9<=>@A-Z\[\\\]^_`a-z{}~]+",slice,allow_greedy=true,priority=10)] Keyword(&'a str),
  #[regex(r#""(\\[^\n]|[^"\\])*""#,slice,allow_greedy=true,priority=10)] Text(&'a str),
  #[regex(r#"[^\s();\u{1c}-\u{1f}]+"#,priority=1,allow_greedy=true)] Bad,
  #[regex(r#""(\\[^\n]|[^"\\])*"[^\s();\u{1c}-\u{1f}]+"#,priority=2,allow_greedy=true)] BadText,
}
fn slice<'a>(l:&mut logos::Lexer<'a,Tok<'a>>)->&'a str{l.slice()}
lalrpop_mod!(ein);
pub fn parse(s:&str)->Result<Vec<Node<'_>>,Err>{
  let toks=Tok::lexer(s).spanned().map(|(t,sp)|match t{Ok(t)=>Ok((sp.start,t,sp.end)),Err(_)=>Err(sp.start)});
  ein::ProgramParser::new().parse(s,toks).map_err(|e|match e{
  ParseError::InvalidToken{location}=>Err{offset:location,expected:vec![],msg:"invalid token".into()},
  ParseError::UnrecognizedEof{location,expected}=>Err{offset:location,expected,msg:"eof".into()},
  ParseError::UnrecognizedToken{token:(l,_,_),expected}=>Err{offset:l,expected,msg:String::new()},
  ParseError::ExtraToken{token:(l,_,_)}=>Err{offset:l,expected:vec![],msg:"extra".into()},
  ParseError::User{error}=>Err{offset:error,expected:vec![],msg:"lex".into()} }) }
