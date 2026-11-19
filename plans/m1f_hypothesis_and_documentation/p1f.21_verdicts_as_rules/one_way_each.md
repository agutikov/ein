# P1f.21 — hypothesis rules and obligations: are they one thing, and one way each to say *must* and *may*

**Phase:** [P1f.21](README.md), part 1. Written 2026-11-19 on the user's word,
[the sixth instruction](README.md#the-sixth-instruction): *"analyze semantic of hrules and obligation rules, I think
they are 100% equivalent, so we can remove hrules completely from kernel, syntax, docs, everything; And replace
them with obligation rules. Can we? Then we still have 2 approaches to hypotheses generation - positive and
negative, the second one is blind generator with filters. Let's leave only one valid way for each. Obligations is
positive way, what would be the negative (filtering) one?"*

**An analysis. It rules nothing and builds nothing.** The question it ends in is
[Q-M1f.561](../open_questions.md#q-m1f561--one-way-to-say-must-and-one-to-say-may-hypothesis-rules-removed),
[review item 43](review/part-1/43-Q-M1f.561/README.md).

**The runs** are the gate build of master at `ca98dec2` (`build/review-bin/ein-head-ca98dec2`, with that commit's
`stdlib/`) on that commit's corpus. The three censuses are scripts kept beside this page, with the tables they
wrote: [`one_way_each/`](one_way_each/).

## The answer, short

1. **They are not equivalent, and the difference is one thing.** As a way to build $H$ an obligation *is* a
   hypothesis rule: what would pay its debts is exactly what the hypothesis rule *trigger, no witness yet, domain ⟹
   witness* proposes. But an obligation also **requires**: a state that holds none of a debt's payments is not in
   $R$. A hypothesis rule requires nothing. **An obligation is a hypothesis rule plus *at least one of these***; a
   hypothesis rule says *may*, an obligation says *must*.
2. **So hypothesis rules can go, but not into obligations alone.** *Must* is the obligation's. *May* has another
   home already, and it is the other way the instruction names: say nothing, and every fact of a declared relation
   is a candidate unless something takes it out. Of the 80 corpus programs with a hypothesis rule that are answered
   today, 12 need only the rule deleted, the puzzles among them; the rest say *may* with it, and move to the
   filtering way.
3. **The filtering way's one mechanism is the closure**, `(__closed__ R)`, a fact, stated or derived. The two
   lists of `(solve …)` are the same thing written twice more: replaced by closure facts they give the same $H$ and
   the same $R$ in 99 of the 100 corpus programs that use one and are answered. What takes out a single fact, and
   not a relation, is no mechanism at all: a fact root denies is no candidate, so an ordinary rule that denies it
   does it.

## 1. What each says, by the definitions

From [hypothesis generation](../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md), § 3 and § 4:

$$
\begin{aligned}
\Pi_{\mathrm{hr}} &= \lbrace\, \psi_h(a, \theta) \;:\; \mathrm{root} \models \varphi_h(a, \theta) \,\rbrace \\
\operatorname{owes}(o, a, \theta, S) &\iff S \models T_o(a, \theta) \ \land\ \lnot \exists \bar{y}.\ S \models \Delta_o(a, \theta, \bar{y}) \land w_o(a, \theta, \bar{y}) \\
\operatorname{pay}(o, a, \theta) &= \lbrace\, w_o(a, \theta, \bar{y}) \;:\; \mathrm{root} \models \Delta_o(a, \theta, \bar{y}) \,\rbrace
\end{aligned}
$$

| | a hypothesis rule $h$ | an obligation $o$ |
|---|---|---|
| read | once, on root's fixpoint | on root's fixpoint for $H$; on every state for what it owes |
| proposes | $\psi_h$ for each match of $\varphi_h$ | $w_o$ for each owed instance and each $\bar{y}$ of its domain: the hypothesis rule with pattern $T_o \land \lnot\exists W_o \land \Delta_o$ and conclusion $w_o$ |
| requires of a state | **nothing** | **a witness for each instance**: a state that owes is not in $R$ |
| what it groups | nothing: each candidate stands alone | the candidates into debts, each *at least one of these* |
| beside the scoping | overrides it: the lists and a closure do not bind it | bound by it |

So every obligation has its hypothesis rule, and the converse fails twice. A hypothesis rule has no requirement to
turn into a debt; and its literal twin, *each match owes its own fact*, is no obligation: a debt needs a choice.

## 2. Six programs

[`review/part-1/43-Q-M1f.561/programs/`](review/part-1/43-Q-M1f.561/programs/). Two things, `a` and `b`, and an atom
`c` that is no thing; `s` is the relation in question. `ein solve` and `ein oracle` agree on each.

| | program | how it is said | $H$ | $R$ |
|---|---|---|---|---|
| 1 | [`1-may-hrule.ein`](review/part-1/43-Q-M1f.561/programs/1-may-hrule.ein) | a hypothesis rule: of each thing, `s` may be supposed | `(s a)`, `(s b)` | **4 states**, `∅` among them: `{}`, `{a}`, `{b}`, `{a, b}` |
| 2 | [`2-must-one.ein`](review/part-1/43-Q-M1f.561/programs/2-must-one.ein) | an obligation: `s` of some thing is owed | the same two | **3 states**: `∅` is out, *root owes 1 instance* |
| 3 | [`3-must-each.ein`](review/part-1/43-Q-M1f.561/programs/3-must-each.ein) | the literal twin of 1: each thing owes its own `s` | | **refused at load**: *`(open ?R)`'s `?R` premise binds no variable of its own, so the obligation is ground — that is a plain `absent` check and not something a witness could discharge* |
| 4 | [`4-may-blind.ein`](review/part-1/43-Q-M1f.561/programs/4-may-blind.ein) | nothing: no rule proposes, `s` is declared | `(s a)`, `(s b)`, **`(s c)`** | 8 states: the enumerator fills the place with every guessable atom |
| 5 | [`5-may-blind-denied.ein`](review/part-1/43-Q-M1f.561/programs/5-may-blind-denied.ein) | the same, and a rule that denies `s` of what is no thing | `(s a)`, `(s b)` | **4 states, program 1's** |
| 6 | [`6-may-as-must-decide.ein`](review/part-1/43-Q-M1f.561/programs/6-may-as-must-decide.ein) | *may* written as a *must*: each thing owes a decision, yes or no | four decisions | 4 states, each a whole decision; `∅` is out |

1 against 2 is the difference. 1 against 5 is *may* said the other way, with the same answer. 6 shows that *may*
can be written as an obligation, by owing the decision instead of the fact; its states are not program 1's: none
is partial, so the region's shape, its bottom at `∅`, its lines, is gone.

## 3. The corpus's hypothesis rules

[`one_way_each/hr_census.py`](one_way_each/hr_census.py), its table
[`hr_census.tsv`](one_way_each/hr_census.tsv). Each program that declares an hrule is run as written (A), with every
`(hrule …)` form and every `:hrules` clause deleted (B), and, where B falls to the blind enumerator, with
`:hypothesis-relations` naming the relations of A's candidates (C). Two runs are *the same* when $H$ is the same
set and the region has the same states, `k_s` and `k_c`.

113 programs declare one; 33 are refused as written (they are witnesses of refusals); **80 are answered**.

**21 where an obligation owes at root: the hypothesis rule stands beside a *must*.**

| what deleting the rule does | programs | which |
|---|---|---|
| nothing: the same $H$ and $R$ | 11 | `zebra2`, `zebra2-minus-15`, `send_more_money` and its twin, `cryptarithm`, the three equations, `lattice/07`, `synth/one_model_per_component`, `tests/region/10` |
| the same $R$, a larger $H$ | 1 | `lattice/06`, 19 candidates to 25 |
| the same one state, $H$ doubled | 1 | `zebra.ein`, 56 candidates to 112; `k_c` moves with it |
| $R$ changes | 7 | `lattice/08`, `13`, `14`, `synth/merged_states`, `trivial_line`, `maximal_not_top`, `tests/region/02`: the rule proposes facts no debt asks for, or keeps the obligations rung from declining |
| refused | 1 | `features/13_mixed_solution_and_open`: its hrule proposes beside an obligation of a relation a list takes out |

**59 where nothing is owed at root: the hypothesis rule is all that says what may be supposed.** 28 are under
`examples/ein-bugs/`, 16 under `examples/lattice/` and `tests/region/`, 6 under `examples/branching/`.

| with the rule deleted (B), then scoped to the same relations (C) | programs |
|---|---|
| B is the same: the blind enumerator proposes what the rule proposed | 12 |
| B has another $H$ and the same $R$ | 2 |
| B differs and C is the same: a list of relations is all the rule was | 5 |
| B and C differ: the rule's pattern picks arguments the enumerator does not | 25 |
| B is on no rung of its own (nothing to guess, the obligations rung declined, or refused) | 15, of which 4 answer the same |

So 23 of the 59 answer the same with the rule deleted (18) or deleted and a list of relations added (5): they are
the filtering way already. The 25 are
program 4 against program 1: the enumerator fills a place with every guessable atom, and what the rule's pattern
kept out has to be denied by a rule (program 5) or taken out by a filter that reaches arguments.

## 4. The filtering way, and its one mechanism

Today a program takes facts out of $H$ five ways, and three of them are one statement.

| | how it is written | what it says | what else reads it |
|---|---|---|---|
| a closure | the fact `(__closed__ R)`, stated or derived by a rule | no commitment adds a hypothesised fact of R | *forbid*: an `absent` over R is safe; the mechanisms' conflicts |
| *never these* | `(solve :no-hypothesis (R …))` | the same, of each R listed | the same; and the loader holds each name to the program's |
| *only these* | `(solve :hypothesis-relations (R …))` | the same, of every declared relation **not** listed | the same |
| a settled fact | root holds F, or `(not F)` | F is decided: no candidate | everything: it is a fact |
| nothing to guess | `(solve :no-search)` | no candidate at all | `--no-search` |

[`one_way_each/scope_census.py`](one_way_each/scope_census.py), its table
[`lists.tsv`](one_way_each/lists.tsv): each corpus program that states a list, with the list replaced by closure
facts.

| the list | answered as written | the same $H$ and $R$ with closures | the others |
|---|---|---|---|
| `:no-hypothesis` | 81 | **80** | `features/13_mixed_solution_and_open`: refused, since a closure of a relation an obligation owes is a conflict, where the same relation in a list beside an hrule is only remarked on |
| `:hypothesis-relations` | 19 | **19** | one of them, `features/09`, only once every declared relation outside the list is closed, `right-of`, `next-to` and `is-a*`, and not just those that had candidates |

**Proposed: the closure is the filtering way's one mechanism.** It is the one of the three that is a fact, so a
rule can derive it, a module can state it of its own relations, and nothing of it is in a clause of `solve`. *Never
these* is a closure a relation. *Only these* is a closure of every other declared relation, which a program
writes, or a rule of the standard library derives from a mark on the ones left open. A single fact is taken out by
denying it, which is no mechanism but what a candidate is: a fact root has not settled.

The same table of the corpus, by the rung that builds $H$ ([`rungs.tsv`](one_way_each/rungs.tsv)), says how much
each way carries today: the blind enumerator **483** programs, the hypothesis rules 82, the obligations 21,
`:no-search` 45, the obligations rung declined 9 and stuck 2; 117 are refused at load and 127 print no rung.

## 5. What removing the hypothesis rules would take

**Out of the language and the kernel**: the declarator `hrule`; the clause `(solve :hrules …)`; the ladder's first
rung and its override of the scoping; two of the five rows of the mechanisms' conflicts; the `hrules` rung of the
events and of `ein show hyps`; `ein-infer/src/hrule.rs`. With the two lists gone as well: both clauses, the checks
of their names, and the conflicts' remaining rows but one, *an obligation owes a closed relation*. The ladder is
then: nothing to guess; else the obligations, where one is activated; else every unsettled fact of a relation that
is not closed.

**In the corpus**, of the 80 programs:

- 12 lose the rule and nothing else;
- `zebra.ein` loses it and searches a universe twice the size for the same state;
- 59 and 7 say *may* with it, and each is rewritten one of three ways: nothing but a closure of the relations not
  meant (23 are that already); a rule that denies what the pattern kept out (program 5); or a decision that is owed
  (program 6), where the program meant a choice all along. Most of these are the fixtures that pin the region's
  shapes, under `examples/lattice/`, `tests/region/` and `examples/ein-bugs/`, so each rewrite is checked against
  the shape it was written to have;
- `features/13`, *may* and *must* of one program, has no form left unless the two ways add up.

**The one thing that has no other home: *may* and *must* in one program.** The ladder takes one rung: under an
activated obligation $H$ is what the debts ask for, and nothing the enumerator would add
([Q-M1f.243](../../../docs/decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md): a root
that owes nothing is complete, and falling back to the enumerator there would let exhaustion decide what discharge
decides). A hypothesis rule is today the only way to have candidates beside the debts' own: the seven programs
whose $R$ changes, and `features/13`. Without it they are written with a decision that is owed, or the two ways
are made to add up, which is that decision reopened.

**The user on this, 2026-11-19**, verbatim: *"Yes, true, hrules and obligations are not the same, easy there could be the program that has obligations, restrictions, but is not solvable without hrules because nothing generate "MAY" hypotheses"*. `examples/lattice/13_witness_of_two.ein` is that program in the corpus. Ann owes a dish (`total-owed fed is-a`), and
the one fact that pays her, `(fed Ann Soup)`, is derived: by one rule from `(orders Ann)`, by another from
`(cooks Bob)` and `(shops Cat)` together. Those three are what may be supposed, and only its three hypothesis rules
propose them. As written: 3 candidates, 5 states. With the hypothesis rules deleted the obligations rung proposes
the one thing the debt names, the derived fact itself: 1 candidate, 1 state, `(fed Ann Soup)` supposed outright and
the four ways of earning it gone ([`hr_census.tsv`](one_way_each/hr_census.tsv)).
So call A of the question is not (a) as first proposed here: the hypothesis rule is what says *may* beside a
*must*, and it stays unless the two ways are made to add up.

**On the pages**: the guide builds the Zebra program with a hypothesis rule, and
[`09-hypotheses.md`](../../../docs/ein/user/03-syntax-and-semantics/09-hypotheses.md) and the reasoning page teach
three rungs.

**What it costs to run**: nothing where the obligations already build $H$. Where a typed *may* moves to the
enumerator, the universe is every guessable atom in every place until rules deny the rest: the enumerator proposes
$p\,k + q\,k\,(k-1)$ facts for $k$ atoms, and each one a rule does not deny at root is a candidate the search
carries.

## 6. The question

[Q-M1f.561](../open_questions.md#q-m1f561--one-way-to-say-must-and-one-to-say-may-hypothesis-rules-removed), in
[review item 43](review/part-1/43-Q-M1f.561/README.md): whether the hypothesis rules go; whether the two lists go
with them, leaving the closure; and what becomes of *may* beside *must*.

It touches the second part of the phase at one point and no more. There, under
[Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules)
(b), a candidate is a fact root holds and a rule may conclude it: that is a hypothesis rule again, as an ordinary
rule and with no declarator. So removing `hrule` now and holding candidates as facts later do not fight: the
first takes a way of saying *may* out of the kernel, the second would give a program one back as a rule of its own.
