# P2.3 — The benchmark (Stage C)

**Depends on:** the kernel as it stands ([Q-M2.1](../open_questions/kernel/Q-M2.1-when-is-the-kernel-frozen.md));
[S2.1.1](../../../docs/einaf/README.md), which creates the package `einaf/` and its
gate step, where the generators and the benchmark's check live; [S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)
for the formatters, which one check of an instance needs (that its canonical program prints the gold short
answer). Building the benchmark needs no model. Running it does: the Level B gate needs all of
[P2.2](../p2.2_formalizer/README.md) and the record of [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md).
**Blocks:** the **Level B gate**, which this phase closes; [P2.5](../p2.5_harness/README.md), whose every table is
over this benchmark's splits; [P2.7](../p2.7_failure_scaling_generalization/README.md), whose difficulty curves
and paraphrase sets are this phase's generators with other arguments.
**Research plan:** [`EinAf.md` § Stage C](../EinAf.md#stage-c--construct-a-heterogeneous-benchmark), C1 to C5.
**Re-planned** to [the frame](../README.md#the-plan-as-re-planned), with
[`analysis/audit_p2.3.md`](../analysis/audit_p2.3.md) as the worklist: a section of the audit is cited below as
*audit § B.3*.

---

## What the re-plan changed

| the old plan said | now | by |
|---|---|---|
| ground truth is *certified by the kernel, exhaustively*, by `ein solve -e` | a canonical program states its answer as `:expect` claims and `ein test` checks them; the answer itself is known before the kernel is asked, by construction or from its author | audit § A.1, § B.3; [§ An instance](#an-instance) |
| the gold of an instance is a verdict (`unique`, `ambiguous`, `unsat`) and bindings, a count `k`, or a core | a gold **class**, a gold **short answer** and a canonical program; no count `k`, and no core as an answer | the frame's defaults 7 and 8; audit § C.4 |
| the verdict is read off the kernel's word | the kernel's word is not the problem's class, so the class is stated by the shape of the program's claims | audit § B.3; [E-14](../evaluation/answer/E-14-the-kernels-word-is-not-the-problems-class.md) |
| `benchmark/` at the repository's root, generators in Python | `einaf/benchmark/`, outside every corpus root; generators in Rust, in the package | [Q-M2.8](../open_questions/einaf/Q-M2.8-where-does-einaf-live.md), [Q-M2.7](../open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md); the frame's defaults 2 and 3 |
| eight families, seeded by five old puzzles | eight families and the word problems of `examples/word_problems/`; of the five old puzzles only Zebra was ever written | the frame's default 8; audit § B.2, § B.4 |
| a closed-world `(absent …)` in three canonical theories | no canonical program is flagged; each such family has an encoding that is not, or it has no canonical program | [Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md); [§ Risks](#risks) |
| texts in English, Russian realisations later | English, names ASCII | the frame's default 9 |
| estimates per stage | none | the frame's default 13 |

## Why Zebra alone is not a benchmark

The old plan's seed set was five attribute-matching puzzles, of which Zebra is the one that exists
(audit § B.2). One family is not a benchmark for the research question, for three reasons Stage C makes precise:

1. **It tests whether Ein solves Zebra-like puzzles**, not whether a formalizer can construct a theory. In one
   family the theory is always the same one, so reinterpretation, theory selection and theory synthesis cannot be
   told apart.
2. **Every instance has one answer.** A loop that drives every theory to one answer scores perfectly on it and is
   inventing constraints on half of the real inputs. A benchmark whose instances are never *ambiguous* or
   *unsatisfiable* as their **right** answer cannot measure whether a system tells *I don't know*, *several
   answers*, *the assumptions are inconsistent* and *the formalization failed* apart.
3. **A few points are not a curve.** Scaling ([P2.7](../p2.7_failure_scaling_generalization/README.md)) needs one
   family at several sizes, with the knobs of C4 set and not found.

## An instance

An instance is four things, and an external instance the first three:

1. **the text**, as a model sees it: English, names ASCII;
2. **its gold class**, the research plan's three ([C3](../EinAf.md#c3-generate-deliberately-ambiguous-problems)):
   *one answer*, *ambiguous*, *unsatisfiable*. It is a property of the problem, not of a run. The finer split of
   [F26](../../followups/f26_consistency_and_completeness.md), item 6 of its last section, stands beside it as
   metadata;
3. **its gold short answer**, a sentence in the problem's words (the user's ruling, [Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md):
   *"a sentence, like zebra's"*), with the values it binds; what it states is what every arm B0 to B6 is scored
   against
   ([the arms](../README.md#the-arms));
4. **its canonical Ein program**: ontology, theory, instance and query, with the answer stated as `:expect` claims
   that the kernel checks, and the gold short answer printed through its answer query's `:goal-text`
   ([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)).

### How a class is established

The kernel's word does not give it. `ein solve --single` says `Solution` on
[`gap.ein`](../../../examples/word_problems/gap.ein), whose right answer is *not enough information*, and
`Ambiguity` on [`boxes12.ein`](../../../examples/word_problems/boxes12.ein), which has one answer (audit § B.3;
both run again at the re-plan). So a class is established in three steps, none of which is the verdict word:

1. **Something other than the kernel says what the answer is.** A generator draws the situation the clues are
   true of and counts the answers itself ([S2.3.2](s2.3.2_generators.md)); a hand-written instance has its
   author; an external one its source.
2. **The canonical program states the class in the shape of its claims**, by the table below. One query of the
   program is its **answer query**, named in the metadata: the one whose `:goal-text` prints the answer.
3. **The kernel holds the claims**: `ein test` over the program, every claim held, none `NOT CHECKED`.

| gold class | finer | the answer query's `:expect` | the `(solve … :expect …)` | `--single` says | a corpus program of this shape |
|---|---|---|---|---|---|
| one answer | complete | one class with a row, `(model F …)` | | `Solution` | [`bob1.ein`](../../../examples/word_problems/bob1.ein) |
| one answer | complete, on a marked assumption | the same, beside a query that claims what the answer rests on | | `Solution` | [`gap_assume.ein`](../../../examples/word_problems/gap_assume.ein) |
| one answer | a reading, and the readings agree | one class, over two states or more | the states, where it helps | `Ambiguity` | [`boxes12.ein`](../../../examples/word_problems/boxes12.ein) |
| ambiguous | a reading | two classes or more, `(or (model …) (model …))` | | `Ambiguity` | [`boxes13.ein`](../../../examples/word_problems/boxes13.ein) |
| ambiguous | something the text does not state | two classes or more, asked `:over maximal` where the program states restrictions only | | `Ambiguity` | [`11_expect_ambiguity.ein`](../../../examples/features/11_expect_ambiguity.ein) |
| ambiguous | something the text does not state, the gap named | the empty class, `(model)`, beside a query that claims the fact that names the gap | | `Solution` | [`gap.ein`](../../../examples/word_problems/gap.ein) |
| unsatisfiable | contradictory: every node is dead | `(false)`, or none | `(false)` | `Contradiction` | [`contra.ein`](../../../examples/word_problems/contra.ein) |
| unsatisfiable | no commitment pays what is owed | `(false)` | `(region (owing …))` | `Open` | [`unsatisfiable.ein`](../../../examples/binary_logic/unsatisfiable.ein) |

The forms are [`12-query.md` § Claims](../../../docs/ein/user/03-syntax-and-semantics/12-query.md#claims)'s:
a query's claim lists the goal's projection classes over the part of R its `:over` names, `(model)` is the class
where the goal has no row, and a query's `(false)` holds over either kind of empty R
([Q-M1f.219](../../../docs/decisions/answer/Q-M1f.219-a-querys-false-holds-over-either-kind-of-empty-r.md)). There
is no `none`, and a region's `(false)` is the dead kind alone.

**The last row is a class the kernel's reply cannot tell from a formalization that is not finished**: an empty R
whose alive nodes owe is `Open`, which [the frame's outcomes](../README.md#the-outcomes) read as *incomplete*.
Which kind of empty R a contradiction gives depends on what the theory states. Probed at the re-plan, on an order
written as a bijection onto named ranks: a cyclic *older than* answers `Open` under `functional`, `injective` and
`total-owed` alone, and still `Open` with `(transitive older)` and `(irreflexive older)` added, since
`std.algebra`'s `transitive` never derives the self-loop; it answers `Contradiction` once the program also writes
`(asymmetric older)`. By default an *unsatisfiable* instance's canonical theory states the property its
contradiction violates, so that root is dead; one that answers `Open` is admitted with its kind in the metadata
and counted in a column of its own ([§ Defaults](#defaults-taken-at-the-re-plan), 5).

### What admits an instance

`einaf bench check` ([S2.3.1](s2.3.1_families_and_seed_set.md), T2.3.1.5) admits an instance when:

- its canonical program loads, with `:why` required
  ([Q-M1f.57](../../../docs/decisions/corpus/Q-M1f.57-every-fixture-the-loader-reads-carries-a-why.md));
- **it is not flagged**: the static check finds no site
  ([Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md));
- **it states its lattice**: its hypotheses are a rule's, the program's or an activated obligation's, or it
  writes `(solve :no-search)`. A program that states facts and rules and no lattice is answered by the blind
  enumerator (probed: three people, one stated order and one rule that reads the order under `absent` are
  monotone with `(solve :no-search)`; with no `(solve …)` form the enumerator supposes eleven facts, `(is-a Ann
  Bob)` among them, the program is flagged and its claim fails);
- **`ein test` holds every claim** at the default `--region-budget`
  ([Q-M1e2.35](../../../docs/decisions/cli/Q-M1e2.35-the-default-region-budget-is-2-20-facts.md)), none `FAILED`
  and none `NOT CHECKED`;
- the gold class is the one its claims' shapes state, by the table above;
- **it prints the gold short answer**: what its answer query's `:goal-text` reads, with EinAf's formatters over
  the values, is correct by the comparison every arm is scored with
  ([S2.5.3](../p2.5_harness/s2.5.3_metrics.md)); or its metadata says that the answer is not one a program
  prints, and is read off the kernel's values ([§ Risks](#risks), the second);
- every `:source` text of its facts is a sentence of the instance's text.

The kernel's half of that is a command anyone with the `ein` binary can run, with no EinAf:
`ein test einaf/benchmark/ --json-report FILE` ([`05-cli.md` § 2.3](../../../docs/ein/user/04-reference/05-cli.md#23-ein-test);
the report is `ein-test-report/6`, one row a claim, with the question that decided it).

## The shape

**Eight families and the word problems** ([C1](../EinAf.md#c1-start-with-several-reasoning-families); the frame's
default 8). The corpus column is audit § B.1's; the last column is where a canonical program has to be written
with care, and [§ Risks](#risks) says what was probed.

| family | an example | what it stresses that Zebra does not | theory today | in the corpus today | external source | where a flag threatens |
|---|---|---|---|---|---|---|
| logical deduction | *A is older than B, B than C; who is youngest?* | an order from partial statements, and an extremum of it | `std.algebra`: `transitive`, `asymmetric` | single-rule demonstrations, no claim, no extremum asked | BBH `logical_deduction_{three,five,seven}_objects` | the minimum written as an `absent` |
| ordering | *Alice arrived before Bob, Carol after Bob* | two relations in one theory, *immediately before* inside *before* | `std.algebra`: `transitive`, `(includes immediately-before before)` | the Zebra row; one three-meal chain | BBH `logical_deduction`, read by position | as above, for *first* and *last* |
| spatial | *A is left of B, C is above A* | two orders at once; `converse` | `std.algebra`: `converse`; adjacency is facts, with no arithmetic lattice ([Q17](../../../docs/decisions/language/Q17-spatial-structure-is-facts-and-rules-with-no.md)) | one order only | none | |
| object tracking | *the key starts in box A; A and B are swapped; then B and C* | state: a sequence of transitions | **none in the library** | nothing; nearest are the word problems' transfers over named moments | BBH `tracking_shuffled_objects_{three,five,seven}_objects` | *unchanged unless swapped*, a frame rule, written as an `absent` |
| attribute matching | Zebra | bijections between several attribute sets and one spine | `std.bijection`, `std.slots` | the one full-size anchor: both Zebra encodings and their variants, none stating a claim | the Zebra texts of `nl2smt/puzzles/` | |
| set and category | *every A is B; no B is C; x is A; can x be C?* | subsumption and disjointness; a *can* question, which is about what is satisfiable | subsumption by `std.algebra`'s `transitive` and `includes`; **no library rule for disjointness** | single-rule demonstrations; no *can* question | ProofWriter, FOLIO | *can* and a closed-world *false* written as an `absent` |
| graph | reachable, adjacent, connected, ancestor, dependency | closure over an edge list; cycles | `std.algebra`: `transitive`, `symmetric` | single-rule demonstrations; no edge list with a question | CLUTRR | *not reachable* as an `absent` |
| temporal | before, after, overlap, interval containment | interval relations and their composition | **none in the library** | orders over events only; no interval | BBH `temporal_sequences` | *could X have been there at that time* |
| **word problems** | *Bob has 5 apples. He gave 2 to Alice. How many does Bob have?* | arithmetic as a theory; a gap named; a marked assumption; a word with two readings | four modules of [`examples/word_problems/`](../../../examples/word_problems/README.md) over `std.math.decimal-arith`; examples, not a library module ([Q-M1f.334](../../../docs/decisions/corpus/Q-M1f.334-the-word-theory-is-examples-not-a-module.md)) | sixteen problems, each sentence beside its fact as `:source`, every claim held, none flagged (audit § B.4) | none adapted | a generated variant is checked like any other: `gaps.ein` reads `absent` |

Two families have no theory in the library to select, object tracking and temporal; set and category has none
for disjointness; and the word problems' theory is the corpus's examples. That is the condition under which a
formalizer's *synthesis* is exercised at all, and a family where it fails is a result about synthesis. The
library does not grow for them inside M2: it is Ein's, and the kernel is changed here only where a ruling says
so (the frame's default 11).

**Both sources** ([C2](../EinAf.md#c2-use-both-external-and-controlled-synthetic-tasks)). Synthetic instances
give any size, an answer known by construction, the canonical theory and the knobs; external instances give
texts someone wrote for another reason and published numbers to stand beside. Neither alone.

**Three classes a family** ([C3](../EinAf.md#c3-generate-deliberately-ambiguous-problems)), by construction: a
clue deleted from an instance with one answer, a clue added that is false of its situation. The word problems
bring the kinds a deleted clue does not: a gap that is named, an answer on a marked assumption, a word with two
readings.

**Knobs** ([C4](../EinAf.md#c4-parameterize-difficulty)): entities, relations, chain depth, branching, constraint
density, irrelevant statements, paraphrase, implicit against explicit relations, the count of answers, the
distance between the statements one deduction needs. Each is a generator's argument and a field of the instance's
metadata, so a table of [P2.7](../p2.7_failure_scaling_generalization/README.md) is a slice of this benchmark.

**Frozen splits** ([C5](../EinAf.md#c5-freeze-benchmark-splits)): `dev`, `val`, `test`, by instance id and
content digest, with a version; `test` drives no prompt.

## Where it lives

`einaf/benchmark/` ([where EinAf's things are](../README.md#where-einafs-things-are)): the instances, the frozen
splits, the manifest, the record of each external source. The generators, the check and the adapters are modules
of the package, under `einaf/src/`, and subcommands of `einaf bench`.

- **It is no corpus root.** `examples/`, `stdlib/` and `tests/` are Ein's; a program under `einaf/benchmark/`
  has no `corpus/corpus.toml` entry and joins no sweep of Ein's gate
  ([Q-M2.8](../open_questions/einaf/Q-M2.8-where-does-einaf-live.md), default 4 of the ruling). Nothing of Ein reads,
  links or names the benchmark.
- **EinAf's gate step** holds what needs no model: the seed set whole, a stated sample of the generated
  instances, the generators' determinism, the manifest's and the split's invariants. The whole benchmark is
  checked when a split is frozen ([S2.3.4](s2.3.4_splits.md)).
- **No external item is in the tree**: a source is fetched to a directory git ignores, and the tree holds its
  record and the ids and digests of the items a split names ([S2.3.3](s2.3.3_external_benchmarks.md)).
- **One directory a family, flat.** An import is resolved from the program file's directory and `..` is refused
  ([Q-M1f.235](../../../docs/decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md),
  [Q-M1f.252](../../../docs/decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md)), so
  a family that keeps a theory of its own in a module, as the word problems do, keeps its instances beside it.

## What P2.2 hands this phase

*Written 2026-11-27, at [P2.2's close](../p2.2_formalizer/README.md#the-close-2026-11-27).* The files of this phase
were planned before P2.2 ran. What that
phase found that bears on them is in each stage's file, under *What P2.2 hands this stage*; this is the map. None of
it is a ruling: where an item rests on a question, the question's default stands.

| stage | what it reads |
|---|---|
| [S2.3.1](s2.3.1_families_and_seed_set.md) | the contract's answer queries ([Q-M2.35](../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md)); what a `:source` quotes and what a membership is ([Q-M2.25](../open_questions/answer/Q-M2.25-is-a-clause-of-a-sentence-a-source-or-only-the-whole.md), [Q-M2.22](../open_questions/answer/Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md), [Q-M2.26](../open_questions/answer/Q-M2.26-does-zebra3-1-quote-in-each-membership-the-sentence-that-names.md)); the target shape ([Q-M2.38](../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md)); `std.slots`' count on an under-determined puzzle ([E-105](../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)) and a relation declared twice under an import ([E-104](../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)); the answer's comparison ([S2.5.3](../p2.5_harness/s2.5.3_metrics.md)); the two texts [S2.2c.2](../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md) writes |
| [S2.3.2](s2.3.2_generators.md) | the attribute generator's count against the kernel's ([E-105](../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)); a change over time ([E-124](../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md), [Q-M2.33](../open_questions/answer/Q-M2.33-is-a-class-with-several-rows-an-answer-of-several-values-or-the-undetermined-text.md)); the sizes [P2.7](../p2.7_failure_scaling_generalization/README.md)'s scaling asks for |
| [S2.3.4](s2.3.4_splits.md) | what a prompt quotes and the families the examples are of ([L-M2.28](../findings/model/L-M2.28-the-prompts-first-example-is-the-zebra-puzzle-s-theory-in-miniature.md)); the texts development has seen; the formalizer the Level B run opens `test` with ([P2.2d](../p2.2d_formalizer_corrected/README.md)); the three classes told apart by accident ([L-M2.22](../findings/model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md)); the record ([L-M2.27](../findings/einaf/L-M2.27-a-record-s-kernel-commit-is-null.md)) |

**Before this phase**, by [the order the routing proposed](../README.md#the-order-before-p23):
[S2.2e.2](../p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md), so that no claim of an attribute
instance is written against a library rule known wrong; and [P2.2d](../p2.2d_formalizer_corrected/README.md), whose
contract an instance is shaped by.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.3.1](s2.3.1_families_and_seed_set.md) | The families and the seed set | the instance format; a hand-written anchor in each of the eight families and the sixteen word problems taken in, each with its text, class, short answer and a canonical program whose claims the kernel holds; Zebra's three; `einaf bench check`; what writing them found of Ein |
| [S2.3.2](s2.3.2_generators.md) | The generators: an answer known by construction, three classes, the knobs | a generator a family, in Rust: it draws a situation, counts the answers itself, writes the program with its claims and the text; every instance admitted by the check; the knobs in the metadata; where each family stops being decided at the default budget |
| [S2.3.3](s2.3.3_external_benchmarks.md) | External benchmarks: adapters | BBH's three task groups, then CLUTRR, ProofWriter and FOLIO: a fetch that records version, digest and licence, an adapter that writes instances with no canonical program, the answer contract a source |
| [S2.3.4](s2.3.4_splits.md) | Frozen splits, the manifest, the Level B run | `dev`, `val`, `test` by id and content digest, versioned; the lock on `test`; the manifest; the one-shot run on `test`; the evaluation of Ein written up for the gate |

## Acceptance

- Eight families and the word problems, each with a hand-written anchor and a generator, and an adapter where
  the table names a source. Every instance with a canonical program carries its text, gold class, gold short
  answer, the knob values that made it, and is admitted by [§ What admits an instance](#what-admits-an-instance).
- In every family there are instances of all three classes, or the family's page says which class it has none of
  and why.
- **No canonical program is flagged.** A family whose instances cannot be written unflagged has no canonical
  program for them, and says so.
- `ein test einaf/benchmark/` exits 0, and EinAf's gate step runs the check of the seed set and of the sample.
- The splits are frozen with a version; the digest of `test` is in the manifest and in every record that used it.
- `einaf/benchmark/` reads without the system: the text, the class, the answer, the program
  ([Stage Q](../EinAf.md#stage-q--public-research-artifact)'s second layer, early).
- **The Level B gate** ([the milestone's acceptance](../README.md#acceptance), items 1 to 5): the one-shot run on
  `test` over three families or more, reported per family; the three classes distinguished; every run a record;
  and **[the evaluation of Ein](../evaluation/README.md) written up as P2.1 to P2.3 found it and shown to the
  user**. The gate is a stop: the user says whether EinAf goes on or an iteration of Ein comes first
  ([Q-M2.15](../open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)).

## Risks

- **A family's natural encoding is flagged.** A minimum, a *can* question and a closed world are each most
  easily written with `absent`, and the static check flags a rule whose `absent` reads a relation a commitment
  can change ([E-31](../evaluation/answer/E-31-of-a-flagged-program-nothing-is-claimed.md); audit § C.3). Probed at the re-plan: *the youngest is who is older
  than nobody*, as an `absent` over `older`, is monotone where nothing of `older` is hypothesised and flagged
  beside candidates of `older`, which the family's *ambiguous* sibling needs. Two encodings that are not flagged
  were probed too: an order as a bijection onto named ranks (`functional`, `injective`, `total-owed`), where the
  youngest is who holds the lowest rank, in all three classes; and a *can* question as a hypothesis the theory
  refutes. [S2.3.1](s2.3.1_families_and_seed_set.md) finds the encoding of each family by experiment; a family
  with none keeps its texts and gold answers and has no canonical program, and that is an entry of the
  evaluation.
- **A negative answer has no sentence.** A `:goal-text` is filled once a row, and the class where the goal has no
  row prints *the goal has no row*. *No*, *unknown* and *not enough information* are printed only where the
  theory derives a fact that says so, as [`gaps.ein`](../../../examples/word_problems/gaps.ein) does, and an
  empty R has no state to answer from. So where the right answer is negative the canonical program derives a
  fact that says it and prints that, or the answer is read by EinAf off the empty class and the metadata says
  so; and the gold short answer of an *ambiguous* or *unsatisfiable* instance is its class, whose words are
  EinAf's ([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)).
- **The realiser writes a dialect.** A generator's English is one template family's English.
  [K1](../EinAf.md#k1-linguistic-paraphrase)'s paraphrase sets are the check, and
  [S2.3.2](s2.3.2_generators.md) writes three surface templates or more a statement type from the start.
- **The generator and the canonical theory share a mistake.** The generator's count of answers and the kernel's
  check are two routes to one answer; they are independent only if the generator's enumeration does not go
  through Ein. A disagreement is a finding: of the generator, of the theory, or of the kernel, and the last is a
  row of [the wrong-answer register](../../wrong_answers/README.md).
- **An ambiguous instance of Zebra's size has an answer that is not short**: with clue 15 dropped, the puzzle's
  query has 18 classes over the 32 states of the maximal line (run at the re-plan, `--maximal`). The short
  answer of such an instance is its class.
- **What a check costs depends on the question** ([E-17](../evaluation/bounds/E-17-what-a-run-costs-depends-on-the-question-asked-of-it.md)): `ein test` asks the least
  question a file's claims need, and a region claim asks for a description. A claim the budget leaves undecided
  is `NOT CHECKED`, and the instance is not admitted.
- **External benchmarks have answers and no theories.** Constraint precision and recall are undefined on them;
  faithfulness falls back to
  [Q-M2.3](../open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md)'s candidates. An
  adapter records which metrics an instance supports.
- **Contamination.** BBH is in every training set. The synthetic families are the half no model has seen, and
  [K4](../EinAf.md#k4-novel-relation-vocabulary)'s novel vocabulary is the test of whether a number is memory.

## Connections

- [`EinAf.md` § Stage C](../EinAf.md#stage-c--construct-a-heterogeneous-benchmark); [the frame](../README.md#the-plan-as-re-planned);
  [the audit](../analysis/audit_p2.3.md), and [the analysis](../analysis/README.md#p23--the-benchmark-audit)'s
  summary of it.
- [The evaluation of Ein](../evaluation/README.md): writing canonical programs for nine families is the largest
  test of the language's expressiveness this milestone has, and each stage that writes or runs them has a task
  for it.
- [`12-query.md`](../../../docs/ein/user/03-syntax-and-semantics/12-query.md), claims and who checks them;
  [`05-cli.md`](../../../docs/ein/user/04-reference/05-cli.md), `ein test` and the questions of `ein solve`;
  [`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md), the read-out and `ein-summary/2`.
- [`examples/word_problems/`](../../../examples/word_problems/README.md), the second domain, with its guide
  [`08-word-problems.md`](../../../docs/ein/user/02-guide/08-word-problems.md);
  [`examples/README.md`](../../../examples/README.md), the Zebra encodings.
- [F26](../../followups/f26_consistency_and_completeness.md), whose four classes *for M2* are this benchmark's finer
  split; [F27](../../followups/f27_cryptarithm_theory.md), a cryptarithm theory, one of whose triggers is a
  benchmark of puzzle families, and which this phase does not pull: no cryptarithm family is in version 1;
  [F25](../../followups/f25_roughness_found_outside_zebra.md), whose trigger is a corpus of real programs meeting
  one of its rows, which the anchors are.
- [M1h](../../m1h_beyond_word_problems/README.md), not started: it hands M2 pairs of a sentence and its facts,
  and its first phase plans a timeline and a causal theory as library modules, the two this benchmark's
  tracking and temporal families do without. [M1n](../../m1n_non_monotone_programs/README.md) owns what a flagged
  program's answer is.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md), the ladder of external sources and the BBH record
  format; [idea 09](../../ideas/09-puzzles-beyond-zebra.md), the menu of human puzzles;
  [`docs/lib/12`](../../../docs/lib/12-llm-and-reasoning-benchmarks.md), the catalogue.
- [M10](../../m10_external_benchmarks/README.md), the other benchmark: formal-language shaped, hand-encoded for
  several systems, not started. Where its data lives is its own open question
  ([Q-M10.3](../../m10_external_benchmarks/open_questions.md#q-m103--where-does-the-benchmark-live-and-is-any-of-it-a-gate));
  nothing is shared but the Zebra programs both read. [`nl2smt/`](../../../nl2smt/README.md) holds the Zebra
  puzzle's three texts and an independent count of their answers.
- [`utils/README.md`](../../../utils/README.md): the corpus's own generators.
  [`gen_zebra2_variants.py`](../../../utils/gen_zebra2_variants.py) writes four corpus files from `zebra2.ein`
  and stays their one writer (audit § C.2); [`gen_arith_examples.py`](../../../utils/gen_arith_examples.py) and
  [`region_synth.py`](../../../utils/region_synth.py) are the precedents for an answer computed outside the
  engine and a program written for a chosen region.

## Defaults taken at the re-plan

Each is recorded so that the user can correct it; none is a ruling.

1. **The gold of an instance is established outside the kernel and checked by it.** A generator counts the
   answers by its own enumeration, in Rust, and writes them as the program's claims; `ein test` is the second
   route. Where a family's enumerator is not written, the metadata says the class rests on the kernel alone.
2. **A class is stated by the shape of the canonical program's claims**, by
   [§ How a class is established](#how-a-class-is-established), with one query named as the answer query.
3. **F26's four classes are the finer split**, read as: *complete*; *under-determined by something the text does
   not state* (F26's *by a quantity*, widened to a deleted clue); *under-determined by a reading*;
   *contradictory*. Two marks the corpus shows are added: an answer on a marked assumption, and which kind of
   empty R an unsatisfiable instance has.
4. **The gold short answer of an ambiguous or unsatisfiable instance is its class**, with the alternatives beside
   it where they are few and each is short. How a class is scored is
   [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)'s.
5. **An unsatisfiable instance's canonical theory is written so that root is dead**; an instance that answers
   `Open` is admitted, marked, and counted apart.
6. **A canonical program states its lattice**, and one answered by the blind enumerator is not admitted.
7. **The families are C1's eight and the word problems.** The corpus's other families that state claims
   (cryptarithms, truth-tellers, Boolean circuits, beliefs: audit § B.1) are not in version 1.
8. **The sixteen word problems are taken in as copies**, with their four theory modules, each copy naming the
   corpus file and its digest; a problem whose `:goal-text` does not print a short answer has it rewritten, and
   its metadata says so. They and every hand-written anchor are in `dev` or excluded, never in `val` or `test`:
   [P2.2](../p2.2_formalizer/README.md) develops on them.
9. **Zebra's benchmark programs are written for the benchmark**: each fact's `:source` is its sentence of
   `nl2smt/puzzles/zebra.txt`, the query states its `:expect`, and the unsatisfiable one encodes the text's
   contradiction. The corpus's Zebra files and their generator are not touched.
10. **Generated instances are files in the tree**, flat in one directory a family, and a generator's `--check`
    regenerates and compares.
11. **No external item is checked in**, whatever its licence allows; an item kept as a fixture is the one
    exception, where the licence the stage read permits it.
12. **The Level B run opens `test` once**, as the milestone's acceptance says, and shows the table a family; its
    records are listed as having opened it.
13. **LogiQA, ReClor and GSM8K are not adapted.** F13's note names the first two in its second ladder; they are
    listed in [S2.3.3](s2.3.3_external_benchmarks.md) so that the omission is a decision the user can reverse.
14. **A split's digest is over ids and content digests**, so that an instance edited after the freeze is seen.
15. **The manifest is generated from the metadata**, never kept by hand, and has no `runs` column: `ein test`
    reads its question off a file's claims.
16. **`True`, `False`, `Unknown` are read off one program with two queries**, not off two runs each judged by a
    `Contradiction`; and an `Unknown` item is of class *ambiguous*. Both are tested on hand-read items before a
    number rests on them ([S2.3.3](s2.3.3_external_benchmarks.md)).
