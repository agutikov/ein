# S1f.21.14 — the pages of the second part: `open`, what a state owes, the hypotheses

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19, cut out of [S1f.21.2](s1f.21.2_the_pages.md) on the user's word (*"leave open facts
and hypotheses to the second part of the phase and finishe everything else before"*). **Done 2026-11-20**
([§ Record](#record)); finished after S1f.21.20's merge, the same day ([§ After S1f.21.20](#after-s1f2120)).
**Group:** [B](modifications.md#b-the-pages-todays-kernel-stated-as-rules), items 8, 11 and 13, and item 10's half
on the obligations.
**Waits on:** the builds of the second part, [S1f.21.7](s1f.21.7_the_debts_term.md),
[S1f.21.12](s1f.21.12_hypotheses_as_facts.md) and [S1f.21.13](s1f.21.13_the_search_and_rules.md), and the rulings
they run on, [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)
and [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules).
It is the last stage before the close. **Since planned**: S1f.21.7 is not run, and the builds it follows are
[S1f.21.12](s1f.21.12_hypotheses_as_facts.md), planned again on the rulings Q-M1f.566 to Q-M1f.573, and
[S1f.21.18](s1f.21.18_the_hrule_form_removed.md); what it writes is those rulings as decisions and as pages.
**Writes:** pages of `docs/ein/reasoning/02-kb-evolution/` and `docs/ein/user/03-syntax-and-semantics/`, their
diagrams, the glossary, and the decisions of the second part's rulings.

## What it does

| item | page | what |
|---|---|---|
| 8 | `10-satisfiability.md` § 1 | *owed* and *discharged* as rules; **a conclusion may be a held fact exactly when it stays true in every larger state**, with the four words of [the overview § 1](overview.md#1-the-phase-in-one-sentence) |
| 10, its second half | the user's `08-obligations.md` | the requirement, the payment and `discharged` for one obligation, a program with its answer |
| 11 | wherever the pages define `(open R)` | the first instruction's remark, that read as facts the verdicts would duplicate and could be removed, worded by the ruling of Q-M1f.390 |
| 13 | the reasoning pages' definition of $R$ | $R$ with no search: every state a context, the oracle its brute-force reading |

And, with them:

- **What S1f.21.12 built, on the pages that say what a hypothesis is**: `04-hypothesis-generation.md`,
  `05-hypothesis-space.md`, the user's `09-hypotheses.md`, and the knowledge-base page's *a fact, its negation, both,
  or neither*, which gains the third context.
- **Each ruling of the second part as a decision**: Q-M1f.390, Q-M1f.549, and the readings its stages took; Q-M1d.2,
  an obligation, rewritten in place where the part amends it.

## What must hold

As [S1f.21.2](s1f.21.2_the_pages.md#what-must-hold): each perspective in its languages, a diagram a file, every
command a page shows run, and the four checks.

## Acceptance

Each item is on its page; `docs/ein/` says nothing of `open`, a debt or a hypothesis that the engine no longer
does; the checks.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `8c5d1bb0`, on one release build of that tree
(`./build.sh --engine`); every command a changed page shows was run on it. No engine file, library module, corpus
program or claim changed. The audit of the documentation was split by perspective among six subagents, each with one
brief (the subjects below, the rules of a page, the binary) and a set of files of its own; this agent wrote the items
of the plan, the decisions of the part and the glossary, read every diff the subagents made, and ran the checks.

During the run the coordinator relayed the user's rulings of Q-M1f.655 and Q-M1f.656 (the pass that writes `open`
facts runs at root, where it makes H, and above root only at a state printed, saved or whose rules read them; what a
state owes is read at every state; a flag runs the pass at every state, for the check in flight). The pages are
written to them, **without the two ids and without the flag's name**: S1f.21.20 builds both and writes the decision
files, which `master` did not hold when this record was first written. They are cited, and the flag named, since
S1f.21.20's merge ([§ After S1f.21.20](#after-s1f2120)).

### The commits

| commit | what |
|---|---|
| `643ed575` | the read-out examples page's X20 says the obligations write at root what would pay a debt, where it said *the obligations rung*; its generator's prose, the page written by it |
| `610a5d11` | the pages, the decisions, the renames and every reference they move |
| this one | the record |

### The four items of the plan

- **Item 8, *owed* and *discharged* as rules** — [`10-satisfiability.md` § 1](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#1-two-bounds-each-a-sentence-about-one-state):
  *§ 1.1 Which conclusion a fork may inherit*, a conclusion may be a fact a fork inherits exactly when it stays true
  in every larger state, with the table of the words: $\mathsf{inconsistent}$, $\operatorname{open}(f)$,
  $\operatorname{open}(r)$, *still open*, *alive*, $\operatorname{due}$, $\operatorname{paid}$, $\operatorname{disch}$;
  *§ 1.2 Owed and discharged, as rules*, the obligation as the rule it is, why *discharged* over `open` would flag a
  program, the requirement, the payment and $\operatorname{disch}$ as three ordinary rules, and
  $\operatorname{disch} \in S \iff \operatorname{discharged}(S)$; *§ 1.3 The verdicts are the last strata*, drawn
  (`diagrams/10-verdicts-as-strata.dot`). The last paragraph of § 1, *the asymmetry is why the debt is not a fact*,
  says why it is held otherwise than $\bot$.
- **Item 10, its second half** — [`08-obligations.md` § The requirement and the payment, as facts](../../../docs/ein/user/03-syntax-and-semantics/08-obligations.md#the-requirement-and-the-payment-as-facts):
  the page's program with three rules more for its own obligation, `seat-due`, `seat-paid` and `nothing-unpaid`, in
  `std.debts`' words; `ein saturate --dump` at root (two requirements, one payment, `(open seat)` and its two
  hypotheses) and `ein solve -p` (both payments, `(debts-discharged)`, no `open` fact), the read-out otherwise the
  program's and still monotone (run, `diff`). Then the debt as an ordinary rule, `unseated`: flagged, its
  `stale` row naming `(unseated Bob)` in the state that seats Bob (run); and *alive*, the same shape.
- **Item 11, the remark on `(open R)`** — worded by Q-M1f.390's ruling, no justification in the fact: it duplicates,
  so two instances over one relation are one fact and the count of instances is kept beside it, where each was written
  being the engine's record; it can be removed, so it is written at the state's fixpoint and no fork inherits it. On
  [`10-satisfiability.md` § 1.1](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#11-which-conclusion-a-fork-may-inherit)
  as a formula, on [`04-hypothesis-generation.md` § 4.1](../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#41-an-instance-and-the-facts-that-would-pay-it),
  and on the user's obligations page, *When an obligation is read*.
- **Item 13, R with no search** — [`06-solution-semantics.md` § 2.6](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#26-r-with-no-search),
  S1f.21.13's statement (§ 2 of its record): every commitment a world, $\operatorname{in}(w_C, f)$, every state a
  context of one knowledge base, $\mathcal{U}$, $\rho^{\mathrm{in}}$, $P^{\mathrm{in}}$, the oracle its brute-force
  reading, the strata of the verdicts and of a fact over the whole of $R$; two candidates drawn as four worlds
  (`diagrams/06-r-with-no-search.dot`). [`07-finding-r.md` § 2](../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md#2-the-oracle-the-cube)
  points at it. The letters are in the notation's second table.

**With them.** The knowledge base's *A fact, its negation, both, or neither* gains *A third context: `open`*, its
shape, that $\operatorname{open}(f)$ holds nothing of $f$, and where the status and the relation meet; *Two contexts
are read*; the reserved atoms name `open`; $W_{\!c}$ is said to hold no `absent` (Q-M1f.571). The notation's first
table has $\operatorname{open}(f)$ and $\operatorname{open}(r)$, and $\operatorname{st}(C)$ is said to be a fork that
drops root's `open` facts. **`open` as a core notion of the user's pages**, four ways (Q-M1f.371):
[`09-hypotheses.md` § `open`](../../../docs/ein/user/03-syntax-and-semantics/09-hypotheses.md#open-what-a-state-supposes-and-what-it-owes),
in Ein (a table of where it is written and what it is, `ein saturate --dump` of root), as a graph (root of the
seating program, the debt at the relation, the two hypotheses at facts in the frame *open ctx*,
`diagrams/09-open-facts.dot`, drawn by hand since `ein render kb` draws no `open` fact, V-72), in words, in
mathematics. **The glossary**: *`open`*, *Writers of H, the blind enumerator* new; *Held, global context, context*,
*Universe, H*, *Model*, *Monotone*, *Obligation* brought to the part.

### The audit

Every page of `docs/ein/` (the three perspectives), `stdlib/README.md`, `examples/README.md` and the READMEs under
it, `tests/README.md`, `corpus/README.md` and the top `README.md`, read whole by the agent of its set, against the
binary. What was stale, fixed in place (`610a5d11`), by subject:

- **What a state owes is a fact.** *Derives nothing*, *the verdict*, *never a fact*, *state of the node*, *read once
  on a quiescent state* where they said the debt is no fact: the rewriting-rules page (a conclusion of one of three
  kinds; *What a state owes, and what it supposes*, with the nesting table), firing (the fork drops the `open` facts,
  a convention for $\operatorname{sat}(S \cup X)$), saturation (*The pass*, $\operatorname{opens}(S)$, the fixpoint
  with it, a node resumes from root less its `open` facts; redrawn `02-two-phases`, `02-resume`), absence (*What a
  world owes and supposes*), cores (*what a state owes has no core*), the three rule-evolution pages; the user's
  relations, ontology, rules, theory, properties, solutions and models, the semantics index; the reference's syntax,
  reserved names (*`open` — what a state owes and supposes*, the heading renamed from *The verdict*, its four links
  moved), CLI, diagnostics (§ 2.2 rewritten, § 2.17 as built with the table of shapes and where each is refused, a new
  § 2.31 for the removed `hrule` form and `:hrules`), read-out, answer dump, region dump, rendering; the library's
  algebra, bijections, slots, the two rules pages' prose and debts (its blocks run again and pasted); the engine's
  architecture, algorithms (the node function and the obligation pass redrawn), data model (the `open` layer,
  redrawn), front end, saturation, search, parallelism, binary format, API, events, lattice dump, reserved strings,
  and the four reasoning-in-code pages (three `--dump` blocks run again and pasted, witnesses c5 and c13 to c15).
- **H is the union**: *the generator's list*, *the ladder*, *the rung*, *override* gone; the writers named, the blind
  enumerator only where root holds no `open` fact and no obligation is activated; *stuck* restated on hypothesis
  generation § 4.5 under the union (Q-M1f.648 below).
- **The lists and a closure** as the blind enumerator's configuration, and the refusal beside root's `open` facts:
  the user's query page, hypotheses page, the algebra page's `std.closure`, reserved engine strings, diagnostics.
- **`(solve :no-search)`**: H empty, and root still holds what its writers write under `open` (run), on the
  hypotheses page and in Q-M1f.254 and Q-M1e2.30.
- **Nesting and names**: facts, rules, reserved names, diagnostics, grammar's prose.
- **A rule that reads `open`, and the static check**: the sites `open-read`, `hypothesis`, `obligation-guard`,
  `obligation-trigger` on monotonicity (§ 2, § 3.1, § 3.2's theorem, § 6, § 8.4, § 9, § 10), absence, the region dump,
  events; the warning found in flight, where the pass writes such a state's `open` facts.
- **`-P`** leaves the `open` facts out: the model on solution semantics § 3 and satisfiability § 6, the glossary,
  Q-M1e.6 and Q-M1f.18.
- **No `hrule` form**: one sentence where each reference page says what is refused; the decision files whose names
  said `hrule` renamed.
- **`ein show hyps`, `ein render h-space`, the read-out and the summary**: the writer's note on the hypotheses page,
  the architecture and reasoning-in-code pages; no page rests on the writer `stuck`, which S1f.21.20 renames.
- **The status of a fact held neither way** is *unknown*, where it was *open*, on the knowledge base, the
  constraints page and Q-M1f.394, and the rewriting-rules and rule-evolution pages' *unknown* test (Q-M1f.646).
- **Beside the subjects, as a command's output**: the bijections page's `-p` block (15 facts, 19 printed, the four
  `std.contradiction` activators), run and pasted.

Found true already, page by page, in the subagents' reports: the guide's chapters 1, 3, 5, 7 and the walkthrough's
transcripts; `11-macros.md`, `14-natural-language.md`; the calculator and boolean pages (their programs run); the
stdlib index, macros, negation and core pages; `corpus/README.md`; the matcher, shared-assets, toolchain pages; the
design index; the read-out's `-p`/`-P` rows; diagnostics § 2.26's lines and § 2.28; most of 04 and 05 of the
reasoning, which S1f.21.12's build had written. The generated regions: `gen_stdlib_rule_pages.py --check` and
`check_generated.py` hold every one as written; the one stale generated text, X20's, went through its generator.

### The decisions

- **Written**: [Q-M1f.390](../../../docs/decisions/hypotheses/Q-M1f.390-open-r-carries-no-justification-and-the-instance.md),
  *`(open R)` carries no justification, and the instance is the engine's record*;
  [Q-M1f.549](../../../docs/decisions/hypotheses/Q-M1f.549-a-hypothesis-is-a-fact-open-f-whoever-writes.md), *a
  hypothesis is a fact `(open F)`, whoever writes it*, the user's main call and what the later rulings carry, and not
  the two uses the stage that built on it declined, which are a default (Q-M1f.651). Each in the folder's index.
- **Rewritten in place** where the part amends them: Q-M1d.2 (the debt a fact of the owing state, the payments
  written as hypotheses); Q-M1f.394 (a stored negation and a hypothesis are contexts; *unknown*); Q-M1f.566 and
  Q-M1f.574 (where the pass runs, as relayed, without the ids); Q-M1f.568 (refused at the parse or at load, as built);
  Q-M1f.575 (its fixture's path). And, by the decisions' subagent, twenty-one others the part made untrue: Q-M1d.3,
  Q-M1d.7, Q-M1e.6, Q-M1e2.3, Q-M1e2.7, Q-M1e2.30, Q-M1f.18, Q-M1f.43, Q-M1f.66, Q-M1f.67, Q-M1f.87, Q-M1f.88,
  Q-M1f.127, Q-M1f.249, Q-M1f.254, Q-M1f.309, Q-M1f.372, Q-M1f.522, Q-M1f.552, Q-M1f.590, Q-M1f.593. Checked against
  the ledger and found as ruled: Q-M1f.567, .569, .570, .571, .572, .573, .576, .577, .578, .579.
- **V-55, fixed as a class** (Q-M1f.645): twelve files whose name said what is no longer true renamed to their
  headings, ids kept, every reference moved in `610a5d11`: Q-M1f.129, Q-M1e2.10, Q-M1f.242, Q-M1f.243, Q-M1f.308,
  Q-M1f.32, Q-M1f.115, Q-M1f.116, Q-M1f.271, Q-M1f.66, Q-M1f.86, Q-M1f.87. **The references in the plans moved with
  them, the ledger among them**: 14 lines of `open_questions.md`, each a link target and nothing else, and one line
  each of `absent_open_not.md`, `design.md` and `one_way_each.md`.
- **Not written**: Q-M1f.655 and Q-M1f.656, S1f.21.20's; no default of this stage.

### Defaults taken (Q-M1f.645 to Q-M1f.654)

| id | default | the other reading |
|---|---|---|
| Q-M1f.645 | V-55 read as its class: every decision file whose name the part made untrue is renamed, twelve, and every reference moves with it, the ledger's link targets among them | Q-M1f.66's file alone; or no rename that touches the ledger |
| Q-M1f.646 | the status of a fact neither held nor negated is *unknown*, as the library's `unknown` tests it, on every page and in Q-M1f.394, so that *open* names the facts of `open` | *open* kept for the status on the reasoning pages, the user's page saying *unknown* |
| Q-M1f.647 | the remark on `(open R)` (item 11) on satisfiability § 1.1, hypothesis generation § 4.1 and the obligations page, worded under Q-M1f.390: one fact however many instances, the count beside it, the instance the engine's record; held by the state alone, no rule removing it | the remark on one page; or worded as a property of a verdict that is no fact |
| Q-M1f.648 | *stuck* on hypothesis generation § 4.5 is what the engine reads since S1f.21.20 (its Q-M1f.637): root owes and $H$ is empty, whoever the writers; beside it, as a proposition, an owed instance whose payments root all settles empties $R$ whatever another writer proposes. The first wording of this stage, the second alone, was replaced after the merge (V-67) | $H_{\mathrm{ob}} = \emptyset$ kept, which the union makes false: `18_two_candidates.ein` has it and a non-empty $R$ |
| Q-M1f.649 | $R$ with no search is solution semantics § 2.6, with a pointer from finding R § 2; its example drawn as four worlds, its program named on no reasoning page, since no user page spells it | a page of its own, renumbering the folder |
| Q-M1f.650 | `open` as a core notion is a section of the user's hypotheses page, its graph drawn by hand from `ein saturate --dump`'s facts, since `ein render kb` draws none (V-72) | a page of its own under `03-syntax-and-semantics/`, renumbering the folder |
| Q-M1f.651 | Q-M1f.549's file states the user's main call and what Q-M1f.566, .570, .573 and .574 carry; the two uses, search control from inside Ein and facts inferred from the search, declined by S1f.21.13 under the recommendation, are a default and no decision | the declines written into the decision |
| Q-M1f.652 | item 10's program restates the page's own obligation by three rules of the program, in `std.debts`' words, rather than importing `std.debts` beside a library obligation, which its page already shows | the library's `total-owed` with `(import std.debts)` |
| Q-M1f.653 | Q-M1f.566 and Q-M1f.574 rewritten to the relayed rulings on where the pass runs, without the ids, as the pages are | left to S1f.21.20, whose build amends them |
| Q-M1f.654 | the reserved-names heading *The verdict — `open`* is *`open` — what a state owes and supposes*, its links moved | the heading kept, since `open` is more than a verdict |

### Findings outside the stage (V-72 to V-77)

- **V-72 — `ein render kb` and `ein render constraints` draw no `open` fact root holds**, on a program or a saved
  root's file (a saved node's file does), while `ein saturate --dump` lists them and Q-M1f.572 says every drawing
  draws them: `ein render kb examples/lattice/05_owed_seats_bounded.ein` has no node `open`, where root holds five
  facts of it. `ein-cli/src/render.rs`' `loaded_and_saturated` saturates and writes no `open` facts.
- **V-73 — an `(open …)` the program writes is not judged against the lists and a closure.** `(open (q b))` beside
  `:hypothesis-relations (p)`, `:no-hypothesis (q)` or `(__closed__ q)` loads, and so does `(open q)` beside
  `:no-hypothesis (q)`; a rule's are refused (Q-M1f.578). The diagnostics page states the decision, and § 2.26's
  *written at FILE:L:C* line is reached by no program.
- **V-74 — a query's printed claim over `open` does not read back.** A goal `(open ?f)` or `(open (p ?x))` loads and
  is answered, and the read-out's `as :expect` row prints `(model (open (q a)))`, which the parser refuses as a claim.
- **V-75 — comments of the engine, its tests and the library in the words the part retired**:
  `ein-infer/src/obligations.rs` (*never stored*, *state of the node*, *verdict atom*), `scope.rs` (*generation
  ladder*, *rung*), `hypgen.rs`' `NO_SEARCH` (*no rung runs*), `oblgen.rs` (*the rung proposes the union*),
  `structure.rs` (*the rung of the ladder*), `ein-cli/src/show.rs` (*which rung built H*), `ein-ir/src/grammar.rs`'
  `notes()` (*where the verdict is written*), a test comment of `from_ir.rs` (`(not (open ?R))` refused at the parse,
  which loads), `ein-ir/tests/grammar_pages.rs` (*`open` outside a rule's whole conclusion*),
  `ein-infer/tests/obligation_rung.rs`' module doc (the ladder), `stdlib/bool/inputs.ein` (*obligations rung*),
  `stdlib/closure.ein`'s header (*the hypothesis generator*). Not changed: engine and library files.
- **V-76 — sentences of the first part's subjects still stale**, beside this stage's: the guide's
  `01-objects-and-relations.md` (a form of another count refused; Q-M1f.447 loads it); `02-relations.md` (`not` and
  `false` the engine's; Q-M1f.557, .593); `03-facts.md` (*where a value goes* for a nested `eq`, a parse error
  *unexpected `eq`*); the glossary's *Label, signature* (the loader holds the count); `examples/broken/load/README.md`
  (a relation declared twice among the refusals, no fixture); `examples/word_problems/README.md` (a redundant
  `:no-hypothesis (has)`, a `note:` on every command); Q-M1f.381 (its test does not exist, and a declaration fixes the
  count); `05-events.md`'s drawing `05-one-run-line-by-line.puml` (no `std.core.inconsistent` lines, every `n`
  shifted); `03-data-model.md` § 5 (`Kb` lists no `ended`). **Fixed** after S1f.21.20's merge, every one
  ([§ After S1f.21.20](#after-s1f2120)).
- **V-77 — the hypothesis found in flight may depend on the count of workers.** A worker sets
  `open::Writers::first_above`, a `OnceLock`, in `open::write`, before the interning that may hand the node back, so
  under `--jobs N` the hypothesis `monotonicity.in_flight` and the read-out name could be another node's than at one
  job; `01-determinism-and-order.md` § 6 says workers share nothing they write. Read off the code, not run.

### The checks

On `610a5d11`: `python3 utils/doc_audit.py --links --check --tracked`, 2443 files, 0 findings in links and in decision
ids; `python3 utils/check_release_voice.py`, 1871 files, none speaks of the development; `python3
utils/check_doc_blocks.py`, 546 pages, rules 1 to 5 hold; `python3 utils/render_diagrams.py --check`, 818 diagrams,
every picture drawn from its source; `python3 utils/check_generated.py`, 9 generators, every file as written. The
crate tests that hold the reference pages, run by the subagents on the release profile: `grammar_pages` 6,
`read_out_reference` 6, `config_reference` 9, `answer_dump` 8, `region_dump` 6, `help_surface` 4, `guide_programs`,
`guide_transcripts`, `natural_language_page`, all passed.

### The gate

On the branch rebased onto `master` at `b578f261` (the engine as at `8c5d1bb0`, rebuilt; the commands of this
stage's pages run again on it, the generators run again, nothing moved), `./run_tests.sh` twice on `15e0334d`, this
record's commit before its gate paragraph:

| run | result |
|---|---|
| default workers | exit 101 at `corpus_cli`'s `the_slow_flag_still_describes_the_sweep` alone: `examples/features/04_open.ein` 4069 ms and `examples/beyond_arith/equation_two_unknowns.ein` 7254 ms of cells with no `slow` flag, `examples/zebra.ein` 2923 ms against its 500, under load (V-60, V-64, V-65); no file this stage touches |
| `EIN_TEST_JOBS=4` | **exit 0**: the ten static checks, `cargo test --profile gate --workspace` (168 test binaries, 1380 tests passed, none failed, by the sum of their `test result` lines), the process's checks (`check_generated.py` 9 generators as written; `check_region_routes.py` 8 programs and 300 regions; `wrong_answers.py` 9 open rows; the review bundle's smoke, 0 failed) and the bench smoke |

Not run, as the brief says: the nightly, `--dev-host`, `utils/ci_container.sh`. The stage changes no DOT writer, no
review bundle and nothing CI installs.

### After S1f.21.20

On the coordinator's word once S1f.21.20 was in `master` (`0a6e6e3b`): the branch rebased onto it, the engine rebuilt
(`--in-flight` on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`, run), and five things.

**The rebase.** Nine files met S1f.21.20's edits: `AGENTS.md`, Q-M1f.574, `02-algorithms.md`, `06-saturation.md`,
`07-search.md`, `05-events.md`, the reasoning-in-code page on hypotheses, `08-read-out.md` and `examples/README.md`.
Where both said where the pass runs, S1f.21.20's sentence was kept, with its ids and the flag; what this stage added
beside it was kept too (the saturator never fires an `(open F)`; a rule that reads `open` flags its program; *stuck*'s
row, whose scoping clause was stale); the catalogue's `features/` row holds both `23_a_rule_reads_open.ein` and
`24_written_hypotheses.ein`. The renames' link targets in the ledger applied beside the coordinator's entries. Every
command this stage's own pages show was run again on the rebuilt binary and matches.

| commit | what |
|---|---|
| `16248b32` | 1. Q-M1f.655 and Q-M1f.656 cited, and `--in-flight` named, where a page says where the pass runs or where the check in flight is made: the user's obligations, hypotheses, solutions and models pages, syntax, reserved names, the region dump, the glossary, the engine's algorithms, data model, saturation, events and reasoning-in-code pages; the reasoning pages (saturation, absence, monotonicity, finding R) cite the two with no flag; Q-M1f.566 cites Q-M1f.655. With it, V-76's glossary and data-model places, in the same files |
| `8ce9aec4` | 2. V-76's other places: the guide (a fact of another count loads), the relations page (`not` and `false` the library's), the facts page (a nested `eq` refused at its word), the two READMEs, Q-M1f.381 (its test is `a_form_of_another_count_loads_on_every_command`), the one run's event lines drawn again with `std.core.inconsistent`'s `compile`, `enqueue` and `fire` (run: 22 lines) |
| `e62c407a` | 3. V-67: *stuck* is a root that owes with H empty, as the engine reads it (`18_two_candidates.ein` is `union`, a probe with every payment denied `stuck`, both run), and an owed instance whose payments root all settles empties R beside it. 4. V-68: the stream's state machine has root's saturation resumed after its obligation pass, and a node's after its `owe` lines, where a rule reads `open` (`23_a_rule_reads_open.ein`'s stream, run) |

**5.** Nothing of S1f.21.21's subjects (the matcher's indexes) or S1f.21.22's (the kb and constraints renders, the
lists' check of a written `(open …)`, the `outside` and `as :expect` rows) was touched; V-72, V-73 and V-74 are
theirs to move.

The checks on `e62c407a`: `doc_audit.py --links --check --tracked`, 2450 files, 0 findings in links and in decision
ids; `check_release_voice.py`, 1876 files, none; `check_doc_blocks.py`, 550 pages, rules 1 to 5 hold;
`render_diagrams.py --check`, 818 diagrams, every picture drawn from its source; `check_generated.py`, 9
generators, every file as written; `read_out_examples.py` run again, nothing moved.

**The gate** on `b0fbcccb`, this section's commit: `EIN_TEST_JOBS=4 ./run_tests.sh`, **exit 0**, run with four
workers from the start, two engine stages building beside it and the run before this round having stopped only at
the slow flag's check under load: the ten static checks, `cargo test --profile gate --workspace` (168 test binaries,
1384 tests passed, none failed, by the sum of their `test result` lines), the process's checks (`check_generated.py`
9 generators as written; `check_region_routes.py` 8 programs and 300 regions; `wrong_answers.py` 9 open rows; the
review bundle's smoke, 0 failed) and the bench smoke. Not run: the nightly, `--dev-host`, `utils/ci_container.sh`.
