//! `ein solve`'s region modes — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4: `--minimal`, `--maximal`, `--single` and `--region-budget`,
//! with `-n`, answered by [`ein_infer::explore`] and, since
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md),
//! printed in the read-out of
//! [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)
//! by [`ein_render::read_out`].
//!
//! **Until S1e2.1a.5's switch the default is still the walk** (ruling 4: the
//! goldens move once). One of the four region flags is therefore what takes a
//! run here, and every option of the walk's is refused beside them rather than
//! ignored: a flag that does nothing is a claim nobody checks.
//!
//! The program is loaded and solved **once**, and every `(query …)` it holds
//! is answered from that one solve (the diff's § 7.1).

use std::path::Path;
use std::time::{Duration, Instant};

use clap::ArgMatches;
use clap::parser::ValueSource;
use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::read_out::{self, ReadOut};
use ein_ir::Ast;

/// The `solve` options a region mode takes. Every other one is the walk's.
const TAKES: [&str; 20] = [
    "file",
    "solutions",
    "max-time",
    "jobs",
    "region-limit",
    "stats",
    "minimal",
    "maximal",
    "single",
    "region-budget",
    "dump-answer",
    "json-summary",
    "models",
    "print-final-state",
    "print-final-positive",
    "print-final-hfacts",
    "diff-models",
    "trace",
    "no-diagrams",
    "hyp-stats",
];

/// Whether this `solve` run asks a region mode.
pub fn wanted(m: &ArgMatches) -> bool {
    m.get_flag("minimal")
        || m.get_flag("maximal")
        || m.get_flag("single")
        || m.get_one::<i64>("region-budget").is_some()
}

/// The mode the flags ask for.
pub fn mode_of(m: &ArgMatches) -> Mode {
    let which = if m.get_flag("minimal") {
        Which::Minimal
    } else if m.get_flag("maximal") {
        Which::Maximal
    } else {
        Which::All
    };
    let ask = if m.get_flag("single") {
        Ask::Single
    } else if m.value_source("solutions") == Some(ValueSource::CommandLine) {
        Ask::Find(*m.get_one::<i64>("solutions").expect("defaulted") as u64)
    } else {
        Ask::Describe
    };
    Mode { which, ask }
}

/// The options the header leaves out, with their values: `-j`, because the
/// same question at another `-j` prints the same lines (C4), and the files a
/// run writes, because writing one changes nothing printed. `--no-diagrams`,
/// which shapes the trace alone, is left out too, as a flag with no value.
const UNSHOWN: [&str; 5] = ["-j", "--jobs", "--json-summary", "--dump-answer", "--trace"];

/// The arguments after `command`, as typed, less [`UNSHOWN`]: the header
/// names the question.
pub fn header_argv(command: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    let start = args
        .iter()
        .position(|a| a == command)
        .map_or(args.len(), |i| i + 1);
    let mut shown: Vec<&str> = Vec::new();
    let mut it = args[start..].iter();
    while let Some(a) = it.next() {
        if a == "--no-diagrams" {
            continue;
        }
        if UNSHOWN.contains(&a.as_str()) {
            it.next();
            continue;
        }
        if UNSHOWN.iter().any(|u| a.starts_with(&format!("{u}=")))
            || (a.starts_with("-j") && a.len() > 2)
        {
            continue;
        }
        shown.push(a);
    }
    shown.join(" ")
}

