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
//!     [--coupling | --readings] [--rule NAME] [--out DIR] [FILE…] > rows.jsonl
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
//! the determinism sweeps' budget and cube limit: the sweeps' own function,
//! `ein-render/tests/corpus_ops/searches.rs`, included here.
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
//! ### The cover
//!
//! With the theory's share each program also gets its **cover**
//! ([`ein_infer::structure::Cover`]): the groups, the maximal sets of two or
//! more candidates that pairwise exclude by the theory alone. Two files
//! more, `cover.json`, raw, and `cover.md`, the same as tables, and a
//! `cover` object on the line:
//!
//! - `groups`, their `sizes` and their `open` sizes (the members alive alone
//!   at root), each a list of `[size, groups]`; `one_relation`, the groups
//!   whose members are all of one relation;
//! - `membership`, a list of `[k, candidates in exactly k groups]`, with
//!   `most` and `mean` (over all of `H`);
//! - `in_no_group`, by what root says of each candidate alone
//!   (`alive_alone`, `dead_alone`, `at_root`, `unread`), and
//!   `killed_by_the_theory_alone` among them;
//! - `components` of the theory's exclusion graph, and
//!   `components_are_groups`, whether every one is a clique;
//! - `steps`, the calls the enumeration made, against `bound`, and `cut`,
//!   what the bound cut, `null` where it cut nothing;
//! - `pairs_alive_at_root`, the theory's pairs whose node is alive at root;
//! - `root_cliques`, the maximal cliques of **root's** exclusion graph,
//!   counted beside and not groups: `cliques`, `sizes`, `most`, `mean`,
//!   `steps`, `cut`;
//! - `declared`, what the program declares, beside: the relations its active
//!   hrules propose (`proposed`), its obligations owe (`owable`) and its
//!   closures take out (`closed`), by [`ein_infer::mechanisms::read`] at
//!   root's fixpoint, and the groups `of_a_proposed_relation`,
//!   `of_an_owable_relation`, `of_a_closed_relation`, `of_none_of_them`;
//! - `owed`, the instances root owes with their alternatives, each read
//!   against the cover, as the owner reads them
//!   ([`ein_infer::structure::ExactlyOne`]): `a_group`, `inside_a_group`,
//!   `not_excluding` (and `holding_a_group` among them), `one_alternative`,
//!   `no_alternative`, and `unlicensed` by `why`, an instance whose
//!   alternatives the engine does not know: a fact can witness it that is no
//!   candidate of `H`. `groups_owed` are the groups that are some instance's
//!   alternatives and `groups_nothing_owes` the rest. `table` is `null` on a
//!   program the static check flags and on one that states no obligation,
//!   where no table is read.
//!
//! ### Exactly one
//!
//! With the cover each program gets `exactly_one.json`, raw, and
//! `exactly_one.md`, the same as tables, and the line an `exactly_one`
//! object inside `cover`:
//!
//! - `owed_groups`, the groups whose members are exactly a licensed
//!   instance's alternatives; `groups_nothing_owes`, the others (`all`,
//!   `with_an_open_member`, `an_unlicensed_instance_is_about`);
//!   `owed_sets_not_a_group`, the licensed instances whose alternatives do
//!   not pairwise exclude;
//! - `unlicensed`, the instances L5 does not license: `all`, by `why`,
//!   `about_a_group`, and what is true of their witnesses:
//!   `witnesses_no_candidate_and_not_denied`, of them
//!   `of_them_brought_by_a_candidate_at_root` (in the closure of a candidate
//!   alive alone), those not brought whose bringers by the theory are all
//!   dead alone at root, and those nothing brings, at root or in the small
//!   KB; and by instance `every_such_witness_brought`,
//!   `some_such_witness_not_brought`, `no_such_witness`;
//! - `grids`, one object a grid: the `relations` of its members (read off
//!   the candidates here, for the reader; the owner reads none), its
//!   `members`, the `groups` of its two families, how many of each are
//!   `owed` and how many of the others an unlicensed instance is `about`,
//!   and `bijection`, every group owed and as many in each family;
//!   `bijections`, `grid_kinds`; and
//!   `not_grids`, the components that are no grid by why.
//!
//! `exactly_one.json` lists every instance with every fact that can witness
//! it, what root says of each, and the candidates whose own node holds it at
//! root and in the small KB.
//!
//! ### The coupling
//!
//! `--coupling` reads each program a second time, loaded again, through
//! [`ein_infer::structure::coupling`] under the reading the owner takes, the
//! pairs and the whole node, and with `--supports` the support of every
//! pair root excludes. Three files more, `coupling.json`, raw,
//! `coupling.md`, the same as tables, and `coupling.dot`, the questions'
//! hypergraph, and a `coupling` object on the line's `theory`: `questions`
//! and those that are a candidate `alone`; `ties` of the pair layer by
//! kind; `edges`, how many, how many the pairs gave, the node gave and are
//! `of_a_debt` (and `of_a_debt_alone`), of three questions or more, the
//! `largest`, and `by_arity` as `[arity, edges, of the pairs, of the node,
//! of a debt]`; `one_step`, the same of the edges one application back,
//! which the whole node's reading keeps; `reasons`, the puzzle's
//! statements named; `components`, over every edge, and
//! `conflict_components`, over the conflicts' and the derivations' alone;
//! `debts`, the instances that are edges, those of them
//! `debts_across_conflict_components` and those `debts_unlicensed`, of an
//! instance L5 does not license, whose payers are the ones read;
//! `unlicensed`, every instance L5 does not license; `node`, what the one
//! saturation cost (`saturations`, `facts` beyond root, `applications`,
//! `steps`, `contradictions`, `rounds`, `merged`), `node_stopped` and
//! `cut`; and `wall`, of the whole second reading, both pair layers
//! included. `--readings` adds the same counts under each other reading,
//! `pairs`, `direct` and `one-step`, each read again.
//!
//! In `coupling.json` an edge and a debt name their questions twice: by
//! text (`questions`), and by place in the file's `questions` list
//! (`question_ids`), which is what `coupling.dot` calls `q<place>`. Two
//! questions of one program can share a text, the three kinds of attribute
//! at one house of `zebra.ein`, and only the place tells them apart.
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
use ein_infer::structure::{
    self, Cliques, Coupling, CouplingOptions, CouplingReading, Cover, Deaths, ExactlyOne, Mark,
    NotAGrid, Origin, Owing, PairLayer, Question, Stated, Theory, Verdict, WitnessIs,
};
use ein_ir::{Ast, load, parse};
use rustc_hash::FxHashSet;
use serde_json::{Value, json};

// What *a program searches* means, by the determinism sweeps' one
// definition.
#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

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

/// Whether the program at `path` searches, loaded again.
fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return false;
    };
    searching::searches(&ast, &mut terms, &mut kb)
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
    let mut coupling = false;
    let mut readings = false;
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
            "--coupling" => (theory, coupling) = (true, true),
            "--readings" => (theory, coupling, readings) = (true, true, true),
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
                    let mut share = theory_row(&path, &rel, &dir, &t, &terms, (rule, &opts));
                    share["cover"] = cover_row(&path, &rel, &dir, &t, &terms);
                    if coupling {
                        share["coupling"] =
                            coupling_row(&path, &rel, &dir, (opts, supports, readings));
                    }
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

