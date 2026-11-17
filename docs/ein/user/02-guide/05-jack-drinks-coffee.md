# 5 — Jack drinks coffee

One small inference, stated four ways: **in words**, **in Ein**, **as a graph** and **in mathematics**, the four
representations every [concept page](../03-syntax-and-semantics/README.md) uses. What it shows is a *type* made by a
program. Ein has no types of its own: `is-a` is a relation the program names, `Human` and `Drink` are atoms like
`Jack` and `Coffee`, and an `is-a` fact does nothing until a rule reads it. Here two rules do, and they make an atom
inherit the facts of the atom it `is-a`: that is all *Jack is of type Human* means in this program
([ontology](../03-syntax-and-semantics/04-ontology.md)).

## In words

We want Ein to conclude

> *Jack can drink coffee.*

from these premises:

| # | claim |
|---|---|
| 1 | Humans can drink drinks. |
| 2 | Coffee is a drink. |
| 3 | Jack is a human. |
| 4 | Jack drinks coffee. *(the puzzle's explicit statement)* |

and, *without* using claim 4, also:

| # | claim | how it follows |
|---|---|---|
| 5a | Jack can drink drinks. | the left slot inherits: (1) and *Jack is a human* |
| 5b | Humans can drink coffee. | the right slot inherits: (1) and *coffee is a drink* |
| 6 | Jack can drink coffee. | chained: (5a) and *coffee is a drink*, or (5b) and *Jack is a human* |

(5a), (5b) and (6) follow from claims (1) to (3) and the two rules. Claim (4) is a separate assertion: *Jack drinks
coffee* is not *Jack can drink coffee*; one is observed, the other is a possibility the `is-a` facts and the rules
entail.

(5a) and (5b) come from **two rules**, one per argument slot of the two-slot relation `can-drink`. (6) is no rule of
its own: it is what the two rules reach when they chain. For a relation of N slots the pattern takes N slot rules,
and the closure over every slot at once is always their chained fixpoint, never a separate rule.

## In Ein

The whole program; `ein solve` answers it as written:

```lisp
;; Inherit a relation through is-a, one rule per argument slot.
(rule inherit-rel-lhs (?r)
  :match  (and (?r ?t ?other)
               (is-a ?inst ?t))
  :assert (?r ?inst ?other)
  :why    "{?inst} is a {?t}, so {?inst} inherits {?r}'s left slot from {?t}.")

(rule inherit-rel-rhs (?r)
  :match  (and (?r ?other ?t)
               (is-a ?inst ?t))
  :assert (?r ?other ?inst)
  :why    "{?inst} is a {?t}, so {?inst} inherits {?r}'s right slot from {?t}.")

(relation is-a      T     T     :why "{?1} is a {?2}")
(relation can-drink Human Drink :why "{?1} can drink {?2}")
(relation drinks    Human Drink :why "{?1} drinks {?2}")

(is-a Jack   Human)                      ; (3)
(is-a Coffee Drink)                      ; (2)

(can-drink Human Drink)                  ; (1) humans can drink drinks

(inherit-rel-lhs can-drink)              ; turn both rules on for can-drink
(inherit-rel-rhs can-drink)

(drinks Jack Coffee :source "puzzle text")   ; (4) the puzzle's statement

(solve :no-search)
(query :goal (can-drink Jack Coffee))
```

Each rule takes the relation it acts on as its parameter `?r`, and the facts `(inherit-rel-lhs can-drink)` and
`(inherit-rel-rhs can-drink)` turn it on for `can-drink`: a rule written once, for any relation, and *activated* per
relation ([rules](../03-syntax-and-semantics/05-rules.md)). `(solve :no-search)` says that nothing is guessed, so the answer
is what saturation derives from the program as written. Without it, Ein would also guess `is-a`, `can-drink` and
`drinks` facts the program does not state, and the answer would list every state those guesses reach; the goal holds
in all of them.

Saved as `jack.ein`, `ein solve jack.ein` ends:

```text
  query             (can-drink Jack Coffee)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      the goal holds: it has no variable
  as :expect        (model (can-drink Jack Coffee))
```

and `ein saturate jack.ein --dump` lists what saturation added, each fact with the rule that derived it:

```text
;; ── DERIVED (3 facts) ──
;;   can-drink (3)
  (can-drink Jack Drink :rule inherit-rel-lhs)
  (can-drink Human Coffee :rule inherit-rel-rhs)
  (can-drink Jack Coffee :rule inherit-rel-rhs)
```

Claim (6) has two derivations, one through each slot. The dump names the one that came first; the other is real too,
and the trace below shows both.

## As a graph

`ein render kb jack.ein` draws the saturated knowledge base as one graph: an atom an `is-a` arrow points at as a box
and an atom one starts from as an oval (the drawing's comments call them *types* and *instances*, its convention for a
relation named `is-a`), an `is-a` fact as a dashed arrow with an open head, any other fact of two arguments as an
arrow labelled by its relation, a derived one dashed and labelled with the rule that derived it. Leaving out the
`(relation …)` declarations it also draws, it is:

![Jack drinks coffee in the compact view: one stated can-drink arrow and three derived](diagrams/05-jack-compact.svg)

The three dashed `can-drink` arrows are the *inheritance closure* of the stated fact `(can-drink Human Drink)` across
the `is-a` arrows: every pair of $\lbrace \mathsf{Human}, \mathsf{Jack} \rbrace \times \lbrace \mathsf{Drink},
\mathsf{Coffee} \rbrace$ but the one it starts from. The compact view puts facts of *two* relations,
`can-drink` and `drinks`, on the same pair, from Jack to Coffee; only the names on the arrows tell them apart.

The **detailed** view, drawn by hand, is the graph itself
([objects](../03-syntax-and-semantics/01-objects.md#as-a-graph)): every fact is a node of its own, joined to its
relation by a dashed arrow and to each argument by a numbered one, and each relation is one node, which all its
facts point at. It makes visible what the compact view hides: `(can-drink Human Drink)` and `(can-drink Jack
Coffee)` are distinct facts, of different argument tuples, of one relation.

![The same facts in the detailed view, every fact a node of its own](diagrams/05-jack-levi.svg)

## In mathematics

The program states a finite set $K$ of facts:

$$
\begin{aligned}
K = \lbrace\ & \operatorname{is-a}(\mathsf{Jack}, \mathsf{Human}), \\
& \operatorname{is-a}(\mathsf{Coffee}, \mathsf{Drink}), \\
& \operatorname{can-drink}(\mathsf{Human}, \mathsf{Drink}), \\
& \operatorname{drinks}(\mathsf{Jack}, \mathsf{Coffee})\ \rbrace
\end{aligned}
$$

Each rule, once activated for `can-drink`, is a Horn clause over it, `inherit-rel-lhs` the first and
`inherit-rel-rhs` the second:

$$
\begin{aligned}
\forall t, o, i.\ \ & \operatorname{can-drink}(t, o) \land \operatorname{is-a}(i, t) \implies \operatorname{can-drink}(i, o) \\
\forall o, t, i.\ \ & \operatorname{can-drink}(o, t) \land \operatorname{is-a}(i, t) \implies \operatorname{can-drink}(o, i)
\end{aligned}
$$

The answer is the least set $K^{\ast} \supseteq K$ closed under both clauses, the fixpoint saturation reaches:

$$
\begin{aligned}
K^{\ast} = K \cup \lbrace\ & \operatorname{can-drink}(\mathsf{Jack}, \mathsf{Drink}), \\
& \operatorname{can-drink}(\mathsf{Human}, \mathsf{Coffee}), \\
& \operatorname{can-drink}(\mathsf{Jack}, \mathsf{Coffee})\ \rbrace
\end{aligned}
$$

and $\operatorname{can-drink}(\mathsf{Jack}, \mathsf{Coffee}) \in K^{\ast}$ is the goal. Since `can-drink` has two
slots and `is-a` is the only way into either, $K^{\ast}$ holds $\operatorname{can-drink}(x, y)$ for exactly the pairs

$$
(x, y) \in \lbrace \mathsf{Human}, \mathsf{Jack} \rbrace \times \lbrace \mathsf{Drink}, \mathsf{Coffee} \rbrace
$$

each slot's arguments closed downward along `is-a`, independently of the other. Nothing in the formulas sets
$\mathsf{Human}$ apart from $\mathsf{Jack}$: both are constants, and the two clauses are the only place `is-a` has a
meaning.

## What the example shows

Each claim is one fact, and the fact's origin is visible in the knowledge base:

| claim | the fact | where it came from |
|---|---|---|
| (1) Humans can drink drinks. | `(can-drink Human Drink)` | stated, background |
| (2) Coffee is a drink. | `(is-a Coffee Drink)` | stated, background |
| (3) Jack is a human. | `(is-a Jack Human)` | stated, background |
| (4) Jack drinks coffee. | `(drinks Jack Coffee :source "puzzle text")` | stated, a given: it has a `:source` |
| (5a) Jack can drink drinks. | `(can-drink Jack Drink)` | derived by `inherit-rel-lhs` |
| (5b) Humans can drink coffee. | `(can-drink Human Coffee)` | derived by `inherit-rel-rhs` |
| (6) Jack can drink coffee. | `(can-drink Jack Coffee)` | derived by either rule, chained |

`inherit-rel-lhs` and `inherit-rel-rhs` are **relation-polymorphic** rules: `?r` ranges over the relations a fact
activates them for ([rules](../03-syntax-and-semantics/05-rules.md)). The shape of each, a fact with a name in one slot
and an `is-a` arrow into that name, concluding the same fact with the arrow's source in that slot, is the
**inheritance pattern**: the same shape as `transitive`, applied along `is-a` instead of along the relation itself. `ein solve
jack.ein --trace out.md` writes the derivation step by step, each step's `:why` filled in:

```text
### Step 1 — `inherit-rel-lhs`
> Jack is a Human, so Jack inherits can-drink's left slot from Human.
Premises: `can-drink(Human, Drink)`, `is-a(Jack, Human)`
Derives `can-drink(Jack, Drink)` — Jack can drink Drink
…
### Step 3 — `inherit-rel-rhs`
> Coffee is a Drink, so Coffee inherits can-drink's right slot from Drink.
Premises: `can-drink(Jack, Drink)`, `is-a(Coffee, Drink)`
Derives `can-drink(Jack, Coffee)` — Jack can drink Coffee
### Step 4 — `inherit-rel-lhs`
> Jack is a Human, so Jack inherits can-drink's left slot from Human.
Premises: `can-drink(Human, Coffee)`, `is-a(Jack, Human)`
Derives `can-drink(Jack, Coffee)` — Jack can drink Coffee
```

## What it leaves out

- **Another encoding of the grouping.** `Human` and `Drink` here are bare `is-a` targets. `zebra2.ein` roots every
  such name in one name, `T`, so that each has an `is-a` fact of its own:

  ```lisp
  (relation is-a T T :why "{?1} is a {?2}")
  (is-a Human  T)
  (is-a Drink  T)
  (is-a Jack   Human)
  (is-a Coffee Drink)
  ```

  Both carry the same content for this example.
- **How `can-drink` and `drinks` relate.** The example keeps them apart. A puzzle that wants `(can-drink Jack Coffee)`
  to follow from `(drinks Jack Coffee)` writes it as one more rule.
- **Negation.** *Jack doesn't drink tea* is the stored negation `(not (drinks Jack Tea))`
  ([constraints](../03-syntax-and-semantics/06-constraints.md)), and an inheritance rule over negations is one more rule of
  the same shape.

## See also

- [Ontology](../03-syntax-and-semantics/04-ontology.md): the atoms a program groups its other atoms under, and the
  facts stated of them.
- [Rules](../03-syntax-and-semantics/05-rules.md): a rule's parameters, its activation, and what a pattern matches.
- [Rendering](../04-reference/12-rendering.md): every view `ein render` draws.
