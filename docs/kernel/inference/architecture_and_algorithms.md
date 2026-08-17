# Inference engine — architecture, operations, and their CS analogs

> A cross-cutting analysis of the reasoning engine: its architecture and
> main steps, the abstract **operations** it performs, those operations'
> **analogs** in other fields of computer science, and the **fast / optimal
> known algorithms** for each. It is a map from this puzzle reasoner to the
> broader literature — written to orient optimization work
> (P1.8a,
> the [M1a Rust port](../../../plans/m1a_rust/)) and to make the engine's
> design choices legible against the state of the art.
>
> Source of truth for the code: `ein.py/src/ein/inference/` (engine),
> `ein.py/src/ein/kb/` (data model). For the *planned* how-to chapters
> (`01_matcher.md` … `05_trace.md`) see [`README.md`](README.md); this file
> is the architecture + algorithms overview those chapters sit under.

---

## 1. What the engine solves, and the three paradigms it fuses

Ein answers **finite-domain constraint-satisfaction** questions over a
[typed hypergraph](../ir/01-ein-graph/01_kb.md) of facts and
[rules](../ir/01-ein-graph/02_rules.md) — the Zebra puzzle and its kin. Per
[idea 03](../../../plans/ideas/03-three-task-classes.md) it answers three
shapes of one question, all read off a single search (P1.7a):

- **solve** — is there a unique complete model? (k = 1)
- **gaps** — which cells are forced vs contingent? (k > 1, the residual)
- **contradictions** — the unsat core of an over-constrained KB: the smallest
  set of given facts from which one recorded contradiction follows
  (provenance-based, searched across every recorded derivation; **not** a
  subset-minimal MUS). (k = 0)

