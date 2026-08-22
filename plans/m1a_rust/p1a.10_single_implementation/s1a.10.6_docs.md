# S1a.10.6 — The docs after the oracle

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Status:** ✅ **shipped 2026-08-21**
**Depends on:** [S1a.10.5](s1a.10.5_removal.md)

> **Instruments (M1a S1a.10.6).** This document names `ir_oracle.py` and

## Context

Three documentation trees describe the repo, and each is wrong in a different
way once the Python engine is gone:

- **`docs/api/`** is *the Python embedding contract* — `parse` →
  `KnowledgeBase` → `solve` → verdict. Its subject moves from ein.py to
  [P1a.9](../p1a.9_release/README.md)'s PyO3 module, and **that
  module does not exist yet**: the phase dependency was reversed on
  2026-08-21, so P1a.9 runs *after* this one. This stage therefore cannot
  give `docs/api/` a subject. What it must do instead is say so **on the
  pages themselves** — a documented API that quietly names a dead module is
  the failure mode; one that names the stage where its implementation lands
  (S1a.9.1, documented
  by [S1a.9.4](../p1a.9_release/s1a.9.4_documentation.md)) is a
  plan. The contract itself is meant to survive unchanged; that is what
  S1a.9.2 is for.
- **`docs/kernel/`** is the specification ein.rs implements, and it is now the
  *only* statement of intent that is not also the implementation. It gets more
  load-bearing. `docs/kernel/inference/python_impl.md` is the exception — it
  describes the Python engine's internals and has no subject.
- **`CLAUDE.md` / `AGENTS.md`** describe a two-implementation repo in almost
  every section.

> **Sized by [S1a.10.5](s1a.10.5_removal.md), 2026-08-21.** The removal left
> **224 dangling markdown links** into the deleted tree, and they are all in
> the two trees this stage owns: **220 in `docs/kernel/`** and **4 in
> `docs/api/`**. Everywhere else — `examples/`, `stdlib/`, `utils/`, `ein.rs`
> source comments — was 8 links and S1a.10.5 fixed them, because they were
> its own breakage rather than a documentation decision.
>
> The 220 are not a find-and-replace. Most are of the form
> "[`world.py`](…/ein/inference/world.py) is the boundary", i.e. **a claim
> about the specification, evidenced by a pointer into an implementation** —
> which is precisely the shape the acceptance below says must go. Some have a
> `ein-infer` counterpart to point at instead; some are describing behaviour
> that is now specified nowhere else, and those are the interesting ones. The
> count is here so the stage is estimated against the real number rather than
> discovered mid-way.
>
> `docs/kernel/inference/python_impl.md` is **34** of the 220 on its own, and
> is the one file whose subject is gone entirely — every row of its module
> table points at a file that no longer exists.

## Acceptance

- ✅ `docs/api/` no longer describes a module that can be imported from this
  repo, and **every page says which stage gives it one** rather than reading
  as current. Describing the PyO3 surface, and verifying the worked example
  against it, is
  [S1a.9.4](../p1a.9_release/s1a.9.4_documentation.md)'s — this
  stage's job is that no page is *false* in the interval.
