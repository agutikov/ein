# Ein — configuration reference

> **Normative, and new (2026-08-28).** Read at `4a47aa3`, M1e
> [S1e.5.1](../history/m1e_review_processing/README.md#s1e51--the-configuration-reference).
> It supersedes no page: before it the seventeen `(config …)` flags were stated
> only in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs)'s `FIELDS`
> and the nine field doc comments that had one, the CLI options only in
> `--help`, and the `EIN_*` environment nowhere at all. **Since M1e2
> S1e2.1a.5's switch (2026-10-18) a program sets eight of them**, nine retired
> with the walk, and `ein solve` has 33 options where it had 42. **Since
> S1e2.1a.6 (2026-11-01) it sets seven**: `enable-symmetric-mirror`, the one
> flag that changed R, retired too. **Since S1e2.1b.5 (2026-11-03)** `ein
> solve` has 34: `--monotone`, the explicit check the user approved, asks
> about the program instead of R. **Since T1e2.1c.4.6 (the same day)** it
> has 35: `--count`, which the user ruled, asks how many states and
> commitments R holds, and reads no state. **Since S1e2.1c.5 (the same day)**
> a program sets three, and `ein solve` has 33: the user retired `-L`, `-K`
> and the four `enable-*` fields no route read, and gave `-y` its meaning,
> the detector's parent-path spot check, unbounded, on every program.
> **Since S1e2.3.1 (the same day)** the binary reads two `EIN_*` names:
> `EIN_OBLIGATION_CHOICE` retired once `--json-summary`'s `root` block, its
> last reader, became root's fixpoint and H as the routes have them (§ 4).
> **Since S1e2.2.5 (2026-11-04)** a program sets two: the user retired
> `warn-derived-naf`, whose refutation half became *forbid*'s refusal.
>
> **It is a reference, not a narrative.** One row per knob. What a lever *does
> to the search*, and what it was measured at, is
> [`inference/features.md`](inference/features.md); what a **solution** and a
> **model** are is
> [`inference/solution_semantics.md`](inference/solution_semantics.md); the
> newcomer path is [`docs/guide/`](../guide/README.md). A reference that
> explains twice will disagree with itself.
>
> **The config principle**, the user's ruling of 2026-10-15: *no config option
> affects the problem's search lattice; config may only affect search speed.*
> The answer is the region R of
> [`solution_semantics.md` § 2](inference/solution_semantics.md#2-the-answer--the-region-r),
> and it is the same under every knob on this page. A **budget** may stop a
> run early, and the read-out then says the answer is partial. A knob
> that changes R, or changes a reported answer that does not say it is
> partial, is a wrong answer and not a mode. § [2.5](#25-the-config-principle-read-against-every-row)
> reads every lever against it, measured.
>
> **Nothing here is hand-maintained.** The defaults block is the binary's own
> `--dump-config` output, the flag table is diffed against `FLAGS`, the option
> counts are read off the help-shape golden, and the environment
> classification is scanned out of the shipping crates —
> [`ein-cli/tests/config_reference.rs`](../../ein.rs/crates/ein-cli/tests/config_reference.rs),
> seven tests, in `cargo test --workspace`. § [6](#6-what-pins-this-page).

---

## 1. Three surfaces, and the order between them

| surface | how many | who owns the list |
|---|---:|---|
| `(config …)` flags | **2** | `FLAGS` in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs), and `RETIRED` for the fifteen a program can no longer set; the form's shape is [`01_grammar.md` § Top-level forms](ir/03-ein-lang/01_grammar.md#2-top-level-forms) |
| CLI options | **71** | `--help`, and [`golden/help_shape.txt`](../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt), which `help_surface.rs` diffs |
| `EIN_*` environment | **2** read by the shipped binary, of 26 greppable names on 2026-08-28 | § [4](#4-the-environment) |

**Precedence**, highest first:

1. **A CLI option** — but only the one that shadows a flag, § [3.2](#32-the-option-that-shadows-a-flag).
2. **The program's own `(config …)` head.** Several blocks are allowed and the
   loader keeps the **last one whole** — not a merge:
   `(config :record-alternative-justifications false) (config :lattice-sanity-check true)`
   leaves `record-alternative-justifications` at its default
   ([`from_ir.rs`](../../ein.rs/crates/ein-ir/src/from_ir.rs), `config_blocks.last()`;
   the last-wins rule is stated in [`01_grammar.md`](ir/03-ein-lang/01_grammar.md)).
3. **`SolverConfig::default()`** — the block in § [2.1](#21-the-defaults).

`ein solve --dump-config` prints the **resolved** result of the three, and is
the only way to see what a run actually used.

Two things are deliberately *not* in that order. There is **no user-level
config file** and no `$EIN_CONFIG`. And **no environment variable reaches
`SolverConfig`**: the `EIN_*` levers that steered the walk sat beside it until
each retired (§ 4),
because `SolverConfig` is rendered into the KB-shape digest and a knob whose
settings are being *compared* would re-bless every shape golden in the corpus
(§ [5](#5-what-is-deliberately-not-configuration)).

---

## 2. `(config …)` — the two solver flags

```lisp
(config :record-alternative-justifications false :lattice-sanity-check true)
```

Names are `:kebab-case`, and since M1e2 S1e2.1a.5 every flag a program can set
is a bool: `true` / `false`, bare or quoted. An unknown flag, a wrong-typed
value and a **retired** flag are all **load errors**, exit 1. The unknown-flag
message lists the valid names *sorted*, where `--dump-config` prints them in
declaration order, and the retired-flag message says why the flag went
([`defined_behaviour.md` § Errors and exit codes](defined_behaviour.md#4-errors-and-exit-codes)):

```text
kb load error: (config …): unknown config flag :nonsense (expected one of: …)
kb load error: (config …): config flag :lattice-sanity-check expects true/false, got Int(value=7)
kb load error: (config …): config flag :lattice-order was retired at M1e2 S1e2.1a.5: an order lever, and every route's order is part of its definition
kb load error: (config …): config flag :enable-symmetric-mirror was retired at M1e2 S1e2.1a.6: off, it changed the answer, and no config option may. The mirror is what `(__symmetric__ R)` means, so it is always on: a relation that is not to be mirrored is not marked
kb load error: (config …): config flag :enable-path-nogoods was retired at M1e2 S1e2.1c.5: the walk's learned clauses: every route reads N, the minimal dead sets, by construction
kb load error: (config …): config flag :warn-derived-naf was retired at M1e2 S1e2.2.5: its refutation half is forbid's refusal: a refutation resting on an (absent …) over a relation a commitment can still extend refuses the program, whatever the config says; its stratification half was the walk's advice, and every non-monotone site is the detector's warning on every surface that reports R
```

### 2.1 The defaults

`ein solve --dump-config` on a program with no `(config …)` head:

<!-- generated: ein solve --dump-config -->
```text
config (resolved)
  lattice-sanity-check             false
  record-alternative-justifications true
```
<!-- /generated -->

### 2.2 The flags

| flag | type | default | what it changes | answer? | R? | stability |
|---|---|---|---|---|---|---|
| `lattice-sanity-check` | `bool` | `false` | `-y`'s: the detector's parent-path spot check, **unbounded and on every program** — every one-step parent of every printed state of two commitments or more, built and saturated again (the user's ruling of 2026-11-03, M1e2 S1e2.1c.5; the walk's re-saturation of every parent path until then). On a program the static check flags it fills the warning's `paths` row and never fails the run; on a monotone one a path that reaches another state contradicts the § 5.1 theorem, and the run is an engine error, exit 2. A clean check prints its `paths` row; its saturations are that row's, outside `work`. `-y` sets it `true` | no | no | check |
| `record-alternative-justifications` | `bool` | `true` | whether a re-derivation of a known fact is recorded as an alternative justification or dropped: the proof is an AND/OR graph, or a tree. It may change which core a dead node's refutation names — the read-out's `unsat core` and `source` rows, the summary's `region.core`, the trace's `N` and `nogoods.json` — and with it whether that core's search was cut (`core_status`, the `core search` row; M1e2 S1e2.2.23) and, where the core moves, the rules its derivation ends in (`refuted by`, `region.refuted_by`; M1e2 S1e2.3.10), **and nothing else**: R, every state and the verdict words stay (the user's ruling of 2026-11-04, Q-M1e2.11 (ii); § 2.5). Off, the core is the smallest over each fact's first derivation; on, over every derivation the program records. `examples/zebra.ein` turns it off for speed, 0.95 s against 188 s, with the same core on each of N's 458 members (`ein solve`, 2026-11-04) | explanation only | no | contract |

### 2.3 Reading the judgement columns

**Two questions about the answer.** The answer is the region R that
[`solution_semantics.md` § 2](inference/solution_semantics.md#2-the-answer--the-region-r)
defines, and since the switch every `ein solve` run reports it, or a question
about it.

- **`R?`** asks whether the flag changes R: the candidates, any node's status
  or state class, root's fixpoint, or the facts of any state in R. It is
  measured, over every program the oracle reaches, by
  [`utils/region_census.py --all --levers`](../../utils/region_census.py),
  § [2.5](#25-the-config-principle-read-against-every-row). Under the config
  principle every cell has to read **no**.
- **`answer?`** asks whether the flag changes what a run **prints**: the
  read-out's rows, its states and their models, its queries' classes and its
  claims' outcomes.

| value | means |
|---|---|
| **yes** | can change R's description, a state or a class |
| **explanation only** | the same R and states; a different unsat core or proof shape |
| **no** | the same read-out; counters and the wall clock may move |

**No flag is unread since M1e2 S1e2.1c.5.** Four were, read by no route since
the switch, and the user retired them on 2026-11-03 (§ 2.4); the fifth,
`lattice-sanity-check`, got its meaning on a route the same day.
`warn-derived-naf` retired at S1e2.2.5.

**Stability** — what depending on a flag commits you to.

| value | means |
|---|---|
| **contract** | stated here, exercised by the corpus or a named test, safe to depend on |
| **check** | asks for more work and moves no answer: its own row, and on a failure an error, never a different answer. The corpus's `levers` cells run `-y` on every `cargo test`, `lever_invariance.rs` holds every question's read-out and summary unmoved under it but for its row (C4, § 2.5), and `config_reference.rs::the_sanity_flag_adds_its_row_and_nothing_else` holds the flag from outside |

### 2.4 The fifteen retired flags

M1e2 S1e2.1a.5's switch retired nine, S1e2.1a.6 a tenth, S1e2.1c.5 four more,
and S1e2.2.5 a fifteenth, each refused at load with the stage and the reason below
(`ein_core::config::RETIRED`). Their fields stay on `SolverConfig` at their
defaults — the lookahead and the kill cache read there only by the walk's
generator, `hypgen::generate`, which no command runs since M1e2 S1e2.3.1 and
which `shape.rs`'s `hyp_shape` still runs for the corpus's `hyp` digest; the
rest for nothing — and the `.einb` container
round-trips all seventeen, so its format did not move. A container stores its program as
text and re-loads it on open, so one saved before a retirement whose program
sets the flag is refused, as the `.ein` is.

| flag | why it went |
|---|---|
| `warn-derived-naf` (S1e2.2.5) | **two questions, one flag**, both `warn` lines after root's saturation. `RefutationUnderAbsentWarning` (M1e [S1e.2.3](../history/m1e_review_processing/README.md#s1e23--q-m1e9s-containment-a-diagnostic-for-a-refutation-resting-on-an-absent)): a rule concluding `(false)` or `(not …)` from a guard over a relation a commitment can still extend, which is *forbid*'s refusal since this stage, a load error whatever the config says ([`defined_behaviour.md` § 4.7](defined_behaviour.md#47-a-refutation-resting-on-an-absent-a-commitment-can-fill--m1e2-s1e225)). `DerivedNafWarning`: the guard watches a rule-derived relation, a stratification signal since S1.21.8, whose advice was the walk's; every non-monotone site is the detector's warning on every surface since S1e2.1b.5, and the library's `derived_naf_warnings` stays, read by the corpus's `naf` shape op. The user retired it on 2026-11-04, and the `warn` event kind left the schema with it |
| `enable-pre-branch-lookahead` (S1e2.1c.5) | the walk's one-step pre-filter, which no route runs: every route saturates each candidate's fork. Its one meaning on a route, a pre-filter on a monotone program, would save at most 194 of the 163 702 saturations describe costs over the corpus's 167 `solve` entries (an upper bound, 2026-11-03, S1e2.1c.5 § question 2), and would have to carry the dead singleton's owing bit. The user retired it on 2026-11-03, reversing `solve_interface.md` § 2.4's *kept, speed only* |
| `enable-lookahead-kill-cache` (S1e2.1c.5) | it wrote the walk's lookahead kills into root, which no route does: L1 prunes a dead extension's supersets and writes nothing |
| `enable-path-nogoods` (S1e2.1c.5) | the walk's learned clauses: every route reads N, the minimal dead sets, by construction |
| `enable-fail-fast-fork` (S1e2.1c.5) | a node is never stopped at its first contradiction: a dead node's debts and core are its fixpoint's, which E, the detector's stale check and N read |
| `enable-symmetric-mirror` (S1e2.1a.6) | off, it took the mirror's facts out of root's fixpoint and so changed R, the one flag that did (§ 2.5). The mirror is what `(__symmetric__ R)` means, and the user ruled on 2026-11-01 that it stays always on: a relation that is not to be mirrored is not marked. The register's W13 |
| `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight`, `lattice-order`, `lattice-order-seed` | order levers: every route's order is part of its definition, so `-n N` prints the same states under every lever (C4) |
| `enable-singleton-writeback`, `enable-forced-positive` | they wrote into root, which no route does. What they knew survives as licence L1: a candidate dead alone leaves the universe |
| `print-alive`, `candidate-order-seed` | inert: read by no code path ([Q-M1e.10](../history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)) |

Six corpus files set one, and lost the line at the switch: `examples/zebra.ein`
(`lattice-order`, `enable-forced-positive`), `examples/syntax/config.ein`, and
three register witnesses whose program is now their sibling's (`forced-2-off`,
`f07-fpoff`, `naf-nowb`). At S1e2.1c.5 twenty lost a line or a word: the
seventeen that set the lookahead (the zebra2 family, regenerated by
`examples/gen_zebra2_variants.py`, `branching/06`, `07` and `13`–`16`, the
`alive-set-fresh-name` pair and `fresh-name-nospawn` among them),
`branching/10` and `11` (the kill cache), `examples/zebra.ein` (three of the
four), and `broken/load/config_bad_value.ein`, whose ill-typed value rode
`warn-derived-naf` until S1e2.2.5 and `lattice-sanity-check` since. The pairs
`06`/`07`, `10`/`11` and `15`/`16` now differ in their comments alone. At
S1e2.2.5 four lost the `warn-derived-naf` line: `naf-upward-closure.ein`,
`naf-nowb.ein` and `stale-naf-fork-warn.ein`, refused by *forbid* either way,
and `syntax/config.ein`, which sets `lattice-sanity-check` since.

**What the answer column was read from.** A sweep of 2026-10-18: each of the
eight settable then flipped in an appended `(config …)` block (last-wins, so programs with a
head of their own are skipped), over the **173** corpus programs that load and
state none, `ein solve -p --region-budget 65536 --region-limit 12` compared
byte for byte — stdout, stderr and the exit code — against the same program
unflipped. **One flag moved one program**: `enable-symmetric-mirror false` on
[`features/06_symmetric_native.ein`](../../examples/features/06_symmetric_native.ein),
which marks three relations `(__symmetric__ R)` and declares no rule closing
them. With the flag off it derives **0 facts instead of 3** and the query goal
`(knows Bob Ann)` is no longer reached. `record-alternative-justifications`
moved nothing, because no program there has a dead node with two minimal
cores; what it buys and costs is measured on
[`examples/zebra.ein`](../../examples/zebra.ein), whose own `(config …)` turns
it off and says why. Since M1e2 S1e2.2.11 (2026-11-04) it moves one program's
core, `alt-cap-core.ein`'s, and nothing else: § 2.5's one exemption.

### 2.5 The config principle, read against every row

The user's ruling of 2026-10-15, made during the review of W1: *"no config
option affects problem search lattice, config may only affect search speed"*.
It is stronger than M1e2's acceptance line, which forbade a lever to change a
*certified* answer only. **A budget** (`--region-budget`, `-T`, `-n`, § 3.3)
does not change R either: it stops a run, and the read-out then says the
answer is partial.

**R, measured.** [`utils/region_census.py --all --levers`](../../utils/region_census.py),
2026-10-16 at `9248236d`: every program the oracle reaches (159, n ≤ 16), run
once at default settings and once under each of 24 levers, the seventeen flags
of the time among them. Each run's R is compared with the default's: the
candidates, every node's status and state class, root's fixpoint, and the
facts of every state in R
([`region_census.md`](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_census.md)).

- **No lever changes R on 158 of the 159**, and 23 of the 24 change it on
  none. That is by construction for most of them: the oracle's universe is
  config-free and its nodes are built by saturation alone
  ([`region_dump.md`](inference/region_dump.md#the-cube)).
- **`enable-symmetric-mirror` changed R**, on `features/06`. The mirror is
  how `(__symmetric__ R)` means anything, so the flag switched off part of the
  program's theory. It was the one flag that **violated the principle**. The
  user ruled on 2026-11-01 that it retires, and
  [S1e2.1a.6](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)
  retired it (§ 2.4): the mirror is always on, and the register's W13 records
  the case, fixed in the commit that filed it.
- **What a run prints, held.** Since M1e2
  [S1e2.1a.6](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)
  [`ein-cli/tests/lever_invariance.rs`](../../ein.rs/crates/ein-cli/tests/lever_invariance.rs)
  is the principle as a test, in `cargo test --workspace`. It runs every
  flag a `(config …)` can set, flipped from the program's own value, on every
  corpus program and register witness whose R is known (a bare describe
  exhausts), under each of `ein solve`'s fifteen questions, and compares the
  read-out uncut, its stderr lines, its exit code and the summary with the
  program's own. It runs every `levers` cell the corpus declares through the
  binary, and ran the `EIN_*` levers on the same files until the last of them
  retired (§ 4). On 2026-11-01:
  180 programs and 18 816 cells in-process, 348 declared cells and 432
  environment cells through the binary, 14 s on this machine's eight workers.
  One flag moved: `enable-symmetric-mirror`, as above. With it retired the
  in-process sweep is 16 464 cells, and nothing moves.
- **One exemption, the core under `record-alternative-justifications`**
  (the user's ruling of 2026-11-04 on
  [Q-M1e2.11](../../plans/m1e2_correctness/open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it)
  (ii), M1e2 S1e2.2.11). Off, the flag drops every re-derivation, the
  proof is a tree, and the explanation search is offered each fact's first
  derivation alone, so the core it names can be another, and whether its
  search was cut with it (M1e2 S1e2.2.23: off,
  `examples/ein-bugs/wrong-answers/core-search-cut.ein`'s `(A k)` has one
  environment and its search is not cut; on, 256, and it is). That is all it
  may change: R, every state and class and the verdict words do not move,
  and `refuted by` moves only where the core does (M1e2 S1e2.3.10, the
  user's ruling of 2026-11-06 on Q-M1e2.21 (a): it names the first
  derivation the core supports, and off, where the core is the same, the
  primaries are that derivation). It moved nothing printed
  on any corpus program until W10's fix, after which
  [`alt-cap-core.ein`](../../examples/ein-bugs/alt-cap-core.ein) reports
  `(w1 X) (w2 X)` refuted by `wide` with it on and `(p1 Y01) (p2 Y01) (p3
  Y01)` refuted by `narrow` with it off;
  `examples/zebra.ein` sets it off for speed (0.95 s against 188.1 s on
  2026-11-04, the same core on each of N's 458 members), and the core
  promise is over the derivations the program records
  ([`glossary.md` § Smallest contradiction frontier](glossary.md#smallest-contradiction-frontier)).
  `lever_invariance.rs` compares the flag's cells less the core rows and
  `region.core`, and, where those moved, the `refuted by` row and
  `region.refuted_by`; `the_core_exemption_is_one_flag_and_the_core_rows`
  holds the exemption that narrow: one flag, the `unsat core`, `source`
  and, since S1e2.2.23, `core search` rows and the summary's core and its
  status, and since S1e2.3.10 the rules beside a core that moved, and
  nothing else, on the one program that shows it, which it fails
  without.
- **`lattice-sanity-check` aborted one program under the walk**:
  [`corpus/fuzz_findings/d3-unsat-core.ein`](../../corpus/fuzz_findings/d3-unsat-core.ein),
  which the static check flags non-monotone, failed the probe with
  *"Saturation commutativity violated"*. Four of the eight nodes it found are
  its own direct path stopping at the first contradiction
  (`enable-fail-fast-fork`), and the other four a stale fact a parent path
  keeps ([`node_census.md` § 11.2](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#112-the-flagged-cubes),
  2026-11-02). No route runs the probe since the switch. Its question ships
  as the detector's parent-path spot check (M1e2 S1e2.1b.5, the read-out's
  `paths` row), on a flagged program at most three one-step parents of each
  printed state, built by the node function rather than by the walk's
  fail-fast direct path; it answers nothing and moves nothing. **Since
  S1e2.1c.5 (2026-11-03) `-y` is that check unbounded and on every program**
  (§ 2.2): every parent, and on a monotone program a mismatch is an engine
  error. On `d3-unsat-core` it reads 35 parent paths and finds 4 that reach
  another state, where the bounded check reads 34 and finds 3 (`ein solve
  corpus/fuzz_findings/d3-unsat-core.ein -y`, 2026-11-03; refused since M1e2
  S1e2.2.5, *forbid* with F2's *via*, so the numbers are the record). Which state a node of a program the static check flags is remains
  open, [F21](../../plans/followups/f21_non_monotone_programs_decided.md)'s D2,
  with [P1e2.1b](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/README.md)'s
  S1e2.1b.2 and S1e2.1b.7 as its evidence.

**What the levers did to the walk's report** — the register rows they reached,
`-L`'s W1, W2 and W4 rows, `-K`'s W9 rows, the writeback's W3 row — were the
walk's, and flipped to `fixed` at the switch where R answers them
([`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml)). Under a region
run every flag reads **no**, and so do `--jobs` (§ 3.3) and every
presentation and artefact option: they change speed, or what is printed beside
the answer, never the answer.

---

## 3. The CLI

### 3.1 Where the option list lives

**Not here.** `--help` is the surface and
[`golden/help_shape.txt`](../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt)
is its checked-in rendering — every option's long name, short key, metavar,
arity, default, choices, group and help string, diffed by
`ein-cli/tests/help_surface.rs`, and normative under
[`defined_behaviour.md` § The CLI surface](defined_behaviour.md#5-the-cli-surface).
A third enumeration would be [AR-M1](../history/m1e_review_processing/README.md#the-findings)'s
parallel copy. What this page states is the *shape*, read off that golden:
**71** options across fifteen parsers, in the default build (`einb`,
`parallel`, `snmalloc`).

| parser | options |
|---|---:|
| `ein solve` | 33 |
| `ein oracle` | 10 |
| `ein test` | 8 |
| `ein saturate` | 5 |
| `ein render kb` | 4 |
| `ein render region` | 3 |
| `ein render rule` | 2 |
| `ein render lattice` | 2 |
| `ein render program` | 2 |
| `ein render rules` | 1 |
| `ein kb save` | 1 |

`ein`, `ein kb`, `ein render` and `ein render constraints` take none — they are
dispatchers. `ein kb save` exists only under the `einb` feature; a
`--no-default-features` build has 70 and no `ein kb` at all. M1e2
S1e2.1a.7 added the last two, `ein render region`'s `--visited` and
`--answer`: a region run's `--dump-lattice` and `--dump-answer` drawn on the
cube ([`04_dot_rendering.md` § Region rendering](ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run)).

**M1e2 S1e2.1a.5's switch** (2026-10-18) made `ein solve` the region's
command: bare `ein solve FILE` describes R, and `--minimal`, `--maximal`,
`--single` and `-n N` ask one question about it, answered by
[`ein_infer::explore`](../../ein.rs/crates/ein-infer/src/explore.rs), held to
the oracle by `ein-infer/tests/explore.rs`, and printed in
[`read_out.md`](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)'s
read-out. Every answer exits 0. **Nine options retired** and stay in the
parser hidden, so a stale argv is refused, exit 2, with what replaced it
(`ein-cli/src/solve.rs`'s `RETIRED`), and S1e2.1c.5 retired two more, `-L`
and `-K` (§ 3.2):

| retired | replaced by |
|---|---|
| `-e`, `--exhaustive` | bare `ein solve`, which describes all of R; `--maximal` for the walk's models |
| `-m`, `--max-set-size` | `--minimal`, the least commitments and their sizes; `--region-budget FACTS` for the work |
| `-E`, `--max-enterings` | `--region-budget FACTS`, which stops a run with what it established |
| `-o`, `-z`, `-d` | nothing: each route's order is part of its definition |
| `-D`, `--dump-states` | `--dump-answer DIR` for each state, `--dump-lattice DIR` for each node visited |
| `--dump-region`, `--region-states` | `ein oracle FILE --dump DIR [--states]` |

`ein test` lost the walk's `-m` and `-E` the same way and took
`--region-budget`. `ein render lattice` lost `--max-set-size` for
`--region-budget`, and draws the nodes a describing run visits. The design is
[`solve_interface.md`](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md).

### 3.2 The option that shadows a flag

`ein-cli`'s `resolved_config` is the whole of it: one option, one flag, written
in one direction only.

| option | flag | set to |
|---|---|---|
| `-y`, `--lattice-sanity-check` | `lattice-sanity-check` | `true` |

`-L`/`--no-lookahead` and `-K`/`--no-kill-cache` shadowed
`enable-pre-branch-lookahead` and `enable-lookahead-kill-cache` until M1e2
S1e2.1c.5 retired them with their flags (the user's ruling of 2026-11-03):
they stay in the parser hidden and are refused, exit 2, with why, as the
switch's nine are. `-y` changes no answer and adds its own `paths` row; the
header line leaves it out, as it leaves out `-j`. Verify the row the same way
as any — a `(config …)` head that sets the flag, and `--dump-config` to see it.

### 3.3 What an option can do to the answer

Five classes. Only the first touches the answer, and then only by asking a
different question or stopping early.

- **The question and its budget** — `--minimal`, `--maximal`, `--single`,
  `-n`/`--solutions`, `--count`, `--monotone`, `--region-budget`,
  `--region-limit`, `-T`/`--max-time`. The first five choose what is asked
  about R. `--count` (M1e2 T1e2.1c.4.6, the user's ruling of 2026-11-03) is
  over all of R only: describe's route, R's two counts and the rows that
  qualify them, no state read, so no query answered and no claim checked,
  which one `claims` row and a `note:` per claim say. Beside an option that
  asks what it does not answer (a line, `-n`, `--single`, `--monotone`, a
  listed state's form, `-H`, `--trace`) it is a usage error, exit 2, naming
  the command that answers it. `--monotone` (M1e2 S1e2.1b.5) asks about the
  program instead: root's fixpoint and the static check, no node built, every
  verdict exit 0, and exclusive with the others. The budget (`--region-budget`, default
  2²⁰ = 1 048 576 facts beyond root for every mode, ruled 2026-11-03; it was
  2¹⁹; `ein solve --help` states it, from `downset::DEFAULT_BUDGET`), the
  cube's limit and the deadline stop a run, and then every surface says the answer is
  partial: a lower bound, a `note:` on stderr, `exhausted no` and why. A
  `-T` abort exits 2 and prints no answer, because a machine-dependent limit
  is never an answer. **`-n 0` is refused**, exit 2, since M1e
  [S1e.1.5](../history/m1e_review_processing/README.md#s1e15---n-0-q7), and so
  is every negative.
- **The check** — `-y`, § [3.2](#32-the-option-that-shadows-a-flag): its own
  row, and on a monotone program a mismatch is an engine error, exit 2; never
  a different answer.
- **Execution** — `--jobs N` / `--jobs auto`: the same read-out, byte for byte,
  by [S1a.7.5](../history/m1a_rust/README.md#s1a75--the---jobs-contract)'s
  contract, pinned by `ein-render/tests/jobs_invariance.rs` and
  `region_modes.rs::jobs_change_no_line_of_a_mode`. The default is 1 and stays
  1.
- **Presentation** — `-p`, `-P`, `-f`, `-s`, `-t`, `-v`, `-H`, `-c`, `-g`
  (under `-v`, every N-th node; 0 logs none and a negative is refused, exit 2,
  since M1e2 S1e2.3.4, the user's ruling (b) of 2026-11-03),
  `--layer-progress`, `--models` (`list`, `key`, `grid`, over the states the
  read-out showed), `--diff-models FILE`, and `ein render program`'s / `ein
  render kb`'s six, which draw a file and solve nothing, and `ein render
  region`'s three, which draw an `ein oracle --dump` directory (`--view`) with
  a solve run's `--dump-lattice` and `--dump-answer` on it (`--visited`,
  `--answer`) and load nothing. stdout and stderr only; a `--diff-models` file that does not parse
  is a usage error, exit 2, before the run. **Six of them are forms printed
  after the read-out, and share its 100 lines** (M1e2
  [S1e2.3.3](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.3_the_forms_past_100_lines.md),
  the user's ruling (a) of 2026-11-03): `-H`, `--models key|grid`, `-p`,
  `-P`, `-f` and `--diff-models` are drawn from the states stdout showed and
  print what fits, at least one item, and then a line naming the rest; that
  rest is `--dump-answer DIR`'s `forms/<form>.txt`, uncut and drawn from every
  listed state ([`answer_dump.md`](inference/answer_dump.md)). **Two of them compare the listed
  states pairwise, and each prices its pair table before it allocates it**
  (M1e2 [S1e2.4.2](../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md)).
  The bound is `ein_render::models::PAIR_TABLE_BOUND`, **16 MiB**, a constant
  and not an option. `--models key` holds 2(n + 1) masks of k(k−1)/2 bits and
  4 bytes a pair for k states over n varying slots. Past the bound it prints
  *determining key — none within budget* and the numbers, as it does past its
  search budget, and exits 0 with the states listed above it.
  `--diff-models` holds 24 bytes per unmatched (recorded, expected) pair for
  its *near* pairing. Past the bound it pairs nothing near, says so in its
  block, and lists every unmatched model as missing or spurious. Both are held
  by `ein-render/tests/key_declines.rs`, which measures what each holds with a
  counting allocator. `ein solve examples/saturation/type-exclusivity/pets.ein
  --models key` held 8.1 GB before the bound and 84 MiB after it (2026-11-02).
  Since S1e2.3.3 stdout's key on `pets` is the 38 shown states', and the
  11 339 states' key is priced and declined only under `--dump-answer`.
- **Artefacts** — `--events`, `--events-level`, `--json-summary`
  (`ein-summary/2`), `--json-report` (`ein-test-report/4`), `--trace` with its
  four modifiers (`-F`, `-G`, `-l`, `-R`), `--dump-lattice`
  ([`lattice_dump.md`](inference/lattice_dump.md)), `--dump-answer`
  ([`answer_dump.md`](inference/answer_dump.md)), and `ein oracle`'s `--dump`
  with `--states` ([`region_dump.md`](inference/region_dump.md)). The
  directories are made before the run. **Additive** in the strict sense:
  stdout, stderr and the exit code are identical with them and without, and a
  write that fails after the run reports itself and leaves the answer's exit
  code ([`defined_behaviour.md` § 4.4](defined_behaviour.md)). A run answers
  every `(query …)` of its file from one solve, so no artefact flag is refused
  over several queries any more; `ein test` refuses `--events` and
  `--json-summary` over a selection of more than one file.

---

## 4. The environment

Twenty-six `EIN_*` names were greppable in this tree on 2026-08-28. Two are
read by the shipped binary since M1e2 S1e2.3.1 (2026-11-03), which retired
`EIN_OBLIGATION_CHOICE` (three after S1e2.1c.5 deleted the walk and the five it
alone read the same day, eight before); ten by the test harness; two by
`utils/` and the shell; and **six are not environment variables at all** — which is the part a grep
cannot tell you and the reason this section is a classification rather than a
list.

**Only the first class is pinned.** `the_shipped_environment_set_is_what_the_page_lists`
scans the shipping crates for `env::var` on an `EIN_*` literal and compares
both ways, so that table cannot drift. The other three are a **dated
census — 2026-08-28**, and deliberately not a check: the second and third are
a judgement about each read site, and the fourth would fail the moment
somebody proposed a new variable in a plan file, which is a page to update
and not a defect.

### Read by the shipped binary

Read from the process environment by a crate that ships. Neither reaches
`SolverConfig`, so neither re-blesses a KB-shape golden, and both are reached
by a command. The scan below pins the set *read by a shipping crate*.

| name | value | what it does | read at |
|---|---|---|---|
| `EIN_STDLIB` | a directory | overrides `std.*` resolution, ahead of the checkout walk and the embedded copy. Since M1e S1e.3.5 it **must carry `MANIFEST.sha256`**, the same marker the walk requires, and the refusal names the variable at the first `std.*` import — [`docs/install.md`](../install.md#point-it-at-a-different-stdlib). `ein --version` still reports an unprovable one as `unreadable` rather than refusing | `ein-ir/src/stdlib.rs` `resolve` |
| `EIN_LEFTOVER` | `1` | fills `--json-summary`'s `leftover` block — what the blind enumerator would still propose at each **listed state**, outside H. **Config-free** since M1e2 S1e2.3.1, as H is: no lookahead filter, so `zebra2`'s unique state leaves 8 203 where the walk's filter left 3 678 (`EIN_LEFTOVER=1 ein solve --maximal examples/zebra2.ein --json-summary J`, 2026-11-03). Runs on a fork of each state, so nothing else in the summary moves (`leftover_probe.rs`) | `ein-cli/src/summary.rs` |

**Five names left this table at M1e2 S1e2.1c.5 (2026-11-03)**, with the code
that read them: the walk's `EIN_TRAVERSAL` (its tree traversal, below) and
`EIN_BATCH_PER_WORKER` (its enterings in flight per worker), both in
`solve.rs`; and the instruments `EIN_FORK_DELTA` and `EIN_FORK_AUDIT`
(`--features fork-delta`, `utils/fork_delta_verify.py`) and `EIN_SPEC_AUDIT`
(`--features spec-audit`, `utils/spec_audit.py`), which audited the walk's
fork and its speculated layers and went with their features and drivers.

**A sixth left at M1e2 S1e2.3.1 (the same day), retired by the user's
ruling**: `EIN_OBLIGATION_CHOICE` (`off` \| `fail-first` \| `rule-order`), the
obligations rung's walk-order lever, whose `off` was the pre-S1d.2.5 engine and
the control arm of every number in `hypotheses_from_obligations.md`. No route
walks the ladder, and the universe is taken config-free with the rung in report
order, so its one reader after S1e2.1c.5 was `--json-summary`'s `root` block,
which ran the walk's generator on a fork of root. Once that block became root's
fixpoint and H as the routes have them, the variable changed nothing the binary
prints, and `oblgen::Choice::from_env` went: a binary that sets it runs as one
that does not, with no warning, as for the five above. The rung's order stays
pinned at `rule-order`.

### Read by the test harness

Not the binary's: these are read by `#[cfg(test)]` code, by a `tests/` target,
or by one of the two dev-only crates (`ein-corpus`, `ein-narration`, both
`publish = false`). Setting one has no effect on `ein solve`.

| name | what it does | read at |
|---|---|---|
| `EIN_BLESS` | `=1` rewrites a golden instead of comparing it — including this page's generated block | `ein-corpus/src/lib.rs` |
| `EIN_CORPUS_SLOW` | `=1` adds the `slow = true` corpus entries' cells, and the deep arm of `obligation_rung` | `ein-cli/tests/corpus_cli.rs`, `ein-infer/tests/obligation_rung.rs` |
| `EIN_CORPUS_TIMEOUT` | per-cell wall budget in seconds, default 300 | `ein-cli/tests/corpus_cli.rs` |
| `EIN_ID_SEEDS` | how many interner permutations `id_order_invariance` sweeps | `ein-render/tests/id_order_invariance.rs` |
| `EIN_ID_FILES` | point that sweep at a directory instead of the corpus — the fuzzer's seam | `ein-render/tests/id_order_invariance.rs` |
| `EIN_ID_REPORT` | print the sweep's per-file report | `ein-render/tests/id_order_invariance.rs` |
| `EIN_JOBS_SWEEP` | the job counts `jobs_invariance` runs, e.g. `2,4,8,16` | `ein-render/tests/jobs_invariance.rs` |
| `EIN_FUZZ_ITERS` | the parser fuzzer's budget, default 2 000 | `ein-ir/tests/fuzz_properties.rs` |
| `EIN_FUZZ_SEED` | moves that fuzzer's stream | `ein-ir/tests/fuzz_properties.rs` |
| `EIN_NARRATION_STRICT` | `=1` runs the id-order, jobs and corpus-shape comparisons with no narration cut — a fork's derivation compared byte for byte, which is how the cut is measured | `ein-narration/src/lib.rs` |

### Read by `utils/` and the shell

| name | what it does | read at |
|---|---|---|
| `EIN_BIN` | which `ein` a script drives; default `ein.rs/target/release/ein`. Every `utils/` script that runs the engine names the binary, through this or through `--bin` | `utils/*.py`, `utils/render_examples.sh` |
| `EIN_RULE_MODE` | `sidebyside` (default) or `overlay`, forwarded to `ein render rules` | `utils/render_examples.sh` |

### Not an environment variable at all

Six names a grep for `EIN_[A-Z_]*` returns that nothing ever reads from the
environment. Naming them is the point of this section: the census that
produced it was written because *"what does `ein` read from the environment"*
had no answer short of doing this exercise, and a grep's answer would be wrong
by six.

| name | what it actually is |
|---|---|
| `EIN_RS` | a Python module-level constant — `EIN_RS = Path(os.environ.get("EIN_BIN", …))` in `utils/e2e_baseline.py` and `utils/feature_matrix.py`, and a path constant in `utils/profile_ein_rs.py`. It reads `$EIN_BIN`; nothing reads it |
| `EIN_CMD` | a shell array in `utils/render_examples.sh`, assigned from `$EIN_BIN` |
| `EIN_C_PROBLEM_H` | an include guard in `c/problem.h` |
| `EIN_RENDER_LEVI` | **documented and never implemented.** `ir/03-ein-lang/04_dot_rendering.md` claims it; the only Levi switch is the library-level `DotOpts.levi`. It is one of [CD-H1](../history/m1e_review_processing/README.md#the-findings)'s items |
| `EIN_PRIORITY` | *proposed* by [S1f.5.15](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md); does not exist |
| `EIN_NOGOOD_INJECT` | *proposed* by [S1e.1.2](../history/m1e_review_processing/README.md#s1e12--determinism-under---jobs-q1); does not exist |

### What `EIN_TRAVERSAL=tree` reports

**The record of the walk's tree.** Since M1e2 S1e2.1a.5 no command ran the
walk, and since S1e2.1c.5 (2026-11-03) the walk and its tree are deleted, the
variable with them; what follows is what it reported while a command ran it,
kept because the register's `W5` twin and M1d's traversal numbers cite it.

It was the one **experimental** surface in the system, and the only knob whose
effect a reader cannot infer from the tables above: it does not change a
setting, it runs a **second traversal**. What *experimental* licenses was
[Q-M1e.5](../history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)'s
question, closed on 2026-11-03 by the user's ruling (a) on M1e2
[S1e2.3.4](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.4_the_inherited_questions_and_the_record.md):
nothing. The ruling of 2026-10-16 already says that no two commands may
disagree and every answer agrees with the oracle, and a surface called
experimental is held to it like any other. What that traversal *reports* has been
observable since M1d S1d.10.6 and stated nowhere, which is M1e
[S1e.3.2](../history/m1e_review_processing/README.md#s1e32--semantics-medium)'s
`SE-M3`. This is the shipped subset — not
[T1d.10.6.4](../history/m1d_satisfiability/README.md#s1d106--the-traversal),
which is still the open question of what a tree *should* report where a lattice
reports layers.

It applies only where the tree **accepts**: the traversal runs on the
obligations rung and declines on every other. A **declined** run answers
exactly as the lattice does — every field of `--json-summary` is identical on
`examples/zebra2.ein`, counters included — but its `--events` stream is not:
root's probe is a real generation call, so a declined run carries one extra
pass of it (125 further `hyp` and 125 further `compile` lines on that file)
plus the `traversal` line that says it declined.

**1 — `exhausted` is `false`, always.** A tree terminates by *discharge* and a
lattice by *exhaustion*, and the sentence saying what discharge licenses is
T1d.10.5.1's and is not written. Until it is, `Run::tree` sets `truncated`
unconditionally, so every count a tree reports is a **lower bound** and every
read-out says so. Three surfaces carry it:

- an `Ambiguity` prints `solutions (k)   3   (a lower bound — the search did
  not exhaust)` and *"distinct complete models **found**"*, the qualifier
  S1d.3.3 added;
- a `Contradiction` reads *No model found — the search did not exhaust the
  lattice* over *refuted so far (n facts)*, where the same program under the
  lattice reads *No solution — the constraints are contradictory* over *unsat
  core (n facts)* — the same n, the same facts, a weaker claim;
- **`ein test` cannot mark such a program's claim `held`.** An expectation is a
  claim about the *exhausted* answer, so every `:expect` on a program the tree
  accepts comes back `NOT CHECKED` and the runner exits **1**. That is not a
  defect in either component and it is not visible from either: it is what
  running a non-certifying traversal under a command that exhausts by
  definition comes to.

**2 — a dead branch is recorded, and nothing is learned from it.** M1e
[S1e.2.1](../history/m1e_review_processing/README.md#s1e21--correctness-the-panic-the-guard-the-traversal),
`CO-H3`(b): the refuted commitment and its core reach the answer — so a
`Contradiction` states what it refuted and `--trace` lists the branches — while
the clause store and the singleton `(not h)` writeback stay untouched. So
`nogoods_emitted` and `nogoods_subsumed` are **0** on any tree run, `emitted=0`
is honest rather than lost, and the search is unchanged: the published **86
enterings** on `examples/zebra2-minus-15-obligations.ein` were re-measured
after the recording landed.

**3 — `layers_explored` is the deepest node.** A tree has no layers; the
counter carries its depth instead, which is a different quantity under the same
name. That conflation is precisely `T1d.10.6.4`'s question, and it is why this
is an environment variable and not a flag.

**4 — the event stream is missing four kinds.** One `traversal` line says
whether the tree ran and what it took of the stop policy; `enter`, `layer`,
`nogood` and `writeback` are **not emitted at all**, so the enterings are
invisible in a stream that still counts them. The row and the reasons are
[`inference/events.md` § `traversal`](inference/events.md#traversal--the-second-traversal-and-the-four-kinds-it-does-not-emit).

**5 — the stop policy** is § [3.3](#33-what-an-option-can-do-to-the-answer):
`-n` honoured, `-m` refused at exit 2 in all three subcommands.

---

## 5. What is deliberately *not* configuration

Three knobs sit outside `SolverConfig` on purpose, and the reason is the same
each time: **every `SolverConfig` field is `--dump-config`-printed,
`(config …)`-settable and rendered into the KB-shape digest.**

- **`--jobs`** ([T1a.7.5.1](../history/m1a_rust/README.md#s1a75--the---jobs-contract)).
  A field would let a *puzzle file* set a thread count — a `.ein` that reads
  differently on an 8-core machine than on a 4-core one, through a field the
  digest compares. Jobs is an execution knob; `SolverConfig` is the semantics.
- **`--models`** (M1d T1d.3.3.4). Presentation has no business in the digest.
- **`EIN_TRAVERSAL` and `EIN_OBLIGATION_CHOICE`**, the walk's, both retired
  (§ 4). Both existed to be *compared*, and a knob whose settings are being
  compared would re-bless every shape golden in the corpus each time the
  comparison moved.

And there is **no config file**. The `(config …)` head travels with the
program, which is what makes a `.ein` reproducible on its own.

---

## 6. What pins this page

[`ein-cli/tests/config_reference.rs`](../../ein.rs/crates/ein-cli/tests/config_reference.rs),
seven tests, in `cargo test --workspace`. The phase's design constraint is
*generate or diff, never a third hand-maintained copy*, and each of these is
one or the other.

| test | what fails it |
|---|---|
| `the_defaults_block_is_the_binarys_own_dump_config` | the block in § 2.1 is not what `ein solve --dump-config` prints — a new flag, a renamed one, a changed default |
| `every_flag_has_a_row_and_no_row_is_orphaned` | § 2.2's flags are not `FLAGS` in declaration order, or a row's type or default is not the block's |
| `the_cli_counts_come_from_the_golden` | § 3.1's per-parser counts or the total are not `help_shape.txt`'s |
| `the_shipped_environment_set_is_what_the_page_lists` | a shipping crate reads an `EIN_*` name with no row in § 4's first class, or that class lists one nothing reads |
| `the_unread_flags_change_nothing_printed` | flipping one of § 2.2's five **unread** flags changes `solve -p`'s stdout, stderr or exit code on either of two fixtures |
| `the_retired_flags_are_refused_with_why` | a program setting one of § 2.4's fifteen loads, or is refused without the stage and the reason |
| `the_surface_table_agrees_with_the_sections_it_summarises` | § 1's three counts are not the ones §§ 2–4 own |

**Re-banking the block:**

```sh
EIN_BLESS=1 cargo test --manifest-path ein.rs/Cargo.toml -p ein-cli --test config_reference
```

Edit the engine, run that, commit. **Never edit the generated block by hand** —
it is `docs/api/rust.md`'s rule for the same reason.

The prose columns are not generated and cannot be: *what it changes* is read
off a call site and *does it change the answer* is a judgement with evidence.
What the tests give them is a **shape** guarantee — every flag has a row, no
row is orphaned, and the two mechanical columns beside the prose agree with
the binary. A wrong judgement is still possible; a *missing* one is not.

---

## Cross-references

- [`inference/features.md`](inference/features.md) — the measured feature ×
  config matrix: ten levers on `zebra2`, fast and exhaustive, with counters.
  This page says what a flag *is*; that one says what it is *worth*.
- [`inference/solution_semantics.md`](inference/solution_semantics.md) — what
  the two answer columns are about: the region R, and what a run reports
  today.
- [`defined_behaviour.md`](defined_behaviour.md) — the config diagnostics'
  wording, the CLI surface's guarantees, and the exit codes.
- [`docs/install.md`](../install.md) — `$EIN_STDLIB` and the three resolution
  steps, `ein --version`'s five lines.
- [`corpus/README.md`](../../corpus/README.md) § `levers` — which four flags
  the corpus sweep runs both ways, and why it cannot run the rest.
- [`utils/feature_matrix.py`](../../utils/feature_matrix.py) — how a lever
  reaches the engine when there is no CLI flag for it: a generated
  `(config …)` block holding the puzzle's own resolved configuration with one
  key changed.
