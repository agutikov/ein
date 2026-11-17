# S1f.20.8 — the census of questions, and the layout of `docs/decisions/`, for the user

**Stage:** [S1f.20.8](s1f.20.8_decisions.md), tasks T1f.20.8.1 (the census) and T1f.20.8.2 (the layout). Written
2026-11-16, to be shown at [S1f.20.2](s1f.20.2_the_disposition_table.md)'s stop with the disposition table and the
tree. **Nothing has moved:** no decision file is written (T3) and no check is built (T4) until the user has read
this page.

Every number below is re-taken by one command, on the tree as it stands:

```sh
python3 utils/decision_census.py --counts      # § 1
python3 utils/decision_census.py --subjects    # § 2
python3 utils/decision_census.py --local-ids   # § 5.5
python3 utils/decision_census.py --cited       # § 5.3, § 5.4
python3 utils/decision_census.py               # every id, one line: class, subject, ledger, answer
python3 utils/decision_census.py --check       # 0 gaps: every row classed, every decision a subject
```

The script is `utils/decision_census.py`; its hand half, one row an id, is
`utils/decision_census.txt` (both retired by S1f.20.9 part C1, in git).

## 1. What the census found

**593 ids in 15 ledgers: 425 decisions, 78 open, 90 not carried.** A *decision* has an answer that stands, a
ruling or a default nobody revised, read to the last word of its chain; *open* has neither; *not carried* is
superseded, moot, moved to another id or to a plan, or a copy of a question another ledger carries.