These are **three answers to one problem, not three problem statements**: the
verdict is *read from* the result — the count `k` of distinct
`state_key`-deduped solution nodes (`verdict_of`) — never *chosen* up front.
There is **one** public entry, `solve()`; the stop policy (single /
`stop_after=N` / exhaustive) only bounds how far the lattice is walked, and the
optional `store_lattice` proof carries the gaps view (the full solution set)
and the contradictions view (the refutation map) for whichever answer the
puzzle yields. (An earlier design exposed sibling `gaps_solve` /
`contradictions_solve` entries that hard-wired `Ambiguity` / `Contradiction` by
*which function was called* — and so returned mutually contradictory verdicts
on the same KB. They were removed 2026-06-16; this is the soundness story
P1.7a's `verdict_of` always intended.)

The engine is the confluence of **three classical paradigms**, and almost
every component below is recognisable as a piece of one of them:

| paradigm | what Ein borrows | classic systems |
|---|---|---|
| **Deductive database** (Datalog) | bottom-up forward chaining to a least fixpoint; negation evaluated stratum-at-a-time (here: at a closure/world boundary, §O3) | Soufflé, LogicBlox, DDlog, Datomic |
| **CSP / SAT solver** | branch on undecided choices, propagate, detect clashes, learn no-goods | DPLL/CDCL (MiniSat, Chaff), CSP (Gecode), ASP (clingo) |
| **Truth-maintenance system** (ATMS) | hypotheses as *assumptions*, no-goods, provenance/justifications, retract-on-contradiction | de Kleer's ATMS, JTMS |

The implementation splits cleanly into two layers along the
monotone/non-monotone seam:

- **Deductive layer (monotone, append-only).** Saturate: fire rules to
  quiescence, never retract. Since S1.21.8 it is itself two-phase — a
  **purely positive closure** that consults no negation, and a **boundary**
  at each closure quiescence where the `(absent …)` guards are judged
  against that fixpoint (§O3). `saturator.py`, `match.py`, `compile.py`,
  `engine.py`, `firing.py`, `world.py`, `contradiction.py`, `resolve.py`,
  `predicates.py`.
- **Search layer (non-monotone).** Branch: enumerate candidate
  *commitments*, fork-and-saturate each, learn from deaths, dedup models,
  read a verdict. `monotonic/solver.py`, `commitment.py`, `hypgen.py`,
  `apriori.py`, `nogoods.py`, `lookahead.py`,
  `monotonic/solution.py`, `verdict.py`.

---

## 2. Architecture and the main steps

```text
  ein-lang (.ein)                       ── IR layer ──
        │  parse  (ir/grammar.lark → SForm/Atom/Var/…)
        ▼
  KnowledgeBase  ───────────────────────────────────────────  ── data model ──
   relations · rules · hrules · facts(ontology|fact|reasoning)
   + 7 reverse indexes  + EqClasses(union-find)  + provenance
        │
        │  Engine.compile_all()   rule × activator → JoinPlan
        │                         positive Scan/Join/Guard steps
        ▼                         + naf_guards (compile.split_naf)
  ┌─────────────────────────────────────────────────────────┐
  │  DEDUCTIVE LAYER  (monotone, per KB)                      │   ── inference ──
  │   Saturator.saturate()  ── two phases to fixpoint ──      │
  │    CLOSURE  (purely positive — no negation consulted)     │
  │     match (multi-way join)  → fire  → append fact         │
  │     ▲ priority queue, delta-driven (semi-naive, S1.8.B2v) │
  │     │ a NAF-guarded match is PARKED, never enqueued       │
  │    BOUNDARY  at quiescence: World(kb) judges the parked   │
  │     └ candidates, admits ONE → back into the closure      │
  │   ContradictionDetector.detect()  (X,¬X) | (false)        │
  └─────────────────────────────────────────────────────────┘
        ▲ fork()                                  │ alive set
        │                                          ▼
  ┌─────────────────────────────────────────────────────────┐
  │  SEARCH LAYER  (non-monotone)                            │
  │   _phase1_root   saturate root, forced-positive cascade  │
  │   hypgen         enumerate undecided candidate facts     │
  │   _phase2_layers BFS the commitment-set lattice by size  │
  │      apriori.generate_layer  (prefix-join + nogood prune) │
  │      try_commitment_set      fork+write+saturate+detect   │
  │      ├─ dead  → emit no-good (prunes supersets)           │
  │      └─ alive → record solution node iff complete∧consistent
  │   verdict_of(k deduped solution nodes by state_key)       │
  └─────────────────────────────────────────────────────────┘
        ▼
  Verdict = Solution(k=1) | Ambiguity(k>1) | Contradiction(k=0)
```

**The deductive inner loop** (one KB → fixpoint). `Engine.compile_all`
compiles every `(rule, activator)` pair into a `JoinPlan` — a sequence of
`Scan`/`Join`/`Guard` opcodes (`compile.py`). Since S1.21.8 that sequence is
**purely positive**: `compile.split_naf` lifts every top-level `(absent …)`
premise out into `JoinPlan.naf_guards`, one guard tuple per `(or …)`
disjunct, which `JoinPlan.disjuncts()` pairs back with its steps. The
`Saturator` then alternates two phases (`step()`). In the **closure** phase it
runs a priority queue of enqueued `(plan, binding)` firings; each
`_closure_step()` pops the highest-priority unfired binding, applies it
(`firing.fire` → append a derived `Fact` with provenance), and
re-enqueues the matches the new fact enables — consulting no negation
anywhere. A match whose disjunct carries guards never enters that queue: it is
**parked** (`_enqueue_binding`). At closure quiescence the **boundary** phase
builds a `World` over the stalled KB, judges the parked candidates against
that fixpoint, and admits exactly **one** (`_admit_from_boundary`), which
re-enters the closure. `step()` returns `None` — the real fixpoint — only when
a quiescence admits nothing. Since S1.8.B2v the closure's re-enqueue is
**delta-driven**: only plans whose premises touch the just-derived fact's
relation are re-matched, and they are **seeded** at the new fact rather than
re-scanned (`saturator._enqueue_pass`, `match.run_seeded_guarded`).
`Engine.step()` — the queue-less loop the `Saturator` wraps — implements the
same two phases directly: positive matches first, the boundary only once none
remain.

**The search outer loop** (many KBs → a verdict). `_phase1_root` saturates
the root and runs the *forced-positive cascade* (while the alive set is a
singleton, promote it to a root fact and re-saturate — unit propagation).
`_phase2_layers` then explores the **commitment-set lattice** breadth-first
by set size: layer 1 = singleton hypotheses, layer k = Apriori prefix-joins
of layer k-1 (`apriori.generate_layer`), pruned by learned no-goods. Each
candidate goes through `try_commitment_set`: `fork()` the root, write the
hypotheses, saturate, detect. A **dead** branch emits a no-good clause (whose
supersets Apriori then prunes — downward closure); an **alive** branch that is
`complete ∧ consistent` (`solution.is_solution_node`) is recorded as a solution
node, deduped by `state_key`. The root stays stable mid-search — an alive
branch's consequences never merge back (the "unconditional"-fact extraction
was retired in P1.21 R2 as unsound under NAF). The verdict is read off
the deduped count k (`verdict_of`).

The **closure/worlds seam** this picture is drawn against — NAF lifted out of
the closure onto an explicit boundary — is **shipped** as of S1.21.8; the seam
census and the package-layout debt it names live in
[`../architecture.md` §closure/worlds seam](../architecture.md) (P1.21 R6),
and the normative reading of a boundary query is
[`absent_semantics.md`](absent_semantics.md).

---

## 3. Data types

