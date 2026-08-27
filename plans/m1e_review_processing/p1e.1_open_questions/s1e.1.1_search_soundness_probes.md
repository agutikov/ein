# S1e.1.1 — Three soundness probes: Q4, Q5, Q6

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 3 days
**Depends on:** nothing.
**Blocks:** [CO-M1](../p1e.3_medium/s1e.3.1_correctness.md) (Q4),
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(c) (Q6), and any golden the
lookahead flip pins (Q5).
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q4, Q5,
Q6.

## Context

Three questions, one shape: a search path whose soundness rests on a premise
that is argued and not checked. Each is answerable by a **constructed
program** — none needs new engine machinery to ask, and all three are the kind
of probe the review could not run because it had reading budget and no
verification stage.

They are one stage because the standard of proof has to be settled once,
here, and because all three end the same way: a fixture in the corpus that
would fail if the premise ever broke.

**Q4 — the inter-layer alive-∅ path.** `phase2` calls `record_node(root)`
when `compute_alive` comes back empty
([`solve.rs:1528-1551`](../../../ein.rs/crates/ein-infer/src/solve.rs)),
**without** the `has_contradiction` re-check that phase1 (`:1091`) and the
cascade (`:2131`) both do. The review's argument for why this is currently
safe is two-part — the writebacks are `(not h)` for `h ∉ root`, and root has
not been re-saturated on this path, so no derived `(false)` exists to detect
— and the second half is the gap: a program encoding totality as a saturation
`(false)` rule rather than as an obligation could have root recorded as a
model whose falsity only a fork would derive. With obligations declared, the
re-read tally at `:1548` catches the owing case as `Open`; without them,
nothing does.

**Reconnaissance narrows the path, and the narrowing is most of the answer.**
Reaching `record_node` at `:1550` needs `a_layer ≠ ∅` at `:1528` *and*
`alive == ∅` at `:1544` — but `promote_forced_positives` runs between them
(`:1536`) and **re-saturates root and re-checks `has_contradiction` on every
iteration** (`:2131`). So the site is reachable un-re-saturated only when
`compute_alive` returns ∅ **directly**, never passing through a singleton.
Two routes, and neither is in the review: **(A)** `(config
:enable-forced-positive false)`, which skips the cascade — reachable, but a
non-default lever; **(B)** `alive` empties because the **lookahead filter**
killed the last candidates against a root that grew, which needs no lever and
means **Q4 and Q5 are the same mechanism seen from two places.** T2's outcome
table stands; which route the fixture takes decides whether `CO-M1` is a bug
at shipped defaults or a guard for a shape only a config line reaches.

**Q5 — the lookahead verdict flip.** With `enable_pre_branch_lookahead` off,
`examples/branching/06_lookahead_on.ein` and `examples/lattice/02_*` change
from `Solution`/`Ambiguity` to `Contradiction`. The README's Known gaps
records that *one of the two configurations is wrong today* — which is to say
a **performance lever currently decides what a complete model is**, in the
project's own phrasing.

**Reconnaissance, 2026-08-27, against `7731848` — three of this stage's
premises did not survive it.** Recorded here because the tasks below are
written against what was found, not against what the review assumed:

| | measured |
|---|---|
| `lattice/02` | `-e` → **Ambiguity k=3, `exhausted = true`**, 6 enterings · `-e -L` → **Contradiction k=0, `exhausted = true`**, 7 enterings, 3-fact core. Both sides *complete*, and the OFF side prints *"No solution — the constraints are contradictory"* unhedged |
| `branching/06` / `07` | `-e` → k=22 · `07 -e` → k=0, and **both report `exhausted = false` at the depth-5 cap**. The pair compares two lower bounds, not two answers |
| `branching/06`'s model set | `--models key`: **8 of 42 varying slots**, `C(42,8) = 118 030 185`, over budget. 20 of the 22 models bind `?h` to `Color` or `House` — the **G3 blind rung is untyped**, `(is-a Color T)` makes `Color` an object, and model 1 contains `(co-located Blue Color)`. It is **not** "small enough to enumerate on paper" |
| the kill cache | `-K` keeps k=3 and the verdict on `lattice/02` and **changes the recorded fact sets**: the models lose their `(not (c-prop X))`, which `write_negated` is what writes. Not a defect under Q-M1e.6 — negatives are not part of a model — and the reason [Q-M1e.7](../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model) exists |

