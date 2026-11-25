# M12 — Rules as data

**Status:** planned 2026-11-12, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started as a milestone; its first phase, the census of rules, was run the same day and its results are
[`rule_census.md`](rule_census.md). It carries no estimate (the user's ruling of 2026-11-08). **This README
is the whole plan today:** it gathers what the repository already says about rules as data, names the one
phase the instruction defines, and leaves every later phase to the user. **Since 2026-11-15 it also owns three
findings of M1f's calculator phase** ([§ Handed over by M1f](#handed-over-by-m1f-2026-11-15)).
**Depends on:** the corpus of theories that exists when it starts. The instruction expects one of *"hundreds
and thousands of rules"*, which the calculator (P1f.6),
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

And on 2026-11-15, at the close of M1f's calculator phase:

> m12 one more note: most of hand-written rules are one fact match and one fact assert, so the logic of the
> theory is in the rules connections - one rule activates another either directly or via asserted relation
> fact, draw diagrams of rules as objects and connections between them as relations for theories

The first instruction has three things in it. The milestone *pulls* the ideas and followups about rules as data, which
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
| [Rules as data](../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md), § 4 *Where it stops: a rule is not data* | The page that says where the boundary is today: *"Atoms, facts and relations reify; rules do not."* A pattern can match a fact, a declaration and a property, *"but not a rule"*, and *"no conclusion declares a relation or a rule"*. The descriptive abstract syntax of a rule as Ein relations, written *"precisely enough for F5 to target"*, was the kernel page's § 4 and is in git (`git log -- docs/kernel/ir/03-ein-lang/08_self_describing.md`). [`02-rewriting-rules.md` § The four levels](../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md#the-four-levels-a-knowledge-base-describes-itself-in) and [`04-ontology.md`](../../docs/ein/user/03-syntax-and-semantics/04-ontology.md) state the same frontier from the graph's side |
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
| P1f.20, M1f's documentation tree (done 2026-11-17) | The reasoning has a part on the rule set as a thing to reason about, [`03-rule-evolution/`](../../docs/ein/reasoning/03-rule-evolution/): [`01-rules-as-data.md`](../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md), what of Ein describes itself in Ein and where that stops; [`02-analysis-of-rules.md`](../../docs/ein/reasoning/03-rule-evolution/02-analysis-of-rules.md), the dependency graph and the strata; and [`03-transformations.md`](../../docs/ein/reasoning/03-rule-evolution/03-transformations.md), which was planned as *a stub with its question in it* and states what preserves the answer as propositions |
| [Idea 09, *Puzzles beyond Zebra*](../ideas/09-puzzles-beyond-zebra.md) | The Tower of Hanoi as *"a recursive structure that should be expressible as a rule schema"*; the liar's sentence as the puzzle form of self-reference |

### What the engine and the stdlib already have of it

| what | where | what it is |
|---|---|---|
| A rule with parameters, and its activator | [`02-syntax.md` § Rule](../../docs/ein/user/04-reference/02-syntax.md), `(rule symmetric (?R) …)` and the fact `(symmetric next-to)` | The one mechanism by which a fact switches a rule on and says which relations it is about. A rule may *conclude* an activator, so a rule can switch a rule on: [`stdlib/algebra.ein`](../../stdlib/algebra.ein)'s lemmas and the setup fan-outs of `std.slots` do. [Rules as data](../../docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md) § 3 calls the property algebra *Ein features defined in Ein and reasoned about by Ein* |
| A closing rule that names every rule of its module | [`stdlib/math/decimal-arith.ein`](../../stdlib/math/decimal-arith.ein)'s `decimal-arith`, `(arith-rule div-try-3)` | Rule names written as constants, so that an import of one name brings the module. The names are data to the import's closure and to nothing else |
| *One rule written nine times* | `div-try-1` to `div-try-9` of `std.math.decimal-arith` ([`stdlib/README.md`](../../stdlib/README.md), S1f.6.6) | *"a rule cannot pick the J-th multiple by a digit without joining on a number"*, so the rule is written out per digit and a test holds the nine alike |
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

**Not defined but for one.** The user said *"No specific goals right now, want to see analyze results"*: what
M12 builds, in what order, and which of the sources above it takes up is the user's to say once the results
are read. The later phases the user has named are [P12.2](#p122--the-theories-drawn-as-graphs-of-rules), by
the note of 2026-11-15, and [P12.3](#p123--the-structural-rules-of-a-pattern-enumerated), by the note of
2026-11-18; nothing else is planned here ahead of them.

### P12.2 — The theories drawn as graphs of rules

**Status:** named by the user's note of 2026-11-15 ([§ The instruction](#the-instruction)); not planned in stages,
not started. **What the note asks:** for each theory, a stdlib module or a program with what it imports, a
diagram in which **a rule is a node** and **a connection between two rules is an edge, labelled by the relation
that makes it**. Rule A connects to rule B **via an asserted relation fact** when A's `:assert` concludes a fact
of a relation that B's `:match` reads, and **directly** when A concludes B's activator, as a closing rule and
the `positional` macro's call do. The note's premise is that the theory's logic lives in those connections,
because most hand-written rules match one fact and assert one: on the census of 2026-11-12, 898 rules of the
corpus are *1 × 1* ([`rule_census.md` § By premises × conclusions](rule_census.md#by-premises--conclusions)),
the largest group by far, and the calculator has added rules since. The phase re-takes that count first.

**What exists to start from:**
- **One rule at a time:** `ein render rules` draws a rule's match → assert shape (`ein-render/src/rules.rs`).
- **The connections, counted but not drawn per rule:** `utils/rule_census.py`'s § B counts the assert → match
  connections between rules ([§ 10, the digraph](rule_census.md#10-the-assert--match-digraph): its components,
  cycles and longest paths), and draws the graph only at the level of modules.
- **The engine's own dependency graph:** `ein_infer::strata`, over the plans a run compiled, printed as the
  `;; Strata` line of `ein saturate --dump`.

### P12.3 — The structural rules of a pattern, enumerated

**Status:** named by the user's note of 2026-11-18; not planned in stages, not started. The user, verbatim:

> plans/m12_rules_as_data - add one more goal - enumerate all possible rules for some subset of match patterns,
> e.g. :match (?R ?a ?b) -> can be either symmetric,asymmetric,antisymmetric, what else? review this way only pure
> structural rules, without constnats, becuase with constants there could be infinite number of rules with
> different constants

**What it asks.** For a small set of `:match` patterns, every rule that can be written over them **with no constant**:
every name a variable, the relation too, so the rule is about the shape of a relation and holds of any. With no
constant the space is finite: a conclusion may use only the variables the match binds (Ein's rule), so for a fixed
match there are finitely many literals to conclude, finitely many guards (`eq`, `neq` between its variables) and
`(false)`. Each rule is then classified: the same rule up to renaming its variables; a tautology (it concludes what
it matched) or a contradiction (it kills every relation that has a fact); **a property with a name** (the
standard library's `std.algebra`, and the literature's list of properties of relations); or one with no name; and
**which rules are equivalent or imply one another**, which a brute force decides, each rule run on every relation
over a small domain.

**The first pattern, `:match (?R ?a ?b)`, worked by hand** (the phase computes it, and every count):

| conclusion | guard | the property | note |
|---|---|---|---|
| `(?R ?b ?a)` | — | **symmetric** | |
| `(not (?R ?b ?a))` | — | **asymmetric** | implies irreflexive |
| `(not (?R ?b ?a))` | `(neq ?a ?b)` | **antisymmetric** | one premise and a guard; the two-premise form `(?R ?a ?b) (?R ?b ?a) (neq ?a ?b)` ⟹ `(false)` is the same rule |
| `(false)` | `(eq ?a ?b)` | **irreflexive** | |
| `(false)` | `(neq ?a ?b)` | **coreflexive**, R ⊆ identity | |
| `(?R ?a ?a)` | — | **left quasi-reflexive**: reflexive on R's domain | |
| `(?R ?b ?b)` | — | **right quasi-reflexive**: reflexive on R's range | |
| both of the two above | — | **quasi-reflexive**: reflexive on R's field | a conjunction is two rules |
| `(not (?R ?a ?a))` | — | irreflexive again | an equivalence the brute force finds: its instance at `a = b` kills every loop |
| `(?R ?b ?a)` | `(neq ?a ?b)` | symmetric again | the diagonal is its own converse |
| `(false)`, or `(not (?R ?a ?b))` | — | **empty**: no fact of R | |
| `(?R ?a ?b)` | — | nothing: a tautology | |

**Two premises** give the rest of the usual list: `(?R ?a ?b) (?R ?b ?c)` ⟹ `(?R ?a ?c)` **transitive**, ⟹
`(not (?R ?a ?c))` **antitransitive**; `(?R ?a ?b) (?R ?a ?c)` ⟹ `(?R ?b ?c)` **right Euclidean**, with
`(neq ?b ?c)` ⟹ `(false)` **functional**; `(?R ?a ?c) (?R ?b ?c) (neq ?a ?b)` ⟹ `(false)` **injective**;
`(?R ?a ?b) (?R ?c ?b)` ⟹ `(?R ?a ?c)` left Euclidean; and with a second relation variable `?S`, inclusion, the
converse, composition. **What no such rule states** is a property with *every* or *some* in it: **reflexive**
(every object, which needs the objects), **total** or **connex** (*R a b or R b a*, a disjunction), **serial** and
**dense** (*some* b): those are an obligation's (`total-owed`) or a hypothesis's, and the census says so of each.

**What the phase would do**: fix the patterns (one premise of one relation, two premises, two relations; the
counts first, to keep it small), generate every rule up to renaming, run each on every relation over a domain of
two, three and four objects, group the rules by what they do there (kill, derive), name each group by
`std.algebra`'s rule or the literature's property, and list the groups with no name. Then: which of
`std.algebra`'s rules is a group of this list, and which group the library lacks. **It reads with**
[F4](../followups/f4_cross_cutting.md)'s Q34 (the cartesian product of relation properties),
[F7](../followups/f7_rule_induction.md)'s taxonomy by shape, [F8](../followups/f8_FCA_RCA_odis_tptp/ideas.md)'s
measure C(n), and [idea 06](../ideas/06-inference-rules-completeness.md) (*are these rules enough?*).

## Handed over by M1f, 2026-11-15

Three findings of M1f's P1f.6, moved here on the
user's ruling of 2026-11-15 at that phase's close (*"all as you recommended"*,
`close.md` § The user's rulings).
Each is a reading the loader makes, or does not make, of what a relation parameter can hold, which is a rule read as
data. None is a wrong answer: E-35's half loads a fact no check holds to its relation's declared count, and E-55 and
E-69 refuse at load a program in which S1f.5.18's check reads a rule's name, written as an argument, as a target a
derived activator *could* bind, where nothing does (Q-M1f.41, Q-M1f.42). Their rows in
`findings.md` link here. Nothing is planned for
them ahead of [§ Later phases](#later-phases).

| id | what | source | re-taken |
|---|---|---|---|
| E-35, its second half | **a rule derives a fact of a declared relation at another count through a parameter.** `(rule widen (?R) :match (?R ?a ?b) :assert (?R ?a ?b ?a))` with `(widen likes)` derives `(likes Ann Tea Ann)` beside `(relation likes Person Drink)`: S1f.6.23's count check reads written forms, and a relation written as a variable is not read ([Q-M1f.202](../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md), defaulted; what it decided is Q-M1f.447's file since M1f's close). Holding it needs the loader to read what a relation parameter can hold, as S1f.5.18's check does, or the firing to refuse the fact. Its first half, a kernel marker's fact at another count, went to [F23's D-16](../followups/f23_diagnostics_in_eins_words.md#the-items) | S1f.6.23's T1 | live: `build/s1f.6.23/p/param3.ein` under `ein saturate --dump` holds `(likes Ann Tea Ann :rule widen)`, exit 0 (the gate binary of `7c3c2878`, 2026-11-15) |
| E-55 | **a generic rule of a module that comes whole makes S1f.5.18's check refuse a program that writes one of the module's rule names as an argument.** `std.algebra`'s `imply1 (?R1 ?R2)`, `(?R1 ?a) ⟹ (?R2 ?a)`, is a conclusion whose `?a` may hold a fact, so a program that writes `injective` as an argument is refused at load, though nothing activates `imply1`: *rule 'std.algebra.imply1': its :assert writes (?R2 ?a), whose ?a is rule 'injective''s parameter ?R, which names a relation, and (?R1 ?a) can bind ?a to a fact, as an argument of the relation ?R1 names — a relation is named by a symbol, not a fact*. It loaded until S1f.6.35, whose selection left `imply1` out; a whole import refused it before and after, so the import invariance holds, and no tracked program reaches it | S1f.6.35's T2; `ein-cli/tests/solve_lists.rs`' fixture `an-imported-rules-conclusion`, whose list names `functional` alone since | live: `(import std.algebra :symbols (bijective-properties functional injective))`, `(relation r A B …)`, `(is-a a A) (is-a b B)`, `(bijective r)`, `(solve :no-hypothesis (functional injective is-a))`: `kb load error`, the line above, exit 1 (2026-11-15) |
| E-69 | **a relation-parameterised rule that a closing rule names makes every program that also imports `std.algebra` a load error.** The same reading as E-55, of a closing rule's `(arith-rule walk-1)`: a name `std.algebra`'s `imply1` could bind to `?R2`, so its conclusion `(?R2 ?a)` could activate `walk-1` with a derived fact | S1f.6.38's T1, on its prototypes `stdlib-w1` and `-w2` (`build/s1f.6.38/`) | no tracked program: no shipped closing rule names a parameterised rule, S1f.6.16's build connecting `std.math.positional` by activators; re-taken at P1f.6's close, `examples/beyond_arith/equation_two_unknowns.ein`, `std.algebra` beside the calculator, loads and holds its claim (`ein test`, 2026-11-15) |

## Open questions

1. **What the milestone builds.** See above.
2. **Where it sits** against M1g, whose S1g.6.5 prices rules as data for logic, and against M2, whose
   S2.6.4 counts rule schemas a model writes.
3. **Which reading of a shape is the one to keep.** The census reports several levels and several
   groupings side by side and chooses none.
