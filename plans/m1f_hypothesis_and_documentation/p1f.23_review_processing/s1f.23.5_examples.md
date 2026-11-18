# S1f.23.5, task 1 — a `:why` reads its own form, shown on real programs

**Stage:** [S1f.23.5](s1f.23.5_a_why_reads_its_own_form.md), task 1, of [P1f.23](README.md). It builds nothing and
stops for the user's look.

**Today** is every line quoted from the binary `build/review-bin/ein-head-b5fb8d70` (`ein 0.1.0`, built from
`b5fb8d70`; no commit since has touched `ein.rs/`, `stdlib/`, `examples/`, `tests/` or `corpus/`), run on the
programs in [`s1f.23.5_examples/`](s1f.23.5_examples/), and marked *output*. **After** is the ruling applied by hand
and marked ***by hand, not built***.

## The ruling, applied

A `:why` reads the form it is written on, and `{?n}` is that form's n-th arrow, counting the relation as 1. A
declaration is the fact `(relation owns Person Pet)`: arrow 1 at `relation`, 2 at `owns`, 3 at `Person`, 4 at `Pet`.

| | today (*output*, [`01-owns.ein`](s1f.23.5_examples/01-owns.ein)) | after (***by hand, not built***) |
|---|---|---|
| the text | `(relation owns Person Pet :why "{?1} owns a {?2}")` | renumbered: `:why "{?3} owns a {?4}"`; as the user wrote it: `:why "a {?3} {?2} a {?4}"` |
| the declaration reads | no surface prints it | renumbered: *Person owns a Pet*; the user's: *a Person owns a Pet* |
| the fact `(owns Ann Dog)` reads | ``Derives `owns(Ann, Dog)` — Ann owns a Dog`` | no template, the fallback: ``Derives `owns(Ann, Dog)`.`` |

The renumbering is mechanical: every `{?k}` of a relation's `:why`, k a number, becomes `{?k+2}`, and the words stay.

## What it comes to

- **The trace's statement goes.** A fact's *statement*, the relation's `:why` filled with the fact's arguments, is
  printed in one place today: after a fact a `--trace` step derives. After, no derived fact has a template. Over
  the root traces of every corpus program, **162 650 of the 216 429 conclusions lose their sentence, and none gains
  one** (§ 2.6).
- **What stays:** a rule's `:why` (the step's quote, the box of the step's drawing), a fact's `:source`, a query's
  `:goal-text`, and the obligations' lines (the read-out's `owed` and `owing`, the JSON summary, the `owe` event).
  After, a rule's `:why` is all the English a trace step has.
- **The declaration's own reading is printed nowhere**, today or after, unless a surface is chosen for it: open
  point 3, whose default is the trace's premise line (1 949 premises of those traces are facts of `relation`).
- **2 036 texts are renumbered** in the four corpus roots: 359 of the standard library's 360 relations, 1 677 of
  the corpus's 1 797. 1 855 more `:why` strings with a `{?k}` sit in the crates' sources and tests, their goldens
  and the pages (§ 4).
- **Many renumbered texts read stiffly as declarations**, since their words were written for a fact: *T is a T*,
  *House is painted Color*, *the number written Number, then the digit Digit*. Open point 9.

## 1. What reads a `:why` today

Each surface the binary has, one line of it from [`01-owns.ein`](s1f.23.5_examples/01-owns.ein) or
[`02-zebra2.ein`](s1f.23.5_examples/02-zebra2.ein) (*output*), and what the ruling does to it (***by hand, not
built***).

| surface | the text it reads | one line, today (*output*) | after |
|---|---|---|---|
| `--trace`, a step's *Derives* line | a derived fact's **statement**: its relation's `:why`, `{?k}` its k-th argument; a firing of several conclusions names each (Q-M1f.214) | ``Derives `owns(Ann, Dog)` — Ann owns a Dog`` | ``Derives `owns(Ann, Dog)`.`` — no statement |
| `--trace`, a step's quote | the rule's `:why`, filled by the firing | `> Ann wants a Dog, so Ann owns one` | unchanged |
| `--trace`, the premises line | a given's `:source` | ``Premises: `wants(Ann, Dog)` — from Ann wants a dog`` | unchanged; a declaration's reading added (open point 3) |
| `--trace`, a step's drawing (no `-G`) | the rule's `:why`, in the rule's box | `label="wanting-is-owning\nAnn wants a Dog, so Ann owns one"` | unchanged (S1f.23.1 redraws the box) |
| the read-out, a query's row | the query's `:goal-text` | `“Ann owns the dog”` | unchanged |
| the read-out, `owed` and `owing` | an obligation rule's `:why` | `owed            color-loc owes Red a House.` | unchanged |
| `--json-summary` | the same obligation texts, `region.root_owed` | `"color-loc owes Red a House."` | unchanged; no relation's `:why`, today or after |
| `--events` | the `owe` event's `why`, an obligation rule's | `"why": "color-loc owes Red a House."` | unchanged |
| `ein render kb` | an arrow's label: a given's `:source`, a derived fact's rule *name* | `"Ann" -> "Dog" [label="owns by wanting-is-owning", …]` | no `:why`, today or after |
| `ein render program` | the query's `:goal-text` | `label=":goal (owns ?who Dog)\n:goal-text \"{?who} owns the dog\""` | unchanged |
| `ein saturate --dump` | `:source` and `:goal-text`, quoted | `(wants Ann Dog :source "Ann wants a dog")` | unchanged |
| a `--dump-answer` state file | the program as written, every text byte for byte | `(relation owns      Person Pet  :why "{?1} owns a {?2}")` | the renumbered text, as written |
| the provenance comment of `ein_render::dump::serialise` | `; rule r: <reason> — <statement>` | no command prints it; a test reads it, `ein-render/tests/corpus_ops/mod.rs` | no statement |
| the KB-shape digest, `ein-core/src/shape.rs` | each relation's `:why`, raw, on its `REL … why=` line | a test instrument, digested in `corpus_shapes.md5` | the renumbered text: every corpus program that declares a relation moves its `load` line |
| the narration, `ein-narration` | none: it calls the rendered trace narration and reads no text | — | — |
| `utils/nl_census.py` | its *statement* column counts a fact whose relation has a `:why` | `examples/beliefs/02_packing.ein  1  0  —  —  —` (0 facts read, below) | the column counts declarations alone |

