# S1e.2.2 — Code ↔ doc: the canonical tree

**Phase:** [P1e.2](README.md) (High)
**Estimate:** 6 days
**Depends on:** [Q3](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)
for T4 — **answered 2026-08-29, and it took T4 with it**;
[Q-M1e.3](../open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
for T1, which this stage takes first and then applies.
**Findings:** [`CD-H1`](../review/code-doc-consistency/high.md),
[`CD-H2`](../review/code-doc-consistency/high.md),
[`CD-H3`](../review/code-doc-consistency/high.md).
**Blocks:** [M2 S2.1.1](../../m2_nl_to_ir/p2.1_kernel_as_instrumentation/s2.1.1_census.md),
whose census re-reads `defined_behaviour.md` and writes into it.

## Context

`CLAUDE.md` makes `docs/kernel` load-bearing in a specific way:

> **This tree is now the only statement of intent that is not also the
> implementation**, so it is load-bearing: a claim here is checked by
> `cargo test --workspace` and by nothing else.

Under that rule the three findings here are not documentation debt. They are:
a **false specification** across at least six pages (`CD-H1`), a tree that
gives **two different answers** to *how many verdict words are there*
(`CD-H2`), and a normative page whose one self-declared latent bug **does not
reproduce** (`CD-H3`).

The scale, from the review, page by page:

- **`algorithm_layer_n.md`** — a P1.5b *design* document presented as live
  specification. Three public solve entries (`monotonic_solve` / `gaps_solve`
  / `contradictions_solve`) where
  [`solve.rs:594`](../../../ein.rs/crates/ein-infer/src/solve.rs) has one
  `pub fn solve`; unconditional-fact flat root-merge, **retired at P1.21 R2 as
  NAF-unsound**; state-hash dedup as identity, replaced by `state_key`
  representation-identity at P1.21 R1; multi-parent integrate, dropped. Its
  links were mechanically re-aimed at the Rust source, so it *reads* current.
  `inference/README.md:1040-1064` cites it as "Algorithm spec" with a
  nonexistent anchor, and `architecture_and_algorithms.md:41-48` records the
  same design as a removed **soundness bug** — the tree asserts a design and
  its refutation at once.
- **`lattice_dump.md`** — a `kb_index/` artifact tree the Rust dumper never
  writes (`dump/lattice.rs:24-30, 284-285` builds it empty **by
  construction**), a "Programmatically" section importing
  `ein.inference.monotonic` from the engine deleted at S1a.10.5, a debugging
  workflow that depends on the artifacts, and `LatticeDumper` attributed to
  the wrong file. `lattice_diagrams.md:284-291` promises the same artifacts
  from a CLI invocation.
- **`02_store.md`** — singular last-wins `query` against `program.rs`'s
  `queries: Vec` and 01_grammar's multi-query semantics (an M1c change never
  propagated); the deleted `add_type`/`add_instance` mutation API, though §6
  of the same file says that view is gone; the `_kb` back-pointer fork caveat
  as current, though two sibling pages call it historical; `kb.rs` listed
  twice under "Sources of truth".
- **`02_patterns.md` + `glossary.md`** — a predicate registry
  (`unique-remaining`, `no-remaining-option`, `forbidden-by-exclusion`) that
  exists nowhere; `predicates.rs` implements exactly `eq`/`neq`. A reader is
  credited with aggregate machinery the engine does not have.
- **`docs/kernel/README.md`** — the tree's own entry point, listing the
  P1.7c-removed six-block-forms surface language and calling the inference
  engine a *placeholder, P1.3* while its own § What's-M1 says the engine
  shipped.
- **`04_dot_rendering.md`** — a runnable-looking Python section against the
  deleted engine, an `EIN_RENDER_LEVI` env var that greps empty, and a
  `from_dot` promised "when implemented in P1.2" — P1.2 closed in May 2026.
- **`inference/README.md`** — un-bannered strata contradicting bannered ones,
  a Budget section documenting a Python exception replaced by
  `Answer::Aborted`, and a header banner that under-counts the removed
  machinery it warns about.
- **`zebra_walkthrough.md:16-21`** — routes embedders to the historical
  Python API pages that `README.md:87` of the same tree calls history.

