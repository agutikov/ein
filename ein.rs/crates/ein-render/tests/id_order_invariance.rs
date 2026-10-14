//! T1a.10.1.4 — **the determinism sweep's successor**: one engine, permuted
//! ids, and the whole corpus has to answer the same way.
//!
//! The sweep it replaces ran `ein-conformance --env-a PYTHONHASHSEED=0
//! --env-b PYTHONHASHSEED=42 --strict` and found hazards H1 and H4. It asked
//! one question — *does an output depend on a hash-order accident rather than
//! on the data?* — and it asked it of ein.py, because ein.py is the engine
//! whose `hash()` is salted. ein.rs is not: `FxHashMap` hashes the same way
//! every run
//! ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §9),
//! so re-running it proves nothing and re-running it against a second engine
//! is not available after
//! [P1a.10](../../../../docs/history/m1a_rust/README.md#p1a10--one-implementation).
//!
//! What *is* available is the same question in this engine's own terms. Ids
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
//! So each file is run twice. The first run is ordinary. The second starts
//! from a `Terms` into which every name the first run interned has already
//! been interned *in a shuffled order*, every integer literal likewise, and a
//! run of junk facts ahead of the real ones so the `FactId` space is offset
//! too. Nothing is faked: the second run parses and loads the same file
//! through the same code, and the only difference is which integer each name
//! got. Both runs must produce the same bytes.
//!
//! This is strictly stronger than the grep. The grep finds an iteration whose
//! order *could* reach an output; this finds one that *does*, and it finds the
//! ones that are not iterations at all — a sort that fell back on `Symbol`'s
//! numeric order, a `min_by_key` on a `Value`, a set rendered in insertion
//! order. It is strictly weaker in one direction, and that is on the ledger:
//! it only sees what the corpus reaches.
//!
//! ## The seam
//!
//! `EIN_ID_FILES=<dir>` sweeps every `.ein` under `<dir>` instead of the
//! corpus. One caller: `utils/fuzz_ein.py`, whose third property is
//! [the ledger's L1](../../../../docs/history/m1a_rust/oracle_ledger.md#6-accepted-loss)
//! item 3 — *the same program under a permuted interner answers the same way*,
//! **applied to generated input rather than to the corpus**. The instrument
//! for that is this file and there is no second copy of it, so the fuzzer
//! writes a batch to a directory and points this sweep at it.
//!
//! Two of the assertions below are about the *corpus* rather than about the
//! engine — the coverage floor, and "some rendering moved" — and they are
//! skipped under the override, named as skipped in the summary line. The
//! invariance claim itself is not: that is the whole point of the seam.
//!
//! ```text
//! EIN_ID_SEEDS=8 cargo test -p ein-render --test id_order_invariance
//! EIN_ID_FILES=corpus/out/fuzz/cases cargo test -p ein-render --test id_order_invariance
//! ```

mod corpus_ops;

use std::path::PathBuf;

use corpus_ops::{Op, ops, run};
use ein_core::{FactId, IntId, Symbol, Tag, Terms, Value};
use ein_corpus::{corpus_files, ein_files_under, repo_root};
use ein_infer::mt19937::Mt19937;
use rustc_hash::FxHashMap;

