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

**Q5 — the lookahead verdict flip.** With `enable_pre_branch_lookahead` off,
`examples/branching/06_lookahead_on.ein` and `examples/lattice/02_*` change
from `Solution`/`Ambiguity` to `Contradiction`. The README's Known gaps
records that *one of the two configurations is wrong today* — which is to say
a **performance lever currently decides what a complete model is**, in the
project's own phrasing. Two things are unknown: which side is right, and
whether the wrong side is pinned by a corpus golden such that fixing the
semantics is a deliberate re-bless.

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

## Acceptance

- The standard of proof is ratified and written into
  [`open_questions.md`](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
  as decided, and every later stage cites it rather than re-arguing it.
- **Q4**: either a fixture under `examples/` that reaches the alive-∅ path
  with a saturation-encoded totality rule and shows what root records, or —
  if the shape is not constructible — the invariant argument written *beside*
  `solve.rs:1528-1551` naming why. Either way `CO-M1`'s disposition is
  determined by this task, not by the later stage.
- **Q5**: the true model sets of both fixtures derived **by hand** and
  recorded in the stage's notes; the correct side named; a golden audit
  listing every corpus artefact that pins the current verdicts for those two
  entries.
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

### Task T1e.1.1.3 — Q5: derive both fixtures' model sets by hand

`examples/branching/06_lookahead_on.ein` and `examples/lattice/02_*` are small
enough to enumerate on paper — that is the whole reason this question is
answerable. Do it in that order:

1. **Enumerate.** For each fixture, write out the complete model set from the
   program text alone, independent of the engine. Record the derivation in
   the stage notes; it is the evidence, and it is the only part of this task
   the engine cannot be asked to confirm.
2. **Run both configurations.** `ein solve -e` with the lookahead lever on
   and off, `--json-summary` both times, and diff the `verdict` blocks. Name
   which side agrees with the hand derivation.
3. **Audit the goldens.** Grep the corpus manifest and the golden tree for
   every artefact that pins either entry's verdict, `k`, digest or event
   stream. The product is a list: *fixing the semantics moves these N
   goldens.* A re-bless nobody predicted is a stop; a re-bless named in
   advance in a stage file is a step.
4. **Rule.** If the lookahead-on side is right, the lever is a correctness
   requirement wearing a performance name and the README's Known gaps entry
   says so. If the off side is right, the lever is unsound and the fix is
   engine work — out of this milestone's scope, filed as a `Q-M1e.<n>` with
   the hand derivation attached, because a milestone about processing a
   review should not quietly become a milestone about fixing the search.

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
