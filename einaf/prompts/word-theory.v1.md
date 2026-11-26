# The word theory: what a word problem writes

Four modules beside the problem, imported by their file's name, give the words of a word problem their meaning
over the calculator: `words` (the words), `gaps` (a quantity the problem does not state), `readings` (a word with
two readings) and `listed_is_all` (*what is not listed is zero*). A problem writes facts of their relations, each
fact with the sentence it encodes as its `:source`, asks its question as a fact, and queries the value the theory
derives. Each module has one entry rule, which the import lists with every short name the file writes:
`(import words :symbols (words has gave asked amount value))`, `(import gaps :symbols (gaps))`,
`(import readings :symbols (readings))`, `(import listed_is_all :symbols (listed-is-all))`.

**A problem writes** (relations of `words`):

| the fact | the sentence |
|---|---|
| `(has WHO KIND NUMBER MOMENT)` | *Bob has 5 apples.* |
| `(gave WHO WHOM KIND NUMBER BEFORE AFTER)` | *He gave 2 to Alice.* The event names the moment before it and the moment after |
| `(holds CONTAINER KIND NUMBER)` | *A box holds 3.* |
| `(packed-in WHO KIND CONTAINER MOMENT)` | *12 boxes of pencils*: the pencils are in the boxes, each full |
| `(there-are KIND NUMBER)` | *31 classes* |
| `(asked QUANTITY)` | the question |
| `(assumed QUANTITY NUMBER)` | an assumption, which is not a sentence of the problem |
| `(reads-as need READING)` | *Every apple must be packed.* (a relation of `readings`) |
| `(adopted listed-is-all has)` | *what is not listed is zero*, adopted for `has`: a fact of `listed_is_all`, which a problem adopts by importing it |

A kind is written in the plural, `apples`, `boxes`, since a `:why` cannot inflect. A moment is a name, `t0`,
`t1`, or `now` where the problem has no event. A number is a numeral of the calculator, `(n z 5)`.

**A quantity** is a nested fact: `(amount WHO KIND MOMENT)`, `(together WHO WHO KIND MOMENT)`, `(apart WHO WHO
KIND MOMENT)` (by how many two differ), `(capacity CONTAINER KIND)`, `(needed CONTAINER KIND QUANTITY)`, `(over
QUANTITY)` (left unpacked), `(number-of KIND)`, `(each-of QUANTITY KIND)` and `(rest-of QUANTITY KIND)` (a
sharing's quotient and remainder).

**The theory derives:**

- **the plan**, backwards from the question: `(wanted Q)`, and `(needs Q Q')`, *to know Q one must know Q'*.
  Nothing is computed that no question needs;
- **the computation**, forwards from the stated numbers: a request to the calculator, `(add …)`, `(sub …)`,
  `(mul …)`, `(div …)` or `(cmp …)`, and its answer read back as `(value Q NUMBER)`;
- `(has-more WHO WHOM KIND MOMENT)` and `(same-amount …)` for a comparison, `(left-over Q FILLED OVER)` where a
  division for *need* leaves a remainder, and the flag `(gave-more-than-had WHO KIND HAD GAVE)`, from which a
  second rule derives `(false)`;
- **what the model leaves out**, stated once: the fact `(model-counts-only)`, whose `:source` says that things
  are counted by number alone and nobody gives away what they do not have.

`gaps` adds, by `absent`, `(not-stated Q)` for a quantity that is wanted, has no value and needs nothing;
`(unanswered QUESTION Q)` for a question that rests on one; and `(rests-on QUESTION Q NUMBER)` for a question
computed from an assumption. `readings` adds the readings of *need* as facts, `(reading-of need whole-only)` and
`(reading-of need pack-all)`, a hypothesis rule that proposes `(reads-as need R)` for every question of how many
containers are needed, and an obligation: a state with such a question and no reading owes one, so every state of
the answer names the reading its answer rests on. `listed_is_all` assumes, for every holder and kind the `has`
sentences name at a moment with no amount listed, `(assumed (amount WHO KIND MOMENT) z)`.
