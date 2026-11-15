//! T1a.10.1.4 and T1a.10.1.2 — **every rendering of every corpus file,
//! banked, and none of them depending on the order ids were assigned in.**
//!
//! Two claims over one pass since M1f S1f.6.39 (the user's ruling of
//! 2026-11-15 on Q-M1f.287): every (file, op) pair of [`corpus_ops`] is
//! rendered once in the ordinary id space and digested against the golden,
//! and the same rendering is the baseline a permuted id space is held to.
//! `corpus_shapes.rs` made the first pass until then, and this sweep computed
//! the same bytes again for its baseline (S1f.6.39's E-72).
//!
//! ## The golden
//!
//! Every observable surface of every corpus file — the read-out under each
//! question, the region trace, the states, the lattice, every core and the
//! oracle's cube, every DOT view, and the shapes behind them — is digested
//! against a checked-in manifest, `tests/golden/corpus_shapes.md5`, one line
//! per pair: sixteen hex digits of its md5, its line count and its name
//! (`corpus_ops::manifest_line`). The lines whose rendering quotes a
//! *resolved path* are banked with the checkout root replaced by `<repo>`
//! (`corpus_ops::portable`): the prefix is the blessing machine's directory,
//! never the engine's answer.
//!
//! It says **that** a rendering moved, on which file, in which view, and by
//! how many lines; not **what** moved, which the engine is right there to
//! show. The full tree of renderings is tens of megabytes, which no repo
//! holds and nobody reviews. It is not evidence that the bytes are *right*:
//! a golden says "still what it was", and the claim "and what it was is what
//! the semantics say" is the acceptance fixtures' and the stdlib
//! conformance suite's. Its job is to make a semantic change **visible**. The
//! `load` and `saturate` ops carry **coverage floors**, 60 files loaded and
//! 50 files with 3 000 events saturated, because a sweep that compares
//! nothing agrees about nothing.
//!
//! ## The determinism sweep
//!
//! It asks one question — *does an output depend on an order accident rather
//! than on the data?* — and it cannot ask it by re-running: `FxHashMap`
//! hashes the same way every run
//! ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §9),
//! so two runs of one build agree whether or not an output leaks an order.
//!
//! What *is* available is the question in this engine's own terms. Ids
//! here are **assignment-ordered**: a [`Symbol`] is "how many distinct names
//! had been seen when this one arrived", and
//! [`ein_core::intern`](../../ein-core/src/intern.rs) says so —
//! `Symbol` deliberately has no `Ord`, observable sorts go through
//! `Interner::rank`, and iterating an `FxHashMap` keyed on one is the thing
//! `utils/check_hashmap_iteration.py` greps for
//! ([design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §1).
//! Every one of those claims is falsified by the same experiment: **assign the
//! ids in a different order and see whether anything moves.**
//!
//! So a file is run twice. The first run is the ordinary one the golden
//! digests. The second starts from a `Terms` into which every name the first
//! run interned has already been interned *in a shuffled order*, and a run of
//! junk facts ahead of the real ones so the `FactId` space is offset too.
//! Nothing is faked: the second run parses and loads the same file through
//! the same code, and the only difference is which integer each name got.
//! Both runs must produce the same bytes. The shuffled table is a permutation
//! of *the first run's own* names and facts, the ones its forks derived
//! among them, so the first run cannot be read off the golden instead
//! (E-72).
//!
//! This is strictly stronger than the grep. The grep finds an iteration whose
//! order *could* reach an output; this finds one that *does*, and it finds the
//! ones that are not iterations at all — a sort that fell back on `Symbol`'s
//! numeric order, a `min_by_key` on a `Value`, a set rendered in insertion
//! order. It is strictly weaker in one direction: it only sees what the
//! corpus reaches.
//!
//! ## Which programs, per commit and in the nightly
//!
//! The user's rulings of 2026-11-15 on Q-M1f.289 and .290. **Per commit** the
//! golden holds every program **outside `tests/`**, whose programs are each
//! one stdlib rule's claim and are checked by their claims (`ein test`, the
//! coverage census, the mutants); and the permutation runs over those of them
//! that **search**, whose describe saturates more than root
//! (`corpus_ops::searches`, computed from the program, never a list): where
//! the region routes fan out and intern what their forks derive. **The
//! nightly**, `EIN_CORPUS_SLOW=1`, holds every corpus program to both, the
//! golden's every line, in shards (`EIN_TEST_SHARD=k/n`, `ein_corpus::shard`,
//! Q-M1f.293). A per-commit run compares the golden's lines of the programs
//! it reads and leaves the others' to the nightly, and `EIN_BLESS=1` re-banks
//! the lines it computed and keeps the others; a line whose program is no
//! corpus file fails every run. Each program is loaded once and every op runs
//! on a copy (`corpus_ops::Loads`, Q-M1f.292); a permuted run loads afresh,
//! into its own id space.
//!
//! ```text
//! cargo test -p ein-render --test id_order_invariance                  # what moved, per commit
//! EIN_BLESS=1 cargo test -p ein-render --test id_order_invariance      # accept it, outside tests/
//! EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test -p ein-render --test id_order_invariance   # every line
//! ```
//!
//! ## The seam
//!
//! `EIN_ID_FILES=<dir>` sweeps every `.ein` under `<dir>` instead of the
//! corpus, every one permuted and none banked. One caller:
//! `utils/fuzz_engine.py`, whose third property is *the same program under a
//! permuted interner answers the same way*, **applied to generated input
//! rather than to the corpus**. The instrument for that is this file and
//! there is no second copy of it, so the fuzzer writes a batch to a directory
//! and points this sweep at it. The assertions about the *corpus* rather than
//! about the engine — the coverage floors and the golden — are skipped under
//! the override, named as skipped in the summary line. Since M1e2 S1e2.2.12
//! the claim is that nothing moves, narration included: a resumed fork's
//! delta was in `FactId` order, which made a dead node's derivations arrive in
//! interning order, and the floor that asked for *some rendering moved*, the
//! narration cut's evidence of use, turned over with it.
//!
//! The per-commit tier runs one permutation; eight are the dev host's,
//! `./run_tests.sh --dev-host`, since M1f S1f.5.27, the CI nightly's until
//! then.
//!
//! ```text
//! EIN_ID_SEEDS=8 EIN_CORPUS_SLOW=1 cargo test -p ein-render --test id_order_invariance
//! EIN_ID_FILES=corpus/out/fuzz/cases cargo test -p ein-render --test id_order_invariance
//! ```

