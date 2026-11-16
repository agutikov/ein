//! **The corpus sweeps' worker pool.**
//!
//! Cargo runs the test binaries one after another, and inside one the
//! harness runs the test *functions* in parallel; but a sweep over the corpus
//! is one `#[test]` with a loop, which on its own keeps one hardware thread
//! busy however many the machine has. A sweep hands its loop here instead:
//! its work is a list of **units** — a (program, output)
//! pair, or a program, or a CLI cell — and [`map`] runs them on
//! [`jobs`] workers and returns their results **in the units' order**.
//!
//! **A verdict does not depend on the worker count**, and that is the whole
//! contract: the caller folds the results in order exactly as its serial loop
//! did, so a failure reads the same at `EIN_TEST_JOBS=1` and at 16. Three
//! things make that true and are this module's to keep:
//!
//! - a unit's result is a function of the unit alone: a sweep keeps no state
//!   that one unit writes and another reads, which is what moving its loop
//!   body into a closure checks for it (the closure is `Fn`, not `FnMut`);
//! - results come back in the order of `units`, whatever order they finished
//!   in;
//! - a unit that panics is reported as the *first* panicking unit in that
//!   order, with its own message, once: the default hook's line, which would
//!   print every panicking worker's message in the order they happened, is
//!   held back for a worker and printed for that one unit only
//!   (`install_hook`).
//!
//! **The count** is `EIN_TEST_JOBS` (default [`DEFAULT_JOBS`]), never more
//! than the machine's threads. Every pool in a
//! process shares it — two sweeps of one binary that the harness runs at
//! once run at most [`jobs`] units between them — and a unit that calls
//! [`map`] runs its own units inline, so nothing waits on a permit it holds.
//!
//! **The heavy lane.** A program on [`HEAVY`], the corpus programs with the
//! largest peak resident memory, runs at most [`heavy_jobs`] units at
//! a time, so the pool's peak stays under the bound written beside the list.
//! Units are started heaviest first, then the programs on [`LONG`], then by
//! the manifest's `cost_ms`, so the last unit to finish is not a long one
//! started last; no order reaches a result.
//!
//! **The instrument.** `EIN_TEST_UNITS=<file>` appends one line per unit,
//! `sweep  program  output  ms  peak_kb`, sorted, after each map: what each
//! unit cost, which is how a sweep's cost per (program, output) is read.
//! The peak is taken at `EIN_TEST_JOBS=1` only, where one
//! unit runs at a time: the process's peak resident set is reset before the
//! unit (`/proc/self/clear_refs`) and read after it (`VmHWM`), less the
//! resident set the unit started from, so it is the unit's own growth over
//! what the process already held — a lower bound where the allocator hands
//! the unit memory an earlier unit freed.

use std::any::Any;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::Instant;

/// The worker count when `EIN_TEST_JOBS` is unset: the performance-core
/// threads of a 32-thread machine, half its threads; on a machine with fewer
/// threads, the cap to the machine's threads is what binds.
pub const DEFAULT_JOBS: usize = 16;

/// **The programs that run in the heavy lane**, repo-relative: the corpus
/// programs whose peak resident memory is 256 MB or more.
///
/// Through the CLI, `ein solve FILE` at the default region budget and `ein
/// oracle FILE`, each process's `ru_maxrss`: `send_more_money` 529 MB and
/// `04_open` 266 MB, every other program 187 MB or less. In-process, every
/// unit of the shape golden's sweep at `EIN_TEST_JOBS=1` under
/// `EIN_TEST_UNITS` left the whole test process under 272 MB. So the bound is
/// loose: four of these at once and twelve of the next-heaviest come to under
/// 5 GB. A program not on the list that grows past the line is not refused;
/// it is the list that goes stale.
pub const HEAVY: &[&str] = &[
    "examples/beyond_arith/send_more_money.ein",
    "examples/features/04_open.ein",
];

