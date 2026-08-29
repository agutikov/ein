# F18 — World-aware negatives

**Filed 2026-08-28**, from
[M1e](../m1e_review_processing/README.md)'s
[Q-M1e.9](../m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)
— option **C** of
[D4](../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md),
where B (a diagnostic) was taken now and this was filed as the real fix.

**Trigger:** when a program that needs the shape shows up, or when
[S1f.10.8](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
rules that a refutation resting on an `absent` stays **legal**. If that stage
instead forbids the shape, this theme is **closed without being done** — there
is nothing left to be careful about — and that is the cheapest outcome
available.

## The theme

`dead` is not upward-closed under `absent`. `sat` is *inflationary* — the KB is
append-only and nothing retracts — but that is not the same as *monotone in its
input*, and `absent` is exactly what separates them
([`absent_semantics.md`](../../docs/kernel/inference/absent_semantics.md) C3:
*removing a fact can flip an absent and fabricate a contradiction the full KB
never had*).

Three shipped mechanisms write a negative and never revisit it:

- the **lookahead kill cache** — `(not h)` for a candidate that dies in one
  firing against *this* state, provenance `<lookahead-dies-immediately>`, **no
  premises**;
- the **singleton writeback** — `(not h)` at root when `{h}` dies;
- the **no-good store** — a width-1 clause that removes every superset.

Each is sound while the `absent` its derivation passed through stays true, and
each is permanent.

## What the fix is

Make the three consumers **world-aware**: no kill-cache write for a lookahead
whose firing used an `absent`; no singleton writeback for such a death; a
no-good clause **tagged with its `absent` premises** and not applied where
those no longer hold.

**The starting point already exists.** `Prov::absent` has recorded the negative
premises since S1.21.8, and the repo's own note on it says *"the dependence is
visible … but no walk yet interprets it"*. This would be the first walk that
does.

## Why it is parked

- It reshapes the no-good store mid-milestone, which M1e declined for a reason.
- Its whole benefit may be unnecessary: if the language forbids concluding
  `(false)` or a `(not …)` under an `absent` over a relation the search can
  still extend, no negative can rest on a premise that flips, and there is
  nothing to make world-aware. `std.algebra`'s `total` already shows the same
  constraint written the other way, demanding a **stored negative** for every
  candidate before it concludes — so the constructive half of the alternative
  is in the tree already.
- The exposure is bounded and measured: of 60 syntactically matching rules
  across `stdlib/`, `examples/` and `tests/`, the safe ones read a **given**
  structure, and the one stdlib rule with the exposed shape is `connex`.

## Read before proposing it

- [D4](../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)
  — the probe, the six-configuration matrix, and the three consumers separated
  so that each is shown sufficient on its own.
- [`docs/kernel/inference/absent_semantics.md`](../../docs/kernel/inference/absent_semantics.md)
  — C1–C6, and C3 in particular.
- [design/08 § The objects](../../docs/history/m1a_rust/design/08_parallelism.md)
  — the monotonicity definition this theme exists because of, and the worked
  example in
  [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md)
  of what it cost to leave it unenforced.