pub fn run(m: &ArgMatches) -> i32 {
    let solve = crate::cmdline::command();
    let solve = solve.find_subcommand("solve").expect("solve is registered");
    for arg in solve.get_arguments() {
        let id = arg.get_id().as_str();
        if TAKES.contains(&id) || m.value_source(id) != Some(ValueSource::CommandLine) {
            continue;
        }
        let name = arg
            .get_long()
            .map(|l| format!("--{l}"))
            .unwrap_or_else(|| id.to_string());
        eprintln!(
            "error: {name} is the walk's: the region modes (--minimal, --maximal, --single, \
             --region-budget) do not take it until S1e2.1a.5 says what it means there"
        );
        return 2;
    }
    let file = m.get_one::<String>("file").expect("required").clone();
    let mut opts = explore::Options::new(mode_of(m));
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    opts.limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    opts.jobs = crate::solve::resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1));
    let max_time = m.get_one::<f64>("max-time").copied();
    opts.deadline = max_time.map(|s| Instant::now() + Duration::from_secs_f64(s.max(0.0)));

    // `--dump-answer` makes its directory before the run, as the walk's
    // does, so a path that cannot be written fails before the work.
    let dump = m.get_one::<String>("dump-answer").cloned();
    if let Some(dir) = dump.as_deref() {
        // Every state file starts with the program's source, which a
        // container does not carry.
        if crate::common::looks_like_einb(&crate::common::read_bytes_or_crash(Path::new(&file))) {
            eprintln!(
                "error: --dump-answer {dir}: a .einb container carries no program source, and \
                 every file the answer directory holds starts with it — run it on the .ein"
            );
            return 2;
        }
        if let Err(e) = std::fs::create_dir_all(dir) {
            crate::solve::artefact_failed("dump-answer", dir, e);
            return 1;
        }
    }
    let summary_path = m.get_one::<String>("json-summary").cloned();
    // `--diff-models FILE` is read before the run, as the walk's is: a file
    // it cannot read is a usage error, and nothing runs.
    let want = match m.get_one::<String>("diff-models") {
        None => None,
        Some(path) => match std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|t| ein_render::models::parse_expected(&t))
        {
            Ok(e) => Some((path.clone(), e)),
            Err(e) => {
                eprintln!("error: --diff-models {path}: {e}");
                return 2;
            }
        },
    };

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = crate::common::load_any_or_exit(&mut ast, &mut terms, Path::new(&file))
    else {
        return 1;
    };
    // `--trace` tells root's derivation first, so root is saturated here, with
    // its firings kept; the run's own preparation then finds it saturated.
    let trace = m.get_one::<String>("trace").cloned();
    let root_firings = match trace {
        Some(_) => match ein_infer::region::root_firings(&mut kb, &mut terms, &ast) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        },
        None => None,
    };
    let r = match read_out::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let code = print(
        &r,
        "solve",
        &file,
        opts.budget,
        max_time,
        m.get_flag("stats"),
    );
    if code == 2 {
        return code;
    }
    if let Err(e) = extras(m, &r, &mut kb, &mut terms, &ast, want.as_ref()) {
        eprintln!("{e}");
        return 1;
    }
    if let (Some(path), Some(firings)) = (trace.as_deref(), root_firings.as_deref()) {
        let diagrams = !m.get_flag("no-diagrams");
        let written = trace_markdown(
            &r,
            &mut kb,
            &mut terms,
            &ast,
            firings,
            &file,
            opts.budget,
            diagrams,
        )
        .and_then(|md| std::fs::write(path, md).map_err(|e| e.to_string()));
        if let Err(e) = written {
            crate::solve::artefact_failed("trace", path, e);
            return 1;
        }
    }
    if summary_path.is_none() && dump.is_none() {
        return code;
    }
    // The summary and the answer directory: the same facts as stdout, as
    // files. Additive: nothing printed above moves with them.
    let summary = match crate::summary::build_region(&ast, &mut terms, &mut kb, &r, &file) {
        Ok(s) => crate::summary::text(&s),
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    if let Some(path) = summary_path.as_deref()
        && let Err(e) = std::fs::write(path, &summary)
    {
        crate::solve::artefact_failed("json-summary", path, e);
        return 1;
    }
    if let Some(dir) = dump.as_deref() {
        let program = crate::solve::inlined_program(&file);
        let answer_txt = uncut(&r, &file, opts.budget);
        let inputs = ein_render::dump::region_answer::Inputs {
            source: &file,
            program: &program,
            summary: &summary,
            answer_txt: &answer_txt,
        };
        if let Err(e) = ein_render::dump::region_answer::write(
            Path::new(dir),
            &r,
            &mut kb,
            &mut terms,
            &ast,
            &inputs,
        ) {
            crate::solve::artefact_failed("dump-answer", dir, e);
            return 1;
        }
    }
    code
}

