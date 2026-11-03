//! `ein solve`'s region run, the only one since M1e2
//! [S1e2.1a.5](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! switch: bare `ein solve FILE` describes R, and `--minimal`, `--maximal`,
//! `--single` and `-n N` each ask one question about it, answered by
//! [`ein_infer::explore`] and printed in the read-out of
//! [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)
//! by [`ein_render::read_out`]. The modes were built at S1e2.1a.9's
//! T1e2.1a.9.4, and reached through a region flag until the switch.
//!
//! The program is loaded and solved **once**, and every `(query …)` it holds
//! is answered from that one solve (the diff's § 7.1).

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clap::ArgMatches;
use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::read_out::{self, ReadOut};
use ein_ir::Ast;

/// The mode the flags ask for.
pub fn mode_of(m: &ArgMatches) -> Mode {
    let which = if m.get_flag("minimal") {
        Which::Minimal
    } else if m.get_flag("maximal") {
        Which::Maximal
    } else {
        Which::All
    };
    let ask = if m.get_flag("count") {
        Ask::Count
    } else if m.get_flag("single") {
        Ask::Single
    } else if let Some(&n) = m.get_one::<i64>("solutions") {
        Ask::Find(n as u64)
    } else {
        Ask::Describe
    };
    Mode { which, ask }
}

/// The options the header leaves out, with their values: `-j`, because the
/// same question at another `-j` prints the same lines (C4), and the files a
/// run writes, because writing one changes nothing printed. The flags that
/// shape the trace alone, or what goes to stderr, are left out too
/// ([`UNSHOWN_FLAGS`]).
const UNSHOWN: [&str; 9] = [
    "-j",
    "--jobs",
    "--json-summary",
    "--dump-answer",
    "--dump-lattice",
    "--trace",
    "--events",
    "--events-level",
    "--progress-every",
];

/// [`UNSHOWN`]'s flags with no value: the trace's shape and stderr's
/// progress change nothing stdout prints, and a lever changes nothing either,
/// for `-j`'s reason (C4).
const UNSHOWN_FLAGS: [&str; 16] = [
    "-L",
    "--no-lookahead",
    "-K",
    "--no-kill-cache",
    "-y",
    "--lattice-sanity-check",
    "--no-diagrams",
    "-G",
    "--full-kb-snapshots",
    "-F",
    "--reorder",
    "-R",
    "--relevant",
    "-l",
    "--layer-progress",
    "--verbose",
];

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
        if UNSHOWN_FLAGS.contains(&a.as_str()) || a == "-v" {
            continue;
        }
        if UNSHOWN.contains(&a.as_str()) || a == "-g" {
            it.next();
            continue;
        }
        if UNSHOWN.iter().any(|u| a.starts_with(&format!("{u}=")))
            || (a.starts_with("-j") && a.len() > 2)
            || (a.starts_with("-g") && a.len() > 2)
        {
            continue;
        }
        shown.push(a);
    }
    shown.join(" ")
}

/// What `--count` refuses beside it, each with the sentence its refusal
/// prints: the command that answers what the other option asks. `--count`
/// answers one question, *how many?*, over all of R, and reads no state
/// (M1e2 T1e2.1c.4.6), so an option that asks for a line, a listed state, a
/// claim or another question is a usage error, exit 2, as a retired option
/// is (`crate::solve::RETIRED`).
const COUNT_REFUSES: [(&str, &str); 12] = [
    (
        "minimal",
        "--count is over all of R only, and --minimal asks about its minimal line: a bare \
         `ein solve FILE` describes R, the minimal line's size among it, and `ein solve FILE \
         --minimal` lists the line",
    ),
    (
        "maximal",
        "--count is over all of R only, and --maximal asks about its maximal line: a bare \
         `ein solve FILE` describes R, the maximal line's size among it, and `ein solve FILE \
         --maximal` lists the line",
    ),
    (
        "solutions",
        "--count lists no state, and -n N asks for N of them: `ein solve FILE -n N` finds them",
    ),
    (
        "single",
        "--count and --single are two questions: `ein solve FILE --single` asks whether R is \
         one state, and stops at its second",
    ),
    (
        "monotone",
        "--count and --monotone are two questions: `ein solve FILE --monotone` asks whether the \
         program is monotone, and builds no node",
    ),
    (
        "models",
        "--count lists no state, and --models draws the states listed: a bare `ein solve FILE \
         --models FORM` draws R's",
    ),
    (
        "diff-models",
        "--count lists no state, and --diff-models compares the states listed with a file: a \
         bare `ein solve FILE --diff-models FILE` compares R's",
    ),
    (
        "print-final-state",
        "--count reads no state, and -p prints each listed state's facts: a bare `ein solve \
         FILE -p` prints R's",
    ),
    (
        "print-final-positive",
        "--count reads no state, and -P prints each listed state's positive facts: a bare `ein \
         solve FILE -P` prints R's",
    ),
    (
        "print-final-hfacts",
        "--count reads no state, and -f prints each listed state's candidates: a bare `ein \
         solve FILE -f` prints R's",
    ),
    (
        "hyp-stats",
        "--count lists nothing, and -H lists H, the universe: a bare `ein solve FILE -H` lists \
         it, and --count's universe row counts it",
    ),
    (
        "trace",
        "--count reads no state, and --trace writes each listed state's derivation: a bare \
         `ein solve FILE --trace FILE.md` writes R's",
    ),
];

