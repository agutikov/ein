# P1f.10 — what the phase found outside its stages

**Phase:** [P1f.10](README.md). Opened 2026-11-15.

What a stage of this phase finds outside its own scope: kept in the phase and shown to the user at its stop,
not fixed by the stage that found it. A wrong answer among them is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml) before anything fixes it; none below is one.

| id | what | witness, and the command | found by | today | state |
|---|---|---|---|---|---|
| K-1 | **What *a program searches* means is written twice.** `corpus_ops::searches` is in `ein-render`'s test tree, which no other crate can read, so the pair layer's census, which T1f.10.1.3 scopes by it, restates its budget, its cube limit and its call in the example. Nothing fails when one copy moves | `grep -nE 'SWEEP_(BUDGET\|LIMIT)' ein.rs/crates/ein-render/tests/corpus_ops/mod.rs ein.rs/crates/ein-infer/examples/pair_layer.rs` | [S1f.10.1](s1f.10.1_exclusion_census.md), 2026-11-15 | the two agree: 188 of the 625 programs the census reads search | open. A home both crates can read is `ein-corpus`, which does not depend on `ein-infer` today |
| K-2 | **The slow half of `ein-infer/tests/pair_layer.rs` is in no tier.** Per commit the test reads every corpus program but the `slow` entries, and three instances named in it whatever their flag (`zebra.ein`, `zebra2.ein`, `lattice/02_genuine_3set_death.ein`). Under `EIN_CORPUS_SLOW=1` it reads the rest, and two more programs reach the comparison, `zebra2-obligations.ein` and `lattice/07_two_pairs_two_tables.ein`; the other slow entries' describes do not exhaust, so nothing is compared on them | `EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test pair_layer -- --nocapture`: 609 programs compared in 19.4 s, against 607 in 4.1 s without it (2026-11-15, 16 workers, on AC) | S1f.10.1, 2026-11-15 | run by hand | open, the user's: a nightly step (`nightly.yml`, `run_tests.sh`, `nightly_steps.rs`), the two programs named per commit beside the three, or the slow half dropped. The default taken is none of the three: by hand |
| K-3 | **`utils/README.md` counts its measurement set wrongly.** § The M1a measurement set opens *Eleven scripts*; its table had nine rows before S1f.10.1 and has ten with `pair_layer` | `sed -n '/^## The M1a measurement set/,/^Results/p' utils/README.md \| grep -c '^\| \['` | S1f.10.1, 2026-11-15 | the sentence stands | open. P1f.20 sweeps the page |
