//! M1e2 [S1e2.4.2](../../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md)
//! — **a pairwise form prices its table before it allocates one.**
//!
//! `ein solve examples/saturation/type-exclusivity/pets.ein --models key`
//! held 8.1 GB for the key of its 11 339 states, and was OOM-killed where the
//! machine had less, because the key form built its pair table before
//! anything asked what the table would cost. Now the key form and
//! `--diff-models`' near pairing compare the cost with
//! [`PAIR_TABLE_BOUND`] first, and past it they decline with the numbers.
//!
//! *Allocates nothing* is measured here rather than asserted. A counting
//! allocator keeps the bytes live on the calling thread and their peak, and
//! every case reads the peak across one call:
//!
//! - **over the bound on k alone**, the peak is below one mask of the table,
//!   so not even the first was built;
//! - **over the bound on k and n**, below the masks `separating` would build;
//! - **under the bound**, the control: at least those masks, so the probe
//!   sees a table when one is built, and no more than
//!   [`key_pair_table_bytes`] and the read-out's text, so the price is not an
//!   underestimate.
//!
//! The diff gets the same three readings.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::Arc;

use ein_core::{Kb, Program, Terms};
use ein_render::models::{
    KeyOf, KeyOutcome, PAIR_TABLE_BOUND, diff_model_sets, key_pair_table_bytes, key_table_of,
};

thread_local! {
    static LIVE: Cell<isize> = const { Cell::new(0) };
    static PEAK: Cell<isize> = const { Cell::new(0) };
}

/// Counts on the *calling thread*, so the other tests `cargo test` runs in
/// parallel cannot perturb a reading (the `match_alloc.rs` precedent).
struct Counting;

fn note(delta: isize) {
    let _ = LIVE.try_with(|live| {
        let now = live.get() + delta;
        live.set(now);
        let _ = PEAK.try_with(|peak| {
            if now > peak.get() {
                peak.set(now);
            }
        });
    });
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            note(layout.size() as isize);
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            note(layout.size() as isize);
        }
        p
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() {
            note(new_size as isize - layout.size() as isize);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        note(-(layout.size() as isize));
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// `f`'s result, and the most the heap held during it above where it stood
/// when `f` began, on this thread.
fn peak_during<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let start = LIVE.with(Cell::get);
    PEAK.with(|p| p.set(start));
    let r = f();
    let peak = PEAK.with(Cell::get);
    (r, usize::try_from(peak - start).unwrap_or(0))
}

/// One mask of the key's pair table: k(k−1)/2 bits, in whole words.
fn mask_bytes(k: usize) -> usize {
    (k * (k - 1) / 2).div_ceil(64) * 8
}

/// `k` distinct states over `atoms` Boolean atoms `(p aJ)`: state `i` holds
/// atom `j` exactly when bit `j % bits` of `i` is set. Every atom varies, and
/// the states are distinct while `k ≤ 2^bits`. No relation is declared
/// functional, so each atom is one varying slot and `n = atoms`.
fn states(k: usize, atoms: usize, bits: usize) -> (Terms, Vec<Kb>) {
    assert!(k <= 1 << bits && atoms >= bits);
    let mut terms = Terms::new();
    let program = Arc::new(Program::new());
    let p = terms.intern_text("p").expect("interns");
    let args: Vec<_> = (0..atoms)
        .map(|j| terms.value_text(&format!("a{j}")).expect("interns"))
        .collect();
    let kbs = (0..k)
        .map(|i| {
            let mut kb = Kb::with_program(Arc::clone(&program));
            for (j, a) in args.iter().enumerate() {
                if i >> (j % bits) & 1 == 1 {
                    kb.add_fact(&mut terms, p, &[*a], None).expect("adds");
                }
            }
            kb
        })
        .collect();
    (terms, kbs)
}

fn key(terms: &Terms, kbs: &[Kb]) -> (KeyOutcome, usize) {
    let refs: Vec<&Kb> = kbs.iter().collect();
    peak_during(|| key_table_of(terms, &refs, true, "  ", KeyOf::Models))
}

fn declined(o: &KeyOutcome) -> Option<&str> {
    match o {
        KeyOutcome::Unaffordable(why) => Some(why),
        KeyOutcome::Table(_) => None,
    }
}

/// The price is the arithmetic the stage file states, on `pets.ein`'s 11 339
/// states over 15 slots, and the bound sits between the largest table the
/// corpus answers and the smallest it declines (2026-11-02).
#[test]
fn the_price_is_the_stated_arithmetic() {
    assert_eq!(key_pair_table_bytes(11_339, 15), 514_246_492);
    assert_eq!(key_pair_table_bytes(11_339, 1), 289_263_580);
    assert_eq!(PAIR_TABLE_BOUND, 16 << 20);
    // colors.ein: 169 states, 8 slots, answered.
    assert!(key_pair_table_bytes(169, 8) <= PAIR_TABLE_BOUND);
    // constraint-scopes.ein: 4 096 states, 9 slots, declined.
    assert!(key_pair_table_bytes(4_096, 9) > PAIR_TABLE_BOUND);
    assert_eq!(key_pair_table_bytes(0, 3), 0);
    assert_eq!(key_pair_table_bytes(1, 3), 0);
}

