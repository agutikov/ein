//! `ein oracle` — the region by brute force. M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4.
//!
//! Every subset of the root candidates is saturated by
//! [`ein_infer::region::compute`], with no licence and nothing skipped, and
//! the answer is printed in `ein solve`'s describing layout
//! ([`crate::modes::print`]) plus what only the cube has: its holes, and the
//! static verdict checked against it. It is the reference every `ein solve`
//! mode is held to (`ein-infer/tests/explore.rs`), and it costs `2ⁿ` on every
//! program, so it runs only when asked for by name.

use std::path::Path;
use std::time::Instant;

use clap::ArgMatches;
use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::region::{self, LOUD_FROM, Outcome, PROBE_NODES, Projection};
use ein_ir::Ast;

pub fn run(m: &ArgMatches) -> i32 {
    let file = m.get_one::<String>("file").expect("required").clone();
    let limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    let jobs = crate::solve::resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1));
    let dir = m.get_one::<String>("dump").cloned();
    let stats = m.get_flag("stats");
    let opts = region::Options {
        limit,
        keep_states: m.get_flag("states"),
        jobs,
    };
    let (mut rc, mut index) = (0, 0usize);
    loop {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Some(mut kb) =
            crate::common::load_any_query_or_exit(&mut ast, &mut terms, Path::new(&file), index)
        else {
            return rc.max(1);
        };
        let n_queries = kb.program().queries.len();
        if n_queries > 1 {
            println!("query {} of {n_queries}", index + 1);
        }
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
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        };
        if let Some(dir) = dir.as_deref() {
            let inputs = ein_render::dump::RegionInputs {
                source: &file,
                limit,
                max_set_size: 0,
                stop_after: None,
            };
            if let Err(e) =
                ein_render::dump::write_cube_dir(Path::new(dir), &outcome, &terms, &inputs)
            {
                crate::solve::artefact_failed("dump", dir, e);
                return 1;
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
        let mode = Mode {
            which: Which::All,
            ask: Ask::Describe,
        };
        let mut answer = explore::of_cube(mode, *r);
        answer.wall = t0.elapsed();
        crate::modes::print(&answer, &explore::Options::new(mode), stats);
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
                (true, true) => " — the static verdict holds",
                (true, false) => " — THE STATIC VERDICT IS CONTRADICTED",
                (false, true) => " — the flagged sites break nothing here",
                (false, false) => "",
            }
        );
        rc = rc.max(if monotone && violations != (0, 0) {
            1
        } else {
            0
        });
        index += 1;
        if index >= n_queries {
            break;
        }
        println!();
    }
    rc
}