Two things this table met that are outside the stage, for the phase's findings: Q-M1f.12 lists `--events` and the
JSON summary among the surfaces that carry no text, while both carry an obligation's `:why`, filled; and
`nl_census.py`'s answer half reads the facts of the state files `--dump-answer` writes, which hold none since a saved
state is its program and its choices (Q-M1f.392), so it reads 0 facts on every entry.

**The fallback ladder as it is**, [`14-natural-language.md`
§ 3](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md#3-a-fact-in-english-the-fallback-ladder),
quoted:

> A fact has two texts, from two places: its **statement**, what it says: its relation's `:why`, filled with its
> arguments; its **reason**, why it holds: read off its **first recorded justification**. […] A fact reads from the
> program's texts where it has one: its `:source`, its rule's `:why`, its relation's `:why`. Where it has none it
> reads as its s-expression, `(R a b)`, and **never as invented prose**.

After (***by hand, not built***), the ladder's last rung but one, *its relation's `:why`*, is reached by a
declaration alone: every other fact reads by its `:source`, its rule's `:why`, or the fallback.

## 2. Real programs, today and after

Each block: the program's relations, their `:why` today and renumbered, what the declaration reads after and what one
fact of it read today and reads after; then steps of `ein solve FILE --trace T.md -G`, today and after side by side
(a line the ruling leaves as it was says *same*). The *after* takes § 3's defaults: the premise line prints a
declaration's reading (open point 3), and a fact with no text prints as the trace prints one today (open point 6).

### 2.1 The user's example — `01-owns.ein`

[`01-owns.ein`](s1f.23.5_examples/01-owns.ein), new: `owns`, `wants` and `counts-as`; a rule that derives
`(owns Ann Dog)`, and one, `owner-kind`, that reads the declaration `(relation owns ?A ?B)` as a premise.

| relation | `:why` today | renumbered | the declaration reads after | a fact, today (*output*) | after |
|---|---|---|---|---|---|
| `owns` (Person Pet) | `{?1} owns a {?2}` | `{?3} owns a {?4}`; the user's `a {?3} {?2} a {?4}` | Person owns a Pet; the user's: a Person owns a Pet | `` `owns(Ann, Dog)` — Ann owns a Dog `` | `owns(Ann, Dog)` |
| `wants` (Person Pet) | `{?1} wants a {?2}` | `{?3} wants a {?4}` | Person wants a Pet | stated, never derived: no surface prints its statement | the same |
| `counts-as` (Thing Kind) | `{?1} counts as a {?2}` | `{?3} counts as a {?4}` | Thing counts as a Kind | `` `counts-as(Bob, Person)` — Bob counts as a Person `` | `counts-as(Bob, Person)` |

| today (*output*) | after (***by hand, not built***; `owns` written as the user wrote it) |
|---|---|
| `` ### Step 1 — `wanting-is-owning` `` | same |
| `> Ann wants a Dog, so Ann owns one` | same |
| ``Premises: `wants(Ann, Dog)` — from Ann wants a dog`` | same |
| ``Derives `owns(Ann, Dog)` — Ann owns a Dog`` | ``Derives `owns(Ann, Dog)`.`` |
| `` ### Step 2 — `owner-kind` `` | same |
| `> owns is declared between Person and Pet, and Bob owns Cat, so Bob counts as a Person` | same |
| ``Premises: `relation(owns, Person, Pet)`, `owns(Bob, Cat)` `` | ``Premises: `relation(owns, Person, Pet)` — a Person owns a Pet, `owns(Bob, Cat)` `` |
| ``Derives `counts-as(Bob, Person)` — Bob counts as a Person`` | ``Derives `counts-as(Bob, Person)`.`` |

The read-out's row, `“Ann owns the dog”`, is the query's `:goal-text`, and stays.

### 2.2 The Zebra puzzle — `02-zebra2.ein`

[`02-zebra2.ein`](s1f.23.5_examples/02-zebra2.ein), a copy of `examples/zebra2.ein` under a one-line header. A step
of root's derivation, and a state's reading: the trace's section for the one state of R.

| relation | `:why` today | renumbered | the declaration reads after | a fact, today (*output*) | after |
|---|---|---|---|---|---|
| `is-a` (T T) | `{?1} is a {?2}` | `{?3} is a {?4}` | T is a T | stated, never derived | the same |
| `is-a*` (T T) | `{?1} is a {?2} (transitively)` | `{?3} is a {?4} (transitively)` | T is a T (transitively) | `` `is-a*(House, Attribute)` — House is a Attribute (transitively) `` | `is-a*(House, Attribute)` |
| `color-loc` (Color House) | `{?2} is painted {?1}` | `{?4} is painted {?3}` | House is painted Color | `` `color-loc(Blue, House-2)` — House-2 is painted Blue `` | `color-loc(Blue, House-2)` |
| `nation-loc` (Nationality House) | `the {?1} lives in {?2}` | `the {?3} lives in {?4}` | the Nationality lives in House | `` `nation-loc(Norwegian, House-1)` — the Norwegian lives in House-1 `` | `nation-loc(Norwegian, House-1)` |
| `drink-loc` (Drink House) | `{?1} is drunk in {?2}` | `{?3} is drunk in {?4}` | Drink is drunk in House | `` `drink-loc(Water, House-1)` — Water is drunk in House-1 `` | `drink-loc(Water, House-1)` |
| `smoke-loc` (Cigarette House) | `{?1} is smoked in {?2}` | `{?3} is smoked in {?4}` | Cigarette is smoked in House | `` `smoke-loc(Kools, House-1)` — Kools is smoked in House-1 `` | `smoke-loc(Kools, House-1)` |
| `pet-loc` (Pet House) | `the {?1} is kept in {?2}` | `the {?3} is kept in {?4}` | the Pet is kept in House | `` `pet-loc(Horse, House-2)` — the Horse is kept in House-2 `` | `pet-loc(Horse, House-2)` |
| `right-of` (House House) | `{?1} is immediately right of {?2}` | `{?3} is immediately right of {?4}` | House is immediately right of House | stated, never derived | the same |
| `next-to` (House House) | `{?1} is next to {?2}` | `{?3} is next to {?4}` | House is next to House | `` `next-to(House-2, House-1)` — House-2 is next to House-1 `` | `next-to(House-2, House-1)` |

| today (*output*, root) | after (***by hand, not built***) |
|---|---|
| `` ### Step 9 — `includes` `` | same |
| `> is-a* includes is-a: from House →is-a→ Attribute, derive House →is-a*→ Attribute.` | same |
| ``Premises: `is-a(House, Attribute)` `` | same |
| ``Derives `is-a*(House, Attribute)` — House is a Attribute (transitively)`` | ``Derives `is-a*(House, Attribute)`.`` |
| `` ### Step 231 — `std.bijection.domain-elimination` `` | same |
| `> every House except House-1 excluded for Norwegian → (nation-loc Norwegian House-1).` | same |
| ``Premises: `std.algebra.functional(nation-loc)`, `std.algebra.total(nation-loc)`, `relation(nation-loc, Nationality, House)`, `is-a(Norwegian, Nationality)`, `is-a(House-1, House)` `` | ``Premises: `std.algebra.functional(nation-loc)`, `std.algebra.total(nation-loc)`, `relation(nation-loc, Nationality, House)` — the Nationality lives in House, `is-a(Norwegian, Nationality)`, `is-a(House-1, House)` `` |
| ``Derives `nation-loc(Norwegian, House-1)` — the Norwegian lives in House-1`` | ``Derives `nation-loc(Norwegian, House-1)`.`` |
| `` ### Step 235 — `adjacent-via-fwd` `` | same |
| `> (nation-loc Norwegian House-1) and unique-next-to neighbour House-2 ⟹ (color-loc Blue House-2).` | same |
| ``Premises: `nation-loc(Norwegian, House-1)`, `next-to(House-2, House-1)` — from condition (10)`` | same |
| ``Derives `color-loc(Blue, House-2)` — House-2 is painted Blue`` | ``Derives `color-loc(Blue, House-2)`.`` |

| today (*output*, the state) | after (***by hand, not built***) |
|---|---|
| `## State 1 of 1 — 18 candidates · minimal and maximal · 261 982 commitments, 63 minimal — 244 steps` | same |
| `Assuming **(drink-loc Juice House-4)**.` | same |
| `` ### Step 9 — `co-located` `` | same |
| `> co-located: (drink-loc Juice House-4) ⟹ (smoke-loc Lucky_Strike House-4).` | same |
| ``Premises: `drink-loc(Juice, House-4)` `` | same |
| ``Derives `smoke-loc(Lucky_Strike, House-4)` — Lucky_Strike is smoked in House-4`` | ``Derives `smoke-loc(Lucky_Strike, House-4)`.`` |

The read-out's `owed` rows (`color-loc owes Red a House.`, `std.algebra.total-owed`'s `:why`) and the query's row
(`“The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”`) stay. In this program's root trace,
171 of 428 conclusions carry a statement today, and 91 of 659 premises are facts of `relation`.