| type (`module`) | what it is | analog |
|---|---|---|
| `Fact` (`kb/entities`) | `(relation_name, args)` identity; `args ∈ str \| int \| Fact` (nested = a relational node); carries `provenance` | a ground atom / tuple / labelled hyperedge |
| `Relation` (`kb/entities`) | a named relation + `signature` (type atoms) | a database relation schema / predicate symbol |
| `Rule` (`kb/entities`) | `params`, `match` pattern, `assert`, `priority`, activator | a Datalog/production rule (Horn-ish clause) |
| `JoinPlan` = `Scan`/`Join`/`Guard` steps + `NafGuard`s (`compile`) | a rule's `:match` compiled to a join program over relations; its `(absent …)` premises split off (`split_naf`) into a per-disjunct guard tuple, paired back by `disjuncts()` | a query plan / RETE network / WAM-ish opcode list, plus its negation side-conditions |
| `World` (`world`) | a read-only view of a saturated KB taken at a quiescence point (not a snapshot), plus the commitment it assumes; the only thing an `(absent …)` is ever asked of (`holds` / `absent` / `admits` / `negative_premises`) | a possible world / an ATMS environment; the `W` of `W ⊭ ∃x̄.Pθ` |
| 7 KB indexes (`kb/store`) | `_facts_by_relation`, **`_facts_by_rel_slot_val`** (the participation index, `(rel,slot,val)→facts`), `_negated_facts`, `_rule_apps_*`, `names`, … | database join indexes; RETE alpha-memories |
| `EqClasses` (`kb/store`) | union-find over names (a *placeholder* — no propagation yet) | disjoint-set / congruence classes / e-graph |
| `Provenance` + `DerivationDAG` (`kb/provenance`) | per-**derivation** justification (`source`/`rule`/`hypothesis`) — a fact is an OR-node over the ones recorded for it (`kb.justifications`); `absent_premises` records the queries that had to *fail* (recorded, not yet interpreted); the derivation AND/OR graph, source frontier | TMS justifications; database why-provenance; proof terms |
| `CanonicalSetId` (`apriori`) | a sorted tuple of FactIds = one **commitment set** | a CSP partial assignment / an ATMS environment / an itemset |
| no-good `Clause = frozenset[FactId]` (`nogoods`) | a learned "this combination is dead" clause, kept subsumption-minimal | CDCL conflict clause / CSP no-good |
| `SolutionRecord` / `DeadCommitment` (`monotonic/lattice`) | a recorded model / refutation with its `state_key` and core | model / unsat certificate |
| `Verdict` (`verdict`) | `Solution \| Ambiguity \| Contradiction` + optional `LatticeProof` | SAT/UNSAT/MULTIPLE + certificate |

---

## 4. The core operations

Strip away the puzzle vocabulary and the engine performs nine abstract
operations. The next two sections give each one its analog in another field
and the fast/optimal algorithm known for it.

- **O1 — Multi-way join / conjunctive pattern match.** Bind a rule body
  `(R ?a ?b) ∧ (S ?b ?c) ∧ …` against the KB. (`match._run_steps`.)
- **O2 — Forward-chaining saturation to a fixpoint.** Fire rules until no
  new fact. (`saturator`.)
- **O3 — Negation as failure.** `(absent P)` / `forall` premises — lifted out
  of the closure plan at compile time, judged at the closure/world boundary.
  (`compile.split_naf` → `NafGuard`, `world.World.absent`,
  `saturator._admit_from_boundary`.)
- **O4 — Equality / congruence.** Merge co-referent names. (`EqClasses`,
  `resolve_leaf` — currently a stub.)
- **O5 — Contradiction detection.** Find `(X, ¬X)` or `(false)`.
  (`contradiction`.)
- **O6 — Provenance & unsat-core.** Track every recorded derivation of each
  fact; search that AND/OR graph for the smallest source frontier of a clash.
  (`provenance`, `store.unsat_core`, `explain`, `frontier`.)
- **O7 — Hypothesis enumeration over a subset lattice.** Generate undecided
  candidates and the size-k commitment sets. (`hypgen`, `apriori`.)
- **O8 — Conflict-driven pruning.** Learn no-goods (Apriori downward-closure
  prune), cache one-step lookahead kills as `(not h)`, prune by that lookahead.
  (`nogoods`, `apriori`, `lookahead`, `hypgen`.)
- **O9 — Model canonicalisation / dedup.** Collapse equivalent models.
  (`canon.state_key`.)

---

## 5. Analogs in other fields

