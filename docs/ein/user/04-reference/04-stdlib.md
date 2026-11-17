# The standard library (`std.*`)

The standard library is a set of modules written in Ein: rules, macros and tables of facts that a program brings in
with one `(import …)`. A rule family that is not about one problem is a module here, and a problem's own rules stay
in its own file ([Q16](../../../decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md)). This page is
the reference for every module: its name and entry, what a program lists to use it, every relation and rule a
program writes or reads, what it flags, and a small program with what `ein solve` answers. The concepts the
relation-algebra modules build on are [Properties and operators](../03-syntax-and-semantics/07-properties-and-operators.md);
the import form itself is [the syntax reference](02-syntax.md) and [Macros and imports](../03-syntax-and-semantics/11-macros.md).

Ein has no numbers ([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): `7`
is a name, and `1` and `01` are two names. Arithmetic is a theory of the library, `std.math.*`, written over the
digit names and computed by saturation; Boolean functions are `std.bool.*`
([Q-M1f.338](../../../decisions/stdlib/Q-M1f.338-the-calculator-is-std-math-and-boolean-functions.md)). An order
is a relation a program declares, never a form of the language
([Q25](../../../decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md)).

## Importing a module

A module's name is its path in the library, a dot a directory: `std.algebra` is `algebra.ein`,
`std.math.decimal-arith` is `math/decimal-arith.ein`. The library ships inside the `ein` binary; a checkout's
`stdlib/` directory, or the directory `$EIN_STDLIB` names, takes its place where present
([`01-install.md`](../01-install.md) says how to check which one a binary reads).

- **An import brings its module whole**, with the modules it imports, one copy a program
  ([Q-M1f.247](../../../decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)). Every name a module
  declares has one full name, `std.math.decimal-arith.add`
  ([Q-M1f.231](../../../decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)), and that is
  how a state prints a name the program does not write.
- **`:symbols (…)` lists the short names the file writes.** A file lists every name it writes from the module that
  declares it, and nothing is re-exported: a program that writes a numeral lists `n` from `std.math.decimal` and `z`
  from `std.math.arith`, though it imports neither directly. `:as a` writes `a.add`; a bare import writes full
  names. The answer does not depend on which of the three a file uses.
- **What to list.** A generic rule is activated by a fact headed by the rule's name, `(symmetric next-to)`, and the
  rule's name is the module's: list it. A relation a module reads and does not declare — `bijective`, `join`,
  `bijection-hierarchy`, `slot-partition`, `slot-spatial`, `type-hierarchy`, `reflexive`, `std.elim`'s positional
  markers — is no module's, and a program writes it as it is. A module's entry rule may be listed and brings nothing
  a whole import does not ([Q-M1f.240](../../../decisions/imports/Q-M1f.240-a-module-keeps-its-entry-rule-which-a-program.md)).
- **A short name the file writes and lists from no import is its own**, whatever a module calls the same
  ([Q-M1f.236](../../../decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)). So a
  `(symmetric R)` written without listing `symmetric` activates nothing.
- **One short name bound to two full names in one file is refused**
  ([Q-M1f.234](../../../decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)): `std.elim`
  and `std.bijection` both declare `domain-elimination`, and a file lists it from one of them.
- **No module reserves its names** ([Q-M1f.319](../../../decisions/imports/Q-M1f.319-no-module-reserves-its-names.md)).
  A program may state a fact of a module's relation: a `(sum-of X Y S)` written by hand is an answer nobody
  computed, and the theory computes with it.
- **No import brings a module's `(query …)`, `(solve …)` or `(config …)`**: those are a program's.

| module | what it is | entry | an import brings |
|---|---|---|---|
| [`std.macro`](#stdmacro--pattern-macros) | `forall` and `unknown` | — | — |
| [`std.algebra`](#stdalgebra--the-relation-algebra) | the relation algebra: operators, property checks and closures, the totality obligations | — | `std.macro` |
| [`std.bijection`](#stdbijection--bijections-read-off-the-signature) | negative completion and elimination for a relation declared `(bijective R)` | — | `std.algebra`, `std.macro` |
| [`std.elim`](#stdelim--elimination-with-positional-markers) | the same elimination, with the types named in the activator | — | `std.macro` |
| [`std.slots`](#stdslots--one-co-location-relation-whose-classes-are-slots) | one co-location relation whose classes are slots | — | `std.macro` |
| [`std.typing`](#stdtyping--a-type-hierarchy-named-once) | a type hierarchy named once; the reflexive closure | — | `std.algebra` |
| [`std.closure`](#stdclosure--closing-a-relation-that-is-a-total-function) | `functional ∧ total` ⟹ `(__closed__ R)` | — | `std.algebra` |
| [`std.math.digits`](#stdmathdigits--the-digit-tables) | the decimal digit tables, facts and no rule | — | — |
| [`std.math.arith`](#stdmatharith--zero-and-the-division-by-zero) | `z`, zero; the division by zero's flag and its rule | `arith` | — |
| [`std.math.decimal`](#stdmathdecimal--the-decimal-numeral) | the decimal numeral `(n PREFIX DIGIT)` and its check | `decimal` | `arith`, `digits` |
| [`std.math.decimal-arith`](#stdmathdecimal-arith--school-arithmetic-on-natural-numbers) | natural numbers of any length: `add`, `sub`, `cmp`, `mul`, `div`, `let` | `decimal-arith` | the four above, `positional` |
| [`std.math.positional`](#stdmathpositional--the-rules-every-base-shares) | the rules every positional base shares, and the macro that connects a base | the macro | `arith` |
| [`std.math.bits`](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith) | binary's tables, facts and no rule | — | — |
| [`std.math.binary`](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith) | the binary numeral `(b2 PREFIX BIT)` and its check | `binary` | `arith`, `bits` |
| [`std.math.binary-arith`](#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith) | binary arithmetic, every name `bin-` | `binary-arith` | `bits`, `arith`, `binary`, `positional` |
| [`std.math.convert`](#stdmathconvert--decimal-to-binary-and-back) | decimal to binary and back | `convert` | both bases |
| [`std.math.int`](#stdmathint--integers) | integers `(int P N)` | `int-arith` | `decimal-arith` and below |
| [`std.math.rat`](#stdmathrat--rational-numbers) | rationals `(rat Z D)`, reduced on request | `rat-arith` | `int`, `gcd`, `decimal-arith` and below |
| [`std.math.gcd`](#stdmathgcd--greatest-common-divisor-and-least-common-multiple) | `gcd` and `lcm` of natural numbers | `gcd` | `decimal-arith` and below |
| [`std.math.num9`](#stdmathnum9--nine-digits-in-one-fact) | nine digits in one fact, a fixed width | `arith9` | `digits`, `decimal-arith` and below |
| [`std.bool.gates`](#stdboolgates--the-sixteen-functions) | the sixteen Boolean functions of two inputs | `gates` | — |
| [`std.bool.props`](#stdboolprops--what-is-known-of-the-sixteen) | thirty relations known of the sixteen, derived by rules | `props` | `gates` |
| [`std.bool.circuit`](#stdboolcircuit--a-circuit-simulated-by-saturation) | a circuit as facts, simulated by saturation | `circuit` | `gates` |
| [`std.bool.inputs`](#stdboolinputs--inputs-as-hypotheses) | a circuit's inputs as hypotheses, and required values | `inputs` | `circuit`, `gates`, `std.algebra` |

An **entry** is a module's closing rule: a rule with an empty `:match` that closes every relation of the module
and names every rule of it, so that nothing of the module is left for the search to guess. The relation-algebra
modules have none, since what they close is the program's to say.

## Three distinctions that govern soundness

Read these before importing a relation-algebra module: they decide whether a rule is safe for a problem.

- **Intrinsic or extensive.** An **intrinsic** operator reads only existing edges: `compose`, `meet`, `difference`,
  `converse`, `join`, `difunctional`. An **extensive** one (⊙ below) ranges over a `Dom × Ran` universe to reach
  pairs that are absent, so it takes the program's membership relation and the argument types as parameters,
  `(?isa ?Dom ?Ran)`: `complement`, `top`, `identity`, `connex`.
- **Check, closure or obligation.** A **check** concludes `(false)` on a violation (`functional`, `injective`,
  `total`, `surjective`, `empty`, `irreflexive`, `antisymmetric`, `asymmetric`, `connex`): a state that breaks it is
  dead. A **closure** derives the edge that makes the property hold (`symmetric`, `transitive`, `includes`,
  `difunctional`, `reflexive`). An **obligation** concludes `(open R)` while a requirement is unmet (`total-owed`,
  `surjective-owed`, `slot-owed-room`, `slot-owed-fill`): it derives nothing, and a state that owes is not an answer
  ([Obligations](../03-syntax-and-semantics/08-obligations.md)).
- **The closed-world caveat.** A rule that reads an absence — the extensive operators, `difference`,
  `compose-negative-*`, `infer-closure`, the `absent` guards of `std.slots`' spatial rules — is sound only where its
  operand is decided by saturation, never where the operand is solved by hypotheses, as the attribute relations of
  the Zebra puzzle are. Each is opted into per use.

## `std.macro` — pattern macros

Two macros, which expand when the program is loaded; neither is a rule. A program that writes one in its own rule
lists it.

```lisp
(import std.macro :symbols (forall unknown))
```

| form | expands to | means |
|---|---|---|
| `(forall ?b G B)` | `(absent (and G (absent B)))` | every `?b` the guard `G` holds of satisfies `B`; `?b` must occur in `G` |
| `(unknown P)` | `(and (absent P) (absent (not P)))` | `P` is neither stated nor negated: the third state of a fact |

`unknown` is how a rule of a program says that a fact was decided neither way: naming a relation closes its positive
facts, and a stored negation is not closed by it, so a rule that probes with `unknown` is how a program states that a
negation was *not* derived.

## `std.algebra` — the relation algebra

Tarski's relation algebra ⟨∪, ∩, ¬, 0, 1, ;, ⁻¹, 1′⟩ over binary relations, as generic rules, with the property
checks, closures and obligations a puzzle declares of a relation. Each rule is activated per relation by a fact
headed by its name; list the rules whose activators the file writes:

```lisp
(import std.algebra :symbols (converse compose meet symmetric transitive))
(converse right-of left-of)             ; left-of = right-of⁻¹
(compose  right-of right-of two-right)  ; two-right ⊇ right-of ; right-of
```

The relation algebra expresses first-order constraints of at most three variables; a constraint over more (a
co-location of four, an adjacency of five) is a puzzle's own named rule. What each property means, in the four
representations, is [Properties and operators](../03-syntax-and-semantics/07-properties-and-operators.md).

**The relative layer**

| activator | effect | kind |
|---|---|---|
| `(converse R1 R2)` | `(R1 a b)` ⟹ `(R2 b a)`: R2 ⊇ R1⁻¹ | intrinsic |
| `(imply1 R1 R2)` | `(R1 a)` ⟹ `(R2 a)`, a property implying a property: `(imply1 functional __closed__)` | intrinsic |
| `(imply2-fwd R1 R2)` | `(R1 a b)` ⟹ `(R2 a b)` | intrinsic |
| `(imply2-reverse R1 R2)` | `(R1 a b)` ⟹ `(R2 b a)`, the same as `converse` | intrinsic |
| `(compose R1 R2 R3)` | `(R1 a b)` ∧ `(R2 b c)` ⟹ `(R3 a c)`: R3 ⊇ R1 ; R2; `(compose R R R)` is R's transitive closure | intrinsic |
| `(identity R isa Dom)` | every `Dom` member `a` ⟹ `(R a a)` | extensive ⊙ |

**The Boolean layer**

| activator | effect | kind |
|---|---|---|
| `(meet R1 R2 R3)` | R3 ⊇ R1 ∩ R2 | intrinsic |
| `(difference R1 R2 R3)` | R3 ⊇ R1 ∖ R2, reading `(absent (R2 a b))` | intrinsic, closed-world |
| `(join R1 R2 R3)` | R3 ⊇ R1 ∪ R2. `join` is no rule's name, so it is written unlisted; `derive-join` fans it out to the copiers `join-l` and `join-r` | intrinsic |
| `(empty R)` | a check: any `(R a b)` ⟹ `(false)` | check |
| `(top R isa Dom Ran)` | R ⊇ the whole `Dom × Ran` | extensive ⊙ |
| `(complement R1 R2 isa Dom Ran)` | R2 ⊇ the pairs of `Dom × Ran` absent from R1 | extensive ⊙, closed-world |

**Cardinality checks** (each concludes `(false)` on a violation)

| activator | effect |
|---|---|
| `(functional R)` | right-unique: `(R a b)`, `(R a c)`, `b ≠ c` ⟹ `(false)` |
| `(injective R)` | left-unique: `(R a c)`, `(R b c)`, `a ≠ b` ⟹ `(false)` |
| `(total R isa)` | left-total: an `a` of R's first type with **every** `b` of its second type stated `(not (R a b))` ⟹ `(false)`. Types are read from `(relation R A B)`; a state that merely knows nothing yet is not refuted |
| `(surjective R isa)` | right-total, the dual |
| `(bijective R)` | no rule's name, written unlisted: `bijective-properties` derives `(functional R)`, `(injective R)`, `(total R)` and `(surjective R)`, the one-argument markers that `std.bijection` and `std.closure` read |

**Totality obligations** (each concludes `(open R)`; [Obligations](../03-syntax-and-semantics/08-obligations.md))

| activator | effect |
|---|---|
| `(total-owed R isa)` | an `a` of R's first type with no `(R a b)` stated for any `b` of its second type: R owes `a` a partner |
| `(surjective-owed R isa)` | a `b` of R's second type with no `(R a b)` for any `a`: R owes `b` a preimage |

`total` and `total-owed` are one property at its two ends: the check is about a candidate set emptied by stated
negatives, so the requirement cannot be met and the state is dead; the obligation is about a set with nothing
stated in it yet, so the requirement is unmet and the state is unfinished.

**Other property checks** (each concludes `(false)` on a violation)

| activator | effect |
|---|---|
| `(irreflexive R)` | `(R a a)` ⟹ `(false)` |
| `(antisymmetric R)` | `(R a b)`, `(R b a)`, `a ≠ b` ⟹ `(false)` |
| `(asymmetric R)` | `(R a b)`, `(R b a)` ⟹ `(false)`, a self-loop included: antisymmetric and irreflexive |
| `(connex R isa Dom)` | two distinct `Dom` members related neither way ⟹ `(false)` (extensive ⊙, closed-world) |

**Property closures** (each derives the edge that makes the property hold)

| activator | effect |
|---|---|
| `(symmetric R)` | `(R a b)` ⟹ `(R b a)`. With `symmetric-negative-setup` listed too, the negation mirrors: `(not (R a b))` ⟹ `(not (R b a))` |
| `(transitive R)` | `(R a b)`, `(R b c)`, `a ≠ c` ⟹ `(R a c)`; the guard keeps an irreflexive closure irreflexive |
| `(includes R S)` | `(R a b)` ⟹ `(S a b)`: S includes R, `(includes is-a is-a*)` |
| `(difunctional R)` | `(R a b)`, `(R c b)`, `(R c d)` ⟹ `(R a d)` |

**Typed converses and the equational lemmas**

| activator | effect |
|---|---|
| `(converse-illtyped-dom R1 R2 isR*)`, `(converse-illtyped-ran R1 R2 isR*)` | `(false)` where a `(converse R1 R2)` pair's signatures, read off the two `(relation …)` declarations, do not reverse: the two types of a side are neither equal nor one a subtype of the other under `isR*`, either way. `std.typing` derives these activators from one knob |
| *none: they read operator facts* | `symmetric-is-self-converse` and `self-converse-is-symmetric` (`(symmetric R)` ⟺ `(converse R R)`), `converse-pair-symmetric` (`(converse R1 R2)` ⟹ `(converse R2 R1)`), `compose-contravariant` ((R ; S)⁻¹ = S⁻¹ ; R⁻¹) and `join-converse` ((R ∪ S)⁻¹ = R⁻¹ ∪ S⁻¹). Every derivation back is a fact already present, so they converge |
| `(compose-negative-s R S T)`, `(compose-negative-r R S T)` | Schröder's law for a composite T = R ; S that composition alone produces: `(not (T a c))` ∧ `(R a b)` ⟹ `(not (S b c))`, and dually `(not (T a c))` ∧ `(S b c)` ⟹ `(not (R a b))`. Closed-world: declared per triple |

A program with closures and the Boolean layer, solved at root:

```lisp
(import std.algebra :symbols (symmetric transitive meet))

(relation linked T T :why "{?1} is linked to {?2}")
(relation owns   T T :why "{?1} owns {?2}")
(relation rents  T T :why "{?1} rents {?2}")
(relation either T T :why "{?1} owns or rents {?2}")
(relation shares T T :why "{?1} owns and rents {?2}")

(symmetric linked)
(transitive linked)
(join owns rents either)
(meet owns rents shares)

(linked a b) (linked b c)
(owns Ann Car) (owns Ann Bike)
(rents Ann Car) (rents Bob Bike)

(solve :no-search)
(query :goal (linked a ?x))
(query :goal (either ?p ?t))
(query :goal (shares ?p ?t))
```

```text
  query 1 of 3      (linked a ?x)
  as :expect        (model (linked a b) (linked a c))
  query 2 of 3      (either ?p ?t)
  as :expect        (model (either Ann Bike) (either Ann Car)
                           (either Bob Bike))
  query 3 of 3      (shares ?p ?t)
  as :expect        (model (shares Ann Car))
```

`(linked a a)` is not derived: the transitive closure's guard keeps it irreflexive.

## `std.bijection` — bijections, read off the signature

The inference a bijection licenses, beside `std.algebra`'s checks: negative completion (a positive excludes the
alternatives), elimination (the last candidate left is forced), and an argument's type check. The types are read
from `(relation R A B)`, and the membership relation is a parameter, so a program names it once. Brings
`std.algebra` and `std.macro`.

```lisp
(import std.bijection)
(bijective wears)             ; per relation
(bijection-hierarchy is-a)    ; direct membership: elimination, totality, negatives
(typecheck-hierarchy is-a*)   ; the transitive closure: the argument type check
```

`bijective`, `bijection-hierarchy` and `typecheck-hierarchy` are no module's names, so this needs no list. The two
setup rules fan each `(bijective R)` out into the activators below, carrying the hierarchy as an argument; a relation
that is a function and not a bijection can skip the setup and declare its activators directly, listing them.

| symbol | form | effect |
|---|---|---|
| setup | `bijective-setup` | `(bijective R)` and `(bijection-hierarchy isa)` ⟹ `(total R isa)`, `(surjective R isa)`, `(total-owed R isa)`, `(surjective-owed R isa)`, `(domain-elimination R isa)`, `(range-elimination R isa)`, `(functional-negative R isa)`, `(injective-negative R isa)` |
| setup | `typecheck-setup` | `(bijective R)`, R typed `A × B`, and `(typecheck-hierarchy isa)` ⟹ `(typecheck-arg-0 R isa A)`, `(typecheck-arg-1 R isa B)` |
| negative completion | `(functional-negative R isa)` | `(R a b)` ⟹ `(not (R a b′))` for every other `b′` of `b`'s type |
| negative completion | `(injective-negative R isa)` | `(R a b)` ⟹ `(not (R a′ b))` for every other `a′` of `a`'s type |
| elimination | `(domain-elimination R isa)` | R functional and total: every `b` but one stated excluded for `a` ⟹ `(R a b)` |
| elimination | `(range-elimination R isa)` | R injective and surjective: the dual |
| check | `(typecheck-arg-0 R isa Dom)`, `(typecheck-arg-1 R isa Ran)` | an argument that is not an `isa`-member of the declared type ⟹ `(false)` |

```lisp
(import std.bijection)

(relation is-a  T T          :why "{?1} is a {?2}")
(relation wears Agent Colour :why "{?1} wears {?2}")

(is-a Ann Agent)  (is-a Bob Agent)    (is-a Cid Agent)
(is-a Red Colour) (is-a Green Colour) (is-a Blue Colour)

(bijective wears)
(bijection-hierarchy is-a)

(not (wears Ann Red))
(wears Bob Green)

(query :goal (wears ?who ?colour))
```

Root alone decides it: `ein solve` prints *universe: no candidate*, R is {∅}, and the query's one class is
`(model (wears Ann Blue) (wears Bob Green) (wears Cid Red))`. [`examples/zebra2.ein`](../../../../examples/zebra2.ein)
is the Zebra puzzle in this encoding.

## `std.elim` — elimination with positional markers

`std.bijection`'s elimination and type check, with the argument types named in each activator and the properties
written as positional markers, `(functional R 0 1)` and `(total R 0)`. Brings `std.macro`. The markers are no
module's names: a file that also lists `functional` or `total` from `std.algebra` makes them `std.algebra`'s, and
this module no longer reads them.

```lisp
(import std.elim :symbols (domain-elimination no-room-left typecheck-arg-0 typecheck-arg-1))
(functional holds 0 1)
(total      holds 0)
(domain-elimination holds is-a Slot Value)
```

| activator | effect |
|---|---|
| `(typecheck-arg-0 R isa Dom)`, `(typecheck-arg-1 R isa Ran)` | argument 0 or 1 not an `isa`-member of `Dom` or `Ran` ⟹ `(false)` |
| `(domain-elimination R isa OT VT)` | R functional and total: one `VT` left for an `OT` member `a` ⟹ it is forced |
| `(no-room-left R isa OT VT)` | R functional and total: every `VT` excluded for `a` ⟹ `(false)` |

## `std.slots` — one co-location relation whose classes are slots

Where `std.bijection` equips one relation a attribute, each a bijection onto the positions, this module equips
**one** relation shared by every attribute: an equivalence R whose classes are *slots*, each holding exactly one
member of every attribute type. `(bijective R)` cannot say this, since the property holds between each pair of
types and not of R as a whole; so it is scoped by a type family instead. Brings `std.macro`; R's symmetry is
`std.algebra`'s, which the program imports and declares.

```lisp
(import std.algebra :symbols (symmetric symmetric-negative-setup))
(import std.slots)
(symmetric co-located)
(slot-partition co-located instance type Attribute House)
(slot-spatial   co-located next-to instance House)
```

`(slot-partition R isa sub Super Index)` reads: *R is an equivalence relation; its classes are slots; each slot
holds exactly one `isa`-member of every type that is a direct `sub`-child of `Super`; and a slot's `Index` member is
its name*. `(slot-spatial R S isa PT)` reads: *S relates two values exactly when it relates their slots, and the
`PT` members carry S's structure*, so one relation name is both a constraint between values, `(right-of Green
Ivory)`, and the structure it is resolved against, `(right-of House-2 House-1)`; the rules tell them apart by
membership of `PT`, never by name.

Transitivity holds of R and justifies the rules, but must **not** be declared: each conclusion is anchored at the
slot's index, which keeps the derived facts on the attribute × index rectangle rather than every pair inside a
class. A puzzle whose clues never name an index would need `(transitive R)` and pays for it.

| rule | from | effect |
|---|---|---|
| `slot-locate` | `slot-partition` | `(R a b)` and `(R b i)`, `i` an index ⟹ `(R a i)`: a cross-attribute clue becomes a placement |
| `slot-exclusive` | `slot-partition` | two distinct members of one slot type ⟹ `(not (R a b))` |
| `slot-occupied` | `slot-partition` | `(R a b)` ⟹ `(not (R a c))` for every other `c` of `b`'s type |
| `slot-negative` | `slot-partition` | `(R a b)` and `(not (R b i))` ⟹ `(not (R a i))` |
| `slot-elimination` | `slot-partition` | every index but `i` excluded for `a` ⟹ `(R a i)` |
| `slot-fill` | `slot-partition` | every member of a type but `v` excluded for slot `i` ⟹ `(R i v)` |
| `slot-no-room`, `slot-no-fill` | `slot-partition` | a value with every slot excluded, a slot with every member of a type excluded ⟹ `(false)` |
| `slot-owed-room`, `slot-owed-fill` | `slot-partition` | the obligations of the two above: a value in no slot yet, a slot with no member of a type yet ⟹ `(open R)` |
| `slot-adjacent-fwd`, `-bwd` | `slot-spatial` | `(S a b)` with `b` at a position whose only S-neighbour is `p` ⟹ `a` at `p`, and the reverse |
| `slot-adjacent-fwd-neg`, `-bwd-neg` | `slot-spatial` | their contrapositives: a negative on one side of a spatial clue reaches the other |
| `slot-prune-fwd`, `-bwd` | `slot-spatial` | once one end is placed, the other is excluded from every position that is not its S-neighbour |
| `slot-endpoint-fwd`, `-bwd` | `slot-spatial` | a position with no S-source cannot hold `b`, one with no S-target cannot hold `a` |

Every negative is a stated consequence of the partition, except the `absent` guards of the spatial rules, which read
the position structure alone: sound where that structure is decided by saturation, as a row of houses is.

```lisp
(import std.algebra :symbols (symmetric symmetric-negative-setup))
(import std.slots)

(relation type     T T                 :why "{?1} is a kind of {?2}")
(relation instance T T                 :why "{?1} is a {?2}")
(relation co-loc   Attribute Attribute :why "{?1} and {?2} are in the same seat")

(symmetric co-loc)
(slot-partition co-loc instance type Attribute Seat)

(type Seat Attribute) (type Who Attribute) (type Pet Attribute)
(instance S1 Seat) (instance S2 Seat)
(instance Ann Who) (instance Bob Who)
(instance Cat Pet) (instance Dog Pet)

(co-loc Ann Cat)
(co-loc Ann S1)

(query :goal (co-loc ?who S2))
```

R is {∅}, and the query's one class is `(model (co-loc Bob S2) (co-loc Dog S2))`: Ann and the cat take seat 1, so
Bob and the dog are left seat 2. [`examples/zebra.ein`](../../../../examples/zebra.ein) is the Zebra puzzle in this
encoding, and reaches the same model as `zebra2.ein`.

## `std.typing` — a type hierarchy, named once

Brings `std.algebra`.

| symbol | form | effect |
|---|---|---|
| knob | `(type-hierarchy isR*)` | name the subtype relation once: `type-hierarchy-converse` then derives, for every `(converse R1 R2)`, the activators `(converse-illtyped-dom R1 R2 isR*)` and `(converse-illtyped-ran R1 R2 isR*)` |
| closure | `(reflexive R)` | `derive-reflexive` activates `reflexive-dom` and `reflexive-cod`: every node R touches, in either position, gets `(R a a)`, which makes an irreflexive `is-a*` a preorder |

```lisp
(import std.algebra :symbols (converse))
(import std.typing)
(converse owns owned-by)
(type-hierarchy is-a*)
```

## `std.closure` — closing a relation that is a total function

| activator | effect |
|---|---|
| *none* | `infer-closure`: `(functional R)` and `(total R)` ⟹ `(__closed__ R)`, so no hypothesis about R is proposed |

Brings `std.algebra`, whose one-argument markers it reads. **The import is the switch**, and it is a completeness
hazard: `functional ∧ total` is a witness about how saturation reaches R, not about R itself, so it is safe only where
R is decided by saturation. Imported into a puzzle that solves R by hypotheses, it closes R and takes away the
candidates the search needs.

```lisp
(import std.closure)
```

## The calculator: `std.math.*`

Arithmetic in Ein is a theory over names. A digit is one of the ten names `0` to `9`; a number is a term built from
them; what school arithmetic knows of the digits is written down once, as tables of facts; and the rules are the
wiring that looks each column up in a table. Nothing in the language knows what `7` is.

Each module, and the modules it imports, which an import of it brings:

```text
std.math.decimal         ←  std.math.arith, std.math.digits
std.math.positional      ←  std.math.arith
std.math.decimal-arith   ←  std.math.decimal, std.math.positional
std.math.binary          ←  std.math.arith, std.math.bits
std.math.binary-arith    ←  std.math.binary, std.math.positional
std.math.convert         ←  std.math.decimal-arith, std.math.binary-arith
std.math.int             ←  std.math.decimal-arith
std.math.gcd             ←  std.math.decimal-arith
std.math.rat             ←  std.math.int, std.math.gcd
std.math.num9            ←  std.math.digits, std.math.decimal-arith
```

What every module of the calculator shares:

- **A calculation is a request fact, answered by a result fact that repeats its operands**
  ([Q-M1f.363](../../../decisions/stdlib/math/Q-M1f.363-a-calculation-is-a-request-fact-answered.md)): the program
  states `(add X Y)`, the theory derives `(sum-of X Y S)`, and a query reads it with a variable in the result's
  place, `(query :goal (sum-of X Y ?s))`. A rule of the program may match a result like any fact. Nothing is
  computed that was not asked for.
- **It searches nothing.** Each module's entry closes every relation of the module and every table it reads
  ([Q-M1f.365](../../../decisions/stdlib/math/Q-M1f.365-the-calculator-closes-its-own-relations-so.md)), so a
  program that imports the calculator and states requests has no candidate: `ein solve` prints *universe: no
  candidate*, R is {∅} or empty, and the answer is root's fixpoint under `ein solve`, `ein saturate` and `ein test`
  alike. A relation the importing program declares is the program's to close.
- **A fault is a flag, and the flag is a contradiction.** A division by zero, a malformed numeral, a digit where a
  number goes and a name no `let` states each derive a flag fact, and a second rule derives `(false)` from the flag,
  so a trace shows the flag as one step and the contradiction as the next. Root is dead, R is empty, every command
  exits 0, and `ein solve` names, in `unsat core`, the fact the fault was written in, and the rule in `refuted by`
  ([§ Malformed numbers](#malformed-numbers-what-is-flagged-and-where)). A program whose root is dead claims only
  `(solve :expect (false))`.
- **No rule joins two facts on a number**
  ([Q-M1f.362](../../../decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)): each
  rule has one premise that holds numbers, and its others are table rows looked up by digits, guards, or a probe.
  A theory a program builds on the calculator keeps to it by the **tagged calls**: a request's result repeats its
  operands, so a rule that wanted an answer for a purpose of its own would have to find it by its operands, a join
  on a number; a tagged call hands the answer back under a tag the caller chose, and the rule that reads it has one
  premise.

The guide's [word problems](../02-guide/08-word-problems.md) are programs written over these modules.

### `std.math.digits` — the digit tables

A library of facts: no rule and no entry. Each table is complete and closed by a written `(__closed__ R)` fact of the
module ([Q-M1f.146](../../../decisions/stdlib/math/Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md)), which
every import brings, so no row is ever a hypothesis. The file is generated and not edited by hand, and every row is
held to the integers ([Q-M1f.366](../../../decisions/stdlib/math/Q-M1f.366-the-calculators-tables-are-generated-facts-every.md)).
A theory that reads a table closes it in its own entry, and a program that imports the theory lists nothing from
here; one that reads a table itself lists it:

```lisp
(import std.math.digits :symbols (fa nz __closed__))
```

| row | reads | gives | rows |
|---|---|---|---:|
| `(fa a b cin d cout)` | two digits and a carry in, `0` or `1` | `a + b + cin = 10·cout + d` | 200 |
| `(fs a b bin d bout)` | two digits and a borrow in, `0` or `1` | `a − b − bin = d − 10·bout` | 200 |
| `(md a b cin d cout)` | two digits and a carry in, `0` to `8` | `a × b + cin = 10·cout + d` | 900 |
| `(cl below a b order)` | the order of the digits below, `lt`, `eq` or `gt`, and one digit of each number | `lt` if `a < b`, `gt` if `a > b`, else `below` | 300 |
| `(nz d)` | a digit | it is not zero: `1` to `9` | 9 |
| `(mid lo hi d)` | two bounds from `0` to `10`, at least two apart | the digit between them to try, `(lo + hi) / 2` rounded down | 45 |
| `(adj lo hi)` | two bounds | nothing lies between them: `hi = lo + 1` | 10 |

A core that passes through a table names the row, since a row is a premise like any other fact.

### `std.math.arith` — zero, and the division by zero

What is true of a number in every base, and so is no base's own:

- **`z`, zero**, the number with no digit: every numeral ends in it, decimal's and binary's alike
  ([Q-M1f.322](../../../decisions/stdlib/math/Q-M1f.322-zero-is-one-z-in-every-base.md)). It is declared as a
  relation of no argument that no fact is ever of, so that it has one full name
  ([Q-M1f.283](../../../decisions/stdlib/math/Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md)), and a file
  that writes it lists it.
- **Division by zero is a contradiction**, in every base
  ([Q-M1f.269](../../../decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md)): a module that
  divides derives the flag `(division-by-zero REQUEST)`, the request or the call as it was asserted, and
  `division-by-zero-is-false` derives `(false)` from it. A theory in which division by zero is allowed would be a
  module that does not import this one; there is none.

```lisp
(import std.math.arith :symbols (z division-by-zero))
```

Entry `arith`. A program lists `z` where it writes a numeral, and `division-by-zero` where a rule or a claim reads the
flag.

### `std.math.decimal` — the decimal numeral

The decimal **representation**: the numeral and what a written one must be
([Q-M1f.273](../../../decisions/stdlib/math/decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md)). It
computes nothing. A natural number is `z`, or `(n PREFIX DIGIT)`, ten times the prefix plus the digit, read in the
order it is written ([Q-M1f.361](../../../decisions/stdlib/math/decimal/Q-M1f.361-a-natural-number-is-a-numeral-of-digit-names.md)):

| number | numeral |
|---|---|
| 7 | `(n z 7)` |
| 42 | `(n (n z 4) 2)` |
| 123078 | `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)` |

A digit is one of `0` to `9`, a prefix `z` or a numeral. Any length; a program may write high zeros, `(n (n z 0) 7)`
is 7, and the arithmetic answers in normal form, with none, so zero is `z` and never `(n z 0)`.

```lisp
(import std.math.decimal :symbols (n))
```

| fact | who writes it | meaning |
|---|---|---|
| `(n PREFIX DIGIT)` | the program | the numeral |
| `(named NAME)` | `std.math.decimal-arith`, for each name a `let` states | NAME stands for a number |
| `(not-decimal NUMERAL THING)` | this module: a **flag** | NUMERAL, written somewhere in the program, holds THING where a digit or a prefix goes; `not-decimal-is-false` derives `(false)` |

**A numeral is checked where it is written**
([Q-M1f.210](../../../decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md)). The
loader states every fact nested in a fact a file writes as `(written T)` ([reserved names](03-reserved-names.md)), so
the module's rules see every place of every numeral, as an operand, inside an integer or a rational, as a `let`'s
value nobody asks about, or in a fact of the program's own relation, and flag a malformed one in root's first layer,
before any calculation, on its smallest `n` term at fault
([Q-M1f.274](../../../decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)).
What it flags and what it leaves silent is [§ Malformed numbers](#malformed-numbers-what-is-flagged-and-where). A
named constant is told by `named`
([Q-M1f.278](../../../decisions/stdlib/math/decimal/Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md)).
Entry `decimal`.

### `std.math.decimal-arith` — school arithmetic on natural numbers

Addition, subtraction, comparison, multiplication and division of natural numbers of any length, computed by
saturation alone over `std.math.digits`' tables
([Q-M1f.261](../../../decisions/stdlib/math/decimal/Q-M1f.261-the-decimal-arithmetic-is-std-math-decimal-arith.md)).
There is no width and no overflow.

```lisp
(import std.math.decimal-arith :symbols (decimal-arith add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add (n (n (n z 1) 2) 3) (n (n z 4) 5))

(query :goal (sum-of ?x ?y ?s))
```

```text
  universe          no candidate: R is {∅} or empty, decided by root alone
  …
  query             (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n (n (n z 1) 6) 8) · ?x = (n (n (n z 1) 2) 3) · ?y = (n (n z 4) 5)
```

| fact | who writes it | meaning |
|---|---|---|
| `(add X Y)` | the program: a **request** | X + Y is asked |
| `(sum-of X Y S)` | the theory: the **result** | X + Y = S, the operands as the request wrote them, S in normal form |
| `(sub X Y)` | the program: a request | X − Y is asked |
| `(diff-of X Y D R)` | the theory: the result | **two numbers** ([Q-M1f.364](../../../decisions/stdlib/math/decimal/Q-M1f.364-a-subtraction-answers-the-difference.md)): D is X − Y where X is not the smaller, else `z`; R is Y − X where X is the smaller, else `z`. One is always `z`, both when X = Y. There is no negative number: `(diff-of X Y z (n ?p ?a))` matches exactly where X is the smaller |
| `(cmp X Y)` | the program: a request | X is compared with Y |
| `(cmp-of X Y O)` | the theory: the result | O is `lt`, `eq` or `gt`, by value: `5` and `005` are `eq` |
| `(mul X Y)` | the program: a request | X × Y is asked |
| `(product-of X Y P)` | the theory: the result | X × Y = P, of whatever length |
| `(div X Y)` | the program: a request | X ÷ Y is asked |
| `(quot-of X Y Q R)` | the theory: the result | X = Q × Y + R, R less than Y |
| `(let NAME NUMBER)` | the program | NAME names NUMBER, and a request may name an operand ([Q-M1f.149](../../../decisions/stdlib/math/decimal/Q-M1f.149-a-request-may-name-an-operand-by-a-let.md)) |
| `(malformed REQUEST)`, `(malformed (let NAME N))`, `(undeclared NAME REQUEST)` | the theory: **flags** | [§ Malformed numbers](#malformed-numbers-what-is-flagged-and-where) |
| `(division-by-zero (div X Y))` | the theory: a flag, `std.math.arith`'s | the divisor is zero by value, however written |

- **An operand is read by value** and may carry high zeros; every answer is in normal form. A request is answered
  once. The same request written with other spellings of its operands, a name for a number or a high zero more, is
  another request with its own answer.
- **Named constants.** With `(let price …)` and `(let qty …)`, `(add price qty)` is answered by
  `(sum-of price qty S)`, the names repeated as written. A name stands for one number, never for another name.
- **Division by zero.** `(div X Y)` with Y zero by value derives `(division-by-zero (div X Y))`, and `(false)` from
  it: root is dead, R is empty, no query has an answer, and `ein solve` names the request as the unsat core and
  `std.math.arith.division-by-zero-is-false` under `refuted by`. It is flagged wherever division is called, through
  the tagged call too, so a public division by zero holds two flags, the request's and its call's
  ([Q-M1f.163](../../../decisions/stdlib/math/decimal/Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md),
  [Q-M1f.211](../../../decisions/stdlib/math/decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md)), and
  is decided by root alone ([Q-M1f.165](../../../decisions/stdlib/math/decimal/Q-M1f.165-a-program-that-divides-by-zero-is-decided.md)).
  A program that must not die compares its divisor with zero first, and derives its `div` request by a rule over
  `(cmp-of Y z gt)`.

**The tagged calls**, an interface for a theory built on this one
([Q-M1f.189](../../../decisions/stdlib/math/Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md),
[Q-M1f.228](../../../decisions/stdlib/math/Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md)). A call
looks nothing up: X and Y are numbers as `n` and `z` build them, high zeros allowed, never a name. TAG is a fact of
the caller's own relation, which the answer carries back; a tag of the theory's own, `(sum X Y)` or `(quotient X Y)`,
is read back as one of its requests. A tag names one call: two calls under one tag answer under one tag, and the
rule that reads cannot tell them apart.

```text
(add-walk TAG X Y C z)   asked: X + Y + C, C a carry in of 0 or 1   answered by (done TAG S)
(sub-walk TAG X Y 0 z)   asked: X − Y                               answered by (done TAG D) where X is not
                                                                    the smaller, by (short TAG) where it is
(cmp-walk TAG X Y eq)    asked: X against Y                         answered by (order-is TAG O)
(mul-call TAG X Y)       asked: X × Y                               answered by (done TAG P)
(div-call TAG X Y)       asked: X ÷ Y                               answered by (div-done TAG Q R);
                                                                    flagged where Y is zero
(operand TAG X)          wanted: what X stands for, a number or     answered by (operand-is TAG N)
                         a name a let states
```

A walk starts with `z`, no digits yet; a subtraction with `0`, no borrow; a comparison with `eq`, nothing decided:
those are the values a caller writes. [`examples/beyond_arith/casting_out_nines.ein`](../../../../examples/beyond_arith/casting_out_nines.ein)
calls the theory with a tag of its own.

**How it computes**, which a trace shows. Each operand is looked up; one walk from the units takes each column through
the full adder's table and puts its digit in front of an accumulator, `(b DIGIT REST)`; high zeros are dropped there;
the answer is turned round into the public shape and handed over as `(done TAG ANSWER)`. Every call carries its
continuation as its tag. Subtraction is two walks side by side, X − Y and Y − X, since which is smaller is not known
before the top column: the walk that ends with no borrow answers. Comparison is one walk over the comparator's
table. Multiplication is the school method from the multiplier's top digit. Division is long division, each digit of
the quotient found by bisection over the `mid` and `adj` tables, at most four trials a digit
([Q-M1f.324](../../../decisions/stdlib/math/decimal/Q-M1f.324-decimal-division-is-long-division-each-digit.md)).
The circuit's own relations — the accumulator `b`, the walks, the steps, the tags, `arith-rule` — are in the state
for a query or a trace to see, and are not an interface; the module's header lists every name it occupies.

**How long a number may be.** The theory has no width. A program can write a number of at most 999 digits, since a
numeral nests a level a digit and the parser refuses a program nested past 1 000 parentheses
([diagnostics](07-diagnostics.md)); a rule may derive a longer one. A step of a circuit holds its operands inside its
tag, two levels deeper than the request for a sum, a difference and a comparison and three for a product and a
quotient, so a state written by `--dump-answer` from operands near that length is refused when it is loaded again,
though every answer is right and printed. A product's or a quotient's facts grow as the two lengths multiplied, each
holding its numbers whole, so a long product is a query's to read rather than a state to write out.

### `std.math.positional` — the rules every base shares

Most of school arithmetic does not know its base: a walk takes a column and looks it up in a table, and nothing in it
says *ten* or *two*. Those rules are written here once, for decimal and binary, every relation a rule names a
parameter in capitals:

```lisp
(rule add-column (?ADD-WALK ?N ?FA ?B)
  :match  (and (?ADD-WALK ?k (?N ?p ?a) (?N ?q ?c) ?e ?acc)
               (?FA ?a ?c ?e ?s ?f))
  :assert (?ADD-WALK ?k ?p ?q ?f (?B ?s ?acc))
  :why    "{?a} + {?c} + carry {?e} is {?s}, carry {?f}: {?s} goes in front of the digits so far")
```

The module declares no relation and has one macro, `positional`, whose one call inside a base's closing rule
activates every shared rule with that base's relations
([Q-M1f.266](../../../decisions/stdlib/math/Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md)):
`(positional operand operand-is n let named add-walk fa b strip turn nz done …)` in decimal's,
`(positional bin-operand … b2 bin-let …)` in binary's. A program never writes it and never imports this module:
it comes with a base. A shared rule prints by its own full name whichever base activated it,
`std.math.positional.add-column` in a trace, `std.math.positional.malformed-is-false` under `refuted by`.

### Binary: `std.math.bits`, `std.math.binary` and `std.math.binary-arith`

Binary arithmetic is a theory of its own beside decimal's
([Q-M1f.321](../../../decisions/stdlib/math/binary/Q-M1f.321-binary-arithmetic-is-a-theory-of-its-own-beside.md)):
its own numeral, its own names, every one `bin-`, and the same shared rules. Nothing of decimal is loaded with it: a
program that imports only binary holds no decimal row. A program that imports both shares `z`, `0`, `1`,
`std.math.arith` and the shared rules, and nothing else.

| module | what it is |
|---|---|
| `std.math.bits` | the tables, facts and no rule, generated: `(bin-fa a b cin d cout)`, 8 rows; `(bin-fs a b bin d bout)`, 8; `(bin-cl below a b order)`, 12; `(bin-nz 1)`. A program that reads a table imports this module ([Q-M1f.192](../../../decisions/stdlib/math/Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md)) |
| `std.math.binary` | the numeral `(b2 PREFIX BIT)`, twice the prefix plus the bit, `bin-named`, and its check where it is written ([Q-M1f.295](../../../decisions/stdlib/math/binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)); entry `binary` |
| `std.math.binary-arith` | the arithmetic, `std.math.decimal-arith`'s twin ([Q-M1f.267](../../../decisions/stdlib/math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md)); entry `binary-arith` |

```lisp
(import std.math.binary-arith :symbols (binary-arith bin-add bin-sum-of bin-mul bin-product-of))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(bin-add (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 + 111
(bin-mul (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))   ; 101 × 111

(query :goal (bin-sum-of ?x ?y ?s))
(query :goal (bin-product-of ?x ?y ?p))
```

The sum is `(b2 (b2 (b2 (b2 z 1) 1) 0) 0)`, 1100, and the product `(b2 (b2 (b2 (b2 (b2 (b2 z 1) 0) 0) 0) 1) 1)`,
100011.

| fact | who writes it | meaning |
|---|---|---|
| `(b2 PREFIX BIT)` | the program | the numeral; the prefix is `z` or a numeral, the bit `0` or `1`: 5 is `(b2 (b2 (b2 z 1) 0) 1)` |
| `(bin-add X Y)` / `(bin-sum-of X Y S)` | the program / the theory | X + Y |
| `(bin-sub X Y)` / `(bin-diff-of X Y D R)` | the program / the theory | **two numbers**, as decimal's `diff-of` |
| `(bin-cmp X Y)` / `(bin-cmp-of X Y O)` | the program / the theory | O is `lt`, `eq` or `gt`, by value |
| `(bin-mul X Y)` / `(bin-product-of X Y P)` | the program / the theory | X × Y = P |
| `(bin-div X Y)` / `(bin-quot-of X Y Q R)` | the program / the theory | X = Q × Y + R, R less than Y |
| `(bin-let NAME NUMBER)` | the program | NAME names NUMBER |
| `(not-binary NUMERAL THING)` | `std.math.binary`: a **flag** | a bit's place holds no bit, or a bit or a named constant is the prefix |
| `(bin-malformed REQUEST)`, `(bin-undeclared NAME REQUEST)` | the theory: **flags** | a bit where a number goes, a `bin-let` that cannot be read; a name no `bin-let` states ([Q-M1f.268](../../../decisions/stdlib/math/binary/Q-M1f.268-binary-flags-a-malformed-request-as-far-as.md)) |
| `(division-by-zero (bin-div X Y))` | the theory: a flag, `std.math.arith`'s | the divisor is zero by value; its call is flagged beside it |

A decimal digit is a name in binary: `(bin-add 7 X)` is undeclared, and `(b2 7 1)` is silent. The tagged calls are
decimal's with `bin-` before each name ([Q-M1f.297](../../../decisions/stdlib/math/binary/Q-M1f.297-binarys-tagged-calls-are-an-interface.md)):
`(bin-add-walk TAG X Y C z)`, `(bin-sub-walk TAG X Y 0 z)`, `(bin-cmp-walk TAG X Y eq)`, `(bin-mul-call TAG X Y)`,
`(bin-div-call TAG X Y)` and `(bin-operand TAG X)`, answered by `bin-done`, `bin-short`, `bin-order-is`,
`bin-div-done` and `bin-operand-is`. Binary has no digit multiplier and no bisection: a row of a product is nothing or
the multiplicand, and one subtraction decides a bit of a quotient.

### `std.math.convert` — decimal to binary and back

A module of its own over both bases
([Q-M1f.323](../../../decisions/stdlib/math/Q-M1f.323-conversion-between-the-bases-is-a-module.md)), so that a
program that computes in one base loads nothing of the other.

```lisp
(import std.math.convert :symbols (convert to-bin to-dec bin-of dec-of))
(import std.math.decimal :symbols (n))
(import std.math.binary :symbols (b2))
(import std.math.arith :symbols (z))

(to-bin (n (n z 1) 3))
(to-dec (b2 (b2 (b2 (b2 z 1) 0) 1) 0))

(query :goal (bin-of ?x ?b))
(query :goal (dec-of ?b ?x))
```

13 is `(b2 (b2 (b2 (b2 z 1) 1) 0) 1)` and 1010 is `(n (n z 1) 0)`.

| fact | who writes it | meaning |
|---|---|---|
| `(to-bin X)` | the program: a request | X, a decimal number, is asked in binary |
| `(bin-of X B)` | the theory: the result | B is X in binary, in normal form |
| `(to-dec B)` | the program: a request | B, a binary number, is asked in decimal |
| `(dec-of B X)` | the theory: the result | X is B in decimal, in normal form |

An operand is a number; a name is not looked up. `to-bin` halves X by decimal's tagged division until nothing is
left, each remainder a bit; `to-dec` reads B's bits from the top and doubles the number so far by one addition walk,
the bit as its carry in.

### `std.math.int` — integers

An integer is `(int P N)`, two natural numbers of `std.math.decimal-arith`, whose value is P less N
([Q-M1f.352](../../../decisions/stdlib/math/int/Q-M1f.352-an-integer-is-two-parts-a-positive.md)): −7 is
`(int z (n z 7))`, zero `(int z z)`, and `(int (n z 5) (n z 3))` is 2. The **normal form** has one part `z` and the
other in normal form, and every integer has exactly one; an operand is read by value, and every answer is in normal
form, so a rule tells a negative answer by its shape, `(int z (n ?p ?a))`, with no negation written. Every name is
`int-` ([Q-M1f.174](../../../decisions/stdlib/math/int/Q-M1f.174-the-names-of-std-math-int-carry-the-prefix-int.md)).

```lisp
(import std.math.int :symbols (int-arith int int-add int-sum-of int-div-floor int-quot-floor-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(int-add (int z (n (n z 1) 2)) (int (n (n z 4) 0) z))     ; −12 + 40
(int-div-floor (int z (n z 7)) (int (n z 2) z))           ; −7 ÷ 2, rounded down

(query :goal (int-sum-of ?x ?y ?s))
(query :goal (int-quot-floor-of ?x ?y ?q ?r))
```

The sum is `(int (n (n z 2) 8) z)`, 28; the quotient `(int z (n z 4))`, −4, and the remainder `(int (n z 1) z)`, 1.

| request | result | meaning |
|---|---|---|
| `(int-simplify X)` | `(int-simple-of X S)` | S is X's normal form |
| `(int-neg X)` | `(int-neg-of X S)` | S is 0 − X |
| `(int-add X Y)` | `(int-sum-of X Y S)` | X + Y = S |
| `(int-sub X Y)` | `(int-diff-of X Y S)` | X − Y = S, **one integer** |
| `(int-mul X Y)` | `(int-product-of X Y S)` | X × Y = S, computed on the parts with no case on a sign ([Q-M1f.176](../../../decisions/stdlib/math/int/Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md)) |
| `(int-cmp X Y)` | `(int-cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value |
| `(int-div-euclid X Y)` | `(int-quot-euclid-of X Y Q R)` | X = Q·Y + R, 0 ≤ R < \|Y\|; Q an integer, R a **natural number** |
| `(int-div-trunc X Y)` | `(int-quot-trunc-of X Y Q R)` | Q rounded toward zero, R with the dividend's sign; both integers |
| `(int-div-floor X Y)` | `(int-quot-floor-of X Y Q R)` | Q rounded down, R with the divisor's sign; both integers |

**A division names its convention**: there is no bare `int-div`
([Q-M1f.179](../../../decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md),
[Q-M1f.200](../../../decisions/stdlib/math/int/Q-M1f.200-an-integer-divisions-result-is-named-after.md)). The
three agree where neither operand is negative, and with `div` there:

| X ÷ Y | Euclidean Q, R | truncated Q, R | floored Q, R |
|---|---|---|---|
| 7 ÷ 2 | 3, 1 | 3, 1 | 3, 1 |
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

A divisor that is zero by value derives `(division-by-zero REQUEST)`. **An operand** is an integer written out, or a
name a `(let NAME (int P N))` states
([Q-M1f.175](../../../decisions/stdlib/math/int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md)); a
natural number in an integer's place is answered by silence. A digit or a named constant where a part goes,
`(int 7 z)`, is the integer's own fault, `(not-integer INTEGER THING)`, flagged where it is written
([Q-M1f.276](../../../decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)).

The tagged calls a later theory makes, each part a natural number written out:

```text
(int-norm TAG P N)               asked: P less N, in normal form       answered by (int-done TAG INTEGER)
(int-add-call TAG P1 N1 P2 N2)   asked: P1 less N1, plus P2 less N2    answered by (int-done TAG INTEGER)
(int-mul-call TAG P1 N1 P2 N2)   asked: P1 less N1, times P2 less N2   answered by (int-done TAG INTEGER)
(int-cmp-call TAG P1 N1 P2 N2)   asked: P1 less N1, against P2 less N2 answered by (order-is TAG O)
```

Entry `int-arith`, which names `decimal-arith`, so one name brings the natural numbers too. A call's tag is its
request ([Q-M1f.177](../../../decisions/stdlib/math/Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md)).

### `std.math.rat` — rational numbers

A rational is `(rat Z D)`: Z an integer of `std.math.int`, D a natural number that is not zero
([Q-M1f.353](../../../decisions/stdlib/math/rat/Q-M1f.353-a-rational-is-an-integer-over-a-natural-number.md)). 1/2 is
`(rat (int (n z 1) z) (n z 2))`, −3/4 is `(rat (int z (n z 3)) (n z 4))`. Every name is `rat-`
([Q-M1f.180](../../../decisions/stdlib/math/rat/Q-M1f.180-the-names-of-std-math-rat-carry-the-prefix-rat.md)).

**An answer is not reduced; `rat-simplify` reduces, and `rat-cmp` is equality**
([Q-M1f.183](../../../decisions/stdlib/math/rat/Q-M1f.183-a-rational-is-reduced-only-on-request.md)). An
operation answers the cross products, or the numerators' product, over the product of the denominators, as computed,
its numerator in `std.math.int`'s normal form and its denominator with no high zero
([Q-M1f.302](../../../decisions/stdlib/math/rat/Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md)): 1/2 +
1/2 is 4/4, and two equal answers may be two facts. Lowest terms, zero written 0 over 1, is the one normal form every
rational has, and `rat-simplify` answers it, by `std.math.gcd`'s tagged call
([Q-M1f.303](../../../decisions/stdlib/math/rat/Q-M1f.303-lowest-terms-hands-the-numerator-to-the-greatest.md)). A
chain of operations grows unless the program asks `rat-simplify` on the way.

```lisp
(import std.math.rat :symbols (rat-arith rat rat-add rat-sum-of rat-simplify rat-simple-of rat-cmp rat-cmp-of))
(import std.math.int :symbols (int))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(rat-add (rat (int (n z 1) z) (n z 2)) (rat (int (n z 1) z) (n z 2)))   ; 1/2 + 1/2
(rat-simplify (rat (int (n z 4) z) (n z 4)))                            ; 4/4 in lowest terms
(rat-cmp (rat (int (n z 1) z) (n z 2)) (rat (int (n z 2) z) (n z 4)))   ; 1/2 against 2/4

(query :goal (rat-sum-of ?x ?y ?s))
(query :goal (rat-simple-of ?x ?s))
(query :goal (rat-cmp-of ?x ?y ?o))
```

The sum is `(rat (int (n z 4) z) (n z 4))`, 4/4; its lowest terms `(rat (int (n z 1) z) (n z 1))`; and 1/2 against
2/4 is `eq`.

| request | result | meaning |
|---|---|---|
| `(rat-simplify X)` | `(rat-simple-of X S)` | S is X in lowest terms: the one request that reduces |
| `(rat-neg X)` | `(rat-neg-of X S)` | S is 0 − X |
| `(rat-add X Y)` | `(rat-sum-of X Y S)` | X + Y = S |
| `(rat-sub X Y)` | `(rat-diff-of X Y S)` | X − Y = S |
| `(rat-mul X Y)` | `(rat-product-of X Y S)` | X × Y = S |
| `(rat-div X Y)` | `(rat-quot-of X Y S)` | X ÷ Y = S, one rational, exact: there is no remainder |
| `(rat-cmp X Y)` | `(rat-cmp-of X Y O)` | O is `lt`, `eq` or `gt`, by value; nothing is reduced for it |

**A zero denominator is a division by zero**
([Q-M1f.182](../../../decisions/stdlib/math/rat/Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md)): a request any
operand of which has a denominator of zero, and a `rat-div` by a rational that is zero, derive
`(division-by-zero REQUEST)`, and `(false)` from it. **An operand** is a rational written out, `(rat (int P N) D)`,
or a name a `let` states ([Q-M1f.181](../../../decisions/stdlib/math/rat/Q-M1f.181-a-rational-operand-is-a-rational-written-out.md));
an integer or a natural number in a rational's place is answered by silence
([Q-M1f.277](../../../decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)). A digit
or a named constant where the integer or the denominator goes is the rational's own fault,
`(not-rational RATIONAL THING)`.

The tagged calls a later theory makes; the denominators must not be zero:

```text
(rat-norm TAG P N D)                  asked: P less N over D, not reduced   answered by (rat-done TAG RATIONAL)
(rat-mul-call TAG P1 N1 D1 P2 N2 D2)  asked: the product, not reduced       answered by (rat-done TAG RATIONAL)
(rat-cross TAG P1 N1 D1 P2 N2 D2)     asked: P1·D2 + P2·D1 and N1·D2 + N2·D1
                                      answered by (rat-cross-is TAG POSITIVE NEGATIVE)
```

Entry `rat-arith`, which names `int-arith`, `decimal-arith`, `decimal`, `arith` and `gcd`.

### `std.math.gcd` — greatest common divisor and least common multiple

Of two natural numbers, by Euclid's chain over the tagged division
([Q-M1f.336](../../../decisions/stdlib/math/gcd/Q-M1f.336-gcd-and-lcm-are-a-module-of-their-own-over.md)). `gcd` is
both the request and the module's entry.

```lisp
(import std.math.gcd :symbols (gcd lcm gcd-of lcm-of))
(import std.math.decimal-arith :symbols (let))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(let a (n (n (n (n z 1) 0) 7) 1))
(gcd a (n (n (n z 4) 6) 2))           ; gcd(1071, 462)
(lcm (n (n z 1) 2) (n (n z 1) 8))     ; lcm(12, 18)

(query :goal (gcd-of ?x ?y ?g))
(query :goal (lcm-of ?x ?y ?l))
```

The greatest common divisor is `(n (n z 2) 1)`, 21, answered as `(gcd-of a … …)` with the name as written; the least
common multiple `(n (n z 3) 6)`, 36.

| fact | who writes it | meaning |
|---|---|---|
| `(gcd X Y)` / `(gcd-of X Y G)` | the program / the theory | G divides both and is the greatest that does; gcd(X, 0) = X, and gcd(0, 0) = 0 ([Q-M1f.299](../../../decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md)) |
| `(lcm X Y)` / `(lcm-of X Y L)` | the program / the theory | both divide L, the least that both divide; zero where an operand is zero |
| `(gcd-call TAG X Y)` / `(gcd-done TAG G)` | a rule / the theory | the tagged call ([Q-M1f.300](../../../decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md)): X and Y numbers, never a name |

An operand of a request is a natural number, high zeros allowed and read by value, or a name a `let` states; an
integer or a rational in its place is answered by silence. No request of the module divides by zero. An undeclared
name and a digit as an operand are flagged, `(undeclared NAME REQUEST)` and `(malformed REQUEST)`, and `(false)`
follows.

### `std.math.num9` — nine digits in one fact

A second representation of a number: `(num9 d8 d7 d6 d5 d4 d3 d2 d1 d0)`, nine digit names, the most significant
first, a fixed width ([Q-M1f.320](../../../decisions/stdlib/math/num9/Q-M1f.320-std-math-num9-is-a-second-representation.md)).
123078 is `(num9 0 0 0 1 2 3 0 7 8)`. The theory is the same circuit over the same tables, wired the other way: a
rule has a premise a column, so a sum is one firing of one rule, and a product or a quotient a step a digit. The
module's names are decimal's with a 9
([Q-M1f.167](../../../decisions/stdlib/math/num9/Q-M1f.167-the-names-of-std-math-num9-are-decimals-with-a-9.md)), so
it stands beside `std.math.decimal-arith` in one program. Its file is generated and not edited by hand.

```lisp
(import std.math.num9 :symbols (arith9 num9 add9 sum9-of mul9 product9-of overflow))

(add9 (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))
(mul9 (num9 0 0 0 1 0 0 0 0 0) (num9 0 0 0 1 0 0 0 0 0))

(query :goal (sum9-of ?x ?y ?s))
(query :goal (overflow ?r))
```

The sum is `(num9 0 0 0 1 2 7 6 4 5)`; the product does not fit, so `(overflow (mul9 …))` is derived and no
`product9-of`.

| request | result | meaning |
|---|---|---|
| `(add9 X Y)` | `(sum9-of X Y S)` | X + Y = S, where S fits nine digits |
| `(sub9 X Y)` | `(diff9-of X Y D R)` | two numbers, as `diff-of`, one of them zero |
| `(cmp9 X Y)` | `(cmp9-of X Y O)` | O is `lt`, `eq` or `gt` |
| `(mul9 X Y)` | `(product9-of X Y P)` | X × Y = P, where P fits nine digits |
| `(div9 X Y)` | `(quot9-of X Y Q R)` | X = Q × Y + R, R less than Y |
| `(to9 X)` | `(as9 X W)`, or the flag `(big X)` | W is X, a number of `std.math.decimal-arith`, as nine digits; `big` where X is written with more than nine |
| `(from9 W)` | `(of9 W X)` | X is W as a number of `std.math.decimal-arith`, in normal form |

- **Overflow is a fact and no number**
  ([Q-M1f.168](../../../decisions/stdlib/math/num9/Q-M1f.168-overflow-is-a-fact-and-no-number-and-a-division.md)): a
  sum or a product that does not fit derives `(overflow REQUEST)` and no result, never a truncated number. Root
  stays alive, the other requests are answered, and a rule may match the flag.
- **A division by zero** derives `(division-by-zero (div9 X Y))`, and `(false)` from it.
- **An operand is a number written out**
  ([Q-M1f.170](../../../decisions/stdlib/math/num9/Q-M1f.170-a-num9-operand-is-a-number-written-out.md)): the module
  has no named constants. A `num9` with another count of digits is refused at load; one with a digit's place holding
  something else is answered by silence, and the module flags no malformed digit.
- **Nothing converts by itself.** A conversion is a request, and no rule of either module hands a request to the
  other: a program that wants both writes the rules that ask
  ([`examples/arith/num9_both_ways.ein`](../../../../examples/arith/num9_both_ways.ein)).

**Which to use.** `std.math.num9` where every number fits nine digits and there are many requests, since a sum is one
firing where `std.math.decimal-arith` walks a column a firing, and where a fixed width is what is modelled — a
register, a counter, a field of a form — with the overflow as a fact a rule reads. `std.math.decimal-arith` for
numbers of any length, for named constants, and for a program with a request or two. Both, in one program, with a
conversion request where a number crosses.

### Malformed numbers: what is flagged, and where

A fault is flagged by the module whose term it is a fault of
([Q-M1f.210](../../../decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md),
[Q-M1f.316](../../../decisions/stdlib/math/Q-M1f.316-the-calculator-flags-each-malformed-request-it.md)): a
numeral's by its representation, as soon as it is written; an operand's and a `let`'s by the module that reads it.

| what the program wrote | the flag | by |
|---|---|---|
| a digit's place that holds a name that is no digit, `(n (n z 1) x)`, or a fact, `(n z (n z 1))` | `(not-decimal NUMERAL THING)` | `std.math.decimal` |
| a numeral that ends in a digit where `z` goes, `(n 1 2)`, `(n 0 2)`, or in a named constant, `(n price 1)` | `(not-decimal NUMERAL THING)` | `std.math.decimal` |
| the same faults in a binary numeral, `(b2 (b2 z 1) 2)`, `(b2 1 0)`, `(b2 mask 1)` | `(not-binary NUMERAL THING)` | `std.math.binary` |
| a digit or a named constant where a part of an integer goes, `(int 7 z)` | `(not-integer INTEGER THING)` | `std.math.int` |
| a digit or a named constant where a rational's integer or denominator goes, `(rat 7 (n z 2))` | `(not-rational RATIONAL THING)` | `std.math.rat` |
| a digit where a number goes, `(add 7 X)`: seven is `(n z 7)` | `(malformed REQUEST)` | the arithmetic that reads the operand |
| a name stated twice with two numbers; `z` or a digit as a constant's name; a `let` that names a digit or another named constant, `(let seven 7)`, `(let alias price)` | `(malformed (let NAME N))`, whether or not a request uses it ([Q-M1f.207](../../../decisions/stdlib/math/decimal/Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md), [Q-M1f.281](../../../decisions/stdlib/math/decimal/Q-M1f.281-a-let-that-names-a-digit-is-flagged.md)) | `std.math.decimal-arith` |
| an operand that is a name no `let` states, `(add foo X)` | `(undeclared foo REQUEST)` ([Q-M1f.317](../../../decisions/stdlib/math/decimal/Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md)) | the arithmetic that reads the operand |
| an undeclared name or a digit as an operand of `std.math.int`, `std.math.rat` or `std.math.gcd` | `(undeclared …)`, `(malformed REQUEST)`, on the request ([Q-M1f.206](../../../decisions/stdlib/math/decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md)) | that module |

Each flag derives `(false)` by a rule of its own — `not-decimal-is-false`, `not-binary-is-false`,
`not-integer-is-false`, `not-rational-is-false`, `malformed-is-false`, `undeclared-is-false` — so the read-out
reads:

```lisp
(import std.math.decimal-arith :symbols (decimal-arith add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add (n (n z 1) x) (n z 5))

(query :goal (sum-of ?x ?y ?s))
```

```text
  empty             every node is dead — root's own fixpoint derives (false)
  …
  unsat core        (add (n (n z 1) x) (n z 5))
  refuted by        std.math.decimal.not-decimal-is-false
```

The core names the fact the numeral was written in. A numeral written whole in a rule's conclusion rests on no fact,
and its core is ∅.

**A flag means malformed; no flag does not mean well-formed**
([Q-M1f.318](../../../decisions/stdlib/math/Q-M1f.318-a-flag-means-malformed-and-no-flag-does-not-mean.md)). These
are answered by silence, a request neither refused nor answered:

- a numeral that ends in a name that is neither a digit nor a named constant, `(n end 1)`, and in binary a decimal
  digit as the prefix, `(b2 7 1)`;
- a fact of another relation where a number goes, as an operand or inside a number, `(add (pair 1 2) X)`;
- a name that stands for either of those;
- a numeral a rule assembles from a variable, `(n ?p x)`, which is no term a file writes, and a top-level fact of `n`;
- an operand of another kind of number: a natural number where `std.math.int` takes an integer, an integer or a
  natural number where `std.math.rat` takes a rational, an integer or a rational where `std.math.gcd` takes a natural
  number ([Q-M1f.277](../../../decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md));
- every malformed digit of `std.math.num9`.

A wrong count of arguments, `(add X)`, `(n 1)`, a `num9` of eight digits, is refused when the program is loaded,
since a form of a declared relation has the declared count of arguments ([diagnostics](07-diagnostics.md)).

## Boolean functions and circuits: `std.bool.*`

The two bits are the names `0` and `1`, which are decimal's digits too: Ein has no truth values of its own, as it has
no numbers. Four modules, each entered by its last name, which closes its relations
([Q-M1f.265](../../../decisions/stdlib/bool/Q-M1f.265-each-std-bool-module-is-entered-by-its-last-name.md),
[Q-M1f.328](../../../decisions/stdlib/bool/Q-M1f.328-boolean-functions-and-circuits-ship-as-four-std.md)):

```text
std.bool.gates  ←  std.bool.props
                ←  std.bool.circuit  ←  std.bool.inputs  →  std.algebra
```

Nothing but `std.bool.inputs` guesses, and it guesses only an input's value. No rule of the four writes `absent`,
`not` or `or`.

### `std.bool.gates` — the sixteen functions

A function of two bits is its column of four outputs, at inputs 00, 01, 10 and 11; there are sixteen, each with a
name, capitalised because `and`, `or` and `not` are Ein's own words:

| name | column | output | | name | column | output |
|---|---|---|---|---|---|---|
| `Zero` | `f0000` | 0 | | `Nor` | `f1000` | not (a or b) |
| `And` | `f0001` | a and b | | `Xnor` | `f1001` | not (a xor b) |
| `Andn` | `f0010` | a and not b | | `Nsnd` | `f1010` | not b |
| `Fst` | `f0011` | a | | `Orn` | `f1011` | a or not b |
| `Randn` | `f0100` | not a and b | | `Nfst` | `f1100` | not a |
| `Snd` | `f0101` | b | | `Rorn` | `f1101` | not a or b |
| `Xor` | `f0110` | a xor b | | `Nand` | `f1110` | not (a and b) |
| `Or` | `f0111` | a or b | | `One` | `f1111` | 1 |

Two layouts ([Q-M1f.325](../../../decisions/stdlib/bool/Q-M1f.325-a-boolean-function-is-a-value-of-one-table.md)):

| fact | meaning |
|---|---|
| `(gate2 NAME A B OUT)` | the table, 64 rows, generated: NAME of A and B is OUT. A function is a value, so a rule about any function needs no parameter, and a search can make the function a hypothesis |
| `(NAME A B OUT)`, `(Xor 0 1 1)` | a relation a function, its four rows, derived from the table by sixteen rules, `Zero-rows` to `One-rows`, in every program that imports the module ([Q-M1f.263](../../../decisions/stdlib/bool/Q-M1f.263-every-program-that-imports-std-bool-gates.md)) |
| `(column-name NAME COLUMN)` | the column as a second name, `(column-name Xor f0110)`: a value, which any program may write |
| `(inv A B)`, `(le A B)`, `(bit A)` | not A is B; A is at most B; A is a bit |

**A function's name is the module's**, as a relation and as the value in the table, `std.bool.gates.Xor`, so a program
that writes one, as a gate's function too, lists it: `(import std.bool.gates :symbols (Xor And))`. A name a program
writes and does not list is its own, a name no row has: a gate of that function computes nothing, and nothing says so
([Q-M1f.264](../../../decisions/imports/Q-M1f.264-a-gate-functions-name-a-program-writes-and-does.md)).

### `std.bool.props` — what is known of the sixteen

Thirty relations, every fact derived from the table's rows by monotone rules in a module of their own
([Q-M1f.326](../../../decisions/stdlib/bool/Q-M1f.326-the-properties-of-the-sixteen-functions-are.md)): a program that
does not import it pays nothing for them. *For every input* is a conjunction of a function's four rows, and *not in a
class* one witness row, so no rule needs `absent`.

```lisp
(import std.bool.props :symbols (commutative identity-of))
(import std.bool.gates :symbols (Xor And))
```

| relation | meaning |
|---|---|
| `(column F A B C D)` | F gives A, B, C and D at 00, 01, 10 and 11 |
| `(swap-of F G)` | G is F with its inputs exchanged |
| `(complement-of F G)` | G is F with its output negated |
| `(dual-of F G)` | G is De Morgan's dual of F: not F(not a, not b) |
| `(left-negated F G)`, `(right-negated F G)` | G is F with its first, its second input negated |
| `(commutative F)` | F(a, b) = F(b, a) |
| `(associative F)` | F(F(a, b), c) = F(a, F(b, c)) at all eight points, each `(assoc-at F A B C)` |
| `(idempotent F)` | F(a, a) = a |
| `(reads-first F)`, `(reads-second F)` | the output depends on the first, the second input |
| `(ignores-first F)`, `(ignores-second F)` | it does not |
| `(right-identity-of F E)`, `(left-identity-of F E)`, `(identity-of F E)` | F(a, E) = a; F(E, a) = a; both, one-sided too ([Q-M1f.327](../../../decisions/stdlib/bool/Q-M1f.327-identity-and-absorbing-elements-are-derived-one.md)) |
| `(right-absorbing-of F E)`, `(left-absorbing-of F E)`, `(absorbing-of F E)` | F(a, E) = E; F(E, a) = E; both |
| `(preserves-zero F)` / `(escapes-zero F)` | F(0, 0) = 0 / F(0, 0) = 1 |
| `(preserves-one F)` / `(escapes-one F)` | F(1, 1) = 1 / F(1, 1) = 0 |
| `(monotone F)` / `(escapes-monotone F)` | raising an input never lowers the output / one row does |
| `(affine F)` / `(escapes-affine F)` | a sum modulo 2 of inputs and a constant / not |
| `(self-dual F)` / `(escapes-self-dual F)` | F is its own dual / a row and its opposite give the same |
| `(complete-alone F)` | outside all five of Post's classes, so every function can be built from F alone: `Nand` and `Nor` |

### `std.bool.circuit` — a circuit, simulated by saturation

Wires are names and a gate is a fact. Saturation is simulation: a firing a gate, and nothing guessed.

| fact or rule | meaning |
|---|---|
| `(gate INSTANCE FUNCTION IN1 IN2 OUT)` | a gate, written by the program; FUNCTION is one of the sixteen |
| `(val WIRE BIT)` | the wire carries the bit |
| `propagate` | a gate whose two inputs carry values puts the table's output on its output wire |
| `one-value` | a wire that carries both bits ⟹ `(false)` |

A gate whose input carries no value computes nothing, and its output carries none.

```lisp
(import std.bool.circuit :symbols (gate val))
(import std.bool.gates :symbols (Xor And))

(gate g1 Xor a b sum)
(gate g2 And a b carry)
(val a 1) (val b 1)

(query :goal (val sum ?s))
(query :goal (val carry ?c))
```

A half adder: R is {∅}, the sum bit is `0` and the carry `1`.

### `std.bool.inputs` — inputs as hypotheses

The one module of `std.bool` with a search in it. An input wire has exactly one value, and which one is for the
search to try.

| fact | meaning |
|---|---|
| `(is-a WIRE Input)` | written by the program: the wire is an input |
| `(in WIRE BIT)` | a hypothesis: the input is set to the bit |
| `(require WIRE BIT)` | written by the program: a state in which the wire carries the other bit is a contradiction |

*Exactly one* is two declarations of `std.algebra`, which the module states: `(total-owed in is-a)`, so a state with
an input unset owes and is not an answer, and `(functional in)`, so no input has two values. The obligation is also
what proposes the candidates, `(in w 0)` and `(in w 1)` for each input, and no other fact is ever guessed
([Hypotheses](../03-syntax-and-semantics/09-hypotheses.md)). So R is the input assignments that contradict nothing: with
no requirement every assignment, and a query's classes are a truth table's rows; with requirements the assignments
that satisfy them, which is satisfiability.

```lisp
(import std.bool.inputs :symbols (is-a in require))
(import std.bool.circuit :symbols (gate val))
(import std.bool.gates :symbols (Xor))

(gate g1 Xor a b out)
(is-a a Input) (is-a b Input)
(require out 1)

(query :goal (and (in a ?a) (in b ?b)))
```

```text
  (region
    ;; state 1 of 2 — 2 candidates · minimal and maximal · from {(in a 0), (in b 1)}
    (model (in a 0) (in b 1))
    ;; state 2 of 2 — 2 candidates · minimal and maximal · from {(in a 1), (in b 0)}
    (model (in a 1) (in b 0)))
```

Where no assignment satisfies the requirements, R is empty because every alive state owes an input it cannot be
given, not because every state is dead: the read-out says *empty — every alive node owes*, and `--single` answers
`Open`. A state of the search is a partial assignment, each input unset, 0 or 1, so the search grows as 3ⁿ in n
inputs: a demonstration, not a SAT solver. Where every input is to be tried anyway, writing the inputs as test vectors
asks one saturation.

### What they are for

[`examples/binary_logic/`](../../../../examples/binary_logic/README.md) has a program for each: simulation
(saturation alone); a truth table (the inputs as hypotheses, a query's classes the rows); satisfiability (requirements
on the outputs); equivalence of two circuits (R empty where they never differ); a puzzle of knights and knaves; and
synthesis of one missing gate (the function a hypothesis over the table). The full adder built from gates and held
against `std.math.bits`' table is there too, and [`examples/binary_arithmetic/`](../../../../examples/binary_arithmetic/README.md)
has binary arithmetic's.

## See also

- [`stdlib/README.md`](../../../../stdlib/README.md) — the library's own page, beside its modules.
- [Properties and operators](../03-syntax-and-semantics/07-properties-and-operators.md) — what the relation algebra's
  properties mean; [Obligations](../03-syntax-and-semantics/08-obligations.md) — `(open R)`;
  [Macros and imports](../03-syntax-and-semantics/11-macros.md).
- [The syntax reference](02-syntax.md) — the `import` form; [Reserved names](03-reserved-names.md) — `__closed__`,
  `written`, `open` and the kernel relations these modules build on; [Diagnostics](07-diagnostics.md) — what a load
  refuses.
- [Word problems](../02-guide/08-word-problems.md) — programs over the calculator.
- [`examples/arith/`](../../../../examples/arith/) — one program per operation of the calculator;
  [`tests/stdlib/`](../../../../tests/stdlib/) — one program per rule or family, each stating what it must and must
  not derive.