/// **The programs whose units start first**, repo-relative: those with a unit
/// of 5 s or more at `EIN_TEST_JOBS=1` in a pooled sweep (`EIN_TEST_UNITS`
/// over the whole workspace's tests at `--profile gate`). A sweep's wall
/// clock is at least its longest unit, and a long
/// one started last adds itself to the rest; the manifest's `cost_ms`, the
/// order after these, is measured on the declared CLI runs alone and misses
/// most of them. Starting order reaches no result.
pub const LONG: &[&str] = &[
    "examples/beyond_arith/equation_two_unknowns.ein",
    "examples/beyond_arith/send_more_money.ein",
    "examples/binary_logic/synthesis.ein",
    "examples/branching/01_saturate_only.ein",
    "examples/ein-bugs/wrong-answers/count-past-2-128.ein",
    "examples/ein-bugs/wrong-answers/count-past-2-64.ein",
    "examples/ein-bugs/wrong-answers/past-limit-lines.ein",
    "examples/features/04_open.ein",
    "examples/lattice/07_two_pairs_two_tables.ein",
    "examples/saturation/square-unique/corner-house.ein",
    "examples/saturation/type-exclusivity/pets.ein",
    "examples/zebra.ein",
    "examples/zebra2-hints.ein",
    "examples/zebra2-minus-15-obligations.ein",
    "examples/zebra2-minus-15.ein",
    "examples/zebra2-obligations.ein",
    "examples/zebra2.ein",
    "tests/stdlib/bijection/06_blind_enumeration.ein",
];

/// At most this many [`HEAVY`] units at once: a quarter of the workers, at
/// least one.
pub fn heavy_jobs() -> usize {
    (jobs() / 4).max(1)
}

/// `EIN_TEST_JOBS`, or [`DEFAULT_JOBS`], capped at the machine's threads.
///
/// A value that is not a count of one or more is a panic, not the default: a
/// mistyped `EIN_TEST_JOBS=l` that ran at 16 would be a measurement of the
/// wrong thing with nothing to say so.
pub fn jobs() -> usize {
    static JOBS: OnceLock<usize> = OnceLock::new();
    *JOBS.get_or_init(|| {
        let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
        let want = match std::env::var("EIN_TEST_JOBS") {
            Err(_) => DEFAULT_JOBS,
            Ok(s) => s
                .trim()
                .parse::<usize>()
                .ok()
                .filter(|n| *n >= 1)
                .unwrap_or_else(|| {
                    panic!("EIN_TEST_JOBS={s:?} is not a worker count of 1 or more")
                }),
        };
        want.min(threads)
    })
}

/// A counting semaphore: the process's permits.
struct Permits {
    free: Mutex<usize>,
    cv: Condvar,
}

impl Permits {
    fn new(n: usize) -> Permits {
        Permits {
            free: Mutex::new(n),
            cv: Condvar::new(),
        }
    }
    fn acquire(&self) {
        let mut free = self.free.lock().unwrap_or_else(|e| e.into_inner());
        while *free == 0 {
            free = self.cv.wait(free).unwrap_or_else(|e| e.into_inner());
        }
        *free -= 1;
    }
    fn release(&self) {
        *self.free.lock().unwrap_or_else(|e| e.into_inner()) += 1;
        self.cv.notify_one();
    }
}

fn all_permits() -> &'static Permits {
    static P: OnceLock<Permits> = OnceLock::new();
    P.get_or_init(|| Permits::new(jobs()))
}

fn heavy_permits() -> &'static Permits {
    static P: OnceLock<Permits> = OnceLock::new();
    P.get_or_init(|| Permits::new(heavy_jobs()))
}

thread_local! {
    /// Set on a pool worker while it runs a unit: a [`map`] called there runs
    /// inline, and a panic there is held back by [`install_hook`].
    static IN_UNIT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The held-back panic's line, for the unit's result.
    static HELD: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// The panic hook, once per process: on a pool worker running a unit, keep
/// the message for the unit's result instead of printing it; anywhere else,
/// the hook that was there before.
fn install_hook() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if IN_UNIT.with(|c| c.get()) {
                let at = info.location().map_or(String::new(), |l| {
                    format!(" at {}:{}:{}", l.file(), l.line(), l.column())
                });
                HELD.with(|h| {
                    *h.borrow_mut() = Some(format!(
                        "a unit panicked{at}:\n{}",
                        payload_text(info.payload())
                    ))
                });
            } else {
                previous(info);
            }
        }));
    });
}

fn payload_text(p: &(dyn Any + Send)) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "<a panic with no message>".to_string()
    }
}

/// One unit's outcome before the results are put back in order.
enum Done<R> {
    Ok(R),
    Panicked(Box<dyn Any + Send>, String),
}

/// A unit's outcome, its milliseconds and, where measured, its peak in kB.
type Ran<R> = (Done<R>, u128, Option<u64>);

