# F21 N-15 — *everybody's thoughts are false*, with `absent`, comes out dead

## 1. The question

The sentence *"if everybody's thoughts are false, then false"* written as two rules over `absent` — `someone-sane`
derives `(some-sane)` for a person who does not think `(false)`, and `all-confused` concludes `(false)` where
`(some-sane)` is absent — answers **dead** (R empty) although Bob thinks nothing false. The program is stratified
(`someone-sane` belongs below `all-confused`), so its perfect model holds `(some-sane)` and no `(false)`, and R
should be {∅}. The register already holds one wrong answer of this family, `W8/fifo-gap-root`: two rules that wait on
an `absent`, the one that reads being admitted before the one that derives what it reads. The question is whether
this case is **a twin of `W8/fifo-gap-root`**, read with that row's reading (D2's admission in stratum order, under
which R is {∅}), or **a row of its own**; and, with that row's first call, whether the reading stands. Nothing is
filed here.

Recorded at [`f21_non_monotone_programs_decided.md` § Met again by P1f.22](../../../../followups/f21_non_monotone_programs_decided.md#met-again-by-p1f22-everybodys-thoughts-are-false-at-a-dead-root)
(finding N-15), and the row at [`register.toml`](../../../../wrong_answers/register.toml), case `W8/fifo-gap-root`, with
its reading in [F21 § Handed over by M1f, 2026-11-17: the FIFO gap at root](../../../../followups/f21_non_monotone_programs_decided.md#handed-over-by-m1f-2026-11-17-the-fifo-gap-at-root).

## 2. What is so today

All runs use `build/review-bin/ein-head-17085b27`, today's binary, every P1f.23 stage merged; [`rerun.sh`](rerun.sh) makes every file below. Both ledger statements reproduce: `x2b.ein` prints
`states (k_s) 0`, *lattice monotone (the static check) — R is convex: no hole*, *unsat core ∅, the empty set: the
refutation rests on no fact*, *refuted by all-confused*; `x3b.ein`, the `forall` form, prints `states (k_s) 1`,
*∅ in R — root is alive and owes nothing*.

### 2.1 Side by side

| | [`fifo-gap-root.ein`](programs/fifo-gap-root.ein) (the register's witness) | [`x2b.ein`](programs/x2b.ein) (N-15) |
|---|---|---|
| the two rules waiting on an `absent` | `r2` reads `q` under `(absent (q ?x))`; `r1` derives `q`, waiting on `(absent (z ?x))` | `all-confused` reads `some-sane` under `(absent (some-sane))`; `someone-sane` derives `some-sane`, waiting on `(absent (think ?p (false)))` |
| stratified? | yes: `r1` below `r2`, nothing derives `z` | yes: `someone-sane` below `all-confused`, nothing derives `think` |
| written order | the reader (`r2`) first | the deriver (`someone-sane`) first |
| the agenda's half ([Q-M1f.313](../../../../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)) | both in the second half, so written order | `all-confused` concludes `(false)`: **first half**; `someone-sane` second |
| admitted first (`--events`) | round 1 `admit r2`, round 2 `admit r1` ([table](runs/fifo-gap-root.events.md), rows 8, 11) | round 1 `admit all-confused`, round 2 `retire someone-sane` (Ann) and `admit someone-sane` (Bob) ([table](runs/x2b.events.md), rows 12, 15, 16) |
| root holds (`ein saturate --dump`) | `(p one :rule r2)` **and** `(q one :rule r1)`: `p`'s guard refuted by root's own fixpoint | `(false :rule all-confused)` **and** `(some-sane :rule someone-sane)`: `(false)`'s guard refuted by root's own fixpoint |
| `ein solve` | R = {∅}, the query `(p ?x)` answers `?x = one` ([read-out](runs/fifo-gap-root.solve.txt)) | R empty, *every node is dead — root's own fixpoint derives (false)* ([read-out](runs/x2b.solve.txt)) |
| `ein oracle` | the same ([oracle](runs/fifo-gap-root.oracle.txt)) | the same ([oracle](runs/x2b.oracle.txt)) |
| the static check, `--monotone` | *monotone yes — no site* ([run](runs/fifo-gap-root.monotone.txt)) | *monotone yes — no site* ([run](runs/x2b.monotone.txt)) |
| `--json-summary`: `monotonicity.monotone`, `.order`, `.state`; `root.saturator` rounds / admitted / retired | `true`, `[]`, `null`; 2 / 2 / 0 ([summary](runs/fifo-gap-root.summary.json)) | `true`, `[]`, `null`; 2 / 2 / 1 ([summary](runs/x2b.summary.json)) |
| the answer in stratum order (D2's reading) | {∅}, no `p`, the query has no row | {∅}, no `(false)` |
| moves with the written order of the two rules | **yes**: [`fifo-gap-root-r1-first.ein`](programs/fifo-gap-root-r1-first.ein) has no `p` | **no**: [`x2b-a-first.ein`](programs/x2b-a-first.ein) is dead too, since the first half wins over written order |
| moves with a conclusion nothing reads (a stored negation, which puts its rule in the first half) | **yes**: [`fifo-gap-root-negation.ein`](programs/fifo-gap-root-negation.ein) has no `p` | **yes**: [`x2b-negation.ein`](programs/x2b-negation.ein), `someone-sane` also concluding `(not (insane ?p))`, is **alive, R = {∅}** |
| the KB view of root | [`renders/fifo-gap-root.kb.svg`](renders/fifo-gap-root.kb.svg) | [`renders/x2b.kb.svg`](renders/x2b.kb.svg): the stale `(false)`, dashed, `by all-confused`, beside `(some-sane)`, `by someone-sane`; `(think Ann (false))`'s arrow `3` enters that same octagon, one object held and thought |
| the trace's steps | [`runs/fifo-gap-root.trace.md`](runs/fifo-gap-root.trace.md): r2, then r1 | [`runs/x2b.trace.md`](runs/x2b.trace.md): confused, all-confused, someone-sane |

### 2.2 The one program between them

Replace `(false)` by a fact and the two cases become one shape:

| program | conclusion of `all-confused` | written first | root holds | R |
|---|---|---|---|---|
| [`x2b-fact.ein`](programs/x2b-fact.ein) | `(all-mad)` | `someone-sane` | `(some-sane)`; the query `(all-mad)` has no row | {∅} |
| [`x2b-fact-a-first.ein`](programs/x2b-fact-a-first.ein) | `(all-mad)` | `all-confused` | `(all-mad)` **beside** `(some-sane)`: a stale fact, the query holds | {∅} |

`x2b-fact-a-first.ein` is `fifo-gap-root.ein` with other names: reader written first, both rules in the second half,
the stale fact at an alive root ([its events](runs/x2b-fact-a-first.events.md): round 1 `admit all-confused`,
round 2 `retire` / `admit someone-sane`). `x2b.ein` is that program with the stale conclusion being `(false)`, which
moves `all-confused` into the agenda's first half, so it is admitted first whatever the written order, and a stale
`(false)` kills root instead of sitting beside an alive one.

### 2.3 The `forall` form, and the corpus

[`x3b.ein`](programs/x3b.ein) writes the sentence as one rule with one nested absence; nothing waits on a second
rule's conclusion, and R is {∅} ([read-out](runs/x3b.solve.txt), [oracle](runs/x3b.oracle.txt); its events have one
NAF round and no admission, [table](runs/x3b.events.md)). The corpus holds the `forall` form:
[`09_false_in_a_context.ein`](programs/09_false_in_a_context.ein) (alive) and
[`10_everybody_contradicts.ein`](programs/10_everybody_contradicts.ein) (dead, every person muddled), both `ein test`
ok ([`runs/09_false_in_a_context.test.txt`](runs/09_false_in_a_context.test.txt),
[`runs/10_everybody_contradicts.test.txt`](runs/10_everybody_contradicts.test.txt)).

## 3. The options

**(1) A twin of `W8/fifo-gap-root`.** Same mechanism: at a boundary the engine admits one parked rule per round, in
the agenda's order, and never withdraws what an admitted rule derived; neither order is stratum order. Under the
row's reading (D2's stratum-order admission) both answer the perfect model, and both twins of § 2.1's last rows move
the same way. Filed as a `[[case.twin]]` of the row, `x2b.ein` with `right` = {∅} (D2 stratum order), the
alternatives the row already lists: (A)/(B) in agenda order, today's, R empty; (C) over (A), guards are premises,
also R empty (root holds a stale `(false)`), so (C) does not separate them here. What a twin row would add that the
witness lacks: **the order that bites is Q-M1f.313's fail-first half, not the written order**, so the row's
`open_because` and Q-M1f.313's *Why* (*"on a monotone program the fixpoint does not depend on the order"*, already
listed among the row's third call) would both name it.

**(2) A row of its own**, e.g. `W8/fail-first-root`: the answer is a function of the written program in the sense
that no rule order moves it (§ 2.1, `x2b-a-first.ein`), but not of what a stratified program means (`x2b-negation.ein`
moves it by a conclusion nothing reads); the symptom differs (an empty R, where the witness has a stale fact at an
alive root); and Q-M1f.313's agenda, not FIFO, is what picks the wrong rule. Its `right` is the same reading.

**(3) Not a row**: the program is read as today's engine reads it, and Q-M1f.313's scope note (*the order decides no
answer where no rule's `absent` reads a relation another rule derives at the same boundary*, `AGENTS.md`) is what
covers it. That is the row's first call answered "the reading does not stand".

## 4. The default

**None taken** in the ledger: *"The user's call: whether it is a twin of `W8/fifo-gap-root`, read with that row's
reading (D2's stratum-order admission, under which R is {∅}), or a row of its own; and with that row's first call,
whether the reading stands."* The row itself is not reviewed, and its reading rules nothing (Q-M1f.134).

## 5. Files

Programs:
- [`programs/fifo-gap-root.ein`](programs/fifo-gap-root.ein), [`programs/fifo-gap-root-r1-first.ein`](programs/fifo-gap-root-r1-first.ein), [`programs/fifo-gap-root-negation.ein`](programs/fifo-gap-root-negation.ein) — copies of the register's witness and its two twins (`plans/wrong_answers/witnesses/`).
- [`programs/x2b.ein`](programs/x2b.ein), [`programs/x3b.ein`](programs/x3b.ein) — copies of the ledger's two programs (F21 § Met again by P1f.22), with a header comment.
- [`programs/x2b-a-first.ein`](programs/x2b-a-first.ein) — `x2b` with `all-confused` written before `someone-sane`.
- [`programs/x2b-negation.ein`](programs/x2b-negation.ein) — `x2b` with `someone-sane` also concluding a stored negation nothing reads.
- [`programs/x2b-fact.ein`](programs/x2b-fact.ein), [`programs/x2b-fact-a-first.ein`](programs/x2b-fact-a-first.ein) — `(false)` replaced by `(all-mad)`, in each written order, with a query.
- [`programs/09_false_in_a_context.ein`](programs/09_false_in_a_context.ein), [`programs/10_everybody_contradicts.ein`](programs/10_everybody_contradicts.ein) — copies of `examples/beliefs/`, the `forall` form in the corpus.
- [`programs/kbview-rule-named-like-its-relation.ein`](programs/kbview-rule-named-like-its-relation.ein), [`programs/kbview-rule-named-apart.ein`](programs/kbview-rule-named-apart.ein) — § 7's drawing finding.

Runs, for each of `fifo-gap-root`, `fifo-gap-root-r1-first`, `fifo-gap-root-negation`, `x2b`, `x2b-a-first`,
`x2b-negation`, `x2b-fact`, `x2b-fact-a-first`, `x3b` (`runs/PROGRAM.KIND`):
- `solve.txt` — `ein solve`, with its exit code; `oracle.txt` — `ein oracle`; `monotone.txt` — `ein solve --monotone`;
- `trace.md` — `ein solve --trace` (its drawings are drawn under `renders/PROGRAM.trace-K.svg`, K the drawing's place in the trace);
- `events.jsonl` — `ein solve --events`, and `events.md`, the same as a table, one row an event;
- `summary.json` — `--json-summary` (`monotonicity` and `root.saturator`);
- `saturate-dump.txt` — `ein saturate --dump`, root's facts by origin.

For example: [`runs/x2b.solve.txt`](runs/x2b.solve.txt), [`runs/x2b.oracle.txt`](runs/x2b.oracle.txt),
[`runs/x2b.monotone.txt`](runs/x2b.monotone.txt), [`runs/x2b.trace.md`](runs/x2b.trace.md),
[`runs/x2b.events.jsonl`](runs/x2b.events.jsonl), [`runs/x2b.events.md`](runs/x2b.events.md),
[`runs/x2b.summary.json`](runs/x2b.summary.json), [`runs/x2b.saturate-dump.txt`](runs/x2b.saturate-dump.txt); the same
eight for [`fifo-gap-root`](runs/fifo-gap-root.solve.txt) and each other program.
Also: [`runs/09_false_in_a_context.test.txt`](runs/09_false_in_a_context.test.txt), [`runs/09_false_in_a_context.solve.txt`](runs/09_false_in_a_context.solve.txt), [`runs/10_everybody_contradicts.test.txt`](runs/10_everybody_contradicts.test.txt), [`runs/10_everybody_contradicts.solve.txt`](runs/10_everybody_contradicts.solve.txt);
[`runs/kbview-rule-named-like-its-relation.saturate-dump.txt`](runs/kbview-rule-named-like-its-relation.saturate-dump.txt), [`runs/kbview-rule-named-apart.saturate-dump.txt`](runs/kbview-rule-named-apart.saturate-dump.txt), [`runs/kbview.like-vs-apart.kb.dot.diff`](runs/kbview.like-vs-apart.kb.dot.diff).

Renders: `renders/PROGRAM.kb.dot` and `.svg`, `ein render kb` of root, drawn with `fdp` as the DOT suggests, for each program above
([`x2b`](renders/x2b.kb.svg), [`fifo-gap-root`](renders/fifo-gap-root.kb.svg), [`x2b-fact-a-first`](renders/x2b-fact-a-first.kb.svg),
[`x2b-negation`](renders/x2b-negation.kb.svg), [`x3b`](renders/x3b.kb.svg), …); `renders/PROGRAM.trace-K.dot`
and `.svg`, each step of the trace, each fact its octagon ([`x2b` step 2, all-confused](renders/x2b.trace-2.svg),
[`fifo-gap-root` step 1, r2](renders/fifo-gap-root.trace-1.svg), …);
[`renders/kbview-rule-named-like-its-relation.kb.svg`](renders/kbview-rule-named-like-its-relation.kb.svg),
[`renders/kbview-rule-named-apart.kb.svg`](renders/kbview-rule-named-apart.kb.svg); [`runs/draw-times.tsv`](runs/draw-times.tsv), each drawing's layout and seconds.
[`rerun.sh`](rerun.sh) makes every file; [`tools/events_table.py`](tools/events_table.py) writes the events' tables.

## 6. Re-run

```sh
bash plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/09-F21-N-15/rerun.sh
```

It runs, with `build/review-bin/ein-head-17085b27`, for each of the nine programs `ein solve`, `ein oracle`,
`ein solve --monotone`, one `ein solve` with `--trace`, `--events` and `--json-summary`, `ein saturate --dump` and
`ein render kb`; `ein test -v` and `ein solve` of the two corpus programs; the KB views of the two `kbview-*`
programs and their diff; then it draws every `.dot` with
[`../../03-drawings/tools/draw.sh`](../../03-drawings/tools/draw.sh).

## 7. Found while building

- **Re-taken with `ein-head-17085b27`** against the first look's `ein-head-b5fb8d70`: no answer moved, in any
  read-out, oracle, static check, summary or event (only the `run` event's argv, which lists every option of the one
  run that now writes the trace, the events and the summary together). The drawings moved, to the canonical form.
- **The KB view's gap the first look found is gone.** At the first look `ein render kb` of `x2b.ein`'s root drew no
  `(confused Ann)`, though root holds it: the rule is named `confused` like the relation it concludes, so the view
  took the fact for an activator and left it out. Today the KB view draws every fact the knowledge base holds
  (Q-M1f.432), so both [`kbview-rule-named-like-its-relation.ein`](programs/kbview-rule-named-like-its-relation.ein)
  and [`kbview-rule-named-apart.ein`](programs/kbview-rule-named-apart.ein) (the rule renamed `c`) draw `(confused Ann)`,
  an octagon whose arrow `1` is at the box `confused`; the two views differ only in the note beside it, `by confused`
  against `by c` ([`runs/kbview.like-vs-apart.kb.dot.diff`](runs/kbview.like-vs-apart.kb.dot.diff)). In
  `x2b.ein`'s view it is the dashed octagon `by confused` ([`renders/x2b.kb.svg`](renders/x2b.kb.svg)).

  ![ein render kb of x2b.ein's root: (confused Ann) drawn, the stale (false) held beside (some-sane)](renders/x2b.kb.svg)
- `x2b`'s twin by written order does **not** move (§ 2.1), unlike the witness's: the ledger's *"Both rules wait on an
  `absent` at one boundary and the agenda admits the one that concludes `(false)` first"* is exactly right, and it is
  why the written-order twin cannot be the test of twinship here; the negation twin is.

## 8. Your call

- **(1) twin** of `W8/fifo-gap-root`, `x2b.ein` a `[[case.twin]]` with D2's stratum-order reading as `right` ({∅}).
- **(2) a row of its own**: the same reading, filed apart because Q-M1f.313's first half, not FIFO, picks the rule,
  and the symptom is an empty R.
- **(3) not a row**: the row's reading does not stand, and Q-M1f.313's scope note covers it.
- **Default**: none taken.
