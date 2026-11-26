# Ein: what a program is, in brief

Ein is a small logic language. A program states facts, declares the relations they are facts of, says by rules
what follows from them, and asks a question; the engine saturates the facts under the rules and answers. Its
meaning is Datalog's, with negation; its syntax is Lisp's, every form a list in parentheses.

## 1. Words

- A **name** is a run of ASCII letters, digits and the characters `_ . - $ % ^ * + = < > ! & @ ~ / ' [ ] { }`:
  `Ann`, `older`, `has-color`, `House-1`, `Old_Gold`. Hyphenated lower case for relations and rules, capitalised
  for things. No other character, no space and no comma in a name.
- **A number is a name.** `7` and `07` are two names; Ein has no numbers. A quantity the program computes with is a
  numeral of the library's calculator (§ 7).
- A **variable** is `?` and a name: `?who`, `?x`. A variable used once stands for anything.
- A **string**, `"…"`, is English text and is written only after `:why`, `:source` and `:goal-text`. Nowhere else:
  `(p "1")` is refused, `(p 1)` is the fact.
- `_` alone and `?_` are refused. A comment runs from `;` to the end of the line.

## 2. The forms

A program is a flat sequence of forms. A form whose head is one of these words is that form; **any other form is a
fact**.

| form | shape |
|---|---|
| a fact | `(NAME ARG* [:source "sentence"])`, or a stored negation `(not (NAME ARG*) [:source "sentence"])` |
| `relation` | `(relation NAME LABEL* :why "template")` |
| `rule` | `(rule NAME (PARAM*) :match PREMISE :assert CONCLUSION :why "template")` |
| `query` | `(query :goal PREMISE [:goal-text "template"])` |
| `solve` | `(solve [:no-search] [:hypothesis-relations (R*)] [:no-hypothesis (R*)])`, at most one |
| `import` | `(import MODULE [:symbols (NAME*)])` |

Each form takes only the keywords its row shows; any other keyword is refused. **A `:why` is required on every
`relation` and every `rule`.**

## 3. Facts and relations

A fact is a relation's name and its arguments: `(older Ann Bob)`. An argument is a name or a nested fact,
`(asked (amount Bob apples t1))`. A fact the text states carries the sentence that states it as its `:source`,
word for word.

A relation is declared once with a word for each of its places. **The words are labels, not types**: nothing checks
a fact against them. The `:why` is a template that reads the declaration: `{?2}` is the relation's name, `{?3}` its
first label, `{?4}` its second.

```lisp
(relation older Person Person :why "a {?3} is older than a {?4}")
(older Ann Bob :source "Ann is older than Bob.")
(older Bob Cy  :source "Bob is older than Cy.")
```

A relation is known by its name and its count of places: `(older Ann)` would be a fact of another relation, and
loads in silence. Declare every relation a fact uses, at the count it uses.

A **stored negation** says a fact does not hold: `(not (sits-in Cy First) :source "Cy does not sit in the first
chair.")`. What is neither stated nor denied is open.

## 4. Rules

```lisp
(relation older    Person Person :why "a {?3} is older than a {?4}")
(relation person   Person        :why "{?3} is a person")
(relation youngest Person        :why "a {?3} is the youngest")
(person Ann) (person Bob) (person Cy)
(older Ann Bob) (older Bob Cy)

(rule older-is-transitive ()
  :match  (and (older ?a ?b) (older ?b ?c))
  :assert (older ?a ?c)
  :why    "{?a} is older than {?b}, who is older than {?c}")

(rule youngest-is-older-than-nobody ()
  :match  (and (person ?p) (absent (older ?p ?q)))
  :assert (youngest ?p)
  :why    "{?p} is older than nobody, so {?p} is the youngest")

(query :goal (youngest ?who) :goal-text "{?who} is the youngest")
(solve :no-search)
```

- **`:match`** is a premise: a fact pattern with variables; `(and P …)` and `(or P …)` of premises; `(not P)`, a
  stored negation matches; `(absent P)`, no fact matches `P` (negation as failure); `(eq ?x ?y)` and `(neq ?x ?y)`,
  two objects are the same or not. `:match ()` is true.
- **`:assert`** is a conclusion: a fact pattern, `(and …)` of several, `(not P)`, or `(false)`, which says the
  state is contradictory. No `or` in a conclusion.
- **Every variable the conclusion writes is bound by the `:match`**, outside any `absent`; a rule that writes one
  nothing binds is refused.
- The `:why` is a sentence over the rule's variables, `{?a}`.
- **The parameter list** `(PARAM*)` is `()` for a rule that fires on every match. A rule with parameters,
  `(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "…")`, fires only where a fact of its
  name, its **activator**, binds them: `(symmetric next-to)`. That is how the library's rules are used (§ 7).