/// What a region run prints after its read-out when asked, over **the states
/// it listed** — `read_out.md` § 8.3 and § 8.6: `--models key|grid`, then
/// `-p`, `-P` and `-f` per state, then `--diff-models FILE`. Each block
/// says which set it is drawn from. These are asked for by name, so the
/// shared 100 lines do not cut them.
fn extras(
    m: &ArgMatches,
    r: &ReadOut,
    kb: &mut ein_core::Kb,
    terms: &mut Terms,
    ast: &Ast,
    want: Option<&(String, ein_render::models::Expected)>,
) -> Result<(), String> {
    use crate::factdump::{Mode as Dump, print_final_state};
    let form = m.get_one::<String>("models").map(String::as_str);
    let dumps: Vec<Dump> = [
        ("print-final-state", Dump::All),
        ("print-final-positive", Dump::Positive),
        ("print-final-hfacts", Dump::Hfacts),
    ]
    .into_iter()
    .filter(|(flag, _)| m.get_flag(flag))
    .map(|(_, d)| d)
    .collect();
    if m.get_flag("hyp-stats") {
        universe(r, terms);
    }
    if !matches!(form, Some("key" | "grid")) && dumps.is_empty() && want.is_none() {
        return Ok(());
    }
    let a = &r.answer;
    let listed: Vec<(usize, &ein_infer::read_out::Listed)> = r.listed().collect();
    let what = match (r.described().is_some(), &r.listing) {
        (true, ein_infer::read_out::Listing::Region) => "every state of R".to_string(),
        (true, _) => "the line members this run certified".to_string(),
        (false, _) => format!(
            "the {} listed",
            if listed.len() == 1 { "state" } else { "states" }
        ),
    };
    let sets: Vec<Vec<ein_core::FactId>> = listed
        .iter()
        .map(|(_, s)| {
            s.rep
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect()
        })
        .collect();
    // The candidates' relations stand in for `:hrules`' targets: they are
    // what a commitment is made of.
    let mut targets: Vec<ein_core::Symbol> = Vec::new();
    for c in &a.candidates {
        let rel = terms.facts.get(c.fact).0;
        if !targets.contains(&rel) {
            targets.push(rel);
        }
    }
    let n = listed.len();
    let exhausted = a.exhausted;
    let mut nodes =
        ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates).map_err(|e| e.to_string())?;
    nodes
        .with_fixpoints(&sets, |kbs, terms| {
            let label = |i: usize| format!("state {} of {}", listed[i].0 + 1, r.states.len());
            match form {
                Some("key") => {
                    println!();
                    println!("models — the determining key of {what}");
                    let refs: Vec<&ein_core::Kb> = kbs.iter().collect();
                    let of = ein_render::models::KeyOf::States {
                        maximal: listed.iter().all(|(_, s)| s.maximal),
                    };
                    match ein_render::models::key_table_of(terms, &refs, exhausted, "  ", of) {
                        ein_render::models::KeyOutcome::Table(rows) => {
                            for l in rows {
                                println!("{l}");
                            }
                        }
                        ein_render::models::KeyOutcome::Unaffordable(why) => {
                            println!("  determining key — none within budget");
                            for l in ein_render::models::wrap(&format!("{why}."), "    ") {
                                println!("{l}");
                            }
                        }
                    }
                }
                Some("grid") => {
                    println!();
                    println!("models — {what}, each as a grid");
                    for (i, kb) in kbs.iter().enumerate() {
                        println!();
                        println!("  ── {} ──", label(i));
                        for l in ein_render::grid::grid_or_reason(terms, kb, "  ") {
                            println!("{l}");
                        }
                    }
                }
                _ => {}
            }
            for (i, kb) in kbs.iter().enumerate() {
                if dumps.is_empty() {
                    break;
                }
                println!();
                println!("── {} ──", label(i));
                for d in &dumps {
                    print_final_state(terms, kb, *d, &targets);
                }
            }
            if let Some((path, want)) = want {
                use ein_render::models::{comparable_facts, diff_lines, diff_model_sets};
                let closes = want.closes.as_deref();
                let got: Vec<Vec<String>> = kbs
                    .iter()
                    .map(|kb| comparable_facts(terms, kb, closes))
                    .collect();
                let d = diff_model_sets(&got, &want.models);
                let over = match closes {
                    Some(c) => format!("compared over {}", c.join(" ")),
                    None => "compared as positive facts minus the given ones".to_string(),
                };
                let head = format!("{what} against {path}, {over}");
                let got_label = |i: usize| label(i);
                let want_label = |j: usize| {
                    let facts = &want.models[j];
                    format!(
                        "model {} of the file [{}]",
                        j + 1,
                        if facts.is_empty() {
                            "no facts".to_string()
                        } else {
                            facts.join(" ")
                        }
                    )
                };
                println!();
                for l in diff_lines(&d, &head, &got_label, &want_label, "") {
                    println!("{l}");
                }
            }
            let _ = n;
        })
        .map_err(|e| e.to_string())
}

