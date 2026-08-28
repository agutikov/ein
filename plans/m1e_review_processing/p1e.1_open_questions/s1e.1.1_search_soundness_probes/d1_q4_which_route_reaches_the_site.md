# D1 — Q4 answered: the alive-∅ path records a state its own rules refute

> **This one is no longer a question.** It was filed as *which of two routes
> reaches the unguarded `record_node`*; both were built on 2026-08-28 against
> `a3f4e7b`, and the answer is **Route B, at shipped defaults, in 16 lines of
> program** — plus a cheaper witness at a **second site the finding does not
> mention**. What is left to decide is which fix
> [S1e.3.1](../../p1e.3_medium/s1e.3.1_correctness.md) takes and what the
> banked fixture claims, not whether [CO-M1](../../README.md#the-findings) is
> real.

**Probes:** [`probes/alive_empty_interlayer.ein`](probes/alive_empty_interlayer.ein)
— Route B, the site the finding names;
[`probes/alive_empty_phase1.ein`](probes/alive_empty_phase1.ein) — the same
shape at `phase1`'s own site, 10 lines and **zero enterings**;
[`probes/run_alive_empty.sh`](probes/run_alive_empty.sh) re-takes the matrix
below.
**Blocks:** [T1e.1.1.2](README.md#task-t1e112--q4-construct-the-alive-path),
which now has a shape.
**Bears on:** `record_node`'s two callers, the singleton writeback and the
lookahead kill cache.

## Why the review's own sketch cannot work

The stage README's TODO is right, and it is worth promoting from a note to the
reason the sketch is a non-starter:

```lisp
(rule needs-p () :match (and (is-a ?x T) (absent (p ?x))) :assert (false) :priority 400)
```

`(absent (p ?x))` is true of **A and B at load**, so `needs-p` fires in the
*first* root saturation, `(false)` lands, and
[`solve.rs:1091`](../../../../ein.rs/crates/ein-infer/src/solve.rs) returns
`Phase1::Done`:

```
solutions (k)   0
verdict         No solution — the constraints are contradictory
unsat core (1 facts)   (is-a A T)
```

Zero hypotheses, zero layers, `:1544` unreached. The general statement, and
the recipe the two probes are built on: **between phase 1 and either record
site the only facts that enter root are `(not …)`** — the singleton writeback
(`solve.rs:2520`) and the kill cache
([`hypgen.rs:464`](../../../../ein.rs/crates/ein-infer/src/hypgen.rs)); a
no-good goes to a side store. So a refutation that is dormant at phase 1 and
live at the site must read **stored negatives**, never `absent`. The TODO's
*"encoding totality as a `(false)` rule is not an option"* is right about the
sketch and too strong as a conclusion: the encoding works when it is
conditioned on the negatives the search itself writes.

## Route A is not a weaker probe — it is an inert one

`solve.rs` has exactly **two** root saturations: `:1066` (phase 1) and `:2123`
(inside the cascade). `(config :enable-forced-positive false)` deletes the
second — that is, it removes the only thing that could ever put a derived
`(false)` into root on the path it was meant to open. Under Route A
`has_contradiction(root)` at `:1544` is false *by construction*, so Route A
cannot witness the defect the option table describes, and the **forced-positive
off** rows of the matrix below are identical to the default rows in every
column. The lever neither opens the path nor closes it.

## Route B, constructed

[`alive_empty_interlayer.ein`](probes/alive_empty_interlayer.ein), stock
config, no flags. `(p Z)` dies in **two** steps so the one-step lookahead
cannot pre-empt it, and `Z` sorts last so the two survivors are entered
against a root that has not grown yet:

```lisp
(rule taint () :match (p Z)   :assert (bad Z) :priority 200)   ; two-step death
(rule kill  () :match (bad Z) :assert (false) :priority 300)
(rule mutex () :match (and (not (p Z)) (p ?x)) :assert (false) :priority 350)
(rule totality ()                                              ; dormant at phase 1
  :match  (and (not (p Z)) (absent (p A)) (absent (p B))) :assert (false) :priority 400)
```

| n | event |
|---|---|
| 15, 23 | `enter (p A)` **alive** · `enter (p B)` **alive** |
| 33–35 | `enter (p Z)` dead-post → `writeback (not (p Z))` |
| 41–43 | recompute at `:1534`: `(p A)` **lookahead_killed**, `(p B)` **lookahead_killed**, `(p Z)` negated → `alive = ∅` |
| — | `:1535` promotes nothing (`while alive.len() == 1` never runs on ∅, so nothing is saturated and nothing is checked) |
| 44 | `:1550` records root → `Solution k=1 exhausted=true` |

The recorded state is `(is-a A T) (is-a B T) (is-a Z T) (not (p A)) (not (p B))
(not (p Z))`. Feed exactly that back into the same program and the same engine
answers **`Contradiction`, k = 0**.

## The same shape at a second site, and it is the cheaper one

[`alive_empty_phase1.ein`](probes/alive_empty_phase1.ein) — 10 lines, **0
enterings, 0 layers, 1 saturation**. `phase1` checks
`has_contradiction(root)` at `:1091`, calls `compute_alive` at `:1098` — which
writes the kill cache **into root** — and records root at `:1118` under

```rust
// Empty alive and no contradiction ⇒ root is itself a complete,
// consistent model — the unique solution.
```

The check ran before the write. So `CO-M1`'s scope is wrong as filed: this is
not *the inter-layer alive-∅ path*, it is **both alive-∅ record sites**, and
the one that needs no layer, no entering and no lever is the one the finding
does not name.

## What every configuration answers

`sh probes/run_alive_empty.sh`, 2026-08-28, `a3f4e7b`. Every row is `-e` and
every row reports `exhausted = true`.

| fixture | configuration | verdict | ent | recorded negatives | that state, re-saturated |
|---|---|---|---:|---|---|
| phase1 | **default** | `Solution` k=1 | 0 | `(not (p A)) (not (p B))` | **`Contradiction`** |
| phase1 | `-K` | `Solution` k=1 | 0 | — | `Solution` |
| phase1 | `-L` | `Contradiction` k=0 | 2 | — | — |
| phase1 | forced-positive off | `Solution` k=1 | 0 | `(not (p A)) (not (p B))` | **`Contradiction`** |
| interlayer | **default** | `Solution` k=1 | 3 | `(not (p A)) (not (p B)) (not (p Z))` | **`Contradiction`** |
| interlayer | `-K` | `Solution` k=1 | 3 | `(not (p Z))` | **`Contradiction`** |
| interlayer | `-L` | `Contradiction` k=0 | 4 | — | — |
| interlayer | forced-positive off | `Solution` k=1 | 3 | `(not (p A)) (not (p B)) (not (p Z))` | **`Contradiction`** |
| interlayer | singleton-writeback off | `Contradiction` k=0 | 4 | — | — |

## What is actually broken

Not *"`record_node` is missing a `has_contradiction` re-check"*. **Root is
recorded stale** — mutated after its last saturation, by writes the program
can match on. Two statements, in increasing strength:

1. **The recorded state is not a fixpoint.** A rule of the program matches it
   and has not fired, and what it would derive is `(false)`. This is
   independent of [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
   and [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model):
   whatever a model is, a *solution* is a saturated state, and this one is not
   saturated. The re-saturation column is the proof, run rather than argued.
2. **The verdict is wrong too.** By hand on the inter-layer fixture:
   `{(p Z)}` dies, so `(not (p Z))` is entailed; with it present `mutex`
   refutes every state holding any `(p ?x)` and `totality` refutes the state
   holding none — **k = 0**, which is what `-L` says. Read a stored negative
   instead as an ordinary fact only a rule may assert, and `{(p A), (p B)}` is
   consistent with its one remaining candidate dead — **k = 1, model
   `{(p A), (p B)}`**. The default reports `Solution k=1` whose model is **∅**.
   That is neither, under either reading, and the two live branches that were
   on their way to `{(p A), (p B)}` are dropped when `alive` empties.

**The corollary that changes the fix:** a `has_contradiction` re-check at
`:1544` — the outcome table's own prescription — **catches neither witness**,
because on that path the `(false)` was never derived. The guard has to be a
re-saturation, or a refusal to record a root that was written since its last
one.

## It is not a corollary of [D4](d4_q_m1e9_upward_closure.md)

The two decisions share two of D4's three writers and **no defect**. Q-M1e.9 is a negative
whose `absent` justification a later promotion invalidates; here every negative
is *sound*. The inter-layer witness's `(not (p Z))` is justified by `taint` →
`kill`, in which no `absent` appears, and the phase-1 witness's `(not (p A))`
by `no-p`, likewise. So D4's option C — no kill-cache write for a lookahead
whose firing used an `absent` — **leaves both standing**, and the `-K` row is
the proof: turning the cache off removes the fact `totality` reads without
touching why root was recorded unsaturated. **Two sites, two writers, one
shape**, and the fixes do not overlap.

## What has to be decided

Not *whether* — **which fix, and what the fixture claims.**

| | fix for S1e.3.1 to take | consequence |
|---|---|---|
| **A** | re-saturate root at both record sites, then check | correct and the smallest diff; costs a saturation per recorded root, on a path that records **one** node |
| **B** | a *dirty* bit — root written since its last saturation — and re-saturate only when set | the same answer, and it prices the common case at a branch. Also documents the invariant the sites actually need |
| **C** | stop writing into root at all: fork-local kill cache, no singleton writeback | this is D4's option C plus more, and it deletes two shipped prunes. Its own stage |
| **D** | record the *pre-write* root — snapshot before `compute_alive` | tempting and wrong: that is the state the search *started* from, which in the inter-layer fixture is not maximal under either reading of Q-M1e.6. `-K`'s phase-1 row is what it would print |

| | what the banked fixture claims | consequence |
|---|---|---|
| **i** | bank both under `examples/` with `:expect` stating **today's** answer, header naming the defect | [D6](d6_the_new_q5_fixture.md)'s policy, and M1d's precedent — `tests/stdlib/closure/02`+`03` were banked wrong so the fixing stage had to move the golden, and it did at S1d.2.6. The gate stays green and the fix is forced to declare itself |
| **ii** | leave both in `probes/`, and let S1e.3.1 bank the corpus fixture with the *right* answer when it fixes it | keeps a knowingly-false `:expect` out of the corpus. The cost is that nothing in `cargo test` mentions the defect until the fix lands |

**Recommended: B, and (i).** B because the invariant the two sites need is
*"root is at its fixpoint"*, and a dirty bit is that invariant written down
rather than a saturation bought blind; (i) because the repo has done exactly
this before and the alternative leaves a confirmed soundness defect with no
presence in the gate. **A** is the honest fallback if the dirty bit turns out
to want threading through `Kb`.

## What this does to the rest of the stage

- **T1e.1.1.2 step 1** — *"instrument or trace which corpus entries reach the
  path"* — is no longer the way in. The path is reachable from a 10-line
  program, so the corpus question is now only about *frequency*, and it can be
  answered from the `layer` event rather than an `eprintln!`.
- **T1e.1.1.2 step 2** keeps its recipe with the conditioning clause added:
  the `(false)` rule must read the negatives the search writes.
- **The outcome table selects row 1 — `fixed`** — with the prescription
  corrected from *re-check* to *re-saturate*, and the scope widened to both
  sites.
- **[D9](d9_kernel_page_overclaims.md) needs re-drafting, not qualifying.**
  `solution_semantics.md` §6's consistency conjunct fails at **stock config**,
  witnessed twice; the row that called this an *exposure* is now a defect with
  a probe.
- **Q5 / [D6](d6_the_new_q5_fixture.md)**: `alive_empty_interlayer.ein` is
  already an ON/OFF pair whose two sides differ (`Solution k=1` / `Contradiction
  k=0`), **both exhausted**, with a solution set derivable in a paragraph —
  which is what [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
  step 3 asks for and what `branching/06` cannot be. Whether one file carries
  both questions is D6's call; they are not the same defect.
- **The stage's "nothing changes a verdict" line holds.** Fixing this moves
  both fixtures to `Contradiction k=0` — the `-L` column — and a verdict change
  belongs to S1e.3.1 with the golden audit T1e.1.1.3 step 4 is already
  building.

## Related

- [D4](d4_q_m1e9_upward_closure.md) — independent, per the section above:
  every negative in both witnesses is entailed with no `absent` in its
  derivation, so Q-M1e.9's fix leaves both standing. What the two decisions
  share is the *writers*, which is why `-K` moves the recorded fact set in
  both files and in D4's.
- [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
  — the recorded fact set is a function of a **performance lever**: `-K` drops
  both negatives from the phase-1 model and two of the three from the
  inter-layer one, without changing `k` or the verdict. Same observation as D4's `-K` row, reached from the
  other site.
- Noticed while probing, **not** one of the 63 and not this decision's: the
  unsat core renders a nested fact with the ein.py value repr —
  `(not Fact(relation_name='p', args=('Z',)))` — where the model table prints
  `(not (p Z))`. If it is kept it wants its own `Q-M1e.<n>`, the way
  [D8](d8_branching06_untyped_models.md) argues for `branching/06`.