## 5. The question and the answer

`(query :goal (youngest ?who) :goal-text "{?who} is the youngest")`: a query's `:goal` is a premise over the program's relations; each row of its answer binds its variables, and the
`:goal-text` is a sentence filled per row: *Cy is the youngest*. **The answer query is the one query with a
`:goal-text`**, and its sentence says the answer in the problem's words. A goal may join several facts:
`(and (drinks ?w Water) (owns ?z Zebra))`.

## 6. What is unknown: the hypothesis mechanism

The engine's answer is every way the unknown facts can be chosen so that nothing contradicts and nothing owed is
missing: a **region of states**, each state the facts stated plus some **hypotheses**. A question has *one*
answer where the region has one state. What proposes the hypotheses is part of the program, and a program states
it:

- **`(solve :no-search)`**: the text states every fact there is; the answer is what the rules derive from them.
  The usual choice for a problem with no unknown (a word problem, an ordering).
- **The library's obligations**: a theory that says *each value has exactly one place* (`std.slots`, § 7) proposes
  every placement and owes one per value; the program writes no `solve` form for it.
- **A rule that concludes `(open F)`**: `F` is a hypothesis a state may suppose.
- A program that states none is answered by guessing a fact of every declared relation between every pair of names,
  which is almost never what is meant.

## 7. The standard library

A module is imported, and **the short names the program writes are listed after `:symbols`**; a name the file
writes and does not list is its own, and a library rule it meant to select stays silent.

- **`std.algebra`**, properties of relations of two places, each selected by a fact of its name:
  `(transitive R)`, `(symmetric R)`, `(includes R S)` (every fact of `R` is one of `S`), `(converse R S)` (`S` is
  `R` turned round), `(compose R S T)` (`R` then `S` is `T`), `(functional R)`, `(injective R)` (a check: a
  second value is a contradiction).

```lisp
(import std.algebra :symbols (transitive))
(relation older Person Person :why "a {?3} is older than a {?4}")
(older Ann Bob) (older Bob Cy)
(transitive older)
(query :goal (older Ann ?y) :goal-text "Ann is older than {?y}")
(solve :no-search)
```

- **`std.slots`**, things in places: `(slot-partition SAME-PLACE instance type Attribute Place)` makes
  `SAME-PLACE` an equivalence whose classes are the places, each holding exactly one value of every kind (a kind
  is `(type K Attribute)`, a value `(instance V K)`), and proposes the placements as hypotheses, owing one per
  value. `(slot-spatial SAME-PLACE NEXT-TO instance Place)` makes a relation between places hold between their
  values. `(import std.slots)` lists nothing.
- **`std.contradiction`**: with it imported, a fact beside its stored negation is a contradiction.
- **The calculator.** A natural number is a numeral: `z` is zero, `(n z 5)` is 5, `(n (n z 1) 2)` is 12,
  `(n (n (n z 1) 4) 4)` is 144. A calculation is a request fact, answered by a result fact that repeats its
  operands: `(add X Y)` → `(sum-of X Y S)`; `(sub X Y)` → `(diff-of X Y D R)`, `D` is X − Y where X is not the
  smaller, else `z`, and `R` is Y − X where X is the smaller, else `z`; `(mul X Y)` → `(product-of X Y P)`;
  `(div X Y)` → `(quot-of X Y Q R)`, Q the quotient and R the remainder; `(cmp X Y)` → `(cmp-of X Y O)`, `O` one of
  `lt`, `eq`, `gt`. A rule may state a request and read its result.

```lisp
(import std.math.decimal-arith :symbols (add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))
(relation total Number :why "the total is {?3}")
(add (n z 5) (n z 7))
(rule the-total ()
  :match  (sum-of (n z 5) (n z 7) ?s)
  :assert (total ?s)
  :why    "5 and 7 make {?s}")
(query :goal (total ?s) :goal-text "The total is {?s}.")
(solve :no-search)
```

## 8. What the kernel refuses

A program the kernel refuses has no answer. The refusals a first program meets most: a parenthesis not closed or one
too many; a string anywhere but after `:why`, `:source` and `:goal-text`; a keyword the form does not take (a fact
takes only `:source`); a `relation` or a `rule` with no `:why`; a character that is not ASCII outside a string or a
comment; `_`; a rule whose conclusion writes a variable its `:match` does not bind; a query whose goal names a
relation the program neither declares nor writes; an import of a module that does not exist. What loads in silence and answers
something else: a fact of a relation declared at another count, a library rule selected under a name the import
does not list, and a program with no hypothesis mechanism.
