#![allow(unused)]
use common::*; use std::{fs,time::Instant,collections::BTreeMap};
type F=for<'a> fn(&'a str)->Result<Vec<Node<'a>>,Err>;
const C:&[(&str,F)]=&[("pest",c_pest::parse),("peg",c_peg::parse),("lalrpop",c_lalrpop::parse),("logos+lalrpop",c_logos::parse),("grmtools",c_grm::parse)];
fn walk(d:&str,o:&mut Vec<String>){ if let Ok(r)=fs::read_dir(d){ for e in r.flatten(){ let p=e.path(); if p.is_dir(){walk(p.to_str().unwrap(),o)} else if p.extension().is_some_and(|x|x=="ein"){o.push(p.to_str().unwrap().into())} } } }
fn leaves<'a>(n:&[Node<'a>],m:&mut std::collections::HashMap<&'a str,u32>,starts:&[u32])->u64{ let mut k=0u64; for x in n { match x { Node::List(v)=>k+=leaves(v,m,starts), Node::Name(s,o)|Node::Var(s,o)|Node::Str(s,o)|Node::Kw(s,o)=>{ let l=m.len() as u32; let id=*m.entry(*s).or_insert(l); let line=starts.partition_point(|a|a<=o) as u64; k+=id as u64+line; } } } k }
fn finish<'a>(src:&str,t:&[Node<'a>])->u64{ let mut starts=vec![0u32]; for (i,b) in src.bytes().enumerate(){ if b==b'\n'{starts.push(i as u32+1)} } let mut m=std::collections::HashMap::new(); leaves(t,&mut m,&starts) }
fn hand(s:&str)->bool{ let mut a=ein_ir::Ast::default(); ein_ir::parse(&mut a,s,None).is_ok() }
fn main(){ let a:Vec<String>=std::env::args().collect(); let root=concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../..");
 match a[1].as_str(){
 "corpus"=>{ let mut fsv=vec![]; for t in ["examples","tests","stdlib","corpus"]{walk(&format!("{root}/{t}"),&mut fsv)} fsv.sort();
   let mut acc:BTreeMap<&str,usize>=BTreeMap::new(); let mut dis=0; let mut hand_acc=0; let mut moved=vec![];
   for f in &fsv { let s=fs::read_to_string(f).unwrap(); let h=hand(&s); if h {hand_acc+=1}
     let mut shows=vec![]; for (n,p) in C { match p(&s){ Ok(t)=>{*acc.entry(n).or_default()+=1; let mut o=String::new(); show(&t,&mut o); shows.push((n,Some(o)))}, Err(_)=>shows.push((n,None)) } }
     let pa=c_parol::accepts(&s).is_ok(); if pa {*acc.entry("parol").or_default()+=1}
     let first=&shows[0].1; if pa!=first.is_some(){ dis+=1; println!("DISAGREE parol {f}: parol {pa}"); } if shows.iter().any(|x|x.1.is_some()!=first.is_some()){ dis+=1; println!("DISAGREE accept {f}: {:?}",shows.iter().map(|x|(x.0,x.1.is_some())).collect::<Vec<_>>()); }
     else if shows.iter().any(|x|x.1!=*first){ dis+=1; println!("DISAGREE tree {f}"); }
     if h && first.is_none(){ let e=c_logos::parse(&s).err().unwrap(); let (l,c)=line_col(&s,e.offset); let line=s.lines().nth(l-1).unwrap_or("");
       moved.push(format!("{}:{l}:{c}: {}",f.trim_start_matches(root),line.trim())); }
     if !h && first.is_some(){ println!("NOW ACCEPTED {f}"); }
   }
   println!("files {}, hand accepts {hand_acc}, candidates accept {acc:?}, disagreements {dis}",fsv.len());
   println!("hand accepts, grammar refuses: {}",moved.len()); for m in moved {println!("  {m}")}
 }
 "speed"=>{ for f in &a[2..] { let s=fs::read_to_string(f).unwrap(); println!("{f}: {} bytes",s.len());
     let n=|ns:f64|format!("{:>9.1} us  {:>7.1} MB/s",ns/1e3,s.len() as f64/ns*1e3);
     let t=|g:&mut dyn FnMut()|{ let mut best=f64::MAX; for _ in 0..7 { let t0=Instant::now(); let mut k=0; while k<40 {g();k+=1} best=best.min(t0.elapsed().as_nanos() as f64/40.0);} best };
     println!("  {:16}{}","hand-written",n(t(&mut ||{assert!(hand(&s))})));
     for (nm,p) in C { println!("  {:16}{}",nm,n(t(&mut ||{ let r=p(&s); if let Ok(t)=&r { std::hint::black_box(finish(&s,t)); } std::hint::black_box(&r);}))) }
     println!("  {:16}{}","parol",n(t(&mut ||{ let r=c_parol::accepts(&s); std::hint::black_box(&r);})));
     let q=c_lalrpop::new(); println!("  {:16}{}","lalrpop reused",n(t(&mut ||{ let r=q.go(&s); std::hint::black_box(&r);})));
     let t0=Instant::now(); let _=c_lalrpop::new(); println!("  lalrpop ProgramParser::new() alone: {:?}",t0.elapsed());
 } }
 "errors"=>{ for s in &a[2..] { println!("--- {s}"); let h=hand(s); println!("  {:16}{}","hand-written",if h{"accepts"}else{"refuses"});
     println!("  {:16}{}","parol",match c_parol::accepts(s){Ok(_)=>"accepts".to_string(),Err(e)=>{let t=e.replace("\n"," ");let i=t.find("at x.ein").or(t.find("LA(1)")).unwrap_or(0);t[i..].chars().take(70).collect()}});
     for (nm,p) in C { match p(s){ Ok(t)=>{let mut o=String::new();show(&t,&mut o);println!("  {:16}accepts {o}",nm)}, Err(e)=>{let (l,c)=line_col(s,e.offset); let mut ex=e.expected.clone(); ex.truncate(6); println!("  {:16}{l}:{c} {} expected {}{:?}",nm,e.msg,e.expected.len(),ex)} } } } }
 "list"=>{ let mut fsv=vec![]; for t in ["examples","tests","stdlib","corpus"]{walk(&format!("{root}/{t}"),&mut fsv)} fsv.sort(); for f in &fsv { let s=fs::read_to_string(f).unwrap(); println!("{} {f}",if c_lalrpop::parse(&s).is_ok(){1}else{0}); } }
 "mutate"=>{ let n:usize=a[2].parse().unwrap(); let mut fsv=vec![]; for t in ["examples","tests","stdlib","corpus"]{walk(&format!("{root}/{t}"),&mut fsv)} fsv.sort();
   let src:Vec<String>=fsv.iter().map(|f|fs::read_to_string(f).unwrap()).filter(|s|s.len()<6000).collect();
   let mut x=0x9E3779B97F4A7C15u64; let mut r=move|m:usize|{x^=x<<13;x^=x>>7;x^=x<<17;(x%(m as u64)) as usize};
   const INS:&[&str]=&["(",")","_","?","?x",":","\"","\"a\""," ","\n",";","#|","|#","1","a","-","=","not","rule",":why",":match","$","*",".","..","\\"];
   let (mut acc,mut dis,mut shown)=(0usize,0usize,0usize);
   for _ in 0..n { let s=&src[r(src.len())]; let idx:Vec<usize>=s.char_indices().map(|c|c.0).collect(); if idx.is_empty(){continue}
     let mut t=s.clone(); for _ in 0..(1+r(2)) { let idx2:Vec<usize>=t.char_indices().map(|c|c.0).collect(); if idx2.is_empty(){break} let i=idx2[r(idx2.len())];
       match r(3){0=>{let c=t[i..].chars().next().unwrap(); t.replace_range(i..i+c.len_utf8(),"")},1=>{t.insert_str(i,INS[r(INS.len())])},_=>{let c=t[i..].chars().next().unwrap(); t.replace_range(i..i+c.len_utf8(),INS[r(INS.len())])}} }
     let mut v:Vec<(&str,bool)>=C.iter().map(|(nm,p)|(*nm,p(&t).is_ok())).collect(); v.push(("parol",c_parol::accepts(&t).is_ok()));
     if v[0].1 {acc+=1}
     if v.iter().any(|y|y.1!=v[0].1){ dis+=1; if shown<14 { shown+=1; let off=C.iter().filter_map(|(_,p)|p(&t).err().map(|e|e.offset)).next().unwrap_or(0); let lo=t.floor_char_boundary(off.saturating_sub(30)); let hi=t.ceil_char_boundary((off+30).min(t.len())); fs::write(format!("{}/dis{shown}.ein",std::env::temp_dir().display()),&t).unwrap(); println!("DIS {:?} near {:?}",v.iter().filter(|y|y.1).map(|y|y.0).collect::<Vec<_>>(),&t[lo..hi]); } }
   }
   println!("mutants {n}, accepted by pest {acc}, disagreements {dis}");
 }
 "deep"=>{ let d:usize=a[3].parse().unwrap(); let s=format!("(p {}a{})","(p ".repeat(d),")".repeat(d));
     match a[2].as_str(){ "hand"=>{println!("hand {}",hand(&s))}, "parol"=>{let r=c_parol::accepts(&s);println!("parol ok={}",r.is_ok());std::mem::forget(r);}, c=>{ let p=C.iter().find(|x|x.0==c).unwrap().1; let r=p(&s); println!("{c} ok={}",r.is_ok()); std::mem::forget(r);} } }
 _=>panic!() } }
