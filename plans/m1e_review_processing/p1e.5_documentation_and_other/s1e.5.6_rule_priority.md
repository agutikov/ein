# S1e.5.6 — Remove `:priority`; derive the order from the rules

**Phase:** [P1e.5](README.md) (Documentation ein does not have)
**Estimate:** 6 days — 3 to measure and decide, 3 to remove.
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md)
[S1e.1.3](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) — **hard.**
Q2 asks whether `MAX_ALT_JUSTIFICATIONS = 32` ever changes which unsat core is
reported; § The one place it still decides an answer shows it doing so, and a
removal taken before that question is answered is a semantics change dressed as
a cleanup.

> **Answered 2026-08-29 — yes, and it splits this stage's consumer 6 in two.**
> The cap *can* decide the core, and S1e.1.3 built the pair that shows it
> (`examples/ein-bugs/alt-cap-core{,-reordered}.ein`, 3 facts against 2, one
> `:priority` apart). **But it is not what moved `branching/07` below.** That
> entry's longest alternatives list is **1** and it refuses nothing at the cap —
> measured over all 202 entries, where exactly one, `zebra2-bad`, reaches 32 at
> all. So the corpus has *two* order-dependencies of the core and this stage's
> § 2 is the other one: which derivations get **recorded at all**, not which of
> the recorded ones survive the cap. A removal has to hold both, and only the
> second has a fixture ([Q-M1e.15](../open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported)).
**Blocks:** nothing. It writes the first content of
[S1e.5.20](s1e.5.20_docs_refactor.md)'s
`docs/ein/reasoning/rule-evolution/analysis-of-rules.md`, which is otherwise an
empty page.
**Source:** the user's note of 2026-08-28 — *"remove `:priority` completely /
analyze rules structure instead."*

---

## Context

`:priority` is a per-rule integer that orders the saturation agenda. **353
occurrences in 82 files**, twelve distinct values, measured at `9ba2349`:

| value | 90 | 100 | 110 | 120 | 200 | 220 | 240 | 250 | 300 | 400 | 500 | 900 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **n** | 7 | 85 | 21 | 3 | 85 | 6 | 46 | 47 | 17 | 13 | 18 | 3 |

**350 of 379 rules carry one (92 %); 0 of 24 `hrule`s do; a relation cannot.**
The stdlib is 79 of the 353. The values are not arbitrary — `ein saturate
--dump` sorts them into four named bands
([`saturate.rs:445`](../../../ein.rs/crates/ein-cli/src/saturate.rs)
`band_label`, *"Q41's priority bands"*):

| band | range | meaning |
|---|---|---|
| `propagate` | < 200 | cheap forward propagation |
| `derive` | < 300 | ordinary derivation |
| `eliminate` | < 900 | domain / range elimination |
| `hypothesis` | ≥ 900 | the guessing rung |
| `unbanded` | absent | `DEFAULT_PRIORITY = 1000` |

### It has been advisory for a milestone, and the repo says so

[`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) § Premise
forms, since S1.21.8:

> **`:priority` no longer decides what is derivable.** On a stratified rule set
> the result is priority-independent; priority still orders firings (hence the
> trace), but the priority-band discipline zebra2 needed for soundness — every
> producer of a watched relation at a strictly lower number than every watcher
> — is now **advisory**. … a static stratification check remains future work.

There is a test for it —
[`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs)
`priority_does_not_decide_what_is_derivable` runs the same gate/derive program
at `(100, 200)` and `(200, 100)` and asserts the same fixpoint both ways.

So the *language* already treats the number as a hint. What has never been
established is whether the hint is worth anything, and that is measurable.

## The control experiment, run while writing this plan

Every `:priority` stripped — from **the whole stdlib** (a mirrored
`$EIN_STDLIB` with a regenerated manifest) **and from every `.ein`** under
`examples/` and `tests/` — then `ein solve --json-summary` on both arms and a
field-by-field diff of the 137-field summary, model fact sets included.