pub fn run(m: &ArgMatches) -> i32 {
    let file = m.get_one::<String>("file").expect("required").clone();
    let counting = m.get_flag("count");
    if counting && crate::solve::refuse_retired(m, &COUNT_REFUSES) {
        return 2;
    }
    if m.get_flag("monotone") {
        return run_monotone(m, &file);
    }
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
        // container does not carry. `--count` writes none.
        if !counting
            && crate::common::looks_like_einb(&crate::common::read_bytes_or_crash(Path::new(&file)))
        {
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
    // `--dump-lattice` too: its directory is made before the run.
    let lattice_dir = m.get_one::<String>("dump-lattice").cloned();
    if let Some(dir) = lattice_dir.as_deref()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::solve::artefact_failed("dump-lattice", dir, e);
        return 1;
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
    let t_load = Instant::now();
    let Some(mut kb) = crate::common::load_any_or_exit(&mut ast, &mut terms, Path::new(&file))
    else {
        return 1;
    };
    let load = t_load.elapsed();
    // The program's config with the levers on it: what `-c` prints and the
    // `run` event carries. No route reads a lever yet (P1e2.1c), so a lever
    // changes nothing printed, which the corpus's `levers` cells check.
    let cfg = crate::solve::resolved_config(&kb, m);
    if m.get_flag("dump-config") {
        crate::printers::print_resolved_config(&cfg);
    }
    // `--trace` tells root's derivation first, so root is saturated here, with
    // its firings kept; the run's own preparation then finds it saturated.
    // `--events FILE`: the walk's `run` and `load` lines, then root and every
    // node the route visits, then one `region` line (`read_out.md` § 8.6).
    let mut events = crate::solve::events_start(m, &file, &cfg);
    // `-v`, `--layer-progress` and `--dump-lattice` read the route's nodes
    // and steps as it visits them.
    let visits = lattice_dir.as_ref().map(|_| {
        Arc::new(Mutex::new(
            ein_render::dump::region_lattice::Visits::default(),
        ))
    });
    let progress = Progress::of(m);
    if progress.is_some() || visits.is_some() {
        events = events.watched(Box::new(Watcher {
            progress,
            visits: visits.clone(),
        }));
    }
    crate::solve::events_load(&mut events, &terms, &kb);
    let trace = m.get_one::<String>("trace").cloned();
    let root_firings = match trace {
        Some(_) => match ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut events) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        },
        None => None,
    };
    let t_run = Instant::now();
    let r = match read_out::run_with(&mut kb, &mut terms, &ast, &opts, &mut events) {
        Ok(r) => r,
        Err(ein_infer::SolveError::Deadline) => {
            eprintln!(
                "** aborted: --max-time {} s reached before the question was decided **",
                max_time.unwrap_or(0.0)
            );
            return 2;
        }
        Err(ein_infer::SolveError::Compile(e)) => {
            eprintln!("ein.inference.compile.CompileError: {e}");
            return 1;
        }
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let run = t_run.elapsed();
    let t_print = Instant::now();
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
    emit_region(&mut events, &r);
    if let Err(e) = extras(&Shown::of(m), &r, &mut kb, &mut terms, &ast, want.as_ref()) {
        eprintln!("{e}");
        return 1;
    }
    if m.get_flag("timing") {
        timing(&r, load, run, t_print.elapsed());
    }
    if let (Some(dir), Some(v)) = (lattice_dir.as_deref(), visits.as_ref()) {
        let v = v.lock().expect("the watch is done").clone();
        let candidates: Vec<String> = r.answer.candidates.iter().map(|c| c.text.clone()).collect();
        let route = route_words(&r.answer);
        let inputs = ein_render::dump::region_lattice::Inputs {
            source: &file,
            argv: &header_argv("solve"),
            route,
            candidates: &candidates,
        };
        // Additive past its directory, as `--dump-answer` is.
        if let Err(e) = ein_render::dump::region_lattice::write(Path::new(dir), &v, &inputs) {
            crate::solve::artefact_failed("dump-lattice", dir, e);
        }
    }
    if let (Some(path), Some(firings)) = (trace.as_deref(), root_firings.as_deref()) {
        let shape = ein_render::trace::region::Shape {
            diagrams: !m.get_flag("no-diagrams"),
            relevant: m.get_flag("relevant"),
            reorder: m.get_flag("reorder"),
            full_kb: m.get_flag("full-kb-snapshots"),
        };
        let argv = header_argv("solve");
        let written = ein_render::trace::region::of_read_out(
            &r,
            &mut kb,
            &mut terms,
            &ast,
            firings,
            &ein_render::read_out::Options {
                command: "solve",
                argv: &argv,
                file: &file,
                budget: opts.budget,
                uncut: false,
            },
            shape,
        )
        .and_then(|md| std::fs::write(path, md).map_err(|e| e.to_string()));
        // Additive: a file that could not be written says so, and the answer
        // printed above stands with its exit code
        // (`defined_behaviour.md` § 4.4).
        if let Err(e) = written {
            crate::solve::artefact_failed("trace", path, e);
        }
    }
    if summary_path.is_none() && dump.is_none() {
        return code;
    }
    // The summary and the answer directory: the same facts as stdout, as
    // files, additive in the same sense.
    let summary = match crate::summary::build_region(&ast, &mut terms, &mut kb, &r, &file, &cfg) {
        Ok(s) => crate::summary::text(&s),
        Err(e) => {
            for (flag, path) in [("json-summary", &summary_path), ("dump-answer", &dump)] {
                if let Some(p) = path {
                    crate::solve::artefact_failed(flag, p, &e);
                }
            }
            return code;
        }
    };
    if let Some(path) = summary_path.as_deref()
        && let Err(e) = std::fs::write(path, &summary)
    {
        crate::solve::artefact_failed("json-summary", path, e);
    }
    if let Some(dir) = dump.as_deref()
        && counting
    {
        // `--count`'s answer is its read-out and the summary: it reads no
        // state, so there is no state file to write, as `--monotone` writes
        // none.
        let answer_txt = uncut(&r, &file, opts.budget);
        if let Err(e) =
            ein_render::dump::region_answer::write_counted(Path::new(dir), &answer_txt, &summary)
        {
            crate::solve::artefact_failed("dump-answer", dir, e);
        }
    } else if let Some(dir) = dump.as_deref() {
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
        }
    }
    code
}

