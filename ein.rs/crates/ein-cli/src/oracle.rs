//! `ein oracle` — the region by brute force. M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4.
//!
//! Every subset of the root candidates is saturated by
//! [`ein_infer::region::compute`], with no licence and nothing skipped, and
//! the answer is printed in `ein solve`'s describing read-out
//! ([`crate::modes::print`]), which on the cube adds what only the cube has:
//! `E`, and the holes. Then one line says whether the static verdict held on
//! it. It is the reference every `ein solve` mode is held to
//! (`ein-infer/tests/explore.rs`), and it costs `2ⁿ` on every program, so it
//! runs only when asked for by name.

use std::path::Path;
use std::time::Instant;

use clap::ArgMatches;
use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out;
use ein_infer::region::{self, LOUD_FROM, Outcome, PROBE_NODES, Projection};
use ein_ir::Ast;

pub fn run(m: &ArgMatches) -> i32 {
    let file = m.get_one::<String>("file").expect("required").clone();
    let limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    let jobs = crate::solve::resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1));
    let dir = m.get_one::<String>("dump").cloned();
    let stats = m.get_flag("stats");
    let max_time = m.get_one::<f64>("max-time").copied();
    let opts = region::Options {
        limit,
        keep_states: m.get_flag("states"),
        jobs,
        deadline: max_time.map(|s| Instant::now() + std::time::Duration::from_secs_f64(s.max(0.0))),
        // Every dead node's core where `--dump` writes them; else root's and
        // the minimal holes', which the read-out and the summary print (M1e2
        // S1e2.2.21, the user's ruling of 2026-11-05).
        cores: dir.is_some(),
    };
    let summary_path = m.get_one::<String>("json-summary").cloned();
    // `--dump DIR` is made before the cube, as `solve`'s directories are: a
    // path that cannot be made fails before the work, exit 1, and a write that
    // fails after it leaves the answer's exit code (`defined_behaviour.md`
    // § 4.4).
    if let Some(dir) = dir.as_deref()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::solve::artefact_failed("dump", dir, e);
        return 1;
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let load_opts = crate::common::load_options(m);
    let Some(mut kb) =
        crate::common::load_program(&mut ast, &mut terms, Path::new(&file), &load_opts)
    else {
        return 1;
    };
    let mut on_projection = |p: &Projection| {
        let secs = p.projected.as_secs_f64();
        if p.n >= LOUD_FROM {
            eprintln!(
                "warning: {file}: {n} root candidates — the region is 2^{n} = {nodes} \
                 saturations.\nwarning:   measured {per:.1} µs per node on {PROBE_NODES} \
                 spread over the cube; projected {secs:.1} s at one job.\nwarning:   \
                 --region-limit {limit} asked for it; interrupt to stop.",
                n = p.n,
                nodes = p.nodes,
                per = p.per_node.as_secs_f64() * 1e6,
            );
        } else {
            eprintln!(
                "warning: {file}: {} root candidates, 2^{} saturations, projected {secs:.1} s \
                 at one job",
                p.n, p.n
            );
        }
    };
    let t0 = Instant::now();
    let outcome = match region::compute(&mut kb, &mut terms, &ast, &opts, &mut on_projection) {
        Ok(o) => o,
        Err(ein_infer::SolveError::Deadline) => {
            eprintln!(
                "** aborted: --max-time {} s reached before the cube was done **",
                max_time.unwrap_or(0.0)
            );
            return 2;
        }
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    if let Some(dir) = dir.as_deref() {
        let inputs = ein_render::dump::RegionInputs {
            source: &file,
            limit,
        };
        if let Err(e) = ein_render::dump::write_cube_dir(Path::new(dir), &outcome, &terms, &inputs)
        {
            crate::solve::artefact_failed("dump", dir, e);
        }
    }
    let r = match outcome {
        Outcome::Region(r) => r,
        Outcome::Refused { n, limit, .. } => {
            eprintln!(
                "error: {file}: {}",
                ein_render::dump::region::refusal_note(n, limit)
            );
            return 2;
        }
    };
    let violations = (
        r.topology.dead_not_upward_closed.count,
        r.topology.discharged_not_upward_closed.count,
    );
    let monotone = r.monotonicity.is_monotone();
    // What the closing line concludes, on a flagged program, from the
    // detector: the status the cube shows, and whether a node's state holds a
    // stale fact, which the status cannot show (M1e2 S1e2.1b.5).
    let flagged_close = r.warning.as_deref().map(|w| match &w.status {
        ein_infer::detector::Status::Exercised { by, .. } => {
            format!(
                " — the status is not monotone here, {}",
                ein_infer::detector::by_words(by)
            )
        }
        _ => {
            let k = w.state.stale.len();
            match k {
                0 => " — the status is monotone here; no node holds a stale fact".to_string(),
                1 => {
                    " — the status is monotone here; 1 node holds a stale fact (above)".to_string()
                }
                k => format!(
                    " — the status is monotone here; {} nodes hold a stale fact (above)",
                    ein_render::read_out::group(k as u64)
                ),
            }
        }
    });
    let mode = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let mut answer = explore::of_cube(mode, *r);
    answer.wall = t0.elapsed();
    let mut eopts = explore::Options::new(mode);
    eopts.limit = limit;
    eopts.jobs = jobs;
    let ro = match read_out::of_answer(&mut kb, &mut terms, &ast, answer, &eopts) {
        Ok(ro) => ro,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let (rc, text) = crate::modes::print(&ro, "oracle", &file, eopts.budget, None, stats);
    // What only the cube can say: whether the static verdict held on it.
    println!();
    println!(
        "the cube   {} dead node{} with an alive superset · {} discharged node{} with an \
         owing superset{}",
        violations.0,
        if violations.0 == 1 { "" } else { "s" },
        violations.1,
        if violations.1 == 1 { "" } else { "s" },
        match (monotone, violations == (0, 0)) {
            (true, true) => " — the static verdict holds".to_string(),
            (true, false) => " — THE STATIC VERDICT IS CONTRADICTED".to_string(),
            (false, _) => flagged_close.unwrap_or_default(),
        }
    );
    // `--models key|grid`, `-p` and `-P`, after the whole read-out, in what
    // it left of stdout's 100 lines. `ein oracle` writes no answer
    // directory, so their cut names `ein solve`'s.
    if let Err(e) = crate::modes::extras(
        &crate::modes::Shown::of(m),
        &ro,
        &text,
        crate::modes::dump_hint("oracle"),
        false,
        &mut kb,
        &mut terms,
        &ast,
        None,
    ) {
        eprintln!("{e}");
        return 1;
    }
    // `--json-summary`: the read-out as `ein-summary/2`, additive as
    // `solve`'s is.
    if let Some(path) = summary_path.as_deref() {
        let config = kb.program().config.clone().unwrap_or_default();
        let written = crate::summary::build_region(&ast, &mut terms, &mut kb, &ro, &file, &config)
            .and_then(|s| {
                std::fs::write(path, crate::summary::text(&s)).map_err(|e| e.to_string())
            });
        if let Err(e) = written {
            crate::solve::artefact_failed("json-summary", path, e);
        }
    }
    // A contradicted static verdict is an engine defect, never an answer.
    rc.max(i32::from(monotone && violations != (0, 0)))
}
