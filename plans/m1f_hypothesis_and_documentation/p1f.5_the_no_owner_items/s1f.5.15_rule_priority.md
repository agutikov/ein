# S1f.5.15 — Remove `:priority`; derive the order from the rules

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Was S1f.5.6**, until 2026-11-08, when P1e2.5's thirteen stages joined the phase ahead of it ([§ The ids](README.md#the-ids)).
**Estimate:** 6 days — 3 to measure and decide, 3 to remove.
**Status:** specified 2026-08-28, not started. **Moved on 2026-11-08, on the user's ruling, to run after
[S1f.5.5](s1f.5.5_the_grammar.md)'s T3 and before its T8, T9, T10 and the swap (T4)**, where P1f.5's order had it
last ([P1f.5 § The order](README.md#the-order)). The user's principle that day: *"collect all requirement for the
parser in this phase and implement grammar that will already meet all of them instead of making wrong parser that
meets our old wrong parser and then updating it step by step"*. `:priority` was one of the two items that would
have edited the generated grammar later, and the user chose *"S1f.5.15 before swap"*. So the generated grammar
never has `:priority`: T3 built it without (`54715b5f`, [S1f.5.5 § T3
record](s1f.5.5_the_grammar.md#t3-record-2026-11-08)), refusing a rule's `:priority` like any keyword a rule does
not take, and attributed to this stage, pending, the 133 corpus files that differ between the two parsers only by
their `:priority`. This stage removes the keyword from every program both parsers read before the swap, so S1f.5.5's
T9 has no `:priority` difference to attribute. **If it ends in the written refusal its acceptance allows, the
production comes back then**, in the generated grammar and before the swap, and S1f.5.4's `:priority` guard
(T1f.5.4.3) with it. **The keyword's disposal (§ T5, decision 1) is settled for the generated grammar**: refused
like any keyword a rule does not take. What the hand-written parser does with it between this stage and the swap is
this stage's to decide and record (§ T5). T1 re-takes § Context's count: T3 found 482 `:priority` clauses in 137 of
the 414 files under `examples/`, `stdlib/`, `tests/` and `corpus/` (`grep -l`, `grep -o`, 2026-11-08), where
§ Context's 353 in 82 files are `9ba2349`'s.
**T1 done 2026-11-08, and the stage stopped there, before any removal** ([§ T1 record](#t1-record-2026-11-08)):
taking every `:priority` out of the corpus and the stdlib moves no R, N, H, claim, exit code or stored fact on any
of the 511 programs or the corpus's 1 378 declared runs, and it moves **the facts of cores** on seven corpus
programs, `zebra.ein`, the zebra2 family and W25's control, each through an order-dependency the store already
has (the alternatives cap, `:record-alternative-justifications false`, W25). Whether to remove it anyway, derive
the order first or keep it is [Q-M1f.4](../open_questions.md#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs),
put to the user.
**Depends on:** [S1f.5.5](s1f.5.5_the_grammar.md)'s T3, done 2026-11-08 (`54715b5f`): the generated grammar this
stage leaves `:priority` out of. It lands on the programs both parsers read, before S1f.5.5's T8 and the swap.
And on [P1e.1](../../../docs/history/m1e_review_processing/README.md#p1e1--the-ten-questions)
[S1e.1.3](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2) — **hard.**
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
> second has a fixture ([Q-M1e.15](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported)).
>
> **A dated note, 2026-11-04, nothing re-scoped** (M1e2
> [S1e2.2.11](../../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md)):
> a full list now keeps its smallest-closure arrival beside it, so the
> `alt-cap-core` pair report one core, 2 facts, and are no longer consumer
> 6b's fixture. 6b stands on
> `examples/ein-bugs/wrong-answers/alt-cap-shared.ein`, `W10/alt-cap-shared`,
> deferred to F22: 3 facts as written and 2 with `x-join` at `:priority 50`
> (`ein solve`, 2026-11-04), a saving between two facts that the full list
> refuses unless its rule fires first. Ten programs reach the cap under R's
> routes, not one (`ein solve FILE --events`, 2026-11-04).
**Blocks:** nothing. It writes the first content of
[P1f.20](../p1f.20_docs_refactor/README.md)'s
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
[Q2](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2) with a witness
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
| 6b | which ≤ 32 of them a full list **keeps** → the same core | `kb.rs` + `explain.rs` | reachable, and reached by exactly one corpus entry (`zebra2-bad`), where it costs nothing. Fixture: `examples/ein-bugs/alt-cap-core{,-reordered}.ein`, 3 facts against 2 — S1e.1.3 / [Q-M1e.15](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) |

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
  removal is free is worth exactly what re-takes it. **Taken 2026-11-08** ([§ T1 record](#t1-record-2026-11-08)),
  on the coordinator's correction: the walk's counters are gone, and the scripts that re-take it stay under
  `build/s1f.5.15/`, as S1f.5.4's did, since once no program writes the keyword there is no arm to take.
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
  the diagnostic. That page is [S1f.20.4](../p1f.20_docs_refactor/s1f.20.4_reasoning.md)'s only
  otherwise-empty file.
- ~~[`MA-L1`](../../../docs/history/m1e_review_processing/README.md#the-findings) — is
  dispositioned **fixed by removal**, and
  [S1e.4.8](../../../docs/history/m1e_review_processing/README.md#s1e48--maintainability-low)
  is told so rather than fixing a comment on a constant that is leaving.~~
  **Overtaken 2026-09-01**: S1e.4.8 fixed it. Under
  [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md) Rule 2 the
  premise *the constant is leaving* is enforced by nothing that fails when it
  stops being true — this stage may end in a written refusal, and it has not
  started — and a deferral would have needed a note **at the site** anyway,
  which costs the same edit as the truth. What is there now is the sort
  direction, the measured bands with their command and date, the second
  consumer (`obligations::priority_of`, which nothing had written down), and
  `explain_semantics::the_default_priority_is_1000_and_fires_last`. **If this
  stage removes `:priority`, all of it goes with the constant** — which is the
  normal cost of a pin, and cheaper than a wrong comment standing for an
  unscheduled milestone.

## Tasks

### Task T1f.5.15.1 — The census and the control experiment, properly

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

### T1 record, 2026-11-08

**Taken as the coordinator corrected it on 2026-11-08**, since the walk this task was written against is gone
(M1e2 S1e2.1c.5): no `EIN_PRIORITY` lever, because the binary reads two `EIN_*` names and `config_reference.rs`
pins them, and no verdict, `k`, enterings or `naf_rounds` to compare. The stripped arm is
`ein_corpus::mutants::without_priority`, S1f.5.5's T3 function, applied to every `.ein` under the five roots and
to the stdlib, which the run then reads through `EIN_STDLIB` with its manifest rewritten; the other arm is the
tree as written. Both arms run one binary, `ein` at `76318fd6`, on the same relative path, so every output
compares byte for byte but for the input's character count `saturate` prints and the absolute path three
`examples/broken/load/` programs' import errors name. The scripts stay under
`build/s1f.5.15/` (gitignored), as S1f.5.4's did: `run_sweep.sh` and `sweep.sh` (pass 1), `declared.py` (pass
2), `analyze.py` (the classes below), `nocap_probe.sh` (the cause), and `strip/`, a crate whose one function
calls `without_priority`.

**What it takes out**: 576 clauses in 178 of the 511 files under `examples/`, `tests/`, `stdlib/`, `corpus/` and
`plans/`; **475 in 136 of the 414 under the first four**, where T3's 482 in 137 (`grep -o`) also counted seven
mentions of the keyword in comments (`alt-cap-core.ein` three times, `-reordered`, `where-on-rule`, `std.algebra`
and `std.slots`); 101 in 42 records under `plans/`. All 77 stdlib rules carry one. The corpus writes seventeen
values, 5 (2), 10 (6), 50 (1), 90 (7), 100 (92), 110 (21), 120 (3), 200 (127), 210 (2), 220 (6), 240 (46), 250
(49), 300 (54), 350 (1), 400 (19), 500 (36) and 900 (3) (`grep -rhoE ':priority +-?[0-9]+' --include='*.ein'
examples tests stdlib corpus`), where § Context counted twelve at `9ba2349`.

**Pass 1, every file through three commands on both arms**: `ein solve FILE --json-summary J -H --dump-answer
D`, `ein saturate FILE --dump` and `ein test FILE`, 511 files, 3 066 runs (`build/s1f.5.15/run_sweep.sh`, then
`analyze.py`):

| compared | moved |
|---|---:|
| the exit code of each run | **0** |
| R: the summary's `region`, but for root's core and its two companions: k_s, k_c, `exhausted`, every state and its facts, the lines, the holes, the components, the universe rows | **0** |
| the state files `--dump-answer` writes, as their forms | **0**; on 167 files they differ by the program's `:priority` clauses, its comments and the order facts are written in, and by nothing else |
| N, the sets of `nogoods.json` | **0** |
| H, `forms/hyp-stats.txt`, and `root.hypgen`'s `outside`, `types_as_objects` and count | **0** |
| the claims: the summary's `claim`, each query's classes and `:expect`, and `ein test`'s report | **0** |
| `saturate --dump`'s stored facts as a set, and its derived facts by relation | **0** |
| the detector's block but `order`, and each `order` finding as its reader, writer and relations | **0** |
| **the facts of a core**, root's or a member of N's | **9 programs**, below |
| `refuted by` beside an unmoved core | 14, 12 of them corpus programs: `branching/01` to `/11` (`symmetric` → `transitive` beside `sibling-exclusive`, 2 to 62 dead sets each), `zebra2-bad` (`adjacent-via-endpoint-bwd` → `co-located-negative`, root's and N's one), and two records, S1e2.3.10's `two-rules-node` and `-reordered` |
| the priority a detector `order` row prints | 9: `features/07`, `f07-fpoff`, `f07-rules-swapped` and `lattice/10`, `200` → `1000`, and five records; on `detector_proposal/fifo_gap_priority.ein`, a record, the wording, *r2 (priority 100) … is admitted first, whatever the order* becoming *both at priority 1000: … the order they are written in* |
| narration: the rule a stored fact prints, its first justification | 23 |
| cost: `saturate`'s per-rule firing breakdown | 21; and the one summary counter, `root.saturator.naf_rounds` on `zebra2-bad`, 486 → 487, as § 1 found |
| presentation: `saturate --dump`'s rule listing, `:priority N (band)` | 205 |

**Pass 2, the corpus's declared runs**: every run `corpus.toml` declares, the 404 entries' 1 378 cells, the
slow ones included, through `ein_corpus::plan`'s argv (`build/s1f.5.15/declared.py`). No exit code moved, and 197
cells differ: 181 `saturate` and `saturate --dump` (the listing, the narration, the character count); 4 `solve
--trace` (`branching/04`, `/05`, `zebra2`, `zebra2-obligations`: the step order); 7 the detector's `order` rows on
the four programs above (`solve`, `--minimal`, `--maximal`, `oracle --dump`); `zebra2-bad`'s `solve` (`refuted
by` and `naf_rounds`); and 4 `solve --dump-lattice`: `refuted by` on 8 of `branching/04`'s nodes and 3 of `/05`'s,
and on `zebra2` and `zebra2-obligations` **the cores of 24 of their 190 dead nodes**, 17 a fact or two larger and
7 the same size, every node's state, status and debts the same.

**No R, N, H, claim, exit code or stored fact moved, and a core's facts did: this is the stage's stop**
(the coordinator's brief: *a core's facts moving is a stop*). `nogoods.json`, root's core with it
(`build/s1f.5.15/core_moves.txt`):

| program | N | cores whose facts move | facts over N | what decides the core |
|---|---:|---|---|---|
| `zebra.ein` | 458 | 292: 223 grow, 50 shrink, 19 the same size | 13 193 → 14 697 | the program sets `:record-alternative-justifications false`, so a fact keeps its first derivation and nothing else, and firing order is the core: the flag's exemption (M1e2 S1e2.2.11). Uncapped, the same 292 move |
| `zebra2.ein`, `zebra2-obligations.ein` | 366 | 31: 18 grow, 13 the same size | 5 154 → 5 184 | **the alternatives cap**, `MAX_ALT_JUSTIFICATIONS = 32`: at 10⁶ the two arms agree on every core |
| `zebra2-hints.ein` | 73 | 24: 20 grow, 4 the same size | 1 005 → 1 042 | the cap, as above |
| `zebra2-minus-15.ein`, `-obligations` | 1 495 | 215: 118 grow, 9 shrink, 88 the same size | 31 151 → 31 284 | the cap, as above |
| `ein-bugs/wrong-answers/guard-only-first.ein` | 1 | `(p a)` → `(b a) (p a) (seed a)` | 1 → 3 | **W25**: a derivation with no premise is never recorded as an alternative, so the rule that fires first is the core. This is W25's **control**, the one program that shows the row's right answer, and stripped it reports the witness's wrong one |
| `plans/…/inventory_2026-11-05/probes/guard_only_first.ein`, a record | 1 | the same | 1 → 3 | W25 |
| `plans/m1e2_correctness/probes/alt-cap-below-noalts.ein`, a record | 1, root | `(p1 Y01) (p2 Y01) (p3 Y01)` → `(w1 X) (w2 X)` | 3 → 2 | alternatives off, as `zebra.ein` |

**The cause, measured** (`build/s1f.5.15/nocap_probe.sh`): a snapshot of `76318fd6` with
`MAX_ALT_JUSTIFICATIONS` at 10⁶ and nothing else changed (`build/s1f.5.15/nocap/`, built
`--no-default-features`). On it the zebra2 family's arms agree on every core (0 of 366, 73 and 1 495 move), and
the capped cores as written differ from the uncapped ones on 113 of `zebra2`'s 366, 34 of `zebra2-hints`' 73 and
455 of `zebra2-minus-15`'s 1 495: **the cap decides those cores today**, and the typed order only picks which 32
derivations it keeps. On `zebra.ein` the cap decides nothing (uncapped, 0 of 458 cores differ from today's) and
the same 292 move, since with the flag off no alternative is recorded at all (`ein saturate --events
--events-level verbose`: no `alt` event on either arm). With the flag turned on in a copy of `zebra.ein`,
neither arm's `ein solve -H --dump-answer` had ended after 20 minutes, two at once on an otherwise idle machine
(stopped there), which is the cost the program's own comment gives for setting it off. On the lattice `solve
--dump-lattice` visits, `zebra2`'s 24 moved cores are the cap's as well: uncapped, the arms agree on all 374
nodes.

**Every move is an order-dependency the store already has**, owned before this stage: the cap is Q-M1e.15's and
F22's (W10), the flag's exemption is S1e2.2.11's ruling, and W25 is F22's. The typed number decides the order those
three read today; removing it hands the order to the written order (FIFO within one priority) and the cores move
with it.

**A seventh consumer**, which § Where `:priority` is actually read predates: **the detector's order check**,
`detector::order` (M1e2 S1e2.1b.5). It reads each deferred rule's priority, drops a race whose writer's priority
is earlier as the stratified order, and words each `order` row by whether the two are at one priority. With the
keyword gone every pair is at one priority. On the 511 no finding is added or lost; the rows print `1000` for
`200`, and `fifo_gap_priority.ein`'s row its other wording. And an eighth that is narration only: the `enqueue`
event's `priority` field (`events.md`).

**What else the stripped corpus moves**, before any code is removed: `cargo test --workspace` on a snapshot of
`76318fd6` with the 475 clauses taken out and the stdlib's manifest rewritten (`build/s1f.5.15/proto/`,
`build/s1f.5.15/proto-test.log`, 2026-11-08): **918 passed and 11 failed, in 9 targets**. Seven are the
removal's by construction and were to be named before it (re-banked on the snapshot with `EIN_BLESS=1`, four
goldens move: `corpus_shapes.md5`, 1 340 of its 15 022 lines, `generated_parser_corpus.txt`, `zebra2.golden` and
`trace_two-level.md`): `corpus_shapes` (`priority=` in every digest),
`generated_parser::the_builder_holds_on_the_corpus` (its two readings become one: 322 the same tree and 18 the
same but for digits, **340 of 414 agree as written**, and the 133 that differed only by their `:priority` are
none), `dump_goldens` (`zebra2.golden`: the parsed program prints no `:priority`), `golden_trace`
(`trace_two-level.md`, the step order), `region_modes::the_monotone_question_is_about_the_program` and
`detector`'s two (the order rows' `200` pinned). **Four are not the stage file's prediction**:

- **`lever_invariance::no_config_flag_moves_a_line_of_any_mode`: the config principle breaks.** On
  `examples/features/12_expect_false.ein` with its `:priority` taken out, `:record-alternative-justifications
  false` prints `refuted by forced, occupied` where the default prints `refuted by no-room`, beside the same
  five-fact core. The flag's exemption is the core (S1e2.2.11), and S1e2.3.10's ruling holds `refuted by` to it:
  *"moves it only where it moves the core"*, argued from *"where the core is the same the primaries are a
  derivation from it"*. Here one core derives the contradiction two ways, and the flag picks which. It is a
  wrong answer by the repo's own rule, and **it exists today**: today's binary prints the two lines on the
  program written without `:priority` (`ein solve`, with and without the flag, 2026-11-08); the typed order is
  what keeps the corpus from showing it.
- **`lever_invariance::the_core_exemption_is_one_flag_and_the_core_rows`**, **`explain_semantics::
  a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core`** and **`ein-einb`'s
  `roundtrip::a_saturated_kb_round_trips_with_its_derivations`**: each reads `alt-cap-core.ein` and its
  `-reordered` twin, which are **one `:priority` apart and nothing else**, so stripped they are one program; the
  flag's exemption loses its witness and the store's smallest-closure arrival is no longer exercised there.
  W10's `alt-cap-shared` stands as it was (3 facts, both arms).

`wrong_answers.rs` passes: W25's control is a record the test does not run, so its move shows in no test.

**So the stage stops here, before any removal, on two questions for the user**:
[Q-M1f.4](../open_questions.md#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs),
whether to remove `:priority` with the cores' moves, derive the order from the rules first (the agent's
recommendation), or keep it until F22; and
[Q-M1f.5](../open_questions.md#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core), whether
`features/12`'s `refuted by` under the flag is a register row owned by F22 (the agent's recommendation) or the
flag's exemption widened. Q-M1f.5's wrong answer exists with or without this stage; the removal would put it in
the corpus, where `lever_invariance` fails on it, so Q-M1f.4 (a) and (b) wait on it too. Nothing in the tree
moved: no engine code, test, golden, fixture or register row.

### Task T1f.5.15.2 — Disposition the six consumers

The two that matter:

- **Boundary admission (2).** On a non-stratified program, admission order
  selects a model. Removing priority does not make that worse — `features/07`
  gives both rules the same number, so FIFO already decides — but it removes
  the only lever an author had. The honest replacement is the **diagnostic**:
  a program whose answer depends on admission order should be *told so*, which
  is T4, rather than silently steered by an integer.
- **The unsat core (6).** Blocked on
  [S1e.1.3](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2). Hand
  it the `branching/07` witness — 220 → 212, strict subset, reproducible — as
  input; it is a concrete instance of the question and the stage should not
  have had to find it.

### Task T1f.5.15.3 — The rule dependency graph and its strata

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

### Task T1f.5.15.4 — The static non-stratifiability diagnostic

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

### Task T1f.5.15.5 — Remove it

Three decisions, in order:

1. **What happens to a file that still says `:priority 100`?** The repo has a
   precedent and it is in the loader: `QUERY_KEYWORDS` keeps `mode`, read by
   nothing, because *"rejecting it would make a stale file fail to load rather
   than fail to matter. Accepted-and-ignored is documented;
   silently-unknown is not."* **Take that**: accept, ignore, document, and
   note it in `defined_behaviour.md`'s diagnostics.

   **Settled for the generated grammar on 2026-11-08, the other way.** The user ruled on 2026-11-07 that each
   form takes a fixed set of keywords and any other is an error
   ([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take),
   [S1f.5.11](s1f.5.11_closed_keywords.md)). That overtakes the `QUERY_KEYWORDS` precedent: S1f.5.11 refuses
   `:mode`, and S1f.5.5's T5 deletes `QUERY_KEYWORDS`. The generated grammar refuses `:priority` like any keyword a
   rule does not take (T3, `54715b5f`). What is left is the hand-written parser, between this stage and the
   swap. Taking `from_ir.rs`' `priority` match out alone would make `:priority` an unknown keyword on a rule,
   which today's loader drops in silence (D-15, S1f.5.11's case): that is accept-and-ignore, undocumented, and a
   program that changes behaviour in silence on upgrade (§ Risks). **Recommended: refuse it at load once no
   program writes it**, one loader line naming the rule and saying the keyword is gone, so that the two parsers
   agree and T9 has nothing to attribute here. The line is the loader's, not the parser's, so it keeps S1f.5.5's
   rule that nothing edits the hand-written parser for a language stage. It is also a line S1f.5.5's T5 deletes
   once the grammar refuses the keyword, which is the one cost: the re-plan wrote no such line for S1f.5.11's lists.

   **Decided 2026-11-08 by the coordinator, the other way, on the user's ruling against double work**: no loader
   line, since S1f.5.5's T5 would delete it. The removal takes out the consumers and `from_ir.rs`' `priority`
   match; until the swap a `:priority` written anyway is a rule keyword the hand-written loader drops, as it drops
   any (D-15, S1f.5.11's case), and from the swap the generated grammar refuses it at the parse. The corpus side
   is held by a test that no program under `examples/`, `stdlib/`, `tests/` or `corpus/` writes `:priority`.
2. **Strip the corpus** — 353 occurrences, 82 files, one commit, no other
   change.
3. **Re-bless, predicted in advance.** The goldens this moves, named now:
   `corpus_shapes.md5` (the digest prints `priority=`);
   `trace_*.md`, `zebra.golden` and `zebra2.golden`
   (firing order); `events_*.jsonl`; `dump_*` (the band column). Anything
   *else* that moves is a finding, not a re-bless.

   **Since S1f.5.5's T3 (2026-11-08)** the list also has `ein-ir`'s
   `generated_parser::the_builder_holds_on_the_corpus`, which reads the corpus both as written and with
   `ein_corpus::mutants::without_priority`, and its golden, `ein-ir/tests/golden/generated_parser_corpus.txt`:
   once no file writes `:priority` the two readings are one, and its 133 files differing only by their
   `:priority` are none.

### Task T1f.5.15.6 — The docs

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
- **Doing it before [Q2](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2).**
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
has promised since S1.7.4. **Re-taken on 2026-11-08 ([§ T1 record](#t1-record-2026-11-08)), it says more**: the
cores of seven corpus programs move, and a config-principle breach shows.

**Owed to this stage's first code commit**, from the coordinator's brief of 2026-11-08: `ein.rs/Cargo.toml`'s
comment on the vendored `lrpar` still says sending the patch upstream is the user's call, and the user ruled that
day *"don't send the lrpar patch upstream either"*. T1 changed no code, so the comment waits for T3's commit or
T5's.

## Connections

- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) § Rules
  and § Premise forms — where `:priority` is specified and where it is already
  called advisory.
- [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) — the
  existing advisory map, and the half of the graph that is already built.
- [`docs/kernel/inference/absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
  — P1/P2, the normative statement that the guard is judged at the closure
  boundary, which is *why* priority stopped deciding derivability.
- [S1e.1.3](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2) — Q2,
  the blocking question, now with a witness.
- [S1e.4.8](../../../docs/history/m1e_review_processing/README.md#s1e48--maintainability-low) — [MA-L1](../../../docs/history/m1e_review_processing/README.md#the-findings),
  which this stage closes by deleting the constant.
- [S1f.20.4](../p1f.20_docs_refactor/s1f.20.4_reasoning.md) (was S1f.5.20's T4) — `rule-evolution/analysis-of-rules.md`,
  the page this stage writes the content of.
