//! S1a.5.2 acceptance — the trace and the answer table, byte for byte.
//!
//! Three modes per corpus entry, each one solve (`ein-render`'s `trace_shape`
//! and `utils/ir_oracle.py`'s `trace-shape` op enumerate the same names):
//!
//! - **`trace`** — the markdown in all six flag combinations the CLI can
//!   produce (`--no-diagrams`, `--full-kb-snapshots`, `--reorder`,
//!   `--relevant`, and `--relevant --reorder`), then the `(trace …)` IR
//!   round-trip. The solve is the *fast* regime, because that is the one that
//!   reaches a solution and hands the renderer a spine to narrate.
//! - **`answer`** — `render_answer` and `render_solution_table` at both
//!   `exhausted` values, plus the exhaustive regime's own trace. Exhaustive,
//!   because `Ambiguity` / `Contradiction` / `Aborted` live only there.
//! - **`no-proof`** — `linearize` over a `store_lattice=False` verdict, the
//!   only way to reach its three proof-less branches; the CLI never does,
//!   since `--trace` implies `store_lattice=True`.
//!
//! `--relevant` and `--reorder` are the two flags whose output nobody looks
//! at often, which is why they get their own rows rather than sharing one.

use ein_core::Terms;
use ein_ir::{Ast, parse};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use ein_render::shape::{TRACE_MODES, trace_shape};
use std::path::Path;

/// Where ein.py is expected to raise and ein.rs to answer —
/// [D2](../../../../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
/// reached by every mode, because every mode runs the search.
// D2 reaches two files since 2026-08-20: the str-vs-int shape
// Q-M1a.4 was written about, and the `Fact`-vs-`Fact` one the S1a.6.6
// fuzzer found, which needs no mixed types at all.
const DIVERGENT: [&str; 2] = [
    "examples/ein-bugs/mixed-type-hypothesis.ein",
    "examples/ein-bugs/nested-fact-hypothesis.ein",
];

/// The narration cut, applied to both sides.
///
/// A trace block whose body is a **rendered derivation** is
/// [D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)'s
/// territory: ein.rs's forks resume root's saturation where ein.py's re-derive
/// it, so a solution's spine is a quarter the length — and, since the
/// derivations that used to arrive by accident now have to be rendered on
/// purpose, ein.rs's trace opens with a *Before any assumption* section that
/// ein.py has no counterpart for. Which blocks those are is
/// `ein-parity`'s closed list and not this file's opinion; `--- answer`,
/// `--- table` and `--- round-trip` are not on it, because the **answer** does
/// not move and the trace-IR round-trip is still asserted on each side.
///
/// The mode is prefixed as a synthetic block head, which is how the
/// `no-proof` mode — one rendered trace and nothing else, so no blocks at all
/// — is covered by the same rule as the others rather than by a special case.
///
/// The bodies are owed an ein.rs golden by
/// [S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md).
fn narrow(shape: &str, mode: &str) -> String {
    if ein_parity::strict() {
        return shape.to_string();
    }
    ein_parity::blank_blocks(&format!("--- {mode}\n{shape}"), "--- ")
}

fn rust_mode(path: &Path, mode: &str) -> Option<Answer> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    match trace_shape(&mut ast, &mut terms, &forms, path.parent(), mode) {
        Ok(out) => Some(Answer::Ok(out)),
        Err(msg) => Some(Answer::Err {
            kind: "TraceShapeError".into(),
            msg,
        }),
    }
}

#[test]
fn the_trace_and_the_table_are_byte_identical_on_the_corpus() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_trace_and_the_table_are_byte_identical_on_the_corpus");
    };
    let (mut bad, mut compared, mut bytes, mut files) = (Vec::new(), 0usize, 0usize, 0usize);
    let (mut round_trips, mut narrated) = (0usize, 0usize);
    let mut seen_divergent: Vec<String> = Vec::new();
    for path in &corpus_files() {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        let name = rel.display();
        let expected = DIVERGENT.contains(&rel.to_str().unwrap_or_default());
        let before = compared;
        for mode in TRACE_MODES {
            let Some(got) = rust_mode(path, mode) else {
                continue;
            };
            let want = py.ask(serde_json::json!({
                "op": "trace-shape",
                "path": path.to_string_lossy(),
                "mode": mode,
            }));
            match (&got, &want) {
                (Answer::Ok(_), Answer::Err { .. }) if expected => {
                    seen_divergent.push(format!("{name} [{mode}]"));
                }
                _ if expected => bad.push(format!(
                    "{name} [{mode}] is a ledger entry and no longer diverges\n  \
                     rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
                (Answer::Ok(a), Answer::Ok(b)) => {
                    compared += 1;
                    bytes += a.len();
                    let (x, y) = (narrow(a, mode), narrow(b, mode));
                    if x != y {
                        bad.push(format!("{name} [{mode}]\n{}", first_difference(&x, &y)));
                    }
                    if a != b {
                        narrated += 1;
                    }
                    // Agreeing on `DIFFERS` would pass the byte diff and fail
                    // the *property*, so the property is asserted separately:
                    // `parse(trace_to_ir(steps))` → `parse_trace_steps` →
                    // `trace_to_ir` has to reproduce its input.
                    if mode == "trace" {
                        if a.contains("--- round-trip ok") {
                            round_trips += 1;
                        } else {
                            bad.push(format!("{name} [{mode}]: the trace IR does not round-trip"));
                        }
                    }
                }
                // Both refuse: a file the loader rejects has nothing to solve.
                (Answer::Err { .. }, Answer::Err { .. }) => {}
                _ => bad.push(format!(
                    "{name} [{mode}]\n  rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
            }
        }
        if compared > before {
            files += 1;
        }
    }
    let mut want_divergent: Vec<String> = DIVERGENT
        .iter()
        .flat_map(|f| TRACE_MODES.iter().map(move |m| format!("{f} [{m}]")))
        .collect();
    want_divergent.sort();
    seen_divergent.sort();
    assert_eq!(
        seen_divergent, want_divergent,
        "the ledger's divergent modes are not the ones that diverged"
    );
    assert!(
        bad.is_empty(),
        "{} of {compared} traces differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!(
        "T3 (trace): {files} files, {compared} modes, {bytes} bytes, \
         {round_trips} IR round-trips, {narrated} narration-only, 0 differences"
    );
    assert!(
        compared >= 150 && round_trips >= 50,
        "only {compared} modes / {round_trips} round-trips compared"
    );
    // The cut has to be load-bearing: if no rendered trace differs before it
    // is applied, D3 stopped reaching this test and the cut should go rather
    // than sit there unexamined.
    assert!(
        ein_parity::strict() || narrated > 0,
        "no trace needed the narration cut — D3 no longer reaches this test"
    );
}

fn brief(a: &Answer) -> String {
    match a {
        Answer::Ok(s) => format!("{} lines", s.lines().count()),
        Answer::Err { kind, msg } => format!("{kind}: {msg}"),
    }
}

/// The first differing line, with three lines of leading context.
fn first_difference(a: &str, b: &str) -> String {
    let (a, b): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for i in 0..a.len().max(b.len()) {
        let (x, y) = (a.get(i), b.get(i));
        if x != y {
            let mut out: Vec<String> = ((i.saturating_sub(3))..i)
                .map(|j| format!("     {}", a[j]))
                .collect();
            out.push(format!("  rs {}", x.unwrap_or(&"<end>")));
            out.push(format!("  py {}", y.unwrap_or(&"<end>")));
            return format!("  line {}:\n{}", i + 1, out.join("\n"));
        }
    }
    "  (no line differs — trailing newline?)".to_string()
}
