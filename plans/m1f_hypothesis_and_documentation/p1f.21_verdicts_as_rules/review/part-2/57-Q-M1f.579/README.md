# 57 · Q-M1f.579: an obligation whose guard or trigger reads what a commitment can change

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change)

**From your answer to item 54**, 2026-11-20, of its first question: *"not exactly, "guard or trigger reads what a
commitment can change" is not declined anymore, it is allowed, detected as non-monotone programs, but allowed."*

**Why it is put back.** Your answer is filed as you wrote it. Before it is built, one word of the question needs
saying plainly, because the answer reads as if *declined* meant *refused*: **nothing is refused today.** Such a
program loads, is answered, and six of the eight in the corpus are answered right and state a claim that holds.
Taken to the letter, your answer changes that.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored, so on this machine only).

## 1. What *declines* means today

An obligation's candidates are read off the guard inside its `absent`, at root:

```lisp
(rule owes-food ()
  :match  (and (person ?a)
               (absent (and (food ?b) (likes ?a ?b))))    ; the guard is (food ?b)
  :assert (open likes)
  :why    "{?a} owes a liked food")
```

At root the foods are what root holds of `food`. In `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`
another rule makes a dish a food once someone likes something, so **a commitment can add a food**, and a
candidate with it that root's reading does not have. So the obligations rung **steps aside**: it proposes nothing,
and the blind enumerator proposes every fact of `likes`. That is all *declines* is. The program is not refused and
is not flagged: with every fact proposed at root, no hypothesis is first held above it.

## 2. The same program, both ways

| | [A, as it is](programs/a-today.ein) | [B, the guard as it reads at root](programs/b-the-guard-at-root.ein) |
|---|---|---|
| who proposes | the blind enumerator, the obligations rung having stepped aside | the obligation's guard at root, with no stepping aside (written here as a hypothesis rule, which overrides today) |
| candidates | 6, `(likes Ann Pie)` among them | 1, `(likes Ann Soup)` |
| $R$ | one state, `(likes Ann Pie)` | **empty**: root owes, and the one candidate is dead |
| its claim | `(model (likes Ann Pie))`, **holds** | none can be stated: the program would be flagged |

Pie is a food only once Ann likes something, so root's reading of the guard never proposes it.

## 3. The corpus programs it touches

Eight decline today for this reason (`ein solve FILE --events`, the `rung` line), none with a hypothesis rule:

| programs | flagged today | state a claim |
|---|---|---|
| `oblig-derived-domain.ein`, `oblig-derived-domain-declared.ein`, `oblig-fresh-name.ein`, `oblig-fresh-name-declared.ein`, `oblig-fresh-name-two.ein`, `oblig-fresh-name-two-declared.ein` | no | yes, and it holds |
| `oblig-stdlib-total-owed.ein`, `oblig-stdlib-total-owed-declared.ein` | yes | yes; nothing holds it, the program being flagged |

All are under `examples/ein-bugs/wrong-answers/`: they are the witnesses of the wrong answer the stepping aside
was built to end. Three more programs have such an obligation beside a hypothesis rule, which overrides it today
([the page of item 54](../../../q-m1f.576_the_unions_edges/README.md)).

## 4. The question

- **(a)** **As today where nothing else proposes.** Such an obligation steps aside and the blind enumerator
  proposes; the program stays answered and, where it is monotone, claimed. Beside a rule that concludes `(open F)`
  it writes nothing and the rule's candidates stand. It is allowed, as it is today.
- **(b)** **Your answer to the letter.** It never steps aside: it writes the candidates its guard gives at root,
  with or without a rule beside it, and the program is flagged as not monotone. The six programs lose their claims,
  and `oblig-derived-domain.ein` answers an empty $R$ under the warning where today it answers `(likes Ann Pie)`.
  **Default: it is what you wrote.**
- **(c)** **Both written.** It writes its guard's candidates at root and the blind enumerator writes its guesses
  too, so nothing is missed and the program need not be flagged for it; $H$ is as large as the blind one's.

## 5. Your call

(a), (b) or (c), in [`ANSWERS.md`](../ANSWERS.md) under item 57. Blank keeps (b), your answer as written.
