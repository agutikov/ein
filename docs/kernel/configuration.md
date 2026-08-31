# Ein — configuration reference

> **Normative, and new (2026-08-28).** Read at `4a47aa3`, M1e
> [S1e.5.1](../../plans/m1e_review_processing/p1e.5_documentation_and_other/s1e.5.1_config_reference.md).
> It supersedes no page: before it the seventeen `(config …)` flags were stated
> only in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs)'s `FIELDS`
> and the nine field doc comments that had one, the CLI options only in
> `--help`, and the `EIN_*` environment nowhere at all.
>
> **It is a reference, not a narrative.** One row per knob. What a lever *does
> to the search*, and what it was measured at, is
> [`inference/features.md`](inference/features.md); what a **solution** and a
> **model** are is
> [`inference/solution_semantics.md`](inference/solution_semantics.md); the
> newcomer path is [`docs/guide/`](../guide/README.md). A reference that
> explains twice will disagree with itself.
>
> **Nothing here is hand-maintained.** The defaults block is the binary's own
> `--dump-config` output, the flag table is diffed against `FIELDS`, the option
> counts are read off the help-shape golden, and the environment
> classification is scanned out of the shipping crates —
> [`ein-cli/tests/config_reference.rs`](../../ein.rs/crates/ein-cli/tests/config_reference.rs),
> six tests, in `cargo test --workspace`. § [6](#6-what-pins-this-page).

---

## 1. Three surfaces, and the order between them

| surface | how many | who owns the list |
|---|---:|---|
| `(config …)` flags | **17** | `FIELDS` in [`config.rs`](../../ein.rs/crates/ein-core/src/config.rs); the form's shape is [`01_grammar.md` § Top-level forms](ir/03-ein-lang/01_grammar.md#2-top-level-forms) |
| CLI options | **52** | `--help`, and [`golden/help_shape.txt`](../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt), which `help_surface.rs` diffs |
| `EIN_*` environment | **8** read by the shipped binary, of 29 greppable names | § [4](#4-the-environment) |

**Precedence**, highest first:

1. **A CLI option** — but only the six that shadow a flag, § [3.2](#32-the-six-options-that-shadow-a-flag).
2. **The program's own `(config …)` head.** Several blocks are allowed and the
   loader keeps the **last one whole** — not a merge:
   `(config :print-alive true) (config :warn-derived-naf true)` leaves
   `print-alive` at its default
   ([`from_ir.rs`](../../ein.rs/crates/ein-ir/src/from_ir.rs), `config_blocks.last()`;
   the last-wins rule is stated in [`01_grammar.md`](ir/03-ein-lang/01_grammar.md)).
3. **`SolverConfig::default()`** — the block in § [2.1](#21-the-defaults).

`ein solve --dump-config` prints the **resolved** result of the three, and is
the only way to see what a run actually used.

Two things are deliberately *not* in that order. There is **no user-level
config file** and no `$EIN_CONFIG`. And **no environment variable reaches
`SolverConfig`**: the `EIN_*` levers that steer the engine sit beside it,
because `SolverConfig` is rendered into the KB-shape digest and a knob whose
settings are being *compared* would re-bless every shape golden in the corpus
(§ [5](#5-what-is-deliberately-not-configuration)).

The library's order is the same shape with a different top: an explicit
`SolveOptions::config`, then `kb.config`, then the defaults
([`solve.rs`](../../ein.rs/crates/ein-infer/src/solve.rs)). `ein-cli` computes
1-then-2 and passes the result as that explicit argument, which is why the two
orders never disagree.

---

## 2. `(config …)` — the seventeen solver flags

```lisp
(config :enable-fail-fast-fork false :lattice-order "score-sum")
```

Names are `:kebab-case`; a value is `true` / `false`, an integer, or a bare or
quoted string. **The surface lexer has no float literal**, so the two
float-valued flags take an integer — `(config :hypgen-rel-weight 2)` resolves
to `2.0`, and `(config :hypgen-rel-weight 2.0)` is a parse error at the `.`.

An unknown flag and a wrong-typed value are both **load errors**, exit 1, and
the message lists the valid names *sorted* where `--dump-config` prints them in
declaration order —
[`defined_behaviour.md` § Errors and exit codes](defined_behaviour.md#4-errors-and-exit-codes).

```text
kb load error: (config …): unknown config flag :nonsense (expected one of: …)
kb load error: (config …): config flag :print-alive expects true/false, got Int(value=7)
```

### 2.1 The defaults

`ein solve --dump-config` on a program with no `(config …)` head:

<!-- generated: ein solve --dump-config -->
```text
config (resolved)
  enable-pre-branch-lookahead      true
  enable-lookahead-kill-cache      true
  hypgen-scoring                   popularity
  hypgen-rel-weight                1.0
  hypgen-obj-weight                1.0
  print-alive                      false
  warn-derived-naf                 false
  candidate-order-seed             -1
  lattice-sanity-check             false
  lattice-order                    lex
  lattice-order-seed               None
  enable-path-nogoods              true
  enable-symmetric-mirror          true
  enable-singleton-writeback       true
  enable-forced-positive           true
  record-alternative-justifications true
  enable-fail-fast-fork            true
```
<!-- /generated -->

### 2.2 The flags

| flag | type | default | what it changes | answer? | stability |
|---|---|---|---|---|---|
| `enable-pre-branch-lookahead` | `bool` | `true` | whether hypgen simulates one rule step over each candidate and drops the ones it can prove die (`hypgen.rs`). Not only a prune: the filtered list is what `complete()` asks, so it decides which states are recorded as solutions | **yes** | measurement lever |
| `enable-lookahead-kill-cache` | `bool` | `true` | whether a lookahead kill is written down as `(not h)`, so a later enumeration skips `h` through the negated index instead of re-simulating | **KB only** † | measurement lever |
| `hypgen-scoring` | `str` | `popularity` | which scorer `score_hypothesis` runs. Read **only** under `lattice-order "score-sum"`; under `lex` nothing calls it. `most-constrained` returns a constant `0.0`; `branch-info` and `popularity+branch-info` error at first call | order only | partly wired |
| `hypgen-rel-weight` | `float` | `1.0` | the coefficient on the relation's extent size in the popularity score | order only | contract |
| `hypgen-obj-weight` | `float` | `1.0` | the coefficient on each named argument's appearances as an argument | order only | contract |
| `print-alive` | `bool` | `false` | **nothing** | **inert** | inert |
| `warn-derived-naf` | `bool` | `false` | **two questions, one flag**, both emitted once after root's saturation. `DerivedNafWarning` — the `(absent …)` guard watches a **rule-derived** relation: a *stratification* signal since S1.21.8, not a soundness one. `RefutationUnderAbsentWarning` (M1e [S1e.2.3](../../plans/m1e_review_processing/p1e.2_high/s1e.2.3_naf_refutation_diagnostic.md)) — the rule concludes `(false)` or a `(not …)` from a guard over a relation the program's **generator can still propose**, which *is* a soundness signal: [`dead` is not upward-closed under `absent`](../../plans/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent). The second shares the flag because a `SolverConfig` field is rendered into the KB-shape digest, so an eighteenth would re-bless every shape golden in the corpus | no | contract |
| `candidate-order-seed` | `int` | `-1` | **nothing** | **inert** | inert |
| `lattice-sanity-check` | `bool` | `false` | for every alive commitment of size ≥ 2, re-saturates each `(k−1)`-subset parent path and compares the KBs — `k+1` extra saturations each. A mismatch aborts the run with `SolveError::Sanity` | no — but it can **fail** the run | probe |
| `lattice-order` | `str` | `lex` | the within-layer candidate order: `lex` is the canonical-tuple sort, `score-sum` sums `hypgen-scoring` over each set, descending, tie-broken by the tuple | order only | measurement lever |
| `lattice-order-seed` | `int` | `None` | seeds one CPython-MT19937 generator per *solve*, applied after the order to permute each layer. `None` disables it | order only | probe |
| `enable-path-nogoods` | `bool` | `true` | whether a dead commitment emits a learned clause. Off, a dead set that a clause would have subsumed is entered again | no | contract |
| `enable-symmetric-mirror` | `bool` | `true` | the kernel's native arg-swap closure over relations marked `(__symmetric__ R)`. Off, such a relation is closed only if the program's own rule closes it | **yes** | contract |
| `enable-singleton-writeback` | `bool` | `true` | whether a size-1 dead clause writes `(not h)` back to root, and whether layer-1 deaths integrate mid-layer rather than at the barrier | no † | contract |
| `enable-forced-positive` | `bool` | `true` | whether a sole-surviving alive singleton is promoted to a root fact and re-saturated — the backbone prune, with provenance `<forced-positive>` and empty premises | no | contract |
| `record-alternative-justifications` | `bool` | `true` | whether a re-derivation of a known fact is recorded as an alternative justification or dropped: the proof is an AND/OR graph, or a tree | explanation only | contract |
| `enable-fail-fast-fork` | `bool` | `true` | whether a fork's saturation stops at the firing whose conclusion makes it inconsistent, instead of running to quiescence and only then scanning | no | contract |

### 2.3 Reading the two judgement columns

The **answer** is what
[`solution_semantics.md`](inference/solution_semantics.md) defines: the
verdict, `k`, and the set of **models** — a model being the positive part of a
solution's KB minus the positive part of the initial KB. A run's recorded
*solution KB* is a bigger object than its models, and the fourth row below is
the whole reason the distinction is load-bearing here.

| value | means | witness |
|---|---|---|
| **yes** | can change the verdict, `k`, or the set of models | `-L` on `examples/lattice/02_genuine_3set_death.ein -e`: `Ambiguity k = 3` becomes an **exhausted** `Contradiction k = 0` |
| **KB only** | same verdict, same `k`, same models — a different recorded solution **KB** | `-K` on the same file: the three models keep every positive and lose their `(not (c-prop X))` |
| **order only** | same model *set*; a different order of discovery. Under a stop rule — and `-n 1` is the default — that is a different model on stdout, and where the search does not exhaust it can be a different *subset* of them | `-o score-sum` on `examples/saturation/type-exclusivity/colors.ein -e`: the same `k = 5` and the same facts, in a different order of `solution n/k` blocks |
| **explanation only** | same verdict, same models; a different unsat core or proof shape | `examples/zebra.ein`'s own `(config …)` comment, which turns `record-alternative-justifications` off and says what that costs and what it buys |
| **no** | same verdict, same `k`, same models, same recorded KBs. Counters and wall clock may move, sometimes by a lot | § [2.4](#24-what-the-answer-column-was-read-from) |
| **inert** | read by no code path at all | [Q-M1e.10](../../plans/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert) |

**† Two rows are scoped to programs where the alive-set premise holds.**
[Q-M1e.9](../../plans/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)
was answered *no* on 2026-08-28: `dead` is **not** upward-closed under
`absent`, and its twenty-line counterexample is a program on which five of the
engine's six shipped configurations report the wrong model with
`exhausted = true`. Three mechanisms read the refuted premise and all three
have a flag here, but only two of them *move* that program:

- **`enable-lookahead-kill-cache`** — `-K` changes the **model**. `{(q A)}`
  becomes `{(p A), (q A)}`, which is the right answer, by accident.
- **`enable-singleton-writeback`** — `false` together with `-L` changes the
  **verdict**: `Solution k = 1` becomes `Contradiction k = 0`.
- **`enable-path-nogoods`** is D4's third mechanism and flipping it does
  nothing there, alone or with the writeback off, with `-L` or without: the
  other two already reach the same wrong state. It keeps a plain **no**.

So those two rows read *KB only* and *no* against the corpus, where the
premise holds, and would read *yes* against a program where it does not. That
is a property of the premise, not of the flags;
[D4](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)
owns it.

**Stability** — what depending on a flag commits you to.

| value | means |
|---|---|
| **contract** | stated here, exercised by the corpus or a named test, safe to depend on |
| **measurement lever** | exists for an A/B. The default is the shipped path and the other arm is for measuring; `corpus.toml`'s `levers` runs four of them both ways on every entry that declares them, and [`features.md`](inference/features.md) is where the numbers are |
| **probe** | exists to test an *invariance*, not to be used. Turning it on asserts something about the engine, not about the puzzle |
| **partly wired** | the value set is larger than the behaviour. Only `hypgen-scoring`, whose four values are one real scorer, one constant and two that raise |
| **inert** | read by nothing — [Q-M1e.10](../../plans/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert) |

There is no **experimental** flag. The word applies to exactly one knob in the
system, `EIN_TRAVERSAL=tree`, and what it licenses is
[Q-M1e.5](../../plans/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)'s
open question.

### 2.4 What the answer column was read from

Each **no** and each **yes** has evidence, and it is not the same evidence for
all of them.

- **The four with a CLI lever** — `-L`, `-K`, `-y`, `-o score-sum` — are run
  both ways on every `cargo test`, over each corpus entry that declares them
  ([`corpus.toml`](../../corpus/corpus.toml)'s `levers`), and are four of the
  ten cells [`features.md`](inference/features.md) measures on `zebra2`.
- **The rest are reachable only from a `(config …)` head**, which is why
  `utils/feature_matrix.py` builds each cell as a copy of the puzzle with one
  generated block appended, and why the corpus sweep — which runs a process —
  cannot flip them ([Q-M1a.16](../history/m1a_rust/open_questions.md#q-m1a16--how-does-the-harness-drive-the-lever-matrix)).
- **A one-lever sweep of 2026-08-28** did the same across the tree: each flag
  flipped in an appended `(config …)` block, over the 29 `examples/` fixtures
  that state no such head of their own (appending is last-wins, so a fixture
  with one would have had it replaced whole), `solve -e -p` compared byte for
  byte against the same fixture unflipped.

  | lever | fixtures moved | which |
  |---|---:|---|
  | `enable-path-nogoods false` | 0 / 29 | |
  | `enable-symmetric-mirror false` | **1** / 29 | `features/06_symmetric_native` |
  | `enable-singleton-writeback false` | 0 / 29 | |
  | `enable-forced-positive false` | 0 / 29 | |
  | `record-alternative-justifications false` | 0 / 29 | |
  | `enable-fail-fast-fork false` | 0 / 29 | |
  | `warn-derived-naf true` | 0 / 29 | |
  | `lattice-order "score-sum"` | **3** / 29 | `branching/08`, `type-exclusivity/{colors,nationalities}` |
  | `hypgen-scoring "most-constrained"` | 0 / 29 | |
  | `lattice-order-seed 7` | **6** / 29 | the three above, plus `branching/02`, `lattice/02`, `features/11` |
  | `print-alive true` | 0 / 29 | |
  | `candidate-order-seed 7` | 0 / 29 | |

  Twelve of the seventeen. The two float weights are not in it — they are read
  only under `score-sum`, so flipping one alone is the `hypgen-scoring` row
  again — and the other three, `-L`, `-K` and `-y`, are the corpus sweep's,
  run both ways on every `cargo test`. The comparison is **stdout**, so the
  `warn-derived-naf` row says only that its warning does not reach stdout;
  the warning is a `warn` *event*, and `--events` is what carries it.

Five rows are worth a sentence each.

**`enable-symmetric-mirror` is `yes`, and `features.md`'s `1.0×` is not the
same claim.** That page measures `zebra2`, where the mirror has a transparent
fallback: the relations it would close are also closed by `std.algebra`'s
`symmetric` rule. `examples/features/06_symmetric_native.ein` marks three
relations and declares no such rule, and it is the only fixture in the tree
that reaches the mirror at all. With the flag off it derives **0 facts instead
of 3** and the query goal `(knows Bob Ann)` is no longer reached. A flag whose
effect is invisible on the puzzle everything is measured on is exactly the row
a reference has to get right.

**`lattice-order "score-sum"` reorders and does not re-select.** Of the three
fixtures it moves — `branching/08_hypothesis_relation_whitelist`,
`type-exclusivity/colors`, `type-exclusivity/nationalities` — all three keep
their `k` and the whole multiset of recorded facts, and what differs is which
`solution n/k` block holds which. All three are also `exhausted = false` runs,
which is where the caveat in the legend row bites: order is only *safe* while
the search finishes, and none of these three does at `-m 5`.
`hypgen-scoring` moved nothing on any of the 29 for the reason its row gives —
under the default `lattice-order lex` no code calls the scorer at all.

**`lattice-sanity-check` cannot change an answer but can remove one.** It is a
regression harness compiled into the solver: if a `(k−1)`-subset parent path
saturates to a different KB, the run ends in `SolveError::Sanity` and there is
no verdict at all. That is `no` in the answer column and a **probe** in the
stability one, and the two together are the honest description.

**`record-alternative-justifications` is the corpus's untested difference.**
Nothing in the 29-fixture sweep distinguishes the two settings, because the
flag changes *which* minimal core a refutation names and no fixture there
pins a core that has two. What it costs is measured and large, and
[`examples/zebra.ein`](../../examples/zebra.ein) is where: with the AND/OR
search on, minimising one dead branch's frontier took **~22.7 s**, half of a
whole stop-after-1 solve, against ~2 s for the branch's own saturation — so
that file turns the flag off and says why, in fifteen lines of comment. The
knob is a property of the *ontology*: `zebra2`'s typed relations give each
fact essentially one derivation, so it leaves the default alone.

**The two inert flags are inert in every direction that matters and live in
every direction that does not.** `print-alive` and `candidate-order-seed` are
parsed, type-checked, `--dump-config`-printed, `--json-summary`-echoed,
rendered into the KB-shape digest and round-tripped through `.einb`'s meta —
and read by no code path. They are port gaps: ein.py's did something, the
surface crossed and the behaviour did not, and no parity tier could have
noticed, because a knob that does nothing produces identical output in both
engines. The claim is held from outside by
`config_reference.rs::the_two_inert_flags_are_still_inert`, and the four
options — wire, delete, document, deprecate — are
[Q-M1e.10](../../plans/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)'s.

---

## 3. The CLI

### 3.1 Where the option list lives

**Not here.** `--help` is the surface and
[`golden/help_shape.txt`](../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt)
is its checked-in rendering — every option's long name, short key, metavar,
arity, default, choices, group and help string, diffed by
`ein-cli/tests/help_surface.rs`, and normative under
[`defined_behaviour.md` § The CLI surface](defined_behaviour.md#5-the-cli-surface).
A third enumeration would be [AR-M1](../../plans/m1e_review_processing/README.md#the-findings)'s
parallel copy. What this page states is the *shape*, read off that golden:
**52** options across eleven parsers, in the default build (`einb`,
`parallel`, `snmalloc`).

| parser | options |
|---|---:|
| `ein solve` | 32 |
| `ein test` | 9 |
| `ein saturate` | 5 |
| `ein render rule` | 2 |
| `ein render lattice` | 2 |
| `ein render rules` | 1 |
| `ein kb save` | 1 |

`ein`, `ein kb`, `ein render` and `ein render constraints` take none — they are
dispatchers. `ein kb save` exists only under the `einb` feature; a
`--no-default-features` build has 51 and no `ein kb` at all.

### 3.2 The six options that shadow a flag

`ein-cli`'s `resolved_config` is the whole of it: six options, five flags, and
each writes in one direction only. There is no `--lookahead`, so an option can
never restore a default the program turned off.

| option | flag | set to |
|---|---|---|
| `-L`, `--no-lookahead` | `enable-pre-branch-lookahead` | `false` |
| `-K`, `--no-kill-cache` | `enable-lookahead-kill-cache` | `false` |
| `-y`, `--lattice-sanity-check` | `lattice-sanity-check` | `true` |
| `-o`, `--lattice-order` | `lattice-order` | its argument (`lex` \| `score-sum`) |
| `-z`, `--shuffle` | `lattice-order-seed` | a fresh seed, echoed to stderr |
| `-d`, `--seed` | `lattice-order-seed` | that seed — **only together with `-z`** |

`--seed` alone does nothing: the assignment sits under `if
m.get_flag("shuffle")`, so `ein solve --dump-config x.ein -d 9` prints
`lattice-order-seed None`. Verify any row of this table the same way — a
`(config …)` head that sets the flag the other way, and `--dump-config` to see
which won.

### 3.3 What an option can do to the answer

Five classes. Only the first two touch the answer.

- **Budget** — `-e`/`--exhaustive`, `-n`/`--solutions`, `-m`/`--max-set-size`,
  `-E`/`--max-enterings`, `-T`/`--max-time`. These change `k` and `exhausted`
  by construction, which is what `Ambiguity`'s *"(a lower bound — the search
  did not exhaust)"* exists to say. **`-n 0` is refused**, exit 2, since M1e
  [S1e.1.5](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.5_cli_semantics.md)
  — and so is every negative, which the CLI used to clamp onto it. It was
  accepted until 2026-08-29 and behaved exactly as `-n 1`: `stop_after` is
  tested *after* a model is recorded, so `Some(0)` cuts at the first one.
  `--jobs 0` had been refused with that argument since S1a.7.5 and `-n 0` was
  not, which was `Q7` / `EH-L1`. **`-m 0` stays legal** and means a truncation
  (§ `defined_behaviour.md` 5), and a **negative** `-m` or `-E` is still
  clamped onto it silently —
  [Q-M1e.17](../../plans/m1e_review_processing/open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative).
  **`-m` is refused under `EIN_TRAVERSAL=tree`** — in all three subcommands
  that take it (`solve`, `test`, `render lattice`) — exit 2, since M1e
  [S1e.2.1](../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md):
  the flag bounds the size of the commitment *sets* the lattice enumerates and
  the tree enumerates none, its depth being what the program owes. It was
  ignored in silence until 2026-08-29, and the reason it is refused rather than
  read as a depth cap is measurable: 6 of `zebra2-minus-15-obligations`'s 32
  models sit at commitment size **6**, one past this flag's own default, so the
  obvious reading would have deleted them at stock settings. `-n`, by contrast,
  is now **honoured** by the tree, which used to record the whole tree while
  being asked for one model.
- **The six shadows** — § [3.2](#32-the-six-options-that-shadow-a-flag). Their
  answer column is the flag's.
- **Execution** — `--jobs N` / `--jobs auto`: same verdict, same models, same
  counters, by [S1a.7.5](../history/m1a_rust/README.md#s1a75--the---jobs-contract)'s
  contract, and pinned by `ein-render/tests/jobs_invariance.rs`. The default is
  1 and stays 1.
- **Presentation** — `-p`, `-P`, `-f`, `-s`, `-t`, `-v`, `-H`, `-c`, `-g`,
  `--layer-progress`, `--models`. stdout and stderr only. `--models key` is
  read by the `Ambiguity` arm alone and reaches nothing recorded — not the
  model list, not `--json-summary`, not `--events`, not `:expect`.
- **Artefacts** — `--events`, `--events-level`, `--json-summary`,
  `--json-report`, `--trace` with its four modifiers (`-F`, `-G`, `-l`, `-R`),
  and `-D`/`--dump-states`. **Additive** in the strict sense: stdout, stderr
  and the exit code are identical with them and without. Four of them name
  **one** path — `events`, `trace`, `json-summary`, `dump-states` — so each is
  refused when the selection asks more than one question; `ein test
  --json-report` deliberately is not, because a report has no run to be more
  than one of.

---

## 4. The environment

Twenty-nine `EIN_*` names are greppable in this tree. Eight are read by the
shipped binary; ten by the test harness; two by `utils/` and the shell; and
**nine are not environment variables at all** — which is the part a grep
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
`SolverConfig`, so none re-blesses a KB-shape golden.

| name | value | what it does | read at |
|---|---|---|---|
| `EIN_STDLIB` | a directory | overrides `std.*` resolution, ahead of the checkout walk and the embedded copy. **Unvalidated** — [`docs/install.md`](../install.md#point-it-at-a-different-stdlib) and [EH-M2](../../plans/m1e_review_processing/README.md#the-findings) | `ein-ir/src/stdlib.rs` `resolve` |
| `EIN_TRAVERSAL` | `tree` | the per-obligation depth-first traversal beside the lattice (M1d S1d.10.6). The one **experimental** surface in the system. Honours `-n`; **refuses `-m`** (§ 3.3) | `ein-infer/src/solve.rs` `tree_traversal` |
| `EIN_OBLIGATION_CHOICE` | `off` \| `fail-first` \| *(default)* `rule-order` | the obligations rung's measurement lever; `off` is the pre-S1d.2.5 engine and the control arm of every number in `hypotheses_from_obligations.md` | `ein-infer/src/oblgen.rs` |
| `EIN_LEFTOVER` | `1` | fills `--json-summary`'s `leftover` block — what the blind enumerator would still propose at each recorded state. Runs on a discarded fork, so nothing else in the summary moves | `ein-cli/src/summary.rs` |
| `EIN_BATCH_PER_WORKER` | a positive integer, default 512 | enterings in flight per worker. Needs the `parallel` feature, which is **on** by default | `ein-infer/src/solve.rs` `batch_per_worker` |
| `EIN_FORK_DELTA` | `0` | the pre-S1a.6.9 fresh-saturator fork path, for `utils/fork_delta_verify.py`'s two arms out of one binary. Needs `--features fork-delta`, **off** by default | `ein-infer/src/solve.rs` `resume_forks` |
| `EIN_FORK_AUDIT` | a file path | one JSON-Lines record per entering. Needs `--features fork-delta`, **off** by default | `ein-infer/src/fork_audit.rs` |
| `EIN_SPEC_AUDIT` | a file path | the speculation audit — every entering re-run against root as of layer start. Needs `--features spec-audit`, **off** by default | `ein-infer/src/spec_audit.rs` |

The first five are live in a stock `cargo install`; the last three need a
non-default build and are inert even when compiled in until the variable is
set. `ein --version`'s `features` line says which build you have.

### Read by the test harness

Not the binary's: these are read by `#[cfg(test)]` code, by a `tests/` target,
or by one of the two dev-only crates (`ein-corpus`, `ein-parity`, both
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
| `EIN_PARITY_STRICT` | `=1` turns off D3's narration normalisation, restoring the byte-identical contract | `ein-parity/src/lib.rs` |

### Read by `utils/` and the shell

| name | what it does | read at |
|---|---|---|
| `EIN_BIN` | which `ein` a script drives; default `ein.rs/target/release/ein`. Every `utils/` script that runs the engine names the binary, through this or through `--bin` | `utils/*.py`, `utils/render_examples.sh` |
| `EIN_RULE_MODE` | `sidebyside` (default) or `overlay`, forwarded to `ein render rules` | `utils/render_examples.sh` |

### Not an environment variable at all

Nine names a grep for `EIN_[A-Z_]*` returns that nothing ever reads from the
environment. Naming them is the point of this section: the census that
produced it was written because *"what does `ein` read from the environment"*
had no answer short of doing this exercise, and a grep's answer would be wrong
by nine.

| name | what it actually is |
|---|---|
| `EIN_RS` | a Python module-level constant — `EIN_RS = Path(os.environ.get("EIN_BIN", …))` in `utils/e2e_baseline.py` and `utils/feature_matrix.py`, and a path constant in `utils/profile_ein_rs.py`. It reads `$EIN_BIN`; nothing reads it |
| `EIN_CMD` | a shell array in `utils/render_examples.sh`, assigned from `$EIN_BIN` |
| `EIN_C_PROBLEM_H` | an include guard in `c/problem.h` |
| `EIN_RENDER_LEVI` | **documented and never implemented.** `ir/03-ein-lang/04_dot_rendering.md` claims it; the only Levi switch is the library-level `DotOpts.levi`. It is one of [CD-H1](../../plans/m1e_review_processing/README.md#the-findings)'s items |
| `EIN_PY` | named in a `run_tests.sh` comment describing the interpreter selection that left with the Python engine. The comment says so; nothing reads it |
| `EIN_RUN_SLOW` | named in three comments in the past tense — a gate ein.py's suite had. Gone with it |
| `EIN_MUTANT` | a frozen measurement's variable in `measurements/baseline.md`, read by the `ein-conformance` harness that M1a S1a.10.3 deleted |
| `EIN_PRIORITY` | *proposed* by [S1f.5.6](../../plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md); does not exist |
| `EIN_NOGOOD_INJECT` | *proposed* by [S1e.1.2](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.2_determinism_under_jobs.md); does not exist |

### What `EIN_TRAVERSAL=tree` reports

The one **experimental** surface in the system (§ 2.3), and the only knob whose
effect a reader cannot infer from the tables above: it does not change a
setting, it runs a **second traversal**. What that traversal *reports* has been
observable since M1d S1d.10.6 and stated nowhere, which is M1e
[S1e.3.2](../../plans/m1e_review_processing/p1e.3_medium/s1e.3.2_semantics.md)'s
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
[S1e.2.1](../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md),
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
- **`EIN_TRAVERSAL` and `EIN_OBLIGATION_CHOICE`.** Both exist to be *compared*,
  and a knob whose settings are being compared would re-bless every shape
  golden in the corpus each time the comparison moved.

And there is **no config file**. The `(config …)` head travels with the
program, which is what makes a `.ein` reproducible on its own.

---

## 6. What pins this page

[`ein-cli/tests/config_reference.rs`](../../ein.rs/crates/ein-cli/tests/config_reference.rs),
six tests, in `cargo test --workspace`. The phase's design constraint is
*generate or diff, never a third hand-maintained copy*, and each of these is
one or the other.

| test | what fails it |
|---|---|
| `the_defaults_block_is_the_binarys_own_dump_config` | the block in § 2.1 is not what `ein solve --dump-config` prints — a new flag, a renamed one, a changed default |
| `every_flag_has_a_row_and_no_row_is_orphaned` | § 2.2's flags are not `FIELDS` in declaration order, or a row's type or default is not the block's |
| `the_cli_counts_come_from_the_golden` | § 3.1's per-parser counts or the total are not `help_shape.txt`'s |
| `the_shipped_environment_set_is_what_the_page_lists` | a shipping crate reads an `EIN_*` name with no row in § 4's first class, or that class lists one nothing reads |
| `the_two_inert_flags_are_still_inert` | flipping `print-alive` or `candidate-order-seed` changes `solve -e -p`'s stdout on either of two fixtures |
| `the_surface_table_agrees_with_the_sections_it_summarises` | § 1's three counts are not the ones §§ 2–4 own |

**Re-banking the block:**

```sh
EIN_BLESS=1 cargo test --manifest-path ein.rs/Cargo.toml -p ein-cli --test config_reference
```

Edit the engine, run that, commit. **Never edit the generated block by hand** —
it is `docs/api/rust.md`'s rule for the same reason, and the five Python pages
beside that one are what happens without it.

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
  the answer column is an answer about: solution, model, `owes`, `exhausted`.
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
