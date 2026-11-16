# Obligations

An **obligation** says that something must exist: every person likes some food, every house has an owner, every
chair is taken. It is a rule that matches while the thing is missing and whose whole conclusion is the verdict
`(open ?R)`, *this state is unfinished: the facts of `?R` are incomplete*. A state the rule matches **owes**; a state
that owes nothing is **discharged**. An obligation derives nothing and is never stored: it is read once on each state
the engine reaches, after saturation, and it is half of what decides the answer. The other half is the restriction, a
rule that concludes `(false)` ([constraints](constraints.md)): restrictions say what a solution may not be,
obligations what it must contain, and the answer is every state between them
([solutions and models](solutions-and-models.md)).

## In Ein

Two people, two chairs, a rule against sharing, and the requirement that everyone sits:

```lisp
;; Two people, two chairs, and everyone sits.
(relation is-a T T          :why "{?1} is a {?2}")
(relation seat Person Chair :why "{?1} sits on {?2}")

(is-a Ann Person) (is-a Bob Person)
(is-a C1 Chair)   (is-a C2 Chair)

(rule one-chair-each ()
  :match  (and (seat ?p ?a) (seat ?p ?b) (neq ?a ?b))
  :assert (false)
  :why    "{?p} cannot sit on both {?a} and {?b}")
(rule one-person-a-chair ()
  :match  (and (seat ?p ?c) (seat ?q ?c) (neq ?p ?q))
  :assert (false)
  :why    "{?p} and {?q} cannot share {?c}")

(rule everyone-sits ()
  :match  (and (is-a ?p Person)
               (absent (and (is-a ?c Chair) (seat ?p ?c))))
  :assert (open seat)
  :why    "{?p} has no chair yet")

(seat Ann C2 :source "Ann sits on the second chair")

(query :goal (seat Bob ?chair))
```

`everyone-sits` is the obligation. Its `:match` has two parts: the **trigger**, `(is-a ?p Person)`, which says who
is owed, and one `(absent …)`, which says what is missing, the **witness**: some chair `?c` with `(seat ?p ?c)`.
`(open seat)` names the relation whose facts would pay the debt. At root Ann sits and Bob does not, so root owes one
instance, for Bob. No rule says where Bob sits, so the engine proposes the two facts that would pay the debt,
`(seat Bob C1)` and `(seat Bob C2)`, and keeps each state that is alive and owes nothing:

```text
  ∅                 not in R — root owes 1 instance
    owed            Bob has no chair yet
  …
  (region
    ;; state 1 of 1 — 1 candidate · minimal and maximal · from {(seat Bob C1)}
    (model (seat Ann C2) (seat Bob C1)))

  query             (seat Bob ?chair)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?chair = C1
```

`(seat Bob C2)` pays the debt too, but `one-person-a-chair` kills that state, so the one solution is Bob on the first
chair. The restriction bounds the answer from above, the obligation from below, and here they meet in one state.

A library states its obligations once, over any relation. `std.algebra`'s `total-owed` is the obligation *every
member of the domain has an image*, activated per relation by a fact:

```lisp
(rule total-owed (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?a ?A)
               (absent (and (?isa ?b ?B) (?R ?a ?b))))
  :assert (open ?R)
  :why    "{?R} owes {?a} a {?B}.")
…
(total-owed likes is-a)     ; every Person likes some Food
```

## As a graph

`ein render rule --name everyone-sits FILE` draws the obligation as its two sides: the match, with the `absent` as a
dashed cluster (*∄*), and the verdict, an octagon named `open` pointing at the relation it names.

```dot
digraph rule_everyone_sits_lhs_rhs {
  rankdir=TB;
  label="everyone-sits"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?p_L" [label="?p", shape=diamond];
    "Person_L" [label="Person", shape=rectangle];
    "?p_L" -> "Person_L" [label="is-a", color="#7f7f7f", fontcolor="#7f7f7f"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?c_L" [label="?c", shape=diamond];
      "Chair_L" [label="Chair", shape=rectangle];
      "?c_L" -> "Chair_L" [label="is-a", color="#7f7f7f", fontcolor="#7f7f7f"];
      "?p_L" -> "?c_L" [label="seat", color="#1f77b4", fontcolor="#1f77b4"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "seat_R" [label="seat", shape=rectangle];
    "h1_open_R" [shape=octagon, label="open", color="#2ca02c", fontcolor="#2ca02c"];
    "h1_open_R" -> "seat_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```