So the stage's Q5 scope is **`lattice/02` plus one purpose-built fixture**, and
`branching/06` is evidence rather than subject.

**Q6 — the tree's inner-node rung flip.** `tree()` probes the
generation-ladder mode **once at root**, on the stated premise that *the mode
is a property of the program rather than of the node*
([`solve.rs:889-914`](../../../ein.rs/crates/ein-infer/src/solve.rs)). But
oblgen's mode per node is a function of activator **facts**, and an activator
is an ordinary fact a rule can derive — inside a fork. A flip at an inner node
falls through to the blind enumerator
([`hypgen.rs:340-378`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)), whose
branches are **not** jointly exhaustive, and the tree would then treat a
non-exhaustive branch set as exhaustive and **miss models** — the failure
class this project's discipline treats as worst. Today's stdlib activators are
root-asserted, so the corpus never reaches it. The question is whether the
shape is *constructible at all*, because that is the difference between a
`debug_assert` and a re-probe per node.

**Two corrections from reconnaissance.** First, **the premise is already
refuted by this repo's own doc comment**: `activators_for`
([`compile.rs:54-69`](../../../ein.rs/crates/ein-infer/src/compile.rs)) says a
parameterised rule *"consults the **fork's** `rule_apps_by_rule`, not the
load-time KB's, because a fork derives activators of its own during
saturation"*, and `oblgen::generate` calls `plans_for(s.kb, …)` per node. The
mode **is** a function of the node. What is open is only whether a *flip* is
constructible, and the three fact-dependent decline conditions are
[`oblgen.rs:232-262`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)'s: a
bare `(open)` plan, a projection that will not resolve for that activator, and
**C4** — an obligation scanning a relation the rung itself proposes. C4 is the
likeliest, and the probe is a rule that derives, under a hypothesis, an
activator for an obligation on a relation an existing obligation's guard
scans.

Second, **the loss mechanism is not the one the finding states.** The tree
enters *every* candidate `G3` returns and recurses, so subsets stay reachable;
what is lost first is the `d!`-per-path explosion, because `one_branch` is a
parameter the blind rung ignores. The **missed model** comes from `complete`
changing meaning: a node whose obligations are discharged is a solution under
`G2` and is *not* complete under `G3` while the blind enumerator still
proposes anything — and `branching/06` is the standing proof that it proposes
junk long after any debt is settled (`(co-located Blue Color)`). So the test
asserts *"the tree's model set ⊇ the lattice's"* **and** *"no node emitted a
`rung` event with a mode other than `obligations`"*, not a bare count.

## Acceptance

