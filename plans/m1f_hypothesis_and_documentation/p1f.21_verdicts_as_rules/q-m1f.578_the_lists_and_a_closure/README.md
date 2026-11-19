# 56 · Q-M1f.578: the two lists of `(solve …)` and a closure, once the hypotheses are `open` facts

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts)

**Ruled 2026-11-20, and out of the review bundle**, where it was item 56 of part 2. Your answer, in the part's
answers' file:

> all 3 (:hypothesis-relations, :no-hypothesis, __closed__) are configuration for blind enumerator, white list and
> black list; blind enumerator runs ONLY if at the root fixpoint (open ...) context is empty - no facts. If any of
> them present together with emitted open facts, but disagree - it's an error, invalid program. e.g. rule emits
> closed relation hypotheses - it is an error. :no-hypothesis forbids existing open facts - error,
> :hypothesis-relations lists relations that are not in open facts - also error. Do not ever run blind enumerator
> together with hypotheses emitted by rules. (a)

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-56-and-57).
It leaves nothing open. The page is kept, with its programs, as the survey the second part's stages cite; where it says
*the default*, read it as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are
on this machine under `build/review-archive/p1f21-part-2/56-Q-M1f.578/`, as they were run.

**From your answer to item 54**, 2026-11-20, of its third question: *"What? Explain in more details"*.

The question was put in one sentence and put wrongly in half of it. This page is the explanation, on runs, and
the question again.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored); `runs/candidates.txt` has one block a program.

## 1. What the three are

A program has three ways to say that a relation is **not** something to suppose:

| | written | says |
|---|---|---|
| `:hypothesis-relations` | `(solve :hypothesis-relations (likes))` | only facts of these relations may be supposed |
| `:no-hypothesis` | `(solve :no-hypothesis (person food knows))` | facts of these relations may not be supposed |
| a closure | `(__closed__ knows)`, a fact | what the state holds of `knows` is all of it |

## 2. What they do today

One person, Ann, one food, Soup, and four relations: `person`, `food`, `likes`, `knows`.

**With the blind enumerator, they filter.** Nothing proposes, so the blind enumerator guesses a fact of every
relation over every name:

| program | candidates |
|---|---|
| [`1-blind-no-list.ein`](programs/1-blind-no-list.ein) | 6: `(food Ann)` `(person Soup)` `(knows Ann Soup)` `(knows Soup Ann)` `(likes Ann Soup)` `(likes Soup Ann)` |
| [`2-blind-hypothesis-relations.ein`](programs/2-blind-hypothesis-relations.ein), `:hypothesis-relations (likes)` | 2: `(likes Ann Soup)` `(likes Soup Ann)` |
| [`3-blind-no-hypothesis.ein`](programs/3-blind-no-hypothesis.ein), `:no-hypothesis (person food knows)` | the same 2 |
| [`4-blind-closed.ein`](programs/4-blind-closed.ein), the three relations closed | the same 2 |

**With a hypothesis rule, they filter nothing, and a clash is refused.** *No list filters an hrule's candidates*
is today's rule: what a rule proposes is proposed.

| program | today |
|---|---|
| [`5-hrule-against-list.ein`](programs/5-hrule-against-list.ein): a rule proposes `likes`, and `:no-hypothesis (likes)` | **refused at load**: *the program says both that likes is guessed and that it is not* |
| [`8-hrule-with-idle-list.ein`](programs/8-hrule-with-idle-list.ein): a rule proposes `likes`, and `:hypothesis-relations (likes)` | loads, with a note: the list *filters no candidate* |

**With an obligation and no hypothesis rule, a clash is refused too.**

| program | today |
|---|---|
| [`6-obligation-against-list.ein`](programs/6-obligation-against-list.ein): an obligation owes `likes`, and `:no-hypothesis (likes)` | **refused at load**: *a state that owes likes and may not be given one is stuck* |

**With both, the rule overrides, so there is no clash.**

| program | today |
|---|---|
| [`7-hrule-and-obligation-against-list.ein`](programs/7-hrule-and-obligation-against-list.ein): a rule proposes `knows`, an obligation owes `likes`, and `:no-hypothesis (likes)` | loads. $H$ is `(knows Ann Soup)`: the obligation counts and proposes nothing, so nothing proposes `likes` |

## 3. What your rulings change

Two of them touch this. The hypothesis rule is now any rule that concludes `(open F)`, so *no list filters an
hrule's candidates* needs saying again. And $H$ is the union, so in program 7 the obligation now writes
`(open (likes Ann Soup))` beside the rule's candidate, of a relation the list takes out. One corpus program has
all three, a hypothesis rule, an obligation and a list: `examples/features/13_mixed_solution_and_open.ein`.

## 4. The question

What do the two lists and a closure do to an `(open F)` fact?

- **(a)** **They refuse a clash, whoever proposes.** A rule that concludes `(open F)`, a written `(open F)`, or an
  obligation's candidate, of a relation a list or a closure takes out, is a load error, as programs 5 and 6 are
  today. They filter the blind enumerator, as today. Program 7 is then refused, where it loads today.
  **Default**: it is today's rule, the override taken out of it.
- **(b)** **They filter $H$, and refuse nothing.** Every writer writes its `(open F)` facts; $H$ is those of the
  relations the lists and the closures leave. Programs 5, 6 and 7 load: in 5 and 6 the excluded candidate is held
  and not proposed, and in 6 root owes what nothing may pay, so $R$ is empty and the read-out says why.
- **(c)** **As (a) for a rule and a written fact, as (b) for an obligation's candidates**: what a program writes
  against its own list is its mistake and is refused; what an obligation derives is filtered.

## 5. The ruling

Ruled: the answer at the head of this page.
