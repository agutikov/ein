# The relation algebra's rules

How `std.algebra`, `std.typing` and `std.closure` work: every rule of the three, as the module writes it and as the
engine draws it. What a program writes to use them, and what each produces, is
[the page before](02-relation-algebra.md); how to read a drawing is
[the index](README.md#how-a-rule-is-shown).

A rule with parameters is activated by a fact headed by its name: `(rule converse (?R1 ?R2) …)` does nothing until
a program states `(converse right-of left-of)`, and then runs with `?R1` and `?R2` bound to those two relations. A
rule with an empty parameter list is active wherever the module is loaded. The rules read no relation of their
own: every premise is a fact of the relation a parameter names, of `relation` (a declaration, which a state holds
as a fact), or of the membership relation the activator passes as `?isa`.

## `std.algebra`

The module imports `std.macro` for `forall`, and declares nothing but its rules.

### The relative layer

Copying a relation into another, turned round or not, and composing two.

#### `converse`

Mirrors each edge of `?R1` into `?R2` with its ends exchanged.

<!-- rule std.algebra converse -->
```lisp
(rule converse (?R1 ?R2)
  :match  (?R1 ?a ?b)
  :assert (?R2 ?b ?a)
  :why    "(converse {?R1} {?R2}): mirror {?R1}({?a},{?b}) → {?R2}({?b},{?a})")
```

![The rule converse of std.algebra, as the engine draws it](diagrams/03-std.algebra.converse.svg)
<!-- /rule -->

#### `imply1`

Copies a property, a fact of one argument, into another.

<!-- rule std.algebra imply1 -->
```lisp
(rule imply1 (?R1 ?R2)
  :match  (?R1 ?a)
  :assert (?R2 ?a)
  :why    "(imply1 {?R1} {?R2}): {?R1}({?a}) → {?R2}({?a})")
```

![The rule imply1 of std.algebra, as the engine draws it](diagrams/03-std.algebra.imply1.svg)
<!-- /rule -->

#### `imply2-fwd`

Copies each edge of `?R1` into `?R2`, as it stands.

<!-- rule std.algebra imply2-fwd -->
```lisp
(rule imply2-fwd (?R1 ?R2)
  :match  (?R1 ?a ?b)
  :assert (?R2 ?a ?b)
  :why    "(imply2-fwd {?R1} {?R2}): {?R1}({?a},{?b}) → {?R2}({?a},{?b})")
```

![The rule imply2-fwd of std.algebra, as the engine draws it](diagrams/03-std.algebra.imply2-fwd.svg)
<!-- /rule -->

#### `imply2-reverse`

The same rule as `converse`, under a second name.

<!-- rule std.algebra imply2-reverse -->
```lisp
(rule imply2-reverse (?R1 ?R2)
  :match  (?R1 ?a ?b)
  :assert (?R2 ?b ?a)
  :why    "(imply2-reverse {?R1} {?R2}): {?R1}({?a},{?b}) → {?R2}({?b},{?a})")
```

![The rule imply2-reverse of std.algebra, as the engine draws it](diagrams/03-std.algebra.imply2-reverse.svg)
<!-- /rule -->

#### `compose`

Joins an edge of `?R1` and an edge of `?R2` on their shared middle and concludes the edge of `?R3` between the ends.

<!-- rule std.algebra compose -->
```lisp
(rule compose (?R1 ?R2 ?R3)
  :match  (and (?R1 ?a ?b) (?R2 ?b ?c))
  :assert (?R3 ?a ?c)
  :why    "(compose {?R1} {?R2} {?R3}): {?R1}({?a},{?b})∧{?R2}({?b},{?c}) → {?R3}({?a},{?c}).")
```

![The rule compose of std.algebra, as the engine draws it](diagrams/03-std.algebra.compose.svg)
<!-- /rule -->

#### `identity`

The identity on a label: every member of `?Dom`, by the membership relation `?isa`, gets a loop.

<!-- rule std.algebra identity -->
```lisp
(rule identity (?R ?isa ?Dom)
  :match  (?isa ?a ?Dom)
  :assert (?R ?a ?a)
  :why    "(identity {?R} {?isa} {?Dom}): {?a}∈{?Dom} → {?R}({?a},{?a}).")
```

![The rule identity of std.algebra, as the engine draws it](diagrams/03-std.algebra.identity.svg)
<!-- /rule -->

### The lemmas about converses

Three rules with no parameter that derive operator facts from operator facts: a derived `(converse R R)` is itself
an activator of `converse`. Each derivation back is a fact that is already there, so the three converge.

#### `symmetric-is-self-converse`

A symmetric relation is its own converse.

<!-- rule std.algebra symmetric-is-self-converse -->
```lisp
(rule symmetric-is-self-converse ()
  :match  (symmetric ?R)
  :assert (converse ?R ?R)
  :why    "symmetric {?R} ⟹ (converse {?R} {?R})")
```

![The rule symmetric-is-self-converse of std.algebra, as the engine draws it](diagrams/03-std.algebra.symmetric-is-self-converse.svg)
<!-- /rule -->

#### `self-converse-is-symmetric`

And a relation that is its own converse is symmetric.

<!-- rule std.algebra self-converse-is-symmetric -->
```lisp
(rule self-converse-is-symmetric ()
  :match  (converse ?R ?R)
  :assert (symmetric ?R)
  :why    "(converse {?R} {?R}) ⟹ symmetric {?R}")
```

![The rule self-converse-is-symmetric of std.algebra, as the engine draws it](diagrams/03-std.algebra.self-converse-is-symmetric.svg)
<!-- /rule -->

#### `converse-pair-symmetric`

Being converses is mutual.

<!-- rule std.algebra converse-pair-symmetric -->
```lisp
(rule converse-pair-symmetric ()
  :match  (converse ?R1 ?R2)
  :assert (converse ?R2 ?R1)
  :why    "(converse {?R1} {?R2}) ⟹ (converse {?R2} {?R1})")
```

![The rule converse-pair-symmetric of std.algebra, as the engine draws it](diagrams/03-std.algebra.converse-pair-symmetric.svg)
<!-- /rule -->

### The converse's declarations

Two checks of a `(converse R1 R2)` pair against the two `(relation …)` declarations: the second label of one must
meet the first of the other. The two labels pass where they are equal (the `neq` guard) or related by `?isR*`
either way (the two `absent` clusters).

#### `converse-illtyped-dom`

Refutes a pair where `?R1`'s second label and `?R2`'s first are unrelated.

<!-- rule std.algebra converse-illtyped-dom -->
```lisp
(rule converse-illtyped-dom (?R1 ?R2 ?isR*)
  :match  (and (converse  ?R1 ?R2)
               (relation  ?R1 ?D1 ?Rn1)
               (relation  ?R2 ?D2 ?Rn2)
               (neq ?Rn1 ?D2)
               (absent (?isR* ?Rn1 ?D2))
               (absent (?isR* ?D2 ?Rn1)))
  :assert (false)
  :why    "converse {?R1}/{?R2} ill-typed: range({?R1})={?Rn1} and domain({?R2})={?D2} are neither equal nor one a subtype of the other under {?isR*}.")
```

![The rule converse-illtyped-dom of std.algebra, as the engine draws it](diagrams/03-std.algebra.converse-illtyped-dom.svg)
<!-- /rule -->

#### `converse-illtyped-ran`

Refutes a pair where `?R1`'s first label and `?R2`'s second are unrelated.

<!-- rule std.algebra converse-illtyped-ran -->
```lisp
(rule converse-illtyped-ran (?R1 ?R2 ?isR*)
  :match  (and (converse  ?R1 ?R2)
               (relation  ?R1 ?D1 ?Rn1)
               (relation  ?R2 ?D2 ?Rn2)
               (neq ?D1 ?Rn2)
               (absent (?isR* ?D1 ?Rn2))
               (absent (?isR* ?Rn2 ?D1)))
  :assert (false)
  :why    "converse {?R1}/{?R2} ill-typed: domain({?R1})={?D1} and range({?R2})={?Rn2} are neither equal nor one a subtype of the other under {?isR*}.")
```

![The rule converse-illtyped-ran of std.algebra, as the engine draws it](diagrams/03-std.algebra.converse-illtyped-ran.svg)
<!-- /rule -->

### The Boolean layer

#### `meet`

An edge in both `?R1` and `?R2` is an edge of `?R3`.

<!-- rule std.algebra meet -->
```lisp
(rule meet (?R1 ?R2 ?R3)
  :match  (and (?R1 ?a ?b) (?R2 ?a ?b))
  :assert (?R3 ?a ?b)
  :why    "(meet {?R1} {?R2} {?R3}): {?R1}({?a},{?b})∧{?R2}({?a},{?b}) → {?R3}({?a},{?b}).")
```

![The rule meet of std.algebra, as the engine draws it](diagrams/03-std.algebra.meet.svg)
<!-- /rule -->

#### `difference`

An edge of `?R1` that `?R2` lacks is an edge of `?R3`; the `absent` is the closed-world reading.

<!-- rule std.algebra difference -->
```lisp
(rule difference (?R1 ?R2 ?R3)
  :match  (and (?R1 ?a ?b) (absent (?R2 ?a ?b)))
  :assert (?R3 ?a ?b)
  :why    "(difference {?R1} {?R2} {?R3}): {?R1}({?a},{?b})∧¬{?R2}({?a},{?b}) → {?R3}({?a},{?b}).")
```

![The rule difference of std.algebra, as the engine draws it](diagrams/03-std.algebra.difference.svg)
<!-- /rule -->

#### `derive-join`

`join` is not a rule but a fact the program writes; this rule turns it into the activators of the two copiers.

<!-- rule std.algebra derive-join -->
```lisp
(rule derive-join ()
  :match  (join ?R1 ?R2 ?R3)
  :assert (and (join-l ?R1 ?R2 ?R3) (join-r ?R1 ?R2 ?R3))
  :why    "(join {?R1} {?R2} {?R3}): fan out into the two per-operand copiers.")
```

![The rule derive-join of std.algebra, as the engine draws it](diagrams/03-std.algebra.derive-join.svg)
<!-- /rule -->

#### `join-l`

Copies the first operand's edges into the union.

<!-- rule std.algebra join-l -->
```lisp
(rule join-l (?R1 ?R2 ?R3)
  :match  (?R1 ?a ?b)
  :assert (?R3 ?a ?b)
  :why    "(join {?R1} {?R2} {?R3}): {?R1}({?a},{?b}) → {?R3}({?a},{?b}).")
```

![The rule join-l of std.algebra, as the engine draws it](diagrams/03-std.algebra.join-l.svg)
<!-- /rule -->

#### `join-r`

Copies the second operand's edges into the union.

<!-- rule std.algebra join-r -->
```lisp
(rule join-r (?R1 ?R2 ?R3)
  :match  (?R2 ?a ?b)
  :assert (?R3 ?a ?b)
  :why    "(join {?R1} {?R2} {?R3}): {?R2}({?a},{?b}) → {?R3}({?a},{?b}).")
```

![The rule join-r of std.algebra, as the engine draws it](diagrams/03-std.algebra.join-r.svg)
<!-- /rule -->

#### `empty`

A check: any edge of `?R` is a contradiction.

<!-- rule std.algebra empty -->
```lisp
(rule empty (?R)
  :match  (?R ?a ?b)
  :assert (false)
  :why    "(empty {?R}) violated: {?R}({?a},{?b}) exists.")
```

![The rule empty of std.algebra, as the engine draws it](diagrams/03-std.algebra.empty.svg)
<!-- /rule -->

#### `top`

Every pair of a member of `?Dom` and a member of `?Ran` is an edge.

<!-- rule std.algebra top -->
```lisp
(rule top (?R ?isa ?Dom ?Ran)
  :match  (and (?isa ?a ?Dom) (?isa ?b ?Ran))
  :assert (?R ?a ?b)
  :why    "(top {?R} {?isa} {?Dom} {?Ran}): {?a}∈{?Dom},{?b}∈{?Ran} → {?R}({?a},{?b}).")
```

![The rule top of std.algebra, as the engine draws it](diagrams/03-std.algebra.top.svg)
<!-- /rule -->

#### `complement`

Every such pair that `?R1` lacks is an edge of `?R2`.

<!-- rule std.algebra complement -->
```lisp
(rule complement (?R1 ?R2 ?isa ?Dom ?Ran)
  :match  (and (?isa ?a ?Dom) (?isa ?b ?Ran) (absent (?R1 ?a ?b)))
  :assert (?R2 ?a ?b)
  :why    "(complement {?R1} {?R2}): {?a}∈{?Dom},{?b}∈{?Ran},¬{?R1}({?a},{?b}) → {?R2}({?a},{?b}).")
```

![The rule complement of std.algebra, as the engine draws it](diagrams/03-std.algebra.complement.svg)
<!-- /rule -->

### The cardinality checks

#### `functional`

Two edges from one source to two distinct targets are a contradiction.

<!-- rule std.algebra functional -->
```lisp
(rule functional (?R)
  :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
  :assert (false)
  :why    "{?R} not functional: {?a} has both {?b} and {?c}.")
```

![The rule functional of std.algebra, as the engine draws it](diagrams/03-std.algebra.functional.svg)
<!-- /rule -->

#### `injective`

Two edges from two distinct sources to one target are a contradiction.

<!-- rule std.algebra injective -->
```lisp
(rule injective (?R)
  :match  (and (?R ?a ?c) (?R ?b ?c) (neq ?a ?b))
  :assert (false)
  :why    "{?R} not injective: {?c} reached from both {?a} and {?b}.")
```

![The rule injective of std.algebra, as the engine draws it](diagrams/03-std.algebra.injective.svg)
<!-- /rule -->

#### `bijective-properties`

Fans a program's `(bijective R)` out into the four one-argument markers.

<!-- rule std.algebra bijective-properties -->
```lisp
(rule bijective-properties ()
  :match  (bijective ?R)
  :assert (and (functional ?R) (injective ?R) (total ?R) (surjective ?R))
  :why    "{?R} is bijective ⟹ functional ∧ injective ∧ total ∧ surjective.")
```

![The rule bijective-properties of std.algebra, as the engine draws it](diagrams/03-std.algebra.bijective-properties.svg)
<!-- /rule -->

#### `total`

Refutes a state in which a member of the first label has every member of the second excluded; the `forall` is
`std.macro`'s.

<!-- rule std.algebra total -->
```lisp
(rule total (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?a ?A)
               (forall ?b (?isa ?b ?B) (not (?R ?a ?b))))
  :assert (false)
  :why    "{?R} not total at {?a}: every {?B} is excluded.")
```

![The rule total of std.algebra, as the engine draws it](diagrams/03-std.algebra.total.svg)
<!-- /rule -->

#### `surjective`

The dual: a member of the second label excluded for every member of the first.

<!-- rule std.algebra surjective -->
```lisp
(rule surjective (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?b ?B)
               (forall ?a (?isa ?a ?A) (not (?R ?a ?b))))
  :assert (false)
  :why    "{?R} not surjective at {?b}: every {?A} is excluded.")
```

![The rule surjective of std.algebra, as the engine draws it](diagrams/03-std.algebra.surjective.svg)
<!-- /rule -->

### The totality obligations

The two rules beside `total` and `surjective` that conclude `(open ?R)`. An obligation derives nothing and is not
run with the others: it is read once over a state that has stopped changing, and a match means the state owes.

#### `total-owed`

A member of the first label with no partner stated: the relation owes it one.

<!-- rule std.algebra total-owed -->
```lisp
(rule total-owed (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?a ?A)
               (absent (and (?isa ?b ?B) (?R ?a ?b))))
  :assert (open ?R)
  :why    "{?R} owes {?a} a {?B}.")
```

![The rule total-owed of std.algebra, as the engine draws it](diagrams/03-std.algebra.total-owed.svg)
<!-- /rule -->

#### `surjective-owed`

A member of the second label with no preimage stated: the relation owes it one.

<!-- rule std.algebra surjective-owed -->
```lisp
(rule surjective-owed (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?b ?B)
               (absent (and (?isa ?a ?A) (?R ?a ?b))))
  :assert (open ?R)
  :why    "{?R} owes {?b} a preimage in {?A}.")
```

![The rule surjective-owed of std.algebra, as the engine draws it](diagrams/03-std.algebra.surjective-owed.svg)
<!-- /rule -->

### The property checks

#### `irreflexive`

A loop is a contradiction.

<!-- rule std.algebra irreflexive -->
```lisp
(rule irreflexive (?R)
  :match  (?R ?a ?a)
  :assert (false)
  :why    "(irreflexive {?R}) violated: self-loop {?R}({?a},{?a}).")
```

![The rule irreflexive of std.algebra, as the engine draws it](diagrams/03-std.algebra.irreflexive.svg)
<!-- /rule -->

#### `antisymmetric`

Two distinct names related both ways are a contradiction.

<!-- rule std.algebra antisymmetric -->
```lisp
(rule antisymmetric (?R)
  :match  (and (?R ?a ?b) (?R ?b ?a) (neq ?a ?b))
  :assert (false)
  :why    "(antisymmetric {?R}) violated: {?R}({?a},{?b})∧{?R}({?b},{?a}), {?a}≠{?b}.")
```

![The rule antisymmetric of std.algebra, as the engine draws it](diagrams/03-std.algebra.antisymmetric.svg)
<!-- /rule -->

#### `asymmetric`

Two names related both ways are a contradiction, a loop included, since nothing keeps `?a` and `?b` apart.

<!-- rule std.algebra asymmetric -->
```lisp
(rule asymmetric (?R)
  :match  (and (?R ?a ?b) (?R ?b ?a))
  :assert (false)
  :why    "(asymmetric {?R}) violated: both {?R}({?a},{?b}) and {?R}({?b},{?a}).")
```

![The rule asymmetric of std.algebra, as the engine draws it](diagrams/03-std.algebra.asymmetric.svg)
<!-- /rule -->

#### `connex`

Two distinct members of `?Dom` related neither way are a contradiction.

<!-- rule std.algebra connex -->
```lisp
(rule connex (?R ?isa ?Dom)
  :match  (and (?isa ?a ?Dom) (?isa ?b ?Dom) (neq ?a ?b)
               (absent (?R ?a ?b)) (absent (?R ?b ?a)))
  :assert (false)
  :why    "(connex {?R}) violated: {?a},{?b}∈{?Dom} incomparable (neither {?R}({?a},{?b}) nor {?R}({?b},{?a})).")
```

![The rule connex of std.algebra, as the engine draws it](diagrams/03-std.algebra.connex.svg)
<!-- /rule -->

### The property closures

#### `difunctional`

Two sources that share a target share every target.

<!-- rule std.algebra difunctional -->
```lisp
(rule difunctional (?R)
  :match  (and (?R ?a ?b) (?R ?c ?b) (?R ?c ?d))
  :assert (?R ?a ?d)
  :why    "(difunctional {?R}): {?R}({?a},{?b})∧{?R}({?c},{?b})∧{?R}({?c},{?d}) → {?R}({?a},{?d}).")
```

![The rule difunctional of std.algebra, as the engine draws it](diagrams/03-std.algebra.difunctional.svg)
<!-- /rule -->

#### `symmetric`

Mirrors each edge.

<!-- rule std.algebra symmetric -->
```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}.")
```

![The rule symmetric of std.algebra, as the engine draws it](diagrams/03-std.algebra.symmetric.svg)
<!-- /rule -->

#### `symmetric-negative-setup`

Activates the negative mirror for every relation declared symmetric, which is why a program lists nothing for it.

<!-- rule std.algebra symmetric-negative-setup -->
```lisp
(rule symmetric-negative-setup ()
  :match  (symmetric ?rel)
  :assert (symmetric-negative ?rel)
  :why    "{?rel} symmetric ⟹ activate its negative mirror.")
```

![The rule symmetric-negative-setup of std.algebra, as the engine draws it](diagrams/03-std.algebra.symmetric-negative-setup.svg)
<!-- /rule -->

#### `symmetric-negative`

Mirrors each stored negation.

<!-- rule std.algebra symmetric-negative -->
```lisp
(rule symmetric-negative (?rel)
  :match  (not (?rel ?a ?b))
  :assert (not (?rel ?b ?a))
  :why    "{?rel} symmetric: ¬{?rel}({?a},{?b}) ⟹ ¬{?rel}({?b},{?a}).")
```

![The rule symmetric-negative of std.algebra, as the engine draws it](diagrams/03-std.algebra.symmetric-negative.svg)
<!-- /rule -->

#### `transitive`

Closes a chain of two edges; the `neq` guard is the one difference from `(compose R R R)`.

<!-- rule std.algebra transitive -->
```lisp
(rule transitive (?rel)
  :match  (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why    "{?rel} is transitive.")
```

![The rule transitive of std.algebra, as the engine draws it](diagrams/03-std.algebra.transitive.svg)
<!-- /rule -->

#### `includes`

Copies each edge of `?p` into `?q`.

<!-- rule std.algebra includes -->
```lisp
(rule includes (?p ?q)
  :match  (?p ?a ?b)
  :assert (?q ?a ?b)
  :why    "{?q} includes {?p}: from {?a} →{?p}→ {?b}, derive {?a} →{?q}→ {?b}.")
```

![The rule includes of std.algebra, as the engine draws it](diagrams/03-std.algebra.includes.svg)
<!-- /rule -->

### The equational lemmas

Schröder's law, opted into per triple, and the two laws that carry a composition and a union through a converse.

#### `compose-negative-s`

From an excluded composite and an edge of the first factor, excludes the edge of the second.

<!-- rule std.algebra compose-negative-s -->
```lisp
(rule compose-negative-s (?R ?S ?T)
  :match  (and (?R ?a ?b) (not (?T ?a ?c)))
  :assert (not (?S ?b ?c))
  :why    "Schröder ({?R};{?S}={?T} closed): {?R}({?a},{?b})∧¬{?T}({?a},{?c}) → ¬{?S}({?b},{?c}).")
```

![The rule compose-negative-s of std.algebra, as the engine draws it](diagrams/03-std.algebra.compose-negative-s.svg)
<!-- /rule -->

#### `compose-negative-r`

From an excluded composite and an edge of the second factor, excludes the edge of the first.

<!-- rule std.algebra compose-negative-r -->
```lisp
(rule compose-negative-r (?R ?S ?T)
  :match  (and (?S ?b ?c) (not (?T ?a ?c)))
  :assert (not (?R ?a ?b))
  :why    "Schröder ({?R};{?S}={?T} closed): {?S}({?b},{?c})∧¬{?T}({?a},{?c}) → ¬{?R}({?a},{?b}).")
```

![The rule compose-negative-r of std.algebra, as the engine draws it](diagrams/03-std.algebra.compose-negative-r.svg)
<!-- /rule -->

#### `compose-contravariant`

The converse of a composition is the composition of the converses, in the other order.

<!-- rule std.algebra compose-contravariant -->
```lisp
(rule compose-contravariant ()
  :match  (and (compose ?R ?S ?T)
               (converse ?R ?Rc) (converse ?S ?Sc) (converse ?T ?Tc))
  :assert (compose ?Sc ?Rc ?Tc)
  :why    "B7 contravariance: ({?R};{?S}={?T}) ⟹ ({?Sc};{?Rc}={?Tc}) (converses).")
```

![The rule compose-contravariant of std.algebra, as the engine draws it](diagrams/03-std.algebra.compose-contravariant.svg)
<!-- /rule -->

#### `join-converse`

The converse of a union is the union of the converses.

<!-- rule std.algebra join-converse -->
```lisp
(rule join-converse ()
  :match  (and (join ?R ?S ?U)
               (converse ?R ?Rc) (converse ?S ?Sc) (converse ?U ?Uc))
  :assert (join ?Rc ?Sc ?Uc)
  :why    "B8: ({?R}∪{?S}={?U}) ⟹ ({?Rc}∪{?Sc}={?Uc}) (converses).")
```

![The rule join-converse of std.algebra, as the engine draws it](diagrams/03-std.algebra.join-converse.svg)
<!-- /rule -->

## `std.typing`

The module imports `std.algebra` and lists `converse` and the two checks of its declarations.

#### `type-hierarchy-converse`

For every converse pair stated, activates both checks with the relation `(type-hierarchy …)` names.

<!-- rule std.typing type-hierarchy-converse -->
```lisp
(rule type-hierarchy-converse ()
  :match  (and (converse ?R1 ?R2) (type-hierarchy ?isR*))
  :assert (and (converse-illtyped-dom ?R1 ?R2 ?isR*)
               (converse-illtyped-ran ?R1 ?R2 ?isR*))
  :why    "typecheck converse {?R1}/{?R2} under hierarchy {?isR*}.")
```

![The rule type-hierarchy-converse of std.typing, as the engine draws it](diagrams/03-std.typing.type-hierarchy-converse.svg)
<!-- /rule -->

#### `derive-reflexive`

Turns a program's `(reflexive R)` into the activators of the two rules below.

<!-- rule std.typing derive-reflexive -->
```lisp
(rule derive-reflexive ()
  :match  (reflexive ?R)
  :assert (and (reflexive-dom ?R) (reflexive-cod ?R))
  :why    "(reflexive {?R}) ⟹ close {?R} reflexively on both argument positions.")
```

![The rule derive-reflexive of std.typing, as the engine draws it](diagrams/03-std.typing.derive-reflexive.svg)
<!-- /rule -->

#### `reflexive-dom`

Every name an edge leaves gets a loop.

<!-- rule std.typing reflexive-dom -->
```lisp
(rule reflexive-dom (?R)
  :match  (?R ?a ?b)
  :assert (?R ?a ?a)
  :why    "{?R} reflexive: ({?R} {?a} {?b}) ⟹ ({?R} {?a} {?a}).")
```

![The rule reflexive-dom of std.typing, as the engine draws it](diagrams/03-std.typing.reflexive-dom.svg)
<!-- /rule -->

#### `reflexive-cod`

Every name an edge reaches gets a loop.

<!-- rule std.typing reflexive-cod -->
```lisp
(rule reflexive-cod (?R)
  :match  (?R ?a ?b)
  :assert (?R ?b ?b)
  :why    "{?R} reflexive: ({?R} {?a} {?b}) ⟹ ({?R} {?b} {?b}).")
```

![The rule reflexive-cod of std.typing, as the engine draws it](diagrams/03-std.typing.reflexive-cod.svg)
<!-- /rule -->

## `std.closure`

The module's one rule reads `std.algebra`'s one-argument markers, which a `(bijective R)` or the program states.

#### `infer-closure`

A relation marked functional and total is closed: no fact of it is a hypothesis.

<!-- rule std.closure infer-closure -->
```lisp
(rule infer-closure ()
  :match  (and (functional ?R) (total ?R))
  :assert (__closed__ ?R)
  :why    "{?R} is functional + total → its extension is fixed (operational closure)")
```

![The rule infer-closure of std.closure, as the engine draws it](diagrams/03-std.closure.infer-closure.svg)
<!-- /rule -->
