# The command line

`ein` is one binary. A program goes in, as a `.ein` file or a `.einb` container saved from one; what comes out is
the program's answer, the region R of its solutions
([`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md)), or a question about R, or a drawing
or a listing of the program. This page says which command answers which question, what each guarantees, and what
its exit code means. Every option and what it can do to the answer is [`06-config.md`](06-config.md); every row a run
prints is [`08-read-out.md`](08-read-out.md); every refusal is [`07-diagnostics.md`](07-diagnostics.md).

| command | answers | what it costs |
|---|---|---|
| **`ein solve FILE`** | a question about R, chosen by flags (§ [2.1](#21-ein-solve)); bare, it describes R | the least work the question needs |
| **`ein oracle FILE`** | R by brute force: every one of the 2ⁿ commitments saturated (§ [2.2](#22-ein-oracle)) | 2ⁿ saturations, always |
| `ein test PATH…` | the claims programs state about themselves (§ [2.3](#23-ein-test)) | per file, the least question that decides its claims |
| `ein saturate FILE` | root, the node ∅: the program's least fixpoint (§ [2.4](#24-ein-saturate)) | one saturation |
| `ein render VIEW …` | a drawing, as DOT (§ [2.5](#25-ein-render-and-ein-show)) | what the view reads |
| `ein show hyps FILE` | the hypothesis set H, as Ein facts (§ [2.5](#25-ein-render-and-ein-show)) | root's saturation and the generator |
| `ein kb save FILE OUT` | a `.einb` container of the program (§ [2.6](#26-ein-kb)) | one load |

`ein --version` prints five lines, the engine, the event protocol, the container format, the build's features and
the standard library it resolves ([`01-install.md`](../01-install.md)); `ein --help` and `ein <command> --help` list the
options.

---

## 1. The contract

Four rules, held for every question and every program, monotone or flagged by the static check. *The oracle's*
means what `ein oracle FILE` prints for the same program
([Q-M1e2.32](../../../decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)).

| # | rule | what it rules out |
|---|---|---|
| **C1** | **Every state a run prints is a member of the oracle's R.** It is printed *as* minimal or maximal only when it is on that line of the oracle's R | a superset printed as a solution; a state printed as maximal while an alive extension exists |
| **C2** | **A claim printed as certified equals the oracle's**: k_s, k_c, a line, *one state*, *none* | a certified line with one of its states missing |
| **C3** | **A run a budget stopped prints bounds, never a different answer**: k_s ≥ …, a line ⊇ what it printed, no verdict word, `exhausted no` | *no solution* said of a region that holds every commitment of its cube |
| **C4** | **No lever changes what is printed**, work counters aside, and that includes *which* states `-n` prints ([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)) | an option that changes R, or picks a different witness |

**Why it holds whether or not the program is monotone.** Every question evaluates a commitment the way the oracle
does: root's fixpoint, the commitment added, saturation alone, and nothing written back into root. A question and
the oracle therefore cannot disagree about a node; they can only visit different nodes. A question skips a node, or
concludes a line, only through a **licence** the program's static class proves, and a program the static check flags
gets none beyond the cube: its questions are certified up to `--region-limit` and partial past it, and C1–C3 still
hold. How each question is computed, and the licences, are
[`docs/ein/reasoning/02-kb-evolution/07-finding-r.md`](../../reasoning/02-kb-evolution/07-finding-r.md)'s. Each question does the least work
it needs, and the whole cube is computed only when `ein oracle` is asked for it by name
([Q-M1e2.31](../../../decisions/answer/Q-M1e2.31-each-question-about-r-does-the-least-work-it.md)).

---

## 2. The commands

### 2.1 `ein solve`

```text
ein solve FILE  [WHICH]  [ASK]  [--region-budget FACTS] [--region-limit N] [-T SECONDS] [-j JOBS]
                [presentation] [artefacts]

WHICH   (all of R, the default) | --minimal | --maximal
ASK     (describe, the default) | -n N | --single | --count (over all of R only)
```

**Two axes**, which part of R and what to ask of it:

| | **describe** (default) | **`-n N`** | **`--single`** |
|---|---|---|---|
| **all of R** (default) | R in full: k_s, k_c, both lines, components, maximal sub-cubes, merged classes, `N` | N states of R | is k_s = 1? |
| **`--minimal`** | the minimal line: its commitments, their states, their sizes | N of them | is the minimal line one state? |
| **`--maximal`** | the maximal line: the states nothing can be added to | N of them | is the maximal line one state? |

- **`-n N` prints the same N states under every lever and every `-j`** (C4). Which N is fixed by the route's own
  order, not by an option. `-n 1` is *any*. `-n` takes a count of one or more
  ([Q-M1e.24](../../../decisions/cli/Q-M1e.24-n-takes-a-count-of-one-or-more.md)).
- **`--single`** answers in one of four words: **`Solution`**, one state; **`Ambiguity`**, two or more, and it
  prints two of them; **`Contradiction`**, none, or **`Open`** when every alive node still owes. A run its budget
  stopped before it decided says so. Every word exits 0
  ([Q-M1e2.33](../../../decisions/cli/Q-M1e2.33-every-answer-exits-0.md)): a program that requires one solution
  states it as a claim, and a refuted claim exits 1.
- **`--minimal` and `--maximal`** exclude each other, and so do `-n` and `--single`.
- **`--count`** asks how many states and commitments R holds: a description's route, k_s and k_c with the rows that
  qualify them (`exhausted`, `∅`, `lattice` with a flagged program's warning, `universe`, `route`, `work`), and
  nothing listed. No state is read, so no query is answered and no claim checked, and one `claims` row says so. It
  is over all of R only: beside `--minimal`, `--maximal`, `-n` or `--single` it is a usage error that names the
  command that answers the other question. A count is exact, and is reported up to 2⁶³ − 1, which every surface
  reads as *this many or more* ([Q-M1f.218](../../../decisions/answer/Q-M1f.218-a-count-is-exact-and-reported-up-to-2-63-1.md)).
- **`--monotone`** asks about the program instead of R: root's fixpoint and the static check, the sites that make
  the program not monotone and what each puts at stake, no node built. Every verdict exits 0, and it excludes the
  questions above.
- **`--no-search`** asks no question about R: it saturates root and reports it, alive or dead, discharged or what it
  owes, states no R, and decides only the claims root decides
  ([Q-M1f.258](../../../decisions/cli/Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md); [`06-config.md`
  § 3.3](06-config.md)).
- **What every run prints beside its answer** is the route it took and the licences it used, whether the run
  `exhausted` its question, and the work done, in saturations and facts ([`08-read-out.md`](08-read-out.md)). A run the
  budget stopped prints what it established and bounds for the rest, and a `note:` on stderr says how far it got,
  what the next step would cost, and which option takes it further.

The options beside the two axes — the budgets, the presentation forms, the artefacts — are listed by
`ein solve --help` and classed by what they can do to the answer in [`06-config.md` § 3.3](06-config.md).

### 2.2 `ein oracle`

```text
ein oracle FILE  [--region-limit N] [--dump DIR] [--states] [-j JOBS] [-T SECONDS]
                 [-s] [--json-summary FILE] [--models FORM] [-p] [-P]
```

- **What it does:** saturates every subset of H, the program's candidates, with no licence and no pruning. It
  prints `ein solve FILE`'s description, in the same layout, plus what only the cube has: the holes, and the static
  verdict checked against the cube.
- **Why it is a command of its own:** it is the reference every question is held to. It costs 2ⁿ saturations on
  every program, including those where a question needs one, so it runs only when asked by name.
- **`--region-limit N`** (default 16): above it the command refuses, exit 2, naming the cube's size and the limit
  that would compute it. Passing a larger N is the request; from 17 the command warns, and from 20 it warns with
  the projected cost.
- **`--dump DIR`** writes `region.json` and `region.md` ([`11-region-dump.md`](11-region-dump.md)), which
  `ein render region DIR` draws, and `--states` keeps every node's state beside them.
- `ein oracle` takes no `--no-search`: it refuses it with the command that judges root alone.

### 2.3 `ein test`

```text
ein test PATH…  [-v | -q] [--region-budget FACTS] [-T SECONDS] [--no-search]
                [--json-report FILE] [--json-summary FILE] [--events FILE]
```

`ein test` turns a selection of programs into a status code, so that **nothing reads output**. Each `PATH` is a
`.ein` file, or a directory walked for them. A program states what it expects of its own answer: a `(query …
:expect …)` about a goal's classes, or a `(solve … :expect …)` about R
([`12-query.md`](../03-syntax-and-semantics/12-query.md)). `ein test` checks every such claim and prints one line a file:

```text
$ ein test examples/features/
(no expect)  examples/features/01_not_and_absent.ein
…
ok           examples/features/10_expect.ein
…
17 files, 19 expectations: 19 held, 0 FAILED, 0 not checked, 0 errors  (… s); 9 files state no expectations
```

A file is `ok` when every claim it states holds, `FAILED` when one is false, `NOT CHECKED` when one could not be
decided, `ERROR` when it did not load or a run of it was aborted, and `(no expect)` when it states no claim. A
`FAILED` line is followed by the claim's disagreements, the same block `ein solve` prints under the claim.

Four things about it are decisions rather than implementation:

- **It asks a file the least question that decides the claims it states**, read off their shapes: `--single` or
  `-n k+1` where the queries' claims are all over one part of R, a description for a `(solve … :expect …)` or for
  claims over more than one part; what that leaves undecided it asks the describing question of the program loaded
  again ([Q-M1f.221](../../../decisions/answer/Q-M1f.221-ein-test-asks-a-file-the-least-question-that.md)). No option
  chooses the question. `-v` names, for each claim, the question its outcome was read off and what the run found
  ([Q-M1f.223](../../../decisions/read-out/Q-M1f.223-ein-test-says-which-question-decided-each-claim.md)), as
  `--json-report`'s `route` and `fell_back_from` do.
- **A claim is held only by an answer that lists all of what it is about**, and a state found whose class the claim
  does not list refutes it ([Q-M1f.222](../../../decisions/answer/Q-M1f.222-under-ein-test-a-state-found-refutes-a-claim.md)).
- **A query with no `:expect` is never solved.** A file that states no claim costs its load.
- **1 means a claim is false, so a load error takes 2**, and so does a selection that checked nothing (§ 3). A runner
  that cannot tell a broken file from a false claim is the one failure a test runner must not have.

`--no-search` asks each file root alone, as `ein solve --no-search` does: a claim root decides is held or `FAILED`
as the full run decides it, and every other is `NOT CHECKED`, which moves no exit code
([Q-M1f.257](../../../decisions/cli/Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md)). `-T` aborts a file
after N seconds, and the abort is an `ERROR`: nothing was established either way. `--region-budget` bounds each
question a file is asked, and a claim no question decided within it is `NOT CHECKED`. `--events` and
`--json-summary` name one run, so they are refused over a selection of more than one file; where a file was asked
two questions, they hold the last. `--json-report` is § [5](#5-what-is-fixed-about-the-command-line)'s.

### 2.4 `ein saturate`

`ein saturate FILE` loads the program and saturates it: root, the node ∅, the program's least fixpoint with no
hypothesis taken. It prints the input, the time of each phase, the state before and after saturation and the
firings by rule; `--dump` adds the saturated knowledge base grouped by origin. It is how a commitment is checked by
hand, too: add its facts to a copy of the program and saturate the copy (§ [4.4](#44-checking)). `--max-steps N`
stops it after N firings (§ [5](#5-what-is-fixed-about-the-command-line)), and `--progress-every N` logs a progress
line every N steps (0 logs none).

### 2.5 `ein render` and `ein show`

**`ein render` draws; `ein show` lists.** `render` writes Graphviz DOT and nothing else on stdout; `show` writes Ein
text a `.ein` file could hold and nothing else on stdout. Both load the program first, so a program the loader
refuses is refused by every view as by every other command
([Q-M1f.71](../../../decisions/cli/Q-M1f.71-a-view-refuses-a-program-the-loader-refuses.md)), and no view changes
an answer. What each view draws, and how, is [`12-rendering.md`](12-rendering.md).

| view | what it draws or lists |
|---|---|
| `ein render program FILE` | the program as written: one digraph per group, rules, ontology, facts, the query |
| `ein render rules FILE`, `rule FILE --name R` | one digraph per rule, or one rule by name |
| `ein render constraints FILE` | the scopes of the structural properties a program states |
| `ein render kb FILE` | a knowledge base, loaded and saturated, as one digraph; a state file of `--dump-answer` is a `FILE`, so a solution draws the way root does |
| `ein render lattice FILE` | the nodes a describing `ein solve` run visits: R green, owing amber, dead red |
| `ein render region DIR` | an `ein oracle --dump` directory as a Hasse diagram, with a solve run's `--dump-lattice` and `--dump-answer` on it (`--visited`, `--answer`); it loads nothing |
| `ein render h-space FILE` | the structure of the hypothesis set: which candidates exclude each other, the groups and grids, what couples the questions ([`docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md`](../../reasoning/02-kb-evolution/05-hypothesis-space.md)) |
| `ein show hyps FILE` | the hypothesis set H as Ein facts, one candidate a line, in the universe's order; a `note:` on stderr says which rung built H and how many it holds ([Q-M1f.309](../../../decisions/cli/Q-M1f.309-ein-render-h-space-shows-the-hypothesis-space.md)) |

### 2.6 `ein kb`

`ein kb save FILE OUT.einb` writes the loaded program as a `.einb` container, and `--saturate` banks its least
fixpoint too. **Every command that takes a `.ein` takes a `.einb`**, recognising it by its first bytes and not by
its name, and answers it as it answers the source: `ein solve x.einb` prints what `ein solve x.ein` prints, byte for
byte, apart from the path it echoes
([Q-M1a.22](../../../decisions/cli/Q-M1a.22-a-containers-solve-is-its-sources-byte-for-byte.md)). A container stores
its program as text and loads it again when opened, so it is refused wherever its `.ein` would be. **`.einb` is a
private cache format, never an interchange one**: what crosses from one tool to another is `.ein` text, or the JSON
of `--json-summary`, `--json-report` and `--events`. `ein kb` exists in a build with the `einb` feature, which the
release binary has.

### 2.7 What every command reads

Every command reads a program by one path, `.ein` or `.einb` alike, and with it every module the program imports.
So a file that cannot be read, or is not UTF-8, is refused with the same line by `solve`, `saturate`, `oracle`,
`test`, each view and `kb save`, at each command's load-error exit; `ein test` counts the file an `ERROR` and goes on
to the next ([`07-diagnostics.md`](07-diagnostics.md)). `--no-require-why`, on every command that loads a program, loads
one whose relations, rules, hrules or macros lack a `:why` ([`06-config.md` § 3.4](06-config.md)).

---

## 3. Exit codes and streams

| code | `solve` / `oracle` / `render` / `show` / `saturate` / `kb` | `test` |
|---|---|---|
| 0 | success: every answer, whatever it is | every expectation held, and at least one was checked |
| 1 | a load error, *forbid*'s refusal among them; a claim the run refutes; an `oracle` cube that contradicts a monotone static verdict | an expectation is **false**, or was not checked (under `--no-search`, false only) |
| 2 | a usage error; a `-T` abort; an engine error (`-y`'s mismatch on a monotone program) | a load error or *forbid*'s refusal, a usage error, a `-T` abort — or nothing to check |

**An answer is never an exit code**
([Q-M1e2.33](../../../decisions/cli/Q-M1e2.33-every-answer-exits-0.md)): R empty, one state or many, a run the budget
stopped with a part of R, each of `--single`'s four words, every `--monotone` verdict — each exits 0. A run the
budget stopped writes one `note:` to stderr. A claim `ein solve` could not check is a `note:` on stderr and exits 0;
a claim it refutes exits 1, with its block on stdout under the claim and one line on stderr that names the file and
the first disagreement:

```text
f.ein: query :expect FAILED — the claim lists 1 class, the query has 1: (model (p a)) is not listed
```

A false claim is a **result**, not a refusal of the input, so its report belongs on stdout under the table it is
about; the stderr line is there because an exit 1 with an empty stderr is a run nobody can diagnose from a pipeline.

**`ein test` exits differently**, on purpose: 1 is taken by a false claim, so a load error takes 2, and 2 dominates
1 over a selection of several files, because *every expectation was checked and some are false* would be a false
description of a run in which one file did not load. A selection in which no `(query …)` or `(solve …)` carries an
`:expect` is 2, `error: nothing to check`.

**`--no-search` is no exit code either.** `ein solve --no-search` reports root and exits 0, or 1 where root decides a
claim false; `ein test --no-search` decides what root decides, and a claim it leaves `NOT CHECKED`, having been
asked not to search, moves no exit code.

**A `-T` abort** prints one line on stderr, `** aborted: --max-time S s reached before the question was decided **`,
nothing on stdout, and exits 2: a machine-dependent limit is never an answer.

**A closed stdout is no code either.** A write to a pipe whose reader has gone ends any command by `SIGPIPE`, with
nothing on stderr, which a shell reports as 141, as it does for `cat`
([Q-M1f.68](../../../decisions/cli/Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md)). An artefact written to a
closed pipe ends the process the same way
([Q-M1f.100](../../../decisions/cli/Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md)).

**Streams.** The answer, the forms after it and every view's output go to stdout; `note:` lines, warnings, progress
(`-v`, `--layer-progress`) and errors go to stderr. An artefact option writes its file and changes neither stream
nor the exit code; a write that fails after the run reports itself and keeps the run's exit code
([Q-M1e.22](../../../decisions/cli/Q-M1e.22-a-failed-artefact-write-keeps-the-runs-exit-code.md)). Stdout's listings
share 100 lines, and `--dump-answer DIR` writes each uncut
([Q-M1e2.34](../../../decisions/read-out/Q-M1e2.34-stdouts-listings-share-100-lines-and-a-dump.md)).

---

## 4. Which command answers which question

`F` is the program. Each answer is a statement about the oracle's R (§ [1](#1-the-contract)).

### 4.1 Is there a solution, and what is one?

| question | command | what you get | cost |
|---|---|---|---|
| Does the program have a solution at all? | `ein solve F -n 1` | one state of R; or *R is empty*, with why: root's core when root is dead, or what every alive node owes | up to the first member found; proving *none* takes every alive node, or the cube |
| One complete solution, one nothing can be added to | `ein solve F --maximal -n 1` | one state, certified maximal | the above, and one saturation per candidate outside it |
| Several | `ein solve F -n 5`, or `--maximal -n 5` | five states, the same five under every option | stops at the fifth |
| What is the least I have to guess? | `ein solve F --minimal -n 1`; all of them: `--minimal` | a minimal commitment and its state; or the whole minimal line, with sizes | owing nodes only |
| Does a solution need more than k guesses? | `ein solve F --minimal` | the sizes of the minimal commitments | as above |

### 4.2 Is it unique?

| question | command | what you get |
|---|---|---|
| Exactly one solution, for a program that states what it requires (obligations) | `ein solve F --single` | `Solution`; `Ambiguity` with two states; `Contradiction` or `Open`. Each exits 0 |
| Exactly one complete solution, for any program, one with restrictions only included | `ein solve F --maximal --single` | the same words, about the maximal line |
| Is the minimal guess unique? | `ein solve F --minimal --single` | the same, about the minimal line |
| The same check, stated in the program | a `(solve … :expect …)` or `(query … :expect …)` in F, then `ein test F` | `ok` or `FAILED`, and an exit code; the runner picks the question (§ [2.3](#23-ein-test)) |

Plain `--single` on a program with restrictions only answers `Ambiguity` whenever any candidate is alive: ∅, root
itself, is in R there, and every alive extension is a second state. That is what the second row is for.

### 4.3 How many, and what shape?

| question | command | what you get |
|---|---|---|
| How many solutions? | `ein solve F --count`; a bare `ein solve F` prints the same two rows above R in full | k_s (states) and k_c (commitments), with the rows that qualify them, and nothing listed |
| Every complete solution | `ein solve F --maximal` | the maximal line |
| Every minimal one | `ein solve F --minimal` | the minimal line |
| Everything | `ein solve F` | R in full: both counts, both lines, components, maximal sub-cubes and their dimensions, the merged classes, `N` |
| Which facts tell the complete solutions apart | `ein solve F --maximal --models key` | the determining key: the smallest set of slots that tells the states apart, and the combinations that occur |
| Every state as a file I can load | `ein solve F … --dump-answer DIR` | one loadable file per listed state, with the commitment that reached it ([`10-answer-dump.md`](10-answer-dump.md)) |
| What a query's goal takes across the solutions | `(query :goal … :expect …)` in F | the goal's classes ([`12-query.md`](../03-syntax-and-semantics/12-query.md)) |

### 4.4 Checking

| question | command |
|---|---|
| Is the answer I stated in the program right? | `ein test F` |
| Is this partial assignment consistent, and is it a solution? | add its facts to a copy of F, then `ein saturate COPY` for that node's state (dead? owing?), or `ein solve COPY -n 1` for what extends it |
| Does `ein solve` agree with the reference? | `ein oracle F` beside `ein solve F`: the same R wherever both finish |
| Is the program monotone, and which shortcuts did a run take? | `ein solve F --monotone` for the static check; a run's `route` row names the licences it used; `ein oracle F` checks the static verdict against the cube |

### 4.5 Cost and inspection

| question | command |
|---|---|
| How much would describing it cost? | `ein solve F`. Past the budget it stops, prints what is certified, and forecasts what the next step needs |
| Spend more, or less | `--region-budget FACTS` for a count of facts, `-T S` for a wall-clock abort |
| Faster, the same answer | `-j N`, or `-j auto` |
| What did the run visit? | the `work` row; `-t` for where the wall clock went; `--dump-lattice DIR` for every node; `ein render lattice F`, or `ein render region ORACLE_DIR --visited DIR` to draw them on the cube |
| The whole lattice, drawn | `ein oracle F --dump D`, then `ein render region D` |
| Why does a solution look like this? | `--trace FILE.md`: the derivation of each listed state from its commitment, and of each minimal dead set |
| What could be guessed, and how do the guesses interact? | `ein show hyps F` for H; `ein render h-space F` for its structure |

---

## 5. What is fixed about the command line

**Everything a script or a habit can depend on is fixed**
([Q-M1a.13](../../../decisions/cli/Q-M1a.13-the-clis-behaviour-is-fixed-and-its-helps-layout.md)): the subcommands
and the `render` and `show` views, every option's long name, short key, metavar, arity, default, choices,
mutually-exclusive group and help string, the accept or reject verdict on every invocation, the exit code, and which
stream each byte goes to. **Free**, and `clap`'s: wrapping, indentation, headings, the order within a section of
`--help`, and the wording of a usage diagnosis. The test suite holds every command's help to a checked-in rendering
of that structure, so a lost or renamed option fails on its own line.

**What a count may claim.** A count that is not exact says *a lower bound*, and says how far the run got; a run that
stopped never says R is empty ([Q-M1d.1](../../../decisions/answer/Q-M1d.1-a-search-stops-early-only-on-a-decided-question.md)).
`Contradiction` is said only where no node is alive
([Q-M1d.6](../../../decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)), and an unsat core
explains why there is no solution, which a stopped run has not shown. `--models list|key|grid` draws the states a
run lists, and `key` has nothing to tell apart below two.

**`-n 0` is refused**, exit 2, and so is every negative, in both spellings; a non-integer keeps its own message:

```text
$ ein solve x.ein -n=0
error: invalid value '0' for '--solutions <N>': invalid solution count: '0' (expected 1 or more; leave -n out to describe all of R)
```

A count of zero has no reading a user could mean: *record nothing* is not a question about R, and *no limit* is
leaving `-n` out, which describes all of R.

**`-g`/`--progress-every 0` logs no node line, and a negative is refused**
([Q-M1e.17](../../../decisions/cli/Q-M1e.17-g-0-logs-no-node-line-and-a-negative-is-refused.md)). Under `-v` the step
lines print and the node lines do not, as `ein saturate --progress-every 0` disables its samples. A negative is a
usage error, exit 2, in both spellings (`-g=-1` and `-g -1`):

```text
$ ein solve x.ein -v -g=-1
error: invalid value '-1' for '--progress-every <PROGRESS_EVERY>': invalid progress interval: '-1' (expected 0 or more nodes; 0 logs none)
```

**`ein saturate --max-steps N` stops at N firings, says where in Ein's spelling, and exits 0.** The limit is checked
before each step, so N firings run, and then the run stops without asking whether the knowledge base is at its
fixpoint, even when the N-th firing was the last there was. One line on stdout names the flag as typed, the firings
it let run and the last of them: its activation as a program writes it, its bindings as `?v = value`, and its facts
as Ein facts. A redundant last firing says it *re-derived* what the knowledge base already held. The state after,
the firing breakdowns, a per-rule breakdown at the limit and `--dump` follow, and they are the knowledge base where
the run stopped. Nothing goes to stderr, and the exit code is 0, as for every answer: a knowledge base stopped at N
firings is what the flag asks for. 0 and a negative stop before the first firing.

```text
$ ein saturate --max-steps 1 examples/saturation/transitive/colocation-chain.ein
…
!! saturator step limit hit: --max-steps 1 stopped saturation after 1 firing, before it checked for a fixpoint. The last firing: (transitive co-located) with ?rel = co-located · ?a = A · ?b = B · ?c = C derived (co-located A C) from (co-located A B) (co-located B C)
```

**`--layer-progress` streams the route's steps to stderr, and moves no byte of stdout.** One line per layer of the
route, or per batch of a question, with the nodes it visited, how many were dead, owing and in R, and the facts it
derived:

```text
$ ein solve examples/branching/04_two_levels.ein --layer-progress
layer 0: 1 visited — 0 dead, 0 owing, 1 in R · 0 facts
layer 1: 46 visited — 38 dead, 0 owing, 8 in R · 100 facts
layer 2: 24 visited — 16 dead, 0 owing, 8 in R · 104 facts
layer 3: 0 visited — 0 dead, 0 owing, 0 in R · 0 facts
```

`--verbose` prints the same step lines and, every `--progress-every` nodes, a line for the node. Both are additive:
stdout, the exit code and every counter are the same with them and without.

**`ein test --json-report FILE.json` writes one row per claim and per `(query …)` of the whole selection.** It is
additive in the strict sense the other artefact options are: stdout, stderr, the exit code and what is solved are the
same with it and without, so a query with no `:expect` is still never solved, and its row says so. The schema is
`ein-test-report/5`, deliberately not `ein-summary/2`: a summary is one run's answer, a report is one row per claim,
and a reader of the same version marker on both would be right to expect the same fields. Its top level is the
`schema`, a `tally` (`files`, `silent`, `held`, `failed`, `not_checked`, `errors`) and the `rows`. Every row carries:

| field | what it holds |
|---|---|
| `path`, `place` | the file, and whether the row is its `(solve …)` claim (`solve`) or a `(query …)` (`query`); a file's solve row is first among its rows, as its header is first among its lines |
| `query`, `queries` | the query's 1-based index, 0 on a solve row, and how many queries the file has |
| `goal`, `goal_relations` | the query's goal as written and the relations it closes; `null` and `[]` on a solve row |
| `outcome` | `held`, `failed`, `not-checked`, `no-expect` or `error` |
| `expect` | the claim's shape (`model`, `or`, `false` for a query; `region` or `false` for a solve claim), how many models it lists, its facts and negated facts, the relations it names, and what part of R it is over; a solve claim adds `merged` and `owing`. `null` where the query states no claim |
| `answer` | what a query's classes are over, whether that is exact, and how many classes and states |
| `region` | on a solve row, the R it was checked against: the `route`, `monotone`, `k_s`, `k_c` and the time it took |
| `outside`, `types_as_objects` | what the universe leaves out beside a claim a run checked |
| `route`, `fell_back_from` | the question the outcome was read off, spelled as `ein solve`'s flags (`--single`, `-n 4`, `--maximal -n 3`) or `describe`; and the question asked first where it left the claim undecided. Both `null` on a row no run checked |

It takes **any selection**, where `--events` and `--json-summary` are refused over more than one file: those name one
run, and a report has no run to be more than one of. **A file that did not load carries no claim**: its row is
`queries = 0`, `outcome = "error"`, `expect = null`, since a claim is a property of a program, and a file that
exists to be refused is not one.

---

## See also

- [`06-config.md`](06-config.md) — every option, `(config …)` flag and `EIN_*` name, and whether it changes the answer.
- [`08-read-out.md`](08-read-out.md) — every row `ein solve` and `ein test -v` print, and what it counts.
- [`10-answer-dump.md`](10-answer-dump.md) and [`11-region-dump.md`](11-region-dump.md) — the answer and the cube as files.
- [`12-rendering.md`](12-rendering.md) — what each `ein render` view draws.
- [`07-diagnostics.md`](07-diagnostics.md) — every refusal, at the parse and at load.
- [`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md) — what R is.
- [`01-install.md`](../01-install.md) — getting the binary, and `ein --version`.
