# What waits for the user: the review bundle

Every item of the list *Waiting for you* given at the close of P1f.22 (2026-11-18), one folder each, built so that
each call can be made by reading the folder. **This folder is gitignored** (`/plans/*/review/`): nothing here is
committed; the ledgers it cites (`../open_questions.md`, `../findings.md`, P1f.21's `findings.md`, the followups)
are where each item is recorded.

## The topics

| topic | items | what they are about |
|---|---|---|
| [01-save-and-load](01-save-and-load/README.md) | 11 | the saved file: what it prints, what reads it, what went with the state files (a fact's English) |
| [02-numerals](02-numerals/README.md) | 8 | the calculator's check of a numeral after `written` was erased |
| [03-drawings](03-drawings/README.md) | 6 | `ein render`'s views and the pages' drawings |
| [04-language-and-decisions](04-language-and-decisions/README.md) | 6 | a relation's count, a non-monotone case, the `is-a` row, three decisions' sentences |
| [05-verdicts-as-rules](05-verdicts-as-rules/README.md) | 2 | P1f.21's two questions: what an obligation emits; the names `false` and `open` |

Each topic's README has its items in a table, the default of each, and what building the item found that the ledger
does not say.

## One item's folder

`NN-<id>/` (the number is the item's place in the list as it was given):

- `README.md`, the page to read: **the question** in plain words with a link to where it is recorded; **what is so
  today**, each claim linked to the run that shows it; **the options**, each with what it changes and what it would
  look like; **the default** and why; every file, a line each; **re-run**, the commands; **found while building**;
  and **your call**, one line an option.
- `programs/`, every `.ein` the page runs: copies of corpus programs (the page says from where) or small ones written
  for the item.
- `runs/`, every output raw, one file a command: the read-out, `--json-summary`, `--events` (and as a table),
  `--trace`, `ein saturate --dump`, `--dump-answer`, `ein show hyps`, `ein test`; before and today side by side,
  with `diff -u` between them.
- `renders/`, every view that bears on the item, `.dot` and `.svg`.
- `tools/` or `rerun.sh`, where a script made the runs: it re-takes them from the repo root.

**A file with `.MOCK.` in its name is not the binary's output.** It is the binary's output edited by hand to show an
option that would need a code change, and the page says what was edited. Nothing in the engine was changed or built
for this bundle.

## The binaries

| name | what it is | used for |
|---|---|---|
| `build/review-bin/ein-head-b5fb8d70` | master today | every run marked *today* |
| `build/review-bin/ein-0a4a97a3` | before `written` was erased (Q-M1f.404), with the read-out's `types` row | *before* of the numeral questions and of Q-M1f.412; run with `EIN_STDLIB=build/wt-rev-0a4/stdlib` |
| `build/review-bin/ein-a8d595eb` | before save and load was merged: state files with each fact's English, `ein-answer/2` | *before* of the save-and-load items; run with `EIN_STDLIB=build/wt-rev-a8d/stdlib` |

The two earlier trees are the worktrees `build/wt-rev-0a4/` and `build/wt-rev-a8d/` (detached, gitignored under
`build/`); a re-run script of an item that compares needs them.

## Every item

| # | item | the question | default |
|---|---|---|---|
| 01 | [Q-M1f.414](01-save-and-load/01-Q-M1f.414/README.md) | a fact's English, which a state file wrote in a comment, now only in the trace | (a) the trace |
| 02 | [L-102](01-save-and-load/02-L-102/README.md) | the writer of a fact's English that no command uses | the code goes, with Q-M1f.414 (a) |
| 03 | [L-105](01-save-and-load/03-L-105/README.md) | `utils/nl_census.py` reads comments that are gone | retired, with Q-M1f.414 (a) |
| 04 | [Q-M1f.405](02-numerals/04-Q-M1f.405/README.md) | which numerals are taken from the program, and *once* | (a) |
| 05 | [Q-M1f.409](02-numerals/05-Q-M1f.409/README.md) | an integer's and a rational's check and label | (a) |
| 06 | [L-107](03-drawings/06-L-107/README.md) | the extent's drawing: $g$, which no frame can mark | as built: grey, frame kept |
| 07 | [L-106](01-save-and-load/07-L-106/README.md) | the round trip's nightly shards near five minutes | as it is |
| 08 | [V-7](04-language-and-decisions/08-V-7/README.md) | a relation only an activator names is not counted | as is (leaning: the registry takes it) |
| 09 | [F21 N-15](04-language-and-decisions/09-F21-N-15/README.md) | *everybody's thoughts are false* with `absent` is dead: twin of `W8/fifo-gap-root`? | none; nothing filed |
| 10 | [L-96](03-drawings/10-L-96/README.md) | a literal `\n` in the KB view's octagon labels | as is |
| 11 | [Q-M1f.406](02-numerals/11-Q-M1f.406/README.md) | the label, and where a made number gets it | (a) |
| 12 | [Q-M1f.407](02-numerals/12-Q-M1f.407/README.md) | a numeral no operation takes is not checked | (a) |
| 13 | [Q-M1f.408](02-numerals/13-Q-M1f.408/README.md) | where the check's rules live | (a) |
| 14 | [Q-M1f.410](02-numerals/14-Q-M1f.410/README.md) | no join on a number; a made number handed on is walked again | (a) |
| 15 | [Q-M1f.411](02-numerals/15-Q-M1f.411/README.md) | a numeral with two faults has one flag | (a) |
| 16 | [Q-M1f.412](04-language-and-decisions/16-Q-M1f.412/README.md) | the `is-a` row dropped: does a schema id move? | (a) no id moves |
| 17 | [Q-M1f.413](01-save-and-load/17-Q-M1f.413/README.md) | `ein kb save` of a saved file | (a) a root's is stored, a node's refused |
| 18 | [Q-M1f.415](01-save-and-load/18-Q-M1f.415/README.md) | `ein show hyps` / `ein render h-space` of a saved file | (a) the program's H |
| 19 | [Q-M1f.416](01-save-and-load/19-Q-M1f.416/README.md) | what `ein test` asks of a saved node | (a) root alone, NOT CHECKED, exit 0 |
| 20 | [Q-M1f.417](01-save-and-load/20-Q-M1f.417/README.md) | the `commit` row, `root.commit`, `ein-answer/3` | (a) |
| 21 | [Q-M1f.418](01-save-and-load/21-Q-M1f.418/README.md) | the nesting limit and a saved file's wrappers | (a) one level more |
| 22 | [L-92](04-language-and-decisions/22-L-92/README.md) | Q-M1f.362's sentence on the index | kept |
| 23 | [L-93](04-language-and-decisions/23-L-93/README.md) | two stale sentences of the pages (a third found) | left |
| 24 | [L-94](04-language-and-decisions/24-L-94/README.md) | Q-M1f.119 misses a fact's argument | kept |
| 25 | [L-97](03-drawings/25-L-97/README.md) | nine `div9-step` rules have no drawing | the bound stays |
| 26 | [L-98](03-drawings/26-L-98/README.md) | the program view is several digraphs | as is |
| 27 | [L-99](03-drawings/27-L-99/README.md) | the derivation views keep a fact's text in a node | none |
| 28 | [L-100](03-drawings/28-L-100/README.md) | the beliefs' two KB drawings have no test | as is |
| 29 | [L-101](02-numerals/29-L-101/README.md) | `int-check-parts` / `rat-check-parts` ask non-numerals | as they are |
| 30 | [L-104](01-save-and-load/30-L-104/README.md) | `saturate --dump` prints `:hypothesis 0` for every choice | as is |
| 31 | [F23 D-24](01-save-and-load/31-F23-D-24/README.md) | the carried-module note when `$EIN_STDLIB` points nowhere | as is |
| 32 | [Q-M1f.390](05-verdicts-as-rules/32-Q-M1f.390/README.md) | what an obligation emits | **not ruled**; recommended (b) |
| 33 | [Q-M1f.399](05-verdicts-as-rules/33-Q-M1f.399/README.md) | the names `false` and `open` | **not ruled**; leaning `(ein.kill)` |