### 2.3 A calculator program — `03-sum.ein`

[`03-sum.ein`](s1f.23.5_examples/03-sum.ein), new: the second request of `examples/arith/add.ein` alone, 17 + 5
named `apples` and `pears`. The program declares no relation: every statement its trace prints is a standard library
relation's.

| relation | `:why` today | renumbered | the declaration reads after | a fact, today (*output*) | after |
|---|---|---|---|---|---|
| `std.math.decimal.named` (Name) | `{?1} is a named constant` | `{?3} is a named constant` | Name is a named constant | `` `std.math.decimal.named(apples)` — apples is a named constant `` | `std.math.decimal.named(apples)` |
| `std.math.decimal-arith.add-walk` (Tag Number Number Digit Digits) | `adding, for {?1}: {?2} and {?3} are still to add, the carry is {?4}, and the digits so far are {?5}` | `adding, for {?3}: {?4} and {?5} are still to add, the carry is {?6}, and the digits so far are {?7}` | adding, for Tag: Number and Number are still to add, the carry is Digit, and the digits so far are Digits | step 14 below | step 14 below |
| `std.math.decimal-arith.done` (Tag Number) | `the answer for {?1} is {?2}` | `the answer for {?3} is {?4}` | the answer for Tag is Number | step 24 below | step 24 below |
| `std.math.decimal.valid` (Number) | `{?1} is a decimal number` | `{?3} is a decimal number` | Number is a decimal number | step 24 below | step 24 below |
| `sum-of` (Operand Operand Number) | `{?1} + {?2} = {?3}` | `{?3} + {?4} = {?5}` | Operand + Operand = Number | step 25 below | step 25 below |

