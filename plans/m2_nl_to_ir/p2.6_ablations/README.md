# P2.6 — Ablations (Stage G)

**Depends on:** [P2.5](../p2.5_harness/README.md): the records, the metrics, the arms and the budget rule; every
ablation is more rows in that table's vocabulary. [P2.4](../p2.4_loop/README.md): every knob turned here is a
parameter the filter or the tool already has.
**Blocks:** [P2.7](../p2.7_failure_scaling_generalization/README.md): the failure sample is drawn from these
runs; the **Level C gate**.
**Feeds:** [the evaluation of Ein](../ein_evaluation.md): each ablation of the feedback is also a measurement of
one thing the kernel says.
**Research plan:** [`EinAf.md` § Stage G](../EinAf.md#stage-g--ablation-program), G1 to G9.
**Depth:** one stage file, [S2.6.4](s2.6.4_representation_ablations.md), which carries the link-grammar
experiment whole. The other three stages are the paragraphs below, written to stage depth when
[S2.5.4](../p2.5_harness/s2.5.4_first_table.md)'s table exists and says which of them matter.

---

## This is where the architecture becomes an experiment

The research plan's sentence, and the phase's. Up to here the milestone has built two arrangements of one model
and one kernel and shown a table; here it asks *which part does the work*, by turning one knob at a time with
everything else (the model, the prompts, the split, the budget, the seeds, the kernel's commit) held by the
record. The knobs are parameters the arrangements already have ([P2.4](../p2.4_loop/README.md): the level, the
strategy, the depth, the library the catalogue lists, the decomposition, the grammar, the thinking), which is why
an ablation is two configs and not two systems.

Every ablation is **pre-registered**: before it runs, its stage writes the reading each outcome would have (*if
G3 shows no gain from the core, the renderer of the core is not the contribution*), so that the table is read by
a rule written before it. The template is the rule the old plan wrote for link grammar, a sign and a threshold
with a consequence each, which [S2.6.4](s2.6.4_representation_ablations.md) keeps.

An ablation runs **as an experiment, by hand**, with its records, in no tier, as the main table does. Its budget
is the main table's: generated tokens, thinking included. A level that says more costs **prompt** tokens, which
are reported beside and are not what is matched.

## The research plan's nine and the rulings' three, in four stages

**S2.6.1 — Retry control and verdict, in both arrangements (G1, G2, the tool's level).** *Does improvement come
merely from additional inference?* One shot against a generic retry against a retry the kernel guides, at an
equal budget (G1): B2, B3 and B4 of the main table with the budget made the x-axis. Then a generic failure
against the verdict (G2): F0 against F2. These are the cheapest ablations and the ones the whole claim rests on:
if F0 matches F2 the kernel is a retry trigger, and nothing in F3 to F8 rescues that. Pre-registered reading: the
claim survives G1 only if B4 exceeds B3 at a matched budget, on `val`, on at least two families, with both
conditional probabilities moving the same way under each reading of *certified*. **The tool's level** is the
same question asked of the other arrangement (below).

**S2.6.2 — The content of the feedback (G3, G4, G5, G6).** Four comparisons, each a pair of levels or a pair of
renderings of one level: the verdict against the verdict with a core (G3), measured as repair success where the
program is inconsistent; a diagnostic without and with provenance (G4); a count of states against two states
and what differs between them (G5), with the hallucination rate beside the repair rate, because that level is
the one most able to leak an answer; and the feedback's size (G6), minimal, medium and the whole derivation,
because *more information may make the model worse* and the research plan says so. Each reading is a sign and a
threshold written before the run. What each reads of the kernel is the table below.

**S2.6.3 — Depth, strategy, and thinking before the program (G7; repair and regenerate; alternatives).** Repair
iterations 0, 1, 2, 3 and on, at F8 and at F2: the curve of marginal improvement against cost, per family. The
transcript already has every point (a run to depth *n* holds its prefix of depth *k* for every *k* below *n*),
so the curve costs one run an instance, not one a depth; for the tool the same curve is over its calls. Two rows
the earlier phases hand over belong here, since each is another way of spending the same budget: *repair* against
*regenerate* ([S2.4.2](../p2.4_loop/s2.4.2_repair.md)), and a reading left open emitted as hypotheses and pruned
by the kernel against one reading committed and caught by the loop
([S2.4.5](../p2.4_loop/s2.4.5_alternatives_as_hypotheses.md); [Q8](../open_questions.md#q8--ambiguous-parses)).
**Thinking before the program** is the third way (below).

**[S2.6.4](s2.6.4_representation_ablations.md) — Representation and library (G8, G9, the link-grammar arm, the
grammar off).** Direct generation of Ein against *text, then a structured semantic representation, then Ein*
(G8), with the user's four oracle ablations from [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) (*text
or program, known or unknown theory*) made arms; the link-grammar arm, which is the same question about the
input side, [Q11](../open_questions.md#q11--link-grammar-value); the library's three conditions (G9), which
answers *how much of the performance is reusable theory a person wrote* and also reads on
[F8](../../followups/f8_FCA_RCA_odis_tptp/ideas.md)'s `C(n)`; and **the filter with its grammar off** (below).

## The three the rulings added

Each follows from [Q-M2.13](../open_questions.md#q-m213--who-calls-the-kernel), and each is a pair of configs of
arms the main table already has.

| ablation | the pair | its stage | the question, and the reading written before it |
|---|---|---|---|
| **the grammar off** | B2 and B5 as they are, against B2 and B5 with no grammar over the tokens: the harness still accepts a program and nothing else, and a text the kernel refuses at the parse is an iteration spent | [S2.6.4](s2.6.4_representation_ablations.md) | The user: *"writing ein program may be not impossible without GBNF"*. [S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md) measures that on the seed set before a grammar is built; this is the same measurement on `val`, in the table's vocabulary. Read first on parse success, then on accuracy at a matched budget. If the filter without a grammar is within the tolerance of the filter with one, the grammar is not what makes the filter work and stays an option; if the grammar loses, constrained decoding costs content and that is reported |
| **thinking before the program** | B2 and B5 as they are (thinking off, the whole output a program) against B2 and B5 with the model's thinking on and the grammar beginning where the thinking ends ([S2.2.3](../p2.2_formalizer/s2.2.3_gbnf.md)); the thinking's tokens count in the budget | S2.6.3 | Is a chain of formal reasoning a replacement for a chain of thought, or its complement? Read against B5 and against B1 at a matched budget. If thinking first wins, the two complement each other and the filter's premise, *the program is the reasoning*, is too strong as stated; if it does not, the premise holds on this model |
| **the level the tool replies at** | B6 at F8, as in the main table, against B6 at F2 and at F1, the tool's lowest: a parse error is what the tool returns, so it has no F0 ([P2.4](../p2.4_loop/README.md)'s default) | S2.6.1 | Does what Ein says matter when the model chooses to call it? Read on accuracy, on calls an episode, and on whether the answer is what the last run printed. A program the kernel passes is answered with what it prints at every level ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md) owns the reply); the level is what the tool says otherwise |

## What each ablation reads of the kernel

The renderers are [S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md)'s. This is what they have to read from, as
the kernel stands, so that a pre-registered reading does not rest on something the kernel does not say
([`analysis/` § 2 and § 3](../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk);
[`audit_p2.5_p2.6.md`](../analysis/audit_p2.5_p2.6.md) § B.3, § B.5).

| | reads | what bounds the reading |
|---|---|---|
| **G1** | whether the kernel passed the program | nothing of the kernel's content |
| **G2** | [the outcome](../README.md#the-outcomes): seven, not the research plan's three words | *incomplete* and *inconsistent* are two kinds of empty R; *not decided* and *flagged* are neither a pass nor a refutation |
| **G3** | a dead root's core, its status and the rules that refuted | **filled under a description only**, `null` under `--single` ([Q-M1f.29](../../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)); an explanation, not minimal and not canonical ([Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md); [F22](../../followups/f22_exact_core_past_the_cap.md)); `[]` is a legal core; the config's `record-alternative-justifications` may change it, so the record pins it; the source sentence of a core's fact is a value only after [S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md); and a fact the program writes with a provenance of its own (`:rule`, `:using`) leaves every core, so a model that writes one gets an emptier core than its program has ([F24](../../followups/f24_written_provenance.md)). A program whose R is empty because every alive state owes has no core: it has what is owed |
| **G4** | which rule fired on which premises, from the events or the trace | a firing's premises omit the absences and the activators it read; an obligation rule and a hypothesis rule never fire (an obligation's event is `owe`) |
| **G5** | a count, and two states | `--single` stops at the second state and lists the two; the count is the maximal line's listing or `k_s` under a description, which a budget can stop first. Which count the level says is S2.4.3's. **Nothing in the kernel computes what differs between two states**: the difference, and which part of it is *relevant*, are EinAf's |
| **G6** | the region trace, whole or pruned to the goal's slice | the size is prompt tokens and has to fit the context, which is a field of the record |
| **G7** | the transcript | nothing more |
| **G8**, **G9**, link grammar, the grammar off | [S2.6.4](s2.6.4_representation_ablations.md) | |

## Acceptance

- Twelve ablations, the research plan's nine and the rulings' three, and the two strategy rows: each a
  pre-registered reading, a pair or a family of configs, a table from records, and the reading applied, in that
  order, with the pre-registration committed before the records exist.
- G1 and G2 are reported first and in full, the case where they refute the claim included: Stage O admits a
  negative result, and this phase is where one would appear.
- G5 reports the hallucination rate beside the repair rate; G6 reports prompt tokens beside accuracy; G9 names
  what each condition's programs may import, and the closure that gives; the grammar off reports parse success
  beside accuracy.
- Every table shows both conditional probabilities under each reading of *certified*
  ([Q-M2.10](../open_questions.md#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r)), and the flagged in
  a column of its own.
- [Q11](../open_questions.md#q11--link-grammar-value) is resolved by the old rule, and the code matches the
  verdict.
- What each ablation showed of the kernel (a level that did not help, a thing the kernel could not say, a cost)
  is in [the evaluation of Ein](../ein_evaluation.md).

## Risks

- **Twelve ablations on one `val` split are multiple comparisons.** The stage that writes the readings also
  writes how many comparisons are being made and what that does to the thresholds. No ablation opens `test`:
  its two sanctioned openings are the Level B run and [P2.10](../p2.10_result_artifact_demo/README.md)'s
  ([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)).
- **Variance.** Every cell is three seeds and reports the spread, or the resampling over instances where the
  sampler does not sample ([S2.5.2](../p2.5_harness/s2.5.2_baselines.md) T2.5.2.3). That is the price of a claim.
- **The interesting negative.** If G6 shows that the whole trace *hurts*, the temptation is to fix the renderer
  and run again. The rule: the renderer that ran is the one reported; a better renderer is a new record and a
  new row, never a replacement.
- **An ablation of a level is only as good as what the kernel says at it.** Where the table above says a
  reading is bounded (a core that is not canonical, a count a budget can stop), the pre-registration says so
  before the run, and the bound is an entry of the evaluation whatever the ablation shows.

## Connections

- [`EinAf.md` § Stage G](../EinAf.md#stage-g--ablation-program).
- [P2.4](../p2.4_loop/README.md): the knobs; [P2.5](../p2.5_harness/README.md): the table these rows join;
  [P2.7](../p2.7_failure_scaling_generalization/README.md): where the failures these runs produce are classified.
- [The frame § The arms](../README.md#the-arms): where the grammar off and the filter after a chain of thought
  are named as ablations; [the ledger](../open_questions.md): Q-M2.13, Q8, Q11.
- [F22](../../followups/f22_exact_core_past_the_cap.md), the exact core, whose trigger is a consumer that needs
  the minimal core; [F24](../../followups/f24_written_provenance.md), what a written provenance does to a core,
  whose trigger is a consumer that reads cores of programs it did not write. G3 is both.
- [M12](../../m12_rules_as_data/README.md): its census of rule shapes is the count S2.6.4 takes of a model's
  rules, taken of the corpus by hand.

## Defaults taken at the re-plan

Each is this re-plan's, recorded so that it can be corrected; none is a ruling.

1. **The three ablations the rulings added are placed in the existing stages**, by what each varies: the tool's
   level with G1 and G2, thinking before the program with depth and strategy, the grammar off with representation
   and library. No stage id is added.
2. **The two strategy rows are S2.6.3's**: repair against regenerate, and alternatives as hypotheses against one
   reading committed. The old plan named them as rows of this phase and gave them no stage.
3. **The grammar off and thinking before the program are measured on B2 and B5**, the two ends of the filter,
   not on all four of its arms.
4. **The tool's level is measured at F1, F2 and F8**: the tool's lowest level, and the two the filter's arms
   B4 and B5 have.
5. **G9's three conditions are lists of what a program may import**, enforced by the harness in every arm and
   by the grammar where one is on; the library on disk is the same in all three, so its manifest's SHA is the
   same, and a condition is named by its list and the closure the list gives.
6. **A family's own modules are importable under G9's full condition only**, and the word problems run under
   it only: their theory is the calculator and the family's four modules, which no smaller condition holds and
   no model is asked to write.
7. **The link-grammar arm uses the installed `link-parser` through its command line**, with no submodule and no
   binding, and puts a linkage into the prompt as plain text, never as Ein.
8. **`C(n)` under the smallest library is counted at more than one reading of *the same shape***, each named:
   which reading is kept is [M12](../../m12_rules_as_data/README.md)'s open question, and the user's.