/// Whether `program` (repo-relative or absolute) is on [`HEAVY`].
pub fn is_heavy(program: &str) -> bool {
    HEAVY.iter().any(|h| program.ends_with(h))
}

/// The manifest's `cost_ms` for `program`, or 0: the order units start in.
fn cost_of(program: &str) -> u64 {
    static COSTS: OnceLock<Vec<(String, u64)>> = OnceLock::new();
    let costs = COSTS.get_or_init(|| {
        crate::corpus()
            .entry
            .iter()
            .filter_map(|e| e.cost_ms.map(|c| (e.path.clone(), c)))
            .collect()
    });
    costs
        .iter()
        .find(|(p, _)| program.ends_with(p.as_str()))
        .map_or(0, |(_, c)| *c)
}

/// **The order the pool starts units in**, as indices into `programs` (each
/// a unit's program): the [`HEAVY`] programs first, then the [`LONG`] ones,
/// then by the manifest's `cost_ms`, longest first, then in the list's own
/// order. No result reads it; `crate::shard` deals programs to the shards in
/// it, so the long ones are spread first.
pub fn start_order(programs: &[String]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..programs.len()).collect();
    order.sort_by_key(|&i| {
        let p = &programs[i];
        (
            !is_heavy(p),
            !LONG.iter().any(|l| p.ends_with(l)),
            std::cmp::Reverse(cost_of(p)),
            i,
        )
    });
    order
}

/// **Run `f` over every unit on the pool; the results in the units' order.**
///
/// `sweep` names the caller for the instrument; `key` names a unit as
/// `(program, output)` — the program repo-relative where it is a corpus
/// file, and `output` what the unit runs it through, empty where the unit is
/// the program. A panic in a unit is re-raised here, after every unit has
/// run, as the first panicking unit in `units`' order, with its message.
pub fn map<U, R, K, F>(sweep: &str, units: &[U], key: K, f: F) -> Vec<R>
where
    U: Sync,
    R: Send,
    K: Fn(&U) -> (String, String) + Sync,
    F: Fn(&U) -> R + Sync,
{
    install_hook();
    // Inside a unit: inline, in order, under the unit's own permit.
    if IN_UNIT.with(|c| c.get()) {
        return units.iter().map(&f).collect();
    }
    let keys: Vec<(String, String)> = units.iter().map(&key).collect();
    let heavy: Vec<bool> = keys.iter().map(|(p, _)| is_heavy(p)).collect();
    // The order units start in: heavy first, then the longest programs.
    let programs: Vec<String> = keys.iter().map(|(p, _)| p.clone()).collect();
    let order = start_order(&programs);
    let instrument = std::env::var_os("EIN_TEST_UNITS");
    let measure_peak = instrument.is_some() && jobs() == 1;

    let next = AtomicUsize::new(0);
    let workers = jobs().min(units.len()).max(1);
    let run_one = |i: usize| -> Ran<R> {
        if heavy[i] {
            heavy_permits().acquire();
        }
        all_permits().acquire();
        let held = if measure_peak { reset_peak() } else { 0 };
        let started = Instant::now();
        IN_UNIT.with(|c| c.set(true));
        let out = catch_unwind(AssertUnwindSafe(|| f(&units[i])));
        IN_UNIT.with(|c| c.set(false));
        let ms = started.elapsed().as_millis();
        let peak = measure_peak
            .then(|| status_kb("VmHWM:"))
            .flatten()
            .map(|p| p.saturating_sub(held));
        all_permits().release();
        if heavy[i] {
            heavy_permits().release();
        }
        let done = match out {
            Ok(r) => Done::Ok(r),
            Err(p) => {
                let line = HELD
                    .with(|h| h.borrow_mut().take())
                    .unwrap_or_else(|| format!("a unit panicked:\n{}", payload_text(&*p)));
                Done::Panicked(p, line)
            }
        };
        (done, ms, peak)
    };
    let mut slots: Vec<Option<Ran<R>>> = (0..units.len()).map(|_| None).collect();
    std::thread::scope(|s| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                s.spawn(|| {
                    let mut mine = Vec::new();
                    loop {
                        let k = next.fetch_add(1, Ordering::Relaxed);
                        let Some(&i) = order.get(k) else { break };
                        mine.push((i, run_one(i)));
                    }
                    mine
                })
            })
            .collect();
        for h in handles {
            for (i, r) in h.join().expect("a pool worker caught its unit's panic") {
                slots[i] = Some(r);
            }
        }
    });

    if let Some(path) = instrument {
        let mut lines: Vec<String> = slots
            .iter()
            .zip(&keys)
            .map(|(r, (p, o))| {
                let (_, ms, peak) = r.as_ref().expect("every unit ran");
                format!(
                    "{sweep}\t{p}\t{o}\t{ms}\t{}",
                    peak.map_or(String::new(), |k| k.to_string())
                )
            })
            .collect();
        lines.sort();
        static WRITE: Mutex<()> = Mutex::new(());
        let _guard = WRITE.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap_or_else(|e| panic!("EIN_TEST_UNITS={}: {e}", path.to_string_lossy()));
        for l in lines {
            writeln!(out, "{l}").expect("EIN_TEST_UNITS is writable");
        }
    }

    let mut results = Vec::with_capacity(units.len());
    for slot in slots {
        match slot.expect("every unit ran").0 {
            Done::Ok(r) => results.push(r),
            Done::Panicked(p, line) => {
                eprintln!("{line}");
                resume_unwind(p);
            }
        }
    }
    results
}