/// `ein solve FILE --monotone` — M1e2 S1e2.1b.5, the form the user approved
/// on 2026-11-03: a question about the **program**. Root's fixpoint and the
/// static check, [`ein_infer::explore::check_monotone`], and nothing else, so
/// it has no R to disagree with a bare `ein solve` about, and its verdict and
/// sites are the `lattice` block's. `--json-summary` writes `ein-summary/2`
/// with `asked.ask = "monotone"` and no region, `--dump-answer` writes the
/// read-out uncut (`answer.txt`) and the summary (`answer.json`), and
/// `--events` narrates root and, on a flagged program, the `monotonicity`
/// line. Every verdict exits 0.
fn run_monotone(m: &ArgMatches, file: &str) -> i32 {
    let limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    let dump = m.get_one::<String>("dump-answer").cloned();
    if let Some(dir) = dump.as_deref()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::solve::artefact_failed("dump-answer", dir, e);
        return 1;
    }
    let summary_path = m.get_one::<String>("json-summary").cloned();
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = crate::common::load_any_or_exit(&mut ast, &mut terms, Path::new(file))
    else {
        return 1;
    };
    let cfg = crate::solve::resolved_config(&kb, m);
    if m.get_flag("dump-config") {
        crate::printers::print_resolved_config(&cfg);
    }
    let mut events = crate::solve::events_start(m, file, &cfg);
    crate::solve::events_load(&mut events, &terms, &kb);
    let checked = match explore::check_monotone(&mut kb, &mut terms, &ast, &mut events) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let argv = header_argv("solve");
    let render = |uncut: bool| {
        ein_render::read_out::render_monotone(
            &checked,
            &ein_render::read_out::Options {
                command: "solve",
                argv: &argv,
                file,
                budget: 0,
                uncut,
            },
            limit,
        )
    };
    let text = render(false);
    print!("{}", text.stdout);
    if m.get_flag("stats") {
        println!();
        println!("  {:<18}{:.1} ms", "wall", checked.wall.as_secs_f64() * 1e3);
    }
    for l in &text.stderr {
        eprintln!("{l}");
    }
    if summary_path.is_none() && dump.is_none() {
        return text.code;
    }
    let summary =
        match crate::summary::build_monotone(&ast, &mut terms, &mut kb, &checked, file, &cfg) {
            Ok(s) => crate::summary::text(&s),
            Err(e) => {
                for (flag, path) in [("json-summary", &summary_path), ("dump-answer", &dump)] {
                    if let Some(p) = path {
                        crate::solve::artefact_failed(flag, p, &e);
                    }
                }
                return text.code;
            }
        };
    if let Some(path) = summary_path.as_deref()
        && let Err(e) = std::fs::write(path, &summary)
    {
        crate::solve::artefact_failed("json-summary", path, e);
    }
    if let Some(dir) = dump.as_deref() {
        let dir = Path::new(dir);
        let written = std::fs::write(dir.join("answer.txt"), render(true).stdout)
            .and_then(|()| std::fs::write(dir.join("answer.json"), &summary));
        if let Err(e) = written {
            crate::solve::artefact_failed("dump-answer", &dir.display().to_string(), e);
        }
    }
    text.code
}

