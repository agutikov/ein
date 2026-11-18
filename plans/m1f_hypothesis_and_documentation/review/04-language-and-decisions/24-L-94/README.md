# L-94 — Q-M1f.119's list of the ways a relation is the program's misses one

## In short, by example: what Ein does now, and what each choice changes

**Ein's behaviour does not change under any of the choices.** Every program below loads or is refused exactly as
today, with the same answer and the same exit code, whichever way you pick. The choices change **words only**:

- **L-94** changes the text of one decision, Q-M1f.119, the document that states the rule;
- **D-23** changes the text of one error message, the part after the dash;
- **§ 3.3** changes one sentence on each of two user pages.

L-94 exists because the decision's text and Ein disagree on one program, example 1. Ein is right and the text is
behind. The question is whether to bring the text up to date (a) or leave it as it is (b).

Every output below is today's binary, `ein solve --no-require-why`, kept in [`runs/now/`](runs/now/).

### Example 1: a relation written only inside a fact (L-94)

[`programs/fact-argument.ein`](programs/fact-argument.ein):

```lisp
(claims Carol (owns Bob Dog))
(query :goal (owns ?a ?b))
```

`owns` is never declared, never stated as a held fact and never in a rule. It appears only as the relation of the
fact `(owns Bob Dog)`, which is an argument of `claims`.

**What Ein does today, under (a), and under (b): it loads.** Exit 0, and the goal has no row, because Carol's claim
is not a held fact ([`runs/now/fact-argument.solve.txt`](runs/now/fact-argument.solve.txt)):

<!-- output -->
```text
  query             (owns ?a ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
exit 0
```

**What differs is only what the decision says about this program:**

| | Q-M1f.119's list of what makes a relation the program's | what the decision says of example 1 | what Ein does |
|---|---|---|---|
| **today, and under (b)** | *"one it declares, states a fact of, or writes in a rule's, an hrule's or an obligation's `:match` or `:assert`, … or passes where a relation's name is held; or one a `(solve :hrules …)` activator has its hrule propose"* | `owns` is none of these (the program states a fact of `claims`, not of `owns`), so **read as written, the goal should be refused** | loads |
| **under (a)** | the same, with **"writes as the relation of a fact that is an argument"** added | `owns` is the program's, so **it loads** | loads |

For contrast, the same program with the relation misspelt inside the fact
([`programs/fact-argument-misspelt.ein`](programs/fact-argument-misspelt.ein)):

```lisp
(claims Carol (own Bob Dog))
(query :goal (owns ?a ?b)
  :expect (model (owns Bob Dog)))
```

is **refused, exit 1, today and under every choice**
([`runs/now/fact-argument-misspelt.solve.txt`](runs/now/fact-argument-misspelt.solve.txt)):

<!-- output -->
```text
kb load error: programs/fact-argument-misspelt.ein:6:15: query: its :goal writes (owns ?a ?b), and the program names no relation owns — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of owns; …
exit 1
```

So it is the `(owns Bob Dog)` inside the claim that makes `owns` the program's. Ein already works this way, and (a)
writes it into the decision.

### Example 2: a misspelt relation, and the error message (D-23)

[`programs/named-nowhere.ein`](programs/named-nowhere.ein):

```lisp
(p a)
(query :goal (payed ?g))
```

**Refused, exit 1, today and under every choice.** Only the explanation after the dash differs.

Today, and with D-23 left as it is ([`runs/now/named-nowhere.solve.txt`](runs/now/named-nowhere.solve.txt)):

<!-- output -->
```text
kb load error: programs/named-nowhere.ein:6:15: query: its :goal writes (payed ?g), and the program names no relation payed
  — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule,
  so no state holds a fact of payed
```

With D-23 re-worded (a mock, [`runs/named-nowhere.solve.MOCK.line-reworded.txt`](runs/named-nowhere.solve.MOCK.line-reworded.txt)):

<!-- output -->
```text
kb load error: programs/named-nowhere.ein:6:15: query: its :goal writes (payed ?g), and the program names no relation payed
  — a relation of the program is one it declares, states a fact of, writes in a fact, a rule or an hrule,
  or passes where a relation's name is held, so no state holds a fact of payed
```

