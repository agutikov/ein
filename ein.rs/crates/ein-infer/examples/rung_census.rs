//! The rung a loaded obligation rule chooses, over the corpus — the census
//! behind Q-M1f.308.
//!
//! Were the generation ladder ([`ein_infer::hypgen`]) to take its second
//! rung wherever the program **loads** an obligation rule, a second rung that
//! owes nothing at root would propose nothing, so H would be empty and the
//! blind enumerator not asked. The rung is chosen by an obligation
//! **activated** at root (Q-M1f.308), so the `today:` columns are (b)'s, and
//! the table's column (a) is the other reading, by loading. This census
//! reads, for every program, which of three things is
//! true at root's fixpoint and what H and R would be were the rung chosen by
//! each:
//!
//! | `class` | at root's fixpoint | (a) a rule loaded | (b) an obligation activated | (c) a debt owed |
//! |---|---|---|---|---|
//! | `none` | no obligation rule is loaded | third rung | third rung | third rung |
//! | `unactivated` | a rule is loaded and root's activators compile no plan of one ([`ein_infer::mechanisms::read`]) | second | **third** | **third** |
//! | `discharged` | a plan is compiled and root owes nothing ([`ein_infer::obligations::tally`]) | second | second | **third** |
//! | `owing` | root owes | second | second | second |
//!
//! An hrule and `(solve :no-search)` come before the second rung under every
//! option, so a program that declares either keeps its rung; a program whose
//! second rung declines is on the third already. The columns `rung-b` and
//! `rung-c` say where each option puts the program, and `moves` which of
//! them changes its rung.
//!
//! ### What the blind enumerator would propose
//!
//! For a program that loads an obligation rule and whose H another rung
//! built, `blind:n` and `blind:relations` are the blind enumerator's own
//! list at root ([`ein_infer::hypgen::generate_blind`]), scoped by the
//! program's lists and closures as the third rung is. Where an option moves
//! the program, that list is its H under the option. Each candidate of it is
//! marked as `object_census` marks one (`shared/marks.rs`): the columns
//! `K1` … `any` count them and the `:names` columns list what they name.
//!
//! ### R under an option, computed
//!
//! For a program an option moves, R over the blind list, by the node
//! function `ein_infer::region` has, call for call: root's fixpoint
//! branched, the commitment written with hypothesis provenance, the
//! saturation resumed from root's snapshot, dead by
//! [`ein_infer::contradiction::has_contradiction`], owing by
//! [`ein_infer::obligations::tally`], a state by
//! [`ein_infer::canon::state_key`]. Before the cube, what a load would say
//! of that H: the static check over root's plans
//! ([`ein_infer::monotone::classify`]). The columns:
//!
//! | column | what |
//! |---|---|
//! | `today:k_c`, `today:k_s` | R as the engine answers today ([`ein_infer::region::compute`] on a second load) |
//! | `static` | `monotone` or `flagged:N`, the static check over the blind list, N its sites |
//! | `cube` | `cube`; `same` where the blind list is empty, so H does not change; `over-limit` past `--limit` |
//! | `nodes`, `alive`, `owing`, `dead-alone` | the cube's nodes, those alive, those of them owing, the candidates dead alone |
//! | `k_c'`, `k_s'` | R over the blind list: its commitments and its states |
//!
//! With `--check`, the same cube is run on every program whose H the blind
//! enumerator builds today and held, node for node, to
//! [`ein_infer::region::compute`]'s two bits and its count of debts: the
//! column `check`. There the blind list **is** H, so the two must agree;
//! `error` is a program whose run ends in an error, which has no region to
//! hold the cube to.
//!
//! ```sh
//! cargo run --release -p ein-infer --example rung_census -- \
//!     [--limit N] [--jobs J] [--check] [FILE…] > rows.tsv
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()`. One tab-separated row a
//! program, then a summary, each line of it starting `# `. It writes nothing
//! and changes no answer.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;
#[allow(dead_code)]
#[path = "shared/marks.rs"]
mod marks;

