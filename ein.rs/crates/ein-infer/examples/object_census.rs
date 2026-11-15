//! What the blind enumerator takes for an object, over the corpus — the
//! census behind Q-M1f.306.
//!
//! [`ein_infer::hypgen::candidate_objects`] is every name the KB's names
//! index holds, a fact's head among them, and every name of the program
//! ([`ein_infer::invariant::program_names`]), less the names a signature
//! declares, the reserved primitives and what the registry calls a relation
//! or a rule. A head is in the registry when a `(relation …)` declares it or a
//! **file writes a fact of it**; a head only a rule derives is in neither, so
//! it is an object. This census reads every candidate of H at root's fixpoint
//! and marks the ones that name:
//!
//! | set | the candidate names |
//! |---|---|
//! | `K1` | a kernel name (`__closed__`, `__symmetric__`, any other reserved name) the program never writes in an argument position |
//! | `K2` | a kernel name the program also writes in an argument position, as `(imply1 functional __closed__)` does |
//! | `R1` | a relation's name the program never writes in an argument position: the head of a fact only a rule derives |
//! | `R2` | a relation's name (a fact's head at root, or a rule's) that the program also writes in an argument position |
//! | `IN` | a name the main file does not write outside a `(query …)`: one only an imported module writes |
//! | `IR` | nothing: the candidate is **of** a relation the main file does not spell, in a form or in an import's `:symbols` list |
//!
//! `E17` is `K1 ∪ K2 ∪ R1 ∪ R2`, `E54` is `IN ∪ IR`, `any` their union. A
//! program with no `(import …)` has no `IN` and no `IR`; one that has is not
//! read, and the summary counts it as `self-check`.
//!
//! **What R would be without them is computed, by the oracle's own cube.** A
//! node is root's fixpoint with a commitment written in and saturated, whatever
//! else H holds, so R over H less a set J of candidates is exactly the
//! commitments of R that hold no member of J. Where the blind enumerator
//! built H, some candidate is marked and `n` is within `--limit`, the census
//! runs [`ein_infer::region::compute`] on a second load of the program and
//! counts, for each set, the commitments and the states of R that are left.
//! A program past the limit is `over-limit` and no count is given.
//!
//! **Where another rung built H**, an hrule's or the obligations', the marks
//! are of that rung's candidates and no cube is run: the question is the
//! blind enumerator's. For those programs the `alone` columns say what the
//! blind enumerator would propose at root were it asked
//! ([`ein_infer::hypgen::generate_blind`]), and how many of those candidates
//! each of the first four sets marks. That list is not H and decides nothing.
//!
//! ```sh
//! cargo run --release -p ein-infer --example object_census -- \
//!     [--limit N] [--jobs J] [FILE…] > rows.tsv
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()`. One tab-separated row a
//! program, then a summary, each line of it starting `# `. It writes nothing
//! and changes no answer.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, sexpr};
use ein_infer::hypgen::{HypGenStats, generate_blind};
use ein_infer::oblgen::Mode;
use ein_infer::region::{self, Outcome};
use ein_infer::saturator::Session;
use ein_ir::{Ast, Node, NodeId, load, parse};
use rustc_hash::{FxHashMap, FxHashSet};

/// The marked sets, in column order.
const SETS: [&str; N] = ["K1", "K2", "R1", "R2", "E17", "IN", "IR", "E54", "any"];
const N: usize = 9;
const K1: usize = 0;
const K2: usize = 1;
const R1: usize = 2;
const R2: usize = 3;
const E17: usize = 4;
const IN: usize = 5;
const IR: usize = 6;
const E54: usize = 7;
const ANY: usize = 8;
/// The sets whose names a row lists.
const NAMED: [usize; 6] = [K1, K2, R1, R2, IN, IR];
/// The sets the `alone` columns count.
const ALONE: [usize; 4] = [K1, K2, R1, R2];

