//! `ein solve`'s region run, the only one since M1e2
//! [S1e2.1a.5](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! switch: bare `ein solve FILE` describes R, and `--minimal`, `--maximal`,
//! `--single` and `-n N` each ask one question about it, answered by
//! [`ein_infer::explore`] and printed in the read-out
//! [`read_out.md`](../../../../docs/kernel/inference/read_out.md) defines, by
//! [`ein_render::read_out`]. The modes were built at S1e2.1a.9's
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
use ein_render::read_out::Row;

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
/// progress change nothing stdout prints. `-y` adds its own row, the spot
/// check's, and changes no answer, so the header leaves it out as it leaves
/// `-j` (C4). `--require-why` decides whether the program loads, and a
/// program it lets through answers what it answers without it, so a run that
/// prints a header prints the same one (M1f T1f.5.14.6).
const UNSHOWN_FLAGS: [&str; 13] = [
    "-y",
    "--lattice-sanity-check",
    "--require-why",
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
        if !counting {
            match crate::common::read_program(Path::new(&file)) {
                Ok(crate::common::Held::Container(_)) => {
                    eprintln!(
                        "error: --dump-answer {dir}: a .einb container carries no program source, and \
                         every file the answer directory holds starts with it — run it on the .ein"
                    );
                    return 2;
                }
                Ok(crate::common::Held::Text(_)) => {}
                Err(e) => {
                    eprintln!("{e}");
                    return 1;
                }
            }
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
    // A dead node's core only where a surface prints one (M1e2 S1e2.2.21, the
    // user's ruling of 2026-11-05, *minimum work per question*), and the two
    // asks apart (M1f S1f.5.12, the user's ruling of 2026-11-08 on P-15):
    // `--dump-lattice`'s `nodes.json` and `--events`' `enter` lines print a
    // core at every dead node the run visits, which the walk takes as it
    // goes; `--dump-answer`'s `nogoods.json` and `--trace`'s *N* print one
    // for each member of `N`, which the run takes after the walk, and only
    // describe prints `N` (`--count` writes no `nogoods.json`). Root's, and on
    // the cube the minimal holes', which the read-out and the summary print,
    // are taken either way.
    let describing = opts.mode
        == (Mode {
            which: Which::All,
            ask: Ask::Describe,
        });
    opts.cores = ein_infer::region::Cores {
        visited: lattice_dir.is_some() || m.get_one::<String>("events").is_some(),
        nogoods: describing && (dump.is_some() || m.get_one::<String>("trace").is_some()),
    };
    // `--diff-models FILE` is read before the run, as the walk's is: a file
    // it cannot read, or whose facts the parser refuses (M1f S1f.5.5,
    // T1f.5.5.6: every error recovery reaches, each with the file's line and
    // column), is a usage error, and nothing runs.
    let want = match m.get_one::<String>("diff-models") {
        None => None,
        Some(path) => match std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|t| ein_render::models::parse_expected(&t, path).map_err(|e| e.to_string()))
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
    let load_opts = crate::common::load_options(m);
    let Some(mut kb) =
        crate::common::load_program(&mut ast, &mut terms, Path::new(&file), &load_opts)
    else {
        return 1;
    };
    let load = t_load.elapsed();
    // The program's config with `-y` on it: what `-c` prints and the `run`
    // event carries. `-y` is the read-out's (the spot check, unbounded, M1e2
    // S1e2.1c.5); no route reads it, so it moves no answer and no `work` row,
    // which the corpus's `levers` cells check.
    let cfg = crate::solve::resolved_config(&kb, m);
    opts.sanity = cfg.lattice_sanity_check;
    if m.get_flag("dump-config") {
        crate::printers::print_resolved_config(&cfg);
    }
    // `--trace` tells root's derivation first, taken on a copy of root, so
    // the run's own preparation is root's one saturation, as without it (M1e2
    // S1e2.2.21: a root saturated twice moved cores).
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
        Some(_) => match ein_infer::region::root_derivation(&mut kb, &mut terms, &ast) {
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
        // `-y` found a parent path that reaches another state on a monotone
        // program: the engine's error, never an answer (M1e2 S1e2.1c.5).
        Err(ein_infer::SolveError::SpotCheck(e)) => {
            eprintln!("error: {e}");
            return 2;
        }
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let run = t_run.elapsed();
    let t_print = Instant::now();
    let (code, text) = print(
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
    // The forms share what the read-out left of stdout's 100 lines; under
    // `--dump-answer` each is also kept uncut for `DIR/forms/`.
    let forms = match extras(
        &Shown::of(m),
        &r,
        &text,
        dump_hint("solve"),
        dump.is_some(),
        &mut kb,
        &mut terms,
        &ast,
        want.as_ref(),
    ) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
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
            forms: &forms,
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
    let load_opts = crate::common::load_options(m);
    let Some(mut kb) =
        crate::common::load_program(&mut ast, &mut terms, Path::new(file), &load_opts)
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
        println!(
            "{}",
            ein_render::read_out::line(
                Row::Wall,
                format!("{:.1} ms", checked.wall.as_secs_f64() * 1e3)
            )
        );
    }
    for l in &text.stderr {
        eprintln!("{l}");
    }
    if summary_path.is_none() && dump.is_none() {
        return text.code;
    }
    let summary = crate::summary::text(&crate::summary::build_monotone(
        &terms, &kb, &checked, file, &cfg,
    ));
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

/// What a region run prints after its read-out when asked — `read_out.md`
/// § 8.3 and § 8.6: `-H`, then `--models key|grid`, then `-p`, `-P` and `-f`,
/// each its own form, then `--diff-models FILE`.
///
/// **They share stdout's 100 lines with the read-out** (M1e2 S1e2.3.3, the
/// user's ruling (a) of 2026-11-03). Each is drawn from the states stdout
/// showed, so the screen and the forms agree; each prints its items while
/// what the read-out left of the lines lasts, and at least one; and its last
/// line says what it left out and where that is. `--dump-answer DIR` writes
/// each form uncut, drawn from every listed state, as `DIR/forms/<form>.txt`.
///
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

/// One form after the read-out, as items, so stdout can cut it where the
/// read-out cuts its listings and the dump can write it whole.
struct Form {
    /// Its file under `DIR/forms/`, `.txt` added.
    name: &'static str,
    /// The lines above its items, never cut, as the read-out's rows are not.
    head: Vec<String>,
    /// Its items in groups, each group under a line that is never cut: `-H`'s
    /// relations. Every other form is one group with no line.
    groups: Vec<(Option<String>, Vec<Vec<String>>)>,
    /// The lines under its items, printed when every item is.
    tail: Vec<String>,
    /// What one item is, counted in the cut's line: `more state`.
    unit: (&'static str, &'static str),
    /// Items the uncut form holds and this one does not draw: the listed
    /// states stdout did not show.
    beyond: usize,
    /// What the uncut form says that no cut of these items holds: a key or a
    /// comparison of every listed state, where stdout drew a part of them.
    also: Option<String>,
}

impl Form {
    fn one_group(items: Vec<Vec<String>>) -> Vec<(Option<String>, Vec<Vec<String>>)> {
        vec![(None, items)]
    }

    /// The form uncut: what `--dump-answer` writes as its file.
    fn text(&self) -> String {
        let mut out = String::new();
        let mut put = |l: &str| {
            out.push_str(l);
            out.push('\n');
        };
        for l in &self.head {
            put(l);
        }
        for (line, items) in &self.groups {
            if let Some(l) = line {
                put(l);
            }
            for l in items.iter().flatten() {
                put(l);
            }
        }
        for l in &self.tail {
            put(l);
        }
        out
    }

    /// Print what fits in `left`, at least one item, then where the rest
    /// is; `left` is what the next form gets.
    fn print_cut(&self, left: &mut usize, hint: &str) {
        for l in &self.head {
            println!("{l}");
        }
        let total: usize = self.groups.iter().map(|(_, i)| i.len()).sum();
        let mut printed = 0usize;
        let mut cut = false;
        for (line, items) in &self.groups {
            if let Some(l) = line {
                println!("{l}");
            }
            for it in items {
                if !cut && (printed == 0 || it.len() <= *left) {
                    for l in it {
                        println!("{l}");
                    }
                    *left = left.saturating_sub(it.len());
                    printed += 1;
                } else {
                    cut = true;
                }
            }
        }
        if !cut {
            for l in &self.tail {
                println!("{l}");
            }
        }
        let mut rest: Vec<String> = Vec::new();
        let more = total - printed + self.beyond;
        if more > 0 {
            rest.push(format!(
                "{} {}",
                ein_render::read_out::group(more as u64),
                if more == 1 { self.unit.0 } else { self.unit.1 }
            ));
        }
        rest.extend(self.also.clone());
        if !rest.is_empty() {
            println!(
                "  ;; not shown: {} — {hint} writes the form uncut to DIR/forms/{}.txt",
                rest.join(", and "),
                self.name
            );
        }
    }
}

/// Where a form's rest is, as the cut's line names it: the read-out's own
/// [`ein_render::read_out::dump_hint`], so a form and the listings above it
/// name one writer (M1e2 S1e2.3.7).
pub(crate) fn dump_hint(command: &str) -> &'static str {
    ein_render::read_out::dump_hint(command)
}

/// Print the forms `shown` asks for after the read-out `text`, and return
/// each uncut, by name, when `dump` asks for them (`--dump-answer`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn extras(
    shown: &Shown,
    r: &ReadOut,
    text: &ein_render::read_out::Text,
    hint: &str,
    dump: bool,
    kb: &mut ein_core::Kb,
    terms: &mut Terms,
    ast: &Ast,
    want: Option<&(String, ein_render::models::Expected)>,
) -> Result<Vec<(&'static str, String)>, String> {
    use crate::factdump::{Mode as Dump, final_state_lines};
    use ein_render::read_out::group;
    let plural = |n: usize, one: &str, many: &str| {
        format!("{} {}", group(n as u64), if n == 1 { one } else { many })
    };
    let mut left = text.left();
    let mut uncut: Vec<(&'static str, String)> = Vec::new();
    if shown.universe {
        let h = universe(r, terms);
        h.print_cut(&mut left, hint);
        if dump {
            uncut.push((h.name, h.text()));
        }
    }
    let form = shown.form.as_deref();
    let dumps = &shown.dumps;
    if !matches!(form, Some("key" | "grid")) && dumps.is_empty() && want.is_none() {
        return Ok(uncut);
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
    // Every listed state, and the ones stdout showed: a prefix of them, since
    // every listing is cut from its end. The frontier is never cut.
    let all = if frontier.is_empty() {
        listed.len()
    } else {
        frontier.len()
    };
    let on_screen = if frontier.is_empty() {
        listed
            .iter()
            .take_while(|(i, _)| text.shown.contains(i))
            .count()
    } else {
        all
    };
    let what_all = match (r.described().is_some(), &r.listing) {
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
    let what_shown = match (r.described().is_some(), &r.listing) {
        (true, ein_infer::read_out::Listing::Region) => format!(
            "the {} shown above, of R's {}",
            plural(on_screen, "state", "states"),
            group(all as u64)
        ),
        (true, _) => format!(
            "the {} shown above, of the {} this run certified",
            plural(on_screen, "line member", "line members"),
            group(all as u64)
        ),
        (false, _) => format!(
            "the {} shown above, of the {} listed",
            plural(on_screen, "state", "states"),
            group(all as u64)
        ),
    };
    let of = |set: &[u32]| -> Vec<ein_core::FactId> {
        set.iter().map(|&b| a.candidates[b as usize].fact).collect()
    };
    // The states read: the ones stdout showed, or every listed one when the
    // dump writes the forms uncut. The first `on_screen` are stdout's.
    let reads = if dump { all } else { on_screen };
    let sets: Vec<Vec<ein_core::FactId>> = if frontier.is_empty() {
        listed[..reads].iter().map(|(_, s)| of(&s.rep)).collect()
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
    let exhausted = a.exhausted;
    let maximal = listed.iter().all(|(_, s)| s.maximal);
    let n_states = r.states.len();
    let mut nodes =
        ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates).map_err(|e| e.to_string())?;
    let written = nodes
        .with_fixpoints(&sets, |kbs, terms| {
            let label = |i: usize| match frontier.get(i) {
                Some(o) => format!(
                    "owing node {} of {} — owes {}",
                    i + 1,
                    frontier.len(),
                    o.node.owes
                ),
                None => format!("state {} of {n_states}", listed[i].0 + 1),
            };
            let unit = if frontier.is_empty() {
                ("more state", "more states")
            } else {
                ("more owing node", "more owing nodes")
            };
            // The forms over the first `c` states read: stdout's, or all.
            let forms_over = |c: usize, kbs: &[ein_core::Kb], terms: &Terms| -> Vec<Form> {
                let whole = c == all;
                let what = if whole { &what_all } else { &what_shown };
                let beyond = all - c;
                let also = |what: &str| {
                    (!whole).then(|| format!("{what} of all {} listed", group(all as u64)))
                };
                let mut out: Vec<Form> = Vec::new();
                match form {
                    // A key tells states apart, so one state or none has no
                    // key, and the flag is inert there, as it was on the
                    // walk's single-model verdicts (M1d S1d.3.3).
                    Some("key") if all < 2 => {}
                    Some("key") => {
                        let mut head = vec![
                            String::new(),
                            format!("models — the determining key of {what}"),
                        ];
                        let mut rows: Vec<Vec<String>> = Vec::new();
                        let mut tail: Vec<String> = Vec::new();
                        if c < 2 {
                            head.push(
                                "  one state shown: a key tells states apart, and needs two"
                                    .to_string(),
                            );
                        } else {
                            let refs: Vec<&ein_core::Kb> = kbs[..c].iter().collect();
                            let of = if whole {
                                ein_render::models::KeyOf::States { maximal }
                            } else {
                                ein_render::models::KeyOf::Shown { listed: all }
                            };
                            match ein_render::models::key_parts_of(
                                terms, &refs, exhausted, "  ", of,
                            ) {
                                Ok(p) => {
                                    head.extend(p.head);
                                    rows = p.rows.into_iter().map(|l| vec![l]).collect();
                                    tail = p.tail;
                                }
                                Err(why) => {
                                    head.push("  determining key — none within budget".to_string());
                                    head.extend(ein_render::models::wrap(
                                        &format!("{why}."),
                                        "    ",
                                    ));
                                }
                            }
                        }
                        out.push(Form {
                            name: "key",
                            head,
                            groups: Form::one_group(rows),
                            tail,
                            unit: ("more row", "more rows"),
                            beyond: 0,
                            also: also("the key"),
                        });
                    }
                    Some("grid") => {
                        let items = (0..c)
                            .map(|i| {
                                let mut b = vec![String::new(), format!("  ── {} ──", label(i))];
                                b.extend(ein_render::grid::grid_or_reason(terms, &kbs[i], "  "));
                                b
                            })
                            .collect();
                        out.push(Form {
                            name: "grid",
                            head: vec![String::new(), format!("models — {what}, each as a grid")],
                            groups: Form::one_group(items),
                            tail: Vec::new(),
                            unit,
                            beyond,
                            also: None,
                        });
                    }
                    _ => {}
                }
                for d in dumps {
                    let items = (0..c)
                        .map(|i| {
                            let mut b = vec![String::new(), format!("── {} ──", label(i))];
                            b.extend(final_state_lines(terms, &kbs[i], *d, &targets));
                            b
                        })
                        .collect();
                    out.push(Form {
                        name: match d {
                            Dump::All => "print-final-state",
                            Dump::Positive => "print-final-positive",
                            Dump::Hfacts => "print-final-hfacts",
                        },
                        head: Vec::new(),
                        groups: Form::one_group(items),
                        tail: Vec::new(),
                        unit,
                        beyond,
                        also: None,
                    });
                }
                if let Some((path, want)) = want {
                    use ein_render::models::{comparable_facts, diff_model_sets, diff_parts};
                    let closes = want.closes.as_deref();
                    let got: Vec<Vec<String>> = if frontier.is_empty() {
                        kbs[..c]
                            .iter()
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
                    let head_line = if frontier.is_empty() {
                        format!("{what} against {path}, {over}")
                    } else {
                        format!("R's states (none) against {path}, {over}")
                    };
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
                    let (h, items) = diff_parts(&d, &head_line, &label, &want_label, "");
                    let mut head = vec![String::new()];
                    head.extend(h);
                    if !whole {
                        head.push(
                            "  over the states shown above only: a model missing here may be a \
                             listed state stdout did not show"
                                .to_string(),
                        );
                    }
                    out.push(Form {
                        name: "diff-models",
                        head,
                        groups: Form::one_group(items),
                        tail: Vec::new(),
                        unit: ("more entry", "more entries"),
                        beyond: 0,
                        also: also("the comparison"),
                    });
                }
                out
            };
            for f in forms_over(on_screen, kbs, terms) {
                f.print_cut(&mut left, hint);
            }
            if dump {
                forms_over(all, kbs, terms)
                    .into_iter()
                    .map(|f| (f.name, f.text()))
                    .collect()
            } else {
                Vec::new()
            }
        })
        .map_err(|e| e.to_string())?;
    uncut.extend(written);
    Ok(uncut)
}

/// `-H`: H, the universe — every candidate, by relation, and those dead
/// alone, which no state of R can hold (`read_out.md` § 8.6, P1e2.3's S4).
/// A relation's line is never cut; its candidates are the form's items.
fn universe(r: &ReadOut, terms: &Terms) -> Form {
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
    let head = vec![
        String::new(),
        format!(
            "H — the universe: {} candidate{} over {} relation{}{}",
            a.candidates.len(),
            if a.candidates.len() == 1 { "" } else { "s" },
            by_rel.len(),
            if by_rel.len() == 1 { "" } else { "s" },
            match dead {
                Some(_) => format!(", {n_dead} dead alone"),
                None => ", dead alone not known: this run did not visit every candidate alone"
                    .to_string(),
            }
        ),
    ];
    let mut groups: Vec<(Option<String>, Vec<Vec<String>>)> = Vec::new();
    for (rel, cands) in &by_rel {
        let d = cands.iter().filter(|(i, _)| is_dead(*i)).count();
        // One space at least after the relation (M1f S1f.5.6, W42): a name of
        // sixteen characters or more touched its count, `lives-next-door-to2
        // candidates`.
        let w = 16.max(rel.chars().count() + 1);
        let line = format!(
            "  {rel:<w$}{} candidate{}{}",
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
        let items = cands
            .iter()
            .map(|(i, text)| {
                vec![if is_dead(*i) {
                    format!("    {text:<width$}   dead alone")
                } else {
                    format!("    {text}")
                }]
            })
            .collect();
        groups.push((Some(line), items));
    }
    Form {
        name: "hyp-stats",
        head,
        groups,
        tail: Vec::new(),
        unit: ("more candidate", "more candidates"),
        beyond: 0,
        also: None,
    }
}

/// `-v` and `--layer-progress`: the route's steps on stderr as it takes them,
/// and under `-v` every `-g`-th node too, none at `-g 0`.
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
        // `-g 0` logs no node line, and a negative never reaches here: the
        // parser refuses it (M1e2 S1e2.3.4, the user's ruling (b) of
        // 2026-11-03).
        let every = *m.get_one::<i64>("progress-every").unwrap_or(&100);
        Some(Progress {
            every: (verbose && every > 0).then_some(every as u64),
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
                    core_status: v.core_status,
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
    for (row, d, note) in [
        (
            Row::TimingLoad,
            load,
            "parse, imports, the program".to_string(),
        ),
        (
            Row::TimingRoute,
            route,
            format!(
                "root, H and every node visited: {} saturation{}",
                r.answer.saturations,
                if r.answer.saturations == 1 { "" } else { "s" }
            ),
        ),
        (
            Row::TimingReadOut,
            read,
            if r.answer.mode.ask == Ask::Count {
                "none: --count reads no state and answers no query".to_string()
            } else {
                "each listed state's model, the queries".to_string()
            },
        ),
        (Row::TimingPrint, print, "the text".to_string()),
    ] {
        println!("  {:<10}{:>12}   {note}", row.label(), ms(d));
    }
    println!(
        "  {:<10}{:>12}",
        Row::TimingTotal.label(),
        ms(load + run + print)
    );
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

/// Print `r`, write its stderr lines, and return its exit code and the text
/// printed, whose `shown` and `listed` the forms after it read. `-T` aborts
/// with 2 before anything is printed.
pub fn print(
    r: &ReadOut,
    command: &str,
    file: &str,
    budget: u64,
    max_time: Option<f64>,
    stats: bool,
) -> (i32, ein_render::read_out::Text) {
    if r.answer.stopped == Some(Stopped::Deadline) {
        eprintln!(
            "** aborted: --max-time {} s reached before the question was decided **",
            max_time.unwrap_or(0.0)
        );
        return (2, ein_render::read_out::Text::default());
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
            "{}",
            ein_render::read_out::line(
                Row::Wall,
                format!("{:.1} ms", r.answer.wall.as_secs_f64() * 1e3)
            )
        );
    }
    for l in &text.stderr {
        eprintln!("{l}");
    }
    (text.code, text)
}
