# What waits for the user: the review bundle

Every item that waits for the user's call, one folder each, built so that each call can be made by reading the
folder: the items of the list *Waiting for you* given at the close of P1f.22 (2026-11-18) that **are still open**,
and the readings later stages took. The first drawings' items (34 to 55), the save-and-load items
(`01-save-and-load/`) and the numerals' (`02-numerals/`) were answered by the user on 2026-11-18 and left the
bundle, and the language and decisions' six (`04-language-and-decisions/`) on 2026-11-19; `03-drawings/` held what S1f.23.6 built of those answers (56 to 60), answered on 2026-11-19 and gone with them; the verdicts' two
(`05-verdicts-as-rules/`) moved into P1f.21's `review/`. An item whose question is resolved, by a ruling, by losing
its object, or otherwise, is removed from the bundle (the user's rule, 2026-11-18); where it went is in its ledger and
in `git log`. **Its pages and programs are committed; what its runs produce is ignored** (`.gitignore`: each item's
`README.md` and `programs/*.ein` are kept, everything else, outputs, dumps, drawings, logs, re-run scripts, is
re-taken by each item's re-run). The ledgers it cites (`../open_questions.md`, `../findings.md`, P1f.21's and
P1f.23's `findings.md`, the followups) are where each item is recorded.

**Re-taken with today's binary**, `ein-head-17085b27`, every P1f.23 stage merged: the canonical drawing and `--fold`,
the program view in three parts, `--models` and `--diff-models` removed, `ein test --diff`, the kernel reading no
theory's name. No item of the first look was resolved by that work: all 26 stay, each with what moved on its page
under *Found while building*.

## The topics

**Your answers go in each topic's `ANSWERS.md`**: one entry an item, its options and default, and a blank for
your answer; a re-render never touches it.


| topic | items | what they are about |
|---|---|---|

No topic is open: the drawings' five items S1f.23.6 put (56 to 60) were answered by the user on 2026-11-19 and left the bundle; S1f.23.11 builds the answers and puts its own items here.

Each topic's README has its items in a table, the default of each, and what building the item found that the ledger
does not say.

## One item's folder

`NN-<id>/` (the number is the item's place in the lists as they were given; 34 on are S1f.23.1's readings and R-18):

- `README.md`, the page to read: **the question** in plain words with a link to where it is recorded; **what is so
  today**, each claim linked to the run that shows it; **the options**, each with what it changes and what it would
  look like; **the default** and why; every file, a line each; **re-run**, the commands; **found while building**;
  and **your call**, one line an option.
- `programs/`, every `.ein` the page runs: copies of corpus programs (the page says from where) or small ones written
  for the item.
- `runs/`, every output raw, one file a command: the read-out, `--json-summary`, `--events` (and as a table),
  `--trace`, `ein saturate --dump`, `--dump-answer`, `ein show hyps`, `ein test`; before and today side by side,
  with `diff -u` between them.
- `renders/`, every view that bears on the item, `.dot` and `.svg`, each drawn with the layout its DOT suggests,
  `dot` for every view of today's binary ([`tools/draw.sh`](tools/draw.sh); a KB view of an earlier binary suggests
  `fdp`).
- `tools/` or `rerun.sh`, the script that made the runs: it re-takes them from the repo root.

**A file with `.MOCK.` in its name is not the binary's output.** It is the binary's output, or a page's drawing,
edited by hand to show an option that would need a change, and the page says what was edited. Nothing in the engine
was changed or built for this bundle.

## The binaries

| name | what it is | used for |
|---|---|---|
| `build/review-bin/ein-head-17085b27` | master today, every P1f.23 stage merged | every run marked *today* |
| `build/review-bin/ein-head-b5fb8d70` | master at the first look, before P1f.23: every object drawn once, labelled octagons, `--models` | *the first look*, and *before* of a drawing |
| `build/review-bin/ein-0a4a97a3` | before `written` was erased (Q-M1f.404), with the read-out's `types` row | *before* of the numeral questions and of Q-M1f.412; run with `EIN_STDLIB=build/wt-rev-0a4/stdlib` |
| `build/review-bin/ein-a8d595eb` | before save and load was merged: state files with each fact's English, `ein-answer/2` | *before* of the save-and-load items; run with `EIN_STDLIB=build/wt-rev-a8d/stdlib` |

The two earlier trees are the worktrees `build/wt-rev-0a4/` and `build/wt-rev-a8d/` (detached, gitignored under
`build/`); a re-run script of an item that compares needs them.

## Every item

| # | item | the question | default |
|---|---|---|---|