/// A `Terms` whose id space is a permutation of `after`'s.
///
/// Every name `after` holds is interned again in shuffled order, every integer
/// literal likewise, and every **fact** is re-interned in an order shuffled
/// within its nesting depth — depth-blind would be wrong, since
/// `(not (color-loc Red House-1))` cannot be interned before the fact it
/// wraps, and a uniform offset would be no permutation at all: `FactId`s that
/// all move by the same amount keep their relative order, which is exactly
/// what a leak would read. `FACT_OFFSET` junk facts go in ahead of everything
/// so that even a file with one fact has its id displaced.
///
/// The kernel names keep ids 0–17 in both runs, because `Terms::new` interns
/// them before a caller can reach the table — that is not a gap in the
/// perturbation, it is what makes it legal: `Kernel` is a struct of `Symbol`s
/// the engine compares by identity, and an engine that read them out of the
/// table by number would be broken in a way no shuffle needs to prove.
fn permuted(after: &Terms, seed: i64) -> Terms {
    let mut rng = Mt19937::seeded(seed);
    let mut names: Vec<String> = (0..after.syms.len())
        .map(|i| after.syms.text(Symbol(i as u32)).to_string())
        .collect();
    rng.shuffle(&mut names);
    let mut ints: Vec<String> = (0..after.ints.len())
        .map(|i| after.ints.text(IntId(i as u32)).to_string())
        .collect();
    rng.shuffle(&mut ints);

    let mut terms = Terms::new();
    let junk = terms.intern_text("@id-order-probe").expect("room");
    for k in 0..FACT_OFFSET {
        let arg = terms.value_text(&format!("@probe-{k}")).expect("room");
        terms.intern_fact(junk, &[arg]).expect("room");
    }
    for name in &names {
        terms.intern_text(name).expect("room");
    }
    for text in &ints {
        terms.value_int(text).expect("room");
    }

    // Facts, by nesting depth so a wrapper never precedes what it wraps, and
    // shuffled inside each depth so the order within one is not the base run's.
    let n = after.facts.len();
    let mut depth = vec![0usize; n];
    for i in 0..n {
        let (_, args) = after.facts.get(FactId(i as u32));
        depth[i] = args
            .iter()
            .filter_map(|v| v.as_fact())
            .map(|f| depth[f.0 as usize] + 1)
            .max()
            .unwrap_or(0);
    }
    let mut by_depth: Vec<Vec<u32>> =
        vec![Vec::new(); depth.iter().copied().max().unwrap_or(0) + 1];
    for (i, d) in depth.iter().enumerate() {
        by_depth[*d].push(i as u32);
    }
    let mut moved_to: FxHashMap<u32, FactId> = FxHashMap::default();
    for bucket in &mut by_depth {
        rng.shuffle(bucket);
        for &i in bucket.iter() {
            let (rel, args) = after.facts.get(FactId(i));
            let rel_text = after.syms.text(rel).to_string();
            let args: Vec<Value> = args
                .iter()
                .map(|&v| remap(after, &mut terms, &moved_to, v))
                .collect();
            let rel = terms
                .intern_text(&rel_text)
                .expect("the relation name is already interned");
            let id = terms.intern_fact(rel, &args).expect("room");
            moved_to.insert(i, id);
        }
    }
    terms
}

/// One of `after`'s values, as the same value in `terms`.
///
/// By *text* for the two atomic shapes and by the depth-ordered map for the
/// nested one, so the proposition is the same proposition and only its id
/// moved. `UNBOUND` is not a value in a stored fact, and reaching this with
/// one would mean the store held a register sentinel.
fn remap(after: &Terms, terms: &mut Terms, moved: &FxHashMap<u32, FactId>, v: Value) -> Value {
    match v.tag() {
        Tag::Sym => {
            let text = after.syms.text(Symbol(v.payload())).to_string();
            terms.value_text(&text).expect("room")
        }
        Tag::Int => {
            let text = after.ints.text(IntId(v.payload())).to_string();
            terms.value_int(&text).expect("room")
        }
        Tag::Fact => Value::fact(moved[&v.payload()]),
    }
}

/// How far the fact space is displaced *before* the permutation. Small on
/// purpose: the point is that `FactId` 0 is not special, not that the store
/// scales.
const FACT_OFFSET: usize = 7;

/// How many ids the shuffle actually moved, and how many it *could* have —
/// the perturbation's own size, asserted so a run cannot pass because nothing
/// was permuted.
///
/// The kernel names are excluded from both halves: `Terms::new` interns them
/// before a caller can reach the table, so they sit at 0–17 in every run and
/// counting them as "unmoved" would understate every permutation by eighteen.
fn moved(after: &Terms, permuted: &Terms) -> (usize, usize) {
    let kernel = Terms::new().syms.len();
    let moved = (kernel..after.syms.len())
        .filter(|&i| {
            let name = after.syms.text(Symbol(i as u32));
            permuted.syms.get(name) != Some(Symbol(i as u32))
        })
        .count();
    (moved, after.syms.len() - kernel)
}

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