- The standard of proof is ratified and written into
  [`open_questions.md`](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
  as decided, and every later stage cites it rather than re-arguing it.
- **Q4**: either a fixture under `examples/` that reaches the alive-∅ path
  with a saturation-encoded totality rule and shows what root records, or —
  if the shape is not constructible — the invariant argument written *beside*
  `solve.rs:1528-1551` naming why. Either way `CO-M1`'s disposition is
  determined by this task, not by the later stage.
- **Q5**: `lattice/02`'s solution set derived **by hand** against
  [Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
  and recorded in the stage's notes; the correct side named; **one new
  fixture** carrying the ON/OFF pair with both sides exhausting, since
  `branching/06` cannot; a golden audit listing every corpus artefact that
  pins the current verdicts for both existing entries.
- **Q6**: a probe program with a rule deriving an obligation activator inside
  a fork, run under `EIN_TRAVERSAL=tree` and diffed against the lattice's
  model set. Constructible or not, the answer is banked as a test.
- Nothing in this stage changes a verdict. Where a probe finds a defect, the
  fix is the dependent stage's; this stage's product is the fixture and the
  ruling.

## Tasks

### Task T1e.1.1.1 — Ratify the standard of proof

Half a day, and it is first because the other three tasks are its first
customers. Write
[Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
three rules — behaviour is refuted only by a banked probe; absence is refuted
by naming the thing; risk is not refutable by argument — into the milestone's
`open_questions.md` as **decided**, with the date, and add the one-line
version to the milestone README's disposition table.

The rule that matters most in practice is the third, because it is the one
that will be argued with: *"it cannot happen"* is a written argument and it
goes **beside the code**, not into a plan file, or the next reader has the
same question with no answer at the site. The precedent is the repo's own —
`design/02` is an argument that lives where a reader of the determinism rules
finds it.

### Task T1e.1.1.2 — Q4: construct the alive-∅ path

The path is reached when `compute_alive` returns empty *between layers*. Build
the fixture in two steps rather than guessing:

1. **Find the path.** Instrument or trace which corpus entries reach
   `solve.rs:1528-1551` at all — the enterings counters plus a temporary
   `eprintln!`, or an `--events` reading if the path already narrates. If no
   corpus entry reaches it, that is itself the finding, and it changes the
   fix: an unreachable branch with an unchecked premise is dead code with a
   comment, not a soundness risk.
2. **Encode totality as a `(false)` rule.** The corpus's totality is
   `std.algebra`'s `total-owed` — an obligation, which the tally at `:1548`
   catches. The probe wants the *other* encoding: a saturation rule that
   derives `(false)` when some object has no value for a relation, with **no**
   `(open …)` declaration anywhere, so the obligations read-out is silent.
   Then drive it onto the alive-∅ path and read what root records.

Expected outcomes and what each means:

| outcome | disposition of `CO-M1` |
|---|---|
| root recorded as a model with a derivable `(false)` | **fixed** — add the `has_contradiction` re-check on this path; the fixture is the regression test |
| root correctly not recorded (the cascade fired, or the rule ran) | **refuted** — bank the fixture as the test that keeps it that way, and write the reason at the site |
| the path is unreachable from any `.ein` program | **accepted** — with the argument at `solve.rs:1528`, and a note that the branch's premise is unchecked because the branch is unreached |

The re-check is cheap and the path is rare, so *add it anyway* is a tempting
shortcut. Resist it until the fixture exists: a check added without a probe
is a check nobody can ever remove.

### Task T1e.1.1.3 — Q5: derive `lattice/02` by hand, against the ruling

The definition is no longer open:
[Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
is decided, and a hand derivation now has something to derive *against*. A
solution is a saturated consistent state in which **every remaining hypothesis
is inconsistent with the state**; a model is the positive part minus the
positive initial KB.

1. **Enumerate `lattice/02`.** Three candidates, one rule asserting `(false)`
   iff all three hold. Maximal consistent states: `{h₁,h₂}`, `{h₁,h₃}`,
   `{h₂,h₃}` — **three solutions**, each with one remaining candidate that is
   inconsistent with it, which is clause 3 satisfied. Write it out anyway,
   from the program text, outside the engine: the derivation is the evidence.
2. **Rule.** The default `k=3` agrees; `-L`'s `k=0` does not, and it says so
   with `exhausted = true`. **The OFF side is wrong**, and the general
   statement is Q-M1e.6's: `complete` is a *sound but incomplete*
   approximation of clause 3 with the lookahead on, and a strictly weaker one
   with it off. Neither is the definition.
3. **Build the fixture the pair needs.** `branching/06` cannot carry this —
   42 varying slots, untyped models, neither side exhausting. One new
   `examples/` fixture: typed, both sides exhausting, its solution set
   derivable in a paragraph, and its ON/OFF verdicts differing. It goes in the
   corpus with an `:expect`, and it is what fails if the lever ever moves.
4. **Audit the goldens.** Every artefact pinning `lattice/02` or
   `branching/06`: `corpus_exits.txt` (7 + 12 rows), `lattice_semantics.rs`
   (4 sites), `search_invariants.rs` (3), `model_set_report.rs`,
   `leftover_probe.rs`, `presentation_semantics.rs`, `corpus_shapes.md5`.
   Neither fixture carries an `:expect`. The product is a list: *fixing the
   semantics moves these N goldens.* A re-bless nobody predicted is a stop; a
   re-bless named in advance in a stage file is a step.
5. **File the fix, do not take it — and file the right one.** Q-M1e.6's
   operational form makes a solution a **maximal alive commitment**, and the
   lattice already computes maximality at layer `n+1` with no extra fork. So
   the fix is *not* "make the lookahead unconditional" (which is still an
   approximation, just a better one): it is to record a surviving commitment
   whose every superset died. Retaining that costs one bitset over `a_prev`
   per layer. File it as [Q-M1e.8](../open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)'s
   fix with the derivation attached; rewrite the README's Known gaps entry
   from *"one of the two is wrong"* to **both under-report, and here is the
   test that does not**; and re-label `-L`'s corpus lever cells as a
   strictly-weaker debug mode rather than an A/B of equals.

The precedent for step 1 is exact and recent: `disjunctive-prune`'s wrong
`(neq ?h_other ?h1)` guard survived a year of byte-exact parity between two
engines and died to **one independent enumeration written outside the engine
on the day**.

### Task T1e.1.1.4 — Q6: try to build the inner-node rung flip

The probe: a program that declares an obligation rule (so root's mode is
`Obligations` and the tree accepts the traversal), plus a saturation rule
that derives a *second* obligation's activator fact only under a hypothesis —
so the mode at an inner node is computed over a fact set root did not have.

Two questions, in order:

1. **Is the activator derivable at all?** Activators are ordinary facts, so a
   rule head can produce one; whether the loader and `oblgen` read a
   *derived* activator the same way they read a declared one is the actual
   unknown ([`oblgen.rs:241-265`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)).
   Check this first with `ein saturate --dump` before building anything on
   top of it.
2. **Does the tree then miss a model?** Run the probe under
   `EIN_TRAVERSAL=tree` and under the default lattice with `-e`, and diff the
   model sets fact for fact — the same comparison
   [S1d.10.6](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
   used to verify the 86-vs-17 204 592 result. A tree set that is a strict
   subset is the bug.

Whichever way it lands, bank it: a passing test named for the premise
(*the tree's branch sets stay jointly exhaustive*) is worth more than the
`debug_assert` it may replace, because the assert only fires in a debug build
and the test runs in the gate. Then say what `CO-H3`(c) becomes:

| outcome | what [S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md) T3 does |
|---|---|
| constructible, and the tree misses models | re-probe the mode at **every** node and hard-decline on a flip; the probe is the regression test |
| constructible, and the tree still agrees | the premise is wrong but harmless — find out why, then either the assert or a written argument |
| not constructible from any `.ein` program | `debug_assert` plus the argument at `solve.rs:889`, and the reason it is not constructible stated there |

## Notes

The three probes share an instrument — *run the same program two ways and
diff the model sets* — and it already exists in three places
(`model_set_census.py`, the S1d.10.6 verification, the `--jobs` sweep). If
the stage finds itself writing the comparison a fourth time, that is a small
`utils/` script, not a fourth copy; but it is not the stage's job to build
one, and the milestone has a finding about exactly this habit
([AR-M1](../README.md#the-findings)).

None of these three questions is M1d's to answer, and this stage does not
touch `Q-M1d.1` or `Q-M1d.6` — the verdict-vocabulary questions the review's
findings repeatedly terminate in. Where a probe's answer would require one of
them, the stage records the dependency and stops.