And `CD-H2`: `01_grammar.md` says of the obligation tally *"nothing reads the
tally … legal and inert"*; `06_reserved_names.md` says *"no verdict word …
S1d.2.6 is where the word is decided"* and gives `:expect`'s third form as
`none` instead of `(false)`; `inference/README.md:83` and
`architecture_and_algorithms.md`'s §3 list **three** verdict words while §2 of
the same file lists four; `implementation.md` has no `Open` and no tree.
Meanwhile `defined_behaviour.md`, `events.md` and
`architecture_and_algorithms.md` §2 document the shipped verdict correctly,
and the code agrees with them.

## Acceptance

- **Every page under `docs/kernel/` is in exactly one declared state** —
  *current*, *superseded with a banner*, or *moved to `docs/history/`* — with
  the state visible from the page itself, not only from this plan. 37 pages;
  the triage table is the deliverable and it covers all of them, not only the
  nine the review named.
- **No page describes machinery that does not exist without saying so.** The
  test a reader can apply: every code identifier a *current* page names
  resolves in `ein.rs/crates/`, and every env var and CLI invocation it shows
  runs.
- **One answer per question, tree-wide**: how many verdict words; who reads
  the obligation tally; what `:expect`'s third form is; whether a tree
  traversal exists in the module map.
- ~~**§3.2 is amended or deleted** per
  [Q3](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md), and
  `Q-M1a.8` is closed in the M1a ledger with a date and the probe's name;
  README's Known gaps moves in the same commit.~~ ✅ **Done 2026-08-29 in
  S1e.1.4** — see T1e.2.2.4.
- A **doc-pass checklist** exists for the next milestone that has to do this,
  naming the pages S1a.10.6 missed and why they were missable.

## Tasks

### Task T1e.2.2.1 — Take the rule, then triage all 37 pages

The rule first, because applying it page by page without it produces 37
independent judgment calls. That rule is
[Q-M1e.3](../open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted),
and it has to answer the awkward case: a page nothing reads as a
specification, that is not evidence, but that **is** the reason another page
records a removed soundness bug. Half a reason. The three candidate
dispositions are a new `docs/history/` entry, deletion (git holds it; the
surviving reason is already stated where the refutation is), and a banner as
strong as `parity_baselines.md`'s.

Then the triage, as a table in this stage's notes and as a section in
[`docs/kernel/README.md`](../../../docs/kernel/README.md) — because the tree's
entry point is where a reader learns which pages to trust, and it is
currently one of the pages that misleads them:

| page | state | why | action |
|---|---|---|---|

37 rows. The nine the review named have their answers above; the other
twenty-eight need at least a glance, since the review's method was a reading
pass and it says so. The cheap filter for a page nobody has checked: grep it
for identifiers and env vars, and resolve them.

### Task T1e.2.2.2 — Execute the triage

Three buckets, three kinds of work, and they are deliberately not one commit:

**Banner or move** — `algorithm_layer_n.md`, the Python sections of
`lattice_dump.md` and `04_dot_rendering.md`, `zebra_walkthrough.md`'s
embedder pointer. A banner in this tree already has a shape
(`parity_baselines.md`, `docs/api/`'s 🏛): what the page describes, when it
stopped being true, and where the current statement is. **Do not rewrite
these to match today's engine** — that is the failure mode the whole
convention exists to prevent.

**Content fix** — `02_store.md` (multi-`query`, the deleted mutation API, the
`_kb` caveat, the duplicated source-of-truth row), `02_patterns.md` +
`glossary.md` (the three phantom predicates; also the false claim that an
`(instnce ?a ?T)` typo is caught at parse time — any generic head parses),
`docs/kernel/README.md` (the removed six-block-forms surface, the *placeholder,
P1.3* self-description), `lattice_dump.md`'s non-Python half (`kb_index` is
empty by construction — say so, and fix the `LatticeDumper` file attribution),
`inference/README.md` (the un-bannered strata, the Budget section's Python
exception, the closing *when P1.3 work begins*).

**Reachability** — `lattice_diagrams.md:284-291` tells a reader a CLI
invocation produces `proof_summary.json` + `kb_index/`. It does not, and the
per-hypothesis lattice dump is **unreachable by any documented means**. That
is not only a doc fix: decide whether the artifact should be reachable (a
flag, or a documented library call) or whether the documentation of it should
go. Route the decision through the render crate's owner rather than deciding
it in a doc pass.

### Task T1e.2.2.3 — The M1d pass over the five stale pages