use std::collections::BTreeMap;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use ein_core::{FactId, Kb, Prov, Symbol, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, sexpr};
use ein_infer::hypgen::{HypGenStats, declares_no_search, generate_blind, generate_config_free};
use ein_infer::oblgen::Mode;
use ein_infer::region::{self, Outcome};
use ein_infer::saturator::{Saturator, Session, Snapshot};
use ein_infer::{mechanisms, monotone, obligations};
use ein_ir::{Ast, load, parse};
use marks::{ANY, E17, E54, IN, IR, K1, K2, Marked, N, R1, R2, Reader, SETS, main_file};
use rustc_hash::FxHashSet;

/// The sets whose names a row lists.
const NAMED: [usize; 6] = [K1, K2, R1, R2, IN, IR];

/// What is true of a program's obligation rules at root's fixpoint.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Class {
    None,
    Unactivated,
    Discharged,
    Owing,
}

impl Class {
    fn as_str(self) -> &'static str {
        match self {
            Class::None => "none",
            Class::Unactivated => "unactivated",
            Class::Discharged => "discharged",
            Class::Owing => "owing",
        }
    }
}

/// Root as `region`'s `prepare` leaves it.
struct Prepared {
    snapshot: Snapshot,
    memo: SharedMemo,
    /// H, as the ladder builds it today.
    h: Vec<FactId>,
    rung: Mode,
}

fn prepare(kb: &mut Kb, terms: &mut Terms, ast: &Ast) -> Option<Prepared> {
    let memo = SharedMemo::default();
    let mut events = Events::off();
    let snapshot = {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
        sat.snapshot(s.kb)
    };
    let mut h = Vec::new();
    let mut stats = HypGenStats::new();
    {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        generate_config_free(&mut s, &mut stats, &mut |f| {
            h.push(f);
            ControlFlow::Continue(())
        })
        .ok()?;
    }
    h.sort();
    h.dedup();
    Some(Prepared {
        snapshot,
        memo,
        h,
        rung: stats.rung.mode,
    })
}

/// The blind enumerator's own list at root, whatever rung the ladder takes.
fn blind_list(kb: &mut Kb, terms: &mut Terms, ast: &Ast, memo: &SharedMemo) -> Option<Vec<FactId>> {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: memo.clone(),
    };
    let mut list = Vec::new();
    generate_blind(&mut s, &mut HypGenStats::new(), &mut |f| {
        list.push(f);
        ControlFlow::Continue(())
    })
    .ok()?;
    list.sort();
    list.dedup();
    Some(list)
}

/// A node's fixpoint: `region`'s `fixpoint`, by the same calls.
fn fixpoint(root: &Kb, p: &Prepared, terms: &mut Terms, ast: &Ast, facts: &[FactId]) -> Option<Kb> {
    let mut fork = root.branch();
    for &h in facts {
        let (rel, args) = terms.facts.get(h);
        let args = args.to_vec();
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        fork.add_and_index_fact(terms, rel, &args, Some(prov))
            .ok()?;
    }
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut fork,
        terms,
        ast,
        events: &mut events,
        memo: p.memo.clone(),
    };
    let delta = p.snapshot.new_facts_of(s.kb);
    let mut sat = Saturator::resume(&mut s, &p.snapshot, delta).ok()?;
    sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    Some(fork)
}

/// The cube over a universe: every node's two bits and its debts.
#[derive(Default)]
struct Cube {
    alive: u64,
    owing: u64,
    dead_alone: usize,
    k_c: u64,
    k_s: usize,
    /// Per mask, in the universe's own bit order: dead, and how much it owes.
    nodes: Vec<(bool, u32)>,
}

fn cube(
    root: &Kb,
    p: &Prepared,
    terms: &mut Terms,
    ast: &Ast,
    universe: &[FactId],
) -> Result<Cube, &'static str> {
    let n = universe.len();
    let mut off = Events::off();
    let mut c = Cube::default();
    let mut states: FxHashSet<u64> = FxHashSet::default();
    for mask in 0..1u64 << n {
        let facts: Vec<FactId> = (0..n)
            .filter(|i| mask >> i & 1 == 1)
            .map(|i| universe[i])
            .collect();
        terms.provs.open_fork();
        let Some(fork) = fixpoint(root, p, terms, ast, &facts) else {
            terms.provs.discard_fork();
            return Err("no-node");
        };
        let dead = ein_infer::contradiction::has_contradiction(&fork, terms);
        let owes = obligations::tally(&fork, terms, ast, &p.memo, &mut off)
            .map_err(|_| "no-tally")?
            .total();
        c.nodes.push((dead, owes as u32));
        if dead {
            if mask.count_ones() == 1 {
                c.dead_alone += 1;
            }
        } else {
            c.alive += 1;
            if owes == 0 {
                c.k_c += 1;
                states.insert(ein_infer::canon::state_digest(
                    &ein_infer::canon::state_key(&fork),
                ));
            } else {
                c.owing += 1;
            }
        }
        drop(fork);
        terms.provs.discard_fork();
    }
    c.k_s = states.len();
    Ok(c)
}

