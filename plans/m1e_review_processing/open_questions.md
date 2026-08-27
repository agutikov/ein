# Open Questions — M1e (review processing)

Questions **this milestone raises**, with sticky `Q-M1e.<n>` ids. Do not
reuse a closed id.

The review's own ten questions are **not** here: they are the subject of
[P1e.1](p1e.1_open_questions/README.md), they keep the review's `Q1`–`Q10`
numbering, and they live in
[`review/open-questions.md`](review/open-questions.md) with their answers
recorded in the stage that answers them. A review question that turns out
*not* to be answerable within this milestone is re-filed here with a fresh
`Q-M1e.<n>` and a named owner — that re-filing is a result, and the stage
records which question became which id.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted) | What is the standard of proof for **refuted**? | open — decided in [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) T1, applied everywhere after |
| [Q-M1e.2](#q-m1e2--may-a-review-finding-be-closed-by-a-comment) | May a finding be closed by a comment rather than a check? | open — the `accepted` disposition's rule |
| [Q-M1e.3](#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted) | Who owns a `docs/kernel` page that should be neither fixed nor deleted? | open — [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) decides per page; the *rule* is here |
| [Q-M1e.4](#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all) | Does the repo want an exact count in prose at all? | open — [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) |
| [Q-M1e.5](#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | Is *experimental* a licence to ship a surface whose read-out is false? | open — [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md), and M1d's `T1d.10.6.4` is the co-owner |
| [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model) | What is a **solution**, and what is a **model**? | **decided 2026-08-28** by the user; binding on [Q5](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) and on [P1e.1b](p1e.1b_hypothesis_structure/README.md) |
| [Q-M1e.7](#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model) | The read-out prints the solution **KB** and calls it a model | open — raised by Q-M1e.6; owner unassigned |
| [Q-M1e.8](#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set) | `exhausted` certifies the **lattice**, not the model set | open — raised by Q-M1e.6; `lattice/02 -e -L` is the witness |
| [Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent) | Is `dead` really upward-closed under `absent`? | open — two kernel pages appear to disagree; **not** promoted to a finding |

---

## Q-M1e.1 — What is the standard of proof for **refuted**?

Sixty of the sixty-three findings are one reader's reading; the review's
verification stage never ran ([`review/summary.md`](review/summary.md)
§ Method). So the milestone will refute some of them, and *refuted* needs a
bar, because the cheap version — "I read the code and disagree" — is the same
epistemic move that produced the finding.

The proposed bar, to be ratified in
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) T1 and then
binding on every stage:

- A finding claiming a **behaviour** is refuted only by an executed probe —
  a program, a command, an output — banked as a test that would fail if the
  behaviour ever appeared. [CD-H3](README.md#the-findings) is the model: the
  review refuted a documented bug with one probe, and the review's own
  recommendation is to *bank the probe both ways*.
- A finding claiming an **absence** (no test holds X, no page states Y) is
  refuted by naming the test or the page. That is cheap and it is enough.
- A finding claiming a **risk** (this is unenforced, this could drift) cannot
  be refuted by argument at all — only `fixed`, `accepted` with the argument
  written at the site, or `deferred`. Saying "it cannot happen" *is* the
  written argument, and it goes beside the code.

## Q-M1e.2 — May a review finding be closed by a comment?

Several findings are of the form *this is stated but not enforced*
([ST-M1](README.md#the-findings), [CO-M1](README.md#the-findings),
[CO-H3](README.md#the-findings)(c)). For each, the honest options are a
check, or a written argument — and the repo's method has used both:
`design/02` is an argument, `check_hashmap_iteration.py` is a check.

The question is when an argument is sufficient. The proposed rule: an
argument suffices when its **premise is itself enforced**. The alive-set
invariant's argument rests on *rules assert no new objects or relations* —
which nothing checks, so the argument is not sufficient and
[ST-M1](README.md#the-findings) needs the cheap post-fixpoint check.
Contrast [ST-L1](README.md#the-findings) (`EqClasses` auto-vivification),
whose premise is *nothing fires equality propagation* — enforced by
`naf_semantics::matching_does_not_resolve_equality_classes`, an existing
named test — so a comment at the future wiring point is enough.

If that rule holds, it decides most of the `accepted` dispositions
mechanically, and it should be written into
[`docs/kernel/defined_behaviour.md`](../../docs/kernel/defined_behaviour.md)
or `design/`-style prose rather than living only here.

## Q-M1e.3 — Who owns a page that should be neither fixed nor deleted?

[CD-H1](README.md#the-findings) covers pages in three states, and the review
names the triage: *current* (fix), *superseded with a banner*, *moved to
`docs/history/`*. The rule that put a document into `docs/history/` is written
down — *it is still read, as a specification, as evidence, or as the reason
something is the way it is* — but `algorithm_layer_n.md` fails it in an
awkward way: nothing reads it as a specification (its three solve entries do
not exist), and it is not evidence, but it **is** the reason
`architecture_and_algorithms.md` §41-48 records a removed soundness bug. Half
a reason.

Three candidate answers, none obviously right: (a) `docs/history/m1_core/` —
a directory that does not exist, for the milestone whose plans were deleted
at P1.22; (b) delete, since git history holds it and the surviving reason is
already stated where the refutation is; (c) keep in place with a banner as
strong as `parity_baselines.md`'s. This question is the *rule*;
[S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T1 applies it per page
and records which pages the rule sent where.

## Q-M1e.4 — Does the repo want an exact count in prose at all?

[DO-M1](README.md#the-findings) is not a list of typos: it is one mechanism,
observed eight times, and the repo already knows the mechanism — *a page
nothing runs goes stale*. Every count a test pins is exactly right; every
count only prose states has drifted.

So the question is not *fix the numbers* (a one-day pass that rots again by
M2) but whether a count belongs in prose. Three shapes are available and the
repo uses all three somewhere: the **generated** count (the embedding page's
marked region, diffed by a test), the **census-owned** count (say *the census
prints it* and link, as `corpus_cost.md` does), and the **dated** count (*as
of the M1a close, 616*). A fourth — a markdown-level check that a stated
number matches a script's output — does not exist and would be new machinery.

The answer decides whether [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md)
is a counting pass or a de-counting pass, and it is worth taking before the
pass rather than during it.

## Q-M1e.5 — Is *experimental* a licence to ship a lying surface?

`EIN_TRAVERSAL=tree` is opt-in, undocumented as stable, and honestly recorded
as open (`T1d.10.6.4`). It also reports `Contradiction` with an empty unsat
core, ignores `-n` and `-m`, and reads `refuted so far (0 facts)` — which is
not an incomplete read-out but a false one
([CO-H3](README.md#the-findings)(b)).

The two positions are both defensible in this repo's terms. *Experimental
means the surface may be absent or may change* — but the project's own
discipline is that a **verdict** is never qualified by how the search got
there, which is exactly why `Ambiguity` learned to say *(a lower bound)* at
S1d.3.3 rather than keep printing a bare `k`. Under that reading, an empty
core printed as evidence is the same defect S1d.3.3 fixed, and the experiment
flag does not license it.

The narrow fix — make the arm refuse to print evidence it does not have — is
available without answering the design question, and
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3 takes it. The general rule is
this question, and it belongs with `T1d.10.6.4` when M1d's traversal work
resumes.

## Q-M1e.6 — What is a **solution**, and what is a **model**?

**Decided 2026-08-28**, by the user, in answer to
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md)'s Q5 —
which had asked which of two engine configurations is right and was told,
correctly, that the question is prior to both. Recorded verbatim:

> Solution is a KB state after saturation (saturated KB state) without open
> obligations or if other hypgen used — with some integrated subset of
> hypothesis and not consistent with any other hypothesis (e.g. set of 3 hyp
> facts integrated, then saturation derives 10 other facts with other 5 from
> hypothesis list, then this state has 8 hypothesis integrated, and it would be
> a solution iff all other hypothesis are inconsistent with this KB). Model is
> a positive part of solution KB minus positive initial KB. Initial KB is
> before first saturation, no derived facts only problem statements.

### The definition, restated

**Solution.** A KB state `S` is a solution iff

1. `S` is **saturated** — quiescent under the rule set; and
2. `S` is **consistent** — no `(false)`, no same-layer `X ∧ ¬X`; and
3. **either** the obligations rung is in play and `S` **owes nothing**,
   **or** — under any other generator — for every hypothesis `h` of that
   generator's list that is not already in `S`, `S ∪ {h}` is **inconsistent**.

Clause 3's second arm is a **maximality** criterion, and the user's worked
example is the part an implementation gets wrong: *integrated* counts the
hypotheses saturation **derived**, not only the ones committed. Commit three,
saturate, and if five more members of the hypothesis list appear among the ten
derived facts, the state has integrated **eight**; the test is over the
remaining ones.

**Model.** `positives(S) \ positives(initial KB)`, where the initial KB is the
loaded program **before the first saturation** — problem statements only, no
derived facts. So a model carries no `(not …)`, no `(is-a …)` the file
declared, no `(relation …)`, and no rule-application marker that was written
down: it is *what the puzzle did not say and the solve established*.

### What it settles, immediately

- **Q5's OFF side is wrong.** With `-L`,
  `examples/lattice/02_genuine_3set_death.ein` **exhausts** and prints *"No
  solution — the constraints are contradictory"* with a three-fact core, on a
  program whose solutions are `{h₁,h₂}`, `{h₁,h₃}`, `{h₂,h₃}` by inspection.
  Under clause 3 each of those is a solution: the third candidate is
  inconsistent with the state. `-L` makes `complete` under-report, and the
  default's `k=3` is right.
- **`complete` is an approximation, and the definition says in which
  direction.** `complete(S) ≡ the generator proposes nothing`
  ([`hypgen.rs:902`](../../ein.rs/crates/ein-infer/src/hypgen.rs)) is
  generator-relative; clause 3 is not. The one-step lookahead only drops a
  candidate it can **prove** dies, so using it inside `complete` is *sound* —
  every state it calls complete is a solution — and *incomplete*: a candidate
  that dies in two firings is missed and a real solution goes unrecorded.
  Turning the lookahead off does not make the test honest, it makes it
  strictly weaker. **Both configurations under-report; `-L` under-reports
  more.**
- **The `-K` fact-set difference is not a defect.** With the kill cache off,
  `lattice/02`'s recorded states lose their `(not (c-prop X))` facts. Those
  are negatives, so they were never part of the **model**; what changed is the
  solution **KB**. See [Q-M1e.7](#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model).

### What *all other hypotheses* quantifies over — settled 2026-08-28

Not the generator's per-node list. The user's second clarification, verbatim:

> "All other hypothesis" mean exactly ALL other than integrated n L1
> hypothesis (which is one L{n} hypothesis) — so no other L{n+1} would be
> consistent — so for this L{n} hypothesis exhausive search ends.

So the reference set is the **layer-1 hypothesis set** — `alive₀`, the set the
lattice enumerates subsets of — and a state reached by an L{n} commitment `C`
(which *is* n L1 hypotheses) is a solution exactly when **no L{n+1} extension
of it is consistent**.

### The operational form, which is the useful one

```
solution(C)  ≡  C is alive  ∧  ∀ h ∈ alive₀ \ integrated(C):  C ∪ {h} is dead
```

*"For this L{n} hypothesis exhaustive search ends"* is the same sentence read
as a stopping rule: **a solution node is a maximal alive commitment — one with
no live child.** Three things follow immediately, and they are why this
clarification is worth more than the declarative form.

**1. The lattice already computes it, one layer later.** Layer `n+1` enters
the supersets of every surviving `C`, and the ones apriori declines to
generate are the ones a subset already proved dead. So *"did any superset of
`C` survive?"* is answered by layer `n+1`'s own results, with **no extra
fork**. The engine computes it and throws it away: `a_layer` becomes `a_prev`
and is never asked which of its members had a live child.

That step **inherits a premise rather than adding one**: it needs `dead` to be
upward-closed, which is exactly what apriori's pruning and the no-good store
already need. The repo asserts it —
[design/08](../../docs/history/m1a_rust/design/08_parallelism.md) § The
objects: *"`dead(X)` … **Monotone**: `X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the
KB is append-only and nothing retracts"* — and
[Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent) is why that
sentence is worth re-reading before anything new leans on it.

**2. `complete()` is an approximation of that lattice property, and it is
sound in one direction only.** `hypgen::complete` asks the generator *at the
node, now*, and its answer is filtered by the pipeline
([`hypgen.rs:422`](../../ein.rs/crates/ein-infer/src/hypgen.rs)). The
lookahead only drops a candidate it can **prove** dies in one firing, so:

| | holds? | consequence |
|---|---|---|
| `complete(S)` ⟹ `solution(S)` | **yes**, with the lookahead on or off | the engine never records a false model |
| `solution(S)` ⟹ `complete(S)` | **no** | a remaining candidate that needs two firings to die is still proposed, so a real solution goes unrecorded |

So the engine **under-reports**, always, and `-L` under-reports far more —
with the lookahead off, `complete` is true only when every remaining candidate
is already asserted or already negated in the KB.

**3. The filters are not part of the definition, and at root they are
harmless.** A candidate the lookahead kills at root is one `root ∪ {h}`
refutes; by monotonicity `S ∪ {h}` refutes it at every descendant too. So
excluding it from `alive₀` loses no solution — F3 at root is
definition-preserving. The damage is at the deeper nodes, where the
*approximation*, not the filtering, is what misses a maximal state.

### The premise this inherits

Quantifying over `alive₀` presupposes that `alive₀` is the whole hypothesis
space — that no fork derives an object or relation that would have made a new
hypothesis possible. That is precisely the **alive-set invariant** of
[ST-M1](README.md#the-findings), which the review found *"is enforced
nowhere"*. The definition and the dedup warrant now rest on the same
unchecked premise, which raises ST-M1 from a Medium tidy-up to the thing the
semantics stands on.

### Where it goes

Into `docs/kernel/` as a normative page —
[P1e.5](p1e.5_documentation/README.md)'s proposed S1e.5.2 — not into this
file. A ruling that lives only in a plan is the shape
[Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted) forbids.

## Q-M1e.7 — The read-out prints the solution **KB** and calls it a model

Raised by [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model), which
defines a model as the positive part of the solution KB minus the positive
initial KB. Nothing in the engine computes that object.

What is printed and stored instead is the whole fact list of the solution
state: `verdict.solutions[i].facts` in `--json-summary`, the `model n/k`
blocks in `ein solve`, the `BTreeSet<Vec<String>>` that
[`tree_traversal.rs`](../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)
compares fact for fact, and the variables
[`model_set_census.py`](../../utils/model_set_census.py) derives. All four
include the negatives, the ontology and the rule markers —
`lattice/02`'s "model" is nine facts of which **six** are `(relation …)`.

Three consequences, and the second is the one that bites:

- The **name is wrong** in four surfaces, which is
  [SE-M1](README.md#the-findings)'s vocabulary defect in a second place.
- Two runs that agree on every model can **disagree on the recorded fact
  sets** — `-K` does exactly this. Any test or census that compares fact sets
  is comparing solution KBs, so it is sensitive to levers that provably do not
  change the answer. `tree_traversal.rs`'s comparison is the load-bearing one.
- `model_set_census.py`'s *varying slots* and determining keys are computed
  over an object that includes facts the program itself supplied, which can
  only inflate them.

Not obviously a defect to *fix* — printing the solution KB is defensible and
`--models key` already projects it. But the two objects need two names, and
the fact-set comparisons need to say which one they mean. Owner unassigned;
candidates are [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) (the vocabulary
split) and P1e.5's proposed S1e.5.3 (the read-out reference).

## Q-M1e.8 — `exhausted` certifies the **lattice**, not the model set

Raised by [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
operational form, and it has a witness in the corpus today.

`exhausted = !truncated` ([`solve.rs:2388`](../../ein.rs/crates/ein-infer/src/solve.rs))
is set when the search did not stop early — no depth cap hit with a live
frontier, no `stop_after` cut, not the tree. It is read by the verdict as a
**certification**: `Ambiguity` prints a bare `k` rather than *"(a lower
bound)"*, and `Contradiction` prints *"No solution — the constraints are
contradictory"* rather than the hedged *"the search did not exhaust the
lattice"*.

It certifies the wrong thing. Run, measured 2026-08-27:

```
ein solve -e -L examples/lattice/02_genuine_3set_death.ein
  solutions (k)   0            exhausted = true      7 enterings, 3 layers
  verdict         No solution — the constraints are contradictory
```

That program has **three** solutions under Q-M1e.6 — `{h₁,h₂}`, `{h₁,h₃}`,
`{h₂,h₃}` — and the search *found all three states*: it entered each pair,
each survived, and it then proved every triple dead. Every fact the right
answer needs was in `lstate`. What failed is that no surviving pair was
flagged `solved`, because `complete()` with the lookahead off still proposes
the third candidate — and `finalise` reads only `lstate.nodes`, which is
empty, so the Contradiction arm unions the dead cores and asserts
unsatisfiability.

**Two completeness notions wear one word.** *The lattice was walked to the
end* and *every solution in it was recognised* are different claims, and only
the first is what `truncated` tracks. A verdict that says *the constraints are
contradictory* is asserting the second.

The narrow fix is available and is **cheaper than making the lookahead
unconditional**: a surviving commitment whose every superset died is a
solution by construction, layer `n+1` already computes that, and retaining it
costs one bitset over `a_prev` per layer — no new fork, no new saturation. Under
`-e` it is free; under `-n 1` it defers a model by one layer, which is a
trade-off to measure rather than assume.

Owner unassigned. It is the same seam as
[CO-H3](README.md#the-findings)(b) (a `Contradiction` whose evidence is empty)
and [Q-M1d.6](../../docs/history/m1d_satisfiability/open_questions.md)
(may `Contradiction` be said with `exhausted = false`) — but strictly worse
than either, because here `exhausted` is **true**.

## Q-M1e.9 — Is `dead` really upward-closed under `absent`?

**Not a finding.** It is two pages of this repo that appear to disagree, found
while writing [Q-M1e.8](#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set),
and it bears on machinery that **already ships** rather than on anything
proposed here.

[design/08 § The objects](../../docs/history/m1a_rust/design/08_parallelism.md)
states the property as a definition:

> `dead(X)` — `X` holds a contradiction. **Monotone**: `X ⊆ Y ∧ dead(X) ⇒
> dead(Y)`, because the KB is append-only and nothing retracts.

[`absent_semantics.md` C3](../../docs/kernel/inference/absent_semantics.md)
states what looks like its contrapositive as a live caveat:

> Removing a fact can flip an absent and **fabricate** a contradiction the
> full KB never had.

Read together: if removing a fact can create a contradiction, then adding one
can remove it — and `dead(X) ∧ X ⊆ Y ⇒ dead(Y)` fails for any `(false)` whose
derivation passed through an `absent` guard that `Y` satisfies. design/08's
stated reason — *append-only, nothing retracts* — establishes that `sat` is
**inflationary**, which is not the same as monotone in its input, and the
distinction is exactly what `absent` introduces.

**What it bears on, in order of exposure:**

1. **The no-good store.** A clause `¬(h₁ ∧ … ∧ h_L)` learned in the world
   `B ∪ c` is applied to every superset. That is a negative result cached
   across worlds, which is the shape [C6](../../docs/kernel/inference/absent_semantics.md)
   — *"`absent` is world-relative. Results must not be cached across worlds"* —
   exists to forbid.
2. **Apriori's downward-closure filter**
   ([`apriori.rs`](../../ein.rs/crates/ein-infer/src/apriori.rs)), which
   declines to generate a superset of a dead set.
3. The maximality test of Q-M1e.8, last and least, because it is the only one
   of the three that does not ship.

**Why it may be harmless in practice, and why that is not an answer.** The
corpus's `(false)`-deriving rules are the algebraic scans — `functional`,
`injective`, `total`, `no-room-left` — and a positive-only `absent` guard is
*anti-monotone*, so a guard that fails stays failed and the derivation it
blocked cannot come back. The exposed shape needs a **nested** absent (a
`forall`), which `absent_semantics.md` says can flip false→true. Whether any
program can put one on the path to a `(false)` is a probe, not an argument,
and [Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s third rule
says a risk is not refutable by argument at all.

**Owner unassigned.** It sits squarely in
[Q9](review/open-questions.md)'s unswept surface — *no dedicated pass over
algorithmic pathology or invariants* — and it is the kind of thing
[S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md) T4 should be told
about before it scopes that sweep. Whichever way it resolves, one of the two
pages needs an edit, which makes it also a
[CD-H2](README.md#the-findings)-shaped defect: two live pages, one subject,
opposite claims.
