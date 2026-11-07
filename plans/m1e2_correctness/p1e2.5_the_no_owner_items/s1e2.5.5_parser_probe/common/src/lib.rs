#![forbid(unsafe_code)]
//! The tree every candidate builds, so that the speed compared is parse + tree.
#[derive(Debug,Clone,PartialEq)]
pub enum Node<'a>{ Name(&'a str,u32), Var(&'a str,u32), Str(&'a str,u32), Kw(&'a str,u32), List(Vec<Node<'a>>) }
#[derive(Debug,Clone)]
pub struct Err { pub offset: usize, pub expected: Vec<String>, pub msg: String }
pub fn count(n:&[Node])->usize{ n.iter().map(|x| match x { Node::List(v)=>1+count(v), _=>1 }).sum() }
/// A canonical text of the tree, for comparing candidates with each other.
pub fn show(n:&[Node],o:&mut String){ for x in n { match x {
  Node::Name(s,_)|Node::Var(s,_)|Node::Str(s,_)|Node::Kw(s,_)=>{o.push_str(s);o.push(' ')}
  Node::List(v)=>{o.push('(');show(v,o);o.push_str(") ")} } } }
pub fn line_col(src:&str,off:usize)->(usize,usize){ let b=&src[..off.min(src.len())]; let l=b.matches('\n').count()+1; let c=b.rsplit('\n').next().unwrap_or("").chars().count()+1; (l,c)}