- ✅ `docs/kernel/` contains no claim that rests on "ein.py does X" as evidence.
  Where the Python implementation *was* the specification of a quirk — the
  `%ignore` delayed-match parse-error positions
  ([Q-M1a.3](../open_questions.md#q-m1a3--parse-error-message-parity)),
  `sorted()` over mixed-type args ([D2](../divergences.md)) — the quirk is now
  **ein.rs's own defined behaviour** and has to be *stated*, not referenced.
  This is the substantive half of the stage.
- ✅ `CLAUDE.md` describes the tree that exists: no `ein.py/`, no
  `nlp/`/`smt/`, one engine, one gate.
- ✅ `docs/guide/` and `docs/lib/` re-checked for invocations that assumed the
  Python CLI.
- ✅ The milestone's own documents keep their history: **P1a.0–P1a.9 were
  written against an oracle and their numbers are real**. They are not
  rewritten, they are read as history — and where a document's *instrument*
  is gone, a line says so.

## Tasks

### Task T1a.10.6.1 — `docs/api/` re-pointed

**Done.** All six pages carry the same banner under the title: *`import ein`
does not work in this repo*, the module was deleted at S1a.10.5, and the
contract is now a **specification** — S1a.9.1 builds what satisfies it,
S1a.9.2 checks it, S1a.9.4 re-verifies these pages sample by sample. The four
"Source: `ein.py/src/ein/…`" lines name the crate behind the surface instead,
`ein.md`'s worked example is labelled as the script the contract has to make
work rather than as a runnable one, and the `PYTHONPATH=ein.py/src
.venv-pypy/bin/python` invocation is gone.

The six "*Verified against commit `60c192b`*" stamps were the quiet lie: they
said *verified* about an implementation that no longer exists. Each now says
so, and `README.md` § Stability records that its own conditional — "if the
M1a port ships, the contract moves to ein.rs" — **resolved**, which inverts
how the pages read: they no longer describe an implementation and get checked
against it, they specify one.

### Task T1a.10.6.2 — `python_impl.md` retired

**Done, and as a third option the stage did not list.** Both `python_impl`
pages were **renamed and re-aimed** rather than deleted or reduced to a note:

| was | is |
|---|---|
| `inference/python_impl.md` | [`inference/implementation.md`](../../../docs/kernel/inference/implementation.md) |
| `ir/02-data-model/03_python_impl.md` | [`…/03_implementation.md`](../../../docs/kernel/ir/02-data-model/03_implementation.md) |

The argument for deleting them is that a file-by-file map of an
implementation is not specification, and `docs/kernel/` is a specification
tree. The argument that won is that the **dev reading path needs an
orientation into the code**, `docs/kernel/README.md` is where it is offered,
and ein.rs has no README of its own — so deleting the map would have removed
the only such page and left the reading path pointing at nothing. Both files
now open with a banner saying *this page is a map, not a specification*, which
is the distinction the deletion argument was really about.

The two are not the same job. The **engine** map is close to a rename: the
module *roles* port one-for-one, and the five places the layouts differ are
flagged **⤳** in the table — `world.py` (no counterpart: the boundary is
`saturator.rs`'s `admit_from_boundary` / `first_failing` /
`negative_premises`, asking the KB at quiescence directly), `solution.py` →
`hypgen.rs`, `frontier.py` → `explain.rs`, the whole `monotonic/` package →
`solve.rs`, and the dumps → `ein-render/dump/`. The **data-model** map is not
a rename at all: §2 and §3 described CPython mechanics — frozen dataclasses,
`object.__setattr__`, dict shapes — and almost none of it survived. What
replaced each is stated, including the one caveat the port *deleted* rather
than reproduced: the `_kb` back-pointer that answered for the root when asked
on a fork.

### Task T1a.10.6.3 — The quirks, restated as ein.rs's own

**Done — [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md),
thirteen behaviours.** The stage said "each becomes a paragraph in
`docs/kernel/`"; they became one page instead, cross-linked from the pages
that own each area, because the set has to be *auditable* — a behaviour that
is only defined by the engine is exactly the thing you want to be able to
enumerate, and distributed paragraphs cannot be counted.

| § | behaviours |
|---|---|
| 1 — parse and load diagnostics | `-1:-1` at EOF · the ±40 window applied before the line is trimmed · the `%ignore` delayed-match column · ambiguity resolving to the earlier alternative · `at None` |
| 2 — values and order | `Int < Sym < Fact` with lexicographic symbol rank · CPython `repr()` where a value is printed or display-sorted · `format(x, spec)` for the `f` type · state identity as the sorted list, never a hash |
| 3 — search | `--shuffle` as CPython's MT19937 exactly · a firing's identity ignoring non-string activator args |
| 4 — errors and exits | the six Python exception classes the CLI prints, and their exit codes |
| 5 — the CLI surface | what stays exact, and what is `clap`'s |

Three things the page does that a list of quirks would not. It says **why the
framing changed**: "because CPython did" was a good reason — reproducing them
exactly is what made the port measurable — and is not a reason any more; what
holds them in place now is that every checked-in fixture, golden and expected
output was baselined against them. It **names the two items that are latent
bugs rather than quirks** — the binding key dropping non-string activator args
(a puzzle with integer rule parameters can lose a firing, with no diagnostic)
and the Python class names, which are now a name without a referent — and says
why neither has been changed. And §6 records the one rendering that is
genuinely **under-determined** rather than defined: the unsorted `rows[0]` a
solve table prints for a multi-row goal, which moves under a permuted id space
in one engine and is filed in `corpus/fuzz_findings/`.

### Task T1a.10.6.4 — `CLAUDE.md` / `AGENTS.md`

**Done.** The `ein.py/` bullet is deleted. `docs/kernel/` gains the two
renamed maps, the new page, and the sentence that matters most for an agent
working here — *this tree is now the only statement of intent that is not also
the implementation*. `docs/api/` gains **"do not fix these pages to match
ein.rs's internals"**, because the obvious repair is the wrong one: a
disagreement between those pages and the PyO3 module is a defect for S1a.9.2,
not a stale doc. `stdlib/` loses "shared by both implementations" and the
wheel copy; `ein.rs/` gains the five shipping crates and their linear stack.

### Task T1a.10.6.5 — The instrument sweep

**Done — 24 documents marked**, one line each, naming the instruments that
document cites and pointing at [`utils/README.md` § The
census](../../../utils/README.md#the-census) for what answers each one's
question now. The marker is uniform and sits under the metadata block:

> `ein-conformance`. They are gone … so the numbers here are a **record**, not
> something you can re-run.

Four documents already carried the statement from
[S1a.10.4](s1a.10.4_utils.md) (`baseline.md`, `design/README.md`, `design/12`,
`features.md`) and were left alone; P1a.10's own six documents are excluded,
because describing the removal is what they are for.

Three more got a marker of their own kind rather than the uniform one, because
they are read for *numbers* and the reader meets a stale column before any
banner: `features.md` (the frozen note existed but sat below four tables —
now one line above the first), `architecture_and_algorithms.md` §7 (which
attributes every figure to one engine or the other, and now says which half is
re-measurable), and `parity_baselines.md` (every wall-clock on the page is
frozen; the *verdict* column is what the corpus sweep still checks).

## What the sweep actually cost

The 224 dangling links resolved into four different jobs, and the ratio is the
useful record:

| job | count | how |
|---|---|---|
| a module pointer with a 1:1 counterpart | ~150 | mechanical, by a table of 60 path mappings |
| link *text* naming a `.py` file | ~60 | mechanical, renamed to the target's own basename |
| a **symbol** that does not exist in ein.rs | 46 | by hand — `world.World` → the boundary phase, `compile.split_naf` → the guard lift, `Provenance.absent_premises` → `Prov::absent`, `frontier.smallest_contradiction_frontier` → `explain::…` |
| a **claim** that only made sense with two engines | ~20 | rewritten: the determinism section's cause (CPython hash randomisation → *any* hash iteration, which is why the rule is stated over the read site), the seam's `World` module, the `.expected` files' provenance, four measured tables |

Everything in the first two rows is a link that was never worth much; the last
row is the stage's actual subject, and it is 9 % of the count. That is the
shape [S1a.10.5](s1a.10.5_removal.md) predicted when it said the 220 "are not
a find-and-replace" — but the prediction was about the *median* link, and the
median link was in fact mechanical. What was not mechanical was small, and
would have been invisible without walking all 224.

**Five test references had to be re-pointed rather than dropped**, because a
spec page citing "the test that pins this" is the strongest thing on it:
`test_absent_semantics.py` and `test_root_stability_naf.py` →
`naf_semantics.rs`, `test_shuffle_invariance.py` and `test_lattice_dumper.py`
→ `lattice_semantics.rs`, `test_zebra_parse.py` → `cli_semantics.rs`, all from
[`suite_dispositions.md`](suite_dispositions.md).

**One diagram was stale and is re-rendered**:
`inference/diagrams/algorithm_layer_n.dot` named `inference/monotonic/solver.py`
and `nogoods.py` inside node labels, so the SVG in the tree was a picture of a
package that does not exist.

## Notes

- The most valuable thing this stage produces is T1a.10.6.3. A quirk that was
  only ever defined as "whatever ein.py did" becomes undefined the moment
  ein.py is gone, and undefined behaviour in a *specification* repo is worse
  than a quirk.
- **The link checker is not checked in**, and that is the one loose end. This
  stage ran a 40-line script over `git ls-files '*.md'` and drove the count
  from 242 to 7 (all seven are false positives: two template placeholders,
  one piece of `[R,S,T](x)` notation, two prose ellipses, and two paths into
  an unrelated repo). Nothing stops the count climbing again. Making it a
  `cargo test` is a real check with a real cost — it wants an allow-list for
  those seven — and it belongs to whoever next finds a dead link, not to a
  stage that has already fixed them all.