| op | this is, elsewhere | canonical names |
|---|---|---|
| **O1** join/match | relational join; conjunctive-query eval; production-system matching; subgraph homomorphism | RETE/TREAT/LEAPS; hash/sort-merge/index-NLJ; worst-case-optimal joins |
| **O2** saturation | Datalog bottom-up eval; transitive closure; chaotic-iteration fixpoint (abstract interpretation); forward chaining | naive vs **semi-naive** evaluation; magic sets; DRed; differential dataflow |
| **O3** NAF | stratified negation; closed-world assumption; default logic | stratified Datalog; well-founded & stable-model (ASP) semantics |
| **O4** equality | disjoint-set; congruence closure; term rewriting | union-find; Nelson-Oppen / Downey-Sethi-Tarjan; **e-graphs / equality saturation** |
| **O5** clash | constraint/clause violation; integrity-constraint check; tableau clash | unit-propagation conflict; watched literals |
| **O6** provenance | truth maintenance; database provenance; proof certificates | ATMS/JTMS justifications; provenance semirings; DRUP/DRAT, MUS |
| **O7** branch/lattice | CSP value enumeration; SAT decisions; **frequent-itemset mining**; version spaces; ATMS environments | DPLL decisions; **Apriori** candidate-gen; minimal hitting sets |
| **O8** learn/prune | conflict-clause learning; constraint propagation; consistency | **CDCL**; conflict-directed backjumping; AC-3/MAC; (singleton) arc consistency / forward checking |
| **O9** canonicalise | symmetry breaking; state canonicalisation; graph canon | order-insensitive hashing; SBDS/SBDD; nauty |

The single most useful reframing: **the deductive layer is a Datalog
engine, and the search layer is an ATMS-style environment search with
Apriori candidate generation and nogood learning** — commitment sets are
assumption environments explored breadth-first by cardinality, a dead
environment is learned whole as a no-good clause (kept
subsumption-minimal), and Apriori's downward-closure filter suppresses
its supersets. **CDCL/CSP is the analog** (no-good ≈ conflict clause /
CSP no-good) **and an optimization direction**
([P1.9 E-catalog](../../../plans/followups/f9_e_catalog/README.md)),
not the mechanism.
Two idiosyncrasies stand out against that backdrop, both in O7: Ein
branches on **sets of commitments enumerated by cardinality (Apriori)**
rather than one decision variable at a time (DPLL), and it keeps **explicit
assumption environments + provenance** (ATMS) rather than a single trail.
These make gaps/contradictions fall out generically, at the cost of the
mature machinery (watched literals, VSIDS, backjumping) that per-variable
DPLL/CDCL enjoys.

---

## 6. Fast / optimal known algorithms, and where Ein sits

### O1 — Multi-way join (the matcher)

**SOTA.** For a single conjunctive query, **worst-case-optimal join**
algorithms — *Leapfrog Triejoin* (Veldhuizen 2014) and *Generic Join* /
NPRR (Ngo–Porat–Ré–Rudra 2012) — run in the AGM bound, provably beating any
binary-join plan on cyclic queries. For *incremental, repeated* matching
against a slowly-changing store (production systems), **RETE** (Forgy 1982)
persists two kinds of state: **alpha-memories** (per-pattern filtered facts)
and **beta-memories** (materialised partial joins, reused across firings);
**TREAT** (Miranker 1987) keeps alpha-memories but *recomputes* the joins
(cheaper memory, more recompute); **LEAPS** is lazy.

**Ein today.** Left-deep binary joins via recursive `_run_steps` over
`Scan`/`Join`. The **participation index** `_facts_by_rel_slot_val`
(S1.8.B-idx) is exactly a **RETE alpha-memory / join index** — it narrows a
bound Scan/Join to candidates by `(relation, slot, value)`. The S1.8.B2v
delta-driven enqueue + `run_seeded` is **TREAT-like**: it keeps no
beta-memories, but re-joins only the *delta* and seeds at the new fact
(semi-naive join). **Gap:** no persisted beta-memories (the partial-join
products are recomputed every relevant firing — the thing RETE would cache),
and no worst-case-optimal join (binary plans only). Beta-memories are the
natural next step beyond D5; WCOJ matters only if cyclic joins appear.

### O2 — Saturation (the fixpoint loop)

**SOTA.** **Semi-naive (incremental) evaluation** is the textbook win: in
each round, join only with the *delta* (newly derived tuples) of the prior
round, never the full relation. **Magic sets** rewrite a program for
goal-directed bottom-up evaluation (sideways information passing). **DRed**
(Delete-Rederive) maintains a materialised fixpoint under deletion.
**Differential dataflow** (Naiad/DDlog) generalises semi-naive to
incremental + iterative dataflows. Production engines (Soufflé) compile to
semi-naive loops over specialised index structures (Brie, B-tree).

**Ein today.** A **priority-banded** saturator (rules fire in priority
order — a scheduling refinement over pure rounds). It was *naive* (re-match
everything each pass); **S1.8.B2v D2** made the within-run re-enqueue
delta-driven (semi-naive at the *which-plans* granularity), and **D5** made
the delta application semi-naive at the *where-in-the-plan* granularity (seed
from the new fact). Measured ~3.6× over naive. S1.21.8 then made the loop
**two-phase** — a purely positive closure run to quiescence, then one boundary
admission, repeat (§O3) — which is the stratum-at-a-time shape of stratified
Datalog evaluation applied to one saturation, and it came out *faster*:
dropping the absent-flip full-match split more than pays for the boundary
evaluations that replace it (an exhaustive `zebra2` solve ~10.4 s → ~8.5 s;
the acceptance gate 130 s → 91 s). **Gap:** no magic-sets / goal-direction
(it's fully bottom-up; the `hypgen` + lookahead layer is the
goal-direction substitute), no DRed (the append-only design means deletion
never happens *within* a saturation — retraction is modelled by forking a
fresh KB instead).

