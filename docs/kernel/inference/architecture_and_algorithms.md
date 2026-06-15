# Inference engine — architecture, operations, and their CS analogs

> A cross-cutting analysis of the reasoning engine: its architecture and
> main steps, the abstract **operations** it performs, those operations'
> **analogs** in other fields of computer science, and the **fast / optimal
> known algorithms** for each. It is a map from this puzzle reasoner to the
> broader literature — written to orient optimization work
> ([P1.8a](../../../plans/m1_core_graph_reasoning/p1.8a_performance/),
> the [M1a Rust port](../../../plans/m1a_rust/)) and to make the engine's
> design choices legible against the state of the art.
>
> Source of truth for the code: `ein.py/src/ein_bot/inference/` (engine),
> `ein.py/src/ein_bot/kb/` (data model). For the *planned* how-to chapters
> (`01_matcher.md` … `05_trace.md`) see [`README.md`](README.md); this file
> is the architecture + algorithms overview those chapters sit under.

---

## 1. What the engine solves, and the three paradigms it fuses

ein-bot answers **finite-domain constraint-satisfaction** questions over a
[typed hypergraph](../ir/01-ein-graph/01_kb.md) of facts and
[rules](../ir/01-ein-graph/02_rules.md) — the Zebra puzzle and its kin. Per
[idea 03](../../../docs/ideas/03-three-task-classes.md) it answers three
shapes of one question, all read off a single search (P1.7a):

- **solve** — is there a unique complete model? (k = 1)
- **gaps** — which cells are forced vs contingent? (k > 1, the residual)
- **contradictions** — a minimal unsat core for an over-constrained KB. (k = 0)

The engine is the confluence of **three classical paradigms**, and almost
every component below is recognisable as a piece of one of them:

| paradigm | what ein-bot borrows | classic systems |
|---|---|---|
| **Deductive database** (Datalog) | bottom-up forward chaining to a least fixpoint; stratified negation | Soufflé, LogicBlox, DDlog, Datomic |
| **CSP / SAT solver** | branch on undecided choices, propagate, detect clashes, learn no-goods | DPLL/CDCL (MiniSat, Chaff), CSP (Gecode), ASP (clingo) |
| **Truth-maintenance system** (ATMS) | hypotheses as *assumptions*, no-goods, provenance/justifications, retract-on-contradiction | de Kleer's ATMS, JTMS |

The implementation splits cleanly into two layers along the
monotone/non-monotone seam:

- **Deductive layer (monotone, append-only).** Saturate: fire rules to
  quiescence, never retract. `saturator.py`, `match.py`, `compile.py`,
  `engine.py`, `firing.py`, `contradiction.py`, `resolve.py`,
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
        ▼                         (Scan/Join/Guard/AbsentGuard)
  ┌─────────────────────────────────────────────────────────┐
  │  DEDUCTIVE LAYER  (monotone, per KB)                      │   ── inference ──
  │   Saturator.saturate()  ── forward-chain to fixpoint ──   │
  │     match (multi-way join)  → fire  → append fact         │
  │     ▲ priority queue, delta-driven (semi-naive, S1.8.B2v) │
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
  │   verdict_of(k deduped solution nodes by state_hash)      │
  └─────────────────────────────────────────────────────────┘
        ▼
  Verdict = Solution(k=1) | Ambiguity(k>1) | Contradiction(k=0)