mod corpus_ops;
mod id_permutation;

use std::path::{Path, PathBuf};

use std::collections::{BTreeMap, BTreeSet};

use corpus_ops::{Loads, Op, manifest_line, ops, pair_name, portable, run};
use ein_core::{Symbol, Terms};
use ein_corpus::{corpus_files, ein_files_under, golden_path, pool, repo_root, shard, under_tests};
use id_permutation::{moved, permuted};

/// `EIN_ID_SEEDS` permutations per file (default 1). More seeds is more of the
/// same question, not a different one: one permutation already displaces every
/// non-kernel id, so a second finds a leak the first missed only when the leak
/// is order-*sensitive* rather than order-*dependent*.
fn seeds() -> Vec<i64> {
    let n: usize = std::env::var("EIN_ID_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    (0..n.max(1) as i64).map(|i| 1 + i * 7919).collect()
}

/// `EIN_CORPUS_SLOW=1`: the nightly's sweep, every corpus program banked and
/// permuted; unset, the per-commit one (the module note).
fn slow_too() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
}

/// The file set, and what this run does with it.
struct Sweep {
    files: Vec<PathBuf>,
    /// The corpus, banked and held to its floors; `false` under
    /// `EIN_ID_FILES`.
    corpus: bool,
    /// Every file permuted; otherwise only the programs that search.
    permute_all: bool,
}

/// The file set: per commit every corpus program outside `tests/`, under
/// `EIN_CORPUS_SLOW=1` every one, either way this run's shard of it; or
/// `EIN_ID_FILES`'s.
///
/// `EIN_ID_FILES=<dir>` replaces it — see the module note. An override naming
/// a directory with no `.ein` in it is an error rather than an empty sweep: a
/// caller that mistyped a path would otherwise get a pass.
fn files_under_test() -> Sweep {
    let Some(dir) = std::env::var_os("EIN_ID_FILES") else {
        let root = repo_root();
        let slow = slow_too();
        let files: Vec<PathBuf> = corpus_files()
            .into_iter()
            .filter(|f| slow || !under_tests(f.strip_prefix(&root).unwrap_or(f)))
            .collect();
        let files = shard::keep(files, |f| {
            f.strip_prefix(&root).unwrap_or(f).display().to_string()
        });
        return Sweep {
            files,
            corpus: true,
            permute_all: slow,
        };
    };
    let dir = PathBuf::from(dir);
    let files = ein_files_under(&dir);
    assert!(
        !files.is_empty(),
        "EIN_ID_FILES={} holds no .ein files",
        dir.display()
    );
    Sweep {
        files,
        corpus: false,
        permute_all: true,
    }
}

