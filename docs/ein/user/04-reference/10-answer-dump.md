# The answer dump — `ein solve --dump-answer DIR`

**What a solve answered, as files.** One directory per run: the read-out uncut, the summary, one **saved file** per
state the run reached, the owing frontier of an empty R, the minimal dead sets, each form asked for uncut, and an
index naming each. It is the answer of the question the run asked: R, or the part of it the question names, as
[`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md) defines it, in the format
`ein-answer/3`. A saved file is the program as written and the choices made, and loads back to its state
([§ A state file is its program and the choices made](#a-state-file-is-its-program-and-the-choices-made)).

```sh
ein solve examples/lattice/02_genuine_3set_death.ein --dump-answer out/
ein solve examples/zebra2.ein --maximal --dump-answer out/
```

## The directory

```text
out/
  index.json      what is here — the question, the summary's region block, and every state file by name
  answer.txt      the read-out uncut: stdout with no listing cut, and every row of a block stdout stops at four
  answer.json     byte-identical to --json-summary on the same run
  models/NN.ein   one state saved: the program, its modules and the choices made, one per state the run reached
  owing/NN.ein    one owing node saved, the same form, on an empty R of the owing kind
  nogoods.json    N: each minimal dead set, its core, the core's status and the rules that refuted it
  forms/F.txt     each form asked for, uncut: -H, -p, -P, -f
```

- **`NN` is stdout's numbering.** `models/02.ein` is *state 2 of k* in the `(region …)` block, two or more digits, so
  the files list in that order. On a partial run it is every state the run reached, listed or not: a cut at 100 lines
  (`-n`, or the shared stdout limit) cuts stdout, not the directory, and `index.json` says which files stdout listed.
  A line mode past `--region-limit` on a flagged program lists the states describe certifies on its line and writes
  every other member it met unlisted, with no line
  ([Q-M1e2.19](../../../decisions/answer/Q-M1e2.19-past-the-limit-a-line-mode-certifies-what.md)).
- **`index.json`** carries `asked` and the same `region` block as the summary, per state file its `path`, `state`
  number, `lines` (`minimal` / `maximal`), `listed` and `commit`, the candidates its `(commit …)` names, and per owing
  file its `path` and `commit`, and `forms`. What the run says of a state is here and not in its file: its number and
  lines, and in `region.states`, the state's `N`-th entry, the commitment it was read from (`from`), how many
  commitments reach it (`commitments`) and its minimal generators. **A count of commitments, here and in
  `answer.json`, is reported up to $2^{63} - 1$**: `9223372036854775807` reads *that many or more*
  ([Q-M1f.218](../../../decisions/answer/Q-M1f.218-a-count-is-exact-and-reported-up-to-2-63-1.md)).
- **`answer.txt`** reads back: its `(region …)` is the claim a `(solve … :expect …)` states, `(merged …)`s included
  ([`12-query.md`](../03-syntax-and-semantics/12-query.md)). Its rows are whole too: every `owed` row under `∅`, each instance
  root owes, where stdout prints four and counts the rest
  ([`08-read-out.md` § 2](08-read-out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle)), as every `site` and `stale`
  row of the warning.
- **`owing/`** holds the frontier of an empty R whose nodes all owe, each with what it owes; where describe's route did
  not compute the frontier (the tree and the classes), the nodes it met, each owing with every extension dead, which
  `answer.txt` says.
- **`nogoods.json`** is `[]` where N is empty or was not computed by a line mode; where describe's route did not
  compute it (the tree and the classes), it holds the members the run met, the candidates dead alone, and the
  summary's `region.nogoods` is `null`. Each member is `{set, core, core_status, refuted_by}`:
  - `refuted_by` names the rules of the last step of the core's derivation: for each fact of the contradiction the
    core was taken over, the first recorded justification whose premises the core derives, `(given)` for a fact the
    core holds itself. Where the core derives a fact through two rules, the order the derivations were recorded in
    picks one, so two programs one rule's written place apart can name two rules beside one core
    ([Q-M1e2.21](../../../decisions/read-out/Q-M1e2.21-refuted-by-names-the-first-recorded-derivation.md)): a core and
    its rules are an explanation
    ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md));
  - `core_status` says whether the core's search was cut at its budget: `within-budget`, or `cut-` and the caps it
    stopped at (`cut-environments`, `cut-rounds`, `cut-facts`, joined);
  - **every member has its core, past `--region-limit` too**
    ([Q-M1f.26](../../../decisions/read-out/Q-M1f.26-every-member-of-n-carries-its-core.md)): a member the route did
    not visit is given, after the route, the core it would have taken where it visits the member, the one
    `--dump-lattice`'s `nodes.json` writes for a visited node. On a monotone program a route stops a dead node at its
    first contradiction ([`07-finding-r.md` § 1](../../reasoning/02-kb-evolution/07-finding-r.md#1-a-node)); a member's core is
    still taken over its fixpoint, the saturation going on from the stop, as `ein oracle --dump` takes it.
- **`forms/`** holds each form the run was asked for, uncut and drawn from every listed state, the states
  `answer.txt` lists: `hyp-stats.txt` (`-H`), `print-final-state.txt`, `print-final-positive.txt` and
  `print-final-hfacts.txt` (`-p`, `-P` and `-f`).
  Stdout prints what fits of each in what the read-out left of its 100 lines, drawn from the states it showed, and its
  last line names the file. A file is what the form prints with nothing cut: where nothing is cut it is stdout's form
  byte for byte. `index.json`'s **`forms`** lists each as `{form, path}`, in the order stdout prints them, and is `[]`
  where no form was asked for.

A state file costs no saturation: it is written from the program as it was read and the commitment, so the
directory's size is about $k$ times the program and its modules.

## A state file is its program and the choices made

A state is a function of its program and of the hypotheses committed, so a state file holds exactly those two and no
fact of the state: **the program unchanged, with the choices made**. Loaded, it derives the state again, every fact by
the rule that derived it, with the origin it had, and **it reads no other file**
([Q-M1f.392](../../../decisions/answer/Q-M1f.392-a-saved-state-is-saved-and-loaded-as-itself.md)). The state
`{(note (bad a))}` of [`tests/saved/02_a_choice_with_a_fact_inside_it.ein`](../../../../tests/saved/02_a_choice_with_a_fact_inside_it.ein),
`models/02.ein`, the program's comments cut here:

```lisp
;;; ein-saved/1 — the program and every module it imports, each as written, and the choices made: this file reads no other.
(program
(relation item Thing :why "{?3} is an item")
(relation bad Thing :why "{?3} is bad")
(relation note Fact :why "a note says {?3}")
(relation flagged Thing :why "{?3} is flagged")

(item a)

(hrule guess ()
  :match  (item ?x)
  :assert (note (bad ?x))
  :why    "suppose a note about {?x}")

(rule flag ()
  :match  (note (bad ?x))
  :assert (flagged ?x)
  :why    "a note says {?x} is bad, so {?x} is flagged")

(solve :expect (region (model) (model (note (bad a)))))

(query :goal (flagged ?x)
       :goal-text "{?x} is flagged"
       :expect (or (model) (model (flagged a))))

)
(commit
  (note (bad a)))
```

- **One fixed first line**, the same in every saved file: what it is, and that it reads no other file. It names no
  run, no path and no state number, which a second save would not reproduce; those are `index.json`'s.
- **`(program …)`** holds the program file's bytes as they were read, between a line break after `program` and one
  before `)`: its comments, its layout and its imports as written. A state of a `.einb` carries the program the
  container stores, resolved and import-flattened, and no module
  ([Q-M1f.27](../../../decisions/cli/Q-M1f.27-a-containers-answer-directory-saves-the-program.md)).
- **`(module NAME …)`**, one for each module the load read, each once, its bytes the same way, sorted by name: `std.x.y`,
  or a path from the program's directory, a dot a directory, a module's own imports among them. Each import of the
  program and of a module is read from these and **never from the disk**; a module of the standard library is the one
  the load read, from `$EIN_STDLIB`, the checkout or the binary's embedded copy. A load reads the file alone and
  compares nothing: a carried module is loaded as carried, whatever the standard library of the binary loading it
  holds, or whether it has one, and nothing is said
  ([Q-M1f.401](../../../decisions/read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md)).
- **`(commit F …)`**, the choices made: the candidates of H the state was read from, each as the read-out prints it,
  in H's order. A load commits them after root's fixpoint, as a node of the search commits its commitment, each an
  origin `:hypothesis`. Root's file has none. An owing node's file is the same form, and nothing in a file says what
  its node is: the load computes it
  ([Q-M1f.400](../../../decisions/read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md)).

Each text is read as the file it was, under its own name, `<program saved in F>` and `<module NAME saved in F>`, so a
position in it is the line and column of the program or the module. A `(program …)`, `(module …)` or `(commit …)`
stands only at the top of a saved file, and a file that opens with `(program` is one; what each refuses is
[`07-diagnostics.md` § 2.30](07-diagnostics.md#230-a-saved-file-is-its-program-its-modules-and-one-commitment-of-h).
The three are forms of the grammar
([`01-grammar.md` § 2.16](01-grammar.md#216-a-saved-file-the-program-its-modules-and-the-choices-made)), and their
heads are names everywhere else ([`03-reserved-names.md`](03-reserved-names.md)).

**Save and load, in both directions.** No command is added: *save* is `ein solve FILE --dump-answer DIR`, and *load*
is any command given a saved file. A saved file's root is the node it names, so `--no-search --dump-answer` saves it
again, and the two invariances are these commands:

```sh
ein solve P --dump-answer A                            # save: A/models/NN.ein
ein solve A/models/NN.ein --no-search --dump-answer B  # load it, and save its root
cmp A/models/NN.ein B/root.ein                         # save → load → save: byte for byte
ein saturate --dump A/models/NN.ein > a.txt
ein saturate --dump B/root.ein > b.txt                 # load → save → load: the KB sections are the same
```

`ein saturate --dump` of a saved file prints the node's state, root's fixpoint and then the choices committed and
saturated, each fact with its first origin, `:source`, `:rule` or `:hypothesis`, in the order held; on the state
above it is root's `(item a)`, then `(note (bad a) :hypothesis 0)` and `(flagged a :rule flag)`. A state's first load
is the state the run saved, fact for fact and origin for origin.

## A saved file is a node of its program's lattice

A saved file is its program with one commitment of the program's H, and every command reads it as that node
([Q-M1f.402](../../../decisions/answer/Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md)):

- **`ein solve F` describes the part of the program's R above the node**, in every mode, and `ein oracle F` that part
  by brute force: the run's universe is H less the commitment, ∅ of its lattice is the node, and every node of it is
  the program's node of the union. The read-out names the node in a `commit` row under `universe`
  ([`08-read-out.md` § 5.3](08-read-out.md#53-under-universe)). A state found above it is saved as the program with
  that state's commitment, the union, in one `(commit …)`: every saved file of a program is the program and one
  commitment.
- **`ein saturate F`, `ein render kb F`** read the node's state; `ein render program F` draws the program it carries.
- **`ein show hyps F` and `ein render h-space F`** read the program's H, the node aside, its choices among H.
- **The program's claims are not checked** on a file with a `(commit …)`: they are about the program's R, and the
  read-out names each NOT CHECKED, *the claim is the program's, about the program's R, and this saved file is a node
  of its lattice*, and exits 0. `ein test` asks it nothing, not root either: each claim is NOT CHECKED with that line
  and counted not checked with no exit code, and its `--json-report` row carries no run's data. Root's file loads to
  the program's root and checks them as the program does
  ([Q-M1f.403](../../../decisions/answer/Q-M1f.403-a-saved-file-with-a-commitment-checks-none.md)).
- **`ein kb save F X.einb`** stores the program the file carries, its imports read from the modules it carries, and
  the `(commit …)` of a node in a section of the container, so that **a node's container opens as that node**: every
  command reads `X.einb` as it reads `F`, and `ein solve X.einb` prints what `ein solve F` prints, byte for byte but
  the path it echoes. A root's file has no `(commit …)`, and its container is a program's
  ([Q-M1f.413](../../../decisions/engine/Q-M1f.413-a-nodes-container-opens-as-the-node.md);
  [`09-binary-format.md`](../../ein.rs/03-design/09-binary-format.md#23-the-sections)).

## The contract

- **Additive.** Stdout, stderr, the exit code, `--json-summary` and the `--events` stream are identical with the flag
  and without, so the read-out's header leaves the flag out, as it leaves out `-j`.
- **`answer.json` is `--json-summary`**, byte for byte, and **`answer.txt` is stdout's read-out uncut**, on every
  question. The forms printed after the read-out are not in it: each is its own file under `forms/`, and the read-out
  and the summary are the same with any form and without.
- **The directory is made before the run**, and a run that cannot make it exits 1 without solving
  ([`07-diagnostics.md`](07-diagnostics.md), an artefact flag's failure). The files are written once the answer exists, and
  a failure there keeps the run's exit code and writes `error: --dump-answer <path>: <os error>`
  ([Q-M1e.22](../../../decisions/cli/Q-M1e.22-a-failed-artefact-write-keeps-the-runs-exit-code.md)). An empty path
  is a usage error, exit 2.
- **One run, one directory.** A file with several `(query …)` blocks is one run, and each query's classes are in
  `answer.json`'s `queries`.
- **A `.einb` writes the directory its `.ein` writes**, but for its state files: each carries the program the
  container stores, resolved and import-flattened, as `ein render program` draws it
  ([Q-M1f.17](../../../decisions/cli/Q-M1f.17-a-view-of-a-container-draws-the-program-it.md)), and no module; on a
  program that imports, that program keeps every declaration its imports brought, and the commitments are the
  `.ein`'s ([Q-M1f.27](../../../decisions/cli/Q-M1f.27-a-containers-answer-directory-saves-the-program.md)). Every
  other file is the `.ein`'s, but for the path it names.
- **A budget stop is an answer**: `--region-budget` and `--region-limit` write the whole directory, and `index.json`'s
  `region` says `exhausted: false` and `stopped: {why, …}`. **A `-T` abort is not**: it exits 2, and the directory it
  made first stays empty.
- **A second dump into the same directory leaves nothing stale.** When `index.json` names an answer directory's
  format, this one or an earlier one, `models/`, `owing/` and `forms/` are cleared before writing. A directory holding anything else is written beside and nothing
  in it is deleted.
- **`--count` writes two files**: `answer.txt`, its read-out uncut, every warning row among it, and `answer.json`, its
  summary, whose `asked.ask` is `count` and whose `region.states` and `region.frontier` are `null`. It reads no state,
  so it writes no state file, no form, no `index.json` and no `nogoods.json`, as `--monotone` writes none (every form
  is refused beside `--count`, exit 2); and where `index.json` names this format it removes what that dump left, the
  two top-level files with the subtrees.
- **`--no-search` writes root**: `answer.txt`, `answer.json`, root's node saved as `root.ein`, the program with no
  `(commit …)` or a saved file's own node, which is that file again, and `forms/`, and no `index.json`,
  `nogoods.json` or state of R. What root is, owing or dead, is `answer.json`'s `root` block.

## The run summary

`answer.json` is `--json-summary`'s document, `ein-summary/2`: `schema`, `source`, `asked`, `region`, `verdict` (under
`--single` only), `queries` and `claim`, each field defined by
[`08-read-out.md` § 11](08-read-out.md#11---json-summary-the-answer-blocks) beside the row that prints it; `work`,
`monotonicity` ([`11-region-dump.md` § Monotonicity](11-region-dump.md#monotonicity--the-static-check)), `leftover`, `root`,
`config`. Every bound in `region` carries `exhausted` beside it, so a reader never has to know which numbers were
complete.

The `work` block is what a reader asks *what did the run cost* of. Each counter is defined here, and so is every field
of the `root` block below; the test suite fails when a counter or a field appears in a summary without a row here.

| counter | what it counts |
|---|---|
| `saturations` | node fixpoints computed, root's included: one per node route 1 visits, one per node of the cube under route 2, one per extension the state route saturates, plus one per listed state a mode re-reads. The read-out's `work` row is this number |
| `facts` | facts those saturations derived beyond root's own fixpoint, summed over nodes — what `--region-budget` is spent against. On a monotone program a dead node's are those to its first contradiction, where its saturation stops ([`07-finding-r.md` § 1](../../reasoning/02-kb-evolution/07-finding-r.md#1-a-node)); every other node's, and the whole node the tree's order reads, are its fixpoint's |
| `layers` | route 1's layers visited, root's layer 0 included. Under the state route, its layers: root's, then one per state size it extended. `null` under route 2, which visits the cube in mask order and has no layers |

The `root` block is **root's fixpoint and how H was built, as every route starts from them**: read off the run's own
root and not recomputed, so a summary adds no work and no option moves the block. `root.facts` is root's fixpoint and
`root.hypgen.emitted` is `region.universe.candidates` on every run of a program, and `--monotone`'s block is the region run's. It
holds root's standing too, `dead`, `owes` and `owed`, on every question: `ein solve --no-search` states no R, so its
`region` is `null` as `--monotone`'s is, and the block is where it says what root is
([Q-M1f.259](../../../decisions/read-out/Q-M1f.259-under-no-search-the-summary-states-no-region.md)); the three
are the full run's `region.root`, `root_owes` and `root_owed`.

| field | what it holds |
|---|---|
| `root.facts` | the facts of root's fixpoint, $\operatorname{sat}(\text{program})$: what `ein saturate FILE` prints as *facts (total)* after saturation, and what the file of state `∅` loads to when $\emptyset \in R$. No route writes into root, so it is root before any commitment and after every one |
| `root.facts_by_relation` | those facts by relation name, sorted, only the relations that have one. They sum to `root.facts` |
| `root.dead` | root's own fixpoint holds a contradiction, `(false)` or a fact beside its stored negation: root's standing, which every run reaches first and `ein solve --no-search` reports. It is `region.root` = `dead` on every description |
| `root.owes` | the obligation instances root's fixpoint leaves undischarged, dead or not, as a node counts them: `region.root_owes` on every description. With `root.dead`, whether $\emptyset$ is in R |
| `root.owed` | those instances, each through its rule's `:why` (the rule's name where it has none), in the order the rules report them: `region.root_owed`, and the read-out's `owed` rows |
| `root.commit` | the candidates of H a saved file's `(commit …)` names, each as the file writes it, in H's order: the node the run is the part of R above, and whose standing `root.dead`, `root.owes` and `root.owed` are. `[]` for a program and for a saved file of root. The read-out's `commit` row under `universe` |
| `root.plans` | the plans root's saturation compiled, the derived activators' included |
| `root.saturator.naf_rounds` | how many times root's saturation reached the NAF boundary with candidates parked on an `absent` guard, and judged them against the quiesced world |
| `root.saturator.naf_admitted` | parked candidates the boundary admitted: every guard passed, and the firing was queued |
| `root.saturator.naf_retired` | parked candidates the boundary retired as dead: an anti-monotone guard found a match, which a growing KB keeps |
| `root.saturator.naf_dropped` | always 0: a guard is judged once, at the boundary, so no admitted firing is dropped later. A boundary that judged wrong would show here |
| `root.hypgen.raw` | candidates the generator built at root's fixpoint, with no field of the config reaching it, the obligations rung in report order, before any filter. It is `emitted` plus the sum of `filtered` |
| `root.hypgen.emitted` | the candidates that survived every filter: **H**, the universe every route reads, so it equals `region.universe.candidates` and is the universe `ein solve --monotone` prints, whose `region` is `null`; on a saved file's run, whose universe is H less its node's choices, it is `region.universe.candidates` and `root.commit`'s count. A repeat is dropped as `seen_in_call` before it counts |
| `root.hypgen.filtered` | why a built candidate is not in H, sparse, only the keys that fired: `fact_already_exists` (root holds it), `negated_fact` (root holds its `(not …)`), `seen_in_call` (proposed again, from another atom the enumerator proposes about) |
| `root.hypgen.pre_candidate` | relation and slot skips before any candidate exists, sparse: `closed_relation` (a `(__closed__ R)` root holds, which the program states or derives through `std.closure`), `no_hypothesis_relation` and `relation_not_whitelisted` (the `(solve …)` lists `:no-hypothesis` and `:hypothesis-relations`), `self_edge` (both arguments the same atom) |
| `root.hypgen.later_debts` | what H holds for debts root does not owe and a commitment can bring, **written only where H holds a candidate for one**, so the key is absent from every other summary: one `{obligation, activator, relation, by, added, candidates}` per activator of an obligation rule that root's fixpoint does not hold and a rule's `:assert` concludes, in the order the obligations rung read them. `obligation` is the rule, `activator` the fact that would activate it, `relation` the relation its plan owes, `by` the rules that conclude the activator, `candidates` the facts of H that would pay the debt and that no debt before it proposed, in H's order, and `added` their count. The read-out's `later debt` rows say the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |
| `root.hypgen.outside` | what the universe leaves out ([Q-M1e2.3](../../../decisions/answer/Q-M1e2.3-r-is-over-the-universe-and-the-read-out-names.md)): one `{relation, arity, why}` per relation the program can conclude or ask about, does not scope out itself, and of which the rung that built H proposes no fact, sorted by name, `[]` when none. `why` is `not-owed` (the obligations rung: no obligation owes it), `no-hrule` (the hrules: none asserts it) or `arity` (the blind enumerator, which proposes arity 1 and 2 only). The read-out's `outside` row says the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |

## Where it is read

`ein render region CUBE --answer DIR` reads `index.json`, and draws each state on the oracle's cube against the part of
R the run asked ([`12-rendering.md`](12-rendering.md)). `ein render kb DIR/models/NN.ein` draws one state, and every
command reads a state file as the node it names.