/// What the main file writes, read before any import is resolved.
#[derive(Default)]
struct MainFile {
    imports: bool,
    /// Every name in any form, an import's `:symbols` list included.
    spelled: BTreeSet<String>,
    /// [`Self::spelled`] less what only a `(query …)` writes.
    names: BTreeSet<String>,
}

fn atoms(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::Atom(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { head, .. } => {
            atoms(ast, head, out);
            for &a in ast.form_args(node) {
                atoms(ast, a, out);
            }
        }
        Node::KwPair { value, .. } => atoms(ast, value, out),
        _ => {}
    }
}

fn main_file(ast: &Ast, forms: &[NodeId]) -> MainFile {
    let mut m = MainFile::default();
    for &f in forms {
        match ast.head_name(f) {
            Some("import") => {
                m.imports = true;
                for &a in ast.form_args(f) {
                    let Node::KwPair { key, value } = ast.node(a) else {
                        continue;
                    };
                    if matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "symbols") {
                        let mut listed = BTreeSet::new();
                        atoms(ast, value, &mut listed);
                        m.names.extend(listed.iter().cloned());
                        m.spelled.extend(listed);
                    }
                }
            }
            Some("query") => atoms(ast, f, &mut m.spelled),
            _ => {
                let mut own = BTreeSet::new();
                atoms(ast, f, &mut own);
                m.names.extend(own.iter().cloned());
                m.spelled.extend(own);
            }
        }
    }
    m
}

/// Every symbol in an argument position of a fact, at any depth.
fn arg_symbols(terms: &Terms, fact: FactId, out: &mut Vec<Symbol>) {
    fn walk(terms: &Terms, v: Value, out: &mut Vec<Symbol>) {
        if let Some(s) = v.as_sym() {
            out.push(s);
        } else if let Some(f) = v.as_fact() {
            for &a in terms.facts.args(f) {
                walk(terms, a, out);
            }
        }
    }
    for &a in terms.facts.args(fact) {
        walk(terms, a, out);
    }
}

/// A name no program binds: the reserved words, the structural primitives,
/// the predicates, and what the loader or the engine makes up.
fn kernel_name(text: &str) -> bool {
    ein_core::terms::RESERVED.contains(&text)
        || ein_core::terms::STRUCTURAL.contains(&text)
        || ein_core::terms::PREDICATES.contains(&text)
        || ein_core::text::is_internal(text)
        || text.starts_with('<')
}

/// A candidate list, each candidate marked.
#[derive(Default)]
struct Marked {
    /// The names each set marks, sorted (`E17`, `E54` and `any` left empty).
    names: [BTreeSet<String>; N],
    /// Each candidate's text, and the sets it is in.
    marks: Vec<(String, [bool; N])>,
}

impl Marked {
    fn count(&self, set: usize) -> usize {
        self.marks.iter().filter(|(_, m)| m[set]).count()
    }
}

/// What the marks are read against: root's fixpoint and the main file.
struct Reader<'a> {
    kb: &'a Kb,
    terms: &'a Terms,
    main: &'a MainFile,
    /// The names of the program: every constant in an argument position.
    written: FxHashSet<Symbol>,
    /// Every relation a rule's `:match` or `:assert` names.
    rule_heads: FxHashSet<Symbol>,
}

impl Reader<'_> {
    fn mark(&self, candidates: &[FactId]) -> Marked {
        let mut out = Marked::default();
        for &c in candidates {
            let mut m = [false; N];
            let rel = self.terms.sym(self.terms.facts.get(c).0).to_string();
            if !self.main.spelled.contains(&rel) {
                m[IR] = true;
                out.names[IR].insert(rel);
            }
            let mut args = Vec::new();
            arg_symbols(self.terms, c, &mut args);
            for s in args {
                let text = self.terms.sym(s).to_string();
                let value = self.written.contains(&s);
                let set = if kernel_name(&text) {
                    Some(if value { K2 } else { K1 })
                } else if self.kb.name_as_head(s).next().is_some() || self.rule_heads.contains(&s) {
                    Some(if value { R2 } else { R1 })
                } else {
                    None
                };
                if let Some(set) = set {
                    m[set] = true;
                    out.names[set].insert(text.clone());
                }
                if !self.main.names.contains(&text) {
                    m[IN] = true;
                    out.names[IN].insert(text);
                }
            }
            m[E17] = m[K1] || m[K2] || m[R1] || m[R2];
            m[E54] = m[IN] || m[IR];
            m[ANY] = m[E17] || m[E54];
            out.marks.push((sexpr(self.terms, c), m));
        }
        out
    }
}

