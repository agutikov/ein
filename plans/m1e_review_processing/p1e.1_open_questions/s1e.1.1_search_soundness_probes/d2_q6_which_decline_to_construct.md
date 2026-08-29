# D2 — Q6: moved to P1f.10 as a stage

> **This decision left the stage.** Its subject — the tree's inner-node rung
> flip, `oblgen`'s three decline conditions, and what to do about an obligation
> a fork derives — became
> [S1f.10.6](../../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)
> on 2026-08-28, on the user's instruction. Everything that was argued here is
> there, unabridged; what stays is the ruling that was taken here and the
> references, so that a reader of this folder is not sent looking.

## The ruling that was taken here

**2026-08-28, by the user: the obligation has to be re-evaluated after each
saturation** — the tree's rung mode is re-read at **every node**, not inherited
from the root probe at
[`solve.rs:889-914`](../../../../ein.rs/crates/ein-infer/src/solve.rs).

The premise it overturns — *"the mode is a property of the program rather than
of the node, so asking once is asking enough"* — was already refuted by this
repo's own doc comment on `activators_for`
([`compile.rs:54-69`](../../../../ein.rs/crates/ein-infer/src/compile.rs)): a
fork derives activators of its own during saturation.

And it is free: `tree_node` builds a `HypGenStats`, calls
`generate_one_branch`, keeps the candidate list and **drops `hs.rung.mode`**
(`solve.rs:945-956`). The change is to stop discarding a value, not to add a
call — which retires the one argument against re-probing, that it costs a
generation call per node.

## Where each part went

| | where |
|---|---|
| the guard — re-probe per node | [S1e.2.1](../../p1e.2_high/s1e.2.1_correctness.md) T3, unchanged in owner; it now *applies a ruling* rather than waiting on a probe |
| which decline condition to construct, and the C4 sketch | [S1f.10.6](../../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) T2 |
| the loss mechanism, and the two-part assertion | [S1f.10.6](../../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) § The loss mechanism |
| what the search should **do** with the new obligation | [Q-M1e.11](../../open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis), ruled on by S1f.10.6 T4 |
| the review's `Q6`, as a question of this milestone | still `Q6`; its answering stage is now S1f.10.6 |

## Why it moved

S1e.1.1 is *three soundness probes*, and Q4 and Q5 are about a search path that
exists. This one is about what a hypothesis set **is** when the theory can
extend it mid-search — which is
[P1f.10](../../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md)'s subject and nothing
else's in the milestone. The phase's founding sentence is *the search
enumerates subsets of a **fixed** `alive` set*; a derived obligation is the
case where it is not fixed, and a phase that builds a load-time group structure
has to say so itself.

## Related, still here

- [D1](d1_q4_which_route_reaches_the_site.md) — the same defect class from the
  other end: a decision computed about a KB that then changed, and reused
  anyway. Root's consistency, checked at `:1091` and reused at `:1118`; the
  rung mode, probed at root and reused at every node.
- [D7](d7_the_diff_instrument.md) — the two-config diff. S1f.10.6 T3 and
  S1f.10.7 are two more customers for it, which strengthens the case D7 makes.
- [D8](d8_branching06_untyped_models.md) — `branching/06`'s junk candidates are
  what make `complete` change meaning under the blind rung, which is S1f.10.6's
  loss mechanism.
