# S1e.4.5 — Tests (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 2 days
**Depends on:** [Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) for
`TE-L5`; [S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md) T1 for `TE-L3`
(the same step-list diff, one flag down);
[T1e.1.6.1](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) for `TE-L2`'s
anchor list, which falls out of that grep.
**Findings:** [`TE-L1`](../review/tests/low.md) …
[`TE-L5`](../review/tests/low.md).

## Context

Five findings about the gate's edges. Two are about **tests that can fail for
a reason other than the code** (`TE-L1`), or **couple to files nobody warns
you about** (`TE-L2`); three are about **checks that do not run** — a flag
that skips more than it says (`TE-L3`), a census wired to nothing (`TE-L4`),
and a release matrix that has never executed (`TE-L5`).

`TE-L1` is the only place in the workspace where a test can fail on machine
load rather than behaviour:
[`test_cli.rs:203-225`](../../../ein.rs/crates/ein-cli/tests/test_cli.rs)
requires the features directory to finish in under 20 s, and
[`corpus_cli.rs:335-412`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)
asserts dev-profile wall clock at 4× tolerance. Both are deliberately
generous; on a badly overloaded runner they become flakes, and a flake in this
suite will be read as an engine regression, because everything else here is
deterministic by construction.

`TE-L5` is the one worth being blunt about: four platform legs, a
jobs-cross-diff and a `--no-default-features` leg, **none of which has ever
run**. The workflow's own header says so — *a green badge here is read as: the
first tag passed* — which is honest and is also exactly why a reader would
believe the badge.

## Acceptance

- No test can fail on machine load without saying so in its failure message,
  or the timing assertions are gated behind an env var.
- The world-anchor list exists in one place and both puzzle files point at it.
- `run_tests.sh --tests-only` either runs the bench smoke or its header stops
  claiming it skips only the static checks.
- `stdlib_census.py --check`'s cadence is stated — nightly, or
  milestone-cadence with that written down.
- The release matrix's status is resolved per
  [Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md).

## Tasks

### Task T1e.4.5.1 — `TE-L1`: make load-sensitivity legible

Two changes, and the second matters more than the first:

1. **Label the failures.** Both assertions' messages say *"machine load?"* and
   name the measured and expected values. A flake that announces itself as a
   possible flake costs a reader a minute instead of an hour.
2. **Decide whether they belong in the per-commit tier at all.** The repo has
   a precedent both ways: the wall-clock floors for slow entries are nightly
   ([TE-M3](../p1e.3_medium/s1e.3.6_tests.md)), and the bench smoke is in the
   gate. The cleanest split is to gate the *timing* on an env var (as
   `EIN_CORPUS_SLOW` already gates the slow entries) and keep the
   *completion* — that the features directory finishes at all — in the
   default run, which is the property that actually guards against a
   pathological regression.

Do not simply raise the margins. A tolerance raised until it never fires is a
test that has been deleted without anyone noticing.

### Task T1e.4.5.2 — `TE-L2`: one anchor list