/// One program's row.
struct Row {
    rung: Mode,
    /// H, marked.
    h: Marked,
    /// Where another rung built H: the blind enumerator's own list at root.
    alone: Option<Marked>,
}

impl Row {
    fn blind(&self) -> bool {
        matches!(self.rung, Mode::Blind | Mode::Declined)
    }
}

/// Root's fixpoint and H, each candidate marked.
fn read(path: &Path) -> Result<Row, &'static str> {
    let text = std::fs::read_to_string(path).map_err(|_| "no-load")?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).map_err(|_| "no-load")?;
    let main = main_file(&ast, &forms);
    let mut kb: Kb = load(&mut ast, &mut terms, &forms, path.parent()).map_err(|_| "no-load")?;
    let (root, ()) =
        census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())).ok_or("no-root")?;
    let blind = matches!(root.rung, Mode::Blind | Mode::Declined);
    let alone: Option<Vec<FactId>> = if blind {
        None
    } else {
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut list = Vec::new();
        generate_blind(&mut s, &mut HypGenStats::new(), &mut |f| {
            list.push(f);
            ControlFlow::Continue(())
        })
        .map_err(|_| "no-root")?;
        Some(list)
    };

    let mut rule_heads: FxHashSet<Symbol> = FxHashSet::default();
    for reg in [
        &kb.program().rules,
        &kb.program().hrules,
        &kb.program().obligations,
    ] {
        for (_, rule) in reg.iter() {
            for p in [rule.match_.as_ref(), rule.assert_.as_ref()]
                .into_iter()
                .flatten()
            {
                rule_heads.extend(p.relation_names.iter().copied());
            }
        }
    }
    let reader = Reader {
        kb: &kb,
        terms: &terms,
        main: &main,
        written: ein_infer::invariant::program_names(&kb, &terms, &ast)
            .into_iter()
            .map(|n| n.name)
            .collect(),
        rule_heads,
    };
    let h = reader.mark(&root.proposed);
    if !main.imports && h.count(E54) > 0 {
        return Err("self-check");
    }
    Ok(Row {
        rung: root.rung,
        h,
        alone: alone.map(|list| reader.mark(&list)),
    })
}

/// R's two counts, and what is left of each without each marked set.
struct Cube {
    k_c: u64,
    k_s: usize,
    without: [(u64, usize); N],
}