`CD-H2` is smaller and entirely mechanical, and the correct text already
exists to copy from — the census documents under
[`docs/history/m1d_satisfiability/`](../../../docs/history/m1d_satisfiability/README.md)
state all of it correctly. Five pages:

| page | what is stale |
|---|---|
| `01_grammar.md:412-414` | *"nothing reads the tally … legal and inert"* — the verdict reads it since S1d.2.6 |
| `06_reserved_names.md:153-157, 228-233` | *"no verdict word"*; `:expect`'s third form given as `none`, which is not the keyword |
| `inference/README.md:83` | three verdict words |
| `architecture_and_algorithms.md:200` | three verdict words, contradicting §2 of the same file |
| `implementation.md:55, 97` | module map with no `Open` and no tree traversal |

Do it as **one commit** so the tree never sits in a half-updated state, and
check the three pages that are already right (`defined_behaviour.md:360-370`,
`events.md:105-118`, `architecture_and_algorithms.md:115-135`) so the edit
converges on their wording rather than inventing a fourth phrasing.

While in `06_reserved_names.md`, note that
[DO-L1](../p1e.4_low/s1e.4.7_documentation.md) has a second defect on the
same lines (the keyword arithmetic does not reconstruct the 7-keyword
allow-list). One visit, both fixes.

### Task T1e.2.2.4 — `CD-H3`: settle §3.2 and close `Q-M1a.8` ✅

**Done 2026-08-29, in
[S1e.1.4](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)
itself** — the probes ran there and the paperwork was one commit's worth, so
splitting it across two stages would have left the ledger disagreeing with the
page for the length of a phase. §3.2 is rewritten to the shape that
reproduces, `Q-M1a.8` is closed as stated, the live half is
[Q-M1e.16](../open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one),
and the README's Known gaps entry, the capability table's cell and **five
source comments** moved with it. §3.2 was **not** deleted, so
`defined_behaviour.md`'s *thirteen* and the sentence in `CLAUDE.md` that
quotes it are unchanged — the paragraph below is left as it was written, and
it is the half this stage no longer has to do.

The task as written:

The probes are
[S1e.1.4](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)'s; the
paperwork is this task, and it is only paperwork if the probes ran. Apply the
outcome table from that stage: rewrite §3.2 to the real trigger, or delete the
item, and close `Q-M1a.8` in
[the M1a ledger](../../../docs/history/m1a_rust/open_questions.md) with the
date and the probe's name. `README.md`'s Known gaps entry cites it and moves
in the same commit.

If §3.2 is deleted, `defined_behaviour.md`'s self-description — *the thirteen
diagnostics, orderings and error strings* — changes, and so does the sentence
in `CLAUDE.md` that quotes it. Two copies again; edit both.

### Task T1e.2.2.5 — A checklist the next doc pass can run

S1a.10.6 was a doc pass and it **missed every page in `CD-H1`**. That is the
useful finding inside the finding: a doc pass driven by *what the milestone
changed* cannot catch a page describing machinery removed two milestones ago,
because nothing in the milestone touched it.

So the deliverable is a checklist that is not milestone-scoped — a short
`utils/`-adjacent procedure, or a section of
[`docs/kernel/README.md`](../../../docs/kernel/README.md), listing the checks
that would have caught these: identifiers resolve, env vars grep non-empty,
CLI invocations run, intra-tree anchors exist, and every page states which of
the three states it is in. Whether any of it becomes mechanical is
[DO-M2](../p1e.3_medium/s1e.3.8_documentation.md)'s question (a
markdown-link checker is the obvious candidate, and `rustdoc` already does
this class for crate docs); what this task owes is the list.

## Notes

Six days is the largest stage estimate in the milestone and most of it is
T1e.2.2.2. The way it goes wrong is by becoming a rewrite: `docs/kernel` is
89 pages' worth of careful prose and the temptation to improve it while
passing through is real. The stage's scope is exactly the three states and the
false claims — not clarity, not structure, not the glossary's coverage.

One page is worth a second look beyond its listed defects. `02_patterns.md`
describes a predicate registry that never existed, which is a different
failure from the rest of `CD-H1` — those pages describe machinery that was
**removed**, this one describes machinery that was **planned**. If there are
other never-built descriptions in the tree, they will be found by the same
identifier-resolution filter, and they belong in the triage table with their
own reason column.

---

# Record

## T1e.2.2.1 — the rule, and all 40 pages

