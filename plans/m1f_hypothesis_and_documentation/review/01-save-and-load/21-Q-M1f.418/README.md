# Q-M1f.418 — the nesting limit, and a saved file's wrappers

## 1. The question

The parser refuses a program nested deeper than **1 000 parentheses** (a top-level form is depth 1), before parsing it,
because the walks over the parsed tree (macro expansion, the loader, the dumper) recurse. A saved file wraps the
program's text in `(program …)` and each module's in `(module NAME …)`, one level above the text. So a program exactly
at the limit is **one level past it** inside its own saved file. Should the parser give a saved file that one extra
level, its wrappers' own, so that every program that loads can be saved and loaded again (today), or should the limit
stay as it is, so that a program at the limit has a state that cannot be saved? With it, the decision
[Q-M1f.185](../../../../../docs/decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md) was rewritten: it
used to say that a **state** deeper than the limit (a fact a rule derives past 1 000) is refused when its state file is
loaded again; it now says such a state is answered, saved and loaded again.

Recorded in [`../../open_questions.md` § Q-M1f.418](../../../open_questions.md#q-m1f418--the-nesting-limit-and-a-saved-files-wrappers).
The decision before and after: [`runs/diff.decision-Q-M1f.185.before-vs-now.diff`](runs/diff.decision-Q-M1f.185.before-vs-now.diff).

## 2. What is so today

Programs, all written for this review, all `(solve :no-search)` so R is root alone (one saved file each), depths in
[`depth.programs.txt`](runs/depth.programs.txt):

| program | deepest text | its state |
|---|---:|---|
| [`at_limit.ein`](programs/at_limit.ein) | 1000 | the fact as written |
| [`past_limit.ein`](programs/past_limit.ein) | 1001 | — refused |
| [`at_limit_deeper_state.ein`](programs/at_limit_deeper_state.ein) | 1000 | a rule wraps it once: a fact 1001 deep |
| [`below_limit_deeper_state.ein`](programs/below_limit_deeper_state.ein) | 999 | a rule wraps it twice: a fact 1001 deep |
| [`with_deep_module.ein`](programs/with_deep_module.ein) + [`deep_module.ein`](programs/deep_module.ein) | 1000 in the module | the module's fact |

Binary: `build/review-bin/ein-head-b5fb8d70`; exits in [`00-exit-codes.txt`](runs/00-exit-codes.txt).

**The limit on a program.** `at_limit.ein` loads and is answered ([`at_limit.solve.txt`](runs/at_limit.solve.txt));
`past_limit.ein` is refused, exit 1 ([`past_limit.solve.stderr.txt`](runs/past_limit.solve.stderr.txt)):

<!-- output -->
```text
programs/past_limit.ein:5:3001: this `(` nests deeper than the limit of 1000 parentheses
```

**Each saved file is one level deeper than its text, and loads.** The saved root files nest 1001 (the three at the
limit) and 1000 (the one below) ([`depth.saved.txt`](runs/depth.saved.txt)); each loads, exit 0
([`at_limit.saved.solve.txt`](runs/at_limit.saved.solve.txt),
[`at_limit_deeper_state.saved.solve.txt`](runs/at_limit_deeper_state.saved.solve.txt),
[`with_deep_module.saved.solve.txt`](runs/with_deep_module.saved.solve.txt),
[`below_limit_deeper_state.saved.solve.txt`](runs/below_limit_deeper_state.saved.solve.txt)). For each of the four:

- **load → state**: `ein saturate --dump` of the saved file and of the program agree from *state AFTER saturation*
  on, fact for fact and origin for origin (`*.diff.saturate.program-vs-saved.diff`, all four empty:
  [`at_limit`](runs/at_limit.diff.saturate.program-vs-saved.diff),
  [`at_limit_deeper_state`](runs/at_limit_deeper_state.diff.saturate.program-vs-saved.diff),
  [`below_limit_deeper_state`](runs/below_limit_deeper_state.diff.saturate.program-vs-saved.diff),
  [`with_deep_module`](runs/with_deep_module.diff.saturate.program-vs-saved.diff));
- **save → load → save**: `ein solve SAVED --no-search --dump-answer` writes `root.ein`, the saved file byte for byte
  (`*.check.save-load-save.txt`: [`at_limit`](runs/at_limit.check.save-load-save.txt),
  [`at_limit_deeper_state`](runs/at_limit_deeper_state.check.save-load-save.txt),
  [`below_limit_deeper_state`](runs/below_limit_deeper_state.check.save-load-save.txt),
  [`with_deep_module`](runs/with_deep_module.check.save-load-save.txt)).

**A state deeper than the limit is answered, saved and loaded again** (Q-M1f.185 as rewritten). Loaded from its saved
file, each `*_deeper_state` holds a derived fact 1001 deep ([`deepest-fact.txt`](runs/deepest-fact.txt)):

<!-- output -->
```text
runs/at_limit_deeper_state.saved.saturate-dump.txt: the deepest line nests 1001 deep; it begins '(q (p (p (p (p (p (p (p (p (p (p (p (p (' and ends ')))))))))))))))))))))))))))))))))))))))))))))))) :rule wrap)'
runs/below_limit_deeper_state.saved.saturate-dump.txt: the deepest line nests 1001 deep; it begins '(q (q (p (p (p (p (p (p (p (p (p (p (p (' and ends ')))))))))))))))))))))))))))))))))))))))))) :rule wrap-twice)'
```

**One level deeper still is refused.** A saved file carrying `past_limit.ein`, written by hand in the saved layout
([`past_limit.saved-by-hand.ein`](runs/past_limit.saved-by-hand.ein), 1002 deep,
[`depth.past-saved.txt`](runs/depth.past-saved.txt)), exit 1, on `solve` and on `saturate`
([`past_limit.saved-by-hand.solve.stderr.txt`](runs/past_limit.saved-by-hand.solve.stderr.txt),
[`past_limit.saved-by-hand.saturate.stderr.txt`](runs/past_limit.saved-by-hand.saturate.stderr.txt)):

<!-- output -->
```text
runs/past_limit.saved-by-hand.ein:7:3001: this `(` nests deeper than the limit of 1000 parentheses
```

**The earlier binary (`ein-a8d595eb`): what Q-M1f.185 used to say.** Its state file is the program reprinted plus every
fact of the state. For `at_limit.ein` that file is 1000 deep and loads
([`a8d.at_limit.state.solve.txt`](runs/a8d.at_limit.state.solve.txt)); for both `*_deeper_state` programs it holds
the derived fact, 1001 deep ([`depth.a8d-state-files.txt`](runs/depth.a8d-state-files.txt)), and is refused when
loaded again, exit 1 ([`a8d.at_limit_deeper_state.state.solve.stderr.txt`](runs/a8d.at_limit_deeper_state.state.solve.stderr.txt),
[`a8d.below_limit_deeper_state.state.solve.stderr.txt`](runs/a8d.below_limit_deeper_state.state.solve.stderr.txt)):

<!-- output -->
```text
runs/a8d.at_limit_deeper_state.dump/models/01.ein:1025:3001: this `(` nests deeper than the limit of 1000 parentheses
runs/a8d.below_limit_deeper_state.dump/models/01.ein:1029:3001: this `(` nests deeper than the limit of 1000 parentheses
```

So the rewrite of Q-M1f.185 comes from the format (Q-M1f.392: no fact of the state in the file), not from the extra
level: `below_limit_deeper_state`'s saved file is 1000 deep and would load under either option.

## 3. The options

- **(a) as built**: a text that opens as a saved file is parsed with one level more, its wrappers' own; each text it
  carries is parsed again alone under the limit, as the file it was. Everything in § 2.
- **(b) the limit stays**: a program (or a module) at exactly 1000 has a saved file that does not load. **Mocks**:
  [`at_limit.saved.solve.MOCK.option-b.stderr.txt`](runs/at_limit.saved.solve.MOCK.option-b.stderr.txt),
  [`at_limit_deeper_state.saved.solve.MOCK.option-b.stderr.txt`](runs/at_limit_deeper_state.saved.solve.MOCK.option-b.stderr.txt),
  [`with_deep_module.saved.solve.MOCK.option-b.stderr.txt`](runs/with_deep_module.saved.solve.MOCK.option-b.stderr.txt),
  each one line, exit 1, written by hand at the first `(` past 1000 in the saved file (the program's 1000th `(p `,
  line 7, 10 or 16, column 2998):

  <!-- output -->
  ```text
  runs/at_limit.dump/models/01.ein:7:2998: this `(` nests deeper than the limit of 1000 parentheses
  ```

  Two of these lines are **also real output**: the earlier binary, whose parser has no level for the wrapper, prints
  exactly them for today's saved files
  ([`a8d.reads-today-saved.solve.stderr.txt`](runs/a8d.reads-today-saved.solve.stderr.txt),
  [`a8d.reads-today-saved-module.solve.stderr.txt`](runs/a8d.reads-today-saved-module.solve.stderr.txt); diffs with
  the mocks, empty: [`at_limit`](runs/at_limit.saved.solve.a8d-vs.MOCK.option-b.diff),
  [`with_deep_module`](runs/with_deep_module.saved.solve.a8d-vs.MOCK.option-b.diff)). Given the 1000-deep saved
  file of `below_limit_deeper_state`, the same binary passes the nesting check and then fails on the `(program …)` form
  it does not know ([`a8d.reads-today-saved-below.solve.stderr.txt`](runs/a8d.reads-today-saved-below.solve.stderr.txt)):
  under (b) that file loads, as it does today. What (b) loses is the texts at exactly 1000; whether `--dump-answer`
  would then refuse to write such a file, or write one that does not load, is not said by the option.

## 4. The default

**(a)**, from the ledger: `(program …)` and `(module …)` stand one level above the texts they carry, so the limit is the
texts'; the limit protects the walks over a parsed tree, and a saved file adds one wrapper level, not a deeper walk.
Built in `ein_ir::grammar`'s parse (`NESTING_LIMIT + 1` when the text opens as a saved file); held by
`ein-ir/tests/generated_parser.rs`' `a_saved_file_carries_a_program_at_the_limit`; the decision Q-M1f.185, rewritten.

## 5. Files

- [`rerun.sh`](rerun.sh) — every command below, in order, and the mocks' diffs; the mocks themselves are not touched by it.
- [`programs/at_limit.ein`](programs/at_limit.ein) — written for this review: a fact 1000 deep.
- [`programs/past_limit.ein`](programs/past_limit.ein) — written for this review: a fact 1001 deep, refused.
- [`programs/at_limit_deeper_state.ein`](programs/at_limit_deeper_state.ein) — written for this review: 1000 deep, a rule derives a fact 1001 deep.
- [`programs/below_limit_deeper_state.ein`](programs/below_limit_deeper_state.ein) — written for this review: 999 deep, a rule derives a fact 1001 deep.
- [`programs/with_deep_module.ein`](programs/with_deep_module.ein), [`programs/deep_module.ein`](programs/deep_module.ein) — written for this review: a program importing a module whose fact is 1000 deep.
- [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) — every command run, with its exit code.
- [`runs/depth.programs.txt`](runs/depth.programs.txt), [`runs/depth.saved.txt`](runs/depth.saved.txt), [`runs/depth.past-saved.txt`](runs/depth.past-saved.txt), [`runs/depth.a8d-state-files.txt`](runs/depth.a8d-state-files.txt) — the deepest nesting of each file, comments and strings aside, and where it is first reached.
- [`runs/at_limit.solve.txt`](runs/at_limit.solve.txt), [`runs/at_limit_deeper_state.solve.txt`](runs/at_limit_deeper_state.solve.txt), [`runs/below_limit_deeper_state.solve.txt`](runs/below_limit_deeper_state.solve.txt), [`runs/with_deep_module.solve.txt`](runs/with_deep_module.solve.txt) — today's read-out of each program (the runs that saved).
- [`runs/past_limit.solve.stderr.txt`](runs/past_limit.solve.stderr.txt) — the program past the limit refused.
- `runs/*.dump/` — today's directories: [`at_limit.dump/`](runs/at_limit.dump/) ([`models/01.ein`](runs/at_limit.dump/models/01.ein)), [`at_limit_deeper_state.dump/`](runs/at_limit_deeper_state.dump/) ([`models/01.ein`](runs/at_limit_deeper_state.dump/models/01.ein)), [`below_limit_deeper_state.dump/`](runs/below_limit_deeper_state.dump/) ([`models/01.ein`](runs/below_limit_deeper_state.dump/models/01.ein)), [`with_deep_module.dump/`](runs/with_deep_module.dump/) ([`models/01.ein`](runs/with_deep_module.dump/models/01.ein), carrying `(module deep_module …)`); each also holds `index.json`, `answer.txt`, `answer.json`, `nogoods.json`.
- `runs/*.saved.solve.txt` — each saved file loaded: [`at_limit`](runs/at_limit.saved.solve.txt), [`at_limit_deeper_state`](runs/at_limit_deeper_state.saved.solve.txt), [`below_limit_deeper_state`](runs/below_limit_deeper_state.saved.solve.txt), [`with_deep_module`](runs/with_deep_module.saved.solve.txt).
- `runs/*.program.saturate-dump.txt`, `runs/*.saved.saturate-dump.txt` — `ein saturate --dump` of each program and its saved file: [`at_limit` program](runs/at_limit.program.saturate-dump.txt), [saved](runs/at_limit.saved.saturate-dump.txt); [`at_limit_deeper_state` program](runs/at_limit_deeper_state.program.saturate-dump.txt), [saved](runs/at_limit_deeper_state.saved.saturate-dump.txt); [`below_limit_deeper_state` program](runs/below_limit_deeper_state.program.saturate-dump.txt), [saved](runs/below_limit_deeper_state.saved.saturate-dump.txt); [`with_deep_module` program](runs/with_deep_module.program.saturate-dump.txt), [saved](runs/with_deep_module.saved.saturate-dump.txt).
- `runs/*.diff.saturate.program-vs-saved.diff` — those two compared after saturation (all empty), linked in § 2.
- `runs/*.saved.resave.txt`, `runs/*.resave/` (`root.ein`, `answer.txt`, `answer.json`), `runs/*.check.save-load-save.txt` — each saved file loaded and saved again under `--no-search`, and the comparison: directories [`at_limit.resave/`](runs/at_limit.resave/), [`at_limit_deeper_state.resave/`](runs/at_limit_deeper_state.resave/), [`below_limit_deeper_state.resave/`](runs/below_limit_deeper_state.resave/), [`with_deep_module.resave/`](runs/with_deep_module.resave/); their `root.ein`: [`at_limit`](runs/at_limit.resave/root.ein), [`at_limit_deeper_state`](runs/at_limit_deeper_state.resave/root.ein), [`below_limit_deeper_state`](runs/below_limit_deeper_state.resave/root.ein), [`with_deep_module`](runs/with_deep_module.resave/root.ein); read-outs [`1`](runs/at_limit.saved.resave.txt), [`2`](runs/at_limit_deeper_state.saved.resave.txt), [`3`](runs/below_limit_deeper_state.saved.resave.txt), [`4`](runs/with_deep_module.saved.resave.txt).
- [`runs/deepest-fact.txt`](runs/deepest-fact.txt) — the deepest fact of each `*_deeper_state` state, loaded from its saved file.
- [`runs/past_limit.saved-by-hand.ein`](runs/past_limit.saved-by-hand.ein) — a saved file one level deeper still, written by hand.
- [`runs/past_limit.saved-by-hand.solve.stderr.txt`](runs/past_limit.saved-by-hand.solve.stderr.txt), [`runs/past_limit.saved-by-hand.saturate.stderr.txt`](runs/past_limit.saved-by-hand.saturate.stderr.txt), [`runs/past_limit.saved-by-hand.saturate.txt`](runs/past_limit.saved-by-hand.saturate.txt) — its refusal, `solve` and `saturate` (the last: the two input lines `saturate` prints before it).
- [`runs/arity-error.saved-by-hand.ein`](runs/arity-error.saved-by-hand.ein), [`runs/arity-error.saved-by-hand.solve.stderr.txt`](runs/arity-error.saved-by-hand.solve.stderr.txt) — for comparison, a load error inside a carried text, placed as `<program saved in F>:2:2`.
- `runs/a8d.*.solve.txt`, `runs/a8d.*.dump/` — the earlier binary's read-outs and directories (`ein-answer/2`: `index.json`, `answer.txt`, `answer.json`, `nogoods.json`, `models/01.ein`) for the three programs: [`at_limit`](runs/a8d.at_limit.solve.txt) ([directory](runs/a8d.at_limit.dump/), [state file](runs/a8d.at_limit.dump/models/01.ein)), [`at_limit_deeper_state`](runs/a8d.at_limit_deeper_state.solve.txt) ([directory](runs/a8d.at_limit_deeper_state.dump/), [state file](runs/a8d.at_limit_deeper_state.dump/models/01.ein)), [`below_limit_deeper_state`](runs/a8d.below_limit_deeper_state.solve.txt) ([directory](runs/a8d.below_limit_deeper_state.dump/), [state file](runs/a8d.below_limit_deeper_state.dump/models/01.ein)).
- [`runs/a8d.at_limit.state.solve.txt`](runs/a8d.at_limit.state.solve.txt) — the earlier binary loading its `at_limit` state file: loads.
- [`runs/a8d.at_limit_deeper_state.state.solve.stderr.txt`](runs/a8d.at_limit_deeper_state.state.solve.stderr.txt), [`runs/a8d.below_limit_deeper_state.state.solve.stderr.txt`](runs/a8d.below_limit_deeper_state.state.solve.stderr.txt) — the earlier binary loading its state files with the derived fact: refused.
- [`runs/a8d.reads-today-saved.solve.stderr.txt`](runs/a8d.reads-today-saved.solve.stderr.txt), [`runs/a8d.reads-today-saved-module.solve.stderr.txt`](runs/a8d.reads-today-saved-module.solve.stderr.txt), [`runs/a8d.reads-today-saved-below.solve.stderr.txt`](runs/a8d.reads-today-saved-below.solve.stderr.txt) — the earlier binary given today's saved files.
- [`runs/diff.decision-Q-M1f.185.before-vs-now.diff`](runs/diff.decision-Q-M1f.185.before-vs-now.diff) — the decision file, the earlier tree's against today's.
- [`runs/at_limit.saved.solve.MOCK.option-b.stderr.txt`](runs/at_limit.saved.solve.MOCK.option-b.stderr.txt), [`runs/at_limit_deeper_state.saved.solve.MOCK.option-b.stderr.txt`](runs/at_limit_deeper_state.saved.solve.MOCK.option-b.stderr.txt), [`runs/with_deep_module.saved.solve.MOCK.option-b.stderr.txt`](runs/with_deep_module.saved.solve.MOCK.option-b.stderr.txt) — **mocks**, (b): the three saved files at the limit refused.
- [`runs/at_limit.saved.solve.a8d-vs.MOCK.option-b.diff`](runs/at_limit.saved.solve.a8d-vs.MOCK.option-b.diff), [`runs/with_deep_module.saved.solve.a8d-vs.MOCK.option-b.diff`](runs/with_deep_module.saved.solve.a8d-vs.MOCK.option-b.diff) — two mocks against the earlier binary's real lines (empty).

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/21-Q-M1f.418/rerun.sh`. The core commands:

```sh
B=/home/user/work/ein/build/review-bin/ein-head-b5fb8d70
A=/home/user/work/ein/build/review-bin/ein-a8d595eb; AS=/home/user/work/ein/build/wt-rev-a8d/stdlib
cd plans/m1f_hypothesis_and_documentation/review/01-save-and-load/21-Q-M1f.418
$B solve programs/past_limit.ein                                                   # exit 1
$B solve programs/at_limit_deeper_state.ein --dump-answer runs/at_limit_deeper_state.dump
$B saturate --dump runs/at_limit_deeper_state.dump/models/01.ein                   # loads; holds a fact 1001 deep
$B solve runs/at_limit_deeper_state.dump/models/01.ein --no-search --dump-answer runs/at_limit_deeper_state.resave
$B solve runs/past_limit.saved-by-hand.ein                                         # exit 1
EIN_STDLIB=$AS $A solve programs/at_limit_deeper_state.ein --dump-answer runs/a8d.at_limit_deeper_state.dump
EIN_STDLIB=$AS $A solve runs/a8d.at_limit_deeper_state.dump/models/01.ein          # exit 1, 1025:3001
EIN_STDLIB=$AS $A solve runs/at_limit.dump/models/01.ein                           # exit 1, 7:2998: what (b) keeps
```

The calculator's case, which Q-M1f.185 names (a request whose operands are near a thousand digits), is read, not run:
[`stdlib/README.md` § How long a number may be](../../../../../stdlib/README.md#how-long-a-number-may-be).

## 7. Found while building

- **The refusal of a carried text past the limit is placed in the saved file, not in the text.** A load error inside a
  carried program is placed as `<program saved in F>:LINE:COL`, the program's own line
  ([`arity-error.saved-by-hand.solve.stderr.txt`](runs/arity-error.saved-by-hand.solve.stderr.txt): `…:2:2`), as
  `10-answer-dump.md` says of every position in a carried text. The nesting refusal of `past_limit.saved-by-hand.ein`
  is placed in the saved file, `…saved-by-hand.ein:7:3001`, where the program's own line is 5
  ([`past_limit.saved-by-hand.solve.stderr.txt`](runs/past_limit.saved-by-hand.solve.stderr.txt) against
  [`past_limit.solve.stderr.txt`](runs/past_limit.solve.stderr.txt)): the outer parse, at 1001, refuses it before the
  text is parsed alone. The message still names *the limit of 1000*.
- The ledger's statements reproduce: a program at the limit is saved and loads; one past it, saved by hand, is refused;
  the rewritten Q-M1f.185 holds (a state 1001 deep is answered, saved and loaded again).

## 8. Your call

- **(a)** a saved file is parsed with one level more, its wrappers'; the limit is the texts' — **the default**.
- **(b)** the limit stays; a program or module at exactly 1000 has a saved file that does not load.