(Each is one line, broken here to fit.) Today's message lists three ways and leaves out two that Ein accepts: a relation written inside a fact (example 1)
and one passed to an activator (example 3). The re-worded message lists them. Example 1's misspelt program prints
the same message, so it changes the same way.

### Example 3: a relation passed to an activator (the pages, § 3.3)

[`programs/passed-to-an-activator.ein`](programs/passed-to-an-activator.ein):

```lisp
(person Ann)
(rule tag (?R) :match (person ?x) :assert (?R ?x) :why "every person is {?R}")
(tag happy)
(query :goal (happy ?x))
```

`happy` is written only as an argument of `(tag happy)`, which activates the rule `tag` with `?R = happy`.
**Today and under every choice it loads**, and the answer is `?x = Ann`, exit 0
([`runs/now/passed-to-an-activator.solve.txt`](runs/now/passed-to-an-activator.solve.txt)):

<!-- output -->
```text
  query             (happy ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = Ann
  as :expect        (model (happy Ann))
exit 0
```

Q-M1f.119 already names this way (*"passes where a relation's name is held"*). The user pages
[`12-query.md`](../../../../../docs/ein/user/03-syntax-and-semantics/12-query.md) and
[`02-syntax.md`](../../../../../docs/ein/user/04-reference/02-syntax.md) § Query do not: their list stops at
*"… written in a rule or an hrule, or proposed by an hrule a `(solve :hrules …)` activator turns on"*. § 3.3 adds
*"passed by an activator where a relation's name is held, `(symmetric next-to)`,"* to both.

### All three, at a glance

| program | Ein today | after any choice | which text the choices touch |
|---|---|---|---|
| example 1, `owns` only inside a fact | loads, exit 0 | the same | L-94: the decision's list (a) or not (b) |
| example 1 misspelt, `own` | refused, exit 1 | the same, and the message's last clause changes if D-23 is re-worded | D-23 |
| example 2, `payed` nowhere | refused, exit 1 | the same, and the message's last clause changes if D-23 is re-worded | D-23 |
| example 3, `happy` passed to an activator | loads, `?x = Ann` | the same | § 3.3: the two pages' list |

**Changing Ein's behaviour would be a different question, and no option here proposes one.** For example, making the
loader follow the decision's text as written today would refuse example 1.

## 1. The question

A query's goal and a claim may ask only about **relations of the program**; any other is refused at load, so that a
misspelt name is not answered as an empty question. Decision Q-M1f.119 lists what makes a relation the program's.
The loader takes one more way than the list names: a relation written only as **the relation of a fact that is an
argument**, `owns` in `(claims Carol (owns Bob Dog))`. Q-M1f.114 (the `(solve …)` lists) and the user's pages say so;
Q-M1f.119 does not. The refusal's own line also lists the ways, and leaves out the same one and more: that half is
[F23](../../../../followups/f23_diagnostics_in_eins_words.md#the-items)'s D-23. The question is the decision's re-wording;
this page also shows the line's.

Recorded at the milestone's [`findings.md`](../../../findings.md), row **L-94** (§ Decisions; P1f.22's N-6 and N-20),
and F23's row **D-23**.

## 2. What is so today

