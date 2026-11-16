# S1f.10.12 — The structure shown, and its surface

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** T1, the bundle, ran on 2026-11-16 ([its record](#record-2026-11-16-the-bundle)). T2, the stop, is the main session's: the surface was ruled (a), a view of `ein render`, on 2026-11-16 ([Q-M1f.309](../open_questions.md#q-m1f309--how-ein-shows-a-programs-structure)). **T3, the view, and T4, the page, ran on 2026-11-16** in a worktree, on that ruling ([their record](#record-2026-11-16-the-surface-and-the-page)): `ein render structure FILE` and [`docs/kernel/inference/structure.md`](../../../docs/ein/reasoning/kb-evolution/hypothesis-space.md). **Not run there, and owed before the batch is done**: `./run_tests.sh`, `utils/ci_container.sh`, which a DOT writer's change makes part of done, and the wall of the two nightly steps that grew, which was not measured. **Renamed and extended on 2026-11-16** on the user's instruction of that day ([the record](#record-2026-11-16-the-view-renamed-h-dumped-plain-k-17)): the view is `ein render h-space FILE`, `ein show hyps FILE` lists H as Ein facts, and K-17 is fixed.
**Depends on:** for its bundle, [S1f.10.1](s1f.10.1_exclusion_census.md), [S1f.10.2](s1f.10.2_groups.md),
[S1f.10.10](s1f.10.10_the_theorys_share.md), [S1f.10.4](s1f.10.4_bounded_groups.md),
[S1f.10.11](s1f.10.11_coupling.md); for its stop, also [S1f.10.13](s1f.10.13_the_price_list.md) and tasks 1 to
3 of [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md),
[S1f.10.6](s1f.10.6_obligations_under_hypothesis.md) and
[S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md).

## Context

The user's first idea: *the structure is itself a valuable piece of information for analysis*. Information is
valuable when somebody reads it, so this stage puts the structure in front of the user on real programs,
computed by the engine and not invented, **before** any surface of `ein` is designed around it. What the user
finds worth looking at decides the surface.

## Tasks

### T1f.10.12.1 — the bundle

`build/structure/`, written by one script from the files the track's stages left, as `utils/review_bundle.py`
writes `build/review/`: an index, a page per instance, and a survey row per corpus program that searches. Each
page, for one program:

- **H listed**, every candidate with its marks: dead alone; what it implies and what it is equivalent to; the
  groups it is in; whether any group it is in is owed; the junk class S1f.10.9 gives it, if any.
- **The exclusion graph**, drawn, the theory's pairs and the puzzle's in two strokes, each puzzle's edge
  labelled with the stated facts it rests on.
- **The cover**: every group as a table of its members, with *owed*, *licensed* and *declared by*; the grids
  as grids.
- **The hypergraph of groups**, drawn, each edge with its reason.
- **Beside what the program declares**: found and declared, declared and not found, found and declared
  nowhere.
- **What it cost** to compute, beside what the program's describe and `--single` cost.
- Every picture as SVG with its DOT beside it, every table with its JSON beside it, and a header on each
  saying what it is and where the raw file is.

The instances first, I-Z1, I-Z2, I-Z2M, I-B06, I-L02, I-SEND, and then every program, so that the survey can
be sorted by any column.

### T1f.10.12.2 — the stop

One message to the user, with the bundle, and four questions, each under the next free id in
[`open_questions.md`](../open_questions.md), each with its default, **none acted on before the reply**:

1. **The surface.** How `ein` shows a program's structure.
   - (a) **a view of `ein render`**, `ein render structure FILE`, writing what a page of the bundle holds: the
     graphs and a `structure.json`. The renderer already draws a program, a KB, a region and a lattice; the
     structure is a fifth thing to draw. `ein solve` is untouched and computes no structure it was not asked
     for.
   - (b) **rows of `ein solve`'s read-out**, a `structure` row with the counts, and the lists with
     `--dump-answer`.
   - (c) both.
   - (d) no surface: the bundle's script stays the way to see it.

   **Default: (a).** It keeps each question's work the least it needs, and it does not put a count in the
   read-out that a reader cannot act on.
2. **What the blind enumerator takes for an object** (S1f.10.9's question).
3. **A debt root was not told of** (S1f.10.6's question, Q-M1e.11).
4. **The rung a loaded rule chooses** (S1f.10.15's question).

And two things shown, with no question unless the user has one: S1f.10.13's verdicts, with the tenth in its
rule; and every default the stages of part 1 took.

### T1f.10.12.3 — the surface, as ruled

Under (a) or (c): the view in `ein-render`, reading the one owner S1f.10.2 named; a test that holds the view's
`structure.json` to the census on every instance; `dot_wellformed` over its graphs, which makes
`utils/ci_container.sh` part of done for this task; the view in `ein render --help`. Under (b) or (c): the
row, with [`read_out.md`](../../../docs/kernel/inference/read_out.md), `read_out_examples.md` and their
reference test in the same commit.

### T1f.10.12.4 — the page

One page under `docs/kernel/inference/` that defines the structure: each object of
[§ The structure](README.md#the-structure), the domain, what it costs, how to see it. In the current-state
voice: it states what the engine does and cites decisions by id, with no stage id, no path into `plans/` and no
date. Its numbers each carry the command that re-takes them. P1f.20 moves it with the rest.

## Acceptance

- The bundle exists for every instance and every corpus program that searches, each artefact raw and rendered.
- The user has seen it and ruled the four questions; each ruling is recorded in `open_questions.md` in the
  user's words.
- The surface, if one is ruled, shows on I-Z2 the five declared bijections as found and declared, and is held
  to the census by a test in the gate.
- The page exists and `utils/doc_audit.py --links --check` is green.
- No answer changes. Under (b) or (c) the read-out gains a row and every golden that holds a read-out moves,
  each named before it moves.

## What it must not do

- **Not design the surface before the stop.**
- **Not curate the bundle.** Every artefact, with navigation; a summary hides what the user is looking for.
- **Not compute a structure in `ein solve` that its question did not ask for.**

## Record (2026-11-16): the bundle

T1 ran. `build/structure/index.html` is the bundle's index, and everything else the script writes is under
`build/structure/bundle/`, which it owns and replaces whole; no file of an earlier stage is rewritten by it. No
surface of `ein` was designed, and nothing in the engine, a route or an answer changed. The working tree over
`51fe98b0`, on AC; every number below is a count.

### The command

```sh
python3 utils/structure_bundle.py            # writes build/structure/index.html and build/structure/bundle/
python3 utils/structure_bundle.py --check    # every link, every fragment, every table's JSON, every picture's DOT
```

[`utils/structure_bundle.py`](../../../utils/structure_bundle.py), one script, the standard library and Graphviz;
a row of [`utils/README.md`](../../../utils/README.md)'s renderers. **It computes no structure**: it reads the
files `pair_layer` and `price_list` wrote under `build/structure/`, track H's censuses under `build/s1f.10.9/`,
`build/s1f.10.6/` and `build/s1f.10.15/`, and this phase's Markdown; it joins the engine's lists by a candidate's
text and writes the DOT of its pictures. Every page states the commands, the two censuses' fingerprints and
`ein --version`'s lines. A missing or malformed input stops it, non-zero, naming the file and the key; a DOT
Graphviz refuses stops it with Graphviz's message. **In no tier**: the gate and the nightly do not run it, and no
test reads it.

### What the bundle holds

**1 179 pages**, 179 MB, 8 838 files with the index:

| | pages |
|---|---:|
| the index: the six instances first, then every program, with the pages below | 1 |
| **a page a corpus program that searches** | 188 |
| the engine's own Markdown twins rendered, five a program (`theory`, `cover`, `exactly_one`, `coupling`, `prices`) | 940 |
| root's cliques, a page a program that has one | 44 |
| the survey; the price list; the price list, every table; every default; the findings; the four questions' material | 6 |

**A program's page**, each part with a header that says what it is and names the raw file and key, each table
with its JSON beside it (`tables/<name>.json`, the rows as shown), each picture an SVG with its DOT beside it:

- **H listed**: every candidate with its mark alone at root, whether the theory kills it alone, what it implies,
  what implies it, its equivalence class, the pairs it is in by stroke, its groups, whether any is owed or an
  unlicensed instance is about one, its questions, the edges through them under both readings, and S1f.10.9's
  mark. The pair layer's other lists and the census line's counts are under it.
- **The exclusion graph** in two strokes: a pair of the theory a solid blue line, dotted where root's layer does
  not list it (a member dead alone at root, on every corpus program); a pair of the puzzle a dashed red line
  labelled with the ids of the statements its support names. Beside it every pair with its support whole (the
  dropped statements, the kept ones, the rules, the search's status, the check), every statement the program
  writes with the small KB's verdict and clause, and the interacting pairs. A support is read for all 5 565 pairs
  root excludes on the 188, so no page's labels are absent; the 25 cut at the search's budget are labelled with
  what they name.
- **The cover**: every group with its members and marks, *owed* (and by which rule), *licensed* (the instances,
  or the unlicensed instance that is about it), *declared by* (the mechanisms the engine reads of its relation,
  and what the program states of it by name); **the grids drawn as grids**, one family the rows and the other
  the columns, a member the cell they share, a bijection marked; the components, each a grid or none with the
  engine's reason; the candidates in no group; the theory's pairs alive at root; root's cliques on a page of
  their own.
- **Exactly one**: every instance root owes with its alternatives, or L5's reason and the group it is about, and
  every witness with what root says of it and the candidates whose node holds it.
- **The hypergraph of questions**: the questions; the engine's own `coupling.dot` drawn as it is; the edges of
  the whole node and the edges one application back, each as a picture, a table (kind, arity, the ties and ways
  it covers, its debts with *licensed* or not, the statements most named) and, for the edges of two questions, a
  matrix; the debts; both sets of components; the ties.
- **Beside what the program declares**: the mechanisms; every relation declared `bijective` as *found and
  declared*, *declared and not found* with what broke it, or *declared with nothing left in H*; every grid as
  *found and declared*, *found and declared nowhere*, or no bijection; and the groups found against the debts
  stated.
- **What it cost**: each object's saturations, facts and walls beside the program's describe, `--single` and
  `--maximal`; the six questions' cells with the split of each; each way of spending on this program with the
  corpus's verdict for it and whether the program is one of its losers.
- **Track H's census rows** for the program, whole, and **every file** of it, raw.

**The survey**: 644 rows, one a corpus file that loads or is refused, and 190 columns, sortable by any, by
inline JavaScript and no fetch; the groups of columns can be hidden, and the 437 programs that load and do not
search and the 19 the engine refuses have a row and no page. `survey.json` and `survey.tsv` are beside it. The
price list's columns are, for each question, the cell's saturations and facts and, for each of the fourteen ways,
how many saturations more or fewer the cell is with the way.

**From the repo's Markdown**, each with a link back to its source: the price list (`verdicts.md` and
`losers_named.md`, and `prices.md` whole on a second page); **every default part 1 took**, seven sections of five
records verbatim under their stages with the section's Markdown folded beneath, and Q-M1f.305 and Q-M1f.310
(S1f.10.1's record has no section so headed and is linked to *What the definition had to settle*; track H's three
stopped at their questions); `findings.md` whole; and the four questions' material, Q-M1f.306, .307 and .308
rendered as `open_questions.md` holds them with every witness under `build/s1f.10.*/` listed and linked. **The
surface's question is a marked slot**, with the stage file's options linked and Q-M1f.309 rendered under it as
the main session wrote it.

**6 669 tables**, each with its JSON. **571 pictures drawn**, each from a DOT beside it: 189 of the exclusion graph
on the 54 programs that have an excluded pair (the other 134 pages say there is nothing to draw), 186 of the
engine's `coupling.dot`, 92 of the whole node's edges and 104 of the edges one application back. A picture of
more than 150 edges is also drawn in parts: the exclusion graph inside each component of the theory's graph and
between each two, the hypergraph by arity and by component.

**16 pictures are kept as DOT and not drawn**, each over the 1 000 edges `dot_wellformed.rs` bounds a layout at,
each with a `.undrawn.txt` beside it and a sentence on its page:

| program | not drawn | drawn instead |
|---|---|---|
| `zebra2-minus-15.ein`, `zebra2-minus-15-obligations.ein` | the exclusion graph whole (1 428 edges) and the puzzle's pairs alone (1 110); the edges one application back whole (1 274) | the theory's pairs alone, the five components and the ten pairs of components; the edges of three questions or more and of two |
| `ein-bugs/wrong-answers/count-past-2-64.ein`, `count-past-2-128.ein` | the engine's `coupling.dot`, and under both readings every edge and the edges of two questions (2 016 and 8 385) | the edges of three questions or more; the tables, and on the first the matrix |

### What is absent, and why

- **The nodes of the price list's runs are linked and not rendered**: 1 128 files, 693 MB, one line a node.
- **The matrix of the edges of two questions** is written for 64 questions or fewer; `count-past-2-128.ein` and
  `saturation/square-unique/terminus.ein` have the table alone.
- **A tie that implies or interacts names no statement**: no support is read for one (S1f.10.11, *What was not
  done*), and its cell is empty.
- **No page for a program that does not search**, by the task; and nothing of the 19 refused programs but the
  engine's refusal in the survey.
- **The surface's question**, which is T2's.

### The census was re-taken, for one field

`coupling.json` named an edge's questions by their text, and two questions of one program can share a text: the
three kinds of attribute at one house of `zebra.ein` ([`findings.md`](findings.md)'s K-14). On twelve programs the
file did not say which questions an edge is of, and `coupling.dot`, which does, has no edge one application back.
So the example got one field, `question_ids`, an edge's and a debt's questions by place, and the census was taken
again by its documented command:

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check --readings \
    > build/structure/census.jsonl 2> build/structure/census.env
```

2026-11-16 from 01:53, one P-core (`cpu4`), on AC. **Nothing else moved**: the 644 lines are the earlier census's
but for their walls; of the 6 875 files it writes, 6 250 are the same bytes and the 625 `coupling.json` are the
same but for the new field and a wall (the census as it was, with every file, is kept under
`build/structure/s12/`). The price list was not re-taken: it reads no file of the census. **The earlier records'
`build/structure/census.env`, *from 00:16*, is now this run's**, and the walls of a census line are this run's.

### Checked

| | |
|---|---|
| two runs from an empty bundle directory, and one more over it | one digest of all 8 838 files |
| `--check` | 1 179 pages, 260 121 links and images resolved against the tree, every fragment an element's id, every picture with its DOT, every JSON parsed: 0 errors |
| the instances' pages against the committed records | I-Z2: 56 candidates, 21 dead alone, 128 pairs of the theory and 282 of the puzzle, 36 groups all owed, 5 grids all bijections, 392 edges of two questions and one of 27. I-Z1: 18 owed groups and 18 nothing owes, 18 unlicensed instances each about one, 374 edges of two, one of 32 and 11 debts' of 9 to 22. I-Z2M: 46 groups all owed, 5 bijections, 354 and one of 45. I-B06: 10 groups, none with an open member. I-L02: no group, one edge of its three candidates. I-SEND: 18 groups, the 8 letters' owed, 5 edges of 2, 3, 4, 5 and 7 questions, each naming its column. **No page disagrees with a record** |
| the script's reading by name, summed over the 188 pages | 29 relations declared bijective, 20 found and declared, 8 declared and not found, 1 with nothing left in H, 1 found and declared nowhere: S1f.10.4's T2 |
| S1f.10.9's marks carried to candidates | every set's count is the census's own on all 188 programs; 8 have a marked candidate |
| a missing file, a truncated JSON, a missing key, a value of another type, a census without `question_ids`, a `dot` that fails, a `bundle/` that is not the script's, a missing binary | each stops the script, exit 1, naming the file |
| the survey's sort and its column groups | by a headless browser: ascending, descending, a group hidden |
| `python3 -m py_compile`, `cargo fmt --all --check`, `cargo clippy -p ein-infer --all-targets -- -D warnings`, `utils/doc_audit.py --links --check` with the gate's argument list | clean |

### Defaults taken

Each the user's or the main session's to revise.

1. **Where it writes**: the index at the top of `build/structure/` and all else under `bundle/`, marked as the
   script's and replaced whole each run, so that no stage's file is touched and a stale page cannot stay.
2. **`question_ids` in the example, and the census re-taken**, over drawing the edges one application back by
   their texts, which on I-Z1 would merge three groups into one box.
3. **The bounds**: no layout past 1 000 edges, the repo's own bound for a DOT view; parts drawn past 150; a table
   folded past 40 rows; a matrix up to 64 questions. Constants of the script.
4. **The layout** is `dot` up to 60 edges and `neato` with straight edges and `overlap=scale` past that: Graphviz
   16.0.0 lays one DOT out differently from run to run under `fdp`, and under `neato` with `overlap=false` on a
   few graphs (K-15), and the bundle is held to its bytes. A dense picture is large, and it is the same picture
   twice; a page shows it at the page's width and links the SVG at its own size.
5. **A puzzle's pair is labelled with statement ids**, `s3 s7`, and the statements are the table beside: a
   support of `zebra.ein` names up to seventeen facts.
6. **A candidate's box is filled by its component of the theory's graph**, a dead one dashed and grey.
7. **Two readings by name are the script's and are marked so on every page**: a relation off a candidate's text
   with what the program states of it, ported from S1f.10.4's `bijections.py`, and S1f.10.9's marks, whose census
   lists marked names a program and not marked candidates. Each is held to the stage's own totals.
8. **The engine's Markdown twins are rendered as pages** beside the script's tables, so that nothing the engine
   tabled is left to the script's choice of columns.
9. **The survey's verdict cells are the program's own**: per question and way, the saturations the way adds or
   removes on that program. The corpus's verdicts are the price list's page and a column of each program's.
10. **A wall is shown as its census took it**, with the fingerprint on the page; the census line's `wall` is
    called *the whole reading*, which it is where the theory's share is asked (K-16).
11. **The defaults page takes every section whose heading says *default(s) taken***, which adds S1f.10.11's *The
    stop, and the default taken on it* to the six headed *Defaults taken*.
12. **`utils/README.md`'s count** is brought to the directory's, forty.

### Not done

- `./run_tests.sh` was not run: the gate is the batch's. The Rust change is a field of an example's JSON.
- The bundle was read by a headless browser's screenshots and by `--check`, not in a browser by hand.
- T2, T3 and T4.

## Record (2026-11-16): the surface and the page

T3 and T4 ran, on the ruling of [Q-M1f.309](../open_questions.md#q-m1f309--how-ein-shows-a-programs-structure),
(a), and on [Q-M1f.310](../open_questions.md#q-m1f310--p1f10-part-1-the-defaults-taken)'s defaults as they stand:
the owner module is built on, and no object is defined anew. In a worktree, on a branch of its own, over
`a3c719bf`. No answer, route or line of `ein solve` moved. rustc 1.97.1.

**Read this first.** Four things below are not verified, and one finding bears on how far the owner's own
`Coupling::complete()` can be trusted:

- **`master` moved while this ran, to an engine that builds H otherwise, and nothing here was run on the merged
  tree** ([below](#master-moved-while-this-ran)). The merge is textually clean in a trial.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run, by instruction. **A DOT writer changed,
  so `utils/ci_container.sh` is part of done and is owed.**
- **No wall was measured.** The machine ran the main tree's nightly throughout (load average 53 on 32 threads when
  looked at, a second `explore` test binary and an `ein` beside this worktree's). So whether the two nightly steps
  that grew stay within five minutes on four P-core threads at `EIN_TEST_JOBS=4` is **not known**. Costs below are
  saturations and facts, which were observed.
- The view's supports were exercised on two programs, and its `.einb` door and `--jobs` on one each.
- [K-17](findings.md): a pair layer read whole can leave the witnesses' holders unread, the unlicensed debts then
  drop out of the coupling, and `Coupling::complete()` stays true. The view does not rely on it.

### The command

```sh
ein render structure FILE [--view summary|exclusion|cover|coupling|one-step|json] [--dump DIR]
                          [--supports] [--region-budget FACTS] [-j JOBS] [--no-require-why]
```

- **stdout holds the view asked.** `summary`, the default, is a read-out: a row an object, what it cost, what each
  graph holds, and every sentence of what was stopped, cut or left unread. `exclusion`, `cover`, `coupling` and
  `one-step` are one undirected graph as DOT. `json` is the document.
- **`--dump DIR` also writes** `structure.json`, `exclusion.dot`, `cover.dot`, `coupling.dot` and
  `coupling_one_step.dot`. stdout and stderr are the same with it. The directory is made before the work.
- **No SVG**: no `ein render` view draws one, and the view adds no Graphviz dependency to a path that had none.
- **Exit codes**: every structure exits 0, whole or not. A program the loader refuses, or the engine refuses to
  read, exits 1; a path `--dump` cannot make exits 1 before the work; a usage error exits 2.
- A `.ein` and a `.einb` alike, through `common::load_program`.

### What was built

| where | what |
|---|---|
| `ein-infer/src/structure/coupling.rs` | **one entry added to the owner**, `whole` (`Whole`, `Parts`, `WholeOptions`): `theory_coupled` with root's pairs asked and no check, `ExactlyOne::of` the cover it found, and `mechanisms::read` of the root it left. Each pair layer is read once. No object is defined in it |
| `ein-infer/src/structure.rs` | `subject` and `names_of` made `pub`, so that a label is read by the owner's function and not by a copy |
| `ein-render/src/structure.rs`, new | the spelling, and no computation: five `*_lists`, the lists of one object each; `coupling_dot`; the exclusion graph and the cover; `unread` and `says`; `json`, `document`; `summary` |
| `ein-render/src/read_out.rs` | `plural` made `pub(crate)`. Nothing it prints moved (`read_out_reference`) |
| `ein-cli/src/{cmdline,lib,render}.rs` | the subcommand and `cmd_structure` |
| `ein-infer/examples/pair_layer.rs` | calls the library for every list it writes and for `coupling.dot`; 2 611 lines to 2 238 |

**`ein-render` and `ein-cli` hold no second definition of a group, a grid, a question or an edge.** What they
compute of their own is joins and labels: a candidate's row of `candidates` joins the owner's lists by place; a
question's name is the pattern of its members' texts; a grid's `relations` are read off its members' texts by the
owner's `subject`; and `declared.relations[].stated_of_it` lists the facts the small KB keeps that name a
relation, by the owner's `names_of`.

### What moved out of the example, and the check

The example now takes from `ein_render::structure`: `stopped`, `pairs_lists`, `theory_lists`, `cover_lists`,
`exactly_one_lists`, `coupling_lists`, `question_members`, `question_name`, `question_open` (and `pattern`
through them) and `coupling_dot`. The library builds `ein_render::dump::Json`; the example converts it to
`serde_json` and keeps its own writer, which sorts keys, and adds the keys only the census has (`program`,
`counts`, `declared`, `owed`, `owed_instances`). What stays in the example: the census line's counts, the Markdown
twins, `theory.ein`, the alternative rules, the routes, and `declared_beside`, which still reads the mechanisms by
a second load of its own.

**The check**, on the six instances with the census's flags, `--routes --check --readings`: the binary built at
`a3c719bf` before any change against the final one.

| | |
|---|---|
| files written | 66 by each |
| the same bytes | 60: every `.md`, `.dot`, `.ein`, and every JSON but six |
| the same as JSON with every `wall` blanked | 6, the six `coupling.json`, whose `counts.wall` is a clock |
| differ | 0 |
| the six census lines, walls aside | the same |

Taken twice, after the first rewiring and again on the final build, with the one result. The comparing script
and both outputs were scratch and are not in the tree; the commands are the census's own, with `--out` elsewhere.

### `structure.json`

Schema `ein-structure/1`. Field by field it is [`docs/kernel/inference/structure.md` § `structure.json`](../../../docs/ein/reasoning/kb-evolution/hypothesis-space.md#h-spacejson).
In outline:

| key | holds |
|---|---|
| `schema`, `program`, `asked` | the header: what was asked, `region_budget` and `supports` |
| `n`, `rung`, `monotone`, `root_dead` | H's size, the rung that built it, the static verdict |
| `whole`, `unread`, `says`, `domain` | whether anything was stopped, cut or left unread, a sentence each; what makes a structure small or empty; a flagged program's three sentences |
| `cost` | each object's saturations and facts: root's layer, the small KB's, the supports, the clique calls, the whole node, and the sum |
| `graphs` | each graph's file, nodes and lines |
| `candidates` | **H listed**: each candidate's mark, what it implies and is equivalent to, its pairs by stroke, its groups, whether one is owed, its questions, its component in each set |
| `pairs` | root's pair layer, every list |
| `theory` | every stated fact with what the small KB does with it, the theory's pairs, each pair root excludes with whether it is the theory's and its support, and the small KB's pair layer whole |
| `cover` | the groups with their open members, the marks, the components, the theory's pairs alive at root, root's cliques |
| `exactly_one` | every owed instance with its witnesses and its standing, the owed groups, the unlicensed instances, the grids, the components that are no grid |
| `coupling` | the questions **by place and by name**, the ties, the edges of both readings with their kind, the debts with `licensed`, both sets of components |
| `declared` | the mechanisms at root's fixpoint, and for each relation a candidate is of the kept statements that name it |

**It holds no clock and no job count.**

### Tests in the gate, and what each holds

| test | holds | its limit |
|---|---|---|
| `ein-cli/tests/structure_view.rs`, new, 13 tests | the document held to the owner's own values, list for list, on the six instances; on I-Z2 five grids, five bijections, 36 owed groups, each of the five relations with `(bijective R)` among what is stated of it; `--dump`; a stopped layer; a refused reading (K-17); a flagged program; no candidate; a dead root; `--supports`; `--jobs`; a `.einb`; a refused program; the page's read-out; the page's keys | the binary's document is compared with the library's on five instances, I-Z2M being read once, in process. `--jobs` is one program at one count. The `.einb` is one program. A write that fails after the run is not tested |
| `ein-infer/tests/structure_invariance.rs`, extended | **the document and the four graphs' DOT, byte for byte, under a permuted id space**, on every corpus program that searches, 175 per commit; the document under the two other import forms, every string with its prefixes off and every list of strings as a set, in 92 of 94 forms; and that each comparison fails when it should | the two forms that reorder the universe are not compared as documents: a place follows the order. The owner's sets are compared there as before. The graphs are not compared under import forms |
| `ein-render/tests/dot_wellformed.rs`, one test added | Graphviz draws every graph of every program that searches, 652 graphs of 164 programs per commit and 744 of 188 under `EIN_CORPUS_SLOW=1`; **every structure read is whole at the default budget**; every pair root excludes is drawn by one stroke; a graph's stated lines are its strokes | 4 graphs per commit and 8 in the nightly are past 1 000 lines and are not laid out, each named |
| `ein-cli/tests/render_doors.rs` | the four graph views through the door, and the refusal | |
| `ein-cli/tests/config_reference.rs` | twelve commands take `--no-require-why`; the page's counts are the golden's | |
| `ein-cli/tests/help_surface.rs` | the golden, with the command | |

**On I-Z2 the five declared bijections are shown as found, and what is stated of each relation is shown beside,
with no stdlib relation named in `ein-render`.** The view lists, for each relation a candidate is of, the facts
the small KB keeps that name it; on `zebra2.ein` that list holds `(bijective R)` for each of the five. Which name
means *bijective* is the test's to know.

**The 1 000-line rule is followed and not weakened.** `SKIPPING`, the directories, is as it was. The structure's
graphs have a list of their own, `STRUCTURE_SKIPPING`, exact by program and view: I-Z2M's and its obligations
twin's exclusion graph (1 428 lines) and edges one application back (1 274), and both hypergraphs of
`count-past-2-64.ein` (2 080) and `count-past-2-128.ein` (8 515). `dot_wellformed`'s `edges` counts arrows, and
these graphs are undirected, so the new test counts strokes.

Run and green, each read from its own result line and not from an exit status: `-p ein-render`, every target;
`-p ein-cli` `structure_view`, `render_doors`, `config_reference`, `gate_steps`, `nightly_steps`, `help_surface`,
`read_out_reference`, `lever_invariance`; `-p ein-infer` `structure_invariance`, `cover`, `coupling`,
`exactly_one`, `pair_layer`, `theory_share`, `explore`, `claim_routes`. `lever_invariance`, `explore`,
`claim_routes` and `read_out_reference` were not edited.

**On which sources.** `structure_view`, `render_doors`, `config_reference`, `help_surface`,
`structure_invariance` and the new `dot_wellformed` test, in both tiers, were run again on the final sources. The
others ran earlier in the session, before the last edits to `ein-render/src/structure.rs`, which were two
sentences of `unread` and a comment: `ein-render`'s other targets, `lever_invariance`, `read_out_reference`,
`gate_steps` and `nightly_steps` call none of that module, and `ein-infer`'s sources did not change after its
targets ran. **No one run covered the whole set at once**: the gate does that.

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`utils/check_hashmap_iteration.py` and `utils/doc_audit.py --links --check` with the gate's list: each exit 0 on
the final sources. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: exit 0 twice, and a third run
after the last wording edits printed no warning. Clippy and rustdoc each finished in seconds, which was checked
and not assumed: clippy reports findings in the new module when asked for `clippy::pedantic`, and rustdoc's
pages for `structure::whole` and `ein_render::structure` are rewritten by a run.

### What it costs

In saturations and facts, each printed by a command. **No wall is given: none was measured.**

| | saturations | facts |
|---|---:|---|
| one structure of I-Z2, `ein render structure examples/zebra2.ein` | 465 + 1 597 + 1 | 68 385 beyond root; 32 086 beyond the small KB's root |
| … with `--supports` | 327 more | |
| a describe of I-Z2, `ein solve examples/zebra2.ein` | 374 | 42 705 beyond root |
| **per commit**, the sweep added to `dot_wellformed`: 164 programs | 33 045 + 62 929 + 150 | 108 906; 186 260 |
| **in the nightly**, the same under `EIN_CORPUS_SLOW=1`: 188 programs | 49 015 + 82 589 + 173 | 1 389 866; 513 855 |

The last two are the test's own line (`cargo test … -p ein-render --test dot_wellformed --
graphviz_accepts_the_structure --nocapture`), and the nightly's agree with S1f.10.13's table to the unit. Beside
the engine's work that test puts 652 graphs, 744 in the nightly, through `dot`.

- **`structure_invariance` reads no program more than it did.** It reads each through `whole` in place of
  `theory_coupled`, which adds one `mechanisms::read` a reading, and spells the document and the graphs of each.
- **`structure_view` is new per-commit work**: the six instances once in process and five of them once more
  through the binary, and about fifty more runs of the view, counted by reading the file and not by an
  instrument: some eight of them on `zebra.ein`, some ten on `zebra2.ein`, the rest on small programs or
  refused.
- **Walls seen under load, and not measurements**: `structure_view` reported 10 to 29 s over its runs,
  `structure_invariance` 13 to 35 s, the new `dot_wellformed` test 7 to 10 s per commit and 27 to 41 s with the
  slow entries. The spread within one test is the load's.

### Goldens

- **`ein-cli/tests/golden/help_shape.txt` moved**: nine lines added, the command, its six options and its
  positional. Re-banked with `EIN_BLESS=1`.
- **`ein-render/tests/golden/corpus_shapes.md5` did not move**, and `id_order_invariance` is green against it as
  it was: the structure is not an op of the digest sweep.
- No other golden moved.

### The page

[`docs/kernel/inference/structure.md`](../../../docs/ein/reasoning/kb-evolution/hypothesis-space.md): each object as built, the
domain, what it costs, how to see it with `structure.json` field by field, and what each reading does not tell.

- **Its read-out is the binary's**: the block is held byte for byte by `the_pages_read_out_is_the_binarys`, so
  every figure in it is taken again per commit.
- **Its other figures each carry a command**: the corpus's totals, printed by the `dot_wellformed` test; I-Z2's
  327 supports of which 7 are cut, by the view with `--supports`. The bounds are named by constant, with the grep
  that prints them.
- **Its keys are held one way**: every key six documents of six kinds hold is named in the page. A key the writer
  has for a case none of them reaches is in the page by reading the writer and is held by nothing.
- It names no stage, no path into `plans/`, no date and no finding. **One decision id is in a code comment**,
  `Q-M1f.309` in `cmdline.rs`. And the new command's `--help` shows `(M1f T1f.5.14.7)`, through the shared
  `--no-require-why` builder, as the eleven others do.
- Registered as *current*: `docs/kernel/README.md`, 47 pages; `inference/README.md`'s tree; two rows of
  `implementation.md`; a section of `04_dot_rendering.md`; `configuration.md`'s counts, a class of § 3.3 with
  *does it change the answer: no*, and § 3.4's list. `docs/guide/` and `docs/api/` list no view and were not
  touched.
- Of the six checks of *Keeping this true*: links and section citations, 0 findings; identifiers, 0 unresolved on
  the page, by a report that is a heuristic and no gate; the page is in one state. **Every command the page shows
  was run but one as shown**: the census's `pair_layer … --theory --routes > rows.jsonl` was run on one program
  with `--out` elsewhere, to check the three fields the page names, and not over the corpus.

### What was got wrong on the way, and what it shows

The sentence the view prints where the witnesses' holders were not read was wrong twice before it was right, and
each time a run beside it said so.

1. It first said *a layer's singles were not read*. The same run reported no single left.
2. It then said *root's layer having stopped first*. One fact under what root's layer spends, the layer is whole.
3. It now says the layer had spent its budget, whether or not it read every pair, and does not say it stopped.
   `a_whole_layer_with_its_holders_refused_is_not_called_whole` holds that wording.

So the `unread` sentences are worth reading against the fields beside them. The sentence for a cut support was
checked the same way, against `CoreStatus`'s own words. And a claim that one DOT need not lay out alike twice was
taken off the page: six `fdp` layouts of the view's coupling graph of `zebra.ein` gave one digest, so K-15 was
not reproduced on the view's graphs. Nothing holds a picture either way.

### Defaults taken

Each the user's or the main session's to revise.

1. **stdout is one view, and the default is the read-out**, not DOT. Every other `ein render` view prints DOT by
   default; this one has four graphs and a document, and one stream cannot hold them. `ein render region --view
   table` is the precedent for a view that is not DOT. `--dump DIR` follows `ein oracle --dump`: additive.
2. **`--supports` is off by default**, against *nearest to what the bundle showed*: the bundle labelled every
   pair of the puzzle. A support is a saturation and a search a pair. No `--check`: the check is the census's.
3. **What a program declares**: the mechanisms as the engine reads them, and the kept statements that name each
   relation. `(bijective R)` is read by no name in the engine.
4. **`whole` is the view's own**: nothing stopped by a budget, nothing cut by a bound, no reading refused. It is
   not `Coupling::complete()`, which K-17 shows true where debts are missing. A dead root and a program with no
   candidate alive alone are `says`, and no cut.
5. **The structure is not in the digest golden.** Its bytes are held under a permuted id space by
   `structure_invariance`, on the readings that test already makes. **Decided without a measured comparison**:
   what an op of the digest sweep would cost per commit was not taken.
6. **`dot_wellformed`'s sweep**: per commit the programs that search outside `tests/`, the `slow` entries aside;
   the rest in the nightly step that test already is.
7. **`structure_view` reads I-Z2M once.**
8. **The graphs**: the exclusion graph's strokes and fills are the bundle's; a statement's label is its place in
   `theory.stated`; the cover is groups as boxes and members as ovals, a component a cluster, which the bundle drew
   as HTML tables; the hypergraph is the census's `coupling.dot` with each question's place in its label
   ([K-14](findings.md)).
9. **`lines`** is the document's word for a graph's strokes: an edge of five questions draws five.
10. **The owner gained `whole`** and two functions became public.
11. **The cut-support sentence counts a cut support as unread**, so `--supports` makes I-Z2 not whole.

### Findings

[K-17](findings.md), above. Not fixed.

### Not done, and not verified

- `./run_tests.sh`, `./run_tests.sh --nightly`, `utils/ci_container.sh`: not run.
- **Any wall**, and so the five-minute bound of the two nightly steps that grew, `dot_wellformed` and
  `structure_invariance`.
- **`utils/structure_bundle.py` is as it was**, and reads the census's files. To read the view instead it would
  need: `structure.json`'s five objects mapped to the five files it loads, which they are but for `program`;
  `cover.declared`, `cover.owed` and `owed_instances`, which are the census's tallies and are not in the
  document; the supports' checks, which the view never asks; the counts of the three other readings; and the
  routes and the price list, which are other examples'. Not tried.
- The example's `declared_beside` and the owner's `whole` each read the mechanisms.
- `--supports` on a program larger than I-Z2; a `--dump` write that fails after the run; `--jobs` past one
  program and one count; the document under an import form that reorders the universe.
- The worktree was at `dbedc535` and not at `a3c719bf` as briefed. The branch was fast-forwarded to `a3c719bf`
  before any work; `master` was not touched.

### `master` moved while this ran

Found after the commit, `6acaa3d7`, and so stated here in a second one. **Every result above was taken over
`a3c719bf`, and none on a tree that holds what `master` gained since.**

- `master` was at `3e59b90a` when this was committed: three commits past `a3c719bf`, the merge of
  [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md)'s task 4. By its message, on the obligations rung H now
  also holds what would pay a debt a rule can bring a commitment to owe, `W26/oblig-derived-activator` is fixed,
  and no other corpus program moves. It changes `oblgen.rs`, `hypgen.rs`, `region.rs`, `forbid.rs`, `compile.rs`,
  `monotone.rs` and `scope.rs`, which build H and L5's table, and `ein-render/src/read_out.rs`; and it moved
  `corpus_shapes.md5`, which this stage's commit did not.
- **A trial merge is textually clean** (`git merge-tree --write-tree master 6acaa3d7`, which writes a tree and
  touches no ref). Five files are changed by both: `AGENTS.md`, `docs/kernel/inference/implementation.md`,
  `ein-render/src/read_out.rs`, this phase's `README.md` and `findings.md`.
- **The merged tree was not built and not tested here.** The owner reads H and L5's table, so these are to be
  taken again on it, each by a command already named above:
  - that it compiles, and `structure_view`, `structure_invariance` and the `dot_wellformed` sweep pass;
  - **the page's corpus figures**, 49 015 and 82 589 saturations with their facts and 173 nodes, which the
    `dot_wellformed` test prints: a program whose H gained a candidate has other layers;
  - the page's read-out of `zebra2.ein`, which a test holds and will say;
  - that every corpus structure is still whole at the default budget, which the same sweep holds;
  - K-17's witness figures on `zebra.ein`, 181 088 and 67 244, which
    `a_whole_layer_with_its_holders_refused_is_not_called_whole` reads off the program and does not hard-code.
- The phase README's sentence before this stage's still calls the stop pending: it is the main session's, and was
  left as written.

## Record (2026-11-16): the view renamed, H dumped plain, K-17

A follow-up to [the surface](#record-2026-11-16-the-surface-and-the-page), on the user's instruction of the same
day, in a worktree on a branch of its own over `2050a886`, three commits and this record. The worktree was at
`dbedc535`; the branch was fast-forwarded to `2050a886` before any change. No answer, route or line of
`ein solve` moved. rustc 1.97.1.

**The user, 2026-11-16, verbatim:**

> does `ein render structure` shows any structure? if only hypothesis then rename it to something
> hypothesis-relate, like `ein render h-structure` or `ein render h-space`; also can we get plain dump of
> hypothesis? we should be able to

and, relayed by the main session while the second change was being built, of where the dump goes:

> ein render writes dot, by listing hypothesis I meant listing them as ein facts, so add one more top level
> command `show` and one sub-command `show hyps` that lists them

### 1. The rename

The view shows the structure of the hypothesis set H and nothing else, so the user's condition holds.

- **The subcommand is `ein render h-space FILE`**, with the options and the output it had but for the name.
  `ein render structure` is refused as any unknown subcommand is, exit 2: no alias.
- What follows the name: the document under `--dump` is `h-space.json`, its schema `ein-h-space/1`; the
  read-out's first row is labelled `h-space`; `--help` says *hypothesis space*; the test file is
  `ein-cli/tests/h_space_view.rs`; the page is
  [`docs/kernel/inference/hypothesis_space.md`](../../../docs/ein/reasoning/kb-evolution/hypothesis-space.md), titled *The
  hypothesis space and its structure*, and every link to it follows, this file's and the phase README's by path
  alone, their prose as written. The four graph files keep their names.
- **The name is written once**: `ein_render::structure`'s `command!()`, from which `COMMAND`, `SCHEMA` and
  `DOCUMENT` are spelled; the parser, the dispatch and the read-out's first row take `COMMAND`. A second rename
  is that literal, the page's file and text, and the goldens below.
- Not renamed: the modules `ein_infer::structure` and `ein_render::structure`, the census example, the bundle
  and its script, and the wording of every earlier record, the one above among them.

### 2. `ein show hyps FILE`

```sh
ein show hyps FILE [--no-require-why]
```

- **A new top-level command, `ein show`**, a group as `ein render` is, with one subcommand. `ein show` alone is a
  usage error that names its subcommands, exit 2, as `ein render` alone is. `ein render` draws; `ein show` lists.
- **stdout is H and nothing else**: one candidate a line, as the fact is written, every candidate, uncut, in the
  universe's canonical order. On `examples/zebra2.ein`:

  ```text
  (color-loc Green House-4)
  (color-loc Green House-5)
  (color-loc Ivory House-3)
  ```

  and on stderr ``note: examples/zebra2.ein: H, built by the rung `hrules`: 56 candidates``.
- **A line is a fact a `.ein` file could hold**: it is `Terms::fact_text`, the printer a state file's facts are
  written by, and spelled as the program's own file spells the fact.
- An empty H prints nothing and exits 0, and the note says why in the words of the read-out's `universe` row for a
  run that searches nothing, *no candidate* and *no candidate: the program states :no-search*. A dead root's H is
  listed. A program the loader refuses, or the engine refuses to read, exits 1 with the line `ein saturate` or
  `ein solve` gives it. A `.ein` and a `.einb` alike.
- **It costs root's saturation and the generator's pass, and no node**: no pair layer, no small KB, no whole
  node. `listing_h_is_one_saturation_and_no_node` narrates the owner's entry on the six instances and counts one
  saturation and no node in the stream; the same count over a pair layer of the conflict of three finds its seven
  nodes.
- **`ein render h-space` gained nothing**: no view and no file under `--dump`. A `--view candidates` and a
  `candidates.txt` were the first design and were not built.

**Where H's one function lives.** `ein_infer::structure::hypotheses` (and `hypotheses_with`, narrated) returns a
`Hypotheses`: the candidates, the rung, whether root is dead. It is `root_prepared`, which is
`region::prepare_with`, the preparation every route starts from; and every space a structure is read in
(`with_space_watched`) is opened over root prepared by that same function. So `whole`'s candidates, the
document's `candidates` and the listing are one list by construction, and a test holds it besides.

**How it is held to `-H` and to the document.**

| test | holds |
|---|---|
| `ein-cli/tests/h_space_view.rs`, in `the_document_is_the_owners_reading_on_every_instance` | on the six instances the lines are the document's `candidates`, line for line, and the candidates of `ein solve FILE -H --no-search --dump-answer DIR`'s `forms/hyp-stats.txt`, which lists them by relation; and the program with the lines written after it loads and holds each line as the printer writes that fact |
| `ein-infer/tests/structure_invariance.rs`, in the sweep it already ran | in every reading, as loaded, in a permuted id space and under each import form, the lines are the document's `candidates`, the owner's candidates and the universe a run that searches nothing is handed, place for place; the listing is the same bytes under a permuted id space and the same set of lines, prefixes off, under every import form; `a_structure_that_moves_is_caught` shows each comparison failing |
| `ein-cli/tests/show_hyps.rs`, new, 6 tests | nothing else on stdout and one note; an empty H with the read-out's own words; a dead root; a refused program, by the loader and by *forbid*; the usage errors, the options it does not take, a `.einb`; the page's listing |

The sweep reads 175 programs per commit, 47 of them importing, in 94 forms; its line is
`cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance -- --nocapture`
(2026-11-16). What it added to that test's work is a root saturation and root's node for each reading, three a
program at most; no wall was measured.

### 3. K-17

[K-17](findings.md) is fixed in the owner, and `explore.rs`' `afford` is as it was.

- **The holders are read off the cache and ask no budget.** `Space::cached`, new and `pub(crate)`, returns nodes
  only where each was computed, computes nothing and charges nothing; `structure.rs`' `holders` reads the singles
  through it. A layer that read its singles has its witnesses' holders read whatever it spent, at root and in the
  small KB.
- **Where they cannot be read, the coupling says so.** They stay unread where a layer stopped before its last
  single, and there alone. `Coupling::payers_read`, new, is false wherever the table holds an unlicensed instance
  with witnesses and root's singles were not all read, and `Coupling::complete()` is false with it. The document
  carries `coupling.payers_read`.
- **At the two witnesses**, `examples/zebra.ein` one fact under what root's layer spends and one under what the
  small KB's does, each taken off the program's own document: the layer reads every pair, the holders are read,
  every debt of an unlicensed instance is an edge, `complete()` is true and the document is the default budget's
  but for what was asked; in the small KB every unlicensed instance is about the group it is about at the default
  budget. At a budget of one fact both layers stop among their singles and `singles_read`, `theory_read`,
  `payers_read` and `complete` are false. `ein-infer/tests/coupling.rs` holds both at the owner and
  `ein-cli/tests/h_space_view.rs` at the view.
- **The view's special case is not dead, and is smaller.** Its two sentences now say a layer *stopped before its
  last single*, which is the one case left, and stand beside the sentence that says the layer stopped. The view's
  `whole` and the owner's `complete()` agree there. `a_whole_layer_with_its_holders_refused_is_not_called_whole`
  is replaced by `a_layer_read_whole_has_its_holders_read` and `holders_a_stopped_layer_left_unread_are_said`.

### Goldens moved

- `ein-cli/tests/golden/help_shape.txt`: by the rename, the view's nine lines under its new name and at its new
  place in the order, its about and its `--dump` and `--view` help reworded; by `ein show`, six lines added.
- `docs/kernel/configuration.md`, held by `config_reference.rs`: the view's rows renamed; then 91 options across
  eighteen parsers where there were 90 across sixteen, a row for `ein show hyps`, 89 without the default features,
  a class of § 3.3 with *does it change the answer: no*, § 3.4's list, and `LOADING` thirteen commands.
- The page's read-out block, its marker and first row; and a second held block, the listing.
- `examples/README.md`'s anchor list, twice ([K-18](findings.md)).
- `ein-render/tests/golden/corpus_shapes.md5` did not move.

### Defaults taken

Each the user's or the main session's to revise.

1. **The name `h-space`**: the main session's choice between the user's two examples, `h-structure` and
   `h-space`, since the command shows the hypothesis space, its candidates and its structure. Kept one literal so
   that another choice is cheap.
2. **No alias** for `ein render structure`: the command was one day old and unreleased.
3. **`hyps`' note**: ``H, built by the rung `R`: N candidates``. The read-out's `universe` words are written a
   second time in `ein_render::structure::hypotheses_note` and held to the read-out by a test, since
   `read_out.rs`' `universe` rows were another agent's to edit while this ran.
4. **No `-o FILE`**: no command that prints text takes one.
5. **The exit of a refused program is 1**, `ein solve`'s and `ein render`'s, for the loader's refusals and the
   engine's alike.
6. **`-H` is compared as a set**: it lists by relation, and the universe's order is the rendered text's.
7. **`Coupling::payers_read` is about the holders alone**: an unlicensed instance whose rule names no witness has
   no payers to read, and lacks none.
8. **K-18's list was re-banked here**, the rename moving one of its two missing entries anyway.

### Findings

[K-18](findings.md): `world_anchors` was red on `master` since the surface's merge; re-banked.
[K-19](findings.md): the support check's own read of the cache is refused as K-17's was, and the check is dropped
without a word; open, not fixed.

### Verification

On the final tree, each read from its own result line: `cargo test --manifest-path ein.rs/Cargo.toml --profile
gate` for `-p ein-render`, every target; `-p ein-cli` `h_space_view`, `show_hyps`, `render_doors`,
`config_reference`, `help_surface`, `gate_steps`, `nightly_steps`, `lever_invariance`, `embedding`,
`world_anchors`, `read_out_reference`; `-p ein-infer` `--lib` and `structure_invariance`, `cover`, `coupling`,
`exactly_one`, `pair_layer`, `theory_share`, `explore`, `region`, `claim_routes`, `price_list`. And
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `utils/check_hashmap_iteration.py`, and
`utils/doc_audit.py --links --check` with the gate's list. `explore`, `region`, `claim_routes` and
`lever_invariance` were not edited.

### Not done, and not verified

- `./run_tests.sh`, `./run_tests.sh --nightly`, `utils/ci_container.sh`: not run, by instruction. The DOT
  writers did not change.
- **No wall was measured**, of the command or of the tests that grew.
- The slow entries' half of `structure_invariance` and of `dot_wellformed` (`EIN_CORPUS_SLOW=1`), and so the
  page's corpus totals, which no change here should move: not re-taken.
- `ein show hyps` on a `.einb` is one program; under `--no-require-why` it is `config_reference`'s one program.
- K-17's fix below the two witnesses and the budget of one fact: no other program and no other budget was probed.
- `utils/structure_bundle.py` and the census example write what they wrote, but for `coupling.json`'s `complete`,
  which is now false where the payers were not read; no corpus program is in that state at the default budget.
- The status line at this file's head and the phase README's earlier sentences still name
  `ein render structure`: earlier records, left as written.
