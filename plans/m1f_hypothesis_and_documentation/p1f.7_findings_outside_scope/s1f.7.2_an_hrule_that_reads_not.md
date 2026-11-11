# S1f.7.2 — An hrule that reads `not`: one answer from both builds

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 13. **Done 2026-11-11**: W85, three rows filed and fixed in one
commit; the assertion is gone, and where an hrule reads `not` every hrule is matched before any candidate is emitted
([the records](#t1-record-2026-11-11)).
**Depends on:** nothing.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **An hrule whose `:match` reads `not` stops a debug build.** `(hrule h () :match (and (a ?x) (absent (not (p
> ?x)))) …)`, written, no macro: `hrule.rs`'s `debug_assert!` that no hrule reads `not` (*an hrule reads `not`,
> which hypgen's kill cache writes mid-enumeration*) fires, exit 101, on every solve question; the release build
> answers, two states. Nothing refuses such an hrule at load, so the two builds disagree.

The assertion is in `Hrules::candidates` ([`ein-infer/src/hrule.rs`](../../../ein.rs/crates/ein-infer/src/hrule.rs)).
Its comment states the premise: the hypothesis generator's lookahead kill cache writes `(not h)` between one
candidate and the next, so an hrule that read `not` would condition on that cache.

## What T1 has to settle first

**Whether the premise still holds.** The comment speaks of the lookahead filter. The walk went at M1e2
S1e2.1c.5, and S1e2.3.1 made the universe config-free. If nothing writes `not` during enumeration any more, the
assertion guards a case that cannot happen. Then an hrule that reads `not` is an ordinary hrule, the release answer
stands, and the fix is to delete the assertion and its comment, with a test that holds the answer to `ein oracle`.

**If something still writes `not` mid-enumeration**, the release answer may depend on the order of the candidates.
Then there are two directions. A refusal at load (one line, every command) agrees with how S1f.5.21 and S1f.5.24
treated a shape the engine cannot answer. Giving the hrule a meaning that does not see the cache is the other
direction, and it is a question for the user.

Either way the two builds give one answer, and it agrees with the oracle.

## Tasks

### T1f.7.2.1 — confirmation, and the premise

Re-take the finding on both builds. Find every writer of a `not` fact between two candidates of one enumeration, or
show there is none. Compare the release answer with `ein oracle` under a permuted candidate order
(`id_order_invariance`'s lever). Census the hrules in the corpus and the stdlib that read `not` (written, or through
`absent`, `unknown` or a macro).

### T1f.7.2.2 — the rows

A row for the debug build's exit 101, where a build's stop on a valid program is a wrong answer. A row for the
release answer if it is wrong or depends on order. Each is filed `open` and held to today's engine.

### T1f.7.2.3 — the fix

Either the stale assertion goes (and `reads_negation` with it, if nothing else reads it), or the load-time refusal
is added and the assertion stays as the invariant that the refusal holds. A test runs the witness on both builds.

### T1f.7.2.4 — the pages

The hrule section of the kernel pages that describe hypothesis generation, `defined_behaviour.md` if a refusal is
added, and the stage's records.

## Acceptance

- The debug and release builds give one answer on every hrule that reads `not`, and it is the oracle's or a refusal.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green, and the debug-build test suite runs the witness.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.2 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release and debug binaries at `55aadd0b`. Three shapes, each an hrule over two guests of whom
Bob is known not to have paid, `(not (paid Bob))`:

| the hrule's `:match` | release | `ein oracle` | debug |
|---|---|---|---|
| `(and (is-a ?g Guest) (absent (not (paid ?g))))`, proposing `(paid ?g)` | H `(paid Ann)`, R two states | the same | exit 101 |
| `(and (is-a ?g Guest) (not (paid ?g)))`, proposing `(owes ?g)` | H `(owes Bob)`, R two states | the same | exit 101 |
| `(and (is-a ?g Guest) (unknown (paid ?g)))`, `std.macro`'s, proposing `(paid ?g)` | H `(paid Ann)`, R two states | the same | exit 101 |

The debug build stops on every command that takes H: `ein solve` and each of its questions, `ein oracle`, `ein
test` and `ein render lattice`, with *an hrule reads \`not\`, which hypgen's kill cache writes mid-enumeration —
see \`Hrules::candidates\`* (`hrule.rs:130`). `ein saturate`, `ein kb save` and the other `render` views take no H
and answer.

**The premise is stale.** The one writer of a `not` fact between two candidates is `hypgen::emit`'s kill cache,
behind `GenOptions::kill_cache`. Every command takes H through `hypgen::generate_config_free` (`region.rs`) or
`generate_blind` (`ein-cli`'s `summary.rs`, the `leftover` probe), which pass `GenOptions::CONFIG_FREE`, the cache
off, since M1e2 S1e2.3.1. The config-reading `hypgen::generate`, which can have it on, is called by the shape
digest's `hyp` op (`shape.rs`) and by one test (`ein-infer/tests/explain_semantics.rs`), and by no command
(`grep -rn 'hypgen::generate\b\|generate_config_free\|generate_blind' ein.rs/crates`, 2026-11-11). So the release
answer does not depend on the order of the candidates, and it is the oracle's.

**The census.** Of the 127 hrules in the tracked `.ein` files outside `plans/`, one reads `not`:
`corpus/fuzz_findings/hrule-reads-not.ein`, the fuzzer's finding of 2026-08-21 that this assertion was first met on,
kept there as an open question. No stdlib hrule exists, and no corpus entry's hrule reads `not`, `unknown` or
`forall` (a scan of each `(hrule …)` up to its `:assert`, 2026-11-11).

## T2 record, 2026-11-11

Three witnesses under `examples/ein-bugs/wrong-answers/`, each with a corpus entry whose runs are `solve`,
`saturate` and `test`, and each carrying an `:expect` that states its two classes: `hrule-reads-absent-not`,
`hrule-reads-not` and `hrule-reads-unknown`, rows of W85.

**The rows were not held `open`.** `wrong_answers.rs` runs under `cargo test`, a build with debug assertions, where
the unchanged engine's run of each ends by a panic, exit 101, whose first line carries a thread id, so no recorded
line can hold it; and `register_region` takes each witness's region in-process and panics with it. The register's
`debug_panic` field, W7's and W16's precedent, is read by no test since M1e2 S1e2.2.16. So the three rows are filed
`fixed` in the fixing commit (Q-M1f.129): their `wrong` records the release answer, which is right, and the debug
build's stop as `debug_panic`. What was held on the unchanged engine instead: with the three rows filed `fixed`,
`wrong_answers` on the sources at `55aadd0b` fails on exactly those three, *exit: recorded 0, got 101*, and
`register_region` panics at `hrule.rs:130` (2026-11-11).

Goldens named before blessing: `corpus_exits.txt` (three lines a witness: `solve`, `saturate`, `test`) and
`corpus_shapes.md5` (the three witnesses' lines).

## T3 record, 2026-11-11

`ein_infer::hrule`'s `Hrules::candidates` holds no assertion. Where an hrule's `:match` reads `not`
(`reads_negation`: a relation premise, under `(absent …)` too, so `unknown`'s expansion), every hrule is matched
before any candidate is emitted, so nothing the pipeline writes between two candidates is visible to the
enumeration, with the kill cache on, as the `hyp` op and the test can have it, and off. Where none reads `not` the
two orders agree, and each hrule is matched as it is reached, as before (Q-M1f.129 (a)).

W85's three rows are `fixed`. The pin is `ein-cli/tests/hrule_reads_not.rs`, which `cargo test` runs with debug
assertions: the three witnesses on thirteen commands, each exit 0, `ein solve` and `ein oracle` both two states
holding the witness's one candidate; the fuzzer's program; and two hrules of which the second reads the negation the
first's candidate is about, four states from both. On the debug binary at `55aadd0b` each of those solve, oracle,
test and lattice runs exits 101 (2026-11-11).
`corpus_cli`'s cells run each witness's `solve`, `saturate` and `test` in the same build, and the shape digest
takes its `hyp` op with the config's kill cache on.

`corpus/fuzz_findings/hrule-reads-not.md` and that directory's README say the finding is settled. The sweep of
S1f.7.4's T3 record, 641 tracked files on the release binaries, moved nothing.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.23 (new, with S1f.7.4), `02_patterns.md`, `examples/README.md`,
`corpus/fuzz_findings/`'s two pages, `AGENTS.md`, the phase README and this file. No refusal was added, so
`00_ebnf.md` is unmoved.
