//! What the engine *did* — the ein.rs half of the T1a.6.1.3 comparison.
//!
//! ```sh
//! cargo run --release --features counters -p ein-infer --example counter_cost
//! cargo run --release --features counters -p ein-infer --example counter_cost -- --json
//! ```
//!
//! Without `--features counters` this prints nothing but a reminder: the
//! counters are compiled out by default ([`ein_core::counters`]), because an
//! increment in `unify_slot` is an increment on the hottest path in the engine
//! and a measurement instrument that changes the measurement is worth less than
//! no instrument at all.
//!
//! The comparable Python numbers came from `utils/count_work.py`, which wrapped
//! the same call sites in ein.py and printed the same field names. Two engines
//! that agree on a verdict but disagree on `candidates` by 3× are not the same
//! search, and that was worth knowing before any speed claim was made about
//! either — it is the argument this example was built for. The script left with
//! its subject at
//! [S1a.10.4](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md);
//! its column is frozen in
//! [baseline.md §4](../../../../plans/m1a_rust/p1a.6_performance/baseline.md#4-what-the-engine-did--the-work-counters).
//! What this half is for now is the same question asked across *commits*:
//! whether an optimisation moved the work or only the clock.

use ein_core::{Terms, counters};
use ein_infer::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};

/// One benchmarked file and the `(label, stop_after)` regimes it is run under.
/// The path borrows from `argv` when one was given, so the lifetime is not
/// `'static`; the labels are.
type Case<'a> = (&'a str, Vec<(&'static str, Option<u64>)>);

fn main() {
    let json = std::env::args().any(|a| a == "--json");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");

    if !cfg!(feature = "counters") {
        eprintln!(
            "counters are compiled out — re-run with \
             `--features counters` (see ein_core::counters)"
        );
        return;
    }

    // With no argument: the four milestone cells. With paths: those files,
    // exhaustively — S1a.6.4 needed the counters of a *blind-mode* puzzle and
    // the four cells are both hrule-driven, so the table could not see the
    // enumerator the rest of the corpus runs.
    let files: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| !a.starts_with("--"))
        .collect();
    let cases: Vec<Case<'_>> = if files.is_empty() {
        vec![
            (
                "examples/zebra2.ein",
                vec![("fast", Some(1)), ("exhaustive", None)],
            ),
            (
                "examples/zebra.ein",
                vec![("fast", Some(1)), ("exhaustive", None)],
            ),
        ]
    } else {
        files
            .iter()
            .map(|f| (f.as_str(), vec![("exhaustive", None)]))
            .collect()
    };

    let mut cells: Vec<(String, ein_core::Counters, u64, String)> = Vec::new();
    for (rel, runs) in &cases {
        for &(label, stop_after) in runs {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let mut events = Events::off();
            let opts = SolveOptions {
                stop_after,
                ..SolveOptions::default()
            };
            // Reset *after* the load, so the row is the solve and the frontend's
            // own counters do not land in it. The load's are P1a.1/P1a.2's rows.
            counters::reset();
            let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
                .expect("solves");
            let c = counters::snapshot();
            let verdict = solved.answer.as_str().to_string();
            let name = rel.rsplit('/').next().unwrap_or(rel).replace(".ein", "");
            cells.push((
                format!("{name} {label}"),
                c,
                solved.stats.base.enterings_total,
                verdict,
            ));
        }
    }

    if json {
        println!("{{");
        for (i, (name, c, enterings, verdict)) in cells.iter().enumerate() {
            let fields: Vec<String> = c
                .rows()
                .iter()
                .map(|(k, v)| format!("\"{k}\": {v}"))
                .collect();
            println!(
                "  \"{name}\": {{\"enterings\": {enterings}, \"verdict\": \"{verdict}\", \
                 {}}}{}",
                fields.join(", "),
                if i + 1 == cells.len() { "" } else { "," }
            );
        }
        println!("}}");
        return;
    }

    print!("{:<18}", "counter");
    for (name, _, _, _) in &cells {
        print!("{name:>20}");
    }
    println!();
    println!("{}", "─".repeat(18 + 20 * cells.len()));
    for (i, (key, _)) in cells[0].1.rows().iter().enumerate() {
        print!("{key:<18}");
        for (_, c, _, _) in &cells {
            print!("{:>20}", group(c.rows()[i].1));
        }
        println!();
    }
    print!("{:<18}", "(enterings)");
    for (_, _, e, _) in &cells {
        print!("{:>20}", group(*e));
    }
    println!();
    print!("{:<18}", "(verdict)");
    for (_, _, _, v) in &cells {
        print!("{v:>20}");
    }
    println!();
    // The frontend six read zero here and that is the measurement, not a bug:
    // this example resets the counters *after* `load_file`, because what it
    // times is a solve. `--example frontend_cost` is where a load is counted.
    println!(
        "\nparse_* / lex_* / intern* are zero by construction — the KB is \
         loaded before the reset;\nsee `--example frontend_cost` for the load."
    );
}

/// `40123456` → `40 123 456`. The numbers here run to eight digits and the
/// comparison is by order of magnitude before it is by digit.
fn group(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() * 4 / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}
