//! The pair layer, measured: what one candidate and two decide at root, over
//! the corpus, in process, beside [`state_reach`](state_reach.rs) and
//! [`route1_reach`](route1_reach.rs).
//!
//! Every corpus file and register witness that loads (or the `FILE`s given),
//! through [`ein_infer::structure::pair_layer`] at a budget: one JSON line per
//! program, and the lists themselves, as rendered text, in
//! `<out>/<program>/pairs.json`, where `<program>` is the file's path from
//! the repo root without its `.ein`.
//!
//! ```sh
//! cargo run --release -p ein-infer --example pair_layer -- \
//!     [--facts F] [--jobs J] [--routes] [--theory | --supports | --check] \
//!     [--rule NAME] [--out DIR] [FILE…] > rows.jsonl
//! ```
//!
//! `--facts` is the budget, in facts beyond root summed over the layer's
//! fixpoints, and defaults to the routes' default; `--jobs` is the node
//! function's fan-out, and no line depends on it but in `wall`. `--out`
//! defaults to `build/structure` under the repo root. With no `FILE` it
//! sweeps `corpus_files()` plus the register's witnesses.
//!
//! A line's names: `n`; `monotone`, the static verdict, and under a flagged
//! program `domain`, the sentence that says what the layer is there;
//! `licences`, those that decided a pair without its node; `at_root`,
//! `dead_alone`, `implications`, `equivalence_classes` (and
//! `equivalent_candidates`, the candidates in them); `pairs`, by what decided
//! each (`all`, `dead_member`, `implied`, `saturated`, `left`) and by what a
//! saturated pair is (`excluded`, `interacting`); `open_by_class`, the pairs
//! neither licence decides counted once per pair of single states, which is
//! what L3 read over the equivalence classes would leave of them (not read
//! while `singles_left` is not 0); `in_excluded`, the candidates in at least one
//! excluded pair, and the two shares, `share_excluded` of all pairs and
//! `share_in_excluded` of all candidates; `against_l1` and `against_l3`, a
//! flagged program's pairs the licences would have read wrongly;
//! `saturations` and `facts`, the layer's work in the read-out's units, root's
//! node counted as the routes count it; `stopped` and `singles_left`; `wall`,
//! root's saturation included; `lists`, the file written.
//!
//! `--routes` asks the same program, loaded again each time, at the same
//! budget, a describe, a `--single` and a `--maximal`, and adds each one's
//! `saturations`, `facts`, `wall` and whether it finished under `routes`;
//! and `searches`, whether the program's describe saturates more than root at
//! the determinism sweeps' budget and cube limit, as
//! `ein-render/tests/corpus_ops`' `searches` reads it.
//!
//! ### The theory's share
//!
//! `--theory` reads each program through [`ein_infer::structure::theory`]:
//! root's layer, and the layer of the **small KB**, the program with the
//! puzzle's statements taken out, over the same candidates. `--supports`
//! also reads the support of every pair root excludes, the stated facts its
//! death rests on, and `--check` checks each one. The line gets a `theory`
//! object and the program's directory three files more:
//!
//! - `theory.json`, the lists raw: every fact the program states with
//!   whether the small KB keeps it and by which clause of the rule, the
//!   candidates the theory kills alone, the pairs it excludes, and each pair
//!   root excludes with whether it is the theory's and its support;
//! - `theory.md`, the same as tables;
//! - `theory.ein`, **the small KB as a program**: the file's text with every
//!   form deleted that writes a fact the small KB drops. It is for review,
//!   and it is checked: `theory_ein.agrees` says the loader reads it to the
//!   facts the engine built the small KB from. A fact a module writes cannot
//!   be deleted from the program's text; where the small KB drops one the
//!   file loads to the small KB with the module's facts beside it, and
//!   `same_layer_with_the_modules_facts` says that program's layer is the
//!   small KB's.
//!
//! `theory`'s names: `rung`, the rung of the ladder that built `H`;
//! `stated` and `dropped`, the facts the program's files write and those of
//! them the small KB drops, by `clauses`; `rules_naming_objects`, the rules
//! whose text names one of the candidates' objects, which the small KB keeps
//! as it keeps every rule; `small`, the small KB's layer in the names of the
//! line's own, its `saturations` and `facts` in its own root's terms, or its
//! `error`; `small_beyond_root`, facts its fixpoint holds that root's does
//! not; `theory_dead_pairs`, the pairs of `H` dead in the small KB, by
//! exclusion or with a member it kills alone; `theory_over_dead_member`, the
//! pairs the theory excludes a member of which is dead alone at root;
//! `root_excluded` and `root_excluded_of_theory`; `against_root`, what is
//! dead in the small KB and alive at root; `lines`, the pairs of two
//! candidates of one relation of two names that share exactly one, a
//! syntactic stand-in for a row or a column that nothing decides by, counted
//! at root and in the small KB; `supports`, how many were `read` and `cut`
//! at the search's budget, the four ways the two readings of a pair meet
//! (`theory_both`, `theory_by_another_way`, `puzzle_both`, `disagree`), the
//! `check`'s outcomes and its `work`; `theory_ein`.
//!
//! `--rule NAME` reads the small KB by another rule than the owner's,
//! `small-kb`: `proposed`, `objects`, `relates`, `layout` or `h-alone`, the
//! alternatives the corpus falsified, kept here so that each falsification
//! can be taken again.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use ein_core::entities::NameCategory;
use ein_core::{FactId, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::region::Cores;
use ein_infer::structure::{self, Deaths, Origin, PairLayer, Stated, Theory, Verdict};
use ein_ir::{Ast, load, parse};
use rustc_hash::FxHashSet;
use serde_json::{Value, json};

/// The determinism sweeps' budget and cube limit, `corpus_ops`' `SWEEP_BUDGET`
/// and `SWEEP_LIMIT`: what *searches* is read at.
const SWEEP_BUDGET: u64 = 1 << 14;
const SWEEP_LIMIT: u32 = 10;

fn loaded(path: &Path) -> Option<(ein_core::Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn stopped(s: &Option<Stopped>) -> Value {
    match s {
        None => Value::Null,
        Some(Stopped::Budget { spent, projected }) => {
            json!({"why": "budget", "spent": spent, "projected": projected})
        }
        Some(Stopped::Limit { n, limit }) => json!({"why": "limit", "n": n, "limit": limit}),
        Some(Stopped::Deadline) => json!({"why": "deadline"}),
        Some(Stopped::NoSearch) => json!({"why": "no-search"}),
    }
}

/// One question about `R`, asked of the program loaded again: its work, and
/// whether it finished.
fn route(path: &Path, mode: Mode, facts: u64, jobs: usize) -> Value {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return Value::Null;
    };
    let opts = explore::Options {
        budget: facts,
        jobs,
        cores: Cores::READ_OUT,
        nogoods: false,
        ..explore::Options::new(mode)
    };
    let t = Instant::now();
    match explore::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(a) => json!({
            "saturations": a.saturations,
            "facts": a.facts,
            "finished": a.stopped.is_none(),
            "exhausted": a.exhausted,
            "verdict": a.verdict.map(|v| v.word()),
            "stopped": stopped(&a.stopped),
            "wall": t.elapsed().as_secs_f64(),
        }),
        Err(e) => json!({"error": e.to_string()}),
    }
}