| | n | |
|---|---:|---|
| **identical in every field** | **137** | verdict, `k`, `exhausted`, every counter, and the full model fact set |
| differing | **2** | below |
| not compared | 51 | 47 `broken/` fixtures, which **exit 1 in both arms**; 4 timed out at 25 s |

`examples/zebra2.ein` is representative: **444 model facts, identical set;
101 enterings, 34 alive, 67 dead, 2 layers, 67 no-goods — identical.** The
whole band discipline of the stdlib and the puzzle, removed, and not one
counter moved.

The two that differ are the interesting part.

### 1. `ein-bugs/zebra2-bad.ein` — one boundary round

`naf_rounds` 486 → 487. `naf_admitted` 486, `naf_dropped` 0, `naf_retired` 416
in both. Pure telemetry: one extra trip round the closure/world boundary,
same admissions, same answer.

### 2. `branching/07_lookahead_off.ein` — the reported unsat core gets *smaller*

Same verdict (`Contradiction`), same `k` (0), same `exhausted` (false). The
**unsat core goes from 220 facts to 212**, and the 212 are a strict subset —
nothing added, eight dropped:

```text
(is-a Green House)  (is-a Ivory House)  (is-a Red House)  (is-a Yellow House)
(is-a H1 Color)     (is-a H2 Color)     (is-a H3 Color)   (is-a H4 Color)
```

Both runs are byte-reproducible. So on the one corpus entry where removing
`:priority` is observable in the *answer*, removing it makes the answer
**strictly better** — a smaller core is a better explanation, and 220 was
carrying eight facts it did not need.

This is [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)'s own
documented hazard, reproduced:

> `unsat_core` walks **one** justification per fact, so what it returns is
> minimal only over the derivations recorded first — flipping two rules'
> `:priority` flipped the reported core between `{C, Y}` and `{A, B, Y}` while
> `{C, Y}` still existed.

The shipped path is the ATMS label search
(`explain::smallest_contradiction_frontier`), which is *order-independent by
construction* — over **the justifications that were recorded**. A fact keeps at
most `MAX_ALT_JUSTIFICATIONS = 32`
([`kb.rs:49`](../../../ein.rs/crates/ein-core/src/kb.rs)), and which 32 those
are is firing order, which is priority. **That is
[Q2](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) with a witness
attached**, and it is why this stage depends on S1e.1.3 rather than merely
citing it.

### What the sweep did **not** compare

Stated so nobody reads it as more than it is: `ein solve` at default depth, not
the corpus's declared per-entry runs; **no** trace, DOT, event-stream or
KB-shape comparison — all four are firing-order-sensitive and **will** move;
four entries not compared at all. T1 is the same experiment done properly.

## Where `:priority` is actually read — six consumers

Every one is a disposition this stage owes an answer for.

| # | consumer | site | what removal does |
|---|---|---|---|
| 1 | the saturation agenda — `BinaryHeap<Reverse<(priority, tiebreaker, entry)>>` | [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) `priority_for` | firing **order** changes; the fixpoint does not (measured above) |
| 2 | **boundary admission** — parked candidates judged in `(priority, FIFO)` order, *the first whose guards pass is admitted* | `saturator.rs` `admit_from_boundary` | on a **non**-stratified program this picks *which* model you get. `features/07_unstratifiable.ein` gives both rules the same 200, so FIFO already decides there |
| 3 | obligation **report order** — `(:priority, load order, activator order)` | [`obligations.rs:88`](../../../ein.rs/crates/ein-infer/src/obligations.rs) | the `owes (rel: n, …)` line reorders |
| 4 | the **KB-shape digest** — `RULE … priority={} why={}` | [`shape.rs:64`](../../../ein.rs/crates/ein-core/src/shape.rs) | `corpus_shapes.md5` re-blesses, all 197 entries |
| 5 | `ein saturate --dump`'s rule listing — `:priority N (band)` | `saturate.rs` `band_label` | a column disappears from a user-visible read-out |
| 6a | which derivations are **recorded at all** → the reported **unsat core** | `saturator.rs` `record_alternative` + `explain.rs` | measured above: −8 facts on `branching/07`. **This is what § 2 measured**, and the cap is not in it: that entry's longest alternatives list is 1 (S1e.1.3, 2026-08-29) |
| 6b | which ≤ 32 of them a full list **keeps** → the same core | `kb.rs` + `explain.rs` | reachable, and reached by exactly one corpus entry (`zebra2-bad`), where it costs nothing. Fixture: `examples/ein-bugs/alt-cap-core{,-reordered}.ein`, 3 facts against 2 — S1e.1.3 / [Q-M1e.15](../open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) |

