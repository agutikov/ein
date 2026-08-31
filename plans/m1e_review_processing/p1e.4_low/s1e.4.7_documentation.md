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
  the rows, fix the two words. ~~This is the same page whose §3.2 may have been
  deleted by [T1e.2.2.4](../p1e.2_high/s1e.2.2_code_doc_consistency.md) — so do
  this **after** that lands, or the count moves twice.~~ **Unblocked
  2026-08-29**: §3.2 was rewritten rather than deleted
  ([S1e.1.4](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)), so
  the page still states thirteen behaviours and this count is free to move on
  its own.
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

---

## ✅ Done 2026-09-01 — one was already fixed, one was worse, and the mechanism was wrong

### `DO-L1` — the four

| | disposition | |
|---|---|---|
| (a) `defined_behaviour.md` § 4.2 | **fixed**, and **larger** | *"Nine more"* over a ten-row table — and the *next* sentence was false too |
| (b) `06_reserved_names.md` | **already fixed** | T1e.2.2.3 (2026-08-30) rewrote the arithmetic *and* the adjacent `CD-H2` `none` → `(false)`, in one visit, exactly as both stages' tasks said it would |
| (c) `01_kb.md` shapes | **fixed**, and it is four blocks on three pages | settled against the renderer, which settles it **negatively** |
| (d) `03_examples.md` | **fixed** | git recovered the intended claim |

**(a) is the one worth reading.** The count was wrong — *nine* over ten rows —
and so was the sentence directly under the table: *"and all ten by `ein-ir`'s
`the_verdict_atom_refuses_every_shape_it_cannot_resolve`"*. That test covered
**nine**. The missing row is an `open` whose argument is neither a variable nor
a name; the refusal is real and live
(`:assert (open (r ?a ?b))` → *"`open`'s argument names a relation — a rule
parameter or a relation name"*, exit 1) and had no case anywhere in the repo.
So the page understated its table by one and simultaneously claimed the
shortfall was pinned — and the citation is what stopped anybody counting, which
is the same failure mode [S1e.4.3](s1e.4.3_state_model.md) found in
`standard_of_proof.md` and [S1e.4.5](s1e.4.5_tests.md) found in
`stdlib/README.md`. Three in one phase. The case is added; the test's own doc
comment said *"the four refusals"* over a twelve-entry loop and says what it
holds now.

**(c) is settled by the renderer saying no.** The table calls a relation node a
round-rect; **four** DOT blocks on three current pages drew it as a
**hexagon**, which is the shape the same table gives `Rule` — so a reader
learning the vocabulary here learned `Relation` as `Rule`. The renderer picks
neither: `ein-render`'s whole shape set is `box` / `oval` / `rectangle` /
`octagon` / `doublecircle` / `diamond`, and
[`04_dot_rendering.md` § Node-shape legend](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
— which *is* normative and *is* the renderer's — says a **relation schema is a
dashed labelled edge**. The two Levi-shaped views that would draw a relation
node at all are library-only ([Q-M1e.20](../open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable)).
So the examples move to the table's round-rect, and the page gains the sentence
that stops the next reader mistaking a teaching diagram for output: **these
diagrams are the encoding, not `ein render`'s**.

### `DO-L2` — the mechanism the review names is refuted, and the two halves are unequal

**S1d.2.4's activator facts moved neither site.** The enterings measured today
(101, 3 557) are identical to `features.md`'s 2026-08-23 re-take, which
*predates* S1d.2.4 by two days; the ten activator facts touch a model's fact
count, which is why `docs/api/rust.md`'s 434 → 444 was the one page that had to
move and no other did. Recorded here so nobody re-audits for it.

**Site 1 is smaller than reported.** `examples/README.md`'s 46.9 / 31.1 ms are
3–9 % stale — re-taken 2026-09-01 through `utils/bench_env.sh` on the same
pinned P-core, median of five: **44.1** and **28.2 ms**, with verdict, `k` and
`exhausted` unchanged. The stage's escape clause (*"if it is now materially
different, the date alone is not enough"*) does not fire, so the banked digits
**keep their date** and gain the re-take beside them, which is
[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)'s
warrant for a measurement. The same table's *12 rules defined in the file* is a
**size** and was wrong (14), so it becomes the command.

**Site 2 is worse than reported.** `docs/guide/03_rule_families.md`'s *≥23×,
101 → 3336+, and it does not finish* is not a stale measurement, it is a
**misattributed** one: those are 2026-08-17 `ein.py`/PyPy figures whose ceiling
was a **90 s budget**, on an engine that left the tree — and `features.md`, the
page the sentence names as the measurer, had already retired both digits. Today
it is 101 → **3 557** and **54×**, and *"does not finish"* is false: it finishes
in 1.5 s.

**And the page that owns the number was wrong about it too.**
`features.md`'s own † footnote said *"ein.rs runs the same search to the end at
3 557"*. It does not — that run reports `exhausted = false` and stops at the
default `--max-set-size 5`; lifting the cap gives **5 405** at `-m 6` and
**6 989** at `-m 7`, still unexhausted. Both numbers in that row are floors, for
different reasons — a clock and a depth cap — and the footnote now says so,
which is a stronger statement of the row's actual content: with the lever off,
nothing anybody has run has finished this search.

**Gate:** `cargo test --workspace` — **818 tests, 0 failures**. No golden moved
(no test reads any of these pages; the one gate step that sees them is the link
check, and it is green over 280 pages).
