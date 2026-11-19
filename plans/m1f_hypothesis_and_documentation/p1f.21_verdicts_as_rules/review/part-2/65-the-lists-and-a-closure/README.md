# 65 · The two lists and a closure, against root's `open` facts

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.578_the_lists_and_a_closure/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.8 of the stage. You ruled (Q-M1f.578) that `:hypothesis-relations`, `:no-hypothesis` and
`(__closed__ R)` are the blind enumerator's white list and black list; that it runs only where root's fixpoint holds
no `open` fact; and that one of the three that disagrees with the `open` facts root holds is an error. Here: the
eight programs of the ruling's page, `examples/features/13_mixed_solution_and_open.ein`, and three programs for the
two things the ruling does not say.

**The answer, short.** In the corpus **one program becomes an error**, `13_mixed_solution_and_open.ein`
([item 58](../58-the-corpus-under-the-rulings/README.md), § 4). Of the ruling's page's eight, program 7, which
loads today, becomes an error; 5 and 6 are refused today and stay errors; 1 to 4 and 8 load. Two questions are put
below: the white list's other direction (Q-M1f.595), and whether the check reads the `open` facts root holds or
what the program declares (Q-M1f.597). No corpus program moves either way on either.

## 1. The runs

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes them
into `runs/` (ignored by git; mirrored under `build/review-incoming/p1f21-s12-examples/65-the-lists-and-a-closure/`);
`runs/after.md` has each program today and once built.

**Once built is read off today's runs**, no binary computing it: root's `open` facts are a rule's candidates
(`ein show hyps` of the program with both lists deleted, so a program refused today for its list shows them too)
and an obligation's (the program with its hypothesis rules and both lists deleted, on the obligations rung); then
the ruling's three errors are applied to them.

All the programs but the last have one person, Ann, one food, Soup, and four relations, `person`, `food`, `likes`
and `knows`. 1 to 8 are [the ruling's page's](../../../q-m1f.578_the_lists_and_a_closure/README.md); 9 to 11 are this
item's.

| program | writes | today | root's `open` facts once built | once built |
|---|---|---|---|---|
| [1](../../../q-m1f.578_the_lists_and_a_closure/programs/1-blind-no-list.ein) | nothing proposes | 6 blind candidates | none | loads; the blind enumerator runs |
| [2](../../../q-m1f.578_the_lists_and_a_closure/programs/2-blind-hypothesis-relations.ein) | `:hypothesis-relations (likes)` | 2 blind candidates | none | loads; the list filters the blind enumerator |
| [3](../../../q-m1f.578_the_lists_and_a_closure/programs/3-blind-no-hypothesis.ein) | `:no-hypothesis (person food knows)` | 2 | none | loads, as 2 |
| [4](../../../q-m1f.578_the_lists_and_a_closure/programs/4-blind-closed.ein) | the three closed | 2 | none | loads, as 2 |
| [5](../../../q-m1f.578_the_lists_and_a_closure/programs/5-hrule-against-list.ein) | a rule proposes `likes`; `:no-hypothesis (likes)` | refused | `likes` | **an error**: `:no-hypothesis` names an `open` fact's relation |
| [6](../../../q-m1f.578_the_lists_and_a_closure/programs/6-obligation-against-list.ein) | an obligation owes `likes`; `:no-hypothesis (likes)` | refused | `likes` | **an error**, the same |
| [7](../../../q-m1f.578_the_lists_and_a_closure/programs/7-hrule-and-obligation-against-list.ein) | a rule proposes `knows`, an obligation owes `likes`; `:no-hypothesis (likes)` | loads: the rule overrides, H is `(knows Ann Soup)` | `knows`, `likes` | **an error**, the same |
| [8](../../../q-m1f.578_the_lists_and_a_closure/programs/8-hrule-with-idle-list.ein) | a rule proposes `likes`; `:hypothesis-relations (likes)` | loads, with a note: the list filters nothing | `likes` | loads: the list agrees |
| [9](programs/9-white-list-leaves-a-rule-out.ein) | rules propose `likes` and `knows`; `:hypothesis-relations (likes)` | refused: the list leaves out `knows`, which a rule proposes | `knows`, `likes` | loads by the ruling's words; **an error** by Q-M1f.595's default |
| [10](programs/10-white-list-names-more.ein) | a rule proposes `likes`; `:hypothesis-relations (likes knows)` | loads, with a note: the list filters nothing | `likes` | **an error**: the list names `knows`, of which no `open` fact is |
| [11](programs/11-obligation-owes-nothing-against-list.ein) | 6, with `(likes Ann Soup)` at root: the obligation owes nothing | refused, as 6 | none | loads by the ruling's words (Q-M1f.597's default); refused by its other option |
| [`13_mixed_solution_and_open.ein`](../../../../../../examples/features/13_mixed_solution_and_open.ein) | a rule proposes `choice`, an obligation owes `likes` (root owes 1); `:no-hypothesis (is-a likes)` | loads: the rule overrides, H `(choice Ann)` `(choice Bob)` | `choice`, `likes` | **an error**, as 7 |

Where a program loads once built and holds `open` facts (8, and 9 by the ruling's words), the list does nothing:
the blind enumerator does not run beside them. Today's notes, *the list filters no candidate*, are said of a list
that is then an error or agrees, which the stage's plan already names.

## 2. What the ruling leaves

**The white list's other direction (Q-M1f.595).** You named three disagreements: a hypothesis of a closed relation,
a `:no-hypothesis` that names an `open` fact's relation, and a `:hypothesis-relations` that lists a relation of
which no `open` fact is. The white list can disagree the other way too: an `open` fact of a relation it does not
list, program 9. Today that is refused (*an active hrule proposes a relation the list leaves out*). No corpus
program has a `:hypothesis-relations` beside an `open` fact.

**What the check reads (Q-M1f.597).** Your words judge the lists against *emitted open facts*. Today's check judges
them against what the program declares: an activated obligation of a listed relation is refused whether or not
root owes (Q-M1f.244). The two differ only where an activated obligation owes nothing at root, program 11. In the
corpus the one program refused today for a closure beside an obligation, `examples/ein-bugs/closed-and-owing.ein`,
owes at root, so it is refused either way, and `13_mixed_solution_and_open.ein` owes 1 at root.

## 3. Your call

**Q-M1f.595: an `open` fact of a relation `:hypothesis-relations` leaves out.**

- **(a)** An error too: a white list beside `open` facts names exactly their relations. Program 9 stays refused, as
  today. **Default**: a white list that leaves out what is supposed disagrees with it as much as one that names what
  is not.
- **(b)** Not an error: only the direction you named is. Program 9 loads, its list doing nothing.

**Q-M1f.597: whether the lists and a closure are judged against the `open` facts root holds, or against what the
program declares.**

- **(a)** Against the `open` facts root holds, your words: an obligation that owes nothing at root writes none and
  disagrees with nothing. Program 11 loads. **Default.**
- **(b)** Against what the program declares, as today: an activated obligation of a listed or closed relation, and
  an active rule that concludes `(open F)` of one, are errors whether or not root holds such a fact. Program 11 stays
  refused.