/// `-H`: H, the universe — every candidate, by relation, and those dead
/// alone, which no state of R can hold (`read_out.md` § 8.6, P1e2.3's S4).
fn universe(r: &ReadOut, terms: &Terms) {
    let a = &r.answer;
    let dead: Option<&[u32]> = r.described().map(|d| d.dead_alone_set.as_slice());
    let mut by_rel: Vec<(String, Vec<(usize, &str)>)> = Vec::new();
    for (i, c) in a.candidates.iter().enumerate() {
        let rel = terms.sym(terms.facts.get(c.fact).0).to_string();
        match by_rel.iter_mut().find(|(r, _)| *r == rel) {
            Some((_, v)) => v.push((i, c.text.as_str())),
            None => by_rel.push((rel, vec![(i, c.text.as_str())])),
        }
    }
    by_rel.sort_by(|x, y| x.0.cmp(&y.0));
    let is_dead = |i: usize| dead.is_some_and(|d| d.contains(&(i as u32)));
    let n_dead = dead.map_or(0, <[u32]>::len);
    println!();
    println!(
        "H — the universe: {} candidate{} over {} relation{}{}",
        a.candidates.len(),
        if a.candidates.len() == 1 { "" } else { "s" },
        by_rel.len(),
        if by_rel.len() == 1 { "" } else { "s" },
        match dead {
            Some(_) => format!(", {n_dead} dead alone"),
            None =>
                ", dead alone not known: a mode does not visit every candidate alone".to_string(),
        }
    );
    for (rel, cands) in &by_rel {
        let d = cands.iter().filter(|(i, _)| is_dead(*i)).count();
        println!(
            "  {rel:<16}{} candidate{}{}",
            cands.len(),
            if cands.len() == 1 { "" } else { "s" },
            if dead.is_some() {
                format!(", {d} dead alone")
            } else {
                String::new()
            }
        );
        let width = cands
            .iter()
            .map(|(_, t)| t.chars().count())
            .max()
            .unwrap_or(0);
        for (i, text) in cands {
            if is_dead(*i) {
                println!("    {text:<width$}   dead alone");
            } else {
                println!("    {text}");
            }
        }
    }
}

