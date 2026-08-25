# Open Questions — M1d (From saturation to satisfiability)

Milestone-scoped questions. Ids are **sticky** — `Q-M1d.<n>`, in the style
[M1a](../../docs/history/m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md).
A closed id is never reused.

**Q-M1d.1 arrived with [P1d.10](p1d.10_exhaustive_search/README.md)** on
2026-08-21, where it was Q-M1a.21; the M1a entry stays as a redirect. Q-M1d.2
to Q-M1d.5 come from [`ideas.md`](ideas.md), the note that is the milestone's
other half, and they are the questions the note leaves open rather than the
ones it answers. **Q-M1d.6 came from neither**: it was found by measuring, in
M1a [S1a.9.0](../../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced), and it is
about a word the engine already says.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1d.1](#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) | May the search stop before the lattice is exhausted? | open — [P1d.10](p1d.10_exhaustive_search/README.md); `exhausted` keeps its meaning either way *(was Q-M1a.21)* |
| [Q-M1d.2](#q-m1d2--where-does-a-requirement-live) | Where does a requirement live — kernel, stdlib, or rule shape? | **decided 2026-08-24** — (c) a rule shape asserting the reserved verdict atom (form G); [S1d.2.3](p1d.2_obligations/s1d.2.3_the_form.md) records it |
| [Q-M1d.3](#q-m1d3--what-closes-a-domain) | What closes a domain? | narrowed 2026-08-24 — the witness domain is the obligation's own guard; [S1d.2.2](p1d.2_obligations/s1d.2.2_domains.md) owns the residue (closure, open extents) |
| [Q-M1d.4](#q-m1d4--may-an-obligation-driven-generator-change-the-traversal) | May an obligation-driven generator change the traversal? | **decided in principle 2026-08-24** — the user: obligations supersede `:hrules`; [S1d.2.5](p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md) executes it with the re-baseline |
| [Q-M1d.5](#q-m1d5--print-or-describe) | 32 models: print or describe? | open — [P1d.3](p1d.3_model_sets/README.md); "enumerate, and say so" is an acceptable answer |
| [Q-M1d.6](#q-m1d6--may-contradiction-be-said-with-exhausted--false) | May `Contradiction` be said with `exhausted = False`? | open — ten corpus entries already say it; [S1d.2.6](p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md) closes it: the word is decided (`Open`), the partition is measured first |
| [Q-M1d.7](#q-m1d7--may-a-program-require-its-own-model-count) | May a program require its own model count? | open — [P1d.4](p1d.4_model_set_closure/README.md); arrived from M1c [S1c.1.2](../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects) on 2026-08-24 |

---

## Q-M1d.1 — May the search stop before the lattice is exhausted?

[P1d.10](p1d.10_exhaustive_search/README.md)'s question, and the measurement
that raises it: on `examples/zebra2-minus-15.ein` **every one of the 32 models
is found by depth 3, and depths 4–5 exist only to prove there are no more** —
which is where the run stops finishing.

So: is there an argument that lets the search stop early?

- **A sound criterion** proves the same thing sooner. It was in scope even
  under M1a's "no new reasoning features" non-goal, because it changes the
  cost of the proof and not the proof; here it is the phase's first prize.
- **A heuristic** ("no new model for k layers") changes the answer. It ships
  behind a flag, off by default, reporting `Ambiguity (not certified)` — and
  **never sets `exhausted = true`**. The word means the lattice was exhausted;
  a second guarantee needs a second word.

The candidates and their obligations are in
[S1d.10.3](p1d.10_exhaustive_search/s1d.10.3_stopping_criterion.md), and a
written refutation is as good an outcome as a proof — that is the discipline
[F9](../followups/f9_e_catalog.md) established for this exact area, and F9's
own judgements were all measured on puzzles with a unique model, which is the
regime this question is not about.

**Moved 2026-08-21 from Q-M1a.21**, with the phase. The one thing the move
adds: [P1d.2](p1d.2_obligations/README.md) is a fourth candidate the M1a
framing did not have — a state that knows what it still owes can recognise a
model *locally*, and an enumeration that branches on requirements is complete
at a depth bounded by the number of requirements rather than by
`max_set_size`. That is not yet a stopping criterion for the *model set*, and
[S1d.10.3](p1d.10_exhaustive_search/s1d.10.3_stopping_criterion.md) should say so
carefully; it is, however, the first candidate that attacks the exponent
instead of the constant.

## Q-M1d.2 — Where does a requirement live?

**Decided 2026-08-24 — (c), a rule shape**, with one reserved verdict atom
(`open`) that rules assert and the engine tallies per quiescent KB: form G on
[`obligation_forms.md`](p1d.2_obligations/obligation_forms.md), recorded in
[S1d.2.3](p1d.2_obligations/s1d.2.3_the_form.md). The candidate set is
neither stored nor narrowed in place — recomputed from the obligation's own
guard when wanted — which dissolves the (a)/(b) cost trade below rather than
picking a side of it. The text that follows stands as the record of the
question as asked.

The note's headline is a design instruction: existence requirements are
**first-class obligations**, not generators of arrows. Three places that could
live, and they cost in different currencies:

- **(a) A derived fact.** `(owes R a {b1 b2 b3})` asserted by a rule, read by
  rules. Costs nothing structurally — it is what the stdlib already does with
  activators — and probably cannot carry a candidate set that shrinks, because
  the store is append-only and a narrowed set is a *new* fact each time.
- **(b) A kernel object.** The saturator tracks obligations beside the fact
  store, with an index that narrows in place. Buys the shrinking candidate set
  and the quiescence report; costs a new concept in the data model, in the
  `.einb` container, in the fork's copy-on-write layer, and in every
  renderer.
- **(c) A rule shape.** `forall` already quantifies over a domain and
  `domain-elimination` already forces a singleton. Perhaps the missing middle
  is expressible without a new object at all — in which case the phase is much
  smaller than it looks.

**No recommendation yet**, and that is deliberate: the choice depends on
[S1d.2.1](p1d.2_obligations/README.md)'s audit of what the rules already do
and on whether the candidate set has to be *stored* or can be *recomputed*.
The last one is a performance question with a measured precedent —
`_admit_from_boundary`'s re-query was 72 % of an exhaustive `zebra2` before
P1a.6 — so "recompute it" is not automatically cheap.

## Q-M1d.3 — What closes a domain?

**Narrowed 2026-08-24, sharpened 2026-08-25**: for obligations, C is the
obligation's own guard — the `?isa`-parameterised scan standing beside the
witness step inside the rule's `absent`, which `(open ?R)` names the relation
of rather than restating — so *stating* and *discharging* a requirement needs
no closed domain at all. What remains is
what needed closure all along: refutation (which stays with the `forall`
scans) and the open-extent regime;
[S1d.2.2](p1d.2_obligations/s1d.2.2_domains.md) banks the contract.

`∀x ∈ D. ∃y ∈ C. R(x,y)` is unanswerable without knowing D and C. The note
lists the sub-questions: what is in the set, is the set closed, and may new
objects appear. Ein has `is-a` extents, `is-a*` for the transitive closure,
the `unknown` macro, and a corpus entry
([`features/04_open.ein`](../../examples/features/04_open.ein)) whose whole
point is that an open domain makes the search unbounded.

**What that costs, measured** (M1a
[S1a.9.0](../../docs/history/m1a_rust/measurements/corpus_cost.md)): `render lattice` on that
one file — an exhaustive solve at `-m 3` with the lattice stored — is **10.2 s,
the slowest cell in the corpus and 640× `zebra2`'s entire `solve`**. At the
`solve` default of `-m 5` the same file has no answer at all: it reaches
**14.3 GB** of anonymous memory and is killed by the OOM killer at 78 s, and so
are the three `saturation/square-unique/*` demos, at around a minute each. An
unbounded hypothesis space is unbounded in **memory** first, and the depth cap
is the only thing standing between a corpus fixture and that wall. Nowhere in
the tree said so before; the sentence belongs to this question.

So the question is not "does Ein have domains" — it is **where the closure is
stated and who is allowed to rely on it**. A lower bound that quantifies over
a domain the puzzle never closed is either unenforceable or wrong, and the
engine has to say which at load time rather than at quiescence.

Related: the stdlib is deliberately **is-a-free in rule bodies** — the
hierarchy relation arrives as an activator parameter. An obligation that
hard-codes `is-a` would put a type system in the kernel, which
[S1.7.23](../../docs/history/m1a_rust/README.md) settled it would not have.

## Q-M1d.4 — May an obligation-driven generator change the traversal?

**Decided in principle 2026-08-24**, by the user: *"the obligations mechanism
also has to supersed the hrule and :hrules, so if no :hrules in query — then
hypothesis must be generated from obligations"* — the generator ladder in
[`obligation_forms.md` § Superseding](p1d.2_obligations/obligation_forms.md).
[S1d.2.5](p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md) executes
it, and the re-baseline discipline below is how.

Generating hypotheses from an obligation's candidate set instead of from
`alive` produces branches that are mutually exclusive and jointly exhaustive —
a different traversal, therefore different `enterings_*`, different no-goods,
different `layers_explored`, and a different order of discovery for the models
themselves.

This is exactly the shape of
[Q-M1a.18](../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint),
which had to be decided before a fork was allowed to narrate less, and of
[design/08](../../docs/history/m1a_rust/design/08_parallelism.md) §7, which rejected parallel
depth-first because "going depth-first changes which no-goods exist when, i.e.
the pruning, i.e. the counters".

The invariants that survive any answer are the ones
[S1a.7.0](../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit) already
pinned as tests: the *answer* depends on neither the entering order nor the
integration time. What is negotiable is everything that is not the answer, and
the phase has to say so explicitly rather than discover it in a golden diff.

## Q-M1d.5 — Print or describe?

If a puzzle has 32 models, is the answer 32 models or a description of them?
[P1d.3](p1d.3_model_sets/README.md) owns it, and the reason it is a question
rather than an obvious yes is that every consumer downstream reads *models*:
the trace, `:expect`, [M20](../m20_gui/README.md)'s views, and the benchmark
adapters that compare Ein's answer to Clingo's.

"Enumerate, and say so" is a legitimate answer. So is "report the factorisation
and enumerate on request". What is not legitimate is a compact form that only
the engine can read.

## Q-M1d.6 — May `Contradiction` be said with `exhausted = False`?

**Arrived 2026-08-22 from M1a
[S1a.9.0](../../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced)**, which re-priced
the corpus's slow tail and found ten entries costing the *same* exhaustively as
on the fast path — `solve` and `solve -e` within 2 % of each other on nine of
them and 1.24× on the tenth, where the two paths should differ by the whole of
the search
([corpus_cost.md § 2B](../../docs/history/m1a_rust/measurements/corpus_cost.md)). Every one of
them ends the same way:

```text
Contradiction  k=0  exhausted=False  layers_explored=5  enterings=…
```

`layers_explored == max_set_size`, the default `-m`. Sweeping `-m` on the
cheapest of them — `examples/features/02_star_in_identifiers.ein`, a lexer demo
with two rules, four `is-a` edges and no hypothesis structure at all — shows
what is happening:

| `-m` | verdict | `layers_explored` | enterings | wall |
|---:|---|---:|---:|---:|
| 1 | `Contradiction`, `exhausted=False` | 1 | 15 | 9 ms |
| 2 | `Contradiction`, `exhausted=False` | 2 | 120 | 8 ms |
| 3 | `Contradiction`, `exhausted=False` | 3 | 575 | 31 ms |
| 4 | `Contradiction`, `exhausted=False` | 4 | 1 940 | 79 ms |
| **5** (the default) | `Contradiction`, `exhausted=False` | 5 | 4 943 | 200 ms |
| 6 | `Contradiction`, `exhausted=False` | 6 | 9 948 | 397 ms |
| 7 | `Contradiction`, `exhausted=False` | 7 | 16 383 | 672 ms |

`layers_explored` tracks `-m` exactly and the enterings roughly double per
layer. Wall is one cold process each, so the first two rows are mostly
start-up (1.3 ms of it). **No depth changes the verdict, and the reason is
structural**: `complete(kb)` asks whether the generator proposes anything
([design/07](../../docs/history/m1a_rust/design/07_search_layer.md)), and on a fixture that
closes no domain it always does — so no node is ever a solution node, at any
cap, and `k` is 0 all the way down.

And the refutation names nothing — the same run's unsat core is **empty**:

```text
solve · examples/features/02_star_in_identifiers.ein
──────────────────────────────────────────────────────────────
  solutions (k)   0
  verdict         No solution — the constraints are contradictory

  unsat core (0 facts)
```

The other nine carry the same signature at the default cap:

| entry | verdict | `layers` | enterings |
|---|---|---:|---:|
| `branching/07_lookahead_off` | `Contradiction`, `exhausted=False` | 5 | 11 501 |
| `features/01_not_and_absent` | `Contradiction`, `exhausted=False` | 5 | 384 167 |
| `features/05_stdlib_domain_elim` | `Contradiction`, `exhausted=False` | 5 | 384 167 |
| `saturation/square-{bwd,fwd}/*` (6) | `Contradiction`, `exhausted=False` | 5 | 21 699 |

The engine is not proving unsat. **It is running out of commitment-set depth
and reporting `Contradiction` anyway**, and the two paths cost the same because
no node is ever *complete*, so `stop_after = 1` never has anything to stop at.

### Where the word comes from

Two functions, and only one of them looks at the cap
([`solve.rs`](../../ein.rs/crates/ein-infer/src/solve.rs)):

```rust
if layer == self.opts.max_set_size {
    // A non-empty frontier at the depth cap means the lattice was
    // not fully explored.
    self.lstate.alive_at_end = a_layer.clone();
    self.lstate.truncated = true;
}
…
fn finalise(&mut self) -> Answer {
    self.stats.solution_nodes = self.lstate.nodes.len() as u64;
    self.stats.exhausted = !self.lstate.truncated;
    if self.lstate.nodes.is_empty() {
        return Answer::Verdict(Verdict::Contradiction { … });   // no `truncated` here
    }
```

The verdict is read from `k` alone, which is exactly what
[`verdict.rs`](../../ein.rs/crates/ein-infer/src/verdict.rs) promises — *"the
verdict is **read from the result**, never chosen up front"*. Both facts land
in the same struct and only one of them reaches the word.

### Two budgets, two vocabularies

The engine has three ways to stop early, and they do not agree:

| budget | what the caller gets |
|---|---|
| `-T` / `--max-time` | `SolveError::Budget` → the CLI prints `** aborted: max-time (0.05s) exceeded **` and exits **2**; under `on_budget = "verdict"`, `Answer::Aborted` |
| `-E` / `--max-enterings` | the same — `** aborted: max-enterings (100) reached **`, exit 2 |
| `-m` / `--max-set-size` | **`Contradiction`**, exit **0**, and `render_answer` prints *"No solution — the constraints are contradictory (unsat core: …)"* |

All three are the same file at the same moment — `02_star_in_identifiers.ein`,
which has no model at any depth — and two of them decline to answer while the
third refutes it.

`Aborted` is kept outside the `Verdict` union on purpose, and `verdict.rs` says
why: *"`solution_nodes == 0` there means **unexplored**, not proven
unsatisfiable."* A depth cap is a budget in precisely that sense. The
asymmetry is visible in the rendering too: a `Solution` at `exhausted = False`
prints *"(a solution — pass `--exhaustive` to certify uniqueness)"*, and a
`Contradiction` at `exhausted = False` prints no qualifier at all.

### What the documents say, which is two things

- The **kernel** pages define the verdict by `k` and never mention exhaustion:
  `k = 0 → Contradiction` in
  [`reserved_engine_strings.md`](../../docs/kernel/inference/reserved_engine_strings.md),
  `Verdict = Solution(k=1) | Ambiguity(k>1) | Contradiction(k=0)` in
  [`architecture_and_algorithms.md`](../../docs/kernel/inference/architecture_and_algorithms.md).
  By those, the engine is correct as written.
- [`docs/api/inference.md`](../../docs/api/inference.md) says
  *"`0` → `Contradiction` — unsat (**when exhausted**)"*. That parenthetical is
  the only statement in the tree that ties the word to the cap — and it is on a
  page specifying a Python embedding API that
  [Q-M1a.23](../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  deferred and nobody is building.

So this is not a doc-versus-code defect anybody can just fix. It is a
**vocabulary that was never decided**, and measuring the corpus is what made it
visible.

### Why it is M1d's

[Q-M1d.1](#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) asks
whether the search may stop *before* the lattice is exhausted. This is the
adjacent question — **what the engine says when it stops without an answer** —
and the two share a constraint: `exhausted` means the lattice was exhausted,
and a second guarantee needs a second word.

[P1d.2](p1d.2_obligations/README.md) may dissolve it rather than answer it. A
state that knows what it still *owes* can tell "no model below depth k" from
"no model": an unsatisfied obligation with a non-empty candidate set is a state
that is **incomplete**, which is [`ideas.md`](ideas.md)'s middle outcome and a
third word for free. Ten corpus entries would then report that instead, and
none of them would be lying.

### The candidates

- **(a) Say `Aborted`.** `Contradiction` only when `exhausted`; a truncated
  `k = 0` is a budget cut. Sound, and the honest reading of the api page. It
  moves every checked-in fixture that reports one — the corpus exit golden
  (`Contradiction` exits 0, an abort exits 2), `corpus_shapes.md5`, the trace
  goldens, and [`features.md`](../../docs/kernel/inference/features.md)'s lever
  tables, which are *written in* the current word.
- **(b) Keep the verdict, qualify the rendering.** One clause — "no solution
  **found**; the lattice was not exhausted (`-m 5`)" — and `summary.json`
  already carries `exhausted`, so nothing but prose moves. Cheap, and it leaves
  a `Contradiction` that is not a refutation in the vocabulary.
- **(c) Report `Incomplete`.** P1d.2's outcome, above: a fourth word that says
  *the state owes something it can still pay*. The most work and the only one
  that makes the distinction locally decidable.

**No recommendation from this stage** — S1a.9.0 measured, and changing a
verdict word is a semantic decision that moves fixtures across the corpus.

**2026-08-24: [S1d.2.6](p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md)
owns the close** — candidate (c) with the user's word (`Open — owes n`,
from the `open` / `false` / `satisfy` triple), and the ten entries
partitioned by the openness census *before* any word moves: owes-something ⇒
`Open`, owes-nothing ⇒ the vacuous-completion question, scan-fires ⇒ today's
word was right.

### The reproducer

```sh
cargo build --release -p ein-cli
# the signature, on any of the ten
./ein.rs/target/release/ein solve examples/features/02_star_in_identifiers.ein -e
# the sweep above
for m in 2 3 4 5 6 7; do
  ./ein.rs/target/release/ein solve examples/features/02_star_in_identifiers.ein \
      -e -m $m --json-summary /tmp/m$m.json
done
# the other vocabulary, on the same file
./ein.rs/target/release/ein solve examples/features/02_star_in_identifiers.ein -e -T 0.05
```

The ten entries are `branching/07_lookahead_off`, `features/01_not_and_absent`,
`features/02_star_in_identifiers`, `features/05_stdlib_domain_elim` and the six
`saturation/square-{bwd,fwd}/*` demos. Seven of them stopped declaring `solve`
at S1a.9.0 — the fixtures are saturation demos and the run asked nothing of
them — so the reproducer above is the record of what those cells did.


## Q-M1d.7 — May a program require its own model count?

**Opened 2026-08-24**, from M1c
[S1c.1.2](../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects).
[P1d.4](p1d.4_model_set_closure/README.md) is the phase.

`:expect (or M₁ … M_k)` says *the model set is exactly these k*. A **test** may
say that. May a **puzzle**?

The two are not the same question and the difference is the phase. `(or A B)`
in a `:match` is a disjunction over premises — it says this world satisfies A
or B, and any world that does satisfies it. Nothing in the rule language
quantifies over models, so "and there are no others" is not a sentence a
program can contain. The same s-expression therefore means satisfaction in one
keyword and enumeration-closure in another, which is either a defect to fix or
a boundary to state.

**The prior is that it is a boundary**, and shared: an ASP program's aggregates
count within an answer set, never over answer sets, and projected model
counting is an operation *on* a program rather than a sentence *in* one. If
that is the answer here too, then the meta level is where the claim belongs and
`:expect` is already the right home for it — but the reason should be written
once rather than rediscovered per keyword, which is the same treatment
[Q-M1d.2](#q-m1d2--where-does-a-requirement-live) gives obligations.

**What makes it urgent rather than academic** is affordability, not taste.
Closure is verified by exhausting the lattice, and
[the milestone's opening measurement](README.md#the-two-halves-of-one-question)
is that `zebra2-minus-15`'s 32 models are all found by depth 3 while depths 4
and 5 exist only to prove there are no more. So the one puzzle M1c's pipeline
names — *Clingo establishes 32, the answer is checked in as an `:expect`, and
`ein test` re-checks it with no solver installed* — has a claim that can be
written and not verified. `Outcome::NotChecked` is that state made honest; it
is not that state resolved.

Three shapes if the answer is "no, a program may not, and here is what a test
does instead", none of them free and all of them P1d.4's to weigh: a vocabulary
that separates *at least these* from *exactly these*; a **certificate** naming
who established the count, which is the sidecar Q-M1c.1 rejected arriving by
another door; or a bound from [P1d.2](p1d.2_obligations/README.md)'s
obligations, where a state that owes nothing may know its own count without
enumerating.