Consumers 1, 4 and 5 are presentation and bookkeeping. Consumer 3 is a
read-out. **Consumers 2, 6a and 6b are the only three that can change an
answer**, and all are about programs the engine already treats as edge cases —
6b needs a fact derived more than 32 ways, which one corpus entry manages and
no puzzle depends on.

## What "analyze rules structure instead" would produce

The number is a hand-written proxy for a property of the rule set, and
`band_label` proves it: the band is already *computed* from the number. Invert
that, and three things become derivable that a typed integer never was:

1. **Strata.** The classic Datalog construction on the relation dependency
   graph — an edge `R → S` for every rule matching `R` and asserting `S`,
   marked **negative** when the premise is under `(absent …)`. Strata are the
   longest negative path; a cycle through a negative edge is
   non-stratifiability. Half of it already exists:
   [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) computes
   which `(absent …)` guards watch a rule-derived relation and calls the result
   *"the remaining hazard"*. It has the edges and never builds the graph.
2. **The static non-stratifiability diagnostic** — the thing
   `01_grammar.md` has called *future work* since S1.7.4. Today
   `(config :warn-derived-naf true)` warns on a *shape* (NAF over a derived
   relation) that is a strict superset of the hazard; a stratification answers
   exactly, and only where the answer is *yes*. That is a better diagnostic
   than the number ever bought, and it is the prize.
3. **A derived band, checkable against the declared one.** `propagate` /
   `derive` / `eliminate` / `hypothesis` recomputed from the rule's shape —
   premise count, NAF presence, whether it asserts `(false)` or `(open ?R)`,
   whether its head is watched by another rule — and **diffed against the
   number the author typed**. The disagreement count is the finding, and it is
   the repo's own *generate or diff* rule applied to a value that has been
   hand-maintained in 353 places.

## Acceptance

- **The control experiment, taken properly and banked**: the corpus's declared
  runs, both arms, comparing summary · exit code · trace · event stream ·
  KB-shape digest. Its number is a document under `docs/`, dated and
  commit-stamped, and `utils/priority_census.py` re-takes it. A claim that
  removal is free is worth exactly what re-takes it.
- **Each of the six consumers has a written disposition** — removed, replaced
  by a derived order, or kept with the reason at the site.
- **The rule dependency graph ships**, with strata, in `ein-infer`, and is
  reachable from a read-out (`ein saturate --strata`, or a column in `--dump`).
  It is the replacement, and *"analyze rules structure"* is not a plan until
  something computes the structure.
- **A static non-stratifiability diagnostic**, with a fixture that trips it
  (`examples/features/07_unstratifiable.ein` is already there and already
  documents what it pins) and one that does not.
- **`:priority` is gone from the language, or the stage records a written
  refusal.** If gone: the keyword's disposal is decided (§ T5), the 353
  occurrences are removed in one commit, and **every golden the removal moves
  is named in this file before the commit** — the milestone's rule is that *a
  re-bless that was not predicted in the stage file is a stop, not a step*.
- **`docs/ein/reasoning/rule-evolution/analysis-of-rules.md`** — or its
  pre-refactor equivalent — states the graph, the strata, the derived bands and
  the diagnostic. That page is [S1e.5.20](s1e.5.20_docs_refactor.md)'s only
  otherwise-empty file.