/// R as the engine answers today, on a second load: `(k_c, k_s)` and the
/// region itself.
fn today(path: &Path, limit: u32) -> Result<(u64, usize, Box<region::Region>), &'static str> {
    let (mut kb, mut terms, ast) = census::loaded(path).ok_or("no-load")?;
    let opts = region::Options {
        limit,
        keep_states: false,
        jobs: 1,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    let r = match region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}) {
        Ok(Outcome::Region(r)) => r,
        Ok(Outcome::Refused { .. }) => return Err("over-limit"),
        Err(_) => return Err("error"),
    };
    let mut classes: FxHashSet<u64> = FxHashSet::default();
    let mut k_c = 0u64;
    for mask in 0..1u64 << r.n() {
        if r.in_r(mask) {
            k_c += 1;
            classes.insert(r.nodes[mask as usize].class);
        }
    }
    Ok((k_c, classes.len(), r))
}

/// R over the blind list, or why there is no cube.
struct Under {
    monotone: bool,
    sites: usize,
    cube: Result<Cube, &'static str>,
}

/// One program's row.
struct Row {
    rung: Mode,
    n: usize,
    rules: usize,
    plans: usize,
    hrules: usize,
    no_search: bool,
    root_dead: bool,
    root_owes: usize,
    class: Class,
    /// The blind enumerator's list at root, marked, where another rung built
    /// H and the program loads an obligation rule.
    blind: Option<Marked>,
    /// Its relations, each with how many candidates.
    relations: BTreeMap<String, usize>,
    today: Option<Result<(u64, usize), &'static str>>,
    under: Option<Under>,
    /// `--check`'s verdict, where the blind list is H today.
    check: &'static str,
}

impl Row {
    /// The rung an option that takes the second rung at `from` and above
    /// puts the program on.
    fn rung_under(&self, from: Class) -> &'static str {
        match self.rung {
            Mode::Hrules | Mode::NoSearch | Mode::Blind | Mode::Declined => self.rung.as_str(),
            Mode::Obligations | Mode::Stuck if self.class < from => "blind",
            Mode::Obligations | Mode::Stuck => self.rung.as_str(),
        }
    }

    fn moves(&self) -> &'static str {
        let b = self.rung_under(Class::Discharged) != self.rung.as_str();
        let c = self.rung_under(Class::Owing) != self.rung.as_str();
        match (b, c) {
            (true, _) => "b,c",
            (false, true) => "c",
            (false, false) => "-",
        }
    }
}