/// `corpus_ops`' `searches`, by the same call.
fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return false;
    };
    let mut opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    opts.budget = SWEEP_BUDGET;
    opts.limit = SWEEP_LIMIT;
    opts.jobs = 1;
    ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
        .is_ok_and(|r| r.answer.saturations > 1)
}

/// The lists, as rendered text.
fn lists(rel: &str, l: &PairLayer) -> Value {
    let one = |v: &[u32]| -> Vec<&str> { v.iter().map(|&b| l.text(b)).collect() };
    let two = |v: &[(u32, u32)]| -> Vec<[&str; 2]> {
        v.iter().map(|&(a, b)| [l.text(a), l.text(b)]).collect()
    };
    json!({
        "program": rel,
        "n": l.n(),
        "monotone": l.monotone,
        "domain": l.domain(),
        "root_dead": l.root_dead,
        "complete": l.complete(),
        "stopped": stopped(&l.stopped),
        "singles_left": l.singles_left,
        "pairs": {
            "all": l.pairs.all,
            "dead_member": l.pairs.dead_member,
            "implied": l.pairs.implied,
            "saturated": l.pairs.saturated,
            "left": l.pairs.left,
        },
        "work": {"saturations": l.work.saturations, "facts": l.work.facts},
        "candidates": l.candidates.iter().map(|c| c.text.as_str()).collect::<Vec<_>>(),
        "at_root": one(&l.at_root),
        "dead_alone": one(&l.dead_alone),
        "implies": two(&l.implies),
        "equivalent": l.equivalent.iter().map(|c| one(c)).collect::<Vec<_>>(),
        "excluded": two(&l.excluded),
        "interacting": two(&l.interacting),
        "against_l1": two(&l.against_l1),
        "against_l3": l
            .against_l3
            .iter()
            .map(|&(a, b, dead)| json!({"pair": [l.text(a), l.text(b)], "dead": dead}))
            .collect::<Vec<_>>(),
    })
}

