# Algorithms

Which algorithm `ein.rs` uses for each operation the engine performs, where that places it among the known ones,
the levers that change how it computes and never what, and then each algorithm as the code runs it: its flow as a
diagram, and pseudocode where the order of the steps is the point (§ 4 to § 10). The operations themselves, and the
paradigms they come from (forward chaining to a fixpoint, negation as failure, a search over assumptions), are
defined independently of this code; this page is the choices `ein.rs` makes for them, and each section links the
design page that is its contract.

## 1. The operations, and the choice for each

| operation | what `ein.rs` does | the family | where |
|---|---|---|---|
| **join**: every match of a rule's premises | a register machine over a fixed join order, with candidates narrowed by an index on argument positions (one level inside a nested argument) and a fully bound premise answered by one interned lookup | nested-loop join over alpha memories, in the RETE family, with no beta memories; not a worst-case-optimal join, whose reordering would change the firing order | [`03-design/05-matcher.md`](03-design/05-matcher.md) |
| **saturation**: the fixpoint | forward chaining, semi-naive: after a firing, only the delta is matched, each plan seeded at the premise a new fact plays; plans compiled once per run | the Datalog ladder: naive, then semi-naive with a seeded delta join | [`03-design/06-saturation.md`](03-design/06-saturation.md) |
| **negation as failure** | the boundary: a guard is judged only on a quiescent KB, one admission a round, a parked candidate judged again only where a relation it watches grew | stratification computed by the run rather than in advance; a refutation resting on an `absent` a commitment could extend is refused | [`03-design/06-saturation.md`](03-design/06-saturation.md), [`03-design/07-search.md`](03-design/07-search.md) |
| **equality** | none: `eq` is name identity, computed, never stored, and names are never merged ([Q1](../../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md)) | no congruence closure | [`03-design/03-data-model.md`](03-design/03-data-model.md) |
| **contradiction** | two bit tests per new fact, `(false)` and a pair `X`, `(not X)`; the agenda fires first what can make one, and a route's node stops at its first | incremental clash detection | [`03-design/06-saturation.md`](03-design/06-saturation.md) |
| **provenance and cores** | every derivation recorded as an OR over AND-nodes, alternatives kept by premise count and smallest closure; a core is the smallest frontier, by an ATMS label search with a budget | ATMS labels; a minimum-cardinality frontier rather than a subset-minimal unsatisfiable subset | [`03-design/03-data-model.md`](03-design/03-data-model.md), [`03-design/07-search.md`](03-design/07-search.md) |
| **hypotheses**: the candidates H | a ladder: the program's hrules, else what the obligations owe, else blind enumeration over the program's names by type | rule-driven, obligation-driven and generate-and-test generation | [`03-design/07-search.md`](03-design/07-search.md) |
| **the search**: the region `R` | routes over the lattice of commitments, each the least work its question needs: the state route (each alive state once), the obligation tree, the climb and the shrink, the alive down-set, the cube | monotone, upward-closed pruning (a dead node's supersets are dead), minimal transversals for the dead sets and the minimal generators, branching on an owed instance's exhaustive alternatives | [`03-design/07-search.md`](03-design/07-search.md) |
| **canonicalisation** | a state is its sorted fact set, compared whole, never by hash | canonical forms rather than isomorphism | [`03-design/07-search.md`](03-design/07-search.md) |

No problem is handed to an external solver: every operation above is the engine's own
([Q2](../../decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md)). Arithmetic is a theory in the
standard library, computed by these same operations.

## 2. What it is shaped against

The engine is dominated by the join inside saturation; the choices above are each a way of asking the join less. The
alpha index reaching inside a nested argument, the ground-premise lookup, the plan memo shared by every node, a node
resumed from root's snapshot rather than saturated from scratch, a dead node stopped at its first contradiction, and
the state route visiting states rather than commitments each remove work without changing a match, a firing order or
an answer. Each is held to that by the invariance tests ([`03-design/01-determinism-and-order.md`](03-design/01-determinism-and-order.md)).

## 3. The levers

A lever changes how the engine computes, never what it answers: **no configuration field, option or environment name
changes `R`** ([Q-M1e2.30](../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)),
held for every mode under every lever by `ein-cli/tests/lever_invariance.rs`.

| lever | what it changes |
|---|---|
| `--jobs N` | the nodes of a batch computed on `N` threads; nothing a run prints ([`03-design/08-parallelism.md`](03-design/08-parallelism.md)) |
| `--region-budget F` | how far a run goes before it stops and says what it established |
| `--region-limit N` | how large a flagged program's cube a run computes whole |
| `--describe-route tree\|states` | which of describe's two routes runs on a monotone program |
| `-T S` | a deadline, past which the run is an error |
| `--no-search` | the question: root alone, `R` not discovered |
| `(config :lattice-sanity-check true)`, `-y` | the parent-path spot check, read by the read-out |
| `(config :record-alternative-justifications false)` | alternatives not recorded; the one lever that may change a dead node's core, never `R` |

Every field the `(config …)` form accepts, every option and every `EIN_*` name is listed, with whether it changes the
answer, in the configuration reference (today [`docs/ein/user/04-reference/06-config.md`](../user/04-reference/06-config.md)); a
field that is retired is refused at load with the reason (`ein_core::config::RETIRED`).

## 4. Saturation: the two-phase loop and the agenda

`Saturator::saturate` brings one KB to its fixpoint, one firing a step. The closure is purely positive; negation
as failure is consulted only between two closures, on a quiescent KB, and admits at most one candidate each time.
The contract is [`03-design/06-saturation.md`](03-design/06-saturation.md).

```plantuml
@startuml
start
if (stopping at the first contradiction,\nand the KB written so far holds one?) then (yes)
  stop
endif
repeat
  :closure step;
  note right
    the enqueue pass where one is due,
    then the mirror, then the queue
  end note
  if (a firing?) then (yes)
    :hand it to the caller;
    if (stopping at the first contradiction,\nand a fact it derived makes one?) then (yes)
      :return, the KB not marked saturated;
      stop
    endif
  else (no: the closure is quiescent)
    :boundary round: walk the parked candidates in rank order,\nadmit the first whose guards all pass;
    if (one admitted?) then (yes)
      :it joins the queue;
    else (no)
      :mark the KB saturated;
      stop
    endif
  endif
repeat while (next step)
@enduml
```

**The closure step** (`Saturator::closure_step`) yields at most one firing:

```text
closure_step():
    if needs_enqueue:
        enqueue_pass(delta.take())          # None: a full pass
    if the mirror has a fact to swap:       # relations marked (__symmetric__ R)
        return the mirror's firing
    while the queue is not empty:
        entry = queue.pop()                 # least rank: the agenda's half, then the tiebreaker
        if entry.key in fired: continue
        fired.insert(entry.key)
        build every conclusion of the plan's :assert against entry's registers
        if none is built: continue          # no :assert, or only eq and neq that hold
        if every conclusion is in the KB:   # a redundant firing
            record the derivation as an alternative, where the fact accepts one
            return the firing, redundant
        write the new facts with one derivation record
        delta += the new facts; needs_enqueue = true
        return the firing
    return None                             # quiescent
```

**The enqueue pass** (`Saturator::enqueue_pass`) is where the closure is semi-naive. It first compiles a plan for
every activator the KB has gained (`Engine::compile_all`), then matches:

```text
enqueue_pass(delta):
    compile_all()                           # a derived activator gets its plan here
    if delta is None:                       # the first pass, or facts written outside step
        for plan in the engine's list: full_match(plan)
        return
    for plan never matched yet: full_match(plan)
    for fact in delta, in order:
        for plan in pos_index[relation of fact], in list order:
            seed_match(plan, fact)          # run_seeded: the fact bound at each step it can play

each match:
    key = (rule, activator, registers); guards = the disjunct's guard set
    if (key, guards) in seen: drop it
    seen.insert((key, guards)); if key in fired: drop it
    tiebreaker += 1
    rank = tiebreaker, with the bit LATE where the plan's rule is not in the agenda's first half
    the disjunct has guards ? parked.insert(rank, entry) : queue.push(rank, entry)
```

**The agenda** is the rank. The first half holds the candidates of a rule whose conclusion a contradiction can be
made of, the second the rest; within a half the order is the tiebreaker's, the order of arrival
([Q-M1f.313](../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)). The queue is a binary
heap over `(rank, entry)` and the parked set a `BTreeSet` of the same keys, so the closure pops, and the boundary
walks, in one order.

**The boundary round** (`Saturator::admit_from_boundary`) judges a parked candidate only where its verdict can
have moved: the extent sizes of the watched relations are taken once a round, and a candidate judged before is
skipped unless a relation its guard set watches grew.

```plantuml
@startuml
start
:take the size of each watched relation;\nadvance the clock of each guard set that grew;
while (next parked candidate, in rank order?) is (one)
  if (judged before, and nothing\nits guard set watches grew?) then (yes)
    :skip it;
  else (no)
    :run each guard's sub-plan on the candidate's bindings\n(Matcher::holds), to the first that finds a match;
    if (no guard found a match?) then (yes)
      :admit: move it to the queue;
      stop
    elseif (the failing guard is monotone?) then (yes)
      :retire: it can only keep failing;
    else (no: it holds a nested absent)
      :stays parked, stamped with this round;
    endif
  endif
endwhile (none left)
:no admission: the fixpoint;
stop
@enduml
```

A node of the search does not start this loop from nothing: `Saturator::resume` continues root's snapshot over a
fork, with the commitment as its first delta (§ 7).

## 5. The matcher: a plan run as a register machine

A rule is compiled once per activator to a `Plan`: flat arrays of steps and slots, one register per variable. The
matcher runs it as nested loops over the KB's indexes, with a register file, a trail and a premise array, and hands
each match to a callback without copying it. The contract is
[`03-design/05-matcher.md`](03-design/05-matcher.md).

```plantuml
@startuml
start
:walk(i): the step at index i of the disjunct;
if (i is past the last step?) then (yes)
  :emit the Match:\nthe registers, the trail, the premises;
  stop
elseif (step i is the seeded one?) then (yes)
  :walk(i + 1);
  stop
endif
switch (the step)
case (Rel)
  if (every slot bound?) then (yes)
    if (FactStore::probe(rel, args) finds the fact,\nand Kb::contains it?) then (yes)
      :prems[ordinal] = fact;\nwalk(i + 1);
    endif
  else (no)
    if (a slot holds a known name?) then (yes)
      :candidates = Kb::facts_with(SlotKey),\na bucket of the participation index;
    else (no)
      :candidates = Kb::facts_of(rel),\nthe whole extent;
    endif
    while (next candidate, in extent order?) is (one)
      :mark = trail.len();
      if (unify every slot with the fact's arguments?) then (yes)
        :prems[ordinal] = fact;\nwalk(i + 1);
      endif
      :unwind the trail to mark;
    endwhile (none)
  endif
case (Guard: eq, neq)
  :both sides bound: test;\na free side: bind it over the program's names;
  :walk(i + 1) for each binding that passes;
case (Absent, inside a guard's sub-plan)
  :walk the nested steps to their first match,\nthen unwind what they bound;
  if (a match was found?) then (yes)
    :fail this branch;
  else (no)
    :walk(i + 1);
  endif
endswitch
stop
@enduml
```

**Unifying a slot** is the machine's one instruction (`Matcher::unify_slot`): a `Const` slot is a `u32` equality; a
`Reg` slot binds an unbound register, pushing it on the trail, or compares a bound one; a `Nested` slot requires a
fact, compares its relation, then unifies its arguments. Unbinding pops the trail back to a mark. The three
candidate sources yield in the extent's order and every slot is re-checked, so the source chosen changes how many
facts are offered and never the sequence of matches.

**The entry points** differ in what is bound before the walk: `run` and `run_one` start each disjunct from the
activator's seed registers; `run_seeded` binds one step to a new fact first, at every step whose relation is the
fact's; `holds` runs a lifted guard's sub-plan in its own register space, seeded from the parent's registers, and
stops at the first match.

## 6. Hypothesis generation: the ladder

H is built once, at root's fixpoint, by `hypgen::generate_config_free`. Which rung proposes is decided by one
function, `hypgen::rung`, before any rung runs; every rung feeds one filter pipeline, and the survivors are sorted
by text and deduplicated into H. The contract is
[`03-design/07-search.md` § 1](03-design/07-search.md#1-hypothesis-generation).

```plantuml
@startuml
start
if (the program states (solve :no-search)?) then (yes)
  :no rung runs: H is empty;
  stop
elseif (the program declares an hrule?) then (yes)
  :rung 1: each hrule plan's matches,\none candidate a match (Hrules::candidates);
elseif (an obligation is activated at root?) then (yes)
  :rung 2: oblgen::generate;
  if (the rung declines the call?) then (yes)
    :rung 3: the blind enumerator;
  else (no)
    :the facts that would discharge what root owes,\nand what a rule can bring a commitment to owe;
  endif
else (no)
  :rung 3: the blind enumerator:\nevery eligible relation, every slot,\nover the program's object names;
endif
:each candidate through the filters, in this order:\na stored negation, a fact root holds, a repeat in the call;
:sort the survivors by text, deduplicate: H;
stop
@enduml
```

Rung 2 declines, and the blind enumerator runs, where an obligation's guard scans a relation the rung itself
proposes, or reads one a commitment can change (`oblgen::Mode::Declined`); a state that owes only relations the
program scopes out proposes nothing and says so (`Mode::Stuck`). The filter that drops a candidate is named on its
`hyp` event, and the generator's statistics assert that nothing is lost between the two counts:

$$
\operatorname{raw} = \operatorname{emitted} + \sum_{f \in \text{filters}} \operatorname{dropped}_f
$$

## 7. The node function

Every route, and the oracle, reads a commitment through `region`'s node function: a fork of root with the
commitment written in, root's saturation resumed over it, and both verdicts read off the result. The contract is
[`03-design/07-search.md` § 2](03-design/07-search.md#2-the-node).

```plantuml
@startuml
start
:fork = root.branch();
:write each fact of the commitment, in H's order,\nwith hypothesis provenance;
:Saturator::resume(root's snapshot,\ndelta = the facts the snapshot lacks);
if (a route's node, on a monotone program,\nand the commitment is not empty?) then (yes)
  :saturate to the first contradiction,\nor to the fixpoint where there is none;
else (no)
  :saturate to the fixpoint;
endif
:contradiction::detect: dead?;
:obligations::tally: what it owes;
:closure = the candidates it holds;\ndenied = those whose negation it stores;
:state key = its facts beyond root's, sorted;
if (dead, and a core is asked of this node?) then (yes)
  :go on to the fixpoint, silently, where it was stopped;
  :explain::contradiction_frontier: the core;
endif
:NodeOut: dead, owes, closure, denied, state, its events;
stop
@enduml
```

A batch of nodes goes through `region::fan_out`, which computes them on the worker pool where one is built and
returns them in the batch's order ([`03-design/08-parallelism.md`](03-design/08-parallelism.md)).

## 8. The routes over R

`explore::run_with` prepares root (its fixpoint, H, what it owes, the static check) and then sends the question to
the route that needs the least work. A route skips a node only under a licence the static check proves for the
program's class ([`03-design/07-search.md` § 3](03-design/07-search.md#3-the-licences)).

```plantuml
@startuml
start
:prepare: root's fixpoint, H, root's debts,\nthe static check, forbid;
if (~--no-search?) then (yes)
  :root's node alone;
  stop
endif
if (the program is monotone?) then (yes)
  if (the question?) then (describe, ~--count)
    if (root owes an instance L5 or L9 licenses,\nand ~--describe-route is tree?) then (yes)
      :the tree and the classes;
    else (no)
      :the state route;
    endif
  else (-n N, ~--single, ~--minimal, ~--maximal)
    if (root is dead?) then (yes)
      :route 1, which visits root alone (L1);
    else (no)
      :the routes over states:\nthe obligation tree, the climb, the shrink;
    endif
  endif
elseif (root is dead?) then (yes)
  :route 1, which visits root alone:\na fork only adds, so every node is dead (L0);
else (flagged)
  if (-n N or ~--single over R or its minimal line,\nor more candidates than ~--region-limit,\nor a minimal question with ∅ in R?) then (yes)
    :the cube in layer order,\nstopped when the question is decided;
  else (no)
    :the whole cube: route 2, the oracle's;
  endif
endif
stop
@enduml
```

### 8.1 The state route

Describe's route where the tree has nothing to branch on, and under `--describe-route states`
(`states.rs`). On a monotone program a commitment's state is its closure's (L3), so the route walks closed sets,
smallest first, and saturates only one-fact extensions:

```text
states = { cl({}) }                         # root's state
for size in increasing order:               # one layer per size
    for S in the alive states of this size:
        for b in H, b not in S:
            if S + {b} holds a set known dead: skip      # L1, L8: the one memory
            if S + {b} is a state already met: skip      # L3
            saturate S + {b} once, whichever state asks  # the node function
            dead  : remember it dead
            alive : its closure is a state, new or met before
describe R off the states met: the lines, the generators, N, k_s, k_c
```

`N`, the minimal dead sets, is read off the alive side: the minimal transversals of the complements of the
maximal alive states (`transversal.rs`). The commitments over a state $T$ are counted without visiting them:

$$
f(T) = 2^{|T|} - \sum_{S \subsetneq T,\; S \text{ alive}} f(S)
$$

### 8.2 The obligation tree, and the lines

The other questions on a monotone program walk the same lattice of states and take only what they need
(`tree.rs`). At an owing state the tree branches on the alternatives of **one** owed instance where L5 or L9
licenses it, since every member of `R` above the state holds one of them; an owing state with no licensed instance
branches on every extension.

```plantuml
@startuml
start
:take the next state C of the tree,\nowing states by size, then closure;
if (C owes nothing?) then (yes)
  :C is in R: a leaf;
  stop
elseif (C owes a licensed instance?) then (yes)
  :choose the instance I (order.rs);\nbranch on its alternatives A(I);
else (no)
  :branch on every candidate not in C;
endif
while (next branch b?) is (one)
  if (C + {b} holds a set known dead?) then (yes)
    :dead with no saturation (L1, L8);
  else (no)
    :the node of C + {b};
    if (dead?) then (yes)
      :remember the set dead;
    else (no)
      :its closure joins the tree;
    endif
  endif
endwhile (none)
:where every branch was dead,\nno member of R lies above C;
stop
@enduml
```

| question | what it does with the tree |
|---|---|
| `-n N` | the tree's leaves in its own order; when it runs out, the states of `R` above its leaves, each extended once (L2) |
| `--single` | stops at the tree's second state of `R`; with exactly one, that state's extensions until one is alive (L4) |
| `--minimal` | each leaf's commitment shrunk while it stays discharged; for the whole line, the alive minimal transversals of the complements of the maximal owing states |
| `--maximal` | a first state of `R` climbed to a maximal one, then dualise and advance above each minimal state: each minimal transversal of the complements of the maximal states found is saturated with the seed, and an alive one is climbed |

**The tree and the classes** (`tree_classes.rs`), describe's default route where root owes a licensed instance,
finds `R`'s states by that tree with no bound on their number, then each state's class, its commitments and
minimal generators, by dualise and advance inside the state. It computes neither `N` nor an empty `R`'s owing
frontier, and the read-out says so.

### 8.3 Route 1, the cube, and the oracle

**Route 1** (`downset.rs`) walks a monotone program's alive down-set layer by layer from ∅: a commitment of size
$k + 1$ is visited only when each of its $k$-subsets is alive, so it saturates the alive commitments and the
minimal dead sets, $|\operatorname{alive}| + |N|$ nodes. **Route 2**, the cube (`region::compute_prepared`), saturates all $2^n$ commitments over $n$ candidates and reads the
topology off the status array; it is the only route a flagged program has, and `ein oracle` is that route asked
for on any program, every node to quiescence. Every mode is held to it (`ein-infer/tests/explore.rs`).

## 9. Core extraction

A dead node's core is the smallest set of given facts, written or committed, from which one of its contradictions
follows. Provenance is an AND/OR graph, a fact an OR over its recorded derivations and each derivation an AND over
its premises, and `explain.rs` computes a label per fact, its subset-minimal environments, by a least-fixpoint
propagation with a budget. The contract is
[`03-design/07-search.md` § 6](03-design/07-search.md#6-cores-and-explanations).

```plantuml
@startuml
start
:build the graph: the premise closure of the contradiction's facts,\nevery justification the KB holds for each (Kb::justifications);
:label each given fact {itself};\na premise-free derivation {the empty environment};
:dirty = the consumers of the labelled facts;
while (dirty is not empty, and rounds < max_rounds?) is (yes)
  :take the dirty facts, sorted by rank;
  while (next fact?) is (one)
    :for each of its derivations: fold the premises' labels\ninto the unions of one environment each;
    :minimise: drop supersets, keep the smallest\nmax_environments of them;
    if (the label changed?) then (yes)
      :its consumers become dirty;
    endif
  endwhile (none)
endwhile (no)
:each contradiction: the smallest environment of (false),\nor of X and (not X) folded as one AND;
:the core is the smallest over the node's contradictions;\nits status: within-budget, or cut and the caps it met;
stop
@enduml
```

Dropping a superset loses nothing, since union is monotone; only the caps lose environments, and a search they
stopped reports a real frontier that may not be the smallest, with a status that says so. A premise closure past
`ExplanationBudget::max_facts` is not searched: the frontier of the recorded derivations is returned in its place,
and the status says it was cut. A core is taken only where a surface prints one (`region::Cores`).

## 10. The obligation pass

A rule whose `:assert` is `(open ?R)` is in no agenda. `obligations::tally` runs once over a KB after its
saturation returns and reports what the KB still owes; nothing it reports is stored
([`03-design/06-saturation.md` § 6.3](03-design/06-saturation.md#63-obligations)).

```plantuml
@startuml
start
if (the program has an obligation rule?) then (no)
  :owes nothing;
  stop
endif
while (next obligation rule, in registry order,\nand next activator of it?) is (one)
  :its plan, from the run's memo;
  :Matcher::run: collect the matches,\none per distinct set of bindings;
  while (next match?) is (one)
    if (a guard of its disjunct holds?) then (yes)
      :a witness exists: discharged;
    else (no)
      :an owed instance: the rule, the activator,\nthe relation (open ?R) names, the bindings,\nthe rule's :why rendered;
    endif
  endwhile (none)
endwhile (none)
:one owe event an instance;\nOwes: the instances in report order;
stop
@enduml
```

The rule matches while the witness is missing and stops matching once it has arrived, so the pass asks no second
question to learn whether a debt was paid. A node's kind is read off the two verdicts: dead where its fixpoint is
contradictory, owing where it is alive and `Owes` is not empty, in `R` otherwise.
