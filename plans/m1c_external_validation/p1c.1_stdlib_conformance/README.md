# P1c.1 — stdlib conformance

**Milestone:** [M1c — External validation](../README.md)
**Estimate:** ~~2.5 weeks (13 days)~~ **3 weeks (15 days)** — S1c.1.4 went
4 days → 6 on 2026-08-23, against the census
[S1c.1.1](s1c.1.1_what_the_stdlib_promises.md) took
**Depends on:** [M1a](../../../docs/history/m1a_rust/README.md)'s
[P1a.10](../../../docs/history/m1a_rust/README.md#p1a10--one-implementation) — a new
surface form is cheap to add to one implementation and expensive to add to two
in step. Landing it before the oracle leaves means writing it twice.
**Was P1a.11; moved here 2026-08-21** at the user's direction. Nothing in the
phase changed — the dependency it was written against is simply a
cross-milestone one now, and the reason for the move is that this phase adds
*language surface*, which M1a's non-goals forbid the port itself.

## Goal

**The stdlib's rules get expectations of their own, checked by the engine
rather than by a diff.** A corpus of small `.ein` programs, one per rule or
rule family, each stating what it should derive — and an `ein test` that runs
them and reports pass/fail without anybody reading output.

## Why this is not more of the same

Every check the repo has today is *relative*: T0–T3 compare two engines, and
the goldens compare ein.rs to its own past. Both answer "did this change?" and
neither answers "is this right?". The stdlib is where that gap is widest:
`std.algebra`, `std.bijection`, `std.elim`, `std.closure`, `std.slots`,
`std.typing` and `std.macro` are the rules every puzzle imports, they are
exercised **only** as a side effect of whatever the zebra corpus happens to
need, and a rule that is never activated by any corpus entry is not tested at
all — it is merely not contradicted.

The session that produced this phase is the argument. `disjunctive-prune`'s
`(neq ?h_other ?h1)` guard was wrong for a year, in a rule five phases of byte
parity signed off, and it took an *independent* enumeration of a puzzle's
models to find it — because both engines agreed, and agreement was all
anything checked. That rule was puzzle-level. The stdlib has no such
independent check at all.

An expectation written next to a rule is that check, and it is the one kind
that gets **stronger** when the oracle leaves.

## Stages

| stage | title | est. | status |
|---|---|---|---|
| [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md) | What the stdlib promises, and what is exercised | 3 d | **shipped 2026-08-23** — [`stdlib_census.md`](stdlib_census.md) |
| [S1c.1.2](s1c.1.2_test_form.md) | How a program states what it expects | 3 d | **shipped 2026-08-23** — `:expect`, and `Program.queries` |
| [S1c.1.3](s1c.1.3_test_subcommand.md) | `ein test` | 2 d | **shipped 2026-08-24** — the fourth subcommand |
| [S1c.1.4](s1c.1.4_stdlib_corpus.md) | The stdlib corpus | ~~4~~ **6 d** | **shipped 2026-08-24** — 45 programs in `tests/stdlib/`, and the zero-firing set is **0** |
| [S1c.1.5](s1c.1.5_gate.md) | In the gate | 1 d | |

### What S1c.1.2 built

`:expect` on `query`, [Q-M1c.1](../open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)'s
option (c), with several `(query …)` blocks per file. Three things are worth
knowing without opening the stage:

- **The value is `(model <fact>*)`, not the bare list of facts the option was
  proposed with**, because a list cannot be a list's head in this grammar and
  widening `ListHead` would have changed every form. It is still one keyword,
  and it cost **zero** new grammar productions — the shape is checked by the
  loader.
- **The last-query-wins discard is gone**, and the corpus proved the trap had
  never been sprung: 0 of 128 entries had a second `(query …)`, and 0 goldens
  moved. What it costs is that an artefact flag naming one path — `--events`,
  `--trace`, `--json-summary`, `--dump-states` — is refused on a file that asks
  more than one question.
- **Five load-time refusals**, and they are the first diagnostics in the repo
  with no Python counterpart, which settled a question `defined_behaviour.md`
  had left for whichever error arrived first.

### What S1c.1.3 built

`ein test <PATH>…` — a file, a directory, or several of either — which runs
every `:expect` in the selection and reports a status code and one summary
line. Three things about it are decisions rather than implementation:

- **It exhausts, and has no flag not to.** An expectation is a claim about the
  *exhausted* answer, so a `-n` here would be a way to ask for `NOT CHECKED`.
  `11_expect_ambiguity.ein` needs `solve -e` and plain `test`.
- **A query with no `:expect` is never solved** — the stage's "only the work
  the expectations need runs", which under the shipped form means exactly this.
  `ein test examples/features/` checks 3 of 12 files and never enters
  `04_open.ein`, the entry the corpus marks as "a run nobody can finish is not
  coverage".
- **1 means a claim is false, so a load error takes 2.** `solve` gives a load
  error 1, because that is ein.py's; here 1 is taken, and a runner that cannot
  tell a broken file from a false claim is the failure T1c.1.3.5 is written
  against. A selection that checked *nothing* is 2 as well.

Two of the stage's five tasks were already decided by S1c.1.2 and S1c.1.1 —
there is one expectation kind, not four, and route is parked, so "decide about
redundant firings" has no `:fires` to decide about. What the stage did find is
a **bug in S1c.1.2's checker**: a `Contradiction` from a depth-capped search
was reported as a hard failure, where `k = 0` from a truncated lattice is "no
model within the cap". It is `NotChecked` now. Nothing reached it before,
because `solve -n 1` cannot produce a truncated `k = 0`.

### What S1c.1.4 built

**45 programs under [`tests/stdlib/`](../../../tests/README.md)**, one per rule
or tight family, and the acceptance number is measured rather than read: the
zero-firing set goes **38 → 0**
([`stdlib_census.md` §11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)).
Four things about them are decisions rather than files:

- **They are not in `examples/`.** That directory is things to read; these are
  things that exist to break, and 45 of them would have tripled a catalogue
  nobody would learn the language from. `tests/` is a third corpus root beside
  `examples/` and `stdlib/`, walked by the same completeness check, so a test
  file with no manifest entry still fails the gate.
- **`(open …)` is how a program says a negative was *not* invented.** Stored
  negatives are deliberately not closed by an expectation, so listing the
  exclusions that exist says nothing about the ones that do not. Ten programs
  carry a four-line `probe-undecided` rule at priority 500 whose body is
  `std.macro`'s `(open P)`, turning "in neither store" into a positive fact the
  expectation *can* close (an eleventh, `macro/02_open.ein`, is the probe's own
  test). It is the stage's one invention and it is what makes
  the `forall` family's open-world claim checkable at all.
- **A refutation rule gets two files.** "It fires and the answer is ⊥" is not
  the test that finds bugs; the other is a program where the rule is loaded,
  activated and *satisfied*, so a guard admitting too much turns an ordinary
  model into a contradiction. `algebra/08_checks_satisfied.ein` is that file for
  seven rules at once, and each of the seven has a `_violated` sibling.
- **The census's one open question is answered.** `slot-prune-bwd` had 606
  firings and no productive one; §5 could not say whether that was structural
  or a scheduling artefact, and named the puzzle that would settle it — a slot
  structure whose spatial relation is asymmetric *and* non-functional on
  positions. `slots/07_spatial_prune.ein` is that puzzle, both rules are
  productive in it, and exchanging the operands inside either `absent` turns it
  into a contradiction (verified against a mutated copy of the stdlib). It is
  structural.

It was also **measured for sensitivity**, which activation does not imply: 51
deliberate defects — one per rule family — injected into a copy of `stdlib/`
and run past `ein test tests/`. The first pass caught 44, and the seven
survivors were one finding seven times over: where two rules reach the same
verdict, a fact-shaped expectation cannot say which one did, so five fixtures
were narrowed to declare a single activator and two were rewritten. **50 of
51** now, with the survivor named rather than hidden.

It opened one question:
[Q-M1c.7](../open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates)
— `:expect` validates its relation names at load, so the seven fan-out rules,
whose entire output is derived activator facts, cannot be pinned directly and
are checked through what they activate.

### What the census settled

The phase opened on a claim — the stdlib is *"exercised only as a side effect
of whatever the zebra corpus happens to need"* — and
[S1c.1.1](s1c.1.1_what_the_stdlib_promises.md) turned it into numbers.
**38 of 73 rules never fire in any of 400 corpus runs**, 33 of them never even
loaded; 23 more are activated by exactly one entry, and `examples/zebra.ein`
is that entry for 20 of them. Two modules — `std.typing` and `std.closure` —
have never executed a single rule. The one module at 100 % coverage,
`std.slots`, is the *most* fragile thing in the table, because all eighteen of
its rules depend on one file.

Three findings change what the later stages do:

- **The `forall` guard is the untested claim that matters.** Ten rules
  quantify, every one of them documents itself as open-world-safe — "fires only
  when the stored negatives cover the whole extent" — and nothing checks it.
  Its failure mode is a wrong model, not a crash.
  [S1c.1.4](s1c.1.4_stdlib_corpus.md) writes that family first.
- **Route is needed by two pairs, and by two pairs only.** The risk below asked
  S1c.1.1's table to decide whether any rule needs `:fires`. `converse` ≡
  `imply2-reverse` and `imply2-fwd` ≡ `includes` are byte-identical bodies
  under two names, so a fact-shaped expectation cannot tell them apart — which
  is a reason to keep `:fires` parked, not to unpark it: two aliases are one
  test plus a naming convention, not a missing capability.
- **`std.bijection` and `std.elim` ship the same `typecheck-arg-{0,1}` body at
  two different priorities** (220 and 110), and nothing has ever had to
  reconcile them because a puzzle cannot import both.
  [`stdlib_census.md` §8](stdlib_census.md#8-four-declarations-are-two-rules).

## Acceptance for the phase

- **Every stdlib rule has at least one program that activates it and states
  what it should derive**, and the coverage claim is *measured* — a firing
  count per rule over the corpus, not a reading of the source.
- `ein test <file>` exits 0 / non-zero and prints what failed. No stdout
  diffing, no golden file, no second engine.
- A **negative** case per rule wherever one is meaningful: not only "this fires
  and derives X" but "this does *not* fire", which is where a guard bug lives.
  `disjunctive-prune`'s was exactly that shape.
- ~~The form is in `grammar.lark`~~ — the spec of record is
  [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) since M1a
  S1a.10.5, and **it needed no new production**: `KwPair ::= KEYWORD Value`
  already admits `:expect`'s shape, so what it carries is a §4 paragraph about
  what the *loader* checks. [M2](../../m2_nl_to_ir/README.md)'s GBNF lift reads
  that file, so the edit is still the deliberate cross-milestone one — it is
  just one paragraph rather than a grammar change.
- Adding a rule to the stdlib without a test fails the gate, the same way a
  file without a corpus entry does today.

## Risks

- **The expectation form is language surface.** M1a's non-goals say "no new
  syntax, no new keywords" — a rule about keeping the *port* honest against an
  oracle. It was narrowed on 2026-08-20 to let this phase add a form; the
  narrowing was withdrawn on the 21st when the phase left M1a instead, which
  is the cleaner arrangement and leaves the rule strict where it belongs. The
  cost of the form does not vanish with the argument: whatever it is, it is
  parsed, dumped, macro-expanded, rendered and round-tripped, and
  `grammar.lark` is the spec of record that M2's GBNF lift reads.
  [S1c.1.2](s1c.1.2_test_form.md) weighs three shapes and recommends the
  cheapest of them — **`:expect` on `query`, with several queries per file**
  (the user's, 2026-08-20) — which costs one keyword instead of a new head.
- **…and it costs a loader change with a trap.** Today the **last `query`
  silently wins**, pinned in both engines by a named test. A *test* file whose
  second check is silently discarded is worse than no test file, so
  `Program.query` becomes plural and every consumer of it says what it does
  with N. That is the widest-reaching part of the phase and it is in the first
  implementation stage, not the last.
- **Expressive creep.** The moment expectations can say "this fact holds", the
  next request is "this fact holds *because*", then a quantifier, then a
  little language. The stdlib's rules are the reason to stop early — and the
  recommended form resists it structurally, because an expectation shaped like
  a *model* has only one thing it can say. The vocabulary then grows one
  keyword at a time, each demanded by a rule.
- **A test that only restates the rule.** `functional-negative` asserts
  `(not (R a b'))`; a test that says "and then `(not (R a b'))` holds" has
  checked that the engine can read. The expectations that matter state the
  *consequence at a distance* — what the rule makes possible two firings
  later — and the stage says so explicitly because the cheap kind is easy to
  write by the dozen.
- **Route is not expressible in the recommended form.** An expectation made of
  facts cannot say *which rule* derived them, and for the stdlib that matters:
  `domain-elimination` and `range-elimination` reach the same positive from
  opposite directions. It is deliberately out of the first cut
  ([Q-M1c.2](../open_questions.md#q-m1c2--what-may-an-expectation-say)), and
  [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)'s table is what decides
  whether any rule actually needs it. **It decided: no, and it named the two
  pairs that raise the question** — `converse` ≡ `imply2-reverse` and
  `imply2-fwd` ≡ `includes`, alpha-identical bodies under two names, which one
  test covers because there is only one behaviour there to cover. The
  `domain-elimination` / `range-elimination` case that motivated the risk is
  *not* one of them: their bodies differ, they fire 1 008 and 203 times
  respectively, and an `:expect` naming the relation distinguishes their
  results. Route stays parked
  ([§8](stdlib_census.md#8-four-declarations-are-two-rules)).

## Cross-links

- [`stdlib_census.md`](stdlib_census.md) — [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)'s
  record: the promise inventory, the firing census, and the work list §6 hands
  to [S1c.1.4](s1c.1.4_stdlib_corpus.md)
- [`utils/stdlib_census.py`](../../../utils/stdlib_census.py) — the instrument,
  and [S1c.1.5](s1c.1.5_gate.md)'s coverage gate before it is a test
- [`stdlib/`](../../../stdlib/) — the seven modules under test
- [`docs/kernel/ir/03-ein-lang/`](../../../docs/kernel/ir/03-ein-lang/) — the
  surface language the new form joins
- [P1a.10](../../../docs/history/m1a_rust/README.md#p1a10--one-implementation) —
  [S1a.10.1](../../../docs/history/m1a_rust/README.md#s1a101--bank-what-only-the-oracle-proves)'s
  accepted-loss list is this phase's first input
