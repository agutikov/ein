# S1f.21.19 — the review items on the built engine

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 on the user's word (*"after implementation complete add renders with the final ein
version. I will review questions after."*); **done**: every item has its runs on the final binary and its section
*On the built engine* ([the record](#record)).
**Waits on:** every build of the phase: [S1f.21.12](s1f.21.12_hypotheses_as_facts.md),
[S1f.21.18](s1f.21.18_the_hrule_form_removed.md), [S1f.21.13](s1f.21.13_the_search_and_rules.md), and the pages.
**Writes:** the review items of [the bundle](review/README.md): 58 to 65, 77 and 78, and the seven findings'
items, 79 to 85: their pages, their programs, their runs and renders. No engine file, no page of `docs/`.

## What it is for

The eight items were written before anything was built, so each shows a stand-in: `question` for `open`, a
hypothesis rule for what an obligation or the union would propose. The user took their defaults to let the build
run, and reviews the questions after it. This stage gives that review the real thing beside the stand-in.

## What it does

For each item, 58 to 65:

1. **Its programs in the spelling ruled**, which the built engine reads: each `*.target.ein` run as it is; each
   stand-in program written again with `open` where it wrote `question`, and with a rule where it wrote a
   hypothesis rule.
2. **Every run again, on the final binary**: the read-out, `ein show hyps`, `ein saturate --dump`, the oracle's
   read-out and its dump where the cube is small, the events and the summary, and **the renders**: the knowledge
   base, the lattice and the region drawn, as `.dot` and `.svg`.
3. **A section on its page, *On the built engine***: each number the stand-in gave beside the number the built
   engine gives, and where the two differ, why. A stand-in's number is never replaced: it stays, marked as what
   it was.
4. **Item 58's list taken again**: every corpus program that moved, as it answers before the phase's second part
   and after it, by the corpus as built.
5. **Item 64's program**, `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`, with what it answers: the user
   expects it still solvable, *"my expectation, not requirement"*, and reviews it here.

**Added 2026-11-20**, once every build was in master and the user had asked that everything that waits for an
answer be in the review folder:

6. **The seven findings that are the user's call**, items 79 to 82 of
   [part 1](review/part-1/README.md) and 83 to 85 of [part 2](review/part-2/README.md): each gets its
   `programs/`, its runs on the final binary, and its section *On the built engine* with what the binary says
   today. Where a finding's figure is no longer what the binary gives, both are shown.
7. **Item 77**: the other seven of the nine pictures,
   each as the page has it beside what `ein render kb` draws today, as its first two are; and what option (b)
   and option (c) would draw, composed from the drawing by leaving the `open` facts or the blind enumerator's
   out, marked as composed.
8. **Item 78**: the profile its figures are from, taken again on
   the final binary and kept in its `runs/`.

## What must hold

Nothing of the answers' file is filled in: the sections are the user's. No question is marked ruled. A number on a
page is in the item's `runs/`. `python3 utils/doc_audit.py --links --check --tracked` is clean.

## Record

### How it was taken

**The final binary** is the release build of `7b72bd84` (master's tip when the stage began, `./build.sh --engine`),
copied to the stage's worktree as `build/review-bin/ein-final-7b72bd84` and run with the checkout's `stdlib/`. Each
item's runs folder has its `--version`'s five lines (`version.txt`). **The engine before the second part**, where an
item compares with it (58, 77, 83, 84), is `build/costs/bin/ein-ddf09492`, run in the read-only worktree of
`ddf09492` with that tree's corpus and library.

**Where the runs are.** Each item's `runs/` and `tools/` are ignored by git and live in the main checkout only:
`plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/review/part-N/<item>/runs/` and `tools/`. Items 58 to
65 had runs from before the build: those are kept as they were, the new ones are in `runs/built/` and the tool that
re-takes them is `tools/run-built.sh`; the other items have `runs/` and `tools/run.sh` (77: `tools/render-all.sh`
beside its `render.sh`, which is unchanged). Each tool takes `EIN_BIN`, `EIN_REPO` (the corpus and library) and
`ITEM_SRC` (where the item's `programs/` is), defaulting to the checkout it sits in, so after the merge it re-takes
the runs from the repository's root with the binary at `ein.rs/target/release/ein`. Every number a page gives names
its file in `runs/`.

The items were taken by seven agents of this stage, each on its own items; the coordinator of the stage took item 78
and checked each page's figures against its runs before it was committed.

### What each item gained

| item | its section *On the built engine* | programs added |
|---|---|---|
| 58 | the list taken again, the corpus before the second part beside the final engine: 931 entries now (933 before), 925 on both sides, **46 moved**; 124 whose text the build changed and whose answer did not (116 by S1f.21.18); the prediction item by item: the union's 5 and the obligations' 8 as predicted, the 20 witnesses load; `13_witness_of_two`, `14_seats_nothing_proposes`, `13_mixed_solution_and_open` and `06_blind_enumeration` not as predicted, each because the build rewrote it as the page's default (a) said; moves not predicted: `absent-fact-nested.ein` (V-46), five refusals reworded by the `hrule` removal, six programs removed and four added | none |
| 59 | each target run as written, each the built spelling of its stand-in: *zebra2* 56 candidates, one state, 261 982 commitments, `--single` 7 saturations, as on the stand-in; the seats 16 candidates, 8 states, as the stand-in, and **not flagged** where the stand-in was; a state of $R$ lists a rule's `(open F)` facts and not an obligation's | none |
| 60 | the seven states of the page, each read three ways: its `open` facts are the page's composed ones at all seven; a state of $R$ prints them under `-p`, no state outside $R$ is printed by any command (V-91) | `drinks.built.ein` |
| 61 | the three targets flagged by the site rows the page composed, word for word; the warning's three rows; $R$ 2, 12 and 3 states as on the stand-ins; `noted` reads every `open` fact, not the *must* alone (V-92) | none |
| 62 | the seats cube's 16 nodes, each read as a saved state and as a root of its own: their `open` facts as the stand-in's table at all 16; the four dead nodes that owe hold `(open seats)` and what would pay it, Q-M1f.598's default (a) | `seats-asked.built.ein`, `probe-hypothesis-rule-absent.ein` (V-89) |
| 63 | the file as it was refused at the parse; the corpus file is now way **d** (Q-M1f.600's default); **b** and **c** as predicted, **c** loading; the Rust test's check read off the events | `a-as-it-was.ein` |
| 64 | the eight `oblig-*` programs: $H$, $R$ and the claim as the stand-in gave them on all eight; six newly flagged by `obligation-guard`, two flagged before gaining it; each read now by a `solve --count` run alone; `oblig-derived-domain.ein` shown plainly (below) | `oblig-derived-domain.a-rule-proposes.built.ein` |
| 65 | the twelve rows on the binary, each as the page's *once built*; Q-M1f.595 and Q-M1f.597 built as their defaults (a): program 9 refused, program 11 loads; `13_mixed_solution_and_open.ein` rewritten, its old text refused as `examples/broken/load/no_hypothesis_beside_open.ein` | five `.built.ein` |
| 77 | all nine pictures beside today's `ein render kb` drawing of their programs, root's `open` facts counted (one is 22, not 21: V-92); two pictures differ in more than the `open` facts, as they did before the second part (V-93); options (b) and (c) composed by a script for all nine, `tiny.ein` and `05_mini_zebra.ein` shown, each marked composed | `kinds.ein`, `neighbour.ein`, `neighbour-no-rule.ein`, `soup.ein` |
| 78 | the profile taken again, three runs a program: the key 37.9 to 39.2 % and the filter 8.9 to 9.8 % of `send_more_money.ein`'s instructions (S1f.21.23: 37.7 % and 9.3 %), 10.2 to 11.0 % and 2.2 to 2.9 % of `equation_two_unknowns.ein`'s (10.9 % and 2.1 %): the same within a sampled profile's point | none |
| 79 | the witness still not flagged and still answering what its row calls wrong; the `r1`-first twin answers the perfect model, the negation twin no longer does (V-90) | the three witnesses, and two variants of the negation twin |
| 80 | the page's 48 blocks of 21 programs against the static check today: 11 blocks of 5 flagged programs, **10 that print the `WARNING` row**, as the finding said; the page still what the binary writes | one scratch variant the page shows |
| 81 | **17 warning lines**, as the finding said, the same on six commands; the smallest program that warns once, and both imports, 18 | `algebra.ein`, `negation.ein`, `both.ein` |
| 82 | the program loads under one warning naming `std.algebra.imply1`, root derives `(in-slice-1 symmetric next-to)`, monotone; $H$ and $R$ *zebra2*'s | the program, and its `bijective` twin |
| 83 | the three pieces in the code and the 14 tests that say they license nothing, run and passing; the two programs that were their subject, on both engines; a census of the corpus on both: no route names L9 and no unlicensed instance is a debt that is an edge, as the finding said; L5's last reason still given, on 15 programs that search nothing (V-92) | none |
| 84 | every figure of V-48 as found, beside the engine before the second part: 56 → 112 candidates, 120 → 353 saturations, 36 groups and 5 grids → 352 and none, 386 → 33 305 coupling edges, 181 088 → 705 808 facts in root's layer; what folding gives for (b), two readings; two rewrites of `zebra.ein` for (c), neither giving 56 (V-92) | two rewrites of `zebra.ein` |
| 85 | a misspelt activator and one of another count each load with nothing said, $H$ 12 blind guesses where the program written right has 4; the finding's six on its own probe, which misspells an argument (V-92); with `is-a` declared the misspelling also moves the flag; the diagnostics page's sentences | seven programs |

### Item 58's list, taken again

Of the 925 corpus programs on both sides, **46 moved** between the engine before the second part and the final one
(`runs/built/census.md` of item 58 holds every figure):

- **by the union**, 6: `zebra.ein` (56 → 112 candidates, one state, 261 982 → 68 719 459 314 commitments),
  `13_witness_of_two.ein`, `14_seats_nothing_proposes.ein`, `08_absent_witness_hole.ein`,
  `06_friends_tables_everyone_sits.ein`, and `13_mixed_solution_and_open.ein` as rewritten;
- **by an obligation that no longer steps aside**, 8: the `oblig-*` witnesses, six of them newly flagged;
- **`open` written elsewhere, refused before and loading now**, 21: the page's twenty and `absent-fact-nested.ein`;
- **refused on both sides with another line**, 11.

Besides those, 2 moved to another path (Q-M1f.614), 6 were removed and 4 added. No program under `tests/` moved.

### `oblig-derived-domain.ein` on the final engine

Flagged by one site, `owes-food: obligation-guard reads food at absent depth 1`, so nothing is claimed of its
answer, and the read-out, stderr and the summary say so. $H$ is one candidate, `(likes Ann Soup)`, written by the
obligation; it is dead alone (`no-soup`). **$R$ is empty**: root owes 1, *Ann owes a liked food*, and every alive
node owes. Its `:expect`, `(model (likes Ann Pie))`, prints `FAILED`. `--in-flight` finds nothing. It answers the one
state `(likes Ann Pie)`, still flagged, if Pie is a food at root (the `-declared` twin) or if a rule proposes what Ann
may like beside the obligation (`oblig-derived-domain.a-rule-proposes.target.ein`): as the stand-in said and as
item 69 reports.

### Defaults taken (Q-M1f.680 to Q-M1f.684)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.680 | how a stand-in is given to the built engine | kept unchanged with its runs, and written again beside it as `NAME.built.ein` only where the built spelling is no program the item already runs (a `*.target.ein`, a corpus file); the built runs in `runs/built/`, the tool `tools/run-built.sh` | each stand-in rewritten in place, its old runs then of a file that is gone |
| Q-M1f.681 | how the `open` facts of a state above root are read where no command prints them (items 60, 62) | two readings beside what the commands print: the state made a root (its commitment written as facts) and a saved state with its `(commit …)` replaced, each by `ein saturate --dump` | the commands' own surfaces alone, which show none outside $R$ (V-91) |
| Q-M1f.682 | what counts as a move in item 58's census | loading or refused, $H$ as a set of facts, the flag, $R$'s counts and states, a claim's verdict; a refusal reworded only in its position or the word *hrule* is not a move (16); a writer's name alone is listed and is not (`hrules` read as `hypothesis-rules`, Q-M1f.626); programs added, removed or moved apart listed apart | every changed refusal line a move (27) |
| Q-M1f.683 | how item 77's options (b) and (c) are composed | by a script that reads today's DOT back into facts and draws it again by the drawing's own rules, frames and layers computed again, the first line of each file saying it is composed and not drawn by the binary; the second program `05_mini_zebra.ein`, and all nine composed | deleting the `open` lines from the DOT, which leaves frames and layers as they were |
| Q-M1f.684 | what an item runs beyond its page's programs, and what it keeps | a variant where it shows why a figure moved or what an option would do (79's negation variants, 81's smallest and both-imports programs, 82's `bijective` twin, 84's rewrites of `zebra.ein`, 85's probe and `is-a` variants, 62's probe); an event stream or a dump past about 50 MB is not kept, and a drawing Graphviz did not lay out within 600 s is kept as its `.dot` alone, each said on its page | the item's programs alone, every artefact kept |

### Findings outside the stage (V-89 to V-94)

- **V-89 — a rule that concludes `(open F)` reads none of its `absent` premises.** `HypothesisRules::candidates_by`
  (`ein-infer/src/hypothesis_rules.rs`) runs the matcher and resolves each match's `(open F)` without the `absent`
  guards the plan lifts out of the match, which the saturator checks for a rule that concludes a fact. Item 62's
  probe, `programs/probe-hypothesis-rule-absent.ein`: with `(q a)` written, `(and (p ?x) (absent (q ?x)))` holds of `b`
  alone, and the rule that concludes a fact concludes `(s b)` alone, but the rule that concludes a hypothesis writes
  `(open (r a))` and `(open (r b))`: $H$ is 2 and $R$ 4 states where the match gives one hypothesis and 2 states. The
  program is monotone, so the engine claims that answer. The engine at `edbe6c3b`, the rule written as an `hrule`,
  proposes the same two. It contradicts `02-syntax.md` (a hypothesis rule's `:match` reads the state it is asked
  about) and Q-M1f.129; on that decision's witnesses it is masked where the `absent` reads the hypothesis's own fact,
  which $H$'s filter removes. **A wrong answer by the register's rule; not filed: the coordinator's call.** Runs:
  item 62's `runs/built/probe-hypothesis-rule-absent.*` and `runs/built/probe-standin/`.
- **V-90 — the agenda's first half reads `std.contradiction`'s activators as written, not as derived.** With the bare
  `(import std.contradiction)` the activators of `clash-1` are derived at root by `clash-of-1` after the rules that
  wait on an `absent` are enqueued, and the rule that concludes a stored negation is not admitted first; with
  `(clash-1 w)` written it is. On item 79's negation twin, monotone and `:no-search`, root holds `(p one)` in the first
  case and none in the second (`runs/fifo-gap-root-negation-imported.events.jsonl`, `-written.events.jsonl`). With it,
  the register's twin of `W8/fifo-gap-root`, `fifo-gap-root-negation.ein`, no longer answers the `(q one)` alone its
  row says (it imports nothing since S1f.21.11, so its stored negation is in the second half), and the headers of the
  witness and the twin still say `r1` is admitted first; `utils/wrong_answers.py` holds each row's answer, not its
  twins', so it stays green.
- **V-91 — no command prints the `open` facts of a state above root outside $R$.** `-p` and the answer's dumps print
  the states of $R$; the oracle's `--states` keeps a node's facts beyond root's, so it lists no `open` fact root also
  holds and cannot show one the node has dropped (the node `{(seats Ann C1)}` holds four, its record lists none);
  `--in-flight` shows no more. Q-M1f.572 has every listing of a state list its `open` facts, and the oracle's `--states`
  is among the printed states (Q-M1f.643). Runs: item 60's `runs/built/states/obligation-1.*`, item 62's
  `runs/built/nodes/`.
- **V-92 — what the runs correct in the findings and pages the items were written from.** V-47: L5's last reason is
  still given, on 81 instances of 15 programs, all under `tests/`, each `:no-search` with an empty $H$, so it steers no
  search (item 83's `runs/census.final.*`); L9 and the coupling as found. V-48: the mirror is written by a second
  obligation, `std.slots`' `slot-owed-fill` (`slot-owed-room` writes the rule's orientation), so `zebra.ein` with its
  rule deleted still has 112 candidates and option (c) is not had by its own writers (item 84's
  `runs/zebra.rule-deleted.*`); folding *two candidates that imply each other* gives 53, folding a fact with its mirror
  56 (`runs/zebra.folds.txt`). V-58: its probe misspells an argument, not the rule's name or its count, and a note on a
  rule with no activator would not fire on it (item 85's `runs/the-findings-probe.*`). V-85: `01-before-firing`'s
  program holds 22 `open` facts at root, not 21 (item 77's `runs/open-counts.txt`). Item 61: its *must* program's
  `noted` reads every `open` fact, the hypotheses too, and concludes five at root (`runs/built/r-held-and-read.target.saturate-dump.txt`).
- **V-93 — pages that say what the binary does not, outside the `open` facts.** `user/02-guide/02-first-rules.md`'s
  `02-co-located-fired` notes two derived facts with the other rule than the binary draws, as the engine before the
  second part did; `reasoning/03-rule-evolution/01-rules-as-data.md`'s `01-instance-of-instance`, called every fact of
  it, lacks four activators of `std.contradiction` (item 77's `runs/page-lines.txt`,
  `runs/kinds.composed-b.page-diff.txt`); `user/04-reference/08-read-out.md`'s `route` row says L9's relation is one
  *the kernel does not mirror*, where the kernel mirrors none (Q-M1f.436); `ein show --help` and `ein show hyps --help`
  say the note names *which rung* built H, which it no longer does (item 85's `runs/show-hyps.help.txt`).
- **V-94 — `obligation_reports.rs` checks a proposal against the obligation rules' short names.**
  `06_blind_enumeration.ein` imports its modules whole, so a proposal that named an obligation rule would name it in
  full and pass the check unseen. Item 63's `runs/built/hyps-check.txt` checks full names too: none is proposed.

### What the user must see

- **V-89**: a monotone program whose hypothesis rule reads an `absent` gets candidates its match excludes, so its $R$
  is not what the program says, and the engine claims it (item 62, its probe).
- **`oblig-derived-domain.ein`** is not solvable on the built engine: $R$ is empty, flagged (item 64).
- **Item 58**: the corpus moved as the rulings said, the four exceptions being programs the build rewrote.
- **Item 77**: today's drawings of the nine pictures, and (b) and (c) composed, to answer its question on.
- **V-90**: on a monotone program the answer moves with whether an activator of `std.contradiction` is written or
  derived (item 79).

### What was not done

No oracle or region of a program whose $H$ passes 16 candidates (*zebra*, *zebra2*, item 85's `is-a` variants); no
tests on the engine before the second part; no `.svg` of the two coupling graphs of `zebra.ein`'s structure, which
Graphviz did not lay out within 600 s. No engine, library, corpus or `docs/` file changed, no `ANSWERS.md` was
touched, and no question was marked ruled. `python3 utils/doc_audit.py --links --check --tracked` is clean on the
branch.