| today (*output*) | after (***by hand, not built***) |
|---|---|
| `` ### Step 4 — `std.math.positional.let-named` `` | same |
| `> apples is the name of a number` | same |
| ``Premises: `let(apples, n(n(z, 1), 7))` `` | same |
| ``Derives `std.math.decimal.named(apples)` — apples is a named constant`` | ``Derives `std.math.decimal.named(apples)`.`` |
| `` ### Step 14 — `std.math.positional.add-start` `` | same |
| `> add (n (n z 1) 7) and (n z 5) from the units, with no carry in` | same |
| ``Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.add-second(apples, pears, n(n(z, 1), 7)), n(z, 5))` `` | same |
| ``Derives `std.math.decimal-arith.add-walk(std.math.decimal-arith.sum(apples, pears), n(n(z, 1), 7), n(z, 5), 0, z)` — adding, for (std.math.decimal-arith.sum apples pears): (n (n z 1) 7) and (n z 5) are still to add, the carry is 0, and the digits so far are z`` | ``Derives `std.math.decimal-arith.add-walk(std.math.decimal-arith.sum(apples, pears), n(n(z, 1), 7), n(z, 5), 0, z)`.`` |
| `` ### Step 24 — `std.math.positional.turn-end` `` | same |
| `> turned round: the answer is (n (n z 2) 2), a number as it is made` | same |
| ``Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.sum(apples, pears), z, n(n(z, 2), 2))` `` | same |
| ``Derives `std.math.decimal-arith.done(std.math.decimal-arith.sum(apples, pears), n(n(z, 2), 2))` — the answer for (std.math.decimal-arith.sum apples pears) is (n (n z 2) 2); `std.math.decimal.valid(n(n(z, 2), 2))` — (n (n z 2) 2) is a decimal number`` | ``Derives `std.math.decimal-arith.done(std.math.decimal-arith.sum(apples, pears), n(n(z, 2), 2))`; `std.math.decimal.valid(n(n(z, 2), 2))`.`` |
| `` ### Step 25 — `std.math.positional.add-answer` `` | same |
| `> apples + pears = (n (n z 2) 2)` | same |
| ``Premises: `std.math.decimal-arith.done(std.math.decimal-arith.sum(apples, pears), n(n(z, 2), 2))` `` | same |
| ``Derives `sum-of(apples, pears, n(n(z, 2), 2))` — apples + pears = (n (n z 2) 2)`` | ``Derives `sum-of(apples, pears, n(n(z, 2), 2))`.`` |

Here the rule's quote often says what the statement said (step 25), so little is lost; in step 14 the statement is the
only line that names the walk's state. The read-out's row, `“there are (n (n z 2) 2) pieces of fruit”`, stays. The
calculator's relations carry most of the statements the corpus prints (§ 2.6).

### 2.4 A fact under an arrow — `04-packing.ein`

[`04-packing.ein`](s1f.23.5_examples/04-packing.ein), a copy of `examples/beliefs/02_packing.ein` under a one-line
header: `witnessed` packs a fact into `knows`' second argument, and `confirm` joins it with Bob's `claims`.

| relation | `:why` today | renumbered | the declaration reads after | a fact, today (*output*) | after |
|---|---|---|---|---|---|
| `likes` (Person Food) | `{?1} likes {?2}` | `{?3} likes {?4}` | Person likes Food | stated, never derived | the same |
| `witness` (Person) | `{?1} is a witness` | `{?3} is a witness` | Person is a witness | stated, never derived | the same |
| `knows` (Person Claim) | `{?1} knows that {?2}` | `{?3} knows that {?4}` | Person knows that Claim | `` `knows(Ann, likes(Dan, Soup))` — Ann knows that (likes Dan Soup) `` | `knows(Ann, likes(Dan, Soup))` |
| `claims` (Person Claim) | `{?1} claims that {?2}` | `{?3} claims that {?4}` | Person claims that Claim | stated, never derived | the same |
| `confirmed` (Claim) | `that {?1} is confirmed` | `that {?3} is confirmed` | that Claim is confirmed | `` `confirmed(likes(Dan, Soup))` — that (likes Dan Soup) is confirmed `` | `confirmed(likes(Dan, Soup))` |

| today (*output*) | after (***by hand, not built***) |
|---|---|
| `` ### Step 1 — `witnessed` `` | same |
| `> Ann is a witness, so knows that Dan likes Soup` | same |
| ``Premises: `witness(Ann)`, `likes(Dan, Soup)` `` | same |
| ``Derives `knows(Ann, likes(Dan, Soup))` — Ann knows that (likes Dan Soup)`` | ``Derives `knows(Ann, likes(Dan, Soup))`.`` |
| `` ### Step 3 — `confirm` `` | same |
| `> Bob claims what Ann knows` | same |
| ``Premises: `claims(Bob, likes(Dan, Soup))`, `knows(Ann, likes(Dan, Soup))` `` | same |
| ``Derives `confirmed(likes(Dan, Soup))` — that (likes Dan Soup) is confirmed`` | ``Derives `confirmed(likes(Dan, Soup))`.`` |

Today a statement prints a fact under an arrow as its s-expression, `(likes Dan Soup)`, and never reads it by its own
relation's `:why` (*Dan likes Soup*); after, neither fact has a statement, so the question does not arise. Each
query's rows (`?c = (likes Dan Soup) · ?w = Ann`) are bindings, and stay.

### 2.5 The standard library: `std.algebra`, `std.slots` and the calculator's modules