/// The `region` event that closes a region run's stream (`read_out.md`
/// § 8.6): the summary's own blocks, as JSON on one line — what the run
/// asked, R as it established it (its states are the summary's and the
/// dump's), `--single`'s word, and the work. `ein solve` and `ein test` both
/// end their streams with it.
pub(crate) fn emit_region(events: &mut ein_infer::events::Events, r: &ReadOut) {
    if !events.on() {
        return;
    }
    let blocks = ein_render::dump::region_answer::summary_blocks(r);
    let get = |k: &str| {
        blocks
            .iter()
            .find(|(key, _)| *key == k)
            .map(|(_, v)| v.clone())
            .unwrap_or(ein_render::dump::Json::Null)
    };
    let mut region = get("region");
    if let ein_render::dump::Json::Object(fields) = &mut region {
        fields.retain(|(k, _)| k != "states" && k != "frontier");
    }
    let compact = |j: &ein_render::dump::Json| ein_render::dump::json::dumps(j);
    events.emit("region", |l| {
        l.json("asked", &compact(&get("asked")));
        l.json("region", &compact(&region));
        l.json("verdict", &compact(&get("verdict")));
        l.json("work", &compact(&get("work")));
    });
}

/// What a region run prints after its read-out when asked, over **the states
/// it listed** — `read_out.md` § 8.3 and § 8.6: `--models key|grid`, then
/// `-p`, `-P` and `-f` per state, then `--diff-models FILE`. Each block
/// says which set it is drawn from. These are asked for by name, so the
/// shared 100 lines do not cut them.
/// What [`extras`] is asked to print — read off `solve`'s flags, or
/// `oracle`'s, which has fewer of them.
pub(crate) struct Shown {
    form: Option<String>,
    dumps: Vec<crate::factdump::Mode>,
    universe: bool,
}

impl Shown {
    /// From the flags of a command that has them: each of `--models`, `-p`,
    /// `-P`, `-f` and `-H` it does not define is off.
    pub(crate) fn of(m: &ArgMatches) -> Shown {
        use crate::factdump::Mode as Dump;
        let has = |id: &str| m.try_get_one::<bool>(id).ok().flatten().copied() == Some(true);
        Shown {
            form: m.try_get_one::<String>("models").ok().flatten().cloned(),
            dumps: [
                ("print-final-state", Dump::All),
                ("print-final-positive", Dump::Positive),
                ("print-final-hfacts", Dump::Hfacts),
            ]
            .into_iter()
            .filter(|(flag, _)| has(flag))
            .map(|(_, d)| d)
            .collect(),
            universe: has("hyp-stats"),
        }
    }
}

