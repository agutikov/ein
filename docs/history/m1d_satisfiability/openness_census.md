# The openness census — what the corpus owes, and who is judged by discharge

**Stage:** [S1d.2.6](README.md#s1d26--verdicts-counters-corpus) · **Phase:** [P1d.2](README.md)
**Taken:** 2026-08-25, over all 197 `corpus.toml` entries.
**Instrument:** [`utils/openness_census.py`](../../../utils/openness_census.py), reading
`--json-summary`'s `owes` block — the engine's own tally, which
[T1d.2.4.5](README.md#s1d24--obligations-in-the-saturator) proved against a hand
count. Nothing here re-derives a debt from the event stream.
**Re-take:** `utils/openness_census.py --json c.json`.

The [scope rule](README.md#s1d26--verdicts-counters-corpus) is a claim about the
corpus — *a program that states no obligation keeps today's verdict* — and a
rule stated that way is worth what the count behind it is worth. This is the
count. It is also the evidence
[Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
closes on, because the ten entries that question is about turn out to be
decided by it rather than by the new word.

---

## 1. The three numbers, and why the third is separate

| | what it is | where it comes from |
|---|---|---|
| `declared` | how many **obligation rules the program states** | `owes.declared`, added by S1d.2.6 |
| `root` | what the initial fixpoint **owes** | `owes.root.total`, S1d.2.4 |
| `models` | what each **recorded node** owes | `owes.models[]`, S1d.2.4 |

`declared` is the one nothing reported before, and it is not inferable from
the other two. **`owes = 0` is equally true of a debt paid and of a debt never
stated**, and only the first may be called *satisfied by discharge* — so a
read-out that asked `root == 0` would call 92 programs satisfied on the
strength of their silence. That is the whole reason the scope rule needs a
third number rather than a threshold on the second.

---

## 2. The partition

| class | entries | |
|---|---:|---|
| **out-of-scope** | **92** | states no obligation — judged by exhaustion, word unchanged |
| **discharged** | **12** | states one and owes nothing — satisfied *by discharge* |
| **owing** | **17** | states one and owes something |
| unmeasured | 47 | no fixpoint to read: `load-negative` (37), `parse-negative` (4), `compile-negative` (3), `regression` (3) — every one a fixture that exists to fail loading |
| not-run | 29 | the manifest declares no `solve` run (`features/04_open` and the `square-unique` demos among them: an open hypothesis space, where the run ends in the OOM killer rather than a verdict) |
| **total** | **197** | |

**121 entries reach a fixpoint and 29 of them are in scope** — 24 %, and the
other 92 are the scope rule's subject. The 27 in-scope-and-interesting split
12 / 17 between discharged and owing, with 263 outstanding instances between
them.

---

## 3. The scope rule holds

The acceptance bullet, as a table — verdict words by scope:

| scope | `Solution` | `Open` | `Contradiction` |
|---|---:|---:|---:|
| out-of-scope | 66 | **0** | 26 |
| discharged | 11 | **0** | 1 |
| owing | 5 | **12** | 0 |

**Zero `Open` outside the owing class, and zero words moved outside it.** All
92 out-of-scope entries report exactly what they reported before P1d.2, which
is the claim; the 26 `Contradiction`s there include the ten
[Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
is about, and §5 is why.

Two rows are worth reading twice.

**`discharged` has one `Contradiction`**, and it is
[`examples/ein-bugs/zebra2-bad.ein`](../../../examples/ein-bugs/zebra2-bad.ein)
— the clue-added variant, where a scan genuinely fires. It owes nothing
because a dead root's debts are unobservable (the read-out consults `(false)`
first, S1d.2.4), and its word is right for the reason it was always right:
**`false` outranks**. A state that derived a refutation is refuted whatever it
owes.

**`owing` has five `Solution`s**, and they are the whole zebra family:

| entry | `root` owes | `models` owe |
|---|---:|---:|
| `examples/zebra.ein` | 36 | **0** |
| `examples/zebra2.ein` | 36 | **0** |
| `examples/zebra2-obligations.ein` | 36 | **0** |
| `examples/zebra2-minus-15.ein` | 46 | **0** |
| `examples/zebra2-minus-15-obligations.ein` | 46 | **0** |

This is the read-out working, not an exception to it. A puzzle *starts* owing
— that is what makes it a puzzle — and the search's job is to discharge it.
The verdict reads the **recorded node's** tally, not root's, so a solved zebra
is `Solution` because its model owes nothing, and the 36 at root is the size
of the question rather than a complaint about the answer.

---

## 4. The twelve that moved

Every one is under `tests/stdlib/`, and every one reported `Solution` before
this stage:

| entry | `declared` | owes | on |
|---|---:|---:|---|
| [`algebra/23_total_owed.ein`](../../../tests/stdlib/algebra/23_total_owed.ein) | 1 | 1 | `likes` |
| [`algebra/25_surjective_owed.ein`](../../../tests/stdlib/algebra/25_surjective_owed.ein) | 1 | 1 | `likes` |
| [`bijection/01_setup_and_negatives.ein`](../../../tests/stdlib/bijection/01_setup_and_negatives.ein) | 2 | 4 | `wears` |
| [`bijection/02_domain_elimination.ein`](../../../tests/stdlib/bijection/02_domain_elimination.ein) | 2 | 4 | `wears` |
| [`bijection/03_range_elimination.ein`](../../../tests/stdlib/bijection/03_range_elimination.ein) | 2 | 4 | `wears` |
| [`closure/03_closed_and_owing.ein`](../../../tests/stdlib/closure/03_closed_and_owing.ein) | 1 | 1 | `r` |
| [`slots/02_negative.ein`](../../../tests/stdlib/slots/02_negative.ein) | 2 | 10 | `co-loc` |
| [`slots/03_fill.ein`](../../../tests/stdlib/slots/03_fill.ein) | 2 | 10 | `co-loc` |
| [`slots/04_elimination.ein`](../../../tests/stdlib/slots/04_elimination.ein) | 2 | 10 | `co-loc` |
| [`slots/07_spatial_prune.ein`](../../../tests/stdlib/slots/07_spatial_prune.ein) | 2 | 12 | `co-loc` |
| [`slots/09_owed_room.ein`](../../../tests/stdlib/slots/09_owed_room.ein) | 1 | 3 | `co-loc` |
| [`slots/11_owed_fill.ein`](../../../tests/stdlib/slots/11_owed_fill.ein) | 1 | 3 | `co-loc` |

**All twelve owe at the model as well as at root**, which is the corner: a
node the generator called complete and the tally calls unfinished. Eleven owe
because `:no-hypothesis` names the relation they owe — the rung reports them
`stuck` ([S1d.2.5](hypotheses_from_obligations.md) §5.1) — and the twelfth is
the closed-and-owing pair's owing half, which reached this census by declaring
an obligation for the first time (§6).

**No entry is in the mixed regime** — some nodes discharged, some owing — so
the read-out's rule that a discharged model outranks an open state is defined
and unexercised. It is written down anyway, because the alternative is
discovering at P1d.10 that the arm was never chosen.

### 4.1 What did *not* move

`stats.solution_nodes`, on any of them. The read-out changed and the **search
did not**: a node the generator calls complete is still recorded, still
counted, still stops `stop_after`. So `verdict.k` and `stats.solution_nodes`
now disagree on exactly these twelve, by exactly the open states, and that
disagreement is asserted rather than tolerated —
`summary_properties.rs`'s identity is conditional on the word and a second
identity pins the `Open` case from the other side.

Costs are therefore unchanged too, which is the point of putting the partition
in `finalise` and nowhere else.

---

## 5. The ten, partitioned — and the answer is the scope rule

[T1d.2.6.2](README.md#s1d26--verdicts-counters-corpus) asked for Q-M1d.6's ten
entries classified by this census before any word moved. Measured under
`solve -e`:

| entry | verdict | `k` | `exhausted` | `declared` | owes | class |
|---|---|---:|---|---:|---:|---|
| `branching/07_lookahead_off` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |
| `features/01_not_and_absent` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |
| `features/02_star_in_identifiers` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |
| `features/05_stdlib_domain_elim` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |
| `saturation/square-bwd/{floors,houses,meetings}` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |
| `saturation/square-fwd/{floors,houses,meetings}` | `Contradiction` | 0 | false | **0** | 0 | out-of-scope |

**Ten of ten state no obligation, so ten of ten keep their word.** The
classification the task named — *owes something reachable* / *owes nothing* /
*genuinely dead* — resolves for all of them to the second, in its sharpest
form: they owe nothing because **nothing ever told them what they owe**.

`features/05_stdlib_domain_elim` is the one worth checking by hand, because it
*does* declare `(total color-of 0)` and `(total color-of 1)`. Those are the
arity-2 activators of the **scan**, and it imports `std.elim` and `std.macro`
— not `std.algebra`'s `total-owed`. A program can state a lower bound and
still state no obligation; the two are different rules and the census counts
the second.

So Q-M1d.6 is not answered by the new word, it is answered by the rule that
scopes it: **the engine now never says `Contradiction` of a state that owes
something it can still pay, and these ten do not owe anything, because they
never said so.** Their `Contradiction` at `exhausted = false` remains what
[Q-M1d.1](open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)
is about — a depth cap, not a refutation — which is a different question and
still open.

**That is a finding, not a dodge.** The ten stop being a vocabulary problem
and become an *authoring* one: a fixture that wants to be told "no model
within the cap" rather than "no model" can say so by declaring what it
requires, and the four `std` obligation rules are how. Whether the engine
should also grow a word for a truncated `k = 0` — candidate (b) of the
original three — is untouched here and belongs to
[P1d.10](README.md#p1d10--exhaustive-search).

---

## 6. The pair S1d.2.2 banked, cashed

[`domain_contract.md` §3](domain_contract.md) banked
`closure/02_closed_and_satisfied.ein` and `03_closed_and_owing.ein` — the same
program one fact apart, **both reporting `Solution`** — and wrote: *"Promotion
to `(false)` is a verdict change, it belongs to S1d.2.6, and
`03_closed_and_owing.ein` is the fixture that stage must move."*

**Decided by the user 2026-08-25: the word is `Open`, and it is still not
`(false)`.** Two reasons, and the contract had already written the second:

- `(false)` is a **derived** refutation — a rule fired and named a conflict —
  and no rule fires here. Promoting on *the debt is unpayable* is a
  closed-world inference the engine declines everywhere else, and declining it
  is what [C3](domain_contract.md) is about.
- It would erase the distinction the contract called strictly more informative
  than the verdict: *owes, and cannot pay* against *owes, and might*. That
  distinction survives, in the rung report — `mode=stuck  owed=1  branches=0
  declined=1`, the `declined` being `r`'s closure scoping it out of generation.

**What moved the fixture is one line, and not the verdict change**:
`(total-owed r is-a)`. Ingredient 3 of the corner was *"`complete(kb)` meaning
the generator proposes nothing is what the engine has, because it has no
vocabulary for what a state owes"* — and S1d.2.4 gave it one. The scope rule
means the vocabulary reaches a program that **uses** it, so declaring the dual
is what put the pair in scope, and nothing else would have. Both halves carry
it now, so they are still one fact apart:

| | states | owes | verdict |
|---|---:|---:|---|
| `02_closed_and_satisfied.ein` | 1 | 0 | `Solution` |
| `03_closed_and_owing.ein` | 1 | 1 | **`Open` — owes 1 (r: 1)** |

The corner is closed by the pair *differing*, which is what it was banked for.

---

## 7. What the corpus paid

| | |
|---|---:|
| shape digests, before / after | 8 171 / 8 171 |
| **new** | **0** |
| **removed** | **0** |
| changed | 193 |
| …the 11 verdict-bearing views × 13 in-scope entries | 143 |
| …the closure pair's other 25 views, whose *text* changed | 50 |
| exit cells changed | **0** |

The 11 views that moved are exactly the ones that print a verdict —
`solve[default|exhaustive|shuffled]`, `trace[trace|answer|no-proof]`,
`dump[monotonic|lattice|progress|abort|snapshot]`. Every `dot`, `ir`, `load`,
`saturate`, `hyp`, `commit` and `explain` digest of every entry whose text this
stage did not edit is byte-identical, which is what "the read-out moved and the
search did not" looks like in a golden.

**No exit code moved**, and that is by construction: `Open` exits 0 where
`Solution` did, and the *claim* channel is `:expect`, unchanged. All 56
`tests/stdlib/` expectations still hold, including the twelve now checked
against an open state rather than a model — because all three `:expect` forms
are assertions about **facts**, and the facts a state reached are the facts it
reached whatever the verdict calls it.

---

## 8. What this closes and what it leaves

| | |
|---|---|
| **the scope rule** | **holds, measured**: 92 out-of-scope entries, 0 words moved, and `declared` is why it is checkable at all |
| **[Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)** | **closed** — `Contradiction` is never again said of a state that owes something it can still pay; the ten are out of scope and §5 is the reason |
| the closed-and-owing corner | **cashed**: the pair differs by a word, and the word is `Open` |
| `verdict.k` vs `stats.solution_nodes` | **split on purpose**, on 12 entries, with both halves pinned as identities |
| the mixed regime | **defined, unexercised** — no entry has both a discharged and an owing node |
| a truncated `k = 0` | **untouched.** `exhausted` still means the lattice ([Q-M1d.1](open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)), and the ten still report `Contradiction` at `exhausted = false` |
| `:expect` for an open verdict | **not grown**, deliberately — [P1d.4](README.md#p1d4--closing-the-model-set)'s if anyone wants it |
