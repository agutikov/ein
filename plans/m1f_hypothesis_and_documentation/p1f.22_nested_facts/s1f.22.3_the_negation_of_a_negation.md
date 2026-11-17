# S1f.22.3 — the negation of a negation is a fact

**Phase:** [P1f.22](README.md)
**Status:** **done 2026-11-17**: `b1a9a635`; one finding outside its scope, [N-22](findings.md).
**Ruling:** [Q-M1f.391](../open_questions.md#q-m1f391--the-negation-of-a-negation-that-a-rule-builds), (c):
`(not (not g))` is a fact a file may write and a rule may match; the kernel is not changed; the rule that
concludes `g` from it is a theory's, and where that rule lives is P1f.21's.

## What it builds

1. **The grammar** (`ein-ir/src/ein.y`): a negation under a negation, at any depth, as a fact, as a fact under an
   arrow, in a pattern and in a conclusion. The grammar page's two generated regions and its diagrams follow
   (`EIN_BLESS=1`), and the row of its table that refuses the shape goes.
2. **The loader and the value rule**: an object is an atom, a fact, or the negation of a fact, a negation among
   them. What stays refused: `not` of an atom, and a bare variable under a nested `not`
   ([`contexts.md` § 6](contexts.md#6-the-negation-of-a-negation-and-its-elimination-as-a-rule)).
3. **Nothing in the kernel.** A fact held with its negation is dead, for every fact; so `(not g)` beside
   `(not (not g))` is dead and `g` beside `(not (not g))` is alive, as the engine does today. Every reader of a
   rule's negative literal (the strata, the monotone check, *forbid*, the mechanisms' check, the invariant) is run
   on a premise and a conclusion that nest a negation, and what each reads is stated in the stage's record.
4. **The witnesses, as corpus programs** under `examples/beliefs/`, each with its `:expect`: the liar's rule over
   a negative statement (`e11`, `f1`, `f2` of the analysis), and one that writes the elimination itself, one rule
   for a count of arguments with the relation a parameter, to show that a theory can.
5. **The pages, in the same commits**: the knowledge-base page's shape of $\mathsf{not}$ and *"$\lnot\lnot g$ is
   no fact"*; the rewriting-rules page's $\lnot t$; the user's objects page, the syntax, the diagnostics; the
   decision Q-M1f.391, and Q-M1f.113's sentence re-worded where it says otherwise.
6. **The explanation's witnesses, as corpus programs** under `examples/beliefs/`, each with its `:expect`, for the
   claims S1f.22.2 wrote with no test of their own ([its record](s1f.22.2_the_explanation_written.md#record)): the
   three cases with their two questions; a global rule beside its wrapped twin; the context as a parameter, with
   `(false)` in a context and *everybody* by `forall`; a declaration, an activator and `(false)` under an arrow
   doing nothing. `08-reasoning-in-code/01-static-ir.md` names each beside the claim it holds. The comments of the
   folder's programs and their corpus notes take the ruled words (N-17).

## Acceptance

`./run_tests.sh` exit 0 on the stage's last commit; the parser's fuzz step of the nightly, both seeds, since the
grammar changed. No R of a corpus program changes: `ein-cli/tests/regressions.rs` and the corpus sweep hold it.
What it finds outside its scope is in [`findings.md`](findings.md).

## Record

### Record (2026-11-17)

**Commit**, on master: `b1a9a635`, the engine, its test, the pages, the decisions and the corpus programs together,
since each page is true only with the engine's change.

**The grammar** (`ein-ir/src/ein.y`). Two productions are new, each taking itself under `not`: `NegFact`, where a
fact is written (`NotFact`, `FactArg`, `ClaimFact`, `ClaimAtom`, a trace's `Derived`), and `NegPattern`, where a
pattern is (`Term` and `Negated`, so a premise, a pattern's argument, a conclusion, a goal). No other production
changed, and the build reports no conflict. `Template` already took `not` under `not`; what a macro's expansion
writes is held by the loader, `Shapes::negation`, which now reads a `not` under a `not` as a negation written as a
value and so refuses a variable or an equality there.

**The loader's `written` step** (`ein_ir::from_ir`). Two readings met the new shape and read it as an ordinary
fact, stating `(written g)` for `(not (not g))`. Both now read through every negation, as the knowledge-base
page's formula does, $\operatorname{sub}(\lnot f) = \operatorname{sub}(f)$ applied at each: `denied`, for a stored
fact, and the conclusion's arm of `written_in_conclusions`. Nothing a program without a double negation loads
moves.

**Nothing in the kernel.** No source file of `ein-core` or `ein-infer` changed but a comment in `firing.rs`; of
their tests, `alive_invariant.rs`' banked list gained the new programs' names.

**What each reader does with a negation under a negation**, each a run on the gate build of this change, the
programs in the session's scratch directory; where a single negation is the comparison, both were run:

| reader | on a premise and on a conclusion that nest a negation |
|---|---|
| the parser and the loader | load: a fact, an argument at any depth, a premise, a pattern's argument, a conclusion, a conclusion's argument, a goal, a claim, a claim's `(absent …)`, through a macro; `(not (not (false)))` too |
| the declared count | held under two negations: *the fact (not (not (p a))) holds (p a), which gives p 1 argument…* |
| a relation's parameter bound to a fact, written or derived, or to `absent` | refused with the lines a single negation gets |
| the imports | a short name under two negations is its full name, under `:symbols` and under `:as` |
| the matcher and the firing | `(not (not (?R ?a ?b)))` matches a held double negation and binds; a conclusion stores one; the three rules *eliminate*, *introduce* and the wrapped *eliminate* answer as written |
| the contradiction | `(not g)` beside `(not (not g))` dead, core the two; `g` beside it alive; alone alive |
| the agenda, the strata | a rule that concludes `(not (not q))` is admitted where one that concludes `(not q)` is: the two programs that differ in that answer alike |
| the monotone check | an `(absent (not (not (q ?x))))` over what a commitment can add is flagged *not monotone: 1 site*, with the stale facts, as the single negation is; `ein solve` and `ein oracle` print the same rows |
| *forbid* | a conclusion `(not (not (q ?x)))` from an `(absent …)` over a relation a commitment can extend is refused with the line a `(not (q ?x))` gets |
| the mechanisms' check | an hrule that proposes `(not (not (p ?x)))` beside `(solve :no-hypothesis (p))`: the note a `(not (p ?x))` gets, and no refusal |
| the hypothesis generators | an hrule proposes `(not (not (p a)))` as a candidate; beside `(not (p a))` the commitment is dead, 1 state and 1 minimal dead set, `ein solve` and `ein oracle` alike. The blind enumerator's objects are the atoms under the negations; a fact under two negations is a candidate where root neither holds nor denies it |
| `ein show hyps`, `ein render kb`, `render program`, `render rule` | exit 0, and every DOT lays out |
| `ein kb save`, and a command on the container | the container holds what the text holds, and `ein solve` prints the same |
| `--dump-answer` and the reload | the state file of a state that holds one loads, 1 state: before the change it did not parse |
| `ein test`, a claim | `(model (not (not (p a))))` and `(absent (not (not (p b))))` are held and refuted as a single negation's are |

**The invariance.** `ein-cli/tests/negation_of_a_negation.rs`, seven tests: every place a file writes one, on eight
commands, and on `ein test` where the shape states a claim; a rule that matches and concludes one; **built by a rule's `(not ?x)` or written in the file, the answer
is the same** on `ein solve`, `ein oracle` and `ein test`, alone, beside the denial and beside the fact; the liar's
rule over a variable and spelt by shape reach one region under a search, four programs, both commands; nine
refusals; what the loader states; the file of a state that holds one.

**The corpus programs**, under `examples/beliefs/`, each with its entry and its line of the catalogue. `06`
declares `=`: as first written, with `=` only under the negations and in the activator, the rule's conclusion was a
fact of a relation the registry lacks ([N-22](findings.md)):

| program | claims |
|---|---|
| `03_liar_denial.ein` | the liar's rule builds `(not (not (= 5 (+ 2 2))))`; no fact of `=` and no stored negation of one |
| `04_liar_denial_contradicted.ein` | the same beside `(not (= 5 (+ 2 2)))`: dead |
| `05_liar_denial_beside_the_fact.ein` | the same beside `(= 5 (+ 2 2))`: alive, both held, the claim writing the double negation in the file |
| `06_double_negation.ein` | the elimination as a rule, one rule a count with the relation a parameter, on for `=` and not for `likes`; the same rule wrapped in `think` |
| `07_three_cases.ein` | a fact held, in a context, or both; the two questions; a rule that reads the held ones; the sums in no answer |
| `08_global_and_wrapped.ein` | a global rule beside the same rule under a macro's wrapping, neither reading the other's facts |
| `09_false_in_a_context.ein` | the context and the fact's relation parameters; `(false)` concluded in two contexts, root alive; a rule reads them; the `forall` over them does not fire |
| `10_everybody_contradicts.ein` | the same with every person contradicting: the `forall` concludes root's `(false)`, dead |
| `11_nothing_acts_in_a_context.ein` | a declaration, an activator, `(false)` and a denial under an arrow do nothing; the activator held does |

The comments of `01_common_belief.ein` and `02_packing.ein`, their notes and the catalogue's row take the ruled
words (N-17). The shape golden gained the nine programs' lines, 351, and no line of it moved.

**The pages and the decisions.** The knowledge-base page: the shape of $\mathsf{not}$ without its last clause,
$\lnot\lnot g$ a fact, the status and the contradiction for any fact with the three implications of a double
negation, $\operatorname{sub}$ through each negation, and *denying twice does not lead back*. The rules page: what
$\lnot t$ may deny, the literal $\lnot x$ over a negation, the elimination as a row of what a rule can spell. The
user's pages: a section *The negation of a negation* on the constraints page, both of its programs run; the
facts, the rules, the syntax, the reserved names, the diagnostics (§ 2.16's row, § 2.22's rule, § 2.30's table);
the grammar page's generated regions and five diagrams, and a fifth note. The engine's page names each new program
and the new test beside its claim. Decisions: Q-M1f.391 written; Q-M1f.113 and Q-M1f.275 re-worded.

**The checks**, on `b1a9a635`:

| check | result |
|---|---|
| `./run_tests.sh` | **exit 0**: the ten static checks, `cargo test --profile gate --workspace` with 161 test binaries `ok` and none failed, the five checks of the process, the bench smoke. On AC power. Two runs before it stopped: one at `cargo fmt --all --check`, on the new test file, and one at `corpus_cli`, whose banked table of exits lacked the nine programs' 36 cells; both are in the commit |
| `EIN_FUZZ_ITERS=20000 EIN_FUZZ_SEED=1 cargo test … -p ein-ir --profile gate --test fuzz_properties` | exit 0, 8 passed |
| the same with `EIN_FUZZ_SEED=987654321` | exit 0, 8 passed |
| `EIN_BLESS=1 cargo test … --profile gate --workspace --no-fail-fast`, once, to re-bank for the new programs | wrote the 36 cells of `corpus_exits.txt` and nothing else; with `id_order_invariance`'s re-bank, 351 lines of `corpus_shapes.md5`, all the nine programs' |
| `ein-infer/tests/alive_invariant.rs` | failed in that run on two counts, both of the new programs: its banked list of the names the index does not hold wanted the nine programs' 26, added; and `06_double_negation.ein` breached *no new relations* ([N-22](findings.md)), so the program declares `=` |

**Not done, or left uncertain.**

- No rule of the elimination is in `stdlib/`: its home is P1f.21's.
- [N-22](findings.md) is not this change's and is not fixed: a rule with its relation a parameter can conclude a
  fact of a relation the engine does not count among the program's. It bears on Q-M1f.393 (a), and is the user's
  call.
- The message *a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation*
  is as it was: it reads true of a negation of a negation, and it is pinned by the tests of every refusal.
- That a marker under an arrow marks nothing has no test: `11_nothing_acts_in_a_context.ein` is root alone, and
  a marker is seen only in what a search proposes.
- `utils/ci_container.sh` was not run: no DOT writer and nothing CI installs changed. The nightly's other steps
  were not run.
- The agenda's own reading of *a fact whose negation can be concluded* was not read in the code for a negation
  under a negation: what is shown is that the two programs that differ in it answer alike.