fn main() {
    let mut facts = ein_infer::downset::DEFAULT_BUDGET;
    let mut jobs = 1usize;
    let mut routes = false;
    let mut theory = false;
    let mut supports = false;
    let mut check = false;
    let mut rule: Rule = structure::small_kb;
    let mut out = repo_root().join("build/structure");
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--facts" => facts = args.next().and_then(|v| v.parse().ok()).expect("--facts F"),
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--routes" => routes = true,
            "--theory" => theory = true,
            "--supports" => (theory, supports) = (true, true),
            "--check" => (theory, supports, check) = (true, true, true),
            "--rule" => {
                let name = args.next().expect("--rule NAME");
                rule = rule_named(&name).unwrap_or_else(|| panic!("no rule `{name}`"));
                theory = true;
            }
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
        for row in ein_corpus::register().case {
            for (_, a) in row.answers() {
                files.push(repo_root().join(row.witness_of(a)));
            }
        }
        files.sort();
        files.dedup();
    }
    let opts = structure::Options {
        budget: facts,
        jobs,
        deadline: None,
    };
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let dir = out.join(rel.trim_start_matches('/').trim_end_matches(".ein"));
        let t = Instant::now();
        let (l, theory_share) = if theory {
            let asked = structure::TheoryOptions {
                layer: opts,
                root_pairs: true,
                supports,
                check,
            };
            match structure::theory_by(&mut kb, &mut terms, &ast, &asked, &rule) {
                Ok(t) => {
                    std::fs::create_dir_all(&dir).expect("the program's directory");
                    let share = theory_row(&path, &rel, &dir, &t, &terms, (rule, &opts));
                    (t.root, Some(share))
                }
                Err(e) => {
                    println!("{}", json!({"path": rel, "error": e.to_string()}));
                    continue;
                }
            }
        } else {
            match structure::pair_layer(&mut kb, &mut terms, &ast, &opts) {
                Ok(l) => (l, None),
                Err(e) => {
                    println!("{}", json!({"path": rel, "error": e.to_string()}));
                    continue;
                }
            }
        };
        let wall = t.elapsed().as_secs_f64();
        drop((kb, terms, ast));

        // The lists, in a file of the program's own.
        let file = dir.join("pairs.json");
        std::fs::create_dir_all(&dir).expect("the program's directory");
        let text = serde_json::to_string_pretty(&lists(&rel, &l)).expect("JSON");
        std::fs::write(&file, text + "\n").expect("pairs.json");

        // The pairs neither licence decides, counted once per pair of single
        // states: two equivalent candidates have one state, so by L3 their
        // pairs with a third are one node.
        let open_by_class = {
            let n = l.n();
            let mut class: Vec<u32> = (0..n as u32).collect();
            for c in &l.equivalent {
                for &b in c {
                    class[b as usize] = c[0];
                }
            }
            let mut closed = vec![false; n];
            for &b in l.dead_alone.iter().chain(&l.at_root) {
                closed[b as usize] = true;
            }
            let implied: std::collections::HashSet<(u32, u32)> =
                l.implies.iter().copied().collect();
            let mut pairs: std::collections::HashSet<(u32, u32)> = Default::default();
            for a in 0..n as u32 {
                for b in a + 1..n as u32 {
                    if closed[a as usize]
                        || closed[b as usize]
                        || implied.contains(&(a, b))
                        || implied.contains(&(b, a))
                    {
                        continue;
                    }
                    let (x, y) = (class[a as usize], class[b as usize]);
                    pairs.insert((x.min(y), x.max(y)));
                }
            }
            pairs.len()
        };
        let n = l.n() as f64;
        let in_excluded = l.in_an_excluded_pair();
        let share = |x: f64, of: f64| if of > 0.0 { json!(x / of) } else { Value::Null };
        let mut row = json!({
            "path": rel,
            "n": l.n(),
            "monotone": l.monotone,
            "domain": l.domain(),
            "root_dead": l.root_dead,
            "licences": l.licences.iter().map(|x| x.label()).collect::<Vec<_>>(),
            "at_root": l.at_root.len(),
            "dead_alone": l.dead_alone.len(),
            "singles_left": l.singles_left,
            "implications": l.implies.len(),
            "equivalence_classes": l.equivalent.len(),
            "equivalent_candidates": l.equivalent.iter().map(Vec::len).sum::<usize>(),
            "pairs": {
                "all": l.pairs.all,
                "dead_member": l.pairs.dead_member,
                "implied": l.pairs.implied,
                "saturated": l.pairs.saturated,
                "excluded": l.excluded.len(),
                "interacting": l.interacting.len(),
                "left": l.pairs.left,
            },
            "open_by_class": open_by_class,
            "in_excluded": in_excluded,
            "share_excluded": share(l.excluded.len() as f64, l.pairs.all as f64),
            "share_in_excluded": share(in_excluded as f64, n),
            "against_l1": l.against_l1.len(),
            "against_l3": l.against_l3.len(),
            "saturations": l.work.saturations,
            "facts": l.work.facts,
            "stopped": stopped(&l.stopped),
            "wall": wall,
            "lists": file.strip_prefix(repo_root()).unwrap_or(&file).to_string_lossy(),
        });
        if let Some(share) = theory_share {
            row["theory"] = share;
        }
        if routes {
            let ask = |which, ask| route(&path, Mode { which, ask }, facts, jobs);
            row["searches"] = json!(searches(&path));
            row["routes"] = json!({
                "describe": ask(Which::All, Ask::Describe),
                "single": ask(Which::All, Ask::Single),
                "maximal": ask(Which::Maximal, Ask::Describe),
            });
        }
        println!("{row}");
    }
}

// ── The theory's share ────────────────────────────────────────────────