/// The file set, and whether it is the corpus.
///
/// `EIN_ID_FILES=<dir>` replaces it — see the module note. An override naming
/// a directory with no `.ein` in it is an error rather than an empty sweep: a
/// caller that mistyped a path would otherwise get a pass.
fn files_under_test() -> (Vec<PathBuf>, bool) {
    let Some(dir) = std::env::var_os("EIN_ID_FILES") else {
        return (corpus_files(), true);
    };
    let dir = PathBuf::from(dir);
    let files = ein_files_under(&dir);
    assert!(
        !files.is_empty(),
        "EIN_ID_FILES={} holds no .ein files",
        dir.display()
    );
    (files, false)
}

#[test]
fn no_observable_depends_on_the_order_ids_were_assigned_in() {
    let ops = ops();
    let (files, is_corpus) = files_under_test();
    let seeds = seeds();
    let (mut bad, mut compared, mut permutations) = (Vec::new(), 0usize, 0usize);
    // A pair whose op never interned anything has no ids to permute — every
    // `Op::Dot` parse view is one, because `dot_shape` answers those off the
    // AST and never builds a KB. Counted rather than skipped: a sweep that
    // reported 3 610 pairs while 1 615 of them tested nothing would be
    // claiming coverage it does not have.
    let mut vacuous = 0usize;
    let (mut moved_at_all, mut moved_stopping_point, mut moved_proof) = (0usize, 0usize, 0usize);
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    let mut weakest: Option<(usize, usize)> = None;

    for path in &files {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        for op in &ops {
            let mut base_terms = Terms::new();
            let Some(base) = run(&mut base_terms, path, *op) else {
                continue;
            };
            let mut tested = false;
            for &seed in &seeds {
                let mut terms = permuted(&base_terms, seed);
                let (moved, permutable) = moved(&base_terms, &terms);
                if permutable == 0 {
                    continue;
                }
                tested = true;
                if weakest.is_none_or(|(m, p)| moved * p < m * permutable) {
                    weakest = Some((moved, permutable));
                }
                permutations += 1;
                let Some(got) = run(&mut terms, path, *op) else {
                    bad.push(format!(
                        "{} [{op}] seed {seed}: refused only when permuted",
                        rel.display()
                    ));
                    continue;
                };
                match moved_by_the_cut(*op, &base, &got) {
                    Moved::No => continue,
                    Moved::StoppingPoint => moved_stopping_point += 1,
                    Moved::Proof => {
                        assert!(
                            !got.trim().is_empty(),
                            "{} [{op}] rendered nothing",
                            rel.display()
                        );
                        moved_proof += 1;
                    }
                    Moved::Answer(x, y) => {
                        *tally.entry(op.to_string()).or_insert(0) += 1;
                        bad.push(format!(
                            "{} [{op}] seed {seed}\n{}",
                            rel.display(),
                            first_difference(&x, &y)
                        ));
                    }
                }
                moved_at_all += 1;
            }
            compared += 1;
            if !tested {
                vacuous += 1;
            }
        }
    }

    if std::env::var("EIN_ID_REPORT").is_ok() {
        for (op, n) in &tally {
            eprintln!("  {n:5} {op}");
        }
        eprintln!("  total differing: {}", bad.len());
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} (file, op) pairs move when the ids do:\n\n{}",
        bad.len(),
        bad.iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    );
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
    // The cut has to be load-bearing here too, or it is a tolerance nobody is
    // examining: if no rendering moves under a permutation, the claim below
    // about *which* observables move has stopped being measured.
    // …over the corpus. A caller-named file set is whatever it is: a batch of
    // small generated programs can legitimately move nothing, and reporting
    // that as a broken instrument would make the seam useless.
    assert!(
        !is_corpus || ein_parity::strict() || moved_at_all > 0,
        "no rendering moved under a permuted id space — \
         either the perturbation stopped perturbing or the proof stopped depending on it"
    );
    assert!(
        !is_corpus || compared - vacuous >= 1500,
        "only {} (file, op) pairs had ids to permute — the sweep stopped looking",
        compared - vacuous
    );
    eprintln!(
        "id-order invariance{}: {} pairs permuted ({vacuous} had no ids to permute) \
         over {} files × {} ops × {} seeds, {permutations} permutations, \
         {moved_at_all} moved ({moved_stopping_point} only where a dying fork stopped, \
         {moved_proof} only in the derivation they narrate), 0 answers differ; \
         weakest permutation moved {moved}/{permutable}",
        if is_corpus {
            String::new()
        } else {
            " [EIN_ID_FILES; the corpus-shaped floors are skipped]".to_string()
        },
        compared - vacuous,
        files.len(),
        ops.len(),
        seeds.len()
    );
}

