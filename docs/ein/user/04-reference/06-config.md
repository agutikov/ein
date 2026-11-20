# Configuration

Everything that configures a run of `ein`, in three places: a program's own `(config …)` form, the command line,
and the environment. Each knob has a row, and each row answers the question that matters about it: **does it
change the answer?** None changes the region R, the answer `ein` gives
([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)). An option
may ask a different question about R, stop a run early and say so, decline to discover R, change how fast the
answer comes or what is printed beside it; one flag may change which explanation a dead node's core names. What R
is and how to read it is [`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md); what each
command does is [`05-cli.md`](05-cli.md); what a run prints is [`08-read-out.md`](08-read-out.md).

---

## 1. Three surfaces, and the order between them

| surface | how many | where the list is |
|---|---:|---|
| `(config …)` flags | **2** | § [2](#2-config---the-two-solver-flags) |
| CLI options | **99** | `ein <command> --help`, and § [3](#3-the-cli) |
| `EIN_*` environment | **2** read by the shipped binary | § [4](#4-the-environment) |

**Precedence**, highest first:

1. **A CLI option**, but only the one that shadows a flag, § [3.2](#32-the-option-that-shadows-a-flag).
2. **The program's own `(config …)` form.** A program may write several, and the last one is kept **whole**, not
   merged: `(config :record-alternative-justifications false) (config :lattice-sanity-check true)` leaves
   `record-alternative-justifications` at its default. An imported module's `(config …)` is not brought by the
   import ([Q-M1f.215](../../../decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md)): the
   flags are the program's own.
3. **The defaults**, § [2.1](#21-the-defaults).

`ein solve --dump-config` (`-c`) prints the **resolved** result of the three before the read-out, and is the way to
see what a run used.

Two things are deliberately outside that order. There is **no configuration file** and no `$EIN_CONFIG`. And **no
environment variable reaches a `(config …)` flag**: the two the binary reads (§ 4) change where the standard
library is found and what one block of `--json-summary` holds, nothing else.

---

## 2. `(config …)` — the two solver flags

```lisp
(config :record-alternative-justifications false :lattice-sanity-check true)
```

A flag is a `:kebab-case` keyword followed by a bool, `true` or `false`, bare. The form takes those two keywords
and no other ([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)), so an
unknown flag, a retired flag and a value that is not a bool are each a **parse error** at their position, with the
repairs the parser found: the load error every command gives such a program, exit 1 (`ein test`: 2, the file an
`ERROR` row; [`07-diagnostics.md`](07-diagnostics.md)). The lines `ein saturate` writes for two of them, each in a file
`c.ein`:

```text
c.ein:1:9: unexpected `:nonsense`: a keyword this form does not take
  repair: Insert `:lattice-sanity-check`, Delete `:nonsense`
  repair: Insert `:record-alternative-justifications`, Delete `:nonsense`
  repair: Delete `:nonsense`, Delete `true`
c.ein:1:31: unexpected `7`: a name
  repair: Insert `false`, Delete `7`
  repair: Insert `true`, Delete `7`
```

A **retired** flag's line carries one `note:` more, which says that the flag was retired and why; the reasons are
§ [2.4](#24-the-fifteen-retired-flags)'s table.

### 2.1 The defaults

`ein solve --dump-config` on a program with no `(config …)` form. The block is generated from the binary's own
output and is never edited by hand:

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
| `lattice-sanity-check` | `bool` | `false` | `-y`'s, § [3.2](#32-the-option-that-shadows-a-flag): the parent-path spot check, **unbounded and on every program**. Every one-step parent of every printed state of two commitments or more is built and saturated again, and the check says whether each reaches the same state. On a program the static check flags non-monotone it fills the warning's `paths` row and never fails the run; on a monotone one a parent path that reaches another state contradicts monotonicity itself, and the run is an engine error, exit 2. A clean check prints its `paths` row, and the saturations it costs are that row's, outside `work` | no | no | check |
| `record-alternative-justifications` | `bool` | `true` | whether a re-derivation of a fact already known is recorded as an alternative justification or dropped: the proof is an AND/OR graph, or a tree. It may change which core a dead node's refutation names — the read-out's `unsat core` and `source` rows, the summary's `region.core`, the trace's `N` and the answer directory's `nogoods.json` — and with it whether that core's search was cut (the `core search` row, `core_status`) and, where the core moves, the rules its derivation ends in (`refuted by`, `region.refuted_by`), **and nothing else**: R, every state and every verdict word stay ([Q-M1e2.11](../../../decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md); § [2.5](#25-the-config-principle-read-against-every-row)). Off, a core is the smallest over each fact's first derivation; on, over every derivation the program records. `examples/zebra.ein` turns it off, for speed, and says why | explanation only | no | contract |

### 2.3 Reading the judgement columns

**Two questions about the answer.** The answer is the region R
([`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md)), and every `ein solve` run reports
it, or a question about it.

