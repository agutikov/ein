# M1c — External validation

**Ran 2026-08-23 → 2026-08-24. Shipped.** One phase, five stages, and the
answer to a question nothing in the repository had ever asked: *is the standard
library right?*

> **This is history, not a plan.** M1c's plan tree —
> `plans/m1c_external_validation/`, a milestone README, a phase README and five
> stage documents — was deleted on 2026-08-24, once the milestone had shipped
> and the only thing a stage file could still do was describe work that was
> already done. What is kept is what is still *read*: this record, the census,
> and the questions. The stage files are in git history —
> `git log --diff-filter=D -- plans/m1c_external_validation` names the commit,
> and `git show <commit>^:plans/m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md`
> reads one.
>
> **Unlike [M1a](../m1a_rust/README.md), every instrument here still exists.**
> `utils/stdlib_census.py` re-takes the census, `ein test` re-runs the suite,
> and `cargo test` holds the coverage claim. Nothing below is a frozen constant.

## What is in this directory

| file | what it is |
|---|---|
| [`stdlib_census.md`](stdlib_census.md) | the milestone's evidence: what 73 rules promise, what the corpus activates, and the same census re-taken twice. §11 is the corpus-wide re-take, §12 the suite on its own. Re-takable with `utils/stdlib_census.py` — the **2026-08-23 "before" column is not**, because the tree it measured is gone |
| [`open_questions.md`](open_questions.md) | Q-M1c.1–7: two closed by building the thing, three moved to [M10](../../../plans/m10_external_benchmarks/README.md) with P1c.2, **two still open on purpose** |

---

## The thesis