/// What a difference between two runs of one op amounts to.
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
/// the cut decides — and the cut is `ein-parity`'s, applied here between two
/// runs of one engine rather than between two engines. See the module note.
///
/// One function because two tests ask it: the sweep, and the register's
/// id-order rows, which must count as moved exactly what the sweep would
/// report.
fn moved_by_the_cut(op: Op, base: &str, got: &str) -> Moved {
    if got == base {
        return Moved::No;
    }
    if !ein_parity::strict() && ein_parity::blank(base) == ein_parity::blank(got) {
        return Moved::StoppingPoint;
    }
    match (op.narrow(base), op.narrow(got)) {
        (Some(x), Some(y)) if x != y => Moved::Answer(x, y),
        _ => Moved::Proof,
    }
}

/// M1e2 S1e2.1.1 — **the register's id-order rows**, whose wrong answer is
/// visible only under a permuted interner (W11).
///
/// `corpus/wrong_answers.toml` is otherwise held by
/// `ein-cli/tests/wrong_answers.rs`, through the binary; a permutation is not
/// something the binary can be asked for, so a row carrying `wrong.id_order`
/// has its second half here, where the permutation is. An **open** row's op
/// must still move at its seed by the same cut the sweep applies — which is
/// why the cut is a function — and a **fixed** one must not. The witnesses are
/// under `corpus/fuzz_findings/` for the reason this file is: as corpus
/// entries they would fail the sweep above.
#[test]
fn the_register_s_id_order_rows_answer_what_they_record() {
    let ops = ops();
    let mut checked = 0usize;
    let mut bad = Vec::new();
    for row in ein_corpus::register().case {
        let Some(id) = &row.wrong.id_order else {
            continue;
        };
        let op = *ops
            .iter()
            .find(|o| o.to_string() == id.op)
            .unwrap_or_else(|| panic!("{}: no op named {:?}", row.id, id.op));
        let path = repo_root().join(&row.witness);
        let mut base_terms = Terms::new();
        let base = run(&mut base_terms, &path, op)
            .unwrap_or_else(|| panic!("{}: [{op}] refused the plain run", row.id));
        let mut terms = permuted(&base_terms, id.seed);
        let got = run(&mut terms, &path, op)
            .unwrap_or_else(|| panic!("{}: [{op}] refused the permuted run", row.id));
        let moved = matches!(moved_by_the_cut(op, &base, &got), Moved::Answer(..));
        checked += 1;
        match (row.status.as_str(), moved) {
            ("open" | "not-wrong", false) => bad.push(format!(
                "{}: [{op}] seed {} no longer moves — an OPEN case changed answer; \
                 the row moves in the same commit",
                row.id, id.seed
            )),
            ("fixed", true) => bad.push(format!(
                "{}: [{op}] seed {} is FIXED and moves:\n{}",
                row.id,
                id.seed,
                first_difference(&base, &got)
            )),
            _ => {}
        }
    }
    assert!(checked > 0, "the register has no id-order rows to check");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
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