> **The list, measured 2026-08-29 in
> [S1e.1.6](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T1e.1.6.1** — and
> the finding **understates** it. It is not four crates, it is **six**, and 26
> test files:
>
> | crate | test files naming `examples/zebra*` |
> |---|---|
> | `ein-infer` | **12** — `acceptance` · `explain_semantics` · `layer_census` · `naf_semantics` · `obligation_reports` · `obligation_rung` · `obligation_rung_control` · `search_invariants` · `search_semantics` · `stdlib_coverage` · `tree_traversal` · `worker_view` |
> | `ein-cli` | **6** — `acceptance_cli` · `cli_semantics` · `corpus_cli` · `einb_cli` · `embedding` · `leftover_probe` |
> | `ein-render` | **3** — `golden_dot` · `idea08_acceptance` · `presentation_semantics` |
> | `ein-einb` | **2** — `invalidation` · `roundtrip` |
> | `ein-ir` | **2** — `dump_goldens` · `kb_semantics` |
> | `ein-corpus` | **1** — `benches/engine.rs` |
>
> Plus, outside the crates: `docs/api/rust.md` (through `embedding.rs`'s marked
> region), **seven** `utils/` scripts (`e2e_baseline` · `feature_matrix` ·
> `fork_split` · `layer_census` · `profile_ein_rs` · `render_examples.sh` ·
> `zebra2_trace.sh`), `corpus/corpus.toml`, `examples/gen_zebra2_variants.py`
> **and its four generated files** — whose `--check` is in the gate, so an edit
> to `zebra2.ein` that is not regenerated fails there — and four goldens, two
> of which are `from_ein_py/zebra{,2}.golden` and **may never be re-blessed**.
>
> That last row is the one the header comment must carry: an edit to
> `zebra.ein` or `zebra2.ein` that changes the *parse* spends the repo's last
> independent provenance, and no test says so at the moment it happens.

Four crates' tests hard-code facts about `zebra.ein` and `zebra2.ein` —
[`embedding.rs:126-139`](../../../ein.rs/crates/ein-cli/tests/embedding.rs),
[`kb_semantics.rs:1101`](../../../ein.rs/crates/ein-ir/tests/kb_semantics.rs),
[`cli_semantics.rs:156-176, 304-330`](../../../ein.rs/crates/ein-cli/tests/cli_semantics.rs),
[`obligation_reports.rs:263-285`](../../../ein.rs/crates/ein-infer/tests/obligation_reports.rs)
— plus `docs/api/rust.md`. They are deliberate anchor tests and their docs say
so; only `embedding.rs` documents the cost. A reviewer changing `zebra2.ein`
has **no list** of what will fire.

The fix is a list, and the place for it is the puzzle files themselves: a
header comment in `zebra.ein` and `zebra2.ein` naming the anchors, since that
is what the person making the edit is looking at. The list comes free from
[T1e.1.6.1](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s grep — take it
from there rather than re-deriving it.

Worth checking while assembling it: whether any anchor is a *number* that
S1d.2.4's activator facts already moved.
[`docs/api/rust.md`](../../../docs/api/rust.md) documents its own 434 → 444
move; the review notes other tables did **not** get the same audit
([DO-L2](s1e.4.7_documentation.md)).

### Task T1e.4.5.3 — `TE-L3`: `--tests-only` skips more than it says

`run_tests.sh:7` says the flag *skips the static checks*; `:186-189` shows it
also skips the **bench smoke**, which is CI's last step. So a green
`--tests-only` is a strict subset of CI — the precise property the script's
own header warns about, in miniature — and a bench that stops compiling is
invisible to `cargo test`.

Prefer **running the bench smoke under `--tests-only`**: it is a compile plus
one short run, the flag exists to skip the ~5 s of static checks, and the
whole point of the flag is to be a faster gate rather than a weaker one. If
that costs too much, amend the header instead — but then the step-list diff
from [S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md) T1 must compare the
flag's list too, or the divergence just moves.

### Task T1e.4.5.4 — `TE-L4`: state the census's cadence

`utils/stdlib_census.py --check` is wired to no gate and no workflow. That is
**by design** — the in-gate check is scoped to `tests/stdlib/`
([`stdlib_coverage.rs:28-35`](../../../ein.rs/crates/ein-infer/tests/stdlib_coverage.rs))
— but the corpus-wide census, with its numbers and its sole-activator table,
is then re-taken only when someone remembers, which is the failure the
coverage test's own doc comment names for scripts.

Two options: a nightly step (cheap — the census is 37 s over 180 entries), or
an explicit statement in [`utils/README.md`](../../../utils/README.md) that it
is milestone-cadence and why. Prefer the nightly: four of the repo's five
censuses are re-takable instruments and only this one has a `--check` mode
already written, so wiring it costs a YAML block.

Whichever, the sentence goes in `utils/README.md` beside the script's line, so
a reader knows whether the number they are reading is current.

### Task T1e.4.5.5 — `TE-L5`: resolve the release matrix ✅ — decided, and mostly done

**Decided 2026-08-29 in
[S1e.1.6](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md): accept.** The
`publish` job creates a **public GitHub release**, so pushing a tag is a
statement about what this project has shipped and not a check a stage may run
on the maintainer's behalf. What that stage did instead:

- recorded the decision, dated, with the one command that reverses it, in
  [`docs/install.md`](../../../docs/install.md)'s existing caveat block;
- fixed `release.yml`'s pointer, which named *§ What CI has not yet proved* —
  a section of the M1a README that **has never existed**. The content is
  there, under *S1a.9.3 — Packaging and release*; the pointer was not, which
  is a failure a reader of a workflow header cannot detect.

Three places now say the same thing and agree. What is left for this stage is
the *accept* branch below.

The task as written:

[Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) took the decision —
run the matrix once on a pre-release tag, or accept until the first real tag.
Carry it out:

- **If run**: tag, watch the four platform legs, the jobs-cross-diff and the
  `--no-default-features` leg, and record what broke. Something usually does,
  and finding it on a pre-release tag rather than on the first real one is the
  entire value.
- **If accepted**: the sentence goes in
  [`docs/install.md`](../../../docs/install.md), which is the page that offers
  *a release binary* as a channel and is therefore where a reader would
  otherwise assume the matrix works.

## Notes

Two days, and `TE-L5` is the one that can consume all of it if the matrix is
actually run and actually breaks. That is a good use of the time and it is
also a reason to sequence it last in the stage: the other four are bounded,
and a broken Windows build is a finding for a followup, not a reason to hold
the phase.

---

## ✅ Done 2026-09-01 — four findings, and one of the four refuted the stage's own preference

`TE-L5` was **accepted** at S1e.1.6 and needed nothing here. The other four:

| | disposition | the short version |
|---|---|---|
| `TE-L1` | **fixed**, and the finding named the wrong site as dangerous | the 20 s ceiling has a ~1 300× margin; the 4× recorded-cost band trips at 2.9× |
| `TE-L2` | **fixed** — generated, because a hand-written list was stale before the phase closed | 26 → **28** files in the two days since S1e.1.6 measured it |
| `TE-L3` | **fixed by amending the header** — its own recommendation is **refuted**, on a measurement | 5.3 s against 1.6 s |
| `TE-L4` | **fixed** — wired to the nightly that already existed | and `--check` could not fail for the thing most likely to break |

### `TE-L1` — the load-sensitive assertions, and there were three

The finding says the two sites are alike (*"both are deliberately generous"*).
Measured 2026-09-01, they are opposites:

| site | budget | actual | margin |
|---|---:|---:|---:|
| `test_cli.rs` — `ein test examples/features` | 20 s | **15 ms** | ~1 300× |
| `corpus_cli.rs` — recorded `cost_ms`, 4× band | 4× | 2.45×, 3.02×, **2.96×** on the three entries it judges | ~1.3× |
| `einb/roundtrip.rs` — cold `.einb` open (**not in the finding**) | 5.0 ms dev | **0.96 ms** | 5.2× |

So the site the finding leads with **cannot** fail on load, and its comment
priced the margin against `04_open`'s ~10 s *in the sweep* — a different
operation. The tight one is the recorded-cost band, and the finding's claim
that these are *"the only tests in the workspace that can fail on machine
load"* is false: `roundtrip.rs` sits between them and above both in
undiagnosability, since its message named neither the budget nor the profile.

**What changed.** The `cost_ms` band is asserted only under `EIN_CORPUS_SLOW`
and **reported** per commit; the two flag-threshold directions keep their
per-commit assertion, because their headroom is the threshold itself (3.9× on
the worst entry) and they are what notices an entry crossing it. Nightly runs
this file with `EIN_CORPUS_SLOW=1` **and** `--release` — the profile `cost_ms`
was measured on — so gating does not weaken the band, it moves it to where it
is truer. Every remaining wall-clock message names the budget, the profile and
*machine load?*.

**The control**, run and not banked: `cost_ms = 252` on
`branching/07_lookahead_off` (the window is `[250, 253]`, boxed in by the
band's own 250 ms floor below and `slow_matches_the_recorded_cost`'s 1 000 ms
ceiling above) →

- `cargo test -p ein-cli --test corpus_cli` — **green**, and `--nocapture`
  prints `corpus cost drift (1 entry(s)), reported not asserted at this tier`;
- `EIN_CORPUS_SLOW=1 …` — **red**, on that entry, *"outside 4× (machine load?
  this sweep is the `dev` profile)"*.

**And the worst wall-clock hazard in the corpus is not an assertion at all.**
`corpus/README.md` blessed `-T` / `--max-time` in the run vocabulary as the
equal of `-E`, and `no_cell_crashes` allows an exit 2 wherever a run names one
— but `-E` counts enterings and `-T` counts seconds, so a `-T` cell's **exit
code is a stopwatch** and `corpus_exits.txt` banks it line by line (measured:
the same argv exits 2 at `-T 0.001` and 0 at `-T 60`). That is the one door in
the repo through which a clock could reach a *golden*; everywhere else it is
scrubbed first. Closed by a check, not a sentence:
`manifest::no_declared_run_budgets_by_wall_clock`.

One more thing found on the way, in the file S1e.3.6 edited: `manifest.rs`'s
S1a.9.0 paragraph — *"no engine run, no wall clock, no flake"* — had been
pasted **above** the wrong test, so it documented one that compares two string
arrays, and a reader chasing `TE-L1` through the code was sent to the wrong
function. Nothing detects a mis-attached doc comment; `cargo doc -D warnings`
is green over it.

### `TE-L2` — the anchor list, generated

The stage says to put the list *in the puzzle files*. Two measured facts say
not to: it moved **26 → 28** in the two days since
[S1e.1.6](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) measured it — both
additions from this milestone's own commits — and whatever goes into
`zebra2.ein` is copied verbatim into **four** generated files, so a
hand-written table becomes five that go stale together.

So: the registry is
[`examples/README.md` § What an edit to these two files fans out into](../../../examples/README.md),
its file list is a **generated block** that
`world_anchors::the_anchor_list_is_the_greps_own_answer` diffs (`EIN_BLESS=1`
re-banks it), and both puzzles carry a nine-line header pointing at it —
`world_anchors::both_puzzles_name_the_registry`.

Four rings, and the fourth is the sentence the section exists for: **three
`from_ein_py/` goldens, not two** (`zebra.golden`, `zebra2.golden`,
`kb_zebra_unified.dot`), and an edit that changes the *parse* spends the repo's
last independent provenance with nothing failing at the moment it happens.

**A `;`-comment is free everywhere except the generator**, and that exception
is real: `python3 examples/gen_zebra2_variants.py` was re-run and its four
outputs are in this commit, because `--check` runs inside `cargo test` and a
comment on `zebra2.ein` is the one comment in this repo that can redden the
local gate.

### `TE-L3` — the header, and why the bench smoke stays where it is

The stage prefers *running the bench smoke under `--tests-only`*. **Refuted**,
on three grounds:

1. **The rationale does not survive.** The review argues a green `--tests-only`
   is *"a strict subset of CI — the precise property the script's own header
   warns about"*. The header's warning is about the **default** run; the flag
   is a subset by construction, and adding the bench leaves it a subset by six
   steps. The change cannot buy the property it is argued for.
2. **It restores half a pair.** The bench's *type* check is `cargo clippy
   --workspace --all-targets`, which the same flag skips. `--tests-only` drops
   the bench's compile check and its codegen together, which is coherent.
3. **Measured**, 2026-09-01: the bench builds the **release** profile, which
   `cargo test`'s dev profile shares no artifacts with, so the first
   `--tests-only` after an engine edit pays **5.3 s** of codegen against the
   **1.6 s** the flag saves. Three times the whole saving, on the iteration
   loop the flag exists for.

The stage's fallback names one condition — *"the step-list diff from S1e.3.4 T1
must compare the flag's list too"* — and it has since S1e.3.6. What it did
**not** compare is the header, which is the copy that drifts, and it drifted
while that test watched: T1e.3.8.4 corrected the paragraph at `:91` when the
link check joined the gate and left `:5` saying *five*, so the same header said
five and six about the same guard. The same stale *five* was in `AGENTS.md` and
in `gate_steps.rs`'s own assertion message, which also called the bench smoke
*the sixth* of a seven-element array.

`what_tests_only_skips_is_what_the_script_guards` now holds the header to the
set it already parses — both assertions derived, so this file cannot restate
the count wrongly either. Controls run by hand: reverting `:5` fails *"the
header does not say `six static check…` anywhere"*; reverting `:7` fails *"the
header's `--tests-only` line reads … — the flag skips the bench smoke too"*.

### `TE-L4` — the census, and a nightly that already existed

Nothing in `.github/workflows/`, `run_tests.sh` or `build.sh` named
`stdlib_census`. Three corrections to the task before it was carried out:

- **A nightly already exists**, and its `deep-corpus` job already leaves
  `target/release/ein` on disk — which is the script's own default binary. Two
  `run:` steps, not a job.
- **The task's reason for preferring the nightly is false.** *"Only this one
  has a `--check` mode already written"* — `utils/stdlib_mutants.py` has had
  one since S1e.3.6, **this milestone, one phase earlier**, with a stage
  document that asked for exactly this cadence and never got it. Both are
  wired, and the real reason only these two are is cost: 38 s and 7 s.
- **`--check` could not fail for the thing most likely to break it.**
  `census["failures"]` — a declared run that exits 0 and narrates nothing — was
  printed under its own heading and never returned, so a *partial* sweep whose
  survivors still covered all 77 rules was green. Fixed in the same commit;
  empty today, so it moves no result.

Re-taken 2026-09-01: **77 of 77, zero set empty, 217 entries, 38 s** — where
three shipped sites said *180 entries* and *37 s*, and one said *73 of 73*
without its date. The cadence is stated in `utils/README.md`'s third column,
beside the numbers.

**And the warrant for two exact counts pointed at nothing.**
`stdlib/README.md` and `tests/README.md` both cite
`every_stdlib_rule_is_activated_by_a_program_here` as the test that lets them
state a number exactly. There is no such function — it is
`every_stdlib_rule_is_activated_by_a_program`. Under
[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
that is worse than a stale count: the citation is what stops anybody checking.
Both fixed; `README.md`'s and `stdlib/README.md`'s *56 programs* became the
command that counts them (it is 57).

**Nothing reads `nightly.yml`** — `gate_steps.rs` diffs the per-commit workflow
only — so the two steps are unpinned, and that is **accepted with the reason at
the site** rather than unnoticed: teaching that test a second marker convention
is more than a Low finding is worth.

**Gate:** `./run_tests.sh` — **exit 0**, six static checks, **813 tests**, the
bench smoke. No golden moved.
