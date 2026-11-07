#![forbid(unsafe_code)]
use common::{Err,Node};
fn nc(c:char)->bool{ c.is_ascii_alphanumeric()||"_-.*$%^+/<>=!@&~,'`\\[]{}".contains(c) }
const RES:&[&str]=&["not","and","or","neq","rule","hrule","query","solve","config","trace","macro","import","_"];
peg::parser!{ grammar ein() for str {
 rule ws()=quiet!{ ([c if c.is_whitespace()||('\u{1c}'..='\u{1f}').contains(&c)] / ";" [^'\n']* / "#|" (!"|#" [_])* "|#")* }
 rule sep()=&([c if c.is_whitespace()||('\u{1c}'..='\u{1f}').contains(&c)]/"("/")"/";"/"#|"/![_])
 rule lp()=ws() "(" rule rp()=ws() ")"
 rule word_raw()->&'input str = $([c if nc(c)]+)
 rule kw(s:&'static str)->Node<'input> = ws() p:position!() x:$(":" [c if nc(c)]+) sep() {? if x==s {Ok(Node::Kw(x,p as u32))} else {Err(s)}}
 rule rw(s:&'static str)->Node<'input> = ws() p:position!() x:word_raw() sep() {? if x==s {Ok(Node::Name(x,p as u32))} else {Err(s)}}
 rule name()->Node<'input> = ws() p:position!() x:word_raw() sep() {? if RES.contains(&x) {Err("name")} else {Ok(Node::Name(x,p as u32))}}
 rule head()->Node<'input> = ws() p:position!() x:word_raw() sep() {? if RES.contains(&x)||x=="relation" {Err("name")} else {Ok(Node::Name(x,p as u32))}}
 rule var()->Node<'input> = ws() p:position!() x:$("?" [c if nc(c)]+) sep() {Node::Var(x,p as u32)}
 rule text()->Node<'input> = ws() p:position!() x:$("\"" ("\\" [c if c!='\n'] / [c if c!='"'&&c!='\\'])* "\"") sep() {Node::Str(x,p as u32)}
 rule count()->Node<'input> = ws() p:position!() x:$(['0'..='9']+) sep() {Node::Name(x,p as u32)}
 rule anykw()->Node<'input> = ws() p:position!() x:$(":" [c if nc(c)]+) sep() {Node::Kw(x,p as u32)}
 rule cl(k:&'static str, v:rule<Node<'input>>)->Vec<Node<'input>> = a:kw(k) b:v() {vec![a,b]}
 rule names()->Node<'input> = lp() v:name()* rp() {Node::List(v)}
 rule hrules()->Node<'input> = lp() v:(name()/names())* rp() {Node::List(v)}
 rule using_and()->Node<'input> = lp() a:rw("and") v:name()+ rp() {let mut r=vec![a];r.extend(v);Node::List(r)}
 rule using()->Node<'input> = names()/using_and()
 rule targ()->Node<'input> = var()/term()
 rule params()->Node<'input> = lp() v:var()* rp() {Node::List(v)}
 rule params1()->Node<'input> = lp() v:var()+ rp() {Node::List(v)}
 rule fact_arg()->Node<'input> = name() / fact_term()
 rule fact_term()->Node<'input> = lp() a:name() v:fact_arg()* rp() {let mut r=vec![a];r.extend(v);Node::List(r)}
 rule fact_clause()->Vec<Node<'input>> = cl(":source",<text()>)/cl(":rule",<name()>)/cl(":using",<using()>)
 rule item()->Node<'input> = name()/var()/term()
 rule term()->Node<'input> = and()/or()/not()/neq()/pattern()
 rule and()->Node<'input> = lp() a:rw("and") v:targ()+ rp() {let mut r=vec![a];r.extend(v);Node::List(r)}
 rule or()->Node<'input> = lp() a:rw("or") v:targ()+ rp() {let mut r=vec![a];r.extend(v);Node::List(r)}
 rule not()->Node<'input> = lp() a:rw("not") t:targ() rp() {Node::List(vec![a,t])}
 rule neq()->Node<'input> = lp() a:rw("neq") x:item() y:item() rp() {Node::List(vec![a,x,y])}
 rule pattern()->Node<'input> = lp() a:(name()/var()) v:item()* rp() {let mut r=vec![a];r.extend(v);Node::List(r)}
 rule rule_clause()->Vec<Node<'input>> = cl(":match",<term()>)/cl(":assert",<term()>)/cl(":why",<text()>)/cl(":priority",<count()>)
 rule rulelike(k:&'static str)->Node<'input> = lp() a:rw(k) n:name() p:params() c:rule_clause()* rp() {let mut r=vec![a,n,p];r.extend(c.into_iter().flatten());Node::List(r)}
 rule relation()->Node<'input> = lp() a:rw("relation") n:name() v:name()* c:cl(":why",<text()>)* rp() {let mut r=vec![a,n];r.extend(v);r.extend(c.into_iter().flatten());Node::List(r)}
 rule query_clause()->Vec<Node<'input>> = cl(":goal",<term()>)/cl(":goal-text",<text()>)/cl(":expect",<term()>)/cl(":over",<name()>)/cl(":no-hypothesis",<names()>)
 rule query()->Node<'input> = lp() a:rw("query") c:query_clause()+ rp() {let mut r=vec![a];r.extend(c.into_iter().flatten());Node::List(r)}
 rule solve_clause()->Vec<Node<'input>> = cl(":expect",<term()>)/cl(":no-hypothesis",<names()>)/cl(":hrules",<hrules()>)/cl(":hypothesis-relations",<names()>)
 rule solve()->Node<'input> = lp() a:rw("solve") c:solve_clause()* rp() {let mut r=vec![a];r.extend(c.into_iter().flatten());Node::List(r)}
 rule config()->Node<'input> = lp() a:rw("config") c:(k:anykw() n:name() {vec![k,n]})* rp() {let mut r=vec![a];r.extend(c.into_iter().flatten());Node::List(r)}
 rule step_clause()->Vec<Node<'input>> = cl(":rule",<name()>)/cl(":using",<using()>)/cl(":derives",<fact_term()>)
 rule step()->Node<'input> = lp() a:rw("step") n:name() c:step_clause()* rp() {let mut r=vec![a,n];r.extend(c.into_iter().flatten());Node::List(r)}
 rule trace()->Node<'input> = lp() a:rw("trace") s:step()* rp() {let mut r=vec![a];r.extend(s);Node::List(r)}
 rule macro_()->Node<'input> = lp() a:rw("macro") n:name() p:params1() t:term() rp() {Node::List(vec![a,n,p,t])}
 rule import_clause()->Vec<Node<'input>> = cl(":symbols",<names()>)/cl(":as",<name()>)
 rule import()->Node<'input> = lp() a:rw("import") n:name() c:import_clause()* rp() {let mut r=vec![a,n];r.extend(c.into_iter().flatten());Node::List(r)}
 rule not_fact()->Node<'input> = lp() a:rw("not") t:fact_term() c:fact_clause()* rp() {let mut r=vec![a,t];r.extend(c.into_iter().flatten());Node::List(r)}
 rule fact()->Node<'input> = lp() a:head() v:fact_arg()* c:fact_clause()* rp() {let mut r=vec![a];r.extend(v);r.extend(c.into_iter().flatten());Node::List(r)}
 rule form()->Node<'input> = relation()/rulelike("rule")/rulelike("hrule")/query()/solve()/config()/trace()/macro_()/import()/not_fact()/fact()
 pub rule program()->Vec<Node<'input>> = v:form()* ws() ![_] {v}
}}
pub fn parse(s:&str)->Result<Vec<Node<'_>>,Err>{ ein::program(s).map_err(|e|Err{offset:e.location.offset,expected:e.expected.tokens().map(|t|t.to_string()).collect(),msg:String::new()}) }
