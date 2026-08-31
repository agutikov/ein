# S1e.4.6 — Code ↔ doc consistency (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 1 day
**Depends on:** nothing.
**Findings:** [`CD-L1`](../review/code-doc-consistency/low.md),
[`CD-L2`](../review/code-doc-consistency/low.md),
[`CD-L3`](../review/code-doc-consistency/low.md).

## Context

Three small wrongnesses, each in a place a reader is entitled to trust.

**`CD-L1` — the five history-page banners.**
`docs/api/{ein,ir,kb,inference,trace}.md` carry the same 🏛 banner, and it
enumerates *"the CLI: `ein solve` · `ein saturate` · `ein render` ·
`ein kb`"*. There have been **five** subcommands since M1c S1c.1.3 — `ein
test` is missing. The banner is the one part of a history page that is
supposed to describe the **present**, and it is copied identically five times,
so one fix is five edits.

**`CD-L2` — the guide's transcript.**
[`04_solving_the_whole_puzzle.md:66-68`](../../../docs/guide/04_solving_the_whole_puzzle.md)
shows two bindings per line, in a different order from what the binary prints
(one per line: `h_water`, `h_zebra`, `who_water`, `who_zebra`). The content is
right and the layout is wrong, so a newcomer diffing against a real run sees a
mismatch on the tutorial's final page. **Nothing runs the guide's
transcripts** — the same rot class the embedding test was built to prevent,
unguarded here.

**`CD-L3` — `render_lattice`'s fallback comment.**
[`lattice_dag.rs:288-292`](../../../ein.rs/crates/ein-render/src/lattice_dag.rs)
emits *"no stored lattice (store_lattice=False) — showing the solution
frontier instead"* — even when the solve ran **with** `store_lattice = true`,
which `ein render lattice` always does
([`render.rs:79-84`](../../../ein.rs/crates/ein-cli/src/render.rs),
[`cmdline.rs:146-150`](../../../ein.rs/crates/ein-cli/src/cmdline.rs)). The
real reason — no per-commitment `SetNode` DAG exists at all — is stated
correctly by the `--view` help text, so the tool contradicts itself at two
surfaces a user sees in the same session.

## Acceptance

- The five banners name five subcommands.
- The guide's chapter-4 transcript matches a real run, and a decision is
  recorded about whether anything will keep it that way.
- The fallback message states the real reason, and agrees with `--view`'s help
  text.

## Tasks

### Task T1e.4.6.1 — `CD-L1`: five banners, one edit

Add `ein test` to the enumeration in all five pages. Then check the rest of
the banner against the present, since the same staleness applies to every
sentence in it — it is the part of those pages that is *supposed* to be
maintained, and it has been maintained once.

Consider whether the banner should be a single included fragment rather than
five copies. In a markdown tree with no include mechanism the honest answer is
usually no, and the alternative is a grep-able marker so the next edit finds
all five — which is what
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s rule asks for when
unification is not available.

### Task T1e.4.6.2 — `CD-L2`: re-paste the transcript, then decide about the guide

Re-paste from an actual run — that is ten minutes. The decision is the rest:
**does anything keep the guide's transcripts true?**

The guide has four chapters and at least one big transcript. The embedding
page solved this exact problem with a marked region diffed by a test, and the
mechanism is available: mark the transcript, add a test that runs the command
and diffs. Its cost is one test and a rule (*edit the test, run it, paste;
never edit the block by hand*), and its known blind spot is prose outside the
marker ([CD-M4](../p1e.3_medium/s1e.3.7_code_doc_consistency.md)).

Take the decision explicitly rather than leaving it: either the guide's
transcripts are pinned like the API page's, or the guide is documented as
hand-maintained with a re-check on the doc-pass checklist
([T1e.2.2.5](../p1e.2_high/s1e.2.2_code_doc_consistency.md)). The guide is the
newcomer's first contact with the tool and a wrong transcript there costs more
trust than a wrong number in a README.

### Task T1e.4.6.3 — `CD-L3`: say the real reason

Fix the comment and the emitted message to name what is actually true — no
per-commitment `SetNode` DAG exists — and align the wording with `--view`'s
help text so the two surfaces agree. While there, check whether the
`store_lattice=False` phrasing appears anywhere else; a message written in
Python's boolean spelling is a good marker for other survivors of the port.

## Notes

One commit for the three, with the caveat from
[P1e.4](README.md#stages): `CD-L2`'s decision about pinning the guide's
transcripts is not a one-liner and comes out of the batch if it turns into
work.

---

## ✅ Done 2026-09-01 — one banner, one fabrication, and a sentence that was false twice

All three **fixed**, and two of the three came with something the review had
not seen.

### `CD-L1` — five banners, and a sixth site

The five banners are byte-identical (md5 `b236ecda…` over the marked region),
so one edit was five — and **six**: `README.md`'s crate table carried the same
four-item enumeration, added the same day (2026-08-23) and stale for the same
reason (`ein test`, 2026-08-24). Every *other* sentence of the banner was
checked against the present and holds: both dates, *three trip-wires*, both
anchors, and *kept whole and unedited* (the one commit on those pages since is
a link retarget).

Six edits are a diff, and five hand-synchronised copies of one text is
[`AR-M1`](../p1e.3_medium/s1e.3.4_architecture.md)'s shape. Markdown has no
include mechanism, so the state available is the third that rule allows —
**mechanically compared by a test** — and the marker T1e.4.6.1 asks for and the
test's extraction anchor are the same thing:

- `api_banner::the_five_banners_are_one_text` — the `<!-- api-history-banner -->`
  regions are one string;
- `api_banner::the_banner_names_every_subcommand` — every top-level `COMMAND
  ein <name>` in `golden/help_shape.txt`, which already owns the CLI surface,
  is named in the banner. The banner cites an owner instead of becoming a sixth
  list.

Controls: dropping `ein test` from one page fails **both** tests, the second
naming the missing subcommand. `README.md` has no possible pin — nothing in the
tree reads it — so it ends as an edit, and the asymmetry is stated rather than
papered over.

### `CD-L2` — the transcript was fabricated, not drifted

Two hunks, not one: the bindings *and* a missing 62-character rule under the
title. And `git show b4d5158` adds that text on 2026-06-17 while the engine
shipping that day already printed one binding per line under a rule — `ein.py`'s
`_two_col` / `_rule`, which is what `answer.rs` still does. **No engine in this
repo's history has printed what the page printed.**

That changes the fix's shape: a pin catches drift; only *taking the bytes from
a run* catches fabrication. So the block is generated —
`<!-- transcript: ein solve examples/zebra2.ein -->`, run and diffed by
`guide_transcripts::every_marked_guide_transcript_is_a_real_run`, `EIN_BLESS=1`
to re-bank. The marker carries the **command**, so pinning a second block later
is adding a marker rather than editing the test.

**`README.md` carries the same run and was right** — it is the copy the guide's
drifted from, and it is marked too, at no cost (blessing it was a no-op).

T1e.4.6.2's decision, taken explicitly: **chapter 4 is generated, chapter 2's
three blocks stay hand-maintained.** They are *excerpts* — header, rule and
empty `query bindings` elided — and an exact diff cannot express an elision;
pinning them would push seven lines of `(query has no :goal-text template)`
into a tutorial to satisfy a test. Their lines are byte-correct as excerpts.
Which kind each block is now has a table on
[`docs/guide/README.md`](../../../docs/guide/README.md), replacing a sentence
that was **false of half the guide**: *"each chapter ends with the exact command
to reproduce what it shows"* — chapters 1 and 3 carry no command at all.

### `CD-L3` — false twice over, and it cost 338 lines to stop saying it

The note is not a code comment: it is a string pushed into the emitted DOT, and
`ein render lattice --view full` prints it. It is **false in two independent
ways**, which is why this is not a `defined_behaviour.md`-style port survivor
(*true in an alien spelling*): every caller that can reach the line sets
`store_lattice = true`, and `LatticeProof` has **no `kb_index` field** for any
value of the flag to populate. A third surface disagreed too —
`04_dot_rendering.md`, a *current* kernel page, said `full` *"needs
`store_lattice=True`"*.

The argument for keeping ein.py's bytes had a premise that left with the oracle
at S1a.10.5. What stopping cost, **named before it was taken and then
measured**:

| golden | rows | |
|---|---:|---|
| `corpus_shapes.md5` — `dot[lattice-full]` | 136 | the view itself |
| `corpus_shapes.md5` — `dump[snapshot]` | 136 | via `snapshot_shape`'s `=== dot full` |
| `corpus_shapes.md5` — `trace[trace]` + `trace[answer]` | 34 + 31 | the markdown trace embeds the DOT — **the investigation predicted these would not move, and the adversarial pass caught it** |
| `dump_snapshot_subset-pruned.txt` | 1 | verbatim |

**337 of 9 015 digests, and not one line count changed** — that equality is the
check that the substitution is intra-line and nothing else moved. This is the
phase's **second** named re-bless; the phase README expected one
([MA-L2](s1e.4.8_maintainability.md)).

`FULL_VIEW_FALLBACK` is one exported constant now, and the acceptance line
*"agrees with `--view`'s help text"* is checkable rather than a matter of
inspection: `ein-render` cannot see `cmdline.rs`, so
`help_surface::the_full_view_fallback_agrees_with_the_help_text` holds them
from the `ein-cli` side, and
`presentation_semantics::the_full_lattice_view_is_the_solution_view_plus_one_honest_note`
holds the note, the absence of `store_lattice`, and the fact nobody had written
down: **the two views differ by exactly that line**. `LatticeView::Full`
appeared in no assertion in the workspace before this.

While there, the port-survivor grep the task asks for: `store_lattice=False`
survives in exactly one other place, `docs/api/inference.md`'s Python signature,
where it is correct — that page is a record of a Python API and `False` is how
Python spells it.

**Gate:** `./run_tests.sh` — exit 0, **818 tests**, 338 golden lines re-blessed
and every one of them the same sentence.
