# Q-M2.8 — Where does EinAf live?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25, in two answers.** The first: *"./einaf/, ./docs/einaf/, etc..."*. The second, to
the coordinator's default that those trees were external to both the project and the process: *"yes, own tree, in
this repository, outside of ein, because ein.rs is an implementation of ein and is a dependency of einaf, but
inside the development process, make clear projects hierachy in root readme and AGENTS.md"*.

**Read as:** the repository holds two projects, one over the other, and one development process over both.

| | what it is | its trees | depends on |
|---|---|---|---|
| **Ein** | the language, what a program means and what its answer is, and **ein.rs**, an implementation of it | `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/ein/`, `docs/decisions/`; the implementation in `ein.rs/` | nothing in the repository |
| **EinAf** | iterative autoformalization, an application of Ein | `einaf/`, `docs/einaf/` | ein.rs, as a library |
| the process | how both are developed | `plans/`, `utils/`, the gate and CI, `AGENTS.md` | reads both |

- **The dependency runs one way.** EinAf reads Ein and links ein.rs. Nothing of Ein reads, links or names EinAf:
  no crate, no page of `docs/ein/`, no decision, no comment, no test, no module of the library, no corpus entry.
  A change the kernel takes for EinAf's sake ([Q-M2.9](../kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md))
  is stated in Ein's terms. The root `README.md` is the repository's page and names both.
- **Inside the development process**, which corrects the default: EinAf is planned under `plans/`, its questions
  are this ledger's, `AGENTS.md`'s standing rules hold for it, and its build and tests are steps of the gate once
  they exist. It is not *external to both*; `c/`, `nl2smt/` and `docs/lib/` stay that.

**Done with the ruling:** the root `README.md` and `AGENTS.md` state the hierarchy. No tree is created by it:
[S2.1.1](../../../../docs/einaf/README.md) creates them.

**Ruled further, 2026-11-25**, to the first of the defaults below: *"One tree - yes, preferred, docs/einaf/ -
separately"*. So item 1 is the user's; the others stay defaults.

**What the first two answers did not say, each taken by default and the user's to correct, but the first:**

1. **One tree a project at the root** (ruled). The plan's other root directories (`grammars/`, `benchmark/`,
   `experiments/`, `packaging/`) are folders of `einaf/`, so that the root shows the hierarchy, and EinAf's
   documentation is `docs/einaf/`, apart from it.
2. **The checks of Ein's prose are Ein's.** The release-voice check and the rules of `docs/ein/` (the current
   state only, no date, no measurement) are scoped to Ein: `einaf/` and `docs/einaf/` are named in
   `utils/check_release_voice.py`'s `NOT_PROJECT` when they are created. EinAf's own pages use the research
   plan's vocabulary (F0 to F8, B0 to B6, G1 to G9), which that check reads as followup ids, and its records are
   dated measurements by nature. What EinAf's pages hold themselves to is decided with its first page.
3. **A gate step of its own.** EinAf is a Cargo package outside the engine's workspace, built and tested by its
   own step of `run_tests.sh` and of CI. What needs the model file and the GPU is an experiment, run by hand with
   its record, in no tier: the runner has neither.
4. **The benchmark's programs stay out of the corpus roots** (`examples/`, `stdlib/`, `tests/`), where each would
   need a `corpus.toml` entry and would join every sweep of Ein's gate; EinAf's own command checks their claims,
   by the library's functions or by the binary's `ein test`.
5. **Where an answer about EinAf lands**: in this ledger while EinAf is planned, and in `docs/einaf/` once the
   thing it decides is built. `docs/decisions/` is about Ein
   ([Q-M1f.346](../../../../docs/decisions/docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).

**Proposed by S2.1.1, with EinAf's first pages
(2026-11-25): what EinAf's pages hold themselves to.** A proposal, the user's to correct; the pages written so far
([`docs/einaf/`](../../../../docs/einaf/README.md)) follow it.

1. **The current state, as Ein's pages do.** A page that stops being true is rewritten or deleted; no page names a
   plan, a phase, a stage, a task or a date, and none links into `plans/`: EinAf is a project, and the process
   reads it as it reads Ein. An answer about EinAf is written into the page of the thing it decides, once that is
   built; until then it is this ledger's, and a page does not cite it by id.
2. **A dated measurement is a record, not a page**: one run, immutable, under `einaf/experiments/`. A number a page
   states carries the command or the test that re-takes it, as Ein's do
   ([Q-M1e.4](../../../../docs/decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md)); the corpus counts
   of S2.1.1's sweep are in its record, and the page names the test that prints them.
3. **The research plan's labels are EinAf's vocabulary**: F0 to F8 (what the kernel is allowed to say), B0 to B6
   (the arms), G1 to G9, written in its pages and its code as they are in [`EinAf.md`](../../EinAf.md). This is why the
   release-voice check does not read EinAf's trees: it would read `F2` as a followup id.
4. **Named as Ein's pages are**: `NN-kebab-name` in reading order with a `README.md` index; a diagram is a file
   with its picture beside the page, under `diagrams/`. `utils/check_doc_blocks.py` reads `docs/ein/` only, and
   reads a `diagrams/` folder under `docs/einaf/` without the page that shows it, so EinAf's first diagram needs the
   check to read EinAf's pages for its rule 1.
5. **What they may link**: Ein's pages, tests and decisions, and EinAf's own code and tests. A decision of Ein's
   they cite resolves to its file: `utils/doc_audit.py --links --check --tracked` reads EinAf's pages for links,
   anchors and decision ids as it reads Ein's.
6. **What enforces it today**: the link check alone. Items 1 to 4 have no check yet; one is proposed for when the
   pages grow, either a check of EinAf's own (no plan id, no date, no path into the process; the labels allowed) or
   the release-voice check with EinAf's labels allowed.
