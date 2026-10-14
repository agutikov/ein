# P1e2.1 — Seeing the answer: dumps, and the answer rendered for review

**Estimate:** ~2.5 weeks: 7 stages, 12 days.
**Depends on:** nothing. Every renderer this phase needs is already written,
and three of them are already tested. What is missing is the way to reach them
from the CLI, a file per answer, and one page that puts a wrong run beside its
control and its right answer.
**Blocks:** [P1e2.2](../p1e2.2_wrong_answers/README.md), every stage of it.
No wrong answer is fixed until the user has *looked* at it, and
[S1e2.1.7](s1e2.1.7_the_review.md) is where that happens.
**Source:** the user's instruction of 2026-10-14: *"first phase is for
solution dumps and rendering to markdown with graphs, so I can review all
existing wrong solutions and initial problems."*

---

## Why seeing comes first

Every wrong answer in [P1e2.2](../p1e2.2_wrong_answers/README.md) was found by
someone **reading** it: a hand derivation in
[`solution_semantics.md` § 2](../../../docs/kernel/inference/solution_semantics.md),
a probe written for a question, a twin fixture one fact apart. None of them was
found by looking at what the engine answered. Today that is hard to do. A
model is a block of facts on stdout, a refutation is a fenced `dot` block that
no markdown viewer draws, and the **program**, which is the thing a reviewer
checks first, has no picture at all. Its two graph renderers are `pub`, pinned
by tests, and reachable from no command
([Q-M1e.20](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable)).

The second reason is that the inventory is a reading, too. The cases in
[P1e2.2](../p1e2.2_wrong_answers/README.md) were collected from questions,
probe files and fixture headers, and each one's *right* answer was derived by
hand or by a control run. A fix stage that starts from a misread case fixes
the wrong thing in a way no test can catch, because the test was written from
the same misreading. So the phase ends with a **review held by the user**,
and the second phase's stage list is edited to match its outcome before any
fix starts.

Q-M1e.20 was left open on one question: *who wants these views?* It names
two candidates. If the answer is *the GUI*, a flag is premature; if it is *a
reader chasing a rule that will not fire*, the flag is small. This phase is
the second candidate, and it answers the question.

## What exists, and what does not

Taken 2026-10-14 against `71dd237`.

