//! **The diff prices its near pairing before it allocates it.**
//!
//! A pair table over many models is gigabytes — `pets.ein`'s 11 339 states
//! against a file of them each one fact off — so `--diff-models`' near
//! pairing compares the cost with [`PAIR_TABLE_BOUND`] before it builds
//! anything, and past it pairs nothing and says why.
//!
//! *Allocates nothing* is measured here rather than asserted. A counting
//! allocator keeps the bytes live on the calling thread and their peak, and
//! each case reads the peak across one call:
//!
//! - **over the bound**, the peak is below a tenth of the candidate list, so
//!   it was not built;
//! - **under the bound**, the control: at least the list, so the probe sees
//!   it when it is built.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use ein_render::models::{PAIR_TABLE_BOUND, diff_model_sets};

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
    assert_eq!(PAIR_TABLE_BOUND, 16 << 20);
    const { assert!(24 * 1_000 * 1_000 > PAIR_TABLE_BOUND) };
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
    const { assert!(24 * 500 * 500 <= PAIR_TABLE_BOUND) };
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