**The tree is 40 pages, not 37**, and both numbers are right: the stage was
written against `7731848` (2026-08-27, the review's base) and three pages
landed the next day, inside M1e itself —
[`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md) (T1e.1.1.1),
[`configuration.md`](../../../docs/kernel/configuration.md) (S1e.5.1) and
[`inference/solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)
(S1e.3.2's predecessor). All three are *current* and all three are the kind of
page the checklist in T1e.2.2.5 exists to keep that way.

### The rule ([Q-M1e.3](../open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted), answered 2026-08-30)

The awkward case had an answer already written down, one directory over.
[`docs/history/README.md`](../../../docs/history/README.md): *"**M1** (core
graph reasoning, shipped 2026-06-17) predates this directory: what survived its
plan tree went to `docs/kernel/inference/` and `plans/followups/` at P1.22, and
the rest is in git history."* Candidate (a) — `docs/history/m1_core/` — is not a
directory nobody has created; it is one the tree **declined** to create, and
`algorithm_layer_n.md` is already where P1.22 put it. What it lacked was the
banner, not the destination.

Generalised, and applied below:

1. Does every claim on the page hold of the engine that ships? → **current**.
2. Otherwise, is the page still *read* — cited by another page, or the record of
   a measurement nothing can re-take? If **no**, and nothing links to it →
   **delete**. If **yes** → it is superseded, and:
3. **Does a `docs/history/` entry for its milestone already exist?** If it does
   → **move** into that record. If it does not → **banner in place**. *A page is
   moved into a milestone record; it is never made into one.*

Rule 3 is what makes the disposition mechanical. `docs/history/` is indexed by
milestone and its three entries are milestone records; a `m1_core/` holding one
page would assert a record nobody wrote. And step 2 disposes of "half a reason"
without weighing halves: `algorithm_layer_n.md` has **five** referrers, one of
them
[`docs/history/m1a_rust/design/07_search_layer.md:301`](../../../docs/history/m1a_rust/design/07_search_layer.md),
which cites it as *"the per-step contract"* — a shipped milestone record naming
it as the specification the port had to reproduce. Deleting a page a history
record cites as its specification falsifies the record, so deletion was never
on the table.

**The rule's output on this tree is three banners and zero moves**, and the zero
is the rule working rather than dodging: all three superseded pages are M1
P1.5b's, and M1 is the one shipped milestone with no `docs/history/` entry — by
the decision `docs/history/README.md` records.

### How the twenty-eight unnamed pages were checked

The review's method was a reading pass and it says so, so the other pages got
the cheap filter the task names, mechanised — two scripts, both throwaway:

- **identifier resolution** — every backticked token on every page classified
  (`EIN_*` · `*.rs` · `fn()` · `Type` · `snake_case` · `a::b`) and resolved
  against `ein.rs/crates/**`. 39 pages produced a candidate list; most
  candidates are prose (`Human`, `rel_can`, DOT node ids), and the residue is
  what the table below cites.
- **links and anchors** — every relative markdown link in the tree, file and
  fragment, GitHub-slugified. **2 broken anchors** in 40 pages, both inside
  pages this stage was already opening.

Plus the empirical half, because three claims were about what the parser
*refuses* and only a run can say: `(instnce ?a ?T)` **loads and solves**
(exit 0), `(neq ?a)` and `(not ?a ?b)` are parse errors, `(eq ?a)` and
`(absent P Q)` are **compile** errors since S1e.2.1, and `(and X)` is accepted
at arity 1.

### The triage — 40 rows

Legend: **current** = every claim holds, fix any that do not · **banner** =
superseded, kept in place with a banner saying what it described and where the
current statement is · **move** = into an existing `docs/history/` record
(nothing qualified).

| page | state | why | action |
|---|---|---|---|
| **root** | | | |
| [`README.md`](../../../docs/kernel/README.md) | current | the tree's entry point, and one of the pages that misleads: *(placeholder, P1.3)* / *Stub before P1.3* against its own § What's-M1, plus the P1.7c-removed six-block-forms surface — `CD-H1` | fix; and carry the triage section |
| [`architecture.md`](../../../docs/kernel/architecture.md) | current | sweep clean; `unconditional_facts` appears only inside the note recording its retirement. One token does not resolve: `StateKey` in the SMT-analogy paragraph (the identity is `canon::state_key`) | one-word fix |
| [`configuration.md`](../../../docs/kernel/configuration.md) | current | M1e S1e.5.1, pinned by `config_reference.rs`; it is already where `EIN_RENDER_LEVI` is recorded as never implemented | — |
| [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) | current | normative; §3.2 rewritten to the shape that reproduces and `Q-M1a.8` closed, 2026-08-29 in S1e.1.4 | — (`DO-L1`'s *"Nine more"* / *"all ten"* is [S1e.4.7](../p1e.4_low/s1e.4.7_documentation.md)'s) |
| [`glossary.md`](../../../docs/kernel/glossary.md) | current | sweep clean. **`CD-H1`'s `glossary.md:194-198` does not reproduce**: `git log -S"unique-remaining" -- docs/kernel/glossary.md` is empty — the page has never named a predicate registry. Its T3 entry is the graph model's rule taxonomy, which is `02_rules.md`'s subject | — |
| [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md) | current | M1e T1e.1.1.1, 2026-08-28 | — |
| **inference/** | | | |
| [`README.md`](../../../docs/kernel/inference/README.md) | current | four sections are bannered and correct; four are not — the verdict row's three words, *Two engines, two termination criteria*, the Budget section's Python exception, and the closing *when P1.3 work begins*. The header banner names two removed-machinery sections where there are four | fix |
| [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md) | current | every claim pinned by `naf_semantics.rs`; the page says so and it is true | — |
| [`algorithm_layer_n.md`](../../../docs/kernel/inference/algorithm_layer_n.md) | **banner** | P1.5b design presented as live spec: `monotonic_solve` / `gaps_solve` / `contradictions_solve` against one `pub fn solve`; flat root-merge (retired P1.21 R2 as NAF-unsound); state-hash dedup as identity; multi-parent integrate. Read by five referrers, one a history record | banner |
| [`architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md) | current | §2 states the four-word verdict correctly and §3's `Verdict` row lists three — the tree's own two answers, in one file. `gaps_solve` / `_closure_step()` / `premises_raw` appear only in historical or ein.py-module contexts | fix the row |
| [`domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md) | current | its *Historical numbers* banner names the removed sibling entries and the removed harness, and scopes exactly what is still true. The model | — |
| [`events.md`](../../../docs/kernel/inference/events.md) | current | states `Open` and the `k`/`solution_nodes` split correctly | — |
| [`features.md`](../../../docs/kernel/inference/features.md) | current | frozen-constant banners are complete. **Not in the review**: twice routes to `docs/api/inference.md` for *what each knob does*, which `README.md:89` calls history; the live definitional table is `configuration.md` | fix the pointer ×2 |
| [`implementation.md`](../../../docs/kernel/inference/implementation.md) | current | the module map's verdict reads `Solution / Ambiguity / Contradiction` in two places and the tree traversal is in no row, though it is `solve.rs`'s | fix |
| [`lattice_diagrams.md`](../../../docs/kernel/inference/lattice_diagrams.md) | **banner** | (9)'s data-model companion and in the same state: multi-parent integrate, `try_branch`, `BranchResult`, *the future* `try_commitment_set`, state-hash dedup as identity — plus a CLI line that produces neither artifact it names | banner; and the CLI line separately (it was never true, banner or not) |
| [`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md) | current | the dump format is live and golden-pinned (`golden_dump.rs` + `dump_parity.rs`). Three defects: `kb_index/` is empty **by construction** and the crate's own module doc says so, the *Programmatically* recipe imports the deleted Python engine, and `LatticeDumper` is attributed to `state.rs` | fix + banner the Python block |
| [`parity_baselines.md`](../../../docs/kernel/inference/parity_baselines.md) | **banner** | already, since S1a.10.6 — the shape the other two copy | — (its `§3d.vii` citation goes with (9)) |
| [`reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md) | current | **not in the review, found by the identifier sweep**: a whole section documents a `Mode` enum in `verdict.rs` with `SOLVE` / `GAPS` / `CONTRADICTIONS` members and a three-row table. There is no such enum anywhere in the crates, and `is_solved` with it | fix |
| [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) | current | M1e, normative, and its §6 states its own approximation before the definitions | — |
| [`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md) | current | routes embedders to `docs/api/ein.md` *"whose contract lands in P1a.9"* — history, per `README.md:89` | fix the pointer |
| **ir/01-ein-graph/** | | | |
| [`README.md`](../../../docs/kernel/ir/01-ein-graph/README.md) | current | — | — |
| [`01_kb.md`](../../../docs/kernel/ir/01-ein-graph/01_kb.md) | current | *"the Python dataclasses that hold it"* in the header banner and *"(P1.3 stub)"* in See-also | fix (2 lines); `DO-L1`'s hexagon row is [S1e.4.7](../p1e.4_low/s1e.4.7_documentation.md)'s |
| [`02_rules.md`](../../../docs/kernel/ir/01-ein-graph/02_rules.md) | current | **not in the review**: §2.3 says the matcher consults a *structural predicate registry*'s **Python implementation**, and §5 lists `unique-remaining` / `no-remaining-option` among the predicates `:where` allows. All eight T3 names in the file resolve nowhere in the repo | fix |
| [`03_ein_model.md`](../../../docs/kernel/ir/01-ein-graph/03_ein_model.md) | current | *"`store.is_symmetric` / `symmetric_relations` survive only as unprivileged property queries"* — neither survives | fix |
| [`04_jack_drinks_coffee.md`](../../../docs/kernel/ir/01-ein-graph/04_jack_drinks_coffee.md) | current | the sweep's unresolved tokens are DOT node ids in the worked example | — |
| [`05_four_level_kb.md`](../../../docs/kernel/ir/01-ein-graph/05_four_level_kb.md) | current | — | — |
| **ir/02-data-model/** | | | |
| [`README.md`](../../../docs/kernel/ir/02-data-model/README.md) | current | — | — |
| [`01_entities.md`](../../../docs/kernel/ir/02-data-model/01_entities.md) | current | **not in the review**: the page opens *"Frozen Python dataclasses … attached to the owning `KnowledgeBase` via a `_kb` back-pointer"* and its own §5 is titled *"the back-pointer, and why it is gone"*. One of the two broken anchors (`#3-provenance`) is here | fix |
| [`02_store.md`](../../../docs/kernel/ir/02-data-model/02_store.md) | current | `CD-H1`'s four — singular last-wins `query`, the deleted `add_type`/`add_instance`, the `_kb` caveat as current against §6 and two sibling pages, `kb.rs` twice under Sources of truth — plus the closing *P1.3 stub* | fix |
| [`03_implementation.md`](../../../docs/kernel/ir/02-data-model/03_implementation.md) | current | — | — |
| **ir/03-ein-lang/** | | | |
| [`README.md`](../../../docs/kernel/ir/03-ein-lang/README.md) | current | — | — |
| [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) | current | honest about its own five unexercised productions. Names *"the named structural-predicate registry"* once, in the list of what §3 does not check — the one phrase that outlives the registry | fix that phrase |
| [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) | current | *"Since S1d.2.3 nothing reads the tally … legal and inert"* — `CD-H2`. Its § Query's keyword arithmetic is the **correct** one and is what `06_reserved_names.md`'s converges on. The second broken anchor is here | fix |
| [`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md) | current | the different failure: machinery **planned**, not removed. A *Predicate registry (initial)* presented as the shipped M1 starter set; `instance` as a kernel meta-primitive; and a parse-time claim the probe refutes | fix |
| [`03_examples.md`](../../../docs/kernel/ir/03-ein-lang/03_examples.md) | current | — | — (`DO-L1`'s garbled sentence is [S1e.4.7](../p1e.4_low/s1e.4.7_documentation.md)'s) |
| [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md) | current | a runnable-looking Python section against the deleted engine, `EIN_RENDER_LEVI` (which `configuration.md` already records as never implemented), and `from_dot` *"when implemented in P1.2"* — P1.2 closed 2026-05 | banner the Python block + fix |
| [`05_inspirations.md`](../../../docs/kernel/ir/03-ein-lang/05_inspirations.md) | current | — | — |
| [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) | current | *"no verdict word … S1d.2.6 is where the word is decided"*; `:expect`'s third form as `none`; and `DO-L1`'s keyword arithmetic on the same page | fix (three) |
| [`07_stdlib_api.md`](../../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md) | current | `b_other` is a rule parameter in an example, not an identifier | — |
| [`08_self_describing.md`](../../../docs/kernel/ir/03-ein-lang/08_self_describing.md) | current | carries a *part operational, part design* banner and is scrupulous about which half is which | — |

**Totals: 37 current · 3 banner · 0 move.** Nine pages needed no edit at all;
four of the ones that did are **not** in `CD-H1`'s list, and every one of the
four was found by the identifier sweep rather than by reading.
