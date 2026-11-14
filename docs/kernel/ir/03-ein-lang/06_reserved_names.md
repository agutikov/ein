# Reserved names — the ein surface language

The **authoritative** list of names an ein puzzle author may *write* but
not *redefine*: the kernel gives them fixed meaning. This is the
surface-language view (what you type in a `.ein` file). For the
engine-internal vocabulary (carrier heads, protocol enums) see
[`../../inference/reserved_engine_strings.md`](../../inference/reserved_engine_strings.md).

After the S1.7.23/.24 kernel-purity pass, the reserved set is small: the
kernel imposes **no type system** (`is-a` / `T` are ordinary
relation/atom — S1.7.23)
and **no symmetric semantics** (`symmetric` is a plain user property tag —
S1.7.24).
A name is reserved **iff** it appears in this table or the engine-strings
doc — nothing else is special.

## Author quick reference — intent → atom

Start here. *"I want to express X — what do I write?"* The authoritative
per-atom detail is in the sections below; this card is the reverse index.
With it + [`01_grammar.md`](01_grammar.md) + [`02_patterns.md`](02_patterns.md)
you can author a puzzle without reading engine source.

| I want to…                                   | write                                   | see |
|----------------------------------------------|-----------------------------------------|-----|
| declare a relation + its arg types           | `(relation R A B)`                      | §declarators; [what the signature means](01_grammar.md#what-the-signature-means--userspace-types-kernel-structure) |
| state a fact                                 | `(R a b)` *(any non-declarator head)*   | §else→fact |
| negate a fact                                | `(not (R a b))` *(stored octagon)*      | §⊥ primitives |
| declare an inference (saturation) rule       | `(rule N (?p…) :match … :assert …)`     | §declarators |
| declare a hypothesis generator               | `(hrule N (?p…) :match … :assert …)`    | §declarators |
| ask the engine something                     | `(query :goal … [:goal-text …])`        | §declarators |
| set a solver knob                            | `(config :flag v)`                      | §declarators |
| import a stdlib module                       | `(import std.X :symbols (…))`           | §declarators |
| conjoin / disjoin premises in `:match`       | `(and …)` / `(or …)`                    | §⊥ primitives |
| match only when a pattern is **absent** (NAF)| `(absent P)`                            | §⊥ primitives; [semantics](../../inference/absent_semantics.md) |
| match a **stored** negative                  | `(not P)` in `:match`                   | §⊥ primitives |
| require two slots differ / are equal         | `(neq ?a ?b)` / `(eq ?a ?b)`            | §predicates |
| declare this branch contradictory            | `:assert (false)`                       | §⊥ primitives |
| declare this state unfinished                | `:assert (open)` / `(open ?R)`          | §the verdict atom |
| say "P is undecided"                          | `(unknown P)` *(import `std.macro`)*       | §macro sugar |
| say "∀ b s.t. G, B holds"                    | `(forall ?b G B)` *(import `std.macro`)*| §macro sugar |
| tag a relation symmetric / transitive / …    | `(symmetric R)` + the rule consuming it | §not-reserved |
| freeze a relation (no guessing on it)        | `(__closed__ R)` *(usually auto)*       | §hypothesis control |
| read a fact nested in what a file writes     | `(written T)` in a `:match`             | §`written` |
| scope / exclude the blind enumerator         | `(solve … :hypothesis-relations (…))` / `:no-hypothesis (…)` | §hypothesis control |

## Top-level declarators — the closed classifier set (P1.7c)

A program is a **flat sequence of forms** (P1.7c — the `(ontology …)` /
`(facts …)` / `(reasoning …)` / `(rules …)` block wrappers were removed in
S1.7c.4).
Each top-level form is classified by its **head**: a head in the table
below is a declarator (`trace` is the engine-emitted sibling); **any other
head is a fact** — "detect facts by *not* being reserved" (the author's
design note). This set is **closed**: the parser keys on it (`rule` / `hrule` / `query` /
`solve` / `config` / `trace` / `macro` / `import` are reserved words, never a name, so a malformed
declarator — e.g. `(query)` with no clause — is a *parse* error; `relation` is a position word, a
name wherever a name goes so rules can pattern-match `(relation ?R ?A ?B)`, and its malformed
form is a parse error too since M1f S1f.5.5's swap, where the loader rejected it until then,
`relation` having been a plain `SYMBOL`),
and the loader
([`ein_ir::from_ir`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs)) routes by the
same set.

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `relation` | `(relation R [A B …])` | declare a relation-type node + its arg-type signature (name + **≥ 0** type atoms; also stores the arity-1 membership fact `(relation R)`. What the signature *means* — userspace types vs kernel structure — is [`01_grammar.md` §relation declarator](01_grammar.md#what-the-signature-means--userspace-types-kernel-structure), its one definitive home) | `kb.from_ir`; `entities.KERNEL_META_RELATIONS` |
| `rule` | `(rule N (?p…) :match … :assert …)` | declare a saturation rewrite rule | `kb.from_ir` |
| `hrule` | `(hrule N (?p…) :match … :assert …)` | declare a hypothesis-generation rule (drives `hypgen`, never fired by the saturator) | `kb.from_ir`; `hypgen` |
| `query` | `(query :goal … …)` | what to ask the engine | `kb.from_ir` (`store.Query`) |
| `solve` | `(solve [:hrules …] [:hypothesis-relations …] [:no-hypothesis …] [:no-search] [:expect <claim>])` | the problem's lattice, at most one per file, and its expected region (M1e2 S1e2.1a.4) | `kb.from_ir` (`ein_core::Solve`); `hypgen`, `hrule`; `ein-ir/claim.rs`, `ein-infer/claim.rs` |
| `config` | `(config [:flag v]*)` | solver-level knobs | `kb.from_ir`; `inference.config.SolverConfig` |
| `macro` | `(macro N (?p…) BODY)` | declare a load-time AST-rewrite alias; a rule clause's `(N a…)` invocation, and since M1f S1f.7.5 a query's `:goal`'s, expands to BODY before compilation (P1.8 S1.5.9); a fact headed by `N`, or holding such a form, is refused at load since M1f S1f.7.12, and so is a relation declared as `N` | `kb.from_ir` (`_ingest_macros`); `ir.macros.expand_macros` |
| `import` | `(import M [:as A \| :symbols (S…)])` | pull in a library module `M` (a dotted logical name, e.g. `std.macro`); qualified-by-default, or aliased/flat-selective (P1.8 S1.8.A1–A2) | `kb.from_ir` (grammar A2; resolve A3) |
| `trace` | `(trace <event>*)` | **engine-emitted** derivation log — parsed by [`ein-render/trace/ast.rs`](../../../../ein.rs/crates/ein-render/src/trace/ast.rs), ignored by `kb.from_ir`; a *sibling*, not part of the declarator-vs-fact dichotomy | `trace/` |

**Else → fact.** A top-level form whose head is none of the above is a
fact: `=`, `not`, or a generic `(NAME args*)`. Where it came from is its
**provenance annotation** — `:rule`/`:using` → an engine derivation,
`:source` → a given condition, neither → a background assumption. That is
the only origin a fact has, and there is no way to override it. A
former-wrapper head like `(facts …)` therefore parses as a plain fact.

**Declared names are user-space**, with one guard (`_reserved_names`,
P1.8 S1.8.A1 D3): a `(rule …)` / `(hrule …)` / `(relation …)` / `(macro …)`
may not *bind* a name that shadows reserved kernel vocabulary — the
structural primitives (`absent` / `false`), the **verdict atom** `open`, the
computed predicates (`eq` / `neq`), `relation`, or, since M1f S1f.5.5's
T1f.5.5.11, the kernel's dunders `__closed__` and `__symmetric__`, which a
program uses and never binds
([Q-M1f.7](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders)
(b)); an import's `:as` takes the same guard since then; and, since M1f
S1f.6.38, `written`, the relation the loader states what a file writes in
([§ `written`](#written--what-a-file-writes-stated-by-the-loader-m1f-s1f638)). The engine's three
writeback rule names are refused wherever a program writes them
([`reserved_engine_strings.md`](../../inference/reserved_engine_strings.md#synthetic-writeback-rule-names)). The thirteen reserved words
(`not` / `and` / `or` / `neq` / the declarators but `relation` / `open`, the last since M1f S1f.5.23) can't be written as a declared
name at all (parse error; *`SYMBOL`-excluded* until M1f S1f.5.5's swap). The guard is about *binding* a name; a **fact** may
still carry a reserved head (a stored `(not X)` octagon). `unknown` / `forall`
are deliberately *not* reserved — they migrated into the `std.macro` module
([`stdlib/macro.ein`](../../../../stdlib/macro.ein)). The macro that *was*
spelled `open` is `unknown` since 2026-08-24, which is what freed the word for
the section below.

**And the guard holds through every import route** — which since M1e S1e.2.1
is a checked claim rather than a stated one. `(import M)` and `(import M :as A)`
prefix every name the module defines, and a reserved name must survive that
prefixing *unrenamed* so the loader still sees it. Between M1d S1d.2.3 and
S1e.2.1 it did not: the resolver filtered against a second, hand-maintained
copy of the reserved list which had eight names where the kernel's had nine,
and a module declaring `open` under any of the four declarators was silently
renamed to `M.open` and loaded with **exit 0** — while the same declaration
written directly, or imported flat via `:symbols (open)`, was refused. `absent`
was in both copies and behaved correctly, which is what made it a drift rather
than a design. There is one list now
([`ein_core::RESERVED`](../../../../ein.rs/crates/ein-core/src/terms.rs)); the
thirty-two cells of *four declarators × {`open`, `absent`} × four routes* are
`ein-ir`'s `reserved_names_are_reserved_through_every_import_route` (forty-eight
since M1f T1f.5.5.11 added `__closed__`; since S1f.5.23 the flat route's
`:symbols (open)` is refused at the importing program's own parse, `open`
being no name), and the
four routes are pinned as fixtures in
[`examples/broken/load/`](../../../../examples/broken/load/) —
`reserved_open_direct`, `_symbols`, `_qualified`, `_aliased`.

### Two sets, and the one that is not this page's — M1e S1e.4.2

The list above is **`ein_core::RESERVED`**, and until M1e S1e.4.2 (`SE-L2`) the
lexer had a different set under the same name. Both pages describing them were
individually accurate and a reader of both concluded the lexer's eleven had
grown to twelve when `open` joined this one. It did not, and it must not. (The
lexer's set is twelve since M1e2 S1e2.1a.4, with the `solve` declarator.)

| | the question it answers | membership | how it fails |
|---|---|---|---|
| `ein_core::RESERVED` — **this page** | what may a declarator **bind**? | `__closed__` `__symmetric__` `absent` `and` `eq` `false` `neq` `not` `open` `or` `relation` `written` (the two dunders since M1f T1f.5.5.11, `written` since M1f S1f.6.38) | a **load** error, at the declaration |
| `ein_ir::grammar::RESERVED_WORDS` — [`00_ebnf.md`](00_ebnf.md) (`ein-ir`'s `LEXER_KEYWORDS` until M1f S1f.5.5's swap, the printer's `ein_core::text::LEXER_KEYWORDS` since) | what may never be a **name**? | `not` `and` `or` `neq` `rule` `hrule` `query` `solve` `config` `trace` `macro` `import` `open` (`open` since M1f S1f.5.23) | a **parse** error, wherever a name goes |

Five names are in both since M1f S1f.5.23, four until then, and neither set
contains the other. **`relation` is the one that decides it**: it must read as
a name, because `(relation ?R ?A ?B)` is a pattern rules match, so it is a
position word, a token of its own that `ein.y` takes wherever a name goes.
**`open` decided it too until S1f.5.23**, read as a name because `(open ?R)`
was a fact head the loader routed; since then `ein.y`'s `Assert` takes the
token `'open'` itself, the one place it is written, and it is never a name
(the user's ruling on Q-M1f.66, [§ the verdict
atom](#the-verdict-atom--open-m1d-p1d2-s1d23-read-since-s1d24)).
`grammar::tests::the_reserved_words_are_the_printers` holds the thirteen and
that no position word is one `NAME`;
`lex::tests::the_lexer_keywords_are_twelve_and_are_not_ein_cores_nine` held the
twelve, the four-name intersection, and that both lexed as `SYMBOL`s, until the
swap. They were eleven until M1e2 S1e2.1a.4 added `solve`.

### A bare `_` and `?_` — reserved, and written nowhere (M1f S1f.5.1)

Neither set above holds them, because neither is a name: `_` and `?_` are
tokens of their own (`ein.l`'s `UNDERSCORE` and `VARUNDERSCORE`) that no
production takes, so a program may write neither anywhere, where a reserved
name may be written and not bound. Each is refused at the parse, with the note
*`_ is reserved and names nothing: where any value will do, write a variable
used once`* (the user's ruling of 2026-11-06, [Q-M1e2.22](../../../../plans/m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)). `_`
anywhere else in a word is a name character, so `_a_in_b_` and `__x` are names
and `?_x` a variable. Until M1f S1f.5.5's swap `_` was the argument wildcard
of a rule's pattern ([`01_grammar.md`'s `WILDCARD` row](01_grammar.md#1-lexical-rules)).

## The verdict atom — `open` (M1d P1d.2 S1d.2.3, read since S1d.2.4)

Reserved, but **not** a rule-body primitive: it appears only as an `:assert`
conclusion, the compiler never meets it in a `:match`, and no detector reads
it. **It is written nowhere else** since M1f
[S1f.5.23](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)
(the user's ruling of 2026-11-10 on
[Q-M1f.66](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f66--may-open-be-written-as-a-value),
*"Refuse it"*): `open` is one of the words that are never a name, so no fact,
pattern, value, activator, goal, claim, list, trace or macro body writes it,
each refused at the parse at the `open`. Until then it was a name, so
`(p (open x))` loaded (the close review's finding 3), a relation parameter
could be bound to it and read the relation `open`, and as a value it was a name
of the program the blind enumerator proposed facts about (W70 to W72). It is `(false)`'s dual — where `(false)` says *this branch is dead*,
`open` says *this state is unfinished* — and the asymmetry between them is
that a contradiction survives any extension while openness exists to be
destroyed by one, so an `open` conclusion is **never stored**. It is a tally
on the search-lattice node, read once per quiescent KB *after* the fixpoint.

| form | arity | meaning |
|------|-------|---------|
| `(open)` | 0 | this state owes something; the rule's `:why` is the report. Countable, with no slot to name |
| `(open ?R)` | 1 | the extent of `?R` is **incomplete**. `?R` is a rule parameter (a relation head comes from the activator, never from a premise), and everything else — the witness domain, the slot to fill — is projected out of the rule's own `(absent …)` |

A rule asserting it is an **obligation** and is kept out of the saturation
agenda entirely (`Program::obligations`), for the reason it derives nothing.
Five things are refused rather than guessed, three at the parse and two at
load, each load line starting with where the `(open …)` is written (since M1f
S1f.5.23; each of the five was refused at load until then, ending `at None`):

| refused | why |
|---|---|
| `(open …)` in a `:match` (the parse) | the atom is a conclusion about the KB. The third-state probe for a *fact* is `(unknown P)` |
| arity ≥ 2 (the parse) | the superseded `forall`-dual triple `(open ?b G B)` restated the guard in the head, where it could disagree with it |
| an `:assert` mixing `open` with anything else (the parse) | such a rule would belong to the agenda and the post-fixpoint pass at once |
| `(open ?R)` where `?R` is not a parameter (at load) | a variable relation head is bound by the activator; the compiler cannot resolve one from a premise |
| a projection that does not resolve (at load) | exactly one `(absent …)` must hold a positive `?R` premise, and exactly one such premise must bind a variable the guard does not already bind. None, two, or a ground body is refused |

### What reads it — the obligation pass (S1d.2.4)

The rules live in `Program::obligations`, which neither the saturator nor
`hypgen` walks. What walks them is
[`ein_infer::obligations::tally`](../../../../ein.rs/crates/ein-infer/src/obligations.rs):
**one pass over the quiescent KB, once the fixpoint is reached** — not a
priority band inside the loop, because a band orders *selection* within the
walk and openness has to be read *after* it. The report order among
obligation rules is their load order, which is what makes the outstanding
list deterministic; `:priority` ordered it first until M1f S1f.5.15 removed
the keyword.

**Firing *is* being undischarged.** The obligation is stated once, in the
guard: the rule matches while the witness is missing and stops matching once
it has arrived. So the pass asks no second question, and there is no `∃b: G ∧
B` restatement that could disagree with the `absent` — which is the whole
reason the `forall`-dual triple went.

The tally is read only where the KB is **consistent**. The three states are
checked in one order — `(false)` first, then the count — so a node carrying a
contradiction never has its debts consulted, and the pass does not run there.

Three surfaces report it, and none of them is the fact store:

| surface | what it carries |
|---|---|
| [`--events`](../../inference/events.md) | one `owe` line per undischarged instance per quiescent KB: `rule`, `activator`, `relation`, `bindings`, the rendered `:why` |
| `--json-summary` | an `owes` block — `root` and one entry per reported model, each with `total`, `by_relation` and the instances |
| `--trace` | an **Outstanding obligations** section, the `:why`s as a list, rendered only when the state owes something |

`(open ?R)`'s per-relation attribution is what makes `by_relation` possible;
a bare `(open)` contributes to the count and names no slot.

**And since S1d.2.6 it decides a word.** A state that is `consistent ∧
complete` by the generator's test and still owes a witness is **`Open`** — the
fourth verdict word, reported as *`Open — owes n (rel: n, …)`* with `k = 0`,
where it read `Solution` before.
[`tests/region/04_owing.ein`](../../../../tests/region/04_owing.ein)
is that state with a number attached (`tests/stdlib/algebra/23_total_owed.ein`
was until M1f S1f.6.34, which refuses the `:no-hypothesis` that kept its
obligation from proposing, and both state `(solve :no-search)` since S1f.6.36, which
is how a program keeps it from proposing; it was one of the twelve corpus entries the word
moved). The distinction it draws is *no model* against *not yet
a model*, so `false` outranks it and a discharged model outranks it; it exits
**0** like the other three. It is **scoped** — only a program that *states* an
obligation can reach it, which is why the 92 corpus entries that declare none
report exactly the words they did before P1d.2
([`defined_behaviour.md` §5](../../defined_behaviour.md),
[the census](../../../history/m1d_satisfiability/openness_census.md)).

`:expect` did **not** grow a word for it, and could not: all three of its forms
are assertions about *facts*, and an `open` conclusion is by construction never
a fact. An open state's `(model …)` claim is checked against the facts it
reached, and every one of the twelve still holds unchanged.

### The stdlib's four

`std.algebra` ships `total-owed` and `surjective-owed` — the obligation duals
of the totality scans beside them, fanned out by `bijective-setup`. `std.slots`
ships `slot-owed-room` and `slot-owed-fill`, fanned out by
`slot-partition-setup`. All four assert `(open ?R)` and nothing else, and the
*direction* is nowhere in the head: it falls out of which slot the rule's own
`absent` leaves free.

## `written` — what a file writes, stated by the loader (M1f S1f.6.38)

Reserved since M1f
[S1f.6.38](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.38_a_number_is_well_formed_from_the_start.md)
(the user's ruling of 2026-11-14 on
[Q-M1f.272](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f272--how-a-representation-module-sees-a-numeral-right-away),
E1), and a kernel relation as `relation` is: the loader states
`(written T)` for every fact `T` nested at any depth in a fact a file writes,
a program's, a local module's or a stdlib module's (for a stored negation
`(not P)`, the facts nested in P's arguments), and for the facts a rule's or an
hrule's conclusion nests where they hold no variable
([`defined_behaviour.md` § 4.30](../../defined_behaviour.md#430-written-is-the-loaders-what-a-file-writes-stated-at-load--m1f-s1f638)).

| | |
|---|---|
| form | `(written T)`, one argument, a fact |
| who writes it | the loader alone. A fact of it, or its negation, written in a file, and a conclusion of it, are refused at load; no declarator binds the name (`ein_core::RESERVED`) |
| who reads it | a rule's `:match` or a goal, `(written (n ?p ?a))`: a representation module's check of its own terms, `std.math.decimal`'s of a numeral, wherever the numeral is written |
| the search | never a hypothesis target, no relation of the program's registry, and decided at root, so nothing that reads it is stale |
| its provenance | the loader's step `#written` (`ein_core::terms::WRITTEN_STEP`, one of [`reserved_engine_strings.md`](../../inference/reserved_engine_strings.md#names-the-parser-and-the-loader-make-up)'s made-up names), from the fact it was read in, so a core names the fact as written; from a rule's text, from no fact |

It shows where `relation`'s mirror facts show, `ein saturate --dump`, a KB
view and a `.einb`, and a `--dump-answer` state file writes none of it, the
loader stating it again when the file is read.

## Rule-body / ⊥ primitives (kept M1 kernel vocabulary)

Declared once in [`ein-core/terms.rs`](../../../../ein.rs/crates/ein-core/src/terms.rs)
(`primitives.STRUCTURAL`); the deep behaviour lives at the *engine site*.

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `not` | 1 | propositional negation; `(not X)` is a stored octagon fact whose arg is the negated proposition | matcher (`match_.rs`) + contradiction detector (`contradiction.rs`) |
| `false` | 0 | direct ⊥ — `(false)` asserts the firing rule reached a contradiction. It takes no argument, as a fact, a value, a premise and a conclusion: `(false k)` is refused since M1f S1f.7.6 (until then a written one was stored with its argument and killed root) | contradiction detector |
| `and` | 2+ | conjunction; flattened into sibling premises of one plan | compiler (`compile.rs`) |
| `or` | 2+ | disjunction; a **top-level** `(or …)` in a `:match` is lowered to one rule per disjunct at load time | loader (`kb.from_ir._match_disjuncts`) |
| `absent` | 1 | negation-as-failure on a sub-pattern (`AbsentGuard`) — a fork-local *query*, never a stored atom, so no fact, value or conclusion is headed by it: a written or concluded `(absent k)` is refused since M1f S1f.7.7 (until then it stored a fact no premise can read), and a claim's class alone lists `(absent (q a))`, what a goal's `absent` says a state does not hold; lifted out of the match plan at compile time and decided once at the closure/world boundary, against the positive fixpoint (S1.21.8); normative semantics in [`absent_semantics.md`](../../inference/absent_semantics.md) | compiler (the guard lift) + the saturator's boundary phase |

## Computed predicates

Declared in [`predicates.rs`](../../../../ein.rs/crates/ein-infer/src/predicates.rs)
(`predicates.names()`). A predicate's truth is *computed*, not looked up in
the KB. Since M1f S1f.5.7 the two are **relations** (the user's ruling of
2026-11-07, Q-M1e2.26): computed and never stored, a free side bound by the
guard, `eq` to the other side's value and `neq` over the program's names
(`predicates::domain`), and a conclusion of either checked, a false one
`(false)`; until then each compared the bindings and bound nothing
([`01_grammar.md` § Premise order around a guard](01_grammar.md#premise-order-around-a-guard)).

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `eq` | 2 | `(eq ?a ?b)` holds iff the two values are one; a free side is bound to the other | matcher `Guard` opcode |
| `neq` | 2 | `(neq ?a ?b)` holds iff the two values differ; a free side ranges over the program's names but the other | matcher `Guard` opcode |

Neither heads a value: `(q (eq k j))`, as `(q (neq k j))`, is refused, in a
fact, a conclusion and a pattern, since M1f S1f.7.10 (the user's ruling of
2026-11-11, (a)). Until then a nested `(eq k j)` was stored, computed by
nothing, and a pattern's argument `(eq ?a ?b)` matched it by its shape, a
second meaning of the one spelling.

## Pattern-macro sugar (`forall` / `unknown`) — NOT reserved

`forall` and `unknown` — the latter named `open` until 2026-08-24, renamed
(M1d P1d.2 naming decision P3) so the obligations phase can reserve bare
`open` for its KB-level verdict atom — were compile-time desugars baked into
the compiler. Since S1.5.9
they are ordinary ein-lang `(macro …)` declarations (the `std.macro` module,
[`stdlib/macro.ein`](../../../../stdlib/macro.ein))
expanded at **load** time (`kb.from_ir` → `ir.macros.expand_macros`) — they
are **no longer kernel vocabulary**, no longer in the `STRUCTURAL` table, and a
puzzle may even redefine them. A puzzle that wants them imports them
(S1.8.A1–A5):
`(import std.macro :symbols (forall unknown))` (flat surface), or
`(import std.macro)` / `:as m` for qualified access.

| macro | form | expands to |
|-------|------|------------|
| `unknown` | `(unknown P)` | `(and (absent P) (absent (not P)))` — P is neither asserted nor negated |
| `forall` | `(forall ?b G B)` | `(absent (and G (absent B)))` — guarded universal ∀b. G→B |

Both expand to nested `absent`s, so their meaning is inherited from the
[`absent` semantics](../../inference/absent_semantics.md): the ∀ arises
as ¬∃¬ over the world at the closure boundary, and a nested absent is
the one guard shape that can flip false→true during saturation — so its
candidate stays parked and is re-judged at every later quiescence
(S1.21.8; the saturator's absent-flip full-match, §C5 there, is retired).

**What a call expands to is held to the grammar** since M1f
[S1f.5.24](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md):
`unknown`'s `(not P)` is the stored negation of one fact, so `(unknown (and (p
?x) (q ?x)))`, whose `(not (and …))` negates no fact, is refused at load, its
line placed at the call in the program's file; until then it was matched as a
pattern no fact holds, so its `absent` held (W67). The same holds of a program's
own macros, a kernel primitive's name among them: a body or a call that writes
`eq`, `absent`, `not`, `false` or `or` in another shape than the grammar gives
it where a rule writes it is refused where the macro is called, and a macro no
rule calls loads ([`02_patterns.md`](02_patterns.md), `defined_behaviour.md` § 4.16).
**A value is a name, or a fact a state could hold** (the user's ruling of
2026-11-11, M1f P1f.7's value rule): `eq`, `neq`, `absent`, `and`, `or` and
`false` with an argument head no value, written or left by a macro's expansion,
and `not` does, a stored negation being a fact
([`defined_behaviour.md` § 4.22](../../defined_behaviour.md#422-a-value-is-a-name-or-a-fact-a-state-could-hold--m1f-s1f77-s1f76-s1f710-s1f73)).

## Hypothesis / query control

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `__closed__` | `(__closed__ R)` | suppress hypothesis generation for R (its extension is fixed). A **dunder** kernel-trigger name (the bare `closed` is now a free userspace name); author-written, or derived by `std.closure` and by a module's closing rule; `emit_closed`, which infers it for any relation no rule produces, runs for the corpus's `hyp` shape op alone and on no command since M1e2 S1e2.3.1. Kept kernel mechanism for M1 (S1.7.10). Used as a relation or a value, never bound (M1f T1f.5.5.11). Read by the generator's rungs 2 and 3 and the read-out's `outside` row, by no matcher (M1f S1f.6.34, E-52); no hrule proposes a closed relation and no obligation owes one (below). | `ein-infer/closed.rs` (`CLOSED = "__closed__"`); the hypgen filter |
| `__symmetric__` | `(__symmetric__ R)` | close R's extension under arg-swap natively in the saturator (`(R a b)` ⇒ `(R b a)`) — a **dunder** kernel perf-opt counterpart of the stdlib `symmetric` rule (identical closure, skips the matcher per mirror). Used as a relation or a value, never bound (M1f T1f.5.5.11) | `ein-infer/saturator.rs` (`SYMMETRIC`) |
| `hypothesis-relations` | `(solve … :hypothesis-relations (R₁ R₂ …))` | restrict the blind enumerator and the obligations rung to the listed relations, never the hrule rung; every name is a relation of the program, or the program is refused at load, as a `:no-hypothesis` name and an `:hrules` hrule and activator relation are (M1f S1f.7.8, `defined_behaviour.md` § 4.20) | `hypgen` (`HYPOTHESIS_RELATIONS`) |
| `no-hypothesis` | `(solve … :no-hypothesis (R₁ R₂ …))` | the exclusion dual of `:hypothesis-relations` — never guess on the listed relations (saturation rules on them still fire); read by the blind enumerator and the obligations rung, never the hrule rung | `hypgen` (`NO_HYPOTHESIS`) |
| `no-search` | `(solve … :no-search)` | a flag, no value: the lattice is root alone, no rung runs and nothing is proposed, whatever else the program declares, so R is {∅} where root is alive and discharged and empty otherwise (M1f S1f.6.36); the way to say *judge root alone* | `hypgen` (`NO_SEARCH`, `declares_no_search`); `mechanisms` |
| `expect` | `(query … :expect (model …) \| (or (model …) …) \| (false))` | what the answer should be — the query's own claim, checked by the engine (M1c S1c.1.2). `ein solve` exits 1 when it is false | `ein.y`'s `Claim` (shape, at the parse) + `ein-ir/expect.rs` (its relations, at load) + `ein-infer/expect.rs` (the comparison) |
| `expect` on `(solve …)` | `(solve … :expect (region …) \| (false))` | the expected region (M1e2 S1e2.1a.4): R's states, `(merged …)` generators, `(owing …)`, read only here. A top-level `(region …)`, `(merged …)` or `(owing …)` is a parse error since M1f S1f.5.5's swap, and was a load error until then; `model` stays a name | `ein.y`'s `RegionClaim` (shape, at the parse) + `ein-ir/claim.rs` (what the shape cannot say, at load) + `ein-infer/claim.rs` (against R) |

**A `(query …)` keyword outside its list is refused** since M1c S1c.1.2, and
**at the parse** since M1f S1f.5.5's swap (S1f.5.11's closed keywords: each
form takes the keywords its production in
[`ein.y`](../../../../ein.rs/crates/ein-ir/src/ein.y) lists). A query takes
**four**: `:goal`, `:goal-text`, `:over` and `:expect` above (the two dunder
triggers in the same table are facts, not keywords); the obsolete `:mode`,
accepted and ignored until the swap, is refused like any other. It was seven
until M1e2 S1e2.1a.4 moved `:hrules`, `:hypothesis-relations` and
`:no-hypothesis` to `(solve …)`, which takes those three and `:expect`, and
`:no-search` since M1f S1f.6.36; on a query each of the four is a parse error
whose note names `(solve …)`. Until
T1f.5.5.5 the loader held the two lists, `QUERY_KEYWORDS` and `SOLVE_KEYWORDS`
in [`from_ir.rs`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs), which no
program reached after the swap. The diagnostics are in
[`defined_behaviour.md` §4.1](../../defined_behaviour.md). It was a silent
no-op, which stopped being survivable when a keyword could carry a *test*.

### The five mechanisms, and each pair — M1f S1f.6.34

What a commitment may be about is steered by five declarations, and a sixth mechanism, the blind enumerator, runs
where none of the generating ones does. `hypgen`'s ladder: the hrules if the program declares any, and then the other
two rungs never run; otherwise the obligations while root owes something they may branch on (a loaded obligation rule
that owes nothing at root proposes nothing, and the blind enumerator does not run); otherwise, or where the
obligations rung declines, the blind enumerator. H is built once, at root.

| | mechanism | what it does | read by |
|---|---|---|---|
| H | `(hrule …)`, activated by `(solve :hrules …)` (one with no parameter always) | generates, rung 1, alone where declared | itself |
| O | an obligation rule, `:assert (open ?R)`, activated by a fact such as `(total-owed R isa)` | two roles: a state that owes is not in R, whatever generated it; and, with no hrule, rung 2 proposes facts of the owed relation | the discharge test; rung 2 |
| B | the blind enumerator | generates, rung 3: no hrule and no obligation rule loaded, or the obligations rung declined | — |
| HR | `(solve :hypothesis-relations …)` | a whitelist, *speculate only these*; `()` names none | rungs 2 and 3; the read-out's `outside` row |
| NH | `(solve :no-hypothesis …)` | a blacklist, *never these* | rungs 2 and 3; the `outside` row |
| C | `(__closed__ R)`, stated or derived | excludes R from rungs 2 and 3, and so keeps R out of `D`, which makes an `absent` over it closed-world | rungs 2 and 3; the `outside` row; no matcher |
| NS | `(solve :no-search)` (M1f S1f.6.36) | no rung runs: H is empty, the lattice root alone; an obligation judges root and proposes nothing | the ladder, before rung 1 |

No list and no closure filters an hrule's candidates. **The pairs** (the user's table of 2026-11-13, refused or noted
at load on every command since S1f.6.34, [`defined_behaviour.md` §
4.27](../../defined_behaviour.md#427-a-programs-hypothesis-mechanisms-do-not-contradict-one-another--m1f-s1f634)):

| pair | verdict |
|---|---|
| H + O | no conflict: H generates, O judges (`zebra2.ein`) |
| H + HR | HR is **dead**, a `note:`; a **conflict** where an active hrule proposes a relation HR leaves out |
| H + NH | NH is **dead**, a `note:`; a **conflict** where an active hrule proposes a relation NH lists (E-47) |
| H + C | a **conflict** where an active hrule proposes a closed relation; closing another relation stands |
| O + HR | with no hrule, a **conflict** where an owed relation is outside HR |
| O + NH | with no hrule, a **conflict** where an owed relation is in NH: *"filtered obligations are nonsense by definition"* |
| O + C | a **conflict** where an owed relation is closed, whatever generates: *"obligated relation can't be __closed__ also"* |
| HR + NH | a **conflict** where one relation is in both; with no hrule, NH naming a relation HR leaves out is **redundant**, a `note:` |
| HR + C | a **conflict** where a listed relation is closed |
| NH + C | with no hrule, an overlap is **redundant**, a `note:`; never a conflict |

An obligation is read as declared, a plan root's activators compile, whether or not root owes it; a derived closure
and a derived activator are read where they are derived, at root's fixpoint.

**`:no-search` beside each** (M1f S1f.6.36): with O it is silent, the obligation judging root alone, which is its use,
and with C silent too; with H one `note:`, *the hrule proposes no candidate*, and with HR or NH one, *the list filters
no candidate*, which replace the notes above. It relaxes no conflict: H, O and the lists are read as declared, so
every row of the table stands beside it
([Q-M1f.254](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f254--s1f634s-conflicts-beside-solve-no-search)).

## Not reserved (removed)

- **`model`** — the head of an `:expect` value, and *not* reserved. It is read
  structurally in that one position and nowhere else, so a relation, rule or
  atom called `model` is unaffected and needs no exclusion (a position word
  since M1f S1f.5.5's swap, a name wherever a name goes). That is
  most of why the shape is `(model …)` rather than a new form: the grammar
  did not have to learn anything
  ([`01_grammar.md` § Query](01_grammar.md#query)).

- **`closed`** (bare) — no longer a kernel trigger since the 2026-06-15 dunder
  split; the kernel keys on `__closed__` (above) and the bare `closed` is free
  for the stdlib/user to define.
- **`is-a` / `T`** — ordinary relation / atom since
  S1.7.23;
  a puzzle's inheritance rules ARE its type system, in user space.
  `T` is merely the conventional **"don't care" filler** for a signature
  slot — any atom gives identical kernel behaviour. The slot *may* be
  dropped since S1.22.4 (`(relation R)` is a legal bare declaration), but
  dropping it changes the semantics: the kernel keys
  hypothesis-eligibility on the signature being **non-empty**, so a bare
  declaration is never guessed. `T` is what you write when you want a
  guessable relation and have no meaningful type to name
  ([`01_grammar.md` §relation declarator](01_grammar.md#relation-declarator)).
  Because nothing is reserved here, **orthogonal type systems coexist** —
  in-tree proof: [`zebra.ein`](../../../../examples/zebra.ein) uses split
  `type` / `instance` relations while
  [`zebra2.ein`](../../../../examples/zebra2.ein) uses a unified `is-a`,
  and both are just declared relations. Types *of relations* are legal for
  the same reason (`(is-a co-located EquivalenceRelation)` parses and
  stores; only a rule gives it meaning — relations are objects, see
  [`../02-data-model/01_entities.md` §1.1](../02-data-model/01_entities.md)).
- **`symmetric`** (and `transitive` / `functional` / …) — plain user
  *property tags*, no kernel search-special-casing since
  S1.7.24;
  symmetry is entirely the user's `(rule symmetric)`.

See also the graph-node subset in
[`../01-ein-graph/03_ein_model.md` §6](../01-ein-graph/03_ein_model.md).
