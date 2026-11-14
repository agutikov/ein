# M12 — Rules as data

**Status:** planned 2026-11-12, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started as a milestone; its first phase, the census of rules, was run the same day and its results are
[`rule_census.md`](rule_census.md). It carries no estimate (the user's ruling of 2026-11-08). **This README
is the whole plan today:** it gathers what the repository already says about rules as data, names the one
phase the instruction defines, and leaves every later phase to the user.
**Depends on:** the corpus of theories that exists when it starts. The instruction expects one of *"hundreds
and thousands of rules"*, which the calculator ([P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)),
school algebra and geometry ([P1g.10](../m1g_logic/p1g.10_school_algebra/README.md) and [P1g.11](../m1g_logic/p1g.11_school_geometry/README.md), M1f's P1f.8 until 2026-11-13),
[M1g](../m1g_logic/README.md) and [M1h](../m1h_beyond_word_problems/README.md) are writing.
**Where it sits against the other milestones is not ruled.** The number is the user's.
**Blocks:** nothing.

## The instruction

The user, 2026-11-12, in full:

> new M12 pulls ideas and followups about rules as data; till the start of it we would have quite big corpus
> of different theories with hudreds and thouthands of rules. Let's analyse them first. Census of the rules
> shapes - name agnostic, only considering equal names when they reused - so analyze only match and assert
> graphs. count and census all of them. Then connections between rules - first list all of possible
> connections: use same name, use same name in same place, assert->match a relation connection, etc. No
> specific goals right now, want to see analyze results.

And the same day, on the census's script:

> also utils/rule_census.py can group rule shapes by number of relations, by number of relations arguments,
> and so on propose ways to group shapes

Three things are in it. The milestone *pulls* the ideas and followups about rules as data, which
[§ What it pulls together](#what-it-pulls-together) does. It starts from an analysis of the rules the
repository has, which is [P12.1](#p121--the-census-of-rules). And it has *no specific goals right now*: what
the milestone builds is decided after the results are read, by the user.

## What this is not

- **Not a design.** No syntax for a rule that reads a rule, no engine change and no stdlib module is
  proposed here. The sources below each hold proposals of their own, and none is taken up by this page.
- **Not a set of goals.** The census's page ends in observations and makes no recommendation, as the
  instruction asks.
- **Not a replacement for its sources.** Each followup and idea stays where it is, with a line pointing
  here. Nothing was moved or rewritten.

## What it pulls together

Every place the repository speaks of rules as data, a rule schema, a rule about rules or a program that
describes itself (`grep -rnEi "rules[- ]as[- ]data|rule schema|meta-rule|self-describ" plans docs/kernel`,
2026-11-12, read through). Each row says what the source asks, in its own framing.

### The followups and ideas that are about it

| source | what it asks |
|---|---|
| [F5, *Operate IR rules as data*](../followups/f5_rules_as_data.md) | The followup this milestone is named after: *"have the engine itself manipulate rules as graph data during reasoning, not just as static configuration loaded once at startup."* Its four shapes are rules that rewrite rules (fusion, specialisation, abstraction: *"a concrete rule generalised by replacing a named relation with a variable"*), inducing a rule from facts, rules over the inference trace, and conditional activation. It lists what the engine lacks (an `:assert` that targets a rule, provenance for a rule's mutation, a termination story) and carries a sub-track, **kernel minimisation**: *"Which inference features make sense to express in ein-lang instead of kernel code?"* |
| [`08_self_describing.md`](../../docs/kernel/ir/03-ein-lang/08_self_describing.md), § 5 *The real frontier: rules as data* | The kernel page that says where the boundary is today: *"L0, L1, L2 reify; L3 does not."* A rule can match a fact, a relation's signature and a property, *"but not another rule: there is no `(rule ?N …)` node a `:match` can bind, and a rule may not `:assert` a new `(rule …)` or `(relation …)`."* Its § 4 writes the abstract syntax of a rule as Ein relations, descriptively, *"precisely enough for F5 to target"*. [`05_four_level_kb.md`](../../docs/kernel/ir/01-ein-graph/05_four_level_kb.md) and [`03_ein_model.md`](../../docs/kernel/ir/01-ein-graph/03_ein_model.md) state the same frontier from the graph's side |
| [Idea 10, *Generic self-modification*](../ideas/10-generic-self-modification.md) | The umbrella: three rungs, the grammar ([F2](../followups/f2_self_modifying_language.md)), the rules (F5), the harness ([F6](../followups/f6_modify_own_harness.md)). Rules as data is **rung 2**, *"the semantic / declarative rung. It changes what the system can prove."* |
| [Idea 01, *Self-modifying constraint language*](../ideas/01-self-modifying-constraint-language.md) | The user's oldest form of the thought, at the level of the grammar: a model that is given its own output grammar and a language for editing it, in a loop. *"Dont need to control syntax because it will be done with gbnf. But semantics - something like compileability/interpretability, how to control it?"* Rules as data is the same loop one level down, where the thing edited is a rule and the control is the engine |
| [Idea 06, *Completeness of inference rules*](../ideas/06-inference-rules-completeness.md) | *"It's unclear whether these two are enough. If they're enough for this task, what other rules might exist and for which tasks?"* A question about the set of rule shapes, which a census can be read against: the taxonomy that page sketches is by hand, and the census counts what was written |
| [F7, *Rule taxonomy + rule induction*](../followups/f7_rule_induction.md) | A classification of rules *"by their structural shape"* into four classes by what is a parameter and what is a literal, and its sub-track A, **generalisation automation**: lifting a rule with literal relations into one with a relation parameter and an activator, mechanically. Its sub-track D is companion-rule synthesis. The census's levels and its families that differ in one relation are the measured form of that taxonomy |
| [F8, *FCA / RCA, ODIS, TPTP*](../followups/f8_FCA_RCA_odis_tptp/ideas.md) | The raw notes that define the measure **C(n)**, the number of distinct rule schemas after n tasks, and the *compression ratio* of domain rules to generic schemas (*one rule schema + three facts*). [M2's S2.6.4](../m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md) plans to take the first point of that curve by machine. The census counts distinct shapes over the corpus as it stands, which is the same count taken over hand-written programs |
| [F4, *Cross-cutting*](../followups/f4_cross_cutting.md), Q34, Q36, Q37 | The cartesian product of relation properties, rule polymorphism over relations, and induction of rules from facts: the lineage idea 10 gives rung 2 |
| [F12, *Rules and relations*](../followups/f12_rules_and_relations/ideas2.md) | Raw notes; among them an order on relations and a *meta-rule* that derives an inclusion between two instances of a monotone rule pattern without enumerating facts |
| [F1b, *Logical formulation*](../followups/f1b_logical_formulation.md) (promoted to M1g) | Notes that rules on rules would change the logical fragment Ein's rules are |

### Where another plan needs it

| source | what it asks |
|---|---|
| [M1g](../m1g_logic/README.md), candidate C-10 and [S1g.6.5](../m1g_logic/p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md) | *"A rule as a value. An Ein rule cannot be named by, matched by, or concluded by a rule."* Logic meets that in a derived rule, in a rule that holds only inside a sub-proof ([S1g.5.3](../m1g_logic/p1g.5_peano_and_induction/s1g.5.3_induction.md)'s scoped induction hypothesis, *"the cleanest witness for rules as data"* that milestone has) and in a statement about Ein's own derivability ([S1g.5.4](../m1g_logic/p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md)). S1g.6.5 is *a design and a price, for the user; a build only on their word* |
| [F26, *Consistency and completeness*](../followups/f26_consistency_and_completeness.md), § Does Gödel's theorem say anything about an Ein program? | *"Self-reference needs rules as data. […] Ein programs cannot name their rules as values today"*; and induction *as a rule schema* is one of the ways the calculator could say more than ground equations |
| [F24, *Written provenance*](../followups/f24_written_provenance.md) | A fact may be written with `:rule R`, which is the one place a program names a rule as a value today, and the loader checks none of it. *What a written provenance may claim* is the user's question there; it is rules as data seen from a fact |
| [P1g.10's S1g.10.7, *Compound numbers without a hardcoded representation*](../m1g_logic/p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md) | The user: *"investigate how to make compound numbers without hardcoded representation so can switch between binary and decimal, and maybe later use any new natural numbers there, or even not numbers..."*, and *"So we potentially can import std.math.rat and std.math.binary-arith and then connect them with single fact?"* A theory whose relations are parameters is a set of rules treated as one schema |
| [P1f.20's S1f.20.4](../m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.4_reasoning.md) | The documentation tree planned there has a `rule-evolution/` part: `rules-as-data.md`, `analysis-of-rules.md` and `transformations.md`, the last *a stub with its question in it* |
| [Idea 09, *Puzzles beyond Zebra*](../ideas/09-puzzles-beyond-zebra.md) | The Tower of Hanoi as *"a recursive structure that should be expressible as a rule schema"*; the liar's sentence as the puzzle form of self-reference |

### What the engine and the stdlib already have of it

| what | where | what it is |
|---|---|---|
| A rule with parameters, and its activator | [`01_grammar.md` § Rule](../../docs/kernel/ir/03-ein-lang/01_grammar.md), `(rule symmetric (?R) …)` and the fact `(symmetric next-to)` | The one mechanism by which a fact switches a rule on and says which relations it is about. A rule may *conclude* an activator, so a rule can switch a rule on: [`stdlib/algebra.ein`](../../stdlib/algebra.ein)'s lemmas and the setup fan-outs of `std.slots` do. [`08_self_describing.md`](../../docs/kernel/ir/03-ein-lang/08_self_describing.md) § 3 calls the property algebra *Ein features defined in Ein and reasoned about by Ein* |
| A closing rule that names every rule of its module | [`stdlib/math/decimal-arith.ein`](../../stdlib/math/decimal-arith.ein)'s `decimal-arith`, `(arith-rule div-try-3)` | Rule names written as constants, so that an import of one name brings the module. The names are data to the import's closure and to nothing else |
| *One rule written nine times* | `div-try-1` to `div-try-9` of `std.math.decimal-arith` ([`stdlib/README.md`](../../stdlib/README.md), [S1f.6.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md)) | *"a rule cannot pick the J-th multiple by a digit without joining on a number"*, so the rule is written out per digit and a test holds the nine alike |
| Modules written by a script | `stdlib/math/num9.ein` (`utils/gen_num9.py`), `stdlib/math/digits.ein`, `examples/binary_arithmetic/bin.ein` (generated from `stdlib/math/decimal-arith.ein`) | Rules as data outside the language: a Python program holds the schema and prints the instances |
| The rule dependency graph and its strata | `ein_infer::strata`, the `;; Strata` line of `ein saturate --dump` (M1f S1f.5.15) | The engine's own analysis of its rules, over the plans a run compiled. The census sets its static graph beside it |
| Macros | `std.macro`'s `forall` and `unknown` | A premise schema expanded at load |

## P12.1 — The census of rules

**Status:** run 2026-11-12. **Results: [`rule_census.md`](rule_census.md).** The instrument is
[`utils/rule_census.py`](../../utils/rule_census.py), which reads every tracked `.ein` under `stdlib/`,
`examples/` and `tests/` as text and runs no engine but for one comparison.

What the instruction asks, and where the page answers it:

| asked | on the page |
|---|---|
| *Census of the rules shapes - name agnostic, only considering equal names when they reused* | § A: a rule's `:match` and `:assert` with every name replaced by a token of its kind, equal names equal tokens, at four levels from the exact shape to counts alone, with the top shapes, the distribution, the two clauses apart, and families of rules that differ in one constant or one relation |
| *propose ways to group shapes* | § Ways to group shapes: a catalogue of grouping keys, the census under each, cross tables, and which grouping refines which |
| *first list all of possible connections* | § B: a catalogue of kinds of connection, each defined, then counted in three scopes |
| *assert->match a relation connection, etc.* | kinds 3 to 6 and the digraph, its components, cycles and longest paths, with the engine's strata beside it |
| *want to see analyze results* | § What stands out: observations, and no recommendation |

**Record, 2026-11-12.** The plan, the script and the page are commit `4f7d2bbb`, made ahead of the gate
because another stage shared the working copy; `./run_tests.sh` was then run on that commit as committed,
in a detached worktree, and is green in every step (`build/gate-m12-census.log`, exit 0). The script was
run twice, the second time under another hash seed, and wrote the same files. No engine source and no
`.ein` file changed.

The census is of the corpus on the day it was taken. The instruction expects a larger one by the time the
milestone starts, and the script re-takes every number: `python3 utils/rule_census.py --page
plans/m12_rules_as_data/rule_census.md` rewrites the page's tables.

## Later phases

**Not defined.** The user said *"No specific goals right now, want to see analyze results"*: what M12
builds, in what order, and which of the sources above it takes up is the user's to say once the results
are read. Nothing is planned here ahead of that.

## Open questions

1. **What the milestone builds.** See above.
2. **Where it sits** against M1g, whose S1g.6.5 prices rules as data for logic, and against M2, whose
   S2.6.4 counts rule schemas a model writes.
3. **Which reading of a shape is the one to keep.** The census reports several levels and several
   groupings side by side and chooses none.
