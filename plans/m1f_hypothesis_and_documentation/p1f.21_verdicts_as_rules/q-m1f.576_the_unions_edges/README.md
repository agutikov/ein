# 54 · Q-M1f.576: the union's edges: an obligation whose candidates cannot be read, and the blind enumerator

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f576--the-unions-edges-an-obligation-whose-candidates-cannot-be-read-and-the-blind-enumerator)

**Ruled 2026-11-20, and out of the review bundle**, where it was item 54 of part 2. Your answer, in the part's
answers' file:

> 1 - not exactly, "guard or trigger reads what a commitment can change" is not declined anymore, it is allowed,
> detected as non-monotone programs, but allowed. 2 - (a); 3 - What? Explain in more details

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-52-to-55).
Its first part is put back as review item 57, since *declined* may have been read as *refused*, and its third, which you asked to have explained, is review item 56. The page is kept, with its programs, as the survey the second part's stages cite; where it says
*the default*, read it as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are
on this machine under `build/review-archive/p1f21-part-2/54-Q-M1f.576/`, as they were run.

**From your answer to item 48**, 2026-11-20: *"Identical open facts deduplicate naturally. Exactly (b)."* So $H$
is every `(open F)` root holds, whoever wrote it.

**The gap.** There are three writers: a rule, an obligation, and the blind enumerator. Today the three are a
ladder, and two of its steps say more than *who proposes*: an obligation whose candidates cannot be read sends
the whole call to the blind enumerator, and the blind enumerator proposes only where nothing else does. With a
rule beside them, the union does not say what those two do.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored, so on this machine only).

## 1. An obligation that declines, beside a rule

An obligation's candidates are read off the guard inside its `absent`. Where that guard, or the trigger, reads a
relation a commitment can change, the set could grow above root, so the rung declines and the blind enumerator
builds $H$. A hypothesis rule overrides that today. Three corpus programs have both (an earlier census of this
phase, on an older build, [`one_way_each/hr_census.tsv`](../one_way_each/hr_census.tsv)):

| program | candidates today, its rule's | candidates with the rule deleted, the blind enumerator's |
|---|---:|---:|
| `examples/lattice/06_friends_tables.ein` | 19 | 923 |
| `examples/lattice/synth/local_not_global.ein` | 2 | 30 |
| `examples/lattice/synth/maximal_not_top.ein` | 3 | 57 |

The second is run here: [`a-local-not-global.ein`](programs/a-local-not-global.ein) proposes 2, and the same with
its rule deleted (`runs/b-local-not-global.ein`) proposes 30, its rung line saying *an obligation scans `h`, which
the rung itself proposes* (`runs/rung-lines.jsonl`). All three are programs the static check flags.

- **(a)** Beside a rule that concludes `(open F)`, an obligation that declines writes no candidate and calls no
  one: it counts, and the rule's candidates stand. Those three programs keep their $H$. **Default.**
- **(b)** It calls the blind enumerator as it does alone: their $H$ becomes the union with the blind one's, 923,
  30 and 57 or more.

## 2. The blind enumerator

- **(a)** It writes its guesses only where no rule concludes `(open F)` at root and no obligation is activated, as
  it proposes today. **Default.** By the same census, 44 corpus programs have a hypothesis rule and no obligation:
  each keeps its $H$.
- **(b)** It always writes them: every program's $H$ holds every guess.

## 3. Unchanged, to confirm

The two lists of `(solve …)` and a closure bind every writer as they bind today: a rule's `(open F)` of a relation
they take out is the conflict a hypothesis rule's is today, refused at root; an obligation's candidate of such a
relation is counted and not written. **Default: so.**

## 4. The ruling

Ruled: the answer at the head of this page.
