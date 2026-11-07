//! `c_ein corpus ROOT` — every `.ein` under ROOT's `examples/`, `tests/`,
//! `stdlib/` and `corpus/` (and `plans/` with `--plans`), read by today's
//! parser and by the final grammar: accept, refuse, and for a file both
//! accept, the two trees compared node for node and `Loc` for `Loc`.
//! `c_ein check FILE…` — the final grammar's report on each file, every error
//! recovery reaches. `c_ein dump FILE…` — the tree it builds, as `ein-ir`'s
//! dumper prints it.
use c_ein::parse;
use ein_ir::{Ast, Node, NodeId, dump_compact};
use std::{collections::BTreeMap, fs};

fn walk(d: &str, o: &mut Vec<String>) {
    if let Ok(r) = fs::read_dir(d) {
        for e in r.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(p.to_str().unwrap(), o)
            } else if p.extension().is_some_and(|x| x == "ein") {
                o.push(p.to_str().unwrap().into())
            }
        }
    }
}

/// Every leaf of `n` that today's lexer reads as no single token: a name, a
/// variable or a keyword only the name rule makes.
fn new_names(a: &Ast, n: NodeId, out: &mut Vec<String>) {
    let sym = |t: &str| {
        let t = t.strip_prefix("__").unwrap_or(t);
        let mut c = t.chars();
        c.next().is_some_and(|x| x.is_ascii_alphabetic()) && c.all(|x| x.is_ascii_alphanumeric() || "_*.-".contains(x))
    };
    let int = |t: &str| { let t = t.strip_prefix('-').unwrap_or(t); !t.is_empty() && t.chars().all(|x| x.is_ascii_digit()) };
    match a.node(n) {
        Node::Atom(s) => {
            let t = a.sym(s);
            if !(sym(t) || int(t) || t == "=" || t.starts_with('@')) { out.push(t.to_string()) }
        }
        Node::Var(s) => {
            let t = a.sym(s);
            let mut c = t.chars();
            if !(c.next().is_some_and(|x| x.is_ascii_alphabetic()) && c.all(|x| x.is_ascii_alphanumeric() || "_*-".contains(x))) { out.push(format!("?{t}")) }
        }
        Node::KwPair { key, value } => { new_names(a, key, out); new_names(a, value, out) }
        Node::SForm { head, args } => { new_names(a, head, out); for &x in a.args(args) { new_names(a, x, out) } }
        _ => {}
    }
}

