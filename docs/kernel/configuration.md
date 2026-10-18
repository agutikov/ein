# Ein — configuration reference

> **Normative, and new (2026-08-28).** Read at `4a47aa3`, M1e
> [S1e.5.1](../history/m1e_review_processing/README.md#s1e51--the-configuration-reference).
> It supersedes no page: before it the seventeen `(config …)` flags were stated
> only in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs)'s `FIELDS`
> and the nine field doc comments that had one, the CLI options only in
> `--help`, and the `EIN_*` environment nowhere at all. **Since M1e2
> S1e2.1a.5's switch (2026-10-18) a program sets eight of them**, nine retired
> with the walk, and `ein solve` has 33 options where it had 42.
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
| `(config …)` flags | **8** | `FLAGS` in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs), and `RETIRED` for the nine a program can no longer set; the form's shape is [`01_grammar.md` § Top-level forms](ir/03-ein-lang/01_grammar.md#2-top-level-forms) |
| CLI options | **69** | `--help`, and [`golden/help_shape.txt`](../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt), which `help_surface.rs` diffs |
| `EIN_*` environment | **8** read by the shipped binary, of 26 greppable names | § [4](#4-the-environment) |

**Precedence**, highest first:

1. **A CLI option** — but only the three that shadow a flag, § [3.2](#32-the-three-options-that-shadow-a-flag).
2. **The program's own `(config …)` head.** Several blocks are allowed and the
   loader keeps the **last one whole** — not a merge:
   `(config :warn-derived-naf true) (config :enable-path-nogoods false)` leaves
   `warn-derived-naf` at its default
   ([`from_ir.rs`](../../ein.rs/crates/ein-ir/src/from_ir.rs), `config_blocks.last()`;
   the last-wins rule is stated in [`01_grammar.md`](ir/03-ein-lang/01_grammar.md)).
3. **`SolverConfig::default()`** — the block in § [2.1](#21-the-defaults).

`ein solve --dump-config` prints the **resolved** result of the three, and is
the only way to see what a run actually used.

Two things are deliberately *not* in that order. There is **no user-level
config file** and no `$EIN_CONFIG`. And **no environment variable reaches
`SolverConfig`**: the `EIN_*` levers that steered the walk sit beside it,
because `SolverConfig` is rendered into the KB-shape digest and a knob whose
settings are being *compared* would re-bless every shape golden in the corpus
(§ [5](#5-what-is-deliberately-not-configuration)).

---

## 2. `(config …)` — the eight solver flags

```lisp
(config :enable-symmetric-mirror false :warn-derived-naf true)
```

Names are `:kebab-case`, and since M1e2 S1e2.1a.5 every flag a program can set
is a bool: `true` / `false`, bare or quoted. An unknown flag, a wrong-typed
value and a **retired** flag are all **load errors**, exit 1. The unknown-flag
message lists the valid names *sorted*, where `--dump-config` prints them in
declaration order, and the retired-flag message says why the flag went
([`defined_behaviour.md` § Errors and exit codes](defined_behaviour.md#4-errors-and-exit-codes)):

```text
kb load error: (config …): unknown config flag :nonsense (expected one of: …)
kb load error: (config …): config flag :warn-derived-naf expects true/false, got Int(value=7)
kb load error: (config …): config flag :lattice-order was retired at M1e2 S1e2.1a.5: an order lever, and every route's order is part of its definition
```

### 2.1 The defaults

`ein solve --dump-config` on a program with no `(config …)` head:

<!-- generated: ein solve --dump-config -->
```text
config (resolved)
  enable-pre-branch-lookahead      true
  enable-lookahead-kill-cache      true
  warn-derived-naf                 false
  lattice-sanity-check             false
  enable-path-nogoods              true
  enable-symmetric-mirror          true
  record-alternative-justifications true
  enable-fail-fast-fork            true
```
<!-- /generated -->

### 2.2 The flags

| flag | type | default | what it changes | answer? | R? | stability |
|---|---|---|---|---|---|---|
| `enable-pre-branch-lookahead` | `bool` | `true` | the walk's one-step lookahead, which dropped a candidate it could prove died. **No route reads it** since the switch: route 1 and the cube take H config-free (`region.rs` § The universe). `-L` sets it `false` | unread | no | lever, unread |
| `enable-lookahead-kill-cache` | `bool` | `true` | the walk's cache of a lookahead kill as `(not h)`. No route reads it; `-K` sets it `false` | unread | no | lever, unread |
| `warn-derived-naf` | `bool` | `false` | **two questions, one flag**, both `warn` events on the stream after root's saturation, on every run that records one (M1e2 S1e2.1a.5 kept them when the walk left). `DerivedNafWarning` — the `(absent …)` guard watches a **rule-derived** relation: a *stratification* signal since S1.21.8, not a soundness one. `RefutationUnderAbsentWarning` (M1e [S1e.2.3](../history/m1e_review_processing/README.md#s1e23--q-m1e9s-containment-a-diagnostic-for-a-refutation-resting-on-an-absent)) — the rule concludes `(false)` or a `(not …)` from a guard over a relation the program's **generator can still propose**, which *is* a soundness signal: [`dead` is not upward-closed under `absent`](../history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent) | no | no | contract |
| `lattice-sanity-check` | `bool` | `false` | the walk's re-saturation of every `(k−1)`-subset parent path, aborting on a mismatch. No route runs it; `-y` sets it `true` | unread | no | probe, unread |
| `enable-path-nogoods` | `bool` | `true` | whether the walk's dead commitment emitted a learned clause. No route reads it: route 1's `N` is the minimal dead sets it met, not a learned clause | unread | no | lever, unread |
| `enable-symmetric-mirror` | `bool` | `true` | the kernel's native arg-swap closure over relations marked `(__symmetric__ R)`. Off, such a relation is closed only if the program's own rule closes it | **yes** | **yes** | contract |
| `record-alternative-justifications` | `bool` | `true` | whether a re-derivation of a known fact is recorded as an alternative justification or dropped: the proof is an AND/OR graph, or a tree. It decides which minimal core a dead node's refutation names — the read-out's `unsat core`, the trace's `N` and `nogoods.json` | explanation only | no | contract |
| `enable-fail-fast-fork` | `bool` | `true` | whether the walk's fork stopped at the firing that made it inconsistent. No route reads it: every node is run to its fixpoint, so a dead node's debts are its fixpoint's | unread | no | lever, unread |

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
| **unread** | read by no route since M1e2 S1e2.1a.5 — the walk's library reads it, and no command runs the walk. P1e2.1c gives it a meaning on a route or retires it |

**Stability** — what depending on a flag commits you to.

| value | means |
|---|---|
| **contract** | stated here, exercised by the corpus or a named test, safe to depend on |
| **lever, unread** | kept by the interface as a speed lever (`solve_interface.md` § 2.4) and read by no route yet. The corpus's `levers` cells run `-L`, `-K` and `-y` on every `cargo test` and hold the read-out unmoved (C4) |
| **probe, unread** | exists to test an *invariance*, and no route runs it yet |

**The four "unread" flags and `lattice-sanity-check` are the inert flags of
[Q-M1e.10](../history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)'s
kind**, said here rather than left to be found: settable, validated,
`--dump-config`-printed, `--json-summary`-echoed, rendered into the KB-shape
digest — and read by no route. They are kept because the approved interface
keeps them as levers for P1e2.1c's routes, and the claim that they move
nothing is held from outside by
`config_reference.rs::the_unread_flags_change_nothing_printed`.

### 2.4 The nine retired flags

M1e2 S1e2.1a.5's switch retired nine, each refused at load with the reason
below (`ein_core::config::RETIRED`). Their fields stay on `SolverConfig` at
their defaults for the walk's library code, and the `.einb` container
round-trips them, so its format did not move.

| flag | why it went |
|---|---|
| `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight`, `lattice-order`, `lattice-order-seed` | order levers: every route's order is part of its definition, so `-n N` prints the same states under every lever (C4) |
| `enable-singleton-writeback`, `enable-forced-positive` | they wrote into root, which no route does. What they knew survives as licence L1: a candidate dead alone leaves the universe |
| `print-alive`, `candidate-order-seed` | inert: read by no code path ([Q-M1e.10](../history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)) |

Six corpus files set one, and lost the line at the switch: `examples/zebra.ein`
(`lattice-order`, `enable-forced-positive`), `examples/syntax/config.ein`, and
three register witnesses whose program is now their sibling's (`forced-2-off`,
`f07-fpoff`, `naf-nowb`).

**What the answer column was read from.** A sweep of 2026-10-18: each of the
eight flipped in an appended `(config …)` block (last-wins, so programs with a
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
it off and says why.

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
- **`enable-symmetric-mirror` changes R**, on `features/06` (§ 2.4). The
  mirror is how `(__symmetric__ R)` means anything, so the flag switches off
  part of the program's theory. It is the one flag that **violates the
  principle**, and its question is
  [S1e2.1a.6](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)'s.
- **`lattice-sanity-check` aborted one program under the walk**:
  [`corpus/fuzz_findings/d3-unsat-core.ein`](../../corpus/fuzz_findings/d3-unsat-core.ein),
  which the static check flags non-monotone, failed the probe with
  *"Saturation commutativity violated"*. No route runs the probe since the
  switch; which state a node of a non-monotone lattice is remains
  [P1e2.1b](../../plans/m1e2_correctness/p1e2.1b_the_non_monotone_lattice/README.md)'s
  S1e2.1b.2.

**What the levers did to the walk's report** — the register rows they reached,
`-L`'s W1, W2 and W4 rows, `-K`'s W9 rows, the writeback's W3 row — were the
walk's, and flipped to `fixed` at the switch where R answers them
([`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml)). Under a region
run every lever reads **no** or **unread**, `enable-symmetric-mirror` apart, and
so do `--jobs` (§ 3.3) and every presentation and artefact option: they change
speed, or what is printed beside the answer, never the answer.

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
**69** options across fifteen parsers, in the default build (`einb`,
`parallel`, `snmalloc`).

| parser | options |
|---|---:|
| `ein solve` | 33 |
| `ein oracle` | 10 |
| `ein test` | 8 |
| `ein saturate` | 5 |
| `ein render kb` | 4 |
| `ein render rule` | 2 |
| `ein render lattice` | 2 |
| `ein render program` | 2 |
| `ein render rules` | 1 |
| `ein render region` | 1 |
| `ein kb save` | 1 |

`ein`, `ein kb`, `ein render` and `ein render constraints` take none — they are
dispatchers. `ein kb save` exists only under the `einb` feature; a
`--no-default-features` build has 68 and no `ein kb` at all.

**M1e2 S1e2.1a.5's switch** (2026-10-18) made `ein solve` the region's
command: bare `ein solve FILE` describes R, and `--minimal`, `--maximal`,
`--single` and `-n N` ask one question about it, answered by
[`ein_infer::explore`](../../ein.rs/crates/ein-infer/src/explore.rs), held to
the oracle by `ein-infer/tests/explore.rs`, and printed in
[`read_out.md`](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)'s
read-out. Every answer exits 0. **Nine options retired** and stay in the
parser hidden, so a stale argv is refused, exit 2, with what replaced it
(`ein-cli/src/solve.rs`'s `RETIRED`):

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

### 3.2 The three options that shadow a flag

`ein-cli`'s `resolved_config` is the whole of it: three options, three flags,
and each writes in one direction only. There is no `--lookahead`, so an option
can never restore a default the program turned off. All three flags are
**unread** by every route (§ 2.2), so each option changes nothing printed, and
the header line leaves them out, as it leaves out `-j`.

| option | flag | set to |
|---|---|---|
| `-L`, `--no-lookahead` | `enable-pre-branch-lookahead` | `false` |
| `-K`, `--no-kill-cache` | `enable-lookahead-kill-cache` | `false` |
| `-y`, `--lattice-sanity-check` | `lattice-sanity-check` | `true` |

Verify any row the same way — a `(config …)` head that sets the flag the other
way, and `--dump-config` to see which won.

### 3.3 What an option can do to the answer

Five classes. Only the first touches the answer, and then only by asking a
different question or stopping early.

- **The question and its budget** — `--minimal`, `--maximal`, `--single`,
  `-n`/`--solutions`, `--region-budget`, `--region-limit`, `-T`/`--max-time`.
  The first four choose what is asked about R; the budget, the cube's limit
  and the deadline stop a run, and then every surface says the answer is
  partial: a lower bound, a `note:` on stderr, `exhausted no` and why. A
  `-T` abort exits 2 and prints no answer, because a machine-dependent limit
  is never an answer. **`-n 0` is refused**, exit 2, since M1e
  [S1e.1.5](../history/m1e_review_processing/README.md#s1e15---n-0-q7), and so
  is every negative.
- **The three shadows** — § [3.2](#32-the-three-options-that-shadow-a-flag),
  unread.
- **Execution** — `--jobs N` / `--jobs auto`: the same read-out, byte for byte,
  by [S1a.7.5](../history/m1a_rust/README.md#s1a75--the---jobs-contract)'s
  contract, pinned by `ein-render/tests/jobs_invariance.rs` and
  `region_modes.rs::jobs_change_no_line_of_a_mode`. The default is 1 and stays
  1.
- **Presentation** — `-p`, `-P`, `-f`, `-s`, `-t`, `-v`, `-H`, `-c`, `-g`,
  `--layer-progress`, `--models` (`list`, `key`, `grid`, over the states the
  read-out listed), `--diff-models FILE`, and `ein render program`'s / `ein
  render kb`'s six, which draw a file and solve nothing, and `ein render
  region`'s `--view`, which draws an `ein oracle --dump` directory and loads
  nothing. stdout and stderr only; a `--diff-models` file that does not parse
  is a usage error, exit 2, before the run.
- **Artefacts** — `--events`, `--events-level`, `--json-summary`
  (`ein-summary/2`), `--json-report` (`ein-test-report/3`), `--trace` with its
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

Twenty-six `EIN_*` names are greppable in this tree. Eight are read by the
shipped binary; ten by the test harness; two by `utils/` and the shell; and
**six are not environment variables at all** — which is the part a grep
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

Read from the process environment by a crate that ships. None of them reaches
`SolverConfig`, so none re-blesses a KB-shape golden. **Since M1e2
S1e2.1a.5's switch only `EIN_STDLIB` and `EIN_LEFTOVER` are reached by a
command.** The other six are read by the walk's library code, which ships,
links and is reached by no command until P1e2.1c decides what of it the region
routes reuse; setting one changes nothing an `ein` run prints. The scan below
pins the set *read by a shipping crate*, which is why they stay listed.

| name | value | what it does | read at |
|---|---|---|---|
| `EIN_STDLIB` | a directory | overrides `std.*` resolution, ahead of the checkout walk and the embedded copy. Since M1e S1e.3.5 it **must carry `MANIFEST.sha256`**, the same marker the walk requires, and the refusal names the variable at the first `std.*` import — [`docs/install.md`](../install.md#point-it-at-a-different-stdlib). `ein --version` still reports an unprovable one as `unreadable` rather than refusing | `ein-ir/src/stdlib.rs` `resolve` |
| `EIN_TRAVERSAL` | `tree` | **the walk's**: the per-obligation depth-first traversal beside its lattice (M1d S1d.10.6), § [What `EIN_TRAVERSAL=tree` reported](#what-ein_traversaltree-reports) | `ein-infer/src/solve.rs` `tree_traversal` |
| `EIN_OBLIGATION_CHOICE` | `off` \| `fail-first` \| *(default)* `rule-order` | **the walk's**: its obligations rung's measurement lever; `off` is the pre-S1d.2.5 engine and the control arm of every number in `hypotheses_from_obligations.md`. The region's universe is taken config-free, rung order fixed | `ein-infer/src/oblgen.rs` |
| `EIN_LEFTOVER` | `1` | fills `--json-summary`'s `leftover` block — what the blind enumerator would still propose at each **listed state**, outside H. Runs on a fork of each state, so nothing else in the summary moves (`leftover_probe.rs`) | `ein-cli/src/summary.rs` |
| `EIN_BATCH_PER_WORKER` | a positive integer, default 512 | **the walk's**: enterings in flight per worker. Needs the `parallel` feature, which is **on** by default | `ein-infer/src/solve.rs` `batch_per_worker` |
| `EIN_FORK_DELTA` | `0` | the pre-S1a.6.9 fresh-saturator fork path, for `utils/fork_delta_verify.py`'s two arms out of one binary. Needs `--features fork-delta`, **off** by default | `ein-infer/src/solve.rs` `resume_forks` |
| `EIN_FORK_AUDIT` | a file path | one JSON-Lines record per entering. Needs `--features fork-delta`, **off** by default | `ein-infer/src/fork_audit.rs` |
| `EIN_SPEC_AUDIT` | a file path | the speculation audit — every entering re-run against root as of layer start. Needs `--features spec-audit`, **off** by default | `ein-infer/src/spec_audit.rs` |

The last three are the walk's too and need a non-default build. `ein
--version`'s `features` line says which build you have.

### Read by the test harness

Not the binary's: these are read by `#[cfg(test)]` code, by a `tests/` target,
or by one of the two dev-only crates (`ein-corpus`, `ein-narration`, both
`publish = false`). Setting one has no effect on `ein solve`.

| name | what it does | read at |
|---|---|---|
| `EIN_BLESS` | `=1` rewrites a golden instead of comparing it — including this page's generated block | `ein-corpus/src/lib.rs` |
| `EIN_CORPUS_SLOW` | `=1` adds the two `slow = true` corpus entries, and the deep arm of `obligation_rung` | `ein-cli/tests/corpus_cli.rs`, `ein-infer/tests/obligation_rung.rs` |
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
| `EIN_PRIORITY` | *proposed* by [S1f.5.6](../../plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md); does not exist |
| `EIN_NOGOOD_INJECT` | *proposed* by [S1e.1.2](../history/m1e_review_processing/README.md#s1e12--determinism-under---jobs-q1); does not exist |

### What `EIN_TRAVERSAL=tree` reports

**The record of the walk's tree.** Since M1e2 S1e2.1a.5 no command runs the
walk, so no command reaches the tree; what follows is what it reported while
one did, kept because the register's `W5` twin and M1d's traversal numbers
cite it.

It was the one **experimental** surface in the system, and the only knob whose
effect a reader cannot infer from the tables above: it does not change a
setting, it runs a **second traversal**. What that traversal *reports* has been
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
- **`EIN_TRAVERSAL` and `EIN_OBLIGATION_CHOICE`**, the walk's. Both existed
  to be *compared*, and a knob whose settings are being compared would
  re-bless every shape golden in the corpus each time the comparison moved.

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
| `the_retired_flags_are_refused_with_why` | a program setting one of § 2.4's nine loads, or is refused without the reason |
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
