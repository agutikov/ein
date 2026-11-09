//! M1e2 S1e2.2.26 (W27) — **past `--region-limit`, a line mode certifies
//! what describe certifies**, the user's ruling of 2026-11-05 on Q-M1e2.19.
//!
//! On a program the static check flags, a question that needs the whole cube
//! is answered past the limit from the members the run met in layer order
//! (`solve_interface.md` § 3.3). Describe certifies a member met with no
//! member of R met below it as minimal, `{∅}` when ∅ is in R, and says the
//! maximal line is not known. Until S1e2.2.26 `--minimal` and `--maximal`
//! labelled every member met as on their line: 65 214 states on
//! `square-unique/corner-house`, where describe certifies one.
//!
//! `ein-infer/tests/explore.rs` holds every mode to the oracle within the
//! limit; this file holds the line modes past it, on every corpus program
//! and register witness the static check flags, under a lowered
//! `--region-limit` `k` for each `k` in [`LIMITS`] below its candidate
//! count, in-process and through the read-out the binary prints:
//!
//! - **to describe**, at the same limit: the states a line mode lists are
//!   the ones describe certifies on that line (`N` of them under `-n N`,
//!   some of them under `--single`), the summary's `lines` say the same of
//!   every state, the line row reads what describe's reads, and every other
//!   member describe met is in the mode's answer, unlisted, for
//!   `--dump-answer` to write;
//! - **to the oracle**, where its R is known (17 candidates or fewer): every
//!   state a mode labels on a line is on the oracle's line.
//!
//! **Since M1f S1f.5.6 a minimal question stops at root where ∅ is in R**
//! (the user's ruling of 2026-11-07 on S-16, *"b, use the ∅ rule in both"*):
//! ∅ is below every commitment, so the minimal line is {∅}, and `--minimal`,
//! `--minimal -n N` and `--minimal --single` each list ∅ alone, decided,
//! after one saturation, at every limit. Until then `-n N` and `--single`
//! were left out of the row check, and past the limit `--single` said *not
//! decided* and `-n 2` a lower bound where describe certified {∅}.
//!
//! ```text
//! cargo test -p ein-render --test line_modes_past_the_limit
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out::ReadOut;
use ein_infer::region::{self, Outcome, Region};
use ein_ir::{Ast, load, parse};
use serde_json::Value as J;

/// The limits each flagged program is asked under, those below its
/// candidate count: past the limit at each. The default, 16, only where the
/// oracle's R is known: the W27 witness, past it at default settings. The
/// programs past 17 candidates take the same code at 2^8 nodes that they
/// take at the default's 2^16, at a 256th of the cost.
const LIMITS: [u32; 8] = [0, 1, 2, 3, 4, 6, 8, 16];

/// The largest program the oracle enumerates here: the default limit, and
/// `examples/ein-bugs/wrong-answers/past-limit-lines.ein`, the W27 witness
/// whose 17 candidates put it past the default limit.
const ORACLE_MAX: usize = 17;

/// Every line mode, and describe to hold them to.
const LINE_MODES: [Mode; 8] = [
    Mode {
        which: Which::Minimal,
        ask: Ask::Describe,
    },
    Mode {
        which: Which::Minimal,
        ask: Ask::Find(1),
    },
    Mode {
        which: Which::Minimal,
        ask: Ask::Find(2),
    },
    Mode {
        which: Which::Minimal,
        ask: Ask::Single,
    },
    Mode {
        which: Which::Maximal,
        ask: Ask::Describe,
    },
    Mode {
        which: Which::Maximal,
        ask: Ask::Find(1),
    },
    Mode {
        which: Which::Maximal,
        ask: Ask::Find(2),
    },
    Mode {
        which: Which::Maximal,
        ask: Ask::Single,
    },
];

const DESCRIBE: Mode = Mode {
    which: Which::All,
    ask: Ask::Describe,
};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// Every corpus file and every register witness, once.
fn corpus_and_register() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
}

fn jobs() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().min(8))
}

fn mask(set: &[u32]) -> u64 {
    set.iter().fold(0, |m, &b| m | 1 << b)
}

/// A state as the read-out has it: its closure and its `(model …)`.
type Key = (Vec<u32>, Vec<String>);

fn key(s: &ein_infer::read_out::Listed) -> Key {
    (s.closure.to_vec(), s.model.clone())
}

fn line_of(which: Which) -> &'static str {
    match which {
        Which::Minimal => "minimal",
        Which::Maximal => "maximal",
        Which::All => "all",
    }
}

/// The read-out of `mode` at `--region-limit limit`, rendered as the binary
/// renders it, and its `--json-summary`.
fn read(path: &Path, mode: Mode, limit: u32) -> (ReadOut, String, J) {
    let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
    let mut opts = explore::Options::new(mode);
    opts.limit = limit;
    opts.jobs = jobs();
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let file = path.to_string_lossy().to_string();
    let text = ein_render::read_out::render(
        &r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &file,
            file: &file,
            budget: opts.budget,
            uncut: true,
        },
    );
    let region = ein_render::dump::region_answer::summary_blocks(&r)
        .into_iter()
        .find(|(k, _)| *k == "region")
        .map(|(_, v)| v)
        .expect("a region block");
    let summary: J = serde_json::from_str(&ein_render::dump::json::dumps_indent(&region))
        .expect("the summary parses");
    (r, text.stdout, summary)
}