```

**The deductive inner loop** (one KB → fixpoint). `Engine.compile_all`
compiles every `(rule, activator)` pair into a `JoinPlan` — a sequence of
`Scan`/`Join`/`Guard`/`AbsentGuard` opcodes (`compile.py`). The
`Saturator` runs a priority queue of enqueued `(plan, binding)` firings; each
`step()` pops the highest-priority unfired binding, applies it (`firing.fire`
→ append a reasoning-layer `Fact` with provenance), and re-enqueues the
matches the new fact enables. It halts at quiescence (no new firing). Since
S1.8.B2v the re-enqueue is **delta-driven**: only plans whose premises touch
the just-derived fact's relation are re-matched, and they are **seeded** at
the new fact rather than re-scanned (`saturator._enqueue_pass`,
`match.run_seeded`).

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
node, deduped by `state_hash`. An alive branch also merges its *unconditional*
consequences (`commitment._is_unconditional`) into the root. The verdict is read off
the deduped count k (`verdict_of`).

---

## 3. Data types

| type (`module`) | what it is | analog |
|---|---|---|
| `Fact` (`kb/entities`) | `(relation_name, args)` identity; `args ∈ str \| int \| Fact` (nested = a relational node); carries `layer` + `provenance` | a ground atom / tuple / labelled hyperedge |
| `Relation` (`kb/entities`) | a named relation + `signature` (type atoms) | a database relation schema / predicate symbol |
| `Rule` (`kb/entities`) | `params`, `match` pattern, `assert`, `priority`, activator | a Datalog/production rule (Horn-ish clause) |
| `JoinPlan` + `Scan`/`Join`/`Guard`/`AbsentGuard` (`compile`) | a rule's `:match` compiled to a join program over relations | a query plan / RETE network / WAM-ish opcode list |
| 7 KB indexes (`kb/store`) | `_facts_by_relation`, **`_facts_by_rel_slot_val`** (the participation index, `(rel,slot,val)→facts`), `_negated_facts`, `_rule_apps_*`, `names`, … | database join indexes; RETE alpha-memories |
| `EqClasses` (`kb/store`) | union-find over names (a *placeholder* — no propagation yet) | disjoint-set / congruence classes / e-graph |
| `Provenance` + `DerivationDAG` (`kb/provenance`) | per-fact justification (`source`/`rule`/`hypothesis`), the derivation graph, source frontier | TMS justifications; database why-provenance; proof terms |
| `CanonicalSetId` (`apriori`) | a sorted tuple of FactIds = one **commitment set** | a CSP partial assignment / an ATMS environment / an itemset |
| no-good `Clause = frozenset[FactId]` (`nogoods`) | a learned "this combination is dead" clause, kept subsumption-minimal | CDCL conflict clause / CSP no-good |
| `SolutionRecord` / `DeadCommitment` (`monotonic/lattice`) | a recorded model / refutation with its `state_hash` and core | model / unsat certificate |
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
- **O3 — Negation as failure.** `(absent P)` / `forall` premises.
  (`match.absents_still_pass`, `AbsentGuard`.)
- **O4 — Equality / congruence.** Merge co-referent names. (`EqClasses`,
  `resolve_leaf` — currently a stub.)
- **O5 — Contradiction detection.** Find `(X, ¬X)` or `(false)`.
  (`contradiction`.)
- **O6 — Provenance & unsat-core.** Track why each fact holds; extract the
  source frontier of a clash. (`provenance`, `store.unsat_core`.)
- **O7 — Hypothesis enumeration over a subset lattice.** Generate undecided
  candidates and the size-k commitment sets. (`hypgen`, `apriori`.)
- **O8 — Conflict-driven pruning.** Learn no-goods (Apriori downward-closure
  prune), cache one-step lookahead kills as `(not h)`, prune by that lookahead.
  (`nogoods`, `apriori`, `lookahead`, `hypgen`.)
- **O9 — Model canonicalisation / dedup.** Collapse equivalent models.
  (`canon.state_hash`.)

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
engine, and the search layer is a CDCL/CSP solver with an ATMS underneath.**
Two idiosyncrasies stand out against that backdrop, both in O7: ein-bot
branches on **sets of commitments enumerated by cardinality (Apriori)**
rather than one decision variable at a time (DPLL), and it keeps **explicit
assumption environments + provenance** (ATMS) rather than a single trail.
These make gaps/contradictions fall out generically, at the cost of the
mature machinery (watched literals, VSIDS, backjumping) that per-variable
DPLL/CDCL enjoys.

---

## 6. Fast / optimal known algorithms, and where ein-bot sits

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

**ein-bot today.** Left-deep binary joins via recursive `_run_steps` over
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

**ein-bot today.** A **priority-banded** saturator (rules fire in priority
order — a scheduling refinement over pure rounds). It was *naive* (re-match
everything each pass); **S1.8.B2v D2** made the within-run re-enqueue
delta-driven (semi-naive at the *which-plans* granularity), and **D5** made
the delta application semi-naive at the *where-in-the-plan* granularity (seed
from the new fact). Measured ~3.6× over naive. **Gap:** no magic-sets /
goal-direction (it's fully bottom-up; the `hypgen` + lookahead layer is the
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

**ein-bot today.** `(absent P)` compiles to an `AbsentGuard`, re-checked at
**fire time** (`match.absents_still_pass`) to close the enqueue-vs-fire race
(S1.5a.1); `forall` desugars to a nested absent. Within one saturation the
KB is append-only, so a once-true absent can only *flip to false* (more
facts) — which the saturator handles by full-matching absent-premise plans
(the S1.8.B2v "absent-flip" case). **The non-monotonicity lives in the
search layer, not the deductive one**: retraction = "this assumption led to
⊥, fork without it." That makes the *whole* system an ATMS/default reasoner
rather than a stratified-Datalog one. **Gap:** no well-founded/stable-model
machinery — sound because the puzzle ruleset is effectively stratified +
the hypothesis layer is monotone-per-branch.

### O4 — Equality / congruence

**SOTA.** **Union-find** with path compression + union-by-rank is
near-O(α(n)) (Tarjan) — the disjoint-set baseline. **Congruence closure**
(Nelson–Oppen; Downey–Sethi–Tarjan O(n log n)) extends equality through
function application and is the heart of SMT equality reasoning. **E-graphs +
equality saturation** (egg, Willsey et al. 2021) maintain *all* equivalent
forms compactly and are the modern engine for rewrite-driven optimisation.

**ein-bot today.** A **union-find stub** (`EqClasses`) wired into the API so
`firing` can call `kb.classes.union` — but with **no propagation** yet (the
glossary reserves *e-graph promotion* for F4). Equality in the puzzles is
currently carried as ordinary relation facts, not congruence. **Gap:** the
whole of O4 — if equality reasoning ever becomes load-bearing, the path is
union-find → congruence closure → e-graph, in that order of ambition.

### O5 — Contradiction detection

**SOTA.** In SAT this is implicit: unit propagation falsifies a clause, and
**2-watched-literals** make it cost-free until a watched literal flips. In
CSP it is constraint-violation checking under propagation.

**ein-bot today.** An explicit scan for same-layer `(X, ¬X)` pairs (using
the O(1) `_negated_facts` index) plus the rule-asserted `(false)` sentinel
(`contradiction.py`). Measured ~0 s — not a bottleneck, so the watched-literal
machinery would be premature. **Adequate as-is.**

### O6 — Provenance & unsat-core

**SOTA.** **Truth-maintenance systems** (de Kleer's ATMS, Doyle's JTMS)
record per-belief justifications and propagate (in)validity. Database
**provenance semirings** (Green–Karvounarakis–Tannen) give a unifying algebra
for why/how-provenance. In SAT/SMT, **resolution proofs** (DRUP/DRAT) certify
UNSAT and **MUS** extraction finds a minimal unsatisfiable subset.

**ein-bot today.** Every derived `Fact` carries a `Provenance`
(`source`/`rule`/`hypothesis`) with `premises_raw`; `DerivationDAG` walks it,
and `store.unsat_core` returns the **source frontier** of a clash (the given
facts that jointly force it) via a `reaches` DFS. This is a faithful
ATMS-style justification graph. **Gap:** the unsat-core is the source
frontier, **not a minimal MUS** (flagged in P1.7a) — minimisation (e.g.
deletion-based MUS) is a follow-up; provenance is not yet a semiring (no
multiplicity/why-vs-how distinction, which M2-scale work might want).

### O7 — Hypothesis enumeration over a subset lattice

**SOTA.** Mainstream solvers do **not** enumerate the subset lattice — DPLL
picks *one* decision variable, propagates, and branches binary, which with
CDCL learning is exponentially stronger than naive enumeration. Where the
*set* structure genuinely matters (minimal diagnoses, ATMS environments,
minimal hitting sets), the canonical algorithm for generating size-k
candidates with downward-closed pruning **is Apriori** (Agrawal–Srikant 1994,
frequent-itemset mining) — the prefix-join + "every subset of a frequent set
is frequent" pruning.

**ein-bot today.** This is the most non-standard part, and named honestly:
`apriori.py` does **literal Apriori** — `apriori_prefix_join` builds size-k
commitment sets from size-(k-1) ones sharing a (k-1)-prefix, and
`filter_candidate` prunes any candidate that is a superset of a learned
no-good (the downward-closure principle: a superset of a dead set is dead).
`hypgen` generates the layer-1 atoms (undecided `(relation, slot, value)`
candidates, ordered by fact-participation). **Gap / trade-off:** branching by
*cardinality* over sets is worst-case `O(2^|alive|)` and forgoes per-variable
CDCL's strength (VSIDS activity, 1UIP learning, **non-chronological
backjumping** — ein-bot does *plain BFS backtracking*, no backjump). The
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

**ein-bot today.** A creditable CDCL-*flavoured* set: `nogoods.py` learns
subsumption-**minimal** conflict clauses and prunes by the subset test
(O7's Apriori filter); `lookahead.py` is a **one-step (singleton-consistency /
forward-checking) lookahead** that kills candidates which would die in one
firing before paying for a fork+saturate, caching each kill as a learned unit
`(not h)` (`hypgen._write_negated`, gated by `enable_lookahead_kill_cache` —
≈ a unit clause + unit propagation); the **forced-positive cascade** —
merging an alive commitment's unconditional consequences
(`commitment._is_unconditional`) into the root — is its positive dual. **Gap:** no backjumping (plain BFS), no VSIDS-style
activity ordering (there is a `score_hypothesis` hook, S1.5a.7, mostly a
stub), no watched-literals. These are exactly the pieces a DPLL/CDCL
re-architecture (O7) would bring.

### O9 — Model canonicalisation

**SOTA.** Order-insensitive **canonical hashing** for memoised dedup;
**symmetry breaking** (static SBP predicates; dynamic SBDS/SBDD) to avoid
exploring symmetric models; **graph canonicalisation** (nauty/bliss) for full
structural symmetry.

**ein-bot today.** `canon.state_hash` hashes the propositional fact set
order-insensitively (excluding bookkeeping heads), so distinct branches that
reach the same model collapse to one solution node — a lightweight
canonicalisation that the S1.7.24 symmetric-removal made fully generic (no
hard-coded symmetry). **Adequate** for Zebra-scale; full symmetry breaking
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
  search is not the bottleneck (saturation is).
- **Congruence closure / e-graph (O4)** — only when equality reasoning earns
  its keep (F4).

The two-layer split also names the engine's **soundness story** cleanly: the
deductive layer is monotone (a least fixpoint, trivially sound), and *all*
non-monotonicity is quarantined in the search layer as
assumption-and-retract — which is why "a correct engine never exhausts a SAT
problem to ⊥ and never calls a non-model a model" (P1.7a) is checkable as a
property of the lattice, not of any single rule firing.

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
  [P1.8a](../../../plans/m1_core_graph_reasoning/p1.8a_performance/) —
  the participation index (S1.8.B-idx) and semi-naive saturation (S1.8.B2v).
- The soundness model: [P1.7a](../../../plans/m1_core_graph_reasoning/p1.7a_solution_search_refactor/).
