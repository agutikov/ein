# The answer dump — `ein solve --dump-answer DIR`

**What a solve answered, as files.** One directory per run: the read-out uncut, the summary, one **loadable** program
per state the run reached, the owing frontier of an empty R, the minimal dead sets, each form asked for uncut, and an
index naming each. It is the answer of the question the run asked: R, or the part of it the question names, as
[`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md) defines it, in the format
`ein-answer/2`.

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
  models/NN.ein   program + one state's facts, one per state the run reached
  owing/NN.ein    program + one owing node's facts, on an empty R of the owing kind
  nogoods.json    N: each minimal dead set, its core, the core's status and the rules that refuted it
  forms/F.txt     each form asked for, uncut: -H, --models key|grid, -p, -P, -f, --diff-models
```

- **`NN` is stdout's numbering.** `models/02.ein` is *state 2 of k* in the `(region …)` block, two or more digits, so
  the files list in that order. On a partial run it is every state the run reached, listed or not: a cut at 100 lines
  (`-n`, or the shared stdout limit) cuts stdout, not the directory, and `index.json` says which files stdout listed.
  A line mode past `--region-limit` on a flagged program lists the states describe certifies on its line and writes
  every other member it met unlisted, with no line
  ([Q-M1e2.19](../../../decisions/answer/Q-M1e2.19-past-the-limit-a-line-mode-certifies-what.md)).
- **`index.json`** carries `asked` and the same `region` block as the summary, per state file its `path`, `state`
  number, `lines` (`minimal` / `maximal`) and `listed`, and `forms`.
- **A state file's header** gives its number, the commitment it was read from, its lines, how many commitments reach
  it and its minimal generators. **A count of commitments, here and in `answer.json`, is reported up to $2^{63} - 1$**:
  `9223372036854775807` reads *that many or more*, and a state file's header says so, *reached by:
  9223372036854775807 or more commitments*
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
  `answer.txt` lists: `hyp-stats.txt` (`-H`), `key.txt` or `grid.txt` (`--models`), `print-final-state.txt`,
  `print-final-positive.txt` and `print-final-hfacts.txt` (`-p`, `-P`, `-f`) and `diff-models.txt` (`--diff-models`).
  Stdout prints what fits of each in what the read-out left of its 100 lines, drawn from the states it showed, and its
  last line names the file. A file is what the form prints with nothing cut: where nothing is cut it is stdout's form
  byte for byte. `index.json`'s **`forms`** lists each as `{form, path}`, in the order stdout prints them, and is `[]`
  where no form was asked for.

Each state file costs one more saturation of a state the run already classified, and is mostly the program, so the
directory's size is about $k$ times the inlined program.

## Every state file loads, and is a fixpoint

A state file is the program as loaded, imports inlined, followed by every fact of the state, one a line, sorted by its
text. The program is written with every module it imports whole, under the names the program prints: its own spelling
where it has one, the full name otherwise ([`11-macros.md`](../03-syntax-and-semantics/11-macros.md) has imports). A state file
declares the full names it was loaded with, `std.math.decimal-arith.strip`, and imports nothing, which the loader reads
as the program's own names
([Q-M1f.239](../../../decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md)). The program's
own `(relation …)` declarations are not repeated, because the loader refuses a second one, and neither are the loader's
`(written T)` facts, which the loader refuses where a file writes one and states again from the facts the file holds
([`07-diagnostics.md`](07-diagnostics.md)): from every fact of the state, a fact the run derived among them, so the reloaded
state holds every `written` fact the state held and one for each fact nested only in a derived one, and the header
says so where the state holds any. So the file **loads as a program**, and saturating it **derives no fact and no
`(false)`**. It holds every candidate of its state's closure and no other candidate. Each state is read again the way
every route reads a node, so the file is the state the read-out names and not a copy of one.

**It loads to the state.** Each fact is written as text the loader reads back to the same object: a name bare where it
reads as one name (`House-1`, `neqw`, `1`, `07`, `1..5`), a nested fact `(rel …)`. A name the loader made up, which
starts with `#`, is written quoted and does not load back. So loading `models/NN.ein` gives that state's facts, **the
same objects**, each fact as given.

**What a reloaded state's facts rest on: nothing; each reads as given.** A fact's provenance is written as a
**comment** after it, which the loader skips: `; source "…"` for a fact the program states with a source, `; rule r`
for one a rule derived, `; hypothesis N` for the commitment. A state file is the state's facts, not a second kind of
program that re-states where each came from, so a reloaded state is its facts as given, and the comment is for a
reader.

**The comment says the fact in English**
([Q-M1f.58](../../../decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-a-state-file.md)): a derived fact's rule
is followed by its `:why` filled with the bindings of the fact's first recorded justification, and every fact whose
relation carries a `:why` by that `:why` filled with its arguments, after ` — ` (alone, for a given with no sentence):

```lisp
(co-located Englishman Red) ; source "condition (2)" — Englishman and Red are in the same house
(co-located House-3 Milk) ; rule symmetric: co-located is symmetric: Milk ↔ House-3. — House-3 and Milk are in the same house
(instance House-5 House) ; House-5 is an instance of House
```

A line break in a text starts a new comment line, `;   `, so the text is carried character for character and the file
still loads to its facts. A fact with no text at all keeps the comment it had, or none. Which justification is *first
recorded* can move with rule order: a reason is an explanation, as a core is
([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).

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
- **A `.einb` writes the directory its `.ein` writes**: every state file starts with the program the container
  stores, resolved and import-flattened, as `ein render program` draws it
  ([Q-M1f.17](../../../decisions/cli/Q-M1f.17-a-view-of-a-container-draws-the-program-it.md)). On a program that
  imports, that program keeps every declaration its imports brought
  ([Q-M1f.27](../../../decisions/cli/Q-M1f.27-a-containers-answer-directory-writes-the-program.md)), and every file
  but a state file's program is the `.ein`'s, but for the path it names.
- **A budget stop is an answer**: `--region-budget` and `--region-limit` write the whole directory, and `index.json`'s
  `region` says `exhausted: false` and `stopped: {why, …}`. **A `-T` abort is not**: it exits 2, and the directory it
  made first stays empty.
- **A second dump into the same directory leaves nothing stale.** When `index.json` names this format, `models/`,
  `owing/` and `forms/` are cleared before writing. A directory holding anything else is written beside and nothing
  in it is deleted.
- **`--count` writes two files**: `answer.txt`, its read-out uncut, every warning row among it, and `answer.json`, its
  summary, whose `asked.ask` is `count` and whose `region.states` and `region.frontier` are `null`. It reads no state,
  so it writes no state file, no form, no `index.json` and no `nogoods.json`, as `--monotone` writes none (every form
  is refused beside `--count`, exit 2); and where `index.json` names this format it removes what that dump left, the
  two top-level files with the subtrees.
- **`--no-search` writes root**: `answer.txt`, `answer.json`, root's state as `root.ein` (loadable, as a state file is,
  its header saying what root owes and *R: not searched*) and `forms/`, and no `index.json`, `nogoods.json` or state of
  R.

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
`root.hypgen.emitted` is `region.universe.candidates` on every run, and `--monotone`'s block is the region run's. It
holds root's standing too, `dead`, `owes` and `owed`, on every question: `ein solve --no-search` states no R, so its
`region` is `null` as `--monotone`'s is, and the block is where it says what root is
([Q-M1f.259](../../../decisions/read-out/Q-M1f.259-under-no-search-the-summary-states-no-region.md)); the three
are the full run's `region.root`, `root_owes` and `root_owed`.

| field | what it holds |
|---|---|
| `root.facts` | the facts of root's fixpoint, $\operatorname{sat}(\text{program})$: what `ein saturate FILE` prints as *facts (total)* after saturation, and what the file of state `∅` holds when $\emptyset \in R$. No route writes into root, so it is root before any commitment and after every one |
| `root.facts_by_relation` | those facts by relation name, sorted, only the relations that have one. They sum to `root.facts` |
| `root.dead` | root's own fixpoint derives `(false)`: root's standing, which every run reaches first and `ein solve --no-search` reports. It is `region.root` = `dead` on every description |
| `root.owes` | the obligation instances root's fixpoint leaves undischarged, dead or not, as a node counts them: `region.root_owes` on every description. With `root.dead`, whether $\emptyset$ is in R |
| `root.owed` | those instances, each through its rule's `:why` (the rule's name where it has none), in the order the rules report them: `region.root_owed`, and the read-out's `owed` rows |
| `root.plans` | the plans root's saturation compiled, the derived activators' included |
| `root.saturator.naf_rounds` | how many times root's saturation reached the NAF boundary with candidates parked on an `absent` guard, and judged them against the quiesced world |
| `root.saturator.naf_admitted` | parked candidates the boundary admitted: every guard passed, and the firing was queued |
| `root.saturator.naf_retired` | parked candidates the boundary retired as dead: an anti-monotone guard found a match, which a growing KB keeps |
| `root.saturator.naf_dropped` | always 0: a guard is judged once, at the boundary, so no admitted firing is dropped later. A boundary that judged wrong would show here |
| `root.hypgen.raw` | candidates the generator built at root's fixpoint, with no field of the config reaching it, the obligations rung in report order, before any filter. It is `emitted` plus the sum of `filtered` |
| `root.hypgen.emitted` | the candidates that survived every filter: **H**, the universe every route reads, so it equals `region.universe.candidates` and is the universe `ein solve --monotone` prints, whose `region` is `null`. A repeat is dropped as `seen_in_call` before it counts |
| `root.hypgen.filtered` | why a built candidate is not in H, sparse, only the keys that fired: `fact_already_exists` (root holds it), `negated_fact` (root holds its `(not …)`), `seen_in_call` (proposed again, from another atom the enumerator proposes about) |
| `root.hypgen.pre_candidate` | relation and slot skips before any candidate exists, sparse: `closed_relation` (a `(__closed__ R)` root holds, which the program states or derives through `std.closure`), `no_hypothesis_relation` and `relation_not_whitelisted` (the `(solve …)` lists `:no-hypothesis` and `:hypothesis-relations`), `self_edge` (both arguments the same atom) |
| `root.hypgen.later_debts` | what H holds for debts root does not owe and a commitment can bring, **written only where H holds a candidate for one**, so the key is absent from every other summary: one `{obligation, activator, relation, by, added, candidates}` per activator of an obligation rule that root's fixpoint does not hold and a rule's `:assert` concludes, in the order the obligations rung read them. `obligation` is the rule, `activator` the fact that would activate it, `relation` the relation its plan owes, `by` the rules that conclude the activator, `candidates` the facts of H that would pay the debt and that no debt before it proposed, in H's order, and `added` their count. The read-out's `later debt` rows say the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |
| `root.hypgen.outside` | what the universe leaves out ([Q-M1e2.3](../../../decisions/answer/Q-M1e2.3-r-is-over-the-universe-and-the-read-out-names.md)): one `{relation, arity, why}` per relation the program can conclude or ask about, does not scope out itself, and of which the rung that built H proposes no fact, sorted by name, `[]` when none. `why` is `not-owed` (the obligations rung: no obligation owes it), `no-hrule` (the hrules: none asserts it) or `arity` (the blind enumerator, which proposes arity 1 and 2 only). The read-out's `outside` row says the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |
| `root.hypgen.types_as_objects.objects` | the atoms the key calls *types as objects*, its *type* an atom an `is-a` fact points at and its *object* a guessable atom ([Q-M1e.12](../../../decisions/hypotheses/Q-M1e.12-the-blind-enumerator-is-untyped-and-an-atom-on.md)): where the blind enumerator built H, each atom that is the second argument of an `(is-a x y)` fact at root's fixpoint and an argument of a candidate, sorted, `[]` when none. The read-out's `types` row under `universe` says the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |
| `root.hypgen.types_as_objects.relations` | the relations of the candidates that name one, sorted, `[]` when none |
| `root.hypgen.types_as_objects.candidates` | how many candidates of H name one, `0` when none |
| `root.hypgen.relation_names_as_objects.objects` | the relations that are guessable atoms and that a candidate has as an argument, the key's *relation names as objects*, **the three keys written only where there is one**, so they are absent from every other summary: where the blind enumerator built H, each argument of a candidate that is the head of a fact root's fixpoint holds or a relation a rule's `:assert` names, sorted. No file declares such a relation or writes a fact of it. The read-out's `relation names` row under `universe` says the same in words ([`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)) |
| `root.hypgen.relation_names_as_objects.relations` | the relations of the candidates that name one, sorted |
| `root.hypgen.relation_names_as_objects.candidates` | how many candidates of H name one |

## Where it is read

`ein render region CUBE --answer DIR` reads `index.json`, and draws each state on the oracle's cube against the part of
R the run asked ([`12-rendering.md`](12-rendering.md)). `ein render kb DIR/models/NN.ein` draws one state.
