# P1f.21 — the overview: what the phase would change, by part, and where things are held today

**Phase:** [P1f.21](README.md). Written 2026-11-19 on the user's word,
[the fifth instruction](README.md#the-fifth-instruction): *"make a distilled overview of proposed modification of
the main parts: language, kernel, stdlib; and write down it for review; also explain is now negated/refuted facts
are stored in kb under (not ...) context or they are stored somehow outside of the kb? the same about
hypotheses"*. **For review.** It is a reading of the phase's pages and of the engine at `e4a5f8fb`; it rules
nothing, builds nothing and runs no stage: the phase is not started without the user's word.

**Since it was written** (2026-11-19): the user ruled Q-M1f.399, and the rows it touches are brought to the ruling
in place, § 1's *kill*, § 3's `:ein` and § 5's order, and then Q-M1f.550, the key's word, `inconsistent`; S1f.21.6
and S1f.21.8 were started on the user's word, and S1f.21.10 unblocked.

**What it stands on.** [`modifications.md`](modifications.md), the 37 items, which this page distils and does not
repeat; [`open_facts.md`](open_facts.md) and [`design.md`](design.md); the code named in § 2; and the runs of
[review item 34](review/part-2/34-Q-M1f.549/README.md), today's engine on the programs kept there.

## 1. The phase in one sentence

**A rule's conclusion is either a fact, or a word addressed to the kernel; the phase makes the second kind one
notion, puts its words under one prefix, and hands every other name the kernel reads to the standard library.**

Today three kinds of rule speak to the kernel, each by its own road:

| the rule | its conclusion | the road today |
|---|---|---|
| a refutation | `(false)`, or a fact beside its `(not …)` | a held fact the kernel reads by its name, and a pair it finds with no rule (`contradiction.rs`) |
| an obligation | `(open ?R)`, its whole `:assert` | its own production of the grammar, kept out of the agenda, read by a pass over each quiescent state (`obligations::tally`) |
| a hypothesis rule | a candidate | its own declarator `hrule`, a clause of `solve`, read once on root's fixpoint, its conclusions kept in a list beside the knowledge base |

They differ in two things only: **when the kernel reads the conclusion, and whether a state may hold it.** The
second follows from one property, which [`design.md` § 2.4](design.md#24-the-verdicts-together-which-may-be-a-fact)
found for the verdicts and [item 34](review/part-2/34-Q-M1f.549/README.md#3-what-a-rule-can-read-of-them-rows-3-and-4)
finds again for the candidates: *a conclusion may be a held fact exactly when it stays true in every larger
state.*

| the word | says of a state | stays true above | so | read |
|---|---|---|---|---|
| *kill* | it is contradictory | yes | it could be a held fact, and is **ruled** a rule's key and no fact ([Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open)) | in the saturation, as the rule fires |
| *asked* | F is a candidate of the program | yes | a held fact, root's | once, at root's fixpoint |
| *owes* | a requirement has no witness | no: a larger state pays | never held; a pass counts it | on each quiescent state |
| *still open* | F is neither held nor denied | no: a larger state decides | never held; a reading | on each quiescent state |

*Stays true above* is said of a program the static check calls monotone, the only kind the engine claims an answer
of: there a larger commitment's state holds every fact of a smaller one's.

## 2. Where things are held today

The knowledge-base graph has every object once: an atom, or a fact, a tuple interned in the fact store
(`ein_core::FactId`). **A fact of the graph need not be held.** A knowledge base (`ein_core::Kb`) is the set of
facts held, a stack of append-only layers; a fork adds a layer and nothing is ever removed.

| what | where it is | held by a knowledge base | shown by |
|---|---|---|---|
| **a negated fact** | `(not F)` is a held fact of the relation `not`, of one argument. F is the node under its arrow, and is not held | `(not F)` yes; F no | `runs/negation.saturate-dump.txt` |
| **a refuted commitment** | nowhere in a knowledge base. The dead node's fork is dropped once its bits and its core are read. The death is kept in the search's memory (`known_dead.rs`) and in the node's record: `core`, `refuted by`, the read-out's $N$, `nogoods.json` | no | `runs/seats.dump-lattice/nodes.md` |
| **a candidate**, a hypothesis not committed | a node of the graph that no knowledge base holds and no held fact points at. $H$ is a list of them beside root (`region.rs`' `Axes::candidates`), built by the generator on root's fixpoint | no | `runs/seats.saturate-dump.txt` against `runs/seats.show-hyps.txt` |
| **a committed hypothesis** | an ordinary held fact of the node's fork, with the provenance `hypothesis` | yes, in that node and above | `runs/seats.state-01.saturate-dump.txt` |
| **a debt**, `(open R)` | never a fact. An obligation rule is in no agenda (`Program::obligations`); a pass counts its matches on each quiescent state, into the node's record | no | [`design.md` § 1.3](design.md#13-owed-and-discharged) |
| **a saved state** | the program and its `(commit …)`; no fact | | `runs/seats.dump-answer/models/01.ein` |

(`runs/` is [item 34](review/part-2/34-Q-M1f.549/README.md)'s, written by its `tools/run.sh` and ignored.)

**Negations: in the knowledge base, under `not`.** [`programs/negation.ein`](review/part-2/34-Q-M1f.549/programs/negation.ein)
writes `(not (p two))` and derives `(not (p one))`. `ein saturate --dump`:

```text
  facts by relation:
    denied-p                                     0       2  +    2
    not                                          1       2  +    1
    q                                            1       1
;; ── DERIVED (3 facts) ──
  (denied-p two :rule seen-denied)
  (denied-p one :rule seen-denied)
  (not (p one) :rule q-denies-p)
```

Two facts of `not`, none of `p`: the rule `seen-held`, whose premise is `(p ?x)`, derived nothing, and
`seen-denied`, whose premise is `(not (p ?x))`, fired twice. Beside the facts the layer keeps one derived index, the
ids of the facts some `(not …)` denies (`Layer::negated`), which makes the contradiction check two bit tests.

**So yes: a denied fact is hidden in the `not` context, as a fact Ann thinks is hidden in `think`**
([Q-M1f.394](../../../docs/decisions/language/Q-M1f.394-a-fact-is-in-the-global-context-in-a-facts.md)). **What is
not yet a convention is the kernel's own reading of it.** The code reads `not` by its name on 29 lines of 13 files
and `false` on 15 of 9 (the census of the kernel's names, taken for the question of the names; [`open_facts.md` § 8](open_facts.md#8-the-reserved-names-that-are-left-and-what-the-kernel-reads) has its commands),
for six decisions: a fact beside its negation is dead; the agenda fires first what leads there; *forbid*; the
static check; a candidate root denies is not in $H$; and a candidate a state denies is dead above it (licence
L8). That is group G.

**Refuted: outside every knowledge base.** A node's fixpoint is *root's fixpoint, the commitment's facts written
with hypothesis provenance, the saturation resumed … no lookahead, no forced positive, no kill cache, no writeback,
no learned clause* (`region.rs`, § What a node is). No `(not F)` is written for a candidate that dies alone;
`ProvKind::Rejected` is in the code and nothing but a test writes it. The one denial the search reads is a
`(not b)` the state's own rules derived, which is a held fact like any other.

**Hypotheses: outside every knowledge base, until committed.** On the seats program root holds twelve facts and no
fact of `seats`; `ein show hyps` lists the four candidates; the state `{(seats Ann C1), (seats Bob C2)}` holds
`(seats Ann C1 :hypothesis 0)` and `(seats Bob C2 :hypothesis 0)`.

So the asymmetry the instruction names is real: **a negation is a fact of the knowledge base under a context,
and a hypothesis is a tuple in a list the kernel keeps to itself.** [§ 4](#4-hypotheses-as-facts) is what closing
it takes.

## 3. The modifications, distilled

The state of a row: **ruled**, the user's word is given; **leaning** or **direction**, a page leans; **open**,
not ruled. The stage is where [§ 5](#5-the-stages) plans it.

### 3.1 The language

| word | today | after | state | stage |
|---|---|---|---|---|
| `false` | the kernel's: a held one kills | an ordinary proposition, held or under an arrow; one rule of the standard library goes from it to the key | ruled | S1f.21.10 |
| `not` | a context the kernel reads | a relation of one argument whose meaning is two rules: the contradiction, and the elimination of a double negation | ruled | S1f.21.8, S1f.21.11 |
| `relation` | a position word; the loader holds a fact to its declaration's count | a convention; any head is a relation (built, Q-M1f.522); the count check [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) | ruled; the check open | S1f.21.11 |
| `:ein inconsistent` | nothing | **a key of a rule**, beside `:assert`: its argument is a word addressed to the engine and no fact. A rule that states it and fires ends the saturation, and the state is dead. No prefix is reserved | **ruled** ([Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein)) | S1f.21.10 |
| `open` | the verdict, an obligation's whole `:assert`; refused anywhere else | beside the verdict, the candidates as facts a rule may conclude and match ([§ 4](#4-hypotheses-as-facts)) | open: [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules) | S1f.21.12 |
| `hrule`, `(solve :hrules …)` | a declarator and its clause | a rule that concludes a candidate | direction | S1f.21.12 |
| `:hypothesis-relations`, `:no-hypothesis`, `__closed__` | two lists and a reserved relation | rules and facts over what is proposed | direction | S1f.21.12 |
| `eq`, `neq` | computed | as they are | ruled | |
| `rule`, `macro`, `import`, `query`, `solve`, `config`, `trace`, `and`, `or`, `absent`, the claim's words | | as they are | | |

**Not proposed**: a rename of `false` (items 21, 22); a premise that is a variable for a whole fact
([Q-M1f.393](../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md)); a form that lifts
a rule into a context.

### 3.2 The kernel

| what it decides | reads today | after | stage |
|---|---|---|---|
| a state is dead | a held `(false)`; a fact beside its `(not …)`, with no rule | a rule that states `:ein inconsistent` fired, and nothing else | S1f.21.10 for `(false)`, S1f.21.11 for the pair |
| what the agenda fires first, *forbid*, the static check | the rules that conclude `(false)` or a `(not …)`, by those names | the rules that lead to a rule with the key | S1f.21.10, S1f.21.11 |
| what a state owes | each obligation's guard, by a pass; a count | the same pass; each debt carries its instance as a term | S1f.21.7 |
| $H$ | three rungs: the `hrule`s under `solve`'s clause; what root's obligations owe, read off their guards; the blind enumerator, over the declarations; then the two lists, `__closed__` and what root holds or denies | **the facts root holds under one relation, less what root has decided** | S1f.21.12 |
| the mechanisms' conflicts | `mechanisms.rs`, five mechanisms against one another | what is left of them once proposing is one form | S1f.21.12 |
| a fact's count of arguments | the declarations | [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) | S1f.21.11 |
| the split | the routes | the routes: the kernel's one primitive | |

**What no stage touches**: the knowledge base only grows, and a fork inherits; $R$, its definition and every
answer; the oracle; the routes and their licences; `absent`'s boundary; a rule's activator; `eq` and `neq`.

**What it then reads from a state**: one relation's extent at root; each obligation's guard; `absent`'s boundary;
a rule's activator. What kills a state is read from no fact: it is a rule's key.

### 3.3 The standard library

| what | today | after | state | stage |
|---|---|---|---|---|
| the meaning of `not` | the kernel's | the contradiction as rules, `clash-0` to `clash-6` with their activators, and the elimination of a double negation, one rule a count | designs that run; where they live open | S1f.21.8 |
| the module every program gets | none | holds the rule from `(false)` to the key, and later the meaning of `not`, with no import line, shown by `ein saturate --dump` as any rule is | a default of S1f.21.10's: `std.core`, always loaded | S1f.21.10, S1f.21.11 |
| the requirement, the payment, `discharged` | no rule: the pass | a module of three ordinary rules a program may import, beside the pass | a design that runs | S1f.21.8 |
| `std.algebra`'s and `std.slots`' four obligations | conclude `(open ?R)` | unchanged: the debt's term is the kernel's ([item 16](modifications.md#c-q-m1f390-a-and-b-the-verdict-stays-a-verdict)) | | |
| proposing | the kernel's rungs | the standard library's rules, where a rule can say it; the kernel's rungs write the same facts | direction | S1f.21.12 |

A rule added to `stdlib/` takes its program under `tests/stdlib/`, its region on a rules page and the manifest
with it, the standing rule.

### 3.4 The surfaces and the pages

The read-out's *derives (false)*, `-p`'s `(alive )` and the saturated dump's missing rules are fixed first and wait
on nothing (S1f.21.6). The pages of `docs/ein/` state the kernel as built, so they are written last (S1f.21.2).

## 4. Hypotheses as facts

The instruction: *"what if we would make hypotheses just ordinary facts hidden in (open ...) context? it is
different case because kernel does search and has to make commitment sets from them, but kernel anyway has access
to all facts … 1. search control from inside ein 2. implement search with rules inside one stage, have search
artefeacts seen by rules and infer facts from search. … we can establish the foundation for this in this phase or
can decline it for some pragmatic reason."*

**The answer, from five runs on today's engine** ([item 34](review/part-2/34-Q-M1f.549/README.md)):

1. **It works today, at no change of answer, in one reading.** *zebra2* with its candidates concluded by an
   ordinary rule as held facts, and one generic hrule that supposes what root holds under that relation, prints the
   same read-out byte for byte. The seats program the same, `ein oracle` included.
2. **The reading is *asked*, not *still open*.** Root holds 125 such facts on *zebra2* and $H$ is 56 of them. A rule
   that reads what is asked keeps the program monotone. A rule that stores *asked and undecided* as a fact gets the
   program flagged, the cube for a route, and a stale fact in 15 of 16 nodes.
3. **So nothing has to be removed.** A candidate, like a negation, is a held fact that points at a fact not held.
   *Held*, *denied* and *asked* are three independent contexts of one fact, and *open* is the reading *asked,
   neither held nor denied*, which the kernel makes per state as it reads a contradiction per state. The move of
   [`open_facts.md` § 5](open_facts.md#5-the-move-open-held-not), its part of a node no fork inherits and its
   stratum, are what the other reading costs, and are not needed for this one.
4. **The search written as rules runs, one absence deep.** A two-candidate lattice in one saturation gives the
   oracle's $R$ and dead set as facts. The first rule that infers from them over a second absence answers wrong:
   three strata printed, not followed ([V-8](findings.md#v-8--the-search-written-as-rules-answers-wrong-at-its-third-stratum)).

**Recommended: establish the foundation, and decline the rest for now.**

| | what | why |
|---|---|---|
| **establish** | the candidates as held facts at root, in the reading *asked*: [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules) (b) | it is the same step as `not`: one more context, and the kernel reads its extent once where it reads a declarator, a clause, two lists and a reserved relation today. No new mechanism; the invariance is taken already |
| **decline, for now** | the move, and rules in a stratum of their own: (c) | a part of a node, a stratum and a class of rule, for a gain (b) shows is not needed |
| **decline, for now** | 1, search control from inside Ein | the tree already takes the owed instance with the fewest alternatives, which is *forced* and *order*, and a debt with none left has no child, which is *hopeless*; no program is known to want another order; and it can be added on (b)'s facts later |
| **decline, until M1n** | 2, search artefacts seen by rules; the search as rules: (d) | the second absence is wrong today, M1n's ground; $2^{|H|}$ worlds against 7 saturations on *zebra2*; no form lifts a rule into a context; a fact written back from the search changes the states, so $R$ |
| **keep of 2, at no cost** | $R$ stated with no search on the reasoning pages; the two-candidate lattice as an example | the first is item 13; the second S1f.21.13 |

## 5. The stages

Each group of [`modifications.md`](modifications.md#the-list-at-a-glance) is a stage, planned 2026-11-19 on the
user's word. **In two parts**, on the user's word of the same day (*"leave open facts and hypotheses to the second
part of the phase and finishe everything else before"*): first the stages that do not touch `open` or the
hypotheses, then those that do. Within a part the rows are in the order the stages run.

**Part 1 — the verdict *dead*.**

| | stage | group | items | what | waits on |
|---|---|---|---|---|---|
| 1 | [S1f.21.6](s1f.21.6_surfaces.md) | A | 1 to 5 | the surfaces: a printer, the dump's rule sections, a wording, the plan's own numbers | **done** |
| 2 | [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md) | D | 17 to 19 | modules of the standard library, additive | **done**; `std.negation` and `std.contradiction` merged, `std.debts` held for part 2 |
| 3 | [S1f.21.10](s1f.21.10_the_kernels_prefix.md) | F | 21 to 25 | the engine's key, `:ein inconsistent`; `(false)` out of the kernel's reading; no prefix, no rename | **done** |
| | [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md) | | | *forbid* becomes a warning: a non-monotone program loads, warned, nothing guaranteed (Q-M1f.564, the user's ruling) | nothing; before S1f.21.11 |
| 4 | [S1f.21.11](s1f.21.11_kill_and_three_conventions.md) | G | 26 to 29 | `not` and `relation` as conventions: the contradiction as rules | S1f.21.10, S1f.21.8; V-1 and a choice with no default; Q-M1f.447 |
| 5 | [S1f.21.2](s1f.21.2_the_pages.md) | B | 6, 7, 9, 12, half of 10 | the pages of the first part | every build of the part |

**Part 2 — `open`, what a state owes, and the hypotheses.**

| | stage | group | items | what | waits on |
|---|---|---|---|---|---|
| 6 | [S1f.21.7](s1f.21.7_the_debts_term.md) | C | 14 to 16 | a debt's instance as a term | Q-M1f.390 |
| 7 | [S1f.21.12](s1f.21.12_hypotheses_as_facts.md) | H | 30 to 35 | the candidates as facts | Q-M1f.549; S1f.21.7 |
| 8 | [S1f.21.13](s1f.21.13_the_search_and_rules.md) | I | 36, 37 | the search and rules: what is kept, and what is declined | Q-M1f.549; S1f.21.12 |
| | [S1f.21.9](s1f.21.9_the_requirement_and_the_payment_as_facts.md) | E | 20 | the requirement and the payment as the kernel's facts | **run only on a ruling for Q-M1f.390 (c)** |
| 9 | [S1f.21.14](s1f.21.14_the_pages_of_open.md) | B | 8, 11, 13, half of 10 | the pages of the second part | every build of the part |
| 10 | S1f.21.5 | | | the close | |

## 6. What is the user's to rule

| question | what it decides | the default or the recommendation |
|---|---|---|
| [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules), new | whether the candidates are held, in which reading, and how far the search goes into rules | recommended (b): held as *asked*; (c), (d) and both uses declined for now |
| [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact) | what an obligation emits | default (b): the verdict stays a verdict and carries its instance. It fits Q-M1f.549 (b): the debt is counted, the candidates are held |
| [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open) | the names | **ruled 2026-11-19**: `false` kept, a convention of the standard library; no prefix; what ends a state is a rule's key, `:ein` |
| [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein) | the key's word | **ruled 2026-11-19**: `inconsistent` |
| [Q-M1f.447](../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) | the argument-count check | no default |
| [V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none) | *forbid* and the contradiction as rules | open; its ground is M1n's |
| the stages' own | where the new rules live and the module every program gets; whether `hrule` stays as a spelling | each a question of its stage, with a default |

Q-M1f.390 and Q-M1f.549 are the second part's and wait with it; the first part asks only what S1f.21.11's examples
show. The answers' files are [`review/part-1/ANSWERS.md`](review/part-1/ANSWERS.md) and
[`review/part-2/ANSWERS.md`](review/part-2/ANSWERS.md).