/// `--trace FILE.md`: root's derivation, then each state stdout printed,
/// derived from its first minimal commitment, then N and the owing frontier
/// (`read_out.md` § 8.4).
#[allow(clippy::too_many_arguments)]
fn trace_markdown(
    r: &ReadOut,
    kb: &mut ein_core::Kb,
    terms: &mut Terms,
    ast: &Ast,
    root_firings: &[ein_infer::Firing],
    file: &str,
    budget: u64,
    diagrams: bool,
) -> Result<String, String> {
    use ein_render::trace::region::{DeadTrace, RegionTrace, StateTrace, region_markdown};
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let name = |set: &[u32]| -> String {
        if set.is_empty() {
            "∅".to_string()
        } else {
            let v: Vec<&str> = set.iter().map(|&b| cands[b as usize].as_str()).collect();
            v.join(", ")
        }
    };
    let set_name = |set: &[u32]| -> String {
        if set.is_empty() {
            "∅".to_string()
        } else {
            format!("{{{}}}", name(set))
        }
    };
    let text = ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &header_argv("solve"),
            file,
            budget,
            uncut: false,
        },
    );
    let head: Vec<String> = text
        .stdout
        .lines()
        .skip(2)
        .take_while(|l| !l.is_empty())
        .map(str::to_string)
        .collect();
    let root = ein_render::trace::steps_of(ast, terms, root_firings, kb, diagrams, &[]);
    let mut states = Vec::new();
    let k = r.states.len();
    {
        let mut nodes = ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates)
            .map_err(|e| e.to_string())?;
        for &i in &text.shown {
            let s = &r.states[i];
            let from = s.generators.first().unwrap_or(&s.rep).clone();
            let facts: Vec<ein_core::FactId> = from
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect();
            let steps = nodes
                .with_fixpoint_traced(&facts, |kb, terms, firings| {
                    ein_render::trace::steps_of(ast, terms, firings, kb, diagrams, &facts)
                })
                .map_err(|e| e.to_string())?;
            states.push(StateTrace {
                title: ein_render::read_out::state_title(i, k, s, &cands),
                commitment: name(&from),
                steps,
            });
        }
    }
    let dead = r
        .described()
        .map(|d| {
            d.dead
                .iter()
                .map(|x| DeadTrace {
                    set: set_name(&x.set),
                    core: x.core.clone(),
                    refuted_by: x.refuted_by.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let owing = match &r.listing {
        ein_infer::read_out::Listing::Owing(f) => f
            .iter()
            .map(|o| (set_name(&o.node.set), o.node.owed.clone()))
            .collect(),
        _ => Vec::new(),
    };
    let listed = r.listed().count();
    Ok(region_markdown(
        &RegionTrace {
            head,
            root,
            untraced: listed.saturating_sub(states.len()),
            states,
            dead,
            owing,
        },
        diagrams,
    ))
}

/// `answer.txt`: the read-out with no listing cut — where stdout's cut says
/// the rest is (the user's ruling of 2026-10-16, the diff's § 4.1).
fn uncut(r: &ReadOut, file: &str, budget: u64) -> String {
    let argv = header_argv("solve");
    ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &argv,
            file,
            budget,
            uncut: true,
        },
    )
    .stdout
}

/// Print `r`, write its stderr lines, and return its exit code. `-T` aborts
/// with 2 before anything is printed.
pub fn print(
    r: &ReadOut,
    command: &str,
    file: &str,
    budget: u64,
    max_time: Option<f64>,
    stats: bool,
) -> i32 {
    if r.answer.stopped == Some(Stopped::Deadline) {
        eprintln!(
            "** aborted: --max-time {} s reached before the question was decided **",
            max_time.unwrap_or(0.0)
        );
        return 2;
    }
    let argv = header_argv(command);
    let text = ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command,
            argv: &argv,
            file,
            budget,
            uncut: false,
        },
    );
    print!("{}", text.stdout);
    if stats {
        println!();
        println!(
            "  {:<18}{:.1} ms",
            "wall",
            r.answer.wall.as_secs_f64() * 1e3
        );
    }
    for l in &text.stderr {
        eprintln!("{l}");
    }
    text.code
}
