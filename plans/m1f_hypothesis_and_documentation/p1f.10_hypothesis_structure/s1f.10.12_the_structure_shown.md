# S1f.10.12 — The structure shown, and its surface

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** planned 2026-11-15, at the re-plan. Not started. **It holds the phase's one stop.**
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