In `ein render region`'s drawing of the answer an owing node is filled amber, and an edge that pays a node's last debt
is drawn blue ([rendering](../reference/rendering.md)).

## In words

An obligation's `:why` is the debt, said of the instance that owes it: *Bob has no chair yet*. It is printed under
the state that owes, one line per undischarged instance, as the `owed` row under `∅` when root owes, and under each
node of the `owing` block when every alive node owes ([read-out](../reference/read-out.md)). Read aloud, the rule is
a sentence with a gap: *for every person, some chair is theirs; until one is, the state owes it*. The trigger is the
*for every*, the `absent` is the *some*, and `(open seat)` says which facts fill the gap.

## In mathematics

An obligation rule `o`, activated by `a`, has a trigger `T` and a witness `W`, and matches a state `S` at the
bindings `θ` of the trigger where no witness exists:

```text
owes(o, a, θ, S)  ⟺  S ⊨ T(θ)  ∧  ¬∃ ȳ. S ⊨ W(θ, ȳ)

owes(S)        = | { (o, a, θ) : owes(o, a, θ, S) } |
discharged(S)  ⟺  owes(S) = 0
```

`(open ?R)` names the relation of the one premise of `W` that binds `ȳ`, the slot a payment fills. A state of the
answer is alive and discharged:

```text
R = { C ⊆ H : (false) ∉ sat(root ∪ C)  ∧  owes(sat(root ∪ C)) = 0 }
```

Discharge is a positive test, `∃ ȳ. W`, so on a program where commitments cannot grow `T`, a state that is
discharged stays discharged as facts are added; an obligation bounds R from below as a restriction bounds it from
above.

## The form: `(open ?R)`

An obligation is a `rule` whose **whole** `:assert` is `(open ?R)` or `(open)`:

| form | says |
|---|---|
| `(open ?R)` | the facts of `?R` are incomplete. `?R` is a parameter of the rule, bound by its activator, or a relation's name |
| `(open)` | the state owes something, and names no relation: it counts, and the `:why` is the whole report |

`open` is written there and nowhere else: not in a `:match`, not beside another conclusion, not inside a fact, a
value or a macro's body ([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md),
[Q-M1f.86](../../../decisions/language/Q-M1f.86-a-macros-body-never-writes-open.md)). The requirement is stated once,
in the guard; `(open ?R)` restates nothing of it
([Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)):

- **the witness** is the premise of `?R` inside the rule's one `(absent …)` that binds a variable the rest of the
  guard does not, `?c` in `(seat ?p ?c)`;
- **the domain** is everything else inside that `absent`, `(is-a ?c Chair)`: the values a payment may take;
- **the direction** falls out of which slot is free. `total-owed` leaves the image `?b` free, so it owes each member
  of the domain an image; `surjective-owed` leaves `?a` free, so it owes each member of the range a preimage. Neither
  carries a position number.

Where the guard does not say this one way, the program is refused at load rather than guessed: no `absent` holds a
premise of `?R`, two do, the premise binds no variable of its own (a ground check, which no witness could pay), or two
premises of `?R` each bind one (a *compound witness*, two facts that together pay, which has no single slot to fill).
Each refusal is a line of [diagnostics](../reference/diagnostics.md).

`(open)` with no relation is legal and pays for being unnamed: the engine cannot tell which facts would pay it, so it
proposes nothing for it, and the blind enumerator guesses over every relation instead. Name the relation.

## When an obligation is read

An obligation is **not in the saturation agenda**. Saturation runs to its fixpoint; then one pass reads every
obligation against that fixpoint, and the instances it matches are the state's debts. Three things follow:

- **It derives nothing.** No rule can read a debt, and an obligation cannot enable another rule. A rule that
  concluded `open` and something else would belong to both passes, so that is refused at the parse.
- **Firing is being undischarged.** The rule matches while the witness is missing and stops matching once it is
  there, so the pass asks no second question: what discharges a debt is exactly what the guard looks for.
- **A debt is never a fact.** `(false)` is stored, because a dead state stays dead whatever is added; a debt exists
  to be paid by an addition, and a stored one would survive its own payment. So a debt is read on each state, root
  and every node the engine builds, and belongs to that state alone.

A state that holds `(false)` is dead whatever it owes. The order of the debts in a report is the order the
obligation rules are written in.

## What an obligation quantifies over

An obligation needs no closed domain, no closed relation and no type system
([Q-M1d.3](../../../decisions/hypotheses/Q-M1d.3-an-obligation-needs-no-closed-domain.md)). Four things hold of every
obligation, and they are what makes it safe to state in an open world.

### The domain is the guard

The domain of a debt is written nowhere but in the rule's own `absent`: the values the premise beside the witness
matches. A library takes the membership relation as a parameter, `?isa`, never as a literal `is-a`, so a program may
pass `is-a`, `is-a*` or a relation of its own, and stating an obligation asks nothing about whether that extent is
closed or complete.

### Paying a debt needs no closure

A debt is paid when a witness is **present**, which is a positive check: once paid, it stays paid as the state grows.
So an obligation assumes nothing about the facts not yet known. It may report a debt that a scan over stored
negations would show can never be paid; it never reports a state discharged that is not.

### Refuting stays with the restrictions

*The requirement can never be met* is a different statement, and an obligation does not make it. It is a
restriction's: a rule that scans every candidate for a **stored negation** and concludes `(false)` when all are
excluded. The stdlib pairs each obligation with such a scan: `total` and `surjective` beside `total-owed` and
`surjective-owed` in `std.algebra`, `slot-no-room` and `slot-no-fill` beside `slot-owed-room` and `slot-owed-fill`
in `std.slots`; `std.elim`'s `no-room-left` is another. A scan for stored negations is safe in an open world, since it fires
only on a genuine exclusion; the one scan in the stdlib that reads absence instead, `connex`, is sound only where its
relation is fully determined by saturation, and its module says so.

| candidates left for an owed slot | which rule speaks | the state is |
|---|---|---|
| none: every one excluded by a stored negation | the restriction (`total`) | dead |
| none present, some still open | the obligation (`total-owed`) | owing |
| one present | neither | discharged for that instance |

### What would pay a debt

The facts that would pay a debt are read off the same guard with the witness step left out: every binding of the
domain premise for which the witness is neither present nor negated. Nothing is stored, so a value that arrives later
is seen. Choosing among those facts is exhaustive only while the set cannot grow under the choice, so where an
obligation's domain could grow, because the guard scans a relation that is itself guessed, or a relation a commitment
can change, the engine does not trust the set and proposes as it would with no obligation
([Q-M1e2.10](../../../decisions/hypotheses/Q-M1e2.10-the-obligations-rung-declines-where-its-domain.md)). The `universe`
row of the read-out says which way H was built.

## What an obligation does to the answer

An obligation plays two parts ([hypotheses](hypotheses.md)):

- **It judges.** A state that owes is not in R, whatever proposed it. This holds beside an `hrule` too: the hrule
  generates, the obligation judges, and that is how [`zebra2.ein`](../../../../examples/zebra2.ein) states its
  puzzle.
- **It generates, where nothing else does.** With no `hrule`, and an obligation **activated** at root (a rule with no
  parameters always is; a parameterised one is once an activator fact matches it), the candidates are the facts that
  would pay the debts. A root that owes nothing then proposes nothing and is the one state. An obligation rule that a
  program loads and nothing activates, as an import of a whole library brings its obligation rules, does neither.

