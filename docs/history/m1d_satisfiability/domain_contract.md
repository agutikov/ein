# The domain contract — what an obligation quantifies over

**Stage:** [S1d.2.2](README.md#s1d22--the-domain-contract) · **Phase:** [P1d.2](README.md)
**Taken:** 2026-08-25, against the corpus at `04b7fe2` (180 entries).
**Cites:** [S1d.2.3](README.md#s1d23--the-form) item 3 for the form `(open ?R)` and
its three resolution rules; [`property_audit.md`](property_audit.md) for what
the existing scans do. **[S1d.2.4](README.md#s1d24--obligations-in-the-saturator)
lifts §1 into the kernel pages.**

---

## 1. The contract

### C1 — the witness domain is the guard, and stating one needs no closure

An obligation is a rule that asserts `(open ?R)` while a witness is missing.
Its domain is not written anywhere: it is the `?isa`-parameterised membership
scan standing beside the witness step **inside the rule's own `absent`**, and
[S1d.2.3](README.md#s1d23--the-form) item 3's resolution rules are how the engine
tells the two apart. Nothing about *stating* an obligation asks whether that
extent is closed, complete, or final.

The membership relation arrives as a rule parameter — `?isa`, never an `is-a`
literal — so the kernel commits to no type system
([S1.7.23](../../../docs/history/m1a_rust/README.md) holds) and a puzzle is
free to pass `is-a`, `is-a*`, or a relation of its own.

### C2 — discharge needs no closure either, because it is positive

An obligation is discharged when its rule stops matching, which happens
exactly when `∃b: G(b) ∧ B(b)` is present. That is a **positive** check, and
a positive check is monotone: a discharged obligation stays discharged under
any extension of the KB. So the report stratum
([S1d.2.4](README.md#s1d24--obligations-in-the-saturator)) makes **no closure
assumption at all** — not about the domain, not about the relation, not about
the search.

This is the clause the superseded triple could not have: `(open ?b G B)`
restated the domain in the head, where it could disagree with the guard.
`(open ?R)` points into the guard, so there is one statement and discharge is
whatever the matcher already computed.

### C3 — refutation stays with the scans, and one of them is not extension-safe

*"The requirement has become unreachable"* quantifies over the whole extent
and stays where it is: `total` / `surjective` (`std.algebra`),
`no-room-left` (`std.elim`), `slot-no-room` / `slot-no-fill` (`std.slots`),
unchanged this phase. The division is sound because the tally may
**under-claim** — report *open* where a scan would prove *false* — and never
over-claims, and `(false)` outranks the tally in the read-out.

**With one exception, and it is inherited rather than introduced.**
[`property_audit.md` F2](property_audit.md#f2--connex-is-a-lower-bound-in-the-form-total-was-written-to-avoid):
five of the six `≥` refutations scan for a *stored negative* per candidate
and so fire only on genuine unreachability; `connex` scans for *absence* and
fires `(false)` on an empty state where `total` reports `Solution`, measured
on the same shape. `std.algebra`'s header already makes `connex` opt-in —
sound only where the operand is saturation-determined — and this contract
inherits that caveat verbatim: against `connex`, "`(false)` outranks the
tally" outranks a verdict that was itself open-world-naive.

### C4 — candidates are per-quiescence, and the extent is stable only where the guard's relation is not guessable

The candidate set is `{b : G(b), B neither present nor forbidden}`,
recomputed at whatever node asks. Nothing is stored, so nothing needs
invalidating, and a membership that arrives late is seen — the "no domain
freeze" property [`obligation_forms.md` § G](obligation_forms.md) claims over
materialised clauses.

**The same property is what bounds the generator rung**, and this is the
clause the measurement below adds. A branch over one obligation's candidates
is mutually exclusive and jointly exhaustive **only if the candidate set
cannot grow underneath it**. It can grow whenever the guard's scanned
relation is itself hypothesis-eligible: a sibling commitment `(is-a b₆ B)`
creates a witness that the branch set never enumerated, so the branch set was
not exhaustive and completeness is lost — silently.

> **The rule.** The obligations rung may branch on an obligation only where
> the relation its guard scans is **not hypothesis-eligible at that node**.
> Where it is, the rung declines the obligation and falls through — to
> another obligation, or to the blind generator — and a state all of whose
> obligations are declined is *stuck*, reported, never silently complete.

Checking it is cheap and static in the ordinary case: the guard's relation is
known per activator (S1d.2.3's projection), and hypothesis-eligibility is the
query's `:hypothesis-relations` / `:no-hypothesis` scoping plus `__closed__`,
all of which the engine already consults before proposing anything.

## 2. The open-extent inventory — measured

Every corpus entry under `solve -m 1 -e --events`, layer-1 enterings counted
and grouped by the relation each commitment names. **49 of the 180 entries
search** — the same 49 the [layer census](layer_census.md)
found — of which **13 are hrule-driven** and 36 run the blind generator.

**Twelve entries propose `is-a` arrows**, and they are the entries where C4's
rule bites:

| entry | layer-1 `alive` | of which `is-a` | mode |
|---|---:|---:|---|
| `examples/branching/07_lookahead_off.ein` | 204 | 122 | blind |
| `examples/saturation/square-unique/terminus.ein` | 153 | 50 | blind |
| `examples/saturation/square-unique/corner-house.ein` | 118 | 38 | blind |
| `examples/saturation/square-unique/cul-de-sac.ein` | 114 | 37 | blind |
| `examples/features/04_open.ein` | 81 | 30 | blind |
| `examples/features/01_not_and_absent.ein` | 35 | 17 | blind |
| `examples/features/05_stdlib_domain_elim.ein` | 35 | 30 | blind |
| `examples/features/02_star_in_identifiers.ein` | 15 | 9 | blind |
| `examples/saturation/type-exclusivity/pets.ein` | 15 | 9 | blind |
| `examples/saturation/type-exclusivity/colors.ein` | 8 | 4 | blind |
| `examples/saturation/type-exclusivity/nationalities.ein` | 8 | 4 | blind |
| `examples/saturation/transitive/taxonomy.ein` | 3 | 3 | blind |

**All twelve are blind; none of the thirteen hrule-driven entries proposes a
membership.** So the partition is exactly the mode: where the puzzle says
what to guess, the guard's extent is stable by construction; where it does
not, the generator guesses over every relation it can see, `is-a` included.
For contrast, the determinate family:

| entry | layer-1 `alive` | relations proposed | `is-a`? |
|---|---:|---|---|
| `examples/zebra2.ein` | 56 | the five `*-loc` | no |
| `examples/zebra2-minus-15.ein` | 96 | the five `*-loc` | no |
| `examples/zebra.ein` | 56 | `co-located` | no |

### What "an open domain" turns out to mean

The corpus's own note on `04_open` calls its 14.3 GB OOM "a property of an
open domain and not of a budget", and Q-M1d.3 inherited that phrasing. The
measurement narrows it, and the narrowing matters:

- **no corpus entry has an infinite or growing domain.** Every `is-a` extent
  is finite and authored; no stdlib rule and no corpus rule asserts an `is-a`
  fact, so saturation never extends one.
- what is unbounded is **the hypothesis space**, and that is a different set:
  the blind generator proposes over |objects|² × |relations|, and the OOM is
  the *subset lattice* over that — `C(81, 5)` ≈ 27 M for `04_open`, `C(154,
  5)` ≈ 750 M for `terminus`.
- the only way an extent moves at all is a **committed hypothesis** that
  happens to be a membership, which is exactly the twelve rows above.

So Q-M1d.3's "may new objects appear" has a measured answer: no new objects,
but new *memberships*, and only inside the blind mode.

### The honest consequence: obligations decline the open regime, they do not rescue it

`04_open` and the three `square-unique` demos are the corpus's OOM cases, and
they are in the twelve. So C4's rule refuses to branch on an obligation
exactly where the blind generator explodes. **Obligations are not a fix for
the open-domain wall** — they are bounded where the puzzle is bounded, and
where it is not they decline and hand back to the generator that was already
failing.

What obligations do buy, where the rule permits them, is the size of the
branch. On a puzzle shaped like `04_open` but with `is-a` scoped out, a
`total`-style obligation on `house-color` would branch over the extent of
`Color` — **3 candidates** — against the blind generator's 81 arrows at layer
1 and `C(81, k)` beyond it. That is the milestone's claim in one comparison,
and it is available only on the closed side of the partition.

## 3. The closed-and-owing corner

`(__closed__ R)` tells the generator never to speculate an R-fact. A relation
both closed and owing has no witness and no way of ever getting one — the
requirement is unreachable **by construction**. Who is allowed to say
`(false)` about that?

**Measured answer today: nobody, and the state is reported as a model.** The
pair is checked in:

| fixture | shape | verdict |
|---|---|---|
| [`tests/stdlib/closure/02_closed_and_satisfied.ein`](../../../tests/stdlib/closure/02_closed_and_satisfied.ein) | closed, one authored witness — the requirement is met | `Solution`, k = 1 |
| [`tests/stdlib/closure/03_closed_and_owing.ein`](../../../tests/stdlib/closure/03_closed_and_owing.ein) | **the same file with the witness line deleted** | `Solution`, k = 1 |

Three mechanisms have to line up, and none of them is individually wrong:

1. `infer-closure` derives `(__closed__ r)` from `functional ∧ total`, so no
   hypothesis can propose the missing arrow. Opt-in and boxed-warned;
   `01_infer_closure.ein` already exhibits its completeness hazard.
2. `std.algebra/total` — the scan whose job is to notice unreachability —
   demands a **stored negative** per candidate before concluding anything.
   In a program with no negative-completion rule none ever arrives, so the
   scan is activated, correct, and permanently silent. That discipline is
   what stops it firing on every empty-yet state; C3 is why it is right.
3. `complete(kb)` means *"does the generator propose anything?"* — and the
   generator, having been closed, proposes nothing.

Complete by exhaustion; owing, by discharge. **So "leave it to the scans" is
not the conservative default the stage plan called it** — the scans cannot
reach this case, and leaving it to them leaves it wrong.

### The rule this stage writes

> **The engine does not promote closed-and-owing to `(false)` in this phase.**
> The obligation pass *reports* it: an obligation whose relation is
> `__closed__` and whose candidate set is empty is `open` and **unreachable**,
> and the unreachable flag is part of the instance's report. Promotion to
> `(false)` is a verdict change, it belongs to
> [S1d.2.6](README.md#s1d26--verdicts-counters-corpus), and
> `03_closed_and_owing.ein` is the fixture that stage must move.

Two reasons for the split rather than fixing it here. The phase is additive
until S1d.2.6 by design — no verdict word moves while the tally is being
built — and the promotion needs the tally to exist before it can be
conditioned on it. And the flag is strictly more informative than the verdict
would be: *owes, and cannot pay* distinguishes this state from *owes, and
might* , which is a distinction `(false)` erases.

## 4. What this closes and what it leaves

| | |
|---|---|
| **C1, C2** | closed — stating and discharging need no closure, and the form makes it structural rather than argued |
| **C3** | closed, with `connex`'s caveat inherited and named |
| **C4** | **new** — the stability rule, with the 12/49 partition behind it |
| the open-extent regime | **measured, and the answer is a decline**: obligations are bounded where the puzzle is, and refuse where it is not |
| closed-and-owing | **fixtures banked, rule written**: report with an unreachable flag, promote never (this phase) |
| [Q-M1d.3](open_questions.md#q-m1d3--what-closes-a-domain) | **answered for obligations.** "May new objects appear" resolves to: no new objects, new memberships only, only in blind mode, in 12 of 49 searching entries. The general lower-bound form nobody has asked for yet keeps the question open |