- **`R?`** asks whether the flag changes R: the candidates, any node's status or state class, root's fixpoint, or
  the facts of any state in R. Under the config principle (§ [2.5](#25-the-config-principle-read-against-every-row))
  every cell reads **no**.
- **`answer?`** asks whether the flag changes what a run **prints**: the read-out's rows, its states and their
  models, its queries' classes and its claims' outcomes.

| value | means |
|---|---|
| **yes** | can change R's description, a state or a class |
| **explanation only** | the same R and states; a different unsat core or proof shape |
| **no** | the same read-out; the work counters and the wall clock may move |

**No flag is unread**: each of the two changes something a run prints or checks, and the fifteen that changed
nothing, or changed R, are retired (§ [2.4](#24-the-fifteen-retired-flags)).

**Stability**, what depending on a flag commits you to:

| value | means |
|---|---|
| **contract** | stated here, exercised by the corpus and the test suite, safe to depend on |
| **check** | asks for more work and moves no answer: its own row, and on a failure an error, never a different answer. The test suite runs it on every corpus program and holds every question's read-out and summary unmoved under it but for its own row |

### 2.4 The fifteen retired flags

Each is refused at the parse with a `note:` that says why it went (§ [2](#2-config---the-two-solver-flags)), so a
program that sets one does not load. A `.einb` container stores its program as text and loads it again when it is
opened, so a container whose program sets a retired flag is refused where its `.ein` is.

| flag | why it went |
|---|---|
| `warn-derived-naf` | it switched advice on a rule that reads `(absent …)`. What it asked is the static check's warning, printed beside every answer that reports R on a program it flags, whatever the configuration says ([Q-M1f.564](../../../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) |
| `enable-pre-branch-lookahead` | a one-step pre-filter of candidates, which no route runs: every route saturates each candidate's own node |
| `enable-lookahead-kill-cache` | it wrote the pre-filter's kills into root, which no route does: a candidate dead alone leaves the universe, and nothing is written |
| `enable-path-nogoods` | learned clauses: every route reads N, the minimal dead sets, by construction |
| `enable-fail-fast-fork` | a node is never stopped at its first contradiction: a dead node's debts and its core are its fixpoint's |
| `enable-symmetric-mirror` | it switched the kernel's native mirror, and the kernel mirrors no relation: a relation closed under swapping its two arguments is `std.algebra`'s `(symmetric R)`, a rule |
| `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight`, `lattice-order`, `lattice-order-seed` | order levers: every route's order is part of its definition, so `-n N` prints the same states whatever the order would have been |
| `enable-singleton-writeback`, `enable-forced-positive` | they wrote into root, which no route does: a candidate dead alone leaves the universe |
| `print-alive`, `candidate-order-seed` | inert: no code read them ([Q-M1e.10](../../../decisions/language/Q-M1e.10-two-inert-config-flags-are-refused.md)) |

### 2.5 The config principle, read against every row

*No configuration option affects the problem's lattice; configuration may only affect the speed of the search*
([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)). The answer is
the region R, and it is the same under every knob on this page. A **budget** (`--region-budget`, `--region-limit`,
§ [3.3](#33-what-an-option-can-do-to-the-answer)) does not change R either: it stops a run, and the read-out then says
the answer is partial. `-T` aborts a run and prints no answer. A knob that changed R, or changed a reported answer
that does not say it is partial, would be a wrong answer and not a mode, and is retired (§ 2.4).

- **R.** The oracle's universe is built with no configuration, and its nodes by saturation alone
  ([`11-region-dump.md`](11-region-dump.md)), so no flag reaches it.
- **What a run prints.** The test suite runs every flag a `(config …)` can set, flipped from the program's own
  value, on every corpus program whose R is known, under each of `ein solve`'s questions, and compares the read-out
  uncut, its stderr lines, its exit code and the summary with the program's own; it runs every lever the corpus
  declares through the binary too. Nothing moves, but for the exemption below.
- **One exemption: the core under `record-alternative-justifications`**
  ([Q-M1e2.11](../../../decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)). Off, the flag
  drops every re-derivation, the proof is a tree, and the explanation search is offered each fact's first derivation
  alone, so the core it names can be another, and whether its search was cut can change with it. That is all it may
  change: R, every state and class and every verdict word stay, and `refuted by`, which names the first derivation
  the core supports, moves only where the core does. On
  [`examples/ein-bugs/alt-cap-core.ein`](../../../../examples/ein-bugs/alt-cap-core.ein) the run reports
  `(w1 X) (w2 X)` refuted by `wide` with the flag on, and `(p1 Y01) (p2 Y01) (p3 Y01)` refuted by `narrow` with it
  off. The core's promise is over the derivations the program records (*smallest contradiction frontier*,
  [`13-glossary.md`](13-glossary.md)). A core is an explanation, not the answer
  ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).
- **`lattice-sanity-check` is a check.** On a program the static check flags, every run takes the parent-path spot
  check bounded, a few one-step parents of each printed state, in the warning's `paths` row; `-y` takes it unbounded
  and on every program. It answers nothing and moves nothing, and on a monotone program a mismatch is an engine
  error, never another answer.

Under a run of `ein solve` every flag reads **no**, and so do `--jobs` (§ 3.3) and every presentation and artefact
option: they change speed, or what is printed beside the answer, never the answer. `--no-search` declines to
discover R and states none: a run that searches reports the R it declined to (§ 3.3). `--no-require-why` (§ 3.4) is
not a lever: it decides whether a program loads, and a program loads to what it loads to with it or without it.

---

## 3. The CLI

### 3.1 Where the option list lives

**`ein <command> --help`.** Each option's long name, short key, metavar, arity, default, choices, group and help
string is fixed, and the test suite holds the help of every command to a checked-in rendering of it; what is free
is `clap`'s layout ([Q-M1a.13](../../../decisions/cli/Q-M1a.13-the-clis-behaviour-is-fixed-and-its-helps-layout.md),
[`05-cli.md` § 5](05-cli.md)). What this page states is the *shape*: **99** options across eighteen parsers, in the
default build (the `einb`, `parallel` and `snmalloc` features, which `ein --version` names).

| parser | options |
|---|---:|
| `ein solve` | 36 |
| `ein test` | 12 |
| `ein oracle` | 11 |
| `ein saturate` | 6 |
| `ein render h-space` | 6 |
| `ein render kb` | 6 |
| `ein render lattice` | 4 |
| `ein render rule` | 4 |
| `ein render region` | 3 |
| `ein render program` | 3 |
| `ein render rules` | 3 |
| `ein kb save` | 2 |
| `ein render constraints` | 2 |
| `ein show hyps` | 1 |

`ein`, `ein kb`, `ein render` and `ein show` take none: they dispatch. `ein kb save` exists only in a build with the
`einb` feature; a build without it has no `ein kb`, and two options fewer. `ein show hyps` takes one,
`--no-require-why`, and nothing that could move what it lists (§ 3.3). `ein render h-space` takes six: `--view`,
`--dump`, `--supports`, `--region-budget`, `-j`/`--jobs` and `--no-require-why`.

**Eleven options of `ein solve` are retired.** Each is still recognised, and refused, exit 2, with what answers its
question instead:

| retired | answered by |
|---|---|
| `-e`, `--exhaustive` | a bare `ein solve`, which describes all of R; `--maximal` for the maximal line |
| `-m`, `--max-set-size` | `--minimal`, the least commitments and their sizes; `--region-budget FACTS` for the work. A depth cap changed answers |
| `-E`, `--max-enterings` | `--region-budget FACTS`, which stops a run with what it established |
| `-o`, `-z`, `-d` | nothing: each route's order is part of its definition, so no lever changes which states are printed |
| `-D`, `--dump-states` | `--dump-answer DIR` for each state, `--dump-lattice DIR` for each node visited |
| `--dump-region`, `--region-states` | `ein oracle FILE --dump DIR [--states]` |
| `-L`, `-K` | nothing: no route runs the one-step lookahead or writes a kill into root, so each changed nothing |

`ein test` refuses `-m` and `-E` the same way, and runs within `--region-budget`; `ein render lattice` refuses
`--max-set-size`, and its `--region-budget` bounds the run it draws.

### 3.2 The option that shadows a flag

One option, one flag, written in one direction only:

| option | flag | set to |
|---|---|---|
| `-y`, `--lattice-sanity-check` | `lattice-sanity-check` | `true` |

`-y` changes no answer and adds its own `paths` row; the header line that echoes a run's options leaves it out, as
it leaves out `-j`. A `(config :lattice-sanity-check true)` form sets the same flag, and `--dump-config` shows it.

### 3.3 What an option can do to the answer

Eleven classes. Only the first touches the answer, and then only by asking a different question about R or by
stopping early and saying so; the second declines to ask about R at all; the third chooses how a description finds
R and never R; the fourth and fifth ask about the hypothesis set and not about R; the last decides whether there is
a program to answer about.

- **The question and its budget** — `--minimal`, `--maximal`, `--single`, `-n`/`--solutions`, `--count`,
  `--monotone`, `--region-budget`, `--region-limit`, `-T`/`--max-time`. The first five choose what is asked about R
  ([`05-cli.md` § 2.1](05-cli.md)). `--count` is over all of R only: a description's route, R's two counts and the rows
  that qualify them, no state read, so no query is answered and no claim checked, which one `claims` row and a
  `note:` per claim say. Beside an option that asks what it does not answer (a line, `-n`, `--single`,
  `--monotone`, a listed state's form, `-H`, `--trace`) it is a usage error, exit 2, naming the command that answers
  it. `--monotone` asks about the program instead: root's fixpoint and the static check, no node built, every
  verdict exit 0, and exclusive with the others. The budget (`--region-budget`, default $2^{20}$, 1 048 576, facts beyond
  root for every question, [Q-M1e2.35](../../../decisions/cli/Q-M1e2.35-the-default-region-budget-is-2-20-facts.md);
  `ein solve --help` states it), the cube's limit and the deadline stop a run. A budget or a limit that stops one
  leaves every surface saying the answer is partial: a lower bound, a `note:` on stderr, `exhausted no` and why. A
  `-T` abort exits 2 and prints no answer, because a machine-dependent limit is never an answer. **`-n 0` is
  refused**, exit 2, and so is every negative
  ([Q-M1e.24](../../../decisions/cli/Q-M1e.24-n-takes-a-count-of-one-or-more.md)).
- **The question not asked** — `--no-search`, on `ein solve` and `ein test`. **Does it change the answer: no — it
  declines to discover it** ([Q-M1f.258](../../../decisions/cli/Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md)).
  It saturates root and reports root, alive or dead, discharged or what it owes, and builds no other node: `ein
  solve` prints root's standing, an `R` row that says R was not searched and no row that counts or lists R
  ([`08-read-out.md`](08-read-out.md)), and `ein test` decides only what root decides, the rest NOT CHECKED with no exit
  code of its own ([Q-M1f.256](../../../decisions/answer/Q-M1f.256-under-no-search-a-claim-is-decided-as-far-as.md),
  [Q-M1f.257](../../../decisions/cli/Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md)). A run without it
  discovers the R it declined to, and what it reports of root is what that run reaches there. Beside a question
  flag (`-n`, `--single`, `--minimal`, `--maximal`, `--count`) or `--monotone` it is a usage error, exit 2, and `ein
  oracle` refuses it with the command that answers. The program's own `(solve :no-search)` is no option: it states
  the lattice, root alone, and so R ([`09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md)).
- **A description's route** — `--describe-route tree|states`, on `ein solve`, for a bare run and `--count` on a
  program the static check calls monotone; `ein oracle`, the other questions and a flagged program take no other
  route for it, and beside a question flag, `--monotone` or `--no-search` it is a usage error, exit 2. **Does it
  change the answer: no — R is the same; the state route alone computes `N` and an empty R's owing frontier where
  the tree branches** ([Q-M1f.311](../../../decisions/answer/Q-M1f.311-describe-takes-the-tree-and-the-classes.md)).
  `tree`, the default, takes R's states by the obligation tree and each state's class inside it, where root owes an
  instance a licence takes; elsewhere the tree has nothing to branch on and the state route is taken. `states` takes
  the state route, every alive state once. Where the tree branches, the read-out's `N` row says `N` is not computed,
  with the candidates dead alone as the members met, and the summary writes `null`; an empty R's owing frontier is
  likewise not computed, and a region claim over it is not checked by that run (`ein test` asks the state route
  then). Every other field — R's states, `k_c`, the lines, the shape, the candidates dead alone — is the same by
  both. The work differs, and so does where a budget stops each, which the `exhausted` row and the stderr `note:`
  say, with the forecast and the other route.
- **The hypothesis space** — `ein render h-space`'s `--view`, `--dump`, `--supports`, `--region-budget` and
  `-j`/`--jobs` ([hypotheses and the answer, in code § 2.3](../../ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md#the-options)). **Does
  it change the answer: no.** The command asks about the hypothesis set H and not about R, and `ein solve` computes
  none of it ([Q-M1f.309](../../../decisions/cli/Q-M1f.309-ein-render-h-space-shows-the-hypothesis-space.md)).
  `--view` chooses what stdout holds: the read-out, one graph as DOT, or the document; `--dump DIR` writes the
  document and the four graphs and leaves stdout and stderr as they were; `--supports` reads more, the stated facts
  each excluded pair's death rests on, and the groups, the grids and the components are the same with it;
  `--region-budget` bounds root's pair layer and the small KB's, each, and a layer that reaches it exits 0 and is
  said to be stopped, with what was not read, in the document, in the read-out and as a `note:` on stderr; `--jobs`
  moves no byte.
- **The hypothesis set, listed** — `ein show hyps`, which takes no option of its own. **Does it change the answer:
  no.** It saturates root, runs the generator at root's fixpoint and prints the candidates, one fact a line; it
  reads no node, and no option, `(config …)` flag or environment name moves what it lists. Its lines are the
  candidates `ein solve -H` lists and the document of `ein render h-space` lists.
- **The check** — `-y`, § [3.2](#32-the-option-that-shadows-a-flag): its own row, and on a monotone program a
  mismatch is an engine error, exit 2; never a different answer.
- **The check in flight** — `--in-flight`, on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`.
  **Does it change the answer: no** ([Q-M1f.656](../../../decisions/cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md)).
  The pass that writes a state's `open` facts runs at root, where it makes H, and above root only at a state that
  is printed or saved, or that reads them, every state of a program a rule of which reads an `open` fact
  ([Q-M1f.655](../../../decisions/engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)); a state's
  verdict, what it owes, is read at every state either way. `--in-flight` runs the pass at every state the run
  builds, and checks at each whether it holds a hypothesis root does not, which makes the program not monotone,
  found in flight ([Q-M1f.574](../../../decisions/answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)):
  what that check finds is the one thing it can add, the `in flight` row, its stderr line and the summary's
  `monotonicity.in_flight`. R, every printed state, every count and every other row are the same with it and
  without; it costs the pass at every state.
- **Execution** — `-j`/`--jobs N`, or `--jobs auto`: the same read-out, byte for byte, at every job count
  ([Q-M1a.7](../../../decisions/engine/Q-M1a.7-a-job-count-moves-nothing-a-run-prints.md)). The default is 1.
- **Presentation** — `-p`, `-P`, `-f`, `-s`, `-t`, `-v`, `-H`, `-c`, `-g`, `--layer-progress`, `ein test`'s `-v`,
  `-q` and `--diff`, the options of the views that draw a file and solve nothing, `ein render program`'s, `kb`'s,
  `rules`', `rule`'s and `constraints`', `--fold` among them
  ([Q-M1f.419](../../../decisions/cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)), and `kb`'s
  `--goal` and `--query`, which draw the slice of a goal, the facts of the knowledge base drawn that answer it, and
  match it against that knowledge base and never against R
  ([Q-M1f.529](../../../decisions/cli/Q-M1f.529-ein-render-kb-draws-the-slice-of-a-goal.md)), and `ein render region`'s three, which draw an `ein oracle
  --dump` directory (`--view`) with a solve run's `--dump-lattice` and `--dump-answer` on it (`--visited`,
  `--answer`) and load nothing. They change stdout and stderr only, and `--diff` the `diff` of `--json-report`'s
  rows. Under `-v`, `-g N` logs every N-th node; `-g 0`
  logs none, and a negative is refused, exit 2
  ([Q-M1e.17](../../../decisions/cli/Q-M1e.17-g-0-logs-no-node-line-and-a-negative-is-refused.md)). **Four of them
  are forms printed after the read-out, and share its 100 lines**
  ([Q-M1e2.34](../../../decisions/read-out/Q-M1e2.34-stdouts-listings-share-100-lines-and-a-dump.md)): `-H`, `-p`,
  `-P` and `-f` are drawn from the states stdout showed and print what fits, at least one item, then
  a line naming the rest; that rest is `--dump-answer DIR`'s `forms/<form>.txt`, uncut and drawn from every listed
  state ([`10-answer-dump.md`](10-answer-dump.md)). **`ein test --diff` compares each claim's models with what its
  run found, and decides nothing** ([`05-cli.md` § 2.3](05-cli.md#23-ein-test)): every verdict and the exit code are
  the same with it and without. It pairs the models it did not match pairwise, and **prices that table before it
  allocates it**, against a fixed bound of 16 MiB that is not an option: past the bound it pairs nothing near, seeks
  no nearest, says so in its block, and counts every unmatched model missing and every unmatched found one extra.
- **Artefacts** — `--events`, `--events-level`, `--json-summary` (`ein-summary/2`), `--json-report`
  (`ein-test-report/6`), `--trace` with its five modifiers (`-F`, `-G`, `-l`, `-R`, `--fold`), `--dump-lattice`
  ([`docs/ein/ein.rs/06-lattice-dump.md`](../../ein.rs/06-lattice-dump.md)), `--dump-answer`
  ([`10-answer-dump.md`](10-answer-dump.md)), and `ein oracle`'s `--dump` with `--states`
  ([`11-region-dump.md`](11-region-dump.md)). The directories are made before the run. **Additive** in the strict sense:
  stdout, stderr and the exit code are identical with them and without, and a write that fails after the run
  reports itself and leaves the answer's exit code
  ([Q-M1e.22](../../../decisions/cli/Q-M1e.22-a-failed-artefact-write-keeps-the-runs-exit-code.md)). A run answers
  every `(query …)` of its file from one solve; `ein test` refuses `--events` and `--json-summary` over a selection
  of more than one file, and where it asked a file two questions they hold the last one.
- **Loading** — `--no-require-why`, § [3.4](#34-the-option-that-decides-loading): without it a program missing a
  `:why` is a load error, and with it the program loads; one that carries its `:why`s answers exactly the same
  either way.

### 3.4 The option that decides loading

One option, on every command that loads a program — `ein solve`, `ein oracle`, `ein test`, `ein saturate`, `ein
render kb`, `ein render lattice`, `ein render h-space`, `ein render program`, `rules`, `rule`, `constraints`, `ein
show hyps` and `ein kb save` — and on none that loads nothing (`ein render region`). The views that draw a program
as written read it through the loader first, and refuse what it refuses
([Q-M1f.71](../../../decisions/cli/Q-M1f.71-a-view-refuses-a-program-the-loader-refuses.md)), so they take it too. A
program that embeds Ein sets the same option through the library ([`docs/ein/ein.rs/04-api.md`](../../ein.rs/04-api.md)).

| option | default | what it changes | answer? | R? | stability |
|---|---|---|---|---|---|
| `--no-require-why` | off: a `:why` is required | whether a program loads. Off, every `(relation …)`, `(rule …)` and `(macro …)` the load reads, the standard library's and an imported module's included, that carries no `:why "…"` is refused, one line per form, `relation 'p' has no :why "…", which this load requires of every relation, rule and macro`, at the exit code the command gives every load error (1, and `ein test`'s 2); on, such a program loads. A `:why` that is not a string is refused at the parse either way. A `.einb` is refused where its `.ein` is: a container loads its program again from the text it stores. The standard library loads without it: every rule and macro it holds carries a `:why` | **no**: it decides whether a program loads, never what a loaded one answers ([Q-M1f.62](../../../decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)). A program that carries its `:why`s loads to the same knowledge base with it and without it, and prints the same bytes; the header leaves it out | no | contract |

---

## 4. The environment

Two `EIN_*` names are read by the shipped binary, and neither reaches a `(config …)` flag. That is the whole list:
a variable this section does not name changes nothing `ein` does.

### Read by the shipped binary

| name | value | what it does |
|---|---|---|
| `EIN_STDLIB` | a directory | overrides where `std.*` modules are found, ahead of the checkout and the copy built into the binary, `std.core` among them: the library's `core.ein` is the module every program gets ([`04-stdlib/21-core.md`](04-stdlib/21-core.md#replacing-it)), and a library with none refuses every program, *std.core was not found in $EIN_STDLIB*. The directory **must carry `MANIFEST.sha256`**, the file that identifies a standard library, and the refusal names the variable at the first `std.*` import ([`01-install.md`](../01-install.md)). `ein --version` reports a directory it cannot prove as `unreadable` rather than refusing |
| `EIN_LEFTOVER` | `1` | fills `--json-summary`'s `leftover` block: what the blind enumerator would still propose at each **listed state**, outside H. The block has three keys on every summary: `taken`, whether the variable was `1`; `states`, the count at each listed state, in `region.states`' listed order; and `states_by_relation`, that count split by relation, one map per listed state from a relation's name to its count. Both lists are empty when `taken` is false. It is computed with no configuration, as H is, on a copy of each state, so nothing else in the summary moves |

### No other name

No other `EIN_*` variable is read by the binary. In particular none reaches a `(config …)` flag, there is no
variable that names a configuration file, and nothing chooses a route, an order or a traversal from the
environment.

---

## 5. What is deliberately *not* configuration

Three knobs are command-line options and not `(config …)` flags, and the reason is the same each time: **a
`(config …)` flag travels with the program**, is printed by `--dump-config`, and makes a `.ein` file reproducible on
its own.

- **`--jobs`.** A flag would let a *program* set a thread count, a `.ein` that reads differently on an eight-core
  machine than on a four-core one. A job count is how a run is executed; a flag is what the program means. It moves
  nothing a run prints either way ([Q-M1a.7](../../../decisions/engine/Q-M1a.7-a-job-count-moves-nothing-a-run-prints.md)).
- **The presentation options**, `-p`, `-H` and the others of § [3.3](#33-what-an-option-can-do-to-the-answer): how the
  answer is drawn is not part of the program.
- **`--no-require-why`** (§ 3.4). A flag would let a program turn its own requirement off; it is a question about
  loading, and the program is its subject.

And there is **no configuration file**. The `(config …)` form travels with the program.

---

## 6. What holds this page

The test suite holds each mechanical part of this page to the binary, so that what is listed cannot drift from
what ships:

| part | what fails it |
|---|---|
| the defaults block, § 2.1 | the block is not what `ein solve --dump-config` prints: a new flag, a renamed one, a changed default |
| the flag table, § 2.2 | its flags are not the solver's, in its order, or a row's type or default is not the block's |
| the option counts, § 3.1 | a parser's count or the total is not the checked-in help's |
| the environment, § 4 | the binary reads an `EIN_*` name with no row under *Read by the shipped binary*, or that class lists one nothing reads |
| the `EIN_LEFTOVER` row | it does not name every key of the `leftover` block, or the block is not what the row says, with the variable and without |
| `-y` and the sanity flag, § 3.2 | `:lattice-sanity-check true` moves `solve -p`'s exit code, stdout or stderr past its own `paths` row, or prints no unbounded row |
| the retired flags, § 2.4 | a program setting one loads, or is refused without saying why |
| `--no-require-why`, § 3.4 | the row is missing; the option is on another set of commands than the thirteen that load a program; a program missing three `:why`s loads without it, is refused at another exit code or without its three lines, or does not run with it; a container is not refused where its `.ein` is; or a corpus program prints other bytes with it |
| the summary table, § 1 | its three counts are not the ones §§ 2–4 own |

The defaults block is generated and never edited by hand. The prose columns are not, and cannot be: *what it
changes* is read off the engine, and *does it change the answer* is a judgement with evidence. What the checks give
them is a **shape** guarantee: every flag has a row, no row is orphaned, and the mechanical columns beside the prose
agree with the binary.

---

## See also

- [`05-cli.md`](05-cli.md) — every command, the questions it answers, and its exit codes.
- [`08-read-out.md`](08-read-out.md) — every row `ein solve` prints, the `paths` row and `--json-summary`'s blocks among
  them.
- [`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md) — what the two answer columns are
  about: the region R.
- [`07-diagnostics.md`](07-diagnostics.md) — the wording of a refusal, and every load error.
- [`01-install.md`](../01-install.md) — `$EIN_STDLIB`, how a standard library is found, and `ein --version`'s lines.