**`std.algebra` and `std.slots` declare no relation**, and neither does `std.bijection`, `std.closure`, `std.elim`,
`std.macro`, `std.typing` or `std.math.positional`: their facts are activators, named after their rules, and the
relations they conclude are declared by the program (or, for `std.math.positional`, by the base that calls it). So
the ruling renumbers nothing of theirs: what they derive loses the statement its declarer gave it, as every fact
does, and their activators read as they did. From
[`05-slot-fill.ein`](s1f.23.5_examples/05-slot-fill.ein), a copy of `tests/stdlib/slots/03_fill.ein` under a
one-line header:

| today (*output*) | after (***by hand, not built***) |
|---|---|
| `` ### Step 1 — `std.algebra.symmetric-is-self-converse` `` | same |
| `> symmetric co-loc ⟹ (converse co-loc co-loc)` | same |
| ``Premises: `symmetric(co-loc)` `` | same |
| ``Derives `std.algebra.converse(co-loc, co-loc)`.`` | same: an activator has no declaration, today or after |
| `` ### Step 46 — `std.slots.slot-fill` `` | same |
| `> every Pet except Rat is excluded for slot S1 ⟹ Rat is at S1.` | same |
| ``Premises: `instance(S1, Seat)`, `type(Pet, Attribute)`, `instance(Rat, Pet)` `` | same |
| ``Derives `co-loc(S1, Rat)` — S1 and Rat are in the same seat`` | ``Derives `co-loc(S1, Rat)`.`` — the statement was the program's `co-loc`, `{?1} and {?2} are in the same seat`, renumbered `{?3} and {?4} are in the same seat`: *Attribute and Attribute are in the same seat* |

Its read-out's `owing` rows (`Ann is in no slot yet — some Seat is still owed.`) are `std.slots`' obligation rules'
`:why`, and stay.