**Every check this repo had was relative.** The conformance tiers compared two
engines; the goldens compare ein.rs to its own past; after
[P1a.10](../m1a_rust/README.md#p1a10--one-implementation) the second engine is
gone, so what was left compared ein.rs to yesterday's ein.rs. All of it answers
*did this change?* None of it answers *is this right?*

There are exactly two ways to answer the second question, and this milestone
owned the first:

1. **What the rules say they do.** An expectation written next to a rule, run
   by the engine — P1c.1. *This milestone.*
2. **What other systems answer.** The same problem stated for Z3, CVC5,
   SWI-Prolog, Soufflé, Clingo and Lean, run by one harness, compared on the
   *answer* first and the clock second —
   [M10](../../../plans/m10_external_benchmarks/README.md), a phase here until
   2026-08-23.

The precedent was specific, recent and expensive. `disjunctive-prune`'s
`(neq ?h_other ?h1)` guard was wrong for a year — through five phases of
byte-exact parity — and what found it was an **independent enumeration** of a
puzzle's models, written outside the engine on the day. Both engines agreed
with each other the whole time, and agreement was all anything checked. P1c.1
made that kind of check cheap for a *rule*; M10 makes it permanent for a
*puzzle*.

### Splitting them did not split the pipeline

M10's ground truth lands in P1c.1's form. When Clingo enumerates 32 models of
`zebra2-minus-15` and Z3's blocking-clause loop agrees, that answer is written
into the `.ein` file as an `:expect`, and from then on `ein test` re-checks it
on a machine with no external solver installed at all. **The external tools are
needed to establish the answer, not to keep it.**

---

## What shipped

The milestone's own summary is one number that moved and one that did not
exist: **38 of 73 stdlib rules never fired**, and now **0** — held by
`cargo test` rather than by a script somebody remembers to run.

| | measured | where |
|---|---|---|
| the gap, before | **38 of 73 rules never fire** in 128 entries × 400 runs, 33 never even loaded; 23 more activated by one entry, `examples/zebra.ein` being that entry for 20; two modules at zero | [census §1–5](stdlib_census.md) |
| the gap, after | **0 of 73**, over 180 entries and 557 runs — and **0 of 73** over `tests/stdlib/` alone, with no `examples/` entry contributing | [census §11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty), [§12](stdlib_census.md#12-the-suite-on-its-own--s1c15) |
| the form | `:expect (model …)` / `(or (model …) …)` / `(false)` — **one keyword, 0 new grammar productions, 0 goldens moved** | [S1c.1.2](#s1c12--how-a-program-states-what-it-expects) |
| the runner | `ein test <file\|dir>…`, the fourth subcommand; exhausts by default, never solves a query with no `:expect` | [S1c.1.3](#s1c13--ein-test) |
| the corpus | **45 programs** in `tests/stdlib/`, 2 628 lines of which 1 499 are header; the whole suite in **0.03 s** | [S1c.1.4](#s1c14--the-stdlib-corpus) |
| its sensitivity | **50 of 51** deliberate defects caught, one per rule family, injected into a copy of `stdlib/`; the survivor is named rather than hidden | [`tests/README.md`](../../../tests/README.md) |
| the gate | two assertions in `cargo test`, **0.04 s**; 225 of the corpus sweep's 889 cells and 0.72 s of its 5.08 s | [S1c.1.5](#s1c15--in-the-gate) |
| new grammar productions, over the whole milestone | **0**. New language keywords: **1** | [Q-M1c.1](open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects) |

**The estimate was 15 days of stages and it took two.** Recorded without a
lesson attached: the *shape* — measure the gap, design the form, build the
runner, write the corpus, then gate it — is what the work followed, and its
duration is not.

### Three things the milestone found that it was not looking for

- **The last-query-wins trap had never been sprung.** `from_ir.rs` silently
  discarded every `(query …)` but the last, pinned in both engines by a named
  test. S1c.1.2 made `Program.query` plural across 14 call sites — and **0 of
  128** corpus files had a second query to discard. A real trap, never hit.
- **A stopped search cannot confirm a verdict.** The checker S1c.1.2 shipped
  called `k = 0` from a depth-capped lattice a refutation. `Outcome::NotChecked`
  exists because S1c.1.3 made exhausting the default, which is the only regime
  that can reach it.
- **`slot-prune-bwd`'s sterility was structural, not scheduling.** The census
  found it firing 606 times and never productively, and could not say why. It
  needed a slot structure whose spatial relation is asymmetric *and*
  non-functional on positions — which no puzzle in the repository had.
  `tests/stdlib/slots/07_spatial_prune.ein` is that puzzle, and exchanging the
  operands inside either `absent` turns it into a contradiction.

---

## What was parked, and on what argument

| parked | the argument |
|---|---|
| **Route — `:fires R` / `:does-not-fire R`** | "the right fact by the wrong rule" has no home in a vocabulary of facts, and the census decided it is not needed: the pairs that raise the question (`converse` ≡ `imply2-reverse`, `imply2-fwd` ≡ `includes`) are **alpha-identical bodies under two names**, so one test covers both. `domain-elimination` / `range-elimination`, the case that motivated the residue, have different bodies and an `:expect` naming the relation tells their results apart ([census §8](stdlib_census.md#8-four-declarations-are-two-rules)). S1c.1.4 then priced the alternative by measurement: **separation-by-activation** costs five header paragraphs and no language surface, and it is what turned 44 caught mutants into 50 |
| **"Relation `r` is empty"** | unsayable, because the only way to name a relation is to list a fact in it. Reachable in exactly one place — rule 1 makes the goal's relations mandatory — and no fixture needs it yet ([Q-M1c.6](open_questions.md#q-m1c6--how-does-an-expectation-say-a-relation-is-empty)) |
| **An expectation naming a relation only saturation creates** | the loader's anti-typo check refuses it, which is right for a typo and wrong for a derived relation. It is the seven fan-out rules' whole output, so they are pinned at one remove — through the rules they activate — which cannot catch a *surplus* activator ([Q-M1c.7](open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates)) |
| **A test framework** | `ein test` runs what a program states about itself. No setup, teardown, fixture, tag, skip or parameterisation, and the corpus already knows how to enumerate files. If a rule needs a framework to be tested, the interesting finding is about the rule |
| **The whole external-solver half** | promoted to [M10](../../../plans/m10_external_benchmarks/README.md) on 2026-08-23 with P1c.2, taking five stages and three method questions (Q-M1c.3–5 → Q-M10.1–3). What stayed here is the half that runs with no external tool installed |

**The three parked questions are one question asked three ways** —
*what may appear on the left of a closure claim?* — and
[Q-M1c.7](open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates)
says that if a fourth arrives they should be answered together.

---

## P1c.1 — stdlib conformance

**Five stages, all shipped.** `std.algebra`, `std.bijection`, `std.elim`,
`std.closure`, `std.slots`, `std.typing` and `std.macro` are the rules every
puzzle imports; they were exercised **only** as a side effect of whatever the
zebra corpus happened to need, and a rule no corpus entry activates is not
tested — it is merely not contradicted.

| stage | est. | landed |
|---|---|---|
| [S1c.1.1](#s1c11--what-the-stdlib-promises-and-what-is-exercised) | 3 d | the census, and the number that justified the rest |
| [S1c.1.2](#s1c12--how-a-program-states-what-it-expects) | 3 d | `:expect`, and `Program.queries` |
| [S1c.1.3](#s1c13--ein-test) | 2 d | the fourth subcommand |
| [S1c.1.4](#s1c14--the-stdlib-corpus) | 6 d | 45 programs, and the zero set is 0 |
| [S1c.1.5](#s1c15--in-the-gate) | 1 d | the census as a test |

### S1c.1.1 — What the stdlib promises, and what is exercised

**Shipped 2026-08-23.** The record is [`stdlib_census.md`](stdlib_census.md);
the instrument is [`utils/stdlib_census.py`](../../../utils/stdlib_census.py),
the nineteenth script in `utils/` and the first check aimed at the *standard
library* rather than the engine.

| finding | number |
|---|---|
| stdlib rules declared | **73**, over six modules (`std.macro` declares none — it ships macros) |
| rules **no corpus run activates** | **38** — 52 %, of which **33** are never even loaded |
| rules activated by **exactly one entry** | **23**, and `examples/zebra.ein` is that entry for **20** |
| untested **or** held up by one file | **61 of 73 — 84 %** |
| coverage if `examples/zebra.ein` were dropped | **35 rules → 15** |
| modules at zero / full coverage | **two** (`std.typing`, `std.closure`) / **one** (`std.slots`, all eighteen by one file) |
| rules that fire and derive **nothing** | **3** — `functional`, `injective`, `slot-prune-bwd` |
| rules that read zero only because of the `normal` elision | **3** — the [S1a.7.0](../m1a_rust/README.md#s1a70--the-speculation-audit) trap, measured rather than assumed |
| example files declaring their **own** copy of a stdlib rule name | **25** — unfiltered, `symmetric` reads 112 271 productive firings over 22 entries against the true **1 084 over 7** |
| declarations that are another declaration renamed | **4 pairs**, one differing only in **priority** (220 vs 110) |
| rules in the zero set that are *unreachable* | **0** — what T1c.1.1.3 went looking for and did not find |

Two findings changed what the later stages did. **The `forall` guard was the
untested claim that mattered**: ten rules quantify, every one documents itself
as open-world-safe, nothing checked it, and its failure mode is a wrong model
rather than a crash. And **the expensive item was not in the zero set at all** —
`std.slots` was the module at 100 % rule coverage and the most fragile thing in
the table, because all eighteen of its rules depended on one file. That is what
moved S1c.1.4 from 4 days to 6, and it added a task the plan did not have.

### S1c.1.2 — How a program states what it expects

**Shipped 2026-08-23**, closing
[Q-M1c.1](open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)
to option (c) and
[Q-M1c.2](open_questions.md#q-m1c2--what-may-an-expectation-say).

```lisp
(query :goal   (pet-loc Zebra ?h)
       :expect (model (pet-loc Zebra House-5) (pet-loc Fox House-1)
                      (not (pet-loc Zebra House-1))))
```

**Naming a relation closes it** — the listed `pet-loc` facts are that
relation's complete extent, so a *surplus* fact fails, which is the case a
per-fact assertion cannot catch and the shape of the bug the milestone was
written around. `(or (model …) …)` compares model **sets** with `k` implied by
the count; `(false)` is `Contradiction`. `ein solve` exits 1 when the claim is
false, so a file carrying one is a test with no harness around it.

| finding | number |
|---|---|
| grammar productions added | **0** — the shape is loader-checked, not parsed |
| why `(model …)` and not the proposed bare list | `ListHead ::= SYMBOL \| VAR \| WILDCARD \| EQ` — **a list head does not parse**, and widening it would change the grammar of every form |
| call sites touched by `Program.query` → `queries` | **14** |
| corpus files that relied on the last-query-wins discard | **0 of 128** |
| goldens that moved | **0** — 188 golden lines added, all cells for new fixtures, none changed |
| load-time refusals added | **5**, each with a `broken/load/` fixture: unknown keyword, malformed shape, unknown relation, omits the goal, not ground |
| …and they are the **first ein.rs-only diagnostics** | which decided what [`defined_behaviour.md` §4](../../kernel/defined_behaviour.md) had left open: a message with no Python counterpart names **no exception class** |
| the cost nobody predicted | an artefact flag names **one** path, so `--events` / `--trace` / `--json-summary` / `--dump-states` are refused (exit 2) on a file that asks more than one question |
| tests added | **31** |

**Corrected 2026-08-24, the day after it shipped**, both found by the user
reading the form back. ⊥ is **`(false)`, not `none`** — `false` is already
ein's contradiction, one of the five `STRUCTURAL` names and what every
refutation rule in the stdlib asserts; shipping an invented word for it was the
mistake, and `(model)` remains legal and means *the empty model*, which is why
`()` was never the answer. And **a non-exhausted search cannot confirm a
verdict**: `Outcome::NotChecked` bites only where more searching could have
changed the answer, so finding *more* models than claimed stays a plain
failure.

Expectations are **ground**: a `?var` or `_` in one is a load error, because an
expectation is an answer and a pattern there would match whatever the engine
happened to derive — a test that cannot fail.

**Where the spec of record went.** The stage was written against
`grammar.lark`, which left with `ein.py` at M1a S1a.10.5. Its successor is
[`00_ebnf.md`](../../kernel/ir/03-ein-lang/00_ebnf.md), and it carries the form
as **§4, what the grammar deliberately does not enforce**, because
`KwPair ::= KEYWORD Value` already admits every shape `:expect` uses. The
cross-milestone edit [M2](../../../plans/m2_nl_to_ir/README.md)'s GBNF lift
reads is therefore one paragraph rather than a grammar change.

### S1c.1.3 — `ein test`

**Shipped 2026-08-24.** The fourth subcommand — `ein {render,saturate,solve,test}`
— which turns a directory of expectations into a status code, so that **nothing
reads output**.

```
$ ein test examples/features/
(no expect)  examples/features/01_not_and_absent.ein
…
ok           examples/features/10_expect.ein
12 files, 3 expectations: 3 held, 0 FAILED, 0 not checked, 0 errors  (0.00 s); 9 files state no expectations
```

Three things about it are decisions rather than implementation:

- **It exhausts, and has no flag not to.** An expectation is a claim about the
  *exhausted* answer, so a `-n` here would be a way to ask for `NOT CHECKED`.
- **A query with no `:expect` is never solved.** `ein test examples/features/`
  checks 3 of 12 files and never enters `04_open.ein`, the entry the corpus
  marks as "a run nobody can finish is not coverage".
- **1 means a claim is false, so a load error takes 2.** `solve` gives a load
  error 1, because that is ein.py's; here 1 is taken, and a runner that cannot
  tell a broken file from a false claim is the failure T1c.1.3.5 was written
  against. A selection that checked *nothing* is 2 as well — which is M1c's "a
  missing tool is reported, never skipped past" in the shape a test runner can
  fail in.

**Two of the stage's five tasks had already been decided by the two stages
before it**, and the stage document said so rather than pretending otherwise:
there is one expectation kind rather than four, and route is parked, so "decide
about redundant firings" had no `:fires` to decide about. What the stage added
beyond its plan: a failure names the **derivation** of a surplus fact, one
level of premises, and a `k` mismatch projects every model through the query's
own `:goal` — both in `ein-infer::expect`, so `ein solve`'s `:expect FAILED`
block grew them too. 29 tests; the help surface went 40 options across 8
parsers → **48 across 9**.

### S1c.1.4 — The stdlib corpus

**Shipped 2026-08-24.** **45 programs under
[`tests/stdlib/`](../../../tests/README.md)**, one per rule or tight family,
and the acceptance number is measured rather than read: the zero-firing set
goes **38 → 0** ([census §11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)).

| finding | number |
|---|---|
| programs | **45**, over seven directories |
| where they live | **`tests/stdlib/`**, not `examples/` — a third corpus root beside `examples/` and `stdlib/`, walked by the same completeness check |
| rules that fire but derive nothing | **3 → 0**; `functional`, `injective` and `slot-prune-bwd` are productive for the first time |
| rules whose sole activator is `examples/zebra.ein` | **20 → 0** |
| lines | 2 628, of which **1 499 are header** — the ratio is the stage's, not an accident |
| the whole suite under `ein test` | **0.03 s** |
| corpus cells added | **225**; 0 existing cells moved |
| mutants caught | **50 of 51** |

Four things about them are decisions rather than files:

- **They are not in `examples/`.** That directory is things to read; these are
  things that exist to break, and 45 of them would have tripled a catalogue
  nobody would learn the language from.
- **`(open …)` is how a program says a negative was *not* invented.** Stored
  negatives are deliberately not closed by an expectation, so listing the
  exclusions that exist says nothing about the ones that do not. Ten programs
  carry a four-line `probe-undecided` rule at priority 500 whose body is
  `std.macro`'s `(open P)`, turning "in neither store" into a positive fact the
  expectation *can* close. It is the stage's one invention and it is what makes
  the `forall` family's open-world claim checkable at all.
- **A refutation rule gets two files.** "It fires and the answer is ⊥" is not
  the test that finds bugs; the other is a program where the rule is loaded,
  activated and *satisfied*, so a guard admitting too much turns an ordinary
  model into a contradiction. `algebra/08_checks_satisfied.ein` is that file for
  seven rules at once, and each of the seven has a `_violated` sibling.
- **Where two rules reach one verdict, separate them by activation.** This is
  the `route` residue arriving as a measurement rather than an argument: the
  mutation sweep's first pass caught 44 of 51, and the seven survivors were one
  finding wearing seven hats — an expectation made of facts cannot say which
  rule produced them. Five fixtures were narrowed to declare a single
  activator, two were rewritten, one was added, and the second pass catches 50.

**What the census predicted and got wrong.** §6 called the seven fan-out rules
the cheapest bucket — *"a claim about facts, and so exactly what an `:expect`
can say with nothing else in the file"*. It is not: `:expect` validates its
relation names at **load**, and a fan-out asserts into relations that exist
only after saturation. That is
[Q-M1c.7](open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates).

### S1c.1.5 — In the gate

**Shipped 2026-08-24.** The coverage claim moves from a 37 s script to
`cargo test` — `ein-infer/tests/stdlib_coverage.rs`, two assertions, **0.04 s**,
no binary and no subprocess:

```
73 rules, all activated; 45 programs, 796 firings, 0.04 s
```

It loads `stdlib/*.ein` with the engine's own parser for its rule heads, solves
every `tests/stdlib/` program the way `ein test` does with an in-memory
`Events` sink, and reads `fire` off the stream — the same observable
`utils/stdlib_census.py` reads, with the same attribution rule (a local
declaration shadows a stdlib name outright, a module the file never imported
cannot have fired, then arity).

**Scope was the whole decision.** `--check` sweeps all 180 corpus entries and
would go on exiting 0 for a rule added tomorrow that happened to fire inside
`examples/zebra.ein` — with *no test written*, which is the state 20 rules were
in before S1c.1.4. The test sweeps `tests/stdlib/` and nothing else, so "adding
a rule to the stdlib without a test fails the gate" is true rather than
aspirational.

**And that scope found the one rule the suite never ran.** `transitive`'s
fixture was a two-cycle where the `(neq ?a ?c)` guard refuses every match —
the right test of the guard, silent about the assertion, which was resting on
six puzzles whose hierarchies are acyclic. `algebra/21_transitive.ein` has a
three-chain now, and the suite's own number is **73 of 73**
([census §12](stdlib_census.md#12-the-suite-on-its-own--s1c15)).

A second assertion fails on any program under `tests/` that states no
`:expect` — the failure mode a refactor produces, and one `ein test` reports
rather than fails on. **All three failure modes were exercised against the tree
and reverted**: revert the fixture and it names `std.algebra/transitive`;
append a rule to `stdlib/algebra.ein` and it names it, 1 of 74; delete an
`:expect` and it names the file. A coverage gate nobody has seen fail is a
coverage gate nobody has tested.

What is deliberately **not** gated: the dual ("every program activates a rule"
— four fixtures exist to show a rule loaded, activated and *silent*),
sensitivity (a mutation score needs 51 copies of the stdlib to re-take, and a
stale one is worse than none), and productivity (the census reports the
productive/redundant split and the gate does not read it).

The stage also put `cargo fmt --all --check` in `./run_tests.sh`, because the
first run of it found **three unformatted files, all three from this
milestone's own `:expect` work**, and nothing anywhere would have said so.

---

## Acceptance, met

- **No stdlib rule rests only on self-agreement.** 73 of 73 have a program
  that activates them and states what they derive, machine-checked rather than
  prose — and the check is scoped to the programs written to activate them.
- The expectations are **checked in as `:expect`**, so an answer established
  anywhere survives the absence of whatever produced it. `ein test` needs
  nothing installed to re-check one.
- **A missing tool is reported, never skipped past.**
  [S1a.10.1](../m1a_rust/README.md#s1a101--bank-what-only-the-oracle-proves)
  found 42 tests that started a Python process and skipped invisibly when one
  would not start. A selection that checked nothing exits 2 and says so, and
  the gate's second assertion is the same rule applied to the directory.
- The milestone's other half — every benchmark problem's answer confirmed by a
  system that is not Ein — is
  [M10](../../../plans/m10_external_benchmarks/README.md)'s acceptance.

## Cross-links

- [`tests/README.md`](../../../tests/README.md) — the suite this milestone
  wrote, its four idioms, and the mutation survivor
- [`stdlib/README.md`](../../../stdlib/README.md) — the seven modules under
  test, and the growth rule the gate now enforces
- [`docs/kernel/ir/03-ein-lang/01_grammar.md` § Query](../../kernel/ir/03-ein-lang/01_grammar.md#query)
  — `:expect` as it is specified today
- [`utils/stdlib_census.py`](../../../utils/stdlib_census.py) — the instrument,
  still runnable; `--check` is the corpus-wide measurement, not the gate
- [M10](../../../plans/m10_external_benchmarks/README.md) — the half that needs
  a solver installed, and the campaign that fills this milestone's form
- [M1d](../../../plans/m1d_satisfiability/README.md) — the sibling created the
  same day; `zebra2-minus-15`'s 32 models are M10's cross-check and M1d's
  subject
- [M5](../../../plans/m5_presentation/README.md) Track A — the consumer