| ledger | decision | open | not carried | all |
|---|---:|---:|---:|---:|
| `docs/history/m1a_rust/open_questions.md` | 9 | 2 | 6 | 17 |
| `docs/history/m1c_external_validation/open_questions.md` | 2 | 2 | 3 | 7 |
| `docs/history/m1d_satisfiability/open_questions.md` | 5 | 1 | 1 | 7 |
| `docs/history/m1e_review_processing/open_questions.md` | 13 | 5 | 4 | 22 |
| `docs/history/m1e_review_processing/review/open-questions.md` (the review's Q1–Q10) | 4 | 0 | 6 | 10 |
| `plans/m1e2_correctness/open_questions.md` | 23 | 1 | 4 | 28 |
| `plans/m1f_hypothesis_and_documentation/open_questions.md` | 299 | 0 | 14 | 313 |
| `plans/m1g_logic/open_questions.md` | 1 | 15 | 0 | 16 |
| `plans/m1h_beyond_word_problems/open_questions.md` | 2 | 19 | 0 | 21 |
| `plans/m10_external_benchmarks/open_questions.md` | 0 | 3 | 0 | 3 |
| `plans/m2_nl_to_ir/open_questions.md` | 0 | 12 | 0 | 12 |
| `plans/open_questions.md` (the global pool, with M1's Q4–Q42) | 19 | 2 | 7 | 28 |
| `plans/followups/` (F1, F2, F3, F4) | 0 | 15 | 1 | 16 |
| P1f.6's `questions.md` 1–73 and round 2's layout item | 43 | 1 | 30 | 74 |
| stage files that number their own questions (six files) | 5 | 0 | 14 | 19 |
| **all** | **425** | **78** | **90** | **593** |

By family: the records under `docs/history/` 33 / 10 / 23; the milestone ledgers under `plans/` 325 / 50 / 18;
the global pool 19 / 2 / 7; the followups 0 / 15 / 1; the phase and stage ledgers 48 / 1 / 41.

**Classed by hand: 225 of 593**, banked in `decision_census.txt`; the other 368 are read off their state's first
words (*ruled*, *defaulted*, *accepted*, *open*, *moved*, …). By hand are every local number, every parked idea,
M2's ledger (whose last column names the stage that will decide, not a state), the global pool's sections (which
carry *Working answer* and no status word), and every row whose state names a chain past its first word
(*superseded*, *re-ruled*, *absorbed*, *overruled*, *reopened*, *moot*, *open*, *moved*): the script refuses to
class such a row until a hand row does, so a ledger edit that adds a chain is caught on the next run.

Not in the census: the user's ideas under `plans/ideas/`, whose open questions carry no ids and are the user's own;
M1h's *fifty questions* (S1h.10.2), which are questions about a text, not about Ein; and the decisions that have
no id at all (§ 5.8).

## 2. The layout: default 1, one file a decision

**361 decision files in 20 folders**, and 64 more if the process-only decisions are carried (§ 5.1). The default's
seven subjects are kept and five are added, each split from one of them where it was large enough to stand alone:
`imports/` from the language, `read-out/` and `diagnostics/` from the CLI, `hypotheses/` from the answer, and
`engine/` for what `ein.rs` decides inside. The stdlib is split by module, as the default says.

```text
docs/decisions/                 361   README.md: the folders, one line each, and how a decision is cited
  language/                      70   grammar, names, values, keywords, rules, macros, what the loader refuses
  imports/                       24   modules and imports: full and short names, what an import brings
  diagnostics/                   13   the line a refusal prints: its place, its wording, its note
  answer/                        19   the region R, cores, claims, the questions about R
  hypotheses/                    16   hrules, obligations, the generation ladder, the mechanisms, H
  read-out/                      27   what `ein solve` and `ein test` print: rows, words, JSON, the trace
  cli/                           16   commands, options, exit codes, views
  engine/                        22   the parser, the compiler's invariants, the `.einb` format, determinism
  stdlib/                         4   the layout, and the modules outside `math` and `bool` (1 pending)
    math/                        20   the shared parts: tables, `positional`, `arith`, `convert`, the calls (1 pending)
      decimal/                   24   `std.math.decimal`, `std.math.decimal-arith`
      int/                        6   `std.math.int`
      rat/                        6   `std.math.rat`
      gcd/                        4   `std.math.gcd`
      binary/                     5   `std.math.binary`, `std.math.binary-arith`
      num9/                       6   `std.math.num9`
    bool/                         6   `std.bool.*`
  corpus/                        37   the corpus, the register, fixtures, examples, the stdlib's test programs
  gate/                          32   the gate and the nightly: what runs, on what budget, under which profile
  docs/                           4   what a page may claim, and how a claim is warranted
  (process/                      64   plans and scheduling only: proposed not carried, § 5.1)
```

Every folder has a `README.md` with one line a decision, in id order: the id, linked to its file, then the title.
**The default fits.** No folder passes 70 files; the largest, `language/`, could be split into values, rules and
macros if the user wants, and nothing else needs it. Grouped files (the rulings' other option) would put about
seventy decisions into one `language.md`, which is not *a reasonable file size*; one file a decision also keeps
every id's link stable when a folder grows, and lets T4 resolve an id by its file name alone
(`docs/decisions/**/<id>-*.md`).

## 3. The slug rule

A decision's file is **`<id>-<slug>.md`**: the id as written (`Q-M1f.313`, `Q23`), then the slug of its title.
The title states the question's answer as a sentence, and its slug is the title in lower case, an apostrophe
dropped, every other run of characters outside `[a-z0-9]` one `-`, cut at the last whole word within 48
characters, a trailing connective (*and*, *or*, *a*, *the*, *of*, …) dropped. The id alone keeps a name unique, so
two decisions may share a slug. `python3 utils/decision_census.py --slug "TITLE"` applies the rule.

| id | title T3 would write | file |
|---|---|---|
| Q-M1e2.24 | Ein has no numbers and no ranges | `language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md` |
| Q-M1f.313 | The agenda fires first what leads to `(false)` | `engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md` |
| Q-M1f.231 | Every name a module declares has a full name, and a file lists the short names it writes | `imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md` |
| Q-M1f.179 | Integer division names its convention | `stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md` |
| Q-M1e2.27 | In Ein's JSON, `[]` is the empty set and `null` no object | `read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md` |
| Q-M1f.340 (was S1f.5.4's Q4) | Every name is a JSON string | `read-out/Q-M1f.340-every-name-is-a-json-string.md` |

## 4. Four decision files, as T3 would write them

Each holds its id, a title that states the answer, the decision, a short reason, and what holds it. None holds an
option not taken, a date, a name, a stage or a plan. A link to another decision is by its file.

`docs/decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md`:

```markdown
# Q-M1e2.24 — Ein has no numbers and no ranges

Ein has no numbers, no ranges and no int kind, and none is planned. A run of name characters is a name, digits
included: `1` and `01` are two names, `-7` is a name, and `1..5` is the name it spells. A name of digits is a
candidate like any other name. A name is spelt from an allow-list of ASCII characters, letters, digits, the
connectors `_ - .` and the decorators, and stays ASCII; `\` is never one.

**Why.** No operation of the engine reads a value as a quantity: its only relations of values are `eq` and `neq`,
fixed by name identity. Arithmetic is a theory of the stdlib, computed by saturation over numerals written in digit
names, so a kernel number would be a second way to say what a rule already says, and one no rule could explain.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_run_of_digits_is_a_name_in_every_place`, `1`, `01`, `7`, `-7`
and `07` five names in every place a name is written; `a_range_is_the_name_it_spells`. The characters of a name:
the grammar, `ein-ir/src/ein.l`, and its table, `ein-ir/tests/grammar_pages.rs`.
```

`docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md`:

```markdown
# Q-M1f.313 — The agenda fires first what leads to `(false)`

Every saturation, root's, each route's node's, the oracle's and every reader's, ranks its candidates in two
halves: first what leads to `(false)`, a rule concluding `(false)`, a stored negation, or a fact whose negation can
be concluded; then the rest. Within each half rules fire in written order. A resumed saturation keeps the order.

**Why.** On a monotone program the fixpoint does not depend on the order, so the order decides no answer, and
firing first what leads to `(false)` finds a dead node's contradiction sooner. Of a program the static check flags
as not monotone, the engine claims nothing under any order.

**Held by.** `saturator::AGENDA`, the one order every saturation takes; `ein-infer/tests/agenda.rs`, which holds
every question and every claim of every monotone corpus program alike under this order and under the written
order alone. A core is an explanation whatever the order: [Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md).
```

`docs/decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md`:

```markdown
# Q-M1f.179 — Integer division names its convention

`std.math.int` has no bare division. Each of the three usual conventions is a request of its own, answered by one
fact with a quotient Q and a remainder R, X = Q × Y + R:

- `(int-div-euclid X Y)`, answered by `(int-quot-euclid-of X Y Q R)`: 0 ≤ R < |Y|, and R is a natural number of
  `std.math.decimal-arith`, since it is never negative;
- `(int-div-trunc X Y)`, answered by `(int-quot-trunc-of X Y Q R)`: Q rounded toward zero, R with the dividend's
  sign;
- `(int-div-floor X Y)`, answered by `(int-quot-floor-of X Y Q R)`: Q rounded down, R with the divisor's sign.

**Why.** The conventions differ only when an operand is negative, and each is the one some reader expects: number
theory the Euclidean, C and Rust the truncated, Python the floored. A request that names its convention leaves no
reader to guess which one the theory took.

**Held by.** `ein-infer/tests/int_theory.rs`, against Rust's integers on every pair of small parts and on requests
of every length and sign; `tests/stdlib/math/int/09_div_euclid.ein` to `14_div_floor_by_zero.ein`. The results'
names: [Q-M1f.200](Q-M1f.200-the-results-of-integer-division.md).
```

`docs/decisions/read-out/Q-M1f.340-every-name-is-a-json-string.md` (a local number, S1f.5.4's Q4, given its id
by default 3):

```markdown
# Q-M1f.340 — Every name is a JSON string

Where Ein's JSON writes a name, it writes a JSON string, whatever its characters: a goal binding of `007` is
`"007"`, and of `7` is `"7"`.

**Why.** A run of digits is a name ([Q-M1e2.24](../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)).
Written as JSON numbers, `007` and `7`, two names, would read back as one.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_goal_binding_of_digits_is_a_json_string`.
```

## 5. What does not fit, and what the census found on the way

### 5.1 The process-only decisions: 64

**64 of the 425 decisions (15 %) are about the plans and not the system**: which phase takes a finding, whether a
phase is made, where a rework goes, what a close does with an inventory, which corpus programs a stage edited,
which of a stage's moves are rows of the wrong-answer register, what a re-plan took. They split as **16** about the
register's bookkeeping (Q-M1f.5, .13, .16, .24, .28, .35, .36, .44, .76, .83, .89, .90, .98, .134, .144, .216),
**17** naming a finding's owner (P1f.6's section J, 56 to 72), and **31** others (phase order, a fixture retired,
a commit left, M1's targets, M1h's text).

**Proposed: not carried.** Under the rulings *only information about the current state of the project* goes into
`docs/`, and these state how work was scheduled, which `git log` keeps. They are deleted with their plans at
S1f.20.9. Two consequences the user should see before ruling:

- **21 of them are cited outside `plans/`**, most by the register (`corpus/wrong_answers.toml` names the question
  a row was filed under) and `corpus/corpus.toml`, the rest by kernel pages, tests, source comments and two
  scripts. S1f.20.9 rewrites each citation: to the decision that rules the behaviour, or to nothing.
- **One of them states a convention still in use**: Q-M1f.36's reading, *a register row for each way the engine
  answers today*, which later rows cite as *Q-M1f.36's reading*. Proposed: carry Q-M1f.36 alone, under `corpus/`,
  as the register's rule, and leave its fifteen applications.

### 5.2 Decided, and not built: 4

Q-M1g.1 (the spelling of M1g's connectives, defaulted), Q-M1h.4 and Q-M1h.21 (the text M1h works on), and P1f.6's
question 42 (*a small cryptarithm theory, `std.math.crypt`*, answered yes, with no stage). **Proposed:** each stays
with its plan until the plan builds it, then gets its file. P1f.6's 42 has no plan to stay with once P1f.6's folder
is deleted, so it needs one (a followup, or M1g's).

### 5.3 Not carried, and cited outside `plans/`: 16 ids, 58 files

A superseded or merged id that code or a page still cites: Q-M1a.8 (refiled as Q-M1e.16), Q-M1d.4, Q-M1e.3,
Q-M1e.8, Q-M1e.9 (its language half is Q-M1e2.5, *forbid*; 15 files), Q-M1e.15 (its ruling is Q-M1e2.11; 13 files),
Q-M1e2.4, Q-M1e2.13 and Q-M1e2.14 (the wildcard, now Q-M1e2.22; 15 and 11 files), Q-M1e2.20, Q-M1f.4, Q-M1f.135,
.141, .142 and .217 (the import selection, gone with Q-M1f.247), and Q-M1f.307 (the same ruling as Q-M1e.11).
**Proposed:** S1f.20.9 rewrites each citation to the id that carries the decision; no redirect file, since a
redirect is history.

### 5.4 Open questions: 78, of which 10 are in `docs/history/` and 9 are cited outside `plans/`

The 78 open ids stay in `plans/`. But **10 live in a ledger under `docs/history/`**, which S1f.20.9 deletes:
Q-M1a.6, Q-M1a.23, Q-M1c.6, Q-M1c.7, Q-M1d.1, Q-M1e.10, .13, .14, .19 and .22. **Proposed:** before S1f.20.9 they
move, ids kept, to `plans/open_questions.md` (or to the followup that owns each: F23 owns Q-M1a.6's `at None`).

**9 open ids are cited in 33 files outside `plans/`** (Q-M1a.6, Q-M1a.23, Q-M1c.6, Q-M1c.7, Q-M1d.1, Q-M1e.10,
Q-M1e.13, Q-M1e.22, Q-M1e2.2): a test, a `.ein` header or a kernel page that says *this gap is Q-n's*. Under the
rulings a page may cite a decision and not a plan, and an open question is a plan's. **Proposed:** each such
citation states the gap in plain words and cites nothing; the gap's question stays in `plans/`.

### 5.5 Local numbers: 52 decisions take a milestone id (31 if § 5.1 is ruled as proposed)

By default 3, a decision known only by a phase's or a stage's own number takes the next free id of the milestone
that asked it, after the ledger's last id (Q-M1f.314, the release-voice check's readings, defaulted by S1f.20.1);
a gap (Q-M1f.49 was never used) is not reused. The 31 that are not process-only come first, so the
21 process-only ones take the last numbers and leave none missing if they are not carried:

| local number | proposed id | folder |
|---|---|---|
| `Q1@M1e-review` (determinism under `--jobs`) | Q-M1e.23 | `engine/` |
| `Q7@M1e-review` (`-n 0`) | Q-M1e.24 | `cli/` |
| `Q8@M1e-review` (both Zebra encodings pinned to the grid) | Q-M1e.25 | `corpus/` |
| `Q10@M1e-review` (the release matrix) | Q-M1e.26 | `gate/` |
| `P1f.6#3` (the loader's arity check) | Q-M1f.315 | `language/` |
| `P1f.6#4` (detection rules) | Q-M1f.316 | `stdlib/math/` |
| `P1f.6#5` (a name no `let` states) | Q-M1f.317 | `stdlib/math/decimal/` |
| `P1f.6#6` (a request no rule completes) | Q-M1f.318 | `stdlib/math/` |
| `P1f.6#7` (no module reserves its names) | Q-M1f.319 | `imports/` |
| `P1f.6#9` (`num9` stays silent) | Q-M1f.320 | `stdlib/math/num9/` |
| `P1f.6#13` (binary a theory of its own) | Q-M1f.321 | `stdlib/math/binary/` |
| `P1f.6#15` (one `z`) | Q-M1f.322 | `stdlib/math/` |
| `P1f.6#17` (`std.math.convert`) | Q-M1f.323 | `stdlib/math/` |
| `P1f.6#18` (one division) | Q-M1f.324 | `stdlib/math/decimal/` |
| `P1f.6#19`, `#20`, `#21`, `#23` (`std.bool`) | Q-M1f.325 to .328 | `stdlib/bool/` |
| `P1f.6#27`, `#28`, `#29`, `#31`, `#32`, `#34`, `#35` (the word problems) | Q-M1f.329 to .335 | `corpus/` |
| `P1f.6#38` (`std.math.gcd`) | Q-M1f.336 | `stdlib/math/gcd/` |
| `P1f.6#42` (`std.math.crypt`, pending) | Q-M1f.337 | `stdlib/math/` |
| `P1f.6#layout` (`std.math.*`, `std.bool.*`) | Q-M1f.338 | `stdlib/` |
| `S1e2.3.4#Q6` (both witnesses pinned) | Q-M1e2.29 | `corpus/` |
| `S1f.5.4#Q1` (the loader's names start with `#`) | Q-M1f.339 | `language/` |
| `S1f.5.4#Q4` (every name a JSON string) | Q-M1f.340 | `read-out/` |

The 21 process-only ones would be Q-M1f.341 to .361 (P1f.6's 22, 56 to 73; S1f.5.4's Q2 and Q3). The other 50
local numbers are not carried: **30** are the same question as a ledger id (P1f.6's 1 and 2 are Q-M1f.179, its 45
is Q-M1f.153 and .161, 22 of P1f.6's in all; S1f.5.5's Q1 and Q2 are Q-M1f.1 and .2; the review's Q2, Q3 and Q6
are Q-M1e2.11, Q-M1e.16 and Q-M1e.11), **9** are a stage file's copy of another local number (`malformed_numbers.md`'s
Q1 to Q6 are P1f.6's 3, 5, 4, 7, 8 and 9; M1e's probe page's Q4 to Q6 are the review's), **8** were moved to a plan
(P1f.6's 10, 24, 25, 26, 33, 36, 37 and 40: M1g's and M1h's phases, and P1f.10), and **3** of the review's are moot
(a defect fixed, a lever gone, a review's coverage). P1f.6's 55 (*may a number print as a number*) is open, with
M1g's S1g.6.3.

### 5.6 Ids used twice

- **The global pool and M2**: M2's Q23, Q24 and Q25 (the local model, one GBNF, the frontend's language) are not
  the global pool's Q23, Q24 and Q25 (M1's type slot, `:where`, ordinality). M2's three are open.
- **The global pool and F4**: F4's Q30, Q31, Q32, Q33 and Q40 are not the global ones of those numbers (F4's
  Q34 to Q38 and Q14 are). F4's Q39 has the number the global pool never used.
- **F3**: its Q34 and Q35 (*gaps* on partial problems, counterfactual traces) are a third pair of those numbers.
- **The M1e review's Q1 to Q10** are not the global Q1 to Q10. Its four decisions take Q-M1e ids (§ 5.5).

The census keys the later one of each pair `Q23@M2`, `Q30@F4`, `Q34@F3`, `Q1@M1e-review`. All the colliding
questions left in `plans/` are open; the 17 carried global decisions keep their bare `Qn`, and nothing in
`docs/decisions/` collides.

### 5.7 Where the census reads a row against its ledger

- **Q-M1f.135, .140, .141 and .142 say *stands*** (the selective import's defaults, confirmed when section I of
  P1f.6's questions was left empty), **and are superseded**: every import brings its module whole, and no closure
  exists (Q-M1f.247, the user: *"in all cases we import everything, the difference only in names"*). Q-M1f.136 the
  same, by full and short names. The census does not carry them.
- **Q-M1f.104** (*`corpus_cores`' slow half stays whole in the nightly*) is superseded by Q-M1f.293's shards; its
  row does not say so.
- **Q-M1e.20** starts *open* and was answered (a) in its own row; **Q-M1e.8** is open in its ledger and is overtaken
  by the region R, W1's rows fixed.
- **Q-M1e.4**'s three warrants for a count in prose are *a command, a date, or a test*, and the rulings say docs
  may not cite dates: the date warrant conflicts (flagged `conflict`). `AGENTS.md` states the same three.
- **Q-M1e.3** (*a page neither fixed nor deleted moves into a milestone record*) is superseded by the rulings on the
  documentation tree, which have no id.
- **Two rulings carry two ids each**: Q-M1e.11 and Q-M1f.307 (*a debt root was not told of*, (A)), carried as
  Q-M1e.11; Q-M1f.147 and .153 (reply 45, *skip big graphs*), carried as .153.

### 5.8 Decisions with no id

The census reads ledgers, and some rulings were never a ledger row: the user's rulings on the documentation tree
(this phase's README, which supersede Q-M1e.3 and part of Q-M1e.4); that no config option changes the answer, that
each mode does the least work its question needs, that every answer exits 0 (rulings `AGENTS.md` states as standing
rules); P1f.6's `design.md` rulings; M1e's stage decisions D1 to D9; and the stage defaults Q-M1f.305 and Q-M1f.310
accept as a set without an id each.

### 5.9 Answers to check against today's engine: 16

Flagged `verify`: their answer as written may not describe the engine that ships, mostly the global pool's M1
answers (Q1, Q16, Q18, Q21, Q25, Q40, whose text describes the 2021 prototype's data model or a `dot → ir` nothing
builds), M1c's expectation forms, the lever matrix, and four calculator defaults written before the numeral became
its representation's (Q-M1f.146, .205, .206, .208). T3 checks each and writes what holds it, or lists it not
carried.

## 6. Questions for the user

Each has a default, which T3 takes unless the user rules otherwise.

1. **The layout.** Default 1 as drawn in § 2: one file a decision, `<id>-<slug>.md`, 20 folders, `diagnostics/`,
   `hypotheses/` and `engine/` added to the default's subjects, the stdlib by module. *Default: as drawn.*
2. **The 64 process-only decisions** (§ 5.1). *Default: not carried; deleted with their plans; their 21 citations
   rewritten by S1f.20.9; Q-M1f.36's register reading carried alone, under `corpus/`.*
3. **The 4 decided and unbuilt** (§ 5.2). *Default: each stays with its plan until built; P1f.6's question 42
   (`std.math.crypt`) is handed to a followup so that it outlives P1f.6's folder.*
4. **Not carried and still cited** (§ 5.3, 16 ids in 58 files). *Default: each citation rewritten to the id that
   carries the decision; no redirect files.*
5. **The open questions** (§ 5.4). *Default: the 10 in `docs/history/` move to `plans/open_questions.md`, ids kept,
   before S1f.20.9; the 9 cited outside `plans/` are cited no more, the gap stated in plain words where it is.*
6. **The local numbers** (§ 5.5). *Default: the map as listed, Q-M1e.23 to .26, Q-M1e2.29, Q-M1f.315 to .340 (and
   .341 to .361 only if the process-only ones are carried).*
7. **The ids used twice** (§ 5.6). *Default: the carried global decisions keep `Qn`; the colliding open ones in
   `plans/` take their ledger's prefix when next touched (M2's Q23 to Q25 become Q-M2.5 to .7).*
8. **Decisions with no id** (§ 5.8). *Default: the standing rules `AGENTS.md` states without an id (the answer's
   rulings, the documentation tree's) take the next free id of the milestone that ruled them, listed in this
   stage's record; D1 to D9, `design.md`'s rulings and the defaults Q-M1f.305 and .310 accept are not carried one
   by one.*
9. **Q-M1e.4's date warrant** (§ 5.7). *Default: a count in prose carries the command that re-takes it or the
   test that pins it; the date warrant goes, here and in `AGENTS.md`.*
10. **The 16 answers to check** (§ 5.9). *Default: T3 checks each against the engine; one that no longer holds is
    listed not carried, with why.*
