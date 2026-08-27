# S1e.4.7 — Documentation (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 1 day
**Depends on:** [S1e.3.8](../p1e.3_medium/s1e.3.8_documentation.md) — the
count/date/citation shape is decided there and applied here to the
measurements.
**Findings:** [`DO-L1`](../review/documentation/low.md),
[`DO-L2`](../review/documentation/low.md).

## Context

**`DO-L1` — four one-line defects, all inside pages the tree calls
normative.** That is why they are grouped rather than ignored:

| page | defect |
|---|---|
| [`defined_behaviour.md:304-328`](../../../docs/kernel/defined_behaviour.md) | *"Nine more"* above a **10-row** table later called *"all ten"* |
| [`06_reserved_names.md:230-233`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) | the keyword arithmetic — *"the six above plus `:goal`, `:goal-text`, `:hrules`"* — does not reconstruct the actual **7**-keyword allow-list |
| [`01_kb.md:26-33, 110-141`](../../../docs/kernel/ir/01-ein-graph/01_kb.md) | the table says relation nodes are round-rects; **both** Levi DOT examples in the same file draw them as hexagons — the shape the table assigns to `Rule` |
| [`03_examples.md:20-21`](../../../docs/kernel/ir/03-ein-lang/03_examples.md) | a garbled sentence: *"whose :source derives the a given"* |

The `01_kb.md` one is the least trivial: a reader learning the Levi encoding
from that page gets the node-shape vocabulary wrong, and the page's own
examples are the counter-evidence. Establish which is right from the renderer
before editing either — `ein render`'s DOT output is the authority, and
`dot_wellformed.rs` is what holds it.

**`DO-L2` — frozen measurements presented as current.** Two tables cite
re-takable measurement documents but inline numbers with **no re-take
mechanism**, and both predate M1d's engine changes:
[`examples/README.md:39-43`](../../../examples/README.md)'s 46.9 ms / 31.1 ms
zebra timings, and
[`docs/guide/03_rule_families.md:73-75`](../../../docs/guide/03_rule_families.md)'s
*≥ 23×, 101 → 3 336+ commitments*. S1d.2.4's activator facts changed fact
counts — `docs/api/rust.md` documents its own 434 → 444 move — and these
tables did not get the same audit.

## Acceptance

- The four `DO-L1` defects are fixed, and the `01_kb.md` shape question is
  settled against the **renderer**, not against whichever of the two page
  halves reads better.
- Every inline measurement in the two `DO-L2` sites is dated in place or
  replaced by a link to the measurement document that owns it — per
  [S1e.3.8](../p1e.3_medium/s1e.3.8_documentation.md) T1's decision, applied,
  not re-decided.
- `06_reserved_names.md`'s arithmetic reconstructs the allow-list a reader can
  check against the parser.

## Tasks

### Task T1e.4.7.1 — `DO-L1`: the four

Three are edits. The fourth needs a check first:

- **`defined_behaviour.md`** — *"Nine more"* / 10 rows / *"all ten"*. Count
  the rows, fix the two words, and note that this is the same page whose §3.2
  may have been deleted by
  [T1e.2.2.4](../p1e.2_high/s1e.2.2_code_doc_consistency.md) — so do this
  **after** that lands, or the count moves twice.
- **`06_reserved_names.md`** — reconstruct the 7-keyword allow-list from the
  parser and write the arithmetic so it adds up. This page also carries a
  [CD-H2](../p1e.2_high/s1e.2.2_code_doc_consistency.md) defect on adjacent
  lines (`:expect`'s third form given as `none`); one visit, both fixes, and
  the High stage's task says the same thing from its side.
- **`01_kb.md`** — settle the shape from `ein render`'s actual DOT output for
  a Levi view, then fix whichever of the table or the examples is wrong. If
  the *examples* are wrong, they are generated-looking prose that nothing
  regenerates, and that is worth a sentence on the doc-pass checklist.
- **`03_examples.md`** — repair the sentence. Read the surrounding paragraph
  to recover what it meant; *"derives the a given"* looks like a half-applied
  edit, so the intended claim may be recoverable from git.

### Task T1e.4.7.2 — `DO-L2`: date them or link them

Two sites, one rule, whichever
[S1e.3.8](../p1e.3_medium/s1e.3.8_documentation.md) T1 chose. For
measurements the recommendation there is **dated in place** — *as of the M1a
close* — because a timing is meaningful only with its machine and its commit,
and a bare link to a measurement document makes a reader chase a number the
sentence needed.

Then the audit the review says these tables missed: check both numbers against
today's engine, at least roughly. If `examples/README.md`'s 46.9 ms is now
materially different, the date alone is not enough — a stale number with a
correct date is honest, and a stale number a reader will assume is current
because it is in a table beside current prose is still misleading.

While auditing, note anything else that S1d.2.4's activator facts moved.
`docs/api/rust.md` did its own 434 → 444; the review implies other fact-count
statements exist and did not get audited. That list, if it is short, belongs
with [DO-M1](../p1e.3_medium/s1e.3.8_documentation.md)'s pass rather than
here.

## Notes

The `01_kb.md` shape defect is the one worth not batching: it is a
**teaching** page for the graph encoding, and getting a node-shape vocabulary
wrong there propagates into how a reader reads every other DOT view in the
tree. Fix it against the renderer and say in the commit which authority
settled it.