fn read(path: &Path, limit: u32, check: bool) -> Result<Row, &'static str> {
    let text = std::fs::read_to_string(path).map_err(|_| "no-load")?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).map_err(|_| "no-load")?;
    let main = main_file(&ast, &forms);
    let mut kb: Kb = load(&mut ast, &mut terms, &forms, path.parent()).map_err(|_| "no-load")?;
    let p = prepare(&mut kb, &mut terms, &ast).ok_or("no-root")?;
    let root_dead = ein_infer::contradiction::has_contradiction(&kb, &terms);
    let mut off = Events::off();
    let root_owes = obligations::tally(&kb, &mut terms, &ast, &p.memo, &mut off)
        .map_err(|_| "no-tally")?
        .total();
    let plans = {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut off,
            memo: p.memo.clone(),
        };
        mechanisms::read(&mut s)
            .map_err(|_| "no-mechanisms")?
            .obligations
            .len()
    };
    let program = kb.program_arc();
    let rules = program.obligations.len();
    let class = if rules == 0 {
        Class::None
    } else if plans == 0 {
        Class::Unactivated
    } else if root_owes == 0 {
        Class::Discharged
    } else {
        Class::Owing
    };
    let mut row = Row {
        rung: p.rung,
        n: p.h.len(),
        rules,
        plans,
        hrules: program.hrules.len(),
        no_search: declares_no_search(&program, &ast),
        root_dead,
        root_owes,
        class,
        blind: None,
        relations: BTreeMap::new(),
        today: None,
        under: None,
        check: "-",
    };
    let on_blind = matches!(p.rung, Mode::Blind | Mode::Declined);

    // `--check`: where the blind list is H, the census's cube against the
    // engine's, node for node.
    if on_blind {
        if check && p.h.len() <= limit as usize {
            row.check = (|| {
                let list = blind_list(&mut kb, &mut terms, &ast, &p.memo).ok_or("no-list")?;
                if list != p.h {
                    return Err("list-differs");
                }
                let (k_c, k_s, r) = today(path, limit)?;
                let texts: Vec<String> = list.iter().map(|&f| sexpr(&terms, f)).collect();
                let mut bit = Vec::with_capacity(texts.len());
                for t in &texts {
                    bit.push(
                        r.candidates
                            .iter()
                            .position(|c| &c.text == t)
                            .ok_or("mismatch")?,
                    );
                }
                if r.n() != list.len() {
                    return Err("mismatch");
                }
                let c = cube(kb.sealed(), &p, &mut terms, &ast, &list)?;
                for (mask, &(dead, owes)) in c.nodes.iter().enumerate() {
                    let theirs = (0..list.len())
                        .filter(|i| mask >> i & 1 == 1)
                        .fold(0usize, |m, i| m | 1 << bit[i]);
                    if (r.status[theirs] & region::DEAD != 0) != dead
                        || r.nodes[theirs].owes != owes
                    {
                        return Err("differs");
                    }
                }
                if c.k_c != k_c || c.k_s != k_s {
                    return Err("differs");
                }
                Ok("ok")
            })()
            .unwrap_or_else(|why| why);
        }
        return Ok(row);
    }
    if rules == 0 {
        return Ok(row);
    }

    // Another rung built H and an obligation rule is loaded: what the blind
    // enumerator alone would propose here.
    let list = blind_list(&mut kb, &mut terms, &ast, &p.memo).ok_or("no-list")?;
    for &f in &list {
        *row.relations
            .entry(terms.sym(terms.facts.get(f).0).to_string())
            .or_default() += 1;
    }
    row.blind = Some(Reader::new(&kb, &terms, &ast, &main).mark(&list));
    if row.moves() == "-" {
        return Ok(row);
    }

    // An option moves the program: R today, and R over the blind list.
    row.today = Some(today(path, limit).map(|(k_c, k_s, _)| (k_c, k_s)));
    let hypothesised: FxHashSet<Symbol> = list.iter().map(|&f| terms.facts.get(f).0).collect();
    let root_plans = monotone::plans_of(&p.memo);
    let m = monotone::classify(&kb, &terms, &root_plans, &hypothesised);
    let cube = if list.is_empty() {
        Err("same")
    } else if list.len() > limit as usize || list.len() >= 64 {
        Err("over-limit")
    } else {
        cube(kb.sealed(), &p, &mut terms, &ast, &list)
    };
    row.under = Some(Under {
        monotone: m.is_monotone(),
        sites: m.sites.len(),
        cube,
    });
    Ok(row)
}

fn joined<'a>(names: impl IntoIterator<Item = &'a String>) -> String {
    let v: Vec<&str> = names.into_iter().map(String::as_str).collect();
    if v.is_empty() {
        "-".to_string()
    } else {
        v.join(",")
    }
}