// ── The cover ─────────────────────────────────────────────────────────

/// `[size, how many]`, ascending.
fn histogram(sizes: impl Iterator<Item = usize>) -> Vec<[usize; 2]> {
    let mut by: BTreeMap<usize, usize> = BTreeMap::new();
    for k in sizes {
        *by.entry(k).or_default() += 1;
    }
    by.into_iter().map(|(k, n)| [k, n]).collect()
}

fn cut_of(c: &Option<structure::Cut>) -> Value {
    match c {
        None => Value::Null,
        Some(c) => json!({"pairs": c.pairs, "candidates": c.candidates}),
    }
}

/// The counts of one list of cliques over `n` candidates.
fn clique_counts(c: &Cliques, n: usize) -> Value {
    let per = c.memberships(n);
    json!({
        "cliques": c.cliques.len(),
        "sizes": histogram(c.cliques.iter().map(Vec::len)),
        "most": per.iter().max().copied().unwrap_or(0),
        "mean": if n > 0 { json!(per.iter().sum::<u32>() as f64 / n as f64) } else { Value::Null },
        "in_none": per.iter().filter(|&&k| k == 0).count(),
        "steps": c.steps,
        "cut": cut_of(&c.cut),
    })
}

/// Why the engine does not know an instance's alternatives, with the fact
/// or the relation it names left out: the kind of reason.
fn kind_of(why: &str) -> String {
    let mut out = String::new();
    let mut quoted = false;
    for c in why.chars() {
        if c == '`' {
            quoted = !quoted;
            if quoted {
                out.push('…');
            }
        } else if !quoted {
            out.push(c);
        }
    }
    out
}

/// What the program declares, read beside the cover: the mechanisms at
/// root's fixpoint and the instances root owes with their alternatives, of
/// the program loaded again.
fn declared_beside(
    path: &Path,
    cover: &Cover,
    exact: &ExactlyOne,
    terms: &Terms,
) -> (Value, Value, Vec<Value>) {
    use ein_infer::events::Events;
    let Some((mut kb, mut again, ast)) = loaded(path) else {
        return (Value::Null, Value::Null, Vec::new());
    };
    // The mechanisms are read at root's fixpoint. This call prepares root
    // as every route does; the table it returns is the one the owner read
    // (`exact.owed`) and is not read again here.
    if let Err(e) = ein_infer::oblgen::alternatives_of(&mut kb, &mut again, &ast) {
        return (Value::Null, json!({"error": e.to_string()}), Vec::new());
    }
    let mut off = Events::off();
    let memo = ein_infer::compile::SharedMemo::default();
    let mechanisms = {
        let mut s = ein_infer::saturator::Session {
            kb: &mut kb,
            terms: &mut again,
            ast: &ast,
            events: &mut off,
            memo: memo.clone(),
        };
        ein_infer::mechanisms::read(&mut s).ok()
    };

    // The relations, as text: the two loads intern alike and are not
    // assumed to.
    let names = |set: &FxHashSet<Symbol>| -> BTreeSet<String> {
        // determinism-ok: collected into a sorted set of texts.
        set.iter().map(|&r| again.sym(r).to_string()).collect()
    };
    let (proposed, owable, closed) = match &mechanisms {
        Some(m) => (names(&m.proposed()), names(&m.owable()), names(&m.closed)),
        None => Default::default(),
    };
    let relation_of = |b: u32| -> String {
        let fact = cover.candidates[b as usize].fact;
        let (rel, args) = terms.fact(fact);
        let rel = match args.first().and_then(|a| a.as_fact()) {
            Some(inner) if rel == terms.kernel.not => terms.fact(inner).0,
            _ => rel,
        };
        terms.sym(rel).to_string()
    };
    let group_relations: Vec<BTreeSet<String>> = cover
        .groups
        .iter()
        .map(|g| g.iter().map(|&b| relation_of(b)).collect())
        .collect();
    let all_in = |set: &BTreeSet<String>| {
        group_relations
            .iter()
            .filter(|r| r.iter().all(|x| set.contains(x)))
            .count()
    };
    let declared = json!({
        "hrules": mechanisms.as_ref().map(|m| m.hrules.len()),
        "active_hrule_plans": mechanisms.as_ref().map(|m| m.proposals.len()),
        "obligation_rules": mechanisms.as_ref().map(|m| m.obligation_rules.len()),
        "obligation_plans": mechanisms.as_ref().map(|m| m.obligations.len()),
        "proposed": proposed,
        "owable": owable,
        "closed": closed,
        "listed": mechanisms.as_ref().and_then(|m| m.listed.as_ref().map(&names)),
        "excluded": mechanisms.as_ref().map(|m| names(&m.excluded)),
        "groups": {
            "one_relation": group_relations.iter().filter(|r| r.len() == 1).count(),
            "of_a_proposed_relation": all_in(&proposed),
            "of_an_owable_relation": all_in(&owable),
            "of_a_closed_relation": all_in(&closed),
            "of_none_of_them": group_relations
                .iter()
                .filter(|r| !r.iter().all(|x| proposed.contains(x)) && !r.iter().all(|x| owable.contains(x)))
                .count(),
        },
    });

    // The owed instances, each against the cover: the owner's reading.
    let mut rows: Vec<Value> = Vec::new();
    let (mut a_group, mut inside, mut not_excluding, mut holding) = (0usize, 0usize, 0usize, 0);
    let (mut one, mut none, mut licensed) = (0usize, 0usize, 0usize);
    let mut unlicensed: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_rule: BTreeMap<String, BTreeMap<&'static str, usize>> = BTreeMap::new();
    for (i, owing) in exact.owed.table().iter().zip(&exact.owing) {
        let detail: Value = match &i.alternatives {
            Err(why) => {
                *unlicensed.entry(kind_of(why)).or_default() += 1;
                json!({"why": why})
            }
            Ok(alts) => {
                licensed += 1;
                let texts: Vec<&str> = alts.iter().map(|&b| cover.text(b)).collect();
                let marks: Vec<&str> = alts
                    .iter()
                    .map(|&b| cover.marks[b as usize].as_str())
                    .collect();
                let more = match owing {
                    Owing::None => {
                        none += 1;
                        json!({})
                    }
                    Owing::One(_) => {
                        one += 1;
                        json!({})
                    }
                    Owing::Group(g) => {
                        a_group += 1;
                        json!({"group": g})
                    }
                    Owing::Inside(groups) => {
                        inside += 1;
                        let beyond: Vec<Vec<&str>> = groups
                            .iter()
                            .map(|&g| {
                                cover
                                    .members(g)
                                    .iter()
                                    .filter(|b| !alts.contains(b))
                                    .map(|&b| cover.text(b))
                                    .collect()
                            })
                            .collect();
                        json!({"groups": groups, "the_groups_other_members": beyond})
                    }
                    Owing::NotExcluding { alive, holds } => {
                        not_excluding += 1;
                        holding += usize::from(!holds.is_empty());
                        json!({"pairs_the_theory_does_not_exclude": alive, "holds_groups": holds})
                    }
                    Owing::Unlicensed { .. } => unreachable!("licensed"),
                };
                json!({
                    "alternatives": texts,
                    "marks": marks,
                    "witnesses_denied_at_root": i.denied,
                    "detail": more,
                })
            }
        };
        let standing = owing.as_str();
        *by_rule
            .entry(i.rule.clone())
            .or_default()
            .entry(standing)
            .or_default() += 1;
        rows.push(
            json!({"instance": i.text, "rule": i.rule, "standing": standing, "read": detail}),
        );
    }
    let owed_groups = exact.owed_groups().len();
    let owed = json!({
        "root_owes": exact.owed.root_owes,
        "table": exact.owed.instances.as_ref().ok().map(Vec::len),
        "licensed": licensed,
        "unlicensed": unlicensed.values().sum::<usize>(),
        "unlicensed_why": unlicensed,
        "a_group": a_group,
        "inside_a_group": inside,
        "not_excluding": not_excluding,
        "holding_a_group": holding,
        "one_alternative": one,
        "no_alternative": none,
        "groups_owed": owed_groups,
        "groups_nothing_owes": cover.groups.len() - owed_groups,
        "by_rule": by_rule,
    });
    (declared, owed, rows)
}