pub(crate) fn extras(
    shown: &Shown,
    r: &ReadOut,
    kb: &mut ein_core::Kb,
    terms: &mut Terms,
    ast: &Ast,
    want: Option<&(String, ein_render::models::Expected)>,
) -> Result<(), String> {
    use crate::factdump::print_final_state;
    let form = shown.form.as_deref();
    let dumps = &shown.dumps;
    if shown.universe {
        universe(r, terms);
    }
    if !matches!(form, Some("key" | "grid")) && dumps.is_empty() && want.is_none() {
        return Ok(());
    }
    let a = &r.answer;
    let listed: Vec<(usize, &ein_infer::read_out::Listed)> = r.listed().collect();
    // On an empty R of the owing kind there is no state, and the owing
    // frontier is what the read-out lists instead: the forms draw it, labelled
    // as what it is, and the diff has no state to compare.
    let frontier: &[ein_infer::read_out::OwingState] = match &r.listing {
        ein_infer::read_out::Listing::Owing(f) if listed.is_empty() => f,
        _ => &[],
    };
    let what = match (r.described().is_some(), &r.listing) {
        _ if !frontier.is_empty() => {
            "the owing frontier — R is empty, and every alive node owes".to_string()
        }
        (true, ein_infer::read_out::Listing::Region) => "every state of R".to_string(),
        (true, _) => "the line members this run certified".to_string(),
        (false, _) => format!(
            "the {} listed",
            if listed.len() == 1 { "state" } else { "states" }
        ),
    };
    let of = |set: &[u32]| -> Vec<ein_core::FactId> {
        set.iter().map(|&b| a.candidates[b as usize].fact).collect()
    };
    let sets: Vec<Vec<ein_core::FactId>> = if frontier.is_empty() {
        listed.iter().map(|(_, s)| of(&s.rep)).collect()
    } else {
        frontier.iter().map(|o| of(&o.node.set)).collect()
    };
    // The candidates' relations stand in for `:hrules`' targets: they are
    // what a commitment is made of.
    let mut targets: Vec<ein_core::Symbol> = Vec::new();
    for c in &a.candidates {
        let rel = terms.facts.get(c.fact).0;
        if !targets.contains(&rel) {
            targets.push(rel);
        }
    }
    let n = sets.len();
    let exhausted = a.exhausted;
    let mut nodes =
        ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates).map_err(|e| e.to_string())?;
    nodes
        .with_fixpoints(&sets, |kbs, terms| {
            let label = |i: usize| match frontier.get(i) {
                Some(o) => format!(
                    "owing node {} of {} — owes {}",
                    i + 1,
                    frontier.len(),
                    o.node.owes
                ),
                None => format!("state {} of {}", listed[i].0 + 1, r.states.len()),
            };
            match form {
                // A key tells states apart, so one state or none has no key,
                // and the flag is inert there, as it was on the walk's
                // single-model verdicts (M1d S1d.3.3).
                Some("key") if n < 2 => {}
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
                for d in dumps {
                    print_final_state(terms, kb, *d, &targets);
                }
            }
            if let Some((path, want)) = want {
                use ein_render::models::{comparable_facts, diff_lines, diff_model_sets};
                let closes = want.closes.as_deref();
                let got: Vec<Vec<String>> = if frontier.is_empty() {
                    kbs.iter()
                        .map(|kb| comparable_facts(terms, kb, closes))
                        .collect()
                } else {
                    Vec::new()
                };
                let d = diff_model_sets(&got, &want.models);
                let over = match closes {
                    Some(c) => format!("compared over {}", c.join(" ")),
                    None => "compared as positive facts minus the given ones".to_string(),
                };
                let head = if frontier.is_empty() {
                    format!("{what} against {path}, {over}")
                } else {
                    format!("R's states (none) against {path}, {over}")
                };
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
    let dead: Option<&[u32]> = r
        .described()
        .filter(|d| ein_render::read_out::dead_alone_known(d))
        .map(|d| d.dead_alone_set.as_slice());
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
                ", dead alone not known: this run did not visit every candidate alone".to_string(),
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

/// `-v` and `--layer-progress`: the route's steps on stderr as it takes them,
/// and under `-v` every `-g`-th node too.
struct Progress {
    every: Option<u64>,
    nodes: u64,
    steps: u64,
}

impl Progress {
    fn of(m: &ArgMatches) -> Option<Progress> {
        let verbose = m.get_flag("verbose");
        if !verbose && !m.get_flag("layer-progress") {
            return None;
        }
        Some(Progress {
            every: verbose
                .then(|| (*m.get_one::<i64>("progress-every").unwrap_or(&100)).max(1) as u64),
            nodes: 0,
            steps: 0,
        })
    }
}

/// The watch a region run is read through: progress to stderr, and the
/// nodes `--dump-lattice` writes and `ein render lattice` draws.
pub(crate) struct Watcher {
    progress: Option<Progress>,
    visits: Option<Arc<Mutex<ein_render::dump::region_lattice::Visits>>>,
}

impl Watcher {
    /// A watch that only collects what the route visits, into `visits`.
    pub(crate) fn collecting(
        visits: Arc<Mutex<ein_render::dump::region_lattice::Visits>>,
    ) -> Watcher {
        Watcher {
            progress: None,
            visits: Some(visits),
        }
    }
}

/// `1, the alive down-set` or `2, the cube`: the route an answer took, as the
/// read-out names it. A mode on the state route meets only the states its
/// question needs (M1e2 S1e2.1c.4); describe meets every alive one.
pub(crate) fn route_words(answer: &explore::Answer) -> &'static str {
    // `--count` takes describe's route.
    let describe = answer.mode.which == explore::Which::All
        && matches!(
            answer.mode.ask,
            explore::Ask::Describe | explore::Ask::Count
        );
    match answer.route {
        explore::Route::DownSet => "1, the alive down-set",
        explore::Route::Cube => "2, the cube",
        explore::Route::States if describe => "the state route, every alive state once",
        explore::Route::States => "the state route, each state at most once",
        explore::Route::Tree => "the obligation tree, over states",
    }
}

impl ein_infer::events::Watch for Watcher {
    fn node(&mut self, v: &ein_infer::events::Visited<'_>) {
        if let Some(p) = self.progress.as_mut() {
            p.nodes += 1;
            if let Some(g) = p.every
                && p.nodes % g == 0
            {
                let set = if v.commitment.is_empty() {
                    "∅".to_string()
                } else {
                    format!("{{{}}}", v.commitment.join(", "))
                };
                let kind = if v.kind == "in-r" { "in R" } else { v.kind };
                eprintln!("  node {}: {set} {kind}", p.nodes);
            }
        }
        if let Some(visits) = self.visits.as_ref() {
            visits.lock().expect("one thread watches").nodes.push(
                ein_render::dump::region_lattice::VisitedNode {
                    commitment: v.commitment.to_vec(),
                    kind: v.kind,
                    owes: v.owes,
                    owed: v.owed.to_vec(),
                    core: v.core.to_vec(),
                    refuted_by: v.refuted_by.to_vec(),
                    closure: v.closure.to_vec(),
                    facts: v.facts,
                },
            );
        }
    }

    fn step(&mut self, s: &ein_infer::events::Stepped) {
        if let Some(p) = self.progress.as_mut() {
            p.steps += 1;
            let name = match s.layer {
                Some(l) => format!("layer {l}"),
                None => format!("step {}", p.steps),
            };
            eprintln!(
                "{name}: {} visited — {} dead, {} owing, {} in R · {} facts",
                s.visited, s.dead, s.owing, s.r, s.facts
            );
        }
        if let Some(visits) = self.visits.as_ref() {
            let mut v = visits.lock().expect("one thread watches");
            let before: usize = v
                .steps
                .iter()
                .map(|(x, _)| x.visited as usize)
                .sum::<usize>();
            v.steps.push((*s, before));
        }
    }
}

/// `-t`: where the wall clock went. The route is everything up to its last
/// node, root's saturation and H included; the read-out is each listed
/// state's model and the queries; the print is the text.
fn timing(r: &ReadOut, load: Duration, run: Duration, print: Duration) {
    let route = r.answer.wall;
    let read = run.saturating_sub(route);
    let ms = |d: Duration| format!("{:.2} ms", d.as_secs_f64() * 1e3);
    println!();
    println!("timing");
    for (what, d, note) in [
        ("load", load, "parse, imports, the program".to_string()),
        (
            "route",
            route,
            format!(
                "root, H and every node visited: {} saturation{}",
                r.answer.saturations,
                if r.answer.saturations == 1 { "" } else { "s" }
            ),
        ),
        (
            "read-out",
            read,
            if r.answer.mode.ask == Ask::Count {
                "none: --count reads no state and answers no query".to_string()
            } else {
                "each listed state's model, the queries".to_string()
            },
        ),
        ("print", print, "the text".to_string()),
    ] {
        println!("  {what:<10}{:>12}   {note}", ms(d));
    }
    println!("  {:<10}{:>12}", "total", ms(load + run + print));
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
