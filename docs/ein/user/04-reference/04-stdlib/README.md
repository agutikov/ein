# The standard library (`std.*`)

The standard library is a set of modules written in Ein: rules, macros and tables of facts that a program brings in
with one `(import …)`. A rule family that is not about one problem is a module here, and a problem's own rules stay
in its own file ([Q16](../../../../decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md)).

Ein has no numbers ([Q-M1e2.24](../../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): `7`
is a name, and `1` and `01` are two names. Arithmetic is a theory of the library, `std.math.*`, written over the
digit names and computed by saturation; Boolean functions are `std.bool.*`
([Q-M1f.338](../../../../decisions/stdlib/Q-M1f.338-the-calculator-is-std-math-and-boolean-functions.md)). An order
is a relation a program declares, never a form of the language
([Q25](../../../../decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md)).

## The pages

Each family of modules has a page that says how a program **uses** it: what it imports and lists, every relation,
macro and entry name it writes, and what each produces, shown by small programs with what `ein` prints for them.
How a family **works**, its rules one by one, each as Ein and as the engine's drawing of it, is kept apart: at the
end of the page where the family's rules are few, on the page after it otherwise
([Q-M1f.375](../../../../decisions/docs/Q-M1f.375-the-standard-librarys-reference-shows-usage.md)). A block of
output is what `ein` printed for the program above it; a line `…` in one stands for rows left out.

| page | modules | what is on it |
|---|---|---|
| [1. Pattern macros](01-macros.md) | `std.macro` | `forall` and `unknown`, and what each expands to |
| [2. The relation algebra](02-relation-algebra.md) | `std.algebra`, `std.typing`, `std.closure` | operators, property checks, closures and the totality obligations a program declares of a relation |
| [3. The relation algebra's rules](03-relation-algebra-rules.md) | the same three | how it works: every rule, drawn |
| [4. Bijections and elimination](04-bijections.md) | `std.bijection`, `std.elim` | what a relation declared a bijection licenses: negative completion, elimination, the argument check; its rules at the end |
| [5. Slots](05-slots.md) | `std.slots` | one co-location relation whose classes are slots |
| [6. The slots' rules](06-slots-rules.md) | `std.slots` | how it works: every rule, drawn |
| [7. The calculator](07-calculator.md) | `std.math.*` | what every arithmetic module shares: requests and results, no search, flags, the tagged calls, and which fault is flagged by whom |
| [8. Natural numbers](08-natural-numbers.md) | `std.math.digits`, `std.math.arith`, `std.math.decimal`, `std.math.decimal-arith` | the decimal numeral and school arithmetic on it: `add`, `sub`, `cmp`, `mul`, `div`, `let` |
| [9. The natural numbers' rules](09-natural-numbers-rules.md) | `std.math.arith`, `std.math.decimal`, `std.math.decimal-arith` | how it works: the rules that are decimal's own |
| [10. The rules every base shares](10-positional-rules.md) | `std.math.positional` | how it works: the walks, the requests and the faults, written once for decimal and binary |
| [11. Integers, rationals, gcd](11-integers-rationals-gcd.md) | `std.math.int`, `std.math.rat`, `std.math.gcd` | `(int P N)`, `(rat Z D)`, and the greatest common divisor |
| [12. The integers' rules](12-integers-rules.md) | `std.math.int` | how it works |
| [13. The rationals' and the divisor's rules](13-rationals-and-gcd-rules.md) | `std.math.rat`, `std.math.gcd` | how it works |
| [14. Binary numbers and conversion](14-binary.md) | `std.math.bits`, `std.math.binary`, `std.math.binary-arith`, `std.math.convert` | the binary numeral, its arithmetic, and decimal to binary and back |
| [15. Binary's and the conversion's rules](15-binary-rules.md) | `std.math.binary`, `std.math.binary-arith`, `std.math.convert` | how it works |
| [16. Nine digits in one fact](16-nine-digit-numbers.md) | `std.math.num9` | a fixed-width number, and overflow as a fact |
| [17. The nine-digit numbers' rules](17-nine-digit-numbers-rules.md) | `std.math.num9` | how it works |
| [18. Boolean functions and circuits](18-boolean-functions.md) | `std.bool.gates`, `std.bool.props`, `std.bool.circuit`, `std.bool.inputs` | the sixteen functions of two bits, what is known of them, circuits, and inputs as hypotheses |
| [19. The Boolean modules' rules](19-boolean-rules.md) | the same four | how it works |

The concepts the relation-algebra modules build on are
[Properties and operators](../../03-syntax-and-semantics/07-properties-and-operators.md); the import form itself is
[the syntax reference](../02-syntax.md) and [Macros and imports](../../03-syntax-and-semantics/11-macros.md).

## Importing a module

A module's name is its path in the library, a dot a directory: `std.algebra` is `algebra.ein`,
`std.math.decimal-arith` is `math/decimal-arith.ein`. The library ships inside the `ein` binary; a checkout's
`stdlib/` directory, or the directory `$EIN_STDLIB` names, takes its place where present
([`01-install.md`](../../01-install.md) says how to check which one a binary reads).

**An import brings its module whole**, with the modules it imports, one copy a program
([Q-M1f.247](../../../../decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)). Every name a module
declares has one full name, `std.math.decimal-arith.add`
([Q-M1f.231](../../../../decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)). The three
shapes of an import differ only in how the file spells those names.

**`:symbols (…)` lists the short names the file writes**, and is the shape most programs use:

```lisp
(import std.math.decimal-arith :symbols (add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add (n z 2) (n z 3))

(query :goal (sum-of ?x ?y ?s))
```

<!-- output -->
```text
  query             (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (n z 5) · ?x = (n z 2) · ?y = (n z 3)
  as :expect        (model (sum-of (n z 2) (n z 3) (n z 5)))
```

**`:as a` writes every name of the module behind a prefix**, `a.add`:

```lisp
(import std.math.decimal-arith :as a)
(import std.math.decimal :as d)
(import std.math.arith :as ar)

(a.add (d.n ar.z 2) (d.n ar.z 3))

(query :goal (a.sum-of ?x ?y ?s))
```

<!-- output -->
```text
  query             (a.sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (d.n ar.z 5) · ?x = (d.n ar.z 2) · ?y = (d.n ar.z 3)
  as :expect        (model (a.sum-of (d.n ar.z 2) (d.n ar.z 3) (d.n ar.z 5)))
```

**A bare import writes full names**:

```lisp
(import std.math.decimal-arith)
(import std.math.decimal)
(import std.math.arith)

(std.math.decimal-arith.add (std.math.decimal.n std.math.arith.z 2)
                            (std.math.decimal.n std.math.arith.z 3))

(query :goal (std.math.decimal-arith.sum-of ?x ?y ?s))
```

<!-- output -->
```text
  query             (std.math.decimal-arith.sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (std.math.decimal.n std.math.arith.z 5) · ?x = (std.math.decimal.n std.math.arith.z 2) · ?y = (std.math.decimal.n std.math.arith.z 3)
```

The answer does not depend on which of the three a file uses: the sum is the same fact, printed in the spelling the
file gave its names. A name the program does not write is printed by its full name, which is how a state shows a
step of a module's own, `std.math.decimal-arith.strip`.

### What to list

- **A file lists every short name it writes, from the module that declares it**, and nothing is re-exported: the
  program above writes a numeral, so it lists `n` from `std.math.decimal` and `z` from `std.math.arith`, though
  `std.math.decimal-arith` imports both.
- **A generic rule is activated by a fact headed by the rule's name**, `(symmetric next-to)`, and the rule's name is
  the module's: list it.
- **A relation a module reads and does not declare is no module's**, and a program writes it as it is: `bijective`,
  `join`, `bijection-hierarchy`, `typecheck-hierarchy`, `slot-partition`, `slot-spatial`, `type-hierarchy`,
  `reflexive`, and `std.elim`'s positional markers.
- **A module's entry rule may be listed** and brings nothing a whole import does not
  ([Q-M1f.240](../../../../decisions/imports/Q-M1f.240-a-module-keeps-its-entry-rule-which-a-program.md)).
- **No module reserves its names**
  ([Q-M1f.319](../../../../decisions/imports/Q-M1f.319-no-module-reserves-its-names.md)). A program may state a
  fact of a module's relation: a `(sum-of X Y S)` written by hand is an answer nobody computed, and the theory
  computes with it.
- **No import brings a module's `(query …)`, `(solve …)` or `(config …)`**: those are a program's.

**A short name the file writes and lists from no import is its own**, whatever a module calls the same
([Q-M1f.236](../../../../decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)). This
program imports `std.algebra` and lists nothing, so its `symmetric` is a relation of its own that no rule reads, and
nothing is mirrored:

```lisp
(import std.algebra)

(relation next-to T T :why "{?3} is next to {?4}")

(symmetric next-to)
(next-to a b)

(solve :no-search)
(query :goal (next-to ?x ?y))
```

<!-- output -->
```text
  query             (next-to ?x ?y)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = a · ?y = b
  as :expect        (model (next-to a b))
```

With `(import std.algebra :symbols (symmetric))` in its place, the same fact activates the module's rule:

<!-- output -->
```text
  query             (next-to ?x ?y)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = a · ?y = b
                      ?x = b · ?y = a
  as :expect        (model (next-to a b) (next-to b a))
```

**One short name bound to two full names in one file is refused**
([Q-M1f.234](../../../../decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)): `std.elim`
and `std.bijection` both declare `domain-elimination`, and a file lists it from one of them.

```lisp
(import std.elim :symbols (domain-elimination))
(import std.bijection :symbols (domain-elimination))
```

<!-- output -->
```text
kb load error: import-clash.ein:2:9: 'domain-elimination' is bound to two full names in the program, std.elim.domain-elimination and std.bijection.domain-elimination — one short name names one declaration in a file: list it from one module, or write the other by its full name
```

## The modules

| module | what it is | entry | an import brings | page |
|---|---|---|---|---|
| `std.macro` | `forall` and `unknown` | none | nothing more | [1](01-macros.md) |
| `std.algebra` | the relation algebra: operators, property checks and closures, the totality obligations | none | `std.macro` | [2](02-relation-algebra.md), rules [3](03-relation-algebra-rules.md#stdalgebra) |
| `std.typing` | a hierarchy of labels named once; the reflexive closure | none | `std.algebra` | [2](02-relation-algebra.md#using-stdtyping--a-hierarchy-named-once), rules [3](03-relation-algebra-rules.md#stdtyping) |
| `std.closure` | functional and total, so closed: `(__closed__ R)` | none | `std.algebra` | [2](02-relation-algebra.md#using-stdclosure--closing-a-relation-that-is-a-total-function), rules [3](03-relation-algebra-rules.md#stdclosure) |
| `std.bijection` | negative completion and elimination for a relation declared `(bijective R)` | none | `std.algebra`, `std.macro` | [4](04-bijections.md) |
| `std.elim` | the same elimination, with the labels named in the activator | none | `std.macro` | [4](04-bijections.md#using-stdelim--elimination-with-positional-markers) |
| `std.slots` | one co-location relation whose classes are slots | none | `std.macro` | [5](05-slots.md), rules [6](06-slots-rules.md) |
| `std.math.digits` | the decimal digit tables, facts and no rule | none | nothing more | [8](08-natural-numbers.md#using-stdmathdigits--the-digit-tables) |
| `std.math.arith` | `z`, zero; the division by zero's flag and its rule | `arith` | nothing more | [8](08-natural-numbers.md#using-stdmatharith--zero-and-the-division-by-zero), rules [9](09-natural-numbers-rules.md#stdmatharith) |
| `std.math.decimal` | the decimal numeral `(n PREFIX DIGIT)` and its check | `decimal` | `arith`, `digits` | [8](08-natural-numbers.md#using-stdmathdecimal--the-decimal-numeral), rules [9](09-natural-numbers-rules.md#stdmathdecimal) |
| `std.math.decimal-arith` | natural numbers of any length: `add`, `sub`, `cmp`, `mul`, `div`, `let` | `decimal-arith` | the three above, `positional` | [8](08-natural-numbers.md#using-stdmathdecimal-arith--school-arithmetic-on-natural-numbers), rules [9](09-natural-numbers-rules.md#stdmathdecimal-arith) |
| `std.math.positional` | the rules every positional base shares, and the macro that connects a base | the macro | `arith` | [10](10-positional-rules.md) |
| `std.math.int` | integers `(int P N)` | `int-arith` | `decimal-arith` and below | [11](11-integers-rationals-gcd.md#using-stdmathint--integers), rules [12](12-integers-rules.md) |
| `std.math.rat` | rationals `(rat Z D)`, reduced on request | `rat-arith` | `int`, `gcd`, `decimal-arith` and below | [11](11-integers-rationals-gcd.md#using-stdmathrat--rational-numbers), rules [13](13-rationals-and-gcd-rules.md#stdmathrat) |
| `std.math.gcd` | `gcd` and `lcm` of natural numbers | `gcd` | `decimal-arith` and below | [11](11-integers-rationals-gcd.md#using-stdmathgcd--greatest-common-divisor-and-least-common-multiple), rules [13](13-rationals-and-gcd-rules.md#stdmathgcd) |
| `std.math.bits` | binary's tables, facts and no rule | none | nothing more | [14](14-binary.md#using-stdmathbits--the-bit-tables) |
| `std.math.binary` | the binary numeral `(b2 PREFIX BIT)` and its check | `binary` | `arith`, `bits` | [14](14-binary.md#using-stdmathbinary--the-binary-numeral), rules [15](15-binary-rules.md#stdmathbinary) |
| `std.math.binary-arith` | binary arithmetic, every name `bin-` | `binary-arith` | `bits`, `arith`, `binary`, `positional` | [14](14-binary.md#using-stdmathbinary-arith--binary-arithmetic), rules [15](15-binary-rules.md#stdmathbinary-arith) |
| `std.math.convert` | decimal to binary and back | `convert` | both bases | [14](14-binary.md#using-stdmathconvert--decimal-to-binary-and-back), rules [15](15-binary-rules.md#stdmathconvert) |
| `std.math.num9` | nine digits in one fact, a fixed width | `arith9` | `digits`, `decimal-arith` and below | [16](16-nine-digit-numbers.md), rules [17](17-nine-digit-numbers-rules.md) |
| `std.bool.gates` | the sixteen Boolean functions of two inputs | `gates` | nothing more | [18](18-boolean-functions.md#using-stdboolgates--the-sixteen-functions), rules [19](19-boolean-rules.md#stdboolgates) |
| `std.bool.props` | thirty relations known of the sixteen, derived by rules | `props` | `gates` | [18](18-boolean-functions.md#using-stdboolprops--what-is-known-of-the-sixteen), rules [19](19-boolean-rules.md#stdboolprops) |
| `std.bool.circuit` | a circuit as facts, simulated by saturation | `circuit` | `gates` | [18](18-boolean-functions.md#using-stdboolcircuit--a-circuit-simulated-by-saturation), rules [19](19-boolean-rules.md#stdboolcircuit) |
| `std.bool.inputs` | a circuit's inputs as hypotheses, and required values | `inputs` | `circuit`, `gates`, `std.algebra` | [18](18-boolean-functions.md#using-stdboolinputs--inputs-as-hypotheses), rules [19](19-boolean-rules.md#stdboolinputs) |

An **entry** is a module's closing rule: a rule with an empty `:match` that closes the held extent of every relation of the module
and names every rule of it, so that nothing of the module is left for the search to guess. The relation-algebra
modules have none, since what they close is the program's to say.

## How a rule is shown

On the pages of rules, each rule is its Ein, as the module writes it, and its drawing, as
`ein render rule --name NAME FILE` writes it for the module's file: the `match` cluster above, the `assert` cluster
below, a variable a diamond, an atom a box, each pattern an octagon with no name in it whose arrow `1` is at its
relation and whose arrows `2`, `3`, … are at its arguments, an `absent` a dashed cluster
([Rendering](../12-rendering.md#rules--ein-render-rules-and-ein-render-rule) is the legend). A rule whose drawing has
more nodes than a page can show is drawn with `--fold`, each pattern of three arrows that nothing points at one
arrow labelled with its relation and each of two a loop on its argument, where that drawing is small enough, and its
image's text says so: a closing rule, which names every relation and rule of its module, each in a pattern of two
arrows, is drawn so where its module is small. Past that the rule is shown as Ein alone. The text and the drawing of a rule on these pages are generated from the module's
file and the engine, and are not edited by hand: the drawing's source is the file `NN-MODULE.RULE.dot` under
[`diagrams/`](diagrams/), `NN` the page's number, and the page shows the picture drawn from it.

A rule with parameters, `(rule transitive (?rel) …)`, is **activated** by a fact headed by its name,
`(transitive linked)`, once for each such fact. A rule with none, `(rule gcd-step () …)`, is active wherever its
module is loaded.

## See also

- [`stdlib/README.md`](../../../../../stdlib/README.md) — the library's own page, beside its modules.
- [Properties and operators](../../03-syntax-and-semantics/07-properties-and-operators.md) — what the relation
  algebra's properties mean; [Obligations](../../03-syntax-and-semantics/08-obligations.md) — `(open R)`;
  [Macros and imports](../../03-syntax-and-semantics/11-macros.md).
- [The syntax reference](../02-syntax.md) — the `import` form; [Reserved names](../03-reserved-names.md) —
  `__closed__`, `open` and the kernel relations these modules build on;
  [Diagnostics](../07-diagnostics.md) — what a load refuses.
- [Word problems](../../02-guide/08-word-problems.md) — programs over the calculator.
- [`examples/arith/`](../../../../../examples/arith/) — one program per operation of the calculator;
  [`tests/stdlib/`](../../../../../tests/stdlib/) — one program per rule or family, each stating what it must and
  must not derive.