/// One seed's run of one (file, op) pair.
#[derive(Debug)]
struct SeedRun {
    /// How many ids the permutation moved, of how many it could.
    moved: usize,
    permutable: usize,
    outcome: Outcome,
}

/// What the permuted run came to, beside the plain one.
#[derive(Debug)]
enum Outcome {
    /// Refused only when permuted.
    Refused(String),
    /// What the cut says moved; [`Moved::Answer`] with its report line.
    Cut(Moved, Option<String>),
}

/// **One (file, op) pair, every seed**: the plain run, whose `Terms` the
/// permutation is built from, and a permuted run per seed, each held to it
/// through the cut. `None` where the plain run has nothing for the op.
///
/// The plain run is the one the golden digests (M1f S1f.6.39, Q-M1f.287),
/// computed here and not read off the golden as `jobs_invariance`'s baseline
/// is: the permuted id space is a permutation of *that run's* names and
/// facts, which no digest holds (S1f.6.39's E-72). `run` is `corpus_ops::run` in the sweep; a test passes
/// one with an id leaked into it
/// ([`an_id_order_that_reaches_an_output_is_caught`]).
fn pair(
    path: &Path,
    rel: &Path,
    op: Op,
    seeds: &[i64],
    plain: (Option<String>, Terms),
    run: &dyn Fn(&mut Terms, &Path, Op) -> Option<String>,
) -> Option<Vec<SeedRun>> {
    let (base, base_terms) = plain;
    let base = base?;
    let mut out = Vec::new();
    for &seed in seeds {
        let mut terms = permuted(&base_terms, seed);
        let (moved, permutable) = moved(&base_terms, &terms);
        if permutable == 0 {
            continue;
        }
        let outcome = match run(&mut terms, path, op) {
            None => Outcome::Refused(format!(
                "{} [{op}] seed {seed}: refused only when permuted",
                rel.display()
            )),
            Some(got) => match moved_by_the_cut(op, &base, &got) {
                Moved::Proof => {
                    assert!(
                        !got.trim().is_empty(),
                        "{} [{op}] rendered nothing",
                        rel.display()
                    );
                    Outcome::Cut(Moved::Proof, None)
                }
                Moved::Answer(x, y) => {
                    let line = format!(
                        "{} [{op}] seed {seed}\n{}",
                        rel.display(),
                        first_difference(&x, &y)
                    );
                    Outcome::Cut(Moved::Answer(x, y), Some(line))
                }
                m => Outcome::Cut(m, None),
            },
        };
        out.push(SeedRun {
            moved,
            permutable,
            outcome,
        });
    }
    Some(out)
}

/// What one (file, op) unit came to.
struct Rendered {
    /// The manifest line, where the run banks the program.
    line: Option<String>,
    bytes: usize,
    loaded: bool,
    events: Option<usize>,
    /// Whether the pair's program was permuted.
    permuted: bool,
    /// The permuted runs, each seed's; `None` where the plain run had nothing
    /// or the program was not permuted.
    runs: Option<Vec<SeedRun>>,
}

