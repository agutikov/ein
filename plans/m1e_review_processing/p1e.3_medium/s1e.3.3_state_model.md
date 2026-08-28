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

### Task T1e.3.3.1 — State the invariant operationally

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

### Task T1e.3.3.2 — Implement the cheap check

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

### Task T1e.3.3.3 — Break it on purpose

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

### Task T1e.3.3.4 — Re-word the docs, and tell the tree's premise about it

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