fn main() {
    let mut limit = region::DEFAULT_LIMIT;
    let mut jobs = 1usize;
    let mut check = false;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--limit" => {
                limit = args.next().and_then(|v| v.parse().ok()).expect("--limit N");
            }
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--check" => check = true,
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

    // Each program on its own load, `jobs` at a time; the rows in the files'
    // order.
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<Option<Result<Row, &'static str>>>> =
        Mutex::new((0..files.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let row = read(path, limit, check);
                    done.lock().expect("no reader panicked")[i] = Some(row);
                }
            });
        }
    });
    let done = done.into_inner().expect("no reader panicked");

    let mut header: Vec<String> = [
        "file",
        "group",
        "status",
        "rung",
        "n",
        "rules",
        "plans",
        "hrules",
        "no-search",
        "root",
        "root-owes",
        "class",
        "rung-b",
        "rung-c",
        "moves",
        "blind:n",
        "blind:relations",
    ]
    .map(String::from)
    .to_vec();
    for set in SETS {
        header.push(set.to_string());
    }
    for set in NAMED {
        header.push(format!("{}:names", SETS[set]));
    }
    header.extend(
        [
            "today:k_c",
            "today:k_s",
            "static",
            "cube",
            "nodes",
            "alive",
            "owing",
            "dead-alone",
            "k_c'",
            "k_s'",
            "check",
        ]
        .map(String::from),
    );
    println!("{}", header.join("\t"));

    let given = files.len();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut read_n = 0usize;
    // (class, rung) → programs
    let mut by_class: BTreeMap<(Class, &'static str), usize> = BTreeMap::new();
    // (class, rung) → programs the blind enumerator would propose on, and
    // the candidates
    let mut proposes: BTreeMap<(Class, &'static str), [usize; 2]> = BTreeMap::new();
    // option → [moved, H the same, over the limit, cube and R the same, cube
    // and R differs]
    let mut moved: BTreeMap<&'static str, [usize; 5]> = BTreeMap::new();
    // option → set → [programs, candidates], over the programs it moves
    let mut marked: BTreeMap<&'static str, [[usize; 2]; N]> = BTreeMap::new();
    let mut candidates: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut lists: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut checked: BTreeMap<&'static str, usize> = BTreeMap::new();

    for (path, row) in files.iter().zip(done) {
        let rel = census::relative(path);
        let group = groups.get(&rel).map_or("-", |g| g.as_str());
        let row = match row.expect("every file was read") {
            Ok(row) => row,
            Err(why) => {
                *skipped.entry(format!("{why} ({group})")).or_default() += 1;
                println!("{rel}\t{group}\t{why}");
                continue;
            }
        };
        read_n += 1;
        let key = (row.class, row.rung.as_str());
        *by_class.entry(key).or_default() += 1;
        if row.check != "-" {
            *checked.entry(row.check).or_default() += 1;
            if row.check != "ok" {
                lists
                    .entry(format!("check {}", row.check))
                    .or_default()
                    .push(rel.clone());
            }
        }
        let moves = row.moves();
        let mut cells = vec![
            rel.clone(),
            group.to_string(),
            "ok".to_string(),
            row.rung.as_str().to_string(),
            row.n.to_string(),
            row.rules.to_string(),
            row.plans.to_string(),
            row.hrules.to_string(),
            if row.no_search { "yes" } else { "-" }.to_string(),
            if row.root_dead { "dead" } else { "alive" }.to_string(),
            row.root_owes.to_string(),
            row.class.as_str().to_string(),
            row.rung_under(Class::Discharged).to_string(),
            row.rung_under(Class::Owing).to_string(),
            moves.to_string(),
        ];
        match &row.blind {
            Some(list) => {
                let n = list.marks.len();
                cells.push(n.to_string());
                let rels: Vec<String> = row
                    .relations
                    .iter()
                    .map(|(r, k)| format!("{r}:{k}"))
                    .collect();
                cells.push(joined(&rels));
                for set in 0..N {
                    cells.push(list.count(set).to_string());
                }
                for set in NAMED {
                    cells.push(joined(&list.names[set]));
                }
                if n > 0 {
                    let e = proposes.entry(key).or_default();
                    e[0] += 1;
                    e[1] += n;
                }
            }
            None => cells.extend(std::iter::repeat_n("-".to_string(), 2 + N + NAMED.len())),
        }
        match (&row.today, &row.under, &row.blind) {
            (Some(today), Some(under), Some(list)) => {
                let (k_c, k_s) = match today {
                    Ok((k_c, k_s)) => (k_c.to_string(), k_s.to_string()),
                    Err(why) => (why.to_string(), "-".to_string()),
                };
                cells.extend([k_c, k_s]);
                cells.push(if under.monotone {
                    "monotone".to_string()
                } else {
                    format!("flagged:{}", under.sites)
                });
                let options: &[&'static str] = if moves == "b,c" { &["b", "c"] } else { &["c"] };
                let slot = match &under.cube {
                    Err("same") => 1,
                    Err(_) => 2,
                    Ok(c) if today.as_ref().ok() == Some(&(c.k_c, c.k_s)) => 3,
                    Ok(_) => 4,
                };
                let what = [
                    "",
                    "H the same",
                    "not reached",
                    "R the same over a new H",
                    "R differs",
                ][slot];
                for &o in options {
                    let e = moved.entry(o).or_default();
                    e[0] += 1;
                    e[slot] += 1;
                    *candidates.entry(o).or_default() += list.marks.len();
                    let m = marked.entry(o).or_default();
                    for (set, cell) in m.iter_mut().enumerate() {
                        if list.count(set) > 0 {
                            cell[0] += 1;
                            cell[1] += list.count(set);
                        }
                    }
                    let line = match &under.cube {
                        Ok(c) => format!(
                            "{rel} (n {} → {}, R {} in {} → {} in {})",
                            row.n,
                            list.marks.len(),
                            today.as_ref().map_or("?".to_string(), |t| t.0.to_string()),
                            today.as_ref().map_or("?".to_string(), |t| t.1.to_string()),
                            c.k_c,
                            c.k_s
                        ),
                        Err(_) => format!("{rel} (n {} → {})", row.n, list.marks.len()),
                    };
                    lists.entry(format!("({o}) {what}")).or_default().push(line);
                }
                match &under.cube {
                    Ok(c) => cells.extend([
                        "cube".to_string(),
                        c.nodes.len().to_string(),
                        c.alive.to_string(),
                        c.owing.to_string(),
                        c.dead_alone.to_string(),
                        c.k_c.to_string(),
                        c.k_s.to_string(),
                    ]),
                    Err(why) => {
                        cells.push(why.to_string());
                        cells.extend(std::iter::repeat_n("-".to_string(), 6));
                    }
                }
            }
            _ => cells.extend(std::iter::repeat_n("-".to_string(), 10)),
        }
        cells.push(row.check.to_string());
        println!("{}", cells.join("\t"));
    }

    println!("# {given} files, {read_n} read");
    for (why, n) in &skipped {
        println!("# not read: {n} {why}");
    }
    println!(
        "# class\trung today\tprograms\tthe blind enumerator alone would propose on\tcandidates"
    );
    for ((class, rung), n) in &by_class {
        let [programs, cands] = proposes.get(&(*class, *rung)).copied().unwrap_or_default();
        let (programs, cands) = if matches!(*rung, "blind" | "declined") || *class == Class::None {
            ("-".to_string(), "-".to_string())
        } else {
            (programs.to_string(), cands.to_string())
        };
        println!("# {}\t{rung}\t{n}\t{programs}\t{cands}", class.as_str());
    }
    println!(
        "# option\tprograms moved to the blind enumerator\tH the same\tnot reached, past --limit {limit}\tR the same over a new H\tR differs\tnew candidates"
    );
    for o in ["b", "c"] {
        let [all, same, over, r_same, r_differs] = moved.get(o).copied().unwrap_or_default();
        println!(
            "# ({o})\t{all}\t{same}\t{over}\t{r_same}\t{r_differs}\t{}",
            candidates.get(o).copied().unwrap_or_default()
        );
    }
    println!("# option\tset\tprograms\tcandidates, of the new candidates of the programs moved");
    for o in ["b", "c"] {
        let m = marked.get(o).copied().unwrap_or([[0; 2]; N]);
        for set in [K1, K2, R1, R2, E17, IN, IR, E54, ANY] {
            println!("# ({o})\t{}\t{}\t{}", SETS[set], m[set][0], m[set][1]);
        }
    }
    if check {
        println!("# --check, the census's cube against region::compute where the blind list is H:");
        for (verdict, n) in &checked {
            println!("# check\t{verdict}\t{n}");
        }
    }
    for (what, files) in &lists {
        for f in files {
            println!("# {what}: {f}");
        }
    }
}
