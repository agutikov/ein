# 49 · Q-M1f.571: what `absent` and `open` take, and what each is under an ordinary relation

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f571--what-absent-and-open-take-and-what-each-is-under-an-ordinary-relation) ·
[the table](../../../absent_open_not.md)

**From your answers**, 2026-11-19. To item 46: *"2,1 and (open (absent P)) - refused at load because both `absent`
and `open` are special kernel primitives (what are they now btw?) and their argument must be a fact, (absent P) is
not a fact, while (open P) is, because `open` is a special kernel promitive AND a fact."* To item 44: *"Under
contexts, like (think Ann (open (= 5 (+ 2 2)))) it is a simple fact, special form works only in global context. The
same with `absent` - it is allowed but doesn't work in context it became simple fact."*

**The gap.** Your verdicts on the cells are clear. The reason given for them, *their argument must be a fact*,
gives three other results than the ones you state, so the rule itself is to be said.

**What they are now.** Both are reserved words and neither is a fact. `absent` is one production of a premise,
`(absent Premise)`, and one of a claim; it compiles to a guard on a rule's plan and nothing stores it. `open` is one
production, a rule's whole `:assert`, `(open)` or `(open R)`; the loader takes such a rule out of the agenda, and
one pass reads it at each state's fixpoint.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored); `runs/cells.txt` has one block a program.

## 1. `absent`'s argument is a premise today, not a fact

| program | the premise | today |
|---|---|---|
| [`1-absent-of-conjunction.ein`](programs/1-absent-of-conjunction.ein) | `(absent (and (is-a ?b Chair) (seats ?a ?b)))` | loads; root derives `(unseated Bob)`. It is how 18 of the corpus's 20 loading obligation rules are written |
| [`1-universal.ein`](programs/1-universal.ein) | `(absent (and (q-of ?g ?x) (absent (p ?x))))` | loads; root derives `(all-p G1)`. It is the standard library's `forall`, and `std.debts`' `discharged-from-nothing-unpaid` |
| [`1-absent-absent.ein`](programs/1-absent-absent.ein) | `(absent (absent (p ?x)))` | loads; root derives `(hit A)`. The form you rule refused |

*Its argument must be a fact* refuses all three, and your item 44 keeps the first (*"Obligations by (open R) and
absent facts is already good"*).

- **(a)** `absent` takes a premise: a pattern, or a conjunction, which may hold an `absent`. Only the direct
  `(absent (absent P))` is refused. **Default.**
- **(b)** No `absent` anywhere inside an `absent`: the universal goes, and `forall` and `std.debts`' rule with it.

## 2. `(open (open P))`

By the reason given it is legal, `(open P)` being a fact. You rule it refused, which is also the default of item
46. So the rule is not about facts: **directly under `open`, neither kernel word is written.**

- **(a)** Refused, by that rule. **Default: your verdict.**
- **(b)** Legal, by the reason: a hypothesis that P may be supposed, held and read by nothing.

## 3. Under an ordinary relation

Today all three are refused (`runs/cells.txt`):

| program | today |
|---|---|
| [`3-open-under-relation.target.ein`](programs/3-open-under-relation.target.ein): `(think Ann (open (p A)))` | parse error, *unexpected `open`* |
| [`3-absent-under-relation.target.ein`](programs/3-absent-under-relation.target.ein): `(think Ann (absent (p A)))` | parse error, *unexpected `absent`* |
| [`3-absent-under-relation-premise.target.ein`](programs/3-absent-under-relation-premise.target.ein): a premise `(think Ann (absent (p ?x)))` | load error: *`(absent (p ?x))` is written where a value goes* |

Your item 44 makes both plain facts there; your item 46 says `(absent P)` is not a fact. With `question` standing
for `open`, a nested one is already a plain fact and nothing reads it:
[`3-question-under-relation.ein`](programs/3-question-under-relation.ein) writes `(think Ann (question (p A)))` and
`(question (p B))`, and $H$ is `(p B)` alone.

- **(a)** Under an ordinary relation both are ordinary names. `(think Ann (absent (p A)))` is a fact whose inner
  relation is called `absent`; a premise `(think ?w (absent (p ?x)))` matches such a stored fact and tests no
  absence. **Default: your word in item 44.**
- **(b)** `open` is a plain fact there, and `absent` stays refused as an argument, as today: no premise then reads
  like a negation and is none.

## 4. The rule the defaults give

*The kernel reads `absent` at the head of a premise, and `open` at the head of a written fact or of a conclusion.
Directly under one of the two, the other is refused, but `(absent (open P))`, a premise over a fact. `absent` takes a
premise, `open` a relation's name or a fact. Under an ordinary relation both are ordinary names.* A written or
concluded `(absent P)` at the top stays refused: there it is the kernel's word, and no fact.

It gives each of your cells: `(absent (absent P))`, `(open (open P))` and `(open (absent P))` refused;
`(absent (open P))`, `(not (open P))`, `(open (not P))` and a written `(open P)` legal.

## 5. Your call

For 1, 2 and 3, a letter each; and whether § 4 is the rule. In [`ANSWERS.md`](../ANSWERS.md) under item 49. Blank
keeps the defaults.
