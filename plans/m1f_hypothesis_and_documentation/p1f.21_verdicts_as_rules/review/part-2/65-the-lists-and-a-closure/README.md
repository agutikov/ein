# 65 · The two lists and a closure, against root's `open` facts

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.578_the_lists_and_a_closure/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.8 of the stage. You ruled (Q-M1f.578) that `:hypothesis-relations`, `:no-hypothesis` and
`(__closed__ R)` are the blind enumerator's white list and black list; that it runs only where root's fixpoint holds
no `open` fact; and that one of the three that disagrees with the `open` facts root holds is an error. Here: the
eight programs of the ruling's page, `examples/features/13_mixed_solution_and_open.ein`, and three programs for the
two things the ruling does not say. What the final binary does with each row, and which reading of the two questions
it built, is [On the built engine](#on-the-built-engine), at the foot of the page.

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

## On the built engine

**In short.** Each of the twelve rows does on the final binary what § 1's last column said, read in built spelling.
The binary built the default (a) of both questions: program 9 is refused (Q-M1f.595), program 11 loads (Q-M1f.597).
One thing moved otherwise: `13_mixed_solution_and_open.ein` loads, because the build rewrote its list and kept the
form the row read as a load fixture of its own, which is refused as the row said.

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of 7b72bd84 (its
five lines: `runs/built/version.txt`), with the checkout's `stdlib/`. `tools/run-built.sh` re-takes every run into
`runs/built/`: for each program the read-out, `-p`, `ein show hyps`, `ein saturate --dump`, `ein test`, the oracle
with its dump and `--states`, the events and the summary, the answer and the visited nodes as files, and the
knowledge base, the lattice and the region drawn where the program loads; `tools/table-built.py` reads them into
`runs/built/table.md` and `table.json`; `runs/built/corpus-lists.txt` is the first line `ein show hyps` says of every
corpus program whose text names one of the two lists or `__closed__`. *Once built* is no longer read off the stand-in's
runs: the binary computes it.

**The programs.** Programs 5, 7 and 8 of the ruling's page and this item's 9 and 10 write `hrule` and `:hrules`, which
the built engine refuses at the parse (`runs/built/5-hrule-against-list.solve.txt` and its kin: *unexpected `)`* at
the hrule's `()`). Each is kept as it is and written again in the spelling the built engine reads, a rule that
concludes `(open F)` in place of the `hrule` and no `:hrules`:
[`q578-5-hrule-against-list.built.ein`](programs/q578-5-hrule-against-list.built.ein),
[`q578-7-hrule-and-obligation-against-list.built.ein`](programs/q578-7-hrule-and-obligation-against-list.built.ein),
[`q578-8-hrule-with-idle-list.built.ein`](programs/q578-8-hrule-with-idle-list.built.ein) (the ruling's page's
three, under names that say so),
[`9-white-list-leaves-a-rule-out.built.ein`](programs/9-white-list-leaves-a-rule-out.built.ein) and
[`10-white-list-names-more.built.ein`](programs/10-white-list-names-more.built.ein). Programs 1 to 4, 6 and 11 are
run as they are. Row 12 is run twice: `examples/features/13_mixed_solution_and_open.ein` as the corpus has it now,
and [`examples/broken/load/no_hypothesis_beside_open.ein`](../../../../../../examples/broken/load/no_hypothesis_beside_open.ein),
which its corpus entry calls *`13_mixed_solution_and_open.ein` as it was written*: its `:no-hypothesis (is-a likes)`.

| row | the page's *once built* | the built engine | root's `open` facts: the page / the built engine | where, in `runs/built/` |
|---:|---|---|---|---|
| 1 | loads; the blind enumerator runs | loads; $H$ from `blind`, 6 candidates; $R$ 64 states; no note | none / the blind enumerator's 6 | `1-blind-no-list.show-hyps.txt`, `.solve.txt`, `.saturate-dump.txt` |
| 2 | loads; the list filters the blind enumerator | loads; `blind`, 2: `(likes Ann Soup)` `(likes Soup Ann)`; no note | none / those 2 | `2-blind-hypothesis-relations.*` |
| 3 | loads, as 2 | loads; `blind`, the same 2; no note | none / those 2 | `3-blind-no-hypothesis.*` |
| 4 | loads, as 2 | loads; `blind`, the same 2; no note | none / those 2 | `4-blind-closed.*` |
| 5 | **an error**: `:no-hypothesis` names an `open` fact's relation | built spelling: **refused**, line A below | `likes` / `(open (likes Ann Soup))`, which the line names | `q578-5-hrule-against-list.built.solve.txt` |
| 6 | **an error**, the same | **refused**, line B | `likes` / `(open likes)` | `6-obligation-against-list.solve.txt` |
| 7 | **an error**, the same | built spelling: **refused**, line C, one line, at the one list that disagrees | `knows`, `likes` / `(open likes)`, which the line names | `q578-7-hrule-and-obligation-against-list.built.solve.txt` |
| 8 | loads: the list agrees | built spelling: loads; $H$ from `hypothesis-rules`, 1: `(likes Ann Soup)`; $R$ 2 states; no note, where the stand-in engine printed *its :hypothesis-relations filters no candidate* | `likes` / `(open (likes Ann Soup))` | `q578-8-hrule-with-idle-list.built.*` |
| 9 | loads by the ruling's words; **an error** by Q-M1f.595's default | built spelling: **refused**, line D: the default | `knows`, `likes` / `(open (knows Ann Soup))`, which the line names | `9-white-list-leaves-a-rule-out.built.solve.txt` |
| 10 | **an error**: the list names `knows`, of which no `open` fact is | built spelling: **refused**, line E | `likes` / *root holds no open fact of knows* | `10-white-list-names-more.built.solve.txt` |
| 11 | loads by the ruling's words (Q-M1f.597's default) | loads; $H$ from `obligations`, no candidate; $R$ {∅}; no note: the default | none / none | `11-obligation-owes-nothing-against-list.*` |
| 12 | **an error**, as 7 | the form the row read: **refused**, line F. The corpus program now: loads; $H$ from `union`, 3: `(choice Ann)` `(choice Bob)` `(likes Ann Soup)`; $R$ 3 states; its `:expect` holds | `choice`, `likes` / `(open likes)` and the three | `no_hypothesis_beside_open.solve.txt`; `13_mixed_solution_and_open.*` |

The lines, each after `kb load error: ` and each the whole of its program's output but the exit, 1:

```text
A  programs/q578-5-hrule-against-list.built.ein:20:24: solve: its :no-hypothesis names likes, and root holds (open (likes Ann Soup)), which rule 'guess' writes — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error: drop likes from the list
B  ../../../q-m1f.578_the_lists_and_a_closure/programs/6-obligation-against-list.ein:16:24: solve: its :no-hypothesis names likes, and root holds (open likes), which obligation 'owes-food' writes — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error: drop likes from the list
C  programs/q578-7-hrule-and-obligation-against-list.built.ein:26:24: solve: its :no-hypothesis names likes, and root holds (open likes), which obligation 'owes-food' writes — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error: drop likes from the list
D  programs/9-white-list-leaves-a-rule-out.built.ein:26:31: solve: its :hypothesis-relations leaves out knows, and root holds (open (knows Ann Soup)), which rule 'meet' writes — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error, so beside them it names exactly their relations: add knows to the list, or drop the list
E  programs/10-white-list-names-more.built.ein:21:37: solve: its :hypothesis-relations names knows, and root holds no open fact of knows — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error, so beside them it names exactly their relations: drop knows from the list
F  examples/broken/load/no_hypothesis_beside_open.ein:41:24: solve: its :no-hypothesis names likes, and root holds (open likes), which obligation 'total-owed' writes — the two lists and a closure configure the blind enumerator, which does not run beside an open fact, and one that disagrees with the open facts root holds is an error: drop likes from the list
```

**Where the built engine reads otherwise than the page.**

- **Root's `open` facts in programs 1 to 4.** The page's *none* counted what a rule and an obligation write. On the
  built engine the blind enumerator writes its own guesses as root's `(open F)`, where no other writer does
  (Q-M1f.570, Q-M1f.576): root holds 6, 2, 2 and 2 `open` facts (`runs/built/1-blind-no-list.saturate-dump.txt` and
  its kin). The lists filtered those guesses, so they agree with them, and nothing is refused.
- **Row 12.** The page read *one corpus program becomes an error*. The build rewrote the corpus program's list to
  `:no-hypothesis (is-a)`, its comment saying the list *names no relation an `open` fact is of*, and kept the form
  with `likes` in the list as `examples/broken/load/no_hypothesis_beside_open.ein`, a load-negative fixture, refused
  with line F. So no corpus program is refused for its list; the refusal's two fixtures are the corpus's two programs
  refused for disagreeing with root's `open` facts, of the 250 whose text names a list or `__closed__`
  (`runs/built/corpus-lists.txt`). § 2's `examples/ein-bugs/closed-and-owing.ein` is the other one, refused at its
  closure: *root holds (open r), which obligation 'total-owed' writes, and (__closed__ r), which rule 'infer-closure'
  derives, says r is closed* (`runs/built/closed-and-owing.solve.txt`).
- **Program 8's note.** The stand-in engine said *its :hypothesis-relations filters no candidate*; the built engine
  says nothing: beside an `open` fact a list that names exactly its relations agrees and is no note.

**Which reading the binary built.**

| question | its default | what the binary does | where |
|---|---|---|---|
| Q-M1f.595, an `open` fact of a relation `:hypothesis-relations` leaves out | (a), an error | (a): program 9 is refused, line D, *leaves out knows … add knows to the list, or drop the list* | `runs/built/9-white-list-leaves-a-rule-out.built.solve.txt` |
| Q-M1f.597, what the lists and a closure are judged against | (a), the `open` facts root holds | (a): program 11, whose activated obligation owes nothing at root, holds no `open` fact and loads, with no note | `runs/built/11-obligation-owes-nothing-against-list.solve.txt`, `.saturate-dump.txt` |