/// The process's peak resident set back to its current one (Linux 4.0+),
/// and that current one in kB, which the unit's peak is read above; 0 where
/// `/proc` has no such file.
fn reset_peak() -> u64 {
    let _ = std::fs::write("/proc/self/clear_refs", "5");
    status_kb("VmRSS:").unwrap_or(0)
}

/// A `kB` line of `/proc/self/status`: `VmHWM:`, the peak resident set since
/// the last reset, or `VmRSS:`, the resident set now.
fn status_kb(key: &str) -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find_map(|l| l.strip_prefix(key))
        .and_then(|v| v.split_whitespace().next()?.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Results in the units' order, whatever order the workers took them in.
    #[test]
    fn results_come_back_in_the_units_order() {
        let units: Vec<u64> = (0..200).collect();
        let got = map(
            "pool-order",
            &units,
            |u| (format!("p{u}"), String::new()),
            |u| {
                // Uneven work, so the finishing order is not the starting one.
                std::thread::sleep(std::time::Duration::from_micros((200 - u) * 20));
                u * 3
            },
        );
        assert_eq!(got, units.iter().map(|u| u * 3).collect::<Vec<_>>());
    }

    /// The first panicking unit in order is the one reported, with its own
    /// message, however many panicked and whichever finished first.
    #[test]
    fn the_first_panicking_unit_in_order_is_the_one_reported() {
        let units: Vec<u64> = (0..64).collect();
        let caught = catch_unwind(AssertUnwindSafe(|| {
            map(
                "pool-panic",
                &units,
                |u| (format!("p{u}"), String::new()),
                |u| {
                    if *u % 10 == 7 {
                        // The later ones finish first.
                        std::thread::sleep(std::time::Duration::from_millis(64 - u));
                        panic!("unit {u} failed");
                    }
                    *u
                },
            )
        }));
        let p = caught.expect_err("a unit panicked");
        assert_eq!(payload_text(&*p), "unit 7 failed");
    }

    /// The heavy lane names corpus programs, so a rename cannot empty it
    /// without a word.
    #[test]
    fn every_heavy_program_is_a_corpus_file() {
        let files = crate::corpus_files();
        for h in HEAVY {
            assert!(
                files.iter().any(|f| f.ends_with(h)),
                "pool::HEAVY names {h}, which is no corpus file"
            );
            assert!(is_heavy(h));
        }
    }

    /// The long programs are corpus programs too.
    #[test]
    fn every_long_program_is_a_corpus_file() {
        let files = crate::corpus_files();
        for l in LONG {
            assert!(
                files.iter().any(|f| f.ends_with(l)),
                "pool::LONG names {l}, which is no corpus file"
            );
        }
    }

    /// A map inside a unit runs inline: no permit is waited on twice.
    #[test]
    fn a_map_inside_a_unit_runs_inline() {
        let outer: Vec<u64> = (0..(jobs() as u64 * 2)).collect();
        let got = map(
            "pool-outer",
            &outer,
            |u| (format!("o{u}"), String::new()),
            |u| {
                let inner: Vec<u64> = (0..5).collect();
                map(
                    "pool-inner",
                    &inner,
                    |v| (format!("i{v}"), String::new()),
                    |v| u * 10 + v,
                )
                .iter()
                .sum::<u64>()
            },
        );
        assert_eq!(got, outer.iter().map(|u| u * 50 + 10).collect::<Vec<_>>());
    }
}