### O3 — Negation as failure

**SOTA.** **Stratified** Datalog evaluates negation stratum by stratum
(each `¬P` fully decided before it is read). Beyond stratification,
**well-founded semantics** (Van Gelder–Ross–Schlipf) gives a polynomial
3-valued model and **stable-model semantics** (Gelfond–Lifschitz) underlies
**Answer-Set Programming** (clingo/clasp, DLV) — the production answer to
non-monotone rules.

**Ein today — one evaluation point, at an explicit boundary (S1.21.8).**
`(absent P)` still compiles to an `AbsentGuard`, but `compile.split_naf`
**lifts** every top-level one out of the plan, so what the closure runs is a
purely positive program and a match says nothing about negation. The lifted
`NafGuard`s hang off `JoinPlan.naf_guards`, one tuple per disjunct; a match
whose disjunct carries them is parked rather than enqueued. At closure
quiescence the saturator builds a `World` over the stalled KB and asks it:
`World.absent` runs the guard's sub-plan under the bindings *projected to*
`NafGuard.scope` — the vars bound by the premises that **preceded** the guard,
which is what makes lifting exactly as strong as evaluating in place
(`(and (absent (P ?x)) (Q ?x))` still means "no `P` at all";
`(and (Q ?x) (absent (P ?x)))` still means "no `P` for this `x`"). One passing
candidate is admitted per round — `naf_rounds` / `naf_admitted` /
`naf_retired` are that loop's observables, "retired" being a candidate whose
*anti-monotone* guard has found a match and so can never pass again. `forall`
still desugars to a nested absent, and that nesting is *not* lifted — it
belongs to the negative query, which the boundary evaluates as one unit.

Three consequences, all load-bearing:

- **Nothing to re-check.** `match.absents_still_pass` (the fire-time re-check
  that closed the enqueue-vs-fire race, S1.5a.1) and
  `saturator._absent_relations` (the absent-flip full-match split, S1.8.B2v)
  are **deleted, not bypassed**. Admission is one-at-a-time into an empty
  queue, so the admitted candidate fires against precisely the world its
  guard was judged against — no window in which a verdict can go stale, and
  `Saturator.naf_dropped` is structurally 0. The `forall` false→true flip the
  full-match split existed for is caught instead by re-judging parked
  candidates at each quiescence, gated on the extent sizes of
  `NafGuard.watched`: cheaper than a full re-match *and* strictly more
  complete (it also catches a flip with no delta in the watched relation).
- **Order-independence on stratified programs.** A guard is judged against a
  positive fixpoint, so `W ⊭ ∃x̄.Pθ` is literal rather than approximated by
  fire-time re-checking, and *what is derivable no longer depends on rule
  priority*. The Q41 priority bands drop from load-bearing to **advisory**
  scheduling.
- **Negative dependence is recorded.** An admitted firing writes the queries
  that had to fail into `Provenance.absent_premises` (§O6) — recorded, not
  yet interpreted.

**The non-monotonicity still lives in the search layer, not the deductive
one**: retraction = "this assumption led to ⊥, fork without it." That makes
the *whole* system an ATMS/default reasoner rather than a stratified-Datalog
one — but the deductive layer now *evaluates* negation the way stratified
Datalog does, one closure at a time. **Gap:** no well-founded/stable-model
machinery and **no stratification checker**. A genuinely non-stratified rule
set (`p ← absent q; q ← absent p`) is still answered by operational order —
now boundary-admission order rather than priority-then-FIFO — and the engine
reports one model where several exist without saying so;
`naf_deps.DerivedNafWarning` (advisory, `SolverConfig.warn_derived_naf`,
default off) flags the shape that can cause it, re-grounded by S1.21.8 from a
soundness warning to a stratification one. The **normative semantics**
(worlds, the `W ⊭ ∃x̄.Pθ` definition, the single evaluation point E1 with
E2 retired, corollaries C1–C7 with C4/C5 retired and C2 re-grounded, and the
explicit non-guarantees this Gap gestures at) is
[`absent_semantics.md`](absent_semantics.md) (P1.21 R4, re-grounded by
S1.21.8).

### O4 — Equality / congruence

**SOTA.** **Union-find** with path compression + union-by-rank is
near-O(α(n)) (Tarjan) — the disjoint-set baseline. **Congruence closure**
(Nelson–Oppen; Downey–Sethi–Tarjan O(n log n)) extends equality through
function application and is the heart of SMT equality reasoning. **E-graphs +
equality saturation** (egg, Willsey et al. 2021) maintain *all* equivalent
forms compactly and are the modern engine for rewrite-driven optimisation.