/// **Over the bound on k alone.** 4 096 states are 8 386 560 pairs: one
/// mask is 1 MiB, and the table over a single slot already 36 MiB. The form
/// declines before the decision variables, and holds less than one mask.
#[test]
fn over_the_bound_on_k_it_declines_before_the_first_mask() {
    let (k, n) = (4_096, 12);
    let (terms, kbs) = states(k, n, 12);
    assert!(key_pair_table_bytes(k, 1) > PAIR_TABLE_BOUND);
    let (out, peak) = key(&terms, &kbs);
    eprintln!(
        "k = {k}, n = {n}: price {} bytes, peak {peak}",
        key_pair_table_bytes(k, n)
    );
    let why = declined(&out).expect("declines");
    assert!(
        why.contains("the 4 096 states are 8 386 560 pairs")
            && why.contains("its size over a single varying slot")
            && why.contains("past the key form's bound of 16 MiB"),
        "{why}"
    );
    assert!(
        peak < mask_bytes(k),
        "held {peak} bytes, one mask is {}: a pair table was allocated",
        mask_bytes(k)
    );
}

/// **Over the bound on k and n.** 1 024 states fit on k alone, and 120
/// varying slots do not: 242 masks of 64 KiB and a 2 MiB order is 18 MiB.
/// The variables are built, linear in k × n, and no mask is.
#[test]
fn over_the_bound_on_n_it_declines_before_separating() {
    let (k, n) = (1_024, 120);
    let (terms, kbs) = states(k, n, 10);
    assert!(key_pair_table_bytes(k, 1) <= PAIR_TABLE_BOUND);
    assert!(key_pair_table_bytes(k, n) > PAIR_TABLE_BOUND);
    let (out, peak) = key(&terms, &kbs);
    eprintln!(
        "k = {k}, n = {n}: price {} bytes, peak {peak}",
        key_pair_table_bytes(k, n)
    );
    let why = declined(&out).expect("declines");
    assert!(why.contains("over 120 varying slots holds 18 MiB"), "{why}");
    let table = (n + 1) * mask_bytes(k);
    assert!(
        peak < table,
        "held {peak} bytes, and `separating`'s masks are {table}: they were allocated"
    );
}

/// **Under the bound, the control.** 1 024 states over 10 slots, 3.4 MiB:
/// the table is built and the key found, every slot of the ten. The peak is
/// at least the masks, which is what makes the two readings above a
/// measurement, and at most the price plus the table's text.
#[test]
fn under_the_bound_the_table_is_built_within_its_price() {
    let (k, n) = (1_024, 10);
    let (terms, kbs) = states(k, n, 10);
    let price = key_pair_table_bytes(k, n) as usize;
    assert!(price as u64 <= PAIR_TABLE_BOUND);
    let (out, peak) = key(&terms, &kbs);
    eprintln!(
        "k = {k}, n = {n}: price {} bytes, peak {peak}",
        key_pair_table_bytes(k, n)
    );
    let KeyOutcome::Table(lines) = out else {
        panic!("declined under the bound: {:?}", declined(&out));
    };
    assert!(
        lines[0].contains("determining key — 10 of 10 varying slots"),
        "{}",
        lines[0]
    );
    let text: usize = lines.iter().map(|l| l.capacity() + 24).sum();
    assert!(
        peak >= (n + 1) * mask_bytes(k),
        "held {peak} bytes, less than the {} its masks take: the probe does not see the table",
        (n + 1) * mask_bytes(k)
    );
    assert!(
        peak <= price + 2 * text + (1 << 20),
        "held {peak} bytes against a price of {price} and {text} of text"
    );
}

/// `n` models, all distinct: one fact of its own and three every model
/// shares, so any two are two facts apart, within *near*'s bound of half of
/// four.
fn models(tag: &str, n: usize) -> Vec<Vec<String>> {
    (0..n)
        .map(|i| {
            let mut m = vec![format!("(q {tag}{i})")];
            m.extend(["(s 1)", "(s 2)", "(s 3)"].map(str::to_string));
            m.sort();
            m
        })
        .collect()
}

/// **The diff, over the bound.** 1 000 recorded against 1 000 expected, none
/// equal: 1 000 000 candidates at 24 bytes is 23 MiB. Nothing is paired near,
/// every model is missing or spurious, the block says why, and the candidate
/// list was never allocated.
#[test]
fn the_diff_declines_its_near_pairing_past_the_bound() {
    let (got, want) = (models("g", 1_000), models("w", 1_000));
    let (d, peak) = peak_during(|| diff_model_sets(&got, &want));
    eprintln!("{} against {}: peak {peak} bytes", got.len(), want.len());
    let why = d.unpaired.as_deref().expect("declines the near pairing");
    assert!(
        why.contains("1 000 unmatched recorded against 1 000 unmatched expected are 1 000 000 candidate pairs, 23 MiB"),
        "{why}"
    );
    assert!(d.near.is_empty() && d.matched.is_empty());
    assert_eq!((d.missing.len(), d.spurious.len()), (1_000, 1_000));
    assert!(!d.agrees());
    assert!(
        peak < 24 * 1_000 * 1_000 / 10,
        "held {peak} bytes: the candidate list was allocated"
    );
    let lines =
        ein_render::models::diff_lines(&d, "what", &|i| i.to_string(), &|j| j.to_string(), "");
    assert!(
        lines
            .iter()
            .any(|l| l.contains("no model was paired as near")),
        "{lines:#?}"
    );
}

/// **The diff, under the bound, the control.** 500 against 500 is 5.7 MiB of
/// candidates: reserved, paired, and seen by the probe.
#[test]
fn under_the_bound_the_diff_pairs_near() {
    let (got, want) = (models("g", 500), models("w", 500));
    let (d, peak) = peak_during(|| diff_model_sets(&got, &want));
    eprintln!("{} against {}: peak {peak} bytes", got.len(), want.len());
    assert!(d.unpaired.is_none());
    assert_eq!(d.near.len(), 500);
    assert!(
        peak >= 24 * 500 * 500,
        "held {peak} bytes: the probe does not see the list"
    );
}
