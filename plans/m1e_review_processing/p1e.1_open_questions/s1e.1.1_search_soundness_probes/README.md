# S1e.1.1 — Two soundness probes: Q4 and Q5

**Phase:** [P1e.1](../README.md) (The ten questions)
**Estimate:** 2 days
**Depends on:** nothing.
**Blocks:** [CO-M1](../../p1e.3_medium/s1e.3.1_correctness.md) (Q4) and any
golden the lookahead flip pins (Q5).
**Answers:** [`review/open-questions.md`](../../review/open-questions.md) Q4 and
Q5. **`Q6` left this stage on 2026-08-28** for
[S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md);
the ruling taken here is kept in
[D2](d2_q6_which_decline_to_construct.md).

## Context

Two questions, one shape: a search path whose soundness rests on a premise
that is argued and not checked. Each is answerable by a **constructed
program** — neither needs new engine machinery to ask, and both are the kind
of probe the review could not run because it had reading budget and no
verification stage.

They are one stage because the standard of proof has to be settled once,
here, and because both end the same way: a fixture in the corpus that
would fail if the premise ever broke.

A third, **`Q6`**, was here until 2026-08-28 — see below.

**Q4 — the inter-layer alive-∅ path.** `phase2` calls `record_node(root)`
when `compute_alive` comes back empty
([`solve.rs:1528-1551`](../../../../ein.rs/crates/ein-infer/src/solve.rs)),
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
| the kill cache | `-K` keeps k=3 and the verdict on `lattice/02` and **changes the recorded fact sets**: the models lose their `(not (c-prop X))`, which `write_negated` is what writes. Not a defect under Q-M1e.6 — negatives are not part of a model — and the reason [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model) exists |

So the stage's Q5 scope is **`lattice/02` plus one purpose-built fixture**, and
`branching/06` is evidence rather than subject.

**Q6 left this stage.** The tree's inner-node rung flip — `tree()` probing the
generation-ladder mode **once at root** on the premise that *the mode is a
property of the program rather than of the node*
([`solve.rs:889-914`](../../../../ein.rs/crates/ein-infer/src/solve.rs)) — is
now [S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md),
which carries the reconnaissance this section held: the premise is already
refuted by `activators_for`'s own doc comment, the three fact-dependent decline
conditions are `oblgen.rs:232-262`'s, and the loss mechanism is `complete`
changing meaning under the blind rung rather than a non-exhaustive branch set
being walked as exhaustive. The **ruling** — the mode is re-read at every node,
2026-08-28 — stays in [D2](d2_q6_which_decline_to_construct.md), and what the
search should *do* with the new obligation is
[Q-M1e.11](../../open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis).

## Acceptance