**Ein today.** A **union-find stub** (`EqClasses`) wired into the API so
`firing` can call `kb.classes.union` — but with **no propagation** yet (the
glossary reserves *e-graph promotion* for F4). Equality in the puzzles is
currently carried as ordinary relation facts, not congruence. **Gap:** the
whole of O4 — if equality reasoning ever becomes load-bearing, the path is
union-find → congruence closure → e-graph, in that order of ambition.

### O5 — Contradiction detection

**SOTA.** In SAT this is implicit: unit propagation falsifies a clause, and
**2-watched-literals** make it cost-free until a watched literal flips. In
CSP it is constraint-violation checking under propagation.

**Ein today.** An explicit scan for `(X, ¬X)` pairs (using
the O(1) `_negated_facts` index) plus the rule-asserted `(false)` sentinel
(`contradiction.py`). Measured ~0 s — not a bottleneck, so the watched-literal
machinery would be premature. **Adequate as-is.**

### O6 — Provenance & unsat-core

**SOTA.** **Truth-maintenance systems** (de Kleer's ATMS, Doyle's JTMS)
record per-belief justifications and propagate (in)validity. Database
**provenance semirings** (Green–Karvounarakis–Tannen) give a unifying algebra
for why/how-provenance. In SAT/SMT, **resolution proofs** (DRUP/DRAT) certify
UNSAT and **MUS** extraction finds a minimal unsatisfiable subset.

**Ein today.** Provenance is per **derivation**, not per fact:
`Fact.provenance` is the primary justification and `kb.justifications(fact)`
returns every recorded one, so a fact is an OR-node over AND-nodes — the proof
structure is an AND/OR graph, a faithful ATMS justification network. A
re-derivation is appended to a KB side table by `store.record_justification`
(capped per fact at `MAX_ALT_JUSTIFICATIONS = 32`, kept sorted by premise count
so the cap retains the *shortest*; terminals take none — a `source`/`hypothesis`
primary is the frontier, and a rule-kind primary with empty `premises_raw` is a
synthetic engine writeback whose contract is that provenance grounds out on it,
[`reserved_engine_strings.md`](reserved_engine_strings.md)). Recording is gated
by `SolverConfig.record_alternative_justifications` (default on; measured +2.5 %
median on an exhaustive `zebra2` solve). S1.21.8 added the *negative* half: a
firing admitted at the NAF boundary (§O3) records the `(absent …)` queries
that had to fail in `Provenance.absent_premises` — one `(relation, args)`
pattern each, `None` where the query ranged free — so `Deps(Y)`, the union of
`PositiveDeps(Y)` and `NegativeDeps(Y)` (REVIEW_M1-01 §2), is finally
*representable*. It is recorded, **not interpreted**: `unsat_core`,
`explain`, `frontier` and the trace's "using" line all still read positive
premises only.

Over that graph, `explain.py`'s `explain()` /
`minimal_contradiction_frontier()` run ATMS-style **label propagation** — a
least fixpoint from the frontier upward, which is what makes the routinely
*cyclic* engine-recorded provenance (symmetric/transitive closure) safe by
construction, and why `detect_provenance_cycles` stays a load-time check on
user-authored provenance rather than something run over a saturated KB.
`frontier.smallest_contradiction_frontier` is that search over the detector's
witnesses: **a minimum-cardinality AND/OR search over every recorded derivation
(provenance-based, NAF-safe, budgeted); not a subset-minimal MUS** — and,
because it *chooses* a justification per fact by search instead of following
whichever one fired first, **independent of rule-firing order**. It is what the
k = 0 verdict and each dead commitment report: on `zebra2-bad` it names exactly
1 fact, the injected `:source "injected contradiction"`, where the union core
names 38. (The verdict still *unions* across dead commitments — with an
exhausted lattice no single dead explains unsat — but each dead's core is the
smallest explanation of that dead.) `store.unsat_core` — the *union* source
frontier of a clash (the given facts that jointly force it, collected by a
`walk_premises` closure walk) — and `store.derivation_dag` stay
**primary-justification only** by default and deliberately so: unioning over
alternatives makes a core monotonically *larger*, the opposite of a legible
explanation; their `all_justifications=True` opt-in gives that union as a
soundness envelope (every explanation is a subset of it), and
`DerivationDAG.and_nodes` / `is_or_graph` carry the conjunction structure a
flat edge set cannot express.

