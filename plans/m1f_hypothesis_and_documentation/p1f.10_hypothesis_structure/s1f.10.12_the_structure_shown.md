# S1f.10.12 — The structure shown, and its surface

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** T1, the bundle, ran on 2026-11-16 ([its record](#record-2026-11-16-the-bundle)). **T2 is the phase's one stop, pending**: the message and the four questions are the main session's. T3 and T4 wait for the ruling; no surface is designed or built.
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