#[test]
fn every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order() {
    let ops = ops();
    let Sweep {
        files,
        corpus,
        permute_all,
    } = files_under_test();
    let seeds = seeds();
    let root = repo_root();
    let root_str = root.display().to_string();
    if corpus {
        // Twenty-one of the renderings name the stdlib root, because
        // `(import std.nope)` reports the path it looked at — and
        // `stdlib::resolve_default` finds that root by walking up from the
        // test executable. A target directory outside the checkout, or an
        // `$EIN_STDLIB`, therefore resolves a *different* stdlib and moves all
        // twenty-one. That is a difference in what ran rather than drift in
        // what it produced, so it is said once here instead of arriving as
        // twenty-one digests for someone to bless.
        let stdlib = ein_ir::stdlib::resolve_default();
        assert_eq!(
            stdlib,
            ein_ir::stdlib::Source::Checkout(root.join("stdlib")),
            "the manifest is of the checkout's stdlib; this run resolved {} — \
             build inside the checkout and leave $EIN_STDLIB unset",
            stdlib.describe()
        );
    }

    // The pairs, on the pool (M1f S1f.6.39 T1f.6.39.4), read back in their
    // order, so every count, the manifest and the first twenty reports are
    // the serial loop's. The plain run on a copy of the program's one load
    // (Q-M1f.292); the permuted ones load into their own id space.
    let units: Vec<(usize, usize)> = (0..files.len())
        .flat_map(|f| (0..ops.len()).map(move |o| (f, o)))
        .collect();
    let rel = |f: usize| {
        files[f]
            .strip_prefix(&root)
            .unwrap_or(&files[f])
            .to_path_buf()
    };
    let loads = Loads::new(&files, &vec![ops.len(); files.len()]);
    let rendered = pool::map(
        "id_order_invariance",
        &units,
        |&(f, o)| (rel(f).display().to_string(), ops[o].to_string()),
        |&(f, o)| {
            let op = ops[o];
            let unit = loads.unit(f);
            let plain = unit.run(op, 1);
            let text = plain.0.as_deref();
            let out = Rendered {
                line: text
                    .filter(|_| corpus)
                    .map(|t| manifest_line(&pair_name(&rel(f), op), &portable(&root_str, t))),
                bytes: text.map_or(0, str::len),
                loaded: matches!(op, Op::Load) && text.is_some_and(|t| !t.starts_with("<refused>")),
                events: text
                    .filter(|_| matches!(op, Op::Saturate))
                    .map(|t| t.lines().count()),
                permuted: permute_all || unit.searches(),
                runs: None,
            };
            if !out.permuted {
                return out;
            }
            Rendered {
                runs: pair(&files[f], &rel(f), op, &seeds, plain, &|t, p, op| {
                    run(t, p, op)
                }),
                ..out
            }
        },
    );

    let mut lines: Vec<String> = Vec::new();
    let (mut bytes, mut loads, mut saturates, mut events) = (0usize, 0usize, 0usize, 0usize);
    let mut permuted_programs: BTreeSet<usize> = BTreeSet::new();
    let (mut bad, mut compared, mut permutations) = (Vec::new(), 0usize, 0usize);
    // A pair whose op never interned anything has no ids to permute — every
    // `Op::Dot` parse view is one, because `dot_shape` answers those off the
    // AST and never builds a KB. Counted rather than skipped: a sweep that
    // reported 3 610 pairs while 1 615 of them tested nothing would be
    // claiming coverage it does not have.
    let mut vacuous = 0usize;
    let (mut moved_at_all, mut moved_stopping_point, mut moved_proof) = (0usize, 0usize, 0usize);
    let mut tally: BTreeMap<String, usize> = Default::default();
    let mut weakest: Option<(usize, usize)> = None;
    for (&(f, o), r) in units.iter().zip(rendered) {
        let op = ops[o];
        lines.extend(r.line);
        bytes += r.bytes;
        loads += usize::from(r.loaded);
        if let Some(n) = r.events {
            saturates += 1;
            events += n;
        }
        if r.permuted {
            permuted_programs.insert(f);
        }
        let Some(runs) = r.runs else {
            continue;
        };
        let mut tested = false;
        for r in runs {
            tested = true;
            if weakest.is_none_or(|(m, p)| r.moved * p < m * r.permutable) {
                weakest = Some((r.moved, r.permutable));
            }
            permutations += 1;
            match r.outcome {
                Outcome::Refused(line) => {
                    bad.push(line);
                    continue;
                }
                Outcome::Cut(Moved::No, _) => continue,
                Outcome::Cut(Moved::StoppingPoint, _) => moved_stopping_point += 1,
                Outcome::Cut(Moved::Proof, _) => moved_proof += 1,
                Outcome::Cut(Moved::Answer(..), line) => {
                    *tally.entry(op.to_string()).or_insert(0) += 1;
                    bad.extend(line);
                }
            }
            moved_at_all += 1;
        }
        compared += 1;
        if !tested {
            vacuous += 1;
        }
    }
    if std::env::var("EIN_ID_REPORT").is_ok() {
        for (op, n) in &tally {
            eprintln!("  {n:5} {op}");
        }
        eprintln!("  total differing: {}", bad.len());
    }

    // The golden first: what a change to the engine moves is most often a
    // rendering, and the list of the renderings that moved is what to read.
    let mut failures: Vec<String> = Vec::new();
    if corpus {
        let covered: BTreeSet<String> = (0..files.len())
            .map(|f| rel(f).display().to_string())
            .collect();
        if let Some(msg) = held_to_the_golden(&lines, &covered) {
            failures.push(msg);
        }
    }
    if !bad.is_empty() {
        failures.push(format!(
            "{} of {compared} (file, op) pairs move when the ids do:\n\n{}",
            bad.len(),
            bad.iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n\n")
        ));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));

    // A permutation that permuted nothing compares an engine against itself.
    // A shuffle has one fixed point on average, so the weakest one should move
    // nearly everything; the bar is half, which no real permutation of eight
    // or more names comes near failing.
    let (moved, permutable) = weakest.unwrap_or((0, 0));
    assert!(
        permutable > 0 && moved * 2 >= permutable,
        "the weakest permutation moved {moved} of {permutable} ids — \
         the sweep is comparing a run against itself"
    );
    // Nothing moves, narration included — M1e2 S1e2.2.12. The cut above was
    // load-bearing here until a resumed fork's delta stopped being sorted by
    // `FactId`: a dead node's derivations arrived in interning order, so which
    // of a fact's equally good derivations a fork recorded first, the rules
    // `refuted_by` names, and the core a full list leaves, moved with the ids
    // (158 renderings over the corpus at eight seeds, 2026-11-04). The delta
    // is in the order the caller wrote it since, and a rendering that moves
    // at all is the id leak this sweep exists for, whichever side of the cut
    // it falls on.
    assert!(
        moved_at_all == 0,
        "{moved_at_all} renderings moved under a permuted id space \
         ({moved_stopping_point} only where a dying fork stopped, {moved_proof} only in the \
         derivation they narrate) — an order the ids were assigned in reached an output"
    );
    if corpus {
        // The floors: a manifest that shrank because a file stopped parsing
        // would otherwise be blessed as progress. A shard holds its share
        // (`ein_corpus::shard::share`).
        let floor = shard::share;
        assert!(
            lines.len() >= floor(4_000),
            "only {} renderings were produced — the sweep stopped looking",
            lines.len()
        );
        assert!(
            loads >= floor(60),
            "only {loads} corpus files loaded — the floor is {}",
            floor(60)
        );
        assert!(
            saturates >= floor(50) && events >= floor(3_000),
            "only {saturates} files / {events} events saturated — \
             the floor is {} and {}",
            floor(50),
            floor(3_000)
        );
        assert!(
            compared - vacuous >= floor(1500),
            "only {} (file, op) pairs had ids to permute — the sweep stopped looking",
            compared - vacuous
        );
        // **The programs that search are a part of what is banked, and a
        // real one** (Q-M1f.289): per commit the permutation runs over them,
        // so a predicate that held of none would leave the id order checked
        // nowhere, and one that held of all would be no reduction.
        if !permute_all {
            assert!(
                permuted_programs.len() >= floor(100) && permuted_programs.len() < files.len(),
                "{} of the {} programs read search, as `corpus_ops::searches` reads them — \
                 the set the permutation runs over per commit has lost its meaning",
                permuted_programs.len(),
                files.len()
            );
        }
    }
    eprintln!(
        "id-order invariance{}{}: {} renderings banked ({:.1} MB of text); {} pairs permuted \
         ({vacuous} had no ids to permute) over {} of {} files × {} ops × {} seeds, \
         {permutations} permutations, {moved_at_all} moved ({moved_stopping_point} only where a \
         dying fork stopped, {moved_proof} only in the derivation they narrate), 0 answers \
         differ; weakest permutation moved {moved}/{permutable}",
        if corpus {
            if permute_all {
                " [every corpus program]"
            } else {
                " [outside tests/, the programs that search permuted]"
            }
        } else {
            " [EIN_ID_FILES; the corpus-shaped floors and the golden are skipped]"
        },
        shard::current().map_or(String::new(), |s| format!(" [shard {s}]")),
        lines.len(),
        bytes as f64 / 1e6,
        compared - vacuous,
        permuted_programs.len(),
        files.len(),
        ops.len(),
        seeds.len()
    );
}

