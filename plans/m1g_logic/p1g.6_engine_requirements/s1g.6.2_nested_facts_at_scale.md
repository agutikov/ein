# S1g.6.2 — Nested facts at scale: the index, a library's cache, and the nesting limit

**Phase:** [P1g.6](README.md) — candidates C-2, C-3, C-4
**Status:** planned 2026-11-11, not started. Provisional until P1g.1's stop.
**Depends on:** the witnesses the theory stages collect: [S1g.2.1](../p1g.2_propositional_logic/s1g.2.1_formulas_and_evaluation.md)'s
two evaluation rules, [S1g.2.3](../p1g.2_propositional_logic/s1g.2.3_normal_forms.md)'s rewriting,
[S1g.5.1](../p1g.5_peano_and_induction/s1g.5.1_unary_numbers_and_the_axioms.md)'s recursion. It runs once
they have numbers.
**Pulls in:** E-3, E-4 and E-12 of the calculator's
[`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md), each *open, shown, no
owner yet* there on 2026-11-11 (if P1f.6's close gave any an owner or a fix, this stage starts from that);
[F11](../../followups/f11_deductive_layer_perf.md), the deductive layer's performance followup.

**Added 2026-11-12, from the calculator's phase** (section J of its
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#j-findings-that-need-an-owner),
rows 57, 63, 69 and 70, each left empty and so taking its recommendation): this stage is now the **owner**
of E-3 and E-12 (C-2), of **E-15 and E-25** (a library has no cache of its own, and a `.einb` of the
calculator opens no faster than its text loads: C-3's evidence), and of **E-24 and E-26**, which are new to
it: `ein solve` writes every fact of a state as text, so a long computation's read-out costs far more than
the computation, and a stored fact costs about a kilobyte while a computation stores every step. E-24 and
E-26 are a fourth part of the stage, **the size of a state**, taken up in T1g.6.2.7 below. E-12's
calculator witness, the comparison table's column order (question 47), was explained to the user in round 2
and left as it is.

## Why the logic needs it

The calculator met the engine's cost on nested values and stepped around it with a **rule of the theory**:
no rule joins two facts on a number, every call carries a continuation. That discipline has a price the
calculator's README states: more rules, and a trace that is a sequence. A logic's natural rules are joins on
nested values almost by definition, *the value of the compound from the values of its parts*, *the line
that proves the premise*, *x + y from x + y′*, and the milestone would rather write them as the textbook
does. So this stage asks whether the engine can make the natural rule cheap, with **no answer moved**. It is
performance work and nothing else: R, N, H, every core, every `refuted by` and every byte of every surface
are the same before and after, on every corpus program.

## The three parts

**C-2, the index.** Two findings, both in `ein-infer/src/match_.rs`'s `probe_key`:

- **E-3:** a premise with a free slot whose bound arguments are nested facts is matched by a scan of its
  relation, since the index does not key on a nested value. A premise with every slot bound is one interned
  lookup whatever its arguments (S1f.6.3's note). Nested facts are interned, so a nested value has an id;
  the question is why the index does not use it, to be read off the code and M1a's design before anything is
  proposed.
- **E-12:** a premise is keyed on its *first* bound argument, whatever it selects, so a table whose first
  column has few values is searched among all rows that share it.
- **The measure** is the one the calculator built: matcher candidates per request under `--features
  counters`, on batches of several sizes, for each witness rule. *Fixed* means the natural rule's count per
  request is flat in the batch size.
- **What may not move:** the order in which matches are found decides the order facts arrive, which decides
  first justifications and so cores and `refuted by` (a core is an explanation, the user's ruling of
  2026-11-08, but the config principle and the goldens still hold every surface). So either the new index
  enumerates candidates in the old order, or the sweep shows exactly which explanations moved and the user
  accepts them, as S1f.5.15's removal of `:priority` did. The stage is planned for the first and prepared
  for the second.

**C-3, a library's cache.** `.einb` is a private cache of a whole program. A program that imports a large
library (the digit tables' 1 664 rows; a logic's tables and rules; a generated universe of formulas) parses
and loads it every run, and S1f.6.15 measures when that load exceeds the calculation. The candidates, to be
priced against that stage's numbers: a `.einb` per module, reused by every program that imports it; or the
stdlib pre-loaded in the binary, since it is already embedded as text. What makes it more than plumbing: an
import is *resolved and minimised* into the importer (the tree-shake E-11 met), so a module's cached form
has to compose with what each importer keeps; and `.einb` is never an interchange format, which stays true.
A cache is a lever: identical output with it and without, held by a test, like `ein solve x.einb` against
`x.ein` today.

**C-4, the nesting limit.** The parser nests to 1 000. A rule derives deeper; such a state's dump does not
reload (E-4). The calculator's side is S1f.6.9's choice for the user, and **this stage follows that ruling**
and does not re-open it. What it adds is the logic's side: which theories of M1g nest how deep (a unary
number n nests n; a context of assumptions; an assignment; a proof term), so the ruling is checked against
them, and the read-out or the dump says so when a state holds a fact a program could not write back, instead
of producing a file that fails to load.

## Tasks

1. **T1g.6.2.1 — the witnesses, counted:** each natural rule and its continuation twin from the theory
   stages, candidates per request at three batch sizes, in one table with the calculator's.
2. **T1g.6.2.2 — the index, read:** why `probe_key` skips nested values and takes the first bound probe; a
   written design with what it would change in match order; a prototype measured on the witnesses.
3. **T1g.6.2.3 — the index, built,** only if the prototype flattens the counts: with the sweep of every
   corpus program on every surface, the bench set (`cargo bench --bench engine`) before and after, and the
   id-order and jobs invariance tests.
4. **T1g.6.2.4 — the cache, priced:** load time against run time for the milestone's libraries; the two
   designs; a recommendation. Built only if a theory of the milestone is dominated by load.
5. **T1g.6.2.5 — the nesting limit, checked** against M1g's theories under S1f.6.9's ruling; a dump that
   cannot reload says so.
6. **T1g.6.2.6 — the findings closed:** E-3 and E-12 marked with what was done, in the calculator's
   `findings.md` if that file is still open, or here.
7. **T1g.6.2.7 — the size of a state (E-24, E-26), priced before anything is built:** what a read-out
   that listed a state's facts only on request, or wrote a number shared between facts once, would save on
   the calculator's long computations and on this milestone's proofs; and the question E-26's row says
   nobody has asked, whether a computation's intermediate steps may be dropped once its answer is stored,
   which changes what a state *is* and is the user's. Nothing is built without that ruling.

## Acceptance

- The natural evaluation rule of S1g.2.1 and the natural recursion of S1g.5.1 cost a flat number of
  candidates per request across batch sizes, or the page says why they cannot and the theories keep their
  continuations.
- Every corpus program's every surface is byte-identical before and after each change, or each moved
  explanation is listed and accepted by the user before any golden is blessed.
- No bench in the set regresses beyond its noise, measured through `utils/bench_env.sh`.
- The cache, if built, is invisible in every output and held so by a test.

## What it may find

- **That the index was left so on purpose** (M1a measured it; F9's ledger records nine search-layer
  optimisations measured and rejected, and the deductive layer has its own history). Then the reason is
  written into `findings.md`'s row and the theories keep their discipline.
- **That the fix is F11's whole subject** (beta memories, worst-case-optimal joins). Then this stage does the
  small part, a key on a nested value's id, and hands the rest to F11 with its witnesses.
