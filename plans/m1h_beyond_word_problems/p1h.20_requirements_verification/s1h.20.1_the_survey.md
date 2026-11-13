# S1h.20.1 — The survey: how requirements are verified, field by field and method by method

**Phase:** [P1h.20](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A survey: it writes a page and catalogue entries, and no
program. **It stops for the user,** who reads it before anything of the phase is planned further.
**Depends on:** nothing. It may be run at any time the user wants it.

## What it rests on

The instruction, 2026-11-12, in full:

> p1h.20 - requirements verification. How is it done in real world? e.g. in systems design, or market
> research, construction, renovation, hair styling, cooking, anywhere. First survey methods of requirement
> verification. I think there are many, different tables, analysis, answering pre-defined or template
> questions and other approaches. How it is done? Probably my modelling. Like threats modelling, failures
> modelling, and many different modellings.

The milestone README, on what the survey has to return: *for each method, what its input is (a list of
requirements, a design, a model), what it produces (a table, a verdict, a list of gaps or conflicts), and
which part of it is "reasoning over stated facts", which is the part Ein could do. Its catalogue entries go
to `docs/lib/` beside the external-tech notes, by that directory's conventions.* And on the meeting points
with Ein: *to be confirmed by the survey and not assumed.*

## What it produces

- **`survey.md`** in this directory: the survey, organised so that the next step can be planned from it.
- **Catalogue entries in [`docs/lib/`](../../../docs/lib/README.md),** by that directory's conventions: an
  entry a method, a description of one to three lines, external links in its order of preference (the
  upstream or official page, Wikipedia, a representative publication), cross-references by relative link,
  and a short note where there is a judgement or a caveat. Each entry goes to the file that is its most
  natural home; if none of the twelve is, a thirteenth thematic file is added and listed in that README. If
  `docs/lib/knowledge-graph.dot` changes, both views are re-rendered
  (`utils/render_knowledge_graph.sh svg all`, `python3 utils/render_knowledge_graph_cy.py`).

## Tasks

1. **T1h.20.1.1 — the words.** What *requirement* and *verification* mean in each field the survey enters,
   in the field's own usage, with its source. Some fields keep *verification* (was it built as specified)
   apart from *validation* (is it what was wanted); the instruction's examples run across both. The page
   says, for each method it describes, which it is, and narrows nothing: both are in scope, since the
   instruction asks how it is done *anywhere*.
2. **T1h.20.1.2 — field by field.** For each field the user names, *systems design, market research,
   construction, renovation, hair styling, cooking*, and for *anywhere*, two or three more that the survey
   finds do it differently: how a requirement is stated there, by whom; how it is checked that the result
   meets it, by whom and when; what is written down. A field in which the method is a practice with no name
   is described as the practice, in the field's words. A field where nothing could be found is listed with
   what was looked for.
3. **T1h.20.1.3 — method by method.** The methods the fields use, each once, with the three things the
   milestone README asks for:
   - **its input:** a list of requirements, a design, a model, a finished thing;
   - **what it produces:** a table, a verdict, a list of gaps or conflicts, a ranked list, a signature;
   - **which part of it is reasoning over stated facts,** and which part is not: judgement, a measurement, a
     trial, a conversation with whoever asked.
   The instruction's own three kinds are each looked for by name: *"different tables"* (the tables that
   relate requirements to something else), *"analysis"*, *"answering pre-defined or template questions"*
   (checklists and question templates), *"and other approaches"*.
4. **T1h.20.1.4 — the conjecture: *done by modelling*.** *"Probably my modelling. Like threats modelling,
   failures modelling, and many different modellings."* For the modelling methods the survey finds, those
   two first: what the model is made of (what is stated), what is asked of it, and what is derived. Then the
   answer to the conjecture, on the evidence of T2 and T3: which of the methods are a model with questions
   put to it, which are not, and in which fields.
5. **T1h.20.1.5 — the meeting points with Ein, each confirmed or not.** Three, from the milestone README,
   each against the methods of T3 and not in the abstract:
   - the three task classes ([idea 03](../../ideas/03-three-task-classes.md)) read as *the requirements are
     satisfiable, incomplete, or in conflict*: which methods ask which of the three;
   - [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s gaps and assumptions: which methods are
     about what a set of requirements leaves unsaid;
   - [P1h.2](../p1h.2_slicing/README.md)'s slice as *which requirements a design decision rests on*: which
     methods trace a decision or a part back to its requirements.
   Each marked *confirmed*, *not confirmed* or *not reached*, with the method that shows it. A meeting point
   the README does not list and the survey finds is added, marked as found.
6. **T1h.20.1.6 — what would come next, as candidates and not as a plan.** For the two or three methods
   whose reasoning part is largest: what a first Ein program of it would have to state, in a sentence or two
   each, and what would stand in the way. No program is written. This is the evidence for
   [Q-M1h.6](../open_questions.md#q-m1h6--whether-p1h20-and-p1h30-end-in-programs-at-all), put for this
   phase: does it end in programs, in part, or not.
7. **T1h.20.1.7 — the catalogue entries,** written to `docs/lib/` as above, each linked from the survey.
8. **T1h.20.1.8 — the page and the stop.** `survey.md`, every claim about a method with its source. **The
   stage stops here.** The user's reading is recorded on the page in the user's words, and the answer to
   Q-M1h.6 for this phase in [`open_questions.md`](../open_questions.md).

## Acceptance

- Each field the user names has a section, with at least one method or the statement of what was looked for
  and not found.
- Each method has its input, its output and its reasoning part, and a source.
- The conjecture has an answer with its evidence.
- Each of the three meeting points is marked, with the method that shows it.
- The catalogue entries are in `docs/lib/` by its conventions; `python3 utils/doc_audit.py --links --check`
  over the gate's paths is green.
- No program was written, and nothing outside `plans/` and `docs/lib/` moved.
- The user has read the survey; the page records it.

## What it must not do

- **Not plan the phase past this page.** [S1h.20.2](s1h.20.2_the_plan_after_the_survey.md) is written after
  the user has read it.
- **Not write an Ein program,** a relation or a rule, even as a sketch that looks like a design.
- **Not assume the meeting points.** *To be confirmed by the survey and not assumed.*
- **Not narrow the fields to the ones with a literature.**
- **Not send anything about the user or the project** in a request to an outside service while surveying.

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to `findings.md`
in this directory, opened at the first. A reading is recorded as the user gave it. The link check is run
before the commit, which changes pages only.