/// **The run's lines against the golden's**, over the programs the run
/// read: `None` when they agree, or were just re-banked under `EIN_BLESS=1`,
/// and otherwise the message, the renderings that moved named.
///
/// The golden holds every corpus program's lines; a run compares those of
/// the programs it read (`covered`) and leaves the rest to the run that reads
/// them, the nightly's for `tests/` and the other shards'. A line whose
/// program is no corpus file is gone, and fails every run. Blessed, the
/// golden is the run's lines and every other corpus program's lines as they
/// were, in the corpus's order, so a per-commit bless and a full one write
/// the same file when nothing outside the per-commit set moved.
fn held_to_the_golden(got: &[String], covered: &BTreeSet<String>) -> Option<String> {
    let path = golden_path("ein-render", "corpus_shapes.md5");
    let root = repo_root();
    let corpus: BTreeMap<String, usize> = corpus_files()
        .iter()
        .enumerate()
        .map(|(i, p)| (p.strip_prefix(&root).unwrap_or(p).display().to_string(), i))
        .collect();
    let op_at: BTreeMap<String, usize> = ops()
        .iter()
        .enumerate()
        .map(|(i, o)| (o.to_string(), i))
        .collect();
    let name = |l: &str| {
        l.split_once("L  ")
            .map_or(String::new(), |(_, n)| n.to_string())
    };
    let program = |n: &str| {
        n.split_once("::")
            .map_or(n.to_string(), |(p, _)| p.to_string())
    };
    let old = std::fs::read_to_string(&path).unwrap_or_default();
    let (mut want, mut kept, mut gone) = (BTreeMap::new(), Vec::new(), Vec::new());
    for l in old.lines() {
        let n = name(l);
        let p = program(&n);
        if covered.contains(&p) {
            want.insert(n, l.to_string());
        } else if corpus.contains_key(&p) {
            kept.push(l.to_string());
        } else {
            gone.push(n);
        }
    }
    if std::env::var("EIN_BLESS").as_deref() == Ok("1") {
        let mut all: Vec<String> = kept.into_iter().chain(got.iter().cloned()).collect();
        all.sort_by_key(|l| {
            let n = name(l);
            let (p, o) = n.split_once("::").unwrap_or((&n, ""));
            (
                corpus.get(p).copied().unwrap_or(usize::MAX),
                op_at.get(o).copied().unwrap_or(usize::MAX),
                n.clone(),
            )
        });
        let text = format!("{}\n", all.join("\n"));
        std::fs::write(&path, text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        return None;
    }
    let got: BTreeMap<String, String> = got.iter().map(|l| (name(l), l.clone())).collect();
    let digest = |l: &str| {
        l.split_once("L  ")
            .map_or(String::new(), |(d, _)| d.to_string())
    };
    let mut out = Vec::new();
    for (n, g) in &got {
        match want.get(n) {
            None => out.push(format!("  + {n} (new)")),
            Some(w) if w != g => out.push(format!(
                "  ~ {n}\n      was {}\n      now {}",
                digest(w),
                digest(g)
            )),
            _ => {}
        }
    }
    for n in want.keys() {
        if !got.contains_key(n) {
            out.push(format!("  - {n} (gone)"));
        }
    }
    for n in &gone {
        out.push(format!("  - {n} (gone: its program is no corpus file)"));
    }
    // A corpus program this run does not read still has its lines: a program
    // added under `tests/` is banked by a run that reads it, and is caught
    // here, per commit, when nothing has. Unsharded runs only, since a shard
    // leaves the other shards' programs unread.
    if shard::current().is_none() {
        let banked: BTreeSet<String> = kept.iter().map(|l| program(&name(l))).collect();
        for p in corpus.keys() {
            if !covered.contains(p) && !banked.contains(p) {
                out.push(format!(
                    "  + {p} (no line: bank it with EIN_BLESS=1 EIN_CORPUS_SLOW=1)"
                ));
            }
        }
    }
    if out.is_empty() {
        return None;
    }
    Some(format!(
        "corpus_shapes.md5: {} of the {} renderings of the {} programs read moved:\n{}\n  \
         re-bank with EIN_BLESS=1 cargo test -p ein-render --test id_order_invariance \
         (and EIN_CORPUS_SLOW=1 for every corpus program's lines)",
        out.len(),
        want.len(),
        covered.len(),
        out.iter().take(25).cloned().collect::<Vec<_>>().join("\n")
    ))
}

/// What a difference between two runs of one op amounts to.
#[derive(Debug)]
enum Moved {
    /// The two renderings are byte-identical.
    No,
    /// Only line-level values moved: a firing count, an event ordinal, a dying
    /// fork's core — where `enable_fail_fast_fork` stopped a fork it was about
    /// to discard.
    StoppingPoint,
    /// The narration of the proof moved and the answer did not: a fact with
    /// two equally valid justifications recorded the other one first, or a
    /// `slice` view compared for presence only.
    Proof,
    /// The answer moved — the two narrowed renderings, plain and permuted.
    Answer(String, String),
}

/// Whether the *answer* or the *narration of the proof* moved, which is what
/// the cut decides — and the cut is `ein-narration`'s, applied here between two
/// runs that differ only in their id space. See the module note.
///
/// One function because two tests ask it: the sweep, and the register's
/// id-order rows, which must count as moved exactly what the sweep would
/// report.
fn moved_by_the_cut(op: Op, base: &str, got: &str) -> Moved {
    if got == base {
        return Moved::No;
    }
    if !ein_narration::strict() && ein_narration::blank(base) == ein_narration::blank(got) {
        return Moved::StoppingPoint;
    }
    match (op.narrow(base), op.narrow(got)) {
        (Some(x), Some(y)) if x != y => Moved::Answer(x, y),
        _ => Moved::Proof,
    }
}

/// **The per-commit sets keep their meaning** (M1f S1f.6.39, the user's
/// rulings on Q-M1f.288 to .290). `tests/` is a path test, and the corpus has
/// programs on both sides of it; *searches* holds of a program whose
/// describe forks and of none whose describe is root alone, a dead root, a
/// load error or a parse error. The sweep itself asserts that the programs
/// that search are a real part of what it reads.
#[test]
fn the_per_commit_sets_keep_their_meaning() {
    let root = repo_root();
    let rels: Vec<PathBuf> = corpus_files()
        .iter()
        .map(|p| p.strip_prefix(&root).unwrap_or(p).to_path_buf())
        .collect();
    let inside = rels.iter().filter(|r| under_tests(r)).count();
    assert!(
        inside >= 100 && rels.len() - inside >= 400,
        "{inside} of the {} corpus programs are under tests/: the per-commit sweeps leave \
         those out, so the split has stopped meaning what Q-M1f.290 ruled on",
        rels.len()
    );
    assert!(under_tests(Path::new(
        "tests/stdlib/algebra/01_copiers.ein"
    )));
    assert!(!under_tests(Path::new("examples/arith/add.ein")));
    assert!(!under_tests(Path::new("examples/tests.ein")));
    for (p, searches) in SEARCHES {
        let path = root.join(p);
        assert!(path.is_file(), "{p} is gone: name another program");
        let loads = Loads::new(std::slice::from_ref(&path), &[1]);
        assert_eq!(
            loads.unit(0).searches(),
            searches,
            "{p}: corpus_ops::searches says {}, where it {}",
            !searches,
            if searches { "searches" } else { "does not" }
        );
    }
}

/// Programs on both sides of `corpus_ops::searches`, each named with why.
const SEARCHES: [(&str, bool); 5] = [
    // Two candidates, one of them dead: describe forks.
    ("examples/branching/02_one_dead_one_alive.ein", true),
    // A small puzzle: describe forks all the way down.
    ("examples/branching/05_mini_zebra.ein", true),
    // A calculation, no hypothesis: describe is root alone.
    ("examples/arith/add.ein", false),
    // A program the loader refuses.
    ("examples/broken/compile/activator_arity.ein", false),
    // A program that does not parse.
    ("examples/broken/bare_top_level_atom.ein", false),
];

/// M1e2 S1e2.1.1 — **the register's id-order rows**, whose walk answer was
/// visible only under a permuted interner (W11).
///
/// `corpus/wrong_answers.toml` is otherwise held by
/// `ein-cli/tests/wrong_answers.rs`, through the binary; a permutation is not
/// something the binary can be asked for, so a row carrying `wrong.id_order`
/// has its second half here, where the permutation is. Since M1e2 S1e2.1a.5
/// the walk op it names is the record of what moved, and what is run is the
/// row's `[case.now]` question at the same seed: its read-out, and since
/// S1e2.2.12 every core the run reports (`corpus_ops::core_ops`), the core of a member
/// of `N` being one the read-out does not print. A **fixed** row's must not
/// move by the cut the sweep applies, and an **open** one's must still. The
/// witnesses are under `corpus/fuzz_findings/` while a row is open, for the
/// reason this file is: as corpus entries they would fail the sweep above.
#[test]
fn the_register_s_id_order_rows_answer_what_they_record() {
    let ops = ops();
    let mut checked = 0usize;
    let mut bad = Vec::new();
    for row in ein_corpus::register().case {
        let Some(id) = &row.wrong.id_order else {
            continue;
        };
        let mode = match row.now.line.as_str() {
            "all" => "describe",
            line => line,
        };
        let path = repo_root().join(&row.witness);
        let mut moved = None;
        for name in [format!("read-out[{mode}]"), format!("cores[{mode}]")] {
            let op = *ops
                .iter()
                .find(|o| o.to_string() == name)
                .unwrap_or_else(|| panic!("{}: no op named {name:?}", row.id));
            let mut base_terms = Terms::new();
            let base = run(&mut base_terms, &path, op)
                .unwrap_or_else(|| panic!("{}: [{op}] refused the plain run", row.id));
            let mut terms = permuted(&base_terms, id.seed);
            let got = run(&mut terms, &path, op)
                .unwrap_or_else(|| panic!("{}: [{op}] refused the permuted run", row.id));
            if moved.is_none() && matches!(moved_by_the_cut(op, &base, &got), Moved::Answer(..)) {
                moved = Some((op, first_difference(&base, &got)));
            }
        }
        checked += 1;
        match (row.status.as_str(), moved) {
            ("open" | "not-wrong", None) => bad.push(format!(
                "{}: [read-out[{mode}]] and [cores[{mode}]] seed {} no longer move — an OPEN \
                 case changed answer; the row moves in the same commit",
                row.id, id.seed
            )),
            ("fixed", Some((op, diff))) => bad.push(format!(
                "{}: [{op}] seed {} is FIXED and moves:\n{diff}",
                row.id, id.seed
            )),
            _ => {}
        }
    }
    assert!(checked > 0, "the register has no id-order rows to check");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **An id order that reaches an output is caught** (M1f S1f.6.39, the
/// stage's probe kept as a test): a run that writes the name holding the
/// first id after the kernel's into its rendering — the shape of a sort that
/// fell back on `Symbol`'s numeric order — moves the answer under a permuted
/// id space, and the honest run moves nothing.
#[test]
fn an_id_order_that_reaches_an_output_is_caught() {
    let rel = PathBuf::from("examples/branching/02_one_dead_one_alive.ein");
    let path = repo_root().join(&rel);
    let op = ops()
        .into_iter()
        .find(|o| o.to_string() == "load")
        .expect("the load op");
    let first = Symbol(Terms::new().syms.len() as u32);
    let leaky = |t: &mut Terms, p: &Path, op: Op| {
        run(t, p, op).map(|text| format!("{text}\n--- first name {}", t.syms.text(first)))
    };
    let honest = |t: &mut Terms, p: &Path, op: Op| run(t, p, op);
    let plain = |run: &dyn Fn(&mut Terms, &Path, Op) -> Option<String>| {
        let mut t = Terms::new();
        let out = run(&mut t, &path, op);
        (out, t)
    };
    let runs = pair(&path, &rel, op, &[1], plain(&honest), &honest).expect("the program loads");
    assert!(
        runs.iter()
            .all(|r| matches!(r.outcome, Outcome::Cut(Moved::No, _))),
        "the honest run moved: {runs:?}"
    );
    let runs = pair(&path, &rel, op, &[1], plain(&leaky), &leaky).expect("the program loads");
    assert!(
        runs.iter()
            .any(|r| matches!(r.outcome, Outcome::Cut(Moved::Answer(..), Some(_)))),
        "a name read by its id did not move the answer: {runs:?}"
    );
}

fn first_difference(a: &str, b: &str) -> String {
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return format!("  line {}\n    plain:    {x}\n    permuted: {y}", i + 1);
        }
    }
    format!(
        "  same {} lines, then {} vs {}",
        a.lines().count().min(b.lines().count()),
        a.lines().count(),
        b.lines().count()
    )
}
