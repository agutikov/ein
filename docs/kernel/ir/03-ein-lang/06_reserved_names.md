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
| scope / exclude the blind enumerator         | `(query … :hypothesis-relations (…))` / `:no-hypothesis (…)` | §hypothesis control |

## Top-level declarators — the closed classifier set (P1.7c)

A program is a **flat sequence of forms** (P1.7c — the `(ontology …)` /
`(facts …)` / `(reasoning …)` / `(rules …)` block wrappers were removed in
S1.7c.4).
Each top-level form is classified by its **head**: a head in the table
below is a declarator (`trace` is the engine-emitted sibling); **any other
head is a fact** — "detect facts by *not* being reserved" (the author's
design note). This set is **closed**: the parser keys on it (`rule` / `hrule` / `query` /
`config` / `trace` / `macro` / `import` are SYMBOL-excluded, so a malformed declarator — e.g.
`(query)` with no kw-pairs — is a *parse* error; `relation` is the one
exception, kept a plain SYMBOL so rules can pattern-match
`(relation ?R ?A ?B)`, so its malformed form is rejected at *load* time),
and the loader
([`ein_ir::from_ir`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs)) routes by the
same set.

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `relation` | `(relation R [A B …])` | declare a relation-type node + its arg-type signature (name + **≥ 0** type atoms; also stores the arity-1 membership fact `(relation R)`. What the signature *means* — userspace types vs kernel structure — is [`01_grammar.md` §relation declarator](01_grammar.md#what-the-signature-means--userspace-types-kernel-structure), its one definitive home) | `kb.from_ir`; `entities.KERNEL_META_RELATIONS` |
| `rule` | `(rule N (?p…) :match … :assert …)` | declare a saturation rewrite rule | `kb.from_ir` |
| `hrule` | `(hrule N (?p…) :match … :assert …)` | declare a hypothesis-generation rule (drives `hypgen`, never fired by the saturator) | `kb.from_ir`; `hypgen` |
| `query` | `(query :goal … …)` | what to ask the engine | `kb.from_ir` (`store.Query`) |
| `config` | `(config [:flag v]*)` | solver-level knobs | `kb.from_ir`; `inference.config.SolverConfig` |
| `macro` | `(macro N (?p…) BODY)` | declare a load-time AST-rewrite alias; a rule clause's `(N a…)` invocation expands to BODY before compilation (P1.8 S1.5.9) | `kb.from_ir` (`_ingest_macros`); `ir.macros.expand_macros` |
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
computed predicates (`eq` / `neq`), or `relation`. The SYMBOL-excluded keywords
(`not` / `and` / `or` / `neq` / the declarators) can't be written as a declared
name at all (parse error). The guard is about *binding* a name; a **fact** may
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
`ein-ir`'s `reserved_names_are_reserved_through_every_import_route`, and the
four routes are pinned as fixtures in
[`examples/broken/load/`](../../../../examples/broken/load/) —
`reserved_open_direct`, `_symbols`, `_qualified`, `_aliased`.

## The verdict atom — `open` (M1d P1d.2 S1d.2.3, read since S1d.2.4)

Reserved, but **not** a rule-body primitive: it appears only as an `:assert`
conclusion, the compiler never meets it in a `:match`, and no detector reads
it. It is `(false)`'s dual — where `(false)` says *this branch is dead*,
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
Five things are refused at load rather than guessed:

| refused | why |
|---|---|
| `(open …)` in a `:match` | the atom is a conclusion about the KB. The third-state probe for a *fact* is `(unknown P)`, and the message says so |
| arity ≥ 2 | the superseded `forall`-dual triple `(open ?b G B)` restated the guard in the head, where it could disagree with it |
| an `:assert` mixing `open` with anything else | such a rule would belong to the agenda and the post-fixpoint pass at once |
| `(open ?R)` where `?R` is not a parameter | a variable relation head is bound by the activator; the compiler cannot resolve one from a premise |
| a projection that does not resolve | exactly one `(absent …)` must hold a positive `?R` premise, and exactly one such premise must bind a variable the guard does not already bind. None, two, or a ground body is refused |

### What reads it — the obligation pass (S1d.2.4)

The rules live in `Program::obligations`, which neither the saturator nor
`hypgen` walks. What walks them is
[`ein_infer::obligations::tally`](../../../../ein.rs/crates/ein-infer/src/obligations.rs):
**one pass over the quiescent KB, once the fixpoint is reached** — not a
priority band inside the loop, because a band orders *selection* within the
walk and openness has to be read *after* it. `:priority` keeps one residual
meaning: the report order among obligation rules, which is what makes the
outstanding list deterministic.

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
[`tests/stdlib/algebra/23_total_owed.ein`](../../../../tests/stdlib/algebra/23_total_owed.ein)
is that state with a number attached, and it is one of the twelve corpus
entries the word moved. The distinction it draws is *no model* against *not yet
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

## Rule-body / ⊥ primitives (kept M1 kernel vocabulary)

Declared once in [`ein-core/terms.rs`](../../../../ein.rs/crates/ein-core/src/terms.rs)
(`primitives.STRUCTURAL`); the deep behaviour lives at the *engine site*.

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `not` | 1 | propositional negation; `(not X)` is a stored octagon fact whose arg is the negated proposition | matcher (`match_.rs`) + contradiction detector (`contradiction.rs`) |
| `false` | 0+ | direct ⊥ — `(false)` asserts the firing rule reached a contradiction (args empty by convention) | contradiction detector |
| `and` | 2+ | conjunction; flattened into sibling premises of one plan | compiler (`compile.rs`) |
| `or` | 2+ | disjunction; a **top-level** `(or …)` in a `:match` is lowered to one rule per disjunct at load time | loader (`kb.from_ir._match_disjuncts`) |
| `absent` | 1 | negation-as-failure on a sub-pattern (`AbsentGuard`) — a fork-local *query*, never a stored atom; lifted out of the match plan at compile time and decided once at the closure/world boundary, against the positive fixpoint (S1.21.8); normative semantics in [`absent_semantics.md`](../../inference/absent_semantics.md) | compiler (the guard lift) + the saturator's boundary phase |

## Computed predicates

Declared in [`predicates.rs`](../../../../ein.rs/crates/ein-infer/src/predicates.rs)
(`predicates.names()`). A predicate's truth is *computed* from the current
bindings, not looked up in the KB.

| name | arity | meaning | engine site |
|------|-------|---------|-------------|
| `eq` | 2 | `(eq ?a ?b)` true iff the slots resolve equal | matcher `Guard` opcode |
| `neq` | 2 | `(neq ?a ?b)` true iff the slots resolve unequal | matcher `Guard` opcode |

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

## Hypothesis / query control

| name | form | meaning | engine site |
|------|------|---------|-------------|
| `__closed__` | `(__closed__ R)` | suppress hypothesis generation for R (its extension is fixed). A **dunder** kernel-trigger name (the bare `closed` is now a free userspace name); author-writable, but usually **auto-inferred** by `emit_closed` for any relation no rule produces, or derived by `std.closure`. Kept kernel mechanism for M1 (S1.7.10). | `ein-infer/closed.rs` (`CLOSED = "__closed__"`); the hypgen filter |
| `__symmetric__` | `(__symmetric__ R)` | close R's extension under arg-swap natively in the saturator (`(R a b)` ⇒ `(R b a)`) — a **dunder** kernel perf-opt counterpart of the stdlib `symmetric` rule (identical closure, skips the matcher per mirror) | `ein-infer/saturator.rs` (`SYMMETRIC`) |
| `hypothesis-relations` | `(query … :hypothesis-relations (R₁ R₂ …))` | restrict the blind enumerator to the listed relations | `hypgen` (`HYPOTHESIS_RELATIONS`) |
| `no-hypothesis` | `(query … :no-hypothesis (R₁ R₂ …))` | the exclusion dual of `:hypothesis-relations` — never guess on the listed relations (saturation rules on them still fire) | `hypgen` (`NO_HYPOTHESIS`) |
| `expect` | `(query … :expect (model …) \| (or (model …) …) \| (false))` | what the answer should be — the query's own claim, checked by the engine (M1c S1c.1.2). `ein solve` exits 1 when it is false | `ein-ir/expect.rs` (shape, at load) + `ein-infer/expect.rs` (the comparison) |

**A `(query …)` keyword outside the allow-list is a load error** since M1c
S1c.1.2. The list is **seven**: the three query keywords above
(`:hypothesis-relations`, `:no-hypothesis`, `:expect` — the two dunder triggers
in the same table are facts, not keywords), plus `:goal`, `:goal-text`,
`:hrules` and the obsolete, ignored `:mode`
([`from_ir.rs`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs)'s
`QUERY_KEYWORDS`; the same seven are named in the diagnostic,
[`defined_behaviour.md` §4.1](../../defined_behaviour.md)). It was a silent
no-op, which stopped being survivable when a keyword could carry a *test*.

## Not reserved (removed)

- **`model`** — the head of an `:expect` value, and *not* reserved. It is read
  structurally in that one position and nowhere else, so a relation, rule or
  atom called `model` is unaffected and needs no SYMBOL exclusion. That is
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
