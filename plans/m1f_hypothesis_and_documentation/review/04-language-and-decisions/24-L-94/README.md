# L-94 — Q-M1f.119's list of the ways a relation is the program's misses one

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

**(a) Re-word Q-M1f.119** to name the fact's argument. Proposed, lines 3–6:

```markdown
Every relation a query's `:goal` asks about, at any depth and under `(absent …)` and `(not …)` too, and every
relation a claim lists a fact of, is a relation of the program: one it declares, states a fact of, writes as the
relation of a fact that is an argument, or writes in a rule's, an hrule's or an obligation's `:match` or `:assert`,
at any depth, or passes where a relation's name is held; or one a `(solve :hrules …)` activator has its hrule
propose. `eq`, `neq` and the declarations, `(relation ?R ?T)`, are every program's.
```

(the last sentence also brought to today's loader: `is-a` is no longer every program's, § 7), and its **Why** gains
nothing: *"One notion of a relation of the program serves the `(solve …)` lists (Q-M1f.114), a
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
  Q-M1f.114, `12-query.md` and `07-diagnostics.md` § 2.20 and § 2.21 were brought to it; **Q-M1f.119 and
  `02-syntax.md` § Query were not**: the decision still says *"`is-a`, `eq`, `neq` and the declarations … are every
  program's"* and the reference page *"`is-a`, `eq`, `neq` and `relation` are every program's"*
  ([`runs/sentences-and-code.txt`](runs/sentences-and-code.txt)). So Q-M1f.119 now misses one way and names one too
  many; the proposed re-wording (§ 3.1) drops `is-a`.

- **The refusal's line misses three ways, not one**: the fact's argument (D-23), a name passed by an activator
  (`passed-to-an-activator.ein` loads), and `eq`/`neq` (every program's; `is-a` was among them at the first look). The last needs no words in a line
  that is only printed when the goal is refused, but the other two do.
- **The user pages miss the activator's way** that Q-M1f.119 names: `12-query.md` and `02-syntax.md` list *declared,
  stated, written as the relation of a fact that is an argument, written in a rule or an hrule, or proposed by an
  hrule a `(solve :hrules …)` activator turns on*; `07-diagnostics.md` § 2.21 covers it only through *"§ 2.20's
  notion without the goals"*. So the decision and the pages each miss one way, a different one.

## 8. Your call

- **L-94**: (a) re-word Q-M1f.119 to add *"writes as the relation of a fact that is an argument"* (§ 3.1); (b) keep it.
  Either way its `is-a` clause is stale since Q-M1f.436, as is `02-syntax.md`'s (§ 7).
- **D-23** (F23's, shown here): the line re-worded (§ 3.2), or as it is.
- The pages' missing activator clause (§ 3.3), with either.
- **Default**: none taken.
