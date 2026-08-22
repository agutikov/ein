# S1a.9.4 — Documentation

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 1 day
**Depends on:** [S1a.9.3](s1a.9.3_packaging.md)
**Implements:** the milestone's closing obligation

> **Amended twice on 2026-08-21.** First for
> [P1a.10](../p1a.10_single_implementation/README.md): the stage was written
> for a tree with two implementations and would have said "ein.py is the
> oracle, ein.rs is what ships", and with the first gone that framing went
> with it.
>
> Then again, when the phase's two binding stages were deferred (the
> [phase README](README.md)'s scope change,
> [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).
> The intermediate version said the documentation has to describe **two
> surfaces, the CLI and `ein_rs`**. There is no `ein_rs`. The shipped surfaces
> are **the binary and the crates**, and `docs/api/` — six pages, 1 051 lines
> — specifies neither.
>
> The division of labour with
> [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md) still holds and
> is the thing to keep straight. S1a.10.6 removed what became false when the
> Python engine left, and stated the one gap it could not close: `docs/api/`
> documents an embedding API with no implementation. **This stage closes that
> gap — by changing what `docs/api/` is about, rather than by implementing
> what it currently says.** It is the last documentation stage in the
> milestone.
>
> **The ex-S1a.9.5 note folds in here.** It read *"forget about removed
> ein.py — find all occurrences, analyze, if it is a reference to removed
> ein.py then reword for ein.rs or delete"*, and it was a stage whose entire
> content is this stage's last acceptance item. It is that item, unchanged.

## Context

The repo's documentation described ein.py as *the* implementation:
`docs/kernel/inference/python_impl.md` was the engine internals page,
`docs/api/` is "the Python embedding API", and `AGENTS.md` oriented a reader
entirely around `ein.py/`. S1a.10.6 took the false half out. What it could
not do is give `docs/api/` a subject: the PyO3 module that was to be its
subject does not exist, and — since the binding was deferred on 2026-08-21 —
is not going to. Giving those pages a subject therefore means **changing what
they are about**, not waiting for an implementation.

This stage makes the documentation say what is true, without rewriting the
kernel docs (which are implementation-independent by design and stay exactly
as they are — they are the specification the engine implements).

