# M2 — the plan against the project (the analysis M2 starts from)

**Taken:** 2026-11-25, on `458bf90b`, with the engine rebuilt to that commit (`ein 0.1.0`, `ein-events/1`,
`einb/2.2`). The user started M2 with it: *"start plans/m2_nl_to_ir from plans analysis"*.
**What it is:** the milestone's plan, written 2026-08-23 against the engine M1a shipped, read against the project
five kernel milestones later (M1c, M1d, M1e, M1e2, M1f). Nothing of the plan is rewritten here but the three
colliding question ids (§ 6); this page says what of it holds, and the ledger holds the questions it raised.
**How it was taken:** six read-only audits, one per group of phases, each checking every statement the plan makes
about the project in the tree or by running the binary, and each reporting in the same six sections (stale claims;
what each stage needs and its state today; conflicts with standing rules; open questions; dependencies; estimates
and counts). The reports are kept whole beside this page; the claims this page leans on hardest were run again by
the coordinator (§ 2's table) and agreed.
**After it:** the user answered the questions (§ 7), the host was checked again, twice ([`host.md`](host.md)),
and **the plan was re-planned from this page and the audits** ([the milestone's README](../README.md#the-plan-as-re-planned)). §§ 1 to 6 read as taken: *open* there means
open when the analysis was written, and **the audits cite the plan's files as they stood at `5d678042`**, by the
names and line numbers of that commit. Four stage files have other names since (`s2.1.1_census.md` is `s2.1.1_tree_and_outcomes.md`; `s2.1.2_feedback_object.md` is `s2.1.3_feedback_object.md`;
`s2.1.3_boundary.md`, whose question the user ruled, is gone, and S2.1.2 is `s2.1.2_diagnostics_surface.md`;
`s2.2.2_llama_server_and_client.md` is `s2.2.2_model_runtime.md`), and every file was rewritten:
`git show 5d678042:plans/m2_nl_to_ir/<path>` has what an audit read. [The ledger](../open_questions/README.md) says what
is open now. **[`probes/`](probes/)** holds the small programs the re-plan's writers ran to check what the plan
says of the kernel, a folder a phase: each is the evidence of an entry of
[the evaluation of Ein](../evaluation/README.md), which names the command it was run under.

| audit | covers | stale | found true |
|---|---|---:|---:|
| [`audit_p2.1.md`](audit_p2.1.md) | P2.1, the kernel as instrumentation; **§ 0 is the fact base**: what the machine surface says today in each situation of Stage A1 | 67 | 35 |
| [`audit_p2.2.md`](audit_p2.2.md) | P2.2, the formalizer; 26 of the plan's Ein snippets run against today's language | 32 | 46 |
| [`audit_p2.3.md`](audit_p2.3.md) | P2.3, the benchmark; the eight families against the corpus | 32 | 36 |
| [`audit_p2.4.md`](audit_p2.4.md) | P2.4, the loop; the feedback ladder against what the engine supplies | 33 | 27 |
| [`audit_p2.5_p2.6.md`](audit_p2.5_p2.6.md) | P2.5 and P2.6, records, baselines, metrics, ablations | 25 | 31 |
| [`audit_p2.7_p2.10_neighbours.md`](audit_p2.7_p2.10_neighbours.md) | P2.7 to P2.10, the milestone's README and ledger, and every plan and followup that names M2 | 46 | 57 |

Every link in the folder resolves (`python3 utils/doc_audit.py --links --check plans/m2_nl_to_ir`): the folder was
swept mechanically each time a page moved. **What is stale is what the sentences say, not where they point**, and
a link check cannot see it.

---

## 1. The result, in one paragraph

**The research plan stands and so does the milestone's shape.** [`EinAf.md`](../EinAf.md) (the question, the
levels, the stages A to Q), the ten phases, their order and the experimental design (the feedback level as the
variable, matched budgets, faithfulness beside correctness, a benchmark with ambiguous and unsatisfiable
instances, pre-registered ablations, records before tables) read today as they read in August. **What does not
stand is the kernel the 22 stage files were written against.** They describe the walk: `ein solve -e`, `k`,
`exhausted`, `--max-set-size`, a verdict `type` on every run, `ein-summary/1`, `hrule`, `defined_behaviour.md`.
All of it is gone. The answer is a region R, a bare `ein solve` has no verdict word, and the engine now says for
itself most of what P2.1 planned to derive. So **no stage can run as written**, some are moot or much smaller, and
a handful of questions exist now that had no reason to exist in August (§ 5, § 7). The first of them, where
EinAf's code and data may live under the project/process rule, has to be answered before a single file of it is
written.

## 2. What the kernel says today, where the plan expects the walk

The situations of [Stage A1](../EinAf.md#a1-freeze-the-semantic-boundary-of-ein), run on corpus programs. Every
row was run twice, by the P2.1 audit and by the coordinator. The summary is `ein-summary/2`
([`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md)), top-level keys `asked, region, verdict,
queries, claim, work, monotonicity, leftover, root, config`.

| A1's situation | command | exit | what says it |
|---|---|---:|---|
| invalid program | `solve examples/broken/unclosed_paren.ein` | 1 | stderr text (`FILE:4:10: unexpected …`, then `repair:` lines). **No summary is written**; `ein test --json-report` says `outcome: "error"` with no message |
| valid, inconsistent | `solve tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | `region.empty "dead"`, `k_s 0`, `exhausted true`, `core ["(loops A A)"]`, `core_status`, `refuted_by ["irreflexive"]`; `verdict` is `null`. Under `--single`: `verdict.word "Contradiction"`, and **the core is `null`** |
| satisfiable, incomplete | `solve tests/region/04_owing.ein` | 0 | `region.root "owing"`, `empty "owing"`, `root.owed ["likes owes Ann a Food."]`; under `--single` the word is `Open` |
| several models | `solve examples/zebra2-minus-15.ein --single` | 0 | `verdict.word "Ambiguity"`, two states listed, `stopped {why: "asked"}`. `--maximal` lists the 32, exhausted. A bare solve finds the 32 states and then stops at the default budget after about 10 s: `exhausted false`, `k_c` a lower bound |
| one model | `solve examples/zebra2.ein --single` | 0 | `verdict {word: "Solution", which: "all"}`, `exhausted true`, 7 saturations (a bare solve: 120) |
| resource limit, facts | `… --single --region-budget 1` | 0 | `verdict.word null`, `exhausted false`, `stopped {why: "budget", …}`, a `note:` on stderr |
| resource limit, clock | `… -T 0` | 2 | one stderr line, `** aborted: … **`. **No summary** |
| *a seventh, not in A1* | `solve examples/lattice/04_obligation_trigger_hole.ein` | 0 | `WARNING: … THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER` on stdout and stderr, `monotonicity.warning` in the summary; an answer is printed and nothing is claimed of it ([Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) |
| *a refuted claim* | a program whose `:expect` the run refutes | 1 | the summary **is** written, `queries[].expect.outcome "failed"`: exit 1 is not *invalid* |

What the plan's vocabulary became:

| the plan says | today |
|---|---|
| `ein solve -e` certifies; `-m`, `--max-set-size`, `--max-enterings` bound | all retired, exit 2 with what replaced each. The questions are a bare `solve` (describe R), `--single`, `--minimal`, `--maximal`, `-n N`, `--count`, `--no-search`; the bounds are `--region-budget` (facts, exits 0 with what it established), `--region-limit`, `-T` |
| `verdict.type` ∈ Solution, Ambiguity, Contradiction, Aborted, on every run | `verdict.word` ∈ Solution, Ambiguity, Contradiction, **Open**, **only under `--single`**; `null` there is *not decided*; a bare solve has no word |
| `k`, and *unique = `k = 1 ∧ exhausted`* | two counts, `k_s` (states) and `k_c` (commitments; 1 against 261 982 on `zebra2`), both `null` under a mode. **R holds partial commitments where a program states no obligation** (`examples/features/11_expect_ambiguity.ein`: two full seatings, `k_s = 7`), so one program is `Ambiguity` over all of R and `Solution` over its maximal line: *one solution* has two readings and the program has to say which ([`06-solution-semantics.md`](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md)) |
| `unknown` does not exist; M2 derives it from `(type, k, exhausted, core)` | shipped by M1d ([Q-M1d.6](../../../docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)): no stopped run says `Contradiction`. `exhausted false` alone is not *unknown* either: see the bare solve above |
| *satisfiable incomplete* waits for M1d | a state **owes**: `(open R)` facts, `root.owed`, `region.frontier`, `Open` |
| `verdict.unsat_core`, entries with their `:source`, a non-empty core as the tell of a real contradiction | `region.core` (strings), `core_status`, `refuted_by`, **under a description only**. `[]` is a legal core. A core is an explanation, may vary with rule order and is not minimal ([Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md), [F22](../../followups/f22_exact_core_past_the_cap.md)). `:source` text is in no JSON key |
| `solutions[].facts`, `goal_bindings`, `stats` | `region.states[].facts` (over the candidates' relations; a whole state is in no JSON key, `--dump-answer` writes each as loadable `.ein`), `queries[].classes[].rows`, `work {saturations, facts, layers}`. No surface carries a time or memory |
| the loader refuses an undeclared relation, a wrong arity, a type mismatch (`arity_mismatch`, `type_error`) | **all three load in silence, exit 0.** Ein has no types ([Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)) and a relation is overloaded by its count ([Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)). What is refused is [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2 |
| a fact beside its negation is a contradiction | only where the program imports `std.contradiction` ([Q-M1f.557](../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)) |
| `hrule`, `:hrules`, hypothesis levels, a `(not h)` writeback | parse errors, or gone. A hypothesis rule concludes `(open F)` ([Q-M1f.567](../../../docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)); with no writer of H **the blind enumerator guesses** (a ten-line probe: 83 candidates, a 69.6 MB summary, exit 0) |
| `(transitive left-of)` selects a theory | only under a name the file lists in `:symbols` from the module that declares it (or its full name, or an alias); otherwise the fact is the file's own and **inert, with no message**. `(bijective R)` also needs its two hierarchy activators |
| `:why` optional; a fact reads through its relation's `:why` | `:why` is required on every relation, rule and macro; a fact takes no text from the declaration ([Q-M1f.58](../../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)) |
| an unknown keyword on a fact (`:origin generated`) is to be measured | answered: a parse error, each form takes a closed list (Q-M1e2.28) |
| `ein test` *not shipped yet*; gold verdicts checked by `ein solve -e` | shipped: `(query … :over … :expect …)`, `(solve … :expect (region …))`, `ein test DIR --json-report` (`ein-test-report/6`), `--diff full`. `ein oracle` is the brute-force reference, and refuses above 16 candidates (`zebra2` has 56) |
| the corpus is 128 entries; `examples/broken/` 37 fixtures | 933 entries; 64 fixtures, one of which solves |
| `ein --version` identifies the build | five lines, the stdlib's manifest SHA among them, **no commit** |
| `defined_behaviour.md`, `docs/kernel/`, `docs/history/`, the root README's *§ EinAf* | none exists. `docs/ein/` in three perspectives, `docs/decisions/`, and a root README that says the loop is not in this repository |

## 3. Phase by phase

Each cell is a summary; the audit named in the heading has the evidence, claim by claim.

### P2.1 — the kernel as instrumentation ([audit](audit_p2.1.md))

- **Its central premise is discharged.** *The sixth outcome, today* and the `unknown` derivation are moot; so are
  the two measurements of T2.1.1.2 (one answered by Q-M1e2.28, the other already a gate test over the stdlib,
  with a trap for a reader of `fire` alone: an obligation rule emits `owe`, never `fire`).
- **Most of `ein-feedback/1`'s content ships in another form**: `search_complete` (`region.exhausted`), the
  count, the obligation-level *incomplete*, the query axis (`queries[].classes`, `over`, `exact`), the models,
  the core with its status and the rules that refuted.
- **What is still missing is one thing: no machine surface carries a diagnostic.** A refusal, a `-T` abort, a
  load's `note:` lines, a core's `:source`, the core under `--single`. The same gap is on file as
  [F23](../../followups/f23_diagnostics_in_eins_words.md)'s D-18 and L-125, which the plan does not cite.
- **The recommended producer, `ein solve --feedback` inside `ein-cli`, meets rules made since**: it is the
  project, so its page and comments may not name the loop or a stage; it moves `help_shape.txt` and
  `06-config.md`'s option census, against the stage's own *no golden re-blessed*; and it would be the first
  artefact a refused program writes.
- **Three planned invariants are false today**: *contradiction ∧ exhausted ⇒ core ≠ ∅*, the core as *smallest*,
  and *no `(X, ¬X)` pair in a solution*.
- **S2.1.1, the census, is largely this analysis.** What is left of it is one list: A1's outcomes on
  `ein-summary/2`'s keys, with the seventh.
- **Q25, the loop's language, is open**, and both columns of its trade grew: a Rust caller gets the whole answer
  as values (`ReadOut`, `Verdict`); a process-driven one gets `ein-summary/2`, `ein-answer/3` (states as loadable
  files), `ein-lattice/2`, `ein-test-report/6`, `ein-events/1`, `h-space.json`. Four load-side errors are still
  `String` newtypes.

### P2.2 — the formalizer ([audit](audit_p2.2.md))

- **The target shape exists, but not in the program the plan calls gold.** `examples/zebra.ein` (over
  `std.slots`) less its one rule, its activators and its config is imports, five `relation` forms, six library
  activators, facts with `:source` and one query, and `--single` answers `Solution` with the canonical bindings.
  `examples/zebra2.ein`, which S2.2.1 names as *the program the formalizer is meant to reproduce*, carries
  fifteen rules of its own, fourteen of them in no stdlib module.
- **The four parts (ontology, theory, instance, query) name no hypothesis mechanism, and the answer depends on
  it.** With the library's obligations activated the hypotheses come for free; with none the blind enumerator
  answers (the contract's own example, minimally completed: 160 states, `Ambiguity`). *Loaded and sat idle* is no
  longer what an untheorised program does.
- **Selecting a theory is spelling it** (§ 2's `:symbols` row). No catalogue says how a property is spelled to be
  active; its parts exist (`stdlib/README.md`'s table of 28 modules, each rule's `:why`, the generated rule
  regions under `04-stdlib/`, `ein saturate --dump`).
- **Of the plan's snippets**, the contract's example is refused, `hrule` and `:hrules` are parse errors, a
  Cyrillic atom is refused (a Russian text needs ASCII names), `7` and `07` are two names.
- **The provenance marking is decided by the language**: no keyword on a fact, so the sidecar. Its key, and four
  other tasks, rely on *the binary's canonical dump*, which no command prints (`ein_ir::dump_canonical` is
  library-only). That bears on Q25.
- **GBNF**: `01-grammar.md` holds no grammar text. The source is `ein.l` and `ein.y` (LR(1), 101 rules) or the
  twenty generated EBNF diagram sources.
- **Smaller since**: the Zebra text exists (`nl2smt/puzzles/zebra.txt`, with a clue-dropped and a contradictory
  edit; there is no `examples/zebra.txt`); the walkthrough has a sentence-to-fact table; sixteen word problems
  carry each sentence as `:source`.
- **The host** (checked again, [`host.md`](host.md)): an RTX 4060 Laptop GPU with 8 GB, 31 GB of RAM, Docker and
  Compose with the GPU's devices listed. `qwen3.5:4b`, the model the user then chose, is in ollama's system store
  as a GGUF file, which the audits did not look in. `llama-server` was installed after the audits and does not
  start (it needs OpenSSL 4); ollama 0.35.1 does not honour a GBNF grammar; and neither runtime has a GPU backend
  installed, so a model runs on the CPU. Q23's working answer, a 30B model, did not fit that GPU and is gone.

### P2.3 — the benchmark ([audit](audit_p2.3.md))

- **The certification it is built on is gone** (`ein solve -e`, in sixteen places). Its replacement is better
  than what was planned: a canonical program states its answer as `:expect` and `ein test DIR --json-report`
  checks it, which is the kernel half of the plan's `einaf bench check`.
- **The three verdict classes do not map onto the kernel**, in two ways. The engine has four words and two
  non-answers (*not decided*, *flagged*). And **the word is not the problem's class**:
  `examples/word_problems/gap.ein`, whose right answer is *not enough information*, says `Solution` (the gap is a
  fact the program derives); `boxes12.ein`, with one answer, says `Ambiguity`.
  [F26](../../followups/f26_consistency_and_completeness.md)'s item 6 holds four classes *for M2*; the plan cites
  neither.
- **`:expect` is not what the plan describes**: no `none`; unsatisfiable is `(false)`; `(model)` is *the goal has
  no row*; a region claim and a query claim are two sites.
- **The corpus against the eight families**: attribute matching alone has a real anchor. Five have single-rule
  demonstrations with no `:expect`; object tracking has nothing; of the old five puzzles only Zebra exists, and
  no Zebra file states an `:expect`, its `:source` texts being `"condition (N)"`, not the sentence.
- **`examples/word_problems/` is the only set of sentence-to-fact pairs in the corpus**: sixteen problems over
  four theory modules, 31 claims held, all monotone, covering complete, assumption-resting, gap-named,
  two-readings and contradictory cases. P2.3 never mentions it; the milestone README does.
- **Risks the plan does not have**: a closed-world theory written with `absent` (a minimum, a *can* question,
  ProofWriter's CWA) may be flagged, and a flagged program has no validated answer; a core as a gold answer is
  not canonical; the `(not C)` contract needs `std.contradiction`.
- **`benchmark/` at the root is unplaced** (§ 5.1), and under a corpus root 4 080 instances would each need a
  `corpus.toml` entry.
- **Generators**: `gen_zebra2_variants.py` (in `utils/`, a text edit, no engine) is not *the one generator*;
  `region_synth.py` and `gen_arith_examples.py` are nearer. None realises English. No external data is in the
  tree; `docs/lib/12` lacks CLUTRR.

### P2.4 — the loop ([audit](audit_p2.4.md))

The ladder, level by level:

| level | the plan's source | today |
|---|---|---|
| F1 syntactic | *the loader's and the grammar's already; the residue is nothing* | the three checks the plan names do not exist as refusals (§ 2); diagnostics are stderr text. The residue is most of it |
| F2 verdict | `verdict.type × exhausted × core` | `verdict.word` under `--single`, `region.empty`, `region.stopped`, `monotonicity.warning`: of a named part of R |
| F3 cardinality | `verdict.k`, `goal_bindings` | `k_s`, `k_c`, `queries[].classes[].rows`; which count is meant is § 5.4 |
| F4 model difference | a set difference of two sorted fact lists, rendered through the relation's `:why` | nothing computes a difference of two states (`ein test --diff` is a claim against what was found); the rendering leans on a mechanism Q-M1f.58 removed, and [M1p](../../m1p_ein_to_natural_language/README.md) is its unstarted replacement. **The acceptance line fails on data**: the five smallest pairwise differences of `zebra2-minus-15`'s 32 states are over `pet-loc` alone, not the dropped clue's relations |
| F5 unsat core | `verdict.unsat_core` with `:source` | `region.core` under a description only, no `:source` in JSON, not canonical. [F22](../../followups/f22_exact_core_past_the_cap.md) and [F24](../../followups/f24_written_provenance.md) each name a consumer like M2 as a trigger |
| F6 dependency | *nothing in the engine* | `ein-infer/src/strata.rs` is the rules' relation dependency graph, library-only. Reachability from a goal does not exist. Partial counterparts: what a state owes, `root.hypgen.outside`, `h-space.json`, `--trace --relevant` |
| F7 provenance | `fire` events, the trace | as planned, with limits: premises omit absences and activators, and an obligation or hypothesis rule never fires |

- **S2.4.5, alternatives as hypotheses, is largely shipped.** `examples/word_problems/readings.ein` and
  `boxes12.ein` / `boxes13.ein` encode a sentence with two readings as candidates with root owing one
  ([Q-M1f.332](../../../docs/decisions/corpus/Q-M1f.332-a-word-with-two-readings-always-has-one-chosen.md)).
- **S2.4.1's `state_digest` has no source**: no JSON key holds a state's fact list, the reasoning page says a
  digest never decides identity, and `.einb` is never an interchange format. New material for Q-M2.4: state files
  that load as themselves, and `03-rule-evolution/`'s transformations that leave R unchanged.
- **The state machine has no case for a flagged program**, and *read the budget from the feedback object, never
  from exit code 2* has nothing to read after a `-T` abort.

### P2.5 and P2.6 — records, baselines, metrics, ablations ([audit](audit_p2.5_p2.6.md))

- **H4's kernel metrics name a block that no longer exists** (`stats`, enterings). Today `work {saturations,
  facts, layers}`; no time on any JSON surface, no memory anywhere, the `counters` feature compiled out of the
  shipped binary.
- **`work`, the core and the counts depend on which question is asked, and the plan never chooses one**
  (`zebra2`: 7 saturations under `--single`, 120 described; the ambiguous variant: 0.07 s under `--maximal`,
  10.6 s described).
- ***certified := k = 1 ∧ exhausted* has no one-to-one translation**, and no clause for a flagged program
  (§ 5.3, § 5.4).
- **The record's kernel fields changed**: no commit in `--version` (it comes from git); the resolved config is
  two flags, not seventeen fields; a `-T` abort and a refusal write no summary; a description of an ambiguous
  instance is 1.3 MB, not kilobytes; `--events` multiplies the wall clock three to six times.
- **`experiments/` at the root is unplaced** (§ 5.1), and `docs/history/`, where the plan sends measurement
  documents, is deleted.
- **G9, no stdlib / minimal / full, means something else**: `std.core` is in every program, an import brings a
  module whole, and a library `$EIN_STDLIB` names without `core.ein` is refused. Without `std.contradiction` no
  fact-beside-negation kills, which moves H1's contradiction detection.
- **The link-grammar arm's runner partly exists**: `nl2smt/` drives the installed `link-parser` with no
  submodule. The plan's `(link-grammar … "the" …)` fragment is not loadable Ein.
- S2.5.4's *the test runs in the gate* meets *the tests read only the project*.

### P2.7 to P2.10 ([audit](audit_p2.7_p2.10_neighbours.md))

- **P2.7.** Three of Stage J's knobs have no single reading today: *model count* (`k_s`, `k_c` or the maximal
  line), *hypothesis depth* (no depth exists; `region.minimal.least` is nearest), *proof depth* (no such number
  found). The solver curve *already measured once* was the walk's. A kernel finding goes to the wrong-answer
  register or a findings page, not *to M1d*. K3 counts on two families having no stdlib theory, and
  [M1h](../../m1h_beyond_word_problems/README.md)'s P1h.1 plans a timeline and a causal theory as stdlib modules.
- **P2.8.** Its dependency is false: [M10](../../m10_external_benchmarks/README.md) has run no stage, so there
  are no installed systems and no runner to reuse; hand-written encodings exist for one target of five. On this
  host `z3`, `cvc5`, `souffle`, and since the audits `swipl` and `clingo`, are installed; `lean` is not
  ([`host.md`](host.md)). `nl2smt/` is a second
  NL-to-SMT route the plan does not know.
- **P2.9.** **The kernel half of the formal account is largely written**:
  [`docs/ein/reasoning/`](../../../docs/ein/reasoning/README.md) states Ein as mathematics, true of any
  implementation, with one notation table, and `ein.rs/08-reasoning-in-code/` pairs each page with its tests. It
  differs from Stage M's sketch where the engine does: the answer is a set of commitments over H, not a set of
  models; the verdict has four values and is asked of a part of R; saturation is monotone only where no rule
  reads an absence. Of the three divergences the phase says it must add, the first is already written, the
  second is stale as put, and the third (*lower bounds are refutations, never one candidate owed*) is false:
  obligations are first-class. **What is new is the loop's half**, and its letters (`E`, `θ`, `T`, `x`, `v`,
  `Γ`) collide with the notation table. Where the account is written is not said. Its depth waits for M5 to fix
  a venue, while M5 is *after M2*.
- **P2.10.** It closes M2 by a dated status and a record in `docs/history/`; today a finished plan is deleted,
  and no rule says where research records live after that. Two things are called *the demo*: P2.10's one command,
  and [`docs/demo/`](../../../docs/demo/README.md), the article and talk whose fifth beat is M2's subject.

## 4. The neighbours

What the rest of `plans/` says M2 does, owes or takes, against what M2's plan says
([the table and the list of eleven](audit_p2.7_p2.10_neighbours.md#e-the-neighbours)):

| neighbour | what it is to M2 | state |
|---|---|---|
| [M1p](../../m1p_ein_to_natural_language/README.md), Ein to natural language | owns the direction Q7 parks with M5 or M20, and says the two meet in a round trip | **four texts, three owners**: `plans/README.md`, `AGENTS.md` and the root ledger's Q20 give M2 the NL explanation; M2's ledger gives it away |
| [M1h](../../m1h_beyond_word_problems/README.md) | *producing those facts from the text is M2's*; hands over pairs of sentence and facts, and fifty graded questions; gives M2 by name *the solution could be correct, but reasoning not* | not cited by M2 |
| [M1g](../../m1g_logic/README.md) | offers a formula language with checked semantics as a formal target; owns F1b's question, which P2.9 also claims | not cited; no order ruled between the two |
| [M10](../../m10_external_benchmarks/README.md) | P2.8 leans on its S10.2 and its runner | not started, and stale in the same way (`-m 38`) |
| [M5](../../m5_presentation/README.md) | Stage P | header and body disagree (the research question against *the Ein system* written up); circular wait with P2.9 |
| [M15](../../m15_mcp_server/README.md), an MCP server | would also define how the kernel's answer reaches a model, and lists M2's formalizer as a client | overlap with P2.1, no order |
| [M12](../../m12_rules_as_data/README.md) | its rule census took by hand the count S2.6.4 takes by machine; ten candidate canonicalisations, none picked | not cited |
| [M1n](../../m1n_non_monotone_programs/README.md) | owns what a flagged program's answer is | not cited; the loop will generate flagged programs |
| [F23](../../followups/f23_diagnostics_in_eins_words.md), [F22](../../followups/f22_exact_core_past_the_cap.md), [F24](../../followups/f24_written_provenance.md), [F26](../../followups/f26_consistency_and_completeness.md), [F27](../../followups/f27_cryptarithm_theory.md) | each names M2, or a consumer like it, as a trigger: diagnostics on a machine surface, the exact core, written provenance in a core, the four benchmark classes, a benchmark of puzzle families | none cited by M2 |

## 5. What the standing rules now ask that the plan does not answer

These are the conflicts that are not a matter of re-aiming a sentence. Each is a question in the ledger.

1. **EinAf is on no side of the project/process partition**
   ([Q-M1f.345](../../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)). The plan
   puts a harness in `einaf/`, pages in `docs/einaf/`, grammars in `grammars/`, a benchmark in `benchmark/`,
   records in `experiments/`, a Compose file in `packaging/`. None is in the project's list, the process's, or
   the external one (`c/`, `nl2smt/`, `docs/lib/`), and `utils/check_release_voice.py` takes a new top-level
   directory as the project's by default. As the project they may hold no date, no measurement
   ([Q-M1f.349](../../../docs/decisions/docs/Q-M1f.349-a-page-of-the-project-states-no-measurement.md)), no
   record of a step of the work
   ([Q-M1f.341](../../../docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md)) and no
   name of a stage; **the research plan's own labels F0 to F8 match the followup-id pattern and fail the check**.
   As the process, under `plans/`, they are deleted with the folder at the close.
   → [Q-M2.8](../open_questions/einaf/Q-M2.8-where-does-einaf-live.md).
2. **A machine surface for diagnostics is a kernel change**, on F23's ground, and the plan's form of it names the
   loop inside the project. → [Q-M2.9](../open_questions/kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md).
3. **A flagged program has no outcome in the plan**: not in A1's six, the state machine, *certified*, either
   conditional probability, or the benchmark's gold. → [Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md).
4. **The loop has to choose a question and a reading of *unique***: which of `--single`, a description and
   `--maximal`; over all of R or its maximal line; `k_s` or `k_c`.
   → [Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md).
5. **Ids.** M2's Q23, Q24 and Q25 reused ids the root ledger gave M1's questions, two of which are decision
   files ([`Q23`](../../../docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md),
   [`Q25`](../../../docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md)). Renumbered
   with this analysis (§ 6).
6. **Where an answer lands.** M2's ledger says beside its question, and that `docs/decisions/` *never existed
   and is not coming*; it exists, for answers about Ein
   ([Q-M1f.346](../../../docs/decisions/docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)). Whether
   an answer about EinAf is *about Ein* follows from 1.
7. **No estimates** (the user, 2026-11-08): every file of the plan carries them. **Dates as status**, and a
   close into `docs/history/`, against *a finished plan is deleted*.

## 6. Changed with this analysis

- **M2's Q23, Q24 and Q25 are Q-M2.5, Q-M2.6 and Q-M2.7**, in the ledger and in every file of the folder that
  cited them, by the rule that an id is never reused. Nothing else of their text moved but a *Since* line each.
- **Four questions are added to the ledger**, Q-M2.8 to Q-M2.11 (§ 7).
- The README's status line, `plans/README.md`'s row and `AGENTS.md`'s pointer say M2 is started.

Nothing else of the plan is edited: the stage files read as they did.

## 7. What the user ruled, and where the re-plan is

**The user, 2026-11-25**, in two rounds of answers, on the questions this analysis raised, on four older ones and
on three the answers themselves raised. Each ruling is in [the ledger](../open_questions/README.md) in the user's words,
with what it is read to mean:

| Q | the question | the user | what it changes |
|---|---|---|---|
| [Q-M2.8](../open_questions/einaf/Q-M2.8-where-does-einaf-live.md) | where EinAf lives | *"./einaf/, ./docs/einaf/, etc..."*, then *"own tree, in this repository, outside of ein … but inside the development process"* | EinAf is a second project, over ein.rs, under the same process; the root README and `AGENTS.md` state the hierarchy |
| [Q-M2.9](../open_questions/kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md) | a machine surface for diagnostics | *"(b)"* | the kernel grows it in its own terms: S2.1.2; the feedback object is EinAf's |
| [Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md) | a flagged program | *"(a)"* | an outcome of its own, never certified, fed back with its sites |
| [Q-M2.7](../open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md) | the loop's language | *"Rust"* | EinAf links the crates |
| [Q-M2.2](../open_questions/model/Q-M2.2-must-the-model-be-local.md), [Q-M2.5](../open_questions/model/Q-M2.5-local-model-choice.md) | the model | *"yes"*, *"Qweb3.5:4b"* | local, `qwen3.5:4b` |
| [Q-M2.14](../open_questions/model/Q-M2.14-what-runs-the-model.md) | what runs it | *"llama-cpp with rust wrapper, not ollama"* | in EinAf's process, through a binding to llama.cpp |
| [Q-M2.1](../open_questions/kernel/Q-M2.1-when-is-the-kernel-frozen.md) | when the kernel is frozen | *"use current state - after M1f closed"* | the instrument is the kernel M1f left |
| [Q8](../open_questions/formalizer/Q8-ambiguous-parses.md) | ambiguous parses | *"there is no `hrule` anymore, rules emitting open facts replaced them"* | the form is a hypothesis, `(open F)` |
| [Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md) | what prints the answer | *"don't run M1p now, use as much of existing ein NL features … use additional external formatters"* | `:goal-text`, with EinAf's formatters over values; M1p does not run |
| [Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md) | who calls the kernel | both: the tool, *"recommened but optional"*, and the filter, *"forced to write ein code"* | two arrangements, both arms, compared with a chain of thought |

**Ruled after the re-plan, the same day** (the ledger has each): the kernel may gain any feature or bug fix that
leaves Ein's syntax and semantics as they are
([Q-M2.16](../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)), so the bound on a saturation is
the kernel's; the answer is a sentence, and EinAf's fixed texts stand
([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)); the minimal goal is the Zebra puzzle solved by
the model with Ein ([Q-M2.17](../open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)); and EinAf is one
tree, its documentation apart ([Q-M2.8](../open_questions/einaf/Q-M2.8-where-does-einaf-live.md)).

**Open, and holding no phase:** [Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md),
what *certified unique* means on R (the user, after experiments: every record carries each reading), and
[Q-M2.15](../open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md), where the milestone stops
for the next iteration of Ein (the user, at a level's gate).

**The frame of the re-plan is the user's two main ideas**, kept verbatim in
[the README](../README.md#the-two-main-ideas-the-user-2026-11-25): a chain of formal reasoning in place of a chain
of thought, with Ein as a tool and as a filter and the answer Ein's own; and the milestone as an evaluation of
ein.rs, for the iteration of Ein that follows ([`evaluation/`](../evaluation/README.md), seeded from this
analysis).

**The re-plan is done** (2026-11-25, on the user's word, *"Do re-plan if ready and no open questions"*): the
milestone's README states the plan as re-planned, with the defaults it took; every phase README and stage file is
rewritten to it, this page and the audits having been the worklist. **No stage has run, and no phase starts until
the user says so** (*"Do not start any phase"*).