All runs use `build/review-bin/ein-head-17085b27`, today's binary, every P1f.23 stage merged (`ein solve
--no-require-why`, as D-23's witness was run); [`rerun.sh`](rerun.sh) makes every file. The sentences and the code, with their commands, are
[`runs/sentences-and-code.txt`](runs/sentences-and-code.txt).

### 2.1 The witness

[`programs/fact-argument.ein`](programs/fact-argument.ein), `(claims Carol (owns Bob Dog))` and
`(query :goal (owns ?a ?b))`, **loads**; the goal has no row, since Carol's claim is not held
([`runs/fact-argument.solve.txt`](runs/fact-argument.solve.txt)):

<!-- output -->
```text
  query             (owns ?a ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
```

With a claim listing `(owns Bob Dog)` ([`programs/fact-argument-claim.ein`](programs/fact-argument-claim.ein)) it
loads and `ein test` checks the claim, FAILED, exit 1 ([`runs/fact-argument-claim.test.txt`](runs/fact-argument-claim.test.txt)).
With the argument spelt `(own Bob Dog)` ([`programs/fact-argument-misspelt.ein`](programs/fact-argument-misspelt.ein))
the goal and the claim are refused, an `ERROR`, exit 2 ([`runs/fact-argument-misspelt.test.txt`](runs/fact-argument-misspelt.test.txt)).
So the fact's argument is what makes `owns` the program's.

### 2.2 Every way, run

| a relation written only … | program | goal over it | Q-M1f.119 names it | the refusal's line names it | 12-query.md / 02-syntax.md name it |
|---|---|---|---|---|---|
| as the relation of a fact that is an argument | [`fact-argument.ein`](programs/fact-argument.ein) | loads ([run](runs/fact-argument.solve.txt)) | **no** | **no** | yes |
| passed where a relation's name is held, `(tag happy)` with `tag (?R)` heading its conclusion | [`passed-to-an-activator.ein`](programs/passed-to-an-activator.ein) | loads, `?x = Ann` ([run](runs/passed-to-an-activator.solve.txt)) | yes | **no** | **no** |
| in a rule's `:match` | [`rule-match-only.ein`](programs/rule-match-only.ein) | loads ([run](runs/rule-match-only.solve.txt)) | yes | yes (*writes in a rule*) | yes |
| nowhere: `eq`, `neq` | [`eq-neq.ein`](programs/eq-neq.ein) | loads ([run](runs/eq-neq.solve.txt)) | yes | **no** | yes |
| nowhere: `is-a` | [`every-programs.ein`](programs/every-programs.ein) | **refused since Q-M1f.436**, as `payed` is ([run](runs/every-programs.solve.txt)); it loaded at the first look | **yes, still**: *"`is-a`, `eq`, `neq` … are every program's"* | no | `12-query.md` and `07-diagnostics.md`: refused, *`is-a` is one where the program writes it*; **`02-syntax.md`: every program's, still** |
| nowhere: `payed` | [`named-nowhere.ein`](programs/named-nowhere.ein) | **refused** ([run](runs/named-nowhere.solve.txt)) | — | — | — |

The refusal (D-23's witness, `(p a)` and `(query :goal (payed ?g))`), exit 1:

<!-- output -->
```text
kb load error: …/named-nowhere.ein:6:15: query: its :goal writes (payed ?g), and the program names no relation payed — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of payed
```

### 2.3 The sentences

[Q-M1f.119](../../../../../docs/decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md), lines 3–6:

> Every relation a query's `:goal` asks about, at any depth and under `(absent …)` and `(not …)` too, and every
> relation a claim lists a fact of, is a relation of the program: one it declares, states a fact of, or writes in a
> rule's, an hrule's or an obligation's `:match` or `:assert`, at any depth, or passes where a relation's name is
> held; or one a `(solve :hrules …)` activator has its hrule propose. `is-a`, `eq`, `neq` and the declarations,
> `(relation ?R ?T)`, are every program's.

[Q-M1f.114](../../../../../docs/decisions/language/Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)
(the solve lists) has it: *"the relation of any form in a rule's, an hrule's or an obligation's `:match` or `:assert`,
in a query's `:goal` **or in a fact's argument**"*. So do
[`12-query.md`](../../../../../docs/ein/user/03-syntax-and-semantics/12-query.md) (*"written as the relation of a fact
that is an argument"*), [`02-syntax.md` § Query](../../../../../docs/ein/user/04-reference/02-syntax.md) and
[`07-diagnostics.md` § 2.20 and § 2.21](../../../../../docs/ein/user/04-reference/07-diagnostics.md#221-a-goal-is-held-to-what-a-rules-match-is-held-to).

### 2.4 The code

`ein-ir/src/from_ir.rs` (lines in [`runs/sentences-and-code.txt`](runs/sentences-and-code.txt), read, not run):

- `program_relations` (line 707) starts from the names passed where a relation's name is held, adds `eq` and
  `neq` (no longer `is-a`, Q-M1f.436), the registry, the heads in every rule's, hrule's and obligation's `:match` and `:assert`, and **every head of
  every fact at any depth** (`relation_heads`, line 677, recurses into a fact's arguments). That recursion is the way
  Q-M1f.119 leaves out.
- `CLAIM_RELATION` (line 693), the refusal's list: *"a relation of the program is one it declares, states a fact of,
  or writes in a rule or an hrule"*. The solve lists' own (line 821): *"… or writes in a rule, an hrule or a goal"*.
- Pinned by `ein-cli/tests/goal_premises.rs` (line 158 holds the sentence; `a_goals_relation_is_refused_at_its_pattern`,
  `a_goal_over_a_relation_of_the_program_loads`).

## 3. The options, with the proposed re-wordings

### 3.1 The decision (L-94)

**(a) Re-word Q-M1f.119** to name the fact's argument. Proposed, lines 3–8, today's text with one clause added
(*"writes as the relation of a fact that is an argument,"*) and nothing else:

```markdown
Every relation a query's `:goal` asks about, at any depth and under `(absent …)` and `(not …)` too, and every
relation a claim lists a fact of, is a relation of the program: one it declares, states a fact of, writes as the
relation of a fact that is an argument, or writes in a rule's, an hrule's or an obligation's `:match` or `:assert`,
at any depth, or passes where a relation's name is held; or one a `(solve :hrules …)` activator has its hrule
propose. `eq`, `neq` and the declarations, `(relation ?R ?T)`, are every program's, and no theory's relation is:
`is-a` is a relation of the program only where it writes one, as any relation is (Q-M1f.436).
```

Its **Why** gains nothing: *"One notion of a relation of the program serves the `(solve …)` lists (Q-M1f.114), a
goal and a claim"* is then true of the two texts.

**(b) Keep it**, reading *"states a fact of"* as covering a fact inside a fact. Q-M1f.114 and the pages say the
argument explicitly, so the two decisions would read differently.

### 3.2 The line (D-23, F23's)

Today's: *"a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule"*. It
leaves out the fact's argument, the name passed where a relation's name is held, and the two every program has,
`eq` and `neq`.
Proposed (`CLAIM_RELATION`, and the two expected lines of `goal_premises.rs` with it):

```text
a relation of the program is one it declares, states a fact of, writes in a fact, a rule or an hrule, or passes where a relation's name is held
```

so that the witness's refusal reads:

<!-- output -->
```text
kb load error: …/named-nowhere.ein:6:15: query: its :goal writes (payed ?g), and the program names no relation payed — a relation of the program is one it declares, states a fact of, writes in a fact, a rule or an hrule, or passes where a relation's name is held, so no state holds a fact of payed
```

(a **mock**: [`runs/named-nowhere.solve.MOCK.line-reworded.txt`](runs/named-nowhere.solve.MOCK.line-reworded.txt),
today's output with the clause after the dash edited and nothing else). The solve lists' line (*"… or writes in a
rule, an hrule or a goal"*) has the same gap. `eq` and `neq` need no mention there, since a goal over them is
never refused.

### 3.3 The user pages (found while building)

`12-query.md` and `02-syntax.md` leave out the activator's way (*passes where a relation's name is held*), which
Q-M1f.119 names and the loader takes (§ 2.2, row 2). Proposed for both lists, after *"written in a rule or an hrule"*:
*"passed by an activator where a relation's name is held, `(symmetric next-to)`,"*.

## 4. The default

**None taken**: L-94 is *"The user's call: re-word the decision"*; D-23 is F23's, which holds no default for it.

## 5. Files

- [`programs/fact-argument.ein`](programs/fact-argument.ein) — the witness; [`runs/fact-argument.solve.txt`](runs/fact-argument.solve.txt).
- [`programs/fact-argument-claim.ein`](programs/fact-argument-claim.ein) — with a claim; [`runs/fact-argument-claim.test.txt`](runs/fact-argument-claim.test.txt).
- [`programs/fact-argument-misspelt.ein`](programs/fact-argument-misspelt.ein) — the argument misspelt, refused; [`runs/fact-argument-misspelt.test.txt`](runs/fact-argument-misspelt.test.txt).
- [`programs/passed-to-an-activator.ein`](programs/passed-to-an-activator.ein), [`programs/rule-match-only.ein`](programs/rule-match-only.ein), [`programs/eq-neq.ein`](programs/eq-neq.ein) — the other ways, each loading; their `runs/*.solve.txt`.
- [`programs/every-programs.ein`](programs/every-programs.ein) — goals over `is-a` and `neq` in a program that writes neither: refused today at the `is-a` goal; [`runs/every-programs.solve.txt`](runs/every-programs.solve.txt).
- [`rerun.sh`](rerun.sh) — every run and the sentences file.
- [`programs/named-nowhere.ein`](programs/named-nowhere.ein) — D-23's witness; [`runs/named-nowhere.solve.txt`](runs/named-nowhere.solve.txt) and the mock [`runs/named-nowhere.solve.MOCK.line-reworded.txt`](runs/named-nowhere.solve.MOCK.line-reworded.txt).
- [`runs/sentences-and-code.txt`](runs/sentences-and-code.txt) — both decisions' sentences, the four pages' sentences and the code, each with its command.

## 6. Re-run

```sh
bash plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/24-L-94/rerun.sh
```

It runs `build/review-bin/ein-head-17085b27 solve --no-require-why` on the six programs and `test -v` on the two
claim programs (exit code appended), and writes [`runs/sentences-and-code.txt`](runs/sentences-and-code.txt): the two
decisions' sentences, the three pages', and the code, each with its command and line numbers.

## 7. Found while building

- **Re-taken with `ein-head-17085b27`**: the fact's argument (L-94) and the two ways the refusal's line leaves out
  are as at the first look; the line itself is the same. **What moved is `is-a`**: since Q-M1f.436 (S1f.23.4) a goal
  over it in a program that writes none is refused, `every-programs.ein` exit 1 where it loaded at the first look.
  Q-M1f.114, `12-query.md` and `07-diagnostics.md` § 2.20 and § 2.21 were brought to it, and **Q-M1f.119 and
  `02-syntax.md` § Query too, by `d5fceff9`** (S1f.23.1's task 4, R-24), after this page's re-take: both now say
  *"no theory's relation is: `is-a` is one where the program writes it"* ([`runs/sentences-and-code.txt`](runs/sentences-and-code.txt),
  re-taken). So Q-M1f.119 today misses
  one way and names none too many; § 3.1's proposal is today's text with the one clause added.

- **The refusal's line misses three ways, not one**: the fact's argument (D-23), a name passed by an activator
  (`passed-to-an-activator.ein` loads), and `eq`/`neq` (every program's; `is-a` was among them at the first look). The last needs no words in a line
  that is only printed when the goal is refused, but the other two do.
- **The user pages miss the activator's way** that Q-M1f.119 names: `12-query.md` and `02-syntax.md` list *declared,
  stated, written as the relation of a fact that is an argument, written in a rule or an hrule, or proposed by an
  hrule a `(solve :hrules …)` activator turns on*; `07-diagnostics.md` § 2.21 covers it only through *"§ 2.20's
  notion without the goals"*. So the decision and the pages each miss one way, a different one.

## 8. Your call

- **L-94**: (a) re-word Q-M1f.119 to add *"writes as the relation of a fact that is an argument"* (§ 3.1); (b) keep it.
  Its `is-a` clause is already today's (§ 7).
- **D-23** (F23's, shown here): the line re-worded (§ 3.2), or as it is.
- The pages' missing activator clause (§ 3.3), with either.
- **Default**: none taken.

What each choice changes, and what it leaves, is § 9.

## 9. What each choice leads to

**No choice here changes what loads, an answer or an exit code.** The loader already takes every way in § 2.2
(`relation_heads` recurses into a fact's arguments). The three choices are about which texts say so, and are
independent: any combination can be taken.

### 9.1 L-94: the decision

| | (a) re-word Q-M1f.119 | (b) keep it |
|---|---|---|
| **What is edited** | one decision file, one clause (§ 3.1) | nothing |
| **Code, tests, goldens** | none | none |
| **The texts of one notion** | Q-M1f.114 (the solve lists), Q-M1f.119 (goals and claims), `12-query.md`, `02-syntax.md` and `07-diagnostics.md` § 2.20 and § 2.21 all name the fact's argument; Q-M1f.119's *Why*, *"one notion … serves the `(solve …)` lists, a goal and a claim"*, is true of the texts | Q-M1f.114 and the pages name it, Q-M1f.119 does not: two decisions give two lists of one notion, and Q-M1f.119's *Why* is true of the code only |
| **What a reader of the decision concludes about the witness** (`(claims Carol (owns Bob Dog))`, `(query :goal (owns ?a ?b))`) | it loads, as it does | it loads only if *"states a fact of"* covers a fact written as another fact's argument. In the project's words it does not: the program states a fact of `claims`, and `(owns Bob Dog)` is a fact in `claims`' context, which no state holds (Q-M1f.394). Read that way, the decision says the witness is refused, and the engine loads it |
| **What guards the way** | the decision: a loader that stopped taking a fact's argument would contradict Q-M1f.119 as well as Q-M1f.114 and the pages | Q-M1f.114 and the pages only |
| **Tests** | none needed for the wording. `goal_premises.rs`' `a_goal_over_a_relation_of_the_program_loads` has 14 shapes and none is a relation written only as a fact's argument; a 15th would pin the way the decision then names | as today |

### 9.2 D-23: the refusal's line

| | the line re-worded (§ 3.2) | as it is |
|---|---|---|
| **What is edited** | `from_ir.rs`' `CLAIM_RELATION` (line 693), and `RELATION` (line 821), the solve lists' line with the same gap, if it goes with it | nothing |
| **What moves with it** (grep today) | `CLAIM_RELATION`, which a goal's, a region claim's and an `:expect`'s refusal share: `goal_premises.rs:158`'s expected text, two `refused` texts in `corpus/regressions.toml` (`goal-no-such-relation`, `goal-absent-no-such-relation`), `examples/broken/load/expect_unknown_relation.expected`, and three rows of `07-diagnostics.md` (lines 212, 215, 730), the normative page, which a change to a refusal's text changes in the same commit. The solve lists' line: three `refused` texts in `regressions.toml` (`solve-hrules-…`, `solve-hypothesis-relations-…`, `solve-no-hypothesis-no-such-relation`) and two rows of `07-diagnostics.md` (683, 684). The shape golden's lines for those refused programs that digest the message, re-banked | nothing |
| **What the user sees** | the line after the dash lists the fact's argument and a name passed where a relation's name is held; it is still a summary (it leaves out *an obligation* and *an activator's hrule proposes*, which the decision names) | the line lists three ways. The ones it leaves out never lead to the refusal, since a goal over such a relation loads, so the line never contradicts what happens to a program; it only teaches less: a user fixing `(payed ?g)` is not told that writing `payed` inside a fact or passing it to an activator would make it the program's |
| **F23** | D-23 done | D-23 stays open in F23 |

### 9.3 The pages' activator clause (§ 3.3)

| | added | left |
|---|---|---|
| **What is edited** | `12-query.md` (line 212) and `02-syntax.md` § Query (line 681), one clause each; docs only, the link check | nothing |
| **Effect** | the user's pages list every way Q-M1f.119 lists (`07-diagnostics.md` § 2.21 already covers it through § 2.20's notion) | a reader of the two pages takes `(symmetric next-to)` alone as not making `next-to` the program's, while the loader takes it (§ 2.2, row 2). The decision and the pages keep missing one way each, a different one, unless (a) is taken too, after which only the pages miss one |

### 9.4 Taken together

- **(a), the line re-worded, the clause added**: every text that lists the ways names the same ones the loader takes.
  The line remains a shorter summary. One decision edit, one page edit pair, and one code change with its pinned
  texts.
- **(b), the line as it is, no clause**: nothing changes. Q-M1f.119 misses the fact's argument, the pages miss the
  activator, and the line misses both. The pages and the line only say less than the loader takes; Q-M1f.119, read
  in the project's words (§ 9.1), says the witness is refused, which the loader does not do.