/// The first place two trees differ, and what kind of difference it is.
fn diff(a: &Ast, x: NodeId, b: &Ast, y: NodeId) -> Option<(String, String)> {
    let here = || format!("{} vs {}", dump_compact(a, x), dump_compact(b, y));
    if a.loc(x) != b.loc(y) {
        return Some(("loc".into(), format!("{:?} vs {:?} at {}", a.loc(x), b.loc(y), here())));
    }
    match (a.node(x), b.node(y)) {
        (Node::Atom(s), Node::Atom(t)) | (Node::Var(s), Node::Var(t)) | (Node::Keyword(s), Node::Keyword(t))
        | (Node::Str(s), Node::Str(t)) | (Node::Int(s), Node::Int(t)) => {
            (a.sym(s) != b.sym(t)).then(|| ("leaf".into(), here()))
        }
        // With C_EIN_DIGITS=same, an int and the name of its digits compare
        // alike, so the walk goes on and finds any other difference.
        (Node::Int(s), Node::Atom(t)) => {
            let same = std::env::var("C_EIN_DIGITS").as_deref() == Ok("same")
                && ein_ir::ast::canonical_int(b.sym(t)) == a.sym(s);
            (!same).then(|| ("digits are a name".into(), here()))
        }
        (Node::KwPair { key: k1, value: v1 }, Node::KwPair { key: k2, value: v2 }) => {
            diff(a, k1, b, k2).or_else(|| diff(a, v1, b, v2))
        }
        (Node::SForm { head: h1, args: a1 }, Node::SForm { head: h2, args: a2 }) => {
            if let Some(d) = diff(a, h1, b, h2) {
                return Some(d);
            }
            let (l1, l2) = (a.args(a1), b.args(a2));
            if l1.len() != l2.len() {
                return Some(("arity".into(), here()));
            }
            l1.iter().zip(l2).find_map(|(&p, &q)| diff(a, p, b, q))
        }
        _ => Some(("shape".into(), here())),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("corpus") => {
            let root = &a[2];
            let mut tops = vec!["examples", "tests", "stdlib", "corpus"];
            if a.iter().any(|x| x == "--plans") {
                tops.push("plans");
            }
            let mut files = vec![];
            for t in &tops {
                walk(&format!("{root}/{t}"), &mut files)
            }
            files.sort();
            let mut n: BTreeMap<&str, usize> = BTreeMap::new();
            let mut lines: Vec<String> = vec![];
            for f in &files {
                let rel = f.strip_prefix(&format!("{root}/")).unwrap_or(f);
                let s = fs::read_to_string(f).unwrap();
                let mut old = Ast::new();
                let o = ein_ir::parse(&mut old, &s, Some(rel));
                let (new, r) = parse(Ast::new(), &s, Some(rel));
                let k = match (&o, &r) {
                    (Ok(of), Ok(nf)) => {
                        let d = if of.len() != nf.len() {
                            Some(("form count".to_string(), String::new()))
                        } else {
                            of.iter().zip(nf).find_map(|(&x, &y)| diff(&old, x, &new, y))
                        };
                        match d {
                            None => "both accept, same tree",
                            Some((kind, what)) => {
                                lines.push(format!("TREE {rel}: {kind}: {what}"));
                                if kind == "digits are a name" { "both accept, digits read as names" } else { "both accept, trees differ" }
                            }
                        }
                    }
                    (Ok(_), Err(e)) => {
                        let line = s.lines().nth(e[0].line.saturating_sub(1)).unwrap_or("").trim();
                        let first = e[0].text.split(": ").last().unwrap_or("");
                        lines.push(format!("REFUSED {rel}:{}:{}: {line}\n      {}{}", e[0].line, e[0].col, e[0].text.lines().next().unwrap_or(""), if e.len() > 1 { format!(" (+{} more)", e.len() - 1) } else { String::new() }));
                        let _ = first;
                        "today accepts, the grammar refuses"
                    }
                    (Err(_), Ok(_)) => {
                        lines.push(format!("ACCEPTED {rel}"));
                        "today refuses, the grammar accepts"
                    }
                    (Err(_), Err(_)) => "both refuse",
                };
                *n.entry(k).or_default() += 1;
            }
            println!("{} files under {}", files.len(), tops.join(", "));
            for (k, v) in &n {
                println!("  {v:4}  {k}");
            }
            for l in lines {
                println!("{l}");
            }
        }
        Some("check") => {
            for f in &a[2..] {
                let s = fs::read_to_string(f).unwrap();
                let (_, r) = parse(Ast::new(), &s, Some(f));
                match r {
                    Ok(forms) => println!("{f}: parses, {} forms", forms.len()),
                    Err(e) => {
                        println!("{f}: refused, {} error(s)", e.len());
                        for x in e {
                            println!("  {}:{}: {}", x.line, x.col, x.text);
                        }
                    }
                }
            }
        }
        Some("dump") => {
            for f in &a[2..] {
                let s = fs::read_to_string(f).unwrap();
                let (ast, r) = parse(Ast::new(), &s, Some(f));
                match r {
                    Ok(forms) => forms.iter().for_each(|&x| println!("{}", dump_compact(&ast, x))),
                    Err(e) => e.iter().for_each(|x| println!("{f}:{}:{}: {}", x.line, x.col, x.text)),
                }
            }
        }
        Some("mutate") => {
            // N seeded mutants of the corpus (one to three edits each, the
            // probe harness's edit alphabet), each read by both parsers: where
            // both accept, the trees compared; where one refuses, the token it
            // refused at, counted.
            let root = &a[2];
            let n: usize = a[3].parse().unwrap();
            let mut files = vec![];
            for t in ["examples", "tests", "stdlib", "corpus"] {
                walk(&format!("{root}/{t}"), &mut files)
            }
            files.sort();
            let src: Vec<String> = files.iter().map(|f| fs::read_to_string(f).unwrap()).filter(|s| s.len() < 6000).collect();
            let mut x = 0x9E37_79B9_7F4A_7C15u64;
            let mut r = move |m: usize| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                (x % (m as u64)) as usize
            };
            const INS: &[&str] = &["(", ")", "_", "?", "?x", ":", "\"", "\"a\"", " ", "\n", ";", "#|", "|#", "1", "a", "-", "=", "not", "rule", ":why", ":match", "$", "*", ".", "..", "\\", "?_", "01", ":k v", "(and)", "()"];
            let mut tally: BTreeMap<String, usize> = BTreeMap::new();
            let mut examples: BTreeMap<String, String> = BTreeMap::new();
            for _ in 0..n {
                let s = &src[r(src.len())];
                let mut t = s.clone();
                for _ in 0..(1 + r(3)) {
                    let idx: Vec<usize> = t.char_indices().map(|c| c.0).collect();
                    if idx.is_empty() {
                        break;
                    }
                    let i = idx[r(idx.len())];
                    let c = t[i..].chars().next().unwrap();
                    match r(3) {
                        0 => t.replace_range(i..i + c.len_utf8(), ""),
                        1 => t.insert_str(i, INS[r(INS.len())]),
                        _ => t.replace_range(i..i + c.len_utf8(), INS[r(INS.len())]),
                    }
                }
                let mut old = Ast::new();
                let o = ein_ir::parse(&mut old, &t, None);
                let (new, nr) = parse(Ast::new(), &t, None);
                let k = match (&o, &nr) {
                    (Ok(of), Ok(nf)) => {
                        let d = if of.len() != nf.len() { Some(("form count".to_string(), String::new())) } else { of.iter().zip(nf).find_map(|(&p, &q)| diff(&old, p, &new, q)) };
                        match d {
                            None => "both accept: same tree".to_string(),
                            Some((kind, what)) => {
                                let k = format!("both accept: {kind}");
                                examples.entry(k.clone()).or_insert(what);
                                k
                            }
                        }
                    }
                    (Ok(_), Err(e)) => {
                        let line = t.lines().nth(e[0].line.saturating_sub(1)).unwrap_or("");
                        let tok: String = line.chars().skip(e[0].col.saturating_sub(1)).take_while(|c| !c.is_whitespace() && *c != ')').collect();
                        let k = format!("today accepts, the grammar refuses at {}", if tok.is_empty() { "<end>".into() } else if tok.starts_with('"') { "a string".into() } else if tok.starts_with('(') { "a form".into() } else { tok.clone() });
                        examples.entry(k.clone()).or_insert(line.trim().chars().take(90).collect());
                        k
                    }
                    (Err(_), Ok(nf)) => {
                        // Why the grammar accepts it: a leaf today's lexer has no
                        // token for, which the name rule reads as a name.
                        let mut odd: Vec<String> = vec![];
                        for &f in nf {
                            new_names(&new, f, &mut odd);
                        }
                        let k = if odd.is_empty() { "today refuses, the grammar accepts: no new name".to_string() } else { "today refuses, the grammar accepts: a name under the name rule".to_string() };
                        examples.entry(k.clone()).or_insert_with(|| odd.first().cloned().unwrap_or_else(|| t.chars().take(90).collect()));
                        k
                    }
                    (Err(_), Err(_)) => "both refuse".to_string(),
                };
                // C_EIN_EXAMPLES=DIR: the first mutant of each kind, as a file.
                if let Ok(dir) = std::env::var("C_EIN_EXAMPLES")
                    && !tally.contains_key(&k)
                {
                    let name: String = k.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
                    let _ = fs::create_dir_all(&dir);
                    let _ = fs::write(format!("{dir}/{name}.ein"), &t);
                }
                *tally.entry(k).or_default() += 1;
            }
            println!("{n} mutants of {} corpus files under 6 000 bytes", src.len());
            let mut v: Vec<_> = tally.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            for (k, c) in v {
                println!("  {c:6}  {k}{}", examples.get(&k).map(|e| format!("   e.g. {e}")).unwrap_or_default());
            }
        }
        _ => eprintln!("c_ein corpus ROOT [--plans] | check FILE… | dump FILE… | mutate ROOT N"),
    }
}
