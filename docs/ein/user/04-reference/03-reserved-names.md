# Reserved names

The names a program may *write* but not *redefine*: the language gives each a fixed meaning. A name is reserved
**if and only if** this page lists it, and nothing else is special. Ein has **no types**, `is-a` and `T`
being an ordinary relation and an ordinary atom
([Q23](../../../decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md)), and **no symmetry**,
`symmetric` being a property a rule gives meaning to
([Q6](../../../decisions/answer/Q6-symmetry-is-the-programs-not-the-engines.md)). The names the engine makes up for
its own records, which no program writes, are
[`07-reserved-engine-strings.md`](../../ein.rs/07-reserved-engine-strings.md).

**The reserved names are the surface between the engine's kernel and a theory written in Ein**
([Q-M1f.397](../../../decisions/docs/Q-M1f.397-the-reserved-words-are-mathematics.md)). They are where
the kernel acts of its own accord: on a declaration and the marker of closure, and on the word a rule states to it
under `:ein`. `(false)` and `(not P)` are shaped by the grammar and judged by no part of the kernel: a state that
holds `(false)` is dead by `std.core`'s rule, which every program gets
([`04-stdlib/21-core.md`](04-stdlib/21-core.md)), and one that holds `P` beside `(not P)` by
`std.contradiction`'s, where the program imports it
([Q-M1f.557](../../../decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). What a fact
of any other relation means is what rules make of it, the program's own or the standard library's. So whether a
fact inside a fact holds is a theory's to say, of `(not P)` as of `(claims Carol …)` or any other context
([a fact inside a fact](../03-syntax-and-semantics/03-facts.md#a-fact-inside-a-fact)).

## Quick reference — from intent to word

*"I want to express X: what do I write?"* With this table, [`02-syntax.md`](02-syntax.md) and its § 3 on patterns, a
program can be written without reading anything else.

| I want to… | write | see |
|---|---|---|
| declare a relation, with a label for each argument slot | `(relation R A B :why "…")` | [§ Top-level declarators](#top-level-declarators--the-closed-set); [what the signature means](02-syntax.md#what-the-signature-means--labels-for-rules-a-shape-for-the-engine) |
| state a fact | `(R a b)`, any head that is no declarator | [§ Top-level declarators](#top-level-declarators--the-closed-set) |
| deny a fact | `(not (R a b))`, a stored negation | [§ Rule-body primitives](#rule-body-primitives) |
| declare a saturation rule | `(rule N (?p…) :match … :assert … :why "…")` | [§ Top-level declarators](#top-level-declarators--the-closed-set) |
| declare a hypothesis rule | `(hrule N (?p…) :match … :assert … :why "…")` | [§ Top-level declarators](#top-level-declarators--the-closed-set) |
| ask about the answer | `(query :goal … [:goal-text …])` | [§ Top-level declarators](#top-level-declarators--the-closed-set) |
| set a solver flag | `(config :flag v)` | [`06-config.md`](06-config.md) |
| import a standard-library module | `(import std.X :symbols (…))` | [`02-syntax.md` § Import](02-syntax.md#import) |
| join or choose premises in a `:match` | `(and …)` / `(or …)` | [§ Rule-body primitives](#rule-body-primitives) |
| match where a pattern is **absent**, negation as failure | `(absent P)` | [§ Rule-body primitives](#rule-body-primitives) |
| match a **stored** negation | `(not P)` in a `:match` | [§ Rule-body primitives](#rule-body-primitives) |
| require two objects to differ, or be one | `(neq ?a ?b)` / `(eq ?a ?b)` | [§ Computed relations](#computed-relations--eq-and-neq) |
| say this state is contradictory | `:assert (false)`, or `:ein inconsistent` in the place of the `:assert` ([`02-syntax.md`](02-syntax.md#conclusion-forms-in-assert)) | [§ Rule-body primitives](#rule-body-primitives) |
| say this state is unfinished | `:assert (open)` / `(open ?R)` | [§ The verdict](#the-verdict--open) |
| say *P is undecided* | `(unknown P)`, imported from `std.macro` | [§ Pattern macros](#pattern-macros-forall-and-unknown--not-reserved) |
| say *for every b with G, B holds* | `(forall ?b G B)`, imported from `std.macro` | [§ Pattern macros](#pattern-macros-forall-and-unknown--not-reserved) |
| make a relation symmetric, transitive, … | `(symmetric R)` and the rule that reads it | [§ Not reserved](#not-reserved) |
| keep the facts of a relation from being guessed | `(__closed__ R)` | [§ Hypothesis and query control](#hypothesis-and-query-control) |
| choose what the blind enumerator guesses | `(solve :hypothesis-relations (…))` / `:no-hypothesis (…)` | [§ Hypothesis and query control](#hypothesis-and-query-control) |
| judge root alone | `(solve :no-search)` | [§ Hypothesis and query control](#hypothesis-and-query-control) |
| save a state, or read one saved | `ein solve FILE --dump-answer DIR`, whose state files are `(program …)`, `(module NAME …)` and `(commit F …)` | [§ A saved file's three heads](#a-saved-files-three-heads--program-module-commit) |

## Top-level declarators — the closed set

A program is a **flat sequence of forms**, each classified by its **head**: a head in this table is a declarator, and
**any other head is a fact**. The set is **closed**. `rule`, `hrule`, `query`, `solve`, `config`, `trace`, `macro`
and `import` are reserved words, never a name, so a malformed declarator, `(query)` with no clause, is a parse error;
`relation` is a position word, a name wherever a name goes, so that a rule can match `(relation ?R ?A ?B)`, and its
malformed form is a parse error too.

| name | form | meaning |
|---|---|---|
| `relation` | `(relation R [A B …] :why "…")` | declare a relation and its signature, a name and **zero or more** slot labels, each an ordinary atom; the declaration is also stored as the fact `(relation R A B …)` and the membership fact `(relation R)` ([what the signature means](02-syntax.md#what-the-signature-means--labels-for-rules-a-shape-for-the-engine)) |
| `rule` | `(rule N (?p…) :match … :assert … :why "…")` | a saturation rule |
| `hrule` | `(hrule N (?p…) :match … :assert … :why "…")` | a hypothesis rule: it proposes what may be guessed, and never fires during saturation |
| `query` | `(query :goal … …)` | a question about the answer |
| `solve` | `(solve [:hrules …] [:hypothesis-relations …] [:no-hypothesis …] [:no-search] [:expect <claim>])` | the problem's lattice, at most one per file, and its expected region |
| `config` | `(config [:flag v]*)` | the solver's flags |
| `macro` | `(macro N (?p…) BODY :why "…")` | a pattern alias: a call `(N a…)` in a rule's clause or a query's `:goal` is replaced by BODY at load. A fact headed by `N`, or holding such a form, is refused at load, and so is a relation declared as `N` |
| `import` | `(import M [:as A \| :symbols (S…)])` | bring module `M`, a dotted name such as `std.macro`, into the program, whole ([`02-syntax.md` § Import](02-syntax.md#import)) |
| `trace` | `(trace <step>*)` | a derivation log the engine writes: read back, and no part of the program |

**Anything else is a fact**: `not`, or a generic `(NAME args*)`. Where it came from is its **provenance annotation**:
`:rule`/`:using` a derivation, `:source` a given condition, neither a background assumption
([`02-syntax.md` § Where a fact came from](02-syntax.md#where-a-fact-came-from--its-provenance-annotation)). A head like
`(facts …)` is a plain fact of the relation `facts`.

**Declared names are the program's, with one guard**: a `(rule …)`, `(hrule …)`, `(relation …)` or `(macro …)` may
not *bind* a name that shadows the language's own vocabulary, and an import's `:as` takes the same guard. The
vocabulary is the connectives `and`, `or` and `absent`, the verdict `open`, the computed relations `eq` and `neq`,
`relation`, and the dunder `__closed__`, which a program uses and never binds
([Q-M1f.7](../../../decisions/language/Q-M1f.7-the-dunder-is-never-bound-and-the-writeback.md)). The engine's three writeback rule names,
`<forced-positive>`, `<monotonic-unconditional>` and `<lookahead-dies-immediately>`, are refused wherever a program
writes them. The guard is about *binding* a name: a fact may still carry a reserved head. **`false` and `not` are the
standard library's names**, which a relation, a rule, an hrule and an alias may bind and a macro may not: `(false)`
is the fact `std.core`'s rule reads, and `(not X)` the fact `std.contradiction`'s rules read beside `X`
([Q-M1f.557](../../../decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)), so
`(relation not Claim)` declares the negation of a claim, and `std.core` may declare either. Each is a position word,
a name wherever a name goes, `(clash-1 not)` among them. The grammar still gives each its shape where it heads a
form: `(false)` takes no argument, and `(not X)` one fact. A macro named after either would make every `(false)` or
`(not X)` its call, `std.core`'s among them, so it is refused as the vocabulary is.
`unknown` and `forall` are deliberately *not* reserved: they are macros of the `std.macro` module.

**The guard holds through every import route.** A reserved name declared by a module is refused whether the module is
imported bare, under `:as`, or with `:symbols`, as it is written directly.

### Two sets: what a declarator may bind, and what is never a name

Two sets answer two questions, and neither contains the other:

| | the question it answers | its members | how it fails |
|---|---|---|---|
| the vocabulary | what may a declarator **not bind**? | `__closed__` `absent` `and` `eq` `neq` `open` `or` `relation` | a **load** error, at the declaration |
| the reserved words ([`01-grammar.md` § 1](01-grammar.md#1-lexical-grammar)) | what may never be a **name**? | `and` `or` `neq` `rule` `hrule` `query` `solve` `config` `trace` `macro` `import` `open` | a **parse** error, wherever a name goes |

Four names are in both, `and`, `neq`, `open` and `or`. **`relation` is the one that keeps them apart**: it must read as a name, because `(relation ?R
?A ?B)` is a pattern rules match, so it is a position word, a token of its own that the grammar takes wherever a name
goes. `open` is a reserved word, never a name: the grammar takes it in the one place it is written
([§ The verdict](#the-verdict--open)).

### A bare `_` and `?_` — reserved, and written nowhere

Neither set holds them, because neither is a name: `_` and `?_` are tokens of their own that the grammar takes
nowhere, so a program writes neither anywhere, where a reserved name may be written and not bound. Each is refused at
the parse, with the note *_ is reserved and names nothing: where any value will do, write a variable used once*
([Q-M1e2.22](../../../decisions/language/Q-M1e2.22-a-bare-is-reserved-and-refused.md)); the note's *value*, as in
every line `ein` prints, is an object in an argument's place. `_` anywhere else in a word
is a name character, so `_a_in_b_` and `__x` are names and `?_x` a variable.

## The verdict — `open`

Reserved, and **written only as a rule's or an hrule's whole `:assert`**
([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)): `open` is one of the
words that are never a name, so no fact, pattern, argument, activator, goal, claim, list, trace or macro body writes it,
each refused at the parse at the `open`. It is `(false)`'s dual: where `(false)` says *this state is dead*, `open`
says *this state is unfinished*. A contradiction survives any extension, while openness exists to be destroyed by one,
so an `open` conclusion is **never stored**: it is a tally on the state, read once the state has reached its fixpoint.

| form | arguments | meaning |
|---|---|---|
| `(open)` | 0 | this state owes something; the rule's `:why` is the report. Counted, with no slot to name |
| `(open ?R)` | 1 | the held extent of `?R` is **incomplete**. `?R` is a parameter of the rule, bound by its activator; the witness domain and the slot to fill are read off the rule's own `(absent …)` |

A rule asserting it is an **obligation**
([Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)), and it never
fires during saturation, since it derives nothing. Five things are refused rather than guessed, three at the parse and
two at load, each load line starting where the `(open …)` is written:

| refused | why |
|---|---|
| `(open …)` in a `:match` (at the parse) | the verdict is a conclusion about the state. The third-state test for a *fact* is `(unknown P)` |
| two arguments or more (at the parse) | a guard restated in the head could disagree with the rule's own |
| an `:assert` that writes `open` beside anything else (at the parse) | such a rule would both fire during saturation and be read after it |
| `(open ?R)` where `?R` is not a parameter (at load) | a relation written as a variable is bound by the activator, never by a premise |
| a projection that does not resolve (at load) | exactly one `(absent …)` must hold a positive `?R` premise, and exactly one such premise must bind a variable the guard does not already bind. None, two, or a ground body is refused |

### What reads it

Obligations are read in **one pass over the state, once it has reached its fixpoint**, never during saturation, and
in the order they are written, which keeps the report deterministic. **Firing is being undischarged**: the obligation
is stated once, in its guard, which matches while the witness is missing and stops matching once it has arrived, so
nothing asks a second question that could disagree with the `absent`.

The tally is read only where the state is **consistent**: a state that holds `(false)` is dead, and its debts are not
consulted. A state that is alive and owes is **owing**, and is no member of the answer R; one that owes nothing is
**discharged** ([`../03-syntax-and-semantics/08-obligations.md`](../03-syntax-and-semantics/08-obligations.md)).

Three surfaces report what a state owes, and none of them is a fact:

| surface | what it carries |
|---|---|
| the read-out, and `--json-summary` | each owing node a run reports, with its count and each instance's rendered `:why`: the `owing` block of an empty R, and `owes` and `owed` beside root and each node of the owing frontier ([`08-read-out.md`](08-read-out.md)) |
| `--events` | one `owe` line per undischarged instance: `rule`, `activator`, `relation`, `bindings` and the rendered `:why` |
| `--trace` | an *Owing* section, the `:why`s as a list, written where a run reports an owing node |

`(open ?R)`'s relation is what lets a debt be counted per relation; a bare `(open)` counts and names no slot.
[`tests/region/04_owing.ein`](../../../../tests/region/04_owing.ein) is a program whose root owes one instance.

`:expect` has no word for it and needs none: every claim lists *facts*, and an `open` conclusion is never a fact. An
owing node is stated in a region claim as `(owing (model …))` ([`02-syntax.md` § Solve](02-syntax.md#solve)).

### The standard library's four

`std.algebra` declares `total-owed` and `surjective-owed`, the obligation duals of the totality checks beside them,
activated by `bijective-setup`. `std.slots` declares `slot-owed-room` and `slot-owed-fill`, activated by
`slot-partition-setup`. All four assert `(open ?R)` and nothing else, and the *direction* is nowhere in the head: it
falls out of which slot the rule's own `absent` leaves free
([`total-owed` and `surjective-owed`](04-stdlib/02-relation-algebra.md#obligations-total-owed-and-surjective-owed), [`std.slots`](04-stdlib/05-slots.md)).

## Rule-body primitives

| name | arguments | meaning |
|---|---|---|
| `not` | 1 | the negation of one fact: `(not X)` is a stored fact whose argument is the fact denied, which may itself be a stored negation. In a `:match` it matches a stored negation, and beside `X` it is a contradiction where a rule refutes the pair, `std.contradiction`'s where the program imports it |
| `false` | 0 | `(false)`: the state is contradictory, by `std.core`'s rule, which matches it and states `:ein inconsistent`. It takes no argument, as a fact, an argument of a fact, a premise and a conclusion: `(false k)` is refused ([Q-M1f.123](../../../decisions/language/Q-M1f.123-false-with-no-argument-is-an-object.md)) |
| `and` | 1 or more | conjunction: every premise holds, or every conclusion is derived |
| `or` | 1 or more | disjunction, in a `:match` only: a top-level `(or …)` is one rule per disjunct, and an `(or …)` anywhere else in a `:match` is refused at load |
| `absent` | 1 | negation as failure on a pattern: a question asked of the state, never a stored fact, so no fact, argument or conclusion is headed by it, and a claim's class alone lists `(absent (q a))`, what a goal's `absent` says a state does not hold ([Q-M1f.122](../../../decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-member-of-a-claims.md)). It is asked once, at the closure boundary, against the positive fixpoint ([`02-syntax.md` § NAF evaluation timing](02-syntax.md#naf-evaluation-timing--the-closureworld-boundary)) |

## Computed relations — `eq` and `neq`

A computed relation's truth is computed from its arguments, not looked up in the state. The two are **relations**
([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)): computed
and never stored, a free side bound by the guard, `eq` to the other side's value and `neq` over the program's names,
and a conclusion of either checked, a false one `(false)`
([`02-syntax.md` § Premise order around a guard](02-syntax.md#premise-order-around-a-guard)).

| name | arguments | meaning |
|---|---|---|
| `eq` | 2 | `(eq ?a ?b)` holds when the two objects are one; a free side is bound to the other |
| `neq` | 2 | `(neq ?a ?b)` holds when the two objects differ; a free side ranges over the program's names but the other |

Neither heads an argument: `(q (eq k j))`, as `(q (neq k j))`, is refused, in a fact, a conclusion and a pattern
([Q-M1f.124](../../../decisions/language/Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md)).

## Pattern macros (`forall` and `unknown`) — not reserved

`forall` and `unknown` are ordinary `(macro …)` declarations of the `std.macro` module,
[`stdlib/macro.ein`](../../../../stdlib/macro.ein), expanded at load: no part of the language, and a program may
declare its own under either name. A program that wants them imports them:
`(import std.macro :symbols (forall unknown))`, or `(import std.macro)` and `:as m` for full or prefixed names.

| macro | form | expands to |
|---|---|---|
| `unknown` | `(unknown P)` | `(and (absent P) (absent (not P)))`: P is neither stated nor denied |
| `forall` | `(forall ?b G B)` | `(absent (and G (absent B)))`: for every `?b` with `G`, `B` holds |

Both expand to nested `absent`s, so they mean what `absent` means: the $\forall$ is $\lnot\exists\lnot$ over the
state at the closure boundary, and a nested `absent` is the one guard that can turn from false to true as saturation goes on, so it is asked
again at every later quiescence.

**What a call expands to is held to the grammar**
([Q-M1f.80](../../../decisions/language/Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md)): `unknown`'s
`(not P)` is the stored negation of one fact, so `(unknown (and (p ?x) (q ?x)))`, whose `(not (and …))` negates no
fact, is refused at load, its line placed at the call in the program's file. The same holds of a program's own
macros: a body or a call that writes `eq`, `absent`, `not`, `false` or `or` in another shape than the grammar gives it
where a rule writes it is refused where the macro is called, and a macro no rule calls loads
([`02-syntax.md` § 3](02-syntax.md#the-primitives-and-where-a-malformed-one-is-caught)). **An object is an atom, or a
fact a state could hold**
([Q-M1f.113](../../../decisions/language/Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md)): `eq`, `neq`,
`absent`, `and`, `or` and `false` with an argument head no argument of a fact, written or left by a macro's
expansion, and `not` does, a stored negation being a fact.

## Hypothesis and query control

| name | form | meaning |
|---|---|---|
| `__closed__` | `(__closed__ R)` | no fact of R is proposed as a hypothesis: the held extent of R is fixed, and R is a **closed relation**. Written by a program, or derived by `std.closure` or a module's closing rule; the engine infers it for no relation. Used as a relation or as an argument, never bound, and never a guessable atom, one the blind enumerator proposes about. No hrule proposes a fact of a closed relation and no obligation owes one ([below](#the-mechanisms-and-each-pair)) |
| `:hypothesis-relations` | `(solve … :hypothesis-relations (R₁ R₂ …))` | the blind enumerator and the obligations rung guess facts of the listed relations only; the hrule rung reads no list. Every name is a relation of the program, or the program is refused at load, as a `:no-hypothesis` name and an `:hrules` hrule and activator relation are |
| `:no-hypothesis` | `(solve … :no-hypothesis (R₁ R₂ …))` | the exclusion dual: never guess a fact of the listed relations, while saturation rules still derive their facts; read by the blind enumerator and the obligations rung, never the hrule rung |
| `:no-search` | `(solve … :no-search)` | a flag, no value: the lattice is root alone, nothing is proposed whatever else the program declares, so R is $\lbrace \emptyset \rbrace$ where root is alive and discharged and empty otherwise. The way to say *judge root alone* |
| `:expect` on a query | `(query … :expect (model …) \| (or (model …) …) \| (false))` | what the answer should be: the query's own claim, checked by the engine, and `ein solve` exits 1 when it is false ([`02-syntax.md` § Query](02-syntax.md#query)) |
| `:expect` on `(solve …)` | `(solve … :expect (region …) \| (false))` | the expected region: R's states, `(merged …)` states, `(owing …)`, written only here. A top-level `(region …)`, `(merged …)` or `(owing …)` is a parse error; `model` is a name ([`02-syntax.md` § Solve](02-syntax.md#solve)) |

**A `(query …)` takes four keywords**, `:goal`, `:goal-text`, `:over` and `:expect`, and any other is refused at the
parse: the dunder above is a fact, not a keyword. `(solve …)` takes `:hrules`, `:hypothesis-relations`,
`:no-hypothesis`, `:no-search` and `:expect`, and on a query each of the first four is a parse error whose note names
`(solve …)`. A keyword that loaded and did nothing would be a silent no-op, which is not survivable once a keyword can
carry a claim.

### The mechanisms, and each pair

What a commitment may be about is steered by declarations, and the blind enumerator runs where none of the generating
ones does. The generation ladder: the hrules, if the program declares any, and then no other rung runs; otherwise,
where an obligation is **activated** at root, the obligations, which propose what would pay the debts root owes, so a
root that owes nothing under an activated obligation proposes nothing; otherwise, or where the obligations rung
declines ([Q-M1e2.10](../../../decisions/hypotheses/Q-M1e2.10-the-obligations-rung-declines-where-its-domain.md)),
the blind enumerator. An obligation rule nothing activates chooses no rung: every import brings its module whole, so a
program that imports `std.algebra` for `symmetric` loads its two obligation rules, and the facts of its own
relations that nothing closes are the blind enumerator's to guess. The hypothesis set H is built once, at root; the obligations rung proposes there too what
would pay a debt only a commitment brings
([Q-M1e.11](../../../decisions/hypotheses/Q-M1e.11-h-holds-what-a-debt-only-a-commitment-brings.md)). What each
mechanism means is [`../03-syntax-and-semantics/09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md).

| | mechanism | what it does | read by |
|---|---|---|---|
| H | `(hrule …)`, activated by `(solve :hrules …)` (one with no parameter always) | generates, rung 1, alone where declared | itself |
| O | an obligation rule, `:assert (open ?R)`, activated by a fact such as `(total-owed R isa)` | two roles: a state that owes is not in R, whatever generated it; and, with no hrule and one activated at root, rung 2 proposes facts of the owed relation | the discharge test; rung 2 |
| B | the blind enumerator | generates, rung 3: no hrule and no obligation activated at root, or the obligations rung declined | — |
| HR | `(solve :hypothesis-relations …)` | a whitelist, *speculate only these*; `()` names none | rungs 2 and 3; the read-out's `outside` row |
| NH | `(solve :no-hypothesis …)` | a blacklist, *never these* | rungs 2 and 3; the `outside` row |
| C | `(__closed__ R)`, stated or derived | keeps the facts of R out of rungs 2 and 3, and so makes an `absent` over them closed-world | rungs 2 and 3; the `outside` row |
| NS | `(solve :no-search)` | no rung runs: H is empty, the lattice root alone; an obligation judges root and proposes nothing | the ladder, before rung 1 |

No list and no closure filters an hrule's candidates. **The pairs**, each refused or noted at load on every command,
read on root's fixpoint
([Q-M1f.245](../../../decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)):

| pair | verdict |
|---|---|
| H + O | no conflict: H generates, O judges |
| H + HR | HR filters nothing, a `note:`; a **conflict** where an active hrule proposes a fact of a relation HR leaves out |
| H + NH | NH filters nothing, a `note:`; a **conflict** where an active hrule proposes a fact of a relation NH lists |
| H + C | a **conflict** where an active hrule proposes a fact of a closed relation; closing another relation stands |
| O + HR | with no hrule, a **conflict** where a relation whose facts are owed is outside HR |
| O + NH | with no hrule, a **conflict** where a relation whose facts are owed is in NH: an obligation filtered away is no obligation |
| O + C | a **conflict** where a relation whose facts are owed is closed, whatever generates ([Q-M1f.244](../../../decisions/hypotheses/Q-M1f.244-an-obligated-relations-held-extent-is-not-closed.md)) |
| HR + NH | a **conflict** where one relation is in both; with no hrule, NH naming a relation HR leaves out is **redundant**, a `note:` |
| HR + C | a **conflict** where a listed relation is closed |
| NH + C | with no hrule, an overlap is **redundant**, a `note:`; never a conflict |

An obligation is read as declared, whether or not root owes it
([Q-M1f.243](../../../decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md)), by this
check and by the ladder alike; a derived closure and a derived activator are read where they are derived, at root's
fixpoint.

**`:no-search` beside each**: with O it is silent, the obligation judging root alone, which is its use, and with C
silent too; with H one `note:`, *the hrule proposes no candidate*, and with HR or NH one, *the list filters no
candidate*, which replace the notes above. It relaxes no conflict: H, O and the lists are read as declared, so every
row of the table stands beside it
([Q-M1f.254](../../../decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md)).

## A saved file's three heads — `program`, `module`, `commit`

A **saved file**, the state `ein solve FILE --dump-answer DIR` writes, is its program as written and the choices made,
in three forms that stand only at the top of a file
([`10-answer-dump.md`](10-answer-dump.md#a-state-file-is-its-program-and-the-choices-made)):

| name | form | meaning |
|---|---|---|
| `program` | `(program <the program's text>)` | the program file's bytes as the load read them, read again as the file they were. A file that opens with `(program` is a saved file, and it holds one |
| `module` | `(module NAME <the module's text>)` | a module the load read, each once, sorted by name; every import of the file is read from these and never from the disk ([Q-M1f.401](../../../decisions/read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md)) |
| `commit` | `(commit F …)` | the choices made: candidates of the program's H, each as the read-out prints it, in H's order, committed after root's fixpoint; at most one, last, and none in a file of root ([Q-M1f.400](../../../decisions/read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md)) |

Each is a **position word**, as `relation` and `model` are: a name wherever a name goes, so a relation, a rule or an
atom may be called `program`, `module` or `commit`, and a nested fact `(note (commit a))` loads. At the top of a file
none is a fact's relation: a top-level `(commit a)` is a parse error, and a program that holds one of the three forms
is refused at load ([`07-diagnostics.md` § 2.29](07-diagnostics.md#229-a-saved-file-is-its-program-its-modules-and-one-commitment-of-h)).

## Not reserved

- **`model`**, the head of an `:expect` value. It is read in that one position and nowhere else, so a relation, rule
  or atom called `model` is unaffected: a position word, a name wherever a name goes.
- **`closed`**, bare: free for a program or the standard library to define. The engine reads `__closed__`.
- **`is-a` and `T`**: an ordinary relation and an ordinary atom. Ein has no types: what a program calls a type is an
  atom its membership facts point at or a declaration labels a slot with, and it means what the program's rules,
  and the library's it imports, make of those facts. `T` is the conventional label for a slot that needs none that
  says something, and any name behaves the same. The slot may be dropped, `(relation R)` being a legal bare
  declaration, but that changes what is guessed: only facts of a relation with a non-empty signature are guessed, so
  `T` is what to write for such a slot of a relation whose facts may be guessed
  ([`02-syntax.md` § Relation declarator](02-syntax.md#relation-declarator)). Because nothing is reserved here, **two
  conventions coexist**: [`zebra.ein`](../../../../examples/zebra.ein) writes membership as `type` and `instance`
  relations, and [`zebra2.ein`](../../../../examples/zebra2.ein) as one `is-a`, and both are declared relations. A
  membership fact *about a relation* is legal for the same reason: `(is-a co-located EquivalenceRelation)` loads and
  is stored, and only a rule gives it meaning.
- **`symmetric`**, `transitive`, `functional` and the rest: property tags the program states, each given meaning by a
  rule that reads it, the standard library's or the program's
  ([`../03-syntax-and-semantics/07-properties-and-operators.md`](../03-syntax-and-semantics/07-properties-and-operators.md)).

The same names in graph terms are [`../03-syntax-and-semantics/04-ontology.md`](../03-syntax-and-semantics/04-ontology.md).