/// The oracle's cube on a second load, or why there is none.
fn cube(path: &Path, h: &Marked, limit: u32, jobs: usize) -> Result<Cube, &'static str> {
    let (mut kb, mut terms, ast) = census::loaded(path).ok_or("no-load")?;
    let opts = region::Options {
        limit,
        keep_states: false,
        jobs,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    let r = match region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}) {
        Ok(Outcome::Region(r)) => r,
        Ok(Outcome::Refused { .. }) => return Err("over-limit"),
        Err(_) => return Err("refused"),
    };
    let by_text: FxHashMap<&str, &[bool; N]> =
        h.marks.iter().map(|(t, m)| (t.as_str(), m)).collect();
    if r.n() != h.marks.len() {
        return Err("mismatch");
    }
    let mut masks = [0u64; N];
    for (i, c) in r.candidates.iter().enumerate() {
        let Some(m) = by_text.get(c.text.as_str()) else {
            return Err("mismatch");
        };
        for (set, mask) in masks.iter_mut().enumerate() {
            if m[set] {
                *mask |= 1 << i;
            }
        }
    }
    let mut k_c = 0u64;
    let mut states: FxHashSet<u64> = FxHashSet::default();
    let mut left_c = [0u64; N];
    let mut left_s: [FxHashSet<u64>; N] = Default::default();
    for mask in 0..1u64 << r.n() {
        if !r.in_r(mask) {
            continue;
        }
        let class = r.nodes[mask as usize].class;
        k_c += 1;
        states.insert(class);
        for set in 0..N {
            if mask & masks[set] == 0 {
                left_c[set] += 1;
                left_s[set].insert(class);
            }
        }
    }
    let mut without = [(0u64, 0usize); N];
    for set in 0..N {
        without[set] = (left_c[set], left_s[set].len());
    }
    Ok(Cube {
        k_c,
        k_s: states.len(),
        without,
    })
}

fn joined(names: &BTreeSet<String>) -> String {
    if names.is_empty() {
        "-".to_string()
    } else {
        names.iter().cloned().collect::<Vec<_>>().join(",")
    }
}