/// One reading of the coupling, of the program loaded again: the cover and
/// the coupling, with the wall of the whole reading, root's saturation and
/// both pair layers included.
fn coupled(
    path: &Path,
    opts: structure::Options,
    reading: CouplingReading,
    supports: bool,
) -> Result<(Cover, Coupling, f64), String> {
    let (mut kb, mut terms, ast) = loaded(path).ok_or("it does not load")?;
    let asked = CouplingOptions {
        layer: opts,
        reading,
        supports,
    };
    let t = Instant::now();
    let (cover, c) =
        structure::coupling(&mut kb, &mut terms, &ast, &asked).map_err(|e| e.to_string())?;
    Ok((cover, c, t.elapsed().as_secs_f64()))
}

/// The members of a set of candidates as one pattern, where they differ in
/// one word: `(val D *)`. Otherwise its first member and how many there
/// are.
fn pattern(texts: &[&str]) -> String {
    let words: Vec<Vec<&str>> = texts
        .iter()
        .map(|t| t.split_whitespace().collect())
        .collect();
    let Some(first) = words.first() else {
        return "(none)".to_string();
    };
    if texts.len() == 1 {
        return texts[0].to_string();
    }
    if words.iter().all(|w| w.len() == first.len()) {
        let differ: Vec<usize> = (0..first.len())
            .filter(|&k| words.iter().any(|w| w[k] != first[k]))
            .collect();
        if let [k] = differ[..] {
            let mut out: Vec<String> = first.iter().map(|w| w.to_string()).collect();
            let (open, close) = (first[k].starts_with('('), first[k].ends_with(')'));
            out[k] = format!(
                "{}*{}",
                if open { "(" } else { "" },
                if close { ")" } else { "" }
            );
            return out.join(" ");
        }
    }
    format!("{} … ×{}", texts[0], texts.len())
}