- [`MA-L1`](../README.md#the-findings) — *`DEFAULT_PRIORITY`'s doc comment is
  arithmetically self-contradicting* (`1000`, described as sitting *"between
  the eliminate band (300) and the hypothesis band (900)"*) — is dispositioned
  **fixed by removal**, and [S1e.4.8](../p1e.4_low/s1e.4.8_maintainability.md)
  is told so rather than fixing a comment on a constant that is leaving.

## Tasks

### Task T1e.5.6.1 — The census and the control experiment, properly

`utils/priority_census.py`. Both arms, over the corpus's **declared** runs, not
a uniform `solve`. Compare, per entry: exit code, `--json-summary` field by
field, the `--events` stream, the markdown trace, the KB-shape digest.

Report the diffs in **four classes**, because they are four different
arguments:

- **answer** — verdict, `k`, model set, unsat core, `owes`;
- **cost** — enterings, firings, `naf_rounds`, wall;
- **narration** — trace step order, event order, DOT node ids;
- **digest** — the shape hash, which is bookkeeping and moves by construction.

The reconnaissance above is the same experiment at one-quarter depth, and its
answer was 137 / 139 identical with the two exceptions characterised. If the
proper run agrees, the removal is a presentation change plus one improvement.
If it does not, the disagreement is the stage.

Include a **lever** rather than a mirrored tree: `EIN_PRIORITY=off` making
`priority_for` return `DEFAULT_PRIORITY` for every rule. The precedent is
`EIN_OBLIGATION_CHOICE=off`, whose whole purpose is to be the control arm every
number is measured against, and it is deliberately an environment variable and
not a `(config …)` field — because `SolverConfig` is rendered into the KB-shape
digest and a knob whose settings are being compared would re-bless every shape
golden in the corpus.

### Task T1e.5.6.2 — Disposition the six consumers

The two that matter:

- **Boundary admission (2).** On a non-stratified program, admission order
  selects a model. Removing priority does not make that worse — `features/07`
  gives both rules the same number, so FIFO already decides — but it removes
  the only lever an author had. The honest replacement is the **diagnostic**:
  a program whose answer depends on admission order should be *told so*, which
  is T4, rather than silently steered by an integer.
- **The unsat core (6).** Blocked on
  [S1e.1.3](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md). Hand
  it the `branching/07` witness — 220 → 212, strict subset, reproducible — as
  input; it is a concrete instance of the question and the stage should not
  have had to find it.

### Task T1e.5.6.3 — The rule dependency graph and its strata

Build it in `ein-infer`, beside `naf_deps.rs`, which already has the edge
extraction:

- nodes: relations; edges: `(match relation) → (assert relation)` per compiled
  plan, tagged positive / negative by whether the premise is under `(absent …)`;
- strata by the standard construction; a negative cycle is the failure;
- a derived band per rule, and the diff against the declared `:priority`.

**The diff is the deliverable of this task.** If the derived band agrees with
the author's number on nearly every rule, the number is redundant and its
removal is safe by construction. If it disagrees widely, the bands encode
something the graph does not see, and *that* is what the stage discovered.

Note the trap the graph must not fall into: a **relation-polymorphic** rule
(`(rule symmetric (?rel) …)`) has no static relation name in its head. Its
edges exist only once an activator fact binds `?rel`, so the graph is over the
compile cache — the same scoping `naf_deps.rs` documents: *"pass the engine of
a saturator that has run."*

### Task T1e.5.6.4 — The static non-stratifiability diagnostic

A negative cycle is reported by name, with the rules and the relation in it.
Fixtures both ways: `features/07_unstratifiable.ein` trips it;
`features/01_not_and_absent.ein` (declared-only NAF) does not.

What it is **not**: a refusal. The engine answers a non-stratified program
today, deliberately, and
[`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs)
`an_unstratifiable_loop_converges_to_a_supported_model` pins the answer *"the
honest statement of what Ein computes — a fixpoint supported at the boundary"*.
The diagnostic says the answer is one of several; it does not withhold it. If
anyone wants a refusal that is a `Q-M1e.<n>`.

### Task T1e.5.6.5 — Remove it

Three decisions, in order:

1. **What happens to a file that still says `:priority 100`?** The repo has a
   precedent and it is in the loader: `QUERY_KEYWORDS` keeps `mode`, read by
   nothing, because *"rejecting it would make a stale file fail to load rather
   than fail to matter. Accepted-and-ignored is documented;
   silently-unknown is not."* **Take that**: accept, ignore, document, and
   note it in `defined_behaviour.md`'s diagnostics.
2. **Strip the corpus** — 353 occurrences, 82 files, one commit, no other
   change.
3. **Re-bless, predicted in advance.** The goldens this moves, named now:
   `corpus_shapes.md5` (the digest prints `priority=`);
   `trace_*.md` and `from_ein_py/{trace_3step.md,zebra.golden,zebra2.golden}`
   (firing order); `events_*.jsonl`; `dump_*` (the band column). Anything
   *else* that moves is a finding, not a re-bless.

### Task T1e.5.6.6 — The docs

`01_grammar.md` § Rules loses the `[:priority <INT>]` line and gains the
sentence about the accepted-and-ignored keyword; its § Premise forms bullet
about priority being advisory becomes the statement that the order is derived,
and its *"a static stratification check remains future work"* becomes a link.
`docs/kernel/glossary.md`, `architecture.md` and `inference/README.md` mention
bands; each is checked, not assumed.

## Risks

- **The measurement was taken on the wrong thing.** The reconnaissance
  compared `--json-summary` and not narration. Traces *will* move, and a
  reviewer who reads "137 identical" as "nothing moves" will be surprised by
  the re-bless. T1 exists to make the four classes separate before anyone
  quotes the number.
- **Removing the only steering wheel on a non-stratified program.** Real, and
  small: the corpus's one such fixture does not use it. But a user's program
  might, and *accepted-and-ignored* means their program silently changes
  behaviour on upgrade. The diagnostic is the mitigation and it must ship in
  the same release, not later.
- **The graph is not free.** A dependency graph over the compile cache, built
  per run, on programs whose plan count is in the hundreds. Measure it; if it
  costs, it is a read-out and a check, not something every `ein solve` builds.
- **Doing it before [Q2](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md).**
  The `branching/07` witness shows priority changing a printed core by eight
  facts. Removing the input to a truncation nobody has characterised means the
  next core change has no baseline to be compared against. Hence the hard
  dependency.
- **`:priority` is 92 % of rules and it reads as intentional.** Someone wrote
  90, 110, 120, 220, 240, 250 — six values inside a hundred-point band. If the
  derived-band diff (T3) says those distinctions are invisible, that is the
  result. If it says they are not, the stage should stop and say so rather than
  remove them because the plan said it would.

## Notes

The strongest version of the user's instruction is not *delete a keyword*. It
is: **a rule's place in the schedule is a property of the rule set, and a
property of the rule set should be computed from the rule set.** `band_label`
already computes the band from the number; the number is the only step in that
chain a human types, and it is the only one that can be wrong.

The measurement says the cost of removing it is one telemetry counter and a
smaller unsat core. The value of replacing it is a diagnostic the grammar page
has promised since S1.7.4.

## Connections

- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) § Rules
  and § Premise forms — where `:priority` is specified and where it is already
  called advisory.
- [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) — the
  existing advisory map, and the half of the graph that is already built.
- [`docs/kernel/inference/absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
  — P1/P2, the normative statement that the guard is judged at the closure
  boundary, which is *why* priority stopped deciding derivability.
- [S1e.1.3](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) — Q2,
  the blocking question, now with a witness.
- [S1e.4.8](../p1e.4_low/s1e.4.8_maintainability.md) — [MA-L1](../README.md#the-findings),
  which this stage closes by deleting the constant.
- [S1e.5.20](s1e.5.20_docs_refactor.md) T4 — `rule-evolution/analysis-of-rules.md`,
  the page this stage writes the content of.