An obligation that owes a relation the program also keeps out of the guessing, by `(__closed__ R)` or, with no hrule,
by a `(solve …)` list, is refused at load: a state that owed it could never be paid
([Q-M1f.244](../../../decisions/hypotheses/Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md)).
To judge root alone, with nothing proposed, a program states `(solve :no-search)` ([query](query.md)).

## Reading a state that owes

Where a state owes, every surface says what:

| surface | what it says |
|---|---|
| the read-out, `∅` row | *not in R — root owes n instances*, and an `owed` row under it per instance, the `:why` |
| the read-out, `empty` row | *every alive node owes — no commitment pays every debt*: R is empty of the second kind, and an `owing` block lists the alive owing nodes with no alive node one fact up, each with its debts |
| the printed region | `(region (owing (model …) …))` for that kind of empty R, which reads back as a claim ([query](query.md)) |
| `--single` | the word `Open`: R is empty and every alive node owes |
| `--json-summary` | `root.owes`, the count, and `root.owed`, the texts |
| `--events` | one `owe` line per undischarged instance: the rule, its activator, the relation, the bindings and the `:why` |

With the seating program above stating `(solve :no-search)`, so that root alone is judged:

```text
  empty             every alive node owes — no commitment pays every debt
  …
  owing             ∅ — owes 1
                      Bob has no chair yet

  (region (owing (model)))
```

`Open` and `Contradiction` are two different empty answers: *not yet a model* against *no model*. `Contradiction`
is said only where no node is alive
([Q-M1d.6](../../../decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)), and `Open` where nodes
are alive and every one of them owes.

## The stdlib's four

| rule | module | owes | activated by |
|---|---|---|---|
| `total-owed` | `std.algebra` | each member of `?R`'s domain an image | `(total-owed R isa)`, or `std.bijection`'s `bijective-setup` from `(bijective R)` |
| `surjective-owed` | `std.algebra` | each member of `?R`'s range a preimage | `(surjective-owed R isa)`, or `bijective-setup` |
| `slot-owed-room` | `std.slots` | each item a slot | `std.slots`' `slot-partition-setup` |
| `slot-owed-fill` | `std.slots` | each slot an item of each kind | `slot-partition-setup` |

Each asserts `(open ?R)` and nothing else, and each is paired with the restriction that refutes the same requirement
once it cannot be met. Their reference is [the standard library](../reference/stdlib.md).

## Saying how many

Ein has no numbers ([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)), so a
bound is not `1..1` or `at least 2`: it is a relation to another set, said with the four properties of a relation.

| the claim | said with relations |
|---|---|
| at least one | an obligation (`total-owed`) |
| at most one | `functional`: two images are `(false)` |
| as many as `S` or more | a pairing from `S`: `total` on it, `functional` and `injective` |
| exactly as many as `S` | a bijection: `(bijective R)` |

*At least two* is an injection from a set of two the program declares: the two is an extent of objects, not a number.
The cost is ontology, one relation per bound, and the gain is that every bound is something the rules can read.

## What is refused

Each of these is refused before anything runs, on every command, with the line under
[diagnostics](../reference/diagnostics.md):

- `open` anywhere but a rule's whole `:assert`, at the parse;
- `(open …)` with more than one argument, or beside another conclusion, at the parse;
- `(open ?R)` whose `?R` is no parameter of the rule, or whose witness the guard does not give one way, at load;
- an obligation owing a relation that is closed, or, with no hrule, one a `(solve …)` list leaves out or names in
  `:no-hypothesis`, at load.

## See also

- [Constraints](constraints.md): `(false)`, the other bound.
- [Hypotheses](hypotheses.md): the ladder an obligation's second part belongs to.
- [Solutions and models](solutions-and-models.md): R, its two lines, and the two kinds of empty R.
- [The standard library](../reference/stdlib.md): `std.algebra`, `std.bijection`, `std.slots`, `std.elim`.
- [The read-out](../reference/read-out.md): every row that reports a debt.
- [The definition of R](../../reasoning/kb-evolution/solution-semantics.md), stated for a second implementation.