- The standard of proof is ratified and written into
  [`open_questions.md`](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
  as decided, and every later stage cites it rather than re-arguing it.
- **Q4**: either a fixture under `examples/` that reaches the alive-∅ path
  with a saturation-encoded totality rule and shows what root records, or —
  if the shape is not constructible — the invariant argument written *beside*
  `solve.rs:1528-1551` naming why. Either way `CO-M1`'s disposition is
  determined by this task, not by the later stage.
- **Q5**: `lattice/02`'s solution set derived **by hand** against
  [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
  and recorded in the stage's notes; the correct side named; **one new
  fixture** carrying the ON/OFF pair with both sides exhausting, since
  `branching/06` cannot; a golden audit listing every corpus artefact that
  pins the current verdicts for both existing entries.
- **Every `record_node` caller has a written answer to *which of § 2's three
  conjuncts do I establish, and when?*** — beside the call, not only in a plan
  file — and the conjuncts it does not establish are named. This is
  [D3](d3_q_m1e8_file_or_take.md)'s B.
- Nothing in this stage changes a verdict. Where a probe finds a defect, the
  fix is the dependent stage's; this stage's product is the fixture and the
  ruling.

## Decisions to take before implementation

Nine files, and **two of them are now records rather than decisions**: D1 is
answered by construction and D2 has moved to [P1e.1b](../../p1e.1b_hypothesis_structure/README.md).
Of the seven that remain, none blocks a task outright — they change what the
stage delivers or are cheap enough to decide in passing, and each file carries
the options with their consequences and a recommendation.

| | decision | gates | recommended |
|---|---|---|---|
| [D1](d1_q4_which_route_reaches_the_site.md) | **answered 2026-08-28** — Route B built at stock config, and a cheaper witness at the phase-1 record site the finding does not name. Open: which fix S1e.3.1 takes, and what the banked fixture claims | **T1e.1.1.2** | the dirty-bit guard; bank the fixture stating today's answer |
| [D2](d2_q6_which_decline_to_construct.md) | **moved** — Q6 became [S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md) on 2026-08-28. What stays is the ruling taken here: the rung mode is re-read at every node | nothing here | — |
| [D3](d3_q_m1e8_file_or_take.md) | **decided 2026-08-28 — B**: the stage takes the *check*, not the fix. `record_node`'s four callers against `solution_semantics.md` § 2 | **T1e.1.1.4** | the fix still files to P1e.2, and *which* fix waits on Q-M1e.7 |
| [D4](d4_q_m1e9_upward_closure.md) | **Q-M1e.9 is reproduced: `dead` is not upward-closed under `absent`.** Five of six configurations answer a twenty-line program wrongly. Who owns it, and how far does the fix go? | nothing here — but it qualifies D3 and D9 | a load-time refusal now, the real fix filed |
| [D5](d5_does_t1_ratify_q_m1e2.md) | Does T1 ratify **Q-M1e.2** as well as Q-M1e.1? Q4's likely `accepted` needs it, and Q-M1e.2 has no owning stage | T1e.1.1.1 | ratify both |
| [D6](d6_the_new_q5_fixture.md) | Where the **new Q5 fixture** lives, what it pins, and whether its `:expect` states today's answer or the right one | T1e.1.1.3 step 3 | `examples/branching/`, state today's answer, let D3's fix move it |
| [D7](d7_the_diff_instrument.md) | The **two-config diff** exists three times already. Build a fourth, borrow, or throw one away? | T1e.1.1.3 — and now S1e.1b.6 T3 and S1e.1b.7, two more customers | throwaway here; name it in S1e.3.4's inputs |
| [D8](d8_branching06_untyped_models.md) | `branching/06` models `(co-located Blue Color)` and prints `?h = Color`. **Evidence, or its own id?** | nothing | its own `Q-M1e.<n>` |
| [D9](d9_kernel_page_overclaims.md) | `solution_semantics.md` §6 claims *the engine never records a false model*; it proves the maximality conjunct only, and D4 already dents that | should land **before** T1e.1.1.2 | qualify the row now |

**D4 is the one to read first.** It was filed as a question about two pages
disagreeing and came back as a reproduced defect in three shipped mechanisms,
which changes what several of the others are about.

## Tasks

### Task T1e.1.1.1 — Ratify the standard of proof

Half a day, and it is first because the other three tasks are its first
customers. Write
[Q-M1e.1](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
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

TODO: Encoding totality as a (false) rule is not an option because it derives contradiction on initial KB.

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
[Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
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
   per layer. File it as [Q-M1e.8](../../open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)'s
   fix with the derivation attached; rewrite the README's Known gaps entry
   from *"one of the two is wrong"* to **both under-report, and here is the
   test that does not**; and re-label `-L`'s corpus lever cells as a
   strictly-weaker debug mode rather than an A/B of equals.

The precedent for step 1 is exact and recent: `disjunctive-prune`'s wrong
`(neq ?h_other ?h1)` guard survived a year of byte-exact parity between two
engines and died to **one independent enumeration written outside the engine
on the day**.

### Task T1e.1.1.4 — The record-site conformance check

Half a day, and it is [D3](d3_q_m1e8_file_or_take.md)'s option **B**, chosen
2026-08-28: not the maximality fix — which stays filed for P1e.2 — but a
**double-check that the implementation meets
[`solution_semantics.md`](../../../../docs/kernel/inference/solution_semantics.md)
§ 2's three conjuncts.** A check changes no verdict, so the stage's acceptance
survives it, and the matrix already exists
([`probes/run_record_sites.sh`](probes/run_record_sites.sh)).

1. **The matrix.** `record_node` has four callers — `:1030`, `:1118`, `:1550`,
   `:1977`. For each, which of *saturated* / *consistent* / *maximal or
   discharged* it establishes, and **when** relative to the last write into the
   KB it records. D3 carries today's answers: the first conjunct is established
   at none of them, and the second at every one of them *before* the last
   write.
2. **The witnesses.** Three exist and re-saturate to `Contradiction`; the
   fourth (`tree_node`) is
   [S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)'s.
   Decide with [D1](d1_q4_which_route_reaches_the_site.md)'s (i)/(ii) whether
   they enter the corpus stating today's answer or stay in `probes/`.
3. **Write the conjunct each site owes beside the call**, not only here —
   [Q-M1e.1](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
   third rule.
4. **State the fix menu and its dependency.** Three candidate fixes, one of
   which covers all three witnesses, and two of which give *different `k`* on
   the same program — so the choice waits on
   [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model),
   which is unowned. Naming that is the task's last product.

The Q6 probe that used to be this task — a program deriving an obligation
activator inside a fork, run under both traversals and diffed fact for fact —
is now S1e.1b.6 T1–T4.

## Notes

These probes share an instrument with S1e.1b.6 and S1e.1b.7 — *run the same
program two ways and diff the model sets* — and it already exists in three
places
(`model_set_census.py`, the S1d.10.6 verification, the `--jobs` sweep). If
the stage finds itself writing the comparison a fourth time, that is a small
`utils/` script, not a fourth copy; but it is not the stage's job to build
one, and the milestone has a finding about exactly this habit
([AR-M1](../../README.md#the-findings)).

Neither of these questions is M1d's to answer, and this stage does not
touch `Q-M1d.1` or `Q-M1d.6` — the verdict-vocabulary questions the review's
findings repeatedly terminate in. Where a probe's answer would require one of
them, the stage records the dependency and stops.
