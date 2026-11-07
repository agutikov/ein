//! grmtools with the grammar read at run time: the probe's own `ein.l` and `ein.y`, no build step.
//! `corpus`: accept/reject against the compile-time c_grm on every corpus file.
//! `time FILE`: grammar build (once per run) and parse, best of 7.
//! `recover FILE`: the CPCT+ report, every error with its repairs.
#![forbid(unsafe_code)]
use cfgrammar::yacc::YaccGrammar;
use lrlex::{DefaultLexerTypes, LRNonStreamingLexerDef, LexerDef};
use lrpar::{RTParserBuilder, RecoveryKind};
use lrtable::{from_yacc, Minimiser, StateTable};
use std::{collections::HashMap, fs, time::Instant};

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../c_grm/src");
const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../..");

struct G { grm: YaccGrammar<u32>, st: StateTable<u32>, lx: LRNonStreamingLexerDef<DefaultLexerTypes<u32>> }

fn build() -> G {
    let y = fs::read_to_string(format!("{DIR}/ein.y")).unwrap();
    let l = fs::read_to_string(format!("{DIR}/ein.l")).unwrap();
    let grm: YaccGrammar<u32> = y.parse().map_err(|e| format!("{e:?}")).unwrap();
    let (_, st) = from_yacc(&grm, Minimiser::Pager).unwrap();
    let mut lx = LRNonStreamingLexerDef::<DefaultLexerTypes<u32>>::from_str(&l).map_err(|e| format!("{e:?}")).unwrap();
    let ids: HashMap<&str, u32> = grm.tokens_map().iter().map(|(&n, &i)| (n, u32::from(i))).collect();
    lx.set_rule_ids(&ids);
    G { grm, st, lx }
}

// `parse_noaction` is deprecated in favour of `parse_map`; it is the plain "errors only" call this needs.
#[allow(deprecated)]
fn errors(g: &G, s: &str, rk: RecoveryKind, show: bool) -> usize {
    let lexer = g.lx.lexer(s);
    let errs = RTParserBuilder::new(&g.grm, &g.st).recoverer(rk).parse_noaction(&lexer);
    if show { for e in &errs { println!("  {}", e.pp(&lexer, &|t| g.grm.token_epp(t))); } }
    errs.len()
}

fn walk(d: &str, o: &mut Vec<String>) {
    if let Ok(r) = fs::read_dir(d) { for e in r.flatten() { let p = e.path();
        if p.is_dir() { walk(p.to_str().unwrap(), o) } else if p.extension().is_some_and(|x| x == "ein") { o.push(p.to_str().unwrap().into()) } } }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "corpus" => {
            let g = build(); let mut fsv = vec![];
            for t in ["examples", "tests", "stdlib", "corpus"] { walk(&format!("{ROOT}/{t}"), &mut fsv) }
            let (mut acc, mut dis) = (0, 0);
            for f in &fsv { let s = fs::read_to_string(f).unwrap();
                let rt = errors(&g, &s, RecoveryKind::None, false) == 0; let ct = c_grm::parse(&s).is_ok();
                if rt { acc += 1 }
                if rt != ct { dis += 1; println!("DISAGREE {f}: run-time {rt}, compile-time {ct}") } }
            println!("{} files, run-time accepts {acc}, {dis} disagreements with compile-time grmtools", fsv.len());
        }
        "time" => {
            let s = fs::read_to_string(&a[2]).unwrap();
            let (mut b, mut p) = (u128::MAX, u128::MAX);
            for _ in 0..7 { let t = Instant::now(); let g = build(); b = b.min(t.elapsed().as_micros());
                let t = Instant::now(); let n = errors(&g, &s, RecoveryKind::None, false); p = p.min(t.elapsed().as_micros()); assert_eq!(n, 0); }
            let g = build(); let mut lx = u128::MAX;
            for _ in 0..7 { let t = Instant::now(); let l = g.lx.lexer(&s); std::hint::black_box(&l); lx = lx.min(t.elapsed().as_micros()); }
            println!("grammar and tables built at run time: {b} µs; lex and parse: {p} µs, of which lexing {lx} µs ({} bytes)", s.len());
        }
        "recover" => {
            let g = build();
            for f in &a[2..] { let s = fs::read_to_string(f).unwrap(); println!("--- {f}");
                let n = errors(&g, &s, RecoveryKind::CPCTPlus, true); println!("  {n} error(s) reported in one run");
                let t = Instant::now(); errors(&g, &s, RecoveryKind::CPCTPlus, false); println!("  recovery run: {} µs", t.elapsed().as_micros()); }
        }
        _ => panic!("corpus | time FILE | recover FILE…"),
    }
}
