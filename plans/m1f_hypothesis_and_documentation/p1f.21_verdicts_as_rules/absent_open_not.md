# `absent`, `open` and `not`, each inside each: what today's binary does, and where it is defined

[← the phase](README.md) · [the runs](absent_open_not/run.py)

**Asked** by the user, 2026-11-19, answering review item 32, verbatim:

> We have to analyze and strictly define interaction between absent, open and not. not is just an ordinary
> relation, absent is kernel primitive, open is both. draw a table 3x3 of now they behave together and where it is
> defined (not (not P)) is simple, (absent (absent P)) probably compilation error, (open (absent P)) also
> compilation error. While (absent (open P)) possible. Maybe kernel primitives can't be nested because it's
> argument always a fact?

**Every cell below is a run.** `python3 absent_open_not/run.py` writes the programs and prints one line each, on
`build/review-bin/ein-head-946ee788`, the gate build of `946ee788`. A form is tried in three places: as a premise
of a rule's `:match`, as its `:assert`, and as a fact the program writes.

## The answer, short

- **Two of your four cells are not what the binary does today.** `(absent (absent P))` loads, as a premise, and is
  true where P is held. `(absent (open P))` is a parse error, as every form with `open` over a fact is: `open` is
  read in one place, a rule's whole `:assert`, with a relation or nothing after it.
- **No, the rule is not that a primitive's argument is always a fact.** `absent`'s argument is a premise, which may
  be a conjunction and may hold another `absent`: that is how *every* is written, `(absent (and G (absent B)))`,
  the standard library's `forall`.
- **Two rules that are already Ein's give the whole table**: `absent` is no fact, so it stands under no fact;
  and a fact's argument is a fact a state could hold. Once `(open P)` is a fact, they fill every cell but two,
  which are yours ([below](#what-is-yours-to-rule)).

## 1. Each alone, today

| | a premise | a conclusion | a fact the program writes |
|---|---|---|---|
| `(not P)` | loads; matches a stored negation that is held, `(not (p A))`, and nothing else | loads; the state holds `(not (p A))` | loads; held |
| `(absent P)` | loads; true where no `(p ?x)` is held at the boundary | parse error, *unexpected `absent`* | parse error |
| `(open P)` | parse error, *unexpected `open`* | parse error: after `open` a rule writes nothing or a relation, `(open)` or `(open ?R)` | parse error |

## 2. One inside another, today

Rows are the outer form, columns the inner. *M* a premise, *A* a conclusion, *F* a written fact.

| outer ↓ · inner → | `not` | `absent` | `open` |
|---|---|---|---|
| **`not`** | **a fact**, in all three places. `(not (not (p A)))` is held as written; it is not `(p A)`, and a premise `(not (not (p ?x)))` matches it and nothing else. `std.negation` eliminates it where a program imports that | parse error, M, A and F | parse error, M, A and F |
| **`absent`** | M: loads; true where no `(not (p ?x))` is held. A, F: parse error, as `absent` alone | M: **loads**; true where `(p ?x)` **is** held, an absence of an absence. A, F: parse error | parse error, M, A and F: `open` is read nowhere in a pattern |
| **`open`** | parse error, M, A and F | parse error, M, A and F | parse error, M, A and F |

## 3. Where each is defined

| | the grammar (`ein-ir/src/ein.y`) | the decision |
|---|---|---|
| `not` | a fact, `(not F)` with F a fact, a negation or an `eq` fact; a premise, `(not …)` of a pattern | a fact may hold a fact as an argument ([Q40](../../../docs/decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md)); the negation of a negation is a fact ([Q-M1f.391](../../../docs/decisions/language/Q-M1f.391-the-negation-of-a-negation-is-a-fact.md)) |
| `absent` | one production of a premise, `(absent Premise)`, and one of a claim | `absent` heads a premise and a member of a claim, nothing else ([Q-M1f.122](../../../docs/decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-member-of-a-claims.md)); it is read at its own boundary ([Q-M1e.18](../../../docs/decisions/language/Q-M1e.18-eq-neq-and-absent-are-read-only-at-their-own.md)) |
| `open` | one production, a rule's whole `:assert`: `(open)` or `(open R)`; a reserved word everywhere else | `open` is written only as a rule's verdict ([Q-M1f.66](../../../docs/decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)) |

And the standing rule of `AGENTS.md` § The language: *a fact that is an argument is one a state could hold: a fact
of an ordinary relation or its negation; `absent` with an argument is never a nested head*.

## 4. Your four cells against today

| you | today |
|---|---|
| `(not (not P))` is simple | so: a fact |
| `(absent (absent P))` probably a compilation error | **loads**, as a premise, and means *P is held*. With a guard between the two it is the universal, `(absent (and G (absent B)))`: the standard library's macro `forall`, its test, and one example program write it (three corpus files); none writes the direct form |
| `(open (absent P))` a compilation error | so; and so is every `(open …)` over a fact today |
| `(absent (open P))` possible | a parse error today; possible once `(open P)` is a fact a state holds |

## 5. Can a kernel primitive not be nested, its argument being always a fact?

For `open`, yes. For `absent`, no, and the difference is what kind of thing each is.

- **`absent` is no fact.** It is a connective of a premise, like `and`. So it stands under no fact: not under
  `not`, not under `open`, not as any argument, and never in a conclusion. That is the middle column.
- **`absent`'s own argument is a premise, not a fact**: a pattern, or a conjunction with variables of its own,
  `(absent (and (is-a ?b Chair) (seats ?a ?b)))`, *there is no chair ?a is seated at*, which is how the standard
  library's four obligations and its `forall` are written. A premise may hold an `absent`, so `absent` nests in
  itself and in nothing else.
- **`open` and `not` make facts**, and a fact's argument is a fact a state could hold. So neither takes an
  `absent`; each takes the other, and itself.

## 6. The table those two rules give, once `(open P)` is a fact

Your ruling of the same answer: *"inside ein we just have (open H). btw, absent see it the same way as other
facts."* With that, and `not` an ordinary relation
([Q-M1f.557](../open_questions.md#q-m1f557--how-the-contradictions-rules-reach-every-relation-and-count)):

| outer ↓ · inner → | `not` | `absent` | `open` |
|---|---|---|---|
| **`not`** | a fact, as today | refused, as today | a fact like any other; the kernel reads nothing in it |
| **`absent`** | a premise, as today | a premise, as today, the universal's inner half; **the direct form is yours to rule** | a premise, *P is not asked*; new |
| **`open`** | a hypothesis that a negation holds; new | refused, as today | **yours to rule**: a hypothesis about a hypothesis |

### What is yours to rule

Put as [Q-M1f.568](../open_questions.md#q-m1f568--absent-open-and-not-nested-the-cells-that-are-the-users), and shown
cell by cell on programs in [review item 46](review/part-2/46-Q-M1f.568/README.md).

1. **`(absent (absent P))` written directly.** (a) Refused at the parse, as you expect, the form with a conjunction
   between kept, since it is the universal. (b) Left: it means *P is held*, binding nothing outside. **Default
   (a)**: no corpus program writes it.
2. **`(open (open P))`**: a hypothesis that P may be supposed. It is a hypothesis that depends on another, which
   you parked ([V-21](findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)). **Default: refused
   at load**, until that is taken up.
3. **`(not (open P))` and `(open (not P))`**: legal by the two rules. The first is an ordinary fact the kernel
   does not read; whether it is how a program says *do not suppose P* is a rule's to say, not the kernel's.
   **Default: legal, with no meaning to the kernel.**
4. **`(open P)` as a fact a program writes**, a candidate given by hand. **Default: legal**, since it is a fact.