| want | what exists | the gap |
|---|---|---|
| the program, as a graph | `ein_render::ir_dot` (the program as written, per form) and `ein_render::kb_dot` (one digraph over the whole KB, entities fused across forms: the 2021 prototype's `linked.svg`). Both swept by `dot_wellformed.rs` | **no CLI door**: `ein render` offers `rules` / `rule` / `constraints` / `lattice`, and `ein ir dot` / `ein kb dot` went in P1.11 (`8378ad7`) |
| the answer, as files | `--json-summary` (`verdict.solutions[].facts` + `goal_bindings`, `open_states`, `unsat_core`); `--dump-states DIR` (`00_root_initial.ein`, `00_timeline.jsonl`, `layers/layer_NN_{pre,post}.ein`, `summary.json`); `-p` / `-P` / `-f` on stdout | **a model is not a file**: no per-model, per-open-state or per-refuted-commitment file, and nothing records *which commitment* reached a model |
| every commitment tested | `LatticeDumper` (`enterings/`, `proof_summary.json`, [`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md)), pinned by `golden_dump.rs` + `dump_parity.rs` | **no CLI door**: `--dump-states` builds the other dumper |
| why a branch died, and every step | `--trace FILE.md --full-kb-snapshots`: every step, refuted hypotheses as reductio DAGs, the commitment lattice, § Solution and § Full KB (final state), as inline ` ```dot ` blocks. On `zebra2 -e`: **15 566 lines, 639 graphs** | **not viewable**: no markdown viewer draws a `dot` fence. `utils/zebra2_trace.sh --svg` rasterises and rewrites, for one hard-coded file |
| how the search went, layer by layer | `--dump-states DIR`: root before and after each layer as loadable `.ein`, and `00_timeline.jsonl` with every entering's commitment, outcome, firings and core size. `--events FILE`: the step-by-step record (**63 475** lines on `zebra2 -e`) | **not readable**: snapshots to diff by hand and JSONL to read by eye. Nothing renders a layer's pre → post difference or a timeline as a table |
| a model you can read | `--models key` (the determining key); `utils/model_set_census.py --form {envelope,key,list,diagram}` | **a model is a fact list**: nothing renders a Zebra-shaped model as the grid a person would draw, or as the graph of what the hypotheses *added* to root |
| wrong beside right | the witness fixtures, their twins, their header comments | **nothing puts them side by side**: the wrong run, the control run and the hand-derived answer live in three places, two of them prose |

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e2.1.1](s1e2.1.1_the_case_register.md) | The case register | 1.5 d | `corpus/wrong_answers.toml`: one row per **witness** of every case in P1e2.2 (W1–W12, C1), the inventory's probes banked as `examples/ein-bugs/` fixtures, each row with the witness, the wrong run, the control run and the right answer as data, plus a cargo test that pins every `open` case to its **wrong** answer, so that a fix by accident is noticed too |
| [S1e2.1.2](s1e2.1.2_every_answer_a_file.md) | Every answer a file | 1.5 d | `ein solve --dump-answer DIR`: the program, root, one file per model / open state / refuted commitment, each loadable, and the commitment that reached each one |
| [S1e2.1.3](s1e2.1.3_the_doors.md) | Three renderers get a door | 2 d | `ein render program`, `ein render kb [--state …]`, and `ein solve --dump-lattice DIR`; Q-M1e.20 answered **(a)**, with `configuration.md` and `help_shape.txt` moved deliberately |
| [S1e2.1.4](s1e2.1.4_a_model_you_can_read.md) | A model you can read | 2 d | a model rendered as the **grid** where the program's shape allows one and as **root + what the commitment added** everywhere; two model sets diffed (missing / spurious / per-fact) |
| [S1e2.1.5](s1e2.1.5_every_artefact_rendered.md) | Every artefact, rendered | 2.5 d | **one run, fully explicit**: every artefact the engine can write for it, including `--trace --full-kb-snapshots`, `--dump-states`, `--events`, `--json-summary`, `--dump-answer` and `--dump-lattice`, kept raw and rendered as pages. The trace is split and every graph rasterised, the timeline and every layer's pre → post diff are tables, events grouped by kind, and five cross-checks between the artefacts |
| [S1e2.1.6](s1e2.1.6_the_case_page.md) | The case page, and the bundle | 1.5 d | `utils/review_bundle.py` → `build/review/`: an index and one page per case, putting the wrong run, the control, any twin and the right answer side by side, the models diffed, and links to the exact layer, entering and reductio where it went wrong |
| [S1e2.1.7](s1e2.1.7_the_review.md) | The review | 1 d (+ the user's reading) | the user's verdict on every case (*confirmed* / *not wrong* / *right answer corrected* / *split* / *merge*), written into the register, and P1e2.2's stage list edited to match |

**Work order is id order except for one pair.** S1e2.1.2 and S1e2.1.3 are
independent and touch different flags. S1e2.1.4 needs both, because a grid is
drawn from a dumped model and a KB-diff graph from `kb_dot`. S1e2.1.5 can
start on the artefacts that exist today (`--trace -F`, `--dump-states`,
`--events`, `--json-summary`) before the doors land, and add the new ones
when they do.

## Acceptance

- **One command renders every case.** `utils/review_bundle.py` with no
  arguments writes `build/review/`, with an index and one page per register
  row, and every graph on every page is an SVG a markdown viewer draws (VS
  Code's preview and GitHub both, checked by opening one page in each).
- **Every artefact, every run.** Each run in the bundle carries **every**
  artefact the engine can write for it: the trace with full KB snapshots,
  the state dump, the events, the summary, the answer dump, the lattice dump,
  the counters, the program, rule, constraint and KB views. Each is kept raw
  and rendered as a page a person reads without decoding anything. A test
  fails when `ein solve` gains an artefact flag the bundle does not drive.
- **Every number on a page is the engine's.** The verdict block is the run's
  stdout verbatim; the model tables are read from `--dump-answer`, not
  re-derived by the script. The script **assembles** and does not compute,
  which is the line `utils/README.md` § Renderers already draws: `ein` writes
  DOT and markdown, and rasterising is a script's job.
- **The register is a test from the day it exists.** An `open` case that stops
  answering its recorded wrong answer fails `cargo test`, as a `fixed` case
  that stops answering its right one does.
- **The doors change no answer.** `corpus_exits.txt`, `corpus_shapes.md5` and
  every existing golden byte-identical. What moves is `help_shape.txt`, the
  option census in [`configuration.md`](../../../docs/kernel/configuration.md)
  (52 options plus the new ones), and `config_reference.rs`, all named in
  S1e2.1.3 before they move.
- **The review is held and written down**, per case, in the register. P1e2.2's
  table matches it.
- `./run_tests.sh` green at every stage boundary.

## Risks

- **Building a viewer instead of a door.** The temptation is a
  `ein report` that does everything in Rust. The phase's line is the existing
  one: the engine emits markdown and DOT for **one run**, and a script that
  needs several runs, the register and Graphviz lives in `utils/`. A
  per-case page in Rust would have to learn the register, which is a corpus
  file, and that is how `ein-cli` would grow a dependency on `ein-corpus`.
- **The bundle rots because nothing runs it.** It is generated, not committed
  (`build/` is gitignored), so the gate must run it or it will fail on first
  use six weeks later. S1e2.1.6 adds a smoke step; Graphviz is already a hard
  dependency of the gate (`dot_wellformed.rs`).
- **Detail that nobody can navigate.** "Every artefact" on `zebra2 -e` is 639
  graphs and 63 475 events. The answer is structure, not omission: every page
  opens by saying what it is and where its raw file is, the trace is split by
  section and step range, and the case page links to the *exact* layer,
  entering and reductio that matter (`look_at`), so explicitness never costs
  the reader the thread.
- **The review is skipped because the cases "look obvious".** Two of the
  cases were already re-derived once inside M1e: `lattice/02`'s header still
  says *Expected verdict: Contradiction* under the P1.5b mode contract, and the
  ruling of 2026-08-28 made it three solutions. Several more depend on that
  same ruling. The review is the cheap place to disagree with it.
- **`kb_dot` on a large KB.** `zebra2`'s unique model is 444 facts. A
  whole-KB graph of that is wallpaper, which is why S1e2.1.4's default is
  *what the commitment added* and the full KB is opt-in.

## Connections

- [Q-M1e.20](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable)
  is answered by S1e2.1.3.
- [`docs/kernel/ir/03-ein-lang/04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
  covers the views and their legend. § How to ask for this view is rewritten
  when the door exists.
- [`docs/kernel/inference/solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)
  defines a solution and a model. The bundle renders against it.
- [M20](../../m20_gui/README.md) is the other consumer Q-M1e.20 named. Nothing
  here pre-empts it: a GUI binds the crates, and these doors are thin CLI
  wrappers over the same calls.
- [M1f S1f.5.5](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.5_nl_required.md)
  takes every statement to NL. A model rendered as a grid is the
  non-NL half of the same wish, and the two should not invent two
  registers for one model.