**Gap:** minimality here is bounded three ways, and every claim of it must say
so. (1) **Not a subset-minimal MUS** — no proper subset is checked for
satisfiability (the caveat flagged in P1.7a), and the textbook deletion-based
minimiser is **NAF-unsound here** (S1.9.E19; corollary C3 of
[`absent_semantics.md`](absent_semantics.md) — the recorded
`absent_premises` make a *sound* variant conceivable, since a candidate
subset would have to preserve every negative query as well, but nothing
implements that and until something does the caveat stands).
(2) **Recorded, not all, derivations** — the alternatives searched are the
firings the saturator attempted, capped per fact, so minimality stays relative
to the rule set and the saturation strategy. (3) **Budgeted** — the
minimum-axiom-set / ATMS-label problem is worst-case exponential, so the search
runs under an `ExplanationBudget` and `Explanation.exhausted` reports whether it
completed; a truncated search is still sound. Provenance is also not yet a
semiring (no multiplicity/why-vs-how distinction, which M2-scale work might
want) — though the AND/OR structure such an algebra would be interpreted over
now exists. The plan record for the multi-justification machinery is
S1.21.7.

### O7 — Hypothesis enumeration over a subset lattice

**SOTA.** Mainstream solvers do **not** enumerate the subset lattice — DPLL
picks *one* decision variable, propagates, and branches binary, which with
CDCL learning is exponentially stronger than naive enumeration. Where the
*set* structure genuinely matters (minimal diagnoses, ATMS environments,
minimal hitting sets), the canonical algorithm for generating size-k
candidates with downward-closed pruning **is Apriori** (Agrawal–Srikant 1994,
frequent-itemset mining) — the prefix-join + "every subset of a frequent set
is frequent" pruning.

**Ein today.** This is the most non-standard part, and named honestly:
`apriori.py` does **literal Apriori** — `apriori_prefix_join` builds size-k
commitment sets from size-(k-1) ones sharing a (k-1)-prefix, and
`filter_candidate` prunes any candidate that is a superset of a learned
no-good (the downward-closure principle: a superset of a dead set is dead).
`hypgen` generates the layer-1 atoms (undecided `(relation, slot, value)`
candidates, ordered by fact-participation). **Gap / trade-off:** branching by
*cardinality* over sets is worst-case `O(2^|alive|)` and forgoes per-variable
CDCL's strength (VSIDS activity, 1UIP learning, **non-chronological
backjumping** — Ein does *plain BFS backtracking*, no backjump). The
payoff is that gaps (k>1 models) and contradictions (union of dead cores)
read off the same lattice generically. A DPLL/CDCL re-architecture is the big
lever if search ever dominates (it currently does not — saturation does).

### O8 — Conflict-driven pruning

**SOTA.** **CDCL** is the modern SAT core: 1UIP **conflict-clause learning**,
**non-chronological backjumping**, **2-watched-literals**, the **VSIDS**
activity heuristic, restarts, and clause-DB minimisation. CSP adds
**conflict-directed backjumping** (Prosser), **dynamic backtracking**
(Ginsberg), **MAC** (maintain arc consistency), **forward checking**, and
**singleton arc consistency** (a one-variable lookahead).

**Ein today.** A creditable CDCL-*flavoured* set: `nogoods.py` learns
subsumption-**minimal** conflict clauses and prunes by the subset test
(O7's Apriori filter); `lookahead.py` is a **one-step (singleton-consistency /
forward-checking) lookahead** that kills candidates which would die in one
firing before paying for a fork+saturate, caching each kill as a learned unit
`(not h)` (`hypgen._write_negated`, gated by `enable_lookahead_kill_cache` —
≈ a unit clause + unit propagation); the **forced-positive cascade** —
promoting a singleton-alive hypothesis to root and re-saturating
(`_helpers._promote_forced_positives`) — is its positive dual. The lookahead
is the search layer's only direct reader of a rule's `(absent …)` guards, and
S1.21.8 fixed the world it read them in (divergence D3): a guard
must hold in the world **with** `h` — no match in `kb` *and* `h` creating
none — and a guard whose verdict that test cannot decide (one with a nested
absent, non-monotone in the KB) makes the lookahead skip the disjunct rather
than guess, which only loses a kill and so keeps the "never reports a live
hypothesis as dead" contract. **Gap:** no backjumping (plain BFS), no VSIDS-style
activity ordering (there is a `score_hypothesis` hook, S1.5a.7, mostly a
stub), no watched-literals. These are exactly the pieces a DPLL/CDCL
re-architecture (O7) would bring.

### O9 — Model canonicalisation

**SOTA.** Order-insensitive **canonical hashing** for memoised dedup;
**symmetry breaking** (static SBP predicates; dynamic SBDS/SBDD) to avoid
exploring symmetric models; **graph canonicalisation** (nauty/bliss) for full
structural symmetry.

**Ein today.** `canon.state_key` canonicalises the propositional fact set
order-insensitively — the sorted, provenance-free `(relation, args)` tuple; the
representation *itself* is the identity (P1.21 R1 — any hash of it is a
display digest, never identity), so distinct branches that reach the same
model collapse to one solution node — a lightweight canonicalisation that
the S1.7.24 symmetric-removal made fully generic (no hard-coded symmetry). **Adequate** for Zebra-scale; full symmetry breaking
(nauty-style) would only matter for far larger or highly-symmetric instances.

