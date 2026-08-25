# S1d.2.2 — Domains: what a requirement quantifies over, and what closes it

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1d.2.1](s1d.2.1_property_audit.md)
**Status: done 2026-08-25** — the contract is
[`domain_contract.md`](domain_contract.md); the closed-and-owing corner is
two checked-in fixtures under `tests/stdlib/closure/`;
[Q-M1d.3](../open_questions.md#q-m1d3--what-closes-a-domain) is answered for
obligations. What it found:

- **C4, the clause the plan did not have.** Stating and discharging need no
  closure (C1/C2, structural), refutation keeps the scans (C3) — but a
  *branch* over an obligation's candidates is jointly exhaustive only where
  the guard's scanned relation cannot itself be guessed. Measured over every
  entry: **49 search, 13 hrule-driven, 12 propose `is-a` arrows — all 12
  blind, none hrule-driven.** The rung branches on the closed side of that
  partition and declines on the other.
- **"An open domain" was the wrong name for it.** No corpus entry has an
  infinite or growing domain: every `is-a` extent is authored and no rule
  anywhere asserts an `is-a` fact. What is unbounded is the *hypothesis
  space*, and `04_open`'s 14.3 GB is the subset lattice over it — `C(81, 5)`.
- **Obligations decline the open regime; they do not rescue it.** `04_open`
  and the three `square-unique` demos are among the twelve, so C4 refuses to
  branch exactly where the blind generator explodes. Where it does branch,
  the win is the size: 3 candidates against 81 arrows and `C(81, k)`.
- **Closed-and-owing: "leave it to the scans" is not conservative, it is
  wrong.** `03_closed_and_owing.ein` is `02_closed_and_satisfied.ein` with
  one fact deleted, and both report `Solution` — a state that owes something
  it can never pay, called a model. The scans *cannot* reach it: closure
  forbids the hypothesis and `total`'s stored-negative discipline means no
  negative ever arrives. The rule written: report it with an **unreachable**
  flag, promote to `(false)` never in this phase — S1d.2.6 owns that, and
  this fixture is what it has to move.

## Context

The phase README called this "the one that can sink the rest". Most of the
sinking risk is already gone, because the decided form answers the domain
question the way `forall` always has: **the witness domain is the
obligation's own guard** — the `?isa`-parameterised membership scan standing
beside the witness step inside the rule's `absent`, is-a-free,
activator-bound, no type-directed quantification in the kernel
([S1.7.23](../../../docs/history/m1a_rust/README.md) holds). Nothing about
*stating* or *discharging* an obligation needs a closed domain: discharge is
the guard ceasing to match, a positive check the matcher already performs.

**Sharpened 2026-08-25 with the form**: `(open ?R)` names the incomplete
relation and nothing else, so the domain is never *restated* — the stage's
contract is about the conjuncts the guard already holds, and
[S1d.2.3](s1d.2.3_the_form.md) item 3's three resolution rules are how the
engine tells the domain scan from the witness step.

What remains is everything that needs the domain's *extent*, and
[Q-M1d.3](../open_questions.md#q-m1d3--what-closes-a-domain)'s residue:

1. **Refutation.** "The requirement is unreachable" quantifies over the whole
   extent. Decided: it stays where it is — the `forall`-over-`(not …)` scans
   (`total`, `no-room-left`), unchanged this phase. This stage writes down
   *why* that division is sound: the tally may under-claim (report *open*
   where a scan would prove *false*) but never over-claims, and `(false)`
   outranks the tally in the read-out.

   **One of the six is not extension-safe, and the contract must say so** —
   [the audit](property_audit.md) F2. Five of the `≥` refutations scan for a
   *stored negative* per candidate and so fire only on genuine
   unreachability; `connex` scans for *absence* and fires `(false)` on an
   empty state where `total` reports `Solution` (measured, same shape, both
   directions). `std.algebra`'s header already documents it as opt-in — sound
   only where the operand is saturation-determined — and this contract
   inherits that caveat verbatim rather than papering over it: against
   `connex` the "`(false)` outranks the tally" rule outranks a verdict that
   was itself open-world-naive, so the division is sound only under the same
   opt-in the module already demands.
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