/// The read-out's `minimal line` or `maximal line` row, its value.
fn row(stdout: &str, label: &str) -> Option<String> {
    stdout.lines().find_map(|l| {
        l.strip_prefix("  ")
            .and_then(|l| l.strip_prefix(label))
            .filter(|v| v.starts_with("  "))
            .map(|v| v.trim_start().to_string())
    })
}

/// The oracle's line, as classes.
fn oracle_line(r: &Region, which: Which) -> BTreeSet<u64> {
    let masks = match which {
        Which::Minimal => &r.topology.minimal,
        Which::Maximal => &r.topology.maximal,
        Which::All => unreachable!("a line"),
    };
    masks.iter().map(|&m| r.nodes[m as usize].class).collect()
}

/// **Every line mode past the limit certifies what describe certifies**, the
/// same states, lines and row, and labels no state on a line the oracle does
/// not put it on.
#[test]
fn past_the_limit_a_line_mode_certifies_what_describe_does() {
    let mut failures: Vec<String> = Vec::new();
    let (mut programs, mut runs, mut labelled_off_describe) = (0usize, 0usize, 0usize);
    let mut empty_ruled = 0usize;
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(checked) = explore::check_monotone(&mut kb, &mut terms, &ast, &mut Events::off())
        else {
            continue;
        };
        if checked.monotonicity.is_monotone() || checked.root_dead {
            continue;
        }
        let n = checked.n;
        let oracle = if n <= ORACLE_MAX {
            let (mut kb, mut terms, ast) = loaded(&path).expect("loaded once already");
            let o = region::Options {
                limit: ORACLE_MAX as u32,
                jobs: jobs(),
                ..region::Options::default()
            };
            match region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {}) {
                Ok(Outcome::Region(r)) => Some(r),
                // *Forbid* refuses it, and every mode with it.
                Err(ein_infer::SolveError::Refused(_)) => continue,
                other => panic!("{rel}: the oracle: {:?}", other.err()),
            }
        } else {
            None
        };
        programs += 1;
        for limit in LIMITS
            .into_iter()
            .filter(|&k| (k as usize) < n && (k < 16 || n <= ORACLE_MAX))
        {
            let (d, d_out, d_summary) = read(&path, DESCRIBE, limit);
            let tag = |mode: Mode| format!("{rel} --region-limit {limit} {mode:?}");
            let described = d.described().expect("describe describes");
            if described.exhausted {
                failures.push(format!("{}: exhausted past the limit", tag(DESCRIBE)));
                continue;
            }
            let met: BTreeSet<Key> = d.states.iter().map(key).collect();
            let empty_in_r = described.root.in_r();
            for mode in LINE_MODES {
                let line = line_of(mode.which);
                let (r, out, summary) = read(&path, mode, limit);
                runs += 1;
                let tag = tag(mode);
                // The ∅ rule (M1f S1f.5.6): where describe says ∅ is in R,
                // every minimal question lists ∅ alone, the state describe
                // certifies, decided after one saturation, and its row says
                // so.
                if mode.which == Which::Minimal && empty_in_r {
                    empty_ruled += 1;
                    let listed: Vec<Key> = r.listed().map(|(_, s)| key(s)).collect();
                    let root: Vec<Key> = d
                        .listed()
                        .map(|(_, s)| s)
                        .filter(|s| s.rep.is_empty())
                        .map(key)
                        .collect();
                    let a = &r.answer;
                    let decided = if mode.ask == Ask::Single {
                        a.verdict == Some(explore::Verdict::Solution)
                    } else {
                        a.exhausted
                    };
                    if listed != root
                        || !decided
                        || a.stopped.is_some()
                        || a.saturations != 1
                        || r.states.len() != 1
                    {
                        failures.push(format!(
                            "{tag}: ∅ is in R, and the run lists {} states, decided {decided}, \
                             stopped {:?}, after {} saturations, {} met",
                            listed.len(),
                            a.stopped,
                            a.saturations,
                            r.states.len()
                        ));
                    }
                    let want = if mode.ask == Ask::Single {
                        ("verdict", "Solution — the minimal line is one state")
                    } else {
                        ("minimal line", "1 state — all of them")
                    };
                    if row(&out, want.0).as_deref() != Some(want.1) {
                        failures.push(format!(
                            "{tag}: the {} row reads {:?}, not {:?}",
                            want.0,
                            row(&out, want.0),
                            want.1
                        ));
                    }
                    continue;
                }
                // What describe certifies on this line, and what the mode lists.
                let on_line = |s: &ein_infer::read_out::Listed| match mode.which {
                    Which::Minimal => s.minimal,
                    _ => s.maximal,
                };
                let certified: Vec<Key> = d
                    .listed()
                    .map(|(_, s)| s)
                    .filter(|s| on_line(s))
                    .map(key)
                    .collect();
                let listed: Vec<Key> = r.listed().map(|(_, s)| key(s)).collect();
                let certified_set: BTreeSet<Key> = certified.iter().cloned().collect();
                let listed_set: BTreeSet<Key> = listed.iter().cloned().collect();
                if !listed_set.is_subset(&certified_set) {
                    labelled_off_describe += 1;
                    failures.push(format!(
                        "{tag}: lists {} states on the {line} line where describe certifies {}",
                        listed.len(),
                        certified.len()
                    ));
                }
                let whole = match mode.ask {
                    Ask::Describe => Some(certified_set.len()),
                    Ask::Find(want) => Some(certified_set.len().min(want as usize)),
                    Ask::Single | Ask::Count => None,
                };
                if let Some(w) = whole
                    && listed_set.len() != w
                {
                    failures.push(format!(
                        "{tag}: lists {} states where describe certifies {} on the {line} line",
                        listed_set.len(),
                        certified_set.len()
                    ));
                }
                // Every state the mode reports carries the line it is
                // certified on, and no other; the rest it met are unlisted.
                for (i, s) in r.states.iter().enumerate() {
                    let want_line = s.listed;
                    if (mode.which == Which::Minimal && s.minimal != want_line)
                        || (mode.which == Which::Maximal && s.maximal != want_line)
                    {
                        failures.push(format!(
                            "{tag}: state {} is labelled {:?}, listed {}",
                            i + 1,
                            (s.minimal, s.maximal),
                            s.listed
                        ));
                    }
                }
                // The summary says the same: `lines` on each listed state, none
                // on the others, and the same states describe's summary marks.
                let states = summary["states"].as_array().cloned().unwrap_or_default();
                let marked = |s: &J| {
                    s["lines"]
                        .as_array()
                        .is_some_and(|v| v.iter().any(|x| x == line))
                };
                for s in &states {
                    if marked(s) != (s["listed"] == true) {
                        failures.push(format!(
                            "{tag}: the summary's lines {} for a state listed {}",
                            s["lines"], s["listed"]
                        ));
                    }
                }
                let d_states = d_summary["states"].as_array().cloned().unwrap_or_default();
                let count = |v: &[J]| v.iter().filter(|s| marked(s)).count();
                if mode.ask == Ask::Describe && count(&states) != count(&d_states) {
                    failures.push(format!(
                        "{tag}: the summary marks {} states {line}, describe's {}",
                        count(&states),
                        count(&d_states)
                    ));
                }
                // The line row reads what describe's reads, where the mode
                // answers from describe's description: `--minimal -n N` is
                // answered in layer order and stops at its `N`th state, so
                // its row says how many it listed.
                if mode.ask == Ask::Describe
                    || (mode.which == Which::Maximal && mode.ask != Ask::Single)
                {
                    let label = format!("{line} line");
                    let (got, want) = (row(&out, &label), row(&d_out, &label));
                    if got != want {
                        failures.push(format!(
                            "{tag}: the {label} row reads {got:?}, describe's {want:?}"
                        ));
                    }
                }
                // The other members met are the mode's too, unlisted, for
                // `--dump-answer` to write.
                if mode.ask == Ask::Describe || mode.which == Which::Maximal {
                    let all: BTreeSet<Key> = r.states.iter().map(key).collect();
                    if all != met {
                        failures.push(format!(
                            "{tag}: reports {} members met where describe met {}",
                            all.len(),
                            met.len()
                        ));
                    }
                }
                // No state labelled on a line the oracle does not put it on.
                if let Some(o) = oracle.as_deref() {
                    let line_classes = oracle_line(o, mode.which);
                    for s in r.states.iter().filter(|s| on_line(s)) {
                        let m = mask(&s.rep);
                        if !o.in_r(m) || !line_classes.contains(&o.nodes[m as usize].class) {
                            failures.push(format!(
                                "{tag}: labels {:?} {line}, which the oracle's {line} line does \
                                 not hold",
                                s.rep
                            ));
                        }
                    }
                    for st in &r.answer.states {
                        let m = mask(&st.set);
                        if !o.in_r(m) || !line_classes.contains(&o.nodes[m as usize].class) {
                            failures.push(format!(
                                "{tag}: answers {:?} on the {line} line, which the oracle's \
                                 does not hold",
                                st.set
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} failures ({labelled_off_describe} runs labelling past describe):\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "{programs} flagged programs, {runs} line-mode runs past the limit, {empty_ruled} of them \
         a minimal question where ∅ is in R"
    );
    // Coverage, so a sweep that reached nothing cannot pass: 29 flagged
    // programs and 800 line-mode runs past the limit on 2026-11-05.
    assert!(programs >= 25, "{programs} flagged programs");
    assert!(runs >= 700, "{runs} line-mode runs past the limit");
    // The ∅ rule reached: 324 of the 800 on 2026-11-09 (M1f S1f.5.6).
    assert!(
        empty_ruled >= 300,
        "{empty_ruled} minimal runs where ∅ is in R"
    );
}
