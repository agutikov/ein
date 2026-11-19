# 51 · Q-M1f.573: `open` as an obligation or as a hypothesis, and which hypotheses are candidates

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates)

**From your answers**, 2026-11-19. To item 44: *"(open R) is obligation and (open (fact)) is an explicit
hypothesis."* To item 45: *"So there is no more need for :hrules special place - we just activate hypotheses
generating rules anywhere."*, and the same day of that sentence: *"by "Activate anywhere" I meant in program, not
necessary in :hrules list"*.

**The gap.** One word now says two things, told apart by what stands under it. Three cases are not settled by
that, and one of your sentences is filed here as you clarified it.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored); `runs/cells.txt` has one block a program.

## 1. A variable under `open`

| program | today |
|---|---|
| [`1-open-relation-parameter.ein`](programs/1-open-relation-parameter.ein): `:assert (open ?R)`, `?R` the rule's parameter | loads: the obligation. Root owes, and $H$ is `(seats Ann C1)` `(seats Ann C2)` |
| [`1-open-matched-variable.target.ein`](programs/1-open-matched-variable.target.ein): `:assert (open ?f)`, `?f` bound by `(candidate ?f)` | refused at load: *`(open ?f)` names a relation the activator does not bind — `?f` is not in the parameter list* |

So today's loader already reads which a variable is.

- **(a)** Decided at load, by what stands under `open`. A parameter that names a relation, or a relation's name:
  the obligation, with its `absent` and its witness as today. A fact, or a variable the `:match` binds: a
  hypothesis, and the variable's value is to be a fact. **Default.**
- **(b)** A hypothesis is always written with its fact, `(open (?R ?x))`; a bare variable under `open` is the
  obligation's and nothing else.

## 2. A hypothesis root has already decided

[`2-decided-candidates.ein`](programs/2-decided-candidates.ein) proposes `(p A)`, `(p B)`, `(p C)` and `(p D)`
where root holds `(p A)` and `(not (p B))`. Today $H$ is `(p C)` `(p D)`.

- **(a)** As today: `(open F)` is held, and F is no candidate where root holds F or its negation. **Default.**
- **(b)** Every `(open F)` root holds is a candidate.

Either way `(open F)` says that F may be supposed, and a state that supposes it holds both: no rule and no kernel
step takes `(open F)` out when F is decided, which is your answer on deleting.

## 3. To confirm: $H$ is root's

An `(open F)` first concluded above root is held there and is not proposed
([item 45](../../../q-m1f.567_the_hrule_form_dropped/README.md), § 3; [V-21](../../../findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root),
which you parked). **Default: so.**

## 4. Filed as you clarified it: where a rule is activated

An activator of a rule that concludes `(open F)` is a fact the program writes, anywhere in it, as any rule's is;
there is no clause for it. A rule that could *derive* such an activator is the case of the loader's check on
what a rule could derive, which is unchanged by this and is the first reading of your part-1 answers
([part 1](../../part-1/README.md)), still to be confirmed.

## 5. Your call

For 1 and 2, a letter each; 3, yes or otherwise. In [`ANSWERS.md`](../ANSWERS.md) under item 51. Blank keeps the
defaults.