/// A rule for the small KB: [`structure::small_kb`], or an alternative
/// measured against it.
type Rule = fn(&Stated<'_>, FactId) -> Verdict;

fn rule_named(name: &str) -> Option<Rule> {
    match name {
        "small-kb" => Some(structure::small_kb),
        "proposed" => Some(rule_proposed),
        "objects" => Some(rule_objects),
        "layout" => Some(rule_layout),
        "relates" => Some(rule_relates),
        "h-alone" => Some(rule_h_alone),
        _ => None,
    }
}

/// **The rule as first proposed**: a fact of a candidates' relation, and a
/// fact that names such a relation with an object, any name that is no
/// relation's and no rule's, are the puzzle's; every other stated fact is
/// the theory's.
fn rule_proposed(p: &Stated<'_>, fact: FactId) -> Verdict {
    if p.hypothesised.contains(&p.relation(fact)) {
        return Verdict::puzzle("(a) a fact of a candidates' relation");
    }
    let names = p.names(fact);
    let relation = names.iter().any(|n| p.hypothesised.contains(n));
    let object = names.iter().any(|&n| p.category(n) == NameCategory::Object);
    if relation && object {
        return Verdict::puzzle("(b) names a candidates' relation and an object");
    }
    Verdict::theory("kept")
}

/// **By one object**: a stated fact that names one of the candidates'
/// objects is the puzzle's, but for a fact of a relation the rung that built
/// `H` reads.
fn rule_objects(p: &Stated<'_>, fact: FactId) -> Verdict {
    let rel = p.relation(fact);
    if p.hypothesised.contains(&rel) {
        return Verdict::puzzle("(a) a fact of a candidates' relation");
    }
    if p.generator_reads.contains(&rel) {
        return Verdict::theory("a fact of a relation the generator reads");
    }
    if p.names(fact).iter().any(|n| p.objects.contains(n)) {
        return Verdict::puzzle("names one of the candidates' objects");
    }
    Verdict::theory("names none of the candidates' objects")
}

/// **By two objects, under every rung**: a fact that names a candidates'
/// relation with one of the candidates' objects, or two or more of the
/// candidates' objects, is the puzzle's, and a fact that names one object
/// and no candidates' relation is kept, whichever rung built `H`: what
/// [`structure::small_kb`] says under the blind enumerator alone.
fn rule_relates(p: &Stated<'_>, fact: FactId) -> Verdict {
    if p.hypothesised.contains(&p.relation(fact)) {
        return Verdict::puzzle("(a) a fact of a candidates' relation");
    }
    let names = p.names(fact);
    let objects = names.iter().filter(|n| p.objects.contains(n)).count();
    if objects >= 1 && names.iter().any(|n| p.hypothesised.contains(n)) {
        return Verdict::puzzle(
            "(c) names a candidates' relation and one of the candidates' objects",
        );
    }
    if objects >= 2 {
        return Verdict::puzzle("(d) names two or more of the candidates' objects");
    }
    Verdict::theory("kept")
}

/// [`structure::small_kb`] over other sets than the owner hands it.
fn small_kb_over(
    p: &Stated<'_>,
    fact: FactId,
    hypothesised: FxHashSet<Symbol>,
    objects: FxHashSet<Symbol>,
) -> Verdict {
    let other = Stated {
        root: p.root,
        terms: p.terms,
        candidates: p.candidates,
        hypothesised,
        objects,
        read_absent: p.read_absent.clone(),
        declared: p.declared,
        generator_reads: p.generator_reads.clone(),
    };
    structure::small_kb(&other, fact)
}

/// **The candidates' relations read off `H` alone**: [`structure::small_kb`]
/// with a candidates' relation one a candidate of `H` is of, and no relation
/// the rung could still propose.
fn rule_h_alone(p: &Stated<'_>, fact: FactId) -> Verdict {
    let hypothesised: FxHashSet<Symbol> = p.candidates.iter().map(|c| p.relation(c.fact)).collect();
    let mut objects: FxHashSet<Symbol> =
        p.candidates.iter().flat_map(|c| p.names(c.fact)).collect();
    // determinism-ok: builds a set; no order reaches it.
    for &rel in &hypothesised {
        for f in p.root.facts_of(rel) {
            objects.extend(p.names(f));
        }
    }
    small_kb_over(p, fact, hypothesised, objects)
}

/// **The layout kept**: [`structure::small_kb`], but a fact of a relation a
/// rule reads the absence of is the theory's whatever it names.
fn rule_layout(p: &Stated<'_>, fact: FactId) -> Verdict {
    let v = structure::small_kb(p, fact);
    if !v.kept
        && !p.hypothesised.contains(&p.relation(fact))
        && p.read_absent.contains(&p.relation(fact))
    {
        return Verdict::theory("a fact of a relation a rule reads the absence of");
    }
    v
}

/// The end of the form that opens at byte `start` of `text`: the byte after
/// its closing parenthesis, strings and comments read as the lexer reads
/// them. `None` where `start` opens no form.
fn form_end(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(start) != Some(&b'(') {
        return None;
    }
    let (mut depth, mut i) = (0usize, start);
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            b'"' => {
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
            }
            b';' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// The top-level forms of `text`, each as its bytes' range.
fn top_forms(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let (mut out, mut i) = (Vec::new(), 0usize);
    while i < bytes.len() {
        match bytes[i] {
            b';' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'(' => match form_end(text, i) {
                Some(end) => {
                    out.push((i, end));
                    i = end;
                    continue;
                }
                None => break,
            },
            _ => {}
        }
        i += 1;
    }
    out
}

/// A form's tokens: each parenthesis, each string whole, each name.
fn tokens(form: &str) -> Vec<&str> {
    let bytes = form.as_bytes();
    let (mut out, mut i) = (Vec::new(), 0usize);
    while i < bytes.len() {
        let start = i;
        match bytes[i] {
            b'(' | b')' => i += 1,
            b';' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'"' => {
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
            }
            c if c.is_ascii_whitespace() => {
                i += 1;
                continue;
            }
            _ => {
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && !matches!(bytes[i], b'(' | b')' | b'"' | b';')
                {
                    i += 1;
                }
            }
        }
        out.push(&form[start..i.min(bytes.len())]);
    }
    out
}

/// The fact a top-level form writes, as the engine prints one: its relation
/// and its arguments, a keyword and its value left out. `None` for a form
/// that states no fact.
fn fact_of_form(form: &str) -> Option<String> {
    const DECLARATORS: [&str; 9] = [
        "relation", "rule", "hrule", "macro", "import", "query", "solve", "config", "trace",
    ];
    let toks = tokens(form);
    if toks.len() < 3 || DECLARATORS.contains(&toks[1]) {
        return None;
    }
    let (mut out, mut depth, mut i) = (String::new(), 0usize, 0usize);
    while i < toks.len() {
        let t = toks[i];
        if depth == 1 && t.starts_with(':') {
            // The keyword, and its value: one token, or one form.
            i += 1;
            if toks.get(i) == Some(&"(") {
                let mut d = 0usize;
                while i < toks.len() {
                    match toks[i] {
                        "(" => d += 1,
                        ")" => d -= 1,
                        _ => {}
                    }
                    i += 1;
                    if d == 0 {
                        break;
                    }
                }
            } else {
                i += 1;
            }
            continue;
        }
        match t {
            "(" => {
                if !out.is_empty() && !out.ends_with('(') {
                    out.push(' ');
                }
                out.push('(');
                depth += 1;
            }
            ")" => {
                out.push(')');
                depth -= 1;
            }
            _ => {
                if !out.ends_with('(') {
                    out.push(' ');
                }
                out.push_str(t);
            }
        }
        i += 1;
    }
    Some(out)
}

/// The small KB as text: the program's file with every form deleted that
/// writes a fact the small KB drops, and how many dropped facts no form of
/// the file writes: a module's, which this text cannot delete.
fn small_text(path: &Path, rel: &str, t: &Theory) -> Option<(String, usize, BTreeSet<String>)> {
    let text = std::fs::read_to_string(path).ok()?;
    let dropped: BTreeSet<&str> = t
        .stated
        .iter()
        .filter(|s| !s.kept && s.origin == Origin::Fact)
        .map(|s| s.text.as_str())
        .collect();
    let mut written: BTreeSet<String> = BTreeSet::new();
    let mut cuts: Vec<(usize, usize)> = Vec::new();
    for (a, b) in top_forms(&text) {
        if let Some(fact) = fact_of_form(&text[a..b]) {
            if dropped.contains(fact.as_str()) {
                cuts.push((a, b));
            }
            written.insert(fact);
        }
    }
    let elsewhere = dropped.iter().filter(|d| !written.contains(**d)).count();
    let mut out = format!(
        ";; The small KB of {rel}: the program with {} of its forms deleted, each a fact the\n\
         ;; small KB drops, written by `pair_layer --theory`. Every rule, declaration and\n\
         ;; import is the program's.\n",
        cuts.len()
    );
    if elsewhere > 0 {
        out.push_str(&format!(
            ";; The small KB drops {elsewhere} facts more, which the modules this file imports\n\
             ;; write and this text cannot delete: it loads to the small KB with those beside it.\n"
        ));
    }
    let mut at = 0usize;
    for &(a, b) in &cuts {
        out.push_str(&text[at..a]);
        at = b;
    }
    out.push_str(&text[at..]);
    Some((out, cuts.len(), written))
}

/// What the small KB's layer says, for a comparison of two.
fn small_said(t: &Theory) -> String {
    match &t.small {
        Err(e) => e.clone(),
        Ok(l) => format!(
            "{:?}",
            (
                &l.dead_alone,
                &l.implies,
                &l.equivalent,
                &l.excluded,
                &l.interacting
            )
        ),
    }
}

/// The facts `text` loads to, unsaturated, as sorted text: its imports
/// resolved from `base`.
fn loaded_facts(text: &str, name: &Path, base: Option<&Path>) -> Result<Vec<String>, String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, name.to_str()).map_err(|e| e.to_string())?;
    let kb = load(&mut ast, &mut terms, &forms, base).map_err(|e| e.to_string())?;
    let mut facts: Vec<String> = kb.facts().map(|f| terms.fact_text(f)).collect();
    facts.sort();
    Ok(facts)
}

