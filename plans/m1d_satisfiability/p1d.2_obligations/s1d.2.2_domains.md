# S1d.2.2 — Domains: what a requirement quantifies over, and what closes it

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1d.2.1](s1d.2.1_property_audit.md)

## Context

The phase README called this "the one that can sink the rest". Most of the
sinking risk is already gone, because the decided form answers the domain
question the way `forall` always has: **the witness domain is the
obligation's own guard**, `(open ?b G B)` with `G` naming the
`?isa`-parameterised membership scan — is-a-free, activator-bound, no
type-directed quantification in the kernel
([S1.7.23](../../../docs/history/m1a_rust/README.md) holds). Nothing about
*stating* or *discharging* an obligation needs a closed domain: discharge is
"∃b: G ∧ B present", a positive check.

What remains is everything that needs the domain's *extent*, and
[Q-M1d.3](../open_questions.md#q-m1d3--what-closes-a-domain)'s residue:

1. **Refutation.** "The requirement is unreachable" quantifies over the whole
   extent. Decided: it stays where it is — the `forall`-over-`(not …)` scans
   (`total`, `no-room-left`), unchanged this phase. This stage writes down
   *why* that division is sound: the tally may under-claim (report *open*
   where a scan would prove *false*) but never over-claims, and `(false)`
   outranks the tally in the read-out.
2. **Candidates.** The generator rung (S1d.2.5) enumerates
   `{b : G(b), (B) neither present nor forbidden}` — finite iff `G`'s extent
   is finite *at that quiescence*. G re-evaluates per state, so a late
   `(is-a b₆ House)` extends the set (no freeze) — but an **open-ended**
   extent is the [`features/04_open`](../../../examples/features/04_open.ein)
   regime, where the blind search reaches 14.3 GB and the OOM killer
   ([Q-M1d.3](../open_questions.md#q-m1d3--what-closes-a-domain) has the
   numbers). The obligations rung must be bounded per node by construction —
   candidates as of this quiescence — and the stage says what the verdict may
   then claim (exhaustive over *what*).
3. **The closed-relation corner.** `(__closed__ R)` suppresses hypgen for R.
   A relation both closed and owing — no witness, none proposable — is
   *unreachable* by construction. Say whether the engine may promote that to
   `(false)` itself or leaves it to the scans; today's answer ("the scans")
   is the conservative default.

## Tasks

### Task T1d.2.2.1 — the domain contract, written

One page: guard-is-the-domain, discharge needs no closure, refutation keeps
the scans, candidates are per-quiescence. This is the text S1d.2.4's kernel
doc work lifts from.

### Task T1d.2.2.2 — the open-extent inventory

Which corpus entries have a guard whose extent an obligation would range over
open-endedly (the `04_open` family, the `saturation/square-unique` demos).
For each: what the tally reports there, and what the generator rung must
refuse. Measured, not argued.

### Task T1d.2.2.3 — the closed-and-owing corner

A two-fixture answer: one where `__closed__` + an obligation coexist
satisfied, one where they contradict — and the written decision of who says
`(false)`.

### Task T1d.2.2.4 — Q-M1d.3 updated

The question narrows to its § "where the closure is stated" residue; the
answer this stage banks moves it to *answered for obligations*, open for the
general lower-bound form nobody has asked for yet.

## Acceptance

- The domain contract is one written page, and S1d.2.4 cites it.
- The open-extent inventory is measured, with the tally's behaviour per entry.
- The closed-and-owing corner has its two fixtures and its one-sentence rule.
- No kernel type system: `is-a` appears in no rule body, no loader path, no
  engine index — the guard carries everything.