**The modules that declare relations are the calculator's and the Boolean ones.** `std.math.decimal` and
`std.math.arith`, whole (each fact from `03-sum.ein`'s trace, *output*):

| relation | `:why` today | renumbered | the declaration reads after | a fact, today (*output*) | after |
|---|---|---|---|---|---|
| `n` (Number Digit) | `the number written {?1}, then the digit {?2}` | `the number written {?3}, then the digit {?4}` | the number written Number, then the digit Digit | a numeral's facts are arguments here, never derived | the same |
| `named` (Name) | `{?1} is a named constant` | `{?3} is a named constant` | Name is a named constant | `` `std.math.decimal.named(apples)` — apples is a named constant `` | `std.math.decimal.named(apples)` |
| `check` (Number Number) | `checking {?1}: {?2} is still to check` | `checking {?3}: {?4} is still to check` | checking Number: Number is still to check | `` `std.math.decimal.check(n(n(z, 1), 7), n(z, 1))` — checking (n (n z 1) 7): (n z 1) is still to check `` | `std.math.decimal.check(n(n(z, 1), 7), n(z, 1))` |
| `valid` (Number) | `{?1} is a decimal number` | `{?3} is a decimal number` | Number is a decimal number | `` `std.math.decimal.valid(n(z, 5))` — (n z 5) is a decimal number `` | `std.math.decimal.valid(n(z, 5))` |
| `not-decimal` (Number Thing) | `{?1} is not a decimal number: {?2} stands where a digit or a number goes` | `{?3} is not a decimal number: {?4} stands where a digit or a number goes` | Number is not a decimal number: Thing stands where a digit or a number goes | not derived in these programs | — |
| `decimal-rule` (Rule) | `{?1} is a rule of std.math.decimal` | `{?3} is a rule of std.math.decimal` | Rule is a rule of std.math.decimal | `` `std.math.decimal.decimal-rule(std.math.decimal.numeral-step)` — std.math.decimal.numeral-step is a rule of std.math.decimal `` | `std.math.decimal.decimal-rule(std.math.decimal.numeral-step)` |
| `z` (no label) | `zero, the number with no digit` | unchanged: no reference | zero, the number with no digit: the declaration `(relation z)` | `z` is an argument, never derived | the same |
| `division-by-zero` (Request) | `{?1} divides by zero` | `{?3} divides by zero` | Request divides by zero | not derived in these programs | — |
| `arith-module-rule` (Rule) | `{?1} is a rule of std.math.arith` | `{?3} is a rule of std.math.arith` | Rule is a rule of std.math.arith | `` `std.math.arith.arith-module-rule(std.math.arith.division-by-zero-is-false)` — std.math.arith.division-by-zero-is-false is a rule of std.math.arith `` | `std.math.arith.arith-module-rule(std.math.arith.division-by-zero-is-false)` |

Every one of the standard library's 360 declarations, today, renumbered and as it reads after, is
[`s1f.23.5_examples/stdlib-relations.md`](s1f.23.5_examples/stdlib-relations.md) (***by hand, not built***: the
census script's text substitution).

### 2.6 The counts

Re-taken by [`s1f.23.5_examples/census.py`](s1f.23.5_examples/census.py) (`decls`, `trace --bin
build/review-bin/ein-head-b5fb8d70`), over the tracked `.ein` files of the four roots.

**The declarations** (`census.py decls`). *Renumbered*: the `:why` writes a `{?k}`. *Unchanged*: it writes no
reference, so its text stays and only what it reads as changes. *No `:why`*: fixtures refused at the parse, whose
`:why` the loader never reads (Q-M1f.57); none is refused for a missing `:why`.

| root | files | files that declare | declarations | renumbered | unchanged | no `:why` |
|---|---:|---:|---:|---:|---:|---:|
| `stdlib/` | 24 | 16 | 360 | 359 | 1 | 0 |
| `examples/` | 597 | 507 | 1 515 | 1 401 | 12 | 102 |
| `tests/` | 273 | 95 | 257 | 257 | 0 | 0 |
| `corpus/fuzz_findings/` | 10 | 8 | 25 | 19 | 0 | 6 |
| all | 904 | 626 | 2 157 | 2 036 | 13 | 108 |

No declaration today writes a `{?k}` past its labels, or a reference that is not a number. Nine renumbered texts never
write `{?1}` (`examples/beyond_arith/collatz.ein`'s six, `fibonacci.ein`'s two, `laws.ein`'s one), which matters to
open point 8.

**The standard library, by module** (`census.py decls`). Three files are generated: `std.math.digits` and
`std.math.bits` by `utils/gen_arith_tables.py`, `std.math.num9` by `utils/gen_num9.py`.

| module | declarations | renumbered | | module | declarations | renumbered |
|---|---:|---:|---|---|---:|---:|
| `std.bool.circuit` | 3 | 3 | | `std.math.convert` | 11 | 11 |
| `std.bool.gates` | 22 | 22 | | `std.math.decimal` | 6 | 6 |
| `std.bool.inputs` | 4 | 4 | | `std.math.decimal-arith` | 70 | 70 |
| `std.bool.props` | 32 | 32 | | `std.math.digits` | 7 | 7 |
| `std.math.arith` | 3 | 2 | | `std.math.gcd` | 16 | 16 |
| `std.math.binary` | 6 | 6 | | `std.math.int` | 49 | 49 |
| `std.math.binary-arith` | 57 | 57 | | `std.math.num9` | 22 | 22 |
| `std.math.bits` | 4 | 4 | | `std.math.rat` | 48 | 48 |

**The trace** (`census.py trace`): `ein solve FILE --no-search --trace T.md -G -T 60` on every corpus entry (root's
derivation alone), counting the conclusions of every *Derives* line and the premises of every *Premises* line. The
244 entries with no trace are refused at the parse or at load (`examples/broken/`, `examples/ein-bugs/`, four
others), none for a missing `:why`.

| root | entries | traced | traces with a statement | steps | conclusions | with a statement, today | with one, after | premises | of them, declarations |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/` | 597 | 353 | 180 | 79 139 | 99 366 | 84 981 | 0 | 117 401 | 1 906 |
| `stdlib/` | 24 | 24 | 15 | 704 | 3 127 | 1 476 | 0 | 1 596 | 0 |
| `tests/` | 273 | 273 | 231 | 54 400 | 113 936 | 76 193 | 0 | 79 040 | 43 |
| all | 894 | 650 | 426 | 134 243 | 216 429 | 162 650 | 0 | 198 037 | 1 949 |

*After* is 0 because only a declaration has a template, and no conclusion in these traces is a fact of `relation`
(counted: 0). *Of them, declarations* counts the premises that are facts of `relation`, a declaration or its
one-argument companion: what open point 3's default would print a reading after (a companion none, open point 10).
The statements that fall back come mostly from the calculator: `std.math.decimal-arith.turn` alone is 27 263 of them,
then `mul-walk` 16 245, `sub-walk` 13 569, `std.math.decimal.valid` 11 477.

## 3. What the ruling leaves open

Each point with the options, the default this page took (and the *after* columns use), and why. A default stands
until the user rules otherwise.

1. **A rule's, an hrule's and a macro's `:why`.** A rule is no fact of the knowledge base, so it has no arrows to
   count. *Options:* (a) unchanged: filled by the firing's variables, `{?x}`, a macro's by a parameter; a `{?1}` in
   a rule's `:why` stays the variable `?1`; (b) read as a form too, `{?1}` the head `rule`, `{?2}` its name.
   **Default (a).** Why: the ruling speaks of *this fact*, and a rule's `:why` is the reason a firing gives, the one
   English line a trace step keeps after the change. `:source` and `:goal-text` are no `:why`, and stay.
2. **A fact with a `:why` of its own** (*"unless something else gives it one"*). Today a stated fact takes
   `:source` alone, keywords closed per form (Q-M1e2.28). *Options:* (a) no: a fact reads by its `:source`, its rule's
   reason, or the fallback; (b) a stated fact may carry `:why`, read by its own arrows, `(owns Ann Dog :why "{?2} has
   a {?3}")` reading *Ann has a Dog*, a grammar change; (c) a rule gives the facts it concludes a template, a larger
   one. **Default (a).** Why: the user chose *Declaration only*; (b) and (c) are language changes the ruling does not
   name, and no corpus program asks for one.
3. **Where a declaration's reading is printed.** No surface prints a statement of a stated fact today, and a
   declaration is never derived, so after the build the reading appears nowhere unless a surface is chosen.
   *Options:* (a) the trace's premise line prints a premise's statement where it has one, after its source:
   ``Premises: `relation(owns, Person, Pet)` — a Person owns a Pet, `owns(Bob, Cat)` ``; (b) the trace, or the
   read-out, lists the program's declarations with their readings once; (c) nowhere: the text documents the
   declaration for a reader of the program, as a macro's `:why` does (Q-M1f.59). **Default (a).** Why: the premise line
   is where the trace already names a declaration, when a rule reads it (1 949 times in the corpus's root traces),
   and it adds no surface; (b) is a new section of a read-out, which a ruling of its own should ask for.
4. **Whether `:why` stays required on a relation.** *Options:* (a) required, as today (Q-M1f.62); (b) optional on a
   relation, required on a rule, an hrule and a macro. **Default (a).** Why: every corpus program carries one, so (a)
   changes no load, and the requirement keeps each declaration saying what it is about; (b) changes which programs
   load, which the ruling does not touch.
5. **What `--no-require-why` becomes.** Follows 4. *Options:* (a) unchanged, it loads a program missing a `:why` on
   any of the four forms; (b) under 4 (b), its refusals name no relation. **Default (a).**
6. **The fallback's form.** *Options:* (a) as the trace prints a fact today, `owns(Ann, Dog)`, with nothing after it
   (``Derives `owns(Ann, Dog)`.``); (b) Ein's s-expression, `(owns Ann Dog)`, as the read-out and the dumps print it;
   (c) a sentence the engine makes, *owns holds of Ann and Dog*. **Default (a).** Why: it is the user's own example,
   and the trace's printing of a fact; (c) is invented prose, which [§ 6 of
   `14-natural-language.md`](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md#6-the-domain-and-the-engine-whose-words-are-whose)
   rules out (the engine owns no domain word). § 3 of that page calls the fallback `(R a b)` while the trace writes
   `R(a, b)`: the build's rewrite of § 3 says which surface writes which.
7. **A `{?n}` past the form's arrows.** *Options:* (a) printed as written, as every reference nothing binds is today
   (Q31), and `template_wellformedness.rs` holds the corpus to n ≤ 2 + the labels; (b) refused at load. **Default
   (a).** Why: it is today's rule for every template, and (b) is a new diagnostic. `{?1}` (always `relation`) and
   `{?2}` (the relation's name, the user's example) become legal in a declaration's `:why`.
8. **The renumbering, generated and checked.** *Options:* (a) one script run in the build's commit, `{?k}` →
   `{?k+2}` in every relation's `:why`, the generated files through their generators, and the wellformedness test
   then refusing a `{?1}` in a declaration's `:why`; (b) a generator kept under `utils/` with a `--check`. **Default
   (a).** Why: the rewrite happens once, and afterwards the texts are source like any other. `{?1}` prints the
   constant `relation`, so no text needs it, and refusing it catches every text the rewrite missed but the nine that
   start at `{?2}` (§ 2.6), which the rewrite's own diff shows.
9. **The words.** The renumbering keeps each text's words, written to read a fact; read with the labels, some make a
   sentence about the declaration (*Person owns a Pet*, *Request divides by zero*) and many do not (*T is a T*,
   *House is painted Color*, *the number written Number, then the digit Digit*, *Name names the number Number*). The
   user's own example adds articles and the name, `a {?3} {?2} a {?4}`. *Options:* (a) mechanical only, the words
   untouched; (b) every text rewritten as a sentence about its declaration, in the user's pattern. **Default (a).**
   Why: the user's choice names the renumbering, and 2 036 texts rewritten by hand is a ruling of its own. Either way
   [`14-natural-language.md`
   § 2](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md#2-every-declaration-carries-a-why)'s
   house style, *a relation's is the sentence one fact of it reads as*, is restated as the sentence its declaration
   reads as.
10. **The companion, and the same fact reached again.** The loader stores `(relation owns Person Pet)` with a
    one-argument companion, `(relation owns)` (Q-M1f.372); and a fact is its tuple (Q-M1f.380). **Default:** the
    `:why` is the written form's, so the companion reads by the fallback; an untyped declaration `(relation z :why
    "…")` is its own form and reads by its text; and the declaration's fact, however a run reaches it again, is the
    one object and reads by the same `:why`. Why: *the form it is written on*, and one object has one text.

## 4. What the build would touch

**The code.** The stage file's `git grep -l 'why' -- ein.rs/crates/*/src` matches no file as written (a glob
pathspec must match a whole path); `git grep -l 'why' -- 'ein.rs/crates/*/src/*'` lists 86, most for the English
word. The ones that read a relation's `:why`:

- `ein-render/src/sentence.rs`: `statement`, today a relation's `:why` with `{?k}` the fact's k-th argument, becomes
  the form's own `:why` by its arrows, for a declaration's fact alone; `FactSentence`'s and the module's text;
- `ein-render/src/trace/linearize.rs` and `trace/region.rs`: the *Derives* line, and the premise line under open
  point 3's default (both files are in S1f.23.1's uncommitted diff today);
- `ein-render/src/dump/serialise.rs`: the provenance comment's ` — <statement>`; `ein-render/src/lib.rs`' text;
- `ein-core/src/why.rs`: the module text on positional references and the unit test
  `a_positional_slot_is_a_reference_and_a_bare_brace_is_not`; `render_why` itself is unchanged;
- `ein-core/src/entities.rs`' `Relation::why` and `ein-ir/src/from_ir.rs`, which stores it and requires it: where the
  text hangs, unchanged by the defaults of points 4 and 5;
- `ein-core/src/shape.rs`: no change, but its `REL … why=` lines move with every renumbered text;
- the unit tests' inline programs under `src/`: 151 `:why` strings with a `{?k}` in 12 files (`from_ir.rs` 59,
  `compile.rs` 26, `saturator.rs` 22, …; `census.py touch`).

**The standard library**: 359 texts in 16 files, 33 of them in the three generated files (through
`utils/gen_arith_tables.py` and `utils/gen_num9.py`), then `utils/stdlib_manifest.py --write`; the rule pages under
`docs/ein/user/04-reference/04-stdlib/` through `utils/gen_stdlib_rule_pages.py`.

**The corpus**: 1 677 texts (`examples/` 1 401, `tests/` 257, `corpus/fuzz_findings/` 19). Generated among them, each
through its generator: the four zebra2 variants (`utils/gen_zebra2_variants.py`; `zebra2.ein` is a world anchor, so
`examples/README.md` § *What an edit to these two files fans out into* applies), three `examples/arith/` programs
(`utils/gen_arith_examples.py`), `send_more_money_written.ein` (`utils/gen_send_written.py`), the `synth/` programs
(`utils/region_synth.py`). Other `utils/` scripts that write a positional `{?k}` beside a `:why`: `arith_cost.py`,
`fuzz_engine.py`, `guard_split.py`, `nl_census.py`, `read_out_examples.py`.

**The tests and goldens** (`census.py touch` for the counts):

| what | why it moves |
|---|---|
| `ein-cli/tests/natural_language_text.rs` | `the_trace_prints_the_rules_why_and_the_premises_source`, and its table of surfaces |
| `ein-cli/tests/natural_language_page.rs` | `the_pages_trace_lines_are_a_runs`: § 3's block carries statements |
| `ein-cli/tests/region_modes.rs` | `a_trace_step_names_every_conclusion_of_its_firing` asserts ``Derives `q(a)` — a is q; …`` |
| `ein-ir/tests/template_wellformedness.rs` | a relation's references `{?1}`…`{?n}` become `{?2}`…`{?n+2}`, and open point 8's refusal of `{?1}` |
| `ein-render/tests/golden_trace.rs` | four of its six goldens carry statements: `trace_unconditional.md` 6, `trace_two-level.md` 10, `trace_one-hypothesis.md` 5, `trace_ambiguous.md` 5 |
| `ein-render/tests/golden/corpus_shapes.md5` | the `load` op's `REL … why=` lines, re-blessed |
| `ein-ir/tests/golden/zebra2.golden`, `zebra.golden` | `dump_goldens.rs`: the parsed program, 9 and 5 relation texts |
| `ein-infer/tests/golden/random_programs.txt`, `num2_theory.ein` | generated (800 and 22 texts) |
| `ein-ir/tests/golden/fuzz_seeds.txt` | 14 texts |
| the tests' inline programs | 606 `:why` strings with a `{?k}` in 65 `.rs` files |
| `ein-render/tests/corpus_ops/mod.rs` | the op that writes the provenance comment |

**The pages**: [`14-natural-language.md`](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)
(§ 1's table, § 2's house style, § 3's ladder and block, § 4's registers, whose *who can give its facts a text* is
now *nobody* for every fact but a declaration, § 5, § 7); the trace blocks with statements in `04-ontology.md`,
`05-rules.md`, `07-properties-and-operators.md`, `10-theory.md`, `11-macros.md` and the guide's
`05-jack-drinks-coffee.md`; every page with a declaration, 247 `:why` strings with a `{?k}` in 38 pages,
`02-relations.md`, `03-facts.md` and `02-syntax.md`'s `:why` section among them. The walkthrough quotes no relation's
`:why`, and the read-out reference reads an obligation's alone: neither moves.

**The decisions** that state a relation's `:why` reads a fact by position:
[Q31](../../../docs/decisions/language/Q31-a-why-template-refers-to-a-variable-as-var.md) (*a relation's `:why`
names its arguments by position*),
[Q-M1f.58](../../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md) (the statement),
[Q-M1f.60](../../../docs/decisions/read-out/Q-M1f.60-a-fact-of-a-relation-nobody-declares-has-no.md) and
[Q-M1f.61](../../../docs/decisions/read-out/Q-M1f.61-a-query-has-one-goal-text-and-each-conjunct.md) (a fact, a
conjunct, reads by its relation's `:why`),
[Q-M1f.12](../../../docs/decisions/read-out/Q-M1f.12-natural-language-text-reaches-a-fixed-set.md) (the trace
carries a derived fact's statement),
[Q-M1f.214](../../../docs/decisions/read-out/Q-M1f.214-a-firings-derives-line-names-every-conclusion.md) (each
conclusion followed by its statement), and
[Q27](../../../docs/decisions/language/Q27-a-relations-properties-are-facts-beside.md)'s example. Under points 4
and 5's defaults, Q-M1f.62, .57 and .63 stand. The stage file also names Q-M1f.112 and Q-M1f.159: the first is a
process question about P1f.7's order, the second multiplication's tag, and neither states `{?1}`.

**Elsewhere in the plans**: P1f.21's
[`open_facts.md` § 8.2](../p1f.21_verdicts_as_rules/open_facts.md#82-each-name-under-this-phases-direction) says the
declaration keeps *the `:why` a fact is read in*, which this ruling ends; `utils/nl_census.py`'s *statement* column.

## The programs

[`s1f.23.5_examples/`](s1f.23.5_examples/):

| file | what | from |
|---|---|---|
| [`01-owns.ein`](s1f.23.5_examples/01-owns.ein) | the user's example, a rule that reads the declaration | new |
| [`02-zebra2.ein`](s1f.23.5_examples/02-zebra2.ein) | the Zebra puzzle | `examples/zebra2.ein` |
| [`03-sum.ein`](s1f.23.5_examples/03-sum.ein) | a calculator program, 17 + 5 | new, from `examples/arith/add.ein`'s second request |
| [`04-packing.ein`](s1f.23.5_examples/04-packing.ein) | a fact under an arrow | `examples/beliefs/02_packing.ein` |
| [`05-slot-fill.ein`](s1f.23.5_examples/05-slot-fill.ein) | `std.algebra` and `std.slots` | `tests/stdlib/slots/03_fill.ein` |
| [`census.py`](s1f.23.5_examples/census.py) | the counts of § 2.6 and § 4 | new |
| [`stdlib-relations.md`](s1f.23.5_examples/stdlib-relations.md) | the standard library's 360 declarations, today and after | `census.py decls --list … --root stdlib` |

Each copy is the file as committed, under one header line. Every *today* above is re-taken by

```sh
build/review-bin/ein-head-b5fb8d70 solve plans/m1f_hypothesis_and_documentation/p1f.23_review_processing/s1f.23.5_examples/NN-name.ein --trace T.md -G
```