/// Two candidates of one relation of two names that share exactly one of
/// them: a *line*, the syntactic stand-in for a row or a column. The census
/// counts them; nothing decides by them.
fn on_a_line(terms: &Terms, a: FactId, b: FactId) -> bool {
    let ((ra, xa), (rb, xb)) = (terms.fact(a), terms.fact(b));
    ra == rb
        && xa.len() == 2
        && xb.len() == 2
        && xa.iter().chain(xb).all(|v| v.as_sym().is_some())
        && (xa[0] == xb[0]) != (xa[1] == xb[1])
}

/// The theory's share of one program: its files written under `dir`, and the
/// census line's `theory` object.
fn theory_row(
    path: &Path,
    rel: &str,
    dir: &Path,
    t: &Theory,
    terms: &Terms,
    (rule, opts): (Rule, &structure::Options),
) -> Value {
    let l = &t.root;
    let text = |b: u32| l.text(b);
    let pair = |&(a, b): &(u32, u32)| [l.text(a), l.text(b)];
    let of_theory = t.of_the_theory();
    let root_dead = Deaths::of(l);

    // The statements, by clause.
    let mut clauses: BTreeMap<(&str, bool), usize> = BTreeMap::new();
    for s in t.stated.iter().filter(|s| s.origin == Origin::Fact) {
        *clauses.entry((s.clause, s.kept)).or_default() += 1;
    }
    let facts = t.stated.iter().filter(|s| s.origin == Origin::Fact);
    let (kept, dropped) = facts.fold((0usize, 0usize), |(k, d), s| {
        if s.kept { (k + 1, d) } else { (k, d + 1) }
    });

    // The small KB as text, and whether the loader reads it as the engine
    // built it.
    let kept_facts: Vec<String> = {
        let mut v: Vec<String> = t
            .stated
            .iter()
            .filter(|s| s.kept)
            .map(|s| s.text.clone())
            .collect();
        v.sort();
        v
    };
    let theory_ein = dir.join("theory.ein");
    let written = match small_text(path, rel, t) {
        Some((text, cut, in_file)) => {
            std::fs::write(&theory_ein, &text).expect("theory.ein");
            let from_program = loaded_facts(&text, &theory_ein, path.parent());
            let from_build = loaded_facts(&text, &theory_ein, theory_ein.parent());
            let elsewhere = t
                .stated
                .iter()
                .filter(|s| !s.kept && s.origin == Origin::Fact && !in_file.contains(&s.text))
                .count();
            // Where a module writes a fact the small KB drops, the text
            // loads to the small KB with the module's facts beside it: that
            // program, read as the small KB is, and its layer against the
            // small KB's.
            let with_modules = (elsewhere > 0)
                .then(|| {
                    let (mut kb, mut terms, ast) = loaded(path)?;
                    let asked = structure::TheoryOptions {
                        layer: *opts,
                        root_pairs: false,
                        ..structure::TheoryOptions::default()
                    };
                    let kept_there = |p: &Stated<'_>, f: FactId| {
                        let v = rule(p, f);
                        if !v.kept && !in_file.contains(&p.terms.fact_text(f)) {
                            Verdict::theory(
                                "a module's fact, which the program's text cannot delete",
                            )
                        } else {
                            v
                        }
                    };
                    structure::theory_by(&mut kb, &mut terms, &ast, &asked, &kept_there).ok()
                })
                .flatten();
            let (agrees, same_layer) = match &with_modules {
                Some(m) => {
                    let mut kept: Vec<String> = m
                        .stated
                        .iter()
                        .filter(|s| s.kept)
                        .map(|s| s.text.clone())
                        .collect();
                    kept.sort();
                    (
                        from_program.as_ref().is_ok_and(|f| *f == kept),
                        Some(small_said(m) == small_said(t)),
                    )
                }
                None => (from_program.as_ref().is_ok_and(|f| *f == kept_facts), None),
            };
            json!({
                "file": theory_ein.strip_prefix(repo_root()).unwrap_or(&theory_ein).to_string_lossy(),
                "forms_deleted": cut,
                "dropped_in_another_file": elsewhere,
                "loads": from_program.is_ok(),
                "load_error": from_program.as_ref().err(),
                "loads_where_written": from_build.is_ok(),
                "agrees": agrees,
                "same_layer_with_the_modules_facts": same_layer,
            })
        }
        None => Value::Null,
    };

    // The lines of H: at root, and in the small KB.
    let small_dead = t.small.as_ref().ok().map(Deaths::of);
    let n = l.n() as u32;
    let (mut lines, mut lines_dead, mut lines_theory, mut lines_missed) = (0u64, 0u64, 0u64, 0u64);
    let mut theory_dead_pairs = 0u64;
    for a in 0..n {
        for b in a + 1..n {
            let small = small_dead.as_ref().is_some_and(|d| d.pair(a, b));
            theory_dead_pairs += u64::from(small);
            if !on_a_line(
                terms,
                l.candidates[a as usize].fact,
                l.candidates[b as usize].fact,
            ) {
                continue;
            }
            let at_root = root_dead.pair(a, b);
            lines += 1;
            lines_dead += u64::from(at_root);
            lines_theory += u64::from(small);
            lines_missed += u64::from(at_root && !small);
        }
    }

    let small = match &t.small {
        Err(e) => json!({"error": e}),
        Ok(s) => json!({
            "monotone": s.monotone,
            "domain": s.domain(),
            "root_dead": s.root_dead,
            "licences": s.licences.iter().map(|x| x.label()).collect::<Vec<_>>(),
            "dead_alone": s.dead_alone.len(),
            "implications": s.implies.len(),
            "equivalence_classes": s.equivalent.len(),
            "pairs": {
                "all": s.pairs.all,
                "dead_member": s.pairs.dead_member,
                "implied": s.pairs.implied,
                "saturated": s.pairs.saturated,
                "excluded": s.excluded.len(),
                "interacting": s.interacting.len(),
                "left": s.pairs.left,
            },
            "against_l1": s.against_l1.len(),
            "against_l3": s.against_l3.len(),
            "saturations": s.work.saturations,
            "facts": s.work.facts,
            "stopped": stopped(&s.stopped),
            "singles_left": s.singles_left,
        }),
    };
    let excluded_of_theory = of_theory.iter().filter(|&&x| x).count();
    // The theory's excluded pairs that root does not call exclusions: a
    // member is dead alone there.
    let theory_over_dead_member = t
        .excluded()
        .iter()
        .filter(|&&(a, b)| root_dead.single(a) || root_dead.single(b))
        .count();
    let r = t.readings();
    let checks = t.supports.iter().filter_map(|s| s.check.as_ref());
    let (mut check_dead, mut check_alive, mut check_unread, mut check_within) = (0, 0, 0, 0);
    let mut check_root_dead = 0;
    for c in checks {
        match (&c.unread, c.dead) {
            (Some(_), _) => check_unread += 1,
            (None, true) => check_dead += 1,
            (None, false) => check_alive += 1,
        }
        check_root_dead += usize::from(c.root_dead);
        check_within += usize::from(c.unread.is_none() && c.dead && !c.root_dead && c.beyond == 0);
    }
    let put_back = t.stated.iter().filter(|s| s.read_absent && !s.kept).count();
    let cut = t
        .supports
        .iter()
        .filter(|s| s.read && s.status.is_some_and(|c| c.qualifier().is_some()))
        .count();

    // The lists, raw.
    let statement = |k: u32| {
        let s = &t.stated[k as usize];
        match &s.source {
            Some(src) => format!("{} :source {src:?}", s.text),
            None => s.text.clone(),
        }
    };
    let lists = json!({
        "program": rel,
        "small_kb": {
            "kept": kept,
            "dropped": dropped,
            "by_clause": clauses.iter().map(|((c, k), n)| json!({"clause": c, "kept": k, "facts": n})).collect::<Vec<_>>(),
            "error": t.small.as_ref().err(),
        },
        "stated": t.stated.iter().map(|s| json!({
            "fact": s.text,
            "source": s.source,
            "origin": s.origin.as_str(),
            "kept": s.kept,
            "clause": s.clause,
            "read_absent": s.read_absent,
        })).collect::<Vec<_>>(),
        "theory_dead_alone": t.small.as_ref().map_or(Vec::new(), |s| s.dead_alone.iter().map(|&b| text(b)).collect()),
        "theory_excluded": t.excluded().iter().map(pair).collect::<Vec<_>>(),
        "against_root": {
            "singles": t.against_root.singles.iter().map(|&b| text(b)).collect::<Vec<_>>(),
            "pairs": t.against_root.pairs.iter().map(pair).collect::<Vec<_>>(),
        },
        "small_beyond_root": t.small_beyond_root,
        "root_excluded": l.excluded.iter().enumerate().map(|(k, p)| {
            let support = t.supports.get(k).map(|s| json!({
                "read": s.read,
                "status": s.status.map(|c| c.word()),
                "dropped": s.stated.iter().filter(|&&k| !t.stated[k as usize].kept).map(|&k| statement(k)).collect::<Vec<_>>(),
                "kept": s.stated.iter().filter(|&&k| t.stated[k as usize].kept).map(|&k| statement(k)).collect::<Vec<_>>(),
                "commitments": s.commitments.iter().map(|&b| text(b)).collect::<Vec<_>>(),
                "other": s.other,
                "rules": s.rules,
                "check": s.check.as_ref().map(|c| json!({
                    "dead": c.dead, "root_dead": c.root_dead, "beyond": c.beyond, "unread": c.unread,
                })),
            }));
            json!({"pair": pair(p), "of_the_theory": of_theory[k], "support": support})
        }).collect::<Vec<_>>(),
    });
    let file = dir.join("theory.json");
    let raw = serde_json::to_string_pretty(&lists).expect("JSON");
    std::fs::write(&file, raw + "\n").expect("theory.json");

    // The same as a table.
    let mut md = format!("# The theory's share of `{rel}`\n\n");
    md.push_str("| | |\n|---|---:|\n");
    let mut line = |what: &str, n: String| md.push_str(&format!("| {what} | {n} |\n"));
    line("candidates, H", l.n().to_string());
    line("pairs of H", l.pairs.all.to_string());
    line(
        "facts the program's files state",
        (kept + dropped).to_string(),
    );
    line("of them the small KB drops", dropped.to_string());
    match &t.small {
        Err(e) => line("the small KB has no layer", e.clone()),
        Ok(s) => {
            line("dead alone in the small KB", s.dead_alone.len().to_string());
            line("pairs the theory excludes", s.excluded.len().to_string());
            line(
                "… a member of which is dead alone at root",
                theory_over_dead_member.to_string(),
            );
            line(
                "the small KB's layer: saturations",
                s.work.saturations.to_string(),
            );
            line(
                "the small KB's layer: facts beyond its root",
                s.work.facts.to_string(),
            );
        }
    }
    line("pairs root excludes", l.excluded.len().to_string());
    line("… of the theory", excluded_of_theory.to_string());
    line(
        "… of the puzzle",
        (l.excluded.len() - excluded_of_theory).to_string(),
    );
    line("root's layer: saturations", l.work.saturations.to_string());
    line("root's layer: facts beyond root", l.work.facts.to_string());
    line(
        "dead in the small KB and alive at root",
        (t.against_root.singles.len() + t.against_root.pairs.len()).to_string(),
    );
    line(
        "statements every check of a support puts back",
        put_back.to_string(),
    );
    md.push_str("\n## What the small KB drops\n\n| fact | `:source` | clause |\n|---|---|---|\n");
    for s in t
        .stated
        .iter()
        .filter(|s| !s.kept && s.origin == Origin::Fact)
    {
        md.push_str(&format!(
            "| `{}` | {} | {} |\n",
            s.text,
            s.source.as_deref().unwrap_or(""),
            s.clause
        ));
    }
    md.push_str("\n## What it keeps, by relation\n\n| relation | facts |\n|---|---:|\n");
    let mut by_rel: BTreeMap<String, usize> = BTreeMap::new();
    for s in t
        .stated
        .iter()
        .filter(|s| s.kept && s.origin == Origin::Fact)
    {
        *by_rel
            .entry(terms.sym(terms.fact(s.fact).0).to_string())
            .or_default() += 1;
    }
    for (rel, n) in &by_rel {
        md.push_str(&format!("| `{rel}` | {n} |\n"));
    }
    md.push_str(
        "\n## The pairs root excludes\n\n| | | of | by | ended by |\n|---|---|---|---|---|\n",
    );
    for (k, &(a, b)) in l.excluded.iter().enumerate() {
        let (by, rules) = match t.supports.get(k) {
            Some(s) if s.read => (
                s.stated
                    .iter()
                    .filter(|&&k| !t.stated[k as usize].kept)
                    .map(|&k| {
                        let s = &t.stated[k as usize];
                        s.source.clone().unwrap_or_else(|| format!("`{}`", s.text))
                    })
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", "),
                s.rules.join(", "),
            ),
            Some(_) => ("no support read".to_string(), String::new()),
            None => (String::new(), String::new()),
        };
        md.push_str(&format!(
            "| `{}` | `{}` | {} | {by} | {rules} |\n",
            l.text(a),
            l.text(b),
            if of_theory[k] {
                "the theory"
            } else {
                "the puzzle"
            },
        ));
    }
    std::fs::write(dir.join("theory.md"), md).expect("theory.md");

    json!({
        "rung": t.rung,
        "rules_naming_objects": t.rules_naming_objects,
        "stated": kept + dropped,
        "dropped": dropped,
        "clauses": clauses.iter().map(|((c, k), n)| json!({"clause": c, "kept": k, "facts": n})).collect::<Vec<_>>(),
        "small": small,
        "small_beyond_root": t.small_beyond_root,
        "theory_dead_pairs": theory_dead_pairs,
        "theory_over_dead_member": theory_over_dead_member,
        "root_excluded": l.excluded.len(),
        "root_excluded_of_theory": excluded_of_theory,
        "against_root": {"singles": t.against_root.singles.len(), "pairs": t.against_root.pairs.len()},
        "lines": {"all": lines, "dead_at_root": lines_dead, "dead_in_theory": lines_theory, "dead_at_root_only": lines_missed},
        "supports": {
            "read": t.supports.iter().filter(|s| s.read).count(),
            "unread": r.unread,
            "cut": cut,
            "other": t.supports.iter().filter(|s| !s.other.is_empty()).count(),
            "theory_both": r.theory_both,
            "theory_by_another_way": r.theory_by_another_way,
            "puzzle_both": r.puzzle_both,
            "disagree": r.disagree,
            "check": {
                "dead": check_dead,
                "dead_within": check_within,
                "root_dead": check_root_dead,
                "alive": check_alive,
                "unread": check_unread,
                "put_back_in_every_check": put_back,
            },
            "work": {
                "supports": t.support_work.supports,
                "check_roots": t.support_work.check_roots,
                "check_saturations": t.support_work.check_saturations,
                "check_facts": t.support_work.check_facts,
            },
        },
        "theory_ein": written,
        "lists": file.strip_prefix(repo_root()).unwrap_or(&file).to_string_lossy(),
    })
}
