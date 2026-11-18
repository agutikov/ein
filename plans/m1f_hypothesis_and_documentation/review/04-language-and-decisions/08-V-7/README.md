# V-7 — a rule can conclude a fact of a relation the program does not count

## 1. The question

A rule may take a relation as a parameter, `(rule unpack (?R) … :assert (?R ?a ?b))`, and an **activator** fact,
`(unpack =)`, binds the parameter. When the relation the activator names is declared nowhere and heads no fact the
file writes (`=` below is written only inside another fact, and `happy` only in the activator), the rule still
concludes facts of it. The engine keeps several lists of "the program's relations", and they disagree about such a
relation: the **registry** (`Program::relations`: declared relations and heads of written facts) does not hold it;
the goal check (`program_relations`) and the reasoning page's $\operatorname{rels}(P)$ do. The question is which
way to make them agree: the registry takes the relation, the loader refuses the activator, or a page says that such
a relation is declared.

Recorded at [`findings.md` § V-7](../../../p1f.21_verdicts_as_rules/findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count),
with [`open_facts.md` § 8.2](../../../p1f.21_verdicts_as_rules/open_facts.md#82-each-name-under-this-phases-direction)'s
row for `relation` (*"any new head of tuple will be new relation atom"*, the user's third instruction).

## 2. What is so today

All runs use `build/review-bin/ein-head-17085b27`, today's binary, every P1f.23 stage merged; [`rerun.sh`](rerun.sh) makes every file below.

### 2.1 The witness, as the ledger writes it

[`programs/v7_unpack.ein`](programs/v7_unpack.ein): `(think Bob (= 7 (+ 3 4)))`, the rule `unpack (?R)` concluding
`(?R ?a ?b)` from `(think ?p (?R ?a ?b))`, and `(unpack =)`.

- **The read-out** says nothing about it: R is {∅}, *universe: no candidate*, `(model)`
  ([`runs/v7_unpack.solve.txt`](runs/v7_unpack.solve.txt)). Root's derivations, `-p`, list the fact
  ([`runs/v7_unpack.solve-p.txt`](runs/v7_unpack.solve-p.txt)):

  <!-- output -->
  ```text
  final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 1 fact):
    (= 7 (+ 3 4))
  ```
- **`ein saturate --dump`, the registry**: two relations, `think` and `unpack` (the activator's head), and a derived
  fact of a third ([`runs/v7_unpack.saturate-dump.txt`](runs/v7_unpack.saturate-dump.txt)):

  <!-- output -->
  ```text
    relations (total)                                2       2
      open-world (auto-vivified)                     2       2
    facts by relation:
      =                                            0       1  +    1
  ;; Relations — auto-vivified (2)
  ;;   think
  ;;   unpack
  ;; ── DERIVED (1 facts) ──
    (= 7 (+ 3 4) :rule unpack)
  ```
- **`--json-summary`'s relation counts** count facts by their head, so `=` is there:
  `root.facts_by_relation = {"=": 1, "think": 1, "unpack": 1}`; no key counts the registry
  ([`runs/v7_unpack.summary.json`](runs/v7_unpack.summary.json)).
- **`ein show hyps`**: no candidate, built by the rung `blind` ([`runs/v7_unpack.show-hyps.stderr.txt`](runs/v7_unpack.show-hyps.stderr.txt);
  stdout empty, [`runs/v7_unpack.show-hyps.txt`](runs/v7_unpack.show-hyps.txt)).
- **The drawings** read `=` as one atom, the relation of the derived fact and the argument of the activator.
  **`ein render kb`** draws root: the activator `(unpack =)` is an octagon whose arrow `1` is at the box `unpack` and
  whose arrow `2` is at the box `=`, and the derived `(= 7 (+ 3 4))`, dashed with `by unpack` beside it, has its arrow
  `1` at that same box; `(+ 3 4)`, under its arrow `3` and held by nothing, stands in the frame `= ctx`
  ([`renders/v7_unpack.kb.svg`](renders/v7_unpack.kb.svg)). **`ein render program`** draws the program's `kb` (the
  two written facts) and the rule, whose conclusion is an octagon with its arrow `1` at the diamond `?R`
  ([`renders/v7_unpack.program.01-kb.svg`](renders/v7_unpack.program.01-kb.svg),
  [`renders/v7_unpack.program.02-rule_unpack_lhs_rhs.svg`](renders/v7_unpack.program.02-rule_unpack_lhs_rhs.svg)).
  The trace's one step, each fact its octagon: [`renders/v7_unpack.trace-1.svg`](renders/v7_unpack.trace-1.svg).

  ![ein render kb of v7_unpack.ein: the activator's arrow 2 and the derived fact's arrow 1 meet at one box, =](renders/v7_unpack.kb.svg)
- `--events` ([`runs/v7_unpack.events.md`](runs/v7_unpack.events.md)): the `load` event's `relation_names` is the
  registry, `["think","unpack"]`; the `fire` event derives `(= 7 (+ 3 4))`.

### 2.2 A relation named only by an activator

[`programs/v7_tag.ein`](programs/v7_tag.ein): `(tag happy)` activates `tag (?R)`, which concludes `(?R ?x)` for
every person. `happy` is written nowhere else. The rule derives `(happy Ann)`; the registry lists `person` and `tag`
only ([`runs/v7_tag.saturate-dump.txt`](runs/v7_tag.saturate-dump.txt)); and a goal over `happy` **loads** and holds
([`runs/v7_tag.test.txt`](runs/v7_tag.test.txt)), while the same goal misspelt `hapy` is refused
([`programs/v7_tag-typo.ein`](programs/v7_tag-typo.ein), [`runs/v7_tag-typo.solve.txt`](runs/v7_tag-typo.solve.txt)).
So the goal check counts `happy` (it reads a name *passed where a relation's name is held*) and the registry does not.

### 2.3 Where the engine counts relations: four lists

[`runs/relation-lists-code.txt`](runs/relation-lists-code.txt) has the code, with line numbers (read, not run):

| list | what it holds | `=` / `happy`? | read by |
|---|---|---|---|
| **the registry**, `ein_core::Program::relations`, filled by `from_ir.rs:5060` | declared relations, and the head of every fact a file states (*auto-vivified*) | no | `Program::categorise` (a registered name is a relation, any other an **object**), so the blind rung's objects; the `rule_apps_on_rel` and `rules_by_relation` indexes; `ein saturate`'s counts; the `load` event; a fact's English |
| **the alive-set invariant's baseline**, `invariant.rs` `Universe::of_facts` | the registry, plus every **constant** head of a rule's `:assert` | no: `(?R …)` has no constant head | `ein-infer/tests/alive_invariant.rs`' `no_run_reaches_a_name_the_program_does_not_have`, which would report `new-relation: = in (= 7 (+ 3 4))` (read, not run; the ledger records the line for `06_double_negation.ein` as first written) |
| **the goal's and claim's list**, `from_ir.rs` `program_relations` (line 707) | every head at any depth of a fact or a rule's text, every name passed where a relation's name is held, `eq`, `neq` (no longer `is-a`: Q-M1f.436) | yes | the refusal of a goal, a claim and a `(solve …)` list over a relation the program does not name |
| **the reasoning's** $\operatorname{rels}(P)$, [`02-saturation.md`](../../../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md) clause 2 | *"the declared ones, the relations of the program's facts, and every atom under the first arrow of a fact pattern a rule can conclude"* | yes | the page's *no new relations* |

### 2.4 Where the registry changes an answer: the blind rung

The registry decides what the blind rung takes for an object, so a relation outside it is **guessed about as an
object**. [`programs/v7_blind.ein`](programs/v7_blind.ein) is `v7_tag.ein` with a second declared relation, `likes P
X`, and no `:no-search`:

<!-- output -->
```text
  states (k_s)      8
  universe          3 candidates, 0 dead alone
    relation names  happy — a relation only a rule concludes, and 3 candidates of likes, person name it as an object: no file declares it or writes a fact of it
```

([`runs/v7_blind.solve.txt`](runs/v7_blind.solve.txt); H is `(likes Ann happy)`, `(likes happy Ann)`,
`(person happy)`, [`runs/v7_blind.show-hyps.txt`](runs/v7_blind.show-hyps.txt); the read-out's `relation names` row is
[`08-read-out.md`](../../../../../docs/ein/user/04-reference/08-read-out.md)'s.) Committing `(person happy)` makes
`tag` derive `(happy happy)`. **The same holds of a relation a rule concludes with a constant head**:
[`programs/v7_blind-constant-head.ein`](programs/v7_blind-constant-head.ein), `:assert (happy ?x)` with no parameter,
answers the same 8 states and the same H ([`runs/v7_blind.vs-constant-head.solve.diff`](runs/v7_blind.vs-constant-head.solve.diff):
the path line only). So today the activator case and the constant-head case answer alike; they differ only in list 2,
the invariant's baseline.

### 2.5 How often, in the corpus

[`runs/v7_census.md`](runs/v7_census.md), by [`runs/v7_census.py`](runs/v7_census.py) over `ein saturate --dump` of
every corpus program (root only, 664 of 888 saturated; the 224 others are `examples/broken/` and `examples/ein-bugs/`
load refusals): **no program's root holds a fact of a relation that is outside the registry and written as no head
anywhere** (0). 29 programs hold a fact of a relation only a rule concludes with a constant head (50 heads: `total`,
`surjective`, `explicitly-dislikes`, `liquid`, …), the case of § 2.4's second program. The text test is a grep, a
heuristic; a relation that appears only in a committed state is not counted.

`examples/beliefs/06_double_negation.ein` declares `=` now. With the declaration deleted
([`programs/06_double_negation-undeclared.ein`](programs/06_double_negation-undeclared.ein)) `ein test` still holds
its three claims; the registry loses `=` and the index `rule_apps_on_relation` loses its 2 entries
([`runs/06_double_negation.declared-vs-undeclared.test.diff`](runs/06_double_negation.declared-vs-undeclared.test.diff),
[`runs/06_double_negation.declared-vs-undeclared.saturate-dump.diff`](runs/06_double_negation.declared-vs-undeclared.saturate-dump.diff)).
`--json-summary` differs only in the two declaration facts
([`runs/06_double_negation.declared-vs-undeclared.summary.diff`](runs/06_double_negation.declared-vs-undeclared.summary.diff)).

## 3. The options

### (a) The registry takes it

The loader adds to `Program::relations`, as auto-vivified (no labels, no `:why`), every relation an activator names
for a parameter that heads a conclusion.

- **`ein saturate --dump`** of the witness: the registry holds three relations, `=` among them, `=` is a relation and no
  longer an object, and `(unpack =)` is indexed as an activator *on* `=`.
  [`runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt`](runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt),
  a **mock**: today's dump with the lines edited that follow from the registry holding `=` (the two relation counts,
  the two name categories, `rule_apps_on_relation`, the auto-vivified list); its diff against today's is
  [`runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.diff`](runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.diff).
- **It moves R where the blind rung runs.** `happy` becomes a relation, so the blind rung stops proposing facts about
  it: `v7_blind.ein` goes from 8 states to 1. This is **run, not mocked**, by an equivalent program:
  [`programs/v7_blind-registered.ein`](programs/v7_blind-registered.ein) adds the fact `(happy Ann)`, which puts
  `happy` in the registry exactly as (a) would (auto-vivified, no labels), and which root's fixpoint holds anyway.
  [`runs/v7_blind.vs-registered.solve.diff`](runs/v7_blind.vs-registered.solve.diff),
  [`runs/v7_blind.vs-registered.show-hyps.diff`](runs/v7_blind.vs-registered.show-hyps.diff); the read-out it gives,
  renamed to `v7_blind.ein`, is [`runs/v7_blind.solve.MOCK.a-registry-takes-it.txt`](runs/v7_blind.solve.MOCK.a-registry-takes-it.txt).
- Whether (a) also takes a **constant** head of a rule's `:assert` (`total`, `explicitly-dislikes`, § 2.5's 29
  programs) is part of the call: if it does not, § 2.4's two programs, which answer alike today, answer differently
  (8 states and 1); if it does, those 29 programs' H can move where the blind rung runs. Not measured.
- The invariant's sweep, the goal check and the reasoning page then agree with the registry.

### (b) The loader refuses the activator

A **mock** of the refusal, the line written for this review in the style of the activator refusals the loader prints
today (`rule 'tag': its parameter ?R names a relation, and (tag (f a)) binds it to the fact (f a) — …`):

<!-- output -->
```text
kb load error: …/v7_unpack.ein:15:1: (unpack =) gives unpack's parameter ?R the relation =, and ?R heads unpack's :assert, so unpack concludes facts of = — the program declares no relation = and states no fact of it; declare it, (relation = …), so that its facts have a count and a sentence
exit 1
```

[`runs/v7_unpack.solve.MOCK.b-refused.txt`](runs/v7_unpack.solve.MOCK.b-refused.txt),
[`runs/v7_tag.solve.MOCK.b-refused.txt`](runs/v7_tag.solve.MOCK.b-refused.txt),
[`runs/v7_blind.solve.MOCK.b-refused.txt`](runs/v7_blind.solve.MOCK.b-refused.txt),
[`runs/06_double_negation-undeclared.solve.MOCK.b-refused.txt`](runs/06_double_negation-undeclared.solve.MOCK.b-refused.txt)
(two lines, one per activator). **What the author then writes moves R where the blind rung runs**, run on the
declared programs:

- [`programs/v7_unpack-declared.ein`](programs/v7_unpack-declared.ein), `(relation = A B …)` added: the blind rung now
  guesses facts of `=` over `Bob`, `3`, `4`, `7`, **12 candidates and 4 096 states**, where the undeclared program has
  {∅} ([`runs/v7_unpack-declared.show-hyps.txt`](runs/v7_unpack-declared.show-hyps.txt),
  [`runs/v7_unpack-declared.solve.txt`](runs/v7_unpack-declared.solve.txt)). `06_double_negation.ein` states
  `(solve :no-search)`, which is why declaring `=` there moved nothing.
- [`programs/v7_blind-declared.ein`](programs/v7_blind-declared.ein), `(relation happy P …)` added: 8 states → 1, as
  under (a) ([`runs/v7_blind-declared.solve.txt`](runs/v7_blind-declared.solve.txt)).

No corpus program would be refused at root (§ 2.5's census finds none); a stdlib rule activated through a parameter
by a relation a program never writes would be. Not measured past root.

### (c) A page says that such a relation is declared

Nothing in the engine moves; the lists keep disagreeing, and the invariant's sweep keeps calling a corpus program
like the witness a breach. A **mock** of the sentence, for [`05-rules.md` § Parameters and activators](../../../../../docs/ein/user/03-syntax-and-semantics/05-rules.md#parameters-and-activators),
after *"an activator may itself be derived by a rule, and then the rule starts firing once it is."*:

> A relation that a parameter heads in a conclusion is declared, or heads a fact the program writes: one named only
> by an activator, `(unpack =)` with `=` written nowhere else, is concluded all the same, but the program does not
> count it among its relations, and the blind rung takes its name for an object.

and for [`02-saturation.md`](../../../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md)'s clause 2, *"every atom
under the first arrow of a fact pattern a rule can conclude"* becomes *"every atom a rule writes under the first
arrow of a conclusion"*, which leaves a parameter's binding out.

## 4. The default

**None taken.** The ledger: *"The user's call, none taken … Under the third instruction's reading of `relation`,
any head is a relation, the first."*

## 5. Files

- [`rerun.sh`](rerun.sh) — every run and drawing below; [`tools/`](tools/) its helpers.

Programs:
- [`programs/v7_unpack.ein`](programs/v7_unpack.ein) — the ledger's witness; `=` only inside a fact and in the activator.
- [`programs/v7_unpack-declared.ein`](programs/v7_unpack-declared.ein) — the same with `=` declared (what (b) makes an author write).
- [`programs/v7_tag.ein`](programs/v7_tag.ein) — `happy` named only by the activator `(tag happy)`; a goal over it.
- [`programs/v7_tag-typo.ein`](programs/v7_tag-typo.ein) — the same goal misspelt `hapy`, refused.
- [`programs/v7_blind.ein`](programs/v7_blind.ein) — `v7_tag` under the blind rung: `happy` guessed about as an object.
- [`programs/v7_blind-constant-head.ein`](programs/v7_blind-constant-head.ein) — the same with `happy` a constant head of the rule.
- [`programs/v7_blind-registered.ein`](programs/v7_blind-registered.ein) — the same with `happy` in the registry: option (a), emulated.
- [`programs/v7_blind-declared.ein`](programs/v7_blind-declared.ein) — the same with `happy` declared: option (b)'s fix.
- [`programs/06_double_negation.ein`](programs/06_double_negation.ein) — copy of `examples/beliefs/06_double_negation.ein`.
- [`programs/06_double_negation-undeclared.ein`](programs/06_double_negation-undeclared.ein) — the same with `=`'s declaration deleted.

Runs (each `PROGRAM.COMMAND`): `solve.txt` the read-out, `solve-p.txt` with root's facts, `summary.json`
`--json-summary`, `saturate-dump.txt` `ein saturate --dump`, `show-hyps.txt` / `.stderr.txt` `ein show hyps`,
`test.txt` `ein test -v`, `h-space.txt` `ein render h-space`:
- [`runs/v7_unpack.solve.txt`](runs/v7_unpack.solve.txt), [`runs/v7_unpack.solve-p.txt`](runs/v7_unpack.solve-p.txt), [`runs/v7_unpack.summary.json`](runs/v7_unpack.summary.json), [`runs/v7_unpack.saturate-dump.txt`](runs/v7_unpack.saturate-dump.txt), [`runs/v7_unpack.show-hyps.txt`](runs/v7_unpack.show-hyps.txt), [`runs/v7_unpack.show-hyps.stderr.txt`](runs/v7_unpack.show-hyps.stderr.txt), [`runs/v7_unpack.test.txt`](runs/v7_unpack.test.txt) (no claim: exit 2, *nothing to check*), [`runs/v7_unpack.trace.md`](runs/v7_unpack.trace.md), [`runs/v7_unpack.events.jsonl`](runs/v7_unpack.events.jsonl) and its table [`runs/v7_unpack.events.md`](runs/v7_unpack.events.md).
- [`runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt`](runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt) and [`.diff`](runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.diff) — mock, option (a).
- [`runs/v7_unpack.solve.MOCK.b-refused.txt`](runs/v7_unpack.solve.MOCK.b-refused.txt), [`runs/v7_tag.solve.MOCK.b-refused.txt`](runs/v7_tag.solve.MOCK.b-refused.txt), [`runs/v7_blind.solve.MOCK.b-refused.txt`](runs/v7_blind.solve.MOCK.b-refused.txt), [`runs/06_double_negation-undeclared.solve.MOCK.b-refused.txt`](runs/06_double_negation-undeclared.solve.MOCK.b-refused.txt) — mocks, option (b).
- [`runs/v7_unpack-declared.solve.txt`](runs/v7_unpack-declared.solve.txt), [`runs/v7_unpack-declared.summary.json`](runs/v7_unpack-declared.summary.json) (3.3 MB: every one of the 4 096 states), [`runs/v7_unpack-declared.saturate-dump.txt`](runs/v7_unpack-declared.saturate-dump.txt), [`runs/v7_unpack-declared.show-hyps.txt`](runs/v7_unpack-declared.show-hyps.txt), [`runs/v7_unpack-declared.show-hyps.stderr.txt`](runs/v7_unpack-declared.show-hyps.stderr.txt).
- [`runs/v7_tag.solve.txt`](runs/v7_tag.solve.txt), [`runs/v7_tag.summary.json`](runs/v7_tag.summary.json), [`runs/v7_tag.saturate-dump.txt`](runs/v7_tag.saturate-dump.txt), [`runs/v7_tag.show-hyps.txt`](runs/v7_tag.show-hyps.txt), [`runs/v7_tag.show-hyps.stderr.txt`](runs/v7_tag.show-hyps.stderr.txt), [`runs/v7_tag.test.txt`](runs/v7_tag.test.txt), [`runs/v7_tag-typo.solve.txt`](runs/v7_tag-typo.solve.txt).
- [`runs/v7_blind.solve.txt`](runs/v7_blind.solve.txt), [`runs/v7_blind.solve-p.txt`](runs/v7_blind.solve-p.txt), [`runs/v7_blind.summary.json`](runs/v7_blind.summary.json), [`runs/v7_blind.saturate-dump.txt`](runs/v7_blind.saturate-dump.txt), [`runs/v7_blind.show-hyps.txt`](runs/v7_blind.show-hyps.txt), [`runs/v7_blind.show-hyps.stderr.txt`](runs/v7_blind.show-hyps.stderr.txt), [`runs/v7_blind.h-space.txt`](runs/v7_blind.h-space.txt).
- [`runs/v7_blind-constant-head.solve.txt`](runs/v7_blind-constant-head.solve.txt), [`runs/v7_blind-constant-head.saturate-dump.txt`](runs/v7_blind-constant-head.saturate-dump.txt), [`runs/v7_blind-constant-head.show-hyps.txt`](runs/v7_blind-constant-head.show-hyps.txt), [`runs/v7_blind-constant-head.show-hyps.stderr.txt`](runs/v7_blind-constant-head.show-hyps.stderr.txt), [`runs/v7_blind.vs-constant-head.solve.diff`](runs/v7_blind.vs-constant-head.solve.diff).
- [`runs/v7_blind-registered.solve.txt`](runs/v7_blind-registered.solve.txt), [`runs/v7_blind-registered.solve-p.txt`](runs/v7_blind-registered.solve-p.txt), [`runs/v7_blind-registered.summary.json`](runs/v7_blind-registered.summary.json), [`runs/v7_blind-registered.saturate-dump.txt`](runs/v7_blind-registered.saturate-dump.txt), [`runs/v7_blind-registered.show-hyps.txt`](runs/v7_blind-registered.show-hyps.txt), [`runs/v7_blind-registered.show-hyps.stderr.txt`](runs/v7_blind-registered.show-hyps.stderr.txt), [`runs/v7_blind-registered.h-space.txt`](runs/v7_blind-registered.h-space.txt); its diffs against `v7_blind`: [`solve`](runs/v7_blind.vs-registered.solve.diff), [`show-hyps`](runs/v7_blind.vs-registered.show-hyps.diff), [`saturate-dump`](runs/v7_blind.vs-registered.saturate-dump.diff), [`summary`](runs/v7_blind.vs-registered.summary.diff); [`runs/v7_blind.solve.MOCK.a-registry-takes-it.txt`](runs/v7_blind.solve.MOCK.a-registry-takes-it.txt) and [`.diff`](runs/v7_blind.solve.MOCK.a-registry-takes-it.diff) (the registered run under `v7_blind.ein`'s name).
- [`runs/v7_blind-declared.solve.txt`](runs/v7_blind-declared.solve.txt), [`runs/v7_blind-declared.summary.json`](runs/v7_blind-declared.summary.json), [`runs/v7_blind-declared.saturate-dump.txt`](runs/v7_blind-declared.saturate-dump.txt), [`runs/v7_blind-declared.show-hyps.txt`](runs/v7_blind-declared.show-hyps.txt), [`runs/v7_blind-declared.show-hyps.stderr.txt`](runs/v7_blind-declared.show-hyps.stderr.txt).
- [`runs/06_double_negation.solve.txt`](runs/06_double_negation.solve.txt), [`runs/06_double_negation.summary.json`](runs/06_double_negation.summary.json), [`runs/06_double_negation.saturate-dump.txt`](runs/06_double_negation.saturate-dump.txt), [`runs/06_double_negation.test.txt`](runs/06_double_negation.test.txt), and the `-undeclared` four ([`solve`](runs/06_double_negation-undeclared.solve.txt), [`summary`](runs/06_double_negation-undeclared.summary.json), [`saturate-dump`](runs/06_double_negation-undeclared.saturate-dump.txt), [`test`](runs/06_double_negation-undeclared.test.txt)); the three diffs [`test`](runs/06_double_negation.declared-vs-undeclared.test.diff), [`saturate-dump`](runs/06_double_negation.declared-vs-undeclared.saturate-dump.diff), [`summary`](runs/06_double_negation.declared-vs-undeclared.summary.diff).
- [`runs/relation-lists-code.txt`](runs/relation-lists-code.txt) — the four lists' code and every reader of the registry, with line numbers, written by [`tools/relation_lists_code.sh`](tools/relation_lists_code.sh) from the checkout.
- [`runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt`](runs/v7_unpack.saturate-dump.MOCK.a-registry-takes-it.txt) is remade from each new dump by [`tools/saturate-dump.MOCK.a-registry-takes-it.patch`](tools/saturate-dump.MOCK.a-registry-takes-it.patch), the hand edit; [`tools/events_table.py`](tools/events_table.py) writes the events' table.
- [`runs/v7_census.md`](runs/v7_census.md), [`runs/v7_census.py`](runs/v7_census.py) — the corpus census at root, and its script.

Renders (`.dot` as the binary wrote it, `.svg` drawn by Graphviz with the layout the DOT suggests, `fdp` for a
knowledge base, else `dot`; a program view's digraphs split one file each, `NN-name`;
[`runs/draw-times.tsv`](runs/draw-times.tsv) the layout and seconds of each):
- [`renders/v7_unpack.kb.svg`](renders/v7_unpack.kb.svg) ([dot](renders/v7_unpack.kb.dot)) — root.
- [`renders/v7_unpack.program.01-kb.svg`](renders/v7_unpack.program.01-kb.svg), [`renders/v7_unpack.program.02-rule_unpack_lhs_rhs.svg`](renders/v7_unpack.program.02-rule_unpack_lhs_rhs.svg) (from [`v7_unpack.program.dot`](renders/v7_unpack.program.dot)).
- [`renders/v7_unpack.trace-1.svg`](renders/v7_unpack.trace-1.svg) ([dot](renders/v7_unpack.trace-1.dot)) — the trace's inline drawing.
- [`renders/v7_tag.kb.svg`](renders/v7_tag.kb.svg); `ein render program`'s three digraphs: [kb](renders/v7_tag.program.01-kb.svg), [rule](renders/v7_tag.program.02-rule_tag_lhs_rhs.svg), [query](renders/v7_tag.program.03-query.svg).
- [`renders/v7_blind.kb.svg`](renders/v7_blind.kb.svg), [`renders/v7_blind-registered.kb.svg`](renders/v7_blind-registered.kb.svg); [`renders/v7_blind.program.01-kb.svg`](renders/v7_blind.program.01-kb.svg), [`renders/v7_blind.program.02-rule_tag_lhs_rhs.svg`](renders/v7_blind.program.02-rule_tag_lhs_rhs.svg), and `v7_blind-registered`'s two ([kb](renders/v7_blind-registered.program.01-kb.svg), [rule](renders/v7_blind-registered.program.02-rule_tag_lhs_rhs.svg)).

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/08-V-7/rerun.sh
```

[`rerun.sh`](rerun.sh) runs every command above with `build/review-bin/ein-head-17085b27` (`solve`, `-p`,
`--json-summary`, `--trace`, `--events`, `saturate --dump`, `show hyps`, `test -v`, `render h-space`, `render kb`,
`render program`), writes the diffs, draws every `.dot` with
[`../../03-drawings/tools/draw.sh`](../../03-drawings/tools/draw.sh), rewrites the code excerpts and runs the corpus
census (`v7_census.py`, over every corpus program at root).

## 7. Found while building

- **The ledger says no answer is known to be wrong by it; the registry does move answers.** Not the witness's: the
  blind rung takes any name outside the registry for an object, so whichever option is taken, a program the blind
  rung searches can change R (§ 2.4, § 3): (a) and (b) both take `happy` out of `v7_blind.ein`'s objects (8 states →
  1), and (b)'s declaration of `=` adds 12 candidates to `v7_unpack.ein` ({∅} → 4 096 states). Today's answer on
  `v7_blind.ein` treats the relation `happy` as an object and is not wrong by any rule written down: the read-out's
  `relation names` row says so.
- **The constant-head case is the same today.** A relation a rule concludes with its name written,
  `:assert (happy ?x)`, is outside the registry too and is guessed about as an object (§ 2.4). V-7 as written is
  about the parameter alone; list 2 (the invariant's baseline) is the only one that tells the two apart.
- The ledger's witness (`ein saturate --dump`: `(= 7 (+ 3 4) :rule unpack)` under `;; Relations — auto-vivified (2)`
  listing `think` and `unpack`) reproduces exactly.
- **Re-taken with `ein-head-17085b27`** (every P1f.23 stage merged) against the first look's `ein-head-b5fb8d70`:
  no answer moved, no read-out, summary, H, test line or event; only timings. What moved: the drawings, in the
  canonical form (§ 2.1); `program_relations` no longer adds `is-a` (Q-M1f.436, S1f.23.4), which V-7 does not turn
  on, so the four lists disagree as before; the census reads 888 corpus programs where it read 894 (six retired
  with the native symmetry), with the same 0 and 29 (50 heads).
- **The drawing shows the disagreement**: in the KB view the box `=` is one atom under the activator's arrow `2` and
  the derived fact's arrow `1`, a relation by where the arrows meet it, whatever the registry says.
- **Q-M1f.447**, the user's own question for P1f.21 (whether the loader keeps its argument-count check once
  `relation` is a convention), bears on option (b): (b)'s fix is a declaration, which under P1f.21's fourth
  instruction is a form of the standard library's and not the kernel's (read, not run).
- The goal refusal's own line (*"a relation of the program is one it declares, states a fact of, or writes in a rule
  or an hrule"*) does not name the activator's case either, though `program_relations` takes it (§ 2.2): item
  `24-L-94` of this bundle shows it.

## 8. Your call

- **(a)** the registry takes a relation an activator names for a parameter that heads a conclusion (and decide
  whether a constant `:assert` head joins it too); moves R where the blind rung guessed about the name.
- **(b)** the loader refuses such an activator; the author declares the relation, which moves R where the blind rung
  then guesses facts of it.
- **(c)** a page says such a relation is declared; nothing moves, and the four lists keep disagreeing.
- **Default**: none taken; the ledger reads the user's third instruction (*any head is a relation*) as pointing at (a).