fn main() {
    let mut limit = region::DEFAULT_LIMIT;
    let mut jobs = 1usize;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--limit" => {
                limit = args.next().and_then(|v| v.parse().ok()).expect("--limit N");
            }
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
    }
    let groups: BTreeMap<String, String> = ein_corpus::corpus()
        .entry
        .into_iter()
        .map(|e| (e.path, e.group))
        .collect();

    let mut header: Vec<String> = ["file", "group", "status", "rung", "n"]
        .map(String::from)
        .to_vec();
    for set in NAMED {
        header.push(format!("{}:names", SETS[set]));
    }
    for set in SETS {
        header.push(format!("{set}:candidates"));
    }
    header.push("alone:n".to_string());
    for set in ALONE {
        header.push(format!("alone:{}:names", SETS[set]));
        header.push(format!("alone:{}:candidates", SETS[set]));
    }
    header.extend(["cube", "k_c", "k_s"].map(String::from));
    for set in SETS {
        header.push(format!("k_c-{set}"));
        header.push(format!("k_s-{set}"));
    }
    println!("{}", header.join("\t"));

    // The summary's tallies.
    let given = files.len();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    // rung → [programs, with a candidate, with a marked candidate]
    let mut by_rung: BTreeMap<&'static str, [usize; 3]> = BTreeMap::new();
    // set → [programs, candidates, in reach, R differs, not reached]
    let mut blind: [[usize; 5]; N] = [[0; 5]; N];
    let mut blind_candidates = 0usize;
    // set → [programs, candidates], of the rung's own H and of the list alone
    let mut elsewhere: [[usize; 2]; N] = [[0; 2]; N];
    let mut alone: [[usize; 2]; N] = [[0; 2]; N];
    let mut alone_lists = [0usize; 2];
    let mut differs: [Vec<String>; N] = Default::default();
    let mut unreached: Vec<String> = Vec::new();

    for path in files {
        let rel = census::relative(&path);
        let group = groups.get(&rel).map_or("-", |g| g.as_str());
        let row = match read(&path) {
            Ok(row) => row,
            Err(why) => {
                *skipped.entry(format!("{why} ({group})")).or_default() += 1;
                println!("{rel}\t{group}\t{why}");
                continue;
            }
        };
        let n = row.h.marks.len();
        let rung = by_rung.entry(row.rung.as_str()).or_default();
        rung[0] += 1;
        if n > 0 {
            rung[1] += 1;
        }
        if row.h.count(ANY) > 0 {
            rung[2] += 1;
        }
        let mut cells = vec![
            rel.clone(),
            group.to_string(),
            "ok".to_string(),
            row.rung.as_str().to_string(),
            n.to_string(),
        ];
        for set in NAMED {
            cells.push(joined(&row.h.names[set]));
        }
        for set in 0..N {
            cells.push(row.h.count(set).to_string());
        }
        match &row.alone {
            Some(list) => {
                cells.push(list.marks.len().to_string());
                if !list.marks.is_empty() {
                    alone_lists[0] += 1;
                    alone_lists[1] += list.marks.len();
                }
                for set in ALONE {
                    cells.push(joined(&list.names[set]));
                    cells.push(list.count(set).to_string());
                }
                for set in [K1, K2, R1, R2, E17] {
                    if list.count(set) > 0 {
                        alone[set][0] += 1;
                        alone[set][1] += list.count(set);
                    }
                }
            }
            None => cells.extend(std::iter::repeat_n("-".to_string(), 1 + 2 * ALONE.len())),
        }
        if !row.blind() {
            for (set, e) in elsewhere.iter_mut().enumerate() {
                if row.h.count(set) > 0 {
                    e[0] += 1;
                    e[1] += row.h.count(set);
                }
            }
            cells.push("not-blind".to_string());
            println!("{}", cells.join("\t"));
            continue;
        }
        blind_candidates += n;
        let answer = if row.h.count(ANY) == 0 {
            Err("no-marked")
        } else {
            cube(&path, &row.h, limit, jobs)
        };
        for set in 0..N {
            if row.h.count(set) == 0 {
                continue;
            }
            blind[set][0] += 1;
            blind[set][1] += row.h.count(set);
            match &answer {
                Ok(c) => {
                    blind[set][2] += 1;
                    if c.without[set].0 != c.k_c {
                        blind[set][3] += 1;
                        differs[set].push(rel.clone());
                    }
                }
                Err(_) => blind[set][4] += 1,
            }
        }
        match answer {
            Ok(c) => {
                cells.extend(["cube".to_string(), c.k_c.to_string(), c.k_s.to_string()]);
                for set in 0..N {
                    cells.push(c.without[set].0.to_string());
                    cells.push(c.without[set].1.to_string());
                }
            }
            Err(why) => {
                if why != "no-marked" {
                    unreached.push(format!("{rel} (n = {n}, {why})"));
                }
                cells.push(why.to_string());
            }
        }
        println!("{}", cells.join("\t"));
    }

    println!("# {given} files");
    for (why, n) in &skipped {
        println!("# not read: {n} {why}");
    }
    println!("# rung\tprograms\twith a candidate\twith a marked candidate");
    for (rung, [programs, with_h, marked]) in &by_rung {
        println!("# {rung}\t{programs}\t{with_h}\t{marked}");
    }
    println!(
        "# where the blind enumerator built H (blind, declined): {blind_candidates} candidates"
    );
    println!(
        "# set\tprograms\tcandidates\tcube within --limit {limit}\tR differs without\tnot reached"
    );
    for (set, [programs, candidates, reached, moved, not]) in blind.iter().enumerate() {
        println!(
            "# {}\t{programs}\t{candidates}\t{reached}\t{moved}\t{not}",
            SETS[set]
        );
    }
    println!("# where another rung built H: that rung's candidates, the same marks, no cube");
    println!("# set\tprograms\tcandidates");
    for (set, [programs, candidates]) in elsewhere.iter().enumerate() {
        println!("# {}\t{programs}\t{candidates}", SETS[set]);
    }
    println!(
        "# where another rung built H: the blind enumerator alone, were it asked, proposes \
         a candidate on {} programs, {} candidates. It is not H",
        alone_lists[0], alone_lists[1]
    );
    println!("# set\tprograms\tcandidates");
    for set in [K1, K2, R1, R2, E17] {
        println!("# {}\t{}\t{}", SETS[set], alone[set][0], alone[set][1]);
    }
    for (set, files) in differs.iter().enumerate() {
        for f in files {
            println!("# R differs without {}: {f}", SETS[set]);
        }
    }
    for f in &unreached {
        println!("# not reached: {f}");
    }
}