The `docs/api/` decision is the substance. Those pages are a *good* contract
— five steps, a worked example with concrete numbers, per-symbol tables — and
the temptation is to keep them warm against the day a binding arrives. That is
what the last four months already tried: they have carried a banner saying
"this has no implementation" since S1a.10.6, and a specification nobody can
run is indistinguishable from a specification nobody checks. They move to
history, intact, so that
[Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
tripping restores a contract rather than starting one.

## Acceptance

- A reader arriving at the repo learns, within one screen, what ein is,
  that ein.rs is the implementation, and how to reach it — as a binary or as
  crates, which are the two ways there are.
- **`docs/api/` documents a surface that exists.** Its subject becomes the
  **Rust** embedding API — the crates, which is what
  [M1b](../../m1b_gui/README.md) binds against and what nothing in the tree
  documents today — and the five Python pages are filed as history with a
  header saying what they specify and what would revive them. No page in
  `docs/api/` describes an import that fails.
- **The Rust embedding page's claims compile.** Its examples are a doctest or
  an `examples/` target in the workspace, so `cargo test --workspace` — the
  gate — is what keeps them true. This is the substitute for the contract
  suite the deferred S1a.9.2 would have been, and it is cheaper *and* stronger
  on the one axis that matters: it cannot rot without the gate going red.
- `plans/README.md`'s status table records M1a as shipped, with its date and
  its measured outcome.
- Every number this milestone changed is updated where it is quoted, with
  the old value **labelled** rather than deleted — and every CPython/PyPy
  figure labelled *frozen*, since nothing can re-measure one
  ([S1a.10.4](../p1a.10_single_implementation/s1a.10.4_utils.md)).
- **`git grep -i 'ein\.py'` returns only history**: the divergence ledger,
  the phase records, and dated measurements. No page describes it as
  something a reader can run. (The user's standing requirement, recorded
  here at the phase that closes the milestone: *after P1a completion the
  repository must not contain any reference to the Python implementation
  except as record; ein.rs is the reference implementation*.)



## Tasks

### Task T1a.9.4.1 — Orientation

`AGENTS.md` (= `CLAUDE.md`): S1a.10.5 and S1a.10.6 have already cut the
Python tree out of *Where things live*. What this stage adds is the shipped
surface — **the binary and the crates** — and where each is documented.
Terse; the file is a map. Its current `docs/api/` bullet says the module is
"P1a.9's PyO3 one"; that sentence is this task's first edit.

`README.md`: the same, plus install pointers (binary download,
`cargo install`).

### Task T1a.9.4.2 — Engine internals

`docs/kernel/inference/` needs **one** engine-internals page describing
ein.rs at the altitude `python_impl.md` used: the integer data model, the
register matcher, the layered KB, the parallel levels. Whether that is a new
`rust_impl.md` or the old page rewritten in place is S1a.10.6's call, since
that stage decides what happens to `python_impl.md`; this one owns the
*content* either way. A line at the top saying
`architecture_and_algorithms.md` is the shared vocabulary.

### Task T1a.9.4.3 — Embedding docs

**Write the Rust embedding page.** `docs/api/` gains the page it has never
had: how to drive the engine from another Rust program — `ein-ir` to parse
and load, `ein-infer` to saturate and solve, `ein-render` to explain,
`ein-einb` to cache a loaded KB. It is the surface
[M1b](../../m1b_gui/README.md)'s Tauri backend binds against, the surface
[M1c](../../m1c_external_validation/README.md)'s `ein-bench` uses, and the
surface an NL frontend would use if
[M2](../../m2_nl_to_ir/README.md) is written in Rust. Three consumers, no
page.

Mirror the Python contract's shape, because the shape was the good part: the
five steps, one worked example on `zebra2.ein` with real numbers in it, then
per-area detail. Its examples compile under `cargo test --workspace`.

**File the five Python pages as history.** `ein.md`, `ir.md`, `kb.md`,
`inference.md`, `trace.md` keep their text and swap their banner: not "this
has no implementation yet", which promises one, but what they are — the
embedding contract of the engine that was, preserved because
[Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
would make them a specification again on the day it trips. `docs/api/README.md`
routes a reader to the Rust page first and says plainly that there is no
Python module.

**Two known defects go with them rather than being fixed in place.**
`ir.md`'s `parse_tree(text) -> lark.Tree` names a parser generator ein.rs
does not use and
[design/04](../design/04_ir_frontend.md) rules out — the working tree already
carries a `TODO: What lark in Rust?` against that line, and the answer is that
the symbol does not survive. And `inference.md`'s `SolverConfig` table has no
`--jobs`, because [P1a.7](../p1a.7_parallelism/README.md) is paused; the Rust
page states the parallelism surface as it actually is on the day it ships.

### Task T1a.9.4.4 — Update the quoted numbers

`docs/kernel/inference/features.md` (regenerated in
[S1a.6.7](../p1a.6_performance/s1a.6.7_relever_matrix.md)),
`architecture_and_algorithms.md` §7's cost split, and any stage or
followup that quotes a wall-clock. Label the old numbers with their date
and engine rather than deleting them — the arc from naive → semi-naive →
fail-fast → boundary → native is the interesting part, and it is the
project's memory.

### Task T1a.9.4.5 — Close the milestone

`plans/README.md`: status table row for M1a → **shipped**, with the date
and the headline measurement. `plans/m1a_rust/README.md`: a closing
`**Status:** done — <date>` line under the heading, per the repo's
"don't delete; the trail is the project's memory" rule.

Update [`divergences.md`](../divergences.md) with its final state and
[`open_questions.md`](../open_questions.md) with each question's resolution.
"Ideally empty" is no longer the bar and should not be pretended to be:
D1–D3 record where two implementations differed, one implementation is left,
and the file is **history that stays** — the phase's own non-goal says so.
What it should gain is a header saying that, and the note that a *new* entry
now means two surfaces of one engine disagreeing rather than two engines.

### Task T1a.9.4.6 — Downstream pointers

[M1b](../../m1b_gui/README.md): note that the engine it binds to is
ein.rs, linked as ordinary crates (its Tauri backend *is* the Rust
process — there is no server between them), and that `.einb`
([P1a.8](../p1a.8_binary_container/README.md)) is available for saved
sessions.

[M2](../../m2_nl_to_ir/README.md) needs more than a note, and this is the
milestone's last chance to leave it accurate. It currently says its frontend
is Python because ein was; that premise is gone, and three of its documents
name modules in a package that no longer exists (`python -m ein.llm.smoke`,
`python -m ein.ir.to_gbnf`, `src/ein/nl/*.py`). **This stage does not decide
M2's language** — [P2.1](../../m2_nl_to_ir/p2.1_investigations/README.md) does
— but it must stop M2's plan asserting a resolved boundary that was deferred:
PyO3 is not the boundary, there is no socket, and the two live options are a
Rust frontend linking the crates and a Python one driving the CLI. Record it
as an M2 open question and leave the choice there.

While editing: M2's `llama-server` is the right target and **ollama is not an
alternative for it** — GBNF is the mechanism
([P2.3](../../m2_nl_to_ir/p2.3_gbnf_for_ir/README.md),
[idea 01](../../ideas/01-self-modifying-constraint-language.md)), llama.cpp's
server takes a `grammar` field, and ollama's API exposes only JSON-schema
`format`. That is a fact about the tools, it is cheap to write down, and it is
expensive to rediscover.

[F11](../../followups/f11_deductive_layer_perf.md): closed or updated by
[S1a.6.7](../p1a.6_performance/s1a.6.7_relever_matrix.md).

## Notes

- Do not touch `docs/kernel/ir/**`. The language specification does not
  change in this milestone — that is the whole point of invariant I1 —
  and editing it would suggest otherwise.
- **`--jobs` is not a new user-facing concept yet.**
  [P1a.7](../p1a.7_parallelism/README.md) is paused after one stage, so the
  documentation states what ships — and if `--jobs` is not in the released
  binary, the honest page says the engine is single-threaded and links the
  paused phase. A documented flag that does not exist is the failure mode this
  whole stage is correcting.
- **`git grep -i 'ein\.py'` is the last check to run**, not the first: the
  earlier tasks create most of its remaining hits and resolve them. What it
  must end at is the standing requirement in the acceptance — only history:
  the divergence ledger, the phase records, and dated measurements.