---

## 7. Summary — where the bodies are, and the levers

The measured cost (P1.8a) is **almost entirely O1+O2** — the matcher inside
saturation (~95 % of a solve). The optimisation arc has been a walk *up the
Datalog ladder*: naive → **semi-naive** (participation index = alpha-memory;
D2 delta-driven; D5 seeded delta join), for ~3.6×. The remaining named
levers map onto the literature precisely:

- **RETE beta-memories** — persist partial joins across firings (the one
  thing D5 still recomputes). The natural successor to D5 for O1.
- **Worst-case-optimal joins** — only if cyclic join patterns appear (they
  don't yet).
- **DPLL/CDCL re-architecture of O7/O8** — watched literals, VSIDS,
  non-chronological backjumping. The big structural change; deferred because
  search is not the bottleneck (saturation is). The recorded forward
  pointers are
  [P1.9 E20](../../../plans/followups/f9_e_catalog/s1.9.e20_conflict_cache.md)
  (conflict-cache cross-call ≈ incremental SAT) and
  [E23](../../../plans/followups/f9_e_catalog/s1.9.e23_prove_speedup.md)
  (exhaustive-search speedup umbrella).
- **Congruence closure / e-graph (O4)** — only when equality reasoning earns
  its keep (F4).
- **Static stratification checking (O3)** — the boundary makes *stratified*
  programs order-independent, but nothing yet tells an author their rule set
  is not one; `naf_deps` is the advisory proxy in the meantime.

The two-layer split also names the engine's **soundness story** cleanly: the
deductive layer is monotone (a least fixpoint, trivially sound), and *all*
retraction-shaped non-monotonicity is quarantined in the search layer as
assumption-and-retract — which is why "a correct engine never exhausts a SAT
problem to ⊥ and never calls a non-model a model" (P1.7a) is checkable as a
property of the lattice, not of any single rule firing. S1.21.8 sharpened the
deductive half: the *closure* is the least fixpoint, purely positive, and the
one non-monotone question it has to answer — `(absent …)` — is asked only at
the boundary between two closures, against a saturated world rather than a
half-built KB (§O3).

---

## 8. References (algorithms named above)

*Joins / matching (O1)*

- Forgy, *Rete* (1982)
- Miranker, *TREAT* (1987)
- Atserias–Grohe–Marx, the *AGM bound* (2008)
- Ngo–Porat–Ré–Rudra, *worst-case-optimal join* / Generic Join (2012)
- Veldhuizen, *Leapfrog Triejoin* (2014)

*Datalog / fixpoint (O2)*

- Bancilhon, *semi-naive evaluation* (1985)
- Bancilhon–Maier–Sagiv–Ullman, *Magic Sets* (1986)
- Gupta–Mumick–Subrahmanian, *DRed* (1993)
- McSherry–Murray–Isaacs–Isard, *Differential Dataflow* (2013)
- Jordan–Scholz–Subotić, *Soufflé* (2016)

*Negation (O3)*

- Gelfond–Lifschitz, *stable models / ASP* (1988)
- Van Gelder–Ross–Schlipf, *well-founded semantics* (1991)
- Gebser–Kaminski–Kaufmann–Schaub, *clingo* (2014)

*Equality (O4)*

- Tarjan, *union-find* (path compression + union by rank) (1975)
- Downey–Sethi–Tarjan, *congruence closure* (1980)
- Nelson–Oppen, *congruence closure decision procedure* (1980)
- Willsey–Nandi–Wang–Flatt–Tatlock–Panchekha, *egg / equality saturation* (2021)

*SAT / CSP search (O7, O8)*

- Davis–Putnam–Logemann–Loveland, *DPLL* (1962)
- Prosser, *conflict-directed backjumping* (1993)
- Ginsberg, *dynamic backtracking* (1993)
- Marques-Silva–Sakallah, *GRASP / CDCL* (1999)
- Moskewicz–Madigan–Zhao–Zhang–Malik, *Chaff / watched literals / VSIDS* (2001)

*Itemsets / provenance / TMS (O6, O7)*

- de Kleer, *ATMS* (1986)
- Agrawal–Srikant, *Apriori* (1994)
- Green–Karvounarakis–Tannen, *provenance semirings* (2007)

## Cross-links

- Data model the engine reads/writes:
  [`../ir/02-data-model/02_store.md`](../ir/02-data-model/02_store.md),
  [`01_entities.md`](../ir/02-data-model/01_entities.md).
- Rule semantics: [`../ir/01-ein-graph/02_rules.md`](../ir/01-ein-graph/02_rules.md).
- Planned how-to chapters: [`README.md`](README.md) (`01_matcher` … `05_trace`).
- The optimisation work this analysis frames:
  P1.8a —
  the participation index (S1.8.B-idx) and semi-naive saturation (S1.8.B2v).
- The soundness model: P1.7a.