/// The coupling, of one program: `coupling.json`, raw, `coupling.md`, the
/// same as tables, and `coupling.dot`, the questions' hypergraph drawn, an
/// edge of two questions a line and an edge of more a point joined to
/// each. The line's object: the questions, the ties by kind, the edges by
/// arity and by what gave them, the components, the debts across them,
/// what the whole node cost, and the wall of the reading. With `readings`,
/// the same counts under each of the other readings.
fn coupling_row(
    path: &Path,
    rel: &str,
    dir: &Path,
    (opts, supports, readings): (structure::Options, bool, bool),
) -> Value {
    let (cover, c, wall) = match coupled(path, opts, CouplingReading::Node, supports) {
        Ok(read) => read,
        Err(e) => return json!({"error": e}),
    };
    let arities = |edges: &[structure::Edge]| -> Vec<Value> {
        // [arity, edges, of the pairs, of the node, of a debt]
        let mut by_arity: BTreeMap<usize, [u32; 4]> = BTreeMap::new();
        for e in edges {
            let at = by_arity.entry(e.arity()).or_default();
            at[0] += 1;
            at[1] += u32::from(e.pairs.all() > 0);
            at[2] += u32::from(e.node.all() > 0);
            at[3] += u32::from(!e.debts.is_empty());
        }
        by_arity
            .iter()
            .map(|(k, v)| json!([k, v[0], v[1], v[2], v[3]]))
            .collect()
    };
    let edge_counts = |edges: &[structure::Edge]| -> Value {
        let debt_alone = |e: &&structure::Edge| e.kind() == structure::EdgeKind::Debt;
        json!({
            "all": edges.len(),
            "of_the_pairs": edges.iter().filter(|e| e.pairs.all() > 0).count(),
            "of_the_node": edges.iter().filter(|e| e.node.all() > 0).count(),
            "of_a_debt": edges.iter().filter(|e| !e.debts.is_empty()).count(),
            "of_a_debt_alone": edges.iter().filter(debt_alone).count(),
            "of_three_or_more": edges.iter().filter(|e| e.arity() >= 3).count(),
            "largest": edges.iter().map(|e| e.arity()).max().unwrap_or(0),
            "by_arity": arities(edges),
        })
    };
    let counts = |c: &Coupling, wall: f64| -> Value {
        let kind = |k: structure::TieKind| c.ties.iter().filter(|t| t.kind == k).count();
        json!({
            "reading": c.reading.as_str(),
            "complete": c.complete(),
            "domain": c.domain(),
            "questions": c.questions.len(),
            "alone": c.alone().count(),
            "ties": {
                "all": c.ties.len(),
                "excludes": kind(structure::TieKind::Excludes),
                "implies": kind(structure::TieKind::Implies),
                "interacts": kind(structure::TieKind::Interacts),
            },
            "edges": edge_counts(&c.edges),
            "one_step": c.one_step.as_deref().map(edge_counts),
            "reasons": c.reasons.len(),
            "components": c.components.len(),
            "largest_component": c.components.iter().map(Vec::len).max().unwrap_or(0),
            "conflict_components": c.conflict_components.len(),
            "debts": c.debts.len(),
            "debts_across_conflict_components": c.debts_across().len(),
            "debts_unlicensed": c.debts.iter().filter(|d| !d.licensed).count(),
            "unlicensed": c.unlicensed,
            "committed": c.committed.len(),
            "node": c.node.map(|w| json!({
                "saturations": w.saturations,
                "facts": w.facts,
                "applications": w.applications,
                "steps": w.steps,
                "contradictions": w.contradictions,
                "rounds": w.rounds,
                "merged": w.merged,
            })),
            "node_stopped": c.node_stopped.map(|s| s.as_str()),
            "cut": {
                "ways": c.cut.ways,
                "descriptions": c.cut.descriptions,
                "unsettled": c.cut.unsettled,
            },
            "wall": wall,
        })
    };
    let mut row = counts(&c, wall);
    if readings {
        let mut others = serde_json::Map::new();
        for reading in [
            CouplingReading::Pairs,
            CouplingReading::Direct,
            CouplingReading::OneStep,
        ] {
            let got = match coupled(path, opts, reading, false) {
                Ok((_, c, wall)) => counts(&c, wall),
                Err(e) => json!({"error": e}),
            };
            others.insert(reading.as_str().to_string(), got);
        }
        row["readings"] = Value::Object(others);
    }

    // The files.
    let members = |q: u32| -> Vec<u32> {
        match c.questions[q as usize] {
            Question::Group(g) => cover.members(g).to_vec(),
            Question::Alone(b) => vec![b],
        }
    };
    let name = |q: u32| -> String {
        let m = members(q);
        pattern(&m.iter().map(|&b| c.text(b)).collect::<Vec<_>>())
    };
    let open = |q: u32| -> usize {
        members(q)
            .iter()
            .filter(|&&b| cover.marks[b as usize] == Mark::AliveAlone)
            .count()
    };
    let texts = |v: &[u32]| -> Vec<&str> { v.iter().map(|&b| c.text(b)).collect() };
    let edges_raw = |edges: &[structure::Edge]| -> Vec<Value> {
        edges.iter().map(|e| json!({
            "questions": e.questions.iter().map(|&q| name(q)).collect::<Vec<_>>(),
            "question_ids": e.questions,
            "kind": e.kind().as_str(),
            "pairs": {"excludes": e.pairs.excludes, "implies": e.pairs.implies, "interacts": e.pairs.interacts},
            "node": {"conflicts": e.node.conflicts, "derivations": e.node.derivations, "implications": e.node.implications},
            "debts": e.debts.iter().map(|&d| c.debts[d as usize].instance.as_str()).collect::<Vec<_>>(),
            "stated": e.stated.iter().map(|&(k, n)| json!([c.reasons[k as usize], n])).collect::<Vec<_>>(),
        })).collect()
    };
    let raw = json!({
        "program": rel,
        "reading": c.reading.as_str(),
        "complete": c.complete(),
        "domain": c.domain(),
        "questions": (0..c.questions.len() as u32).map(|q| json!({
            "is": match c.questions[q as usize] {
                Question::Group(_) => "a group",
                Question::Alone(_) => "a candidate in no group",
            },
            "name": name(q),
            "open": open(q),
            "members": members(q).iter().map(|&b| json!([c.text(b), cover.marks[b as usize].as_str()])).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "ties": c.ties.iter().map(|t| json!({
            "pair": [c.text(t.pair.0), c.text(t.pair.1)],
            "kind": t.kind.as_str(),
            "stated": t.stated.as_ref().map(|s| s.iter().map(|&k| c.reasons[k as usize].as_str()).collect::<Vec<_>>()),
        })).collect::<Vec<_>>(),
        "edges": edges_raw(&c.edges),
        "edges_one_step_back": c.one_step.as_deref().map(edges_raw),
        "components": c.components.iter().map(|k| texts(k)).collect::<Vec<_>>(),
        "conflict_components": c.conflict_components.iter().map(|k| texts(k)).collect::<Vec<_>>(),
        "debts": c.debts.iter().map(|d| json!({
            "rule": d.rule,
            "instance": d.instance,
            "licensed": d.licensed,
            "payers": texts(&d.payers),
            "questions": d.questions.iter().map(|&q| name(q)).collect::<Vec<_>>(),
            "question_ids": d.questions,
        })).collect::<Vec<_>>(),
        "debts_across_conflict_components": c.debts_across(),
        "unlicensed_instances": c.unlicensed,
        "committed": texts(&c.committed),
        "counts": row,
    });
    let text = serde_json::to_string_pretty(&raw).expect("JSON");
    std::fs::write(dir.join("coupling.json"), text + "\n").expect("coupling.json");

    let mut md = format!("# The coupling of `{rel}`\n\n");
    let whole = if c.complete() {
        ""
    } else {
        ", **not read whole**"
    };
    md += &format!(
        "Reading: **{}**{whole}. {} questions ({} a candidate in no group), {} ties of the pair \
         layer, {} edges ({} a debt's), {} components ({} over the conflicts' and the \
         derivations' edges alone), {} debts that are edges ({} of an instance L5 does not \
         license).\n\n",
        c.reading.as_str(),
        c.questions.len(),
        c.alone().count(),
        c.ties.len(),
        c.edges.len(),
        c.edges.iter().filter(|e| !e.debts.is_empty()).count(),
        c.components.len(),
        c.conflict_components.len(),
        c.debts.len(),
        c.debts.iter().filter(|d| !d.licensed).count()
    );
    if let Some(domain) = c.domain() {
        md += &format!("**{domain}.**\n\n");
    }
    match (c.node, c.node_stopped) {
        (_, Some(stop)) => md += &format!("The whole node was not read: {}.\n\n", stop.as_str()),
        (Some(w), None) => {
            md += &format!(
                "The whole node: {} candidates committed, 1 saturation, {} facts beyond root, {} \
                 applications, {} derivation steps, {} contradictions; labels settled in {} \
                 rounds, {} facts with ways of more than one kind.\n\n",
                c.committed.len(),
                w.facts,
                w.applications,
                w.steps,
                w.contradictions,
                w.rounds,
                w.merged
            );
        }
        (None, None) => {}
    }
    let table = |edges: &[structure::Edge]| -> String {
        let mut md = "| questions | arity | kind | pairs: excludes / implies / interacts | node: conflicts / derivations / implications | the debts, and the statements most named |\n|---|---:|---|---|---|---|\n".to_string();
        let mut order: Vec<&structure::Edge> = edges.iter().collect();
        order.sort_by_key(|e| {
            (
                std::cmp::Reverse(e.arity()),
                std::cmp::Reverse(e.pairs.all() + e.node.all()),
            )
        });
        for e in &order {
            let questions: Vec<String> = e
                .questions
                .iter()
                .map(|&q| format!("`{}`", name(q)))
                .collect();
            let mut why: Vec<String> = e
                .debts
                .iter()
                .map(|&d| format!("owed: `{}`", c.debts[d as usize].instance))
                .collect();
            why.extend(
                e.stated
                    .iter()
                    .take(3)
                    .map(|&(k, n)| format!("`{}` ×{n}", c.reasons[k as usize])),
            );
            md += &format!(
                "| {} | {} | {} | {} / {} / {} | {} / {} / {} | {} |\n",
                questions.join(" + "),
                e.arity(),
                e.kind().as_str(),
                e.pairs.excludes,
                e.pairs.implies,
                e.pairs.interacts,
                e.node.conflicts,
                e.node.derivations,
                e.node.implications,
                why.join(", ")
            );
        }
        md
    };
    md += "## The edges\n\n";
    md += &table(&c.edges);
    if let Some(one_step) = &c.one_step {
        md += &format!(
            "\n## The edges one application back\n\n{} edges, {} of three questions or more, \
             against {} and {} above.\n\n",
            one_step.len(),
            one_step.iter().filter(|e| e.arity() >= 3).count(),
            c.edges.len(),
            c.edges.iter().filter(|e| e.arity() >= 3).count()
        );
        md += &table(one_step);
    }
    md += "\n## The questions\n\n| question | is | members | open at root |\n|---|---|---:|---:|\n";
    for q in 0..c.questions.len() as u32 {
        md += &format!(
            "| `{}` | {} | {} | {} |\n",
            name(q),
            match c.questions[q as usize] {
                Question::Group(_) => "a group",
                Question::Alone(_) => "a candidate in no group",
            },
            members(q).len(),
            open(q)
        );
    }
    let listed = |parts: &[Vec<u32>]| -> String {
        let mut md = String::new();
        for (k, members) in parts.iter().enumerate() {
            md += &format!(
                "{}. {} candidates: `{}`\n",
                k + 1,
                members.len(),
                texts(members).join("` `")
            );
        }
        md
    };
    md += "\n## The components\n\n";
    md += &listed(&c.components);
    if c.conflict_components.len() != c.components.len() {
        md += "\n## The components over the conflicts' and the derivations' edges alone\n\n";
        md += &listed(&c.conflict_components);
    }
    if !c.debts.is_empty() {
        md += "\n## The debts that are edges\n\nAn instance root owes whose payers lie in two questions or more. Where L5 does not license it the payers are those read, the candidates whose own node holds one of its witnesses, and the edge may be too small.\n\n| rule | instance | L5 licenses it | payers | questions | across the conflicts' components |\n|---|---|---|---|---|---|\n";
        let across = c.debts_across();
        for (k, d) in c.debts.iter().enumerate() {
            let questions: Vec<String> = d
                .questions
                .iter()
                .map(|&q| format!("`{}`", name(q)))
                .collect();
            md += &format!(
                "| `{}` | `{}` | {} | `{}` | {} | {} |\n",
                d.rule,
                d.instance,
                if d.licensed { "yes" } else { "**no**" },
                texts(&d.payers).join("` `"),
                questions.join(" + "),
                if across.contains(&(k as u32)) {
                    "yes"
                } else {
                    "no"
                }
            );
        }
    }
    std::fs::write(dir.join("coupling.md"), md).expect("coupling.md");

    let quoted = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
    let mut dot = format!(
        "graph coupling {{\n  label=\"{}\";\n  node [shape=box, fontsize=10];\n",
        quoted(rel)
    );
    let in_an_edge: BTreeSet<u32> = c
        .edges
        .iter()
        .flat_map(|e| e.questions.iter().copied())
        .collect();
    for q in 0..c.questions.len() as u32 {
        dot += &format!(
            "  q{q} [label=\"{}\\n{} open of {}\"{}];\n",
            quoted(&name(q)),
            open(q),
            members(q).len(),
            if in_an_edge.contains(&q) {
                ""
            } else {
                ", color=gray, fontcolor=gray"
            }
        );
    }
    for (k, e) in c.edges.iter().enumerate() {
        let weight = e.pairs.all() + e.node.all();
        // A debt's edge alone is dashed, and named for its debts.
        let (style, label) = match e.kind() {
            structure::EdgeKind::Debt => {
                let unlicensed = e.debts.iter().any(|&d| !c.debts[d as usize].licensed);
                let label = format!(
                    "owed ×{}{}",
                    e.debts.len(),
                    if unlicensed { ", unlicensed" } else { "" }
                );
                (", style=dashed", label)
            }
            structure::EdgeKind::Both => ("", format!("{weight}, owed ×{}", e.debts.len())),
            structure::EdgeKind::Tie => ("", weight.to_string()),
        };
        if let [a, b] = e.questions[..] {
            dot += &format!("  q{a} -- q{b} [label=\"{label}\", fontsize=8{style}];\n");
        } else {
            dot += &format!("  e{k} [shape=point, xlabel=\"{label}\"];\n");
            let dashed = if style.is_empty() {
                ""
            } else {
                " [style=dashed]"
            };
            for q in &e.questions {
                dot += &format!("  e{k} -- q{q}{dashed};\n");
            }
        }
    }
    dot += "}\n";
    std::fs::write(dir.join("coupling.dot"), dot).expect("coupling.dot");

    row["files"] = json!(["coupling.json", "coupling.md", "coupling.dot"]);
    row
}

/// *Exactly one*, of one program: `exactly_one.json` and `exactly_one.md`
/// written under `dir`, and the census line's `exactly_one` object. The
/// relations of a grid's members are read here off the candidates, for the
/// reader and for the comparison with what a program declares by name; the
/// owner reads none.
fn exactly_one_row(rel: &str, dir: &Path, c: &Cover, x: &ExactlyOne, terms: &Terms) -> Value {
    let one = |v: &[u32]| -> Vec<&str> { v.iter().map(|&b| c.text(b)).collect() };
    let relation_of = |b: u32| -> String {
        let (r, args) = terms.fact(c.candidates[b as usize].fact);
        let r = match args.first().and_then(|a| a.as_fact()) {
            Some(inner) if r == terms.kernel.not => terms.fact(inner).0,
            _ => r,
        };
        terms.sym(r).to_string()
    };
    let open = |g: u32| c.open(g).count();
    let table = x.owed.table();

    // The instances.
    let mut unlicensed_why: BTreeMap<String, usize> = BTreeMap::new();
    let (mut about_a_group, mut unknown, mut unproposed, mut brought) = (0usize, 0usize, 0u32, 0);
    let (mut all_brought, mut none_unproposed, mut some_not_brought) = (0usize, 0usize, 0usize);
    // Of the witnesses no candidate brings at root: those the theory's
    // bringers of which are all dead alone at root, and those nothing
    // brings in the small KB either.
    let (mut bringers_dead, mut brought_nowhere) = (0usize, 0usize);
    let mut instances: Vec<Value> = Vec::new();
    for (k, (i, owing)) in table.iter().zip(&x.owing).enumerate() {
        let witnesses = match &i.witnesses {
            Err(why) => json!({"unknown": why}),
            Ok(ws) => json!(
                ws.iter()
                    .map(|w| json!({
                        "fact": w.text,
                        "is": w.is.as_str(),
                        "held_at_root_by": one(&w.held_at_root),
                        "held_in_the_theory_by": one(&w.held_in_theory),
                    }))
                    .collect::<Vec<_>>()
            ),
        };
        let mut row = json!({
            "instance": i.text,
            "rule": i.rule,
            "standing": owing.as_str(),
            "licensed": i.alternatives.is_ok(),
            "witnesses_denied_at_root": i.witnesses.as_ref().ok().map(|ws| {
                ws.iter().filter(|w| w.is == WitnessIs::Denied).count()
            }),
            "witnesses": witnesses,
        });
        match (&i.alternatives, owing) {
            (Ok(alts), _) => {
                row["alternatives"] = json!(one(alts));
                match owing {
                    Owing::Group(g) => row["group"] = json!(g),
                    Owing::Inside(groups) => row["inside"] = json!(groups),
                    Owing::NotExcluding { alive, holds } => {
                        row["pairs_the_theory_does_not_exclude"] = json!(alive);
                        row["holds_groups"] = json!(holds);
                    }
                    _ => {}
                }
            }
            (Err(why), Owing::Unlicensed { payers, about }) => {
                *unlicensed_why.entry(kind_of(why)).or_default() += 1;
                row["why"] = json!(why);
                row["pay_by_the_theory_alone"] = json!(payers.as_ref().map(|p| one(p)));
                row["about_group"] = json!(about);
                about_a_group += usize::from(about.is_some());
                for w in i.witnesses.iter().flatten() {
                    if w.is == WitnessIs::Unproposed && w.held_at_root.is_empty() {
                        if w.held_in_theory.is_empty() {
                            brought_nowhere += 1;
                        } else if w
                            .held_in_theory
                            .iter()
                            .all(|&b| c.marks[b as usize] == Mark::DeadAlone)
                        {
                            bringers_dead += 1;
                        }
                    }
                }
                match x.brought(k as u32) {
                    None => unknown += 1,
                    Some((all, by)) => {
                        row["witnesses_no_candidate_and_not_denied"] = json!(all);
                        row["of_them_brought_by_a_candidate_at_root"] = json!(by);
                        unproposed += all;
                        brought += by;
                        if all == 0 {
                            none_unproposed += 1;
                        } else if all == by {
                            all_brought += 1;
                        } else {
                            some_not_brought += 1;
                        }
                    }
                }
            }
            (Err(_), _) => unreachable!("an unlicensed instance stands as one"),
        }
        instances.push(row);
    }

    // The groups, each with who owes it.
    let grid_of = |g: u32| x.grids.of_group[g as usize];
    let groups: Vec<Value> = (0..c.groups.len() as u32)
        .map(|g| {
            json!({
                "group": g,
                "members": one(c.members(g)),
                "open": c.open(g).map(|b| c.text(b)).collect::<Vec<_>>(),
                "owed_by": x.owed_by[g as usize].iter().map(|&k| &table[k as usize].text).collect::<Vec<_>>(),
                "an_unlicensed_instance_is_about_it": x.about[g as usize].iter().map(|&k| &table[k as usize].text).collect::<Vec<_>>(),
                "grid": grid_of(g).map(|(grid, family)| json!({"grid": grid, "family": family})),
            })
        })
        .collect();
    let unowed = x.unowed();
    let unowed_open = unowed.iter().filter(|&&g| open(g) > 0).count();
    let unowed_about = unowed
        .iter()
        .filter(|&&g| !x.about[g as usize].is_empty())
        .count();

    // The grids.
    let members_of = |component: u32| &c.components[component as usize];
    let relations_of =
        |members: &[u32]| -> BTreeSet<String> { members.iter().map(|&b| relation_of(b)).collect() };
    let mut kinds: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut grid_rows: Vec<Value> = Vec::new();
    let mut grid_counts: Vec<Value> = Vec::new();
    for (k, g) in x.grids.grids.iter().enumerate() {
        let share = x.share(k as u32);
        let members = members_of(g.component);
        let kind = if x.is_bijection(k as u32) {
            "a bijection: every group owed"
        } else if share.iter().all(|s| s.owed == s.groups) {
            "every group owed, and the families differ in number"
        } else if share.iter().any(|s| s.owed == s.groups) {
            "one family owed whole"
        } else if share.iter().any(|s| s.owed > 0) {
            "owed in part"
        } else {
            "owed nowhere"
        };
        *kinds.entry(kind).or_default() += 1;
        let family = |f: usize| {
            json!({
                "groups": g.families[f],
                "owed": share[f].owed,
                "an_unlicensed_instance_is_about": share[f].about,
            })
        };
        grid_rows.push(json!({
            "grid": k,
            "component": g.component,
            "members": members.len(),
            "open": members.iter().filter(|&&b| c.marks[b as usize] == Mark::AliveAlone).count(),
            "cells": g.families[0].len() * g.families[1].len(),
            "relations": relations_of(members),
            "families": [family(0), family(1)],
            "kind": kind,
            "bijection": x.is_bijection(k as u32),
        }));
        grid_counts.push(json!({
            "relations": relations_of(members),
            "members": members.len(),
            "groups": [share[0].groups, share[1].groups],
            "owed": [share[0].owed, share[1].owed],
            "about": [share[0].about, share[1].about],
            "bijection": x.is_bijection(k as u32),
        }));
    }
    let mut not_why: BTreeMap<&'static str, usize> = BTreeMap::new();
    let not_grids: Vec<Value> = x
        .grids
        .not
        .iter()
        .map(|&(component, why)| {
            *not_why.entry(why.as_str()).or_default() += 1;
            let members = members_of(component);
            let mut groups: Vec<u32> = members
                .iter()
                .flat_map(|&b| c.groups_of(b).iter().copied())
                .collect();
            groups.sort_unstable();
            groups.dedup();
            json!({
                "component": component,
                "members": members.len(),
                "groups": groups,
                "owed": groups.iter().filter(|&&g| x.is_owed(g)).count(),
                "relations": relations_of(members),
                "why": why.as_str(),
                "detail": match why {
                    NotAGrid::MoreThanTwo { most } => json!({"most": most}),
                    NotAGrid::InOne { members } => json!({"members_in_one_group": members}),
                    _ => Value::Null,
                },
            })
        })
        .collect();

    let no_table = x.owed.instances.as_ref().err().map(|e| e.as_str());
    let raw = json!({
        "program": rel,
        "n": c.n(),
        "domain": c.domain(),
        "root_owes": x.owed.root_owes,
        "no_table": no_table,
        "singles_read": x.owed.singles_read,
        "theory_read": x.owed.theory_read,
        "instances": instances,
        "groups": groups,
        "owed_groups": x.owed_groups(),
        "groups_nothing_owes": unowed,
        "owed_sets_not_a_group": x.not_groups(),
        "unlicensed": x.unlicensed(),
        "grids": grid_rows,
        "components_that_are_no_grid": not_grids,
    });
    let file = dir.join("exactly_one.json");
    let text = serde_json::to_string_pretty(&raw).expect("JSON");
    std::fs::write(&file, text + "\n").expect("exactly_one.json");

    // The same as tables.
    let mut md = format!("# Exactly one, on `{rel}`\n\n");
    if let Some(d) = c.domain() {
        md.push_str(&format!("**{d}.**\n\n"));
    }
    match no_table {
        Some(why) => md.push_str(&format!(
            "Root owes {} instance(s) and no table of alternatives is read: {why}.\n\n",
            x.owed.root_owes
        )),
        None => md.push_str(&format!(
            "Root owes {} instance(s); the table lists {}.\n\n",
            x.owed.root_owes,
            table.len()
        )),
    }
    md.push_str("| | |\n|---|---:|\n");
    let mut line = |what: &str, n: usize| md.push_str(&format!("| {what} | {n} |\n"));
    line("groups", c.groups.len());
    line("**owed groups**: exactly one", x.owed_groups().len());
    line("**groups nothing owes**: at most one", unowed.len());
    line("… with an open member", unowed_open);
    line("… an unlicensed instance is about", unowed_about);
    line(
        "**owed sets that are not a group**: at least one",
        x.not_groups().len(),
    );
    line("instances L5 does not license", x.unlicensed().len());
    line("grids", x.grids.grids.len());
    line(
        "… bijections",
        (0..x.grids.grids.len() as u32)
            .filter(|&k| x.is_bijection(k))
            .count(),
    );
    line("components that are no grid", x.grids.not.len());
    let shown = |b: u32| {
        format!(
            "`{}`{}",
            c.text(b),
            match c.marks[b as usize] {
                Mark::DeadAlone => "†",
                Mark::AtRoot => "‡",
                _ => "",
            }
        )
    };
    let listed = |v: &[u32]| v.iter().map(|&b| shown(b)).collect::<Vec<_>>().join(" ");
    md.push_str(
        "\n## The groups\n\nA member dead alone at root is marked †. *Owed by* is the licensed \
         instance whose alternatives are exactly the members; *about* an unlicensed instance \
         whose witnesses the members, and no other candidate, bring by the theory alone.\n\n\
         | # | grid, family | open | owed by | about | members |\n|---:|---|---:|---|---|---|\n",
    );
    for g in 0..c.groups.len() as u32 {
        let by = |v: &[u32]| {
            v.iter()
                .map(|&k| format!("`{}`", table[k as usize].text))
                .collect::<Vec<_>>()
                .join(" ")
        };
        md.push_str(&format!(
            "| {g} | {} | {} | {} | {} | {} |\n",
            grid_of(g).map_or(String::new(), |(grid, f)| format!("{grid}, {f}")),
            open(g),
            by(&x.owed_by[g as usize]),
            by(&x.about[g as usize]),
            listed(c.members(g)),
        ));
    }
    if !table.is_empty() {
        md.push_str(
            "\n## What root owes\n\n| # | instance | stands as | alternatives, or why none is \
             known | witnesses: candidates, denied at root, neither (of them brought by a \
             candidate at root) |\n|---:|---|---|---|---|\n",
        );
        for (k, (i, owing)) in table.iter().zip(&x.owing).enumerate() {
            let stands = match owing {
                Owing::Group(g) => format!("group {g}: exactly one"),
                Owing::Unlicensed { about: Some(g), .. } => format!("unlicensed, about group {g}"),
                other => other.as_str().to_string(),
            };
            let what = match &i.alternatives {
                Ok(alts) => listed(alts),
                Err(why) => why.clone(),
            };
            let ws = match &i.witnesses {
                Err(why) => why.clone(),
                Ok(ws) => {
                    let count =
                        |is: fn(&WitnessIs) -> bool| ws.iter().filter(|w| is(&w.is)).count();
                    let neither: Vec<&structure::Witness> = ws
                        .iter()
                        .filter(|w| w.is == WitnessIs::Unproposed)
                        .collect();
                    format!(
                        "{}, {}, {} ({})",
                        count(|w| matches!(w, WitnessIs::Candidate(_))),
                        count(|w| *w == WitnessIs::Denied),
                        neither.len(),
                        neither
                            .iter()
                            .filter(|w| !w.held_at_root.is_empty())
                            .count()
                    )
                }
            };
            md.push_str(&format!(
                "| {k} | `{}` | {stands} | {what} | {ws} |\n",
                i.text
            ));
        }
    }
    let unlicensed = x.unlicensed();
    if !unlicensed.is_empty() {
        md.push_str(
            "\n## The witnesses of the instances L5 does not license\n\n| instance | witness | \
             it is | held at root by the node of | held in the small KB by the node of |\n\
             |---:|---|---|---|---|\n",
        );
        for &k in &unlicensed {
            if let Ok(ws) = &table[k as usize].witnesses {
                for w in ws {
                    md.push_str(&format!(
                        "| {k} | `{}` | {} | {} | {} |\n",
                        w.text,
                        w.is.as_str(),
                        listed(&w.held_at_root),
                        listed(&w.held_in_theory)
                    ));
                }
            }
        }
    }
    if !x.grids.grids.is_empty() {
        md.push_str(
            "\n## The grids\n\nA family's groups, then how many of them are owed, and of the \
             others how many an unlicensed instance is about.\n\n| # | members | open | \
             relations | family 0 | owed | about | family 1 | owed | about | |\n\
             |---:|---:|---:|---|---|---:|---:|---|---:|---:|---|\n",
        );
        for row in &grid_rows {
            let f = |k: usize, name: &str| row["families"][k][name].to_string();
            let groups = |k: usize| {
                let n = row["families"][k]["groups"].as_array().map_or(0, Vec::len);
                format!("{n}: {}", f(k, "groups"))
            };
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                row["grid"],
                row["members"],
                row["open"],
                row["relations"]
                    .as_array()
                    .map(|r| r
                        .iter()
                        .map(|x| format!("`{}`", x.as_str().unwrap_or("")))
                        .collect::<Vec<_>>()
                        .join(" "))
                    .unwrap_or_default(),
                groups(0),
                f(0, "owed"),
                f(0, "an_unlicensed_instance_is_about"),
                groups(1),
                f(1, "owed"),
                f(1, "an_unlicensed_instance_is_about"),
                row["kind"].as_str().unwrap_or(""),
            ));
        }
    }
    if !not_grids.is_empty() {
        md.push_str(
            "\n## The components that are no grid\n\n| component | members | groups | owed | \
             relations | why |\n|---:|---:|---:|---:|---|---|\n",
        );
        for row in &not_grids {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {}{} |\n",
                row["component"],
                row["members"],
                row["groups"].as_array().map_or(0, Vec::len),
                row["owed"],
                row["relations"]
                    .as_array()
                    .map(|r| r
                        .iter()
                        .map(|x| format!("`{}`", x.as_str().unwrap_or("")))
                        .collect::<Vec<_>>()
                        .join(" "))
                    .unwrap_or_default(),
                row["why"].as_str().unwrap_or(""),
                match &row["detail"] {
                    Value::Null => String::new(),
                    d => format!(" ({d})"),
                }
            ));
        }
    }
    std::fs::write(dir.join("exactly_one.md"), md).expect("exactly_one.md");

    json!({
        "table": x.owed.instances.as_ref().ok().map(Vec::len),
        "no_table": no_table,
        "root_owes": x.owed.root_owes,
        "groups": c.groups.len(),
        "owed_groups": x.owed_groups().len(),
        "groups_nothing_owes": {
            "all": unowed.len(),
            "with_an_open_member": unowed_open,
            "an_unlicensed_instance_is_about": unowed_about,
        },
        "owed_sets_not_a_group": x.not_groups().len(),
        "unlicensed": {
            "all": x.unlicensed().len(),
            "why": unlicensed_why,
            "about_a_group": about_a_group,
            "witnesses_not_read": unknown,
            "witnesses_no_candidate_and_not_denied": unproposed,
            "of_them_brought_by_a_candidate_at_root": brought,
            "not_brought_and_the_theorys_bringers_all_dead_alone_at_root": bringers_dead,
            "not_brought_and_brought_by_nothing_in_the_theory": brought_nowhere,
            "every_such_witness_brought": all_brought,
            "some_such_witness_not_brought": some_not_brought,
            "no_such_witness": none_unproposed,
        },
        "grids": grid_counts,
        "bijections": (0..x.grids.grids.len() as u32).filter(|&k| x.is_bijection(k)).count(),
        "grid_kinds": kinds,
        "components": c.components.len(),
        "not_grids": not_why,
        "lists": file.strip_prefix(repo_root()).unwrap_or(&file).to_string_lossy(),
    })
}

