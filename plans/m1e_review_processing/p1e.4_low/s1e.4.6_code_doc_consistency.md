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
