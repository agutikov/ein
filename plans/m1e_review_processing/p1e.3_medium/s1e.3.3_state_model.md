# S1e.3.3 — State model (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 2 days
**Depends on:** nothing.
**Findings:** [`ST-M1`](../review/state-model/medium.md).
**Related:** [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(c) — the tree
traversal leans on the same warrant;
[Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
— this is the finding that rule was written for.

## Context

One finding, and it is the largest unenforced soundness warrant in the tree.

The **M1 alive-set invariant**
([`inference/README.md:140-187`](../../../docs/kernel/inference/README.md)):
rules assert no new objects, no new relations, and hypotheses connect existing
names only — therefore `alive` is a pure function of the closed KB. That
purity is what licenses:

- the **per-KB alive recompute** (a fork can recompute rather than inherit);
- **state-key dedup** (two KBs with the same canonical fact list are the same
  state, so one may be dropped);
- and, since M1d, the tree traversal's **exhaustiveness-by-discharge**
  argument — jointly exhaustive alternatives are only jointly exhaustive over
  a fixed universe of names.

Which is to say: the entire model-counting story — `k`, dedup, exhaustion — is
conditional on a property that **only the stdlib's conventions maintain**. A
rule library that asserts a new `(relation …)`, or derives a fact naming a
fresh symbol, invalidates the warrant with no diagnostic and no observable
symptom other than a wrong count.

The docs are honest about it — `:184-187` says the invariant should be
*promoted to a typed invariant check when F5 lands*, and
[`implementation.md:190-192`](../../../docs/kernel/inference/implementation.md)
repeats it. F5 is a followup with no schedule. Meanwhile
[M2](../../m2_nl_to_ir/README.md) plans to **generate rule modules with a
model**, which is precisely the input the convention does not cover.

## Acceptance

- A post-fixpoint check exists: derived facts' symbols ⊆ the load-time symbol
  set, and derived relations ⊆ declared ∪ auto-vivified. Behind a debug
  assertion, a diagnostic, or a `warn` event — the stage picks, but it exists.
- The check is exercised by a fixture that **violates** the invariant, so the
  check is known to fire rather than assumed to.
- The corpus passes it. If any corpus entry violates the invariant, that is
  the finding of the milestone and the stage stops to report it before
  choosing a fix.
- The docs' *"when F5 lands"* sentence is updated to say what exists now, so
  the next reader is not told the check is absent when it is present.
- The cost is measured. This runs once per fixpoint, so it should be
  invisible; *should be* is not a measurement, and the bench set exists.

## Tasks

### Task T1e.3.3.1 — State the invariant operationally ✅

Before checking it, write it as a predicate over things the engine has. The
docs' prose version is three clauses; the operational version needs to name
the exact sets and the exact moment:

- **Which symbol set is the baseline?** Everything interned at load, or
  everything *reachable from the loaded program* (the interner is global and
  holds stdlib names a program never uses)? The second is the meaningful one
  and the first is the cheap one; say which and why.
- **What counts as a new relation?** `auto-vivified` is in the invariant's
  own statement, so the check has to know which relations were vivified during
  load versus which appear first in a derived fact.
- **When is the check run?** After the root fixpoint only, or after every
  fork's fixpoint? The warrant is used per-KB (alive is recomputed per KB), so
  the honest answer is *every fixpoint*, and that is what makes the cost
  question real.

The product is three lines that a test can assert, and they belong in
[`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) beside
the other normative statements — not only in a comment, because this
invariant is *cited* by three separate mechanisms and a citation needs a
target.

### Task T1e.3.3.2 — Implement the cheap check ✅

Not F5's typed form. A comparison of two sets after the fixpoint, with the
baseline captured once at load:

```
derived_symbols ⊆ load_symbols
derived_relations ⊆ declared ∪ auto_vivified
```

Choose the failure mode deliberately. The three available shapes and what
each costs:

| shape | fires where | cost | fits when |
|---|---|---|---|
| `debug_assert!` | debug builds only, including the gate's dev-profile tests | zero in release | the violation is believed impossible |
| a `warn` event | every run with `--events` | one comparison per fixpoint | the violation is possible and diagnosable |
| a load-or-solve error | every run | same | the violation is a program bug the engine should refuse |

The review recommends the first two. The third is tempting and premature: a
third-party module that vivifies a relation may be doing something the
engine *should* support, and refusing it before M2 has asked is deciding a
question nobody has posed. Prefer the `warn` event with the assertion in
debug — it makes the property visible to `stdlib_census.py`-style tooling
without changing what any program does.

### Task T1e.3.3.3 — Break it on purpose ✅

A fixture that violates the invariant: a rule whose head names a relation no
declaration mentions, or whose head constructs a symbol not present at load.
Confirm the check fires; confirm the corpus's 197 entries do not.

Then the second, sharper fixture: a violation that produces a **wrong count**
— a program where the invariant's failure makes state-key dedup drop a state
that is genuinely distinct. If that can be built, it is the strongest possible
argument for the check and it belongs in the stage notes and in the docs'
statement of the invariant, which currently explains the *rule* without
exhibiting the *consequence*. If it cannot be built, say so; that is also
information about how much the warrant is load-bearing.

### Task T1e.3.3.4 — Re-word the docs, and tell the tree's premise about it ✅

Three places cite this invariant and one of them is new: `inference/README.md`
(the statement), `implementation.md` (the pointer), and — since M1d —
`solve.rs:889`'s *asking once is asking enough*, which is
[Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)'s subject.
The tree's premise is a **strictly stronger** claim than the alive-set
invariant (it needs the *rung mode* to be a program property, not just the
name universe), so the check landed here does not discharge it — say so at
`solve.rs:889` explicitly, or the next reader will assume it does.

Update *"when F5 lands"* to name what now exists and what F5 would still add:
a typed form that makes the violation unrepresentable rather than detected.

## Notes

Two days looks generous for what is ultimately a set comparison. It is not:
T1e.3.3.1's three questions are the stage, and getting the baseline wrong
produces a check that either never fires or fires on the stdlib. Run the
check as a **report** over the whole corpus before making it an assertion —
the same order [S1e.3.6](s1e.3.6_tests.md) uses for the drifted floors, and
for the same reason.

---

## Outcome

Taken 2026-08-31. **The check was built, and the thing it checks is false.**

| | |
|---|---|
| **`ST-M1`** | **checked** — [`ein-infer/src/invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs). Not the post-fixpoint scan the finding asked for as the *shipped* form, but the **static** one it implies: the rules' `:assert` constants, read once at load, **7 µs** on `zebra2` (84 facts, 30 rules). It is free, it is total — it answers for every run the program could have — and, measured, it finds every breach the post-fixpoint scan finds. The scan exists too and is what confirms the induction the static one rests on |
| the corpus | **2 programs break it**: `examples/ein-bugs/mixed-type-hypothesis.ein` (`Ann`, from an `hrule`) and `tests/stdlib/algebra/07_schroder.ein` (`G`, from a probe rule). Neither pays for it, and the reasons are different — the first drives the *hrule* rung, which never consults `kb.names()`; the second fires during **root** saturation, before `alive₀` is taken |
| what a breach costs | an **answer**, and the stage built the eleven lines that show it. [`alive-set-fresh-name.ein`](../../../examples/ein-bugs/alive-set-fresh-name.ein) reports `k = 0, exhausted = true`, *No solution — the constraints are contradictory*, where `{(q A Z), (q B Z)}` is a model. A refutation, not a lower bound |
| the control | [`alive-set-fresh-name-declared.ein`](../../../examples/ein-bugs/alive-set-fresh-name-declared.ein) — the same file plus **one fact**, `(seen Z)`, over a relation nothing else mentions. `Solution k = 1` over exactly that model, in seven enterings. And that one fact is also what makes the program *conform*, so the invariant and the right answer are the same condition |
| reported as | a `warn` event, category `alive-set-invariant`, at root under `--events` and nowhere else — `warn_derived_naf`'s disposition for `refutation-under-absent`, taken for its reason: a diagnostic that fires on a working example is one that gets turned off, and `07_schroder`'s probe rule is doing nothing wrong |
| the defect | filed as [Q-M1e.21](../open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about), three candidate fixes priced. **Not fixed here**: the stage's own acceptance says a corpus violation is *the finding of the milestone* and is reported before a fix is chosen |
| gate | `./run_tests.sh` green — **786 tests**. No golden moved but the two new fixtures' six cells; no counter and no verdict moved anywhere else |

### Four things the tasks did not predict

**1. The baseline the task called "cheap" is the one that says nothing.**
T1e.3.3.1 offered *everything interned at load* against *everything reachable
from the loaded program* and called the first cheap and the second meaningful.
It is stronger than that: the first is **already checked**, by
`ein-infer/tests/interning.rs`, which measures that the interner does not grow
during a search — and it is checked for an unrelated reason (a shared table
cannot be a growing one, P1a.7). `Ann` is interned at *load*, so that test
passes on the very program that breaks this invariant. A check on the interner
would have been a second copy of an existing test with a new name on it.

**2. "Declared ∪ auto-vivified" is 49 relations short.** `from_ir` vivifies an
undeclared **fact** head at load and does **not** vivify an undeclared
**rule-`:assert`** head, so a relation a rule derives and no fact states is in
no registry at all — **49** such names over 33 corpus files: `total` derived
from `(bijective …)`, `slot-endpoint-fwd`, `converse-illtyped-dom` and 45 more
stdlib activators, plus one puzzle's own `explicitly-dislikes`. Read as the
review wrote it, the check fires on the standard library on its first run. The
baseline unions the rules' assertable heads, and the asymmetry is recorded in
[`defined_behaviour.md` § 3.3](../../../docs/kernel/defined_behaviour.md).

**3. Clause 2 is closed by the type system, and has been since the port.**
*"Rules don't `:assert (relation N S₀ S₁)` declarations — the relation registry
is fixed by the ontology block"* is not something to check: `Program` is an
`Arc` every fork shares and `program_mut` panics once it is shared. What the
check can say about relations is a different and weaker thing — whether a
stored fact's head is registered — and the answer is *often not*, harmlessly,
per item 2.

**4. The sharp fixture needed no `(absent …)`, and that took two tries.** The
first encoding refuted worlds lacking `(q A Z)` with a NAF guard, which worked
— and gave the file the shape of
[Q-M1e.9](../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent),
so `refutation_under_absent`'s banked set failed and, worse, a reader could not
have told which of two mechanisms cost the answer. Two positive refutation
rules (`no-a`, `no-b`) and `(config :enable-pre-branch-lookahead false)` give
the same false refutation with nothing confounded. The flag earns its line: with
lookahead **on** the kill cache empties `alive` at root and the run records root
itself, which is a wrong *count* (1 where there are 2) rather than a wrong
verdict, and the refutation is the sharper claim to bank.

**5. The shape goldens picked the warning up for free.** `solve_shape` runs
with `--events` at `Level::Verbose` and already filters for `"warn"` lines, so
the six `solve[*]` rows of the two breaching corpus programs each grew exactly
one line and **nothing else in 8 000-odd rows moved**. That is the containment
as a fixture rather than a claim, and it is the same mechanism
`naf-upward-closure.ein` uses for `RefutationUnderAbsentWarning` — which was
not planned for here, and is the reason no separate golden was needed.

### What this stage did **not** do

- **Run the check at every fixpoint.** The task called that the honest scope
  and it is, for the post-fixpoint reading — but the static read subsumes it
  on this corpus and the dynamic scan is where the cost is (O(delta) per fork
  with `Kb::facts_from`, which this stage added, and O(root) without it). The
  scan is `pub` and the corpus test runs it; the engine does not, and the
  measurement that licenses that is in `alive_invariant.rs`'s second test.
- **Fix the two corpus programs.** `07_schroder`'s `probe-undecided` names `G`
  because the file is *about* a name in neither triple; making it a fact would
  change what the program tests. `mixed-type-hypothesis`'s `Ann` is the point
  of that fixture. Both are named in the banked set instead.
- **Decide whether a rule may name a fresh object.** That is
  [Q-M1e.21](../open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about),
  and its candidate (b) — seeding `candidate_objects` with the rules' constants
  — is a *search* change with a golden cost, which a state-model stage does not
  get to take on the strength of one fixture.