/// The cover of one program: its files written under `dir`, and the census
/// line's `cover` object.
fn cover_row(path: &Path, rel: &str, dir: &Path, t: &Theory, terms: &Terms) -> Value {
    let Some((c, exact)) = t.exactly_one() else {
        return json!({"error": t.small.as_ref().err()});
    };
    let n = c.n();
    let one = |v: &[u32]| -> Vec<&str> { v.iter().map(|&b| c.text(b)).collect() };
    let none = c.in_no_group();
    let marked = |m: Mark| none.iter().filter(|&&b| c.marks[b as usize] == m).count();
    let killed = |b: &u32| c.killed_alone.binary_search(b).is_ok();
    let per: Vec<usize> = c.member_of.iter().map(Vec::len).collect();
    let open: Vec<Vec<u32>> = (0..c.groups.len() as u32)
        .map(|g| c.open(g).collect())
        .collect();
    let components_are_groups = c.components == c.groups;
    let (declared, owed, instances) = declared_beside(path, &c, &exact, terms);
    let exactly_one = exactly_one_row(rel, dir, &c, &exact, terms);

    let raw = json!({
        "program": rel,
        "n": n,
        "complete": c.complete(),
        "monotone": c.monotone,
        "small_monotone": c.small_monotone,
        "domain": c.domain(),
        "stopped": stopped(&c.stopped),
        "bound": structure::CLIQUE_STEPS,
        "steps": c.steps,
        "cut": cut_of(&c.cut),
        "candidates": (0..n as u32).map(|b| json!({
            "candidate": c.text(b),
            "at_root": c.marks[b as usize].as_str(),
            "killed_by_the_theory_alone": killed(&b),
            "groups": c.groups_of(b),
        })).collect::<Vec<_>>(),
        "groups": c.groups.iter().zip(&open).map(|(g, o)| json!({
            "members": one(g),
            "open": one(o),
        })).collect::<Vec<_>>(),
        "in_no_group": one(&none),
        "components": c.components.iter().map(|x| one(x)).collect::<Vec<_>>(),
        "pairs_alive_at_root": c.alive_at_root.as_ref().map(|v| {
            v.iter().map(|&(a, b)| [c.text(a), c.text(b)]).collect::<Vec<_>>()
        }),
        "root_cliques": c.root_cliques.as_ref().map(|r| json!({
            "steps": r.steps,
            "cut": cut_of(&r.cut),
            "cliques": r.cliques.iter().map(|x| one(x)).collect::<Vec<_>>(),
        })),
        "declared": declared,
        "owed": owed,
        "owed_instances": instances,
    });
    let file = dir.join("cover.json");
    let text = serde_json::to_string_pretty(&raw).expect("JSON");
    std::fs::write(&file, text + "\n").expect("cover.json");

    // The same as tables.
    let mut md = format!("# The cover of `{rel}`\n\n");
    if let Some(d) = c.domain() {
        md.push_str(&format!("**{d}.**\n\n"));
    }
    md.push_str("| | |\n|---|---:|\n");
    let mut line = |what: &str, n: String| md.push_str(&format!("| {what} | {n} |\n"));
    line("candidates, H", n.to_string());
    line("pairs the theory excludes", c.excluded.len().to_string());
    line("groups", c.groups.len().to_string());
    line(
        "their sizes (size × groups)",
        histogram(c.groups.iter().map(Vec::len))
            .iter()
            .map(|[k, g]| format!("{k} × {g}"))
            .collect::<Vec<_>>()
            .join(", "),
    );
    line(
        "candidates by the number of groups they are in (groups × candidates)",
        histogram(per.iter().copied())
            .iter()
            .map(|[k, g]| format!("{k} × {g}"))
            .collect::<Vec<_>>()
            .join(", "),
    );
    line("candidates in no group", none.len().to_string());
    line(
        "… alive alone at root",
        marked(Mark::AliveAlone).to_string(),
    );
    line("… dead alone at root", marked(Mark::DeadAlone).to_string());
    line(
        "… the theory kills alone",
        none.iter().filter(|b| killed(b)).count().to_string(),
    );
    line(
        "components of the theory's exclusion graph",
        c.components.len().to_string(),
    );
    line(
        "calls of the enumeration, of the bound",
        format!("{} of {}", c.steps, structure::CLIQUE_STEPS),
    );
    line(
        "what the bound cut",
        match &c.cut {
            None => "nothing".to_string(),
            Some(x) => format!("{} pairs, {} candidates", x.pairs, x.candidates),
        },
    );
    line(
        "the theory's pairs alive at root",
        c.alive_at_root
            .as_ref()
            .map_or("root's pairs not read".to_string(), |v| v.len().to_string()),
    );
    line(
        "maximal cliques of root's exclusion graph, beside",
        c.root_cliques
            .as_ref()
            .map_or("root's pairs not read".to_string(), |r| {
                r.cliques.len().to_string()
            }),
    );
    md.push_str(
        "\n## The groups\n\nA member dead alone at root is marked †, one root's own fixpoint \
         holds ‡.\n\n| # | members | open | the members |\n|---:|---:|---:|---|\n",
    );
    let shown = |b: u32| {
        format!(
            "`{}`{}",
            c.text(b),
            match c.marks[b as usize] {
                Mark::DeadAlone => "†",
                Mark::AtRoot => "‡",
                _ => "",
            }
        )
    };
    for (g, members) in c.groups.iter().enumerate() {
        md.push_str(&format!(
            "| {g} | {} | {} | {} |\n",
            members.len(),
            open[g].len(),
            members
                .iter()
                .map(|&b| shown(b))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    md.push_str("\n## The candidates\n\n| candidate | at root | groups |\n|---|---|---|\n");
    for b in 0..n as u32 {
        md.push_str(&format!(
            "| `{}` | {}{} | {} |\n",
            c.text(b),
            c.marks[b as usize].as_str(),
            if killed(&b) {
                ", killed by the theory alone"
            } else {
                ""
            },
            c.groups_of(b)
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !instances.is_empty() {
        md.push_str(
            "\n## What root owes, beside\n\n| instance | its alternatives are | alternatives |\n|---|---|---|\n",
        );
        for i in &instances {
            let read = &i["read"];
            let what = match read["alternatives"].as_array() {
                Some(a) => a
                    .iter()
                    .map(|x| format!("`{}`", x.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
                    .join(" "),
                None => read["why"].as_str().unwrap_or("").to_string(),
            };
            let standing = match (i["standing"].as_str(), read["detail"]["group"].as_u64()) {
                (Some("a group"), Some(g)) => format!("group {g}"),
                (Some(s), _) => s.to_string(),
                _ => String::new(),
            };
            md.push_str(&format!(
                "| `{}` | {standing} | {what} |\n",
                i["instance"].as_str().unwrap_or("")
            ));
        }
    }
    std::fs::write(dir.join("cover.md"), md).expect("cover.md");

    json!({
        "complete": c.complete(),
        "pairs": c.excluded.len(),
        "groups": c.groups.len(),
        "sizes": histogram(c.groups.iter().map(Vec::len)),
        "open": histogram(open.iter().map(Vec::len)),
        "membership": histogram(per.iter().copied()),
        "most": per.iter().max().copied().unwrap_or(0),
        "mean": if n > 0 { json!(per.iter().sum::<usize>() as f64 / n as f64) } else { Value::Null },
        "in_no_group": {
            "all": none.len(),
            "alive_alone": marked(Mark::AliveAlone),
            "dead_alone": marked(Mark::DeadAlone),
            "at_root": marked(Mark::AtRoot),
            "unread": marked(Mark::Unread),
            "killed_by_the_theory_alone": none.iter().filter(|b| killed(b)).count(),
        },
        "components": c.components.len(),
        "components_are_groups": components_are_groups,
        "bound": structure::CLIQUE_STEPS,
        "steps": c.steps,
        "cut": cut_of(&c.cut),
        "pairs_alive_at_root": c.alive_at_root.as_ref().map(Vec::len),
        "root_cliques": c.root_cliques.as_ref().map(|r| clique_counts(r, n)),
        "declared": declared,
        "owed": owed,
        "exactly_one": exactly_one,
        "lists": file.strip_prefix(repo_root()).unwrap_or(&file).to_string_lossy(),
    })
}
